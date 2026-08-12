# Hollow Grove Recipe Constitution V1

**Status:** additive Recipe and discovery law
**Purpose:** make Synthesis a treasure hunt built from evidence, materials, and lived trials.

## Discovery loop

`Rumor → Clue → Fragment → Material → Method → Trial → Recipe → Maintenance`

Recipes are discovered possibilities, not menu upgrades. A complete Recipe
must preserve provenance and name its Form, materials, operation order,
compatibility condition, Current/Aura balance, failure warning, maintenance
requirements, and return condition.

`src/recipe_discovery.rs` is the discovery and evidence gate above the existing
`PowerRecipeDefinition` catalog and `SynthesisRecipe` compiler. It does not
replace either system. A successful discovery must identify the canonical power
Recipe it exposes before the capability can become a candidate.

## Evidence and fragments

Rumors originate in people, ruins, tools, records, failed Forms, and witnessed
events. Fragments may be incomplete, damaged, biased, mistranslated, or
contradictory. The player compares fragments rather than trusting a single
authority.

The winged-Gargoyle pilot enforces this order and minimum confidence:

```text
Stonebend structure (corroborated)
→ Sandmanor measurement (corroborated)
→ Glaüshouse compatibility (cleared)
→ Flynt witness (witnessed)
→ Stonebend material (corroborated)
→ Sandmanor method (corroborated)
→ trial synthesis → final clinical Clearance → safe flight and return
```

Out-of-order or weak evidence does not advance discovery, and no trial may be
recorded before all six fragments are accepted.

House evidence has different strengths:

- Stonebend: load scars, materials, structural diagrams, repair records.
- Sandmanor: measurements, lineage, models, annotations, and criticism.
- Glaüshouse: compatibility charts, patient testimony, consent, and recovery.
- Flynt: demonstrations, challenge records, operational results, and witnesses.

## Materials

Material identity includes source, condition, treatment, prior use, and
provenance. Substitution is allowed only when the Recipe permits it; otherwise
it creates a variant, instability, or refusal rather than a weaker copy.

## Trial synthesis

An incomplete Recipe may be attempted. Outcomes include stable discovery,
variant discovery, partial Form, harmful remainder, new clue, refusal, or
recovery debt. Failure must reveal information and must not be a meaningless
resource tax.

## Authority and safety

Stonebend may develop structural methods. Sandmanor may validate models and
lineage. Glaüshouse controls clinical compatibility, consent, Clearance,
recovery, and final constitutional Synthesis. Flynt may test and recognize
demonstrated function. No Recipe fragment silently creates office, Title,
consent, Clearance, or permanent authority.

## Pilot Recipe family: winged Gargoyle

The pilot target is the Flynt Gargoyle branch, identified as
`recipe.flynt.winged-gargoyle.v1`, with Stonebend Presynce support for embodied
load and structural continuity. The wings provide anatomical lift; the unlocked
remote-force capability is Flynt `Resynce / Telekinesis`, which supplies
orientation, braking, and controlled force across separation. Structural
evidence and support do not transfer Recipe or capability ownership to
Stonebend.

Required evidence: Stonebend load geometry, Sandmanor flight measurements,
Glaüshouse compatibility and recovery records, and a Flynt storm-flight
witness. The completed Recipe must define lift limits, Current cost, Aura
orientation, failure/recoil, maintenance, and safe return to ordinary Hueman
form. Only a stable trial followed by final Glaüshouse Clearance exposes the
Flynt Telekinesis candidate through the existing
`recipe.flight.resynce-propulsion` power Recipe; safe flight and witnessed
return complete the hunt.
