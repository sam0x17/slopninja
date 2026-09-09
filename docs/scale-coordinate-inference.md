# Rust inference for the 3,557-coordinate scale model

The selected 300-author model uses the frozen `all_m2` subset: 3,557 named
coordinates from the 7,353-coordinate catalog. The new adapter and replay
command preserve the earlier scorer and experiment sources.

[`export_scale_metric.py`](../grammar/crates/grammar-eval/python/export_scale_metric.py)
reads the three selected PyTorch checkpoints and writes portable
`slopninja-positive-coordinate-metric-v1` JSON files. It verifies the pinned
selection and protocol hashes, checkpoint bindings, original numerical scales,
and exact subset order. Each model binds the maximal catalog hash and the
`fixed_subset_indices` hash. Family weights, spline parameters and temperature
remain those of the inherited family model.

Run the adapter from the project root, supplying the frozen selection and
protocol digests and a fresh output directory:

```sh
.venv/bin/python grammar/crates/grammar-eval/python/export_scale_metric.py \
  --selection SCALE_FIT/selection.json --selection-sha256 FROZEN_SELECTION_SHA256 \
  --protocol SCALE_PROTOCOL.json --protocol-sha256 FROZEN_PROTOCOL_SHA256 \
  --catalog MAXIMAL_COORDINATE_EXPORT/catalog.json --space ORIGINAL_SPACE.json \
  --baseline-fit FAMILY_SPLINE_FIT --out data/portable-scale/models
```

[`slopninja-score-subset`](../grammar/crates/grammar-eval/src/bin/slopninja-score-subset.rs)
replays frontier and fixed-panel exports through the unchanged Rust `Metric`.
It gathers the named subset from the maximal query and author arrays while
keeping all 14 complete family distances. Contributions from unselected or
unseen words and constructions retain multiplier one. Numerical coordinates
keep their original scales and full family denominators.

```sh
cargo build --release --manifest-path grammar/Cargo.toml \
  -p grammar-eval --bin slopninja-score-subset
grammar/target/release/slopninja-score-subset \
  --model data/portable-scale/models/metric-seed-0.json export \
  --export MAXIMAL_TEST_EXPORT --subset all_m2 \
  --expected-scores SAVED_TREATMENT_SCORES.json \
  --out data/portable-scale/replay-seed-0-test-1
```

The replay checks catalog and subset hashes, support-ranked membership, named
coordinate transforms, parser and feature identities, array integrity, and query
and author order. Training exports use the saved profile excluding the query's
whole date only for that query's own author. Validation and test exports use
full training profiles.

For every pair, the command also expands the compact model to the maximal
catalog with inactive multipliers set to one. It compares the resulting family
distances and score against compact inference, then checks the saved Python
score within an absolute tolerance of `1e-10`. Candidate ordering and tie groups
must agree exactly. Outputs include binary logits and a manifest binding the
model, export, reference scores, executed source and binary.

For operational scoring of two full writing profiles, use the existing
[`slopninja-score profiles` command](coordinate-inference.md). It accepts these
compact model artifacts through the same dimension-generic `Metric` API. Supply
the required identity envelopes, every family, and every original numerical
axis. A score measures proximity under the learned geometry; it does not
certify meaning preservation or writing quality.

The completed replay covers all six scale-study test galleries for all three
selected seeds: 361,800 candidate scores and 3,618 rankings, representing 1,206
distinct query texts. Every candidate ordering and tie group matched Python.
The largest absolute logit difference was `5.684341886080802e-14`. Compact and
maximal-catalog inference produced identical scores and adjusted family
distances across every comparison.

Ten focused Rust tests cover inherited metric validation, altered subset and
catalog bindings, support-policy changes, original numerical denominators,
unselected and unseen mass, training target replacement, exact ties, and
corrupted arrays. Full workspace formatting, tests and Clippy passed. The local
models, source and binary snapshots, command script, logs, and
`replay-receipt-v1.json` remain under ignored
`data/author-corpora/blog-authorship-2004/scale-coordinate-inference-v1/`.
