# Conditional negative-form preferences

This study asks whether an author's negative contraction choices on an excluded
writing date are more predictable from that author's other dates than from
other authors. It starts with the two unchanged negative contraction/expansion
rules. It does not infer a general preference from how often an author uses a
particular auxiliary.

The input is the existing 3,889 TRAIN posts from 300 authors in the original,
replication and confirmation panels. No fresh authors are allocated. These
authors already informed development, and the excluded-date calculation may
use dates later than the query. It is a retrospective conditional prediction
study, not prospective prediction or fresh author confirmation.

## Observations and conditioning

The extractor calls the original negative rule descriptors directly, without a
search-candidate cap. Each licensed occurrence records its observed form,
opposite form, exact reversible patch, clause anchor and provenance.
The response is `y=1` for contracted and `y=0` for expanded. The conditioning
key is the compact JSON pair `[inflected auxiliary, clause type]`.

Inflected forms stay distinct: *does* and *did* do not collapse to *do*.
Declaratives and do-imperatives have separate keys. The frozen rule recognizes
*cannot* as the expanded counterpart of *can't*; spaced *can not* remains
outside its licensed choices. Eligibility is the existing rule's decision,
not a new judgment that every negative expression is interchangeable.
Corpus counterparts are not reparsed or passed through the claim guard during
observation extraction. Eligibility therefore describes proposals from the
source parse; it does not prove that both surface forms receive identical
parser treatment or that every counterpart preserves the intended claim.

Every input post and author remains in coverage results, including those with
no licensed occurrences. Counts distinguish accepted directions, adjacent
auxiliary/negation pairs without a proposal, and unmatched negations. Rejection
records say that the frozen rule produced no proposal; they do not invent a
more specific syntactic or semantic reason.

## Two fixed estimators

For author `a`, context `f` and date `d`, let `q[a,f,d]` be the mean contracted
fraction. First average occurrences within each post containing that context;
then average those posts equally within the date. A date therefore contributes
one effective observation, regardless of the number of repeated occurrences.
Posts with no occurrence of that context contribute no artificial zero.

For each other author with context `f`, average their `q[b,f,d]` values equally
over dates to obtain `r[b,f]`. The population prediction for target author `a`
is:

```text
p_population[a,f] = (sum(r[b,f] for supported b != a) + 0.5)
                   / (number of supported other authors + 1)
```

This is a Beta(0.5, 0.5) prior with one effective observation per supported
other author. The target author is excluded completely. If no other author
has that context, the prediction is exactly 0.5. Counts from other auxiliary
forms are not substituted for missing context support.

For a query on date `d0`, remove every post from that target author on `d0`
before computing their context history. With `D` remaining dates containing
`f`, the personalized prediction is:

```text
p_author[a,f,d0] = (sum(q[a,f,d] for retained d) + 4*p_population[a,f])
                  / (D + 4)
```

The prior strength is fixed at four effective dates. There is no grid search,
author-wide rate, log-odds correction or model selection in this stage. With
`D=0`, personalization returns the population prediction exactly. Both
probabilities remain strictly between zero and one, so log loss needs no
outcome-dependent clipping.

Population and personalized predictions condition on the same context. This
controls the measured auxiliary/clause-type mixture; it does not isolate all
effects of subject matter, genre, emphasis, quoted language or circumstance.
Different within-context situations may still explain an apparent preference.

## Evaluation and coverage

The primary comparison is population Brier loss minus personalized Brier loss;
positive values favor personalization. For an occurrence, Brier loss is
`(p-y)^2`. The secondary comparison uses the same direction for binary log loss,
`-y*ln(p) - (1-y)*ln(1-p)`.

Average occurrence losses within posts, contributing posts within dates,
contributing dates within authors, then authors equally. These evaluation
weights apply to all licensed opportunities in a post. They differ from the
context-specific means used to construct the estimator. Report each panel
and the pooled result on the identical paired support.

Posts and authors without any licensed opportunity have null losses and
explicit zero support. They remain in the result catalog and coverage
denominators; they do not receive fabricated correct predictions or losses.
Sparse queries with no remaining author/context history are retained, with
their exact population fallback and zero personalization difference.

Report raw event, post and date support separately from effective weights,
including population support, author/context support after date exclusion,
and the proportion of queries using fallback. Fixed ten-bin calibration
summaries are descriptive: bins are `[i/10,(i+1)/10)`, with the final bin
including one, using the same evaluation weights.

Paired intervals use 2,000 replicates that resample scorable authors within each
fixed panel, retaining the panel's original scorable-author count and saved
excluded-date predictions. An author is scorable if there is at least one
licensed opportunity; this rule does not depend on prediction performance.
Authors with no defined loss remain explicit null rows in coverage. The
intervals are conditional on scorable authors and descriptive on reused data;
they do not account for earlier feature development or refit the population
estimator. No adoption or preference threshold is chosen from the results.

## From a predicted preference to an exact edit

The [six synthetic application fixtures](../grammar/fixtures/edit-preferences-v1/manifest.json)
cover both expanded and contracted sources for *does*, *could* and *can*.
Their exact sentences, patches, extraction expectations and guard expectations
were fixed before reading corpus preference counts or application outcomes.
They contain no private examples or invented human preference profiles.

For this separate demonstration, construct each of the 300 author profiles from
all of its TRAIN dates, with the same estimator and population exclusion. There
is no excluded query date for a new synthetic sentence. Keep every author's
context support and population-fallback status beside the recommendation.

The existing source-aligned claim guard must first observe preservation.
Compare the predicted probabilities of the two forms and recommend the
counterpart only when its probability is strictly greater. Exact ties retain
the source. A changed or unresolved claim remains ineligible regardless of
preference. Extraction and guard expectations are checked separately, and
every mismatch remains in the output.

The fixtures require retention of the same predicate, actor, negation and exact
modal or finite-auxiliary qualification. An author preference cannot authorize
changing *could* to *must*, dropping *not*, or moving polarity to another claim.
Register, negation emphasis, readability and tonal intent still require review.
All recommendations retain `semantic_equivalence_certified: false`.

Six cases times 300 profiles produce correlated demonstrations, not 1,800
independent successful edits. Their purpose is to connect a measured conditional
preference to a deterministic licensed choice while retaining no-change and
preservation checks. Neither detector performance nor human authorship is
inferred.

## Results on the reused TRAIN dates

The frozen run retained all 3,889 posts and 300 authors. It found 8,349 licensed
opportunities: 6,874 contracted and 1,475 expanded. These occurred in 2,704 posts
and 2,445 author/date groups, out of 3,375 total author/date groups. The paired
loss denominator is 293 scorable authors; the other seven retain null losses.

The raw parse contained 104,587 auxiliary tokens and 21,564 tokens labeled as
negation. There were 16,538 adjacent auxiliary/negation pairs, of which 8,189
received no proposal. Another coverage count records 13,215 negation tokens
without an accepted opportunity. These counts describe the frozen extractor's
coverage, not grammatical judgments about all rejected expressions.

The source-rule classifier labeled 8,119 opportunities declarative and 230
`do_imperative`. It emitted 19 observed conditioning keys. Here `do_imperative`
names the frozen rule's do-lemma branch without an overt subject, not a verified
imperative reading. The rule requires the parsed head to be `ROOT/VERB/VB` but
allows inflected *did* and *does* as well as *do*. The actual counts are:

| Auxiliary in `do_imperative` context | Contracted | Expanded | Total |
|---|---:|---:|---:|
| do | 147 | 17 | 164 |
| did | 36 | 2 | 38 |
| does | 23 | 5 | 28 |

The 66 *did*/*does* occurrences expose a limitation of that classifier label.
Refining the classification belongs in a later version with separately retained
results. No context label or occurrence was removed after seeing these counts.

Lower values indicate better predictions in the following table. Every
comparison uses the same scorable support and the fixed weighting described
above. Log loss uses natural logarithms.

| Panel | Scorable authors | Opportunities | Population Brier | Personal Brier | Population log loss | Personal log loss |
|---|---:|---:|---:|---:|---:|---:|
| Original | 99/100 | 2,712 | 0.177725 | 0.156888 | 0.539805 | 0.480609 |
| Replication | 97/100 | 2,950 | 0.147006 | 0.136940 | 0.462774 | 0.432635 |
| Confirmation | 97/100 | 2,687 | 0.154226 | 0.137409 | 0.481829 | 0.436819 |
| Pooled | 293/300 | 8,349 | 0.159776 | 0.143835 | 0.495110 | 0.450230 |

| Paired population-minus-personal loss | Improvement | Descriptive 95% interval |
|---|---:|---:|
| Brier, primary | 0.015940 | [0.010686, 0.022059] |
| Log loss, secondary | 0.044880 | [0.030718, 0.061338] |

Personalization fell back exactly to the population prediction for 1,480 of
8,349 opportunities, or 17.73%, because no other date from that author contained
the context. Those queries remained in the paired evaluation. Every query had
some other-author population support; none used the population's prior alone.

The lower losses support the usefulness of author/context history for this
retrospective prediction task. They do not establish future-date performance
or performance on new authors. Both estimators still overpredict contraction
on average under the evaluation weights: observed contraction is 75.603%,
versus mean predictions of 79.633% for the population and 79.765% for the personal
estimator. The local ten-bin calibration report retains the raw sample counts
and weight mass; sparse bins should not be treated as precise calibration curves.

## Results of the six synthetic applications

All six extraction expectations matched, with no unavailable rule. Four of six
guard expectations matched. The table shows the actual choice after applying
the frozen guard and then the personal probability, across all 300 profiles.

| Source form | Proposed counterpart | Guard outcome | Choose counterpart | Retain source |
|---|---|---|---:|---:|
| does not | doesn't | observed preserved | 297 | 3 |
| doesn't | does not | observed preserved | 3 | 297 |
| could not | couldn't | observed preserved | 300 | 0 |
| couldn't | could not | observed preserved | 0 | 300 |
| cannot | can't | changed, expectation mismatch | 0 | 300 |
| can't | cannot | changed, expectation mismatch | 0 | 300 |

For *does*, three authors' supported context histories reverse the population's
contraction preference. The same three profiles retain *does not* when it is
the source and expand *doesn't* when that is the source. For the other 297
profiles, the preferred form is *doesn't*. All 300 profiles prefer *couldn't*
in its declared context. There are no exact probability ties in these fixtures.
Full-profile population fallback still applies to 101 authors for *does*, 140
for *could*, and 60 for *can*; those recommendations should not be described as
evidence of an observed personal choice.

Both *can* fixtures were expected to preserve the observed claim, but the
unchanged guard returned `changed`. The parser attached `VerbForm=Fin` to the
auxiliary in *cannot* and omitted that morphology from the contracted *ca*
token in *can't*, while retaining the finite `MD` tag. The guard reported
`finite_evidence_changed` and `morphology_changed` in both directions. This is
an observed parser/guard discrepancy, not proof that the sentences have different
meanings. Both directions retained the source for every profile. Their underlying
probabilities split 297 toward contraction and three toward expansion; none
overrode the guard.

Across the six cases, 600 decisions select a counterpart and 1,200 retain the
source. These are correlated applications of 300 full TRAIN profiles to six
fixed sentences. They are not 600 independently validated improvements, and
they make no judgment about fidelity, readability, emphasis, tonal intent,
detector scores or human authorship. The frozen guard, parser, observations
and prediction rules were not changed in response to the mismatches.

The run completed once in 9.82 seconds after the release build. No optimizer,
hyperparameter search, new author allocation, neural-network fit, LLM call or
detector call was used; the context probabilities were estimated analytically
from the fixed data. Local observations, profiles and prediction rows remain
ignored under `data/author-corpora/blog-authorship-2004/edit-preferences-v1/`.
The frozen protocol SHA256 is
`2f3d8c3774a70da95631ef52ce5b81171d61f03a3f48e0064261b6aeaa2039a4`;
the report SHA256 is
`3568f0ed736c5a534910f64c020f64f5072e88ee90ed5bd7888970e130d948c9`;
the complete evaluation SHA256 is
`731f01e74e110ce4f5b9cd5f44dfef928b6f155aa4d4eab30e4a82aa0d1deb0c`.

## Implementation and retained profiles

The Rust modules separate
[choice extraction](../grammar/crates/grammar-eval/src/edit_choice_observation.rs),
[preference estimation](../grammar/crates/grammar-eval/src/edit_preference_model.rs)
and the [study runner](../grammar/crates/grammar-eval/src/conditional_edit_preferences.rs).
`PreferenceModel::new` consumes post metadata and observed binary choices.
`predict(author, excluded_date, conditioning_key)` returns both probabilities,
effective and raw support, and explicit fallback flags. It needs no parser or
LLM once observations are available.

The local `profiles.json` artifact contains all 300 full-author profiles in the
same observed context dictionary, with support beside every probability and
log-odds value. Unsupported personal contexts retain population fallback. These
coordinates describe available grammatical choices; the study does not define
a distance that certifies semantic similarity or edit quality. `model-input.json`
retains the exact observations needed to reconstruct the profiles.

With the pinned local inputs present, reproduce into a new directory:

```sh
cargo run --release --manifest-path grammar/Cargo.toml -p grammar-eval \
  --bin slopninja-edit-preferences -- \
  --protocol data/author-corpora/blog-authorship-2004/edit-preferences-v1/protocol.json \
  --expected-protocol-sha256 2f3d8c3774a70da95631ef52ce5b81171d61f03a3f48e0064261b6aeaa2039a4 \
  --repo . \
  --out data/author-corpora/blog-authorship-2004/edit-preferences-v1/run-new
```

Two separate Rust audits reproduced the extraction and numerical results.
The extraction audit checked every bound annotation and all 8,349 original-rule
opportunities, including counterpart bytes, reverse patches and rejected-event
accounting. The numerical audit checked all held-date predictions and balanced
aggregates, bootstrap replicates, calibration bins, 5,700 full-profile context
predictions and 1,800 synthetic decisions. Its largest numerical difference was
6.67e-16. It did not rerun the claim guard or supply human fidelity labels.

The local audit records preserve source hashes and the timing of outcome access.
Aggregate result messages reached the numerical reviewer before its final audit
source freeze; its separate arithmetic implementation was already written, and
no estimator or evaluation arithmetic changed in response. This was a numerical
consistency audit rather than a blinded replication.

All 339 workspace test invocations across 32 suites passed, including 15
installed-parser invocations. Formatting and all-target Clippy also passed.
Shared module tests count once per executable invocation. Executed source files
and earlier experiment snapshots remain unchanged.
