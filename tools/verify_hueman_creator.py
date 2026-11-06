#!/usr/bin/env python3
"""Run creator contracts and actual UI save/reload in separate Godot processes."""
from pathlib import Path
import argparse
import os
import shutil
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--godot", default=os.environ.get("GODOT_BIN") or shutil.which("godot4") or shutil.which("godot"))
    args = parser.parse_args()
    if not args.godot:
        parser.error("Godot is required")
    with tempfile.TemporaryDirectory(prefix="hueman-01-check-") as directory:
        for phase in ("contracts", "create", "reload"):
            result = subprocess.run(
                [args.godot, "--headless", "--path", str(ROOT / "hueman_godot"),
                 "--script", "res://tools/hueman_01/verify.gd", "--",
                 f"--profile-dir={directory}", f"--phase={phase}"],
                text=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, timeout=60,
            )
            print(result.stdout, end="")
            # Godot may return zero for script errors; require an explicit completion marker.
            if result.returncode or f"HUEMAN-01 PASS {phase} (" not in result.stdout or "SCRIPT ERROR:" in result.stdout:
                raise SystemExit(f"HUEMAN-01 verification failed during {phase}")
    print("HUEMAN-01 appearance, persistence, restart, and studio checks passed")


if __name__ == "__main__":
    main()
