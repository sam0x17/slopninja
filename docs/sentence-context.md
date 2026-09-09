# Sentence context and structural edit checks

The [identical-text control](parser-repeat.md) found no annotation differences
across repeated inputs, fresh interpreters or the tested order change. This
study separates two possible reasons that local checks could improve: excluding
the rest of the post from the comparison, and changing the annotations by
parsing the sentence in isolation.

Every one of the preceding transformer's 60 dative proposals remains in the
study. These come from 57 training posts. The original whole-post result stays
attached to each case, including the two passing comparisons. No successful
subset is selected, and no additional authors or later posts are introduced.

The primary full-source annotation defines the sentence containing the
predicate and edit. The source slice retains its exact bytes. Applying the
rebased original patch produces the candidate slice, which must match the
corresponding full-candidate substring. The inverse patch must restore the
source slice exactly. The predicate, argument spans, untouched prefix and suffix,
and byte correspondence are checked before parsing.

A changed candidate sentence boundary does not remove the case. The projected
candidate string is still parsed as defined by the source sentence and patch.
The report records separately whether that span is one complete sentence in the
saved candidate analysis. Cases that cannot be prepared remain unavailable in
the original 60-case denominator.

Both parsers receive the same primary-defined strings. Unique exact strings are
parsed once per parser in SHA256 order; all source/candidate case origins remain
attached when two cases share a string. There is one unchanged batch call per
parser. No extra candidates from regenerated proposal reports are parsed.

## Three views of each edit

| View | Text included | Annotation source |
| --- | --- | --- |
| Original primary comparison | Whole post | Saved preceding experiment |
| Projected comparison | Exact source sentence and patched counterpart | Existing full-post annotations, rebased |
| Isolated comparison | The same sentence strings | Fresh parses under the same pinned parser |

A projection is available only for one exact saved sentence span with complete
token coverage and internal dependency heads. Rebasing preserves every token
annotation and edge. It changes byte offsets and positional indices only.
The code never invents a root, repairs an attachment, merges sentences or trims
text to obtain a usable projection. The smaller parser may disagree with the
primary sentence boundaries; such projections are explicitly unavailable.

The projected and isolated views run the same structural evaluator. Each view
regenerates proposals using the unchanged role-aware argument policy. A forward
match must have the same predicate occurrence, resource class and membership
ordinal, direction and frame pair, exact expected counterpart, and rebased
forward and inverse patches. A reverse match must restore those same
occurrences and membership. Zero or multiple matches remain explicit; the code
does not select the first match.

Both directions then use the unchanged dative comparison, including participant
bindings, morphology, dependencies, operator observations and exact inverse
movement. Unresolved findings remain incomplete. A missing supported proposal
is separate from a parser or execution error. All matching evidence and proposal
reports are retained.

A fourth, same-text comparison checks each closed full-post projection against
its isolated annotation using the [repeat observer](parser-repeat.md). It
compares exact validated documents, token occurrences, morphology, anchored
dependency heads, sentence segmentation and the unchanged word/grammar feature
families. These comparisons measure annotation changes associated with
isolation; they do not compare the source with its edited counterpart.

## Interpretation and reproduction

Improvement in the projected view alone can result from excluding other
sentences from the check. Differences between projected and isolated views
involve changed annotations on identical sentence strings. Isolation changes
surrounding language, available antecedents and encoded positions together.
Neither comparison isolates a transformer-window mechanism, and configured
window sizes alone do not measure actual window exposure.

The original whole-post guard remains unchanged. A local result cannot override
its rejection or establish equivalent meaning, emphasis, reference or
readability. The existing fused-punctuation and absent-case limitations remain
relevant. This study fits no author model and makes no LLM or detector calls.

The Rust executable
[`slopninja-sentence-context`](../grammar/crates/grammar-eval/src/bin/slopninja-sentence-context.rs)
provides `freeze` and `run` commands; the latter requires the expected protocol
SHA256. The protocol binds every case, exact parse input and origin, predecessor,
source file, executable, parser configuration and resource manifest before
annotation. Execution retains parser errors without replacement requests.

Corpus text, slices and identifying reports stay under ignored
`data/author-corpora/blog-authorship-2004/sentence-context-v1/`. Public source
consists of the
[projection module](../grammar/crates/grammar-eval/src/sentence_context_projection.rs),
[structural comparison](../grammar/crates/grammar-eval/src/sentence_context_comparison.rs)
and [experiment runner](../grammar/crates/grammar-eval/src/sentence_context_study.rs).

## Completed results

All 60 cases were prepared, yielding 120 distinct slice texts per parser. Both
parsers completed all requests without annotation or process errors. There
were 240 new annotations in total.

| Measurement, out of 60 requested cases | Small parser | Transformer, primary |
| --- | ---: | ---: |
| Both full-post sentence projections available | 27 | 59 |
| Projected view: exact forward and reverse comparisons executed | 19 | 55 |
| Projected view: reciprocal structural passes | 13 | 32 |
| Isolated view: exact forward and reverse comparisons executed | 45 | 52 |
| Isolated view: reciprocal structural passes | 24 | 32 |

The one unavailable primary candidate projection had changed sentence
boundaries; its source-defined candidate string was still parsed in isolation.
For the smaller parser, 33 primary-defined source spans and 33 candidate spans
were not exact full sentences in its saved annotations. Those cases remain in
the denominator. This is sensitivity analysis on primary-defined units, not a
comparison on each parser's independently selected sentences.

The original primary whole-post gate passed two cases. Both still passed in
the projected and isolated views. The rise from two to 32 projected passes
occurred without reparsing: it follows from checking the saved sentence
annotations while excluding the rest of the post. These additional passes
have not been validated as equivalent or readable rewrites.
Of the 30 newly passing projected cases, 29 had every failed whole-post finding
anchored outside the edited sentence. The remaining case also had 40 local
numeric sentence-index findings; projection rebased both sentence indices to
zero. This reinforces the need to distinguish position from sentence membership.

Reparsing in isolation also produced 32 primary passes, with 28 shared between
the projected and isolated views. Four projected passes were lost and four
isolated passes were gained, including the case without a candidate projection.
There is no net pass-count gain from isolated parsing on this fixed set.

The same-text comparison found changed annotations in 30 of 60 primary source
projections and 30 of 59 primary candidate projections. The smaller parser
changed four of 27 source projections and two of 27 candidate projections.
Exact source/candidate slice bytes were unchanged within these comparisons.
These are per-case counts; both sides of an edit can change. They show context
sensitivity while leaving the particular transformer mechanism unresolved.

Twelve projected cases and eight isolated cases failed only because one parse
supplied `Case=Acc` on the moved Recipient pronoun head while the other omitted
case. None involved conflicting supplied case or an unchanged Agent. These
cases still fail the frozen comparison. Missing evidence must be distinguished from a
contradiction without discarding supplied features or assuming that every
missing value is compatible.

Word occurrence counts stayed equal in every comparable same-text pair.
The `word_bigram` family changed in three source and two candidate comparisons:
its current definition stops at parsed sentence boundaries. It is therefore
partly conditioned on the grammatical analysis. A parser-independent alternative
would require a separate feature version and matched evaluation.

The next work is a fresh synthetic evaluation of explicit evidence contracts
for case features, punctuation-bearing tokens and sentence correspondence.
Numeric sentence-index shifts need to remain distinct from changed membership
or boundaries. Another window experiment is lower priority after the lack of
net benefit here. Author-preference fitting also remains unsupported: the raw
dative inventory already lacked repeated author/date observations before the
strict preservation checks, and additional local passes cannot create them.

The required workspace checks passed: 949 tests in 45 suites, installed spaCy
integration, formatting, Clippy and the release build. All 45 bound source files
remained unchanged during execution, and all 408 run artifact hashes passed
verification.
