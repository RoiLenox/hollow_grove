#!/usr/bin/env python3
import hashlib
import importlib.util
import json
from pathlib import Path
import tempfile
import tomllib
import unittest

spec = importlib.util.spec_from_file_location("source_inventory", Path(__file__).with_name("install-source-inventory.py"))
inventory = importlib.util.module_from_spec(spec)
spec.loader.exec_module(inventory)


class SourceInventoryTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name) / "source"
        self.root.mkdir()
        self.dest = Path(self.temp.name) / "installed"
        (self.root / "contract.md").write_text("authored input\n")
        self.manifest = {"files": {"contract.md": hashlib.sha256((self.root / "contract.md").read_bytes()).hexdigest()}}
        self.write_manifest()

    def write_manifest(self):
        (self.root / "SOURCE-MANIFEST.json").write_text(json.dumps(self.manifest))

    def test_only_verified_source_ships_after_build_checks(self):
        (self.root / "byproduct.log").write_text("must not ship")
        inventory.install(self.root, self.dest)
        self.assertEqual(sorted(p.name for p in self.dest.iterdir()), ["SOURCE-MANIFEST.json", "contract.md"])
        self.assertEqual((self.dest / "contract.md").read_bytes(), (self.root / "contract.md").read_bytes())

    def test_changed_input_and_escape_fail_before_installing(self):
        (self.root / "contract.md").write_text("modified")
        with self.assertRaisesRegex(ValueError, "differs"):
            inventory.install(self.root, self.dest)
        self.assertFalse(self.dest.exists())
        self.manifest["files"] = {"../outside": "0" * 64}
        self.write_manifest()
        with self.assertRaisesRegex(ValueError, "unsafe"):
            inventory.install(self.root, self.dest)
        self.assertFalse(self.dest.exists())

    def test_repository_package_policy_matches_recipes(self):
        root = Path(__file__).resolve().parents[1]
        policy = tomllib.loads((root / "distro/channels.toml").read_text())
        names = {p.parent.name for p in (root / "packaging").glob("*/PKGBUILD.in")}
        self.assertEqual(names, set(policy["packages"]["required"]))
        self.assertIn("hollow-grove-cli", names)


if __name__ == "__main__":
    unittest.main()
