#!/usr/bin/env python3
"""Verify the dated occurrence inventory and preservation witnesses, read-only."""
import hashlib
import json
from pathlib import Path
import subprocess
import sys


ROOT = Path(__file__).resolve().parents[1]
INVENTORY = ROOT / "docs/HOLLOW_GROVE_ACTIVE_GEOGRAPHY_OCCURRENCES_2026-10-27.json"


def main():
    inventory = json.loads(INVENTORY.read_text())
    scan = subprocess.run(
        ["rg", "--json", "--text", "--hidden", "--no-ignore", "-i", "-g", "!.git/**",
         "lazerhorn", "."], cwd=ROOT, capture_output=True, text=True, check=False,
    )
    if scan.returncode not in (0, 1) or scan.stderr:
        raise ValueError(f"incomplete repository scan: {scan.stderr}")
    actual = {}
    for line in scan.stdout.splitlines():
        record = json.loads(line)
        if record["type"] == "match":
            data = record["data"]
            path = data["path"]["text"].removeprefix("./")
            actual.setdefault(path, []).append({
                "line": data["line_number"], "count": len(data["submatches"]),
                "byte_offsets": [data["absolute_offset"] + m["start"]
                                 for m in data["submatches"]],
            })
    expected = {
        item["path"]: item["after"] for item in inventory["files"] if item["after"]
    }
    if actual != expected:
        changed = sorted(p for p in actual.keys() | expected.keys()
                         if actual.get(p) != expected.get(p))
        raise ValueError(f"occurrence inventory drift: {changed}")
    for item in inventory["files"]:
        if item["after"] and item["after_category"] == "1-active":
            raise ValueError(f"active occurrence remains: {item['path']}")
    for path, digest in inventory["protected_sha256"].items():
        if hashlib.sha256((ROOT / path).read_bytes()).hexdigest() != digest:
            raise ValueError(f"protected source/archive changed: {path}")
    source = ROOT / "CURRENT_SYNTHESIS_HOLLOW_GROVE_WORLD_CONTEXT_v0.1.0.md"
    mirror = ROOT / "artifacts/current_synthesis_world_context.md"
    if source.read_bytes() != mirror.read_bytes():
        raise ValueError("world context mirror differs")
    print(f"PASS: {len(actual)} matching paths (including build caches); "
          "every occurrence classified; no unclassified source occurrence")
    print(f"PASS: {len(inventory['protected_sha256'])} protected files unchanged")
    print("SOURCE_CONFLICT remains: Compromise accession clause; no law or succession activated")


if __name__ == "__main__":
    try:
        main()
    except (ValueError, OSError, KeyError) as error:
        print(f"FAIL: {error}", file=sys.stderr)
        sys.exit(1)
