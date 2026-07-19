# Hollow Grove Repository Authority Map

Status: review and consolidation guide
Purpose: make repository ownership visible without creating new domain law

## Read Order

Use this order when reviewing Hollow Grove V2:

1. `HOLLOW_GROVE_V2_CAPABILITY_REPORT.md` for the guided tour.
2. `V2_CAPABILITY_MATRIX.md` for a compact support and failure map.
3. `V2_CAPABILITY_INVENTORY.md` for exact types, paths, gaps, and stability.
4. `HOLLOW_GROVE_V2_CONSTITUTIONAL_SPECIFICATION.md` for normative law.
5. House constitutions for the authority assigned to each House.
6. `src/constitutional/` for production enforcement.
7. `examples/` and `tests/` for executable proof.
8. `artifacts/` only for generated or runtime-facing projections.

## Authority Classes

### Normative Constitutional Law

These sources define meaning and legality:

- `HOLLOW_GROVE_V2_CONSTITUTIONAL_SPECIFICATION.md`;
- `STONEBEND_CONSTITUTION_V1_DRAFT.md`;
- `SANDMANOR_CONSTITUTION_V1_DRAFT.md`;
- `GLAUSHOUSE_CONSTITUTION_V1_DRAFT.md`;
- the existing Flynt constitutional law and conformance surfaces;
- `SANDMANOR_LINEAGE_LOCK_v1.md`;
- `CURRENT_SYNTHESIS_HOLLOW_GROVE_WORLD_CONTEXT_v0.1.0.md` for the existing
  world and ontology context incorporated by the runtime.

Normative documents may be expanded or amended, but generated artifacts,
examples, traces, and presentation code must not silently redefine them.

### Production Enforcement

`src/constitutional/` is the V2 enforcement layer above the pure recursion
kernel:

- `bond.rs` and `runtime.rs`: Bond state and append-only orchestration;
- `houses.rs`: institution-backed House decisions;
- `regional.rs`: the two ratified regional Synthesis rules and assignments;
- `persistence.rs` and `regional_persistence.rs`: canonical codecs and replay;
- `adapters.rs`: completed-kernel-pass and external evidence boundaries;
- `application.rs`: the presentation-facing owner of a selected runtime and
  archive;
- `trace.rs` and `tui.rs`: read-only observation and transport projection.

The two recursion kernels remain below this layer and own no constitutional
geography, authority, evidence, or regional Synthesis decision.

### Executable Proof

- `src/constitutional/scenarios.rs`: reusable production-backed fixtures;
- `examples/constitutional_v2.rs`: human capability demonstrator using the
  application service for ordinary commands;
- `examples/constitutional_v2_service.rs`: minimal wire-protocol witness;
- `examples/constitutional_v2_bench.rs`: dependency-free performance harness;
- `tests/constitutional_*.rs` and `tests/regional_synthesis.rs`: conformance;
- `tests/kernel_purity.rs`: kernel-boundary protection.

Proof surfaces exercise production law. They do not possess independent domain
authority.

### Derived Review Documents

- `V2_CAPABILITY_INVENTORY.md` records repository mapping and implementation
  status;
- `V2_CAPABILITY_MATRIX.md` records cross-surface coverage;
- `HOLLOW_GROVE_V2_CAPABILITY_REPORT.md` explains the implemented system;
- `V2_PERFORMANCE_CHARACTERIZATION.md` records measurements;
- `V2_TUI_READINESS_CONTRACT.md` defines the presentation boundary.

These documents explain or measure the system. If they conflict with normative
law or reducer behavior, the conflict is a defect to resolve rather than a new
rule.

### Generated And Runtime Artifacts

`artifacts/` contains client projections, status, snapshots, event logs,
benchmarks, and runtime handoffs. The artifact index identifies the individual
families.

`artifacts/current_synthesis_world_context.md` is intentionally an exact
generated mirror of
`CURRENT_SYNTHESIS_HOLLOW_GROVE_WORLD_CONTEXT_v0.1.0.md`. The root document is
authoritative; the artifact exists because runtime clients consume a stable
artifact path.

Generated artifacts must either be reproducible, be an explicitly named golden
fixture, or be ignored as local runtime state. File size alone does not make an
artifact authoritative.

## Artifact Retention Inventory

Use these retention classes when deciding whether an artifact belongs in a
change. The classes describe repository ownership; they do not grant domain
authority to an artifact.

| Class | Repository treatment | Current examples |
| --- | --- | --- |
| Checked interface contract | Track and review semantically | `hollow_grove_application_registry.json`, `hollow_grove_hueman_coordinate_contract.json`, `hueman_screen_map.json` |
| Checked deterministic projection | Track when it is a deliberate review witness; refresh through its producer | `kernel_pass_snapshot.json`, the checked Current Synthesis and Hueman Markdown families, ontology and movement witnesses |
| Intentional generated mirror | Track only because a stable consumer path requires it | `current_synthesis_world_context.md` |
| Historical golden record | Preserve as named evidence; never refresh as ordinary runtime output | `current_synthesis_v1_freeze_snapshot.md`, `current_synthesis_v1_refinement_baseline.md`, `version_2_decision_engine_status.md` |
| Local runtime state | Ignore; it changes with the active session | runtime, niri, screen-map intent/state, pair-preview, Current Synthesis TUI/status/inspector/event files |
| Local measurement output | Ignore; publish conclusions in a reviewed report | benchmark output, `adversarial_verification_report.*`, and `artifacts/verification/` |
| Failure reproducer | Ignore raw crash output; promote a minimized reproducer into a checked test or `fuzz/corpus/` | timestamped fuzz logs and crash directories |

The three historical golden records above are currently untracked review
candidates. They are not ignored because they contain deliberate milestone
context rather than disposable run state. They should be included or rejected
explicitly when the surrounding milestone is committed.

Some older tracked projections have live-sounding names, including
`desktop_status.txt` and `current_synthesis_snapshot.txt`. They remain tracked
compatibility witnesses in this pass. Converting them to local state requires a
separate consumer audit and a deliberate history decision; `.gitignore` cannot
and should not silently untrack them.

The exhaustive per-file family index remains in `artifacts/index.md`.

## Compatibility Surfaces

- `hollow_grove::Bond` remains a compatibility alias for `KernelBond`.
- Constitutional Bond types live under `hollow_grove::constitutional`.
- Public historical snapshot fields remain compatibility surfaces where noted
  by the capability inventory.
- `src/constitutional/mod.rs` currently re-exports each constitutional module
  through the established façade. Those broad exports are retained for source
  compatibility; narrowing them requires a deprecation inventory and consumer
  migration rather than an aesthetic cleanup.

Compatibility should be removed only through an explicit deprecation and
migration decision. It must not be deleted merely to reduce line count.

## Constitutional Public Façade Inventory

`src/constitutional/mod.rs` keeps its implementation modules private and
re-exports their public items through one flat façade. The façade currently
contains thirteen module-wide re-exports and at least 187 top-level public
types, constants, errors, and functions, including the stable identifier types
declared by the ID macro.

| Source module | Public responsibility | Review status |
| --- | --- | --- |
| `adapters` | Typed kernel-pass and external-evidence boundaries | Retain; production boundary |
| `application` | Presentation-neutral request/response ownership | Preferred client entry point |
| `bond` | Canonical Bond events, aggregate state, phases, and errors | Retain; constitutional core |
| `houses` | House decisions, authority snapshots, and fail-closed reserved procedures | Retain; constitutional core |
| `ids` | Stable caller-supplied constitutional identities | Retain; constitutional core |
| `model` | Signed quantities, terms, evidence references, Waves, and Current edges | Retain; constitutional core |
| `persistence` | Versioned Bond archive, migration, and replay digests | Retain; stable persistence boundary |
| `regional` | Ratified regional Synthesis reducer and typed assignments | Retain; constitutional core |
| `regional_persistence` | Versioned regional archive and migration | Retain; stable persistence boundary |
| `runtime` | Append-only Bond orchestration and replay | Retain; expert production API |
| `scenarios` | Reusable capability fixtures | Public support surface; candidate for a named support namespace |
| `trace` | Read-only explanations derived from reducer outcomes | Public observability surface |
| `tui` | Presentation-neutral commands, events, and stable wire encoding | Preferred terminal-client contract |

No dangerous public mutation constructor was found. `BondAggregate::apply` is
crate-private, aggregate fields are private, runtime indexes are private,
regional state is private, and trace/TUI values cannot decide law. The public
runtime methods accept commands or events but still pass them through the
canonical reducers.

All current constitutional examples and integration tests import the flat
façade with `constitutional::*`. Removing or narrowing a wildcard re-export
would therefore be a source-compatibility change even before unknown external
consumers are considered.

If a future release chooses a narrower façade, the lawful migration order is:

1. expose named public module paths without removing the flat exports;
2. migrate repository consumers to explicit imports;
3. add compile-time compatibility witnesses for retained paths;
4. deprecate redundant flat exports for a stated compatibility window;
5. remove them only in an announced breaking release.

No public export is deprecated by this review. In particular,
`hollow_grove::Bond` remains the compatibility alias for `KernelBond`, while
new constitutional code should name `constitutional::BondAggregate`.

## Consolidation Order

Safe reduction should proceed in this order:

1. ignore generated build output in every nested crate;
2. replace duplicate orchestration with calls to the application service;
3. identify source-versus-generated document pairs and record one authority;
4. remove exact generated duplicates only when no runtime path requires them;
5. narrow public exports only with compatibility tests;
6. refactor reducers last and only when replay, archive, and state-machine
   equality remain proven.

Every consolidation must pass:

```text
cargo fmt --all -- --check
cargo test --all --no-fail-fast
cargo run --example constitutional_v2_service -- audit
```
