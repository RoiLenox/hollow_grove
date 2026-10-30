# Hollow Grove Operational Entrypoint V1

Status: active implementation guide

This is the short path from Hollow Grove's constitutional meaning to the
currently playable experience. It does not outrank the Compromise, the House
constitutions, the Guide seam, or the runtime contracts.

## Current source and release status — 29 October 2026

This is the current implementation-status entry point. Older audit and
continuation documents retain their dated evidence; their historical "next"
steps do not override this summary. Re-run `hollow-grove architecture --json`
and `python3 tools/verify-backend-contracts.py` for current declared coverage.

The reviewed source checkpoint `d67c5f0` includes the two Boardwalk levels,
read-only Registrar, all four bounded House governance adapters, and current
geography signage with historical IDs preserved. Architecture V3 declares
23 components and 14 bindings, including ten headless House handoffs whose
gameplay presentations remain deferred. These counts describe declared
bindings, not completed actions in a player's save.

The subsequent lean-release work adds reproducible five-package ownership,
source-only geography validation, deterministic Registrar generation, optional
graphical dependencies and separate edit/integration verification commands.
The October 27 occurrence inventory remains an immutable historical record;
`contracts/geography/active-source.v1.json` records the current classifications
and three reviewed later checkpoint changes without resolving source law.

Deployment is recorded in per-host installation receipts, not inferred from a
source commit or test count. The October 29 laptop installation already included
its local fifth CLI package; the repository now owns that recipe. Release
manifests must identify all five products and their exact source/payload hashes.

The existing one-mint path remains incomplete: Route 2's four-House founding
model is approved design, while the [founding procedure](docs/HOLLOW_GROVE_FOUNDING_PROCEDURE_AND_FIRST_ADOPTION_DRAFT_V1.md)
and [civic administration](docs/HOLLOW_GROVE_FOUNDING_CIVIC_ROLLS_AND_ADMINISTRATION_DRAFT_V1.md)
retain pending terms/evidence. Q terminal succession remains fail-closed.
General Synthesis, Synblock crafting, Synbalm healing, grade advancement,
shared institutional integration and remote Hueman collaboration are separate
unfinished product work. The backend decision register, existing authority map
and founding dockets own those choices; optimization does not enact them.

Use `./verify-edit.sh` during narrow edits, `./verify-integration.sh` for source
qualification, and the existing extended release checks before final issuance.
Keep authored source, compatibility fixtures, current/rollback packages and
state evidence. Retire only reviewed derived caches under the
[retention policy](docs/BUILD_RETENTION.md).

## The One-Line Model

```text
Rust owns consequence → Godot presents consequence → Niri supplies focus and geometry
```

Hueman is the player's persistent Being and rendered spatial face. Hollow
Grove is the authoritative world, institutional, and persistence layer. The
Guide seam changes layer expression without replacing identity:

```text
outside: Roï + Rosaliña
inside:  Roi + Roselina
```

## What Is Playable Now

- Guide entry, exit, restart recovery, and save continuity;
- Hueman establishment and four-direction movement with Rust-owned collision;
- all ten constitutional routes and the three singular interior surfaces;
- physical exits, scheduled people, dialogue, deterministic world shifts, and
  authored living-region cases;
- Deep Pressure's cross-region campaign and four House endings;
- Hueman plus five companions, recruitment, lead changes, field actions, and
  refusal/recovery behavior;
- checksummed save/load and deterministic replay;
- the normalized Hueman/Hollow Grove/Niri coordinate contract and paired-window
  projection.

## The Golden Loop

```text
launch
  → synchronize outside Guides
  → cross into Hollow Grove
  → establish or resume Hueman
  → traverse and inspect
  → disclose faculty evidence
  → support a bounded lawful choice
  → receive an authoritative consequence
  → save
  → restart and load
  → leave and re-enter without identity or position drift
```

Every step must remain an authoritative runtime event. Godot may request and
animate it; it may not decide it. Niri focus, placement, and normalized window
geometry are presentation evidence only.

## Ownership Boundaries

| Concern | Owner | Presentation consumer |
|---|---|---|
| identity, authority, evidence, consequences | Rust runtime and House modules | Godot view |
| Guide crossing and continuity | Guide coordinator and gameplay service | Godot transition projection |
| map, collision, facing, party, cases | Rust gameplay reducers | retro overworld |
| normalized screen geometry | spatial bridge and shared JSON contracts | Hueman screen shell |
| window focus and placement | Niri | pair overlay |
| lore meaning | constitutional and functional-lore authorities | all clients |

## Explicitly Deferred

These are real protocol reservations, not missing presentation tricks:

- general combat and encounter reducers;
- equipment and broad progression;
- authoritative Synthesis decisions;
- procedural generation;
- any application or Godot shortcut that would manufacture House authority,
  consent, Clearance, Title, recognition, or world mutation.

The next gameplay expansion should complete one faculty-driven action through
evidence, choice, consequence, persistence, and replay before adding another
large region or lore subsystem.

## Verification Path

```bash
./verify-playable-lifecycle.sh
godot --headless --path hueman_godot \
  --script res://tests/retro_overworld_semantics_smoke.gd
cargo test --offline --test guide_operational_seam \
  --test gameplay_boardwalk --test party_recruitment
```

If the full lifecycle script cannot bind its loopback service in a restricted
environment, run it in a normal user session; the individual Godot and Rust
tests remain useful without widening any authority boundary.
