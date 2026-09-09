# A local view of period and semicolon boundaries

This study tests whether a narrowly defined dependency difference accounts
for some period/semicolon pairs that the existing guard rejects. It also
tests whether reporting or stance tails share that structure. Both questions
matter: a useful local representation could still omit who asserted a claim.

Use the saved incumbent-small annotations for the preceding 48 synthetic
directions. Parse only the exact new challenge texts, once per distinct full
string, using the same bound parser and CPU environment. The old examples
are developmental regressions. The challenge set must be frozen before its
annotations or structural outcomes are read. It contains 24 reciprocal pairs:
five ordinary controls, four reporting or stance tails, and 15 exclusions.
The four pure tails provide eight directed collision probes. The broader
attribution-sensitive label covers 16 directions because it also includes
comma-aside, complement and quotation cases; keep those denominators distinct.
There is no corpus work, model
training, style scoring or parser selection in this stage.

## One specific structural comparison

The new module compares a source, a candidate and their actual UTF-8 edits.
It identifies the period and semicolon forms in either direction. The
unchanged `join_independent_sentences` descriptor must supply an exact period
to semicolon witness. Equal target text and equality of the actual patch
spans are separate facts; a broad replacement cannot supply missing token
correspondence merely because its final text matches.

The proposed comparison requires two adjacent sentences in the period form
and one semicolon boundary between the same anchored clauses. Only the
literal punctuation exchange, one-space gap and the first-token casing
allowed by the witness may change. Each clause retains its own finite
predicate and preceding subject. Preserve the existing witness's scope,
quotation, code, paragraph and coordination restrictions.

The exceptional semicolon edge has exactly one permitted topology: the
left predicate depends on the right sentence root through `ccomp`. It is
the only non-punctuation edge crossing the boundary. In the period form,
both aligned predicates are self-headed roots. A different direction,
endpoint or extra crossing edge does not qualify.

Removing that recorded edge must leave two connected, acyclic local trees.
Their other content-token dependencies and aligned parents must match the
period form. Preserve all lexical anchors, lemmas, POS, tags and finite or
operator observations. Subjects, objects and modifiers cannot change sides.
Only the edited boundary punctuation may change its punctuation attachment;
the remaining punctuation stays checked. Additional complement, subordinate,
relative, modal or negative structure remains outside this comparison.

Report structural agreement, a differing condition or unavailable evidence.
Retain the period-rule witness, actual edits, token correspondence and every
condition's evidence. Neither source Document is rewritten. Retain the raw
cross-boundary edge with its original endpoints, label and token-side
membership beside the local representation. Otherwise, a later consumer could
mistake information removed by normalization for information absent from the
source.

The old claim guard runs separately on the original Documents and edits.
Its changed or unresolved outcome remains visible even when the new local
comparison agrees. Every new report keeps `edit_licensed: false`,
`automatic_edit_license: false` and
`semantic_equivalence_certified: false`.

## Reporting tails as falsifiers

A pair such as `The road was closed; the guard said.` and
`The road was closed. The guard said.` could satisfy the same local edge
conditions as an ordinary action pair. Yet the reporting tail raises an
attribution question. The experiment must record whether such a collision
actually occurs; the constant absence of an edit license is not a successful
test of that question.

Report agreement among intended independent-clause probes and among the
separately labeled reporting or stance tails. Identify each agreeing tail,
its original edge and both directional outcomes. If a tail agrees, the
structural certificate cannot alone establish independent assertions. If
none agree, report only the absence of a collision among these fixed probes.
Do not infer that all other reporting or stance predicates are safe.

Complement insertion/deletion, quotation, scope changes and altered subject
or object anchors have separate exclusion obligations. Expected structural
exclusions must be tested against the structural outcome itself. Constant
license or certification flags cannot satisfy those assertions. Unspecified
structural outcomes remain unspecified. In particular,
`boundary_view_must_not_agree: false` imposes no agreement expectation.
Parser errors are unavailable, not
successful exclusions.

## Coverage and stopping

Keep every case, including failed parses and unavailable witnesses. Summaries
separate the old fixture panels from the new challenge, and separate ordinary
probes, attribution tails and exclusions. Report each direction and its
shared undirected pair. Distinguish agreement in both directions, agreement
in only one direction, no agreement with both directions available, and
unavailable directions. These repeated views of the same text pair are
correlated observations.

Freeze one set of edge conditions and one challenge panel. Do not widen the
conditions after reading failures. An agreeing attribution tail defeats use
of structural agreement alone as an automatic edit gate. A structural
exclusion mismatch requires explanation before extending the representation.
Keep the raw evidence and stop automatic licensing where attribution or
valency remains unknown; any later predicate/frame policy needs its own
design and evaluation.

An independent saved-result audit checks bound inputs, exact patches,
condition evidence, removed-edge retention, case and pair summaries, and
the separation from the old guard. It does not rerun NLP or claim to certify
meaning, discourse, tone or readability.

## Results

The local comparison agreed in both directions for 19 of 48 pairs, including
all five new ordinary controls and all four new reporting or stance tails.
Those tails share the tested structural pattern with the ordinary examples.
The comparison cannot resolve attribution and does not justify an automatic
boundary edit. Every edit-license and semantic-certification flag remained
false.

| Fixture panel | Directions | Structural agreement | Structural difference | Unavailable | Pairs agreeing both ways |
| --- | ---: | ---: | ---: | ---: | ---: |
| Reused boundary choices | 24 | 10 | 0 | 14 | 5 / 12 |
| Reused parser counterexamples | 24 | 10 | 0 | 14 | 5 / 12 |
| New boundary challenges | 48 | 18 | 0 | 30 | 9 / 24 |
| Total | 96 | 38 | 0 | 58 | 19 / 48 |

No pair agreed in only one direction. The remaining 29 pairs were unavailable
in both directions. There were no pairs with a completed structural comparison
that disagreed. Directional counts describe repeated views of each pair, so
38 agreeing directions do not represent 38 independent examples.

The new ordinary controls covered transitive clauses, intransitives,
first-person `I`, past copulas and the name `René`. All ten directions agreed.
All eight directions of the four pure tails also agreed:

| Semicolon form | Retained exceptional dependency | Directions agreeing |
| --- | --- | ---: |
| `The road was closed; the guard said.` | `closed` depends on `said` through `ccomp` | 2 / 2 |
| `The server stopped; the editor believed.` | `stopped` depends on `believed` through `ccomp` | 2 / 2 |
| `The engine failed; the mechanic reported.` | `failed` depends on `reported` through `ccomp` | 2 / 2 |
| `The shipment arrived; the driver claimed.` | `arrived` depends on `claimed` through `ccomp` | 2 / 2 |

In every agreeing case, the semicolon annotation attaches the left predicate
to the right root through `ccomp`; the period annotation gives each predicate
its own root. The saved comparison retains that exceptional edge, its raw
endpoints and the other crossing punctuation evidence. A post-hoc check
reconstructed the complete crossing-edge lists directly from the saved
annotations for all 38 agreeing directions and found exact equality.

The tail labels identify attribution-sensitive probes, without supplying a
semantic verdict about either punctuation form. Their agreement shows that
the local representation leaves an attribution question unresolved. The
broader attribution-sensitive set contains 26 directions across the three
panels: eight agreed and 18 were unavailable. Within the new panel, that
breakdown is eight agreements and eight unavailable directions. These totals
include complement, quotation and comma-aside cases as well as the pure tails.

### Unavailable evidence and the unchanged guard

All 58 unavailable directions stopped at the same earliest condition,
`original_period_join_witness`: the unchanged join rule supplied zero exact
full-text witnesses. The comparison therefore never established the anchors
needed to extract its boundary trace. The stored empty `crossing_edges` arrays
in those records mean that extraction was not reached; they do not establish
that the raw annotations contain zero crossing edges.

All 56 frozen `must_not_agree` assertions belong to those unavailable records.
Their structural test denominator is zero and their violation rate is null.
They cannot be counted as passed structural exclusions. The other two
unavailable directions are the reused `The service reads records. Users check
results.` probe. Consequently this run provides no completed challenge test of
the deeper exclusion conditions against modal, scope, quotation or argument
changes. Those conditions still have synthetic unit tests, but the fixture
study stopped earlier for these examples.

The original claim guard was evaluated separately for every direction. It
reported 90 changed and six unresolved outcomes, with zero observed-preserved
outcomes. All 50 declared guard-rejection expectations held. That result has a
different denominator and meaning from the unavailable structural assertions.
The original proposer emitted 19 exact text-and-patch joins and no inverse
proposals; it produced zero paired eligible directions. The new comparison's
stored mechanical inverse patches do not count as emitted inverse proposals.

Before parsing the challenge, the runner exactly replayed the original guard,
observation, proposal and pair-result fields for all 48 reused directions.
There were no differences from the saved incumbent results. All 96 annotation
records were valid: 48 reused records and 48 newly parsed texts, with no parser
errors. The new texts were processed in one batch using spaCy 3.8.16 and
`en_core_web_sm` 3.8.0 on CPU.

Parsing and writing the 48 new annotations took 0.904 seconds. The historical
replay took 0.015 seconds, and subsequent evaluation and result writing took
0.027 seconds. These are single-run timings that include process, transport
and file-output overhead where applicable; they do not estimate steady parser
throughput. Workspace formatting, Clippy and 520 test invocations across 37
suites passed, including 19 installed-parser test invocations. Shared module
tests repeat across executable targets, so this is an invocation count.

The independent saved-result audit passed on its first frozen execution. It
verified all 96 case rows, 48 pairs, 99 directional summaries, 35 pair summaries
and the retained raw crossings and residual projections. This establishes
consistency with the saved annotations and protocol; it does not adjudicate
the meaning of a reporting tail.

The evidence supports retaining this representation as a local structural
diagnostic. Automatic licensing remains blocked by the agreeing attribution
tails and by the unavailable exclusion tests. A later study would need an
explicit attribution or predicate-frame policy and challenge cases that
actually reach its exclusion checks. This run measured no author preference,
readability, tonal fidelity or detector performance.

## Reproduction and retained records

From the repository root, the command shape is:

```sh
study=data/author-corpora/blog-authorship-2004/boundary-view-v1
grammar/target/release/slopninja-boundary-view \
  --protocol "$study/protocol.json" \
  --expected-protocol-sha256 0d7cf7f681f1244f304fceab4007827309de2a1c0ecb4d32e4db54216e4921ba \
  --repo "$PWD" \
  --out "$study/run-new"
```

The protocol binds the executable hash, source snapshot, fixture labels,
parser assets and reused annotation/result hashes. A reproduction requires
those exact assets and a fresh output directory. The completed execution is
`run-v1`; its receipt SHA256 is
`894570d43139be36275401a34e79569aa9b412132b3ebfb6dba6c97c4ace445a`.
Its `report.json`, per-fixture case and pair files, annotation records and
`baseline-check.json` remain unchanged under the ignored study directory.

The bounded saved-data diagnostic is retained in
`posthoc-saved-view-v2/diagnostic.json`, with its jq program, command script,
input hashes and receipt. It checks the raw edge records and early unavailable
conditions without parsing again. The receipt SHA256 is
`090dc18f6aead522221fdba83180ab8c2d4eb25de19837ae1cb7951d99ecae1d`.
An earlier local diagnostic attempt stopped at its fixed-count precondition
because of a jq expression-binding error; its files and failure note remain
in `posthoc-saved-view-v1`. The correction changed only that diagnostic.

The separate independent audit is retained in
`independent-consistency-audit-v1/`; its receipt SHA256 is
`1f68ea1931e47f0cf367b61de6bf26a063d879c65c80db9ec4e7246c8ef602ae`.
