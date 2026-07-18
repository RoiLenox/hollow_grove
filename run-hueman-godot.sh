#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
GODOT_BIN="${GODOT_BIN:-}"
PAIR_STATE_PATH="$SCRIPT_DIR/artifacts/hueman_pair_state.json"
APPLICATION_REGISTRY_PATH="$SCRIPT_DIR/artifacts/hollow_grove_application_registry.json"
PREVIEW_IMAGE_PATH="$SCRIPT_DIR/artifacts/hueman_pair_preview.png"
PREVIEW_PID=""
SYNC_PID=""

if [[ -z "$GODOT_BIN" ]]; then
    if command -v godot4 >/dev/null 2>&1; then
        GODOT_BIN="godot4"
    elif command -v godot >/dev/null 2>&1; then
        GODOT_BIN="godot"
    else
        printf 'godot executable not found\n' >&2
        exit 1
    fi
fi

export HOLLOW_GROVE_ROOT="$SCRIPT_DIR"

mkdir -p "$SCRIPT_DIR/artifacts"

if command -v niri >/dev/null 2>&1; then
    focused_window_json="$(niri msg -j focused-window 2>/dev/null || printf 'null')"
else
    focused_window_json='null'
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

if [[ -f "$APPLICATION_REGISTRY_PATH" ]]; then
    managed_application_json="$(jq --arg app_id "$focused_app_id" '
        first(.applications[]? | select(.window_app_id == $app_id)) // null
    ' "$APPLICATION_REGISTRY_PATH")"
else
    managed_application_json='null'
fi

if [[ "$managed_application_json" == "null" && "$focused_app_id" == hollow-grove.* ]]; then
    rm -f "$PREVIEW_IMAGE_PATH"
    printf 'reserved managed window identity is not present in the application registry: %s\n' "$focused_app_id" >&2
    exit 1
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
        binding_source: "run-hueman-godot",
        binding_status: (if $managed_application == null then "binding" else "attaching" end),
        diagonal_angle_degrees: (.diagonal_angle_degrees // 135),
        spread_ratio: (.spread_ratio // 0.25),
        rotation_index: (.rotation_index // 0),
        spread_index: (.spread_index // 0),
        active_output: (.active_output // null),
        normalized: (
            if $managed_application == null then (.normalized // null) else {
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

cleanup() {
    if [[ -n "$SYNC_PID" ]]; then
        kill "$SYNC_PID" 2>/dev/null || true
        wait "$SYNC_PID" 2>/dev/null || true
    fi
    if [[ -n "$PREVIEW_PID" ]]; then
        kill "$PREVIEW_PID" 2>/dev/null || true
        wait "$PREVIEW_PID" 2>/dev/null || true
    fi
}

trap cleanup EXIT INT TERM

if [[ -x "$SCRIPT_DIR/sync-hueman-pair-state.sh" ]]; then
    "$SCRIPT_DIR/sync-hueman-pair-state.sh" &
    SYNC_PID="$!"
fi

if [[ -x "$SCRIPT_DIR/capture-hueman-pair-preview.sh" ]]; then
    "$SCRIPT_DIR/capture-hueman-pair-preview.sh" &
    PREVIEW_PID="$!"
fi

"$GODOT_BIN" --path "$SCRIPT_DIR/hueman_godot" "$@"
