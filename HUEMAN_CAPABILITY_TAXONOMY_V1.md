# Hueman Capability Taxonomy V1

**Status:** active expression catalog plus LEGACY V1 backend adapters
**Authority:** subordinate to `HUEMAN_FACULTIES_V1.md`, House constitutions, and the Synthesis canon.

This taxonomy gives player-facing and Recipe-facing names to capabilities. The
formal faculty remains constitutional ownership. Current and Aura are semantic
practitioner axes; a capability term may express one of those axes. It never
creates a new House, office, species, or kernel authority.

`CurrentPowerRoot` and `AuraPowerRoot` are LEGACY physical/runtime fields. Their
stable identifiers remain decodable for Recipe and replay compatibility, but
they do not define Practitioner Current or Aura.

## Capability stack

```text
Being → People/Frame → canonical Faculty and power root
     → scoped Kinesis alias → Recipe/Synthesis modification
     → skill nodes → witnessed use and return
```

## Flynt regional kinesis taxonomy

| People or Form | Regional expression | Canonical relationship | Scope |
|---|---|---|---|
| Gargoyle | Telekinesis | `Resynce / Telekinesis` | external force across distance |
| Werewolf | Somatokinesis | `Presynce / Somatokinesis` | self-motion, reflex, impact, and controlled transformation |
| Merman | Hydrokinesis | compound Recipe method resolved through canonical contributions | water pressure, flow, vibration, and navigation |

These are distinct expressions of applied force. Gargoyles move the world;
Werewolves move the self; Mermen move force through a medium. None of the three
is an unlimited power. Range, mass, line of coupling, fatigue, living Aura
resistance, environmental compatibility, and recovery remain active limits.

Hydrokinesis is a regional outcome name, not an additional persisted power
root. A Merman Recipe must identify the canonical faculty/root contributions
that produce it.

## Locked active House expressions

| House | Axis | Expression | Faculty | Legacy V1 root adapter |
|---|---|---|---|---|
| Glaüshouse | Current | Biokinesis | Precog | Biokinesis |
| Glaüshouse | Aura | Noomancy | Precog | Clairvoyance |
| Flynt | Current | Telekinesis | Resynce | Telekinesis |
| Flynt | Aura | Nookinesis | Resynce | Telepathy |

Noomancy knows, reads, interprets, or regulates an internal mind/Aura state.
Nookinesis moves, acts upon, or manipulates such a state. They are not aliases.

Stonebend's exact eight Current/Aura level pairings are locked in the Synthesis
canon. Sandmanor's locked axes and six equations are defined there as well; the older
Barokinesis/Haptokinesis/Technokinesis/Ideokinesis records remain LEGACY V1
root adapters and are not active canonical House-axis assignments.

## Locked Aura manipulation boundary

Glaüshouse Precog uses the indirect manipulation sequence:

```text
Suggestokinesis → Affectokinesis → Cognikinesis → Mnemonokinesis
→ Volitikinesis → Prognokinesis
```

Flynt Nookinesis is the inward action/manipulation family. The
Glaüshouse sequence shapes conditions and predicts decisions; it does not seize
another Being's will.

## Canonical faculty relationship

- Gargoyle Telekinesis is a Flynt Gargoyle Synthesis expression of
  `Resynce / Telekinesis`: synchronization across separation supplies the
  remote-force relationship. Presynce may support embodied load, but does not
  own the remote-force unlock.
- Werewolf Somatokinesis is a `Presynce / Somatokinesis` Current/Frame
  expression. Resynce may coordinate it with a group, route, or encounter
  without changing its root.
- Merman Hydrokinesis is a compound Current/Aura Recipe method. It may include
  Resynce coordination, but must preserve every canonical contribution rather
  than becoming a sixth power root.
- Glaüshouse Noomancy may know, read, interpret, and regulate an internal
  mind/Aura state but does not become Flynt Nookinesis.
- Sandmanor Prefog and Prefig generate, model, and demonstrate legal
  possibilities and Recipes; they do not silently grant a capability.

## Progression grammar

Every kinesis uses the same skill vocabulary:

1. **Sense** — detect the relevant force, medium, or boundary.
2. **Couple** — establish a safe Current/Aura relationship.
3. **Direct** — produce one deliberate movement or effect.
4. **Sustain** — maintain the effect under ordinary pressure.
5. **Shape** — alter direction, timing, or distribution.
6. **Coordinate** — use the capability with tools, Beings, routes, or teams.
7. **Return** — release, recover, repair, and record consequence.

Gargoyle Presence is the Telekinesis progression. Wing Synthesis expands the
Gargoyle's movement envelope but does not skip Presence training. Werewolf
Somatokinesis grows through controlled body states and recovery. Merman
Hydrokinesis grows through flow complexity, pressure range, and environmental
coupling.

## Recipe and Synthesis fields

Every capability-bearing Recipe should record:

- `capability_id` (a stable identifier such as
  `capability.flynt.telekinesis`);
- `power_recipe_id` naming an existing canonical `PowerRecipeDefinition`;
- `kinesis_alias` for presentation and Recipe vocabulary;
- `canonical_faculty`;
- `canonical_power_root` (`CurrentPowerRoot` or `AuraPowerRoot`);
- `current_aura_polarity`;
- `origin_frame` and `target_frame`;
- `anchor` and preserved identity marks;
- `pressure`, `range`, `load`, and `duration` limits;
- `distortion` and failure mode;
- `consent`, compatibility, Clearance, and witness requirements;
- `maintenance`, recovery, and return condition;
- `provenance` for every material and evidence fragment.

## Non-goals

The taxonomy does not turn every House practice into a superpower, divide the
Current ladder into exclusive species, add a parallel power-root enum, or add
these names to the recursion kernel. It is a validated adapter vocabulary for
gameplay, Recipes, UI, art states, and future persistence schemas.
