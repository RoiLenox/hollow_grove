#!/usr/bin/env python3
"""Check current source signage independently of dated build-cache evidence."""
import collections
import hashlib
import json
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
POLICY = "contracts/geography/active-source.v1.json"
HISTORY = "docs/HOLLOW_GROVE_ACTIVE_GEOGRAPHY_OCCURRENCES_2026-10-27.json"
CONTROL_PATHS = {POLICY, "tools/verify-active-geography.py", "tools/test_active_geography.py"}
EXCLUDED_PARTS = {".git", "target", "dist", "release", "__pycache__", ".godot"}
RETIRED = "lazerhorn"


def safe_path(root, relative):
    path = root / relative
    if Path(relative).is_absolute() or ".." in Path(relative).parts or not path.resolve().is_relative_to(root):
        raise ValueError(f"unsafe source path: {relative}")
    return path


def source_paths(root):
    if (root / ".git").exists():
        paths = subprocess.check_output(
            ["git", "-C", str(root), "ls-files", "--cached", "--others", "--exclude-standard", "-z"],
            text=True,
        ).split("\0")
    else:
        # Standalone clean source bundles and the installed proof inventory.
        paths = json.loads((root / "SOURCE-MANIFEST.json").read_text())["files"]
    return sorted({p for p in paths if p and not EXCLUDED_PARTS.intersection(Path(p).parts)})


def occurrences(root, paths):
    actual = {}
    for relative in paths:
        if relative in CONTROL_PATHS:
            continue
        path = safe_path(root, relative)
        if not path.is_file():
            raise ValueError(f"missing source input: {relative}")
        data = path.read_bytes()
        if b"\0" in data:
            continue
        try:
            text = data.decode("utf-8")
        except UnicodeDecodeError:
            continue
        lines = collections.Counter(line.strip() for line in text.splitlines() if RETIRED in line.lower())
        if lines:
            actual[relative] = dict(sorted(lines.items()))
    return actual


def verify(root):
    root = root.resolve()
    policy = json.loads((root / POLICY).read_text())
    if policy["schema_version"] != 1:
        raise ValueError("unsupported geography policy version")
    history_bytes = (root / HISTORY).read_bytes()
    if hashlib.sha256(history_bytes).hexdigest() != policy["historical_inventory_sha256"]:
        raise ValueError("dated geography inventory changed")
    history = json.loads(history_bytes)
    expected = {}
    for relative, entry in policy["allowed_occurrences"].items():
        if entry["category"] not in ("protected-source-conflict", "historical", "compatibility-verification"):
            raise ValueError(f"unreviewed occurrence classification: {relative}")
        if relative in CONTROL_PATHS or EXCLUDED_PARTS.intersection(Path(relative).parts):
            raise ValueError(f"invalid occurrence exception: {relative}")
        expected[relative] = entry["lines"]
    paths = source_paths(root)
    actual = occurrences(root, paths)
    if actual != expected:
        changed = sorted(p for p in actual.keys() | expected.keys() if actual.get(p) != expected.get(p))
        raise ValueError(f"unreviewed source occurrence drift: {changed}")
    protected = dict(history["protected_sha256"])
    for relative, update in policy["reviewed_protected_updates"].items():
        if protected.get(relative) != update["previous_sha256"] or not update["review"]:
            raise ValueError(f"invalid protected-file review: {relative}")
        protected[relative] = update["sha256"]
    for relative, digest in protected.items():
        if hashlib.sha256(safe_path(root, relative).read_bytes()).hexdigest() != digest:
            raise ValueError(f"protected source/archive changed: {relative}")
    source = root / "CURRENT_SYNTHESIS_HOLLOW_GROVE_WORLD_CONTEXT_v0.1.0.md"
    mirror = root / "artifacts/current_synthesis_world_context.md"
    if source.read_bytes() != mirror.read_bytes():
        raise ValueError("world context mirror differs")
    return {"source_paths": len(paths), "classified_paths": len(actual), "protected_files": len(protected)}


def main():
    result = verify(ROOT)
    print(f"PASS: {result['source_paths']} source inputs; {result['classified_paths']} classified occurrence paths; build caches excluded")
    print(f"PASS: {result['protected_files']} protected files match reviewed baselines; dated inventory unchanged")
    print("SOURCE_CONFLICT remains: Compromise accession clause; no law or succession activated")


if __name__ == "__main__":
    try:
        main()
    except (ValueError, OSError, KeyError, subprocess.CalledProcessError) as error:
        print(f"FAIL: {error}", file=sys.stderr)
        sys.exit(1)
