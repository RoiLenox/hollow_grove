# Proxy Moxy Foxy Manager Language V1

This document records the locked readable spatial-manager layer added over the existing Rule-of-Twelve world model.

It extends the existing semantic foundation, Ranina-centered geometry, and Point² progression model. It does not replace the frozen Version 1.1 execution path, world coordinates, or progression state.

The Being/Object action ontology that consumes these addressing modes now lives in:

- `BEING_OBJECT_ROOT_ONTOLOGY_V1.md`

The civic-body layer that applies these addressing modes to Gerald, Nightingale, Warden, Minorian, and Minoan response roles now lives in:

- `CIVIC_BODY_CORRESPONDENCE_V1.md`

The embodied Flow / Glow action grammar that uses Proxy, Moxy, and Foxy as addressing modes now lives in:

- `FLOW_GLOW_EMBODIED_GESTURE_GRAMMAR_V1.md`

## Checkpoint

This pass starts from the verified Rule-of-Twelve checkpoint:

- `Ranina = unique unnumbered center`
- `world coordinate = ring + absolute position`
- `Position 6 = Sandmanor = Glaüshouse threshold`
- `Position 7 = Glaüshouse = primary opposite pole`
- `Position 12 = Flynt = rotational completion`
- `ordinary Position 12 -> Position 1 wrap does not grant Point²`
- `legal Point² still applies Current Capacity +1 and Aura Capacity +1 exactly once`

## Canonical Mapping

- `PLEB = Proxy`
- `META = Moxy`
- `BLEP = Foxy`

- `Clouseau = Proxy`
- `HAL = Moxy`
- `Cleopatra = Foxy`

Canonical doctrine:

- `Proxy places.`
- `Moxy bonds.`
- `Foxy inverts.`

Manager responsibility lock:

- `Clouseau locates.`
- `HAL connects.`
- `Cleopatra reflects.`

## Layers

- `world coordinate = Ring + Absolute Position`
- `Proxy = House-relative spatial interpretation`
- `Moxy = relation or bond opened from the Proxy`
- `Foxy = reflected or inverted Proxy or Moxy`

Within the Being/Object root ontology these also remain:

- `Proxy = immediate/local Being-Object relation`
- `Moxy = what the addressed Object connects toward`
- `Foxy = the inverse, reflected, hidden, or return Object relation`

These layers coexist. They do not replace one another.

## Proxy

Proxy answers:

- `Where is the player?`

Locked Proxy aspects:

- `anchor`
- `direction`
- `geometry`
- `proximity`

Optional contextual fields:

- `route`
- `coordinate`
- derived `pass`
- derived `House alignment`
- derived `threshold`
- derived `rotation completion`

Canonical readable order:

- `Proximity Geometry direction of House`

Example:

- `Distal Round northwest of Stonebend`

Definitions:

- `Proximal = nearer to the selected Proxy anchor`
- `Distal = farther from the selected Proxy anchor`

Proxy is not Flat-only. A player can have Proxy on:

- `Flat`
- `Round`
- `Inverted`

## Absolute Alignment Versus Proxy Anchor

Absolute Rule-of-Twelve alignment and Proxy anchor are related but not identical.

Canonical fixture:

- `World Coordinate = Ring 2 / Position 12`
- derived `Pass 3 / House Number 4 / Flynt / rotation complete`
- `Proxy = Distal Round northwest of Stonebend`

This is valid because:

- Rule-of-Twelve alignment describes absolute rotational grammar
- Proxy describes readable House-relative local address

## Moxy

Moxy answers:

- `What does this place connect toward?`

Locked meaning:

- `bond`
- `relation`
- `destination`
- `information from beyond`

Canonical example:

- `Bond toward Flynt through Stairway to Heaven`

Moxy may use:

- route
- destination House
- pass boundary
- opposition axis
- rotational completion
- spiral-transition context

But Moxy does not:

- replace movement
- force movement
- grant Point²
- override V2 legality
- bypass frozen V1.1 execution

## Foxy

Foxy answers:

- `What is the reflected expression?`

Locked meaning:

- `reflection`
- `inversion`
- `return`
- `underworld relation`

Foxy can reflect:

- `Proxy`
- `Moxy`

Canonical example:

- `Inverted reflection of the Stonebend-Flynt bond`

Foxy does not automatically mean:

- evil
- corruption
- failure
- invalidity

## Rule-of-Twelve Compatibility

The Rule-of-Twelve remains authoritative for:

- `absolute position`
- `pass`
- `House number`
- `House alignment`
- `threshold state`
- `rotation completion`

Proxy, Moxy, and Foxy cannot overwrite those derived facts.

Canonical example:

- `Position 6 = Pass 2 = House Number 2 = Sandmanor = Glaüshouse threshold`

A local Proxy may describe the player’s route-relative address there, but it cannot change that Rule-of-Twelve derivation.

## V2 Observation Boundary

Optional world-aware `DecisionObservation` now exposes derived spatial interpretation when available:

- `rotation_context`
- `spatial_interpretation`

This enriches traces and route-aware explanation without changing:

- candidate legality
- recipe legality
- V1.1 landing
- Point² application rules

## V1.1 Boundary

Frozen execution topology remains:

- `Point -> Triway -> Fourway -> HollowGrove -> CurrentSeam -> AuraBeam -> Point²`

Proxy, Moxy, and Foxy live outside that kernel in:

- world geometry
- semantic validation
- witnesses
- V2 observation context
- CLI inspection

## Enforced Source Locations

- manager-language mapping and validation:
  `src/manager_domain.rs`
- Proxy / Moxy / Foxy types, derivation, fixtures, and validation:
  `src/world_map_geometry.rs`
- optional V2 observation context:
  `src/decision_engine.rs`
- foundation verification:
  `src/hollow_grove_content.rs`
- CLI surfaces:
  `src/bin/current_synthesis_tui.rs`
  `src/main.rs`

## Test Coverage

- manager-language contract fixtures:
  `src/manager_domain.rs` tests
- Proxy / Moxy / Foxy positive and negative fixtures:
  `src/world_map_geometry.rs` tests
- optional observation exposure:
  `src/decision_engine.rs` tests
- CLI witnesses and validation:
  `src/bin/current_synthesis_tui.rs` tests
  `tests/main_cli.rs`
- foundation summary:
  `src/hollow_grove_content.rs` tests
