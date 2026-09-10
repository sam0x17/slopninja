# Parser sensitivity of anchored lexical frames

This study measures how parser annotations affect the frozen lexical-frame
matcher. The preceding [frame study](lexical-frames.md) retained all resource
alternatives but left many intended bindings incomplete. Saved examples include
nominal analyses of intended verbs, punctuation carrying argument relations,
and complement markers attached outside the supported mapping. Those examples
motivate this diagnostic; they do not measure general parser quality.

The completed comparison found more complete target bindings with the medium
and transformer models, with regressions on some texts. On the 64 fresh texts,
the counts were 36/67 targets for small, 47/67 for medium and 46/67 for
transformer. All four configurations preserved the eight exact embedded
participant checks. The transformer used prepositional `dative` annotations
that the frozen matcher does not support, explaining its four fresh losses of
previously complete targets. Coverage depends on both parsing and the mapping.

Run the same frozen matcher and VerbNet 3.4 resource with four declared parser
configurations: the incumbent small model, the same small model in the isolated
environment, and the isolated medium and transformer models. The small-model
environment comparison is a control. Compare medium and transformer results
with the small model in that same environment. CPU execution and exact model,
package, parser-wrapper and executable hashes belong in the protocol.

All original 96 texts are an openly reused diagnostic panel. A separate panel
contains 64 fresh texts constructed before any of these new parser outputs.
Run all four configurations on both panels. The study neither selects a winner
nor modifies the matcher, resource, expectations or parser settings after
results. More accepted frames alone would not justify adopting a parser or
licensing an edit.

## Fresh fixture allocation

The public [fixture](../grammar/fixtures/lexical-frame-parser-v1/manifest.json)
uses the existing `slopninja-lexical-frame-fixtures-v1` schema. The fixed budget
is 56 confirmation rows and eight masked rows. Here, confirmation means fresh
texts: target lemmas and resource classes were used previously, so this is not
a new lemma or class holdout.

| Group | Texts | Purpose |
| --- | ---: | --- |
| Six contexts for each of eight known lemmas | 48 | Proper names, common-noun subjects, present/past tense, added sentence context, final punctuation, and repeated-token or attachment challenges |
| Embedded participant changes for `state` and `conjecture` | 8 | Four exact source/candidate relations with an unchanged matrix subject and predicate |
| All entries masked for `repair` and `conceal` | 8 | Explicit missing-coverage behavior across proper/common subjects, tense and punctuation |

The first group uses `lend`, `loan`, `jog`, `walk`, `adore`, `compile`, `state`
and `conjecture`. The preposition challenges place `to` inside a relative clause
modifying the lent object, with no declared matrix Recipient. They require the
selected three-argument frame to remain incomplete. The bare-complement
challenges omit `that` and require the selected positive `that_comp` frame to
remain incomplete. Other resource frames may still complete. These are checks
of the specified matcher contract, without a grammaticality or semantic verdict.

The repeated-predicate cases for `jog`, `walk` and `compile` assert both heads
and their separate argument occurrences, giving 67 designated targets across
64 texts. Every `resource_backed_bindings` entry also includes its
`predicate_span`, so the intended occurrence remains explicit.

The remaining explicit-use cases record the intended complete syntactic frame
and its resource-backed role anchors. Every case keeps the exact predicate
span and expected lemma. Every role check explicitly states whether it needs
complete syntax. Short noun/verb-ambiguous forms and repeated names remain in
the denominator when a parser gives an unexpected analysis. No expected label
is repaired to fit an output.

All masked cases suppress every resource membership of the designated lemma.
They require a retained predicate and unknown lookup; they make no positive
role-binding assertion. Masking simulates missing resource coverage and does
not imply the original resource lacks the lemma.

## Proposition and occurrence identity

Four controlled relations change only the embedded subject and object. The
matrix subject, matrix predicate and alphabetic-word occurrence multiset remain
identical. Two relations repeat the matrix subject's literal noun inside the
proposition. Exact byte patches identify the changed occurrences; identical
token text cannot substitute for occurrence identity.

The supplemental `linked_proposition_expectations` field has this shape:

```json
{
  "matrix_predicate_span": [0, 1],
  "proposition_span": [2, 3],
  "subject_span": [4, 5],
  "object_span": [6, 7],
  "proposition_lemma": "respect"
}
```

The spans above illustrate the field types; actual fixtures contain exact UTF-8
token-head spans. The frozen producer preserves this field without evaluating
it. A new comparator checks saved evidence for a direct matrix `ccomp`, the
proposition's own `nsubj` and direct `dobj` or `obj`, and the matrix frame's link
to that proposition when a corresponding binding is available. It must compare
token indices and spans, including repeated occurrences. Missing observations
and unavailable frame links stay separate from checked mismatches.

These supplemental checks accompany the existing matrix role/frame assertions.
They do not infer semantic types, select a sense, or establish full compatibility.
The existing compact pair signatures omit absolute occurrence indices and can
collide on repeated identical tokens. Report their changes alongside the exact
anchored checks; signature equality is not proof that participant assignments
are preserved.

## Reporting and limits

Report each panel and parser separately, preserving all case and expectation
rows. Separate predicate discovery, role assertions, expected incompleteness,
frame retention, masked lookup and supplemental proposition checks. Show
complete target coverage within the selected class as well as any-class
coverage. Several possible frames for one predicate are alternatives, not
independent samples.

For each fixed parser contrast, retain expectation transitions and raw token
evidence when either parser fails a role check. Keep parse/observation errors
and unavailable comparisons explicit. The same-model environment control must
be assessed before attributing differences to model size. Do not collapse the
64 related texts, repeated templates or multiple assertions into independent
accuracy trials.

All expectations are assistant-constructed engineering observations checked
against the pinned resource and literal source bytes. There are no independent
human semantic ratings. A supported syntax binding remains distinct from
unresolved full compatibility and intended meaning. No author corpus, fitting,
author-style scoring or automatic editing is part of this study.

The local metadata-only builder and its receipt are retained under
`data/author-corpora/blog-authorship-2004/lexical-frame-parser-v1/fixture-build-v2`.
It binds the fixed allocation, checks the pinned XML and prior slot declarations,
verifies exact anchors and patches, and rejects duplicate texts or exact reuse
of an earlier frame fixture or resource example. It makes no parser calls.
The first builder and its manifest remain in `fixture-build-v1`; the pre-output
review added the second repeated-head assertions without changing any text.

## Results

All seven new producer runs and the saved-output comparison completed on the
first attempt. There were 544 new annotations and 96 reused annotations, with
no parse or observation errors. The incumbent and isolated small environments
produced identical Documents on all 160 texts. Thus the small-model control
showed no environment difference on this panel.

| Panel | Parser | Found target heads | Complete in any class | Complete in selected root | Role assertions met |
| --- | --- | ---: | ---: | ---: | ---: |
| Reused development | Small, either environment | 96/96 | 41/96 | 40/96 | 121/166 |
| Reused development | Medium | 95/96 | 49/96 | 49/96 | 136/166 |
| Reused development | Transformer | 96/96 | 52/96 | 51/96 | 147/166 |
| Fresh texts | Small, either environment | 65/67 | 36/67 | 36/67 | 96/140 |
| Fresh texts | Medium | 66/67 | 47/67 | 47/67 | 116/140 |
| Fresh texts | Transformer | 67/67 | 46/67 | 46/67 | 109/140 |

These are coverage and fixture-contract counts, not semantic accuracy. The
target denominators include 16 masked old targets and eight masked fresh
targets, which intentionally have no resource bindings. Some designated frames
also intentionally remain incomplete. A found head can have the wrong lemma;
for example, medium retained two fresh `walks` heads but supplied `walks`
instead of the expected `walk`. Head presence and the stricter predicate
expectation are separate checks.

| Panel and comparison against isolated small | Complete-required role checks, gained / lost | Partial-allowed role checks, gained / lost | Any-class target completions, gained / lost |
| --- | ---: | ---: | ---: |
| Reused, medium | 28 / 12 | 2 / 3 | 13 / 5 |
| Reused, transformer | 30 / 12 | 8 / 0 | 16 / 5 |
| Fresh, medium | 19 / 0 | 1 / 0 | 11 / 0 |
| Fresh, transformer | 15 / 3 | 1 / 0 | 14 / 4 |

There are 132 complete-required role assertions in each panel, plus 34
partial-allowed assertions in the reused panel and eight in the fresh panel.
The fresh medium gains include one formerly missing target, and transformer
gains include two. Medium also lost one previously present but incomplete
target in the reused panel. All missing targets remain in the denominators.
All four fresh assertions that require an incomplete frame remained met in
every configuration. Every masked-lookup assertion remained met as well.

The four fresh participant-swap pairs retained equal word counts and changed
both raw argument and complete-frame signatures in every configuration.
Across all eight constituent texts, the exact matrix-to-proposition edge,
embedded subject and object occurrences, ledger links and complete matrix
frame links met their checks. This includes the cases that repeat the matrix
subject inside the proposition. The compact signatures still have the general
collision limitation described above; these exact occurrence checks provide
the additional evidence for these examples.

On the 24 reused pairs, raw argument signatures changed in every configuration.
Complete-frame signature changes were available for 11 pairs with small, 13
with medium and 16 with transformer. The remaining pairs were unavailable,
with no unchanged complete-frame comparison. Full semantic compatibility
remained unresolved for every observed frame alternative in all eight runs.

## What to change next

Both larger parsers repaired punctuation-as-object and complement-marker
attachments in some reused texts. For example, they labeled the period in
`Ada adores Ben.` as `punct` instead of small's `dobj`, allowing complete
bindings. Medium introduced the reverse punctuation error in `Ada annoys Ben.`.
On fresh texts, both larger parsers recovered the verb, subject and complement
marker in `Clara states that Daniel arrived.`. Transformer also recovered both
predicate occurrences in `The walker walks, and the walker walks.`. Subject
attachments and noun/verb readings remain unresolved in other examples.

The fresh sentence `Clara lends a booklet to Daniel.` illustrates a mapping
gap. Small and transformer attach `to` directly to `lends`, with an ADP token
and its nominal object. Small labels that edge `prep`; transformer labels it
`dative`. The unchanged matcher supports the first label only, so all three
roles that require a complete frame fail with transformer despite identical
participant anchors. In other `lend` contexts, transformer also moves `to`
from the object noun to the verb. A reduction in accepted frames therefore
does not by itself establish a worse parse.

The next bounded change is to test a typed mapping for a direct prepositional
`dative`, retaining the original annotation and requiring the same literal
preposition and nominal-object evidence as the supported `prep` form. Bare
nominal datives, noun-attached prepositions, missing objects and prepositions
inside relative clauses need separate checks. That mapping has not been added
or evaluated in this study. These results do not select a replacement parser.

## Reproduction

The Rust comparator is `slop_ninja-lexical-frame-parsers`. It reads saved,
hash-bound outputs and makes no parser calls. The producer executable and its
25 source bindings remained identical to the preceding lexical-frame study.
The new comparator protocol binds those sources, its two new files, both
fixtures, all producer protocols and its release executable.

Local records are under
`data/author-corpora/blog-authorship-2004/lexical-frame-parser-v1/`:

| Record | SHA-256 |
| --- | --- |
| `protocol.json` | `39ac89c0407c19ff7fe85c74168d848883810e6ab46a64c333518a0087562b2b` |
| `inputs-v1.json` | `76ae6361de49fb8eb654a88327c27ec469ee4a9367a1e457f26a39cea37f2297` |
| `comparison-v1/report.json` | `8063c580055c7450b1b7f7301e98a8df43059a4aa77f2ad3fe5effcd0758b359` |
| `comparison-v1/receipt.json` | `460c083d194181e7779127f6c23f1e202191fc0c7c8524d62240c589f6a20c5c` |

Workspace formatting, all 559 tests, Clippy with warnings denied, and the new
release build passed. Six new synthetic tests cover occurrence identity,
missing evidence, frame filters, partial versus complete links, target counts
and comparison transitions. The installed spaCy integrations also passed.
There were no author-corpus reads, model fitting, LLM calls, detector calls or
automatic edits in this study. Author-style matching, readability, tone and
detector performance remain unevaluated by this layer.
