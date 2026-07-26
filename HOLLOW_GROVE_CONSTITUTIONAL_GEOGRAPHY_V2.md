# Hollow Grove V2 Constitutional Geography

Status: canonical and locked.

This specification constitutionalizes the major Hollow Grove routes without changing the recursion kernel, Constitutional Runtime V2, the Flynt Constitution, House authority, or frozen Current Synthesis routing. The executable world-facing authority is `src/world/geography.rs`. Aura Ridge's shared public economy and the Central Junction district are governed additively by `CENTRAL_JUNCTION_FOUR_POLE_ECONOMY_V1.md` and `src/world/central_junction.rs`.

## Constitutional boundary

The following remain unchanged:

- Current repeats.
- Aura reveals.
- Relativity bends.
- Synthesis transforms.
- Stonebend, Sandmanor, Glaüshouse, and Flynt retain their existing authority.
- Current Runtime V2 remains canonical for state and causality.
- `flynt-constitution` remains canonical for Flynt authority.
- the recursion kernel remains neutral.

Constitutional geography is an upper world layer. It answers why civilization uses a route. It does not decide whether a movement intent succeeds, grant institutional authority, perform Synthesis, or feed route meaning into a recursion kernel.

## Frozen Glaüshouse law

Glaüshouse remains the House of repair, recovery, medicine, and Synthesis. Its question remains:

> How is it restored?

Its hierarchy remains:

```text
Doctor Ratchet (Prima Donna)
  |
  v
Nurse House (Persephone)
  |
  v
The Nightingales
```

Glauspitals and Chromacord remain the frozen institutions. Route law may present their intake, recovery, diagnostic, or discharge work; it may not change their constitutional place.

## Route law

A major route is a constitutional verb expressed as geography. Its dominant purpose is an institutional process, never mere connection.

Every canonical route has:

- one stable route identity;
- one House boundary;
- exactly one dominant constitutional verb;
- exactly one non-empty constitutional purpose;
- an explicit process flow;
- a declared relationship to the frozen runtime projection;
- a visually and emotionally distinguishable presentation recommendation.

Straight and curved geometry remains useful presentation and traversal information. It is subordinate to route purpose: geometry describes how the route is shaped; the constitutional verb explains why civilization maintains it.

## Canonical route roster

| Stable ID | Route | House boundary | Dominant verb | Constitutional purpose |
|---|---|---|---|---|
| `geography.route.boardwalk` | Boardwalk | Glaüshouse / Flynt | Return | constitutional discharge, reintegration, and recognition after recovery |
| `geography.route.riptide` | Riptide | Flynt / Glaüshouse | Retrieve | involuntary emergency retrieval of damaged beings toward repair |
| `geography.route.current-sea` | Current Sea | Glaüshouse / Stonebend | Certify | certification that restoration survives depth and endurance |
| `geography.route.aura-ridge` | Aura Ridge | Glaüshouse / Stonebend | Witness | public witness, presentation, exchange, and civic reintegration |
| `geography.route.glausbahn` | Glausbahn | Glaüshouse / Sandmanor | Refine | high-speed iteration where design and repair improve one another |
| `geography.route.current-seanad` | Current Seanad | Glaüshouse / Sandmanor | Deliberate | constitutional review of difficult design, repair, and Synthesis questions |
| `geography.route.aura-way` | Aura Way | Stonebend / Sandmanor | Design | the established constitutional design process |
| `geography.route.mnt-aura` | Mt. Aura | Stonebend / Sandmanor | Aspire | the established constitutional path of aspiration |
| `geography.route.basin-motorspeedway` | Basin Motor Speedway | Stonebend / Flynt | Produce | the established constitutional production process |
| `geography.route.stairway-to-heaven` | Stairway to Heaven | Stonebend / Flynt | Ascend | the established constitutional path of ascension |

`Mt. Aura` and `Basin Motor Speedway` are the canonical display spellings ratified by the Stonebend Constitution. The legacy stable IDs and frozen runtime projection keys retain `mnt-aura` and `basin-motorspeedway` only for deterministic compatibility; those keys are not display names.

## Executable route geometry

`src/world/route_network.rs` now projects the locked screen geometry without
changing route purpose:

- Straight: Aura Ridge, Aura Way, Glausbahn, Boardwalk, and Basin Motor
  Speedway;
- Round, presented as curved: Mt. Aura, Current Seanad, Riptide, and Stairway
  to Heaven;
- Sea Ordeal: Current Sea only.

`Round` is the executable world-geometry term corresponding to the screen
map's Curved Routes heading. It remains separate from Light/Dark Aura, moral
alignment, Moxy, and House authority. The exact maps and endpoint-sharing
traversal contract live in `HOLLOW_GROVE_ROUTE_NETWORK_V1.md`.

## Flynt / Glaüshouse boundary

### Boardwalk — Return

```text
Recovery Ward
  |
  v
Boardwalk
  |
  v
Flynt
  |
  v
Recognition
```

The Boardwalk is the constitutional discharge corridor. It is calm, accessible, stable, public, and documented. A being leaves by choice. Recovery becomes ordinary civic presence, and Flynt recognition closes reintegration without granting new authority.

### Riptide — Retrieve

```text
Flynt Crisis
  |
  v
Riptide
  |
  v
Glaüshouse Intake
  |
  v
Repair
```

The Riptide is the constitutional emergency corridor. Its dominant movement is involuntary: it carries trauma, machine failure, Aura collapse, Current overload, failed Synthesis, rescues, shipwrecks, lost people, and unidentified patients toward Glaüshouse intake. Nobody elects the Riptide as ordinary travel.

The Boardwalk and Riptide are reciprocal civic functions, not duplicate transportation: Riptide retrieves crises into repair; Boardwalk returns recovered beings into public life.

Riptide is also the geographic and metaphysical symbol of Bathos: material
weight, consequence, exposure, and unsupported elevation meeting reality.
This meaning does not make Riptide evil, move it to Stonebend, or alter its
Flynt–Glaüshouse `Retrieve` authority. Stonebend owns neither Riptide nor
Bathos.

## Glaüshouse / Stonebend boundary

These northern paths are constitutionally separate.

### Current Sea — Certify

```text
Repair
  |
  v
Current Sea
  |
  v
Stonebend
  |
  v
Naming
  |
  v
Title
```

The Current Sea is a constitutional ordeal, not another road. The crossing certifies that restoration survives depth, endurance, and structural truth. Bridges, ferries, repair barges, undersea tunnels, medical vessels, and inspection platforms are possible presentation forms; the constitutional test is the crossing itself.

### Aura Ridge — Witness

```text
Repair
  |
  v
Recovery
  |
  v
Aura Ridge
  |
  v
Equal Gaze
  |
  v
Central Junction
```

Aura Ridge is civic reintegration, not legal certification. A traveler on the ridge has already been restored and is ready to be seen. The route is elevated, open, visible, beautiful, and publicly shared. Equal Gaze means reciprocal civic witness; it does not create a new House office or override existing recognition law. Central Junction is the district where that public route meets the shared economy; the South Ridge Exchange is one institution inside it.

Current Sea asks whether repair survives depth. Aura Ridge asks whether the restored being can re-enter common sight. Certification and witness may support one another, but they are not the same responsibility.

## Glaüshouse / Sandmanor boundary

### Glausbahn — Refine

```text
Design
  |
  v
Prototype
  |
  v
Repair
  |
  v
Improved Design
```

The Glausbahn is the high-speed iteration corridor. Designs move toward Glaüshouse repair knowledge; diagnostic and repair knowledge returns toward Sandmanor design. The reciprocal cycle is its constitutional function.

Its presentation combines the sustained speed and engineering clarity of a German Autobahn with the cliff, bridge, coast, and horizon drama of the California Pacific Coast Highway. This is a visual reference only, not copied infrastructure or signage.

### Current Seanad — Deliberate

```text
Question
  |
  v
Evidence
  |
  v
Institutional Deliberation
  |
  v
Judgment
```

The Current Seanad is not a transportation corridor. It is the place where difficult design and repair questions receive constitutional review:

- should a Synthesis proceed;
- should a repair become permanent;
- should a body be redesigned;
- should a prototype be approved.

The Seanad holds institutional thought. It does not perform Glaüshouse Synthesis, assume Sandmanor proof authority, or invent a new decision hierarchy.

## Frozen established boundaries

The following meanings and process spines are preserved without redesign:

| Boundary | Route | Verb | Executable process spine |
|---|---|---|---|
| Stonebend / Sandmanor | Aura Way | Design | Need -> Design -> Arrangement -> Usable Form |
| Stonebend / Sandmanor | Mt. Aura | Aspire | Present Form -> Aspiration -> Ascent Pressure -> Higher Aim |
| Stonebend / Flynt | Basin Motor Speedway | Produce | Named Plan -> Production -> Field Trial -> Deployable Work |
| Stonebend / Flynt | Stairway to Heaven | Ascend | Recognized Capability -> Ascent -> Higher Burden -> Title |

Constitutional geography records these routes so the ten-route audit is complete. It does not add new lore, procedures, institutions, or authority to them.

## House-boundary flow audit

| Stable boundary ID | Boundary | Inward constitutional flow | Outward constitutional flow |
|---|---|---|---|
| `geography.boundary.flynt-glaushouse` | Flynt / Glaüshouse | Riptide retrieves Flynt crises into Glaüshouse intake and repair | Boardwalk returns discharged beings to Flynt reintegration and recognition |
| `geography.boundary.glaushouse-stonebend` | Glaüshouse / Stonebend | restored beings enter either depth certification or public witness | the boundary releases certified title or witnessed civic reintegration |
| `geography.boundary.glaushouse-sandmanor` | Glaüshouse / Sandmanor | designs, repairs, and hard questions enter iteration or review | improved designs, repair knowledge, and constitutional judgments leave |
| `geography.boundary.stonebend-sandmanor` | Stonebend / Sandmanor | named needs enter the established design or aspiration path | designed forms and articulated aspirations return to civilization |
| `geography.boundary.stonebend-flynt` | Stonebend / Flynt | named capability enters production or the established ascent path | deployable work and accepted higher burden return to civilization |

Each boundary has two routes and one coherent process pair. A shared endpoint does not make route responsibilities interchangeable.

## Frozen runtime projection boundary

Nine constitutional routes have a one-to-one presentation key already exposed by the frozen Current Synthesis route layer. Current Sea does not.

The frozen runtime currently uses an internal `CurrentSea` token for the route displayed as Current Seanad. That implementation remains untouched. The constitutional geography therefore:

- maps Current Seanad to the frozen `current-seanad` key;
- assigns Current Sea the distinct stable ID `geography.route.current-sea`;
- assigns no frozen runtime route key to Current Sea;
- rejects any attempt to map Current Sea onto Current Seanad.

This is a fail-closed compatibility boundary. Future presentation work may render Current Sea from the neutral geography view, but no client may pretend that the frozen Current Seanad token is Current Sea.

## Visual world recommendations

These are recommendations for a later Godot implementation. They are not executable terrain law and do not authorize scene changes in this milestone.

Skyboxes should reinforce each route's emotional weather and horizon without becoming a source of route law. Mountains, coastlines, bridges, shorelines, vegetation, lighting, traffic, and district transitions should make the dominant verb recognizable before the route label appears.

### Boardwalk

- Terrain and shoreline: level timber, stone, and glass promenade above a calm managed shore.
- Vegetation: maintained salt grasses, shade trees, recovery gardens, and accessible planters.
- Lighting and sky: warm late-afternoon light, clear public lamps, soft Flynt color entering gradually.
- Traffic: civilians, discharged patients, family escorts, mobility aids, and public transit.
- Architecture and bridges: low ramps, covered rest bays, discharge kiosks, railings, and documented wayfinding.
- Landmark: the Recognition Gate where clinic signage yields to Flynt civic signage.
- District transition: clinical quiet becomes ordinary public life without a hard checkpoint.
- Gameplay: safe travel, reintegration conversations, civilian errands, optional recognition, and companion reactions.

### Riptide

- Terrain and shoreline: deep water cuts, violent inlets, rescue channels, wreck fields, and unstable Current seams.
- Vegetation: bent coastal growth, kelp, storm-torn reeds, and sparse emergency markers.
- Lighting and sky: storm light, rotating rescue beacons, low visibility, and sharp Glaüshouse intake signals.
- Traffic: rescue craft, medical vessels, tow machines, search crews, and unidentified arrivals.
- Architecture and bridges: reinforced intake docks, lift cranes, flood gates, triage piers, and breakwaters.
- Landmark: the Nightingale Intake Beacon visible through weather and Aura collapse.
- District transition: Flynt crisis space narrows rapidly into controlled Glaüshouse intake.
- Gameplay: rescues, dynamic emergencies, sea hazards, triage routing, containment, escort, and recovery under pressure.

### Current Sea

- Terrain and shoreline: dark deep water, exposed structural spans, pressure shafts, and undersea works.
- Vegetation: minimal surface growth; pressure-adapted underwater life visible near inspection glass.
- Lighting and sky: severe horizon light above, disciplined work light below, with depth increasingly obscuring the sky.
- Traffic: ferries, repair barges, inspection platforms, medical vessels, and certification crews.
- Architecture and bridges: load-bearing bridges, pressure locks, undersea tunnels, test pylons, and named structural stations.
- Landmark: the Depth Certification Platform before the Stonebend approach.
- District transition: Glaüshouse repair finishes become exposed structural truth and finally Stonebend naming.
- Gameplay: inspection, certification trials, endurance management, deep infrastructure, repair verification, and escorted crossings.

### Aura Ridge

- Terrain and vegetation: elevated open ridge, flowering high grass, civic gardens, and long visible approaches.
- Lighting and sky: clear luminous sky, long sightlines, reflective Aura weather, and ceremonial dawn or sunset.
- Traffic: tourists, restored civilians, traders, ceremonies, public gatherings, and companion processions.
- Architecture and bridges: overlooks, public stairs, exchange terraces, accessible lifts, and transparent shelter roofs.
- Landmark: the Equal Gaze overlook aligned with Central Junction.
- District transition: recovery privacy opens gradually into mutual visibility and civic scale.
- Gameplay: tourism, ceremony, public dialogue, exchange, reputation scenes, Equal Gaze encounters, and low-pressure exploration.

#### Central Junction and the Summit boundary

Central Junction is a district, not a stock exchange or a single building. Its
market institutions are the South Ridge Exchange, Junction Board, Clearing
House, and Junction Wire. The Craft Corridor leads toward Stonebend, the Repair
Corridor toward Glaüshouse, and the Design Corridor toward Sandmanor. Flynt
reaches and powers the Junction through the Engineering Ring, transportation,
power, communications, computation, and settlement infrastructure.

Current Haze, Equal Gaze, and Aura Beam remain noninstitutional Summit and
future-vision concepts:

> Current Haze is unresolved possibility.

> Equal Gaze is reconciled perspective.

> Aura Beam reveals or transmits the visible shared future.

A contract may colloquially remain in the Current Haze, and a recognized
decision may be described as reaching Equal Gaze. Neither expression creates a
market office. Aura Beam is not a financial feed; the Junction Wire publishes
market information.

#### Sandmanor coast and farm geography

Aura Farm is one Minorian cultivation system with two halves: Aura Fields for
physical nourishment and Content Farm for mental, cultural, and Aura
nourishment.

The Sandmanor coast proceeds north to south as:

```text
Free Aura Beach
→ Southern Coast
→ Current Break
→ Minoan County Courthouse
→ Glaüshouse
```

Regulation increases gradually southward. Southern Law remains Minoan coastal
jurisdiction. Current Break is Sandmanor territory that may host
Flynt-authorized Manticorp training without transferring command. The
courthouse is the final Sandmanor institution before Glaüshouse; a clinical
transfer across the border does not transfer legal authority over it.

### Glausbahn

- Terrain and coastline: fast coastal grades across cliffs, tunnels, coves, bridges, and glass overlooks.
- Vegetation: wind-shaped coastal trees, scrub, cliff flowers, and carefully protected diagnostic verges.
- Lighting and sky: bright coastal clarity by day; precise Chromacord wayfinding and lane light at night.
- Traffic: prototype carriers, logistics vehicles, ambulances, repair crews, and controlled high-speed civilian travel.
- Architecture and bridges: long engineered spans, cliff tunnels, repair stations, medical pull-offs, diagnostic checkpoints, and Nightingale emergency lanes.
- Landmark: a glass diagnostic overlook where moving prototypes can be diverted safely.
- District transition: Sandmanor design geometry becomes Glaüshouse diagnostic legibility through increasingly clinical road systems.
- Gameplay: high-speed logistics, ambulance routing, timed prototype transport, breakdown response, lane choice, and mobile diagnostics.

### Current Seanad

- Terrain and shoreline: enclosed civic water court or protected tidal basin rather than a through-road.
- Vegetation: contemplative courtyards, reeds, shaded gardens, and quiet evidence walks.
- Lighting and sky: measured daylight, subdued interiors, and deliberate shifts marking sessions rather than speed.
- Traffic: delegates, witnesses, designers, repair experts, patients, advocates, and guarded evidence transport.
- Architecture and bridges: chambers, galleries, archives, hearing bridges, consultation rooms, and public witness balconies.
- Landmark: the central deliberation chamber where Glaüshouse and Sandmanor perspectives remain visibly distinct.
- District transition: moving traffic slows, separates, and becomes institutional assembly.
- Gameplay: story events, constitutional choices, hearings, institutional meetings, evidence review, and companion testimony.

### Aura Way

- Terrain and vegetation: legible planned road through measured fields and shaped civic land.
- Lighting and traffic: clear working light, survey crews, designers, material carriers, and public prototypes.
- Architecture and landmark: plan tables, measuring arches, and a visible design exchange.
- Transition and gameplay: Stonebend named need becomes Sandmanor arrangement through design tasks and proof-ready prototypes.

Aura Way is also the standard institutional metaphor for recognized
advancement: prerequisite, education, supervised practice, examination,
demonstrated responsibility, and recognition eligibility. This meaning remains
subordinate to the route's fixed `Design` verb and creates no shortcut or new
authority. The Houses teach the work; Aura Way organizes the path; Stonebend
names a proven completion.

### Mt. Aura

- Terrain and vegetation: rising mountain curves, exposed stone, high meadows, and thinning alpine growth.
- Lighting and sky: ambitious vertical light, distant goals, changing weather, and visible higher paths.
- Architecture and landmark: shelters, aspiration markers, overlooks, and a summit-facing threshold.
- Transition and gameplay: climbing, long-view choices, aspiration trials, and mastery-oriented encounters without adding a new authority gate.

Mt. Aura is Aether: the ideal and pinnacle toward which every House may rise
through its own Way. No House owns the summit. Mt. Aura does not grant ordinary
credentials, and Stonebend recognition cannot decree metaphysical perfection.
This vertical meaning preserves the fixed `Aspire` verb and the route's shared
Stonebend–Sandmanor boundary.

The canonical route may be read geographically and institutionally as
Sandmanor → Aura Way → Mt. Aura → Sandmanor-facing Stonebend gate → Stonebend:
possibility → disciplined advancement → measurement against the ideal →
evidence review → lawful identity. It does not require every profession to climb physically.

Stonebend's three principal constitutional interfaces are the Flynt-facing gate
through Stairway to Heaven and Basin Motor Speedway, the Central Junction-facing
gate through the Craft Corridor, and the Sandmanor-facing gate
through Aura Way and Mt. Aura. Every facing supports inward and outward
constitutional transfer. Central Junction remains a district rather than a
House; Mt. Aura remains an ideal and route landmark rather than a gate; Riptide
remains on its established Flynt–Glaüshouse route.

During Diamond vacancy, these interfaces continue only through existing
delegated authority or bounded terminating continuity mandates. Geographic
access does not create a Regent, transfer Diamond, enlarge a gate scope, or
appoint a Hypergiant. The controlling lifecycle instrument is
`STONEBEND_TITLE_LIFECYCLE_AND_CONSTITUTIONAL_CONTINUITY_V1.md`.

The route now branches physically to **Mt. Aura High Mine**. The mine cuts
solid Aura-bearing stone through survey, support, ventilation, refuge, hoist,
assay, and custody. It materializes aspiration without making the route, the
summit, or mere access into mineral Title.

### Basin Motor Speedway

- Terrain and vegetation: engineered basin floor, industrial verges, test fields, and hard service corridors.
- Lighting and traffic: practical work light, freight, production teams, field-test vehicles, and repair crews.
- Architecture and landmark: fabrication depots, test loops, load bridges, and a production handoff yard.
- Transition and gameplay: materials and named plans become deployable work through logistics, production tests, and field trials.

### Stairway to Heaven

- Terrain and vegetation: steep monumental ascent, exposed ledges, high stone, and sparse resilient growth.
- Lighting and sky: upward-breaking light, deep lower shadow, changing altitude, and a visibly remote destination.
- Architecture and landmark: landings, burden stations, ascent markers, and a final title-facing threshold.
- Transition and gameplay: recognized capability accepts higher burden through ascent, endurance, and deliberate progression.

The Stairway mountain complex contains the **Stairway Burden Mine** and its
descending **Highway to Hell Deepworks**. The Burden Mine works solid ore
alongside the public ascent. Highway to Hell descends from that mine into heat,
gas, water, and deep iron. It is an industrial mine road inside the complex,
not an eleventh constitutional route and not a moral classification.

### Riptide and Current Sea extraction branches

Riptide branches to the **Riptide Current Recovery Rig**. Its extraction posture
is emergency retrieval: shut in a failed well, retrieve crews and equipment,
contain spills, and quarantine recovered Current-bearing fluid.

Current Sea branches to the **Current Sea Depth Production Rig**. Its posture is
depth certification: pressure-test a sustained well, separate and assay its
fluid, verify custody, certify a bounded rate, or suspend it.

These offshore operations share drilling equipment but do different
constitutional work. Riptide retrieval cannot certify ordinary production;
Current Sea certification cannot waive Riptide rescue. Their
petroleum-side fluid-well analogy remains distinct from the mines' coal-side
solid-seam work.

No extraction classification authorizes blood taken from a living Being.
Contrary identity, tissue, continuity, consent, or provenance evidence ends
commodity handling and invokes quarantine, rescue, care, Illegal Hollowing, or
identity review.

## Godot scene hierarchy recommendation

No Godot scenes are implemented by this specification. A later client should use a hierarchy like:

```text
ConstitutionalWorldRoot
├── HouseRegionLayer
│   ├── StonebendRegion
│   ├── SandmanorRegion
│   ├── GlaushouseRegion
│   └── FlyntRegion
├── ConstitutionalRouteLayer
│   ├── BoardwalkRoute
│   ├── RiptideRoute
│   ├── CurrentSeaRoute
│   ├── AuraRidgeRoute
│   ├── GlausbahnRoute
│   ├── CurrentSeanadRoute
│   ├── AuraWayRoute
│   ├── MntAuraRoute
│   ├── BasinMotorspeedwayRoute
│   └── StairwayToHeavenRoute
├── RouteTransitionLayer
├── RouteLandmarkLayer
├── RouteTrafficPresentation
├── RouteEncounterPresentation
├── RouteAudioAndWeather
└── ConstitutionalGeographyViewAdapter
```

Each route scene should carry only its stable geography route ID and presentation resources. The view adapter reads immutable route definitions and authoritative runtime views. Godot may choose terrain tiles, sprites, animation, sound, camera, lighting, traffic effects, transitions, and encounter presentation. It may not choose a route's verb, alter a process, resolve movement legality, grant House authority, or treat the unprojected Current Sea as Current Seanad.

Recommended reusable scene components:

- `RouteIdentityView`: stable ID, display name, and dominant verb label;
- `RouteTerrainPresentation`: tiles, meshes, vegetation, shoreline, and district transition;
- `RouteTrafficPresentation`: visual traffic populations without causal authority;
- `RouteLandmarkPresentation`: route-specific landmark anchors;
- `RouteEncounterAnchor`: submits encounter or interaction intents to the runtime;
- `RouteTransitionPresentation`: thresholds and camera/audio changes;
- `RouteStatusOverlay`: presents closures, emergency state, certification status, or deliberation state from authoritative views.

## Constitutional validation lock

A conforming implementation must prove:

- exactly ten major constitutional routes exist;
- every route has exactly one dominant verb and one purpose;
- all ten verbs are unique;
- all ten purposes are non-empty and non-duplicated;
- each route has a complete process flow;
- each of the five House boundaries has exactly two routes;
- every boundary declares coherent inward and outward process flow;
- Current Sea and Current Seanad remain distinct;
- frozen runtime projection keys are not duplicated;
- documentation names every stable ID, route, and verb;
- canonical spellings remain `Mt. Aura` and `Basin Motor Speedway`;
- Aura Ridge terminates publicly at Central Junction;
- Current Haze, Equal Gaze, and Aura Beam remain noninstitutional Summit concepts;
- Godot remains a presentation client;
- no frozen constitutional layer imports or depends on constitutional geography.

Run the executable audit with:

```text
cargo run --bin constitutional_geography_audit
```

## Explicit non-goals

- no recursion-kernel changes;
- no Constitutional Runtime V2 changes;
- no Current Synthesis route-enum changes;
- no Flynt, Officials & Outlaws, House, or Glaüshouse hierarchy changes;
- no new organizations, offices, mythology, or route names;
- no Godot scene implementation in this milestone;
- no terrain asset production;
- no speculative route mechanics beyond the supplied constitutional purposes.
