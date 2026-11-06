# Chromatic v2.3 / Growth v0.4 implementation handoff

Date: 2026-11-03. Source root on both hosts:
`/home/warren/Projects/active/hollow_grove`.

## Result and authority

The active design source is v2.3: 34 unchanged colors with 32 approved inverted
Form bindings. Light expresses emergence; dark expresses mastery. Swatch order
and IDs remain stable, while visual Form sequences are explicit separate data.
Historical v1.0 and v2.2 manifests and approval records remain byte-identical.

Growth v0.4 is a separate working art guide. It calculates 32 proposed height
targets using normalized cumulative adjacent RGB distance within five intervals.
Only the Gremlin 0, Goblin 1 and Gargoyle 2 Growth Indices are creator-directed.
All dimensions, the formula/endpoints and 29 other indices remain provisional.
The base Hueman has personalized natural skin and no House swatch/Growth Index.
Gargoyle branches directly from Gremlin; the Flynt chart is not a serial ladder.

Both supplied Markdown documents were copied exactly from Downloads and their
hashes recorded. The companion JSON files named in them were absent from the
searched local files and remote Downloads/Documents/project docs. Data was
reconstructed from their complete tables; the schemas are implementation choices,
not recovered originals. The working growth source stores references, not an
independent palette. Godot's cached projection is generated and checked against
both sources, with their SHA-256 hashes embedded.

## Exact changed paths

Seven existing files were updated on each host:

- `AGENTS.md`: current sources, explicit version/slot/Form/Growth distinctions.
- `README.md`: active v2.3 and working growth entrypoints.
- `docs/design/CHROMATIC_INTEGRATION_PLAN_V1.md`: current architecture, migration
  boundaries, preview commands and unresolved decisions.
- `tools/chromatic_color_standard.py`: v2.3 schema/binding validation, preserved
  historical validation and explicitly versioned color/Form lookups.
- `tools/test_chromatic_color_standard.py`: retain the 19 historical tests and
  add five v2.3 compatibility/inversion/boundary tests.
- `hueman_godot/tools/chromatic_swatch_preview.gd`: updated swatches, four-character
  study and complete 32-Form sizing chart; source-hash checks.
- `verify-godot.sh`: generated-projection check and the third preview mode.

New files:

- `docs/design/HOLLOW_GROVE_CHROMATIC_COLOR_STANDARD_V2_3_CODEX_ALL_IN_ONE.md`
- `docs/design/HUEMAN_CHROMATIC_GROWTH_SYSTEM_V0_4_HEX_DISTANCE_CODEX.md`
- `docs/design/hollow_grove_chromatic_color_standard_v2_3_0.json`
- `docs/design/hueman_chromatic_growth_system_v0_4_hex_distance_working.json`
- `docs/design/CHROMATIC_V2_3_GROWTH_V0_4_HANDOFF_20261103.md` (this record)
- `tools/chromatic_growth.py`
- `tools/test_chromatic_growth.py`
- `hueman_godot/tools/chromatic_growth_preview.json` (generated projection)
- `hueman_godot/tools/chromatic_study_art.gd`
- `hueman_godot/tools/chromatic_study_art.gd.uid`

The existing `tools/**` export exclusion keeps these Godot development resources
out of production game packs. No shipping art, atlas, hitbox, runtime palette,
save format, progression rule, compositor, package or hardware configuration
was modified in this phase.

## Actual validation

These commands passed on **both noahsarch and occamsrazer**:

```sh
python3 tools/chromatic_color_standard.py validate
python3 tools/chromatic_color_standard.py validate --version 2.2
python3 tools/chromatic_color_standard.py validate --version 1.0
python3 tools/chromatic_growth.py validate
python3 -m unittest discover -s tools -p 'test_chromatic*.py'
./verify-art-assets.sh
./verify-godot.sh
godot --headless --editor --path hueman_godot --import
git diff --check
```

Results: 29 Python tests passed per host (19 retained, 10 new); all three palette
versions validated; 32 calculated targets matched every rounded gap/height/pixel
entry in the v0.4 guide. Tests distinguish cumulative path length from endpoint
distance, preserve all old color references, reject ambiguous Form versions,
and retain the working/approved boundary.

Both production assets and their 16-color legacy contract passed asset checks.
Godot 4.7.2 loaded all three review views, validated the four new standing-height
examples and exact pigment usage, and passed existing runtime client, transport,
performance, overworld semantics and survey-frame smoke checks. The generated
growth projection matched both neutral sources. Editor import passed on both.
These focused checks are not a full gameplay or hardware qualification.

Local editor import was rerun successfully outside the sandbox after its editor
socket was blocked. The complete remote check sequence was run explicitly under
`/usr/bin/bash -s` because the remote login shell is Fish. Its successful output
is saved on noahsarch at:
`/home/warren/Projects/audits/chromatic-v23-20261103/occamsrazer-checks.log`.

## Visual review

All three views were rendered on noahsarch's AMD RX 7900 XTX and inspected:

- `/home/warren/Projects/audits/chromatic-v23-20261103/swatches.png`
- `/home/warren/Projects/audits/chromatic-v23-20261103/four-form-study.png`
- `/home/warren/Projects/audits/chromatic-v23-20261103/32-form-growth.png`

The four-character study uses newly drawn in-memory poses with different anatomy,
common feet pivots and equal pixel density: Hueman/Gremlin 32px, Goblin 36px,
Gargoyle 40px standing examples. Their ratios are 1.000×/1.000×/~1.113×/1.250×.
It includes three illustrative natural skin samples, 1×/2× views, dark-background
comparisons and the untouched pilot. Skin/hair samples are not new House tokens.

The actual production pilot was inspected: a 32×32 atlas containing four 16×16
frames. The historical 24×32 reference and world tile dimensions do not prescribe
production bounds. The 32-Form chart contains sizing bars, not 32 finished sprite
designs. The new study is static; no animation or geometry approval is implied.
Occamsrazer verification was headless; desktop GPU rendering remains unqualified.

From the source root, launch:

```sh
godot --path hueman_godot --script res://tools/chromatic_swatch_preview.gd
godot --path hueman_godot --script res://tools/chromatic_swatch_preview.gd -- --study
godot --path hueman_godot --script res://tools/chromatic_swatch_preview.gd -- --growth
```

## Two-host preservation

Noahsarch remains at `0a81b32`; occamsrazer remains at `d9fbf15` on
`os/package-foundation`. Both retain uncommitted work. Backups on each host:
`/home/warren/Projects/backups/chromatic-v23-20261103`.
They contain before-state hashes, unfinished-work archives, source copies and
diffs. The checked transfer is `v23-growth.patch`, followed by the separate
handoff patch. The transfer passed reverse-apply checking on occamsrazer.

Before application, all 1,311 pre-existing remote source files still matched
the backup. Afterwards exactly the seven scoped existing files changed;
1,304 other existing remote source files retained identical bytes. Local source
hashes likewise preserved all 1,305 existing files outside the seven changes.
Twenty-two shared implementation/history files and the README chromatic section
matched across hosts, followed by a separate match for this handoff. The
pre-existing unrelated README checkpoint-link difference was preserved.

No commit, push, deployment, background sync or world handoff was performed.
The whole repositories remain distinct. Recheck both working trees before the
next scoped transfer.

## Open decisions and next refinement

The twelve color names remain unassigned. Legacy Oxford Blue outlines and Flynt
Authority/role mapping still need production review. Sandmanor's approved visual
Satyr/Faun ordering still differs from older runtime ancestry; no prerequisites
or saved Forms were rewritten. Current Synthesis and one active Current Form
remain authoritative.

Next, review the four silhouettes, anatomy and common feet pivots at native size;
then author separate idle/walk frames if accepted. Approve production sprite
bounds and outline treatment before integrating a character into a live scene.
