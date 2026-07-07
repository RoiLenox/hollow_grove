# Current Synthesis Clients v1.1.0

Date: 2026-07-06

## Architecture

```text
Hollow Grove
↓
Current Synthesis
├── HAL
└── Clouseau
```

## Hollow Grove

- the stable recursive core
- owns the recursive kernel
- owns `KernelPass`
- owns deterministic artifacts
- does not know `PLEB` or `META`

## Current Synthesis

- the operating system built on Hollow Grove
- owns higher-level path semantics
- owns `PLEB` and `META`
- owns workflow interpretation
- owns application behavior

## HAL

- Current Synthesis client
- always occupies the `META` (bent) path
- automation client
- watches, follows, and may automate outside the kernel
- never mutates Hollow Grove

## Clouseau

- Current Synthesis client
- always occupies the `PLEB` (straight) path
- clue client
- reveals traces, residue, and explanations
- never mutates Hollow Grove

## Rule

`PLEB` and `META` are not Hollow Grove concepts.

They belong to Current Synthesis.

HAL and Clouseau are not kernel roles.

They are Current Synthesis clients built on top of Hollow Grove.

## Boundary

```text
Hollow Grove
↓
KernelPass
↓
Consumers
↓
Artifacts
↓
Current Synthesis
├── HAL
└── Clouseau
```

Neither HAL nor Clouseau may redefine the kernel.

Neither HAL nor Clouseau may feed information back into Hollow Grove.

Their responsibilities begin only after the artifact boundary.
