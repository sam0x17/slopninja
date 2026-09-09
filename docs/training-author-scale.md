# Training the fixed representation on more authors

Training on 300 authors improved retrieval from **54.72% to 56.30%** on
600 new authors and 1,206 test posts. The paired gain was **1.58 percentage
points**, with a 95% interval of **[+0.51, +2.69]**. All six test galleries
improved. This passes the study's predefined primary comparison.

This study compares the same 3,557-coordinate model fitted on 100 versus 300
authors. The previous capacity frontier found little additional improvement
from larger catalogs, while its selected model was fitted on only 100 writers.
Here the representation, numerical scales, family scorer, multiplier bounds
and optimizer remain fixed. Only the training panels change.

## Results

Each query chooses among 100 authors. These are means of three inherited-seed
metrics, with equal weight per author; they are not ensemble predictions.

| Model | Top-1 | Top-5 | MRR |
|---|---:|---:|---:|
| Frozen family baseline | 50.86% | 69.54% | 0.5947 |
| Earlier 1,864-coordinate incumbent | 53.49% | 72.16% | 0.6257 |
| 3,557 coordinates, 100 training authors | 54.72% | 73.06% | 0.6369 |
| 3,557 coordinates, 300 training authors | **56.30%** | **74.57%** | **0.6498** |
| Treatment with only word corrections | 56.22% | 73.80% | 0.6474 |
| Treatment with only grammar corrections | 51.17% | 69.27% | 0.5980 |
| Treatment with all corrections reset | 50.86% | 69.54% | 0.5947 |

The reset variants retain all original word and grammar distances. They reset
selected learned corrections to unit multipliers, so the word-correction row
is not a word-only representation. Resetting all corrections reproduced every
family-baseline score exactly.

| Top-1 comparison | Difference, percentage points | Paired 95% interval |
|---|---:|---:|
| 300 minus 100 training authors, primary | +1.58 | [+0.51, +2.69] |
| 300 minus earlier incumbent | +2.80 | [+1.46, +4.28] |
| 100 minus earlier incumbent | +1.23 | [-0.05, +2.51] |
| 300 minus family baseline | +5.44 | [+3.71, +7.29] |
| Grammar corrections, conditional on word corrections | +0.08 | [-0.47, +0.67] |
| Word corrections, conditional on grammar corrections | +5.13 | [+3.39, +6.96] |

Only the first comparison is primary. Other intervals are descriptive and
unadjusted. Grammar corrections also improved top-5 by 0.76 points, with an
unadjusted interval of [+0.14, +1.44], but neither top-1 nor MRR established
a conditional gain. This does not show that the underlying grammar families
are useless: their frozen contributions remain active in every reset variant.

| Fresh test gallery | Test posts | 100-author control | 300-author treatment | Gain, points |
|---|---:|---:|---:|---:|
| 1 | 197 | 54.61% | 54.83% | +0.22 |
| 2 | 203 | 50.42% | 51.58% | +1.17 |
| 3 | 205 | 48.09% | 49.09% | +1.00 |
| 4 | 194 | 55.42% | 59.08% | +3.67 |
| 5 | 199 | 58.33% | 59.58% | +1.25 |
| 6 | 208 | 61.45% | 63.62% | +2.17 |

Validation selected lambda 0.1 and epochs 20, 19 and 19 for the treatment's
seeds 0, 17 and 29. Mean validation top-1 was 61.95%, against the control's
61.49%. The twelve new fits took 181.06 seconds in total, excluding data
preparation, evaluation and audits. Treatment test top-1 varied from 56.27%
to 56.35% across the three seeds. No seed was selected from these results.

The 300-author model is a useful working baseline for further representation
research. Larger training coverage helped this fixed representation, while
the added grammar weights supplied little measurable top-1 improvement.
The next question is which word and grammatical signals transfer when topic
similarity is less helpful. This experiment does not answer that question,
and these six galleries are now evaluated data for any follow-up analysis.
The subsequent [content-proxy diagnostic](content-proxy-diagnostic.md) reports
30.23% retrieval where another author is a closer content match, compared
with 27.92% for the control. Those conditional results remain exploratory.

## Fitting data and control

| Training panel | Authors | Training posts | Minimum reference dates |
|---|---:|---:|---:|
| Original | 100 | 1,224 | 2 |
| Replication | 100 | 1,363 | 3 |
| Spline confirmation | 100 | 1,302 | 4 |
| Treatment total | 300 | 3,889 | 2 |

The added panels are the earliest previously evaluated cohorts that retain all
100 eligible authors. The intervening frontier cohort retained only 98 and was
skipped by this metadata rule. Selection did not use a cohort's model score.
The added cohorts previously informed development and now serve explicitly as
training data. Their earlier test results no longer supply independent
confirmation.

Each query competes only within its fixed 100-author panel. This retains
99 competing authors per query in both arms. The true author's reference
profile excludes the query's entire date. Within each panel, the loss weights
authors equally, dates equally within an author and posts equally within a
date. The treatment averages the three panel losses, applies the coordinate
penalty once and performs one Adam update.

Both arms use 200 updates at learning rate 0.03, betas 0.9 and 0.999, epsilon
1e-8, CPU float64 and eight threads. Coordinate log multipliers start at zero
and remain between log(0.5) and log(2). The penalty is lambda divided by
fourteen, multiplied by the sum of each family's mean squared log multipliers.
Only those coordinate multipliers train. Family weights, the shared monotone
spline and temperature stay fixed.

The treatment processes more training pairs per update. The experiment compares
the two data configurations at the same update count; it does not hold compute
constant or isolate author count from the composition of the added panels.

The control reuses the twelve completed `all_m2` fits from the coordinate
capacity study. All 2,412 control checkpoint hashes, epoch choices and lambda
selection are verified. The control's selected validation metrics must replay
exactly. Reusing those trajectories saves identical fitting work.

## Fixed coordinates and complete distances

Both arms use the immutable `all_m2` subset: 2,048 word coordinates and 1,509
grammar coordinates. Rust applies the original maximal catalog to the new
panels, and Python selects the same 3,557 indices. The added writers do not
change support thresholds, add newly supported coordinates or refit scales.

Categorical coordinates are square roots of half the observed probability.
Author profiles average probabilities within dates and then across dates
before transformation. Numerical coordinates retain their original scales
and complete family denominators. Every unselected and unseen feature remains
in the complete family distance with unit multiplier. See the
[fixed-panel exporter](fixed-coordinate-panels.md).

The independent geometry audit reconstructs all added source profiles,
whole-date positive replacements and complete original family distances,
including vocabulary outside the trainable subset. Separate gradient tests
check the pooled objective with finite differences, its exact one-panel
reduction and a single regularization penalty.

## Selection and fresh confirmation

Both arms use the same 297 validation authors and 514 development queries
from the capacity study. These authors are disjoint from all training panels.
Their test queries remain unused. Reusing validation authors is part of this
study's development history.

The treatment runs penalties 0.1, 1, 10 and 100 with inherited seeds 0, 17 and
29: twelve new fits and 2,412 retained checkpoints. Validation macro-author
top-1 selects each checkpoint, followed by MRR and then the earliest epoch,
including zero. Mean validation results across seeds select lambda, using
top-1, then MRR, then stronger penalty. Both arms receive the same selection
procedure. No seed or training arm is chosen from test outcomes.

Six fresh test galleries request 100 source authors each. The sampling plan
excludes every author in all seventeen prior full cohorts, including those
later excluded during annotation, and each preceding new gallery. The raw
metadata audit covers 2,300 distinct authors and 39,273 posts across all
23 cohorts. It found no repeated source posts, exact or normalized text hashes,
or source-date groups across cohorts.

Test eligibility applies the same existing rule to every variant. Unsupported
authors or posts are excluded without repair or replacement, with actual
counts frozen before model scoring. Each gallery's training posts build
reference profiles; its development posts remain unused. Only test posts
evaluate the frozen choices.

The single primary comparison is the 300-author treatment minus the
100-author control in macro-author top-1 accuracy. Metrics average queries
within each author and then weight authors equally. Seed metrics are averaged
before paired author resampling; they are not ensemble predictions.

The paired bootstrap resamples within each test gallery using its actual
eligible author count, concatenates the sampled authors and averages them.
It uses 2,000 replicates, xorshift64 seed `0x6a09e667f3bcc909`, lexical gallery
and author ordering, continuous generator state per comparison, and sorted
percentile indices 49 and 1949. Superiority requires the primary 95% interval's
lower bound to exceed zero.

The 1,864-coordinate incumbent and frozen family baseline are secondary
comparators. Further diagnostics reset the treatment's word corrections,
grammar corrections or all corrections. Resetting all must reproduce the
family baseline exactly. These measure conditional contributions of the fitted
corrections while retaining every original feature family. Their intervals
are descriptive and unadjusted for multiple comparisons. Top-5 accuracy and
MRR are also secondary.

The study adds no new grammar representation and does not jointly retrain
family calibration. Author retrieval on blogs still mixes style with topic,
genre and other persistent author characteristics. Successful text edits,
fidelity, readability and detector performance require separate evidence.

## Verification

The independent reviewer checked 2,412 new checkpoint hashes and 2,412 reused
control hashes, reconstructed every selected treatment state and all twelve
three-panel training objectives, and replayed checkpoint and lambda selection.
Separate source-cache audits reconstructed the complete geometry for both
added training panels and all six test galleries.

The final audit reconstructed all seven variants across three seeds and six
galleries: 126 score files and 25,326 complete candidate rankings. Every
per-query ranking metric and all six paired comparison intervals agreed;
the largest candidate-score difference was 7.11e-14. All 38 Python tests and
the required Rust formatting, workspace tests and Clippy checks passed.

The report has one descriptive metadata omission: the earlier incumbent's
three `selected_epoch` fields are null. Its protocol records epochs 52, 53
and 56, and the audit verified the corresponding checkpoint hashes and
numerical states. The audit records the omission; the frozen report is
preserved, with the correct epochs in `incumbent-epoch-metadata-addendum.json`.
It does not affect scores, selection or the primary comparison.

Protocol SHA-256:
`e5a080a9b7580c687d718f67a67cd174a2b02f6a6db0173dccfd177ddf035b08`.

Frozen selection SHA-256:
`fdfceaa808b9db8c574c163fc6ba2eed0a54ee592e4da15dc5ebc931b4fae0fe`.
Final test report SHA-256:
`433673fee5e2bc6a386ae634b60ca16940fb4c4b8d90b76a104042c41694c761`.

Exact corpus text, profiles, checkpoints and detailed outcomes remain in
ignored `data/author-corpora/blog-authorship-2004/training-author-scale-v1/`.
The raw metadata audit is in `training-author-scale-cohort-audit-v1/`.
The [training commands and implementation checks](../grammar/crates/grammar-eval/python/coordinate-scale.md)
describe reproduction. Standalone PNG, SVG and PDF figures and their aggregate
CSV inputs are retained in the study's `figures-v1/` directory.
