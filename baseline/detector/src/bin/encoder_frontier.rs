//! Execute a frozen encoder frontier and select using Development alone.
use anyhow::{Context, Result, ensure};
use clap::Parser;
use serde_json::{Value, json};
use slop_ninja_detector::dataset::sha256;
use std::{fs, path::PathBuf, process::Command};

#[derive(Parser)]
struct Args {
    #[arg(long)]
    python: PathBuf,
    #[arg(long)]
    runner_dir: PathBuf,
    #[arg(long)]
    checkpoint: PathBuf,
    #[arg(long)]
    shards: PathBuf,
    #[arg(long)]
    whitespace_train: PathBuf,
    #[arg(long)]
    token_audit: PathBuf,
    #[arg(long)]
    output_dir: PathBuf,
    #[arg(long, default_value = "mps", value_parser = ["cpu", "mps"])]
    device: String,
    /// Freeze all rates before fitting. Lower rates win exact loss ties.
    #[arg(long, value_delimiter = ',', default_value = "0.00001,0.00002")]
    learning_rates: Vec<f64>,
    #[arg(long, default_value_t = 5, value_parser = clap::value_parser!(u32).range(1..=20))]
    epochs: u32,
    #[arg(long, default_value = "three_class", value_parser = ["three_class", "human_model"])]
    selection_objective: String,
}

fn main() -> Result<()> {
    let mut args = Args::parse();
    ensure!(!args.output_dir.exists(), "Use a new frontier directory");
    ensure!(
        (1..=8).contains(&args.learning_rates.len())
            && args
                .learning_rates
                .iter()
                .all(|rate| rate.is_finite() && *rate > 0.0 && *rate <= 0.001),
        "Declare 1..8 finite learning rates in (0, 0.001]"
    );
    args.learning_rates.sort_by(f64::total_cmp);
    ensure!(
        args.learning_rates
            .windows(2)
            .all(|pair| pair[0] != pair[1]),
        "Duplicate learning rate"
    );
    let audit: Value = serde_json::from_slice(&fs::read(&args.token_audit)?)?;
    ensure!(
        audit["fits_configured_token_limit"] == true
            && audit["max_tokens"] == 1024
            && audit["model_executed"] == false
            && audit["records_dropped"] == 0,
        "Require a passing 1024-token audit without filtering or predictions"
    );
    let mut hashes = serde_json::Map::new();
    for split in ["train", "development", "calibration", "test"] {
        let hash = sha256(fs::read(args.shards.join(format!("{split}.jsonl")))?);
        ensure!(
            audit["shards"][split]["sha256"] == hash,
            "Audit shard mismatch"
        );
        hashes.insert(split.into(), json!(hash));
    }
    let checkpoint_hash = sha256(fs::read(args.checkpoint.join("checkpoint.json"))?);
    ensure!(
        audit["checkpoint_pin_sha256"] == checkpoint_hash,
        "Checkpoint audit mismatch"
    );
    let train_path = args.runner_dir.join("train.py");
    let protocol = json!({
        "schema":"slop_ninja_encoder_frontier_v1",
        "learning_rates":args.learning_rates,"epochs":args.epochs,"batch_size":4,"seed":17,
        "sample_weighting":"source_origin","class_weights":"none",
        "selection_objective":args.selection_objective,
        "selection":"positive Development recall for each class in the selected objective, then minimum uncalibrated Development objective log loss; earlier epoch and lower learning rate break exact ties",
        "partition_sha256":hashes,"checkpoint_pin_sha256":checkpoint_hash,
        "whitespace_train_sha256":sha256(fs::read(&args.whitespace_train)?),
        "data_rights_sha256":sha256(fs::read(args.shards.join("data_rights.json"))?),
        "train_py_sha256":sha256(fs::read(&train_path)?),
        "common_py_sha256":sha256(fs::read(args.runner_dir.join("common.py"))?),
        "token_audit_sha256":sha256(fs::read(&args.token_audit)?),
        "python":args.python,"device":args.device,"final_test_opened":false,
        "test_access":"count/length/hash audit only; no Test argument is passed to the trainer"
    });
    fs::create_dir_all(&args.output_dir)?;
    fs::write(
        args.output_dir.join("protocol.json"),
        serde_json::to_vec_pretty(&protocol)?,
    )?;
    let mut candidates = Vec::new();
    let mut selected: Option<(f64, f64, PathBuf, String)> = None;
    for &rate in &args.learning_rates {
        let name = format!("lr-{rate}");
        let output = args.output_dir.join(&name);
        let log_path = args.output_dir.join(format!("{name}.log"));
        let log = fs::File::create_new(&log_path)?;
        let mut command = Command::new(&args.python);
        command
            .arg(&train_path)
            .arg("--checkpoint")
            .arg(&args.checkpoint)
            .arg("--data-rights")
            .arg(args.shards.join("data_rights.json"))
            .arg("--training-whitespace-view")
            .arg(&args.whitespace_train)
            .arg("--output")
            .arg(&output)
            .arg("--device")
            .arg(&args.device)
            .args([
                "--max-tokens",
                "1024",
                "--batch-size",
                "4",
                "--seed",
                "17",
                "--weight-decay",
                "0.01",
                "--class-weights",
                "none",
                "--sample-weighting",
                "source_origin",
                "--require-class-coverage",
                "--learning-rate",
            ])
            .arg(rate.to_string())
            .arg("--epochs")
            .arg(args.epochs.to_string())
            .env("HF_HUB_OFFLINE", "1")
            .env("TRANSFORMERS_OFFLINE", "1")
            .env("PYTHONUNBUFFERED", "1");
        // The original frozen v4 runner predates this optional argument.
        if args.selection_objective != "three_class" {
            command
                .arg("--selection-objective")
                .arg(&args.selection_objective);
        }
        for split in ["train", "development", "calibration"] {
            command
                .arg(format!("--{split}-jsonl"))
                .arg(args.shards.join(format!("{split}.jsonl")));
        }
        fs::write(
            args.output_dir.join(format!("{name}-invocation.json")),
            serde_json::to_vec_pretty(&json!({
                "program":command.get_program().to_string_lossy(),
                "args":command.get_args().map(|a| a.to_string_lossy()).collect::<Vec<_>>()
            }))?,
        )?;
        println!("Fitting {name}; log: {}", log_path.display());
        let status = command.stdout(log.try_clone()?).stderr(log).status()?;
        let diagnostic = args
            .output_dir
            .join(format!("{name}-no-eligible-epoch.json"));
        if !status.success() {
            ensure!(
                diagnostic.exists() && !output.exists(),
                "Training failed; inspect {}",
                log_path.display()
            );
            let failed: Value = serde_json::from_slice(&fs::read(&diagnostic)?)?;
            ensure!(
                failed["status"] == "no_eligible_epoch",
                "Unexpected failure diagnostic"
            );
            candidates.push(json!({"name":name,"learning_rate":rate,"eligible":false,
                "diagnostic_sha256":sha256(fs::read(diagnostic)?)}));
            continue;
        }
        let training: Value = serde_json::from_slice(&fs::read(output.join("training.json"))?)?;
        let manifest: Value = serde_json::from_slice(&fs::read(output.join("manifest.json"))?)?;
        let coverage_key = if args.selection_objective == "human_model" {
            "requires_nonzero_development_binary_recall"
        } else {
            "requires_nonzero_development_class_recall"
        };
        ensure!(
            training["final_test_opened"] == false
                && training[coverage_key] == true
                && training["selection_objective"]
                    .as_str()
                    .unwrap_or("three_class")
                    == args.selection_objective,
            "Unexpected training selection contract"
        );
        for split in ["train", "development", "calibration"] {
            ensure!(
                training["partition_sha256"][split] == protocol["partition_sha256"][split],
                "Training corpus changed after the frontier was frozen"
            );
        }
        ensure!(
            training["checkpoint_pin_sha256"] == protocol["checkpoint_pin_sha256"]
                && training["data_rights_sha256"] == protocol["data_rights_sha256"]
                && training["formatting_augmentation"]["alternate_train_sha256"]
                    == protocol["whitespace_train_sha256"]
                && manifest["files"]["runner/train.py"] == protocol["train_py_sha256"]
                && manifest["files"]["runner/common.py"] == protocol["common_py_sha256"],
            "Training inputs or runner changed after the frontier was frozen"
        );
        let epoch = training["epoch_history"]
            .as_array()
            .context("Missing epoch history")?
            .iter()
            .find(|e| e["epoch"] == training["selected_epoch"])
            .context("Missing selected epoch")?;
        ensure!(
            epoch["selection_eligible"] == true,
            "Selected epoch was ineligible"
        );
        let development_key = if args.selection_objective == "human_model" {
            "development_human_model"
        } else {
            "development"
        };
        let loss = epoch[development_key]["log_loss"]
            .as_f64()
            .context("Missing Development loss")?;
        ensure!(loss.is_finite(), "Non-finite Development loss");
        let artifact_id = manifest["artifact_id"]
            .as_str()
            .context("Missing artifact identity")?
            .to_owned();
        candidates.push(json!({"name":name,"learning_rate":rate,"eligible":true,
            "artifact_id":artifact_id,"selected_epoch":training["selected_epoch"],
            "selection_objective":args.selection_objective,
            "development_uncalibrated":epoch["development"],
            "development_selection_metrics":epoch[development_key],
            "training_sha256":sha256(fs::read(output.join("training.json"))?)}));
        if selected.as_ref().is_none_or(|(best, _, _, _)| loss < *best) {
            selected = Some((loss, rate, output, artifact_id));
        }
    }
    let winner = selected.map(|(loss, rate, path, id)| {
        json!({
            "path":path,"artifact_id":id,"selection_objective":args.selection_objective,
            "development_uncalibrated_log_loss":loss,"learning_rate":rate
        })
    });
    let report = json!({"schema":"slop_ninja_encoder_frontier_selection_v1",
        "protocol_sha256":sha256(fs::read(args.output_dir.join("protocol.json"))?),
        "candidates":candidates,"selected":winner,"final_test_opened":false,
        "calibration_used_to_select_candidate":false});
    fs::write(
        args.output_dir.join("selection.json"),
        serde_json::to_vec_pretty(&report)?,
    )?;
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}
