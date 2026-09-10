# Role-aware argument phrases

The [corpus audit](dative-corpus.md) found 169 complete dative constructions
under the transformer, but the generator rejected 137 at an unchanged pronoun
subject. Applying the same noun-phrase restrictions to every argument excluded
ordinary personal writing before the moved objects were considered.

The new policy produced 60 proposals on those same posts with the transformer,
up from one. Only two passed the unchanged reciprocal checks. The smaller
parser produced 63 proposals and passed 32. Broader phrase coverage therefore
did not resolve the full-post comparison problem or establish enough repeated
choices to fit author preferences.

The new optional argument policy separates phrase description from eligibility
for movement. A phrase description retains exact token occurrences, subtree
membership, morphology, internal dependencies and reference features. The
policy assesses each argument by its role:

| Argument | Structural test scope |
| --- | --- |
| Unchanged Agent | A uniquely bound nominal subject, including pronouns. Preserve the entire subtree outside the edit; apply no moved-object modifier gate. |
| Moved Theme | A contiguous simple nominal phrase, optionally with an attached my/our/your determiner. |
| Moved Recipient | The same nominal grammar, or a singleton personal oblique pronoun with compatible supplied case evidence. |
| Deferred moved material | Theme pronouns, reflexives, reciprocals, strong quantifiers, third-person or nominal possessors, and unsupported modifiers. |

These assessments authorize structural testing only. Matching person features
does not resolve a pronoun's referent, speaker, addressee or ownership. Exact
word movement does not establish equivalent emphasis, discourse effect,
grammaticality or meaning.

The default API retains the original policy and serialized outputs. The new
policy has separate proposal and rule identities; its reports retain argument
evidence for accepted and rejected complete frames. Both policies use the same
pinned VerbNet frames, participant bindings and word/grammar representations.

The comparison still maps every token occurrence, checks morphology and
dependencies, verifies opposite-frame participants, and requires an actual
reverse proposal that restores the source bytes. Full-post parser instability
can therefore prevent a candidate from passing, even when its local role
bindings match. That limitation remains measurable in this experiment.

## Fixed experiment

Before parsing, the fixture builder fixed 64 new synthetic sources: 24
reciprocal positive pairs and 16 exclusions. The positive pairs cover seven
personal subject forms, seven oblique Recipient forms, six possessive
combinations, two combinations of supported features and two ordinary nominal
baselines. The exclusions cover moved Theme pronouns, reflexives and
reciprocals, quantifiers, possessors and nominative-only Recipient forms.

Each positive fixture records exact role anchors, its intended counterpart,
forward and inverse patches, and the production word-count change: one `to`.
The texts do not duplicate prior fixtures or pinned VerbNet examples. These
expectations concern proposals under the intended structural analysis; parser
failures remain results and do not trigger fixture edits.

The primary transformer and sensitivity small parser process every fixture
and every actual candidate. Both policies' source proposals are retained.
The new policy then runs on all 3,889 existing TRAIN posts using the previous
whole-post annotations. Only new candidate texts require parsing. This is a
descriptive replay of reused training sources. The support thresholds,
author/date denominators and missingness treatment remain unchanged; no
preference fitting or detector calls occur.

The `slop_ninja-dative-arguments` executable provides `freeze` and `run`
commands. The protocol binds the new sources, changed predecessor sources,
fixture manifest, parser/resource assets and every reused transformer
annotation before execution. Corpus passages and identifying artifacts remain
under ignored `data/author-corpora/`.

## Fresh fixture results

Both parsers completed 65 unique annotations without execution errors: the
64 frozen sources and one additional actual candidate. The intended reciprocal
counterparts were already among the sources.

| Result | Transformer, primary | Small parser, sensitivity |
| --- | ---: | ---: |
| Intended sources with legacy proposals | 4/48 | 2/48 |
| Exact intended proposals under the new policy | 48/48 | 36/48 |
| Intended sources passing reciprocal checks | 44/48 | 22/48 |
| Exclusions with no proposals | 15/16 | 15/16 |

The unexpected negative was `negative-recipient-case-i`. Both parsers fused
sentence-final `I.` into one proper-noun token, so both policies proposed
`The registrar gives I. the form` from `The registrar gives the form to I.`.
The period moved into the sentence with the token. The transformer also passed
that edit in both directions. The fixture intended a nominative pronoun;
the parser instead supplied an abbreviation-like analysis. This failed
exclusion exposes both that ambiguity and missing evidence about the period's
role at the sentence boundary. Consistent parser analyses do not certify
grammaticality. The report's `expected_reciprocal_observed_preserved` field is
conditioned on matching a declared positive counterpart; its false value for
this negative case must not be read as a failed actual reciprocal comparison.
The actual comparisons are retained in the case artifact.

Four transformer positive sources, comprising the two reciprocal pairs with
Recipient forms `you` and `it`, produced their exact expected edits but failed
the reciprocal check. Each had the expected roles and exact inverse; the sole
failed condition was the morphology map. The prepositional parse supplied
`Case=Acc`, while the double-object parse omitted `Case`. The phrase policy
permits that omission for supported pronoun forms, but the alignment still
requires identical morphology maps. This is a mismatch between the two
contracts, distinct from a supplied accusative/nominative contradiction.
The frozen comparison and all four failures remain unchanged.
The synthetic results do not establish corpus coverage or author preference.

## Completed corpus replay

Both variants reused all 3,889 whole-post annotations, with no source,
candidate or comparison execution errors. Complete-frame counts remained
unchanged at 169 for the transformer and 166 for the smaller parser.

| Result | Transformer, primary | Small parser, sensitivity |
| --- | ---: | ---: |
| Source proposals | 60 | 63 |
| Posts with proposals | 57 | 61 |
| Authors with proposals | 53 | 56 |
| Proposals passing both structural checks | 2 | 32 |
| Authors with passing proposals | 2 | 31 |
| Writing dates with passing proposals | 2 | 32 |
| Incomplete candidate assessments | 33 | 9 |

These passing counts describe the frozen structural criterion, whose false
acceptance on the synthetic exclusion remains a known limitation. They do not
certify readable, grammatical or equivalent rewrites. Incomplete candidate
assessments remain distinct from confirmed comparison failures; their counts
are per proposal, not necessarily per post.

All three evidence tiers still failed the fixed support threshold under both
parsers. No author/date block qualified, and no model was fitted. The two
transformer passing choices come from two different authors, with one date
each. That cannot identify a personal construction preference.

The saved-alignment audit found preserved participant roles and an exact
same-membership inverse for 56 of the 60 transformer proposals. Nevertheless,
54 proposals failed dependency comparisons. In 51 proposals, dependency or
predicate-operator evidence differed outside the edited source sentence;
53 had some failed evidence outside that sentence. Only two failures were
exclusively missing `Case` versus supplied `Case=Acc`. Fixing that mismatch
alone would leave the dominant problem intact.

Neither of the two passing transformer proposals contained the observed fused
terminal-punctuation pattern. This narrow check does not validate their
meaning, grammar or readability.

The next measurement is identical-source whole-post parsing under the pinned
runtime. Saved source/candidate differences alone cannot separate effects of
the edit from variability when parsing the same text. That control should
precede changes to comparison scope or annotation units.

The phrase policy and comparator also need consistent
treatment of absent versus contradictory case features. The phrase
representation also needs to distinguish lexical abbreviation punctuation
from sentence-boundary punctuation. Additional historical posts can address
sample sparsity, but cannot repair either representation problem.

The required workspace checks passed: 849 tests in 43 suites, installed spaCy
integration, formatting, Clippy and the release build. An independent replay
also reproduced all 96 legacy proposal reports and 41 legacy alignment reports
from the previous synthetic study exactly, without new parsing. The scoped
alignment comparison excludes only the study wrapper's separate candidate-role
expectation field.
