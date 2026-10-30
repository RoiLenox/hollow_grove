#!/usr/bin/env python3
"""Regression coverage for source checks independent of local build output."""
import hashlib
import importlib.util
import json
from pathlib import Path
import subprocess
import tempfile
import unittest

spec = importlib.util.spec_from_file_location("geography", Path(__file__).with_name("verify-active-geography.py"))
geography = importlib.util.module_from_spec(spec)
spec.loader.exec_module(geography)


class SourceGeographyTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.write("legacy.rs", "// legacy Lazerhorn identifier\n")
        self.write("active.md", "The Acclamation\n")
        self.write("CURRENT_SYNTHESIS_HOLLOW_GROVE_WORLD_CONTEXT_v0.1.0.md", "current context\n")
        self.write("artifacts/current_synthesis_world_context.md", "current context\n")
        self.write(geography.HISTORY, json.dumps({"protected_sha256": {"legacy.rs": self.digest("legacy.rs")}}))
        self.policy = {
            "schema_version": 1, "historical_inventory_sha256": self.digest(geography.HISTORY),
            "allowed_occurrences": {"legacy.rs": {"category": "compatibility-verification",
                "lines": {"// legacy Lazerhorn identifier": 1}}}, "reviewed_protected_updates": {},
        }
        self.write(geography.POLICY, json.dumps(self.policy))
        self.write(".gitignore", "target/\ndist/\n")
        self.write("SOURCE-MANIFEST.json", json.dumps({"files": {
            str(p.relative_to(self.root)): self.digest(str(p.relative_to(self.root)))
            for p in self.root.rglob("*") if p.is_file()}}))

    def write(self, name, content):
        path = self.root / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(content)

    def digest(self, name):
        return hashlib.sha256((self.root / name).read_bytes()).hexdigest()

    def test_exported_source_ignores_warmed_and_removed_caches(self):
        before = geography.verify(self.root)
        self.write("target/debug/old", "obsolete Lazerhorn string\n")
        self.write("dist/old.log", "Lazerhorn log\n")
        self.assertEqual(before, geography.verify(self.root))
        (self.root / "target/debug/old").unlink()
        self.assertEqual(before, geography.verify(self.root))

    def test_new_active_signage_fails(self):
        self.write("active.md", "Travel to Lazerhorn\n")
        with self.assertRaisesRegex(ValueError, "source occurrence drift"):
            geography.verify(self.root)

    def test_duplicating_allowed_line_does_not_gain_an_exception(self):
        self.write("legacy.rs", "// legacy Lazerhorn identifier\n" * 2)
        with self.assertRaisesRegex(ValueError, "source occurrence drift"):
            geography.verify(self.root)

    def test_protected_content_and_historical_inventory_remain_checked(self):
        self.write("legacy.rs", "// legacy Lazerhorn identifier\n// changed\n")
        with self.assertRaisesRegex(ValueError, "protected source/archive changed"):
            geography.verify(self.root)
        self.write(geography.HISTORY, "{}")
        with self.assertRaisesRegex(ValueError, "dated geography inventory changed"):
            geography.verify(self.root)

    def test_source_mirror_drift_fails(self):
        self.write("artifacts/current_synthesis_world_context.md", "different\n")
        with self.assertRaisesRegex(ValueError, "mirror differs"):
            geography.verify(self.root)

    def test_git_source_includes_new_nonignored_files(self):
        (self.root / "SOURCE-MANIFEST.json").unlink()
        subprocess.run(["git", "init", "-q", str(self.root)], check=True)
        subprocess.run(["git", "-C", str(self.root), "add", "."], check=True)
        geography.verify(self.root)
        self.write("target/cache", "Lazerhorn\n")
        geography.verify(self.root)
        self.write("new-signage.md", "Lazerhorn\n")
        with self.assertRaisesRegex(ValueError, "new-signage.md"):
            geography.verify(self.root)


if __name__ == "__main__":
    unittest.main()
