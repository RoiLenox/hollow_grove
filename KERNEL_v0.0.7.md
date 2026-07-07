# Kernel v0.0.7

Date: 2026-07-06

Root recursion:

`Point -> Triway -> HollowGrove -> Bond + Atmosphere -> CurrentSeam -> AuraBeam -> Point`

## Lock

- `KernelPass` is the witnessed shape of one completed recursion.
- `KernelPass` contains only existing kernel stages.
- `KernelPass` is one pass only, not history.
- `KernelPass` is a kernel record, not logging and not a UI event system.
- `Point` remains the singular overlaid landing type.

## What changed

- `KernelPass` was added as the minimal record of one completed kernel pass.
- `run_kernel_cycle()` now returns `KernelPass` instead of only the landed `Point`.
- The TUI boundary can now read the completed pass directly from the kernel.

## KernelPass shape

- start `Point`
- `Triway`
- `HollowGrove`
- `CurrentSeam`
- `AuraBeam`
- landed `Point`

## Intentionally not built yet

- No history
- No logging
- No UI event system
- No split `CurrentPoint`
- No split `CurrentBeam`

## Exact proof output

```text
Current Synthesis creates Point #1
Point becomes Triway.
Triway carries one Point through three ways.
Triway becomes Hollow Grove
Hollow Grove forms Bond on one Way and leaves two ways as Atmosphere.
Hollow Grove becomes CurrentSeam
CurrentSeam projects AuraBeam
AuraBeam lands Point #2
KernelPass witnesses one completed recursion.
Kernel recursion verified.
```
