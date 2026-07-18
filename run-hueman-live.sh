#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
WATCH_PID=""

cleanup() {
    if [[ -n "$WATCH_PID" ]]; then
        kill "$WATCH_PID" 2>/dev/null || true
        wait "$WATCH_PID" 2>/dev/null || true
    fi
}

trap cleanup EXIT INT TERM

"$SCRIPT_DIR/watch-aseprite.sh" &
WATCH_PID="$!"

"$SCRIPT_DIR/run-hueman-godot.sh" "$@"
