use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::constitutional::{
    CausalPosition, HouseDecision, HouseFunction, ParticipantId, scenario_house_decision,
};
use crate::institution::IdentityId;
use crate::world::geography::{ConstitutionalRouteId, ConstitutionalRouteVerb};
use crate::world::session::WorldSession;
use crate::world::stonebend::{
    DecisionRecord, DecisionRecordId, EvidenceRecord, EvidenceRecordId, NameClass, NameRecord,
    NameRecordId, NameStatus, PrincipalAuthority, SealRecord, SealRecordId,
};

use super::{HuemanFaculty, InteractionId};

pub const CURRENT_SEA_CONTINUITY_CASE_ID: &str = "case.stonebend.current-sea-continuity.v1";
pub const MERCY_DEEP_BEING_ID: &str = "being.current-sea.mercy-deep";
pub const MERCY_DEEP_PARTICIPANT_ID: &str = "participant.current-sea.mercy-deep";
pub const MERCY_DEEP_EXISTING_NAME: &str = "Mercy Deep";
pub const MERCY_DEEP_TRANSFORMED_NAME: &str = "Mercy Deep, Aftertide";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StonebendCaseId {
    CurrentSeaContinuityV1,
}

impl StonebendCaseId {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentSeaContinuityV1 => CURRENT_SEA_CONTINUITY_CASE_ID,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StonebendBeingId {
    MercyDeep,
}

impl StonebendBeingId {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MercyDeep => MERCY_DEEP_BEING_ID,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum StonebendEvidence {
    SubjectContinuityTestimony,
    PreTreatmentName,
    GlaushouseRestorationRecord,
    /// Stable archive variant for the Current Sea public-continuity witness.
    DepthEnduranceWitness,
    MercuryMirrorCorrespondence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StonebendContinuityChoice {
    AffirmExistingName,
    ProvisionalTransformedFormName,
    ReferIdentityConflict,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StonebendOutcomeId {
    ExistingContinuitySealedV1,
    ProvisionalTransformedFormNameV1,
    IdentityConflictReferredV1,
}

impl StonebendOutcomeId {
    #[must_use]
    pub const fn for_choice(choice: StonebendContinuityChoice) -> Self {
        match choice {
            StonebendContinuityChoice::AffirmExistingName => Self::ExistingContinuitySealedV1,
            StonebendContinuityChoice::ProvisionalTransformedFormName => {
                Self::ProvisionalTransformedFormNameV1
            }
            StonebendContinuityChoice::ReferIdentityConflict => Self::IdentityConflictReferredV1,
        }
    }

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExistingContinuitySealedV1 => "outcome.stonebend.existing-continuity-sealed.v1",
            Self::ProvisionalTransformedFormNameV1 => {
                "outcome.stonebend.provisional-transformed-form-name.v1"
            }
            Self::IdentityConflictReferredV1 => "outcome.stonebend.identity-conflict-referred.v1",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StonebendAuthorityClass {
    ConstitutionalIdentity,
    ProvisionalIdentityContinuity,
    HighIdentityReview,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StonebendCasePhase {
    Open,
    ReadyForSupport,
    SupportRecorded,
    Resolved,
}

/// Exact Stonebend records behind the gameplay outcome.
///
/// The Mercury Mirror appears only as evidence. The live Hypergiant performs
/// the Name act, and a Freemason Seal makes the scoped decision durable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StonebendContinuityDetermination {
    pub subject: IdentityId,
    pub evidence: Vec<EvidenceRecord>,
    pub decision: DecisionRecord,
    pub seal: SealRecord,
    pub name_records: Vec<NameRecord>,
}

impl StonebendContinuityDetermination {
    pub fn validate(
        &self,
        choice: StonebendContinuityChoice,
        house_decision: &HouseDecision,
    ) -> Result<(), StonebendCaseError> {
        if self.subject.as_str() != MERCY_DEEP_BEING_ID
            || house_decision.function != HouseFunction::Name
            || house_decision.authority.house != crate::hollow_grove_contract::House::Stonebend
            || house_decision.authority.actor.as_str().is_empty()
        {
            return Err(StonebendCaseError::InvalidConstitutionalRecord);
        }
        if self.evidence.len() != StonebendEvidence::ALL.len()
            || self
                .evidence
                .iter()
                .any(|record| record.subject != self.subject)
        {
            return Err(StonebendCaseError::InvalidConstitutionalRecord);
        }
        let evidence_ids = self
            .evidence
            .iter()
            .map(|record| record.id.clone())
            .collect::<BTreeSet<_>>();
        if evidence_ids.len() != self.evidence.len()
            || self.decision.authority != PrincipalAuthority::Hypergiant
            || self.decision.subject != self.subject
            || self.decision.scope.trim().is_empty()
            || self
                .decision
                .evidence
                .iter()
                .any(|record| !evidence_ids.contains(record))
            || !matches!(
                self.seal.issuing_authority,
                PrincipalAuthority::HighFreemason | PrincipalAuthority::FreemasonInstitution
            )
            || self.seal.subject != self.subject
            || self.seal.scope != self.decision.scope
            || self.seal.decision != self.decision.id
        {
            return Err(StonebendCaseError::InvalidConstitutionalRecord);
        }
        if self.name_records.iter().any(|record| {
            record.subject != self.subject
                || record.scope != self.decision.scope
                || record
                    .evidence
                    .iter()
                    .any(|evidence| !evidence_ids.contains(evidence))
        }) {
            return Err(StonebendCaseError::InvalidConstitutionalRecord);
        }
        match choice {
            StonebendContinuityChoice::AffirmExistingName => {
                if !matches!(
                    self.name_records.as_slice(),
                    [NameRecord {
                        class: NameClass::Personal,
                        status: NameStatus::Active,
                        exclusive: true,
                        ..
                    }]
                ) || self.name_records[0].name != MERCY_DEEP_EXISTING_NAME
                {
                    return Err(StonebendCaseError::InvalidConstitutionalRecord);
                }
            }
            StonebendContinuityChoice::ProvisionalTransformedFormName => {
                if self.name_records.len() != 2 {
                    return Err(StonebendCaseError::InvalidConstitutionalRecord);
                }
                let former = &self.name_records[0];
                let provisional = &self.name_records[1];
                if former.name != MERCY_DEEP_EXISTING_NAME
                    || former.status != NameStatus::Active
                    || provisional.name != MERCY_DEEP_TRANSFORMED_NAME
                    || provisional.class != NameClass::TransformedForm
                    || provisional.status != NameStatus::Provisional
                    || provisional.exclusive
                    || provisional.former_names != vec![former.id.clone()]
                {
                    return Err(StonebendCaseError::InvalidConstitutionalRecord);
                }
            }
            StonebendContinuityChoice::ReferIdentityConflict => {
                if !self.name_records.is_empty() || !self.decision.scope.contains("high review") {
                    return Err(StonebendCaseError::InvalidConstitutionalRecord);
                }
            }
        }
        Ok(())
    }
}

impl StonebendEvidence {
    pub const ALL: [Self; 5] = [
        Self::SubjectContinuityTestimony,
        Self::PreTreatmentName,
        Self::GlaushouseRestorationRecord,
        Self::DepthEnduranceWitness,
        Self::MercuryMirrorCorrespondence,
    ];
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StonebendOutcomeRecord {
    pub id: StonebendOutcomeId,
    pub case: StonebendCaseId,
    pub subject: StonebendBeingId,
    pub choice: StonebendContinuityChoice,
    pub authority_class: StonebendAuthorityClass,
    pub jurisdiction: ConstitutionalRouteId,
    pub participants: Vec<ParticipantId>,
    pub dominant_verb: ConstitutionalRouteVerb,
    pub trigger: &'static str,
    pub evidence: Vec<StonebendEvidence>,
    pub faculty_uncertainties: Vec<&'static str>,
    pub player_support_is_nonbinding: bool,
    pub lawful_state_change: &'static str,
    pub stonebend_naming: HouseDecision,
    pub determination: StonebendContinuityDetermination,
    pub title_granted: bool,
    pub persistence_and_replay: &'static str,
    pub presentation: &'static str,
    pub failure_and_refusal: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StonebendContinuityCase {
    id: StonebendCaseId,
    subject: StonebendBeingId,
    evidence: BTreeSet<StonebendEvidence>,
    faculties: BTreeSet<HuemanFaculty>,
    supported_choice: Option<StonebendContinuityChoice>,
    committed_choice: Option<StonebendContinuityChoice>,
    outcome: Option<StonebendOutcomeRecord>,
}

impl Default for StonebendContinuityCase {
    fn default() -> Self {
        Self::new()
    }
}

impl StonebendContinuityCase {
    #[must_use]
    pub fn new() -> Self {
        Self {
            id: StonebendCaseId::CurrentSeaContinuityV1,
            subject: StonebendBeingId::MercyDeep,
            evidence: BTreeSet::new(),
            faculties: BTreeSet::new(),
            supported_choice: None,
            committed_choice: None,
            outcome: None,
        }
    }

    #[must_use]
    pub const fn id(&self) -> StonebendCaseId {
        self.id
    }

    #[must_use]
    pub const fn subject(&self) -> StonebendBeingId {
        self.subject
    }

    #[must_use]
    pub fn evidence(&self) -> &BTreeSet<StonebendEvidence> {
        &self.evidence
    }

    #[must_use]
    pub fn faculties(&self) -> &BTreeSet<HuemanFaculty> {
        &self.faculties
    }

    #[must_use]
    pub const fn supported_choice(&self) -> Option<StonebendContinuityChoice> {
        self.supported_choice
    }

    #[must_use]
    pub const fn committed_choice(&self) -> Option<StonebendContinuityChoice> {
        self.committed_choice
    }

    #[must_use]
    pub const fn outcome(&self) -> Option<&StonebendOutcomeRecord> {
        self.outcome.as_ref()
    }

    #[must_use]
    pub fn phase(&self) -> StonebendCasePhase {
        if self.committed_choice.is_some() {
            StonebendCasePhase::Resolved
        } else if self.supported_choice.is_some() {
            StonebendCasePhase::SupportRecorded
        } else if self.is_ready() {
            StonebendCasePhase::ReadyForSupport
        } else {
            StonebendCasePhase::Open
        }
    }

    #[must_use]
    pub fn is_ready(&self) -> bool {
        self.evidence.len() == StonebendEvidence::ALL.len()
            && self.faculties.len() == HuemanFaculty::ALL.len()
    }

    pub fn observe_interaction(&mut self, target: InteractionId) -> Option<StonebendEvidence> {
        let evidence = match target {
            InteractionId::CurrentSeaMercyDeep => {
                Some(StonebendEvidence::SubjectContinuityTestimony)
            }
            InteractionId::CurrentSeaNameLedger => Some(StonebendEvidence::PreTreatmentName),
            InteractionId::CurrentSeaRestorationArchive => {
                Some(StonebendEvidence::GlaushouseRestorationRecord)
            }
            InteractionId::CurrentSeaDepthWitness => Some(StonebendEvidence::DepthEnduranceWitness),
            InteractionId::CurrentSeaMercuryMirror => {
                Some(StonebendEvidence::MercuryMirrorCorrespondence)
            }
            _ => None,
        };
        if self.committed_choice.is_none()
            && let Some(value) = evidence
        {
            self.evidence.insert(value);
        }
        evidence
    }

    pub fn disclose_faculty(&mut self, faculty: HuemanFaculty) -> Result<(), StonebendCaseError> {
        if self.committed_choice.is_some() {
            return Err(StonebendCaseError::AlreadyResolved);
        }
        self.faculties.insert(faculty);
        Ok(())
    }

    pub fn support(&mut self, choice: StonebendContinuityChoice) -> Result<(), StonebendCaseError> {
        if self.committed_choice.is_some() {
            return Err(StonebendCaseError::AlreadyResolved);
        }
        if !self.is_ready() {
            return Err(StonebendCaseError::CaseNotReady);
        }
        self.supported_choice = Some(choice);
        Ok(())
    }

    pub fn commit_with_authority(
        &mut self,
        at: CausalPosition,
        authority_world: &WorldSession,
    ) -> Result<StonebendContinuityChoice, StonebendCaseError> {
        if self.committed_choice.is_some() {
            return Err(StonebendCaseError::AlreadyResolved);
        }
        let choice = self
            .supported_choice
            .ok_or(StonebendCaseError::SupportRequired)?;
        let outcome = resolve_outcome(self, choice, at, authority_world)?;
        outcome
            .determination
            .validate(choice, &outcome.stonebend_naming)?;
        self.outcome = Some(outcome);
        self.committed_choice = Some(choice);
        Ok(choice)
    }
}

fn resolve_outcome(
    case: &StonebendContinuityCase,
    choice: StonebendContinuityChoice,
    at: CausalPosition,
    authority_world: &WorldSession,
) -> Result<StonebendOutcomeRecord, StonebendCaseError> {
    let key = choice_key(choice);
    let stonebend_naming = scenario_house_decision(
        &authority_world.institutional().catalog,
        &format!("current-sea.mercy-deep.{key}"),
        HouseFunction::Name,
        at.get(),
    )
    .map_err(|error| StonebendCaseError::Constitutional(error.to_string()))?;
    let determination = determination(choice)?;
    let (authority_class, lawful_state_change, presentation, uncertainties, refusal) =
        outcome_terms(choice);
    Ok(StonebendOutcomeRecord {
        id: StonebendOutcomeId::for_choice(choice),
        case: case.id,
        subject: case.subject,
        choice,
        authority_class,
        jurisdiction: ConstitutionalRouteId::CurrentSea,
        participants: vec![
            ParticipantId::new(MERCY_DEEP_PARTICIPANT_ID)
                .map_err(|error| StonebendCaseError::Constitutional(error.to_string()))?,
        ],
        dominant_verb: ConstitutionalRouteVerb::Certify,
        trigger: "Mercy Deep petitions after five evidence sources and all five Hueman faculties are disclosed",
        evidence: case.evidence.iter().copied().collect(),
        faculty_uncertainties: uncertainties,
        player_support_is_nonbinding: true,
        lawful_state_change,
        stonebend_naming,
        determination,
        title_granted: false,
        persistence_and_replay: "the choice event, live authority snapshot, evidence IDs, Name records, decision, and Seal replay together",
        presentation,
        failure_and_refusal: refusal,
    })
}

fn determination(
    choice: StonebendContinuityChoice,
) -> Result<StonebendContinuityDetermination, StonebendCaseError> {
    let subject = IdentityId::new(MERCY_DEEP_BEING_ID)
        .map_err(|error| StonebendCaseError::Constitutional(error.to_string()))?;
    let evidence = [
        (
            "evidence.current-sea.mercy-deep.testimony",
            "Mercy Deep identifies memories, obligations, and chosen Name across restoration",
            "capable subject testimony at the deep-certification landing",
        ),
        (
            "evidence.current-sea.mercy-deep.prior-name",
            "the pre-treatment registry identifies Mercy Deep in the same civic scope",
            "sealed Stonebend Name ledger predating restoration",
        ),
        (
            "evidence.current-sea.mercy-deep.restoration",
            "Glaüshouse records material restoration without making an identity decision",
            "Glaüshouse discharge archive and treatment provenance",
        ),
        (
            "evidence.current-sea.mercy-deep.depth-witness",
            "a crowd witness observed continuous identity through dense public circulation",
            "witnessed Current Sea concourse passage",
        ),
        (
            "evidence.current-sea.mercy-deep.mirror",
            "Mercury Mirror correspondence links present and prior structure with unresolved differences",
            "non-sovereign Mercury Mirror comparison",
        ),
    ]
    .into_iter()
    .map(|(id, description, provenance)| {
        Ok(EvidenceRecord {
            id: EvidenceRecordId::new(id)?,
            subject: subject.clone(),
            description: description.into(),
            provenance: provenance.into(),
        })
    })
    .collect::<Result<Vec<_>, crate::world::stonebend::StonebendIdError>>()
    .map_err(|error| StonebendCaseError::Constitutional(error.to_string()))?;
    let evidence_ids = evidence
        .iter()
        .map(|record| record.id.clone())
        .collect::<Vec<_>>();
    let scope = match choice {
        StonebendContinuityChoice::AffirmExistingName => "Current Sea civic identity continuity",
        StonebendContinuityChoice::ProvisionalTransformedFormName => {
            "Current Sea provisional transformed-form reference"
        }
        StonebendContinuityChoice::ReferIdentityConflict => {
            "Current Sea identity conflict held for high review"
        }
    }
    .to_owned();
    let decision = DecisionRecord {
        id: DecisionRecordId::new(format!(
            "decision.current-sea.mercy-deep.{0}",
            choice_key(choice)
        ))
        .map_err(|error| StonebendCaseError::Constitutional(error.to_string()))?,
        authority: PrincipalAuthority::Hypergiant,
        subject: subject.clone(),
        scope: scope.clone(),
        evidence: evidence_ids.clone(),
    };
    let seal = SealRecord {
        id: SealRecordId::new(format!(
            "seal.current-sea.mercy-deep.{0}",
            choice_key(choice)
        ))
        .map_err(|error| StonebendCaseError::Constitutional(error.to_string()))?,
        issuing_authority: PrincipalAuthority::HighFreemason,
        subject: subject.clone(),
        scope: scope.clone(),
        decision: decision.id.clone(),
    };
    let former_id = NameRecordId::new("name.current-sea.mercy-deep")
        .map_err(|error| StonebendCaseError::Constitutional(error.to_string()))?;
    let name_records = match choice {
        StonebendContinuityChoice::AffirmExistingName => vec![NameRecord {
            id: former_id,
            subject: subject.clone(),
            name: MERCY_DEEP_EXISTING_NAME.into(),
            class: NameClass::Personal,
            scope,
            exclusive: true,
            status: NameStatus::Active,
            evidence: evidence_ids,
            former_names: vec![],
        }],
        StonebendContinuityChoice::ProvisionalTransformedFormName => vec![
            NameRecord {
                id: former_id.clone(),
                subject: subject.clone(),
                name: MERCY_DEEP_EXISTING_NAME.into(),
                class: NameClass::Personal,
                scope: scope.clone(),
                exclusive: true,
                status: NameStatus::Active,
                evidence: evidence_ids.clone(),
                former_names: vec![],
            },
            NameRecord {
                id: NameRecordId::new("name.current-sea.mercy-deep.aftertide")
                    .map_err(|error| StonebendCaseError::Constitutional(error.to_string()))?,
                subject: subject.clone(),
                name: MERCY_DEEP_TRANSFORMED_NAME.into(),
                class: NameClass::TransformedForm,
                scope,
                exclusive: false,
                status: NameStatus::Provisional,
                evidence: evidence_ids,
                former_names: vec![former_id],
            },
        ],
        StonebendContinuityChoice::ReferIdentityConflict => vec![],
    };
    Ok(StonebendContinuityDetermination {
        subject,
        evidence,
        decision,
        seal,
        name_records,
    })
}

type OutcomeTerms = (
    StonebendAuthorityClass,
    &'static str,
    &'static str,
    Vec<&'static str>,
    Vec<&'static str>,
);

fn outcome_terms(choice: StonebendContinuityChoice) -> OutcomeTerms {
    match choice {
        StonebendContinuityChoice::AffirmExistingName => (
            StonebendAuthorityClass::ConstitutionalIdentity,
            "Mercy Deep's existing Name and continuity are sealed in the Current Sea civic scope",
            "the prior and restored silhouettes align beneath one durable Name and open certification lane",
            vec![
                "future changes remain separately reviewable",
                "clinical success remains evidence rather than identity authority",
            ],
            vec![
                "Mercy Deep may challenge any later substitution",
                "the Name grants no Title, office, ownership, or clinical Clearance",
            ],
        ),
        StonebendContinuityChoice::ProvisionalTransformedFormName => (
            StonebendAuthorityClass::ProvisionalIdentityContinuity,
            "a provisional transformed-form Name preserves Mercy Deep and the former Name without deciding total continuity",
            "old and present silhouettes remain visibly linked while the provisional lane opens",
            vec![
                "the extent of material continuity remains under review",
                "public familiarity may lag behind the provisional form",
            ],
            vec![
                "Mercy Deep may revoke the petition before a final exclusive determination",
                "provisional status cannot grant Title or erase the former Name",
            ],
        ),
        StonebendContinuityChoice::ReferIdentityConflict => (
            StonebendAuthorityClass::HighIdentityReview,
            "the conflict, evidence, and reason for uncertainty are sealed while no final Name acts",
            "the certification lane remains closed and the Mercury Mirror points toward a visible appeal route",
            vec![
                "high review may affirm, revise, or reject the continuity claim",
                "the Mirror cannot decide which outcome should follow",
            ],
            vec![
                "Mercy Deep may challenge substitution and supply new evidence",
                "delay cannot become disappearance, forced renaming, or adverse Title inference",
            ],
        ),
    }
}

const fn choice_key(choice: StonebendContinuityChoice) -> &'static str {
    match choice {
        StonebendContinuityChoice::AffirmExistingName => "affirm-existing-name",
        StonebendContinuityChoice::ProvisionalTransformedFormName => {
            "provisional-transformed-form-name"
        }
        StonebendContinuityChoice::ReferIdentityConflict => "refer-identity-conflict",
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StonebendCaseError {
    CaseNotReady,
    SupportRequired,
    AlreadyResolved,
    InvalidConstitutionalRecord,
    Constitutional(String),
}

impl std::fmt::Display for StonebendCaseError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "Stonebend continuity case rejected action: {self:?}"
        )
    }
}

impl std::error::Error for StonebendCaseError {}
