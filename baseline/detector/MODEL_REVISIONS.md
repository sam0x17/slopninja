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

Use `export-shards` to validate and export a revision corpus. The older
`assemble_expansion` command requires direct human-root draft/edit pairs and
intentionally rejects these deeper chains. The frozen v5 experiment continues
with its original corpus and binaries. Revision chains need a separately
declared training/evaluation cohort; they are not added to v5 after selection.

The [first writing probe](MODEL_ONLY_REVISION_PROBE.md) showed why this category
matters: a detector may mistake wholly model-written revisions for mixed origin,
and a rewrite may preserve named facts while introducing false implications or
an unintended tone. Correct production labels establish origin. They do not
certify successful cleanup, meaning preservation or detector evasion.
