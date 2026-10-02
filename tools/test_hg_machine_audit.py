#!/usr/bin/env python3
"""Unit tests for the read-only Hollow Grove machine audit."""

import importlib.util
from pathlib import Path
import tempfile
import unittest


spec = importlib.util.spec_from_file_location(
    "hg_machine_audit", Path(__file__).with_name("hg-machine-audit.py")
)
hg_machine_audit = importlib.util.module_from_spec(spec)
spec.loader.exec_module(hg_machine_audit)


class MachineAuditTests(unittest.TestCase):
    def test_check_omits_empty_evidence(self):
        self.assertEqual(
            hg_machine_audit.check("pass", "okay"),
            {"status": "pass", "summary": "okay"},
        )

    def test_profile_identity_must_match_filename(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            (root / "distro/machines").mkdir(parents=True)
            (root / "distro/machines/canary.toml").write_text(
                'schema_version = 1\nmachine_id = "other"\n'
            )
            with self.assertRaisesRegex(hg_machine_audit.AuditError, "identity"):
                hg_machine_audit.load_profile(root, "canary")

    def test_unsafe_machine_identity_is_rejected_before_path_lookup(self):
        with self.assertRaisesRegex(hg_machine_audit.AuditError, "unsafe"):
            hg_machine_audit.load_profile(Path("/tmp"), "../machine")


if __name__ == "__main__":
    unittest.main()
