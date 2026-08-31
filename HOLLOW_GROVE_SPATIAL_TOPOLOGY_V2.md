# Hollow Grove Spatial Topology V2

- Status: implemented corrected spatial authority
- Executable contract: `src/world/spatial_topology_v2.rs`
- Adaptive hinges: `src/world/adaptive_midpoint_hinges.rs`
- Presentation overlay: `artifacts/hollow_grove_spatial_topology_v2_overlay.json`
- Adaptive morph contract: `artifacts/hollow_grove_adaptive_morph_v1.json`
- Frozen compatibility source: Spatial Inheritance V1
- Schema: `2`

## Canonical structure

Spatial Topology V2 has nine constitutional routes. Aura Ridge is the only
central spine. The remaining eight routes occupy four double-slotted Midway
hinges.

| Quadrant | Region title | Straight / Inner | Curved / Outer |
|---|---|---|---|
| Northeast | Aura Field | The Aura Midway | Mount Aura Midway |
| Southeast | Aura Beach | Glausbaughn Midway | Current Sea Midway |
| Southwest | Flynt Basin | Boardwalk Midway | Riptide Midway |
| Northwest | Stonebend Rise | Basin Motor Speedway Midway | Stairway to Heaven Midway |

`Current Seanad` is not a route. It was a typo for Current Sea. `Glausbahn` is
the legacy spelling of Glausbaughn. Neither legacy identity is emitted by V2.

## Aura Farm partition

The upper-right composite triangle is Aura Farm:

| Region | Ordered per-mille polygon |
|---|---|
| Aura Farm | `(500,140)` → `(735,500)` → `(500,500)` |
| Aura Field | `(500,140)` → `(735,500)` → `(500,320)` |
| Content Farm | `(500,320)` → `(735,500)` → `(500,500)` |
| Aura Beach | `(500,500)` → `(735,500)` → `(500,860)` |

The split point `(500,320)` is the midpoint of the Stonebend–Junction edge.
The median to Sandmanor divides Aura Farm into two exact equal-area triangles.
Content Farm's lower edge is Aura Beach's complete upper edge, so the two
regions are adjacent without a gap or overlap.

The old `aura-field` gameplay surface remains a compatible entry into this
composite. V2 views expose Aura Farm plus both typed subregions. Content Farm
is a subregion with an inspectable entry, not merely one facility marker.

## Corrected route geometry

| Route | Geometry | Endpoints |
|---|---|---|
| Boardwalk | Straight | Flynt / Glaüshouse |
| Riptide | Round | Flynt / Glaüshouse |
| Aura Ridge | Straight | Glaüshouse / Stonebend |
| Glausbaughn | Straight | Sandmanor / Glaüshouse |
| Current Sea | Round | Sandmanor / Glaüshouse |
| Aura Way | Straight | Stonebend / Sandmanor |
| Mount Aura | Round | Stonebend / Sandmanor |
| Basin Motor Speedway | Straight | Stonebend / Flynt |
| Stairway to Heaven | Round | Flynt / Stonebend |

Current Sea uses the quadratic control formerly stored on the typo route:
`(0.730, 0.800)`. Glausbaughn has no curve control.

## Compatibility and migration

Spatial Inheritance V1 remains byte-for-byte replayable. V2 has a separate
template, rule set, archive, and digest. Migration validates V1 first and then
emits the deterministic V2 topology.

Decode-only aliases are:

```text
geography.route.current-seanad -> geography.route.current-sea
geography.route.glausbahn      -> geography.route.glausbaughn
```

The obsolete V1 straight Current Sea projection and curved Glausbahn geometry
are not emitted as canonical V2 paths. Adaptive-hinge V1 archives migrate the
same two route identities before V2 validation.

## Validation lock

V2 proves:

- exactly nine canonical routes: five Straight and four Round;
- exactly five regions, including Aura Farm and its two typed halves;
- equal Aura Field and Content Farm area;
- exact Aura Farm coverage with no partition gap;
- exact Content Farm / Aura Beach shared edge;
- Current Sea is curved and Glausbaughn is straight;
- Current Seanad and Glausbahn are absent from canonical output;
- eight isolated Midway slots and one unhinged Aura Ridge spine;
- canonical circular geometry at `0%` and exact rectangular symmetry at `100%`;
- fixed anchors, endpoints, and region polygons throughout the morph;
- deterministic archive replay and fixed-point inherited projection;
- frozen V1 archive bytes remain unchanged.

## Inspection

```bash
cargo run --quiet --features dev-tools --bin hg-dev -- spatial_topology_v2_audit
cargo run --quiet --features dev-tools --bin hg-dev -- adaptive_midpoint_hinge_audit
cargo run --quiet --features dev-tools --bin hg-dev -- aura_field_audit
```
