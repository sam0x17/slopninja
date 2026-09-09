# Model-size frontier

The subsequent [spline confirmation](spline-confirmation.md) reused the frozen
models on another 100 authors. The spline gained 5.67 points, with a paired
95% interval of [-0.17, +11.00], falling short of the predeclared criterion.

Larger tanh networks did not improve author retrieval on the fresh cohort in this
experiment. The 4,096-parameter network won the frozen development comparison
at 64.18%, then scored 42.06% on new authors. The original 15-parameter model
scored 44.01% on those authors, and the 8-parameter model matched its top-1
results. Keep the small positive-weight model as the reference for further
representation work. The evidence does not justify scaling the tanh network.

We trained fifteen architectures with three seeds each. All received the same
fourteen word and grammar family distances. Counts range from 8 to 16,384
stored trainable scalars, or 0.53 to 1,092.27 times the original model. Integer
parameter counts approximate the requested 0.5, 1.2, 1.3, 1.5 and 2 times sizes;
larger variants extend the comparison well beyond that range.

The original cohort supplied 1,224 training queries and 168 development posts
from 100 authors. Each training query excluded its entire date from its own
author's reference profile. The training loss weighted authors equally, then
dates, then posts. Each fit ran 200 full-batch Adam updates at learning rate
0.03, using CPU float64, eight threads and seeds 0, 17 and 29.

Each seed selected its checkpoint by development macro-author top-1, then mean
reciprocal rank, then earliest epoch. Epoch zero was eligible. The architecture
selection used mean development top-1 across the three seeds, then mean MRR,
then fewer parameters, then name. We froze that selection before inspecting
fresh-author outcomes. No best seed was selected.

The fresh cohort excluded all 200 authors used in the two previous cohorts.
Its original export contained 100 authors. Before opening performance results,
we recorded an eligibility addendum excluding two authors with only one
training date and two posts with unavailable required features. Every model
used the same remaining 98-author gallery and 219 test posts. Fresh authors
provided their own training reference profiles; the original numerical scales,
learned parameters and preprocessing remained fixed.

| Model | Parameters | Multiple | Development top-1 | Fresh top-1 | Training seconds/fit |
|---|---:|---:|---:|---:|---:|
| Tied positive weights | 8 | 0.53× | 57.54% | 44.01% | 0.346 |
| Tied positive weights | 12 | 0.80× | 57.54% | 44.01% | 0.333 |
| Signed linear control | 14 | 0.93× | 60.12% | 43.50% | 0.140 |
| Independent positive weights | 15 | 1.00× | 57.54% | 44.01% | 0.335 |
| Shared monotone spline | 18 | 1.20× | 58.15% | 43.17% | 1.428 |
| Shared monotone spline | 20 | 1.33× | 60.83% | 46.41% | 1.329 |
| Shared monotone spline | 23 | 1.53× | 61.29% | 45.88% | 1.191 |
| Shared monotone spline | 30 | 2.00× | 61.54% | 44.86% | 1.152 |
| Shared monotone spline | 45 | 3.00× | 63.29% | 46.53% | 1.105 |
| Shared monotone spline | 60 | 4.00× | 62.79% | 46.02% | 1.067 |
| Tanh network, 4 hidden units | 64 | 4.27× | 60.38% | 41.17% | 0.543 |
| Tanh network, 16 hidden units | 256 | 17.07× | 63.63% | 42.17% | 0.838 |
| Tanh network, 64 hidden units | 1,024 | 68.27× | 64.01% | 41.54% | 2.609 |
| Tanh network, 256 hidden units; development selection | 4,096 | 273.07× | 64.18% | 42.06% | 10.450 |
| Tanh network, 1,024 hidden units | 16,384 | 1,092.27× | 63.63% | 42.49% | 44.909 |

Accuracy gives each author equal weight and averages metrics across three
independent fits. It is not an ensemble prediction. The fixed words-and-bigrams
baseline scored 40.92% on the fresh test; the original fixed combined weights
scored 36.33%. These scores describe author identification, not AI detection.

The primary comparison, the development-selected network against the original
15-parameter model, was -1.95 percentage points with a paired author-bootstrap
95% interval of [-6.54, +2.72]. Against fixed words and bigrams, it was +1.15
points with an interval of [-2.94, +5.78]. We used 2,000 paired author resamples;
the intervals condition on these three seeds. Seed variation is reported
separately.

The 45-parameter spline led the fresh test at 46.53%. Its +2.52-point difference
from the original model had an interval of [-0.54, +6.12]. That result is
exploratory because we identified it by comparing test outcomes. A separate
author cohort is needed to evaluate this choice. Matching top-1 results also
does not make the 8- and 15-parameter models equivalent: some lower-ranked
author orderings differ.

The cost column measures mean training compute across three full 200-update
fits, excluding development scoring and checkpoint I/O. All 45 fits completed
without failure; their recorded elapsed times sum to 237.6 seconds. On the same
warm scoring workload of 168 queries against 100 candidates, the 15-parameter
model took 0.114 ms, the selected network 4.517 ms, and the largest network
19.581 ms. The 8-parameter model took 0.116 ms, so parameter compression did not
produce a measured latency saving. All models still need fourteen distances.

Counts include every stored trainable scalar. Positive-weight softmax models
have a common-logit redundancy, and their temperature does not change candidate
ranking. Splines add a shared monotone calibration whose knots use only
training distances. The signed linear control and tanh networks instead
standardize `log1p` distances using training means and population standard
deviations. The networks omit output bias and temperature. Architecture and
preprocessing vary along with size, so this study cannot isolate parameter
count as the cause of a difference. It also uses one optimizer schedule and
one corpus. Author-disjoint development data would better match the transfer
task in the next experiment.

Rust handles corpus preparation, validated annotations, feature extraction,
profiles and geometry. The [PyTorch runner](../grammar/crates/grammar-eval/python/README.md)
handles these ML fits. A separate NumPy implementation reproduced all 45
selected checkpoints' training losses and development metrics, all 9,855 fresh
query rankings, and all 30 paired comparisons. It verified every one of the
9,045 checkpoint hashes. Maximum logit disagreement was 1.42e-13. The original
15-parameter seed-zero run also reproduced all 201 Rust checkpoints within
1e-8. All nine Python tests and 85 Rust workspace tests passed, along with
formatting and all-target clippy checks.

The frozen selection digest is
`aa511c72105b48f08243d49153698c923cb0f0641d69a46c32ea9d6ef93e7b4a`.
Full local artifacts are under
`data/author-corpora/blog-authorship-2004/model-size-frontier-v1/`:

- `protocol.json` and `eligibility-addendum.json`: pre-outcome rules.
- `train-v1/`: inputs, transforms, every checkpoint, timings and frozen choice.
- `test-v1/`: every candidate score, seed result and paired comparison.
- `independent-audit.py` and `independent-audit.json`: independent reconstruction.
- `figures-v2/`: accuracy and cost plots in PNG, SVG and PDF; aggregate CSV and
  provenance hashes. Whiskers show seed ranges, not confidence intervals.

Use a separate environment with
`grammar/crates/grammar-eval/python/requirements-plot.lock` to reproduce the
figures without changing training dependencies. From the project root:

```sh
python grammar/crates/grammar-eval/python/plot_frontier.py \
  --training data/author-corpora/blog-authorship-2004/model-size-frontier-v1/train-v1 \
  --evaluation data/author-corpora/blog-authorship-2004/model-size-frontier-v1/test-v1 \
  --out data/author-corpora/blog-authorship-2004/model-size-frontier-v1/figures-new
```

Corpus text, author identifiers and fitted artifacts remain ignored locally.
No LLM or detector calls were used. This experiment does not evaluate edits,
readability, preservation of meaning or detector performance.
