# Applying a frozen coordinate catalog to additional authors

`unslop-export-fixed-panels` supplies additional training panels and test
galleries for the training-author-scale experiment. It applies the original
7,353-coordinate frontier catalog and records the fixed 3,557-coordinate
`all_m2` subset. The exporter checks both catalog and subset hashes. It does not
fit support counts, add coordinates, or change numerical scales.

Run from the project root after freezing the experiment protocol:

```sh
cargo build --release --manifest-path grammar/Cargo.toml \
  -p grammar-eval --bin unslop-export-fixed-panels
grammar/target/release/unslop-export-fixed-panels PREPARED_EVALUATION_DIR \
  --export AUTHOR_EXPORT_DIR --cache CACHE_DIR --space ORIGINAL_SPACE.json \
  --split train --catalog ORIGINAL_FRONTIER_TRAIN/catalog.json \
  --protocol FROZEN_SCALE_PROTOCOL.json --out data/fixed-panels/train
```

Use a new output directory. For a prepared test gallery, use `--split test`.
Reuse the existing original training and validation coordinate exports
directly. Additional training panels must retain exactly 100 eligible authors
and must be disjoint from the authors who fitted the original catalog.

The complete existing preparations are `vector-evaluation-neural-v1` for
`authors-replication-v1`, and `vector-evaluation-spline-confirmation-v1` for
`authors-spline-confirmation-v1`. The similarly named
`vector-evaluation-replication-v1` preparation is incomplete.

The loader checks source manifests, text hashes, exact and normalized duplicate
hashes, cached annotations, re-extracted features, parser identity, and the
complete saved training-source bindings. Older completed preparations need not
have a separate `author-exclusions.json`: eligibility comes from each post's
entry in the hash-bound annotation audit. Applying a frozen catalog does not
require the new panel's training sources to be the original space's fitting
sources. Its source and eligibility checks otherwise match the earlier
frontier exporter.

Training author profiles average posts within a date and then average dates.
For a query's true author, exclude that entire query date before averaging.
Other candidate profiles use their complete training posts. Query loss weights
give equal weight to each author, then each date, then each post within a date.
Because each training panel has 100 authors, averaging panel losses equally
also weights all training authors equally.

The existing `slopninja-coordinate-frontier-export-v1` array schema is retained:
query coordinates, full author coordinates, training-only targets with the
query date excluded, and complete unweighted fourteen-family distances. The
maximal catalog is copied byte for byte. Consumers gather `all_m2` columns using
its existing index map and preserve every other contribution in the complete
family distances.

The manifest adds `role` (`training_panel` or `test_gallery`),
`training_catalog_policy` (`apply_frozen_catalog_without_fitting`),
`catalog_fit_performed: false`, `fixed_subset_name`, and `fixed_subset`.
`catalog.training_authors` continues to describe the original support-fitting
authors; the export's `authors` and source audit describe the panel being
applied. All derived arrays, manifests and source snapshots remain in ignored
local `data/`.
