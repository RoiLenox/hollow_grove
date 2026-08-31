#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
SOURCE_DIR="$SCRIPT_DIR/hueman_godot/assets/source"
FALLBACK_INTERVAL_SECONDS="${ASEPRITE_WATCH_INTERVAL:-5}"

mkdir -p "$SOURCE_DIR"

if find "$SOURCE_DIR" -type f \( -name '*.aseprite' -o -name '*.ase' \) -print -quit | rg -q .; then
    "$SCRIPT_DIR/export-aseprite.sh"
fi

printf 'watching %s for Aseprite changes\n' "$SOURCE_DIR"

WATCHER_BIN="${HOLLOW_GROVE_NIRI_BIN:-${HOLLOW_GROVE_BIN_DIR:-$HOME/.local/lib/hollow-grove/bin}/hollow-niri}"
if [[ ! -x "$WATCHER_BIN" && -x "$SCRIPT_DIR/target/release/hollow-niri" ]]; then
    WATCHER_BIN="$SCRIPT_DIR/target/release/hollow-niri"
fi
if [[ -x "$WATCHER_BIN" ]]; then
    exec "$WATCHER_BIN" asset-watch --root "$SCRIPT_DIR"
fi

if command -v inotifywait >/dev/null 2>&1; then
    inotifywait --monitor --recursive --quiet \
        --event close_write --event moved_to --event create \
        --format '%w%f' "$SOURCE_DIR" |
    while IFS= read -r changed_path; do
        case "$changed_path" in
            *.aseprite|*.ase)
                [[ -f "$changed_path" ]] && "$SCRIPT_DIR/export-aseprite.sh" "$changed_path"
                ;;
        esac
    done
    exit 0
fi

printf 'inotifywait is unavailable; using a %ss metadata fallback\n' "$FALLBACK_INTERVAL_SECONDS" >&2
declare -A known_signatures=()

scan_sources() {
    find "$SOURCE_DIR" -type f \( -name '*.aseprite' -o -name '*.ase' \) \
        -printf '%P\t%T@:%s\n' | sort
}

while IFS=$'\t' read -r relative_path signature; do
    [[ -n "$relative_path" ]] && known_signatures["$relative_path"]="$signature"
done < <(scan_sources)

while true; do
    sleep "$FALLBACK_INTERVAL_SECONDS"
    declare -A current_signatures=()
    changed_files=()
    while IFS=$'\t' read -r relative_path signature; do
        [[ -n "$relative_path" ]] || continue
        current_signatures["$relative_path"]="$signature"
        if [[ "${known_signatures[$relative_path]:-}" != "$signature" ]]; then
            changed_files+=("$SOURCE_DIR/$relative_path")
        fi
    done < <(scan_sources)
    known_signatures=()
    for relative_path in "${!current_signatures[@]}"; do
        known_signatures["$relative_path"]="${current_signatures[$relative_path]}"
    done
    if [[ "${#changed_files[@]}" -gt 0 ]]; then
        "$SCRIPT_DIR/export-aseprite.sh" "${changed_files[@]}"
    fi
done
