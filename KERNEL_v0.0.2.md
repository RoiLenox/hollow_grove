# Kernel v0.0.2

Date: 2026-07-06

Root recursion:

`Point -> Triway -> HollowGrove -> AuraSeam -> Point`

## What changed

- `AuraSeam` was added to the kernel cycle.
- `run_kernel_cycle()` now follows `Point -> Triway -> HollowGrove -> AuraSeam -> Point`.
- The kernel proof and tests were updated to reflect the new cycle.

## Why AuraSeam exists

`AuraSeam` exists because `HollowGrove -> Point` was not the full root recursion.

The next `Point` does not appear directly from `HollowGrove`. It passes through `AuraSeam` first. At kernel depth, this is the seam where the same `Point` is read in passage before landing as the next `Point`.

## Why CurrentPoint and CurrentBeam are not separate types

They are not separate kernel primitives yet.

At the base layer, `Point` already contains beam-direction as potential through overlay:

- location-view = Current Point
- direction-view = Current Beam

Because of that, the kernel keeps a single `Point` type. A split into separate `CurrentPoint` and `CurrentBeam` types is deferred unless the software later proves that position and direction must be distinguished in code.

## Exact proof output

```text
Current Synthesis creates Point #1
Point becomes Triway
Triway becomes Hollow Grove
Hollow Grove becomes AuraSeam
AuraSeam creates Point #2
Kernel recursion verified.
```

## Intentionally not built yet

- No `CurrentPoint` type
- No `CurrentBeam` type
- No extra kernel structs beyond the locked recursion
- No stateful direction model
- No overlay system implementation
- No above-kernel workspace, routing, or world logic
