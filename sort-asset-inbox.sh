#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
ASSET_DIR="$SCRIPT_DIR/hueman_godot/assets"
INBOX_DIR="$ASSET_DIR/reference/inbox"

usage() {
    cat <<'EOF'
Usage:
  ./sort-asset-inbox.sh list
  ./sort-asset-inbox.sh move <file> <bucket>

Buckets:
  ref-characters
  ref-tilesets
  ref-ui
  ref-world
  ref-moodboards
  ref-palettes
  third-party

Examples:
  ./sort-asset-inbox.sh list
  ./sort-asset-inbox.sh move town-ui.png ref-ui
  ./sort-asset-inbox.sh move cc0-pack.zip third-party
EOF
}

bucket_dir() {
    case "${1:-}" in
        ref-characters) printf '%s\n' "$ASSET_DIR/reference/characters" ;;
        ref-tilesets) printf '%s\n' "$ASSET_DIR/reference/tilesets" ;;
        ref-ui) printf '%s\n' "$ASSET_DIR/reference/ui" ;;
        ref-world) printf '%s\n' "$ASSET_DIR/reference/world" ;;
        ref-moodboards) printf '%s\n' "$ASSET_DIR/reference/moodboards" ;;
        ref-palettes) printf '%s\n' "$ASSET_DIR/reference/palettes" ;;
        third-party) printf '%s\n' "$ASSET_DIR/third_party" ;;
        *) return 1 ;;
    esac
}

list_inbox() {
    printf 'Inbox: %s\n' "$INBOX_DIR"
    find "$INBOX_DIR" -maxdepth 1 -type f ! -name '.*' | sort
}

move_file() {
    local file_name="$1"
    local bucket="$2"
    local source_path="$INBOX_DIR/$file_name"
    local target_dir
    local target_path

    if [[ ! -f "$source_path" ]]; then
        printf 'missing inbox file: %s\n' "$file_name" >&2
        exit 1
    fi

    if ! target_dir="$(bucket_dir "$bucket")"; then
        printf 'unknown bucket: %s\n' "$bucket" >&2
        usage >&2
        exit 1
    fi

    mkdir -p "$target_dir"
    target_path="$target_dir/$file_name"

    if [[ -e "$target_path" ]]; then
        printf 'target already exists: %s\n' "$target_path" >&2
        exit 1
    fi

    mv "$source_path" "$target_path"
    printf 'moved %s -> %s\n' "$source_path" "$target_path"
}

mkdir -p "$INBOX_DIR"

command_name="${1:-}"

case "$command_name" in
    list)
        list_inbox
        ;;
    move)
        if [[ "$#" -ne 3 ]]; then
            usage >&2
            exit 1
        fi
        move_file "$2" "$3"
        ;;
    *)
        usage >&2
        exit 1
        ;;
esac
