# FIRST SYNTHESIS v0.2.0

This milestone locks the first end-to-end Hollow Grove synthesis path without changing the canonical kernel topology.

## Canonical Kernel Route

The kernel witness remains:

Point
→ Triway
→ Fourway
→ HollowGrove
→ CurrentSeam
→ AuraBeam
→ Point² (Landed Point)

Point² is the single landed state.
Point² becomes the next pass's Point through the existing landing behavior.

## Frame / Flow / Glow

Frame is the carried gameplay identity.
Flow is the carried Current learnset.
Glow is the carried Aura learnset.

The kernel carries FrameState deterministically and does not interpret Frame, Flow, or Glow beyond their opaque identifiers.

## Current Prism 1:1

The Current Prism is the attribute map.
One stored Prism value equals one gameplay attribute unit.

Canonical channels:

- Stonebend / Body
- Flynt / Spirit
- Glaüshouse / Mind
- Sandmanor / Soul
- Minorian / Interior Soul
- Minoan / Exterior Soul

## Recipe → Scripts

Recipes are authored intent.
Scripts are compiled engine instructions.

Initial script vocabulary:

- ApplyPrismDelta
- AddFlow
- AddGlow
- SetFrame

Canonical sequence:

Recipe compiled
→ Scripts ready

## Aim → Fire

Aim is prepared synthesis.
Fire commits the contact attempt.

Canonical sequence:

Recipe compiled
→ Scripts ready
→ Aim prepared
→ Fire committed

## Miss | Kiss

Fire produces exactly one contact outcome:

- Miss
- Kiss

Miss leaves the starting FrameState unchanged and produces no changed Point².

## Transactional Kiss Landing

Kiss applies the ordered Script sequence transactionally.

Guarantees:

- every Script succeeds, or no changes are committed;
- Script order is preserved exactly;
- failed application rolls back completely;
- the original starting FrameState remains unchanged;
- the resulting FrameState lands at Point².

Canonical sequence:

Recipe compiled
→ Scripts ready
→ Aim prepared
→ Fire committed
→ Kiss
→ Scripts applied
→ Point² produced

## Hueman → Pixy Fixture

Starting FrameState:

- Frame: Hueman
- Body: 1
- Spirit: 1
- Mind: 1
- Soul Interior: 1
- Soul Exterior: 1
- Flow: none
- Glow: none

Canonical compiled Script order:

1. ApplyPrismDelta(Mind +2)
2. AddGlow(Confusion)
3. SetFrame(Pixy)

Canonical Kiss result at Point²:

- Frame: Pixy
- Body: 1
- Spirit: 1
- Mind: 3
- Soul Interior: 1
- Soul Exterior: 1
- Flow: none
- Glow: Confusion

## Rollback Guarantees

Prism underflow rejects transactionally.
Prism overflow rejects transactionally.
Duplicate AddFlow is deterministic and idempotent.
Duplicate AddGlow is deterministic and idempotent.
SetFrame preserves unrelated FrameState fields.

## Verification Snapshot

Snapshot record:

- type: repository milestone snapshot
- date: 2026-07-14
- verification: `cargo fmt --check`, `cargo test`, `cargo build`

Current test count:

- 339 Rust tests in the verified workspace suite

Milestone scope:

- canonical kernel route unchanged
- Frame / Flow / Glow carried through FrameState
- Current Prism 1:1
- Recipe compilation
- Aim preparation
- Fire contact
- Miss / Kiss outcome
- transactional Kiss landing
- next-pass Point² carryover
