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
| Succession | `ResolutionDisposition::{Renew,Merge,Branch,Split,Transfer}` plus House-specific accession law | `bond.rs`, `houses.rs`, `world/glaushouse.rs` | Bond successor graph is supported; the frozen common layer still rejects generic office succession while Glaüshouse validates its generative rank ladder and ordered Prima Donna accession above it | Glaüshouse for Bond resolution; multiple Persephone rank records and the singular Prima Donna office remain House-specific | resolution/inheritance/advancement/candidacy/accession evidence | resolved Bond → successor formation(s); Nightingale → equal branch → Persephone candidacy; lawful apex vacancy → sealed Prima Donna accession | successor cardinality/integrity errors; missing branch proofs; unqualified candidacy; closed advancement; incomplete or skipped accession | archived Bond graph; House-specific stable-person advancement and accession records | renewal suites, Glaüshouse ladder/cardinality/selection suites, invalid-origin rejection | common split/merge/branch/transfer scenarios | Bond succession partial by disposition; Glaüshouse generative advancement and singular apex procedure ratified above common runtime |
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
| Regional occupation | Aura Field/Aura Beach sites in `InstitutionCatalog`; private presentation definitions | [`src/world/house_institutions.rs`](src/world/house_institutions.rs), `hueman_support.rs` | catalog lookup / presentation generation | site control only | catalog facts | no transformation state machine | none specific | institution persistence only | institution fixture tests | typed regional standing and occupation | Scaffolded at audit |
| Regional guardianship | no canonical production type at audit | — | none | none | none | none | none | none | none | all required behavior | Absent at audit |
| Synthesis prerequisites | Recipe legality and adjacent lineage legality are separate | `synthesis_recipe.rs`, `synthesis_execution.rs`, `sandmanor_lineage.rs` | compile/execute Recipe; `validate_sandmanor_transition` | bounded execution has no regional Glaüshouse authorization | execution evidence can be adapted | adjacent forms only | Recipe and lineage errors | Recipe result not in constitutional archive | Recipe and lineage tests | combined region/authority/evidence gate | Partial at audit |
| Synthesis authority | Glaüshouse clinical law and Bond resolution authority implemented in separate scopes | `GLAUSHOUSE_CONSTITUTION_V2.md`, `world/glaushouse.rs`, `houses.rs` | typed clinical Synthesis record plus `HouseFunction::Resolve` in Bond | explicit consent, active clearance, privilege, lawful provenance, recovery; common Glaüshouse resolution authority | clinical record and Bond decision evidence remain referenced, not collapsed | cleared procedure → stabilized actual outcome; proof-complete Bond → resolved | consent, clearance, privilege, provenance, recovery, identity, outcome, or common authority failure | House clinical registry plus common Bond archive | Glaüshouse architecture and Bond lifecycle suites | shared durable adapter between the two scopes | Complete House validation; common integration remains scoped |
| Synthesis evidence | `synthesis_execution_evidence` adapter | `adapters.rs` | opaque reference from `SynthesisExecution` | adapter invents none | execution artifact reference | evidence only | invalid ref | persists if attached to event | compile coverage | subject-bound regional evidence | Scaffolded at audit |
| Synthesis lineage | `SandmanorForm::previous_form`, `next_legal_form`, `lineage` | `sandmanor_lineage.rs` | read-only typed methods | none | none | Gnome→Minotaur; Elf→Centaur and mastery steps | cross-lineage/stage skip | not persisted constitutionally | lineage tests | stable predecessor identity and replay | Scaffolded at audit |
| Gnome→Minotaur | `validate_sandmanor_transition(FrameId::Gnome, FrameId::Minotaur)` | `sandmanor_lineage.rs` | legality function | no execution authority | none | adjacent Minorian forms | cross-lineage/stage skip | none | lineage unit/integration tests | region, authority, evidence, result record | Scaffolded at audit |
| Elf→Centaur | `validate_sandmanor_transition(FrameId::Elf, FrameId::Centaur)` | `sandmanor_lineage.rs` | legality function | no execution authority | none | adjacent Minoan forms | cross-lineage/stage skip | none | lineage unit/integration tests | region, authority, evidence, result record | Scaffolded at audit |
| Aura Field stewardship | Minotaur traits include `Bull`, `Fieldwork`, `Endurance`; Aura Field site exists | lineage and world institution modules | read-only trait/site lookup | none combined | none | no executable grant | none specific | none | trait tests only | typed duties and replay | Scaffolded at audit |
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

`HouseDecision::from_catalog` validates an active office holder and copies an immutable historical snapshot. This frozen common runtime does not select House-office succession. Stonebend's now-ratified Hypergiant selection order and Stonebend-specific Name, Title, accession, Seal, Hollowing, custody, succession, and Tombstone invariants live above it in `src/world/stonebend.rs`; they do not change the common reducer. Flynt remains independently authoritative through `flynt-constitution`. `world::institutional_access_fixture` supplies anonymous active holders for demonstrations and tests only.

### 4.1 Stonebend-specific constitutional inventory

| Capability | Canonical type / function | Enforced result |
|---|---|---|
| Principal authority placement | `PrincipalAuthority`, `PRINCIPAL_AUTHORITIES`, `validate_principal_authorities` | at most one active Hypergiant bearer; distributed Proliteriate network; High Freemason office and wider institution; Claim, Title, and Yield stay distinct |
| Hypergiant selection order | `HypergiantSelectionProcess`, `second_pass::HypergiantSuccession` | Claim, independent Freemason examination, Yield hearing, relinquishment, consequence descent, Proof of Persistence, Lazerhorn, eligibility, and Diamond investiture cannot be skipped or reordered |
| Name and Title | `NameRecord`, `TitleRecord`, `StonebendRegistry::validate` | every active Title has one valid active Name and lawful grant; transformation, recognition, clearance, custody, and legacy progression cannot manufacture Title |
| Office accession | `AccessionRecord`, `AccessionBasis` | every active office has matching holder, office, evidence, Seal, and ratified accession requirements; Diamond may be vacant and never has more than one active bearer |
| Seal | `SealRecord` | Freemason issuance binds the correct subject, scope, and decision |
| Hollowing | `HollowingRecord`, `HollowingConsent` | authority, purpose, scope, consent, evidence, operator, plans, continuity, disposition, and Seal are mandatory; emergency review cannot be omitted |
| Extracted Hollow | `ExtractedHollowRecord`, `CustodyRecord` | source provenance and chain of custody are mandatory |
| Rename and succession | `RenameRecord`, `SuccessionRecord` | prior Name/history and successor benefits/obligations are preserved |
| Tombstone | `TombstoneRecord` | ended authority remains durable history and cannot remain active |

### 4.2 Stonebend second-pass constitutional inventory

Normative source:
`STONEBEND_THREE_GATES_OFFICES_AND_TITLE_SCOPE_V1.md`.

| Capability | Runtime owner | Enforced boundary |
|---|---|---|
| Three-gate topology | `stonebend::second_pass::{StonebendGateFacing, StonebendGate, canonical_stonebend_gates}` | exactly Flynt, Central Junction, and Sandmanor facings; both directions; existing routes and Craft Corridor; Central Junction is not a House |
| Bounded Title scopes | `StonebendTitleCore`, `GateScopeRecognition`, `GateEvidenceTransfer` | one stable `TitleRecordId`; Formation, Circulation, and Deployment are independent; returned failure evidence cannot silently erase another scope |
| Diamond tenure | `DiamondState`, `DiamondTenure`, `OfficeTombstone` | Diamond persists through bearer change and vacancy; the Hypergiant is a temporary bearer rather than owner |
| Freemason Claim review | `ClaimRecord::validate_freemason_examination` | the existing High Freemason office bears Claim authority; sovereign self-certification is rejected |
| Proliteriate network | `ProliteriateNetwork`, `ProliteriateNode`, `NetworkMembership`, `NetworkMandate`, `RaisedWitness` | stable overlapping nodes and memberships; temporary bounded witness; recall, completion, invalidation, and replacement preserve the network |
| Challenge and removal | `ConstitutionalChallenge`, `ConstitutionalConcurrence`, `RemovalAuthorization` | one power opens review; exactly the other two distinct powers remove; duplicates fail; representation removal cannot abolish the people |
| Lazerhorn succession | `HypergiantSuccessionStage`, `HypergiantSuccession` | semantic stage order ignores insertion order; Lazerhorn is mandatory; recommendation, lineage, self-certification, and former tenure never bypass the full path |
| Gate accountability | `GateCrossingRecord`, `DelegatedPowerTrace` | routine work is delegable while every crossing remains traceable to Claim, Title, and Yield authority |

### 4.3 Stonebend third-pass lifecycle and continuity inventory

Normative source:
`STONEBEND_TITLE_LIFECYCLE_AND_CONSTITUTIONAL_CONTINUITY_V1.md`.

| Capability | Runtime owner | Enforced boundary |
|---|---|---|
| Title lifecycle policy | `stonebend::third_pass::{TitleLifecyclePolicy, StonebendTitleLifecycle, TitleLifecycleStage}` | semantic stage order; optional stages remain policy-defined; one existing `StonebendTitleCore` persists |
| Recognition and activation | `TitleRecognition`, `TitleActivation` | a Claim is not a Title; recognized inactive Titles are lawful; exercise requires policy conditions |
| Maintenance and renewal | `TitleMaintenanceRecord`, `TitleRenewalRecord`, `TitleTerm` | continuing support differs from formal renewal; renewed terms do not duplicate the Title; rejected renewal preserves history |
| Targeted intervention | `TitleIntervention`, `TitleInterventionTarget`, `SupervisionTerms` | limitation, supervision, suspension, remediation, and removal identify scope, activation, license, tenure, mandate, or explicit core review |
| Proportionate failure | existing `second_pass::GateFailureKind` | honest failure, negligence, fraud, illegality, and hollowness remain distinct; illegality creates a typed future referral rather than a court implementation |
| Restoration and ending | `RestorationRecord`, `TitleTerminalDisposition`, `TitleTermTombstone` | stable identity and interruption evidence persist; every ended active term links a Tombstone |
| Diamond vacancy continuity | `DiamondContinuityMandate`, `EmergencyContinuityAction` | only enumerated existing duties; all actions terminate and require review; no Regent, appointment, investiture, sovereign law, or permanent scope expansion |
| High Freemason replacement | `FreemasonOfficeState`, `FreemasonSuccessionClaim`, `IndependentForgeReview` | prior records persist; independent non-self review; recommendation is evidence; exactly one active seal bearer |
| Proliteriate continuity | `ProliteriateContinuityPolicy` plus existing `ProliteriateNetwork` | node dissolution preserves identity and membership history; witness completion/recall preserves network; no permanent speaker or numeric threshold |
| Temporal review | `ClaimTitleYieldReview` | Claim, Title, and Yield evidence remain traceable; deterioration targets the affected constitutional dimension |

### 4.4 First Stonebend foundation inventory

| Capability | Runtime owner | Authority boundary | Determinism / identity | Verification | Status |
|---|---|---|---|---|---|
| Vertical law | `stonebend::foundation::{VerticalPole, VerticalLandmark, PhysicalManifestation}` | Mt. Aura represents Aether; Riptide represents Bathos; neither is a House-owned office or resource | exact enum mapping to existing stable route identities | foundation suite and documentation audit | **Implemented without route changes** |
| Aura Way | `AuraWayPath`, `AuraWayStage`, `recognize_aura_way_completion` | Houses teach; Aura Way orders the standard path; Stonebend recognizes completion without declaring perfection | semantic stage order and caller-supplied stable path, candidate, evidence, recognition, and Seal IDs | complete, missing-stage, reversed-insertion, and non-perfection proofs | **Implemented narrow generic path** |
| Aether–Current continuum | `MediumState`, `BurdenState`, `CurrentBatch`, `AetherBatch` | one common medium lineage; Current is heavy and embodied; refined Aether remains source-linked and is not automatically Aura | stable source/result IDs and explicit lineage/provenance | lawful fixture and reversed-fraction proof | **Implemented material first pass** |
| Lawful material Hollowing | `HollowingAuthorization`, `HollowingRequest`, `hollow_current` | existing Stonebend decision and Seal identities authorize removable burden; essential fractions remain | sorted stable fraction identities; source is immutable on rejection | lawful, unauthorized, scope, provenance, essential-fraction, and failure tests | **Implemented without full criminal code** |
| Proof | `ProofEvidence`, `ProofStatus` | measurement and genuine-process evidence remain distinct; no public numeric grade is locked | stable evidence IDs and explicit lifecycle | dual-proof assertion | **Implemented nonnumeric first pass** |
| Geographic refraction | `StoneFormationContext`, `RegionalStoneProfile`, `AuraManifestation` | geography forms the lattice; stone reveals supplied Aether as particular Aura; it creates no Aether | stable geography/profile/manifestation IDs plus Aether and Current provenance | multi-profile prism and two-geography proofs | **Implemented without House stone assignments** |
| Foundational stone behavior | `opal_profile`, `diamond_profile`, `quartz_profile` | Opal varies, Diamond concentrates, Quartz resonates; existing House resource rhetoric is nonexclusive | typed behavior/property sets independent of color | behavior, no-House-lock, and no-melting assertions | **Implemented extensible examples** |

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

Repository-wide boundary suites additionally protect kernel purity, institutional isolation, lineage legality, composition/provenance separation, Flynt hierarchy integrity, Recipe execution, decision replay, and persistence patterns.

## 7. Audit Conclusions and Expansion Targets

The Bond constitutional runtime is a functioning event-sourced authority. The regional transformation requirement does not justify changing that reducer or turning presentation geography into law.

The smallest coherent expansion is a sibling event-sourced regional Synthesis aggregate inside `src/constitutional/` that:

1. reuses `SandmanorForm`, `SandmanorLineage`, and `validate_sandmanor_transition`;
2. binds stable Being identities to explicit regional standing;
3. requires Sandmanor proof of the configuration and Glaüshouse Synthesis resolution authority;
4. requires evidence explicitly bound to the predecessor Being;
5. derives, rather than narratively invents, the only two ratified regional outcomes;
6. preserves predecessor identity and lineage;
7. creates typed Aura Field stewardship for Minotaur;
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
| Sandmanor constitutional law | `SandmanorRegistry` and typed design, method, evidence, demonstration, failure, reproduction, criticism, proof, recipe, teaching, Contest, accession, education, standard, emergency, regional-proof, and succession records in `src/world/sandmanor.rs` | callers construct explicit records and invoke `SandmanorRegistry::validate`; no legacy state is promoted implicitly | the singular `office.sandmanor.sandman` issues proof; active Sandman authority derives only from a completed reciprocal Contest plus Stonebend Title, Flynt recognition, public learning statement, and Seal | proposed records → validated House-specific constitutional state; `Gnome→Minotaur` and `Elf→Centaur` proof records bind the already-ratified regional rules without executing Synthesis | missing scope/evidence/version/lineage, erased failure, non-independent reproduction, invalid Contest, alternate authority origin, House substitution, wrong regional pairing, or lost successor obligations fails closed | House records are explicit domain inputs; common Bond and regional persistence remain authoritative for their own events; legacy Sandman roles migrate only to ordinary membership and never office | `tests/sandmanor_constitutional_architecture.rs`, `tests/sandmanor_documentation_conformance.rs`, and `sandmanor_constitutional_audit` | **Complete House-specific validator above frozen common runtime** |
| Four polarity states | `Sign`, `SignedQuantity`, `CurrentAuraEvaluation`, `ConstitutionalPolarity` in `src/constitutional/model.rs` and `bond.rs` | `run_polarity_scenario`; production submission remains `ConstitutionalRuntime::append` and `evaluate` | Formation/validation authority plus transaction, observation, and evaluation evidence | Active accumulated history → evaluated Active history | mismatched derived evaluation is rejected by `BondAggregate`; no append occurs | all four tags encode in `persistence.rs`; live replay uses the same reducer | `examples/constitutional_v2.rs`; `all_four_polarities_round_trip_through_persistence_and_replay` | **Complete** |
| Regional identity | `RegionalBeingId`, `RegionalBeingRecord`, `RegionalBeingStatus` in `src/constitutional/ids.rs` and `regional.rs`; existing `SandmanorForm` is reused | `RegionalSynthesisRuntime::register_being` with explicit `RegionalBeingRegistration` | Sandmanor-controlled regional jurisdiction and evidence bound to the Being | absent identity → active Gnome or Elf origin | conflicting/reused identity, evolved origin, invalid site, future jurisdiction, or missing/mismatched evidence is rejected | registration event in `HGREGV2` V1; replay rebuilds Being indexes | regional tests for unlined evolved forms, identity stability, and location non-inference | **Complete for ratified origins** |
| Regional occupation | `RegionalStanding`, `RegionalJurisdictionSnapshot`, `RegionalStandingKind`, `ConstitutionalRegion` in `regional.rs` | `RegionalJurisdictionSnapshot::from_catalog`; `scenario_regional_registration` is reusable fixture support | actual `site.sandmanor.aura-fields` or `site.sandmanor.aura-beach`, controlled by `institution.sandmanor.sandmen`; nonempty evidence | registration establishes standing; Synthesis preserves the required standing | Aura Sea cannot be primary standing; Visitor or wrong-region standing cannot trigger Synthesis | standing and catalog snapshot encode in every registration; replay validates it | `show-region`, wrong-region tests, location-only role rejection | **Complete** |
| Synthesis prerequisites | `RegionalSynthesisPrerequisites` with standing, lineage, readiness, constitutional-rule, and supporting evidence | field of `RegionalSynthesisCommand` | each item is a `SubjectEvidence` whose subject must equal the predecessor | complete prerequisites permit further validation; they do not themselves transform | missing or mismatched subject evidence returns a typed error and leaves history unchanged | every prerequisite is encoded; decode re-runs validation | lawful scenarios plus missing/mismatched evidence rejection | **Complete** |
| Synthesis authority | `RegionalSynthesisAuthority` | field of `RegionalSynthesisCommand`; fixtures use `HouseDecision::from_catalog` | accepted Sandmanor `Prove` from `institution.sandmanor.sandmen` and accepted Glaüshouse `Resolve` from `institution.glaushouse.medical-civilization` | validated active predecessor → authority-cleared Synthesis candidate | wrong House, function, institution, outcome, time, or reused decision is rejected before mutation | full historical `HouseDecision` snapshots encode and replay | `show-authority`; rejected-authority and forged-institution tests | **Complete for the two ratified regional rules** |
| Synthesis evidence | `SubjectEvidence` and evidence-bearing command/standing/jurisdiction types | explicit construction; never derived from presentation | stable `EvidenceRef`, nonempty and bound to predecessor identity | accepted evidence participates in a committed Synthesis record | no evidence, another Being's evidence, or jurisdiction without evidence is rejected | lossless binary codec; canonical decode | `show-evidence`; negative evidence tests | **Complete** |
| Synthesis lineage | `RegionalLineageEntry`, `RegionalBeingRecord::predecessor`, `lineage_history` | reducer output; inspection by `RegionalSynthesisRuntime::lineage` | lawful adjacent `SandmanorForm` transition plus the command authority/evidence | Gnome→Minotaur or Elf→Centaur in this frozen reducer; higher guardian stages use `sandmanor::milestone` | cross-lineage, unsupported mastery through this reducer, identity reuse, evolved origin registration, terminal source, or replay-altered predecessor is rejected | lineage is deterministically rebuilt from source events and checked against accepted event payload during replay | `show-lineage`; altered-history, origin-registration, and end-to-end tests | **Complete** |
| Gnome→Minotaur | `RegionalSynthesisRule::GnomeToMinotaurAuraFields` | `RegionalSynthesisRuntime::synthesize`; fixture `run_gnome_minotaur_scenario` | Sandmanor proof, Glaüshouse resolution, Aura Field standing, five prerequisite evidence records, result evidence | active Gnome with established Aura Field standing → active Minotaur; predecessor becomes `SynthesizedInto` | Gnome→Centaur, wrong region/function, missing authority/evidence, duplicate non-idempotent request, terminal source, or reused identity fails closed | V1 archive, V0→V1 migration fixture, live replay, canonical re-encoding, stable digest | `run gnome-minotaur`, persistence/replay/lineage/stewardship commands; regional suite | **Complete** |
| Aura Field stewardship | `AuraFieldsStewardship`, `AuraFieldsDuty`, `RegionalAssignment::Minotaur` | reducer-derived only; read by `stewardship` | inherited from accepted Synthesis authority and evidence | granted atomically with valid Minotaur result | no location-only grant; Centaur lookup returns none; Minotaur has no sea guardianship | derived during archive decode and replay; exact equality tested | seven typed duties demonstrated and asserted | **Complete** |
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

- `src/world/hueman_faculties.rs`: additive typed Hueman faculty ownership,
  manifestation bounds, deterministic `HGFAC` archive/replay, equal
  Prefog/Prefig Soul cycle, exact regional manifestations, Presynce ladder,
  and Resynce cultural-jurisdiction validation. It does not choose, prove, or
  execute Synthesis.
- `src/synthesis_recipe.rs`: optional `FacultyManifestation` storage on
  `SynthesisRecipe`; compilation validates manifestations while emitting only
  the established intent-derived scripts.
- `tests/hueman_faculties.rs`: ownership, serialization/replay, proof boundary,
  regional, cultural, Frame-preservation, and legacy-migration conformance.

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
- `src/world/sandmanor.rs`: House-specific design, proof, reciprocal teaching,
  Contest of Improvement, Sandman accession, education, standards, regional
  proof, and institutional-obligation validation above the frozen common
  runtime.
- `examples/constitutional_v2.rs`: executable capability demonstrator.
- `examples/constitutional_v2_service.rs`: executable witness of the
  application-owned request/event boundary.
- `examples/constitutional_v2_bench.rs`: dependency-free release benchmark
  harness.
- `tests/regional_synthesis.rs`: 17 regional conformance tests.
- `tests/sandmanor_constitutional_architecture.rs`: Sandmanor House-law and
  cross-House-boundary conformance.
- `tests/sandmanor_documentation_conformance.rs`: constitutional terminology,
  public-document, generated-projection, and recursion-kernel isolation lock.
- `tests/constitutional_v2_demonstration.rs`: seven cross-capability and
  end-to-end tests.
- `tests/constitutional_application_service.rs`: ten application-boundary
  conformance tests.

## 10. Deliberately Unavailable After Expansion

The expansion does not ratify Gnome→Centaur, Elf→Minotaur,
location-triggered automatic Synthesis, automatic office, regional transfer,
regional appellate authority, House-office succession, a mutable assignment
API, or a TUI. `AuraFieldsDuty` and `AuraBeachDuty` are constitutional duty
assignments; they do not yet implement crop simulation, pathfinding, combat,
escort AI, horizon sensing, or Current-control mechanics. Those mechanics may
consume the assignments later but cannot redefine them.

## 11. Boardwalk Gameplay Witness

The compromise now has one fully projected playable witness plus a reusable
four-House lived-lore layer above the V2 constitutional layer:

| Capability | Runtime owner | Authority boundary | Replay / persistence | Verification | Status |
|---|---|---|---|---|---|
| Aura Ridge + Boardwalk maps | `src/gameplay/world.rs` | Rust owns map, collision, position, facing, and interaction identity | gameplay event replay and V2 gameplay archive | legacy gameplay tests plus Boardwalk traversal/revisit test | **Implemented** |
| Returning-Goon case | `BoardwalkCase` | player gathers/discloses/supports; Returning Goon is recorded as final decision-maker | append-only gameplay events | evidence-gate, independent-return, and subject-attribution tests | **Implemented for one authored case** |
| Four typed Boardwalk outcomes | `BoardwalkOutcomeRecord` | Glaüshouse clears discharge; Returning Goon commits; Flynt recognizes; player support remains nonbinding | stable outcome ID, evidence, uncertainty, refusal, authority, presentation, and optional Bond replay | all-choice and vacant-office tests | **Implemented** |
| Three finite Boardwalk relationship Bonds | common `ConstitutionalRuntime` through `src/gameplay/boardwalk.rs` | Stonebend Name, Sandmanor proof, finite term, no ownership, choice-specific leave/challenge/refusal permissions | exact child `Formed` / `Validated` / `Activated` replay | active-phase, term, obligation, permission, and replay assertions | **Implemented for patronage, Goon Bond, and limited cooperation** |
| Live gameplay authority | schema-V2 `WorldSession` state | current office-holder records are loaded and validated; gameplay has no fixture fallback | office holders persist separately and are embedded in gameplay saves | non-fixture actor and fail-closed vacancy tests | **Implemented** |
| First-slice save/load | `src/gameplay/archive.rs` and protocol save root | schema and checksum fail closed; exact institutional snapshot embedded; V1 migration explicit; no partial mixed archive | disk restart/load test | checksum tamper, authority replay, and service restart tests | **Implemented for Hueman/Aura-Ridge/Boardwalk history; regional-registration history rejected** |
| Four-House functional lore | `src/world/lived_lore.rs` | exact House function, authority class, route jurisdiction, evidence, choice, transition, replay, presentation, and refusal | checksummed canonical JSON and exact replay | `functional_lore_integration` | **12 loops implemented: 3 per House, all 10 routes covered** |

## 12. Aura Ridge And Central Junction Public Economy

| Capability | Runtime owner | Authority boundary | Determinism / identity | Verification | Status |
|---|---|---|---|---|---|
| Four-Pole work classification | `src/world/central_junction.rs` | Sandmanor designs Form; Flynt engineers Function; Stonebend continues Form; Glaüshouse continues Function; poles are primary classifications, not monopolies | exhaustive `WorkObject × WorkLifecycle` mapping and stable enum identities | Central Junction architecture suite and executable audit | **Implemented as additive world law** |
| Work disposition | `WorkDisposition` | urgency selects timing; importance selects value | exhaustive boolean matrix produces Act, Cultivate, Reroute, Release | matrix conformance test | **Implemented** |
| Ordinary currency separation | `StandardCurrencyAmount`, `ValueInstrument` | one unnamed standard currency; Toke/Tokens, shares, event positions, indexes, and Gremlincoin remain distinct | neutral minor-unit amount; no lore currency name exists | currency/instrument conformance test | **Implemented without naming the currency** |
| Central Junction geography | `CentralJunctionDistrict`, `JunctionApproach`, `HouseSectorHall`, `CentralJunctionFunction` | Central Junction is a district; Exchange, Board, Clearing House, and Wire are institutions within it; the Service Tournament is its largest public Function | stable district, institution, approach, Hall, and Function IDs independent of catalog order | exact roster, geography, and executable audit tests | **Implemented without changing movement maps** |
| Service Tournament House registry | `PairedServiceIdentity`, `HouseServiceProfile` in `src/world/service_tournament.rs` | exactly one fixed paired-service reference and one representative per House; external models create no agencies, armed-service governments, or extra factions | four exhaustive enum identities and stable caller-supplied competitor IDs | exact-pair, four-representative, duplicate-House, and split-roster tests | **Implemented as additive shared Function law** |
| War of a Thousand Hues | `WarDefinition`, `HouseColorFamily`, `PaintMark`, `MarkGrammar`, `LayerReading` | nonlethal central event; Stonebend Blue, Sandmanor Red, Glaüshouse Green, Flynt Black; existing visual constitution remains the sole palette | stable War, mark, event, competitor, location, and evidence IDs; explicit layer sequence | nonlethal, four-color, palette-conformance, grammar, and layer-reading tests | **Implemented deterministic operational record model** |
| Service Tournament scenario and result runtime | `ServiceTournamentRuntime`, `TournamentScenario`, `TournamentObjective`, `ServiceMark`, `TournamentResult` | only nonlethal scenarios; complete constitutional scorecards; Service Marks retain source provenance; awards never transfer permanent sovereignty | schema `service-tournament/1.0.0`; stable caller-supplied identities; semantic-sequence replay; duplicate and reference validation | Service Tournament canon suite plus full constitutional regression suite | **Implemented deterministic runtime; no movement/combat executor** |
| Service Tournament archive and golden year | `service_tournament_archive`, `service_tournament_fixture`, `house_synthesis_semantics` | `HGSTA` V1 persists years, phases, alliances, emergencies, scores, violations, ordered marks, prizes, artifact refinements/custody, and House Synthesis events; custody is not sovereignty | caller-supplied IDs; canonical ordering; checksum; V0 migration; reducer replay; `service-tournament.canonical-year.v1` | archive/adversarial suite and `service_tournament_constitutional_audit` | **Frozen executable archive and fixture; no Godot or visual client** |
| Four public indexes | `MarketIndexDefinition`, `calculate_official_index` | Junction Board owns methodology, South Ridge Exchange calculates, Junction Wire publishes; Houses own none | stable index IDs plus recognized Hall evidence and checked arithmetic | index independence and calculation tests | **Implemented narrow deterministic calculation** |
| Enterprise/project classification | `EconomicClassification`, `ListedEnterprise`, `ListedProject` | one evidence-supported primary pole plus optional weighted secondary exposure | stable IDs; no insertion/index-position identity; weights total 100% | listing identity, primary, and secondary tests | **Implemented as validation schema** |
| Blackroot event proof | `EventContract`, `JunctionBoardDecision`, `ClearingSettlement`, `JunctionWirePublication` | House Halls attest domain facts; Board recognizes; Clearing House settles; Wire publishes; prices never choose the outcome | outcome definitions precede opening; exact evidence keys; stable contract/decision/settlement/publication IDs | fixture, dispute, incomplete-evidence, and price-independence tests | **Implemented deterministic constitutional fixture** |
| Market conflicts | `ConflictDisclosure`, `MarketDuty`, `validate_conflicts` | reviewers, settlement officials, and commanders cannot secretly act on positions they hold; disclosed recusal also prevents acting | actor and position IDs are stable; checks run before recognition or settlement | three-duty conflict rejection tests | **Implemented direct obvious-conflict safeguards** |
| Summit terminology | `SummitConcept` plus canon documents | Current Haze is unresolved possibility; Equal Gaze is reconciled perspective; Aura Beam reveals/transmits the visible shared future | ordinary market states remain Proposed/Open/UnderReview/Recognized/Disputed/Settled/Voided | documentation and schema conformance suite | **Preserved; no market bureaucracy introduced** |

## 13. Sandmanor Guardian and Reciprocal Succession

| Capability | Runtime owner | Authority boundary | Determinism / identity | Verification | Status |
|---|---|---|---|---|---|
| Aura Farm dual cultivation | `sandmanor::milestone::{CultivationDomain, ContentFarmAssessment}` | Sandmanor distinguishes Aura Fields and Content Farm; healthy content is not presumed corrupt | exhaustive typed practices | guardian/succession suite | **Implemented constitutional model** |
| Coastal gradient | `CoastalZone`, `NORTH_TO_SOUTH_COAST` | Southern Law remains Minoan/Sandmanor; Glaüshouse begins at the border | exact ordered enum projection | coast and transfer tests | **Implemented without movement changes** |
| Current Break hosting | `ManticorpCurrentBreakTraining` | Flynt commands Manticorp; Sandmanor authorizes territory; Minoans instruct coastal survival | exact Manticorp institution ID and House bindings | hosting proof | **Implemented boundary record** |
| Guardian progression | `GuardianState` event replay | Sandmanor proves and authorizes; Glaüshouse clears Synthesis; Form and mantle remain distinct | caller-supplied stable IDs; adjacent lineage validation; deterministic replay | two complete lineage fixtures | **Implemented through Hecaton/Pegasus** |
| Contest of Improvement | `ContestOfImprovementProof` | one Hecaton and one Pegasus teach reciprocally; review audits and crowd judges | all five trials, own-baseline deltas, stable voters, duplicate/conflict exclusion, order-independent result | Contest fixture and reversed-order replay | **Implemented constitutional adjudication** |
| Sandman succession | `SandmanConvergence`, `SandmanSuccession` | one-person maintained convergence plus one singular mantle; Aegon is historical alias | stable winner/loser identity; no fusion; max one active succession | succession and alias tests | **Implemented** |
