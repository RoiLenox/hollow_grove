# Hollow Grove Semantic Foundation V1

This checkpoint records the semantic foundation that is now enforced in code.

It is a boundary document, not a second lore source. Canonical meaning lives in the contract, validators, fixtures, and freeze tests named below.

## Root Material Laws

- `Current = blood`
- `Hollow = pus`
- `Aura = air / pressure / light`
- `Whole -> Hollowing -> Hollow + Hollowed`

Primary enforcement:

- `src/hollow_grove_contract.rs`
- `tests/hollow_grove_alignment.rs`

## Formal Ontology

- `Frame = named living-mech form only`
- `Scene = active situation`
- `Structure = built or grown arrangement`
- `System = continuing operational relationship`

Frame exclusivity and alignment checks:

- `src/hollow_grove_contract.rs`
- `src/hollow_grove_content.rs`
- `src/current_synthesis_scenario.rs`
- `tests/hollow_grove_alignment.rs`

## Frame / Flow / Glow

- `Frame` stays attached only to legal living-mech forms.
- `Flow` remains Current-based physical capability.
- `Glow` remains Aura-based perceptual and expressive capability.

Runtime enforcement:

- `src/frame_state.rs`
- `src/point.rs`
- `src/landing.rs`
- `src/synthesis_execution.rs`

## Persistent Hueman Identity

- Player Being remains `Hueman`.
- Active transformations change `Frame` and legal capabilities without deleting the underlying Being.
- Learned legal `Flow` and `Glow` persist across frame switching unless a future explicit lock says otherwise.

Runtime and progression sources:

- `src/frame_state.rs`
- `src/landing.rs`
- `src/hueman_progression.rs`
- `src/current_synthesis_engine.rs`
- `src/hollow_grove_content.rs`

Regression coverage:

- `src/landing.rs` tests
- `src/hollow_grove_content.rs` tests

## Point² Current Depths Extension

The locked Point² / Current-depth / Aura-capacity extension now lives in:

- `POINT_SQUARED_CURRENT_DEPTHS_V1.md`

The locked Ranina / twelve-position / ring-expansion geometry extension now lives in:

- `RANINA_TWELVE_POSITION_MAP_V1.md`

The locked four-House repeated wheel grammar now lives in:

- `RULE_OF_TWELVE_HOUSE_GRAMMAR_V1.md`

The locked readable Proxy / Moxy / Foxy spatial-manager layer now lives in:

- `PROXY_MOXY_FOXY_MANAGER_LANGUAGE_V1.md`

The locked Being / Object root ontology now lives in:

- `BEING_OBJECT_ROOT_ONTOLOGY_V1.md`

Primary enforcement for that extension:

- `src/hollow_grove_contract.rs`
- `src/point_progression.rs`
- `src/being_object_ontology.rs`
- `src/hollow_grove_content.rs`
- `src/bin/current_synthesis_tui.rs`
- `tests/main_cli.rs`

## Progression Ladders

Current ladder:

- `Gremlin -> Goblin -> Ghoul -> Troll -> Ork -> Ogre -> Troglodyte`

Aura ladder:

- `Pixy -> Sprite -> Faerie -> Nymph -> Siren -> Muse`

Canonical note:

- `Troglodyte` and `Muse` remain matched apex counterparts.
- `Troglodyte + Muse = Seer` remains canonical world law and is not flattened into a single ladder.

Current runtime references:

- `src/frame_state.rs`
- `src/hueman_support.rs`
- `src/hollow_grove_content.rs`

## Hollow / Hollowing / Hollowed

- `Hollow` is the extracted interior resource materially represented by pus.
- `Hollowing` extracts the useful interior while leaving the outer form.
- `Hollowed` is the preserved remainder after extraction.

Contract and fixtures:

- `src/hollow_grove_contract.rs`
- `tests/hollow_grove_alignment.rs`

## House and Rock Semantics

- `Stonebend -> Diamond -> claims`
- `Sandmanor -> Crystal -> measures`
- `Glaushouse -> Jade -> clears`
- `Flynt -> Opal -> shimmers`

Enforcement and generated artifacts:

- `src/hollow_grove_contract.rs`
- `src/hollow_grove_content.rs`
- `src/hueman_support.rs`

## Nightingales

- Nightingales directly represent white blood cells.
- Origin: Stonebend marrow.
- Medium: Current.
- They are not a generic staff class or generic nurse species.

Enforcement:

- `src/hollow_grove_contract.rs`
- `src/hollow_grove_content.rs`
- `src/current_synthesis_scenario.rs`
- `tests/hollow_grove_alignment.rs`

## Connected Civilization and Composition-Based Professions

Canonical social rule:

- `Species gives tendencies.`
- `House training gives discipline.`
- `Profession gives social function.`
- `The individual determines mastery.`

Validated composition builders:

- `src/hollow_grove_content.rs`
- `src/hueman_support.rs`

Covered examples:

- `Gargoyle Surgeon`
- `Elf Radiologist`
- `Werewolf Emergency Nurse`
- `Gnome Emergency Physician`

## Scenario and Content Validation Boundaries

Content enters runtime through:

- raw content
- parse
- semantic alignment validation
- domain construction
- runtime use

Current boundary hooks:

- `src/current_synthesis_scenario.rs`
- `src/hollow_grove_content.rs`

Regression coverage:

- `tests/hollow_grove_alignment.rs`
- `tests/main_cli.rs`

## Vertical Witness and Regression Command

Primary runtime checkpoints:

- `hollow-grove world witness`
- `hollow-grove world validate`
- `hollow-grove verify-foundation`

The full foundation witness and regression summary are built in:

- `src/hollow_grove_content.rs`
- `src/main.rs`

## Frozen V1.1 Boundary

The Version 1.1 execution topology remains unchanged:

- `Observe -> Generate -> Evaluate -> Choose -> Recipe -> Execute -> Point²`

The semantic contract validates the handoff into this path. It does not rewrite it.

Freeze and witness sources:

- `src/decision_engine.rs`
- `src/synthesis_execution.rs`
- `src/kernel_pass.rs`
- `src/kernel_pass_output.rs`
- `tests/kernel_routing_slice_freeze.rs`

## Checkpoint Rule

Future world additions should extend content and builders only after passing the contract, fixture, contradiction, and vertical witness surfaces already named here.
