# Hollow Grove Transformer Integration Map

Status: Phase 0 discovery record  
Date: 2026-07-29  
Directive: Master Architecture and Implementation Directive V1

This map records the repository that existed before transformer integration and
the exact reuse boundary for the semantic system. It is an integration plan,
not a competing architecture.

## Baseline repository state

- Repository: `/home/warren/hollow_grove`
- Branch: `master`
- Starting revision: `f17a7d0`
- Upstream relation at discovery: four local commits ahead of
  `origin/master`
- Worktree after index refresh: clean
- Root package: `hollow-grove` (`Cargo.toml`)
- Path crates:
  - `hollow-grove-semantic-kernel` (`hollow-grove-semantic-kernel/Cargo.toml`)
  - `flynt-constitution` (`officials-and-outlaws/Cargo.toml`)
- The root manifest is a package manifest, not a declared Cargo workspace.
- No Cargo feature flags existed at discovery.
- No Ollama client, semantic-interpreter trait, or language-model abstraction
  existed at discovery.

Baseline verification:

| Command | Result |
| --- | --- |
| `cargo fmt --all -- --check` | passed |
| `cargo test --all-targets` in the root package | passed; 1,244 tests |
| `cargo test` in `hollow-grove-semantic-kernel` | passed; 21 tests |
| `cargo test` in `officials-and-outlaws` | passed; 16 tests |

No pre-existing test failure was observed.

## Existing architecture

### Authoritative runtime

The root `hollow-grove` crate is the authoritative integration runtime.
Authority is already layered rather than concentrated in one file:

- `src/constitutional/runtime.rs` owns constitutional state transitions.
- `src/constitutional/application.rs` is the constitutional application
  boundary.
- `src/gameplay/runtime.rs` and `src/gameplay/application.rs` own
  authoritative gameplay mutation.
- `src/runtime_federation.rs` owns deterministic cross-domain continuity,
  replay, component validation, event indexes, and the `HGRF` archive.
- `src/main.rs` is the integrated command boundary.

The transformer integration will extend these boundaries. It will not replace
the recursion runtime, constitutional runtime, gameplay runtime, or Runtime
Federation.

### Authoritative kernel

There are two related, physically separate kernel surfaces:

- `hollow-grove-recursion-kernel/src/lib.rs` owns the dependency-free, bounded
  cycle topology and Pleb/Meta routing grammar.
- `hollow-grove-semantic-kernel/src/lib.rs` owns isolated, House-neutral
  geometry, composition, and provenance.

The root crate adapts application-owned `Point`, progression, and world state
through that neutral routing grammar in `src/symptom.rs`, `src/triway.rs`,
`src/hollow_grove.rs`, `src/grove_seam.rs`, `src/hollow_beam.rs`, and
`src/kernel_pass.rs`. `src/pleb_meta.rs` is only a compatibility re-export.

`src/composition.rs` is only a compatibility re-export of
`hollow-grove-semantic-kernel`; it is not a second implementation.

The fixed-point coordinate primitives and neutral scoped position records will
be added to `hollow-grove-semantic-kernel`. House names and constitutional
interpretation will remain in the root crate so the neutral-kernel purity
boundary stays intact.

### Point and Point²

- The existing recursion `Point` is `src/point.rs::Point`.
- Its stable physical identity is represented through
  `hollow_grove_semantic_kernel::OrientedPoint` and related IDs in
  `hollow-grove-semantic-kernel/src/oriented_point.rs`.
- Point² is an established progression operation rather than a duplicate
  `PointSquared` entity type. Its principal records are
  `PointSquaredAscension`, `PointSquaredApplication`, and
  `PointSquaredSemanticClaim` in `src/point_progression.rs` and
  `src/hollow_grove_contract.rs`.

Semantic coordinates will reference a Point through `ExternalRef`; they will
not alter `Point`, `OrientedPoint`, physical coordinates, or Point²
progression.

### Catalogs

- `hollow_grove_semantic_kernel::CompositionCatalog` is the authoritative neutral
  catalog for addressable nodes, causal composition records, and containment.
- `SpatialInheritanceCatalog` is a separate physical map-template catalog and
  is not suitable for contextual meaning.
- `InstitutionCatalog` owns institutions.
- `FunctionalLoreCatalog` owns bounded functional-lore projections.

Semantic position storage will extend `CompositionCatalog`. No
`SemanticCatalog` will be introduced.

### Composition, causality, and provenance

`hollow-grove-semantic-kernel/src/lib.rs` provides:

- `CompositionNode`
- `CompositionRecord`
- `CompositionCatalog`
- source and result indexes
- caller-supplied stable identities
- explicit operation and evidence references

`src/world/composition_witnesses.rs` proves that existing domain systems can
share this neutral catalog without changing authority.

`CausalPosition`, constitutional event records, `FederatedEventReference`,
and domain archives provide higher-level causal order. Semantic assertion and
translation will be represented as typed semantic operations associated with
ordinary `CompositionRecord` provenance rather than a second event log.

### Containment

`hollow_grove_semantic_kernel::Containment` is direct structural membership.
`CompositionCatalog::add_containment` rejects missing endpoints,
self-containment, duplicates, and cycles. Its reachability check is the
existing containment-cycle detector.

Semantic position will neither read as containment nor create containment.
Containment will not update semantic position.

### Stable identity

The repository consistently uses validated caller-supplied ASCII-safe IDs:

- `CompositionNodeId`, `CompositionRecordId`, `ScaleKey`, and spatial IDs in
  `hollow-grove-semantic-kernel/src/lib.rs`
- constitutional IDs in `src/constitutional/ids.rs`
- cycle IDs in `src/constitutional/grove_phase.rs`
- domain-specific stable IDs throughout `src/world/` and `src/gameplay/`

Canonical archive digests use deterministic FNV-1a over canonical serialized
data. IDs are not generated from insertion order or random values.

Semantic record IDs will reuse `CompositionRecordId`; point identity will
reuse `ExternalRef`; scale will reuse `ScaleKey`; cycle identity will reuse
`GroveCycleId`. A distinct semantic proposal ID is genuinely required because
a proposal is neither a composition record nor a confirmed state.

### External references and evidence

`hollow_grove_semantic_kernel::ExternalRef` is the neutral reference to a
domain-owned object, operation, or evidence trace.
`src/constitutional/model.rs::EvidenceRef` is a constitutional wrapper around
`ExternalRef`.

The repository has several domain-specific evidence records, but no universal
evidence catalog and no generic witness record. Semantic validation will use
an explicit reference resolver over `ExternalRef`. It will not reinterpret
domain evidence types or manufacture evidence.

### Houses and anchors

The reused constitutional House type is
`src/hollow_grove_contract.rs::House`, with exactly:

- `Stonebend`
- `Sandmanor`
- `Glaushouse`
- `Flynt`

The internal identifier remains ASCII-safe. The semantic boundary will render
the canonical display name `Glaüshouse`.

`Central Junction` is already protected as a non-House in constitutional,
route, gameplay, and documentation tests. A new `KernelAnchor` wrapper is
needed to distinguish the four `House` variants from `CentralJunction`.

Existing map, oriented-point, and normalized spatial types describe physical
or presentation geometry. None implements the directive's bounded,
reserved-Y semantic coordinate field, so reusing one would collapse distinct
relations.

### Four-phase cycle

`src/constitutional/grove_phase.rs` already owns the exact public grammar:

1. `GrovePhase::TheWayBack`
2. `GrovePhase::TheInitiation`
3. `GrovePhase::TheGathering`
4. `GrovePhase::TheFestival`

It validates exact phase order, accepted and rejected resolution, continuous
next-state return, deterministic replay, and migration from legacy abstract
verbs. These public phase names will be reused.

`src/runtime_federation.rs` already records phase evidence and requires a
Festival result to become the next Way Back state. Semantic orchestration will
extend this single Federation boundary with Roï continuity and Rosaliña
world-state interfaces. It will not create a Runtime Federation/World
Federation competitor.

### Intent and synthesis

The repository has domain-specific intents (`GameplayIntent`,
`DecisionIntent`, `RecipeIntent`, runtime screen-map intents) and
domain-specific Synthesis records. No existing intent enum describes the
model-facing semantic allow-list without granting domain execution.

A small `ProposedSemanticOperation` enum is therefore required. Its values are
untrusted semantic requests. Confirmed semantic assertion and spatial
translation will attach typed operation data to existing composition
provenance; they will not replace recipe execution or Point².

### Persistence and migration

Established persistence patterns include:

- strict versioned envelopes
- `serde(deny_unknown_fields)`
- checksums over canonical payloads
- explicit decoding of supported historical versions
- rejection of unknown future versions
- deterministic re-encoding and idempotent migration

Representative implementations are:

- `src/runtime_federation.rs`
- `src/gameplay/archive.rs`
- `src/constitutional/persistence.rs`
- `src/constitutional/regional_persistence.rs`
- `src/world/world_point_archive.rs`
- `src/world/spatial_inheritance.rs`

Existing Points without semantic positions will remain valid. Semantic V1
will use an optional/additive archive payload and will never infer or backfill
coordinates from names.

### CLI and TUI

- `src/main.rs` is the public integrated launcher and command router.
- `src/bin/current_synthesis_tui.rs` is the established text inspection
  surface for Current Synthesis.
- `src/constitutional/tui.rs` is the constitutional TUI projection boundary.

Semantic commands will be routed by `src/main.rs` to one focused semantic CLI
implementation. Mutating semantic commands will call the Federation
orchestrator; inspection and validation commands remain read-only.

The local hosted-model-style boundary extends that same focused semantic
binary rather than creating another runtime:

- `hollow-grove federation serve` owns loopback-only HTTP transport.
- `FederationHttpApi` owns staged catalog execution and atomic provenance
  persistence.
- `POST /v1/federation/cycles` calls the same four-phase orchestrator used by
  `federation run`.
- `/health` and `/v1/models` are read-only discovery routes.

Concurrent connections serialize their semantic cycles through one existing
`CompositionCatalog`; no API-specific semantic catalog or confirmation path is
introduced.

### Godot boundary

- `src/gameplay/protocol.rs` owns strict Rust request/response envelopes.
- `src/bin/hollow_grove_game_service.rs` owns loopback transport.
- `hueman_godot/scripts/runtime_client.gd` is transport-only.
- `hueman_godot/protocol/` contains shared protocol documentation and frozen
  fixtures.

Semantic Godot events will be Rust-owned serialized event structures.
Godot will receive proposed/confirmed/rejected presentation data and will not
write canonical semantic state.

### Model boundary

No model abstraction or Ollama integration existed at discovery. The only
Ollama reference is packaging metadata advertising an optional external
application.

The new model-independent `SemanticInterpreter` boundary will have:

- `DeterministicSemanticInterpreter` for tests and offline operation
- `OllamaSemanticInterpreter` as the only inference backend

Neither implementation receives a mutable catalog.

## Reused types

| Existing type | Semantic use |
| --- | --- |
| `House` | the four constitutional House anchors |
| `ScaleKey` | required semantic scale identifier |
| `ExternalRef` | Point, context, evidence, witness, canon, cycle, and actor references |
| `CompositionNode` / `CompositionNodeId` | stable addressable Point projection |
| `CompositionRecord` / `CompositionRecordId` | confirmed semantic causal provenance and semantic record identity |
| `CompositionCatalog` | authoritative integrated storage and query boundary |
| `Containment` | retained unchanged to prove relation separation |
| `CompositionCatalogError` | existing duplicate, missing-reference, and containment failures |
| `GrovePhase` | exact four public phases |
| `GroveCycleId` | runtime-assigned cycle identity |
| `GroveCycleRecord` / `GroveCycleRuntime` | phase-order and returning-state continuity |
| `EvidenceRef` | bridge to constitutional evidence where applicable |
| `FederatedEventReference` | cross-domain causal reference at the Federation boundary |
| `RuntimeFederation` | existing unified replay and continuity boundary |

## New types genuinely required

Neutral kernel types:

- `AxisValue`
- `AxisDelta`
- `Position3`
- `Translation3`
- `CoordinateError`
- `SemanticScope`
- `SemanticOperation`
- `SemanticPositionRecord`
- `SemanticRejectionRecord`
- semantic indexes inside `CompositionCatalog`

Root constitutional/runtime types:

- `KernelAnchor`
- `HouseDistance`
- `OriginRelation`
- `SemanticProposalId`
- `ModelSemanticProposal`
- `SemanticProposal`
- `ProposalStatus`
- `SemanticDecision`
- confirmed/rejected transition records
- validation errors, warnings, and report
- semantic witness and archive records
- Way Back, Initiation, Gathering, and Festival results
- `RoiContinuity`
- `RoselinaWorldState`
- semantic extension of the existing Federation orchestrator
- retrieval request/bundle/fragment/example types
- `SemanticInterpreter`
- deterministic and Ollama interpreter implementations
- semantic Godot event structures
- deterministic dataset records and split manifest

These types represent relationships not already encoded by physical
orientation, normalized maps, Point², gameplay intent, or domain-specific
evidence.

## Dependency map

```text
Hollow Grove canon documents and existing House law
    ↓
hollow-grove-semantic-kernel semantic geometry
    ↓
CompositionCatalog scoped semantic records
    ↓
existing CompositionRecord provenance and containment validation
    ↓
root constitutional semantic validation and witnesses
    ↓
existing Runtime Federation + four GrovePhase values
    ↓
SemanticInterpreter trait and retrieval inputs
    ↓
Ollama adapter
    ↓
root CLI/TUI and Rust-owned Godot events
```

Enforced negative dependencies:

- `hollow-grove-semantic-kernel` will not contain House names, Federation names,
  Ollama, Godot, or canon document paths.
- geometry will not depend on model response types.
- validation will not depend on Ollama wire JSON.
- composition will not depend on presentation.
- retrieval and tests will work without Ollama.
- Godot will not receive a mutation interface.

## Risks and resolutions

### Naming conflicts

- Existing `PhysicalPosition`, `NormalizedPoint`, `PointLocation`, and
  rotational positions are physical or presentation concepts.
- Resolution: introduce the deliberately specific semantic `Position3` in
  the neutral kernel and never alias it to those types.
- Existing `HouseAnchor` means a body-function relationship, not a coordinate.
- Resolution: use `KernelAnchor` for constitutional coordinate anchors.

### Federation terminology

Corrected canon defines one Federation and two continuous beings expressed
through inside Guide forms and outside Seer forms.

Resolution: keep the one existing executable `RuntimeFederation` and add
paired identity-derived Guide contexts to its semantic orchestration boundary.
Treat `RuntimeFederation` as HGRF compatibility machinery. Do not create a
second executable Federation, change House constitutions, or rewrite the
Hollow Grove Compromise.

### Serialization compatibility

`ExternalRef`, `CompositionNode`, `CompositionRecord`, and `Containment` were
not all serializable at discovery.

Resolution: add strict, validated serialization only where the semantic
archive needs it; keep prior in-memory behavior source-compatible. Use an
additive semantic schema with an explicit marker and fail closed on unknown
versions.

### Migration

Automatically assigning semantic positions to existing Points would invent
history.

Resolution: empty semantic state is valid; V1 migration preserves absence.
Only witnessed assertions establish the first position for a scope.

### Duplicate concepts

Physical orientation, normalized map placement, House classification, Point²,
and semantic context can look superficially similar.

Resolution: document and test that identity, physical placement, containment,
causality, and semantic position remain distinct.

### Circular dependencies

The neutral kernel cannot import the root `House` type.

Resolution: compile neutral coordinate constants into `Position3`; map
`House` to those constants in the root `KernelAnchor` implementation.

### Model/runtime coupling

Parsing a model response could accidentally become authorization.

Resolution: the model type contains no canonical IDs or mutable references.
The runtime wraps it, resolves every reference, re-reads source state, and
commits only in Festival after complete validation.

### Transactionality

The current neutral catalog mutates one record at a time.

Resolution: prepare and validate semantic, composition, witness, and world
effects against staged state, then swap or commit the staged state as one
Festival result. Rejection records may be retained but cannot change the
current semantic index.

### Network and test availability

No local model can be assumed during tests.

Resolution: all tests use the deterministic interpreter or mocked Ollama
transport. The catalog, retriever, parser, validator, Federation, dataset
exporter, CLI inspection, and Godot event tests remain model-independent.
