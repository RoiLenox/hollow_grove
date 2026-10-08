#!/usr/bin/env python3
"""Measure the packaged service's active-view sync using disposable state."""

import argparse
import hashlib
import json
from pathlib import Path
import platform
import selectors
import statistics
import subprocess
import tempfile
import time


def main():
    repo = Path(__file__).resolve().parents[1]
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--service', type=Path, default=repo / 'target/release/hollow_grove_game_service')
    parser.add_argument('--world-root', type=Path, default=repo)
    parser.add_argument('--warmup', type=int, default=10)
    parser.add_argument('--samples', type=int, default=100)
    parser.add_argument('--output', type=Path, default=repo / 'dist/gameplay-service-performance.json')
    args = parser.parse_args()
    if args.warmup < 0 or args.samples < 1:
        parser.error('warmup must be nonnegative and samples must be positive')
    service = args.service.resolve()
    session = 'session.performance-audit'
    with tempfile.TemporaryDirectory(prefix='hollow-grove-service-benchmark.') as temporary:
        with tempfile.TemporaryFile(mode='w+') as errors:
            process = subprocess.Popen(
                [str(service), '--stdio', '--session', session,
                 '--save-root', temporary, '--world-root', str(args.world_root.resolve())],
                stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=errors,
                text=True, bufsize=1,
            )
            selector = selectors.DefaultSelector()
            selector.register(process.stdout, selectors.EVENT_READ)
            revision = 0

            def transact(request_id, intent):
                nonlocal revision
                request = {'protocol_version': 1, 'session_id': session,
                           'request_id': request_id, 'expected_revision': revision,
                           'intent': intent}
                encoded = json.dumps(request) + '\n'
                started = time.perf_counter_ns()
                process.stdin.write(encoded)
                process.stdin.flush()
                if not selector.select(timeout=10):
                    raise RuntimeError('service response timed out')
                line = process.stdout.readline()
                elapsed = time.perf_counter_ns() - started
                if not line:
                    errors.seek(0)
                    raise RuntimeError('service stopped: ' + errors.read())
                response = json.loads(line)
                if response['status'] != 'Completed':
                    raise RuntimeError(str(response.get('rejection')))
                revision = response['revision']
                return elapsed, len(line.encode('utf-8'))

            try:
                transact('request.performance.enter', {'type': 'EnterHollowGroveIntent'})
                transact('request.performance.establish', {
                    'type': 'EstablishHuemanIntent', 'continuity_id': 'being-continuity.hueman',
                    'participant_id': 'participant.hueman', 'institutional_being_id': 'being.hueman',
                })
                durations, sizes = [], []
                for index in range(args.warmup + args.samples):
                    elapsed, size = transact(f'request.performance.sync.{index}', {'type': 'SyncIntent'})
                    if index >= args.warmup:
                        durations.append(elapsed / 1000)
                        sizes.append(size)
            finally:
                selector.close()
                process.stdin.close()
                try:
                    process.wait(timeout=5)
                except subprocess.TimeoutExpired:
                    process.kill()
                    process.wait()
                process.stdout.close()
            if process.returncode:
                errors.seek(0)
                raise RuntimeError('service failed: ' + errors.read())
    ordered = sorted(durations)
    with service.open('rb') as stream:
        binary_sha256 = hashlib.file_digest(stream, 'sha256').hexdigest()
    cpu = next((line.split(':', 1)[1].strip() for line in Path('/proc/cpuinfo').read_text().splitlines()
                if line.startswith('model name')), 'unavailable')
    report = {'schema_version': 1, 'workload': 'active_view_sync_stdio',
              'description': 'Request write/flush to response-line receipt; JSON response decoding is outside the timing.',
              'warmup': args.warmup, 'samples': args.samples,
              'cpu': cpu, 'kernel': platform.release(), 'architecture': platform.machine(),
              'service_sha256': binary_sha256, 'service_bytes': service.stat().st_size,
              'median_us': statistics.median(ordered),
              'p95_us': ordered[min(len(ordered) - 1, (len(ordered) * 95 + 99) // 100 - 1)],
              'max_us': max(ordered), 'response_bytes_min': min(sizes), 'response_bytes_max': max(sizes),
              'disposable_state': True, 'remote_connection': False}
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps(report))


if __name__ == '__main__':
    main()
