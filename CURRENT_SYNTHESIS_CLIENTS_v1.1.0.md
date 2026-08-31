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

`PLEB` and `META` remain owned by Current Synthesis; they are not universal
kernel primitives or Hollow Grove House authority. Hollow Grove may define a
named, read-only boundary adapter that receives a scoped witness from those
clients without importing their route grammar.

HAL and Clouseau remain Current Synthesis clients, not House roles. Cleopatra
may follow the inverse BLEP/Underway reflection beneath Clouseau's route, but
does not occupy `PLEB` or `META`.

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

Neither HAL nor Clouseau may mutate Hollow Grove or feed authority back into
it. A scoped witness may cross the artifact boundary only as attributable,
challengeable evidence through the Compromise/Rampart adapter; the
receiving House or Being must still decide.
