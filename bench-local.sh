#!/usr/bin/env bash

set -euo pipefail

cd /home/warren/hollow-grove

REPEAT_COUNT=10
CANONICAL_WITNESS=$'start Point\n↓\nTriway\n↓\nHollowGrove\n↓\nCurrentSeam\n↓\nAuraBeam\n↓\nlanded Point'

time_once_ms() {
    local start_ns end_ns
    start_ns="$(date +%s%N)"
    "$@" >/dev/null 2>&1
    end_ns="$(date +%s%N)"
    printf '%s\n' "$(((end_ns - start_ns) / 1000000))"
}

time_average_us() {
    local total_ns=0
    local start_ns end_ns _

    for _ in $(seq 1 "${REPEAT_COUNT}"); do
        start_ns="$(date +%s%N)"
        "$@" >/dev/null 2>&1
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

cargo fmt

echo "cargo test time (ms):"
test_time_ms="$(time_once_ms cargo test)"
printf '%s\n' "${test_time_ms}"

echo "cargo build --release time (ms):"
build_time_ms="$(time_once_ms cargo build --release)"
printf '%s\n' "${build_time_ms}"

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
