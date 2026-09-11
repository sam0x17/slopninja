//! Budgeted Pangram bulk submission and resumable, exact-input collection.
use anyhow::{Context, Result, ensure};
use clap::{Parser, Subcommand};
use reqwest::blocking::Client;
use serde_json::{Value, json};
use slop_ninja_detector::dataset::{Evidence, read_records, sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    time::Duration,
};

const BASE: &str = "https://text.external-api.pangram.com";

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}
#[derive(Subcommand)]
enum Command {
    /// Create a cumulative spending ledger. Does not contact Pangram.
    Init {
        #[arg(long)]
        budget_dir: PathBuf,
        #[arg(long)]
        cap_cents: u64,
    },
    /// A reservation is durable before POST; uncertain submissions never retry.
    Submit {
        #[arg(long)]
        plan_dir: PathBuf,
        #[arg(long, default_value_t = 0)]
        batch: usize,
        #[arg(long)]
        budget_dir: PathBuf,
        #[arg(long)]
        reserve_cents: u64,
    },
    /// Poll an existing job once, then archive and validate all terminal results.
    Collect {
        #[arg(long)]
        job_dir: PathBuf,
    },
}

fn write_new(path: &Path, bytes: &[u8]) -> Result<()> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    Ok(())
}
fn save(path: &Path, value: &Value) -> Result<()> {
    write_new(path, &serde_json::to_vec_pretty(value)?)
}
fn read(path: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}
fn field<'a>(value: &'a Value, key: &str) -> Result<&'a str> {
    value[key].as_str().context(format!("Missing {key}"))
}

struct Api {
    client: Client,
    key: String,
}
impl Api {
    fn new() -> Result<Self> {
        let key = std::env::var("PANGRAM_API_KEY").context("PANGRAM_API_KEY is required")?;
        ensure!(!key.is_empty(), "Empty API key");
        Ok(Self {
            key,
            client: Client::builder()
                .timeout(Duration::from_secs(45))
                .redirect(reqwest::redirect::Policy::none())
                .retry(reqwest::retry::never())
                .build()?,
        })
    }
    fn request(&self, route: &str, payload: Option<&[u8]>, archive: &Path) -> Result<Value> {
        let url = format!("{BASE}{route}");
        let request = match payload {
            Some(bytes) => self.client.post(&url).body(bytes.to_vec()),
            None => self.client.get(&url),
        };
        let response = request
            .header("x-api-key", &self.key)
            .header("Content-Type", "application/json")
            .send()
            .context("Pangram transport failure; do not resubmit an uncertain POST")?;
        let status = response.status();
        let bytes = response
            .bytes()
            .context("Pangram response read failed; do not resubmit an uncertain POST")?;
        write_new(&archive.with_extension("body"), &bytes)?;
        save(
            &archive.with_extension("http.json"),
            &json!({"url":url,"method":if payload.is_some(){"POST"}else{"GET"},
            "received_at":chrono::Utc::now().to_rfc3339(),"status":status.as_u16(),"body_sha256":sha256(&bytes)}),
        )?;
        ensure!(
            status.is_success(),
            "Pangram HTTP {}; response retained",
            status.as_u16()
        );
        Ok(serde_json::from_slice(&bytes)?)
    }
}

fn budget_used(directory: &Path) -> Result<u64> {
    let mut used = 0u64;
    for entry in fs::read_dir(directory.join("jobs"))? {
        let reservation = entry?.path().join("reservation.json");
        ensure!(
            reservation.exists(),
            "Incomplete budget job; inspect it before another submission"
        );
        used = used
            .checked_add(
                read(&reservation)?["reserved_usd_cents"]
                    .as_u64()
                    .context("Invalid reservation")?,
            )
            .context("Budget overflow")?;
    }
    Ok(used)
}

fn validate_payload(plan_dir: &Path, batch: usize) -> Result<(Value, Vec<u8>, u64)> {
    let plan = read(&plan_dir.join("plan.json"))?;
    ensure!(
        plan["schema"] == "slop_ninja_pangram_annotation_plan_v1"
            && plan["model_selector"] == "pangram-4",
        "Unsupported plan"
    );
    let source = plan_dir.join("source-records.jsonl");
    ensure!(
        sha256(fs::read(&source)?) == field(&plan, "input_sha256")?,
        "Corpus hash mismatch"
    );
    let records = read_records(&source)?;
    let records: BTreeMap<_, _> = records.iter().map(|r| (r.id.as_str(), r)).collect();
    let path = format!("requests/batch-{batch:04}.json");
    let payload = fs::read(plan_dir.join(&path))?;
    ensure!(
        plan["request_files"][batch]["path"] == path
            && plan["request_files"][batch]["sha256"] == sha256(&payload),
        "Request binding mismatch"
    );
    let request: Value = serde_json::from_slice(&payload)?;
    ensure!(request["model"] == "pangram-4", "Wrong selector");
    let items = request["items"].as_array().context("Missing items")?;
    let members: Vec<_> = plan["members"]
        .as_array()
        .context("Missing members")?
        .iter()
        .filter(|m| m["batch_index"].as_u64() == Some(batch as u64))
        .collect();
    ensure!(
        members.len() == items.len() && !items.is_empty(),
        "Coverage mismatch"
    );
    let mut ids = BTreeSet::new();
    let mut units = 0u64;
    for (index, (item, member)) in items.iter().zip(&members).enumerate() {
        let id = field(item, "id")?;
        let text = field(item, "text")?;
        ensure!(ids.insert(id), "Duplicate request ID");
        ensure!(
            member["item_index"] == index
                && member["request_id"] == id
                && member["text_sha256"] == sha256(text),
            "Item mismatch"
        );
        let provenance = member["records"].as_array().context("Missing provenance")?;
        ensure!(!provenance.is_empty(), "Empty provenance");
        for reference in provenance {
            let record = records
                .get(field(reference, "record_id")?)
                .context("Unknown origin record")?;
            ensure!(
                record.text == text
                    && record.rights.external_evaluation
                    && record.evidence != Evidence::SyntheticFixture,
                "Text or external rights mismatch"
            );
            ensure!(
                reference["origin"] == serde_json::to_value(record.origin)?
                    && reference["split"] == serde_json::to_value(record.split)?
                    && reference["evidence"] == serde_json::to_value(record.evidence)?
                    && reference["source_group"] == record.source_group,
                "Origin metadata mismatch"
            );
        }
        let words = text.split_whitespace().count();
        ensure!(words >= 50, "Input too short");
        units += words.div_ceil(100) as u64;
    }
    ensure!(units <= 1_000, "Batch exceeds provider unit limit");
    Ok((plan, payload, units))
}

fn submit(plan_dir: &Path, batch: usize, budget_dir: &Path, reserve_cents: u64) -> Result<Value> {
    let (plan, payload, units) = validate_payload(plan_dir, batch)?;
    // Reserve at least realtime pricing, leaving headroom over the bulk estimate.
    ensure!(
        reserve_cents >= units * 5,
        "Reservation must cover realtime estimate as headroom"
    );
    let api = Api::new()?;
    let lock = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(budget_dir.join("lock"))?;
    lock.try_lock()
        .context("Another budget operation is active")?;
    let authorization = read(&budget_dir.join("authorization.json"))?;
    let cap = authorization["cap_usd_cents"]
        .as_u64()
        .context("Missing cap")?;
    let used = budget_used(budget_dir)?;
    ensure!(
        used.checked_add(reserve_cents).is_some_and(|v| v <= cap),
        "Cumulative budget exhausted"
    );
    let job_id = format!("{}-{batch}", sha256(fs::read(plan_dir.join("plan.json"))?));
    let job = budget_dir.join("jobs").join(&job_id);
    ensure!(
        !job.exists(),
        "Existing or uncertain submission; collect its receipt, never resubmit"
    );
    fs::create_dir(&job)?;
    save(
        &job.join("reservation.json"),
        &json!({"schema":"slop_ninja_pangram_budget_reservation_v1",
        "reserved_usd_cents":reserve_cents,"estimated_bulk_usd_cents":units*4,"estimated_units":units,
        "job_id":job_id,"reserved_at":chrono::Utc::now().to_rfc3339(),"authorization_sha256":sha256(fs::read(budget_dir.join("authorization.json"))?)}),
    )?;
    write_new(&job.join("request.json"), &payload)?;
    save(&job.join("plan.json"), &plan)?;
    save(
        &job.join("binding.json"),
        &json!({"batch_index":batch,"payload_sha256":sha256(&payload),
        "plan_sha256":sha256(fs::read(job.join("plan.json"))?),"runner_sha256":sha256(fs::read(std::env::current_exe()?)?)}),
    )?;
    File::open(&job)?.sync_all()?;
    File::open(budget_dir.join("jobs"))?.sync_all()?;
    let models = api.request("/models", None, &job.join("models"))?;
    ensure!(
        models["models"]
            .as_array()
            .is_some_and(|m| m.iter().any(|x| x == "pangram-4")),
        "No Pangram 4 entitlement"
    );
    save(
        &job.join("submission-intent.json"),
        &json!({"at":chrono::Utc::now().to_rfc3339(),"payload_sha256":sha256(&payload)}),
    )?;
    File::open(&job)?.sync_all()?;
    let receipt = api.request("/bulk", Some(&payload), &job.join("submission"))?;
    let bulk_id = field(&receipt, "bulk_id")?;
    ensure!(
        !bulk_id.is_empty(),
        "Receipt omitted bulk ID; do not resubmit"
    );
    save(&job.join("receipt.json"), &receipt)?;
    Ok(json!({"status":"submitted","job_dir":job,"bulk_id":bulk_id,
        "estimated_bulk_usd_cents":units*4,"cumulative_reserved_usd_cents":used+reserve_cents,"cap_usd_cents":cap}))
}

fn validate_result(item: &Value, expected: &Value, task_id: &str) -> Result<()> {
    ensure!(
        item["id"] == expected["id"] && item["task_id"] == task_id,
        "Result identity mismatch"
    );
    ensure!(
        item["stage"] == "STAGE_SUCCESS",
        "Unexpected non-success result"
    );
    let result = &item["result"];
    ensure!(
        result["text"] == expected["text"],
        "Provider returned different input text"
    );
    ensure!(
        !field(result, "version")?.is_empty(),
        "Missing provider version"
    );
    let mut sum = 0.0;
    for name in ["fraction_human", "fraction_ai", "fraction_ai_assisted"] {
        let value = result[name].as_f64().context("Missing fraction")?;
        ensure!(
            value.is_finite() && (0.0..=1.0).contains(&value),
            "Invalid content fraction"
        );
        sum += value;
    }
    ensure!(
        (sum - 1.0).abs() < 1e-5,
        "Content fractions do not sum to one"
    );
    Ok(())
}

fn collect(job: &Path) -> Result<Value> {
    let receipt = read(&job.join("receipt.json"))?;
    let bulk_id = field(&receipt, "bulk_id")?;
    ensure!(
        !bulk_id.is_empty()
            && bulk_id
                .bytes()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, b'-' | b'_')),
        "Invalid bulk ID"
    );
    let binding = read(&job.join("binding.json"))?;
    ensure!(
        binding["payload_sha256"] == sha256(fs::read(job.join("request.json"))?)
            && binding["plan_sha256"] == sha256(fs::read(job.join("plan.json"))?),
        "Job input changed"
    );
    let request = read(&job.join("request.json"))?;
    let items = request["items"].as_array().context("Missing items")?;
    let api = Api::new()?;
    let attempt = job.join(format!(
        "collection-{}",
        chrono::Utc::now()
            .timestamp_nanos_opt()
            .context("Invalid timestamp")?
    ));
    fs::create_dir(&attempt)?;
    let status = api.request(&format!("/bulk/{bulk_id}"), None, &attempt.join("status"))?;
    ensure!(
        status["bulk_id"] == bulk_id && status["total_items"] == items.len(),
        "Bulk status identity mismatch"
    );
    if !["succeeded", "failed", "partial"].contains(&field(&status, "status")?) {
        return Ok(
            json!({"status":status["status"],"bulk_id":bulk_id,"provider":status,"resume_job_dir":job}),
        );
    }
    let mut accepted = BTreeMap::new();
    let mut seen_tasks = BTreeSet::new();
    let mut initial_failed = BTreeMap::new();
    for slot in receipt["accepted_items"]
        .as_array()
        .context("Missing acceptance roster")?
    {
        let index = slot["index"].as_u64().context("Missing accepted index")? as usize;
        ensure!(
            index < items.len() && slot["id"] == items[index]["id"],
            "Acceptance mismatch"
        );
        let task = field(slot, "task_id")?;
        ensure!(
            seen_tasks.insert(task) && accepted.insert(index, task).is_none(),
            "Duplicate accepted task"
        );
    }
    for slot in receipt["failed_items"]
        .as_array()
        .context("Missing failed roster")?
    {
        let index = slot["index"].as_u64().context("Missing failed index")? as usize;
        ensure!(
            index < items.len()
                && slot["id"] == items[index]["id"]
                && !accepted.contains_key(&index)
                && initial_failed.insert(index, slot).is_none(),
            "Failure roster mismatch"
        );
    }
    ensure!(
        accepted.len() + initial_failed.len() == items.len(),
        "Receipt omitted inputs"
    );
    let mut observations = BTreeMap::<usize, Value>::new();
    for offset in (0..items.len()).step_by(100) {
        let page = api.request(
            &format!("/bulk/{bulk_id}/results?offset={offset}&limit=100"),
            None,
            &attempt.join(format!("page-{offset}")),
        )?;
        ensure!(
            page["bulk_id"] == bulk_id
                && page["total_items"] == items.len()
                && page["offset"] == offset,
            "Page identity mismatch"
        );
        for key in ["items", "failed_items"] {
            let empty = Vec::new();
            let rows = match page.get(key) {
                Some(v) => v.as_array().context("Invalid results array")?,
                None => &empty,
            };
            for item in rows {
                let index = item["index"].as_u64().context("Missing result index")? as usize;
                ensure!(
                    (offset..items.len().min(offset + 100)).contains(&index)
                        && item["id"] == items[index]["id"],
                    "Page item mismatch"
                );
                ensure!(!observations.contains_key(&index), "Duplicate result index");
                if key == "items" {
                    validate_result(
                        item,
                        &items[index],
                        accepted.get(&index).context("Unaccepted success")?,
                    )?;
                } else {
                    ensure!(item["stage"] == "STAGE_FAILED", "Unresolved failed item");
                    if let Some(task) = accepted.get(&index) {
                        ensure!(item["task_id"] == *task, "Failed task identity mismatch");
                    } else {
                        ensure!(
                            initial_failed.contains_key(&index) && item["task_id"].is_null(),
                            "Unknown failed input"
                        );
                    }
                }
                observations.insert(index, item.clone());
            }
        }
    }
    ensure!(
        observations.len() == items.len(),
        "Terminal results do not cover every input"
    );
    let plan = read(&job.join("plan.json"))?;
    let batch = binding["batch_index"]
        .as_u64()
        .context("Missing batch binding")?;
    let members: BTreeMap<_, _> = plan["members"]
        .as_array()
        .context("Missing members")?
        .iter()
        .filter(|m| m["batch_index"].as_u64() == Some(batch))
        .map(|m| (m["item_index"].as_u64().unwrap() as usize, m))
        .collect();
    let mut annotations = Vec::new();
    let mut versions = BTreeSet::new();
    let mut succeeded = 0;
    for (index, item) in &observations {
        if item["stage"] == "STAGE_SUCCESS" {
            succeeded += 1;
            versions.insert(field(&item["result"], "version")?);
        }
        let annotation = json!({"schema":"slop_ninja_pangram_corpus_annotation_v1","bulk_id":bulk_id,
            "member":members.get(index).context("Missing source member")?,"model_selector":"pangram-4",
            "collected_at":chrono::Utc::now().to_rfc3339(),"observation":item});
        serde_json::to_writer(&mut annotations, &annotation)?;
        annotations.push(b'\n');
    }
    ensure!(
        status["succeeded"] == succeeded && status["failed"] == items.len() - succeeded,
        "Final counters disagree with coverage"
    );
    write_new(&attempt.join("annotations.jsonl"), &annotations)?;
    let summary = json!({"schema":"slop_ninja_pangram_corpus_collection_v1","status":"complete",
        "bulk_id":bulk_id,"items":items.len(),"succeeded":succeeded,"failed":items.len()-succeeded,
        "returned_versions":versions,"annotations_sha256":sha256(&annotations),"collection_dir":attempt,
        "billing":"unverified; retain the full reservation until account usage is reconciled"});
    save(&attempt.join("summary.json"), &summary)?;
    Ok(summary)
}

fn main() -> Result<()> {
    let result = match Args::parse().command {
        Command::Init {
            budget_dir,
            cap_cents,
        } => {
            ensure!(
                cap_cents > 0 && !budget_dir.exists(),
                "Use a new budget directory and a positive cap"
            );
            fs::create_dir_all(budget_dir.join("jobs"))?;
            write_new(&budget_dir.join("lock"), b"")?;
            let value = json!({"schema":"slop_ninja_pangram_budget_v1","cap_usd_cents":cap_cents,
                "authorization":"Caller-supplied cumulative experimental spending cap; not per run",
                "reserved_not_released_without_billing_evidence":true});
            save(&budget_dir.join("authorization.json"), &value)?;
            value
        }
        Command::Submit {
            plan_dir,
            batch,
            budget_dir,
            reserve_cents,
        } => submit(&plan_dir, batch, &budget_dir, reserve_cents)?,
        Command::Collect { job_dir } => collect(&job_dir)?,
    };
    println!("{}", serde_json::to_string_pretty(&result)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn results_require_exact_text_task_and_valid_fractions() {
        let expected = json!({"id":"row","text":"An invented fixture."});
        let good = json!({"id":"row","task_id":"task","stage":"STAGE_SUCCESS","result":{
            "text":"An invented fixture.","version":"test","fraction_human":0.2,"fraction_ai":0.5,"fraction_ai_assisted":0.3}});
        validate_result(&good, &expected, "task").unwrap();
        let mut changed = good.clone();
        changed["result"]["text"] = json!("Different text");
        assert!(validate_result(&changed, &expected, "task").is_err());
        assert!(validate_result(&good, &expected, "other-task").is_err());
        let mut changed = good;
        changed["result"]["fraction_ai"] = json!(0.7);
        assert!(validate_result(&changed, &expected, "task").is_err());
    }
    #[test]
    fn uncertain_jobs_still_consume_budget() {
        let temp = tempfile::tempdir().unwrap();
        fs::create_dir(temp.path().join("jobs")).unwrap();
        let job = temp.path().join("jobs/uncertain");
        fs::create_dir(&job).unwrap();
        assert!(budget_used(temp.path()).is_err());
        save(
            &job.join("reservation.json"),
            &json!({"reserved_usd_cents":1000}),
        )
        .unwrap();
        assert_eq!(budget_used(temp.path()).unwrap(), 1000);
    }
}
