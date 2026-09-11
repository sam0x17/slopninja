"""Check synthetic tensor execution/export. This produces no qualified detector."""

import argparse
from pathlib import Path
import time

import torch
from transformers import AutoModelForSequenceClassification

from common import (configure_cpu, load_checkpoint, package_artifact, sha256,
                    software, token_ids, write_json)


class LogitsOnly(torch.nn.Module):
    def __init__(self, model):
        super().__init__()
        self.model = model

    def forward(self, input_ids, attention_mask):
        return self.model(input_ids=input_ids, attention_mask=attention_mask).logits


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--checkpoint", required=True, type=Path)
    p.add_argument("--output", required=True, type=Path)
    p.add_argument("--device", choices=["cpu", "mps"], default="cpu")
    p.add_argument("--try-export", action="store_true")
    args = p.parse_args()
    args.output.mkdir(parents=True, exist_ok=False)
    configure_cpu()
    tokenizer, model = load_checkpoint(args.checkpoint, 17)
    texts = ["An editor checked this short paragraph.", "The second sample is a synthetic runtime check."]
    inputs = tokenizer(texts, return_tensors="pt", padding=True)
    model.eval()
    start = time.monotonic()
    with torch.inference_mode():
        initial = model(**inputs).logits
    cpu_forward_seconds = time.monotonic() - start
    model.to(args.device).train()
    optimizer = torch.optim.AdamW(model.parameters(), lr=2e-5)
    # Arbitrary labels exercise the gradient path; they are no production-history evidence.
    synthetic_labels = torch.tensor([0, 1], device=args.device)
    device_inputs = {k: v.to(args.device) for k, v in inputs.items()}
    start = time.monotonic()
    loss = torch.nn.functional.cross_entropy(model(**device_inputs).logits, synthetic_labels)
    loss.backward()
    optimizer.step()
    if args.device == "mps":
        torch.mps.synchronize()
    step_seconds = time.monotonic() - start
    model.cpu().eval()
    with torch.inference_mode():
        reference = model(**inputs).logits
    training = {"status": "synthetic_smoke_only_not_a_detector_release",
                "checkpoint_pin_sha256": sha256(args.checkpoint / "checkpoint.json"), "software": software(),
                "device": args.device, "steps": 1, "parameters": sum(p.numel() for p in model.parameters()),
                "seed": 17, "synthetic_labels_are_arbitrary": True, "final_test_opened": False}
    manifest = package_artifact(args.output / "control_bundle", tokenizer, model,
                                {"temperature": 1.0, "status": "not_fitted_synthetic_control"}, training, 64, args.checkpoint)
    reloaded = AutoModelForSequenceClassification.from_pretrained(
        args.output / "control_bundle" / "classifier", local_files_only=True, trust_remote_code=False,
        attn_implementation="eager", reference_compile=False, dtype=torch.float32,
    ).eval()
    with torch.inference_mode():
        reload_logits = reloaded(**inputs).logits
    reload_difference = float((reference - reload_logits).abs().max())
    if reload_difference > 1e-6:
        raise ValueError("safetensors reload exceeded the CPU tolerance")
    length_rejected = False
    try:
        token_ids(tokenizer, "word " * 200, 64)
    except ValueError:
        length_rejected = True
    if not length_rejected:
        raise ValueError("unsupported input was not rejected")
    report = {**training, "cpu_forward_seconds": cpu_forward_seconds, "optimizer_step_seconds": step_seconds,
              "loss": float(loss.detach().cpu()), "initial_logits": initial.tolist(),
              "reference_logits": reference.tolist(), "safetensors_reload_max_logit_difference": reload_difference,
              "length_limit_rejected": length_rejected, "artifact_id": manifest["artifact_id"],
              "export": {"attempted": args.try_export}}
    if args.try_export:
        try:
            exported = torch.export.export(LogitsOnly(model), (inputs["input_ids"], inputs["attention_mask"]))
            torch.export.save(exported, args.output / "synthetic_static_graph.pt2")
            graph = torch.export.load(args.output / "synthetic_static_graph.pt2").module()
            with torch.inference_mode():
                graph_logits = graph(inputs["input_ids"], inputs["attention_mask"])
            difference = float((reference - graph_logits).abs().max())
            report["export"].update(success=difference <= 1e-6, max_logit_difference=difference,
                                    input_shape=list(inputs["input_ids"].shape),
                                    limitation="Static-shape PyTorch graph smoke only; Rust/ONNX inference is not implemented.")
        except Exception as error:
            report["export"].update(success=False, exception_type=type(error).__name__, error=str(error)[:6000])
    write_json(args.output / "smoke_report.json", report)
    print(args.output / "smoke_report.json")


if __name__ == "__main__":
    main()
