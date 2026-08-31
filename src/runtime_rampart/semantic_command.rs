//! Authoritative local entry point for a complete transformer-assisted cycle.
//!
//! This layer prepares resolved local-operator evidence and bounded canon
//! retrieval, then delegates every state decision to the existing Rampart
//! orchestrator. It does not define another catalog or confirmation path.

use std::fmt;

use crate::composition::{CompositionCatalog, ExternalRef, ScaleKey, SemanticScope};
use crate::guide_identity::{DualGuideSession, StableGuideSessionId};
use crate::semantic::{
    CanonRetriever, CanonStatus, CatalogCanonRetriever, RetrievalError, RetrievalRequest,
    RetrievedCanonFragment,
};

use super::{
    CatalogRoiContinuity, HOLLOW_GROVE_SEMANTIC_PROMPT_VERSION, KernelSemanticConstitution,
    RampartError, RampartOrchestrator, RampartRequest, RampartResult, ReadOnlyRoselinaWorldState,
    SemanticInterpreter,
};

pub const LOCAL_RAMPART_ACTOR_REF: &str = "actor.local-operator";
pub const LOCAL_RAMPART_AUTHORITY_REF: &str = "authority.local-operator.semantic-rampart-v1";
pub const LOCAL_RAMPART_GUIDE_SESSION_ID: &str = "session.semantic-rampart-cli";
pub const SEMANTIC_KERNEL_CONSTITUTION_REF: &str = "constitution.semantic-kernel-v1";
pub const MAX_LOCAL_RAMPART_INTENT_BYTES: usize = 16_384;

const SEMANTIC_KERNEL_V1: &str = include_str!("../../docs/SEMANTIC_KERNEL_V1.md");

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalRampartCycleInput {
    pub point_ref: ExternalRef,
    pub scale: ScaleKey,
    pub context_ref: Option<ExternalRef>,
    pub user_intent: String,
}

impl LocalRampartCycleInput {
    pub fn new(
        point_ref: ExternalRef,
        scale: ScaleKey,
        user_intent: impl Into<String>,
    ) -> Result<Self, LocalRampartCycleError> {
        let user_intent = user_intent.into();
        if user_intent.trim().is_empty() || user_intent.len() > MAX_LOCAL_RAMPART_INTENT_BYTES {
            return Err(LocalRampartCycleError::InvalidIntent {
                actual_bytes: user_intent.len(),
                maximum_bytes: MAX_LOCAL_RAMPART_INTENT_BYTES,
            });
        }
        Ok(Self {
            point_ref,
            scale,
            context_ref: None,
            user_intent,
        })
    }

    #[must_use]
    pub fn scope(&self) -> SemanticScope {
        SemanticScope::new(self.scale.clone(), self.context_ref.clone())
    }

    pub fn evidence_ref(&self) -> Result<ExternalRef, LocalRampartCycleError> {
        let context = self
            .context_ref
            .as_ref()
            .map_or_else(|| "-".into(), external_label);
        let seed = format!(
            "{}\u{1f}{}\u{1f}{}\u{1f}{}",
            external_label(&self.point_ref),
            self.scale.as_str(),
            context,
            self.user_intent,
        );
        reference(
            "evidence",
            format!("semantic-request.{:016x}", fnv1a64(seed.as_bytes())),
        )
    }
}

pub async fn run_local_rampart_cycle<I>(
    interpreter: I,
    input: &LocalRampartCycleInput,
    catalog: &mut CompositionCatalog,
) -> Result<RampartResult, LocalRampartCycleError>
where
    I: SemanticInterpreter,
{
    let scope = input.scope();
    let constitutional_ref = parse_reference(SEMANTIC_KERNEL_CONSTITUTION_REF)?;
    let evidence_ref = input.evidence_ref()?;
    let current_position = catalog
        .current_semantic_position(&input.point_ref, &scope)
        .map(|record| record.position);
    let retrieval_request = RetrievalRequest {
        point_ref: input.point_ref.clone(),
        scope: scope.clone(),
        coordinate: current_position,
        constitutional_refs: vec![constitutional_ref.clone()],
        maximum_canon_fragments: 8,
        maximum_nearby_examples: 8,
        semantic_tags: Vec::new(),
    };
    let retriever = CatalogCanonRetriever::new(
        catalog,
        [semantic_kernel_fragment(constitutional_ref.clone())],
    )
    .map_err(LocalRampartCycleError::Retrieval)?;
    let retrieval = retriever
        .retrieve(&retrieval_request)
        .map_err(LocalRampartCycleError::Retrieval)?;
    let nearby_semantic_refs = retrieval
        .nearby_examples
        .iter()
        .map(|example| example.record_ref.clone())
        .collect();
    let guide_session = DualGuideSession::outside(
        StableGuideSessionId::new(LOCAL_RAMPART_GUIDE_SESSION_ID)
            .map_err(|error| LocalRampartCycleError::GuideSession(error.to_string()))?,
    );
    let request = RampartRequest {
        actor_ref: parse_reference(LOCAL_RAMPART_ACTOR_REF)?,
        point_ref: input.point_ref.clone(),
        scope,
        user_intent: input.user_intent.clone(),
        prompt_version: HOLLOW_GROVE_SEMANTIC_PROMPT_VERSION.into(),
        evidence_refs: vec![evidence_ref],
        constitutional_refs: vec![constitutional_ref],
        authority_refs: vec![parse_reference(LOCAL_RAMPART_AUTHORITY_REF)?],
        unresolved_condition_refs: Vec::new(),
        world_state_refs: Vec::new(),
        nearby_semantic_refs,
        retrieved_canon: retrieval.canon,
        nearby_examples: retrieval.nearby_examples,
        guide_session,
    };
    RampartOrchestrator::new(
        CatalogRoiContinuity,
        ReadOnlyRoselinaWorldState,
        interpreter,
        KernelSemanticConstitution,
    )
    .run_cycle(request, catalog)
    .await
    .map_err(LocalRampartCycleError::Rampart)
}

fn semantic_kernel_fragment(reference: ExternalRef) -> RetrievedCanonFragment {
    RetrievedCanonFragment {
        reference,
        source_document: "docs/SEMANTIC_KERNEL_V1.md".into(),
        section: "Hollow Grove Semantic Kernel V1".into(),
        status: CanonStatus::Constitutional,
        version: Some("v1".into()),
        content: SEMANTIC_KERNEL_V1.into(),
    }
}

fn parse_reference(value: &str) -> Result<ExternalRef, LocalRampartCycleError> {
    let Some((namespace, key)) = value.split_once('.') else {
        return Err(LocalRampartCycleError::ExternalReference(value.into()));
    };
    reference(namespace, key)
}

fn reference(
    namespace: impl Into<String>,
    key: impl Into<String>,
) -> Result<ExternalRef, LocalRampartCycleError> {
    ExternalRef::new(namespace, key)
        .map_err(|error| LocalRampartCycleError::ExternalReference(error.to_string()))
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
pub enum LocalRampartCycleError {
    InvalidIntent {
        actual_bytes: usize,
        maximum_bytes: usize,
    },
    ExternalReference(String),
    GuideSession(String),
    Retrieval(RetrievalError),
    Rampart(RampartError),
}

impl fmt::Display for LocalRampartCycleError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "local semantic Rampart cycle failed: {self:?}")
    }
}

impl std::error::Error for LocalRampartCycleError {}
