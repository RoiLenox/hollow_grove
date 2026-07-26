# Current Grip Inheritance V1

This document records the locked Hollow Grove Current Grip inheritance layer enforced in code.

It extends the existing Being/Object ontology and Flow/Glow embodied grammar. It does not replace the frozen Version 1.1 execution path.

## Purpose

This pass proves one complete inherited Current Skill family from Gremlin through Troglodyte.

It locks that:

- `SkillRoot::Grip` survives transformation
- lower expressions remain available
- Object-family familiarity matters
- Gesture, Mode, AddressingMode, and Aim remain distinct inputs
- Move resolution still passes through `Move -> Recipe -> V2 -> frozen V1.1`

## Critical Distinction

- `SkillRoot::Grip = the practiced inherited Current Skill root`
- `EmbodiedGesture::Grip = the physical gesture of controlled contact`

They are related but not identical.

The implementation keeps them separate:

- `SkillId::Grip` is the concrete code representation of the canonical `SkillRoot::Grip`
- `EmbodiedGesture::Grip` remains part of the Flow/Glow grammar

## Current Form Lineage

- `Gremlin = precise Current`
- `Goblin = pressured Current`
- `Ghoul = persistent Current`
- `Troll = structural Current`
- `Ork = collective Current`
- `Ogre = massive Current`
- `Troglodyte = world-bearing Current`

## Grip Expression Lineage

- `Gremlin -> TinkerGrip`
- `Goblin -> WeaponGrip`
- `Ghoul -> CarrionGrip`
- `Troll -> BridgeGrip`
- `Ork -> FormationGrip`
- `Ogre -> SiegeGrip`
- `Troglodyte -> WorldGrip`

The predecessor chain is cumulative:

- `TinkerGrip -> none`
- `WeaponGrip -> TinkerGrip`
- `CarrionGrip -> WeaponGrip`
- `BridgeGrip -> CarrionGrip`
- `FormationGrip -> BridgeGrip`
- `SiegeGrip -> FormationGrip`
- `WorldGrip -> SiegeGrip`

Higher forms keep the lower expressions when context supports them.

Canonical doctrine:

- `Troglodyte retains Gremlin precision.`

## Object Resolution

Grip inheritance resolves from structured Object state, not names alone.

The current implementation inspects:

- `ObjectId`
- `ObjectFamily`
- `ObjectScale`
- `ObjectCondition`
- `ObjectMaterial`
- `ObjectFunction`
- `ObjectConnection`
- practice familiarity

Move choice is not determined by scale alone or by object name alone.

## Flow / Glow Grammar Integration

Current Grip inheritance is a `Flow`-domain skill family, but it resolves through the newer embodied grammar:

- `SkillRoot::Grip + EmbodiedGesture::Grip + ActionMode::Seam`
- `SkillRoot::Grip + EmbodiedGesture::Grip + ActionMode::Beam`
- `SkillRoot::Grip + EmbodiedGesture::Grit + ActionMode::Seam`
- `SkillRoot::Grip + EmbodiedGesture::Show + ActionMode::Beam`

Canonical high-value uses include:

- `Grip + Seam = controlled contact acting on continuity`
- `Grip + Beam = controlled contact directing a line of force`
- `Grit + Seam = holding a collective or structural relation under pressure`
- `Show + Beam = revealing or establishing the line through which Grip acts`

Valid cross-pairings remain possible when the Object, Aim, and context justify them.

## Addressing Integration

- `Proxy = grip the immediate/local Object`
- `Moxy = grip what the local Object connects toward`
- `Foxy = grip the inverse, hidden, underside, return, or counter-relation`

Canonical examples:

- `Troll + Moxy + Near Bridge Support -> BridgeGrip toward the far anchor`
- `Gremlin + Foxy + Reverse-facing Hidden Latch -> TinkerGrip candidate on the hidden mechanism`

## Practice History

Practice comes only from meaningful validated action.

It records:

- form history
- expression history
- object-family familiarity
- material familiarity
- scale familiarity
- gesture history
- mode history
- pressure history

Idle time does not grant mastery.

Invalid or rejected actions do not grant full practice.

## Specialization Tendency Hooks

This pass exposes projected tendencies only.

- `Structural practice` tendency tracks Beam-heavy tool, weapon, machine, and directed-craft history
- `Civic witness` tendency tracks Seam-heavy bridge, formation, labor, and collective continuity history
- `Identity custodian` tendency tracks Gleam-heavy terrain, monument, burden, and visible-proof history

No permanent class selection is created here.

## Positive Fixtures

The enforced fixtures include:

- `Gremlin + Mechanical Latch -> TinkerGrip`
- `Goblin + Weapon -> WeaponGrip`
- `Ghoul + Damaged Wreckage -> CarrionGrip`
- `Troll + Broken Crossing Support -> BridgeGrip`
- `Ork + Shield Formation Anchor -> FormationGrip`
- `Ogre + Siege Engine -> SiegeGrip`
- `Troglodyte + Fractured Cliff -> WorldGrip`
- `Troglodyte + Mechanical Latch -> retained TinkerGrip`
- `Troll + Moxy + Near Bridge Support -> BridgeGrip toward connected far anchor`
- `Gremlin + Foxy + Reverse-facing Hidden Latch -> TinkerGrip candidate`
- projected `Structural practice`, `Civic witness`, and `Identity custodian` tendencies

## Negative Fixtures

The validator rejects:

- collapsing `SkillRoot::Grip` into `EmbodiedGesture::Grip`
- seven unrelated Grip skills
- higher forms losing lower expressions
- Troglodyte always selecting `WorldGrip`
- Gremlin selecting `WorldGrip`
- object-name-only selection
- scale-only selection
- `Moxy` as velocity
- `Foxy` as evil or automatic time reversal
- idle-time mastery
- direct execution outside `Recipe -> V2 -> frozen V1.1`
- direct `CurrentPrism` or capacity mutation
- automatic Aura Frame grant
- `Point³`
- `Position 13`

## Commands

The pass adds:

- `hollow-grove current-inheritance witness`
- `hollow-grove current-inheritance validate`
- `hollow-grove grip witness`

These are wired through the top-level launcher and the current-synthesis TUI command router.

## Boundary

Grip inheritance remains outside the frozen kernel.

Resolution path:

- `Being + Domain + Gesture + Mode + Skill + Object + AddressingMode + Aim -> Move`
- `Move -> Recipe -> V2 -> frozen V1.1`

`LandingOutcome::Kiss` remains the legal Point² landing.

`Point²` still grants:

- `Current Capacity +1`
- `Aura Capacity +1`

exactly once.

## Enforcing Sources

- `src/current_grip_inheritance.rs`
- `src/being_object_ontology.rs`
- `src/bin/current_synthesis_tui.rs`
- `src/main.rs`
- `src/hollow_grove_content.rs`
- `tests/main_cli.rs`

## Compatibility

- Being and Object remain separate
- Skill and Move remain separate
- Flow and Glow remain separate
- `Structural practice = Beam`
- `Civic witness = Seam`
- `Identity custodian = Gleam`
- `CurrentPrism` remains distinct
- no save migration was required
