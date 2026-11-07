#!/usr/bin/env python3
"""Measure the packaged service's active-view sync using disposable state."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import selectors
import statistics
import subprocess
import tempfile
import time


def source_provenance(world_root: Path, source_root: Path):
    """Report Git metadata only when the measured world root is the declared source tree."""
    root = world_root.resolve()
    source = source_root.resolve()
    if root != source:
        return None, None
    toplevel = subprocess.run(['git', 'rev-parse', '--show-toplevel'], cwd=source, text=True,
                              capture_output=True)
    if toplevel.returncode != 0:
        return None, None
    try:
        source.relative_to(Path(toplevel.stdout.strip()).resolve())
    except ValueError:
        return None, None
    commit = subprocess.run(['git', 'rev-parse', 'HEAD'], cwd=source, text=True, capture_output=True)
    status = subprocess.run(['git', 'status', '--porcelain=v1'], cwd=source, text=True,
                            capture_output=True)
    return (commit.stdout.strip() if commit.returncode == 0 else None,
            bool(status.stdout) if status.returncode == 0 else None)


def main():
    repo = Path(__file__).resolve().parents[1]
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--service', type=Path, default=repo / 'target/release/hollow_grove_game_service')
    parser.add_argument('--world-root', type=Path, default=repo)
    parser.add_argument('--warmup', type=int, default=10)
    parser.add_argument('--samples', type=int, default=100)
    parser.add_argument('--scenario', choices=('active-view', 'sympiote-return'), default='active-view',
                        help='Measure the ordinary view or a discharged, practiced Sympiote foundation.')
    parser.add_argument('--assessment-rounds', type=int, default=1,
                        help='Foundation assessments before establishment; exercises retained clinical history.')
    parser.add_argument('--output', type=Path, default=repo / 'dist/gameplay-service-performance.json')
    args = parser.parse_args()
    if args.warmup < 0 or args.samples < 1:
        parser.error('warmup must be nonnegative and samples must be positive')
    if args.assessment_rounds < 1:
        parser.error('assessment-rounds must be positive')
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
            last_view = None

            def transact(request_id, intent):
                nonlocal revision, last_view
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
                last_view = response['view']
                return elapsed, len(line.encode('utf-8'))

            try:
                transact('request.performance.enter', {'type': 'EnterHollowGroveIntent'})
                transact('request.performance.establish', {
                    'type': 'EstablishHuemanIntent', 'continuity_id': 'being-continuity.hueman',
                    'participant_id': 'participant.hueman', 'institutional_being_id': 'being.hueman',
                })
                if args.scenario == 'sympiote-return':
                    step = 0

                    def foundation(action):
                        nonlocal step
                        step += 1
                        transact(f'request.performance.foundation.{step}', {
                            'type': 'AdvanceSympioteFoundationIntent', 'action': action,
                        })

                    for _ in range(args.assessment_rounds):
                        foundation('RequestAssessment')
                        foundation('ConsentToFoundation')
                    for action in ('AskClinicianToEstablish', 'Rest', 'AskClinicianToDischarge'):
                        foundation(action)
                    for direction in ('Stonebend', 'Minorian', 'Minoan', 'Glaushouse', 'Flynt'):
                        foundation({'ExploreDirection': {'direction': direction}})
                    for camp in ('PimpAndTrooners', 'GimpAndGooners'):
                        foundation({'ConsiderBoardwalkOffer': {'camp': camp}})
                    for action in ({'Practice': {'adaptation': 'Coordination'}}, 'Rest',
                                   {'Practice': {'adaptation': 'Coordination'}}, 'Rest'):
                        foundation(action)
                    case = last_view['sympiote_foundation']
                    if not (case['form_established'] and case['comfortable_use'] and not case['needs_rest']):
                        raise RuntimeError('foundation setup did not reach rested comfortable use')
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
    if args.scenario != 'active-view':
        report['workload'] = 'sympiote_return_sync_stdio'
    source_commit, source_dirty = source_provenance(args.world_root, repo)
    service_commit = None
    for parent in list(service.parents)[:4]:
        manifest = parent / 'BUILD-MANIFEST.txt'
        if manifest.is_file():
            service_commit = next((line.removeprefix('git_commit=') for line in manifest.read_text().splitlines()
                                   if line.startswith('git_commit=')), None)
            break
    report.update({'scenario': args.scenario, 'assessment_rounds': args.assessment_rounds if args.scenario == 'sympiote-return' else 0,
                   'source_commit': source_commit,
                   'source_worktree_dirty': source_dirty,
                   'service_source_commit': service_commit,
                   'harness_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
                   'executed_via_ssh': bool(os.environ.get('SSH_CONNECTION')),
                   'godot_response_limit_bytes': 64 * 1024,
                   'within_godot_response_limit': max(sizes) <= 64 * 1024,
                   'clinical_receipt_count': last_view['sympiote_foundation']['clinical_receipt_count'],
                   'projected_clinical_receipts': len(last_view['sympiote_foundation']['clinical_receipts']),
                   'form_established': last_view['sympiote_foundation']['form_established']})
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps(report))


if __name__ == '__main__':
    main()
