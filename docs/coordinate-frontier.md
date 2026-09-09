# Coordinate capacity and learned feature groups

The validation-selected 3,557-coordinate model scored **52.96%** on 300 new
authors and 624 test posts, versus **51.06%** for the frozen 1,864-coordinate
incumbent. The paired difference was **+1.91 percentage points**, with a 95%
interval of **[-0.04, +3.69]**. The primary superiority criterion was not met;
the incumbent remains the reference model.

The previous coordinate study established a 4.74-point improvement over the
family-level model on different test authors. This study tests smaller and
larger trainable vocabularies and separate word and grammar updates. All 192
fits completed, and an independent reconstruction verified the selected
states, test rankings and paired intervals.

## Results

| Frozen choice | Trainable coordinates | Validation top-1 | Test top-1 | Test top-5 | Test MRR |
|---|---:|---:|---:|---:|---:|
| Family baseline | 0 | Unused for selection | 46.61% | 66.48% | 0.5692 |
| Incumbent | 1,864 | Unused for selection | 51.06% | 70.03% | 0.6035 |
| Selected word updates | 2,189 | 60.83% | 52.67% | 72.24% | 0.6202 |
| Selected grammar updates | 3,740 | 54.47% | 47.88% | 67.35% | 0.5793 |
| Selected combined updates | 3,557 | 61.49% | 52.96% | 72.79% | 0.6231 |

Every row uses all fourteen original families. Word or grammar updates specify
which individual multipliers can change; they do not remove the other families.
Results average metrics across the three inherited seeds, rather than scoring
an ensemble of their predictions. The selected combined model used penalty
0.1 and epochs 21, 23 and 23 for seeds 0, 17 and 29. Its test top-1 ranged from
52.66% to 53.41% across seeds.

| Test gallery | Authors / queries | Incumbent | Selected combined | Difference |
|---|---:|---:|---:|---:|
| 1 | 100 / 215 | 54.00% | 54.50% | +0.50 points |
| 2 | 100 / 192 | 50.83% | 55.50% | +4.67 points |
| 3 | 100 / 217 | 48.34% | 48.89% | +0.56 points |

All galleries improved, but most of the gain came from the second gallery.
Neither that pattern nor the positive pooled estimate changes the primary
interval's inconclusive result.

### Capacity and adjustable groups

Each row below uses its validation-selected penalty and checkpoints. The
test outcomes did not select a different configuration.

| Configuration | Coordinates | Penalty | Validation top-1 | Test top-1 |
|---|---:|---:|---:|---:|
| Words, 0.5 cap | 512 | 0.1 | 59.70% | 51.67% |
| Words, 1 cap | 1,024 | 10 | 60.33% | 50.89% |
| Words, 2 cap | 2,048 | 0.1 | 60.77% | 51.83% |
| Words, all supported | 2,189 | 10 | 60.83% | 52.67% |
| Grammar, 0.5 cap | 460 | 0.1 | 53.72% | 47.75% |
| Grammar, 1 cap | 840 | 0.1 | 54.04% | 47.71% |
| Grammar, 2 cap | 1,509 | 0.1 | 54.10% | 47.81% |
| Grammar, 4 cap | 2,449 | 0.1 | 54.32% | 47.92% |
| Grammar, 8 cap | 3,740 | 0.1 | 54.47% | 47.88% |
| Grammar, all supported | 5,164 | 0.1 | 54.47% | 47.92% |
| Combined, 0.5 cap | 972 | 10 | 59.20% | 50.60% |
| Combined, 1 cap | 1,864 | 0.1 | 60.72% | 51.08% |
| Combined, 2 cap, overall selection | 3,557 | 0.1 | 61.49% | 52.96% |
| Combined, 4 cap | 4,638 | 0.1 | 60.96% | 53.35% |
| Combined, 8 cap | 5,929 | 0.1 | 60.89% | 53.30% |
| Combined, all supported | 7,353 | 0.1 | 60.96% | 53.41% |

Grammar-only updates changed little as the catalog grew. Larger combined
catalogs also gave little additional test gain beyond 3,557 coordinates.
The largest model's slightly higher test score is descriptive; it does not
replace the validation-selected choice. The base-sized combined row uses new
validation selection, so its penalty and checkpoints differ from the incumbent.

### Contributions of learned corrections

| Comparison on the selected combined model | Top-1 difference | Paired 95% interval |
|---|---:|---:|
| Full model minus incumbent, primary | +1.91 points | [-0.04, +3.69] |
| Full model minus family baseline | +6.35 points | [+3.85, +9.00] |
| Full model minus word corrections alone | +1.19 points | [-0.22, +2.50] |
| Full model minus grammar corrections alone | +4.61 points | [+2.30, +7.06] |

Keeping only the learned word corrections scored 51.77%; keeping only grammar
corrections scored 48.35%. Resetting both reproduced the family baseline
exactly. Word corrections account for most of the observed improvement.
Grammar's conditional contribution remains uncertain. These removal tests
use the jointly fitted corrections; separately fitting a word-adjustable
model gives different weights. Secondary intervals are unadjusted diagnostics.

The selected model contains 2,048 word coordinates and 1,509 grammar coordinates.
Zero, 94 and 91 of its 3,557 multipliers reached a bound across the three seeds,
or 0%, 2.64% and 2.56%. This gives limited evidence that the current bounds
constrain the whole model. The seed-averaged word multiplier median was 0.8271;
the word-bigram median was 0.9591. These summaries describe weights, not the
independent usefulness of each feature.

Total fitting time was 1,121.62 seconds, about 18.69 minutes. At each
configuration's selected penalty, a full fit averaged 6.30 seconds for the
selected configuration and 9.44 seconds for the largest configuration.
Those timings include 200 updates, validation and
checkpoint writes, but exclude corpus preparation, exports and independent
audits.

## Frozen search

| Cap relative to the previous catalog | Word coordinates | Grammar coordinates | Combined coordinates |
|---|---:|---:|---:|
| 0.5 | 512 | 460 | 972 |
| 1 | 1,024 | 840 | 1,864 |
| 2 | 2,048 | 1,509 | 3,557 |
| 4 | 2,189 | 2,449 | 4,638 |
| 8 | 2,189 | 3,740 | 5,929 |
| All supported | 2,189 | 5,164 | 7,353 |

Word coordinates include single words and sentence-bounded word bigrams.
Grammar includes the ten categorical syntax families and twelve numerical
measurements. The original support thresholds require each categorical feature
to occur in at least ten fitting authors and thirty fitting posts. All twelve
original numerical axes vary and remain eligible.

Within each family, descending author support, descending post support and
ascending feature key determine the nested vocabulary. The base caps are 512
words, 512 bigrams and 96 coordinates per categorical grammar family. The
largest endpoint includes every supported coordinate without a cap.

Each size is paired with three choices of trainable coordinates: words,
grammar, or both. The original fourteen family distances contribute to every
model, including families whose coordinate weights stay fixed. The word
catalogs at four times, eight times and all supported are identical, so they
share one fit. This leaves sixteen distinct configurations.

Each configuration runs penalties 0.1, 1, 10 and 100 against the three
inherited baseline seeds: 192 fits in total. The new upper penalty extends
beyond the previous study's selected value of 10. Each fit starts with unit
coordinate multipliers and runs 200 full-batch Adam updates at learning rate
0.03, with betas 0.9 and 0.999, epsilon 1e-8, CPU float64 and eight threads.
Multipliers remain between 0.5 and 2. Family coefficients, spline and
temperature stay frozen.

The penalty averages squared log multipliers within each trainable family,
sums those family means and divides by fourteen. Inactive families contribute
zero. This keeps the shrinkage for a word coordinate the same at a given
penalty strength across the word-adjustable and combined models. The combined
model retains the previous study's penalty convention.

Rust exports one maximal coordinate catalog and the indices of every nested
subset. Selected coordinate differences add corrections to the complete
original family distances. Unselected and unseen coordinates retain their
original contributions. A smaller trainable vocabulary never renormalizes
probabilities or drops the remaining vocabulary.

The original 1,224 training posts from 100 authors fit the model. The true
author's reference profile excludes each query's entire date. Author profiles
average probabilities within dates and then across dates before applying the
square-root transform. Original numerical scales and full family denominators
remain fixed. The base-sized combined export must reproduce the previous
coordinate arrays exactly, and all raw family distances must remain identical.

## Selection and confirmation

Three fresh validation galleries and three fresh test galleries request
100 authors each. They exclude all 1,100 authors in the eleven previous full
exports and exclude one another. The seventeen-cohort metadata audit found
1,700 distinct author IDs, with no repeated source posts or exact/normalized
text hashes. Unsupported authors are excluded by the existing fixed rule,
without replacements or performance-based filtering.

Annotation and chronological eligibility checks retained 297 validation
authors with 514 development queries and 3,977 reference posts. Three authors
had only one eligible reference date and were excluded before fitting.
No additional posts were excluded. The fitting input and eligibility
manifests were frozen before the first update.

All 300 test authors remained eligible, with 624 test queries and 4,019
reference posts. Their eligibility and input manifests were frozen after
model selection and before final scoring. No fitting occurred during testing.

Only development queries from the new validation galleries select models.
Metrics average posts within each author, then weight every eligible author
equally across galleries. The validation galleries' test queries are unused.

For each fit, validation top-1 selects the checkpoint, then mean reciprocal
rank, then the earliest epoch including zero. For each configuration, mean
selected results across three seeds select a penalty, using top-1, then MRR,
then stronger penalty. The overall configuration uses mean top-1, then MRR,
then fewer trainable coordinates, then canonical configuration ID. Separate
winners are also frozen within each group of adjustable coordinates.

All 192 fits must finish before selection. The runner retains 38,592 epoch
checkpoints and stops if a fit fails. The final test comparison uses the
frozen overall winner against the incumbent. The single primary criterion
requires the paired 95% bootstrap interval's lower bound to exceed zero.
Bootstrap resampling preserves each gallery's eligible author count and
weights all authors equally. It uses the established xorshift seed, 2,000
replicates and sorted percentile indices 49 and 1949.

All sixteen selected-penalty models and the three group winners are reported
as secondary results. Additional diagnostics reset the chosen combined
model's word corrections, grammar corrections, or both to zero. Resetting
both must reproduce the frozen family baseline. These diagnostics measure
conditional contributions of the learned corrections. All original feature
families still contribute to their scores. Secondary intervals are descriptive
and unadjusted for multiple comparisons.

This study holds support thresholds, fitting authors and the family scorer
fixed. Wider multiplier bounds, joint family learning, more fitting authors
and new grammar representations remain open investigations. Author retrieval
also needs content and reference-sample robustness checks. Successful edits,
readability, fidelity and detector performance require their own evaluations.

## Verification and reproduction

Rust formatting, full workspace tests and all-target Clippy passed. All 34
Python tests and the dependency check passed. The original-size subset was
byte-identical to the previous coordinate export. All nine overlapping fits
reproduced the old training trajectories exactly: 1,809 comparisons of
coordinate weights, training cross-entropy and objective had zero error.

The independent audit checked all 38,592 checkpoint hashes, reconstructed
all 192 selected states, validation results and training objectives, and
verified every selection rule. It reconstructed all 21 test variants and
39,312 complete candidate rankings, including ties, with maximum absolute
score error `7.105427357601002e-14`. All four paired bootstrap comparisons
matched. Independent geometry audits covered TRAIN and all six new exports.

The [Rust exporter instructions](../grammar/crates/grammar-eval/README.md)
and [Python fitting commands](../grammar/crates/grammar-eval/python/coordinate-frontier.md)
describe reproduction. Use fresh output directories and preserve the frozen
input identities. No external model or detector calls occurred in this study.

The separate [Rust inference implementation](coordinate-inference.md) makes
the incumbent usable without PyTorch. Its 177,000 earlier test scores matched
Python within `5.684341886080802e-14`, with identical candidate orderings.

To regenerate the aggregate CSV and PNG, SVG and PDF plots, run from the
project root using the separate environment pinned by `requirements-plot.lock`:

```sh
PLOT_PYTHON grammar/crates/grammar-eval/python/plot_coordinate_frontier.py \
  --selection data/author-corpora/blog-authorship-2004/coordinate-frontier-v1/train-v1/selection.json \
  --report data/author-corpora/blog-authorship-2004/coordinate-frontier-v1/test-v1/report.json \
  --out data/author-corpora/blog-authorship-2004/coordinate-frontier-v1/figures-replay
```

Protocol SHA-256:
`cbe994d178eace3252a9644af2b1fe7d2c6536373a6bf6a504c30da216dfa780`.

Selection SHA-256:
`ef1f095929d80e5ae9a282d671e1b060a35e48a4ae8ef83617ee3d35e96f84a8`.

Test report SHA-256:
`e9b9616d7d36dbe406c6be1d2acf892b1917e8c7100c68f056f5fd697d881004`.

Local corpus files, coordinate exports, fitted models and detailed outcomes
remain in ignored `data/author-corpora/blog-authorship-2004/coordinate-frontier-v1/`.
The cohort audit is in `coordinate-frontier-cohort-audit-v1/`. Existing frozen
sources and artifacts are preserved; this stage adds separate Rust and Python
modules.
