# Reference detector

The first trained candidate is a ModernBERT origin classifier. Read the
[pilot results](PILOT_RESULTS.md) and download the
[experimental release](https://github.com/sam0x17/slopninja/releases/tag/detector-pilot-v0.1.0).
It passed inference checks after transfer from the Studio, but has substantial
generator-transfer and formatting weaknesses. The release includes the selected
encoder, all three linear controls and all 16 aggregate evaluations.
The [style-diverse pilot](STYLE_PILOT_RESULTS.md) added fresh sources, two teachers,
varied instructions and matched whitespace exposure. Its candidate improved
pooled log loss but never chose the mixed class on any test view. It is available
as a separate experimental release, with all comparisons and exclusions reported.
The [word/grammar development probe](LINEAR_STYLE_RESULTS.md) recovered mixed
predictions with smaller optimization steps and independent coordinate budgets.
The [standard-corpus plan](STANDARD_CORPUS.md) describes admitted HC3 sources and
the reusable Pangram annotation workflow. The first [Pangram comparison](PANGRAM_CORPUS_RESULTS.md)
flagged all 38 model/mixed Development texts, compared with 20 for the encoder.
Three passes had identical document fractions; auxiliary window scores varied.
The [next corpus expansion](CORPUS_EXPANSION_V4.md) adds historical encyclopedia
sources, prepares narrative summaries and introduces OLMo as a third generator.
The [v4 encoder experiment](ENCODER_EXPANSION_V4.md) specifies balanced source
weighting and two learning rates, with Development selection fixed before fitting.
Its [completed comparison](ENCODER_EXPANSION_RESULTS.md) improved fresh Test
accuracy from 52.9% to 78.4% and recovered mixed-class predictions. It remains
experimental and still trails Pangram on the previously scored Development texts.
Download the [v4 weights and inference bundle](https://github.com/sam0x17/slopninja/releases/tag/detector-corpus-v0.4.0).

The [v5 narrative experiment](ENCODER_NARRATIVE_V5.md) adds 289 admitted historical
plot summaries and reserves Phi-4 for an unseen-generator Test. Its
[completed comparison](ENCODER_NARRATIVE_RESULTS.md) reduced primary-Test binary
log loss from 0.6970 to 0.1430 and human false positives from 5/25 to 1/25, while
detecting all 25 model-written drafts. Phi-4 log loss also improved, but detection
fell from 21/21 to 19/21 at the stricter threshold. The missed drafts used Anti-AI
and Fix Slop instructions. V5 is the next experimental binary detector; v4 remains
available for the reported sensitivity comparison.
Download the [v5 weights and CPU inference bundle](https://github.com/sam0x17/slopninja/releases/tag/detector-narrative-v0.5.0).
An early [Train-only overlap audit](NARRATIVE_GENERATION_QA.md) found substantial
rewriting in Qwen's requested copyedits. The [clarified main target](ENCODER_NARRATIVE_V5_TARGET.md)
is fully model-written prose deliberately steered away from the model's usual
voice. Selection now prioritizes human-versus-model-draft detection, with explicit
writing-profile slices. Selection was frozen before either Test opened.
The [narrative corpus report](NARRATIVE_CORPUS_RESULTS.md) records completed
provider cohorts and all exclusions.
Its first Development comparison found that v4 flags nine of 26 historical human
narrative sources, despite detecting all 26 model-written drafts. V5 reduces
this register's observed false positives, but these small historical samples
cannot establish a population false-positive bound or Pangram parity.

The [model-only revision probe](MODEL_ONLY_REVISION_PROBE.md) records strong
DeepSeek style steering, a Fix Slop pass and a DeepSeek-to-Qwen rewrite. V4
detected all four synthetic passages, but the rewrites introduced meaning or
tone problems. Both same-model and cross-model revisions retain model-only
lineage; these examples remain outside the frozen v5 experiment.
The [revision-chain generator](MODEL_REVISIONS.md) now records this workflow
directly, retaining every ancestor and a model-only label through successive
rewrites by the same or different models.

The first reference candidate estimates three document-origin classes:
`human_only`, `model_only`, and `mixed`. Rust handles corpus admission,
provenance, splitting, word/grammar features, linear models and evaluation.
The optional [ModernBERT path](ml/README.md) uses PyTorch for encoder training
and inference. Neither path infers a percentage of AI-written words.

Read the [implementation plan](PLAN.md), [source register](SOURCES.md) and
[frozen pilot protocol](PILOT_PROTOCOL.md) for the scope and evidence limits.
The initial corpus uses licensed historical human proxies, recorded
source-conditioned model drafts and model copyedits of those proxies. It does
not contain verified contemporary human/model collaborations.

## Build and collect

Run these commands from this directory:

```sh
cargo build --release
cargo run --release --bin slop_ninja_detector -- acquire \
  --output-dir ../../data/baseline-detector/acquisition-v1 --plos 80 --wikinews 80
cargo run --release --bin slop_ninja_detector -- split \
  --input ../../data/baseline-detector/acquisition-v1/human-records.jsonl \
  --output ../../data/baseline-detector/acquisition-v1/frozen-roots.jsonl \
  --seed slop-ninja-origin-pilot-v1
```

Acquisition retains raw source and license captures, extraction decisions and
rejections. Records bind exact text hashes, source versions, attribution and
origin evidence. The splitter refuses to reshuffle assigned partitions. An
original and all of its generated descendants belong to one source family.
Identical and normalized duplicates are joined before splitting.

## Generate recorded examples

For new corpora, use the [versioned prompt profiles](PROMPT_PROFILES.md) with
`--prompt-profile-set style-mix-v1`, fresh frozen input and a new output directory.
The mix includes varied registers, explicit anti-AI phrasing instructions and an
adapted Fix Slop snapshot. The command below retains the original pilot prompt
pair for reproduction; omitting the profile option preserves its cache IDs.

`generate_samples` calls an explicitly configured local model endpoint. Its
model-spec JSON binds the model ID, immutable revision/file hash, Apache-2.0
license evidence, quantization, runtime, temperature and token limit. See the
struct in `src/bin/generate_samples.rs` for the exact fields. Verify the loaded
weights against that spec before using an export.
The [native generation notes](ml/GENERATION_RUNTIME.md) describe the separate
MLX environment and the pinned Mistral tokenizer correction.

```sh
cargo run --release --bin generate_samples -- \
  --input ../../data/baseline-detector/acquisition-v1/frozen-roots.jsonl \
  --model-spec ../../data/baseline-detector/generators/qwen-pilot.json \
  --output-dir ../../data/baseline-detector/qwen-pilot \
  --base-url http://127.0.0.1:18123/v1 --concurrency 4 --max-calls 320 \
  --complete-families-only
```

Requests and responses stay in ignored `data/`. Calls are capped, failed attempts
are retained, and completed responses are reused only after validation against
the request and model spec. The optional `LM_STUDIO_API_KEY` is read from the
environment and never written into the request archive. Use an SSH tunnel for a
remote Studio endpoint. Full-family export requires all tasks to have been
attempted and reports every family excluded because an output failed admission.
Before any new request, the generator requires a reachable endpoint with a
nonempty model catalog. A failed readiness check leaves the run resumable and
does not count every source as a failed model output.

To pause generation, create `PAUSE` in that run's output directory:

```sh
touch ../../data/baseline-detector/qwen-pilot/PAUSE
```

Workers stop taking new tasks when they observe the file. Requests already in
flight finish and checkpoint before the process exits. An incomplete paused run
has `status: "paused"` and `complete_export_ready: false` in `summary.json`; it
writes no cohort export, including when partial-export flags were supplied.
A clean exit means the drain succeeded, not that the cohort is complete. Keep
the file in place until the process exits, then remove it and rerun the same
generation command to resume from validated cached responses:

```sh
rm ../../data/baseline-detector/qwen-pilot/PAUSE
```

The pause file also blocks new invocations until removed. Pausing does not change
prompts, task identities or admission rules.

## Train and evaluate the Rust controls

Install the repository's pinned spaCy environment first. From this directory,
the existing environment is `../../.venv/bin/python`.

```sh
cargo run --release --bin slop_ninja_detector -- featurize \
  --input ../../data/baseline-detector/qwen-pilot/records.jsonl \
  --output ../../data/baseline-detector/qwen-pilot/features.jsonl \
  --mode combined --python ../../.venv/bin/python
cargo run --release --bin slop_ninja_detector -- train \
  --features ../../data/baseline-detector/qwen-pilot/features.jsonl \
  --mode combined --max-coordinates 8192 \
  --output-dir ../../data/baseline-detector/runs/linear-combined-v1
```

Use `--mode word` and `--mode grammar` with the same feature file for the two
controls. Vocabulary/scaling fit on training data; development log loss selects
the checkpoint; calibration data fit the temperature and operating thresholds.
Test feature values never enter the fitting function. Open a frozen test only
after candidate selection:

```sh
cargo run --release --bin slop_ninja_detector -- evaluate \
  --features ../../data/baseline-detector/qwen-pilot/features.jsonl \
  --artifact ../../data/baseline-detector/runs/linear-combined-v1/model.json \
  --output ../../data/baseline-detector/runs/linear-combined-v1-test.json \
  --open-final-test
```

Check `--help` for inference, shard export and audit commands. The runner rejects
unsupported lengths and schema/parser mismatches instead of truncating text or
changing preprocessing. Artifacts bind class order, preprocessing, source code,
dependencies and numerical rules. Later source edits may require rebuilding an
artifact under a new identity.

Corpus text, invocation logs, fitted checkpoints and run caches stay in `data/`.
Publish a reference candidate only with its model manifest, provenance, exact
runtime and measured limitations. Low observed false-positive counts in this
small pilot cannot establish a reliable 1% or 5% population bound.
