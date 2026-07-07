# Current Synthesis Sequence v1.5.1

Date: 2026-07-06

## Boundary

- Hollow Grove owns the recursive core.
- Hollow Grove owns `KernelPass`.
- Hollow Grove does not know `PLEB` or `META`.
- Current Synthesis owns `PLEB`.
- Current Synthesis owns `META`.
- Current Synthesis owns path semantics.
- Current Synthesis owns sequencing.
- Current Synthesis owns topology later.

## Canonical Sequence

```text
P/M
↓
L/E
↓
E/T
↓
B/A
```

These are paired joints between the `PLEB` and `META` sides.

Each joint has:

- a `PLEB` side
- a `META` side
- three possible arms of movement on each side
- one arm bonds
- unused arms become clue context, environmental residue, or route material

## Unbonded Resolution

- bonded arms remain the selected route through the joint
- unbonded arms do not disappear after bond selection
- unbonded arms may later resolve into `current` or `aura`
- that later resolution depends on downstream physics rather than kernel bond selection alone

## Client Roles

- HAL belongs to `META`.
- HAL occupies the `META` side of each joint.
- Clouseau belongs to `PLEB`.
- Clouseau occupies the `PLEB` side of each joint.

## Lock

- This sequence belongs entirely to Current Synthesis.
- It does not change Hollow Grove.
- It does not change `KernelPass`.
- It should be documented before topology.
- Topology is downstream from this sequence.

## Deferral

- topology deferred
- `PLEB`/`META` execution deferred
- HAL behavior deferred
- Clouseau behavior deferred
