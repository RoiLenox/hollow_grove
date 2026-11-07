#!/usr/bin/env python3
from pathlib import Path
import subprocess
import tempfile
import unittest

from source_boundary import audit_source, findings


class SourceBoundaryTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name) / 'source'
        self.root.mkdir()
        self.git('init', '-q')

    def git(self, *args):
        subprocess.run(['git', '-C', str(self.root), *args], check=True,
                       stdout=subprocess.DEVNULL, stderr=subprocess.PIPE)

    def test_named_hardware_profiles_and_product_skeleton_are_source(self):
        for name in ('distro/machines/noahsarch.toml', 'distro/machines/occamsrazer.toml',
                     'packaging/example/etc/skel/.config/niri/config.kdl', '.env.example'):
            self.assertEqual(findings(name, b'nonsecret template'), [])

    def test_private_files_and_live_saves_are_rejected(self):
        for name in ('.ssh/config', 'nested/.aws/credentials', 'authorized_keys',
                     '.env', '.env.production', 'identity.pem', 'world.hgplay.json',
                     'artifacts/gameplay-saves/state.json'):
            self.assertTrue(findings(name, b''), name)
        self.assertFalse(findings('tests/fixtures/example.hgplay.json', b'{}'))

    def test_key_material_is_detected_even_in_an_innocent_filename(self):
        for kind in (b'OPENSSH ', b'RSA ', b'EC ', b''):
            secret = b'-----BEGIN ' + kind + b'PRIVATE KEY-----'
            self.assertIn('private key', findings('notes.txt', secret))

    def test_staged_secret_cannot_be_hidden_by_clean_working_bytes(self):
        path = self.root / 'notes.txt'
        token = 'ghp_' + 'A' * 36
        path.write_text(token)
        self.git('add', 'notes.txt')
        path.write_text('clean working file')
        problems = audit_source(self.root)
        self.assertTrue(any('(index): GitHub credential' in p for p in problems))
        self.assertNotIn(token, '\n'.join(problems))

    def test_untracked_source_is_checked_but_ignored_state_is_not_uploaded(self):
        (self.root / '.gitignore').write_text('.env\n')
        (self.root / '.env').write_text('local only')
        self.assertEqual(audit_source(self.root), [])
        self.git('add', '-f', '.env')
        self.assertTrue(any('local environment file' in p for p in audit_source(self.root)))

    def test_working_changes_and_external_symlinks_are_detected(self):
        path = self.root / 'source.txt'
        path.write_text('ordinary source')
        self.git('add', 'source.txt')
        path.write_bytes(b'-----BEGIN ' + b'PRIVATE KEY-----')
        self.assertTrue(any('(working tree): private key' in p for p in audit_source(self.root)))
        (self.root / 'outside').symlink_to(self.root.parent / 'private')
        self.git('add', 'outside')
        self.assertTrue(any('symlink escapes' in p for p in audit_source(self.root)))


if __name__ == '__main__':
    unittest.main()
