//! Four-House proofs and Stonebend's final Laws of Permanence.
//!
//! The universal kernel remains House-neutral. This constitutional layer
//! requires Identity, Pattern, Integrity, and Recognition proofs together,
//! while reserving the final Permanence Seal to Stonebend.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::constitutional::{AuthoritativeTimestamp, CalendarEvidenceId, CanonicalYearId};
use crate::hollow_grove_contract::House;

pub const PERMANENCE_SCHEMA_VERSION: u16 = 1;
pub const PERMANENCE_SOURCE: &str = "FUNCTION_JUNCTION_SEASONAL_WORLD_CYCLE_AND_PERMANENCE_V1.md";
pub const PERMANENCE_MAXIM: &str = "The world cannot pretend the thing never happened.";
pub const PERMANENCE_CYCLE: &str = "Claim → Pattern → Integrity → Recognition → Title → Yield";
pub const ILLEGAL_HOLLOWING: &str = "Illegal Hollowing";

macro_rules! permanence_id {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, PermanenceError> {
                let value = value.into();
                if value.is_empty()
                    || !value.bytes().all(|byte| {
                        byte.is_ascii_lowercase()
                            || byte.is_ascii_digit()
                            || matches!(byte, b'.' | b'-')
                    })
                {
                    return Err(PermanenceError::InvalidIdentifier(value));
                }
                Ok(Self(value))
            }

            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl TryFrom<String> for $name {
            type Error = PermanenceError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(formatter)
            }
        }
    };
}

permanence_id!(PermanentSubjectId);
permanence_id!(PermanencePetitionId);
permanence_id!(PermanenceAttestationId);
permanence_id!(PermanenceAuthorityId);
permanence_id!(PermanenceSealId);
permanence_id!(PermanentVersionId);
permanence_id!(PermanentChangeId);
permanence_id!(PermanenceTombstoneId);
permanence_id!(YieldProtectionId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PermanenceLaw {
    Identity,
    Pattern,
    Integrity,
    Recognition,
}

impl PermanenceLaw {
    pub const ALL: [Self; 4] = [
        Self::Identity,
        Self::Pattern,
        Self::Integrity,
        Self::Recognition,
    ];

    #[must_use]
    pub const fn authority_house(self) -> House {
        match self {
            Self::Identity => House::Stonebend,
            Self::Pattern => House::Sandmanor,
            Self::Integrity => House::Glaushouse,
            Self::Recognition => House::Flynt,
        }
    }

    #[must_use]
    pub const fn constitutional_question(self) -> &'static str {
        match self {
            Self::Identity => "What exactly is being made permanent?",
            Self::Pattern => "What arrangement is being preserved?",
            Self::Integrity => "Can this endure without becoming harmful, dead, or fraudulent?",
            Self::Recognition => "Does the world actually recognize and operate this thing?",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AttestationStatus {
    Accepted,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermanenceAttestation {
    pub attestation_id: PermanenceAttestationId,
    pub canonical_year_id: CanonicalYearId,
    pub subject_id: PermanentSubjectId,
    pub law: PermanenceLaw,
    pub authority_house: House,
    pub authority_id: PermanenceAuthorityId,
    pub status: AttestationStatus,
    pub evidence_ids: BTreeSet<CalendarEvidenceId>,
    pub attested_at: AuthoritativeTimestamp,
    pub provenance_ids: BTreeSet<CalendarEvidenceId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermanencePetition {
    pub petition_id: PermanencePetitionId,
    pub canonical_year_id: CanonicalYearId,
    pub subject_id: PermanentSubjectId,
    pub claim_authority_id: PermanenceAuthorityId,
    pub freemason_claim: bool,
    pub attestation_ids: BTreeSet<PermanenceAttestationId>,
    pub evidence_ids: BTreeSet<CalendarEvidenceId>,
    pub opened_at: AuthoritativeTimestamp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum StonebendSealOffice {
    DiamondHypergiant,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermanenceSeal {
    pub seal_id: PermanenceSealId,
    pub petition_id: PermanencePetitionId,
    pub canonical_year_id: CanonicalYearId,
    pub subject_id: PermanentSubjectId,
    pub version_id: PermanentVersionId,
    pub issuing_house: House,
    pub office: StonebendSealOffice,
    pub stonebend_authority_id: PermanenceAuthorityId,
    pub supporting_attestation_ids: BTreeSet<PermanenceAttestationId>,
    pub evidence_ids: BTreeSet<CalendarEvidenceId>,
    pub yield_protection_id: YieldProtectionId,
    pub sealed_at: AuthoritativeTimestamp,
    pub immutable: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PermanentChangeKind {
    Amendment,
    Succession,
    Dissolution,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermanentChangeRecord {
    pub change_id: PermanentChangeId,
    pub canonical_year_id: CanonicalYearId,
    pub subject_id: PermanentSubjectId,
    pub kind: PermanentChangeKind,
    pub prior_version_id: PermanentVersionId,
    pub result_version_id: PermanentVersionId,
    pub authorizing_house: House,
    pub stonebend_authority_id: Option<PermanenceAuthorityId>,
    pub evidence_ids: BTreeSet<CalendarEvidenceId>,
    pub changed_at: AuthoritativeTimestamp,
    pub preserves_prior_version: bool,
    pub tombstone_id: Option<PermanenceTombstoneId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermanenceTombstone {
    pub tombstone_id: PermanenceTombstoneId,
    pub canonical_year_id: CanonicalYearId,
    pub subject_id: PermanentSubjectId,
    pub final_version_id: PermanentVersionId,
    pub dissolution_change_id: PermanentChangeId,
    pub evidence_ids: BTreeSet<CalendarEvidenceId>,
    pub recorded_at: AuthoritativeTimestamp,
    pub silently_deletes_history: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PermanentSubjectHistory {
    pub subject_id: PermanentSubjectId,
    pub seal_id: PermanenceSealId,
    pub versions: Vec<PermanentVersionId>,
    pub changes: Vec<PermanentChangeId>,
    pub tombstone_id: Option<PermanenceTombstoneId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PermanenceRuntime {
    attestations: BTreeMap<PermanenceAttestationId, PermanenceAttestation>,
    petitions: BTreeMap<PermanencePetitionId, PermanencePetition>,
    seals: BTreeMap<PermanenceSealId, PermanenceSeal>,
    changes: BTreeMap<PermanentChangeId, PermanentChangeRecord>,
    tombstones: BTreeMap<PermanenceTombstoneId, PermanenceTombstone>,
    histories: BTreeMap<PermanentSubjectId, PermanentSubjectHistory>,
}

impl PermanenceRuntime {
    pub fn replay(
        canonical_year_id: &CanonicalYearId,
        attestations: &[PermanenceAttestation],
        petitions: &[PermanencePetition],
        seals: &[PermanenceSeal],
        changes: &[PermanentChangeRecord],
        tombstones: &[PermanenceTombstone],
    ) -> Result<Self, PermanenceError> {
        let mut attestations_by_id = BTreeMap::new();
        for attestation in attestations {
            validate_attestation(canonical_year_id, attestation)?;
            if attestations_by_id
                .insert(attestation.attestation_id.clone(), attestation.clone())
                .is_some()
            {
                return Err(PermanenceError::DuplicateAttestation(
                    attestation.attestation_id.clone(),
                ));
            }
        }

        let mut petitions_by_id = BTreeMap::new();
        let mut petition_eligibility = BTreeMap::new();
        for petition in petitions {
            let eligible = validate_petition(canonical_year_id, petition, &attestations_by_id)?;
            if petitions_by_id
                .insert(petition.petition_id.clone(), petition.clone())
                .is_some()
            {
                return Err(PermanenceError::DuplicatePetition(
                    petition.petition_id.clone(),
                ));
            }
            petition_eligibility.insert(petition.petition_id.clone(), eligible);
        }

        let mut seals_by_id = BTreeMap::new();
        let mut seal_by_subject = BTreeMap::new();
        for seal in seals {
            validate_seal(
                canonical_year_id,
                seal,
                &petitions_by_id,
                &petition_eligibility,
            )?;
            if seals_by_id
                .insert(seal.seal_id.clone(), seal.clone())
                .is_some()
            {
                return Err(PermanenceError::DuplicateSeal(seal.seal_id.clone()));
            }
            if seal_by_subject
                .insert(seal.subject_id.clone(), seal.seal_id.clone())
                .is_some()
            {
                return Err(PermanenceError::DuplicatePermanentSubject(
                    seal.subject_id.clone(),
                ));
            }
        }

        let mut changes_by_id = BTreeMap::new();
        let mut changes_by_subject: BTreeMap<PermanentSubjectId, Vec<PermanentChangeRecord>> =
            BTreeMap::new();
        for change in changes {
            validate_change(canonical_year_id, change, &seal_by_subject)?;
            if changes_by_id
                .insert(change.change_id.clone(), change.clone())
                .is_some()
            {
                return Err(PermanenceError::DuplicateChange(change.change_id.clone()));
            }
            changes_by_subject
                .entry(change.subject_id.clone())
                .or_default()
                .push(change.clone());
        }

        let mut tombstones_by_id = BTreeMap::new();
        for tombstone in tombstones {
            validate_tombstone(canonical_year_id, tombstone, &changes_by_id)?;
            if tombstones_by_id
                .insert(tombstone.tombstone_id.clone(), tombstone.clone())
                .is_some()
            {
                return Err(PermanenceError::DuplicateTombstone(
                    tombstone.tombstone_id.clone(),
                ));
            }
        }

        let mut histories = BTreeMap::new();
        for (subject_id, seal_id) in &seal_by_subject {
            let seal = seals_by_id
                .get(seal_id)
                .expect("seal index was built from seal map");
            let mut ordered = changes_by_subject.remove(subject_id).unwrap_or_default();
            ordered.sort_by(|left, right| {
                left.changed_at
                    .cmp(&right.changed_at)
                    .then_with(|| left.change_id.cmp(&right.change_id))
            });
            let mut expected_prior = seal.version_id.clone();
            let mut versions = vec![seal.version_id.clone()];
            let mut change_ids = Vec::new();
            let mut tombstone_id = None;
            for change in ordered {
                if change.prior_version_id != expected_prior {
                    return Err(PermanenceError::BrokenVersionHistory(subject_id.clone()));
                }
                if tombstone_id.is_some() {
                    return Err(PermanenceError::ChangeAfterDissolution(change.change_id));
                }
                expected_prior = change.result_version_id.clone();
                versions.push(change.result_version_id.clone());
                change_ids.push(change.change_id.clone());
                if change.kind == PermanentChangeKind::Dissolution {
                    let expected = change.tombstone_id.clone().ok_or_else(|| {
                        PermanenceError::MissingDissolutionTombstone(change.change_id.clone())
                    })?;
                    let tombstone = tombstones_by_id.get(&expected).ok_or_else(|| {
                        PermanenceError::MissingDissolutionTombstone(change.change_id.clone())
                    })?;
                    if tombstone.subject_id != *subject_id
                        || tombstone.final_version_id != change.result_version_id
                    {
                        return Err(PermanenceError::InvalidTombstone(expected));
                    }
                    tombstone_id = Some(expected);
                }
            }
            histories.insert(
                subject_id.clone(),
                PermanentSubjectHistory {
                    subject_id: subject_id.clone(),
                    seal_id: seal_id.clone(),
                    versions,
                    changes: change_ids,
                    tombstone_id,
                },
            );
        }
        if !changes_by_subject.is_empty() {
            return Err(PermanenceError::ChangeWithoutSeal);
        }

        Ok(Self {
            attestations: attestations_by_id,
            petitions: petitions_by_id,
            seals: seals_by_id,
            changes: changes_by_id,
            tombstones: tombstones_by_id,
            histories,
        })
    }

    #[must_use]
    pub fn attestations(&self) -> &BTreeMap<PermanenceAttestationId, PermanenceAttestation> {
        &self.attestations
    }

    #[must_use]
    pub fn petitions(&self) -> &BTreeMap<PermanencePetitionId, PermanencePetition> {
        &self.petitions
    }

    #[must_use]
    pub fn seals(&self) -> &BTreeMap<PermanenceSealId, PermanenceSeal> {
        &self.seals
    }

    #[must_use]
    pub fn changes(&self) -> &BTreeMap<PermanentChangeId, PermanentChangeRecord> {
        &self.changes
    }

    #[must_use]
    pub fn tombstones(&self) -> &BTreeMap<PermanenceTombstoneId, PermanenceTombstone> {
        &self.tombstones
    }

    #[must_use]
    pub fn histories(&self) -> &BTreeMap<PermanentSubjectId, PermanentSubjectHistory> {
        &self.histories
    }
}

fn validate_attestation(
    year_id: &CanonicalYearId,
    attestation: &PermanenceAttestation,
) -> Result<(), PermanenceError> {
    if &attestation.canonical_year_id != year_id
        || attestation.authority_house != attestation.law.authority_house()
        || attestation.evidence_ids.is_empty()
        || attestation.provenance_ids.is_empty()
    {
        return Err(PermanenceError::InvalidAttestation(
            attestation.attestation_id.clone(),
        ));
    }
    Ok(())
}

fn validate_petition(
    year_id: &CanonicalYearId,
    petition: &PermanencePetition,
    attestations: &BTreeMap<PermanenceAttestationId, PermanenceAttestation>,
) -> Result<bool, PermanenceError> {
    if &petition.canonical_year_id != year_id
        || !petition.freemason_claim
        || petition.evidence_ids.is_empty()
        || petition.attestation_ids.len() != PermanenceLaw::ALL.len()
    {
        return Err(PermanenceError::InvalidPetition(
            petition.petition_id.clone(),
        ));
    }
    let mut laws = BTreeSet::new();
    let mut eligible = true;
    for id in &petition.attestation_ids {
        let attestation = attestations
            .get(id)
            .ok_or_else(|| PermanenceError::MissingAttestation(id.clone()))?;
        if attestation.subject_id != petition.subject_id {
            return Err(PermanenceError::ConflictingSubjectIdentity(
                petition.petition_id.clone(),
            ));
        }
        laws.insert(attestation.law);
        eligible &= attestation.status == AttestationStatus::Accepted;
    }
    if laws != PermanenceLaw::ALL.into_iter().collect() {
        return Err(PermanenceError::IncompleteProofSet(
            petition.petition_id.clone(),
        ));
    }
    Ok(eligible)
}

fn validate_seal(
    year_id: &CanonicalYearId,
    seal: &PermanenceSeal,
    petitions: &BTreeMap<PermanencePetitionId, PermanencePetition>,
    eligibility: &BTreeMap<PermanencePetitionId, bool>,
) -> Result<(), PermanenceError> {
    let petition = petitions
        .get(&seal.petition_id)
        .ok_or_else(|| PermanenceError::MissingPetition(seal.petition_id.clone()))?;
    if &seal.canonical_year_id != year_id
        || seal.subject_id != petition.subject_id
        || seal.issuing_house != House::Stonebend
        || seal.supporting_attestation_ids != petition.attestation_ids
        || !eligibility.get(&seal.petition_id).copied().unwrap_or(false)
        || seal.evidence_ids.is_empty()
        || seal.immutable
    {
        return Err(PermanenceError::InvalidSeal(seal.seal_id.clone()));
    }
    Ok(())
}

fn validate_change(
    year_id: &CanonicalYearId,
    change: &PermanentChangeRecord,
    seals_by_subject: &BTreeMap<PermanentSubjectId, PermanenceSealId>,
) -> Result<(), PermanenceError> {
    if &change.canonical_year_id != year_id
        || !seals_by_subject.contains_key(&change.subject_id)
        || change.authorizing_house != House::Stonebend
        || change.stonebend_authority_id.is_none()
        || change.evidence_ids.is_empty()
        || !change.preserves_prior_version
        || change.prior_version_id == change.result_version_id
        || (change.kind == PermanentChangeKind::Dissolution) != change.tombstone_id.is_some()
    {
        return Err(PermanenceError::IllegalHollowing(change.change_id.clone()));
    }
    Ok(())
}

fn validate_tombstone(
    year_id: &CanonicalYearId,
    tombstone: &PermanenceTombstone,
    changes: &BTreeMap<PermanentChangeId, PermanentChangeRecord>,
) -> Result<(), PermanenceError> {
    let change = changes
        .get(&tombstone.dissolution_change_id)
        .ok_or_else(|| PermanenceError::InvalidTombstone(tombstone.tombstone_id.clone()))?;
    if &tombstone.canonical_year_id != year_id
        || change.kind != PermanentChangeKind::Dissolution
        || change.tombstone_id.as_ref() != Some(&tombstone.tombstone_id)
        || change.subject_id != tombstone.subject_id
        || change.result_version_id != tombstone.final_version_id
        || tombstone.evidence_ids.is_empty()
        || tombstone.silently_deletes_history
    {
        return Err(PermanenceError::InvalidTombstone(
            tombstone.tombstone_id.clone(),
        ));
    }
    Ok(())
}

#[derive(Debug)]
pub enum PermanenceError {
    InvalidIdentifier(String),
    DuplicateAttestation(PermanenceAttestationId),
    DuplicatePetition(PermanencePetitionId),
    DuplicateSeal(PermanenceSealId),
    DuplicatePermanentSubject(PermanentSubjectId),
    DuplicateChange(PermanentChangeId),
    DuplicateTombstone(PermanenceTombstoneId),
    InvalidAttestation(PermanenceAttestationId),
    InvalidPetition(PermanencePetitionId),
    MissingAttestation(PermanenceAttestationId),
    ConflictingSubjectIdentity(PermanencePetitionId),
    IncompleteProofSet(PermanencePetitionId),
    MissingPetition(PermanencePetitionId),
    InvalidSeal(PermanenceSealId),
    IllegalHollowing(PermanentChangeId),
    InvalidTombstone(PermanenceTombstoneId),
    BrokenVersionHistory(PermanentSubjectId),
    ChangeAfterDissolution(PermanentChangeId),
    MissingDissolutionTombstone(PermanentChangeId),
    ChangeWithoutSeal,
}

impl fmt::Display for PermanenceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Permanence rejected state: {self:?}")
    }
}

impl std::error::Error for PermanenceError {}
