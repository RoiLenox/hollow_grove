# Rule Of Twelve House Grammar V1

This document records the locked Hollow Grove Rule of Twelve as enforced code.

It extends the existing semantic foundation, Point² progression model, and Ranina-centered map geometry. It does not replace the frozen Version 1.1 execution path.

The readable House-relative manager-language layer built over this geometry now lives in:

- `PROXY_MOXY_FOXY_MANAGER_LANGUAGE_V1.md`

## Core Law

Ranina is the unique, unnumbered center.

The Houses have a local order:

- `1 Stonebend`
- `2 Sandmanor`
- `3 Glaüshouse`
- `4 Flynt`

The local grammar repeats three times:

- `Pass 1 = Positions 1–4`
- `Pass 2 = Positions 5–8`
- `Pass 3 = Positions 9–12`

Therefore:

- `4 Houses × 3 Passes = 12 World Positions`

## Numbering Systems

- `House number` describes the local four-step grammar.
- `absolute position` describes angular location around Ranina.
- `pass` describes which repetition of the grammar is active.
- `ring` describes radial progression outward from Ranina.

These values are related but not interchangeable.

Examples:

- `Stonebend = House 1`, but its grammar recurs at `Positions 1, 5, 9`
- `Glaüshouse = House 3`, but its grammar recurs at `Positions 3, 7, 11`

Primary geographic anchors remain:

- `Stonebend primary anchor = Position 1`
- `Glaüshouse threshold = Position 6`
- `Glaüshouse primary pole = Position 7`

## Four-House Grammar

- `Stonebend = Diamond claims = establish, hold, embody, stabilize`
- `Sandmanor = Crystal measures = measure, compare, model, diagnose`
- `Glaüshouse = Jade clears = clear, repair, treat, restore`
- `Flynt = Opal shimmers = move, test, execute, transmit`

The reusable local cycle is:

- `Claim -> Measure -> Clear -> Shimmer`

## Position Table

| Absolute position | Pass | House number | House alignment | Special meaning |
| ---: | ---: | ---: | --- | --- |
| 1 | 1 | 1 | Stonebend | Primary Stonebend pole; Capricorn / Sea-Goat |
| 2 | 1 | 2 | Sandmanor | First measurement phase |
| 3 | 1 | 3 | Glaüshouse | First clearing phase |
| 4 | 1 | 4 | Flynt | First execution phase |
| 5 | 2 | 1 | Stonebend | Reinforcement / reclamation phase |
| 6 | 2 | 2 | Sandmanor | Glaüshouse threshold; diagnosis before descent |
| 7 | 2 | 3 | Glaüshouse | Primary Glaüshouse pole; Cancer / Crab; Abyss |
| 8 | 2 | 4 | Flynt | Movement after opposite-depth encounter |
| 9 | 3 | 1 | Stonebend | Integrated stabilization phase |
| 10 | 3 | 2 | Sandmanor | Integrated understanding phase |
| 11 | 3 | 3 | Glaüshouse | Deep clearing / integration phase |
| 12 | 3 | 4 | Flynt | Completed execution / release of rotation |

The descriptive phase language above is explanatory. The locked invariants are:

- absolute position
- pass
- House number
- House alignment
- primary-anchor status
- threshold status

## Position 6 and Position 7

- `Position 6 = Pass 2 = House Number 2 = Sandmanor`
- `Position 6 = measurement and diagnosis`
- `Position 6 = threshold into Glaüshouse`

- `Position 7 = Pass 2 = House Number 3 = Glaüshouse`
- `Position 7 = Cancer / Crab`
- `Position 7 = Abyss`
- `Position 7 = primary opposite pole to Stonebend Position 1`

The locked distinction is:

- `6 = diagnosis before descent`
- `7 = full treatment and depth pole`

## Primary Axis

The primary opposition remains:

- `Stonebend 1 ↔ Ranina ↔ Glaüshouse 7`

This keeps the Hollow Current / Abyss axis intact while the full four-House grammar repeats around it.

## Position 12, Wrap, and Spiral Ascension

- `Position 12 = Pass 3 = House Number 4 = Flynt`
- `Position 12` completes the angular rotation
- `next_position(12) = 1`

That wrap is angular only.

It does not:

- grant `Point²`
- raise capacity
- create `Position 13`
- create `Point³`

Canonical spiral interpretation:

- `Ring N, Position 12`
- legal `Point²` landing through frozen V1.1
- stabilization
- `Ring N+1, Position 1`

This is a valid canonical transition, not the only legal Point² route.

## Point and Point²

- `Point = Hueman + currently reachable world`
- `Point² = Current Capacity +1 + Aura Capacity +1 + expanded self + expanded surroundings`

The Rule of Twelve adds geometric interpretation:

- `position = angular state around Ranina`
- `ring = radial progression level`
- `Point²` may open the next ring without forcing angular motion

Ordinary wrap and Point² ascension remain separate:

- `Position 12 -> Position 1` can happen with no capacity gain
- `Point²` can happen with no automatic angular rotation

## Current Synthesis Integration

The Rule of Twelve is an outer grammar for observation, route meaning, threshold interpretation, and world-state explanation.

It does not replace the frozen kernel:

- `Point -> Triway -> Fourway -> HollowGrove -> CurrentSeam -> AuraBeam -> Point²`

Boundary split:

- semantic contract: meanings, House order, anchors, opposition
- map geometry: center, position, pass, ring, wrap, spiral transition
- progression: exactly-once capacity growth and stabilization
- V2: observation, generation, evaluation, choice
- V1.1: Recipe, Execute, legal Point² landing

## Enforced Source Locations

- Rule-of-Twelve types, derivations, validators, witnesses, and fixtures:
  `src/world_map_geometry.rs`
- Point progression and stabilized Point state:
  `src/point_progression.rs`
- optional V2 observation geometry:
  `src/decision_engine.rs`
- readable Proxy / Moxy / Foxy interpretation:
  `src/world_map_geometry.rs`
  `src/manager_domain.rs`
- runtime CLI:
  `src/bin/current_synthesis_tui.rs`
  `src/main.rs`
- foundation verification:
  `src/hollow_grove_content.rs`

## Test Coverage

- canonical position derivation and contradiction fixtures:
  `src/world_map_geometry.rs` tests
- progression persistence and angular/radial distinction:
  `src/point_progression.rs` tests
- V2 observation access:
  `src/decision_engine.rs` tests
- CLI witness and validation surfaces:
  `src/bin/current_synthesis_tui.rs` tests
  `tests/main_cli.rs`
- foundation verification:
  `src/hollow_grove_content.rs` tests
