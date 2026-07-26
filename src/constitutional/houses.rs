use std::fmt;

use crate::hollow_grove_contract::House;
use crate::institution::{InstitutionCatalog, InstitutionId, OfficeId};

use super::{AuthorityActorId, CausalPosition, EvidenceRef, HouseDecisionId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstitutionalJurisdictionSnapshot {
    pub institution: InstitutionId,
    pub house: House,
    pub observed_at: CausalPosition,
    pub evidence: Vec<EvidenceRef>,
}

impl InstitutionalJurisdictionSnapshot {
    pub fn from_catalog(
        catalog: &InstitutionCatalog,
        institution_id: &InstitutionId,
        observed_at: CausalPosition,
        evidence: Vec<EvidenceRef>,
    ) -> Result<Self, HouseLawError> {
        let institution = catalog
            .institution(institution_id)
            .ok_or_else(|| HouseLawError::MissingInstitution(institution_id.clone()))?;
        let house = institution
            .house
            .ok_or_else(|| HouseLawError::InstitutionWithoutHouse(institution_id.clone()))?;
        if evidence.is_empty() {
            return Err(HouseLawError::MissingJurisdictionEvidence(
                institution_id.clone(),
            ));
        }
        Ok(Self {
            institution: institution.id.clone(),
            house,
            observed_at,
            evidence,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HouseFunction {
    Name,
    Prove,
    Clear,
    Recognize,
    Resolve,
}

impl HouseFunction {
    #[must_use]
    pub const fn constitutional_house(self) -> House {
        match self {
            Self::Name => House::Stonebend,
            Self::Prove => House::Sandmanor,
            Self::Clear | Self::Resolve => House::Glaushouse,
            Self::Recognize => House::Flynt,
        }
    }

    #[must_use]
    pub const fn required_authority(self) -> &'static str {
        match self {
            Self::Name => "ConstitutionalIdentity",
            Self::Prove => "WitnessedImprovement",
            Self::Clear => "PublicClearance",
            Self::Recognize => "InstitutionalRecognition",
            Self::Resolve => "FinalJudgmentAnswerability",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HouseDecisionOutcome {
    Accepted,
    Rejected,
    Inconclusive,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthoritySnapshot {
    pub actor: AuthorityActorId,
    pub office: OfficeId,
    pub institution: Option<InstitutionId>,
    pub house: House,
    pub authorities: Vec<String>,
    pub observed_at: CausalPosition,
}

impl AuthoritySnapshot {
    pub fn from_catalog(
        catalog: &InstitutionCatalog,
        office_id: &OfficeId,
        actor: AuthorityActorId,
        observed_at: CausalPosition,
    ) -> Result<Self, HouseLawError> {
        let office = catalog
            .offices
            .iter()
            .find(|candidate| &candidate.id == office_id)
            .ok_or_else(|| HouseLawError::MissingOffice(office_id.clone()))?;
        let holder = catalog
            .office_holders
            .iter()
            .find(|holder| {
                holder.active
                    && &holder.office == office_id
                    && holder.being.as_str() == actor.as_str()
            })
            .ok_or_else(|| HouseLawError::InactiveAuthorityActor(actor.clone()))?;
        let _ = holder;
        let house = office
            .house
            .ok_or_else(|| HouseLawError::OfficeWithoutHouse(office_id.clone()))?;
        Ok(Self {
            actor,
            office: office.id.clone(),
            institution: office.institution.clone(),
            house,
            authorities: office.authority.clone(),
            observed_at,
        })
    }

    #[must_use]
    pub fn grants(&self, authority: &str) -> bool {
        self.authorities
            .iter()
            .any(|candidate| candidate == authority)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HouseDecision {
    pub id: HouseDecisionId,
    pub function: HouseFunction,
    pub authority: AuthoritySnapshot,
    pub outcome: HouseDecisionOutcome,
    pub evidence: Vec<EvidenceRef>,
    pub causal_position: CausalPosition,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HouseDecisionDraft {
    pub id: HouseDecisionId,
    pub function: HouseFunction,
    pub office: OfficeId,
    pub actor: AuthorityActorId,
    pub outcome: HouseDecisionOutcome,
    pub evidence: Vec<EvidenceRef>,
    pub causal_position: CausalPosition,
}

impl HouseDecision {
    pub fn from_catalog(
        catalog: &InstitutionCatalog,
        draft: HouseDecisionDraft,
    ) -> Result<Self, HouseLawError> {
        let HouseDecisionDraft {
            id,
            function,
            office,
            actor,
            outcome,
            evidence,
            causal_position,
        } = draft;
        let decision = Self {
            id,
            function,
            authority: AuthoritySnapshot::from_catalog(catalog, &office, actor, causal_position)?,
            outcome,
            evidence,
            causal_position,
        };
        decision.validate_for(function)?;
        Ok(decision)
    }

    pub fn validate_for(&self, function: HouseFunction) -> Result<(), HouseLawError> {
        if self.function != function {
            return Err(HouseLawError::WrongFunction {
                expected: function,
                actual: self.function,
            });
        }
        let expected_house = function.constitutional_house();
        if self.authority.house != expected_house {
            return Err(HouseLawError::WrongHouse {
                function,
                expected: expected_house,
                actual: self.authority.house,
            });
        }
        let required = function.required_authority();
        if !self.authority.grants(required) {
            return Err(HouseLawError::MissingAuthority {
                office: self.authority.office.clone(),
                authority: required,
            });
        }
        if self.evidence.is_empty() {
            return Err(HouseLawError::MissingEvidence(self.id.clone()));
        }
        if self.authority.observed_at > self.causal_position {
            return Err(HouseLawError::AuthorityFromFuture(self.id.clone()));
        }
        Ok(())
    }

    pub fn require_accepted(&self, function: HouseFunction) -> Result<(), HouseLawError> {
        self.validate_for(function)?;
        if self.outcome != HouseDecisionOutcome::Accepted {
            return Err(HouseLawError::DecisionNotAccepted(self.id.clone()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReservedHouseProcedure {
    StonebendTitleSuccession,
    SandmanorContestSuccession,
    GlaushousePrimaDonnaSuccession,
    HouseAppealCourt,
}

pub fn invoke_reserved_procedure(procedure: ReservedHouseProcedure) -> Result<(), HouseLawError> {
    Err(HouseLawError::ReservedProcedure(procedure))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourtAppealReferral {
    pub challenge: super::ChallengeId,
    pub court_system: InstitutionId,
}

/// Refers a constitutional challenge to the one ratified Minoan County Court
/// System. The constitutional layer names the receiving institution without
/// importing its world-model implementation or granting it House authority.
pub fn appeal_challenge(
    challenge: &super::ChallengeId,
) -> Result<CourtAppealReferral, HouseLawError> {
    Ok(CourtAppealReferral {
        challenge: challenge.clone(),
        court_system: InstitutionId::new("institution.sandmanor.minoan-county-courthouse")
            .expect("canonical Minoan County Court System institution ID"),
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HouseLawError {
    MissingInstitution(InstitutionId),
    InstitutionWithoutHouse(InstitutionId),
    MissingJurisdictionEvidence(InstitutionId),
    MissingOffice(OfficeId),
    InactiveAuthorityActor(AuthorityActorId),
    OfficeWithoutHouse(OfficeId),
    WrongFunction {
        expected: HouseFunction,
        actual: HouseFunction,
    },
    WrongHouse {
        function: HouseFunction,
        expected: House,
        actual: House,
    },
    MissingAuthority {
        office: OfficeId,
        authority: &'static str,
    },
    MissingEvidence(HouseDecisionId),
    AuthorityFromFuture(HouseDecisionId),
    DecisionNotAccepted(HouseDecisionId),
    ReservedProcedure(ReservedHouseProcedure),
}

impl fmt::Display for HouseLawError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "House-law violation: {self:?}")
    }
}

impl std::error::Error for HouseLawError {}
