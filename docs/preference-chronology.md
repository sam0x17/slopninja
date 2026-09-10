# Negative-form preferences on later writing

This study tests whether the frozen [conditional preference model](conditional-edit-preferences.md)
predicts negative contraction choices in later posts by the same writers.
The earlier comparison excluded one writing date at a time, so a prediction
could use that author's later dates. Here both estimators use only the original
TRAIN posts. The author's evaluation posts must all have strictly later
supplied composition dates.

The model, contraction rules, context labels and prior strengths stay fixed.
There is no parameter selection, new author allocation or profile update after
an evaluation post. The existing DEV and TEST splits form one combined primary
evaluation; their separate results are descriptive breakdowns.

## Data and chronology

The three named TRAIN exports establish the exact author catalog and histories.
Their hashes bind the prepared annotation audits, which retain post identities,
source groups, supplied dates, chronological split labels, source provenance
and text hashes. The metadata review opens no source text, annotation JSON,
feature counts or preference outcomes. It hashes the later annotation file
bytes without decoding them so the evaluation can consume exact bound files.

| Existing panel | TRAIN posts | Later DEV posts | Later TEST posts | Authors |
|---|---:|---:|---:|---:|
| Original | 1,224 | 168 | 188 | 100 |
| Replication | 1,363 | 167 | 205 | 100 |
| Confirmation | 1,302 | 165 | 187 | 100 |
| Total | 3,889 | 500 | 580 | 300 |

The original panel comes from `vector-evaluation-v1`; replication comes from
`vector-evaluation-neural-v1`; confirmation comes from
`vector-evaluation-spline-confirmation-v1`. These paths identify historical
preparations. The panel named *confirmation* is an existing cohort, not a new
set of authors for this study.

For every author, require:

```text
maximum TRAIN date < minimum DEV date
maximum DEV date   < minimum TEST date
```

Also require that the named TRAIN rows match their prepared metadata exactly,
that every split retains the declared author catalog, and that no source group
or author/date group crosses a split. Compare source-post identities, archive
entry and raw-post spans, exact text hashes and normalized duplicate hashes
across all 4,969 posts. These are checks against the retained provenance;
they do not independently authenticate the supplied historical dates or prove
the absence of paraphrases and quotation.

Chronology applies within each author. The fixed background corpus can contain
other authors' TRAIN posts dated after a query. This is a test of later writing
from known authors with that background corpus, not a simulation of a single
calendar-time cutoff. The authors and their later texts also participated in
earlier retrieval studies. No new-author or completely untouched-data claim
follows from this evaluation.

## Predictions and denominators

Build the two fixed estimators from the same 3,889 TRAIN posts and their licensed
choice observations as before.
The population prediction excludes the target author completely and averages
other authors' context-specific rates with the fixed Beta(0.5, 0.5) prior.
The personal prediction uses that author's TRAIN context-date rates with the
fixed prior strength of four effective dates. With no personal context history,
return the population probability exactly; with no population context support,
use the declared 0.5 prior. Do not add DEV observations before predicting TEST.

Extract evaluation choices with the unchanged source-rule descriptors, without
the search-candidate cap. Context remains the exact inflected auxiliary plus
the frozen source-rule clause label. The known `do_imperative` classification
limitation is retained. Corpus counterparts are not reparsed or claim-guarded;
the labels describe source-licensed alternatives, not certified equivalent
rewrites.

Keep all 1,080 declared later posts and all 300 authors. Posts, dates and authors
without licensed opportunities have null losses and explicit zero support.
Sparse contexts remain in the evaluation with their fallback status. A missing
or mismatched bound cache is a provenance failure, not permission to remove a
row after observing its content.

The primary comparison is the paired population-minus-personal Brier loss.
Positive values favor personalization. Average opportunities within posts,
scorable posts within dates, scorable dates within authors, then scorable
authors equally. Report natural-log loss and fixed ten-bin calibration as
secondary diagnostics. Report the three panels and both original split labels
without selecting a preferred breakdown.

Use the same fixed 2,000-replicate author bootstrap within the three panels,
conditional on scorable authors and keeping saved predictions fixed. Null
authors remain in coverage. The interval does not refit population estimates
or account for all prior development on this corpus. It supports a bounded
comparison of these predictions, not a general guarantee of future behavior.

The local metadata plan, complete post bindings and per-author chronology are
retained under ignored
`data/author-corpora/blog-authorship-2004/preference-chronology-v1/metadata-v1/`.
Public documentation contains aggregate counts only. Readability, tone,
claim preservation and detector performance require separate evaluations.

The metadata audit passed for all 300 authors and all 4,969 source posts.
All declared later annotation files were present and their bytes were hashed
without decoding the annotations. The plan SHA256 is
`55aa4ea1d156766bd5b8f00fbc8a85fbd36a031282af8a0851b621d9f196c16e`;
the metadata receipt SHA256 is
`d1c769abccea19139bf6f64c41d4d4e1ebff64bad6b419fd99f9050019493d01`;
the later-post binding SHA256 is
`1c8bfd4e8a81a730c198c82d1346028ee3f93a9155d04b82ed06febb04043682`.

## Results

The run retained all 1,080 later posts and all 300 known authors. The frozen
rules identified 2,434 opportunities in 741 posts and 677 author/date groups.
There were 970 author/date groups in total. Losses are defined for 273 authors;
the other 27 remain in the report with null losses. Personal predictions used
only the 3,889 TRAIN posts throughout, including when scoring the TEST subset
after DEV.

Both losses improved under personalization in every panel. The combined result
is the predefined primary evaluation. Lower loss is better; log loss uses
natural logarithms.

| Panel | Scorable authors | Opportunities | Population Brier | Personal Brier | Population log loss | Personal log loss |
|---|---:|---:|---:|---:|---:|---:|
| Original | 88/100 | 825 | 0.145942 | 0.126389 | 0.462455 | 0.404817 |
| Replication | 93/100 | 814 | 0.137020 | 0.125130 | 0.436433 | 0.403197 |
| Confirmation | 92/100 | 795 | 0.152243 | 0.148130 | 0.474502 | 0.463716 |
| Combined | 273/300 | 2,434 | 0.145026 | 0.133287 | 0.457650 | 0.424114 |

| Paired population-minus-personal loss | Improvement | Descriptive 95% interval |
|---|---:|---:|
| Brier, primary | 0.011739 | [0.005816, 0.018484] |
| Log loss, secondary | 0.033537 | [0.017521, 0.051033] |

The descriptive split breakdown uses exactly the same TRAIN estimates. It does
not choose a preferred split or allow an update between splits. Because the
scorable authors and date weights differ between breakdowns, the combined
result is not a simple average of the two rows.

| Later subset | Posts | Scorable authors | Opportunities | Population Brier | Personal Brier | Population log loss | Personal log loss |
|---|---:|---:|---:|---:|---:|---:|---:|
| DEV | 500 | 234/300 | 1,106 | 0.150580 | 0.133207 | 0.471529 | 0.421973 |
| TEST | 580 | 243/300 | 1,328 | 0.128936 | 0.120920 | 0.419699 | 0.395371 |

For 410 of 2,434 opportunities, or 16.84%, the author had no TRAIN history in
that context and the personal estimate equaled the population estimate exactly.
These queries remained in the paired losses. None lacked other-author TRAIN
support for the population prediction.

The raw opportunities include 2,078 contracted and 356 expanded forms. The
extractor labeled 2,389 declarative and 45 `do_imperative`, retaining the known
source-rule classification limitation. It saw 6,015 dependency-negation tokens
and 4,630 adjacent auxiliary/negation pairs; 2,196 adjacent pairs received no
proposal, and 3,581 negation tokens had no accepted opportunity. Those are
coverage counts, not proof that the rejected forms lack grammatical alternatives.

Under the balanced evaluation weights, the observed contraction rate was
78.587%. Mean predictions were 79.284% for the population and 79.640% for the
personal estimator. The fixed ten-bin report retains every bin's weight mass
and raw count, with null means for empty bins. Lower prediction loss does not
establish perfect calibration or an appropriate rewrite in a particular passage.

These results show that the fixed author/context estimates improve prediction
on later writing by these known authors. They address the earlier use of later
personal dates when predicting earlier choices. They do not remove the prior
use of these authors and texts in development, establish a global calendar-time
simulation, or validate semantic fidelity, tone, readability or detector scores.
No threshold, prior strength, context definition or rule was changed after
the results were opened.

## Reproduction

The run completed once in 2.14 seconds after the release build. It used retained
annotations, with no new parser calls, LLM calls, detector calls, optimizer
steps or query-driven profile updates. The local study directory is
`data/author-corpora/blog-authorship-2004/preference-chronology-v1/`.

The standalone runner requires the frozen protocol, its exact source/input
bindings and a fresh output directory:

```sh
cargo build --release --manifest-path grammar/Cargo.toml -p grammar-eval \
  --bin slop_ninja-preference-chronology
grammar/target/release/slop_ninja-preference-chronology \
  --protocol data/author-corpora/blog-authorship-2004/preference-chronology-v1/protocol.json \
  --expected-protocol-sha256 950ce30520e6618869bc31245e5e789772be85e4b931b55e625ae7ecdfacb6fb \
  --repo "$PWD" --out NEW_OUTPUT_DIRECTORY
```

The evaluator is
[`preference_chronology_evaluation.rs`](../grammar/crates/grammar-eval/src/preference_chronology_evaluation.rs).
It validates TRAIN dates before deriving each author's last date, rejects
overlapping IDs and earlier queries, and constructs the unchanged preference
model from TRAIN only. Tests change query labels and add later observations
while requiring identical predictions. Authors without any query posts remain
in its general API's coverage, although all 300 have query metadata in this run.

The report SHA256 is
`b77d61d62bd51caa21986e26eef7d63ee6b4c9336856a51743ee570c2439c62c`;
the full combined evaluation SHA256 is
`0ee74774405c244cbb7eccb4bb8dfa364e8c33e9bd1a5adaaef1e70b97afd9f8`.
The receipt binds the combined and separate split evaluations, query
observations and executed sources. Post identities and corpus-derived examples
remain in ignored storage.

An independent arithmetic implementation reproduced the combined and split
evaluations, including all 2,434 unique later choices, null rows, calibration
and bootstrap calculations. Its maximum numerical discrepancy was 4.44e-16.
It checked all 1,080 compact query rows and their labels against the retained
extraction traces and bound metadata. It did not rerun the unchanged extractor
or decode the cached annotation JSON. The audit receipt SHA256 is
`c849562f9e9db71c242e357416c23711bea26209a2cc7858a83047e711b59379`.
Both new study binaries passed the combined workspace checks: formatting,
408 test invocations (including 16 installed-parser invocations), and Clippy
with warnings denied. Shared module tests count once per executable target.

## A bounded next grammar preference

Independent-clause grouping is a useful next candidate: period versus semicolon,
using the existing `split_independent_semicolon` and `join_independent_sentences`
rules. This adds a structural choice beyond contraction. Before estimating
author preference, a separate, versioned eligibility audit should require both
directional proposals and unchanged aligned predicates, arguments and operators.
Report acceptance by source form and all unsupported pairs; earlier synthetic
checks already exposed a semicolon split that the parser/rule combination could
not materialize.

If that audit yields usable paired support, compare fixed population and author
preferences within declared clause-length and subject/coordination contexts on
later dates. Keep the inference separate from author-recognition distance and
from discourse or tone approval. Passive-relative reduction is a poorer first
extension: its existing rule explicitly warns about loss of tense and temporal
interpretation and does not infer an inverse expansion. This is a follow-up
proposal; no new representation, estimator or study was run here.
