# Hollow Grove V2 TUI Readiness Contract

Status: application service and interface implemented; terminal client deliberately not implemented
Canonical types: `src/constitutional/application.rs`, `src/constitutional/tui.rs`
Canonical source of truth: constitutional and regional reducers

## 1. Purpose

This contract defines the boundary a future terminal client may consume. The
client presents commands, transition traces, authority checks, evidence checks,
replay comparisons, persistence results, and regional assignments. It does not
decide constitutional law.

The interface exists so terminal rendering can evolve independently from the
recursion kernel, Bond reducer, regional Synthesis reducer, archive codecs, and
scenario catalog.

## 2. Architectural Boundary

```text
terminal input
    ↓
public application/service command boundary
    ↓
canonical runtime command or read-only query
    ↓
production reducer / archive / replay
    ↓
ConstitutionalTrace (read-only projection)
    ↓
TuiEvent (presentation-neutral record)
    ↓
terminal rendering
```

The future TUI MUST NOT call `BondAggregate::apply`, reproduce
`RegionalSynthesisRule::for_transition`, infer a House from a label, construct
assignments from location, or mutate persisted state. It submits public commands
to an application boundary and renders returned events.

## 3. Command Model

`TuiCommand` is the request vocabulary:

| Command | Read-only | Meaning | Required application action |
|---|---:|---|---|
| `Catalog` | yes | list supported scenarios and expectations | read `SCENARIO_CATALOG` |
| `RunScenario { scenario }` | no | execute and select an isolated demonstration | call the public scenario function, which submits production commands; retain its runtime/archive as the selected snapshot |
| `ReplayScenario { scenario }` | yes | compare accepted history with production replay | invoke the matching replay entry point |
| `PersistScenario { scenario }` | yes | encode and decode canonical history | use public archive functions |
| `MigrateScenario { scenario }` | yes | migrate a supported archive | use public migration function |
| `InspectTrace { scenario }` | yes | obtain transition projections | derive from actual events/outcome |
| `InspectAuthority { scenario }` | yes | show historical House decisions | read committed command/event data |
| `InspectEvidence { scenario }` | yes | show stable evidence references | read committed command/event data |
| `InspectPolarity { scenario }` | yes | show independent signed domains | read accepted evaluation |
| `InspectLineage { scenario }` | yes | show predecessor/result chain | call `RegionalSynthesisRuntime::lineage` |
| `InspectRegion { scenario }` | yes | show established standing/assignment region | call read-only regional inspection |
| `InspectStewardship { scenario }` | yes | show Minotaur field assignment | call `stewardship` |
| `InspectGuardianship { scenario }` | yes | show Centaur sea assignment | call `guardianship` |
| `Audit` | yes | execute conformance comparisons and inventory checks | application-owned orchestration |
| `Cancel { request_id }` | no domain mutation | request cancellation of orchestration | stop only at a safe command boundary |

`TuiCommand::is_read_only` reports the classification. “Run” is classified as
mutating because it constructs an isolated scenario runtime, even though it does
not touch a repository archive unless an application explicitly supplies one.

## 4. Event Model

`TuiEvent` contains:

- monotonically increasing stream-local `sequence`;
- stable scenario name;
- typed `TuiEventKind`;
- a deterministic `BTreeMap<String, String>` of presentation fields.

The implemented event kinds are:

- `SessionStarted`;
- `CatalogEntry`;
- `CatalogCompleted`;
- `ScenarioStarted`;
- `TransitionProposed`;
- `AuthorityChecked`;
- `EvidenceChecked`;
- `PolarityObserved`;
- `TransitionAccepted`;
- `TransitionRejected`;
- `StateChanged`;
- `Persisted`;
- `ReplayStarted`;
- `ReplayCompleted`;
- `MigrationStarted`;
- `MigrationCompleted`;
- `WaveCreated`;
- `RegionEntered`;
- `SynthesisProposed`;
- `SynthesisAccepted`;
- `SynthesisRejected`;
- `LineagePreserved`;
- `StewardshipGranted`;
- `GuardianshipGranted`;
- `AuraFieldsAssigned`;
- `AuraBeachAssigned`;
- `AuraSeaGuardianshipAssigned`;
- `ScenarioCompleted`;
- `AuditCompleted`.
- `CancellationAccepted`;
- `RequestCancelled`.

Not every projection emits every available kind. The vocabulary is stable for
an application service to provide finer-grained streaming without changing the
terminal representation.

## 5. Serialization

`TuiEvent::encode_line` produces one tab-separated record:

```text
sequence<TAB>escaped-scenario<TAB>EventKind<TAB>escaped-key=escaped-value...
```

Backslash, tab, newline, and equals are escaped. Fields originate from a
`BTreeMap`, so wire order is deterministic. `decode_line` rejects:

- missing sequence, scenario, or kind;
- invalid sequence;
- unknown kind;
- malformed key/value field;
- duplicate key;
- invalid escape.

The codec has a conformance round-trip test for every event emitted by both
lawful regional scenarios. A future transport MAY wrap records in a framed
protocol but MUST preserve type, sequence, and field meaning.

## 6. Request and Response Boundary

Each request has an application-owned request identity outside the domain event
identity. The implemented `TuiRequest` contains that identity and one
`TuiCommand`. `ConstitutionalApplicationService::execute` returns a
`TuiResponse` containing zero or more ordered `TuiEvent` records and exactly one
terminal `ApplicationResponseStatus`:

- completed successfully;
- rejected by typed constitutional law;
- cancelled at a safe boundary;
- failed before reaching the reducer due to application/transport input.

A constitutional rejection is a successful application response containing a
typed rejection event. It is not transport failure and MUST remain visibly
different from parser, I/O, or process errors.

Exact retry of one request identity and command returns the exact stored
response without re-executing the scenario. Reuse of that identity for a
different command returns `ApplicationServiceError::RequestIdConflict` and
does not replace the selected state.

## 7. Streaming Boundary

The application MAY stream observation events after each completed production
operation. It MUST NOT emit `TransitionAccepted`, `StateChanged`,
`SynthesisAccepted`, `StewardshipGranted`, or `GuardianshipGranted` before the
production reducer has returned success.

Recommended accepted transition order:

```text
TransitionProposed
AuthorityChecked
EvidenceChecked
TransitionAccepted
StateChanged
```

Recommended regional accepted order:

```text
SynthesisProposed
AuthorityChecked
EvidenceChecked
SynthesisAccepted
LineagePreserved
AuraFieldsAssigned / AuraBeachAssigned
StewardshipGranted / AuraSeaGuardianshipAssigned
```

Recommended rejection order:

```text
TransitionProposed or SynthesisProposed
AuthorityChecked and/or EvidenceChecked where reached
TransitionRejected or SynthesisRejected
```

There is no `StateChanged` after a rejection.

## 8. Cancellation Boundary

Cancellation may occur:

- before submitting a command;
- after a reducer call returns and before the next independent command;
- between scenario operations;
- between replay batches if no partial replay is presented as final state.

Cancellation MUST NOT interrupt an event append halfway, truncate an archive
write and report it as persisted, or present a partial migration as canonical.
The in-memory reducers are synchronous and atomic at their public call boundary.
The implemented synchronous service accepts cancellation for a request that
has not begun. The target request then returns `Cancelled` before scenario
construction or selection. It rejects self-cancellation and cancellation of a
completed request. Mid-reducer interruption remains forbidden.

## 9. Approval Boundary

Read-only catalog, trace, replay, lineage, region, stewardship, and guardianship
queries need no domain approval. An application MAY require user confirmation
before:

- submitting a command to a non-demo live runtime;
- overwriting an archive path;
- replacing a current archive with migrated bytes;
- invoking an external authority/evidence source.

Approval is application safety, not House authority. Clicking “approve” MUST NOT
create Stonebend, Sandmanor, Flynt, or Glaüshouse authority.

## 10. Persistence Boundary

The TUI receives a `Persisted` event only after the archive function succeeds.
It may display:

- archive version;
- byte length;
- stable digest;
- canonical decode equality;
- path chosen by the application.

It MUST NOT edit archive bytes, deserialize unchecked state, or mark a failed
write as persisted. Regional decode derives assignments through
`RegionalSynthesisRuntime`; it does not trust a rendered duty list.

The implemented service retains selected archive bytes privately. Its public
metadata view exposes scenario, runtime kind, byte length, and digest, never a
mutable runtime or byte buffer. File-path ownership and overwrite approval
remain responsibilities of a future host process, not the terminal renderer.

## 11. Error Rendering

Errors have two layers:

1. stable code for filtering, testing, and concise rendering;
2. descriptive detail for developers.

Regional failures use `RegionalSynthesisError::code`, including
`REGIONAL_ILLEGAL_LINEAGE_TRANSITION`, `REGIONAL_INSUFFICIENT_STANDING`,
`REGIONAL_MISSING_EVIDENCE`, `REGIONAL_EVIDENCE_SUBJECT_MISMATCH`, and
`REGIONAL_HOUSE_LAW_REJECTED`.

The renderer MUST display:

- attempted command;
- prior state;
- required authority/evidence/region when available;
- stable failure code;
- `State Changed: No` for a reducer rejection.

It MUST NOT silently coerce a wrong form, region, function, or institution.

## 12. Regional Synthesis Rendering

The canonical Gnome projection is:

```text
Gnome
  ↓ lawful regional Synthesis
Minotaur
  Region: Aura Field
  Role: Steward, worker, maintainer, defender
```

The renderer obtains `Gnome`, `Minotaur`, `Aura Field`, and duties from the
accepted `RegionalSynthesisRecord` and `AuraFieldsStewardship`. It does not
choose the result from the source form.

The canonical Elf projection is:

```text
Elf
  ↓ lawful regional Synthesis
Centaur
  Region: Aura Beach
  Role: Roamer and shoreline patrol
  Guardianship: Aura Sea
```

The renderer obtains beach occupation and sea guardianship from the two typed
Centaur assignments. “Aura Sea” is not cosmetic text: it is
`AuraSeaGuardianship.region`.

## 13. Lineage Rendering

Lineage display consumes `RegionalLineageEntry` in canonical order. Each row
shows stable Being identity, form, and optional Synthesis identity. A renderer
MUST NOT collapse predecessor and result identities or omit the source when it
shows a transformed Being.

Example:

```text
[0] Gnome     being.gnome-minotaur.origin
[1] Minotaur  being.gnome-minotaur.result  synthesis.gnome-minotaur
```

## 14. Stewardship and Guardianship Rendering

Stewardship display consumes `AuraFieldsStewardship` and its ordered duty set.
Guardianship display consumes `AuraSeaGuardianship` and its ordered duty set.
The TUI MAY group or abbreviate duties visually but MUST retain a detail view
that exposes every typed duty, holder, region, authority decision, and evidence
reference.

The TUI MUST NOT:

- render a Gnome as already possessing Minotaur stewardship;
- render an Elf as already possessing Centaur guardianship;
- render Minotaur sea authority;
- render Centaur field authority;
- infer a transformation from occupancy;
- hide a failed lineage or authority check.

## 15. Implemented Public Service Boundary

`ConstitutionalApplicationService` is the only implemented TUI-facing mutating
boundary. It:

- accepts `TuiRequest` values containing `TuiCommand`;
- constructs scenarios only through the production-backed catalog fixtures;
- owns exactly one selected runtime snapshot and its canonical archive;
- keeps runtime mutation and archive bytes private;
- returns deterministic, request-local `TuiEvent` sequences;
- exposes `execute_streaming` for ordered callback delivery without exposing
  runtime state or letting the sink influence an already-decided result;
- separates completed, constitutionally rejected, and cancelled statuses;
- makes exact request retries idempotent;
- rejects request-ID conflicts before state selection;
- runs replay, persistence, migration, and read-only inspections against the
  selected owned snapshot;
- emits accepted/state-change events only from reducer-accepted traces;
- emits no state-change event for a constitutional rejection.

The service is synchronous and demonstration-scoped. It does not expose a live
gameplay command that has not been constitutionally specified. The
`constitutional_v2_service` example is an executable protocol witness; its
argument parser is not part of the service API.

## 16. Verification Contract

A conforming TUI integration must test:

- wire round-trip and deterministic field order;
- accepted event emitted only after reducer success;
- rejection has no state-change event;
- trace/event projection leaves runtime event count unchanged;
- Gnome→Minotaur renders Aura Field and every stewardship duty;
- Elf→Centaur renders Aura Beach and Aura Sea guardianship;
- both lineage chains show source and result identities;
- reversed Syntheses render stable rejection codes;
- persistence/replay equality is displayed from actual comparisons;
- cancellation never exposes partial canonical state;
- no UI type has an assignment or legality mutation method.

`tests/constitutional_application_service.rs` verifies the implemented service
boundary, including selection, private archive metadata, stable wire records,
batch/stream equivalence, request retry, cancellation, accepted/rejected
Synthesis, regional authority separation, persistence, replay, migration, and
audit behavior.
