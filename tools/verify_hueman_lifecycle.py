#!/usr/bin/env python3
"""Exercise the actual creator, overworld, save/exit and restart against Rust."""
import argparse
import os
from pathlib import Path
import re
import shutil
import subprocess
import tempfile
import time

ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--skip-build', action='store_true')
    parser.add_argument('--graphical', action='store_true')
    parser.add_argument('--artifacts', type=Path)
    args = parser.parse_args()
    godot = os.environ.get('GODOT_BIN') or shutil.which('godot4') or shutil.which('godot')
    if not godot:
        parser.error('Godot is required')
    if not args.skip_build:
        subprocess.run(['cargo', 'build', '--locked', '--offline', '--bin', 'hollow_grove_game_service'], cwd=ROOT, check=True)
    target = Path(os.environ.get('CARGO_TARGET_DIR', str(ROOT / 'target')))
    service_bin = target / 'debug/hollow_grove_game_service'
    with tempfile.TemporaryDirectory(prefix='hueman02-lifecycle-') as temp:
        directory = args.artifacts.resolve() if args.artifacts else Path(temp)
        directory.mkdir(parents=True, exist_ok=True)
        # All durable gameplay and draft data stay in a fresh temporary root.
        state = Path(temp)
        for phase in ('create', 'recover', 'return', 'final'):
            log_path = directory / f'service-{phase}.log'
            with log_path.open('w') as service_log:
                service = subprocess.Popen([str(service_bin), '--listen', '127.0.0.1:0',
                    '--session', 'session.hueman02-lifecycle', '--save-root', str(state / 'saves'),
                    '--world-root', str(ROOT)], stdout=service_log, stderr=service_log)
                try:
                    port = None
                    deadline = time.monotonic() + 15
                    while time.monotonic() < deadline and service.poll() is None:
                        match = re.search(r'listening on 127\.0\.0\.1:(\d+) for session', log_path.read_text())
                        if match:
                            port = match.group(1)
                            break
                        time.sleep(0.05)
                    if not port:
                        raise RuntimeError(f'{phase}: service not ready\n{log_path.read_text()}')
                    env = dict(os.environ, HOLLOW_GROVE_PORT=port, HOLLOW_GROVE_HOST='127.0.0.1',
                        HOLLOW_GROVE_SESSION='session.hueman02-lifecycle', HOLLOW_GROVE_CHARACTER_ENTRY='1',
                        HUEMAN02_PHASE=phase, HUEMAN02_SNAPSHOT=str(state / 'snapshot.json'))
                    env.pop('HOLLOW_GROVE_SHARED_CONFIG', None)
                    env.pop('HUEMAN02_CAPTURE_DIR', None)
                    if args.graphical:
                        env['HUEMAN02_CAPTURE_DIR'] = str(directory)
                    command = [godot] + ([] if args.graphical else ['--headless']) + ['--path', str(ROOT / 'hueman_godot'),
                        '--script', 'res://tools/hueman_02/lifecycle.gd', '--', f'--profile-dir={state / "draft"}']
                    result = subprocess.run(command, env=env, text=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, timeout=60)
                    (directory / f'godot-{phase}.log').write_text(result.stdout)
                    print(result.stdout, end='')
                    if result.returncode or f'HUEMAN-02 PASS {phase} (' not in result.stdout or 'ERROR:' in result.stdout:
                        raise RuntimeError(f'HUEMAN-02 {phase} failed')
                finally:
                    service.terminate()
                    try:
                        service.wait(timeout=5)
                    except subprocess.TimeoutExpired:
                        service.kill()
                        service.wait()
        print('HUEMAN-02 full creator / Stonebend / mission / Form / save-and-exit / restart lifecycle passed')


if __name__ == '__main__':
    main()
