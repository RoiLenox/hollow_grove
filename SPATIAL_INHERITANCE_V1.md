# Hollow Grove Spatial Inheritance V1

- Status: canonical spatial-composition and live-projection contract
- Contract schema: `1`
- Rule set: `ruleset.hollow-grove.spatial-inheritance-v1`
- Kernel primitive: `hollow-grove-semantic-kernel::spatial_inheritance`
- World binding: `src/world/spatial_inheritance.rs`
- Live projection: `src/bin/hollow_grove_niri_bridge.rs`
- Presentation consumer: `hueman_godot/scripts/hueman_screen_shell.gd`
- Runtime verifier: `src/bin/hollow_grove_runtime.rs`
- Executable conformance proof: `tests/spatial_inheritance.rs`

## 1. Status and Precedence

This document defines how one spatial plan is inherited through the Hollow
Grove world, each House city, and every lawful subordinate scale.

It is beneath:

1. the Hollow Grove Compromise;
2. House constitutional law;
3. Constitutional Geography;
4. Route Network authority.

It does not amend those authorities. If a spatial declaration conflicts with a
route identity, endpoint, Straight/Round classification, House placement, or
constitutional purpose, the spatial declaration is invalid.

The short rule is:

> Geometry may inherit. Constitutional meaning does not arise from
> inheritance.

## 2. Purpose

Spatial inheritance exists to answer:

> How can the same lawful local plan remain identical at every spatial scale
> while each instance retains a unique place, lineage, authority basis, and
> root coordinate?

The contract prevents every House, district, site, room, object, or detail from
copying and gradually changing the main map. A child frame references one
immutable template and receives a deterministic transform into its parent and
the root.

The relationship is intentionally similar to a C foundation extended through a
C++ application layer:

- the kernel owns small neutral values and deterministic operations;
- the world layer binds those values to named Hollow Grove geography;
- presentation consumes published projections;
- authority remains in its original constitutional source.

Rust implements the relationship through composition, stable identities, and
validated publication rather than class inheritance.

## 3. Responsibilities

Spatial inheritance SHALL:

- possess one versioned rule-set identity;
- possess one versioned archive schema;
- use caller-controlled stable identities;
- distinguish templates, frames, anchors, paths, regions, scales, and
  viewpoints;
- preserve one immutable local topology for every inheriting frame;
- record each frame's parent;
- record each frame's semantic scale;
- record each frame's bounds in its parent;
- record placement authority;
- record at least one placement-evidence reference;
- validate the parent before publishing a child;
- reject ancestry cycles;
- reject identity reuse;
- reject missing templates, parents, anchors, path endpoints, malformed path
  controls, zero-width path bands, and malformed region polygons;
- validate the exact Hollow Grove scale progression;
- compose every child transform into a cached root transform;
- resolve local coordinates to root coordinates deterministically;
- reverse a contained root coordinate into a local coordinate deterministically;
- expose every candidate frame for a root point;
- choose one candidate with a versioned deterministic selection rule;
- preserve root and local coordinates together;
- preserve frame, template, scale, depth, and rule-set identity;
- preserve the catalog, template, and world replay digests;
- serialize canonical declarations without derived caches;
- replay a catalog independent of declaration insertion order;
- recompute every derived transform during replay;
- reject unsupported schema or rule-set drift;
- survive process restart without identity, topology, or coordinate drift;
- support arbitrary repeated `Detail` inheritance;
- keep the neutral kernel free of Hollow Grove proper names;
- permit deterministic conformance and tamper tests.

## 4. Non-responsibilities

Spatial inheritance SHALL NOT:

- create a House;
- create a city merely because a rectangle exists;
- name or rename a route;
- change a route's endpoints;
- change Straight to Round or Round to Straight;
- assign a route's constitutional verb or purpose;
- grant House, institutional, judicial, clinical, or federal authority;
- create evidence;
- treat a coordinate as proof;
- treat containment as ownership;
- treat a parent frame as owner of its child;
- treat a child as a copy of its parent's history;
- copy mutable topology into every frame;
- mutate a published declaration;
- reuse an identity for a correction or replacement;
- infer a missing parent, scale, authority, or evidence record;
- depend on wall-clock time, randomness, memory address, hash-map iteration
  order, or filesystem enumeration order;
- claim that recursive mapping alone improves renderer, compositor, audio,
  collision, or pathfinding performance;
- turn a viewpoint into a Bond, Title, Recipe, transformation, or accession.

## 5. Constitutional Questions

The contract answers:

- Which template supplies the local plan?
- Which frame is being addressed?
- Which parent contains that frame?
- At which named scale does it exist?
- Which bounds place it in its parent?
- Which authority declared the placement?
- Which evidence supports that declaration?
- Which anchor, if any, supplied the canonical placement?
- Which frames contain a root point?
- Why was one frame selected?
- What is the point's root coordinate?
- What is the point's coordinate inside the selected frame?
- What is the complete lineage from root to selected frame?
- Which rule set and archive schema govern replay?
- Does a replay produce the same topology, transform, and digest?
- Has a live client altered or omitted any part of the published scope?

It does not answer:

- Who constitutionally owns the place?
- Who may enter it?
- What consequences occur there?
- Whether a Bond is lawful?
- Whether a route action is permitted?

Those questions remain with their governing domains.

## 6. Required Domain Model

### 6.1 Normalized coordinate

A normalized coordinate is an integer in the inclusive range:

```text
0 .. 1,000,000,000
```

The value represents `[0, 1]` with nine decimal places of fixed-point
precision. Floating-point values are presentation projections only.

### 6.2 Normalized point

A point contains exactly:

- normalized `x`;
- normalized `y`.

The origin is top-left. This orientation is a coordinate convention, not
constitutional direction or polarity.

### 6.3 Normalized rectangle

A rectangle contains:

- minimum point;
- maximum point.

Both axes MUST have positive extent. Minimum and maximum boundaries are
inclusive for containment. The root rectangle MUST be the complete unit
rectangle.

### 6.4 Spatial template

A template contains exactly:

- immutable template identity;
- a sorted set of uniquely identified anchors;
- a sorted set of uniquely identified paths;
- a sorted set of uniquely identified regions.

Every path contains:

- immutable path identity;
- two distinct existing anchor identities;
- neutral geometry classification `Straight` or `Round`;
- ordered local control points;
- positive local route-band width.

A `Straight` path has no control points. A V1 `Round` path has exactly one
quadratic control point. A renderer may sample the curve at any resolution,
but may not replace that canonical control point.

Every region contains:

- immutable region identity;
- an ordered local polygon with at least three distinct vertices;
- nonzero polygon area.

A template contains topology, not placement. It has no parent and no root
coordinate.

### 6.5 Spatial frame

A frame contains exactly:

- immutable frame identity;
- optional parent-frame identity;
- template identity;
- semantic `ScaleKey`;
- bounds in the parent.

The root has no parent and fills the unit rectangle. Every non-root frame has
exactly one existing parent.

### 6.6 Hollow Grove spatial declaration

The world-facing declaration contains:

- frame identity;
- optional parent-frame identity;
- canonical template identity;
- typed Hollow Grove level;
- bounds in parent;
- placement-authority identity;
- one or more placement-evidence identities;
- optional canonical source anchor.

The declaration is the authoritative world-layer statement. The kernel frame
is its neutral geometric projection.

### 6.7 Cached transform

A cached transform contains the frame's fully composed bounds in the root.
It is derived state. It is never archived as authority.

### 6.8 Viewpoint

A resolved viewpoint contains:

- frame identity;
- template identity;
- scale identity;
- depth;
- local point;
- root point.

A live viewpoint additionally carries:

- rule-set identity;
- selection-policy identity;
- candidate count;
- catalog digest;
- template digest;
- world replay digest.

## 7. Identity

Every template, frame, anchor, path, region, scale, authority, evidence item,
and rule set uses a stable key.

A stable spatial identity MUST:

- be explicit;
- be immutable;
- use the stable lowercase identifier grammar;
- be independent of insertion order;
- be independent of memory location;
- be independent of wall-clock time;
- be independent of randomness;
- not be derived from mutable display text;
- remain resolvable after process restart.

Reusing an identity for a correction, moved frame, changed parent, changed
scale, changed template, changed bounds, changed authority basis, or migrated
semantic replacement is forbidden.

A changed topology requires a new template identity and a declared migration.
A changed placement requires a new frame identity or a versioned successor
declaration. Silent in-place correction is forbidden.

## 8. Canonical Template

Hollow Grove owns exactly one inherited local template:

```text
template.hollow-grove.main-map
```

It contains:

- Ranina at `(0.500, 0.500)`;
- Aura Ridge Junction at `(0.500, 0.500)`;
- Stonebend at `(0.500, 0.140)`;
- Sandmanor at `(0.735, 0.500)`;
- Glaüshouse at `(0.500, 0.860)`;
- Flynt at `(0.265, 0.500)`;
- all ten Route Network paths;
- the local Aura Ridge East screen projection;
- each route's existing endpoints;
- each route's existing Straight/Round classification;
- exact route-band width `0.045`;
- exact Round control points;
- the exact Aura Field, Aura Beach, and Aura Basin polygons.

Constitutional route identity, endpoints, and Straight/Round class MUST be
reconstructed from Route Network authority. Screen-local route shape,
route-band width, the non-constitutional Aura Ridge East projection, and
surface polygons MUST match the Hueman Screen Map contract. Neither source may
override the other's authority.

The complete V1 path plan is:

| Stable path identity | Class | Endpoints | Controls |
|---|---|---|---|
| `geography.route.aura-ridge` | Straight | Stonebend / Glaüshouse | none |
| `geography.route.aura-way` | Straight | Stonebend / Sandmanor | none |
| `geography.route.current-sea` | Straight | Glaüshouse / Stonebend | none |
| `geography.route.boardwalk` | Straight | Glaüshouse / Flynt | none |
| `geography.route.basin-motorspeedway` | Straight | Flynt / Stonebend | none |
| `geography.route.mnt-aura` | Round | Stonebend / Sandmanor | `(0.690, 0.200)` |
| `geography.route.current-seanad` | Round | Sandmanor / Glaüshouse | `(0.730, 0.800)` |
| `geography.route.riptide` | Round | Glaüshouse / Flynt | `(0.310, 0.800)` |
| `geography.route.stairway-to-heaven` | Round | Flynt / Stonebend | `(0.310, 0.200)` |
| `geography.route.glausbahn` | Round | Sandmanor / Glaüshouse | `(0.610, 0.680)` |
| `projection.route.aura-ridge-east` | Straight, local only | Aura Ridge Junction / Sandmanor | none |

Ranina and Aura Ridge Junction intentionally retain separate identities at the
same coordinate. Coordinate equality does not imply identity equality. Human
Core remains a rendered overlay, not an additional topological anchor.

Glausbahn uses the inner reciprocal curve and Current Seanad uses the outer
deliberative curve. Their shared House endpoints do not merge their identities.
Current Sea remains a Straight Glaüshouse–Stonebend path and MUST NOT reuse the
Current Seanad identity.

The complete V1 region plan is:

| Stable region identity | Ordered polygon |
|---|---|
| `region.aura-field` | Stonebend → Sandmanor → Ranina |
| `region.aura-beach` | Sandmanor → Glaüshouse → Ranina |
| `region.aura-basin` | Stonebend → Flynt → Glaüshouse |

The world frame and every inherited frame reference the same template
identity. They do not own private anchor, path, or region copies.

## 9. Mt. Aura Invariant

Mt. Aura remains the same local Round path in:

- the world;
- Stonebend city;
- Sandmanor city;
- Glaüshouse city;
- Flynt city;
- every lawful district;
- every lawful site;
- every lawful room;
- every lawful object;
- every lawful recursive detail.

Its local endpoints and geometry MUST remain identical. Its root endpoints
change only through the frame's cached transform.

An inherited Mt. Aura instance is identified by both:

- path identity;
- frame identity.

Dropping the frame identity collapses distinct instances and is invalid at a
scope-preserving boundary.

## 10. Scale Grammar

The named scale progression is:

```text
World
└── City
    └── District
        └── Site
            └── Room
                └── Object
                    └── Detail
                        └── Detail
                            └── ...
```

Legal transitions are exactly:

- World to City;
- City to District;
- District to Site;
- Site to Room;
- Room to Object;
- Object to Detail;
- Detail to Detail.

All other transitions are illegal.

The depth invariants are:

- World at depth `0`;
- City at depth `1`;
- District at depth `2`;
- Site at depth `3`;
- Room at depth `4`;
- Object at depth `5`;
- Detail at depth `6` or greater.

A viewpoint is not another frame level. It is an observation inside a
published frame.

## 11. Declaration and Publication Lifecycle

Spatial declarations do not undergo Bond chemistry. They use the narrower
publication lifecycle appropriate to immutable geometry.

The ordered gates are:

1. Construction.
2. Structural validation.
3. Domain conformance.
4. Atomic publication.
5. Viewpoint resolution.
6. Archive and replay.

### 11.1 Construction

Construction assembles all required declaration fields. An in-memory
construction draft is not canonical history and is not addressable.

### 11.2 Structural validation

Structural validation checks:

- stable identities;
- normalized coordinates;
- positive bounds;
- template existence;
- parent existence;
- distinct parent and child identities;
- exactly one root;
- no ancestry cycle;
- existing distinct path endpoints.

### 11.3 Domain conformance

Hollow Grove conformance checks:

- exact rule-set identity;
- exactly one canonical template;
- exact Route Network-derived topology;
- exact world declaration;
- exact four House city declarations;
- typed level transition;
- depth/level agreement;
- placement authority;
- non-empty unique evidence;
- declaration/frame equality.

### 11.4 Atomic publication

Publication inserts the neutral frame and world declaration as one successful
operation. If any validation fails, neither becomes addressable.

After publication:

- identity is immutable;
- parent is immutable;
- template is immutable;
- level is immutable;
- bounds are immutable;
- placement authority and evidence are immutable.

Correction uses a new identity. Publication retry with the same identity after
a successful commit is rejected as identity reuse.

### 11.5 Viewpoint resolution

Resolution reads published state only. It does not mutate a declaration,
template, transform, or archive.

### 11.6 Archive and replay

Archive records declarations, templates, frames, schema, and rule set. Replay
validates them and recomputes all derived transforms.

## 12. Transform Law

When a frame is published, its parent-to-root transform is composed once with
its bounds. Known-frame resolution therefore does not walk every ancestor.

For either axis:

```text
root = root_min + round(root_span × local / 1,000,000,000)
```

Reverse mapping for a contained root point is:

```text
local = round((root - root_min) × 1,000,000,000 / root_span)
```

Integer arithmetic uses a widened intermediate. The result is clamped only at
the normalized unit boundary during reverse projection.

The laws are:

- identity transform maps root local coordinates to themselves;
- parent composition is deterministic;
- the same declaration history produces the same transform;
- a known frame resolves in constant ancestry time;
- reverse resolution is available only when root lies inside the frame;
- fixed-point rounding is the canonical result;
- presentation rounding cannot become archive authority.

## 13. Frame Selection

A root point can be contained by its root, its descendants, and—if declarations
overlap—multiple frames at equal depth.

Selection policy V1 is:

```text
deepest-then-stable-frame-id-v1
```

It orders candidates by:

1. greatest depth;
2. lexicographically smallest stable frame identity.

The resolver MUST expose the complete ordered candidate list as well as the
selected viewpoint. Live projection MUST include the candidate count.

This rule makes overlapping declarations deterministic without pretending that
the stable identifier grants constitutional priority. A domain that requires
semantic priority must define a future versioned selection rule instead of
silently changing V1.

## 14. Provenance and Authority

Every Hollow Grove frame declaration MUST identify:

- placement authority;
- at least one evidence reference.

The canonical authority is:

```text
authority.hollow-grove.shared-map
```

Canonical evidence includes:

- Constitutional Geography V2;
- Route Network V1;
- Hueman Screen Map v0.1.0;
- this Spatial Inheritance V1 contract.

Evidence references prove why the placement was declared. They do not move the
referenced authority into the kernel.

The optional source anchor records that a canonical House city placement came
from the corresponding template anchor. A declaration carrying a source anchor
MUST have bounds centered on that anchor's local coordinate. The world root
cannot have a source anchor.

## 15. Immutability and Derived State

Authoritative:

- rule-set identity;
- schema version;
- template declarations;
- frame declarations;
- placement provenance.

Derived and replaceable by recomputation:

- cached root transforms;
- frame depths;
- lineages;
- resolved viewpoints;
- candidate lists;
- digests.

Replacing a derived cache is lawful only when recomputation from the same
authoritative declarations yields the same observable result.

## 16. Persistence

The archive contains:

- archive schema version;
- kernel catalog snapshot;
- world declaration list.

The kernel snapshot contains:

- kernel schema version;
- rule-set identity;
- sorted templates;
- sorted frames.

It deliberately omits cached transforms.

Archive encoding is deterministic. Replay:

1. validates the archive schema;
2. canonicalizes declaration ordering;
3. validates stable identities;
4. validates every template;
5. locates exactly one root;
6. rebuilds frames in parent-ready order;
7. detects missing parents and cycles;
8. recomputes cached transforms;
9. validates Hollow Grove conformance;
10. recomputes digests.

A restart MUST NOT:

- create a new identity;
- change a parent;
- change a level;
- reorder topology semantically;
- accept a different canonical placement;
- retain an archived derived cache;
- select a different equal-depth frame;
- produce a different digest from identical input.

## 17. Integrity Digests

Three checksums are carried through the live boundary:

### 17.1 Template digest

Commits to:

- template identity;
- sorted anchors and coordinates;
- sorted paths, endpoints, geometry, band widths, and controls;
- sorted regions and ordered polygon vertices.

Prefix:

```text
fnv1a64-spatial-template-v1:
```

### 17.2 Catalog digest

Commits to:

- kernel schema;
- rule set;
- every template;
- every frame declaration.

Prefix:

```text
fnv1a64-spatial-v1:
```

### 17.3 Hollow Grove replay digest

Commits to the deterministic world archive, including placement authority and
evidence.

Prefix:

```text
fnv1a64-hollow-grove-spatial-v1:
```

FNV-1a is an integrity checksum, not a cryptographic signature. Authority comes
from validated declarations and governing canon, not from the checksum alone.

## 18. Migration

Migration decodes a supported archive, performs full replay and conformance,
then writes the current canonical representation.

Migration SHALL NOT:

- guess unsupported schema meaning;
- repair missing authority silently;
- invent evidence;
- rename an identity;
- move a frame;
- change route topology;
- accept a cycle;
- preserve an invalid cached transform;
- translate a changed topology under the old template identity.

Unsupported schemas fail closed.

## 19. Live Projection Boundary

The Niri bridge:

- constructs the canonical map once;
- validates conformance;
- calculates all three digests once;
- converts the focused-window center into a normalized root point;
- selects the canonical viewpoint;
- publishes root and local coordinates with complete scope.

The scoped live-state schema is `0.3.0`.

Godot:

- retains immutable map and coordinate contracts in memory;
- polls only live state;
- resolves the inherited local plan;
- preserves frame scope on emitted intent;
- owns presentation but not mutation authority.

The checked `artifacts/hueman_screen_map.json` projection MUST expose the
canonical constitutional route identity separately from its display ID.
Current Sea and Current Seanad are separate entries. Glausbahn is in
`curved_routes`. Aura Ridge East is marked `local_only` and
`projection_of = geography.route.aura-ridge`. Cross-layer conformance compares
that artifact's endpoints, geometry class, band width, controls, and surface
polygons directly with the kernel template.

The spatially scoped intent schema is `0.2.0`. Schema `0.1.0` remains valid
only for an unscoped legacy intent.

The Hollow Grove runtime:

- parses normalized scope;
- reconstructs the canonical spatial map independently;
- verifies rule set and selection policy;
- verifies catalog, template, and replay digests;
- recomputes selection from the claimed root point;
- verifies frame, template, scale, depth, candidate count, and local point;
- rejects tampering with `PermissionDenied`;
- preserves valid scope in the consumed receipt.

The consumed receipt schema is `0.3.0`.

No presentation client can authorize a changed map merely by supplying
self-consistent JSON.

## 20. Relationship to Bond

Spatial inheritance is defined to the same constitutional standard as Bond,
but it is not a second Bond grammar.

The corresponding disciplines are:

| Bond discipline | Spatial-inheritance discipline |
|---|---|
| immutable Bond identity | immutable template and frame identity |
| complete formation declaration | complete spatial declaration |
| governing authority and evidence | placement authority and evidence |
| ordered lifecycle gates | ordered publication gates |
| append-only committed history | immutable published declaration |
| derived accounting projection | derived transform and viewpoint |
| replayable persistence | replayable catalog/archive |
| rule-set stability | spatial rule-set stability |
| independent validation | runtime scope recomputation |
| contradiction rejection | conformance and tamper rejection |

A Bond may reference a spatial frame as location evidence. Spatial inheritance
does not own the Bond, its participants, Current, Aura, obligations, term,
challenge, Tombstone, Toke, or resolution.

## 21. Illegal States

Illegal states include:

- no template;
- more than one Hollow Grove template;
- no root;
- more than one root;
- root with a parent;
- root not filling the unit;
- root with a source anchor;
- child without a parent;
- child referencing itself;
- missing parent;
- ancestry cycle;
- duplicate frame identity;
- duplicate template identity;
- duplicate anchor identity;
- duplicate path identity;
- duplicate region identity;
- path with one endpoint twice;
- path with a missing endpoint;
- Straight path with a control point;
- Round path without exactly one control point;
- path with zero band width;
- region with fewer than three distinct vertices;
- region with zero area;
- zero-width or zero-height bounds;
- out-of-range coordinate;
- missing placement authority;
- missing placement evidence;
- duplicate placement evidence in a canonical archive;
- bounds not centered on a declared source anchor;
- template identity drift;
- canonical topology drift;
- canonical House placement drift;
- declaration differing from its published kernel frame;
- illegal level transition;
- level/depth disagreement;
- changed rule set without migration;
- changed archive schema without migration;
- differing replay digest for identical archive input;
- live local point inconsistent with the live root point;
- live selected frame inconsistent with canonical selection;
- live digest inconsistent with canonical replay;
- frame identity omitted from an inherited route instance.

## 22. Failure Modes and Recovery

### Invalid construction

Return a typed error. Publish nothing.

### Missing parent

Preserve the declaration outside canonical state until its parent is lawfully
available, or reject it. Never attach it to the root by inference.

### Cycle

Reject the archive or declaration set. Do not choose an arbitrary break point.

### Unsupported schema or rule set

Fail closed. Require an explicit migration implementation.

### Corrupt or tampered archive

Reject replay. Preserve the original bytes for diagnosis when the surrounding
storage policy permits; do not rewrite them as canonical.

### Process interruption

Re-read the last completely written archive and recompute transforms. Cached
state has no recovery authority.

### Live digest mismatch

Reject the intent. Refresh the bridge/runtime contract from one canonical
archive before retrying.

### Equal-depth overlap

Apply the recorded V1 selection policy and expose all candidates. Do not use
hash-map iteration order.

### Correction

Create a new identity and explicit successor/migration record. Do not mutate
the published declaration in place.

## 23. Concurrency

Publication is logically atomic per catalog instance.

Concurrent publishers MUST serialize against a common authoritative archive or
use an equivalent compare-and-commit boundary. Two successful publications may
not claim the same frame identity.

Readers may share immutable published catalogs and cached transforms. A reader
must observe either the state before publication or the state after complete
publication, never a frame without its declaration.

The current implementation constructs the canonical catalog before sharing it
through `OnceLock` in the live bridge.

## 24. Visibility and Privacy

Spatial scope is semantic geometry. It does not authorize capture of window
pixels or disclosure of protected application content.

Managed application privacy remains governed by the application registry.
A semantic world anchor may be projected when capture is forbidden. Spatial
inheritance cannot weaken that restriction.

Restricted visibility may hide presentation details, but it must not alter the
canonical frame, topology, or replay result.

## 25. Performance Boundary

The implementation removes:

- per-frame topology copies;
- repeated ancestor transform walks for known frames;
- repeated parsing of immutable Godot map contracts;
- repeated live digest calculation.

Known-frame projection requires one frame lookup and one cached fixed-point
transform regardless of depth.

Arbitrary root-point selection currently scans the frame catalog. Fine-grained
Godot route and surface hit-testing currently scans the small shared template.
A larger catalog should add an index at those two boundaries while preserving
the same candidate order and result.

This contract does not claim to tune:

- PipeWire;
- compositor scheduling;
- GPU rendering;
- collision;
- navigation meshes;
- general pathfinding.

## 26. Verification Requirements

Kernel tests MUST cover:

- fixed-point bounds;
- template validation;
- missing parents;
- invalid endpoints;
- cached multi-level transforms;
- reverse viewpoint recovery;
- deterministic equal-depth selection;
- snapshot order independence;
- stable digest;
- unsupported schema;
- missing root;
- cycle rejection;
- kernel purity.

World tests MUST cover:

- one canonical template;
- exact four House city declarations;
- exact placement provenance;
- six anchors, including distinct co-located Ranina and Aura Ridge Junction;
- all ten constitutional routes;
- the local Aura Ridge East projection;
- all path band widths and Round controls;
- all three surface polygons;
- exact static screen-artifact parity;
- Current Sea/Current Seanad separation;
- Round Glausbahn classification;
- Mt. Aura at every scale;
- legal complete scale progression;
- repeated Detail inheritance;
- missing evidence;
- skipped scale;
- identity reuse;
- canonical drift;
- archive round trip;
- restart digest equality;
- archive tampering;
- declaration/frame equality.

Live tests MUST cover:

- root viewpoint;
- House city viewpoint;
- candidate count;
- rule-set projection;
- digest projection;
- offscreen rejection;
- normalized-coordinate validation;
- independent runtime recomputation;
- digest tampering rejection;
- receipt preservation.

## 27. Repository Mapping

- Neutral coordinate, template, frame, transform, snapshot, digest, and
  selection rules:
  `hollow-grove-semantic-kernel/src/spatial_inheritance.rs`
- Stable neutral identities:
  `hollow-grove-semantic-kernel/src/lib.rs`
- Hollow Grove levels, declarations, provenance, conformance, archives, and
  canonical map:
  `src/world/spatial_inheritance.rs`
- Public compatibility exports:
  `src/lib.rs`
- Live compositor projection:
  `src/bin/hollow_grove_niri_bridge.rs`
- Godot local-frame consumption:
  `hueman_godot/scripts/hueman_screen_shell.gd`
- Independent live-scope verification:
  `src/bin/hollow_grove_runtime.rs`
- Cross-layer proof:
  `tests/spatial_inheritance.rs`

## 28. Canonical Summary

One template defines the plan.

Each frame defines one immutable instance.

Each child records a parent, scale, bounds, authority, and evidence.

Every transform is derived and replayable.

Every viewpoint retains root and local identity.

Every overlap resolves under a versioned deterministic rule.

Every live scope is independently verified.

Every correction receives a new identity.

Topology inherits.

Meaning remains with its constitutional authority.
