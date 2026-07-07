# Kernel v0.0.9

Date: 2026-07-06

Root recursion:

`Point -> Triway -> HollowGrove -> Bond + Atmosphere -> CurrentSeam -> AuraBeam -> Point`

## Lock

- The TUI remains a read-only witness of `KernelPass`.
- Witness formatting belongs to the TUI boundary, not the kernel.
- The kernel still exposes one completed pass only.
- No controls, history, routing, or added semantics were introduced.

## What changed

- The witness output was reformatted to display the completed pass as canonical stage names only.
- Raw debug internals are no longer shown in the witness view.
- `KernelPass` remains the read-only input to the TUI boundary.

## Witness output

```text
start Point
↓
Triway
↓
HollowGrove
↓
CurrentSeam
↓
AuraBeam
↓
landed Point
```
