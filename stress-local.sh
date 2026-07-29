#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
cd "${SCRIPT_DIR}"

CANONICAL_WITNESS=$'Point\n↓\nTriway\n↓\nFourway\n↓\nHollowGrove\n↓\nCurrentSeam [PlebExterior]\n↓\nAuraBeam [BlepReturn]\n↓\nPoint² (Landed Point) [BlepArrival]'
BINARY="target/release/hollow-grove"
WARMUP_RUNS="${WARMUP_RUNS:-100}"
STANDARD_RUNS="${STANDARD_RUNS:-10000}"
HEAVY_RUNS="${HEAVY_RUNS:-100000}"
SKIP_BUILD="${SKIP_BUILD:-0}"

run_level() {
    local label="$1"
    local total_runs="$2"
    local failures=0
    local mismatches=0
    local run_start_ns
    local run_end_ns
    local run_elapsed_ns
    local total_elapsed_ns=0
    local min_elapsed_ns=0
    local max_elapsed_ns=0
    local output
    local i

    printf '%s\n' "${label}"
    printf 'total runs: %s\n' "${total_runs}"

    for ((i = 1; i <= total_runs; i++)); do
        run_start_ns="$(date +%s%N)"

        if ! output="$("${BINARY}")"; then
            failures="$((failures + 1))"
            output=""
        fi

        run_end_ns="$(date +%s%N)"
        run_elapsed_ns="$((run_end_ns - run_start_ns))"
        total_elapsed_ns="$((total_elapsed_ns + run_elapsed_ns))"

        if ((min_elapsed_ns == 0 || run_elapsed_ns < min_elapsed_ns)); then
            min_elapsed_ns="${run_elapsed_ns}"
        fi

        if ((run_elapsed_ns > max_elapsed_ns)); then
            max_elapsed_ns="${run_elapsed_ns}"
        fi

        if [[ "${output}" != "${CANONICAL_WITNESS}" ]]; then
            mismatches="$((mismatches + 1))"
        fi
    done

    printf 'failures: %s\n' "${failures}"
    printf 'output mismatches: %s\n' "${mismatches}"
    printf 'total elapsed time (ms): %s\n' "$((total_elapsed_ns / 1000000))"
    printf 'average runtime per run (us): %s\n' "$((total_elapsed_ns / total_runs / 1000))"
    printf 'min runtime (us): %s\n' "$((min_elapsed_ns / 1000))"
    printf 'max runtime (us): %s\n' "$((max_elapsed_ns / 1000))"

    if ((failures > 0 || mismatches > 0)); then
        return 1
    fi
}

printf 'Active Linux kernel:\n'
uname -r

printf 'CPU model:\n'
lscpu | grep "Model name"

cargo fmt --check
if [[ "${SKIP_BUILD}" == "0" ]]; then
    cargo test --quiet
    cargo build --release --quiet
elif [[ ! -x "${BINARY}" ]]; then
    printf 'Missing release binary while SKIP_BUILD is set: %s\n' "${BINARY}" >&2
    exit 1
fi

printf 'Release binary:\n'
ls -lh "${BINARY}"

run_level "warmup" "${WARMUP_RUNS}"
run_level "standard" "${STANDARD_RUNS}"
run_level "heavy" "${HEAVY_RUNS}"
