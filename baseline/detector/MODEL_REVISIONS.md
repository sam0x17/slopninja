# Model-only revision chains

`generate_samples --model-revisions` revises the final model-written passage in
each recorded branch. Supply an admitted corpus containing those passages and
their complete ancestry, with source-family splits already frozen. Both
same-model and cross-model revisions receive `model_only` labels and the new
`recorded_model_revision` evidence value. A model copyedit of a historical human
source keeps its existing `mixed` label and cannot enter this revision mode.

From `baseline/detector/`, with verified weights behind the configured endpoint:

```sh
cargo run --release --bin generate_samples -- \
  --input ../../data/baseline-detector/selected-model-drafts/records.jsonl \
  --model-spec ../../data/baseline-detector/generators/reviser.json \
  --output-dir ../../data/baseline-detector/model-revisions-round1 \
  --base-url http://127.0.0.1:18123/v1 \
  --model-revisions --revision-style anti-ai \
  --concurrency 1 --max-calls 8 --complete-families-only
```

Each call revises one leaf. To continue a chain, use the completed export as the
next invocation's input, a new output directory, and the desired reviser spec.
`--revision-style fix-slop` adds the existing adapted Fix Slop instructions to
the heavy voice-avoidance prompt. The prompt names the previous and current
model revisions, asks for substantial structural changes, and preserves meaning
and intended tone as constraints. It does not apply the light-copyedit wording
requirement from the original style mix. The legacy `--prompt-profile-set` and
revision mode are mutually exclusive.

Exports preserve every input ancestor and accepted new passage. Each new record
links to its immediate parent; that link must resolve to recorded model-only
prose in the same source family and partition. The cache identity binds the full
parent record, reviser spec, style and request. The run also binds the complete
input file. Raw requests and responses stay in `calls/`; revision text retains
the full returned content, including surrounding whitespace. No one repairs
wording or removes an unwanted introduction before admission.

The existing length, completion and formatting checks still apply. Reusing
phrases from a model-written parent does not create human authorship, so the
human-source copying check applies only to initial drafts. Once every planned
task has been attempted, complete-family export excludes the entire source
family if any selected revision failed. Accepted siblings and failed responses
remain archived. The summary distinguishes new accepted calls from inherited
records and lists failed parents and excluded source groups. There are no
automatic retries or detector-guided redraws.

Use `archive_generation` after a cohort closes to collect all input records,
every accepted sibling and a hash-bound receipt for each failed attempt. Supply
each cohort's exact original input, including families absent from its complete
export:

```sh
cargo run --release --bin archive_generation -- \
  --input ../../data/baseline-detector/selected-model-drafts/records.jsonl \
  --generation-dir ../../data/baseline-detector/model-revisions-round1 \
  --output-dir ../../data/baseline-detector/model-revisions-master1
```

Repeat `--input` and `--generation-dir` for later stages. The command checks
closed summaries, request/response hashes, complete exports and full ancestry.
Its master archive retains unsuccessful branches and is not the fitting or
paired-evaluation cohort. Freeze that complete-family selection separately.
On the first v6 Train pass, this retained all 471 source families and 1,882
records, including 469 accepted revisions and the original records from two
failed branches. The separate complete export has 469 families and 1,876 rows.
Those counts describe admission; no v6 detector score has been measured.

Use `export-shards` to validate and export a revision corpus. The older
`assemble_expansion` command requires direct human-root draft/edit pairs and
intentionally rejects these deeper chains. The frozen v5 experiment continues
with its original corpus and binaries. Revision chains need a separately
declared training/evaluation cohort; they are not added to v5 after selection.

`assemble_revision_expansion` implements the [frozen v6 selection](ENCODER_REVISION_V6.md).
It retains all original v5 Train rows, adds both new revisions only for completed
Train chains, and exports the fresh primary ancestry with R0/R1/R2 observation
views. Phi-4 remains a separate Test route. Run it after all required cohorts
close; it does not generate text, tokenize inputs, fit an encoder or score Test.

Its `--revision-runs` JSON manifest uses schema
`slop_ninja_v6_revision_runs_v1`, with `train`, `primary` and optional `phi4`
objects. Each route contains `first` and `second` objects with `run`, `input`
and `records` paths, resolved relative to the manifest. Those paths bind the
actual generator run metadata, exact pass input and closed complete export.
The assembler reconstructs each revision request and cache key through the
same pure helper used by generation. It checks the declared model, style,
source allocation and initial profile, while the master archive retains raw
response hashes and attempt counts. Complete-only exports cannot establish
failure rates; the assembly report leaves those counts explicitly absent.

The [first writing probe](MODEL_ONLY_REVISION_PROBE.md) showed why this category
matters: a detector may mistake wholly model-written revisions for mixed origin,
and a rewrite may preserve named facts while introducing false implications or
an unintended tone. Correct production labels establish origin. They do not
certify successful cleanup, meaning preservation or detector evasion.

## Completed training-only integration sample

The [integration report](results/model-revision-integration-v1.json) records a
two-pass run over eight existing Train drafts. Four were originally written by
Qwen and four by Mistral. Qwen revised all eight with the Anti-AI instruction,
then revised each result with the Fix Slop instruction. All 16 calls completed
and passed admission; no output was repaired, retried or selected by a detector.

The resulting corpus retains eight historical human references, eight initial
model drafts and 16 model-only revisions across eight source families. There
are 12 same-model revision steps and four cross-model steps. All 32 rows pass
lineage validation and fit the 1,024-token encoder contract; the longest has
488 tokens. The source attribution and ShareAlike notices remain attached.

The first pass retained a mean 38.9% of its parent's words in longest-common-
subsequence order; every first-pass revision retained less than half. The second
pass retained a mean 53.0%. Neither pass returned an unchanged word sequence.
These surface measurements show substantial rewriting, without certifying
meaning, tone, authorship resemblance or detector evasion. No detector was run.
V4 has already trained on these source families, so this sample cannot measure
unseen-source performance. It remains available for a future declared corpus.

The amended generator also replayed the completed Qwen narrative cache with
zero new calls. Its 819-row export and terminal summary matched the original
bytes exactly. The revision implementation passed the required Rust tests,
formatting and clippy checks. The current v5 corpus, frozen binaries and fit
inputs remained unchanged.
