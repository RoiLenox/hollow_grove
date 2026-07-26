//! Executable Glaüshouse constitutional law above the frozen Constitutional
//! Runtime V2.
//!
//! This module owns Glaüshouse-specific diagnosis, consent, capacity,
//! clearance, clinical privilege, recovery, and Synthesis validation. It does
//! not alter the recursion kernel, prove a Sandmanor design, grant a Stonebend
//! Title, grant Flynt recognition, or duplicate the common Current Runtime.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::institution::{IdentityId, InstitutionId, OfficeId, RoleId};

pub const GLAUSHOUSE_CONSTITUTION_SOURCE: &str = "GLAUSHOUSE_CONSTITUTION_V2.md";
pub const GLAUSHOUSE_GOVERNING_VERB: &str = "Clear";
pub const GLAUSHOUSE_SIGNATURE_OFFENSE: &str = "Illegal Synthesis";

fn institution(value: &str) -> InstitutionId {
    InstitutionId::new(value).expect("canonical Glaüshouse institution ID")
}

fn office(value: &str) -> OfficeId {
    OfficeId::new(value).expect("canonical Glaüshouse office ID")
}

#[must_use]
pub fn medical_civilization_id() -> InstitutionId {
    institution("institution.glaushouse.medical-civilization")
}

#[must_use]
pub fn glauspitals_id() -> InstitutionId {
    institution("institution.glaushouse.glauspitals")
}

#[must_use]
pub fn chromacord_id() -> InstitutionId {
    institution("institution.glaushouse.chromacord")
}

#[must_use]
pub fn nightingales_id() -> InstitutionId {
    institution("institution.glaushouse.nightingales")
}

#[must_use]
pub fn prima_donna_office_id() -> OfficeId {
    office("office.glaushouse.prima-donna")
}

fn role(value: &str) -> RoleId {
    RoleId::new(value).expect("canonical Glaüshouse role ID")
}

#[must_use]
pub fn nightingale_rank_id() -> RoleId {
    role("role.glaushouse.nightingale")
}

#[must_use]
pub fn matron_rank_id() -> RoleId {
    role("role.glaushouse.matron")
}

#[must_use]
pub fn marshal_rank_id() -> RoleId {
    role("role.glaushouse.marshal")
}

#[must_use]
pub fn persephone_rank_id() -> RoleId {
    role("role.glaushouse.persephone")
}

/// Frozen current identity of the holder presented as Doctor Ratchet.
#[must_use]
pub fn doctor_ratchet_identity_id() -> IdentityId {
    IdentityId::new("being.glaushouse.doctor-ratchet").expect("canonical holder identity")
}

/// Frozen current identity of the holder presented as Nurse House.
#[must_use]
pub fn nurse_house_identity_id() -> IdentityId {
    IdentityId::new("being.glaushouse.nurse-house").expect("canonical holder identity")
}

macro_rules! glaushouse_id {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, GlaushouseIdError> {
                let value = value.into();
                if value.is_empty()
                    || !value.bytes().all(|byte| {
                        byte.is_ascii_lowercase()
                            || byte.is_ascii_digit()
                            || byte == b'.'
                            || byte == b'-'
                    })
                {
                    return Err(GlaushouseIdError(value));
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlaushouseIdError(String);

impl fmt::Display for GlaushouseIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "invalid Glaüshouse stable identifier: {}",
            self.0
        )
    }
}

impl std::error::Error for GlaushouseIdError {}

glaushouse_id!(AccessionRecordId);
glaushouse_id!(DiagnosisRecordId);
glaushouse_id!(CapacityRecordId);
glaushouse_id!(ConsentRecordId);
glaushouse_id!(ClearanceRecordId);
glaushouse_id!(PrivilegeRecordId);
glaushouse_id!(MaterialRecordId);
glaushouse_id!(SynthesisRecordId);
glaushouse_id!(RecoveryPlanId);
glaushouse_id!(NightingaleStopId);
glaushouse_id!(ClinicalReviewId);
glaushouse_id!(CustodyRecordId);
glaushouse_id!(RecoveryObligationId);
glaushouse_id!(InstitutionalSuccessionId);
glaushouse_id!(AdvancementTokenId);
glaushouse_id!(PrimaDonnaCandidacyId);
glaushouse_id!(LedgerEntryId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PrincipalAuthority {
    PrimaDonna,
    Persephone,
    Matron,
    Marshal,
    Nightingales,
}

impl PrincipalAuthority {
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::PrimaDonna => "Prima Donna",
            Self::Persephone => "Persephone",
            Self::Matron => "Matron",
            Self::Marshal => "Marshal",
            Self::Nightingales => "The Nightingales",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConstitutionalPlacement {
    SingularHighestClinicalOffice,
    MultipleBalancedRank,
    EqualAuraForwardBranch,
    EqualCurrentForwardBranch,
    UniversalClinicalFoundation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AuthorityDefinition {
    pub authority: PrincipalAuthority,
    pub placement: ConstitutionalPlacement,
    pub singular: bool,
    pub constitutional_power: &'static str,
}

pub const PRINCIPAL_AUTHORITIES: [AuthorityDefinition; 5] = [
    AuthorityDefinition {
        authority: PrincipalAuthority::PrimaDonna,
        placement: ConstitutionalPlacement::SingularHighestClinicalOffice,
        singular: true,
        constitutional_power: "clinical sovereignty",
    },
    AuthorityDefinition {
        authority: PrincipalAuthority::Persephone,
        placement: ConstitutionalPlacement::MultipleBalancedRank,
        singular: false,
        constitutional_power: "whole-course Continuance",
    },
    AuthorityDefinition {
        authority: PrincipalAuthority::Matron,
        placement: ConstitutionalPlacement::EqualAuraForwardBranch,
        singular: false,
        constitutional_power: "lived continuity",
    },
    AuthorityDefinition {
        authority: PrincipalAuthority::Marshal,
        placement: ConstitutionalPlacement::EqualCurrentForwardBranch,
        singular: false,
        constitutional_power: "bodily continuity",
    },
    AuthorityDefinition {
        authority: PrincipalAuthority::Nightingales,
        placement: ConstitutionalPlacement::UniversalClinicalFoundation,
        singular: false,
        constitutional_power: "bedside protection, maintenance, and renewal",
    },
];

pub fn validate_principal_authorities() -> Result<(), GlaushouseValidationError> {
    let unique = PRINCIPAL_AUTHORITIES
        .iter()
        .map(|entry| entry.authority)
        .collect::<BTreeSet<_>>();
    if unique.len() != PRINCIPAL_AUTHORITIES.len() {
        return Err(GlaushouseValidationError::DuplicatePrincipalAuthority);
    }
    if PRINCIPAL_AUTHORITIES
        .iter()
        .filter(|entry| entry.placement == ConstitutionalPlacement::SingularHighestClinicalOffice)
        .count()
        != 1
    {
        return Err(GlaushouseValidationError::InvalidPrincipalAuthorityPlacement);
    }
    let matron = PRINCIPAL_AUTHORITIES
        .iter()
        .find(|entry| entry.authority == PrincipalAuthority::Matron)
        .expect("canonical Matron authority");
    let marshal = PRINCIPAL_AUTHORITIES
        .iter()
        .find(|entry| entry.authority == PrincipalAuthority::Marshal)
        .expect("canonical Marshal authority");
    if matron.singular
        || marshal.singular
        || matron.constitutional_power == marshal.constitutional_power
    {
        return Err(GlaushouseValidationError::UnequalClinicalBranches);
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ClinicalRank {
    Nightingale,
    Matron,
    Marshal,
    Persephone,
}

impl ClinicalRank {
    #[must_use]
    pub const fn role_id(self) -> &'static str {
        match self {
            Self::Nightingale => "role.glaushouse.nightingale",
            Self::Matron => "role.glaushouse.matron",
            Self::Marshal => "role.glaushouse.marshal",
            Self::Persephone => "role.glaushouse.persephone",
        }
    }

    #[must_use]
    pub const fn advancement_level(self) -> u8 {
        match self {
            Self::Nightingale => 0,
            Self::Matron | Self::Marshal => 1,
            Self::Persephone => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClinicalBranch {
    Matron,
    Marshal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AdvancementEvidenceKind {
    NightingaleCare,
    AuraSensitiveObservation,
    CurrentSensitiveStabilization,
    IdentityContinuity,
    BodilyContinuity,
    MaintenanceAndRenewal,
    RecoveryAndRehabilitation,
    SafeRegression,
    DischargeJudgment,
    ReconciledEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdvancementToken {
    pub id: AdvancementTokenId,
    pub clinician: IdentityId,
    pub recognized_rank: ClinicalRank,
    pub evidence: BTreeSet<AdvancementEvidenceKind>,
    pub clinical_experience: Vec<String>,
    pub authority_granted: Vec<String>,
    pub education_access: Vec<String>,
    pub patient_responsibility: Vec<String>,
    pub objective_benefit: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClinicalStanding {
    pub clinician: IdentityId,
    pub earned_ranks: BTreeSet<ClinicalRank>,
    pub active_branch_emphasis: Option<ClinicalBranch>,
    pub advancement_tokens: Vec<AdvancementTokenId>,
}

impl ClinicalStanding {
    #[must_use]
    pub fn has_rank(&self, rank: ClinicalRank) -> bool {
        self.earned_ranks.contains(&rank)
    }

    #[must_use]
    pub fn is_qualified_persephone(&self) -> bool {
        [
            ClinicalRank::Nightingale,
            ClinicalRank::Matron,
            ClinicalRank::Marshal,
            ClinicalRank::Persephone,
        ]
        .into_iter()
        .all(|rank| self.has_rank(rank))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PrimaDonnaEvidenceKind {
    NightingaleFoundation,
    MatronMastery,
    MarshalMastery,
    PersephoneService,
    PhysicianMastery,
    PatientOutcomes,
    TechnicalViabilityJudgment,
    LivedViabilityJudgment,
    DiagnosticMastery,
    RecipeAuthorshipOrRevision,
    SurgicalAuthority,
    SynthesisLedgerStewardship,
    TeachingAndCultivation,
    AdvancementPathsRemainOpen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PrimaDonnaSuccessionSource {
    NightingaleTestimony,
    MatronRecordOrTestimony,
    MarshalRecordOrTestimony,
    PersephoneRecordOrTestimony,
    TreatedHuemanTestimony,
    LivingLedger,
    RecipeLedger,
    ClinicalOutcomes,
    TeachingHistory,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrimaDonnaCandidacy {
    pub id: PrimaDonnaCandidacyId,
    pub candidate: IdentityId,
    pub evidence: BTreeSet<PrimaDonnaEvidenceKind>,
    pub succession_sources: BTreeSet<PrimaDonnaSuccessionSource>,
    pub testimony: Vec<String>,
    pub eligible: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClinicalOffice {
    PrimaDonna,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AuthorityOrigin {
    ClinicalAccession,
    Transformation,
    RecognitionAlone,
    LegacyState,
    TechnicalAbility,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessionRecord {
    pub id: AccessionRecordId,
    pub office: ClinicalOffice,
    pub holder: IdentityId,
    pub active: bool,
    pub tombstoned: bool,
    pub origin: AuthorityOrigin,
    pub clinical_competence_reviewed: bool,
    pub nightingale_testimony_recorded: bool,
    pub persephone_review_completed: bool,
    pub flynt_recognition_recorded: bool,
    pub stonebend_title_recorded: bool,
    pub evidence: Vec<String>,
    pub candidacy: PrimaDonnaCandidacyId,
    pub advancement_paths_open: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrimaDonnaSelectionPhase {
    Vacancy,
    CandidateNomination,
    ClinicalRecordReview,
    AdverseEventReview,
    NightingaleTestimony,
    PersephoneEvaluation,
    PublicAndProfessionalHearing,
    SandmanorReviewIfMaterial,
    FlyntRecognition,
    StonebendTitleAndAccession,
    GlaushouseOath,
    ConstitutionalSeal,
}

impl PrimaDonnaSelectionPhase {
    #[must_use]
    pub const fn next(self) -> Option<Self> {
        match self {
            Self::Vacancy => Some(Self::CandidateNomination),
            Self::CandidateNomination => Some(Self::ClinicalRecordReview),
            Self::ClinicalRecordReview => Some(Self::AdverseEventReview),
            Self::AdverseEventReview => Some(Self::NightingaleTestimony),
            Self::NightingaleTestimony => Some(Self::PersephoneEvaluation),
            Self::PersephoneEvaluation => Some(Self::PublicAndProfessionalHearing),
            Self::PublicAndProfessionalHearing => Some(Self::SandmanorReviewIfMaterial),
            Self::SandmanorReviewIfMaterial => Some(Self::FlyntRecognition),
            Self::FlyntRecognition => Some(Self::StonebendTitleAndAccession),
            Self::StonebendTitleAndAccession => Some(Self::GlaushouseOath),
            Self::GlaushouseOath => Some(Self::ConstitutionalSeal),
            Self::ConstitutionalSeal => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrimaDonnaSelectionProcess {
    candidate: IdentityId,
    phase: PrimaDonnaSelectionPhase,
    evidence: Vec<String>,
}

impl PrimaDonnaSelectionProcess {
    #[must_use]
    pub fn open(candidate: IdentityId) -> Self {
        Self {
            candidate,
            phase: PrimaDonnaSelectionPhase::Vacancy,
            evidence: Vec::new(),
        }
    }

    #[must_use]
    pub const fn phase(&self) -> PrimaDonnaSelectionPhase {
        self.phase
    }

    #[must_use]
    pub const fn candidate(&self) -> &IdentityId {
        &self.candidate
    }

    pub fn advance(
        &mut self,
        next: PrimaDonnaSelectionPhase,
        evidence: impl Into<String>,
    ) -> Result<(), GlaushouseValidationError> {
        let expected = self
            .phase
            .next()
            .ok_or(GlaushouseValidationError::CompletedSelectionCannotAdvance)?;
        if next != expected {
            return Err(GlaushouseValidationError::InvalidSelectionTransition {
                from: self.phase,
                expected,
                attempted: next,
            });
        }
        let evidence = evidence.into();
        require_text(&evidence, "selection evidence")?;
        self.phase = next;
        self.evidence.push(evidence);
        Ok(())
    }

    pub fn require_complete(&self) -> Result<(), GlaushouseValidationError> {
        if self.phase == PrimaDonnaSelectionPhase::ConstitutionalSeal && self.evidence.len() == 11 {
            Ok(())
        } else {
            Err(GlaushouseValidationError::IncompleteSelection(self.phase))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClinicalSubjectRecord {
    pub id: IdentityId,
    pub prior_identity_history: Vec<String>,
    pub tombstoned: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiagnosisStatus {
    Provisional,
    Confirmed,
    Differential,
    Functional,
    Aura,
    Current,
    TransformationReadiness,
    PostSynthesis,
    Recovery,
    Terminal,
    PublicHealth,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosisRecord {
    pub id: DiagnosisRecordId,
    pub subject: IdentityId,
    pub status: DiagnosisStatus,
    pub findings: String,
    pub evidence: Vec<String>,
    pub uncertainty_disclosed: bool,
    pub operator: IdentityId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapacityRecord {
    pub id: CapacityRecordId,
    pub subject: IdentityId,
    pub understands_information: bool,
    pub appreciates_consequences: bool,
    pub compares_options: bool,
    pub communicates_choice: bool,
    pub support_offered: Vec<String>,
    pub assessed_at: u64,
    pub expires_at: u64,
}

impl CapacityRecord {
    #[must_use]
    pub const fn has_capacity(&self) -> bool {
        self.understands_information
            && self.appreciates_consequences
            && self.compares_options
            && self.communicates_choice
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConsentScope {
    Examination,
    Treatment,
    Hollowing,
    Synthesis,
    Research,
    MentalOrAuraIntervention,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConsentOrigin {
    Explicit,
    Silence,
    Custody,
    Dependence,
    Recognition,
    AuraInfluence,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConsentRecord {
    pub id: ConsentRecordId,
    pub subject: IdentityId,
    pub scope: ConsentScope,
    pub procedure: String,
    pub capacity: CapacityRecordId,
    pub origin: ConsentOrigin,
    pub informed: bool,
    pub voluntary: bool,
    pub specific: bool,
    pub comprehensible: bool,
    pub current: bool,
    pub material_risks_disclosed: bool,
    pub alternatives_disclosed: bool,
    pub recovery_disclosed: bool,
    pub experimental_status_disclosed: bool,
    pub withdrawn_at: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClearanceClass {
    Routine,
    Conditional,
    Emergency,
    HighRisk,
    Experimental,
    Restorative,
    Synthesis,
    Reversal,
    PublicHealth,
    Postmortem,
    Temporary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClearanceStatus {
    Active,
    Suspended,
    Revoked,
    Expired,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClearanceRecord {
    pub id: ClearanceRecordId,
    pub subject: IdentityId,
    pub diagnosis: DiagnosisRecordId,
    pub procedure: String,
    pub operator: IdentityId,
    pub facility: String,
    pub scope: String,
    pub class: ClearanceClass,
    pub consent: ConsentRecordId,
    pub capacity: CapacityRecordId,
    pub required_equipment: Vec<String>,
    pub required_witnesses: Vec<String>,
    pub emergency_plan: String,
    pub recovery_plan: Option<RecoveryPlanId>,
    pub stopping_conditions: Vec<String>,
    pub issued_at: u64,
    pub expires_at: u64,
    pub status: ClearanceStatus,
    pub review_authority: PrincipalAuthority,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrivilegeStatus {
    Active,
    Suspended,
    Revoked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorPrivilegeRecord {
    pub id: PrivilegeRecordId,
    pub operator: IdentityId,
    pub permitted_procedures: Vec<String>,
    pub facility: String,
    pub status: PrivilegeStatus,
    pub valid_until: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SynthesisMaterialRecord {
    pub id: MaterialRecordId,
    pub material: IdentityId,
    pub source: IdentityId,
    pub provenance: Vec<String>,
    pub custody_chain: Vec<String>,
    pub lawfully_obtained: bool,
    pub illegally_hollowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryPlan {
    pub id: RecoveryPlanId,
    pub subject: IdentityId,
    pub stabilization: Vec<String>,
    pub monitoring: Vec<String>,
    pub rehabilitation: Vec<String>,
    pub identity_support: Vec<String>,
    pub discharge_conditions: Vec<String>,
    pub responsible_institution: InstitutionId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SynthesisDepth {
    Adjustment,
    Graft,
    Reconstruction,
    Transfiguration,
    Overgrowth,
}

impl SynthesisDepth {
    #[must_use]
    pub const fn is_intended_clinical_depth(self) -> bool {
        !matches!(self, Self::Overgrowth)
    }

    #[must_use]
    pub const fn requires_major_viability(self) -> bool {
        matches!(self, Self::Reconstruction | Self::Transfiguration)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SynthesisLifecycleState {
    Established,
    Integrated,
    Maintained,
    Renewed,
    Refined,
    Revised,
    Weakened,
    Regressed,
    SafelyDiscontinued,
    Replaced,
    CatastrophicallyCollapsed,
}

impl SynthesisLifecycleState {
    #[must_use]
    pub const fn may_transition_to(self, next: Self) -> bool {
        use SynthesisLifecycleState as State;
        match self {
            State::Established => matches!(
                next,
                State::Integrated
                    | State::Weakened
                    | State::Regressed
                    | State::SafelyDiscontinued
                    | State::CatastrophicallyCollapsed
            ),
            State::Integrated => matches!(
                next,
                State::Maintained
                    | State::Weakened
                    | State::Regressed
                    | State::SafelyDiscontinued
                    | State::CatastrophicallyCollapsed
            ),
            State::Maintained => matches!(
                next,
                State::Renewed
                    | State::Refined
                    | State::Revised
                    | State::Weakened
                    | State::Regressed
                    | State::SafelyDiscontinued
                    | State::Replaced
                    | State::CatastrophicallyCollapsed
            ),
            State::Renewed | State::Refined | State::Revised => matches!(
                next,
                State::Maintained
                    | State::Renewed
                    | State::Refined
                    | State::Revised
                    | State::Weakened
                    | State::Regressed
                    | State::SafelyDiscontinued
                    | State::Replaced
                    | State::CatastrophicallyCollapsed
            ),
            State::Weakened => matches!(
                next,
                State::Maintained
                    | State::Renewed
                    | State::Revised
                    | State::Regressed
                    | State::SafelyDiscontinued
                    | State::Replaced
                    | State::CatastrophicallyCollapsed
            ),
            State::Regressed => matches!(
                next,
                State::Maintained
                    | State::Renewed
                    | State::Revised
                    | State::SafelyDiscontinued
                    | State::Replaced
                    | State::CatastrophicallyCollapsed
            ),
            State::SafelyDiscontinued | State::Replaced | State::CatastrophicallyCollapsed => false,
        }
    }

    #[must_use]
    pub const fn is_coherently_active(self) -> bool {
        matches!(
            self,
            Self::Integrated | Self::Maintained | Self::Renewed | Self::Refined | Self::Revised
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SynthesisLifecycle {
    pub history: Vec<SynthesisLifecycleState>,
}

impl SynthesisLifecycle {
    #[must_use]
    pub fn current(&self) -> Option<SynthesisLifecycleState> {
        self.history.last().copied()
    }

    pub fn validate(&self) -> Result<(), GlaushouseValidationError> {
        if self.history.first() != Some(&SynthesisLifecycleState::Established) {
            return Err(GlaushouseValidationError::InvalidSynthesisLifecycle);
        }
        if self
            .history
            .windows(2)
            .any(|states| !states[0].may_transition_to(states[1]))
        {
            return Err(GlaushouseValidationError::InvalidSynthesisLifecycle);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SynthesisWay {
    pub name: String,
    pub practices: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SynthesisRecipeReference {
    pub name: String,
    pub revision: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContinuanceHorizon {
    NaturalLifePossible,
    TimeBound,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContinuanceConditions {
    pub maintenance_current: bool,
    pub renewal_current: bool,
    pub recipe_practiced: bool,
    pub environment_compatible: bool,
    pub institutional_care_available: bool,
    pub ways_known_and_practiced: bool,
}

impl ContinuanceConditions {
    #[must_use]
    pub const fn fully_satisfied(&self) -> bool {
        self.maintenance_current
            && self.renewal_current
            && self.recipe_practiced
            && self.environment_compatible
            && self.institutional_care_available
            && self.ways_known_and_practiced
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SynthesisContinuance {
    pub recipe: SynthesisRecipeReference,
    pub ways: Vec<SynthesisWay>,
    pub maintenance: Vec<String>,
    pub renewal: Vec<String>,
    pub environmental_conditions: Vec<String>,
    pub institutional_care: Vec<InstitutionId>,
    pub conditions: ContinuanceConditions,
    pub expected_continuance: ContinuanceHorizon,
}

impl SynthesisContinuance {
    #[must_use]
    pub const fn may_continue_for_natural_life(&self) -> bool {
        matches!(
            self.expected_continuance,
            ContinuanceHorizon::NaturalLifePossible
        ) && self.conditions.fully_satisfied()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HostRejectionMechanism {
    Attack,
    Isolate,
    Starve,
    ScarAround,
    Destabilize,
    Expel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SympioteRejectionMechanism {
    TreatHostAsDamagedMaterial,
    InvadeHealthyStructures,
    Overcorrect,
    ImposeRecipeAggressively,
    EraseNeededCapacities,
    ConstructUnconsentedBody,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RejectionRecord {
    Host {
        mechanisms: Vec<HostRejectionMechanism>,
    },
    Sympiote {
        mechanisms: Vec<SympioteRejectionMechanism>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TechnicalViability {
    pub recipe_possible: bool,
    pub materials_available: bool,
    pub compatibility_understood: bool,
    pub intended_form_described: bool,
    pub risks_known: bool,
    pub lawful_path_exists: bool,
}

impl TechnicalViability {
    #[must_use]
    pub const fn is_viable(&self) -> bool {
        self.recipe_possible
            && self.materials_available
            && self.compatibility_understood
            && self.intended_form_described
            && self.risks_known
            && self.lawful_path_exists
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LivedViability {
    pub consent_meaningful: bool,
    pub consequences_understood: bool,
    pub maintenance_resources_exist: bool,
    pub recovery_resources_exist: bool,
    pub continuing_care_available: bool,
    pub expected_form_livable: bool,
    pub coercion_excluded: bool,
}

impl LivedViability {
    #[must_use]
    pub const fn is_viable(&self) -> bool {
        self.consent_meaningful
            && self.consequences_understood
            && self.maintenance_resources_exist
            && self.recovery_resources_exist
            && self.continuing_care_available
            && self.expected_form_livable
            && self.coercion_excluded
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LedgerLayer {
    Public,
    Living,
    Recipe,
    Black,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatronEvidence {
    pub cognition: String,
    pub perception: String,
    pub consent: String,
    pub emotion: String,
    pub identity: String,
    pub aura_coherence: String,
    pub patient_testimony: String,
    pub lived_adaptation: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarshalEvidence {
    pub current_flow: String,
    pub structural_condition: String,
    pub graft_integrity: String,
    pub mobility: String,
    pub physical_tolerance: String,
    pub containment: String,
    pub bodily_adaptation: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LivingLedgerEntry {
    pub id: LedgerEntryId,
    pub subject: IdentityId,
    pub matron: MatronEvidence,
    pub marshal: MarshalEvidence,
    pub overseen_by_persephones: Vec<IdentityId>,
    pub continuities_form_one_viable_life: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecipeLedgerEntry {
    pub id: LedgerEntryId,
    pub recipe: SynthesisRecipeReference,
    pub architecture: Vec<String>,
    pub revision_history: Vec<String>,
    pub authorized_by_prima_donna: IdentityId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SynthesisClass {
    Restorative,
    Elective,
    Developmental,
    Regional,
    Emergency,
    Composite,
    Staged,
    Experimental,
    Reversible,
    PartiallyReversible,
    Irreversible,
    Successor,
    Corrective,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SynthesisOutcome {
    Successful,
    NoTransformation,
    PartialTransformation,
    UnstableTransformation,
    UnintendedForm,
    LossOfCapability,
    IncompatibleFusion,
    SevereAuraInstability,
    SevereCurrentInstability,
    IdentityFragmentation,
    HostRejection,
    SympioteRejection,
    Overgrowth,
    SafeRegression,
    CatastrophicCollapse,
    IrreversibleInjury,
    Death,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SynthesisRecord {
    pub id: SynthesisRecordId,
    pub subject: IdentityId,
    pub class: SynthesisClass,
    pub depth: SynthesisDepth,
    pub recipe: String,
    pub intended_result: String,
    pub clearance: ClearanceRecordId,
    pub consent: ConsentRecordId,
    pub operator_privilege: PrivilegeRecordId,
    pub materials: Vec<MaterialRecordId>,
    pub recovery_plan: Option<RecoveryPlanId>,
    pub started_at: u64,
    pub irreversible_threshold_at: Option<u64>,
    pub experimental_marked: bool,
    pub nightingale_witness: Option<IdentityId>,
    pub emergency_post_event_review: Option<ClinicalReviewId>,
    pub actual_outcome: SynthesisOutcome,
    pub recorded_outcome: SynthesisOutcome,
    pub lifecycle: SynthesisLifecycle,
    pub continuance: SynthesisContinuance,
    pub rejection: Option<RejectionRecord>,
    pub technical_viability: Option<TechnicalViability>,
    pub lived_viability: Option<LivedViability>,
    pub prior_identity_history_preserved: bool,
    pub resulting_title_or_office: bool,
    pub stabilized: bool,
    pub recovery_status_recorded: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NightingaleStopRecord {
    pub id: NightingaleStopId,
    pub subject: IdentityId,
    pub issued_by: IdentityId,
    pub reason: String,
    pub issued_at: u64,
    pub mandatory_review: Option<ClinicalReviewId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClinicalReviewRecord {
    pub id: ClinicalReviewId,
    pub subject: IdentityId,
    pub authority: PrincipalAuthority,
    pub scope: String,
    pub completed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClinicalCustodyRecord {
    pub id: CustodyRecordId,
    pub subject: IdentityId,
    pub custodian: IdentityId,
    pub purpose: String,
    pub ownership_claimed: bool,
    pub expires_at: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryObligation {
    pub id: RecoveryObligationId,
    pub subject: IdentityId,
    pub responsible_institution: InstitutionId,
    pub duty: String,
    pub discharged: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstitutionalSuccessionRecord {
    pub id: InstitutionalSuccessionId,
    pub predecessor: InstitutionId,
    pub successor: InstitutionId,
    pub predecessor_obligations: Vec<RecoveryObligationId>,
    pub successor_obligations: Vec<RecoveryObligationId>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct GlaushouseRegistry {
    pub subjects: Vec<ClinicalSubjectRecord>,
    pub clinical_standings: Vec<ClinicalStanding>,
    pub advancement_tokens: Vec<AdvancementToken>,
    pub prima_donna_candidacies: Vec<PrimaDonnaCandidacy>,
    pub accessions: Vec<AccessionRecord>,
    pub diagnoses: Vec<DiagnosisRecord>,
    pub capacities: Vec<CapacityRecord>,
    pub consents: Vec<ConsentRecord>,
    pub clearances: Vec<ClearanceRecord>,
    pub privileges: Vec<OperatorPrivilegeRecord>,
    pub materials: Vec<SynthesisMaterialRecord>,
    pub recovery_plans: Vec<RecoveryPlan>,
    pub syntheses: Vec<SynthesisRecord>,
    pub nightingale_stops: Vec<NightingaleStopRecord>,
    pub reviews: Vec<ClinicalReviewRecord>,
    pub custody: Vec<ClinicalCustodyRecord>,
    pub recovery_obligations: Vec<RecoveryObligation>,
    pub successions: Vec<InstitutionalSuccessionRecord>,
    pub living_ledger: Vec<LivingLedgerEntry>,
    pub recipe_ledger: Vec<RecipeLedgerEntry>,
}

impl GlaushouseRegistry {
    pub fn validate(&self) -> Result<(), GlaushouseValidationError> {
        validate_principal_authorities()?;
        validate_unique(self.subjects.iter().map(|record| &record.id), "subject")?;
        validate_unique(
            self.clinical_standings
                .iter()
                .map(|record| &record.clinician),
            "clinical standing",
        )?;
        validate_unique(
            self.advancement_tokens.iter().map(|record| &record.id),
            "advancement token",
        )?;
        validate_unique(
            self.prima_donna_candidacies.iter().map(|record| &record.id),
            "Prima Donna candidacy",
        )?;
        validate_unique(self.accessions.iter().map(|record| &record.id), "accession")?;
        validate_unique(self.diagnoses.iter().map(|record| &record.id), "diagnosis")?;
        validate_unique(self.capacities.iter().map(|record| &record.id), "capacity")?;
        validate_unique(self.consents.iter().map(|record| &record.id), "consent")?;
        validate_unique(self.clearances.iter().map(|record| &record.id), "clearance")?;
        validate_unique(self.privileges.iter().map(|record| &record.id), "privilege")?;
        validate_unique(self.materials.iter().map(|record| &record.id), "material")?;
        validate_unique(
            self.recovery_plans.iter().map(|record| &record.id),
            "recovery plan",
        )?;
        validate_unique(self.syntheses.iter().map(|record| &record.id), "Synthesis")?;
        validate_unique(
            self.nightingale_stops.iter().map(|record| &record.id),
            "Nightingale stop",
        )?;
        validate_unique(
            self.reviews.iter().map(|record| &record.id),
            "clinical review",
        )?;
        validate_unique(
            self.custody.iter().map(|record| &record.id),
            "clinical custody",
        )?;
        validate_unique(
            self.recovery_obligations.iter().map(|record| &record.id),
            "recovery obligation",
        )?;
        validate_unique(
            self.successions.iter().map(|record| &record.id),
            "institutional succession",
        )?;
        validate_unique(
            self.living_ledger.iter().map(|record| &record.id),
            "Living Ledger entry",
        )?;
        validate_unique(
            self.recipe_ledger.iter().map(|record| &record.id),
            "Recipe Ledger entry",
        )?;

        let subjects = self
            .subjects
            .iter()
            .map(|record| (&record.id, record))
            .collect::<BTreeMap<_, _>>();
        let capacities = self
            .capacities
            .iter()
            .map(|record| (&record.id, record))
            .collect::<BTreeMap<_, _>>();
        let diagnoses = self
            .diagnoses
            .iter()
            .map(|record| (&record.id, record))
            .collect::<BTreeMap<_, _>>();
        let consents = self
            .consents
            .iter()
            .map(|record| (&record.id, record))
            .collect::<BTreeMap<_, _>>();
        let clearances = self
            .clearances
            .iter()
            .map(|record| (&record.id, record))
            .collect::<BTreeMap<_, _>>();
        let privileges = self
            .privileges
            .iter()
            .map(|record| (&record.id, record))
            .collect::<BTreeMap<_, _>>();
        let materials = self
            .materials
            .iter()
            .map(|record| (&record.id, record))
            .collect::<BTreeMap<_, _>>();
        let recovery_plans = self
            .recovery_plans
            .iter()
            .map(|record| (&record.id, record))
            .collect::<BTreeMap<_, _>>();
        let reviews = self
            .reviews
            .iter()
            .map(|record| (&record.id, record))
            .collect::<BTreeMap<_, _>>();
        let obligations = self
            .recovery_obligations
            .iter()
            .map(|record| (&record.id, record))
            .collect::<BTreeMap<_, _>>();

        self.validate_clinical_ladder(&subjects)?;
        self.validate_accessions(&subjects)?;
        self.validate_ledgers(&subjects)?;

        for record in &self.diagnoses {
            require_subject(&subjects, &record.subject)?;
            require_subject(&subjects, &record.operator)?;
            require_text(&record.findings, "diagnostic findings")?;
            require_nonempty_texts(&record.evidence, "diagnostic evidence")?;
            if record.status == DiagnosisStatus::Provisional && !record.uncertainty_disclosed {
                return Err(GlaushouseValidationError::DiagnosticUncertaintyConcealed(
                    record.id.clone(),
                ));
            }
        }

        for record in &self.capacities {
            require_subject(&subjects, &record.subject)?;
            if record.expires_at <= record.assessed_at {
                return Err(GlaushouseValidationError::InvalidCapacityWindow(
                    record.id.clone(),
                ));
            }
        }

        for record in &self.consents {
            require_subject(&subjects, &record.subject)?;
            require_text(&record.procedure, "consented procedure")?;
            let capacity = capacities.get(&record.capacity).ok_or_else(|| {
                GlaushouseValidationError::MissingRecord("capacity", record.capacity.to_string())
            })?;
            if capacity.subject != record.subject || !capacity.has_capacity() {
                return Err(GlaushouseValidationError::InvalidConsentCapacity(
                    record.id.clone(),
                ));
            }
            if record.origin != ConsentOrigin::Explicit
                || !record.informed
                || !record.voluntary
                || !record.specific
                || !record.comprehensible
                || (!record.current && record.withdrawn_at.is_none())
                || !record.material_risks_disclosed
                || !record.alternatives_disclosed
                || !record.recovery_disclosed
            {
                return Err(GlaushouseValidationError::InvalidConsent(record.id.clone()));
            }
        }

        for record in &self.privileges {
            require_subject(&subjects, &record.operator)?;
            require_text(&record.facility, "privilege facility")?;
            require_nonempty_texts(&record.permitted_procedures, "permitted procedures")?;
        }

        for record in &self.recovery_plans {
            require_subject(&subjects, &record.subject)?;
            require_nonempty_texts(&record.stabilization, "recovery stabilization")?;
            require_nonempty_texts(&record.monitoring, "recovery monitoring")?;
            require_nonempty_texts(&record.discharge_conditions, "discharge conditions")?;
        }

        for record in &self.clearances {
            require_subject(&subjects, &record.subject)?;
            require_subject(&subjects, &record.operator)?;
            require_text(&record.procedure, "clearance procedure")?;
            require_text(&record.facility, "clearance facility")?;
            require_text(&record.scope, "clearance scope")?;
            require_text(&record.emergency_plan, "clearance emergency plan")?;
            require_nonempty_texts(&record.stopping_conditions, "stopping conditions")?;
            if record.expires_at <= record.issued_at {
                return Err(GlaushouseValidationError::InvalidClearanceWindow(
                    record.id.clone(),
                ));
            }
            let diagnosis = diagnoses.get(&record.diagnosis).ok_or_else(|| {
                GlaushouseValidationError::MissingRecord("diagnosis", record.diagnosis.to_string())
            })?;
            let consent = consents.get(&record.consent).ok_or_else(|| {
                GlaushouseValidationError::MissingRecord("consent", record.consent.to_string())
            })?;
            let capacity = capacities.get(&record.capacity).ok_or_else(|| {
                GlaushouseValidationError::MissingRecord("capacity", record.capacity.to_string())
            })?;
            if diagnosis.subject != record.subject
                || consent.subject != record.subject
                || capacity.subject != record.subject
                || consent.procedure != record.procedure
            {
                return Err(GlaushouseValidationError::ClearanceMismatch(
                    record.id.clone(),
                ));
            }
            if capacity.assessed_at > record.issued_at || capacity.expires_at < record.issued_at {
                return Err(GlaushouseValidationError::ClearanceMismatch(
                    record.id.clone(),
                ));
            }
            if record.class == ClearanceClass::Synthesis && consent.scope != ConsentScope::Synthesis
            {
                return Err(GlaushouseValidationError::ClearanceMismatch(
                    record.id.clone(),
                ));
            }
            if matches!(
                record.class,
                ClearanceClass::HighRisk | ClearanceClass::Experimental | ClearanceClass::Synthesis
            ) && record.recovery_plan.is_none()
            {
                return Err(GlaushouseValidationError::RecoveryPlanRequired(
                    record.id.clone(),
                ));
            }
            if let Some(plan) = &record.recovery_plan {
                let plan = recovery_plans.get(plan).ok_or_else(|| {
                    GlaushouseValidationError::MissingRecord("recovery plan", plan.to_string())
                })?;
                if plan.subject != record.subject {
                    return Err(GlaushouseValidationError::ClearanceMismatch(
                        record.id.clone(),
                    ));
                }
            }
        }

        for record in &self.materials {
            require_subject(&subjects, &record.material)?;
            require_subject(&subjects, &record.source)?;
            require_nonempty_texts(&record.provenance, "material provenance")?;
            require_nonempty_texts(&record.custody_chain, "material custody")?;
            if !record.lawfully_obtained || record.illegally_hollowed {
                return Err(GlaushouseValidationError::UnlawfulMaterial(
                    record.id.clone(),
                ));
            }
        }

        for record in &self.syntheses {
            self.validate_synthesis(
                record,
                &subjects,
                &consents,
                &clearances,
                &privileges,
                &materials,
                &recovery_plans,
                &reviews,
            )?;
        }

        for record in &self.reviews {
            require_subject(&subjects, &record.subject)?;
            require_text(&record.scope, "clinical review scope")?;
        }
        for record in &self.nightingale_stops {
            require_subject(&subjects, &record.subject)?;
            require_subject(&subjects, &record.issued_by)?;
            require_text(&record.reason, "Nightingale stop reason")?;
            let review = record.mandatory_review.as_ref().ok_or_else(|| {
                GlaushouseValidationError::NightingaleStopWithoutReview(record.id.clone())
            })?;
            let review = reviews.get(review).ok_or_else(|| {
                GlaushouseValidationError::MissingRecord("clinical review", review.to_string())
            })?;
            if review.subject != record.subject {
                return Err(GlaushouseValidationError::NightingaleStopWithoutReview(
                    record.id.clone(),
                ));
            }
        }

        for record in &self.custody {
            require_subject(&subjects, &record.subject)?;
            require_subject(&subjects, &record.custodian)?;
            require_text(&record.purpose, "clinical custody purpose")?;
            if record.ownership_claimed {
                return Err(GlaushouseValidationError::CustodyClaimedAsOwnership(
                    record.id.clone(),
                ));
            }
        }

        for record in &self.recovery_obligations {
            require_subject(&subjects, &record.subject)?;
            require_text(&record.duty, "recovery obligation")?;
        }
        for record in &self.successions {
            let predecessor = record
                .predecessor_obligations
                .iter()
                .collect::<BTreeSet<_>>();
            let successor = record.successor_obligations.iter().collect::<BTreeSet<_>>();
            if predecessor != successor {
                return Err(GlaushouseValidationError::RecoveryObligationsLost(
                    record.id.clone(),
                ));
            }
            for obligation in successor {
                if !obligations.contains_key(obligation) {
                    return Err(GlaushouseValidationError::MissingRecord(
                        "recovery obligation",
                        obligation.to_string(),
                    ));
                }
            }
        }
        Ok(())
    }

    fn validate_clinical_ladder(
        &self,
        subjects: &BTreeMap<&IdentityId, &ClinicalSubjectRecord>,
    ) -> Result<(), GlaushouseValidationError> {
        let tokens = self
            .advancement_tokens
            .iter()
            .map(|record| (&record.id, record))
            .collect::<BTreeMap<_, _>>();
        let standings = self
            .clinical_standings
            .iter()
            .map(|record| (&record.clinician, record))
            .collect::<BTreeMap<_, _>>();
        let mut referenced_tokens = BTreeSet::new();

        for standing in &self.clinical_standings {
            require_subject(subjects, &standing.clinician)?;
            if !standing.has_rank(ClinicalRank::Nightingale) {
                return Err(GlaushouseValidationError::MissingNightingaleFoundation(
                    standing.clinician.clone(),
                ));
            }
            if standing.has_rank(ClinicalRank::Persephone) && !standing.is_qualified_persephone() {
                return Err(GlaushouseValidationError::UnbalancedPersephone(
                    standing.clinician.clone(),
                ));
            }
            if matches!(
                standing.active_branch_emphasis,
                Some(ClinicalBranch::Matron)
            ) && !standing.has_rank(ClinicalRank::Matron)
                || matches!(
                    standing.active_branch_emphasis,
                    Some(ClinicalBranch::Marshal)
                ) && !standing.has_rank(ClinicalRank::Marshal)
            {
                return Err(GlaushouseValidationError::InvalidBranchEmphasis(
                    standing.clinician.clone(),
                ));
            }

            let mut token_ranks = BTreeSet::new();
            for token_id in &standing.advancement_tokens {
                if !referenced_tokens.insert(token_id) {
                    return Err(GlaushouseValidationError::AdvancementTokenReused(
                        token_id.clone(),
                    ));
                }
                let token = tokens.get(token_id).ok_or_else(|| {
                    GlaushouseValidationError::MissingRecord(
                        "advancement token",
                        token_id.to_string(),
                    )
                })?;
                if token.clinician != standing.clinician
                    || !standing.has_rank(token.recognized_rank)
                    || !token_ranks.insert(token.recognized_rank)
                {
                    return Err(GlaushouseValidationError::InvalidAdvancementToken(
                        token.id.clone(),
                    ));
                }
                require_text(&token.objective_benefit, "advancement objective benefit")?;
                require_nonempty_texts(
                    &token.clinical_experience,
                    "advancement clinical experience",
                )?;
                require_nonempty_texts(&token.authority_granted, "advancement authority granted")?;
                require_nonempty_texts(&token.education_access, "advancement education access")?;
                require_nonempty_texts(
                    &token.patient_responsibility,
                    "advancement patient responsibility",
                )?;
                let required = required_advancement_evidence(token.recognized_rank);
                if !required.is_subset(&token.evidence) {
                    return Err(GlaushouseValidationError::InvalidAdvancementToken(
                        token.id.clone(),
                    ));
                }
            }
            if token_ranks != standing.earned_ranks {
                return Err(GlaushouseValidationError::UnrecordedClinicalRank(
                    standing.clinician.clone(),
                ));
            }
        }
        if referenced_tokens.len() != self.advancement_tokens.len() {
            return Err(GlaushouseValidationError::OrphanedAdvancementToken);
        }

        let required_candidacy = required_prima_donna_evidence();
        let required_sources = required_prima_donna_succession_sources();
        for candidacy in &self.prima_donna_candidacies {
            require_subject(subjects, &candidacy.candidate)?;
            let standing = standings.get(&candidacy.candidate).ok_or_else(|| {
                GlaushouseValidationError::UnqualifiedPrimaDonnaCandidate(
                    candidacy.candidate.clone(),
                )
            })?;
            if !candidacy.eligible
                || !standing.is_qualified_persephone()
                || !required_candidacy.is_subset(&candidacy.evidence)
                || !required_sources.is_subset(&candidacy.succession_sources)
                || candidacy.testimony.is_empty()
                || candidacy
                    .testimony
                    .iter()
                    .any(|entry| entry.trim().is_empty())
            {
                return Err(GlaushouseValidationError::UnqualifiedPrimaDonnaCandidate(
                    candidacy.candidate.clone(),
                ));
            }
        }
        Ok(())
    }

    fn validate_ledgers(
        &self,
        subjects: &BTreeMap<&IdentityId, &ClinicalSubjectRecord>,
    ) -> Result<(), GlaushouseValidationError> {
        let standings = self
            .clinical_standings
            .iter()
            .map(|record| (&record.clinician, record))
            .collect::<BTreeMap<_, _>>();
        let living_ids = self
            .living_ledger
            .iter()
            .map(|entry| &entry.id)
            .collect::<BTreeSet<_>>();
        if self
            .recipe_ledger
            .iter()
            .any(|entry| living_ids.contains(&entry.id))
        {
            return Err(GlaushouseValidationError::LedgerLayersCollapsed);
        }
        for entry in &self.living_ledger {
            require_subject(subjects, &entry.subject)?;
            if entry.overseen_by_persephones.is_empty() {
                return Err(GlaushouseValidationError::LivingLedgerWithoutPersephone(
                    entry.id.clone(),
                ));
            }
            for overseer in &entry.overseen_by_persephones {
                let standing = standings.get(overseer).ok_or_else(|| {
                    GlaushouseValidationError::LivingLedgerWithoutPersephone(entry.id.clone())
                })?;
                if !standing.is_qualified_persephone() {
                    return Err(GlaushouseValidationError::LivingLedgerWithoutPersephone(
                        entry.id.clone(),
                    ));
                }
            }
            for evidence in [
                &entry.matron.cognition,
                &entry.matron.perception,
                &entry.matron.consent,
                &entry.matron.emotion,
                &entry.matron.identity,
                &entry.matron.aura_coherence,
                &entry.matron.patient_testimony,
                &entry.matron.lived_adaptation,
                &entry.marshal.current_flow,
                &entry.marshal.structural_condition,
                &entry.marshal.graft_integrity,
                &entry.marshal.mobility,
                &entry.marshal.physical_tolerance,
                &entry.marshal.containment,
                &entry.marshal.bodily_adaptation,
            ] {
                require_text(evidence, "Living Ledger evidence")?;
            }
        }
        for entry in &self.recipe_ledger {
            require_subject(subjects, &entry.authorized_by_prima_donna)?;
            require_text(&entry.recipe.name, "Recipe Ledger recipe")?;
            require_text(&entry.recipe.revision, "Recipe Ledger revision")?;
            require_nonempty_texts(&entry.architecture, "Recipe Ledger architecture")?;
            if !self.accessions.iter().any(|accession| {
                accession.holder == entry.authorized_by_prima_donna
                    && accession.office == ClinicalOffice::PrimaDonna
            }) {
                return Err(GlaushouseValidationError::RecipeLedgerWithoutPrimaDonna(
                    entry.id.clone(),
                ));
            }
        }
        Ok(())
    }

    fn validate_accessions(
        &self,
        subjects: &BTreeMap<&IdentityId, &ClinicalSubjectRecord>,
    ) -> Result<(), GlaushouseValidationError> {
        let active_prima_donnas = self
            .accessions
            .iter()
            .filter(|record| record.active && record.office == ClinicalOffice::PrimaDonna)
            .count();
        if active_prima_donnas > 1 {
            return Err(GlaushouseValidationError::ActivePrimaDonnaCount(
                active_prima_donnas,
            ));
        }
        for record in &self.accessions {
            let holder = require_subject(subjects, &record.holder)?;
            require_nonempty_texts(&record.evidence, "accession evidence")?;
            let candidacy = self
                .prima_donna_candidacies
                .iter()
                .find(|candidacy| candidacy.id == record.candidacy)
                .ok_or_else(|| {
                    GlaushouseValidationError::MissingRecord(
                        "Prima Donna candidacy",
                        record.candidacy.to_string(),
                    )
                })?;
            if record.active
                && (record.tombstoned
                    || holder.tombstoned
                    || record.origin != AuthorityOrigin::ClinicalAccession
                    || !record.clinical_competence_reviewed
                    || !record.stonebend_title_recorded
                    || !record.flynt_recognition_recorded
                    || candidacy.candidate != record.holder
                    || !candidacy.eligible
                    || !record.advancement_paths_open)
            {
                return Err(GlaushouseValidationError::InvalidAccession(
                    record.id.clone(),
                ));
            }
            if record.active
                && (!record.nightingale_testimony_recorded || !record.persephone_review_completed)
            {
                return Err(GlaushouseValidationError::InvalidAccession(
                    record.id.clone(),
                ));
            }
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn validate_synthesis(
        &self,
        record: &SynthesisRecord,
        subjects: &BTreeMap<&IdentityId, &ClinicalSubjectRecord>,
        consents: &BTreeMap<&ConsentRecordId, &ConsentRecord>,
        clearances: &BTreeMap<&ClearanceRecordId, &ClearanceRecord>,
        privileges: &BTreeMap<&PrivilegeRecordId, &OperatorPrivilegeRecord>,
        materials: &BTreeMap<&MaterialRecordId, &SynthesisMaterialRecord>,
        recovery_plans: &BTreeMap<&RecoveryPlanId, &RecoveryPlan>,
        reviews: &BTreeMap<&ClinicalReviewId, &ClinicalReviewRecord>,
    ) -> Result<(), GlaushouseValidationError> {
        let subject = require_subject(subjects, &record.subject)?;
        if subject.tombstoned {
            return Err(GlaushouseValidationError::TombstonedSubjectActive(
                record.subject.clone(),
            ));
        }
        require_text(&record.recipe, "Synthesis recipe")?;
        require_text(&record.intended_result, "intended Synthesis result")?;
        if !record.depth.is_intended_clinical_depth() {
            return Err(GlaushouseValidationError::OvergrowthTreatedAsIntendedForm(
                record.id.clone(),
            ));
        }
        record.lifecycle.validate()?;
        require_text(&record.continuance.recipe.name, "Continuance Recipe")?;
        require_text(
            &record.continuance.recipe.revision,
            "Continuance Recipe revision",
        )?;
        require_nonempty_texts(&record.continuance.maintenance, "Continuance maintenance")?;
        require_nonempty_texts(&record.continuance.renewal, "Continuance renewal")?;
        require_nonempty_texts(
            &record.continuance.environmental_conditions,
            "Continuance environment",
        )?;
        if record.continuance.recipe.name != record.recipe
            || record.continuance.ways.is_empty()
            || record.continuance.institutional_care.is_empty()
            || record.continuance.ways.iter().any(|way| {
                way.name.trim().is_empty()
                    || way.practices.is_empty()
                    || way
                        .practices
                        .iter()
                        .any(|practice| practice.trim().is_empty())
            })
        {
            return Err(GlaushouseValidationError::InvalidContinuancePlan(
                record.id.clone(),
            ));
        }
        if record
            .lifecycle
            .current()
            .is_some_and(SynthesisLifecycleState::is_coherently_active)
            && !record.continuance.conditions.fully_satisfied()
        {
            return Err(GlaushouseValidationError::UnmaintainedContinuance(
                record.id.clone(),
            ));
        }
        if record.depth.requires_major_viability()
            && (!record
                .technical_viability
                .as_ref()
                .is_some_and(TechnicalViability::is_viable)
                || !record
                    .lived_viability
                    .as_ref()
                    .is_some_and(LivedViability::is_viable))
        {
            return Err(GlaushouseValidationError::MajorSynthesisWithoutViability(
                record.id.clone(),
            ));
        }
        match record.actual_outcome {
            SynthesisOutcome::HostRejection
                if !matches!(record.rejection, Some(RejectionRecord::Host { .. })) =>
            {
                return Err(GlaushouseValidationError::RejectionDirectionMismatch(
                    record.id.clone(),
                ));
            }
            SynthesisOutcome::SympioteRejection
                if !matches!(record.rejection, Some(RejectionRecord::Sympiote { .. })) =>
            {
                return Err(GlaushouseValidationError::RejectionDirectionMismatch(
                    record.id.clone(),
                ));
            }
            SynthesisOutcome::SafeRegression
                if !matches!(
                    record.lifecycle.current(),
                    Some(
                        SynthesisLifecycleState::Regressed
                            | SynthesisLifecycleState::SafelyDiscontinued
                    )
                ) =>
            {
                return Err(GlaushouseValidationError::RegressionCollapseMismatch(
                    record.id.clone(),
                ));
            }
            SynthesisOutcome::CatastrophicCollapse
                if record.lifecycle.current()
                    != Some(SynthesisLifecycleState::CatastrophicallyCollapsed) =>
            {
                return Err(GlaushouseValidationError::RegressionCollapseMismatch(
                    record.id.clone(),
                ));
            }
            SynthesisOutcome::Overgrowth
                if matches!(
                    record.lifecycle.current(),
                    Some(SynthesisLifecycleState::Maintained | SynthesisLifecycleState::Renewed)
                ) =>
            {
                return Err(GlaushouseValidationError::OvergrowthTreatedAsIntendedForm(
                    record.id.clone(),
                ));
            }
            _ => {}
        }
        let consent = consents.get(&record.consent).ok_or_else(|| {
            GlaushouseValidationError::MissingRecord("consent", record.consent.to_string())
        })?;
        if consent.subject != record.subject
            || consent.scope != ConsentScope::Synthesis
            || consent.withdrawn_at.is_some_and(|withdrawn| {
                withdrawn <= record.irreversible_threshold_at.unwrap_or(u64::MAX)
            })
        {
            return Err(GlaushouseValidationError::SynthesisWithoutConsent(
                record.id.clone(),
            ));
        }
        let clearance = clearances.get(&record.clearance).ok_or_else(|| {
            GlaushouseValidationError::MissingRecord("clearance", record.clearance.to_string())
        })?;
        if clearance.subject != record.subject
            || clearance.consent != record.consent
            || clearance.status != ClearanceStatus::Active
            || clearance.issued_at > record.started_at
            || clearance.expires_at <= record.started_at
        {
            return Err(GlaushouseValidationError::SynthesisWithoutClearance(
                record.id.clone(),
            ));
        }
        let privilege = privileges.get(&record.operator_privilege).ok_or_else(|| {
            GlaushouseValidationError::MissingRecord(
                "operator privilege",
                record.operator_privilege.to_string(),
            )
        })?;
        if privilege.operator != clearance.operator
            || privilege.status != PrivilegeStatus::Active
            || privilege.valid_until < record.started_at
            || privilege.facility != clearance.facility
            || !privilege
                .permitted_procedures
                .iter()
                .any(|procedure| procedure == &clearance.procedure)
        {
            return Err(GlaushouseValidationError::UnprivilegedOperator(
                record.id.clone(),
            ));
        }
        for material in &record.materials {
            let material = materials.get(material).ok_or_else(|| {
                GlaushouseValidationError::MissingRecord("material", material.to_string())
            })?;
            if !material.lawfully_obtained || material.illegally_hollowed {
                return Err(GlaushouseValidationError::UnlawfulMaterial(
                    material.id.clone(),
                ));
            }
        }
        let high_risk = matches!(
            clearance.class,
            ClearanceClass::HighRisk | ClearanceClass::Experimental | ClearanceClass::Synthesis
        ) || matches!(
            record.class,
            SynthesisClass::Experimental
                | SynthesisClass::Emergency
                | SynthesisClass::Composite
                | SynthesisClass::Irreversible
        ) || record.depth.requires_major_viability()
            || matches!(
                record.actual_outcome,
                SynthesisOutcome::Overgrowth | SynthesisOutcome::CatastrophicCollapse
            );
        if high_risk && record.recovery_plan.is_none() {
            return Err(GlaushouseValidationError::SynthesisWithoutRecovery(
                record.id.clone(),
            ));
        }
        if let Some(plan) = &record.recovery_plan {
            let plan = recovery_plans.get(plan).ok_or_else(|| {
                GlaushouseValidationError::MissingRecord("recovery plan", plan.to_string())
            })?;
            if plan.subject != record.subject || clearance.recovery_plan.as_ref() != Some(&plan.id)
            {
                return Err(GlaushouseValidationError::SynthesisWithoutRecovery(
                    record.id.clone(),
                ));
            }
        }
        if high_risk {
            let witness = record.nightingale_witness.as_ref().ok_or_else(|| {
                GlaushouseValidationError::MissingNightingaleWitness(record.id.clone())
            })?;
            require_subject(subjects, witness)?;
        }
        if record.class == SynthesisClass::Experimental
            && (!record.experimental_marked
                || clearance.class != ClearanceClass::Experimental
                || !consent.experimental_status_disclosed)
        {
            return Err(GlaushouseValidationError::ExperimentalStatusConcealed(
                record.id.clone(),
            ));
        }
        if record.class == SynthesisClass::Emergency {
            let review = record.emergency_post_event_review.as_ref().ok_or_else(|| {
                GlaushouseValidationError::EmergencySynthesisWithoutReview(record.id.clone())
            })?;
            let review = reviews.get(review).ok_or_else(|| {
                GlaushouseValidationError::MissingRecord("clinical review", review.to_string())
            })?;
            if review.subject != record.subject {
                return Err(GlaushouseValidationError::EmergencySynthesisWithoutReview(
                    record.id.clone(),
                ));
            }
        }
        if record.actual_outcome != record.recorded_outcome {
            return Err(GlaushouseValidationError::FalseSynthesisOutcome(
                record.id.clone(),
            ));
        }
        if !record.prior_identity_history_preserved || subject.prior_identity_history.is_empty() {
            return Err(GlaushouseValidationError::IdentityHistoryErased(
                record.id.clone(),
            ));
        }
        if record.resulting_title_or_office {
            return Err(GlaushouseValidationError::TransformationCreatedAuthority(
                record.id.clone(),
            ));
        }
        if !record.recovery_status_recorded
            || (record.actual_outcome == SynthesisOutcome::Successful && !record.stabilized)
        {
            return Err(GlaushouseValidationError::SynthesisIncomplete(
                record.id.clone(),
            ));
        }
        Ok(())
    }
}

fn required_advancement_evidence(rank: ClinicalRank) -> BTreeSet<AdvancementEvidenceKind> {
    use AdvancementEvidenceKind as Evidence;
    match rank {
        ClinicalRank::Nightingale => BTreeSet::from([Evidence::NightingaleCare]),
        ClinicalRank::Matron => BTreeSet::from([
            Evidence::NightingaleCare,
            Evidence::AuraSensitiveObservation,
            Evidence::IdentityContinuity,
            Evidence::MaintenanceAndRenewal,
            Evidence::RecoveryAndRehabilitation,
        ]),
        ClinicalRank::Marshal => BTreeSet::from([
            Evidence::NightingaleCare,
            Evidence::CurrentSensitiveStabilization,
            Evidence::BodilyContinuity,
            Evidence::MaintenanceAndRenewal,
            Evidence::RecoveryAndRehabilitation,
        ]),
        ClinicalRank::Persephone => BTreeSet::from([
            Evidence::NightingaleCare,
            Evidence::AuraSensitiveObservation,
            Evidence::CurrentSensitiveStabilization,
            Evidence::IdentityContinuity,
            Evidence::BodilyContinuity,
            Evidence::MaintenanceAndRenewal,
            Evidence::RecoveryAndRehabilitation,
            Evidence::SafeRegression,
            Evidence::DischargeJudgment,
            Evidence::ReconciledEvidence,
        ]),
    }
}

fn required_prima_donna_evidence() -> BTreeSet<PrimaDonnaEvidenceKind> {
    use PrimaDonnaEvidenceKind as Evidence;
    BTreeSet::from([
        Evidence::NightingaleFoundation,
        Evidence::MatronMastery,
        Evidence::MarshalMastery,
        Evidence::PersephoneService,
        Evidence::PhysicianMastery,
        Evidence::PatientOutcomes,
        Evidence::TechnicalViabilityJudgment,
        Evidence::LivedViabilityJudgment,
        Evidence::DiagnosticMastery,
        Evidence::RecipeAuthorshipOrRevision,
        Evidence::SurgicalAuthority,
        Evidence::SynthesisLedgerStewardship,
        Evidence::TeachingAndCultivation,
        Evidence::AdvancementPathsRemainOpen,
    ])
}

fn required_prima_donna_succession_sources() -> BTreeSet<PrimaDonnaSuccessionSource> {
    use PrimaDonnaSuccessionSource as Source;
    BTreeSet::from([
        Source::NightingaleTestimony,
        Source::MatronRecordOrTestimony,
        Source::MarshalRecordOrTestimony,
        Source::PersephoneRecordOrTestimony,
        Source::TreatedHuemanTestimony,
        Source::LivingLedger,
        Source::RecipeLedger,
        Source::ClinicalOutcomes,
        Source::TeachingHistory,
    ])
}

fn validate_unique<'a, T: Ord + fmt::Display + 'a>(
    values: impl Iterator<Item = &'a T>,
    kind: &'static str,
) -> Result<(), GlaushouseValidationError> {
    let mut seen = BTreeSet::new();
    for value in values {
        if !seen.insert(value) {
            return Err(GlaushouseValidationError::DuplicateRecord(
                kind,
                value.to_string(),
            ));
        }
    }
    Ok(())
}

fn require_subject<'a>(
    subjects: &'a BTreeMap<&IdentityId, &'a ClinicalSubjectRecord>,
    subject: &IdentityId,
) -> Result<&'a ClinicalSubjectRecord, GlaushouseValidationError> {
    let record = subjects
        .get(subject)
        .copied()
        .ok_or_else(|| GlaushouseValidationError::MissingRecord("subject", subject.to_string()))?;
    Ok(record)
}

fn require_text(value: &str, field: &'static str) -> Result<(), GlaushouseValidationError> {
    if value.trim().is_empty() {
        Err(GlaushouseValidationError::MissingRequiredText(field))
    } else {
        Ok(())
    }
}

fn require_nonempty_texts(
    values: &[String],
    field: &'static str,
) -> Result<(), GlaushouseValidationError> {
    if values.is_empty() || values.iter().any(|value| value.trim().is_empty()) {
        Err(GlaushouseValidationError::MissingRequiredText(field))
    } else {
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GlaushouseValidationError {
    DuplicatePrincipalAuthority,
    InvalidPrincipalAuthorityPlacement,
    UnequalClinicalBranches,
    CompletedSelectionCannotAdvance,
    InvalidSelectionTransition {
        from: PrimaDonnaSelectionPhase,
        expected: PrimaDonnaSelectionPhase,
        attempted: PrimaDonnaSelectionPhase,
    },
    IncompleteSelection(PrimaDonnaSelectionPhase),
    DuplicateRecord(&'static str, String),
    MissingRecord(&'static str, String),
    MissingRequiredText(&'static str),
    ActivePrimaDonnaCount(usize),
    MissingNightingaleFoundation(IdentityId),
    UnbalancedPersephone(IdentityId),
    InvalidBranchEmphasis(IdentityId),
    AdvancementTokenReused(AdvancementTokenId),
    InvalidAdvancementToken(AdvancementTokenId),
    UnrecordedClinicalRank(IdentityId),
    OrphanedAdvancementToken,
    UnqualifiedPrimaDonnaCandidate(IdentityId),
    InvalidAccession(AccessionRecordId),
    TombstonedSubjectActive(IdentityId),
    DiagnosticUncertaintyConcealed(DiagnosisRecordId),
    InvalidCapacityWindow(CapacityRecordId),
    InvalidConsentCapacity(ConsentRecordId),
    InvalidConsent(ConsentRecordId),
    InvalidClearanceWindow(ClearanceRecordId),
    ClearanceMismatch(ClearanceRecordId),
    RecoveryPlanRequired(ClearanceRecordId),
    UnlawfulMaterial(MaterialRecordId),
    SynthesisWithoutConsent(SynthesisRecordId),
    SynthesisWithoutClearance(SynthesisRecordId),
    UnprivilegedOperator(SynthesisRecordId),
    SynthesisWithoutRecovery(SynthesisRecordId),
    InvalidSynthesisLifecycle,
    InvalidContinuancePlan(SynthesisRecordId),
    UnmaintainedContinuance(SynthesisRecordId),
    OvergrowthTreatedAsIntendedForm(SynthesisRecordId),
    MajorSynthesisWithoutViability(SynthesisRecordId),
    RejectionDirectionMismatch(SynthesisRecordId),
    RegressionCollapseMismatch(SynthesisRecordId),
    MissingNightingaleWitness(SynthesisRecordId),
    ExperimentalStatusConcealed(SynthesisRecordId),
    EmergencySynthesisWithoutReview(SynthesisRecordId),
    FalseSynthesisOutcome(SynthesisRecordId),
    IdentityHistoryErased(SynthesisRecordId),
    TransformationCreatedAuthority(SynthesisRecordId),
    SynthesisIncomplete(SynthesisRecordId),
    NightingaleStopWithoutReview(NightingaleStopId),
    CustodyClaimedAsOwnership(CustodyRecordId),
    RecoveryObligationsLost(InstitutionalSuccessionId),
    LedgerLayersCollapsed,
    LivingLedgerWithoutPersephone(LedgerEntryId),
    RecipeLedgerWithoutPrimaDonna(LedgerEntryId),
}

impl fmt::Display for GlaushouseValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "Glaüshouse constitutional validation failed: {self:?}"
        )
    }
}

impl std::error::Error for GlaushouseValidationError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn identity(value: &str) -> IdentityId {
        IdentityId::new(value).expect("test identity")
    }

    fn accession(
        id: &str,
        office: ClinicalOffice,
        holder: &str,
        candidacy: &str,
    ) -> AccessionRecord {
        AccessionRecord {
            id: AccessionRecordId::new(id).unwrap(),
            office,
            holder: identity(holder),
            active: true,
            tombstoned: false,
            origin: AuthorityOrigin::ClinicalAccession,
            clinical_competence_reviewed: true,
            nightingale_testimony_recorded: true,
            persephone_review_completed: true,
            flynt_recognition_recorded: true,
            stonebend_title_recorded: true,
            evidence: vec!["sealed accession record".into()],
            candidacy: PrimaDonnaCandidacyId::new(candidacy).unwrap(),
            advancement_paths_open: true,
        }
    }

    fn qualified_standing(
        holder: &IdentityId,
        prefix: &str,
    ) -> (ClinicalStanding, Vec<AdvancementToken>) {
        let ranks = [
            ClinicalRank::Nightingale,
            ClinicalRank::Matron,
            ClinicalRank::Marshal,
            ClinicalRank::Persephone,
        ];
        let tokens = ranks
            .into_iter()
            .map(|rank| AdvancementToken {
                id: AdvancementTokenId::new(format!("token.{prefix}.{rank:?}").to_lowercase())
                    .unwrap(),
                clinician: holder.clone(),
                recognized_rank: rank,
                evidence: required_advancement_evidence(rank),
                clinical_experience: vec![format!("{rank:?} clinical experience")],
                authority_granted: vec![format!("{rank:?} clinical authority")],
                education_access: vec![format!("{rank:?} advanced education access")],
                patient_responsibility: vec![format!("{rank:?} patient responsibility")],
                objective_benefit: format!("{rank:?} care improved a patient outcome"),
            })
            .collect::<Vec<_>>();
        (
            ClinicalStanding {
                clinician: holder.clone(),
                earned_ranks: ranks.into_iter().collect(),
                active_branch_emphasis: None,
                advancement_tokens: tokens.iter().map(|token| token.id.clone()).collect(),
            },
            tokens,
        )
    }

    fn authority_registry() -> GlaushouseRegistry {
        let doctor = doctor_ratchet_identity_id();
        let nurse = nurse_house_identity_id();
        let (doctor_standing, mut doctor_tokens) =
            qualified_standing(&doctor, "glaushouse.doctor-ratchet");
        let (nurse_standing, nurse_tokens) = qualified_standing(&nurse, "glaushouse.nurse-house");
        doctor_tokens.extend(nurse_tokens);
        let candidacy_id =
            PrimaDonnaCandidacyId::new("candidacy.glaushouse.doctor-ratchet").unwrap();
        GlaushouseRegistry {
            subjects: vec![
                ClinicalSubjectRecord {
                    id: doctor.clone(),
                    prior_identity_history: vec!["Doctor Ratchet".into()],
                    tombstoned: false,
                },
                ClinicalSubjectRecord {
                    id: nurse,
                    prior_identity_history: vec!["Nurse House".into()],
                    tombstoned: false,
                },
            ],
            clinical_standings: vec![doctor_standing, nurse_standing],
            advancement_tokens: doctor_tokens,
            prima_donna_candidacies: vec![PrimaDonnaCandidacy {
                id: candidacy_id.clone(),
                candidate: doctor.clone(),
                evidence: required_prima_donna_evidence(),
                succession_sources: required_prima_donna_succession_sources(),
                testimony: vec!["Nightingale, patient, and peer testimony".into()],
                eligible: true,
            }],
            accessions: vec![accession(
                "accession.glaushouse.doctor-ratchet",
                ClinicalOffice::PrimaDonna,
                doctor.as_str(),
                candidacy_id.as_str(),
            )],
            ..GlaushouseRegistry::default()
        }
    }

    #[test]
    fn exact_authority_placements_are_valid() {
        validate_principal_authorities().unwrap();
        authority_registry().validate().unwrap();
        assert_eq!(
            authority_registry()
                .clinical_standings
                .iter()
                .filter(|standing| standing.is_qualified_persephone())
                .count(),
            2
        );
    }

    #[test]
    fn selection_requires_all_twelve_ordered_phases() {
        let mut process = PrimaDonnaSelectionProcess::open(doctor_ratchet_identity_id());
        while let Some(next) = process.phase().next() {
            process
                .advance(next, format!("evidence for {next:?}"))
                .unwrap();
        }
        process.require_complete().unwrap();
    }

    #[test]
    fn legacy_or_transformation_state_cannot_create_clinical_office() {
        for origin in [
            AuthorityOrigin::LegacyState,
            AuthorityOrigin::Transformation,
        ] {
            let mut registry = authority_registry();
            registry.accessions[0].origin = origin;
            assert!(matches!(
                registry.validate(),
                Err(GlaushouseValidationError::InvalidAccession(_))
            ));
        }
    }

    #[test]
    fn silence_custody_dependence_and_recognition_never_become_consent() {
        for origin in [
            ConsentOrigin::Silence,
            ConsentOrigin::Custody,
            ConsentOrigin::Dependence,
            ConsentOrigin::Recognition,
            ConsentOrigin::AuraInfluence,
        ] {
            assert_ne!(origin, ConsentOrigin::Explicit);
        }
    }

    #[test]
    fn consent_scopes_do_not_collapse() {
        assert_ne!(ConsentScope::Hollowing, ConsentScope::Synthesis);
        assert_ne!(ConsentScope::Treatment, ConsentScope::Synthesis);
        assert_ne!(ConsentScope::Synthesis, ConsentScope::Research);
    }
}
