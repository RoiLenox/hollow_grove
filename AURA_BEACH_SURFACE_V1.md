# Aura Beach Surface V1

Status: implemented singular interior region
Backbone: `HOLLOW_GROVE_COMPROMISE_V1_DRAFT.md`
Primary constitutional authority: `SANDMANOR_CONSTITUTION_V2.md`
Executable contract: `src/world/aura_beach.rs`
Gameplay projection: `aura-beach.coastal-commons`

## Geographic and Regional Lock

Aura Beach is one singular coastal surface. Its beach, strand, landing, pier,
dunes, market, rescue post, and proving places are constituents of that one
region.

Aura Beach is attributed first to **Sandmanor's Minoan exterior tradition**.
Its constitutional purpose is to prove how a design meets the world beyond
itself: approach, orientation, transport, public meaning, visibility, climate,
navigation, connection, and shared space.

The Current Sea is Aura Beach's linked Minoan maritime jurisdiction. It is not
a second beach and Aura Beach is not another Current Sea route.

## North-to-South Constitutional Gradient

```text
Sandmanor
→ Free Aura Beach
→ Southern Coast
→ Current Break
→ Minoan County Courthouse
→ Glaüshouse
```

Free Aura Beach is the northern liberty-oriented public coast. Regulation
increases gradually southward as Current danger, rescue, emergency traffic,
restricted water, Manticorp training, and proximity to Glaüshouse intake
increase. Southern Law is Minoan coastal jurisdiction, not a fifth House or
Glaüshouse law.

Current Break is Sandmanor territory and may host Flynt-authorized Manticorp
maritime training. Minoan guardians teach coastal survival and rescue; Flynt
retains Manticorp command, curriculum, identity, equipment, and advancement.

The **MINOAN COUNTY COURTHOUSE** stands after Current Break and immediately
before Glaüshouse. It remains a Sandmanor institution even when it lawfully
transfers an injured or detained person into Glaüshouse clinical care.

The executable facility and movement map remains unchanged in this milestone;
these zones are constitutional geography for future presentation.

## Boundary and Route Access

Aura Beach is the lower-right interior triangle:

| Boundary point | Normalized per-mille coordinate |
|---|---:|
| Sandmanor | `(735, 500)` |
| Glaüshouse | `(500, 860)` |
| Aura Ridge Junction | `(500, 500)` |

Its lawful gameplay approaches are:

- Aura Ridge, carrying public witness to the central shore approach;
- Current Sea, carrying maritime certification and return;
- Glausbahn, carrying refinement between Sandmanor and Glaüshouse;
- Current Seanad, carrying difficult coastal design into deliberation.

An approach grants physical entry only. It does not establish Minoan standing,
Elf competence, Centaur mobility, Synthesis eligibility, Clearance, ownership,
office, recognition, or Title.

## Coastal Working System

| Place | Coastal function |
|---|---|
| Coastal Proving Strand | repeated access, visibility, movement, and shoreline-impact trials |
| Common Shore Approach | accessible arrival, orientation, meeting, and departure |
| Minoan Navigation School | charts, bearings, current, weather, communication, and return plans |
| Aura Beach Beacon | public position, hazard, and return signals |
| Tide and Current Station | tide, depth, erosion, current, and shoreline-change records |
| Coastal Weather Station | wind, pressure, light, storm, visibility, and Aura exposure |
| Shore Rescue Post | watch, retrieval, stabilization, consent, and care transfer |
| Centaur Mobility Run | roaming, escort, patrol, changing terrain, and mobility proof |
| Elf Exterior-Design Yard | provisional approaches, shelters, signals, craft, and transport forms |
| Common Boat Landing | launches, landings, passengers, cargo custody, and route continuity |
| Public Pier | shared access, inspection, loading, fishing, and observation |
| Recovery Pavilion | shade, water, warming, first response, rest, and protected discharge |
| Living Dune Ward | erosion control, habitat, storm buffering, and marked foot access |
| Shore Fish Market | landing records, weights, temperature, provenance, prices, and reserves |
| Shore Salvage Yard | recovered craft, cargo, hazards, claims, repair, and return |

The facilities form one coastal commons. A pier, private boat, market lot, or
design trial does not become a sovereign shoreline.

## Minoan Exterior Proof

Minoan work asks whether a proposed form remains usable when it meets changing
weather, public movement, distance, tide, cargo, visibility, and another
person's needs.

```text
Need beyond the interior
→ provisional exterior form
→ disclosed route and conditions
→ public demonstration
→ weather, tide, access, and rescue observations
→ criticism and failed-result preservation
→ reproduction under changed conditions
→ scoped Sandmanor proof or revision
```

An attractive beacon that cannot be read in fog is not proven. A fast landing
that excludes injured travelers is not complete. A successful single crossing
is evidence, not automatic credential or Synthesis.

## Elves and Centaurs

Elves design, explore, orient, navigate, communicate, and give provisional form
to exterior relationships. An Elf Name identifies a person and tradition; it
does not infer mastery.

Centaurs roam Aura Beach, escort travelers, patrol the shoreline, watch coastal
routes, defend against incursion, and maintain the land-sea boundary after
lawful Elf-to-Centaur Synthesis. Mobility is responsibility rather than
dominion. Presence, speed, or a completed patrol does not create Synthesis,
office, or ownership.

Current Sea guardianship remains related but distinct. Beach roaming cannot be
silently treated as maritime certification.

## Four-House Responsibilities

Sandmanor proves Minoan exterior design, access, navigation, movement, and
shoreline relationship. Proof cannot create Synthesis, Clearance, recognition,
office, or Title.

Stonebend names shoreline boundaries, vessels, lots, persons, continuities,
salvage claims, and disputes. Naming a vessel does not prove seaworthiness or
ownership of recovered cargo.

Glaüshouse clears food, water, exposure, injury, rescue, recovery, and public
health hazards within an explicit scope. Rescue custody is not ownership and
first response is not a claim of coastal competence.

Flynt recognizes completed public service, exchange, resolved claims, and
performed function. Recognition cannot manufacture design proof, consent, or
property.

## Tide, Weather, Ecology, and Aura

Aura appears here as air, pressure, light, glare, mist, visibility, and
exposure. It reveals conditions; it does not navigate, choose, or clear them.

Tide and weather records preserve time, station, observer, instrument, depth,
current, wind, pressure, visibility, and uncertainty. A forecast remains a
forecast. A storm warning may narrow access without becoming permanent law.

The living dune ward holds vegetation, nesting ground, sand movement, storm
buffering, and erosion control. Marked routes keep public access from destroying
the protection that makes access possible.

Fish and salvage records preserve source, time, temperature or condition,
custody, contamination, weight, claim status, reserve duty, and transfer.
Unresolved material stays segregated rather than becoming market stock.

## Rescue and Recovery Loop

```text
Watch
→ receive distress
→ locate
→ retrieve
→ establish identity and capacity where possible
→ stabilize within scope
→ record custody and uncertainty
→ transfer to qualified Glaüshouse care
→ recover or refuse further participation
→ review the route and design failure
```

Emergency action remains narrow and reviewable. No rescued person owes service,
patronage, testimony, labor, or Synthesis consent in exchange for recovery.

## Public Economy and Access

The shore approach and pier remain public within safety limits. Accessibility,
orientation, rest, loading, observation, fishing, rescue lanes, and protected
dune areas are marked separately so one use does not silently erase another.

The fish market exposes weights, provenance, temperature, prices, reserve
obligations, and challenged lots. The salvage yard exposes custody and claims.
A private receipt is not Flynt recognition, and possession at landing is not a
Stonebend determination of ownership.

## Gameplay Contract

`aura-beach.coastal-commons` is a fixed `20 × 18` map. Every one of its fifteen
coastal facilities has a walkable approach and an interaction witness.

The immutable surface view exposes:

- singular identity and Sandmanor attribution;
- the exact three-point boundary;
- four lawful route approaches;
- every facility's stable identity, kind, and function.

Wrong-route entry fails without revision or event. Accepted entry persists and
replays through gameplay archive schema V2. Godot renders water, sand, coastal
works, dialogue, and attribution without owning proof, rescue, Synthesis, or
House authority.

Inspect all three surface contracts with:

```bash
cargo run --quiet --bin aura_surfaces_audit
```

## Completion Boundary

This implementation establishes the region, coastal systems, constitutional
limits, traversal, facility witnesses, persistence, and presentation. It does

The living-surface pass now adds tide height, storm pressure, visibility,
traffic, rescue readiness, fish stock, dune integrity, temporary closure,
scheduled shore workers, deterministic weather, and the evidence-gated storm
rescue case. Riptide spills can harm the shore; Mt. Aura stone can reinforce
dunes; Current Sea certified lots can support rescue logistics.

It does not yet claim boat physics, individual fish simulation, autonomous
rescue AI, a complete commodity market, or the full Elf-to-Centaur Synthesis
case. See `HOLLOW_GROVE_LIVING_SURFACES_AND_EXTRACTION_V1.md`.
