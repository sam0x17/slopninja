# unslop

A local research process for making prose easier to read while preserving its arguments, details, qualifications, and intended tone. The detector target is **less than 10% on every repeated full-document check**, with preservation and readability reviewed separately.

This is a working profiling and experiment toolkit. It is not a trained rewriting model, and reliable performance below 10% has not been demonstrated.

## What is here

- SQLite word counts, total words, document counts, and source-family counts, with exact provider/model provenance for new corpora.
- Sparse lexical and grammatical vectors: word ngrams, lemmas, POS sequences, dependency patterns, and sentence constructions.
- Smoothed frequency contrasts with raw evidence counts. These identify editing hypotheses, not a detector's hidden coordinates.
- Explicit single-word perturbations, before/after feature inspection, and resumable Pangram studies with bounded request counts.
- OpenAI and Anthropic corpus collectors with configurable model IDs and matched writing prompts.
- The entire preface experiment: source copy, revisions, 130 detector records, fidelity review, and rendered preview.

`unslop/` is an independent Git repository ignored by the parent `fix-slop` repository. The former parent-level `preface.tex`, `preface.txt`, and `experiments/` now live here. Historical absolute paths inside result records describe where the original requests were made; submitted text and hashes remain unchanged.

## Setup

Python 3.12 was used for the recorded grammar analysis. The lexical toolkit needs no runtime dependencies.

```sh
python3.12 -m venv .venv
.venv/bin/python -m pip install -e '.[grammar]'
.venv/bin/python -m spacy download en_core_web_sm
.venv/bin/python -m unittest discover -s tests -v
```

The initial environment used spaCy 3.8.16 and `en_core_web_sm` 3.8.0. Every extraction records tokenizer, Unicode, parser, model, and rule versions; comparisons reject mixed versions. `requirements-grammar.lock` records this working environment for reproduction on compatible platforms.

## Inspect the existing experiment

```sh
.venv/bin/unslop init
.venv/bin/unslop import-pangram experiments/pangram-preface/results --grammar
.venv/bin/unslop status
.venv/bin/unslop profile --corpus preface-experiments --split exploratory --family word
.venv/bin/unslop runs --corpus preface-experiments
.venv/bin/python scripts/reproduce_preface.py
```

The import contains 120 distinct submitted texts and 130 requests from **one underlying preface**. Exact duplicates do not inflate word counts; repeated detector calls remain separate observations. The corpus is marked `experimental` and `exploratory`, with unverified generator provenance. It is not a Codex or Claude frequency baseline. Some historical inputs are raw LaTeX and others are rendered prose; use this archive for experiments, not a pooled prose reference.

The database defaults to `data/unslop.sqlite3`. Derived databases and live output belong in ignored `data/`; checked-in scripts recreate the archived analysis. SQLite views `word_counts` and `corpus_totals` expose the requested word/occurrence/total structure. Document-level tables retain enough detail to filter, deduplicate, and investigate concentrated counts.

## Build comparable reference corpora

Use JSONL documents with `text`, `corpus`, `source_kind`, `domain`, `register`, `group_id`, and `split`. Model documents also require `provider` and the exact returned `model`. Preserve source URLs, licenses, dates, prompts, and other provenance in `metadata`. See [corpus design](docs/corpus.md).

```sh
.venv/bin/unslop ingest data/human-reference.jsonl --grammar
.venv/bin/unslop collect examples/prompts.jsonl --provider openai \
  --model "$UNSLOP_OPENAI_MODEL" --corpus openai-pilot \
  --out data/openai-pilot.jsonl --max-requests 6
.venv/bin/unslop collect examples/prompts.jsonl --provider anthropic \
  --model "$UNSLOP_ANTHROPIC_MODEL" --corpus anthropic-pilot \
  --out data/anthropic-pilot.jsonl --max-requests 6
.venv/bin/unslop ingest data/openai-pilot.jsonl --grammar
```

Set explicit model IDs and provider API credentials in the environment. The initial setup found the Pangram credential but no OpenAI or Anthropic API credentials, so no live model corpus was generated. The sample prompts exercise the pipeline; they do not constitute a sufficiently sized benchmark or a human reference.

```sh
.venv/bin/unslop compare --left openai-pilot --right human-reference \
  --domain software --register technical-review --family word
.venv/bin/unslop analyze data/paragraph.txt --reference human-reference \
  --domain software --register technical-review --grammar --family construction
```

Keep source groups in one of `train`, `dev`, `test`, or `exploratory`; the database rejects exact-text or source-group leakage between splits. Profiles default to training data. Analysis excludes a known input's source family from its reference; pass `--group-id` for a new variant whose family is known. Compare domain/register strata separately and balance their sample sizes before interpreting pooled results.

## Test an edit

```sh
.venv/bin/unslop perturb experiments/pangram-preface/sections/part_c/03.txt \
  --old accordingly --new thus --out data/closing-thus.txt
.venv/bin/unslop inspect-pair experiments/pangram-preface/sections/part_c/03.txt \
  data/closing-thus.txt --grammar
```

`perturb` changes one exact whole-word occurrence and writes an edit manifest with offsets and hashes. For a sentence restructuring, save a separate candidate and inspect the pair. Review the diff against the argument before scoring.

```sh
.venv/bin/unslop study data/original.txt data/candidate.txt \
  --out data/studies/example --repeats 3 --max-requests 6 --full-document
```

This command submits paid Pangram requests using `PANGRAM_API_KEY`. It alternates the order of baseline and candidate across repeat blocks, saves task IDs before polling, and resumes completed or pending requests. Use `--section` for fragment experiments. Invalid inputs and insufficient request budgets fail before submission. An uncertain submission remains recorded and cannot silently be submitted again; recover its task ID before resuming. Do not run two writers against the same study directory.

The strict gate uses `fraction_ai + fraction_ai_assisted < 0.10` and reports both fractions separately. It requires at least three repeats and a human review of readability, argumentation, detail, and tone to accept a full-document candidate. Copy [the audit template](examples/quality-audit.json), fill in the exact source/candidate hashes and your review, then pass `--audit`. Without a completed review the report remains unaccepted, even if scores are low. See [evaluation details](docs/evaluation.md).

## Research direction

The first useful test is whether lexical features predict score changes on new source families. Next compare grammar-only and combined features, then use the results to rank faithful edit candidates. Keep editing budgets fixed and evaluate a frozen process on untouched sources. The current preface results support context-sensitive experiments, not a universal list of forbidden words.

Read [the measured pilot](docs/pilot.md), [statistical conventions](docs/statistics.md), and [the possible Bittensor contract](docs/subnet.md). A subnet would need a reliable quality evaluator before detector scores could serve as rewards.
