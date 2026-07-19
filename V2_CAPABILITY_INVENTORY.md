# Hollow Grove V2 Capability Inventory

Status: Expansion-phase audit and implementation inventory
Audit date: 2026-07-18
Repository root: `hollow-grove/`

## 1. Audit Method

This inventory distinguishes constitutional law, executable production behavior, fixtures, projections, and presentation. A capability is classified as:

- **Complete** — executable through a public production path, persisted or explicitly ephemeral as constitutionally appropriate, replayed deterministically, and covered by a positive and relevant negative test.
- **Partial** — executable core behavior exists, but one or more required authority, evidence, lifecycle, persistence, trace, or failure surfaces are incomplete.
- **Scaffolded** — typed structure or legality validation exists without the complete constitutional execution path.
- **Intentionally unratified** — the constitution expressly reserves the procedure and the implementation rejects it.
- **Absent** — no production type currently represents the capability.

The recursion selector `hollow_grove::Bond` is not counted as the constitutional Bond. It is the compatibility alias for `hollow_grove::KernelBond` in [`src/hollow_grove.rs`](src/hollow_grove.rs).

## 2. Executive Capability Table

| Capability | Canonical type | Canonical module | Public construction or submission path | Authority | Evidence | Legal predecessor → successor | Principal typed failures | Persistence, replay, migration | Existing tests | Missing coverage at audit | Status |
|---|---|---|---|---|---|---|---|---|---|---|---|
| Bond creation | `BondFormation`, `BondEvent::Formed`, `BondAggregate` | [`src/constitutional/bond.rs`](src/constitutional/bond.rs) | `ConstitutionalRuntime::append` | Stonebend `HouseFunction::Name`; explicit `InstitutionalJurisdictionSnapshot` | Formation, naming, jurisdiction, starting-Current evidence | no Bond → `BondPhase::Formed` | `FormationRequired`, `ParticipantsRequired`, `JurisdictionMismatch`, invalid term, missing Wave | Archive event tag `Formed`; reducer replay; schema migration dispatch | `full_proof_lifecycle_requires_all_four_house_functions`, `no_stage_can_be_skipped` | public draft/builder hardening; additional malformed formations | Complete |
| Naming | `HouseDecision`, `HouseFunction::Name` | [`src/constitutional/houses.rs`](src/constitutional/houses.rs) | `HouseDecision::from_catalog`; consumed by formation | Stonebend; `ConstitutionalIdentity` | non-empty House-decision evidence | proposal data → formed Bond | `WrongHouse`, `WrongFunction`, `MissingAuthority`, `DecisionNotAccepted` | embedded in `BondFormation`; replayed byte-for-byte | institution-derived authority test | exhaustive wrong-House matrix | Complete |
| Proof | `BondValidation`, `HouseFunction::Prove` | `bond.rs`, `houses.rs` | `BondEvent::Validated` | Sandmanor; `WitnessedImprovement` | validation and proof evidence | Formed → Validated | wrong phase/House/function, rejected decision, future authority | archive `Validated`; replayed | full lifecycle; challenge proof test | denied-proof scenario reporting | Complete |
| Recognition | `BondEvent::FlyntRecognized`, `HouseFunction::Recognize` | `bond.rs`, `houses.rs` | append after Tombstone validation | Flynt; `InstitutionalRecognition` | recognition evidence | TombstoneValidated → recognition recorded in same phase → Toke | `FlyntRecognitionRequired`, wrong House/function/authority | archive tag `FlyntRecognized`; replayed | premature Toke test; four-House catalog test | complete wrong-House matrix | Complete |
| Clearance | `CondensationDecision`, `HouseFunction::Clear` | `bond.rs`, `houses.rs` | `BondEvent::CondensationDecided` | Glaüshouse; `PublicClearance` | eligibility and clearance evidence | ExcessCalculated → EligibilityDecided | zero excess, missing/rejected/inconclusive clearance mismatch | archive tag `CondensationDecided`; replayed | full lifecycle | explicit rejected/inconclusive scenarios | Complete |
| Resolution | `BondResolution`, `HouseFunction::Resolve` | `bond.rs`, `houses.rs` | `BondEvent::Resolved` | Glaüshouse; `FinalJudgmentAnswerability` | resolution evidence | Recorded or complete no-proof branch → Resolved | wrong phase, wrong authority, invalid successor cardinality, self-successor | archive `Resolved`; replayed | full lifecycle renewal | split/merge/transfer examples | Complete |
| Positive/Negative Current | `Sign`, `SignedQuantity`, `SignedTotals`, `CurrentTransaction` | `model.rs`, `bond.rs` | exact `SignedQuantity::new`; `BondEvent::CurrentMoved` | active validated Bond plus causal Wave | transaction evidence | Active → Active with new immutable movement history | zero magnitude, unit mismatch, insufficient Current, invalid edge/operation | exact `u128` archive encoding; replay-derived totals | cross-polarity tests, finite freeze | all operation-shape examples | Complete |
| Positive/Negative Aura | `Sign`, `AuraObservation`, `SignedTotals` | `model.rs`, `bond.rs` | `BondEvent::AuraObserved` | active Bond; explicit participant observer | subject and observation evidence | Active → Active with new observation history | unknown observer, missing evidence, unit mismatch, post-term observation | exact archive encoding; replay-derived totals | two cross-polarity tests | all four persisted in one table-driven test | Complete |
| Four polarity states | `ConstitutionalPolarity` | `bond.rs` | `ConstitutionalRuntime::evaluate` derives, reducer verifies | prior lawful validation; no new House decision | evaluation evidence | accumulated and observed Active history → evaluated Active history | `UnevaluableSignedDomain`, `EvaluationMismatch` | `Evaluated` event; archive enum tag; replay equality | Positive/Negative and Negative/Positive tests | Positive/Positive and Negative/Negative scenario fixtures | Partial |
| Cross-polarity Bonds | `PositiveCurrentNegativeAura`, `NegativeCurrentPositiveAura` | `bond.rs` | same evaluation path | same as evaluation | same as evaluation | Active history → typed cross-polarity result | same as evaluation | persisted and replayed | both cross-polarity tests | demonstration catalog and trace | Complete |
| Evidence references | `EvidenceRef(ExternalRef)` | `model.rs`; neutral type in `hollow-grove-kernel` | `EvidenceRef::new`; adapter functions | domain-owned source | stable namespace and non-empty one-line key | attached to commands and records; never independently changes phase | invalid namespace/key; reducer-specific missing evidence | encoded losslessly in every applicable event | archive round-trip, adapter tests | generic Bond evidence is opaque rather than subject-bound | Partial |
| Authority snapshot | `AuthoritySnapshot`, `HouseDecisionDraft` | `houses.rs` | `AuthoritySnapshot::from_catalog`, `HouseDecision::from_catalog` | active `OfficeHolder`; exact office capability | decision evidence | institutional fact → historical snapshot | missing office/holder/House/capability, future snapshot | embedded in event archive and replay | four-House catalog test | public struct literals remain a bypass-prone compatibility surface | Partial |
| Institutional membership | `InstitutionalMembership`, `InstitutionalWorldState` | [`src/institution_affiliation.rs`](src/institution_affiliation.rs) | existing institutional commands and world fixtures | institution-specific | institutional events and claims | affiliation lifecycle defined outside Bond | membership validation errors | existing world persistence and validation | institutional boundary and affiliation unit tests | direct constitutional standing adapter beyond office authority | Complete in institution domain; partial Bond integration |
| Default | `BondDefault`, `BondDefaultResolution`, `DefaultOutcome` | `bond.rs` | `DefaultDeclared`, `DefaultResolved` events | participant and declared formation obligation; no separate House authority | non-empty declaration/resolution evidence | Active ↔ Active; unresolved default blocks maturity | unknown participant/obligation/default; duplicate/pending default | event tags 20/21; replay; remaining obligation preserved in Tombstone | confirmed-default Tombstone test | cured-default and invalid-standing demonstrations | Partial |
| Challenge | `BondChallenge`, `BondChallengeResolution`, `ChallengeOutcome` | `bond.rs` | challenge events | participant standing; Sandmanor proof resolves | challenged evidence plus filing/resolution evidence | Active ↔ Active; pending challenge blocks maturity | unknown participant/challenge, missing evidence, wrong proof authority | event tags 18/19; replay | pending challenge test | denied challenge and evidence-subject binding | Partial |
| Appeal | `appeal_challenge`, `ReservedHouseProcedure::HouseAppealCourt` | `houses.rs` | explicit call returns error | none ratified | none accepted | no transition | `ReservedProcedure(HouseAppealCourt)` | no event is committed | reserved appeal test | none until ratification | Intentionally unratified |
| Maturity | `BondMaturity`, `MaturityTrigger` | `bond.rs` | `BondEvent::Matured` | completed finite term or explicit perpetual termination | maturity evidence | Active with complete circulation/accumulation/observation/evaluation and no blockers → Mature | premature term, wrong trigger, missing stages, pending default/challenge | archive `Matured`; replay | no-stage-skip, term freeze, challenge/default tests | explicit perpetual scenario | Complete |
| Tombstone | `Tombstone`, `TombstoneValidation` | `bond.rs` | formation and validation events | eligible Glaüshouse-cleared excess; independent validator | complete Tombstone evidence, validation basis, exact pre-validation digest | EligibilityDecided → TombstoneFormed → TombstoneValidated | mismatch, ineligible, participant validator, replay digest mismatch | global Tombstone index; canonical archive; replay | full lifecycle, digest failure, default visibility | resurrection-specific scenario | Complete |
| Toke | `Toke`, `BondEvent::TokeRecorded` | `bond.rs` | append after Flynt recognition | Flynt recognition must already be committed | Toke evidence | TombstoneValidated+recognized → Recorded | missing recognition/Tombstone, ID or index mismatch | global Toke→Tombstone index; archive; replay | full lifecycle premature Toke test | lookup and distinction report | Complete |
| Renewal | `ResolutionDisposition::Renew` | `bond.rs` | resolved source reserves exactly one successor ID; successor formation cites parent | Glaüshouse resolution; new Stonebend naming for child | source resolution and child inheritance evidence | Recorded source → Resolved; absent child → Formed child | wrong cardinality, self-successor, unresolved/unreserved parent | both histories archived and replayed | full lifecycle renewal | renewal after terminal source with unrelated ID | Complete |
| Inheritance | `BondFormation::parent_bonds`, `inheritance_evidence` | `bond.rs`, `runtime.rs` | successor formation after parent resolution | parent resolution and child formation authorities | required non-empty inheritance evidence | resolved parent reservation → formed child | unknown/unresolved parent, unreserved successor, evidence mismatch | formation payload; replay; integrity audit | full lifecycle successor | authority/evidence-drop mutation tests | Complete |
| Succession | `ResolutionDisposition::{Renew,Merge,Branch,Split,Transfer}` plus Reserved House procedures | `bond.rs`, `houses.rs` | Bond successor graph is supported; House-office succession is rejected where Reserved | Glaüshouse for Bond resolution; House-specific succession unavailable | resolution/inheritance evidence | resolved Bond → successor formation(s) | successor cardinality/integrity errors; `ReservedProcedure` | archived graph; replay integrity | renewal and Flynt succession suites | common split/merge/branch/transfer scenarios | Partial by disposition; House succession intentionally unratified |
| Persistence | `encode/decode/write/read_constitutional_archive` | [`src/constitutional/persistence.rs`](src/constitutional/persistence.rs) | public codec and file functions | none; decoder validates contained authority through reducer | all event evidence retained | runtime ↔ canonical bytes | invalid magic/version/tag/ID/UTF-8, truncation, trailing bytes, reducer failure | schema version 1 | archive round trips | supported prior schema fixture | Complete current schema; prior migration scaffolded |
| Replay | `ConstitutionalRuntime::replay`, `BondAggregate::replay` | `runtime.rs`, `bond.rs` | public replay from Waves and events | historical snapshots replayed, not re-queried | exact event evidence | empty runtime + history → equivalent state/indexes | sequence, causality, Wave, digest, state errors | same reducer as live append | multiple replay-equality tests | high-volume and tamper matrix | Complete |
| Migration | `migrate_constitutional_archive` | `persistence.rs` | decode supported version then canonical encode | preserved from history | preserved from history | supported archive → current canonical archive | unsupported version fails closed | current schema is idempotently canonical | migration idempotence test | no supported prior version at audit | Scaffolded |
| Idempotency | duplicate Wave and event handling | `runtime.rs` | resubmit same ID and identical payload | unchanged | unchanged | identical retry → same result/no new event | same ID with different content → conflict | stable archive and digest | command retry test | Synthesis-specific retry | Complete for Bond runtime |
| Digests | `constitutional_replay_digest`, `constitutional_bond_replay_digest` | `persistence.rs` | public functions; Tombstone validation compares Bond prefix | independent validator | digest covers direct Waves and exact event prefix | pre-validation history → deterministic checksum | mismatch rejects before mutation | versioned FNV-1a integrity checksum | full lifecycle mismatch test | scaling benchmark and tamper catalog | Complete |
| Waves | `WaveRecord`, `record_wave` | `model.rs`, `runtime.rs` | caller-controlled record | domain cause, not House authority by itself | origin evidence | absent Wave → recorded Wave; no Bond mutation | conflict, missing/future Wave reference | archived separately; cited by events; replayed | kernel-Wave test | multiple-Wave and tamper examples | Complete |
| Kernel adapter | `record_kernel_wave`, `kernel_pass_evidence` | [`src/constitutional/adapters.rs`](src/constitutional/adapters.rs) | completed `KernelPass` → `WaveRecord` | none invented | versioned witness checksum reference | completed pass → Wave only | invalid evidence ref, Wave conflict | Wave persists through archive | explicit non-movement test | CLI demonstration | Complete |
| House boundaries | `HouseFunction::{Name,Prove,Clear,Recognize,Resolve}` | `houses.rs` | decision validation at each reducer gate | fixed House and capability mapping | decision evidence | stage-specific | wrong House/function/capability/holder | authority snapshot embedded and replayed | four-House integration test | every wrong-House permutation | Complete core; demonstration missing |
| Illegal states | `BondStateError`, `ConstitutionalRuntimeError`, `HouseLawError` | constitutional modules | returned before commit | varies | varies | no state change | typed exhaustive enums | rejected attempts are not authoritative events; prior archive remains | stage and digest failures | systematic catalog | Partial coverage |
| Recovery | idempotent retry, replay, migration, explicit challenge/default resolution | runtime and persistence | replay canonical history or submit lawful follow-up | applicable authority | follow-up evidence | failure leaves prior state intact | no rollback/deletion API | rebuild from archive | retry/replay tests | documented operational recovery scenarios | Partial |
| Failure closure | candidate aggregate clone before live commit | `ConstitutionalRuntime::append` | all commands | all gates | all gates | invalid attempt → unchanged runtime | typed errors | no rejected event enters canonical archive | illegal-state tests | broad state-before/after property table | Complete mechanism; partial matrix |
| Regional identity | `SandmanorForm`, `SandmanorLineage`, `SandmanorStage` | [`src/sandmanor_lineage.rs`](src/sandmanor_lineage.rs) | `SandmanorForm::from_frame`, typed methods | none; descriptive legality only | none | typed form identity | `NotSandmanorForm` | not in constitutional archive | lineage unit tests | constitutional stable Being identity | Scaffolded at audit |
| Regional occupation | Aura Fields/Aura Beach sites in `InstitutionCatalog`; private presentation definitions | [`src/world/house_institutions.rs`](src/world/house_institutions.rs), `hueman_support.rs` | catalog lookup / presentation generation | site control only | catalog facts | no transformation state machine | none specific | institution persistence only | institution fixture tests | typed regional standing and occupation | Scaffolded at audit |
| Regional guardianship | no canonical production type at audit | — | none | none | none | none | none | none | none | all required behavior | Absent at audit |
| Synthesis prerequisites | Recipe legality and adjacent lineage legality are separate | `synthesis_recipe.rs`, `synthesis_execution.rs`, `sandmanor_lineage.rs` | compile/execute Recipe; `validate_sandmanor_transition` | bounded execution has no regional Glaüshouse authorization | execution evidence can be adapted | adjacent forms only | Recipe and lineage errors | Recipe result not in constitutional archive | Recipe and lineage tests | combined region/authority/evidence gate | Partial at audit |
| Synthesis authority | Glaüshouse exclusive law exists; Bond resolution authority implemented | `GLAUSHOUSE_CONSTITUTION_V1_DRAFT.md`, `houses.rs` | `HouseFunction::Resolve` in Bond | Glaüshouse `FinalJudgmentAnswerability` | decision evidence | proof-complete Bond → resolved | wrong authority | Bond archive only | Bond lifecycle | bounded regional transformation authorization | Partial at audit |
| Synthesis evidence | `synthesis_execution_evidence` adapter | `adapters.rs` | opaque reference from `SynthesisExecution` | adapter invents none | execution artifact reference | evidence only | invalid ref | persists if attached to event | compile coverage | subject-bound regional evidence | Scaffolded at audit |
| Synthesis lineage | `SandmanorForm::previous_form`, `next_legal_form`, `lineage` | `sandmanor_lineage.rs` | read-only typed methods | none | none | Gnome→Minotaur; Elf→Centaur and mastery steps | cross-lineage/stage skip | not persisted constitutionally | lineage tests | stable predecessor identity and replay | Scaffolded at audit |
| Gnome→Minotaur | `validate_sandmanor_transition(FrameId::Gnome, FrameId::Minotaur)` | `sandmanor_lineage.rs` | legality function | no execution authority | none | adjacent Minorian forms | cross-lineage/stage skip | none | lineage unit/integration tests | region, authority, evidence, result record | Scaffolded at audit |
| Elf→Centaur | `validate_sandmanor_transition(FrameId::Elf, FrameId::Centaur)` | `sandmanor_lineage.rs` | legality function | no execution authority | none | adjacent Minoan forms | cross-lineage/stage skip | none | lineage unit/integration tests | region, authority, evidence, result record | Scaffolded at audit |
| Aura Fields stewardship | Minotaur traits include `Bull`, `Fieldwork`, `Endurance`; Aura Fields site exists | lineage and world institution modules | read-only trait/site lookup | none combined | none | no executable grant | none specific | none | trait tests only | typed duties and replay | Scaffolded at audit |
| Aura Beach patrol | Centaur traits include `Horse`, `CoastalRange`, `Endurance`; Aura Beach site exists | lineage and world institution modules | read-only trait/site lookup | none combined | none | no executable grant | none specific | none | trait tests only | typed occupation and replay | Scaffolded at audit |
| Aura Sea guardianship | descriptive regional text only at audit | world context / generated presentation | none | none | none | no executable grant | none | none | none | typed guardianship target/duties | Absent at audit |
| Invalid cross-regional Synthesis | `SandmanorTransitionError::CrossLineage` | `sandmanor_lineage.rs` | `validate_sandmanor_transition` | none | none | invalid request → error | `CrossLineage` | not archived | lineage tests | region mismatch, wrong function, authority/evidence failures | Partial at audit |

## 3. State and Transition Inventory

### 3.1 Bond phases

`BondPhase` in `src/constitutional/bond.rs` contains:

```text
Formed
Validated
Active
Mature
ExcessCalculated
EligibilityDecided
TombstoneFormed
TombstoneValidated
Recorded
Resolved
```

Living-history events for Current, accumulation, Aura, evaluation, challenge, and default operate only within `Active`. They do not create alternative phase machines. Pending challenges/defaults and an invalidated evaluation block maturity.

### 3.2 Proof and no-proof branches

The non-zero eligible path is:

```text
Mature
→ ExcessCalculated
→ CondensationDecided(Eligible)
→ TombstoneFormed
→ TombstoneValidated
→ FlyntRecognized
→ TokeRecorded
→ Resolved
```

The ineligible path explicitly records Tombstone, Tombstone-validation, and Toke omissions before resolution. The stages are not silently skipped.

### 3.3 Parent and successor graph

Resolution reserves successor identities. A child formation must name every parent and carry inheritance evidence. `ConstitutionalRuntime::verify_successor_integrity` detects missing reserved successors and missing reciprocal parent references. Because parent Bonds must already be resolved before child formation, forward construction plus self-parent rejection prevents cycles.

## 4. Authority Inventory

| Function | House | Existing office | Required capability | Governing runtime use |
|---|---|---|---|---|
| Name | Stonebend | `office.stonebend.hypergiant` | `ConstitutionalIdentity` | Bond Formation |
| Prove | Sandmanor | `office.sandmanor.sandman` | `WitnessedImprovement` | Bond Validation and challenge resolution |
| Clear | Glaüshouse | `office.glaushouse.prima-donna` | `PublicClearance` | Condensation eligibility |
| Recognize | Flynt | `office.flynt.tross` | `InstitutionalRecognition` | pre-Toke recognition |
| Resolve | Glaüshouse | `office.glaushouse.prima-donna` | `FinalJudgmentAnswerability` | final Bond resolution |

`HouseDecision::from_catalog` validates an active office holder and copies an immutable historical snapshot. The canonical world does not invent non-Flynt office succession. `world::institutional_access_fixture` supplies anonymous active holders for demonstrations and tests only.

## 5. Persistence Inventory

The constitutional archive is a dependency-free canonical binary format defined in `src/constitutional/persistence.rs`.

- Magic: `HGCONST\0`.
- Current schema: `CONSTITUTIONAL_ARCHIVE_VERSION = 1` at audit.
- Integer representation: fixed-width little-endian `u16`, `u64`, and `u128`.
- Strings/lists: `u64` length prefixes.
- Enums: explicit stable tags.
- Authority/evidence: embedded in full, never re-queried during replay.
- Decode behavior: all state and indexes are rebuilt through `ConstitutionalRuntime::replay`.
- Unknown versions: fail closed.
- Migration at audit: current-version canonical rewrite only; no prior version accepted.

## 6. Test Inventory

The primary conformance suite is [`tests/constitutional_runtime.rs`](tests/constitutional_runtime.rs). At audit it covers:

- both cross-polarity states;
- full proof lifecycle and all four House functions;
- renewal and successor integrity;
- Reserved House procedures;
- canonical archive round-trip and unsupported version rejection;
- event/Wave idempotency and identity conflict;
- institution-derived authority;
- kernel-pass-to-Wave adaptation without Current movement;
- challenge blocking and Sandmanor resolution;
- finite-term Current/Aura freezing;
- confirmed default visibility in Tombstone;
- skipped-stage rejection;
- Bond-specific pre-validation digest mismatch.

Repository-wide boundary suites additionally protect kernel purity, institutional isolation, lineage legality, composition/provenance separation, Flynt succession, Recipe execution, decision replay, and persistence patterns.

## 7. Audit Conclusions and Expansion Targets

The Bond constitutional runtime is a functioning event-sourced authority. The regional transformation requirement does not justify changing that reducer or turning presentation geography into law.

The smallest coherent expansion is a sibling event-sourced regional Synthesis aggregate inside `src/constitutional/` that:

1. reuses `SandmanorForm`, `SandmanorLineage`, and `validate_sandmanor_transition`;
2. binds stable Being identities to explicit regional standing;
3. requires Sandmanor proof of the configuration and Glaüshouse Synthesis resolution authority;
4. requires evidence explicitly bound to the predecessor Being;
5. derives, rather than narratively invents, the only two ratified regional outcomes;
6. preserves predecessor identity and lineage;
7. creates typed Aura Fields stewardship for Minotaur;
8. creates typed Aura Beach occupation and Aura Sea guardianship for Centaur;
9. persists an append-only event history and replays through the same reducer;
10. rejects cross-lineage, wrong-region, wrong-function, authority-free, evidence-free, duplicate, terminal, and lineage-erasing requests without mutation.

Presentation, traces, scenarios, and the future TUI must consume those committed results. They must not assign form, region, stewardship, or guardianship themselves.

## 8. Expansion-Phase Implementation Reconciliation

The preceding tables preserve the repository state at the opening audit. The
following table is the post-implementation authority for capabilities changed
during this phase. Entries not listed here retain the status recorded above.

| Capability | Final canonical types and module | Public construction path | Authority and evidence | Legal transition | Failure and no-mutation behavior | Persistence, replay, and migration | Demonstration and verification | Final status |
|---|---|---|---|---|---|---|---|---|
| Four polarity states | `Sign`, `SignedQuantity`, `CurrentAuraEvaluation`, `ConstitutionalPolarity` in `src/constitutional/model.rs` and `bond.rs` | `run_polarity_scenario`; production submission remains `ConstitutionalRuntime::append` and `evaluate` | Formation/validation authority plus transaction, observation, and evaluation evidence | Active accumulated history → evaluated Active history | mismatched derived evaluation is rejected by `BondAggregate`; no append occurs | all four tags encode in `persistence.rs`; live replay uses the same reducer | `examples/constitutional_v2.rs`; `all_four_polarities_round_trip_through_persistence_and_replay` | **Complete** |
| Regional identity | `RegionalBeingId`, `RegionalBeingRecord`, `RegionalBeingStatus` in `src/constitutional/ids.rs` and `regional.rs`; existing `SandmanorForm` is reused | `RegionalSynthesisRuntime::register_being` with explicit `RegionalBeingRegistration` | Sandmanor-controlled regional jurisdiction and evidence bound to the Being | absent identity → active Gnome or Elf origin | conflicting/reused identity, evolved origin, invalid site, future jurisdiction, or missing/mismatched evidence is rejected | registration event in `HGREGV2` V1; replay rebuilds Being indexes | regional tests for unlined evolved forms, identity stability, and location non-inference | **Complete for ratified origins** |
| Regional occupation | `RegionalStanding`, `RegionalJurisdictionSnapshot`, `RegionalStandingKind`, `ConstitutionalRegion` in `regional.rs` | `RegionalJurisdictionSnapshot::from_catalog`; `scenario_regional_registration` is reusable fixture support | actual `site.sandmanor.aura-fields` or `site.sandmanor.aura-beach`, controlled by `institution.sandmanor.sandmen`; nonempty evidence | registration establishes standing; Synthesis preserves the required standing | Aura Sea cannot be primary standing; Visitor or wrong-region standing cannot trigger Synthesis | standing and catalog snapshot encode in every registration; replay validates it | `show-region`, wrong-region tests, location-only role rejection | **Complete** |
| Synthesis prerequisites | `RegionalSynthesisPrerequisites` with standing, lineage, readiness, constitutional-rule, and supporting evidence | field of `RegionalSynthesisCommand` | each item is a `SubjectEvidence` whose subject must equal the predecessor | complete prerequisites permit further validation; they do not themselves transform | missing or mismatched subject evidence returns a typed error and leaves history unchanged | every prerequisite is encoded; decode re-runs validation | lawful scenarios plus missing/mismatched evidence rejection | **Complete** |
| Synthesis authority | `RegionalSynthesisAuthority` | field of `RegionalSynthesisCommand`; fixtures use `HouseDecision::from_catalog` | accepted Sandmanor `Prove` from `institution.sandmanor.sandmen` and accepted Glaüshouse `Resolve` from `institution.glaushouse.medical-civilization` | validated active predecessor → authority-cleared Synthesis candidate | wrong House, function, institution, outcome, time, or reused decision is rejected before mutation | full historical `HouseDecision` snapshots encode and replay | `show-authority`; rejected-authority and forged-institution tests | **Complete for the two ratified regional rules** |
| Synthesis evidence | `SubjectEvidence` and evidence-bearing command/standing/jurisdiction types | explicit construction; never derived from presentation | stable `EvidenceRef`, nonempty and bound to predecessor identity | accepted evidence participates in a committed Synthesis record | no evidence, another Being's evidence, or jurisdiction without evidence is rejected | lossless binary codec; canonical decode | `show-evidence`; negative evidence tests | **Complete** |
| Synthesis lineage | `RegionalLineageEntry`, `RegionalBeingRecord::predecessor`, `lineage_history` | reducer output; inspection by `RegionalSynthesisRuntime::lineage` | lawful adjacent `SandmanorForm` transition plus the command authority/evidence | Gnome→Minotaur or Elf→Centaur | cross-lineage, unratified mastery, identity reuse, evolved origin registration, terminal source, or replay-altered predecessor is rejected | lineage is deterministically rebuilt from source events and checked against accepted event payload during replay | `show-lineage`; altered-history, origin-registration, and end-to-end tests | **Complete** |
| Gnome→Minotaur | `RegionalSynthesisRule::GnomeToMinotaurAuraFields` | `RegionalSynthesisRuntime::synthesize`; fixture `run_gnome_minotaur_scenario` | Sandmanor proof, Glaüshouse resolution, Aura Fields standing, five prerequisite evidence records, result evidence | active Gnome with established Aura Fields standing → active Minotaur; predecessor becomes `SynthesizedInto` | Gnome→Centaur, wrong region/function, missing authority/evidence, duplicate non-idempotent request, terminal source, or reused identity fails closed | V1 archive, V0→V1 migration fixture, live replay, canonical re-encoding, stable digest | `run gnome-minotaur`, persistence/replay/lineage/stewardship commands; regional suite | **Complete** |
| Aura Fields stewardship | `AuraFieldsStewardship`, `AuraFieldsDuty`, `RegionalAssignment::Minotaur` | reducer-derived only; read by `stewardship` | inherited from accepted Synthesis authority and evidence | granted atomically with valid Minotaur result | no location-only grant; Centaur lookup returns none; Minotaur has no sea guardianship | derived during archive decode and replay; exact equality tested | seven typed duties demonstrated and asserted | **Complete** |
| Elf→Centaur | `RegionalSynthesisRule::ElfToCentaurAuraBeach` | `RegionalSynthesisRuntime::synthesize`; fixture `run_elf_centaur_scenario` | Sandmanor proof, Glaüshouse resolution, Aura Beach standing, five prerequisite evidence records, result evidence | active Elf with established Aura Beach standing → active Centaur; predecessor becomes `SynthesizedInto` | Elf→Minotaur and all corresponding regional/authority/evidence/identity failures fail closed | same V1/V0 migration, replay, canonical persistence, and digest support | `run elf-centaur`; regional suite | **Complete** |
| Aura Beach patrol | `AuraBeachOccupation`, `AuraBeachDuty`, `RegionalAssignment::Centaur` | reducer-derived only; read by `beach_occupation` | same accepted Centaur Synthesis authority/evidence | granted atomically with Centaur result | Gnome/Minotaur cannot obtain it through these rules | derived on decode/replay | eight typed duties asserted and traced | **Complete** |
| Aura Sea guardianship | `AuraSeaGuardianship`, `AuraSeaGuardianshipDuty` | reducer-derived only; read by `guardianship` | same accepted Centaur Synthesis; Aura Sea is the required guardianship target, not a primary Synthesis site | granted atomically with Centaur beach occupation | Minotaur lookup is none; Aura Sea primary standing is rejected; Centaur cannot be valid without the canonical assignment | V1 archive and replay reconstruct exact holder, region, duties, authority, and evidence | `show-guardianship elf-centaur`; four duty assertions | **Complete** |
| Invalid regional Synthesis | `RegionalSynthesisError` and stable `code()` values | production reducer return value; fixtures in `run_rejected_regional_scenario` and read-only claim verification | missing/incorrect authority or evidence never defaults to inferred law | no legal successor | every rejected command or assignment claim leaves event count and Being indexes unchanged | rejection is audit trace data, not canonical history; accepted prior history remains replayable | nine CLI regional failures and 17-test regional conformance suite | **Complete for enumerated current law** |
| Regional persistence | `encode_regional_archive`, `decode_regional_archive`, `write_regional_archive`, `read_regional_archive` in `regional_persistence.rs` | public dependency-free binary codec | persists the exact authority/evidence inputs; decoder trusts no stored assignment snapshot | runtime ↔ canonical V1 bytes | invalid magic/version/tag/identifier/reference/length/sequence/trailing bytes or reducer violation fails closed | magic `HGREGV2\0`; explicit enum tags; V1 current | canonical byte equality and unsupported-version tests | **Complete** |
| Regional migration | `encode_legacy_regional_archive_v0`, `migrate_regional_archive` | public fixture encoder and migration entry point | authority/evidence retained | supported V0 archive → canonical V1 archive | every other version is rejected | decode through production reducer, then V1 encode | migration equality test and `migrate` command | **Complete for V0; future versions unratified** |
| Regional trace | `ConstitutionalTrace`, `TransitionTrace`, `TraceDisposition` in `trace.rs` | `trace_regional_scenario`, `trace_rejected_regional_scenario` | reports committed decisions/evidence or the reducer's returned error code | no domain transition; read-only projection | has no mutation or decision method | reports live/replay and canonical persistence comparisons | trace/TUI non-mutation test | **Complete** |
| TUI readiness | `TuiCommand`, `TuiEvent`, `TuiEventKind` in `tui.rs` | trace-to-event projection; stable `encode_line`/`decode_line` | carries authority/evidence results but grants none | presentation event stream only | malformed wire records fail with `TuiWireError` | deterministic tab-separated escaped record | wire round-trip test; `V2_TUI_READINESS_CONTRACT.md` | **Complete contract; TUI deliberately not built** |
| Application service | `ConstitutionalApplicationService`, `TuiRequest`, `TuiResponse`, `ApplicationResponseStatus` in `application.rs` | `execute` selects production-built scenario state and owns its archive; `execute_streaming` delivers the same ordered response to a sink | reuses reducer outcomes; cannot grant authority/evidence/assignments | selected runtime → replay/persist/migrate/inspect; exact retry returns stored response | request-ID conflict, unavailable inspection, unsafe cancellation, unknown/unselected scenario | archive bytes private; metadata exposes only kind/length/digest; replay and migration use canonical codecs | 10-test service suite plus `constitutional_v2_service` example | **Complete synchronous demonstration boundary; live gameplay and TUI deliberately absent** |

## 9. Final File and Type Index

- `src/constitutional/regional.rs`: regional domain model, two ratified rules,
  reducer, typed assignments, lineage, rejection codes, and read-only lookups.
- `src/constitutional/regional_persistence.rs`: V0/V1 codecs, canonical migration,
  file I/O, replay-on-load, and digest.
- `src/constitutional/scenarios.rs`: production-backed scenario inputs and
  catalog. Fixtures call public reducers and institution catalogs.
- `src/constitutional/trace.rs`: read-only accepted/rejected projections.
- `src/constitutional/application.rs`: TUI-facing request executor and private
  selected runtime/archive owner.
- `src/constitutional/tui.rs`: deterministic presentation-neutral wire events.
- `examples/constitutional_v2.rs`: executable capability demonstrator.
- `examples/constitutional_v2_service.rs`: executable witness of the
  application-owned request/event boundary.
- `examples/constitutional_v2_bench.rs`: dependency-free release benchmark
  harness.
- `tests/regional_synthesis.rs`: 17 regional conformance tests.
- `tests/constitutional_v2_demonstration.rs`: seven cross-capability and
  end-to-end tests.
- `tests/constitutional_application_service.rs`: ten application-boundary
  conformance tests.

## 10. Deliberately Unavailable After Expansion

The expansion does not ratify Gnome→Centaur, Elf→Minotaur, Minotaur→Hecaton,
Centaur→Pegasus, location-triggered automatic Synthesis, regional transfer,
regional appellate authority, House-office succession, a mutable assignment
API, or a TUI. `AuraFieldsDuty` and `AuraBeachDuty` are constitutional duty
assignments; they do not yet implement crop simulation, pathfinding, combat,
escort AI, horizon sensing, or Current-control mechanics. Those mechanics may
consume the assignments later but cannot redefine them.
