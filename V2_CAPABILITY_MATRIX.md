# Hollow Grove V2 Capability Matrix

Status: implemented surface as of 2026-07-19
Normative runtime root: `src/constitutional/`

## Reading the Matrix

“Supported” means the operation is executable through production types and a
production reducer. “Observed” means the capability is a read-only projection.
“Reserved” means an explicit typed failure protects an unratified procedure.
Paths are relative to the `hollow-grove` crate.

## Additive Hueman Faculty Matrix

| Faculty | Domain and authority | Typed posture | Hard boundary | Persistence / migration | Status |
|---|---|---|---|---|---|
| Presynce | Body / Stonebend | observe physical emergence through Frame and Flow | no automatic dodge or counter; entire eight-stage Current Form ladder remains Presynce | deterministic `HGFAC`; legacy infers none | Supported descriptive law |
| Resynce | Spirit / Flynt | observe relational emergence through Bond, affiliation, routes, encounters, and recognition | no mind reading or Frame mutation; We Fairy Men/Aura Ridge remains distinct from Gallows/Flynt civic recognition | deterministic `HGFAC`; legacy infers none | Supported descriptive law |
| Precog | Mind / Glaüshouse | evaluate probable continuation from evidence and Glow | no certainty or guaranteed future | deterministic `HGFAC`; legacy infers none | Supported descriptive law |
| Prefog | Soul Interior / Minorian-Gnome | generate multiple legal possibilities | possibility is not proof; equal to Prefig | deterministic `HGFAC`; legacy infers no mastery | Supported descriptive law |
| Prefig | Soul Exterior / Minoan-Elf | provisionally embody the selected legal candidate | prototype is not proof and does not execute Synthesis | deterministic `HGFAC`; legacy infers no mastery | Supported descriptive law |

The existing deterministic decision logic performs Choose. The existing
Sandmanor proof lifecycle alone advances Prefig evidence. Faculties are stored
optionally on `SynthesisRecipe` and emit no independent execution scripts.

## Stonebend Constitutional Layer Above The Frozen Runtime

`STONEBEND_CONSTITUTION_V2.md` is ratified Stonebend-specific law. Its executable
projection is `src/world/stonebend.rs`; its neutral institutional projection is
`src/world/house_institutions.rs`. These surfaces validate Name, Title,
accession, Seal, Hollowing, extraction, custody, rename, succession, and
Tombstone records without changing the common reducer below.

| Domain concept | Supported operation | Authority placement | Positive proof | Failure proof | Persistence boundary |
|---|---|---|---|---|---|
| Principal authorities | validate exact roster and power separation | Hypergiant / Proliteriate / High Freemason / Freemason | Stonebend architecture tests | invalid highest authority or power separation | stable institutional IDs |
| Hypergiant selection | record the semantic Diamond path | stable Claim; independent Freemason examination; Proliteriate Yield hearing; relinquishment; consequence descent; Proof of Persistence; The Lazerhorn; eligibility; investiture | ordered and reversed-insertion succession tests | missing Lazerhorn, self-certification, recommendation, lineage, and shortened-return tests | stable Claim, evidence, succession, Diamond tenure, and Tombstone IDs |
| Name / Title / office | validate active constitutional records | Stonebend registry | lawful registry and audit unit test | unlawful origin, missing accession, duplicate Hypergiant, provisional or Tombstoned Name | typed records above common runtime |
| Seal / Mirror evidence | bind issuer, subject, scope, decision | Freemason issuance; Mirror remains evidence | lawful registry test | wrong issuer, subject, scope, or decision | typed Seal and evidence IDs |
| Lawful Hollowing | validate ordinary or emergency procedure | Stonebend authorization and qualified execution | registry validation | missing consent elements, continuity, custody, or post-event review | typed Hollowing/extraction/custody records |
| Rename / succession / Tombstone | preserve history and prevent ended authority | Stonebend Registry | registry validation | erased former Name, dropped benefits or obligations, active Tombstone target | typed immutable records |
| Title lifecycle | recognize, activate, maintain, renew, intervene, restore, and end | existing Stonebend Title core plus policy-defined delegated authority | Third Pass deterministic lifecycle suite | missing semantic stage, unmet activation, untargeted intervention, erased break, or missing Tombstone | existing stable Title/evidence/Tombstone IDs; no new archive schema |
| Diamond vacancy continuity | preserve existing duties without transferring Diamond | delegated Claim, boundary, and Yield traces | bounded continuity and emergency proofs | investiture, appointment, sovereign law, permanent scope growth, missing termination, or missing review | stable mandate/action IDs above existing office history |
| High Freemason replacement | independently review one replacement seal bearer | independent Forge reviewers, Proliteriate Yield accountability, occupied-Diamond boundary recognition where applicable | reversed-candidate-order replacement proof | self-certification, outgoing appointment, Hypergiant/temporary-witness absorption, or duplicate active bearer | stable Claim/review/tenure/Seal/Tombstone records |

## Constitutional Runtime Matrix

| Domain concept | Supported operation | House | Institution / office | Required authority | Required evidence | Reducer entry point | Persisted representation | Replay | Migration | Trace | Scenario | Positive test | Failure test | Public API | Stability |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Wave | record an explicit cause | none inferred | none inferred | caller owns causal source | `WaveRecord::origin` | `ConstitutionalRuntime::record_wave` | Wave table in V1 archive | yes | current-version rewrite | Wave command output | `kernel-wave` | `completed_kernel_pass_records_a_wave_without_moving_current` | Wave ID conflict/future reference | stable | V2 |
| Kernel adapter | completed pass → Wave | none | none | none manufactured | `kernel_pass_evidence` | `record_kernel_wave` | as Wave | yes | as Wave | `show-wave` | `kernel-wave` | adapter and end-to-end tests | implicit Current count remains zero | stable | V2 |
| Bond formation/naming | form | Stonebend | `institution.stonebend.constitution`; `office.stonebend.hypergiant` | `ConstitutionalIdentity` | Wave, jurisdiction, naming, formation, starting Current | `ConstitutionalRuntime::append(Formed)` | event tag 0 | yes | current schema | yes | `ordinary-lifecycle` | lifecycle tests | malformed formation/wrong House | stable | V2 |
| Bond proof | validate | Sandmanor | `institution.sandmanor.sandmen`; `office.sandmanor.sandman` | `WitnessedImprovement` | proof and validation evidence | `append(Validated)` | event tag 1 | yes | current schema | yes | ordinary lifecycle | lifecycle test | wrong House/outcome/evidence | stable | V2 |
| Activation | activate chemistry | formation authority already established | governing jurisdiction | validated predecessor | activation evidence | `append(Activated)` | event tag 2 | yes | current schema | yes | all polarity scenarios | stage tests | validation skipped | stable | V2 |
| Current | move and accumulate signed quantities | no new House grant | active Bond | permission and causal Wave | transaction/edge evidence | `append(CurrentMoved)`; `accumulate_current` | exact `u128`, sign, unit, edges | yes | current schema | yes | four polarity scenarios | polarity suite | term/unit/holding errors | stable | V2 |
| Aura | observe signed meaning | no new House grant | active Bond | participant observer | subject/observation evidence | `append(AuraObserved)` | exact `u128`, sign, unit | yes | current schema | yes | four polarity scenarios | polarity suite | observer/unit/term errors | stable | V2 |
| Evaluation | derive polarity | no new House grant | validated active Bond | accumulated Current and observed Aura | evaluation evidence | `ConstitutionalRuntime::evaluate` | `CurrentAuraEvaluation` | yes | current schema | yes | four polarity scenarios | table-driven round trip | mismatch/unevaluable domain | stable | V2 |
| Default | declare/resolve | participant standing; no new final House in current rule | source Bond | formation participant/obligation | declaration/resolution evidence | Default events | event tags 20/21 | yes | current schema | event trace supported | existing conformance fixture | confirmed default test | unknown/pending default | stable | V2 partial demonstrations |
| Challenge | file/resolve | Sandmanor resolves | Sandmanor proof office | challenger standing + `WitnessedImprovement` | challenged reference, filing, proof, resolution | Challenge events | event tags 18/19 | yes | current schema | event trace supported | existing conformance fixture | pending challenge test | no standing/wrong proof | stable | V2 partial demonstrations |
| Appeal | reject invocation | none ratified | none | unavailable | unavailable | `appeal_challenge` | none | n/a | n/a | typed failure | audit | Reserved procedure test | always `HouseAppealCourt` reserved | stable rejection | Reserved |
| Maturity | freeze living chemistry | term law | governing jurisdiction | all predecessor stages, no blockers | maturity evidence | `append(Matured)` | event tag 7 | yes | current schema | yes | ordinary lifecycle | no-skip/term tests | premature/current-after-term | stable | V2 |
| Excess | calculate signed remainder | governing rule set | governing jurisdiction | mature state | prior event history | `calculate_excess` | event tag 8 | yes | current schema | yes | ordinary lifecycle | lifecycle test | maturity required | stable | V2 |
| Clearance | decide condensation | Glaüshouse | medical civilization / Prima Donna | `PublicClearance` | decision and eligibility evidence | `append(CondensationDecided)` | event tag 9 | yes | current schema | yes | ordinary lifecycle | full lifecycle | missing/rejected clearance | stable | V2 |
| Tombstone | form/validate proof | Glaüshouse clearance; independent validator | governing institution retained | eligible excess; nonparticipant validator | complete proof, validation basis, exact digest | Tombstone events | global Tombstone index and events | yes | current schema | yes | ordinary lifecycle | digest and Tombstone tests | premature/mismatch/resurrection | stable | V2 |
| Recognition | recognize proof | Flynt | Tross office through the canonical Flynt authority boundary | `InstitutionalRecognition` | recognition evidence | `append(FlyntRecognized)` | event tag 14 | yes | current schema | yes | ordinary lifecycle | premature Toke test | wrong House/missing recognition | stable | V2 |
| Toke | record validated proof | prior Flynt recognition | historical index | validated Tombstone already exists | Toke evidence | `append(TokeRecorded)` | global Toke→Tombstone index | yes | current schema | yes | ordinary lifecycle | full lifecycle | missing/duplicate Tombstone | stable | V2 |
| Resolution | final disposition | Glaüshouse | medical civilization / Prima Donna | `FinalJudgmentAnswerability` | resolution evidence | `append(Resolved)` | event tag 21 | yes | current schema | yes | ordinary lifecycle | full lifecycle | phase/cardinality/authority | stable | V2 |
| Renewal | reserve/form successor | Glaüshouse parent; Stonebend child | both institutions | resolution plus new naming | resolution and inheritance evidence | parent `Resolved`; child `Formed` | reciprocal parent/successor graph | yes | current schema | yes | ordinary lifecycle | successor integrity test | unreserved/missing successor | stable | V2 |
| House succession | reject unratified procedure | none ratified | none | unavailable | unavailable | `invoke_reserved_procedure` | none | n/a | n/a | typed failure | audit | Reserved procedure test | always fails closed | stable rejection | Reserved |

## Polarity Matrix

| Current | Aura | Constitutional value | Construction | Lifecycle | Persistence | Replay | Migration | Scenario | Failure coverage | Status |
|---|---|---|---|---|---|---|---|---|---|---|
| Positive | Positive | `PositiveCurrentPositiveAura` | exact signed quantities | evaluated Active through mature proof path | explicit enum tag | same reducer | current schema | `positive-positive` | evaluation mismatch and stage failures | Supported |
| Positive | Negative | `PositiveCurrentNegativeAura` | exact signed quantities | evaluated Active through mature proof path | explicit enum tag | same reducer | current schema | `positive-negative` | evaluation mismatch and stage failures | Supported |
| Negative | Positive | `NegativeCurrentPositiveAura` | exact signed quantities | evaluated Active; negative magnitude remains signed history | explicit enum tag | same reducer | current schema | `negative-positive` | evaluation mismatch and stage failures | Supported |
| Negative | Negative | `NegativeCurrentNegativeAura` | exact signed quantities | evaluated Active; both domains remain distinct | explicit enum tag | same reducer | current schema | `negative-negative` | evaluation mismatch and stage failures | Supported |

Aura never offsets Current. The matrix records two independently signed
domains. `CurrentAuraEvaluation` derives the quadrant; it does not combine the
domains into a single score.

## Regional Synthesis Matrix

| Predecessor | Required region | Lawful result | Canonical function | Sandmanor proof | Glaüshouse resolution | Subject evidence | Stewardship | Guardianship | Persistence | Replay | V0 migration | Trace | Scenario | Failure coverage | Status |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Gnome | Aura Field | Minotaur | field stewardship, work, maintenance, and defense | required, accepted, exact Sandmen institution | required, accepted, exact medical civilization institution | standing, lineage, readiness, rule, supporting, result | `AuraFieldsStewardship`, seven duties | none | V1 canonical | exact | supported | full regional trace | `gnome-minotaur` | wrong region/function/authority/evidence/identity/terminal/duplicate | Supported |
| Elf | Aura Beach | Centaur | beach patrol and Aura Sea guardianship | required, accepted, exact Sandmen institution | required, accepted, exact medical civilization institution | standing, lineage, readiness, rule, supporting, result | `AuraBeachOccupation`, eight duties | `AuraSeaGuardianship`, four duties | V1 canonical | exact | supported | full regional trace | `elf-centaur` | wrong region/function/authority/evidence/identity/terminal/duplicate | Supported |
| Gnome | Aura Beach | Centaur | none | irrelevant after lineage rejection | irrelevant | cannot cure illegal lineage | none | none | no rejected event | prior history replayable | n/a | rejected trace | `gnome-centaur` | `REGIONAL_ILLEGAL_LINEAGE_TRANSITION` | Rejected |
| Elf | Aura Field | Minotaur | none | irrelevant after lineage rejection | irrelevant | cannot cure illegal lineage | none | none | no rejected event | prior history replayable | n/a | rejected trace | `elf-minotaur` | `REGIONAL_ILLEGAL_LINEAGE_TRANSITION` | Rejected |
| Gnome | Aura Beach standing, Aura Field requested | Minotaur | none | may be valid but insufficient | may be valid but insufficient | may be valid but insufficient | none | none | no rejected event | prior history replayable | n/a | rejected trace | `gnome-minotaur-wrong-region` | `REGIONAL_INSUFFICIENT_STANDING` | Rejected |
| Elf | Aura Field standing, Aura Beach requested | Centaur | none | may be valid but insufficient | may be valid but insufficient | may be valid but insufficient | none | none | no rejected event | prior history replayable | n/a | rejected trace | `elf-centaur-wrong-region` | `REGIONAL_INSUFFICIENT_STANDING` | Rejected |

## Typed Regional Assignment Matrix

| Assignment | Holder form | Required region | Typed responsibilities | Inspection API | Cannot grant |
|---|---|---|---|---|---|
| `AuraFieldsStewardship` | Minotaur | Aura Field | tend crops; guard boundary; carry load; maintain route; guard harvest; protect worker; stabilize field Current | `RegionalSynthesisRuntime::stewardship` | Aura Beach patrol or Aura Sea guardianship |
| `AuraBeachOccupation` | Centaur | Aura Beach | roam; patrol shoreline; guard sea access; watch route; escort; recognize horizon change; defend incursion; maintain land-sea boundary | `beach_occupation` | Aura Field stewardship |
| `AuraSeaGuardianship` | Centaur | Aura Sea | guard access; watch horizon; defend boundary; maintain lawful passage | `guardianship` | primary Aura Sea Synthesis standing or field authority |

## Public API and Stability Matrix

| Surface | Canonical entry points | Mutation power | Bypass status | Stability |
|---|---|---|---|---|
| Bond runtime | `ConstitutionalRuntime::{record_wave,append,accumulate_current,evaluate,calculate_excess,replay}` | constitutional event submission | reducer validates every event | Stable V2 |
| Regional runtime | `RegionalSynthesisRuntime::{register_being,synthesize,tombstone_being,replay}` | regional event submission | no assignment setter; result derived | Stable V2 |
| Regional inspection | `being`, `synthesis`, `lineage`, `stewardship`, `beach_occupation`, `guardianship` | none | read-only references | Stable V2 |
| Archive | Bond and regional encode/decode/read/write/migrate functions | creates/loads bytes, not unchecked state | decode uses production reducers | Stable V2 |
| Scenario support | `run_*_scenario`, `scenario_*` constructors | submits through public runtime | fixtures do not mutate internals | Example-support stable |
| Trace | `trace_*` and trace records | none | no reducer or assignment method | Stable projection |
| TUI contract | `TuiCommand`, `TuiEvent`, `tui_events_from_trace` | commands describe intent; events present results | no internal reducer exposure | V2 readiness contract |
| Application service | `ConstitutionalApplicationService::{execute,execute_streaming}`, `TuiRequest`, `TuiResponse` | selects production-built runtime/archive; executes replay, persistence, migration, inspection, audit; delivers ordered events to a sink | private runtime/archive mutation; idempotent request IDs; safe-boundary cancellation | V2 stable demonstration boundary |
| Gameplay service | `GameApplicationService`, `GameProtocolService`, `GameplayCommand`, protocol V1 intents | owns Hueman/Aura-Ridge/Boardwalk history, four typed outcomes, and three finite common Bonds | Godot receives immutable views only; capable-subject attribution; live `WorldSession`; revision/idempotency; schema-V2 checksum archive with embedded authority and V1 migration | Boardwalk vertical-slice boundary |
| Functional-lore catalog | `FunctionalLoreCatalog`, 12 canonical definitions | binds three lived loops per House to exact House authority and all ten routes | twelve-field validation, fail-closed incumbency, checksummed JSON, exact replay, no kernel imports | Four-House world integration boundary |
| Compatibility alias | `hollow_grove::Bond = KernelBond` | recursion selector only | cannot be confused through constitutional module path | Preserved |

## Deliberately Unsupported Matrix

| Requested or conceivable behavior | Status | Constitutional reason |
|---|---|---|
| Gnome→Centaur | Rejected | cross-lineage transition not ratified |
| Elf→Minotaur | Rejected | cross-lineage transition not ratified |
| Minotaur→Hecaton regional execution | Reserved | typed lineage adjacency exists, but no regional rule/authority/function assignment is ratified |
| Centaur→Pegasus regional execution | Reserved | same |
| automatic Synthesis from location | Rejected | standing is a prerequisite, never the transformation |
| Minotaur Aura Sea guardianship | `require_guardianship` returns `REGIONAL_ASSIGNMENT_NOT_HELD` | field and coastal authority are distinct |
| Centaur Aura Field stewardship | `require_stewardship` returns `REGIONAL_ASSIGNMENT_NOT_HELD` | field and coastal authority are distinct |
| Aura Sea as primary Synthesis site | Rejected | Aura Sea is the typed guardianship target attached to Aura Beach Centaur Synthesis |
| regional transfer | Reserved | no transfer command or authority is ratified |
| appellate procedure | Reserved | `HouseAppealCourt` remains explicit fail-closed law |
| TUI domain decisions | Forbidden | presentation consumes events; reducer owns law |
| mixed gameplay archive containing regional-registration events | Rejected | first-slice archive refuses partial cross-runtime persistence until a lossless migration is implemented |
