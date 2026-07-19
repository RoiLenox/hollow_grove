# Hollow Grove V2 Capability Matrix

Status: implemented surface as of 2026-07-19
Normative runtime root: `src/constitutional/`

## Reading the Matrix

“Supported” means the operation is executable through production types and a
production reducer. “Observed” means the capability is a read-only projection.
“Reserved” means an explicit typed failure protects an unratified procedure.
Paths are relative to the `hollow-grove` crate.

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
| Recognition | recognize proof | Flynt | Gallowry / Tross office | `InstitutionalRecognition` | recognition evidence | `append(FlyntRecognized)` | event tag 14 | yes | current schema | yes | ordinary lifecycle | premature Toke test | wrong House/missing recognition | stable | V2 |
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
| Gnome | Aura Fields | Minotaur | field stewardship, work, maintenance, and defense | required, accepted, exact Sandmen institution | required, accepted, exact medical civilization institution | standing, lineage, readiness, rule, supporting, result | `AuraFieldsStewardship`, seven duties | none | V1 canonical | exact | supported | full regional trace | `gnome-minotaur` | wrong region/function/authority/evidence/identity/terminal/duplicate | Supported |
| Elf | Aura Beach | Centaur | beach patrol and Aura Sea guardianship | required, accepted, exact Sandmen institution | required, accepted, exact medical civilization institution | standing, lineage, readiness, rule, supporting, result | `AuraBeachOccupation`, eight duties | `AuraSeaGuardianship`, four duties | V1 canonical | exact | supported | full regional trace | `elf-centaur` | wrong region/function/authority/evidence/identity/terminal/duplicate | Supported |
| Gnome | Aura Beach | Centaur | none | irrelevant after lineage rejection | irrelevant | cannot cure illegal lineage | none | none | no rejected event | prior history replayable | n/a | rejected trace | `gnome-centaur` | `REGIONAL_ILLEGAL_LINEAGE_TRANSITION` | Rejected |
| Elf | Aura Fields | Minotaur | none | irrelevant after lineage rejection | irrelevant | cannot cure illegal lineage | none | none | no rejected event | prior history replayable | n/a | rejected trace | `elf-minotaur` | `REGIONAL_ILLEGAL_LINEAGE_TRANSITION` | Rejected |
| Gnome | Aura Beach standing, Aura Fields requested | Minotaur | none | may be valid but insufficient | may be valid but insufficient | may be valid but insufficient | none | none | no rejected event | prior history replayable | n/a | rejected trace | `gnome-minotaur-wrong-region` | `REGIONAL_INSUFFICIENT_STANDING` | Rejected |
| Elf | Aura Fields standing, Aura Beach requested | Centaur | none | may be valid but insufficient | may be valid but insufficient | may be valid but insufficient | none | none | no rejected event | prior history replayable | n/a | rejected trace | `elf-centaur-wrong-region` | `REGIONAL_INSUFFICIENT_STANDING` | Rejected |

## Typed Regional Assignment Matrix

| Assignment | Holder form | Required region | Typed responsibilities | Inspection API | Cannot grant |
|---|---|---|---|---|---|
| `AuraFieldsStewardship` | Minotaur | Aura Fields | tend crops; guard boundary; carry load; maintain route; guard harvest; protect worker; stabilize field Current | `RegionalSynthesisRuntime::stewardship` | Aura Beach patrol or Aura Sea guardianship |
| `AuraBeachOccupation` | Centaur | Aura Beach | roam; patrol shoreline; guard sea access; watch route; escort; recognize horizon change; defend incursion; maintain land-sea boundary | `beach_occupation` | Aura Fields stewardship |
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
| Centaur Aura Fields stewardship | `require_stewardship` returns `REGIONAL_ASSIGNMENT_NOT_HELD` | field and coastal authority are distinct |
| Aura Sea as primary Synthesis site | Rejected | Aura Sea is the typed guardianship target attached to Aura Beach Centaur Synthesis |
| regional transfer | Reserved | no transfer command or authority is ratified |
| appellate procedure | Reserved | `HouseAppealCourt` remains explicit fail-closed law |
| TUI domain decisions | Forbidden | presentation consumes events; reducer owns law |
