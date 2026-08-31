#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
BROKER_BIN="${HOLLOW_GROVE_NIRI_BIN:-${HOLLOW_GROVE_BIN_DIR:-$HOME/.local/lib/hollow-grove/bin}/hollow-niri}"
if [[ ! -x "$BROKER_BIN" && -x "$SCRIPT_DIR/target/release/hollow-niri" ]]; then
    BROKER_BIN="$SCRIPT_DIR/target/release/hollow-niri"
fi
if [[ ! -x "$BROKER_BIN" ]]; then
    printf 'optimized Hollow Niri broker not found; run ./install-local-release.sh\n' >&2
    exit 1
fi

exec "$BROKER_BIN" broker --root "$SCRIPT_DIR" \
    --preview-fps "${HUEMAN_PAIR_PREVIEW_FPS:-1}"
