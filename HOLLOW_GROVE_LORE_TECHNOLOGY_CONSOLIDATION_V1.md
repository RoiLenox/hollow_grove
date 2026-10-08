# Hollow Grove Lore, Technology, and Project-State Consolidation V1

Status: additive canon integration, design-state registry, and implementation
audit

Date: 2026-09-03

Authority boundary: this document interprets the new spiral, technological,
weapon, and professional vocabulary beneath the Hollow Grove Compromise and
the four House constitutions. It does not amend a locked constitutional office,
House order, Form identity, route, Recipe, or runtime identifier by implication.
Where the new vocabulary conflicts with a stronger existing lock, the conflict
is recorded here and the implementation remains unchanged until a named
migration or amendment resolves it.

Status words in this document are exact:

- **LOCKED / CANON** — safe to depend on as setting or design law;
- **STRONGLY ESTABLISHED** — the direction is accepted, but its complete
  mechanics, institutional expression, or implementation contract is not;
- **WORKING / UNRESOLVED** — useful design material that must not yet create a
  wire identity, mandatory asset, final Recipe, office, or progression rule.

## 1. Consolidated Project State

Hollow Grove is currently several related products and authority layers rather
than one finished game loop.

### Documented and enforced foundations

- The Hollow Grove Compromise is the shared Constitution. Four ratified House
  constitutions preserve different authority geometries rather than one common
  faction template.
- The Rust workspace contains the main `hollow-grove` crate, the separate
  `flynt-constitution` authority crate, a neutral recursion kernel, a neutral
  semantic kernel, and the `hollow-niri` presentation/desktop integration.
- The constitutional runtime has typed House decisions, Bond lifecycle,
  evidence, persistence, migration, replay, regional Synthesis, institutional
  state, routes, spatial inheritance, and functional-lore records.
- Existing playable slices include Guide entry and continuity, Hueman movement,
  the constitutional route network, regional encounters, the Boardwalk return
  case, Deep Pressure, party recruitment, refusal/recovery, save/load, and
  replay. These are bounded vertical slices, not yet a general combat or
  equipment system.
- The Hollow Deck schema is `1.6.0`. It indexes Houses, Dominions, Natures,
  Primary Traits, Current Forms, Aura Sets, ordinary Recipes, Synthesis
  Recipes, Syntheses, Banes, and House-specific institutional projections. Its
  checksummed player-state overlay persists cultivation, growth, Aura moves,
  discovered Recipes, and known Syntheses without mutating the catalog.
- A terminal Deck browser and integrated CLI exist. A native winit/wgpu client
  is present as a Phase II prototype with a test world, Hueman prototype atlas,
  movement, collision, camera, integer-aware scaling, and renderer-neutral
  presentation state. It is not yet the full constitutional world client.
- The art bible already locks the Pokémon Yellow visual-discipline adaptation:
  16×16-minded overworld readability, 8×8-minded tile logic, separate
  overworld/Deck/encounter/optional portrait assets, bounded palette and
  animation, cumulative transformation readability, and Hueman then Gargoyle
  as the initial visual proof.

### Major incomplete areas

- General combat and encounters, equipment, broad character progression, and
  authoritative general-purpose Synthesis decisions remain deferred.
- Most Nature-specific Current Form growth paths and Aura Sets are catalog
  shells. Gargoyle is the most developed biological/visual slice.
- The four technological languages added by this pass are documented design
  grammar, not yet catalog fields or crafting simulation.
- Bolo, Crossbow, and Harpoon are now canonical Flynt weapon doctrines, but no
  weapon inventory, animation, combat reducer, or integrated higher weapon has
  been implemented.
- Doctrine has no runtime type, by design at this stage. It must not be added as
  another numerical tree before its narrative and procedural use is proven.
- Stonebend has a mature constitutional structure and a clear physical
  vocabulary, but its bottom-up signature object/weapon progression is still
  undesigned.
- Sandmanor's Motion specialty and Flynt's Reflection specialty need exact
  institutional and gameplay expressions.
- Chromacord exists as a Glaüshouse clinical record institution/application;
  the broader lore term **Chromacord / CC** that grounds CordPunk still needs a
  bounded definition.
- The former Flynt-apex and Sandmanor-Hand Deck conflicts are resolved by the
  ratified 2026-09-03 amendment. Stable identifiers are preserved, while their
  semantic projections and validators follow the amended order and Hand sides.

The historical integration record states that `cargo test --workspace --offline` passed and
`cargo test --workspace --offline -- --list` reports 1,514 tests. `hollow_deck
validate` also passes its internal schema checks and reports 4 Houses,
5 Dominions, 32 Natures, 32 Primary Traits, 5 paths, 4 Flynt pairs, and 8
cross-House counterparts. The 2026-10-07 implementation in
`docs/LORE_KERNEL_OPTIMIZATION_V1.md` corrects the still-reversed Deck seed and
validator to the amended Flynt order. The historical passing test count did
not prove that conformance: those tests also asserted the superseded order.
Sandmanor Hand culminations retain their separate investiture and Form axes.

## 2. Central Spiral Model

### Locked image and law

The central progression image is a revolving staircase with circular landings:

> **From above, it looks circular. From the side, it’s actually ascending.**

Recurrence is therefore not reset. A completed revolution returns to a known
principle with more capacity, consequence, integration, and range. This is
compatible with the locked Point² distinction between angular position and
radial ring: a wrap can complete a circle without ascent, while a lawful
Point² transition can open the next ring without changing the center.

The canonical cosmological law phrase is:

> **Hollow holds.**
>
> **Current flows.**
>
> **Abyss feels.**
>
> **Aura reveals.**

This phrase does not erase the historically scoped phrase `Hollow Current
holds` in the legacy physical Point² doctrine. `Hollow holds` is the current
cosmological wording; `Hollow Current holds` remains valid only inside that
named compatibility model.

### Working developmental circuit

The following is **WORKING / UNRESOLVED** as an exchange and development
sequence:

```text
Stonebend → Sandmanor → Flynt → Glaüshouse → Stonebend at a higher level
```

Its approximate phase verbs are:

```text
Solidify → Flow → Reflect → Reveal → Solidify at a higher order
```

These verbs are lenses, not replacements for either the cosmological law above
or the locked constitutional acts:

```text
Stonebend names → Sandmanor proves → Glaüshouse clears → Flynt recognizes
```

The existing Rule of Twelve also retains its locked local order:

```text
Stonebend → Sandmanor → Glaüshouse → Flynt
```

The sequences describe different axes. Rule-of-Twelve order governs the
existing position grammar; Name/Prove/Clear/Recognize governs constitutional
handoff; the new Stonebend/Sandmanor/Flynt/Glaüshouse circuit is a working
technology-development model. None silently reorders another.

## 3. Four-House Reference Matrix

| House | Punk Language | Fundamental Specialty | Core Verb | Major Material/Aura Association | Important Institutional Figures | Current Design Focus |
|---|---|---|---|---|---|---|
| Stonebend | **PrimePunk** | **Hollowing**; material structure, subtraction, and useful absence | **Names** in constitutional law; **Solidify** in the working spiral | Hollow Current, Diamond, Body, Presynce, The Cut | Hypergiant, Proliteriate, High Freemason/Freemason, Geralds | Design practical PrimePunk objects and Synthesis-weapon realization from the eight-stage Nature/trait vocabulary; allow the final meaning of Hollowing to emerge afterward |
| Sandmanor | **MachPunk** | **Motion** and force transfer, strongly established but not yet an exclusive constitutional monopoly | **Proves** in constitutional law; **Flow** in the working spiral | Regular Current, Crystal, Soul, Prefog and Prefig | The Sandman/Demiurge, Hecaton Right Hand, Pegasus Left Hand, Geomancer, Labyrinthian, Minorian and Minoan traditions | Define how the two cultures teach, test, certify, and govern motion without merging them; preserve The Sandman above both perfected Hands |
| Flynt | **PetPunk** | **Reflection**, response, and retained pattern as an emerging philosophy beneath the locked Law of Capacity | **Recognizes** in constitutional law; **Reflect** in the working spiral | Regular Current, Opal, Spirit, Resynce, Capacity | The Tross/Manticore, Chimera, Mystery Men, Bro White, Beauty, Cinderellaman, Manticorp | Develop Reflection and the exact Manticorp combined and Chimera perfected mechanisms across the three independent tracks |
| Glaüshouse | **CordPunk** | **Synthesis**, relationship, composition, compatibility, and lawful transformation | **Clears** in constitutional law; **Reveal** in the working spiral | Hollow Current, Jade/Emerald, Mind, Precog, Biokinesis/Noomancy | Prima Donna, Persephones, Nightingales, Matrons, Marshals, Chromacord, Glauspitals | Define Chromacord/CC beyond its clinical application and express Synthesis as tuned multi-disciplinary composition |

The Punk Language names and their core constraints are **LOCKED / CANON** as
House design languages. The one-word Fundamental Specialty summaries are
organizing principles, not replacements for House Domain, Law, faculty,
constitutional verb, or existing institutional authority.

## 4. Four Technological Languages

### Stonebend — PrimePunk

**Status: LOCKED / CANON design language; individual devices remain WORKING.**

PrimePunk engineers from fundamental physical principles and continually
redesigns toward better realization. Its vocabulary includes force, leverage,
pressure, tension, compression, counterweights, springs, gearing, hydraulics,
pneumatics, structural geometry, heat, inertia, material science, mechanical
advantage, and precision tolerance.

> **If a mechanism is visible, it should perform or communicate a real
> function.**

> **PrimePunk preserves the principle, not the casing.**

Marks, patterns, generations, and configurations may change while the
functional identity stays recognizable. A Steam-era or early-industrial echo
is permitted, but decorative gears, arbitrary pipes, and machinery without
load, transfer, control, or information are not.

PrimePunk is:

> **Primal laws. Perfect engineering. Constant redesign.**

### Sandmanor — MachPunk

**Status: LOCKED / CANON design language; institutional monopoly remains
WORKING.**

MachPunk is the science and culture of movement and force transfer, not merely
fast vehicles. It covers speed, acceleration, traction, torque, leverage, load
transfer, balance, agility, momentum, velocity, aerodynamics, lift, mechanical
transfer, suspension, gyroscopic behavior, and flight.

Its two cultures remain distinct:

```text
Minorian / Prefog
Gnome → Faun → Minotaur → Hecaton
Grip → Traction → Leverage → Multiplication
Hecaton: perfected force / Right Hand of The Sandman

Minoan / Prefig
Elf → Satyr → Centaur → Pegasus
Balance → Agility → Momentum → Flight
Pegasus: perfected motion / Left Hand of The Sandman
```

The four-part lines are **LOCKED complete branch progressions**. Their first
three identities remain biological Forms; their fourth identities are
separately invested perfected Hands. The Sandman is command of both and the
final sovereign culmination above them, not a fifth biological Form.

### Flynt — PetPunk

**Status: LOCKED / CANON design language; process details remain WORKING.**

`Pet` derives from petroleum/petrol. PetPunk imagines petroleum technology
refined toward clean, controlled, captured, and near-closed-loop operation. Its
usable vocabulary includes highly refined hydrocarbons, fuels, lubricants,
solvents, polymers, sealants, hydraulic and pressure fluids, precision
injection, catalytic processing, capture, and recirculation.

The visual and material character is sleek rather than sooty: dark void-blue
fluids, white or opalescent effects, transparent or deliberately routed fluid
paths, sealed mechanisms, immaculate pressure vessels, reflective surfaces,
and precise valves or injectors.

> **PetPunk is petroleum perfected, not crude-oil industrialism.**

PetPunk extends Flynt's Regular Current and Opal language without declaring
petroleum a new universal Current, Aura Material, exclusive fuel, or moral
alignment.

### Glaüshouse — CordPunk

**Status: LOCKED / CANON design language; Chromacord metaphysics remain
WORKING.**

CordPunk arises from Chromacord/CC and its deliberate fashion resonance with
Coco Chanel. It is elegant, chromatic, compositional, tuned, and synthetic.
Its vocabulary includes synthesis, resonance, harmony, color, optics,
acoustics, projection, medicine and restoration, wearable technology, couture
engineering, patterned interfaces, emerald materials, prisms, resonators, and
carefully composed systems.

Its deeper rule is:

> **Function emerges from the relationship between components.**

CordPunk is not merely fashion technology. Fashion is one cultural proof of
composition, proportion, interface, and relationship. `Chromacord` already
names a clinical record institution and `chroma_cord` its attached application;
the broader CC principle must be formally defined without causing the software
to manufacture consent, Clearance, or Synthesis authority.

### Art-bible boundary

These languages are semantic filters, not instructions to increase sprite
detail. The existing asset-class split, silhouette rules, palette budgets,
animation economy, and tiny-size readability remain controlling. A PrimePunk
machine may need a large encounter or civic-object asset; its complete mechanism
does not have to be squeezed into a 16×16 actor. At overworld scale, select the
one part that communicates function.

## 5. Interdependence and the Civilizational Technology Spiral

**Status: STRONGLY ESTABLISHED civilizational grammar.**

The Houses are specialized parts of one civilization. Their knowledge and
constitutional acts remain distinct precisely so that another House's work is
genuinely necessary.

One possible advanced-development cycle is:

1. Stonebend realizes a system in matter and proves that its structure can
   hold.
2. Sandmanor improves motion, load transfer, balance, efficiency, or dynamics
   and tests the claim.
3. Flynt refines fluids, coatings, fuels, pressure behavior, response, or
   reflective behavior and demonstrates Capacity under load.
4. Glaüshouse composes the relationships into a coherent, compatible, and
   maintainable whole.
5. Stonebend realizes the next Mark or generation without erasing the earlier
   device's provenance.

This is not a mandatory production checklist for every knife, road, medicine,
or vehicle. It is a grammar for advanced exchange and a reason PrimePunk forms
can retain identity through constant redesign.

The legal handoff remains more exact than this design sketch. A Synthesis
Weapon can require, for example:

```text
Glaüshouse Synthesis knowledge and Clearance
+ a specialized Recipe and lawful custodian
+ Sandmanor proof of the design and method
+ Freemason physical realization, provenance, and refusal discipline
+ Flynt recognition of demonstrated function
```

No participant's contribution grants every other participant's authority.

## 6. House Specialties

### Stonebend — Hollowing

**Status: LOCKED domain and process; terminal metaphysical definition remains
WORKING.**

Stonebend owns the constitutional discipline of lawful Hollowing: opening,
exposure, excavation, extraction, draining, separation, severing, inspection,
preservation, and removal while maintaining identity, scope, provenance,
custody, restoration, and Seal. Hollowing is not Synthesis.

The practical developmental vocabulary is strongly established:

```text
Scavenge → Rig → Devour → Phase → Regenerate → Sunder → Crush → Hollow
```

It corresponds to the locked Deck Nature/Primary Trait path:

```text
Gremlin      → Goblin  → Ghoul     → Spectre → Troll
Scavenging   → Rigging → Devouring → Phasing → Regeneration

→ Orc      → Ogre     → Troglodyte
  Sundering → Crushing → Hollowing
```

`Orc` is the active Deck spelling; `Ork` remains a stable archive alias outside
the Deck. The path is biological/Nature progression and must not be collapsed
with the Freemason, Proliteriate, or Hypergiant office and mastery axes.

> **The Hollow isn't merely nothing. The Hollow is room for something.**

That sentence is a **STRONGLY ESTABLISHED** design direction, not yet a
complete metaphysics. Stonebend's objects should prove when absence supports
use, motion, shelter, access, repair, fit, or future assembly before the final
definition is ratified.

### Sandmanor — Motion

**Status: STRONGLY ESTABLISHED direction; institutional expression
WORKING.**

Motion includes force transfer, traction, leverage, torque, momentum, balance,
acceleration, aerodynamics, and flight. It complements rather than replaces
Sandmanor's locked authority over Design, Method, Measurement, Proof,
Criticism, Failure, and Revision. The open question is which schools, guilds,
standards, licenses, Contests, or public works make Sandmanor's mastery of
motion indispensable to the other Houses.

### Flynt — Reflection

**Status: STRONGLY ESTABLISHED direction; deeper philosophy WORKING.**

`Reflects` is preferred to `remembers` or `recognizes` when naming the emerging
physical and metaphysical principle. Memory, recognition, tracing, and response
can be consequences of reflection. Reflection connects Opal, Spirit, identity,
retained pattern, response, and Resynce.

Reflection does not replace Flynt's locked Law of Capacity, constitutional
recognition, or Steel Temper. A useful developing relation is: a system
reflects what pressure reveals, and Flynt judges what that reflected behavior
can actually do under load.

### Glaüshouse — Synthesis

**Status: LOCKED constitutional authority and STRONGLY ESTABLISHED
civilizational specialty.**

Glaüshouse is the civilization's deepest authority on the compatibility,
relationship, care, transformation, Continuance, recovery, and composition
that make Synthesis possible. Existing law already assigns it consent,
Clearance, final clinical Synthesis, maintained-Form care, and the Synthesis
Ledger.

This is not ownership of every Recipe. Recipes may be discovered, authored,
divided, inherited, hidden, specialized, or held elsewhere. Glaüshouse's
knowledge explains whether components and a subject can coherently become and
remain something new; it does not erase the Recipe holder, designer,
fabricator, operator, or recognizing institution.

## 7. Universal Progression: Form, Set, and Doctrine

**Status: LOCKED distinction between Current Form and Aura Set; Doctrine is
STRONGLY ESTABLISHED and mechanically OPEN.**

The Hollow Deck's qualified terms remain:

- **Current Form** — what the practitioner's body has become;
- **Aura Set** — the supernatural techniques the practitioner has learned and
  may equip;
- **Doctrine** — how the practitioner understands, selects, restrains, and
  integrates what has been learned.

`Current Form` must remain qualified. Unqualified **Current** in the universal
Practitioner ontology describes relational participation in a larger whole.
The Deck's `CurrentForm` body record and universal Practitioner Current are not
synonyms and need no destructive rename.

Current Form and Aura Set remain independently cultivable. Doctrine is not a
third stat bar, a second Aura Set, a level bonus, or an excuse to advance both
axes together. It is a coherent Way visible in judgment, timing, posture,
restraint, conduct, maintenance, and execution.

Collectible Synthesis Recipes can bridge the axes without merging them. A
Recipe records what relationship may produce a result; it does not by itself
grant the body, learned technique, Doctrine, consent, Clearance, Title, proof,
or recognition.

Final forms should read as final because their practices have become coherent,
not only because their numerical budget is larger.

## 8. Flynt Weapon and Synthesis Matrix

### Isolated Mystery Men weapon doctrines

**Status: LOCKED / CANON mappings; implementation not yet present.**

| Nature | Mystery Men sect | Underlying discipline | Signature weapon | Preserved function |
|---|---|---|---|---|
| Gargoyle | Hunter | Stillness / Fortification | **Bolo** | restraint, entanglement, tripping, binding, and arresting motion |
| Werewolf | Chief | Circulation / Siphone | **Crossbow** | prepared ranged force during stalking, tracking, climbing, movement, and pursuit |
| Mariner | Mariner | Position / Anchoring | **Harpoon** | ranged connection, tethering, anchoring, reeling, dragging, and control of relative position |

`Hunter` remains the Gargoyle-aligned formal sect. The Werewolf's hunter and
pursuit character is descriptive and does not rename the canonical Werewolf
sect `Chief`.

The weapons are embodied disciplines, not arbitrary loot, Nature anatomy,
automatic office grants, or three cosmetic skins for the same attack.

### Three independent synthesis tracks

**Status: LOCKED architecture and tier meanings; exact Recipes and mechanics
remain WORKING.**

| Track | Inputs | Intended result | Independent upgrade rule |
|---|---|---|---|
| **Current Synthesis Recipe** | relevant Current Form/body principles of Gargoyle, Werewolf, and Mariner | an integrated bodily relationship that preserves the three source functions without erasing the Hueman or source provenance | may refine without automatically refining Aura or Weapon |
| **Aura Synthesis Recipe** | relevant learned Aura principles and compatible techniques | an integrated learned/expressive system guided by one coherent Doctrine | may refine without automatically refining Current or Weapon |
| **Weapon Synthesis Recipe** | Bolo + Crossbow + Harpoon disciplines | one engineered system preserving restraint, ranged projection, and tethering/anchoring | may refine without automatically refining Current or Aura |

Each track advances from isolated discipline to **Manticorp / combined
synthesis**, then to **Chimera / perfected synthesis**. Mixed states are
therefore permitted: one track may be more mature than another. Exact
eligibility, compatibility, failure, recovery, maintenance, and UI rules remain
open.

The integrated weapon must not look like three complete weapons bolted
together. Its final silhouette and mechanism are **WORKING / UNRESOLVED**. The
Freemason realization pass should begin only after the functional Recipe and
safe operating sequence are clear.

### Manticorp and Chimera amendment resolution

The ratified synthesis order is:

```text
Gargoyle + Werewolf + Mariner/Merman → Manticorp → Chimera
```

**Manticorp is combined synthesis. Chimera is perfected synthesis.** Neither is
an additional isolated Mystery Men school. The order applies independently to
Current, Aura, and Weapon Synthesis.

Manticorp also remains Flynt's military institution, now understood as the
institutional expression and training body of combined synthesis. Membership
does not grant the state. Manticore remains the Tross's established masculine
mythological identity and a stable compatibility Form/Recipe namespace; it is
not a third general synthesis tier after Chimera. The Deck's stable
`manticorp` and `chimera` Nature IDs are preserved, but its directional mastery
now follows the amended order.

The Bushido comparison is retained as a **WORKING Doctrine analogy**: the
higher tier is where techniques become a Way of judgment, timing, posture,
restraint, conduct, and execution. It creates no Japanese ethnicity, costume,
rank, terminology, or visual requirement for Flynt.

### The Tross, Recipes, and Glaüshouse

**Status: STRONGLY ESTABLISHED division of labor; exact Recipe inventory
WORKING.**

The Tross is a synthesizer, sovereign governor of the Flynt Synthesis
ecosystem, custodian/holder of specialized Flynt Recipes, and an authority able
to enable advanced Mystery Men synthesis. Glaüshouse remains the deeper
authority on the science, care, compatibility, and Continuance of Synthesis
itself. The exact named inventory of Tross-held Recipes remains open.

The distinction is:

> **Glaüshouse understands Synthesis. Specialized holders preserve Recipes.**

For the legacy record called the Manticore Recipe specifically, canon is
narrower than saying the Tross owns or holds the complete Recipe. Its
indispensable fragments are divided among Bro White, Beauty, and Cinderellaman;
no person or institution holds the complete reproducible Recipe. The record
preserves Tross Continuance and combined/perfected lineage provenance without
restoring the superseded synthesis order. Any additional Recipe personally
held by the Tross must be named later rather than inferred.

## 9. Stonebend Development Board

### Established professional and civic expressions

The terms below already carry locked constitutional meanings. Their developing
PrimePunk expressions add practical and cultural focus without replacing that
law.

| Identity | Locked constitutional/mastery meaning | Developing PrimePunk expression | Guardrail | Status of new expression |
|---|---|---|---|---|
| **Proliteriate** | permanent distributed public network shielding Yield; also the Deck's Pugilist + Foreman mastery register | buildings, architecture, fortifications, roads, tunnels, arenas, cranes, elevators, waterworks, ventilation, rotating structures, and large civic machinery; Hollowing applied to space | not one architect, chamber, monarch, or exclusive owner of all construction | **STRONGLY ESTABLISHED** |
| **Freemason** | singular Claim/craft/legitimacy power led by the High Freemason; structural, defensive, sealing, surveying, custody, execution, and controlled-weapon authority; also Mason + Mage mastery register | supreme physical realization of a true Synthesis Weapon through machining, gunsmithing, mechanical engineering, material science, tolerance, provenance, and refusal discipline | ordinary craftspeople may make ordinary weapons; a Recipe is not physical realization; the Freemason does not gain Synthesis science, clinical Clearance, or Recipe ownership | **STRONGLY ESTABLISHED** |
| **Hypergiant** | Stonebend's singular sovereign Title and integration of the two Deck mastery registers | public spectacle: demonstrations, competitions, ceremonies, reveals, engineering exhibitions, arenas, and civic events that make Stonebend achievement visible | not merely the strongest Stonebend person, an entertainer-only office, or a larger Ogre | **STRONGLY ESTABLISHED** |

The working cultural relation is:

> **The Proliteriate builds the stage. The Freemason builds the extraordinary
> object. The Hypergiant makes it legendary.**

A **cane gun** is a **WORKING / UNRESOLVED** Hypergiant signature direction. It
is attractive because ceremony, fashion, concealment, mechanical precision,
and theatrical reveal meet in one object, but no final design, lawful use,
Recipe, or asset requirement is locked.

### Bottom-up object and weapon board

**Status: WORKING / UNRESOLVED. No final object names are created here.**

Stonebend objects should cross `tool → instrument → machine → weapon → civic
object` when their function warrants it. Each design must make a real principle
legible.

| Nature | Primary Trait | Practical verb | Principle to prove | Open design question |
|---|---|---|---|---|
| Gremlin | Scavenging | Scavenge | selection, recovery, fit, reuse | What object makes discarded material immediately useful without pretending salvage is manufacture? |
| Goblin | Rigging | Rig | tension, connection, trigger, redirection | What rig teaches that arrangement can create more function than any one part? |
| Ghoul | Devouring | Devour | material conversion, fuel, consumption, endurance | What device makes its consumed input and resulting cost visible? |
| Spectre | Phasing | Phase | boundary, clearance, tolerance, controlled passage | What instrument changes the relation to a boundary without making matter meaningless? |
| Troll | Regeneration | Regenerate | repair, joining, replacement, restored load path | What system shows repair as engineered continuity rather than free healing? |
| Orc | Sundering | Sunder | fracture, separation, exact cut, release | What tool separates at the intended boundary and preserves evidence of the break? |
| Ogre | Crushing | Crush | pressure, impact, compression, structural failure | What machine controls overwhelming force instead of merely depicting strength? |
| Troglodyte | Hollowing | Hollow | excavation, subtraction, useful void, room | What object proves that the removed space—not only the remaining shell—is the useful product? |

Candidate vocabulary includes measure, tension, leverage, pressure,
counterweight, load, balance, impact, structural failure, repair, joining,
separation, removal, and useful void. It is a prompt set, not an arsenal.

## 10. Reconciliation Register

| Apparent conflict | Controlling existing canon | Reconciliation in this pass | Remaining work |
|---|---|---|---|
| New technology circuit orders Flynt before Glaüshouse; Rule of Twelve and the constitutional handoff order Glaüshouse before Flynt | Rule of Twelve; Compromise and House acts | treat the new sequence as a working product-development circuit, not map or legal order | test the circuit against concrete devices before ratifying an exact order |
| `Hollow holds` versus legacy `Hollow Current holds` | Point² legacy physical doctrine retains the latter in scope | current cosmological maxim uses `Hollow holds`; old wording remains a scoped compatibility phrase | propagate only when editing the owning legacy document deliberately |
| `Current Form` means body while universal `Current` means relational participation | Synthesis Canon and Hollow Deck | require the qualified term `Current Form` for body state; no destructive runtime rename | improve UI labels where context is ambiguous |
| Four-part Sandmanor branches versus three biological Forms and separate Hands | 2026-09-03 Hands amendment; Sandmanor Constitution; executable regional law | lock each complete branch as three biological Forms followed by separate perfected-Hand investiture; Hecaton is Right/perfected force, Pegasus is Left/perfected motion, and The Sandman stands above both | exact Hand appointment, removal, and succession mechanics remain open; no Form-enum migration is required |
| Manticorp then Chimera versus older Chimera then Manticore and Deck's reversed projection | 2026-09-03 Flynt synthesis amendment | lock Manticorp as combined synthesis and Chimera as perfected synthesis on each independent track; preserve Tross-specific `manticore` compatibility keys without treating them as a third tier | exact combined and perfected mechanisms remain open; stable IDs require no destructive save migration |
| Tross as Recipe holder versus the divided Manticore Recipe | Flynt Constitution | Tross may govern, synthesize, enable, or hold other named Recipes; Bro White, Beauty, and Cinderellaman retain the complete Manticore Recipe's divided custody | specify which additional Recipes, if any, the Tross personally holds |
| Glaüshouse Synthesis monopoly versus distributed Recipes and fabrication | Glaüshouse law, Recipe Constitution, Sandmanor proof, Stonebend craft, Flynt recognition | monopoly means deepest Synthesis knowledge plus clinical/constitutional Clearance and final procedure, not ownership of all Recipes or fabrication | define cross-House commissioning and commercial custody |
| Proliteriate/Freemason/Hypergiant professional concepts versus constitutional offices and network | Stonebend Constitution | add civic/professional expressions without replacing Claim/Yield/Title geometry | prove them in one Stonebend object project and one public event |
| Punk languages could imply noisy skins that violate the art bible | Art Bible | make every language functional and subordinate it to silhouette, asset role, palette, and animation limits | make one compliant object sheet per House before broad production |
| CordPunk/Chromacord versus the existing `Chromacord` institution and `chroma_cord` application | Glaüshouse Constitution and clinical charting contract | the application remains evidence-only; broader CC principle is not yet allowed to create authority | define Chromacord as lore without collapsing it into software or vague color magic |
| PetPunk fluids versus Current and Aura materials | legacy material law and Hollow Deck alchemy | petroleum-derived media are technologies and materials, not a new cosmic substance or alchemical slot | define sourcing, capture, waste, safety, and closed-loop failure states |
| Werewolf's pursuit/hunter wording could collide with the formal Hunter sect | Flynt Constitution and Deck identity matrix | Werewolf stays in the Chief sect; `hunter` is descriptive only in that sentence | use `pursuit` in schema/UI labels where ambiguity would occur |

## 11. Canon Status Registry

### LOCKED / CANON

- Hollow Grove has four asymmetric, interdependent Houses, not four elemental
  kingdoms with identical ladders.
- “From above, it looks circular. From the side, it’s actually ascending.” is
  the central spiral image.
- “Hollow holds. Current flows. Abyss feels. Aura reveals.” is the canonical
  cosmological phrase, with legacy scoped wording preserved.
- PrimePunk, MachPunk, PetPunk, and CordPunk are the four House technological
  language names and carry the functional constraints defined above.
- The existing House Domains, Laws, faculties, constitutional acts, offices,
  institutions, and route/position orders remain unchanged.
- Current Form and Aura Set are independent. Qualified `Current Form` means
  body; unqualified universal Practitioner Current remains relational.
- Stonebend's Gremlin through Troglodyte Nature path and Scavenging through
  Hollowing Primary Traits remain the active Deck path, with `Ork` preserved
  only as an older compatibility spelling.
- Gargoyle/Hunter uses Bolo, Werewolf/Chief uses Crossbow, and
  Mariner/Mariner uses Harpoon as their isolated weapon doctrines.
- Manticorp and Chimera are not extra isolated weapon specialties equivalent
  to the preceding three.
- Manticorp is combined synthesis; Chimera is perfected synthesis. Current,
  Aura, and Weapon Synthesis reach and refine those states independently.
- Hecaton is perfected force and the Right Hand of The Sandman; Pegasus is
  perfected motion and the Left Hand; The Sandman is command of both and the
  final sovereign culmination above them.
- Glaüshouse holds final clinical Synthesis/Clearance authority and the deeper
  maintained-Form knowledge without owning every Recipe.
- Tross-specific `manticore` and `divided-manticorp` identifiers remain stable
  compatibility records and do not create a third tier after Chimera.
- Pokémon Yellow remains a visual-discipline reference, never a mandate to
  copy assets, hardware, palettes, content, or UI.

### STRONGLY ESTABLISHED

- Stonebend specializes in Hollowing and material realization; Sandmanor in
  Motion; Flynt in Reflection; Glaüshouse in Synthesis and composition.
- “The Hollow isn't merely nothing. The Hollow is room for something.” guides
  Stonebend design without yet completing its metaphysics.
- Advanced technology can circulate through House contributions and return to
  Stonebend as a higher-generation physical realization.
- Doctrine describes integrated understanding and conduct rather than another
  stat tree.
- Proliteriate/buildings, Freemason/true Synthesis Weapon realization, and
  Hypergiant/public spectacle are the current Stonebend professional design
  directions, bounded by their constitutional meanings.
- The Tross governs and enables advanced Flynt synthesis and holds specialized
  Flynt Recipes, while the exact inventory remains open and other specialized
  Recipe custody can remain elsewhere.

### WORKING / UNRESOLVED

- Whether the exact technology-development order must always be Stonebend →
  Sandmanor → Flynt → Glaüshouse → Stonebend.
- Whether Solidify, Flow, Reflect, and Reveal should become formal phase names.
- Sandmanor's Motion monopoly, institutions, certifications, objects, and
  failure doctrine.
- Flynt's complete Reflection philosophy and its relation to memory, identity,
  response, Resynce, and Opal.
- The full definition of Chromacord/CC beyond its existing clinical
  institution and application.
- The exact Manticorp combined weapon mechanism and its operation, failure,
  recovery, and maintenance.
- The exact perfected Chimera weapon form.
- Mixed-state Recipe, eligibility, and UI rules for the three Flynt tracks.
- The precise Current and Aura principles contributed by each Flynt specialty.
- The exact names and contents of the specialized Recipes held by the Tross
  beyond his governance and bearer roles.
- The full Stonebend signature object/weapon progression and every final item
  assigned to a Nature or professional identity.
- Cane gun as a Hypergiant signature object.
- The institutional and economic contract among Glaüshouse Synthesis experts,
  Recipe custodians, Sandmanor proof bodies, Freemason fabricators, and Flynt
  operators/recognizers.
- The final metaphysical meaning of Hollowing.

## 12. Recommended Next Design Work

1. **Design Stonebend's PrimePunk signature objects from the bottom up.** Start
   with Gremlin/Scavenging and Goblin/Rigging. For each, document input,
   mechanism, load path, control, failure, repair, ordinary civic use, weapon
   use if any, and what visible part communicates function at overworld scale.
2. **Advance one object through the eight-stage Stonebend vocabulary.** Do not
   force eight unrelated fantasy weapons. Let a tool become an instrument,
   machine, weapon, or civic object only when the underlying principle changes.
3. **Run the professional handoff.** Have a Proliteriate mandate state the civic
   need and spatial Yield, a Freemason realize the extraordinary object, and a
   Hypergiant demonstration make the achievement publicly legible. Preserve
   the three constitutional roles while testing their new cultural expression.
4. **Create an asset-class proof.** Give the selected object one tiny overworld
   read and one larger Deck/encounter or civic-object view under the existing
   art bible. This will test PrimePunk function without breaking the Pokémon
   Yellow discipline.
5. **Only then define Hollowing more deeply.** Use the practical evidence to
   decide when removal creates useful room, when absence bears structure, and
   when a Hollow remains continuous with what was cut.
6. **Develop Sandmanor Motion and Flynt Reflection through concrete cases.** A
   MachPunk suspension/flight problem and a PetPunk closed-loop pressure system
   will expose useful institutions and failure modes more reliably than a
   top-down one-word metaphysics.
7. **Bound Chromacord/CC.** Specify what a cord relates, how harmony and
   dissonance are observed, what evidence a composition carries, and what the
   existing clinical application can display without gaining authority.

The immediate priority remains Stonebend. Its philosophy should emerge from a
coherent practical system rather than being imposed before the tools, loads,
failures, rooms, and repairs make the meaning visible.

> **From above, it looks circular. From the side, it’s actually ascending.**
