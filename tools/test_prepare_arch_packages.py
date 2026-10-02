#!/usr/bin/env python3
"""Exercise source isolation, provenance, and repeatability with real makepkg metadata."""

import importlib.util
import json
from pathlib import Path
import subprocess
import tarfile
import tempfile
import unittest

spec = importlib.util.spec_from_file_location("prepare_arch", Path(__file__).with_name("prepare-arch-packages.py"))
prepare_arch = importlib.util.module_from_spec(spec)
spec.loader.exec_module(prepare_arch)


class SourceBundleTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name) / "source"
        self.root.mkdir()
        inputs = {
            "Cargo.toml": '[package]\nname="fixture"\nversion="0.1.0"\n',
            ".gitignore": 'dist/\nsecret.env\n',
            "packaging/hollow-grove-general/etc/hollow-grove/applications.toml": '[required_base]\narch_packages=["niri"]\n',
            "distro/profiles/desktop.toml": 'packages=["niri", "hueman"]\n[external_packages]\nrequired=["walker"]\n',
            "packaging/fixture/PKGBUILD.in": "pkgname=fixture\npkgver=@VERSION@\npkgrel=1\narch=('any')\nsource=('@SOURCE_ARCHIVE@')\nsha256sums=('@SOURCE_SHA256@')\ndepends=(@DESKTOP_DEPENDENCIES@)\npackage() { :; }\n",
        }
        for name, content in inputs.items():
            path = self.root / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(content)
        for args in [("init", "-q"), ("add", "."),
                     ("-c", "user.name=Fixture", "-c", "user.email=fixture@localhost", "commit", "-qm", "fixture")]:
            subprocess.run(["git", "-C", str(self.root), *args], check=True)

    def tearDown(self):
        self.temp.cleanup()

    def test_bundle_is_repeatable_and_independent_of_checkout(self):
        (self.root / "secret.env").write_text("must not ship")
        first = prepare_arch.prepare(self.root, self.root / "dist/one")
        second = prepare_arch.prepare(self.root, self.root / "dist/two")
        self.assertEqual(first.read_bytes(), second.read_bytes())
        recipe = (first.parent / "fixture/PKGBUILD").read_text()
        build_script = first.parent / "fixture/build.sh"
        self.assertNotIn(str(self.root), recipe)
        self.assertNotIn("@SOURCE", recipe)
        self.assertIn("export SOURCE_DATE_EPOCH=", build_script.read_text())
        self.assertTrue(build_script.stat().st_mode & 0o100)
        self.assertEqual(prepare_arch.desktop_dependencies(self.root), ["hueman", "niri", "walker"])
        with tarfile.open(first) as archive:
            self.assertFalse(any(name.endswith("secret.env") for name in archive.getnames()))
            manifest = json.load(archive.extractfile("hollow-grove-0.1.0/SOURCE-MANIFEST.json"))
            self.assertFalse(manifest["dirty_worktree"])
        # A moved, standalone directory can verify its source without Git or
        # any reference to the original repository.
        detached = Path(self.temp.name) / "detached"
        (first.parent / "fixture").rename(detached)
        subprocess.run(["makepkg", "--verifysource"], cwd=detached, check=True,
                       stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)

    def test_dirty_source_requires_explicit_snapshot(self):
        (self.root / "new.txt").write_text("new input")
        with self.assertRaisesRegex(ValueError, "dirty"):
            prepare_arch.prepare(self.root, self.root / "dist/refused")
        archive = prepare_arch.prepare(self.root, self.root / "dist/snapshot", allow_dirty=True)
        with tarfile.open(archive) as source:
            manifest = json.load(source.extractfile("hollow-grove-0.1.0/SOURCE-MANIFEST.json"))
            self.assertTrue(manifest["dirty_worktree"])
            self.assertIn("new.txt", manifest["files"])
            self.assertEqual(manifest["release_status"], "development-candidate")

    def test_source_cannot_capture_external_symlink(self):
        outside = Path(self.temp.name) / "private.txt"
        outside.write_text("private")
        (self.root / "escaped").symlink_to(outside)
        with self.assertRaisesRegex(ValueError, "escapes"):
            prepare_arch.prepare(self.root, self.root / "dist/refused", allow_dirty=True)

    def test_existing_output_is_not_overwritten(self):
        output = self.root / "dist/existing"
        prepare_arch.prepare(self.root, output)
        with self.assertRaises(FileExistsError):
            prepare_arch.prepare(self.root, output)


if __name__ == "__main__":
    unittest.main()
