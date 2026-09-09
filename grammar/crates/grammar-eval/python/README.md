# Model-size frontier

The experiment trains PyTorch models on the same fourteen per-family distances
used by the Rust author evaluator. Python is limited to this ML dependency;
Rust supplies corpus preparation, parser validation, features, profiles and
the original distances.

`requirements-ml.lock` pins the training dependencies. `training-lock.json`
records the tested Python/platform versions and the PyTorch wheel digest. The
local installation constrained all existing environment packages to their
installed versions and added only missing dependencies. This run used PyTorch
2.14.0, CPU float64, eight intra-operation threads, one inter-operation thread,
and deterministic algorithms.

Run from the `slop_ninja` directory:

```sh
.venv/bin/python -m unittest discover \
  -s grammar/crates/grammar-eval/python -p 'test_*.py' -v

.venv/bin/python grammar/crates/grammar-eval/python/train_frontier.py train \
  --original-fit data/author-corpora/blog-authorship-2004/neural-weight-transfer-v1/fit-v1 \
  --evaluation data/author-corpora/blog-authorship-2004/vector-evaluation-v1 \
  --protocol data/author-corpora/blog-authorship-2004/model-size-frontier-v1/protocol.json \
  --eligibility-addendum data/author-corpora/blog-authorship-2004/model-size-frontier-v1/eligibility-addendum.json \
  --out data/author-corpora/blog-authorship-2004/model-size-frontier-v1/train-v1
```

The output directory must not exist. The protocol fixes fifteen architectures,
three seeds, 200 Adam updates, and development selection before the run begins.
Every model receives all fourteen inputs. Tied positive weights use 8 or 12
parameters; independent positive weights use 15. Shared monotone splines use
18, 20, 23, 30, 45 or 60 parameters. A signed linear control uses 14. One-hidden-
layer tanh networks use 64, 256, 1,024, 4,096 or 16,384 parameters. These counts
include every trainable scalar; the unconstrained networks omit output bias and
temperature.

Spline knots use a nested sequence of training-distance quantiles. For the
linear control and tanh networks, `log1p` distances are standardized with
training-only means and population standard deviations. All transforms remain
fixed when scoring development or fresh-author queries.

Each seed selects its checkpoint by original-development macro-author top-1,
then MRR, then earliest epoch, including epoch zero. Model selection uses the
mean of the three selected development scores, then smaller parameter count
and model name. It never selects a seed. The original 15-parameter seed-zero
run must reproduce all 201 saved Rust checkpoints within a float64 tolerance.

The trainer retains per-epoch parameter values, losses, development scores,
checkpoint hashes and timings. Each `.pt` checkpoint also contains model and
optimizer state. It records failures explicitly and excludes an architecture
from model selection unless all three seeds complete. It writes the selected
architecture and every seed's selected checkpoint into `selection.json`.

After that selection is frozen, evaluate the complete frontier:

```sh
.venv/bin/python grammar/crates/grammar-eval/python/train_frontier.py evaluate \
  --fit data/author-corpora/blog-authorship-2004/model-size-frontier-v1/train-v1 \
  --evaluation data/author-corpora/blog-authorship-2004/vector-evaluation-frontier-eligible-v1 \
  --protocol data/author-corpora/blog-authorship-2004/model-size-frontier-v1/protocol.json \
  --eligibility-addendum data/author-corpora/blog-authorship-2004/model-size-frontier-v1/eligibility-addendum.json \
  --split test \
  --out data/author-corpora/blog-authorship-2004/model-size-frontier-v1/test-v1
```

The evaluator verifies frozen hashes, author separation, eligible author/post
IDs, parser/features and the original numerical space. It saves every candidate
score and exact-tie-aware query metric. Summary scores average metrics across
the three independent seed fits. They are not ensemble predictions. Paired
author-bootstrap comparisons use those seed means, with seed variation shown
separately.

Spline monotonicity does not establish the triangle inequality. Signed linear
and tanh networks can learn nonmonotone ranking functions. Architecture and
preprocessing differ between model families, so parameter count alone cannot
explain a difference in performance. This experiment does not test text edits,
readability, semantic fidelity or detector scores.

`benchmark_frontier.py` uses synthetic tensors only. Its preliminary MLP timing
included a temperature parameter; the final frozen MLPs omit that parameter.
Keep this difference visible when comparing projected and measured cost.
