# Composition and Provenance Constitutional Review v0.1.0

Status: design review only. No production code, existing module, fixture,
runtime behavior, or test has been changed.

## Question tested

Can Hollow Grove express recursive scaling with less than the accepted
`CompositionNode` / `CompositionRecord` candidate, while preserving:

```text
one Point becomes a whole
the whole becomes a Point
```

The answer is: the **composition index is necessary**, but the accepted shape
contains four independent ideas that do not protect a distinct invariant.

## Constitutional result

| Principle | Result | Reason |
|---|---|---|
| One Point becomes a whole | Pass | A record can index sources and one completed result without changing the routing kernel. |
| The whole becomes a Point | Pass | A result is an addressable node and may be a source in a later record. |
| One idea exists once | Needs reduction | Rule/evidence IDs, depth, and one-value relation type duplicate meaning carried elsewhere. |
| Lawful composition over duplication | Pass after reduction | Domain legality remains in KernelPass, recipes, lineages, and builders. |
| Kernel is smallest mechanical truth | Pass | The model observes kernel results above the kernel. |
| Other layers ask kernel | Pass | Composition records cannot validate or authorize routing/transformations. |
| Domain meaning stays in domains | Pass | Domain references are opaque; no Forms, Houses, routes, or desktop lore enter the model. |
| Evidence is referenced, not copied | Pass | Native traces remain authoritative. |
| Future scales need no new engine | Pass after reduction | An open scale key permits a future node without enum changes. |

## Attempt to remove the architecture entirely

### Could existing traces alone be enough?

No. `KernelPass`, `PointSquaredAscension`, `SynthesisExecution`, `DecisionTrace`,
Sandmanor lineage data, House fixtures, and artifact builders have different
owners and identifier systems. None can answer the cross-domain query:

```text
Which completed House, artifact, or stabilized Point is the source of this
next-scale composition, and what contains it?
```

Adding that query independently to each domain would create the duplicate
provenance bridges the contract is intended to remove.

### Could a generic graph replace the model?

Not without becoming larger. A generic graph needs node types, edge kinds,
domain payloads, validation rules, and traversal semantics. The required law
needs only causal composition plus containment. A graph engine would invent
capability not yet demanded by any witness.

### Could containment be inferred from composition?

No. A House may compose an artifact outside its ordinary containment, and a
node may be contained by a Grove without being produced by it. Causality and
location/context remain distinct facts.

### Could scale be inferred from containment?

No. Containment does not always impose a total scale order, and an uncontained
Point or runtime artifact still needs an explicit scale meaning. However, the
original optional numeric depth is not required.

## Reductions required

### 1. Remove `CompositionRuleId` and `EvidenceRefId`

They are both opaque external references. Their role is already fixed by the
`rule` and `evidence` fields. Separate newtypes add compile-time convenience
but no new model invariant.

Use one `ExternalRef { domain, key }` for:

- the canonical object projected by a node;
- the rule that selected a composition;
- evidence such as KernelPass, PointSquaredAscension, SynthesisExecution,
  DecisionTrace, fixture validation, or artifact build record.

The field name supplies the semantic role. This removes two identifier
concepts and makes cross-domain references uniform.

### 2. Remove `ScaleRef.depth`

The required invariant is an open-ended, meaningful scale—not a universal
ordering. The concrete witnesses already know their relative order from the
record graph and domain rules. A numeric depth would invite false comparisons:
for example, a House and a runtime context should not be globally ordered by a
neutral layer merely because both have numbers.

Use one validated namespaced `ScaleKey`, for example `scale.point` or
`scale.house`. Future keys require no enum edit.

### 3. Replace `NodeRelation` plus one-value `CompositionRelation`

`CompositionRelation::Contains` has one legal value, so the enum carries no
information. `NodeRelation` is also unnecessarily generic because derivation
is already a `CompositionRecord`.

Use the direct relation:

```rust
pub struct Containment {
    pub container: CompositionNodeId,
    pub member: CompositionNodeId,
}
```

This is not a universal relationship engine. It supports multiple containers
when canon needs them and avoids an ambiguous `parent` field.

### 4. Rename and narrow `DomainObjectRef` to `ExternalRef`

The original wrapper is necessary as a *pair* because a local key is not
globally meaningful. It is not necessary as an object-specific concept.
`ExternalRef` is smaller semantically because it consistently refers outward
to an owning domain without claiming every target is an object.

## Reduced implementation target

Support newtypes are retained only where they protect graph-role mistakes.
`CompositionNodeId` and `CompositionRecordId` must remain distinct; confusing
one for the other breaks reference integrity. `ScaleKey` must remain distinct
because using an arbitrary `String` would erase its validation boundary.

```rust
pub struct CompositionNodeId(String);
pub struct CompositionRecordId(String);
pub struct ScaleKey(String);

/// Opaque pointer to an existing domain-owned object, rule, or trace.
pub struct ExternalRef {
    pub domain: String,
    pub key: String,
}

/// Every node is addressable and composable by definition.
pub struct CompositionNode {
    pub id: CompositionNodeId,
    pub subject: ExternalRef,
    pub scale: ScaleKey,
}

/// Causal provenance: sources --rule--> result, optionally in a context.
pub struct CompositionRecord {
    pub id: CompositionRecordId,
    pub sources: Vec<CompositionNodeId>,
    pub rule: ExternalRef,
    pub result: CompositionNodeId,
    pub context: Option<CompositionNodeId>,
    pub evidence: Vec<ExternalRef>,
}

/// Structural provenance, intentionally distinct from causal provenance.
pub struct Containment {
    pub container: CompositionNodeId,
    pub member: CompositionNodeId,
}

pub struct CompositionCatalog {
    pub nodes: Vec<CompositionNode>,
    pub containments: Vec<Containment>,
    pub records: Vec<CompositionRecord>,
}
```

`context` replaces the former `container` field on `CompositionRecord` to
avoid claiming that the operational context is automatically a permanent
containment relationship. Permanent/structural containment belongs only in
`Containment`.

## Field-by-field burden of proof

| Field/type | Must remain? | Protected invariant | What fails if removed? |
|---|---|---|---|
| `CompositionNodeId` | Yes | Persistent identity independent of state equality | Equal values and copies cannot be distinguished from one evolving entity. |
| `CompositionRecordId` | Yes | A composition itself can be addressed, persisted, and audited | No stable reference for one derivation/history entry. |
| `ScaleKey` | Yes | Open semantic scale remains explicit and validated | Cross-scale query becomes lore-specific or raw ambiguous strings. |
| `ExternalRef.domain` | Yes | Canonical owner namespace is explicit | `key` can collide and cannot be resolved. |
| `ExternalRef.key` | Yes | Exact domain-owned rule/object/trace is identified | Record cannot point to canonical evidence or subject. |
| `CompositionNode.subject` | Yes | Node projects a real existing-domain thing | Nodes become anonymous graph tokens with no canonical owner. |
| `CompositionNode.scale` | Yes | A node’s current operating scale is inspectable | Cannot answer required scale question without reimplementing domain logic. |
| `CompositionRecord.sources` | Yes | Participating units are explicit | The recursive chain cannot be queried. |
| `CompositionRecord.rule` | Yes | Selection legality is named without copying details | A result has causes but no stated lawful selector. |
| `CompositionRecord.result` | Yes | One stabilized whole is addressable next | No next-scale handoff. |
| `CompositionRecord.context` | Yes, optional | Operation can be located without changing containment | “Produced in/under X” is lost or incorrectly inferred. |
| `CompositionRecord.evidence` | Yes | Detailed native proof remains linkable rather than copied | Either no proof link or a duplicate provenance log. |
| `Containment` | Yes | Structural membership differs from causality and may be many-to-many | House/Grove and artifact/runtime placement are conflated with derivation. |
| `CompositionCatalog` | Yes | One validation/query home prevents per-domain parallel indexes | Every domain would recreate graph validation and lookup. |
| `ScaleRef.depth` | No | None beyond optional convenience ordering | Remove it. |
| `CompositionRelation` | No | None; it had only `Contains` | Remove it. |
| separate rule/evidence ID types | No | Field role already protects their interpretation | Replace with `ExternalRef`. |
| `NodeCapability` / `PointLike` trait | No | Nodes are composable by definition | Remove it. |

## Witness re-check under the reduced target

### Point -> Point² -> next Point

`node.point.hueman.origin` is the source of a KernelPass record. Its result
references the landed Point² object/trace. A second record, evidenced by
`PointSquaredAscension`, results in `node.point.hueman.level-2`. That resulting
node becomes a source of a later record. No Point, KernelPass, or progression
field is copied or altered.

### Sandmanor Being transformation

`node.being.hueman.primary` remains the result of its own Gnome -> Minotaur
and Minotaur -> Hecaton records by default. The rule/evidence references point
to existing Sandmanor legality and execution detail. The composition catalog
does not accept or reject cross-lineage transformations; the existing
transformation contract does that first.

### House composition

House identity, local institution catalog, lineages, topology, and context
artifact can be source nodes of a House record. The result is
`node.house.sandmanor`, which can then be a source of a Grove record. The
current four-House/Fourway/Rule-of-Twelve canon remains external evidence and
does not become a generic topology system.

### Runtime artifacts

Source artifact nodes become sources of one builder record. The generated
artifact is its result and may source the next builder record. Existing builder
functions, paths, text formats, artifact I/O, and niri bridge remain separate.

## Guardrails

The implementation target must retain these prohibitions:

- no `CompositionRecord` may authorize routing, transformation, access,
  decisions, or presentation;
- no neutral validation may inspect Form, House, route, institution, or desktop
  lore;
- no existing detailed trace may be copied into an `ExternalRef` or record;
- no closed scale enum or maximum depth may be introduced;
- no existing kernel primitive gets a composition field in the first phase;
- no domain is forced to create a node unless it participates in a witness.

## Final conclusion

**3. A smaller architecture exists; describe it.** The accepted candidate is
necessary in purpose but not minimal in shape. The implementation target is the
reduced composition catalog above: `CompositionNodeId`, `CompositionRecordId`,
`ScaleKey`, `ExternalRef`, `CompositionNode`, `CompositionRecord`, direct
`Containment`, and one `CompositionCatalog`. It preserves all four
witnesses and every constitutional principle while removing numeric depth,
one-value relation machinery, duplicate external-ID types, and unnecessary
capability vocabulary.
