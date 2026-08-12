# Hueman Synthesis Pilot Specification V1

**Status:** first visual vertical slice
**Subject:** the player Hueman at Aura Ridge
**Purpose:** test trainer/avatar continuity against several Synthesis forms before full asset production.

## Pilot identity

The pilot is the player’s ordinary Hueman before major kingdom-form commitment. The player may customize face, hair, skin tone, clothing accent, and starting tool. The sprite must remain recognizable after every transformation.

Base continuity anchors:

- a distinct head/hair silhouette;
- a short field coat with one replaceable House-color seam;
- a personal hand tool carried at the hip;
- a two-pixel Aura mark near the collar or wrist;
- a grounded, slightly forward working stance.

## Trainer sprite set

Use a 32×32 logical sprite with four-direction overworld views and 48×48 battle/registry views.

Required poses:

1. idle and directional walk;
2. carrying or repairing a route object;
3. speaking/witnessing;
4. refusal or stop gesture;
5. rest/recovery;
6. preparation for Synthesis.

The ordinary trainer palette is slate, parchment, muted teal, and one configurable accent. No transformation may erase the face, coat seam, tool, or Aura mark.

## Synthesis forms and Current techniques

Creature-like forms are the actual Synthesis transformations. The first pilot family is:

| Form | Function | Preserved anchor |
|---|---|---|
| Gremlin | improvises small repairs and manipulates mechanisms | personal tool becomes a hand/ear silhouette |
| Goblin | carries, scavenges, and adapts under field pressure | coat seam becomes a belt or utility band |
| Gargoyle | anchors, guards, and withstands structural strain | stance and shoulder line remain recognizable |
| Traverse Form | crosses difficult terrain and vertical routes | Aura mark becomes a navigation node |
| Witness Form | reveals and records a contested state | eyes/head silhouette remain exposed |
| Return Form | dissolves safely back to ordinary Hueman | all anchors reappear before the effect ends |

Gremlin, Goblin, Gargoyle, and the later creature families receive 48×48 registry/battle sprites and 32×32 overworld icons. They are not separate species or summoned companions; they are Hueman Synthesis forms with preserved lineage.

Signal and Brace are Current-Synthesis techniques, not creature forms. Signal is controlled transmission through a repeated Current pathway. Brace is temporary stabilization through measured load distribution. Both are closer to experimental engineering than spellcasting: they require setup, materials or energy, observable constraints, failure modes, and recovery time. A Hueman may use Signal or Brace while ordinary, while transformed, or while assisting another Being.

## Required state coverage

For Gremlin and Gargoyle, produce idle, active, strain, refusal, damage, recovery, transform-in, and transform-out. For Goblin and the remaining forms, produce idle, active, and transform-in/out first. Signal and Brace each need setup, active, overload/failure, safe release, and recovery indicators rather than creature animation cycles.

No “damage” frame may be only a red flash. It must alter posture, silhouette, or Aura visibility. No “refusal” frame may be treated as an error animation; it is a valid agency state.

## Review gates

- At native scale, the trainer is identifiable in every form.
- In monochrome, each form remains distinguishable by silhouette and function.
- The form’s preserved anchor is visible without a tooltip.
- The active, strained, refused, and recovering states are visually distinct.
- Returning to trainer form does not imply that consequences disappeared.
- The sheet reads as a registry of one Being’s capacities, not a list of unrelated creatures.

## Production order

1. Base trainer overworld sheet.
2. Base trainer battle/registry sheet.
3. Gremlin and Gargoyle forms with full state coverage.
4. Goblin and Traverse forms with core states.
5. Signal and Brace Current-Synthesis technique diagrams and effect sprites.
6. Return sequence and monochrome witness variants.
7. Review against `HOLLOW_GROVE_HUEMAN_SYNTHESIS_ART_GRAMMAR_V1.md` before expanding the cast.
