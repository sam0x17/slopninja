//! Frozen CPU encoder evaluation with separately bound calibration thresholds.
use anyhow::{Context, Result, ensure};
use clap::{Parser, Subcommand, ValueEnum};
use serde_json::{Value, json};
use slop_ninja_detector::{
    dataset::{self, OriginRecord, Split, sha256},
    metrics::{self, OperatingPoint, ProbabilityRow},
    model::CLASS_NAMES,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Clone, Copy, ValueEnum)]
enum Partition {
    Development,
    Test,
}

#[derive(Subcommand)]
enum Task {
    /// Freeze decision thresholds from the artifact's original Calibration shard.
    FreezeThresholds {
        #[arg(long)]
        calibration_jsonl: PathBuf,
        /// Reuse an archived calibrated export instead of recomputing its scores.
        #[arg(long, requires = "expected_predictions_sha256")]
        calibration_predictions: Option<PathBuf>,
        #[arg(long, requires = "calibration_predictions")]
        expected_predictions_sha256: Option<String>,
    },
    /// Evaluate a partition using an existing, artifact-bound threshold file.
    Evaluate {
        #[arg(long)]
        records: PathBuf,
        #[arg(long)]
        thresholds: PathBuf,
        #[arg(long, value_enum)]
        split: Partition,
        #[arg(long)]
        open_final_test: bool,
    },
}

#[derive(Parser)]
struct Args {
    #[arg(long)]
    artifact: PathBuf,
    #[arg(long)]
    python: PathBuf,
    #[arg(long)]
    output_dir: PathBuf,
    #[command(subcommand)]
    task: Task,
}

fn save(path: &Path, value: &Value) -> Result<()> {
    let mut file = fs::File::create_new(path)?;
    serde_json::to_writer_pretty(&mut file, value)?;
    file.write_all(b"\n")?;
    Ok(())
}

fn object(path: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}

fn bind_predictions(
    records: &[OriginRecord],
    outputs: &[Value],
    artifact_id: &str,
) -> Result<Vec<ProbabilityRow>> {
    let by_id: BTreeMap<_, _> = records.iter().map(|r| (r.id.as_str(), r)).collect();
    ensure!(by_id.len() == records.len(), "Duplicate input ID");
    let mut seen = BTreeSet::new();
    let mut rows = Vec::new();
    for output in outputs {
        let id = output["id"].as_str().context("Missing prediction ID")?;
        let record = by_id.get(id).context("Unexpected prediction ID")?;
        ensure!(
            seen.insert(id)
                && output["artifact_id"] == artifact_id
                && output["status"] == "ok"
                && output["labels"] == json!(CLASS_NAMES),
            "Prediction coverage, artifact, status or class order differs"
        );
        rows.push(ProbabilityRow {
            id: record.id.clone(),
            group: record.source_group.clone(),
            label: record.origin.index(),
            probabilities: serde_json::from_value(output["probabilities"].clone())?,
        });
    }
    ensure!(rows.len() == records.len(), "Missing predictions");
    metrics::summarize(&rows)?;
    Ok(rows)
}

fn infer(args: &Args, records: &[OriginRecord], artifact_id: &str) -> Result<Vec<ProbabilityRow>> {
    let input = args.output_dir.join("requests.jsonl");
    let mut file = fs::File::create_new(&input)?;
    for record in records {
        // No labels, grouping or source metadata are sent to the detector.
        serde_json::to_writer(&mut file, &json!({"id":record.id,"text":record.text}))?;
        file.write_all(b"\n")?;
    }
    file.sync_all()?;
    drop(file);
    let output = args.output_dir.join("responses.jsonl");
    let status = Command::new(&args.python)
        .arg(args.artifact.join("runner/infer.py"))
        .arg("--artifact")
        .arg(&args.artifact)
        .env("HF_HUB_OFFLINE", "1")
        .env("TRANSFORMERS_OFFLINE", "1")
        .stdin(fs::File::open(&input)?)
        .stdout(fs::File::create_new(&output)?)
        .stderr(fs::File::create_new(args.output_dir.join("inference.log"))?)
        .status()?;
    ensure!(
        status.success(),
        "Bundled CPU inference failed; partial outputs retained"
    );
    let values = fs::read_to_string(&output)?
        .lines()
        .map(serde_json::from_str)
        .collect::<serde_json::Result<Vec<Value>>>()?;
    let rows = bind_predictions(records, &values, artifact_id)?;
    save(
        &args.output_dir.join("inference-bindings.json"),
        &json!({
            "artifact_id":artifact_id,"requests_sha256":sha256(fs::read(input)?),
            "responses_sha256":sha256(fs::read(output)?),
            "records":records.iter().map(|r| json!({"id":r.id,"text_sha256":r.text_sha256})).collect::<Vec<_>>()
        }),
    )?;
    Ok(rows)
}

fn import_calibration(
    args: &Args,
    records: &[OriginRecord],
    artifact_id: &str,
    path: &Path,
    expected_hash: &str,
) -> Result<Vec<ProbabilityRow>> {
    let bytes = fs::read(path)?;
    ensure!(
        sha256(&bytes) == expected_hash,
        "Archived prediction hash differs"
    );
    let expected: BTreeMap<_, _> = records
        .iter()
        .map(|r| (r.id.as_str(), r.text_sha256.as_str()))
        .collect();
    let mut values = Vec::new();
    for line in bytes.split(|b| *b == b'\n').filter(|l| !l.is_empty()) {
        let mut value: Value = serde_json::from_slice(line)?;
        let id = value["id"]
            .as_str()
            .context("Missing archived prediction ID")?;
        ensure!(
            value["text_sha256"]
                == *expected
                    .get(id)
                    .context("Unexpected archived prediction ID")?,
            "Archived prediction text differs from the original Calibration shard"
        );
        value["labels"] = value["classes"].clone();
        values.push(value);
    }
    let rows = bind_predictions(records, &values, artifact_id)?;
    fs::File::create_new(args.output_dir.join("archived-predictions.jsonl"))?.write_all(&bytes)?;
    save(
        &args.output_dir.join("inference-bindings.json"),
        &json!({
            "mode":"archived_calibrated_cpu_export","artifact_id":artifact_id,
            "predictions_sha256":expected_hash,"recomputed":false,
            "records":records.iter().map(|r| json!({"id":r.id,"text_sha256":r.text_sha256})).collect::<Vec<_>>()
        }),
    )?;
    Ok(rows)
}

fn training_prior(training: &Value) -> Result<[f64; 3]> {
    if training["sample_weight_policy"] == "source_origin"
        || training["class_weight_policy"] == "inverse_frequency"
    {
        return Ok([1.0 / 3.0; 3]);
    }
    let counts: [usize; 3] = serde_json::from_value(training["class_counts"]["train"].clone())?;
    let total = counts.iter().sum::<usize>();
    ensure!(total > 0, "Empty training prior");
    Ok(counts.map(|n| n as f64 / total as f64))
}

fn main() -> Result<()> {
    let args = Args::parse();
    if matches!(
        args.task,
        Task::Evaluate {
            split: Partition::Test,
            open_final_test: false,
            ..
        }
    ) {
        anyhow::bail!("Final Test remains sealed; explicitly freeze selection before opening it");
    }
    ensure!(!args.output_dir.exists(), "Use a new evaluation directory");
    let output_parent = args
        .output_dir
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    ensure!(
        !output_parent
            .canonicalize()
            .context("Evaluation parent directory must already exist")?
            .starts_with(args.artifact.canonicalize()?),
        "Evaluation outputs must stay outside the immutable artifact"
    );
    let manifest = object(&args.artifact.join("manifest.json"))?;
    let artifact_id = manifest["artifact_id"]
        .as_str()
        .context("Missing artifact ID")?;
    let training_path = args.artifact.join("training.json");
    let training_hash = sha256(fs::read(&training_path)?);
    ensure!(
        manifest["files"]["training.json"] == training_hash,
        "Training manifest differs"
    );
    let training = object(&training_path)?;
    let prior = training_prior(&training)?;
    match &args.task {
        Task::FreezeThresholds {
            calibration_jsonl,
            calibration_predictions,
            expected_predictions_sha256,
        } => {
            let hash = sha256(fs::read(calibration_jsonl)?);
            ensure!(
                training["partition_sha256"]["calibration"] == hash,
                "Calibration input differs from the frozen artifact's original shard"
            );
            let records = dataset::read_records(calibration_jsonl)?;
            ensure!(
                records.iter().all(|r| r.split == Some(Split::Calibration)),
                "Unexpected Calibration split"
            );
            fs::create_dir_all(&args.output_dir)?;
            let rows = match calibration_predictions {
                Some(path) => import_calibration(
                    &args,
                    &records,
                    artifact_id,
                    path,
                    expected_predictions_sha256
                        .as_deref()
                        .context("Missing archived prediction hash")?,
                )?,
                None => infer(&args, &records, artifact_id)?,
            };
            let points = metrics::select_operating_points(&rows, &[0.01, 0.05])?;
            let content = json!({"schema":"slop_ninja_encoder_thresholds_v1",
                "artifact_id":artifact_id,"training_sha256":training_hash,"calibration_shard_sha256":hash,
                "inference_bindings_sha256":sha256(fs::read(args.output_dir.join("inference-bindings.json"))?),
                "calibration_ids":records.iter().map(|r| &r.id).collect::<Vec<_>>(),
                "calibration_groups":records.iter().map(|r| &r.source_group).collect::<BTreeSet<_>>(),
                "calibration_text_hashes":records.iter().map(|r| &r.text_sha256).collect::<BTreeSet<_>>(),
                "effective_training_class_prior":prior,"points":points,
                "temperature_refitted":false,"test_predictions_opened":false});
            save(
                &args.output_dir.join("thresholds.json"),
                &json!({
                "content_sha256":sha256(serde_json::to_vec(&content)?),"content":content}),
            )?;
            println!("{}", serde_json::to_string_pretty(&points)?);
        }
        Task::Evaluate {
            records,
            thresholds,
            split,
            ..
        } => {
            let frozen = object(thresholds)?;
            let content = &frozen["content"];
            ensure!(
                frozen["content_sha256"] == sha256(serde_json::to_vec(content)?)
                    && content["schema"] == "slop_ninja_encoder_thresholds_v1"
                    && content["artifact_id"] == artifact_id
                    && content["training_sha256"] == training_hash
                    && content["calibration_shard_sha256"]
                        == training["partition_sha256"]["calibration"],
                "Threshold file is not bound to this artifact and its original calibration"
            );
            let points: Vec<OperatingPoint> = serde_json::from_value(content["points"].clone())?;
            ensure!(
                points.len() == 2,
                "Expected the two declared operating points"
            );
            for (point, rate) in points.iter().zip([0.01, 0.05]) {
                point.validate()?;
                ensure!(
                    point.target_human_false_positive_rate == rate,
                    "Unexpected target FPR"
                );
            }
            let records_hash = sha256(fs::read(records)?);
            let selected_split = match split {
                Partition::Development => Split::Development,
                Partition::Test => Split::Test,
            };
            let selected: Vec<_> = dataset::read_records(records)?
                .into_iter()
                .filter(|r| r.split == Some(selected_split))
                .collect();
            ensure!(!selected.is_empty(), "Empty evaluation partition");
            let cal_ids: BTreeSet<String> =
                serde_json::from_value(content["calibration_ids"].clone())?;
            let cal_groups: BTreeSet<String> =
                serde_json::from_value(content["calibration_groups"].clone())?;
            let cal_hashes: BTreeSet<String> =
                serde_json::from_value(content["calibration_text_hashes"].clone())?;
            ensure!(
                selected.iter().all(|r| !cal_ids.contains(&r.id)
                    && !cal_groups.contains(&r.source_group)
                    && !cal_hashes.contains(&r.text_sha256)),
                "Evaluation overlaps the model's calibration IDs, families or exact texts"
            );
            let mut providers = BTreeMap::<String, BTreeSet<String>>::new();
            let mut profiles = BTreeMap::<String, BTreeSet<String>>::new();
            for record in &selected {
                if let Some(generation) = &record.generation {
                    providers
                        .entry(record.source_group.clone())
                        .or_default()
                        .insert(generation.model_revision.clone());
                    profiles
                        .entry(record.source_group.clone())
                        .or_default()
                        .insert(
                            generation
                                .prompt_profile
                                .as_ref()
                                .map(|p| p.profile.id.clone())
                                .unwrap_or_else(|| "legacy-unprofiled".into()),
                        );
                }
            }
            ensure!(
                selected.iter().all(|r| providers
                    .get(&r.source_group)
                    .is_some_and(|p| p.len() == 1)
                    && profiles.get(&r.source_group).is_some_and(|p| p.len() == 1)),
                "Evaluation requires one declared generator/profile pair per source family"
            );
            fs::create_dir_all(&args.output_dir)?;
            save(
                &args.output_dir.join("inputs.json"),
                &json!({
                "records_sha256":records_hash,"thresholds_sha256":sha256(fs::read(thresholds)?),
                "artifact_id":artifact_id,"split":selected_split,
                "final_test_opened":matches!(split, Partition::Test)}),
            )?;
            let rows = infer(&args, &selected, artifact_id)?;
            let by_id: BTreeMap<_, _> = selected.iter().map(|r| (r.id.as_str(), r)).collect();
            let mut slices = BTreeMap::<String, Vec<ProbabilityRow>>::new();
            for row in &rows {
                let record = by_id[row.id.as_str()];
                let provider = providers[&record.source_group].iter().next().unwrap();
                let profile = profiles[&record.source_group].iter().next().unwrap();
                for key in [
                    format!("provider:{provider}"),
                    format!("collection:{}", record.source.collection),
                    format!("profile:{profile}"),
                    format!("provider-profile:{provider}|{profile}"),
                ] {
                    slices.entry(key).or_default().push(row.clone());
                }
            }
            let mut slice_reports = BTreeMap::new();
            let mut binary_slice_reports = BTreeMap::new();
            for (key, slice_rows) in slices {
                binary_slice_reports
                    .insert(key.clone(), metrics::summarize_human_model(&slice_rows)?);
                slice_reports.insert(
                    key,
                    metrics::evaluate_probability_rows(artifact_id, &slice_rows, prior, &points)?,
                );
            }
            let report = metrics::evaluate_probability_rows(artifact_id, &rows, prior, &points)?;
            let tolerance = manifest["reference_runtime"]["probability_absolute_tolerance"]
                .as_f64()
                .context("Missing reference probability tolerance")?;
            ensure!(
                tolerance.is_finite() && tolerance > 0.0,
                "Invalid reference tolerance"
            );
            let near_cutoffs: Vec<_> = points.iter().map(|point| {
                let near: Vec<_> = rows.iter().filter(|r|
                    (r.probabilities[1] + r.probabilities[2] - point.threshold).abs() <= 2.0 * tolerance).collect();
                json!({"target_human_false_positive_rate":point.target_human_false_positive_rate,
                    "threshold":point.threshold,"diagnostic_margin":2.0*tolerance,"rows":near.len(),
                    "human_rows":near.iter().filter(|r| r.label == 0).count(),
                    "ids":near.iter().map(|r| &r.id).collect::<Vec<_>>()})
            }).collect();
            save(
                &args.output_dir.join("report.json"),
                &json!({
                "schema":"slop_ninja_frozen_encoder_evaluation_v1","artifact_id":artifact_id,
                "records_sha256":records_hash,"split":selected_split,
                "thresholds_sha256":sha256(fs::read(thresholds)?),"effective_training_class_prior":prior,
                "final_test_opened":matches!(split, Partition::Test),"report":report,"slices":slice_reports,
                "human_model":metrics::summarize_human_model(&rows)?,"human_model_slices":binary_slice_reports,
                "near_cutoffs":near_cutoffs,
                "notes":["Each detector retains its own original Calibration shard and frozen temperature; threshold files must exist before evaluation.",
                    "Provider slices include the corresponding human roots selected with that provider's draft/edit pair.",
                    "Human/model metrics exclude mixed-origin rows and use P(model_only)+P(mixed); accuracy_at_half is diagnostic, separate from frozen operating thresholds. Profile slices include the corresponding human roots; legacy rows retain an explicit unprofiled category.",
                    "Effective training-prior control accounts for source-origin or inverse-frequency loss weights.",
                    "Different frozen operating points do not prove superiority at matched population FPR.",
                    "Cutoff proximity uses twice the per-class reference-vector tolerance as a diagnostic margin, not a proven bound on cross-platform drift for every text."]}),
            )?;
            println!("{}", serde_json::to_string_pretty(&report.model)?);
        }
    }
    Ok(())
}
