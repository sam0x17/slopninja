//! Run a bounded local MLX campaign, retaining each server and generation log.
use anyhow::{Context, Result, ensure};
use clap::Parser;
use serde::Deserialize;
use serde_json::{Value, json};
use slop_ninja_detector::dataset::{self, Split, sha256};
use slop_ninja_detector::generation::{self, RevisionStyle};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    net::TcpListener,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    thread,
    time::{Duration, Instant},
};

#[derive(Parser)]
struct Args {
    #[arg(long)]
    plan: PathBuf,
    #[arg(long)]
    output_dir: PathBuf,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Plan {
    input: PathBuf,
    input_sha256: String,
    generator: PathBuf,
    generator_sha256: String,
    python: PathBuf,
    runtime_files: BTreeMap<PathBuf, String>,
    protocol: PathBuf,
    protocol_sha256: String,
    profile_ids: Vec<String>,
    cases: Vec<Case>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Case {
    name: String,
    model_dir: PathBuf,
    model_spec: PathBuf,
    model_spec_sha256: String,
    server_script: Option<PathBuf>,
    port: u16,
    split: Option<Split>,
    max_calls: usize,
    /// Absent for initial draft/edit pairs; present for one call per model-only leaf.
    revision_style: Option<RevisionStyle>,
}

struct Server(Child);
impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

fn check_hash(path: &Path, expected: &str) -> Result<()> {
    ensure!(
        sha256(fs::read(path)?) == expected,
        "Input changed: {}",
        path.display()
    );
    Ok(())
}

fn check_inputs(plan: &Plan) -> Result<()> {
    check_hash(&plan.input, &plan.input_sha256)?;
    check_hash(&plan.generator, &plan.generator_sha256)?;
    check_hash(&plan.protocol, &plan.protocol_sha256)?;
    for (path, hash) in &plan.runtime_files {
        check_hash(path, hash)?;
    }
    for case in &plan.cases {
        check_hash(&case.model_spec, &case.model_spec_sha256)?;
    }
    Ok(())
}

fn invocation(command: &Command) -> Value {
    json!({"program": command.get_program().to_string_lossy(),
        "args": command.get_args().map(|arg| arg.to_string_lossy()).collect::<Vec<_>>()})
}

fn write_json(path: &Path, value: &Value) -> Result<()> {
    fs::write(path, serde_json::to_vec_pretty(value)?)?;
    Ok(())
}

fn generation_command(plan: &Plan, case: &Case, cohort: &Path, base_url: &str) -> Result<Command> {
    let mut generate = Command::new(&plan.generator);
    generate
        .arg("--input")
        .arg(&plan.input)
        .arg("--model-spec")
        .arg(&case.model_spec)
        .arg("--output-dir")
        .arg(cohort)
        .arg("--base-url")
        .arg(base_url)
        .args(["--concurrency", "1", "--max-calls"])
        .arg(case.max_calls.to_string());
    if let Some(style) = case.revision_style {
        ensure!(
            plan.profile_ids.is_empty(),
            "Revision campaigns require empty profile_ids"
        );
        generate
            .args(["--model-revisions", "--revision-style"])
            .arg(serde_json::to_value(style)?.as_str().unwrap());
    } else {
        generate
            .args([
                "--prompt-profile-set",
                "style-mix-v1",
                "--prompt-profile-ids",
            ])
            .arg(plan.profile_ids.join(","));
    }
    generate.arg("--complete-families-only");
    if let Some(split) = case.split {
        generate
            .arg("--split")
            .arg(serde_json::to_value(split)?.as_str().unwrap());
    }
    Ok(generate)
}

fn request_count(plan: &Plan, case: &Case, records: &[dataset::OriginRecord]) -> Result<usize> {
    let count = if case.revision_style.is_some() {
        ensure!(
            plan.profile_ids.is_empty(),
            "Revision campaigns require empty profile_ids"
        );
        generation::revision_parents(records)?
            .iter()
            .filter(|r| case.split.is_none() || r.split == case.split)
            .count()
    } else {
        ensure!(
            records
                .iter()
                .all(|r| r.parent_id.is_none() && r.split.is_some()),
            "Initial generation requires frozen source roots"
        );
        2 * records
            .iter()
            .filter(|r| case.split.is_none() || r.split == case.split)
            .count()
    };
    ensure!(
        count > 0 && case.max_calls == count,
        "Call cap must equal the selected request count ({count})"
    );
    Ok(count)
}

fn run_case(plan: &Plan, case: &Case, output: &Path) -> Result<Value> {
    check_inputs(plan)?;
    // Never adopt an unrelated listener or send a request to its default model.
    let port_guard = TcpListener::bind(("127.0.0.1", case.port))
        .context("Campaign port already occupied; inspect its owner")?;
    let spec: Value = serde_json::from_slice(&fs::read(&case.model_spec)?)?;
    let max_tokens = spec["max_tokens"]
        .as_u64()
        .context("Missing generation token cap")?;
    let mut server_command = Command::new(&plan.python);
    if let Some(script) = &case.server_script {
        ensure!(
            plan.runtime_files.contains_key(script),
            "Unpinned server adapter"
        );
        server_command.arg(script);
    } else {
        server_command.args(["-m", "mlx_lm.server"]);
    }
    server_command
        .arg("--model")
        .arg(&case.model_dir)
        .args(["--host", "127.0.0.1", "--port"])
        .arg(case.port.to_string())
        .arg("--allowed-origins")
        .arg(format!("http://127.0.0.1:{}", case.port))
        .args([
            "--decode-concurrency",
            "1",
            "--prompt-concurrency",
            "1",
            "--prompt-cache-size",
            "0",
            "--max-tokens",
        ])
        .arg(max_tokens.to_string())
        .env("HF_HUB_OFFLINE", "1")
        .env("TRANSFORMERS_OFFLINE", "1")
        .env("PYTHONUNBUFFERED", "1");
    write_json(
        &output.join(format!("{}-server-invocation.json", case.name)),
        &invocation(&server_command),
    )?;
    let server_log = fs::File::create_new(output.join(format!("{}-server.log", case.name)))?;
    drop(port_guard);
    let mut server = Server(
        server_command
            .stdin(Stdio::null())
            .stdout(server_log.try_clone()?)
            .stderr(server_log)
            .spawn()?,
    );
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?;
    let base_url = format!("http://127.0.0.1:{}/v1", case.port);
    let started = Instant::now();
    loop {
        ensure!(
            server.0.try_wait()?.is_none(),
            "Model server exited before readiness"
        );
        if let Ok(response) = client.get(format!("{base_url}/models")).send()
            && response.status().is_success()
            && let Ok(catalog) = response.json::<Value>()
            && catalog["data"]
                .as_array()
                .is_some_and(|items| !items.is_empty())
        {
            write_json(
                &output.join(format!("{}-readiness.json", case.name)),
                &json!({"pid":server.0.id(),"seconds":started.elapsed().as_secs_f64(),"catalog":catalog}),
            )?;
            break;
        }
        ensure!(
            started.elapsed() < Duration::from_secs(120),
            "Model server readiness timed out"
        );
        thread::sleep(Duration::from_secs(1));
    }
    let cohort = output.join(&case.name);
    let mut generate = generation_command(plan, case, &cohort, &base_url)?;
    write_json(
        &output.join(format!("{}-generation-invocation.json", case.name)),
        &invocation(&generate),
    )?;
    let generation_log =
        fs::File::create_new(output.join(format!("{}-generation.log", case.name)))?;
    println!(
        "Generating {}: at most {} local calls",
        case.name, case.max_calls
    );
    let status = generate
        .stdin(Stdio::null())
        .stdout(generation_log.try_clone()?)
        .stderr(generation_log)
        .status()?;
    ensure!(
        status.success(),
        "Generation failed for {}; preserve its logs and responses",
        case.name
    );
    let summary_bytes = fs::read(cohort.join("summary.json"))?;
    let summary: Value = serde_json::from_slice(&summary_bytes)?;
    ensure!(
        summary["requested"] == case.max_calls,
        "Generator task count differs from the frozen campaign"
    );
    ensure!(
        summary["status"] == "paused"
            || (summary["status"] == "complete" && summary["complete_export_ready"] == true),
        "Generation did not produce a terminal or paused cohort"
    );
    check_inputs(plan)?;
    let mut result = json!({"name":case.name,"status":summary["status"],"requested":summary["requested"],
        "completed":summary["completed"],"excluded_root_count":summary["excluded_root_count"],
        "summary_sha256":sha256(summary_bytes),"seconds":started.elapsed().as_secs_f64()});
    if let Some(style) = case.revision_style {
        ensure!(
            summary["cohort_kind"] == "model_only_revision_chains",
            "Expected a model-only revision cohort"
        );
        result["revision_style"] = serde_json::to_value(style)?;
        result["excluded_source_group_count"] = summary["excluded_source_group_count"].clone();
    }
    Ok(result)
}

fn main() -> Result<()> {
    let args = Args::parse();
    ensure!(
        !args.output_dir.exists(),
        "Use a new campaign directory; resume individual cohorts explicitly"
    );
    let plan_bytes = fs::read(&args.plan)?;
    let plan: Plan = serde_json::from_slice(&plan_bytes)?;
    check_inputs(&plan)?;
    let records = dataset::read_records(&plan.input)?;
    ensure!(
        (1..=4).contains(&plan.cases.len()),
        "Declare 1..4 local models"
    );
    let mut names = BTreeSet::new();
    for case in &plan.cases {
        ensure!(
            !case.name.is_empty()
                && case
                    .name
                    .bytes()
                    .all(|c| c.is_ascii_alphanumeric() || c == b'-'),
            "Unsafe case name"
        );
        ensure!(names.insert(&case.name), "Duplicate case name");
        request_count(&plan, case, &records)?;
    }
    fs::create_dir(&args.output_dir)?;
    fs::write(args.output_dir.join("plan.json"), &plan_bytes)?;
    let state_path = args.output_dir.join("status.json");
    let mut results = Vec::new();
    for case in &plan.cases {
        if args.output_dir.join("PAUSE").exists() {
            write_json(
                &state_path,
                &json!({"status":"paused","results":results,"pangram_calls":0}),
            )?;
            return Ok(());
        }
        write_json(
            &state_path,
            &json!({"status":"running","case":case.name,"results":results,"pangram_calls":0}),
        )?;
        match run_case(&plan, case, &args.output_dir) {
            Ok(result) => {
                let paused = result["status"] == "paused";
                results.push(result);
                if paused {
                    write_json(
                        &state_path,
                        &json!({"status":"paused","results":results,"pangram_calls":0}),
                    )?;
                    return Ok(());
                }
            }
            Err(error) => {
                write_json(
                    &state_path,
                    &json!({"status":"failed","case":case.name,"error":format!("{error:#}"),"results":results,"pangram_calls":0}),
                )?;
                return Err(error);
            }
        }
    }
    write_json(
        &state_path,
        &json!({"status":"complete","results":results,"pangram_calls":0,"detector_predictions_opened":false}),
    )?;
    println!("Generation campaign complete; no detector was fitted or evaluated");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn plan() -> Plan {
        serde_json::from_value(json!({
            "input":"source.jsonl", "input_sha256":"unused", "generator":"generator",
            "generator_sha256":"unused", "python":"python", "runtime_files":{},
            "protocol":"protocol.md", "protocol_sha256":"unused", "profile_ids":["anti-ai"],
            "cases":[{"name":"test", "model_dir":"weights", "model_spec":"model.json",
                "model_spec_sha256":"unused", "port":18123, "split":"test", "max_calls":234}]
        }))
        .unwrap()
    }

    #[test]
    fn legacy_and_revision_commands_keep_their_distinct_prompt_contracts() -> Result<()> {
        let mut plan = plan();
        let cohort = Path::new("cohort");
        let base = "http://127.0.0.1:18123/v1";
        let expected_prefix = vec![
            "--input",
            "source.jsonl",
            "--model-spec",
            "model.json",
            "--output-dir",
            "cohort",
            "--base-url",
            base,
            "--concurrency",
            "1",
            "--max-calls",
        ];
        let mut legacy = expected_prefix.clone();
        legacy.extend([
            "234",
            "--prompt-profile-set",
            "style-mix-v1",
            "--prompt-profile-ids",
            "anti-ai",
            "--complete-families-only",
            "--split",
            "test",
        ]);
        assert_eq!(
            invocation(&generation_command(&plan, &plan.cases[0], cohort, base)?)["args"],
            json!(legacy)
        );
        plan.cases[0].revision_style = Some(RevisionStyle::AntiAi);
        assert!(generation_command(&plan, &plan.cases[0], cohort, base).is_err());
        plan.profile_ids.clear();
        plan.cases[0].max_calls = 117;
        for (style, name) in [
            (RevisionStyle::AntiAi, "anti-ai"),
            (RevisionStyle::FixSlop, "fix-slop"),
        ] {
            plan.cases[0].revision_style = Some(style);
            let mut expected = expected_prefix.clone();
            expected.extend([
                "117",
                "--model-revisions",
                "--revision-style",
                name,
                "--complete-families-only",
                "--split",
                "test",
            ]);
            assert_eq!(
                invocation(&generation_command(&plan, &plan.cases[0], cohort, base)?)["args"],
                json!(expected)
            );
        }
        Ok(())
    }

    #[test]
    #[ignore = "requires a closed local revision archive and its frozen input"]
    fn archived_revision_campaign_counts_only_the_next_stage() -> Result<()> {
        let archive = PathBuf::from(std::env::var("SLOP_NINJA_REVISION_ARCHIVE")?);
        let input = PathBuf::from(std::env::var("SLOP_NINJA_REVISION_INPUT")?);
        ensure!(!archive.join("run.lock").exists(), "Archive is still open");
        let run: Value = serde_json::from_slice(&fs::read(archive.join("run.json"))?)?;
        let summary: Value = serde_json::from_slice(&fs::read(archive.join("summary.json"))?)?;
        ensure!(summary["status"] == "complete", "Archive is incomplete");
        ensure!(
            run["input_sha256"] == sha256(fs::read(&input)?),
            "Wrong input"
        );
        let records = dataset::read_records(&input)?;
        let mut plan = plan();
        plan.profile_ids.clear();
        plan.cases[0].revision_style = Some(serde_json::from_value(run["revision_style"].clone())?);
        plan.cases[0].split = serde_json::from_value(run["split"].clone())?;
        let expected = summary["requested"]
            .as_u64()
            .context("Missing task count")? as usize;
        plan.cases[0].max_calls = expected;
        assert_eq!(request_count(&plan, &plan.cases[0], &records)?, expected);
        plan.cases[0].max_calls += 1;
        assert!(request_count(&plan, &plan.cases[0], &records).is_err());
        plan.cases[0].max_calls = expected;
        plan.cases[0].revision_style = None;
        assert!(request_count(&plan, &plan.cases[0], &records).is_err());
        Ok(())
    }
}
