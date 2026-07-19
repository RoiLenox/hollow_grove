use crate::aura_polarity::AuraPolarityEvaluation;
use crate::composition::{
    CompositionNode, CompositionNodeId, CompositionRecord, CompositionRecordId, ExternalRef,
    ExternalRefError, ScaleKey,
};
use crate::decision_engine::DecisionTrace;
use crate::kernel_pass::KernelPass;
use crate::synthesis_execution::SynthesisExecution;

use super::{
    ArtifactId, BondId, CausalPosition, ConstitutionalRuntime, ConstitutionalRuntimeError,
    EvidenceRef, Toke, Tombstone, WaveId, WaveRecord,
};

fn evidence(namespace: &str, artifact: &ArtifactId) -> Result<EvidenceRef, ExternalRefError> {
    ExternalRef::new(namespace, artifact.as_str()).map(EvidenceRef)
}

/// Adapts a completed recursion-kernel witness into an opaque constitutional
/// evidence reference. The runtime never parses or changes the pass.
pub fn kernel_pass_evidence(
    artifact: &ArtifactId,
    pass: &KernelPass,
) -> Result<EvidenceRef, ExternalRefError> {
    let checksum = fnv1a64(pass.canonical_witness().as_bytes());
    ExternalRef::new(
        "kernel-pass",
        format!("{}:fnv1a64-v1:{checksum:016x}", artifact.as_str()),
    )
    .map(EvidenceRef)
}

#[derive(Debug)]
pub enum KernelConstitutionalBoundaryError {
    Evidence(ExternalRefError),
    Runtime(ConstitutionalRuntimeError),
}

impl From<ExternalRefError> for KernelConstitutionalBoundaryError {
    fn from(value: ExternalRefError) -> Self {
        Self::Evidence(value)
    }
}

impl From<ConstitutionalRuntimeError> for KernelConstitutionalBoundaryError {
    fn from(value: ConstitutionalRuntimeError) -> Self {
        Self::Runtime(value)
    }
}

impl std::fmt::Display for KernelConstitutionalBoundaryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "kernel/constitutional boundary error: {self:?}")
    }
}

impl std::error::Error for KernelConstitutionalBoundaryError {}

/// Records a completed kernel pass as the causal Wave that constitutional
/// commands may later reference. No Current moves merely because it is
/// recorded; formation and activation remain separate stages.
pub fn record_kernel_wave(
    runtime: &mut ConstitutionalRuntime,
    wave: WaveId,
    artifact: &ArtifactId,
    causal_position: CausalPosition,
    pass: &KernelPass,
) -> Result<(), KernelConstitutionalBoundaryError> {
    runtime.record_wave(WaveRecord {
        id: wave,
        origin: kernel_pass_evidence(artifact, pass)?,
        causal_position,
    })?;
    Ok(())
}

/// Adapts repository-owned recipe execution evidence without interpreting it
/// as Current. Quantification remains an explicit constitutional input.
pub fn synthesis_execution_evidence(
    artifact: &ArtifactId,
    _execution: &SynthesisExecution,
) -> Result<EvidenceRef, ExternalRefError> {
    evidence("synthesis-execution", artifact)
}

/// Adapts the existing Aura-orientation evaluation as evidence. It does not
/// convert descriptive orientation into signed Aura automatically.
pub fn aura_evaluation_evidence(
    artifact: &ArtifactId,
    _evaluation: &AuraPolarityEvaluation,
) -> Result<EvidenceRef, ExternalRefError> {
    evidence("aura-evaluation", artifact)
}

/// Adapts deterministic decision replay as evidence only. Decision output may
/// justify a command but cannot append constitutional history by itself.
pub fn decision_trace_evidence(
    artifact: &ArtifactId,
    _trace: &DecisionTrace,
) -> Result<EvidenceRef, ExternalRefError> {
    evidence("decision-trace", artifact)
}

#[derive(Debug, PartialEq, Eq)]
pub struct ConstitutionalCompositionProjection {
    pub nodes: Vec<CompositionNode>,
    pub records: Vec<CompositionRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstitutionalCompositionIds {
    pub bond_node: CompositionNodeId,
    pub tombstone_node: CompositionNodeId,
    pub toke_node: CompositionNodeId,
    pub condensation_record: CompositionRecordId,
    pub recording_record: CompositionRecordId,
    pub scale: ScaleKey,
}

/// Produces neutral composition records for already-proven constitutional
/// history. This is an indexing projection, never the authoritative store.
pub fn constitutional_composition_projection(
    bond: &BondId,
    tombstone: &Tombstone,
    toke: &Toke,
    ids: ConstitutionalCompositionIds,
) -> Result<ConstitutionalCompositionProjection, ExternalRefError> {
    let ConstitutionalCompositionIds {
        bond_node,
        tombstone_node,
        toke_node,
        condensation_record,
        recording_record,
        scale,
    } = ids;
    let nodes = vec![
        CompositionNode {
            id: bond_node.clone(),
            object: ExternalRef::new("constitutional-bond", bond.as_str())?,
            scale: scale.clone(),
        },
        CompositionNode {
            id: tombstone_node.clone(),
            object: ExternalRef::new("constitutional-tombstone", tombstone.id.as_str())?,
            scale: scale.clone(),
        },
        CompositionNode {
            id: toke_node.clone(),
            object: ExternalRef::new("constitutional-toke", toke.id.as_str())?,
            scale,
        },
    ];
    let records = vec![
        CompositionRecord {
            id: condensation_record,
            sources: vec![bond_node],
            result: tombstone_node.clone(),
            operation: ExternalRef::new("constitutional-operation", "condensation-v1")?,
            evidence: tombstone.evidence.first().map(|entry| entry.0.clone()),
        },
        CompositionRecord {
            id: recording_record,
            sources: vec![tombstone_node],
            result: toke_node,
            operation: ExternalRef::new("constitutional-operation", "toke-recording-v1")?,
            evidence: toke.evidence.first().map(|entry| entry.0.clone()),
        },
    ];
    Ok(ConstitutionalCompositionProjection { nodes, records })
}

fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}
