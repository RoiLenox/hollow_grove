#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
GODOT_BIN="${GODOT_BIN:-}"

if [[ -z "$GODOT_BIN" ]]; then
    if command -v godot4 >/dev/null 2>&1; then
        GODOT_BIN="godot4"
    elif command -v godot >/dev/null 2>&1; then
        GODOT_BIN="godot"
    else
        printf 'godot executable not found; Godot verification is required\n' >&2
        exit 1
    fi
fi

if ! grep -Fqx 'export/convert_text_resources_to_binary=false' \
    "$SCRIPT_DIR/hueman_godot/project.godot"; then
    printf 'Hueman export must retain text resources for reproducible scene bytes\n' >&2
    exit 1
fi

"$GODOT_BIN" --headless --path "$SCRIPT_DIR/hueman_godot" \
    --script res://tests/runtime_client_smoke.gd
"$GODOT_BIN" --headless --path "$SCRIPT_DIR/hueman_godot" \
    --script res://tests/retro_overworld_semantics_smoke.gd

printf 'Hollow Grove Godot smoke verification passed\n'
