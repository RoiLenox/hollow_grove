import copy
import importlib.util
import json
import os
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location('shared_session', ROOT / 'packaging/hollow-grove-core/shared_session.py')
shared = importlib.util.module_from_spec(spec)
spec.loader.exec_module(shared)


class SharedCredentialTests(unittest.TestCase):
    def test_app_credential_binds_device_account_and_session(self):
        config = dict(account_id='player.warren', session_id='session.warren', devices={'desktop': 'a' * 64})
        hello = dict(type='SharedSessionHello', version=1, account_id='player.warren',
                     session_id='session.warren', device_id='desktop', token='a' * 64)
        self.assertTrue(shared.authenticate(config, hello))
        for key, value in [('token', 'b' * 64), ('device_id', 'stranger'), ('account_id', 'player.other'),
                           ('session_id', 'session.other'), ('version', 2), ('type', 'MoveIntent')]:
            self.assertFalse(shared.authenticate(config, dict(hello, **{key: value})))
        self.assertFalse(shared.authenticate(config, None))
        for value in (None, [], {}, 7):
            self.assertFalse(shared.authenticate(config, dict(hello, device_id=value)))
            self.assertFalse(shared.authenticate(config, dict(hello, token=value)))
        self.assertFalse(shared.authenticate(config, dict(hello, token='é' * 64)))

    def test_world_readable_credentials_are_rejected(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / 'client.json'
            path.write_text(json.dumps(dict(version=1, account_id='player.warren', session_id='session.warren', device_id='desktop')))
            path.chmod(0o644)
            with self.assertRaisesRegex(ValueError, 'private'):
                shared.read_config(path)
            path.chmod(0o600)
            self.assertEqual(shared.read_config(path)['device_id'], 'desktop')


@unittest.skipUnless(os.environ.get('HOLLOW_GROVE_TEST_SERVICE'), 'set HOLLOW_GROVE_TEST_SERVICE for Core integration')
class SharedAuthorityTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.config = dict(service_bin=os.environ['HOLLOW_GROVE_TEST_SERVICE'], session_id='session.shared-test',
                           account_id='player.warren', save_root=self.temp.name, world_root=str(ROOT))
        self.host = shared.Authority(self.config)
        self.addCleanup(lambda: self.host.close())

    def send(self, kind, **payload):
        response = self.host.handle(shared.envelope(self.config, kind, self.host.revision, **payload))
        self.assertEqual(response['status'], 'Completed', response.get('rejection'))
        return response

    def enter(self):
        self.send('EnterHollowGroveIntent')
        return self.send('EstablishHuemanIntent', continuity_id='being-continuity.hueman',
                         participant_id='participant.hueman', institutional_being_id='being.hueman')

    def test_two_clients_share_identity_movement_and_restart_checkpoint(self):
        first = self.enter()
        moved = self.send('MoveIntent', direction='East')
        observed = self.send('ObserveIntent')
        self.assertEqual(moved['view'], observed['view'])
        self.assertEqual(first['view']['hueman'], moved['view']['hueman'])
        self.host.close()
        self.host = shared.Authority(self.config)
        resumed = self.send('ObserveIntent')
        self.assertEqual(resumed['view'], moved['view'])
        self.send('MoveIntent', direction='West')

    def test_failed_checkpoint_stops_authority_before_acknowledging_movement(self):
        self.enter()
        with patch.object(self.host, 'checkpoint', side_effect=OSError('disk unavailable')):
            with self.assertRaises(OSError):
                self.send('MoveIntent', direction='East')
        self.assertTrue(self.host.failed)
        with self.assertRaisesRegex(RuntimeError, 'recovery'):
            self.send('ObserveIntent')

    def test_wrong_player_and_reserved_checkpoint_cannot_mutate_state(self):
        initial = self.send('ObserveIntent')
        for kind, payload in [('EstablishHuemanIntent', dict(continuity_id='another')),
                              ('CheckpointIntent', dict(slot_id='shared-autosave')),
                              ('LoadIntent', dict(slot_id='shared-autosave'))]:
            with self.assertRaises(ValueError):
                self.host.handle(shared.envelope(self.config, kind, self.host.revision, **payload))
        self.assertEqual(initial['view'], self.send('ObserveIntent')['view'])

    def test_malformed_intent_does_not_break_authority(self):
        for request in (None, [], {}, {'intent': None}, {'intent': []}):
            with self.assertRaises(ValueError):
                self.host.handle(request)
        self.send('ObserveIntent')


if __name__ == '__main__':
    unittest.main()
