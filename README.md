# unslop

Research toward an adversarial Bittensor subnet for text revision: make writing easier to read while preserving its arguments, details, qualifications, and intended tone, with less than 10% AI-generated **plus AI-assisted** content on repeated detector checks.

The runtime is **Rust**. Python is confined to spaCy parsing and its grammatical annotations. The project contains corpus collection, SQLite profiles, paired feature analysis, detector experiments, and an offline miner/validator contract. It does not yet contain a trained rewriting model or demonstrate reliable performance below 10%.

The current direction is **author-conditioned editing**: fit a measured style
profile from the writer's own samples, ask an LLM for small changes toward that
profile, and remeasure the proposals before an independent preservation review.
The Rust prototype includes author/register profiles, grammatical and lexical
coordinates, target controls, prompt generation and candidate ranking. Read
[the implementation and runnable workflow](docs/style-space.md) and
[the research and grammar specification](docs/style-research.md).

## Current data and findings

- 12 licensed scientific abstracts published in 2018, matched with 12 Codex rewrites and 12 Claude Opus rewrites. Source groups were assigned to 8 training, 2 development, and 2 test groups before generation.
- 100 FineWeb documents captured in 2021: 93,458 whitespace words from 94 source hosts, with pinned revision and response hashes. This is a deterministic first-row pilot, not a representative random sample or verified human-authorship ground truth.
- Eight additional Claude Fable outputs and a rejected model-fallback invocation are retained separately. They do not enter the complete matched comparison.
- The earlier preface experiment remains intact: 120 distinct inputs and 130 Pangram requests from one source family.

The first detector pass used only the eight training sources per complete cohort:

| Corpus | Inputs | Mean AI + assisted fraction | Inputs below 10% |
| --- | ---: | ---: | ---: |
| Human originals | 8 | 0% | 8 |
| Codex rewrites | 8 | 36.39% | 3 |
| Claude Opus rewrites | 8 | 72.44% | 1 |

These are single observations per input. They do not establish repeated success, fidelity, readability, or generalization. All Codex inputs scored 0% AI-generated, but five were detected as AI-assisted. The combined gate matters.

The profiles also contradict a simple passive-voice rule: Codex lowered the passive-sentence rate in every training pair. Read [the matched pilot report](docs/matched-pilot.md), [public dataset catalog](docs/public-datasets.md), and [model-card implications](docs/model-card-notes.md).

A subsequent controlled screen produced one complete abstract revision at **0% AI plus assisted on three fresh confirmation scans**, against 100% for its baseline. One minimal relative-clause edit repeatedly reduced another input from 72.31% to 36.61%. Other edits failed or worsened scores. See [all controlled edit results](docs/controlled-edits.md), including preservation findings and the limits of these reused training examples.

The first fixed-prompt development evaluation produced **no new passes**: two already-zero inputs stayed at zero, while two flagged inputs worsened, consistently over three repeats each. Independent assistant review found no material information loss. [Development results](docs/fixed-process-results.md) explain why the successful training edits do not yet provide a reliable rewriting process.

Retesting the editing method on the preface also produced no improvement. The existing copy scored **27.90%** in a fresh scan; four preservation-reviewed revisions scored **44.33% to 69.11%**. The selected preface remains unchanged. [Preface results](docs/preface-edit-v2.md) retain all five observations, the claim review, and grammatical comparisons.

Provider watermarks need separate provenance. Anthropic documents text watermarking for specific current models, but Pangram scores do not establish watermark presence or removal. See [the verified coverage and limits](docs/watermark-provenance.md).

## Build and run

```sh
cargo build --release
target/release/unslop --help
```

Only grammatical extraction needs Python:

```sh
python3.12 -m venv .venv
.venv/bin/python -m pip install -r requirements-grammar.lock
target/release/unslop features preface.txt --grammar
```

The recorded environment used Rust 1.98.1, Python 3.12, spaCy 3.8.16, and `en_core_web_sm` 3.8.0. Keep `Cargo.lock` and the grammar lockfile for reproduction. Rust and historical Python extractors have separate identities; incompatible versions are never silently pooled. `--python` selects another parser interpreter.

```sh
cargo fmt --check
cargo test
cargo clippy --all-targets -- -D warnings
```

Tests make no paid calls. The grammar integration test uses the installed local parser when `.venv` exists. The former Python CLI is archived in [legacy/python-v0.1](legacy/python-v0.1/README.md); new work uses Rust.

## Reproduce corpus collection

```sh
target/release/unslop human-pilot
target/release/unslop fineweb-sample --rows 100
target/release/unslop ingest data/matched-pilot-v1/human.jsonl --grammar
target/release/unslop ingest data/public-datasets/fineweb-2021-43-pilot/human.jsonl --grammar
```

Source manifests retain dates, authors, URLs, licenses, extraction rules, and hashes. FineWeb's database license does not replace rights in its source pages. Public corpora help with scale; topic/register matching and provenance still matter.

The pilot prompts request **source-conditioned rewrites**, not writing from scratch. To collect outputs using existing CLI sign-ins:

```sh
target/release/unslop collect experiments/matched-pilot-v1/prompts.jsonl \
  --provider codex-cli --model gpt-6-astra --corpus codex-rewrites-v1 \
  --out data/matched-pilot-v1/codex.jsonl --max-requests 12
target/release/unslop collect experiments/matched-pilot-v1/prompts.jsonl \
  --provider claude-cli --model claude-opus-5 --corpus claude-opus-rewrites-v1 \
  --out data/matched-pilot-v1/claude-opus.jsonl --max-requests 12
```

These commands consume model usage. Completed invocations resume from captured records; uncertain or rejected invocations require inspection before another submission. The harness invokes each CLI once; internal retries remain under CLI control and are recorded when exposed. Model switches and tool use are rejected. Codex's JSON stream does not report a resolved model, so its identity is explicitly `requested:gpt-6-astra`. Claude records the model reported in assistant messages. CLI system instructions remain part of the experimental conditions.

Direct API providers `openai` and `anthropic` also work with environment credentials and explicit model IDs. See [corpus design](docs/corpus.md).

For a fixed editing process, `edit-prompts` selects an explicit split from model corpora and combines each immediate model text with a frozen instruction file. It excludes earlier human-reference prompts and detector data from generation context:

```sh
target/release/unslop edit-prompts data/matched-pilot-v1/codex.jsonl \
  data/matched-pilot-v1/claude-opus.jsonl \
  --instructions experiments/edit-process-v1/instructions.txt --split dev \
  --out data/edit-process-v1/prompts.jsonl
```

## Profiles and exact perturbations

```sh
target/release/unslop ingest data/matched-pilot-v1/codex.jsonl --grammar
target/release/unslop ingest data/matched-pilot-v1/claude-opus.jsonl --grammar
target/release/unslop pilot-report --models codex-rewrites-v1 claude-opus-rewrites-v1
target/release/unslop profile --corpus codex-rewrites-v1 --family word
target/release/unslop perturb experiments/pangram-preface/sections/part_c/03.txt \
  --old accordingly --new thus --out data/candidate.txt
target/release/unslop inspect-pair experiments/pangram-preface/sections/part_c/03.txt \
  data/candidate.txt --grammar
```

SQLite defaults to `data/unslop.sqlite3`. The `word_counts` and `corpus_totals` views expose word occurrences and total opportunities. Per-document counts retain source groups for paired analysis and uncertainty. Profiles default to training data; exact-text and source-group leakage between splits is rejected. See [statistical conventions](docs/statistics.md).

## Detector experiments and subnet contract

```sh
target/release/unslop study data/original.txt data/candidate.txt \
  --out data/studies/example --repeats 3 --max-requests 6 --full-document
target/release/unslop score-corpus data/matched-pilot-v1/codex.jsonl \
  --out data/matched-pilot-v1/scores/codex --max-requests 8
target/release/unslop attach-scores --corpus codex-rewrites-v1 data/matched-pilot-v1/scores/codex
```

Pangram commands use `PANGRAM_API_KEY` and make paid requests. Inputs, model versions, task IDs, and complete results are retained. Request budgets and cached records are checked before submission; uncertain submissions cannot silently be submitted again. Use `--section` for fragment experiments. A passing full-document study requires at least three repeats and a human audit bound to exact source/candidate hashes. See [the audit template](examples/quality-audit.json) and [evaluation protocol](docs/evaluation.md).

The Rust subnet contract checks challenge/submission bindings and computes an offline reference reward after quality and detector gates. It does not establish network authentication, replay protection, cumulative budget enforcement, or an economical automated quality judge. [The subnet design](docs/subnet.md) describes those requirements and the route to a Rust validator implementation.

`unslop` is an independent local Git repo ignored by its parent `fix-slop` repo. Corpora, API responses, and derived databases stay in ignored `data/`; manifests, code, and reports are tracked. The preface copies and all earlier adversarial work live here.
