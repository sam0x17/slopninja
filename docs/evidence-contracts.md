# Structural evidence contracts

The [sentence-context experiment](sentence-context.md) found three distinct
problems in edit comparison. Missing case annotations were treated as supplied
differences, numeric sentence indices stood in for sentence membership, and a
period fused into a nominal token could move into the sentence interior.

This experiment adds a separate versioned evidence report. The original
proposal generator, comparison API, annotation objects and saved reports remain
unchanged. Each new report retains the complete baseline comparison. Untouched
checks for participant identity, dependency attachment, operator evidence and
unmapped material remain part of the new decision.

## Results

The revised comparison accepted 44 of 60 primary saved sentence edits, up from
32. No previously accepted corpus case lost its pass in any of the five views.
All 476 executed baseline comparisons reproduced the saved reports exactly.
All 37 gained passes across the five views came solely from compatible
Recipient accusative case versus missing case. None depended on replacing
numeric sentence indices; other morphology and lexical differences remained
subject to comparison.

| View (60 requested cases each) | Baseline reciprocal passes | Revised reciprocal passes | Unavailable reciprocal comparisons |
| --- | ---: | ---: | ---: |
| Primary whole post | 2 | 4 | 4 |
| Primary saved sentence projection | 32 | 44 | 5 |
| Primary isolated sentence | 32 | 40 | 8 |
| Small-parser saved sentence projection | 13 | 16 | 41 |
| Small-parser isolated sentence | 24 | 36 | 15 |

Unavailable comparisons include missing sentence projections and absent source
or inverse proposals. All remain in the 60-case denominators. There were no
overlay execution errors.

The fresh synthetic cases gave a separate check of the intended behavior:

| Parser | Intended rearrangements accepted, baseline / revised (36) | Unsupported terminal movements rejected by new checks (4) | Terminal controls unavailable |
| --- | ---: | ---: | ---: |
| Transformer | 32 / 36 | 4 | 0 |
| Small | 16 / 20 | 2 | 2 |

The old comparison accepted all four transformer terminal-movement controls and
the two small-parser controls with matching proposals. The revised comparison
rejected those cases. The small parser emitted no exact proposal for the other
two controls, so they do not count as tested rejections. Among its positive
cases, six tested directions failed the reciprocal criterion and ten lacked a
matching forward proposal. Both parsers completed all 40 fresh annotations
without errors.
All six tested terminal controls changed through the terminal-suffix check;
sentence membership remained preserved. The new positive passes in each parser
were the two directions of the `you` and `it` pairs.

This improves structural filtering on the observed edits. The targeted synthetic
cases and reused corpus do not establish general rewriting quality, author-style
transfer or detector performance. The author-scoring model is unchanged.

## Missing and contradictory morphology

The morphology comparison distinguishes equal supplied values, conflicting
supplied values and missing annotations. An unlicensed missing value remains
unresolved. Conflicting values remain changed. POS, tag, punctuation/space flags
and normalized lexical evidence still matter.

One explicit inference handles the pattern observed in the preceding study.
Missing case versus supplied accusative case can be compatible only for a
singleton Recipient pronoun in the closed set me/us/you/him/her/it/them. Both
annotations must identify it as PRON/PRP, and the existing same-membership
opposite-frame role and attachment checks must bind that exact Recipient
occurrence. A pronoun spelling alone cannot authorize the inference.

The report retains both raw morphology maps and the reason for any inference.
It never inserts an invented case feature into the annotation. A supplied
non-accusative case on that supported oblique Recipient conflicts with the
role/form evidence. Unchanged subjects, unsupported forms, other missing
features and unresolved roles do not receive this exception.

## Sentence membership and terminal punctuation

Sentence correspondence follows the exact token mapping. Reordering tokens
within one sentence can preserve membership. A source sentence mapped across
multiple candidate sentences is a split; the reverse is a merge. Both remain
changes. Missing token correspondence remains unresolved.

Later sentences can retain their membership despite changed numeric indices.
Such positional shifts are recorded separately. An earlier genuine split still
fails the whole-document partition check; correcting later index-only findings
does not erase it. Empty or carrier-only sentences cannot pass through vacuous
equality. Omitted or inserted preposition carriers require explicit evidence
from the existing dative comparison.

The orthographic comparison tracks terminal marks and following closing
punctuation by exact UTF-8 character occurrences inside mapped tokens. This
includes marks fused into a word token. It checks complete suffix sequences in
both directions, retaining group/order changes and unmapped characters.

The scanner uses supplied sentence boundaries and records punctuation roles
without deciding whether a period denotes an abbreviation. If a moved period
remains terminal only because the candidate parser split the sentence there,
the sentence-membership comparison must expose that split. Ordinary internal
abbreviation punctuation is distinct from a supplied sentence-terminal mark.

## Fixed evaluation

Before parsing, the experiment freezes 40 synthetic directional cases,
comprising 20 pairs. They cover the seven supported Recipient forms, internal
abbreviations, several terminal punctuation forms, Unicode names, modal or
negative constructions, and unsupported fused-terminal movements.
Declared positive cases are structural expectations; parser abstentions and
failed checks remain results.

Both pinned parsers receive the same deduplicated exact source and target
strings. Generated alternatives outside those declared targets are not parsed.
Every matching proposal and same-membership inverse is retained, including zero
or multiple matches. Structural acceptance requires a unique matching forward
and reverse with both new comparisons observed preserved.

The saved-data replay retains all 60 primary proposal occurrences in five views:

| View | Requested case rows |
| --- | ---: |
| Primary whole post | 60 |
| Primary saved sentence projection | 60 |
| Small-parser saved sentence projection | 60 |
| Primary isolated sentence | 60 |
| Small-parser isolated sentence | 60 |
| Total | 300 |

Replay requires the new wrapper's baseline alignment to match the previously
saved alignment exactly for every executed forward and reverse. It regenerates
the exact saved proposal under the pinned resource. Missing projections or
source/reverse matches remain in the denominator. These reused cases diagnose
policy differences; they are not an unseen-author evaluation.

The Rust runner binds fixtures, parse inputs and origins, predecessor reports,
source files, executable, parser assets and the resource manifest before
execution. Per-document and process errors remain unavailable with no replacement
requests. Corpus text and identifying replay artifacts stay under ignored
data/author-corpora/blog-authorship-2004/evidence-contracts-v1/.

These checks do not resolve pronoun reference, rhetorical emphasis, readability
or semantic equivalence. The existing whole-post eligibility gate remains
unchanged, and local diagnostic passes cannot override it. No author-preference
model is fitted and no LLM or detector is called. The raw dative inventory still
lacks the repeated author/date support required by the current preference
experiment.
