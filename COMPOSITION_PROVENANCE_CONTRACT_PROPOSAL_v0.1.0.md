# Composition and Provenance Contract Proposal v0.1.0

Status: design only. This document proposes no production-code change, refactor,
deletion, migration, or commit.

## 1. Problem

Hollow Grove already has one real recurrence:

```text
Point -> KernelPass -> landed Point² -> stabilized next Point
```

`PointProgressionState` proves that the landed result can become the next
composable Point. The repository does not yet have a neutral way to record the
same fact when the completed thing is a transformed Being, a House, a Grove, or
a runtime artifact. Those domains currently connect through specific adapters,
closed canon types, and artifact files.

The missing idea is not a new routing, transformation, world, or artifact
engine. It is a small index that says:

```text
these addressable units participated
under this named rule
inside this optional container
and produced this addressable unit at this scale.
```

That index allows a completed result to become a source in a later record. This
is the neutral meaning of “the whole becomes a Point.” It does not mean the
result must literally become `crate::Point`.

## 2. Proposed minimum model

The proposal has four public concepts only:

1. `CompositionNodeId`: stable identity for one persistent, addressable node.
2. `DomainObjectRef`: opaque reference to the canonical object owned by a
   domain.
3. `CompositionNode`: an addressable projection of that object.
4. `CompositionRecord`: one legal result derived from source nodes.

Containment is a typed edge owned by the same catalog. Trace detail remains in
existing domain traces and is referenced, never copied.

```rust
/// Neutral, stable, scoped identifier. Example:
/// "node.point.hueman.origin" or "node.artifact.current-synthesis.base".
pub struct CompositionNodeId(String);

/// Neutral stable identifier for a record, rule, or trace reference.
pub struct CompositionRecordId(String);
pub struct CompositionRuleId(String);
pub struct EvidenceRefId(String);

/// Opaque projection to the domain that owns the object and its local identity.
/// The composition layer never dereferences this by itself.
pub struct DomainObjectRef {
    pub domain: String,
    pub key: String,
}

/// Open-ended semantic scale. `kind` is a stable namespaced key, while `depth`
/// is optional ordering metadata rather than a hard limit.
pub struct ScaleRef {
    pub kind: String,          // e.g. "scale.point", "scale.house"
    pub depth: Option<u32>,    // e.g. 0, 5; None if no total ordering exists
}

pub struct CompositionNode {
    pub id: CompositionNodeId,
    pub object: DomainObjectRef,
    pub scale: ScaleRef,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CompositionRelation {
    Contains,
}

pub struct NodeRelation {
    pub source: CompositionNodeId,
    pub kind: CompositionRelation,
    pub target: CompositionNodeId,
}

pub struct CompositionRecord {
    pub id: CompositionRecordId,
    pub sources: Vec<CompositionNodeId>,
    pub rule: CompositionRuleId,
    pub result: CompositionNodeId,
    pub container: Option<CompositionNodeId>,
    pub evidence: Vec<EvidenceRefId>,
}
```

`CompositionCatalog` is the storage and query boundary:

```rust
pub struct CompositionCatalog {
    pub nodes: Vec<CompositionNode>,
    pub containment: Vec<NodeRelation>,
    pub records: Vec<CompositionRecord>,
}
```

It validates unique IDs, references, at least one source per record, and no
self-containment. Every `CompositionNode` is composable by definition. It does **not**
validate Frame legality, routing correctness, membership authority, artifact
contents, or decision choice. Their existing systems remain authoritative.

## 3. Why each field exists

| Item | Invariant protected | Why existing types cannot do it | Loss if removed |
|---|---|---|---|
| `CompositionNodeId` | Persistent identity independent of value state | `Point` equality is state equality; many domains have incompatible ID types | Cannot distinguish equal values, copies, and one evolving entity |
| `DomainObjectRef` | Canonical owner and local identity remain explicit | A giant enum would import every domain into the neutral layer | No safe link back to existing objects |
| `ScaleRef.kind` | Meaningful scale classification | `ObjectScale` is a closed ontology-specific enum | Future scales become kernel edits or raw unlabelled integers |
| `ScaleRef.depth` | Optional relative ordering | Kind alone cannot express known ordering where useful | Cannot ask whether a result moved outward/upward without domain logic |
| `NodeRelation::Contains` | Containment is distinct from derivation | `Group.parent` is institutional-only; `CompositionRecord` is temporal/causal | A House cannot be located inside a Grove independently of how it was made |
| `sources` | Participating nodes are explicit | Existing traces are domain-local | Cross-domain composition cannot be queried |
| `rule` | The legal selector is named once | Trace content is not a stable common query key | Provenance says what happened but not under which rule |
| `result` | Each record has one stabilized result | Existing systems use different output types | No reusable next-scale handoff |
| `container` | Operation context is explicit without inferring containment | A source can be used outside its normal container | Cannot distinguish “made by a House” from “made inside a House” |
| `evidence` | Detailed execution stays canonical in its native trace | Copying KernelPass/Recipe/artifact data would fork truth | Neutral index either loses proof or duplicates it |

`CompositionRelation` intentionally has only `Contains`. Derivation is already
unambiguously represented by `CompositionRecord.sources -> result`; adding a
second `DerivedFrom` edge would duplicate the same fact. “Transformed from,”
“routed through,” and “represented by” are domain meanings expressed by the
`rule`, `evidence`, and `DomainObjectRef`, rather than universal relationship
kinds.

## 4. Identity recommendation

Use **validated, scoped, human-readable stable IDs** with a neutral `node.`
namespace. The catalog owns node identity; existing values do not gain UUIDs.

Examples:

```text
node.point.hueman.origin
node.point.hueman.level-2
node.being.hueman.primary
node.house.sandmanor
node.artifact.current-synthesis.base
```

This is the smallest fit for the current repository:

- It works with existing lower-case stable-ID conventions.
- It can be authored in canonical fixtures and persisted plainly.
- It distinguishes two equal-state values because their node IDs differ.
- It permits one persistent node to appear in many records.
- A state update does not change node identity unless the owning domain declares
  a distinct result node.

Do not use content-derived identity: a changed artifact or Frame state would
silently become a different identity. Do not use UUIDs: no distributed writer,
untrusted merge domain, or opaque identity requirement currently justifies them.
Do not use only container-relative identity: nodes must be referenced across
containers and records.

`DomainObjectRef` carries the existing domain identity separately. For example,
the same `node.being.hueman.primary` can refer to `domain = "frame-state"`,
`key = "being.hueman"`; a later stateful snapshot or trace is evidence, not a
new Being identity.

## 5. Scale is open-ended

`ScaleRef` is not an enum. Stable namespaced scale kinds are data:

```text
scale.point
scale.relationship
scale.topology
scale.being
scale.institution
scale.house
scale.grove
scale.world
scale.runtime-context
scale.operating-environment
```

`depth` is optional. It may be used for the canonical outward sequence, but it
does not create a maximum and is not a replacement for containment. A future
`scale.federated-environment` needs no neutral enum edit. Domains can introduce
their own kinds, such as `scale.sandmanor-lineage-stage`, without the
composition layer understanding their lore.

This is deliberately not a general numeric physics scale. `ObjectScale` remains
unchanged because it describes object ontology, not composition depth.

## 6. “Point-like” capability

The contract defines a point-like next-scale unit minimally as an **addressable
`CompositionNode`**. There is no `PointLike` trait, capability bitset, or
second object hierarchy.

Thus a stabilized Point², a transformed Being, a House, or an artifact can
participate in the same grammar by becoming a `sources` entry in another
record.

Routable, transformable, observable, or semantic-context behavior remains a
domain adapter concern. Making them universal capabilities now would introduce
concepts with no cross-domain invariant.

## 7. Four required witnesses

### 7.1 Point progression

Nodes:

```text
node.point.hueman.origin      -> Point / FrameState origin
node.point.hueman.landed-2    -> landed Point² FrameState
node.point.hueman.level-2     -> stabilized Point
```

Records:

```text
record.kernel-pass.origin
  sources: [node.point.hueman.origin]
  rule: rule.kernel-pass.v1
  result: node.point.hueman.landed-2
  evidence: [evidence.kernel-pass.origin]

record.point-squared.ascension-1
  sources: [node.point.hueman.landed-2]
  rule: rule.point-squared-ascension.v1
  result: node.point.hueman.level-2
  evidence: [evidence.point-squared-ascension.1]
```

The first record references the existing `KernelPass`; the second references
the existing `PointSquaredAscension` and application. It does not re-store
routes, capacities, Frame state, or the witness string. `node.point.hueman.level-2`
is composable and may be a source in the next cycle.

### 7.2 Sandmanor transformation

Persistent Being node:

```text
node.being.hueman.primary
  object: ("frame-state", "being.hueman")
  scale: scale.being
```

Frame snapshots are represented as evidence/object references, not new Being
nodes by default. Records can therefore preserve both stable Being identity and
state transitions:

```text
record.sandmanor.gnome-to-minotaur
  sources: [node.being.hueman.primary]
  rule: rule.sandmanor.gnome-to-minotaur
  result: node.being.hueman.primary
  container: node.house.sandmanor
  evidence: [evidence.sandmanor.transition.gnome-minotaur]
```

The evidence points to the existing legal transition/Recipe and its source and
result `FrameId`s. The composition catalog permits a record whose result is the
same persistent node as its source: it records a stateful transformation, not a
new entity. If later canon declares a split, merger, clone, or replacement, the
domain creates a new node and the same contract already supports it.

`SandmanorForm`, `SandmanorLineage`, and
`validate_sandmanor_transition` remain the sole authority on Gnome → Minotaur
→ Hecaton and Elf → Centaur → Pegasus legality.

### 7.3 House composition

```text
node.house.sandmanor
  object: ("hollow-grove-contract", "house.sandmanor")
  scale: scale.house
```

One House record may have only the currently meaningful components:

```text
record.house.sandmanor.canonical
  sources: [
    node.house.sandmanor.identity,
    node.institutions.sandmanor,
    node.lineage.sandmanor,
    node.topology.sandmanor,
    node.artifact.sandmanor.context
  ]
  rule: rule.house-canonical-composition.v1
  result: node.house.sandmanor
  evidence: [evidence.house.sandmanor.fixture]
```

Absent components are simply absent. The record does not force a universal
House schema. A Grove record can later use the four House nodes as sources;
the current Fourway/Rule-of-Twelve canon remains the evidence and rule owner,
not a generic topology engine.

### 7.4 Runtime artifact

```text
node.artifact.current-synthesis.base
  object: ("artifact", "artifacts/current_synthesis_base.md")
  scale: scale.runtime-context
```

```text
record.artifact.current-synthesis.base
  sources: [node.artifact.snapshot, node.artifact.prompt,
            node.artifact.desktop-status]
  rule: rule.builder.current-synthesis-base.v1
  result: node.artifact.current-synthesis.base
  container: node.runtime.hollow-grove
  evidence: [evidence.builder.current-synthesis-base]
```

The result is composable and can be a source for an existing later builder.
The actual builder function, file format, and filesystem operation remain
unchanged during migration.

## 8. Existing-type mapping

| Existing type/system | Contract relationship | Rationale |
|---|---|---|
| `Point` | Adapted into a node; otherwise unchanged | Node identity sits above its state equality |
| `KernelPass` | Referenced by composition evidence | It already owns route/landing detail |
| `PointSquaredAscension` | Referenced by composition evidence | It already owns capacity and legal application detail |
| `FrameId` | Already provides required state data | It remains a Frame classification, not a composition node ID |
| `BeingId` | Potentially generalized later | Current one-value enum cannot identify persistent multiple beings |
| `SynthesisExecution` | Referenced by composition evidence | It remains the authoritative execution trace |
| `DecisionTrace` | Referenced by composition evidence | It remains decision provenance; composition does not choose tactics |
| `House` | Adapted into a node; fixed canon unchanged | A node projects the current House enum/fixture |
| Fourway / Rule of Twelve | Referenced by composition evidence | Fixed topology remains fixed canon, not reusable engine |
| `Way` | Entirely separate | Kernel routing primitive, not a cross-scale identity |
| `ObjectScale` | Entirely separate | Ontology classification, not recursive scale |
| institutional `Group.parent` | Already provides institutional containment | Do not replace it; optionally project selected groups into nodes later |
| runtime artifacts | Adapted into nodes | Files stay canonical storage; nodes index dependencies |
| bridge state | Referenced or adapted later | Keep niri bridge mechanics outside kernel and composition core |
| Hueman traversal state | Referenced by composition evidence | It keeps its current state machine until a real cross-scale need exists |

## 9. Serialization

Use one optional, versioned, line-oriented neutral document, initially only for
the catalog. The project has no dependencies, so this avoids introducing a
serialization framework solely for this contract.

Conceptual format:

```text
schema: composition-catalog/v1
node: node.point.hueman.origin|frame-state|being.hueman|scale.point|0
node: node.point.hueman.level-2|point|point.hueman.level-2|scale.point|0
contains: node.grove.hollow|node.house.sandmanor
record: record.point-squared.ascension-1
sources: node.point.hueman.landed-2
rule: rule.point-squared-ascension.v1
result: node.point.hueman.level-2
container:
evidence: evidence.point-squared-ascension.1
```

The exact syntax should be finalized only alongside implementation. Required
properties are a schema version, stable IDs, explicit relation kind, sources,
result, and extensible unknown-field handling. Existing text fixtures and
persistence remain untouched. Initial adapters may construct the catalog in
memory; serialization is optional until a genuine persistence consumer exists.

## 10. Tests before implementation

1. Equal state may project to two distinct node IDs.
2. A stateful Being transition may retain the same node ID.
3. A domain-declared replacement may use a distinct result node ID.
4. Point² records reference, rather than duplicate, `KernelPass` evidence.
5. A stabilized Point² node is valid as a later record source.
6. Gnome → Minotaur records retain the Sandmanor lineage evidence.
7. Minotaur → Hecaton retains the same Being node by default.
8. Cross-lineage transitions are rejected by `validate_sandmanor_transition`,
   while a composition record remains mechanically neutral.
9. A House record accepts several local component nodes without requiring a
   universal component list.
10. A House result can be a source of a Grove-level record.
11. An artifact record accepts multiple artifact sources.
12. Its output can be a source of another artifact record.
13. `Contains` and record derivation return different query answers.
14. A future `scale.federated-environment` validates without an enum change.
15. Domain trace details remain absent from neutral record fields and available
   only through evidence references.
16. The existing 324 library tests plus all binary/integration tests remain
   green before and after the first isolated addition.

## 11. Incremental migration

1. Add a new, isolated `composition` module with IDs, catalog validation, and
   in-memory query tests only. No existing module imports it yet.
2. Add four read-only fixture/adapters for the witnesses above. Each adapter
   references current canonical data and traces.
3. Add composition-specific tests and a design-level validation artifact only
   if a consumer needs it.
4. Optionally serialize the catalog after a real persistence consumer appears.
5. Adopt one real runtime artifact builder as a pilot dependency record while
   retaining the existing builder and artifact format.
6. Evaluate an artifact dependency graph only after the pilot proves that the
   catalog removes duplication.
7. Revisit closed enums only where a concrete future scale cannot be projected
   through `DomainObjectRef`; do not generalize them preemptively.

The first implementation changes only the new neutral module, one focused test
module, and perhaps one design/fixture module. It should not modify Point,
routing, Sandmanor legality, world geometry, or artifact builders.

## 12. Excluded concepts

This proposal intentionally excludes:

- a universal `Entity`, `Object`, or `WorldObject` hierarchy;
- a generic topology or graph-routing engine;
- a second transformation engine;
- a second event/provenance log;
- lore terms, House names, routes, Forms, or desktop concepts in the neutral
  module;
- UUIDs, global registries, runtime reflection, or mandatory Serde;
- automatic scale inference, automatic capability grants, and automatic
  containment;
- migration of all current fixtures or artifact formats.

## 13. Risks and unresolved questions

1. **Evidence dereferencing:** the neutral catalog needs a convention for
   resolving `EvidenceRefId` without becoming a universal trace registry.
   Initial implementation should validate only reference syntax, while witness
   adapters prove their evidence exists.
2. **Stateful versus replacement transformations:** the default proposed
   Sandmanor projection keeps Being identity, but other transformation canon may
   require a new persistent node. That must remain an owning-domain decision.
3. **Scale depth:** `depth` should remain advisory. If it becomes a mandatory
   total order, it will falsely impose one hierarchy on unrelated domains.
4. **Artifact identity:** a path is a suitable initial object key, but a future
   artifact version/history model may need a revision key in `DomainObjectRef`.
5. **Catalog ownership:** a first implementation should use a dedicated
   fixture/catalog rather than place mutable composition state into `Point` or
   `WorldSession` prematurely.

## 14. Estimated first-implementation files

Likely additions only:

```text
src/composition.rs
tests/composition_contract.rs
```

Potentially, if exported publicly:

```text
src/lib.rs
```

Potentially, if canonical read-only witnesses are kept with world fixtures:

```text
src/world/composition_witnesses.rs
src/world/mod.rs
```

No existing routing, transformation, House geometry, runtime builder, or
institutional production module should need modification in the first pass.

## 15. Recommended commit boundaries after approval

1. `Add neutral composition catalog and invariant tests`
2. `Add read-only provenance witnesses for Point, Sandmanor, House, and artifacts`
3. `Add optional composition catalog persistence` (only if a consumer exists)
4. `Pilot composition provenance for one runtime artifact builder`

## Verdict

This contract is **genuinely necessary** if Hollow Grove is to make the
recursive scaling law true beyond Point progression. A smaller solution exists
only in the sense that it should not include traits, generic object hierarchies,
universal relationship vocabularies, or a new engine; the four-concept catalog
above is that smaller solution.

It belongs **above the kernel**. It preserves the kernel’s existing recurrence
and observes its results rather than changing routing primitives. It enables a
completed structure to become a Point-like next-scale unit by making every
eligible result a stable, composable node. It reduces independent architectural
ideas by replacing per-domain provenance bridges with one neutral index while
leaving domain-specific mechanics where they already belong.
