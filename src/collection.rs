//! Attributed model samples through explicit APIs or installed authenticated CLIs.
use crate::util::{digest, read, read_jsonl, write_json};
use anyhow::{Context, Result, bail, ensure};
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Write,
    path::Path,
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};

pub fn load_prompts(path: &Path) -> Result<Vec<Value>> {
    let prompts = read_jsonl(path)?;
    ensure!(!prompts.is_empty(), "Prompt manifest is empty");
    let mut ids = BTreeSet::new();
    let mut groups = BTreeMap::new();
    for p in &prompts {
        for k in ["id", "prompt", "domain", "register", "group_id"] {
            ensure!(
                p[k].as_str().is_some_and(|s| !s.trim().is_empty()),
                "Prompt requires {k}"
            );
        }
        let id = p["id"].as_str().unwrap();
        ensure!(ids.insert(id), "Duplicate prompt ID {id}");
        let split = p["split"].as_str().unwrap_or("train");
        ensure!(
            ["train", "dev", "test", "exploratory"].contains(&split),
            "Invalid split"
        );
        let group = p["group_id"].as_str().unwrap();
        if let Some(old) = groups.insert(group, split) {
            ensure!(old == split, "Source family crosses splits");
        }
    }
    Ok(prompts)
}

pub fn parse_cli(
    provider: &str,
    requested: &str,
    events: &[Value],
) -> Result<(String, String, Value)> {
    if provider == "codex-cli" {
        ensure!(
            events.iter().any(|e| e["type"] == "turn.completed"),
            "Codex turn did not complete"
        );
        ensure!(
            !events
                .iter()
                .any(|e| e["type"] == "turn.failed" || e["type"] == "error"),
            "Codex reported a failed turn"
        );
        let mut messages = Vec::new();
        for event in events {
            if event["type"]
                .as_str()
                .is_some_and(|s| s.starts_with("item."))
            {
                let kind = event["item"]["type"].as_str().unwrap_or("");
                ensure!(
                    ["agent_message", "reasoning"].contains(&kind),
                    "Codex used a tool or non-prose item: {kind}"
                );
                if event["type"] == "item.completed" && kind == "agent_message" {
                    messages.push(
                        event["item"]["text"]
                            .as_str()
                            .context("Missing Codex text")?,
                    );
                }
            }
        }
        ensure!(
            messages.len() == 1,
            "Expected one final Codex prose message"
        );
        Ok((
            messages[0].to_string(),
            format!("requested:{requested}"),
            json!({
            "model_identity_source":"explicit_cli_request; resolved_model_not_reported",
            "usage":events.iter().find(|e|e["type"]=="turn.completed").map(|e|&e["usage"]),
            "session_id":events.iter().find(|e|e["type"]=="thread.started").map(|e|&e["thread_id"])}),
        ))
    } else {
        let result = events
            .iter()
            .rev()
            .find(|e| e["type"] == "result")
            .context("No Claude result")?;
        ensure!(
            result["subtype"] == "success" && result["is_error"] == false,
            "Claude did not complete successfully"
        );
        let init = events
            .iter()
            .find(|e| e["type"] == "system" && e["subtype"] == "init")
            .context("No Claude init metadata")?;
        ensure!(
            init["tools"].as_array().is_some_and(|a| a.is_empty()),
            "Claude tools were not disabled"
        );
        ensure!(
            init["mcp_servers"].as_array().is_some_and(|a| a.is_empty()),
            "Claude MCP servers were not disabled"
        );
        let mut models = BTreeSet::new();
        for e in events.iter().filter(|e| e["type"] == "assistant") {
            if let Some(model) = e["message"]["model"].as_str() {
                models.insert(model);
            }
            for block in e["message"]["content"]
                .as_array()
                .context("Missing Claude content")?
            {
                ensure!(
                    block["type"] != "fallback",
                    "Claude switched models; keep this invocation separate from the requested model corpus"
                );
                ensure!(
                    ["text", "thinking", "redacted_thinking"]
                        .contains(&block["type"].as_str().unwrap_or("")),
                    "Claude used a tool/non-prose output"
                );
            }
        }
        ensure!(
            models.len() == 1,
            "Claude must report exactly one resolved model"
        );
        if requested.starts_with("claude-") {
            ensure!(
                models.contains(requested),
                "Claude returned a different model than explicitly requested"
            );
        }
        Ok((
            result["result"]
                .as_str()
                .context("Missing Claude prose")?
                .to_string(),
            models.into_iter().next().unwrap().to_string(),
            json!({
            "model_identity_source":"assistant_message.model","usage":result["usage"],"model_usage":result["modelUsage"],
            "reported_cost_usd":result["total_cost_usd"],"session_id":result["session_id"],"init":init}),
        ))
    }
}

fn collect_cli(
    prompt: &Value,
    provider: &str,
    model: &str,
    run_dir: &Path,
    timeout: Duration,
) -> Result<Value> {
    fs::create_dir_all(run_dir)?;
    let work = run_dir.join("workdir");
    fs::create_dir_all(&work)?;
    let program = if provider == "codex-cli" {
        "codex"
    } else {
        "claude"
    };
    let version = Command::new(program)
        .arg("--version")
        .output()
        .context("Read CLI version")?;
    ensure!(version.status.success(), "Cannot identify CLI version");
    let args: Vec<String> = if provider == "codex-cli" {
        [
            "exec",
            "--ignore-user-config",
            "--ephemeral",
            "--skip-git-repo-check",
            "--sandbox",
            "read-only",
            "--model",
            model,
            "-c",
            "model_reasoning_effort=\"low\"",
            "--json",
            "-",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    } else {
        [
            "-p",
            "--safe-mode",
            "--tools",
            "",
            "--no-session-persistence",
            "--max-turns",
            "1",
            "--model",
            model,
            "--output-format",
            "stream-json",
            "--verbose",
            "--effort",
            "low",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    };
    let stdout = fs::File::create(run_dir.join("stdout.jsonl"))?;
    let stderr = fs::File::create(run_dir.join("stderr.txt"))?;
    let mut child = Command::new(program)
        .args(&args)
        .current_dir(work)
        .stdin(Stdio::piped())
        .stdout(stdout)
        .stderr(stderr)
        .spawn()?;
    let mut input = child.stdin.take().context("Missing CLI stdin")?;
    input.write_all(prompt["prompt"].as_str().unwrap().as_bytes())?;
    drop(input);
    let deadline = Instant::now() + timeout;
    let exit = loop {
        if let Some(status) = child.try_wait()? {
            break status;
        }
        if Instant::now() >= deadline {
            child.kill()?;
            child.wait()?;
            bail!("CLI timed out; output preserved and invocation was not retried");
        }
        thread::sleep(Duration::from_millis(100));
    };
    ensure!(
        exit.success(),
        "CLI exited unsuccessfully; inspect captured output in {}",
        run_dir.display()
    );
    let events = read_jsonl(&run_dir.join("stdout.jsonl"))?;
    let (text, reported, metadata) = parse_cli(provider, model, &events)?;
    ensure!(!text.trim().is_empty(), "CLI returned empty prose");
    Ok(json!({"text":text,"model":reported,"metadata":{
        "transport":"authenticated_cli","cli_version":String::from_utf8(version.stdout)?.trim(),
        "argv":args,"events":events,"generation_metadata":metadata,
        "cli_internal_retries":"Controlled by CLI; this harness invokes once and records the event stream"}}))
}

fn collect_api(
    prompt: &Value,
    provider: &str,
    model: &str,
    max_tokens: u64,
    timeout: Duration,
    run_dir: &Path,
) -> Result<Value> {
    let (url, keyname) = match provider {
        "openai" => ("https://api.openai.com/v1/responses", "OPENAI_API_KEY"),
        "anthropic" => ("https://api.anthropic.com/v1/messages", "ANTHROPIC_API_KEY"),
        _ => bail!("Unknown provider"),
    };
    let key = std::env::var(keyname).with_context(|| format!("Set {keyname}"))?;
    ensure!(!key.trim().is_empty(), "Set {keyname}");
    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .retry(reqwest::retry::never())
        .timeout(timeout)
        .build()?;
    let mut req = client.post(url).bearer_auth(&key);
    let payload = if provider == "openai" {
        json!({"model":model,"input":prompt["prompt"],"store":false,"max_output_tokens":max_tokens})
    } else {
        req = req.header("anthropic-version", "2023-06-01");
        if let Ok(id) = std::env::var("ANTHROPIC_WORKSPACE_ID") {
            req = req.header("anthropic-workspace-id", id);
        }
        json!({"model":model,"messages":[{"role":"user","content":prompt["prompt"]}],"max_tokens":max_tokens})
    };
    let response = req
        .json(&payload)
        .send()
        .map_err(|_| anyhow::anyhow!("Provider transport failed; no automatic retry"))?;
    ensure!(
        response.status().is_success(),
        "Provider HTTP {}; not retried",
        response.status()
    );
    let response: Value = response.json().context("Invalid provider JSON")?;
    // Keep the complete billed response, including usage and incomplete/refusal
    // details, even when the following corpus acceptance checks reject it.
    write_json(&run_dir.join("response.json"), &response)?;
    let model = response["model"]
        .as_str()
        .context("Provider omitted resolved model")?;
    ensure!(
        !model.trim().is_empty()
            && response["id"]
                .as_str()
                .is_some_and(|s| !s.trim().is_empty()),
        "Provider must report nonempty model and response ID"
    );
    let mut text = String::new();
    if provider == "openai" {
        ensure!(
            response["status"] == "completed" && response["error"].is_null(),
            "OpenAI generation incomplete or rejected"
        );
        for item in response["output"]
            .as_array()
            .context("Missing OpenAI output")?
        {
            if item["type"] == "reasoning" {
                continue;
            }
            ensure!(
                item["type"] == "message"
                    && item["role"] == "assistant"
                    && item["status"] == "completed",
                "Non-prose OpenAI output"
            );
            for part in item["content"].as_array().context("Missing content")? {
                ensure!(part["type"] == "output_text", "Refusal or non-text output");
                text.push_str(part["text"].as_str().context("Missing text")?);
            }
        }
    } else {
        ensure!(
            response["stop_reason"] == "end_turn"
                && response["role"] == "assistant"
                && response["type"] == "message",
            "Anthropic generation incomplete"
        );
        ensure!(
            response["stop_details"]["type"] != "refusal",
            "Anthropic refused"
        );
        for part in response["content"]
            .as_array()
            .context("Missing Anthropic content")?
        {
            if part["type"] == "thinking" || part["type"] == "redacted_thinking" {
                continue;
            }
            ensure!(part["type"] == "text", "Non-prose Anthropic output");
            text.push_str(part["text"].as_str().context("Missing text")?);
        }
    }
    ensure!(!text.trim().is_empty(), "Empty provider prose");
    Ok(
        json!({"text":text,"model":model,"metadata":{"transport":"api","request_parameters":payload,"response":response,"model_identity_source":"response.model"}}),
    )
}

fn validate_cached(
    record: &Value,
    prompt: &Value,
    provider: &str,
    model: &str,
    corpus: &str,
) -> Result<()> {
    ensure!(
        record["status"] == "accepted",
        "Uncertain/rejected previous invocation; inspect before any resubmission"
    );
    let doc = &record["document"];
    ensure!(
        record["prompt"] == *prompt
            && record["provider"] == provider
            && record["model_requested"] == model,
        "Cached collection request provenance mismatch"
    );
    ensure!(
        doc["corpus"] == corpus
            && doc["source_kind"] == "model"
            && doc["provider"] == provider
            && doc["model_requested"] == model,
        "Cached document model/corpus provenance mismatch"
    );
    for key in ["domain", "register", "group_id"] {
        ensure!(doc[key] == prompt[key], "Cached document {key} mismatch");
    }
    ensure!(
        doc["split"] == prompt.get("split").cloned().unwrap_or(json!("train")),
        "Cached document split mismatch"
    );
    let text = doc["text"]
        .as_str()
        .context("Cached document text missing")?;
    ensure!(
        !text.trim().is_empty() && doc["model"].as_str().is_some_and(|s| !s.trim().is_empty()),
        "Cached text/model empty"
    );
    ensure!(
        doc["metadata"]["text_sha256"] == digest(text)
            && doc["metadata"]["prompt_record"] == *prompt,
        "Cached text hash or prompt mismatch"
    );
    Ok(())
}

pub fn collect_batch(
    prompts_path: &Path,
    provider: &str,
    model: &str,
    corpus: &str,
    out: &Path,
    max_requests: usize,
    max_tokens: u64,
) -> Result<Value> {
    ensure!(
        ["openai", "anthropic", "codex-cli", "claude-cli"].contains(&provider),
        "Unknown provider"
    );
    ensure!(
        !model.trim().is_empty() && !corpus.trim().is_empty() && max_tokens > 0,
        "Model, corpus and output limit required"
    );
    let prompts = load_prompts(prompts_path)?;
    let runs = out.with_extension("runs");
    fs::create_dir_all(&runs)?;
    let manifest = json!({"provider":provider,"model":model,"corpus":corpus,"max_output_tokens":max_tokens,"prompts":prompts});
    let manifest_path = runs.join("manifest.json");
    if manifest_path.exists() {
        ensure!(
            serde_json::from_str::<Value>(&read(&manifest_path)?)? == manifest,
            "Existing collection has different inputs"
        );
    } else {
        ensure!(
            !out.exists(),
            "Output already exists without its collection manifest"
        );
        write_json(&manifest_path, &manifest)?;
    }
    let mut missing = 0;
    for prompt in &prompts {
        let path = runs
            .join(digest(prompt["id"].as_str().unwrap()))
            .join("record.json");
        if path.exists() {
            let r: Value = serde_json::from_str(&read(&path)?)?;
            validate_cached(&r, prompt, provider, model, corpus)?;
        } else {
            missing += 1;
        }
    }
    ensure!(
        missing <= max_requests,
        "Needs {missing} new invocations but budget is {max_requests}"
    );
    let mut docs = Vec::new();
    for prompt in &prompts {
        let dir = runs.join(digest(prompt["id"].as_str().unwrap()));
        let path = dir.join("record.json");
        let record = if path.exists() {
            serde_json::from_str::<Value>(&read(&path)?)?
        } else {
            let mut intent = json!({"status":"submission_uncertain","provider":provider,"model_requested":model,
                "requested_at":chrono::Utc::now().to_rfc3339(),"prompt":prompt});
            write_json(&path, &intent)?;
            let collected = if provider.ends_with("-cli") {
                collect_cli(prompt, provider, model, &dir, Duration::from_secs(300))
            } else {
                collect_api(
                    prompt,
                    provider,
                    model,
                    max_tokens,
                    Duration::from_secs(180),
                    &dir,
                )
            };
            match collected {
                Ok(sample) => {
                    let mut metadata = sample["metadata"].clone();
                    metadata["prompt_record"] = prompt.clone();
                    metadata["model_requested"] = json!(model);
                    metadata["requested_at"] = intent["requested_at"].clone();
                    metadata["received_at"] = json!(chrono::Utc::now().to_rfc3339());
                    metadata["collector_version"] = json!("rust-v1");
                    metadata["text_sha256"] = json!(digest(sample["text"].as_str().unwrap()));
                    let doc = json!({"text":sample["text"],"model":sample["model"],"corpus":corpus,"source_kind":"model","provider":provider,
                        "model_requested":model,"domain":prompt["domain"],"register":prompt["register"],"group_id":prompt["group_id"],
                        "split":prompt.get("split").cloned().unwrap_or(json!("train")),"metadata":metadata});
                    intent["status"] = json!("accepted");
                    intent["document"] = doc;
                    write_json(&path, &intent)?;
                }
                Err(e) => {
                    intent["status"] = json!("failed_or_rejected");
                    intent["error"] = json!(e.to_string());
                    write_json(&path, &intent)?;
                    return Err(e);
                }
            }
            eprintln!(
                "Collected {} via {provider}",
                prompt["id"].as_str().unwrap()
            );
            intent
        };
        docs.push(record["document"].clone());
        let parent = out.parent().unwrap_or(Path::new("."));
        fs::create_dir_all(parent)?;
        let mut temp = tempfile::NamedTempFile::new_in(parent)?;
        for doc in &docs {
            serde_json::to_writer(&mut temp, doc)?;
            temp.write_all(b"\n")?;
        }
        temp.as_file().sync_all()?;
        temp.persist(out)?;
    }
    Ok(
        json!({"documents":docs.len(),"new_invocations":missing,"path":out,"provider":provider,"model_requested":model}),
    )
}

pub fn make_rewrite_prompts(human_path: &Path, out: &Path) -> Result<Value> {
    let docs = read_jsonl(human_path)?;
    ensure!(
        !out.exists(),
        "Prompt file exists; preserve the frozen manifest"
    );
    fs::create_dir_all(out.parent().unwrap_or(Path::new(".")))?;
    let mut f = fs::File::create(out)?;
    for doc in &docs {
        let text = doc["text"].as_str().context("Missing source text")?;
        let n = text.split_whitespace().count();
        let prompt = format!(
            "Rewrite the scientific abstract below in your own natural prose. Preserve every factual claim, numerical value, qualification, method, result, and conclusion. Keep the same scientific register and tonal intention. Aim for {n} words, within roughly 10 percent, without omitting details to meet length. Return only the rewritten abstract, with no heading, commentary, or quotation marks around it. Do not use tools or external information.\n\nSOURCE ABSTRACT:\n{text}"
        );
        let p = json!({"id":format!("rewrite-{}",&digest(doc["group_id"].as_str().context("Missing source group")?)[..16]),"prompt":prompt,
            "domain":doc["domain"],"register":doc["register"],"group_id":doc["group_id"],"split":doc["split"],
            "generation_regime":"source-conditioned-rewrite","source_sha256":digest(text),"source_metadata":doc["metadata"],"length_target_whitespace_words":n});
        serde_json::to_writer(&mut f, &p)?;
        f.write_all(b"\n")?;
    }
    Ok(json!({"prompts":docs.len(),"path":out,"regime":"source-conditioned-rewrite"}))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn codex_identity_is_not_fabricated() {
        let events = vec![
            json!({"type":"item.completed","item":{"type":"agent_message","text":"Some prose."}}),
            json!({"type":"turn.completed","usage":{}}),
        ];
        let (_, model, meta) = parse_cli("codex-cli", "gpt-example", &events).unwrap();
        assert_eq!(model, "requested:gpt-example");
        assert!(
            meta["model_identity_source"]
                .as_str()
                .unwrap()
                .contains("not_reported")
        );
    }
    #[test]
    fn codex_tool_use_is_rejected() {
        let events = vec![
            json!({"type":"item.completed","item":{"type":"command_execution"}}),
            json!({"type":"turn.completed"}),
        ];
        assert!(parse_cli("codex-cli", "model", &events).is_err());
    }
    #[test]
    fn claude_uses_reported_model() {
        let events = vec![
            json!({"type":"system","subtype":"init","tools":[],"mcp_servers":[]}),
            json!({"type":"assistant","message":{"model":"actual-model","content":[{"type":"text","text":"Prose."}]}}),
            json!({"type":"result","subtype":"success","is_error":false,"result":"Prose."}),
        ];
        let (_, m, _) = parse_cli("claude-cli", "alias", &events).unwrap();
        assert_eq!(m, "actual-model");
    }
}
