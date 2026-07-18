#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
PAIR_STATE_PATH="$SCRIPT_DIR/artifacts/hueman_pair_state.json"
PREVIEW_IMAGE_PATH="$SCRIPT_DIR/artifacts/hueman_pair_preview.png"
PREVIEW_STATE_PATH="$SCRIPT_DIR/artifacts/hueman_pair_preview_state.json"
FRAME_DELAY="${HUEMAN_PAIR_PREVIEW_INTERVAL:-0.12}"

mkdir -p "$SCRIPT_DIR/artifacts"

write_state() {
    local status="$1"
    local detail="$2"
    cat >"$PREVIEW_STATE_PATH" <<EOF
{
  "schema_version": "0.2.0",
  "status": "$status",
  "detail": $(jq -Rn --arg value "$detail" '$value')
}
EOF
}

if ! command -v niri >/dev/null 2>&1; then
    write_state "unavailable" "niri executable not found"
    exit 0
fi

if ! command -v grim >/dev/null 2>&1; then
    write_state "unavailable" "grim executable not found"
    exit 0
fi

while true; do
    if [[ ! -f "$PAIR_STATE_PATH" ]]; then
        write_state "waiting" "pair state not found"
        sleep "$FRAME_DELAY"
        continue
    fi

    capture_allowed="$(jq -r '
        if ((.focused_window.app_id // "") | startswith("hollow-grove.")) then
            (.application_attachment.privacy.capture_allowed // false)
        else
            true
        end
    ' "$PAIR_STATE_PATH")"
    if [[ "$capture_allowed" != "true" ]]; then
        rm -f "$PREVIEW_IMAGE_PATH"
        write_state "masked" "managed clinical surface uses semantic-only projection"
        sleep "$FRAME_DELAY"
        continue
    fi

    pair_window_id="$(jq -r '.focused_window.id // empty' "$PAIR_STATE_PATH")"
    if [[ -z "$pair_window_id" ]]; then
        write_state "waiting" "paired window id not available"
        sleep "$FRAME_DELAY"
        continue
    fi

    if ! windows_json="$(niri msg -j windows 2>/dev/null)"; then
        write_state "unavailable" "failed to read niri windows"
        sleep "$FRAME_DELAY"
        continue
    fi

    geometry_json="$(printf '%s\n' "$windows_json" | jq --argjson window_id "$pair_window_id" '
        def rect:
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
            );
        first(.[] | select(.id == $window_id) | rect) // empty
    ')"

    if [[ -z "$geometry_json" ]]; then
        write_state "waiting" "paired window not found in niri window list"
        sleep "$FRAME_DELAY"
        continue
    fi

    x="$(printf '%s\n' "$geometry_json" | jq -r '.x | floor')"
    y="$(printf '%s\n' "$geometry_json" | jq -r '.y | floor')"
    width="$(printf '%s\n' "$geometry_json" | jq -r '.width | floor')"
    height="$(printf '%s\n' "$geometry_json" | jq -r '.height | floor')"

    if [[ "$width" -le 1 || "$height" -le 1 ]]; then
        write_state "waiting" "paired window geometry too small"
        sleep "$FRAME_DELAY"
        continue
    fi

    tmp_png="$(mktemp --suffix=.png)"
    if grim -g "${x},${y} ${width}x${height}" "$tmp_png" >/dev/null 2>&1; then
        mv "$tmp_png" "$PREVIEW_IMAGE_PATH"
        write_state "ok" "captured ${width}x${height} at ${x},${y}"
    else
        rm -f "$tmp_png"
        write_state "unavailable" "grim capture failed"
    fi

    sleep "$FRAME_DELAY"
done
