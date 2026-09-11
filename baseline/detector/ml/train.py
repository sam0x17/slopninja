"""Fine-tune the three-class encoder from separately exported, admitted Rust shards."""

import argparse
import json
from pathlib import Path
import random
import tempfile
import time

import torch
from safetensors.torch import load_model, save_model

from common import (LABELS, batch_tensors, check_partition_separation, class_counts,
                    configure_cpu, fit_temperature, load_checkpoint, load_partition,
                    logits_for, metrics, package_artifact, sha256, software)


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--checkpoint", required=True, type=Path)
    for name in ["train", "development", "calibration"]:
        p.add_argument(f"--{name}-jsonl", required=True, type=Path)
    p.add_argument("--output", required=True, type=Path)
    p.add_argument("--device", choices=["cpu", "mps"], default="cpu")
    p.add_argument("--max-tokens", type=int, default=1024)
    p.add_argument("--batch-size", type=int, default=4)
    p.add_argument("--epochs", type=int, default=3)
    p.add_argument("--learning-rate", type=float, default=2e-5)
    p.add_argument("--weight-decay", type=float, default=0.01)
    p.add_argument("--class-weights", choices=["none", "inverse_frequency"], default="none")
    p.add_argument("--seed", type=int, default=17)
    p.add_argument("--freeze-encoder", action="store_true", help="Train the classification layers only as a control")
    p.add_argument("--synthetic-smoke-only", action="store_true", help="Mark output as an unvalidated synthetic control")
    args = p.parse_args()
    if args.output.exists():
        p.error("output already exists; use a new immutable run directory")
    if min(args.batch_size, args.epochs, args.max_tokens) < 1 or args.max_tokens > 8192:
        p.error("positive batch/epoch/token limits required; max tokens cannot exceed 8192")
    if args.learning_rate <= 0 or args.weight_decay < 0:
        p.error("invalid optimizer parameters")
    if args.device == "mps" and not torch.backends.mps.is_available():
        p.error("MPS is unavailable")
    configure_cpu()
    torch.manual_seed(args.seed)
    rng = random.Random(args.seed)
    tokenizer, model = load_checkpoint(args.checkpoint, args.seed)
    partitions = {name: load_partition(getattr(args, f"{name}_jsonl"), name, tokenizer, args.max_tokens)
                  for name in ["train", "development", "calibration"]}
    check_partition_separation(partitions)
    if any(row["evidence"] == "synthetic_fixture" for rows in partitions.values() for row in rows) and not args.synthetic_smoke_only:
        raise ValueError("synthetic fixtures require --synthetic-smoke-only")
    counts = {name: class_counts(rows) for name, rows in partitions.items()}
    if any(n == 0 for values in counts.values() for n in values):
        raise ValueError("all three classes must occur in training, development and calibration")
    if args.freeze_encoder:
        for parameter in model.base_model.parameters():
            parameter.requires_grad_(False)
    model.to(args.device)
    parameters = [parameter for parameter in model.parameters() if parameter.requires_grad]
    optimizer = torch.optim.AdamW(parameters, lr=args.learning_rate, weight_decay=args.weight_decay)
    weights = None
    if args.class_weights == "inverse_frequency":
        weights = torch.tensor([sum(counts["train"]) / (3 * n) for n in counts["train"]], device=args.device)
    epochs = []
    best_loss = float("inf")
    best_epoch = None
    started = time.monotonic()
    args.output.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix=".encoder-selection-", dir=args.output.parent) as temporary:
        selected = Path(temporary) / "selected.safetensors"
        for epoch in range(1, args.epochs + 1):
            order = list(partitions["train"])
            rng.shuffle(order)
            model.train()
            loss_total = 0.0
            epoch_start = time.monotonic()
            for start in range(0, len(order), args.batch_size):
                rows = order[start:start + args.batch_size]
                inputs = batch_tensors(rows, tokenizer, args.device)
                labels = torch.tensor([r["label"] for r in rows], device=args.device)
                optimizer.zero_grad(set_to_none=True)
                logits = model(**inputs).logits
                loss = torch.nn.functional.cross_entropy(logits, labels, weight=weights)
                if not torch.isfinite(loss):
                    raise ValueError("non-finite training loss")
                loss.backward()
                torch.nn.utils.clip_grad_norm_(parameters, 1.0, error_if_nonfinite=True)
                optimizer.step()
                loss_total += float(loss.detach().cpu()) * len(rows)
            development_logits = logits_for(model, tokenizer, partitions["development"], args.batch_size, args.device)
            development = metrics(development_logits, [r["label"] for r in partitions["development"]])
            summary = {"epoch": epoch, "training_loss": loss_total / len(order), "development": development,
                       "elapsed_seconds": time.monotonic() - epoch_start}
            epochs.append(summary)
            print(json.dumps(summary), flush=True)
            if development["log_loss"] < best_loss:
                best_loss = development["log_loss"]
                best_epoch = epoch
                save_model(model, selected)
        # Model choice is complete before calibration. Final-test input has no CLI argument here.
        load_model(model, selected, strict=True, device=args.device)
        model.cpu().eval()
        calibration_logits = logits_for(model, tokenizer, partitions["calibration"], args.batch_size, "cpu")
        calibration_labels = [r["label"] for r in partitions["calibration"]]
        calibration = fit_temperature(calibration_logits, calibration_labels)
        calibration["before"] = metrics(calibration_logits, calibration_labels)
        calibration["after"] = metrics(calibration_logits, calibration_labels, calibration["temperature"])
        training = {
            "status": "synthetic_smoke_only_not_a_detector_release" if args.synthetic_smoke_only else "unqualified_research_candidate",
            "checkpoint_pin_sha256": sha256(args.checkpoint / "checkpoint.json"),
            "software": software(), "device": args.device, "dtype": "float32", "seed": args.seed,
            "optimizer": "AdamW", "learning_rate": args.learning_rate, "weight_decay": args.weight_decay,
            "gradient_norm_limit": 1.0, "batch_size": args.batch_size, "epochs_requested": args.epochs,
            "class_weight_policy": args.class_weights, "class_weights": weights.cpu().tolist() if weights is not None else None,
            "freeze_encoder": args.freeze_encoder, "trainable_parameters": sum(p.numel() for p in parameters),
            "parameters": sum(p.numel() for p in model.parameters()), "class_counts": counts,
            "source_group_counts": {name: len({r["source_group"] for r in rows}) for name, rows in partitions.items()},
            "evidence_counts": {name: {evidence: sum(r["evidence"] == evidence for r in rows)
                                       for evidence in sorted({r["evidence"] for r in rows})}
                                for name, rows in partitions.items()},
            "partition_sha256": {name: sha256(getattr(args, f"{name}_jsonl")) for name in partitions},
            "selection_metric": "development_unweighted_log_loss", "selected_epoch": best_epoch,
            "epoch_history": epochs, "elapsed_seconds": time.monotonic() - started,
            "final_test_opened": False,
        }
        manifest = package_artifact(args.output, tokenizer, model, calibration, training, args.max_tokens, args.checkpoint)
        print(json.dumps({"artifact": str(args.output), "artifact_id": manifest["artifact_id"], "status": training["status"]}))


if __name__ == "__main__":
    main()
