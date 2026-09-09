# Selecting word and grammar models on unseen authors

The validation-selected combined model scored **53.08%** on 300 new test
authors, compared with **52.09%** for the selected words-only model. Its
**+0.99 percentage-point** gain had a stratified paired 95% interval of
**[-1.50, +3.28]**. The gain was positive in all three galleries, but the
predeclared superiority criterion was not met. The main result is inconclusive.

Validation selected the spline for all three feature subsets. Grammar alone
scored 30.56%, well below words alone on full-gallery author identification.
On the secondary similar-content pair comparison, the combined model scored
71.58% against the word model's 68.70%, a +2.88-point difference with a 95%
interval of [+0.13, +5.61]. That diagnostic is exploratory, with no adjustment
for the other secondary comparisons. It does not change the primary conclusion.

This study tests whether grammar helps a fixed model-fitting and selection
procedure identify unseen authors. It compares the validation-selected
words-plus-grammar model with the independently selected words-only model.
The earlier experiments selected checkpoints using new posts from the authors
used for fitting. Here, fitting authors, validation authors and final test
authors are separate.

The original 100-author training tensor supplies 1,224 queries. Each query
excludes its complete date from its own author's reference profile. Training
weights authors equally, then dates within an author, then posts within a date.
The original numerical scales and feature definitions stay fixed.

Four newly exported cohorts exclude every author from all four previous
exports and from one another. One cohort supplies validation authors; three
cohorts supply test authors. Each requested cohort has 100 authors, with up to
20 posts per author. Chronological date groups define training references,
development posts and test posts within each cohort. Fresh reference posts
fit only their authors' profiles. They do not fit model parameters or spline
knots.

Only the validation cohort's development posts select checkpoints. Its test
posts are unused. Final scoring uses the test posts from each of the other
three cohorts, with a separate author gallery for each. Their development
outcomes are unused. The original development outcomes also do not select
anything in this study.

| Inputs | Model | Parameters | Validation top-1 | Test top-1 | Similar-content pair accuracy |
|---|---|---:|---:|---:|---:|
| Words and bigrams | Positive linear | 3 | 51.68% | 51.76% | 68.87% |
| Words and bigrams | Spline, selected | 33 | 52.36% | 52.09% | 68.70% |
| Grammar | Positive linear | 13 | 28.01% | 30.41% | 71.79% |
| Grammar | Spline, selected | 43 | 28.65% | 30.56% | 72.87% |
| Words and grammar | Positive linear | 15 | 50.20% | 50.87% | 68.17% |
| Words and grammar | Spline, selected | 45 | 52.79% | 53.08% | 71.58% |

The final fixed-prior baselines scored 51.42% for words, 23.60% for grammar and
43.57% for their combination. The selected combined model's top-5 accuracy
was 72.72%, compared with 71.66% for the selected word model. Those comparisons
are secondary.

| Test gallery | Authors / test posts | Selected words | Selected combined | Difference |
|---|---:|---:|---:|---:|
| 1 | 100 / 201 | 53.83% | 55.00% | +1.17 points |
| 2 | 100 / 194 | 49.27% | 50.07% | +0.80 points |
| 3 | 100 / 198 | 53.17% | 54.17% | +1.00 points |

All 300 test authors and 593 test posts passed eligibility. One validation
author had only one eligible training date, so the fixed support rule removed
that author and its fourteen posts. The remaining validation data contained
99 authors, 1,362 reference posts and 178 selection posts; its 198 test posts
were unused. All 593 final test queries had an available content proxy and a
unique nearest different author under that proxy.

Each model has one positive softmax weight per included family and a learned
temperature. Spline models add thirty positive interval slopes. The first
interval slope stays at one, and the calibration starts at zero. Counts include
stored trainable scalars, including the common-logit redundancy; temperature
changes probabilities but not candidate ordering for fixed weights.

Each subset gets thirty knot locations from nested dyadic quantiles of its own
original training distances. The word model therefore learns calibration over
the word-distance range, rather than inheriting a grid dominated by grammar
distances. This compares complete fitted and selected pipelines: the subsets
differ in their number of parameters and their calibration grids as well as
their available features.

All six configurations run with seeds 0, 17 and 29 for 200 full-batch Adam
updates, at learning rate 0.03. The runner uses CPU float64, eight threads,
beta values 0.9 and 0.999, and epsilon 1e-8. Initial family weights are the
original prior renormalized within the subset. Seed zero uses that prior
exactly; the other seeds add Gaussian logit jitter with standard deviation
0.05. Initial slopes are one and initial temperature is 0.1. Log-temperature
is bounded to [-6, 2] and log-slopes to [-6, 6].

For each seed, the highest validation macro-author top-1 selects the checkpoint,
then mean reciprocal rank breaks ties, then the earliest epoch. Epoch zero
is eligible. For each feature subset, mean selected-checkpoint top-1 across
the three seeds selects an architecture, followed by mean MRR, fewer
parameters and name. All eighteen checkpoint choices and the three
architecture choices are frozen before inspecting final test performance.
Every epoch and checkpoint is retained. No best seed is selected.

The single primary comparison is the selected combined pipeline minus the
selected word pipeline in macro-author top-1 accuracy. Query metrics are
averaged across three independent fits, then within each author, then equally
across all eligible test authors. Bootstrap resampling preserves each gallery's
eligible author count. For each of 2,000 replicates it samples authors with
replacement within each gallery, then pools them with equal weight per author.
The interval uses sorted bootstrap values at zero-based indices 49 and 1949. A positive lower 95%
bound is required for the predeclared superiority conclusion.

All six configurations, fixed-architecture comparisons, top-5, MRR, grammar-only
retrieval and seed variation are secondary. The similar-content diagnostic
compares the true author against the nearest different author under the
existing content-lemma TF-IDF representation, fitted from fresh reference
posts. Every model receives the same impostor. This is a proxy for a difficult
negative, not verified topic matching or proof of topic-independent style.

Eligibility follows the previous evaluator: all fourteen feature families
must be available, every retained author must have eligible posts in all three
chronological splits and at least two training dates. Unsupported authors are
excluded uniformly from every model's gallery and query set. Exact exclusions
are frozen from metadata before opening outcomes. Authors are not replaced.

The intervals condition on the three fixed galleries, validation choices and
three fitted seeds. They do not estimate variation across arbitrary galleries
or across repeated model-selection datasets. Author accounts are not verified
identities, and topic, quotations and near duplicates can still contribute to
retrieval. This study does not evaluate edits, preservation of meaning,
readability or AI-detector performance.

All source text, profiles, fitted models and detailed outcomes remain in ignored
local `data/author-corpora/blog-authorship-2004/author-disjoint-selection-v1/`.
The four exports are `authors-disjoint-validation-v1` and
`authors-disjoint-test-{1,2,3}-v1`; their joint metadata audit is in
`disjoint-cohort-audit-v1/`. Each corresponding `vector-evaluation-disjoint-*`
directory contains the Rust-prepared annotations, geometry and distances.

Frozen protocol SHA-256:
`ef47c911536d986cb3927ef28d370d60a14229a305305e596172dab43caf3c31`.

All eighteen fits completed in 14.87 seconds of summed recorded elapsed time,
excluding corpus preparation. The selected spline checkpoints were epochs
19/20/18 for words, 125/134/160 for grammar and 99/96/99 for the combination.
All 3,618 checkpoints remain available, including optimizer state and each
epoch's validation metrics. The 54 final score files preserve every candidate
score for each model, seed and gallery.

The [runner and reproduction instructions](../grammar/crates/grammar-eval/python/author-selection.md)
describe the train and evaluate commands. All eighteen Python tests passed;
dependency checks were clean, and both the current and earlier frozen scoring
source hashes remained unchanged. A separate NumPy implementation reconstructed
all 3,618 epochs' validation summaries, the selected training losses, all 10,674
complete test rankings and all nine stratified comparisons. Maximum logit
disagreement was 7.11e-14. This study required no Rust source changes or external
model or detector calls.

Within the local experiment directory, `train-v1/selection.json` records the
frozen choices and `test-v1/report.json` records final outcomes. The two
eligibility manifests bind the metadata-only freezes; their scripts are
`freeze-eligibility.py` and `freeze-eligibility-v2.py`. The latter also binds the
selected-model digest. `independent-audit.py`, `independent-audit.json` and
`final-checks.json` preserve the verification. `figures-v1/` contains the
standalone PNG, SVG and PDF comparison, aggregate CSV and provenance hashes.

Selection SHA-256:
`6c5fbf758bc0b095b2e2801a774c5dc084e18b3e939f10377c8ca1f01f8a9d14`.
Result SHA-256:
`5148fa5b719f6f0da1a44cb6ecb1a73186fd5b4f4881f55b8014f239f953454e`.

The optional plot uses the separate environment pinned in
`grammar/crates/grammar-eval/python/requirements-plot.lock`:

```sh
python grammar/crates/grammar-eval/python/plot_author_selection.py \
  --selection data/author-corpora/blog-authorship-2004/author-disjoint-selection-v1/train-v1/selection.json \
  --report data/author-corpora/blog-authorship-2004/author-disjoint-selection-v1/test-v1/report.json \
  --out data/author-corpora/blog-authorship-2004/author-disjoint-selection-v1/figures-new
```

The benchmark now selects on unseen authors and gives future representation
changes a fixed reference. The current models still weight family-level
distances. Learning individual word and construction weights remains a separate
experiment; no such coordinate-level learning or text-edit model was added here.
