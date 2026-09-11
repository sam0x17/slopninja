"""Explicit frozen-artifact evaluation; no training or calibration updates."""

import argparse
from pathlib import Path

from common import configure_cpu, load_artifact, load_partition, logits_for, metrics, sha256, write_json


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--artifact", required=True, type=Path)
    p.add_argument("--test-jsonl", required=True, type=Path)
    p.add_argument("--output", required=True, type=Path)
    p.add_argument("--acknowledge-final-test", action="store_true", required=True,
                   help="Confirm that model selection and calibration are frozen before opening this test")
    p.add_argument("--batch-size", type=int, default=4)
    args = p.parse_args()
    if args.output.exists() or args.batch_size < 1:
        p.error("output must be new and batch size positive")
    configure_cpu()
    manifest, tokenizer, model, temperature = load_artifact(args.artifact, "evaluate.py")
    rows = load_partition(args.test_jsonl, "test", tokenizer, manifest["max_tokens"])
    logits = logits_for(model, tokenizer, rows, args.batch_size, "cpu")
    report = {"artifact_id": manifest["artifact_id"], "test_sha256": sha256(args.test_jsonl),
              "source_groups": len({r["source_group"] for r in rows}),
              "metrics": metrics(logits, [r["label"] for r in rows], temperature),
              "limitations": "Point estimates only; source-cluster uncertainty and preregistered false-positive operating points require the Rust evaluation stage."}
    write_json(args.output, report)


if __name__ == "__main__":
    main()
