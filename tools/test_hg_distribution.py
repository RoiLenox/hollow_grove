#!/usr/bin/env python3
"""Tests for Hollow Grove distribution policy and repository verification."""

import hashlib
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest


spec = importlib.util.spec_from_file_location(
    "hg_distribution", Path(__file__).with_name("hg-distribution.py")
)
hg_distribution = importlib.util.module_from_spec(spec)
spec.loader.exec_module(hg_distribution)


CHANNELS = """\
schema_version = 1
repository_name = "hollow-grove"
target_architecture = "x86_64"
package_signatures_required = true
database_signatures_required = true
build_once_promote_unchanged = true

[packages]
required = ["hollow-grove-core", "hollow-grove-desktop", "hollow-grove-general", "hueman"]

[channels.testing]
repository_section = "hollow-grove-testing"
consumer = "occamsrazer"
purpose = "canary"
allow_unsigned_development = true
publication = false

[channels.stable]
repository_section = "hollow-grove"
consumer = "noahsarch"
purpose = "stable"
allow_unsigned_development = false
publication = false
"""


MACHINES = {
    "noahsarch.toml": """\
schema_version = 1
machine_id = "noahsarch"
role = "forge-and-stable"
architecture = "x86_64"
update_channel = "stable"
build_authority = true
promotion_authority = true
signing_authority = false
personal_configuration = "machine-owned"
network_addresses_recorded = false
""",
    "occamsrazer.toml": """\
schema_version = 1
machine_id = "occamsrazer"
role = "canary-and-mobile"
architecture = "x86_64"
update_channel = "testing"
build_authority = false
promotion_authority = false
signing_authority = false
personal_configuration = "machine-owned"
network_addresses_recorded = false
""",
}


class DistributionTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        (self.root / "distro/machines").mkdir(parents=True)
        (self.root / "distro/keys").mkdir()
        (self.root / "distro/channels.toml").write_text(CHANNELS)
        for name, contents in MACHINES.items():
            (self.root / "distro/machines" / name).write_text(contents)

    def tearDown(self):
        self.temp.cleanup()

    def test_policy_and_machine_roles_are_explicit(self):
        policy = hg_distribution.load_policy(self.root)
        profiles = hg_distribution.load_machine_profiles(self.root)
        self.assertEqual(policy["channels"]["testing"]["consumer"], "occamsrazer")
        self.assertEqual(
            {profile["machine_id"] for profile in profiles}, {"noahsarch", "occamsrazer"}
        )

    def test_machine_cannot_silently_claim_signing_authority(self):
        path = self.root / "distro/machines/noahsarch.toml"
        path.write_text(path.read_text().replace("signing_authority = false", "signing_authority = true"))
        with self.assertRaisesRegex(hg_distribution.DistributionError, "signing authority"):
            hg_distribution.load_machine_profiles(self.root)

    def test_stable_rejects_unsigned_assembly_before_touching_output(self):
        output = self.root / "stable"
        with self.assertRaisesRegex(hg_distribution.DistributionError, "stable forbids"):
            hg_distribution.assemble_repository(
                self.root,
                self.root / "packages",
                output,
                "stable",
                True,
                None,
            )
        self.assertFalse(output.exists())

    def test_trusted_fingerprint_parser_fails_closed(self):
        path = self.root / "distro/keys/trusted-fingerprints.txt"
        path.write_text("not-a-fingerprint\n")
        with self.assertRaisesRegex(hg_distribution.DistributionError, "invalid trusted fingerprint"):
            hg_distribution.trusted_fingerprints(self.root)
        path.write_text("A" * 40 + "\n")
        self.assertEqual(hg_distribution.trusted_fingerprints(self.root), {"A" * 40})

    def test_repository_verifier_detects_artifact_tampering(self):
        repository = self.root / "repository"
        repository.mkdir()
        payload = repository / "payload.bin"
        payload.write_bytes(b"canonical")
        digest = hashlib.sha256(payload.read_bytes()).hexdigest()
        release = {
            "schema_version": 1,
            "repository": "hollow-grove",
            "repository_section": "hollow-grove-testing",
            "channel": "testing",
            "mode": "development-unsigned",
            "packages": [{"name": name} for name in (
                "hollow-grove-core",
                "hollow-grove-desktop",
                "hollow-grove-general",
                "hueman",
            )],
            "artifacts": {payload.name: digest},
            "symlinks": {},
            "publication_ready": False,
        }
        (repository / "RELEASE.json").write_text(json.dumps(release))
        (repository / "SHA256SUMS").write_text(f"{digest}  {payload.name}\n")
        payload.write_bytes(b"tampered")
        with self.assertRaisesRegex(hg_distribution.DistributionError, "failed verification"):
            hg_distribution.verify_repository(self.root, repository)

    def test_repository_verifier_rejects_unrecorded_artifacts(self):
        repository = self.root / "repository"
        repository.mkdir()
        release = {
            "schema_version": 1,
            "repository": "hollow-grove",
            "repository_section": "hollow-grove-testing",
            "channel": "testing",
            "mode": "development-unsigned",
            "packages": [{"name": name} for name in (
                "hollow-grove-core",
                "hollow-grove-desktop",
                "hollow-grove-general",
                "hueman",
            )],
            "artifacts": {"payload.bin": hashlib.sha256(b"canonical").hexdigest()},
            "symlinks": {},
            "publication_ready": False,
        }
        (repository / "payload.bin").write_bytes(b"canonical")
        (repository / "extra.bin").write_bytes(b"unrecorded")
        (repository / "RELEASE.json").write_text(json.dumps(release))
        (repository / "SHA256SUMS").write_text(
            f"{release['artifacts']['payload.bin']}  payload.bin\n"
        )
        with self.assertRaisesRegex(hg_distribution.DistributionError, "unrecorded"):
            hg_distribution.verify_repository(self.root, repository)

    def test_signed_source_identity_requires_clean_verified_tag(self):
        record = {
            "source_manifest": {
                "git_commit": "a" * 40,
                "dirty_worktree": False,
                "release_status": "signed-source",
                "verified_tag": "v0.1.0",
            },
            "source_manifest_sha256": "b" * 64,
        }
        identity = hg_distribution.common_source_identity([record])
        self.assertEqual(identity["verified_tag"], "v0.1.0")
        dirty = json.loads(json.dumps(record))
        dirty["source_manifest"]["dirty_worktree"] = True
        self.assertTrue(hg_distribution.common_source_identity([dirty])["dirty_worktree"])


if __name__ == "__main__":
    unittest.main()
