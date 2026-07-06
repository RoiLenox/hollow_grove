# Kernel v0.0.3

Date: 2026-07-06

Root recursion:

`Point -> Triway -> HollowGrove -> AuraSeam -> Point`

## Lock

- Point remains singular.
- Triway is one Point read three ways.
- Way has no meaning yet.
- Way is ordinal only: `One`, `Two`, `Three`.
- Triway is still not routes, directions, or coordinates.

## What changed

- `Way` was added as an ordinal kernel enum.
- `Triway` now holds:
  - `source: Point`
  - `ways: [Way; 3]`
- The kernel proof now states that Triway carries one Point through three ways.

## Intentionally not built yet

- No semantic meaning for `Way`
- No route logic
- No direction model
- No coordinate system
- No split of `Point` into separate primitives

## Exact proof output

```text
Current Synthesis creates Point #1
Point becomes Triway.
Triway carries one Point through three ways.
Triway becomes Hollow Grove
Hollow Grove becomes AuraSeam
AuraSeam creates Point #2
Kernel recursion verified.
```
