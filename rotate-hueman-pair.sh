#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
PAIR_STATE_PATH="$SCRIPT_DIR/artifacts/hueman_pair_state.json"

mkdir -p "$SCRIPT_DIR/artifacts"

if [[ ! -f "$PAIR_STATE_PATH" ]]; then
    cat >"$PAIR_STATE_PATH" <<'EOF'
{
  "schema_version": "0.2.0",
  "paired_window_mode": true,
  "diagonal_angle_degrees": 135,
  "spread_ratio": 0.25,
  "rotation_index": 0,
  "spread_index": 0,
  "focused_window": null
}
EOF
fi

tmpfile="$(mktemp)"
jq '
  .paired_window_mode = true
  | .rotation_index = (((.rotation_index // 0) + 1) % 4)
  | .diagonal_angle_degrees = ([135,225,315,45][.rotation_index])
' "$PAIR_STATE_PATH" >"$tmpfile"
mv "$tmpfile" "$PAIR_STATE_PATH"
