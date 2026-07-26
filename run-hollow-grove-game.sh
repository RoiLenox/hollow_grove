#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
GODOT_BIN="${GODOT_BIN:-}"
SERVICE_PID=""

if [[ -z "$GODOT_BIN" ]]; then
    if command -v godot4 >/dev/null 2>&1; then
        GODOT_BIN="godot4"
    elif command -v godot >/dev/null 2>&1; then
        GODOT_BIN="godot"
    else
        printf 'godot executable not found\n' >&2
        exit 1
    fi
fi

cleanup() {
    if [[ -n "$SERVICE_PID" ]]; then
        kill "$SERVICE_PID" 2>/dev/null || true
        wait "$SERVICE_PID" 2>/dev/null || true
    fi
}

trap cleanup EXIT INT TERM

export HOLLOW_GROVE_ROOT="$SCRIPT_DIR"

cargo run --quiet --manifest-path "$SCRIPT_DIR/Cargo.toml" \
    --bin hollow_grove_game_service -- \
    --listen 127.0.0.1:47819 \
    --session session.hollow-grove.local \
    --save-root "$SCRIPT_DIR/artifacts/gameplay-saves" \
    --world-root "$SCRIPT_DIR" &
SERVICE_PID="$!"

"$GODOT_BIN" --path "$SCRIPT_DIR/hueman_godot" \
    res://scenes/retro_overworld.tscn "$@"
