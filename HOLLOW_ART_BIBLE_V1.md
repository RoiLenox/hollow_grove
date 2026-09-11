# Hollow Grove Art and Rendering Bible V1

Status: canonical rendering direction and asset-modeling guidance for Hollow
Deck catalog schema 1.5.0

This document defines what future art and rendering systems must preserve. It
does not implement a graphical client, establish mutable player progression,
or replace the biological, institutional, constitutional, or gameplay models
that supply its inputs.

## 1. Presentation direction

Hollow Grove is:

- modern;
- timeless;
- 2D;
- hand-authored;
- clear.

> Hollow Grove should be built like a modern game that uses Pokémon Yellow's
> visual discipline.

The modern side is Rust, Wayland, wgpu, Aseprite-authored assets, a modern
windowing and rendering pipeline, clean scaling and pixel snapping, flexible
presentation, and renderer support for multiple sprite classes. The
Yellow-discipline side is tiny overworld sprites, limited animation frames,
immediate readability, minimal clutter, strict silhouette logic, distinct
asset roles, strong transformation readability, limited color use per asset,
and compact sheet organization.

Pokémon Yellow is a design-grammar reference, not a technical or content
target. Do not require a 160×144 framebuffer, Game Boy 2bpp, literal ROM
formats, original hardware palettes, four-direction-only movement, fake CRT
effects, or Game Boy UI imitation in every context. The base overworld design
target is nevertheless **16×16 actor logic**: a visual discipline defined in
the next section, not a universal canvas-size or hardware constraint.

No wgpu or Wayland client is introduced by this bible.

## 2. Pokémon Yellow visual grammar adaptation

### What the reference means

“Pokémon Yellow feel” means iconic, compact, collectible, immediately
classifiable, restrained, charming but serious, silhouette-driven, expressive
through a few pixels, readable in the tiny overworld, and supported by
separate larger display assets where needed.

It does not mean copying Pokémon assets or designs, reproducing its sheet
layouts, enforcing every original hardware restriction, preserving an
intentionally archaic user experience, adding nostalgia filters, or refusing
modern rendering tools. The locally supplied `Game Boy _ GBC - Pokemon Yellow`
overworld, battle, compendium, and Pikachu expression sheets are non-shipping
study references. They demonstrate role-specific sheets, systematic pose and
state grouping, compact expression changes, silhouette economy, and bounded
per-asset color. Do not import or redistribute their pixels.

### Canonical asset scale and roles

**Overworld actors.** Use 16×16 actor logic as the base design target. Author
every major actor and Nature to succeed first as a tiny overworld body. If a
later implementation uses a larger canvas, layers, or additional directions,
the result must still read with the decisiveness of a 16×16-era character.
Passing this test is about information density and silhouette, not compulsory
storage dimensions.

**Tile logic.** Keep the underlying visual grammar 8×8-minded. Maps may use
16×16 metatiles or another practical modern layout, but world art must remain
gridded, clean, and readable.

**Larger representations.** Support distinct, deliberately authored asset
classes:

- Overworld Sprite;
- Deck Sprite;
- Encounter / Battle Sprite;
- Expression / Portrait Variant, optional when useful.

These classes are not interchangeable by default. A successful overworld
silhouette is not merely a larger image reduced, and a Deck or encounter image
is not merely the overworld frame enlarged.

### Restriction is a design rule

The first question when making Hollow Grove art is:

> Can this read instantly at tiny size?

The first question is not, “How much detail can we add?” Apply these artistic
constraints even when performance would permit more:

- strong silhouette first;
- one clear idea per sprite;
- minimal noise and ornament overload;
- limited accessories;
- limited palette per asset;
- limited animation frames;
- strong contrast between head, body, and appendages;
- recognizable transformation increments;
- no texture mush or painterly shading at tiny scale.

These are production restrictions, not merely performance optimizations.

### Animation economy

A typical overworld actor set is idle, walk 1, walk 2, and the required
directional or facing frames. Later additions may include transform,
attack/use, hurt, rest, petrified, or another state-specific variant, but no
Nature begins with a bloated animation obligation.

Motion should come primarily from pose clarity, rhythm, timing, silhouette
shifts, and strong limb placement—not many in-between frames, hyper-fluid
animation, excessive secondary motion, or subpixel effects required for
readability.

### Hueman is the baseline body grammar

Hueman is the baseline body standard for Hollow Grove and the first visual
proof before broad Nature production. Its overworld sprite follows the useful
trainer logic in the reference sheets: a readable head shape, simple torso
block, clear leg separation, clear feet or shoe read, one or two defining
accessories at most, and an identifiable silhouette before detail. Express
gender and style variation through shape rather than clutter.

Hueman Deck, encounter, and portrait representations may be more expressive,
but they remain governed by the same restriction philosophy.

### Transformation stages must add one readable fact

Transformations grow through clean, cumulative stages that remain legible at
overworld scale. Gargoyle is the first full proof:

```text
Hueman
  → Fangs
  → Claws
  → Horns
  → Wing Buds
  → Winglets
  → Wings
  → Mature Fortification
  → White Opal Statue Mastery
```

Each step preserves the core body identity, adds one clear new visual fact,
strengthens the silhouette progressively, and avoids premature overdesign.
This follows the readability principle of collectible and evolution games; it
does not copy Pokémon evolution art.

### Canonical Nature asset-family model

Plan each major Nature line as this long-term family:

- **A.** Hueman Overworld Base
- **B.** Hueman Larger / Deck / Encounter Base
- **C.** Nature Overworld Growth Stages
- **D.** Nature Deck Sprite
- **E.** Nature Encounter / Battle Sprite
- **F.** State Variants
- **G.** Transformation / special-use frames as needed

State variants may include petrified, white opal statue, poisoned, sleeping,
fortified, wings spread, and aura-active presentations. Not every Nature needs
every asset immediately; this model defines where an asset belongs when its
need is proven.

### House color and material under restriction

Each sprite should usually rely on a dark outline foundation, two to three
major local colors, one accent, and one restrained highlight where needed.
House identity must survive within that budget:

- **Flynt:** void-blue base, opal appendages or accessories, restrained
  highlights, and a dark, dense, fortified read;
- **Stonebend:** Hollow Current clarity, diamond-like hard or light accents,
  and a structural, material read;
- **Sandmanor:** Crystal, sky, aquamarine, and blue-white openness, with
  movement, possibility, and coastal brightness;
- **Glaüshouse:** a Jade/Emerald branch split with a clinical, refined, and
  restorative read.

House palettes establish immediate identity without forcing busy multicolor
sprites. Material language should be carried by a few decisive shapes and
values before extra hue or surface texture is considered.

### House technological-language overlay

The four canonical technological design languages are functional semantics,
not permission to add visual noise:

- **Stonebend / PrimePunk:** force, leverage, pressure, tension, compression,
  counterweight, springs, gearing, hydraulics, pneumatics, structural geometry,
  heat, inertia, material science, and precision tolerances. Every visible
  mechanism must perform or communicate a real function. PrimePunk preserves
  the principle rather than the casing, so later Marks may be newly engineered
  while retaining recognizable functional identity.
- **Sandmanor / MachPunk:** movement and force transfer through traction,
  torque, leverage, load transfer, balance, agility, momentum, velocity,
  aerodynamics, lift, suspension, gyroscopic behavior, and flight. It is not a
  generic fast-vehicle skin.
- **Flynt / PetPunk:** petroleum-derived fuels, lubricants, solvents, polymers,
  sealants, hydraulic/pressure fluids, precision injection, capture,
  recirculation, and catalytic processing refined toward clean, controlled,
  near-closed-loop operation. Prefer dark void-blue fluids, opalescent effects,
  sealed vessels, transparent or deliberately routed lines, reflective
  surfaces, and precise valves over soot and crude-oil grime.
- **Glaüshouse / CordPunk:** Chromacord/CC, synthesis, resonance, harmony,
  optics, acoustics, projection, medicine, wearable systems, couture
  engineering, prisms, resonators, emerald materials, and composed interfaces.
  Function should read as emerging from the relationship among parts. Fashion
  is one expression of composition, not the whole technology.

At overworld scale, show the single shape, joint, vessel, cord, control, or
motion cue that explains the object. Large machinery, civic structures, and
integrated weapons may use Deck, encounter, environmental, or dedicated civic
object assets rather than violating 16×16 actor readability. These languages
do not change the asset classes, palettes, production order, or rule that one
clear idea comes first.

### Modern implementation interpretation

The implementation may be modern and flexible while the art behaves as though
handheld-era constraints matter. Prefer crisp integer-aware scaling, pixel
snapping, authored atlases, clean sprite batching, distinct asset categories,
and a logical presentation around 640×360 or a similar practical modern
equivalent. Use a modern aspect ratio. Optional future shaders and effects are
acceptable only when sprite readability survives without them.

This guidance does not prescribe the final renderer architecture, atlas
format, camera, direction count, or hard output resolution.

### Production order

Use this production order:

1. Hueman overworld base.
2. Hueman larger / Deck / encounter base.
3. Gargoyle overworld growth stages.
4. Gargoyle larger / Deck / encounter assets.
5. Gargoyle state variants.
6. Hollow Deck graphical presentation for Hueman and Gargoyle.
7. Expand to other Natures.

Do not attempt to design every Nature equally at once. Prove the style through
Hueman and Gargoyle first.

### Renderer-facing identity layers

Visual identity must resolve independently across House, Nature, Current Form,
Aura Set, Institution, Sect or Order, Organization, and Alias. Social labels
may change posture, clothing, tools, insignia, context, or animation, but they
must not duplicate sprite biology unnecessarily.

Hunter, Bro White, and Johnny Apple are three presentation layers over a
Flynt/Gargoyle base, not three biological sprite species. Mason, Freemason,
and Superstar likewise do not automatically imply new anatomy.

## 3. The eleven identity axes

The renderer must preserve these answers separately:

1. **Biology:** What is the body?
2. **Biological growth:** What has that body biologically developed into?
3. **Culture:** Which complete way of life frames the person?
4. **Citizenship or social/Current identity:** What is the person's social
   baseline or achieved social identity?
5. **Institutional service:** What institution has the person entered or
   served?
6. **Institutional mastery:** What level of authority or mastery has been
   earned?
7. **Sect, branch, order, or Aura path:** Which specialist discipline or
   faculty mastery is practiced?
8. **Synthesis or apex attainment:** What convergent or completed mastery has
   been achieved?
9. **Operating organization:** Where and how is the person operating?
10. **Public or social alias:** What does that social world call the person?
11. **Visual presentation:** How are all prior answers expressed now?

These axes may affect clothing, equipment, silhouette, accessories, insignia,
animation behavior, environment, and VFX. They do not automatically mutate
Nature.

An eventual actor projection may carry Nature, biological growth, citizenship,
service, elite status, discipline, mastery, organization, alias, outfit, and
accessories. Prefer typed canonical inputs and derived projections over a bag
of independently mutable strings that can contradict each other.

## 4. Asset composition rule

Nature owns anatomy. Biological growth owns anatomical milestones. Institution
and status own trained bearing, tools, permission, and recognized mastery.
Sect or order owns specialist equipment and discipline. Operating organization
owns the social register and environment. Alias owns contextual recognition
and restrained personal motifs.

Do not create new biological sprite families solely for Corpman, Mystery Man,
Hunter, Chief, Mariner, Bro White, Johnny Apple, Beauty the Beast, Tommy Rose,
Cindarellaman, Jimmy Pumps, Mason, Mage, Freemason, Pugilist, Foreman, or
Proliteriate. The same prohibition applies to Hearthkeeper, Householder,
Husbandman, Beach Bum, Party Boy, Socialite, Geomancer, Labyrinthian, Demiurge,
Acolyte, Navigator, Ascendant, The Transcendent, and The Sandman. Reuse the
canonical body unless actual Nature anatomy or growth requires a different
body asset.

Layering must look authored and resolved. It must not resemble unrelated paper
dolls or two full costumes stacked over one another.

## 5. House institutional contrast

The Houses intentionally do not share one normalized rank tree.

### Glaüshouse — seat and coequal branch

```text
Nightingale de <Seat>
        │
    ┌───┴───┐
    │       │
 Matron   Marshall
Presensor Preceptor
  Jade     Emerald
```

Presensor/Matron/Jade and Preceptor/Marshall/Emerald are parallel identities,
not sequential ranks. Their biological Nature paths are branch-specific.

### Flynt — service, elite specialization, and convergence

```text
Warden
  ↓
Corpman
  ↓  prove Capacity
Mystery Man
  ↓
Hunter | Chief | Mariner
   ╲       │       ╱
 Current | Aura | Weapon synthesis tracks
           ↓
  Manticorp / combined synthesis
           ↓
   Chimera / perfected synthesis
```

Every Mystery Man was a Corpman; not every Corpman becomes a Mystery Man.
Corpman is general service and is not Manticorp. Hunter, Chief, and Mariner are
parallel sects. They contribute principles to three independently refinable
higher-Synthesis tracks; none of the resulting tiers is a fourth ordinary
sect. Manticorp combines and Chimera perfects. Manticorp also names Flynt's
combined-synthesis military institution, but membership alone does not grant
the state. Do not draw the progression as an ordinary career promotion.

### Stonebend — paired competency, Synthesis identity, and apex

```text
Gerald
  ↓
Apprentice
  ↓

Mason Current + Mage Diamond Aura
                 ↓
       Freemason / Superstar
                 ╲
                  + → Hypergiant
                 ╱
    Proliteriate / Rockstar
                 ↑
Pugilist Current + Foreman Diamond Aura
```

Applicant is retired from active canon. Mason and Mage are parallel
competencies; Pugilist and Foreman are parallel competencies. Freemason and
Superstar are two registers for one completed mastery. Proliteriate and
Rockstar are two registers for the other. The two masteries are peers before
their integration as Hypergiant.

### Sandmanor — two complete ways of life and sovereign reconciliation

```text
MINORIAN                              MINOAN

Hearthkeeper                         Beach Bum
     ↓                                  ↓
Householder                          Party Boy
     ↓                                  ↓
Husbandman                           Socialite

Geomancer                            Acolyte
     ↓                                  ↓
Labyrinthian                         Navigator
     ↓                                  ↓
Demiurge                             Ascendant
          ╲                         ╱
             The Transcendent
                    ↓
               The Sandman

Gnome → Faun → Minotaur → Hecaton   Elf → Satyr → Centaur → Pegasus
                         Right Hand   Left Hand
                              ╲       ╱
                              The Sandman
```

Minorian and Minoan are parallel cultures, not branches of a shared rank
ladder. Their social, Aura, and Nature paths remain three independent axes.
Demiurge and Ascendant are peer Aura apexes reconciled as The Transcendent;
The Sandman is the sovereign embodiment of that synthesis, not a Nature.
Hecaton and Pegasus complete their separate Nature/cultural branches as
perfected Hands rather than ordinary biological fourth Forms: Hecaton is the
Right Hand and perfected force; Pegasus is the Left Hand and perfected motion.
The Sandman stands above and commands both.

## 6. Flynt service rendering grammar

### Warden

Warden is ordinary Flynt citizenship. It grants social baseline, not elite
equipment, specialist insignia, or Mystery Men permission.

### Corpman

Corpman is practical, general-purpose, field-ready, disciplined, and shared
Flynt service presentation. Use minimal specialist symbolism. A Corpman has not
yet become a Hunter, Chief, or Mariner.

### Mystery Man

Mystery Man is an earned elite threshold after proven Capacity. It permits
Mystery Men recognition, a personalized field kit, specialist identity, and
greater material or supernatural confidence. Avoid generic wizard robes.

### Hunter

Hunter maps to Gargoyle and Fortification. Show preparation, observation,
containment, protection, surveillance, survival, patient pursuit, field tools,
and Van Helsing-like practical supernatural discipline. The signature weapon
is the **Bolo**: its readable action is restraint, entanglement, tripping,
binding, or arresting motion. Use Gargoyle, Opal, Stillness, and Fortification
language without replacing Gargoyle anatomy.

### Chief

Chief maps to Werewolf and Siphone. Show pack leadership, command, territory,
pursuit, group organization, movement, and living physical authority. Preserve
the Werewolf body. The signature weapon is the **Crossbow**: it should read as
prepared ranged force that remains ready during stalking, tracking, climbing,
movement, and pursuit. `Chief` remains the formal sect; this pursuit language
does not rename it Hunter.

### Mariner

Mariner maps to Mariner and Anchoring. Show navigation, positional control,
water, rigging, routes, rope, coats, and maritime fieldcraft. Preserve the
Mariner body. The signature weapon is the **Harpoon**: show ranged connection,
tether, anchor, reel, drag, and deliberate control of relative position.

### Chimera

Chimera is perfected synthesis and the unique feminine First Companion is its
foremost constitutional embodiment. Communicate an already unified Manticorp
system refined into judgment, timing, restraint, coherence, and economy. She is
not a fourth Mystery Men sect, a generic promotion available to every member,
or random monster clutter.

The Hollow Deck retains reciprocal Manticorp–Chimera combat mechanics using
Convergence and Divergence. Those peer mechanics do not reverse the directional
mastery order `Manticorp → Chimera`.

### Manticore and Manticorp

Manticore is the Tross's masculine mythological identity and the display of a
stable compatibility Form retained by his Continuance records. It is not a
third general synthesis tier beyond Chimera. It should remain recognizable as
the Tross rather than introducing another public mastery rung.

Manticorp is combined synthesis and also Flynt's body of soldiers and lawful
military institution. Render combination through functional integration,
organization, equipment, coordination, and command without depicting three
complete weapons bolted together or granting every soldier the synthesis
state. Chimera's exact perfected weapon form remains open.

## 7. Flynt operating and social contexts

We Fairy Men, The Gallows, and The Marquise are operating contexts, not formal
career steps.

### We Fairy Men

Use open rural terrain, practical workwear, roads, fields, forest, rivers,
farms, daylight, weather, travel gear, personal insignia, and inherited or
handmade objects. Social power comes from protection, trust, reputation,
mutual aid, and favors given. These people look like local legends, not a
uniformed army.

### The Gallows

Use streets, docks, clubs, warehouses, apartments, offices, back rooms,
sharper tailoring, urban workwear, industrial clothing, sparing jewelry, and
neon, street, or harbor light. Social power comes from favors owed, debts,
leverage, access, routes, and contracts. Avoid stereotypical gangster costumes
and costume-drama mafia.

### The Marquise

The Marquise is a maritime institutional and operating extension strongly
connected to the Mariner tradition. Use waterways, ships, ports, navigation,
routes, and maritime operational confidence. It is not the next step after We
Fairy Men or The Gallows.

## 8. Flynt principal identity motifs

| Nature/sect | We Fairy Men | Gallows | Motif |
|---|---|---|---|
| Gargoyle/Hunter | Bro White | Johnny Apple | apple |
| Werewolf/Chief | Beauty the Beast | Tommy Rose | rose |
| Mariner/Mariner | Cindarellaman | Jimmy Pumps | shoe or pump |

Johnny Apple draws from Snow White's poisoned apple with a Johnny Appleseed
echo. Tommy Rose draws from Beauty and the Beast. Jimmy Pumps draws from
Cinderella's glass slipper, with pumpkin carriage, midnight, and Midnight Crew
echoes. Use jewelry, pins, embroidery, logos, tattoos, vehicles, tools, office
decor, Deck iconography, or environmental storytelling. Never use literal
fairy-tale cosplay.

## 9. Flynt sorcery and formulation rendering grammar

Flynt sorcery is a synthesis-manufacturing and formulation system governed
within the wider Flynt vice and synthesis ecosystem. It is separate from the
Mystery Men service ladder, its Hunter/Chief/Mariner sects, all biological
Nature paths, and Sandmanor's Aura faculties.

```text
Wizard    ↔ Witch
   ↓          ↓
Warlock   ↔ Enchantress
   ↓          ↓
Sorcerer  ↔ Sorceress
```

The paired identities are peers at each tier. Wizard/Witch manufacture
established Synthesis Recipes and products: show practiced tools, a repeatable
station, stable reference patterns, and finished goods. Warlock/Enchantress
alter existing patterns by combining, counterfeiting, corrupting,
strengthening, weakening, repurposing, or reformulating them: show controlled
splices, substitutions, overlays, and modified diagrams rather than automatic
villain coding. Sorcerer/Sorceress author new Synthesis Recipes or patterns:
show original schematics, prototypes, formulation choices, and controlled
validation.

Rank must read through the practitioner's relationship to the pattern, not
through generic robes, hat size, visual clutter, or invented anatomy.
Sanctioned practitioners are not inherently criminal, hostile, villainous, or
anti-Tross. Uncontrolled synthesis is the systemic danger.

The wider ecosystem remains legible as distinct functions:

- The Tross: sovereign direction and governance;
- Mystery Men: enforcement, interdiction, investigation, and regulation;
- Sorcery: manufacture, formulation, modification, and authorship;
- The Gallows: urban vice, leverage, distribution, and commerce;
- We Fairy Men: rural public legitimacy, mutual aid, and protection;
- The Marquise: maritime operating and routing extension.

These roles may share scenes and material language without becoming one rank
tree. Sorcery presentation layers over a person's existing biological body.

## 10. Stonebend physical language

Stonebend remains North, Presynce, Hollow Current, Diamond, The Cut, and
Hollowing.

PrimePunk makes those principles visible through working mechanisms. Stonebend
objects may blur tool, instrument, machine, weapon, and civic object, but every
visible lever, joint, counterweight, vessel, brace, spring, gear, pressure line,
or void must explain load, transfer, control, repair, separation, or use. Avoid
decorative machinery. The final eight-stage object arsenal remains open and
must be designed from practical principles rather than filled with conventional
fantasy weapons.

Presynce apprehends physical presence as matter, structure, support, load,
fault, and relationship. It asks what is here, what it is made from, where its
weight lies, what supports it, where it fails, what bears load, what moves when
another part moves, and what remains after a cut. It is not omniscience.

The Cut is the selection of the exact physical boundary where one material
state, relationship, or structure becomes another:

- Mason: raw material → Cut → fitted material;
- Pugilist: static load → Cut → displaced load;
- Orc: whole structure → Cut → sundered structure;
- Troglodyte: solid mass → Cut → controlled hollow.

Hollowing remains the terminal Stonebend Nature and process expression. This
bible does not redefine it.

## 11. Stonebend institutional rendering grammar

### Gerald and Apprentice

Gerald is normal Stonebend citizen presentation. Apprentice is the shared
training base: practical work clothing, tools, training equipment, minimal
order distinction, and restrained Diamond/Hollow Current hints.

### Mason and Mage

Mason is Current specialization in material contact, placement, cutting,
fitting, joining, bracing, and tool use. Emphasize structured workwear,
measurement, and hands-on construction.

Mage is Diamond Aura specialization in geometry, alignment, planes, angle,
measurement, structure, load path, fault, and intended form. Emphasize
structural reading and precise pattern or perception. Do not default to generic
fantasy wizard robes.

### Freemason / Superstar

Freemason/Superstar integrates Mason craftsmanship with Mage Diamond structural
perception. It is one completed mastery in two cultural registers, not a
Freemason costume followed by a Superstar costume. Its developing professional
expression is the physical realization of true Synthesis Weapons: master
machining, gunsmithing, mechanical engineering, material science, precision
fit, and functional proof rather than generic blacksmith imagery. Ordinary
craftspeople still make ordinary weapons.

### Pugilist and Foreman

Pugilist is Current specialization in generating, absorbing, displacing,
carrying, striking, bracing, grappling, compressing, and enduring force. Show a
body under load, practical protection, leverage, and impact without generic
fighter-class shorthand.

Foreman is Diamond Aura specialization in organizing force, load distribution,
sequence, coordination, support, dependency, and directing bodies, tools, and
work through a physical system. Use worksite authority, measurement, planning,
and structural overlays. Foreman is not generic mind control or a psychic
commander.

### Proliteriate / Rockstar

Proliteriate/Rockstar integrates Pugilist capacity with Foreman organization of
load. It is one completed mastery in two registers and should communicate
force made productive. Its developing civic expression includes architecture,
fortification, roads, tunnels, arenas, cranes, elevators, waterworks,
ventilation, rotating structures, and large PrimePunk machinery. Show Hollowing
applied to useful space without depicting the constitutional Proliteriate as
one architect or permanent chamber.

### Hypergiant

Hypergiant integrates Freemason/Superstar structural mastery with
Proliteriate/Rockstar capacity and load mastery. Show something monumental,
controlled, load-bearing, precise, and complete—not merely a bigger Ogre.
Public demonstrations, competitions, ceremonies, reveals, engineering
exhibitions, arenas, and civic events may make that achievement legendary.
The proposed cane gun is a working signature-object direction only; do not make
it a mandatory asset before its mechanism and lawful use are designed.

The Deck's mastery rendering does not merge Hypergiant into Stonebend's Nature
ladder. Existing Stonebend constitutional authority also defines Freemason,
Proliteriate, and Hypergiant in Claim, Yield, and Title domains; those office
and civic meanings remain authoritative in their own scope.

## 12. Stonebend biology remains independent

```text
Gremlin → Goblin → Ghoul → Spectre → Troll → Orc → Ogre → Troglodyte
```

The trait arc remains Scavenging, Rigging, Devouring, Phasing, Regeneration,
Sundering, Crushing, and Hollowing. A Troll may separately train as Mason,
Mage, Freemason, Pugilist, Foreman, or Proliteriate. Do not duplicate the
biological ladder per order.

## 13. Sandmanor visual foundation

Sandmanor remains East, Regular Current, Crystal, and The Way. Its visual
character is bright, open, aquamarine, sky-blue, and blue-white, with Crystal
refraction, coast, distance, movement, and possibility. Crystal effects should
clarify conditions, routes, dependencies, and openings rather than cover the
character in undifferentiated sparkle.

The two cultures should be recognizable through posture, equipment, setting,
rhythm, and relational composition while still sharing Sandmanor's Crystal
material language. Minorian emphasizes interior continuity, household and
material stewardship, and Prefog. Minoan emphasizes coastal mobility, social
movement, possibility, and Prefig.

Prefog asks, “What is going to be needed?” It anticipates pressure, need,
shortage, support requirements, and continuity demands. Use evidence of
resource depletion, structural strain, household and land needs,
environmental pressure, and future support requirements. Do not render it as
generic prophecy.

Prefig asks, “What is this becoming?” It anticipates form, trajectory,
possibility, social movement, and emerging opportunity. Use routes opening,
groups forming, events becoming important, opportunities emerging, trends
gaining momentum, and viable trajectories. Do not render it as generic
fortune-telling.

## 14. Minorian rendering grammar

### Hearthkeeper

Show practical domestic continuity: a sustained hearth, work tools, basic
care, repeatable routine, and modest durable clothing. Hearthkeeper is the
baseline Minorian social/Current identity and Beach Bum's conceptual peer.

### Householder

Show property and household authority through organized equipment, stores,
dependents' needs, tools, obligations, management motifs, and contingency.
This is organization of a complete household rather than merely a more ornate
Hearthkeeper.

### Husbandman

Show mature stewardship integrating land, resources, animals, family,
production, planning, and long-term continuity. Use practical authority.
Husbandman is the apex Minorian social/Current identity, not a beginner farmer
and not a caricature farmer.

### Geomancer

Show local terrain and resource reading through soil, stone, water, grade,
boundary, orientation, immediate constraint, and a precise Crystal-ground
relationship. The visual question is, “What does this place permit?”

### Labyrinthian

Show routes, storage, thresholds, bottlenecks, resource chains, relationships,
contingencies, nested systems, and overlays that make dependencies legible.
Use maze and system motifs without literal maze-costume shorthand. The visual
question is, “If this changes, where does the effect travel?”

### Demiurge

Show resolved systems mastery and a mature Crystal world-building identity.
Demiurge arranges conditions into a functioning, sustainable world or system;
it does not mean an omnipotent creator of a universe. Its images should make
viability, integration, and sustained conditions readable.

## 15. Minoan rendering grammar

### Beach Bum

Show low-attachment coastal freedom through practical relaxed clothing,
lightly carried necessities, adaptability, and a movement-ready silhouette.
This is baseline freedom of movement, not frivolous incompetence.

### Party Boy

Show social activation through rhythm, expressive movement, timing,
attention, invitation, connection, and a composition in which unrelated people
become an event. The power lies in gathering formation rather than costume
excess.

### Socialite

Show access, polish, network mobility, and recognizable status carried between
groups, places, and relationships. Avoid fashion caricature; the key visual is
mature social range and portable reputation.

### Acolyte

Show disciplined Prefig initiation: poised, observant, balanced, and alert to
emerging possibility, with restrained Crystal possibility motifs. Acolyte is a
trained initiate, not a generic religious follower. The visual question is,
“What possibility is opening?”

### Navigator

Show route and trajectory language through maps, direction, movement, coast or
water, social paths, negotiation, and clean Crystal lines or refraction. The
visual question is, “Which route do we take, and how do we stay on it?”

### Ascendant

Show lightness, openness, boundary-breaking, and aerial or vertical visual
logic. Ascendant exceeds the limits of existing trajectories and possibility
structures. Keep the image resolved rather than cluttered. The visual question
is, “What if the existing route is no longer the limit?”

Witch, Wizard, Sorcerer, and Warlock are not current Sandmanor Aura identities.
They belong to Flynt's sorcery and synthesis-manufacturing ecosystem and must
not be used as substitutes for Acolyte, Navigator, or Ascendant.

## 16. The Transcendent and The Sandman

The sovereign visual synthesis is:

```text
Demiurge + Ascendant → The Transcendent → The Sandman
```

Demiurge contributes world-making, viable conditions, and sustained systems.
Ascendant contributes world-exceeding openness beyond existing conditions.
The Transcendent masters The Way between them: the Minorian says to build the
house, the Minoan says to leave the house, and The Transcendent knows when to
do either.

The Sandman should therefore communicate sovereign mastery of the movement
between stability and transcendence. Elevate the folkloric Sandman archetype
within Sandmanor's own bright Crystal, coast, distance, and possibility
language. Incorporate command of Hecaton's perfected force and Pegasus's
perfected motion without turning The Sandman into a larger version of either
Hand or an unrequested Morpheus or Hypnos deity identity.

The Transcendent is neither an ordinary social rank nor a Nature rung. The
Sandman is its sovereign embodiment. Husbandman is not Hecaton; Socialite is
not Pegasus; Demiurge is not Hecaton; Ascendant is not Pegasus.

## 17. Sandmanor biology remains independent

```text
Minorian Forms: Gnome → Faun → Minotaur
MachPunk traits: Grip → Traction → Leverage → Multiplication
Branch culmination: Hecaton / perfected force / Right Hand

Minoan Forms:   Elf → Satyr → Centaur
MachPunk traits: Balance → Agility → Momentum → Flight
Branch culmination: Pegasus / perfected motion / Left Hand
```

Hecaton may express multiplication of working anatomy, especially arms, hands,
and simultaneous physical work. Pegasus may express biological wings and
Flight, with Buds → Winglets → Wings used when wing development is shown. The
four-part lines are locked branch progressions, but art must keep the final
Hand investiture visually distinct from ordinary biological Form growth. The
Sandman must read as the sovereign whole above both.

## 18. Source-asset and implementation boundary

Author source sprites in Aseprite or another lossless hand-authored workflow,
retain source layers, and export deterministic runtime sheets later. Keep body,
growth, equipment, insignia, contextual accessory, and effects concerns
separable where animation and occlusion permit, but merge authored layers when
that produces a clearer resolved silhouette.

Future rendering code should consume the Hollow Deck's immutable hierarchy and
derived projections. It must not infer institutional status from sprite file
names, catalog order, biological tier, or save-file booleans. This pass creates
no sprites, shaders, window, renderer, or graphical runtime.
