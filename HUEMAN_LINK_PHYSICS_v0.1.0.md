# Hueman Link Physics v0.1.0

Date: 2026-07-07

## Rule

Links that do not get bonded may later resolve into `current` or `aura` according to downstream physics.

## Bond Split

- bonded link stays the selected route
- unbonded links remain available as unresolved world material
- unresolved material is not empty; it carries later directional bias

## Current Bias Physics

- continuity pressure favors `current`
- occupancy load favors `current`
- inland persistence favors `current`
- repeat traversal favors `current`

## Aura Bias Physics

- exposure pressure favors `aura`
- threshold bleed favors `aura`
- atmospheric spill favors `aura`
- edge drift favors `aura`

## Boundary

- This is descriptive Hueman reading only.
- It does not procedurally resolve links yet.
- It does not rewrite `Bond`.
- It does not change Current Synthesis ownership of sequence semantics.
- It does not change Hollow Grove recursion.
