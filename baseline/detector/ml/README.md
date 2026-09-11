# Encoder comparison

This is the optional PyTorch path for the reference origin detector. Rust owns
acquisition, rights review, corpus admission, grouped splitting and exports.
These scripts load the admitted shards for ML; they do not acquire or silently
filter corpus records. The first runtime checks used synthetic controls. No
qualified detector, final evaluation or Pangram parity result follows from them.

The candidate is [ModernBERT-base](https://huggingface.co/answerdotai/ModernBERT-base),
an encoder released under Apache-2.0. We pin revision
`8949b909ec900327062f0ebf497f51aef5e6f0c8`. With the new three-class head it has
149,607,171 parameters. The upstream checkpoint is a masked-language model;
the classification head requires training. See the
[official Transformers implementation](https://huggingface.co/docs/transformers/v4.57.6/en/model_doc/modernbert).

## Install and prepare weights

From the repository root, use an isolated Python 3.12 environment:

```sh
python3.12 -m venv baseline/detector/.venv
baseline/detector/.venv/bin/python -m pip install -r baseline/detector/ml/requirements.lock
baseline/detector/.venv/bin/python baseline/detector/ml/fetch_checkpoint.py \
  --output data/baseline-detector/model-cache/modernbert-base
```

The dependency closure was installed and checked on macOS arm64, Python 3.12.14.
`fetch_checkpoint.py` is the explicit network step. It downloads the pinned
checkpoint, tokenizer and notices, records their SHA256 hashes and preserves the
upstream license declaration. Training and inference load local files with
remote code disabled. Inference makes no provider calls.

## Train and calibrate

Export separate `train`, `development`, `calibration` and sealed `test` JSONL
shards using the Rust dataset commands. Each ML row carries the versioned record
schema, `id`, `source_group`, `split`, `origin`, `evidence`, `text` and
`text_sha256`; other provenance fields remain in the Rust export and its hash.
All three origin classes must occur in each of the first three shards. The ML
handoff rechecks text hashes, split labels and cross-shard source/text overlap.
Rust's full admission audit remains required, including the sealed test's
independence and text permissions.

Before training, inspect token lengths without discarding any records:

```sh
baseline/detector/.venv/bin/python baseline/detector/ml/audit.py \
  --checkpoint data/baseline-detector/model-cache/modernbert-base \
  --train-jsonl data/baseline-detector/export/train.jsonl \
  --development-jsonl data/baseline-detector/export/development.jsonl \
  --calibration-jsonl data/baseline-detector/export/calibration.jsonl \
  --test-jsonl data/baseline-detector/export/test.jsonl \
  --max-tokens 2048 --output data/baseline-detector/export/token-audit.json
```

The audit reports class/group counts, evidence counts, token-length percentiles
and every overlength record ID. It checks hashes and partition separation. Test
inspection is limited to these counts and token lengths: no encoder weights
are loaded, no scores are calculated and no parameters are fitted. Omit
`--test-jsonl` when even that metadata should remain sealed. An overlength report
never authorizes dropping or modifying the listed records.

```sh
baseline/detector/.venv/bin/python baseline/detector/ml/train.py \
  --checkpoint data/baseline-detector/model-cache/modernbert-base \
  --train-jsonl data/baseline-detector/export/train.jsonl \
  --development-jsonl data/baseline-detector/export/development.jsonl \
  --calibration-jsonl data/baseline-detector/export/calibration.jsonl \
  --output data/baseline-detector/runs/encoder-001 \
  --device mps --max-tokens 2048 --batch-size 4 --epochs 3 --seed 17
```

The model trains only on `train`; the best epoch minimizes unweighted development
log loss. A scalar temperature then minimizes unweighted calibration log loss
on a deterministic bounded grid. Calibration uses the actual calibration class
mixture without resampling or training class weights. Its class counts, priors
and any boundary optimum are recorded. This does not establish calibrated
probabilities for a different deployment population.

`--selection-objective human_model` changes checkpoint selection to binary
Development log loss on human and fully model-written rows, using the combined
model/mixed probability. Mixed-origin rows still contribute to training and
calibration but do not enter this selection metric. With `--require-class-coverage`,
both binary classes need positive recall at the fixed 0.5 diagnostic boundary.
The default `three_class` objective retains the original selection rule. The
[v5 target amendment](../ENCODER_NARRATIVE_V5_TARGET.md) declares the new objective
before fitting and keeps the final operating thresholds separate from this gate.

`--class-weights inverse_frequency` computes weights from training counts only.
The default is unweighted training. `--freeze-encoder` is a classification-head
control. `--synthetic-smoke-only` explicitly marks artifacts trained using
synthetic fixtures; the script rejects these fixtures without that flag.
Hyperparameter sweeps and selection across seeds remain Rust orchestration work.

The token cap includes special tokens. A shard containing a longer record fails
with its token count; no text is truncated, dropped or split into undocumented
windows. This first runtime supports rejection only. A separate trained and
calibrated aggregation policy would be needed for longer documents.

## Execute the immutable bundle

An artifact contains safetensors, tokenizer/configuration, calibration, training
metadata, upstream notices, a machine-readable model card and exact copies of
the inference, training and acquisition scripts.
`manifest.json` binds those files, label order, preprocessing, reference runtime
and numerical policy to a SHA256 identity. Inference verifies hashes and numeric
dependency versions before loading. Use the bundled runner so later source edits
cannot change an existing model's execution.

```sh
baseline/detector/.venv/bin/python \
  data/baseline-detector/runs/encoder-001/runner/infer.py \
  --artifact data/baseline-detector/runs/encoder-001 < requests.jsonl
```

A request is `{"id":"request-1","text":"Exact text to analyze."}`. Only `id`
and `text` are accepted. Results return `status`, the artifact hash, token count
and probabilities ordered as `human_only`, `model_only`, `mixed`. The combined
model/assistance probability is the sum of the last two. Empty or overlength
input returns `unsupported_input` and null probabilities.

The reference path uses CPU FP32 logits, FP64 probability postprocessing, eager
attention, one CPU thread and one request per inference batch. The initial
absolute probability tolerance is `1e-6`; the canonical CPU result resolves a
downstream threshold disagreement. This runner returns probabilities without a
thresholded human/AI verdict. Each bundle includes synthetic input/output vectors;
the loader verifies their token counts and probability agreement before serving
requests. It also enforces the Python 3.12 and numeric dependency contract.
MPS is an explicit training option. Broader CPU/MPS
numerical qualification and rare-error evaluation remain pending.

The final test has a separate command with an explicit acknowledgement:

```sh
baseline/detector/.venv/bin/python \
  data/baseline-detector/runs/encoder-001/runner/evaluate.py \
  --artifact data/baseline-detector/runs/encoder-001 \
  --test-jsonl data/baseline-detector/export/test.jsonl \
  --output data/baseline-detector/runs/encoder-001-final-test.json \
  --probabilities-output data/baseline-detector/runs/encoder-001-test-probabilities.jsonl \
  --acknowledge-final-test
```

That command writes point estimates for log loss, multiclass Brier score,
accuracy, confusion and per-class precision/recall. It cannot change the model
or its temperature. Keep the output outside the immutable artifact directory.
The optional probability export contains `id`, the exact UTF-8 `text_sha256`,
`artifact_id`, `classes`, `probabilities` and `status: "ok"` for every row.
It uses the frozen CPU reference contract: one request per batch and FP64
softmax. Overlength or invalid input fails the export without dropping records.

Use `--calibration-jsonl` in place of `--test-jsonl` to export probabilities for
the Rust operating-point fit, with distinct output paths. No test acknowledgement
is needed for that command. It requires the exact calibration shard hash recorded
in the bundle and applies the already fitted temperature; it performs no fitting.
The test command requires `--acknowledge-final-test` even when only exporting
probabilities. Each report records its input and probability-export hashes.

Source-cluster uncertainty, prespecified false-positive operating points,
generator/register slices and paired Pangram comparisons remain part of the
larger Rust evaluation stage. Reusing a test to guide another model choice
requires retiring that test and reserving fresh confirmation data.

## Data release terms

When training on the Wikipedia ShareAlike lane, pass `--data-rights` with the
`data_rights.json` file from Rust's shard export. The trainer verifies its shard
hashes and source-notice coverage, then bundles the attribution and declared
fine-tuned weight license. Omitting or mismatching this file stops fitting.
The base checkpoint's license remains in `UPSTREAM_LICENSE` for that lane.
See the [distribution policy](../SHARE_ALIKE_POLICY.md).

## Runtime spike

```sh
baseline/detector/.venv/bin/python baseline/detector/ml/smoke.py \
  --checkpoint data/baseline-detector/model-cache/modernbert-base \
  --output data/baseline-detector/runtime-smoke-001 --device mps --try-export
```

This command performs one gradient step on two synthetic strings with arbitrary
labels, exports a clearly marked control bundle and checks safetensors reload.
An optional static-shape `torch.export` check records success or its exact
failure. Its scores have no detector meaning. The static graph establishes
export feasibility for that input shape; a general Rust/ONNX runner is not yet
implemented. The pinned PyTorch runner is the current inference reference.
