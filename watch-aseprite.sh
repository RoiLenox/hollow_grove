#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
SOURCE_DIR="$SCRIPT_DIR/hueman_godot/assets/source"
STATE_DIR="$SCRIPT_DIR/artifacts/aseprite-watch"
INTERVAL_SECONDS="${ASEPRITE_WATCH_INTERVAL:-1}"

mkdir -p "$SOURCE_DIR" "$STATE_DIR"

checksum_tree() {
    find "$SOURCE_DIR" -type f \( -name '*.aseprite' -o -name '*.ase' \) -printf '%P\t%T@\t%s\n' | sort
}

last_snapshot="$(checksum_tree || true)"

if [[ -n "$last_snapshot" ]]; then
    "$SCRIPT_DIR/export-aseprite.sh"
fi

printf 'watching %s for aseprite changes\n' "$SOURCE_DIR"

while true; do
    sleep "$INTERVAL_SECONDS"
    current_snapshot="$(checksum_tree || true)"
    if [[ "$current_snapshot" != "$last_snapshot" ]]; then
        last_snapshot="$current_snapshot"
        "$SCRIPT_DIR/export-aseprite.sh"
    fi
done
