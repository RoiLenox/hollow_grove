# Current Synthesis (Hollow Grove) V1

This document is the canonical V1 specification after the refinement and optimization pass.

`FIRST_SYNTHESIS_v0.2.0.md` remains the historical milestone lock.
This document is the cleaned V1 freeze.

## V1 Freeze

V1 is frozen at the semantic level.

Refinement in this phase may:

- reduce duplication;
- tighten internal API boundaries;
- improve measured hot paths;
- correct stale public witness/support text;
- simplify extension points.

Refinement in this phase must not:

- change the canonical kernel topology;
- change Miss or Kiss semantics;
- change transactional landing guarantees;
- add V2 systems.

## Canonical Kernel Witness

Point
→ Triway
→ Fourway
→ HollowGrove
→ CurrentSeam
→ AuraBeam
→ Point² (Landed Point)

Point² is the single landed state.
Point² becomes the next pass's Point without an intermediate history stage.

## FrameState

FrameState contains:

- Frame
- Current Prism
- Flow
- Glow

Frame remains opaque gameplay identity.
Flow remains opaque Current learnset.
Glow remains opaque Aura learnset.

## Current Prism

The Current Prism remains the 1:1 attribute map.
One stored Prism value equals one gameplay attribute unit.

Canonical channels:

- Stonebend / Body
- Flynt / Spirit
- Glaüshouse / Mind
- Sandmanor / Soul
- Minorian / Interior
- Minoan / Exterior

## Synthesis Grammar

Synthesis Recipe
→ Compiler
→ Synthesis Scripts
→ Aim
→ Fire
→ Miss | Kiss
→ Point²

Canonical staging labels:

Recipe compiled
→ Scripts ready
→ Aim prepared
→ Fire committed
→ Kiss
→ Scripts applied
→ Point² produced

## Scripts

V1 script vocabulary:

- ApplyPrismDelta
- AddFlow
- AddGlow
- SetFrame

Ordering remains deterministic.

## Aim / Fire

Aim is prepared synthesis.
Fire is pure contact calculation.

Manager locks remain canonical:

- HAL = META = PLEB ↔ META = Curved = Information From Beyond
- Clouseau = PLEB = PLEB ↔ PLEB = Straight = Bond
- Cleopatra = BLEP = PLEB ↔ BLEP = Inverted = Underworld Reflection

## Miss / Kiss

Miss:

- applies no scripts;
- preserves FrameState exactly;
- produces no changed Point².

Kiss:

- applies the full stored script sequence transactionally;
- commits all changes or none;
- lands the resulting FrameState at Point².

## Transactional Landing

V1 landing strategy remains:

1. Read immutable starting FrameState.
2. Clone one working FrameState.
3. Apply scripts to the working state in stored order.
4. Commit the finished working state as Point² only if every step succeeds.
5. Discard the working state completely on failure.

Rejected conditions:

- Prism underflow
- Prism overflow

Duplicate learning remains idempotent:

- AddFlow never creates duplicate Flow entries.
- AddGlow never creates duplicate Glow entries.

## Canonical Fixtures

### Aura Fixture

Hueman
→ Pixy Confusion Recipe
→ ApplyPrismDelta(Mind +2)
→ AddGlow(Confusion)
→ SetFrame(Pixy)
→ HAL Aim
→ Fire
→ Kiss
→ Pixy Point²

Result:

- Frame: Pixy
- Body: 1
- Spirit: 1
- Mind: 3
- Soul Interior: 1
- Soul Exterior: 1
- Flow: none
- Glow: Confusion

### Current Fixture

Hueman
→ Gremlin Tinker Recipe
→ ApplyPrismDelta(Body +2)
→ AddFlow(TinkerGrip)
→ SetFrame(Gremlin)
→ Clouseau Aim
→ Fire
→ Kiss
→ Gremlin Point²

Result:

- Frame: Gremlin
- Body: 3
- Spirit: 1
- Mind: 1
- Soul Interior: 1
- Soul Exterior: 1
- Flow: TinkerGrip
- Glow: none

## Public API Boundary

The public crate boundary remains centered on:

- kernel witness construction;
- snapshot/prompt/tree/desktop witness builders;
- FrameState / Prism / recipe / script / aim / fire / landing value types;
- manager-domain locks.

Refinement reductions made in this pass:

- removed the redundant public `AimId` wrapper;
- canonical aim fixture builders are crate-internal rather than public API.

## Snapshot / Consumer Boundary

Snapshot and consumer compatibility remain V1-stable.

Display-only witness additions remain in renderer/output code.
The core snapshot boundary remains deterministic and parseable by existing consumers and binaries in this workspace.

## Measured Baseline

Baseline before refinement:

- test count: `348`
- warm `cargo build`: `0.014s`
- warm `cargo build --release`: `0.868s`
- `client_desktop_status` direct release run average: `0.000353s`
- release benchmark average full route: `0.074 ms`
- release benchmark p95 full route: `0.080 ms`
- release benchmark `client_desktop_status` stage: `6.479 us`
- release benchmark drift count: `0`
- `target/release/hollow-grove`: `604840` bytes

## Measured Final State

After refinement:

- test count: `348`
- warm `cargo build`: `0.013s`
- warm `cargo build --release`: `0.013s`
- `client_desktop_status` direct release run average: `0.000383s`
- release benchmark average full route: `0.073 ms`
- release benchmark p95 full route: `0.095 ms`
- release benchmark `client_desktop_status` stage: `5.808 us`
- release benchmark drift count: `0`
- `target/release/hollow-grove`: `611848` bytes

Measured improvement retained:

- `client_desktop_status` benchmark stage improved from `6.479 us` to `5.808 us`
- improvement: about `10.4%`

Interpretation:

- the measured witness/render hotspot improved;
- total route runtime remained effectively flat to slightly better;
- whole-process direct binary timing remained noise-sensitive;
- binary size increased slightly, which is acceptable for the clarity and boundary cleanup retained here.

## Retained Internal Legacy Names

These remain valid internal/kernel implementation names:

- `GroveSeam`
- `HollowBeam`
- `LandedSymptom`

They remain internal implementation vocabulary.
They do not replace the canonical public witness:

Point
→ Triway
→ Fourway
→ HollowGrove
→ CurrentSeam
→ AuraBeam
→ Point² (Landed Point)

## Known V1 Limitations

- renderer and synthesis witness code are still verbose by design;
- canonical fixture data is still authored in code rather than data files;
- benchmark variance is still highest in downstream artifact stages rather than the kernel path;
- V1 still uses small owned `String` identifiers for recipe and aim provenance;
- no V2 decision, residue, turbulence, or accumulation systems exist yet.

## Freeze Statement

Current Synthesis (Hollow Grove) V1 is frozen at this boundary.

The canonical kernel witness, manager locks, synthesis grammar, Miss/Kiss behavior, transactional landing, Pixy fixture, and Gremlin fixture are all verified and must remain semantically stable until an explicit V2 change is authorized.
