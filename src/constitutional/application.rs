//! Application-owned boundary for presentation clients.
//!
//! The service owns the selected production runtime and its canonical archive.
//! It accepts presentation-neutral requests, invokes existing scenario and
//! reducer APIs, and emits observations only after those APIs return. It is not
//! a reducer and contains no House, polarity, or regional Synthesis law.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TuiRequest {
    pub id: String,
    pub command: TuiCommand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplicationResponseStatus {
    Completed,
    ConstitutionallyRejected,
    Cancelled,
}

impl ApplicationResponseStatus {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Completed => "Completed",
            Self::ConstitutionallyRejected => "ConstitutionallyRejected",
            Self::Cancelled => "Cancelled",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TuiResponse {
    pub request_id: String,
    pub status: ApplicationResponseStatus,
    pub events: Vec<TuiEvent>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectedArchiveMetadata {
    pub scenario: String,
    pub runtime: &'static str,
    pub byte_length: usize,
    pub digest: String,
}

#[derive(Debug, Clone)]
enum OwnedRuntime {
    Bond(ConstitutionalRuntime),
    Regional(RegionalSynthesisRuntime),
}

#[derive(Debug, Clone)]
struct ScenarioSnapshot {
    name: String,
    trace: ConstitutionalTrace,
    runtime: OwnedRuntime,
    archive: Vec<u8>,
    legacy_archive: Option<Vec<u8>>,
    regional_subject: Option<RegionalBeingId>,
}

impl ScenarioSnapshot {
    const fn runtime_name(&self) -> &'static str {
        match self.runtime {
            OwnedRuntime::Bond(_) => "Bond",
            OwnedRuntime::Regional(_) => "RegionalSynthesis",
        }
    }

    fn archive_digest(&self) -> String {
        fn fnv1a64(bytes: &[u8]) -> u64 {
            let mut digest = 0xcbf29ce484222325_u64;
            for byte in bytes {
                digest ^= u64::from(*byte);
                digest = digest.wrapping_mul(0x100000001b3);
            }
            digest
        }

        match self.runtime {
            OwnedRuntime::Bond(_) => format!("fnv1a64-v1:{:016x}", fnv1a64(&self.archive)),
            OwnedRuntime::Regional(_) => {
                format!("fnv1a64-regional-v1:{:016x}", fnv1a64(&self.archive))
            }
        }
    }

    fn replay_equivalent(&self) -> Result<bool, ApplicationServiceError> {
        match &self.runtime {
            OwnedRuntime::Bond(runtime) => {
                let replayed = ConstitutionalRuntime::replay(
                    runtime.waves().cloned(),
                    runtime.events().iter().cloned(),
                )
                .map_err(application_error)?;
                Ok(
                    encode_constitutional_archive(&replayed).map_err(application_error)?
                        == self.archive,
                )
            }
            OwnedRuntime::Regional(runtime) => {
                let replayed = RegionalSynthesisRuntime::replay(runtime.events().iter().cloned())
                    .map_err(application_error)?;
                Ok(replayed == *runtime)
            }
        }
    }

    fn persisted_canonically(&self) -> Result<bool, ApplicationServiceError> {
        match &self.runtime {
            OwnedRuntime::Bond(_) => {
                let decoded =
                    decode_constitutional_archive(&self.archive).map_err(application_error)?;
                Ok(
                    encode_constitutional_archive(&decoded).map_err(application_error)?
                        == self.archive,
                )
            }
            OwnedRuntime::Regional(_) => {
                let decoded = decode_regional_archive(&self.archive).map_err(application_error)?;
                Ok(encode_regional_archive(&decoded).map_err(application_error)? == self.archive)
            }
        }
    }

    fn migrated_canonically(&self) -> Result<(u16, u16, bool), ApplicationServiceError> {
        match &self.runtime {
            OwnedRuntime::Bond(_) => {
                let migrated =
                    migrate_constitutional_archive(&self.archive).map_err(application_error)?;
                Ok((
                    CONSTITUTIONAL_ARCHIVE_VERSION,
                    CONSTITUTIONAL_ARCHIVE_VERSION,
                    migrated == self.archive,
                ))
            }
            OwnedRuntime::Regional(_) => {
                let legacy = self.legacy_archive.as_ref().ok_or_else(|| {
                    ApplicationServiceError::Scenario(
                        "regional snapshot omitted its supported legacy archive".into(),
                    )
                })?;
                let migrated = migrate_regional_archive(legacy).map_err(application_error)?;
                Ok((
                    REGIONAL_LEGACY_ARCHIVE_VERSION,
                    REGIONAL_ARCHIVE_VERSION,
                    migrated == self.archive,
                ))
            }
        }
    }
}

#[derive(Debug, Clone)]
struct CompletedRequest {
    command: TuiCommand,
    response: TuiResponse,
}

/// The sole presentation-facing owner of a selected constitutional scenario.
///
/// Runtime and archive mutation are intentionally private. Callers submit a
/// `TuiRequest` and receive immutable `TuiEvent` values. Exact request retries
/// return the exact prior response; conflicting reuse of an ID fails closed.
#[derive(Debug)]
pub struct ConstitutionalApplicationService {
    session_id: String,
    session_announced: bool,
    selected: Option<ScenarioSnapshot>,
    completed_requests: BTreeMap<String, CompletedRequest>,
    cancelled_requests: BTreeSet<String>,
}

impl ConstitutionalApplicationService {
    pub fn new(session_id: impl Into<String>) -> Result<Self, ApplicationServiceError> {
        let session_id = session_id.into();
        if session_id.trim().is_empty() {
            return Err(ApplicationServiceError::InvalidSessionId);
        }
        Ok(Self {
            session_id,
            session_announced: false,
            selected: None,
            completed_requests: BTreeMap::new(),
            cancelled_requests: BTreeSet::new(),
        })
    }

    #[must_use]
    pub fn selected_scenario(&self) -> Option<&str> {
        self.selected
            .as_ref()
            .map(|snapshot| snapshot.name.as_str())
    }

    #[must_use]
    pub fn constitutional_event_count(&self) -> usize {
        match self.selected.as_ref().map(|snapshot| &snapshot.runtime) {
            Some(OwnedRuntime::Bond(runtime)) => runtime.events().len(),
            _ => 0,
        }
    }

    #[must_use]
    pub fn regional_event_count(&self) -> usize {
        match self.selected.as_ref().map(|snapshot| &snapshot.runtime) {
            Some(OwnedRuntime::Regional(runtime)) => runtime.events().len(),
            _ => 0,
        }
    }

    #[must_use]
    pub fn selected_archive_metadata(&self) -> Option<SelectedArchiveMetadata> {
        let snapshot = self.selected.as_ref()?;
        Some(SelectedArchiveMetadata {
            scenario: snapshot.name.clone(),
            runtime: snapshot.runtime_name(),
            byte_length: snapshot.archive.len(),
            digest: snapshot.archive_digest(),
        })
    }

    pub fn execute(&mut self, request: TuiRequest) -> Result<TuiResponse, ApplicationServiceError> {
        if request.id.trim().is_empty() {
            return Err(ApplicationServiceError::InvalidRequestId);
        }
        if let Some(completed) = self.completed_requests.get(&request.id) {
            return if completed.command == request.command {
                Ok(completed.response.clone())
            } else {
                Err(ApplicationServiceError::RequestIdConflict(request.id))
            };
        }

        if self.cancelled_requests.remove(&request.id) {
            let response = self.finish_response(
                request.id.clone(),
                ApplicationResponseStatus::Cancelled,
                vec![event(
                    "application",
                    TuiEventKind::RequestCancelled,
                    [("request_id", request.id.as_str())],
                )],
            );
            self.remember(request, &response);
            return Ok(response);
        }

        let (status, events) = match &request.command {
            TuiCommand::Catalog => (ApplicationResponseStatus::Completed, catalog_events()),
            TuiCommand::RunScenario { scenario } => {
                let snapshot = materialize_scenario(scenario)?;
                let rejected = descriptor(scenario)?.expectation == ScenarioExpectation::Rejected;
                let events = tui_events_from_trace(&snapshot.trace);
                self.selected = Some(snapshot);
                (
                    if rejected {
                        ApplicationResponseStatus::ConstitutionallyRejected
                    } else {
                        ApplicationResponseStatus::Completed
                    },
                    events,
                )
            }
            TuiCommand::ReplayScenario { scenario } => {
                let snapshot = self.snapshot(scenario)?;
                let equivalent = snapshot.replay_equivalent()?;
                (
                    ApplicationResponseStatus::Completed,
                    vec![
                        bare_event(scenario, TuiEventKind::ReplayStarted),
                        event(
                            scenario,
                            TuiEventKind::ReplayCompleted,
                            [
                                ("event_count", runtime_event_count(snapshot).to_string()),
                                ("equivalent", yes_no(equivalent).to_owned()),
                            ],
                        ),
                    ],
                )
            }
            TuiCommand::PersistScenario { scenario } => {
                let snapshot = self.snapshot(scenario)?;
                let canonical = snapshot.persisted_canonically()?;
                (
                    ApplicationResponseStatus::Completed,
                    vec![event(
                        scenario,
                        TuiEventKind::Persisted,
                        [
                            ("runtime", snapshot.runtime_name().to_owned()),
                            ("archive_bytes", snapshot.archive.len().to_string()),
                            ("archive_digest", snapshot.archive_digest()),
                            ("canonical", yes_no(canonical).to_owned()),
                        ],
                    )],
                )
            }
            TuiCommand::MigrateScenario { scenario } => {
                let snapshot = self.snapshot(scenario)?;
                let (source, target, canonical) = snapshot.migrated_canonically()?;
                (
                    ApplicationResponseStatus::Completed,
                    vec![
                        event(
                            scenario,
                            TuiEventKind::MigrationStarted,
                            [("source_version", source.to_string())],
                        ),
                        event(
                            scenario,
                            TuiEventKind::MigrationCompleted,
                            [
                                ("source_version", source.to_string()),
                                ("target_version", target.to_string()),
                                ("canonical", yes_no(canonical).to_owned()),
                            ],
                        ),
                    ],
                )
            }
            TuiCommand::InspectTrace { scenario } => {
                let snapshot = self.snapshot(scenario)?;
                (
                    ApplicationResponseStatus::Completed,
                    tui_events_from_trace(&snapshot.trace),
                )
            }
            TuiCommand::InspectAuthority { scenario } => (
                ApplicationResponseStatus::Completed,
                inspect_authority(self.snapshot(scenario)?)?,
            ),
            TuiCommand::InspectEvidence { scenario } => (
                ApplicationResponseStatus::Completed,
                inspect_evidence(self.snapshot(scenario)?)?,
            ),
            TuiCommand::InspectPolarity { scenario } => (
                ApplicationResponseStatus::Completed,
                inspect_polarity(self.snapshot(scenario)?)?,
            ),
            TuiCommand::InspectLineage { scenario } => (
                ApplicationResponseStatus::Completed,
                inspect_lineage(self.snapshot(scenario)?)?,
            ),
            TuiCommand::InspectRegion { scenario } => (
                ApplicationResponseStatus::Completed,
                inspect_region(self.snapshot(scenario)?)?,
            ),
            TuiCommand::InspectStewardship { scenario } => (
                ApplicationResponseStatus::Completed,
                inspect_stewardship(self.snapshot(scenario)?)?,
            ),
            TuiCommand::InspectGuardianship { scenario } => (
                ApplicationResponseStatus::Completed,
                inspect_guardianship(self.snapshot(scenario)?)?,
            ),
            TuiCommand::Audit => (ApplicationResponseStatus::Completed, audit_events()?),
            TuiCommand::Cancel { request_id } => {
                if request_id == &request.id {
                    return Err(ApplicationServiceError::SelfCancellation(request.id));
                }
                if self.completed_requests.contains_key(request_id) {
                    return Err(ApplicationServiceError::RequestAlreadyCompleted(
                        request_id.clone(),
                    ));
                }
                self.cancelled_requests.insert(request_id.clone());
                (
                    ApplicationResponseStatus::Completed,
                    vec![event(
                        "application",
                        TuiEventKind::CancellationAccepted,
                        [("request_id", request_id.as_str())],
                    )],
                )
            }
        };

        let response = self.finish_response(request.id.clone(), status, events);
        self.remember(request, &response);
        Ok(response)
    }

    /// Executes one atomic request and delivers its immutable observation
    /// records to a presentation sink in canonical sequence order.
    ///
    /// The reducer operation completes before an accepted or state-change
    /// record can reach the sink. The sink has no access to runtime state and
    /// cannot affect the already-determined response.
    pub fn execute_streaming(
        &mut self,
        request: TuiRequest,
        mut emit: impl FnMut(&TuiEvent),
    ) -> Result<ApplicationResponseStatus, ApplicationServiceError> {
        let response = self.execute(request)?;
        for event in &response.events {
            emit(event);
        }
        Ok(response.status)
    }

    fn snapshot(&self, scenario: &str) -> Result<&ScenarioSnapshot, ApplicationServiceError> {
        self.selected
            .as_ref()
            .filter(|snapshot| snapshot.name == scenario)
            .ok_or_else(|| ApplicationServiceError::ScenarioNotSelected(scenario.to_owned()))
    }

    fn finish_response(
        &mut self,
        request_id: String,
        status: ApplicationResponseStatus,
        mut events: Vec<TuiEvent>,
    ) -> TuiResponse {
        if !self.session_announced {
            events.insert(
                0,
                event(
                    "application",
                    TuiEventKind::SessionStarted,
                    [("session_id", self.session_id.as_str())],
                ),
            );
            self.session_announced = true;
        }
        for (sequence, item) in events.iter_mut().enumerate() {
            item.sequence = u64::try_from(sequence).unwrap_or(u64::MAX);
            item.fields.insert("request_id".into(), request_id.clone());
        }
        TuiResponse {
            request_id,
            status,
            events,
        }
    }

    fn remember(&mut self, request: TuiRequest, response: &TuiResponse) {
        self.completed_requests.insert(
            request.id,
            CompletedRequest {
                command: request.command,
                response: response.clone(),
            },
        );
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApplicationServiceError {
    InvalidSessionId,
    InvalidRequestId,
    RequestIdConflict(String),
    RequestAlreadyCompleted(String),
    SelfCancellation(String),
    UnknownScenario(String),
    ScenarioNotSelected(String),
    InspectionUnavailable {
        scenario: String,
        inspection: &'static str,
    },
    Scenario(String),
}

impl fmt::Display for ApplicationServiceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "constitutional application service error: {self:?}"
        )
    }
}

impl std::error::Error for ApplicationServiceError {}

fn application_error(error: impl fmt::Display) -> ApplicationServiceError {
    ApplicationServiceError::Scenario(error.to_string())
}

fn descriptor(name: &str) -> Result<&'static ScenarioDescriptor, ApplicationServiceError> {
    SCENARIO_CATALOG
        .iter()
        .find(|item| item.name == name)
        .ok_or_else(|| ApplicationServiceError::UnknownScenario(name.to_owned()))
}

fn event<K, V, const N: usize>(scenario: &str, kind: TuiEventKind, fields: [(K, V); N]) -> TuiEvent
where
    K: Into<String>,
    V: Into<String>,
{
    TuiEvent {
        sequence: 0,
        scenario: scenario.to_owned(),
        kind,
        fields: fields
            .into_iter()
            .map(|(key, value)| (key.into(), value.into()))
            .collect(),
    }
}

fn bare_event(scenario: &str, kind: TuiEventKind) -> TuiEvent {
    TuiEvent {
        sequence: 0,
        scenario: scenario.to_owned(),
        kind,
        fields: BTreeMap::new(),
    }
}

const fn yes_no(value: bool) -> &'static str {
    if value { "Yes" } else { "No" }
}

const fn category_name(category: ScenarioCategory) -> &'static str {
    match category {
        ScenarioCategory::BondLifecycle => "Bond Lifecycle",
        ScenarioCategory::Polarity => "Polarity",
        ScenarioCategory::Failure => "Failure",
        ScenarioCategory::Persistence => "Persistence",
        ScenarioCategory::KernelAdapter => "Kernel Adapter",
        ScenarioCategory::RegionalSynthesis => "Regional Synthesis",
    }
}

const fn expectation_name(expectation: ScenarioExpectation) -> &'static str {
    match expectation {
        ScenarioExpectation::Accepted => "Accepted",
        ScenarioExpectation::Rejected => "Rejected",
    }
}

fn catalog_events() -> Vec<TuiEvent> {
    let mut events: Vec<_> = SCENARIO_CATALOG
        .iter()
        .map(|descriptor| {
            event(
                descriptor.name,
                TuiEventKind::CatalogEntry,
                [
                    ("name", descriptor.name),
                    ("category", category_name(descriptor.category)),
                    ("expectation", expectation_name(descriptor.expectation)),
                    ("summary", descriptor.summary),
                ],
            )
        })
        .collect();
    events.push(event(
        "catalog",
        TuiEventKind::CatalogCompleted,
        [("scenario_count", SCENARIO_CATALOG.len().to_string())],
    ));
    events
}

fn materialize_scenario(name: &str) -> Result<ScenarioSnapshot, ApplicationServiceError> {
    descriptor(name)?;
    match name {
        "ordinary-lifecycle" => bond_snapshot(run_ordinary_lifecycle().map_err(application_error)?),
        "default-challenge" => {
            bond_snapshot(run_default_challenge_scenario().map_err(application_error)?)
        }
        "positive-positive" => polarity_snapshot(Sign::Positive, Sign::Positive),
        "positive-negative" => polarity_snapshot(Sign::Positive, Sign::Negative),
        "negative-positive" => polarity_snapshot(Sign::Negative, Sign::Positive),
        "negative-negative" => polarity_snapshot(Sign::Negative, Sign::Negative),
        "premature-maturity" => {
            rejected_bond_snapshot(run_premature_maturity_scenario().map_err(application_error)?)
        }
        "renewal-after-terminal" => {
            rejected_bond_snapshot(run_terminal_renewal_rejection().map_err(application_error)?)
        }
        "gnome-minotaur" => {
            regional_snapshot(run_gnome_minotaur_scenario().map_err(application_error)?)
        }
        "elf-centaur" => regional_snapshot(run_elf_centaur_scenario().map_err(application_error)?),
        "minotaur-sea-claim" | "centaur-fields-claim" => rejected_assignment_snapshot(
            run_rejected_assignment_scenario(name).map_err(application_error)?,
        ),
        "kernel-wave" => kernel_wave_snapshot(),
        _ => rejected_regional_snapshot(
            run_rejected_regional_scenario(name).map_err(application_error)?,
        ),
    }
}

fn polarity_snapshot(
    current: Sign,
    aura: Sign,
) -> Result<ScenarioSnapshot, ApplicationServiceError> {
    bond_snapshot(run_polarity_scenario(current, aura).map_err(application_error)?)
}

fn bond_snapshot(scenario: BondScenario) -> Result<ScenarioSnapshot, ApplicationServiceError> {
    let trace = trace_bond_scenario(&scenario).map_err(application_error)?;
    let archive = encode_constitutional_archive(&scenario.runtime).map_err(application_error)?;
    Ok(ScenarioSnapshot {
        name: scenario.name.into(),
        trace,
        runtime: OwnedRuntime::Bond(scenario.runtime),
        archive,
        legacy_archive: None,
        regional_subject: None,
    })
}

fn rejected_bond_snapshot(
    scenario: RejectedBondScenario,
) -> Result<ScenarioSnapshot, ApplicationServiceError> {
    let trace = trace_rejected_bond_scenario(&scenario);
    let archive = encode_constitutional_archive(&scenario.runtime).map_err(application_error)?;
    Ok(ScenarioSnapshot {
        name: scenario.name.into(),
        trace,
        runtime: OwnedRuntime::Bond(scenario.runtime),
        archive,
        legacy_archive: None,
        regional_subject: None,
    })
}

fn regional_snapshot(
    scenario: RegionalScenario,
) -> Result<ScenarioSnapshot, ApplicationServiceError> {
    let trace = trace_regional_scenario(&scenario).map_err(application_error)?;
    let legacy_archive =
        encode_legacy_regional_archive_v0(&scenario.runtime).map_err(application_error)?;
    Ok(ScenarioSnapshot {
        name: scenario.name.into(),
        trace,
        archive: scenario.archive,
        legacy_archive: Some(legacy_archive),
        regional_subject: Some(scenario.result),
        runtime: OwnedRuntime::Regional(scenario.runtime),
    })
}

fn rejected_regional_snapshot(
    scenario: RejectedRegionalScenario,
) -> Result<ScenarioSnapshot, ApplicationServiceError> {
    let trace = trace_rejected_regional_scenario(&scenario);
    let archive = encode_regional_archive(&scenario.runtime).map_err(application_error)?;
    let legacy_archive =
        encode_legacy_regional_archive_v0(&scenario.runtime).map_err(application_error)?;
    Ok(ScenarioSnapshot {
        name: scenario.name.into(),
        trace,
        runtime: OwnedRuntime::Regional(scenario.runtime),
        archive,
        legacy_archive: Some(legacy_archive),
        regional_subject: Some(scenario.predecessor),
    })
}

fn rejected_assignment_snapshot(
    scenario: RejectedRegionalAssignmentScenario,
) -> Result<ScenarioSnapshot, ApplicationServiceError> {
    let trace = trace_rejected_assignment_scenario(&scenario);
    let archive = encode_regional_archive(&scenario.runtime).map_err(application_error)?;
    let legacy_archive =
        encode_legacy_regional_archive_v0(&scenario.runtime).map_err(application_error)?;
    Ok(ScenarioSnapshot {
        name: scenario.name.into(),
        trace,
        runtime: OwnedRuntime::Regional(scenario.runtime),
        archive,
        legacy_archive: Some(legacy_archive),
        regional_subject: Some(scenario.being),
    })
}

fn kernel_wave_snapshot() -> Result<ScenarioSnapshot, ApplicationServiceError> {
    let scenario = run_kernel_wave_scenario().map_err(application_error)?;
    let archive = encode_constitutional_archive(&scenario.runtime).map_err(application_error)?;
    let replayed = ConstitutionalRuntime::replay(
        scenario.runtime.waves().cloned(),
        scenario.runtime.events().iter().cloned(),
    )
    .map_err(application_error)?;
    let trace = ConstitutionalTrace {
        scenario: "kernel-wave".into(),
        transitions: vec![TransitionTrace {
            sequence: 0,
            event_id: scenario.wave.as_str().into(),
            command: "CREATE_WAVE",
            prior_state: "Completed Kernel Pass".into(),
            proposed_state: "Wave Recorded / Current Unmoved".into(),
            acting_house: None,
            acting_institution: None,
            authority: vec![],
            evidence: vec![],
            current_polarity: None,
            aura_polarity: None,
            region: None,
            predecessor_being: None,
            resulting_being: None,
            synthesis_rule: None,
            regional_function: None,
            stewardship: vec![],
            guardianship: vec![],
            prevalidation_digest: None,
            disposition: TraceDisposition::Accepted,
            failure_code: None,
            state_changed: true,
        }],
        live_replay_equivalent: encode_constitutional_archive(&replayed)
            .map_err(application_error)?
            == archive,
        canonical_persistence: decode_constitutional_archive(&archive)
            .and_then(|decoded| encode_constitutional_archive(&decoded))
            .is_ok_and(|encoded| encoded == archive),
        archive_digest: Some(
            constitutional_replay_digest(&scenario.runtime).map_err(application_error)?,
        ),
    };
    Ok(ScenarioSnapshot {
        name: "kernel-wave".into(),
        trace,
        runtime: OwnedRuntime::Bond(scenario.runtime),
        archive,
        legacy_archive: None,
        regional_subject: None,
    })
}

fn runtime_event_count(snapshot: &ScenarioSnapshot) -> usize {
    match &snapshot.runtime {
        OwnedRuntime::Bond(runtime) => runtime.events().len(),
        OwnedRuntime::Regional(runtime) => runtime.events().len(),
    }
}

fn inspect_authority(
    snapshot: &ScenarioSnapshot,
) -> Result<Vec<TuiEvent>, ApplicationServiceError> {
    let events: Vec<_> = snapshot
        .trace
        .transitions
        .iter()
        .filter(|transition| {
            transition.acting_house.is_some()
                || transition.acting_institution.is_some()
                || !transition.authority.is_empty()
        })
        .map(|transition| {
            event(
                &snapshot.name,
                TuiEventKind::AuthorityChecked,
                [
                    ("command", transition.command.to_owned()),
                    (
                        "house",
                        transition
                            .acting_house
                            .map(house_name)
                            .unwrap_or("None")
                            .to_owned(),
                    ),
                    (
                        "institution",
                        transition
                            .acting_institution
                            .clone()
                            .unwrap_or_else(|| "None".into()),
                    ),
                    ("authority", transition.authority.join("; ")),
                ],
            )
        })
        .collect();
    require_inspection(snapshot, "authority", events)
}

fn inspect_evidence(snapshot: &ScenarioSnapshot) -> Result<Vec<TuiEvent>, ApplicationServiceError> {
    let events: Vec<_> = snapshot
        .trace
        .transitions
        .iter()
        .filter(|transition| !transition.evidence.is_empty())
        .map(|transition| {
            event(
                &snapshot.name,
                TuiEventKind::EvidenceChecked,
                [
                    ("command", transition.command.to_owned()),
                    ("evidence", transition.evidence.join("; ")),
                ],
            )
        })
        .collect();
    require_inspection(snapshot, "evidence", events)
}

fn inspect_polarity(snapshot: &ScenarioSnapshot) -> Result<Vec<TuiEvent>, ApplicationServiceError> {
    let events: Vec<_> = snapshot
        .trace
        .transitions
        .iter()
        .filter_map(|transition| {
            Some(event(
                &snapshot.name,
                TuiEventKind::PolarityObserved,
                [
                    (
                        "current",
                        sign_name(transition.current_polarity?).to_owned(),
                    ),
                    ("aura", sign_name(transition.aura_polarity?).to_owned()),
                    ("command", transition.command.to_owned()),
                ],
            ))
        })
        .collect();
    require_inspection(snapshot, "polarity", events)
}

fn regional_runtime<'a>(
    snapshot: &'a ScenarioSnapshot,
    inspection: &'static str,
) -> Result<(&'a RegionalSynthesisRuntime, &'a RegionalBeingId), ApplicationServiceError> {
    let OwnedRuntime::Regional(runtime) = &snapshot.runtime else {
        return Err(ApplicationServiceError::InspectionUnavailable {
            scenario: snapshot.name.clone(),
            inspection,
        });
    };
    let subject = snapshot.regional_subject.as_ref().ok_or_else(|| {
        ApplicationServiceError::InspectionUnavailable {
            scenario: snapshot.name.clone(),
            inspection,
        }
    })?;
    Ok((runtime, subject))
}

fn inspect_lineage(snapshot: &ScenarioSnapshot) -> Result<Vec<TuiEvent>, ApplicationServiceError> {
    let (runtime, subject) = regional_runtime(snapshot, "lineage")?;
    let lineage =
        runtime
            .lineage(subject)
            .ok_or_else(|| ApplicationServiceError::InspectionUnavailable {
                scenario: snapshot.name.clone(),
                inspection: "lineage",
            })?;
    Ok(lineage
        .iter()
        .enumerate()
        .map(|(index, entry)| {
            event(
                &snapshot.name,
                TuiEventKind::LineagePreserved,
                [
                    ("index", index.to_string()),
                    ("being", entry.being.as_str().to_owned()),
                    ("form", sandmanor_form_name(entry.form).to_owned()),
                    (
                        "synthesis",
                        entry
                            .synthesis
                            .as_ref()
                            .map_or("Origin", RegionalSynthesisId::as_str)
                            .to_owned(),
                    ),
                ],
            )
        })
        .collect())
}

fn inspect_region(snapshot: &ScenarioSnapshot) -> Result<Vec<TuiEvent>, ApplicationServiceError> {
    let (runtime, subject) = regional_runtime(snapshot, "region")?;
    let being =
        runtime
            .being(subject)
            .ok_or_else(|| ApplicationServiceError::InspectionUnavailable {
                scenario: snapshot.name.clone(),
                inspection: "region",
            })?;
    Ok(vec![event(
        &snapshot.name,
        TuiEventKind::RegionEntered,
        [
            ("being", being.id.as_str().to_owned()),
            ("form", sandmanor_form_name(being.form).to_owned()),
            ("region", being.standing.region.as_str().to_owned()),
            ("standing", format!("{:?}", being.standing.kind)),
        ],
    )])
}

fn inspect_stewardship(
    snapshot: &ScenarioSnapshot,
) -> Result<Vec<TuiEvent>, ApplicationServiceError> {
    let (runtime, subject) = regional_runtime(snapshot, "stewardship")?;
    let stewardship = runtime
        .require_stewardship(subject)
        .map_err(application_error)?;
    Ok(vec![event(
        &snapshot.name,
        TuiEventKind::StewardshipGranted,
        [
            ("holder", stewardship.steward.as_str().to_owned()),
            ("region", stewardship.region.as_str().to_owned()),
            (
                "duties",
                stewardship
                    .duties
                    .iter()
                    .map(|duty| duty.as_str())
                    .collect::<Vec<_>>()
                    .join("; "),
            ),
            (
                "authority_decision",
                stewardship.authority.as_str().to_owned(),
            ),
        ],
    )])
}

fn inspect_guardianship(
    snapshot: &ScenarioSnapshot,
) -> Result<Vec<TuiEvent>, ApplicationServiceError> {
    let (runtime, subject) = regional_runtime(snapshot, "guardianship")?;
    let guardianship = runtime
        .require_guardianship(subject)
        .map_err(application_error)?;
    Ok(vec![event(
        &snapshot.name,
        TuiEventKind::GuardianshipGranted,
        [
            ("holder", guardianship.guardian.as_str().to_owned()),
            ("region", guardianship.region.as_str().to_owned()),
            (
                "duties",
                guardianship
                    .duties
                    .iter()
                    .map(|duty| duty.as_str())
                    .collect::<Vec<_>>()
                    .join("; "),
            ),
            (
                "authority_decision",
                guardianship.authority.as_str().to_owned(),
            ),
        ],
    )])
}

fn require_inspection(
    snapshot: &ScenarioSnapshot,
    inspection: &'static str,
    events: Vec<TuiEvent>,
) -> Result<Vec<TuiEvent>, ApplicationServiceError> {
    if events.is_empty() {
        Err(ApplicationServiceError::InspectionUnavailable {
            scenario: snapshot.name.clone(),
            inspection,
        })
    } else {
        Ok(events)
    }
}

fn audit_events() -> Result<Vec<TuiEvent>, ApplicationServiceError> {
    let mut accepted = 0_usize;
    let mut rejected = 0_usize;
    let mut replay_equivalent = true;
    let mut persistence_canonical = true;
    for descriptor in SCENARIO_CATALOG {
        let snapshot = materialize_scenario(descriptor.name)?;
        let has_rejection = snapshot
            .trace
            .transitions
            .iter()
            .any(|transition| transition.disposition == TraceDisposition::Rejected);
        match descriptor.expectation {
            ScenarioExpectation::Accepted if !has_rejection => accepted += 1,
            ScenarioExpectation::Rejected if has_rejection => rejected += 1,
            _ => {
                return Err(ApplicationServiceError::Scenario(format!(
                    "scenario {} violated its catalog expectation",
                    descriptor.name
                )));
            }
        }
        replay_equivalent &= snapshot.replay_equivalent()?;
        persistence_canonical &= snapshot.persisted_canonically()?;
    }
    Ok(vec![event(
        "audit",
        TuiEventKind::AuditCompleted,
        [
            ("scenario_count", SCENARIO_CATALOG.len().to_string()),
            ("accepted", accepted.to_string()),
            ("rejected", rejected.to_string()),
            ("replay_equivalent", yes_no(replay_equivalent).to_owned()),
            (
                "persistence_canonical",
                yes_no(persistence_canonical).to_owned(),
            ),
        ],
    )])
}

const fn house_name(house: crate::hollow_grove_contract::House) -> &'static str {
    match house {
        crate::hollow_grove_contract::House::Stonebend => "Stonebend",
        crate::hollow_grove_contract::House::Sandmanor => "Sandmanor",
        crate::hollow_grove_contract::House::Flynt => "Flynt",
        crate::hollow_grove_contract::House::Glaushouse => "Glaüshouse",
    }
}

const fn sign_name(sign: Sign) -> &'static str {
    match sign {
        Sign::Positive => "Positive",
        Sign::Negative => "Negative",
    }
}
