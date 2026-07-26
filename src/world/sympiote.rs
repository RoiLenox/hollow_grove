//! Crafted Sympiote graft lifecycle and Stonebend proving.
//!
//! A Sympiote is a crafted living graft, not the Sympian lineage and not a
//! player-selected power package. Its expression emerges from the host,
//! Glaüshouse craft, and lived integration.

use serde::{Deserialize, Serialize};

pub const SYMPIOTE_SOURCE: &str = "HOLLOW_GROVE_POWER_RECIPE_CONSTITUTION_V1.md";
pub const SYMPIOTE_ARCHIVE_FORMAT: &str = "HGSYM";
pub const SYMPIOTE_ARCHIVE_SCHEMA_VERSION: u16 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SympioteClassification {
    CraftedLivingGraft,
}

impl SympioteClassification {
    #[must_use]
    pub const fn is_sympian_lineage(self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SympioteIntegrationOutcome {
    HostRejection,
    #[serde(alias = "GraftRejection")]
    SympioteRejection,
    PartialIntegration,
    FailedIntegration,
    ReciprocalSynthesis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SympiotePhase {
    AwaitingHostSample,
    HostSampled,
    HostRecipeRead,
    LivingTissueCultivated,
    CraftedForHost,
    Grafted,
    Monitoring,
    HostRejected,
    #[serde(alias = "GraftRejected")]
    SympioteRejected,
    PartiallyIntegrated,
    IntegrationFailed,
    ReciprocallyIntegrated,
}

impl SympiotePhase {
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::HostRejected
                | Self::SympioteRejected
                | Self::PartiallyIntegrated
                | Self::IntegrationFailed
                | Self::ReciprocallyIntegrated
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SympioteAction {
    SampleHost,
    ReadHostCurrentAuraRecipe,
    CultivateLivingTissue,
    CraftForHost {
        requested_power_package: Option<String>,
    },
    GraftWithConsent,
    BeginCompatibilityMonitoring,
    ResolveIntegration {
        outcome: SympioteIntegrationOutcome,
        emergent_form: Option<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SympioteActionRecord {
    pub causal_position: u64,
    pub evidence: String,
    pub action: SympioteAction,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SympioteEventRecord {
    pub causal_position: u64,
    pub evidence: String,
    pub action: SympioteAction,
    pub from: SympiotePhase,
    pub to: SympiotePhase,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SympioteGraft {
    pub graft_id: String,
    pub host_id: String,
    pub classification: SympioteClassification,
    pub phase: SympiotePhase,
    pub emergent_form: Option<String>,
    pub events: Vec<SympioteEventRecord>,
}

impl SympioteGraft {
    pub fn new(graft_id: impl Into<String>, host_id: impl Into<String>) -> Self {
        Self {
            graft_id: graft_id.into(),
            host_id: host_id.into(),
            classification: SympioteClassification::CraftedLivingGraft,
            phase: SympiotePhase::AwaitingHostSample,
            emergent_form: None,
            events: Vec::new(),
        }
    }

    pub fn apply(&mut self, record: SympioteActionRecord) -> Result<(), SympioteError> {
        if self.graft_id.trim().is_empty() {
            return Err(SympioteError::EmptyGraftId);
        }
        if self.host_id.trim().is_empty() {
            return Err(SympioteError::EmptyHostId);
        }
        if record.evidence.trim().is_empty() {
            return Err(SympioteError::MissingEvidence);
        }
        if self
            .events
            .last()
            .is_some_and(|event| record.causal_position <= event.causal_position)
        {
            return Err(SympioteError::NonMonotonicCausalPosition);
        }
        if self.phase.is_terminal() {
            return Err(SympioteError::TerminalState(self.phase));
        }

        let from = self.phase;
        let (to, emergent_form) = transition(self.phase, &record.action)?;
        self.phase = to;
        if emergent_form.is_some() {
            self.emergent_form = emergent_form;
        }
        self.events.push(SympioteEventRecord {
            causal_position: record.causal_position,
            evidence: record.evidence,
            action: record.action,
            from,
            to,
        });
        Ok(())
    }

    #[must_use]
    pub const fn player_selected_bias(&self) -> Option<&str> {
        None
    }

    #[must_use]
    pub const fn successful_reciprocal_synthesis(&self) -> bool {
        matches!(self.phase, SympiotePhase::ReciprocallyIntegrated)
    }
}

fn transition(
    phase: SympiotePhase,
    action: &SympioteAction,
) -> Result<(SympiotePhase, Option<String>), SympioteError> {
    match (phase, action) {
        (SympiotePhase::AwaitingHostSample, SympioteAction::SampleHost) => {
            Ok((SympiotePhase::HostSampled, None))
        }
        (SympiotePhase::HostSampled, SympioteAction::ReadHostCurrentAuraRecipe) => {
            Ok((SympiotePhase::HostRecipeRead, None))
        }
        (SympiotePhase::HostRecipeRead, SympioteAction::CultivateLivingTissue) => {
            Ok((SympiotePhase::LivingTissueCultivated, None))
        }
        (
            SympiotePhase::LivingTissueCultivated,
            SympioteAction::CraftForHost {
                requested_power_package: None,
            },
        ) => Ok((SympiotePhase::CraftedForHost, None)),
        (
            SympiotePhase::LivingTissueCultivated,
            SympioteAction::CraftForHost {
                requested_power_package: Some(_),
            },
        ) => Err(SympioteError::PlayerSelectedBiasForbidden),
        (SympiotePhase::CraftedForHost, SympioteAction::GraftWithConsent) => {
            Ok((SympiotePhase::Grafted, None))
        }
        (SympiotePhase::Grafted, SympioteAction::BeginCompatibilityMonitoring) => {
            Ok((SympiotePhase::Monitoring, None))
        }
        (
            SympiotePhase::Monitoring,
            SympioteAction::ResolveIntegration {
                outcome,
                emergent_form,
            },
        ) => {
            let to = match outcome {
                SympioteIntegrationOutcome::HostRejection => SympiotePhase::HostRejected,
                SympioteIntegrationOutcome::SympioteRejection => SympiotePhase::SympioteRejected,
                SympioteIntegrationOutcome::PartialIntegration => {
                    SympiotePhase::PartiallyIntegrated
                }
                SympioteIntegrationOutcome::FailedIntegration => SympiotePhase::IntegrationFailed,
                SympioteIntegrationOutcome::ReciprocalSynthesis => {
                    SympiotePhase::ReciprocallyIntegrated
                }
            };
            if *outcome == SympioteIntegrationOutcome::ReciprocalSynthesis
                && emergent_form
                    .as_deref()
                    .is_none_or(|form| form.trim().is_empty())
            {
                return Err(SympioteError::SuccessfulIntegrationWithoutForm);
            }
            if matches!(
                outcome,
                SympioteIntegrationOutcome::HostRejection
                    | SympioteIntegrationOutcome::SympioteRejection
                    | SympioteIntegrationOutcome::FailedIntegration
            ) && emergent_form.is_some()
            {
                return Err(SympioteError::FailedIntegrationClaimsForm);
            }
            Ok((to, emergent_form.clone()))
        }
        _ => Err(SympioteError::InvalidTransition { phase }),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SympioteError {
    EmptyGraftId,
    EmptyHostId,
    MissingEvidence,
    NonMonotonicCausalPosition,
    TerminalState(SympiotePhase),
    InvalidTransition { phase: SympiotePhase },
    PlayerSelectedBiasForbidden,
    SuccessfulIntegrationWithoutForm,
    FailedIntegrationClaimsForm,
    Json(String),
    UnsupportedFormat(String),
    UnsupportedSchema(u16),
    ChecksumMismatch,
}

impl std::fmt::Display for SympioteError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "Sympiote constitution rejected state: {self:?}")
    }
}

impl std::error::Error for SympioteError {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct SympioteArchivePayload {
    graft_id: String,
    host_id: String,
    actions: Vec<SympioteActionRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct SympioteArchive {
    format: String,
    schema_version: u16,
    checksum: String,
    payload: SympioteArchivePayload,
}

pub fn encode_sympiote_history(graft: &SympioteGraft) -> Result<Vec<u8>, SympioteError> {
    let payload = SympioteArchivePayload {
        graft_id: graft.graft_id.clone(),
        host_id: graft.host_id.clone(),
        actions: graft
            .events
            .iter()
            .map(|event| SympioteActionRecord {
                causal_position: event.causal_position,
                evidence: event.evidence.clone(),
                action: event.action.clone(),
            })
            .collect(),
    };
    let replayed = replay_payload(&payload)?;
    if &replayed != graft {
        return Err(SympioteError::ChecksumMismatch);
    }
    let archive = SympioteArchive {
        format: SYMPIOTE_ARCHIVE_FORMAT.into(),
        schema_version: SYMPIOTE_ARCHIVE_SCHEMA_VERSION,
        checksum: checksum(&payload)?,
        payload,
    };
    serde_json::to_vec(&archive).map_err(|error| SympioteError::Json(error.to_string()))
}

pub fn decode_sympiote_history(bytes: &[u8]) -> Result<SympioteGraft, SympioteError> {
    let archive: SympioteArchive =
        serde_json::from_slice(bytes).map_err(|error| SympioteError::Json(error.to_string()))?;
    if archive.format != SYMPIOTE_ARCHIVE_FORMAT {
        return Err(SympioteError::UnsupportedFormat(archive.format));
    }
    if archive.schema_version != SYMPIOTE_ARCHIVE_SCHEMA_VERSION {
        return Err(SympioteError::UnsupportedSchema(archive.schema_version));
    }
    if archive.checksum != checksum(&archive.payload)? {
        return Err(SympioteError::ChecksumMismatch);
    }
    replay_payload(&archive.payload)
}

fn replay_payload(payload: &SympioteArchivePayload) -> Result<SympioteGraft, SympioteError> {
    let mut graft = SympioteGraft::new(&payload.graft_id, &payload.host_id);
    for action in &payload.actions {
        graft.apply(action.clone())?;
    }
    Ok(graft)
}

fn checksum<T: Serialize>(value: &T) -> Result<String, SympioteError> {
    let bytes =
        serde_json::to_vec(value).map_err(|error| SympioteError::Json(error.to_string()))?;
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for byte in bytes {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    Ok(format!("{hash:016x}"))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StonebendProvingJudgment {
    Recognized,
    Provisional,
    ReferredToGlaushouse,
    Rejected,
    Severance,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StonebendProvingAssessment {
    pub candidate_id: String,
    pub integration: SympioteIntegrationOutcome,
    pub stable_form: bool,
    pub restraint: bool,
    pub repeatable: bool,
    pub reciprocal_control: bool,
    pub identity_survives_pressure: bool,
    pub glaushouse_clearance: bool,
    pub coercive: bool,
    pub actively_destructive: bool,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StonebendProvingRecord {
    pub assessment: StonebendProvingAssessment,
    pub judgment: StonebendProvingJudgment,
    pub power_grants_title_or_office: bool,
}

pub fn judge_sympiote_integration(
    assessment: StonebendProvingAssessment,
) -> Result<StonebendProvingRecord, StonebendProvingError> {
    if assessment.candidate_id.trim().is_empty() {
        return Err(StonebendProvingError::EmptyCandidate);
    }
    if assessment.evidence.is_empty()
        || assessment
            .evidence
            .iter()
            .any(|evidence| evidence.trim().is_empty())
    {
        return Err(StonebendProvingError::MissingEvidence);
    }
    let judgment = if assessment.coercive || assessment.actively_destructive {
        StonebendProvingJudgment::Severance
    } else if !assessment.glaushouse_clearance {
        StonebendProvingJudgment::ReferredToGlaushouse
    } else if assessment.integration == SympioteIntegrationOutcome::ReciprocalSynthesis
        && assessment.stable_form
        && assessment.restraint
        && assessment.repeatable
        && assessment.reciprocal_control
        && assessment.identity_survives_pressure
    {
        StonebendProvingJudgment::Recognized
    } else if matches!(
        assessment.integration,
        SympioteIntegrationOutcome::PartialIntegration
            | SympioteIntegrationOutcome::ReciprocalSynthesis
    ) && assessment.identity_survives_pressure
        && assessment.restraint
    {
        StonebendProvingJudgment::Provisional
    } else {
        StonebendProvingJudgment::Rejected
    };
    Ok(StonebendProvingRecord {
        assessment,
        judgment,
        power_grants_title_or_office: false,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StonebendProvingError {
    EmptyCandidate,
    MissingEvidence,
}

impl std::fmt::Display for StonebendProvingError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "Stonebend proving rejected state: {self:?}")
    }
}

impl std::error::Error for StonebendProvingError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SympioteDiversionWitness {
    pub starting_form: &'static str,
    pub expected_progression: &'static str,
    pub emergent_synthesis_form: &'static str,
    pub executes_progression: bool,
    pub status: &'static str,
}

#[must_use]
pub const fn gremlin_gargoyle_diversion_witness() -> SympioteDiversionWitness {
    SympioteDiversionWitness {
        starting_form: "Gremlin",
        expected_progression: "Goblin",
        emergent_synthesis_form: "Gargoyle",
        executes_progression: false,
        status: "proposed iconic witness requiring lived integration and Stonebend judgment",
    }
}
