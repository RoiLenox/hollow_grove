#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
GODOT_BIN="${GODOT_BIN:-}"
PAIR_STATE_PATH="$SCRIPT_DIR/artifacts/hueman_pair_state.json"
APPLICATION_REGISTRY_PATH="$SCRIPT_DIR/artifacts/hollow_grove_application_registry.json"
PREVIEW_IMAGE_PATH="$SCRIPT_DIR/artifacts/hueman_pair_preview.png"
BROKER_PID=""
TOPOLOGY_PID=""

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
    if [[ -n "$TOPOLOGY_PID" ]]; then
        kill "$TOPOLOGY_PID" 2>/dev/null || true
        wait "$TOPOLOGY_PID" 2>/dev/null || true
    fi
    if [[ -n "$BROKER_PID" ]]; then
        kill "$BROKER_PID" 2>/dev/null || true
        wait "$BROKER_PID" 2>/dev/null || true
    fi
}

trap cleanup EXIT INT TERM

BROKER_BIN="${HOLLOW_GROVE_NIRI_BIN:-${HOLLOW_GROVE_BIN_DIR:-$HOME/.local/lib/hollow-grove/bin}/hollow-niri}"
if [[ ! -x "$BROKER_BIN" && -x "$SCRIPT_DIR/target/release/hollow-niri" ]]; then
    BROKER_BIN="$SCRIPT_DIR/target/release/hollow-niri"
fi
if [[ ! -x "$BROKER_BIN" ]]; then
    printf 'optimized Hollow Niri broker not found; run ./install-local-release.sh\n' >&2
    exit 1
fi

TOPOLOGY_BIN="${HOLLOW_GROVE_TOPOLOGY_BIN:-${HOLLOW_GROVE_BIN_DIR:-$HOME/.local/lib/hollow-grove/bin}/adaptive_topology_live}"
if [[ ! -x "$TOPOLOGY_BIN" && -x "$SCRIPT_DIR/target/release/adaptive_topology_live" ]]; then
    TOPOLOGY_BIN="$SCRIPT_DIR/target/release/adaptive_topology_live"
fi
if [[ ! -x "$TOPOLOGY_BIN" ]]; then
    printf 'optimized adaptive topology live bridge not found; run ./install-local-release.sh\n' >&2
    exit 1
fi

"$BROKER_BIN" broker --root "$SCRIPT_DIR" \
    --preview-fps "${HUEMAN_PAIR_PREVIEW_FPS:-1}" &
BROKER_PID="$!"

"$TOPOLOGY_BIN" serve --root "$SCRIPT_DIR" \
    --poll-ms "${HOLLOW_GROVE_TOPOLOGY_POLL_MS:-50}" --quiet &
TOPOLOGY_PID="$!"

"$GODOT_BIN" --path "$SCRIPT_DIR/hueman_godot" \
    res://scenes/hueman_screen_shell.tscn "$@"
