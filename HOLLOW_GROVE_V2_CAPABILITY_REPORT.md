# Hollow Grove V2 Capability Report

Status: verified implementation report
Date: 2026-07-18
Scope: `hollow-grove` crate

Capability labels used throughout:

- **Implemented** — executable through production types and reducers.
- **Partially implemented** — lawful core exists; named breadth remains absent.
- **Scaffolded** — a type or fail-closed boundary exists without executable law.
- **Deliberately unsupported** — current constitution prohibits or reserves it.
- **Proposed** — possible later work, not current capability.

## 1. Executive Summary

Hollow Grove V2 is an event-sourced constitutional layer above the existing
recursion kernel. It governs explicit Waves, institution-backed House decisions,
signed Current, signed Aura, Bond lifecycle state, durable Tombstones, Toke
indexes, resolution, successors, canonical persistence, migration, and replay.

This expansion adds an executable regional Synthesis aggregate without changing
either kernel. It reuses the existing Sandmanor form lineage and ratifies exactly
two region-bound transformations:

```text
Gnome → Minotaur
Aura Fields
field stewardship, work, maintenance, and defense
```

```text
Elf → Centaur
Aura Beach
beach patrol plus typed guardianship of the Aura Sea
```

Both require stable identities, established regional standing, subject-bound
prerequisite evidence, an accepted Sandmanor proof decision, and an accepted
Glaüshouse resolution decision. Both preserve predecessor lineage through
persistence and replay. Reversed transformations fail closed.

## 2. Architectural Position Above Recursion

**Implemented.** `src/hollow_grove.rs` retains the ordinal recursion selector as
`KernelBond`, with `Bond` preserved as a compatibility alias. The constitutional
Bond is independently located in `src/constitutional/bond.rs`.

The kernel computes recursion. The constitutional layer consumes completed
kernel facts only through stable evidence references and explicit Waves. It does
not change ordinal selection, kernel state, or recursion output.

## 3. Why the Kernel Remains Pure

Kernel purity prevents House authority, narrative presentation, persistence,
and regional geography from changing recursion. `record_kernel_wave` accepts a
completed `KernelPass`, creates a versioned evidence reference, and records one
Wave. It appends no Bond event and moves no Current.

This separation is verified in both the original conformance suite and the new
end-to-end demonstration.

## 4. Ordinal Recursion Becomes Constitutional Meaning

The causal bridge is explicit:

1. kernel input produces a completed pass;
2. `kernel_pass_evidence` identifies that immutable result;
3. `record_kernel_wave` records a caller-identified Wave;
4. later Bond formation may cite the Wave;
5. later Current movement must be separately submitted and authorized.

The bridge carries causal evidence. It never treats calculation as authority.

## 5. Bond Lifecycle

**Implemented.** `BondEvent` models formation, validation, activation, Current
movement, accumulation, Aura observation, evaluation, challenge/default paths,
maturity, excess, condensation eligibility, Tombstone formation/validation,
Flynt recognition, Toke recording, and final resolution.

`BondAggregate::replay` and `ConstitutionalRuntime::append` use the same state
transition implementation. Every accepted event has a per-Bond sequence, global
causal position, caller-supplied stable identity, and rule-set identity.

The `ordinary-lifecycle` scenario performs the complete proof branch and forms
a reserved renewal child.

## 6. House Authority

**Implemented.** The fixed functional boundary is:

| Function | House |
|---|---|
| Name | Stonebend |
| Prove | Sandmanor |
| Recognize | Flynt |
| Clear | Glaüshouse |
| Resolve | Glaüshouse |

`HouseDecision::validate_for` rejects the wrong House, wrong function, missing
capability, missing evidence, future authority snapshot, or non-accepted outcome.
The cross-House conformance test exercises all five wrong-office combinations.

## 7. Institutional Authority

**Implemented with a compatibility caveat.** `AuthoritySnapshot::from_catalog`
requires an active office holder and copies the office, institution, House,
capabilities, actor, and observation position into history.

Scenario fixtures use the repository's actual institutional catalog. Regional
Synthesis adds exact institution requirements: Sandmanor proof must name the
Sandmen; Glaüshouse resolution must name the medical civilization.

The historical snapshot fields remain public for source compatibility. The
canonical construction path is `from_catalog`; removing struct-literal access
would be a breaking API change and was not done in this phase.

## 8. Evidence

**Implemented.** `EvidenceRef` wraps the repository-neutral `ExternalRef`.
Bond evidence remains an opaque stable reference interpreted by its owning
domain. Regional evidence strengthens the contract with `SubjectEvidence`,
which explicitly names the predecessor Being.

Regional registration, standing, prerequisites, Synthesis, and Tombstone paths
reject empty or mismatched evidence. Adapters identify external facts but do not
manufacture authority.

## 9. Current and Aura Polarity

**Implemented.** Both domains use exact `Sign` plus `u128` magnitude and a typed
unit. Current accounts for movement and retained consequence. Aura accounts for
observation and recognition. They remain separate totals.

## 10. Cross-Polarity Semantics

**Implemented.** `ConstitutionalPolarity` represents:

- Positive Current / Positive Aura;
- Positive Current / Negative Aura;
- Negative Current / Positive Aura;
- Negative Current / Negative Aura.

Aura never offsets Current. The scenario catalog constructs, persists, and
replays all four combinations through actual transactions and observations.

## 11. Default

**Implemented; demonstrations partially expanded.** A formation participant may
declare default against a formation obligation. `Cured` and `Confirmed` are
typed outcomes. Pending defaults block maturity. Confirmed obligations remain
visible in Tombstone remaining obligations.

The conformance suite proves the confirmed branch. A dedicated narrative CLI
scenario for every default outcome is still absent.

## 12. Challenge

**Implemented; evidence registry binding remains limited.** Participant standing
is required to file. Sandmanor proof is required to resolve. Pending challenges
block maturity and require re-evaluation after resolution.

Evidence is a stable opaque reference; the Bond runtime does not maintain a
global ownership registry capable of proving that an arbitrary challenged
reference belongs to another Bond. Regional Synthesis does provide subject
binding where the new constitutional rule requires it.

## 13. Maturity

**Implemented.** A finite Bond matures at its declared term end only after
validation, activation, circulation, accumulation, Aura observation, and
evaluation. Pending challenge/default state blocks maturity. Current and Aura
cannot change at or after the finite term boundary.

Maturity is independent of success. It freezes living chemistry for judgment.

## 14. Renewal

**Implemented.** `ResolutionDisposition::Renew` requires exactly one successor
identity. The resolved parent reserves that identity. Child formation cites the
parent and carries nonempty inheritance evidence. A second resolution after the
terminal state is rejected by phase law.

## 15. Tombstones

**Implemented.** A Tombstone records the source Bond, jurisdiction,
participants, net excess, polarity, completed/remaining obligations, and
evidence. It forms only after eligible condensation. Validation checks a
Bond-prefix replay digest and requires an independent validator.

The Tombstone does not erase the Bond history. A resolved or Tombstoned path has
no event capable of returning it to Active.

## 16. Tokes

**Implemented.** A Toke is a permanent typed index to a validated and
Flynt-recognized Tombstone. It is not a Wave, evidence reference, Bond, or
successor. The runtime maintains a global Toke→Tombstone index and rejects a
premature or dangling Toke.

## 17. Inheritance

**Implemented.** A successor formation contains explicit parent Bond identities
and inheritance evidence. Parent resolution must already reserve the child.
The child retains its own identity, Wave, term, and Stonebend naming decision.

## 18. Succession

**Partially implemented by disposition; House-office succession deliberately
unsupported.** Bond resolution supports Renew, Merge, Branch, Split, Transfer,
Complete, and Dissolve dispositions with cardinality/integrity checks. The
demonstrator fully exercises Renew.

Stonebend title succession, Sandmanor contest succession, Glaüshouse Prima Donna
succession, and House appellate court remain `ReservedHouseProcedure` failures.

## 19. Successor Bonds

**Implemented.** `verify_successor_integrity` checks that every reserved child
exists and reciprocally names its parent. Child formation rejects unknown,
unresolved, or unreserved parents and missing inheritance evidence.

## 20. Persistence

**Implemented.** Bond archives use `HGCONST\0`, schema V1. Regional archives use
`HGREGV2\0`, schema V1. Both are dependency-free binary codecs with fixed enum
tags and length-prefixed strings/lists.

Regional persistence stores accepted constitutional inputs and reconstructs
lineage and assignments through the production reducer. It never trusts a
mutable duty snapshot.

## 21. Replay

**Implemented.** Live and replayed history use the same reducers. Bond replay
rebuilds aggregate and global indexes from Waves/events. Regional replay rebuilds
Being, Synthesis, authority-decision, lineage, and assignment indexes.

Altered regional result lineage produces `REGIONAL_REPLAY_DIVERGENCE`.

## 22. Migration

**Implemented for regional V0→V1; scaffolded for unknown future/prior Bond
versions.** `migrate_regional_archive` accepts V0 and V1, decodes through the
reducer, and emits canonical V1. All other versions fail closed. Bond migration
currently performs canonical rewrite of the supported V1 schema only.

## 23. Idempotency

**Implemented.** Repeating the exact Wave, Bond event, registration, or
Synthesis event identity and payload produces no duplicate effect. Reusing an
identity with different content returns a conflict. Reusing a Synthesis identity
under a new event is a non-idempotent conflict.

## 24. Digests

**Implemented.** Bond digests use versioned FNV-1a labels and protect Tombstone
validation. Regional archive digest uses stable FNV-1a over canonical V1 bytes.
Digests are integrity/replay checksums, not cryptographic signatures or sources
of identity.

## 25. Waves

**Implemented.** A Wave has stable identity, causal position, and origin
evidence. It must precede formation or Current movement that cites it. Recording
a Wave changes no Bond aggregate.

## 26. Illegal States

**Implemented fail-closed mechanism; broad conformance coverage.** Illegal
phases, House functions, authority, evidence, units, terms, sequence, causality,
digests, identities, successor graphs, regional forms, regions, functions,
lineage, and terminal sources return typed errors before commit.

Rejected attempts are observable through read-only traces but are not inserted
into canonical history as accepted facts.

## 27. Fail-Closed Procedures

**Implemented.** Unknown archive versions, reserved succession, reserved appeal,
unratified regional mastery forms, cross-lineage Synthesis, region transfer,
location-only transformation, and missing authority/evidence are unavailable.
The runtime never guesses a procedure.

## 28. Deterministic Identity

**Implemented.** IDs are validated stable strings supplied by the caller. They
never depend on insertion order, memory address, current time, or randomness.
Digests do not become identities. Re-running canonical fixtures yields identical
identities, events, and archive bytes.

## 29. Observability and Traces

**Implemented.** `ConstitutionalTrace` and `TransitionTrace` report sequence,
command, prior/proposed state, authority, evidence, polarity, region, lineage,
rule, function, assignments, digest, replay, persistence, or stable failure
code.

Trace types have no reducer or mutation path. Constructing traces leaves runtime
event counts unchanged.

`tui_events_from_trace` now emits proposed, authority, evidence, accepted or
rejected, and state-change observations in reducer-result order. Its regional
projection branches on the typed region: Centaur beach duties cannot render as
Minotaur field stewardship, and Minotaur duties cannot render as coastal
authority.

## 30. Demonstration Scenarios

**Implemented.** `SCENARIO_CATALOG` includes the ordinary lifecycle, all four
polarity quadrants, both regional Syntheses, nine regional failures, and the
kernel-Wave boundary. `run-all` executes every entry and compares it with its
declared accepted/rejected expectation.

The `end-to-end` scenario combines kernel, Bond, both regional lineages, both
reversed failures, persistence, replay, and trace non-authority.

## 31. Public API

**Implemented.** Canonical entry points include:

- `ConstitutionalRuntime` submission/replay/inspection;
- Bond and regional archive functions;
- `RegionalSynthesisRuntime` registration/Synthesis/Tombstone/replay;
- regional Being, lineage, stewardship, occupation, and guardianship lookups;
- scenario fixtures;
- read-only traces;
- TUI-neutral commands/events and wire codec;
- `ConstitutionalApplicationService`, `TuiRequest`, and `TuiResponse` as the
  presentation-facing owner of a selected production runtime/archive;
- kernel-pass-to-Wave adapters.

There are no public setters for lineage or regional assignments. The service
does not return mutable runtimes or archive bytes. Exact request retries are
idempotent; conflicting request-ID reuse and unsafe cancellation fail closed.
`execute_streaming` delivers those same canonical response events to a caller
sink in sequence order after the synchronous operation reaches an atomic
boundary.

The service boundary can be exercised directly:

```text
cargo run --example constitutional_v2_service -- run gnome-minotaur
cargo run --example constitutional_v2_service -- stewardship gnome-minotaur
cargo run --example constitutional_v2_service -- run elf-centaur
cargo run --example constitutional_v2_service -- guardianship elf-centaur
cargo run --example constitutional_v2_service -- run gnome-centaur
cargo run --example constitutional_v2_service -- replay elf-centaur
cargo run --example constitutional_v2_service -- audit
```

## 32. Regional Synthesis Architecture

**Implemented.** The regional aggregate is a sibling of the Bond runtime inside
`src/constitutional/`. It reuses `SandmanorForm` and
`validate_sandmanor_transition` rather than duplicating lineage. It remains
separate from bounded Recipe execution and final Bond Resolution.

The fixed rule selects legality only from explicit command forms; location is a
required standing check, not an outcome selector.

## 33. Gnome-to-Minotaur Lineage

**Implemented.** The source Gnome retains its identity and becomes
`SynthesizedInto(result_id)`. The result Minotaur receives a new caller-supplied
identity, an explicit predecessor, and a two-entry `lineage_history` containing
the Gnome and Minotaur plus the Synthesis identity.

## 34. Minotaur Stewardship of the Aura Fields

**Implemented as constitutional assignments.** A lawful Minotaur atomically
receives `AuraFieldsStewardship` with duties to tend Aura crops, guard field
boundaries, carry loads, maintain routes, guard harvests, protect field workers,
and stabilize field Current.

**Not implemented as simulation.** Crop growth, load physics, navigation,
combat, worker AI, and Current stabilization algorithms remain downstream.

## 35. Elf-to-Centaur Lineage

**Implemented.** The source Elf remains the first lineage entry and becomes
`SynthesizedInto(result_id)`. The Centaur has a new stable identity and explicit
predecessor/Synthesis references.

## 36. Centaur Patrol of the Aura Beach

**Implemented as constitutional assignments.** `AuraBeachOccupation` grants
typed duties to roam, patrol, guard sea access, watch coastal routes, escort,
recognize horizon changes, defend incursions, and maintain the land-sea
boundary.

**Not implemented as simulation.** Movement, pathfinding, encounter detection,
and escort AI are downstream consumers.

## 37. Centaur Guardianship of the Aura Sea

**Implemented.** Every valid Centaur result from the ratified rule has an
`AuraSeaGuardianship` naming `ConstitutionalRegion::AuraSea`, the Centaur holder,
Glaüshouse authority decision, predecessor-bound evidence, and four duties.

Aura Sea is deliberately not a primary regional standing site for Synthesis.
It is the required guardianship relationship attached to the Aura Beach result.

## 38. Invalid Cross-Regional Transformations

**Deliberately unsupported and tested.** Gnome→Centaur and Elf→Minotaur are
rejected by the existing lineage validator. Correct forms in wrong regions are
rejected for insufficient standing. Wrong function, authority, institution,
evidence subject, identity, terminal source, or duplicate Synthesis also fails
without state change.

## 39. Regional Persistence and Replay

**Implemented.** V1 archives encode registration and Synthesis commands,
including jurisdiction, standing, prerequisites, both House decisions, and
subject evidence. Decode submits them to `RegionalSynthesisRuntime`. Equality
therefore proves that form, lineage, field duties, beach duties, and sea
guardianship survive canonical reconstruction.

## 40. Performance Characteristics

**Measured.** Release medians on the recorded machine were:

- 0.592 µs single regional registration transition;
- 90.580 µs full Bond lifecycle;
- 43.852 µs Gnome→Minotaur complete scenario;
- 45.566 µs Elf→Centaur complete scenario;
- 8.346 µs two-event regional replay;
- 107.348 ms 10,000-event regional replay.

See `V2_PERFORMANCE_CHARACTERIZATION.md` for method, all averages/medians/worst
observations, scaling analysis, and allocation notes.

## 41. Features Now Cheap to Build

These are **proposed consumers**, not implemented features:

- a terminal or web constitutional history viewer;
- regional roster views with verified lineage;
- field-work task systems gated by Minotaur stewardship;
- coastal patrol/escort systems gated by Centaur occupation;
- Aura Sea access control gated by Centaur guardianship;
- archive inspection and migration tooling;
- scenario snapshot regression output;
- institutional authority/evidence audit panels;
- event-stream export;
- operational replay recovery commands.

They are cheap because legality, identity, authority, evidence, persistence,
replay, and read-only projections already exist.

## 42. Features Requiring Constitutional Ratification

**Deliberately unsupported until amended:**

- Gnome→Centaur or Elf→Minotaur;
- Minotaur→Hecaton and Centaur→Pegasus regional execution;
- regional transfer between Aura Fields and Aura Beach;
- Aura Sea primary standing;
- Minotaur sea authority or Centaur field authority;
- automatic transformation from occupation;
- House-office succession;
- appellate court and appeal outcomes;
- synthesis proposer/validator/executor role separation beyond current decisions;
- cryptographic authority signatures;
- cross-archive federation/conflict resolution.

## 43. Known Limitations

- Bond archive migration has no accepted pre-V1 schema; unsupported versions
  fail closed.
- Bond evidence is opaque and not globally subject-indexed.
- The regional aggregate and Bond aggregate are siblings; no new law binds every
  regional Synthesis to a specific Bond because that requirement was not
  ratified in the regional lock.
- Replaying 10,000 regional registration events exposes superlinear scanning in
  idempotency lookup.
- Trace fields are stable textual projections rather than a versioned binary
  trace archive.
- The application service is synchronous, single-selected-scenario, and
  demonstration-scoped; it is not a multiuser or live gameplay service.
- A TUI is not built.

## 44. Recommended V2.x Roadmap

1. Build a terminal renderer that consumes only
   `ConstitutionalApplicationService`, beginning with catalog, run, trace,
   replay, lineage, stewardship, and guardianship views.
2. Add an event-ID→index projection to regional runtime after profiling, with
   equality and replay invariants unchanged.
3. Add a subject/evidence registry only if constitutional law ratifies global
   Bond evidence ownership semantics.
4. Add concrete field and coastal mechanics as consumers of typed assignments,
   never as alternate sources of regional authority.
5. Define and ratify any additional regional form, transfer, succession, or
   appeal law before implementing it.
