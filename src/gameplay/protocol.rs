use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::constitutional::{CausalPosition, ParticipantId, RuleSetId};
use crate::institution::InstitutionalBeingId;
use crate::world::session::WorldSession;
use crate::{read_text_artifact, write_text_artifact};

use super::{
    BeingContinuityId, BoardwalkChoice, CardinalDirection, DeepPressurePhase,
    DeepPressureSettlementChoice, GameApplicationService, GameView, GameplayCommand,
    GameplayEventId, GameplayEventMetadata, GameplayEventView, HollowGroveGameRuntime,
    HuemanFaculty, IntentCapabilityView, LivingCaseChoice, LivingCaseId, MAX_PARTY_MEMBERS,
    PartyActionId, PartyActorId, PartyMemberAvailability, RecruitmentCandidateId, RecruitmentPath,
    StonebendContinuityChoice, WorldMapId,
};

pub const HOLLOW_GROVE_GAMEPLAY_PROTOCOL_VERSION: u16 = 1;
pub const DEFAULT_GAMEPLAY_SERVICE_ADDRESS: &str = "127.0.0.1:47819";
pub const MAX_GAMEPLAY_MESSAGE_BYTES: usize = 64 * 1024;

/// Version-one player intents. Schema presence does not imply capability.
/// Availability is projected by the runtime and unavailable intents fail
/// explicitly without mutating state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", deny_unknown_fields)]
pub enum GameplayIntent {
    SyncIntent,
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
    StaleRevision,
    CapabilityUnavailable,
    MalformedRequest,
    RuntimeRejected,
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

/// Stateful application boundary used by transport adapters.
///
/// It owns request idempotency and revision checks. Only accepted intents are
/// translated into authoritative gameplay commands.
#[derive(Debug)]
pub struct GameProtocolService {
    session_id: String,
    application: GameApplicationService,
    completed_requests: BTreeMap<String, CompletedProtocolRequest>,
    save_root: Option<PathBuf>,
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
        Ok(Self {
            session_id,
            application: GameApplicationService::new(rule_set),
            completed_requests: BTreeMap::new(),
            save_root: None,
        })
    }

    pub fn new_with_save_root(
        session_id: impl Into<String>,
        rule_set: RuleSetId,
        save_root: impl Into<PathBuf>,
    ) -> Result<Self, ProtocolServiceError> {
        let mut service = Self::new(session_id, rule_set)?;
        service.save_root = Some(save_root.into());
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
        let world = WorldSession::load_or_canonical_at(world_root.as_ref())
            .map_err(|error| ProtocolServiceError::InstitutionalState(error.to_string()))?;
        Ok(Self {
            session_id,
            application: GameApplicationService::with_world_session(rule_set, world),
            completed_requests: BTreeMap::new(),
            save_root: Some(save_root.into()),
        })
    }

    #[must_use]
    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    #[must_use]
    pub fn revision(&self) -> u64 {
        self.application.revision().get()
    }

    #[must_use]
    pub const fn application(&self) -> &GameApplicationService {
        &self.application
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
        self.completed_requests.insert(
            request.request_id.clone(),
            CompletedProtocolRequest {
                request,
                response: response.clone(),
            },
        );
        response
    }

    fn dispatch(&mut self, request: &ProtocolRequestEnvelope) -> ProtocolResponseEnvelope {
        match &request.intent {
            GameplayIntent::SyncIntent => self.completed(
                request.request_id.clone(),
                vec![GameplayEventView::snapshot_loaded(
                    self.application.revision(),
                )],
            ),
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
                let causal = match self.revision().checked_add(1) {
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
                    Ok(event) => self.completed(
                        request.request_id.clone(),
                        vec![event],
                    ),
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
                let causal = match self.revision().checked_add(1) {
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
                    Ok(event) => self.completed(request.request_id.clone(), vec![event]),
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
                let causal = match self.revision().checked_add(1) {
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
                    Ok(event) => self.completed(request.request_id.clone(), vec![event]),
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
                write_text_artifact(&path, &archive).map_err(|error| error.to_string())
            });
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
        let loaded = read_text_artifact(&path)
            .map_err(|error| error.to_string())
            .and_then(|archive| {
                GameApplicationService::from_archive_with_world_session(
                    &archive,
                    self.application.runtime().world_session().clone(),
                )
                .map_err(|error| error.to_string())
            });
        let application = match loaded {
            Ok(application) => application,
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
        self.application = application;
        self.completed(
            request.request_id.clone(),
            vec![GameplayEventView::snapshot_loaded(
                self.application.revision(),
            )],
        )
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
        let Some(causal) = self.revision().checked_add(1) else {
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
            Ok(event) => self.completed(request.request_id.clone(), vec![event]),
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
        ProtocolResponseEnvelope {
            protocol_version: HOLLOW_GROVE_GAMEPLAY_PROTOCOL_VERSION,
            session_id: self.session_id.clone(),
            request_id,
            status: ProtocolResponseStatus::Completed,
            revision: self.revision(),
            events,
            view: Some(self.current_view()),
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

    fn current_view(&self) -> GameView {
        GameView::from_runtime(
            self.application.runtime(),
            capability_views(self.application.runtime(), self.save_root.is_some()),
        )
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
        serde_json::to_string(&response).expect("protocol response values are serializable")
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

fn capability_views(
    runtime: &HollowGroveGameRuntime,
    persistence_available: bool,
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
    capabilities
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProtocolServiceError {
    InvalidSessionId(String),
    InstitutionalState(String),
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
        }
    }
}

impl std::error::Error for ProtocolServiceError {}
