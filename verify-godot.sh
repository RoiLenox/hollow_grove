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

godot_check() {
    local output
    if ! output=$("$GODOT_BIN" "$@" 2>&1); then
        printf '%s\n' "$output" >&2
        return 1
    fi
    printf '%s\n' "$output"
    # Godot can exit zero after a GDScript assertion or parse error.
    if [[ "$output" == *"ERROR:"* ]]; then return 1; fi
}

python3 "$SCRIPT_DIR/tools/chromatic_growth.py" project --check
python3 "$SCRIPT_DIR/tools/verify_hueman_creator.py" --godot "$GODOT_BIN"

godot_check --headless --path "$SCRIPT_DIR/hueman_godot" \
    --script res://tools/chromatic_swatch_preview.gd -- --validate-only
godot_check --headless --path "$SCRIPT_DIR/hueman_godot" \
    --script res://tools/chromatic_swatch_preview.gd -- --study --validate-only
godot_check --headless --path "$SCRIPT_DIR/hueman_godot" \
    --script res://tools/chromatic_swatch_preview.gd -- --growth --validate-only

godot_check --headless --path "$SCRIPT_DIR/hueman_godot" \
    --script res://tests/runtime_client_smoke.gd
godot_check --headless --path "$SCRIPT_DIR/hueman_godot" \
    --script res://tests/performance_policy_smoke.gd
godot_check --headless --path "$SCRIPT_DIR/hueman_godot" \
    --script res://tests/runtime_client_transport_smoke.gd
godot_check --headless --path "$SCRIPT_DIR/hueman_godot" \
    --script res://tests/retro_overworld_semantics_smoke.gd

godot_check --headless --path "$SCRIPT_DIR/hueman_godot" \
    --script res://tests/survey_frame_smoke.gd

godot_check --headless --path "$SCRIPT_DIR/hueman_godot" \
    --script res://tests/hueman_save_recovery_smoke.gd

printf 'Hollow Grove Godot smoke verification passed\n'
