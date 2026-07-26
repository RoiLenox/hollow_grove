//! Typed Hueman faculty law above the frozen recursion kernel and common
//! constitutional runtime.
//!
//! Faculties describe how a Recipe perceives, relates, anticipates, generates,
//! or provisionally embodies action. They do not select a candidate, execute a
//! Recipe, grant proof, mutate a Frame, or create House authority.

use serde::{Deserialize, Serialize};

use crate::constitutional::{BondPhase, ConstitutionalPolarity, EvidenceRef};
use crate::institution::{GroupId, InstitutionId};
use crate::institution_affiliation::InstitutionalMembership;
use crate::lineage_contract::SandmanorForm;
use crate::{FlowId, FrameId, GlowId};

use super::flynt;

pub const HUEMAN_FACULTIES_SOURCE: &str = "HUEMAN_FACULTIES_V1.md";
pub const FACULTY_ARCHIVE_FORMAT: &str = "HGFAC";
pub const FACULTY_ARCHIVE_SCHEMA_VERSION: u16 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HuemanFaculty {
    Presynce,
    Resynce,
    Precog,
    Prefog,
    Prefig,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HuemanFacultyDomain {
    Body,
    Spirit,
    Mind,
    SoulInterior,
    SoulExterior,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FacultyAuthority {
    Stonebend,
    Flynt,
    Glaushouse,
    SandmanorMinorian,
    SandmanorMinoan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FacultyDefinition {
    pub faculty: HuemanFaculty,
    pub domain: HuemanFacultyDomain,
    pub authority: FacultyAuthority,
    pub canonical_phrase: &'static str,
}

pub const FACULTY_DEFINITIONS: [FacultyDefinition; 5] = [
    FacultyDefinition {
        faculty: HuemanFaculty::Presynce,
        domain: HuemanFacultyDomain::Body,
        authority: FacultyAuthority::Stonebend,
        canonical_phrase: "Presynce feels emergence.",
    },
    FacultyDefinition {
        faculty: HuemanFaculty::Resynce,
        domain: HuemanFacultyDomain::Spirit,
        authority: FacultyAuthority::Flynt,
        canonical_phrase: "Resynce joins relation.",
    },
    FacultyDefinition {
        faculty: HuemanFaculty::Precog,
        domain: HuemanFacultyDomain::Mind,
        authority: FacultyAuthority::Glaushouse,
        canonical_phrase: "Precog foresees consequence.",
    },
    FacultyDefinition {
        faculty: HuemanFaculty::Prefog,
        domain: HuemanFacultyDomain::SoulInterior,
        authority: FacultyAuthority::SandmanorMinorian,
        canonical_phrase: "Prefog opens possibility.",
    },
    FacultyDefinition {
        faculty: HuemanFaculty::Prefig,
        domain: HuemanFacultyDomain::SoulExterior,
        authority: FacultyAuthority::SandmanorMinoan,
        canonical_phrase: "Prefig forms becoming.",
    },
];

impl HuemanFaculty {
    #[must_use]
    pub const fn definition(self) -> FacultyDefinition {
        match self {
            Self::Presynce => FACULTY_DEFINITIONS[0],
            Self::Resynce => FACULTY_DEFINITIONS[1],
            Self::Precog => FACULTY_DEFINITIONS[2],
            Self::Prefog => FACULTY_DEFINITIONS[3],
            Self::Prefig => FACULTY_DEFINITIONS[4],
        }
    }

    #[must_use]
    pub const fn decision_posture(self) -> FacultyDecisionPosture {
        match self {
            Self::Presynce | Self::Resynce => FacultyDecisionPosture::Observe,
            Self::Prefog => FacultyDecisionPosture::Generate,
            Self::Precog => FacultyDecisionPosture::Evaluate,
            Self::Prefig => FacultyDecisionPosture::ExecuteOrDemonstrate,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FacultyDecisionPosture {
    Observe,
    Generate,
    Evaluate,
    ExecuteOrDemonstrate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FacultyTrigger {
    PhysicalEmergence,
    RelationalEmergence,
    EvidencePattern,
    OpenPossibility,
    SelectedLegalCandidate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PrefigEmbodimentStatus {
    Provisional,
    Demonstrable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FacultyExpression {
    EmbodiedAnticipation,
    RelationalSynchronization,
    ProbableContinuation,
    CandidatePossibilities { legal_candidate_count: u16 },
    ProvisionalEmbodiment { status: PrefigEmbodimentStatus },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FacultyUncertainty {
    pub basis: String,
    pub confidence_basis_points: u16,
    pub guaranteed: bool,
    pub alternatives_preserved: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FacultyRegion {
    AuraFields,
    AuraBeachAndCurrentSea,
    AuraRidge,
    FlyntCivic,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FacultyBounds {
    pub frame: FrameId,
    pub required_flow: Option<FlowId>,
    pub required_glow: Option<GlowId>,
    pub allowed_polarities: Vec<ConstitutionalPolarity>,
    pub bond_phase: Option<BondPhase>,
    pub regional_jurisdiction: Option<FacultyRegion>,
    pub institutional_recognition: Option<InstitutionId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FacultyManifestation {
    pub faculty: HuemanFaculty,
    pub domain: HuemanFacultyDomain,
    pub authority: FacultyAuthority,
    pub trigger: FacultyTrigger,
    pub expression: FacultyExpression,
    pub uncertainty: FacultyUncertainty,
    pub evidence_requirements: Vec<EvidenceRef>,
    pub bounds: FacultyBounds,
}

impl FacultyManifestation {
    #[must_use]
    pub const fn decision_posture(&self) -> FacultyDecisionPosture {
        self.faculty.decision_posture()
    }

    #[must_use]
    pub const fn can_create_proof(&self) -> bool {
        false
    }

    #[must_use]
    pub const fn independently_executes_transformation(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FacultyLawError {
    OwnershipMismatch(HuemanFaculty),
    TriggerMismatch(HuemanFaculty),
    ExpressionMismatch(HuemanFaculty),
    MissingEvidence(HuemanFaculty),
    MissingPolarityBound(HuemanFaculty),
    InvalidUncertainty(HuemanFaculty),
    PrecogWithoutGlow,
    ResynceWithoutRelation,
    PrefogWithoutMultiplePossibilities,
    PrefogClosedAlternatives,
    InvalidSoulCycle,
    SoulHalvesUnequal,
    InvalidRegionalSoulManifestation,
    InvalidResynceCulture,
    NotResynce,
}

impl std::fmt::Display for FacultyLawError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "Hueman faculty law rejected state: {self:?}")
    }
}

impl std::error::Error for FacultyLawError {}

pub fn validate_faculty_manifestation(
    manifestation: &FacultyManifestation,
) -> Result<(), FacultyLawError> {
    let expected = manifestation.faculty.definition();
    if manifestation.domain != expected.domain || manifestation.authority != expected.authority {
        return Err(FacultyLawError::OwnershipMismatch(manifestation.faculty));
    }
    if manifestation.trigger != expected_trigger(manifestation.faculty) {
        return Err(FacultyLawError::TriggerMismatch(manifestation.faculty));
    }
    if !expression_matches(manifestation.faculty, manifestation.expression) {
        return Err(FacultyLawError::ExpressionMismatch(manifestation.faculty));
    }
    if manifestation.evidence_requirements.is_empty() {
        return Err(FacultyLawError::MissingEvidence(manifestation.faculty));
    }
    if manifestation.bounds.allowed_polarities.is_empty() {
        return Err(FacultyLawError::MissingPolarityBound(manifestation.faculty));
    }
    if manifestation.uncertainty.basis.trim().is_empty()
        || manifestation.uncertainty.confidence_basis_points > 10_000
        || manifestation.uncertainty.guaranteed
    {
        return Err(FacultyLawError::InvalidUncertainty(manifestation.faculty));
    }
    match manifestation.faculty {
        HuemanFaculty::Precog if manifestation.bounds.required_glow.is_none() => {
            return Err(FacultyLawError::PrecogWithoutGlow);
        }
        HuemanFaculty::Resynce
            if manifestation.bounds.bond_phase.is_none()
                && manifestation.bounds.regional_jurisdiction.is_none()
                && manifestation.bounds.institutional_recognition.is_none() =>
        {
            return Err(FacultyLawError::ResynceWithoutRelation);
        }
        HuemanFaculty::Prefog => {
            let FacultyExpression::CandidatePossibilities {
                legal_candidate_count,
            } = manifestation.expression
            else {
                return Err(FacultyLawError::ExpressionMismatch(HuemanFaculty::Prefog));
            };
            if legal_candidate_count < 2 {
                return Err(FacultyLawError::PrefogWithoutMultiplePossibilities);
            }
            if !manifestation.uncertainty.alternatives_preserved {
                return Err(FacultyLawError::PrefogClosedAlternatives);
            }
        }
        _ => {}
    }
    Ok(())
}

pub fn validate_faculty_manifestations(
    manifestations: &[FacultyManifestation],
) -> Result<(), FacultyLawError> {
    for manifestation in manifestations {
        validate_faculty_manifestation(manifestation)?;
    }
    Ok(())
}

const fn expected_trigger(faculty: HuemanFaculty) -> FacultyTrigger {
    match faculty {
        HuemanFaculty::Presynce => FacultyTrigger::PhysicalEmergence,
        HuemanFaculty::Resynce => FacultyTrigger::RelationalEmergence,
        HuemanFaculty::Precog => FacultyTrigger::EvidencePattern,
        HuemanFaculty::Prefog => FacultyTrigger::OpenPossibility,
        HuemanFaculty::Prefig => FacultyTrigger::SelectedLegalCandidate,
    }
}

const fn expression_matches(faculty: HuemanFaculty, expression: FacultyExpression) -> bool {
    matches!(
        (faculty, expression),
        (
            HuemanFaculty::Presynce,
            FacultyExpression::EmbodiedAnticipation
        ) | (
            HuemanFaculty::Resynce,
            FacultyExpression::RelationalSynchronization
        ) | (
            HuemanFaculty::Precog,
            FacultyExpression::ProbableContinuation
        ) | (
            HuemanFaculty::Prefog,
            FacultyExpression::CandidatePossibilities { .. }
        ) | (
            HuemanFaculty::Prefig,
            FacultyExpression::ProvisionalEmbodiment { .. }
        )
    )
}

pub const CURRENT_FORM_PRESYNCE_LADDER: [FrameId; 8] = [
    FrameId::Gremlin,
    FrameId::Goblin,
    FrameId::Ghoul,
    FrameId::Spectre,
    FrameId::Troll,
    FrameId::Ork,
    FrameId::Ogre,
    FrameId::Troglodyte,
];

pub fn resynce_preserves_current_form(
    frame: FrameId,
    manifestation: &FacultyManifestation,
) -> Result<FrameId, FacultyLawError> {
    validate_faculty_manifestation(manifestation)?;
    if manifestation.faculty != HuemanFaculty::Resynce {
        return Err(FacultyLawError::NotResynce);
    }
    Ok(frame)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SoulCycleOutcome {
    Evidence(Vec<EvidenceRef>),
    Failure(Vec<EvidenceRef>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SandmanorSoulCycle {
    pub prefog: FacultyManifestation,
    pub prefig: FacultyManifestation,
    pub outcome: SoulCycleOutcome,
    pub revision_evidence: Vec<EvidenceRef>,
    pub returns_to_prefog: bool,
}

pub fn validate_sandmanor_soul_cycle(cycle: &SandmanorSoulCycle) -> Result<(), FacultyLawError> {
    validate_faculty_manifestation(&cycle.prefog)?;
    validate_faculty_manifestation(&cycle.prefig)?;
    if cycle.prefog.faculty != HuemanFaculty::Prefog
        || cycle.prefog.authority != FacultyAuthority::SandmanorMinorian
        || cycle.prefig.faculty != HuemanFaculty::Prefig
        || cycle.prefig.authority != FacultyAuthority::SandmanorMinoan
        || cycle.revision_evidence.is_empty()
        || !cycle.returns_to_prefog
    {
        return Err(FacultyLawError::InvalidSoulCycle);
    }
    let outcome_evidence = match &cycle.outcome {
        SoulCycleOutcome::Evidence(evidence) | SoulCycleOutcome::Failure(evidence) => evidence,
    };
    if outcome_evidence.is_empty()
        || outcome_evidence
            .iter()
            .any(|evidence| !cycle.revision_evidence.contains(evidence))
    {
        return Err(FacultyLawError::InvalidSoulCycle);
    }
    Ok(())
}

#[must_use]
pub fn sandmanor_soul_halves_equal() -> bool {
    HuemanFaculty::Prefog.definition().domain == HuemanFacultyDomain::SoulInterior
        && HuemanFaculty::Prefig.definition().domain == HuemanFacultyDomain::SoulExterior
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MatureSoulExpression {
    CultivatedPrefog,
    EmbodiedPrefig,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RegionalSoulManifestation {
    pub form: SandmanorForm,
    pub faculty: HuemanFaculty,
    pub authority: FacultyAuthority,
    pub region: FacultyRegion,
    pub expression: MatureSoulExpression,
    pub replaces_people_or_authority: bool,
}

#[must_use]
pub const fn canonical_regional_soul_manifestations() -> [RegionalSoulManifestation; 2] {
    [
        RegionalSoulManifestation {
            form: SandmanorForm::Minotaur,
            faculty: HuemanFaculty::Prefog,
            authority: FacultyAuthority::SandmanorMinorian,
            region: FacultyRegion::AuraFields,
            expression: MatureSoulExpression::CultivatedPrefog,
            replaces_people_or_authority: false,
        },
        RegionalSoulManifestation {
            form: SandmanorForm::Centaur,
            faculty: HuemanFaculty::Prefig,
            authority: FacultyAuthority::SandmanorMinoan,
            region: FacultyRegion::AuraBeachAndCurrentSea,
            expression: MatureSoulExpression::EmbodiedPrefig,
            replaces_people_or_authority: false,
        },
    ]
}

pub fn validate_regional_soul_manifestation(
    manifestation: RegionalSoulManifestation,
) -> Result<(), FacultyLawError> {
    let valid = matches!(
        manifestation,
        RegionalSoulManifestation {
            form: SandmanorForm::Minotaur,
            faculty: HuemanFaculty::Prefog,
            authority: FacultyAuthority::SandmanorMinorian,
            region: FacultyRegion::AuraFields,
            expression: MatureSoulExpression::CultivatedPrefog,
            replaces_people_or_authority: false,
        } | RegionalSoulManifestation {
            form: SandmanorForm::Centaur,
            faculty: HuemanFaculty::Prefig,
            authority: FacultyAuthority::SandmanorMinoan,
            region: FacultyRegion::AuraBeachAndCurrentSea,
            expression: MatureSoulExpression::EmbodiedPrefig,
            replaces_people_or_authority: false,
        }
    );
    if valid {
        Ok(())
    } else {
        Err(FacultyLawError::InvalidRegionalSoulManifestation)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResynceCulture {
    WeFairyMenAuraRidge,
    GallowsFlyntCivicRecognition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResynceCultureFunction {
    MobileCommunalRelation,
    ConsequentialCivicRecognition,
}

pub const WE_FAIRY_MEN_FRONTIER_ROLES: [&str; 8] = [
    "caravaners",
    "scouts",
    "guides",
    "traders",
    "performers",
    "salvagers",
    "escorts",
    "pathfinders",
];

pub const GALLOWS_CIVIC_RECOGNITION_DOMAINS: [&str; 7] = [
    "recognized consequence",
    "reputation",
    "public challenge",
    "accountability",
    "honor",
    "disgrace",
    "judgment",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResynceCultureEntity {
    Group(GroupId),
    Institution(InstitutionId),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResynceCultureManifestation {
    pub culture: ResynceCulture,
    pub entity: ResynceCultureEntity,
    pub region: FacultyRegion,
    pub function: ResynceCultureFunction,
    pub formal_flynt_state_authority: bool,
}

#[must_use]
pub fn canonical_resynce_cultures() -> [ResynceCultureManifestation; 2] {
    [
        ResynceCultureManifestation {
            culture: ResynceCulture::WeFairyMenAuraRidge,
            entity: ResynceCultureEntity::Group(flynt::we_fairy_men_group_id()),
            region: FacultyRegion::AuraRidge,
            function: ResynceCultureFunction::MobileCommunalRelation,
            formal_flynt_state_authority: false,
        },
        ResynceCultureManifestation {
            culture: ResynceCulture::GallowsFlyntCivicRecognition,
            entity: ResynceCultureEntity::Institution(flynt::gallows_id()),
            region: FacultyRegion::FlyntCivic,
            function: ResynceCultureFunction::ConsequentialCivicRecognition,
            formal_flynt_state_authority: true,
        },
    ]
}

pub fn validate_resynce_cultures(
    cultures: &[ResynceCultureManifestation],
) -> Result<(), FacultyLawError> {
    let [frontier, civic] = canonical_resynce_cultures();
    if cultures.len() != 2
        || !cultures.contains(&frontier)
        || !cultures.contains(&civic)
        || frontier.entity == civic.entity
    {
        return Err(FacultyLawError::InvalidResynceCulture);
    }
    Ok(())
}

/// Legacy affiliation is retained as affiliation only. It cannot create a
/// faculty manifestation, mastery, proof, credential, or office.
#[must_use]
pub fn migrate_legacy_faculty_manifestations(
    _memberships: &[InstitutionalMembership],
) -> Vec<FacultyManifestation> {
    Vec::new()
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct FacultyArchive {
    format: String,
    schema_version: u16,
    manifestations: Vec<FacultyWireManifestation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct FacultyWireManifestation {
    faculty: HuemanFaculty,
    domain: HuemanFacultyDomain,
    authority: FacultyAuthority,
    trigger: FacultyTrigger,
    expression: FacultyExpression,
    uncertainty: FacultyUncertainty,
    evidence_requirements: Vec<FacultyWireEvidence>,
    bounds: FacultyWireBounds,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct FacultyWireEvidence {
    namespace: String,
    key: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct FacultyWireBounds {
    frame: String,
    required_flow: Option<String>,
    required_glow: Option<String>,
    allowed_polarities: Vec<String>,
    bond_phase: Option<String>,
    regional_jurisdiction: Option<FacultyRegion>,
    institutional_recognition: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FacultyCodecError {
    InvalidLaw(FacultyLawError),
    Json(String),
    UnsupportedSchema(u16),
    UnsupportedFormat(String),
    UnknownFrame(String),
    UnknownFlow(String),
    UnknownGlow(String),
    UnknownPolarity(String),
    UnknownBondPhase(String),
    InvalidEvidence(String),
    InvalidInstitution(String),
}

impl std::fmt::Display for FacultyCodecError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "Hueman faculty archive error: {self:?}")
    }
}

impl std::error::Error for FacultyCodecError {}

pub fn encode_faculty_manifestations(
    manifestations: &[FacultyManifestation],
) -> Result<Vec<u8>, FacultyCodecError> {
    validate_faculty_manifestations(manifestations).map_err(FacultyCodecError::InvalidLaw)?;
    let archive = FacultyArchive {
        format: FACULTY_ARCHIVE_FORMAT.into(),
        schema_version: FACULTY_ARCHIVE_SCHEMA_VERSION,
        manifestations: manifestations.iter().map(Into::into).collect(),
    };
    serde_json::to_vec(&archive).map_err(|error| FacultyCodecError::Json(error.to_string()))
}

pub fn decode_faculty_manifestations(
    bytes: &[u8],
) -> Result<Vec<FacultyManifestation>, FacultyCodecError> {
    let archive: FacultyArchive = serde_json::from_slice(bytes)
        .map_err(|error| FacultyCodecError::Json(error.to_string()))?;
    if archive.format != FACULTY_ARCHIVE_FORMAT {
        return Err(FacultyCodecError::UnsupportedFormat(archive.format));
    }
    if archive.schema_version != FACULTY_ARCHIVE_SCHEMA_VERSION {
        return Err(FacultyCodecError::UnsupportedSchema(archive.schema_version));
    }
    let manifestations = archive
        .manifestations
        .into_iter()
        .map(TryInto::try_into)
        .collect::<Result<Vec<_>, _>>()?;
    validate_faculty_manifestations(&manifestations).map_err(FacultyCodecError::InvalidLaw)?;
    Ok(manifestations)
}

pub fn replay_faculty_manifestations(
    manifestations: &[FacultyManifestation],
) -> Result<Vec<FacultyManifestation>, FacultyCodecError> {
    decode_faculty_manifestations(&encode_faculty_manifestations(manifestations)?)
}

impl From<&FacultyManifestation> for FacultyWireManifestation {
    fn from(value: &FacultyManifestation) -> Self {
        Self {
            faculty: value.faculty,
            domain: value.domain,
            authority: value.authority,
            trigger: value.trigger,
            expression: value.expression,
            uncertainty: value.uncertainty.clone(),
            evidence_requirements: value
                .evidence_requirements
                .iter()
                .map(|evidence| FacultyWireEvidence {
                    namespace: evidence.0.namespace.clone(),
                    key: evidence.0.key.clone(),
                })
                .collect(),
            bounds: FacultyWireBounds {
                frame: frame_name(value.bounds.frame).into(),
                required_flow: value.bounds.required_flow.map(flow_name).map(Into::into),
                required_glow: value.bounds.required_glow.map(glow_name).map(Into::into),
                allowed_polarities: value
                    .bounds
                    .allowed_polarities
                    .iter()
                    .copied()
                    .map(polarity_name)
                    .map(Into::into)
                    .collect(),
                bond_phase: value.bounds.bond_phase.map(bond_phase_name).map(Into::into),
                regional_jurisdiction: value.bounds.regional_jurisdiction,
                institutional_recognition: value
                    .bounds
                    .institutional_recognition
                    .as_ref()
                    .map(|institution| institution.as_str().to_string()),
            },
        }
    }
}

impl TryFrom<FacultyWireManifestation> for FacultyManifestation {
    type Error = FacultyCodecError;

    fn try_from(value: FacultyWireManifestation) -> Result<Self, Self::Error> {
        Ok(Self {
            faculty: value.faculty,
            domain: value.domain,
            authority: value.authority,
            trigger: value.trigger,
            expression: value.expression,
            uncertainty: value.uncertainty,
            evidence_requirements: value
                .evidence_requirements
                .into_iter()
                .map(|evidence| {
                    EvidenceRef::new(evidence.namespace, evidence.key)
                        .map_err(|error| FacultyCodecError::InvalidEvidence(error.to_string()))
                })
                .collect::<Result<Vec<_>, _>>()?,
            bounds: FacultyBounds {
                frame: parse_frame(&value.bounds.frame)?,
                required_flow: value
                    .bounds
                    .required_flow
                    .as_deref()
                    .map(parse_flow)
                    .transpose()?,
                required_glow: value
                    .bounds
                    .required_glow
                    .as_deref()
                    .map(parse_glow)
                    .transpose()?,
                allowed_polarities: value
                    .bounds
                    .allowed_polarities
                    .iter()
                    .map(|polarity| parse_polarity(polarity))
                    .collect::<Result<Vec<_>, _>>()?,
                bond_phase: value
                    .bounds
                    .bond_phase
                    .as_deref()
                    .map(parse_bond_phase)
                    .transpose()?,
                regional_jurisdiction: value.bounds.regional_jurisdiction,
                institutional_recognition: value
                    .bounds
                    .institutional_recognition
                    .map(|institution| {
                        InstitutionId::new(institution.clone())
                            .map_err(|_| FacultyCodecError::InvalidInstitution(institution))
                    })
                    .transpose()?,
            },
        })
    }
}

fn frame_name(frame: FrameId) -> &'static str {
    match frame {
        FrameId::Hueman => "hueman",
        FrameId::Gremlin => "gremlin",
        FrameId::Goblin => "goblin",
        FrameId::Ghoul => "ghoul",
        FrameId::Spectre => "spectre",
        FrameId::Troll => "troll",
        FrameId::Ork => "ork",
        FrameId::Ogre => "ogre",
        FrameId::Troglodyte => "troglodyte",
        FrameId::Pixy => "pixy",
        FrameId::Sprite => "sprite",
        FrameId::Faerie => "faerie",
        FrameId::Nymph => "nymph",
        FrameId::Siren => "siren",
        FrameId::Muse => "muse",
        FrameId::Werewolf => "werewolf",
        FrameId::Gargoyle => "gargoyle",
        FrameId::Merman => "merman",
        FrameId::Chimera => "chimera",
        FrameId::Gnome => "gnome",
        FrameId::Minotaur => "minotaur",
        FrameId::Hecaton => "hecaton",
        FrameId::Elf => "elf",
        FrameId::Centaur => "centaur",
        FrameId::Pegasus => "pegasus",
    }
}

fn parse_frame(value: &str) -> Result<FrameId, FacultyCodecError> {
    match value {
        "hueman" => Ok(FrameId::Hueman),
        "gremlin" => Ok(FrameId::Gremlin),
        "goblin" => Ok(FrameId::Goblin),
        "ghoul" => Ok(FrameId::Ghoul),
        "spectre" => Ok(FrameId::Spectre),
        "troll" => Ok(FrameId::Troll),
        "ork" => Ok(FrameId::Ork),
        "ogre" => Ok(FrameId::Ogre),
        "troglodyte" => Ok(FrameId::Troglodyte),
        "pixy" => Ok(FrameId::Pixy),
        "sprite" => Ok(FrameId::Sprite),
        "faerie" => Ok(FrameId::Faerie),
        "nymph" => Ok(FrameId::Nymph),
        "siren" => Ok(FrameId::Siren),
        "muse" => Ok(FrameId::Muse),
        "werewolf" => Ok(FrameId::Werewolf),
        "gargoyle" => Ok(FrameId::Gargoyle),
        "merman" => Ok(FrameId::Merman),
        "chimera" => Ok(FrameId::Chimera),
        "gnome" => Ok(FrameId::Gnome),
        "minotaur" => Ok(FrameId::Minotaur),
        "hecaton" => Ok(FrameId::Hecaton),
        "elf" => Ok(FrameId::Elf),
        "centaur" => Ok(FrameId::Centaur),
        "pegasus" => Ok(FrameId::Pegasus),
        _ => Err(FacultyCodecError::UnknownFrame(value.into())),
    }
}

fn flow_name(flow: FlowId) -> &'static str {
    match flow {
        FlowId::TinkerGrip => "tinker-grip",
        FlowId::Stonefold => "stonefold",
        FlowId::PressureRelocation => "pressure-relocation",
        FlowId::PackRelay => "pack-relay",
        FlowId::Moonrush => "moonrush",
        FlowId::MeteorDrop => "meteor-drop",
        FlowId::RiptideSwim => "riptide-swim",
    }
}

fn parse_flow(value: &str) -> Result<FlowId, FacultyCodecError> {
    match value {
        "tinker-grip" => Ok(FlowId::TinkerGrip),
        "stonefold" => Ok(FlowId::Stonefold),
        "pressure-relocation" => Ok(FlowId::PressureRelocation),
        "pack-relay" => Ok(FlowId::PackRelay),
        "moonrush" => Ok(FlowId::Moonrush),
        "meteor-drop" => Ok(FlowId::MeteorDrop),
        "riptide-swim" => Ok(FlowId::RiptideSwim),
        _ => Err(FacultyCodecError::UnknownFlow(value.into())),
    }
}

fn glow_name(glow: GlowId) -> &'static str {
    match glow {
        GlowId::Confusion => "confusion",
        GlowId::Projection => "projection",
        GlowId::Recognition => "recognition",
        GlowId::SpriteCall => "sprite-call",
        GlowId::FaerieVeil => "faerie-veil",
        GlowId::MuseChorus => "muse-chorus",
    }
}

fn parse_glow(value: &str) -> Result<GlowId, FacultyCodecError> {
    match value {
        "confusion" => Ok(GlowId::Confusion),
        "projection" => Ok(GlowId::Projection),
        "recognition" => Ok(GlowId::Recognition),
        "sprite-call" => Ok(GlowId::SpriteCall),
        "faerie-veil" => Ok(GlowId::FaerieVeil),
        "muse-chorus" => Ok(GlowId::MuseChorus),
        _ => Err(FacultyCodecError::UnknownGlow(value.into())),
    }
}

fn polarity_name(polarity: ConstitutionalPolarity) -> &'static str {
    match polarity {
        ConstitutionalPolarity::PositiveCurrentPositiveAura => "positive-current-positive-aura",
        ConstitutionalPolarity::PositiveCurrentNegativeAura => "positive-current-negative-aura",
        ConstitutionalPolarity::NegativeCurrentPositiveAura => "negative-current-positive-aura",
        ConstitutionalPolarity::NegativeCurrentNegativeAura => "negative-current-negative-aura",
    }
}

fn parse_polarity(value: &str) -> Result<ConstitutionalPolarity, FacultyCodecError> {
    match value {
        "positive-current-positive-aura" => Ok(ConstitutionalPolarity::PositiveCurrentPositiveAura),
        "positive-current-negative-aura" => Ok(ConstitutionalPolarity::PositiveCurrentNegativeAura),
        "negative-current-positive-aura" => Ok(ConstitutionalPolarity::NegativeCurrentPositiveAura),
        "negative-current-negative-aura" => Ok(ConstitutionalPolarity::NegativeCurrentNegativeAura),
        _ => Err(FacultyCodecError::UnknownPolarity(value.into())),
    }
}

fn bond_phase_name(phase: BondPhase) -> &'static str {
    match phase {
        BondPhase::Formed => "formed",
        BondPhase::Validated => "validated",
        BondPhase::Active => "active",
        BondPhase::Mature => "mature",
        BondPhase::ExcessCalculated => "excess-calculated",
        BondPhase::EligibilityDecided => "eligibility-decided",
        BondPhase::TombstoneFormed => "tombstone-formed",
        BondPhase::TombstoneValidated => "tombstone-validated",
        BondPhase::Recorded => "recorded",
        BondPhase::Resolved => "resolved",
    }
}

fn parse_bond_phase(value: &str) -> Result<BondPhase, FacultyCodecError> {
    match value {
        "formed" => Ok(BondPhase::Formed),
        "validated" => Ok(BondPhase::Validated),
        "active" => Ok(BondPhase::Active),
        "mature" => Ok(BondPhase::Mature),
        "excess-calculated" => Ok(BondPhase::ExcessCalculated),
        "eligibility-decided" => Ok(BondPhase::EligibilityDecided),
        "tombstone-formed" => Ok(BondPhase::TombstoneFormed),
        "tombstone-validated" => Ok(BondPhase::TombstoneValidated),
        "recorded" => Ok(BondPhase::Recorded),
        "resolved" => Ok(BondPhase::Resolved),
        _ => Err(FacultyCodecError::UnknownBondPhase(value.into())),
    }
}
