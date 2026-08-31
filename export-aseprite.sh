#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
SOURCE_DIR="$SCRIPT_DIR/hueman_godot/assets/source"
EXPORT_DIR="$SCRIPT_DIR/hueman_godot/assets/export"
ASEPRITE_BIN="${ASEPRITE_BIN:-/usr/bin/aseprite}"
BIN_DIR="${HOLLOW_GROVE_BIN_DIR:-$HOME/.local/lib/hollow-grove/bin}"
PALETTE_TOOL="${HOLLOW_GROVE_PALETTE_TOOL:-$BIN_DIR/visual_color_constitution}"
PALETTE_TOOL_ARGS=()
PALETTE_FILE="$SCRIPT_DIR/artifacts/visual_color_constitution.gpl"
REVISION_FILE="$SCRIPT_DIR/artifacts/aseprite_export_revision.json"
export_count=0

cleanup() {
    if [[ -n "${REVISION_TMP:-}" ]]; then
        rm -f -- "$REVISION_TMP"
    fi
}
trap cleanup EXIT

if [[ ! -x "$PALETTE_TOOL" && -x "$SCRIPT_DIR/target/release/hg-dev" ]]; then
    PALETTE_TOOL="$SCRIPT_DIR/target/release/hg-dev"
    PALETTE_TOOL_ARGS=(visual_color_constitution)
fi
if [[ ! -x "$PALETTE_TOOL" ]]; then
    printf 'release palette tool not installed; run %s/install-local-release.sh\n' "$SCRIPT_DIR" >&2
    exit 1
fi

mkdir -p "$SCRIPT_DIR/artifacts"
if [[ ! -f "$PALETTE_FILE" || "$PALETTE_TOOL" -nt "$PALETTE_FILE" ]]; then
    palette_tmp="$(mktemp --tmpdir="$SCRIPT_DIR/artifacts" .visual-palette.XXXXXX.gpl)"
    "$PALETTE_TOOL" "${PALETTE_TOOL_ARGS[@]}" aseprite-gpl >"$palette_tmp"
    mv "$palette_tmp" "$PALETTE_FILE"
fi

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
    export_count=$((export_count + 1))
}

publish_revision() {
    REVISION_TMP="$(mktemp --tmpdir="$SCRIPT_DIR/artifacts" .aseprite-revision.XXXXXX.json)"
    printf '{\n  "schema_version": "0.1.0",\n  "exported_files": %d,\n  "unix_time_s": %(%s)T\n}\n' \
        "$export_count" -1 >"$REVISION_TMP"
    mv "$REVISION_TMP" "$REVISION_FILE"
    REVISION_TMP=""
}

if [[ "$#" -gt 0 ]]; then
    for input_path in "$@"; do
        if [[ ! -f "$input_path" ]]; then
            printf 'missing source file: %s\n' "$input_path" >&2
            exit 1
        fi
        export_one "$(realpath "$input_path")"
    done
    publish_revision
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
publish_revision
