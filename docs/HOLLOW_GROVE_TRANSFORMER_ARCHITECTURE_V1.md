# Hollow Grove Transformer Architecture V1

## Constitutional foundation

```text
Central Junction is Origin.

Hollow Grove is positive Z.
Stonebend is the positive-Z House Point.

Grove Hollow is negative Z.
Glaüshouse is the negative-Z House Point.

Sandmanor is positive X.
Flynt is negative X.

Y exists structurally but remains reserved in V1.

The four Houses are finite Points.

Their synonyms, expressions, combinations, and overlaps are infinite.

A transformer may reveal where something appears to stand.

Only the constitutional runtime may confirm its canonical position.

A confirmed transformation becomes the returning state
of The Way Back again.
```

The implementation keeps this division literal:

```text
Transformer interpretation = Aura
Rust authority             = Current
Validated movement         = Synthesis
Contextual position        = Relativity
```

## Dependency direction

```text
existing constitutional canon and House
        ↓
hollow-grove-semantic-kernel semantic geometry
        ↓
hollow-grove-semantic-kernel scoped semantic records
        ↓
existing CompositionCatalog provenance and replay
        ↓
existing Runtime Federation semantic orchestration
        ↓
model-independent SemanticInterpreter
        ↓
local-only Ollama adapter
        ↓
read-only CLI and Godot event projections
```

The kernel has no Ollama, CLI, or Godot dependency. The catalog and all tests
remain usable without a running model.

## Geometry

Canonical state uses checked fixed-point integers:

```text
Stonebend       [    0, 0,  1000]
Sandmanor       [ 1000, 0,     0]
Glaüshouse      [    0, 0, -1000]
Flynt           [-1000, 0,     0]
Central Junction[    0, 0,     0]
```

`AxisValue` accepts `-1000..=1000`. `AxisDelta` accepts
`-2000..=2000`. `Position3` and `Translation3` reject nonzero Y.
Translations use checked arithmetic and must satisfy
`source + translation = destination`.

`KernelAnchor::CentralJunction` is separate from the existing four-variant
`House` enum. House relationships are derived as all four squared Euclidean
distances in the constitutional tie order Stonebend, Sandmanor, Glaüshouse,
Flynt. A coordinate is allowed to overlap several directions; no winner or
percentage becomes canonical.

## Scoped records and history

`SemanticScope` reuses `ScaleKey` and an optional `ExternalRef` context.
`SemanticPositionRecord`, `SemanticWitnessRecord`, and their snapshots live in
the existing `CompositionCatalog`. The current index is keyed by Point and
scope. History is append-only and links to the previous composition record.

Position assertion and spatial translation are distinct `SemanticOperation`
variants. A position does not create identity, containment, ownership,
succession, or an unrelated causal claim. Confirmed movement adds a checked
composition record and witness. Rejection adds only a rejection witness.
Snapshot replay is deterministic and transactional.

Existing records without semantic positions remain valid. V1 decoders use
explicit schema markers and do not infer positions from names. The legacy
witness migration supplies only an `unspecified-v1` operation marker, and
legacy dataset projection supplies only a noncanonical `point.unknown-v1`
display placeholder.

## Federation cycle

Every transformer-assisted action follows:

```text
The Way Back
→ The Initiation
→ The Gathering
→ The Festival
→ The Way Back again
```

The Way Back restores identity, current scoped position, history, evidence,
authority, canon references, world references, and nearby records.

The Initiation calls `SemanticInterpreter`, strictly parses a proposal, assigns
runtime IDs and actor/Point/scope, and performs no canonical mutation.

The Gathering resolves evidence claims, verifies Point and source state,
prepares replay, invokes constitutional and embodiment validators, and collects
typed preflight failures.

The Festival re-reads the source record, rejects stale state, prepares
deterministic records, stages catalog and world results, and commits all
approved mutation only after validation. A failure before commit leaves the
canonical position unchanged. The resulting state is restored as the next Way
Back context.

Roï continuity and the world-state counterpart jointly serve the one existing
Runtime Federation through traits. The default world-state implementation is
read-only. Domain-specific House, Compromise, containment, succession, and
embodied-effect rules plug into those validation traits; they are never
delegated to the model.

## Interpretation and retrieval

`SemanticInterpreter` has two implementations:

- `DeterministicSemanticInterpreter` for offline operation and failure tests.
- `OllamaSemanticInterpreter` as the only inference backend.

Ollama reads `HOLLOW_GROVE_MODEL`; its endpoint defaults to the local
`127.0.0.1` service and may be overridden with
`HOLLOW_GROVE_OLLAMA_URL`. Nonlocal endpoints, unbounded responses, arbitrary
tools, file mutation, and command execution are not accepted.

The versioned prompt is
`ai/prompts/hollow-grove-semantic-interpreter-v1.txt`. Responses reject unknown
fields, missing fields, invalid operations, excessive strings or arrays,
out-of-range coordinates, nonzero Y, and confidence outside `0..=1000`.
Parsing proves syntax only.

`CatalogCanonRetriever` resolves only supplied exact canon references and
returns bounded nearby examples. Nearby ordering is same scale, squared
distance, then stable record ID. When no contextual coordinate exists it
returns no nearby examples; it never substitutes origin for uncertainty.

## Presentation boundaries

The integrated CLI surfaces are:

```text
hollow-grove semantic inspect ...
hollow-grove semantic history ...
hollow-grove semantic nearby ...
hollow-grove model validate-response ...
hollow-grove dataset validate-semantic ...
hollow-grove dataset split-semantic ...
```

Semantic inspection reads witnessed JSONL. It visually distinguishes proposal,
confirmation, and rejection and cannot mutate canonical state.

Godot receives `hollow-grove-semantic-event-v1` events only. Proposed Aura is
temporary and unconfirmed; confirmed Current is persistent and witnessed.
There is no inverse semantic mutation command in the protocol.

## Witnessed datasets

Confirmed and rejected witnesses export as deterministic
`hollow-grove-semantic-witness-v1` JSONL:

- stable witness ordering,
- stable field order,
- one object per line,
- final newline for nonempty output,
- no random shuffle or generated timestamp,
- no hidden chain-of-thought.

Training preparation validates the same schema and splits by a stable hash of
an explicit seed plus witness reference. The provided shared-adapter template
is disabled. No training or custom coordinate head is implemented in V1.

## Remaining integration seams

The V1 kernel and complete proposal path are operational. Production
deployment still requires repository canon to be registered as retrievable
fragments, a selected local Ollama model, and domain validators/world-effect
implementations for each concrete gameplay or operating-system operation.
Those seams are intentional: the generic semantic kernel must not guess
House-law decisions or mutate external state on its own.
