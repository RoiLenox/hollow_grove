#!/usr/bin/env bash

set -euo pipefail

direction="${1:-}"
target_height="${2:-50%}"

usage() {
    printf 'usage: %s {left|right|top|bottom} [height] | {up|down}\n' "${0##*/}" >&2
    exit 2
}

case "$direction" in
    left|right|top|bottom)
        (( $# <= 2 )) || usage
        [[ "$target_height" =~ ^([0-9]+([.][0-9]+)?%|[0-9]+)$ ]] || usage
        ;;
    up|down)
        (( $# == 1 )) || usage
        ;;
    *) usage ;;
esac

for required_command in jq niri; do
    if ! command -v "$required_command" >/dev/null 2>&1; then
        printf '%s is required for %s\n' "$required_command" "${0##*/}" >&2
        exit 1
    fi
done

if ! windows_json="$(niri msg -j windows 2>/dev/null)"; then
    printf 'niri is not reachable\n' >&2
    exit 1
fi

if ! jq -e 'type == "array"' >/dev/null 2>&1 <<<"$windows_json"; then
    printf 'niri returned an invalid windows response\n' >&2
    exit 1
fi

focused_id="$(
    jq -r '.[] | select(.is_focused) | .id' <<<"$windows_json"
)"

if [[ -z "$focused_id" || "$focused_id" == "null" ]]; then
    exit 0
fi

focused_window_json="$(
    jq -c --argjson id "$focused_id" '.[] | select(.id == $id)' <<<"$windows_json"
)"

if [[ -z "$focused_window_json" || "$focused_window_json" == "null" ]]; then
    exit 0
fi

if ! jq -e '
    ((.is_floating // false) | not)
    and (.workspace_id != null)
    and ((.layout.pos_in_scrolling_layout | type) == "array")
    and ((.layout.window_size | type) == "array")
' >/dev/null <<<"$focused_window_json"; then
    exit 0
fi

read -r workspace_id source_col <<EOF
$(jq -r '"\(.workspace_id) \(.layout.pos_in_scrolling_layout[0])"' <<<"$focused_window_json")
EOF

if [[ -z "${workspace_id:-}" || -z "${source_col:-}" ]]; then
    exit 0
fi

index_in_array() {
    local needle="$1"
    shift
    local i=0
    for item in "$@"; do
        if [[ "$item" == "$needle" ]]; then
            printf '%s\n' "$i"
            return 0
        fi
        i=$((i + 1))
    done
    return 1
}

move_window_into_adjacent_column() {
    local horizontal_direction="$1"
    mapfile -t cols < <(
        jq -r --argjson ws "$workspace_id" '
            [.[] | select(
                .workspace_id == $ws
                and ((.is_floating // false) | not)
                and ((.layout.pos_in_scrolling_layout | type) == "array")
            ) | .layout.pos_in_scrolling_layout[0]]
            | unique | sort | .[]
        ' <<<"$windows_json"
    )

    local count="${#cols[@]}"
    if (( count <= 1 )); then
        niri msg action set-window-height --id "$focused_id" "$target_height"
        return
    fi

    local current_idx
    current_idx="$(index_in_array "$source_col" "${cols[@]}")"

    local target_idx target_col target_count
    if [[ "$horizontal_direction" == "left" ]]; then
        target_idx=$(( (current_idx - 1 + count) % count ))
    else
        target_idx=$(( (current_idx + 1) % count ))
    fi
    target_col="${cols[$target_idx]}"

    local source_count source_index workspace_window_count
    source_count="$(
        jq -r --argjson ws "$workspace_id" --argjson col "$source_col" '
            [.[] | select(
                .workspace_id == $ws
                and ((.is_floating // false) | not)
                and ((.layout.pos_in_scrolling_layout | type) == "array")
                and .layout.pos_in_scrolling_layout[0] == $col
            )]
            | length
        ' <<<"$windows_json"
    )"
    source_index="$(
        jq -r --argjson ws "$workspace_id" --argjson col "$source_col" --argjson id "$focused_id" '
            [.[] | select(
                .workspace_id == $ws
                and ((.is_floating // false) | not)
                and ((.layout.pos_in_scrolling_layout | type) == "array")
                and .layout.pos_in_scrolling_layout[0] == $col
            )
             | {id, row: .layout.pos_in_scrolling_layout[1]}]
            | sort_by(.row)
            | map(.id)
            | index($id)
        ' <<<"$windows_json"
    )"
    target_count="$(
        jq -r --argjson ws "$workspace_id" --argjson col "$target_col" '
            [.[] | select(
                .workspace_id == $ws
                and ((.is_floating // false) | not)
                and ((.layout.pos_in_scrolling_layout | type) == "array")
                and .layout.pos_in_scrolling_layout[0] == $col
            )]
            | length
        ' <<<"$windows_json"
    )"
    workspace_window_count="$(
        jq -r --argjson ws "$workspace_id" '
            [.[] | select(
                .workspace_id == $ws
                and ((.is_floating // false) | not)
                and ((.layout.pos_in_scrolling_layout | type) == "array")
            )]
            | length
        ' <<<"$windows_json"
    )"

    # Two solo columns swap cleanly; larger layouts launch the focused window
    # into the adjacent column, wrapping across the horizontal edges.
    if (( workspace_window_count == 2 && source_count == 1 && target_count == 1 )); then
        if [[ "$horizontal_direction" == "left" ]]; then
            if (( current_idx == 0 )); then
                niri msg action move-column-to-last
            else
                niri msg action move-column-left
            fi
        else
            if (( current_idx == count - 1 )); then
                niri msg action move-column-to-first
            else
                niri msg action move-column-right
            fi
        fi
        niri msg action set-window-height --id "$focused_id" "$target_height"
        return
    fi

    local at_edge=false
    if [[ "$horizontal_direction" == "left" ]] && (( current_idx == 0 )); then
        at_edge=true
    elif [[ "$horizontal_direction" == "right" ]] && (( current_idx == count - 1 )); then
        at_edge=true
    fi

    if [[ "$at_edge" == true ]]; then
        if (( source_count > 1 )); then
            niri msg action "consume-or-expel-window-$horizontal_direction"
        fi

        if [[ "$horizontal_direction" == "left" ]]; then
            niri msg action move-column-to-last
        else
            niri msg action move-column-to-first
        fi

        niri msg action "consume-or-expel-window-$horizontal_direction"
    elif (( source_count > 1 )); then
        niri msg action "consume-or-expel-window-$horizontal_direction"
        niri msg action "consume-or-expel-window-$horizontal_direction"
    else
        niri msg action "consume-or-expel-window-$horizontal_direction"
    fi

    # Map the source row onto the destination stack. This keeps top windows
    # moving straight across, bottom windows on the bottom, and intermediate
    # rows proportionally aligned even when the two columns have different
    # stack depths.
    if (( target_count > 0 )); then
        local desired_index moves_up
        desired_index="$(
            jq -nr \
                --argjson src_index "$source_index" \
                --argjson src_count "$source_count" \
                --argjson dst_count "$target_count" '
                    if $src_count <= 1 then
                        0
                    else
                        (($src_index * $dst_count) / ($src_count - 1) | round)
                    end
                '
        )"
        moves_up=$((target_count - desired_index))
        for ((i = 0; i < moves_up; i++)); do
            niri msg action move-window-up
        done
    fi

    niri msg action set-window-height --id "$focused_id" "$target_height"
}

snap_window_to_stack_edge() {
    local stack_edge="$1"
    mapfile -t window_ids < <(
        jq -r --argjson ws "$workspace_id" --argjson col "$source_col" '
            [.[] | select(
                .workspace_id == $ws
                and ((.is_floating // false) | not)
                and ((.layout.pos_in_scrolling_layout | type) == "array")
                and .layout.pos_in_scrolling_layout[0] == $col
            )
             | {id, row: .layout.pos_in_scrolling_layout[1]}]
            | sort_by(.row) | .[].id
        ' <<<"$windows_json"
    )

    local count="${#window_ids[@]}"
    (( count > 0 )) || exit 0

    local current_idx
    current_idx="$(index_in_array "$focused_id" "${window_ids[@]}")"

    if [[ "$stack_edge" == "top" ]]; then
        for ((i = 0; i < current_idx; i++)); do
            niri msg action move-window-up
        done
    else
        local moves_down=$((count - current_idx - 1))
        for ((i = 0; i < moves_down; i++)); do
            niri msg action move-window-down
        done
    fi

    niri msg action set-window-height --id "$focused_id" "$target_height"
}

wrap_move_window_vertical() {
    mapfile -t window_ids < <(
        jq -r --argjson ws "$workspace_id" --argjson col "$source_col" '
            [.[] | select(
                .workspace_id == $ws
                and ((.is_floating // false) | not)
                and ((.layout.pos_in_scrolling_layout | type) == "array")
                and .layout.pos_in_scrolling_layout[0] == $col
            )
             | {id, row: .layout.pos_in_scrolling_layout[1]}]
            | sort_by(.row) | .[].id
        ' <<<"$windows_json"
    )

    local count="${#window_ids[@]}"
    (( count > 0 )) || exit 0

    local current_idx
    current_idx="$(index_in_array "$focused_id" "${window_ids[@]}")"

    case "$direction" in
        down)
            if (( current_idx == count - 1 )); then
                for ((i = 1; i < count; i++)); do
                    niri msg action move-window-up
                done
            else
                niri msg action move-window-down
            fi
            ;;
        up)
            if (( current_idx == 0 )); then
                for ((i = 1; i < count; i++)); do
                    niri msg action move-window-down
                done
            else
                niri msg action move-window-up
            fi
            ;;
    esac
}

case "$direction" in
    left|right) move_window_into_adjacent_column "$direction" ;;
    top|bottom) snap_window_to_stack_edge "$direction" ;;
    up|down) wrap_move_window_vertical ;;
esac
