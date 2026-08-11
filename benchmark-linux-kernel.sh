#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
LABEL=""
WARMUP="${WARMUP:-25}"
SAMPLES="${SAMPLES:-100}"

usage() {
    printf 'usage: %s [--label NAME] [--warmup N] [--samples N]\n' "$0"
}

while (( $# > 0 )); do
    case "$1" in
        --label)
            LABEL="${2:-}"
            shift 2
            ;;
        --warmup)
            WARMUP="${2:-}"
            shift 2
            ;;
        --samples)
            SAMPLES="${2:-}"
            shift 2
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            printf 'unknown option: %s\n' "$1" >&2
            usage >&2
            exit 2
            ;;
    esac
done

if [[ ! "$WARMUP" =~ ^[1-9][0-9]*$ || ! "$SAMPLES" =~ ^[1-9][0-9]*$ ]]; then
    printf 'warmup and samples must be positive integers\n' >&2
    exit 2
fi
if [[ -z "$LABEL" ]]; then
    LABEL="$(uname -r)"
fi
LABEL="$(printf '%s' "$LABEL" | tr -cs 'A-Za-z0-9._-' '_')"
if [[ -z "$LABEL" ]]; then
    printf 'benchmark label contains no usable characters\n' >&2
    exit 2
fi

timestamp="$(date -u +%Y%m%dT%H%M%SZ)"
output_root="$SCRIPT_DIR/artifacts/kernel-benchmarks/$LABEL/$timestamp"
mkdir -p "$output_root"

cd "$SCRIPT_DIR"
cargo build --release --locked --offline --features dev-tools \
    --bin hollow-grove --bin current_synthesis_benchmark

governor="unavailable"
energy_preference="unavailable"
amd_pstate="unavailable"
platform_profile="unavailable"
power_profile="unavailable"
[[ -r /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor ]] \
    && governor="$(< /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor)"
[[ -r /sys/devices/system/cpu/cpu0/cpufreq/energy_performance_preference ]] \
    && energy_preference="$(< /sys/devices/system/cpu/cpu0/cpufreq/energy_performance_preference)"
[[ -r /sys/devices/system/cpu/amd_pstate/status ]] \
    && amd_pstate="$(< /sys/devices/system/cpu/amd_pstate/status)"
[[ -r /sys/firmware/acpi/platform_profile ]] \
    && platform_profile="$(< /sys/firmware/acpi/platform_profile)"
if command -v powerprofilesctl >/dev/null 2>&1; then
    power_profile="$(powerprofilesctl get 2>/dev/null || printf unavailable)"
fi

{
    printf 'label=%s\n' "$LABEL"
    printf 'timestamp_utc=%s\n' "$timestamp"
    printf 'git_commit=%s\n' "$(git rev-parse HEAD)"
    printf 'kernel=%s\n' "$(uname -r)"
    printf 'kernel_build=%s\n' "$(uname -v)"
    printf 'architecture=%s\n' "$(uname -m)"
    printf 'cpu=%s\n' "$(lscpu | sed -n 's/^Model name:[[:space:]]*//p' | head -n1)"
    printf 'logical_cpus=%s\n' "$(getconf _NPROCESSORS_ONLN)"
    printf 'scaling_governor=%s\n' "$governor"
    printf 'energy_performance_preference=%s\n' "$energy_preference"
    printf 'amd_pstate=%s\n' "$amd_pstate"
    printf 'platform_profile=%s\n' "$platform_profile"
    printf 'power_profile=%s\n' "$power_profile"
    printf 'rustc=%s\n' "$(rustc --version)"
    printf 'warmup=%s\n' "$WARMUP"
    printf 'samples=%s\n' "$SAMPLES"
} >"$output_root/environment.txt"

benchmark_start_ns="$(date +%s%N)"
target/release/current_synthesis_benchmark \
    --warmup "$WARMUP" --samples "$SAMPLES" --no-write --quiet
benchmark_end_ns="$(date +%s%N)"
printf 'wall_elapsed_ns=%s\n' "$((benchmark_end_ns - benchmark_start_ns))" \
    >"$output_root/wall-time.txt"

cp artifacts/current_synthesis_benchmark.md \
    artifacts/current_synthesis_benchmark.json \
    artifacts/current_synthesis_benchmark_release.md \
    "$output_root/"
sha256sum target/release/hollow-grove \
    target/release/current_synthesis_benchmark \
    >"$output_root/binary-sha256.txt"
(
    cd "$output_root"
    sha256sum environment.txt wall-time.txt current_synthesis_benchmark.md \
        current_synthesis_benchmark.json current_synthesis_benchmark_release.md \
        binary-sha256.txt >SHA256SUMS
)

printf 'kernel benchmark recorded: %s\n' "$output_root"
