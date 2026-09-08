# Detector evaluation

The target is a readable revision that preserves argumentation, detail, and tone,
with less than 10% of the full document classified as AI or AI-assisted. The
default gate uses `fraction_ai + fraction_ai_assisted < 0.10`, and also reports
AI-only scores separately. Exactly 10% fails. Never round before applying the gate.

Pangram reports character-weighted class fractions, not the probability that the
whole document was written by AI. Its segment assistance score is a different
quantity. Headline labels are also unsuitable for an exact threshold test.
Returned segments are merged output spans, not independent experimental samples.
See the [Pangram 4 model card](https://www.pangram.com/research/model-card/pangram-4).

## Stored observations

Every request keeps the original submitted text, its SHA-256 over exact UTF-8
bytes, explicit selector, returned version, timestamp, task ID, and full decoded
response. Do not trim text, normalize line endings, or render LaTeX differently
between nominally identical repeats. Preserve returned `text` separately:
Pangram can normalize the input, and its offsets refer to its returned text.
See the [current API reference](https://docs.pangram.com/api-reference/ai-detection).

The adapter submits asynchronously:

```python
import json
from pathlib import Path
from unslop.evaluation import PangramClient

client = PangramClient()  # PANGRAM_API_KEY is read from the environment.
# Read bytes first: Path.read_text() can normalize CRLF to LF.
text = Path("candidate.txt").read_bytes().decode("utf-8")
record = client.submit(text, model="pangram-4")
path = Path(f"{record['task_id']}.json")
path.write_text(json.dumps(record, ensure_ascii=False, indent=2) + "\n")
# The persisted task ID allows this step to resume without another submission.
record["result"] = client.poll(record["task_id"])
path.write_text(json.dumps(record, ensure_ascii=False, indent=2) + "\n")
```

Call `client.models()` to inspect the account's current selectors. Availability
can differ between accounts; `default` can move to a different model. Use an
explicit selector and keep the returned version, as described in the
[model catalog documentation](https://docs.pangram.com/api-reference/models).

POST requests are never automatically retried. If a POST response is lost, a
task might have been created and charged even though no ID was received. A GET
timeout leaves the saved ID available for resumption. `PangramTimeout` preserves
`task_id` and `last_response`; `PangramTaskFailed` preserves `task_id` and `result`.
The caller should save failure responses too. No public dashboard link is requested.

## Repeated full-document comparisons

`summarize_runs(records, threshold=.1, min_repeats=3)` groups by detector,
selector, returned version, and exact submitted input hash. It shows all scores,
means, medians, and maxima. An identical task response copied twice is one
observation. Conflicting responses for one task ID, invalid fractions, failed
tasks, and mismatched stored hashes cause errors rather than accidental passes.
Different documents or models stay in separate groups.

`paired_study(baseline_runs, candidate_runs, quality_audit)` accepts ordered
baseline/candidate repeat pairs. Each record must explicitly declare
`full_document: true`. Each arm must repeat one exact input, with distinct task
IDs, and each pair must use the same detector, selector, and returned version.
The caller supplies temporal pairing through list order; the function cannot
infer whether the experiment was randomized or interleaved correctly.

For a new edit, randomize or alternate baseline/candidate order in short blocks.
Keep every result, including failures and regressions. Use isolated sentences
to investigate mechanisms, then test the complete document. Predeclare the
candidate and final repeat count before confirmation scans; choosing the best
of many scans is selection bias. Start with three repeat pairs, and increase
the predeclared count if pilot observations show meaningful variation.

The quality audit must contain literal `true` for all five fields:

```json
{
  "reviewed_by_human": true,
  "readability": true,
  "argumentation": true,
  "detail": true,
  "tone": true
}
```

These are human attestations, not automatic fidelity metrics. Set them after
review; never generate a passing audit from detector scores. Record the reviewer,
date, and claim-by-claim notes alongside them. A missing, false, or nonboolean
flag cannot pass. The paired study passes only when the audit passes, both arms
have enough repeats, and every candidate observation passes the strict threshold.

## What a passing document establishes

Three successful repeats support a statement about that input in that test.
They cannot establish reliability on new documents, explain Pangram's internal
decision boundary, or prove human authorship. All summaries therefore set
`reliability_established: false`. Score reductions from one preface are useful
pilot observations, not independent samples of a general rewriting process.

Freeze a process before testing it on untouched source groups. Evaluate multiple
topics, registers, lengths, generator versions, and detectors. Keep revisions of
the same source, paraphrases of the same prompt, and near duplicates in one
split. Report the joint fidelity-and-detector success rate across source groups,
uncertainty from source-group resampling, worst-domain performance, query cost,
and readability review results. Repeat this evaluation after detector updates.

For a potential Bittensor validator, evaluate on private rotating source groups,
with strict content audits before any detector reward. Repeatedly querying a
public test set measures adaptation to those examples. It does not measure
generalization. A subnet reward and its anti-replay mechanisms should be designed
after the local process demonstrates a useful held-out success rate.
