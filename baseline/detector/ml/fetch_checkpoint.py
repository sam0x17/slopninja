"""Explicit network-only preparation for the pinned public encoder checkpoint."""

import argparse
import hashlib
import json
from pathlib import Path
import urllib.request

from huggingface_hub import snapshot_download

MODEL_ID = "answerdotai/ModernBERT-base"
REVISION = "8949b909ec900327062f0ebf497f51aef5e6f0c8"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()
    snapshot_download(MODEL_ID, revision=REVISION, local_dir=args.output, allow_patterns=[
        "config.json", "model.safetensors", "tokenizer.json", "tokenizer_config.json",
        "special_tokens_map.json", "README.md", "LICENSE",
    ])
    (args.output / "UPSTREAM_README.md").write_bytes((args.output / "README.md").read_bytes())
    # The model card declares Apache-2.0 but this revision has no separate license file.
    license_path = args.output / "LICENSE"
    if not license_path.exists():
        with urllib.request.urlopen("https://www.apache.org/licenses/LICENSE-2.0.txt", timeout=30) as response:
            license_path.write_bytes(response.read())
    files = {p.name: hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(args.output.iterdir())
             if p.is_file() and p.name != "checkpoint.json"}
    pin = {"model_id": MODEL_ID, "revision": REVISION, "weight_license": "Apache-2.0", "files": files}
    (args.output / "checkpoint.json").write_text(json.dumps(pin, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"checkpoint": str(args.output), "revision": REVISION, "files": files}))


if __name__ == "__main__":
    main()
