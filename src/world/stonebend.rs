//! Executable Stonebend constitutional law above the frozen Constitutional
//! Runtime V2.
//!
//! This module owns Stonebend-specific identity, Title, Seal, Hollowing,
//! succession, and Tombstone validation. It does not alter the recursion
//! kernel, make a generic House decision, clear Synthesis, prove a Sandmanor
//! design, or grant Flynt recognition.

pub mod foundation;
pub mod second_pass;
pub mod third_pass;

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::institution::{IdentityId, InstitutionId, OfficeId};

pub const STONEBEND_CONSTITUTION_SOURCE: &str = "STONEBEND_CONSTITUTION_V2.md";
pub const STONEBEND_GOVERNING_VERB: &str = "Name";
pub const STONEBEND_SIGNATURE_OFFENSE: &str = "Illegal Hollowing";

#[cfg(test)]
fn identity(value: &str) -> IdentityId {
    IdentityId::new(value).expect("canonical Stonebend identity ID")
}

fn institution(value: &str) -> InstitutionId {
    InstitutionId::new(value).expect("canonical Stonebend institution ID")
}

fn office(value: &str) -> OfficeId {
    OfficeId::new(value).expect("canonical Stonebend office ID")
}

#[must_use]
pub fn stonebend_constitution_id() -> InstitutionId {
    institution("institution.stonebend.constitution")
}

#[must_use]
pub fn proliteriate_id() -> InstitutionId {
    institution("institution.stonebend.proliteriate")
}

#[must_use]
pub fn freemason_institution_id() -> InstitutionId {
    institution("institution.stonebend.freemason")
}

#[must_use]
pub fn hypergiant_office_id() -> OfficeId {
    office("office.stonebend.hypergiant")
}

#[must_use]
pub fn high_freemason_office_id() -> OfficeId {
    office("office.stonebend.high-freemason")
}

macro_rules! stonebend_id {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, StonebendIdError> {
                let value = value.into();
                if value.is_empty()
                    || !value.bytes().all(|byte| {
                        byte.is_ascii_lowercase()
                            || byte.is_ascii_digit()
                            || byte == b'.'
                            || byte == b'-'
                    })
                {
                    return Err(StonebendIdError(value));
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
pub struct StonebendIdError(String);

impl fmt::Display for StonebendIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "invalid Stonebend stable identifier: {}", self.0)
    }
}

impl std::error::Error for StonebendIdError {}

stonebend_id!(NameRecordId);
stonebend_id!(TitleRecordId);
stonebend_id!(DecisionRecordId);
stonebend_id!(AccessionRecordId);
stonebend_id!(SealRecordId);
stonebend_id!(EvidenceRecordId);
stonebend_id!(HollowingRecordId);
stonebend_id!(ExtractedHollowRecordId);
stonebend_id!(CustodyRecordId);
stonebend_id!(TombstoneRecordId);
stonebend_id!(RenameRecordId);
stonebend_id!(SuccessionRecordId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PrincipalAuthority {
    Hypergiant,
    Proliteriate,
    HighFreemason,
    FreemasonInstitution,
}

impl PrincipalAuthority {
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Hypergiant => "Hypergiant",
            Self::Proliteriate => "Proliteriate",
            Self::HighFreemason => "High Freemason",
            Self::FreemasonInstitution => "Freemason",
        }
    }

    #[must_use]
    pub const fn constitutional_power(self) -> ConstitutionalPower {
        match self {
            Self::Hypergiant => ConstitutionalPower::Title,
            Self::Proliteriate => ConstitutionalPower::Yield,
            Self::HighFreemason | Self::FreemasonInstitution => ConstitutionalPower::Claim,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ConstitutionalPower {
    Claim,
    Title,
    Yield,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConstitutionalPlacement {
    SingularHighestOffice,
    DistributedPublicNetwork,
    SingularInstitutionalOffice,
    StructuralExecutionInstitution,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AuthorityDefinition {
    pub authority: PrincipalAuthority,
    pub placement: ConstitutionalPlacement,
    pub singular: bool,
}

pub const PRINCIPAL_AUTHORITIES: [AuthorityDefinition; 4] = [
    AuthorityDefinition {
        authority: PrincipalAuthority::Hypergiant,
        placement: ConstitutionalPlacement::SingularHighestOffice,
        singular: true,
    },
    AuthorityDefinition {
        authority: PrincipalAuthority::Proliteriate,
        placement: ConstitutionalPlacement::DistributedPublicNetwork,
        singular: false,
    },
    AuthorityDefinition {
        authority: PrincipalAuthority::HighFreemason,
        placement: ConstitutionalPlacement::SingularInstitutionalOffice,
        singular: true,
    },
    AuthorityDefinition {
        authority: PrincipalAuthority::FreemasonInstitution,
        placement: ConstitutionalPlacement::StructuralExecutionInstitution,
        singular: false,
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConstitutionalOffice {
    Hypergiant,
    HighFreemason,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HypergiantSelectionPhase {
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

impl HypergiantSelectionPhase {
    #[must_use]
    pub const fn next(self) -> Option<Self> {
        match self {
            Self::ClaimPresented => Some(Self::FreemasonExamination),
            Self::FreemasonExamination => Some(Self::ProliteriateYieldHearing),
            Self::ProliteriateYieldHearing => Some(Self::ProtectedElevationRelinquished),
            Self::ProtectedElevationRelinquished => Some(Self::ConsequenceDescentCompleted),
            Self::ConsequenceDescentCompleted => Some(Self::FlyntProofOfPersistence),
            Self::FlyntProofOfPersistence => Some(Self::LazerhornClimbed),
            Self::LazerhornClimbed => Some(Self::AccessionEligible),
            Self::AccessionEligible => Some(Self::DiamondInvested),
            Self::DiamondInvested => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HypergiantSelectionProcess {
    candidate: IdentityId,
    phase: HypergiantSelectionPhase,
    evidence: Vec<EvidenceRecordId>,
}

impl HypergiantSelectionProcess {
    #[must_use]
    pub fn open(candidate: IdentityId) -> Self {
        Self {
            candidate,
            phase: HypergiantSelectionPhase::ClaimPresented,
            evidence: Vec::new(),
        }
    }

    #[must_use]
    pub const fn candidate(&self) -> &IdentityId {
        &self.candidate
    }

    #[must_use]
    pub const fn phase(&self) -> HypergiantSelectionPhase {
        self.phase
    }

    #[must_use]
    pub fn evidence(&self) -> &[EvidenceRecordId] {
        &self.evidence
    }

    pub fn advance(
        &mut self,
        next: HypergiantSelectionPhase,
        evidence: EvidenceRecordId,
    ) -> Result<(), StonebendValidationError> {
        let expected = self
            .phase
            .next()
            .ok_or(StonebendValidationError::CompletedSelectionCannotAdvance)?;
        if next != expected {
            return Err(StonebendValidationError::InvalidSelectionTransition {
                from: self.phase,
                expected,
                attempted: next,
            });
        }
        self.phase = next;
        self.evidence.push(evidence);
        Ok(())
    }

    pub fn require_complete(&self) -> Result<(), StonebendValidationError> {
        if self.phase == HypergiantSelectionPhase::DiamondInvested && !self.evidence.is_empty() {
            Ok(())
        } else {
            Err(StonebendValidationError::IncompleteSelection(self.phase))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SubjectKind {
    Being,
    Creature,
    TransformedForm,
    Artifact,
    Material,
    Structure,
    GeographicPlace,
    Office,
    Institution,
    Record,
    Bond,
    Contract,
    Claim,
    Estate,
    ExtractedHollow,
    PublicWork,
    ConstitutionalInstrument,
    SoftwareModule,
    PersistentRuntimeEntity,
    SuccessorEntity,
    Concept,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubjectRecord {
    pub id: IdentityId,
    pub kind: SubjectKind,
    pub continuity_established: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NameClass {
    Personal,
    Civic,
    Institutional,
    Geographic,
    Structural,
    Office,
    Artifact,
    Lineage,
    TransformedForm,
    Provisional,
    Ceremonial,
    Alias,
    Historical,
    Successor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NameStatus {
    Provisional,
    Active,
    Tombstoned,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NameRecord {
    pub id: NameRecordId,
    pub subject: IdentityId,
    pub name: String,
    pub class: NameClass,
    pub scope: String,
    pub exclusive: bool,
    pub status: NameStatus,
    pub evidence: Vec<EvidenceRecordId>,
    pub former_names: Vec<NameRecordId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TitleClass {
    Office,
    Property,
    Custodial,
    Stewardship,
    Jurisdictional,
    Professional,
    Hereditary,
    Temporary,
    Emergency,
    Delegated,
    Collective,
    Institutional,
    Successor,
    Ceremonial,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TitleStatus {
    Pending,
    Active,
    Suspended,
    Revoked,
    Expired,
    Tombstoned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TitleOrigin {
    LawfulGrant,
    Transformation,
    Recognition,
    Clearance,
    LegacyProgression,
    CustodyOnly,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TitleRecord {
    pub id: TitleRecordId,
    pub subject: IdentityId,
    pub name: NameRecordId,
    pub class: TitleClass,
    pub legal_basis: String,
    pub status: TitleStatus,
    pub expires_at: Option<u64>,
    pub origin: TitleOrigin,
    pub office: Option<ConstitutionalOffice>,
    pub accession: Option<AccessionRecordId>,
    pub evidence: Vec<EvidenceRecordId>,
    pub seal: SealRecordId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceRecord {
    pub id: EvidenceRecordId,
    pub subject: IdentityId,
    pub description: String,
    pub provenance: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionRecord {
    pub id: DecisionRecordId,
    pub authority: PrincipalAuthority,
    pub subject: IdentityId,
    pub scope: String,
    pub evidence: Vec<EvidenceRecordId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessionBasis {
    Hypergiant {
        lawful_vacancy: bool,
        stable_claim_presented: bool,
        freemason_independently_examined: bool,
        proliteriate_yield_hearing_completed: bool,
        protected_elevation_relinquished: bool,
        consequence_descent_completed: bool,
        proof_of_persistence_completed: bool,
        lazerhorn_climbed: bool,
        public_oath_recorded: bool,
    },
    HighFreemason {
        institution_nominated: bool,
        proliteriate_reviewed: bool,
        hypergiant_confirmed: bool,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessionRecord {
    pub id: AccessionRecordId,
    pub title: TitleRecordId,
    pub holder: IdentityId,
    pub office: ConstitutionalOffice,
    pub basis: AccessionBasis,
    pub evidence: Vec<EvidenceRecordId>,
    pub seal: SealRecordId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SealRecord {
    pub id: SealRecordId,
    pub issuing_authority: PrincipalAuthority,
    pub subject: IdentityId,
    pub scope: String,
    pub decision: DecisionRecordId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HollowingPurpose {
    Medicine,
    Repair,
    Investigation,
    Rescue,
    StructuralSafety,
    EvidencePreservation,
    RemoveCorruption,
    AuthorizedExtraction,
    ConstitutionalTransition,
    Decommissioning,
    Restoration,
    Defense,
    EmergencyStabilization,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HollowingConsent {
    NotApplicable {
        reason: String,
    },
    Granted {
        informed: bool,
        specific: bool,
        voluntary: bool,
        capacity_confirmed: bool,
        revocability_recorded: bool,
        evidence: EvidenceRecordId,
    },
    Emergency {
        immediate_harm_recorded: bool,
        limited_to_necessity: bool,
        post_event_review: Option<DecisionRecordId>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HollowingRecord {
    pub id: HollowingRecordId,
    pub subject: IdentityId,
    pub authority: DecisionRecordId,
    pub purpose: HollowingPurpose,
    pub scope: String,
    pub consent: HollowingConsent,
    pub pre_procedure_evidence: Vec<EvidenceRecordId>,
    pub qualified_operator: IdentityId,
    pub safety_plan: String,
    pub custody_plan: String,
    pub procedure_record: String,
    pub extracted_hollow: Vec<ExtractedHollowRecordId>,
    pub post_procedure_evidence: Vec<EvidenceRecordId>,
    pub identity_continuity_determined: bool,
    pub restoration_or_disposition_plan: String,
    pub seal: SealRecordId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtractedHollowRecord {
    pub id: ExtractedHollowRecordId,
    pub source_subject: IdentityId,
    pub source_hollowing: HollowingRecordId,
    pub provenance: Vec<EvidenceRecordId>,
    pub custody: CustodyRecordId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustodyRecord {
    pub id: CustodyRecordId,
    pub custodian: IdentityId,
    pub subject: IdentityId,
    pub chain_of_custody: Vec<EvidenceRecordId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TombstoneTarget {
    Name(NameRecordId),
    Title(TitleRecordId),
    Subject(IdentityId),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TombstoneRecord {
    pub id: TombstoneRecordId,
    pub target: TombstoneTarget,
    pub former_name: String,
    pub former_titles: Vec<TitleRecordId>,
    pub reason: String,
    pub sequence: u64,
    pub successor: Option<IdentityId>,
    pub surviving_obligations: Vec<String>,
    pub preserved_evidence: Vec<EvidenceRecordId>,
    pub access_restrictions: String,
    pub issuing_authority: PrincipalAuthority,
    pub seal: SealRecordId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenameRecord {
    pub id: RenameRecordId,
    pub subject: IdentityId,
    pub former_name: NameRecordId,
    pub new_name: NameRecordId,
    pub reason: String,
    pub continuity_preserved: bool,
    pub history_preserved: bool,
    pub seal: SealRecordId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuccessionRecord {
    pub id: SuccessionRecordId,
    pub predecessor: IdentityId,
    pub successor: IdentityId,
    pub continuity_proven: bool,
    pub differences_recorded: bool,
    pub predecessor_benefits: Vec<String>,
    pub successor_benefits: Vec<String>,
    pub predecessor_obligations: Vec<String>,
    pub successor_obligations: Vec<String>,
    pub evidence: Vec<EvidenceRecordId>,
    pub seal: SealRecordId,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StonebendRegistry {
    pub subjects: Vec<SubjectRecord>,
    pub names: Vec<NameRecord>,
    pub titles: Vec<TitleRecord>,
    pub evidence: Vec<EvidenceRecord>,
    pub decisions: Vec<DecisionRecord>,
    pub accessions: Vec<AccessionRecord>,
    pub seals: Vec<SealRecord>,
    pub hollowings: Vec<HollowingRecord>,
    pub extracted_hollow: Vec<ExtractedHollowRecord>,
    pub custody: Vec<CustodyRecord>,
    pub tombstones: Vec<TombstoneRecord>,
    pub renames: Vec<RenameRecord>,
    pub successions: Vec<SuccessionRecord>,
}

impl StonebendRegistry {
    pub fn validate(&self) -> Result<(), StonebendValidationError> {
        validate_unique(self.subjects.iter().map(|record| &record.id), "subject")?;
        validate_unique(self.names.iter().map(|record| &record.id), "Name")?;
        validate_unique(self.titles.iter().map(|record| &record.id), "Title")?;
        validate_unique(self.evidence.iter().map(|record| &record.id), "evidence")?;
        validate_unique(self.decisions.iter().map(|record| &record.id), "decision")?;
        validate_unique(self.accessions.iter().map(|record| &record.id), "accession")?;
        validate_unique(self.seals.iter().map(|record| &record.id), "Seal")?;
        validate_unique(self.hollowings.iter().map(|record| &record.id), "Hollowing")?;
        validate_unique(
            self.extracted_hollow.iter().map(|record| &record.id),
            "extracted Hollow",
        )?;
        validate_unique(self.custody.iter().map(|record| &record.id), "custody")?;
        validate_unique(self.tombstones.iter().map(|record| &record.id), "Tombstone")?;
        validate_unique(self.renames.iter().map(|record| &record.id), "rename")?;
        validate_unique(
            self.successions.iter().map(|record| &record.id),
            "succession",
        )?;

        let subjects = self
            .subjects
            .iter()
            .map(|record| (&record.id, record))
            .collect::<BTreeMap<_, _>>();
        let names = self
            .names
            .iter()
            .map(|record| (&record.id, record))
            .collect::<BTreeMap<_, _>>();
        let titles = self
            .titles
            .iter()
            .map(|record| (&record.id, record))
            .collect::<BTreeMap<_, _>>();
        let evidence = self
            .evidence
            .iter()
            .map(|record| &record.id)
            .collect::<BTreeSet<_>>();
        let decisions = self
            .decisions
            .iter()
            .map(|record| (&record.id, record))
            .collect::<BTreeMap<_, _>>();
        let accessions = self
            .accessions
            .iter()
            .map(|record| (&record.id, record))
            .collect::<BTreeMap<_, _>>();
        let seals = self
            .seals
            .iter()
            .map(|record| (&record.id, record))
            .collect::<BTreeMap<_, _>>();

        for subject in &self.subjects {
            if !subject.continuity_established {
                return Err(StonebendValidationError::UnstableSubject(
                    subject.id.clone(),
                ));
            }
        }
        for record in &self.evidence {
            require_subject(&subjects, &record.subject)?;
            require_text(&record.description, "evidence description")?;
            require_text(&record.provenance, "evidence provenance")?;
        }
        for record in &self.names {
            require_subject(&subjects, &record.subject)?;
            require_text(&record.name, "Name")?;
            require_text(&record.scope, "Name scope")?;
            require_evidence(&evidence, &record.evidence)?;
            for former in &record.former_names {
                if !names.contains_key(former) {
                    return Err(StonebendValidationError::MissingRecord(
                        "former Name",
                        former.to_string(),
                    ));
                }
            }
        }
        let mut exclusive_names = BTreeSet::new();
        for record in self
            .names
            .iter()
            .filter(|record| record.status == NameStatus::Active && record.exclusive)
        {
            if !exclusive_names.insert((record.scope.as_str(), record.name.as_str())) {
                return Err(StonebendValidationError::ContradictoryExclusiveIdentity {
                    scope: record.scope.clone(),
                    name: record.name.clone(),
                });
            }
        }
        for record in &self.decisions {
            require_subject(&subjects, &record.subject)?;
            require_text(&record.scope, "decision scope")?;
            require_evidence(&evidence, &record.evidence)?;
        }
        for record in &self.seals {
            require_subject(&subjects, &record.subject)?;
            require_text(&record.scope, "Seal scope")?;
            let decision = decisions.get(&record.decision).ok_or_else(|| {
                StonebendValidationError::MissingRecord("decision", record.decision.to_string())
            })?;
            if decision.subject != record.subject || decision.scope != record.scope {
                return Err(StonebendValidationError::SealMismatch(record.id.clone()));
            }
            if !matches!(
                record.issuing_authority,
                PrincipalAuthority::HighFreemason | PrincipalAuthority::FreemasonInstitution
            ) {
                return Err(StonebendValidationError::InvalidSealIssuer(
                    record.id.clone(),
                ));
            }
        }

        let mut active_hypergiants = Vec::new();
        let mut active_high_freemasons = Vec::new();
        for record in &self.titles {
            require_subject(&subjects, &record.subject)?;
            let name = names.get(&record.name).ok_or_else(|| {
                StonebendValidationError::MissingRecord("Name", record.name.to_string())
            })?;
            if name.subject != record.subject
                || (record.status == TitleStatus::Active && name.status != NameStatus::Active)
            {
                return Err(StonebendValidationError::TitleNameMismatch(
                    record.id.clone(),
                ));
            }
            require_text(&record.legal_basis, "Title legal basis")?;
            require_evidence(&evidence, &record.evidence)?;
            require_subject_seal(&seals, &record.seal, &record.subject)?;
            if record.status == TitleStatus::Active && record.origin != TitleOrigin::LawfulGrant {
                return Err(StonebendValidationError::UnlawfulTitleOrigin(
                    record.id.clone(),
                ));
            }
            if record.class == TitleClass::Office && record.office.is_none() {
                return Err(StonebendValidationError::OfficeTitleWithoutOffice(
                    record.id.clone(),
                ));
            }
            if record.status == TitleStatus::Active
                && record.class == TitleClass::Emergency
                && record.expires_at.is_none()
            {
                return Err(StonebendValidationError::EmergencyTitleWithoutExpiration(
                    record.id.clone(),
                ));
            }
            if record.status == TitleStatus::Active
                && let Some(office) = record.office
            {
                let accession_id = record.accession.as_ref().ok_or_else(|| {
                    StonebendValidationError::ActiveOfficeWithoutAccession(record.id.clone())
                })?;
                let accession = accessions.get(accession_id).ok_or_else(|| {
                    StonebendValidationError::MissingRecord("accession", accession_id.to_string())
                })?;
                if accession.title != record.id
                    || accession.holder != record.subject
                    || accession.office != office
                {
                    return Err(StonebendValidationError::AccessionMismatch(
                        accession.id.clone(),
                    ));
                }
                match office {
                    ConstitutionalOffice::Hypergiant => active_hypergiants.push(&record.subject),
                    ConstitutionalOffice::HighFreemason => {
                        active_high_freemasons.push(&record.subject)
                    }
                }
            }
        }
        if active_hypergiants.len() > 1 {
            return Err(StonebendValidationError::ActiveHypergiantCount(
                active_hypergiants.len(),
            ));
        }
        if active_high_freemasons.len() > 1 {
            return Err(StonebendValidationError::ActiveHighFreemasonCount(
                active_high_freemasons.len(),
            ));
        }
        if active_high_freemasons
            .iter()
            .any(|holder| active_hypergiants.contains(holder))
        {
            return Err(StonebendValidationError::IncompatibleOfficeHolding);
        }

        for record in &self.accessions {
            require_subject(&subjects, &record.holder)?;
            require_evidence(&evidence, &record.evidence)?;
            require_subject_seal(&seals, &record.seal, &record.holder)?;
            let title = titles.get(&record.title).ok_or_else(|| {
                StonebendValidationError::MissingRecord("Title", record.title.to_string())
            })?;
            if title.subject != record.holder || title.office != Some(record.office) {
                return Err(StonebendValidationError::AccessionMismatch(
                    record.id.clone(),
                ));
            }
            match (&record.office, &record.basis) {
                (
                    ConstitutionalOffice::Hypergiant,
                    AccessionBasis::Hypergiant {
                        lawful_vacancy: true,
                        stable_claim_presented: true,
                        freemason_independently_examined: true,
                        proliteriate_yield_hearing_completed: true,
                        protected_elevation_relinquished: true,
                        consequence_descent_completed: true,
                        proof_of_persistence_completed: true,
                        lazerhorn_climbed: true,
                        public_oath_recorded: true,
                    },
                )
                | (
                    ConstitutionalOffice::HighFreemason,
                    AccessionBasis::HighFreemason {
                        institution_nominated: true,
                        proliteriate_reviewed: true,
                        hypergiant_confirmed: true,
                    },
                ) => {}
                _ => {
                    return Err(StonebendValidationError::InvalidAccession(
                        record.id.clone(),
                    ));
                }
            }
        }

        self.validate_hollowing(&subjects, &evidence, &decisions, &seals)?;
        self.validate_renaming_and_succession(&subjects, &names, &evidence, &seals)?;
        self.validate_tombstones(&subjects, &names, &titles, &evidence, &seals)?;
        Ok(())
    }

    fn validate_hollowing(
        &self,
        subjects: &BTreeMap<&IdentityId, &SubjectRecord>,
        evidence: &BTreeSet<&EvidenceRecordId>,
        decisions: &BTreeMap<&DecisionRecordId, &DecisionRecord>,
        seals: &BTreeMap<&SealRecordId, &SealRecord>,
    ) -> Result<(), StonebendValidationError> {
        let hollowings = self
            .hollowings
            .iter()
            .map(|record| (&record.id, record))
            .collect::<BTreeMap<_, _>>();
        let custody = self
            .custody
            .iter()
            .map(|record| (&record.id, record))
            .collect::<BTreeMap<_, _>>();
        let extracted = self
            .extracted_hollow
            .iter()
            .map(|record| (&record.id, record))
            .collect::<BTreeMap<_, _>>();

        for record in &self.custody {
            require_subject(subjects, &record.custodian)?;
            require_subject(subjects, &record.subject)?;
            require_evidence(evidence, &record.chain_of_custody)?;
        }
        for record in &self.extracted_hollow {
            require_subject(subjects, &record.source_subject)?;
            if !hollowings.contains_key(&record.source_hollowing) {
                return Err(StonebendValidationError::MissingRecord(
                    "source Hollowing",
                    record.source_hollowing.to_string(),
                ));
            }
            require_evidence(evidence, &record.provenance)?;
            if !custody.contains_key(&record.custody) {
                return Err(StonebendValidationError::MissingRecord(
                    "custody",
                    record.custody.to_string(),
                ));
            }
        }
        for record in &self.hollowings {
            require_subject(subjects, &record.subject)?;
            require_subject(subjects, &record.qualified_operator)?;
            if !decisions.contains_key(&record.authority) {
                return Err(StonebendValidationError::MissingRecord(
                    "Hollowing authority",
                    record.authority.to_string(),
                ));
            }
            require_text(&record.scope, "Hollowing scope")?;
            require_text(&record.safety_plan, "Hollowing safety plan")?;
            require_text(&record.custody_plan, "Hollowing custody plan")?;
            require_text(&record.procedure_record, "Hollowing procedure record")?;
            require_text(
                &record.restoration_or_disposition_plan,
                "Hollowing restoration or disposition plan",
            )?;
            require_evidence(evidence, &record.pre_procedure_evidence)?;
            require_evidence(evidence, &record.post_procedure_evidence)?;
            require_subject_seal(seals, &record.seal, &record.subject)?;
            if !record.identity_continuity_determined {
                return Err(StonebendValidationError::HollowingContinuityMissing(
                    record.id.clone(),
                ));
            }
            match &record.consent {
                HollowingConsent::NotApplicable { reason } => {
                    require_text(reason, "consent not-applicable reason")?;
                }
                HollowingConsent::Granted {
                    informed: true,
                    specific: true,
                    voluntary: true,
                    capacity_confirmed: true,
                    revocability_recorded: true,
                    evidence: consent_evidence,
                } => {
                    if !evidence.contains(consent_evidence) {
                        return Err(StonebendValidationError::MissingRecord(
                            "consent evidence",
                            consent_evidence.to_string(),
                        ));
                    }
                }
                HollowingConsent::Emergency {
                    immediate_harm_recorded: true,
                    limited_to_necessity: true,
                    post_event_review: Some(review),
                } => {
                    if !decisions.contains_key(review) {
                        return Err(StonebendValidationError::MissingRecord(
                            "emergency post-event review",
                            review.to_string(),
                        ));
                    }
                }
                HollowingConsent::Emergency { .. } => {
                    return Err(StonebendValidationError::EmergencyHollowingWithoutReview(
                        record.id.clone(),
                    ));
                }
                HollowingConsent::Granted { .. } => {
                    return Err(StonebendValidationError::InvalidConsent(record.id.clone()));
                }
            }
            for extracted_id in &record.extracted_hollow {
                let extracted_record = extracted.get(extracted_id).ok_or_else(|| {
                    StonebendValidationError::MissingRecord(
                        "extracted Hollow",
                        extracted_id.to_string(),
                    )
                })?;
                if extracted_record.source_subject != record.subject
                    || extracted_record.source_hollowing != record.id
                {
                    return Err(StonebendValidationError::ExtractedHollowSourceMismatch(
                        extracted_id.clone(),
                    ));
                }
            }
        }
        Ok(())
    }

    fn validate_renaming_and_succession(
        &self,
        subjects: &BTreeMap<&IdentityId, &SubjectRecord>,
        names: &BTreeMap<&NameRecordId, &NameRecord>,
        evidence: &BTreeSet<&EvidenceRecordId>,
        seals: &BTreeMap<&SealRecordId, &SealRecord>,
    ) -> Result<(), StonebendValidationError> {
        for record in &self.renames {
            require_subject(subjects, &record.subject)?;
            let former = names.get(&record.former_name).ok_or_else(|| {
                StonebendValidationError::MissingRecord(
                    "former Name",
                    record.former_name.to_string(),
                )
            })?;
            let new = names.get(&record.new_name).ok_or_else(|| {
                StonebendValidationError::MissingRecord("new Name", record.new_name.to_string())
            })?;
            if former.subject != record.subject
                || new.subject != record.subject
                || !new.former_names.contains(&record.former_name)
                || !record.continuity_preserved
                || !record.history_preserved
            {
                return Err(StonebendValidationError::RenamingErasesHistory(
                    record.id.clone(),
                ));
            }
            require_text(&record.reason, "rename reason")?;
            require_subject_seal(seals, &record.seal, &record.subject)?;
        }
        for record in &self.successions {
            require_subject(subjects, &record.predecessor)?;
            require_subject(subjects, &record.successor)?;
            require_evidence(evidence, &record.evidence)?;
            require_subject_seal(seals, &record.seal, &record.successor)?;
            let predecessor = record
                .predecessor_obligations
                .iter()
                .collect::<BTreeSet<_>>();
            let successor = record.successor_obligations.iter().collect::<BTreeSet<_>>();
            let predecessor_benefits = record.predecessor_benefits.iter().collect::<BTreeSet<_>>();
            let successor_benefits = record.successor_benefits.iter().collect::<BTreeSet<_>>();
            if !record.continuity_proven
                || !record.differences_recorded
                || !predecessor_benefits.is_subset(&successor_benefits)
                || !predecessor.is_subset(&successor)
            {
                return Err(StonebendValidationError::SuccessionDropsObligations(
                    record.id.clone(),
                ));
            }
        }
        Ok(())
    }

    fn validate_tombstones(
        &self,
        subjects: &BTreeMap<&IdentityId, &SubjectRecord>,
        names: &BTreeMap<&NameRecordId, &NameRecord>,
        titles: &BTreeMap<&TitleRecordId, &TitleRecord>,
        evidence: &BTreeSet<&EvidenceRecordId>,
        seals: &BTreeMap<&SealRecordId, &SealRecord>,
    ) -> Result<(), StonebendValidationError> {
        for record in &self.tombstones {
            let target_subject = match &record.target {
                TombstoneTarget::Name(id) => {
                    let name = names.get(id).ok_or_else(|| {
                        StonebendValidationError::MissingRecord("Name", id.to_string())
                    })?;
                    if name.status != NameStatus::Tombstoned {
                        return Err(StonebendValidationError::ActiveTombstoneTarget(
                            record.id.clone(),
                        ));
                    }
                    name.subject.clone()
                }
                TombstoneTarget::Title(id) => {
                    let title = titles.get(id).ok_or_else(|| {
                        StonebendValidationError::MissingRecord("Title", id.to_string())
                    })?;
                    if title.status != TitleStatus::Tombstoned {
                        return Err(StonebendValidationError::ActiveTombstoneTarget(
                            record.id.clone(),
                        ));
                    }
                    title.subject.clone()
                }
                TombstoneTarget::Subject(id) => {
                    require_subject(subjects, id)?;
                    id.clone()
                }
            };
            require_text(&record.former_name, "Tombstone former Name")?;
            require_text(&record.reason, "Tombstone reason")?;
            require_text(&record.access_restrictions, "Tombstone access restrictions")?;
            require_evidence(evidence, &record.preserved_evidence)?;
            require_subject_seal(seals, &record.seal, &target_subject)?;
            for title in &record.former_titles {
                if !titles.contains_key(title) {
                    return Err(StonebendValidationError::MissingRecord(
                        "former Title",
                        title.to_string(),
                    ));
                }
            }
            if let Some(successor) = &record.successor {
                require_subject(subjects, successor)?;
            }
        }
        for name in self
            .names
            .iter()
            .filter(|record| record.status == NameStatus::Tombstoned)
        {
            if !self
                .tombstones
                .iter()
                .any(|record| matches!(&record.target, TombstoneTarget::Name(id) if id == &name.id))
            {
                return Err(StonebendValidationError::MissingTombstone(
                    name.id.to_string(),
                ));
            }
        }
        for title in self
            .titles
            .iter()
            .filter(|record| record.status == TitleStatus::Tombstoned)
        {
            if !self.tombstones.iter().any(
                |record| matches!(&record.target, TombstoneTarget::Title(id) if id == &title.id),
            ) {
                return Err(StonebendValidationError::MissingTombstone(
                    title.id.to_string(),
                ));
            }
        }
        Ok(())
    }

    pub fn audit(&self) -> Result<StonebendConstitutionalAudit, StonebendValidationError> {
        self.validate()?;
        Ok(StonebendConstitutionalAudit {
            active_hypergiants: self
                .titles
                .iter()
                .filter(|title| {
                    title.status == TitleStatus::Active
                        && title.office == Some(ConstitutionalOffice::Hypergiant)
                })
                .count(),
            active_high_freemasons: self
                .titles
                .iter()
                .filter(|title| {
                    title.status == TitleStatus::Active
                        && title.office == Some(ConstitutionalOffice::HighFreemason)
                })
                .count(),
            named_subjects: self.names.len(),
            active_titles: self
                .titles
                .iter()
                .filter(|title| title.status == TitleStatus::Active)
                .count(),
            sealed_decisions: self.seals.len(),
            lawful_hollowings: self.hollowings.len(),
            protected_extractions: self.extracted_hollow.len(),
            tombstones: self.tombstones.len(),
            exact_principal_authority_roster: validate_principal_authorities().is_ok(),
            transformation_never_grants_title: true,
            recognition_never_grants_title: true,
            clearance_never_replaces_consent: true,
            legacy_progression_never_grants_authority: true,
        })
    }
}

fn require_subject(
    subjects: &BTreeMap<&IdentityId, &SubjectRecord>,
    id: &IdentityId,
) -> Result<(), StonebendValidationError> {
    if subjects.contains_key(id) {
        Ok(())
    } else {
        Err(StonebendValidationError::MissingSubject(id.clone()))
    }
}

fn require_evidence(
    known: &BTreeSet<&EvidenceRecordId>,
    supplied: &[EvidenceRecordId],
) -> Result<(), StonebendValidationError> {
    if supplied.is_empty() {
        return Err(StonebendValidationError::MissingEvidence);
    }
    if let Some(missing) = supplied.iter().find(|id| !known.contains(id)) {
        return Err(StonebendValidationError::MissingRecord(
            "evidence",
            missing.to_string(),
        ));
    }
    Ok(())
}

fn require_seal(
    seals: &BTreeMap<&SealRecordId, &SealRecord>,
    id: &SealRecordId,
) -> Result<(), StonebendValidationError> {
    if seals.contains_key(id) {
        Ok(())
    } else {
        Err(StonebendValidationError::MissingRecord(
            "Seal",
            id.to_string(),
        ))
    }
}

fn require_subject_seal(
    seals: &BTreeMap<&SealRecordId, &SealRecord>,
    id: &SealRecordId,
    subject: &IdentityId,
) -> Result<(), StonebendValidationError> {
    require_seal(seals, id)?;
    if seals.get(id).is_some_and(|seal| &seal.subject == subject) {
        Ok(())
    } else {
        Err(StonebendValidationError::SealSubjectMismatch(id.clone()))
    }
}

fn require_text(value: &str, field: &'static str) -> Result<(), StonebendValidationError> {
    if value.trim().is_empty() {
        Err(StonebendValidationError::MissingText(field))
    } else {
        Ok(())
    }
}

fn validate_unique<'a, T>(
    ids: impl Iterator<Item = &'a T>,
    kind: &'static str,
) -> Result<(), StonebendValidationError>
where
    T: Ord + fmt::Display + 'a,
{
    let mut seen = BTreeSet::new();
    for id in ids {
        if !seen.insert(id) {
            return Err(StonebendValidationError::DuplicateRecord(
                kind,
                id.to_string(),
            ));
        }
    }
    Ok(())
}

pub fn validate_principal_authorities() -> Result<(), StonebendValidationError> {
    if PRINCIPAL_AUTHORITIES
        .iter()
        .filter(|definition| definition.placement == ConstitutionalPlacement::SingularHighestOffice)
        .count()
        != 1
        || PRINCIPAL_AUTHORITIES
            .iter()
            .find(|definition| {
                definition.placement == ConstitutionalPlacement::SingularHighestOffice
            })
            .map(|definition| definition.authority)
            != Some(PrincipalAuthority::Hypergiant)
    {
        return Err(StonebendValidationError::InvalidHighestAuthority);
    }
    let powers = PRINCIPAL_AUTHORITIES
        .iter()
        .map(|definition| definition.authority.constitutional_power())
        .collect::<BTreeSet<_>>();
    if powers
        != BTreeSet::from([
            ConstitutionalPower::Claim,
            ConstitutionalPower::Title,
            ConstitutionalPower::Yield,
        ])
    {
        return Err(StonebendValidationError::InvalidPowerSeparation);
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StonebendConstitutionalAudit {
    pub active_hypergiants: usize,
    pub active_high_freemasons: usize,
    pub named_subjects: usize,
    pub active_titles: usize,
    pub sealed_decisions: usize,
    pub lawful_hollowings: usize,
    pub protected_extractions: usize,
    pub tombstones: usize,
    pub exact_principal_authority_roster: bool,
    pub transformation_never_grants_title: bool,
    pub recognition_never_grants_title: bool,
    pub clearance_never_replaces_consent: bool,
    pub legacy_progression_never_grants_authority: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StonebendValidationError {
    DuplicateRecord(&'static str, String),
    MissingRecord(&'static str, String),
    MissingSubject(IdentityId),
    UnstableSubject(IdentityId),
    MissingText(&'static str),
    MissingEvidence,
    InvalidHighestAuthority,
    InvalidPowerSeparation,
    SealMismatch(SealRecordId),
    SealSubjectMismatch(SealRecordId),
    InvalidSealIssuer(SealRecordId),
    TitleNameMismatch(TitleRecordId),
    EmergencyTitleWithoutExpiration(TitleRecordId),
    ContradictoryExclusiveIdentity {
        scope: String,
        name: String,
    },
    UnlawfulTitleOrigin(TitleRecordId),
    OfficeTitleWithoutOffice(TitleRecordId),
    ActiveOfficeWithoutAccession(TitleRecordId),
    AccessionMismatch(AccessionRecordId),
    InvalidAccession(AccessionRecordId),
    ActiveHypergiantCount(usize),
    ActiveHighFreemasonCount(usize),
    IncompatibleOfficeHolding,
    InvalidConsent(HollowingRecordId),
    EmergencyHollowingWithoutReview(HollowingRecordId),
    HollowingContinuityMissing(HollowingRecordId),
    ExtractedHollowSourceMismatch(ExtractedHollowRecordId),
    RenamingErasesHistory(RenameRecordId),
    SuccessionDropsObligations(SuccessionRecordId),
    ActiveTombstoneTarget(TombstoneRecordId),
    MissingTombstone(String),
    InvalidSelectionTransition {
        from: HypergiantSelectionPhase,
        expected: HypergiantSelectionPhase,
        attempted: HypergiantSelectionPhase,
    },
    CompletedSelectionCannotAdvance,
    IncompleteSelection(HypergiantSelectionPhase),
}

impl fmt::Display for StonebendValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "Stonebend constitutional validation failed: {self:?}"
        )
    }
}

impl std::error::Error for StonebendValidationError {}

#[must_use]
pub fn build_stonebend_constitutional_audit(audit: &StonebendConstitutionalAudit) -> String {
    format!(
        "Stonebend Constitution: pass\n\
         governing verb: {}\n\
         signature offense: {}\n\
         active Hypergiants: {}\n\
         active High Freemasons: {}\n\
         named subjects: {}\n\
         active Titles: {}\n\
         sealed decisions: {}\n\
         lawful Hollowings: {}\n\
         protected extractions: {}\n\
         Tombstones: {}\n\
         principal authority roster exact: {}\n\
         transformation grants office or Title: false\n\
         recognition substitutes for Title: false\n\
         clearance substitutes for consent: false\n\
         legacy progression grants authority: false\n\
         source: {}\n",
        STONEBEND_GOVERNING_VERB,
        STONEBEND_SIGNATURE_OFFENSE,
        audit.active_hypergiants,
        audit.active_high_freemasons,
        audit.named_subjects,
        audit.active_titles,
        audit.sealed_decisions,
        audit.lawful_hollowings,
        audit.protected_extractions,
        audit.tombstones,
        audit.exact_principal_authority_roster,
        STONEBEND_CONSTITUTION_SOURCE,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record_id<T>(value: &str, make: impl FnOnce(String) -> Result<T, StonebendIdError>) -> T {
        make(value.into()).unwrap()
    }

    fn minimal_registry(origin: TitleOrigin) -> StonebendRegistry {
        let holder = identity("being.stonebend.fixture-hypergiant");
        let operator = identity("being.stonebend.fixture-freemason");
        let subject_name = record_id("name.fixture.hypergiant", NameRecordId::new);
        let evidence_id = record_id("evidence.fixture.identity", EvidenceRecordId::new);
        let decision_id = record_id("decision.fixture.accession", DecisionRecordId::new);
        let seal_id = record_id("seal.fixture.accession", SealRecordId::new);
        let title_id = record_id("title.fixture.hypergiant", TitleRecordId::new);
        let accession_id = record_id("accession.fixture.hypergiant", AccessionRecordId::new);
        StonebendRegistry {
            subjects: vec![
                SubjectRecord {
                    id: holder.clone(),
                    kind: SubjectKind::Being,
                    continuity_established: true,
                },
                SubjectRecord {
                    id: operator,
                    kind: SubjectKind::Being,
                    continuity_established: true,
                },
            ],
            names: vec![NameRecord {
                id: subject_name.clone(),
                subject: holder.clone(),
                name: "Fixture Hypergiant holder".into(),
                class: NameClass::Personal,
                scope: "Stonebend fixture".into(),
                exclusive: true,
                status: NameStatus::Active,
                evidence: vec![evidence_id.clone()],
                former_names: vec![],
            }],
            titles: vec![TitleRecord {
                id: title_id.clone(),
                subject: holder.clone(),
                name: subject_name,
                class: TitleClass::Office,
                legal_basis: "lawful Hypergiant selection".into(),
                status: TitleStatus::Active,
                expires_at: None,
                origin,
                office: Some(ConstitutionalOffice::Hypergiant),
                accession: Some(accession_id.clone()),
                evidence: vec![evidence_id.clone()],
                seal: seal_id.clone(),
            }],
            evidence: vec![EvidenceRecord {
                id: evidence_id.clone(),
                subject: holder.clone(),
                description: "independent identity and accession record".into(),
                provenance: "fixture constitutional hearing".into(),
            }],
            decisions: vec![DecisionRecord {
                id: decision_id.clone(),
                authority: PrincipalAuthority::Proliteriate,
                subject: holder.clone(),
                scope: "Hypergiant accession".into(),
                evidence: vec![evidence_id.clone()],
            }],
            accessions: vec![AccessionRecord {
                id: accession_id,
                title: title_id,
                holder: holder.clone(),
                office: ConstitutionalOffice::Hypergiant,
                basis: AccessionBasis::Hypergiant {
                    lawful_vacancy: true,
                    stable_claim_presented: true,
                    freemason_independently_examined: true,
                    proliteriate_yield_hearing_completed: true,
                    protected_elevation_relinquished: true,
                    consequence_descent_completed: true,
                    proof_of_persistence_completed: true,
                    lazerhorn_climbed: true,
                    public_oath_recorded: true,
                },
                evidence: vec![evidence_id],
                seal: seal_id.clone(),
            }],
            seals: vec![SealRecord {
                id: seal_id,
                issuing_authority: PrincipalAuthority::FreemasonInstitution,
                subject: holder,
                scope: "Hypergiant accession".into(),
                decision: decision_id,
            }],
            ..Default::default()
        }
    }

    #[test]
    fn principal_authorities_preserve_claim_title_and_yield() {
        validate_principal_authorities().unwrap();
        assert_eq!(
            PRINCIPAL_AUTHORITIES
                .iter()
                .filter(|definition| {
                    definition.placement == ConstitutionalPlacement::SingularHighestOffice
                })
                .count(),
            1
        );
    }

    #[test]
    fn lawful_minimal_registry_validates() {
        let registry = minimal_registry(TitleOrigin::LawfulGrant);
        let audit = registry.audit().unwrap();
        assert_eq!(audit.active_hypergiants, 1);
        assert!(build_stonebend_constitutional_audit(&audit).contains("Illegal Hollowing"));
    }

    #[test]
    fn transformation_cannot_manufacture_hypergiant_title() {
        assert!(matches!(
            minimal_registry(TitleOrigin::Transformation).validate(),
            Err(StonebendValidationError::UnlawfulTitleOrigin(_))
        ));
    }

    #[test]
    fn diamond_vacancy_allows_zero_but_never_two_active_hypergiants() {
        let mut registry = minimal_registry(TitleOrigin::LawfulGrant);
        registry.titles.clear();
        registry.accessions.clear();
        registry.validate().expect("Diamond may be honestly vacant");

        let duplicate = minimal_registry(TitleOrigin::LawfulGrant);
        registry.subjects = duplicate.subjects;
        registry.names = duplicate.names;
        registry.titles = duplicate.titles;
        registry.evidence = duplicate.evidence;
        registry.decisions = duplicate.decisions;
        registry.accessions = duplicate.accessions;
        registry.seals = duplicate.seals;
        let mut second = minimal_registry(TitleOrigin::LawfulGrant);
        second.subjects[0].id = identity("being.stonebend.fixture-hypergiant-two");
        second.names[0].id = record_id("name.fixture.hypergiant-two", NameRecordId::new);
        second.names[0].subject = second.subjects[0].id.clone();
        second.names[0].name = "Fixture Hypergiant holder two".into();
        second.titles[0].id = record_id("title.fixture.hypergiant-two", TitleRecordId::new);
        second.titles[0].subject = second.subjects[0].id.clone();
        second.titles[0].name = second.names[0].id.clone();
        second.evidence[0].id = record_id("evidence.fixture.hypergiant-two", EvidenceRecordId::new);
        second.evidence[0].subject = second.subjects[0].id.clone();
        second.decisions[0].id =
            record_id("decision.fixture.hypergiant-two", DecisionRecordId::new);
        second.decisions[0].subject = second.subjects[0].id.clone();
        second.decisions[0].evidence = vec![second.evidence[0].id.clone()];
        second.seals[0].id = record_id("seal.fixture.hypergiant-two", SealRecordId::new);
        second.seals[0].subject = second.subjects[0].id.clone();
        second.seals[0].decision = second.decisions[0].id.clone();
        second.accessions[0].id =
            record_id("accession.fixture.hypergiant-two", AccessionRecordId::new);
        second.accessions[0].title = second.titles[0].id.clone();
        second.accessions[0].holder = second.subjects[0].id.clone();
        second.accessions[0].evidence = vec![second.evidence[0].id.clone()];
        second.accessions[0].seal = second.seals[0].id.clone();
        second.titles[0].accession = Some(second.accessions[0].id.clone());
        second.titles[0].evidence = vec![second.evidence[0].id.clone()];
        second.titles[0].seal = second.seals[0].id.clone();
        registry.subjects.push(second.subjects.remove(0));
        registry.names.push(second.names.remove(0));
        registry.titles.push(second.titles.remove(0));
        registry.evidence.push(second.evidence.remove(0));
        registry.decisions.push(second.decisions.remove(0));
        registry.accessions.push(second.accessions.remove(0));
        registry.seals.push(second.seals.remove(0));
        assert_eq!(
            registry.validate(),
            Err(StonebendValidationError::ActiveHypergiantCount(2))
        );
    }

    #[test]
    fn emergency_hollowing_requires_post_event_review() {
        let mut registry = minimal_registry(TitleOrigin::LawfulGrant);
        let holder = registry.subjects[0].id.clone();
        let operator = registry.subjects[1].id.clone();
        let evidence = registry.evidence[0].id.clone();
        let decision = registry.decisions[0].id.clone();
        let seal = registry.seals[0].id.clone();
        registry.hollowings.push(HollowingRecord {
            id: record_id("hollowing.fixture.emergency", HollowingRecordId::new),
            subject: holder,
            authority: decision,
            purpose: HollowingPurpose::EmergencyStabilization,
            scope: "limited stabilization".into(),
            consent: HollowingConsent::Emergency {
                immediate_harm_recorded: true,
                limited_to_necessity: true,
                post_event_review: None,
            },
            pre_procedure_evidence: vec![evidence.clone()],
            qualified_operator: operator,
            safety_plan: "stabilize only".into(),
            custody_plan: "protect extraction".into(),
            procedure_record: "emergency opening".into(),
            extracted_hollow: vec![],
            post_procedure_evidence: vec![evidence],
            identity_continuity_determined: true,
            restoration_or_disposition_plan: "restore opening".into(),
            seal,
        });
        assert!(matches!(
            registry.validate(),
            Err(StonebendValidationError::EmergencyHollowingWithoutReview(_))
        ));
    }

    #[test]
    fn hypergiant_selection_follows_the_ratified_order() {
        let candidate = identity("being.stonebend.fixture-candidate");
        let evidence = record_id("evidence.fixture.selection", EvidenceRecordId::new);
        let mut process = HypergiantSelectionProcess::open(candidate);
        for phase in [
            HypergiantSelectionPhase::FreemasonExamination,
            HypergiantSelectionPhase::ProliteriateYieldHearing,
            HypergiantSelectionPhase::ProtectedElevationRelinquished,
            HypergiantSelectionPhase::ConsequenceDescentCompleted,
            HypergiantSelectionPhase::FlyntProofOfPersistence,
            HypergiantSelectionPhase::LazerhornClimbed,
            HypergiantSelectionPhase::AccessionEligible,
            HypergiantSelectionPhase::DiamondInvested,
        ] {
            process.advance(phase, evidence.clone()).unwrap();
        }
        process.require_complete().unwrap();
    }

    #[test]
    fn hypergiant_selection_cannot_skip_public_process() {
        let candidate = identity("being.stonebend.fixture-candidate");
        let evidence = record_id("evidence.fixture.selection", EvidenceRecordId::new);
        let mut process = HypergiantSelectionProcess::open(candidate);
        assert!(matches!(
            process.advance(HypergiantSelectionPhase::ProliteriateYieldHearing, evidence),
            Err(StonebendValidationError::InvalidSelectionTransition { .. })
        ));
    }
}
