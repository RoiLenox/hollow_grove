#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="${HOLLOW_GROVE_ROOT:-$SCRIPT_DIR}"
BIN_DIR="${HOLLOW_GROVE_BIN_DIR:-$HOME/.local/lib/hollow-grove/bin}"
TARGET_DIR="${CARGO_TARGET_DIR:-$SCRIPT_DIR/target}"
[[ "$TARGET_DIR" = /* ]] || TARGET_DIR="$PWD/$TARGET_DIR"
export CARGO_TARGET_DIR="$TARGET_DIR"
RUNTIME_BIN="$BIN_DIR/hollow_grove_runtime"

if [[ "${1:-}" == "--dev" ]]; then
    shift
    cargo build --manifest-path "$SCRIPT_DIR/Cargo.toml" --bin hollow_grove_runtime >/dev/null
    RUNTIME_BIN="$TARGET_DIR/debug/hollow_grove_runtime"
elif [[ ! -x "$RUNTIME_BIN" && -x "$TARGET_DIR/release/hollow_grove_runtime" ]]; then
    RUNTIME_BIN="$TARGET_DIR/release/hollow_grove_runtime"
fi

if [[ ! -x "$RUNTIME_BIN" ]]; then
    printf 'release runtime not installed; run %s/install-local-release.sh\n' "$SCRIPT_DIR" >&2
    exit 1
fi

cd "$ROOT_DIR"
if [[ $# -eq 0 ]]; then
    exec "$RUNTIME_BIN" --cycles 5 --interval-ms 1000
fi
exec "$RUNTIME_BIN" "$@"
