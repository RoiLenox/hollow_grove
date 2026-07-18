#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
PAIR_STATE_PATH="$SCRIPT_DIR/artifacts/hueman_pair_state.json"
PREVIEW_IMAGE_PATH="$SCRIPT_DIR/artifacts/hueman_pair_preview.png"
PREVIEW_STATE_PATH="$SCRIPT_DIR/artifacts/hueman_pair_preview_state.json"

mkdir -p "$SCRIPT_DIR/artifacts"

if [[ -f "$PAIR_STATE_PATH" ]]; then
    existing_pair_state_json="$(cat "$PAIR_STATE_PATH")"
else
    existing_pair_state_json='{}'
fi

printf '%s\n' "$existing_pair_state_json" | jq '
    {
        schema_version: "0.2.0",
        paired_window_mode: false,
        sticky_actor: false,
        probe_source: "focused_window_center",
        binding_source: "clear-hueman-pair",
        binding_status: "released",
        diagonal_angle_degrees: (.diagonal_angle_degrees // 135),
        spread_ratio: (.spread_ratio // 0.25),
        rotation_index: (.rotation_index // 0),
        spread_index: (.spread_index // 0),
        active_output: null,
        normalized: null,
        application_attachment: null,
        focused_window: null
    }
' >"$PAIR_STATE_PATH"

rm -f "$PREVIEW_IMAGE_PATH"

cat >"$PREVIEW_STATE_PATH" <<'EOF'
{
  "schema_version": "0.2.0",
  "status": "released",
  "detail": "paired window released"
}
EOF
