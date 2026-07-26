//! Sandmanor's guardian, geography, reciprocal-improvement, and sovereign
//! succession law.
//!
//! This is a constitutional world model. It reuses stable person identity,
//! Sandmanor proof/Recipe IDs, the existing lineage contract, and Glaüshouse's
//! maintained-Synthesis lifecycle. It does not execute transformation, grant
//! another House's authority, or alter runtime movement or combat.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::hollow_grove_contract::House;
use crate::institution::{IdentityId, InstitutionId, SiteId};
use crate::lineage_contract::{SandmanorForm, SandmanorLineage, validate_sandmanor_transition};
use crate::world::glaushouse::{SynthesisContinuance, SynthesisLifecycle, SynthesisLifecycleState};

use super::{ContestId, EvidenceId, RecipeId};

macro_rules! milestone_id {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, SandmanorMilestoneError> {
                let value = value.into();
                if value.is_empty()
                    || !value.bytes().all(|byte| {
                        byte.is_ascii_lowercase()
                            || byte.is_ascii_digit()
                            || matches!(byte, b'.' | b'-')
                    })
                {
                    return Err(SandmanorMilestoneError::InvalidStableId(value));
                }
                Ok(Self(value))
            }

            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(formatter)
            }
        }
    };
}

milestone_id!(GuardianQualificationId);
milestone_id!(GuardianAuthorizationId);
milestone_id!(GuardianSynthesisId);
milestone_id!(GuardianInvestitureId);
milestone_id!(GuardianEventId);
milestone_id!(TrialId);
milestone_id!(CrowdJudgmentId);
milestone_id!(SuccessionId);
milestone_id!(CoastalTransferId);
milestone_id!(MaritimeTrainingId);

#[must_use]
pub fn minoan_county_courthouse_id() -> InstitutionId {
    InstitutionId::new("institution.sandmanor.minoan-county-courthouse")
        .expect("canonical Minoan County Courthouse institution ID")
}

#[must_use]
pub fn minoan_county_courthouse_site_id() -> SiteId {
    SiteId::new("site.sandmanor.minoan-county-courthouse")
        .expect("canonical Minoan County Courthouse site ID")
}

/// The equal Sandmanor peoples. The existing `SandmanorLineage` remains the
/// transformation-lineage authority; this type names the civic people.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SandmanorPeople {
    Minorian,
    Minoan,
}

impl SandmanorPeople {
    #[must_use]
    pub const fn base_form(self) -> SandmanorForm {
        match self {
            Self::Minorian => SandmanorForm::Gnome,
            Self::Minoan => SandmanorForm::Elf,
        }
    }

    #[must_use]
    pub const fn first_guardian_form(self) -> SandmanorForm {
        match self {
            Self::Minorian => SandmanorForm::Minotaur,
            Self::Minoan => SandmanorForm::Centaur,
        }
    }

    #[must_use]
    pub const fn senior_guardian_form(self) -> SandmanorForm {
        match self {
            Self::Minorian => SandmanorForm::Hecaton,
            Self::Minoan => SandmanorForm::Pegasus,
        }
    }

    #[must_use]
    pub const fn lineage(self) -> SandmanorLineage {
        match self {
            Self::Minorian => SandmanorLineage::Minorian,
            Self::Minoan => SandmanorLineage::Minoan,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CultivationDomain {
    AuraFields,
    ContentFarm,
}

pub const AURA_FARM_HALVES: [CultivationDomain; 2] = [
    CultivationDomain::AuraFields,
    CultivationDomain::ContentFarm,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ContentFarmPractice {
    Educates,
    PreservesMemory,
    EntertainsResponsibly,
    CultivatesSkill,
    NourishesAttention,
    QuantityOverQuality,
    ExhaustsAudience,
    ImitatesWithoutUnderstanding,
    ManufacturesOutrage,
    CultivatesAddiction,
    AutomatesEmptyRepetition,
    HarvestsAttentionWithoutNourishment,
    FalsifiesPopularity,
}

impl ContentFarmPractice {
    #[must_use]
    pub const fn exploitative(self) -> bool {
        matches!(
            self,
            Self::QuantityOverQuality
                | Self::ExhaustsAudience
                | Self::ImitatesWithoutUnderstanding
                | Self::ManufacturesOutrage
                | Self::CultivatesAddiction
                | Self::AutomatesEmptyRepetition
                | Self::HarvestsAttentionWithoutNourishment
                | Self::FalsifiesPopularity
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentFarmAssessment {
    pub practices: BTreeSet<ContentFarmPractice>,
}

impl ContentFarmAssessment {
    #[must_use]
    pub fn is_exploitative(&self) -> bool {
        self.practices
            .iter()
            .any(|practice| practice.exploitative())
    }

    #[must_use]
    pub fn is_healthy(&self) -> bool {
        !self.practices.is_empty() && !self.is_exploitative()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CoastalZone {
    FreeAuraBeach,
    SouthernCoast,
    CurrentBreak,
    MinoanCountyCourthouse,
    GlaushouseBorder,
}

pub const NORTH_TO_SOUTH_COAST: [CoastalZone; 5] = [
    CoastalZone::FreeAuraBeach,
    CoastalZone::SouthernCoast,
    CoastalZone::CurrentBreak,
    CoastalZone::MinoanCountyCourthouse,
    CoastalZone::GlaushouseBorder,
];

impl CoastalZone {
    #[must_use]
    pub const fn governing_house(self) -> House {
        match self {
            Self::GlaushouseBorder => House::Glaushouse,
            Self::FreeAuraBeach
            | Self::SouthernCoast
            | Self::CurrentBreak
            | Self::MinoanCountyCourthouse => House::Sandmanor,
        }
    }

    #[must_use]
    pub const fn regulation_level(self) -> u8 {
        match self {
            Self::FreeAuraBeach => 0,
            Self::SouthernCoast => 1,
            Self::CurrentBreak => 2,
            Self::MinoanCountyCourthouse => 3,
            Self::GlaushouseBorder => 4,
        }
    }
}

#[must_use]
pub fn coast_is_progressively_regulated() -> bool {
    NORTH_TO_SOUTH_COAST
        .windows(2)
        .all(|zones| zones[0].regulation_level() < zones[1].regulation_level())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DesignDiscipline {
    InteriorCultivatedDesign,
    ExteriorCoastalDesign,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SandmanorDesignExposure {
    pub interior_cultivated_basis_points: u16,
    pub exterior_coastal_basis_points: u16,
}

impl SandmanorDesignExposure {
    pub fn validate(&self) -> Result<(), SandmanorMilestoneError> {
        if u32::from(self.interior_cultivated_basis_points)
            + u32::from(self.exterior_coastal_basis_points)
            != 10_000
        {
            return Err(SandmanorMilestoneError::InvalidDesignExposure);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GuardianMantle {
    GuardianOfTheFields,
    GuardianOfTheWholeFarm,
    GuardianOfTheBeach,
    GuardianOfTheHorizon,
    SandmanSovereign,
}

impl GuardianMantle {
    #[must_use]
    pub const fn required_regional_form(self) -> Option<SandmanorForm> {
        match self {
            Self::GuardianOfTheFields => Some(SandmanorForm::Minotaur),
            Self::GuardianOfTheWholeFarm => Some(SandmanorForm::Hecaton),
            Self::GuardianOfTheBeach => Some(SandmanorForm::Centaur),
            Self::GuardianOfTheHorizon => Some(SandmanorForm::Pegasus),
            Self::SandmanSovereign => None,
        }
    }

    #[must_use]
    pub const fn singular(self) -> bool {
        matches!(self, Self::SandmanSovereign)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GuardianAuthorityState {
    Active,
    UnderInvestigation,
    Suspended,
    Removed,
    Retired,
}

impl GuardianAuthorityState {
    #[must_use]
    pub const fn may_exercise_authority(self) -> bool {
        matches!(self, Self::Active)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuardianQualification {
    pub id: GuardianQualificationId,
    pub person: IdentityId,
    pub people: SandmanorPeople,
    pub current_form: SandmanorForm,
    pub target_form: SandmanorForm,
    pub evidence: Vec<EvidenceId>,
    pub sustained_service: bool,
    pub long_term_health_preserved: bool,
    pub accepts_responsibility_for_failure: bool,
    pub qualified: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuardianRecipeAuthorization {
    pub id: GuardianAuthorizationId,
    pub person: IdentityId,
    pub qualification: GuardianQualificationId,
    pub recipe: RecipeId,
    pub from: SandmanorForm,
    pub to: SandmanorForm,
    pub sandmanor_proof_recorded: bool,
    pub authorized: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuardianSynthesisRecord {
    pub id: GuardianSynthesisId,
    pub person: IdentityId,
    pub authorization: GuardianAuthorizationId,
    pub recipe: RecipeId,
    pub from: SandmanorForm,
    pub to: SandmanorForm,
    pub glaushouse_compatibility_cleared: bool,
    pub lawful_synthesis_completed: bool,
    pub lifecycle: SynthesisLifecycle,
    pub continuance: SynthesisContinuance,
}

impl GuardianSynthesisRecord {
    #[must_use]
    pub fn form_is_physically_present(&self) -> bool {
        self.lawful_synthesis_completed
            && self
                .lifecycle
                .current()
                .is_some_and(SynthesisLifecycleState::is_coherently_active)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuardianInvestiture {
    pub id: GuardianInvestitureId,
    pub person: IdentityId,
    pub synthesis: GuardianSynthesisId,
    pub mantle: GuardianMantle,
    pub jurisdiction: String,
    pub authority_state: GuardianAuthorityState,
    pub renewal_current: bool,
}

impl GuardianInvestiture {
    #[must_use]
    pub const fn may_exercise_authority(&self) -> bool {
        self.authority_state.may_exercise_authority() && self.renewal_current
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GuardianEventKind {
    QualificationRecorded(GuardianQualification),
    RecipeAuthorized(GuardianRecipeAuthorization),
    SynthesisRecorded(GuardianSynthesisRecord),
    MantleInvested(GuardianInvestiture),
    MantleInvestigationOpened(GuardianInvestitureId),
    MantleSuspended(GuardianInvestitureId),
    MantleRestored(GuardianInvestitureId),
    MantleRemoved(GuardianInvestitureId),
    MantleRetired(GuardianInvestitureId),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuardianEvent {
    pub id: GuardianEventId,
    pub kind: GuardianEventKind,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct GuardianState {
    pub qualifications: BTreeMap<GuardianQualificationId, GuardianQualification>,
    pub authorizations: BTreeMap<GuardianAuthorizationId, GuardianRecipeAuthorization>,
    pub syntheses: BTreeMap<GuardianSynthesisId, GuardianSynthesisRecord>,
    pub investitures: BTreeMap<GuardianInvestitureId, GuardianInvestiture>,
}

impl GuardianState {
    pub fn replay(events: &[GuardianEvent]) -> Result<Self, SandmanorMilestoneError> {
        let mut state = Self::default();
        let mut event_ids = BTreeSet::new();
        for event in events {
            if !event_ids.insert(event.id.clone()) {
                return Err(SandmanorMilestoneError::DuplicateRecord(
                    event.id.as_str().into(),
                ));
            }
            state.apply(&event.kind)?;
        }
        state.validate()?;
        Ok(state)
    }

    /// Applies one caller-ordered constitutional event and revalidates the
    /// resulting state. Stable record identity, not collection position,
    /// determines every reference.
    pub fn apply_event(&mut self, event: GuardianEventKind) -> Result<(), SandmanorMilestoneError> {
        self.apply(&event)?;
        self.validate()
    }

    fn apply(&mut self, event: &GuardianEventKind) -> Result<(), SandmanorMilestoneError> {
        match event {
            GuardianEventKind::QualificationRecorded(record) => {
                insert_unique(&mut self.qualifications, record.id.clone(), record.clone())?
            }
            GuardianEventKind::RecipeAuthorized(record) => {
                insert_unique(&mut self.authorizations, record.id.clone(), record.clone())?
            }
            GuardianEventKind::SynthesisRecorded(record) => {
                insert_unique(&mut self.syntheses, record.id.clone(), record.clone())?
            }
            GuardianEventKind::MantleInvested(record) => {
                insert_unique(&mut self.investitures, record.id.clone(), record.clone())?
            }
            GuardianEventKind::MantleInvestigationOpened(id) => {
                self.investiture_mut(id)?.authority_state =
                    GuardianAuthorityState::UnderInvestigation
            }
            GuardianEventKind::MantleSuspended(id) => {
                self.investiture_mut(id)?.authority_state = GuardianAuthorityState::Suspended
            }
            GuardianEventKind::MantleRestored(id) => {
                self.investiture_mut(id)?.authority_state = GuardianAuthorityState::Active
            }
            GuardianEventKind::MantleRemoved(id) => {
                self.investiture_mut(id)?.authority_state = GuardianAuthorityState::Removed
            }
            GuardianEventKind::MantleRetired(id) => {
                self.investiture_mut(id)?.authority_state = GuardianAuthorityState::Retired
            }
        }
        Ok(())
    }

    fn investiture_mut(
        &mut self,
        id: &GuardianInvestitureId,
    ) -> Result<&mut GuardianInvestiture, SandmanorMilestoneError> {
        self.investitures
            .get_mut(id)
            .ok_or_else(|| SandmanorMilestoneError::MissingRecord(id.as_str().into()))
    }

    pub fn validate(&self) -> Result<(), SandmanorMilestoneError> {
        for qualification in self.qualifications.values() {
            let prior_form_established = qualification.current_form
                == qualification.people.base_form()
                || self.syntheses.values().any(|synthesis| {
                    synthesis.person == qualification.person
                        && synthesis.to == qualification.current_form
                        && synthesis.form_is_physically_present()
                });
            if qualification.current_form.lineage() != qualification.people.lineage()
                || qualification.target_form.lineage() != qualification.people.lineage()
                || validate_sandmanor_transition(
                    qualification.current_form.frame(),
                    qualification.target_form.frame(),
                )
                .is_err()
                || qualification.evidence.is_empty()
                || !qualification.sustained_service
                || !qualification.long_term_health_preserved
                || !qualification.accepts_responsibility_for_failure
                || !qualification.qualified
                || !prior_form_established
            {
                return Err(SandmanorMilestoneError::InvalidQualification(
                    qualification.id.clone(),
                ));
            }
        }

        for authorization in self.authorizations.values() {
            let qualification = self
                .qualifications
                .get(&authorization.qualification)
                .ok_or_else(|| {
                    SandmanorMilestoneError::MissingRecord(
                        authorization.qualification.as_str().into(),
                    )
                })?;
            if authorization.person != qualification.person
                || authorization.from != qualification.current_form
                || authorization.to != qualification.target_form
                || !authorization.sandmanor_proof_recorded
                || !authorization.authorized
            {
                return Err(SandmanorMilestoneError::InvalidAuthorization(
                    authorization.id.clone(),
                ));
            }
        }

        for synthesis in self.syntheses.values() {
            let authorization = self
                .authorizations
                .get(&synthesis.authorization)
                .ok_or_else(|| {
                    SandmanorMilestoneError::MissingRecord(synthesis.authorization.as_str().into())
                })?;
            if synthesis.person != authorization.person
                || synthesis.recipe != authorization.recipe
                || synthesis.from != authorization.from
                || synthesis.to != authorization.to
                || !synthesis.glaushouse_compatibility_cleared
                || !synthesis.lawful_synthesis_completed
                || synthesis.lifecycle.validate().is_err()
                || !synthesis.form_is_physically_present()
                || !synthesis.continuance.conditions.fully_satisfied()
            {
                return Err(SandmanorMilestoneError::InvalidGuardianSynthesis(
                    synthesis.id.clone(),
                ));
            }
        }

        let active_sandmen = self
            .investitures
            .values()
            .filter(|record| {
                record.mantle == GuardianMantle::SandmanSovereign && record.may_exercise_authority()
            })
            .count();
        if active_sandmen > 1 {
            return Err(SandmanorMilestoneError::MultipleActiveSandmen);
        }
        for investiture in self.investitures.values() {
            let synthesis = self.syntheses.get(&investiture.synthesis).ok_or_else(|| {
                SandmanorMilestoneError::MissingRecord(investiture.synthesis.as_str().into())
            })?;
            if investiture.person != synthesis.person
                || investiture.jurisdiction.trim().is_empty()
                || investiture
                    .mantle
                    .required_regional_form()
                    .is_some_and(|required| synthesis.to != required)
            {
                return Err(SandmanorMilestoneError::InvalidInvestiture(
                    investiture.id.clone(),
                ));
            }
        }
        Ok(())
    }
}

fn insert_unique<K: Ord + fmt::Display, V>(
    records: &mut BTreeMap<K, V>,
    id: K,
    record: V,
) -> Result<(), SandmanorMilestoneError> {
    if records.insert(id, record).is_some() {
        return Err(SandmanorMilestoneError::DuplicateRecord(
            "stable guardian record".into(),
        ));
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourthouseTransfer {
    pub id: CoastalTransferId,
    pub person: IdentityId,
    pub from: CoastalZone,
    pub to: CoastalZone,
    pub lawful_transfer: bool,
    pub medical_or_clinical_reason: bool,
    pub courthouse_authority_retained_by: House,
    pub receiving_care_authority: House,
}

impl CourthouseTransfer {
    pub fn validate(&self) -> Result<(), SandmanorMilestoneError> {
        if self.from != CoastalZone::MinoanCountyCourthouse
            || self.to != CoastalZone::GlaushouseBorder
            || !self.lawful_transfer
            || !self.medical_or_clinical_reason
            || self.courthouse_authority_retained_by != House::Sandmanor
            || self.receiving_care_authority != House::Glaushouse
        {
            return Err(SandmanorMilestoneError::InvalidCourthouseTransfer(
                self.id.clone(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManticorpCurrentBreakTraining {
    pub id: MaritimeTrainingId,
    pub manticorp_institution: InstitutionId,
    pub flynt_authorized_unit: bool,
    pub sandmanor_authorized_access: bool,
    pub minoan_coastal_instruction: bool,
    pub command_house: House,
    pub territorial_house: House,
    pub creates_second_manticorp: bool,
}

impl ManticorpCurrentBreakTraining {
    pub fn validate(&self) -> Result<(), SandmanorMilestoneError> {
        if self.manticorp_institution != crate::world::flynt::manticorp_id()
            || !self.flynt_authorized_unit
            || !self.sandmanor_authorized_access
            || !self.minoan_coastal_instruction
            || self.command_house != House::Flynt
            || self.territorial_house != House::Sandmanor
            || self.creates_second_manticorp
        {
            return Err(SandmanorMilestoneError::InvalidManticorpHosting(
                self.id.clone(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TrialDomain {
    AuraField,
    ContentFarm,
    LibertyHospitality,
    RescueHorizon,
    ReciprocalIntegration,
}

impl TrialDomain {
    pub const ALL: [Self; 5] = [
        Self::AuraField,
        Self::ContentFarm,
        Self::LibertyHospitality,
        Self::RescueHorizon,
        Self::ReciprocalIntegration,
    ];
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandidateBaseline {
    pub candidate: IdentityId,
    pub competencies: BTreeMap<TrialDomain, u16>,
    pub weaknesses: Vec<String>,
    pub evidence: Vec<EvidenceId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImprovementEvidence {
    pub candidate: IdentityId,
    pub final_competencies: BTreeMap<TrialDomain, u16>,
    pub evidence: Vec<EvidenceId>,
    pub integration_demonstrated: bool,
    pub imitation_only: bool,
    pub degraded_other_candidate: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImprovementCandidate {
    pub person: IdentityId,
    pub people: SandmanorPeople,
    pub form: SandmanorForm,
    pub mantle: GuardianInvestitureId,
    pub baseline: CandidateBaseline,
    pub improvement: ImprovementEvidence,
    pub unresolved_disqualifying_corruption: bool,
}

impl ImprovementCandidate {
    pub fn improvement_total(&self) -> Result<u32, SandmanorMilestoneError> {
        if self.baseline.candidate != self.person
            || self.improvement.candidate != self.person
            || self.baseline.evidence.is_empty()
            || self.improvement.evidence.is_empty()
            || !self.improvement.integration_demonstrated
            || self.improvement.imitation_only
            || self.improvement.degraded_other_candidate
        {
            return Err(SandmanorMilestoneError::InvalidImprovementEvidence(
                self.person.clone(),
            ));
        }
        TrialDomain::ALL
            .into_iter()
            .try_fold(0_u32, |total, domain| {
                let baseline = self
                    .baseline
                    .competencies
                    .get(&domain)
                    .copied()
                    .ok_or_else(|| {
                        SandmanorMilestoneError::MissingTrialCompetency(self.person.clone(), domain)
                    })?;
                let final_score = self
                    .improvement
                    .final_competencies
                    .get(&domain)
                    .copied()
                    .ok_or_else(|| {
                        SandmanorMilestoneError::MissingTrialCompetency(self.person.clone(), domain)
                    })?;
                Ok(total + u32::from(final_score.saturating_sub(baseline)))
            })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicTrial {
    pub id: TrialId,
    pub domain: TrialDomain,
    pub candidate: IdentityId,
    pub teacher: IdentityId,
    pub evidence: Vec<EvidenceId>,
    pub candidate_completed_work: bool,
    pub valid: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeachingIntegrityFinding {
    pub teacher: IdentityId,
    pub sincere: bool,
    pub complete_enough_for_safety: bool,
    pub deliberately_false: bool,
    pub deliberately_dangerous: bool,
}

impl TeachingIntegrityFinding {
    #[must_use]
    pub const fn sabotaging(&self) -> bool {
        !self.sincere
            || !self.complete_enough_for_safety
            || self.deliberately_false
            || self.deliberately_dangerous
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrowdJudgment {
    pub id: CrowdJudgmentId,
    pub voter: IdentityId,
    pub candidate: IdentityId,
    pub assessed_improvement: u32,
    pub eligible: bool,
    pub conflicted: bool,
    pub coerced: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditedCrowdTally {
    pub valid_votes: BTreeMap<IdentityId, u32>,
    pub excluded_judgments: BTreeSet<CrowdJudgmentId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CrowdVerdict {
    Winner(IdentityId),
    Tie,
    NoWinner,
    VoidForTeachingSabotage(IdentityId),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContestOfImprovementProof {
    pub id: ContestId,
    pub candidates: [ImprovementCandidate; 2],
    pub trials: Vec<PublicTrial>,
    pub teaching_integrity: Vec<TeachingIntegrityFinding>,
    pub judgments: Vec<CrowdJudgment>,
}

impl ContestOfImprovementProof {
    pub fn evaluate(
        &self,
        guardians: &GuardianState,
    ) -> Result<(AuditedCrowdTally, CrowdVerdict), SandmanorMilestoneError> {
        let [first, second] = &self.candidates;
        if first.person == second.person
            || first.people == second.people
            || !matches!(
                (first.form, second.form),
                (SandmanorForm::Hecaton, SandmanorForm::Pegasus)
                    | (SandmanorForm::Pegasus, SandmanorForm::Hecaton)
            )
        {
            return Err(SandmanorMilestoneError::InvalidContestPair);
        }
        for candidate in &self.candidates {
            if candidate.unresolved_disqualifying_corruption {
                return Err(SandmanorMilestoneError::DisqualifiedContestCandidate(
                    candidate.person.clone(),
                ));
            }
            let mantle = guardians
                .investitures
                .get(&candidate.mantle)
                .ok_or_else(|| {
                    SandmanorMilestoneError::MissingRecord(candidate.mantle.as_str().into())
                })?;
            let expected = match candidate.form {
                SandmanorForm::Hecaton => GuardianMantle::GuardianOfTheWholeFarm,
                SandmanorForm::Pegasus => GuardianMantle::GuardianOfTheHorizon,
                _ => return Err(SandmanorMilestoneError::InvalidContestPair),
            };
            if mantle.person != candidate.person
                || mantle.mantle != expected
                || !mantle.may_exercise_authority()
            {
                return Err(SandmanorMilestoneError::CandidateWithoutLawfulMantle(
                    candidate.person.clone(),
                ));
            }
            candidate.improvement_total()?;
        }

        let trial_domains = self
            .trials
            .iter()
            .filter(|trial| {
                trial.valid && trial.candidate_completed_work && !trial.evidence.is_empty()
            })
            .map(|trial| trial.domain)
            .collect::<BTreeSet<_>>();
        if trial_domains != TrialDomain::ALL.into_iter().collect() {
            return Err(SandmanorMilestoneError::IncompleteTrialDomains);
        }
        let candidates = BTreeSet::from([first.person.clone(), second.person.clone()]);
        if self.trials.iter().any(|trial| {
            !candidates.contains(&trial.candidate)
                || !candidates.contains(&trial.teacher)
                || trial.candidate == trial.teacher
        }) {
            return Err(SandmanorMilestoneError::InvalidPublicTrial);
        }
        for finding in &self.teaching_integrity {
            if finding.sabotaging() {
                return Ok((
                    AuditedCrowdTally {
                        valid_votes: BTreeMap::new(),
                        excluded_judgments: self
                            .judgments
                            .iter()
                            .map(|judgment| judgment.id.clone())
                            .collect(),
                    },
                    CrowdVerdict::VoidForTeachingSabotage(finding.teacher.clone()),
                ));
            }
        }
        if self
            .teaching_integrity
            .iter()
            .map(|finding| &finding.teacher)
            .collect::<BTreeSet<_>>()
            != candidates.iter().collect::<BTreeSet<_>>()
        {
            return Err(SandmanorMilestoneError::MissingTeachingIntegrity);
        }

        let duplicate_voters = duplicate_values(self.judgments.iter().map(|entry| &entry.voter));
        let improvement = self
            .candidates
            .iter()
            .map(|candidate| Ok((candidate.person.clone(), candidate.improvement_total()?)))
            .collect::<Result<BTreeMap<_, _>, SandmanorMilestoneError>>()?;
        let mut tally = AuditedCrowdTally {
            valid_votes: candidates
                .iter()
                .cloned()
                .map(|candidate| (candidate, 0))
                .collect(),
            excluded_judgments: BTreeSet::new(),
        };
        let mut judgment_ids = BTreeSet::new();
        for judgment in &self.judgments {
            let valid = judgment_ids.insert(judgment.id.clone())
                && judgment.eligible
                && !judgment.conflicted
                && !judgment.coerced
                && !duplicate_voters.contains(&judgment.voter)
                && candidates.contains(&judgment.candidate)
                && improvement.get(&judgment.candidate) == Some(&judgment.assessed_improvement);
            if valid {
                *tally
                    .valid_votes
                    .get_mut(&judgment.candidate)
                    .expect("candidate tally was initialized") += 1;
            } else {
                tally.excluded_judgments.insert(judgment.id.clone());
            }
        }
        let first_votes = tally.valid_votes[&first.person];
        let second_votes = tally.valid_votes[&second.person];
        let verdict = if first_votes == 0 && second_votes == 0 {
            CrowdVerdict::NoWinner
        } else if first_votes == second_votes {
            CrowdVerdict::Tie
        } else if first_votes > second_votes {
            CrowdVerdict::Winner(first.person.clone())
        } else {
            CrowdVerdict::Winner(second.person.clone())
        };
        Ok((tally, verdict))
    }
}

fn duplicate_values<'a, T: Ord + Clone + 'a>(values: impl Iterator<Item = &'a T>) -> BTreeSet<T> {
    let mut seen = BTreeSet::new();
    let mut duplicate = BTreeSet::new();
    for value in values {
        if !seen.insert(value.clone()) {
            duplicate.insert(value.clone());
        }
    }
    duplicate
}

pub const SANDMAN_HISTORICAL_NAME: &str = "Aegon";
pub const SANDMAN_COMMON_NAME: &str = "The Sandman";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SandmanConvergence {
    pub person: IdentityId,
    pub source_form: SandmanorForm,
    pub recipe: RecipeId,
    pub integrates_other_way: bool,
    pub glaushouse_compatibility_cleared: bool,
    pub lifecycle: SynthesisLifecycle,
    pub continuance: SynthesisContinuance,
    pub fused_with_other_person: bool,
}

impl SandmanConvergence {
    pub fn validate(&self) -> Result<(), SandmanorMilestoneError> {
        if !matches!(
            self.source_form,
            SandmanorForm::Hecaton | SandmanorForm::Pegasus
        ) || !self.integrates_other_way
            || !self.glaushouse_compatibility_cleared
            || self.lifecycle.validate().is_err()
            || !self
                .lifecycle
                .current()
                .is_some_and(SynthesisLifecycleState::is_coherently_active)
            || !self.continuance.conditions.fully_satisfied()
            || self.fused_with_other_person
        {
            return Err(SandmanorMilestoneError::InvalidSandmanConvergence(
                self.person.clone(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SandmanMantleInvestiture {
    pub id: GuardianInvestitureId,
    pub person: IdentityId,
    pub mantle: GuardianMantle,
    pub authority_state: GuardianAuthorityState,
    pub renewal_current: bool,
}

impl SandmanMantleInvestiture {
    #[must_use]
    pub fn may_exercise_authority(&self) -> bool {
        self.mantle == GuardianMantle::SandmanSovereign
            && self.authority_state.may_exercise_authority()
            && self.renewal_current
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SandmanSuccession {
    pub id: SuccessionId,
    pub contest: ContestId,
    pub winner: IdentityId,
    pub crowd_verdict: CrowdVerdict,
    pub recipe_authorized: bool,
    pub convergence: SandmanConvergence,
    pub mantle: SandmanMantleInvestiture,
    pub historical_names: BTreeSet<String>,
    pub losing_candidate: IdentityId,
    pub loser_retains_form: bool,
    pub loser_retains_mantle: bool,
}

pub fn validate_sandman_successions(
    successions: &[SandmanSuccession],
) -> Result<(), SandmanorMilestoneError> {
    if successions
        .iter()
        .filter(|record| record.mantle.may_exercise_authority())
        .count()
        > 1
    {
        return Err(SandmanorMilestoneError::MultipleActiveSandmen);
    }
    for record in successions {
        if record.crowd_verdict != CrowdVerdict::Winner(record.winner.clone())
            || !record.recipe_authorized
            || record.convergence.person != record.winner
            || record.convergence.validate().is_err()
            || record.mantle.person != record.winner
            || !record.mantle.may_exercise_authority()
            || record.winner == record.losing_candidate
            || !record.loser_retains_form
            || !record.loser_retains_mantle
            || !record.historical_names.contains(SANDMAN_HISTORICAL_NAME)
            || record.historical_names.contains("Aegaeon")
        {
            return Err(SandmanorMilestoneError::InvalidSandmanSuccession(
                record.id.clone(),
            ));
        }
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SandmanorMilestoneError {
    InvalidStableId(String),
    DuplicateRecord(String),
    MissingRecord(String),
    InvalidDesignExposure,
    InvalidQualification(GuardianQualificationId),
    InvalidAuthorization(GuardianAuthorizationId),
    InvalidGuardianSynthesis(GuardianSynthesisId),
    InvalidInvestiture(GuardianInvestitureId),
    MultipleActiveSandmen,
    InvalidCourthouseTransfer(CoastalTransferId),
    InvalidManticorpHosting(MaritimeTrainingId),
    InvalidContestPair,
    CandidateWithoutLawfulMantle(IdentityId),
    DisqualifiedContestCandidate(IdentityId),
    InvalidImprovementEvidence(IdentityId),
    MissingTrialCompetency(IdentityId, TrialDomain),
    IncompleteTrialDomains,
    InvalidPublicTrial,
    MissingTeachingIntegrity,
    InvalidSandmanConvergence(IdentityId),
    InvalidSandmanSuccession(SuccessionId),
}

impl fmt::Display for SandmanorMilestoneError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Sandmanor milestone violation: {self:?}")
    }
}

impl std::error::Error for SandmanorMilestoneError {}
