# The Runtime Federation — Executable Implementation V1

Date: 2026-07-28

Status: executable milestone complete

Stable identity: `runtime-federation.hollow-grove.v1`

Archive identity: `archive.runtime-federation.hollow-grove.v1`

## Formal name

The formal public name of the shared continuity layer is:

# **The Runtime Federation**

“Federation,” “runtime federation,” and `HGRF` may be used as contextual
shorthand. They do not replace the formal name in public headings, audit
output, or constitutional references.

## Executable milestone

The implementation now provides:

- one deterministic `HGRF` archive version 1;
- a manifest with stable federation, archive, ruleset, canonical-year,
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
- gameplay archive schema V3 with an explicit Runtime Federation binding;
- explicit V1 and V2 gameplay migration decisions that do not invent history;
- idempotent current-archive loading and migration;
- one contained, nonlethal Central Junction four-House bridge operation;
- a dedicated audit that returns failure when an invariant is violated.

## Production surfaces

- `src/runtime_federation.rs` owns HGRF types, validation, deterministic
  encoding, replay, inspection, and migration.
- `src/runtime_federation_fixture.rs` composes real canonical component
  archives and the contained Central Junction operation.
- `src/gameplay/archive.rs` owns federation-aware gameplay schema V3 and
  V1/V2 migration.
- `src/bin/runtime_federation_audit.rs` is the failing-status operational
  audit.
- `tests/runtime_federation_v1.rs` proves executable federation behavior.
- `tests/runtime_federation_documentation.rs` proves naming and constitutional
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
cargo test --test runtime_federation_documentation
cargo test --test runtime_federation_v1
cargo run --bin runtime_federation_audit
cargo test
```

The audit prints the formal name, component identities and versions, checksums,
canonical year, phase loop, checkpoints, world Point, Function Junction,
Tournament, routes, Permanence, gameplay linkage, accepted and rejected
history, replay result, insertion-order result, presentation boundary, and
sovereignty boundary.

## Federal sovereignty

Runtime sovereignty belongs solely to **Roi**. The executable federation
preserves and replays lawful state but does not itself possess sovereignty.
The complete federal sovereignty and Seer Ceremony lock is recorded in
[Federal Sovereignty and Seer Ceremony V1](FEDERAL_SOVEREIGNTY_AND_SEER_CEREMONY_V1.md).

## Authority boundary

The implementation completes continuity, not centralization.

The universal kernel still computes one bounded pass without Federation lore.
Each component still validates and replays through its own production boundary.
The Houses retain their domains. Gameplay may ask for lawful action.
Presentation may witness accepted results. Only accepted domain records carry
canonical consequence forward.
