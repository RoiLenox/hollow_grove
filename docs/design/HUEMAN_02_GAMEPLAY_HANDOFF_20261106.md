# HUEMAN-02: persistent character introduction

Status: source development milestone, continuing HUEMAN-01. This connects the
existing Skin/Face/Hair creator to the authoritative Rust runtime and the
existing Stonebend recovery case. It does not publish or install a new package.

From a graphical session in the source checkout:

```sh
./run-hollow-grove-game.sh --dev --hueman
```

Choose an appearance and select **Enter Stonebend**. Subsequent launches resume
the saved character automatically. Move with WASD/arrows; face the Aura Way
witness marker and press Enter; press O for Tess's repair bench. The existing
Receive Need and Recover Parts actions complete the introduction. Further
measurement/repair choices remain available through the same existing case.

F5 saves. F10 or window close saves before exiting; Escape cancels a pending
exit. Failed send, disconnect, invalid response or missing acknowledgement
keeps the game open. An uncertain save times out after ten seconds and reconnects;
wait for synchronization, then retry. Canceling an exit cannot undo a save
already accepted by Core. The game exits only on the matching successful save
acknowledgement. These are local-runtime recovery rules, not distributed account
or multiwriter guarantees.

The introductory title **The Initiation** describes this character-entry
exercise on Aura Way. It does not relocate or enact the seasonal Initiation at
Aura Field, advance the world phase, or issue authority. The read-only baseline
statistics come from the Hollow Deck and use its Strength/Speed/Attack/Defense
labels. They are not Body/Soul/Mind/Spirit faculty scores.

Profile ownership, identity binding, legacy saves and V6→V7 compatibility are
specified in [the migration record](../HUEMAN_PROFILE_MIGRATION_V1.md). Normal
installed launches and their saves are a separate world/session. An independent
source save root can be selected with `HOLLOW_GROVE_CHARACTER_SAVE_ROOT`.
The source launcher pins its new service to loopback and ignores an inherited
shared-session configuration.

Cosmetic samples have one owned catalog at
`hueman_godot/protocol/hueman_cosmetics_v1.json`. Skin/hair choices and fixed
clothing materials have no House or Form semantics. Both the creator and
world renderer consume this catalog; Rust checks accepted preset IDs. A frozen
pixel digest verifies all 480 appearances against the pre-refactor HUEMAN-01
renderer. All 34 v2.3 swatches, 32 Form bindings and historical palettes remain
unchanged. The shipping-presentation scan continues to reject local color
literals; isolated development tools are excluded only while the export preset
explicitly excludes them, and shipping scripts may not depend on those tools.

Art retains the existing static 24×32 customized Hueman and the legacy 16×16
Gargoyle pilot frames. No new production dimensions, outline policy, growth
geometry, animation set or progression mechanism is approved by this milestone.
Review their in-world transition before directional animation and package work.

Run the source qualification with:

```sh
./verify-integration.sh
```

This includes formatting, geography/Registrar/backend checks, Python suites,
art/keyboard checks, strict Clippy, both Rust feature configurations, Godot
smoke checks, the existing gameplay lifecycle, and the new HUEMAN-02 lifecycle.
The lifecycle checks require local loopback sockets and disposable save space.
`python3 tools/verify_hueman_lifecycle.py --graphical --artifacts /absolute/path`
additionally captures the four stages in a graphical session.

Two-host recovery snapshots are under
`/home/warren/Projects/backups/hueman02-integration-20261106/` on each host.
The laptop's exact pre-integration source is also retained on
`preserved/occamsrazer-before-hueman02-20261106`. Shared chromatic/HUEMAN-01 work
is checkpoint `a923d44`; HUEMAN-02 is a separate change. Transfer reviewed Git
bundles/patches after checking the receiving tree; never copy `.git` or mirror
saves, caches or machine settings.

The creator/entry scenes remain under `tools/**`, excluded from exported packs.
Next work is visual acceptance and an explicit production entry/export design,
followed by an identified package candidate and installed canary checks. General
Synthesis, Synblock crafting, Synbalm healing, grade advancement and authenticated
multiplayer remain separate work.
