#!/usr/bin/env python3
"""Reject private account files and common credentials from source snapshots."""

import argparse
from pathlib import Path, PurePosixPath
import re
import subprocess
import sys


PRIVATE_DIRECTORIES = {'.ssh', '.gnupg', '.aws'}
PRIVATE_NAMES = {'authorized_keys', 'authorized_keys2', 'known_hosts',
                 'id_rsa', 'id_dsa', 'id_ecdsa', 'id_ed25519', 'credentials.json'}
PATTERNS = {
    'private key': re.compile(rb'-----BEGIN (?:RSA |EC |DSA |OPENSSH |PGP )?PRIVATE KEY(?: BLOCK)?-----'),
    'GitHub credential': re.compile(rb'\b(?:gh[pousr]_[A-Za-z0-9]{30,}|github_pat_[A-Za-z0-9_]{40,})\b'),
    'cloud access key': re.compile(rb'\b(?:AKIA|ASIA)[A-Z0-9]{16}\b'),
    'OpenAI credential': re.compile(rb'\bsk-(?:proj-|svcacct-)?[A-Za-z0-9_-]{32,}\b'),
    'Slack credential': re.compile(rb'\bxox[baprs]-[A-Za-z0-9-]{20,}\b'),
    'credential-bearing URL': re.compile(rb'https?://[^\s/:@]+:[^\s/@]+@'),
}


def findings(name: str, data: bytes) -> list[str]:
    path = PurePosixPath(name)
    problems = []
    if PRIVATE_DIRECTORIES.intersection(path.parts) or path.name in PRIVATE_NAMES:
        problems.append('private account file')
    if path.name == '.env' or (path.name.startswith('.env.') and
                              not path.name.endswith(('.example', '.sample', '.template'))):
        problems.append('local environment file')
    if path.suffix.lower() in {'.key', '.pem', '.p12', '.pfx'}:
        problems.append('key or certificate container')
    if name.startswith('artifacts/gameplay-saves/') or (
        name.endswith(('.hgplay.json', '.sav', '.sav.backup')) and
        not name.startswith('tests/fixtures/')
    ):
        problems.append('live gameplay state')
    problems.extend(label for label, pattern in PATTERNS.items() if pattern.search(data))
    return problems


def audit_source(root: Path) -> list[str]:
    """Inspect the index and working source; never print matched secret bytes."""
    root = root.resolve()

    def git(*args):
        return subprocess.check_output(['git', '-C', str(root), *args])

    entries = git('ls-files', '--stage', '-z').split(b'\0')
    names = set()
    problems = set()

    def inspect(name, data, scope):
        for problem in findings(name, data):
            problems.add(f'{name} ({scope}): {problem}')

    with subprocess.Popen(['git', '-C', str(root), 'cat-file', '--batch'],
                          stdin=subprocess.PIPE, stdout=subprocess.PIPE) as blobs:
        try:
            for entry in entries:
                if not entry:
                    continue
                metadata, raw_name = entry.split(b'\t', 1)
                mode, object_id, stage = metadata.split()
                name = raw_name.decode('utf-8', errors='surrogateescape')
                names.add(name)
                if mode == b'160000':
                    problems.add(f'{name} (index): external submodule needs a separate source review')
                    continue
                blobs.stdin.write(object_id + b'\n')
                blobs.stdin.flush()
                header = blobs.stdout.readline().split()
                if len(header) != 3 or header[1] != b'blob':
                    raise ValueError(f'cannot inspect indexed source: {name}')
                data = blobs.stdout.read(int(header[2]))
                if blobs.stdout.read(1) != b'\n':
                    raise ValueError('invalid Git blob response')
                inspect(name, data, 'index')
                if mode == b'120000' and not (root / name).parent.joinpath(
                    data.decode('utf-8', errors='surrogateescape')
                ).resolve().is_relative_to(root):
                    problems.add(f'{name} (index): symlink escapes the source tree')
        finally:
            blobs.stdin.close()
            blobs.stdout.close()
    names.update(name.decode('utf-8', errors='surrogateescape') for name in
                 git('ls-files', '--others', '--exclude-standard', '-z').split(b'\0') if name)
    for name in sorted(names):
        path = root / name
        if not path.resolve().is_relative_to(root):
            problems.add(f'{name} (working tree): symlink escapes the source tree')
        elif path.is_file():
            inspect(name, path.read_bytes(), 'working tree')
    return sorted(problems)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root', type=Path, default=Path(__file__).resolve().parents[1])
    args = parser.parse_args()
    problems = audit_source(args.root)
    if problems:
        print('FAIL: private material must stay outside the source backup:', file=sys.stderr)
        for problem in problems:
            print('  ' + problem, file=sys.stderr)
        return 1
    print('PASS: source index and working inventory contain no recognized private account files or credentials')
    return 0


if __name__ == '__main__':
    sys.exit(main())
