#!/usr/bin/env python3
"""Generate the deterministic Hollow Grove keyboard ecology map."""

from __future__ import annotations

import argparse
import html
import json
import sys
from pathlib import Path


PACKAGE_DIR = Path(__file__).resolve().parent
REPOSITORY_ROOT = PACKAGE_DIR.parents[1]
REGISTRY_PATH = PACKAGE_DIR / "etc/hollow-grove/keyboard-grove.json"
PALETTE_PATH = (
    REPOSITORY_ROOT / "src/constitutional/hollow_grove_visual_color_palette.json"
)
ARTIFACT_SVG = REPOSITORY_ROOT / "artifacts/hollow_grove_keyboard_grove_map.svg"
ARTIFACT_MD = REPOSITORY_ROOT / "artifacts/hollow_grove_keyboard_grove_map.md"
INSTALLED_SVG = (
    PACKAGE_DIR
    / "usr/share/hollow-grove/general-edition/hollow-grove-keyboard-map.svg"
)

WIDTH = 1600
KEY_UNIT = 82
KEY_GAP = 10
KEY_HEIGHT = 78


def load_json(path: Path) -> dict:
    with path.open(encoding="utf-8") as source:
        return json.load(source)


def color_table(palette: dict) -> dict[str, str]:
    return {entry["id"]: entry["hex"] for entry in palette["colors"]}


def esc(value: object) -> str:
    return html.escape(str(value), quote=True)


def svg_text(
    x: float,
    y: float,
    value: str,
    css_class: str,
    anchor: str = "middle",
) -> str:
    return (
        f'<text x="{x:.1f}" y="{y:.1f}" class="{css_class}" '
        f'text-anchor="{anchor}">{esc(value)}</text>'
    )


def house_color_ids(palette: dict) -> dict[str, dict[str, str]]:
    return {
        house["name"]: {
            "dark": house["dark"],
            "primary": house["primary"],
            "highlight": house["highlight"],
        }
        for house in palette["houses"]
    }


def semantic_indexes(registry: dict) -> tuple[dict, dict, dict]:
    landmarks: dict[str, dict] = {}
    for anchor in registry["house_anchors"]:
        for member in anchor["members"]:
            landmarks[member["key"]] = {
                "house": anchor["house"],
                "label": member["map_label"],
            }

    plants = {plant["key"]: plant for plant in registry["domain_anchors"]}

    tools: dict[str, dict] = {}
    for tool in registry["operational_tools"]:
        for member in tool["members"]:
            tools[member["key"]] = {
                "name": tool["name"],
                "label": member["map_label"],
                "affinity": member.get("affinity", tool.get("affinity")),
            }
    return landmarks, plants, tools


def keyboard_rows() -> list[tuple[float, list[tuple[str, float]]]]:
    return [
        (
            78,
            [("Escape", 1.25), ("Grave", 1.0)]
            + [(str(number), 1.0) for number in range(1, 10)]
            + [("0", 1.0), ("BackSpace", 2.0)],
        ),
        (
            118,
            [(letter, 1.0) for letter in "QWERTYUIOP"]
            + [("BracketLeft", 1.0), ("BracketRight", 1.0)],
        ),
        (
            164,
            [(letter, 1.0) for letter in "ASDFGHJKL"] + [("Return", 2.15)],
        ),
        (218, [(letter, 1.0) for letter in "ZXCVBNM"]),
        (478, [("Space", 6.2)]),
    ]


def display_key(key: str) -> str:
    return {
        "Escape": "ESC",
        "Grave": "`",
        "BackSpace": "BACKSPACE",
        "Return": "ENTER",
        "Space": "SPACE",
        "BracketLeft": "[",
        "BracketRight": "]",
    }.get(key, key)


def key_rendering(
    key: str,
    landmarks: dict,
    plants: dict,
    tools: dict,
    house_ids: dict,
    color_values: dict[str, str],
) -> tuple[str, str, str, str]:
    if key in landmarks:
        landmark = landmarks[key]
        house_color_ids = house_ids[landmark["house"]]
        return (
            color_values[house_color_ids["dark"]],
            color_values[house_color_ids["highlight"]],
            landmark["label"],
            "LANDMARK",
        )

    if key in plants:
        plant = plants[key]
        return (
            f"url(#plant-{esc(key)})",
            f"url(#plant-edge-{esc(key)})",
            plant["domain"],
            "PLANT",
        )

    if key in tools:
        tool = tools[key]
        affinity = tool.get("affinity") or "Stonebend"
        house_color_ids = house_ids[affinity]
        return (
            "var(--raised)",
            color_values[house_color_ids["highlight"]],
            tool["label"],
            "TOOL",
        )

    return "var(--deep)", "var(--raised)", "Open plot", "PROVISIONAL"


def render_key(
    x: float,
    y: float,
    width: float,
    key: str,
    landmarks: dict,
    plants: dict,
    tools: dict,
    house_ids: dict,
    color_values: dict[str, str],
) -> str:
    fill, stroke, semantic_label, category = key_rendering(
        key, landmarks, plants, tools, house_ids, color_values
    )
    center_x = x + width / 2
    display = display_key(key)
    display_class = "key-name key-name-small" if len(display) > 4 else "key-name"
    category_class = "category tool-category" if category == "TOOL" else "category"
    dash = ' stroke-dasharray="7 5"' if category == "TOOL" else ""
    return "\n".join(
        [
            (
                f'<g class="key {category.lower()}">'
                f'<rect x="{x:.1f}" y="{y:.1f}" width="{width:.1f}" '
                f'height="{KEY_HEIGHT}" rx="13" fill="{fill}" stroke="{stroke}"{dash}/>'
            ),
            svg_text(center_x, y + 27, display, display_class),
            svg_text(center_x, y + 49, semantic_label, "semantic-label"),
            svg_text(center_x, y + 68, category, category_class),
            "</g>",
        ]
    )


def gradient_definitions(
    registry: dict, house_ids: dict, colors: dict[str, str]
) -> str:
    definitions: list[str] = []
    for plant in registry["domain_anchors"]:
        affinities = plant["affinities"]
        gradient_colors = [colors[house_ids[house]["dark"]] for house in affinities]
        edge_colors = [colors[house_ids[house]["highlight"]] for house in affinities]
        if len(gradient_colors) == 1:
            gradient_colors *= 2
            edge_colors *= 2
        stops = "".join(
            f'<stop offset="{index * 100 / (len(gradient_colors) - 1):.0f}%" '
            f'stop-color="{value}"/>'
            for index, value in enumerate(gradient_colors)
        )
        edge_stops = "".join(
            f'<stop offset="{index * 100 / (len(edge_colors) - 1):.0f}%" '
            f'stop-color="{value}"/>'
            for index, value in enumerate(edge_colors)
        )
        definitions.append(
            f'<linearGradient id="plant-{esc(plant["key"])}">{stops}</linearGradient>'
        )
        definitions.append(
            f'<linearGradient id="plant-edge-{esc(plant["key"])}">{edge_stops}</linearGradient>'
        )
    return "\n".join(definitions)


def render_plant_card(
    plant: dict,
    x: float,
    y: float,
    width: float,
    house_ids: dict,
    colors: dict[str, str],
) -> str:
    affinity_names = " + ".join(plant["affinities"])
    edge = colors[house_ids[plant["affinities"][0]]["highlight"]]
    parts = [
        f'<g class="plant-card">',
        (
            f'<rect x="{x:.1f}" y="{y:.1f}" width="{width:.1f}" height="218" '
            f'rx="18" fill="var(--deep)" stroke="{edge}"/>'
        ),
        svg_text(x + 24, y + 34, f'{plant["key"]} · {plant["domain"]}', "card-title", "start"),
        svg_text(x + 24, y + 59, f"Affinities: {affinity_names}", "card-subtitle", "start"),
    ]
    branch_width = (width - 60) / 2
    for index, binding in enumerate(plant["bindings"]):
        column = index % 2
        row = index // 2
        branch_x = x + 20 + column * (branch_width + 20)
        branch_y = y + 78 + row * 43
        parts.extend(
            [
                (
                    f'<rect x="{branch_x:.1f}" y="{branch_y:.1f}" '
                    f'width="{branch_width:.1f}" height="34" rx="9" '
                    f'fill="var(--raised)" stroke="var(--shadow)"/>'
                ),
                svg_text(
                    branch_x + 12,
                    branch_y + 22,
                    f'{binding["lane"]}: {binding["action"]}',
                    "branch-label",
                    "start",
                ),
            ]
        )
    parts.append("</g>")
    return "\n".join(parts)


def render_svg(registry: dict, palette: dict) -> str:
    colors = color_table(palette)
    house_ids = house_color_ids(palette)
    landmarks, plants, tools = semantic_indexes(registry)
    universal_outline = colors["universal.oxford_blue"]
    universal_shadow = colors["universal.midnight_blue"]
    universal_raised = colors["universal.space_cadet"]
    general_text = colors["flynt.powder_blue"]
    foundation_text = colors["stonebend.air_superiority_blue"]
    plant_columns = 2
    plant_count = len(registry["domain_anchors"])
    plant_rows = max(1, (plant_count + plant_columns - 1) // plant_columns)
    height = 1180 + (plant_rows - 1) * 238

    parts = [
        '<?xml version="1.0" encoding="UTF-8"?>',
        (
            f'<svg xmlns="http://www.w3.org/2000/svg" width="{WIDTH}" height="{height}" '
            f'viewBox="0 0 {WIDTH} {height}" role="img" '
            f'aria-labelledby="map-title map-description">'
        ),
        '<title id="map-title">Hollow Grove Keyboard Grove Map</title>',
        (
            '<desc id="map-description">A keyboard ecology map showing House landmarks, '
            'Office and Music domain plants, operational tools, provisional plots, and the '
            'modifier branches of each admitted plant.</desc>'
        ),
        (
            '<metadata>Generated from keyboard-grove.json under '
            'HOLLOW_GROVE_KEYBOARD_CONSTITUTION_V1.md. Do not edit by hand.</metadata>'
        ),
        '<defs>',
        gradient_definitions(registry, house_ids, colors),
        "</defs>",
        "<style>",
        (
            f':root{{--outline:{universal_outline};--shadow:{universal_shadow};'
            f'--raised:{universal_raised};--text:{general_text};'
            f'--foundation:{foundation_text};--deep:{universal_shadow};}}'
        ),
        "text{font-family:Inter,ui-sans-serif,system-ui,sans-serif;fill:var(--text)}",
        ".title{font-size:38px;font-weight:750;letter-spacing:.5px}",
        ".subtitle{font-size:17px;fill:var(--foundation)}",
        ".section-title{font-size:20px;font-weight:700;fill:var(--foundation)}",
        ".key rect{stroke-width:2.5}",
        ".key-name{font-size:18px;font-weight:800}",
        ".key-name-small{font-size:13px}",
        ".semantic-label{font-size:11px;font-weight:650}",
        ".category{font-size:8.5px;letter-spacing:1.3px;opacity:.88}",
        ".tool-category{fill:var(--foundation)}",
        ".card-title{font-size:22px;font-weight:750}",
        ".card-subtitle{font-size:13px;fill:var(--foundation)}",
        ".branch-label{font-size:12px;font-weight:600}",
        ".legend-label{font-size:13px}",
        ".ontology{font-size:14px;font-weight:650}",
        "</style>",
        f'<rect width="{WIDTH}" height="{height}" fill="{universal_outline}"/>',
        f'<rect x="40" y="32" width="1520" height="{height - 64}" rx="28" fill="var(--shadow)" stroke="var(--raised)" stroke-width="3"/>',
        svg_text(78, 86, "Hollow Grove Keyboard Grove", "title", "start"),
        svg_text(
            78,
            116,
            "Stonebend foundation · House landmarks · domain plants · operational tools",
            "subtitle",
            "start",
        ),
        '<rect x="62" y="142" width="1476" height="544" rx="24" fill="var(--outline)" stroke="var(--raised)" stroke-width="2"/>',
        svg_text(82, 176, "THE KEYBOARD ECOLOGY", "section-title", "start"),
    ]

    row_y = [202, 294, 386, 478, 582]
    for (start_x, keys), y in zip(keyboard_rows(), row_y, strict=True):
        x = start_x
        for key, units in keys:
            key_width = KEY_UNIT * units + KEY_GAP * (units - 1)
            parts.append(
                render_key(
                    x,
                    y,
                    key_width,
                    key,
                    landmarks,
                    plants,
                    tools,
                    house_ids,
                    colors,
                )
            )
            x += key_width + KEY_GAP

    legend_y = 718
    parts.extend(
        [
            svg_text(78, legend_y, "READING", "section-title", "start"),
            '<rect x="78" y="737" width="24" height="24" rx="6" fill="#163A5F" stroke="#79B7D8"/>',
            svg_text(112, 755, "Landmark — House meaning", "legend-label", "start"),
            '<rect x="326" y="737" width="24" height="24" rx="6" fill="#5A1F2A" stroke="#E07A86"/>',
            svg_text(360, 755, "Plant — admitted domain", "legend-label", "start"),
            '<rect x="596" y="737" width="24" height="24" rx="6" fill="var(--raised)" stroke="#79B7D8" stroke-dasharray="5 4"/>',
            svg_text(630, 755, "Tool — operates on the Grove", "legend-label", "start"),
            '<rect x="920" y="737" width="24" height="24" rx="6" fill="var(--deep)" stroke="var(--raised)"/>',
            svg_text(954, 755, "Provisional plot — useful, not admitted", "legend-label", "start"),
            svg_text(78, 810, "ADMITTED PLANTS AND BRANCHES", "section-title", "start"),
        ]
    )

    card_width = 704
    card_gap = 28
    for index, plant in enumerate(registry["domain_anchors"]):
        parts.append(
            render_plant_card(
                plant,
                78 + (index % plant_columns) * (card_width + card_gap),
                832 + (index // plant_columns) * 238,
                card_width,
                house_ids,
                colors,
            )
        )

    flow_y = 832 + plant_rows * 238 + 20
    flow = ["Stonebend foundation", "House affinity", "Plot", "Plant", "Branch", "Fruit"]
    flow_x = 90
    for index, item in enumerate(flow):
        box_width = 208 if index == 0 else 174
        parts.append(
            f'<rect x="{flow_x}" y="{flow_y}" width="{box_width}" height="42" rx="12" fill="var(--raised)" stroke="var(--foundation)"/>'
        )
        parts.append(svg_text(flow_x + box_width / 2, flow_y + 27, item, "ontology"))
        flow_x += box_width
        if index < len(flow) - 1:
            parts.append(svg_text(flow_x + 15, flow_y + 27, "→", "ontology"))
            flow_x += 30
    parts.extend(
        [
            "</svg>",
            "",
        ]
    )
    rendered = "\n".join(parts)
    for token, value in {
        "var(--outline)": universal_outline,
        "var(--shadow)": universal_shadow,
        "var(--raised)": universal_raised,
        "var(--text)": general_text,
        "var(--foundation)": foundation_text,
        "var(--deep)": universal_shadow,
    }.items():
        rendered = rendered.replace(token, value)
    return rendered


def render_markdown(registry: dict) -> str:
    lines = [
        "# Hollow Grove Keyboard Grove Map",
        "",
        "Status: deterministic generated projection; not constitutional authority",
        "",
        "> Generated from `packaging/hollow-grove-general/etc/hollow-grove/keyboard-grove.json`.",
        "> Do not edit this file or the SVG by hand.",
        "",
        "![Hollow Grove keyboard ecology map](hollow_grove_keyboard_grove_map.svg)",
        "",
        "## Reading",
        "",
        "Stonebend supplies the stable foundation. House landmarks carry direct",
        "constitutional meaning; plants occupy mnemonic letter plots; modifier",
        "branches remain inside a plant; applications and actions are replaceable",
        "fruit. Tools use the same binding grammar but operate on the desktop",
        "environment instead of opening a domain.",
        "",
        "## Admitted Plants",
        "",
        "| Plot | Plant | Affinities | Branches |",
        "|---|---|---|---|",
    ]
    for plant in registry["domain_anchors"]:
        branches = "; ".join(
            f'{binding["lane"]}: {binding["action"]}' for binding in plant["bindings"]
        )
        lines.append(
            f'| `{plant["key"]}` | {plant["domain"]} | '
            f'{" + ".join(plant["affinities"])} | {branches} |'
        )
    lines.extend(
        [
            "",
            "All other letter groups remain provisional launch families. Appearance",
            "on the keyboard does not admit a plant.",
            "",
            "## Regeneration",
            "",
            "```bash",
            "./packaging/hollow-grove-general/generate-keyboard-grove-map.py",
            "./packaging/hollow-grove-general/verify-keyboard-grove.sh",
            "```",
            "",
        ]
    )
    return "\n".join(lines)


def write_if_changed(path: Path, content: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    if path.exists() and path.read_text(encoding="utf-8") == content:
        return
    path.write_text(content, encoding="utf-8")


def check_output(path: Path, expected: str) -> bool:
    if not path.exists():
        print(f"missing generated keyboard map: {path}", file=sys.stderr)
        return False
    if path.read_text(encoding="utf-8") != expected:
        print(f"stale generated keyboard map: {path}", file=sys.stderr)
        return False
    return True


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--check",
        action="store_true",
        help="fail when generated outputs differ from the registry and palette",
    )
    arguments = parser.parse_args()

    registry = load_json(REGISTRY_PATH)
    palette = load_json(PALETTE_PATH)
    svg = render_svg(registry, palette)
    markdown = render_markdown(registry)
    outputs = [
        (ARTIFACT_SVG, svg),
        (ARTIFACT_MD, markdown),
        (INSTALLED_SVG, svg),
    ]

    if arguments.check:
        return 0 if all(check_output(path, content) for path, content in outputs) else 1

    for output_path, content in outputs:
        write_if_changed(output_path, content)
        print(f"generated keyboard grove map: {output_path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
