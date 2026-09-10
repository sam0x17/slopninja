# Prepositional dative mapping

The [parser comparison](lexical-frame-parsers.md) found a specific compatibility
gap: spaCy's transformer sometimes uses `dative` for the preposition in a
recipient phrase. The baseline matcher accepts a direct ADP `prep` with its
own nominal `pobj`, but leaves the corresponding `dative` unsupported. The
participant anchors and verb attachment can otherwise agree.

The shared Rust observer now has an explicit mapping choice. `BaselineV1`
preserves the original report schema and behavior. `PrepositionalDativeV1`
adds a slot for a lexical ADP `dative` directly attached after a VERB, with
its own following nominal `pobj`. The existing frame matcher still checks
the resource's literal preposition, slot order, competing arguments and head
support. A bare nominal dative does not qualify for this new slot.

The extension keeps every original token, dependency label, parent, span,
predicate event and resource alternative. It changes frame interpretation;
it does not rewrite a parser annotation. Extended reports use observation
schema v2 and `mapping_identity: prepositional_dative_v1`. Existing observer
APIs continue to use baseline v1 and omit the optional mapping field.

## Comparison design

Before evaluating the extension, the study reproduces every historical case
row from the two earlier fixture panels and all four parsers: 640 comparisons
covering the observation, expectations, target coverage, errors and resource
retention. Exact JSON-value equality is required for every baseline row.
No extension comparison or new parsing starts until all of them pass.

The original producer executable and executed source snapshots remain bound
to their historical receipts. The current observer and study sources now
share a versioned mapping API and case evaluator. Their new hashes are bound
by the dative protocol. Historical source verification uses the executed
snapshots, since the live paths have intentionally evolved. This avoids
duplicating the entire matcher to preserve a baseline comparison.

Both mappings receive the same immutable Document. Changes to raw ledgers,
predicate discovery or resource alternatives stop the study. Observation-body
comparison excludes only the declared schema and mapping-identity fields.
Unavailable observations remain unavailable, with errors and coverage counts
retained. Changed extended cases are saved in full; unchanged bodies explicitly
reference their bound baseline case.

A separate [fresh fixture](../grammar/fixtures/lexical-frame-dative-v1/manifest.json)
contains 24 texts, 12 each for `lend` and `loan`. It covers ordinary explicit
recipient phrases, repeated-name recipient/possessor swaps, bare double
objects, wrong prepositions, missing objects, relative-clause attachment,
ambiguous noun-modifier attachment and an absent recipient. Its 132 assertions
include 30 complete-required and 28 partial-allowed role checks. Two controlled
pairs exchange the intended recipient and possessor. Their production word
counts differ: the word extractor keeps an internal apostrophe, so `Mira's`
and `Theo's` are distinct words. They test participant anchors but do not
isolate grammar from word-occurrence changes.

The initial fixture builder compared lowercase alphabetic runs, splitting
`Mira's` into `Mira` and `s`. The producer rejected the resulting two equality
flags before any fresh annotation. `fixture-build-v2` recomputes those flags
with the original `grammar_core::features::words` implementation. It preserves
all 24 texts, exact patches and role expectations, and verifies that only those
two booleans change. The original builder, manifest and receipt remain in
`fixture-build-v1`.

All fresh rows use existing lemmas and classes; they are new text contexts,
not a lemma or class holdout. Ambiguous noun-modifier phrases have no
unconditional frame-completeness verdict. Conditional mapping evidence records
the exact observed token anchors and parents. Missing annotation anchors are
distinct from a tested failed mapping condition. Bare indirect objects retain
their observed POS, label and parent even though no ADP carrier is declared.

The four pinned parsers run once each on the 24 fresh texts, in fixed order.
All 160 historical texts are reused development evidence. The resource and
parser assets remain unchanged. There is no per-case parser selection or
outcome-dependent text replacement. No author corpus, model fit, LLM or
detector is involved.

## Results

The revised run completed with 96 new annotations and no parse or observation
errors. All 640 historical case rows reproduced exactly before the extension
comparison. Across all 736 parser/case comparisons, the extension caused no
previously met expectation to fail and lost no complete target binding. It
preserved every raw ledger and resource alternative. These are results on
the declared panels, not a guarantee for every possible annotation.

The fresh panel has ten positive sentences, each with three complete-required
role assertions for the specified Agent, Theme and Recipient. Sixteen other
frame assertions require incompleteness. The two ambiguous noun-modifier
contexts have no completeness verdict.

| Parser | Positive frames, baseline / extended | Complete-required roles, baseline / extended | Incomplete-frame checks, extended |
| --- | ---: | ---: | ---: |
| Small, either environment | 0/10 / 0/10 | 0/30 / 0/30 | 14/16 |
| Medium | 3/10 / 4/10 | 9/30 / 12/30 | 16/16 |
| Transformer | 2/10 / 10/10 | 6/30 / 30/30 | 16/16 |

The two small-model incomplete-frame failures concern the missing target in
`Mira loans Theo a ticket.`. Small parses `loans` as a noun and `Theo` as the
verb root. Those failures are missing target evidence, not completed frames
that should have been rejected. The partial-allowed role counts stay unchanged:
26/28 for small, 28/28 for medium and 24/28 for transformer. Transformer's four
remaining mismatches are Agent/Theme assertions in the two `from` controls.
Their selected `to` frame is contradicted by the observed literal preposition;
the evaluator excludes bindings from a contradicted frame even when partial
syntax is allowed. Those mismatches remain in the results.

| Panel | Small complete targets, baseline / extended | Medium | Transformer |
| --- | ---: | ---: | ---: |
| Reused 96 texts | 41/96 / 41/96 | 49/96 / 49/96 | 52/96 / 57/96 |
| Reused 64 texts, 67 targets | 36/67 / 37/67 | 47/67 / 48/67 | 46/67 / 56/67 |
| Fresh 24 texts | 18/24 / 18/24 | 17/24 / 18/24 | 6/24 / 16/24 |

This second table counts a complete binding in any possible frame. It must
not replace the intended-role checks above. Small often completes the shorter
Agent/Theme frame while attaching the recipient phrase under the object noun;
the selected three-participant frame then remains incomplete. Transformer's
ten fresh completion gains comprise eight positive recipient frames and two
ambiguous noun-modifier cases. The latter gains establish mapping applicability
under the observed attachment, without resolving the intended interpretation.

Both fresh recipient/possessor swaps change raw argument signatures for all
parsers. With the extension, transformer also distinguishes both pairs by
complete-frame signatures. Medium leaves both complete signatures unchanged;
small leaves one unchanged and the other unavailable. Those summaries can omit
the intended recipient when only a shorter frame completes. An unchanged
signature is therefore not evidence that the participants or meaning stayed
the same. The production word counts also differ in these pairs, as described
above.

All 608 workspace tests, formatting, Clippy with warnings denied and the
release build passed. The added tests cover the new slot's direct attachment,
POS, object, order and literal-preposition conditions; occurrence identity;
baseline serialization; and unavailable comparison evidence. No observed frame
received checked full semantic compatibility. The transformer `from` controls
retain four contradicted frame alternatives; the other full-compatibility
statuses remain unresolved.

## Interpretation

More complete bindings would establish coverage for this mapping on these
annotations. A passing incomplete-frame control would establish the tested
exclusion only. Neither result selects a verb sense or proves a natural-text
interpretation. Full semantic compatibility and automatic edit licensing
remain unresolved. Author-style scoring and rewriting need separate tests.

Local protocol, execution logs, snapshots and outputs belong under
`data/author-corpora/blog-authorship-2004/lexical-frame-dative-v1/`.
The study command is `slop_ninja-lexical-frame-datives` in the grammar workspace.

| Record | SHA-256 |
| --- | --- |
| Active `protocol-v2.json` | `f053c410b46c39e6f68d01d59e3957e90a5b6505b725b396389be3f158dab487` |
| `run-v2/report.json` | `895c4a71d48c912eaabc29127e5c541fc41bb2171ad1743c8b6d66c23346f725` |
| `run-v2/receipt.json` | `b7309bc1d686e832080420bd8d7ec97d9598f6e7b11de986fa0ee3137574796f` |

The first attempt and its fixture-validation failure remain under `run-v1`.
It completed historical replay and reused-data comparisons, then stopped
before fresh parsing. The revised protocol changes only the fixture hash and
correction provenance. The second run repeats the required historical checks
and supplies the only fresh annotations. It does not tune the mapping or
replace texts after observing their outputs.

The next step toward actual edit candidates is a separate mapping for the
double-object frame. The current matcher deliberately leaves multiple
unmarked postverbal noun-phrase slots unresolved. Supporting them requires
matching the whole ordered construction: one bare indirect object and one
distinct direct object, bound to the resource's Recipient and Theme slots.
Simply removing the ambiguity warning would permit incorrect assignments.
Once both constructions are supported, an explicit frame-pair registry can
propose noun-phrase movements and `to` insertion/removal. Those candidates will
need inverse patches, participant alignment, surrounding-claim checks and
feature-vector scoring before they can support author-style editing.
