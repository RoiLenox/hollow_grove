# Hueman Current Body / Aura Mind Progression Matrix V1

**Status:** LEGACY / DEPRECATED physical-runtime compatibility matrix
**Purpose:** connect lore, skill trees, Recipes, backend capability records, and art states.

This V1 matrix does not define the active Practitioner ontology or active
House progressions. Its serialized stages and power-root fields remain
decodable for replay and pilot compatibility. Current canon is governed by
`HOLLOW_GROVE_SYNTHESIS_CANON_V1.md`.

## Two-axis model

Hueman progression has two independent but combinable axes:

- **Current Body:** physical force, continuity, movement, transformation, and
  material endurance.
- **Aura Mind:** perception, attention, memory, relation, intention, and
  probable consequence.

A high value on one axis does not automatically raise the other. Synthesis may
couple them, but Relativity determines the cost and return.

## Current Body ladder

| Stage | Form | Canonical faculty/root | Primary capacity | Typical distortion |
|---|---|---|---|---|
| C1 | Gremlin | Presynce / Somatokinesis | fine tool and mechanism control | twitch, overfocus, tool feedback |
| C2 | Goblin | Presynce / Somatokinesis | redirect embodied motion and momentum | recoil, loss of balance |
| C3 | Ghoul | Presynce / Somatokinesis | bodily force, recovery, tissue response | pain transfer, unstable repair |
| C4 | Spectre | Presynce / Somatokinesis | reduce contact and pass embodied force through gaps | desynchronization, weak anchoring |
| C5 | Troll | Presynce / Somatokinesis | carry and resist heavy load | rigidity, Current exhaustion |
| C6 | Ork | Presynce / Somatokinesis | direct force through strikes and launches | impact debt, uncontrolled discharge |
| C7 | Ogre | Presynce / Somatokinesis | sustain heavy pressure across the body's immediate field | compression, environmental damage |
| C8 | Troglodyte | Presynce / Somatokinesis | couple to deep structure, ground, and large-scale load | ossification, slow return |

The ladder changes Form capacity, load, and distortion within the existing
`Presynce / Somatokinesis` root. Stage names do not create Mechanokinesis,
Kinetokinesis, Phasokinesis, Massokinesis, Ballistokinesis, Gravikinesis, or
Geokinesis as parallel runtime roots.

The Gargoyle branch begins from a Gremlin-compatible Current state and is a
specialized Synthesis path rather than an extra ladder:

`Gremlin → Gargoyle Synthesis → Presence I → Presence II → Presence III → Winged Gargoyle`

Gargoyle Telekinesis is the separate Flynt `Resynce / Telekinesis` capability
for precise external force at a distance. Other Current forms remain within
their embodied root and do not inherit Gargoyle Presence automatically.

## Aura Mind ladder

| Stage | Form | Canonical faculty/root | Primary capacity | Typical distortion |
|---|---|---|---|---|
| A1 | Pixie | Precog / Clairvoyance | detect attention, feeling, and field pressure | sensory overload |
| A2 | Sprite | Precog / Clairvoyance | read emotional synchronization and calming possibilities | emotional bleed |
| A3 | Fairy | Precog / Clairvoyance | read shifts in attention and interpretation | distraction rebound |
| A4 | Spirit | Precog / Clairvoyance | read continuity across embodied and relational state | desynchronization burden |
| A5 | Siren | Precog / Clairvoyance | read group feeling, rhythm, and focus | group feedback |
| A6 | Sylph | Precog / Clairvoyance | read movement, distance, and changing field relation | drift, weak anchoring |
| A7 | Nymph | Precog / Clairvoyance | read memory association and recognition pressure | identity confusion |
| A8 | Muse | Precog / Clairvoyance | perceive branching intention and probable consequence | future fixation, uncertainty burden |

`Pixy` and `Faerie` remain stable compatibility aliases for `Pixie` and
`Fairy`.

## Exact level pairing

| Level | Current Body | Aura Mind |
|---:|---|---|
| 1 | Gremlin | Pixie |
| 2 | Goblin | Sprite |
| 3 | Ghoul | Fairy |
| 4 | Spectre | Spirit |
| 5 | Troll | Siren |
| 6 | Ork | Sylph |
| 7 | Ogre | Nymph |
| 8 | Troglodyte | Muse |

The table is the canonical one-to-one crosswalk; neither ladder may be shifted
or compressed when deriving a counterpart.

The Aura ladder changes perceptual scope within `Precog / Clairvoyance`.
Nookinesis is a separate Flynt `Resynce / Telepathy` compatibility capability, not a Fairy
stage or an unrestricted mind-control upgrade. Living Aura resists, consent and
identity remain meaningful, and influence cannot create knowledge from nothing.

Glaüshouse Precog manipulation follows the locked sequence:

```text
Suggestokinesis → Affectokinesis → Cognikinesis → Mnemonokinesis
→ Volitikinesis → Prognokinesis
```

This is indirect influence through prediction, emotion, interpretation, memory,
and decision pressure. Direct movement of attention or intention remains
Flynt's Nookinesis.

## Combined Current/Aura expressions

| Current + Aura | Result |
|---|---|
| Gargoyle Telekinesis + Pixy-stage Precog | precise force guided by pressure and attention |
| Winged Gargoyle + separately trained Nookinesis | aerial control while coordinating attention or rescue movement |
| Werewolf Somatokinesis + Muse-stage Precog | predictive movement and controlled transformation |
| Merman compound flow method + Siren-stage Precog | flow shaping, vibration, navigation, and communication |
| Troglodyte Presynce + Muse-stage Precog | deep structural intervention with consequence awareness |

Combined expressions require a Recipe or trained faculty manifestation. They
consume the costs of both axes and add coupling distortion if the Frame cannot
carry the relationship.

## Backend capability record

Every unlocked capability should be representable with:

```text
axis
capability_id
power_recipe_id
canonical_faculty
canonical_power_root
kinesis_alias
current_stage
aura_stage
origin_frame
target_frame
anchor
range
load
duration
resistance
distortion
recovery
recipe_requirements
consent_and_clearance
witness_and_recognition
```

## Pilot acceptance slice

The first vertical slice must support:

1. Gremlin Current Body baseline;
2. Gargoyle Synthesis branch;
3. Presence I telekinetic coupling;
4. winged Gargoyle movement and braking;
5. Pixy/Sprite perception support;
6. one Nookinesis attention effect;
7. safe release, strain, refusal, recovery, and return;
8. a Recipe record preserving every material, evidence fragment, and outcome.

The matrix is progression vocabulary, not new kernel authority. The recursion
kernel receives only the bounded state transition and replay data derived from
these records.
