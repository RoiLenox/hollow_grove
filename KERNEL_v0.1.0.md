# Kernel v0.1.0

Date: 2026-07-06

Root recursion:

`Point -> Triway -> HollowGrove -> Bond + Atmosphere -> CurrentSeam -> AuraBeam -> Point`

## Lock

- `KernelPass` remains the witnessed shape of one completed recursion.
- Canonical witness formatting now belongs to `KernelPass`.
- Every boundary reads the same kernel-owned witness representation.
- No history, replay, controls, state, routing, or added semantics were introduced.

## What changed

- Deterministic witness rendering moved out of `main.rs`.
- `KernelPass` now owns one canonical witness display.
- `main.rs` became a thin display boundary over the kernel.

## Canonical witness output

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
