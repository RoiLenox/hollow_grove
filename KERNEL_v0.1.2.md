# Kernel v0.1.2

Date: 2026-07-06

Root recursion:

`Point -> Triway -> HollowGrove -> Bond + Atmosphere -> CurrentSeam -> AuraBeam -> Point`

## Boundary Lock

- `KernelPass` represents one completed recursion only.
- The TUI witnesses one completed pass only.
- There is no history.
- There is no replay.
- There is no continuing state.
- There is no Driver yet.

## Deferral

Continuing recursion is deferred until the kernel naturally demands recurrence beyond one pass.

## Purpose

This note protects the architecture from premature expansion.
