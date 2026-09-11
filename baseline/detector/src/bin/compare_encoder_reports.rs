//! Paired source-family uncertainty for already frozen encoder evaluations.
use anyhow::{Context, Result, ensure};
use clap::Parser;
use serde_json::{Value, json};
use slop_ninja_detector::{
    dataset::sha256,
    metrics::{self, ProbabilityRow},
};
use std::{collections::BTreeMap, fs, io::Write, path::PathBuf};

#[derive(Parser)]
struct Args {
    #[arg(long)]
    baseline: PathBuf,
    #[arg(long)]
    candidate: PathBuf,
    #[arg(long)]
    output: PathBuf,
}

fn paired(before: &[ProbabilityRow], after: &[ProbabilityRow]) -> Result<Value> {
    metrics::summarize(before)?;
    metrics::summarize(after)?;
    ensure!(before.len() == after.len(), "Prediction counts differ");
    let before: BTreeMap<_, _> = before.iter().map(|r| (r.id.as_str(), r)).collect();
    let after: BTreeMap<_, _> = after.iter().map(|r| (r.id.as_str(), r)).collect();
    let mut groups = BTreeMap::<&str, ([f64; 3], usize)>::new();
    for (id, a) in &before {
        let b = after.get(id).context("Paired prediction ID is missing")?;
        ensure!(
            a.group == b.group && a.label == b.label,
            "Paired family or origin label differs"
        );
        let ma = metrics::summarize(std::slice::from_ref(*a))?;
        let mb = metrics::summarize(std::slice::from_ref(*b))?;
        let delta = [
            mb.log_loss - ma.log_loss,
            mb.multiclass_brier - ma.multiclass_brier,
            mb.accuracy - ma.accuracy,
        ];
        let (sum, n) = groups.entry(&a.group).or_default();
        for i in 0..3 {
            sum[i] += delta[i];
        }
        *n += 1;
    }
    let values: Vec<_> = groups.into_values().collect();
    let mean = |sample: &[([f64; 3], usize)]| -> [f64; 3] {
        let n = sample.iter().map(|v| v.1).sum::<usize>() as f64;
        std::array::from_fn(|i| sample.iter().map(|v| v.0[i]).sum::<f64>() / n)
    };
    let point = mean(&values);
    let mut draws: [Vec<f64>; 3] = std::array::from_fn(|_| Vec::new());
    if values.len() >= 2 {
        let mut rng = 0x243f6a8885a308d3_u64;
        for _ in 0..512 {
            let sample: Vec<_> = (0..values.len())
                .map(|_| {
                    rng = rng.wrapping_add(0x9e3779b97f4a7c15);
                    let mut bits = rng;
                    bits = (bits ^ (bits >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
                    bits = (bits ^ (bits >> 27)).wrapping_mul(0x94d049bb133111eb);
                    bits ^= bits >> 31;
                    values[(bits % values.len() as u64) as usize]
                })
                .collect();
            let estimate = mean(&sample);
            for i in 0..3 {
                draws[i].push(estimate[i]);
            }
        }
    }
    let mut deltas = serde_json::Map::new();
    for (i, name) in ["log_loss", "multiclass_brier", "accuracy"]
        .iter()
        .enumerate()
    {
        draws[i].sort_by(f64::total_cmp);
        let (interval, status) = if draws[i].is_empty() {
            (None, "insufficient_source_groups")
        } else if (draws[i][499] - draws[i][12]).abs() < 1e-12 {
            (None, "degenerate_observed_outcomes")
        } else {
            (
                Some([draws[i][12], draws[i][499]]),
                "paired_source_group_percentile",
            )
        };
        deltas.insert(
            (*name).into(),
            json!({"candidate_minus_baseline":point[i],
            "group_bootstrap_percentile_95":interval,"uncertainty_status":status,
            "improvement_direction":if i == 2 {"positive"} else {"negative"}}),
        );
    }
    Ok(
        json!({"rows":before.len(),"source_groups":values.len(),"bootstrap_replicates":draws[0].len(),
        "bootstrap_seed":"243f6a8885a308d3","deltas":deltas,
        "method":"Sample complete source families with replacement, using the same draw for both detectors; average all sampled rows. Sorted IDs/groups and a fixed SplitMix64 stream make the calculation reproducible.",
        "limits":"Intervals assume independent source families and do not cover arbitrary new registers, generators or pretraining contamination. Degenerate intervals are omitted. Different frozen thresholds do not establish matched-FPR superiority."}),
    )
}

fn main() -> Result<()> {
    let args = Args::parse();
    ensure!(!args.output.exists(), "Use a new comparison output");
    let baseline_bytes = fs::read(&args.baseline)?;
    let candidate_bytes = fs::read(&args.candidate)?;
    let a: Value = serde_json::from_slice(&baseline_bytes)?;
    let b: Value = serde_json::from_slice(&candidate_bytes)?;
    for report in [&a, &b] {
        ensure!(
            report["schema"] == "slop_ninja_frozen_encoder_evaluation_v1",
            "Unexpected report schema"
        );
        ensure!(
            report["artifact_id"] == report["report"]["artifact_sha256"],
            "Artifact identity mismatch"
        );
        ensure!(
            report["split"] == "development"
                || (report["split"] == "test" && report["final_test_opened"] == true),
            "Unexpected evaluation partition or Test state"
        );
    }
    ensure!(
        a["records_sha256"] == b["records_sha256"] && a["split"] == b["split"],
        "Corpora or partitions differ"
    );
    ensure!(
        a["artifact_id"] != b["artifact_id"],
        "Comparison requires distinct detectors"
    );
    let before: Vec<ProbabilityRow> = serde_json::from_value(a["report"]["predictions"].clone())?;
    let after: Vec<ProbabilityRow> = serde_json::from_value(b["report"]["predictions"].clone())?;
    let comparison = paired(&before, &after)?;
    let report = json!({"schema":"slop_ninja_paired_encoder_comparison_v1",
        "baseline_report_sha256":sha256(baseline_bytes),"candidate_report_sha256":sha256(candidate_bytes),
        "baseline_artifact_id":a["artifact_id"],"candidate_artifact_id":b["artifact_id"],
        "records_sha256":a["records_sha256"],"split":a["split"],"comparison":comparison,
        "baseline_metrics":a["report"]["model"],"candidate_metrics":b["report"]["model"],
        "baseline_operating_points":a["report"]["operating_points"],"candidate_operating_points":b["report"]["operating_points"],
        "baseline_near_cutoffs":a["near_cutoffs"],"candidate_near_cutoffs":b["near_cutoffs"]});
    let mut output = fs::File::create_new(args.output)?;
    serde_json::to_writer_pretty(&mut output, &report)?;
    output.write_all(b"\n")?;
    println!("{}", serde_json::to_string_pretty(&comparison)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paired_rows_require_matching_families_and_labels_and_omit_degenerate_intervals() {
        let before: Vec<_> = (0..6)
            .map(|i| ProbabilityRow {
                id: format!("synthetic-{i}"),
                group: format!("synthetic-family-{}", i / 3),
                label: i % 3,
                probabilities: std::array::from_fn(|j| if j == i % 3 { 0.6 } else { 0.2 }),
            })
            .collect();
        let after: Vec<_> = before
            .iter()
            .map(|r| ProbabilityRow {
                probabilities: std::array::from_fn(|j| if j == r.label { 0.8 } else { 0.1 }),
                ..r.clone()
            })
            .collect();
        let result = paired(&before, &after).unwrap();
        assert!(
            (result["deltas"]["log_loss"]["candidate_minus_baseline"]
                .as_f64()
                .unwrap()
                - (0.6_f64 / 0.8).ln())
            .abs()
                < 1e-12
        );
        assert_eq!(
            result["deltas"]["accuracy"]["uncertainty_status"],
            "degenerate_observed_outcomes"
        );
        let mut changed = after.clone();
        changed[0].group = "wrong-family".into();
        assert!(paired(&before, &changed).is_err());
        changed[0] = after[0].clone();
        changed[0].label = 1;
        assert!(paired(&before, &changed).is_err());
    }
}
