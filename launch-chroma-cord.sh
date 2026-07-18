#!/usr/bin/env bash

set -euo pipefail

CHROMA_CORD_ROOT="${CHROMA_CORD_ROOT:-/home/warren/chroma_cord}"
CHROMA_CORD_BIN="${CHROMA_CORD_BIN:-$CHROMA_CORD_ROOT/chroma_cord}"

if ! command -v kitty >/dev/null 2>&1; then
    printf 'kitty executable not found\n' >&2
    exit 1
fi

if [[ ! -x "$CHROMA_CORD_BIN" ]]; then
    printf 'chroma_cord executable not found: %s\n' "$CHROMA_CORD_BIN" >&2
    exit 1
fi

exec kitty \
    --class hollow-grove.chroma-cord \
    --title chroma_cord \
    "$CHROMA_CORD_BIN" tui "$@"
