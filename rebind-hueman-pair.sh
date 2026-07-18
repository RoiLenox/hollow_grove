#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
PAIR_STATE_PATH="$SCRIPT_DIR/artifacts/hueman_pair_state.json"
PREVIEW_IMAGE_PATH="$SCRIPT_DIR/artifacts/hueman_pair_preview.png"
PREVIEW_STATE_PATH="$SCRIPT_DIR/artifacts/hueman_pair_preview_state.json"
APPLICATION_REGISTRY_PATH="$SCRIPT_DIR/artifacts/hollow_grove_application_registry.json"
REQUIRED_APPLICATION="${HOLLOW_GROVE_REQUIRE_APPLICATION:-}"
TARGET_WINDOW_ID="${HOLLOW_GROVE_WINDOW_ID:-}"

mkdir -p "$SCRIPT_DIR/artifacts"

if ! command -v niri >/dev/null 2>&1; then
    exit 0
fi

if ! command -v jq >/dev/null 2>&1; then
    exit 0
fi

if [[ -n "$TARGET_WINDOW_ID" ]]; then
    if [[ ! "$TARGET_WINDOW_ID" =~ ^[0-9]+$ ]]; then
        printf 'invalid HOLLOW_GROVE_WINDOW_ID: %s\n' "$TARGET_WINDOW_ID" >&2
        exit 2
    fi
    focused_window_json="$(niri msg -j windows 2>/dev/null | jq --argjson window_id "$TARGET_WINDOW_ID" '
        first(.[] | select(.id == $window_id)) // null
    ')"
else
    focused_window_json="$(niri msg -j focused-window 2>/dev/null || printf 'null')"
fi

focused_window_json="$(printf '%s\n' "$focused_window_json" | jq '
    if type == "object" then
        {
            id: (.id // null),
            title: (.title // null),
            app_id: (.app_id // .["app-id"] // null),
            output: (.output // .output_name // null),
            rect: (
                .geometry
                // .rect
                // .window_rect
                // (
                    if (.position != null and .size != null) then
                        {
                            x: (.position.x // 0),
                            y: (.position.y // 0),
                            width: (.size.width // .size.w // 0),
                            height: (.size.height // .size.h // 0)
                        }
                    else
                        {
                            x: (.x // 0),
                            y: (.y // 0),
                            width: (.width // .w // 0),
                            height: (.height // .h // 0)
                        }
                    end
                )
            )
        }
    else
        null
    end
')"
focused_app_id="$(printf '%s\n' "$focused_window_json" | jq -r '.app_id // ""')"

if [[ "$focused_window_json" == "null" ]]; then
    cat >"$PREVIEW_STATE_PATH" <<'EOF'
{
  "schema_version": "0.2.0",
  "status": "waiting",
  "detail": "rebind skipped because no focused window was available"
}
EOF
    exit 0
fi

if [[ -f "$APPLICATION_REGISTRY_PATH" ]]; then
    managed_application_json="$(jq --arg app_id "$focused_app_id" '
        first(.applications[]? | select(.window_app_id == $app_id)) // null
    ' "$APPLICATION_REGISTRY_PATH")"
else
    managed_application_json='null'
fi

if [[ "$managed_application_json" == "null" && "$focused_app_id" == hollow-grove.* ]]; then
    rm -f "$PREVIEW_IMAGE_PATH"
    jq -n --arg app_id "$focused_app_id" '
        {
            schema_version: "0.2.0",
            status: "rejected",
            detail: ("reserved managed window identity is not present in the application registry: " + $app_id)
        }
    ' >"$PREVIEW_STATE_PATH"
    exit 1
fi

if [[ -n "$REQUIRED_APPLICATION" ]]; then
    attached_name="$(printf '%s\n' "$managed_application_json" | jq -r '.canonical_name // ""')"
    if [[ "$attached_name" != "$REQUIRED_APPLICATION" ]]; then
        jq -n --arg required "$REQUIRED_APPLICATION" '
            {
                schema_version: "0.2.0",
                status: "rejected",
                detail: ("focused window is not the managed application " + $required)
            }
        ' >"$PREVIEW_STATE_PATH"
        exit 1
    fi
fi

if [[ -f "$PAIR_STATE_PATH" ]]; then
    existing_pair_state_json="$(cat "$PAIR_STATE_PATH")"
else
    existing_pair_state_json='{}'
fi

printf '%s\n' "$existing_pair_state_json" | jq \
    --argjson focused_window "$focused_window_json" \
    --argjson managed_application "$managed_application_json" '
    {
        schema_version: "0.2.0",
        paired_window_mode: true,
        sticky_actor: true,
        probe_source: (if $managed_application == null then "paired_window_center" else "application_world_anchor" end),
        binding_source: "rebind-hueman-pair",
        binding_status: (if $managed_application == null then "binding" else "attaching" end),
        diagonal_angle_degrees: (.diagonal_angle_degrees // 135),
        spread_ratio: (.spread_ratio // 0.25),
        rotation_index: (.rotation_index // 0),
        spread_index: (.spread_index // 0),
        active_output: null,
        normalized: (
            if $managed_application == null then null else {
                center: $managed_application.world_anchor.normalized,
                rect: null
            } end
        ),
        application_attachment: (
            if $managed_application == null then null else {
                control_plane: "hollow_grove",
                application_id: $managed_application.id,
                canonical_name: $managed_application.canonical_name,
                kind: $managed_application.kind,
                lifecycle: "attached",
                world_anchor: $managed_application.world_anchor,
                privacy: $managed_application.privacy,
                authority: $managed_application.authority,
                scope: $managed_application.scope
            } end
        ),
        focused_window: $focused_window
    }
' >"$PAIR_STATE_PATH"

rm -f "$PREVIEW_IMAGE_PATH"

if [[ "$managed_application_json" == "null" ]]; then
    jq -n '{schema_version: "0.2.0", status: "rebinding", detail: "paired window rebind requested"}' >"$PREVIEW_STATE_PATH"
else
    jq -n '{schema_version: "0.2.0", status: "masked", detail: "managed clinical surface uses semantic-only projection"}' >"$PREVIEW_STATE_PATH"
fi
