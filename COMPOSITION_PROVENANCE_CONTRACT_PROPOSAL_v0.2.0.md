# Composition and Provenance Contract Proposal v0.2.0

Status: accepted proof-of-concept specification. This supersedes v0.1.0 where
the constitutional review reduced the design.

## Purpose

Provide one neutral, read-only index for lawful recursive scaling:

```text
source nodes + operation + optional evidence -> result node
```

Every result node may later appear as a source node. This expresses “one Point
becomes a whole; the whole becomes a Point” without changing the routing kernel
or replacing domain mechanics.

## Neutral core

```rust
pub struct CompositionNodeId(String);
pub struct CompositionRecordId(String);
pub struct ScaleKey(String);

pub struct ExternalRef {
    pub namespace: String,
    pub key: String,
}

pub struct CompositionNode {
    pub id: CompositionNodeId,
    pub object: ExternalRef,
    pub scale: ScaleKey,
}

pub struct CompositionRecord {
    pub id: CompositionRecordId,
    pub sources: Vec<CompositionNodeId>,
    pub result: CompositionNodeId,
    pub operation: ExternalRef,
    pub evidence: Option<ExternalRef>,
}

pub struct Containment {
    pub container: CompositionNodeId,
    pub member: CompositionNodeId,
}
```

`CompositionCatalog` owns nodes, records, direct containments, and only the
indexes necessary to resolve them and list records by source/result.

## Required invariants

| Field/type | Invariant |
|---|---|
| `CompositionNodeId` | Stable identity distinguishes equal values, copies, and persistent entities. |
| `CompositionRecordId` | One causal record can be addressed and audited independently. |
| `ExternalRef` | The neutral layer can point to an authoritative domain object, operation, or trace without interpreting it. |
| `ScaleKey` | Scale remains explicit and open-ended without a closed enum or numeric depth. |
| `CompositionNode.object` | A neutral node projects a canonical domain-owned subject. |
| `CompositionNode.scale` | A node’s current semantic scale is queryable. |
| `CompositionRecord.sources` | Participating addressable units are explicit; order is preserved but given no neutral meaning. |
| `CompositionRecord.result` | A completed whole has one addressable result that may be reused as a source. |
| `CompositionRecord.operation` | The authoritative selector is named without re-executing or validating it. |
| `CompositionRecord.evidence` | One optional native trace/build reference is linked rather than copied. |
| `Containment` | Structural membership remains distinct from causal composition. |

No capability type, numeric scale depth, rule/evidence ID wrapper, generic
relation enum, universal graph engine, domain parser, or domain-specific field
belongs in the core.

## Validation boundary

The catalog validates only neutral structure: valid identifiers/references,
unique node and record IDs, source/result existence, containment endpoint
existence, direct self-containment, and containment cycles.

It does not execute operations or validate routing, Recipes, transformations,
House canon, decisions, institutions, artifacts, or presentation. Existing
systems remain authoritative.

## Four proof-of-concept witnesses

1. Point progression: existing `KernelPass` and `PointSquaredAscension` are
   referenced, not copied; Point² becomes a later source.
2. Sandmanor: existing legal Gnome -> Minotaur -> Hecaton transitions remain
   validated by `validate_sandmanor_transition`; the persistent Being node is
   retained across Frame changes.
3. House: Sandmanor is a House-scale result from a deliberately small local
   set; local membership uses direct containment; it becomes a Grove source.
4. Runtime artifact: existing builders remain unchanged; source artifacts,
   builder operation, output artifact, and runtime context are recorded after
   the normal build; the output becomes a later source.

## Serialization decision

Deferred. The four witnesses validate in memory, and no current consumer needs
composition-catalog persistence. Adding a custom parser now would create the
duplication this contract is meant to reduce. A future persisted catalog must
use a versioned format and preserve all fields above without migrating existing
fixtures.

## Placement and dependency direction

`src/composition.rs` contains only neutral core types and standard-library
dependencies. Witness constructors live above it in the world/domain layer and
may depend on both their existing domain and the neutral core. The composition
core imports no Houses, Forms, routes, artifacts, desktop types, decision
logic, or kernel primitives.
