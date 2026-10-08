# Hollow Grove Foundation Freeze V1

Status: Foundation V1 release-candidate lock for the art phase
Scope: everything beneath gameplay presentation

The foundation is now considered stable enough for an art-focused phase. The
following contracts are frozen unless a change is explicitly marked as a
foundation migration:

- the universal kernel and constitutional runtime authority boundary;
- the four House jurisdictions and Central Junction's coordination-only role;
- one Rampart and exactly two continuous witnesses, Roi and Roselina, known
  in-world as the Twofold Witness;
- the continuity/embodiment witness seam, form transitions, and operational
  session records (the legacy technical API may still use `Guide` names);
- typed evidence, advisory, decision, refusal, provenance, and consequence
  boundaries;
- Rampart binding, archive checksums, schema migrations, and replay rules;
- the Platinum Path (`path.platinum.v1`), also called the Plath;
- the Platinum Path Cultural Canon's shared five tests and local definitions;
- the Rust/Godot protocol's rule that Godot presents records but never decides.

The universal recursion implementation and neutral semantic implementation are
separate crates. The recursion crate is dependency-free and contains no world,
House, route, character, evidence, or lore terms. Art consumes the packaged
palette projection and asset manifest through Godot's renderer seam; it never
imports either kernel into presentation code.

The art-phase Synthesis interpretation remains above this boundary. Current
ladder names such as Gremlin and Goblin, plus the separate Flynt Gargoyle
branch, may be rendered as bounded Hueman transformations, while
Current-Synthesis techniques such as Signal and Brace remain
world/presentation effects with measurable setup, load, failure, and recovery.
None of these names or meanings are added to the pure recursion kernel.

## What art may change

Art work may change sprites, tiles, palettes, animations, sound, layout,
dialogue presentation, accessibility copy, and other renderer-owned assets. It
must consume the existing stable IDs and projections rather than inventing
new authority, identity, House, Rampart, or kernel state.

## What requires a foundation migration

Any change to a stable ID, archive schema, causal ordering rule, Guide
identity, House authority, refusal semantics, advisory commitment boundary,
Rampart binding, or protocol ownership requires:

1. a versioned migration note;
2. a replay fixture before and after migration;
3. an invariant-audit update;
4. full Rust validation;
5. explicit approval before art-facing projections are changed.

The functional-lore role envelope is the first applied additive migration under
this rule. Its V1→V2 record is documented in
`FUNCTIONAL_LORE_ARCHIVE_V2_MIGRATION.md`; it changes responsibility metadata,
not constitutional authority or the neutral recursion kernel.

The later Sympiote foundation uses an applied additive gameplay V4→V5 migration
documented in `docs/SYMPIOTE_FOUNDATION_MIGRATION_V1.md`. The user explicitly
authorized integration of the accepted foundation Form into the core/runtime.
It reuses host-specific grafting, preserves legacy state, and creates no new
House authority or neutral-kernel meaning.

## Required validation

The later approved object-case migration is documented in
`docs/SURVEY_FRAME_MIGRATION_V1.md`. The user authorized the doorway direction,
case responsibilities and continued implementation. Gameplay V5→V6 preserves
old progress and adds typed object trials, resident decisions and recovery.
The read-only workshop panel uses existing renderer constraints and acquires
no authority.

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --offline -- -D warnings
cargo test --workspace --offline
./verify-art-assets.sh
godot --headless --path hueman_godot --script res://tests/retro_overworld_semantics_smoke.gd
./verify-playable-lifecycle.sh
git diff --check
```

The live restart/load lifecycle should additionally be run outside restricted
sandboxes with `./verify-playable-lifecycle.sh`.

The foundation's governing proof is the executable kernel audit in
`src/kernel_invariants.rs`. Its output is exposed as the read-only
`kernel_audit` gameplay projection, allowing art and presentation work to
surface foundation health without acquiring authority over it.

## Foundation V1 RC disposition

The 2026-08-10 RC audit passed 1,320 Rust tests across the workspace, strict
all-feature Clippy, deterministic Aseprite re-export and constitutional-palette
verification, a fresh-cache Godot renderer smoke check, and the live
save/restart/load lifecycle. Long-session response caches, request replay,
regional indexes, party projections, and save fallback are bounded and covered
by focused stress or recovery tests.

Tross succession already has typed constitutional and gate-order tests. Its
production Rampart archive binding remains deliberately deferred until a
first playable succession case exists; creating placeholder gameplay solely to
close that content dependency would violate this freeze rather than strengthen
it.
