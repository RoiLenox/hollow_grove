# Hollow Grove Hueman / Synthesis Art Grammar V1

**Status:** V1 visual specification proposal
**Scope:** Hueman trainer/avatar sprites and Pokémon-like Synthesis transformation sprites
**Boundary:** Presentation grammar only; this document does not add authority, lore law, runtime state, or new Frame IDs.

## 1. Core distinction

Hueman sprites show the persistent Being: the person, trainer, and avatarized real-world self.

Synthesis sprites show what that Being becomes through a bounded Recipe, Frame, Skill, Aura, and Current history. A Synthesis form is not a separate species or replacement character. It must retain a visible lineage to its originating Hueman and return with memory, strain, evidence, and consequence intact.

**Visual rule:** the trainer answers “who is this?”; the Synthesis form answers “what can this Being become, do, or endure right now?”

## 2. Pixel and palette constraints

- Use a fixed logical grid: 16×16 icons, 24×24 or 32×32 ordinary sprites, and 48×48 or 64×64 important Synthesis forms.
- Preserve silhouette readability at native scale and in monochrome.
- Use one contour color, two or three body colors, one House or role accent, and at most one Aura highlight.
- Prefer deep colored inks over universal pure black; reserve near-black for void, Hollow, or strongest separation.
- Palette swaps must communicate a real change in House, environment, condition, or witnessed state.
- Do not use detail to compensate for an unclear silhouette.

## 3. Hueman trainer/avatar grammar

Every trainer sprite must preserve at least three continuity anchors:

- face, hair, or head silhouette;
- posture, gait, or characteristic stance;
- clothing, tool, House mark, or personal color.

Required ordinary states are idle, walking, working, speaking/witnessing, repairing, resting, and refusal. Important characters may add strain, grief, celebration, or retirement/return states.

Trainer sprites should look lived-in rather than heroic by default. Tools, repairs, carrying posture, and imperfect clothing are valid identity information.

## 4. Synthesis-form grammar

Each Synthesis form records its origin through at least two anchors selected from face/eyes, hair silhouette, clothing motif, personal tool, House color, signature stance, and Aura mark.

Each form must communicate:

1. its immediate function or capacity;
2. the Frame or Recipe being expressed;
3. its current condition (stable, strained, damaged, refused, recovering, or returning);
4. one remainder or obligation that survives transformation.

Synthesis forms may be animal-like, elemental, mechanical, spectral, or abstract. Their appearance must still read as a transformation of a Hueman, not an unrelated creature.

Gremlin, Goblin, Gargoyle, and related creature families are valid Synthesis forms. Current-Synthesis techniques such as Signal and Brace are separate visual objects: diagrams, conduits, field effects, load paths, and instrument-like overlays. They may accompany a form but must not be mistaken for the form itself. Their presentation should imply repeatable physical or informational behavior, measurable limits, setup, overload, and recovery rather than unexplained spellcasting.

A shared Current-ladder form can be approached through four House traditions:
Stonebend emphasizes material structure and repair; Sandmanor emphasizes
measurement, lineage, and method; Glaüshouse emphasizes embodied care,
adaptation, consent, and recovery; Flynt emphasizes demonstrated capacity,
force, witness, and consequence. Gargoyle is the separate Flynt Synthesis
branch. Cross-House structure, proof, and clinical contributions support its
Recipe without creating Stonebend, Sandmanor, or Glaüshouse Gargoyle variants.
These traditions remain procedural readings of one Current, not separate
species or capability ownership.

## 5. Transformation sequence

The standard visual sequence is:

`Hueman avatar → preparation cue → partial overlay → Synthesis form → strain/adaptation → return`

The transition should reuse the trainer’s anchor pixels rather than replacing the character with an unrelated animation. Return frames should show continuity, not a clean reset.

## 6. Animation-state vocabulary

The reference-sheet state density is retained, but each state has lived meaning:

| State | Meaning |
|---|---|
| Idle | stable identity and present condition |
| Move | direction and intention |
| Work | demonstrated capacity |
| Witness | social or institutional participation |
| Strain | pressure becoming visible |
| Refusal | boundary, consent, or unsafe continuation |
| Damage | consequence requiring care or restitution |
| Recovery | repair and resumed agency |
| Transform | bounded Synthesis transition |
| Return | form dissolves while history remains |

## 7. Sheet and UI grammar

Asset sheets should use compact registry-card presentation:

- stable labels and indexed entries;
- overworld trainer row;
- battle/command trainer row;
- Synthesis-form row;
- status and recovery variants;
- monochrome or silhouette witness variant;
- small icons for Current, Aura, consent, risk, witness, and maintenance.

The presentation should feel like a witnessed civic record, not a creature encyclopedia.

## 8. Pilot asset requirement

Before producing the full art library, create one pilot Hueman with:

- one overworld trainer sheet;
- one battle/command trainer sheet;
- three to six related Synthesis forms;
- idle, movement, strain, refusal, damage, recovery, transform, and return states;
- one monochrome/silhouette version for uncertainty or incomplete record;
- a short registry card documenting the anchors and Recipe relationship.

The pilot passes review only if the trainer remains identifiable inside every Synthesis form and every state remains readable at native scale.

## 9. Non-goals

This grammar does not authorize copying Pokémon names, silhouettes, poses, proprietary interface decoration, or exact palettes. It borrows the useful handheld discipline—grid, clarity, state coverage, and catalog structure—while making Hueman identity and Synthesis continuity the defining language.
