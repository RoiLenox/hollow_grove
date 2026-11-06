# HUEMAN-01: Create Your Hueman

Date: 2026-11-04. Status: isolated source prototype; production integration is
proposed below. Resumes the unfinished November 3 HUEMAN-01 milestone.

## Run and try it

From `/home/warren/Projects/active/hollow_grove` in a graphical desktop session:

```sh
godot --path hueman_godot res://tools/hueman_01/creator.tscn
```

Choose Skin, Face and Hair, then **Save appearance**. Close and relaunch to load
the same avatar automatically, or use **Load saved** to discard unsaved edits.
Select the visual Gremlin comparison and return to Hueman: the customized base
appearance remains intact. Click the floor or **Walk in preview** and use WASD
or the arrow keys. Tab returns focus to the controls. Movement translates a
standing pose; idle/walk animations have not been authored.

The page scrolls in short windows and follows keyboard focus. It stays centered
with a bounded studio on tall displays. This fixes the recovered prototype's
wrapped-text sizing feedback, which could grow the page outside its window.
An ordinary SSH shell can run the headless checks below; the interactive scene
needs a graphical session. Its launch command does not start the Core service.

## Implemented scope

- Exactly three customization categories: six natural Skin samples, four Face
  presets, and Hair with five styles and four colors. Shaved retains the chosen
  hair color for later use. Presets are illustrative choices, not new canon.
- A newly drawn 24×32 preview humanoid with fixed body/leg geometry and clothing.
  No body, height, House, statistics or growth controls. This canvas does not
  replace the existing production atlas or approve production sprite bounds.
- A separate avatar ID and strict version-1 appearance record, saved with a
  revision check, exclusive local writer lock, temporary replacement file and
  backup of the previous valid record. Invalid files and stale writers are
  refused without replacing the saved appearance.
- A visual Gremlin comparison drawn with its existing v2.3 binding. The selected
  view is never saved as a gameplay Form. Returning to Hueman restores identical
  customized pixels. No Synthesis event or unlock is issued.
- A focused local movement area. Editing appearance controls does not move the
  avatar. There is no networking, world collision or gameplay mutation here.

The saved record contains only `schema_version`, `avatar_id`, `revision`, and
`appearance`. Appearance contains `skin`, `face`, and `hair: {style, color}`.
The preview ID is not a constitutional Name or a Core identity credential.

## Existing systems inspected

| Area | Existing capability | Relationship to HUEMAN-01 |
| --- | --- | --- |
| Creator | No Skin/Face/Hair creator or customizable base-Hueman appearance record was found in the inspected production renderer, gameplay identities, views or archives. | The new scene lives under `hueman_godot/tools/hueman_01/`. |
| Rendering | `scripts/retro_overworld.gd::_draw_player` selects 16×16 frames from the existing Gargoyle pilot atlas, or draws the fallback actor; `scripts/art_asset_catalog.gd` resolves production assets. | Preview art is generated in memory; no asset manifest, atlas or production renderer changes. |
| Identity | `src/gameplay/identity.rs` maps stable `BeingContinuityId` to participant and institutional identities and tracks active incarnation. `EstablishHuemanIntent` already exists. | The preview profile has no trusted binding to those identities yet. |
| Persistence | `src/gameplay/archive.rs` and `protocol.rs` provide versioned authoritative archives, save/load, revisions and runtime leases. | Preview storage is separate; it does not open, migrate or overwrite Core saves. |
| Collaboration | `src/bin/hollow_grove_game_service.rs` serves local clients through loopback TCP or stdio. Installed-world handoff is described in `docs/TWO_DEVICE_OPERATION_V1.md`. | Neither source sharing nor the local writer lock supplies authenticated multiplayer. Appearance profiles are not part of the installed-world handoff. |

The existing `tools/**` export exclusion keeps the creator out of shipping packs.
It requires the source checkout's v2.3 manifest, just like the chromatic studies.

## Preview persistence and recovery

Default storage is `user://hueman-01-preview/hueman-01-appearance.json`; the
absolute location is printed at launch. For an independent preview profile:

```sh
godot --path hueman_godot res://tools/hueman_01/creator.tscn -- \
  --profile-dir=/absolute/path/to/a/preview-folder
```

Each machine retains its own profile. This milestone transfers source, not
personal appearance records or world saves. Save explicitly before closing;
unsaved edits are not automatically written.

A successful overwrite preserves the previous valid file as
`hueman-01-appearance.json.backup.json`. Corrupt or unsupported files are kept
and reported in the UI. Close every preview using that folder before recovery.
Preserve the complete folder elsewhere, then restore a known valid backup as
`hueman-01-appearance.json`, retaining the damaged original. A `.lock` directory
left by interruption must be moved aside only after confirming no preview is
saving. Retain any `.tmp` file for inspection; it is not automatically committed.
For a stale-writer message, load the saved appearance before saving again.
These are local prototype recovery rules, not a distributed transaction system.

## Stonebend connection assessment

These are next implementation proposals, not new mechanics or adopted lore.

| Step | Reuse | Work still needed |
| --- | --- | --- |
| Enter The Initiation | `FUNCTION_JUNCTION_SEASONAL_WORLD_CYCLE_AND_PERMANENCE_V1.md` places the seasonal Initiation at Aura Field; `src/constitutional/grove_phase.rs` supplies the four-phase cycle. | Bind an appearance profile to the runtime's established continuity identity, with explicit ownership and archive compatibility. Completing a cosmetic profile must not grant a Name, Title, office, proof or Permanence Seal. Design the entry scene separately. |
| First harvesting / recovery | `src/gameplay/living_world.rs` already models field harvest readiness and extraction operations. The adopted Survey Frame case has `RecoverParts` and disclosed custody. | There is no general player harvest/inventory intent in the inspected protocol. Recovered parts in Tess's custody offer the smallest existing activity; free resource gathering still needs material, permission, inventory, depletion and provenance rules. Neither ambient readiness nor a button click grants materials. |
| First construction | `STONEBEND_SURVEY_FRAME_FIRST_CASE_V1.md`, `src/world/survey_frame.rs`, and `AdvanceSurveyFrameIntent` already cover recovery, registration, measurement, rig preparation, failed trials, repair and resident choices. | Present those existing permitted actions and their evidence in an introductory scene. Reuse Tess Splice, Ivo Mark and Mercy Deep's responsibilities. Do not invent a free-building reducer, universal support rating, or XP/Form reward. |
| First Synthesis | `src/gameplay/synthesis_pilot.rs` and `AdvanceWingedGargoylePilotIntent` implement the bounded fragment/trial/clearance/return pilot. Generic `BeginSynthesisIntent` remains unavailable. | Choose the intended first playable Synthesis path. The visual Gremlin comparison is not a new Gremlin unlock. Keep base cosmetics keyed to continuity and render the runtime's active Form separately; returning to Hueman must restore the same appearance. |

A bounded next milestone could bind a saved appearance to one development
world identity, present entry at Aura Field, and expose the existing Survey
Frame recovery/measurement sequence. Acceptance should demonstrate restart,
the same identity throughout, rejected actions leaving state intact, and
appearance preservation through a runtime-controlled Form change and return.
It must not convert the seasonal Initiation into automatic identity authority.

## Remaining design and integration decisions

- Review the preset art, silhouettes, native dimensions, feet pivots and
  outline policy before production art integration or animation work.
- Decide profile ownership, identity binding, portable storage and migration
  for existing players. Do not substitute a locally generated avatar ID for
  the runtime's continuity/participant/institutional IDs.
- Select the intended first harvesting and Synthesis experience. Existing
  bounded cases supply reusable behavior, not a complete new-player loop.
- Authenticated remote collaboration needs its own identity and authority
  work. Preview appearance sharing is not currently implemented.
- Keep v2.3's 34 swatches and 32 bindings fixed. Twelve names remain unassigned;
  v0.4 sizes remain proposals. Legacy outlines/runtime palette migration and
  Sandmanor visual/runtime ancestry differences remain separate decisions.

## Validation and two-host record

The creator runner uses fresh temporary profile folders and three independent
Godot processes. It checks all 480 preset combinations (408 distinct images,
because shaved hides the four retained colors), fixed lower-body pixels,
exact Gremlin pigment, invalid/oversized/versioned records, stale writers,
writer locks, backups, I/O errors, UI save/reload, identical identity and pixels
after restart, Form-view independence, focused movement and window layout.

```sh
python3 tools/verify_hueman_creator.py
python3 -m unittest discover -s tools -p 'test_chromatic*.py'
./verify-art-assets.sh
./verify-godot.sh
godot --headless --editor --path hueman_godot --import
git diff --check
```

All listed checks passed on **both noahsarch and occamsrazer**, with Godot
4.7.2. Per host: 1,515 creator contract checks, 21 create-phase checks and 23
reload-phase checks; 29 chromatic Python tests; both production assets and
their 16-color contract; the three chromatic previews and existing Godot smoke
checks; generated growth projection; editor import and tracked whitespace checks.
The v2.3 and v0.4 validators also passed. These are focused prototype checks, not
a new full gameplay or multiplayer qualification.

Noahsarch rendered the actual save/reload UI on its AMD RX 7900 XTX. The
Hueman and Gremlin captures were inspected after the layout fix. Captures and
logs are under `/home/warren/Projects/audits/hueman01-resume-20261104/`:

- `creator-saved.png` and `creator-saved-gremlin.png`
- `creator-reloaded.png` and `creator-reloaded-gremlin.png`
- `noahsarch-checks.log`, `import.log`, `create-visual.log`, `reload-visual.log`

Occamsrazer's headless results are in `occamsrazer-checks.log` in the same audit
directory on that host. Its interactive desktop rendering remains unverified.

Backups of each host's unfinished work and source hashes are under
`/home/warren/Projects/backups/hueman01-resume-20261104`.
The reviewed `hueman01.patch` added 13 creator scene/script/UID files, the Python
runner and this document to occamsrazer. It updated only `README.md`,
`hueman_godot/README.md` and the creator check invocation in `verify-godot.sh`;
all other 1,318 pre-existing remote source files remained byte-identical.
The patch passed apply and reverse-apply checks. `handoff-results.patch` records
the final report update separately. On noahsarch, resumption changed only the
two README files and the recovered creator/layout-check scripts among 1,335
existing source files; the remaining 1,331 stayed byte-identical. The new
handoff and generated `verify.gd.uid` complete its recovered prototype.
Shared creator files, runner, verification script and Godot README match across
hosts. The root README retains its pre-existing unrelated checkpoint difference.

The source roots remain at different checkpoints: noahsarch `0a81b32`,
occamsrazer `d9fbf15`. Transfer only the reviewed HUEMAN-01 patch after checking
the receiver's current files. No commit, push, package installation or world
handoff is part of this milestone.
