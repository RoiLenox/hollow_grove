# Chromatic Color Standard: integration and next construction phase

Updated: 2026-11-03. Active design: **v2.3**. Companion sizing: **v0.4 working proposal**.
The V1 in this plan's filename identifies the integration plan, not the palette.

## Sources and approval boundaries

Read the [v2.3 approval](HOLLOW_GROVE_CHROMATIC_COLOR_STANDARD_V2_3_CODEX_ALL_IN_ONE.md)
and [growth v0.4 guide](HUEMAN_CHROMATIC_GROWTH_SYSTEM_V0_4_HEX_DISTANCE_CODEX.md).
The supplied Markdown files were copied byte-for-byte from Downloads. Their
referenced companion JSON files were not found on either host. The neutral data
below was reconstructed from the complete tables, with that provenance recorded;
its schema is an integration choice, not the missing downloads' original schema.

- [Active color source](hollow_grove_chromatic_color_standard_v2_3_0.json):
  all 34 unchanged swatches and 32 creator-approved inverted Form bindings.
- [Working growth source](hueman_chromatic_growth_system_v0_4_hex_distance_working.json):
  branch endpoints, Form references, index labels and the proposed sizing rule.
  It contains no independent color catalog.
- [Historical v2.2](hollow_grove_chromatic_color_standard_v2_2.json) and
  [v1.0](hollow_grove_chromatic_color_standard_v1_0.json), with their approval
  documents, remain unchanged and validated.

Light expresses emergence; dark expresses mastery. This changes visual Form
identity, not gameplay progression. Gremlin now uses Air Superiority Blue,
Goblin uses `#67A6CA`, and Gargoyle uses Silver Gray. Sandmanor reverses each
four-Form visual branch independently. Wine `#5A1F2A` belongs to Hecaton.

All exact geometry and numerical size targets remain working proposals. Only
Growth Indices Gremlin 0, Goblin 1 and Gargoyle 2 are creator-directed; the other
29 indices are working labels. Gargoyle remains a direct Gremlin branch, not a
Goblin descendant. The base Hueman has individual natural skin and no House
swatch or Synthesis Form index.

## Swatches, Form sequences and Growth Indices

Schema 3 separates each House's `shades` from its `forms`. Swatch slots remain
ordered 1–8, dark to light. Existing `chromatic.v2_2.house.*` token IDs retain
their original colors in v2.3; their namespace records when those swatches were
introduced. The 12 unnamed tokens still have null formal names and unassigned
status. No shade value is recomputed from a Form's lineage root.

A Form has a stable `design.form.*` ID, a visual sequence position and an
explicit swatch reference. Growth data references that ID separately.
For example, Gargoyle has visual sequence 1, swatch slot 8, Growth Index 2 and
a working height target of 1.25×. These numbers are not interchangeable.

`resolve(data, slot, source_version=...)` preserves historical color references.
The v1 index map is still 1→1, 2→2, 3→4, 4→6, 5→8. Explicitly versioned
`resolve_form(form_id, source_version=...)` can inspect either the old v2.2
binding or current v2.3 binding. No saved Form or color reference is silently
migrated; runtime enum/save IDs are separate from these design IDs.

The v0.4 proposal sums adjacent Euclidean distances of sRGB channel triples
along each branch, normalizes by that branch's total path, then interpolates
between its working height anchors. It uses neither packed hex integers nor
straight-line distance from the first endpoint. Indices do not calculate size.
The resulting height is a review aid only; palette identity is never derived
back from height or used to change gameplay statistics.

`tools/chromatic_growth.py project` generates
`hueman_godot/tools/chromatic_growth_preview.json` from those two sources.
The projection contains derived values and source hashes, is excluded from
shipping packs with the other tools, and is checked for drift. Do not hand-edit
it or treat its cached colors as a second palette.

## Isolated review views

The swatch view labels both swatch slots and assigned Form sequences. The four
character study shows natural Hueman → Gremlin → Goblin and the direct
Gremlin → Gargoyle alternate branch. It draws separate in-memory anatomy for
each Form, keeps a common feet pivot and display pixel density, and includes
1×/2× views, a dark background sample, multiple illustrative Hueman skin tones,
and the unchanged pilot for comparison.

Standing-height examples are 32, 32, 36 and 40 pixels. They are not new production
frame bounds. Inspection of the actual asset manifest and Aseprite generator
confirmed that the current pilot is a 32×32 sheet containing four 16×16 frames.
The historical 24×32 study reference and 16×16 world tiles are separate concepts.
No existing atlas, source sprite, collision shape or world pivot was changed.

The 32-Form chart shows all five independent sizing intervals, exact binding
hexes, relative heights, illustrative pixel counts, and distinct GI/sequence/slot
labels. Its Flynt row is a comparison, not eight consecutive unlocks. These are
measurement bars, not claims to have authored 32 finished characters.

All views run from the source checkout without loading gameplay services or
saves. They do not enact Current Synthesis, grant XP/Forms/Agency/office, or
change the one-active-Current-Form rule.

## Verification and preview commands

From the repository root on either host:

```sh
python3 tools/chromatic_color_standard.py validate
python3 tools/chromatic_color_standard.py validate --version 2.2
python3 tools/chromatic_color_standard.py validate --version 1.0
python3 tools/chromatic_growth.py validate
python3 tools/chromatic_growth.py project --check
python3 -m unittest discover -s tools -p 'test_chromatic*.py'
./verify-art-assets.sh
./verify-godot.sh
```

The Godot runner checks generated data, all three isolated views and existing
runtime smoke tests. For the editor import check, run
`godot --headless --editor --path hueman_godot --import`.
A normal rendering display is required for visual inspection/captures:

```sh
godot --path hueman_godot --script res://tools/chromatic_swatch_preview.gd
godot --path hueman_godot --script res://tools/chromatic_swatch_preview.gd -- --study
godot --path hueman_godot --script res://tools/chromatic_swatch_preview.gd -- --growth
```

Add `-- --capture=/absolute/path/to/swatches.png` to the first command, or
`--capture=/absolute/path/to/preview.png` after the existing `--study` or
`--growth` option. The destination directory must exist. For headless view
checks, use `--headless` and append `-- --validate-only`, optionally with
`--study` or `--growth`.

GPL output defaults to v2.3:
`python3 tools/chromatic_color_standard.py gpl`.
Historical versions require explicit `--version`.

## Remaining decisions and production dependencies

| Concern | Current boundary |
| --- | --- |
| Twelve formal color names | Still intentionally unassigned; do not invent canon. |
| Working size targets | Formula, endpoints, exact anatomy and 29 indices are proposals; no approved production geometry. |
| Sandmanor ancestry | Visual Gnome→Satyr and Elf→Faun coexist with older runtime Gnome→Faun and Elf→Satyr. `src/sandmanor_lineage.rs` retains the older transitions; governance already has the newer visual order. No prerequisites or saves changed. |
| Flynt | Preserve three sibling families, Gargoyle/Werewolf/Merman primary mastery for Chimera, Manticorp Form/organization/Tross distinctions and explicit cross-pairs. |
| Outline | Legacy Oxford Blue `#0A1024` is darker than Void Blue `#0B1026`. Its historical assets remain intact; no production replacement was chosen. |
| Runtime palette | The 16-color constitutional/runtime contract, pure-black restrictions, Flynt Authority overlay and dark/primary/highlight role mapping still need explicit migration. |
| Materials and typography | Natural skin/hair, Diamond/Crystal/Jade/Opal, mirrors, oil, lighting and accessibility remain independent. The preview uses Godot's fallback font; no font assets were added. |
| OS consumers | Waybar, edition TOML and keyboard map still use the legacy runtime palette. No production consumer used v2.2 Form bindings; these require their own reviewed migrations. |

## Working across both hosts

Noahsarch and occamsrazer have different commits and independent unfinished
work. Backups for this phase are under
`/home/warren/Projects/backups/chromatic-v23-20261103` on each host.
Transfer reviewed patches after checking current source hashes; never mirror
repositories, `.git`, caches, saves or machine configuration. Source changes
are separate from the [installed-world handoff](../TWO_DEVICE_OPERATION_V1.md).

The [earlier v2.2 record](CHROMATIC_TWO_HOST_HANDOFF_20261103.md) remains a
historical account, including its older three-pose study. Current results and
changed paths are in the [v2.3/v0.4 handoff](CHROMATIC_V2_3_GROWTH_V0_4_HANDOFF_20261103.md).
Commits, pushes and deployments remain distinct review steps.

The next bounded refinement is a review of the four silhouettes, head/body
proportions and feet pivots, followed by separately authored idle/walk frames
if the study is accepted. Settle production bounds and outline policy before
putting a character into a live area.
