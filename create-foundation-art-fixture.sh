#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
ASEPRITE_BIN="${ASEPRITE_BIN:-/usr/bin/aseprite}"
OUTPUT="$SCRIPT_DIR/hueman_godot/assets/source/prototypes/foundation_pipeline_probe.aseprite"
PALETTE="$SCRIPT_DIR/src/constitutional/hollow_grove_visual_color_palette.json"

if [[ ! -x "$ASEPRITE_BIN" ]]; then
    ASEPRITE_BIN="$(command -v aseprite)"
fi

mkdir -p "$(dirname -- "$OUTPUT")"
"$ASEPRITE_BIN" --batch \
    --script-param "output=$OUTPUT" \
    --script-param "palette=$PALETTE" \
    --script "$SCRIPT_DIR/tools/create-foundation-art-fixture.lua"

printf 'created %s\n' "$OUTPUT"
