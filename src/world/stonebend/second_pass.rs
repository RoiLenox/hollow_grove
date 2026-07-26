//! Stonebend Second Pass: three gates, constitutional offices, and bounded
//! Title scope.
//!
//! This layer composes the repository's existing stable Title, evidence,
//! office, route, challenge, Tombstone, and succession identities. It does not
//! replace the First Stonebend Pass material continuum or complete the later
//! government, court, property, inheritance, or Illegal Hollowing systems.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::constitutional::ChallengeId;
use crate::hollow_grove_contract::House;
use crate::institution::{IdentityId, InstitutionId, OfficeId};
use crate::world::central_junction::JunctionApproach;
use crate::world::geography::ConstitutionalRouteId;
use crate::world::stonebend::{
    DecisionRecordId, EvidenceRecordId, NameRecordId, SealRecordId, SuccessionRecordId,
    TitleRecordId, TombstoneRecordId, freemason_institution_id, high_freemason_office_id,
    hypergiant_office_id, proliteriate_id,
};

pub const STONEBEND_SECOND_PASS_SOURCE: &str =
    "STONEBEND_THREE_GATES_OFFICES_AND_TITLE_SCOPE_V1.md";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum StonebendGateFacing {
    Flynt,
    CentralJunction,
    Sandmanor,
}

impl StonebendGateFacing {
    pub const ALL: [Self; 3] = [Self::Flynt, Self::CentralJunction, Self::Sandmanor];

    #[must_use]
    pub const fn stable_id(self) -> &'static str {
        match self {
            Self::Flynt => "gate.stonebend.flynt-facing",
            Self::CentralJunction => "gate.stonebend.central-junction-facing",
            Self::Sandmanor => "gate.stonebend.sandmanor-facing",
        }
    }

    #[must_use]
    pub const fn domain(self) -> GateConstitutionalDomain {
        match self {
            Self::Flynt => GateConstitutionalDomain::OperationalPersistence,
            Self::CentralJunction => GateConstitutionalDomain::PublicCirculation,
            Self::Sandmanor => GateConstitutionalDomain::FormationRecognition,
        }
    }

    #[must_use]
    pub const fn scope(self) -> GateScope {
        match self {
            Self::Flynt => GateScope::OperationalDeployment,
            Self::CentralJunction => GateScope::PublicCirculation,
            Self::Sandmanor => GateScope::FormationRecognition,
        }
    }

    #[must_use]
    pub const fn house_endpoint(self) -> Option<House> {
        match self {
            Self::Flynt => Some(House::Flynt),
            Self::CentralJunction => None,
            Self::Sandmanor => Some(House::Sandmanor),
        }
    }

    #[must_use]
    pub const fn junction_approach(self) -> Option<JunctionApproach> {
        match self {
            Self::CentralJunction => Some(JunctionApproach::CraftCorridor),
            Self::Flynt | Self::Sandmanor => None,
        }
    }

    #[must_use]
    pub const fn question(self) -> &'static str {
        match self {
            Self::Flynt => "Can this named Form function and persist under real pressure?",
            Self::CentralJunction => {
                "Can this named thing circulate without losing its identity or deceiving the public?"
            }
            Self::Sandmanor => "Has this possibility become stable enough to bear a lawful name?",
        }
    }

    #[must_use]
    pub const fn routes(self) -> &'static [ConstitutionalRouteId] {
        match self {
            Self::Flynt => &[
                ConstitutionalRouteId::StairwayToHeaven,
                ConstitutionalRouteId::BasinMotorspeedway,
            ],
            Self::CentralJunction => &[],
            Self::Sandmanor => &[
                ConstitutionalRouteId::AuraWay,
                ConstitutionalRouteId::MntAura,
            ],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GateConstitutionalDomain {
    OperationalPersistence,
    PublicCirculation,
    FormationRecognition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GateCrossingDirection {
    IntoStonebend,
    OutOfStonebend,
}

impl GateCrossingDirection {
    pub const ALL: [Self; 2] = [Self::IntoStonebend, Self::OutOfStonebend];
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StonebendGate {
    pub identity: IdentityId,
    pub facing: StonebendGateFacing,
    pub domain: GateConstitutionalDomain,
    pub supported_directions: BTreeSet<GateCrossingDirection>,
}

#[must_use]
pub fn canonical_stonebend_gates() -> [StonebendGate; 3] {
    StonebendGateFacing::ALL.map(|facing| StonebendGate {
        identity: IdentityId::new(facing.stable_id()).expect("canonical Stonebend gate identity"),
        facing,
        domain: facing.domain(),
        supported_directions: GateCrossingDirection::ALL.into_iter().collect(),
    })
}

pub fn validate_three_gate_topology(
    gates: &[StonebendGate],
) -> Result<(), SecondPassValidationError> {
    if gates.len() != StonebendGateFacing::ALL.len() {
        return Err(SecondPassValidationError::PrincipalGateCount(gates.len()));
    }
    let by_facing = gates
        .iter()
        .map(|gate| (gate.facing, gate))
        .collect::<BTreeMap<_, _>>();
    if by_facing.len() != StonebendGateFacing::ALL.len() {
        return Err(SecondPassValidationError::DuplicateGateFacing);
    }
    for facing in StonebendGateFacing::ALL {
        let gate = by_facing
            .get(&facing)
            .ok_or(SecondPassValidationError::MissingGate(facing))?;
        if gate.identity.as_str() != facing.stable_id() || gate.domain != facing.domain() {
            return Err(SecondPassValidationError::GateIdentityOrDomainMismatch(
                facing,
            ));
        }
        if gate.supported_directions
            != GateCrossingDirection::ALL
                .into_iter()
                .collect::<BTreeSet<_>>()
        {
            return Err(SecondPassValidationError::GateNotBidirectional(facing));
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GateScope {
    FormationRecognition,
    PublicCirculation,
    OperationalDeployment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TitleScopeDisposition {
    Pending,
    Recognized,
    Limited,
    Suspended,
    Rejected,
    Removed,
}

impl TitleScopeDisposition {
    #[must_use]
    pub const fn authorizes_operation(self) -> bool {
        matches!(self, Self::Recognized | Self::Limited)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DomainEvidenceAuthority {
    SandmanorDesignAndFormation,
    CentralJunctionPublicStandard,
    FlyntProofOfPersistence,
    StonebendIdentityAndBoundary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GateFailureKind {
    HonestFailure,
    Negligence,
    Fraud,
    Illegality,
    ConstitutionalHollowness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReturnedEvidenceDisposition {
    NoAction,
    Correction,
    Remediation,
    ScopeLimitation,
    ScopeSuspension,
    ScopeRemoval,
    RenewalReview,
    CoreTitleChallenge,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateEvidenceTransfer {
    pub identity: IdentityId,
    pub source_gate: StonebendGateFacing,
    pub title: TitleRecordId,
    pub evidence: Vec<EvidenceRecordId>,
    pub failure_kind: GateFailureKind,
    pub recommended_disposition: ReturnedEvidenceDisposition,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateScopeRecognition {
    pub identity: IdentityId,
    pub title: TitleRecordId,
    pub facing: StonebendGateFacing,
    pub scope: GateScope,
    pub disposition: TitleScopeDisposition,
    pub domain_authority: DomainEvidenceAuthority,
    pub evidence: Vec<EvidenceRecordId>,
    pub boundary: String,
    pub returned_evidence: Vec<GateEvidenceTransfer>,
}

impl GateScopeRecognition {
    pub fn validate(&self) -> Result<(), SecondPassValidationError> {
        if self.scope != self.facing.scope() {
            return Err(SecondPassValidationError::GateScopeMismatch {
                facing: self.facing,
                scope: self.scope,
            });
        }
        let correct_domain_authority = matches!(
            (self.facing, self.domain_authority),
            (
                StonebendGateFacing::Sandmanor,
                DomainEvidenceAuthority::SandmanorDesignAndFormation
            ) | (
                StonebendGateFacing::CentralJunction,
                DomainEvidenceAuthority::CentralJunctionPublicStandard
            ) | (
                StonebendGateFacing::Flynt,
                DomainEvidenceAuthority::FlyntProofOfPersistence
            )
        );
        if !correct_domain_authority {
            return Err(SecondPassValidationError::GateDomainAuthorityMismatch(
                self.facing,
            ));
        }
        if self.boundary.trim().is_empty() || self.evidence.is_empty() {
            return Err(SecondPassValidationError::IncompleteGateRecognition(
                self.identity.clone(),
            ));
        }
        if self
            .returned_evidence
            .iter()
            .any(|transfer| transfer.title != self.title || transfer.source_gate != self.facing)
        {
            return Err(SecondPassValidationError::ReturnedEvidenceMismatch(
                self.identity.clone(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StonebendTitleCore {
    pub title: TitleRecordId,
    pub subject: IdentityId,
    pub lawful_name: NameRecordId,
    pub supporting_claim: IdentityId,
    pub core_evidence: Vec<EvidenceRecordId>,
    pub general_boundary: String,
    scopes: BTreeMap<GateScope, GateScopeRecognition>,
    core_challenges: BTreeSet<ChallengeId>,
}

impl StonebendTitleCore {
    #[must_use]
    pub fn new(
        title: TitleRecordId,
        subject: IdentityId,
        lawful_name: NameRecordId,
        supporting_claim: IdentityId,
        core_evidence: Vec<EvidenceRecordId>,
        general_boundary: impl Into<String>,
    ) -> Self {
        Self {
            title,
            subject,
            lawful_name,
            supporting_claim,
            core_evidence,
            general_boundary: general_boundary.into(),
            scopes: BTreeMap::new(),
            core_challenges: BTreeSet::new(),
        }
    }

    pub fn record_scope(
        &mut self,
        recognition: GateScopeRecognition,
    ) -> Result<(), SecondPassValidationError> {
        recognition.validate()?;
        if recognition.title != self.title {
            return Err(SecondPassValidationError::ScopeCreatesDuplicateTitle {
                core: self.title.clone(),
                supplied: recognition.title,
            });
        }
        if self.scopes.contains_key(&recognition.scope) {
            return Err(SecondPassValidationError::DuplicateGateScope(
                recognition.scope,
            ));
        }
        self.scopes.insert(recognition.scope, recognition);
        Ok(())
    }

    #[must_use]
    pub fn scope(&self, scope: GateScope) -> Option<&GateScopeRecognition> {
        self.scopes.get(&scope)
    }

    #[must_use]
    pub fn scopes(&self) -> &BTreeMap<GateScope, GateScopeRecognition> {
        &self.scopes
    }

    #[must_use]
    pub fn authorizes(&self, scope: GateScope) -> bool {
        self.scopes
            .get(&scope)
            .is_some_and(|record| record.disposition.authorizes_operation())
    }

    /// Applies a later constitutional lifecycle disposition to an existing
    /// gate scope without manufacturing a second core Title.
    pub fn update_scope_disposition(
        &mut self,
        scope: GateScope,
        disposition: TitleScopeDisposition,
    ) -> Result<TitleScopeDisposition, SecondPassValidationError> {
        let recognition = self
            .scopes
            .get_mut(&scope)
            .ok_or(SecondPassValidationError::UnknownGateScope(scope))?;
        let previous = recognition.disposition;
        recognition.disposition = disposition;
        Ok(previous)
    }

    pub fn open_core_challenge(&mut self, challenge: ChallengeId) {
        self.core_challenges.insert(challenge);
    }

    #[must_use]
    pub fn has_explicit_core_challenge(&self, challenge: &ChallengeId) -> bool {
        self.core_challenges.contains(challenge)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum StonebendConstitutionalPower {
    Hypergiant,
    Freemason,
    Proliteriate,
}

impl StonebendConstitutionalPower {
    pub const ALL: [Self; 3] = [Self::Hypergiant, Self::Freemason, Self::Proliteriate];

    #[must_use]
    pub const fn domain(self) -> ConstitutionalDimension {
        match self {
            Self::Freemason => ConstitutionalDimension::Claim,
            Self::Hypergiant => ConstitutionalDimension::Title,
            Self::Proliteriate => ConstitutionalDimension::Yield,
        }
    }

    #[must_use]
    pub fn office(self) -> Option<OfficeId> {
        match self {
            Self::Hypergiant => Some(hypergiant_office_id()),
            Self::Freemason => Some(high_freemason_office_id()),
            Self::Proliteriate => None,
        }
    }

    #[must_use]
    pub fn institution(self) -> InstitutionId {
        match self {
            Self::Hypergiant => InstitutionId::new("institution.stonebend.constitution")
                .expect("canonical Stonebend constitution identity"),
            Self::Freemason => freemason_institution_id(),
            Self::Proliteriate => proliteriate_id(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ConstitutionalDimension {
    Claim,
    Title,
    Yield,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SovereignTitle {
    Diamond,
}

#[must_use]
pub fn diamond_title_id() -> TitleRecordId {
    TitleRecordId::new("title.stonebend.diamond").expect("canonical Diamond Title identity")
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClaimRecord {
    pub identity: IdentityId,
    pub subject: IdentityId,
    pub proposed_title: TitleRecordId,
    pub evidence: Vec<EvidenceRecordId>,
    pub examiner: IdentityId,
    pub examination: DecisionRecordId,
    pub seal: Option<SealRecordId>,
    pub sovereign_claim: bool,
}

impl ClaimRecord {
    pub fn validate_freemason_examination(
        &self,
        active_freemason: &IdentityId,
    ) -> Result<(), SecondPassValidationError> {
        if &self.examiner != active_freemason {
            return Err(SecondPassValidationError::ClaimNotExaminedByFreemason);
        }
        if self.sovereign_claim && self.subject == self.examiner {
            return Err(SecondPassValidationError::SovereignSelfCertification);
        }
        if self.evidence.is_empty() {
            return Err(SecondPassValidationError::ClaimWithoutEvidence);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiamondTenureStatus {
    Active,
    Ended,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiamondTenure {
    pub identity: IdentityId,
    pub diamond: TitleRecordId,
    pub bearer: IdentityId,
    pub supporting_claim: IdentityId,
    pub succession: SuccessionRecordId,
    pub began_at: u64,
    pub status: DiamondTenureStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OfficeEnding {
    HonorableCompletion,
    RemovedForFailure,
    Expiration,
    Death,
    EndOfForm,
    Surrender,
    Succession,
    RemovedForFraud,
    RemovedForIllegality,
    ConstitutionalDissolution,
    Supersession,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OfficeTombstone {
    pub record: TombstoneRecordId,
    pub office: OfficeId,
    pub bearer_or_representation: IdentityId,
    pub sovereign_title: Option<TitleRecordId>,
    pub began_at: u64,
    pub ended_at: u64,
    pub supporting_claim: IdentityId,
    pub gate_scopes: BTreeSet<GateScope>,
    pub ending: OfficeEnding,
    pub challenge: Option<ChallengeId>,
    pub yield_record: Option<IdentityId>,
    pub successor: Option<IdentityId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiamondState {
    pub title: TitleRecordId,
    pub active_tenure: Option<DiamondTenure>,
    pub ended_tenures: Vec<OfficeTombstone>,
}

impl Default for DiamondState {
    fn default() -> Self {
        Self {
            title: diamond_title_id(),
            active_tenure: None,
            ended_tenures: Vec::new(),
        }
    }
}

impl DiamondState {
    #[must_use]
    pub fn is_vacant(&self) -> bool {
        self.active_tenure.is_none()
    }

    pub fn invest(&mut self, tenure: DiamondTenure) -> Result<(), SecondPassValidationError> {
        if self.active_tenure.is_some() {
            return Err(SecondPassValidationError::DiamondAlreadyBorne);
        }
        if tenure.diamond != self.title || tenure.status != DiamondTenureStatus::Active {
            return Err(SecondPassValidationError::InvalidDiamondTenure);
        }
        self.active_tenure = Some(tenure);
        Ok(())
    }

    pub fn end_active_tenure(
        &mut self,
        tombstone_id: TombstoneRecordId,
        ending: OfficeEnding,
        ended_at: u64,
        challenge: Option<ChallengeId>,
        yield_record: Option<IdentityId>,
        successor: Option<IdentityId>,
    ) -> Result<OfficeTombstone, SecondPassValidationError> {
        if ending == OfficeEnding::RemovedForFailure {
            return Err(SecondPassValidationError::RemovalRequiresConcurrence);
        }
        self.finish_active_tenure(
            tombstone_id,
            ending,
            ended_at,
            challenge,
            yield_record,
            successor,
        )
    }

    fn finish_active_tenure(
        &mut self,
        tombstone_id: TombstoneRecordId,
        ending: OfficeEnding,
        ended_at: u64,
        challenge: Option<ChallengeId>,
        yield_record: Option<IdentityId>,
        successor: Option<IdentityId>,
    ) -> Result<OfficeTombstone, SecondPassValidationError> {
        let mut tenure = self
            .active_tenure
            .take()
            .ok_or(SecondPassValidationError::DiamondAlreadyVacant)?;
        if ended_at < tenure.began_at {
            self.active_tenure = Some(tenure);
            return Err(SecondPassValidationError::InvalidTenureSequence);
        }
        tenure.status = DiamondTenureStatus::Ended;
        let tombstone = OfficeTombstone {
            record: tombstone_id,
            office: hypergiant_office_id(),
            bearer_or_representation: tenure.bearer,
            sovereign_title: Some(self.title.clone()),
            began_at: tenure.began_at,
            ended_at,
            supporting_claim: tenure.supporting_claim,
            gate_scopes: GateScope::ALL.into_iter().collect(),
            ending,
            challenge,
            yield_record,
            successor,
        };
        self.ended_tenures.push(tombstone.clone());
        Ok(tombstone)
    }

    pub fn remove_active_hypergiant(
        &mut self,
        authorization: &RemovalAuthorization,
        tombstone_id: TombstoneRecordId,
        ended_at: u64,
        yield_record: Option<IdentityId>,
    ) -> Result<RemovalDisposition, SecondPassValidationError> {
        let bearer = self
            .active_tenure
            .as_ref()
            .map(|tenure| tenure.bearer.clone())
            .ok_or(SecondPassValidationError::DiamondAlreadyVacant)?;
        if authorization.target != ConstitutionalTarget::Hypergiant(bearer) {
            return Err(SecondPassValidationError::RemovalTargetMismatch);
        }
        let tombstone = self.finish_active_tenure(
            tombstone_id,
            OfficeEnding::RemovedForFailure,
            ended_at,
            Some(authorization.challenge.clone()),
            yield_record,
            None,
        )?;
        Ok(RemovalDisposition {
            challenge: authorization.challenge.clone(),
            target: authorization.target.clone(),
            tombstone: tombstone.record,
            diamond_vacant: true,
            network_survives: true,
        })
    }
}

impl GateScope {
    pub const ALL: [Self; 3] = [
        Self::FormationRecognition,
        Self::PublicCirculation,
        Self::OperationalDeployment,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ProliteriateNodeKind {
    District,
    GuildOrWorkshop,
    LaborCrewOrWorksite,
    InheritedOrCommonwealthCommunity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProliteriateNode {
    pub identity: IdentityId,
    pub kind: ProliteriateNodeKind,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NetworkMembership {
    pub person: IdentityId,
    pub node: IdentityId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MandateAuthority {
    PresentYield,
    OpenChallenge,
    GiveGateTestimony,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YieldImpact {
    pub description: String,
    pub beneficiaries: BTreeSet<IdentityId>,
    pub burden_bearers: BTreeSet<IdentityId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YieldRecord {
    pub identity: IdentityId,
    pub issue: IdentityId,
    pub impacts: Vec<YieldImpact>,
    pub evidence: Vec<EvidenceRecordId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WitnessRecall {
    pub witness: IdentityId,
    pub evidence: EvidenceRecordId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkMandate {
    pub identity: IdentityId,
    pub participating_nodes: BTreeSet<IdentityId>,
    pub issue_claim: IdentityId,
    pub affected_yield: YieldRecord,
    pub testimony: Vec<EvidenceRecordId>,
    pub authorities: BTreeSet<MandateAuthority>,
    pub boundary: String,
    pub active_witness: Option<IdentityId>,
    pub recalls: Vec<WitnessRecall>,
    pub completed: bool,
    pub invalidated: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RaisedWitness {
    pub identity: IdentityId,
    pub person: IdentityId,
    pub mandate: IdentityId,
    pub active: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProliteriateNetwork {
    pub identity: InstitutionId,
    nodes: BTreeMap<IdentityId, ProliteriateNode>,
    dissolved_nodes: BTreeSet<IdentityId>,
    memberships: BTreeSet<NetworkMembership>,
    mandates: BTreeMap<IdentityId, NetworkMandate>,
    witnesses: BTreeMap<IdentityId, RaisedWitness>,
}

impl Default for ProliteriateNetwork {
    fn default() -> Self {
        Self {
            identity: proliteriate_id(),
            nodes: BTreeMap::new(),
            dissolved_nodes: BTreeSet::new(),
            memberships: BTreeSet::new(),
            mandates: BTreeMap::new(),
            witnesses: BTreeMap::new(),
        }
    }
}

impl ProliteriateNetwork {
    pub fn add_node(&mut self, node: ProliteriateNode) -> Result<(), SecondPassValidationError> {
        if node.name.trim().is_empty() {
            return Err(SecondPassValidationError::UnnamedNetworkNode);
        }
        if self.nodes.insert(node.identity.clone(), node).is_some() {
            return Err(SecondPassValidationError::DuplicateNetworkNode);
        }
        Ok(())
    }

    pub fn add_membership(
        &mut self,
        membership: NetworkMembership,
    ) -> Result<(), SecondPassValidationError> {
        if !self.nodes.contains_key(&membership.node) {
            return Err(SecondPassValidationError::UnknownNetworkNode(
                membership.node,
            ));
        }
        self.memberships.insert(membership);
        Ok(())
    }

    pub fn add_mandate(
        &mut self,
        mandate: NetworkMandate,
    ) -> Result<(), SecondPassValidationError> {
        if mandate.participating_nodes.is_empty()
            || mandate
                .participating_nodes
                .iter()
                .any(|node| !self.nodes.contains_key(node) || self.dissolved_nodes.contains(node))
        {
            return Err(SecondPassValidationError::InvalidMandateNodes);
        }
        if mandate.authorities.is_empty()
            || mandate.boundary.trim().is_empty()
            || mandate.testimony.is_empty()
            || mandate.completed
            || mandate.invalidated
            || mandate.active_witness.is_some()
        {
            return Err(SecondPassValidationError::InvalidMandate);
        }
        if self
            .mandates
            .insert(mandate.identity.clone(), mandate)
            .is_some()
        {
            return Err(SecondPassValidationError::DuplicateMandate);
        }
        Ok(())
    }

    pub fn raise_witness(
        &mut self,
        witness: RaisedWitness,
    ) -> Result<(), SecondPassValidationError> {
        if !witness.active {
            return Err(SecondPassValidationError::InactiveWitnessCannotBeRaised);
        }
        let mandate = self
            .mandates
            .get_mut(&witness.mandate)
            .ok_or_else(|| SecondPassValidationError::UnknownMandate(witness.mandate.clone()))?;
        if mandate.completed || mandate.invalidated || mandate.active_witness.is_some() {
            return Err(SecondPassValidationError::MandateCannotRaiseWitness);
        }
        if self.witnesses.contains_key(&witness.identity) {
            return Err(SecondPassValidationError::DuplicateWitness);
        }
        mandate.active_witness = Some(witness.identity.clone());
        self.witnesses.insert(witness.identity.clone(), witness);
        Ok(())
    }

    pub fn witness_has_authority(&self, witness: &IdentityId, authority: MandateAuthority) -> bool {
        let Some(witness) = self.witnesses.get(witness).filter(|record| record.active) else {
            return false;
        };
        self.mandates.get(&witness.mandate).is_some_and(|mandate| {
            !mandate.completed
                && !mandate.invalidated
                && mandate.active_witness.as_ref() == Some(&witness.identity)
                && mandate.authorities.contains(&authority)
        })
    }

    pub fn recall_witness(
        &mut self,
        witness_id: &IdentityId,
        evidence: EvidenceRecordId,
    ) -> Result<(), SecondPassValidationError> {
        let witness = self
            .witnesses
            .get_mut(witness_id)
            .ok_or_else(|| SecondPassValidationError::UnknownWitness(witness_id.clone()))?;
        if !witness.active {
            return Err(SecondPassValidationError::WitnessAlreadyInactive);
        }
        witness.active = false;
        let mandate = self
            .mandates
            .get_mut(&witness.mandate)
            .expect("witness cannot outlive its validated mandate");
        mandate.active_witness = None;
        mandate.recalls.push(WitnessRecall {
            witness: witness_id.clone(),
            evidence,
        });
        Ok(())
    }

    pub fn complete_mandate(
        &mut self,
        mandate_id: &IdentityId,
    ) -> Result<(), SecondPassValidationError> {
        let mandate = self
            .mandates
            .get_mut(mandate_id)
            .ok_or_else(|| SecondPassValidationError::UnknownMandate(mandate_id.clone()))?;
        if mandate.invalidated || mandate.completed {
            return Err(SecondPassValidationError::MandateAlreadyClosed);
        }
        if let Some(witness_id) = mandate.active_witness.take()
            && let Some(witness) = self.witnesses.get_mut(&witness_id)
        {
            witness.active = false;
        }
        mandate.completed = true;
        Ok(())
    }

    pub fn invalidate_representation(
        &mut self,
        mandate_id: &IdentityId,
        authorization: &RemovalAuthorization,
        tombstone: TombstoneRecordId,
    ) -> Result<RemovalDisposition, SecondPassValidationError> {
        if authorization.target
            != ConstitutionalTarget::ProliteriateRepresentation(mandate_id.clone())
        {
            return Err(SecondPassValidationError::RemovalTargetMismatch);
        }
        let mandate = self
            .mandates
            .get_mut(mandate_id)
            .ok_or_else(|| SecondPassValidationError::UnknownMandate(mandate_id.clone()))?;
        if let Some(witness_id) = mandate.active_witness.take()
            && let Some(witness) = self.witnesses.get_mut(&witness_id)
        {
            witness.active = false;
        }
        mandate.invalidated = true;
        Ok(RemovalDisposition {
            challenge: authorization.challenge.clone(),
            target: authorization.target.clone(),
            tombstone,
            diamond_vacant: false,
            network_survives: true,
        })
    }

    #[must_use]
    pub fn nodes(&self) -> &BTreeMap<IdentityId, ProliteriateNode> {
        &self.nodes
    }

    #[must_use]
    pub fn memberships(&self) -> &BTreeSet<NetworkMembership> {
        &self.memberships
    }

    #[must_use]
    pub fn mandates(&self) -> &BTreeMap<IdentityId, NetworkMandate> {
        &self.mandates
    }

    #[must_use]
    pub fn witnesses(&self) -> &BTreeMap<IdentityId, RaisedWitness> {
        &self.witnesses
    }

    /// Ends a node's present mandate eligibility while retaining its identity,
    /// membership history, and the identities of every person who belonged to
    /// it.
    pub fn dissolve_node(&mut self, node: &IdentityId) -> Result<(), SecondPassValidationError> {
        if !self.nodes.contains_key(node) {
            return Err(SecondPassValidationError::UnknownNetworkNode(node.clone()));
        }
        self.dissolved_nodes.insert(node.clone());
        Ok(())
    }

    #[must_use]
    pub fn node_is_active(&self, node: &IdentityId) -> bool {
        self.nodes.contains_key(node) && !self.dissolved_nodes.contains(node)
    }

    #[must_use]
    pub fn dissolved_nodes(&self) -> &BTreeSet<IdentityId> {
        &self.dissolved_nodes
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConstitutionalTarget {
    Hypergiant(IdentityId),
    Freemason(IdentityId),
    ProliteriateRepresentation(IdentityId),
}

impl ConstitutionalTarget {
    #[must_use]
    pub const fn power(&self) -> StonebendConstitutionalPower {
        match self {
            Self::Hypergiant(_) => StonebendConstitutionalPower::Hypergiant,
            Self::Freemason(_) => StonebendConstitutionalPower::Freemason,
            Self::ProliteriateRepresentation(_) => StonebendConstitutionalPower::Proliteriate,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChallengeGround {
    UnsupportedClaim,
    TitleAbuse,
    InvalidContinuity,
    IntolerableYield,
    ConcealedEvidence,
    CounterfeitSeal,
    FalseProvenance,
    ExceededMandate,
    ConstitutionalViolation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChallengeStage {
    Opened,
    Answered,
    UnderReview,
    Rejected,
    RemediationOrdered,
    AuthorityLimited,
    RemovalAuthorized,
    Removed,
}

impl ChallengeStage {
    #[must_use]
    pub const fn semantic_order(self) -> u8 {
        match self {
            Self::Opened => 0,
            Self::Answered => 1,
            Self::UnderReview => 2,
            Self::Rejected | Self::RemediationOrdered | Self::AuthorityLimited => 3,
            Self::RemovalAuthorized => 4,
            Self::Removed => 5,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstitutionalChallenge {
    pub identity: ChallengeId,
    pub challenger: StonebendConstitutionalPower,
    pub target: ConstitutionalTarget,
    pub ground: ChallengeGround,
    pub evidence: Vec<EvidenceRecordId>,
    pub answer_evidence: Vec<EvidenceRecordId>,
    pub stage: ChallengeStage,
}

impl ConstitutionalChallenge {
    pub fn open(
        identity: ChallengeId,
        challenger: StonebendConstitutionalPower,
        target: ConstitutionalTarget,
        ground: ChallengeGround,
        evidence: Vec<EvidenceRecordId>,
    ) -> Result<Self, SecondPassValidationError> {
        if challenger == target.power() {
            return Err(SecondPassValidationError::PowerCannotReviewItself);
        }
        if evidence.is_empty() {
            return Err(SecondPassValidationError::ChallengeWithoutEvidence);
        }
        Ok(Self {
            identity,
            challenger,
            target,
            ground,
            evidence,
            answer_evidence: Vec::new(),
            stage: ChallengeStage::Opened,
        })
    }

    pub fn answer(
        &mut self,
        evidence: Vec<EvidenceRecordId>,
    ) -> Result<(), SecondPassValidationError> {
        if self.stage != ChallengeStage::Opened || evidence.is_empty() {
            return Err(SecondPassValidationError::InvalidChallengeTransition);
        }
        self.answer_evidence = evidence;
        self.stage = ChallengeStage::Answered;
        Ok(())
    }

    pub fn open_review(&mut self) -> Result<(), SecondPassValidationError> {
        if self.stage != ChallengeStage::Answered {
            return Err(SecondPassValidationError::InvalidChallengeTransition);
        }
        self.stage = ChallengeStage::UnderReview;
        Ok(())
    }

    pub fn record_removal_authorized(
        &mut self,
        authorization: &RemovalAuthorization,
    ) -> Result<(), SecondPassValidationError> {
        if self.stage != ChallengeStage::UnderReview
            || authorization.challenge != self.identity
            || authorization.target != self.target
        {
            return Err(SecondPassValidationError::InvalidChallengeTransition);
        }
        self.stage = ChallengeStage::RemovalAuthorized;
        Ok(())
    }

    pub fn record_removed(
        &mut self,
        disposition: &RemovalDisposition,
    ) -> Result<(), SecondPassValidationError> {
        if self.stage != ChallengeStage::RemovalAuthorized
            || disposition.challenge != self.identity
            || disposition.target != self.target
        {
            return Err(SecondPassValidationError::InvalidChallengeTransition);
        }
        self.stage = ChallengeStage::Removed;
        Ok(())
    }

    #[must_use]
    pub const fn removes_target(&self) -> bool {
        matches!(self.stage, ChallengeStage::Removed)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstitutionalConcurrence {
    pub challenge: ChallengeId,
    pub target: ConstitutionalTarget,
    endorsements: BTreeSet<StonebendConstitutionalPower>,
}

impl ConstitutionalConcurrence {
    pub fn new(
        challenge: &ConstitutionalChallenge,
        endorsements: Vec<StonebendConstitutionalPower>,
    ) -> Result<Self, SecondPassValidationError> {
        if challenge.stage != ChallengeStage::UnderReview {
            return Err(SecondPassValidationError::ChallengeNotReviewed);
        }
        let distinct = endorsements.iter().copied().collect::<BTreeSet<_>>();
        if distinct.len() != endorsements.len() {
            return Err(SecondPassValidationError::DuplicateConcurrence);
        }
        if distinct.len() < 2 {
            return Err(SecondPassValidationError::InsufficientConcurrence);
        }
        if distinct.contains(&challenge.target.power())
            || distinct
                != StonebendConstitutionalPower::ALL
                    .into_iter()
                    .filter(|power| *power != challenge.target.power())
                    .collect()
        {
            return Err(SecondPassValidationError::InvalidRemovingPowers);
        }
        Ok(Self {
            challenge: challenge.identity.clone(),
            target: challenge.target.clone(),
            endorsements: distinct,
        })
    }

    #[must_use]
    pub fn endorsements(&self) -> &BTreeSet<StonebendConstitutionalPower> {
        &self.endorsements
    }

    #[must_use]
    pub fn authorize(self) -> RemovalAuthorization {
        RemovalAuthorization {
            challenge: self.challenge,
            target: self.target,
            concurring_powers: self.endorsements,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemovalAuthorization {
    pub challenge: ChallengeId,
    pub target: ConstitutionalTarget,
    pub concurring_powers: BTreeSet<StonebendConstitutionalPower>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemovalDisposition {
    pub challenge: ChallengeId,
    pub target: ConstitutionalTarget,
    pub tombstone: TombstoneRecordId,
    pub diamond_vacant: bool,
    pub network_survives: bool,
}

pub fn remove_active_freemason(
    authorization: &RemovalAuthorization,
    active_freemason: &IdentityId,
    tombstone: TombstoneRecordId,
    supporting_claim: IdentityId,
    began_at: u64,
    ended_at: u64,
    yield_record: Option<IdentityId>,
) -> Result<(RemovalDisposition, OfficeTombstone), SecondPassValidationError> {
    if authorization.target != ConstitutionalTarget::Freemason(active_freemason.clone()) {
        return Err(SecondPassValidationError::RemovalTargetMismatch);
    }
    if ended_at < began_at {
        return Err(SecondPassValidationError::InvalidTenureSequence);
    }
    let office_tombstone = OfficeTombstone {
        record: tombstone.clone(),
        office: high_freemason_office_id(),
        bearer_or_representation: active_freemason.clone(),
        sovereign_title: None,
        began_at,
        ended_at,
        supporting_claim,
        gate_scopes: GateScope::ALL.into_iter().collect(),
        ending: OfficeEnding::RemovedForFailure,
        challenge: Some(authorization.challenge.clone()),
        yield_record,
        successor: None,
    };
    let disposition = RemovalDisposition {
        challenge: authorization.challenge.clone(),
        target: authorization.target.clone(),
        tombstone,
        diamond_vacant: false,
        network_survives: true,
    };
    Ok((disposition, office_tombstone))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum HypergiantSuccessionStage {
    ClaimPresented,
    FreemasonExamination,
    ProliteriateYieldHearing,
    ProtectedElevationRelinquished,
    ConsequenceDescentCompleted,
    FlyntProofOfPersistence,
    LazerhornClimbed,
    AccessionEligible,
    DiamondInvested,
}

impl HypergiantSuccessionStage {
    pub const ALL: [Self; 9] = [
        Self::ClaimPresented,
        Self::FreemasonExamination,
        Self::ProliteriateYieldHearing,
        Self::ProtectedElevationRelinquished,
        Self::ConsequenceDescentCompleted,
        Self::FlyntProofOfPersistence,
        Self::LazerhornClimbed,
        Self::AccessionEligible,
        Self::DiamondInvested,
    ];

    #[must_use]
    pub const fn semantic_order(self) -> u8 {
        match self {
            Self::ClaimPresented => 0,
            Self::FreemasonExamination => 1,
            Self::ProliteriateYieldHearing => 2,
            Self::ProtectedElevationRelinquished => 3,
            Self::ConsequenceDescentCompleted => 4,
            Self::FlyntProofOfPersistence => 5,
            Self::LazerhornClimbed => 6,
            Self::AccessionEligible => 7,
            Self::DiamondInvested => 8,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuccessionStageEvidence {
    pub stage: HypergiantSuccessionStage,
    pub evidence: EvidenceRecordId,
    pub responsible_authority: IdentityId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HypergiantSuccession {
    pub identity: SuccessionRecordId,
    pub candidate: IdentityId,
    pub claim: IdentityId,
    pub outgoing_recommendation: Option<EvidenceRecordId>,
    pub lineage_evidence: Option<EvidenceRecordId>,
    pub former_bearer_returning: bool,
    stage_evidence: BTreeMap<HypergiantSuccessionStage, SuccessionStageEvidence>,
}

impl HypergiantSuccession {
    #[must_use]
    pub fn new(identity: SuccessionRecordId, candidate: IdentityId, claim: IdentityId) -> Self {
        Self {
            identity,
            candidate,
            claim,
            outgoing_recommendation: None,
            lineage_evidence: None,
            former_bearer_returning: false,
            stage_evidence: BTreeMap::new(),
        }
    }

    pub fn record_stage(
        &mut self,
        record: SuccessionStageEvidence,
    ) -> Result<(), SecondPassValidationError> {
        if record.stage == HypergiantSuccessionStage::FreemasonExamination
            && record.responsible_authority == self.candidate
        {
            return Err(SecondPassValidationError::SovereignSelfCertification);
        }
        if self.stage_evidence.insert(record.stage, record).is_some() {
            return Err(SecondPassValidationError::DuplicateSuccessionStage);
        }
        Ok(())
    }

    #[must_use]
    pub fn ordered_stage_evidence(&self) -> Vec<&SuccessionStageEvidence> {
        let mut records = self.stage_evidence.values().collect::<Vec<_>>();
        records.sort_by_key(|record| record.stage.semantic_order());
        records
    }

    #[must_use]
    pub fn has_stage(&self, stage: HypergiantSuccessionStage) -> bool {
        self.stage_evidence.contains_key(&stage)
    }

    pub fn require_accession_eligibility(&self) -> Result<(), SecondPassValidationError> {
        for stage in HypergiantSuccessionStage::ALL
            .into_iter()
            .take_while(|stage| *stage != HypergiantSuccessionStage::DiamondInvested)
        {
            if !self.has_stage(stage) {
                return Err(SecondPassValidationError::MissingSuccessionStage(stage));
            }
        }
        Ok(())
    }

    pub fn invest_diamond(
        &mut self,
        diamond: &mut DiamondState,
        tenure_identity: IdentityId,
        began_at: u64,
        investiture_evidence: SuccessionStageEvidence,
    ) -> Result<DiamondTenure, SecondPassValidationError> {
        self.require_accession_eligibility()?;
        if investiture_evidence.stage != HypergiantSuccessionStage::DiamondInvested {
            return Err(SecondPassValidationError::InvalidInvestitureEvidence);
        }
        self.record_stage(investiture_evidence)?;
        let tenure = DiamondTenure {
            identity: tenure_identity,
            diamond: diamond.title.clone(),
            bearer: self.candidate.clone(),
            supporting_claim: self.claim.clone(),
            succession: self.identity.clone(),
            began_at,
            status: DiamondTenureStatus::Active,
        };
        diamond.invest(tenure.clone())?;
        Ok(tenure)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DelegatedPowerTrace {
    pub power: StonebendConstitutionalPower,
    pub constitutional_source: InstitutionId,
    pub delegated_actor: IdentityId,
    pub evidence: EvidenceRecordId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClaimAccountability {
    pub claim: IdentityId,
    pub trace: DelegatedPowerTrace,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TitleBoundaryAccountability {
    pub title: TitleRecordId,
    pub disposition: TitleScopeDisposition,
    pub trace: DelegatedPowerTrace,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YieldAccountability {
    pub yield_record: IdentityId,
    pub trace: DelegatedPowerTrace,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateCrossingRecord {
    pub identity: IdentityId,
    pub facing: StonebendGateFacing,
    pub direction: GateCrossingDirection,
    pub subject: IdentityId,
    pub claim: ClaimAccountability,
    pub title_boundary: TitleBoundaryAccountability,
    pub yield_accountability: YieldAccountability,
}

impl GateCrossingRecord {
    pub fn validate(&self) -> Result<(), SecondPassValidationError> {
        let expected = [
            (
                self.claim.trace.power,
                StonebendConstitutionalPower::Freemason,
                ConstitutionalDimension::Claim,
            ),
            (
                self.title_boundary.trace.power,
                StonebendConstitutionalPower::Hypergiant,
                ConstitutionalDimension::Title,
            ),
            (
                self.yield_accountability.trace.power,
                StonebendConstitutionalPower::Proliteriate,
                ConstitutionalDimension::Yield,
            ),
        ];
        if expected.iter().any(|(actual, required, dimension)| {
            actual != required || actual.domain() != *dimension
        }) {
            return Err(SecondPassValidationError::IncompleteGateAccountability);
        }
        for trace in [
            &self.claim.trace,
            &self.title_boundary.trace,
            &self.yield_accountability.trace,
        ] {
            if trace.constitutional_source != trace.power.institution() {
                return Err(SecondPassValidationError::InvalidDelegationTrace(
                    trace.power,
                ));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecondPassValidationError {
    PrincipalGateCount(usize),
    DuplicateGateFacing,
    MissingGate(StonebendGateFacing),
    GateIdentityOrDomainMismatch(StonebendGateFacing),
    GateNotBidirectional(StonebendGateFacing),
    GateScopeMismatch {
        facing: StonebendGateFacing,
        scope: GateScope,
    },
    GateDomainAuthorityMismatch(StonebendGateFacing),
    IncompleteGateRecognition(IdentityId),
    ReturnedEvidenceMismatch(IdentityId),
    ScopeCreatesDuplicateTitle {
        core: TitleRecordId,
        supplied: TitleRecordId,
    },
    DuplicateGateScope(GateScope),
    UnknownGateScope(GateScope),
    ClaimNotExaminedByFreemason,
    SovereignSelfCertification,
    ClaimWithoutEvidence,
    DiamondAlreadyBorne,
    InvalidDiamondTenure,
    DiamondAlreadyVacant,
    InvalidTenureSequence,
    RemovalRequiresConcurrence,
    UnnamedNetworkNode,
    DuplicateNetworkNode,
    UnknownNetworkNode(IdentityId),
    InvalidMandateNodes,
    InvalidMandate,
    DuplicateMandate,
    InactiveWitnessCannotBeRaised,
    UnknownMandate(IdentityId),
    MandateCannotRaiseWitness,
    DuplicateWitness,
    UnknownWitness(IdentityId),
    WitnessAlreadyInactive,
    MandateAlreadyClosed,
    RemovalTargetMismatch,
    PowerCannotReviewItself,
    ChallengeWithoutEvidence,
    InvalidChallengeTransition,
    ChallengeNotReviewed,
    DuplicateConcurrence,
    InsufficientConcurrence,
    InvalidRemovingPowers,
    DuplicateSuccessionStage,
    MissingSuccessionStage(HypergiantSuccessionStage),
    InvalidInvestitureEvidence,
    IncompleteGateAccountability,
    InvalidDelegationTrace(StonebendConstitutionalPower),
}

impl fmt::Display for SecondPassValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for SecondPassValidationError {}
