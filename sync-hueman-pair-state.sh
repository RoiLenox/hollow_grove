#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
PAIR_STATE_PATH="$SCRIPT_DIR/artifacts/hueman_pair_state.json"
FRAME_DELAY="${HUEMAN_PAIR_SYNC_INTERVAL:-0.12}"

mkdir -p "$SCRIPT_DIR/artifacts"

if ! command -v jq >/dev/null 2>&1; then
    exit 0
fi

if ! command -v niri >/dev/null 2>&1; then
    exit 0
fi

while true; do
    if [[ ! -f "$PAIR_STATE_PATH" ]]; then
        sleep "$FRAME_DELAY"
        continue
    fi

    pair_window_id="$(jq -r '.focused_window.id // empty' "$PAIR_STATE_PATH")"
    if [[ -z "$pair_window_id" ]]; then
        sleep "$FRAME_DELAY"
        continue
    fi

    if ! windows_json="$(niri msg -j windows 2>/dev/null)"; then
        sleep "$FRAME_DELAY"
        continue
    fi

    if ! outputs_json="$(niri msg -j outputs 2>/dev/null)"; then
        sleep "$FRAME_DELAY"
        continue
    fi

    matched_window_json="$(printf '%s\n' "$windows_json" | jq --argjson window_id "$pair_window_id" '
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
        first(.[] | select(.id == $window_id) | {
            id: (.id // null),
            title: (.title // null),
            app_id: (.app_id // .["app-id"] // null),
            output: (.output // .output_name // null),
            rect: rect
        }) // empty
    ')"

    if [[ -z "$matched_window_json" ]]; then
        tmpfile="$(mktemp)"
        jq '
            .schema_version = "0.2.0"
            | .paired_window_mode = true
            | .sticky_actor = true
            | .probe_source = (if .application_attachment == null then "paired_window_center" else "detached_application" end)
            | .binding_status = "window-missing"
            | .normalized = null
            | .active_output = null
            | if .application_attachment != null then
                .application_attachment.lifecycle = "orphaned"
              else
                .
              end
        ' "$PAIR_STATE_PATH" >"$tmpfile"
        mv "$tmpfile" "$PAIR_STATE_PATH"
        sleep "$FRAME_DELAY"
        continue
    fi

    pair_output_name="$(printf '%s\n' "$matched_window_json" | jq -r '.output // empty')"
    if [[ -n "$pair_output_name" ]]; then
        matched_output_json="$(printf '%s\n' "$outputs_json" | jq --arg output_name "$pair_output_name" '
            def rect:
                .logical
                // .geometry
                // .rect
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
            first(.[] | select(.name == $output_name) | {
                name: .name,
                is_focused: (.is_focused // false),
                rect: rect
            }) // empty
        ')"
    else
        matched_output_json=""
    fi

    if [[ -z "$matched_output_json" ]]; then
        matched_output_json="$(printf '%s\n' "$outputs_json" | jq '
            def rect:
                .logical
                // .geometry
                // .rect
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
            (first(.[] | select(.is_focused == true)) // first(.[])) | {
                name: .name,
                is_focused: (.is_focused // false),
                rect: rect
            }
        ')"
    fi

    if [[ -n "$matched_output_json" ]]; then
        normalized_json="$(jq -n --argjson window "$matched_window_json" --argjson output "$matched_output_json" '
            {
                center: {
                    x: ((($window.rect.x - $output.rect.x) + ($window.rect.width / 2.0)) / $output.rect.width),
                    y: ((($window.rect.y - $output.rect.y) + ($window.rect.height / 2.0)) / $output.rect.height)
                },
                rect: {
                    x: (($window.rect.x - $output.rect.x) / $output.rect.width),
                    y: (($window.rect.y - $output.rect.y) / $output.rect.height),
                    width: ($window.rect.width / $output.rect.width),
                    height: ($window.rect.height / $output.rect.height)
                }
            }
        ')"
        binding_status="ok"
    else
        normalized_json="null"
        binding_status="no-matching-output"
    fi

    application_anchor_json="$(jq -c '.application_attachment.world_anchor.normalized // empty' "$PAIR_STATE_PATH")"
    if [[ -n "$application_anchor_json" && "$normalized_json" != "null" ]]; then
        normalized_json="$(jq -n --argjson actual "$normalized_json" --argjson anchor "$application_anchor_json" '
            {
                center: $anchor,
                rect: $actual.rect
            }
        ')"
        probe_source="application_world_anchor"
        if [[ "$binding_status" == "ok" ]]; then
            binding_status="attached"
        fi
    else
        probe_source="paired_window_center"
    fi

    tmpfile="$(mktemp)"
    jq \
        --argjson matched_window "$matched_window_json" \
        --argjson matched_output "${matched_output_json:-null}" \
        --argjson normalized "$normalized_json" \
        --arg binding_status "$binding_status" \
        --arg probe_source "$probe_source" '
        .schema_version = "0.2.0"
        | .paired_window_mode = true
        | .sticky_actor = true
        | .probe_source = $probe_source
        | .binding_status = $binding_status
        | .focused_window = $matched_window
        | .active_output = $matched_output
        | .normalized = $normalized
        | if (.application_attachment != null and $binding_status == "attached") then
            .application_attachment.lifecycle = "attached"
          else
            .
          end
    ' "$PAIR_STATE_PATH" >"$tmpfile"
    mv "$tmpfile" "$PAIR_STATE_PATH"

    sleep "$FRAME_DELAY"
done
