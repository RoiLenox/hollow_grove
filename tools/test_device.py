import base64
import fcntl
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch

SOURCE = Path(__file__).resolve().parents[1] / 'packaging/hollow-grove-core'
spec = importlib.util.spec_from_file_location('device', SOURCE / 'device.py')
device = importlib.util.module_from_spec(spec)
spec.loader.exec_module(device)


class DeviceTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.base = Path(self.temp.name)
        self.prefix = self.base / 'prefix'
        service = self.prefix / 'lib/hollow-grove/core/hollow_grove_game_service'
        service.parent.mkdir(parents=True)
        service.write_text('#!/bin/sh\ncat >/dev/null\nprintf \'{"status":"Completed"}\\n\'\n')
        service.chmod(0o755)
        pack = self.prefix / 'share/hueman/hueman.pck'
        pack.parent.mkdir(parents=True)
        pack.write_bytes(b'fixture client')
        (self.prefix / 'lib/hollow-grove/device.py').write_bytes((SOURCE / 'device.py').read_bytes())
        self.a = device.Device(self.base / 'a', self.prefix)
        self.b = device.Device(self.base / 'b', self.prefix)
        self.a.host, self.b.host = 'alpha', 'beta'
        self.source = self.base / 'source'
        (self.source / 'artifacts').mkdir(parents=True)
        (self.source / device.CATALOG).write_text('schema_version:2\noffice-holder\toffice.glaushouse.prima-donna\tbeing.test\ttrue\n')

    def seed(self, target):
        with target.locked():
            return target.import_world(self.source)

    def bundle(self):
        files = device.inventory(self.a.root)
        return {'schema_version': 1, 'id': 'a' * 32, 'source': 'alpha', 'target': 'beta',
                'files': files, 'state_sha256': device.fingerprint(files),
                'payload_sha256': self.a.binaries()}

    def test_import_preserves_saves_and_backs_up_original(self):
        saves = self.a.root / 'gameplay-saves'
        saves.mkdir(parents=True)
        (saves / 'progress.json').write_text('original')
        result = self.seed(self.a)
        self.assertEqual((saves / 'progress.json').read_text(), 'original')
        self.assertEqual((Path(result['backup']) / 'gameplay-saves/progress.json').read_text(), 'original')
        self.assertTrue(self.a.status()['clinical_office_record_present'])
        with self.a.locked(), self.assertRaisesRegex(ValueError, 'not empty'):
            self.a.import_world(self.source)

    def test_active_lease_and_shared_launcher_lock_refuse_mutation(self):
        saves = self.a.root / 'gameplay-saves'
        saves.mkdir(parents=True)
        stat = Path('/proc/self/stat').read_text()
        start = stat[stat.rfind(')') + 1:].split()[19]
        (saves / 'session.test.runtime.lock').write_text(f'.session.test.runtime-lease.{os.getpid()}.{start}.0.claim')
        with self.assertRaisesRegex(ValueError, 'active runtime'):
            with self.a.locked():
                pass
        (saves / 'session.test.runtime.lock').unlink()
        with (self.a.root / '.device.lock').open('a') as lock:
            fcntl.flock(lock, fcntl.LOCK_SH)
            with self.assertRaisesRegex(ValueError, 'close Hollow Grove'):
                with self.a.locked():
                    pass

    def test_symlink_and_traversal_rejected(self):
        self.seed(self.a)
        (self.a.root / 'world/escape').symlink_to(self.source / device.CATALOG)
        with self.assertRaisesRegex(ValueError, 'symlink'):
            device.inventory(self.a.root)
        item = {'data': base64.b64encode(b'x').decode(), 'sha256': device.digest(b'x')}
        for name in ('../escape', '/world/a', 'world/../escape', 'world//a', 'world/a.runtime.lock'):
            with self.subTest(name=name), self.assertRaises(ValueError):
                device.validate_files({name: item})

    def test_receive_verifies_bytes_and_refuses_divergence(self):
        self.seed(self.a)
        self.seed(self.b)
        (self.b.root / 'gameplay-saves/new-progress').write_text('keep me')
        bundle = self.bundle()
        with self.b.locked(), self.assertRaisesRegex(ValueError, 'independent progress'):
            self.b.receive(bundle)
        self.assertEqual((self.b.root / 'gameplay-saves/new-progress').read_text(), 'keep me')
        bundle['payload_sha256']['share/hueman/hueman.pck'] = 'bad'
        with self.b.locked(), self.assertRaisesRegex(ValueError, 'differs'):
            self.b.receive(bundle)

    def test_parked_receive_is_recoverable_and_idempotent(self):
        self.seed(self.a)
        self.seed(self.b)
        (self.a.root / 'gameplay-saves/new-progress').write_text('new')
        device.atomic_json(self.b.pause, {'target': 'alpha', 'state_sha256': device.fingerprint(device.inventory(self.b.root))})
        bundle = self.bundle()
        with self.b.locked():
            result = self.b.receive(bundle)
        self.assertFalse(self.b.pause.exists())
        self.assertEqual((self.b.root / 'gameplay-saves/new-progress').read_text(), 'new')
        self.assertTrue(Path(result['backup']).is_dir())
        with self.b.locked():
            self.assertTrue(self.b.receive(bundle)['already_received'])

    def test_corrupt_bundle_never_changes_target(self):
        self.seed(self.a)
        bundle = self.bundle()
        bundle['files']['world/' + device.CATALOG]['sha256'] = 'bad'
        with self.b.locked(), self.assertRaisesRegex(ValueError, 'checksum'):
            self.b.receive(bundle)
        self.assertFalse((self.b.root / 'world').exists())

    def test_failed_install_restores_both_roots(self):
        self.seed(self.a)
        before = device.fingerprint(device.inventory(self.a.root))
        files = device.inventory(self.a.root)
        files['gameplay-saves/new'] = {'data': 'eA==', 'sha256': device.digest(b'x')}
        original = os.rename
        def fail_second_root(source, destination):
            if Path(source).parent.parent.name == '.device-staging' and Path(source).name == 'gameplay-saves':
                raise OSError('simulated interruption')
            return original(source, destination)
        with self.a.locked(), patch.object(device.os, 'rename', side_effect=fail_second_root):
            with self.assertRaisesRegex(OSError, 'interruption'):
                self.a.install(files, {'test': True})
        self.assertEqual(device.fingerprint(device.inventory(self.a.root)), before)
        self.assertFalse(self.a.journal.exists())

    def test_launcher_refuses_parked_and_interrupted_state(self):
        actual = device.Device(self.base / 'hollow-grove', self.prefix)
        actual.root.mkdir(exist_ok=True)
        for marker in (actual.pause, actual.journal):
            device.atomic_json(marker, {'test': True})
            result = subprocess.run(['bash', '-c', 'source "$1"; hg_device_lock', 'bash',
                                     str(SOURCE / 'device-launch.sh')], capture_output=True,
                                    env={**os.environ, 'XDG_DATA_HOME': str(self.base)})
            self.assertNotEqual(result.returncode, 0)
            self.assertIn(b'parked', result.stderr)
            marker.unlink()

    def test_network_failure_keeps_sender_parked_with_retry_packet(self):
        self.seed(self.a)
        self.seed(self.b)
        replies = [subprocess.CompletedProcess([], 0, json.dumps(self.b.status()), ''),
                   subprocess.CompletedProcess([], 255, b'', b'connection lost')]
        with self.a.locked(), patch.object(device.subprocess, 'run', side_effect=replies):
            with self.assertRaisesRegex(ValueError, 'remains parked'):
                self.a.handoff('beta')
        pause = device.read_json(self.a.pause)
        self.assertEqual(pause['target'], 'beta')
        self.assertTrue((self.a.root / 'handoffs' / (pause['id'] + '.json')).is_file())


if __name__ == '__main__':
    unittest.main()
