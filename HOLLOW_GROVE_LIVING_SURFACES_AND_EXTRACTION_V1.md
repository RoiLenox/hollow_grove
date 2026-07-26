# Hollow Grove Living Surfaces and Extraction V1

Status: implemented event-sourced gameplay layer
Backbone: `HOLLOW_GROVE_COMPROMISE_V1_DRAFT.md`
Surface authority: `AURA_FIELD_SURFACE_V1.md`,
`AURA_BEACH_SURFACE_V1.md`, `AURA_BASIN_SURFACE_V1.md`
Route authority: `HOLLOW_GROVE_CONSTITUTIONAL_GEOGRAPHY_V2.md`
Executable contracts: `src/gameplay/living_world.rs`,
`src/world/extraction.rs`

## Purpose

This layer makes Hollow Grove continue to exist after the player stops looking
at it.

It adds:

- a persistent day-and-shift clock;
- deterministic Aura weather;
- changing agricultural, coastal, Basin, mine, and rig conditions;
- ten scheduled working people with explicit limits;
- seven evidence-gated cases;
- solid-seam mining at Mt. Aura and in the Stairway mountain complex;
- the Highway to Hell descending mine road;
- offshore Current-well work on Riptide and Current Sea;
- material provenance and custody;
- cross-region consequences;
- physical map exits;
- checksummed save and exact event replay;
- Godot weather, condition, case, mine, rig, and exit projections.

The layer is above constitutional law and below presentation. It consumes
House and route meaning; it does not replace either.

## Stable Geographic and Route Lock

There remain exactly:

- one Aura Field;
- one Aura Beach;
- one Aura Basin;
- ten constitutional routes.

Mines and rigs are working sites attached to routes. They are not new Houses,
regions, routes, or kernel layers.

**Highway to Hell** is the descending industrial gallery inside the
Stairway-to-Heaven mountain complex. The paired movement is:

```text
Stairway to Heaven
    public ascent, recognized burden, higher exposure

Highway to Hell
    industrial descent, buried consequence, heat, gas, water, and return
```

The Stairway remains the constitutional `Ascend` route. Highway to Hell is
neither an eleventh route nor a moral inverse. Going down does not make a
worker evil; going up does not make a claimant lawful.

## The Living Clock

The clock advances through:

```text
Dawn → Day → Dusk → Night → next Dawn
```

Each shift event records its exact prior clock, next clock, and deterministic
weather. The weather cycle uses:

- Clear;
- Crosswind;
- Pressure Drop;
- Storm.

Weather changes observable state. It can dry soil, advance a harvest, improve
visibility, erode dunes, increase storm pressure, or move contamination.
Unresolved incidents also worsen on a shift. A Riptide blowout, for example,
continues to reduce fish stock and increase Basin contamination until a lawful
response commits.

No client invents time or weather. Save/load replays the same sequence and
reconstructs the same conditions.

## Aura Field State

The one Aura Field persistently records:

| Condition | Meaning |
|---|---|
| soil moisture | water presently held by the worked soil |
| irrigation reserve | rationable water available through the irrigation works |
| crop health | combined crop stress and recovery projection |
| livestock health | welfare state of animals using paddock and barn |
| granary reserve | protected food and seed capacity |
| labor available | people presently available for field work |
| harvest ready | mature work that can become a recorded harvest |

### Case: drought allocation

Stable ID: `case.aura-field.drought-allocation.v1`

Evidence:

1. irrigation water gauge;
2. proving-plot soil probe;
3. granary reserve ledger.

Lawful operational postures:

- **Equitable Ration** shares immediate loss across crops, animals, and
  households;
- **Protect Seed Reserve** accepts a harsher present crop loss to preserve the
  next planting.

**Maximize Immediate Yield** is present as a visible refusal path. Under the
active conditions it would conceal reserve and welfare costs, so the reducer
rejects it transactionally.

The player gathers evidence and may support a posture. The recorded public
water steward remains the decision-maker. Presence in Aura Field grants no
stewardship.

## Aura Beach State

Aura Beach persistently records:

| Condition | Meaning |
|---|---|
| tide height | present shore and landing exposure |
| storm pressure | approaching coastal hazard |
| visibility | navigable and rescuable sight distance |
| shore traffic | people, craft, and cargo presently exposed |
| rescue readiness | remaining trained response capacity |
| fish stock | visible ecological pressure, not a promise of future catch |
| dune integrity | storm protection and habitat continuity |
| public access closure | temporary safety closure, never permanent title |

### Case: storm rescue

Stable ID: `case.aura-beach.storm-rescue.v1`

Evidence:

1. tide record;
2. weather record;
3. rescue manifest.

Lawful postures:

- **Close and Shelter** protects the greatest number and holds rescue capacity;
- **Guided Rescue** exposes a bounded rescue team to retrieve people already
  at risk.

**Keep Shore Open** is refused while the recorded storm condition makes open
traffic disproportionate. A warning may close public access temporarily; it
cannot become permanent shore ownership.

## Aura Basin State

Aura Basin persistently records:

| Condition | Meaning |
|---|---|
| wildlife health | living territorial and ecological condition |
| territorial pressure | active conflict and crossing stress |
| injured Beings | identified living subjects needing aid |
| damaged Frames | embodied equipment or Frame work needing recovery |
| contamination | Hollow, spill, waste, and exposure pressure |
| salvage backlog | Objects awaiting identity, claim, assay, repair, or return |
| rescue readiness | remaining Basin response capacity |

### Case: injured Being or salvage

Stable ID: `case.aura-basin.injured-being.v1`

Evidence:

1. vital signs;
2. continuity record;
3. competing salvage claim.

Lawful postures:

- **Transfer to Care** moves the Being under narrow recorded custody toward
  qualified Glaüshouse care;
- **Stabilize in Place** uses Basin capacity when movement is the greater
  immediate hazard.

**Salvage the Subject** is always refused. No claim, victory, transformation,
damage, or location converts a Being into salvage. The salvage claim remains
evidence of the attempted misclassification.

## Solid-Seam Mining

Solid-seam mining is the coal-side analogy:

```text
survey and Name the working boundary
→ prove support and ventilation
→ record the crew entering
→ cut the solid face
→ support, pump, and ventilate
→ hoist under measured load
→ assay and grade
→ form a custody lot
→ deliver, reserve, challenge, or refuse
```

Every land mine contains:

- Survey and Name Office;
- headframe;
- ventilation house;
- hoist and cage;
- working face;
- refuge chamber;
- pump station;
- grade and custody yard.

### Mt. Aura High Mine

Map: `mnt-aura.high-mine`
Resource: Aura-bearing stone
Route: Mt. Aura / `Aspire`

The High Mine makes aspiration materially accountable. A summit claim is not
proof that the rock below it is safe to cut.

Its first case is a roof fall:

- evidence: survey, support inspection, crew roll;
- lawful choices: reinforce and continue at reduced rate, or withdraw the
  crew;
- refused choice: blast through the fall.

Reinforcement produces a small assayed stone lot. Under recorded custody, that
lot can improve Aura Field irrigation and Aura Beach dune protection. This is
a cross-region material consequence, not automatic mineral ownership.

### Stairway Burden Mine

Map: `stairway-to-heaven.burden-mine`
Resource: burden ore
Route: Stairway to Heaven / `Ascend`

The Burden Mine asks whether recognized capability can accept extraction work
without claiming the entire ascent. Hoist capacity, public passage, refuge,
and the right to refuse overload stay visible.

The mine operates persistently even though its first authored incident is
centered in the linked deepworks. Its output, crew, integrity, hazard, and
contamination remain available to later Stairway cases.

### Highway to Hell Deepworks

Map: `highway-to-hell.deepworks`
Resource: deep iron
Container: Stairway-to-Heaven mountain complex

The Deepworks descend below the public ascent through heat, gas, flooding,
roof convergence, and buried labor. They are the place where the benefits of
ascending infrastructure meet responsibility for what was cut underneath it.

Its first case is a gas pocket:

- evidence: gas reading, ventilation log, escape check;
- lawful choices: seal and vent, or evacuate and flood;
- refused choice: continue cutting.

Deep iron may move to the Aura Basin Frame Recovery Garage only as a named,
assayed, custody-bearing lot. The Highway is never a disposal chute for
unrecorded bodies, Hollow, waste, or failed equipment.

## Offshore Current-Well Work

Offshore drilling is the petroleum-side analogy:

```text
survey a deep pressure structure
→ drill through a controlled well
→ test pressure and casing
→ retrieve or produce Current-bearing fluid
→ separate water, gas, sediment, and hazards
→ sample and assay
→ certify or quarantine
→ seal custody at the manifold
→ transport under spill and rescue obligations
```

Every rig contains:

- drill floor;
- depth derrick;
- pressure-control house;
- Current separator;
- spill-boom depot;
- dive and rescue bay;
- depth certification laboratory;
- custody transfer manifold.

### Current and living blood

The universal law remains `Current = blood`.

`Current-bearing brine` is the working extraction classification for the
circulating deep fluid encountered by these wells. It does not declare the
fluid ownerless and does not prove that it is safe or nonliving.

Every custody lot therefore records `living_blood_excluded = true`. This means:

1. the sample has no evidence of blood taken from a living Being;
2. extraction from a Being was not the source process;
3. any contrary identity, continuity, tissue, consent, or provenance evidence
   immediately stops commodity handling;
4. the material then enters quarantine, rescue, care, Illegal Hollowing, or
   identity review as appropriate.

The field is a protection, not a metaphysical shortcut. It cannot be set false
for a marketable lot.

### Riptide Current Recovery Rig

Map: `riptide.current-recovery-rig`
Resource: recovered Current-bearing brine
Route: Riptide / `Retrieve`

Riptide is emergency first. Its rig retrieves leaking fluid, endangered crews,
and failed equipment. Recovered fluid is quarantined and cannot masquerade as
normal production.

Its first case is a blowout:

- evidence: well pressure, spill extent, crew manifest;
- lawful choices: shut in and retrieve, or rescue crew first while containment
  remains incomplete;
- refused choice: continue flow.

An unresolved blowout damages Aura Beach fish stock and increases Aura Basin
contamination. Shut-in and recovery reduce both pressures. The recovered lot
moves to the Current Sea certification laboratory under quarantine rather than
directly to market.

### Current Sea Depth Production Rig

Map: `current-sea.depth-production-rig`
Resource: certified Current-bearing brine
Route: Current Sea / `Certify`

Current Sea tests sustained depth integrity. It does not replace Riptide
rescue, Stonebend naming, Sandmanor proof, Glaüshouse Clearance, or Flynt
recognition.

Its first case is a production-well certification:

- evidence: pressure test, sample assay, custody chain;
- lawful choices: certify reduced-rate operation, or suspend for repair;
- refused choice: bypass certification.

A certified reduced-rate lot can support Beach and Basin rescue logistics. The
certificate describes the tested well and lot; it does not grant ownership,
Title, office, or immunity from a later shutdown.

## Material Custody

Every material lot answers:

- what stable lot ID identifies it;
- what resource and quantity it contains;
- what unit was used;
- where it originated;
- who claims an interest;
- who presently holds custody;
- where it is going;
- whether it is quarantined, assayed, certified, in transit, or delivered;
- whether living blood has been excluded;
- what survey, incident, assay, and transfer facts establish provenance.

Custody is never ownership. A custodian may be obligated to hold something
they cannot use, sell, destroy, rename, or recognize.

Initial authored lots are:

- Mt. Aura Aura-stone lot to the irrigation works;
- Highway deep-iron lot to the Basin Frame Recovery Garage;
- Riptide recovered-brine lot to Current Sea quarantine;
- Current Sea certified-brine lot to the common boat landing.

Failed and refused cases form no fictional lot.

## People and Schedules

The first scheduled roster is intentionally small and concrete:

| Person | Work | Main locations |
|---|---|---|
| Brindle Reed | Gnome irrigation tender | Aura Field |
| Sella Windward | Elf weather reader | Aura Beach / Current Seanad |
| Harrow Vale | Gargoyle Basin rescue warden | Aura Basin |
| Oren Pike | Gerald mine surveyor | Mt. Aura High Mine / Mt. Aura |
| Maela Downroad | deepworks ventilation keeper | Highway to Hell |
| Bram Burden | Stairway hoist operator | Burden Mine / Stairway |
| Corin Wake | Merman well diver | Riptide rig / Aura Beach |
| Iona Depth | Current Sea pressure technician | Current Sea rig |
| Pel Marrow | material custody clerk | mines / Aura Basin |
| Tess Breakwater | shore and rig rescue liaison | Beach / both rigs |

Each roster entry has a Dawn, Day, Dusk, and Night location plus an explicit
authority limit. A schedule says where a person may be found; it does not make
them an office-holder.

## Cross-Region Consequence Graph

```text
Mt. Aura reinforced stone
├── strengthens Aura Field irrigation
└── strengthens Aura Beach dunes

Highway deep iron
└── supports Aura Basin Frame recovery

Riptide blowout
├── harms Aura Beach fish stock
└── raises Aura Basin contamination

Riptide shut-in
├── protects the coast and Basin
└── sends quarantined fluid to Current Sea

Current Sea certification
├── supports Aura Beach rescue logistics
└── supports Aura Basin rescue logistics

Beach guided rescue
└── transfers a living care burden into the Basin/Glaüshouse loop
```

These links change numeric, custody, case, and access-visible state. They never
rewrite a prior event.

## Physical Traversal

Every map projects a physical exit coordinate and its legally connected
destinations. `TraverseExitIntent` succeeds only while Hueman stands on that
coordinate.

Route-to-mine connections are exact:

- Mt. Aura ↔ Mt. Aura High Mine;
- Stairway to Heaven ↔ Stairway Burden Mine;
- Stairway Burden Mine ↔ Highway to Hell Deepworks;
- Riptide ↔ Riptide Current Recovery Rig;
- Current Sea ↔ Current Sea Depth Production Rig.

The compatibility `EnterRegionIntent` remains in protocol V1 for archives and
diagnostics. The Godot client uses only physical exits.

Godot controls:

- `Tab`: select another connected exit;
- `B`: traverse the selected physical exit;
- `T`: advance one living-world shift;
- action key: inspect a facility and gather its case evidence;
- `1` or `2`: record nonbinding support for a lawful local response after all
  evidence;
- `3`: exercise the authored refusal path and see it fail closed;
- `C`: ask the responsible duty officer to commit the supported response;
- `F5` / `F9`: save/load.

## Event and Replay Law

The living aggregate accepts only:

- `EvidenceObserved`;
- `CaseSupportRecorded`;
- `CaseResolved`;
- `ShiftAdvanced`.

Every event:

1. validates against the current aggregate;
2. applies to a cloned candidate;
3. rejects without mutation on failure;
4. increments the living revision once;
5. enters the authoritative gameplay event stream;
6. is covered by the gameplay archive checksum;
7. replays against the same canonical initial state.

A case cannot resolve without its exact evidence set. Evidence cannot be filed
under the wrong case. An unsafe choice cannot produce an outcome ID, custody
lot, revision, or partial consequence.

## Verification

`tests/living_surfaces_extraction.rs` proves:

- three solid mines and two offshore wells remain distinct;
- Highway to Hell is nested under Stairway rather than counted as a route;
- all extraction facilities have reachable map witnesses;
- physical exits bind each site to its exact route;
- leaving away from the exit tile fails transactionally;
- all seven cases require evidence;
- every authored unsafe choice fails without mutation;
- cross-region effects change the intended states;
- custody excludes living blood and preserves provenance;
- clocks, weather, schedules, and unresolved damage are deterministic;
- living events persist in checksummed gameplay saves and replay exactly.

Inspect the canonical state with:

```bash
cargo run --quiet --bin living_world_audit
```

The first character-driven consumer of this substrate is the implemented Deep
Pressure campaign. It links all seven outcomes into named-character memory,
classified speech, a cross-region evidence journal, Boardwalk settlement,
finite recovery Bonds, and persistent aftermath without changing this
aggregate's local authority. See
`HOLLOW_GROVE_DEEP_PRESSURE_CAMPAIGN_V1.md`.
