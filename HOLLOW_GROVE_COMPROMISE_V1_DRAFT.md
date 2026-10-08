# Hollow Grove Compromise V1

Status: authoritative Hollow Grove Constitution
Authority: shared constitutional compact connecting the four House constitutions
Source seed: `/home/warren/hollow_grove_compromise.txt`
Purpose: one backbone for constitutional law, world lore, progression, culture,
and playable expansion

## Authority Posture

The Hollow Grove Compromise is the Hollow Grove Constitution.

> The Houses govern their domains. The Hollow Grove Compromise governs how
> those domains coexist.

The Compromise is the shared constitutional compact connecting the Stonebend,
Sandmanor, Glaüshouse, and Flynt constitutions. It governs universal law,
constitutional interoperability, cross-House interfaces, and the conditions
under which the four Houses remain one Hollow Grove. It is not a fifth House, a
superior sovereign, a central ruler, a replacement for a House constitution, a
second Bond constitution, or a second constitutional runtime.

This file retains `DRAFT` in its filename for stable repository references.
Before the Whole Constitutional Reconciliation Pass it was classified as an
active synthesis draft and master integration outline. That earlier status is
provenance, not current authority.

The Compromise does not silently supersede a House rule inside that House's
lawful domain, nor does it alter production reducers or persistence formats
without an explicit constitutional reconciliation and executable migration.

Each branch is classified as one of:

- **Locked** — already established by normative law and production behavior;
- **Reconcile** — present in the Compromise and existing work, but names,
  membership, or extent disagree;
- **Expand** — established in principle but not yet sufficiently embodied in
  characters, places, choices, or gameplay;
- **Proposed** — new material that must remain additive until ratified and
  validated.

When this document disagrees with locked authority, the disagreement is an
explicit reconciliation task. A client, artifact, or draft may not resolve the
disagreement by itself.

The detailed Bond and Constitutional Runtime law is specified by
`HOLLOW_GROVE_V2_CONSTITUTIONAL_SPECIFICATION.md` beneath this Compromise. That
specification is normative for its runtime domain but is not another Hollow
Grove Constitution.

The detailed spatial-composition, inheritance, viewpoint, and replay law is
specified by `SPATIAL_INHERITANCE_V1.md` beneath this Compromise and beneath
Constitutional Geography and Route Network authority. It is normative for
spatial identity and projection, but it creates no House, route meaning,
ownership, access right, or second constitutional runtime.

The operational role, AI-associated persona, and Current Synthesis boundary
contracts are likewise Compromise-level interfaces. They may prepare and carry
evidence, but they cannot perform a House gate or redefine the PLEB/META route
grammar. Cleopatra is the Underway/BLEP reflection, Hal is the Boardwalk/META
public interface, and Clouseau is the Current Synthesis `PLEB ↔ PLEB` witness.
Their accepted records enter the Rampart of Roi and Roselina only through the same authority,
provenance, refusal, and replay envelope as every other domain record.

## Backbone

```text
Universal Laws
└── Constitutional Runtime
    └── Bond Constitution
        └── Four Houses
            └── Hueman Constitution
                └── Sympian Lineage
                    └── Practitioner Ontology
                        ├── Current
                        ├── Aura
                        └── Level 1 Synthesis
                            └── House-Specific Geometry
                                └── Level 2 / Higher Synthesis
```

The order is causal rather than hierarchical authority. Later layers express
earlier laws in living, social, and playable form; they do not rewrite them.

## Integration Matrix

| Compromise branch | Status | Existing authority | Expansion responsibility |
|---|---|---|---|
| Universal Laws | Locked / Balance Adopted | `CURRENT_SYNTHESIS_HOLLOW_GROVE_WORLD_CONTEXT_v0.1.0.md`; semantic and embodied foundations; `HOLLOW_GROVE_METAPHYSICAL_BALANCE_V1_DRAFT.md` | make Current, Aura, Relativity, Hollow, and Synthesis perceptible through play, including cost, excess, and return |
| Constitutional Runtime | Locked | `src/constitutional/`; V2 specification | preserve determinism, evidence, replay, serialization, migration, and fail-closed decisions |
| Bond Constitution | Locked / Expand | `src/constitutional/bond.rs`; runtime, persistence, trace, and application layers | give lawful Bond events social and narrative consequences |
| Spatial Inheritance | Locked / Expand | `SPATIAL_INHERITANCE_V1.md`; neutral kernel, world binding, archive, bridge, Godot, and runtime verifier | author lawful district/site/room/object/detail instances without copying or redefining the canonical plan |
| Four Houses | Locked / Expand | the four House constitutions and world projections | express Name, Proof, Clearance, and Recognition through ordinary people and conflicts |
| Hueman Constitution | Locked / Expand | Frame, Flow, Glow, faculty, progression, and Recipe law | turn faculties into player-facing perception and action grammar |
| Sympian Lineage | V1 Reconciled / Expand | `HOLLOW_GROVE_CLASSIFICATION_RECONCILIATION_V1.md` | embody Natural, Matural, and Synthetic continuity without replacing Being identity |
| Practitioner Ontology | Locked / Expand | `HOLLOW_GROVE_SYNTHESIS_CANON_V1.md`; typed practitioner registry | preserve Practitioner as the whole person; keep Current relational, Aura internal, and faculty orthogonal |
| Legacy Frame Aliases | LEGACY / DEPRECATED | stable V1 Frame enums, archive codecs, and replay fixtures | decode Pixy and Faerie as aliases without displacing active Pixie and Fairy canon |
| Synthesis Constitution | Locked / Expand | `CURRENT + AURA → SYNTHESIS`; `Troglodyte + Muse → Seer`; House-specific higher Synthesis law | distinguish Level 1 from Level 2 and preserve different House geometries |
| Regional / Cultural Domains | Expand / Proposed | geography, House, faculty, and Flynt law | build lived cultures, characters, economies, routes, Bonds, and playable cases |
| Operational Lore Kernel | Adopted / Expand | `HOLLOW_GROVE_OPERATIONAL_LORE_KERNEL_V1_DRAFT.md`, functional-lore archive schema V2, and Kernel Constitutional State | give every role a bounded question, input, output, stop law, and handoff |
| AI-associated personas | Reconcile / Expand | `HOLLOW_GROVE_AI_ASSOCIATED_PERSONAS_CONSTITUTION_V1_DRAFT.md`, `src/ai_personas.rs` | carry Cleopatra, Hal, and Clouseau advisories without committing authority |
| Current Synthesis boundary | Reconcile | PLEB/META client and routing contracts; Current Synthesis remains owner | accept only scoped Clouseau/Hal/Cleopatra witnesses through a shared adapter |
| Rampart continuity | Locked / Expand | `THE_RAMPART_V1.md`, `THE_RAMPART_IMPLEMENTATION_V1.md`, `src/runtime_rampart.rs` | bind accepted records, preserve domain custody, and replay cross-layer consequence |

### Compromise-to-Rampart acceptance rule

The Rampart of Roi and Roselina accepts a record only when its originating constitutional
authority, subject identity, causal position, evidence, refusal state, and
handoff are present. Persona output, dialogue, route visibility, or player
support alone is never an accepted consequence. Rejected and inconclusive
records remain cross-domain history without becoming successful state.

## Universal Practitioner Laws

The non-negotiable semantic foundation is:

```text
PRACTITIONER
├── CURRENT
└── AURA
      together → SYNTHESIS

CURRENT + AURA → SYNTHESIS
```

Practitioner names the whole person. Current names relational participation in
a larger social, institutional, environmental, or collective whole. Aura names
individually cultivated internal knowledge, ability, magic, perception,
discipline, or capacity. Synthesis is the manifested integration of the two.
Faculty is a distinct recognition axis.

The formerly universal material statements that identified Current with blood
and Aura with air, pressure, or light are **SUPERSEDED** as lore definitions.
Existing runtime quantities and archive fields carrying those names remain
**LEGACY PHYSICAL EXPRESSIONS / COMPATIBILITY MECHANICS** until a versioned
serialization migration exists. Hollow's separate material law remains
`Hollow = pus`.

Level 1 practitioner Synthesis and Level 2 higher/composite Synthesis are
distinct. In particular, `Troglodyte + Muse → Seer` remains locked Level 2
canon and is not flattened into the Level 1 equation.

New lore must expose at least one observable consequence of the law it invokes.
A phrase that never affects a body, relationship, place, choice, or persistent
record remains atmosphere rather than functional lore.

The adopted additive balance contract in
`HOLLOW_GROVE_METAPHYSICAL_BALANCE_V1_DRAFT.md` supplies the formerly thin
Relativity grammar, shared material pressures, House shadow grammar, and
consequence lifecycle. It does not alter the neutral recursion kernel or create
new constitutional authority.

## Constitutional Runtime

Functional lore inherits the runtime requirements:

- causes are explicit;
- evidence is addressable;
- decisions replay deterministically;
- state changes serialize;
- migrations preserve identity and prior truth;
- unavailable authority fails closed;
- presentation never invents a result.

This layer is why a promise, betrayal, recovery, recognition, or transformation
can matter after the scene ends.

## Bond Constitution

Bond is the principal bridge between world law and character drama.

The locked lifecycle already supports:

```text
Formed
→ Validated
→ Active
→ Mature
→ Excess Calculated
→ Eligibility Decided
→ Tombstone Formed or Lawfully Omitted
→ Tombstone Validated
→ Flynt Recognition
→ Recorded
→ Resolved
```

Active Bonds may carry Current, accumulate it, observe Aura, evaluate polarity,
receive challenges, record defaults, and resolve those disputes. Resolution may
complete, renew, merge, branch, split, transfer, or dissolve a Bond.

Regional names such as **Goon Bond** describe a lawful social use of this
lifecycle. They do not create a second reducer or a shortcut around House
decisions.

## Spatial Inheritance

Spatial inheritance is the principal bridge between one canonical map plan and
its recursively situated world instances.

The locked publication and resolution chain is:

```text
Complete Declaration
→ Structural Validation
→ Hollow Grove Conformance
→ Atomic Publication
→ Cached Root Transform
→ Deterministic Viewpoint Selection
→ Versioned Archive
→ Independent Live Verification
```

The main world and every lawful child frame share one immutable local topology.
Each instance retains its own frame identity, parent, scale, bounds, placement
authority, evidence, lineage, root coordinate, and local coordinate.

“One topology” means the complete plan: six anchors (including distinct
co-located Ranina and Aura Ridge Junction identities), ten constitutional
routes, the local Aura Ridge East projection, route-band widths, Round control
points, and the Aura Field, Aura Beach, and Aura Basin polygons. In
particular, Current Sea and Current Seanad remain distinct and Glausbahn
remains Round at every inherited scale.

The legal scale grammar is:

```text
World
→ City
→ District
→ Site
→ Room
→ Object
→ Detail
→ Detail ...
```

Spatial inheritance is not constitutional inheritance of power. Containment is
not ownership; a coordinate is not evidence; a viewpoint is not a Bond; and a
child map cannot rename, reshape, or reinterpret a constitutional route.

## Four Houses

The common civic loop remains:

```text
Stonebend names.
Sandmanor proves.
Glaüshouse clears.
Flynt recognizes.
```

No act substitutes for another:

- Name is not proof;
- proof is not consent or Clearance;
- Clearance is not recognition;
- Flynt recognition is not Stonebend Title;
- a Bond is not ownership;
- transformation is not accession.

Regional stories should create tension between lawful House responsibilities
without reducing any House to a simple moral alignment.

## Hueman Constitution

Hueman remains one persistent Being across lawful Frame changes.

The player-facing faculty grammar is:

```text
Observe
├── Presynce feels physical emergence.
└── Resynce joins relational emergence.

Generate
└── Prefog opens legal possibilities.

Evaluate
└── Precog projects probable consequences with uncertainty.

Choose
└── the player and existing deterministic decision posture choose.

Demonstrate / Execute
└── Prefig gives the selected possibility provisional form.
```

Faculties disclose, relate, anticipate, generate, or demonstrate. They never
choose for the player or manufacture legality.

## Sympian Lineage Reconciliation

V1 interpretation is locked by
`HOLLOW_GROVE_CLASSIFICATION_RECONCILIATION_V1.md`.

The V1 categories are:

- **Natural Sympian**;
- **Matural Sympian**;
- **Synthetic**.

Sympian describes a Being's continuity relationship to embodied Synthesis. It
is not a species ladder, profession, or replacement for Frame identity. One
Being may move between categories while retaining stable identity; prior Names,
Forms, evidence, consent, Recipe, recovery, and recognition remain historical.
No legacy field, visual appearance, or transformation may infer a category.

## Stonebend Synthesis Ladder Reconciliation

The active Stonebend Synthesis ladder is:

```text
Gremlin
→ Goblin
→ Ghoul
→ Spectre
→ Troll
→ Ork
→ Ogre
→ Troglodyte
```

`Spectre` is the active fourth level.

The exact bidirectional Current/Aura crosswalk is:

```text
Gremlin      ↔ Pixie
Goblin       ↔ Sprite
Ghoul        ↔ Fairy
Spectre      ↔ Spirit
Troll        ↔ Siren
Ork          ↔ Sylph
Ogre         ↔ Nymph
Troglodyte   ↔ Muse
```

`Pixy` and `Faerie` remain stable compatibility aliases for active `Pixie` and
`Fairy`. The pair crosswalk does not create or rename House offices.

## Glaüshouse Branch Reconciliation

V1 interpretation is locked by
`HOLLOW_GROVE_CLASSIFICATION_RECONCILIATION_V1.md`.

Active Glaüshouse law uses a split-and-reunify topology:

```text
Nightingale / Pixie
├─ Marshal / Sprite → Spirit
└─ Matron / Fairy → Nymph
        both → Persephone / Siren → Prima Donna / Muse
```

`Pixie` is the active foundation identity; `Pixy` remains its stable runtime
and archive alias. `Fairy` is the active canonical spelling; `Faerie` survives
as a legacy serialization alias. Spirit is the advanced Marshal-branch
Synthesis, not a public office. Sylph is active as Ork's level-6 Aura
counterpart but creates no clinical office by itself.

## Synthesis Constitution

The following relationship remains a higher, cross-House Synthesis:

```text
Troglodyte + Muse → Seer
```

Troglodyte and Muse remain equal counterparts. Seer is a Level 2 Synthesis
result, not an extra rung that makes either source inferior. The Level 1
practitioner law `CURRENT + AURA → SYNTHESIS` remains distinct.

Every other proposed Synthesis must name:

- persistent Being continuity;
- distinct predecessor and result identities where required;
- jurisdiction;
- evidence;
- consent and capacity;
- Glaüshouse Clearance;
- provenance;
- recovery;
- recognition;
- lawful failure behavior.

## Regional / Cultural Domains

Established regional anchors include:

- Sandmanor's equal Minorian and Minoan traditions;
- Aura Field and the Minorian Gnome-to-Faun-to-Minotaur Form line;
- Aura Beach and the Minoan Elf-to-Satyr-to-Centaur Form line;
- We Fairy Men;
- the Flynt Gallows;
- the Flynt Boardwalk.

Aura Field is one singular geographic surface. It contains multiple Aura
farms, the Engagement Farm, plots, paddocks, orchard, irrigation, barns, granary, seed works,
farmstead, tool works, produce market, windbreak, apiary, compost, and proving
ground. None of those constituents becomes another Aura Field. Older plural
machine identifiers remain compatibility symbols only. The implemented
surface contract lives in `AURA_FIELD_SURFACE_V1.md`.

The Engagement Farm is the Field's bounded joint-work constituent. It lets
capable people test finite cooperation, apprenticeship, and voluntary party
company through named work, disclosed safety edges, evidence, and an exit.
Entering or working there never implies romance, consent outside the named
task, recruitment, a Bond, ownership, or debt for refusing or leaving.

Aura Beach is one singular Sandmanor Minoan exterior surface for coastal
approach, navigation, public meeting, weather, rescue, Elf design, and Centaur
mobility. `region.current-sea` is the distinct Current body that sets in
Glaüshouse. Its public concourse is densely populated and witnesses continuity,
but the civic crowd is a surrounding presentation of the Current body rather
than a replacement for it. The older civic-circulation jurisdiction and map key
remain compatibility projections only. The implemented surface contract lives
in `AURA_BEACH_SURFACE_V1.md`.

Aura Basin is one singular surface serving Flynt first through hunting,
competition, rescue, racing, collision, material recovery, and lawful return.
It remains constitutionally linked to Glaüshouse care, Stonebend naming and
Hollowing, and Sandmanor proof without becoming the property of any Flynt
institution or crew. The implemented contract lives in
`AURA_BASIN_SURFACE_V1.md`.

The Boardwalk is the first Compromise expansion target because it joins
Glaüshouse recovery to Flynt return and recognition while exposing Current,
Aura, Bond, identity, and agency in one public place.

Its proposed social polarity is:

```text
Pimp + Hoes
    Aura / glamour / patronage / leverage / recognition

Gimp + Goons
    Current / endurance / protection / labor / loyalty

Goon Bond
    the persistent relational bridge across the polarity
```

The detailed additive law lives in
`FLYNT_BOARDWALK_SOCIAL_CONSTITUTION_V1_DRAFT.md`.

## Functional Lore Rule

Every new lore unit should identify:

1. stable identity;
2. authority class;
3. location and jurisdiction;
4. involved Beings, Objects, institutions, or routes;
5. dominant world or House verb;
6. trigger;
7. evidence and uncertainty;
8. player-visible choice;
9. lawful state change;
10. persistence and replay behavior;
11. presentation projection;
12. failure or refusal path.

Character speech must also declare whether it expresses constitutional fact,
public record, local tradition, rumor, private belief, or deliberate
deception. A character may be wrong without rewriting the world.

## First Milestone

The first implementation sequence is:

1. ratify or revise the Boardwalk social constitution;
2. specify one returning-Goon case;
3. project the case through the existing party-RPG application boundary;
4. add one consequential choice backed by authoritative events;
5. replay the result exactly;
6. render the committed state in Godot;
7. expand only after the vertical witness passes.

The first slice does not require full combat, a six-member party, procedural
generation, or a new constitutional reducer.

### First milestone status

Implemented on 2026-07-23.

The executable witness is `boardwalk.return-vestibule`, backed by
`src/gameplay/world.rs`, `src/gameplay/boardwalk.rs`, the authoritative gameplay
runtime/protocol, and the common constitutional Bond runtime. Deterministic
coverage lives in `tests/gameplay_boardwalk.rs`. The Godot presentation exposes
the case without owning its evidence, choice, Bond, replay, or persistence
rules.

This completion authorizes building upward from the witness. It does not
silently ratify every social-constitution draft term or create a generic quest
engine.

### Four-House functional-lore milestone status

Implemented on 2026-07-23.

`src/world/lived_lore.rs` now makes the twelve-field Functional Lore Rule an
executable validation and persistence contract. Three lived loops exist for
each House, and the combined set covers every constitutional route. Each loop
resolves the House's exact act from a supplied `WorldSession`:

```text
Stonebend names.
Sandmanor proves.
Glaüshouse clears.
Flynt recognizes.
```

Missing authority fails closed, one House cannot substitute for another, and
the module has no recursion-kernel import. Canonical definitions and their
authority snapshots encode to checksummed JSON and replay exactly. The detailed
contract and loop roster live in
`HOLLOW_GROVE_FUNCTIONAL_LORE_INTEGRATION_V1.md`.

The gameplay runtime no longer reaches into
`world::institutional_access_fixture`. Dynamic schema-V2 institutional state
persists current role-addressed office holders. These current-incumbent IDs are
authority-bearing session identities, not personal names, biographies, visual
designs, or silently ratified succession law. Gameplay archive schema V3 embeds
the institutional snapshot and Runtime Rampart binding needed for
deterministic House-decision replay; schema V1 and V2 migrations preserve their
historical dependencies explicitly.

The Boardwalk witness now gives all four choices a stable, typed outcome.
Pimp Patronage, Goon Bond, and Limited Cooperation form distinct finite common
Bonds. Independent Return forms no Bond and persists protected refusal. Every
outcome requires Glaüshouse discharge Clearance and Flynt recognition, and the
player still cannot decide for the capable Returning Goon.

### Stonebend playable milestone status

Implemented on 2026-07-23.

`case.stonebend.current-sea-continuity.v1` projects the existing
`lore.stonebend.current-sea-continuity` contract through the authoritative
gameplay runtime and Godot client. Mercy Deep is a persistent restored Being
whose materially changed form creates a real identity-continuity conflict.

The player gathers five evidence sources, discloses all five Hueman faculties,
and gives nonbinding support. Stonebend then either seals the existing Name,
issues a linked provisional transformed-form Name, or seals a referral in which
no final Name acts. The runtime creates typed Stonebend evidence, decision,
Name, and Seal records against the live Hypergiant. The Mercury Mirror remains
evidence, the Freemason Seal remains execution, and no outcome grants Title.

Checksummed gameplay archive schema V3 replays the exact event, outcome,
institutional actor, constitutional records, and Runtime Rampart binding.
The detailed witness is
`STONEBEND_CURRENT_SEA_VERTICAL_SLICE_V1.md`. The all-region order and common
completion standard are locked in
`HOLLOW_GROVE_PLAYABLE_EXPANSION_ROADMAP_V1.md`.

### Route-network milestone status

Implemented on 2026-07-23.

The shared map now has one executable skeleton containing all ten
constitutional route projections: five Straight and five Round/curved. The
older Current Sea row is a Straight civic-interface compatibility projection;
Glausbahn is the Round road refinement sector. Every projection has one gameplay map, exact
House endpoints, immutable geometry, its locked verb and process, a route
witness, endpoint-sharing traversal, and schema-V3 replay.

Round and Straight describe shape only. They do not imply Light or Dark Aura,
morality, authority, Moxy, or constitutional function. Aura Field, Aura Beach,
and Aura Basin are now implemented as the three distinct interior surfaces and
have not been silently treated as roads. See
`HOLLOW_GROVE_ROUTE_NETWORK_V1.md`.

### Living-surfaces and extraction milestone status

Implemented on 2026-07-23.

Aura Field, Aura Beach, and Aura Basin now hold deterministic persistent
conditions, scheduled people, and one evidence-gated regional case each. Mt.
Aura, Stairway to Heaven, and the Stairway complex's nested Highway to Hell
Deepworks support solid-seam mining. Specialized extraction branches attached
to Riptide and Current Sea support distinct offshore Current-well operations:
Riptide retrieves emergencies and the historically named Current Sea Depth
Production Rig certifies sustained depth production. That branch does not make
the Current Sea civic route itself maritime.

The shared event-sourced aggregate records shifts, Aura weather, evidence,
nonbinding player support, duty-officer outcomes, extraction integrity,
hazards, output, contamination, cross-region consequences, and material
custody. Unsafe actions—including salvaging a living Being, continuing through
a blowout, bypassing certification, or cutting through a gas/roof hazard—fail
without revision or partial effects.

`Current-bearing brine` is a controlled working classification, never
permission to commodify blood from a living Being. Every market-capable lot
must affirm that living blood was excluded; contrary continuity or provenance
evidence diverts the material into quarantine, care, Illegal Hollowing, or
identity review.

The five extraction maps branch from their exact route through physical exits.
Highway to Hell remains an industrial descent inside the Stairway complex, not
an eleventh constitutional route. Full law, schedules, cases, custody, and
verification live in
`HOLLOW_GROVE_LIVING_SURFACES_AND_EXTRACTION_V1.md`.

### Deep Pressure cross-region campaign milestone status

Implemented on 2026-07-23.

`case.arc.hollow-grove.deep-pressure.v1` now joins the seven living-world duty
cases into one character-driven campaign without replacing their local
authority. The Riptide blowout reaches Aura Beach, Aura Basin, Mt. Aura,
Highway to Hell, Aura Field, Current Sea, and a final Boardwalk settlement.

All ten scheduled workers now physically appear on their current map, block
movement, speak through classified statements, and retain persistent
condition, trust, outcome memory, promises, boundaries, and recovery-Bond
history. The evidence journal distinguishes constitutional fact, public
record, local tradition, rumor, private belief, and deliberate deception.

After all seven duty outcomes and twelve required affected statements, the player
may give nonbinding support for Shared Burden, Crew-and-Coast Restitution,
Production Under Review, or Protected Refusal. The affected assembly commits.
The first three endings form distinct finite Bonds in the constitutional
runtime; refusal forms none. Every ending separately records Stonebend Name,
Sandmanor Proof, Glaüshouse Clearance, and Flynt recognition plus persistent
crew, coast, Field, Basin, production, and unresolved-obligation aftermath.

A locally lawful but compromised Riptide/Current Sea evidence combination bars
the production ending without rewriting either prior decision. Full campaign
law, speech, phases, endings, memory, controls, and verification live in
`HOLLOW_GROVE_DEEP_PRESSURE_CAMPAIGN_V1.md`.

### Party and recruitment milestone status

Implemented on 2026-07-23.

The active party now holds Hueman plus at most five companions. Six
Deep Pressure people have role-based candidate and continuity IDs so their
current display names may change without rewriting saves. Recruitment requires
a completed campaign aftermath and a physically faced conversation. The
candidate—not the player—decides from their condition, trust, remembered
ending, accepted work paths, and agency boundaries.

Accepted company is finite and voluntary. Decline is persistent, creates no
debt, and prevents repeated requests from becoming harassment. Exhausted
companions rest before leading. Each recruit has one location-bound field
action, including Brindle's Engagement Farm work reading inside the one Aura
Field. Party events, lead, recovery, decisions, actions, Godot presentation,
and schema-V3 save replay are implemented in
`HOLLOW_GROVE_PARTY_AND_RECRUITMENT_V1.md`.

### Power Recipe constitution milestone status

Implemented as an additive executable draft on 2026-07-25.

`src/world/power_recipes.rs` now defines the five exact Faculty Current–Aura
root pairs and seventeen competing methods for Flight, Invisibility, Healing,
and Super Speed. Each method preserves its processor, capacities, costs,
strain, range, duration, precision, restrictions, strengths, weaknesses,
trace, and risks. No outcome grants House ownership. The five-sector Power
Wheel requires Capacity, Proof, Recognition, a compatible Form and Frame, and a
discovered or taught Recipe; its prepared field set holds at most four methods.

`src/world/sympiote.rs` implements the host-specific Glaüshouse sequence from
sampling through reciprocal Synthesis or explicit rejection and failure. A
player-selected power package fails without mutation. Sympiote remains a
crafted living graft rather than an inferred Sympian lineage, and the
Gremlin-to-Gargoyle route never advances from a numeric threshold by itself.
Gremlincoin processes the Gremlin Way into evidence, while Gargoyle still
requires Recipe viability, lawful Synthesis, maintenance, and renewal. Stonebend
derives Recognized, Provisional, referred, Rejected, or narrowly bounded
Severance judgments from evidence and pressure testing.

Both recipe catalogs and Sympiote histories serialize with schema and checksum
guards and replay through the same validation law. The four recipe
institutions—Stonebend's proving arena, Sandmanor's Contest of Improvement,
Glaüshouse's clinical practicum, and Flynt's Manticorp Academy—cover all five
Faculties without claiming the outcome. The detailed boundary and verification
map live in `HOLLOW_GROVE_POWER_RECIPE_CONSTITUTION_V1.md`.

The adopted `HOLLOW_GROVE_SYMPIOTE_FOUNDATION_V1.md` adds a host-specific early
Form established at Glaüshouse through the existing crafted-graft lifecycle.
`src/world/sympiote_foundation.rs` and its gameplay binding record consent,
actual integration, recovery/discharge, bounded adaptation, rest, strain,
clinical review, and exploration of all existing House directions. Exploration
does not grant later Forms or authority. The gameplay V4→V5 migration preserves
old identity and Synthesis history without inferring a foundation or consent.

### Flynt dual-leadership, maintained Manticore, and Manticorp milestone status

Constitutionally recorded on 2026-07-25 as an additive canon extension.

`FLYNT_DUAL_LEADERSHIP_AND_MANTICORP_RECIPE_V1.md` and the executable
`flynt-constitution` model now lock `The Tross = Manticore = Mystery Man = Mr.
X`. Manticore is the Tross's specifically masculine mythological identity and
maintained Synthesis Form, not a second person or office; Manticorp remains
Flynt's distinct body of soldiers and lawful military order. The same sovereign
publicly commands Manticorp and leads We Fairy Men. When
We Fairy Men act collectively under his direction in the underworld, they
manifest as The Gallows,
leaving the containment-versus-expansion constitutional tension unresolved.

Earlier milestone text described Chimera as the lower apex integration. That
mechanical reading is **SUPERSEDED**. Chimera = SELECT and Manticore = INTEGRATE
are the current **WORKING** distinctions; detailed mechanics remain **STATUS:
OPEN**. Manticore remains the sovereign Wolf-to-Lion, Bat-to-Eagle,
Snake/Fish-to-Hydra transfiguration. The complete
Manticore Recipe remains divided among Bro White/Gargoyle, Beauty/Werewolf, and
Cinderellaman/Mariner-Merman. The Basin creature pairs are now locked as
masculine Gargoyle/feminine Medusa, masculine Werewolf/feminine Vampire,
masculine Mariner/feminine Mermaid, and masculine Manticore/feminine Chimera.
Bro White, Beauty, and Cinderellaman lead the first three respective sects
beneath the Tross through the proper collective identity **We Fairy Men**.
Mystery Men remains the separate consolidated clandestine and investigative
apparatus. Glaüshouse's Synthesis Ledger records the
medical, legal, trade, intelligence, and political consequences of maintained
Forms. The Tross may maintain the Form for life through bodily discipline,
Recipe renewal, divided Basin knowledge, institutional recognition, and
specialized Glaüshouse care. No runtime combat behavior was changed.

### Glaüshouse Continuance and generative ladder milestone status

Constitutionally recorded on 2026-07-25 without changing runtime combat,
movement, or the PokéWilds reference workspace.

Synthesis establishes a real living Form whose Continuance depends on
maintenance, renewal, Recipe practice, environmental compatibility,
institutional care, and knowledge of the Form's Ways. It is not a costume or
arbitrary loadout, and one procedure does not guarantee continuation forever.
Adjustment, Graft, Reconstruction, and Transfiguration are intended clinical
depths; Overgrowth is an emergency failure state. Host rejection and Sympiote
rejection remain distinct. Major Synthesis requires both technical and lived
viability.

The unified clinical ladder is Nightingale into either equal Matron or Marshal
specialization, then Persephone after lawful mastery of both domains, then
Prima Donna candidacy after physician, diagnostic, Recipe, surgical, teaching,
Ledger, and open-path mastery. Many Persephones and multiple qualified
candidates may serve while only one Prima Donna is active. Advancement
Toke/Tokens preserve one stable person and accumulated mastery.

Glaüshouse succession is the inverse of Stonebend: Stonebend prepares three
offices to remove one failed leader, while Glaüshouse requires one leader to
prepare many people to rise. Living Ledger authority remains with Persephones
and the Nightingale order; Recipe Ledger authority remains with Prima Donna.

### Aura Ridge and Central Junction market milestone status

Constitutionally recorded on 2026-07-25 as an additive public-economy layer.

`CENTRAL_JUNCTION_FOUR_POLE_ECONOMY_V1.md` and
`src/world/central_junction.rs` define Sandmanor Design as Creation of Form,
Flynt Engineering as Creation of Function, Stonebend Craft as Continuance of
Form, and Glaüshouse Repair as Continuance of Function. Hollow Grove uses one
unnamed standard ordinary currency; Toke/Tokens, shares, event positions,
indexes, and nonmonetary Gremlincoin remain distinct value records.

Central Junction is the district. The South Ridge Exchange is its primary
market institution. House Sector Halls verify domain facts without setting
prices; the Junction Board governs shared standards and recognition, the
Clearing House settles recognized obligations, and the Junction Wire publishes
the result. The four official indexes are filled by productive work but owned
by no House.

The deterministic Blackroot Workshop Conflict contract records its outcome
definitions before opening, receives House-domain attestations, passes through
Junction Board recognition, Clearing House settlement, and Junction Wire
publication, and rejects conflicted reviewers, settlement officials, or
commanders acting on positions they hold.

Current Haze remains unresolved possibility. Equal Gaze remains reconciled
perspective. Aura Beam remains the revelation or transmission of the visible
shared future. None is a market office, regulator, settlement service, or
financial ticker.

### Sandmanor guardian and reciprocal-succession milestone status

Constitutionally recorded on 2026-07-25 without changing combat, movement,
party, rendering, the PokéWilds reference workspace, Central Junction market
behavior, another House's authority, or the recursion kernel.

`SANDMANOR_GUARDIAN_AND_SUCCESSION_V1.md` and
`src/world/sandmanor/milestone.rs` complete the equal Minorian/Gnome interior
and Minoan/Elf exterior peoples. Aura Farm contains Aura Fields and Content
Farm. Aura Beach follows the canonical order Free Aura Beach, Southern Coast,
Current Break, Minoan County Courthouse, then Glaüshouse.

The Form lines are Gnome → Faun → Minotaur and Elf → Satyr → Centaur. Hecaton
and Pegasus are the Minorian and Minoan sovereign Hands, not additional Forms.
Qualification, Recipe authorization, Glaüshouse compatibility, Form, social
role, attendant class, Hand, mantle, jurisdiction, renewal, and present authority remain
distinct. Current Break hosts authorized Manticorp training without moving
Manticorp command out of Flynt. Courthouse transfer into Glaüshouse transfers
clinical care, not courthouse authority.

The Contest of Improvement requires one lawful Minotaur holding the Hecaton
Hand and one lawful Centaur holding the Pegasus Hand,
published baselines, sincere reciprocal teaching, five public trial domains,
auditable crowd judgments, and baseline-relative improvement. Candidate and
insertion order cannot choose the winner. A tie leaves the office unresolved;
teaching sabotage invalidates the responsible evidence or candidate.

**The Sandman = Demiurge.** The winner alone undergoes lawful maintained Sandman convergence and may
receive the one active sovereign mantle. The losing guardian keeps their Form
and mantle unless separately removed. Aegon is the historical name for the
same sovereign now called the Sandman; Aegaeon is not canonical.

### First Stonebend foundation milestone status — legacy physical boundary

`STONEBEND_AURA_WAY_AETHER_HOLLOWING_FOUNDATION_V1.md` and
`src/world/stonebend/foundation.rs` establish the bounded first Stonebend pass
without completing government, succession, Title lifecycle, Freemason
structure, or the full Illegal Hollowing code.

The following vocabulary is authoritative only for legacy physical material
mechanics and archive compatibility; it does not define Practitioner Current or
Aura. Mt. Aura is Aether and Riptide is Bathos. Legacy aura-medium is Aether revealed through Form;
legacy current-medium is Bathos embodied through weight. Aether is weightless current-medium and
legacy current-medium is the heaviest Aether: opposite burden states of one medium, neither
automatically good nor evil. Riptide remains the Flynt–Glaüshouse `Retrieve`
route; no House owns the summit or the downward pole.

Aura Way is the ordinary recognized route through explicit prerequisites,
education, supervised practice, examination, demonstrated responsibility, and
recognition eligibility. Stage order is semantic. Houses teach the work, Aura
Way organizes the path, and Stonebend names completion without declaring
perfection.

Lawful material Hollowing carries stable source, result, common-lineage,
fraction, authority, proof, provenance, custody, and Seal records. Essential
fractions must remain. Unauthorized extraction, essential-fraction removal,
scope excess, over-Hollowing, and false provenance remain distinct from honest
authorized failure and are recognized as Illegal Hollowing where applicable.

Geographic formation produces distinct preserved stone profiles. The same
proofed Aether may reveal variable, concentrating, or resonant Aura through
opal, diamond, or quartz examples. No final House stone, public numeric proof
scale, required melting process, synthetic recast material, guild, currency,
index, or market instrument is introduced.

### Second Stonebend constitutional milestone status

`STONEBEND_THREE_GATES_OFFICES_AND_TITLE_SCOPE_V1.md` and
`src/world/stonebend/second_pass.rs` establish the bounded second Stonebend
pass. Stonebend has three bidirectional gates: Flynt-facing through Stairway to
Heaven and Basin Motor Speedway, Central Junction-facing through the Craft Corridor,
and Sandmanor-facing through Aura Way and Mt. Aura. Central Junction is not a
House, Mt. Aura is not itself a gate, and Riptide remains in its existing
geography.

One stable core Title carries independent Formation Recognition, Public
Circulation, and Operational Deployment scopes. Scope limitation or failure
does not silently destroy the Title or another gate's recognition. Other
Houses and Central Junction retain their domain evidence and operational
authority.

Hypergiant is Stonebend's sovereign title. Diamond's current role is
**STATUS: OPEN**; former Diamond sovereign semantics and stable identifiers are
LEGACY / SUPERSEDED compatibility data. The Freemason forges the Claim, the Hypergiant bears the Title,
and the Proliteriate shields the Yield. The Proliteriate is a distributed
network of stable nodes whose temporary witnesses carry bounded, recallable
mandates. Spartacus remains an archetype, not a fourth sovereign office.

One power may challenge. The other two distinct powers must concur to remove,
and duplicate endorsements never count twice. A corrupted representation may
end, but the Proliteriate network and the people cannot be abolished. Ended
tenures and representations link to Tombstones.

No practitioner accedes as Hypergiant without the full stable Claim, independent
Freemason examination, Proliteriate Yield hearing, relinquishment, consequence
descent, Flynt or Basin Proof of Persistence, Lazerhorn climb, eligibility, and
investiture path. Recommendation and lineage do not appoint a successor; a
former bearer receives no shortened return.

### Third Stonebend constitutional milestone status

`STONEBEND_TITLE_LIFECYCLE_AND_CONSTITUTIONAL_CONTINUITY_V1.md` and
`src/world/stonebend/third_pass.rs` establish the shared Title lifecycle without
replacing the common Bond aggregate. Recognition and activation are separate;
maintenance and renewal are separate; interventions target a scope,
activation, license, tenure, mandate, or explicit core review.

Restoration preserves the interruption. Every ended active term links to a
Tombstone. During Hypergiant vacancy, bounded continuity mandates may preserve
existing routine or emergency duties but cannot exercise or appoint the
Hypergiant, or become permanent by continued use.

High Freemason replacement requires an independently examined Claim,
Proliteriate Yield accountability, and Hypergiant-boundary recognition when a
Hypergiant is active. No candidate may self-certify and the outgoing bearer
cannot appoint a replacement. Proliteriate witnesses remain temporary while
the distributed network persists.

### Service Tournament shared-Function status

The Service Tournament is the largest public Function held at Central
Junction. Under
`SERVICE_TOURNAMENT_CENTRAL_JUNCTION_CANON_V1.md`, Central Junction may become
the temporary shared operational arena of Flynt, Stonebend, Sandmanor, and
Glaüshouse for nonlethal simulations, including the War of a Thousand Hues.

Each House enters as one complete paired-service cultural-government identity:
Flynt, MI as ATF & Army; Stonebend as DEA & Air Force; Sandmanor as CIA &
Navy; and Glaüshouse as FBI & Marines. These are external reference models,
not new Hollow Grove institutions or eight independent factions. Every House
act remains subject to that House's constitution and to shared Function law.

Tournament honor, ceremonial titles, bounded stewardship, temporary
operational responsibility, custody of a standard or cup, first-mark rights,
and public recognition may be awarded. Victory never transfers permanent
sovereignty, changes House succession, makes Central Junction a House, or
allows one House to govern another. The Tournament tests which House can
remain fully itself while serving the whole of Hollow Grove.

### Service Tournament archive and prize custody

`SERVICE_TOURNAMENT_ARCHIVE_AND_CANONICAL_YEAR_FIXTURE_V1.md` freezes the
`HGSTA` archive, migration, replay, golden annual fixture, and dedicated audit
under this existing shared-Function authority. Tournament history may preserve
years, scenarios, paint layers, Service Marks, emergencies, scores,
violations, results, prizes, and House Synthesis semantic events, but an
archive is evidence and continuity—not a new constitutional source.

The Edge of Tomorrow and Glass of a Thousand Hues are coequal existing
completed Syntheses at the apex of legacy physical Current/Aura prize lineages.
This archive classification does not make them Level 1 Practitioner equations. Their
stable identities survive lawful refinement. Temporary custody is not
ownership, office, accession, or sovereignty. The Edge and Glass may have
different custodians, and neither custodian gains permanent control of Central
Junction or another House.

House Synthesis meaning remains above the universal recursion kernel. Aim may
record an inward House operation; only an accepted Kiss may apply an outward
manifestation; Point² records the completed evidence-bearing relationship.
Miss and client presentation never create a manifestation.

### Function Junction, seasonal synchronization, and Permanence status

`FUNCTION_JUNCTION_SEASONAL_WORLD_CYCLE_AND_PERMANENCE_V1.md` establishes
the exact kernel-facing Hollow Grove phase grammar as **The Way Back → The
Initiation → The Gathering → The Festival → The Way Back**. These four phases
sit above the House-neutral universal recursion kernel, repeat at every lawful
scale, and retain the prior confirmed result as the next Way Back input.
Failure is witnessed at The Festival without falsely changing canonical state.
Detailed Aim, Fire, Miss, Kiss, witness, record, transform, and Point²
operations remain subordinate evidence-bearing steps.

The same specification establishes
Function Junction as the constitutional handoff performed by each Great
Function at Central Junction. Function Junction is not a fifth Function or a
geographic place. Winter closes Flynt and opens Glaüshouse; spring closes
Glaüshouse and opens Stonebend; summer closes Stonebend and opens Sandmanor;
autumn closes Sandmanor and opens Flynt. The physical and digital checkpoints
of each handoff remain separate evidence-bearing records.

The Great Functions and their Seasons preserve this Compromise and all four
House constitutions. Derrick remains an alias of The Gathering. The Service
Tournament remains nested inside the broader Summer Function. Neither a
Function Junction, House Season, Great Function, Tournament result, artifact
custody, nor archive record transfers sovereignty.

Stonebend always enforces the Laws of Permanence. A final Permanence Seal
requires Stonebend Identity, Sandmanor Pattern, Glaüshouse Integrity, and Flynt
Capacity proofs for the same stable subject. The Freemason forges Claim;
The Hypergiant authorizes final Title and Seal; Diamond remains STATUS: OPEN; the
Proliteriate protects Yield. No other House may independently declare a
temporary state permanent.

Permanence is not immutability. Lawful amendment, succession, and dissolution
are explicit, witnessed, versioned, and provenance-preserving. Dissolution
creates a Tombstone rather than silent deletion. Unauthorized alteration of a
permanent identity, boundary, Title, record, interior, provenance, or state
remains Illegal Hollowing.
