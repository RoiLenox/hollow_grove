use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use serde::{Deserialize, Serialize};

use crate::composition::ExternalRef;
use crate::constitutional::{CausalPosition, ParticipantId, RuleSetId};
use crate::guide_identity::{
    DualGuideSession, GuideTransitionDirection, GuideTransitionEvidence, PairedGuideTransition,
    StableGuideSessionId, StableTransitionId, WorldLayer,
};
use crate::guide_state_machine::{
    CanonicalGuideSession, DurableGuideTransitionCoordinator, DurableGuideTransitionOutcome,
    GuideTransitionEvent, GuideTransitionJournalStatus, GuideTransitionJournalStore,
};
use crate::institution::InstitutionalBeingId;
use crate::kernel_invariants::{audit_gameplay_federation_binding, audit_guide_context};
use crate::world::session::WorldSession;
use crate::{read_text_artifact, write_text_artifact};

use super::{
    BeingContinuityId, BoardwalkChoice, CardinalDirection, DeepPressurePhase,
    DeepPressureSettlementChoice, GameApplicationService, GameView, GameplayCommand,
    GameplayEventId, GameplayEventMetadata, GameplayEventView, HollowGroveGameRuntime,
    HuemanFaculty, IntentCapabilityView, KernelAuditView, LivingCaseChoice, LivingCaseId,
    MAX_PARTY_MEMBERS, PartyActionId, PartyActorId, PartyMemberAvailability,
    RecruitmentCandidateId, RecruitmentPath, StonebendContinuityChoice, WorldMapId,
};

pub const HOLLOW_GROVE_GAMEPLAY_PROTOCOL_VERSION: u16 = 1;
pub const DEFAULT_GAMEPLAY_SERVICE_ADDRESS: &str = "127.0.0.1:47819";
pub const MAX_GAMEPLAY_MESSAGE_BYTES: usize = 64 * 1024;
/// Maximum encoded response sent by line-oriented transports.
pub const MAX_GAMEPLAY_RESPONSE_BYTES: usize = 64 * 1024;
/// Recent responses retained for byte-for-byte retry handling.
pub const MAX_CACHED_PROTOCOL_RESPONSES: usize = 256;
/// Session-wide request IDs retained to prevent an evicted retry from running twice.
pub const MAX_TRACKED_PROTOCOL_REQUESTS: usize = 16_384;
pub const GAMEPLAY_SERVICE_SAVE_SCHEMA_VERSION: u16 = 1;
pub const GUIDE_OPERATIONAL_STATE_SCHEMA_VERSION: &str = "hollow-grove-guide-operational-state-v1";

static RUNTIME_LEASE_NONCE: AtomicU64 = AtomicU64::new(0);

/// Version-one player intents. Schema presence does not imply capability.
/// Availability is projected by the runtime and unavailable intents fail
/// explicitly without mutating state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", deny_unknown_fields)]
pub enum GameplayIntent {
    SyncIntent,
    EnterHollowGroveIntent,
    LeaveHollowGroveIntent,
    EstablishHuemanIntent {
        continuity_id: String,
        participant_id: String,
        institutional_being_id: String,
    },
    MoveIntent {
        direction: CardinalDirection,
    },
    InteractIntent,
    OpenPartyIntent,
    SelectPartyMemberIntent {
        continuity_id: String,
    },
    UseActionIntent {
        actor_continuity_id: String,
        action_id: String,
        target_continuity_id: Option<String>,
    },
    RecruitIntent {
        target_id: String,
        recruitment_path: String,
    },
    SwitchLeadIntent {
        continuity_id: String,
    },
    EnterRegionIntent {
        region_id: String,
    },
    TraverseExitIntent {
        destination_map_id: String,
    },
    AdvanceWorldShiftIntent,
    SupportLivingCaseOptionIntent {
        case_id: LivingCaseId,
        choice: LivingCaseChoice,
    },
    AskLivingDutyOfficerToDecideIntent {
        case_id: LivingCaseId,
    },
    SupportDeepPressureSettlementIntent {
        choice: DeepPressureSettlementChoice,
    },
    AskDeepPressureAssemblyToCommitIntent,
    DiscloseFacultyObservationIntent {
        faculty: HuemanFaculty,
    },
    SupportBoardwalkOptionIntent {
        choice: BoardwalkChoice,
    },
    AskReturningGoonToDecideIntent,
    SupportStonebendContinuityOptionIntent {
        choice: StonebendContinuityChoice,
    },
    AskStonebendToDetermineContinuityIntent,
    BeginSynthesisIntent {
        continuity_id: String,
        procedure_id: String,
    },
    SaveIntent {
        slot_id: String,
    },
    LoadIntent {
        slot_id: String,
    },
}

impl GameplayIntent {
    #[must_use]
    pub const fn kind(&self) -> &'static str {
        match self {
            Self::SyncIntent => "SyncIntent",
            Self::EnterHollowGroveIntent => "EnterHollowGroveIntent",
            Self::LeaveHollowGroveIntent => "LeaveHollowGroveIntent",
            Self::EstablishHuemanIntent { .. } => "EstablishHuemanIntent",
            Self::MoveIntent { .. } => "MoveIntent",
            Self::InteractIntent => "InteractIntent",
            Self::OpenPartyIntent => "OpenPartyIntent",
            Self::SelectPartyMemberIntent { .. } => "SelectPartyMemberIntent",
            Self::UseActionIntent { .. } => "UseActionIntent",
            Self::RecruitIntent { .. } => "RecruitIntent",
            Self::SwitchLeadIntent { .. } => "SwitchLeadIntent",
            Self::EnterRegionIntent { .. } => "EnterRegionIntent",
            Self::TraverseExitIntent { .. } => "TraverseExitIntent",
            Self::AdvanceWorldShiftIntent => "AdvanceWorldShiftIntent",
            Self::SupportLivingCaseOptionIntent { .. } => "SupportLivingCaseOptionIntent",
            Self::AskLivingDutyOfficerToDecideIntent { .. } => "AskLivingDutyOfficerToDecideIntent",
            Self::SupportDeepPressureSettlementIntent { .. } => {
                "SupportDeepPressureSettlementIntent"
            }
            Self::AskDeepPressureAssemblyToCommitIntent => "AskDeepPressureAssemblyToCommitIntent",
            Self::DiscloseFacultyObservationIntent { .. } => "DiscloseFacultyObservationIntent",
            Self::SupportBoardwalkOptionIntent { .. } => "SupportBoardwalkOptionIntent",
            Self::AskReturningGoonToDecideIntent => "AskReturningGoonToDecideIntent",
            Self::SupportStonebendContinuityOptionIntent { .. } => {
                "SupportStonebendContinuityOptionIntent"
            }
            Self::AskStonebendToDetermineContinuityIntent => {
                "AskStonebendToDetermineContinuityIntent"
            }
            Self::BeginSynthesisIntent { .. } => "BeginSynthesisIntent",
            Self::SaveIntent { .. } => "SaveIntent",
            Self::LoadIntent { .. } => "LoadIntent",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProtocolRequestEnvelope {
    pub protocol_version: u16,
    pub session_id: String,
    pub request_id: String,
    pub expected_revision: u64,
    pub intent: GameplayIntent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProtocolResponseStatus {
    Completed,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProtocolRejectionCode {
    UnsupportedProtocolVersion,
    InvalidSession,
    InvalidRequestId,
    RequestIdConflict,
    RequestReplayExpired,
    RequestLedgerFull,
    StaleRevision,
    CapabilityUnavailable,
    MalformedRequest,
    RuntimeRejected,
    ResponseTooLarge,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProtocolRejection {
    pub code: ProtocolRejectionCode,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProtocolResponseEnvelope {
    pub protocol_version: u16,
    pub session_id: String,
    pub request_id: String,
    pub status: ProtocolResponseStatus,
    pub revision: u64,
    pub events: Vec<GameplayEventView>,
    pub view: Option<GameView>,
    pub rejection: Option<ProtocolRejection>,
}

#[derive(Debug, Clone)]
struct CompletedProtocolRequest {
    request: ProtocolRequestEnvelope,
    response: ProtocolResponseEnvelope,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct GameplayServiceSaveArchive {
    service_schema_version: u16,
    service_revision: u64,
    gameplay_archive: String,
    guide_session: DualGuideSession,
    #[serde(default)]
    applied_guide_transitions: BTreeSet<StableTransitionId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct GuideOperationalStateArchive {
    schema_version: String,
    session_id: String,
    service_revision: u64,
    guide_session: DualGuideSession,
    #[serde(default)]
    applied_guide_transitions: BTreeSet<StableTransitionId>,
}

#[derive(Debug)]
struct GameplayRuntimeLease {
    lock_path: PathBuf,
    claim_path: PathBuf,
    claim_name: String,
}

impl GameplayRuntimeLease {
    fn acquire(root: &Path, session_id: &str) -> Result<Self, ProtocolServiceError> {
        fs::create_dir_all(root)
            .map_err(|error| ProtocolServiceError::OperationalState(error.to_string()))?;
        let lock_path = gameplay_runtime_lease_path(root, session_id);
        for _ in 0..3 {
            let nonce = RUNTIME_LEASE_NONCE.fetch_add(1, Ordering::Relaxed);
            let pid = std::process::id();
            let start = process_start_token(pid).unwrap_or(0);
            let claim_name = format!(".{session_id}.runtime-lease.{pid}.{start}.{nonce}.claim");
            let claim_path = root.join(&claim_name);
            let mut claim = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&claim_path)
                .map_err(|error| ProtocolServiceError::RuntimeLease(error.to_string()))?;
            claim
                .write_all(claim_name.as_bytes())
                .and_then(|()| claim.sync_all())
                .map_err(|error| ProtocolServiceError::RuntimeLease(error.to_string()))?;
            drop(claim);
            match fs::hard_link(&claim_path, &lock_path) {
                Ok(()) => {
                    if let Err(error) =
                        fs::File::open(root).and_then(|directory| directory.sync_all())
                    {
                        let _ = fs::remove_file(&lock_path);
                        let _ = fs::remove_file(&claim_path);
                        return Err(ProtocolServiceError::RuntimeLease(error.to_string()));
                    }
                    return Ok(Self {
                        lock_path,
                        claim_path,
                        claim_name,
                    });
                }
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                    let _ = fs::remove_file(&claim_path);
                    let owner = fs::read_to_string(&lock_path)
                        .map_err(|error| ProtocolServiceError::RuntimeLease(error.to_string()))?;
                    if runtime_lease_owner_is_active(owner.trim()) {
                        return Err(ProtocolServiceError::RuntimeLease(format!(
                            "session {session_id} already has an active gameplay service"
                        )));
                    }
                    let stale_claim = root.join(owner.trim());
                    fs::remove_file(&lock_path)
                        .map_err(|error| ProtocolServiceError::RuntimeLease(error.to_string()))?;
                    let _ = fs::remove_file(stale_claim);
                }
                Err(error) => {
                    let _ = fs::remove_file(&claim_path);
                    return Err(ProtocolServiceError::RuntimeLease(error.to_string()));
                }
            }
        }
        Err(ProtocolServiceError::RuntimeLease(format!(
            "could not acquire the gameplay service lease for {session_id}"
        )))
    }
}

impl Drop for GameplayRuntimeLease {
    fn drop(&mut self) {
        if fs::read_to_string(&self.lock_path).is_ok_and(|owner| owner.trim() == self.claim_name) {
            let _ = fs::remove_file(&self.lock_path);
        }
        let _ = fs::remove_file(&self.claim_path);
    }
}

/// Stateful application boundary used by transport adapters.
///
/// It owns request idempotency and revision checks. Only accepted intents are
/// translated into authoritative gameplay commands.
#[derive(Debug)]
pub struct GameProtocolService {
    session_id: String,
    application: GameApplicationService,
    guide_session: DualGuideSession,
    guide_coordinator: Option<DurableGuideTransitionCoordinator>,
    pending_guide_events: Vec<GuideTransitionEvent>,
    applied_guide_transitions: BTreeSet<StableTransitionId>,
    operational_revision: u64,
    completed_requests: BTreeMap<String, CompletedProtocolRequest>,
    completed_request_order: VecDeque<String>,
    tracked_requests: BTreeMap<String, ProtocolRequestEnvelope>,
    save_root: Option<PathBuf>,
    _runtime_lease: Option<GameplayRuntimeLease>,
}

impl GameProtocolService {
    pub fn new(
        session_id: impl Into<String>,
        rule_set: RuleSetId,
    ) -> Result<Self, ProtocolServiceError> {
        let session_id = session_id.into();
        if !is_stable_wire_id(&session_id) {
            return Err(ProtocolServiceError::InvalidSessionId(session_id));
        }
        let guide_session_id = StableGuideSessionId::new(session_id.clone())
            .map_err(|error| ProtocolServiceError::OperationalState(error.to_string()))?;
        Ok(Self {
            session_id,
            application: GameApplicationService::new(rule_set),
            guide_session: DualGuideSession::inside(guide_session_id),
            guide_coordinator: None,
            pending_guide_events: Vec::new(),
            applied_guide_transitions: BTreeSet::new(),
            operational_revision: 0,
            completed_requests: BTreeMap::new(),
            completed_request_order: VecDeque::new(),
            tracked_requests: BTreeMap::new(),
            save_root: None,
            _runtime_lease: None,
        })
    }

    pub fn new_with_save_root(
        session_id: impl Into<String>,
        rule_set: RuleSetId,
        save_root: impl Into<PathBuf>,
    ) -> Result<Self, ProtocolServiceError> {
        let mut service = Self::new(session_id, rule_set)?;
        let save_root = save_root.into();
        service.guide_coordinator = Some(DurableGuideTransitionCoordinator::new(
            GuideTransitionJournalStore::new(guide_transition_journal_root(
                &save_root,
                &service.session_id,
            )),
        ));
        service.save_root = Some(save_root);
        Ok(service)
    }

    pub fn new_with_roots(
        session_id: impl Into<String>,
        rule_set: RuleSetId,
        save_root: impl Into<PathBuf>,
        world_root: impl AsRef<Path>,
    ) -> Result<Self, ProtocolServiceError> {
        let session_id = session_id.into();
        if !is_stable_wire_id(&session_id) {
            return Err(ProtocolServiceError::InvalidSessionId(session_id));
        }
        let save_root = save_root.into();
        let runtime_lease = GameplayRuntimeLease::acquire(&save_root, &session_id)?;
        let world = WorldSession::load_or_canonical_at(world_root.as_ref())
            .map_err(|error| ProtocolServiceError::InstitutionalState(error.to_string()))?;
        let guide_session_id = StableGuideSessionId::new(session_id.clone())
            .map_err(|error| ProtocolServiceError::OperationalState(error.to_string()))?;
        let state_path = guide_operational_state_path(&save_root, &session_id);
        let (guide_session, operational_revision, applied_guide_transitions) =
            if state_path.exists() {
                load_guide_operational_state(&state_path, &session_id, &guide_session_id)?
            } else {
                (
                    DualGuideSession::outside(guide_session_id),
                    0,
                    BTreeSet::new(),
                )
            };
        let coordinator = DurableGuideTransitionCoordinator::new(GuideTransitionJournalStore::new(
            guide_transition_journal_root(&save_root, &session_id),
        ));
        let mut service = Self {
            session_id,
            application: GameApplicationService::with_world_session(rule_set, world),
            guide_session,
            guide_coordinator: Some(coordinator),
            pending_guide_events: Vec::new(),
            applied_guide_transitions,
            operational_revision,
            completed_requests: BTreeMap::new(),
            completed_request_order: VecDeque::new(),
            tracked_requests: BTreeMap::new(),
            save_root: Some(save_root),
            _runtime_lease: Some(runtime_lease),
        };
        service.recover_guide_transitions()?;
        service.persist_guide_operational_state()?;
        Ok(service)
    }

    #[must_use]
    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    #[must_use]
    pub fn revision(&self) -> u64 {
        self.operational_revision
    }

    #[must_use]
    pub const fn application(&self) -> &GameApplicationService {
        &self.application
    }

    #[must_use]
    pub const fn guide_session(&self) -> &DualGuideSession {
        &self.guide_session
    }

    #[must_use]
    pub fn cached_response_count(&self) -> usize {
        self.completed_requests.len()
    }

    #[must_use]
    pub fn tracked_request_count(&self) -> usize {
        self.tracked_requests.len()
    }

    #[must_use]
    pub fn handle(&mut self, request: ProtocolRequestEnvelope) -> ProtocolResponseEnvelope {
        if request.protocol_version != HOLLOW_GROVE_GAMEPLAY_PROTOCOL_VERSION {
            return self.rejection(
                request.request_id,
                ProtocolRejectionCode::UnsupportedProtocolVersion,
                format!(
                    "protocol version {} is unsupported; expected {}",
                    request.protocol_version, HOLLOW_GROVE_GAMEPLAY_PROTOCOL_VERSION
                ),
                false,
            );
        }
        if request.session_id != self.session_id {
            return self.rejection(
                request.request_id,
                ProtocolRejectionCode::InvalidSession,
                "request session does not match the authoritative runtime session".into(),
                false,
            );
        }
        if !is_stable_wire_id(&request.request_id) {
            return self.rejection(
                request.request_id,
                ProtocolRejectionCode::InvalidRequestId,
                "request ID must be a nonempty lowercase stable identifier".into(),
                false,
            );
        }
        if let Some(completed) = self.completed_requests.get(&request.request_id) {
            return if completed.request == request {
                completed.response.clone()
            } else {
                self.rejection(
                    request.request_id,
                    ProtocolRejectionCode::RequestIdConflict,
                    "request ID was already used for a different request".into(),
                    true,
                )
            };
        }
        if let Some(completed) = self.tracked_requests.get(&request.request_id) {
            return if completed == &request {
                self.rejection(
                    request.request_id,
                    ProtocolRejectionCode::RequestReplayExpired,
                    "the exact request was completed, but its response aged out; synchronize with a new request ID"
                        .into(),
                    true,
                )
            } else {
                self.rejection(
                    request.request_id,
                    ProtocolRejectionCode::RequestIdConflict,
                    "request ID was already used for a different request".into(),
                    true,
                )
            };
        }
        if self.tracked_requests.len() >= MAX_TRACKED_PROTOCOL_REQUESTS {
            return self.rejection(
                request.request_id,
                ProtocolRejectionCode::RequestLedgerFull,
                format!(
                    "this session reached its {} request safety limit; save and restart the service before issuing more intents",
                    MAX_TRACKED_PROTOCOL_REQUESTS
                ),
                true,
            );
        }

        let response = if !matches!(request.intent, GameplayIntent::SyncIntent)
            && request.expected_revision != self.revision()
        {
            self.rejection(
                request.request_id.clone(),
                ProtocolRejectionCode::StaleRevision,
                format!(
                    "expected revision {} does not match current revision {}",
                    request.expected_revision,
                    self.revision()
                ),
                true,
            )
        } else {
            self.dispatch(&request)
        };
        let request_id = request.request_id.clone();
        self.tracked_requests
            .insert(request_id.clone(), request.clone());
        self.completed_requests.insert(
            request_id.clone(),
            CompletedProtocolRequest {
                request,
                response: response.clone(),
            },
        );
        self.completed_request_order.push_back(request_id);
        while self.completed_request_order.len() > MAX_CACHED_PROTOCOL_RESPONSES {
            if let Some(expired) = self.completed_request_order.pop_front() {
                self.completed_requests.remove(&expired);
            }
        }
        response
    }

    fn dispatch(&mut self, request: &ProtocolRequestEnvelope) -> ProtocolResponseEnvelope {
        if self.guide_session.current_layer != WorldLayer::HollowGrove
            && intent_requires_hollow_grove(&request.intent)
        {
            return self.rejection(
                request.request_id.clone(),
                ProtocolRejectionCode::CapabilityUnavailable,
                format!(
                    "{} requires Roï and Rosaliña to enter Hollow Grove first",
                    request.intent.kind()
                ),
                true,
            );
        }
        match &request.intent {
            GameplayIntent::SyncIntent => {
                let guide_events = std::mem::take(&mut self.pending_guide_events);
                self.completed_with_guide_events(
                    request.request_id.clone(),
                    vec![GameplayEventView::snapshot_loaded(
                        self.application.revision(),
                    )],
                    guide_events,
                )
            }
            GameplayIntent::EnterHollowGroveIntent => {
                self.transition_guides(request, GuideTransitionDirection::EnterHollowGrove)
            }
            GameplayIntent::LeaveHollowGroveIntent => {
                self.transition_guides(request, GuideTransitionDirection::LeaveHollowGrove)
            }
            GameplayIntent::EstablishHuemanIntent {
                continuity_id,
                participant_id,
                institutional_being_id,
            } => {
                let command = BeingContinuityId::new(continuity_id.clone())
                    .map_err(|error| error.to_string())
                    .and_then(|continuity| {
                        ParticipantId::new(participant_id.clone())
                            .map_err(|error| error.to_string())
                            .map(|participant| (continuity, participant))
                    })
                    .and_then(|(continuity, participant)| {
                        InstitutionalBeingId::new(institutional_being_id.clone())
                            .map_err(|error| error.to_string())
                            .map(|institutional| (continuity, participant, institutional))
                    });
                let (continuity, participant, institutional) = match command {
                    Ok(value) => value,
                    Err(message) => {
                        return self.rejection(
                            request.request_id.clone(),
                            ProtocolRejectionCode::RuntimeRejected,
                            message,
                            true,
                        );
                    }
                };
                if self.operational_revision == u64::MAX {
                    return self.rejection(
                        request.request_id.clone(),
                        ProtocolRejectionCode::RuntimeRejected,
                        "gameplay service revision overflowed".into(),
                        true,
                    );
                }
                let causal = match self.application.revision().get().checked_add(1) {
                    Some(value) => value,
                    None => {
                        return self.rejection(
                            request.request_id.clone(),
                            ProtocolRejectionCode::RuntimeRejected,
                            "gameplay causal position overflowed".into(),
                            true,
                        );
                    }
                };
                let event_id = match GameplayEventId::new(format!(
                    "game-event.{}",
                    request.request_id
                )) {
                    Ok(value) => value,
                    Err(error) => {
                        return self.rejection(
                            request.request_id.clone(),
                            ProtocolRejectionCode::RuntimeRejected,
                            error.to_string(),
                            true,
                        );
                    }
                };
                let projected = self
                    .application
                    .execute(
                        GameplayEventMetadata {
                            id: event_id,
                            causal_position: CausalPosition::new(causal),
                        },
                        GameplayCommand::EstablishHuemanIdentity {
                            continuity,
                            participant,
                            institutional,
                        },
                    )
                    .map(GameplayEventView::from_canonical)
                    .map_err(|error| error.to_string());
                match projected {
                    Ok(event) => {
                        self.operational_revision += 1;
                        self.completed(request.request_id.clone(), vec![event])
                    }
                    Err(message) => self.rejection(
                        request.request_id.clone(),
                        ProtocolRejectionCode::RuntimeRejected,
                        message,
                        true,
                    ),
                }
            }
            GameplayIntent::MoveIntent { direction } => {
                if self.application.hueman().is_none() {
                    return self.rejection(
                        request.request_id.clone(),
                        ProtocolRejectionCode::CapabilityUnavailable,
                        "MoveIntent requires the permanent Hueman identity".into(),
                        true,
                    );
                }
                if self.operational_revision == u64::MAX {
                    return self.rejection(
                        request.request_id.clone(),
                        ProtocolRejectionCode::RuntimeRejected,
                        "gameplay service revision overflowed".into(),
                        true,
                    );
                }
                let causal = match self.application.revision().get().checked_add(1) {
                    Some(value) => value,
                    None => {
                        return self.rejection(
                            request.request_id.clone(),
                            ProtocolRejectionCode::RuntimeRejected,
                            "gameplay causal position overflowed".into(),
                            true,
                        );
                    }
                };
                let event_id = match GameplayEventId::new(format!(
                    "game-event.{}",
                    request.request_id
                )) {
                    Ok(value) => value,
                    Err(error) => {
                        return self.rejection(
                            request.request_id.clone(),
                            ProtocolRejectionCode::RuntimeRejected,
                            error.to_string(),
                            true,
                        );
                    }
                };
                let projected = self
                    .application
                    .execute(
                        GameplayEventMetadata {
                            id: event_id,
                            causal_position: CausalPosition::new(causal),
                        },
                        GameplayCommand::MoveHueman {
                            direction: *direction,
                        },
                    )
                    .map(GameplayEventView::from_canonical)
                    .map_err(|error| error.to_string());
                match projected {
                    Ok(event) => {
                        self.operational_revision += 1;
                        self.completed(request.request_id.clone(), vec![event])
                    }
                    Err(message) => self.rejection(
                        request.request_id.clone(),
                        ProtocolRejectionCode::RuntimeRejected,
                        message,
                        true,
                    ),
                }
            }
            GameplayIntent::InteractIntent => {
                if self.application.hueman().is_none() {
                    return self.rejection(
                        request.request_id.clone(),
                        ProtocolRejectionCode::CapabilityUnavailable,
                        "InteractIntent requires the permanent Hueman identity".into(),
                        true,
                    );
                }
                if self.operational_revision == u64::MAX {
                    return self.rejection(
                        request.request_id.clone(),
                        ProtocolRejectionCode::RuntimeRejected,
                        "gameplay service revision overflowed".into(),
                        true,
                    );
                }
                let causal = match self.application.revision().get().checked_add(1) {
                    Some(value) => value,
                    None => {
                        return self.rejection(
                            request.request_id.clone(),
                            ProtocolRejectionCode::RuntimeRejected,
                            "gameplay causal position overflowed".into(),
                            true,
                        );
                    }
                };
                let event_id = match GameplayEventId::new(format!(
                    "game-event.{}",
                    request.request_id
                )) {
                    Ok(value) => value,
                    Err(error) => {
                        return self.rejection(
                            request.request_id.clone(),
                            ProtocolRejectionCode::RuntimeRejected,
                            error.to_string(),
                            true,
                        );
                    }
                };
                let projected = self
                    .application
                    .execute(
                        GameplayEventMetadata {
                            id: event_id,
                            causal_position: CausalPosition::new(causal),
                        },
                        GameplayCommand::InteractHueman,
                    )
                    .map(GameplayEventView::from_canonical)
                    .map_err(|error| error.to_string());
                match projected {
                    Ok(event) => {
                        self.operational_revision += 1;
                        self.completed(request.request_id.clone(), vec![event])
                    }
                    Err(message) => self.rejection(
                        request.request_id.clone(),
                        ProtocolRejectionCode::RuntimeRejected,
                        message,
                        true,
                    ),
                }
            }
            GameplayIntent::OpenPartyIntent => {
                if self.application.hueman().is_none() {
                    self.rejection(
                        request.request_id.clone(),
                        ProtocolRejectionCode::CapabilityUnavailable,
                        "OpenPartyIntent requires the permanent Hueman identity".into(),
                        true,
                    )
                } else {
                    self.completed(request.request_id.clone(), Vec::new())
                }
            }
            GameplayIntent::SelectPartyMemberIntent { continuity_id } => {
                let Some(actor) = self
                    .application
                    .runtime()
                    .party()
                    .actor_from_continuity_id(continuity_id)
                else {
                    return self.rejection(
                        request.request_id.clone(),
                        ProtocolRejectionCode::RuntimeRejected,
                        "unknown party continuity ID".into(),
                        true,
                    );
                };
                self.execute_command(request, GameplayCommand::SelectPartyMember { actor })
            }
            GameplayIntent::RecruitIntent {
                target_id,
                recruitment_path,
            } => {
                let Some(candidate) = RecruitmentCandidateId::from_stable_id(target_id) else {
                    return self.rejection(
                        request.request_id.clone(),
                        ProtocolRejectionCode::RuntimeRejected,
                        "unknown recruitment candidate ID".into(),
                        true,
                    );
                };
                let Some(path) = RecruitmentPath::from_wire(recruitment_path) else {
                    return self.rejection(
                        request.request_id.clone(),
                        ProtocolRejectionCode::RuntimeRejected,
                        "unknown recruitment path".into(),
                        true,
                    );
                };
                self.execute_command(
                    request,
                    GameplayCommand::RecruitPartyCandidate { candidate, path },
                )
            }
            GameplayIntent::SwitchLeadIntent { continuity_id } => {
                let Some(actor) = self
                    .application
                    .runtime()
                    .party()
                    .actor_from_continuity_id(continuity_id)
                else {
                    return self.rejection(
                        request.request_id.clone(),
                        ProtocolRejectionCode::RuntimeRejected,
                        "unknown party continuity ID".into(),
                        true,
                    );
                };
                self.execute_command(request, GameplayCommand::SwitchPartyLead { actor })
            }
            GameplayIntent::UseActionIntent {
                actor_continuity_id,
                action_id,
                target_continuity_id,
            } => {
                let Some(actor) = self
                    .application
                    .runtime()
                    .party()
                    .actor_from_continuity_id(actor_continuity_id)
                else {
                    return self.rejection(
                        request.request_id.clone(),
                        ProtocolRejectionCode::RuntimeRejected,
                        "unknown party continuity ID".into(),
                        true,
                    );
                };
                let Some(action) = PartyActionId::from_stable_id(action_id) else {
                    return self.rejection(
                        request.request_id.clone(),
                        ProtocolRejectionCode::RuntimeRejected,
                        "unknown party action ID".into(),
                        true,
                    );
                };
                self.execute_command(
                    request,
                    GameplayCommand::UsePartyAction {
                        actor,
                        action,
                        target_continuity_id: target_continuity_id.clone(),
                    },
                )
            }
            GameplayIntent::EnterRegionIntent { region_id } => {
                if self.application.hueman().is_none() {
                    return self.rejection(
                        request.request_id.clone(),
                        ProtocolRejectionCode::CapabilityUnavailable,
                        "EnterRegionIntent requires the permanent Hueman identity".into(),
                        true,
                    );
                }
                let map = match WorldMapId::from_wire(region_id) {
                    Ok(map) => map,
                    Err(error) => {
                        return self.rejection(
                            request.request_id.clone(),
                            ProtocolRejectionCode::RuntimeRejected,
                            error.to_string(),
                            true,
                        );
                    }
                };
                self.execute_command(request, GameplayCommand::EnterMap { map })
            }
            GameplayIntent::TraverseExitIntent { destination_map_id } => {
                let map = match WorldMapId::from_wire(destination_map_id) {
                    Ok(map) => map,
                    Err(error) => {
                        return self.rejection(
                            request.request_id.clone(),
                            ProtocolRejectionCode::RuntimeRejected,
                            error.to_string(),
                            true,
                        );
                    }
                };
                self.execute_command(request, GameplayCommand::TraverseMapExit { map })
            }
            GameplayIntent::AdvanceWorldShiftIntent => {
                self.execute_command(request, GameplayCommand::AdvanceLivingWorldShift)
            }
            GameplayIntent::SupportLivingCaseOptionIntent { case_id, choice } => self.execute_command(
                request,
                GameplayCommand::SupportLivingCase {
                    case_id: *case_id,
                    choice: *choice,
                },
            ),
            GameplayIntent::AskLivingDutyOfficerToDecideIntent { case_id } => self.execute_command(
                request,
                GameplayCommand::AskLivingDutyOfficerToDecide { case_id: *case_id },
            ),
            GameplayIntent::SupportDeepPressureSettlementIntent { choice } => self.execute_command(
                request,
                GameplayCommand::SupportDeepPressureSettlement { choice: *choice },
            ),
            GameplayIntent::AskDeepPressureAssemblyToCommitIntent => self.execute_command(
                request,
                GameplayCommand::AskDeepPressureAssemblyToCommit,
            ),
            GameplayIntent::DiscloseFacultyObservationIntent { faculty } => self.execute_command(
                request,
                GameplayCommand::DiscloseFacultyObservation { faculty: *faculty },
            ),
            GameplayIntent::SupportBoardwalkOptionIntent { choice } => self.execute_command(
                request,
                GameplayCommand::SupportBoardwalkOption { choice: *choice },
            ),
            GameplayIntent::AskReturningGoonToDecideIntent => {
                self.execute_command(request, GameplayCommand::AskReturningGoonToDecide)
            }
            GameplayIntent::SupportStonebendContinuityOptionIntent { choice } => self
                .execute_command(
                    request,
                    GameplayCommand::SupportStonebendContinuityOption { choice: *choice },
                ),
            GameplayIntent::AskStonebendToDetermineContinuityIntent => self.execute_command(
                request,
                GameplayCommand::AskStonebendToDetermineContinuity,
            ),
            GameplayIntent::SaveIntent { slot_id } => self.save(request, slot_id),
            GameplayIntent::LoadIntent { slot_id } => self.load(request, slot_id),
            unavailable => self.rejection(
                request.request_id.clone(),
                ProtocolRejectionCode::CapabilityUnavailable,
                format!(
                    "{} is reserved by protocol V1 but its authoritative reducer is not implemented",
                    unavailable.kind()
                ),
                true,
            ),
        }
    }

    fn transition_guides(
        &mut self,
        request: &ProtocolRequestEnvelope,
        direction: GuideTransitionDirection,
    ) -> ProtocolResponseEnvelope {
        let required_source = direction.source().1;
        if self.guide_session.current_layer != required_source {
            return self.rejection(
                request.request_id.clone(),
                ProtocolRejectionCode::CapabilityUnavailable,
                format!(
                    "{} requires the Guides to be in {:?}, but they are in {:?}",
                    request.intent.kind(),
                    required_source,
                    self.guide_session.current_layer
                ),
                true,
            );
        }
        if self.operational_revision == u64::MAX {
            return self.rejection(
                request.request_id.clone(),
                ProtocolRejectionCode::RuntimeRejected,
                "gameplay service revision overflowed".into(),
                true,
            );
        }
        let Some(coordinator) = self.guide_coordinator.clone() else {
            return self.rejection(
                request.request_id.clone(),
                ProtocolRejectionCode::CapabilityUnavailable,
                "Guide crossing requires a configured durable save root".into(),
                true,
            );
        };
        let transition_id =
            match StableTransitionId::new(format!("transition.{}", request.request_id)) {
                Ok(transition_id) => transition_id,
                Err(error) => {
                    return self.rejection(
                        request.request_id.clone(),
                        ProtocolRejectionCode::RuntimeRejected,
                        error.to_string(),
                        true,
                    );
                }
            };
        let evidence = match guide_transition_evidence(&request.request_id) {
            Ok(evidence) => evidence,
            Err(message) => {
                return self.rejection(
                    request.request_id.clone(),
                    ProtocolRejectionCode::RuntimeRejected,
                    message,
                    true,
                );
            }
        };
        let transition = match PairedGuideTransition::proposed(
            &self.guide_session,
            transition_id,
            direction,
            evidence,
        ) {
            Ok(transition) => transition,
            Err(error) => {
                return self.rejection(
                    request.request_id.clone(),
                    ProtocolRejectionCode::RuntimeRejected,
                    error.to_string(),
                    true,
                );
            }
        };
        let applied_transition_id = transition.transition_id.clone();
        let outcome = match (
            direction,
            CanonicalGuideSession::restore(self.guide_session.clone()),
        ) {
            (
                GuideTransitionDirection::EnterHollowGrove,
                Ok(CanonicalGuideSession::Outside(outside)),
            ) => outside
                .begin_entry(transition)
                .map_err(|error| error.to_string())
                .and_then(|pending| {
                    coordinator
                        .execute_entry(pending)
                        .map_err(|error| error.to_string())
                }),
            (
                GuideTransitionDirection::LeaveHollowGrove,
                Ok(CanonicalGuideSession::Inside(inside)),
            ) => inside
                .begin_exit(transition)
                .map_err(|error| error.to_string())
                .and_then(|pending| {
                    coordinator
                        .execute_exit(pending)
                        .map_err(|error| error.to_string())
                }),
            (_, Ok(_)) => Err("Guide transition source layer did not match its direction".into()),
            (_, Err(error)) => Err(error.to_string()),
        };
        match outcome {
            Ok(DurableGuideTransitionOutcome::Confirmed {
                session, events, ..
            }) => {
                self.guide_session = session.as_session().clone();
                self.applied_guide_transitions.insert(applied_transition_id);
                self.operational_revision += 1;
                if let Err(error) = self.persist_guide_operational_state() {
                    eprintln!(
                        "Guide transition was journaled but its operational checkpoint failed: \
                         {error}"
                    );
                }
                self.completed_with_guide_events(request.request_id.clone(), Vec::new(), events)
            }
            Ok(DurableGuideTransitionOutcome::Rejected {
                session,
                rejection,
                events,
                ..
            }) => {
                self.guide_session = session.as_session().clone();
                self.rejection_with_guide_events(
                    request.request_id.clone(),
                    ProtocolRejectionCode::RuntimeRejected,
                    format!("the Federation rejected the Guide crossing: {rejection:?}"),
                    events,
                )
            }
            Err(message) => self.rejection(
                request.request_id.clone(),
                ProtocolRejectionCode::RuntimeRejected,
                message,
                true,
            ),
        }
    }

    fn recover_guide_transitions(&mut self) -> Result<(), ProtocolServiceError> {
        let Some(coordinator) = self.guide_coordinator.clone() else {
            return Ok(());
        };
        let mut journals = coordinator
            .store()
            .journals()
            .map_err(|error| ProtocolServiceError::OperationalState(error.to_string()))?
            .into_iter()
            .filter(|journal| journal.source_session.session_id == self.guide_session.session_id)
            .filter(|journal| {
                journal.status != GuideTransitionJournalStatus::Rejected
                    && !self
                        .applied_guide_transitions
                        .contains(&journal.transition_id)
            })
            .collect::<Vec<_>>();
        let mut recovered_events = Vec::new();
        loop {
            let continuing = journals
                .iter()
                .enumerate()
                .filter(|(_, journal)| journal.source_session == self.guide_session)
                .map(|(index, _)| index)
                .collect::<Vec<_>>();
            if continuing.len() > 1 {
                return Err(ProtocolServiceError::OperationalState(format!(
                    "multiple Guide transitions fork from the same recovered session: {}",
                    continuing
                        .iter()
                        .map(|index| journals[*index].transition_id.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                )));
            }
            let Some(index) = continuing.first().copied() else {
                break;
            };
            let journal = journals.remove(index);
            let before = self.guide_session.clone();
            let outcome = coordinator
                .recover(&journal.transition_id)
                .map_err(|error| ProtocolServiceError::OperationalState(error.to_string()))?;
            match outcome {
                DurableGuideTransitionOutcome::Confirmed {
                    session, events, ..
                } => {
                    self.guide_session = session.as_session().clone();
                    self.applied_guide_transitions
                        .insert(journal.transition_id.clone());
                    recovered_events.extend(events);
                }
                DurableGuideTransitionOutcome::Rejected {
                    session, events, ..
                } => {
                    self.guide_session = session.as_session().clone();
                    recovered_events.extend(events);
                }
            }
            if self.guide_session != before {
                self.operational_revision =
                    self.operational_revision.checked_add(1).ok_or_else(|| {
                        ProtocolServiceError::OperationalState(
                            "gameplay service revision overflowed during Guide recovery".into(),
                        )
                    })?;
            }
        }
        if let Some(orphan) = journals.iter().find(|journal| {
            journal.status == GuideTransitionJournalStatus::Prepared
                && journal.source_session.session_id == self.guide_session.session_id
        }) {
            return Err(ProtocolServiceError::OperationalState(format!(
                "prepared Guide transition {} does not continue from the recovered session",
                orphan.transition_id
            )));
        }
        self.pending_guide_events = recovered_events;
        Ok(())
    }

    fn persist_guide_operational_state(&self) -> Result<(), ProtocolServiceError> {
        let Some(root) = self.save_root.as_deref() else {
            return Ok(());
        };
        self.guide_session
            .validate()
            .map_err(|error| ProtocolServiceError::OperationalState(error.to_string()))?;
        let archive = GuideOperationalStateArchive {
            schema_version: GUIDE_OPERATIONAL_STATE_SCHEMA_VERSION.into(),
            session_id: self.session_id.clone(),
            service_revision: self.operational_revision,
            guide_session: self.guide_session.clone(),
            applied_guide_transitions: self.applied_guide_transitions.clone(),
        };
        let encoded = serde_json::to_string_pretty(&archive)
            .map_err(|error| ProtocolServiceError::OperationalState(error.to_string()))?;
        write_text_artifact(
            &guide_operational_state_path(root, &self.session_id),
            &encoded,
        )
        .map_err(|error| ProtocolServiceError::OperationalState(error.to_string()))
    }

    fn save(&self, request: &ProtocolRequestEnvelope, slot_id: &str) -> ProtocolResponseEnvelope {
        let Some(path) = self.save_path(slot_id) else {
            return self.rejection(
                request.request_id.clone(),
                ProtocolRejectionCode::CapabilityUnavailable,
                "SaveIntent requires a configured gameplay save root and stable slot ID".into(),
                true,
            );
        };
        let result = self
            .application
            .encode_archive()
            .map_err(|error| error.to_string())
            .and_then(|archive| {
                self.guide_session
                    .validate()
                    .map_err(|error| error.to_string())?;
                serde_json::to_string_pretty(&GameplayServiceSaveArchive {
                    service_schema_version: GAMEPLAY_SERVICE_SAVE_SCHEMA_VERSION,
                    service_revision: self.operational_revision,
                    gameplay_archive: archive,
                    guide_session: self.guide_session.clone(),
                    applied_guide_transitions: self.applied_guide_transitions.clone(),
                })
                .map_err(|error| error.to_string())
            })
            .and_then(|archive| self.write_save_with_backup(&path, &archive));
        match result {
            Ok(()) => self.completed(
                request.request_id.clone(),
                vec![GameplayEventView::snapshot_saved(
                    self.application.revision(),
                )],
            ),
            Err(message) => self.rejection(
                request.request_id.clone(),
                ProtocolRejectionCode::RuntimeRejected,
                message,
                true,
            ),
        }
    }

    fn load(
        &mut self,
        request: &ProtocolRequestEnvelope,
        slot_id: &str,
    ) -> ProtocolResponseEnvelope {
        let Some(path) = self.save_path(slot_id) else {
            return self.rejection(
                request.request_id.clone(),
                ProtocolRejectionCode::CapabilityUnavailable,
                "LoadIntent requires a configured gameplay save root and stable slot ID".into(),
                true,
            );
        };
        let loaded = self.read_save_or_backup(&path);
        let (application, guide_session, service_revision, applied_guide_transitions) = match loaded
        {
            Ok(loaded) => loaded,
            Err(message) => {
                return self.rejection(
                    request.request_id.clone(),
                    ProtocolRejectionCode::RuntimeRejected,
                    message,
                    true,
                );
            }
        };
        if application.runtime().rule_set() != self.application.runtime().rule_set() {
            return self.rejection(
                request.request_id.clone(),
                ProtocolRejectionCode::RuntimeRejected,
                "saved gameplay rule set does not match this service".into(),
                true,
            );
        }
        let mut applied_guide_transitions = applied_guide_transitions;
        if let Some(coordinator) = &self.guide_coordinator {
            let journals = match coordinator.store().journals() {
                Ok(journals) => journals,
                Err(error) => {
                    return self.rejection(
                        request.request_id.clone(),
                        ProtocolRejectionCode::RuntimeRejected,
                        error.to_string(),
                        true,
                    );
                }
            };
            if journals.iter().any(|journal| {
                journal.source_session.session_id == guide_session.session_id
                    && journal.status == GuideTransitionJournalStatus::Prepared
            }) {
                return self.rejection(
                    request.request_id.clone(),
                    ProtocolRejectionCode::RuntimeRejected,
                    "a prepared Guide transition must be recovered before loading a save".into(),
                    true,
                );
            }
            applied_guide_transitions.extend(
                journals
                    .into_iter()
                    .filter(|journal| {
                        journal.source_session.session_id == guide_session.session_id
                            && journal.status == GuideTransitionJournalStatus::Confirmed
                    })
                    .map(|journal| journal.transition_id),
            );
        }
        self.application = application;
        self.guide_session = guide_session;
        self.applied_guide_transitions = applied_guide_transitions;
        self.operational_revision = service_revision;
        if let Err(error) = self.persist_guide_operational_state() {
            return self.rejection(
                request.request_id.clone(),
                ProtocolRejectionCode::RuntimeRejected,
                error.to_string(),
                true,
            );
        }
        self.completed(
            request.request_id.clone(),
            vec![GameplayEventView::snapshot_loaded(
                self.application.revision(),
            )],
        )
    }

    fn decode_service_save(
        &self,
        archive: &str,
    ) -> Result<
        (
            GameApplicationService,
            DualGuideSession,
            u64,
            BTreeSet<StableTransitionId>,
        ),
        String,
    > {
        let value = serde_json::from_str::<serde_json::Value>(archive)
            .map_err(|error| error.to_string())?;
        if value.get("service_schema_version").is_none() {
            let application = GameApplicationService::from_archive_with_world_session(
                archive,
                self.application.runtime().world_session().clone(),
            )
            .map_err(|error| error.to_string())?;
            let session_id = StableGuideSessionId::new(self.session_id.clone())
                .map_err(|error| error.to_string())?;
            let revision = application.revision().get();
            return Ok((
                application,
                DualGuideSession::inside(session_id),
                revision,
                BTreeSet::new(),
            ));
        }
        let saved = serde_json::from_value::<GameplayServiceSaveArchive>(value)
            .map_err(|error| error.to_string())?;
        if saved.service_schema_version != GAMEPLAY_SERVICE_SAVE_SCHEMA_VERSION {
            return Err(format!(
                "unsupported gameplay service save schema version {}",
                saved.service_schema_version
            ));
        }
        saved
            .guide_session
            .validate()
            .map_err(|error| error.to_string())?;
        if saved.guide_session.session_id.as_str() != self.session_id {
            return Err("saved Guide session does not match this gameplay service".into());
        }
        let application = GameApplicationService::from_archive_with_world_session(
            &saved.gameplay_archive,
            self.application.runtime().world_session().clone(),
        )
        .map_err(|error| error.to_string())?;
        Ok((
            application,
            saved.guide_session,
            saved.service_revision,
            saved.applied_guide_transitions,
        ))
    }

    fn write_save_with_backup(&self, path: &Path, archive: &str) -> Result<(), String> {
        if let Ok(existing) = read_text_artifact(path)
            && self.decode_service_save(&existing).is_ok()
        {
            write_text_artifact(&gameplay_save_backup_path(path), &existing)
                .map_err(|error| error.to_string())?;
        }
        write_text_artifact(path, archive).map_err(|error| error.to_string())
    }

    fn read_save_or_backup(
        &self,
        path: &Path,
    ) -> Result<
        (
            GameApplicationService,
            DualGuideSession,
            u64,
            BTreeSet<StableTransitionId>,
        ),
        String,
    > {
        match read_text_artifact(path)
            .map_err(|error| error.to_string())
            .and_then(|archive| self.decode_service_save(&archive))
        {
            Ok(save) => Ok(save),
            Err(primary_error) => read_text_artifact(&gameplay_save_backup_path(path))
                .map_err(|backup_error| {
                    format!(
                        "primary save failed ({primary_error}); backup could not be read ({backup_error})"
                    )
                })
                .and_then(|archive| {
                    self.decode_service_save(&archive).map_err(|backup_error| {
                        format!(
                            "primary save failed ({primary_error}); backup failed ({backup_error})"
                        )
                    })
                }),
        }
    }

    fn save_path(&self, slot_id: &str) -> Option<PathBuf> {
        if !is_stable_wire_id(slot_id) {
            return None;
        }
        self.save_root
            .as_deref()
            .map(|root| gameplay_save_path(root, &self.session_id, slot_id))
    }

    fn execute_command(
        &mut self,
        request: &ProtocolRequestEnvelope,
        command: GameplayCommand,
    ) -> ProtocolResponseEnvelope {
        if self.operational_revision == u64::MAX {
            return self.rejection(
                request.request_id.clone(),
                ProtocolRejectionCode::RuntimeRejected,
                "gameplay service revision overflowed".into(),
                true,
            );
        }
        let Some(causal) = self.application.revision().get().checked_add(1) else {
            return self.rejection(
                request.request_id.clone(),
                ProtocolRejectionCode::RuntimeRejected,
                "gameplay causal position overflowed".into(),
                true,
            );
        };
        let event_id = match GameplayEventId::new(format!("game-event.{}", request.request_id)) {
            Ok(value) => value,
            Err(error) => {
                return self.rejection(
                    request.request_id.clone(),
                    ProtocolRejectionCode::RuntimeRejected,
                    error.to_string(),
                    true,
                );
            }
        };
        let projected = self
            .application
            .execute(
                GameplayEventMetadata {
                    id: event_id,
                    causal_position: CausalPosition::new(causal),
                },
                command,
            )
            .map(GameplayEventView::from_canonical)
            .map_err(|error| error.to_string());
        match projected {
            Ok(event) => {
                self.operational_revision += 1;
                self.completed(request.request_id.clone(), vec![event])
            }
            Err(message) => self.rejection(
                request.request_id.clone(),
                ProtocolRejectionCode::RuntimeRejected,
                message,
                true,
            ),
        }
    }

    fn completed(
        &self,
        request_id: String,
        events: Vec<GameplayEventView>,
    ) -> ProtocolResponseEnvelope {
        self.completed_with_guide_events(request_id, events, Vec::new())
    }

    fn completed_with_guide_events(
        &self,
        request_id: String,
        events: Vec<GameplayEventView>,
        guide_events: Vec<GuideTransitionEvent>,
    ) -> ProtocolResponseEnvelope {
        ProtocolResponseEnvelope {
            protocol_version: HOLLOW_GROVE_GAMEPLAY_PROTOCOL_VERSION,
            session_id: self.session_id.clone(),
            request_id,
            status: ProtocolResponseStatus::Completed,
            revision: self.revision(),
            events,
            view: Some(self.current_view_with_guide_events(guide_events)),
            rejection: None,
        }
    }

    fn rejection(
        &self,
        request_id: String,
        code: ProtocolRejectionCode,
        message: String,
        include_view: bool,
    ) -> ProtocolResponseEnvelope {
        ProtocolResponseEnvelope {
            protocol_version: HOLLOW_GROVE_GAMEPLAY_PROTOCOL_VERSION,
            session_id: self.session_id.clone(),
            request_id,
            status: ProtocolResponseStatus::Rejected,
            revision: self.revision(),
            events: Vec::new(),
            view: include_view.then(|| self.current_view()),
            rejection: Some(ProtocolRejection { code, message }),
        }
    }

    fn rejection_with_guide_events(
        &self,
        request_id: String,
        code: ProtocolRejectionCode,
        message: String,
        guide_events: Vec<GuideTransitionEvent>,
    ) -> ProtocolResponseEnvelope {
        ProtocolResponseEnvelope {
            protocol_version: HOLLOW_GROVE_GAMEPLAY_PROTOCOL_VERSION,
            session_id: self.session_id.clone(),
            request_id,
            status: ProtocolResponseStatus::Rejected,
            revision: self.revision(),
            events: Vec::new(),
            view: Some(self.current_view_with_guide_events(guide_events)),
            rejection: Some(ProtocolRejection { code, message }),
        }
    }

    fn current_view(&self) -> GameView {
        self.current_view_with_guide_events(Vec::new())
    }

    fn current_view_with_guide_events(&self, guide_events: Vec<GuideTransitionEvent>) -> GameView {
        let mut view = GameView::from_runtime_with_guide_session(
            self.application.runtime(),
            capability_views(
                self.application.runtime(),
                self.save_root.is_some(),
                self.guide_coordinator.is_some(),
                self.guide_session.current_layer,
            ),
            self.guide_session.session_id.clone(),
        );
        view.revision = self.operational_revision;
        view.guide_context = self
            .guide_session
            .guide_context_v1()
            .expect("the service stores only canonical Guide sessions");
        let federation_audit =
            audit_gameplay_federation_binding(self.application.federation_binding());
        let guide_audit = audit_guide_context(&view.guide_context);
        view.kernel_audit = KernelAuditView::from_reports(&federation_audit, &guide_audit);
        view.guide_transition_events = guide_events;
        view
    }

    #[must_use]
    pub fn handle_json_line(&mut self, line: &str) -> String {
        let response = if line.len() > MAX_GAMEPLAY_MESSAGE_BYTES {
            self.rejection(
                "unparseable".into(),
                ProtocolRejectionCode::MalformedRequest,
                format!(
                    "request exceeds the {} byte protocol limit",
                    MAX_GAMEPLAY_MESSAGE_BYTES
                ),
                false,
            )
        } else {
            match serde_json::from_str::<ProtocolRequestEnvelope>(line) {
                Ok(request) => self.handle(request),
                Err(error) => self.rejection(
                    "unparseable".into(),
                    ProtocolRejectionCode::MalformedRequest,
                    error.to_string(),
                    false,
                ),
            }
        };
        let encoded =
            serde_json::to_string(&response).expect("protocol response values are serializable");
        if encoded.len() <= MAX_GAMEPLAY_RESPONSE_BYTES {
            return encoded;
        }
        serde_json::to_string(&ProtocolResponseEnvelope {
            protocol_version: HOLLOW_GROVE_GAMEPLAY_PROTOCOL_VERSION,
            session_id: self.session_id.clone(),
            request_id: response.request_id,
            status: ProtocolResponseStatus::Rejected,
            revision: self.revision(),
            events: Vec::new(),
            view: None,
            rejection: Some(ProtocolRejection {
                code: ProtocolRejectionCode::ResponseTooLarge,
                message: format!(
                    "authoritative response exceeded the {} byte transport limit; retry SyncIntent after reducing projected history",
                    MAX_GAMEPLAY_RESPONSE_BYTES
                ),
            }),
        })
        .expect("bounded protocol rejection is serializable")
    }
}

fn is_stable_wire_id(value: &str) -> bool {
    !value.is_empty()
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'.' | b'-')
        })
}

fn gameplay_save_path(root: &Path, session_id: &str, slot_id: &str) -> PathBuf {
    root.join(format!("{session_id}.{slot_id}.hgplay.json"))
}

fn gameplay_save_backup_path(path: &Path) -> PathBuf {
    let mut value = path.as_os_str().to_owned();
    value.push(".backup");
    PathBuf::from(value)
}

#[must_use]
pub fn guide_operational_state_path(root: &Path, session_id: &str) -> PathBuf {
    root.join(format!("{session_id}.guides.json"))
}

#[must_use]
pub fn guide_transition_journal_root(root: &Path, session_id: &str) -> PathBuf {
    root.join(format!("{session_id}.guide-transitions-v2"))
}

fn gameplay_runtime_lease_path(root: &Path, session_id: &str) -> PathBuf {
    root.join(format!("{session_id}.runtime.lock"))
}

fn process_start_token(pid: u32) -> std::io::Result<u64> {
    let stat = fs::read_to_string(format!("/proc/{pid}/stat"))?;
    let close = stat.rfind(')').ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "process stat omitted its command terminator",
        )
    })?;
    stat[close + 1..]
        .split_whitespace()
        .nth(19)
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "process stat omitted its start token",
            )
        })?
        .parse()
        .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error))
}

fn runtime_lease_owner_is_active(owner: &str) -> bool {
    if Path::new(owner)
        .file_name()
        .and_then(|value| value.to_str())
        != Some(owner)
    {
        return true;
    }
    let Some(without_suffix) = owner.strip_suffix(".claim") else {
        return true;
    };
    let mut parts = without_suffix.rsplit('.');
    let Some(_nonce) = parts.next().and_then(|value| value.parse::<u64>().ok()) else {
        return true;
    };
    let Some(start) = parts.next().and_then(|value| value.parse::<u64>().ok()) else {
        return true;
    };
    let Some(pid) = parts.next().and_then(|value| value.parse::<u32>().ok()) else {
        return true;
    };
    if start == 0 {
        return true;
    }
    process_start_token(pid).is_ok_and(|actual| actual == start)
}

fn load_guide_operational_state(
    path: &Path,
    session_id: &str,
    guide_session_id: &StableGuideSessionId,
) -> Result<(DualGuideSession, u64, BTreeSet<StableTransitionId>), ProtocolServiceError> {
    let encoded = read_text_artifact(path)
        .map_err(|error| ProtocolServiceError::OperationalState(error.to_string()))?;
    let archive = serde_json::from_str::<GuideOperationalStateArchive>(&encoded)
        .map_err(|error| ProtocolServiceError::OperationalState(error.to_string()))?;
    if archive.schema_version != GUIDE_OPERATIONAL_STATE_SCHEMA_VERSION {
        return Err(ProtocolServiceError::OperationalState(format!(
            "unsupported Guide operational state schema {}",
            archive.schema_version
        )));
    }
    archive
        .guide_session
        .validate()
        .map_err(|error| ProtocolServiceError::OperationalState(error.to_string()))?;
    if archive.session_id != session_id || archive.guide_session.session_id != *guide_session_id {
        return Err(ProtocolServiceError::OperationalState(
            "Guide operational state does not match the requested gameplay session".into(),
        ));
    }
    Ok((
        archive.guide_session,
        archive.service_revision,
        archive.applied_guide_transitions,
    ))
}

fn guide_transition_evidence(request_id: &str) -> Result<GuideTransitionEvidence, String> {
    let reference = |namespace: &str, suffix: &str| {
        ExternalRef::new(namespace, format!("{request_id}.{suffix}"))
            .map_err(|error| error.to_string())
    };
    Ok(GuideTransitionEvidence {
        roi_continuity_ref: reference("guide-continuity-proof", "roi")?,
        roi_embodiment_ref: reference("guide-embodiment-proof", "roi")?,
        roselina_continuity_ref: reference("guide-continuity-proof", "roselina")?,
        roselina_embodiment_ref: reference("guide-embodiment-proof", "roselina")?,
        source_witness_ref: Some(
            ExternalRef::new("protocol-request", request_id).map_err(|error| error.to_string())?,
        ),
    })
}

fn intent_requires_hollow_grove(intent: &GameplayIntent) -> bool {
    !matches!(
        intent,
        GameplayIntent::SyncIntent
            | GameplayIntent::EnterHollowGroveIntent
            | GameplayIntent::LeaveHollowGroveIntent
            | GameplayIntent::SaveIntent { .. }
            | GameplayIntent::LoadIntent { .. }
    )
}

fn capability_views(
    runtime: &HollowGroveGameRuntime,
    persistence_available: bool,
    guide_transition_available: bool,
    guide_layer: WorldLayer,
) -> Vec<IntentCapabilityView> {
    let can_establish_hueman = runtime.hueman().is_none();
    let mut capabilities = vec![
        IntentCapabilityView {
            intent_type: "SyncIntent".into(),
            available: true,
            unavailable_reason: None,
        },
        IntentCapabilityView {
            intent_type: "EstablishHuemanIntent".into(),
            available: can_establish_hueman,
            unavailable_reason: (!can_establish_hueman)
                .then(|| "the permanent Hueman identity is already established".into()),
        },
        IntentCapabilityView {
            intent_type: "MoveIntent".into(),
            available: !can_establish_hueman,
            unavailable_reason: can_establish_hueman
                .then(|| "the permanent Hueman identity is not established".into()),
        },
        IntentCapabilityView {
            intent_type: "InteractIntent".into(),
            available: !can_establish_hueman,
            unavailable_reason: can_establish_hueman
                .then(|| "the permanent Hueman identity is not established".into()),
        },
    ];
    let hueman_exists = !can_establish_hueman;
    let party = runtime.party();
    capabilities.push(IntentCapabilityView {
        intent_type: "OpenPartyIntent".into(),
        available: hueman_exists,
        unavailable_reason: (!hueman_exists)
            .then(|| "the permanent Hueman identity is not established".into()),
    });
    capabilities.push(IntentCapabilityView {
        intent_type: "SelectPartyMemberIntent".into(),
        available: party.member_count() > 1,
        unavailable_reason: (party.member_count() <= 1)
            .then(|| "recruit at least one companion first".into()),
    });
    let active_candidate = match runtime.active_interaction() {
        Some(super::InteractionId::DeepPressurePerson(person)) => {
            RecruitmentCandidateId::for_person(person)
        }
        _ => None,
    };
    let can_recruit = hueman_exists
        && runtime.deep_pressure().outcome.is_some()
        && party.member_count() < MAX_PARTY_MEMBERS
        && active_candidate.is_some_and(|candidate| {
            !party.recruitment_decisions.contains_key(&candidate)
                && runtime
                    .scheduled_people()
                    .iter()
                    .any(|presence| presence.person_id == candidate.person())
        });
    let ready_companion_leads = party.companions.iter().any(|member| {
        member.availability == PartyMemberAvailability::Ready
            && party.lead != PartyActorId::Companion(member.candidate_id)
    });
    let lead_action_available = match party.lead {
        PartyActorId::Hueman => false,
        PartyActorId::Companion(candidate) => party.member(party.lead).is_some_and(|member| {
            member.availability == PartyMemberAvailability::Ready
                && member.field_action == candidate.action()
        }),
    };
    capabilities.push(IntentCapabilityView {
        intent_type: "UseActionIntent".into(),
        available: lead_action_available,
        unavailable_reason: (!lead_action_available)
            .then(|| "a ready companion with a field action must lead".into()),
    });
    capabilities.push(IntentCapabilityView {
        intent_type: "RecruitIntent".into(),
        available: can_recruit,
        unavailable_reason: (!can_recruit).then(|| {
            "finish Deep Pressure, face a present candidate, and keep an open party slot".into()
        }),
    });
    capabilities.push(IntentCapabilityView {
        intent_type: "SwitchLeadIntent".into(),
        available: party.member_count() > 1
            && (ready_companion_leads || party.lead != PartyActorId::Hueman),
        unavailable_reason: (!(party.member_count() > 1
            && (ready_companion_leads || party.lead != PartyActorId::Hueman)))
            .then(|| "no other ready party member can lead".into()),
    });
    capabilities.push(IntentCapabilityView {
        intent_type: "BeginSynthesisIntent".into(),
        available: false,
        unavailable_reason: Some("authoritative reducer not implemented".into()),
    });
    for intent_type in ["SaveIntent", "LoadIntent"] {
        capabilities.push(IntentCapabilityView {
            intent_type: intent_type.into(),
            available: persistence_available,
            unavailable_reason: (!persistence_available)
                .then(|| "authoritative reducer not implemented".into()),
        });
    }
    capabilities.insert(
        9,
        IntentCapabilityView {
            intent_type: "EnterRegionIntent".into(),
            available: hueman_exists,
            unavailable_reason: (!hueman_exists)
                .then(|| "authoritative reducer not implemented".into()),
        },
    );
    capabilities.push(IntentCapabilityView {
        intent_type: "TraverseExitIntent".into(),
        available: hueman_exists,
        unavailable_reason: (!hueman_exists)
            .then(|| "the permanent Hueman identity is not established".into()),
    });
    capabilities.push(IntentCapabilityView {
        intent_type: "AdvanceWorldShiftIntent".into(),
        available: hueman_exists,
        unavailable_reason: (!hueman_exists)
            .then(|| "the permanent Hueman identity is not established".into()),
    });
    let active_living_case = runtime.living_world().cases.values().find(|case| {
        case.resolved_choice.is_none()
            && match case.case_id {
                LivingCaseId::AuraFieldDroughtAllocation => {
                    runtime.hueman_map() == WorldMapId::AuraFieldWorkingLand
                }
                LivingCaseId::AuraBeachStormRescue => {
                    runtime.hueman_map() == WorldMapId::AuraBeachCoastalCommons
                }
                LivingCaseId::AuraBasinInjuredBeing => {
                    runtime.hueman_map() == WorldMapId::AuraBasinCollisionGrounds
                }
                LivingCaseId::MntAuraRoofFall => {
                    runtime.hueman_map()
                        == WorldMapId::ExtractionSite(
                            crate::world::extraction::ExtractionSiteId::MntAuraHighMine,
                        )
                }
                LivingCaseId::HighwayToHellGasPocket => {
                    runtime.hueman_map()
                        == WorldMapId::ExtractionSite(
                            crate::world::extraction::ExtractionSiteId::HighwayToHellDeepworks,
                        )
                }
                LivingCaseId::RiptideWellBlowout => {
                    runtime.hueman_map()
                        == WorldMapId::ExtractionSite(
                            crate::world::extraction::ExtractionSiteId::RiptideRecoveryRig,
                        )
                }
                LivingCaseId::CurrentSeaWellCertification => {
                    runtime.hueman_map()
                        == WorldMapId::ExtractionSite(
                            crate::world::extraction::ExtractionSiteId::CurrentSeaDepthRig,
                        )
                }
            }
    });
    let living_support_available =
        active_living_case.is_some_and(|case| case.ready() && case.supported_choice.is_none());
    let living_decision_available = active_living_case
        .is_some_and(|case| case.supported_choice.is_some() && case.resolved_choice.is_none());
    capabilities.push(IntentCapabilityView {
        intent_type: "SupportLivingCaseOptionIntent".into(),
        available: hueman_exists && living_support_available,
        unavailable_reason: (!(hueman_exists && living_support_available))
            .then(|| "the local living-world case requires all three evidence records".into()),
    });
    capabilities.push(IntentCapabilityView {
        intent_type: "AskLivingDutyOfficerToDecideIntent".into(),
        available: hueman_exists && living_decision_available,
        unavailable_reason: (!(hueman_exists && living_decision_available))
            .then(|| "record nonbinding support for a lawful local option first".into()),
    });
    let deep_pressure = runtime.deep_pressure();
    let at_deep_pressure_settlement = runtime.hueman_map() == WorldMapId::BoardwalkReturnVestibule
        && deep_pressure.phase() == DeepPressurePhase::BoardwalkSettlement;
    capabilities.push(IntentCapabilityView {
        intent_type: "SupportDeepPressureSettlementIntent".into(),
        available: hueman_exists
            && at_deep_pressure_settlement
            && deep_pressure.ready_for_settlement_support(),
        unavailable_reason: (!(hueman_exists
            && at_deep_pressure_settlement
            && deep_pressure.ready_for_settlement_support()))
        .then(|| {
            format!(
                "Deep Pressure requires all seven duty outcomes and {} missing affected statements",
                deep_pressure.missing_required_statements().len()
            )
        }),
    });
    capabilities.push(IntentCapabilityView {
        intent_type: "AskDeepPressureAssemblyToCommitIntent".into(),
        available: hueman_exists
            && at_deep_pressure_settlement
            && deep_pressure.supported_settlement.is_some()
            && deep_pressure.outcome.is_none(),
        unavailable_reason: (!(hueman_exists
            && at_deep_pressure_settlement
            && deep_pressure.supported_settlement.is_some()
            && deep_pressure.outcome.is_none()))
        .then(|| "record advisory settlement support on the Boardwalk first".into()),
    });
    if runtime.hueman_map() == WorldMapId::BoardwalkReturnVestibule
        && let Some(case) = runtime.boardwalk_case()
    {
        let resolved = case.committed_choice().is_some();
        capabilities.extend([
            IntentCapabilityView {
                intent_type: "DiscloseFacultyObservationIntent".into(),
                available: !resolved && case.faculties().len() < HuemanFaculty::ALL.len(),
                unavailable_reason: (resolved
                    || case.faculties().len() == HuemanFaculty::ALL.len())
                .then(|| "faculty testimony is complete or the case is resolved".into()),
            },
            IntentCapabilityView {
                intent_type: "SupportBoardwalkOptionIntent".into(),
                available: case.is_ready() && !resolved,
                unavailable_reason: (!case.is_ready() || resolved)
                    .then(|| "all evidence and faculty observations are required first".into()),
            },
            IntentCapabilityView {
                intent_type: "AskReturningGoonToDecideIntent".into(),
                available: case.supported_choice().is_some() && !resolved,
                unavailable_reason: (case.supported_choice().is_none() || resolved)
                    .then(|| "record support before asking the Returning Goon to decide".into()),
            },
        ]);
    }
    if runtime.hueman_map() == WorldMapId::CurrentSeaDeepCertificationLanding
        && let Some(case) = runtime.stonebend_case()
    {
        let resolved = case.committed_choice().is_some();
        capabilities.extend([
            IntentCapabilityView {
                intent_type: "DiscloseFacultyObservationIntent".into(),
                available: !resolved && case.faculties().len() < HuemanFaculty::ALL.len(),
                unavailable_reason: (resolved
                    || case.faculties().len() == HuemanFaculty::ALL.len())
                .then(|| "faculty testimony is complete or the case is resolved".into()),
            },
            IntentCapabilityView {
                intent_type: "SupportStonebendContinuityOptionIntent".into(),
                available: case.is_ready() && !resolved,
                unavailable_reason: (!case.is_ready() || resolved)
                    .then(|| "all evidence and faculty observations are required first".into()),
            },
            IntentCapabilityView {
                intent_type: "AskStonebendToDetermineContinuityIntent".into(),
                available: case.supported_choice().is_some() && !resolved,
                unavailable_reason: (case.supported_choice().is_none() || resolved).then(|| {
                    "record advisory support before asking Stonebend to determine continuity".into()
                }),
            },
        ]);
    }
    capabilities.extend([
        IntentCapabilityView {
            intent_type: "EnterHollowGroveIntent".into(),
            available: guide_transition_available && guide_layer == WorldLayer::OutsideHollowGrove,
            unavailable_reason: (!(guide_transition_available
                && guide_layer == WorldLayer::OutsideHollowGrove))
                .then(|| {
                    if !guide_transition_available {
                        "a durable Guide transition coordinator is not configured".into()
                    } else {
                        "Roi and Roselina are already inside Hollow Grove".into()
                    }
                }),
        },
        IntentCapabilityView {
            intent_type: "LeaveHollowGroveIntent".into(),
            available: guide_transition_available && guide_layer == WorldLayer::HollowGrove,
            unavailable_reason: (!(guide_transition_available
                && guide_layer == WorldLayer::HollowGrove))
                .then(|| {
                    if !guide_transition_available {
                        "a durable Guide transition coordinator is not configured".into()
                    } else {
                        "Roï and Rosaliña are already outside Hollow Grove".into()
                    }
                }),
        },
    ]);
    if guide_layer != WorldLayer::HollowGrove {
        for capability in &mut capabilities {
            if !matches!(
                capability.intent_type.as_str(),
                "SyncIntent"
                    | "EnterHollowGroveIntent"
                    | "LeaveHollowGroveIntent"
                    | "SaveIntent"
                    | "LoadIntent"
            ) {
                capability.available = false;
                capability.unavailable_reason =
                    Some("Roï and Rosaliña must enter Hollow Grove first".into());
            }
        }
    }
    capabilities
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProtocolServiceError {
    InvalidSessionId(String),
    InstitutionalState(String),
    OperationalState(String),
    RuntimeLease(String),
}

impl std::fmt::Display for ProtocolServiceError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSessionId(value) => {
                write!(formatter, "invalid gameplay protocol session ID: {value}")
            }
            Self::InstitutionalState(error) => {
                write!(formatter, "invalid gameplay institutional state: {error}")
            }
            Self::OperationalState(error) => {
                write!(formatter, "invalid Guide operational state: {error}")
            }
            Self::RuntimeLease(error) => {
                write!(formatter, "gameplay runtime lease failed: {error}")
            }
        }
    }
}

impl std::error::Error for ProtocolServiceError {}
