# Dative choices in author writing

The synthetic [dative study](dative-alternation.md) established reversible
structural edits, but its global author-distance objective preferred the same
construction for every target author. This study asks whether real writers
provide enough repeated, usable construction choices for a conditional
preference model.

The fixed source is the existing 3,889 TRAIN posts from 300 Blog Authorship
authors, spanning 3,375 author/date groups. The original, replication and
confirmation panels contain 1,224, 1,363 and 1,302 posts respectively. These
are reused training sources. No later DEV or TEST writing enters this audit.
Source passages and identifying metadata remain in ignored local artifacts.

The primary parser is the pinned transformer. It parses every complete source
post, including posts where the smaller parser finds no relevant verb or edit.
The cached small-parser annotations provide a separate sensitivity analysis.
There is no filtering by smaller-parser success, sentence extraction, parser
mixing or fitted author-space projection.

## Counted evidence

The scanner derives its verb catalog from the effective member classes that
retain both pinned `give-13.1` frames, including inherited memberships. It
records every lexical token with a registered supplied lemma, regardless of
POS or predicate eligibility. Missing lemmas and unobserved predicate heads
remain explicit. It preserves every frame alternative for relevant heads,
without selecting a lexical sense.

The audit separates these stages:

1. Registered-lemma exposure among all lexical tokens.
2. A unique complete prepositional or double-object frame at an anchored
   predicate. Incomplete, contradicted and ambiguous evidence remains separate.
3. A source proposal from the unchanged production generator.
4. An exact candidate that passes forward alignment and has a same-membership
   reverse proposal restoring the original bytes and passing reverse alignment.

Each source predicate counts once per post and parser. Repeated predicates at
different byte spans remain distinct; duplicated frame memberships do not
multiply the statistical observations. All original proposals and rejection
reasons are retained. Each candidate changes one original site, and every
unique full candidate text is parsed once per parser.

The comparison retains token occurrences, participants, dependencies,
morphology and operator evidence throughout the whole post. A changed parse
elsewhere can prevent structural eligibility. These failures are reported
without weakening the comparison.

Source annotation availability and assessment completeness are separate.
A post may retain confirmed choices while another candidate check remains
unresolved. A missing counterpart or failed comparison cannot become an
observed zero. Every source post, writing date and author remains in the
denominators, including zero-opportunity and incompletely assessed cases.

## Preference support

The protocol fixes the support calculation before corpus observation. A query
author/date block qualifies only if an observed predicate lemma has at least
four other eligible personal writing dates and at least ten other authors
with eligible observations for that lemma. The proposed modeling stage requires
at least 30 qualifying authors and 100 qualifying date blocks. This is a
feasibility threshold, not a power calculation. In fact, the first condition
implies at least five qualifying dates per qualifying author, making the
100-block threshold redundant once 30 authors qualify; both declared checks
remain unchanged.

Only the final reciprocal tier can establish support for the intended edit
scope. The broader tiers describe how much evidence is lost at each step.
The support function counts recorded confirmed choices; it does not prove
complete opportunity recall or semantic equivalence.

If support is adequate, the fixed next comparison is pooled versus
lemma-conditioned versus author-and-lemma construction probabilities. Events
are averaged within posts, posts within writing dates, and dates within
authors. Population estimates exclude the target author and use a Jeffreys
prior. Personal estimates exclude the entire query date and shrink toward the
lemma population rate with four effective reference dates. No model fitting,
parameter search, prediction evaluation, LLM calls or detector calls occurs in
this support audit.

## Completed corpus results

Both parsers completed all 3,889 posts without source annotation, candidate
annotation or comparison execution errors. The primary transformer found
169 complete construction occurrences, but the unchanged generator proposed
only one edit. Neither parser produced an edit that passed the reciprocal
structural checks.

| Evidence | Transformer, primary | Small parser, sensitivity |
| --- | ---: | ---: |
| Complete construction occurrences | 169 | 166 |
| Double-object occurrences | 128 | 129 |
| Prepositional occurrences | 41 | 37 |
| Posts with complete constructions | 159 | 154 |
| Writing dates with complete constructions | 157 | 152 |
| Authors with complete constructions | 118 | 116 |
| Source proposals | 1 | 1 |
| Reciprocally confirmed edits | 0 | 0 |

The sole candidate assessment remained incomplete under each parser. Each
reciprocal support report retains that missing result separately from the
3,888 fully assessed posts with no confirmed edits. The corresponding choice
rate is unavailable, not zero.

No tier met the fixed author/date support threshold, including the broader
complete-construction tier. No preference model was fitted or evaluated.
These counts do not establish that the remaining constructions cannot be
edited: they describe coverage of this particular generator and comparison.

The same simple-noun gate currently applies to all three arguments, including
the subject that the edit leaves untouched. It rejected 137 complete
transformer constructions at a pronoun subject. Among the remaining first
failures, eight were pronoun objects, 14 concerned quantifier, possessive or
nonlexical evidence, and nine concerned unsupported modifiers. The small parser
showed the same pattern, with 132 first failures at pronoun subjects. These are
first-failure counts; removing one restriction does not establish that later
checks would pass.

The transformer candidate preserved the expected opposite-frame participant
roles and had an exact restoring reverse proposal. Its full-post analyses
nevertheless differed in both directions: dependency attachments, morphology,
predicate observations and sentence segmentation changed. Some predicate
comparisons remained unresolved. The edit therefore failed the fixed
reciprocal criterion. This distinguishes proposal coverage from instability
in the parser-based comparison.

The next experiment separates unchanged subject evidence from the restrictions
on moved objects. A shared phrase description will support role-specific
policies for personal pronouns and simple possessive determiners. Fresh
synthetic cases will precede a descriptive replay of the cached corpus.
The existing frame, byte occurrence, inverse and alignment checks remain fixed
for that comparison. More historical posts are available if the resulting
eligible choices still lack repeated author/date support.

The implementation passed 754 tests across 42 test suites, including the
installed spaCy integration, plus formatting, Clippy and the release build.
Corpus sources, annotations, candidate texts and individual author reports
remain in ignored local storage. This experiment does not assess readability,
meaning preservation or detector performance.

## Reproduction

With the existing ignored corpus and parser artifacts present, run from the
repository root:

```sh
cargo build --release --manifest-path grammar/Cargo.toml \
  -p grammar-eval --bin slopninja-dative-corpus
grammar/target/release/slopninja-dative-corpus freeze \
  --repo "$PWD" --out /absolute/path/to/new-protocol-directory
grammar/target/release/slopninja-dative-corpus run \
  --repo "$PWD" \
  --protocol /absolute/path/to/new-protocol-directory/protocol.json \
  --expected-protocol-sha256 SHA256_PRINTED_BY_FREEZE \
  --out /absolute/path/to/new-run-directory
```

Use fresh output directories. The protocol binds the executable, source files,
resource registry, parser assets, training metadata and support policy before
the run. The source metadata binds every cached annotation, source hash and
author/date group. Full transformer source annotations, candidate annotations,
observations, reverse comparisons, support inputs and errors remain local.
