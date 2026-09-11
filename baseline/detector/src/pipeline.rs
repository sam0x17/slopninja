//! Corpus-to-feature handoff and explicit, separated fit/evaluation commands.

use anyhow::{Context, Result, ensure};
use grammar_core::features::{FEATURE_VERSION, Family, Features, extract, words};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use slop_ninja_detector::{
    dataset::{self, Evidence, Origin, OriginRecord, Split, sha256},
    features::{FeatureMode, FeatureRow},
    metrics, model,
};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

pub const MIN_WORDS: usize = 40;
pub const MAX_WORDS: usize = 2048;
pub const MAX_BYTES: usize = 100_000;
pub const FEATURE_RECORD_SCHEMA: &str = "slop_ninja_origin_feature_record_v1";
pub const BRIDGE: &str =
    include_str!("../../../grammar/crates/grammar-spacy/python/syntax_bridge.py");

#[derive(Clone, Debug, Serialize, Deserialize)]
struct FeatureHeader {
    schema: String,
    id: String,
    group: String,
    origin: Origin,
    evidence: Evidence,
    split: Split,
    text_sha256: String,
    normalized_text_sha256: String,
    word_count: usize,
    source_manifest_sha256: String,
    features_sha256: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct FeatureRecord {
    #[serde(flatten)]
    header: FeatureHeader,
    features: Features,
}

fn split_name(split: Split) -> &'static str {
    match split {
        Split::Train => "train",
        Split::Development => "development",
        Split::Calibration => "calibration",
        Split::Test => "test",
    }
}

fn normal_records(input: &Path) -> Result<Vec<OriginRecord>> {
    let records = dataset::read_records(input)?;
    ensure!(
        records
            .iter()
            .all(|record| record.evidence != Evidence::SyntheticFixture),
        "synthetic protocol fixtures cannot enter normal corpus training/export"
    );
    ensure!(
        records.iter().all(|record| record.split.is_some()),
        "freeze source-family splits before export or feature extraction"
    );
    Ok(records)
}

pub fn export_shards(input: &Path, output: &Path) -> Result<Value> {
    let records = normal_records(input)?;
    fs::create_dir_all(output)?;
    let mut manifest = BTreeMap::new();
    for split in [
        Split::Train,
        Split::Development,
        Split::Calibration,
        Split::Test,
    ] {
        let selected: Vec<_> = records
            .iter()
            .filter(|record| record.split == Some(split))
            .cloned()
            .collect();
        if selected.is_empty() {
            continue;
        }
        let path = output.join(format!("{}.jsonl", split_name(split)));
        dataset::write_records(&path, &selected)?;
        manifest.insert(
            split_name(split),
            json!({"rows":selected.len(), "sha256":sha256(fs::read(&path)?)}),
        );
    }
    let summary = json!({"schema":"slop_ninja_origin_shards_v1", "source_manifest_sha256":sha256(fs::read(input)?), "partitions":manifest});
    write_json(&output.join("manifest.json"), &summary)?;
    Ok(summary)
}

fn lexical_schema() -> String {
    format!("slop_ninja_words_only_v1;{FEATURE_VERSION}")
}

fn validate_input(text: &str) -> Result<usize> {
    ensure!(
        text.len() <= MAX_BYTES,
        "input exceeds {MAX_BYTES} UTF-8 bytes"
    );
    let count = words(text).len();
    ensure!(
        (MIN_WORDS..=MAX_WORDS).contains(&count),
        "input contains {count} words; supported range is {MIN_WORDS}..={MAX_WORDS}; text is never truncated"
    );
    Ok(count)
}

fn lexical_features(text: &str) -> Features {
    let tokens = words(text);
    let mut counts = BTreeMap::new();
    for word in &tokens {
        *counts.entry(word.clone()).or_insert(0_u64) += 1;
    }
    Features {
        schema: lexical_schema(),
        parser_identity: "none;grammar-core-words-v1".into(),
        source_sha256: sha256(text),
        families: BTreeMap::from([(
            "word".into(),
            Family::Distribution {
                counts,
                opportunities: tokens.len() as u64,
            },
        )]),
    }
}

fn parsed_features(texts: &[&str], python: &str) -> Result<Vec<Features>> {
    let mut bridge = tempfile::Builder::new()
        .prefix("slop-ninja-syntax-")
        .suffix(".py")
        .tempfile()?;
    bridge.write_all(BRIDGE.as_bytes())?;
    bridge.flush()?;
    let annotations = grammar_spacy::parse_batch_with_bridge(texts, python, bridge.path())?;
    annotations
        .into_iter()
        .enumerate()
        .map(|(index, annotation)| {
            let doc = annotation
                .map_err(anyhow::Error::msg)
                .with_context(|| format!("annotation failed at batch index {index}"))?;
            let mut features = extract(&doc)?;
            features
                .parser_identity
                .push_str(&format!(";detector-bridge-sha256={}", sha256(BRIDGE)));
            Ok(features)
        })
        .collect()
}

pub fn featurize(
    input: &Path,
    output: &Path,
    mode: FeatureMode,
    python: Option<&str>,
    batch_size: usize,
) -> Result<Value> {
    ensure!(
        (1..=512).contains(&batch_size),
        "batch size must be 1..=512"
    );
    let records = normal_records(input)?;
    let word_counts: Vec<usize> = records
        .iter()
        .map(|record| {
            validate_input(&record.text)
                .with_context(|| format!("{} is outside the input contract", record.id))
        })
        .collect::<Result<_>>()?;
    let source_manifest_sha256 = sha256(fs::read(input)?);
    let mut result = Vec::with_capacity(records.len());
    for (batch_index, batch) in records.chunks(batch_size).enumerate() {
        let features = if mode == FeatureMode::Word {
            batch
                .iter()
                .map(|record| lexical_features(&record.text))
                .collect()
        } else {
            parsed_features(
                &batch
                    .iter()
                    .map(|record| record.text.as_str())
                    .collect::<Vec<_>>(),
                python.context("--python is required for grammar/combined feature extraction")?,
            )?
        };
        for (offset, (record, features)) in batch.iter().zip(features).enumerate() {
            ensure!(
                features.source_sha256 == record.text_sha256,
                "parser/extractor changed source identity"
            );
            result.push(FeatureRecord {
                header: FeatureHeader {
                    schema: FEATURE_RECORD_SCHEMA.into(),
                    id: record.id.clone(),
                    group: record.source_group.clone(),
                    origin: record.origin,
                    evidence: record.evidence,
                    split: record.split.context("missing frozen split")?,
                    text_sha256: record.text_sha256.clone(),
                    normalized_text_sha256: sha256(words(&record.text).join(" ")),
                    word_count: word_counts[batch_index * batch_size + offset],
                    source_manifest_sha256: source_manifest_sha256.clone(),
                    features_sha256: sha256(serde_json::to_vec(&features)?),
                },
                features,
            });
        }
        eprintln!("Extracted {} / {} records", result.len(), records.len());
    }
    write_jsonl(output, &result)?;
    Ok(
        json!({"schema":FEATURE_RECORD_SCHEMA, "rows":result.len(), "mode":mode, "feature_file_sha256":sha256(fs::read(output)?), "source_manifest_sha256":source_manifest_sha256, "contains_source_text":false, "input_limits":{"min_words":MIN_WORDS,"max_words":MAX_WORDS,"max_utf8_bytes":MAX_BYTES}}),
    )
}

fn validate_header(header: &FeatureHeader) -> Result<()> {
    ensure!(
        header.schema == FEATURE_RECORD_SCHEMA,
        "unsupported feature record schema"
    );
    ensure!(
        !header.id.is_empty() && !header.group.is_empty(),
        "feature record lacks ID/group"
    );
    ensure!(
        !matches!(
            header.evidence,
            Evidence::SyntheticFixture | Evidence::Unknown
        ),
        "{}: synthetic/unknown evidence cannot enter normal training",
        header.id
    );
    let compatible = match header.evidence {
        Evidence::HistoricalProxy | Evidence::DocumentedHuman => header.origin == Origin::HumanOnly,
        Evidence::RecordedModelGeneration => header.origin == Origin::ModelOnly,
        Evidence::ModelEditOfHistoricalProxy => header.origin == Origin::Mixed,
        Evidence::RecordedMixedWorkflow | Evidence::SyntheticFixture | Evidence::Unknown => false,
    };
    ensure!(
        compatible,
        "{}: unsupported feature origin/evidence combination",
        header.id
    );
    ensure!(
        (MIN_WORDS..=MAX_WORDS).contains(&header.word_count),
        "{}: feature record outside supported word count",
        header.id
    );
    for hash in [
        &header.text_sha256,
        &header.normalized_text_sha256,
        &header.source_manifest_sha256,
        &header.features_sha256,
    ] {
        ensure!(
            hash.len() == 64 && hash.bytes().all(|byte| byte.is_ascii_hexdigit()),
            "invalid feature provenance hash"
        );
    }
    Ok(())
}

fn load_features(input: &Path, selected: &[Split]) -> Result<BTreeMap<Split, Vec<FeatureRow>>> {
    let mut result: BTreeMap<Split, Vec<FeatureRow>> = BTreeMap::new();
    let mut ids = BTreeSet::new();
    let mut groups = BTreeMap::new();
    let mut texts = BTreeMap::new();
    for (line_number, line) in BufReader::new(File::open(input)?).lines().enumerate() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        // Inspect identity/split headers for leakage without materializing the
        // held-out test feature payload in the fitting path.
        let header: FeatureHeader = serde_json::from_str(&line)
            .with_context(|| format!("feature header at line {}", line_number + 1))?;
        validate_header(&header)?;
        ensure!(
            ids.insert(header.id.clone()),
            "duplicate feature record ID: {}",
            header.id
        );
        if let Some(previous) = groups.insert(header.group.clone(), header.split) {
            ensure!(
                previous == header.split,
                "feature source group crosses splits: {}",
                header.group
            );
        }
        for hash in [&header.text_sha256, &header.normalized_text_sha256] {
            if let Some(previous) = texts.insert(hash.clone(), header.split) {
                ensure!(
                    previous == header.split,
                    "duplicate feature source crosses splits: {}",
                    header.id
                );
            }
        }
        if !selected.contains(&header.split) {
            continue;
        }
        let record: FeatureRecord = serde_json::from_str(&line)?;
        ensure!(
            record.features.source_sha256 == header.text_sha256,
            "feature/source text hash mismatch for {}",
            header.id
        );
        ensure!(
            sha256(serde_json::to_vec(&record.features)?) == header.features_sha256,
            "feature payload hash mismatch for {}",
            header.id
        );
        result.entry(header.split).or_default().push(FeatureRow {
            id: header.id,
            group: header.group,
            label: header.origin.index(),
            features: record.features,
        });
    }
    for split in selected {
        ensure!(
            result.contains_key(split),
            "no {} feature rows",
            split_name(*split)
        );
    }
    Ok(result)
}

pub fn train(input: &Path, output: &Path, config: &model::TrainConfig) -> Result<Value> {
    let partitions = load_features(
        input,
        &[Split::Train, Split::Development, Split::Calibration],
    )?;
    let (artifact, report) = model::train(
        &partitions[&Split::Train],
        &partitions[&Split::Development],
        &partitions[&Split::Calibration],
        config,
    )?;
    fs::create_dir_all(output)?;
    write_json(&output.join("model.json"), &artifact)?;
    write_json(&output.join("training_report.json"), &report)?;
    let contract = json!({
        "schema":"slop_ninja_detector_runtime_contract_v1", "artifact_sha256":artifact.content_sha256,
        "classes":model::CLASS_NAMES, "model_file_sha256":sha256(fs::read(output.join("model.json"))?),
        "feature_file_sha256":sha256(fs::read(input)?), "input_language_scope":"English; no language detector is fitted",
        "min_words":MIN_WORDS,"max_words":MAX_WORDS,"max_utf8_bytes":MAX_BYTES,"truncation":false,
        "word_tokenizer_feature_version":FEATURE_VERSION,"embedded_bridge_sha256":sha256(BRIDGE),
        "source_payload_sha256":source_payload_hashes(),
        "reference_executable_sha256":sha256(fs::read(std::env::current_exe()?)?),
        "reference_platform":{"os":std::env::consts::OS,"architecture":std::env::consts::ARCH,"crate_version":env!("CARGO_PKG_VERSION")},
        "arithmetic":"Rust f64 CPU; exact strict-greater-than threshold rule; cross-platform numerical qualification remains pending",
        "parser_identity":artifact.content.feature_space.parser_identity,
        "parser_assets":"The parser identity binds every installed model-package file except generated Python bytecode, including weights, config, tokenizer and loader source; pre/post-load hashes must agree.",
        "probability_event":"document production history, not percentage of AI-written words",
        "evidence_policy":"Historical human proxies and model edits of those proxies are weak labels; this pilot is not a documented contemporary collaboration dataset.",
        "test_feature_payloads_used_in_training":false,
        "qualification":"experimental reference control; no Pangram parity or reward qualification claimed"
    });
    let bound_contract =
        json!({"content_sha256":sha256(serde_json::to_vec(&contract)?),"content":contract});
    write_json(&output.join("runtime.json"), &bound_contract)?;
    Ok(serde_json::to_value(report)?)
}

pub fn evaluate(
    input: &Path,
    artifact_path: &Path,
    output: &Path,
    split: Split,
    open_final_test: bool,
) -> Result<Value> {
    ensure!(
        split != Split::Train,
        "use development/calibration diagnostics or explicitly open the final test"
    );
    ensure!(
        split != Split::Test || open_final_test,
        "final test remains sealed; pass --open-final-test for a frozen candidate"
    );
    ensure!(
        !output.exists(),
        "evaluation report already exists; use a new report path"
    );
    let artifact = load_bundle(artifact_path)?;
    let partitions = load_features(input, &[split])?;
    let report = metrics::evaluate(&artifact, &partitions[&split])?;
    let result = json!({"schema":"slop_ninja_detector_evaluation_v1", "split":split,"final_test_opened":split == Split::Test,"feature_file_sha256":sha256(fs::read(input)?),"report":report});
    write_json(output, &result)?;
    Ok(result)
}

pub fn infer(artifact_path: &Path, text_path: &Path, python: Option<&str>) -> Result<Value> {
    let artifact = load_bundle(artifact_path)?;
    let text = fs::read_to_string(text_path)?;
    if let Err(error) = validate_input(&text) {
        return Ok(
            json!({"status":"unsupported_input","artifact_sha256":artifact.content_sha256,"reason":error.to_string()}),
        );
    }
    let features = if artifact.content.feature_space.feature_schema == lexical_schema() {
        lexical_features(&text)
    } else {
        parsed_features(
            &[&text],
            python.context("--python is required by this artifact's parser")?,
        )?
        .remove(0)
    };
    let prediction = artifact.predict_validated(&features)?;
    Ok(
        json!({"status":"ok", "prediction":prediction, "qualification":"experimental reference; accuracy outside its evaluation population is unknown"}),
    )
}

fn source_payload_hashes() -> BTreeMap<&'static str, String> {
    [
        ("detector/main.rs", include_bytes!("main.rs").as_slice()),
        (
            "detector/pipeline.rs",
            include_bytes!("pipeline.rs").as_slice(),
        ),
        (
            "detector/features.rs",
            include_bytes!("features.rs").as_slice(),
        ),
        ("detector/model.rs", include_bytes!("model.rs").as_slice()),
        (
            "detector/metrics.rs",
            include_bytes!("metrics.rs").as_slice(),
        ),
        (
            "detector/Cargo.toml",
            include_bytes!("../Cargo.toml").as_slice(),
        ),
        (
            "detector/Cargo.lock",
            include_bytes!("../Cargo.lock").as_slice(),
        ),
        (
            "grammar/Cargo.lock",
            include_bytes!("../../../grammar/Cargo.lock").as_slice(),
        ),
        (
            "grammar-core/features.rs",
            include_bytes!("../../../grammar/crates/grammar-core/src/features.rs").as_slice(),
        ),
        (
            "grammar-core/syntax.rs",
            include_bytes!("../../../grammar/crates/grammar-core/src/syntax.rs").as_slice(),
        ),
        (
            "grammar-spacy/lib.rs",
            include_bytes!("../../../grammar/crates/grammar-spacy/src/lib.rs").as_slice(),
        ),
        ("grammar-spacy/syntax_bridge.py", BRIDGE.as_bytes()),
    ]
    .into_iter()
    .map(|(name, bytes)| (name, sha256(bytes)))
    .collect()
}

fn load_bundle(artifact_path: &Path) -> Result<model::DetectorArtifact> {
    let artifact_bytes = fs::read(artifact_path)?;
    let artifact: model::DetectorArtifact = serde_json::from_slice(&artifact_bytes)?;
    artifact.validate()?;
    let runtime_path = artifact_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("runtime.json");
    let runtime: Value = serde_json::from_slice(
        &fs::read(&runtime_path)
            .with_context(|| format!("missing runtime contract {}", runtime_path.display()))?,
    )?;
    let content = &runtime["content"];
    ensure!(
        runtime["content_sha256"] == sha256(serde_json::to_vec(content)?),
        "runtime contract content hash mismatch"
    );
    ensure!(
        content["schema"] == "slop_ninja_detector_runtime_contract_v1",
        "unsupported runtime contract schema"
    );
    ensure!(
        content["artifact_sha256"] == artifact.content_sha256
            && content["model_file_sha256"] == sha256(&artifact_bytes),
        "runtime contract binds a different model payload"
    );
    ensure!(
        content["classes"] == json!(model::CLASS_NAMES),
        "runtime class order mismatch"
    );
    ensure!(
        content["parser_identity"] == artifact.content.feature_space.parser_identity,
        "runtime parser identity mismatch"
    );
    ensure!(
        content["min_words"] == MIN_WORDS
            && content["max_words"] == MAX_WORDS
            && content["max_utf8_bytes"] == MAX_BYTES
            && content["truncation"] == false,
        "runtime input contract differs from this executable"
    );
    ensure!(
        content["embedded_bridge_sha256"] == sha256(BRIDGE)
            && content["source_payload_sha256"] == json!(source_payload_hashes()),
        "runtime source/dependency payloads differ from this executable; use the artifact's frozen source revision"
    );
    Ok(artifact)
}

fn output_temp(path: &Path) -> Result<tempfile::NamedTempFile> {
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent)?;
    Ok(tempfile::NamedTempFile::new_in(parent)?)
}

fn write_json(path: &Path, value: &impl Serialize) -> Result<()> {
    let mut file = output_temp(path)?;
    serde_json::to_writer_pretty(&mut file, value)?;
    file.write_all(b"\n")?;
    file.as_file().sync_all()?;
    file.persist_noclobber(path)
        .with_context(|| format!("cannot create {} without overwriting", path.display()))?;
    Ok(())
}

fn write_jsonl(path: &Path, rows: &[impl Serialize]) -> Result<()> {
    let mut file = output_temp(path)?;
    for row in rows {
        serde_json::to_writer(&mut file, row)?;
        file.write_all(b"\n")?;
    }
    file.as_file().sync_all()?;
    file.persist_noclobber(path)
        .with_context(|| format!("cannot create {} without overwriting", path.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn final_test_gate_precedes_any_artifact_or_feature_read() {
        let root = tempfile::tempdir().unwrap();
        let error = evaluate(
            &root.path().join("missing-features.jsonl"),
            &root.path().join("missing-model.json"),
            &root.path().join("report.json"),
            Split::Test,
            false,
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("final test remains sealed"), "{error}");
        assert!(!root.path().join("report.json").exists());
    }

    #[test]
    fn normal_feature_loader_rejects_synthetic_protocol_labels() {
        let root = tempfile::tempdir().unwrap();
        let text = "violet ".repeat(MIN_WORDS);
        let features = lexical_features(&text);
        let fixture = FeatureRecord {
            header: FeatureHeader {
                schema: FEATURE_RECORD_SCHEMA.into(),
                id: "synthetic-contract-fixture".into(),
                group: "synthetic-contract-group".into(),
                origin: Origin::HumanOnly,
                evidence: Evidence::SyntheticFixture,
                split: Split::Train,
                text_sha256: sha256(&text),
                normalized_text_sha256: sha256(words(&text).join(" ")),
                word_count: MIN_WORDS,
                source_manifest_sha256: sha256("synthetic test manifest"),
                features_sha256: sha256(serde_json::to_vec(&features).unwrap()),
            },
            features,
        };
        let path = root.path().join("features.jsonl");
        write_jsonl(&path, &[fixture]).unwrap();
        assert!(
            load_features(&path, &[Split::Train])
                .unwrap_err()
                .to_string()
                .contains("synthetic/unknown evidence")
        );
    }
}
