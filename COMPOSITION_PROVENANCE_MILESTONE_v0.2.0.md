# Composition Provenance Milestone v0.2.0

STATUS:
COMPLETE
REVIEWED
FROZEN

## Purpose

The composition provenance layer records how an addressable whole is lawfully
produced from addressable sources, so that the completed result may become a
source at the next scale.

It solves the cross-domain provenance problem: existing Point progression,
transformation, House, and artifact systems each had their own authoritative
logic and traces, but no shared neutral index for expressing:

```text
one Point becomes a whole
the whole becomes a Point
```

The layer remains above the routing kernel. It observes and links results from
existing mechanics; it does not route, execute, decide, validate domain
legality, or present outcomes.

## Final Accepted Concepts

- Stable, caller-controlled `CompositionNodeId` identity.
- Stable, caller-controlled `CompositionRecordId` identity.
- `ExternalRef` for opaque domain-owned subjects, operations, and evidence.
- Open-ended semantic `ScaleKey`.
- Causal `CompositionRecord`: sources, operation, optional evidence, result.
- Independent direct `Containment` relationship.
- In-memory `CompositionCatalog`.
- Source/result lookup indexes.
- Typed structural errors for malformed references, duplicate identities,
  missing endpoints, empty sources, duplicate containment, self-containment,
  and containment cycles.

## Constitutional Decisions

The following were intentionally excluded:

- Serialization.
- Persistence.
- Runtime adoption.
- Domain authority.
- Numeric scale depth.
- `DomainObjectRef`.
- A universal object abstraction.
- Automatic ID generation.
- Parallel execution logic.
- A second decision engine.

The catalog is not a graph engine, a topology engine, or a replacement for
`KernelPass`, `PointSquaredAscension`, Recipes, lineage validation,
Stanislavski, Falloutman, or artifact builders.

## Witness Summary

Four production-backed witnesses were verified:

1. **Point progression:** `Point -> Point² -> stabilized Point`, using the
   existing kernel cycle and canonical Point² fixture. The landed Point² is a
   source for the stabilization record.
2. **Sandmanor transformation:** `Gnome -> Minotaur -> Hecaton`, using the
   existing Sandmanor legality validator. The Being node remains stable while
   the existing domain owns Frame legality.
3. **House composition:** selected Sandmanor local components compose into a
   House node, which then participates as a Grove-scale source. Containment is
   recorded separately from causal sources.
4. **Runtime artifacts:** existing Hueman artifact builders produce artifact
   nodes that become sources of later artifact-builder records.

Every witness invoked existing production behavior. No parallel routing,
transformation, House, or artifact logic was introduced.

## Review Outcomes

The constitutional POC review verified:

- deterministic caller-controlled identities;
- insertion-order independence of identities;
- causal composition and containment separation;
- indirect containment-cycle detection, including disconnected branches;
- precise duplicate and missing-endpoint diagnostics;
- a neutral core with no domain imports or authority;
- real production-backed witnesses;
- reduced public API surface;
- no unused fields or lookup indexes.

## Verification

- `cargo fmt --check` passed.
- `cargo test` passed.
- 336 library tests passed.
- New composition code introduced no reported Clippy warnings.
- Existing unrelated Clippy warnings remain outside this work.
- An existing unrelated whitespace issue remains outside this work.

## Architectural Status

This milestone is the constitutional foundation for future composition work.
It is **not** adopted by runtime systems, does not persist catalogs, and does
not migrate any existing subsystem.

The proof of concept remains isolated intentionally: broad adoption requires a
separate, explicit architectural decision so that the neutral catalog does not
quietly become a second engine or a source of domain authority.

## Next Approved Phase

> Adopt the composition catalog into exactly one bounded runtime path
> (preferably the artifact builder path) in a separate architectural milestone.

Do not begin that work as part of this milestone.

## Contributor Summary

This checkpoint exists to preserve one small, proven rule: completed
structures can be addressed and lawfully reused as later composition sources
without changing the kernel or duplicating domain mechanics. Keep it isolated
until runtime adoption is intentionally approved; doing so protects the
kernel’s small mechanical truth and prevents composition provenance from
becoming an unbounded universal system.
