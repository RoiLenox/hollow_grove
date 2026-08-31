# Hollow Grove Adaptive Midpoint Hinges V2

Runtime pose authority is specified separately by [Hollow Grove Adaptive Topology State V1](HOLLOW_GROVE_ADAPTIVE_TOPOLOGY_STATE_V1.md). This document remains the geometric hinge contract.

## Bottom-up morph authority

The semantic kernel performs the circle-to-rectangle interpolation with
integer fixed-point geometry. The Hollow Grove layer derives the four hinge
definitions, eight named Midway slots, fixed anchors, locked regions, and the
unhinged Aura Ridge spine from Spatial Topology V2. The checked presentation
contract is `artifacts/hollow_grove_adaptive_morph_v1.json`; Godot consumes
that artifact and never supplies its own Midway coordinates.

The canonical circular curves are preserved at `0%`. They are intentionally
not normalized into bilateral copies: Current Sea and Mount Aura retain their
established, distinct curve controls. At `100%`, all four double-slotted hinge
targets form an exact symmetric rectangle. The audit samples every 5% step and
proves fixed endpoints, isolated slots, the unchanged central spine, the
canonical starting geometry, and the exact rectangular target symmetry.

The Godot developer slider previews the full range without changing traversal
or constitutional route identity. Aura Farm, Aura Field, Content Farm, Aura
Beach, and Aura Basin remain locked to their Spatial Topology V2 polygons.

The accepted 0%, 25%, 50%, 75%, and 100% renders and review criteria are
recorded in `HOLLOW_GROVE_MORPH_VISUAL_APPROVAL_V1.md`.

Spatial Topology V2 supplies four double-slotted Midway hinges around the one
central Aura Ridge spine. See `HOLLOW_GROVE_SPATIAL_TOPOLOGY_V2.md` for the
authoritative route, region, migration, and partition contract.

The V2 hinge rule set is:

```text
ruleset.hollow-grove.adaptive-midpoint-hinges-v2
fnv1a64-adaptive-midpoint-hinges-v2:<digest>
```

The southeast hinge is corrected to:

- Inner / Straight: Glausbaughn Midway;
- Outer / Round: Current Sea Midway.

The V1 path names `geography.route.glausbahn` and
`geography.route.current-seanad` are accepted only while decoding a V1 hinge
archive and are immediately normalized to their V2 identities.

All fixed-point interpolation, exact de Casteljau splitting, rectangular
targets, slot isolation, inherited frame projection, and structural
non-authority rules from V1 remain in force.
