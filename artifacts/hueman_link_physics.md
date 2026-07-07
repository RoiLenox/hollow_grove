# Hueman Link Physics

## Structural Rule

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

## Crossover Reading

- shared starts can touch the same unresolved material with different bias
- the same region may feel more `current` from one route and more `aura` from another
- crossover zones are where the physics split becomes most visible in Hueman

## Status

- link physics is descriptive-only for now
- no procedural resolver chooses `current` or `aura` yet
- bond selection remains kernel-simple underneath this layer
- no feedback into Current Synthesis
- no feedback into Hollow Grove

## Artifact Inputs

Current Synthesis sequence bytes: 1065.
Hueman Path Crossovers bytes: 1618.

## Boundary Reminder

Link physics explains how unbonded links may later read as `current` or `aura`. It does not rewrite Bond, HollowGrove, or Current Synthesis sequence ownership.
