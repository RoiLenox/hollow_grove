//! Stonebend Third Pass: Title lifecycle and constitutional continuity.
//!
//! This module is a Stonebend policy adapter over the repository's existing
//! constitutional event discipline, stable identity types, Second Pass Title
//! core and gate scopes, office tenures, challenges, and Tombstones. It does
//! not replace the Bond aggregate or create another universal lifecycle
//! engine.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::constitutional::ChallengeId;
use crate::institution::{IdentityId, InstitutionId};
use crate::world::stonebend::second_pass::{
    ConstitutionalDimension, DiamondState, GateFailureKind, GateScope, OfficeEnding,
    OfficeTombstone, ProliteriateNetwork, StonebendConstitutionalPower, StonebendGateFacing,
    StonebendTitleCore, TitleScopeDisposition, YieldRecord, diamond_title_id,
};
use crate::world::stonebend::{
    EvidenceRecordId, SealRecordId, TitleRecordId, TombstoneRecordId, high_freemason_office_id,
};

pub const STONEBEND_THIRD_PASS_SOURCE: &str =
    "STONEBEND_TITLE_LIFECYCLE_AND_CONSTITUTIONAL_CONTINUITY_V1.md";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TitleLifecycleStage {
    NameEstablished,
    ClaimPresented,
    ApplicationSubmitted,
    EvidenceAssembled,
    EligibilityReviewed,
    GateReviewed,
    ChallengePeriod,
    Recognized,
    Invested,
    Active,
    Maintenance,
    RenewalReview,
    Limited,
    Supervised,
    Suspended,
    Remediation,
    Restored,
    Ended,
}

impl TitleLifecycleStage {
    pub const ALL: [Self; 18] = [
        Self::NameEstablished,
        Self::ClaimPresented,
        Self::ApplicationSubmitted,
        Self::EvidenceAssembled,
        Self::EligibilityReviewed,
        Self::GateReviewed,
        Self::ChallengePeriod,
        Self::Recognized,
        Self::Invested,
        Self::Active,
        Self::Maintenance,
        Self::RenewalReview,
        Self::Limited,
        Self::Supervised,
        Self::Suspended,
        Self::Remediation,
        Self::Restored,
        Self::Ended,
    ];

    #[must_use]
    pub const fn semantic_order(self) -> u8 {
        match self {
            Self::NameEstablished => 0,
            Self::ClaimPresented => 1,
            Self::ApplicationSubmitted => 2,
            Self::EvidenceAssembled => 3,
            Self::EligibilityReviewed => 4,
            Self::GateReviewed => 5,
            Self::ChallengePeriod => 6,
            Self::Recognized => 7,
            Self::Invested => 8,
            Self::Active => 9,
            Self::Maintenance => 10,
            Self::RenewalReview => 11,
            Self::Limited => 12,
            Self::Supervised => 13,
            Self::Suspended => 14,
            Self::Remediation => 15,
            Self::Restored => 16,
            Self::Ended => 17,
        }
    }

    #[must_use]
    pub const fn is_pre_recognition(self) -> bool {
        self.semantic_order() < Self::Recognized.semantic_order()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TitleStageRecord {
    pub identity: IdentityId,
    pub title: TitleRecordId,
    pub stage: TitleLifecycleStage,
    pub evidence: Vec<EvidenceRecordId>,
    pub sequence: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActivationMode {
    Explicit,
    AutomaticWhenRequirementsSatisfied,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ActivationRequirement {
    Investiture,
    GateScope(GateScope),
    ActiveLicense(IdentityId),
    SealDelivered(SealRecordId),
    ResponsibilityAccepted,
    FinalCondition(IdentityId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TitleMaintenanceRequirement {
    ContinuingCompetence,
    PhysicalUpkeep,
    ValidCustody,
    CurrentProof,
    LawfulOperation,
    EssentialFormPreserved,
    RequiredSupervision,
    UpdatedProvenance,
    PublicAccountability,
    OrdinaryRepair,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TitleRenewalPolicy {
    NotRequired,
    ConditionBased {
        policy: IdentityId,
        condition: String,
        expires_without_renewal: bool,
    },
}

impl TitleRenewalPolicy {
    #[must_use]
    pub fn policy_identity(&self) -> Option<&IdentityId> {
        match self {
            Self::NotRequired => None,
            Self::ConditionBased { policy, .. } => Some(policy),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChallengePeriodPolicy {
    pub eligible_challengers: String,
    pub affected_nodes: BTreeSet<IdentityId>,
    pub closing_condition: String,
    pub activation_stayed: bool,
    pub resolving_authority: IdentityId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TitleLifecyclePolicy {
    pub identity: IdentityId,
    pub required_pre_recognition_stages: BTreeSet<TitleLifecycleStage>,
    pub required_gate_scopes: BTreeSet<GateScope>,
    pub activation_mode: ActivationMode,
    pub activation_requirements: BTreeSet<ActivationRequirement>,
    pub maintenance_requirements: BTreeSet<TitleMaintenanceRequirement>,
    pub renewal: TitleRenewalPolicy,
    pub challenge_period: Option<ChallengePeriodPolicy>,
    pub succession_supported: bool,
    pub direct_restoration_allowed: bool,
    pub permitted_terminal_dispositions: BTreeSet<TitleTerminalDisposition>,
}

impl TitleLifecyclePolicy {
    pub fn validate(&self) -> Result<(), ThirdPassValidationError> {
        if self.identity.as_str().trim().is_empty()
            || self
                .required_pre_recognition_stages
                .iter()
                .any(|stage| !stage.is_pre_recognition())
        {
            return Err(ThirdPassValidationError::InvalidLifecyclePolicy);
        }
        if self.challenge_period.is_some()
            && !self
                .required_pre_recognition_stages
                .contains(&TitleLifecycleStage::ChallengePeriod)
        {
            return Err(ThirdPassValidationError::InvalidLifecyclePolicy);
        }
        if self
            .activation_requirements
            .iter()
            .filter_map(|requirement| match requirement {
                ActivationRequirement::GateScope(scope) => Some(*scope),
                _ => None,
            })
            .any(|scope| !self.required_gate_scopes.contains(&scope))
        {
            return Err(ThirdPassValidationError::InvalidLifecyclePolicy);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TitleExerciseState {
    ClaimPending,
    RecognizedInactive,
    Active,
    Ended,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TitleRecognition {
    pub identity: IdentityId,
    pub title: TitleRecordId,
    pub authority: IdentityId,
    pub evidence: Vec<EvidenceRecordId>,
    pub boundary: String,
    pub sequence: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TitleActivation {
    pub identity: IdentityId,
    pub title: TitleRecordId,
    pub satisfied_requirements: BTreeSet<ActivationRequirement>,
    pub authority: IdentityId,
    pub evidence: Vec<EvidenceRecordId>,
    pub accepted_responsibility: bool,
    pub term: IdentityId,
    pub sequence: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TitleTermEnding {
    Renewal(IdentityId),
    Terminal(TitleTerminalDisposition),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TitleTerm {
    pub identity: IdentityId,
    pub title: TitleRecordId,
    pub began_at: u64,
    pub ended_at: Option<u64>,
    pub ending: Option<TitleTermEnding>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TitleMaintenanceRecord {
    pub identity: IdentityId,
    pub title: TitleRecordId,
    pub term: IdentityId,
    pub subject: IdentityId,
    pub satisfied_requirements: BTreeSet<TitleMaintenanceRequirement>,
    pub evidence: Vec<EvidenceRecordId>,
    pub reviewed_by: IdentityId,
    pub sequence: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TitleRenewalDisposition {
    Renewed,
    RenewedWithLimitations,
    RemediationRequired,
    Deferred,
    Rejected,
    Expired,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TitleRenewalRecord {
    pub identity: IdentityId,
    pub title: TitleRecordId,
    pub current_term: IdentityId,
    pub renewal_policy: IdentityId,
    pub evidence: Vec<EvidenceRecordId>,
    pub gate_scopes_reviewed: BTreeSet<GateScope>,
    pub maintenance_records: Vec<IdentityId>,
    pub known_failures: Vec<GateFailureKind>,
    pub known_remediation: Vec<IdentityId>,
    pub yield_evidence: Vec<EvidenceRecordId>,
    pub renewal_authority: IdentityId,
    pub disposition: TitleRenewalDisposition,
    pub renewed_boundaries: String,
    pub effective_sequence: u64,
    pub next_term: Option<IdentityId>,
    pub prior_term_tombstone: Option<TombstoneRecordId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TitleInterventionTarget {
    CoreTitle,
    GateScope(GateScope),
    Activation,
    License(IdentityId),
    BearerTenure(IdentityId),
    Mandate(IdentityId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TitleInterventionKind {
    Limitation,
    Supervision,
    Suspension,
    Remediation,
    Removal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupervisionTerms {
    pub supervising_authority: IdentityId,
    pub supervised_scope: String,
    pub required_evidence: Vec<EvidenceRecordId>,
    pub completion_or_review_condition: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConstitutionalReferral {
    FutureCourt,
    FutureCriminalProcess,
    FutureIllegalHollowingProcedure,
    FutureRestitutionProcess,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TitleIntervention {
    pub identity: IdentityId,
    pub title: TitleRecordId,
    pub target: TitleInterventionTarget,
    pub kind: TitleInterventionKind,
    pub failure: GateFailureKind,
    pub evidence: Vec<EvidenceRecordId>,
    pub boundary: String,
    pub supervision: Option<SupervisionTerms>,
    pub remediation_condition: Option<String>,
    pub referral: Option<ConstitutionalReferral>,
    pub core_challenge: Option<ChallengeId>,
    pub sequence: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestorationRecord {
    pub identity: IdentityId,
    pub title: TitleRecordId,
    pub target: TitleInterventionTarget,
    pub interruption: IdentityId,
    pub cause: GateFailureKind,
    pub remediation_evidence: Vec<EvidenceRecordId>,
    pub reviewing_authority: IdentityId,
    pub restored_scopes: BTreeSet<GateScope>,
    pub continuing_limitations: Vec<String>,
    pub new_effective_boundary: String,
    pub sequence: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TitleTerminalDisposition {
    HonorableCompletion,
    Surrender,
    Expiration,
    Death,
    EndOfForm,
    Succession,
    RemovalForFailure,
    RemovalForFraud,
    RemovalForIllegality,
    ConstitutionalDissolution,
    Supersession,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TitleTermTombstone {
    pub record: TombstoneRecordId,
    pub title: TitleRecordId,
    pub term: IdentityId,
    pub disposition: TitleTerminalDisposition,
    pub sequence: u64,
    pub evidence: Vec<EvidenceRecordId>,
    pub successor: Option<IdentityId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StonebendTitleLifecycle {
    pub core: StonebendTitleCore,
    pub policy: TitleLifecyclePolicy,
    pub state: TitleExerciseState,
    stage_records: BTreeMap<TitleLifecycleStage, TitleStageRecord>,
    recognition: Option<TitleRecognition>,
    activation: Option<TitleActivation>,
    active_term: Option<IdentityId>,
    terms: BTreeMap<IdentityId, TitleTerm>,
    maintenance: BTreeMap<IdentityId, TitleMaintenanceRecord>,
    renewals: BTreeMap<IdentityId, TitleRenewalRecord>,
    interventions: BTreeMap<IdentityId, TitleIntervention>,
    restorations: BTreeMap<IdentityId, RestorationRecord>,
    tombstones: Vec<TitleTermTombstone>,
}

impl StonebendTitleLifecycle {
    pub fn new(
        core: StonebendTitleCore,
        policy: TitleLifecyclePolicy,
    ) -> Result<Self, ThirdPassValidationError> {
        policy.validate()?;
        Ok(Self {
            core,
            policy,
            state: TitleExerciseState::ClaimPending,
            stage_records: BTreeMap::new(),
            recognition: None,
            activation: None,
            active_term: None,
            terms: BTreeMap::new(),
            maintenance: BTreeMap::new(),
            renewals: BTreeMap::new(),
            interventions: BTreeMap::new(),
            restorations: BTreeMap::new(),
            tombstones: Vec::new(),
        })
    }

    pub fn record_stage(
        &mut self,
        record: TitleStageRecord,
    ) -> Result<(), ThirdPassValidationError> {
        if record.title != self.core.title || record.evidence.is_empty() {
            return Err(ThirdPassValidationError::InvalidStageRecord);
        }
        if matches!(
            record.stage,
            TitleLifecycleStage::Recognized
                | TitleLifecycleStage::Invested
                | TitleLifecycleStage::Active
                | TitleLifecycleStage::Maintenance
                | TitleLifecycleStage::RenewalReview
                | TitleLifecycleStage::Limited
                | TitleLifecycleStage::Supervised
                | TitleLifecycleStage::Suspended
                | TitleLifecycleStage::Remediation
                | TitleLifecycleStage::Restored
                | TitleLifecycleStage::Ended
        ) {
            return Err(
                ThirdPassValidationError::ManagedStageRequiresLifecycleAction(record.stage),
            );
        }
        if self.stage_records.insert(record.stage, record).is_some() {
            return Err(ThirdPassValidationError::DuplicateLifecycleStage);
        }
        Ok(())
    }

    #[must_use]
    pub fn ordered_stage_records(&self) -> Vec<&TitleStageRecord> {
        let mut records = self.stage_records.values().collect::<Vec<_>>();
        records.sort_by_key(|record| record.stage.semantic_order());
        records
    }

    fn require_recognition_prerequisites(&self) -> Result<(), ThirdPassValidationError> {
        for stage in &self.policy.required_pre_recognition_stages {
            if !self.stage_records.contains_key(stage) {
                return Err(ThirdPassValidationError::MissingLifecycleStage(*stage));
            }
        }
        for scope in &self.policy.required_gate_scopes {
            if !self.core.authorizes(*scope) {
                return Err(ThirdPassValidationError::RequiredGateScopeUnavailable(
                    *scope,
                ));
            }
        }
        Ok(())
    }

    pub fn recognize(
        &mut self,
        recognition: TitleRecognition,
    ) -> Result<(), ThirdPassValidationError> {
        if self.state != TitleExerciseState::ClaimPending
            || recognition.title != self.core.title
            || recognition.evidence.is_empty()
            || recognition.boundary.trim().is_empty()
        {
            return Err(ThirdPassValidationError::InvalidRecognition);
        }
        self.require_recognition_prerequisites()?;
        self.stage_records.insert(
            TitleLifecycleStage::Recognized,
            TitleStageRecord {
                identity: recognition.identity.clone(),
                title: recognition.title.clone(),
                stage: TitleLifecycleStage::Recognized,
                evidence: recognition.evidence.clone(),
                sequence: recognition.sequence,
            },
        );
        self.recognition = Some(recognition);
        self.state = TitleExerciseState::RecognizedInactive;
        Ok(())
    }

    pub fn activate(
        &mut self,
        activation: TitleActivation,
    ) -> Result<(), ThirdPassValidationError> {
        if self.state != TitleExerciseState::RecognizedInactive
            || activation.title != self.core.title
            || activation.evidence.is_empty()
            || !activation.accepted_responsibility
            || !activation
                .satisfied_requirements
                .is_superset(&self.policy.activation_requirements)
        {
            return Err(ThirdPassValidationError::ActivationRequirementsUnsatisfied);
        }
        for scope in &self.policy.required_gate_scopes {
            if !self.core.authorizes(*scope) {
                return Err(ThirdPassValidationError::RequiredGateScopeUnavailable(
                    *scope,
                ));
            }
        }
        if self.terms.contains_key(&activation.term) {
            return Err(ThirdPassValidationError::DuplicateTitleTerm);
        }
        if self
            .policy
            .activation_requirements
            .contains(&ActivationRequirement::Investiture)
        {
            self.stage_records.insert(
                TitleLifecycleStage::Invested,
                TitleStageRecord {
                    identity: activation.identity.clone(),
                    title: activation.title.clone(),
                    stage: TitleLifecycleStage::Invested,
                    evidence: activation.evidence.clone(),
                    sequence: activation.sequence,
                },
            );
        }
        self.stage_records.insert(
            TitleLifecycleStage::Active,
            TitleStageRecord {
                identity: activation.identity.clone(),
                title: activation.title.clone(),
                stage: TitleLifecycleStage::Active,
                evidence: activation.evidence.clone(),
                sequence: activation.sequence,
            },
        );
        self.terms.insert(
            activation.term.clone(),
            TitleTerm {
                identity: activation.term.clone(),
                title: self.core.title.clone(),
                began_at: activation.sequence,
                ended_at: None,
                ending: None,
            },
        );
        self.active_term = Some(activation.term.clone());
        self.activation = Some(activation);
        self.state = TitleExerciseState::Active;
        Ok(())
    }

    #[must_use]
    pub fn recognition(&self) -> Option<&TitleRecognition> {
        self.recognition.as_ref()
    }

    #[must_use]
    pub fn activation(&self) -> Option<&TitleActivation> {
        self.activation.as_ref()
    }

    #[must_use]
    pub fn may_exercise(&self) -> bool {
        self.state == TitleExerciseState::Active
    }

    pub fn record_maintenance(
        &mut self,
        record: TitleMaintenanceRecord,
    ) -> Result<(), ThirdPassValidationError> {
        if self.state != TitleExerciseState::Active
            || record.title != self.core.title
            || self.active_term.as_ref() != Some(&record.term)
            || record.evidence.is_empty()
            || !record
                .satisfied_requirements
                .is_superset(&self.policy.maintenance_requirements)
        {
            return Err(ThirdPassValidationError::MaintenanceRequirementsUnsatisfied);
        }
        if self
            .maintenance
            .insert(record.identity.clone(), record)
            .is_some()
        {
            return Err(ThirdPassValidationError::DuplicateMaintenanceRecord);
        }
        Ok(())
    }

    pub fn record_renewal(
        &mut self,
        record: TitleRenewalRecord,
    ) -> Result<(), ThirdPassValidationError> {
        let policy_id = self
            .policy
            .renewal
            .policy_identity()
            .ok_or(ThirdPassValidationError::RenewalNotRequired)?;
        if self.state != TitleExerciseState::Active
            || record.title != self.core.title
            || self.active_term.as_ref() != Some(&record.current_term)
            || &record.renewal_policy != policy_id
            || record.evidence.is_empty()
            || record
                .maintenance_records
                .iter()
                .any(|id| !self.maintenance.contains_key(id))
        {
            return Err(ThirdPassValidationError::InvalidRenewalRecord);
        }
        let renewed = matches!(
            record.disposition,
            TitleRenewalDisposition::Renewed | TitleRenewalDisposition::RenewedWithLimitations
        );
        if renewed != record.next_term.is_some() || renewed != record.prior_term_tombstone.is_some()
        {
            return Err(ThirdPassValidationError::RenewalTermMismatch);
        }
        if self.renewals.contains_key(&record.identity) {
            return Err(ThirdPassValidationError::DuplicateRenewalRecord);
        }
        if let Some(next_term) = &record.next_term {
            if self.terms.contains_key(next_term) {
                return Err(ThirdPassValidationError::DuplicateTitleTerm);
            }
            let current = self
                .terms
                .get_mut(&record.current_term)
                .expect("active term is always registered");
            current.ended_at = Some(record.effective_sequence);
            current.ending = Some(TitleTermEnding::Renewal(record.identity.clone()));
            self.terms.insert(
                next_term.clone(),
                TitleTerm {
                    identity: next_term.clone(),
                    title: self.core.title.clone(),
                    began_at: record.effective_sequence,
                    ended_at: None,
                    ending: None,
                },
            );
            self.active_term = Some(next_term.clone());
            self.tombstones.push(TitleTermTombstone {
                record: record
                    .prior_term_tombstone
                    .clone()
                    .expect("renewed term requires a Tombstone linkage"),
                title: self.core.title.clone(),
                term: record.current_term.clone(),
                disposition: TitleTerminalDisposition::Supersession,
                sequence: record.effective_sequence,
                evidence: record.evidence.clone(),
                successor: Some(next_term.clone()),
            });
        }
        self.renewals.insert(record.identity.clone(), record);
        Ok(())
    }

    pub fn apply_intervention(
        &mut self,
        intervention: TitleIntervention,
    ) -> Result<(), ThirdPassValidationError> {
        validate_intervention(&intervention, &self.core)?;
        if intervention.title != self.core.title {
            return Err(ThirdPassValidationError::InterventionTitleMismatch);
        }
        if self.interventions.contains_key(&intervention.identity) {
            return Err(ThirdPassValidationError::DuplicateIntervention);
        }
        match (&intervention.target, intervention.kind) {
            (TitleInterventionTarget::GateScope(scope), TitleInterventionKind::Limitation) => {
                self.core
                    .update_scope_disposition(*scope, TitleScopeDisposition::Limited)?;
            }
            (TitleInterventionTarget::GateScope(scope), TitleInterventionKind::Suspension) => {
                self.core
                    .update_scope_disposition(*scope, TitleScopeDisposition::Suspended)?;
            }
            (TitleInterventionTarget::GateScope(scope), TitleInterventionKind::Removal) => {
                self.core
                    .update_scope_disposition(*scope, TitleScopeDisposition::Removed)?;
            }
            (TitleInterventionTarget::Activation, TitleInterventionKind::Suspension) => {
                if self.state != TitleExerciseState::Active {
                    return Err(ThirdPassValidationError::NoActiveExercise);
                }
                self.state = TitleExerciseState::RecognizedInactive;
            }
            (TitleInterventionTarget::CoreTitle, TitleInterventionKind::Removal) => {
                return Err(ThirdPassValidationError::CoreRemovalRequiresTombstone);
            }
            _ => {}
        }
        self.interventions
            .insert(intervention.identity.clone(), intervention);
        Ok(())
    }

    pub fn restore(
        &mut self,
        restoration: RestorationRecord,
    ) -> Result<(), ThirdPassValidationError> {
        if restoration.title != self.core.title
            || restoration.remediation_evidence.is_empty()
            || restoration.new_effective_boundary.trim().is_empty()
        {
            return Err(ThirdPassValidationError::InvalidRestoration);
        }
        let interruption = self
            .interventions
            .get(&restoration.interruption)
            .ok_or(ThirdPassValidationError::UnknownInterruption)?;
        if interruption.target != restoration.target {
            return Err(ThirdPassValidationError::RestorationTargetMismatch);
        }
        if !matches!(
            interruption.kind,
            TitleInterventionKind::Limitation
                | TitleInterventionKind::Supervision
                | TitleInterventionKind::Suspension
                | TitleInterventionKind::Remediation
        ) {
            return Err(ThirdPassValidationError::InterruptionNotRestorable);
        }
        match &restoration.target {
            TitleInterventionTarget::GateScope(scope) => {
                let disposition = if restoration.continuing_limitations.is_empty() {
                    TitleScopeDisposition::Recognized
                } else {
                    TitleScopeDisposition::Limited
                };
                self.core.update_scope_disposition(*scope, disposition)?;
            }
            TitleInterventionTarget::Activation => {
                if self.activation.is_none() || self.state == TitleExerciseState::Ended {
                    return Err(ThirdPassValidationError::InvalidRestoration);
                }
                self.state = TitleExerciseState::Active;
            }
            _ => {}
        }
        if self
            .restorations
            .insert(restoration.identity.clone(), restoration)
            .is_some()
        {
            return Err(ThirdPassValidationError::DuplicateRestoration);
        }
        Ok(())
    }

    pub fn end_active_term(
        &mut self,
        tombstone: TitleTermTombstone,
    ) -> Result<(), ThirdPassValidationError> {
        if tombstone.title != self.core.title
            || self.active_term.as_ref() != Some(&tombstone.term)
            || tombstone.evidence.is_empty()
            || !self
                .policy
                .permitted_terminal_dispositions
                .contains(&tombstone.disposition)
        {
            return Err(ThirdPassValidationError::InvalidTerminalDisposition);
        }
        let term = self
            .terms
            .get_mut(&tombstone.term)
            .expect("active term is registered");
        term.ended_at = Some(tombstone.sequence);
        term.ending = Some(TitleTermEnding::Terminal(tombstone.disposition));
        self.active_term = None;
        self.state = TitleExerciseState::Ended;
        self.tombstones.push(tombstone);
        Ok(())
    }

    #[must_use]
    pub fn terms(&self) -> &BTreeMap<IdentityId, TitleTerm> {
        &self.terms
    }

    #[must_use]
    pub fn maintenance_records(&self) -> &BTreeMap<IdentityId, TitleMaintenanceRecord> {
        &self.maintenance
    }

    #[must_use]
    pub fn renewal_records(&self) -> &BTreeMap<IdentityId, TitleRenewalRecord> {
        &self.renewals
    }

    #[must_use]
    pub fn interventions(&self) -> &BTreeMap<IdentityId, TitleIntervention> {
        &self.interventions
    }

    #[must_use]
    pub fn restorations(&self) -> &BTreeMap<IdentityId, RestorationRecord> {
        &self.restorations
    }

    #[must_use]
    pub fn tombstones(&self) -> &[TitleTermTombstone] {
        &self.tombstones
    }
}

fn validate_intervention(
    intervention: &TitleIntervention,
    core: &StonebendTitleCore,
) -> Result<(), ThirdPassValidationError> {
    if intervention.title != core.title
        || intervention.evidence.is_empty()
        || intervention.boundary.trim().is_empty()
    {
        return Err(ThirdPassValidationError::InvalidIntervention);
    }
    if intervention.kind == TitleInterventionKind::Supervision {
        let terms = intervention
            .supervision
            .as_ref()
            .ok_or(ThirdPassValidationError::SupervisionTermsRequired)?;
        if terms.required_evidence.is_empty()
            || terms.supervised_scope.trim().is_empty()
            || terms.completion_or_review_condition.trim().is_empty()
        {
            return Err(ThirdPassValidationError::SupervisionTermsRequired);
        }
    } else if intervention.supervision.is_some() {
        return Err(ThirdPassValidationError::UnexpectedSupervisionTerms);
    }
    if intervention.kind == TitleInterventionKind::Remediation
        && intervention
            .remediation_condition
            .as_ref()
            .is_none_or(|condition| condition.trim().is_empty())
    {
        return Err(ThirdPassValidationError::RemediationConditionRequired);
    }
    if intervention.failure == GateFailureKind::HonestFailure
        && intervention.kind == TitleInterventionKind::Removal
    {
        return Err(ThirdPassValidationError::DisproportionateHonestFailureRemoval);
    }
    if intervention.failure == GateFailureKind::Illegality && intervention.referral.is_none() {
        return Err(ThirdPassValidationError::IllegalityReferralRequired);
    }
    if intervention.failure == GateFailureKind::ConstitutionalHollowness
        && intervention.target == TitleInterventionTarget::CoreTitle
    {
        let challenge = intervention
            .core_challenge
            .as_ref()
            .ok_or(ThirdPassValidationError::HollownessRequiresCoreReview)?;
        if !core.has_explicit_core_challenge(challenge) {
            return Err(ThirdPassValidationError::HollownessRequiresCoreReview);
        }
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestrictedRestorationSubject {
    RemovedHypergiant(IdentityId),
    RecalledProliteriateWitness(IdentityId),
    RemovedFreemason(IdentityId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RequiredReturnPath {
    CompleteLazerhornSuccession,
    NewNetworkMandate,
    IndependentForgeReplacement,
}

#[must_use]
pub const fn required_return_path(subject: &RestrictedRestorationSubject) -> RequiredReturnPath {
    match subject {
        RestrictedRestorationSubject::RemovedHypergiant(_) => {
            RequiredReturnPath::CompleteLazerhornSuccession
        }
        RestrictedRestorationSubject::RecalledProliteriateWitness(_) => {
            RequiredReturnPath::NewNetworkMandate
        }
        RestrictedRestorationSubject::RemovedFreemason(_) => {
            RequiredReturnPath::IndependentForgeReplacement
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ContinuityActionKind {
    MaintainExistingInfrastructure,
    FulfillExistingObligation,
    PreservePublicRecords,
    DefendImmediatelyThreatenedBoundary,
    CompleteRoutineGateProcessing,
    PreventCatastrophicMaterialFailure,
    PreserveSuccessionEvidence,
    InvestDiamond,
    CreateSovereignLaw,
    PermanentlyExpandGateScope,
    AppointHypergiant,
    EraseChallenge,
    RemovePrincipalPower,
}

impl ContinuityActionKind {
    #[must_use]
    pub const fn lawful_during_vacancy(self) -> bool {
        matches!(
            self,
            Self::MaintainExistingInfrastructure
                | Self::FulfillExistingObligation
                | Self::PreservePublicRecords
                | Self::DefendImmediatelyThreatenedBoundary
                | Self::CompleteRoutineGateProcessing
                | Self::PreventCatastrophicMaterialFailure
                | Self::PreserveSuccessionEvidence
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContinuityTerminationCondition {
    ExistingDutyCompleted,
    BoundaryStabilized,
    EmergencyResolved,
    MandatoryReviewEndsAuthority,
    DiamondInvestedThroughSuccession,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContinuityMandateStatus {
    Active,
    Terminated,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VacancyDimensionTrace {
    pub dimension: ConstitutionalDimension,
    pub constitutional_power: StonebendConstitutionalPower,
    pub constitutional_source: InstitutionId,
    pub delegated_actor: IdentityId,
    pub evidence: EvidenceRecordId,
}

impl VacancyDimensionTrace {
    pub fn validate(&self) -> Result<(), ThirdPassValidationError> {
        if self.constitutional_power.domain() != self.dimension
            || self.constitutional_source != self.constitutional_power.institution()
        {
            return Err(ThirdPassValidationError::InvalidContinuityTrace(
                self.dimension,
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiamondContinuityMandate {
    pub identity: IdentityId,
    pub vacancy_tombstone: TombstoneRecordId,
    pub diamond: TitleRecordId,
    pub existing_legal_authority: EvidenceRecordId,
    pub permitted_actions: BTreeSet<ContinuityActionKind>,
    pub responsible_administrator: IdentityId,
    pub affected_gate: Option<StonebendGateFacing>,
    pub start_condition: String,
    pub termination_condition: ContinuityTerminationCondition,
    pub evidence: Vec<EvidenceRecordId>,
    pub dimension_traces: BTreeMap<ConstitutionalDimension, VacancyDimensionTrace>,
    pub later_review_required: bool,
    pub status: ContinuityMandateStatus,
    actions: BTreeMap<IdentityId, EmergencyContinuityAction>,
}

impl DiamondContinuityMandate {
    // The constitutional opening record intentionally requires every bounded
    // authority, evidence, and termination field at construction.
    #[allow(clippy::too_many_arguments)]
    pub fn open(
        diamond: &DiamondState,
        identity: IdentityId,
        vacancy_tombstone: TombstoneRecordId,
        existing_legal_authority: EvidenceRecordId,
        permitted_actions: BTreeSet<ContinuityActionKind>,
        responsible_administrator: IdentityId,
        affected_gate: Option<StonebendGateFacing>,
        start_condition: impl Into<String>,
        termination_condition: ContinuityTerminationCondition,
        evidence: Vec<EvidenceRecordId>,
        dimension_traces: Vec<VacancyDimensionTrace>,
    ) -> Result<Self, ThirdPassValidationError> {
        let start_condition = start_condition.into();
        if !diamond.is_vacant() {
            return Err(ThirdPassValidationError::DiamondNotVacant);
        }
        if permitted_actions.is_empty()
            || permitted_actions
                .iter()
                .any(|action| !action.lawful_during_vacancy())
            || start_condition.trim().is_empty()
            || evidence.is_empty()
        {
            return Err(ThirdPassValidationError::InvalidContinuityMandate);
        }
        let mut traces = BTreeMap::new();
        for trace in dimension_traces {
            trace.validate()?;
            if traces.insert(trace.dimension, trace).is_some() {
                return Err(ThirdPassValidationError::DuplicateContinuityDimension);
            }
        }
        if traces.keys().copied().collect::<BTreeSet<_>>()
            != [
                ConstitutionalDimension::Claim,
                ConstitutionalDimension::Title,
                ConstitutionalDimension::Yield,
            ]
            .into_iter()
            .collect()
        {
            return Err(ThirdPassValidationError::IncompleteContinuityDimensions);
        }
        Ok(Self {
            identity,
            vacancy_tombstone,
            diamond: diamond.title.clone(),
            existing_legal_authority,
            permitted_actions,
            responsible_administrator,
            affected_gate,
            start_condition,
            termination_condition,
            evidence,
            dimension_traces: traces,
            later_review_required: true,
            status: ContinuityMandateStatus::Active,
            actions: BTreeMap::new(),
        })
    }

    pub fn record_action(
        &mut self,
        action: EmergencyContinuityAction,
    ) -> Result<(), ThirdPassValidationError> {
        if self.status != ContinuityMandateStatus::Active
            || action.mandate != self.identity
            || action.evidence.is_empty()
            || action.emergency.trim().is_empty()
            || action.constitutional_basis.trim().is_empty()
            || !action.later_review_required
            || !action.action.lawful_during_vacancy()
            || !self.permitted_actions.contains(&action.action)
        {
            return Err(ThirdPassValidationError::UnauthorizedContinuityAction);
        }
        if self
            .actions
            .insert(action.identity.clone(), action)
            .is_some()
        {
            return Err(ThirdPassValidationError::DuplicateContinuityAction);
        }
        Ok(())
    }

    pub fn terminate(
        &mut self,
        condition: ContinuityTerminationCondition,
        review_evidence: EvidenceRecordId,
    ) -> Result<(), ThirdPassValidationError> {
        if self.status != ContinuityMandateStatus::Active
            || condition != self.termination_condition
            || review_evidence.as_str().trim().is_empty()
        {
            return Err(ThirdPassValidationError::InvalidContinuityTermination);
        }
        self.status = ContinuityMandateStatus::Terminated;
        Ok(())
    }

    #[must_use]
    pub fn actions(&self) -> &BTreeMap<IdentityId, EmergencyContinuityAction> {
        &self.actions
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmergencyContinuityAction {
    pub identity: IdentityId,
    pub mandate: IdentityId,
    pub emergency: String,
    pub constitutional_basis: String,
    pub action: ContinuityActionKind,
    pub responsible_authority: IdentityId,
    pub affected_gate: Option<StonebendGateFacing>,
    pub evidence: Vec<EvidenceRecordId>,
    pub termination_condition: ContinuityTerminationCondition,
    pub later_review_required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FreemasonRecordRef {
    Claim(IdentityId),
    Seal(SealRecordId),
    Evidence(EvidenceRecordId),
    Tombstone(TombstoneRecordId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FreemasonTenureStatus {
    Active,
    Ended,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FreemasonTenure {
    pub identity: IdentityId,
    pub bearer: IdentityId,
    pub claim: IdentityId,
    pub seal: SealRecordId,
    pub began_at: u64,
    pub status: FreemasonTenureStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FreemasonSuccessionClaim {
    pub identity: IdentityId,
    pub candidate: IdentityId,
    pub qualification_evidence: Vec<EvidenceRecordId>,
    pub craft_and_constitutional_evidence: Vec<EvidenceRecordId>,
    pub prior_seal_competence: Vec<EvidenceRecordId>,
    pub conflict_disclosures: Vec<EvidenceRecordId>,
    pub outgoing_recommendation: Option<EvidenceRecordId>,
}

impl FreemasonSuccessionClaim {
    pub fn validate(&self) -> Result<(), ThirdPassValidationError> {
        if self.qualification_evidence.is_empty()
            || self.craft_and_constitutional_evidence.is_empty()
            || self.conflict_disclosures.is_empty()
        {
            return Err(ThirdPassValidationError::IncompleteFreemasonClaim);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ForgeReviewDisposition {
    Qualified,
    RemediationRequired,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndependentForgeReview {
    pub identity: IdentityId,
    pub claim: IdentityId,
    pub candidate: IdentityId,
    pub examiners: BTreeSet<IdentityId>,
    pub evidence: Vec<EvidenceRecordId>,
    pub disposition: ForgeReviewDisposition,
}

impl IndependentForgeReview {
    pub fn validate(&self) -> Result<(), ThirdPassValidationError> {
        if self.examiners.is_empty()
            || self.examiners.contains(&self.candidate)
            || self.evidence.is_empty()
        {
            return Err(ThirdPassValidationError::FreemasonSelfCertification);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FreemasonInvestiture {
    pub identity: IdentityId,
    pub claim: IdentityId,
    pub candidate: IdentityId,
    pub independent_review: IdentityId,
    pub proliteriate_yield_hearing: EvidenceRecordId,
    pub diamond_boundary_recognition: Option<EvidenceRecordId>,
    pub active_diamond_bearer: Option<IdentityId>,
    pub seal: SealRecordId,
    pub evidence: Vec<EvidenceRecordId>,
    pub began_at: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FreemasonOfficeState {
    pub active_tenure: Option<FreemasonTenure>,
    pub ended_tenures: Vec<OfficeTombstone>,
    pub preserved_records: BTreeSet<FreemasonRecordRef>,
    candidates: BTreeMap<IdentityId, FreemasonSuccessionClaim>,
    reviews: BTreeMap<IdentityId, IndependentForgeReview>,
}

impl FreemasonOfficeState {
    pub fn from_active(
        tenure: FreemasonTenure,
        preserved_records: BTreeSet<FreemasonRecordRef>,
    ) -> Result<Self, ThirdPassValidationError> {
        if tenure.status != FreemasonTenureStatus::Active {
            return Err(ThirdPassValidationError::InvalidFreemasonInvestiture);
        }
        Ok(Self {
            active_tenure: Some(tenure),
            ended_tenures: Vec::new(),
            preserved_records,
            candidates: BTreeMap::new(),
            reviews: BTreeMap::new(),
        })
    }

    #[must_use]
    pub fn is_vacant(&self) -> bool {
        self.active_tenure.is_none()
    }

    pub fn preserve_record(&mut self, record: FreemasonRecordRef) {
        self.preserved_records.insert(record);
    }

    pub fn present_candidate(
        &mut self,
        claim: FreemasonSuccessionClaim,
    ) -> Result<(), ThirdPassValidationError> {
        claim.validate()?;
        if self
            .candidates
            .insert(claim.identity.clone(), claim)
            .is_some()
        {
            return Err(ThirdPassValidationError::DuplicateFreemasonCandidate);
        }
        Ok(())
    }

    pub fn record_independent_review(
        &mut self,
        review: IndependentForgeReview,
    ) -> Result<(), ThirdPassValidationError> {
        review.validate()?;
        let claim = self
            .candidates
            .get(&review.claim)
            .ok_or(ThirdPassValidationError::UnknownFreemasonClaim)?;
        if claim.candidate != review.candidate {
            return Err(ThirdPassValidationError::FreemasonReviewMismatch);
        }
        if self
            .reviews
            .insert(review.identity.clone(), review)
            .is_some()
        {
            return Err(ThirdPassValidationError::DuplicateForgeReview);
        }
        Ok(())
    }

    pub fn invest_replacement(
        &mut self,
        investiture: FreemasonInvestiture,
        active_hypergiant: Option<&IdentityId>,
        prohibited_witnesses: &BTreeSet<IdentityId>,
    ) -> Result<FreemasonTenure, ThirdPassValidationError> {
        if self.active_tenure.is_some() {
            return Err(ThirdPassValidationError::FreemasonAlreadyActive);
        }
        let claim = self
            .candidates
            .get(&investiture.claim)
            .ok_or(ThirdPassValidationError::UnknownFreemasonClaim)?;
        let review = self
            .reviews
            .get(&investiture.independent_review)
            .ok_or(ThirdPassValidationError::UnknownForgeReview)?;
        if claim.candidate != investiture.candidate
            || review.claim != investiture.claim
            || review.candidate != investiture.candidate
            || review.disposition != ForgeReviewDisposition::Qualified
            || investiture.evidence.is_empty()
            || active_hypergiant == Some(&investiture.candidate)
            || prohibited_witnesses.contains(&investiture.candidate)
            || match active_hypergiant {
                Some(bearer) => {
                    investiture.active_diamond_bearer.as_ref() != Some(bearer)
                        || investiture.diamond_boundary_recognition.is_none()
                }
                None => {
                    investiture.active_diamond_bearer.is_some()
                        || investiture.diamond_boundary_recognition.is_some()
                }
            }
        {
            return Err(ThirdPassValidationError::InvalidFreemasonInvestiture);
        }
        let tenure = FreemasonTenure {
            identity: investiture.identity,
            bearer: investiture.candidate,
            claim: investiture.claim,
            seal: investiture.seal,
            began_at: investiture.began_at,
            status: FreemasonTenureStatus::Active,
        };
        self.preserved_records
            .insert(FreemasonRecordRef::Claim(tenure.claim.clone()));
        self.preserved_records
            .insert(FreemasonRecordRef::Seal(tenure.seal.clone()));
        self.active_tenure = Some(tenure.clone());
        Ok(tenure)
    }

    pub fn end_active_tenure(
        &mut self,
        tombstone: TombstoneRecordId,
        disposition: TitleTerminalDisposition,
        ended_at: u64,
        evidence: Vec<EvidenceRecordId>,
        successor: Option<IdentityId>,
    ) -> Result<OfficeTombstone, ThirdPassValidationError> {
        let mut tenure = self
            .active_tenure
            .take()
            .ok_or(ThirdPassValidationError::FreemasonAlreadyVacant)?;
        if ended_at < tenure.began_at || evidence.is_empty() {
            self.active_tenure = Some(tenure);
            return Err(ThirdPassValidationError::InvalidFreemasonEnding);
        }
        tenure.status = FreemasonTenureStatus::Ended;
        let record = OfficeTombstone {
            record: tombstone.clone(),
            office: high_freemason_office_id(),
            bearer_or_representation: tenure.bearer,
            sovereign_title: None,
            began_at: tenure.began_at,
            ended_at,
            supporting_claim: tenure.claim,
            gate_scopes: GateScope::ALL.into_iter().collect(),
            ending: office_ending(disposition),
            challenge: None,
            yield_record: None,
            successor,
        };
        self.preserved_records
            .insert(FreemasonRecordRef::Tombstone(tombstone));
        self.preserved_records
            .extend(evidence.into_iter().map(FreemasonRecordRef::Evidence));
        self.ended_tenures.push(record.clone());
        Ok(record)
    }

    #[must_use]
    pub fn candidates(&self) -> &BTreeMap<IdentityId, FreemasonSuccessionClaim> {
        &self.candidates
    }

    #[must_use]
    pub fn reviews(&self) -> &BTreeMap<IdentityId, IndependentForgeReview> {
        &self.reviews
    }
}

#[must_use]
pub const fn office_ending(disposition: TitleTerminalDisposition) -> OfficeEnding {
    match disposition {
        TitleTerminalDisposition::HonorableCompletion => OfficeEnding::HonorableCompletion,
        TitleTerminalDisposition::Surrender => OfficeEnding::Surrender,
        TitleTerminalDisposition::Expiration => OfficeEnding::Expiration,
        TitleTerminalDisposition::Death => OfficeEnding::Death,
        TitleTerminalDisposition::EndOfForm => OfficeEnding::EndOfForm,
        TitleTerminalDisposition::Succession => OfficeEnding::Succession,
        TitleTerminalDisposition::RemovalForFailure => OfficeEnding::RemovedForFailure,
        TitleTerminalDisposition::RemovalForFraud => OfficeEnding::RemovedForFraud,
        TitleTerminalDisposition::RemovalForIllegality => OfficeEnding::RemovedForIllegality,
        TitleTerminalDisposition::ConstitutionalDissolution => {
            OfficeEnding::ConstitutionalDissolution
        }
        TitleTerminalDisposition::Supersession => OfficeEnding::Supersession,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProliteriateContinuityPolicy {
    pub direct_affected_party_testimony_required: bool,
    pub bounded_authority_required: bool,
    pub recallability_required: bool,
    pub deterministic_completion_required: bool,
    pub permanent_selection_threshold: Option<String>,
    pub permanent_speaker: Option<IdentityId>,
}

impl Default for ProliteriateContinuityPolicy {
    fn default() -> Self {
        Self {
            direct_affected_party_testimony_required: true,
            bounded_authority_required: true,
            recallability_required: true,
            deterministic_completion_required: true,
            permanent_selection_threshold: None,
            permanent_speaker: None,
        }
    }
}

impl ProliteriateContinuityPolicy {
    pub fn validate(&self) -> Result<(), ThirdPassValidationError> {
        if !self.direct_affected_party_testimony_required
            || !self.bounded_authority_required
            || !self.recallability_required
            || !self.deterministic_completion_required
            || self.permanent_selection_threshold.is_some()
            || self.permanent_speaker.is_some()
        {
            return Err(ThirdPassValidationError::InvalidProliteriateContinuityPolicy);
        }
        Ok(())
    }

    pub fn validate_network(
        &self,
        network: &ProliteriateNetwork,
    ) -> Result<(), ThirdPassValidationError> {
        self.validate()?;
        if network.identity != StonebendConstitutionalPower::Proliteriate.institution() {
            return Err(ThirdPassValidationError::ProliteriateNetworkIdentityChanged);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClaimTemporalRecord {
    pub supporting_claim: IdentityId,
    pub evidence: Vec<EvidenceRecordId>,
    pub provenance_changed: bool,
    pub form_materially_changed: bool,
    pub structurally_hollow: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TitleTemporalRecord {
    pub title: TitleRecordId,
    pub active: bool,
    pub lawful_bearer: bool,
    pub boundaries_clear: bool,
    pub valid_scopes: BTreeSet<GateScope>,
    pub renewal_required: bool,
    pub exceeded_scope: Option<GateScope>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YieldTemporalRecord {
    pub yield_record: YieldRecord,
    pub accumulated_risk: String,
    pub inherited_consequence: String,
    pub purpose_diverged: bool,
    pub continuation_justifiable: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TemporalReviewDisposition {
    NoAction,
    MaintenanceReview,
    GateScopeReview(GateScope),
    CoreClaimReview,
    YieldHearing,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemporalDimensionTrace {
    pub dimension: ConstitutionalDimension,
    pub authority: StonebendConstitutionalPower,
    pub delegated_actor: IdentityId,
    pub evidence: EvidenceRecordId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClaimTitleYieldReview {
    pub identity: IdentityId,
    pub title: TitleRecordId,
    pub claim: ClaimTemporalRecord,
    pub title_state: TitleTemporalRecord,
    pub yield_state: YieldTemporalRecord,
    pub dimension_traces: BTreeMap<ConstitutionalDimension, TemporalDimensionTrace>,
    pub disposition: TemporalReviewDisposition,
    pub evidence: Vec<EvidenceRecordId>,
    pub sequence: u64,
}

impl ClaimTitleYieldReview {
    pub fn validate(&self) -> Result<(), ThirdPassValidationError> {
        if self.title_state.title != self.title
            || self.evidence.is_empty()
            || self.claim.evidence.is_empty()
            || self.yield_state.yield_record.evidence.is_empty()
        {
            return Err(ThirdPassValidationError::IncompleteTemporalReview);
        }
        for dimension in [
            ConstitutionalDimension::Claim,
            ConstitutionalDimension::Title,
            ConstitutionalDimension::Yield,
        ] {
            let trace = self
                .dimension_traces
                .get(&dimension)
                .ok_or(ThirdPassValidationError::IncompleteTemporalReview)?;
            if trace.dimension != dimension || trace.authority.domain() != dimension {
                return Err(ThirdPassValidationError::InvalidTemporalTrace(dimension));
            }
        }
        if self.claim.structurally_hollow
            && self.disposition != TemporalReviewDisposition::CoreClaimReview
        {
            return Err(ThirdPassValidationError::UntargetedTemporalReview);
        }
        if let Some(scope) = self.title_state.exceeded_scope
            && self.disposition != TemporalReviewDisposition::GateScopeReview(scope)
        {
            return Err(ThirdPassValidationError::UntargetedTemporalReview);
        }
        if (!self.yield_state.continuation_justifiable || self.yield_state.purpose_diverged)
            && !matches!(
                self.disposition,
                TemporalReviewDisposition::YieldHearing
                    | TemporalReviewDisposition::CoreClaimReview
            )
        {
            return Err(ThirdPassValidationError::UntargetedTemporalReview);
        }
        Ok(())
    }
}

#[must_use]
pub fn canonical_vacancy_dimension_traces(
    administrator: &IdentityId,
) -> Vec<VacancyDimensionTrace> {
    [
        StonebendConstitutionalPower::Freemason,
        StonebendConstitutionalPower::Hypergiant,
        StonebendConstitutionalPower::Proliteriate,
    ]
    .into_iter()
    .map(|power| VacancyDimensionTrace {
        dimension: power.domain(),
        constitutional_power: power,
        constitutional_source: power.institution(),
        delegated_actor: administrator.clone(),
        evidence: EvidenceRecordId::new(format!(
            "evidence.stonebend.continuity.{}",
            match power {
                StonebendConstitutionalPower::Freemason => "claim",
                StonebendConstitutionalPower::Hypergiant => "boundary",
                StonebendConstitutionalPower::Proliteriate => "yield",
            }
        ))
        .expect("canonical continuity evidence identity"),
    })
    .collect()
}

#[must_use]
pub fn diamond_vacancy_has_no_bearer(diamond: &DiamondState) -> bool {
    diamond.title == diamond_title_id() && diamond.is_vacant()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThirdPassValidationError {
    InvalidLifecyclePolicy,
    InvalidStageRecord,
    ManagedStageRequiresLifecycleAction(TitleLifecycleStage),
    DuplicateLifecycleStage,
    MissingLifecycleStage(TitleLifecycleStage),
    RequiredGateScopeUnavailable(GateScope),
    InvalidRecognition,
    ActivationRequirementsUnsatisfied,
    DuplicateTitleTerm,
    MaintenanceRequirementsUnsatisfied,
    DuplicateMaintenanceRecord,
    RenewalNotRequired,
    InvalidRenewalRecord,
    RenewalTermMismatch,
    DuplicateRenewalRecord,
    InvalidIntervention,
    InterventionTitleMismatch,
    DuplicateIntervention,
    SupervisionTermsRequired,
    UnexpectedSupervisionTerms,
    RemediationConditionRequired,
    DisproportionateHonestFailureRemoval,
    IllegalityReferralRequired,
    HollownessRequiresCoreReview,
    CoreRemovalRequiresTombstone,
    NoActiveExercise,
    InvalidRestoration,
    UnknownInterruption,
    RestorationTargetMismatch,
    InterruptionNotRestorable,
    DuplicateRestoration,
    InvalidTerminalDisposition,
    DiamondNotVacant,
    InvalidContinuityTrace(ConstitutionalDimension),
    DuplicateContinuityDimension,
    IncompleteContinuityDimensions,
    InvalidContinuityMandate,
    UnauthorizedContinuityAction,
    DuplicateContinuityAction,
    InvalidContinuityTermination,
    IncompleteFreemasonClaim,
    FreemasonSelfCertification,
    DuplicateFreemasonCandidate,
    UnknownFreemasonClaim,
    FreemasonReviewMismatch,
    DuplicateForgeReview,
    FreemasonAlreadyActive,
    UnknownForgeReview,
    InvalidFreemasonInvestiture,
    FreemasonAlreadyVacant,
    InvalidFreemasonEnding,
    InvalidProliteriateContinuityPolicy,
    ProliteriateNetworkIdentityChanged,
    IncompleteTemporalReview,
    InvalidTemporalTrace(ConstitutionalDimension),
    UntargetedTemporalReview,
    SecondPass(super::second_pass::SecondPassValidationError),
}

impl From<super::second_pass::SecondPassValidationError> for ThirdPassValidationError {
    fn from(value: super::second_pass::SecondPassValidationError) -> Self {
        Self::SecondPass(value)
    }
}

impl fmt::Display for ThirdPassValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for ThirdPassValidationError {}
