//! Transformer-assisted actions inside the one Runtime Federation.
//!
//! Roï restores continuity, Rosaliña inspects embodiment, and the existing
//! composition catalog remains the authoritative semantic state.

use std::fmt;
use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};

use crate::composition::{
    CompositionCatalog, CompositionRecordId, ExternalRef, Position3, ScaleKey, SemanticOperation,
    SemanticPositionRecord, SemanticScope, SemanticWitnessDecision, SemanticWitnessRecord,
    Translation3,
};
use crate::constitutional::{GroveCycleId, GrovePhase};
use crate::semantic::{
    NearbySemanticExample, RetrievedCanonFragment, SemanticProvenanceError,
    confirm_semantic_transition, record_rejected_semantic_witness,
};

pub const SEMANTIC_PROPOSAL_SCHEMA_VERSION: &str = "hollow-grove-semantic-proposal-v1";
pub const DETERMINISTIC_INTERPRETER_VERSION: &str = "deterministic-semantic-interpreter-v1";
pub const MAX_MODEL_RESPONSE_BYTES: usize = 65_536;
pub const MAX_MODEL_SCALE_BYTES: usize = 128;
pub const MAX_MODEL_SUMMARY_BYTES: usize = 2_048;
pub const MAX_MODEL_EVIDENCE_REFS: usize = 32;
pub const MAX_MODEL_EVIDENCE_REF_BYTES: usize = 512;
pub const MAX_MODEL_TAGS: usize = 32;
pub const MAX_MODEL_TAG_BYTES: usize = 128;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProposedSemanticOperation {
    Locate,
    Inspect,
    Translate,
    Repair,
    Compose,
    Transform,
}

impl ProposedSemanticOperation {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Locate => "locate",
            Self::Inspect => "inspect",
            Self::Translate => "translate",
            Self::Repair => "repair",
            Self::Compose => "compose",
            Self::Transform => "transform",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelSemanticProposal {
    pub scale_key: String,
    pub proposed_position: Position3,
    pub operation: ProposedSemanticOperation,
    pub evidence_refs: Vec<String>,
    pub semantic_tags: Vec<String>,
    pub summary: String,
    pub confidence_millis: u16,
}

impl ModelSemanticProposal {
    pub fn validate(&self) -> Result<(), InterpreterError> {
        if self.scale_key.trim().is_empty() {
            return Err(InterpreterError::MissingModelField("scale_key".into()));
        }
        if self.summary.trim().is_empty() {
            return Err(InterpreterError::MissingModelField("summary".into()));
        }
        if self.confidence_millis > 1000 {
            return Err(InterpreterError::ConfidenceOutOfBounds(
                self.confidence_millis,
            ));
        }
        if self.scale_key.len() > MAX_MODEL_SCALE_BYTES {
            return Err(InterpreterError::ModelFieldTooLarge {
                field: "scale_key",
                actual: self.scale_key.len(),
                maximum: MAX_MODEL_SCALE_BYTES,
            });
        }
        if self.summary.len() > MAX_MODEL_SUMMARY_BYTES {
            return Err(InterpreterError::ModelFieldTooLarge {
                field: "summary",
                actual: self.summary.len(),
                maximum: MAX_MODEL_SUMMARY_BYTES,
            });
        }
        if self.evidence_refs.len() > MAX_MODEL_EVIDENCE_REFS {
            return Err(InterpreterError::TooManyEvidenceReferences {
                actual: self.evidence_refs.len(),
                maximum: MAX_MODEL_EVIDENCE_REFS,
            });
        }
        if self.semantic_tags.len() > MAX_MODEL_TAGS {
            return Err(InterpreterError::TooManySemanticTags {
                actual: self.semantic_tags.len(),
                maximum: MAX_MODEL_TAGS,
            });
        }
        if let Some(reference) = self
            .evidence_refs
            .iter()
            .find(|reference| reference.len() > MAX_MODEL_EVIDENCE_REF_BYTES)
        {
            return Err(InterpreterError::ModelFieldTooLarge {
                field: "evidence_refs[]",
                actual: reference.len(),
                maximum: MAX_MODEL_EVIDENCE_REF_BYTES,
            });
        }
        if let Some(tag) = self
            .semantic_tags
            .iter()
            .find(|tag| tag.len() > MAX_MODEL_TAG_BYTES)
        {
            return Err(InterpreterError::ModelFieldTooLarge {
                field: "semantic_tags[]",
                actual: tag.len(),
                maximum: MAX_MODEL_TAG_BYTES,
            });
        }
        Ok(())
    }
}

pub fn parse_model_semantic_proposal(
    response: &[u8],
) -> Result<ModelSemanticProposal, InterpreterError> {
    if response.len() > MAX_MODEL_RESPONSE_BYTES {
        return Err(InterpreterError::ResponseTooLarge {
            actual: response.len(),
            maximum: MAX_MODEL_RESPONSE_BYTES,
        });
    }
    let proposal: ModelSemanticProposal =
        serde_json::from_slice(response).map_err(classify_model_json_error)?;
    proposal.validate()?;
    Ok(proposal)
}

fn classify_model_json_error(error: serde_json::Error) -> InterpreterError {
    let message = error.to_string();
    if message.contains("unknown field") {
        return InterpreterError::UnknownModelField(extract_json_field(&message));
    }
    if message.contains("missing field") {
        return InterpreterError::MissingModelField(extract_json_field(&message));
    }
    InterpreterError::MalformedModelResponse(message)
}

fn extract_json_field(message: &str) -> String {
    message
        .split('`')
        .nth(1)
        .map_or_else(|| "unknown".into(), str::to_owned)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticInterpretationRequest {
    pub prompt_version: String,
    pub user_intent: String,
    pub point_ref: ExternalRef,
    pub scope: SemanticScope,
    pub current_position: Option<Position3>,
    pub retrieved_canon: Vec<RetrievedCanonFragment>,
    pub nearby_examples: Vec<NearbySemanticExample>,
    pub evidence_refs: Vec<ExternalRef>,
}

pub trait SemanticInterpreter: Send + Sync {
    fn propose<'a>(
        &'a self,
        request: &'a SemanticInterpretationRequest,
    ) -> Pin<Box<dyn Future<Output = Result<ModelSemanticProposal, InterpreterError>> + Send + 'a>>;

    fn version(&self) -> &str;
}

#[derive(Debug, Clone)]
pub struct DeterministicSemanticInterpreter {
    response: Result<ModelSemanticProposal, InterpreterError>,
    version: String,
}

impl DeterministicSemanticInterpreter {
    #[must_use]
    pub fn responding_with(proposal: ModelSemanticProposal) -> Self {
        Self {
            response: Ok(proposal),
            version: DETERMINISTIC_INTERPRETER_VERSION.into(),
        }
    }

    #[must_use]
    pub fn failing_with(error: InterpreterError) -> Self {
        Self {
            response: Err(error),
            version: DETERMINISTIC_INTERPRETER_VERSION.into(),
        }
    }

    #[must_use]
    pub fn from_json(response: &[u8]) -> Self {
        match parse_model_semantic_proposal(response) {
            Ok(proposal) => Self::responding_with(proposal),
            Err(error) => Self::failing_with(error),
        }
    }

    #[must_use]
    pub fn timeout() -> Self {
        Self::failing_with(InterpreterError::InterpreterTimeout)
    }

    #[must_use]
    pub fn unavailable() -> Self {
        Self::failing_with(InterpreterError::InterpreterUnavailable)
    }
}

impl SemanticInterpreter for DeterministicSemanticInterpreter {
    fn propose<'a>(
        &'a self,
        _request: &'a SemanticInterpretationRequest,
    ) -> Pin<Box<dyn Future<Output = Result<ModelSemanticProposal, InterpreterError>> + Send + 'a>>
    {
        Box::pin(async move { self.response.clone() })
    }

    fn version(&self) -> &str {
        &self.version
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterpreterError {
    MalformedModelResponse(String),
    UnknownModelField(String),
    MissingModelField(String),
    ResponseTooLarge {
        actual: usize,
        maximum: usize,
    },
    ModelFieldTooLarge {
        field: &'static str,
        actual: usize,
        maximum: usize,
    },
    TooManyEvidenceReferences {
        actual: usize,
        maximum: usize,
    },
    TooManySemanticTags {
        actual: usize,
        maximum: usize,
    },
    ConfidenceOutOfBounds(u16),
    InvalidScale(String),
    InvalidEvidenceReference(String),
    InterpreterUnavailable,
    InterpreterTimeout,
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "semantic interpreter rejected response: {self:?}"
        )
    }
}

impl std::error::Error for InterpreterError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProposalStatus {
    Proposed,
    Gathering,
    Confirmed,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticProposal {
    pub proposal_id: ExternalRef,
    pub cycle_id: GroveCycleId,
    pub actor_ref: ExternalRef,
    pub point_ref: ExternalRef,
    pub scope: SemanticScope,
    pub previous_position: Option<Position3>,
    pub proposed_position: Position3,
    pub translation: Option<Translation3>,
    pub operation: ProposedSemanticOperation,
    pub evidence_refs: Vec<ExternalRef>,
    pub semantic_tags: Vec<String>,
    pub summary: String,
    pub confidence_millis: u16,
    pub status: ProposalStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WayBackContext {
    pub cycle_id: GroveCycleId,
    pub actor_ref: ExternalRef,
    pub point_ref: ExternalRef,
    pub scope: SemanticScope,
    pub current_position: Option<Position3>,
    pub current_record_id: Option<CompositionRecordId>,
    pub prior_state_refs: Vec<ExternalRef>,
    pub evidence_refs: Vec<ExternalRef>,
    pub constitutional_refs: Vec<ExternalRef>,
    pub authority_refs: Vec<ExternalRef>,
    pub unresolved_condition_refs: Vec<ExternalRef>,
    pub world_state_refs: Vec<ExternalRef>,
    pub nearby_semantic_refs: Vec<ExternalRef>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InitiationResult {
    pub proposal: SemanticProposal,
    pub raw_response_ref: Option<ExternalRef>,
    pub prompt_version: String,
    pub interpreter_version: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatheringResult {
    pub proposal: SemanticProposal,
    pub resolved_evidence_refs: Vec<ExternalRef>,
    pub resolved_constitutional_refs: Vec<ExternalRef>,
    pub nearby_position_refs: Vec<ExternalRef>,
    pub calculated_translation: Option<Translation3>,
    pub preflight_errors: Vec<SemanticValidationError>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationReport {
    pub errors: Vec<SemanticValidationError>,
    pub warnings: Vec<SemanticValidationWarning>,
}

impl ValidationReport {
    #[must_use]
    pub const fn valid() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemanticValidationError {
    AxisOutOfBounds,
    DeltaOutOfBounds,
    ArithmeticOverflow,
    ReservedYAxisNonZero,
    ReservedYAxisTranslation,
    TranslationMismatch,
    MalformedCoordinate,
    MalformedModelResponse,
    UnknownModelField,
    MissingModelField,
    ResponseTooLarge,
    UnknownPoint(ExternalRef),
    UnknownScale(ScaleKey),
    UnknownContext(ExternalRef),
    UnknownEvidence(ExternalRef),
    UnknownConstitutionalReference(ExternalRef),
    UnsupportedOperation(ProposedSemanticOperation),
    StaleSourcePosition,
    UnauthorizedActor(ExternalRef),
    InsufficientEvidence,
    ConstitutionalViolation(String),
    CompositionViolation(String),
    ContainmentViolation(String),
    ContainmentCycle,
    SuccessionViolation(String),
    DuplicateStableId,
    CatalogWriteFailure(String),
    WitnessWriteFailure(String),
    ReplayMismatch,
    InterpreterUnavailable,
    InterpreterTimeout,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemanticValidationWarning {
    LowConfidence(u16),
    NoNearbyExamples,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemanticRejectionReason {
    Validation(SemanticValidationError),
    Provenance(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemanticDecision {
    Confirmed(ConfirmedSemanticTransition),
    Rejected(RejectedSemanticProposal),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfirmedSemanticTransition {
    pub proposal_id: ExternalRef,
    pub point_ref: ExternalRef,
    pub scope: SemanticScope,
    pub previous_position: Option<Position3>,
    pub confirmed_position: Position3,
    pub translation: Option<Translation3>,
    pub composition_ref: ExternalRef,
    pub witness_ref: ExternalRef,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RejectedSemanticProposal {
    pub proposal_id: ExternalRef,
    pub point_ref: ExternalRef,
    pub scope: SemanticScope,
    pub proposed_position: Position3,
    pub reasons: Vec<SemanticRejectionReason>,
    pub witness_ref: ExternalRef,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FestivalResult {
    pub decision: SemanticDecision,
    pub next_state_refs: Vec<ExternalRef>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FederationResult {
    pub festival: FestivalResult,
    pub phase_trace: Vec<GrovePhase>,
    pub next_way_back: WayBackContext,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FederationRequest {
    pub actor_ref: ExternalRef,
    pub point_ref: ExternalRef,
    pub scope: SemanticScope,
    pub user_intent: String,
    pub prompt_version: String,
    pub evidence_refs: Vec<ExternalRef>,
    pub constitutional_refs: Vec<ExternalRef>,
    pub authority_refs: Vec<ExternalRef>,
    pub unresolved_condition_refs: Vec<ExternalRef>,
    pub world_state_refs: Vec<ExternalRef>,
    pub nearby_semantic_refs: Vec<ExternalRef>,
    pub retrieved_canon: Vec<RetrievedCanonFragment>,
    pub nearby_examples: Vec<NearbySemanticExample>,
}

pub trait RoiContinuity: Send + Sync {
    fn restore_history(
        &self,
        request: &FederationRequest,
        runtime: &CompositionCatalog,
        cycle_id: GroveCycleId,
    ) -> Result<WayBackContext, FederationError>;

    fn resolve_causality(
        &self,
        point_ref: &ExternalRef,
        scope: &SemanticScope,
        runtime: &CompositionCatalog,
    ) -> Vec<ExternalRef>;

    fn verify_succession(
        &self,
        proposal: &SemanticProposal,
        context: &WayBackContext,
    ) -> Result<(), SemanticValidationError>;

    fn prepare_replay(&self, proposal: &SemanticProposal) -> Result<(), SemanticValidationError>;
}

pub trait RosalinaWorldState: Send + Sync {
    fn inspect_world_state(
        &self,
        request: &FederationRequest,
    ) -> Result<Vec<ExternalRef>, FederationError>;

    fn validate_embodiment(
        &self,
        proposal: &SemanticProposal,
        context: &WayBackContext,
    ) -> ValidationReport;

    /// Prepares the effect and returns its result references. Implementations
    /// must not expose mutations until `persist_world_result` is called.
    fn execute_confirmed_effect(
        &self,
        proposal: &SemanticProposal,
    ) -> Result<Vec<ExternalRef>, FederationError>;

    fn persist_world_result(&self, result_refs: &[ExternalRef]) -> Result<(), FederationError>;
}

pub trait SemanticConstitution: Send + Sync {
    fn validate(&self, proposal: &SemanticProposal, context: &WayBackContext) -> ValidationReport;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CatalogRoiContinuity;

impl RoiContinuity for CatalogRoiContinuity {
    fn restore_history(
        &self,
        request: &FederationRequest,
        runtime: &CompositionCatalog,
        cycle_id: GroveCycleId,
    ) -> Result<WayBackContext, FederationError> {
        if runtime.nodes_for_object(&request.point_ref).is_empty() {
            return Err(FederationError::Validation(
                SemanticValidationError::UnknownPoint(request.point_ref.clone()),
            ));
        }
        let current = runtime.current_semantic_position(&request.point_ref, &request.scope);
        let mut prior_state_refs =
            self.resolve_causality(&request.point_ref, &request.scope, runtime);
        prior_state_refs.sort();
        let mut evidence_refs = request.evidence_refs.clone();
        evidence_refs.sort();
        evidence_refs.dedup();
        let mut constitutional_refs = request.constitutional_refs.clone();
        constitutional_refs.sort();
        constitutional_refs.dedup();
        let mut authority_refs = request.authority_refs.clone();
        authority_refs.sort();
        authority_refs.dedup();
        Ok(WayBackContext {
            cycle_id,
            actor_ref: request.actor_ref.clone(),
            point_ref: request.point_ref.clone(),
            scope: request.scope.clone(),
            current_position: current.map(|record| record.position),
            current_record_id: current.map(|record| record.record_id.clone()),
            prior_state_refs,
            evidence_refs,
            constitutional_refs,
            authority_refs,
            unresolved_condition_refs: request.unresolved_condition_refs.clone(),
            world_state_refs: request.world_state_refs.clone(),
            nearby_semantic_refs: request.nearby_semantic_refs.clone(),
        })
    }

    fn resolve_causality(
        &self,
        point_ref: &ExternalRef,
        scope: &SemanticScope,
        runtime: &CompositionCatalog,
    ) -> Vec<ExternalRef> {
        runtime
            .semantic_position_history(point_ref, scope)
            .into_iter()
            .filter_map(|record| ExternalRef::new("semantic-state", record.record_id.as_str()).ok())
            .collect()
    }

    fn verify_succession(
        &self,
        proposal: &SemanticProposal,
        context: &WayBackContext,
    ) -> Result<(), SemanticValidationError> {
        if proposal.previous_position != context.current_position {
            return Err(SemanticValidationError::StaleSourcePosition);
        }
        Ok(())
    }

    fn prepare_replay(&self, _proposal: &SemanticProposal) -> Result<(), SemanticValidationError> {
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ReadOnlyRosalinaWorldState;

impl RosalinaWorldState for ReadOnlyRosalinaWorldState {
    fn inspect_world_state(
        &self,
        request: &FederationRequest,
    ) -> Result<Vec<ExternalRef>, FederationError> {
        Ok(request.world_state_refs.clone())
    }

    fn validate_embodiment(
        &self,
        _proposal: &SemanticProposal,
        _context: &WayBackContext,
    ) -> ValidationReport {
        ValidationReport::valid()
    }

    fn execute_confirmed_effect(
        &self,
        _proposal: &SemanticProposal,
    ) -> Result<Vec<ExternalRef>, FederationError> {
        Ok(Vec::new())
    }

    fn persist_world_result(&self, _result_refs: &[ExternalRef]) -> Result<(), FederationError> {
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct KernelSemanticConstitution;

impl SemanticConstitution for KernelSemanticConstitution {
    fn validate(&self, proposal: &SemanticProposal, context: &WayBackContext) -> ValidationReport {
        let mut report = ValidationReport::valid();
        if context.authority_refs.is_empty() {
            report
                .errors
                .push(SemanticValidationError::UnauthorizedActor(
                    proposal.actor_ref.clone(),
                ));
        }
        if context.constitutional_refs.is_empty() {
            report
                .errors
                .push(SemanticValidationError::UnknownConstitutionalReference(
                    ExternalRef::new("constitution", "semantic-kernel-v1")
                        .expect("static reference is valid"),
                ));
        }
        if proposal.confidence_millis < 500 {
            report
                .warnings
                .push(SemanticValidationWarning::LowConfidence(
                    proposal.confidence_millis,
                ));
        }
        report
    }
}

pub struct FederationOrchestrator<R, S, I, C> {
    roi: R,
    rosalina: S,
    interpreter: I,
    constitution: C,
}

impl<R, S, I, C> FederationOrchestrator<R, S, I, C> {
    #[must_use]
    pub const fn new(roi: R, rosalina: S, interpreter: I, constitution: C) -> Self {
        Self {
            roi,
            rosalina,
            interpreter,
            constitution,
        }
    }
}

impl<R, S, I, C> FederationOrchestrator<R, S, I, C>
where
    R: RoiContinuity,
    S: RosalinaWorldState,
    I: SemanticInterpreter,
    C: SemanticConstitution,
{
    pub async fn run_cycle(
        &self,
        request: FederationRequest,
        runtime: &mut CompositionCatalog,
    ) -> Result<FederationResult, FederationError> {
        let mut staged_runtime = runtime.clone();
        let cycle_id = deterministic_cycle_id(&request, &staged_runtime)?;
        let mut phase_trace = vec![GrovePhase::TheWayBack];
        let way_back = self.run_way_back(&request, &staged_runtime, cycle_id.clone())?;

        phase_trace.push(GrovePhase::TheInitiation);
        let initiation = self.run_initiation(&request, &way_back).await?;

        phase_trace.push(GrovePhase::TheGathering);
        let gathering = self.run_gathering(&way_back, initiation, &staged_runtime)?;

        phase_trace.push(GrovePhase::TheFestival);
        let (festival, prepared_world_result_refs) =
            self.run_festival(&request, &way_back, gathering, &mut staged_runtime)?;
        let next_way_back = self
            .roi
            .restore_history(&request, &staged_runtime, cycle_id)?;
        self.rosalina
            .persist_world_result(&prepared_world_result_refs)?;
        *runtime = staged_runtime;

        Ok(FederationResult {
            festival,
            phase_trace,
            next_way_back,
        })
    }

    fn run_way_back(
        &self,
        request: &FederationRequest,
        runtime: &CompositionCatalog,
        cycle_id: GroveCycleId,
    ) -> Result<WayBackContext, FederationError> {
        let mut context = self.roi.restore_history(request, runtime, cycle_id)?;
        context.world_state_refs = self.rosalina.inspect_world_state(request)?;
        Ok(context)
    }

    async fn run_initiation(
        &self,
        request: &FederationRequest,
        way_back: &WayBackContext,
    ) -> Result<InitiationResult, FederationError> {
        let interpretation = SemanticInterpretationRequest {
            prompt_version: request.prompt_version.clone(),
            user_intent: request.user_intent.clone(),
            point_ref: request.point_ref.clone(),
            scope: request.scope.clone(),
            current_position: way_back.current_position,
            retrieved_canon: request.retrieved_canon.clone(),
            nearby_examples: request.nearby_examples.clone(),
            evidence_refs: way_back.evidence_refs.clone(),
        };
        let model = self
            .interpreter
            .propose(&interpretation)
            .await
            .map_err(FederationError::Interpreter)?;
        model.validate().map_err(FederationError::Interpreter)?;
        if model.scale_key != request.scope.scale.as_str() {
            return Err(FederationError::Validation(
                SemanticValidationError::UnknownScale(
                    ScaleKey::new(model.scale_key)
                        .map_err(|error| FederationError::StableKey(error.to_string()))?,
                ),
            ));
        }
        let evidence_refs = model
            .evidence_refs
            .iter()
            .map(|claim| parse_external_claim(claim))
            .collect::<Result<Vec<_>, _>>()
            .map_err(FederationError::Interpreter)?;
        let translation = way_back
            .current_position
            .map(|current| current.translation_to(model.proposed_position))
            .transpose()
            .map_err(|error| FederationError::Coordinate(error.to_string()))?;
        let proposal_id = deterministic_proposal_ref(
            &way_back.cycle_id,
            &request.point_ref,
            &request.scope,
            model.proposed_position,
        )?;
        Ok(InitiationResult {
            proposal: SemanticProposal {
                proposal_id,
                cycle_id: way_back.cycle_id.clone(),
                actor_ref: request.actor_ref.clone(),
                point_ref: request.point_ref.clone(),
                scope: request.scope.clone(),
                previous_position: way_back.current_position,
                proposed_position: model.proposed_position,
                translation,
                operation: model.operation,
                evidence_refs,
                semantic_tags: model.semantic_tags,
                summary: model.summary,
                confidence_millis: model.confidence_millis,
                status: ProposalStatus::Proposed,
            },
            raw_response_ref: None,
            prompt_version: request.prompt_version.clone(),
            interpreter_version: self.interpreter.version().into(),
        })
    }

    fn run_gathering(
        &self,
        way_back: &WayBackContext,
        mut initiation: InitiationResult,
        runtime: &CompositionCatalog,
    ) -> Result<GatheringResult, FederationError> {
        initiation.proposal.status = ProposalStatus::Gathering;
        let mut preflight_errors = Vec::new();
        let mut resolved_evidence_refs = Vec::new();
        for claimed in &initiation.proposal.evidence_refs {
            if way_back.evidence_refs.contains(claimed) {
                resolved_evidence_refs.push(claimed.clone());
            } else {
                preflight_errors.push(SemanticValidationError::UnknownEvidence(claimed.clone()));
            }
        }
        if resolved_evidence_refs.is_empty() {
            preflight_errors.push(SemanticValidationError::InsufficientEvidence);
        }
        if runtime
            .nodes_for_object(&initiation.proposal.point_ref)
            .is_empty()
        {
            preflight_errors.push(SemanticValidationError::UnknownPoint(
                initiation.proposal.point_ref.clone(),
            ));
        }
        if let Err(error) = self.roi.verify_succession(&initiation.proposal, way_back) {
            preflight_errors.push(error);
        }
        if let Err(error) = self.roi.prepare_replay(&initiation.proposal) {
            preflight_errors.push(error);
        }
        let constitution = self.constitution.validate(&initiation.proposal, way_back);
        preflight_errors.extend(constitution.errors);
        let embodiment = self
            .rosalina
            .validate_embodiment(&initiation.proposal, way_back);
        preflight_errors.extend(embodiment.errors);

        Ok(GatheringResult {
            calculated_translation: initiation.proposal.translation,
            proposal: initiation.proposal,
            resolved_evidence_refs,
            resolved_constitutional_refs: way_back.constitutional_refs.clone(),
            nearby_position_refs: way_back.nearby_semantic_refs.clone(),
            preflight_errors,
        })
    }

    fn run_festival(
        &self,
        request: &FederationRequest,
        way_back: &WayBackContext,
        mut gathering: GatheringResult,
        runtime: &mut CompositionCatalog,
    ) -> Result<(FestivalResult, Vec<ExternalRef>), FederationError> {
        let current = runtime
            .current_semantic_position(&gathering.proposal.point_ref, &gathering.proposal.scope);
        let current_position = current.map(|record| record.position);
        let current_record_id = current.map(|record| record.record_id.clone());
        let mut errors = std::mem::take(&mut gathering.preflight_errors);
        if current_position != way_back.current_position
            || current_record_id != way_back.current_record_id
        {
            errors.push(SemanticValidationError::StaleSourcePosition);
        }

        let record_id = deterministic_record_id(&gathering.proposal)?;
        let witness_ref = ExternalRef::new(
            "witness",
            format!("semantic.{}", stable_suffix(&gathering.proposal)),
        )
        .map_err(|error| FederationError::ExternalReference(error.to_string()))?;
        let cycle_ref = ExternalRef::new("cycle", gathering.proposal.cycle_id.as_str())
            .map_err(|error| FederationError::ExternalReference(error.to_string()))?;

        if !errors.is_empty() {
            let reasons = errors
                .iter()
                .cloned()
                .map(SemanticRejectionReason::Validation)
                .collect::<Vec<_>>();
            let witness = build_rejected_witness(
                request,
                &gathering,
                witness_ref.clone(),
                cycle_ref,
                &reasons,
                self.interpreter.version(),
            );
            record_rejected_semantic_witness(runtime, witness)
                .map_err(|error| FederationError::Provenance(Box::new(error)))?;
            return Ok((
                FestivalResult {
                    decision: SemanticDecision::Rejected(RejectedSemanticProposal {
                        proposal_id: gathering.proposal.proposal_id,
                        point_ref: gathering.proposal.point_ref,
                        scope: gathering.proposal.scope,
                        proposed_position: gathering.proposal.proposed_position,
                        reasons,
                        witness_ref: witness_ref.clone(),
                    }),
                    next_state_refs: vec![witness_ref],
                },
                Vec::new(),
            ));
        }

        let operation = match (
            gathering.proposal.previous_position,
            gathering.calculated_translation,
        ) {
            (None, None) => SemanticOperation::PositionAssertion {
                scope: gathering.proposal.scope.clone(),
                position: gathering.proposal.proposed_position,
            },
            (Some(from), Some(translation)) => SemanticOperation::SpatialTranslation {
                scope: gathering.proposal.scope.clone(),
                from,
                translation,
                to: gathering.proposal.proposed_position,
            },
            _ => {
                return Err(FederationError::Validation(
                    SemanticValidationError::TranslationMismatch,
                ));
            }
        };
        let position_record = SemanticPositionRecord {
            record_id: record_id.clone(),
            point_ref: gathering.proposal.point_ref.clone(),
            scope: gathering.proposal.scope.clone(),
            position: gathering.proposal.proposed_position,
            operation,
            previous_record_id: way_back.current_record_id.clone(),
            evidence_refs: gathering.resolved_evidence_refs.clone(),
            constitutional_refs: gathering.resolved_constitutional_refs.clone(),
            witness_ref: witness_ref.clone(),
            cycle_ref: cycle_ref.clone(),
        };
        let composition_ref = ExternalRef::new("composition-record", record_id.as_str())
            .map_err(|error| FederationError::ExternalReference(error.to_string()))?;
        let witness = build_confirmed_witness(
            request,
            &gathering,
            witness_ref.clone(),
            cycle_ref,
            composition_ref.clone(),
            self.interpreter.version(),
        );

        let world_refs = self
            .rosalina
            .execute_confirmed_effect(&gathering.proposal)?;
        confirm_semantic_transition(runtime, position_record, witness)
            .map_err(|error| FederationError::Provenance(Box::new(error)))?;
        let state_ref = ExternalRef::new("semantic-state", record_id.as_str())
            .map_err(|error| FederationError::ExternalReference(error.to_string()))?;
        let mut next_state_refs = vec![state_ref, composition_ref.clone(), witness_ref.clone()];
        next_state_refs.extend(world_refs.iter().cloned());
        Ok((
            FestivalResult {
                decision: SemanticDecision::Confirmed(ConfirmedSemanticTransition {
                    proposal_id: gathering.proposal.proposal_id,
                    point_ref: gathering.proposal.point_ref,
                    scope: gathering.proposal.scope,
                    previous_position: gathering.proposal.previous_position,
                    confirmed_position: gathering.proposal.proposed_position,
                    translation: gathering.proposal.translation,
                    composition_ref,
                    witness_ref,
                }),
                next_state_refs,
            },
            world_refs,
        ))
    }
}

fn build_confirmed_witness(
    request: &FederationRequest,
    gathering: &GatheringResult,
    witness_ref: ExternalRef,
    cycle_ref: ExternalRef,
    composition_ref: ExternalRef,
    interpreter_version: &str,
) -> SemanticWitnessRecord {
    SemanticWitnessRecord {
        witness_ref,
        proposal_ref: gathering.proposal.proposal_id.clone(),
        cycle_ref,
        actor_ref: gathering.proposal.actor_ref.clone(),
        point_ref: gathering.proposal.point_ref.clone(),
        scope: gathering.proposal.scope.clone(),
        input: request.user_intent.clone(),
        prior_position: gathering.proposal.previous_position,
        proposed_position: gathering.proposal.proposed_position,
        translation: gathering.proposal.translation,
        confirmed_position: Some(gathering.proposal.proposed_position),
        operation: ExternalRef::new("semantic-intent", gathering.proposal.operation.as_str())
            .expect("static semantic operation is valid"),
        decision: SemanticWitnessDecision::Confirmed,
        evidence_refs: gathering.resolved_evidence_refs.clone(),
        constitutional_refs: gathering.resolved_constitutional_refs.clone(),
        composition_ref: Some(composition_ref),
        rejection_reasons: Vec::new(),
        semantic_tags: gathering.proposal.semantic_tags.clone(),
        summary: gathering.proposal.summary.clone(),
        prompt_version: request.prompt_version.clone(),
        interpreter_version: interpreter_version.into(),
    }
}

fn build_rejected_witness(
    request: &FederationRequest,
    gathering: &GatheringResult,
    witness_ref: ExternalRef,
    cycle_ref: ExternalRef,
    reasons: &[SemanticRejectionReason],
    interpreter_version: &str,
) -> SemanticWitnessRecord {
    SemanticWitnessRecord {
        witness_ref,
        proposal_ref: gathering.proposal.proposal_id.clone(),
        cycle_ref,
        actor_ref: gathering.proposal.actor_ref.clone(),
        point_ref: gathering.proposal.point_ref.clone(),
        scope: gathering.proposal.scope.clone(),
        input: request.user_intent.clone(),
        prior_position: gathering.proposal.previous_position,
        proposed_position: gathering.proposal.proposed_position,
        translation: gathering.proposal.translation,
        confirmed_position: None,
        operation: ExternalRef::new("semantic-intent", gathering.proposal.operation.as_str())
            .expect("static semantic operation is valid"),
        decision: SemanticWitnessDecision::Rejected,
        evidence_refs: gathering.proposal.evidence_refs.clone(),
        constitutional_refs: gathering.resolved_constitutional_refs.clone(),
        composition_ref: None,
        rejection_reasons: reasons.iter().map(|reason| format!("{reason:?}")).collect(),
        semantic_tags: gathering.proposal.semantic_tags.clone(),
        summary: gathering.proposal.summary.clone(),
        prompt_version: request.prompt_version.clone(),
        interpreter_version: interpreter_version.into(),
    }
}

fn deterministic_cycle_id(
    request: &FederationRequest,
    runtime: &CompositionCatalog,
) -> Result<GroveCycleId, FederationError> {
    let current = runtime.current_semantic_position(&request.point_ref, &request.scope);
    let context = request
        .scope
        .context_ref
        .as_ref()
        .map_or_else(|| "-".into(), external_label);
    let current_record = current
        .map(|record| record.record_id.as_str())
        .unwrap_or("-");
    let seed = format!(
        "{}\u{1f}{}\u{1f}{}\u{1f}{}\u{1f}{}\u{1f}{}",
        external_label(&request.actor_ref),
        external_label(&request.point_ref),
        request.scope.scale.as_str(),
        context,
        request.user_intent,
        current_record,
    );
    GroveCycleId::new(format!("cycle.semantic.{:016x}", fnv1a64(seed.as_bytes())))
        .map_err(|error| FederationError::StableKey(error.to_string()))
}

fn deterministic_proposal_ref(
    cycle_id: &GroveCycleId,
    point_ref: &ExternalRef,
    scope: &SemanticScope,
    position: Position3,
) -> Result<ExternalRef, FederationError> {
    let seed = format!(
        "{}\u{1f}{}\u{1f}{}\u{1f}{},{},{}",
        cycle_id.as_str(),
        external_label(point_ref),
        scope.scale.as_str(),
        position.x().get(),
        position.y().get(),
        position.z().get(),
    );
    ExternalRef::new(
        "proposal",
        format!("semantic.{:016x}", fnv1a64(seed.as_bytes())),
    )
    .map_err(|error| FederationError::ExternalReference(error.to_string()))
}

fn deterministic_record_id(
    proposal: &SemanticProposal,
) -> Result<CompositionRecordId, FederationError> {
    CompositionRecordId::new(format!("record.semantic.{}", stable_suffix(proposal)))
        .map_err(|error| FederationError::StableKey(error.to_string()))
}

fn stable_suffix(proposal: &SemanticProposal) -> String {
    let seed = format!(
        "{}\u{1f}{}\u{1f}{}\u{1f}{},{},{}",
        external_label(&proposal.proposal_id),
        external_label(&proposal.point_ref),
        proposal.scope.scale.as_str(),
        proposal.proposed_position.x().get(),
        proposal.proposed_position.y().get(),
        proposal.proposed_position.z().get(),
    );
    format!("{:016x}", fnv1a64(seed.as_bytes()))
}

fn parse_external_claim(claim: &str) -> Result<ExternalRef, InterpreterError> {
    let Some((namespace, key)) = claim.split_once('.') else {
        return Err(InterpreterError::InvalidEvidenceReference(claim.into()));
    };
    ExternalRef::new(namespace, key)
        .map_err(|_| InterpreterError::InvalidEvidenceReference(claim.into()))
}

fn external_label(reference: &ExternalRef) -> String {
    format!("{}.{}", reference.namespace, reference.key)
}

fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

#[derive(Debug)]
pub enum FederationError {
    Interpreter(InterpreterError),
    Validation(SemanticValidationError),
    Provenance(Box<SemanticProvenanceError>),
    StableKey(String),
    ExternalReference(String),
    Coordinate(String),
    WorldState(String),
}

impl fmt::Display for FederationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "semantic Federation cycle failed: {self:?}")
    }
}

impl std::error::Error for FederationError {}
