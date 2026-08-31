# Civic Body Correspondence V1

This document records the active civic-body correspondence and its legacy
compatibility boundary as enforced by code.

It lives above the frozen Version 1.1 execution path. It does not literalize Hollow Grove into biology. It defines how the civic peoples function as if Hollow Grove were a living body.

The inherited Current Grip skill family that now integrates with civic response roles through the shared Flow/Glow grammar lives in:

- `CURRENT_GRIP_INHERITANCE_V1.md`

## Purpose

The model answers:

- what each people contributes to Hollow Grove survival
- how civic response chains compose during breach, injury, shortage, and repair
- how civic identity informs Being/Object actions without replacing Being, Skill, Object, Move, AddressingMode, Aim, Recipe legality, V2, or V1.1

Canonical distinction:

- `body correspondence = functional and civic`
- `body correspondence != literal species definition`

## Canonical Mapping

- `Geralds = Stonebend = red blood cells = carry`
- `Nightingales = Glaüshouse = white blood cells = clear`
- `Minorians = Sandmanor Interior = interior signaling and regulation = measure`
- `Minoans = Sandmanor Exterior = epithelial and sensory boundary = reveal`
- `Wardens = Flynt civic identity`
- `Flynt platelet correspondence = STATUS: OPEN`

Canonical chant:

- `Geralds carry.`
- `Nightingales clear.`
- `Minorians measure.`
- `Minoans reveal.`

The broader House civic identities are `Wardens = Flynt`,
`Geralds = Stonebend`, and `Sandmen = Sandmanor`. Sandmen is the broader
Sandmanor civic identity; Minorians and Minoans remain its Interior and
Exterior civic-body faces in this correspondence model.

## Geralds

Geralds are Stonebend’s civic circulation people.

They correspond to red blood cells by function:

- carry Current
- transport fuel, material, tools, labor, and recovery loads
- sustain distant structures
- keep physical continuity alive across the Grove

Canonical note:

- Geralds are systemic circulation, not generic couriers.

## Nightingales

The Nightingales are Glaüshouse’s constitutional nursing and clinical-care
institution as well as its civic recognition and clearing body.

Their white-blood-cell correspondence describes what the institution protects
and how it responds; it does not prevent Nightingales from serving as nurses,
advocates, witnesses, emergency responders, or operators within lawful scope,
and it does not make generic nurses a species.

They correspond to white blood cells by function:

- recognize danger and injury
- distinguish harmful from healthy conditions
- clear damaged matter
- coordinate healing
- remember threats

Canonical note:

- Nightingales clear.
- They are not generic healers.
- They are not the legacy Flynt Wardens compatibility class.

## Flynt civic correspondence

The former active assignment `Wardens = Flynt = platelets = close` is
**SUPERSEDED**. `Wardens = Flynt` is restored only as the canonical civic
identity. Flynt's final platelet or other civic-body correspondence remains
**STATUS: OPEN** and is not inferred from the shared name.

Historical `CivicPeople::Wardens`, `WARDENS`, chant, and crisis-step records are
**LEGACY / DEPRECATED** compatibility data only when they encode the retired
platelet/close mapping. They may be decoded or reproduced for old archives and
replays without displacing the active Warden civic identity.

## Minorians

Minorians are Sandmanor's Interior people. Gnome is their base Form, not a name
for the entire population.

They correspond to interior signaling and regulatory cells by function:

- measure internal conditions
- count and allocate resources
- route messages
- maintain hidden maps and timing
- coordinate internal response

Canonical note:

- Minorians measure.
- Gnomes retain horizontal occupational specializations. That horizontal civic
  structure does not erase the distinct `Gnome → Faun → Minotaur` Form
  progression or the Husbandman → Householder → Hearthkeeper social line.

## Minoans

Minoans are Sandmanor's Exterior people. Elf is their base Form, not a name for
the entire population.

They correspond to epithelial and sensory boundary cells by function:

- define and preserve the boundary
- regulate exchange
- sense the outside world
- present outward condition
- reveal environmental change inward

Canonical note:

- Minoans reveal.
- They are not Minorians.

## Crisis Loop

The active civic breach-response witness records the unresolved Flynt step:

1. `Minoans reveal the breach.`
2. `Minorians measure the breach.`
3. `Flynt breach-closing civic designation: STATUS OPEN.`
4. `Nightingales clear the threat.`
5. `Geralds carry what recovery requires.`

This is a semantic response grammar, not a frozen kernel route. Responses may overlap or run in parallel.

## Failure States

- `Geralds absent -> The body starves.`
- `Nightingales absent -> The body becomes infected.`
- `Flynt breach-closing civic function unresolved -> STATUS: OPEN.`
- `Minorians absent -> The body loses internal coordination.`
- `Minoans absent -> The body loses its boundary.`

## Being Object Integration

Civic role belongs to the Being. The response acts on or through an Object.

Examples:

- `Gerald + Carry + Supply Object -> delivery interpretation`
- `Nightingale + Diagnose/Clear + Injury Object -> recognition and clearing interpretation`
- `Flynt OPEN civic role + Brace/Close + Breach Object -> containment interpretation`
- `Minorian + Measure + System Object -> internal regulation interpretation`
- `Minoan + Reveal/Sense + Boundary Object -> exterior perception interpretation`

The civic layer influences interpretation and candidate meaning. It does not replace the Being/Object root ontology.

## Proxy Moxy Foxy

AddressingMode remains separate:

- `Proxy = immediate/local`
- `Moxy = relation through what the Object connects toward`
- `Foxy = inverse, reflected, hidden, or return relation`

Civic examples:

- `Gerald + Moxy` can supply a connected district through a route.
- `Nightingale + Foxy` can reveal a hidden or mistaken threat relation.
- the OPEN Flynt civic role may eventually use Foxy to stabilize an underside or return leak.
- `Minorian + Moxy` can measure what a system routes beyond itself.
- `Minoan + Foxy` can reveal a concealed boundary condition.

## House Materials

The civic-body lock reinforces existing House materials without replacing them:

- `Diamond claims`
- `Crystal measures`
- `Jade clears`
- `Opal shimmers`

## Stonebend Hand Triad

The Stonebend apex mapping now uses the embodied action grammar documented in:

- `FLOW_GLOW_EMBODIED_GESTURE_GRAMMAR_V1.md`

Canonical specialization alignment:

- `Structural practice = Beam`
- `Civic witness = Seam`
- `Identity custodian = Gleam`

Gerald circulation supplies all three powers rather than replacing them.

## Enforcement

Primary enforcement:

- `src/civic_body.rs`
- `src/hollow_grove_contract.rs`
- `src/hollow_grove_content.rs`
- `src/bin/current_synthesis_tui.rs`
- `src/main.rs`
- `tests/main_cli.rs`

Supporting semantic sources:

- `HOLLOW_GROVE_SEMANTIC_FOUNDATION_V1.md`
- `BEING_OBJECT_ROOT_ONTOLOGY_V1.md`
- `PROXY_MOXY_FOXY_MANAGER_LANGUAGE_V1.md`
- `RULE_OF_TWELVE_HOUSE_GRAMMAR_V1.md`

## Compatibility

- no new kernel topology
- no new persisted progression truth
- no save migration required
- legacy `CivicPeople::Wardens` decoding remains stable and deprecated
- active registries omit Flynt Wardens until the OPEN civic designation is resolved
- `CurrentPrism` remains distinct
- `Point²` legality remains unchanged
- `V2 -> Recipe -> V1.1` remains unchanged
