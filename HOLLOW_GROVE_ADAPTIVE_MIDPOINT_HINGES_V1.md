# Hollow Grove Adaptive Midpoint Hinges V1

> Historical compatibility contract. The corrected current authority is
> `HOLLOW_GROVE_ADAPTIVE_MIDPOINT_HINGES_V2.md` and
> `HOLLOW_GROVE_SPATIAL_TOPOLOGY_V2.md`. V1 remains documented so its archive
> aliases can be migrated deterministically.

- Status: additive structural-geometry contract
- Kernel primitive: `hollow-grove-semantic-kernel::spatial_inheritance`
- World binding: `src/world/adaptive_midpoint_hinges.rs`
- Existing spatial archive: unchanged Spatial Inheritance V1
- Adaptive structure schema: `1`

## Purpose

Hollow Grove uses four double-slotted midpoint hinges to project the existing
quadrant routes from their canonical circular pose toward an adaptive
rectangular pose. The hinges change geometry only. They do not create route
identity, traversal, constitutional authority, House authority, offices, or
Stonebend Titles.

> One hinge body moves two isolated route slots. The slots never merge.

Aura Ridge and Current Sea remain the central Stonebend–Glaüshouse spine. They
do not occupy a quadrant hinge. Current Sea is not substituted for Current
Seanad at the southeast boundary.

## Canonical Midways and hinge-region titles

| Quadrant | Hinge-region title | House boundary | Inner Midway | Outer Midway |
|---|---|---|---|---|
| Northeast | Aura Field | Stonebend / Sandmanor | The Aura Midway | Mount Aura Midway |
| Southeast | Aura Beach | Sandmanor / Glaüshouse | Glausbahn Midway | Current Seanad Midway |
| Southwest | Flynt Basin | Glaüshouse / Flynt | Boardwalk Midway | Riptide Midway |
| Northwest | Stonebend Rise | Flynt / Stonebend | Basin Motor Speedway Midway | Stairway to Heaven Midway |

Every route appears in exactly one slot. Inner/Outer order is immutable and
does not grant priority or authority.

The names are display titles layered over stable route and hinge identities;
they do not change archive keys. `Aura Field` remains the singular geographic
surface. Within it, the physical `Aura Fields` and the adjacent `Content Farm`
are the two cultivation halves of Aura Farm. `Flynt Basin` and `Stonebend Rise`
are hinge-region titles within the existing left-side Aura Basin, not new
interior surfaces or new jurisdictions.

## Geometry law

Each slot derives its circular position by evaluating its canonical route at
parametric position `t = 0.5`:

- Straight routes use fixed-point linear interpolation;
- Round routes use the existing fixed-point quadratic Bézier control;
- a Round control point is never mistaken for a route midpoint.

The rectangular target is an isolated lane corner. Outer slots reach the
canonical rectangle corners. Inner slots remain inset by `0.045`, matching the
canonical route-band width:

| Hinge | Inner target | Outer target |
|---|---:|---:|
| Northeast | `(0.690, 0.185)` | `(0.735, 0.140)` |
| Southeast | `(0.690, 0.815)` | `(0.735, 0.860)` |
| Southwest | `(0.310, 0.815)` | `(0.265, 0.860)` |
| Northwest | `(0.310, 0.185)` | `(0.265, 0.140)` |

`rectangularity` is a normalized fixed-point value:

```text
0.0 = exact existing route geometry
0.5 = deterministic intermediate hinge pose
1.0 = two straight legs through the slot's rectangular target
```

For each route, the kernel performs an exact de Casteljau split at `t = 0.5`.
It then interpolates the split midpoint and both controls toward the
rectangular corner construction. At zero, the two quadratic segments reproduce
the original Straight or Round path exactly. At one, each segment's control is
its line midpoint, producing two exact straight legs. Intermediate values stay
deterministic and require no floating-point identity.

The hinge pivot is always the midpoint of the two adaptive slot positions.
The pivot may coincide with neither route. It is structural identity, not a
third path.

## Inheritance and replay

Hinge definitions use stable hinge and path identities. A hinge resolves
through the same cached frame transform as anchors, routes, and regions, so the
local pose remains identical at world, city, district, and deeper scales while
root coordinates inherit normally.

The adaptive definitions use a standalone canonical JSON contract and digest:

```text
ruleset.hollow-grove.adaptive-midpoint-hinges-v1
fnv1a64-adaptive-midpoint-hinges-v1:<digest>
```

Keeping this archive additive preserves byte-for-byte compatibility with the
frozen Spatial Inheritance V1 fixture and existing live replay digests.

## Validation lock

A conforming structure proves:

- exactly four hinge bodies and eight slots;
- exactly one Inner and one Outer slot per hinge;
- the two paths at a hinge share one endpoint pair;
- no path appears in two hinges;
- circular and rectangular slot positions remain distinct;
- all coordinates and interpolation use normalized fixed-point arithmetic;
- Aura Ridge and Current Sea remain outside the quadrant hinge roster;
- route identity and endpoints never change;
- no slot implies transfer into the other slot;
- serialization is deterministic and future schemas fail closed.

## Non-goals

- no gameplay exit or route-transfer changes;
- no constitutional route reclassification;
- no Current Sea / Current Seanad conflation;
- no live animation timing policy;
- no Godot rendering or collision integration in V1;
- no legal or sovereign meaning for the word “hinge.”

## Inspection

```bash
cargo run --quiet --features dev-tools --bin hg-dev -- adaptive_midpoint_hinge_audit
```

The audit prints the canonical definitions plus circle, halfway, and rectangle
pivots and slot positions.
