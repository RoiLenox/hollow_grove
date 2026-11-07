#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
GODOT_BIN="${GODOT_BIN:-}"
SERVICE_PID=""
ENTRY_SCENE="res://scenes/retro_overworld.tscn"
HUEMAN_ENTRY=0

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

BIN_DIR="${HOLLOW_GROVE_BIN_DIR:-$HOME/.local/lib/hollow-grove/bin}"
SERVICE_BIN="$BIN_DIR/hollow_grove_game_service"
if [[ "${1:-}" == "--hueman" ]]; then
    HUEMAN_ENTRY=1
    shift
fi
if [[ "${1:-}" == "--dev" ]]; then
    shift
    cargo build --manifest-path "$SCRIPT_DIR/Cargo.toml" --bin hollow_grove_game_service >/dev/null
    SERVICE_BIN="$SCRIPT_DIR/target/debug/hollow_grove_game_service"
elif [[ ! -x "$SERVICE_BIN" && -x "$SCRIPT_DIR/target/release/hollow_grove_game_service" ]]; then
    SERVICE_BIN="$SCRIPT_DIR/target/release/hollow_grove_game_service"
fi
if [[ ! -x "$SERVICE_BIN" ]]; then
    printf 'release game service not installed; run %s/install-local-release.sh\n' "$SCRIPT_DIR" >&2
    exit 1
fi

# The opt-in character entry uses the same service and overworld with its own
# session namespace, preserving pre-existing local-world slots.
if [[ "${1:-}" == "--hueman" ]]; then
    HUEMAN_ENTRY=1
    shift
fi
if [[ "$HUEMAN_ENTRY" == "1" ]]; then
    if [[ "$SERVICE_BIN" != "$SCRIPT_DIR/target/debug/hollow_grove_game_service" ]]; then
        printf 'HUEMAN-02 source development requires --dev (no package installation needed)\n' >&2
        exit 1
    fi
    ENTRY_SCENE="res://tools/hueman_02/entry.tscn"
    export HOLLOW_GROVE_SESSION="session.hollow-grove.hueman"
    export HOLLOW_GROVE_CHARACTER_ENTRY=1
    export HOLLOW_GROVE_HOST=127.0.0.1
    SAVE_ROOT="${HOLLOW_GROVE_CHARACTER_SAVE_ROOT:-$SCRIPT_DIR/artifacts/gameplay-saves}"
    SERVICE_LOG="$(mktemp "${TMPDIR:-/tmp}/hollow-grove-hueman.XXXXXX.log")"
    "$SERVICE_BIN" --listen 127.0.0.1:0 --session "$HOLLOW_GROVE_SESSION" \
        --save-root "$SAVE_ROOT" --world-root "$SCRIPT_DIR" 2>"$SERVICE_LOG" &
    SERVICE_PID="$!"
    for ((attempt=0; attempt<100; attempt++)); do
        HOLLOW_GROVE_PORT="$(sed -nE 's/.*listening on 127\.0\.0\.1:([0-9]+) for session.*/\1/p' "$SERVICE_LOG" | head -n 1)"
        if [[ -n "$HOLLOW_GROVE_PORT" ]]; then break; fi
        if ! kill -0 "$SERVICE_PID" 2>/dev/null; then
            cat "$SERVICE_LOG" >&2
            exit 1
        fi
        sleep 0.05
    done
    if [[ -z "$HOLLOW_GROVE_PORT" ]]; then
        printf 'Hueman service did not become ready; see %s\n' "$SERVICE_LOG" >&2
        exit 1
    fi
    export HOLLOW_GROVE_PORT
    # Source entry must use the local service, not an inherited shared session.
    unset HOLLOW_GROVE_SHARED_CONFIG
else
    "$SERVICE_BIN" \
        --listen 127.0.0.1:47819 \
        --session session.hollow-grove.local \
        --save-root "$SCRIPT_DIR/artifacts/gameplay-saves" \
        --world-root "$SCRIPT_DIR" &
    SERVICE_PID="$!"
fi

"$GODOT_BIN" --path "$SCRIPT_DIR/hueman_godot" "$ENTRY_SCENE" "$@"
