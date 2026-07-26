//! Compromise-level adapters for cross-House constitutional history.
//!
//! Domain models keep their own IDs and decision rules. This module converts
//! those IDs and outcomes into the one shared constitutional-interface event
//! envelope, then adds Court, amendment, Restitution, transfer, gate, and
//! Central Junction policy over the common archive.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::constitutional::{
    AuthorityActorId, InterfaceAggregateId, InterfaceApplyOutcome, InterfaceDomain,
    InterfaceEventEnvelope, InterfaceEventId, InterfaceEventKind, InterfaceFailureState,
    InterfaceIdentityKind, InterfaceIdentityRef, ParticipantId, RegionalBeingId,
    SHARED_INTERFACE_ARCHIVE_VERSION, SharedInterfaceError, SharedInterfaceRuntime,
    decode_shared_interface_archive, encode_shared_interface_archive,
    migrate_shared_interface_archive,
};
use crate::hollow_grove_contract::House;
use crate::institution::{IdentityId, InstitutionId, InstitutionalBeingId, OfficeId};
use crate::world::minoan_court::{AmendmentScope, JudicialStage, ResponsibleInstitution};
use crate::world::stonebend::second_pass::StonebendGateFacing;
use crate::world::stonebend::{EvidenceRecordId, TitleRecordId};

pub const SHARED_INTERFACE_AUTHORITY_SOURCE: &str = "HOLLOW_GROVE_COMPROMISE_V1_DRAFT.md";

fn identity_ref(
    kind: InterfaceIdentityKind,
    stable_id: &str,
    authority_source: &str,
) -> Result<InterfaceIdentityRef, SharedInterfaceError> {
    InterfaceIdentityRef::new(kind, stable_id, authority_source)
}

pub fn person_identity_ref(
    identity: &IdentityId,
    authority_source: &str,
) -> Result<InterfaceIdentityRef, SharedInterfaceError> {
    identity_ref(
        InterfaceIdentityKind::Person,
        identity.as_str(),
        authority_source,
    )
}

pub fn institutional_being_identity_ref(
    identity: &InstitutionalBeingId,
    authority_source: &str,
) -> Result<InterfaceIdentityRef, SharedInterfaceError> {
    identity_ref(
        InterfaceIdentityKind::Person,
        identity.as_str(),
        authority_source,
    )
}

pub fn participant_identity_ref(
    identity: &ParticipantId,
    authority_source: &str,
) -> Result<InterfaceIdentityRef, SharedInterfaceError> {
    identity_ref(
        InterfaceIdentityKind::Participant,
        identity.as_str(),
        authority_source,
    )
}

pub fn regional_being_identity_ref(
    identity: &RegionalBeingId,
    authority_source: &str,
) -> Result<InterfaceIdentityRef, SharedInterfaceError> {
    identity_ref(
        InterfaceIdentityKind::RegionalBeing,
        identity.as_str(),
        authority_source,
    )
}

pub fn institution_identity_ref(
    identity: &InstitutionId,
    authority_source: &str,
) -> Result<InterfaceIdentityRef, SharedInterfaceError> {
    identity_ref(
        InterfaceIdentityKind::Institution,
        identity.as_str(),
        authority_source,
    )
}

pub fn office_identity_ref(
    identity: &OfficeId,
    authority_source: &str,
) -> Result<InterfaceIdentityRef, SharedInterfaceError> {
    identity_ref(
        InterfaceIdentityKind::Authority,
        identity.as_str(),
        authority_source,
    )
}

pub fn authority_actor_identity_ref(
    identity: &AuthorityActorId,
    authority_source: &str,
) -> Result<InterfaceIdentityRef, SharedInterfaceError> {
    identity_ref(
        InterfaceIdentityKind::Authority,
        identity.as_str(),
        authority_source,
    )
}

pub fn house_identity_ref(house: House) -> Result<InterfaceIdentityRef, SharedInterfaceError> {
    identity_ref(
        InterfaceIdentityKind::House,
        &format!("house.{}", house.as_str().to_ascii_lowercase()),
        SHARED_INTERFACE_AUTHORITY_SOURCE,
    )
}

pub fn title_identity_ref(
    identity: &TitleRecordId,
) -> Result<InterfaceIdentityRef, SharedInterfaceError> {
    identity_ref(
        InterfaceIdentityKind::Title,
        identity.as_str(),
        "STONEBEND_CONSTITUTION_V2.md",
    )
}

pub fn gate_identity_ref(
    facing: StonebendGateFacing,
) -> Result<InterfaceIdentityRef, SharedInterfaceError> {
    identity_ref(
        InterfaceIdentityKind::Gate,
        facing.stable_id(),
        "STONEBEND_THREE_GATES_OFFICES_AND_TITLE_SCOPE_V1.md",
    )
}

pub fn court_case_identity_ref(
    identity: &IdentityId,
) -> Result<InterfaceIdentityRef, SharedInterfaceError> {
    identity_ref(
        InterfaceIdentityKind::CourtCase,
        identity.as_str(),
        "MINOAN_COUNTY_COURT_SYSTEM_AND_RESTITUTION_CYCLE_V1.md",
    )
}

pub fn amendment_identity_ref(
    identity: &IdentityId,
) -> Result<InterfaceIdentityRef, SharedInterfaceError> {
    identity_ref(
        InterfaceIdentityKind::Amendment,
        identity.as_str(),
        "MINOAN_COUNTY_COURT_SYSTEM_AND_RESTITUTION_CYCLE_V1.md",
    )
}

pub fn remedy_identity_ref(
    identity: &IdentityId,
) -> Result<InterfaceIdentityRef, SharedInterfaceError> {
    identity_ref(
        InterfaceIdentityKind::Remedy,
        identity.as_str(),
        "MINOAN_COUNTY_COURT_SYSTEM_AND_RESTITUTION_CYCLE_V1.md",
    )
}

pub fn central_junction_record_identity_ref(
    stable_id: &str,
) -> Result<InterfaceIdentityRef, SharedInterfaceError> {
    identity_ref(
        InterfaceIdentityKind::CentralJunctionRecord,
        stable_id,
        "CENTRAL_JUNCTION_FOUR_POLE_ECONOMY_V1.md",
    )
}

pub fn evidence_identity_ref(
    identity: &EvidenceRecordId,
    authority_source: &str,
) -> Result<InterfaceIdentityRef, SharedInterfaceError> {
    identity_ref(
        InterfaceIdentityKind::Evidence,
        identity.as_str(),
        authority_source,
    )
}

fn aggregate(identity: &IdentityId) -> Result<InterfaceAggregateId, SharedInterfaceError> {
    InterfaceAggregateId::new(identity.as_str())
}

fn canonical_house_list(houses: &BTreeSet<House>) -> String {
    houses
        .iter()
        .map(|house| house.as_str())
        .collect::<Vec<_>>()
        .join(",")
}

fn parse_house_list(value: &str) -> Result<BTreeSet<House>, InterfacePolicyError> {
    value
        .split(',')
        .filter(|part| !part.is_empty())
        .map(|part| match part {
            "Stonebend" => Ok(House::Stonebend),
            "Sandmanor" => Ok(House::Sandmanor),
            "Glaushouse" => Ok(House::Glaushouse),
            "Flynt" => Ok(House::Flynt),
            _ => Err(InterfacePolicyError::InvalidPayload("house list".into())),
        })
        .collect()
}

fn stage_name(stage: JudicialStage) -> &'static str {
    match stage {
        JudicialStage::Conciliation => "conciliation",
        JudicialStage::FirstHearing => "first-hearing",
        JudicialStage::Appeal => "appeal",
        JudicialStage::ConstitutionalReview => "constitutional-review",
        JudicialStage::Restitution => "restitution",
    }
}

fn parse_stage(value: &str) -> Result<JudicialStage, InterfacePolicyError> {
    match value {
        "conciliation" => Ok(JudicialStage::Conciliation),
        "first-hearing" => Ok(JudicialStage::FirstHearing),
        "appeal" => Ok(JudicialStage::Appeal),
        "constitutional-review" => Ok(JudicialStage::ConstitutionalReview),
        "restitution" => Ok(JudicialStage::Restitution),
        _ => Err(InterfacePolicyError::InvalidPayload(
            "judicial stage".into(),
        )),
    }
}

fn payload_value<'a>(
    event: &'a InterfaceEventEnvelope,
    key: &str,
) -> Result<&'a str, InterfacePolicyError> {
    event
        .payload
        .get(key)
        .map(String::as_str)
        .ok_or_else(|| InterfacePolicyError::InvalidPayload(key.into()))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourtInterfaceRuntime {
    case: InterfaceAggregateId,
    runtime: SharedInterfaceRuntime,
}

impl CourtInterfaceRuntime {
    pub fn open(
        event_id: InterfaceEventId,
        case: &IdentityId,
        authority: InterfaceIdentityRef,
        parties: Vec<InterfaceIdentityRef>,
        jurisdictions: BTreeSet<String>,
    ) -> Result<Self, InterfacePolicyError> {
        let case = aggregate(case)?;
        let mut payload = BTreeMap::new();
        payload.insert(
            "jurisdictions".into(),
            jurisdictions.into_iter().collect::<Vec<_>>().join(","),
        );
        let event = InterfaceEventEnvelope {
            event_id,
            source_aggregate: case.clone(),
            source_domain: InterfaceDomain::MinoanCourt,
            receiving_aggregate: case.clone(),
            receiving_domain: InterfaceDomain::MinoanCourt,
            authority,
            subjects: parties,
            evidence: Vec::new(),
            kind: InterfaceEventKind::CourtCaseOpened,
            semantic_sequence: 0,
            causal_parent: None,
            related_case: Some(case.clone()),
            related_amendment: None,
            responsible_remedy_institution: None,
            failure_state: InterfaceFailureState::None,
            migration_version: SHARED_INTERFACE_ARCHIVE_VERSION,
            historical_links: Vec::new(),
            payload,
        };
        let mut state = Self {
            case,
            runtime: SharedInterfaceRuntime::default(),
        };
        state.apply(event)?;
        Ok(state)
    }

    #[must_use]
    pub fn case(&self) -> &InterfaceAggregateId {
        &self.case
    }

    #[must_use]
    pub fn runtime(&self) -> &SharedInterfaceRuntime {
        &self.runtime
    }

    #[must_use]
    pub fn is_closed(&self) -> bool {
        self.runtime
            .aggregate_history(&self.case)
            .iter()
            .any(|event| event.kind == InterfaceEventKind::CourtCaseClosed)
    }

    #[must_use]
    pub fn stage_history(&self) -> Vec<(u32, JudicialStage, &InterfaceEventEnvelope)> {
        let mut records = self
            .runtime
            .aggregate_history(&self.case)
            .into_iter()
            .filter(|event| event.kind == InterfaceEventKind::CourtStageTransition)
            .filter_map(|event| {
                let stage = parse_stage(event.payload.get("stage")?).ok()?;
                let cycle = event.payload.get("cycle")?.parse().ok()?;
                Some((cycle, stage, event))
            })
            .collect::<Vec<_>>();
        records.sort_by_key(|(cycle, stage, event)| {
            (*cycle, stage.semantic_order(), event.event_id.as_str())
        });
        records
    }

    pub fn apply(
        &mut self,
        event: InterfaceEventEnvelope,
    ) -> Result<InterfaceApplyOutcome, InterfacePolicyError> {
        if let Some(existing) = self.runtime.event(&event.event_id) {
            return if existing == &event {
                Ok(InterfaceApplyOutcome::Idempotent)
            } else {
                Err(SharedInterfaceError::ConflictingEventIdentity(event.event_id).into())
            };
        }
        if event.source_aggregate != self.case
            || event.related_case.as_ref() != Some(&self.case)
            || event.source_domain != InterfaceDomain::MinoanCourt
        {
            return Err(InterfacePolicyError::CaseIdentityMismatch);
        }
        if self.is_closed() {
            return Err(InterfacePolicyError::ClosedCase);
        }
        let history = self.runtime.aggregate_history(&self.case);
        match event.kind {
            InterfaceEventKind::CourtCaseOpened => {
                if !history.is_empty() || event.semantic_sequence != 0 {
                    return Err(InterfacePolicyError::DuplicateCaseOpening);
                }
            }
            InterfaceEventKind::CourtStageTransition => self.validate_stage(&event)?,
            InterfaceEventKind::CourtJudgment => {
                self.require_stage(JudicialStage::FirstHearing)?;
            }
            InterfaceEventKind::CourtAppealDisposition => {
                self.require_stage(JudicialStage::Appeal)?;
            }
            InterfaceEventKind::CourtConstitutionalDisposition => {
                self.require_stage(JudicialStage::ConstitutionalReview)?;
            }
            InterfaceEventKind::RestitutionOrdered
            | InterfaceEventKind::RestitutionDelivered
            | InterfaceEventKind::RestitutionFailed
            | InterfaceEventKind::CourtEquilibriumConfirmed => {
                self.require_stage(JudicialStage::Restitution)?;
            }
            InterfaceEventKind::RestitutionRecurrence => {
                if !history
                    .iter()
                    .any(|prior| prior.kind == InterfaceEventKind::RestitutionFailed)
                {
                    return Err(InterfacePolicyError::RecurrenceWithoutFailure);
                }
                let to_cycle: u32 = payload_value(&event, "to-cycle")?
                    .parse()
                    .map_err(|_| InterfacePolicyError::InvalidPayload("to-cycle".into()))?;
                let from_cycle: u32 = payload_value(&event, "from-cycle")?
                    .parse()
                    .map_err(|_| InterfacePolicyError::InvalidPayload("from-cycle".into()))?;
                if to_cycle != from_cycle + 1 {
                    return Err(InterfacePolicyError::InvalidRecurrence);
                }
                parse_stage(payload_value(&event, "return-stage")?)?;
            }
            InterfaceEventKind::CourtCaseClosed => {
                if !history
                    .iter()
                    .any(|prior| prior.kind == InterfaceEventKind::CourtEquilibriumConfirmed)
                {
                    return Err(InterfacePolicyError::ClosureBeforeEquilibrium);
                }
            }
            InterfaceEventKind::CourtEvidenceSubmitted
            | InterfaceEventKind::CourtProtectiveOrder => {}
            _ => return Err(InterfacePolicyError::WrongAggregateEvent(event.kind)),
        }
        self.runtime.apply(event).map_err(Into::into)
    }

    fn validate_stage(&self, event: &InterfaceEventEnvelope) -> Result<(), InterfacePolicyError> {
        let stage = parse_stage(payload_value(event, "stage")?)?;
        let cycle: u32 = payload_value(event, "cycle")?
            .parse()
            .map_err(|_| InterfacePolicyError::InvalidPayload("cycle".into()))?;
        let stages = self.stage_history();
        let same_cycle = stages
            .iter()
            .filter(|(record_cycle, _, _)| *record_cycle == cycle)
            .collect::<Vec<_>>();
        if same_cycle.is_empty() {
            if cycle == 0 {
                if stage != JudicialStage::Conciliation {
                    return Err(InterfacePolicyError::InvalidStageOrder);
                }
            } else {
                let expected = self
                    .runtime
                    .aggregate_history(&self.case)
                    .into_iter()
                    .rev()
                    .find(|prior| prior.kind == InterfaceEventKind::RestitutionRecurrence)
                    .ok_or(InterfacePolicyError::InvalidStageOrder)?;
                let to_cycle: u32 = payload_value(expected, "to-cycle")?
                    .parse()
                    .map_err(|_| InterfacePolicyError::InvalidPayload("to-cycle".into()))?;
                let return_stage = parse_stage(payload_value(expected, "return-stage")?)?;
                if to_cycle != cycle || return_stage != stage {
                    return Err(InterfacePolicyError::InvalidStageOrder);
                }
            }
        } else {
            let last = same_cycle
                .iter()
                .max_by_key(|(_, prior, _)| prior.semantic_order())
                .expect("nonempty stage list")
                .1;
            if last.semantic_order() + 1 != stage.semantic_order() {
                return Err(InterfacePolicyError::InvalidStageOrder);
            }
        }
        Ok(())
    }

    fn require_stage(&self, required: JudicialStage) -> Result<(), InterfacePolicyError> {
        if self
            .stage_history()
            .iter()
            .any(|(_, stage, _)| *stage == required)
        {
            Ok(())
        } else {
            Err(InterfacePolicyError::RequiredStageMissing(required))
        }
    }

    pub fn replay(
        events: impl IntoIterator<Item = InterfaceEventEnvelope>,
    ) -> Result<Self, InterfacePolicyError> {
        let mut events = events.into_iter().collect::<Vec<_>>();
        events.sort_by_key(|event| (event.semantic_sequence, event.event_id.as_str().to_owned()));
        let opening = events
            .iter()
            .find(|event| event.kind == InterfaceEventKind::CourtCaseOpened)
            .ok_or(InterfacePolicyError::MissingCaseOpening)?;
        let mut state = Self {
            case: opening.source_aggregate.clone(),
            runtime: SharedInterfaceRuntime::default(),
        };
        for event in events {
            state.apply(event)?;
        }
        Ok(state)
    }

    pub fn encode(&self) -> Result<Vec<u8>, InterfacePolicyError> {
        encode_shared_interface_archive(&self.runtime).map_err(Into::into)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, InterfacePolicyError> {
        let runtime = decode_shared_interface_archive(bytes)?;
        Self::replay(runtime.events().into_iter().cloned())
    }
}

#[allow(clippy::too_many_arguments)]
pub fn court_event(
    event_id: InterfaceEventId,
    case: &InterfaceAggregateId,
    kind: InterfaceEventKind,
    semantic_sequence: u64,
    authority: InterfaceIdentityRef,
    evidence: Vec<InterfaceIdentityRef>,
    payload: BTreeMap<String, String>,
    causal_parent: Option<InterfaceEventId>,
) -> InterfaceEventEnvelope {
    InterfaceEventEnvelope {
        event_id,
        source_aggregate: case.clone(),
        source_domain: InterfaceDomain::MinoanCourt,
        receiving_aggregate: case.clone(),
        receiving_domain: match kind {
            InterfaceEventKind::RestitutionOrdered
            | InterfaceEventKind::RestitutionDelivered
            | InterfaceEventKind::RestitutionFailed
            | InterfaceEventKind::RestitutionRecurrence
            | InterfaceEventKind::CourtEquilibriumConfirmed => InterfaceDomain::Restitution,
            _ => InterfaceDomain::MinoanCourt,
        },
        authority,
        subjects: Vec::new(),
        evidence,
        kind,
        semantic_sequence,
        causal_parent,
        related_case: Some(case.clone()),
        related_amendment: None,
        responsible_remedy_institution: None,
        failure_state: InterfaceFailureState::None,
        migration_version: SHARED_INTERFACE_ARCHIVE_VERSION,
        historical_links: Vec::new(),
        payload,
    }
}

#[allow(clippy::too_many_arguments)]
pub fn court_stage_event(
    event_id: InterfaceEventId,
    case: &InterfaceAggregateId,
    stage: JudicialStage,
    cycle: u32,
    semantic_sequence: u64,
    authority: InterfaceIdentityRef,
    evidence: Vec<InterfaceIdentityRef>,
    causal_parent: Option<InterfaceEventId>,
) -> InterfaceEventEnvelope {
    court_event(
        event_id,
        case,
        InterfaceEventKind::CourtStageTransition,
        semantic_sequence,
        authority,
        evidence,
        BTreeMap::from([
            ("cycle".into(), cycle.to_string()),
            ("stage".into(), stage_name(stage).into()),
        ]),
        causal_parent,
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DurableAmendmentProposal {
    pub identity: IdentityId,
    pub exact_text: String,
    pub superseded_text: String,
    pub proposer: IdentityId,
    pub proposer_standing: String,
    pub affected_houses: BTreeSet<House>,
    pub affected_offices: BTreeSet<String>,
    pub affected_titles: BTreeSet<String>,
    pub affected_communities: BTreeSet<String>,
    pub altered_authority: String,
    pub expected_yield: String,
    pub scope: AmendmentScope,
    pub prior_version: Option<IdentityId>,
}

impl DurableAmendmentProposal {
    pub fn validate(&self) -> Result<(), InterfacePolicyError> {
        self.scope
            .validate()
            .map_err(|error| InterfacePolicyError::Domain(error.to_string()))?;
        if self.exact_text.trim().is_empty()
            || self.proposer_standing.trim().is_empty()
            || self.altered_authority.trim().is_empty()
            || self.expected_yield.trim().is_empty()
            || self.affected_houses != self.scope.required_houses()
        {
            return Err(InterfacePolicyError::InvalidAmendment);
        }
        Ok(())
    }

    fn scope_name(&self) -> &'static str {
        match self.scope {
            AmendmentScope::HouseLocal(_) => "house-local",
            AmendmentScope::CrossHouse(_) => "cross-house",
            AmendmentScope::Foundational => "foundational",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AmendmentInterfaceRuntime {
    amendment: InterfaceAggregateId,
    runtime: SharedInterfaceRuntime,
}

impl AmendmentInterfaceRuntime {
    pub fn propose(
        event_id: InterfaceEventId,
        proposal: &DurableAmendmentProposal,
        authority: InterfaceIdentityRef,
        evidence: Vec<InterfaceIdentityRef>,
    ) -> Result<Self, InterfacePolicyError> {
        proposal.validate()?;
        let amendment = aggregate(&proposal.identity)?;
        let event = InterfaceEventEnvelope {
            event_id,
            source_aggregate: amendment.clone(),
            source_domain: InterfaceDomain::Amendment,
            receiving_aggregate: amendment.clone(),
            receiving_domain: InterfaceDomain::Amendment,
            authority,
            subjects: vec![person_identity_ref(
                &proposal.proposer,
                SHARED_INTERFACE_AUTHORITY_SOURCE,
            )?],
            evidence,
            kind: InterfaceEventKind::AmendmentProposed,
            semantic_sequence: 0,
            causal_parent: None,
            related_case: None,
            related_amendment: Some(amendment.clone()),
            responsible_remedy_institution: None,
            failure_state: InterfaceFailureState::None,
            migration_version: SHARED_INTERFACE_ARCHIVE_VERSION,
            historical_links: Vec::new(),
            payload: BTreeMap::from([
                (
                    "affected-communities".into(),
                    proposal
                        .affected_communities
                        .iter()
                        .cloned()
                        .collect::<Vec<_>>()
                        .join(","),
                ),
                (
                    "affected-houses".into(),
                    canonical_house_list(&proposal.affected_houses),
                ),
                (
                    "affected-offices".into(),
                    proposal
                        .affected_offices
                        .iter()
                        .cloned()
                        .collect::<Vec<_>>()
                        .join(","),
                ),
                (
                    "affected-titles".into(),
                    proposal
                        .affected_titles
                        .iter()
                        .cloned()
                        .collect::<Vec<_>>()
                        .join(","),
                ),
                (
                    "altered-authority".into(),
                    proposal.altered_authority.clone(),
                ),
                ("exact-text".into(), proposal.exact_text.clone()),
                ("expected-yield".into(), proposal.expected_yield.clone()),
                (
                    "proposer-standing".into(),
                    proposal.proposer_standing.clone(),
                ),
                ("scope".into(), proposal.scope_name().into()),
                (
                    "superseded-text".into(),
                    if proposal.superseded_text.is_empty() {
                        "none".into()
                    } else {
                        proposal.superseded_text.clone()
                    },
                ),
            ]),
        };
        let mut state = Self {
            amendment,
            runtime: SharedInterfaceRuntime::default(),
        };
        if let Some(prior) = &proposal.prior_version {
            let link =
                InterfaceEventId::new(format!("event.amendment.{}.superseded", prior.as_str()))?;
            // The link may refer to an archived predecessor outside this runtime.
            // Historical links are identifiers, not authority-bearing live parents.
            let mut event = event;
            event.historical_links.push(link);
            state.apply(event)?;
        } else {
            state.apply(event)?;
        }
        Ok(state)
    }

    #[must_use]
    pub fn runtime(&self) -> &SharedInterfaceRuntime {
        &self.runtime
    }

    #[must_use]
    pub fn amendment(&self) -> &InterfaceAggregateId {
        &self.amendment
    }

    fn history(&self) -> Vec<&InterfaceEventEnvelope> {
        self.runtime.aggregate_history(&self.amendment)
    }

    fn has(&self, kind: InterfaceEventKind) -> bool {
        self.history().iter().any(|event| event.kind == kind)
    }

    fn proposal(&self) -> Result<&InterfaceEventEnvelope, InterfacePolicyError> {
        self.history()
            .into_iter()
            .find(|event| event.kind == InterfaceEventKind::AmendmentProposed)
            .ok_or(InterfacePolicyError::InvalidAmendment)
    }

    pub fn apply(
        &mut self,
        event: InterfaceEventEnvelope,
    ) -> Result<InterfaceApplyOutcome, InterfacePolicyError> {
        if let Some(existing) = self.runtime.event(&event.event_id) {
            return if existing == &event {
                Ok(InterfaceApplyOutcome::Idempotent)
            } else {
                Err(SharedInterfaceError::ConflictingEventIdentity(event.event_id).into())
            };
        }
        if event.source_aggregate != self.amendment
            || event.related_amendment.as_ref() != Some(&self.amendment)
            || event.source_domain != InterfaceDomain::Amendment
        {
            return Err(InterfacePolicyError::AmendmentIdentityMismatch);
        }
        let history = self.history();
        match event.kind {
            InterfaceEventKind::AmendmentProposed => {
                if !history.is_empty() {
                    return Err(InterfacePolicyError::InvalidAmendmentOrder);
                }
            }
            InterfaceEventKind::AmendmentPublicNotice => {
                self.require(InterfaceEventKind::AmendmentProposed)?;
            }
            InterfaceEventKind::AmendmentTestimony => {
                self.require(InterfaceEventKind::AmendmentPublicNotice)?;
            }
            InterfaceEventKind::AmendmentProcessCertified => {
                self.require(InterfaceEventKind::AmendmentTestimony)?;
                if payload_value(&event, "court-ratified")? != "false"
                    || payload_value(&event, "path-lawful")? != "true"
                {
                    return Err(InterfacePolicyError::CourtCannotRatify);
                }
            }
            InterfaceEventKind::AmendmentHouseRatified => {
                self.require(InterfaceEventKind::AmendmentProcessCertified)?;
                let house = parse_house_list(payload_value(&event, "house")?)?;
                if house.len() != 1 {
                    return Err(InterfacePolicyError::InvalidRatification);
                }
                let required =
                    parse_house_list(payload_value(self.proposal()?, "affected-houses")?)?;
                if !house.is_subset(&required) {
                    return Err(InterfacePolicyError::InvalidRatification);
                }
                let already = history
                    .iter()
                    .filter(|prior| prior.kind == InterfaceEventKind::AmendmentHouseRatified)
                    .filter_map(|prior| prior.payload.get("house"))
                    .flat_map(|value| parse_house_list(value).unwrap_or_default())
                    .collect::<BTreeSet<_>>();
                if !already.is_disjoint(&house) {
                    return Err(InterfacePolicyError::DuplicateHouseRatification);
                }
            }
            InterfaceEventKind::AmendmentStonebendSealed => {
                let required =
                    parse_house_list(payload_value(self.proposal()?, "affected-houses")?)?;
                let assents = history
                    .iter()
                    .filter(|prior| prior.kind == InterfaceEventKind::AmendmentHouseRatified)
                    .filter_map(|prior| prior.payload.get("house"))
                    .flat_map(|value| parse_house_list(value).unwrap_or_default())
                    .collect::<BTreeSet<_>>();
                if assents != required || payload_value(&event, "court-ratified")? != "false" {
                    return Err(InterfacePolicyError::SealBeforeRatification);
                }
            }
            InterfaceEventKind::AmendmentImplemented => {
                self.require(InterfaceEventKind::AmendmentStonebendSealed)?;
                if payload_value(&event, "implemented-text")?
                    != payload_value(self.proposal()?, "exact-text")?
                {
                    return Err(InterfacePolicyError::ImplementationExceedsRatification);
                }
            }
            InterfaceEventKind::AmendmentRestitutionReviewed => {
                self.require(InterfaceEventKind::AmendmentImplemented)?;
                if payload_value(&event, "ratified-text")?
                    != payload_value(self.proposal()?, "exact-text")?
                {
                    return Err(InterfacePolicyError::RestitutionRewritesAmendment);
                }
            }
            _ => return Err(InterfacePolicyError::WrongAggregateEvent(event.kind)),
        }
        self.runtime.apply(event).map_err(Into::into)
    }

    fn require(&self, kind: InterfaceEventKind) -> Result<(), InterfacePolicyError> {
        if self.has(kind) {
            Ok(())
        } else {
            Err(InterfacePolicyError::InvalidAmendmentOrder)
        }
    }

    pub fn replay(
        events: impl IntoIterator<Item = InterfaceEventEnvelope>,
    ) -> Result<Self, InterfacePolicyError> {
        let mut events = events.into_iter().collect::<Vec<_>>();
        events.sort_by_key(|event| (event.semantic_sequence, event.event_id.as_str().to_owned()));
        let proposal = events
            .iter()
            .find(|event| event.kind == InterfaceEventKind::AmendmentProposed)
            .ok_or(InterfacePolicyError::InvalidAmendment)?;
        let mut state = Self {
            amendment: proposal.source_aggregate.clone(),
            runtime: SharedInterfaceRuntime::default(),
        };
        for event in events {
            state.apply(event)?;
        }
        Ok(state)
    }

    pub fn encode(&self) -> Result<Vec<u8>, InterfacePolicyError> {
        encode_shared_interface_archive(&self.runtime).map_err(Into::into)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, InterfacePolicyError> {
        let runtime = decode_shared_interface_archive(bytes)?;
        Self::replay(runtime.events().into_iter().cloned())
    }
}

#[allow(clippy::too_many_arguments)]
pub fn amendment_event(
    event_id: InterfaceEventId,
    amendment: &InterfaceAggregateId,
    kind: InterfaceEventKind,
    semantic_sequence: u64,
    authority: InterfaceIdentityRef,
    evidence: Vec<InterfaceIdentityRef>,
    payload: BTreeMap<String, String>,
    causal_parent: Option<InterfaceEventId>,
) -> InterfaceEventEnvelope {
    InterfaceEventEnvelope {
        event_id,
        source_aggregate: amendment.clone(),
        source_domain: InterfaceDomain::Amendment,
        receiving_aggregate: amendment.clone(),
        receiving_domain: if kind == InterfaceEventKind::AmendmentRestitutionReviewed {
            InterfaceDomain::Restitution
        } else {
            InterfaceDomain::Amendment
        },
        authority,
        subjects: Vec::new(),
        evidence,
        kind,
        semantic_sequence,
        causal_parent,
        related_case: None,
        related_amendment: Some(amendment.clone()),
        responsible_remedy_institution: None,
        failure_state: InterfaceFailureState::None,
        migration_version: SHARED_INTERFACE_ARCHIVE_VERSION,
        historical_links: Vec::new(),
        payload,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum HouseRemedyAction {
    StonebendTitleCorrection,
    StonebendGateScopeRestoration,
    StonebendSealCorrection,
    StonebendProvenanceRestoration,
    StonebendRecordCorrection,
    StonebendMaterialReturn,
    StonebendHollowingCorrection,
    StonebendBoundaryRestoration,
    SandmanorDesignCorrection,
    SandmanorFormationRepair,
    SandmanorReciprocalDutyRestoration,
    SandmanorGuardianshipCorrection,
    SandmanorContestCorrection,
    SandmanorCoastalAccessRestoration,
    SandmanorPublicResponsibilityRepair,
    GlaushouseCareDelivery,
    GlaushouseTransfer,
    GlaushouseCompatibilityReview,
    GlaushouseConsentCorrection,
    GlaushouseMaintainedSynthesisRepair,
    GlaushouseClinicalRecordCorrection,
    GlaushouseFunctionalRestoration,
    FlyntTechnicalShutdown,
    FlyntDeploymentRestriction,
    FlyntInfrastructureRepair,
    FlyntOperationalCorrection,
    FlyntPersistenceReevaluation,
    FlyntFunctionalRestoration,
    CentralJunctionTransactionCorrection,
    CentralJunctionSettlementCorrection,
    CentralJunctionMeasureCorrection,
    CentralJunctionProvenanceCorrection,
    CentralJunctionCirculationCorrection,
    CentralJunctionClearingCorrection,
    CentralJunctionPublicationCorrection,
}

impl HouseRemedyAction {
    #[must_use]
    pub const fn responsible_institution(self) -> ResponsibleInstitution {
        match self {
            Self::StonebendTitleCorrection
            | Self::StonebendGateScopeRestoration
            | Self::StonebendSealCorrection
            | Self::StonebendProvenanceRestoration
            | Self::StonebendRecordCorrection
            | Self::StonebendMaterialReturn
            | Self::StonebendHollowingCorrection
            | Self::StonebendBoundaryRestoration => ResponsibleInstitution::Stonebend,
            Self::SandmanorDesignCorrection
            | Self::SandmanorFormationRepair
            | Self::SandmanorReciprocalDutyRestoration
            | Self::SandmanorGuardianshipCorrection
            | Self::SandmanorContestCorrection
            | Self::SandmanorCoastalAccessRestoration
            | Self::SandmanorPublicResponsibilityRepair => ResponsibleInstitution::Sandmanor,
            Self::GlaushouseCareDelivery
            | Self::GlaushouseTransfer
            | Self::GlaushouseCompatibilityReview
            | Self::GlaushouseConsentCorrection
            | Self::GlaushouseMaintainedSynthesisRepair
            | Self::GlaushouseClinicalRecordCorrection
            | Self::GlaushouseFunctionalRestoration => ResponsibleInstitution::Glaushouse,
            Self::FlyntTechnicalShutdown
            | Self::FlyntDeploymentRestriction
            | Self::FlyntInfrastructureRepair
            | Self::FlyntOperationalCorrection
            | Self::FlyntPersistenceReevaluation
            | Self::FlyntFunctionalRestoration => ResponsibleInstitution::Flynt,
            Self::CentralJunctionTransactionCorrection
            | Self::CentralJunctionSettlementCorrection
            | Self::CentralJunctionMeasureCorrection
            | Self::CentralJunctionProvenanceCorrection
            | Self::CentralJunctionCirculationCorrection
            | Self::CentralJunctionClearingCorrection
            | Self::CentralJunctionPublicationCorrection => ResponsibleInstitution::CentralJunction,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HouseRemedyExecution {
    pub identity: IdentityId,
    pub case: IdentityId,
    pub remedy: IdentityId,
    pub action: HouseRemedyAction,
    pub performed_by: InstitutionId,
    pub authority_source: String,
    pub delivery_evidence: Vec<EvidenceRecordId>,
    pub completion_condition: String,
    pub completed: bool,
    pub punishment: bool,
}

impl HouseRemedyExecution {
    pub fn validate(&self) -> Result<(), InterfacePolicyError> {
        if self.authority_source.trim().is_empty()
            || self.delivery_evidence.is_empty()
            || self.completion_condition.trim().is_empty()
            || self.punishment
        {
            return Err(InterfacePolicyError::InvalidRemedyExecution);
        }
        Ok(())
    }

    pub fn to_event(
        &self,
        event_id: InterfaceEventId,
        semantic_sequence: u64,
        authority: InterfaceIdentityRef,
        causal_parent: Option<InterfaceEventId>,
    ) -> Result<InterfaceEventEnvelope, InterfacePolicyError> {
        self.validate()?;
        let case = aggregate(&self.case)?;
        Ok(InterfaceEventEnvelope {
            event_id,
            source_aggregate: case.clone(),
            source_domain: InterfaceDomain::Restitution,
            receiving_aggregate: InterfaceAggregateId::new(self.performed_by.as_str())?,
            receiving_domain: self
                .action
                .responsible_institution()
                .domain_house()
                .map(InterfaceDomain::House)
                .unwrap_or(InterfaceDomain::CentralJunction),
            authority,
            subjects: vec![remedy_identity_ref(&self.remedy)?],
            evidence: self
                .delivery_evidence
                .iter()
                .map(|record| evidence_identity_ref(record, &self.authority_source))
                .collect::<Result<_, _>>()?,
            kind: InterfaceEventKind::HouseRemedyExecuted,
            semantic_sequence,
            causal_parent,
            related_case: Some(case),
            related_amendment: None,
            responsible_remedy_institution: Some(institution_identity_ref(
                &self.performed_by,
                &self.authority_source,
            )?),
            failure_state: if self.completed {
                InterfaceFailureState::None
            } else {
                InterfaceFailureState::RemediationRequired {
                    code: "remedy-incomplete".into(),
                }
            },
            migration_version: SHARED_INTERFACE_ARCHIVE_VERSION,
            historical_links: Vec::new(),
            payload: BTreeMap::from([
                ("action".into(), format!("{:?}", self.action)),
                ("completed".into(), self.completed.to_string()),
                (
                    "completion-condition".into(),
                    self.completion_condition.clone(),
                ),
            ]),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmergencyCourtTransfer {
    pub identity: IdentityId,
    pub originating_institution: InstitutionId,
    pub legal_authority: IdentityId,
    pub person: IdentityId,
    pub custody_status: String,
    pub emergency_evidence: Vec<EvidenceRecordId>,
    pub transfer_reason: String,
    pub receiving_glaushouse_institution: InstitutionId,
    pub clinical_acknowledgment: EvidenceRecordId,
    pub care_accepted: bool,
    pub lawful_rejection_reason: Option<String>,
    pub changed_custody_status: String,
    pub treatment_boundary: String,
    pub return_or_discharge: String,
    pub restitution_case: Option<IdentityId>,
}

impl EmergencyCourtTransfer {
    pub fn validate(&self) -> Result<(), InterfacePolicyError> {
        if self.originating_institution
            != crate::world::sandmanor::milestone::minoan_county_courthouse_id()
            || self.receiving_glaushouse_institution != crate::world::glaushouse::glauspitals_id()
            || self.custody_status.trim().is_empty()
            || self.emergency_evidence.is_empty()
            || self.transfer_reason.trim().is_empty()
            || self.changed_custody_status.trim().is_empty()
            || self.treatment_boundary.trim().is_empty()
            || self.return_or_discharge.trim().is_empty()
            || (!self.care_accepted
                && self
                    .lawful_rejection_reason
                    .as_ref()
                    .is_none_or(|reason| reason.trim().is_empty()))
        {
            return Err(InterfacePolicyError::InvalidEmergencyTransfer);
        }
        Ok(())
    }

    pub fn events(
        &self,
        event_ids: [InterfaceEventId; 4],
    ) -> Result<Vec<InterfaceEventEnvelope>, InterfacePolicyError> {
        self.validate()?;
        let transfer = aggregate(&self.identity)?;
        let authority = person_identity_ref(
            &self.legal_authority,
            "MINOAN_COUNTY_COURT_SYSTEM_AND_RESTITUTION_CYCLE_V1.md",
        )?;
        let person = person_identity_ref(
            &self.person,
            "MINOAN_COUNTY_COURT_SYSTEM_AND_RESTITUTION_CYCLE_V1.md",
        )?;
        let evidence = self
            .emergency_evidence
            .iter()
            .map(|record| {
                evidence_identity_ref(
                    record,
                    "MINOAN_COUNTY_COURT_SYSTEM_AND_RESTITUTION_CYCLE_V1.md",
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        let related_case = self.restitution_case.as_ref().map(aggregate).transpose()?;
        let common = |event_id: InterfaceEventId,
                      kind: InterfaceEventKind,
                      sequence: u64,
                      parent: Option<InterfaceEventId>,
                      payload: BTreeMap<String, String>|
         -> InterfaceEventEnvelope {
            InterfaceEventEnvelope {
                event_id,
                source_aggregate: transfer.clone(),
                source_domain: InterfaceDomain::EmergencyTransfer,
                receiving_aggregate: transfer.clone(),
                receiving_domain: InterfaceDomain::House(House::Glaushouse),
                authority: authority.clone(),
                subjects: vec![person.clone()],
                evidence: evidence.clone(),
                kind,
                semantic_sequence: sequence,
                causal_parent: parent,
                related_case: related_case.clone(),
                related_amendment: None,
                responsible_remedy_institution: None,
                failure_state: InterfaceFailureState::None,
                migration_version: SHARED_INTERFACE_ARCHIVE_VERSION,
                historical_links: Vec::new(),
                payload,
            }
        };
        let opened = common(
            event_ids[0].clone(),
            InterfaceEventKind::EmergencyTransferOpened,
            0,
            None,
            BTreeMap::from([
                ("custody-status".into(), self.custody_status.clone()),
                ("reason".into(), self.transfer_reason.clone()),
            ]),
        );
        let acknowledged = common(
            event_ids[1].clone(),
            InterfaceEventKind::EmergencyTransferAcknowledged,
            1,
            Some(event_ids[0].clone()),
            BTreeMap::from([
                ("care-accepted".into(), self.care_accepted.to_string()),
                (
                    "clinical-acknowledgment".into(),
                    self.clinical_acknowledgment.as_str().into(),
                ),
                (
                    "lawful-rejection-reason".into(),
                    self.lawful_rejection_reason
                        .clone()
                        .unwrap_or_else(|| "none".into()),
                ),
            ]),
        );
        let custody = common(
            event_ids[2].clone(),
            InterfaceEventKind::EmergencyTransferCustodyChanged,
            2,
            Some(event_ids[1].clone()),
            BTreeMap::from([
                (
                    "changed-custody-status".into(),
                    self.changed_custody_status.clone(),
                ),
                ("treatment-boundary".into(), self.treatment_boundary.clone()),
            ]),
        );
        let discharge = common(
            event_ids[3].clone(),
            InterfaceEventKind::EmergencyTransferDischarged,
            3,
            Some(event_ids[2].clone()),
            BTreeMap::from([(
                "return-or-discharge".into(),
                self.return_or_discharge.clone(),
            )]),
        );
        Ok(vec![opened, acknowledged, custody, discharge])
    }
}

pub fn persist_interface_events(
    events: impl IntoIterator<Item = InterfaceEventEnvelope>,
) -> Result<Vec<u8>, InterfacePolicyError> {
    let runtime = SharedInterfaceRuntime::replay(events)?;
    encode_shared_interface_archive(&runtime).map_err(Into::into)
}

pub fn replay_interface_archive(
    bytes: &[u8],
) -> Result<SharedInterfaceRuntime, InterfacePolicyError> {
    decode_shared_interface_archive(bytes).map_err(Into::into)
}

pub fn migrate_interface_archive(bytes: &[u8]) -> Result<Vec<u8>, InterfacePolicyError> {
    migrate_shared_interface_archive(bytes).map_err(Into::into)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterfacePolicyError {
    Shared(SharedInterfaceError),
    Domain(String),
    CaseIdentityMismatch,
    AmendmentIdentityMismatch,
    DuplicateCaseOpening,
    MissingCaseOpening,
    ClosedCase,
    InvalidStageOrder,
    RequiredStageMissing(JudicialStage),
    RecurrenceWithoutFailure,
    InvalidRecurrence,
    ClosureBeforeEquilibrium,
    InvalidPayload(String),
    WrongAggregateEvent(InterfaceEventKind),
    InvalidAmendment,
    InvalidAmendmentOrder,
    CourtCannotRatify,
    InvalidRatification,
    DuplicateHouseRatification,
    SealBeforeRatification,
    ImplementationExceedsRatification,
    RestitutionRewritesAmendment,
    InvalidRemedyExecution,
    InvalidEmergencyTransfer,
}

impl From<SharedInterfaceError> for InterfacePolicyError {
    fn from(value: SharedInterfaceError) -> Self {
        Self::Shared(value)
    }
}

impl fmt::Display for InterfacePolicyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "constitutional interface policy error: {self:?}")
    }
}

impl std::error::Error for InterfacePolicyError {}
