//! Character-driven, cross-region campaign built over the living-world cases.
//!
//! Deep Pressure does not replace a local duty officer or any House act. It
//! links already-authored operational records, classified speech, remembered
//! consequences, and a final Boardwalk settlement into one replayable arc.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::constitutional::{
    BondActivation, BondEvent, BondFormation, BondId, BondParticipant, BondTerm, BondValidation,
    CausalPosition, ConstitutionalEventId, ConstitutionalRuntime, EventMetadata, EvidenceRef,
    HouseDecision, HouseFunction, InitialCurrent, InstitutionalJurisdictionSnapshot, ObligationId,
    ParticipantId, ParticipantKind, PermissionId, RoleId, RuleSetId, Sign, SignedQuantity, UnitId,
    WaveId, WaveRecord, scenario_house_decision,
};
use crate::hollow_grove_contract::House;
use crate::world::geography::ConstitutionalRouteId;
use crate::world::house_institutions::stonebend_constitution_id;
use crate::world::session::WorldSession;

use super::{
    CardinalDirection, LivingCaseChoice, LivingCaseId, LivingClock, LivingEvidence,
    LivingWorldEvent, LivingWorldState, ScheduledLocation, TilePosition, WorkShift, WorldMapId,
    map_definition,
};

pub const DEEP_PRESSURE_CASE_ID: &str = "case.arc.hollow-grove.deep-pressure.v1";
pub const DEEP_PRESSURE_ASSEMBLY_ID: &str = "participant.boardwalk.deep-pressure-assembly";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DeepPressurePersonId {
    BrindleReed,
    SellaWindward,
    HarrowVale,
    OrenPike,
    MaelaDownroad,
    BramBurden,
    CorinWake,
    IonaDepth,
    PelMarrow,
    TessBreakwater,
}

impl DeepPressurePersonId {
    pub const ALL: [Self; 10] = [
        Self::BrindleReed,
        Self::SellaWindward,
        Self::HarrowVale,
        Self::OrenPike,
        Self::MaelaDownroad,
        Self::BramBurden,
        Self::CorinWake,
        Self::IonaDepth,
        Self::PelMarrow,
        Self::TessBreakwater,
    ];

    #[must_use]
    pub const fn stable_id(self) -> &'static str {
        match self {
            Self::BrindleReed => "person.brindle-reed",
            Self::SellaWindward => "person.sella-windward",
            Self::HarrowVale => "person.harrow-vale",
            Self::OrenPike => "person.oren-pike",
            Self::MaelaDownroad => "person.maela-downroad",
            Self::BramBurden => "person.bram-burden",
            Self::CorinWake => "person.corin-wake",
            Self::IonaDepth => "person.iona-depth",
            Self::PelMarrow => "person.pel-marrow",
            Self::TessBreakwater => "person.tess-breakwater",
        }
    }

    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::BrindleReed => "Brindle Reed",
            Self::SellaWindward => "Sella Windward",
            Self::HarrowVale => "Harrow Vale",
            Self::OrenPike => "Oren Pike",
            Self::MaelaDownroad => "Maela Downroad",
            Self::BramBurden => "Bram Burden",
            Self::CorinWake => "Corin Wake",
            Self::IonaDepth => "Iona Depth",
            Self::PelMarrow => "Pel Marrow",
            Self::TessBreakwater => "Tess Breakwater",
        }
    }

    #[must_use]
    pub const fn initials(self) -> &'static str {
        match self {
            Self::BrindleReed => "BR",
            Self::SellaWindward => "SW",
            Self::HarrowVale => "HV",
            Self::OrenPike => "OP",
            Self::MaelaDownroad => "MD",
            Self::BramBurden => "BB",
            Self::CorinWake => "CW",
            Self::IonaDepth => "ID",
            Self::PelMarrow => "PM",
            Self::TessBreakwater => "TB",
        }
    }

    #[must_use]
    pub fn from_stable_id(value: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|person| person.stable_id() == value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpeechClassification {
    ConstitutionalFact,
    PublicRecord,
    LocalTradition,
    Rumor,
    PrivateBelief,
    DeliberateDeception,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JournalEvidenceKind {
    OperationalRecord,
    CapableSubjectStatement,
    CustodyRecord,
    LocalClaim,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DeepPressureStatementId {
    Person(DeepPressurePersonId),
    BoardwalkPimpPitch,
    BoardwalkGimpPitch,
}

impl DeepPressureStatementId {
    pub const REQUIRED: [Self; 12] = [
        Self::Person(DeepPressurePersonId::BrindleReed),
        Self::Person(DeepPressurePersonId::SellaWindward),
        Self::Person(DeepPressurePersonId::HarrowVale),
        Self::Person(DeepPressurePersonId::OrenPike),
        Self::Person(DeepPressurePersonId::MaelaDownroad),
        Self::Person(DeepPressurePersonId::BramBurden),
        Self::Person(DeepPressurePersonId::CorinWake),
        Self::Person(DeepPressurePersonId::IonaDepth),
        Self::Person(DeepPressurePersonId::PelMarrow),
        Self::Person(DeepPressurePersonId::TessBreakwater),
        Self::BoardwalkPimpPitch,
        Self::BoardwalkGimpPitch,
    ];

    #[must_use]
    pub fn stable_id(self) -> String {
        match self {
            Self::Person(person) => {
                format!("statement.deep-pressure.{}", person.stable_id())
            }
            Self::BoardwalkPimpPitch => "statement.deep-pressure.boardwalk-pimp-pitch".into(),
            Self::BoardwalkGimpPitch => "statement.deep-pressure.boardwalk-gimp-pitch".into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeepPressureEvidenceId {
    Operational(LivingEvidence),
    Statement(DeepPressureStatementId),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceJournalRecord {
    pub evidence_id: DeepPressureEvidenceId,
    pub stable_id: String,
    pub source: String,
    pub kind: JournalEvidenceKind,
    pub speech_classification: Option<SpeechClassification>,
    pub claim: String,
    pub uncertainty: String,
    pub constitutional_effect: String,
    pub observed_day: u32,
    pub observed_shift: WorkShift,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CharacterCondition {
    Well,
    Exposed,
    Injured,
    Recovering,
    Exhausted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RelationshipMemory {
    pub person: DeepPressurePersonId,
    pub affinity: i8,
    pub reliability: i8,
    pub condition: CharacterCondition,
    pub remembered_outcomes: Vec<String>,
    pub unresolved_promises: BTreeSet<String>,
    pub boundaries: Vec<String>,
    pub constitutional_bond: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeepPressurePhase {
    RiptideEmergency,
    ShorelineAftermath,
    BurdenOfRepair,
    DepthCertification,
    GatherAffectedVoices,
    BoardwalkSettlement,
    PersistentAftermath,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeepPressureSettlementChoice {
    SharedBurdenCompact,
    CrewAndCoastRestitution,
    ProductionUnderReview,
    ProtectedRefusal,
}

impl DeepPressureSettlementChoice {
    #[must_use]
    pub const fn key(self) -> &'static str {
        match self {
            Self::SharedBurdenCompact => "shared-burden-compact",
            Self::CrewAndCoastRestitution => "crew-and-coast-restitution",
            Self::ProductionUnderReview => "production-under-review",
            Self::ProtectedRefusal => "protected-refusal",
        }
    }

    #[must_use]
    pub const fn forms_bond(self) -> bool {
        !matches!(self, Self::ProtectedRefusal)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeepPressureOutcomeId {
    SharedBurdenCompactV1,
    CrewAndCoastRestitutionV1,
    ProductionUnderReviewV1,
    ProtectedRefusalV1,
}

impl DeepPressureOutcomeId {
    #[must_use]
    pub const fn for_choice(choice: DeepPressureSettlementChoice) -> Self {
        match choice {
            DeepPressureSettlementChoice::SharedBurdenCompact => Self::SharedBurdenCompactV1,
            DeepPressureSettlementChoice::CrewAndCoastRestitution => {
                Self::CrewAndCoastRestitutionV1
            }
            DeepPressureSettlementChoice::ProductionUnderReview => Self::ProductionUnderReviewV1,
            DeepPressureSettlementChoice::ProtectedRefusal => Self::ProtectedRefusalV1,
        }
    }

    #[must_use]
    pub const fn stable_id(self) -> &'static str {
        match self {
            Self::SharedBurdenCompactV1 => "outcome.deep-pressure.shared-burden-compact.v1",
            Self::CrewAndCoastRestitutionV1 => {
                "outcome.deep-pressure.crew-and-coast-restitution.v1"
            }
            Self::ProductionUnderReviewV1 => "outcome.deep-pressure.production-under-review.v1",
            Self::ProtectedRefusalV1 => "outcome.deep-pressure.protected-refusal.v1",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HouseActRecord {
    pub function: String,
    pub decision_id: String,
    pub authority_actor: String,
    pub office: String,
    pub institution: String,
    pub causal_position: u64,
}

impl HouseActRecord {
    fn from_decision(decision: &HouseDecision) -> Self {
        Self {
            function: format!("{:?}", decision.function),
            decision_id: decision.id.as_str().into(),
            authority_actor: decision.authority.actor.as_str().into(),
            office: decision.authority.office.as_str().into(),
            institution: decision
                .authority
                .institution
                .as_ref()
                .map_or("none", |institution| institution.as_str())
                .into(),
            causal_position: decision.causal_position.get(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecoveryBondRecord {
    pub bond_id: String,
    pub formed_event_id: String,
    pub validated_event_id: String,
    pub activated_event_id: String,
    pub term_end: u64,
    pub participants: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeepPressureAftermath {
    pub crew_care: u8,
    pub coast_recovery: u8,
    pub field_security: u8,
    pub basin_repair: u8,
    pub production_posture: String,
    pub contested_certificate: bool,
    pub unresolved_obligations: Vec<String>,
    pub visible_changes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeepPressureOutcomeRecord {
    pub id: DeepPressureOutcomeId,
    pub choice: DeepPressureSettlementChoice,
    pub committed_by: String,
    pub player_support_is_nonbinding: bool,
    pub four_house_acts: Vec<HouseActRecord>,
    pub recovery_bond: Option<RecoveryBondRecord>,
    pub aftermath: DeepPressureAftermath,
    pub refusal_and_limits: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", deny_unknown_fields)]
pub enum DeepPressureEvent {
    EvidenceJournaled {
        record: EvidenceJournalRecord,
    },
    OperationalResolutionIntegrated {
        case_id: LivingCaseId,
        choice: LivingCaseChoice,
        outcome_id: String,
    },
    SettlementSupportRecorded {
        choice: DeepPressureSettlementChoice,
    },
    SettlementCommitted {
        outcome: DeepPressureOutcomeRecord,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeepPressureState {
    pub revision: u64,
    pub case_id: String,
    pub journal: Vec<EvidenceJournalRecord>,
    pub integrated_resolutions: BTreeMap<LivingCaseId, LivingCaseChoice>,
    pub relationships: BTreeMap<DeepPressurePersonId, RelationshipMemory>,
    pub supported_settlement: Option<DeepPressureSettlementChoice>,
    pub outcome: Option<DeepPressureOutcomeRecord>,
}

impl Default for DeepPressureState {
    fn default() -> Self {
        Self::new()
    }
}

impl DeepPressureState {
    #[must_use]
    pub fn new() -> Self {
        let relationships = DeepPressurePersonId::ALL
            .into_iter()
            .map(|person| {
                (
                    person,
                    RelationshipMemory {
                        person,
                        affinity: 0,
                        reliability: 0,
                        condition: CharacterCondition::Well,
                        remembered_outcomes: Vec::new(),
                        unresolved_promises: BTreeSet::new(),
                        boundaries: vec![
                            "support is advisory unless the named authority belongs to the actor"
                                .into(),
                            "no relationship creates ownership, Title, or automatic consent".into(),
                        ],
                        constitutional_bond: None,
                    },
                )
            })
            .collect();
        Self {
            revision: 0,
            case_id: DEEP_PRESSURE_CASE_ID.into(),
            journal: Vec::new(),
            integrated_resolutions: BTreeMap::new(),
            relationships,
            supported_settlement: None,
            outcome: None,
        }
    }

    pub fn validate(&self) -> Result<(), DeepPressureError> {
        if self.case_id != DEEP_PRESSURE_CASE_ID
            || self.relationships.len() != DeepPressurePersonId::ALL.len()
            || DeepPressurePersonId::ALL
                .into_iter()
                .any(|person| self.relationships.get(&person).is_none())
        {
            return Err(DeepPressureError::IncompleteCanonicalState);
        }
        let mut evidence = BTreeSet::new();
        if self
            .journal
            .iter()
            .any(|record| !evidence.insert(record.stable_id.as_str()))
        {
            return Err(DeepPressureError::DuplicateEvidence);
        }
        if self.relationships.values().any(|memory| {
            !(-5..=5).contains(&memory.affinity) || !(-5..=5).contains(&memory.reliability)
        }) {
            return Err(DeepPressureError::InvalidRelationshipMemory);
        }
        if let Some(outcome) = &self.outcome {
            if self.supported_settlement != Some(outcome.choice)
                || outcome.id != DeepPressureOutcomeId::for_choice(outcome.choice)
                || outcome.four_house_acts.len() != 4
                || outcome.recovery_bond.is_some() != outcome.choice.forms_bond()
            {
                return Err(DeepPressureError::OutcomeDivergence);
            }
        }
        Ok(())
    }

    #[must_use]
    pub fn phase(&self) -> DeepPressurePhase {
        if self.outcome.is_some() {
            return DeepPressurePhase::PersistentAftermath;
        }
        if !self
            .integrated_resolutions
            .contains_key(&LivingCaseId::RiptideWellBlowout)
        {
            return DeepPressurePhase::RiptideEmergency;
        }
        if [
            LivingCaseId::AuraBeachStormRescue,
            LivingCaseId::AuraBasinInjuredBeing,
        ]
        .into_iter()
        .any(|case| !self.integrated_resolutions.contains_key(&case))
        {
            return DeepPressurePhase::ShorelineAftermath;
        }
        if [
            LivingCaseId::AuraFieldDroughtAllocation,
            LivingCaseId::MntAuraRoofFall,
            LivingCaseId::HighwayToHellGasPocket,
        ]
        .into_iter()
        .any(|case| !self.integrated_resolutions.contains_key(&case))
        {
            return DeepPressurePhase::BurdenOfRepair;
        }
        if !self
            .integrated_resolutions
            .contains_key(&LivingCaseId::CurrentSeaWellCertification)
        {
            return DeepPressurePhase::DepthCertification;
        }
        if !self.required_statements_complete() {
            return DeepPressurePhase::GatherAffectedVoices;
        }
        DeepPressurePhase::BoardwalkSettlement
    }

    #[must_use]
    pub fn journal_contains(&self, evidence_id: DeepPressureEvidenceId) -> bool {
        self.journal
            .iter()
            .any(|record| record.evidence_id == evidence_id)
    }

    #[must_use]
    pub fn required_statements_complete(&self) -> bool {
        DeepPressureStatementId::REQUIRED
            .into_iter()
            .all(|statement| self.journal_contains(DeepPressureEvidenceId::Statement(statement)))
    }

    #[must_use]
    pub fn missing_required_statements(&self) -> Vec<DeepPressureStatementId> {
        DeepPressureStatementId::REQUIRED
            .into_iter()
            .filter(|statement| {
                !self.journal_contains(DeepPressureEvidenceId::Statement(*statement))
            })
            .collect()
    }

    #[must_use]
    pub fn ready_for_settlement_support(&self) -> bool {
        self.phase() == DeepPressurePhase::BoardwalkSettlement
            && self.supported_settlement.is_none()
            && self.outcome.is_none()
    }

    #[must_use]
    pub fn contested_certificate(&self) -> bool {
        self.integrated_resolutions
            .get(&LivingCaseId::RiptideWellBlowout)
            == Some(&LivingCaseChoice::RescueCrewFirst)
            && self
                .integrated_resolutions
                .get(&LivingCaseId::CurrentSeaWellCertification)
                == Some(&LivingCaseChoice::CertifyReducedRate)
    }

    #[must_use]
    pub fn settlement_choice_available(&self, choice: DeepPressureSettlementChoice) -> bool {
        !(choice == DeepPressureSettlementChoice::ProductionUnderReview
            && self.contested_certificate())
    }

    pub fn observe_living_event(
        &mut self,
        event: &LivingWorldEvent,
        clock: LivingClock,
    ) -> Result<Option<DeepPressureEvent>, DeepPressureError> {
        let event = match event {
            LivingWorldEvent::EvidenceObserved { evidence, .. } => {
                let id = DeepPressureEvidenceId::Operational(*evidence);
                if self.journal_contains(id) {
                    return Ok(None);
                }
                Some(DeepPressureEvent::EvidenceJournaled {
                    record: operational_record(*evidence, clock),
                })
            }
            LivingWorldEvent::CaseResolved {
                case_id,
                choice,
                outcome_id,
            } => {
                if self.integrated_resolutions.contains_key(case_id) {
                    return Ok(None);
                }
                Some(DeepPressureEvent::OperationalResolutionIntegrated {
                    case_id: *case_id,
                    choice: *choice,
                    outcome_id: outcome_id.clone(),
                })
            }
            LivingWorldEvent::CaseSupportRecorded { .. }
            | LivingWorldEvent::ShiftAdvanced { .. } => None,
        };
        let Some(event) = event else {
            return Ok(None);
        };
        let mut candidate = self.clone();
        candidate.apply(&event)?;
        *self = candidate;
        Ok(Some(event))
    }

    pub fn observe_statement(
        &mut self,
        statement: DeepPressureStatementId,
        clock: LivingClock,
    ) -> Result<Option<DeepPressureEvent>, DeepPressureError> {
        let evidence_id = DeepPressureEvidenceId::Statement(statement);
        if self.journal_contains(evidence_id) {
            return Ok(None);
        }
        let event = DeepPressureEvent::EvidenceJournaled {
            record: deep_pressure_statement_record(statement, clock),
        };
        let mut candidate = self.clone();
        candidate.apply(&event)?;
        *self = candidate;
        Ok(Some(event))
    }

    pub fn support_settlement(
        &mut self,
        choice: DeepPressureSettlementChoice,
    ) -> Result<DeepPressureEvent, DeepPressureError> {
        let event = DeepPressureEvent::SettlementSupportRecorded { choice };
        let mut candidate = self.clone();
        candidate.apply(&event)?;
        *self = candidate;
        Ok(event)
    }

    pub fn commit_settlement(
        &mut self,
        at: CausalPosition,
        rule_set: &RuleSetId,
        constitutional: &mut ConstitutionalRuntime,
        world: &WorldSession,
    ) -> Result<DeepPressureEvent, DeepPressureError> {
        if self.outcome.is_some() {
            return Err(DeepPressureError::SettlementAlreadyCommitted);
        }
        let choice = self
            .supported_settlement
            .ok_or(DeepPressureError::SettlementSupportRequired)?;
        if !self.settlement_choice_available(choice) {
            return Err(DeepPressureError::CompromisedEvidenceBarsProductionAccord);
        }
        let outcome = resolve_settlement(self, choice, at, rule_set, constitutional, world)?;
        let event = DeepPressureEvent::SettlementCommitted { outcome };
        let mut candidate = self.clone();
        candidate.apply(&event)?;
        *self = candidate;
        Ok(event)
    }

    pub fn apply(&mut self, event: &DeepPressureEvent) -> Result<(), DeepPressureError> {
        match event {
            DeepPressureEvent::EvidenceJournaled { record } => {
                if self.journal_contains(record.evidence_id)
                    || record.stable_id != stable_evidence_id(record.evidence_id)
                    || record.constitutional_effect.is_empty()
                {
                    return Err(DeepPressureError::DuplicateOrInvalidEvidence);
                }
                let classification_valid = match record.evidence_id {
                    DeepPressureEvidenceId::Operational(_) => {
                        record.speech_classification.is_none()
                    }
                    DeepPressureEvidenceId::Statement(_) => record.speech_classification.is_some(),
                };
                if !classification_valid {
                    return Err(DeepPressureError::InvalidSpeechClassification);
                }
                self.journal.push(record.clone());
            }
            DeepPressureEvent::OperationalResolutionIntegrated {
                case_id,
                choice,
                outcome_id,
            } => {
                if self.integrated_resolutions.contains_key(case_id) {
                    return Err(DeepPressureError::ResolutionAlreadyIntegrated(*case_id));
                }
                if !choice.belongs_to(*case_id) || choice.is_forbidden() {
                    return Err(DeepPressureError::InvalidOperationalResolution {
                        case_id: *case_id,
                        choice: *choice,
                    });
                }
                let expected = format!(
                    "outcome.{}.{}",
                    case_id.stable_id(),
                    living_choice_key(*choice)
                );
                if outcome_id != &expected {
                    return Err(DeepPressureError::OutcomeDivergence);
                }
                self.integrated_resolutions.insert(*case_id, *choice);
                self.remember_operational_resolution(*case_id, *choice)?;
            }
            DeepPressureEvent::SettlementSupportRecorded { choice } => {
                if self.outcome.is_some() {
                    return Err(DeepPressureError::SettlementAlreadyCommitted);
                }
                if self.supported_settlement.is_some() {
                    return Err(DeepPressureError::SettlementSupportAlreadyRecorded);
                }
                if !self.ready_for_settlement_support() {
                    return Err(DeepPressureError::SettlementNotReady);
                }
                if !self.settlement_choice_available(*choice) {
                    return Err(DeepPressureError::CompromisedEvidenceBarsProductionAccord);
                }
                self.supported_settlement = Some(*choice);
            }
            DeepPressureEvent::SettlementCommitted { outcome } => {
                if self.outcome.is_some() {
                    return Err(DeepPressureError::SettlementAlreadyCommitted);
                }
                if self.supported_settlement != Some(outcome.choice)
                    || outcome.id != DeepPressureOutcomeId::for_choice(outcome.choice)
                    || outcome.committed_by != DEEP_PRESSURE_ASSEMBLY_ID
                    || !outcome.player_support_is_nonbinding
                    || outcome.four_house_acts.len() != 4
                    || outcome.recovery_bond.is_some() != outcome.choice.forms_bond()
                {
                    return Err(DeepPressureError::OutcomeDivergence);
                }
                self.remember_settlement(outcome);
                self.outcome = Some(outcome.clone());
            }
        }
        self.revision = self
            .revision
            .checked_add(1)
            .ok_or(DeepPressureError::RevisionOverflow)?;
        self.validate()
    }

    fn remember_operational_resolution(
        &mut self,
        case_id: LivingCaseId,
        choice: LivingCaseChoice,
    ) -> Result<(), DeepPressureError> {
        let memory = format!("{} resolved as {:?}", case_id.stable_id(), choice);
        match (case_id, choice) {
            (LivingCaseId::RiptideWellBlowout, LivingCaseChoice::ShutInAndRetrieve) => {
                self.remember(
                    DeepPressurePersonId::CorinWake,
                    2,
                    2,
                    CharacterCondition::Recovering,
                    &memory,
                    None,
                )?;
                self.remember(
                    DeepPressurePersonId::TessBreakwater,
                    2,
                    2,
                    CharacterCondition::Exhausted,
                    &memory,
                    None,
                )?;
            }
            (LivingCaseId::RiptideWellBlowout, LivingCaseChoice::RescueCrewFirst) => {
                self.remember(
                    DeepPressurePersonId::CorinWake,
                    3,
                    2,
                    CharacterCondition::Exposed,
                    &memory,
                    Some("return to Riptide and complete the shut-in"),
                )?;
                self.remember(
                    DeepPressurePersonId::TessBreakwater,
                    1,
                    1,
                    CharacterCondition::Exhausted,
                    &memory,
                    Some("contain the spill reaching Aura Beach"),
                )?;
            }
            (LivingCaseId::AuraBeachStormRescue, LivingCaseChoice::CloseAndShelter) => {
                self.remember(
                    DeepPressurePersonId::SellaWindward,
                    2,
                    2,
                    CharacterCondition::Well,
                    &memory,
                    None,
                )?;
                self.remember(
                    DeepPressurePersonId::TessBreakwater,
                    1,
                    2,
                    CharacterCondition::Recovering,
                    &memory,
                    None,
                )?;
            }
            (LivingCaseId::AuraBeachStormRescue, LivingCaseChoice::GuidedRescue) => {
                self.remember(
                    DeepPressurePersonId::SellaWindward,
                    1,
                    1,
                    CharacterCondition::Exhausted,
                    &memory,
                    None,
                )?;
                self.remember(
                    DeepPressurePersonId::HarrowVale,
                    1,
                    1,
                    CharacterCondition::Exhausted,
                    &memory,
                    Some("carry the additional rescue burden into Basin care"),
                )?;
            }
            (LivingCaseId::AuraBasinInjuredBeing, LivingCaseChoice::TransferToCare) => {
                self.remember(
                    DeepPressurePersonId::HarrowVale,
                    3,
                    2,
                    CharacterCondition::Recovering,
                    &memory,
                    None,
                )?;
                self.remember(
                    DeepPressurePersonId::PelMarrow,
                    2,
                    2,
                    CharacterCondition::Well,
                    &memory,
                    None,
                )?;
            }
            (LivingCaseId::AuraBasinInjuredBeing, LivingCaseChoice::StabilizeInPlace) => {
                self.remember(
                    DeepPressurePersonId::HarrowVale,
                    2,
                    3,
                    CharacterCondition::Exhausted,
                    &memory,
                    Some("complete the care transfer when the route clears"),
                )?;
            }
            (LivingCaseId::MntAuraRoofFall, LivingCaseChoice::ReinforceAndContinue) => {
                self.remember(
                    DeepPressurePersonId::OrenPike,
                    2,
                    3,
                    CharacterCondition::Well,
                    &memory,
                    None,
                )?;
                self.remember(
                    DeepPressurePersonId::BrindleReed,
                    2,
                    2,
                    CharacterCondition::Well,
                    &memory,
                    None,
                )?;
                self.remember(
                    DeepPressurePersonId::PelMarrow,
                    1,
                    3,
                    CharacterCondition::Well,
                    &memory,
                    None,
                )?;
            }
            (LivingCaseId::MntAuraRoofFall, LivingCaseChoice::WithdrawCrew) => {
                self.remember(
                    DeepPressurePersonId::OrenPike,
                    3,
                    2,
                    CharacterCondition::Recovering,
                    &memory,
                    None,
                )?;
                self.remember(
                    DeepPressurePersonId::BrindleReed,
                    0,
                    1,
                    CharacterCondition::Well,
                    &memory,
                    Some("find lawful replacement stone for Field irrigation"),
                )?;
            }
            (LivingCaseId::HighwayToHellGasPocket, LivingCaseChoice::SealAndVent) => {
                self.remember(
                    DeepPressurePersonId::MaelaDownroad,
                    2,
                    3,
                    CharacterCondition::Recovering,
                    &memory,
                    None,
                )?;
                self.remember(
                    DeepPressurePersonId::HarrowVale,
                    1,
                    2,
                    CharacterCondition::Well,
                    &memory,
                    None,
                )?;
            }
            (LivingCaseId::HighwayToHellGasPocket, LivingCaseChoice::EvacuateAndFlood) => {
                self.remember(
                    DeepPressurePersonId::MaelaDownroad,
                    3,
                    2,
                    CharacterCondition::Exhausted,
                    &memory,
                    None,
                )?;
                self.remember(
                    DeepPressurePersonId::HarrowVale,
                    0,
                    1,
                    CharacterCondition::Well,
                    &memory,
                    Some("source lawful replacement iron for Basin Frame recovery"),
                )?;
            }
            (LivingCaseId::AuraFieldDroughtAllocation, LivingCaseChoice::EquitableRation) => {
                self.remember(
                    DeepPressurePersonId::BrindleReed,
                    2,
                    2,
                    CharacterCondition::Well,
                    &memory,
                    None,
                )?;
            }
            (LivingCaseId::AuraFieldDroughtAllocation, LivingCaseChoice::ProtectSeedReserve) => {
                self.remember(
                    DeepPressurePersonId::BrindleReed,
                    1,
                    3,
                    CharacterCondition::Well,
                    &memory,
                    Some("return water to the stressed crop rows when reserve permits"),
                )?;
            }
            (LivingCaseId::CurrentSeaWellCertification, LivingCaseChoice::CertifyReducedRate) => {
                let reliability = if self.contested_certificate() { -1 } else { 2 };
                self.remember(
                    DeepPressurePersonId::IonaDepth,
                    1,
                    reliability,
                    CharacterCondition::Well,
                    &memory,
                    None,
                )?;
                self.remember(
                    DeepPressurePersonId::TessBreakwater,
                    1,
                    reliability,
                    CharacterCondition::Recovering,
                    &memory,
                    None,
                )?;
            }
            (LivingCaseId::CurrentSeaWellCertification, LivingCaseChoice::SuspendForRepair) => {
                self.remember(
                    DeepPressurePersonId::IonaDepth,
                    3,
                    3,
                    CharacterCondition::Well,
                    &memory,
                    None,
                )?;
                self.remember(
                    DeepPressurePersonId::TessBreakwater,
                    1,
                    2,
                    CharacterCondition::Recovering,
                    &memory,
                    Some("cover rescue supply while the depth well is suspended"),
                )?;
            }
            _ => return Err(DeepPressureError::InvalidOperationalResolution { case_id, choice }),
        }
        Ok(())
    }

    fn remember(
        &mut self,
        person: DeepPressurePersonId,
        affinity_delta: i8,
        reliability_delta: i8,
        condition: CharacterCondition,
        outcome: &str,
        promise: Option<&str>,
    ) -> Result<(), DeepPressureError> {
        let memory = self
            .relationships
            .get_mut(&person)
            .ok_or(DeepPressureError::IncompleteCanonicalState)?;
        memory.affinity = (memory.affinity + affinity_delta).clamp(-5, 5);
        memory.reliability = (memory.reliability + reliability_delta).clamp(-5, 5);
        memory.condition = condition;
        memory.remembered_outcomes.push(outcome.into());
        if let Some(promise) = promise {
            memory.unresolved_promises.insert(promise.into());
        }
        Ok(())
    }

    fn remember_settlement(&mut self, outcome: &DeepPressureOutcomeRecord) {
        let bond = outcome
            .recovery_bond
            .as_ref()
            .map(|bond| bond.bond_id.clone());
        for memory in self.relationships.values_mut() {
            memory
                .remembered_outcomes
                .push(outcome.id.stable_id().into());
            memory.constitutional_bond = bond.clone();
            match outcome.choice {
                DeepPressureSettlementChoice::SharedBurdenCompact => {
                    memory.affinity = (memory.affinity + 2).clamp(-5, 5);
                    memory.reliability = (memory.reliability + 2).clamp(-5, 5);
                    memory.unresolved_promises.clear();
                }
                DeepPressureSettlementChoice::CrewAndCoastRestitution => {
                    memory.affinity = (memory.affinity + 3).clamp(-5, 5);
                    memory.reliability = (memory.reliability + 1).clamp(-5, 5);
                    memory
                        .unresolved_promises
                        .retain(|promise| promise.contains("stone") || promise.contains("iron"));
                }
                DeepPressureSettlementChoice::ProductionUnderReview => {
                    memory.reliability = (memory.reliability + 2).clamp(-5, 5);
                    memory
                        .unresolved_promises
                        .insert("attend the next public production review".into());
                }
                DeepPressureSettlementChoice::ProtectedRefusal => {
                    memory.affinity = (memory.affinity + 1).clamp(-5, 5);
                    memory.unresolved_promises.insert(
                        "return only when the affected people choose to reopen settlement".into(),
                    );
                }
            }
        }
    }
}

fn resolve_settlement(
    state: &DeepPressureState,
    choice: DeepPressureSettlementChoice,
    at: CausalPosition,
    rule_set: &RuleSetId,
    constitutional: &mut ConstitutionalRuntime,
    world: &WorldSession,
) -> Result<DeepPressureOutcomeRecord, DeepPressureError> {
    if state.phase() != DeepPressurePhase::BoardwalkSettlement
        || state.supported_settlement != Some(choice)
    {
        return Err(DeepPressureError::SettlementNotReady);
    }
    let catalog = &world.institutional().catalog;
    let key = choice.key();
    let decisions = [
        (HouseFunction::Name, "name-loss-wells-lots-and-claims"),
        (HouseFunction::Prove, "prove-failure-repair-and-custody"),
        (
            HouseFunction::Clear,
            "clear-care-recovery-and-bounded-return",
        ),
        (
            HouseFunction::Recognize,
            "recognize-responsibility-and-refusal",
        ),
    ]
    .into_iter()
    .map(|(function, act)| {
        scenario_house_decision(
            catalog,
            &format!("deep-pressure.{key}.{act}"),
            function,
            at.get(),
        )
        .map_err(|error| DeepPressureError::Constitutional(error.to_string()))
    })
    .collect::<Result<Vec<_>, _>>()?;

    let recovery_bond = if choice.forms_bond() {
        Some(form_recovery_bond(
            choice,
            at,
            rule_set,
            constitutional,
            catalog,
        )?)
    } else {
        None
    };
    let contested = state.contested_certificate();
    let aftermath = build_aftermath(state, choice, contested);
    let refusal_and_limits = match choice {
        DeepPressureSettlementChoice::SharedBurdenCompact => vec![
            "no region's contribution creates ownership of another region or person".into(),
            "every participant may challenge hidden scope, unsafe work, or a broken custody chain"
                .into(),
            "the compact ends at its finite term unless lawfully renewed".into(),
        ],
        DeepPressureSettlementChoice::CrewAndCoastRestitution => vec![
            "care and ecological repair cannot be converted into silence or waived testimony"
                .into(),
            "restitution acknowledges harm without buying a person's consent".into(),
            "production remains separately reviewable".into(),
        ],
        DeepPressureSettlementChoice::ProductionUnderReview => vec![
            "certification remains bounded, reviewable, and distinct from responsibility".into(),
            "any failed pressure, provenance, or living-blood exclusion record stops transfer"
                .into(),
            "affected people retain challenge and exit".into(),
        ],
        DeepPressureSettlementChoice::ProtectedRefusal => vec![
            "refusal creates no debt, retaliation, adverse status, or compelled affiliation".into(),
            "existing care, rescue, and custody duties survive the absence of a compact".into(),
            "Flynt recognition does not manufacture consensus".into(),
        ],
    };
    Ok(DeepPressureOutcomeRecord {
        id: DeepPressureOutcomeId::for_choice(choice),
        choice,
        committed_by: DEEP_PRESSURE_ASSEMBLY_ID.into(),
        player_support_is_nonbinding: true,
        four_house_acts: decisions
            .iter()
            .map(HouseActRecord::from_decision)
            .collect(),
        recovery_bond,
        aftermath,
        refusal_and_limits,
    })
}

fn form_recovery_bond(
    choice: DeepPressureSettlementChoice,
    at: CausalPosition,
    rule_set: &RuleSetId,
    runtime: &mut ConstitutionalRuntime,
    catalog: &crate::institution::InstitutionCatalog,
) -> Result<RecoveryBondRecord, DeepPressureError> {
    if at.get() == 0 {
        return Err(DeepPressureError::CausalPositionRequired);
    }
    let key = choice.key();
    let bond = stable(&format!("bond.deep-pressure.{key}.v1"), BondId::new)?;
    let wave = stable(&format!("wave.deep-pressure.{key}.choice"), WaveId::new)?;
    runtime
        .record_wave(WaveRecord {
            id: wave.clone(),
            origin: evidence("deep-pressure-affected-assembly-choice")?,
            causal_position: CausalPosition::new(at.get() - 1),
        })
        .map_err(|error| DeepPressureError::Constitutional(error.to_string()))?;

    let institution = stonebend_constitution_id();
    let term_end = CausalPosition::new(
        at.get()
            .checked_add(240)
            .ok_or(DeepPressureError::CausalOverflow)?,
    );
    let participants = DeepPressurePersonId::ALL
        .into_iter()
        .map(|person| {
            Ok(BondParticipant {
                id: stable(
                    &format!("participant.deep-pressure.{}", person.stable_id()),
                    ParticipantId::new,
                )?,
                kind: ParticipantKind::Npc,
                roles: vec![stable(
                    &format!("role.deep-pressure.{}", person.stable_id()),
                    RoleId::new,
                )?],
            })
        })
        .collect::<Result<Vec<_>, DeepPressureError>>()?;
    let participant_ids = participants
        .iter()
        .map(|participant| participant.id.as_str().to_owned())
        .collect();
    let current_unit = stable("unit.current", UnitId::new)?;
    let formation = BondFormation {
        id: bond.clone(),
        initiating_wave: wave,
        governing_house: House::Stonebend,
        governing_institution: institution.clone(),
        jurisdiction: InstitutionalJurisdictionSnapshot::from_catalog(
            catalog,
            &institution,
            at,
            vec![evidence("deep-pressure-cross-region-jurisdiction")?],
        )
        .map_err(|error| DeepPressureError::Constitutional(error.to_string()))?,
        parent_bonds: vec![],
        inheritance_evidence: vec![],
        participants,
        obligations: settlement_obligations(choice)
            .iter()
            .map(|obligation| stable(obligation, ObligationId::new))
            .collect::<Result<_, _>>()?,
        permissions: [
            "permission.deep-pressure.challenge",
            "permission.deep-pressure.refuse-unsafe-work",
            "permission.deep-pressure.inspect-custody",
            "permission.deep-pressure.exit-at-term",
        ]
        .into_iter()
        .map(|permission| stable(permission, PermissionId::new))
        .collect::<Result<_, _>>()?,
        term: BondTerm::Finite { end: term_end },
        current_unit: current_unit.clone(),
        aura_unit: stable("unit.aura", UnitId::new)?,
        starting_current: vec![InitialCurrent {
            owner: stable(
                "participant.deep-pressure.person.corin-wake",
                ParticipantId::new,
            )?,
            custodian: stable(
                "participant.deep-pressure.person.corin-wake",
                ParticipantId::new,
            )?,
            quantity: SignedQuantity::new(Sign::Positive, 1, current_unit)
                .map_err(|error| DeepPressureError::Constitutional(error.to_string()))?,
            evidence: vec![evidence("corin-wake-retains-current-and-agency")?],
        }],
        initial_aura: vec![],
        evidence: vec![
            evidence("all-required-affected-statements-recorded")?,
            evidence("all-seven-operational-resolutions-linked")?,
            evidence("finite-term-and-exit-disclosed")?,
        ],
        stonebend_naming: scenario_house_decision(
            catalog,
            &format!("deep-pressure.{key}.bond-name"),
            HouseFunction::Name,
            at.get(),
        )
        .map_err(|error| DeepPressureError::Constitutional(error.to_string()))?,
    };
    let formed_event = stable(
        &format!("event.deep-pressure.{key}.form"),
        ConstitutionalEventId::new,
    )?;
    runtime
        .append(
            bond.clone(),
            EventMetadata {
                id: formed_event.clone(),
                causal_position: at,
                rule_set: rule_set.clone(),
            },
            BondEvent::Formed(formation),
        )
        .map_err(|error| DeepPressureError::Constitutional(error.to_string()))?;
    let validated_event = stable(
        &format!("event.deep-pressure.{key}.validate"),
        ConstitutionalEventId::new,
    )?;
    runtime
        .append(
            bond.clone(),
            EventMetadata {
                id: validated_event.clone(),
                causal_position: at,
                rule_set: rule_set.clone(),
            },
            BondEvent::Validated(BondValidation {
                sandmanor_proof: scenario_house_decision(
                    catalog,
                    &format!("deep-pressure.{key}.bond-proof"),
                    HouseFunction::Prove,
                    at.get(),
                )
                .map_err(|error| DeepPressureError::Constitutional(error.to_string()))?,
                evidence: vec![evidence("deep-pressure-bond-terms-proven")?],
            }),
        )
        .map_err(|error| DeepPressureError::Constitutional(error.to_string()))?;
    let activated_event = stable(
        &format!("event.deep-pressure.{key}.activate"),
        ConstitutionalEventId::new,
    )?;
    runtime
        .append(
            bond.clone(),
            EventMetadata {
                id: activated_event.clone(),
                causal_position: at,
                rule_set: rule_set.clone(),
            },
            BondEvent::Activated(BondActivation {
                evidence: vec![evidence("deep-pressure-assembly-accepted-bounded-terms")?],
            }),
        )
        .map_err(|error| DeepPressureError::Constitutional(error.to_string()))?;
    Ok(RecoveryBondRecord {
        bond_id: bond.as_str().into(),
        formed_event_id: formed_event.as_str().into(),
        validated_event_id: validated_event.as_str().into(),
        activated_event_id: activated_event.as_str().into(),
        term_end: term_end.get(),
        participants: participant_ids,
    })
}

fn settlement_obligations(choice: DeepPressureSettlementChoice) -> &'static [&'static str] {
    match choice {
        DeepPressureSettlementChoice::SharedBurdenCompact => &[
            "obligation.deep-pressure.share-repair-burden",
            "obligation.deep-pressure.publish-custody",
            "obligation.deep-pressure.complete-crew-care",
            "obligation.deep-pressure.restore-coast-and-basin",
            "obligation.deep-pressure.protect-field-reserve",
        ],
        DeepPressureSettlementChoice::CrewAndCoastRestitution => &[
            "obligation.deep-pressure.pay-crew-restitution",
            "obligation.deep-pressure.complete-care",
            "obligation.deep-pressure.restore-fish-and-dunes",
            "obligation.deep-pressure.publish-harm-record",
        ],
        DeepPressureSettlementChoice::ProductionUnderReview => &[
            "obligation.deep-pressure.limit-production-rate",
            "obligation.deep-pressure.publish-pressure-tests",
            "obligation.deep-pressure.preserve-living-blood-exclusion",
            "obligation.deep-pressure.fund-repair-from-production",
        ],
        DeepPressureSettlementChoice::ProtectedRefusal => &[],
    }
}

fn build_aftermath(
    state: &DeepPressureState,
    choice: DeepPressureSettlementChoice,
    contested: bool,
) -> DeepPressureAftermath {
    let mut crew = 45_i16;
    let mut coast = 45_i16;
    let mut field = 45_i16;
    let mut basin = 45_i16;
    match state
        .integrated_resolutions
        .get(&LivingCaseId::RiptideWellBlowout)
    {
        Some(LivingCaseChoice::ShutInAndRetrieve) => {
            crew += 15;
            coast += 22;
            basin += 8;
        }
        Some(LivingCaseChoice::RescueCrewFirst) => {
            crew += 28;
            coast -= 14;
            basin -= 8;
        }
        _ => {}
    }
    match state
        .integrated_resolutions
        .get(&LivingCaseId::AuraBeachStormRescue)
    {
        Some(LivingCaseChoice::CloseAndShelter) => coast += 13,
        Some(LivingCaseChoice::GuidedRescue) => {
            crew += 7;
            basin -= 4;
        }
        _ => {}
    }
    match state
        .integrated_resolutions
        .get(&LivingCaseId::AuraBasinInjuredBeing)
    {
        Some(LivingCaseChoice::TransferToCare) => {
            crew += 10;
            basin += 8;
        }
        Some(LivingCaseChoice::StabilizeInPlace) => {
            crew += 6;
            basin -= 5;
        }
        _ => {}
    }
    match state
        .integrated_resolutions
        .get(&LivingCaseId::MntAuraRoofFall)
    {
        Some(LivingCaseChoice::ReinforceAndContinue) => {
            field += 20;
            coast += 6;
        }
        Some(LivingCaseChoice::WithdrawCrew) => crew += 7,
        _ => {}
    }
    match state
        .integrated_resolutions
        .get(&LivingCaseId::HighwayToHellGasPocket)
    {
        Some(LivingCaseChoice::SealAndVent) => basin += 19,
        Some(LivingCaseChoice::EvacuateAndFlood) => crew += 8,
        _ => {}
    }
    match state
        .integrated_resolutions
        .get(&LivingCaseId::AuraFieldDroughtAllocation)
    {
        Some(LivingCaseChoice::EquitableRation) => field += 12,
        Some(LivingCaseChoice::ProtectSeedReserve) => field += 9,
        _ => {}
    }
    match choice {
        DeepPressureSettlementChoice::SharedBurdenCompact => {
            crew += 12;
            coast += 12;
            field += 12;
            basin += 12;
        }
        DeepPressureSettlementChoice::CrewAndCoastRestitution => {
            crew += 20;
            coast += 20;
            field -= 3;
        }
        DeepPressureSettlementChoice::ProductionUnderReview => {
            coast += 8;
            field += 8;
            basin += 8;
        }
        DeepPressureSettlementChoice::ProtectedRefusal => {
            crew += 4;
            coast += 4;
        }
    }
    let production_posture = match (
        contested,
        state
            .integrated_resolutions
            .get(&LivingCaseId::CurrentSeaWellCertification),
        choice,
    ) {
        (true, _, _) => "contested certificate held for higher review",
        (_, Some(LivingCaseChoice::SuspendForRepair), _) => "depth production suspended for repair",
        (_, _, DeepPressureSettlementChoice::ProductionUnderReview) => {
            "reduced production under public recurring review"
        }
        (_, Some(LivingCaseChoice::CertifyReducedRate), _) => {
            "reduced certified production remains separately reviewable"
        }
        _ => "no production posture inferred",
    }
    .into();
    let mut unresolved_obligations = state
        .relationships
        .values()
        .flat_map(|memory| memory.unresolved_promises.iter().cloned())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    if choice == DeepPressureSettlementChoice::SharedBurdenCompact {
        unresolved_obligations.clear();
    }
    let visible_changes = match choice {
        DeepPressureSettlementChoice::SharedBurdenCompact => vec![
            "a four-House recovery ledger is posted on the Boardwalk".into(),
            "crew, coast, Field, Basin, mine, and rig markers share one finite-term seal".into(),
        ],
        DeepPressureSettlementChoice::CrewAndCoastRestitution => vec![
            "care and coast-repair claims occupy the Boardwalk's public return lane".into(),
            "production banners remain behind the restitution ledger".into(),
        ],
        DeepPressureSettlementChoice::ProductionUnderReview => vec![
            "a pressure clock and custody board remain publicly visible".into(),
            "every production shift shows its next review point".into(),
        ],
        DeepPressureSettlementChoice::ProtectedRefusal => vec![
            "the settlement table remains empty without erasing any testimony".into(),
            "a protected-refusal seal leaves the public return lane open".into(),
        ],
    };
    DeepPressureAftermath {
        crew_care: score(crew),
        coast_recovery: score(coast),
        field_security: score(field),
        basin_repair: score(basin),
        production_posture,
        contested_certificate: contested,
        unresolved_obligations,
        visible_changes,
    }
}

fn score(value: i16) -> u8 {
    u8::try_from(value.clamp(0, 100)).expect("score is clamped to u8")
}

fn operational_record(evidence: LivingEvidence, clock: LivingClock) -> EvidenceJournalRecord {
    let (source, claim, uncertainty) = match evidence {
        LivingEvidence::FieldWaterGauge => (
            "Aura Field public water gauge",
            "the irrigation reserve is below simultaneous full-allocation demand",
            "one reading cannot predict the next weather shift",
        ),
        LivingEvidence::FieldSoilProbe => (
            "Aura Field soil probe",
            "crop rows are under uneven moisture stress",
            "root recovery after allocation remains uncertain",
        ),
        LivingEvidence::FieldGranaryLedger => (
            "Aura Field granary ledger",
            "food and seed reserves are distinct obligations",
            "future yield is projected rather than guaranteed",
        ),
        LivingEvidence::BeachTideRecord => (
            "Aura Beach tide record",
            "the public approach is exposed to a high tide",
            "the next local surge remains uncertain",
        ),
        LivingEvidence::BeachWeatherRecord => (
            "Aura Beach weather station",
            "pressure and visibility cross the authored warning threshold",
            "Aura reveals conditions but does not choose a rescue order",
        ),
        LivingEvidence::BeachRescueManifest => (
            "Aura Beach rescue manifest",
            "people and craft remain exposed",
            "the manifest may change as guided rescue proceeds",
        ),
        LivingEvidence::BasinVitalSigns => (
            "Aura Basin triage vital record",
            "the claimed subject is alive and needs stabilization",
            "the final recovery course remains clinical uncertainty",
        ),
        LivingEvidence::BasinContinuityRecord => (
            "Aura Basin continuity record",
            "the injured subject retains Being continuity",
            "continuity does not itself settle every later Name question",
        ),
        LivingEvidence::BasinSalvageClaim => (
            "Aura Basin attempted salvage claim",
            "a material claimant tried to classify a living Being as salvage",
            "the claim is evidence of conduct, not lawful ownership",
        ),
        LivingEvidence::MntAuraSurvey => (
            "Mt. Aura survey",
            "the high face crossed its roof-support boundary",
            "unseen fracture propagation remains uncertain",
        ),
        LivingEvidence::MntAuraSupportInspection => (
            "Mt. Aura support inspection",
            "reinforcement is required before reduced-rate work",
            "inspection cannot grant mineral Title",
        ),
        LivingEvidence::MntAuraCrewRoll => (
            "Mt. Aura crew roll",
            "named workers were inside the affected district",
            "the roll records presence, not consent to new danger",
        ),
        LivingEvidence::HighwayGasReading => (
            "Highway to Hell gas reading",
            "the descending gallery crossed its gas stop threshold",
            "the pocket extent remains bounded by later readings",
        ),
        LivingEvidence::HighwayVentilationLog => (
            "Highway to Hell ventilation log",
            "the current circuit cannot safely clear the working face",
            "recovery time remains uncertain",
        ),
        LivingEvidence::HighwayEscapeCheck => (
            "Highway to Hell escape check",
            "one protected egress remains available",
            "availability can change if the district is not stopped",
        ),
        LivingEvidence::RiptideWellPressure => (
            "Riptide pressure recorder",
            "well control was lost during Current recovery",
            "the subsurface pressure envelope remains partly unknown",
        ),
        LivingEvidence::RiptideSpillExtent => (
            "Riptide spill survey",
            "released fluid is moving toward Aura Beach and the Basin watershed",
            "weather makes the final extent uncertain",
        ),
        LivingEvidence::RiptideCrewManifest => (
            "Riptide crew manifest",
            "Corin Wake's crew remained exposed during the blowout",
            "a manifest records assignment, never consent to abandonment",
        ),
        LivingEvidence::CurrentSeaPressureTest => (
            "Current Sea pressure test",
            "the depth well can hold a reduced-rate envelope",
            "one test does not erase the Riptide failure record",
        ),
        LivingEvidence::CurrentSeaSampleAssay => (
            "Current Sea sample assay",
            "the sample is geological Current-bearing brine",
            "contrary identity or living-tissue evidence requires quarantine",
        ),
        LivingEvidence::CurrentSeaCustodyChain => (
            "Current Sea custody chain",
            "the tested lot has a named origin, custodian, and destination",
            "custody is not ownership and certification is not restitution",
        ),
    };
    EvidenceJournalRecord {
        evidence_id: DeepPressureEvidenceId::Operational(evidence),
        stable_id: stable_evidence_id(DeepPressureEvidenceId::Operational(evidence)),
        source: source.into(),
        kind: if matches!(evidence, LivingEvidence::CurrentSeaCustodyChain) {
            JournalEvidenceKind::CustodyRecord
        } else {
            JournalEvidenceKind::OperationalRecord
        },
        speech_classification: None,
        claim: claim.into(),
        uncertainty: uncertainty.into(),
        constitutional_effect:
            "addressable evidence only; it neither chooses nor substitutes for a House act".into(),
        observed_day: clock.day,
        observed_shift: clock.shift,
    }
}

#[must_use]
pub fn deep_pressure_statement_record(
    statement: DeepPressureStatementId,
    clock: LivingClock,
) -> EvidenceJournalRecord {
    let (source, kind, classification, claim, uncertainty) = match statement {
        DeepPressureStatementId::Person(DeepPressurePersonId::BrindleReed) => (
            "Brindle Reed",
            JournalEvidenceKind::CapableSubjectStatement,
            SpeechClassification::PrivateBelief,
            "If the repair stone goes only to the shore, the Field will lose a planting before anyone calls it a casualty.",
            "Brindle knows the gates and crops but cannot predict every weather shift.",
        ),
        DeepPressureStatementId::Person(DeepPressurePersonId::SellaWindward) => (
            "Sella Windward",
            JournalEvidenceKind::CapableSubjectStatement,
            SpeechClassification::PublicRecord,
            "My warning log placed the pressure drop before the Riptide sheen reached the public approach.",
            "The log orders observations; it does not alone prove the spill's source.",
        ),
        DeepPressureStatementId::Person(DeepPressurePersonId::HarrowVale) => (
            "Harrow Vale",
            JournalEvidenceKind::CapableSubjectStatement,
            SpeechClassification::ConstitutionalFact,
            "A living Being enters care, never the salvage ledger, even when the Frame is badly damaged.",
            "Care status does not settle a later Stonebend Name conflict.",
        ),
        DeepPressureStatementId::Person(DeepPressurePersonId::OrenPike) => (
            "Oren Pike",
            JournalEvidenceKind::CapableSubjectStatement,
            SpeechClassification::PublicRecord,
            "The roof can be reinforced for a measured stone lot, or the crew can leave; blasting is outside my stop order.",
            "The survey cannot guarantee a hidden seam.",
        ),
        DeepPressureStatementId::Person(DeepPressurePersonId::MaelaDownroad) => (
            "Maela Downroad",
            JournalEvidenceKind::CapableSubjectStatement,
            SpeechClassification::PublicRecord,
            "Deep iron exists beyond the gas red line, but no repair need makes the alarm disappear.",
            "Ventilation recovery time remains uncertain.",
        ),
        DeepPressureStatementId::Person(DeepPressurePersonId::BramBurden) => (
            "Bram Burden",
            JournalEvidenceKind::LocalClaim,
            SpeechClassification::LocalTradition,
            "Stairway folk say every lifted burden owes a safe descent.",
            "Local tradition guides conduct but is not a House decision.",
        ),
        DeepPressureStatementId::Person(DeepPressurePersonId::CorinWake) => (
            "Corin Wake",
            JournalEvidenceKind::CapableSubjectStatement,
            SpeechClassification::PublicRecord,
            "I was on the Riptide crew. Rescue and shut-in were both real duties; choosing one first did not erase the other.",
            "Corin witnessed the rig, not the full offshore plume.",
        ),
        DeepPressureStatementId::Person(DeepPressurePersonId::IonaDepth) => (
            "Iona Depth",
            JournalEvidenceKind::CapableSubjectStatement,
            SpeechClassification::ConstitutionalFact,
            "A depth certificate controls a tested transfer. It does not certify innocence, ownership, or a living Being's blood.",
            "The technician may test and stop; the recorded certifier decides certification.",
        ),
        DeepPressureStatementId::Person(DeepPressurePersonId::PelMarrow) => (
            "Pel Marrow",
            JournalEvidenceKind::CustodyRecord,
            SpeechClassification::PublicRecord,
            "Stone, iron, and brine each have a claimant, custodian, destination, and refusal point; none becomes ownerless in transit.",
            "A complete chain proves custody history, not the truth of every upstream claim.",
        ),
        DeepPressureStatementId::Person(DeepPressurePersonId::TessBreakwater) => (
            "Tess Breakwater",
            JournalEvidenceKind::LocalClaim,
            SpeechClassification::Rumor,
            "Dock talk says the Current Sea well was ready before Riptide failed.",
            "Tess identifies this as unverified dock rumor and refuses to treat it as a production record.",
        ),
        DeepPressureStatementId::BoardwalkPimpPitch => (
            "Boardwalk Pimp",
            JournalEvidenceKind::LocalClaim,
            SpeechClassification::DeliberateDeception,
            "Once the barrel carries a certificate, nobody can owe restitution for how it reached shore.",
            "The claim knowingly confuses certification with responsibility and has no constitutional effect.",
        ),
        DeepPressureStatementId::BoardwalkGimpPitch => (
            "Boardwalk Gimp",
            JournalEvidenceKind::LocalClaim,
            SpeechClassification::LocalTradition,
            "A shared burden holds only when every goon can name the edge and leave at the term.",
            "Boardwalk tradition does not itself form or validate a Bond.",
        ),
    };
    EvidenceJournalRecord {
        evidence_id: DeepPressureEvidenceId::Statement(statement),
        stable_id: statement.stable_id(),
        source: source.into(),
        kind,
        speech_classification: Some(classification),
        claim: claim.into(),
        uncertainty: uncertainty.into(),
        constitutional_effect: "classified speech is evidence; the speaker may be limited, mistaken, or deceptive without rewriting Hollow Grove".into(),
        observed_day: clock.day,
        observed_shift: clock.shift,
    }
}

fn stable_evidence_id(id: DeepPressureEvidenceId) -> String {
    match id {
        DeepPressureEvidenceId::Operational(evidence) => {
            format!("evidence.deep-pressure.operational.{:?}", evidence).to_lowercase()
        }
        DeepPressureEvidenceId::Statement(statement) => statement.stable_id(),
    }
}

#[must_use]
pub fn statement_for_interaction(
    interaction: super::InteractionId,
) -> Option<DeepPressureStatementId> {
    match interaction {
        super::InteractionId::DeepPressurePerson(person) => {
            Some(DeepPressureStatementId::Person(person))
        }
        super::InteractionId::BoardwalkPimp => Some(DeepPressureStatementId::BoardwalkPimpPitch),
        super::InteractionId::BoardwalkGimp => Some(DeepPressureStatementId::BoardwalkGimpPitch),
        _ => None,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeepPressurePersonPresence {
    pub person_id: DeepPressurePersonId,
    pub stable_id: String,
    pub display_name: String,
    pub role: String,
    pub authority_limit: String,
    pub initials: String,
    pub position: TilePosition,
}

#[must_use]
pub fn scheduled_people_on_map(
    living: &LivingWorldState,
    map: WorldMapId,
) -> Vec<DeepPressurePersonPresence> {
    let location = match map {
        WorldMapId::AuraFieldWorkingLand => Some(ScheduledLocation::Surface(
            crate::world::interior_surface::InteriorSurfaceId::AuraField,
        )),
        WorldMapId::AuraBeachCoastalCommons => Some(ScheduledLocation::Surface(
            crate::world::interior_surface::InteriorSurfaceId::AuraBeach,
        )),
        WorldMapId::AuraBasinCollisionGrounds => Some(ScheduledLocation::Surface(
            crate::world::interior_surface::InteriorSurfaceId::AuraBasin,
        )),
        WorldMapId::ExtractionSite(site) => Some(ScheduledLocation::Extraction(site)),
        WorldMapId::AuraRidgeGroveApproach => {
            Some(ScheduledLocation::Route(ConstitutionalRouteId::AuraRidge))
        }
        WorldMapId::BoardwalkReturnVestibule => {
            Some(ScheduledLocation::Route(ConstitutionalRouteId::Boardwalk))
        }
        WorldMapId::CurrentSeaDeepCertificationLanding => {
            Some(ScheduledLocation::Route(ConstitutionalRouteId::CurrentSea))
        }
        WorldMapId::RoutePassage(route) => Some(ScheduledLocation::Route(route)),
    };
    let Some(location) = location else {
        return Vec::new();
    };
    let definition = map_definition(map);
    let mut candidates = Vec::new();
    for y in 1..super::MAP_HEIGHT - 1 {
        for x in 1..super::MAP_WIDTH - 1 {
            let tile = definition.rows[usize::from(y)].as_bytes()[usize::from(x)];
            if matches!(tile, b'.' | b'=') && !(definition.spawn.x == x && definition.spawn.y == y)
            {
                let distance = x.abs_diff(super::MAP_WIDTH / 2) + y.abs_diff(super::MAP_HEIGHT / 2);
                candidates.push((distance, y, x));
            }
        }
    }
    candidates.sort_unstable();
    living
        .people_at(location)
        .into_iter()
        .filter_map(|person| {
            DeepPressurePersonId::from_stable_id(&person.person_id)
                .map(|person_id| (person_id, person))
        })
        .zip(candidates)
        .map(
            |((person_id, person), (_, y, x))| DeepPressurePersonPresence {
                person_id,
                stable_id: person.person_id.clone(),
                display_name: person.display_name.clone(),
                role: person.role.clone(),
                authority_limit: person.authority_limit.clone(),
                initials: person_id.initials().into(),
                position: TilePosition {
                    x,
                    y,
                    facing: CardinalDirection::South,
                },
            },
        )
        .collect()
}

#[must_use]
pub fn person_in_front(
    living: &LivingWorldState,
    map: WorldMapId,
    hueman: TilePosition,
) -> Option<DeepPressurePersonId> {
    let (dx, dy) = hueman.facing.offset();
    let x = i32::from(hueman.x) + dx;
    let y = i32::from(hueman.y) + dy;
    scheduled_people_on_map(living, map)
        .into_iter()
        .find(|person| i32::from(person.position.x) == x && i32::from(person.position.y) == y)
        .map(|person| person.person_id)
}

#[must_use]
pub fn person_occupies(living: &LivingWorldState, map: WorldMapId, position: TilePosition) -> bool {
    scheduled_people_on_map(living, map)
        .iter()
        .any(|person| person.position.x == position.x && person.position.y == position.y)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeepPressureFunctionalLoreDefinition {
    pub stable_identity: &'static str,
    pub authority_class: &'static str,
    pub location_and_jurisdiction: &'static str,
    pub involved: &'static [&'static str],
    pub dominant_verb: &'static str,
    pub trigger: &'static str,
    pub evidence_and_uncertainty: &'static str,
    pub player_visible_choice: &'static str,
    pub lawful_state_change: &'static str,
    pub persistence_and_replay: &'static str,
    pub presentation_projection: &'static str,
    pub failure_or_refusal: &'static str,
}

#[must_use]
pub const fn deep_pressure_functional_lore() -> DeepPressureFunctionalLoreDefinition {
    DeepPressureFunctionalLoreDefinition {
        stable_identity: DEEP_PRESSURE_CASE_ID,
        authority_class: "linked four-House public recovery settlement; local duty acts remain local",
        location_and_jurisdiction: "Riptide, Aura Beach, Aura Basin, Mt. Aura, Highway to Hell, Aura Field, Current Sea, and final public return on the Boardwalk",
        involved: &[
            "Corin Wake's Riptide crew",
            "shore and Basin recovery workers",
            "Field households",
            "mine and rig crews",
            "material custody keepers",
            "four House authorities",
            "Boardwalk witnesses",
        ],
        dominant_verb: "name, prove, clear, recognize, remember, repair, restitute, review, or refuse",
        trigger: "a Riptide blowout sends one pressure failure through bodies, coast, Basin, repair supply, Field need, certification, and public responsibility",
        evidence_and_uncertainty: "twenty-one operational witnesses plus classified affected-person speech; fact, record, tradition, rumor, belief, and deception never share authority silently",
        player_visible_choice: "support shared burden, crew-and-coast restitution, production under review, or protected refusal",
        lawful_state_change: "local outcomes, character condition, trust, promises, custody meaning, four House acts, an optional finite Bond, and regional aftermath persist",
        persistence_and_replay: "every journal entry, linked duty result, support, House act, Bond event, ending, relationship memory, and aftermath score replays from ordered gameplay events",
        presentation_projection: "scheduled people move by shift, speak in classified pages, appear on maps, and retain visible memories while the journal and aftermath remain client-readable",
        failure_or_refusal: "missing records fail closed; compromised certification bars the production accord; protected refusal creates no debt; no ending commodifies living blood or substitutes one House for another",
    }
}

impl DeepPressureFunctionalLoreDefinition {
    pub fn validate(&self) -> Result<(), DeepPressureError> {
        if self.stable_identity != DEEP_PRESSURE_CASE_ID
            || self.involved.is_empty()
            || [
                self.authority_class,
                self.location_and_jurisdiction,
                self.dominant_verb,
                self.trigger,
                self.evidence_and_uncertainty,
                self.player_visible_choice,
                self.lawful_state_change,
                self.persistence_and_replay,
                self.presentation_projection,
                self.failure_or_refusal,
            ]
            .into_iter()
            .any(str::is_empty)
        {
            return Err(DeepPressureError::InvalidFunctionalLore);
        }
        Ok(())
    }
}

fn living_choice_key(choice: LivingCaseChoice) -> &'static str {
    match choice {
        LivingCaseChoice::EquitableRation => "equitable-ration",
        LivingCaseChoice::ProtectSeedReserve => "protect-seed-reserve",
        LivingCaseChoice::MaximizeImmediateYield => "maximize-immediate-yield",
        LivingCaseChoice::CloseAndShelter => "close-and-shelter",
        LivingCaseChoice::GuidedRescue => "guided-rescue",
        LivingCaseChoice::KeepShoreOpen => "keep-shore-open",
        LivingCaseChoice::TransferToCare => "transfer-to-care",
        LivingCaseChoice::StabilizeInPlace => "stabilize-in-place",
        LivingCaseChoice::SalvageTheSubject => "salvage-the-subject",
        LivingCaseChoice::ReinforceAndContinue => "reinforce-and-continue",
        LivingCaseChoice::WithdrawCrew => "withdraw-crew",
        LivingCaseChoice::BlastThroughFall => "blast-through-fall",
        LivingCaseChoice::SealAndVent => "seal-and-vent",
        LivingCaseChoice::EvacuateAndFlood => "evacuate-and-flood",
        LivingCaseChoice::ContinueCutting => "continue-cutting",
        LivingCaseChoice::ShutInAndRetrieve => "shut-in-and-retrieve",
        LivingCaseChoice::RescueCrewFirst => "rescue-crew-first",
        LivingCaseChoice::ContinueFlow => "continue-flow",
        LivingCaseChoice::CertifyReducedRate => "certify-reduced-rate",
        LivingCaseChoice::SuspendForRepair => "suspend-for-repair",
        LivingCaseChoice::BypassCertification => "bypass-certification",
    }
}

fn evidence(key: &str) -> Result<EvidenceRef, DeepPressureError> {
    EvidenceRef::new("gameplay.deep-pressure", key)
        .map_err(|error| DeepPressureError::Constitutional(error.to_string()))
}

fn stable<T>(
    value: &str,
    constructor: impl FnOnce(String) -> Result<T, crate::constitutional::ConstitutionalIdError>,
) -> Result<T, DeepPressureError> {
    constructor(value.into()).map_err(|error| DeepPressureError::Constitutional(error.to_string()))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeepPressureError {
    IncompleteCanonicalState,
    InvalidFunctionalLore,
    DuplicateEvidence,
    DuplicateOrInvalidEvidence,
    InvalidSpeechClassification,
    InvalidRelationshipMemory,
    ResolutionAlreadyIntegrated(LivingCaseId),
    InvalidOperationalResolution {
        case_id: LivingCaseId,
        choice: LivingCaseChoice,
    },
    SettlementNotReady,
    SettlementSupportRequired,
    SettlementSupportAlreadyRecorded,
    SettlementAlreadyCommitted,
    CompromisedEvidenceBarsProductionAccord,
    OutcomeDivergence,
    CausalPositionRequired,
    CausalOverflow,
    RevisionOverflow,
    Constitutional(String),
}

impl fmt::Display for DeepPressureError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "Deep Pressure campaign rejected action: {self:?}"
        )
    }
}

impl std::error::Error for DeepPressureError {}
