# Chromatic v2.2: two-host resumption record

Date: 2026-11-03. Source root on both hosts:
`/home/warren/Projects/active/hollow_grove`.

## Current state

The unfinished November 1 work already contained the approved v2.2 manifest,
validator, 19 tests, swatch preview and static Gremlin→Gargoyle study. Resumption
verified those implementations, corrected stale v1-only guidance, and shared
the scoped chromatic integration with occamsrazer.

| Host | Source checkpoint | Work preserved |
| --- | --- | --- |
| noahsarch | `0a81b32`, `os/package-foundation` | Existing uncommitted chromatic implementation and all approved records |
| occamsrazer | `d9fbf15`, `os/package-foundation` | Existing uncommitted runtime, device, packaging, performance and machine changes |

The repositories retain different checkpoints. Matching chromatic files do not
mean the entire repositories or installed games are synchronized. There is no
background source synchronization, new commit, push, package installation or
deployment from this pass.

## Changed paths and transfer scope

Changes made on noahsarch during resumption:

- `AGENTS.md`: point at active v2.2, preserve historical records and explicit
  version migration, and record the two-host source workflow.
- `README.md`: point at v2.2 and its 34 colors / 32 visual Form assignments.
- `docs/design/CHROMATIC_INTEGRATION_PLAN_V1.md`: update the integration status,
  conflicts, actual 16×16 frame dimensions, both preview commands and next steps.
- `docs/design/CHROMATIC_TWO_HOST_HANDOFF_20261103.md`: this record.

The reviewed patch sent to occamsrazer added the following chromatic files that
were already present on noahsarch, plus this record:

- `AGENTS.md`
- `docs/design/CHROMATIC_INTEGRATION_PLAN_V1.md`
- `docs/design/CODEX_HOLLOW_GROVE_CHROMATIC_COLOR_STANDARD.md`
- `docs/design/HOLLOW_GROVE_CHROMATIC_COLOR_STANDARD_V2_2_CODEX_ALL_IN_ONE.md`
- `docs/design/hollow_grove_chromatic_color_standard_v1_0.json`
- `docs/design/hollow_grove_chromatic_color_standard_v2_2.json`
- `tools/chromatic_color_standard.py`
- `tools/test_chromatic_color_standard.py`
- `hueman_godot/tools/chromatic_swatch_preview.gd`
- `hueman_godot/tools/chromatic_swatch_preview.gd.uid`

Only three existing remote files received scoped hunks: `README.md`,
`verify-godot.sh` (two preview checks) and `hueman_godot/export_presets.cfg`
(exclude development tools from exported game packs).

Backups on each host are under:
`/home/warren/Projects/backups/chromatic-resume-20261103T085623`.
They include before-state source hashes, existing diffs and an archive of
unfinished work. The transfer patch is `chromatic-v2_2.patch`; this record is
added separately by `chromatic-handoff.patch`. Source hashes verified that all
1,297 other existing remote source files retained their original bytes. Local
comparison found only the three intended guidance edits to existing files.

The twelve shared implementation/guidance files matched byte-for-byte after
transfer, and the README chromatic section matched separately. The README has
a pre-existing difference: only noahsarch links its later software checkpoint.
That unrelated difference was retained. This handoff is shared identically too.

## Verification results

The following passed on both hosts with Godot 4.7.2:

- v2.2 validation: 34 distinct colors/slots, exact names/status and Form
  assignments, RGB arithmetic, provenance and compatibility rules.
- v1.0 validation: all 22 original tokens and approval records preserved.
- `python3 -m unittest discover -s tools -p test_chromatic_color_standard.py`:
  19 tests passed.
- `./verify-art-assets.sh`: both historical production assets and their 16-color
  contract passed.
- `./verify-godot.sh`: swatch and study loads, runtime client, performance policy,
  transport, overworld semantics and survey-frame smoke checks passed.
- Tracked diff whitespace checks passed. The approved v2.2 Markdown contains
  three intentional two-space Markdown line breaks; they were preserved exactly.

The transfer patch also passed reverse-apply checking on occamsrazer. No Rust,
gameplay/save, constitutional palette or production asset files were changed by
this integration. Smoke checks are not a full gameplay or GPU qualification.

On noahsarch, both previews were rendered on the AMD RX 7900 XTX and visually
inspected for complete labels, swatches and integer pixel scaling. Captures are:

- `/home/warren/Projects/audits/chromatic-20261103/swatches-noahsarch.png`
- `/home/warren/Projects/audits/chromatic-20261103/gremlin-gargoyle-noahsarch.png`

Occamsrazer verification was headless; its desktop rendering remains a separate
visual check. The study consists of three static draft poses and an unchanged
historical pilot comparison, not completed transformation animation.

## Continue here

From the source root in a desktop terminal on either host:

```sh
godot --path hueman_godot --script res://tools/chromatic_swatch_preview.gd
godot --path hueman_godot --script res://tools/chromatic_swatch_preview.gd -- --study
```

The next bounded design step is to review the Gremlin→Gargoyle silhouette at
native size and against dark scenes before authoring idle/walk frames. The
approved palette is complete; twelve formal color names remain unassigned.
Legacy Oxford Blue outlines, Flynt Authority mapping and the mixed Sandmanor
visual/runtime Form ordering remain unresolved production migration decisions.
See the [integration plan](CHROMATIC_INTEGRATION_PLAN_V1.md) for exact boundaries.

Before the next shared edit, inspect status/diffs on both hosts. Transfer only
reviewed source changes after checking the peer's current files; do not mirror
repositories, saves, caches or machine configuration.
