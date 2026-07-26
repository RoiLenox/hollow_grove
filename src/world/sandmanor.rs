//! Executable Sandmanor constitutional law above the frozen Constitutional
//! Runtime V2.
//!
//! This module owns Sandmanor-specific design, method, evidence, proof,
//! reciprocal teaching, Contest of Improvement, education, revision, and
//! regional-role validation. It reuses the existing lineage contract and does
//! not alter the recursion kernel, grant Stonebend Title, grant Glaushouse
//! clearance, grant Flynt recognition, or execute Synthesis.

pub mod milestone;

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::institution::{IdentityId, InstitutionId, OfficeId};
use crate::lineage_contract::{SandmanorForm, SandmanorLineage, validate_sandmanor_transition};
use crate::world::hueman_faculties::PrefigEmbodimentStatus;

pub const SANDMANOR_CONSTITUTION_SOURCE: &str = "SANDMANOR_CONSTITUTION_V2.md";
pub const SANDMANOR_GOVERNING_VERB: &str = "Prove";
pub const SANDMANOR_SIGNATURE_OFFENSE: &str = "Fraudulent Design";

fn institution(value: &str) -> InstitutionId {
    InstitutionId::new(value).expect("canonical Sandmanor institution ID")
}

fn office(value: &str) -> OfficeId {
    OfficeId::new(value).expect("canonical Sandmanor office ID")
}

#[must_use]
pub fn proof_civilization_id() -> InstitutionId {
    institution("institution.sandmanor.sandmen")
}

#[must_use]
pub fn sandman_office_id() -> OfficeId {
    office("office.sandmanor.sandman")
}

macro_rules! sandmanor_id {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, SandmanorIdError> {
                let value = value.into();
                if value.is_empty()
                    || !value.bytes().all(|byte| {
                        byte.is_ascii_lowercase()
                            || byte.is_ascii_digit()
                            || byte == b'.'
                            || byte == b'-'
                    })
                {
                    return Err(SandmanorIdError(value));
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
pub struct SandmanorIdError(String);

impl fmt::Display for SandmanorIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "invalid Sandmanor stable identifier: {}", self.0)
    }
}

impl std::error::Error for SandmanorIdError {}

sandmanor_id!(DesignId);
sandmanor_id!(MethodId);
sandmanor_id!(ClaimId);
sandmanor_id!(EvidenceId);
sandmanor_id!(DemonstrationId);
sandmanor_id!(FailureId);
sandmanor_id!(ReproductionId);
sandmanor_id!(ProofJudgmentId);
sandmanor_id!(RecipeId);
sandmanor_id!(TeachingRecordId);
sandmanor_id!(ContestId);
sandmanor_id!(AccessionRecordId);
sandmanor_id!(AssessmentId);
sandmanor_id!(CredentialId);
sandmanor_id!(ApprenticeshipId);
sandmanor_id!(StandardId);
sandmanor_id!(CriticismId);
sandmanor_id!(EmergencyDesignId);
sandmanor_id!(RegionalProofId);
sandmanor_id!(KnowledgeObligationId);
sandmanor_id!(InstitutionalSuccessionId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CivicTradition {
    Minorian,
    Minoan,
}

impl CivicTradition {
    #[must_use]
    pub const fn people(self) -> &'static str {
        match self {
            Self::Minorian => "Gnomes",
            Self::Minoan => "Elves",
        }
    }

    #[must_use]
    pub const fn domain(self) -> DesignDomain {
        match self {
            Self::Minorian => DesignDomain::Interior,
            Self::Minoan => DesignDomain::Exterior,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DesignDomain {
    Interior,
    Exterior,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TraditionDefinition {
    pub tradition: CivicTradition,
    pub domain: DesignDomain,
    pub people: &'static str,
    pub governing_question: &'static str,
    pub equal_standing: bool,
}

pub const CIVIC_TRADITIONS: [TraditionDefinition; 2] = [
    TraditionDefinition {
        tradition: CivicTradition::Minorian,
        domain: DesignDomain::Interior,
        people: "Gnomes",
        governing_question: "How does this work from within?",
        equal_standing: true,
    },
    TraditionDefinition {
        tradition: CivicTradition::Minoan,
        domain: DesignDomain::Exterior,
        people: "Elves",
        governing_question: "How does this meet the world beyond itself?",
        equal_standing: true,
    },
];

pub fn validate_civic_traditions() -> Result<(), SandmanorValidationError> {
    let traditions = CIVIC_TRADITIONS
        .iter()
        .map(|entry| entry.tradition)
        .collect::<BTreeSet<_>>();
    let domains = CIVIC_TRADITIONS
        .iter()
        .map(|entry| entry.domain)
        .collect::<BTreeSet<_>>();
    if traditions.len() != 2
        || domains.len() != 2
        || CIVIC_TRADITIONS.iter().any(|entry| !entry.equal_standing)
    {
        return Err(SandmanorValidationError::TraditionEqualityViolated);
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DesignAuthor {
    Being(IdentityId),
    Institution(InstitutionId),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SandmanorSubjectRecord {
    pub id: IdentityId,
    pub intellectual_lineage: Vec<String>,
    pub tombstoned: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DesignStatus {
    Conceptual,
    Preliminary,
    Prototype,
    Experimental,
    Validated,
    Proven,
    Conditional,
    Regional,
    Interior,
    Exterior,
    Public,
    Institutional,
    Recipe,
    Emergency,
    Replacement,
    Successor,
    Deprecated,
    Tombstoned,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesignRecord {
    pub id: DesignId,
    pub author: DesignAuthor,
    pub purpose: String,
    pub intended_users: Vec<String>,
    pub problem: String,
    pub assumptions: Vec<String>,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub dependencies: Vec<String>,
    pub constraints: Vec<String>,
    pub method: MethodId,
    pub materials: Vec<String>,
    pub expected_result: String,
    pub known_risks: Vec<String>,
    pub failure_states: Vec<String>,
    pub maintenance: Vec<String>,
    pub alternatives: Vec<String>,
    pub measurement_plan: String,
    pub version: String,
    pub prior_version: Option<DesignId>,
    pub materially_revised: bool,
    pub inherits_prior_proof: bool,
    pub conflicts_of_interest: Vec<String>,
    pub status: DesignStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodRecord {
    pub id: MethodId,
    pub objective: String,
    pub sequence: Vec<String>,
    pub materials: Vec<String>,
    pub operators: Vec<DesignAuthor>,
    pub controls: Vec<String>,
    pub comparison_basis: String,
    pub measurements: Vec<String>,
    pub recording_procedure: String,
    pub stopping_conditions: Vec<String>,
    pub exclusion_rules: Vec<String>,
    pub uncertainty: Vec<String>,
    pub deviations: Vec<String>,
    pub analysis: String,
    pub changed_after_results_without_disclosure: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClaimRecord {
    pub id: ClaimId,
    pub design: DesignId,
    pub statement: String,
    pub scope: String,
    pub requires_independent_reproduction: bool,
    pub active: bool,
    pub tombstoned: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EvidenceClass {
    Direct,
    Indirect,
    Observational,
    Experimental,
    Comparative,
    Field,
    Testimonial,
    Historical,
    Simulated,
    Negative,
    Replication,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceRecord {
    pub id: EvidenceId,
    pub claim: ClaimId,
    pub class: EvidenceClass,
    pub source: String,
    pub collector: DesignAuthor,
    pub collected_at: u64,
    pub method: MethodId,
    pub custody: Vec<String>,
    pub alterations: Vec<String>,
    pub content: String,
    pub fabricated: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DemonstrationRecord {
    pub id: DemonstrationId,
    pub claim: ClaimId,
    pub method: MethodId,
    pub environment: String,
    pub operator: DesignAuthor,
    pub observers: Vec<DesignAuthor>,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub deviations: Vec<String>,
    pub measurements: Vec<String>,
    pub actual_result: String,
    pub complete_result_recorded: bool,
    pub prototype: bool,
    pub represented_as_completed_production: bool,
    pub simulated: bool,
    pub represented_as_direct_physical_performance: bool,
    pub failure: Option<FailureId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FailureRecord {
    pub id: FailureId,
    pub demonstration: DemonstrationId,
    pub intended_result: String,
    pub actual_result: String,
    pub conditions: Vec<String>,
    pub known_cause_or_uncertainty: String,
    pub harm: Vec<String>,
    pub corrective_action: Vec<String>,
    pub effect_on_proof: String,
    pub recommendations: Vec<String>,
    pub erased: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReproductionRecord {
    pub id: ReproductionId,
    pub claim: ClaimId,
    pub original_design_body: DesignAuthor,
    pub reproducing_body: DesignAuthor,
    pub evidence: Vec<EvidenceId>,
    pub independent: bool,
    pub result: String,
    pub successful: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProofStatus {
    Unexamined,
    Proposed,
    ProvisionallySupported,
    Demonstrated,
    ConditionallyProven,
    ProvenWithinScope,
    IndependentlyReproduced,
    FieldProven,
    Disputed,
    Weakened,
    Superseded,
    Disproven,
    Deprecated,
    Tombstoned,
}

impl ProofStatus {
    #[must_use]
    pub const fn supports_reliance(self) -> bool {
        matches!(
            self,
            Self::ConditionallyProven
                | Self::ProvenWithinScope
                | Self::IndependentlyReproduced
                | Self::FieldProven
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofJudgmentRecord {
    pub id: ProofJudgmentId,
    pub claim: ClaimId,
    pub scope: String,
    pub status: ProofStatus,
    pub evidence: Vec<EvidenceId>,
    pub demonstrations: Vec<DemonstrationId>,
    pub reproductions: Vec<ReproductionId>,
    pub criticism_considered: Vec<CriticismId>,
    pub issued_by: OfficeId,
    pub active: bool,
    pub emergency_expires_at: Option<u64>,
    pub entered_ordinary_review: bool,
    pub grants_title: bool,
    pub grants_clearance: bool,
    pub grants_recognition: bool,
    pub prefig_source: Option<PrefigProofSourceRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrefigProofSourceRecord {
    pub recipe_id: String,
    pub status: PrefigEmbodimentStatus,
    pub evidence: Vec<EvidenceId>,
}

impl ProofJudgmentRecord {
    #[must_use]
    pub fn advances_prefig_through_existing_proof(&self) -> bool {
        self.prefig_source.is_some() && self.active && self.status.supports_reliance()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecipeRecord {
    pub id: RecipeId,
    pub name: String,
    pub version: String,
    pub author_or_lineage: Vec<DesignAuthor>,
    pub design: DesignId,
    pub purpose: String,
    pub inputs: Vec<String>,
    pub provenance_requirements: Vec<String>,
    pub conditions: Vec<String>,
    pub sequence: Vec<String>,
    pub operator_requirements: Vec<String>,
    pub expected_result: String,
    pub failure_states: Vec<String>,
    pub stopping_conditions: Vec<String>,
    pub revision_history: Vec<DesignId>,
    pub proof: ProofJudgmentId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeachingRecord {
    pub id: TeachingRecordId,
    pub teacher: IdentityId,
    pub learner: IdentityId,
    pub teacher_tradition: CivicTradition,
    pub learner_tradition: CivicTradition,
    pub practice: String,
    pub design_principle: String,
    pub method: String,
    pub observation: String,
    pub criticism: String,
    pub baseline_evidence: Vec<EvidenceId>,
    pub final_evidence: Vec<EvidenceId>,
    pub genuine: bool,
    pub comprehensible: bool,
    pub relevant: bool,
    pub non_sabotaging: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContestCandidate {
    pub being: IdentityId,
    pub tradition: CivicTradition,
    pub baseline_evidence: Vec<EvidenceId>,
    pub final_evidence: Vec<EvidenceId>,
    pub teaching_experience_demonstrated: bool,
    pub willingness_to_learn_demonstrated: bool,
    pub unresolved_fraudulent_design: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContestOutcome {
    Winner,
    InsufficientImprovement,
    Tied,
    VoidForFraud,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ContestReviewBody {
    pub minorian_reviewers: bool,
    pub minoan_reviewers: bool,
    pub teaching_representatives: bool,
    pub ordinary_affected_citizens: bool,
    pub evidence_stewards: bool,
    pub conflict_reviewers: bool,
}

impl ContestReviewBody {
    #[must_use]
    pub const fn complete(self) -> bool {
        self.minorian_reviewers
            && self.minoan_reviewers
            && self.teaching_representatives
            && self.ordinary_affected_citizens
            && self.evidence_stewards
            && self.conflict_reviewers
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContestOfImprovementRecord {
    pub id: ContestId,
    pub candidates: [ContestCandidate; 2],
    pub teaching: Vec<TeachingRecordId>,
    pub review_body: ContestReviewBody,
    pub audience_received_baselines: bool,
    pub audience_received_process: bool,
    pub audience_received_results: bool,
    pub audience_could_question_candidates: bool,
    pub conflicts_disclosed: bool,
    pub outcome: ContestOutcome,
    pub winner: Option<IdentityId>,
    pub joint_applied_design_challenge_completed: bool,
    pub challenges_resolved: bool,
    pub fraudulent: bool,
    pub complete: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SandmanAuthorityOrigin {
    ContestOfImprovement,
    Heredity,
    Combat,
    Popularity,
    Transformation,
    RecognitionAlone,
    LegacyProgression,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SandmanAccessionRecord {
    pub id: AccessionRecordId,
    pub holder: IdentityId,
    pub contest: ContestId,
    pub origin: SandmanAuthorityOrigin,
    pub stonebend_title_recorded: bool,
    pub flynt_recognition_recorded: bool,
    pub public_learning_statement: String,
    pub sealed: bool,
    pub active: bool,
    pub tombstoned: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssessmentRecord {
    pub id: AssessmentId,
    pub subject: IdentityId,
    pub criteria: Vec<String>,
    pub evidence: Vec<EvidenceId>,
    pub demonstrated: bool,
    pub hidden_criteria: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CredentialRecord {
    pub id: CredentialId,
    pub holder: IdentityId,
    pub assessment: AssessmentId,
    pub scope: String,
    pub active: bool,
    pub fraudulent: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApprenticeshipRecord {
    pub id: ApprenticeshipId,
    pub teacher: IdentityId,
    pub apprentice: IdentityId,
    pub field: String,
    pub duration: String,
    pub learning_goals: Vec<String>,
    pub teaching_obligations: Vec<String>,
    pub work_expectations: Vec<String>,
    pub support: Vec<String>,
    pub assessment: Option<AssessmentId>,
    pub complaint_process: String,
    pub labor_replaces_teaching: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StandardRecord {
    pub id: StandardId,
    pub purpose: String,
    pub scope: String,
    pub evidence: Vec<EvidenceId>,
    pub public_reviewed: bool,
    pub minorian_considered: bool,
    pub minoan_considered: bool,
    pub implementation_plan: String,
    pub transition_period: String,
    pub revision_process: String,
    pub appeal_process: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CriticismRecord {
    pub id: CriticismId,
    pub claim: ClaimId,
    pub critic: IdentityId,
    pub evidence: Vec<EvidenceId>,
    pub good_faith: bool,
    pub erased: bool,
    pub retaliation: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmergencyDesignRecord {
    pub id: EmergencyDesignId,
    pub design: DesignId,
    pub incomplete_proof_disclosed: bool,
    pub risks_disclosed: bool,
    pub urgency_reason: String,
    pub temporary_scope: String,
    pub monitoring: Vec<String>,
    pub stopping_conditions: Vec<String>,
    pub expires_at: u64,
    pub ordinary_review_started: bool,
    pub treated_as_permanent_proof: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RegionalProofDomain {
    AuraFields,
    AuraBeachAndCurrentSea,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RegionalFunction {
    AdvancedTendingAndFieldLabor,
    RoamAuraBeachAndGuardCurrentSea,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegionalSynthesisProofRecord {
    pub id: RegionalProofId,
    pub predecessor: SandmanorForm,
    pub result: SandmanorForm,
    pub domain: RegionalProofDomain,
    pub function: RegionalFunction,
    pub proof: ProofJudgmentId,
    pub grants_synthesis_clearance: bool,
    pub grants_title_or_office: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KnowledgeObligation {
    pub id: KnowledgeObligationId,
    pub responsible_institution: InstitutionId,
    pub duty: String,
    pub discharged: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstitutionalSuccessionRecord {
    pub id: InstitutionalSuccessionId,
    pub predecessor: InstitutionId,
    pub successor: InstitutionId,
    pub predecessor_obligations: Vec<KnowledgeObligationId>,
    pub successor_obligations: Vec<KnowledgeObligationId>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SandmanorRegistry {
    pub subjects: Vec<SandmanorSubjectRecord>,
    pub designs: Vec<DesignRecord>,
    pub methods: Vec<MethodRecord>,
    pub claims: Vec<ClaimRecord>,
    pub evidence: Vec<EvidenceRecord>,
    pub demonstrations: Vec<DemonstrationRecord>,
    pub failures: Vec<FailureRecord>,
    pub reproductions: Vec<ReproductionRecord>,
    pub criticisms: Vec<CriticismRecord>,
    pub proofs: Vec<ProofJudgmentRecord>,
    pub recipes: Vec<RecipeRecord>,
    pub teaching: Vec<TeachingRecord>,
    pub contests: Vec<ContestOfImprovementRecord>,
    pub accessions: Vec<SandmanAccessionRecord>,
    pub assessments: Vec<AssessmentRecord>,
    pub credentials: Vec<CredentialRecord>,
    pub apprenticeships: Vec<ApprenticeshipRecord>,
    pub standards: Vec<StandardRecord>,
    pub emergency_designs: Vec<EmergencyDesignRecord>,
    pub regional_proofs: Vec<RegionalSynthesisProofRecord>,
    pub knowledge_obligations: Vec<KnowledgeObligation>,
    pub successions: Vec<InstitutionalSuccessionRecord>,
}

impl SandmanorRegistry {
    pub fn validate(&self) -> Result<(), SandmanorValidationError> {
        validate_civic_traditions()?;
        validate_unique(self.subjects.iter().map(|record| &record.id), "subject")?;
        validate_unique(self.designs.iter().map(|record| &record.id), "design")?;
        validate_unique(self.methods.iter().map(|record| &record.id), "method")?;
        validate_unique(self.claims.iter().map(|record| &record.id), "claim")?;
        validate_unique(self.evidence.iter().map(|record| &record.id), "evidence")?;
        validate_unique(
            self.demonstrations.iter().map(|record| &record.id),
            "demonstration",
        )?;
        validate_unique(self.failures.iter().map(|record| &record.id), "failure")?;
        validate_unique(
            self.reproductions.iter().map(|record| &record.id),
            "reproduction",
        )?;
        validate_unique(self.criticisms.iter().map(|record| &record.id), "criticism")?;
        validate_unique(self.proofs.iter().map(|record| &record.id), "proof")?;
        validate_unique(self.recipes.iter().map(|record| &record.id), "recipe")?;
        validate_unique(self.teaching.iter().map(|record| &record.id), "teaching")?;
        validate_unique(self.contests.iter().map(|record| &record.id), "contest")?;
        validate_unique(self.accessions.iter().map(|record| &record.id), "accession")?;
        validate_unique(
            self.assessments.iter().map(|record| &record.id),
            "assessment",
        )?;
        validate_unique(
            self.credentials.iter().map(|record| &record.id),
            "credential",
        )?;
        validate_unique(
            self.apprenticeships.iter().map(|record| &record.id),
            "apprenticeship",
        )?;
        validate_unique(self.standards.iter().map(|record| &record.id), "standard")?;
        validate_unique(
            self.emergency_designs.iter().map(|record| &record.id),
            "emergency design",
        )?;
        validate_unique(
            self.regional_proofs.iter().map(|record| &record.id),
            "regional proof",
        )?;
        validate_unique(
            self.knowledge_obligations.iter().map(|record| &record.id),
            "knowledge obligation",
        )?;
        validate_unique(
            self.successions.iter().map(|record| &record.id),
            "institutional succession",
        )?;

        let subjects = map(&self.subjects, |record| &record.id);
        let methods = map(&self.methods, |record| &record.id);
        let designs = map(&self.designs, |record| &record.id);
        let claims = map(&self.claims, |record| &record.id);
        let evidence = map(&self.evidence, |record| &record.id);
        let demonstrations = map(&self.demonstrations, |record| &record.id);
        let failures = map(&self.failures, |record| &record.id);
        let reproductions = map(&self.reproductions, |record| &record.id);
        let criticisms = map(&self.criticisms, |record| &record.id);
        let proofs = map(&self.proofs, |record| &record.id);
        let teaching = map(&self.teaching, |record| &record.id);
        let contests = map(&self.contests, |record| &record.id);
        let assessments = map(&self.assessments, |record| &record.id);
        let obligations = map(&self.knowledge_obligations, |record| &record.id);

        self.validate_methods()?;
        self.validate_designs(&subjects, &methods, &designs)?;
        self.validate_claims(&designs)?;
        self.validate_evidence(&claims, &methods)?;
        self.validate_demonstrations(&claims, &methods, &failures)?;
        self.validate_failures(&demonstrations)?;
        self.validate_reproductions(&claims, &evidence)?;
        self.validate_criticisms(&subjects, &claims, &evidence)?;
        self.validate_proofs(
            &claims,
            &evidence,
            &demonstrations,
            &reproductions,
            &criticisms,
        )?;
        self.validate_recipes(&designs, &proofs)?;
        self.validate_teaching(&subjects, &evidence)?;
        self.validate_contests(&subjects, &teaching, &evidence)?;
        self.validate_accessions(&subjects, &contests)?;
        self.validate_education(&subjects, &evidence, &assessments)?;
        self.validate_standards(&evidence)?;
        self.validate_emergencies(&designs)?;
        self.validate_regional_proofs(&proofs)?;
        self.validate_successions(&obligations)?;
        Ok(())
    }

    fn validate_methods(&self) -> Result<(), SandmanorValidationError> {
        for record in &self.methods {
            required(&record.objective, "method objective")?;
            required_list(&record.sequence, "method sequence")?;
            required_list(&record.operators, "method operators")?;
            required(&record.comparison_basis, "comparison basis")?;
            required_list(&record.measurements, "method measurements")?;
            required(&record.recording_procedure, "recording procedure")?;
            required_list(&record.stopping_conditions, "stopping conditions")?;
            required(&record.analysis, "method analysis")?;
            if record.changed_after_results_without_disclosure {
                return Err(SandmanorValidationError::PostResultMethodManipulation(
                    record.id.clone(),
                ));
            }
        }
        Ok(())
    }

    fn validate_designs(
        &self,
        subjects: &BTreeMap<&IdentityId, &SandmanorSubjectRecord>,
        methods: &BTreeMap<&MethodId, &MethodRecord>,
        designs: &BTreeMap<&DesignId, &DesignRecord>,
    ) -> Result<(), SandmanorValidationError> {
        for record in &self.designs {
            require_author(&record.author, subjects)?;
            require_ref(methods, &record.method, "method")?;
            required(&record.purpose, "design purpose")?;
            required_list(&record.intended_users, "intended users")?;
            required(&record.problem, "design problem")?;
            required_list(&record.assumptions, "design assumptions")?;
            required_list(&record.inputs, "design inputs")?;
            required_list(&record.outputs, "design outputs")?;
            required(&record.expected_result, "expected result")?;
            required_list(&record.failure_states, "failure states")?;
            required(&record.measurement_plan, "measurement plan")?;
            required(&record.version, "design version")?;
            if let Some(prior) = &record.prior_version {
                require_ref(designs, prior, "prior design version")?;
                if prior == &record.id {
                    return Err(SandmanorValidationError::RevisionErasesHistory(
                        record.id.clone(),
                    ));
                }
            }
            if record.materially_revised
                && (record.prior_version.is_none() || record.inherits_prior_proof)
            {
                return Err(SandmanorValidationError::BreakingRevisionInheritedProof(
                    record.id.clone(),
                ));
            }
            if record.status == DesignStatus::Tombstoned && record.inherits_prior_proof {
                return Err(SandmanorValidationError::TombstonedDesignActive(
                    record.id.clone(),
                ));
            }
        }
        Ok(())
    }

    fn validate_claims(
        &self,
        designs: &BTreeMap<&DesignId, &DesignRecord>,
    ) -> Result<(), SandmanorValidationError> {
        for record in &self.claims {
            require_ref(designs, &record.design, "claim design")?;
            required(&record.statement, "claim statement")?;
            required(&record.scope, "claim scope")?;
            if record.active && record.tombstoned {
                return Err(SandmanorValidationError::TombstonedClaimActive(
                    record.id.clone(),
                ));
            }
        }
        Ok(())
    }

    fn validate_evidence(
        &self,
        claims: &BTreeMap<&ClaimId, &ClaimRecord>,
        methods: &BTreeMap<&MethodId, &MethodRecord>,
    ) -> Result<(), SandmanorValidationError> {
        for record in &self.evidence {
            require_ref(claims, &record.claim, "evidence claim")?;
            require_ref(methods, &record.method, "evidence method")?;
            required(&record.source, "evidence source")?;
            required_list(&record.custody, "evidence custody")?;
            required(&record.content, "evidence content")?;
            if record.fabricated {
                return Err(SandmanorValidationError::FabricatedEvidence(
                    record.id.clone(),
                ));
            }
        }
        Ok(())
    }

    fn validate_demonstrations(
        &self,
        claims: &BTreeMap<&ClaimId, &ClaimRecord>,
        methods: &BTreeMap<&MethodId, &MethodRecord>,
        failures: &BTreeMap<&FailureId, &FailureRecord>,
    ) -> Result<(), SandmanorValidationError> {
        for record in &self.demonstrations {
            require_ref(claims, &record.claim, "demonstration claim")?;
            require_ref(methods, &record.method, "demonstration method")?;
            required(&record.environment, "demonstration environment")?;
            required_list(&record.observers, "demonstration observers")?;
            required(&record.actual_result, "actual result")?;
            if !record.complete_result_recorded {
                return Err(SandmanorValidationError::IncompleteDemonstration(
                    record.id.clone(),
                ));
            }
            if record.prototype && record.represented_as_completed_production {
                return Err(SandmanorValidationError::PrototypeAsProduction(
                    record.id.clone(),
                ));
            }
            if record.simulated && record.represented_as_direct_physical_performance {
                return Err(SandmanorValidationError::SimulationAsPhysical(
                    record.id.clone(),
                ));
            }
            if let Some(failure) = &record.failure {
                let failure = require_ref(failures, failure, "failure record")?;
                if failure.demonstration != record.id {
                    return Err(SandmanorValidationError::FailureMismatch(
                        failure.id.clone(),
                    ));
                }
            }
        }
        Ok(())
    }

    fn validate_failures(
        &self,
        demonstrations: &BTreeMap<&DemonstrationId, &DemonstrationRecord>,
    ) -> Result<(), SandmanorValidationError> {
        for record in &self.failures {
            require_ref(demonstrations, &record.demonstration, "demonstration")?;
            required(&record.intended_result, "failure intended result")?;
            required(&record.actual_result, "failure actual result")?;
            required(
                &record.known_cause_or_uncertainty,
                "failure cause or uncertainty",
            )?;
            required(&record.effect_on_proof, "failure proof effect")?;
            if record.erased {
                return Err(SandmanorValidationError::FailureErased(record.id.clone()));
            }
        }
        Ok(())
    }

    fn validate_reproductions(
        &self,
        claims: &BTreeMap<&ClaimId, &ClaimRecord>,
        evidence: &BTreeMap<&EvidenceId, &EvidenceRecord>,
    ) -> Result<(), SandmanorValidationError> {
        for record in &self.reproductions {
            require_ref(claims, &record.claim, "reproduction claim")?;
            require_refs(evidence, &record.evidence, "reproduction evidence")?;
            required(&record.result, "reproduction result")?;
            if !record.independent || record.original_design_body == record.reproducing_body {
                return Err(SandmanorValidationError::ReproductionNotIndependent(
                    record.id.clone(),
                ));
            }
        }
        Ok(())
    }

    fn validate_criticisms(
        &self,
        subjects: &BTreeMap<&IdentityId, &SandmanorSubjectRecord>,
        claims: &BTreeMap<&ClaimId, &ClaimRecord>,
        evidence: &BTreeMap<&EvidenceId, &EvidenceRecord>,
    ) -> Result<(), SandmanorValidationError> {
        for record in &self.criticisms {
            require_subject(subjects, &record.critic)?;
            require_ref(claims, &record.claim, "criticism claim")?;
            require_refs(evidence, &record.evidence, "criticism evidence")?;
            if record.good_faith && (record.erased || record.retaliation) {
                return Err(SandmanorValidationError::GoodFaithCriticismSuppressed(
                    record.id.clone(),
                ));
            }
        }
        Ok(())
    }

    fn validate_proofs(
        &self,
        claims: &BTreeMap<&ClaimId, &ClaimRecord>,
        evidence: &BTreeMap<&EvidenceId, &EvidenceRecord>,
        demonstrations: &BTreeMap<&DemonstrationId, &DemonstrationRecord>,
        reproductions: &BTreeMap<&ReproductionId, &ReproductionRecord>,
        criticisms: &BTreeMap<&CriticismId, &CriticismRecord>,
    ) -> Result<(), SandmanorValidationError> {
        for record in &self.proofs {
            let claim = require_ref(claims, &record.claim, "proof claim")?;
            required(&record.scope, "proof scope")?;
            require_refs(evidence, &record.evidence, "proof evidence")?;
            require_refs(
                demonstrations,
                &record.demonstrations,
                "proof demonstrations",
            )?;
            require_refs(reproductions, &record.reproductions, "proof reproductions")?;
            require_refs(criticisms, &record.criticism_considered, "proof criticism")?;
            if record.issued_by != sandman_office_id() {
                return Err(SandmanorValidationError::WrongProofAuthority(
                    record.id.clone(),
                ));
            }
            if record.status.supports_reliance() && record.evidence.is_empty() {
                return Err(SandmanorValidationError::ProofWithoutEvidence(
                    record.id.clone(),
                ));
            }
            if claim.requires_independent_reproduction
                && record.status.supports_reliance()
                && record.reproductions.is_empty()
            {
                return Err(SandmanorValidationError::ProofWithoutReproduction(
                    record.id.clone(),
                ));
            }
            if record.status == ProofStatus::Tombstoned && record.active {
                return Err(SandmanorValidationError::TombstonedProofActive(
                    record.id.clone(),
                ));
            }
            if record.emergency_expires_at.is_some()
                && record.active
                && !record.entered_ordinary_review
            {
                return Err(SandmanorValidationError::EmergencyProofWithoutReview(
                    record.id.clone(),
                ));
            }
            if record.grants_title || record.grants_clearance || record.grants_recognition {
                return Err(SandmanorValidationError::ProofSubstitutesOtherHouse(
                    record.id.clone(),
                ));
            }
            if let Some(source) = &record.prefig_source {
                required(&source.recipe_id, "Prefig source Recipe")?;
                require_refs(evidence, &source.evidence, "Prefig source evidence")?;
                if source.evidence.is_empty()
                    || source
                        .evidence
                        .iter()
                        .any(|source_evidence| !record.evidence.contains(source_evidence))
                {
                    return Err(SandmanorValidationError::InvalidPrefigProofSource(
                        record.id.clone(),
                    ));
                }
            }
        }
        Ok(())
    }

    fn validate_recipes(
        &self,
        designs: &BTreeMap<&DesignId, &DesignRecord>,
        proofs: &BTreeMap<&ProofJudgmentId, &ProofJudgmentRecord>,
    ) -> Result<(), SandmanorValidationError> {
        for record in &self.recipes {
            require_ref(designs, &record.design, "recipe design")?;
            let proof = require_ref(proofs, &record.proof, "recipe proof")?;
            required(&record.name, "recipe name")?;
            required(&record.version, "recipe version")?;
            required_list(&record.author_or_lineage, "recipe lineage")?;
            required_list(&record.inputs, "recipe inputs")?;
            required_list(&record.sequence, "recipe sequence")?;
            required(&record.expected_result, "recipe result")?;
            if !proof.status.supports_reliance() || !proof.active {
                return Err(SandmanorValidationError::RecipeWithoutActiveProof(
                    record.id.clone(),
                ));
            }
        }
        Ok(())
    }

    fn validate_teaching(
        &self,
        subjects: &BTreeMap<&IdentityId, &SandmanorSubjectRecord>,
        evidence: &BTreeMap<&EvidenceId, &EvidenceRecord>,
    ) -> Result<(), SandmanorValidationError> {
        for record in &self.teaching {
            require_subject(subjects, &record.teacher)?;
            require_subject(subjects, &record.learner)?;
            require_refs(evidence, &record.baseline_evidence, "teaching baseline")?;
            require_refs(evidence, &record.final_evidence, "teaching final evidence")?;
            required(&record.practice, "teaching practice")?;
            required(&record.design_principle, "teaching design principle")?;
            required(&record.method, "teaching method")?;
            required(&record.observation, "teaching observation")?;
            required(&record.criticism, "teaching criticism")?;
            if record.teacher == record.learner
                || record.teacher_tradition == record.learner_tradition
                || !record.genuine
                || !record.comprehensible
                || !record.relevant
                || !record.non_sabotaging
            {
                return Err(SandmanorValidationError::InvalidReciprocalTeaching(
                    record.id.clone(),
                ));
            }
        }
        Ok(())
    }

    fn validate_contests(
        &self,
        subjects: &BTreeMap<&IdentityId, &SandmanorSubjectRecord>,
        teaching: &BTreeMap<&TeachingRecordId, &TeachingRecord>,
        evidence: &BTreeMap<&EvidenceId, &EvidenceRecord>,
    ) -> Result<(), SandmanorValidationError> {
        for record in &self.contests {
            let [first, second] = &record.candidates;
            require_subject(subjects, &first.being)?;
            require_subject(subjects, &second.being)?;
            require_refs(evidence, &first.baseline_evidence, "candidate baseline")?;
            require_refs(evidence, &first.final_evidence, "candidate final evidence")?;
            require_refs(evidence, &second.baseline_evidence, "candidate baseline")?;
            require_refs(evidence, &second.final_evidence, "candidate final evidence")?;
            if first.being == second.being
                || first.tradition == second.tradition
                || first.unresolved_fraudulent_design
                || second.unresolved_fraudulent_design
                || !first.teaching_experience_demonstrated
                || !second.teaching_experience_demonstrated
                || !first.willingness_to_learn_demonstrated
                || !second.willingness_to_learn_demonstrated
            {
                return Err(SandmanorValidationError::InvalidContestCandidates(
                    record.id.clone(),
                ));
            }
            let lessons = record
                .teaching
                .iter()
                .map(|id| require_ref(teaching, id, "contest teaching"))
                .collect::<Result<Vec<_>, _>>()?;
            let first_taught_second = lessons
                .iter()
                .any(|lesson| lesson.teacher == first.being && lesson.learner == second.being);
            let second_taught_first = lessons
                .iter()
                .any(|lesson| lesson.teacher == second.being && lesson.learner == first.being);
            if !first_taught_second || !second_taught_first {
                return Err(SandmanorValidationError::ContestWithoutReciprocalTeaching(
                    record.id.clone(),
                ));
            }
            if !record.review_body.complete()
                || !record.audience_received_baselines
                || !record.audience_received_process
                || !record.audience_received_results
                || !record.audience_could_question_candidates
                || !record.conflicts_disclosed
            {
                return Err(SandmanorValidationError::InvalidContestReview(
                    record.id.clone(),
                ));
            }
            if record.fraudulent && record.outcome != ContestOutcome::VoidForFraud {
                return Err(SandmanorValidationError::FraudulentContestNotVoid(
                    record.id.clone(),
                ));
            }
            match record.outcome {
                ContestOutcome::Winner => {
                    let Some(winner) = &record.winner else {
                        return Err(SandmanorValidationError::ContestWinnerMismatch(
                            record.id.clone(),
                        ));
                    };
                    if (winner != &first.being && winner != &second.being)
                        || record.fraudulent
                        || !record.complete
                        || !record.challenges_resolved
                    {
                        return Err(SandmanorValidationError::ContestWinnerMismatch(
                            record.id.clone(),
                        ));
                    }
                }
                ContestOutcome::Tied => {
                    if record.winner.is_some() || !record.joint_applied_design_challenge_completed {
                        return Err(SandmanorValidationError::ContestTieWithoutJointChallenge(
                            record.id.clone(),
                        ));
                    }
                }
                ContestOutcome::InsufficientImprovement | ContestOutcome::VoidForFraud => {
                    if record.winner.is_some() {
                        return Err(SandmanorValidationError::ContestWinnerMismatch(
                            record.id.clone(),
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    fn validate_accessions(
        &self,
        subjects: &BTreeMap<&IdentityId, &SandmanorSubjectRecord>,
        contests: &BTreeMap<&ContestId, &ContestOfImprovementRecord>,
    ) -> Result<(), SandmanorValidationError> {
        let active = self
            .accessions
            .iter()
            .filter(|record| record.active)
            .count();
        if active != 1 {
            return Err(SandmanorValidationError::ActiveSandmanCount(active));
        }
        for record in &self.accessions {
            let holder = require_subject(subjects, &record.holder)?;
            let contest = require_ref(contests, &record.contest, "accession contest")?;
            if record.active
                && (record.tombstoned
                    || holder.tombstoned
                    || record.origin != SandmanAuthorityOrigin::ContestOfImprovement
                    || contest.outcome != ContestOutcome::Winner
                    || contest.winner.as_ref() != Some(&record.holder)
                    || !contest.complete
                    || !contest.challenges_resolved
                    || !record.stonebend_title_recorded
                    || !record.flynt_recognition_recorded
                    || record.public_learning_statement.trim().is_empty()
                    || !record.sealed)
            {
                return Err(SandmanorValidationError::InvalidSandmanAccession(
                    record.id.clone(),
                ));
            }
        }
        Ok(())
    }

    fn validate_education(
        &self,
        subjects: &BTreeMap<&IdentityId, &SandmanorSubjectRecord>,
        evidence: &BTreeMap<&EvidenceId, &EvidenceRecord>,
        assessments: &BTreeMap<&AssessmentId, &AssessmentRecord>,
    ) -> Result<(), SandmanorValidationError> {
        for record in &self.assessments {
            require_subject(subjects, &record.subject)?;
            required_list(&record.criteria, "assessment criteria")?;
            require_refs(evidence, &record.evidence, "assessment evidence")?;
            if record.hidden_criteria {
                return Err(SandmanorValidationError::AssessmentHasHiddenCriteria(
                    record.id.clone(),
                ));
            }
        }
        for record in &self.credentials {
            require_subject(subjects, &record.holder)?;
            let assessment = require_ref(assessments, &record.assessment, "credential assessment")?;
            required(&record.scope, "credential scope")?;
            if record.active
                && (!assessment.demonstrated
                    || assessment.subject != record.holder
                    || record.fraudulent)
            {
                return Err(SandmanorValidationError::CredentialWithoutAssessment(
                    record.id.clone(),
                ));
            }
        }
        for record in &self.apprenticeships {
            require_subject(subjects, &record.teacher)?;
            require_subject(subjects, &record.apprentice)?;
            required(&record.field, "apprenticeship field")?;
            required(&record.duration, "apprenticeship duration")?;
            required_list(&record.learning_goals, "apprenticeship learning goals")?;
            required_list(
                &record.teaching_obligations,
                "apprenticeship teaching obligations",
            )?;
            required(
                &record.complaint_process,
                "apprenticeship complaint process",
            )?;
            if record.labor_replaces_teaching {
                return Err(SandmanorValidationError::ExploitativeApprenticeship(
                    record.id.clone(),
                ));
            }
            if let Some(assessment) = &record.assessment {
                require_ref(assessments, assessment, "apprenticeship assessment")?;
            }
        }
        Ok(())
    }

    fn validate_standards(
        &self,
        evidence: &BTreeMap<&EvidenceId, &EvidenceRecord>,
    ) -> Result<(), SandmanorValidationError> {
        for record in &self.standards {
            required(&record.purpose, "standard purpose")?;
            required(&record.scope, "standard scope")?;
            require_refs(evidence, &record.evidence, "standard evidence")?;
            required(&record.implementation_plan, "standard implementation plan")?;
            required(&record.transition_period, "standard transition period")?;
            required(&record.revision_process, "standard revision process")?;
            required(&record.appeal_process, "standard appeal process")?;
            if !record.public_reviewed || !record.minorian_considered || !record.minoan_considered {
                return Err(SandmanorValidationError::InvalidStandard(record.id.clone()));
            }
        }
        Ok(())
    }

    fn validate_emergencies(
        &self,
        designs: &BTreeMap<&DesignId, &DesignRecord>,
    ) -> Result<(), SandmanorValidationError> {
        for record in &self.emergency_designs {
            require_ref(designs, &record.design, "emergency design")?;
            required(&record.urgency_reason, "emergency urgency")?;
            required(&record.temporary_scope, "emergency scope")?;
            required_list(&record.monitoring, "emergency monitoring")?;
            required_list(&record.stopping_conditions, "emergency stopping conditions")?;
            if !record.incomplete_proof_disclosed
                || !record.risks_disclosed
                || !record.ordinary_review_started
                || record.treated_as_permanent_proof
            {
                return Err(SandmanorValidationError::InvalidEmergencyDesign(
                    record.id.clone(),
                ));
            }
        }
        Ok(())
    }

    fn validate_regional_proofs(
        &self,
        proofs: &BTreeMap<&ProofJudgmentId, &ProofJudgmentRecord>,
    ) -> Result<(), SandmanorValidationError> {
        for record in &self.regional_proofs {
            validate_sandmanor_transition(record.predecessor.frame(), record.result.frame())
                .map_err(|_| SandmanorValidationError::InvalidRegionalProof(record.id.clone()))?;
            let expected = match (record.predecessor, record.result) {
                (SandmanorForm::Gnome, SandmanorForm::Minotaur) => (
                    SandmanorLineage::Minorian,
                    RegionalProofDomain::AuraFields,
                    RegionalFunction::AdvancedTendingAndFieldLabor,
                ),
                (SandmanorForm::Elf, SandmanorForm::Centaur) => (
                    SandmanorLineage::Minoan,
                    RegionalProofDomain::AuraBeachAndCurrentSea,
                    RegionalFunction::RoamAuraBeachAndGuardCurrentSea,
                ),
                _ => {
                    return Err(SandmanorValidationError::InvalidRegionalProof(
                        record.id.clone(),
                    ));
                }
            };
            let proof = require_ref(proofs, &record.proof, "regional proof judgment")?;
            if record.predecessor.lineage() != expected.0
                || record.domain != expected.1
                || record.function != expected.2
                || !proof.status.supports_reliance()
                || !proof.active
                || record.grants_synthesis_clearance
                || record.grants_title_or_office
            {
                return Err(SandmanorValidationError::InvalidRegionalProof(
                    record.id.clone(),
                ));
            }
        }
        Ok(())
    }

    fn validate_successions(
        &self,
        obligations: &BTreeMap<&KnowledgeObligationId, &KnowledgeObligation>,
    ) -> Result<(), SandmanorValidationError> {
        for record in &self.knowledge_obligations {
            required(&record.duty, "knowledge obligation")?;
        }
        for record in &self.successions {
            let predecessor = record
                .predecessor_obligations
                .iter()
                .collect::<BTreeSet<_>>();
            let successor = record.successor_obligations.iter().collect::<BTreeSet<_>>();
            if predecessor != successor {
                return Err(SandmanorValidationError::KnowledgeObligationsLost(
                    record.id.clone(),
                ));
            }
            for obligation in successor {
                require_ref(obligations, obligation, "knowledge obligation")?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SandmanorValidationError {
    DuplicateId(&'static str, String),
    MissingRecord(&'static str, String),
    MissingRequired(&'static str),
    MissingSubject(IdentityId),
    TombstonedSubjectActive(IdentityId),
    TraditionEqualityViolated,
    PostResultMethodManipulation(MethodId),
    RevisionErasesHistory(DesignId),
    BreakingRevisionInheritedProof(DesignId),
    TombstonedDesignActive(DesignId),
    TombstonedClaimActive(ClaimId),
    FabricatedEvidence(EvidenceId),
    IncompleteDemonstration(DemonstrationId),
    PrototypeAsProduction(DemonstrationId),
    SimulationAsPhysical(DemonstrationId),
    FailureMismatch(FailureId),
    FailureErased(FailureId),
    ReproductionNotIndependent(ReproductionId),
    GoodFaithCriticismSuppressed(CriticismId),
    WrongProofAuthority(ProofJudgmentId),
    ProofWithoutEvidence(ProofJudgmentId),
    ProofWithoutReproduction(ProofJudgmentId),
    TombstonedProofActive(ProofJudgmentId),
    EmergencyProofWithoutReview(ProofJudgmentId),
    ProofSubstitutesOtherHouse(ProofJudgmentId),
    InvalidPrefigProofSource(ProofJudgmentId),
    RecipeWithoutActiveProof(RecipeId),
    InvalidReciprocalTeaching(TeachingRecordId),
    InvalidContestCandidates(ContestId),
    ContestWithoutReciprocalTeaching(ContestId),
    InvalidContestReview(ContestId),
    FraudulentContestNotVoid(ContestId),
    ContestWinnerMismatch(ContestId),
    ContestTieWithoutJointChallenge(ContestId),
    ActiveSandmanCount(usize),
    InvalidSandmanAccession(AccessionRecordId),
    AssessmentHasHiddenCriteria(AssessmentId),
    CredentialWithoutAssessment(CredentialId),
    ExploitativeApprenticeship(ApprenticeshipId),
    InvalidStandard(StandardId),
    InvalidEmergencyDesign(EmergencyDesignId),
    InvalidRegionalProof(RegionalProofId),
    KnowledgeObligationsLost(InstitutionalSuccessionId),
}

impl fmt::Display for SandmanorValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Sandmanor constitutional violation: {self:?}")
    }
}

impl std::error::Error for SandmanorValidationError {}

fn map<T, K: Ord>(records: &[T], key: impl Fn(&T) -> &K) -> BTreeMap<&K, &T> {
    records.iter().map(|record| (key(record), record)).collect()
}

fn validate_unique<'a, T: fmt::Display + Ord + 'a>(
    ids: impl Iterator<Item = &'a T>,
    kind: &'static str,
) -> Result<(), SandmanorValidationError> {
    let mut seen = BTreeSet::new();
    for id in ids {
        if !seen.insert(id) {
            return Err(SandmanorValidationError::DuplicateId(kind, id.to_string()));
        }
    }
    Ok(())
}

fn required(value: &str, kind: &'static str) -> Result<(), SandmanorValidationError> {
    if value.trim().is_empty() {
        Err(SandmanorValidationError::MissingRequired(kind))
    } else {
        Ok(())
    }
}

fn required_list<T>(values: &[T], kind: &'static str) -> Result<(), SandmanorValidationError> {
    if values.is_empty() {
        Err(SandmanorValidationError::MissingRequired(kind))
    } else {
        Ok(())
    }
}

fn require_ref<'a, K: fmt::Display + Ord, V>(
    records: &'a BTreeMap<&K, &V>,
    id: &K,
    kind: &'static str,
) -> Result<&'a V, SandmanorValidationError> {
    records
        .get(id)
        .copied()
        .ok_or_else(|| SandmanorValidationError::MissingRecord(kind, id.to_string()))
}

fn require_refs<K: fmt::Display + Ord, V>(
    records: &BTreeMap<&K, &V>,
    ids: &[K],
    kind: &'static str,
) -> Result<(), SandmanorValidationError> {
    for id in ids {
        require_ref(records, id, kind)?;
    }
    Ok(())
}

fn require_subject<'a>(
    subjects: &'a BTreeMap<&IdentityId, &SandmanorSubjectRecord>,
    id: &IdentityId,
) -> Result<&'a SandmanorSubjectRecord, SandmanorValidationError> {
    let subject = subjects
        .get(id)
        .copied()
        .ok_or_else(|| SandmanorValidationError::MissingSubject(id.clone()))?;
    if subject.tombstoned {
        return Err(SandmanorValidationError::TombstonedSubjectActive(
            id.clone(),
        ));
    }
    Ok(subject)
}

fn require_author(
    author: &DesignAuthor,
    subjects: &BTreeMap<&IdentityId, &SandmanorSubjectRecord>,
) -> Result<(), SandmanorValidationError> {
    if let DesignAuthor::Being(being) = author {
        require_subject(subjects, being)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn civic_traditions_are_equal_distinct_and_exact() {
        validate_civic_traditions().unwrap();
        assert_eq!(CivicTradition::Minorian.people(), "Gnomes");
        assert_eq!(CivicTradition::Minoan.people(), "Elves");
        assert_ne!(
            CivicTradition::Minorian.domain(),
            CivicTradition::Minoan.domain()
        );
    }

    #[test]
    fn proof_status_never_collapses_all_statuses_into_pass_fail() {
        assert!(ProofStatus::ProvenWithinScope.supports_reliance());
        assert!(!ProofStatus::Disputed.supports_reliance());
        assert!(!ProofStatus::Tombstoned.supports_reliance());
    }
}
