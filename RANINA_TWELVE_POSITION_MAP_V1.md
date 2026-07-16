# Ranina Twelve-Position Map V1

This document records the locked Hollow Grove geometry extension that makes `Ranina`, the twelve-position rotation, the `1 ↔ 7` axis, and Point² ring expansion enforceable in code.

It extends the existing semantic foundation and Point² progression model. It does not replace the frozen Version 1.1 execution path, the House map, or the route contract.

## Core Law

Ranina is the unmoving center.
Twelve positions complete one world rotation.
Stonebend is Position 1.
Glaüshouse is the opposite pole at Position 7.
Position 6 is the threshold into Glaüshouse.
Point² opens the next ring around the same center.

## Center

- `Ranina = exact center = unnumbered origin`
- `Ranina = frog / frog-crab / living transformation hinge`
- `Ranina` is not a fifth House
- `Ranina` is the fixed center from which ring, direction, distance, and opposition are measured

Current coordinate overlays may still place `Aura Ridge Junction` and `Human Core` at the same visible center anchor, but they now do so by sharing `Ranina` rather than replacing it.

## Twelve Positions

- full rotation: `12 positions`
- full rotation: `360 degrees`
- degrees per position: `30`
- wraparound: `next(12) = 1`
- wraparound: `previous(1) = 12`
- opposition: `opposite(1) = 7`

The twelve-position type is stored as a validated integer `1..=12`. `Ranina` is not Position 0 inside that set.

## Anchors

- `Position 1 = Stonebend = Capricorn = Sea-Goat = Hollow Current = life held in form`
- `Position 6 = Glaüshouse threshold = descent from visible condition toward Abyss`
- `Position 7 = Glaüshouse = Cancer = Crab = Abyss = life felt in depth`

The primary axis is:

- `Stonebend 1 ↔ Ranina ↔ Glaüshouse 7`

That is the canonical Hollow Current / Abyss opposition.

## Position Versus Ring

- `position = angular location around Ranina`
- `ring = radial progression outward from Ranina`

These are separate dimensions. Moving from one position to another does not grant capacity. Point² expands the ring and does not rotate the Point automatically.

## Point and Point²

- `Point = Hueman + currently reachable ring and world`
- `Point² = Current Capacity +1 + Aura Capacity +1 + next ring opened + Ranina unchanged`

Canonical geometry consequence:

- `Point level 1 / ring 1`
- legal landed `Point²`
- `Point level 2 / ring 2`
- same center
- same position unless movement occurs

## Stairway to Heaven

The first major world proof remains the `Stairway to Heaven` toward `Stonebend`.

Canonical fixture:

- before Point²: ring `1`, position `7`, Stairway not yet fully visible or not yet survivable
- after Point²: ring `2`, position `7`, Stairway visible and survivable
- destination remains `Stonebend / Position 1`
- next Frame becomes possible, not automatically granted

## Source Enforcement

- geometry types and witness builders:
  `src/world_map_geometry.rs`
- Point and ReachableWorld state:
  `src/point.rs`
  `src/point_progression.rs`
- semantic contract integration:
  `src/hollow_grove_contract.rs`
- foundation verification:
  `src/hollow_grove_content.rs`
- runtime-facing CLI:
  `src/bin/current_synthesis_tui.rs`
  `src/main.rs`

## Test Enforcement

- rotation law, opposition, threshold, center invariance, and contradiction fixtures:
  `src/world_map_geometry.rs` tests
- Point² ring preservation and progression persistence:
  `src/point_progression.rs` tests
- CLI witnesses:
  `src/bin/current_synthesis_tui.rs` tests
  `tests/main_cli.rs`
- foundation summary:
  `src/hollow_grove_content.rs` tests
