#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="${HOLLOW_GROVE_ROOT:-$SCRIPT_DIR}"
BIN_DIR="${HOLLOW_GROVE_BIN_DIR:-$HOME/.local/lib/hollow-grove/bin}"
TARGET_DIR="${CARGO_TARGET_DIR:-$SCRIPT_DIR/target}"
[[ "$TARGET_DIR" = /* ]] || TARGET_DIR="$PWD/$TARGET_DIR"
export CARGO_TARGET_DIR="$TARGET_DIR"
LAUNCHER_BIN="$BIN_DIR/hollow-grove"

if [[ "${1:-}" == "--dev" ]]; then
    shift
    cargo build --manifest-path "$SCRIPT_DIR/Cargo.toml" \
        --bin hollow-grove --bin hollow_grove_runtime --bin hollow_grove_niri_bridge >/dev/null
    LAUNCHER_BIN="$TARGET_DIR/debug/hollow-grove"
    export HOLLOW_GROVE_DEV_BUILD=1
elif [[ ! -x "$LAUNCHER_BIN" && -x "$TARGET_DIR/release/hollow-grove" ]]; then
    LAUNCHER_BIN="$TARGET_DIR/release/hollow-grove"
fi

if [[ ! -x "$LAUNCHER_BIN" ]]; then
    printf 'release desktop launcher not installed; run %s/install-local-release.sh\n' "$SCRIPT_DIR" >&2
    exit 1
fi

cd "$ROOT_DIR"
exec "$LAUNCHER_BIN" desktop "$@"
