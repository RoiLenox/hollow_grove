use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::constitutional::{
    AuthorityActorId, CausalPosition, ConstitutionalRuntime, HouseDecisionId, ParticipantId,
    RegionalBeingId, RegionalBeingRegistration, RegionalEvent, RegionalEventEnvelope,
    RegionalEventMetadata, RegionalSynthesisError, RegionalSynthesisRuntime, RuleSetId,
};
use crate::institution::InstitutionalBeingId;
use crate::world::aura_basin::{AuraBasinError, AuraBasinFacilityId, canonical_aura_basin};
use crate::world::aura_beach::{AuraBeachError, AuraBeachFacilityId, canonical_aura_beach};
use crate::world::aura_field::{AuraFieldError, AuraFieldFacilityId, canonical_aura_field};
use crate::world::extraction::{ExtractionFacilityId, ExtractionSiteId};
use crate::world::geography::ConstitutionalRouteId;
use crate::world::interior_surface::InteriorSurfaceId;
use crate::world::route_network::{RouteNetwork, RouteNetworkError};
use crate::world::session::WorldSession;

use super::{
    ActiveIncarnationRef, BeingContinuityId, BeingContinuityRecord, BoardwalkCase,
    BoardwalkCaseError, BoardwalkChoice, BoardwalkEvidence, BoardwalkOutcomeId, DeepPressureError,
    DeepPressureEvent, DeepPressureSettlementChoice, DeepPressureState, GameplayEventId,
    GameplayIdentityError, HuemanFaculty, IdentityRegistry, InteractionId, LivingCaseChoice,
    LivingCaseId, LivingEvidence, LivingWorldError, LivingWorldEvent, LivingWorldState,
    MERCY_DEEP_PARTICIPANT_ID, PartyActionId, PartyActorId, PartyError, PartyEvent, PartyState,
    RETURNING_GOON_PARTICIPANT_ID, RecruitmentCandidateId, RecruitmentPath, StonebendCaseError,
    StonebendContinuityCase, StonebendContinuityChoice, StonebendEvidence, StonebendOutcomeId,
    TilePosition, WorldMapError, WorldMapId, map_definition, scheduled_people_on_map,
    statement_for_interaction,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardinalDirection {
    North,
    East,
    South,
    West,
}

impl CardinalDirection {
    pub(crate) const fn offset(self) -> (i32, i32) {
        match self {
            Self::North => (0, -1),
            Self::East => (1, 0),
            Self::South => (0, 1),
            Self::West => (-1, 0),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct GameRevision(u64);

impl GameRevision {
    #[must_use]
    pub const fn initial() -> Self {
        Self(0)
    }

    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }

    pub(crate) const fn from_archive(value: u64) -> Self {
        Self(value)
    }

    fn next(self) -> Result<Self, GameplayRuntimeError> {
        self.0
            .checked_add(1)
            .map(Self)
            .ok_or(GameplayRuntimeError::RevisionOverflow)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameplayEventMetadata {
    pub id: GameplayEventId,
    pub causal_position: CausalPosition,
}

/// Commands accepted by the first authoritative gameplay seam.
///
/// Later party and encounter commands should be added here only when their
/// reducers are ready. Presentation-specific commands do not belong here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameplayCommand {
    EstablishHuemanIdentity {
        continuity: BeingContinuityId,
        participant: ParticipantId,
        institutional: InstitutionalBeingId,
    },
    RegisterRegionalBeing {
        continuity: BeingContinuityId,
        participant: ParticipantId,
        institutional: InstitutionalBeingId,
        regional_metadata: RegionalEventMetadata,
        registration: Box<RegionalBeingRegistration>,
    },
    MoveHueman {
        direction: CardinalDirection,
    },
    InteractHueman,
    EnterMap {
        map: WorldMapId,
    },
    TraverseMapExit {
        map: WorldMapId,
    },
    AdvanceLivingWorldShift,
    SupportLivingCase {
        case_id: LivingCaseId,
        choice: LivingCaseChoice,
    },
    AskLivingDutyOfficerToDecide {
        case_id: LivingCaseId,
    },
    SupportDeepPressureSettlement {
        choice: DeepPressureSettlementChoice,
    },
    AskDeepPressureAssemblyToCommit,
    RecruitPartyCandidate {
        candidate: RecruitmentCandidateId,
        path: RecruitmentPath,
    },
    SelectPartyMember {
        actor: PartyActorId,
    },
    SwitchPartyLead {
        actor: PartyActorId,
    },
    UsePartyAction {
        actor: PartyActorId,
        action: PartyActionId,
        target_continuity_id: Option<String>,
    },
    DiscloseFacultyObservation {
        faculty: HuemanFaculty,
    },
    SupportBoardwalkOption {
        choice: BoardwalkChoice,
    },
    AskReturningGoonToDecide,
    SupportStonebendContinuityOption {
        choice: StonebendContinuityChoice,
    },
    AskStonebendToDetermineContinuity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameplayEvent {
    HuemanIdentityEstablished {
        identity: BeingContinuityRecord,
    },
    RegionalBeingIdentityEstablished {
        identity: BeingContinuityRecord,
        regional_event: Box<RegionalEventEnvelope>,
    },
    HuemanMovementResolved {
        from: TilePosition,
        to: TilePosition,
    },
    HuemanInteractionOpened {
        at: TilePosition,
        target: InteractionId,
        evidence: Option<BoardwalkEvidence>,
        stonebend_evidence: Option<StonebendEvidence>,
        living_world_event: Option<LivingWorldEvent>,
        deep_pressure_event: Option<DeepPressureEvent>,
    },
    HuemanMapEntered {
        from: WorldMapId,
        to: WorldMapId,
        at: TilePosition,
    },
    BoardwalkFacultyDisclosed {
        faculty: HuemanFaculty,
    },
    BoardwalkOptionSupported {
        choice: BoardwalkChoice,
    },
    ReturningGoonChoiceCommitted {
        committed_by: ParticipantId,
        choice: BoardwalkChoice,
        outcome_id: BoardwalkOutcomeId,
        relationship_bond: Option<crate::constitutional::BondId>,
    },
    StonebendFacultyDisclosed {
        faculty: HuemanFaculty,
    },
    StonebendContinuityOptionSupported {
        choice: StonebendContinuityChoice,
    },
    StonebendContinuityDeterminationCommitted {
        subject: ParticipantId,
        choice: StonebendContinuityChoice,
        outcome_id: StonebendOutcomeId,
        decision_id: HouseDecisionId,
        authority_actor: AuthorityActorId,
    },
    LivingWorldChanged {
        event: LivingWorldEvent,
        deep_pressure_event: Option<DeepPressureEvent>,
        party_event: Option<PartyEvent>,
    },
    DeepPressureChanged {
        event: DeepPressureEvent,
    },
    PartyChanged {
        event: PartyEvent,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameplayEventEnvelope {
    pub id: GameplayEventId,
    pub sequence: u64,
    pub revision: GameRevision,
    pub causal_position: CausalPosition,
    pub rule_set: RuleSetId,
    pub payload: GameplayEvent,
}

/// Single authoritative aggregate for gameplay-facing identity and subruntime
/// coordination.
///
/// Mutation is private to `execute`: commands are applied to a cloned whole
/// candidate and the candidate replaces live state only after every involved
/// reducer and identity index accepts the operation.
#[derive(Debug, Clone)]
pub struct HollowGroveGameRuntime {
    rule_set: RuleSetId,
    revision: GameRevision,
    events: Vec<GameplayEventEnvelope>,
    event_ids: BTreeSet<GameplayEventId>,
    identities: IdentityRegistry,
    hueman_map: WorldMapId,
    hueman_position: Option<TilePosition>,
    active_interaction: Option<InteractionId>,
    boardwalk_case: Option<BoardwalkCase>,
    stonebend_case: Option<StonebendContinuityCase>,
    living_world: LivingWorldState,
    deep_pressure: DeepPressureState,
    party: PartyState,
    constitutional: ConstitutionalRuntime,
    regional: RegionalSynthesisRuntime,
    world: WorldSession,
}

impl HollowGroveGameRuntime {
    #[must_use]
    pub fn new(rule_set: RuleSetId) -> Self {
        Self::with_world_session(rule_set, WorldSession::canonical())
    }

    #[must_use]
    pub fn with_world_session(rule_set: RuleSetId, world: WorldSession) -> Self {
        Self {
            rule_set,
            revision: GameRevision::initial(),
            events: Vec::new(),
            event_ids: BTreeSet::new(),
            identities: IdentityRegistry::default(),
            hueman_map: WorldMapId::AuraRidgeGroveApproach,
            hueman_position: None,
            active_interaction: None,
            boardwalk_case: None,
            stonebend_case: None,
            living_world: LivingWorldState::canonical()
                .expect("canonical living Hollow Grove state must validate"),
            deep_pressure: DeepPressureState::new(),
            party: PartyState::new(),
            constitutional: ConstitutionalRuntime::new(),
            regional: RegionalSynthesisRuntime::new(),
            world,
        }
    }

    pub fn execute(
        &mut self,
        metadata: GameplayEventMetadata,
        command: GameplayCommand,
    ) -> Result<&GameplayEventEnvelope, GameplayRuntimeError> {
        if let Some(position) = self.retry_position(&metadata, &command)? {
            return Ok(&self.events[position]);
        }
        self.validate_metadata(&metadata)?;

        let mut candidate = self.clone();
        let payload = candidate.reduce_command(&metadata, command)?;
        let sequence = u64::try_from(candidate.events.len())
            .map_err(|_| GameplayRuntimeError::SequenceOverflow)?;
        let revision = candidate.revision.next()?;
        let envelope = GameplayEventEnvelope {
            id: metadata.id,
            sequence,
            revision,
            causal_position: metadata.causal_position,
            rule_set: candidate.rule_set.clone(),
            payload,
        };
        candidate.event_ids.insert(envelope.id.clone());
        candidate.revision = revision;
        candidate.events.push(envelope);

        *self = candidate;
        Ok(self
            .events
            .last()
            .expect("gameplay event was just appended"))
    }

    fn retry_position(
        &self,
        metadata: &GameplayEventMetadata,
        command: &GameplayCommand,
    ) -> Result<Option<usize>, GameplayRuntimeError> {
        let Some(position) = self.events.iter().position(|event| event.id == metadata.id) else {
            return Ok(None);
        };
        let existing = &self.events[position];
        if existing.causal_position == metadata.causal_position
            && Self::event_matches_command(existing, command)
        {
            Ok(Some(position))
        } else {
            Err(GameplayRuntimeError::EventIdConflict(metadata.id.clone()))
        }
    }

    fn event_matches_command(event: &GameplayEventEnvelope, command: &GameplayCommand) -> bool {
        match (&event.payload, command) {
            (
                GameplayEvent::HuemanIdentityEstablished { identity },
                GameplayCommand::EstablishHuemanIdentity {
                    continuity,
                    participant,
                    institutional,
                },
            ) => {
                identity.id() == continuity
                    && identity.incarnation() == &ActiveIncarnationRef::Hueman
                    && identity.domain_refs().participant() == participant
                    && identity.domain_refs().institutional() == institutional
            }
            (
                GameplayEvent::RegionalBeingIdentityEstablished {
                    identity,
                    regional_event,
                },
                GameplayCommand::RegisterRegionalBeing {
                    continuity,
                    participant,
                    institutional,
                    regional_metadata,
                    registration,
                },
            ) => {
                identity.id() == continuity
                    && identity.incarnation().regional_being() == Some(&registration.id)
                    && identity.domain_refs().participant() == participant
                    && identity.domain_refs().institutional() == institutional
                    && regional_event.id == regional_metadata.id
                    && regional_event.causal_position == regional_metadata.causal_position
                    && regional_event.rule_set == regional_metadata.rule_set
                    && matches!(
                        &regional_event.payload,
                        RegionalEvent::BeingRegistered(existing) if existing == registration.as_ref()
                    )
            }
            (
                GameplayEvent::HuemanMovementResolved { to, .. },
                GameplayCommand::MoveHueman { direction },
            ) => to.facing == *direction,
            (GameplayEvent::HuemanInteractionOpened { .. }, GameplayCommand::InteractHueman) => {
                true
            }
            (GameplayEvent::HuemanMapEntered { to, .. }, GameplayCommand::EnterMap { map }) => {
                to == map
            }
            (
                GameplayEvent::HuemanMapEntered { to, .. },
                GameplayCommand::TraverseMapExit { map },
            ) => to == map,
            (
                GameplayEvent::LivingWorldChanged {
                    event: LivingWorldEvent::ShiftAdvanced { .. },
                    ..
                },
                GameplayCommand::AdvanceLivingWorldShift,
            ) => true,
            (
                GameplayEvent::LivingWorldChanged {
                    event:
                        LivingWorldEvent::CaseResolved {
                            case_id: existing_case,
                            ..
                        },
                    ..
                },
                GameplayCommand::AskLivingDutyOfficerToDecide { case_id },
            ) => existing_case == case_id,
            (
                GameplayEvent::LivingWorldChanged {
                    event:
                        LivingWorldEvent::CaseSupportRecorded {
                            case_id: existing_case,
                            choice: existing_choice,
                        },
                    ..
                },
                GameplayCommand::SupportLivingCase { case_id, choice },
            ) => existing_case == case_id && existing_choice == choice,
            (
                GameplayEvent::DeepPressureChanged {
                    event:
                        DeepPressureEvent::SettlementSupportRecorded {
                            choice: existing_choice,
                        },
                },
                GameplayCommand::SupportDeepPressureSettlement { choice },
            ) => existing_choice == choice,
            (
                GameplayEvent::DeepPressureChanged {
                    event: DeepPressureEvent::SettlementCommitted { .. },
                },
                GameplayCommand::AskDeepPressureAssemblyToCommit,
            ) => true,
            (
                GameplayEvent::PartyChanged {
                    event: PartyEvent::RecruitmentDecided { record, .. },
                },
                GameplayCommand::RecruitPartyCandidate { candidate, path },
            ) => record.candidate_id == *candidate && record.recruitment_path == *path,
            (
                GameplayEvent::PartyChanged {
                    event: PartyEvent::MemberSelected { to, .. },
                },
                GameplayCommand::SelectPartyMember { actor },
            ) => to == actor,
            (
                GameplayEvent::PartyChanged {
                    event: PartyEvent::LeadChanged { to, .. },
                },
                GameplayCommand::SwitchPartyLead { actor },
            ) => to == actor,
            (
                GameplayEvent::PartyChanged {
                    event: PartyEvent::FieldActionResolved { record },
                },
                GameplayCommand::UsePartyAction {
                    actor,
                    action,
                    target_continuity_id,
                },
            ) => {
                record.actor == *actor
                    && record.action == *action
                    && record.target_continuity_id == *target_continuity_id
            }
            (
                GameplayEvent::BoardwalkFacultyDisclosed { faculty: existing },
                GameplayCommand::DiscloseFacultyObservation { faculty },
            ) => existing == faculty,
            (
                GameplayEvent::BoardwalkOptionSupported { choice: existing },
                GameplayCommand::SupportBoardwalkOption { choice },
            ) => existing == choice,
            (
                GameplayEvent::ReturningGoonChoiceCommitted { .. },
                GameplayCommand::AskReturningGoonToDecide,
            ) => true,
            (
                GameplayEvent::StonebendFacultyDisclosed { faculty: existing },
                GameplayCommand::DiscloseFacultyObservation { faculty },
            ) => existing == faculty,
            (
                GameplayEvent::StonebendContinuityOptionSupported { choice: existing },
                GameplayCommand::SupportStonebendContinuityOption { choice },
            ) => existing == choice,
            (
                GameplayEvent::StonebendContinuityDeterminationCommitted { .. },
                GameplayCommand::AskStonebendToDetermineContinuity,
            ) => true,
            _ => false,
        }
    }

    fn reduce_command(
        &mut self,
        metadata: &GameplayEventMetadata,
        command: GameplayCommand,
    ) -> Result<GameplayEvent, GameplayRuntimeError> {
        match command {
            GameplayCommand::EstablishHuemanIdentity {
                continuity,
                participant,
                institutional,
            } => {
                let identity =
                    BeingContinuityRecord::hueman(continuity, participant, institutional);
                self.identities.insert(identity.clone())?;
                self.party.establish_hueman(identity.id().as_str())?;
                self.hueman_map = WorldMapId::AuraRidgeGroveApproach;
                self.hueman_position = Some(map_definition(self.hueman_map).spawn);
                Ok(GameplayEvent::HuemanIdentityEstablished { identity })
            }
            GameplayCommand::RegisterRegionalBeing {
                continuity,
                participant,
                institutional,
                regional_metadata,
                registration,
            } => {
                self.validate_child_metadata(metadata, &regional_metadata)?;
                let registration = *registration;
                let regional_event = self
                    .regional
                    .register_being(regional_metadata, registration.clone())?
                    .clone();
                let identity = BeingContinuityRecord::regional(
                    continuity,
                    registration.id,
                    participant,
                    institutional,
                );
                self.identities.insert(identity.clone())?;
                Ok(GameplayEvent::RegionalBeingIdentityEstablished {
                    identity,
                    regional_event: Box::new(regional_event),
                })
            }
            GameplayCommand::MoveHueman { direction } => {
                let from = self
                    .hueman_position
                    .ok_or(GameplayRuntimeError::MovementRequiresHueman)?;
                let resolved_choice = self
                    .boardwalk_case
                    .as_ref()
                    .and_then(BoardwalkCase::committed_choice);
                let resolved_stonebend_choice = self
                    .stonebend_case
                    .as_ref()
                    .and_then(StonebendContinuityCase::committed_choice);
                let mut to = map_definition(self.hueman_map).move_actor_with_cases(
                    from,
                    direction,
                    resolved_choice,
                    resolved_stonebend_choice,
                );
                if self.unrecruited_person_occupies(to) && (to.x != from.x || to.y != from.y) {
                    to.x = from.x;
                    to.y = from.y;
                }
                self.hueman_position = Some(to);
                self.active_interaction = None;
                Ok(GameplayEvent::HuemanMovementResolved { from, to })
            }
            GameplayCommand::InteractHueman => {
                let at = self
                    .hueman_position
                    .ok_or(GameplayRuntimeError::InteractionRequiresHueman)?;
                let target = self
                    .unrecruited_person_in_front(at)
                    .map(InteractionId::DeepPressurePerson)
                    .or_else(|| {
                        map_definition(self.hueman_map).interaction_in_front_with_cases(
                            at,
                            self.boardwalk_case
                                .as_ref()
                                .and_then(BoardwalkCase::committed_choice),
                            self.stonebend_case
                                .as_ref()
                                .and_then(StonebendContinuityCase::committed_choice),
                        )
                    })
                    .ok_or(GameplayRuntimeError::NoInteractionTarget)?;
                let evidence = self
                    .boardwalk_case
                    .as_mut()
                    .and_then(|case| case.observe_interaction(target));
                let stonebend_evidence = self
                    .stonebend_case
                    .as_mut()
                    .and_then(|case| case.observe_interaction(target));
                let living_world_event = self.observe_living_evidence(target)?;
                let deep_pressure_event = if let Some(event) = &living_world_event {
                    self.deep_pressure
                        .observe_living_event(event, self.living_world.clock)?
                } else if let Some(statement) = statement_for_interaction(target) {
                    self.deep_pressure
                        .observe_statement(statement, self.living_world.clock)?
                } else {
                    None
                };
                self.active_interaction = Some(target);
                Ok(GameplayEvent::HuemanInteractionOpened {
                    at,
                    target,
                    evidence,
                    stonebend_evidence,
                    living_world_event,
                    deep_pressure_event,
                })
            }
            GameplayCommand::EnterMap { map } => {
                self.hueman_position
                    .ok_or(GameplayRuntimeError::MapEntryRequiresHueman)?;
                let from = self.hueman_map;
                self.validate_route_transfer(from, map)?;
                self.hueman_map = map;
                let at = map_definition(map).spawn;
                self.hueman_position = Some(at);
                self.active_interaction = None;
                if map == WorldMapId::BoardwalkReturnVestibule && self.boardwalk_case.is_none() {
                    self.boardwalk_case = Some(BoardwalkCase::new());
                }
                if map == WorldMapId::CurrentSeaDeepCertificationLanding
                    && self.stonebend_case.is_none()
                {
                    self.stonebend_case = Some(StonebendContinuityCase::new());
                }
                Ok(GameplayEvent::HuemanMapEntered { from, to: map, at })
            }
            GameplayCommand::TraverseMapExit { map } => {
                let position = self
                    .hueman_position
                    .ok_or(GameplayRuntimeError::MapEntryRequiresHueman)?;
                let from = self.hueman_map;
                if position.x != map_definition(from).spawn.x
                    || position.y != map_definition(from).spawn.y
                {
                    return Err(GameplayRuntimeError::PhysicalExitRequired {
                        map: from.as_str().into(),
                        exit_x: map_definition(from).spawn.x,
                        exit_y: map_definition(from).spawn.y,
                    });
                }
                self.validate_route_transfer(from, map)?;
                self.enter_map(map);
                let at = self
                    .hueman_position
                    .expect("entering a map establishes the spawn");
                Ok(GameplayEvent::HuemanMapEntered { from, to: map, at })
            }
            GameplayCommand::AdvanceLivingWorldShift => {
                let event = self.living_world.advance_shift()?;
                self.active_interaction = None;
                let deep_pressure_event = self
                    .deep_pressure
                    .observe_living_event(&event, self.living_world.clock)?;
                let party_event = self.party.advance_shift()?;
                Ok(GameplayEvent::LivingWorldChanged {
                    event,
                    deep_pressure_event,
                    party_event,
                })
            }
            GameplayCommand::SupportLivingCase { case_id, choice } => {
                if !map_hosts_case(self.hueman_map, case_id) {
                    return Err(GameplayRuntimeError::LivingCaseLocationRequired {
                        case_id,
                        map: self.hueman_map.as_str().into(),
                    });
                }
                let event = self.living_world.support(case_id, choice)?;
                let deep_pressure_event = self
                    .deep_pressure
                    .observe_living_event(&event, self.living_world.clock)?;
                Ok(GameplayEvent::LivingWorldChanged {
                    event,
                    deep_pressure_event,
                    party_event: None,
                })
            }
            GameplayCommand::AskLivingDutyOfficerToDecide { case_id } => {
                if !map_hosts_case(self.hueman_map, case_id) {
                    return Err(GameplayRuntimeError::LivingCaseLocationRequired {
                        case_id,
                        map: self.hueman_map.as_str().into(),
                    });
                }
                let event = self.living_world.commit_duty_decision(case_id)?;
                let deep_pressure_event = self
                    .deep_pressure
                    .observe_living_event(&event, self.living_world.clock)?;
                Ok(GameplayEvent::LivingWorldChanged {
                    event,
                    deep_pressure_event,
                    party_event: None,
                })
            }
            GameplayCommand::SupportDeepPressureSettlement { choice } => {
                if self.hueman_map != WorldMapId::BoardwalkReturnVestibule {
                    return Err(GameplayRuntimeError::DeepPressureSettlementLocationRequired);
                }
                let event = self.deep_pressure.support_settlement(choice)?;
                Ok(GameplayEvent::DeepPressureChanged { event })
            }
            GameplayCommand::AskDeepPressureAssemblyToCommit => {
                if self.hueman_map != WorldMapId::BoardwalkReturnVestibule {
                    return Err(GameplayRuntimeError::DeepPressureSettlementLocationRequired);
                }
                let event = self.deep_pressure.commit_settlement(
                    metadata.causal_position,
                    &self.rule_set,
                    &mut self.constitutional,
                    &self.world,
                )?;
                Ok(GameplayEvent::DeepPressureChanged { event })
            }
            GameplayCommand::RecruitPartyCandidate { candidate, path } => {
                let event = self.recruit_party_candidate(candidate, path)?;
                self.active_interaction = None;
                Ok(GameplayEvent::PartyChanged { event })
            }
            GameplayCommand::SelectPartyMember { actor } => {
                let event = self.party.select(actor)?;
                Ok(GameplayEvent::PartyChanged { event })
            }
            GameplayCommand::SwitchPartyLead { actor } => {
                let event = self.party.switch_lead(actor)?;
                Ok(GameplayEvent::PartyChanged { event })
            }
            GameplayCommand::UsePartyAction {
                actor,
                action,
                target_continuity_id,
            } => {
                let event = self.party.use_action(
                    actor,
                    action,
                    self.hueman_map,
                    target_continuity_id,
                    &self.deep_pressure,
                )?;
                Ok(GameplayEvent::PartyChanged { event })
            }
            GameplayCommand::DiscloseFacultyObservation { faculty } => match self.hueman_map {
                WorldMapId::BoardwalkReturnVestibule => {
                    self.require_boardwalk_case()?.disclose_faculty(faculty)?;
                    Ok(GameplayEvent::BoardwalkFacultyDisclosed { faculty })
                }
                WorldMapId::CurrentSeaDeepCertificationLanding => {
                    self.require_stonebend_case()?.disclose_faculty(faculty)?;
                    Ok(GameplayEvent::StonebendFacultyDisclosed { faculty })
                }
                _ => Err(GameplayRuntimeError::CaseRequired),
            },
            GameplayCommand::SupportBoardwalkOption { choice } => {
                self.require_boardwalk_case()?.support(choice)?;
                Ok(GameplayEvent::BoardwalkOptionSupported { choice })
            }
            GameplayCommand::AskReturningGoonToDecide => {
                let rule_set = self.rule_set.clone();
                let authority_world = self.world.clone();
                let case = self
                    .boardwalk_case
                    .as_mut()
                    .ok_or(GameplayRuntimeError::BoardwalkCaseRequired)?;
                let choice = case.commit_returning_goon_choice_with_authority(
                    metadata.causal_position,
                    &rule_set,
                    &mut self.constitutional,
                    &authority_world,
                )?;
                let committed_by = ParticipantId::new(RETURNING_GOON_PARTICIPANT_ID)
                    .expect("canonical Returning Goon participant ID");
                Ok(GameplayEvent::ReturningGoonChoiceCommitted {
                    committed_by,
                    choice,
                    outcome_id: case
                        .outcome()
                        .expect("a committed Boardwalk choice has an outcome")
                        .id,
                    relationship_bond: case
                        .outcome()
                        .and_then(|outcome| outcome.relationship.as_ref())
                        .map(|commit| commit.bond.clone()),
                })
            }
            GameplayCommand::SupportStonebendContinuityOption { choice } => {
                self.require_stonebend_case()?.support(choice)?;
                Ok(GameplayEvent::StonebendContinuityOptionSupported { choice })
            }
            GameplayCommand::AskStonebendToDetermineContinuity => {
                let authority_world = self.world.clone();
                let case = self
                    .stonebend_case
                    .as_mut()
                    .ok_or(GameplayRuntimeError::StonebendCaseRequired)?;
                let choice =
                    case.commit_with_authority(metadata.causal_position, &authority_world)?;
                let outcome = case
                    .outcome()
                    .expect("a committed Stonebend choice has an outcome");
                Ok(GameplayEvent::StonebendContinuityDeterminationCommitted {
                    subject: ParticipantId::new(MERCY_DEEP_PARTICIPANT_ID)
                        .expect("canonical Mercy Deep participant ID"),
                    choice,
                    outcome_id: outcome.id,
                    decision_id: outcome.stonebend_naming.id.clone(),
                    authority_actor: outcome.stonebend_naming.authority.actor.clone(),
                })
            }
        }
    }

    fn enter_map(&mut self, map: WorldMapId) {
        self.hueman_map = map;
        self.hueman_position = Some(map_definition(map).spawn);
        self.active_interaction = None;
        if map == WorldMapId::BoardwalkReturnVestibule && self.boardwalk_case.is_none() {
            self.boardwalk_case = Some(BoardwalkCase::new());
        }
        if map == WorldMapId::CurrentSeaDeepCertificationLanding && self.stonebend_case.is_none() {
            self.stonebend_case = Some(StonebendContinuityCase::new());
        }
    }

    fn observe_living_evidence(
        &mut self,
        target: InteractionId,
    ) -> Result<Option<LivingWorldEvent>, GameplayRuntimeError> {
        let Some((case_id, evidence)) = living_evidence_for(target) else {
            return Ok(None);
        };
        let already_observed = self.living_world.cases.get(&case_id).is_some_and(|case| {
            case.resolved_choice.is_some() || case.evidence.contains(&evidence)
        });
        if already_observed {
            return Ok(None);
        }
        self.living_world
            .observe(case_id, evidence)
            .map(Some)
            .map_err(Into::into)
    }

    fn recruit_party_candidate(
        &mut self,
        candidate: RecruitmentCandidateId,
        path: RecruitmentPath,
    ) -> Result<PartyEvent, GameplayRuntimeError> {
        let person = candidate.person();
        if self.active_interaction != Some(InteractionId::DeepPressurePerson(person)) {
            return Err(GameplayRuntimeError::RecruitmentConversationRequired(
                candidate,
            ));
        }
        if !self
            .scheduled_people()
            .iter()
            .any(|presence| presence.person_id == person)
        {
            return Err(GameplayRuntimeError::RecruitmentConversationRequired(
                candidate,
            ));
        }
        let outcome = self
            .deep_pressure
            .outcome
            .as_ref()
            .ok_or(GameplayRuntimeError::DeepPressureOutcomeRequiredForRecruitment)?;
        let relationship = self
            .deep_pressure
            .relationships
            .get(&person)
            .ok_or(GameplayRuntimeError::DeepPressureOutcomeRequiredForRecruitment)?;
        self.party
            .request_recruitment(candidate, path, relationship, outcome)
            .map_err(Into::into)
    }

    fn unrecruited_person_in_front(
        &self,
        hueman: TilePosition,
    ) -> Option<super::DeepPressurePersonId> {
        let (dx, dy) = hueman.facing.offset();
        let x = i32::from(hueman.x) + dx;
        let y = i32::from(hueman.y) + dy;
        self.scheduled_people()
            .into_iter()
            .find(|person| i32::from(person.position.x) == x && i32::from(person.position.y) == y)
            .map(|person| person.person_id)
    }

    fn unrecruited_person_occupies(&self, position: TilePosition) -> bool {
        self.scheduled_people()
            .iter()
            .any(|person| person.position.x == position.x && person.position.y == position.y)
    }

    fn require_boardwalk_case(&mut self) -> Result<&mut BoardwalkCase, GameplayRuntimeError> {
        if self.hueman_map != WorldMapId::BoardwalkReturnVestibule {
            return Err(GameplayRuntimeError::BoardwalkCaseRequired);
        }
        self.boardwalk_case
            .as_mut()
            .ok_or(GameplayRuntimeError::BoardwalkCaseRequired)
    }

    fn require_stonebend_case(
        &mut self,
    ) -> Result<&mut StonebendContinuityCase, GameplayRuntimeError> {
        if self.hueman_map != WorldMapId::CurrentSeaDeepCertificationLanding {
            return Err(GameplayRuntimeError::StonebendCaseRequired);
        }
        self.stonebend_case
            .as_mut()
            .ok_or(GameplayRuntimeError::StonebendCaseRequired)
    }

    fn validate_metadata(
        &self,
        metadata: &GameplayEventMetadata,
    ) -> Result<(), GameplayRuntimeError> {
        debug_assert!(!self.event_ids.contains(&metadata.id));
        if let Some(last) = self.events.last()
            && metadata.causal_position < last.causal_position
        {
            return Err(GameplayRuntimeError::CausalRegression {
                previous: last.causal_position,
                actual: metadata.causal_position,
            });
        }
        Ok(())
    }

    fn validate_route_transfer(
        &self,
        from: WorldMapId,
        to: WorldMapId,
    ) -> Result<(), GameplayRuntimeError> {
        if !to.is_canonical() {
            return Err(WorldMapError::NonCanonicalMap(to.as_str().into()).into());
        }
        if let Some(site) = from.extraction() {
            let permitted = to.route() == Some(site.route())
                || (site == ExtractionSiteId::StairwayBurdenMine
                    && to.extraction() == Some(ExtractionSiteId::HighwayToHellDeepworks))
                || (site == ExtractionSiteId::HighwayToHellDeepworks
                    && to.extraction() == Some(ExtractionSiteId::StairwayBurdenMine))
                || to.extraction() == Some(site);
            if permitted {
                return Ok(());
            }
            return Err(GameplayRuntimeError::DisconnectedMapTransfer {
                from: from.as_str().into(),
                to: to.as_str().into(),
            });
        }
        if let Some(site) = to.extraction() {
            if site != ExtractionSiteId::HighwayToHellDeepworks
                && from.route() == Some(site.route())
            {
                return Ok(());
            }
            return Err(GameplayRuntimeError::DisconnectedMapTransfer {
                from: from.as_str().into(),
                to: to.as_str().into(),
            });
        }
        let network = RouteNetwork::canonical()?;
        match (from.route(), to.route()) {
            (Some(from_route), Some(to_route)) => {
                if !network.can_transfer(from_route, to_route) {
                    return Err(GameplayRuntimeError::DisconnectedRouteTransfer {
                        from: from_route,
                        to: to_route,
                    });
                }
            }
            (Some(route), None) => {
                let Some(surface) = to.surface() else {
                    return Err(GameplayRuntimeError::DisconnectedMapTransfer {
                        from: from.as_str().into(),
                        to: to.as_str().into(),
                    });
                };
                if !self.surface_permits_route(surface, route)? {
                    return Err(GameplayRuntimeError::DisconnectedMapTransfer {
                        from: from.as_str().into(),
                        to: to.as_str().into(),
                    });
                }
            }
            (None, Some(route)) => {
                let Some(surface) = from.surface() else {
                    return Err(GameplayRuntimeError::DisconnectedMapTransfer {
                        from: from.as_str().into(),
                        to: to.as_str().into(),
                    });
                };
                if !self.surface_permits_route(surface, route)? {
                    return Err(GameplayRuntimeError::DisconnectedMapTransfer {
                        from: from.as_str().into(),
                        to: to.as_str().into(),
                    });
                }
            }
            (None, None) if from == to => {}
            _ => {
                return Err(GameplayRuntimeError::DisconnectedMapTransfer {
                    from: from.as_str().into(),
                    to: to.as_str().into(),
                });
            }
        }
        Ok(())
    }

    fn surface_permits_route(
        &self,
        surface: InteriorSurfaceId,
        route: ConstitutionalRouteId,
    ) -> Result<bool, GameplayRuntimeError> {
        match surface {
            InteriorSurfaceId::AuraField => Ok(canonical_aura_field()?.permits_route_entry(route)),
            InteriorSurfaceId::AuraBeach => Ok(canonical_aura_beach()?.permits_route_entry(route)),
            InteriorSurfaceId::AuraBasin => Ok(canonical_aura_basin()?.permits_route_entry(route)),
        }
    }

    fn validate_child_metadata(
        &self,
        gameplay: &GameplayEventMetadata,
        regional: &RegionalEventMetadata,
    ) -> Result<(), GameplayRuntimeError> {
        if regional.rule_set != self.rule_set {
            return Err(GameplayRuntimeError::RuleSetMismatch {
                expected: self.rule_set.clone(),
                actual: regional.rule_set.clone(),
            });
        }
        if regional.causal_position != gameplay.causal_position {
            return Err(GameplayRuntimeError::ChildCausalPositionMismatch {
                gameplay: gameplay.causal_position,
                regional: regional.causal_position,
            });
        }
        Ok(())
    }

    pub fn replay(
        rule_set: RuleSetId,
        events: impl IntoIterator<Item = GameplayEventEnvelope>,
    ) -> Result<Self, GameplayRuntimeError> {
        Self::replay_with_world_session(rule_set, events, WorldSession::canonical())
    }

    pub fn replay_with_world_session(
        rule_set: RuleSetId,
        events: impl IntoIterator<Item = GameplayEventEnvelope>,
        world: WorldSession,
    ) -> Result<Self, GameplayRuntimeError> {
        let mut runtime = Self::with_world_session(rule_set, world);
        for expected in events {
            runtime.replay_event(expected)?;
        }
        Ok(runtime)
    }

    fn replay_event(
        &mut self,
        expected: GameplayEventEnvelope,
    ) -> Result<(), GameplayRuntimeError> {
        let metadata = GameplayEventMetadata {
            id: expected.id.clone(),
            causal_position: expected.causal_position,
        };
        self.validate_metadata(&metadata)?;
        if expected.rule_set != self.rule_set {
            return Err(GameplayRuntimeError::RuleSetMismatch {
                expected: self.rule_set.clone(),
                actual: expected.rule_set,
            });
        }
        let expected_sequence =
            u64::try_from(self.events.len()).map_err(|_| GameplayRuntimeError::SequenceOverflow)?;
        if expected.sequence != expected_sequence {
            return Err(GameplayRuntimeError::ReplaySequenceMismatch {
                expected: expected_sequence,
                actual: expected.sequence,
            });
        }
        let expected_revision = self.revision.next()?;
        if expected.revision != expected_revision {
            return Err(GameplayRuntimeError::ReplayRevisionMismatch {
                expected: expected_revision,
                actual: expected.revision,
            });
        }

        self.apply_replayed_payload(expected.causal_position, &expected.payload)?;
        self.event_ids.insert(expected.id.clone());
        self.revision = expected.revision;
        self.events.push(expected);
        Ok(())
    }

    fn apply_replayed_payload(
        &mut self,
        causal_position: CausalPosition,
        payload: &GameplayEvent,
    ) -> Result<(), GameplayRuntimeError> {
        match payload {
            GameplayEvent::HuemanIdentityEstablished { identity } => {
                if identity.incarnation() != &ActiveIncarnationRef::Hueman {
                    return Err(GameplayRuntimeError::IdentityPayloadMismatch);
                }
                self.identities.insert(identity.clone())?;
                self.party.establish_hueman(identity.id().as_str())?;
                self.hueman_map = WorldMapId::AuraRidgeGroveApproach;
                self.hueman_position = Some(map_definition(self.hueman_map).spawn);
            }
            GameplayEvent::RegionalBeingIdentityEstablished {
                identity,
                regional_event,
            } => {
                if regional_event.rule_set != self.rule_set {
                    return Err(GameplayRuntimeError::RuleSetMismatch {
                        expected: self.rule_set.clone(),
                        actual: regional_event.rule_set.clone(),
                    });
                }
                if regional_event.causal_position != causal_position {
                    return Err(GameplayRuntimeError::ChildCausalPositionMismatch {
                        gameplay: causal_position,
                        regional: regional_event.causal_position,
                    });
                }
                let RegionalEvent::BeingRegistered(registration) = &regional_event.payload else {
                    return Err(GameplayRuntimeError::UnexpectedRegionalEvent);
                };
                if identity.incarnation().regional_being() != Some(&registration.id) {
                    return Err(GameplayRuntimeError::IdentityPayloadMismatch);
                }
                let actual = self
                    .regional
                    .register_being(
                        RegionalEventMetadata {
                            id: regional_event.id.clone(),
                            causal_position: regional_event.causal_position,
                            rule_set: regional_event.rule_set.clone(),
                        },
                        registration.clone(),
                    )?
                    .clone();
                if actual != **regional_event {
                    return Err(GameplayRuntimeError::RegionalReplayDivergence(
                        regional_event.id.clone(),
                    ));
                }
                self.identities.insert(identity.clone())?;
            }
            GameplayEvent::HuemanMovementResolved { from, to } => {
                let current = self
                    .hueman_position
                    .ok_or(GameplayRuntimeError::MovementRequiresHueman)?;
                let resolved_choice = self
                    .boardwalk_case
                    .as_ref()
                    .and_then(BoardwalkCase::committed_choice);
                let resolved_stonebend_choice = self
                    .stonebend_case
                    .as_ref()
                    .and_then(StonebendContinuityCase::committed_choice);
                let mut expected = map_definition(self.hueman_map).move_actor_with_cases(
                    current,
                    to.facing,
                    resolved_choice,
                    resolved_stonebend_choice,
                );
                if self.unrecruited_person_occupies(expected)
                    && (expected.x != current.x || expected.y != current.y)
                {
                    expected.x = current.x;
                    expected.y = current.y;
                }
                if current != *from || expected != *to {
                    return Err(GameplayRuntimeError::MovementReplayDivergence);
                }
                self.hueman_position = Some(*to);
                self.active_interaction = None;
            }
            GameplayEvent::HuemanInteractionOpened {
                at,
                target,
                evidence,
                stonebend_evidence,
                living_world_event,
                deep_pressure_event,
            } => {
                let current = self
                    .hueman_position
                    .ok_or(GameplayRuntimeError::InteractionRequiresHueman)?;
                let resolved_choice = self
                    .boardwalk_case
                    .as_ref()
                    .and_then(BoardwalkCase::committed_choice);
                let resolved_stonebend_choice = self
                    .stonebend_case
                    .as_ref()
                    .and_then(StonebendContinuityCase::committed_choice);
                let expected_target = self
                    .unrecruited_person_in_front(current)
                    .map(InteractionId::DeepPressurePerson)
                    .or_else(|| {
                        map_definition(self.hueman_map).interaction_in_front_with_cases(
                            current,
                            resolved_choice,
                            resolved_stonebend_choice,
                        )
                    });
                if current != *at || expected_target != Some(*target) {
                    return Err(GameplayRuntimeError::InteractionReplayDivergence);
                }
                let actual_evidence = self
                    .boardwalk_case
                    .as_mut()
                    .and_then(|case| case.observe_interaction(*target));
                if actual_evidence != *evidence {
                    return Err(GameplayRuntimeError::InteractionReplayDivergence);
                }
                let actual_stonebend_evidence = self
                    .stonebend_case
                    .as_mut()
                    .and_then(|case| case.observe_interaction(*target));
                if actual_stonebend_evidence != *stonebend_evidence {
                    return Err(GameplayRuntimeError::InteractionReplayDivergence);
                }
                let actual_living_event = self.observe_living_evidence(*target)?;
                if actual_living_event != *living_world_event {
                    return Err(GameplayRuntimeError::LivingWorldReplayDivergence);
                }
                let actual_deep_pressure_event = if let Some(event) = &actual_living_event {
                    self.deep_pressure
                        .observe_living_event(event, self.living_world.clock)?
                } else if let Some(statement) = statement_for_interaction(*target) {
                    self.deep_pressure
                        .observe_statement(statement, self.living_world.clock)?
                } else {
                    None
                };
                if actual_deep_pressure_event != *deep_pressure_event {
                    return Err(GameplayRuntimeError::DeepPressureReplayDivergence);
                }
                self.active_interaction = Some(*target);
            }
            GameplayEvent::HuemanMapEntered { from, to, at } => {
                self.hueman_position
                    .ok_or(GameplayRuntimeError::MapEntryRequiresHueman)?;
                if self.hueman_map != *from || map_definition(*to).spawn != *at {
                    return Err(GameplayRuntimeError::MapReplayDivergence);
                }
                self.validate_route_transfer(*from, *to)?;
                self.enter_map(*to);
            }
            GameplayEvent::BoardwalkFacultyDisclosed { faculty } => {
                self.require_boardwalk_case()?.disclose_faculty(*faculty)?;
            }
            GameplayEvent::BoardwalkOptionSupported { choice } => {
                self.require_boardwalk_case()?.support(*choice)?;
            }
            GameplayEvent::ReturningGoonChoiceCommitted {
                committed_by,
                choice,
                outcome_id,
                relationship_bond,
            } => {
                if committed_by.as_str() != RETURNING_GOON_PARTICIPANT_ID {
                    return Err(GameplayRuntimeError::BoardwalkReplayDivergence);
                }
                let rule_set = self.rule_set.clone();
                let authority_world = self.world.clone();
                let case = self
                    .boardwalk_case
                    .as_mut()
                    .ok_or(GameplayRuntimeError::BoardwalkCaseRequired)?;
                let actual = case.commit_returning_goon_choice_with_authority(
                    causal_position,
                    &rule_set,
                    &mut self.constitutional,
                    &authority_world,
                )?;
                let outcome = case
                    .outcome()
                    .expect("a committed Boardwalk choice has an outcome");
                let actual_bond = outcome
                    .relationship
                    .as_ref()
                    .map(|commit| commit.bond.clone());
                if actual != *choice
                    || outcome.id != *outcome_id
                    || actual_bond != *relationship_bond
                {
                    return Err(GameplayRuntimeError::BoardwalkReplayDivergence);
                }
            }
            GameplayEvent::StonebendFacultyDisclosed { faculty } => {
                self.require_stonebend_case()?.disclose_faculty(*faculty)?;
            }
            GameplayEvent::StonebendContinuityOptionSupported { choice } => {
                self.require_stonebend_case()?.support(*choice)?;
            }
            GameplayEvent::StonebendContinuityDeterminationCommitted {
                subject,
                choice,
                outcome_id,
                decision_id,
                authority_actor,
            } => {
                if subject.as_str() != MERCY_DEEP_PARTICIPANT_ID {
                    return Err(GameplayRuntimeError::StonebendReplayDivergence);
                }
                let authority_world = self.world.clone();
                let case = self
                    .stonebend_case
                    .as_mut()
                    .ok_or(GameplayRuntimeError::StonebendCaseRequired)?;
                let actual = case.commit_with_authority(causal_position, &authority_world)?;
                let outcome = case
                    .outcome()
                    .expect("a committed Stonebend choice has an outcome");
                if actual != *choice
                    || outcome.id != *outcome_id
                    || outcome.stonebend_naming.id != *decision_id
                    || outcome.stonebend_naming.authority.actor != *authority_actor
                {
                    return Err(GameplayRuntimeError::StonebendReplayDivergence);
                }
            }
            GameplayEvent::LivingWorldChanged {
                event,
                deep_pressure_event,
                party_event,
            } => {
                self.living_world.apply(event)?;
                if matches!(event, LivingWorldEvent::ShiftAdvanced { .. }) {
                    self.active_interaction = None;
                }
                let actual = self
                    .deep_pressure
                    .observe_living_event(event, self.living_world.clock)?;
                if actual != *deep_pressure_event {
                    return Err(GameplayRuntimeError::DeepPressureReplayDivergence);
                }
                let actual_party = if matches!(event, LivingWorldEvent::ShiftAdvanced { .. }) {
                    self.party.advance_shift()?
                } else {
                    None
                };
                if actual_party != *party_event {
                    return Err(GameplayRuntimeError::PartyReplayDivergence);
                }
            }
            GameplayEvent::DeepPressureChanged { event } => {
                let actual = match event {
                    DeepPressureEvent::SettlementSupportRecorded { choice } => {
                        self.deep_pressure.support_settlement(*choice)?
                    }
                    DeepPressureEvent::SettlementCommitted { .. } => {
                        self.deep_pressure.commit_settlement(
                            causal_position,
                            &self.rule_set,
                            &mut self.constitutional,
                            &self.world,
                        )?
                    }
                    DeepPressureEvent::EvidenceJournaled { .. }
                    | DeepPressureEvent::OperationalResolutionIntegrated { .. } => {
                        return Err(GameplayRuntimeError::DeepPressureReplayDivergence);
                    }
                };
                if actual != *event {
                    return Err(GameplayRuntimeError::DeepPressureReplayDivergence);
                }
            }
            GameplayEvent::PartyChanged { event } => {
                let actual = match event {
                    PartyEvent::RecruitmentDecided { record, .. } => {
                        self.recruit_party_candidate(record.candidate_id, record.recruitment_path)?
                    }
                    PartyEvent::MemberSelected { to, .. } => self.party.select(*to)?,
                    PartyEvent::LeadChanged { to, .. } => self.party.switch_lead(*to)?,
                    PartyEvent::FieldActionResolved { record } => self.party.use_action(
                        record.actor,
                        record.action,
                        self.hueman_map,
                        record.target_continuity_id.clone(),
                        &self.deep_pressure,
                    )?,
                    PartyEvent::ShiftRecoveryApplied { .. } => {
                        return Err(GameplayRuntimeError::PartyReplayDivergence);
                    }
                };
                if actual != *event {
                    return Err(GameplayRuntimeError::PartyReplayDivergence);
                }
                if matches!(event, PartyEvent::RecruitmentDecided { .. }) {
                    self.active_interaction = None;
                }
            }
        }
        Ok(())
    }

    #[must_use]
    pub const fn rule_set(&self) -> &RuleSetId {
        &self.rule_set
    }

    #[must_use]
    pub const fn world_session(&self) -> &WorldSession {
        &self.world
    }

    #[must_use]
    pub const fn revision(&self) -> GameRevision {
        self.revision
    }

    #[must_use]
    pub fn events(&self) -> &[GameplayEventEnvelope] {
        &self.events
    }

    #[must_use]
    pub fn identity_count(&self) -> usize {
        self.identities.len()
    }

    #[must_use]
    pub fn identity(&self, id: &BeingContinuityId) -> Option<&BeingContinuityRecord> {
        self.identities.get(id)
    }

    #[must_use]
    pub fn hueman(&self) -> Option<&BeingContinuityRecord> {
        self.identities.hueman()
    }

    #[must_use]
    pub const fn hueman_position(&self) -> Option<TilePosition> {
        self.hueman_position
    }

    #[must_use]
    pub const fn hueman_map(&self) -> WorldMapId {
        self.hueman_map
    }

    #[must_use]
    pub const fn active_interaction(&self) -> Option<InteractionId> {
        self.active_interaction
    }

    #[must_use]
    pub const fn boardwalk_case(&self) -> Option<&BoardwalkCase> {
        self.boardwalk_case.as_ref()
    }

    #[must_use]
    pub const fn stonebend_case(&self) -> Option<&StonebendContinuityCase> {
        self.stonebend_case.as_ref()
    }

    #[must_use]
    pub const fn living_world(&self) -> &LivingWorldState {
        &self.living_world
    }

    #[must_use]
    pub const fn deep_pressure(&self) -> &DeepPressureState {
        &self.deep_pressure
    }

    #[must_use]
    pub const fn party(&self) -> &PartyState {
        &self.party
    }

    #[must_use]
    pub fn scheduled_people(&self) -> Vec<super::DeepPressurePersonPresence> {
        scheduled_people_on_map(&self.living_world, self.hueman_map)
            .into_iter()
            .filter(|presence| !self.party.recruited_person(presence.person_id))
            .collect()
    }

    #[must_use]
    pub fn physical_exit_destinations(&self) -> Vec<WorldMapId> {
        let mut candidates: Vec<_> = ConstitutionalRouteId::ALL
            .into_iter()
            .map(WorldMapId::for_route)
            .collect();
        candidates.extend([
            WorldMapId::AuraFieldWorkingLand,
            WorldMapId::AuraBeachCoastalCommons,
            WorldMapId::AuraBasinCollisionGrounds,
        ]);
        candidates.extend(
            ExtractionSiteId::ALL
                .into_iter()
                .map(WorldMapId::ExtractionSite),
        );
        candidates
            .into_iter()
            .filter(|candidate| {
                *candidate != self.hueman_map
                    && self
                        .validate_route_transfer(self.hueman_map, *candidate)
                        .is_ok()
            })
            .collect()
    }

    #[must_use]
    pub fn identity_for_regional(&self, id: &RegionalBeingId) -> Option<&BeingContinuityRecord> {
        self.identities.by_regional(id)
    }

    #[must_use]
    pub const fn constitutional(&self) -> &ConstitutionalRuntime {
        &self.constitutional
    }

    #[must_use]
    pub const fn regional(&self) -> &RegionalSynthesisRuntime {
        &self.regional
    }

    #[must_use]
    pub const fn world(&self) -> &WorldSession {
        &self.world
    }
}

fn map_hosts_case(map: WorldMapId, case_id: LivingCaseId) -> bool {
    matches!(
        (map, case_id),
        (
            WorldMapId::AuraFieldWorkingLand,
            LivingCaseId::AuraFieldDroughtAllocation
        ) | (
            WorldMapId::AuraBeachCoastalCommons,
            LivingCaseId::AuraBeachStormRescue
        ) | (
            WorldMapId::AuraBasinCollisionGrounds,
            LivingCaseId::AuraBasinInjuredBeing
        ) | (
            WorldMapId::ExtractionSite(ExtractionSiteId::MntAuraHighMine),
            LivingCaseId::MntAuraRoofFall
        ) | (
            WorldMapId::ExtractionSite(ExtractionSiteId::HighwayToHellDeepworks),
            LivingCaseId::HighwayToHellGasPocket
        ) | (
            WorldMapId::ExtractionSite(ExtractionSiteId::RiptideRecoveryRig),
            LivingCaseId::RiptideWellBlowout
        ) | (
            WorldMapId::ExtractionSite(ExtractionSiteId::CurrentSeaDepthRig),
            LivingCaseId::CurrentSeaWellCertification
        )
    )
}

fn living_evidence_for(target: InteractionId) -> Option<(LivingCaseId, LivingEvidence)> {
    use LivingCaseId as Case;
    use LivingEvidence as Evidence;

    match target {
        InteractionId::AuraFieldFacility(AuraFieldFacilityId::IrrigationWorks) => {
            Some((Case::AuraFieldDroughtAllocation, Evidence::FieldWaterGauge))
        }
        InteractionId::AuraFieldFacility(AuraFieldFacilityId::ProvingPlots) => {
            Some((Case::AuraFieldDroughtAllocation, Evidence::FieldSoilProbe))
        }
        InteractionId::AuraFieldFacility(AuraFieldFacilityId::Granary) => Some((
            Case::AuraFieldDroughtAllocation,
            Evidence::FieldGranaryLedger,
        )),
        InteractionId::AuraBeachFacility(AuraBeachFacilityId::TideStation) => {
            Some((Case::AuraBeachStormRescue, Evidence::BeachTideRecord))
        }
        InteractionId::AuraBeachFacility(AuraBeachFacilityId::WeatherStation) => {
            Some((Case::AuraBeachStormRescue, Evidence::BeachWeatherRecord))
        }
        InteractionId::AuraBeachFacility(AuraBeachFacilityId::RescuePost) => {
            Some((Case::AuraBeachStormRescue, Evidence::BeachRescueManifest))
        }
        InteractionId::AuraBasinFacility(AuraBasinFacilityId::TriageEvacuationPoint) => {
            Some((Case::AuraBasinInjuredBeing, Evidence::BasinVitalSigns))
        }
        InteractionId::AuraBasinFacility(AuraBasinFacilityId::LawfulHollowingStation) => {
            Some((Case::AuraBasinInjuredBeing, Evidence::BasinContinuityRecord))
        }
        InteractionId::AuraBasinFacility(AuraBasinFacilityId::SalvageDepot) => {
            Some((Case::AuraBasinInjuredBeing, Evidence::BasinSalvageClaim))
        }
        InteractionId::ExtractionFacility {
            site: ExtractionSiteId::MntAuraHighMine,
            facility: ExtractionFacilityId::SurveyOffice,
        } => Some((Case::MntAuraRoofFall, Evidence::MntAuraSurvey)),
        InteractionId::ExtractionFacility {
            site: ExtractionSiteId::MntAuraHighMine,
            facility: ExtractionFacilityId::WorkingFace,
        } => Some((Case::MntAuraRoofFall, Evidence::MntAuraSupportInspection)),
        InteractionId::ExtractionFacility {
            site: ExtractionSiteId::MntAuraHighMine,
            facility: ExtractionFacilityId::RefugeChamber,
        } => Some((Case::MntAuraRoofFall, Evidence::MntAuraCrewRoll)),
        InteractionId::ExtractionFacility {
            site: ExtractionSiteId::HighwayToHellDeepworks,
            facility: ExtractionFacilityId::WorkingFace,
        } => Some((Case::HighwayToHellGasPocket, Evidence::HighwayGasReading)),
        InteractionId::ExtractionFacility {
            site: ExtractionSiteId::HighwayToHellDeepworks,
            facility: ExtractionFacilityId::VentilationHouse,
        } => Some((
            Case::HighwayToHellGasPocket,
            Evidence::HighwayVentilationLog,
        )),
        InteractionId::ExtractionFacility {
            site: ExtractionSiteId::HighwayToHellDeepworks,
            facility: ExtractionFacilityId::RefugeChamber,
        } => Some((Case::HighwayToHellGasPocket, Evidence::HighwayEscapeCheck)),
        InteractionId::ExtractionFacility {
            site: ExtractionSiteId::RiptideRecoveryRig,
            facility: ExtractionFacilityId::PressureControl,
        } => Some((Case::RiptideWellBlowout, Evidence::RiptideWellPressure)),
        InteractionId::ExtractionFacility {
            site: ExtractionSiteId::RiptideRecoveryRig,
            facility: ExtractionFacilityId::SpillBoomDepot,
        } => Some((Case::RiptideWellBlowout, Evidence::RiptideSpillExtent)),
        InteractionId::ExtractionFacility {
            site: ExtractionSiteId::RiptideRecoveryRig,
            facility: ExtractionFacilityId::DiveAndRescueBay,
        } => Some((Case::RiptideWellBlowout, Evidence::RiptideCrewManifest)),
        InteractionId::ExtractionFacility {
            site: ExtractionSiteId::CurrentSeaDepthRig,
            facility: ExtractionFacilityId::PressureControl,
        } => Some((
            Case::CurrentSeaWellCertification,
            Evidence::CurrentSeaPressureTest,
        )),
        InteractionId::ExtractionFacility {
            site: ExtractionSiteId::CurrentSeaDepthRig,
            facility: ExtractionFacilityId::CertificationLaboratory,
        } => Some((
            Case::CurrentSeaWellCertification,
            Evidence::CurrentSeaSampleAssay,
        )),
        InteractionId::ExtractionFacility {
            site: ExtractionSiteId::CurrentSeaDepthRig,
            facility: ExtractionFacilityId::TransferManifold,
        } => Some((
            Case::CurrentSeaWellCertification,
            Evidence::CurrentSeaCustodyChain,
        )),
        _ => None,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameplayRuntimeError {
    EventIdConflict(GameplayEventId),
    CausalRegression {
        previous: CausalPosition,
        actual: CausalPosition,
    },
    RuleSetMismatch {
        expected: RuleSetId,
        actual: RuleSetId,
    },
    ChildCausalPositionMismatch {
        gameplay: CausalPosition,
        regional: CausalPosition,
    },
    SequenceOverflow,
    RevisionOverflow,
    ReplaySequenceMismatch {
        expected: u64,
        actual: u64,
    },
    ReplayRevisionMismatch {
        expected: GameRevision,
        actual: GameRevision,
    },
    IdentityPayloadMismatch,
    UnexpectedRegionalEvent,
    RegionalReplayDivergence(crate::constitutional::RegionalEventId),
    MovementRequiresHueman,
    MovementReplayDivergence,
    InteractionRequiresHueman,
    NoInteractionTarget,
    InteractionReplayDivergence,
    MapEntryRequiresHueman,
    PhysicalExitRequired {
        map: String,
        exit_x: u16,
        exit_y: u16,
    },
    MapReplayDivergence,
    DisconnectedRouteTransfer {
        from: ConstitutionalRouteId,
        to: ConstitutionalRouteId,
    },
    DisconnectedMapTransfer {
        from: String,
        to: String,
    },
    BoardwalkCaseRequired,
    StonebendCaseRequired,
    CaseRequired,
    BoardwalkReplayDivergence,
    StonebendReplayDivergence,
    LivingWorldReplayDivergence,
    DeepPressureReplayDivergence,
    PartyReplayDivergence,
    DeepPressureSettlementLocationRequired,
    DeepPressureOutcomeRequiredForRecruitment,
    RecruitmentConversationRequired(RecruitmentCandidateId),
    LivingCaseLocationRequired {
        case_id: LivingCaseId,
        map: String,
    },
    Identity(GameplayIdentityError),
    Regional(RegionalSynthesisError),
    Boardwalk(BoardwalkCaseError),
    Stonebend(StonebendCaseError),
    RouteNetwork(RouteNetworkError),
    AuraField(AuraFieldError),
    AuraBeach(AuraBeachError),
    AuraBasin(AuraBasinError),
    LivingWorld(LivingWorldError),
    DeepPressure(DeepPressureError),
    Party(PartyError),
    WorldMap(WorldMapError),
}

impl From<GameplayIdentityError> for GameplayRuntimeError {
    fn from(value: GameplayIdentityError) -> Self {
        Self::Identity(value)
    }
}

impl From<RegionalSynthesisError> for GameplayRuntimeError {
    fn from(value: RegionalSynthesisError) -> Self {
        Self::Regional(value)
    }
}

impl From<BoardwalkCaseError> for GameplayRuntimeError {
    fn from(value: BoardwalkCaseError) -> Self {
        Self::Boardwalk(value)
    }
}

impl From<StonebendCaseError> for GameplayRuntimeError {
    fn from(value: StonebendCaseError) -> Self {
        Self::Stonebend(value)
    }
}

impl From<RouteNetworkError> for GameplayRuntimeError {
    fn from(value: RouteNetworkError) -> Self {
        Self::RouteNetwork(value)
    }
}

impl From<AuraFieldError> for GameplayRuntimeError {
    fn from(value: AuraFieldError) -> Self {
        Self::AuraField(value)
    }
}

impl From<AuraBeachError> for GameplayRuntimeError {
    fn from(value: AuraBeachError) -> Self {
        Self::AuraBeach(value)
    }
}

impl From<AuraBasinError> for GameplayRuntimeError {
    fn from(value: AuraBasinError) -> Self {
        Self::AuraBasin(value)
    }
}

impl From<WorldMapError> for GameplayRuntimeError {
    fn from(value: WorldMapError) -> Self {
        Self::WorldMap(value)
    }
}

impl From<LivingWorldError> for GameplayRuntimeError {
    fn from(value: LivingWorldError) -> Self {
        Self::LivingWorld(value)
    }
}

impl From<DeepPressureError> for GameplayRuntimeError {
    fn from(value: DeepPressureError) -> Self {
        Self::DeepPressure(value)
    }
}

impl From<PartyError> for GameplayRuntimeError {
    fn from(value: PartyError) -> Self {
        Self::Party(value)
    }
}

impl fmt::Display for GameplayRuntimeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "gameplay runtime rejected command: {self:?}")
    }
}

impl std::error::Error for GameplayRuntimeError {}
