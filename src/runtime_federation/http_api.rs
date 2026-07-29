//! Versioned local HTTP API contract for authoritative semantic Federation cycles.
//!
//! Transport code may expose this service over loopback HTTP, but all semantic
//! decisions still pass through the existing Federation orchestrator and
//! Festival. State is staged in a cloned catalog and becomes current only after
//! its provenance snapshot has been durably replaced.

use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::artifact_io::write_bytes_atomically;
use crate::composition::{CompositionCatalog, ExternalRef, Position3, ScaleKey, Translation3};
use crate::constitutional::GrovePhase;
use crate::semantic::{
    SemanticProvenanceError, decode_semantic_provenance_snapshot,
    encode_semantic_provenance_snapshot,
};
use crate::world::composition_witnesses::canonical_composition_witness_catalog;

use super::{
    FederationError, FederationResult, LocalFederationCycleError, LocalFederationCycleInput,
    SemanticDecision, SemanticInterpreter, run_local_federation_cycle,
};

pub const FEDERATION_HTTP_API_SCHEMA_VERSION: &str = "hollow-grove-federation-api-v1";
pub const DEFAULT_FEDERATION_HTTP_ADDRESS: &str = "127.0.0.1:47820";
pub const MAX_FEDERATION_HTTP_REQUEST_BODY_BYTES: usize = 65_536;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FederationApiCycleRequest {
    pub point_ref: String,
    pub scale_key: String,
    pub intent: String,
    #[serde(default)]
    pub context_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FederationApiCycleResponse {
    pub schema_version: &'static str,
    pub cycle_id: String,
    pub phases: Vec<String>,
    pub decision: FederationApiDecision,
    pub next_position: Option<Position3>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum FederationApiDecision {
    Confirmed {
        proposal_ref: String,
        point_ref: String,
        scale_key: String,
        previous_position: Option<Position3>,
        confirmed_position: Position3,
        translation: Option<Translation3>,
        composition_ref: String,
        witness_ref: String,
    },
    Rejected {
        proposal_ref: String,
        point_ref: String,
        scale_key: String,
        proposed_position: Position3,
        reasons: Vec<String>,
        witness_ref: String,
    },
}

impl FederationApiCycleResponse {
    #[must_use]
    pub fn from_result(result: &FederationResult) -> Self {
        let decision = match &result.festival.decision {
            SemanticDecision::Confirmed(confirmed) => FederationApiDecision::Confirmed {
                proposal_ref: external_label(&confirmed.proposal_id),
                point_ref: external_label(&confirmed.point_ref),
                scale_key: confirmed.scope.scale.as_str().into(),
                previous_position: confirmed.previous_position,
                confirmed_position: confirmed.confirmed_position,
                translation: confirmed.translation,
                composition_ref: external_label(&confirmed.composition_ref),
                witness_ref: external_label(&confirmed.witness_ref),
            },
            SemanticDecision::Rejected(rejected) => FederationApiDecision::Rejected {
                proposal_ref: external_label(&rejected.proposal_id),
                point_ref: external_label(&rejected.point_ref),
                scale_key: rejected.scope.scale.as_str().into(),
                proposed_position: rejected.proposed_position,
                reasons: rejected
                    .reasons
                    .iter()
                    .map(|reason| format!("{reason:?}"))
                    .collect(),
                witness_ref: external_label(&rejected.witness_ref),
            },
        };
        Self {
            schema_version: FEDERATION_HTTP_API_SCHEMA_VERSION,
            cycle_id: result.next_way_back.cycle_id.as_str().into(),
            phases: result
                .phase_trace
                .iter()
                .copied()
                .map(phase_name)
                .map(str::to_owned)
                .collect(),
            decision,
            next_position: result.next_way_back.current_position,
        }
    }
}

/// Owns one authoritative semantic catalog for a local API process.
///
/// Callers must serialize mutable access to this value. The loopback HTTP
/// server does so with a single mutex, which prevents lost updates and ensures
/// each request's Way Back observes the preceding committed Festival.
pub struct FederationHttpApi<I> {
    interpreter: I,
    catalog: CompositionCatalog,
    state_path: PathBuf,
}

impl<I> FederationHttpApi<I>
where
    I: SemanticInterpreter + Clone,
{
    pub fn from_state_path(
        interpreter: I,
        state_path: impl Into<PathBuf>,
    ) -> Result<Self, FederationHttpApiError> {
        let state_path = state_path.into();
        let catalog = load_local_federation_catalog(&state_path)?;
        Ok(Self {
            interpreter,
            catalog,
            state_path,
        })
    }

    #[must_use]
    pub const fn catalog(&self) -> &CompositionCatalog {
        &self.catalog
    }

    #[must_use]
    pub fn state_path(&self) -> &Path {
        &self.state_path
    }

    pub async fn execute(
        &mut self,
        request: FederationApiCycleRequest,
    ) -> Result<FederationApiCycleResponse, FederationHttpApiError> {
        let point_ref = parse_external_reference(&request.point_ref, "point_ref")?;
        let scale = ScaleKey::new(request.scale_key)
            .map_err(|error| FederationHttpApiError::InvalidRequest(error.to_string()))?;
        let mut input = LocalFederationCycleInput::new(point_ref, scale, request.intent)
            .map_err(FederationHttpApiError::Cycle)?;
        input.context_ref = request
            .context_ref
            .as_deref()
            .map(|reference| parse_external_reference(reference, "context_ref"))
            .transpose()?;

        let mut staged = self.catalog.clone();
        let result = run_local_federation_cycle(self.interpreter.clone(), &input, &mut staged)
            .await
            .map_err(FederationHttpApiError::Cycle)?;
        let snapshot = encode_semantic_provenance_snapshot(&staged)
            .map_err(FederationHttpApiError::Provenance)?;
        write_bytes_atomically(&self.state_path, &snapshot)
            .map_err(FederationHttpApiError::StateWrite)?;
        self.catalog = staged;
        Ok(FederationApiCycleResponse::from_result(&result))
    }
}

pub fn load_local_federation_catalog(
    path: &Path,
) -> Result<CompositionCatalog, FederationHttpApiError> {
    let mut catalog = canonical_composition_witness_catalog()
        .map_err(|error| FederationHttpApiError::Catalog(error.to_string()))?;
    match fs::read(path) {
        Ok(bytes) => {
            decode_semantic_provenance_snapshot(&bytes, &mut catalog)
                .map_err(FederationHttpApiError::Provenance)?;
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => return Err(FederationHttpApiError::StateRead(error)),
    }
    Ok(catalog)
}

fn parse_external_reference(
    value: &str,
    field: &'static str,
) -> Result<ExternalRef, FederationHttpApiError> {
    let Some((namespace, key)) = value.split_once('.') else {
        return Err(FederationHttpApiError::InvalidRequest(format!(
            "{field} must contain a namespace and key"
        )));
    };
    ExternalRef::new(namespace, key)
        .map_err(|error| FederationHttpApiError::InvalidRequest(error.to_string()))
}

fn external_label(reference: &ExternalRef) -> String {
    format!("{}.{}", reference.namespace, reference.key)
}

const fn phase_name(phase: GrovePhase) -> &'static str {
    match phase {
        GrovePhase::TheWayBack => "The Way Back",
        GrovePhase::TheInitiation => "The Initiation",
        GrovePhase::TheGathering => "The Gathering",
        GrovePhase::TheFestival => "The Festival",
    }
}

#[derive(Debug)]
pub enum FederationHttpApiError {
    InvalidRequest(String),
    Catalog(String),
    StateRead(io::Error),
    StateWrite(io::Error),
    Provenance(SemanticProvenanceError),
    Cycle(LocalFederationCycleError),
}

impl FederationHttpApiError {
    #[must_use]
    pub const fn http_status(&self) -> u16 {
        match self {
            Self::InvalidRequest(_) => 400,
            Self::Cycle(LocalFederationCycleError::InvalidIntent { .. })
            | Self::Cycle(LocalFederationCycleError::ExternalReference(_))
            | Self::Cycle(LocalFederationCycleError::GuideSession(_)) => 400,
            Self::Cycle(LocalFederationCycleError::Federation(FederationError::Interpreter(
                super::InterpreterError::InterpreterTimeout,
            ))) => 504,
            Self::Cycle(LocalFederationCycleError::Federation(FederationError::Interpreter(
                super::InterpreterError::InterpreterUnavailable,
            ))) => 503,
            Self::Cycle(LocalFederationCycleError::Federation(FederationError::Interpreter(_))) => {
                502
            }
            Self::Cycle(_) => 422,
            Self::Catalog(_) | Self::StateRead(_) | Self::StateWrite(_) | Self::Provenance(_) => {
                500
            }
        }
    }

    #[must_use]
    pub const fn error_type(&self) -> &'static str {
        match self.http_status() {
            400 => "invalid_request",
            422 => "federation_rejected_request",
            502 => "invalid_interpreter_response",
            503 => "interpreter_unavailable",
            504 => "interpreter_timeout",
            _ => "internal_error",
        }
    }
}

impl fmt::Display for FederationHttpApiError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Federation HTTP API failed: {self:?}")
    }
}

impl std::error::Error for FederationHttpApiError {}
