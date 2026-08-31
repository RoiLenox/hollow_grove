# Hollow Grove Adaptive Topology State V1

Status: implemented world-state contract layered above the approved Spatial Topology V2 geometry and Adaptive Midpoint Hinge V2 structure.

## Authority boundary

Spatial Topology V2 owns the nine route identities, endpoints, regions, and canonical geometry. The Adaptive Midpoint Hinge contract owns each double-slotted Midway and its circle-to-rectangle projection. Adaptive Topology State V1 owns only the currently committed pose and the transition lifecycle.

The state is persisted separately at `artifacts/hollow_grove_adaptive_topology_state_v1.json`. It does not rewrite frozen gameplay or compatibility archives.

## Deterministic lifecycle

Every morph uses the following boundary:

1. A caller supplies a typed trigger, a unique transition identity, a circular or rectangular target, and a tick duration.
2. The state authority prepares exactly one fixed-point pose for the next revision.
3. Collision and navigation systems rebuild against that prepared pose and return snapshot identities tied to its transition and revisions.
4. The state authority commits only matching evidence. Missing, altered, or stale evidence leaves the persisted state unchanged.
5. On the last tick, the transition identity is retained in the completed set so replay cannot apply it twice.

A seasonal request whose target is already satisfied is also durably recorded without rebuilding unchanged geometry. This prevents an old Winter record from reversing a later Spring transition during replay.

## Function Junction binding

The topology does not react directly to a date, player location, or unvalidated seasonal message. It consumes a completed Function Junction from the existing constitutional runtime, where both physical and digital checkpoints, authority, evidence, outgoing closure, and incoming opening have already been validated.

| Completed Function Junction | Topology result |
|---|---|
| Winter Solstice — **The Way Back** | Return to the canonical circle over four ticks. |
| Spring Equinox — **The Initiation** | Incarnate the locked rectangle over four ticks. |
| Summer Solstice — **The Gathering / Derrick** | Preserve the rectangular communal form. |
| Autumn Equinox — **The Festival of Mirrors** | Preserve the rectangular form while results are confirmed. |

Four ticks produce the approved 0%, 25%, 50%, 75%, and 100% visual poses. Every moving pose still requires matching collision and navigation rebuild evidence before commit. Transition and trigger identities are derived from the caller-supplied Function Junction identity, making replay deterministic without transferring seasonal sovereignty to the topology layer.

## Traveler continuity

Travelers attached to adaptive paths retain their constitutional route identity and fixed-point route progress. Their world position is re-projected from those two values after each committed pose. A traveler therefore follows the moving route without teleporting to a different named path or accumulating float drift.

Aura Ridge is the fixed central spine. The other eight routes are projected through their assigned double-slotted Midway hinges.

## Presentation boundary

The Godot screen shell polls the persisted state and renders its `rectangularity` value through the approved morph contract. Moving the UI slider enters an explicit developer override; it never mutates the persisted state. “Use world state” clears that override and reapplies the authoritative revision.

The approved five-checkpoint review mode remains presentation-only and suppresses the authority suffix and override button so its locked visual evidence stays unchanged.

## Live execution bridge

`adaptive_topology_live` is the transactional boundary between a completed Function Junction and Godot physics. A producer submits the full validated Function Junction archive plus the selected junction identity; the bridge revalidates the archive checksum and constitutional runtime before the seasonal adapter can prepare a transition.

The bridge uses three replace-in-place runtime artifacts:

1. `artifacts/hollow_grove_adaptive_topology_seasonal_event_v1.json` carries the validated archive and selected Function Junction identity.
2. `artifacts/hollow_grove_adaptive_topology_rebuild_command_v1.json` carries all nine kernel-projected route corridors for the next prepared pose. Every corridor has 33 fixed-point samples and participates in a digest that Godot independently recomputes.
3. `artifacts/hollow_grove_adaptive_topology_rebuild_ack_v1.json` records the exact command, transition, revisions, digest, collision snapshot, navigation snapshot, and realized geometry counts.

Godot turns every accepted command into real `CollisionPolygon2D` corridors and bakes their union into a `NavigationRegion2D`. A prepared command is acknowledged only after both structures have entered the scene tree and the navigation map has synchronized. Rust rejects stale, partial, altered, or zero-navigation acknowledgements; a matching acknowledgement commits the pose and atomically replaces both the durable state and the next rebuild command. A committed command also restores collision and navigation geometry when the scene starts, but does not require an acknowledgement.

`run-hueman-godot.sh` starts this bridge with the screen shell. A Function Junction producer can submit an accepted runtime archive directly:

```text
adaptive_topology_live submit \
  --root /path/to/hollow_grove \
  --archive /path/to/function_junction_archive.json \
  --junction function-junction-id
```

For supervision or diagnosis, `adaptive_topology_live step --root PATH` performs one transaction pass and `adaptive_topology_live serve --root PATH` runs the durable poll loop. Replaying the same seasonal event is safe: completed transitions and already-satisfied targets do not rebuild or recommit unchanged geometry.

The deterministic rehearsal input can be regenerated without touching world state:

```text
adaptive_topology_live export-rehearsal-archive \
  --output artifacts/hollow_grove_function_junction_rehearsal_archive_v1.json
```

The accepted Spring run is recorded in `HOLLOW_GROVE_ADAPTIVE_TOPOLOGY_LIVE_REHEARSAL_V1.md` and `artifacts/hollow_grove_adaptive_topology_live_rehearsal_v1.json`.

## Audit

Run:

```text
cargo run --features dev-tools --bin hg-dev -- adaptive_topology_state_audit
```

The audit consumes the canonical validated Winter, Spring, Summer, and Autumn Function Junction records. It records the already-circular Winter result, performs Spring's four-tick circle-to-rectangle transition, confirms Summer and Autumn preserve the form, supplies distinct collision and navigation rebuild identities for every moving pose, round-trips persistence after every commit, and reports a Current Sea traveler at constant route progress.
