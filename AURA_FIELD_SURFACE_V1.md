# Aura Field Surface V1

Status: V1-compatible gameplay surface with corrected V2 Aura Farm partition
Backbone: `HOLLOW_GROVE_COMPROMISE_V1_DRAFT.md`
Constitutional authority: `SANDMANOR_CONSTITUTION_V2.md`
Executable contract: `src/world/aura_field.rs`
Gameplay projection: `aura-field.working-land`

## Aura Farm Geographic Lock

There is one upper-right composite triangle named **Aura Farm**. It is divided
by the median from Sandmanor to the midpoint of the Stonebend–Junction edge
into two equal-area triangular cultivation halves:

- upper half: **Aura Field**;
- lower half: **Content Farm**.

Content Farm shares the complete Sandmanor–Junction edge with Aura Beach
directly below it. Aura Beach remains its own complete lower-right triangle.

Older machine identities such as `ConstitutionalRegion::AuraFields`,
`AuraFieldsStewardship`, and `site.sandmanor.aura-fields` remain stable solely
for archive, event, API, and save compatibility. The V1 gameplay surface ID
`aura-field` remains a compatible entry into the Aura Farm composite.

## Aura Farm Constitutional System

The Minorian Aura Farm contains two cultivation halves:

- **Aura Field** cultivates and preserves what a family is and projects:
  identity, reputation, memory, continuity, tradition, relationships,
  presence, heritage, accumulated Aura, and household culture. Its physical
  fields also cultivate food, medicine, materials, and useful household life.
- **Content Farm** cultivates what a family has and makes: family,
  descendants, relationships, memories, stories, traditions, knowledge,
  craft, creative work, possessions, household culture, inheritance,
  reputation, accumulated experience, and material or cultural output.

They remain parts of one Sandmanor system, not separate Houses or indexes.

> **Content Farm cultivates what the family has and makes. Aura Field preserves what the family is and projects.**

Healthy Content Farm practice forms and sustains family, preserves memory,
transmits tradition, cultivates skill and craft, stewards inheritance, and
nourishes attention. Media production may be one expression, but the Content
Farm is not an internet-content studio or social-media production system.

> **A field can be overworked. So can an audience.**

The Content Farm is both a typed cultivation half and an inspectable entry on
the working-land map. It is not a new House, route, jurisdiction, or authority.

## Boundary and Access

Aura Farm is the upper-right interior triangle of the Hollow Grove screen:

| Boundary point | Normalized per-mille coordinate |
|---|---:|
| Stonebend | `(500, 140)` |
| Sandmanor | `(735, 500)` |
| Aura Ridge Junction | `(500, 500)` |

The exact partition is:

| Region | Polygon |
|---|---|
| Aura Field | `(500,140)` → `(735,500)` → `(500,320)` |
| Content Farm | `(500,320)` → `(735,500)` → `(500,500)` |
| Aura Beach | `(500,500)` → `(735,500)` → `(500,860)` |

Aura Farm is fill enclosed by the world route structure, not a tenth route. Its
three lawful gameplay approaches are:

- Aura Ridge, carrying witness toward the Stonebend boundary;
- Aura Way, carrying public design and comparison;
- Mt. Aura, carrying aspiration, endurance, and return.

Route access establishes physical entry only. It grants no land claim,
stewardship, proof, Clearance, recognition, office, or Title.

## Working Land

The Aura Field half contains two named Aura farms and one Engagement Farm. The
Content Farm occupies the equal lower half:

- **Upper Aura Farm** grows grain and pulse crops in rotations that preserve
  soil structure and make repeated yield comparison possible;
- **East Aura Farm** grows vegetables and fodder while holding seasonal trials
  whose failed results remain evidence.
- **Content Farm** cultivates family, relationship, memory, tradition,
  knowledge, craft, creative work, possessions, inheritance, reputation, and
  accumulated experience alongside the Aura Field.
- **Engagement Farm** gives capable people a working place to try voluntary,
  finite joint labor, apprenticeship, recruitment company, and disclosed
  responsibility before taking on a longer commitment. Every engagement names
  its work, term, safety edge, evidence, and exit. Participation never implies
  romance, consent beyond the stated task, recruitment, a Bond, ownership, or
  debt for leaving.

All are working units inside the same Aura Farm composite. A facility may
change tenants, crops, methods, or internal parcel boundaries without changing
the two canonical cultivation halves.

The supporting field system is:

| Place | Field function |
|---|---|
| Content Farm | family and household life, memory, tradition, knowledge, craft, inheritance, reputation, and cultural output |
| Sandmanor Proving Plots | repeatable cultivation proof, including negative results |
| Engagement Farm | voluntary finite joint work, apprenticeship, disclosed roles, refusal, and leave-without-debt practice |
| Ridge Orchard | fruit, nuts, grafting, pruning, and pollination |
| Common Paddock | grazing, draft-animal rest, rotation, and manure recovery |
| Aura Irrigation Works | wells, channels, gates, drainage, and drought rationing |
| Common Barn | livestock shelter, hay, shared labor, and weather refuge |
| Field Granary | drying, grading, reserves, spoilage checks, and issue records |
| Seed House | selection, provenance, storage, exchange, and seasonal memory |
| Field Farmstead | dwelling, kitchen, wash, first aid, and work coordination |
| Tool Shed and Smithy | issue, sharpening, repair, harness, and spare parts |
| Field Produce Market | weights, public prices, exchange, and recognition records |
| Living Windbreak | Aura-pressure shelter, soil retention, and habitat |
| Field Apiary | pollination, honey, wax, and colony-health observation |
| Compost Yard | manure, crop residue, decay, heat, and return to soil |

The map places all seventeen facilities as distinct interaction witnesses. A
player can inspect their role, but inspection does not simulate ownership or
complete their work.

## Material Cycles

Aura Field functions through linked cycles rather than a single harvest:

```text
Seed provenance
→ soil preparation
→ planting
→ irrigation and tending
→ pest, weather, and health observation
→ harvest
→ grading and storage
→ market or reserve issue
→ crop residue and manure
→ compost
→ soil return
→ next rotation
```

The livestock loop joins paddock, barn, fodder crop, veterinary observation,
manure recovery, compost, and soil. The orchard and apiary join flowering,
pollination, fruit set, honey and wax production, colony health, pruning
residue, and compost. The tool loop joins wear, issue records, repair, proof of
a revised method, and safe return to service.

No output appears from atmosphere alone. Every claimed improvement must point
to method, labor, conditions, observations, and repeatable results.

## Water, Soil, Weather, and Aura

The V1 surface runtime's **legacy physical Aura expression** is perceptible
here as air, pressure, light, and revealed condition. It changes evaporation,
wind exposure, pollination, lodging risk, visibility, and the felt pressure of
work. These observations are compatibility mechanics, not the universal
definition of Aura in the Practitioner ontology. Aura does not choose a crop,
certify health, or recognize a claim.

Water records distinguish source, allocation, gate state, rainfall, field
delivery, runoff, and contamination. Drought rationing is public and
challengeable. Flood drainage protects soil without erasing the evidence of a
failed bed or damaged crop.

Soil records distinguish parcel, horizon, compaction, moisture, amendment,
rotation, contamination, and recovery. Compost is observed through inputs,
heat, time, decay, and safe use. A green appearance alone is not proof of
fertility or Clearance.

The living windbreak reduces pressure and erosion while holding habitat. It is
managed as field infrastructure, not decorative scenery.

## Four-House Constitutional Loop

The common civic loop applies to every serious field claim:

```text
Stonebend names.
Sandmanor proves.
Glaüshouse clears.
Flynt recognizes.
```

Stonebend names parcels, boundaries, continuities, tools, animals, seed lots,
and disputes. A Name establishes addressability; it does not prove a yield.

Sandmanor proves methods through disclosed scope, repetition, maintenance,
criticism, negative results, and measured outcomes. Proof does not grant
consent, Clearance, recognition, ownership, office, or Title.

Glaüshouse clears food, water, animal, worker, soil, and tool hazards within a
stated clinical scope. Clearance does not rename a parcel or recognize a
commercial claim.

Flynt recognizes public records, exchanges, achievements, resolved
obligations, and completed functions. Recognition does not manufacture proof
or convert work into ownership.

## Minorian Forms, Roles, and Stewardship

Minorians are the people of this interior/family side. Their Forms progress
**Gnome → Faun → Minotaur**; Gnome is not a generic name for all Sandmanor
inhabitants. Their distinct social roles progress **Hearthkeeper → Householder →
Husbandman**: maintenance, organization, and mature stewardship of household
life. Geomancer is a separate institutional attendant class, and Hecaton is
the Minorian sovereign Hand rather than a fourth Form.

Minotaurs perform mature field labor and stewardship after lawful progression.
Their duties cover tending, load, route
maintenance, harvest protection, worker protection, boundary defense, and
stabilization of field Current. Stewardship is responsibility, not dominion
over people or automatic ownership of the land.

Visitors, traders, healers, surveyors, witnesses, seasonal workers, and
apprentices may enter within the scope of access and consent. Being present in
Aura Field never infers lineage, standing, skill, office, or Synthesis
eligibility.

## Economy and Records

The produce market makes weights, grades, price terms, provenance, reserve
obligations, and disputed lots visible. Private exchange remains possible, but
no private record may masquerade as Flynt recognition.

The granary separates household stores, seed reserve, emergency reserve,
market stock, and challenged stock. Spoilage and shortfall remain recorded;
they may not be deleted to protect a steward's reputation.

Seed exchange preserves source, generation, conditions, known failure, and
recipient. A renamed seed lot retains provenance. A promising trial remains a
trial until its stated proof conditions are met.

## Hazards and Refusal

Field hazards include:

- drought, flood, erosion, fire, frost, and excessive Aura pressure;
- contaminated water, soil, compost, food, or seed;
- animal injury, crop disease, pest pressure, and apiary collapse;
- tool failure, overwork, incapacity, coercion, and unsafe assignment;
- false weights, hidden spoilage, erased failures, and unsupported yield
  claims;
- boundary conflict, reserve diversion, and attempted stewardship by presence.

Any worker may report a hazard or refuse work outside consent or safe scope.
Emergency action must remain narrow, recorded, reviewable, and temporary.
Protecting a harvest never authorizes ownership of a worker.

## Seasonal Rhythm

Aura Field retains continuity across seasonal states:

```text
Plan and repair
→ prepare and plant
→ tend and prove
→ harvest and clear
→ store and recognize
→ rest, compost, and revise
```

Seasons change which work is visible; they do not create new regions. The same
parcel and facility identities persist through fallow, crop rotation, damage,
repair, and succession.

## Gameplay Contract

`aura-field.working-land` is a fixed `20 × 18` map. It projects cultivated
plots, orchard, paddock, paths, irrigation, barn, granary, farmstead, market,
and all cataloged facility witnesses.

The runtime exposes an interior-surface view with:

- stable surface identity and singular display Name;
- dominant Sandmanor Minorian interior attribution;
- the three-point boundary;
- three lawful access routes;
- every facility identity, kind, and working function.

Entering from an unrelated route fails without revising state or emitting an
event. Accepted entry persists in gameplay archive schema V3 and replays
through the same surface-access rule and Runtime Rampart binding. Godot renders this immutable projection
and does not own the region, facility catalog, or constitutional decisions.

Inspect all three surface contracts with:

```bash
cargo run --quiet --features dev-tools --bin hg-dev -- aura_surfaces_audit
```

## Completion Boundary

This surface implementation establishes the place, its working systems, its
route access, its institutional limits, and inspectable gameplay witnesses.

The living-surface pass now adds deterministic shifts and Aura weather, soil
moisture, irrigation reserve, crop and livestock health, granary reserve,
labor, harvest readiness, scheduled workers, and the evidence-gated drought
allocation case. Mt. Aura stone can strengthen the irrigation works through a
recorded custody lot. These states persist and replay through the authoritative
gameplay event stream.

It does not yet claim parcel-tenure adjudication, individual crop or animal
simulation, a complete commodity market, or the deeper biological or
metaphysical mechanics of Gnome-to-Faun-to-Minotaur progression.

Those systems may be built inside Aura Field without multiplying the region.
The living contract is
`HOLLOW_GROVE_LIVING_SURFACES_AND_EXTRACTION_V1.md`.
