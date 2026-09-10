# Rust inference for coordinate weights

`slop_ninja-score` runs the frozen positive coordinate metric without PyTorch.
The reusable `Metric` type lives in
[`coordinate_inference.rs`](../grammar/crates/grammar-eval/src/coordinate_inference.rs).
It accepts complete family distances and selected coordinates through
`score_pair`, or raw family profiles through `transform_profile` and
`score_profiles`.

For each family, inference computes:

```text
adjusted_distance = complete_distance
                  + sum(expm1(log_multiplier[j]) * (query[j] - target[j])^2)
logit = -sum(family_weight[f] * spline(adjusted_distance[f])) / temperature
```

Larger logits rank closer candidates first. Unselected and unseen contributions
remain in the complete distance with multiplier one. Categorical coordinates
are `sqrt(probability / 2)`. Numerical coordinates are
`value / (original_scale * sqrt(full_original_family_axis_count))`. Average raw
probabilities before transforming an author profile.
The final spline score is a ranking function; its monotonicity does not
establish the triangle inequality or permit ordinary Euclidean nearest-neighbor
indexing of the complete score.

The portable JSON schema, `slopninja-positive-coordinate-metric-v1`, contains
named coordinate transforms and positive multipliers, the full original numeric
axis lists and scales, ordered family weights, common spline knots and positive
slopes, and temperature. It binds the original space, catalog, selection and
both source checkpoints by SHA256. Model validation rejects incompatible
dimensions, invalid parameters and incomplete numeric geometry.

The thin Python adapter reads the three selected PyTorch checkpoints from the
original coordinate experiment. Run from the project root with existing frozen
artifact paths and a fresh output directory:

```sh
.venv/bin/python grammar/crates/grammar-eval/python/export_coordinate_metric.py \
  --selection COORDINATE_FIT/selection.json --protocol COORDINATE_PROTOCOL.json \
  --catalog ORIGINAL_COORDINATE_EXPORT/catalog.json --space ORIGINAL_SPACE.json \
  --baseline-fit FAMILY_SPLINE_FIT --out data/portable-metric/models

cargo build --release --manifest-path grammar/Cargo.toml \
  -p grammar-eval --bin slop_ninja-score
grammar/target/release/slop_ninja-score \
  --model data/portable-metric/models/metric-seed-0.json export \
  --export ORIGINAL_COORDINATE_EXPORT --out data/portable-metric/scores \
  --expected-scores SAVED_PYTHON_SCORES.json
```

The current export command consumes `slopninja-coordinate-export-v1`. It checks
the export's identities, original-space binding, exact coordinate catalog,
array hashes and shapes. Training exports use the saved target excluding the
query's entire date for the true-author candidate. Outputs include float64
logits and a manifest binding the model, input export and executed scorer.
`--expected-scores` additionally requires every candidate ordering and tie group
to agree, with maximum absolute logit error at most `1e-10`.

For direct profile scoring, use `profiles --query QUERY.json --target TARGET.json`
after `--model MODEL.json`. Each file is an identity envelope:

```json
{
  "schema": "slopninja-coordinate-profile-v1",
  "source_space_id": "the model's source space ID",
  "feature_schema": "the model's feature schema",
  "parser_identity": "the model's parser identity",
  "profile": {"family_name": {"feature_name": 0.25}}
}
```

Provide every family and every original numeric axis. Categorical profiles can
include new words or constructions, with nonnegative probabilities summing to
one per family. The CLI checks envelope identities. Callers using bare Rust
`Profile` values must bind parser and feature provenance themselves, since that
type contains only family names and values.

The initial verification covers the earlier 1,864-coordinate incumbent: three
test galleries and three inherited seeds, totaling 177,000 candidate scores.
Every ordering and tie group matched Python; the largest absolute score error
was `5.684341886080802e-14`. Synthetic tests cover unseen words, unselected
numeric contributions, missing numeric axes, invalid parameters, spline knot
continuity, and corrupted binary inputs. Full workspace formatting, tests and
Clippy passed. Local models, source snapshots and the replay receipt are under
ignored `data/author-corpora/blog-authorship-2004/coordinate-inference-v1/`.
