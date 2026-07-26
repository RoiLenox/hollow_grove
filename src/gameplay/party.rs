//! Event-sourced six-person party and bounded recruitment vertical slice.
//!
//! Hueman and up to five companions form the active party. Recruitment is a
//! request to a capable person, never a player-owned capture result. Stable
//! candidate and continuity identities are role-based so display names may
//! change later without rewriting saves.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::world::extraction::ExtractionSiteId;

use super::{
    CharacterCondition, DeepPressureOutcomeRecord, DeepPressurePersonId,
    DeepPressureSettlementChoice, DeepPressureState, RelationshipMemory, WorldMapId,
};

pub const MAX_PARTY_MEMBERS: usize = 6;
pub const MAX_PARTY_COMPANIONS: usize = MAX_PARTY_MEMBERS - 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RecruitmentCandidateId {
    RiptidePressureKeeper,
    FieldEngagementSteward,
    BasinCareRunner,
    HighMineSupportReader,
    DeepworksAirKeeper,
    BreakwaterCurrentReader,
}

impl RecruitmentCandidateId {
    pub const ALL: [Self; 6] = [
        Self::RiptidePressureKeeper,
        Self::FieldEngagementSteward,
        Self::BasinCareRunner,
        Self::HighMineSupportReader,
        Self::DeepworksAirKeeper,
        Self::BreakwaterCurrentReader,
    ];

    #[must_use]
    pub const fn stable_id(self) -> &'static str {
        match self {
            Self::RiptidePressureKeeper => "recruitment-candidate.riptide-pressure-keeper",
            Self::FieldEngagementSteward => "recruitment-candidate.field-engagement-steward",
            Self::BasinCareRunner => "recruitment-candidate.basin-care-runner",
            Self::HighMineSupportReader => "recruitment-candidate.high-mine-support-reader",
            Self::DeepworksAirKeeper => "recruitment-candidate.deepworks-air-keeper",
            Self::BreakwaterCurrentReader => "recruitment-candidate.breakwater-current-reader",
        }
    }

    #[must_use]
    pub const fn continuity_id(self) -> &'static str {
        match self {
            Self::RiptidePressureKeeper => "being-continuity.party.riptide-pressure-keeper",
            Self::FieldEngagementSteward => "being-continuity.party.field-engagement-steward",
            Self::BasinCareRunner => "being-continuity.party.basin-care-runner",
            Self::HighMineSupportReader => "being-continuity.party.high-mine-support-reader",
            Self::DeepworksAirKeeper => "being-continuity.party.deepworks-air-keeper",
            Self::BreakwaterCurrentReader => "being-continuity.party.breakwater-current-reader",
        }
    }

    #[must_use]
    pub const fn person(self) -> DeepPressurePersonId {
        match self {
            Self::RiptidePressureKeeper => DeepPressurePersonId::CorinWake,
            Self::FieldEngagementSteward => DeepPressurePersonId::BrindleReed,
            Self::BasinCareRunner => DeepPressurePersonId::HarrowVale,
            Self::HighMineSupportReader => DeepPressurePersonId::OrenPike,
            Self::DeepworksAirKeeper => DeepPressurePersonId::MaelaDownroad,
            Self::BreakwaterCurrentReader => DeepPressurePersonId::TessBreakwater,
        }
    }

    #[must_use]
    pub fn from_stable_id(value: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|candidate| candidate.stable_id() == value)
    }

    #[must_use]
    pub fn for_person(person: DeepPressurePersonId) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|candidate| candidate.person() == person)
    }

    #[must_use]
    pub const fn action(self) -> PartyActionId {
        match self {
            Self::RiptidePressureKeeper => PartyActionId::ReadPressure,
            Self::FieldEngagementSteward => PartyActionId::ReadEngagementWork,
            Self::BasinCareRunner => PartyActionId::SurveyCareRoute,
            Self::HighMineSupportReader => PartyActionId::InspectSupport,
            Self::DeepworksAirKeeper => PartyActionId::TestAir,
            Self::BreakwaterCurrentReader => PartyActionId::ReadCurrent,
        }
    }

    #[must_use]
    pub const fn role(self) -> &'static str {
        match self {
            Self::RiptidePressureKeeper => "Riptide pressure keeper",
            Self::FieldEngagementSteward => "Aura Field engagement steward",
            Self::BasinCareRunner => "Basin care runner",
            Self::HighMineSupportReader => "high-mine support reader",
            Self::DeepworksAirKeeper => "Deepworks air keeper",
            Self::BreakwaterCurrentReader => "breakwater Current reader",
        }
    }

    #[must_use]
    pub const fn accepted_paths(self) -> &'static [RecruitmentPath] {
        match self {
            Self::RiptidePressureKeeper => {
                &[RecruitmentPath::SharedWork, RecruitmentPath::RecoveryFirst]
            }
            Self::FieldEngagementSteward => &[
                RecruitmentPath::SharedWork,
                RecruitmentPath::IndependentCompany,
            ],
            Self::BasinCareRunner => &[RecruitmentPath::RecoveryFirst, RecruitmentPath::SharedWork],
            Self::HighMineSupportReader => &[
                RecruitmentPath::SharedWork,
                RecruitmentPath::IndependentCompany,
            ],
            Self::DeepworksAirKeeper => &[
                RecruitmentPath::RecoveryFirst,
                RecruitmentPath::IndependentCompany,
            ],
            Self::BreakwaterCurrentReader => &[
                RecruitmentPath::IndependentCompany,
                RecruitmentPath::RecoveryFirst,
            ],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecruitmentPath {
    SharedWork,
    RecoveryFirst,
    IndependentCompany,
}

impl RecruitmentPath {
    #[must_use]
    pub const fn stable_id(self) -> &'static str {
        match self {
            Self::SharedWork => "shared-work",
            Self::RecoveryFirst => "recovery-first",
            Self::IndependentCompany => "independent-company",
        }
    }

    #[must_use]
    pub fn from_wire(value: &str) -> Option<Self> {
        [
            Self::SharedWork,
            Self::RecoveryFirst,
            Self::IndependentCompany,
        ]
        .into_iter()
        .find(|path| path.stable_id() == value)
    }

    #[must_use]
    pub const fn trust_threshold(self) -> i8 {
        match self {
            Self::SharedWork => 4,
            Self::RecoveryFirst => 3,
            Self::IndependentCompany => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecruitmentDecision {
    Accepted,
    Declined,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecruitmentDecisionReason {
    BoundedCompanyAccepted,
    ProtectedRefusalRequiresIndependentCompany,
    ConditionRequiresCare,
    ExhaustionRequiresRecoveryFirst,
    TrustNotEstablished,
    RoleBoundary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PartyMemberAvailability {
    Ready,
    Resting,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecruitmentDecisionRecord {
    pub candidate_id: RecruitmentCandidateId,
    pub person_id: DeepPressurePersonId,
    pub recruitment_path: RecruitmentPath,
    pub decision: RecruitmentDecision,
    pub reason: RecruitmentDecisionReason,
    pub relationship_affinity: i8,
    pub relationship_reliability: i8,
    pub condition_at_request: CharacterCondition,
    pub campaign_outcome_id: String,
    pub capable_subject_decided: bool,
    pub player_request_was_nonbinding: bool,
    pub refusal_creates_debt: bool,
    pub decision_is_persistent: bool,
    pub may_be_asked_again: bool,
    pub statement: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PartyMemberRecord {
    pub candidate_id: RecruitmentCandidateId,
    pub person_id: DeepPressurePersonId,
    pub continuity_id: String,
    pub joined_via: RecruitmentPath,
    pub availability: PartyMemberAvailability,
    pub field_action: PartyActionId,
    pub agency_boundaries: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PartyActorId {
    Hueman,
    Companion(RecruitmentCandidateId),
}

impl PartyActorId {
    #[must_use]
    pub const fn continuity_id(self) -> &'static str {
        match self {
            Self::Hueman => "being-continuity.hueman",
            Self::Companion(candidate) => candidate.continuity_id(),
        }
    }

    #[must_use]
    pub fn from_continuity_id(value: &str) -> Option<Self> {
        if value == Self::Hueman.continuity_id() {
            return Some(Self::Hueman);
        }
        RecruitmentCandidateId::ALL
            .into_iter()
            .find(|candidate| candidate.continuity_id() == value)
            .map(Self::Companion)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PartyActionId {
    ReadPressure,
    ReadEngagementWork,
    SurveyCareRoute,
    InspectSupport,
    TestAir,
    ReadCurrent,
}

impl PartyActionId {
    pub const ALL: [Self; 6] = [
        Self::ReadPressure,
        Self::ReadEngagementWork,
        Self::SurveyCareRoute,
        Self::InspectSupport,
        Self::TestAir,
        Self::ReadCurrent,
    ];

    #[must_use]
    pub const fn stable_id(self) -> &'static str {
        match self {
            Self::ReadPressure => "action.party.read-pressure",
            Self::ReadEngagementWork => "action.party.read-engagement-work",
            Self::SurveyCareRoute => "action.party.survey-care-route",
            Self::InspectSupport => "action.party.inspect-support",
            Self::TestAir => "action.party.test-air",
            Self::ReadCurrent => "action.party.read-current",
        }
    }

    #[must_use]
    pub fn from_stable_id(value: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|action| action.stable_id() == value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PartyRecoveryChange {
    pub candidate_id: RecruitmentCandidateId,
    pub from: PartyMemberAvailability,
    pub to: PartyMemberAvailability,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FieldActionRecord {
    pub sequence: u64,
    pub actor: PartyActorId,
    pub action: PartyActionId,
    pub map: WorldMapId,
    pub target_continuity_id: Option<String>,
    pub finding: String,
    pub evidence_limit: String,
    pub creates_constitutional_decision: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", deny_unknown_fields)]
pub enum PartyEvent {
    RecruitmentDecided {
        record: RecruitmentDecisionRecord,
        member: Option<PartyMemberRecord>,
    },
    MemberSelected {
        from: PartyActorId,
        to: PartyActorId,
    },
    LeadChanged {
        from: PartyActorId,
        to: PartyActorId,
    },
    FieldActionResolved {
        record: FieldActionRecord,
    },
    ShiftRecoveryApplied {
        changes: Vec<PartyRecoveryChange>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PartyState {
    pub revision: u64,
    pub hueman_continuity_id: Option<String>,
    pub companions: Vec<PartyMemberRecord>,
    pub recruitment_decisions: BTreeMap<RecruitmentCandidateId, RecruitmentDecisionRecord>,
    pub selected: PartyActorId,
    pub lead: PartyActorId,
    pub field_actions: Vec<FieldActionRecord>,
}

impl Default for PartyState {
    fn default() -> Self {
        Self::new()
    }
}

impl PartyState {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            revision: 0,
            hueman_continuity_id: None,
            companions: Vec::new(),
            recruitment_decisions: BTreeMap::new(),
            selected: PartyActorId::Hueman,
            lead: PartyActorId::Hueman,
            field_actions: Vec::new(),
        }
    }

    pub fn establish_hueman(&mut self, continuity_id: &str) -> Result<(), PartyError> {
        if self.hueman_continuity_id.is_some() {
            return Err(PartyError::HuemanAlreadyEstablished);
        }
        self.hueman_continuity_id = Some(continuity_id.into());
        self.validate()
    }

    #[must_use]
    pub fn member_count(&self) -> usize {
        usize::from(self.hueman_continuity_id.is_some()) + self.companions.len()
    }

    #[must_use]
    pub fn is_recruited(&self, candidate: RecruitmentCandidateId) -> bool {
        self.companions
            .iter()
            .any(|member| member.candidate_id == candidate)
    }

    #[must_use]
    pub fn recruited_person(&self, person: DeepPressurePersonId) -> bool {
        RecruitmentCandidateId::for_person(person)
            .is_some_and(|candidate| self.is_recruited(candidate))
    }

    #[must_use]
    pub fn member(&self, actor: PartyActorId) -> Option<&PartyMemberRecord> {
        let PartyActorId::Companion(candidate) = actor else {
            return None;
        };
        self.companions
            .iter()
            .find(|member| member.candidate_id == candidate)
    }

    #[must_use]
    pub fn contains_actor(&self, actor: PartyActorId) -> bool {
        match actor {
            PartyActorId::Hueman => self.hueman_continuity_id.is_some(),
            PartyActorId::Companion(candidate) => self.is_recruited(candidate),
        }
    }

    #[must_use]
    pub fn actor_from_continuity_id(&self, value: &str) -> Option<PartyActorId> {
        if self.hueman_continuity_id.as_deref() == Some(value) {
            return Some(PartyActorId::Hueman);
        }
        RecruitmentCandidateId::ALL
            .into_iter()
            .find(|candidate| candidate.continuity_id() == value && self.is_recruited(*candidate))
            .map(PartyActorId::Companion)
    }

    #[must_use]
    pub fn actor_continuity_id(&self, actor: PartyActorId) -> Option<&str> {
        match actor {
            PartyActorId::Hueman => self.hueman_continuity_id.as_deref(),
            PartyActorId::Companion(candidate) => self
                .member(actor)
                .filter(|member| member.candidate_id == candidate)
                .map(|member| member.continuity_id.as_str()),
        }
    }

    pub fn request_recruitment(
        &mut self,
        candidate: RecruitmentCandidateId,
        path: RecruitmentPath,
        relationship: &RelationshipMemory,
        outcome: &DeepPressureOutcomeRecord,
    ) -> Result<PartyEvent, PartyError> {
        if self.hueman_continuity_id.is_none() {
            return Err(PartyError::PartyRequiresHueman);
        }
        if self.recruitment_decisions.contains_key(&candidate) {
            return Err(PartyError::RecruitmentDecisionAlreadyRecorded(candidate));
        }
        if self.companions.len() >= MAX_PARTY_COMPANIONS {
            return Err(PartyError::PartyAtCapacity);
        }
        if relationship.person != candidate.person() {
            return Err(PartyError::RelationshipPersonMismatch);
        }

        let (decision, reason) = decide_recruitment(candidate, path, relationship, outcome.choice);
        let record = RecruitmentDecisionRecord {
            candidate_id: candidate,
            person_id: candidate.person(),
            recruitment_path: path,
            decision,
            reason,
            relationship_affinity: relationship.affinity,
            relationship_reliability: relationship.reliability,
            condition_at_request: relationship.condition,
            campaign_outcome_id: outcome.id.stable_id().into(),
            capable_subject_decided: true,
            player_request_was_nonbinding: true,
            refusal_creates_debt: false,
            decision_is_persistent: true,
            may_be_asked_again: false,
            statement: recruitment_statement(candidate, decision, reason),
        };
        let member = (decision == RecruitmentDecision::Accepted).then(|| PartyMemberRecord {
            candidate_id: candidate,
            person_id: candidate.person(),
            continuity_id: candidate.continuity_id().into(),
            joined_via: path,
            availability: if relationship.condition == CharacterCondition::Exhausted {
                PartyMemberAvailability::Resting
            } else {
                PartyMemberAvailability::Ready
            },
            field_action: candidate.action(),
            agency_boundaries: vec![
                "company is finite, voluntary, and may be left without debt".into(),
                "party membership grants no ownership, office, Title, or automatic consent".into(),
                "the member retains Current, refusal, voice, and capable-subject authority".into(),
            ],
        });
        let event = PartyEvent::RecruitmentDecided { record, member };
        let mut candidate_state = self.clone();
        candidate_state.apply(&event)?;
        *self = candidate_state;
        Ok(event)
    }

    pub fn select(&mut self, actor: PartyActorId) -> Result<PartyEvent, PartyError> {
        self.require_actor(actor)?;
        if self.selected == actor {
            return Err(PartyError::ActorAlreadySelected);
        }
        let event = PartyEvent::MemberSelected {
            from: self.selected,
            to: actor,
        };
        let mut candidate = self.clone();
        candidate.apply(&event)?;
        *self = candidate;
        Ok(event)
    }

    pub fn switch_lead(&mut self, actor: PartyActorId) -> Result<PartyEvent, PartyError> {
        self.require_actor(actor)?;
        self.require_ready(actor)?;
        if self.lead == actor {
            return Err(PartyError::ActorAlreadyLeads);
        }
        let event = PartyEvent::LeadChanged {
            from: self.lead,
            to: actor,
        };
        let mut candidate = self.clone();
        candidate.apply(&event)?;
        *self = candidate;
        Ok(event)
    }

    pub fn use_action(
        &mut self,
        actor: PartyActorId,
        action: PartyActionId,
        map: WorldMapId,
        target_continuity_id: Option<String>,
        campaign: &DeepPressureState,
    ) -> Result<PartyEvent, PartyError> {
        self.require_actor(actor)?;
        self.require_ready(actor)?;
        if self.lead != actor {
            return Err(PartyError::FieldActionRequiresLead);
        }
        let PartyActorId::Companion(candidate) = actor else {
            return Err(PartyError::HuemanPartyActionNotImplemented);
        };
        if candidate.action() != action {
            return Err(PartyError::ActionDoesNotBelongToActor);
        }
        if !action_permitted_at(candidate, map) {
            return Err(PartyError::ActionLocationRequired {
                action,
                map: map.as_str().into(),
            });
        }
        if target_continuity_id.as_deref() == Some("") {
            return Err(PartyError::InvalidTarget);
        }
        let sequence = u64::try_from(self.field_actions.len())
            .map_err(|_| PartyError::FieldActionSequenceOverflow)?;
        let (finding, evidence_limit) = field_finding(candidate, campaign);
        let record = FieldActionRecord {
            sequence,
            actor,
            action,
            map,
            target_continuity_id,
            finding,
            evidence_limit,
            creates_constitutional_decision: false,
        };
        let event = PartyEvent::FieldActionResolved { record };
        let mut candidate_state = self.clone();
        candidate_state.apply(&event)?;
        *self = candidate_state;
        Ok(event)
    }

    pub fn advance_shift(&mut self) -> Result<Option<PartyEvent>, PartyError> {
        let changes = self
            .companions
            .iter()
            .filter(|member| member.availability == PartyMemberAvailability::Resting)
            .map(|member| PartyRecoveryChange {
                candidate_id: member.candidate_id,
                from: PartyMemberAvailability::Resting,
                to: PartyMemberAvailability::Ready,
            })
            .collect::<Vec<_>>();
        if changes.is_empty() {
            return Ok(None);
        }
        let event = PartyEvent::ShiftRecoveryApplied { changes };
        let mut candidate = self.clone();
        candidate.apply(&event)?;
        *self = candidate;
        Ok(Some(event))
    }

    pub fn apply(&mut self, event: &PartyEvent) -> Result<(), PartyError> {
        match event {
            PartyEvent::RecruitmentDecided { record, member } => {
                if self
                    .recruitment_decisions
                    .contains_key(&record.candidate_id)
                    || record.person_id != record.candidate_id.person()
                    || !record.capable_subject_decided
                    || !record.player_request_was_nonbinding
                    || record.refusal_creates_debt
                    || !record.decision_is_persistent
                    || record.may_be_asked_again
                    || (record.decision == RecruitmentDecision::Accepted) != member.is_some()
                {
                    return Err(PartyError::RecruitmentRecordDivergence);
                }
                if let Some(member) = member {
                    if self.companions.len() >= MAX_PARTY_COMPANIONS
                        || member.candidate_id != record.candidate_id
                        || member.person_id != record.person_id
                        || member.continuity_id != member.candidate_id.continuity_id()
                        || member.field_action != member.candidate_id.action()
                        || member.joined_via != record.recruitment_path
                    {
                        return Err(PartyError::RecruitmentRecordDivergence);
                    }
                    self.companions.push(member.clone());
                }
                self.recruitment_decisions
                    .insert(record.candidate_id, record.clone());
            }
            PartyEvent::MemberSelected { from, to } => {
                if self.selected != *from || !self.contains_actor(*to) || from == to {
                    return Err(PartyError::SelectionReplayDivergence);
                }
                self.selected = *to;
            }
            PartyEvent::LeadChanged { from, to } => {
                if self.lead != *from || !self.contains_actor(*to) || from == to {
                    return Err(PartyError::LeadReplayDivergence);
                }
                self.require_ready(*to)?;
                self.lead = *to;
            }
            PartyEvent::FieldActionResolved { record } => {
                if record.sequence != u64::try_from(self.field_actions.len()).unwrap_or(u64::MAX)
                    || record.actor != self.lead
                    || !self.contains_actor(record.actor)
                    || record.creates_constitutional_decision
                {
                    return Err(PartyError::FieldActionReplayDivergence);
                }
                let PartyActorId::Companion(candidate) = record.actor else {
                    return Err(PartyError::FieldActionReplayDivergence);
                };
                if candidate.action() != record.action
                    || !action_permitted_at(candidate, record.map)
                {
                    return Err(PartyError::FieldActionReplayDivergence);
                }
                self.require_ready(record.actor)?;
                self.field_actions.push(record.clone());
            }
            PartyEvent::ShiftRecoveryApplied { changes } => {
                if changes.is_empty() {
                    return Err(PartyError::RecoveryReplayDivergence);
                }
                let mut seen = BTreeSet::new();
                for change in changes {
                    if !seen.insert(change.candidate_id)
                        || change.from != PartyMemberAvailability::Resting
                        || change.to != PartyMemberAvailability::Ready
                    {
                        return Err(PartyError::RecoveryReplayDivergence);
                    }
                    let member = self
                        .companions
                        .iter_mut()
                        .find(|member| member.candidate_id == change.candidate_id)
                        .ok_or(PartyError::RecoveryReplayDivergence)?;
                    if member.availability != change.from {
                        return Err(PartyError::RecoveryReplayDivergence);
                    }
                    member.availability = change.to;
                }
            }
        }
        self.revision = self
            .revision
            .checked_add(1)
            .ok_or(PartyError::RevisionOverflow)?;
        self.validate()
    }

    pub fn validate(&self) -> Result<(), PartyError> {
        if self.companions.len() > MAX_PARTY_COMPANIONS || self.member_count() > MAX_PARTY_MEMBERS {
            return Err(PartyError::PartyAtCapacity);
        }
        let companion_ids = self
            .companions
            .iter()
            .map(|member| member.candidate_id)
            .collect::<BTreeSet<_>>();
        if companion_ids.len() != self.companions.len()
            || self.companions.iter().any(|member| {
                member.person_id != member.candidate_id.person()
                    || member.continuity_id != member.candidate_id.continuity_id()
                    || member.field_action != member.candidate_id.action()
                    || self
                        .recruitment_decisions
                        .get(&member.candidate_id)
                        .is_none_or(|record| record.decision != RecruitmentDecision::Accepted)
            })
        {
            return Err(PartyError::DuplicateOrInvalidMember);
        }
        if self.hueman_continuity_id.is_none()
            && (!self.companions.is_empty()
                || !self.recruitment_decisions.is_empty()
                || self.lead != PartyActorId::Hueman
                || self.selected != PartyActorId::Hueman)
        {
            return Err(PartyError::PartyRequiresHueman);
        }
        if self.hueman_continuity_id.is_some()
            && (!self.contains_actor(self.lead) || !self.contains_actor(self.selected))
        {
            return Err(PartyError::InvalidPartyActor);
        }
        Ok(())
    }

    fn require_actor(&self, actor: PartyActorId) -> Result<(), PartyError> {
        self.contains_actor(actor)
            .then_some(())
            .ok_or(PartyError::ActorNotInParty)
    }

    fn require_ready(&self, actor: PartyActorId) -> Result<(), PartyError> {
        if actor == PartyActorId::Hueman {
            return self.require_actor(actor);
        }
        self.member(actor)
            .filter(|member| member.availability == PartyMemberAvailability::Ready)
            .map(|_| ())
            .ok_or(PartyError::ActorUnavailable)
    }
}

fn decide_recruitment(
    candidate: RecruitmentCandidateId,
    path: RecruitmentPath,
    relationship: &RelationshipMemory,
    settlement: DeepPressureSettlementChoice,
) -> (RecruitmentDecision, RecruitmentDecisionReason) {
    if matches!(
        relationship.condition,
        CharacterCondition::Injured | CharacterCondition::Exposed
    ) {
        return (
            RecruitmentDecision::Declined,
            RecruitmentDecisionReason::ConditionRequiresCare,
        );
    }
    if settlement == DeepPressureSettlementChoice::ProtectedRefusal
        && path != RecruitmentPath::IndependentCompany
    {
        return (
            RecruitmentDecision::Declined,
            RecruitmentDecisionReason::ProtectedRefusalRequiresIndependentCompany,
        );
    }
    if relationship.condition == CharacterCondition::Exhausted
        && path != RecruitmentPath::RecoveryFirst
    {
        return (
            RecruitmentDecision::Declined,
            RecruitmentDecisionReason::ExhaustionRequiresRecoveryFirst,
        );
    }
    if !candidate.accepted_paths().contains(&path) {
        return (
            RecruitmentDecision::Declined,
            RecruitmentDecisionReason::RoleBoundary,
        );
    }
    if relationship.affinity + relationship.reliability < path.trust_threshold() {
        return (
            RecruitmentDecision::Declined,
            RecruitmentDecisionReason::TrustNotEstablished,
        );
    }
    (
        RecruitmentDecision::Accepted,
        RecruitmentDecisionReason::BoundedCompanyAccepted,
    )
}

fn recruitment_statement(
    candidate: RecruitmentCandidateId,
    decision: RecruitmentDecision,
    reason: RecruitmentDecisionReason,
) -> String {
    let name = candidate.person().display_name();
    match (decision, reason) {
        (RecruitmentDecision::Accepted, _) => format!(
            "{name} chooses bounded company for the disclosed work and keeps the right to stop, refuse, and leave without debt."
        ),
        (_, RecruitmentDecisionReason::ProtectedRefusalRequiresIndependentCompany) => format!(
            "{name} declines a shared compact after protected refusal; only independent company would honor the recorded boundary."
        ),
        (_, RecruitmentDecisionReason::ConditionRequiresCare) => {
            format!("{name} declines while care or exposure safety remains unresolved.")
        }
        (_, RecruitmentDecisionReason::ExhaustionRequiresRecoveryFirst) => {
            format!("{name} declines work that does not put recovery first.")
        }
        (_, RecruitmentDecisionReason::TrustNotEstablished) => format!(
            "{name} declines because reliable shared practice has not yet been established."
        ),
        (_, RecruitmentDecisionReason::RoleBoundary) => {
            format!("{name} declines a role outside the work they consented to carry.")
        }
        (_, RecruitmentDecisionReason::BoundedCompanyAccepted) => {
            unreachable!("accepted reason cannot accompany a declined decision")
        }
    }
}

fn action_permitted_at(candidate: RecruitmentCandidateId, map: WorldMapId) -> bool {
    match candidate {
        RecruitmentCandidateId::RiptidePressureKeeper => matches!(
            map,
            WorldMapId::ExtractionSite(ExtractionSiteId::RiptideRecoveryRig)
                | WorldMapId::ExtractionSite(ExtractionSiteId::CurrentSeaDepthRig)
        ),
        RecruitmentCandidateId::FieldEngagementSteward => map == WorldMapId::AuraFieldWorkingLand,
        RecruitmentCandidateId::BasinCareRunner => matches!(
            map,
            WorldMapId::AuraBeachCoastalCommons | WorldMapId::AuraBasinCollisionGrounds
        ),
        RecruitmentCandidateId::HighMineSupportReader => matches!(
            map,
            WorldMapId::ExtractionSite(ExtractionSiteId::MntAuraHighMine)
                | WorldMapId::ExtractionSite(ExtractionSiteId::StairwayBurdenMine)
        ),
        RecruitmentCandidateId::DeepworksAirKeeper => matches!(
            map,
            WorldMapId::ExtractionSite(ExtractionSiteId::HighwayToHellDeepworks)
                | WorldMapId::ExtractionSite(ExtractionSiteId::StairwayBurdenMine)
        ),
        RecruitmentCandidateId::BreakwaterCurrentReader => matches!(
            map,
            WorldMapId::AuraBeachCoastalCommons
                | WorldMapId::ExtractionSite(ExtractionSiteId::RiptideRecoveryRig)
                | WorldMapId::ExtractionSite(ExtractionSiteId::CurrentSeaDepthRig)
        ),
    }
}

fn field_finding(
    candidate: RecruitmentCandidateId,
    campaign: &DeepPressureState,
) -> (String, String) {
    let aftermath = campaign.outcome.as_ref().map(|outcome| &outcome.aftermath);
    let finding = match candidate {
        RecruitmentCandidateId::RiptidePressureKeeper => aftermath.map_or_else(
            || "Corin separates immediate pressure control from unresolved shut-in duty.".into(),
            |record| {
                format!(
                    "Corin reads the pressure against the public production posture: {}.",
                    record.production_posture
                )
            },
        ),
        RecruitmentCandidateId::FieldEngagementSteward => aftermath.map_or_else(
            || {
                "Brindle identifies which Engagement Farm task has a disclosed term, exit, and measurable Field need."
                    .into()
            },
            |record| {
                format!(
                    "Brindle compares Engagement Farm work to Field security {} and names the next finite shared task.",
                    record.field_security
                )
            },
        ),
        RecruitmentCandidateId::BasinCareRunner => aftermath.map_or_else(
            || "Harrow marks a care route without turning the injured into cargo.".into(),
            |record| {
                format!(
                    "Harrow surveys a care route against Basin repair {} and preserves refusal at every transfer.",
                    record.basin_repair
                )
            },
        ),
        RecruitmentCandidateId::HighMineSupportReader => {
            "Oren distinguishes measured support, unstable roof, and a lawful withdrawal edge."
                .into()
        }
        RecruitmentCandidateId::DeepworksAirKeeper => {
            "Maela tests the air and names the stop line before any production claim.".into()
        }
        RecruitmentCandidateId::BreakwaterCurrentReader => aftermath.map_or_else(
            || "Tess separates observed Current movement from dock rumor.".into(),
            |record| {
                format!(
                    "Tess reads Current movement against coast recovery {} and labels rumor separately.",
                    record.coast_recovery
                )
            },
        ),
    };
    (
        finding,
        "the field action discloses bounded evidence; it does not decide a duty case, form a Bond, grant Clearance, or compel another Being".into(),
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PartyError {
    HuemanAlreadyEstablished,
    PartyRequiresHueman,
    PartyAtCapacity,
    RecruitmentDecisionAlreadyRecorded(RecruitmentCandidateId),
    RelationshipPersonMismatch,
    RecruitmentRecordDivergence,
    DuplicateOrInvalidMember,
    InvalidPartyActor,
    ActorNotInParty,
    ActorAlreadySelected,
    ActorAlreadyLeads,
    ActorUnavailable,
    FieldActionRequiresLead,
    HuemanPartyActionNotImplemented,
    ActionDoesNotBelongToActor,
    ActionLocationRequired { action: PartyActionId, map: String },
    InvalidTarget,
    FieldActionSequenceOverflow,
    SelectionReplayDivergence,
    LeadReplayDivergence,
    FieldActionReplayDivergence,
    RecoveryReplayDivergence,
    RevisionOverflow,
}

impl fmt::Display for PartyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "party runtime rejected: {self:?}")
    }
}

impl std::error::Error for PartyError {}
