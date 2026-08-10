#!/usr/bin/env bash
set -euo pipefail

package_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
registry_path="${package_dir}/etc/hollow-grove/keyboard-grove.json"
portable_config="${package_dir}/etc/skel/.config/niri/config.kdl"

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
    .schema_version == 1
    and .id == "hollow-grove-keyboard-grove-v1"
    and .authority == "HOLLOW_GROVE_KEYBOARD_CONSTITUTION_V1.md"
    and .foundation.house == "Stonebend"
    and ([.form_categories[].id] == ["landmark", "plant", "branch", "fruit", "tool"])
    and ([.modifier_lanes[].lane] == ["Primary", "Adjacent", "Focused", "Alternate"])
    and all(.house_anchors[];
        .form == "landmark"
        and ((.members | length) >= 1)
        and all(.members[]; (.key | length) >= 1 and (.map_label | length) >= 1))
    and all(.operational_tools[];
        .form == "tool"
        and ((.members | length) >= 1)
        and all(.members[]; (.key | length) >= 1 and (.map_label | length) >= 1))
    and (([.domain_anchors[].key] | unique | length) == (.domain_anchors | length))
    and (([.domain_anchors[].domain] | unique | length) == (.domain_anchors | length))
    and (([.domain_anchors[].bindings[].chord] | unique | length)
        == ([.domain_anchors[].bindings[].chord] | length))
    and all(.domain_anchors[];
        . as $domain
        | (.form == "plant")
        and (.key | test("^[A-Z]$"))
        and (.domain | test("^[A-Z][A-Za-z]*$"))
        and ((.bindings | length) >= 2)
        and any(.bindings[];
            .lane == "Primary"
            and .chord == ("Mod+" + ($domain.key | ascii_downcase)))
        and all(.affinities[];
            . as $house
            | (["Stonebend", "Sandmanor", "Glaüshouse", "Flynt"] | index($house)) != null)
        and all(.bindings[];
            . as $binding
            | ($binding.overlay_title == ($domain.domain + " · " + $binding.lane + ": " + $binding.action))
            and ($binding.chord | startswith("Mod+"))
            and ($binding.chord | endswith("+" + ($domain.key | ascii_downcase)))
            and ($binding.display_chord | startswith("Super+"))
            and ($binding.display_chord | endswith("+" + $domain.key))
            and ($binding.lane as $lane
                | (["Primary", "Adjacent", "Focused", "Alternate", "Focused Alternate"]
                    | index($lane)) != null)))
' "${registry_path}" >/dev/null

"${package_dir}/generate-keyboard-grove-map.py" --check

for config_path in "${config_paths[@]}"; do
    if [[ ! -f "${config_path}" ]]; then
        printf 'keyboard grove config not found: %s\n' "${config_path}" >&2
        exit 2
    fi

    niri validate -c "${config_path}" >/dev/null

    while IFS=$'\t' read -r chord overlay_title; do
        binding_needle="${chord} hotkey-overlay-title=\"${overlay_title}\" {"
        binding_count="$(grep -F -c -- "${binding_needle}" "${config_path}" || true)"
        if [[ "${binding_count}" != "1" ]]; then
            printf '%s: expected exactly one canonical binding: %s\n' \
                "${config_path}" "${binding_needle}" >&2
            exit 1
        fi
    done < <(jq -r '.domain_anchors[].bindings[] | [.chord, .overlay_title] | @tsv' "${registry_path}")

    while IFS= read -r domain_key; do
        lower_key="${domain_key,,}"
        expected_chords="$(
            jq -r --arg key "${domain_key}" \
                '.domain_anchors[] | select(.key == $key) | .bindings[].chord' \
                "${registry_path}" | sort
        )"
        actual_chords="$(
            awk -v base="Mod+${lower_key}" -v suffix="+${lower_key}" '
                $1 == base || (length($1) > length(suffix) && substr($1, length($1) - length(suffix) + 1) == suffix) {
                    print $1
                }
            ' "${config_path}" | sort
        )"
        if [[ "${actual_chords}" != "${expected_chords}" ]]; then
            printf '%s: %s domain chords differ from the registry\n' \
                "${config_path}" "${domain_key}" >&2
            printf 'expected:\n%s\nactual:\n%s\n' "${expected_chords}" "${actual_chords}" >&2
            exit 1
        fi
    done < <(jq -r '.domain_anchors[].key' "${registry_path}")

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
