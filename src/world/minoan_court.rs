//! The Minoan County Court System and five-stage judicial/Restitution cycle.
//!
//! This is a bounded shared-judiciary adapter over Hollow Grove's existing
//! stable identity, evidence, House, Stonebend Title-scope, challenge, and
//! institutional records. The Minoans host the Court; the Court does not own
//! House law, fabricate House evidence, execute House remedies, ratify
//! amendments, or create a new constitutional lifecycle.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::hollow_grove_contract::House;
use crate::institution::{IdentityId, InstitutionId};
use crate::world::sandmanor::milestone::minoan_county_courthouse_id;
use crate::world::stonebend::second_pass::{GateScope, StonebendConstitutionalPower};
use crate::world::stonebend::third_pass::TitleInterventionTarget;
use crate::world::stonebend::{EvidenceRecordId, SealRecordId, TitleRecordId};

pub const MINOAN_COUNTY_COURT_SOURCE: &str =
    "MINOAN_COUNTY_COURT_SYSTEM_AND_RESTITUTION_CYCLE_V1.md";

#[must_use]
pub fn minoan_county_court_system_id() -> InstitutionId {
    minoan_county_courthouse_id()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CourtJurisdiction {
    MinoanCoastal,
    Stonebend,
    Flynt,
    Glaushouse,
    Sandmanor,
    CentralJunction,
    CrossHouse,
    Constitutional,
}

impl CourtJurisdiction {
    pub const ALL: [Self; 8] = [
        Self::MinoanCoastal,
        Self::Stonebend,
        Self::Flynt,
        Self::Glaushouse,
        Self::Sandmanor,
        Self::CentralJunction,
        Self::CrossHouse,
        Self::Constitutional,
    ];

    #[must_use]
    pub const fn domain_house(self) -> Option<House> {
        match self {
            Self::Stonebend => Some(House::Stonebend),
            Self::Flynt => Some(House::Flynt),
            Self::Glaushouse => Some(House::Glaushouse),
            Self::Sandmanor | Self::MinoanCoastal => Some(House::Sandmanor),
            Self::CentralJunction | Self::CrossHouse | Self::Constitutional => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DomainEvidenceSource {
    MinoanCoastal,
    House(House),
    CentralJunction,
    AffectedParty,
    PublicRecord,
}

impl DomainEvidenceSource {
    #[must_use]
    pub const fn supplied_by_court(self) -> bool {
        false
    }

    #[must_use]
    pub const fn supports(self, jurisdiction: CourtJurisdiction) -> bool {
        match (self, jurisdiction) {
            (Self::MinoanCoastal, CourtJurisdiction::MinoanCoastal)
            | (Self::House(House::Stonebend), CourtJurisdiction::Stonebend)
            | (Self::House(House::Flynt), CourtJurisdiction::Flynt)
            | (Self::House(House::Glaushouse), CourtJurisdiction::Glaushouse)
            | (Self::House(House::Sandmanor), CourtJurisdiction::Sandmanor)
            | (Self::CentralJunction, CourtJurisdiction::CentralJunction)
            | (Self::AffectedParty, _)
            | (Self::PublicRecord, CourtJurisdiction::Constitutional)
            | (Self::PublicRecord, CourtJurisdiction::CrossHouse) => true,
            (Self::House(_), CourtJurisdiction::CrossHouse)
            | (Self::House(_), CourtJurisdiction::Constitutional)
            | (Self::CentralJunction, CourtJurisdiction::CrossHouse)
            | (Self::CentralJunction, CourtJurisdiction::Constitutional)
            | (Self::MinoanCoastal, CourtJurisdiction::CrossHouse)
            | (Self::MinoanCoastal, CourtJurisdiction::Constitutional)
            | (Self::PublicRecord, _) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum StandingGround {
    DirectInjury,
    LawfulCustody,
    TitleOrScopeInterest,
    AffectedProliteriateNode,
    PublicDuty,
    ConstitutionalOffice,
    ContractualRelation,
    OtherLawfulInterest,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaseParty {
    pub identity: IdentityId,
    pub standing: BTreeSet<StandingGround>,
    pub represented_by: Option<IdentityId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceSubmission {
    pub identity: IdentityId,
    pub source: DomainEvidenceSource,
    pub jurisdiction: CourtJurisdiction,
    pub records: Vec<EvidenceRecordId>,
    pub authenticated_by: IdentityId,
    pub description: String,
}

impl EvidenceSubmission {
    pub fn validate(&self) -> Result<(), CourtValidationError> {
        if self.records.is_empty()
            || self.description.trim().is_empty()
            || !self.source.supports(self.jurisdiction)
            || self.source.supplied_by_court()
        {
            return Err(CourtValidationError::InvalidDomainEvidence(
                self.identity.clone(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestimonyRecord {
    pub identity: IdentityId,
    pub witness: IdentityId,
    pub evidence: EvidenceRecordId,
    pub jurisdiction: CourtJurisdiction,
    pub affected_party: bool,
    pub compelled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum JudicialStage {
    Conciliation,
    FirstHearing,
    Appeal,
    ConstitutionalReview,
    Restitution,
}

impl JudicialStage {
    pub const ALL: [Self; 5] = [
        Self::Conciliation,
        Self::FirstHearing,
        Self::Appeal,
        Self::ConstitutionalReview,
        Self::Restitution,
    ];

    #[must_use]
    pub const fn semantic_order(self) -> u8 {
        match self {
            Self::Conciliation => 0,
            Self::FirstHearing => 1,
            Self::Appeal => 2,
            Self::ConstitutionalReview => 3,
            Self::Restitution => 4,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JudicialStageRecord {
    pub identity: IdentityId,
    pub case: IdentityId,
    pub stage: JudicialStage,
    pub cycle: u32,
    pub evidence: Vec<EvidenceRecordId>,
}

impl JudicialStageRecord {
    #[must_use]
    pub fn semantic_key(&self) -> (u32, u8, &str) {
        (
            self.cycle,
            self.stage.semantic_order(),
            self.identity.as_str(),
        )
    }
}

#[must_use]
pub fn semantic_judicial_history(records: &[JudicialStageRecord]) -> Vec<&JudicialStageRecord> {
    let mut ordered = records.iter().collect::<Vec<_>>();
    ordered.sort_by_key(|record| record.semantic_key());
    ordered
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConciliationSettlement {
    pub identity: IdentityId,
    pub case: IdentityId,
    pub parties: BTreeSet<IdentityId>,
    pub agreed_remedies: BTreeSet<IdentityId>,
    pub authority: IdentityId,
    pub completion_condition: String,
    pub voluntary: bool,
    pub coerced: bool,
    pub surrenders_lawful_right: bool,
}

impl ConciliationSettlement {
    pub fn validate(&self) -> Result<(), CourtValidationError> {
        if self.parties.is_empty()
            || self.agreed_remedies.is_empty()
            || self.completion_condition.trim().is_empty()
            || !self.voluntary
            || self.coerced
            || self.surrenders_lawful_right
        {
            return Err(CourtValidationError::InvalidConciliationSettlement(
                self.identity.clone(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConciliationRecord {
    pub identity: IdentityId,
    pub case: IdentityId,
    pub relationships: Vec<String>,
    pub agreed_facts: Vec<String>,
    pub disputed_facts: Vec<String>,
    pub immediate_risks: Vec<String>,
    pub affected_titles: BTreeSet<TitleRecordId>,
    pub affected_yield: Vec<EvidenceRecordId>,
    pub voluntary_repair_possible: bool,
    pub settlement: Option<ConciliationSettlement>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ProtectiveAction {
    TemporaryStay,
    PreserveEvidence,
    TemporaryCustody,
    RestrictedDeployment,
    TemporaryClinicalTransfer,
    FreezeRecordAlteration,
    ProhibitFurtherExtraction,
    TemporaryGateScopeLimitation,
    PreserveDisputedMaterial,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtectiveOrder {
    pub identity: IdentityId,
    pub case: IdentityId,
    pub authority: IdentityId,
    pub evidence: Vec<EvidenceRecordId>,
    pub protected_subject: IdentityId,
    pub action: ProtectiveAction,
    pub exact_scope: String,
    pub reason: String,
    pub start_condition: String,
    pub termination_condition: String,
    pub review_required: bool,
    pub affected_titles: BTreeSet<TitleRecordId>,
    pub permanent: bool,
}

impl ProtectiveOrder {
    pub fn validate(&self) -> Result<(), CourtValidationError> {
        if self.evidence.is_empty()
            || self.exact_scope.trim().is_empty()
            || self.reason.trim().is_empty()
            || self.start_condition.trim().is_empty()
            || self.termination_condition.trim().is_empty()
            || !self.review_required
            || self.permanent
        {
            return Err(CourtValidationError::UnboundedProtectiveOrder(
                self.identity.clone(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FindingKind {
    Fact,
    Testimony,
    ExpertEvidence,
    LegalConclusion,
    ConstitutionalConclusion,
    Remedy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FirstHearingFinding {
    pub identity: IdentityId,
    pub kind: FindingKind,
    pub statement: String,
    pub evidence: Vec<EvidenceRecordId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum JudgmentTarget {
    Person(IdentityId),
    TitleCore(TitleRecordId),
    GateScope {
        title: TitleRecordId,
        scope: GateScope,
    },
    License(IdentityId),
    BearerTenure(IdentityId),
    Mandate(IdentityId),
    CustodyRelation(IdentityId),
    Contract(IdentityId),
    PublicRecord(IdentityId),
    MaterialBatch(IdentityId),
    SynthesisRecord(IdentityId),
    HollowingAuthorization(IdentityId),
    Institution(InstitutionId),
    SpecificAction(IdentityId),
}

impl JudgmentTarget {
    #[must_use]
    pub fn title_intervention_target(&self) -> Option<TitleInterventionTarget> {
        match self {
            Self::TitleCore(_) => Some(TitleInterventionTarget::CoreTitle),
            Self::GateScope { scope, .. } => Some(TitleInterventionTarget::GateScope(*scope)),
            Self::License(identity) => Some(TitleInterventionTarget::License(identity.clone())),
            Self::BearerTenure(identity) => {
                Some(TitleInterventionTarget::BearerTenure(identity.clone()))
            }
            Self::Mandate(identity) => Some(TitleInterventionTarget::Mandate(identity.clone())),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RemedyKind {
    PropertyReturn,
    CustodyRestoration,
    FormRepair,
    TitleRestoration,
    GateScopeRestoration,
    PublicRecordCorrection,
    MaterialReplacement,
    LaborOrBurdenCompensation,
    CareDelivery,
    TransferReversal,
    AccessRestoration,
    OpportunityRestoration,
    InfrastructureRepair,
    CentralJunctionRecordCorrection,
    CurrentReturn,
    ProvenanceRestoration,
    MonetaryCompensation,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResponsibleInstitution {
    Stonebend,
    Flynt,
    Glaushouse,
    Sandmanor,
    CentralJunction,
    MinoanCoastal,
    ProliteriateWitness,
    HypergiantBoundaryEnforcement,
}

impl ResponsibleInstitution {
    #[must_use]
    pub const fn domain_house(self) -> Option<House> {
        match self {
            Self::Stonebend | Self::HypergiantBoundaryEnforcement => Some(House::Stonebend),
            Self::Flynt => Some(House::Flynt),
            Self::Glaushouse => Some(House::Glaushouse),
            Self::Sandmanor | Self::MinoanCoastal => Some(House::Sandmanor),
            Self::CentralJunction | Self::ProliteriateWitness => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Remedy {
    pub identity: IdentityId,
    pub case: IdentityId,
    pub kind: RemedyKind,
    pub target: JudgmentTarget,
    pub responsible_institution: ResponsibleInstitution,
    pub harmed_party_or_community: IdentityId,
    pub ordered_action: String,
    pub completion_condition: String,
    pub evidence: Vec<EvidenceRecordId>,
}

impl Remedy {
    pub fn validate(&self) -> Result<(), CourtValidationError> {
        if self.ordered_action.trim().is_empty()
            || self.completion_condition.trim().is_empty()
            || self.evidence.is_empty()
        {
            return Err(CourtValidationError::InvalidRemedy(self.identity.clone()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JudgmentConstitutionalEffect {
    TargetedRemedy,
    EvidenceForStonebendChallenge(StonebendConstitutionalPower),
    Referral,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Judgment {
    pub identity: IdentityId,
    pub case: IdentityId,
    pub findings: BTreeSet<IdentityId>,
    pub legal_conclusions: Vec<String>,
    pub targets: BTreeSet<JudgmentTarget>,
    pub remedies: BTreeSet<IdentityId>,
    pub constitutional_effect: JudgmentConstitutionalEffect,
    pub evidence: Vec<EvidenceRecordId>,
    pub court_executes_remedy: bool,
    pub court_removes_principal_power: bool,
}

impl Judgment {
    pub fn validate(&self) -> Result<(), CourtValidationError> {
        if self.findings.is_empty()
            || self.legal_conclusions.is_empty()
            || self.targets.is_empty()
            || self.evidence.is_empty()
            || self.court_executes_remedy
            || self.court_removes_principal_power
        {
            return Err(CourtValidationError::JudicialAppropriation(
                self.identity.clone(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FirstHearingRecord {
    pub identity: IdentityId,
    pub case: IdentityId,
    pub standing_confirmed: BTreeSet<IdentityId>,
    pub jurisdictions_reviewed: BTreeSet<CourtJurisdiction>,
    pub evidence_considered: BTreeSet<IdentityId>,
    pub testimony_considered: BTreeSet<IdentityId>,
    pub findings: Vec<FirstHearingFinding>,
    pub judgment: Option<Judgment>,
    pub dismissed: bool,
    pub referral: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AppealGround {
    LegalError,
    ProceduralError,
    JurisdictionalError,
    ExcludedMaterialEvidence,
    UnsupportedFinding,
    DisproportionateRemedy,
    ImproperStandard,
    AbuseOfJudicialAuthority,
    ConflictOfInterest,
    DenialOfLawfulParticipation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AppealDisposition {
    Affirmed,
    Modified,
    Reversed,
    Remanded,
    Vacated,
    Narrowed,
    AdditionalEvidenceRequired,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppealRecord {
    pub identity: IdentityId,
    pub case: IdentityId,
    pub challenged_judgment: IdentityId,
    pub grounds: BTreeSet<AppealGround>,
    pub record_evidence: Vec<EvidenceRecordId>,
    pub review_standard: String,
    pub disposition: AppealDisposition,
    pub effect_on_judgment: String,
    pub retries_all_facts: bool,
    pub stay: Option<IdentityId>,
}

impl AppealRecord {
    pub fn validate(&self) -> Result<(), CourtValidationError> {
        if self.grounds.is_empty()
            || self.record_evidence.is_empty()
            || self.review_standard.trim().is_empty()
            || self.effect_on_judgment.trim().is_empty()
            || self.retries_all_facts
        {
            return Err(CourtValidationError::InvalidAppeal(self.identity.clone()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ConstitutionalReviewGround {
    HouseBoundary,
    ClaimTitleYield,
    StonebendPowerSeparation,
    DiamondOrVacancyRule,
    ProliteriateContinuity,
    TitleLifecycleProtection,
    LawfulSynthesis,
    LawfulHollowing,
    CrossHouseAuthority,
    ProtectedPublicParticipation,
    AmendmentProcedure,
    HigherConstitutionalLaw,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ConstitutionalReviewDisposition {
    ConstitutionallyValid,
    ConstitutionallyInvalid,
    ValidIfNarrowed,
    ReturnedForCorrection,
    Stayed,
    ProcessDefective,
    EligibleForRatification,
    NotEligibleForRatification,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstitutionalReviewRecord {
    pub identity: IdentityId,
    pub case: IdentityId,
    pub grounds: BTreeSet<ConstitutionalReviewGround>,
    pub evidence: Vec<EvidenceRecordId>,
    pub disposition: ConstitutionalReviewDisposition,
    pub retries_settled_facts: bool,
    pub ratifies_amendment: bool,
    pub amends_constitution: bool,
    pub bears_diamond: bool,
    pub forges_stonebend_claim: bool,
    pub replaces_proliteriate: bool,
    pub transfers_house_authority: bool,
}

impl ConstitutionalReviewRecord {
    pub fn validate(&self) -> Result<(), CourtValidationError> {
        if self.grounds.is_empty()
            || self.evidence.is_empty()
            || self.retries_settled_facts
            || self.ratifies_amendment
            || self.amends_constitution
            || self.bears_diamond
            || self.forges_stonebend_claim
            || self.replaces_proliteriate
            || self.transfers_house_authority
        {
            return Err(CourtValidationError::InvalidConstitutionalReview(
                self.identity.clone(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RestitutionDisposition {
    EquilibriumConfirmed,
    PartiallySatisfied,
    RemedyFailed,
    NewHarmCreated,
    ResponsibleInstitutionDefaulted,
    AdditionalRestitutionRequired,
    ReturnedToConciliation,
    ReturnedToFirstHearing,
    ReturnedToAppeal,
    ReturnedToConstitutionalReview,
}

impl RestitutionDisposition {
    #[must_use]
    pub const fn closes_case(self) -> bool {
        matches!(self, Self::EquilibriumConfirmed)
    }

    #[must_use]
    pub const fn return_stage(self) -> Option<JudicialStage> {
        match self {
            Self::ReturnedToConciliation => Some(JudicialStage::Conciliation),
            Self::ReturnedToFirstHearing => Some(JudicialStage::FirstHearing),
            Self::ReturnedToAppeal => Some(JudicialStage::Appeal),
            Self::ReturnedToConstitutionalReview => Some(JudicialStage::ConstitutionalReview),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EquilibriumAssessment {
    pub lawful_boundary_restored_or_clarified: bool,
    pub remedy_reached_intended_subject: bool,
    pub continuing_burden_lawfully_assigned: bool,
    pub historical_record_accurate: bool,
    pub hidden_constitutional_violation: bool,
    pub immediate_same_case_harm_unresolved: bool,
    pub remaining_burden: String,
}

impl EquilibriumAssessment {
    #[must_use]
    pub const fn can_hold(&self) -> bool {
        self.lawful_boundary_restored_or_clarified
            && self.remedy_reached_intended_subject
            && self.continuing_burden_lawfully_assigned
            && self.historical_record_accurate
            && !self.hidden_constitutional_violation
            && !self.immediate_same_case_harm_unresolved
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestitutionRecord {
    pub identity: IdentityId,
    pub case: IdentityId,
    pub judgment: IdentityId,
    pub responsible_institutions: BTreeSet<ResponsibleInstitution>,
    pub harmed_parties_or_communities: BTreeSet<IdentityId>,
    pub remedies: BTreeSet<IdentityId>,
    pub delivery_evidence: Vec<EvidenceRecordId>,
    pub completion_evidence: Vec<EvidenceRecordId>,
    pub remaining_burden: String,
    pub unintended_effects: Vec<String>,
    pub yield_evidence: Vec<EvidenceRecordId>,
    pub equilibrium: EquilibriumAssessment,
    pub disposition: RestitutionDisposition,
    pub cycle: u32,
}

impl RestitutionRecord {
    pub fn validate(&self) -> Result<(), CourtValidationError> {
        if self.responsible_institutions.is_empty()
            || self.harmed_parties_or_communities.is_empty()
            || self.remedies.is_empty()
            || self.delivery_evidence.is_empty()
            || self.yield_evidence.is_empty()
        {
            return Err(CourtValidationError::InvalidRestitution(
                self.identity.clone(),
            ));
        }
        if self.disposition == RestitutionDisposition::EquilibriumConfirmed
            && (self.completion_evidence.is_empty() || !self.equilibrium.can_hold())
        {
            return Err(CourtValidationError::EquilibriumNotEstablished(
                self.identity.clone(),
            ));
        }
        if self.disposition.return_stage().is_some() && self.equilibrium.can_hold() {
            return Err(CourtValidationError::InvalidRestitution(
                self.identity.clone(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaseRecurrence {
    pub identity: IdentityId,
    pub case: IdentityId,
    pub failed_restitution: IdentityId,
    pub from_cycle: u32,
    pub to_cycle: u32,
    pub return_stage: JudicialStage,
    pub lawful_reason: String,
    pub evidence: Vec<EvidenceRecordId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CaseClosure {
    Open,
    EquilibriumConfirmed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourtCasePolicy {
    pub conciliation_required: bool,
    pub full_review_cycle_required: bool,
}

impl Default for CourtCasePolicy {
    fn default() -> Self {
        Self {
            conciliation_required: true,
            full_review_cycle_required: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourtCase {
    pub identity: IdentityId,
    pub parties: BTreeMap<IdentityId, CaseParty>,
    pub jurisdictions: BTreeSet<CourtJurisdiction>,
    pub claims: BTreeSet<IdentityId>,
    pub affected_titles: BTreeSet<TitleRecordId>,
    pub closure: CaseClosure,
    pub policy: CourtCasePolicy,
    evidence: BTreeMap<IdentityId, EvidenceSubmission>,
    testimony: BTreeMap<IdentityId, TestimonyRecord>,
    stage_records: BTreeMap<IdentityId, JudicialStageRecord>,
    conciliations: BTreeMap<IdentityId, ConciliationRecord>,
    protective_orders: BTreeMap<IdentityId, ProtectiveOrder>,
    findings: BTreeMap<IdentityId, FirstHearingFinding>,
    hearings: BTreeMap<IdentityId, FirstHearingRecord>,
    judgments: BTreeMap<IdentityId, Judgment>,
    remedies: BTreeMap<IdentityId, Remedy>,
    appeals: BTreeMap<IdentityId, AppealRecord>,
    constitutional_reviews: BTreeMap<IdentityId, ConstitutionalReviewRecord>,
    restitutions: BTreeMap<IdentityId, RestitutionRecord>,
    recurrences: BTreeMap<IdentityId, CaseRecurrence>,
}

impl CourtCase {
    pub fn new(
        identity: IdentityId,
        parties: Vec<CaseParty>,
        jurisdictions: BTreeSet<CourtJurisdiction>,
        claims: BTreeSet<IdentityId>,
        affected_titles: BTreeSet<TitleRecordId>,
        policy: CourtCasePolicy,
    ) -> Result<Self, CourtValidationError> {
        let parties = parties
            .into_iter()
            .map(|party| (party.identity.clone(), party))
            .collect::<BTreeMap<_, _>>();
        if parties.is_empty()
            || parties.values().any(|party| party.standing.is_empty())
            || jurisdictions.is_empty()
        {
            return Err(CourtValidationError::InvalidCase(identity));
        }
        Ok(Self {
            identity,
            parties,
            jurisdictions,
            claims,
            affected_titles,
            closure: CaseClosure::Open,
            policy,
            evidence: BTreeMap::new(),
            testimony: BTreeMap::new(),
            stage_records: BTreeMap::new(),
            conciliations: BTreeMap::new(),
            protective_orders: BTreeMap::new(),
            findings: BTreeMap::new(),
            hearings: BTreeMap::new(),
            judgments: BTreeMap::new(),
            remedies: BTreeMap::new(),
            appeals: BTreeMap::new(),
            constitutional_reviews: BTreeMap::new(),
            restitutions: BTreeMap::new(),
            recurrences: BTreeMap::new(),
        })
    }

    pub fn submit_evidence(
        &mut self,
        submission: EvidenceSubmission,
    ) -> Result<(), CourtValidationError> {
        self.ensure_open()?;
        submission.validate()?;
        if !self.jurisdictions.contains(&submission.jurisdiction)
            && !self.jurisdictions.contains(&CourtJurisdiction::CrossHouse)
            && !self
                .jurisdictions
                .contains(&CourtJurisdiction::Constitutional)
        {
            return Err(CourtValidationError::JurisdictionNotAssigned(
                submission.jurisdiction,
            ));
        }
        insert_unique(&mut self.evidence, submission.identity.clone(), submission)
    }

    pub fn submit_testimony(
        &mut self,
        testimony: TestimonyRecord,
    ) -> Result<(), CourtValidationError> {
        self.ensure_open()?;
        if !self.jurisdictions.contains(&testimony.jurisdiction)
            && !self.jurisdictions.contains(&CourtJurisdiction::CrossHouse)
            && !self
                .jurisdictions
                .contains(&CourtJurisdiction::Constitutional)
        {
            return Err(CourtValidationError::JurisdictionNotAssigned(
                testimony.jurisdiction,
            ));
        }
        insert_unique(&mut self.testimony, testimony.identity.clone(), testimony)
    }

    pub fn issue_protective_order(
        &mut self,
        order: ProtectiveOrder,
    ) -> Result<(), CourtValidationError> {
        self.ensure_case_identity(&order.case)?;
        self.ensure_open()?;
        order.validate()?;
        insert_unique(&mut self.protective_orders, order.identity.clone(), order)
    }

    pub fn record_conciliation(
        &mut self,
        stage: JudicialStageRecord,
        record: ConciliationRecord,
    ) -> Result<(), CourtValidationError> {
        self.ensure_stage(&stage, JudicialStage::Conciliation)?;
        self.ensure_case_identity(&record.case)?;
        if let Some(settlement) = &record.settlement {
            self.ensure_case_identity(&settlement.case)?;
            settlement.validate()?;
            if settlement
                .parties
                .iter()
                .any(|party| !self.parties.contains_key(party))
            {
                return Err(CourtValidationError::InvalidConciliationSettlement(
                    settlement.identity.clone(),
                ));
            }
        }
        insert_unique(&mut self.conciliations, record.identity.clone(), record)?;
        self.insert_stage(stage)
    }

    pub fn add_remedy(&mut self, remedy: Remedy) -> Result<(), CourtValidationError> {
        self.ensure_case_identity(&remedy.case)?;
        self.ensure_open()?;
        remedy.validate()?;
        insert_unique(&mut self.remedies, remedy.identity.clone(), remedy)
    }

    pub fn record_first_hearing(
        &mut self,
        stage: JudicialStageRecord,
        hearing: FirstHearingRecord,
    ) -> Result<(), CourtValidationError> {
        self.ensure_stage(&stage, JudicialStage::FirstHearing)?;
        self.ensure_case_identity(&hearing.case)?;
        if self.policy.conciliation_required
            && !self.has_stage_in_or_before_cycle(JudicialStage::Conciliation, stage.cycle)
        {
            return Err(CourtValidationError::MissingRequiredStage(
                JudicialStage::Conciliation,
            ));
        }
        if hearing.standing_confirmed.is_empty()
            || hearing
                .standing_confirmed
                .iter()
                .any(|party| !self.parties.contains_key(party))
            || hearing.jurisdictions_reviewed.is_empty()
            || hearing
                .jurisdictions_reviewed
                .iter()
                .any(|jurisdiction| !self.jurisdictions.contains(jurisdiction))
            || hearing.evidence_considered.is_empty()
            || hearing
                .evidence_considered
                .iter()
                .any(|evidence| !self.evidence.contains_key(evidence))
            || hearing
                .testimony_considered
                .iter()
                .any(|testimony| !self.testimony.contains_key(testimony))
            || (hearing.judgment.is_none() && !hearing.dismissed && hearing.referral.is_none())
        {
            return Err(CourtValidationError::InvalidFirstHearing(
                hearing.identity.clone(),
            ));
        }
        for finding in &hearing.findings {
            if finding.statement.trim().is_empty() || finding.evidence.is_empty() {
                return Err(CourtValidationError::InvalidFirstHearing(
                    hearing.identity.clone(),
                ));
            }
            insert_unique(
                &mut self.findings,
                finding.identity.clone(),
                finding.clone(),
            )?;
        }
        if let Some(judgment) = &hearing.judgment {
            self.ensure_case_identity(&judgment.case)?;
            judgment.validate()?;
            if judgment
                .findings
                .iter()
                .any(|finding| !self.findings.contains_key(finding))
                || judgment
                    .remedies
                    .iter()
                    .any(|remedy| !self.remedies.contains_key(remedy))
            {
                return Err(CourtValidationError::InvalidFirstHearing(
                    hearing.identity.clone(),
                ));
            }
            insert_unique(
                &mut self.judgments,
                judgment.identity.clone(),
                judgment.clone(),
            )?;
        }
        insert_unique(&mut self.hearings, hearing.identity.clone(), hearing)?;
        self.insert_stage(stage)
    }

    pub fn record_appeal(
        &mut self,
        stage: JudicialStageRecord,
        appeal: AppealRecord,
    ) -> Result<(), CourtValidationError> {
        self.ensure_stage(&stage, JudicialStage::Appeal)?;
        self.ensure_case_identity(&appeal.case)?;
        if !self.has_stage_in_or_before_cycle(JudicialStage::FirstHearing, stage.cycle)
            || !self.judgments.contains_key(&appeal.challenged_judgment)
        {
            return Err(CourtValidationError::MissingRequiredStage(
                JudicialStage::FirstHearing,
            ));
        }
        appeal.validate()?;
        if let Some(stay) = &appeal.stay
            && !self.protective_orders.contains_key(stay)
        {
            return Err(CourtValidationError::UnknownRecord(stay.clone()));
        }
        insert_unique(&mut self.appeals, appeal.identity.clone(), appeal)?;
        self.insert_stage(stage)
    }

    pub fn record_constitutional_review(
        &mut self,
        stage: JudicialStageRecord,
        review: ConstitutionalReviewRecord,
    ) -> Result<(), CourtValidationError> {
        self.ensure_stage(&stage, JudicialStage::ConstitutionalReview)?;
        self.ensure_case_identity(&review.case)?;
        if self.policy.full_review_cycle_required
            && !self.has_stage_in_or_before_cycle(JudicialStage::Appeal, stage.cycle)
        {
            return Err(CourtValidationError::MissingRequiredStage(
                JudicialStage::Appeal,
            ));
        }
        review.validate()?;
        insert_unique(
            &mut self.constitutional_reviews,
            review.identity.clone(),
            review,
        )?;
        self.insert_stage(stage)
    }

    pub fn record_restitution(
        &mut self,
        stage: JudicialStageRecord,
        restitution: RestitutionRecord,
        recurrence: Option<CaseRecurrence>,
    ) -> Result<(), CourtValidationError> {
        self.ensure_stage(&stage, JudicialStage::Restitution)?;
        self.ensure_case_identity(&restitution.case)?;
        let has_settlement = self
            .conciliations
            .values()
            .any(|record| record.settlement.is_some());
        let has_review =
            self.has_stage_in_or_before_cycle(JudicialStage::ConstitutionalReview, stage.cycle);
        if !has_settlement && !has_review {
            return Err(CourtValidationError::MissingRequiredStage(
                JudicialStage::ConstitutionalReview,
            ));
        }
        if !self.judgments.contains_key(&restitution.judgment) && !has_settlement {
            return Err(CourtValidationError::UnknownRecord(
                restitution.judgment.clone(),
            ));
        }
        if restitution
            .remedies
            .iter()
            .any(|remedy| !self.remedies.contains_key(remedy))
        {
            return Err(CourtValidationError::InvalidRestitution(
                restitution.identity.clone(),
            ));
        }
        restitution.validate()?;
        for prior_recurrence in self
            .recurrences
            .values()
            .filter(|prior| prior.to_cycle == restitution.cycle)
        {
            if !self.stage_records.values().any(|record| {
                record.cycle == prior_recurrence.to_cycle
                    && record.stage == prior_recurrence.return_stage
            }) {
                return Err(CourtValidationError::MissingRecurrenceStage(
                    prior_recurrence.return_stage,
                ));
            }
        }
        match (restitution.disposition.return_stage(), &recurrence) {
            (Some(return_stage), Some(recurrence)) => {
                self.ensure_case_identity(&recurrence.case)?;
                if recurrence.failed_restitution != restitution.identity
                    || recurrence.from_cycle != restitution.cycle
                    || recurrence.to_cycle != restitution.cycle + 1
                    || recurrence.return_stage != return_stage
                    || recurrence.lawful_reason.trim().is_empty()
                    || recurrence.evidence.is_empty()
                {
                    return Err(CourtValidationError::InvalidRecurrence(
                        recurrence.identity.clone(),
                    ));
                }
            }
            (None, None) => {}
            _ => {
                return Err(CourtValidationError::RestitutionRecurrenceMismatch(
                    restitution.identity.clone(),
                ));
            }
        }
        insert_unique(
            &mut self.restitutions,
            restitution.identity.clone(),
            restitution.clone(),
        )?;
        if let Some(recurrence) = recurrence {
            insert_unique(
                &mut self.recurrences,
                recurrence.identity.clone(),
                recurrence,
            )?;
        }
        self.insert_stage(stage)?;
        if restitution.disposition.closes_case() {
            self.closure = CaseClosure::EquilibriumConfirmed;
        }
        Ok(())
    }

    #[must_use]
    pub fn semantic_stage_history(&self) -> Vec<&JudicialStageRecord> {
        let mut records = self.stage_records.values().collect::<Vec<_>>();
        records.sort_by_key(|record| record.semantic_key());
        records
    }

    #[must_use]
    pub fn evidence(&self) -> &BTreeMap<IdentityId, EvidenceSubmission> {
        &self.evidence
    }

    #[must_use]
    pub fn remedies(&self) -> &BTreeMap<IdentityId, Remedy> {
        &self.remedies
    }

    #[must_use]
    pub fn restitutions(&self) -> &BTreeMap<IdentityId, RestitutionRecord> {
        &self.restitutions
    }

    #[must_use]
    pub fn recurrences(&self) -> &BTreeMap<IdentityId, CaseRecurrence> {
        &self.recurrences
    }

    #[must_use]
    pub fn protective_orders(&self) -> &BTreeMap<IdentityId, ProtectiveOrder> {
        &self.protective_orders
    }

    #[must_use]
    pub fn judgment(&self, identity: &IdentityId) -> Option<&Judgment> {
        self.judgments.get(identity)
    }

    fn ensure_open(&self) -> Result<(), CourtValidationError> {
        if self.closure == CaseClosure::Open {
            Ok(())
        } else {
            Err(CourtValidationError::CaseAlreadyClosed(
                self.identity.clone(),
            ))
        }
    }

    fn ensure_case_identity(&self, case: &IdentityId) -> Result<(), CourtValidationError> {
        if case == &self.identity {
            Ok(())
        } else {
            Err(CourtValidationError::CaseIdentityMismatch {
                expected: self.identity.clone(),
                actual: case.clone(),
            })
        }
    }

    fn ensure_stage(
        &self,
        record: &JudicialStageRecord,
        expected: JudicialStage,
    ) -> Result<(), CourtValidationError> {
        self.ensure_open()?;
        self.ensure_case_identity(&record.case)?;
        if record.stage != expected || record.evidence.is_empty() {
            return Err(CourtValidationError::InvalidStageRecord(
                record.identity.clone(),
            ));
        }
        Ok(())
    }

    fn insert_stage(&mut self, record: JudicialStageRecord) -> Result<(), CourtValidationError> {
        insert_unique(&mut self.stage_records, record.identity.clone(), record)
    }

    fn has_stage_in_or_before_cycle(&self, stage: JudicialStage, cycle: u32) -> bool {
        self.stage_records
            .values()
            .any(|record| record.stage == stage && record.cycle <= cycle)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MinoanCountyCourtSystem {
    pub identity: InstitutionId,
    pub hosted_by_minoans: bool,
    pub owns_house_law: bool,
    pub is_house: bool,
    pub jurisdictions: BTreeSet<CourtJurisdiction>,
    cases: BTreeMap<IdentityId, CourtCase>,
}

impl Default for MinoanCountyCourtSystem {
    fn default() -> Self {
        Self {
            identity: minoan_county_court_system_id(),
            hosted_by_minoans: true,
            owns_house_law: false,
            is_house: false,
            jurisdictions: CourtJurisdiction::ALL.into_iter().collect(),
            cases: BTreeMap::new(),
        }
    }
}

impl MinoanCountyCourtSystem {
    pub fn validate(&self) -> Result<(), CourtValidationError> {
        if self.identity != minoan_county_court_system_id()
            || !self.hosted_by_minoans
            || self.owns_house_law
            || self.is_house
            || self.jurisdictions != CourtJurisdiction::ALL.into_iter().collect::<BTreeSet<_>>()
        {
            return Err(CourtValidationError::InvalidCourtSystem);
        }
        Ok(())
    }

    pub fn open_case(&mut self, case: CourtCase) -> Result<(), CourtValidationError> {
        self.validate()?;
        if case
            .jurisdictions
            .iter()
            .any(|jurisdiction| !self.jurisdictions.contains(jurisdiction))
        {
            return Err(CourtValidationError::InvalidCase(case.identity));
        }
        insert_unique(&mut self.cases, case.identity.clone(), case)
    }

    #[must_use]
    pub fn case(&self, identity: &IdentityId) -> Option<&CourtCase> {
        self.cases.get(identity)
    }

    #[must_use]
    pub fn case_mut(&mut self, identity: &IdentityId) -> Option<&mut CourtCase> {
        self.cases.get_mut(identity)
    }

    #[must_use]
    pub fn case_count(&self) -> usize {
        self.cases.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AmendmentScope {
    HouseLocal(House),
    CrossHouse(BTreeSet<House>),
    Foundational,
}

impl AmendmentScope {
    #[must_use]
    pub fn required_houses(&self) -> BTreeSet<House> {
        match self {
            Self::HouseLocal(house) => BTreeSet::from([*house]),
            Self::CrossHouse(houses) => houses.clone(),
            Self::Foundational => BTreeSet::from([
                House::Stonebend,
                House::Sandmanor,
                House::Glaushouse,
                House::Flynt,
            ]),
        }
    }

    pub fn validate(&self) -> Result<(), CourtValidationError> {
        match self {
            Self::CrossHouse(houses) if houses.len() < 2 => {
                Err(CourtValidationError::InvalidAmendmentScope)
            }
            _ => Ok(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AmendmentProposal {
    pub identity: IdentityId,
    pub exact_text: String,
    pub superseded_text: Vec<String>,
    pub purpose: String,
    pub affected_houses: BTreeSet<House>,
    pub affected_offices: BTreeSet<IdentityId>,
    pub affected_titles: BTreeSet<TitleRecordId>,
    pub affected_communities: BTreeSet<IdentityId>,
    pub altered_authority: String,
    pub expected_yield: Vec<EvidenceRecordId>,
    pub scope: AmendmentScope,
    pub public_notice: Vec<EvidenceRecordId>,
    pub affected_party_testimony: Vec<EvidenceRecordId>,
    pub bundled_unrelated_changes: bool,
}

impl AmendmentProposal {
    pub fn validate(&self) -> Result<(), CourtValidationError> {
        self.scope.validate()?;
        if self.exact_text.trim().is_empty()
            || self.purpose.trim().is_empty()
            || self.altered_authority.trim().is_empty()
            || self.expected_yield.is_empty()
            || self.public_notice.is_empty()
            || self.affected_party_testimony.is_empty()
            || self.bundled_unrelated_changes
            || self.affected_houses != self.scope.required_houses()
        {
            return Err(CourtValidationError::InvalidAmendmentProposal(
                self.identity.clone(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AmendmentStage {
    Proposal,
    PublicNotice,
    AffectedPartyTestimony,
    Conciliation,
    ProceduralHearing,
    Appeal,
    ConstitutionalReview,
    Ratification,
    StonebendSeal,
    Implementation,
    RestitutionReview,
    Equilibrium,
}

impl AmendmentStage {
    #[must_use]
    pub const fn semantic_order(self) -> u8 {
        match self {
            Self::Proposal => 0,
            Self::PublicNotice => 1,
            Self::AffectedPartyTestimony => 2,
            Self::Conciliation => 3,
            Self::ProceduralHearing => 4,
            Self::Appeal => 5,
            Self::ConstitutionalReview => 6,
            Self::Ratification => 7,
            Self::StonebendSeal => 8,
            Self::Implementation => 9,
            Self::RestitutionReview => 10,
            Self::Equilibrium => 11,
        }
    }
}

#[must_use]
pub fn semantic_amendment_history(records: &[AmendmentStageRecord]) -> Vec<&AmendmentStageRecord> {
    let mut ordered = records.iter().collect::<Vec<_>>();
    ordered.sort_by_key(|record| (record.stage.semantic_order(), record.identity.as_str()));
    ordered
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AmendmentStageRecord {
    pub identity: IdentityId,
    pub proposal: IdentityId,
    pub stage: AmendmentStage,
    pub evidence: Vec<EvidenceRecordId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AmendmentProcessCertification {
    pub identity: IdentityId,
    pub proposal: IdentityId,
    pub court_system: InstitutionId,
    pub disposition: ConstitutionalReviewDisposition,
    pub evidence: Vec<EvidenceRecordId>,
    pub court_ratified: bool,
    pub path_lawful: bool,
}

impl AmendmentProcessCertification {
    pub fn validate(&self) -> Result<(), CourtValidationError> {
        if self.court_system != minoan_county_court_system_id()
            || self.evidence.is_empty()
            || self.court_ratified
            || !matches!(
                self.disposition,
                ConstitutionalReviewDisposition::EligibleForRatification
                    | ConstitutionalReviewDisposition::NotEligibleForRatification
                    | ConstitutionalReviewDisposition::ProcessDefective
                    | ConstitutionalReviewDisposition::ReturnedForCorrection
            )
            || (self.path_lawful
                != matches!(
                    self.disposition,
                    ConstitutionalReviewDisposition::EligibleForRatification
                ))
        {
            return Err(CourtValidationError::InvalidAmendmentCertification(
                self.identity.clone(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RatificationRecord {
    pub identity: IdentityId,
    pub proposal: IdentityId,
    pub scope: AmendmentScope,
    pub house_assents: BTreeMap<House, IdentityId>,
    pub evidence: Vec<EvidenceRecordId>,
    pub court_enacted: bool,
    pub central_junction_counted_as_house: bool,
}

impl RatificationRecord {
    pub fn validate(&self) -> Result<(), CourtValidationError> {
        self.scope.validate()?;
        if self.evidence.is_empty()
            || self.court_enacted
            || self.central_junction_counted_as_house
            || self.house_assents.keys().copied().collect::<BTreeSet<_>>()
                != self.scope.required_houses()
        {
            return Err(CourtValidationError::InvalidRatification(
                self.identity.clone(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AmendmentSealRecord {
    pub identity: SealRecordId,
    pub proposal: IdentityId,
    pub ratification: IdentityId,
    pub final_text: String,
    pub stonebend_record_authority: IdentityId,
    pub evidence: Vec<EvidenceRecordId>,
}

impl AmendmentSealRecord {
    pub fn validate(&self) -> Result<(), CourtValidationError> {
        if self.final_text.trim().is_empty() || self.evidence.is_empty() {
            return Err(CourtValidationError::InvalidAmendmentSeal(
                self.identity.clone(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AmendmentImplementationReview {
    pub identity: IdentityId,
    pub proposal: IdentityId,
    pub ratified_text: String,
    pub implemented_text: String,
    pub affected_title_updates: BTreeSet<TitleRecordId>,
    pub affected_house_evidence: BTreeMap<House, EvidenceRecordId>,
    pub notice_and_transition_evidence: Vec<EvidenceRecordId>,
    pub yield_evidence: Vec<EvidenceRecordId>,
    pub restitution_disposition: RestitutionDisposition,
    pub equilibrium: EquilibriumAssessment,
}

impl AmendmentImplementationReview {
    pub fn validate(&self) -> Result<(), CourtValidationError> {
        if self.ratified_text.trim().is_empty()
            || self.implemented_text.trim().is_empty()
            || self.notice_and_transition_evidence.is_empty()
            || self.yield_evidence.is_empty()
        {
            return Err(CourtValidationError::InvalidAmendmentImplementation(
                self.identity.clone(),
            ));
        }
        if self.ratified_text != self.implemented_text
            && self.restitution_disposition == RestitutionDisposition::EquilibriumConfirmed
        {
            return Err(CourtValidationError::RatifiedTextSilentlyRewritten(
                self.identity.clone(),
            ));
        }
        if self.restitution_disposition == RestitutionDisposition::EquilibriumConfirmed
            && !self.equilibrium.can_hold()
        {
            return Err(CourtValidationError::EquilibriumNotEstablished(
                self.identity.clone(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AmendmentProcess {
    pub proposal: AmendmentProposal,
    stage_records: BTreeMap<IdentityId, AmendmentStageRecord>,
    pub certification: Option<AmendmentProcessCertification>,
    pub ratification: Option<RatificationRecord>,
    pub seal: Option<AmendmentSealRecord>,
    pub implementation_review: Option<AmendmentImplementationReview>,
}

impl AmendmentProcess {
    pub fn new(proposal: AmendmentProposal) -> Result<Self, CourtValidationError> {
        proposal.validate()?;
        Ok(Self {
            proposal,
            stage_records: BTreeMap::new(),
            certification: None,
            ratification: None,
            seal: None,
            implementation_review: None,
        })
    }

    pub fn add_stage(&mut self, record: AmendmentStageRecord) -> Result<(), CourtValidationError> {
        if record.proposal != self.proposal.identity || record.evidence.is_empty() {
            return Err(CourtValidationError::InvalidAmendmentStage(
                record.identity.clone(),
            ));
        }
        insert_unique(&mut self.stage_records, record.identity.clone(), record)
    }

    pub fn certify(
        &mut self,
        certification: AmendmentProcessCertification,
    ) -> Result<(), CourtValidationError> {
        if certification.proposal != self.proposal.identity
            || !self.has_stage(AmendmentStage::PublicNotice)
            || !self.has_stage(AmendmentStage::AffectedPartyTestimony)
            || !self.has_stage(AmendmentStage::ProceduralHearing)
            || !self.has_stage(AmendmentStage::ConstitutionalReview)
        {
            return Err(CourtValidationError::AmendmentStageMissing);
        }
        certification.validate()?;
        self.certification = Some(certification);
        Ok(())
    }

    pub fn ratify(&mut self, ratification: RatificationRecord) -> Result<(), CourtValidationError> {
        if ratification.proposal != self.proposal.identity
            || ratification.scope != self.proposal.scope
            || !self
                .certification
                .as_ref()
                .is_some_and(|certification| certification.path_lawful)
        {
            return Err(CourtValidationError::RatificationBeforeCertification);
        }
        ratification.validate()?;
        self.ratification = Some(ratification);
        Ok(())
    }

    pub fn seal(&mut self, seal: AmendmentSealRecord) -> Result<(), CourtValidationError> {
        let ratification = self
            .ratification
            .as_ref()
            .ok_or(CourtValidationError::SealBeforeRatification)?;
        if seal.proposal != self.proposal.identity
            || seal.ratification != ratification.identity
            || seal.final_text != self.proposal.exact_text
        {
            return Err(CourtValidationError::SealBeforeRatification);
        }
        seal.validate()?;
        self.seal = Some(seal);
        Ok(())
    }

    pub fn review_implementation(
        &mut self,
        review: AmendmentImplementationReview,
    ) -> Result<(), CourtValidationError> {
        if self.seal.is_none()
            || review.proposal != self.proposal.identity
            || review.ratified_text != self.proposal.exact_text
        {
            return Err(CourtValidationError::ImplementationBeforeSeal);
        }
        review.validate()?;
        self.implementation_review = Some(review);
        Ok(())
    }

    #[must_use]
    pub fn semantic_stage_history(&self) -> Vec<&AmendmentStageRecord> {
        let mut records = self.stage_records.values().collect::<Vec<_>>();
        records.sort_by_key(|record| (record.stage.semantic_order(), record.identity.as_str()));
        records
    }

    fn has_stage(&self, stage: AmendmentStage) -> bool {
        self.stage_records
            .values()
            .any(|record| record.stage == stage)
    }
}

fn insert_unique<K, V>(
    records: &mut BTreeMap<K, V>,
    identity: K,
    record: V,
) -> Result<(), CourtValidationError>
where
    K: Ord + Clone + IntoIdentity,
{
    if records.insert(identity.clone(), record).is_some() {
        return Err(CourtValidationError::DuplicateRecord(
            identity.into_identity(),
        ));
    }
    Ok(())
}

trait IntoIdentity {
    fn into_identity(self) -> IdentityId;
}

impl IntoIdentity for IdentityId {
    fn into_identity(self) -> IdentityId {
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CourtValidationError {
    InvalidCourtSystem,
    InvalidCase(IdentityId),
    CaseAlreadyClosed(IdentityId),
    CaseIdentityMismatch {
        expected: IdentityId,
        actual: IdentityId,
    },
    DuplicateRecord(IdentityId),
    UnknownRecord(IdentityId),
    JurisdictionNotAssigned(CourtJurisdiction),
    InvalidDomainEvidence(IdentityId),
    InvalidStageRecord(IdentityId),
    MissingRequiredStage(JudicialStage),
    InvalidConciliationSettlement(IdentityId),
    UnboundedProtectiveOrder(IdentityId),
    InvalidFirstHearing(IdentityId),
    InvalidRemedy(IdentityId),
    JudicialAppropriation(IdentityId),
    InvalidAppeal(IdentityId),
    InvalidConstitutionalReview(IdentityId),
    InvalidRestitution(IdentityId),
    EquilibriumNotEstablished(IdentityId),
    InvalidRecurrence(IdentityId),
    RestitutionRecurrenceMismatch(IdentityId),
    MissingRecurrenceStage(JudicialStage),
    InvalidAmendmentScope,
    InvalidAmendmentProposal(IdentityId),
    InvalidAmendmentStage(IdentityId),
    AmendmentStageMissing,
    InvalidAmendmentCertification(IdentityId),
    InvalidRatification(IdentityId),
    RatificationBeforeCertification,
    InvalidAmendmentSeal(SealRecordId),
    SealBeforeRatification,
    InvalidAmendmentImplementation(IdentityId),
    RatifiedTextSilentlyRewritten(IdentityId),
    ImplementationBeforeSeal,
}

impl fmt::Display for CourtValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Minoan Court constitutional violation: {self:?}")
    }
}

impl std::error::Error for CourtValidationError {}
