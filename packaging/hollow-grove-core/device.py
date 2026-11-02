#!/usr/bin/env python3
"""Manage an installed local world and explicit, recoverable device handoffs."""
import argparse
import base64
from contextlib import contextmanager
import fcntl
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import platform
import re
import shutil
import subprocess
import sys
import tempfile
import uuid

ROOTS = ('world', 'gameplay-saves')
MAX_BYTES = 128 * 1024 * 1024
MAX_FILES = 10000
CATALOG = 'artifacts/institutional_state.txt'


def digest(data):
    return hashlib.sha256(data).hexdigest()


def encoded(value):
    return (json.dumps(value, sort_keys=True, indent=2) + '\n').encode()


def atomic_json(path, value):
    path.parent.mkdir(parents=True, exist_ok=True, mode=0o700)
    fd, temporary = tempfile.mkstemp(dir=path.parent, prefix='.device-write-')
    try:
        with os.fdopen(fd, 'wb') as stream:
            stream.write(encoded(value))
            stream.flush()
            os.fsync(stream.fileno())
        os.replace(temporary, path)
        sync_directory(path.parent)
    finally:
        Path(temporary).unlink(missing_ok=True)


def sync_directory(path):
    fd = os.open(path, os.O_RDONLY | os.O_DIRECTORY)
    try:
        os.fsync(fd)
    finally:
        os.close(fd)


def read_json(path, default=None):
    return json.loads(path.read_text()) if path.exists() else default


def durable_name(path):
    return not (path.name.endswith(('.runtime.lock', '.claim')))


def inventory(root):
    files = {}
    total = 0
    if root.is_symlink():
        raise ValueError(f'state root is a symlink: {root}')
    for name in ROOTS:
        directory = root / name
        if directory.is_symlink():
            raise ValueError(f'state directory is a symlink: {directory}')
        if not directory.exists():
            continue
        for path in sorted(directory.rglob('*')):
            if path.is_symlink():
                raise ValueError(f'state symlink requires manual review: {path}')
            if path.is_dir():
                continue
            if not path.is_file():
                raise ValueError(f'unsupported state entry: {path}')
            if not durable_name(path):
                continue
            total += path.stat().st_size
            if total > MAX_BYTES or len(files) >= MAX_FILES:
                raise ValueError('state exceeds the bounded handoff size; use a reviewed offline migration')
            data = path.read_bytes()
            files[path.relative_to(root).as_posix()] = {
                'sha256': digest(data), 'data': base64.b64encode(data).decode(),
            }
    return files


def fingerprint(files):
    return digest(encoded({name: item['sha256'] for name, item in files.items()}))


def validate_files(files):
    if not isinstance(files, dict) or len(files) > MAX_FILES:
        raise ValueError('invalid file inventory')
    total = 0
    for name, item in files.items():
        path = PurePosixPath(name)
        if (path.is_absolute() or len(path.parts) < 2 or path.parts[0] not in ROOTS
                or '..' in path.parts or path.as_posix() != name or '\\' in name
                or not durable_name(path)):
            raise ValueError(f'unsafe state path: {name}')
        data = base64.b64decode(item['data'], validate=True)
        total += len(data)
        if total > MAX_BYTES or digest(data) != item['sha256']:
            raise ValueError('state payload size or checksum mismatch')
    return files


class Device:
    def __init__(self, root, prefix=Path('/usr')):
        self.root = root
        self.prefix = prefix
        self.host = platform.node()
        self.pause = root / '.handoff-paused.json'
        self.journal = root / '.device-transaction.json'
        self.receipt = root / '.device-receipt.json'

    @contextmanager
    def locked(self):
        self.root.mkdir(parents=True, exist_ok=True, mode=0o700)
        with (self.root / '.device.lock').open('a') as lock:
            try:
                fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
            except BlockingIOError as error:
                raise ValueError('close Hollow Grove on this machine before changing or handing off state') from error
            self.check_leases()
            yield

    def check_leases(self):
        for path in (self.root / 'gameplay-saves').glob('*.runtime.lock'):
            owner = path.read_text().strip()
            match = re.fullmatch(r'\.[a-z0-9.-]+\.runtime-lease\.(\d+)\.(\d+)\.\d+\.claim', owner)
            if not match or int(match[2]) == 0:
                raise ValueError(f'unverifiable runtime lease: {path}')
            stat = Path('/proc') / match[1] / 'stat'
            try:
                text = stat.read_text()
            except FileNotFoundError:
                continue
            start = text[text.rfind(')') + 1:].split()[19]
            if start == match[2]:
                raise ValueError(f'active runtime lease: {path}; close Hollow Grove first')

    def binaries(self):
        return {name: digest((self.prefix / name).read_bytes()) for name in (
            'lib/hollow-grove/core/hollow_grove_game_service',
            'share/hueman/hueman.pck',
            'lib/hollow-grove/device.py',
        )}

    def status(self):
        files = inventory(self.root)
        catalog = self.root / 'world' / CATALOG
        office = False
        if catalog.is_file():
            office = any(line.startswith('office-holder\toffice.glaushouse.prima-donna\t')
                         and line.endswith('\ttrue') for line in catalog.read_text().splitlines())
        return {'schema_version': 1, 'host': self.host, 'state_sha256': fingerprint(files),
                'file_count': len(files), 'payload_sha256': self.binaries(),
                'world': str(self.root / 'world'), 'catalog_present': catalog.is_file(),
                'clinical_office_record_present': office,
                'paused': read_json(self.pause), 'recovery_required': self.journal.exists(),
                'last_receipt': read_json(self.receipt)}

    def require_ready(self):
        if self.journal.exists():
            raise ValueError('interrupted state installation; run hollow-grove-device recover first')

    def probe_world(self, world):
        with tempfile.TemporaryDirectory(prefix='hg-world-validation-') as temporary:
            request = {'protocol_version': 1, 'session_id': 'session.device-probe',
                       'request_id': 'request.device-probe', 'expected_revision': 0,
                       'intent': {'type': 'SyncIntent'}}
            result = subprocess.run([
                str(self.prefix / 'lib/hollow-grove/core/hollow_grove_game_service'),
                '--stdio', '--session', request['session_id'], '--save-root', temporary,
                '--world-root', str(world),
            ], input=json.dumps(request) + '\n', text=True, capture_output=True, timeout=15)
            if result.returncode or not result.stdout:
                raise ValueError('Core refused the world catalog: ' + result.stderr[-1500:])
            if json.loads(result.stdout.splitlines()[0]).get('status') != 'Completed':
                raise ValueError('Core could not synchronize the proposed world')

    def install(self, files, receipt, pause_after=None):
        """Caller holds the device lock. Journal makes a partial install fail closed."""
        validate_files(files)
        transaction = uuid.uuid4().hex
        stage = self.root / '.device-staging' / transaction
        backup = self.root / 'device-backups' / transaction
        stage.mkdir(parents=True, mode=0o700)
        backup.mkdir(parents=True, mode=0o700)
        for name in ROOTS:
            (stage / name).mkdir(mode=0o700)
        for name, item in files.items():
            path = stage / name
            path.parent.mkdir(parents=True, exist_ok=True, mode=0o700)
            with path.open('xb') as stream:
                os.chmod(path, 0o600)
                stream.write(base64.b64decode(item['data'], validate=True))
                stream.flush()
                os.fsync(stream.fileno())
        self.probe_world(stage / 'world')
        for path in sorted(stage.rglob('*'), reverse=True):
            if path.is_dir():
                sync_directory(path)
        sync_directory(stage)
        before = {'pause': read_json(self.pause), 'receipt': read_json(self.receipt)}
        atomic_json(backup / 'before.json', before)
        atomic_json(backup / 'incoming.json', {'files': files, 'receipt': receipt})
        journal = {'id': transaction, 'existed': {name: (self.root / name).exists() for name in ROOTS}}
        atomic_json(self.journal, journal)
        try:
            for name in ROOTS:
                if (self.root / name).exists():
                    os.rename(self.root / name, backup / name)
                    sync_directory(backup)
                    sync_directory(self.root)
                os.rename(stage / name, self.root / name)
                sync_directory(self.root)
            atomic_json(self.receipt, receipt)
            if pause_after is None:
                self.pause.unlink(missing_ok=True)
            else:
                atomic_json(self.pause, pause_after)
            self.journal.unlink()
            sync_directory(self.root)
        except BaseException:
            # A process crash leaves the same journal for the explicit recover command.
            self.recover()
            raise
        return str(backup)

    def recover(self):
        journal = read_json(self.journal)
        if not journal:
            return {'recovered': False}
        transaction = journal['id']
        if not re.fullmatch(r'[0-9a-f]{32}', transaction):
            raise ValueError('invalid recovery transaction')
        backup = self.root / 'device-backups' / transaction
        before = read_json(backup / 'before.json')
        for name in ROOTS:
            original = backup / name
            if original.exists() or not journal['existed'][name]:
                current = self.root / name
                if current.exists():
                    os.rename(current, backup / f'{name}.interrupted-{uuid.uuid4().hex}')
                if original.exists():
                    # Preserve the original backup so recovery can itself be retried.
                    shutil.copytree(original, current)
        for path, key in ((self.pause, 'pause'), (self.receipt, 'receipt')):
            if before[key] is None:
                path.unlink(missing_ok=True)
            else:
                atomic_json(path, before[key])
        self.journal.unlink()
        sync_directory(self.root)
        return {'recovered': True, 'backup': str(backup)}

    def import_world(self, source):
        self.require_ready()
        if self.pause.exists():
            raise ValueError('this device is parked; receive its current state before importing a world')
        files = inventory(self.root)
        if any(name.startswith('world/') for name in files):
            raise ValueError('world is not empty; preserving it for an explicit migration review')
        catalog = source / CATALOG
        if not catalog.is_file() or catalog.is_symlink():
            raise ValueError('source does not contain a regular institutional-state catalog')
        self.probe_world(source)
        data = catalog.read_bytes()
        files['world/' + CATALOG] = {'sha256': digest(data), 'data': base64.b64encode(data).decode()}
        receipt = {'operation': 'import-world', 'source': str(source),
                   'catalog_sha256': digest(data), 'state_sha256': fingerprint(files)}
        return {'imported': True, 'backup': self.install(files, receipt), **receipt}

    def receive(self, bundle):
        self.require_ready()
        if (bundle.get('schema_version') != 1 or bundle.get('target') != self.host
                or bundle.get('source') == self.host
                or not re.fullmatch(r'[0-9a-f]{32}', bundle.get('id', ''))):
            raise ValueError('invalid handoff destination or identity')
        if bundle['payload_sha256'] != self.binaries():
            raise ValueError('installed Core, Hueman or device tool differs; align packages before handoff')
        files = validate_files(bundle['files'])
        if fingerprint(files) != bundle['state_sha256']:
            raise ValueError('handoff manifest fingerprint mismatch')
        receipt = read_json(self.receipt, {})
        if receipt.get('handoff_id') == bundle['id']:
            if self.pause.exists():
                raise ValueError('this handoff was already received and subsequently handed onward')
            return {'received': True, 'already_received': True, 'handoff_id': bundle['id']}
        current = inventory(self.root)
        current_hash = fingerprint(current)
        pause = read_json(self.pause)
        if pause:
            if pause.get('target') != bundle['source'] or pause.get('state_sha256') != current_hash:
                raise ValueError('parked destination changed or belongs to a different peer')
        elif current and current_hash != bundle['state_sha256']:
            raise ValueError('destination has independent progress; refusing to overwrite it')
        receipt = {'operation': 'handoff', 'handoff_id': bundle['id'],
                   'source': bundle['source'], 'state_sha256': bundle['state_sha256']}
        backup = self.install(files, receipt)
        return {'received': True, 'handoff_id': bundle['id'], 'backup': backup}

    def handoff(self, peer):
        self.require_ready()
        if not re.fullmatch(r'[a-zA-Z0-9][a-zA-Z0-9_.@-]*', peer):
            raise ValueError('use a configured SSH host alias')
        ssh = ['ssh', '-o', 'BatchMode=yes', '-o', 'ConnectTimeout=8',
               '-o', 'StrictHostKeyChecking=yes', peer, '/usr/bin/hollow-grove-device']
        result = subprocess.run([*ssh, 'status'], capture_output=True, text=True, timeout=30)
        if result.returncode:
            raise ValueError('peer status failed: ' + result.stderr[-1500:])
        remote = json.loads(result.stdout)
        if remote['host'] == self.host or remote['recovery_required']:
            raise ValueError('peer is this machine or needs state recovery')
        if remote['payload_sha256'] != self.binaries():
            raise ValueError('peer package payloads differ; align packages first')
        files = inventory(self.root)
        state_hash = fingerprint(files)
        pause = read_json(self.pause)
        if pause:
            if pause.get('target') != remote['host'] or pause.get('state_sha256') != state_hash:
                raise ValueError('this device was parked for another state or peer')
            bundle = read_json(self.root / 'handoffs' / f"{pause['id']}.json")
        else:
            remote_pause = remote['paused']
            if remote_pause:
                if (remote_pause.get('target') != self.host
                        or remote_pause.get('state_sha256') != remote['state_sha256']):
                    raise ValueError('peer state changed after parking or belongs to another device')
            elif remote['file_count'] and remote['state_sha256'] != state_hash:
                raise ValueError('peer has independent progress; refusing to overwrite it')
            bundle = {'schema_version': 1, 'id': uuid.uuid4().hex, 'source': self.host,
                      'target': remote['host'], 'payload_sha256': self.binaries(),
                      'state_sha256': state_hash, 'files': files}
            atomic_json(self.root / 'handoffs' / f"{bundle['id']}.json", bundle)
            pause = {key: bundle[key] for key in ('id', 'source', 'target', 'state_sha256')}
            atomic_json(self.pause, pause)
        result = subprocess.run([*ssh, 'receive'], input=encoded(bundle), capture_output=True, timeout=120)
        if result.returncode:
            raise ValueError('handoff incomplete; this device remains parked. Repeat the same handoff command. '
                             + result.stderr.decode(errors='replace')[-1500:])
        receipt = json.loads(result.stdout)
        if not receipt.get('received') or receipt.get('handoff_id') != bundle['id']:
            raise ValueError('peer receipt mismatch; sender remains parked')
        atomic_json(self.root / 'handoffs' / f"{bundle['id']}.receipt.json", receipt)
        return {'handed_off_to': remote['host'], 'sender_parked': True, **receipt}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--data-root', type=Path,
                        default=Path(os.environ.get('XDG_DATA_HOME', str(Path.home() / '.local/share'))) / 'hollow-grove')
    parser.add_argument('--prefix', type=Path, default=Path('/usr'))
    sub = parser.add_subparsers(dest='command', required=True)
    sub.add_parser('status', help='read-only world, package and handoff status')
    sub.add_parser('recover', help='restore the saved state after an interrupted incoming installation')
    sub.add_parser('receive', help=argparse.SUPPRESS)
    sub.add_parser('import-world', help='copy an existing catalog into an empty installed world').add_argument('source', type=Path)
    sub.add_parser('handoff', help='hand current world and saves to the configured peer').add_argument('peer', nargs='?')
    sub.add_parser('configure-peer', help='set the existing pinned SSH alias').add_argument('peer')
    args = parser.parse_args()
    config = Path(os.environ.get('XDG_CONFIG_HOME', str(Path.home() / '.config'))) / 'hollow-grove/device.json'
    device = Device(args.data_root.absolute(), args.prefix.absolute())
    try:
        if args.command == 'status':
            result = device.status()
        elif args.command == 'configure-peer':
            if not re.fullmatch(r'[a-zA-Z0-9][a-zA-Z0-9_.@-]*', args.peer):
                raise ValueError('invalid SSH alias')
            atomic_json(config, {'peer': args.peer})
            result = {'peer': args.peer}
        else:
            with device.locked():
                if args.command == 'import-world':
                    result = device.import_world(args.source.resolve())
                elif args.command == 'recover':
                    result = device.recover()
                elif args.command == 'receive':
                    data = sys.stdin.buffer.read(MAX_BYTES * 2 + 1)
                    if len(data) > MAX_BYTES * 2:
                        raise ValueError('handoff envelope is too large')
                    result = device.receive(json.loads(data))
                else:
                    peer = args.peer or read_json(config, {}).get('peer')
                    if not peer:
                        raise ValueError('configure-peer needs the other machine\'s existing SSH alias')
                    result = device.handoff(peer)
        print(json.dumps(result, indent=2, sort_keys=True))
    except (OSError, ValueError, KeyError, TypeError, subprocess.SubprocessError) as error:
        parser.exit(1, f'Hollow Grove device: {error}\n')


if __name__ == '__main__':
    main()
