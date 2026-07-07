# Kernel v0.0.6

Date: 2026-07-06

Root recursion:

`Point -> Triway -> HollowGrove -> Bond + Atmosphere -> CurrentSeam -> AuraBeam -> Point`

## Lock

- `CurrentSeam` replaces `AuraSeam` as the seam/view frame.
- `AuraBeam` is the projection that emerges from `CurrentSeam`.
- `Point` remains the singular landing type.
- `CurrentPoint` and `CurrentBeam` remain overlaid in `Point`.
- No split is introduced unless the kernel later proves it is necessary.

## What changed

- `AuraSeam` was removed from the kernel path.
- `CurrentSeam` was added as the next step after `HollowGrove`.
- `AuraBeam` was added as the next step after `CurrentSeam`.
- `run_kernel_cycle()` now follows the canonical recursion exactly.

## Intentionally not built yet

- No separate `CurrentPoint`
- No separate `CurrentBeam`
- No extra view semantics inside `CurrentSeam`
- No beam metadata inside `AuraBeam`
- No TUI logic in the kernel

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
Kernel recursion verified.
```
