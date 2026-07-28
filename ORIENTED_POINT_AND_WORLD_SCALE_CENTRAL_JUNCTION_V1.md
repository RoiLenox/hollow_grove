# Oriented Point and World-Scale Central Junction V1

- Status: frozen universal physical primitive and Hollow Grove world binding
- Kernel layer: `hollow-grove-kernel::oriented_point`
- World archive: `HGPNT` version `1`
- Legacy orientation migration: `HGPNT` version `0` → version `1`
- Constitutional basis: the existing Hollow Grove Compromise

## 1. Controlling physical rule

> **Central Junction is the world-scale physical expression of the universal
> Point primitive.**

Every lawful Point contains a stable center, an oriented polarity axis, one
positive pole, one negative pole, a physical scale, evidence, and provenance.
The lawful axis reads:

`negative pole → center → positive pole`

The center is neither pole and is not a third pole.

## 2. Architecture

The universal kernel knows only generic physical types:

- `PointId`, `PointCenterId`, and `PoleId`;
- `PhysicalPosition` and `PolarityAxis`;
- `ScaleKey`, `PhysicalExtent`, and `PositiveScaleFactor`;
- `OrientedPoint`, `PointScaling`, and `PointInversion`;
- `ExpandedPointField` and `RelativePolarity`;
- generic authority, evidence, region, field, and composition identifiers.

It contains no Central Junction, Light Aura, Dark Aura, or House proper names.
Those names enter only through the Hollow Grove constitutional binding.

The existing recursion witness remains:

`Point → Triway → Fourway → HollowGrove → CurrentSeam → AuraBeam → Point²`

The existing `Point` now carries neutral oriented physical state and preserves
it through ordinary progression and Point² stabilization. A base Point is not
automatically Point². Point² remains the landed consequential relationship
that may organize an expressed field at a larger scale.

## 3. Polarity is not morality

Positive polarity tends toward outward expression, projection, exposure,
emergence, visibility, and expansion.

Negative polarity tends toward inward containment, depth, absorption,
concealment, incubation, and contraction.

The center supports crossing, neutrality, exchange, polarity transition,
balanced witness, and zero displacement along the axis.

None of these tendencies automatically establishes lawfulness. Positive may
be unlawful; negative may be lawful. Constitutional authority, evidence, and
restraint remain separate determinations.

## 4. Scale and inversion

The open `ScaleKey` accepts microscopic, material, object, entity, room,
district, region, world, and future caller-supplied scales.

Lawful scaling records a stable composition ID, source and result Point IDs,
source and result scales, strictly positive scale factor, authority, evidence,
and provenance. Scaling enlarges physical extent while preserving:

- center identity and position;
- positive-pole identity;
- negative-pole identity;
- axis direction;
- handedness;
- the complete source link.

Zero or negative numeric scale is rejected. Scaling cannot invert polarity.

Inversion is a distinct `PointInversion` operation. It requires a distinct
result Point identity, authority, evidence, and composition provenance. It
deterministically reverses the axis and swaps pole relationships. Camera
orientation, map rotation, sprite facing, insertion order, and serialization
order cannot perform inversion.

## 5. Expanded field and classification

An `ExpandedPointField` records three relational regions:

1. positive region;
2. center or zero seam;
3. negative region.

The center is the sign-changing seam, not a pole.

`RelativePolarity` is calculated by integer projection of a physical
displacement onto the Point axis:

- projection > 0 → Positive;
- projection = 0 → Center;
- projection < 0 → Negative.

The kernel does not hard-code north as positive.

## 6. Hollow Grove world binding

The stable world identities are:

- `point.world.hollow-grove`;
- `field.world.hollow-grove`;
- `pole.world.positive`;
- `pole.world.negative`;
- `region.light-aura`;
- `region.central-junction`;
- `region.dark-aura`.

Hollow Grove explicitly binds its generic world axis:

```text
north / top / positive → Light Aura
center / zero seam     → Central Junction
south / bottom / negative → Dark Aura
```

Central Junction is therefore the expanded physical center of the world Point.
It is where polarity changes sign, regions meet, routes cross, Great Functions
receive shared witness, and Current/Aura relations may be measured. It is
neither pole and does not receive automatic sovereignty from its centrality.

## 7. Fixture, archive, and migration

The golden fixture begins with `point.seed.hollow-grove` at `scale.object` and
lawfully scales it to `point.world.hollow-grove` at `scale.world`. The Points
have distinct identities while sharing center, poles, axis, handedness, and
provenance. It expands the world field, classifies positive/center/negative
positions, and performs one explicitly identified inversion probe without
mutating the canonical world Point.

`HGPNT` version `1` uses deterministic JSON and an FNV-1a digest. Replay
reconstructs scaling, validates the exact geographic binding, validates all
field relationships, executes the explicit inversion probe, and rejects
duplicate or contradictory records.

Version `0` models a legacy unoriented Point. Migration succeeds only with the
stable decision `migration.world-point.orientation-v0-to-v1` and evidence
explicitly binding north/top to positive and south/bottom to negative.
Ambiguous legacy orientation is rejected. Migration never infers pole roles
from insertion order.

The complete annual `HGSEA` archive embeds the same world payload so seasonal
venues and route relationships consume, rather than recreate, the world field.

## 8. Audit and deferral

Run:

`cargo run --bin hollow_grove_world_point_audit`

The audit fails on duplicate poles, center/pole collapse, scaling inversion,
incorrect Light/Central/Dark binding, replay drift, checksum failure, improper
lawfulness inference, presentation control of polarity, or Hollow Grove proper
names in the universal kernel.

This milestone adds no shaders, map skins, camera effects, gradients,
particles, Godot scenes, pole markers, seasonal lighting, or polarity combat.
Presentation must later render accepted state rather than define it.
