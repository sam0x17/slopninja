"""Audit exported shard lengths/counts without model execution, fitting or filtering."""

import argparse
import json
import math
from pathlib import Path

from common import (LABELS, check_partition_separation, class_counts, load_checkpoint_tokenizer,
                    load_partition, sha256, write_json)


def length_distribution(rows):
    lengths = sorted(len(row["ids"]) for row in rows)
    return {
        "min": lengths[0], "max": lengths[-1], "mean": sum(lengths) / len(lengths),
        "nearest_rank_percentiles": {str(p): lengths[max(0, math.ceil(p * len(lengths) / 100) - 1)]
                                     for p in [25, 50, 75, 90, 95, 99]},
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--checkpoint", required=True, type=Path)
    for split in ["train", "development", "calibration", "test"]:
        parser.add_argument(f"--{split}-jsonl", type=Path)
    parser.add_argument("--max-tokens", type=int, default=1024)
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()
    if args.output.exists() or not 1 <= args.max_tokens <= 8192:
        parser.error("output must be new; max tokens must be between 1 and 8192")
    paths = {split: getattr(args, f"{split}_jsonl") for split in ["train", "development", "calibration", "test"]
             if getattr(args, f"{split}_jsonl") is not None}
    if not paths:
        parser.error("provide at least one partition")
    tokenizer = load_checkpoint_tokenizer(args.checkpoint)
    # The uncapped loader retains every record. Overlength IDs are reported below.
    partitions = {split: load_partition(path, split, tokenizer, None) for split, path in paths.items()}
    check_partition_separation(partitions)
    shards = {}
    for split, rows in partitions.items():
        overflow = [{"id": row["id"], "token_count": len(row["ids"])} for row in rows if len(row["ids"]) > args.max_tokens]
        shards[split] = {
            "sha256": sha256(paths[split]), "records": len(rows), "source_groups": len({r["source_group"] for r in rows}),
            "class_counts": dict(zip(LABELS, class_counts(rows))),
            "class_source_group_counts": {label: len({r["source_group"] for r in rows if r["label"] == i})
                                          for i, label in enumerate(LABELS)},
            "evidence_counts": {evidence: sum(r["evidence"] == evidence for r in rows)
                                for evidence in sorted({r["evidence"] for r in rows})},
            "token_lengths_including_special_tokens": length_distribution(rows),
            "token_lengths_by_class": {label: length_distribution(class_rows) if class_rows else None
                                       for i, label in enumerate(LABELS)
                                       for class_rows in [[r for r in rows if r["label"] == i]]},
            "over_limit_count": len(overflow), "over_limit_records": overflow,
        }
    report = {"schema": "slop_ninja.encoder_shard_audit.v1", "max_tokens": args.max_tokens,
              "checkpoint_pin_sha256": sha256(args.checkpoint / "checkpoint.json"), "shards": shards,
              "cross_partition_id_group_text_overlap": False, "records_dropped": 0,
              "model_executed": False, "model_fitted": False, "calibration_fitted": False,
              "test_inspection": "counts_and_token_lengths_only" if "test" in paths else "not_opened",
              "fits_configured_token_limit": all(shard["over_limit_count"] == 0 for shard in shards.values())}
    write_json(args.output, report)
    print(json.dumps({"audit": str(args.output), "records": sum(len(rows) for rows in partitions.values()),
                      "fits_configured_token_limit": report["fits_configured_token_limit"]}))


if __name__ == "__main__":
    main()
