#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
cd "${SCRIPT_DIR}"

REPEAT_COUNT=10
CANONICAL_WITNESS=$'Point\n↓\nTriway\n↓\nFourway\n↓\nHollowGrove\n↓\nCurrentSeam [PlebExterior]\n↓\nAuraBeam [BlepReturn]\n↓\nPoint² (Landed Point) [BlepArrival]'

time_once_ms() {
    local start_ns end_ns
    start_ns="$(date +%s%N)"
    "$@" >/dev/null
    end_ns="$(date +%s%N)"
    printf '%s\n' "$(((end_ns - start_ns) / 1000000))"
}

time_average_us() {
    local total_ns=0
    local start_ns end_ns _

    for _ in $(seq 1 "${REPEAT_COUNT}"); do
        start_ns="$(date +%s%N)"
        "$@" >/dev/null
        end_ns="$(date +%s%N)"
        total_ns="$((total_ns + end_ns - start_ns))"
    done

    printf '%s\n' "$((total_ns / REPEAT_COUNT / 1000))"
}

echo "Active Linux kernel:"
uname -r

echo "CPU model:"
lscpu | grep "Model name"

echo "Rust build mode:"
printf 'debug + release\n'

if [[ "${SKIP_FMT:-0}" != "1" ]]; then
  cargo fmt --check
fi

if [[ "${SKIP_TESTS:-0}" != "1" ]]; then
  echo "cargo test time (ms):"
  test_time_ms="$(time_once_ms cargo test --quiet)"
  printf '%s\n' "${test_time_ms}"
else
  echo "cargo test time (ms): skipped (already verified)"
fi

if [[ "${SKIP_BUILD:-0}" != "1" ]]; then
  echo "cargo build --release time (ms):"
  build_time_ms="$(time_once_ms cargo build --release --quiet)"
  printf '%s\n' "${build_time_ms}"
else
  echo "cargo build --release time (ms): skipped (already built)"
fi

echo "Release binary:"
ls -lh target/release/hollow-grove

echo "cargo run --quiet average (${REPEAT_COUNT} runs, us):"
debug_runtime_us="$(time_average_us cargo run --quiet)"
printf '%s\n' "${debug_runtime_us}"

echo "cargo run --release --quiet average (${REPEAT_COUNT} runs, us):"
release_cargo_runtime_us="$(time_average_us cargo run --release --quiet)"
printf '%s\n' "${release_cargo_runtime_us}"

echo "target/release/hollow-grove average (${REPEAT_COUNT} runs, us):"
direct_runtime_us="$(time_average_us target/release/hollow-grove)"
printf '%s\n' "${direct_runtime_us}"

echo "Canonical witness:"
witness_output="$(target/release/hollow-grove)"
printf '%s\n' "${witness_output}"

if [[ "${witness_output}" != "${CANONICAL_WITNESS}" ]]; then
    printf 'Witness output drifted from canonical form.\n' >&2
    exit 1
fi
