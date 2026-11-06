# Hollow Grove project guidance

For Hueman and Hollow Grove art/color work, read
`docs/design/HOLLOW_GROVE_CHROMATIC_COLOR_STANDARD_V2_3_CODEX_ALL_IN_ONE.md`,
`docs/design/HUEMAN_CHROMATIC_GROWTH_SYSTEM_V0_4_HEX_DISTANCE_CODEX.md` and
`docs/design/CHROMATIC_INTEGRATION_PLAN_V1.md` first. The shared design-token
source is `docs/design/hollow_grove_chromatic_color_standard_v2_3_0.json`.
Its 34 approved hexes, 32 visual Form assignments and equations are locked in
v2.3: light means emergence, dark means mastery. Twelve color names remain
UNASSIGNED; do not invent canonical names. Keep v1.0, v2.2 and their approval
records unchanged as history. Swatch IDs/colors retain their v2.2 identities;
Form bindings are separate and require an explicit source version. Old House
levels map 1→1, 2→2, 3→4, 4→6, 5→8; references require an explicit source version.
A revision requires a new version and creator approval; consumers read or
project the active source. Validate with `python3 tools/chromatic_color_standard.py`
and `python3 -m unittest discover -s tools -p 'test_chromatic*.py'`.

`docs/design/hueman_chromatic_growth_system_v0_4_hex_distance_working.json`
contains separate review-only sizing inputs. Only Growth Indices Gremlin 0,
Goblin 1 and Gargoyle 2 are creator-directed; the formula, exact dimensions and
29 other indices remain working proposals. Swatch slot, visual Form sequence
and Growth Index are distinct. Gargoyle is a direct Gremlin branch, never a
Goblin descendant. Regenerate the Godot projection with
`python3 tools/chromatic_growth.py project`; verify with `project --check`.
Use `--study` for the four poses and `--growth` for the 32-Form comparison chart.

The existing 16-color constitutional/runtime palette remains a compatibility
contract pending the documented migration decisions. Design approval does not
authorize changing gameplay law, saves, material meanings, or system settings.
Use the isolated swatch preview before migrating production assets or themes.

Noahsarch and occamsrazer may have different commits and independent uncommitted
work. Inspect both before sharing changes; use reviewed patches with backups,
never mirror their repositories or copy `.git`, saves, caches or machine settings.
The integration plan records the current source-sharing and preview workflow.
