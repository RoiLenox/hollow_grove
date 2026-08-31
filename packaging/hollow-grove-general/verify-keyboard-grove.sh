#!/usr/bin/env bash
set -euo pipefail

package_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
registry_path="${package_dir}/etc/hollow-grove/keyboard-grove.json"
inventory_path="${package_dir}/etc/hollow-grove/applications.toml"
portable_config="${package_dir}/etc/skel/.config/niri/config.kdl"
keyboard_command="${package_dir}/usr/bin/hollow-keyboard"
desktop_entry="${package_dir}/usr/share/applications/hollow-grove-keyboard.desktop"
native_manifest="${package_dir}/../../hollow-niri/Cargo.toml"
native_service="${package_dir}/usr/lib/systemd/user/hollow-niri.service"
compatibility_helper="${package_dir}/usr/lib/hollow-grove/bin/niri-wrap-move"
shell_command="${package_dir}/usr/bin/hollow-shell"
shell_helper="${package_dir}/usr/lib/hollow-grove/bin/start-shell"
shell_config="${package_dir}/etc/xdg/hollow-grove/waybar/config.jsonc"
shell_style="${package_dir}/etc/xdg/hollow-grove/waybar/style.css"

for required_command in jq niri awk grep sort python3; do
    if ! command -v "${required_command}" >/dev/null 2>&1; then
        printf 'keyboard grove verification requires %s\n' "${required_command}" >&2
        exit 127
    fi
done

if (( $# == 0 )); then
    config_paths=("${portable_config}")
else
    config_paths=("$@")
fi

jq -e '
    .schema_version == 2
    and .id == "hollow-grove-keyboard-grove-v1"
    and .authority == "HOLLOW_GROVE_KEYBOARD_CONSTITUTION_V1.md"
    and .governing_rule == "Key = WHAT · Modifier = HOW"
    and .foundation.house == "Stonebend"
    and ([.form_categories[].id] == ["landmark", "plant", "branch", "fruit", "tool"])
    and ([.modifier_lanes[] | [.display_modifier, .meaning, .think]] == [
        ["Super", "Primary", "Normal way"],
        ["Super+Shift", "Adjacent", "Different form"],
        ["Super+Ctrl", "Focused", "Specialized/control"],
        ["Super+Alt", "Alternate", "Other way"]
    ])
    and ([.letter_vocabulary[] | [.key, .meaning, .memory_hook]] == [
        ["A", "Assets", "MAKE"],
        ["B", "Browse", "Web"],
        ["C", "Chat", "Communicate"],
        ["D", "Develop", "Code/build"],
        ["E", "Edit", "Modify"],
        ["F", "Files", "Filesystem"],
        ["G", "Games", "PLAY"],
        ["H", "← Movement", "Flynt"],
        ["I", "Images", "View images"],
        ["J", "↓ Movement", "Glaüshouse"],
        ["K", "↑ Movement", "Stonebend"],
        ["L", "→ Movement", "Sandmanor"],
        ["M", "Music", "HEAR"],
        ["N", "News", "Feeds"],
        ["O", "Office", "Productivity"],
        ["P", "Photos", "Photography"],
        ["Q", "Query", "Ask AI/search"],
        ["R", "Record", "Capture"],
        ["S", "Settings", "Configure"],
        ["T", "Tasks", "To-do/work"],
        ["U", "Utilities", "System tools"],
        ["V", "Video", "WATCH"],
        ["W", "Write", "Compose"],
        ["X", "eXchange", "Finance/markets"],
        ["Y", "sYnc", "Transfer/sync"],
        ["Z", "Zip", "Archives"]
    ])
    and ([.letter_vocabulary[] | select(.form == "tool") | .key] == ["H", "J", "K", "L"])
    and ([.letter_vocabulary[] | select(.form == "plant") | .key] | length) == 22
    and all(.house_anchors[];
        .form == "landmark"
        and ((.members | length) >= 1)
        and all(.members[]; (.key | length) >= 1 and (.map_label | length) >= 1))
    and all(.operational_tools[];
        .form == "tool"
        and ((.members | length) >= 1)
        and all(.members[]; (.key | length) >= 1 and (.map_label | length) >= 1))
    and .overlay_contract == "Meaning · Modifier meaning: Action"
    and .profile_policy.status == "semantically-reconciled"
' "${registry_path}" >/dev/null

python3 - "${registry_path}" "${inventory_path}" "${portable_config}" <<'PY'
import json
import re
import sys
import tomllib
from pathlib import Path

registry = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
with Path(sys.argv[2]).open("rb") as source:
    inventory = tomllib.load(source)

expected = [
    (entry["key"], entry["meaning"], entry["memory_hook"])
    for entry in registry["letter_vocabulary"]
    if entry["form"] == "plant"
]
actual = [
    (entry["key"], entry["meaning"], entry["memory_hook"])
    for entry in inventory["family"]
]
if actual != expected:
    raise SystemExit("application inventory does not match the constitutional domain vocabulary")

config_text = Path(sys.argv[3]).read_text(encoding="utf-8")
active: dict[str, list[str]] = {key: [] for key, _, _ in expected}
for line in config_text.splitlines():
    match = re.match(r"\s*(Mod(?:\+[A-Za-z]+)+)\s", line)
    if not match:
        continue
    chord = match.group(1)
    key = chord.rsplit("+", 1)[-1].upper()
    if key in active:
        active[key].append(chord.rsplit("+", 1)[0] + "+" + key)

for family in inventory["family"]:
    if sorted(family["bindings"]) != sorted(active[family["key"]]):
        raise SystemExit(
            f'application inventory bindings differ for {family["key"]}: '
            f'{family["bindings"]!r} != {active[family["key"]]!r}'
        )
PY

"${package_dir}/generate-keyboard-grove-map.py" --check

if [[ ! -x "${keyboard_command}" ]]; then
    printf 'keyboard discovery command is missing or not executable: %s\n' \
        "${keyboard_command}" >&2
    exit 1
fi

if [[ ! -f "${desktop_entry}" ]] \
    || ! grep -F -q 'Exec=hollow-keyboard show' "${desktop_entry}"; then
    printf 'keyboard desktop entry is missing or invalid: %s\n' \
        "${desktop_entry}" >&2
    exit 1
fi

if [[ ! -f "${native_manifest}" ]] \
    || ! grep -F -q 'name = "hollow-niri"' "${native_manifest}"; then
    printf 'native Niri broker manifest is missing or invalid: %s\n' \
        "${native_manifest}" >&2
    exit 1
fi

if [[ ! -f "${native_service}" ]] \
    || ! grep -F -q 'ExecStart=/usr/lib/hollow-grove/bin/hollow-niri daemon' "${native_service}"; then
    printf 'native Niri broker service is missing or invalid: %s\n' \
        "${native_service}" >&2
    exit 1
fi

if [[ ! -x "${compatibility_helper}" ]] \
    || ! grep -F -q 'hollow-niri move' "${compatibility_helper}"; then
    printf 'native movement compatibility entrance is missing or invalid: %s\n' \
        "${compatibility_helper}" >&2
    exit 1
fi

if [[ ! -x "${shell_command}" ]] \
    || ! grep -F -q 'HOLLOW GROVE DESKTOP DOCTOR' "${shell_command}"; then
    printf 'public shell command is missing or invalid: %s\n' "${shell_command}" >&2
    exit 1
fi

if [[ ! -x "${shell_helper}" ]] \
    || ! grep -F -q 'exec waybar --config' "${shell_helper}"; then
    printf 'shell startup helper is missing or invalid: %s\n' "${shell_helper}" >&2
    exit 1
fi

jq -e '
    length == 1
    and .[0].name == "hollow-grove-top"
    and .[0].position == "top"
    and .[0]["modules-left"] == ["custom/grove", "niri/workspaces"]
    and .[0]["modules-center"] == ["niri/window"]
    and .[0]["custom/grove"]["on-click"] == "/usr/bin/hollow-shell launcher"
' "${shell_config}" >/dev/null

for color in '#0A1024' '#121B38' '#1C2A52' '#A7B6D9' '#7F8BC4' '#78C9A4' '#E07A86'; do
    if ! grep -F -q "${color}" "${shell_style}"; then
        printf 'shell style omits constitutional color %s\n' "${color}" >&2
        exit 1
    fi
done

keyboard_environment=(
    "HOLLOW_GROVE_KEYBOARD_REGISTRY=${registry_path}"
    "HOLLOW_GROVE_KEYBOARD_MAP=${package_dir}/usr/share/hollow-grove/general-edition/hollow-grove-keyboard-map.svg"
    "HOLLOW_GROVE_NIRI_CONFIG=${portable_config}"
)

env "${keyboard_environment[@]}" "${keyboard_command}" list --json \
    | jq -e '
        .governing_rule == "Key = WHAT · Modifier = HOW"
        and (.modifier_grammar | length) == 4
        and (.letter_vocabulary | length) == 26
    ' >/dev/null

keyboard_search_output="$(
    env "${keyboard_environment[@]}" "${keyboard_command}" search finance
)"
grep -F -q 'X · eXchange' <<<"${keyboard_search_output}"

env "${keyboard_environment[@]}" "${keyboard_command}" map --json \
    | jq -e '.id == "hollow-grove-keyboard-grove-v1"' >/dev/null

env "${keyboard_environment[@]}" "${keyboard_command}" doctor --json \
    | jq -e '
        .status == "pass"
        and .registry.valid
        and .map.available
        and .niri.semantic_status == "pass"
    ' >/dev/null

if command -v desktop-file-validate >/dev/null 2>&1; then
    desktop-file-validate "${desktop_entry}"
fi

for config_path in "${config_paths[@]}"; do
    if [[ ! -f "${config_path}" ]]; then
        printf 'keyboard grove config not found: %s\n' "${config_path}" >&2
        exit 2
    fi

    niri validate -c "${config_path}" >/dev/null

    if ! grep -F -q 'animations { off; }' "${config_path}"; then
        printf '%s: immediate animation profile is missing\n' "${config_path}" >&2
        exit 1
    fi
    if ! grep -F -q 'hollow-niri" "move"' "${config_path}"; then
        printf '%s: native Niri movement bindings are missing\n' "${config_path}" >&2
        exit 1
    fi
    if ! grep -F -q 'spawn-at-startup "/usr/lib/hollow-grove/bin/start-shell"' "${config_path}"; then
        printf '%s: Hollow Grove shell startup is missing\n' "${config_path}" >&2
        exit 1
    fi
    if grep -F -q 'niri-wrap-move"' "${config_path}"; then
        printf '%s: legacy shell movement binding remains\n' "${config_path}" >&2
        exit 1
    fi
    if grep -E -q 'spawn-at-startup.*xwayland-satellite' "${config_path}"; then
        printf '%s: duplicate manual XWayland startup remains\n' "${config_path}" >&2
        exit 1
    fi
    if grep -E -q '^[[:space:]]*proportion[[:space:]]+0([.]0+)?[[:space:]]*;?[[:space:]]*$' "${config_path}"; then
        printf '%s: zero-size layout proportion can collapse a tiled window\n' \
            "${config_path}" >&2
        exit 1
    fi

    while IFS=$'\t' read -r key meaning; do
        lower_key="${key,,}"
        comment_needle="// ${key} — ${meaning}"
        if ! grep -F -q -- "${comment_needle}" "${config_path}"; then
            printf '%s: missing canonical family heading: %s\n' \
                "${config_path}" "${comment_needle}" >&2
            exit 1
        fi

        if [[ "${key}" =~ ^[HJKL]$ ]]; then
            continue
        fi

        while IFS= read -r binding_line; do
            [[ -n "${binding_line}" ]] || continue
            chord="$(awk '{ print $1 }' <<<"${binding_line}")"
            case "${chord}" in
                "Mod+${lower_key}") expected_lane="Primary" ;;
                "Mod+Shift+${lower_key}") expected_lane="Adjacent" ;;
                "Mod+Ctrl+${lower_key}") expected_lane="Focused" ;;
                "Mod+Alt+${lower_key}") expected_lane="Alternate" ;;
                *) continue ;;
            esac
            title_needle="hotkey-overlay-title=\"${meaning} · ${expected_lane}:"
            if [[ "${binding_line}" != *"${title_needle}"* ]]; then
                printf '%s: %s does not preserve WHAT/HOW title %s\n' \
                    "${config_path}" "${chord}" "${title_needle}" >&2
                exit 1
            fi
        done < <(
            awk -v base="Mod+${lower_key}" -v suffix="+${lower_key}" '
                $1 == base || (length($1) > length(suffix) && substr($1, length($1) - length(suffix) + 1) == suffix) {
                    print
                }
            ' "${config_path}"
        )
    done < <(jq -r '.letter_vocabulary[] | [.key, .meaning] | @tsv' "${registry_path}")

    duplicate_chords="$(
        awk '
            $1 ~ /^Mod\+/ { count[$1] += 1 }
            END { for (chord in count) if (count[chord] > 1) print chord }
        ' "${config_path}" | sort
    )"
    if [[ -n "${duplicate_chords}" ]]; then
        printf '%s: duplicate Mod chords:\n%s\n' "${config_path}" "${duplicate_chords}" >&2
        exit 1
    fi

    printf 'keyboard grove valid: %s\n' "${config_path}"
done
