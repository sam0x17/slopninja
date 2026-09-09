# Reversible dative alternatives

The Rust grammar pipeline can generate a restricted dative alternation:
`The teacher gives the booklet to the pupil.` becomes
`The teacher gives the pupil the booklet.`, and the reverse edit restores
the original bytes. The implementation retains each participant's original
noun phrase and changes only its placement and the word `to`.

This is a structural candidate, not a proof of interchangeable meaning.
The two constructions can differ in focus and possession implications.
VerbNet's sense alternatives and selectional restrictions remain unresolved.
Neither a generated proposal nor a lower author distance permits an automatic
edit.

## Construction and alignment

`DativeAlternationV1` adds an atomic interpretation of the pinned VerbNet
`give-13.1` double-object frame. It requires an explicit preceding subject,
one direct nominal indirect object and one distinct direct nominal object,
in that order. Missing or competing arguments leave the frame unresolved.
The two earlier observer versions retain their behavior and output schemas.

The generator recognizes the pinned prepositional/double-object frame pair
within the same effective member class. This includes `give-13.1-1`, which
inherits both frames. It keeps separate membership alternatives and records
every rejected site. It accepts contiguous, disjoint noun phrases with ordinary
articles, adjectives and noun compounds; pronouns, possessives, quantifiers,
coordination and clauses inside those phrases are outside this first rule.

After reparsing a candidate, the checker maps exact token occurrences through
the byte moves. It checks the opposite frame, the same three role occurrences,
lemmas, morphology, sentence membership, dependencies and predicate/operator
evidence throughout the document. Only the declared recipient attachment
change and insertion or removal of its preposition are exempt. Repeated names
cannot substitute for one another. Missing or ambiguous evidence stays
unresolved. The general claim-preservation guard is unchanged.

## Fixed evaluation

The [fixture](../grammar/fixtures/dative-alternation-v1/manifest.json) contains
24 new source texts: six reciprocal positive pairs, 11 exclusion expectations
and one embedded-clause scope probe without an expected verdict. It uses
`give`, `lend` and `loan`. All 184 earlier exact source texts and the resource's
example sentences were excluded; these are fresh contexts, not held-out verb
classes. Production Rust edit application and word tokenization validate the
declared patches, inverse patches and sole `to` word-count change.

Before fresh parsing, the runner reproduces both earlier mapping versions on
all 736 saved parser/case combinations, requiring 1,472 whole-case JSON-value
comparisons to match. It verifies historical executed-source snapshots rather
than assuming that their live source paths have remained unchanged.

Four pinned spaCy configurations each parse the same 24 sources. Every proposal
is reparsed with its source parser; an exact reciprocal source annotation can
be reused. All candidates, abstentions and errors remain visible. The runner
does not select a favorable parser for an individual case.

## Word and grammar coordinates

Vector scoring uses the original 14-family space and the first ten
lexicographically sorted original TRAIN author profiles. It keeps their scales,
weights and date-balanced profiles fixed. It reads the fitted aggregate
artifacts, not corpus passages, and trains no new model.

The scorer uses the incumbent small parser for every text, regardless of which
parser proposed an edit. It records word counts, grammar features, sparse
coordinates, coordinate deltas and distances for combined, word-only and
grammar-only modes. Categorical coordinates use `sqrt(p / 2)` on a common
union of feature names; absent categorical names have zero probability only
when their family is observed. Numerical coordinates retain the frozen centers
and scales. This sparse representation permits unseen categorical names without
refitting the geometry or pretending they belong to the original finite axis
list.

Short sentences can lack opportunities for a grammar family. Any mode requiring
that family remains unavailable; its weights are not redistributed. A separate
diagnostic appends the same unrelated paragraph to every source and candidate.
It measures sensitivity to context and does not replace the raw-sentence result.

These distances describe movement toward a profile. They do not estimate author
preference, readability, semantic fidelity or detector performance.

## Results

The frozen run completed with 125 new annotations and no parser, proposal or
scoring errors. Both old mappings reproduced all 736 historical cases exactly
before fresh parsing. The 41 generated proposals represent 13 distinct directed
text pairs across the four parser configurations.

The new v3 interpretation changed 174 historical case bodies. Seven frozen
assertions changed from met to mismatch: they required the double-object frame
to remain incomplete under the earlier mappings. V3 now completes that frame
with the intended participant anchors on those seven annotations. It gained
seven complete target bindings and lost none. The other 3,457 verdicts are unchanged. The study
keeps the old assertions and their mismatches visible; exact replay applies to
the two older versions, not to an assertion that v3 behaves identically.

| Parser | Intended proposals | Positive edits passing structural checks | Exclusions with no proposal |
| --- | ---: | ---: | ---: |
| Small, either environment | 8/12 | 4/12 | 11/11 |
| Medium | 9/12 | 6/12 | 11/11 |
| Transformer | 12/12 | 12/12 | 11/11 |

The embedded-clause scope probe generated one additional proposal per parser,
and all four passed the structural checks. It had no predeclared expected
verdict and is excluded from the positive totals. For every positive candidate
with a complete reparsed role set, all three head and noun-phrase spans matched
the fixture. Small and medium sometimes changed the observed attachments or
lacked a complete opposite-frame interpretation after reparsing; those edits
remain changed or unresolved.

All 26 raw/appended vector comparisons had observed opportunities in every
family. The result exposes a limitation of the distance objective: on all six
positive construction pairs, all ten target authors were closer to the
prepositional form in every raw scoring mode. Combined and word-only distance
kept that direction with the appended context. Grammar-only distance changed
direction in 7 of the 60 pair/author comparisons after appending the same
unrelated paragraph. The embedded probe had four such grammar-only changes
among its ten targets and is counted separately.

Thus the combined distance offers no evidence of author-specific construction
choice on this panel: it favors the same form for everyone. Its grammar
coordinates also come from the fixed small parser, including that parser's
misanalyses; transformer alignment does not repair those coordinates. The next
useful test is to measure authors' observed choices between eligible
constructions, with opportunity counts and held-out source groups, and compare
that conditional preference against the global distance.

All 684 workspace tests, formatting, Clippy with warnings denied and the release
build passed. Tests cover exact inverse bytes, moved UTF-8 phrases, repeated
names, negation and surrounding attachments, missing observations, sparse
distance equivalence and fixture consistency. This experiment establishes a
working candidate-and-measurement path for the declared construction. It does
not establish semantic interchangeability, readability gains or detector gains.

## Reproduction

Run from the repository root with the existing ignored resource, parser and
author artifacts present:

```sh
cargo build --release --manifest-path grammar/Cargo.toml \
  -p grammar-eval --bin slopninja-dative-alternation
grammar/target/release/slopninja-dative-alternation freeze \
  --repo "$PWD" --out /absolute/path/to/new-protocol-directory
grammar/target/release/slopninja-dative-alternation run \
  --repo "$PWD" \
  --protocol /absolute/path/to/new-protocol-directory/protocol.json \
  --expected-protocol-sha256 SHA256_PRINTED_BY_FREEZE \
  --out /absolute/path/to/new-run-directory
```

Both output directories must be new. The protocol binds executable, source,
fixture, resource, parser and fitted-profile hashes before the experiment.
Local artifacts remain under ignored `data/`. No LLM or detector calls occur.
