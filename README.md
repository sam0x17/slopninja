<picture>
  <source media="(prefers-color-scheme: dark)" srcset="assets/brand/slop-ninja-full-logo.svg">
  <img src="assets/brand/slop-ninja-icon.svg" alt="Slop Ninja" width="280">
</picture>

# Slop Ninja

Planned domain: **slop.ninja**.

Slop Ninja aims to clean up prose so it matches a particular writer's profile
and evades both Pangram and the subnet's qualified AI detectors. These are
complementary goals of every B transformation. Remove unwanted generic or
mechanical patterns while preserving every argument, detail and qualification
and satisfying the target tonal intent. The target comes from the brief and
authorized examples of the writer's own work:
their word choices, grammatical constructions and rhetorical habits, including
deliberate informality and quirks. Use the writer's style as the standard for
cleanup.

The product thesis is that readers object to slop because the writing is weak
and recognizably generic. Remove those defects while retaining the substance
and individual voice, and the result should be good writing that sounds like
its author. That is what successful cleanup must demonstrate to readers.

The empirical goal is writing that readers cannot reliably distinguish from
that writer's own work in the same register. Blinded comparisons and independent
author-profile tests on held-out writing must establish how close we get, with
uncertainty and failures reported. The benchmark also measures evasion against
Pangram and subnet AI detectors. We test whether removing generic model habits
improves both author fit and resistance to detection; neither outcome guarantees
the other. The joint result remains to be demonstrated and would not establish
human authorship. Meaning preservation, readability, required cleanup and target
tonal intent remain hard acceptance requirements.

Read the subnet **[whitepaper (PDF)](whitepaper/slop_ninja.pdf)**, with editable
**[LaTeX source](whitepaper/main.tex)**.

The first experimental reference detector is trained and available under
`baseline/detector/`. Read the [measured results](baseline/detector/PILOT_RESULTS.md)
and download the [model bundle](https://github.com/sam0x17/slopninja/releases/tag/detector-pilot-v0.1.0).
Generator transfer and formatting sensitivity remain substantial weaknesses.
Its [commands](baseline/detector/README.md),
[implementation plan](baseline/detector/PLAN.md) and
[commercial-use source register](baseline/detector/SOURCES.md) cover acquisition,
training, evaluation and release. The [pilot protocol](baseline/detector/PILOT_PROTOCOL.md)
fixes the initial comparison and its evidence limits before training.
New corpora can use [recorded style profiles](baseline/detector/PROMPT_PROFILES.md),
including anti-AI instructions and a pinned Fix Slop pass. These profiles were
added after the first pilot and have not yet been evaluated in a fitted detector.
The whitepaper surveys related work and defines public A detector artifacts,
private B transformation models, training data,
two benchmark task interfaces, and paid asynchronous inference at market prices
denominated in subnet alpha.
B emissions, mandatory rewrites and all paid B jobs require qualified attested
execution: customers verify the
approved application, CPU/GPU evidence and instance key before encrypting
input. The miner retains its weights but receives no customer plaintext key.
The owner separately provisions encrypted model weights to the approved runtime;
its key service cannot authorize customer decryption. Ordinary hosting grants
operator access and is available only for A customer jobs, with explicit
acceptance. Customers can separately share evidence with a specific validator.
Task A detects author and origin; Task B jointly matches the authorized writer's
profile and evades Pangram and subnet AI detectors under the source and brief.
Private customer jobs pursue the same goals without automatically sending text
to Pangram. External measurement requires a separate customer-authorized
disclosure flow outside the confidential job protocol.
B miners train and search against published A models locally, then submit
self-scores bound to their final text and assigned panel. Validators rerun A to
verify those scores; no A inference endpoint or replay of B's generator is
required. Every credited B entry needs a verified execution receipt binding
its output to the named private version and bounded serving profile. Attestation adds no quality
bonus and never replaces semantic or Pangram checks.
B transformation is the primary paid product; A is an open detector resource
with optional hosting.
The [confidential inference design](whitepaper/sections/05-confidentiality.tex)
specifies key release, runtime isolation and remaining hardware assumptions.
The [investor brief](docs/confidential-transformation-brief.md) describes proposed
media workflows, an editorial pilot, miner and application revenue, and the
requirements for private B paid launch. B payment requires a TEE execution
receipt, complete encrypted result publication and a strict weighted validator
delivery certificate. Customer acknowledgment is unnecessary; validators receive
no customer decryption capability. Missed certification deadlines permit a refund.
The aim is freely accessible detection competitive with Pangram, supporting
better private transformers. Detection parity remains to be demonstrated;
running or hosting an A model still costs compute.
Pangram is A's bootstrap quality anchor for AI-origin detection. A subnet detector
takes over after demonstrating superiority; an improved Pangram becomes the
target again if it proves superior under the same frozen comparison protocol.
Certified changes apply to future epochs. At launch, compare Pangram and the
A origin artifacts on a fresh shared held-out reference set each A scoring epoch,
including the incumbent and challengers, with independently documented production
histories and common human false-positive constraints. The operator funds the
shared Pangram observations throughout. Those histories supply the origin labels;
Pangram supplies neither ground truth nor named-author labels. The moving target
does not change A's Brier utility or fixed public reward baseline.
Bounded training tickets assign A as requester and B as rewrite provider,
free to the requester and funded by the provider's expected emissions.
Deterministic hash assignments pair training counterparts on released,
authorized text. Each B comparison batch faces Pangram and a common three-A
panel, including the strongest qualified prior-round origin detector. Its
artifacts and thresholds are validated, cached and frozen before B task issue.
Three
hash-assigned preparers organize each submission's evidence. Final semantic
certificates require agreeing weight `3w > 2W` from the frozen eligible validator
set; every signer inspects the authorized encrypted benchmark evidence. The
conditional assumption is dishonest eligible weight below `W/3`, with no
denominator reduction for missing or abstaining validators.
Hidden labels, other miners' candidates and detailed review material remain
private until retirement; B can calculate its own public A scores. The whitepaper's
[reward rules](whitepaper/sections/07-rewards.tex) define fixed budgets,
mandatory assignments, aggregate/interface/role availability gates and recovery
deficits for earned A and B emissions. The [service-evidence rules](docs/service-evidence.md)
retain successes, failures, unresolved work and platform voids separately; the
[adjudication rules](docs/reward-adjudication.md) define exact style utility `V`
and certified preservation, readability and tone gate `G`.

The [assignment gameability experiment](docs/assignment-gameability.md) compares
peer selection, selective failures, committee collusion and recovery accounting
with reproducible Rust simulations. It records both blocked strategies and
attacks that pass availability checks while corrupting scores.
The [settlement hypothesis](docs/settlement-gameability.md) conserves complementary
policy credits but finds a counterexample after simplified native weight
normalization. That credit proposal and the [funded-alpha option](docs/alpha-match-reserve.md)
are retained as unadopted research into the earlier private-score design.
The [shared-pool experiment](docs/shared-pool-gameability.md) removes the earlier
miner-incentive counterexample under fixed-state assumptions, but reproduces a
historical-dividend attack using pinned Subtensor math. Current-only legacy
bonds remove that attack in the fixtures; native rounding and broader economic
claims remain unresolved. The selected [public-model design](docs/public-model-evaluation.md)
removes private A reports from scoring authority. Artifact replay, detector
qualification and copying incentives still need implementation and evaluation.

Every launch B benchmark measures author fit and detector evasion. Strict
detector success requires less than 10% AI-generated **plus AI-assisted** content
on the committed Pangram reports, a majority pass against the subnet panel, and
a pass against its strongest qualified prior-round origin detector at their
frozen thresholds. Report those outcomes alongside author fit and the semantic
gate. Historical single-objective experiments and optional-mode proposals are
ablations, not the launch policy; adapting the existing offline contract remains
implementation work.

The proposed launch cadence is one joint B benchmark round per scoring epoch,
with Pangram reports for every credited candidate. Miners fund candidate reports;
the operator funds shared source baselines and independent detector-parity
studies. Reducing B candidate checks requires demonstrated parity and a future
policy that retains periodic Pangram evaluation. A's ongoing Pangram comparison
remains required independently of any change to B's API frequency. The
[cadence and parity specification](whitepaper/sections/08-pangram-audits.tex)
defines that evidence; epoch throughput still needs measurement.

The local repository lives in `slop_ninja/`. CLI commands and artifact schemas
use the original `unslop` name. Historical experiment records retain their
original paths and hashes; resolve the old repository root to `slop_ninja/`
when replaying those records.

The CLI, corpus handling, feature extraction and geometry use **Rust**. Python supplies spaCy annotations and PyTorch author-scoring experiments. The project contains corpus collection, SQLite profiles, paired feature analysis, detector experiments, and an offline miner/validator contract. It does not yet contain a trained rewriting model or demonstrate reliable performance below 10%.

Current work is a **fresh composable base for grammar and word occurrence** in
[the `grammar/` Rust workspace](grammar/README.md). It exposes versioned feature
extractors, explicit high-dimensional coordinates and deterministic edit
possibilities. No LLM is needed to generate or measure those possibilities.
The feature representation and geometry can both change, which leaves room for
miners to develop them later. Preservation and readability still need independent
evaluation; distance in a miner's own space cannot establish either.

The [Blog Authorship workflow](docs/blog-corpus.md) now includes a Rust evaluator
for word-only, grammar-only and combined profiles. It compares chronological
held-out posts against training author profiles and a content-word baseline.
Corpus text, annotations and full results stay in ignored local `data/`.
The [learned metric experiment](docs/learned-metric.md) uses a small Rust model
to choose feature-family weights and tests their transfer to new authors.
The [model-size frontier](docs/model-size-frontier.md) compares 15 architectures
from 8 to 16,384 parameters across three seeds. Larger tanh networks improved
development scores without improving retrieval on the fresh author cohort.
In the [spline confirmation](docs/spline-confirmation.md), the frozen spline
scored 58.08% against the small model's 52.42% on another 100 authors. Its
paired confidence interval still included zero.
The [author-disjoint selection study](docs/author-disjoint-selection.md) now
chooses models on different authors from those used for fitting. On 300 new
test authors, selected words-plus-grammar scored 53.08% versus 52.09% for words
alone; the +0.99-point difference remained inconclusive.
The [coordinate-weighting experiment](docs/coordinate-weighting.md) learns
1,864 individual word and grammar weights around that frozen combined model.
On a further 300 authors, accuracy rose from 48.56% to 53.30%, a +4.74-point
gain with a paired 95% interval of [+2.67, +6.80]. Each of the three fresh
test galleries improved. Most weight changes were lexical; a separate ablation
would be needed to establish grammar's contribution to this gain.
The [coordinate capacity study](docs/coordinate-frontier.md) completed 192 fits
across sixteen vocabulary and grammar configurations. The 3,557-weight model
selected on 297 fresh validation authors scored 52.96% versus the incumbent's
51.06% on 300 new test authors. The difference was +1.91 points, with a 95%
interval of [-0.04, +3.69], so superiority remains unconfirmed. Larger catalogs gave little additional
gain, and grammar's conditional contribution remained uncertain.
The [training-author scale study](docs/training-author-scale.md) then held those
3,557 coordinates fixed and increased fitting coverage from 100 to 300 authors.
On 600 new authors, accuracy rose from 54.72% to 56.30%, with a paired gain
interval of [+0.51, +2.69] points. All six test galleries improved. Learned
grammar corrections added only 0.08 points conditionally, with an interval
crossing zero; the improvement remains mainly lexical.
An exploratory [content-proxy check](docs/content-proxy-diagnostic.md) found
30.23% accuracy on posts where a different author was a closer content match,
versus the control's 27.92%. This shows signal beyond that particular proxy;
it does not establish independence from topic.
The incumbent now has [standalone Rust inference](docs/coordinate-inference.md),
verified against every Python ranking on its earlier three test galleries.
The improved 300-author model also has [portable Rust inference](docs/scale-coordinate-inference.md):
all 361,800 candidate scores replayed within 5.69e-14, with every ranking and
tie group identical to Python.
The first [Global Voices transfer test](docs/global-voices-transfer.md) evaluates
the same frozen models on 168 articles from fourteen provisionally attributed
writers. Combined accuracy is 73.81%, versus 77.98% for complete lexical families
and 39.88% for grammar alone. Every query was scored. This small gallery has
topic and editorial confounds; the result does not support transferring the
learned grammar weights unchanged or labeling these articles human-only.
The first [document projection experiment](docs/author-projection.md) then tested
shared linear and two-layer encoders, using individual-document prototypes.
The selected linear model scored 32.82% on the same Blog test authors and 25.00%
on Global Voices. It improved its matching untrained control on Blog data but
lost ground on Global Voices. A separate diagnostic found that removing
unselected features from the old scorer alone reduced accuracy to 40.83% and
29.17%, respectively. Keep the full metric as the reference and preserve those
wider word and grammar distributions in the next learned model.
The [complete-family ablation](docs/full-family-ablation.md) clarifies grammar's
role: removing all grammar contributions lowers accuracy from 56.30% to
53.91%. The earlier reset experiments retained those contributions. A
[TRAIN-only representation audit](docs/grammar-representation-audit.md) found
that complete clause frames are sparse, motivating an additional family of
shared clause-child counts alongside the existing features.
The [matched clause-family study](docs/clause-family-study.md) found almost
no gain from that addition: 50.0812% versus 50.0882% on 397 new authors,
with a paired interval of [-0.161, +0.154] percentage points. Better support
for recurring patterns did not establish improved author recognition.
The next [grammar-choice audit](docs/grammar-choice-support.md) adds function
words conditioned on syntactic role and local predicate/operator sequences.
Across 3,889 training posts, recurring types cover 98.98% and 98.52% of their
observations. These remain diagnostic features while the
[edit utility matrix](docs/edit-utility-matrix.md) connects measurements to the
existing rewrite rules and explicit preservation questions. Five of six expected
suggestions and all six abstentions matched. Changing *may* to *must* improved
the frozen score against every tested target; swapping those modals between
claims left both new families and all tested scores unchanged. Choosing edits
by author proximity requires a separate claim-preservation check.
The [source-aligned claim guard](docs/claim-preservation.md) now supplies that
check for supported parser observations. All 19 synthetic cases expected to
change a claim or leave it unresolved stayed ineligible for selection. Two
desired sentence-boundary edits were also rejected after parser attachment
changes. The [grammar association comparison](docs/grammar-association.md)
found no clear retrieval gain from joint function-word roles over their simpler
counts, or from operator order and attachment over inventories. These reused
training-data results leave the working model unchanged.

The [conditional preference model](docs/conditional-edit-preferences.md) now
predicts a writer's contraction choice from their other writing dates, matching
the auxiliary and source-rule context. Personalization reduced Brier loss from
0.1598 to 0.1438 across 293 scorable authors, with all 300 authors retained in
coverage. It can rank exact grammar proposals with separate claim checks.
This result covers one grammatical choice family on reused training data;
readability, tone and detector performance remain separate questions.
The [later-post test](docs/preference-chronology.md) keeps those TRAIN profiles
fixed and evaluates 1,080 subsequent posts. Personalization reduced Brier loss
from 0.1450 to 0.1333 across 273 scorable authors, about 8.1%, with all 300
authors retained in coverage. In a separate
[comparison of exact edits](docs/preference-score-comparison.md), the neural
author score agreed with conditional preferences in only 19 of 90 raw
comparisons and 3 of 90 after adding unrelated context. Author recognition
alone is insufficient evidence for using that score to select grammatical edits.
The [boundary-choice eligibility test](docs/boundary-choice-eligibility.md)
then checked period/semicolon alternatives across all 3,889 TRAIN posts.
None passed the paired checks: the rules proposed 103 joins and no splits,
and none of the joins had a recognized inverse. The saved parses changed a
matched predicate from `ROOT` to `ccomp` in 100 of those joins. Parser/rule
compatibility needs further work before fitting this structural preference.
The [parser comparison](docs/parser-variants.md) then tested 24 synthetic pairs
with spaCy's small, medium and transformer models, including the small model
in two environments. Each configuration emitted 10 joins and no splits;
none passed the paired checks. All declared exclusion checks held. Switching
among these parsers did not resolve the boundary-rule failure on this screen.
The [local clause view](docs/boundary-view.md) retains the original attachment
while comparing clause structure across a period/semicolon edit. All five fresh
ordinary pairs and all four reporting or stance-tail pairs matched in both
directions. The latter examples leave attribution unresolved, so structural
agreement alone cannot license these edits. The other 15 fresh pairs remained
unsupported; they are not successful structural-preservation checks.
The [lexical-frame ledger](docs/lexical-frames.md) now connects possible VerbNet
roles to specific participants and embedded propositions. In 96 frozen synthetic
cases it retained every resource alternative and kept all 16 masked predicates
explicitly unknown. Complete syntax bindings were available for 33 of 48
confirmation targets; 23 of 112 confirmation role assertions failed. All 24
controlled participant edits changed the raw argument signature, including 14
with identical word counts. Parser attachments and unsupported mappings still
limit coverage, and semantic compatibility remains unresolved.
The [lexical-frame parser comparison](docs/lexical-frame-parsers.md) held that
matcher fixed across four parser configurations. On 64 fresh texts, complete
bindings covered 36/67 targets with small, 47/67 with medium and 46/67 with
transformer. All configurations tracked the eight embedded participant checks.
Transformer's four fresh completion losses involved a prepositional `dative`
label that the baseline matcher does not support. No author-style or detector
gain follows from these counts.
The [versioned dative mapping](docs/lexical-frame-datives.md) now supports that
ADP form while preserving baseline behavior and raw annotations. On 24 new
texts, transformer annotations met the expected complete recipient-frame
checks in 10/10 positive sentences, up from 2/10, and all 16 incomplete-frame
checks held. Baseline replay matched all 640 saved cases.
The [dative alternation pipeline](docs/dative-alternation.md) adds double-object
binding, exact reversible noun-phrase moves and checks against reparsed
participant occurrences. It materializes word-and-grammar coordinate changes
against fixed author profiles; a smaller distance remains a diagnostic rather
than evidence of author preference or preserved meaning.
The subsequent [corpus audit](docs/dative-corpus.md) completed all 3,889 training
posts from 300 authors with both parsers. The transformer found 169 complete
constructions, but the current rules proposed only one edit and confirmed none
in both directions. Even the broader construction counts lacked the repeated
author/date support required by the proposed preference model. Coverage of
ordinary argument phrases needs work before fitting that model.
The [role-aware argument experiment](docs/dative-arguments.md) separates the
unchanged subject from moved objects and adds explicit phrase evidence for
pronouns and simple possessives. Its fixed evaluation combines 64 fresh
synthetic sources with a replay of the cached training corpus. The transformer
produced 60 corpus proposals, but only two passed reciprocal checks; the small
parser passed 32 of 63. A synthetic exclusion also exposed a punctuation defect
that both parsers accepted. Phrase coverage improved, while comparison
reliability and repeated author-choice support remain unresolved.
The [identical-text parser controls](docs/parser-repeat.md) check that unresolved
comparison problem on the same 57 source posts and 60 candidate texts under
both parsers. Adjacent repetitions, fresh interpreters and reversed input order
separate observed repeat variability from comparisons involving edited text.
All 1,638 controlled repeat comparisons and all 221 available historical
comparisons were identical, leaving the differences after edits unexplained.
The [sentence-context diagnostic](docs/sentence-context.md) then kept all 60
edits and compared saved sentence projections with isolated parses. The
transformer passed 32 local comparisons in each view, compared with two under
the original whole-post gate. Four edits gained a pass and four lost one after
reparsing. Narrowing the comparison explains the increased count; isolated
parsing gave no net gain, and local agreement does not validate the rewrites.
The [revised structural comparison](docs/evidence-contracts.md) then increased
primary saved-sentence acceptance from 32 to 44 of 60 while retaining all prior
passes. On fresh synthetic cases, it accepted 36 intended rearrangements and
rejected four terminal-punctuation movements under the transformer parser.
These results concern edit filtering; writing quality remains untested.

The earlier author-profile and LLM candidate-review prototype is preserved in
the existing CLI. Its [workflow](docs/style-space.md) and
[research notes](docs/style-research.md) remain available. Private reference
samples, extraction notes, profiles and local edit reports belong in ignored
`data/grammar-private/`.

## Current data and findings

- 12 licensed scientific abstracts published in 2018, matched with 12 Codex rewrites and 12 Claude Opus rewrites. Source groups were assigned to 8 training, 2 development, and 2 test groups before generation.
- 100 FineWeb documents captured in 2021: 93,458 whitespace words from 94 source hosts, with pinned revision and response hashes. This is a deterministic first-row pilot, not a representative random sample or verified human-authorship ground truth.
- Eight additional Claude Fable outputs and a rejected model-fallback invocation are retained separately. They do not enter the complete matched comparison.
- The earlier preface experiment remains available in ignored local files: 120 distinct inputs and 130 Pangram requests from one source family. Its source text and derived artifacts have been removed from Git history.

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

Grammatical extraction uses a local Python environment:

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

The [private receipt prototype](docs/receipt-envelope.md) encrypts exact payloads
to signed validator keys, checks salted commitments, and can retrieve an existing
Pangram report after decryption. Chain assignments and replay accounting remain
outside the prototype.

The [two-task mechanism proposal](docs/subnet-mechanisms.md) defines author/origin
detection in on-chain mechanism 0 and author-directed transformation under the
source and brief, jointly targeting author fit and evasion against Pangram and
subnet AI detectors, in mechanism 1. Each receives
50% of pilot emissions. Fidelity, readability and the requested audience and
style controls remain transformation gates and brief requirements.
The [model plan](docs/miner-models-and-data.md) specifies starter
classifiers and an editing adapter, with a [training-source plan](docs/miner-training-sources.md)
covering Global Voices, PLOS and consented writing. A artifacts are public,
B weights remain private, and miners can earn customer fees through
[asynchronous inference](docs/paid-inference.md)
with proposed on-chain job settlement and private execution. Draft 0.19 requires
qualified attestation for B emissions and paid service, with independent key
release and version-bound receipts. The [consistency review](docs/design-consistency-review.md)
records the incentive corrections and remaining launch requirements.
Pangram is the required external benchmark. The [API funding and verification
design](docs/pangram-oracle.md) separates the cost of independent measurement
from the remaining trust in Pangram and human quality judgments.
The [repeatability probe](docs/pangram-repeatability.md) found matching document
fractions across fresh requests for three texts, with varying auxiliary window
scores. It provides no near-10% observation or general determinism guarantee;
miner selection of committed reports can still bias observed pass rates.
The [public-result reader](docs/pangram-public-results.md) checks an existing
Pangram report without buying another inference. The
[receipt and audit proposal](docs/subnet-receipts-and-audits.md) uses provider
reports, three evidence preparers and final global weighted certificates.
The [audit rules](whitepaper/sections/08-pangram-audits.tex) retain native
validator weight commit-reveal; [adjudication](docs/reward-adjudication.md)
defines field ballots, private openings and fixed closure deadlines.

`slop_ninja/` is an independent local Git repo ignored by its parent `fix-slop` repo. Corpora, API responses, and derived databases stay in ignored `data/`; reusable manifests, code, and reports are tracked. Preface copies, derived experiments and the report containing source excerpts remain local and ignored.
