#!/usr/bin/env python3
"""Unit tests for the read-only Hollow Grove machine audit."""

import importlib.util
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch
import json


spec = importlib.util.spec_from_file_location(
    "hg_machine_audit", Path(__file__).with_name("hg-machine-audit.py")
)
hg_machine_audit = importlib.util.module_from_spec(spec)
spec.loader.exec_module(hg_machine_audit)


class MachineAuditTests(unittest.TestCase):
    def test_kernel_upgrade_does_not_hide_missing_running_modules(self):
        with tempfile.TemporaryDirectory() as temporary:
            modules = Path(temporary)
            (modules / "new-kernel").mkdir()
            self.assertEqual(hg_machine_audit.kernel_modules("old-kernel", modules)["status"], "attention")
            self.assertEqual(hg_machine_audit.kernel_modules("new-kernel", modules)["status"], "pass")

    def test_failed_service_and_unavailable_bus_are_not_healthy(self):
        for result in [dict(exit_code=0, output="broken.service loaded failed failed Example"),
                       dict(exit_code=1, output="Cannot connect to bus")]:
            with patch.object(hg_machine_audit, "command", return_value=result):
                self.assertEqual(hg_machine_audit.failed_units()["status"], "attention")
        with patch.object(hg_machine_audit, "command", return_value=dict(exit_code=0, output="")):
            self.assertEqual(hg_machine_audit.failed_units(user=True)["status"], "pass")

    def test_unavailable_listener_inspection_is_explicit(self):
        with patch.object(hg_machine_audit, "command", return_value=dict(exit_code=1, output="denied")):
            self.assertFalse(hg_machine_audit.ssh_listener_scope()["inspection_available"])

    def test_device_recovery_and_parking_are_not_reported_as_launch_ready(self):
        data = dict(schema_version=1, catalog_present=True, recovery_required=False, paused=None,
                    payload_sha256={"core": "a" * 64})
        for update, expected in [({}, "pass"), ({"paused": {"peer": "private-peer"}}, "attention"),
                                 ({"recovery_required": True}, "blocker"),
                                 ({"catalog_present": False}, "attention"),
                                 ({"payload_sha256": {}}, "attention")]:
            with patch.object(hg_machine_audit, "command", return_value=dict(
                exit_code=0, output=json.dumps(data | update)
            )):
                result = hg_machine_audit.device_readiness()
                self.assertEqual(result["status"], expected)
                self.assertNotIn("private-peer", json.dumps(result))

    def test_invalid_device_status_cannot_pass(self):
        for output in ["not json", "[]", "{}", '{"schema_version": 2}']:
            with patch.object(hg_machine_audit, "command", return_value=dict(exit_code=0, output=output)):
                self.assertEqual(hg_machine_audit.device_readiness()["status"], "attention")

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
