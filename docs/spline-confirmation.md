# Confirming the spline on new authors

The subsequent [author-disjoint selection study](author-disjoint-selection.md)
selects checkpoints on different authors and tests grammar's contribution
against a separately calibrated words-only model.

The frozen 45-parameter spline scored 58.08% on a fourth author cohort, compared
with 52.42% for the 15-parameter positive-weight model. The difference was
**+5.67 percentage points**, with a paired 95% author-bootstrap interval of
**[-0.17, +11.00]**. This did not meet the criterion fixed before scoring: the
interval's lower bound had to exceed zero. The observed gain is encouraging,
but this experiment remains inconclusive about superiority.

The [earlier frontier](model-size-frontier.md) identified the spline by comparing
test results. This follow-up used 100 new authors, excluding all 300 previously
exported author IDs, including authors excluded from earlier evaluations. The
new corpus export contained 1,654 posts and 577,741 words: 1,302 training posts,
165 development posts and 187 test posts. All authors and posts passed the
predeclared eligibility rules. The final reference gallery contained all 100
authors. The metadata audit found no author, source-post, exact-text or
normalized-text overlap with the three earlier exports and no date-group
leakage across splits.

We froze the models, comparison, source hashes and eligibility rule before
annotation or scoring. We reused the original per-seed development-selected
checkpoints for three architectures and seeds 0, 17 and 29. There was no
retraining, new checkpoint selection or seed selection. Fresh training posts
supplied only author reference profiles; the feature meanings, numerical
scales, spline knots and learned parameters stayed fixed. Fresh development
outcomes did not select anything. Earlier test outcomes were not pooled into
the primary result.

| Model | Parameters | Fresh top-1 | Fresh top-5 | Mean reciprocal rank |
|---|---:|---:|---:|---:|
| Shared monotone spline | 45 | 58.08% | 77.75% | 0.6696 |
| Independent positive weights | 15 | 52.42% | 71.58% | 0.6201 |
| Tied positive weights | 8 | 51.42% | 71.58% | 0.6141 |
| Fixed words and bigrams | Fixed | 51.92% | 72.58% | 0.6095 |
| Original fixed combined weights | Fixed | 45.25% | 64.25% | 0.5399 |

Scores average per-query metrics across three independent fits, then average
within each author, then give each author equal weight. They are not ensemble
predictions. The spline's three seed scores ranged from 57.92% to 58.42%.
The bootstrap used 2,000 paired author resamples and conditions on this fixed
gallery and these three seeds. The wider interval reflects variation across
authors, which is different from variation across seeds.

The secondary spline comparison with fixed words and bigrams was +6.17 points,
with an interval of [0.00, +12.50], after rounding floating-point zero. That
lower endpoint does not establish a strictly positive effect. The 8-parameter
model scored 1.00 point below the 15-parameter model, with an interval of
[-3.00, 0.00]. These secondary comparisons are descriptive. Neither establishes
equivalence or noninferiority of the smaller model. The previous cohort's
matching top-1 scores did not persist here.

The spline changes how family distances contribute to the score. In its frozen
seed-zero checkpoint, raw squared distances 0.1, 1 and 2 become approximately
2.21, 5.01 and 5.24 before weighting. It leaves distances below about 0.015
unchanged, expands increments above that small-distance threshold, and reduces
additional distance above about 0.96 to roughly 0.22 times its original
increment. Its weight coefficients allocate about 67.1% to
words, 17.6% to bigrams and 15.3% to the twelve grammar families, compared with
78.5%, 15.6% and 6.0% for the seed-zero 15-parameter model. These coefficients
are not feature-importance estimates: the calibration and each family's
distance distribution also affect the score. This observation describes the
saved models; it was not used to change the confirmation protocol.

Keep the 15-parameter model as the reference. The spline deserves further
study, with checkpoint selection on authors disjoint from fitting authors and
a separately fixed evaluation sample. Expanding the current sample after
seeing its interval would change the stopping rule, so this run ends at its
predeclared sample size. This is one corpus of author accounts with relatively
few held-out posts per author. Topic, quotations, copied passages and account
attribution can still contribute to retrieval. The result does not establish
editing quality, meaning preservation or detector performance.

The new [confirmation evaluator](../grammar/crates/grammar-eval/python/confirmation.md)
verifies the frozen checkpoints, original scoring-source hashes, reference
space, cohort provenance, eligibility and Rust artifact hashes before scoring.
It saves all nine candidate-score files. A separate NumPy implementation
reproduced all 1,683 complete candidate rankings and all five paired comparisons;
maximum logit disagreement was 8.53e-14. All twelve Python tests passed, the
dependency check was clean, and the original trainer/scorer source hashes
remained unchanged. This follow-up did not change Rust code.

Full artifacts remain in ignored local
`data/author-corpora/blog-authorship-2004/spline-confirmation-v1/`:

- `protocol.json`: frozen comparison, nine checkpoints and cohort bindings.
- `eligibility.json` and `freeze-eligibility.py`: metadata-only eligibility
  freeze and independent author-support check.
- `prepare.sh` and `preparation.log`: Rust annotation and distance preparation.
- `test-v1/`: inputs, all scores and the final report.
- `independent-audit.py`, `independent-audit.json` and `final-checks.json`:
  numerical reconstruction, tests and source-hash verification.
- `frozen-model-mechanism.json`: saved coefficients and calibration probes.
- `figures-v2/`: standalone PNG, SVG and PDF comparison, with provenance hashes.

The cohort and its reproducible metadata audit are in the adjacent
`authors-spline-confirmation-v1/` directory. The shared Rust distance artifacts
are in `vector-evaluation-spline-confirmation-v1/`.

Protocol SHA-256:
`d7b46dbec346e62865df4a827f861f014046d813a92bce813ec3b18b8ce16fef`.
Eligibility SHA-256:
`fc9a7afedccccb21caff25cc93df2360460e1440bfb16ba4fec44e0e002a5ef0`.
Result SHA-256:
`c29f3897a86ead682c627c7aae76bdc032168db41c908838cc355bc47ca05baf`.

Use the separate plotting environment pinned by
`grammar/crates/grammar-eval/python/requirements-plot.lock`:

```sh
python grammar/crates/grammar-eval/python/plot_confirmation.py \
  --report data/author-corpora/blog-authorship-2004/spline-confirmation-v1/test-v1/report.json \
  --out data/author-corpora/blog-authorship-2004/spline-confirmation-v1/figures-new
```
