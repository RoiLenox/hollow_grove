# The Rampart of Roi and Roselina — Executable Implementation V1

Date: 2026-07-28

Status: executable compatibility milestone complete

The active `RuntimeRampart` implementation belongs to the one constitutional
whole, **The Rampart of Roi and Roselina**. Its authorities are exactly two
continuous beings: Roi/Roï and Roselina/Rosaliña. The older HGRF identity
strings remain frozen only for archive replay. See
`docs/GUIDE_AND_SEER_IDENTITY_V1.md`,
`docs/CENTRAL_JUNCTION_TRANSITION_V1.md`, and
`docs/SURFACE_HUEMAN_GUIDE_PROTOCOL_V1.md`.

Stable identity: `runtime-rampart.hollow-grove.v1`

Archive identity: `archive.runtime-rampart.hollow-grove.v1`

Frozen legacy identity: `runtime-federation.hollow-grove.v1`

Frozen legacy archive identity: `archive.runtime-federation.hollow-grove.v1`

## Canonical name and frozen wire identity

Every current HGRF V1 manifest writes:

# **The Rampart of Roi and Roselina**

Older manifests labeled `The Runtime Federation` remain readable. That former
label and the `runtime-federation.*` strings are compatibility data, not a
second constitutional institution.

## Executable milestone

The implementation now provides:

- one deterministic `HGRF` archive version 1;
- a manifest with stable Rampart, archive, ruleset, canonical-year,
  component, checkpoint, event, evidence, authority, provenance, migration,
  phase, and result identities;
- ten required component kinds covering the completed kernel pass,
  constitutional runtime, regional Synthesis, institutional authority, world
  Point, seasonal and Function Junction state, Service Tournament, routes and
  passages, Permanence, and authoritative gameplay;
- component digests and an aggregate digest;
- exact component replay through each existing production decoder and reducer;
- deterministic canonical serialization independent of insertion order;
- missing, duplicate, self-referential, cyclical, contradictory, and
  tampered-input rejection;
- read-only event, subject, and evidence inspection indexes;
- rejected history that retains evidence without producing its attempted
  result;
- one accepted Festival result that becomes the next Way Back input;
- gameplay archive schema V3 with an explicit Runtime Rampart binding;
- explicit V1 and V2 gameplay migration decisions that do not invent history;
- idempotent current-archive loading and migration;
- one contained, nonlethal Central Junction four-House bridge operation;
- a dedicated audit that returns failure when an invariant is violated.

## Production surfaces

- `src/runtime_rampart.rs` owns HGRF types, validation, deterministic
  encoding, replay, inspection, and migration.
- `src/runtime_rampart_fixture.rs` composes real canonical component
  archives and the contained Central Junction operation.
- `src/gameplay/archive.rs` owns Rampart-aware gameplay schema V3 and
  V1/V2 migration.
- `src/bin/runtime_rampart_audit.rs` is the failing-status operational
  audit.
- `tests/runtime_rampart_v1.rs` proves executable Rampart behavior.
- `tests/runtime_rampart_documentation.rs` proves naming and constitutional
  boundary consistency.

## First playable proof

The contained operation is
`operation.central-junction.four-house-bridge`. It proves:

1. The Way Back loads `state.bridge-control.v1`.
2. The Initiation creates a bounded candidate.
3. The Gathering places it under the Service Tournament relationships.
4. A rejected bridge-control attempt retains evidence and produces no state.
5. A real emergency is distinguished from play.
6. A constitutional restraint decision remains visible.
7. A Permanence petition retains four distinct House attestations.
8. The Festival accepts `state.bridge-control.v2`.
9. That exact accepted state becomes the next Way Back input.
10. Cross-domain causes retain their source component identities.
11. Replay reproduces the same manifest, component archives, state, and bytes.
12. Presentation remains read-only and transfers no sovereignty.

## Verification

Run:

```text
cargo test --test runtime_rampart_documentation
cargo test --test runtime_rampart_v1
cargo run --features dev-tools --bin hg-dev -- runtime_rampart_audit
cargo test
```

The audit prints the formal name, component identities and versions, checksums,
canonical year, phase loop, checkpoints, world Point, Function Junction,
Tournament, routes, Permanence, gameplay linkage, accepted and rejected
history, replay result, insertion-order result, presentation boundary, and
sovereignty boundary.

## Federal authority

Roi/Roï and Roselina/Rosaliña retain equal, continuous authority across the
seam. The HGRF executable machinery preserves and replays lawful state but
does not itself possess sovereignty or create a separate Rampart. The
paired Guide seam is recorded in
[Rampart Sovereignty and Seer Ceremony V1](RAMPART_SOVEREIGNTY_AND_SEER_CEREMONY_V1.md).
The singular, sequential Seer creation and Point-founding rules are recorded in
[Seer Ceremony Protocol V1](SEER_CEREMONY_PROTOCOL_V1.md).

## Authority boundary

The implementation completes continuity, not centralization.

The universal kernel still computes one bounded pass without Rampart lore.
Each component still validates and replays through its own production boundary.
The Houses retain their domains. Gameplay may ask for lawful action.
Presentation may witness accepted results. Only accepted domain records carry
canonical consequence forward.
