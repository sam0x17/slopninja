# Source-aligned claim checks

The [claim-guard fixtures](../grammar/fixtures/claim-guard-v1/manifest.json)
test whether an edit retains the local qualifications and content anchors of
each source claim. An aggregate style profile can lose this association: swapping
*may* and *must* between two predicates preserves the modal inventory while
changing who may do what. A source-aligned check must distinguish those claims.

The manifest contains 32 assistant-constructed cases. Sixteen reproduce the
exact source texts and selected candidates from the
[edit utility matrix](edit-utility-matrix.md); sixteen add challenges. The
expectations were fixed before reading claim-guard outputs. Readability, tone
and semantic equivalence remain unjudged.

The first execution matched 27 of 32 expectations. All twelve expected changes
were detected, including modal swaps, moved negation, tense changes and altered
subjects. Every case expected to be changed or unresolved remained ineligible
for selection. Two intended boundary edits were also blocked after the parser
changed a clause attachment. These results describe this fixed synthetic set.

## What an outcome means

| Outcome | Engineering interpretation |
| --- | --- |
| `observed_preserved` | The supported observations agree under the available source correspondence. |
| `changed` | The guard identifies a changed qualification or content anchor. |
| `unresolved` | The available correspondence or supported syntax is insufficient for a preservation observation. |

Every outcome retains `semantic_equivalence_certified: false`. Parser agreement
cannot establish truth, resolve pronoun reference, infer an author's intended
scope, or judge pragmatic force. A no-change option remains available.

Local operator checks and content checks answer separate questions. Replacing
*client* with *server* leaves the modal and retry predicate unchanged but changes
the actor. The two subject-edit fixtures therefore expect `changed` overall and
`observed_preserved` for each aligned predicate's local operator observations.
The optional `expected_local_operator_outcome` field expresses this distinction;
`null` leaves that separate expectation unspecified.

## Frozen cases

The regression set contains six previously licensed proposals, six rule
abstentions and four explicit counterfactuals. A licensed case uses its exact
previous expected candidate, including the semicolon split that the installed
parser did not propose. An abstention case uses the original text as its
candidate with an empty patch list. These six explicit no-change controls
preserve the separate rule-abstention expectations, including the quoted case.

The new challenges cover:

- Repeated predicates with different subjects, including modal and negation
  swaps between two occurrences of *leave*.
- A modal edit after the multibyte name *Zoë*, to exercise UTF-8 byte offsets.
- An independent coordination split and a negative modal contraction.
- A finite-tense change, an added predicate and a deleted predicate.
- Negation moved across coordinated claims, plus an untouched *not* token whose
  surrounding claims are replaced. Stable token bytes cannot establish its
  parent when that parent lacks reliable correspondence.
- Subject substitutions with unchanged local operators.
- Edits inside quoted text and backtick-delimited code.
- Whole-span paraphrase and clause reordering, both expected to remain
  unresolved because the patch provides no trusted claim correspondence.

The manifest declares 13 `observed_preserved`, 12 `changed` and 7 `unresolved`
expectations. These are engineering targets, not measured outcomes or human
fidelity labels. In particular, desired preservation expectations for sentence
boundaries remain fixed even if the parser or conservative scope guard abstains.
Record that mismatch instead of changing the fixture afterward.

## Exact patches and execution

Each case contains `source_text`, `candidate_text`, `edits`, `expected_outcome`,
an optional local-operator expectation, and protected review prompts. Regression
metadata binds the previous manifest hash, case ID and rule expectations.

The patch shape is the existing `grammar_core::edits::Edit`:

```json
{
  "start_byte": 5,
  "end_byte": 8,
  "expected": "may",
  "replacement": "must"
}
```

Those offsets address the original UTF-8 bytes in `Zoë may file. René may wait.`.
Validate every boundary and expected substring, reject overlapping patches,
and apply the complete patch list against the original source. The result must
equal `candidate_text` exactly. Protected source/candidate anchors must also
match their declared bytes. Minimal word substitutions and a few deliberate
whole-span replacements expose different alignment obligations.

Parse the source and candidate using the same pinned parser. Pass only their
documents and exact patches to the guard; expected outcomes, fixture IDs and
review prompts must not influence its decisions. Preserve per-predicate
correspondence, local operator changes, content findings, added/deleted claims,
unsupported cases and parser failures. Compare observations with expectations
afterward, and retain every mismatch.

A supported one-token substitution can preserve a source position even when
its surface spelling changes. It still requires a lexical and grammatical
check. Matching a lemma elsewhere in the document cannot establish alignment;
the repeated-predicate cases exercise this failure mode. Broader replacements
must remain unresolved unless an explicit supported correspondence applies.
The guard may recognize existing negative-contraction forms and exact patches
from the unchanged boundary rules. Such recognition remains subject to all
other operator and content checks.

This synthetic set measures coverage and exposes counterexamples. It provides
no corpus performance estimate and no guarantee for arbitrary rewrites. Style
distance may rank candidates only after the separate preservation checks and
review obligations are retained; a favorable score cannot override a changed
or unresolved claim.

## Completed execution

The runner parsed 100 distinct synthetic texts without a parse failure. All
32 source/candidate pairs were scored in raw form and with the same predeclared
appended context. The guard evaluated the raw pairs only; its observations do
not certify the text in arbitrary new surroundings.

| Expected outcome | Observed preserved | Changed | Unresolved |
| --- | ---: | ---: | ---: |
| Observed preserved | 11 | 2 | 0 |
| Changed | 0 | 12 | 0 |
| Unresolved | 0 | 3 | 4 |

Both subject-edit cases had the expected split result: local operators were
preserved and content was changed. All six no-change controls were preserved.
The four semantic counterfactuals from the preceding edit matrix were changed,
including the modal swap whose frozen target scores had tied exactly.

All five mismatches remain recorded:

- The semicolon split changed *reads* from `ccomp` to `ROOT`. The original rule
  did not license that parse, and the guard reported changed attachment plus
  unresolved boundary correspondence.
- The sentence join was proposed by the original rule, but reparsing changed
  *reads* from `ROOT` to `ccomp`. The supported boundary exception does not
  normalize that relationship. Both boundary cases retained their local
  operator observations while failing the content/scope check.
- Adding or deleting the *Lee must stay* predicate produced an explicit
  observed change. The fixtures had expected unresolved correspondence.
- The quoted modal edit produced an explicit *may* to *must* operator change.
  The fixture had expected unresolved literal scope. Known change takes
  precedence over unresolved evidence in the guard's overall outcome.

The four unresolved results concern code, the two broad replacements and the
unchanged *not* token surrounded by rewritten claims. They remain ineligible;
the runner does not guess their missing correspondence.

Selection used the same three frozen models and ten fixed original TRAIN author
profiles as the edit utility study. Each seed/target pair retained the source
unless the guard observed preservation and the candidate's score was strictly
higher. Exact ties and unavailable features also retain the source.

| Scoring context | Candidate selected | Source retained | Closer pairs blocked by guard |
| --- | ---: | ---: | ---: |
| Raw | 30 | 930 | 312 |
| Predeclared appended context | 33 | 927 | 287 |

Negative expansion accounted for all thirty raw selections and thirty appended
selections. The independent coordination split supplied the other three
appended selections. These are correlated measurements over thirty fixed
seed/target pairs per case, not independent successful edits or judgments of
writing quality. Every score and decision remains available for inspection.

The ignored artifacts are under
`data/author-corpora/blog-authorship-2004/claim-guard-v1/`. The frozen manifest
SHA256 is `8118a4168fa5200c0fb4256fa62d12438ed88c95b90508bbea6cfbeb9951bf7b`;
`run-v1/report.json` has SHA256
`19562e12549cf701dff511466d02f488923c4e3cd719dc91c4bb5cc8ed4a8b86`.
The independent consistency audit checked all 100 saved annotations, 218
alignment rows, 47 matched predicates and 1,920 paired decisions containing
3,840 source/candidate scores. It reconstructed local and overall outcomes and
checked all five mismatches. It did not rerun the parser or author models, or
assign semantic-equivalence labels.

With the pinned local inputs present, reproduce into a new directory:

```sh
grammar/target/release/slop_ninja-claim-guard evaluate \
  --protocol data/author-corpora/blog-authorship-2004/claim-guard-v1/protocol.json \
  --expected-protocol-sha256 9ca196751e364f9c1c5845c18da8ff7e30c41766cc27726ba07b1d9289ceceb8 \
  --repo . \
  --out data/author-corpora/blog-authorship-2004/claim-guard-v1/run-new
```

No new author allocation, model fitting, LLM call or detector request was used.

## Compare a proposed edit

The `slop_ninja-claim-guard compare` command accepts two syntax `Document` JSON
files and an array of exact `Edit` patches. Supply their SHA256 values with
`--expected-source-annotation-sha256`, `--expected-candidate-annotation-sha256`
and `--expected-edits-sha256`, alongside `--source-annotation`,
`--candidate-annotation`, `--edits` and a new `--out` path. Both annotations must
use the same parser, and applying the patches must reconstruct the candidate.
The command does not need author profiles or a scoring model.

The output contains `guard.outcome`, the separate local-operator and
content/scope outcomes, and source-aligned findings. A successful process exit
means the comparison ran; inspect the outcome before considering the candidate.
`observed_preserved` retains the stated parser-observation limits and does not
certify semantic equivalence or tone.
