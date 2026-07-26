# Flow Glow Embodied Gesture Grammar V1

This document records the locked Hollow Grove extension that makes embodied Flow and Glow actions enforceable through one shared typed grammar.

It extends the existing Being/Object ontology, civic-body correspondence, Proxy/Moxy/Foxy addressing, and Point² foundation. It does not replace the frozen Version 1.1 execution path.

## Purpose

The grammar answers five separate questions without collapsing them:

- `Being`: who is acting
- `Domain`: whether the action manifests as `Flow` or `Glow`
- `Gesture`: what the body physically does
- `Mode`: what relation the action acts through
- `Object`: what is being addressed
- `Aim`: what the action is trying to accomplish
- `AddressingMode`: how the Object relation is selected

Canonical resolution:

- `Being + Domain + Gesture + Mode + Object + AddressingMode + Aim -> Move`
- `Move -> Recipe -> V2 -> frozen V1.1`

## Domains

- `Flow = the Current/material expression of embodied action`
- `Glow = the Aura/psychic expression of embodied action`

Flow stays physical, structural, pressured, and materially consequential. Glow stays psychic, perceptual, expressive, relational, diagnostic, and meaningful.

## Modes

- `Seam = connection, joining, separation, opening, closing, stitching, and tearing`
- `Beam = direction, focus, aim, projection, transmission, and intervention`
- `Gleam = revealed condition, presence, recognition, finish, emotional effect, and visible proof`

## Gestures

- `Grip = controlled contact and stabilization`
- `Show = reveal, diagnose, present, stage, expose, and direct attention`
- `Grit = remain present, endure pressure, contain fear, and sustain action`

`EmbodiedGesture::Grip` is intentionally distinct from the older `SkillId::Grip`. The former is a universal input gesture. The latter remains a practiced Being/Object skill root inside the earlier ontology layer.

The full inherited Current Grip lineage that uses this distinction now lives in:

- `CURRENT_GRIP_INHERITANCE_V1.md`

## Canonical Pairings

- `Grip enters Seam.`
- `Show directs Beam.`
- `Grit sustains Gleam.`

These pairings carry the highest default affinity.

Valid cross-pairings remain possible when semantics justify them, for example:

- `Grip + Beam` for instrument stabilization
- `Show + Seam` for revealing where a relation is broken
- `Grit + Seam` for holding a collective continuity under pressure
- `Show + Gleam` for public recognition and staged consequence

## Stonebend Practice Projection

The canonical Stonebend practice projection is:

- `Structural practice channels Beam.`
- `Civic witness channels Seam.`
- `Identity custodian channels Gleam.`

These are specialization alignments inside the larger embodied grammar. They are not separate execution systems.

## Glaüshouse Clinical Use

Glaüshouse uses physical action psychically and clinically:

- `Grip the condition.`
- `Show the condition.`
- `Grit through the condition.`

Then:

- `Seam the relation.`
- `Beam the intervention.`
- `Gleam the meaning.`

Canonical examples:

- `Nightingale + Glow + Grip + Seam + Open Wound -> Aura Stitch`
- `Nightingale + Glow + Show + Beam + Hidden Infection -> Diagnostic Beam`
- `Nightingale + Glow + Show + Seam + Hidden Emotional Rupture -> Seam Diagnosis`

## Form and Frame Scaling

The grammar is shared across scale. Frame and form determine how large or subtle the result becomes.

Current examples:

- `Gremlin Flow Seam = fine physical splice`
- `Troglodyte Flow Beam = terrain-scale directed force`
- `Troglodyte Flow Gleam = visible proof of burden and consequence`

Aura examples:

- `Pixy Glow Seam = sensitive repair contact`
- `Siren Glow Gleam = commanding emotional presence`
- `Muse Glow Beam = meaning revelation at scene scale`

## Practice Hooks

Practice is recorded as validated action metadata, not idle time. The current typed hook records:

- role
- domain
- gesture
- mode
- object family
- addressing mode
- aim
- successful uses

This remains projected semantic state rather than a new persisted progression truth.

## Positive Fixtures

The enforced fixtures include:

- `Nightingale + Glow + Grip + Seam + Open Wound`
- `Surgeon + Flow + Grip + Seam + Tissue`
- `Nightingale + Glow + Show + Beam + Hidden Infection`
- `Muse + Glow + Show + Beam + Hidden Meaning`
- `Siren + Glow + Grit + Gleam + Frightened Crowd`
- `Structural practice + Flow + Grip + Beam + Cutting Tool`
- `Civic witness + Flow + Grit + Seam + Formation`
- `Identity custodian + Flow + Show + Gleam + Monument`
- `Nightingale + Glow + Show + Seam + Hidden Emotional Rupture`

## Negative Fixtures

The validator rejects:

- collapsed `Flow` and `Glow`
- `Seam`, `Beam`, or `Gleam` treated as fixed Moves
- gestures treated as Frames or species
- `Show` replaced with `Point`
- `Grip` collapsed into `SkillId::Grip`
- `Grit` reduced to stamina
- reversed Stonebend practice projection
- reversed canonical pairings
- `Flow`, `Glow`, `Beam`, `Seam`, or `Gleam` reduced to one narrow meaning
- `AddressingMode` replacing `ActionMode`
- direct execution outside `Recipe -> V2 -> V1.1`
- `Foxy = evil`
- `Moxy = velocity`
- idle-time mastery
- `Point³`
- `Position 13`
- automatic Aura Frame grant
- `CurrentPrism` conflation

## Enforcement

Primary enforcement:

- `src/flow_glow_grammar.rs`
- `src/being_object_ontology.rs`
- `src/hollow_grove_content.rs`
- `src/bin/current_synthesis_tui.rs`
- `src/main.rs`
- `tests/main_cli.rs`

Supporting semantic references:

- `HOLLOW_GROVE_SEMANTIC_FOUNDATION_V1.md`
- `BEING_OBJECT_ROOT_ONTOLOGY_V1.md`
- `PROXY_MOXY_FOXY_MANAGER_LANGUAGE_V1.md`
- `CIVIC_BODY_CORRESPONDENCE_V1.md`

## Compatibility

- no V1.1 change
- no Point³
- no Position 13
- no automatic Aura Frame grant
- no save migration required
- `CurrentPrism` remains distinct
- `Being`, `Object`, `Gesture`, `Mode`, `AddressingMode`, and `Aim` remain separate
