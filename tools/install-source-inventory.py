#!/usr/bin/env python3
"""Install only source-manifest inputs, excluding check/build byproducts."""
import argparse
import hashlib
import json
from pathlib import Path
import shutil


def install(root, destination):
    root = root.resolve()
    manifest_path = root / "SOURCE-MANIFEST.json"
    manifest = json.loads(manifest_path.read_text())
    sources = []
    for relative, expected in manifest["files"].items():
        path = root / relative
        if Path(relative).is_absolute() or ".." in Path(relative).parts or not path.resolve().is_relative_to(root):
            raise ValueError(f"unsafe source path: {relative}")
        if not path.is_file() or hashlib.sha256(path.read_bytes()).hexdigest() != expected:
            raise ValueError(f"source differs from manifest: {relative}")
        sources.append((relative, path))
    destination.mkdir(parents=True, exist_ok=False)
    for relative, path in sources:
        target = destination / relative
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(path, target)
        target.chmod(0o644)
    shutil.copyfile(manifest_path, destination / "SOURCE-MANIFEST.json")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source-root", type=Path, default=Path(__file__).resolve().parents[1])
    parser.add_argument("--destination", type=Path, required=True)
    args = parser.parse_args()
    install(args.source_root, args.destination)


if __name__ == "__main__":
    main()
