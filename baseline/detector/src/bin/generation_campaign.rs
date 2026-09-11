//! Run a bounded local MLX campaign, retaining each server and generation log.
use anyhow::{Context, Result, ensure};
use clap::Parser;
use serde::Deserialize;
use serde_json::{Value, json};
use slop_ninja_detector::dataset::{self, Split, sha256};
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
    let mut generate = Command::new(&plan.generator);
    generate
        .arg("--input")
        .arg(&plan.input)
        .arg("--model-spec")
        .arg(&case.model_spec)
        .arg("--output-dir")
        .arg(&cohort)
        .arg("--base-url")
        .arg(&base_url)
        .args(["--concurrency", "1", "--max-calls"])
        .arg(case.max_calls.to_string())
        .args([
            "--prompt-profile-set",
            "style-mix-v1",
            "--prompt-profile-ids",
        ])
        .arg(plan.profile_ids.join(","))
        .arg("--complete-families-only");
    if let Some(split) = case.split {
        generate
            .arg("--split")
            .arg(serde_json::to_value(split)?.as_str().unwrap());
    }
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
        summary["status"] == "paused"
            || (summary["status"] == "complete" && summary["complete_export_ready"] == true),
        "Generation did not produce a terminal or paused cohort"
    );
    check_inputs(plan)?;
    Ok(
        json!({"name":case.name,"status":summary["status"],"requested":summary["requested"],
        "completed":summary["completed"],"excluded_root_count":summary["excluded_root_count"],
        "summary_sha256":sha256(summary_bytes),"seconds":started.elapsed().as_secs_f64()}),
    )
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
    let roots = dataset::read_records(&plan.input)?;
    ensure!(
        roots
            .iter()
            .all(|r| r.parent_id.is_none() && r.split.is_some()),
        "Use frozen source roots"
    );
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
        let count = roots
            .iter()
            .filter(|r| case.split.is_none() || r.split == case.split)
            .count();
        ensure!(
            count > 0 && case.max_calls == 2 * count,
            "Call cap must equal one draft/edit pair per selected source"
        );
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
