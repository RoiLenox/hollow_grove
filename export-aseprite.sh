#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
SOURCE_DIR="$SCRIPT_DIR/hueman_godot/assets/source"
EXPORT_DIR="$SCRIPT_DIR/hueman_godot/assets/export"
ASEPRITE_BIN="${ASEPRITE_BIN:-/usr/bin/aseprite}"
PALETTE_FILE="$(mktemp --tmpdir hollow-grove-visual-palette.XXXXXX.gpl)"

cleanup() {
    rm -f -- "$PALETTE_FILE"
}
trap cleanup EXIT

cargo run --quiet \
    --manifest-path "$SCRIPT_DIR/Cargo.toml" \
    --bin visual_color_constitution \
    -- aseprite-gpl >"$PALETTE_FILE"

if [[ ! -x "$ASEPRITE_BIN" ]]; then
    if command -v aseprite >/dev/null 2>&1; then
        ASEPRITE_BIN="$(command -v aseprite)"
    else
        printf 'aseprite executable not found\n' >&2
        exit 1
    fi
fi

mkdir -p "$SOURCE_DIR" "$EXPORT_DIR"

export_one() {
    local source_file="$1"
    local relative_path
    local target_base
    local target_dir

    relative_path="${source_file#$SOURCE_DIR/}"
    target_base="${relative_path%.*}"
    target_dir="$EXPORT_DIR/$(dirname -- "$target_base")"

    mkdir -p "$target_dir"

    "$ASEPRITE_BIN" -b "$source_file" \
        --palette "$PALETTE_FILE" \
        --sheet "$EXPORT_DIR/$target_base.png" \
        --data "$EXPORT_DIR/$target_base.json" \
        --format json-array \
        --list-tags \
        --list-slices \
        --sheet-pack

    printf 'exported %s -> %s.png\n' "$relative_path" "$target_base"
}

if [[ "$#" -gt 0 ]]; then
    for input_path in "$@"; do
        if [[ ! -f "$input_path" ]]; then
            printf 'missing source file: %s\n' "$input_path" >&2
            exit 1
        fi
        export_one "$(realpath "$input_path")"
    done
    exit 0
fi

mapfile -t aseprite_files < <(find "$SOURCE_DIR" -type f \( -name '*.aseprite' -o -name '*.ase' \) | sort)

if [[ "${#aseprite_files[@]}" -eq 0 ]]; then
    printf 'no aseprite files found in %s\n' "$SOURCE_DIR" >&2
    exit 0
fi

for source_file in "${aseprite_files[@]}"; do
    export_one "$source_file"
done
