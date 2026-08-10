//! Constitutional, advisory-only records for Cleopatra and Hal.
//!
//! These personas interpret a case; they never commit a House, Bond,
//! Sympiote, Title, or gameplay state transition.

use serde::{Deserialize, Serialize};

pub const AI_PERSONA_SCHEMA_VERSION: u16 = 1;
pub const RETURNING_GOON_CASE_ID: &str = "case.boardwalk.returning-goon.v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SharedInterfaceRecordKind {
    Court,
    Amendment,
    Restitution,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SharedInterfaceRecord {
    pub kind: SharedInterfaceRecordKind,
    pub record_id: String,
    pub subject_id: String,
    pub authority_id: String,
    pub status: String,
    pub causal_position: u64,
    pub provenance_id: String,
}

impl SharedInterfaceRecord {
    pub fn validate(&self) -> Result<(), AiPersonaError> {
        for (label, value) in [
            ("record_id", &self.record_id),
            ("subject_id", &self.subject_id),
            ("authority_id", &self.authority_id),
            ("status", &self.status),
            ("provenance_id", &self.provenance_id),
        ] {
            if value.is_empty() || value.chars().any(char::is_whitespace) {
                return Err(AiPersonaError::InvalidRecord(format!(
                    "shared interface {label} is empty or not stable"
                )));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AiPersona {
    Cleopatra,
    Hal,
    Clouseau,
}

impl AiPersona {
    #[must_use]
    pub const fn stable_id(self) -> &'static str {
        match self {
            Self::Cleopatra => "ai.persona.cleopatra.v1",
            Self::Hal => "ai.persona.hal.v1",
            Self::Clouseau => "current-synthesis.client.clouseau.v1",
        }
    }

    #[must_use]
    pub const fn associated_title(self) -> &'static str {
        match self {
            Self::Cleopatra => "Gimp of the Underway",
            Self::Hal => "Pimp of the Boardwalk",
            Self::Clouseau => "PLEB-PLEB witness",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AdvisoryStatus {
    AcceptedAdvisory,
    Inconclusive,
    SourceConflict,
    ScopeDenied,
    ConsentAbsent,
    CoercionRisk,
    OperatorReviewRequired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PersonaCaseInput {
    pub case_id: String,
    pub subject_id: String,
    pub causal_position: u64,
    pub question: String,
    pub source_records: Vec<String>,
    pub consent_state: String,
    pub public_scope: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PersonaAdvisoryRecord {
    pub schema_version: u16,
    pub record_id: String,
    pub persona: AiPersona,
    pub persona_id: String,
    pub associated_title: String,
    pub case: PersonaCaseInput,
    pub status: AdvisoryStatus,
    pub finding: String,
    pub uncertainty: String,
    pub refusal_state: String,
    pub next_handoff: String,
    pub world_state_committed: bool,
    pub replay_checksum: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AiPersonaError {
    Json(String),
    ChecksumMismatch { expected: String, actual: String },
    InvalidRecord(String),
}

impl std::fmt::Display for AiPersonaError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json(error) => write!(formatter, "persona JSON error: {error}"),
            Self::ChecksumMismatch { expected, actual } => {
                write!(
                    formatter,
                    "persona checksum mismatch: expected {expected}, got {actual}"
                )
            }
            Self::InvalidRecord(error) => write!(formatter, "invalid persona record: {error}"),
        }
    }
}

impl std::error::Error for AiPersonaError {}

#[must_use]
pub fn evaluate_persona(persona: AiPersona, case: PersonaCaseInput) -> PersonaAdvisoryRecord {
    let (finding, uncertainty, refusal_state, next_handoff) = match persona {
        AiPersona::Cleopatra => (
            "The returning Goon has a protected Underway route back to the Boardwalk, but the Goon's affiliation remains undecided.".into(),
            "Route testimony is attributable but does not prove consent, Bond, or title succession.".into(),
            "The Goon may refuse the route, the Gimp's protection, or any proposed Bond.".into(),
            "affected Being and Stonebend continuity review".into(),
        ),
        AiPersona::Hal => (
            "A public return may be staged through disclosed patronage, provided the Goon retains independent identity and exit.".into(),
            "Public attention is not proof of competence, Hoe affiliation, consent, or recognition.".into(),
            "The Goon may decline visibility, patronage, or public association without retaliation.".into(),
            "affected Being and Flynt public-function review".into(),
        ),
        AiPersona::Clouseau => (
            "The PLEB-PLEB path preserves a clue-continuity and Bond relation for review; it does not create or execute a Bond.".into(),
            "Current Synthesis owns the route grammar, and the witness does not authorize movement or partner selection.".into(),
            "The witness may be refused, challenged, or returned to the Current Synthesis boundary without penalty.".into(),
            "Current Synthesis boundary and affected Being".into(),
        ),
    };

    let mut record = PersonaAdvisoryRecord {
        schema_version: AI_PERSONA_SCHEMA_VERSION,
        record_id: format!(
            "record.{}.{}.{}",
            persona.stable_id(),
            case.case_id,
            case.causal_position
        ),
        persona,
        persona_id: persona.stable_id().into(),
        associated_title: persona.associated_title().into(),
        case,
        status: AdvisoryStatus::AcceptedAdvisory,
        finding,
        uncertainty,
        refusal_state,
        next_handoff,
        world_state_committed: false,
        replay_checksum: String::new(),
    };
    record.replay_checksum =
        checksum_without_digest(&record).expect("advisory record is serializable");
    record
}

/// Serializes an advisory record with its checksum. No function in this module
/// can commit gameplay or constitutional state.
pub fn encode_advisory(record: &PersonaAdvisoryRecord) -> Result<Vec<u8>, AiPersonaError> {
    serde_json::to_vec(record).map_err(|error| AiPersonaError::Json(error.to_string()))
}

pub fn replay_advisory(bytes: &[u8]) -> Result<PersonaAdvisoryRecord, AiPersonaError> {
    let record: PersonaAdvisoryRecord =
        serde_json::from_slice(bytes).map_err(|error| AiPersonaError::Json(error.to_string()))?;
    let expected = checksum_without_digest(&record)?;
    if record.replay_checksum != expected {
        return Err(AiPersonaError::ChecksumMismatch {
            expected,
            actual: record.replay_checksum,
        });
    }
    if record.world_state_committed {
        return Err(AiPersonaError::InvalidRecord(
            "an AI persona advisory cannot commit world state".into(),
        ));
    }
    Ok(record)
}

#[must_use]
pub fn canonical_returning_goon_case() -> PersonaCaseInput {
    PersonaCaseInput {
        case_id: RETURNING_GOON_CASE_ID.into(),
        subject_id: "being.boardwalk.returning-goon.v1".into(),
        causal_position: 42,
        question: "What can return safely, and what remains the Goon's choice?".into(),
        source_records: vec![
            "testimony.boardwalk.hoe-witness-a.v1".into(),
            "testimony.boardwalk.goon-witness-a.v1".into(),
            "clearance.glaüshouse.return.v1".into(),
        ],
        consent_state: "capable-subject-choice-required".into(),
        public_scope: "boardwalk-return-invitation-only".into(),
    }
}

fn checksum_without_digest(record: &PersonaAdvisoryRecord) -> Result<String, AiPersonaError> {
    let mut unsigned = record.clone();
    unsigned.replay_checksum.clear();
    let bytes =
        serde_json::to_vec(&unsigned).map_err(|error| AiPersonaError::Json(error.to_string()))?;
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in bytes {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    Ok(format!("fnv1a64-ai-persona-v1:{hash:016x}"))
}
