# Matched pilot v1

We collected 12 PLOS ONE abstracts published in 2018 and gave the same source-conditioned rewrite task to Codex and Claude. The reusable collectors, profiler, detector orchestration, and subnet contract are Rust. spaCy parsing remains behind a Python subprocess boundary.

## Data and protocol

The [pinned manifest](../experiments/matched-pilot-v1/human-source-manifest.json) records DOI, authors, date, article-specific CC BY 4.0 license, XML hash, and abstract hash. There are four abstracts each in ecology, public health, and molecular biology, with 86 distinct author names and 2,860 whitespace words overall. These are broad overlapping publisher categories, all within the scientific-abstract register.

Before model generation, eight source groups were assigned to training, two to development, and two to test. [Identical prompts](../experiments/matched-pilot-v1/prompts.jsonl) specified preservation of facts, numbers, qualifications, methods, results, conclusions, and register. They targeted the original length within roughly 10%. This is a rewrite experiment: source vocabulary and phrasing are available to each model, so it does not characterize independent writing from a short brief.

Codex CLI 0.153.4 produced 12 accepted samples with requested model `gpt-6-astra` and low reasoning effort. Its JSON events do not report the resolved model; the corpus explicitly records `requested:gpt-6-astra`, rather than inventing an observed model identifier.

Claude Code 2.1.263 initially produced eight `claude-fable-5-1` samples, then switched to `claude-opus-5` on the ninth invocation. The collector rejected that fallback for the Fable cohort and kept all captured events. A separate, explicitly requested Opus cohort then produced 12 samples at low effort, each reporting `claude-opus-5`. Fable's incomplete cohort is preserved but excluded from the complete matched comparison. There were 33 corpus-generation invocations: 32 accepted outputs and one excluded fallback. Two short access checks are stored separately.

The CLIs used fresh sessions and disabled tool use; invocation arguments, prompt records, usage, timestamps, exact output, and event streams were retained. Their built-in system instructions remain a potential style influence. Old publication dates support human provenance but do not rule out detector or generator training exposure.

## Lexical and grammatical observations

The [Rust profile report](../experiments/matched-pilot-v1/profile-report.json) analyzes **training groups only**. Each comparison checks exactly one model and human document per group and matching topic/register assignments. It reports pooled counts and rates with equal weight for each source group. Heuristic z ranks are not significance tests.

| Construction | Human count / sentences | Codex count / sentences | Opus count / sentences |
| --- | ---: | ---: | ---: |
| Passive sentence | 37 / 82 | 22 / 83 | 24 / 78 |
| Relative-clause sentence | 6 / 82 | 12 / 83 | 16 / 78 |
| Pronoun-subject sentence | 15 / 82 | 20 / 83 | 28 / 78 |

Codex's passive-sentence rate decreased in all eight pairs. Opus's pronoun-subject rate increased in seven pairs and its relative-clause rate increased in seven. These are parser-derived patterns, not judgments about animacy or writing quality. Removing passive constructions or insisting on human grammatical subjects would not follow from these observations.

Among word coordinates, Codex reduced the rate of “of” in seven pairs and increased “these” in five. Opus reduced “was” in seven pairs. These are candidates for further controlled study, not replacement rules. A word can correlate with a model's rewriting choices without causing a detector response.

Numeric-token multisets matched in six of eight Codex pairs and five of eight Opus pairs. This check is deliberately literal: a mismatch can reflect changed formatting or an omitted/repeated number. It requires inspection, and matching does not prove preservation of claims. Every pair remains marked `fidelity_status: not_reviewed`. No readability improvement has been certified.

## Initial detector observations

We made 24 Pangram 4 requests, one per training document in the three complete cohorts. No development or test document was scored. Exact records live under `data/matched-pilot-v1/scores/`; the database attaches them to their attributed documents.

Tracked summaries retain each input hash, detector version, task ID, and measured fractions for the [human](../experiments/matched-pilot-v1/human-detector-report.json), [Codex](../experiments/matched-pilot-v1/codex-detector-report.json), and [Opus](../experiments/matched-pilot-v1/claude-opus-detector-report.json) cohorts. Reproduce them with `unslop runs --corpus human-plos-abstracts-v1`, `--corpus codex-rewrites-v1`, and `--corpus claude-opus-rewrites-v1`. The reports mark repeated success false because each input has only one observation.

| Cohort | Mean AI + assisted fraction | Below 10% in this single check |
| --- | ---: | ---: |
| Human originals | 0% | 8 / 8 |
| Codex rewrites | 36.39% | 3 / 8 |
| Claude Opus rewrites | 72.44% | 1 / 8 |

Every Codex rewrite received 0% AI-generated, but five received a nonzero AI-assisted fraction. Seven Opus outputs were marked partly or wholly assisted/generated; one was fully AI-generated. The user-supplied [Pangram model card](https://www.pangram.com/research/model-card/pangram-4) distinguishes these categories. Optimizing AI-only would conceal the primary detector response in this experiment.

These observations do not demonstrate a successful editing process. The low scores have not passed repeat confirmation or a human preservation/readability review. Do not infer a provider-wide ranking from eight source-conditioned samples or from CLI runs with different model and system settings.

## Public-corpus expansion and the subnet

The first real public reference sample contains 100 FineWeb documents captured in October 2021, with 93,458 whitespace words from 94 hosts. The Rust loader verifies dataset revision, dates, language, complete rows, response hashes, and duplicates. The [dataset catalog](public-datasets.md) distinguishes actual releases from the proprietary evaluation slices named in Pangram's card. This broad web sample is not a topic-matched substitute for the PLOS cohort.

For the eventual adversarial subnet, the current Rust contract binds source, revision, challenge nonce, detector version, and quality audit before computing an offline reward. Validators will still need authenticated submission state, cumulative query accounting, replay protection, and a calibrated automated quality evaluator with human audits. A lower score must not compensate for deleting an argument or changing its qualifications.

Next, freeze a small set of lexical and structural edit hypotheses from training profiles. Test them on the development groups with repeated full-context controls and preservation reviews. Keep the test groups untouched until the candidate-selection process is fixed. Larger corpora and fresh private tasks are necessary before making a claim about general reliability or subnet reward economics.
