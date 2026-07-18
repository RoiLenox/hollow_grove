#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
SOURCE_DIR="$SCRIPT_DIR/hueman_godot/assets/source"
EXPORT_DIR="$SCRIPT_DIR/hueman_godot/assets/export"

mapfile -t source_files < <(find "$SOURCE_DIR" -type f \( -name '*.aseprite' -o -name '*.ase' \) | sort)
mapfile -t export_pngs < <(find "$EXPORT_DIR" -type f -name '*.png' | sort)
mapfile -t export_jsons < <(find "$EXPORT_DIR" -type f -name '*.json' | sort)

printf 'Hueman asset pipeline status\n'
printf 'source dir: %s\n' "$SOURCE_DIR"
printf 'export dir: %s\n' "$EXPORT_DIR"
printf '\n'
printf 'sources: %s\n' "${#source_files[@]}"
printf 'png exports: %s\n' "${#export_pngs[@]}"
printf 'json exports: %s\n' "${#export_jsons[@]}"
printf '\n'

missing_exports=0
for source_path in "${source_files[@]}"; do
    relative_path="${source_path#$SOURCE_DIR/}"
    base_path="${relative_path%.*}"
    png_path="$EXPORT_DIR/$base_path.png"
    json_path="$EXPORT_DIR/$base_path.json"

    if [[ ! -f "$png_path" || ! -f "$json_path" ]]; then
        ((missing_exports+=1))
        printf 'missing export: %s\n' "$relative_path"
    fi
done

orphan_exports=0
for png_path in "${export_pngs[@]}"; do
    relative_path="${png_path#$EXPORT_DIR/}"
    base_path="${relative_path%.png}"
    source_aseprite="$SOURCE_DIR/$base_path.aseprite"
    source_ase="$SOURCE_DIR/$base_path.ase"

    if [[ ! -f "$source_aseprite" && ! -f "$source_ase" ]]; then
        ((orphan_exports+=1))
        printf 'orphan export: %s\n' "$relative_path"
    fi
done

printf '\n'
printf 'missing exports: %s\n' "$missing_exports"
printf 'orphan exports: %s\n' "$orphan_exports"
