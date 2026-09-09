# Learning individual word and construction weights

Learning individual coordinate weights raised author-identification accuracy
from **48.56% to 53.30%** on **300 fresh authors and 590 test posts**. The
**+4.74 percentage-point** difference had a paired 95% bootstrap interval of
**[+2.67, +6.80]**, satisfying the predeclared superiority criterion. Each of
the three test galleries improved.

| Test gallery | Authors / test posts | Frozen baseline | Learned coordinates | Difference |
|---|---:|---:|---:|---:|
| 1 | 100 / 201 | 44.33% | 48.50% | +4.17 points |
| 2 | 100 / 187 | 50.17% | 56.39% | +6.22 points |
| 3 | 100 / 202 | 51.17% | 55.00% | +3.83 points |
| All galleries | 300 / 590 | 48.56% | 53.30% | +4.74 points |

Top-5 accuracy rose from 67.93% to 72.90%; mean reciprocal rank rose from
0.5817 to 0.6223. These are secondary outcomes. Across the three inherited
seeds, baseline top-1 ranged from 48.39% to 48.72%, and coordinate top-1 from
53.17% to 53.50%. Results average seed metrics, then posts within each author,
then authors equally. Each query ranks the 100 authors in its own gallery.
The earlier studies used different test authors; this comparison evaluates
both models on the same new galleries.

This experiment tests whether individual coordinate weights improve author
retrieval beyond the previous model's fourteen feature-family distances.
The representation still combines word occurrence and grammar. Rust computes
the coordinates and distances; PyTorch fits the weights.

The previous combined spline supplies three frozen baselines, one per seed.
Their family coefficients, spline and temperature stay fixed. The new model
learns only positive multipliers on supported coordinates. Initializing every
multiplier to one must reproduce the baseline's distances, scores and rankings.
This makes the comparison specific to coordinate weighting.

## Geometry

For a categorical family, let `p_i` be a coordinate's original probability.
The exported value is `sqrt(p_i / 2)`, so the sum of squared coordinate
differences equals squared Hellinger distance. For a numerical family, divide
the original value by its frozen scale and by the square root of the family's
full original axis count. Centering cancels in a difference.

Reference profiles average probabilities within each date, then equally across
dates, before the square-root transform. Averaging transformed points would
produce a different target. Training queries exclude their whole date from
their own author's reference profile. Other candidate authors use their full
training reference profiles.

For each family, the model adds a weighted correction to its complete original
distance:

```text
adjusted_distance = original_distance
                  + sum_selected (exp(eta_i) - 1) * (query_i - target_i)^2
```

Unselected and previously unseen coordinates retain weight one. The vocabulary
cutoff never renormalizes probabilities or discards their remaining mass.
The existing spline then calibrates the adjusted family distances, using its
frozen family weights and temperature.

The implementation uses query-by-coordinate and author-by-coordinate matrices.
Weighted norms and matrix products provide the correction without allocating
a query-by-author-by-coordinate tensor. Only the true author's column needs
the training query's date-excluded target.

## Fitting and evaluation

Only the original 100 fitting authors' 1,224 training posts select coordinates.
A categorical feature must occur in at least ten fitting authors and thirty
fitting posts. Within each family, descending author support, descending post
support and ascending feature key determine selection. The fixed caps are 512
words, 512 word bigrams and 96 coordinates for each categorical grammar family.
Variable numerical axes remain, with their original scales and denominators.
The catalog retains the support counts and exclusion decision for every axis.

The resulting catalog contains **1,864 learned coordinates**: 512 words,
512 word bigrams, 828 categorical grammatical constructions and twelve
numerical measurements. The complete original catalog has 397,223 axes;
the remaining axes still contribute through the unweighted residual.

Each multiplier stays between 0.5 and 2. The penalty averages squared log
multipliers within each nonempty selected family, then averages across
families. Equal family averaging prevents a large word vocabulary from
diluting the penalty on smaller grammar families.

Four penalty strengths, 0.01, 0.1, 1 and 10, each run against all three inherited
baselines. Every fit starts with zero log multipliers and runs 200 full-batch
Adam updates at learning rate 0.03, on CPU float64 with eight threads.
The beta values are 0.9 and 0.999; epsilon is 1e-8. No new initialization noise
is added. All twelve fits must finish before selection.

The previous validation cohort supplies 178 development queries from 99 authors.
This is reused development data. For each fit, macro-author top-1 selects an
epoch, followed by mean reciprocal rank and then the earliest epoch, including
zero. Mean selected top-1 across the three seeds selects one shared penalty
strength; ties use mean reciprocal rank, then the stronger penalty.

Validation chose **lambda 10**, with epochs **52, 53 and 56** for inherited
seeds 0, 17 and 29. Mean validation top-1 was 56.11%, against the frozen
baseline's 52.79%. The complete fitting grid took 53.79 seconds of summed
recorded elapsed time, excluding corpus preparation. All twelve fits and
2,412 checkpoints completed successfully; none failed.

| Penalty strength | Selected mean validation top-1 | Selected mean validation MRR |
|---|---:|---:|
| 0.01 | 54.43% | 0.6242 |
| 0.1 | 54.56% | 0.6251 |
| 1 | 54.07% | 0.6280 |
| 10, selected | 56.11% | 0.6303 |

Three new test galleries request 100 authors each. They exclude all 800 author
IDs from the eight previous full exports, including authors excluded later
from evaluation, and exclude one another. All eleven exports have 1,100
distinct author IDs and no recorded duplicate source posts, exact texts or
normalized texts. Eligibility uses the existing parser support rules uniformly
across models. Authors are not replaced after exclusion.

Only each new gallery's test posts evaluate the frozen selection. Reference
posts fit that gallery's author profiles; development outcomes stay unused.
The primary comparison is the selected coordinate learner minus its matched
frozen combined baseline in macro-author top-1. Seed results are averaged
before calculating paired author differences. A 2,000-replicate bootstrap
resamples authors within each gallery and weights all eligible authors equally.
The predeclared superiority criterion requires the paired 95% interval's
lower bound to exceed zero.

The test galleries are fresh, but the feature design and reused validation
cohort are products of earlier development. The interval conditions on these
galleries, choices and fitted baselines. Author retrieval can also reflect
topic and quotations; it does not establish style isolation, successful edits,
readability, preservation of meaning or detector performance.

The comparison tests bounded coordinate corrections under one frozen family
scorer. It does not test joint retraining of family and coordinate weights,
interactions between coordinates, or every possible vocabulary and grammar.

## Learned coordinates

The largest multiplier changes are lexical. Averaging the three selected seeds,
word multipliers range from 0.5 to 2, with a median of 0.924; bigram multipliers
range from 0.782 to 2, with a median of 1.013. Most grammatical multipliers
remain near one. The largest grammatical decrease is singular proper-noun
morphology (`PROPN|Number=Sing`, 0.808); unmarked interjections (`INTJ|_`, 1.119)
receive the largest grammatical increase.

Words including `and`, `because` and `anyway` reach the upper bound in every
selected seed, while `are`, `was`, `were` and `love` reach the lower bound.
These are weights on mismatches between a query and its reference profile.
They are not preferences to insert or remove those words. A higher weight
makes a frequency difference more consequential when ranking authors.

These observations describe the fitted model. An ablation would be needed to
attribute the retrieval gain to word or grammar coordinates. The model also
retains topic-related features; its proper-noun downweighting does not establish
that it isolates style from content.

## Artifacts and reproduction

Rust formatting, workspace tests and all-target clippy passed. All 29 Python
tests and dependency checks passed. The zero-multiplier identity gate matched
all three baseline training losses and every validation author's metrics
exactly. Rebuilding stale development artifacts resolved the path errors
left by the repository rename.

The independent coordinate audit reconstructed all five exports from 7,465
saved feature records. It verified vocabulary support, original numerical
scales, probability averaging before transformation, whole-date exclusion
and complete original training-distance equality. A separate NumPy scorer
replayed all 2,412 validation checkpoints, the selected training objectives,
all 3,540 final candidate rankings and the 2,000 bootstrap replicates. Maximum
candidate-score disagreement was 5.68e-14. All selected results and the primary
interval matched.

The audit receipts are `independent-export-{train,validation,test-1,test-2,test-3}.json`,
`independent-training-audit.json` and `independent-test-audit.json`. The Rust
receipt also records exact executed-source snapshots and a later test-only
clippy correction; the fitted experiment retains its original binary and
source bindings. No external generation or detector calls were made.

Protocol SHA-256:
`3dab08e1f2f78ba120c41bd67071f123b7fd24ebe9688d04e4b5f26dab27536b`.

Source text, profiles, coordinate exports, checkpoints and detailed outcomes
stay in ignored `data/author-corpora/blog-authorship-2004/coordinate-weighting-v1/`.
The joint metadata audit is in `coordinate-cohort-audit-v1/`.

Selection SHA-256:
`e60c2fde07c14288efa1cacde4a3906d2ea5001ccc2e6f2b19db991e812c36c4`.
Result SHA-256:
`a94cc2a26b5ca1b066aea113c27f53e4a4d34169af6d710bb789c16f9b87de09`.

The [Rust exporter](../grammar/crates/grammar-eval/README.md) documents the
coordinate matrices, complete family distances and source bindings.
The [training and evaluation commands](../grammar/crates/grammar-eval/python/coordinate-weighting.md)
use the frozen protocol and input manifests. `train-v1/coordinate-weights.json`
contains every named multiplier for all three selected seeds. `weight-summary.json`
contains descriptive family ranges and the largest changes.

The standalone figure, aggregate CSV and provenance are in `figures-v1/`.
Regenerate the PNG, SVG and PDF using the separate pinned plotting environment:

```sh
python grammar/crates/grammar-eval/python/plot_coordinate_weights.py \
  --report data/author-corpora/blog-authorship-2004/coordinate-weighting-v1/test-v1/report.json \
  --out data/author-corpora/blog-authorship-2004/coordinate-weighting-v1/figures-new
```
