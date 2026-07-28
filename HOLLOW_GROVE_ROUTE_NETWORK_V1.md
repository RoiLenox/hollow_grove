# Hollow Grove Route Network V1

Status: implemented traversable skeleton
Constitutional purpose: `HOLLOW_GROVE_CONSTITUTIONAL_GEOGRAPHY_V2.md`
Screen orientation: `HUEMAN_SCREEN_MAP_v0.1.0.md`
Executable network: `src/world/route_network.rs`
Gameplay maps: `src/gameplay/world.rs`

## Layer Boundary

The route network answers:

- which Houses a route joins;
- whether its world geometry is Straight or Round;
- where its endpoints sit on the shared normalized map;
- whether a traveler may transfer from one route to another at a shared House;
- which immutable route record presentation receives.

Constitutional geography still answers why a route exists through its dominant
verb, purpose, direction, and process. Gameplay owns actual movement and
collision. Godot renders the immutable result. No route classification grants
House authority or changes Aura polarity.

Interior regions are a separate overlay. The three singular surfaces are
established by `AURA_FIELD_SURFACE_V1.md`, `AURA_BEACH_SURFACE_V1.md`, and
`AURA_BASIN_SURFACE_V1.md`. This network supplies their lawful approaches but
does not turn any surface into a route.

This skeleton retains three old map keys for replay compatibility. The
`aura-way.design-corridor` map is not permanent `route.aura-way`; the
`current-sea.deep-certification-landing` map is not `region.current-sea`; and
the `riptide.emergency-intake` map is not `force.riptide`. Controlling
directional and body/force/route law lives in
`CENTRAL_JUNCTION_SEASONAL_FUNCTIONS_V1.md`. Presentation and endpoint-sharing
for these legacy projections cannot redefine the permanent Aura Way route, the
Current Sea body, Riptide, Undertow, or Boardwalk.

## Shared House Anchors

The route skeleton uses the locked 16:9 normalized screen anchors, stored as
integer per-mille coordinates:

| House | Coordinate |
|---|---|
| Stonebend | `(500, 140)` |
| Flynt | `(265, 500)` |
| Glaüshouse | `(500, 860)` |
| Sandmanor | `(735, 500)` |

Integer coordinates keep route topology deterministic and replayable without
floating-point identity.

## Straight Routes

| Route | Endpoints | Verb | Gameplay map |
|---|---|---|---|
| Aura Ridge | Glaüshouse / Stonebend | Witness | `aura-ridge.grove-approach` |
| Aura Way | Stonebend / Sandmanor | Design | `aura-way.design-corridor` |
| Current Sea | Glaüshouse / Stonebend | Certify | `current-sea.deep-certification-landing` |
| Boardwalk | Glaüshouse / Flynt | Return | `boardwalk.return-vestibule` |
| Basin Motor Speedway | Flynt / Stonebend | Produce | `basin-motor-speedway.production-circuit` |

The row named Current Sea is a Straight civic-interface projection through
dense public circulation, competing claims, witnesses, and ordinary civic
pressure. Its stable gameplay map ID retains
`deep-certification-landing` so existing archives continue to replay. The
projection is not `region.current-sea`, the Current body that sets in
Glaüshouse.

## Round Routes

`Round` is the world geometry class rendered as a curved route. It is not Aura
polarity, moral alignment, Moxy, or Dark Aura.

| Route | Endpoints | Verb | Gameplay map |
|---|---|---|---|
| Mt. Aura | Stonebend / Sandmanor | Aspire | `mnt-aura.aspiration-path` |
| Current Seanad | Sandmanor / Glaüshouse | Deliberate | `current-seanad.deliberation-chamber` |
| Riptide | Flynt / Glaüshouse | Retrieve | `riptide.emergency-intake` |
| Stairway to Heaven | Flynt / Stonebend | Ascend | `stairway-to-heaven.ascent-path` |
| Glausbahn | Sandmanor / Glaüshouse | Refine | `glausbahn.refinement-span` |

Glausbahn is the Round road sector. Its curvature creates a repeated
design–prototype–repair–redesign circuit rather than a one-way span. Current
Seanad remains a deliberative place rather than ordinary transit even though
its screen geometry also belongs to the Round class. The Riptide row remains
an emergency-intake projection rather than elective travel; map entry is a
development witness for that interface, not the natural `force.riptide` and
not a claim that ordinary civilians choose emergency retrieval.

Current Sea remains constitutionally distinct from Current Seanad: Current Sea
certifies continuity in public circulation at the Glaüshouse–Stonebend
boundary; Current Seanad deliberates at the Glaüshouse–Sandmanor boundary.

## Traversal Law

A route transfer is valid when:

1. both route maps are canonical projections of constitutional routes and
   share at least one House endpoint; or
2. one map is an interior surface and the other route belongs to its declared
   approach set:
   - Aura Field: Aura Ridge, Aura Way, or Mt. Aura;
   - Aura Beach: Aura Ridge, Current Sea, Glausbahn, or Current Seanad;
   - Aura Basin: Aura Ridge, Current Sea, Boardwalk, Riptide, Basin Motor
     Speedway, or Stairway to Heaven.
3. one map is a route and the other is its declared extraction worksite:
   - Mt. Aura ↔ Mt. Aura High Mine;
   - Stairway to Heaven ↔ Stairway Burden Mine;
   - Riptide ↔ Riptide Current Recovery Rig;
   - Current Sea ↔ Current Sea Depth Production Rig.
4. Stairway Burden Mine and Highway to Hell Deepworks connect inside the same
   mountain complex. Highway to Hell remains a mine gallery, not an eleventh
   constitutional route.

Disconnected jumps fail without changing revision, position, event history, or
case state. Accepted map-entry events persist in federation-aware gameplay
archive schema V3 and replay through the same network check.

The playable client now uses a physical exit projected at each map spawn.
`Tab` selects among connected destinations and `B` traverses. Traversal away
from that exit coordinate fails without mutation. `EnterRegionIntent` remains a
protocol/archive compatibility operation, not the Godot travel control.

The Godot development cycle follows one continuous lawful circuit of
endpoint-sharing route transfers and declared surface approaches:

```text
Aura Ridge
→ Current Sea
→ Aura Beach
→ Glausbahn
→ Current Seanad
→ Aura Way
→ Aura Field
→ Mt. Aura
→ Basin Motor Speedway
→ Aura Basin
→ Stairway to Heaven
→ Boardwalk
→ Riptide
→ Aura Ridge
```

This cycle remains a traversal witness, not a new constitutional process
order. Mine and rig branches now leave and rejoin their exact routes rather
than extending the constitutional circuit.

## Presentation

Every route view exposes:

- stable route ID and display Name;
- Straight or Round geometry;
- presentation geometry term;
- two House endpoints;
- dominant verb;
- constitutional purpose;
- complete process stages.

Generic route maps contain an interactive constitutional witness. It explains
the route's actual verb and explicitly states that geometry is not authority.
Boardwalk and Current Sea retain their case-specific maps.

Interior surface views separately expose singular identity, dominant regional
attribution, exact polygon, declared route approaches, and complete facility
catalog. Route geometry never supplies those facts.

## Verification

`tests/route_network_gameplay.rs` proves:

- exactly five Straight and five Round routes;
- all ten constitutional routes have exactly one canonical gameplay map;
- fixed map dimensions and stable wire IDs;
- exact route-witness attribution;
- endpoint-sharing traversal;
- fail-closed disconnected jumps;
- schema-V3 save, explicit Runtime Federation binding, and deterministic replay;
- all three singular surfaces enter only from their declared approaches;
- surface maps remain distinct from the ten-route count.

Inspect the network with:

```bash
cargo run --quiet --bin hollow_grove_route_network_audit
```
