#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

GODOT_BIN="${GODOT_BIN:-}"
CARGO_BIN="${CARGO_BIN:-}"
TARGET_DIR="${CARGO_TARGET_DIR:-$SCRIPT_DIR/target}"
SERVICE_BIN="$TARGET_DIR/debug/hollow_grove_game_service"
SERVICE_PID=""
SERVICE_PORT=""

if [[ -z "$GODOT_BIN" ]]; then
    if command -v godot4 >/dev/null 2>&1; then
        GODOT_BIN="godot4"
    elif command -v godot >/dev/null 2>&1; then
        GODOT_BIN="godot"
    else
        printf 'godot executable not found; playable lifecycle verification is required\n' >&2
        exit 1
    fi
fi

if [[ -z "$CARGO_BIN" ]]; then
    if command -v cargo >/dev/null 2>&1; then
        CARGO_BIN="cargo"
    elif [[ -x "${CARGO_HOME:-$HOME/.cargo}/bin/cargo" ]]; then
        CARGO_BIN="${CARGO_HOME:-$HOME/.cargo}/bin/cargo"
    else
        printf 'cargo executable not found; playable lifecycle verification is required\n' >&2
        exit 1
    fi
fi

LIFECYCLE_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hollow-grove-playable-lifecycle.XXXXXX")"
SNAPSHOT_PATH="$LIFECYCLE_DIR/pre-restart-view.json"
SAVE_ROOT="$LIFECYCLE_DIR/saves"

stop_service() {
    if [[ -n "$SERVICE_PID" ]]; then
        kill "$SERVICE_PID" 2>/dev/null || true
        wait "$SERVICE_PID" 2>/dev/null || true
        SERVICE_PID=""
    fi
}

cleanup() {
    stop_service
    rm -rf "$LIFECYCLE_DIR"
}

trap cleanup EXIT INT TERM

start_service() {
    local phase="$1"
    local service_log="$LIFECYCLE_DIR/service-$phase.log"
    "$SERVICE_BIN" \
        --listen 127.0.0.1:0 \
        --session session.lifecycle \
        --save-root "$SAVE_ROOT" \
        --world-root "$SCRIPT_DIR" \
        >"$LIFECYCLE_DIR/service-$phase.stdout" 2>"$service_log" &
    SERVICE_PID="$!"
    SERVICE_PORT=""

    local attempt
    for ((attempt = 0; attempt < 100; attempt++)); do
        SERVICE_PORT="$(sed -nE \
            's/.*listening on 127\.0\.0\.1:([0-9]+) for session.*/\1/p' \
            "$service_log" | head -n 1)"
        if [[ -n "$SERVICE_PORT" ]]; then
            return
        fi
        if ! kill -0 "$SERVICE_PID" 2>/dev/null; then
            printf 'gameplay service stopped before becoming ready\n' >&2
            sed -n '1,120p' "$service_log" >&2
            exit 1
        fi
        sleep 0.05
    done

    printf 'gameplay service did not become ready\n' >&2
    sed -n '1,120p' "$service_log" >&2
    exit 1
}

run_godot_phase() {
    local phase="$1"
    HOLLOW_GROVE_LIFECYCLE_PHASE="$phase" \
    HOLLOW_GROVE_LIFECYCLE_PORT="$SERVICE_PORT" \
    HOLLOW_GROVE_LIFECYCLE_SNAPSHOT="$SNAPSHOT_PATH" \
        "$GODOT_BIN" --headless --path "$SCRIPT_DIR/hueman_godot" \
        --script res://tests/playable_lifecycle_smoke.gd
}

if [[ "${SKIP_BUILD:-0}" != "1" ]]; then
    "$CARGO_BIN" build --quiet --bin hollow_grove_game_service
fi
if [[ ! -x "$SERVICE_BIN" ]]; then
    printf 'gameplay service binary is missing: %s\n' "$SERVICE_BIN" >&2
    exit 1
fi

mkdir -p "$SAVE_ROOT"

start_service save
run_godot_phase save
stop_service

start_service load
run_godot_phase load
stop_service

printf 'Hollow Grove playable lifecycle verification passed\n'
