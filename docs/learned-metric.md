# Learning feature weights

For the subsequent comparison of smaller models and larger neural networks,
see the [model-size frontier](model-size-frontier.md).

The author experiment keeps the current grammar and word-occurrence extractor
fixed and learns how much each feature family contributes to distance. The
first model has fourteen weight parameters and one temperature parameter. It
has no hidden layer. Rust handles both training and evaluation; saved local
spaCy annotations supply the syntax.

For each query and candidate author, the model receives fourteen squared
distances. Categorical families use squared Hellinger distance. Numerical
families use the original training scales. A softmax over the weight parameters
gives positive family weights that sum to one. Candidate-author logits are
the negative weighted distance divided by a positive temperature. Training
minimizes author classification cross entropy.

Temperature changes training probabilities. For a fixed set of family weights,
it does not change the order of nearest authors. The saved model retains it for
reproduction; the evaluation uses the selected family weights directly.

## Data separation

The original author cohort supplies training and development data. Each training
post acts as a query. Its own author's target excludes every post from the
query's date, while other authors' targets use their training dates. This avoids
self-matches and same-date leakage. It leaves a small difference in profile
support between the positive author and other authors.

Profiles weight dates equally, then posts within each date equally. The training
loss weights authors equally, then dates, then posts. A fixed development rule
selects the checkpoint with the highest macro-author top-1 accuracy, then mean
reciprocal rank, then the earliest epoch. Epoch zero is eligible. All epoch
results remain available.

A fresh cohort excludes every original author and all exact or normalized text
duplicates. The fresh authors supply their own training references, but their
held-out outcomes never fit the model or select a checkpoint. The original
numerical scales remain fixed. `Space::extend_basis` adds new categorical names
and binds a new space identity while preserving existing transforms and target
values.

## Comparisons

The fixed baseline gives words and grammar half the total family weight each.
A smaller grid also compares different total grammar weights, either uniform
within grammar or proportional to measured training consistency. Both word-only
and grammar-only endpoints are included.

The learned model can change each family separately, including the balance
between single words and word pairs. The final evaluation therefore includes
an additional lexical ablation: keep the model's selected word and word-pair
weights and renormalize them. Comparing the full learned model with this ablation
helps identify whether grammar adds anything beyond a better lexical balance.
No additional training or checkpoint selection occurs for that comparison.

Reports contain paired author-bootstrap differences against the baselines.
The primary task is identifying an author among the candidate profiles. Topic,
register, copied passages and account attribution remain possible explanations
for part of that signal. The training stability diagnostic matches pairs within
content-distance and length-ratio bins; those coarse controls leave residual
differences and do not supply independently verified topics.

These experiments change fourteen family weights. Individual word and syntax
coordinates, feature definitions and rewrite rules remain fixed. Author
retrieval alone does not measure readable revision, semantic preservation or
detector scores. Store all corpus text, training tensors, checkpoints and author
identifiers in ignored local `data/`.

## Reproduction

Run from the `slop_ninja` directory after producing the original evaluation and
freezing the neural experiment protocol:

```sh
cargo build --release --manifest-path grammar/Cargo.toml \
  -p grammar-eval --bin unslop-learn-weights

grammar/target/release/unslop-learn-weights \
  data/author-corpora/blog-authorship-2004/vector-evaluation-v1 \
  --export data/author-corpora/blog-authorship-2004/authors-v1 \
  --cache data/author-corpora/blog-authorship-2004/vector-feature-cache-v1 \
  --protocol data/author-corpora/blog-authorship-2004/neural-weight-transfer-v1/protocol.json \
  --out data/author-corpora/blog-authorship-2004/neural-weight-transfer-v1/fit-v1
```

The output directory must not exist. The fixed optimizer uses full-batch Adam
for 200 epochs, with learning rate 0.03, beta values 0.9 and 0.999, and epsilon
1e-8. Initial family weights match the original combined geometry: 0.25 each
for words and word bigrams, and 0.5/12 for each grammar family. The initial
temperature is 0.1, and log-temperature stays within [-6, 2]. The runner checks
these settings against the saved protocol before loading training sources.

Before training, it checks original manifest and evaluation hashes, author
identities, exact text, dates, source groups and cached annotations. It
re-extracts features from the validated annotations and compares them with the
cache. It also checks that the original space was fitted to exactly these
training sources. No external model or detector calls are needed.

The output includes:

- `input-validation.json`: source hashes, cache hashes and original attrition.
- `training-tensor.json`: every query, candidate distance, loss weight and source
  binding needed to replay optimization.
- `epochs.jsonl`: every checkpoint, training loss, development metrics, family
  weights, temperature, optimizer state and gradient norm.
- `selection.json`: the selected weights in the existing `FrozenSelection`
  format, selected checkpoint, input/artifact hashes and executed binary hash.

Analytic gradient tests cover every weight parameter and log-temperature.
Additional tests exercise whole-date exclusion, author/date/post loss balance,
optimizer convergence, positivity and temperature bounds, and selected
checkpoint replay with the earliest-epoch tie rule.
