//! Deterministic JSONL projection of confirmed and rejected semantic witnesses.

use std::fmt;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::artifact_io::write_bytes_atomically;
use crate::composition::{
    CompositionCatalog, ExternalRef, Position3, SEMANTIC_WITNESS_SCHEMA_VERSION, Translation3,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticDatasetScope {
    pub scale_key: String,
    pub context_ref: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticDatasetDecision {
    Confirmed,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticTrainingCategory {
    PositionAssertion,
    Translation,
    ConstitutionalRejection,
    EvidenceRejection,
    StaleStateRejection,
    OtherRejection,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticWitnessDatasetRecord {
    pub schema_version: String,
    pub witness_ref: String,
    pub proposal_ref: String,
    pub cycle_id: String,
    pub input: String,
    pub scope: SemanticDatasetScope,
    pub prior_position: Option<Position3>,
    pub proposed_position: Position3,
    pub translation: Option<Translation3>,
    pub confirmed_position: Option<Position3>,
    pub operation: String,
    pub decision: SemanticDatasetDecision,
    pub rejection_reasons: Vec<String>,
    pub evidence_refs: Vec<String>,
    pub constitutional_refs: Vec<String>,
    pub semantic_tags: Vec<String>,
    pub training_categories: Vec<SemanticTrainingCategory>,
    pub summary: String,
    pub prompt_version: String,
    pub interpreter_version: String,
}

pub fn export_semantic_witness_jsonl(
    catalog: &CompositionCatalog,
) -> Result<Vec<u8>, SemanticDatasetError> {
    let snapshot = catalog.semantic_witness_snapshot().canonicalized();
    let mut output = Vec::new();
    for witness in snapshot.witnesses {
        let decision = match witness.decision {
            crate::composition::SemanticWitnessDecision::Confirmed => {
                SemanticDatasetDecision::Confirmed
            }
            crate::composition::SemanticWitnessDecision::Rejected => {
                SemanticDatasetDecision::Rejected
            }
        };
        let training_categories =
            training_categories(decision, witness.translation, &witness.rejection_reasons);
        let record = SemanticWitnessDatasetRecord {
            schema_version: SEMANTIC_WITNESS_SCHEMA_VERSION.into(),
            witness_ref: external_label(&witness.witness_ref),
            proposal_ref: external_label(&witness.proposal_ref),
            cycle_id: external_label(&witness.cycle_ref),
            input: witness.input,
            scope: SemanticDatasetScope {
                scale_key: witness.scope.scale.as_str().into(),
                context_ref: witness.scope.context_ref.as_ref().map(external_label),
            },
            prior_position: witness.prior_position,
            proposed_position: witness.proposed_position,
            translation: witness.translation,
            confirmed_position: witness.confirmed_position,
            operation: witness.operation.key,
            decision,
            rejection_reasons: witness.rejection_reasons,
            evidence_refs: witness.evidence_refs.iter().map(external_label).collect(),
            constitutional_refs: witness
                .constitutional_refs
                .iter()
                .map(external_label)
                .collect(),
            semantic_tags: witness.semantic_tags,
            training_categories,
            summary: witness.summary,
            prompt_version: witness.prompt_version,
            interpreter_version: witness.interpreter_version,
        };
        output.extend(
            serde_json::to_vec(&record)
                .map_err(|error| SemanticDatasetError::Json(error.to_string()))?,
        );
        output.push(b'\n');
    }
    validate_semantic_witness_jsonl(&output)?;
    Ok(output)
}

pub fn write_semantic_witness_dataset(
    path: &Path,
    catalog: &CompositionCatalog,
) -> Result<(), SemanticDatasetError> {
    let bytes = export_semantic_witness_jsonl(catalog)?;
    write_bytes_atomically(path, &bytes)
        .map_err(|error| SemanticDatasetError::Write(error.to_string()))
}

pub fn validate_semantic_witness_jsonl(bytes: &[u8]) -> Result<usize, SemanticDatasetError> {
    if !bytes.is_empty() && !bytes.ends_with(b"\n") {
        return Err(SemanticDatasetError::MissingFinalNewline);
    }
    let text = std::str::from_utf8(bytes)
        .map_err(|error| SemanticDatasetError::Utf8(error.to_string()))?;
    let mut count = 0;
    for line in text.lines() {
        if line.trim().is_empty() {
            return Err(SemanticDatasetError::EmptyLine(count + 1));
        }
        let record: SemanticWitnessDatasetRecord = serde_json::from_str(line)
            .map_err(|error| SemanticDatasetError::Json(error.to_string()))?;
        if record.schema_version != SEMANTIC_WITNESS_SCHEMA_VERSION {
            return Err(SemanticDatasetError::UnsupportedSchema(
                record.schema_version,
            ));
        }
        count += 1;
    }
    Ok(count)
}

fn training_categories(
    decision: SemanticDatasetDecision,
    translation: Option<Translation3>,
    rejection_reasons: &[String],
) -> Vec<SemanticTrainingCategory> {
    if decision == SemanticDatasetDecision::Confirmed {
        return vec![if translation.is_some() {
            SemanticTrainingCategory::Translation
        } else {
            SemanticTrainingCategory::PositionAssertion
        }];
    }
    let reason = rejection_reasons.join(" ").to_ascii_lowercase();
    let category = if reason.contains("stalesourceposition") || reason.contains("stale") {
        SemanticTrainingCategory::StaleStateRejection
    } else if reason.contains("evidence") {
        SemanticTrainingCategory::EvidenceRejection
    } else if reason.contains("constitution") {
        SemanticTrainingCategory::ConstitutionalRejection
    } else {
        SemanticTrainingCategory::OtherRejection
    };
    vec![category]
}

fn external_label(reference: &ExternalRef) -> String {
    format!("{}.{}", reference.namespace, reference.key)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemanticDatasetError {
    Json(String),
    Utf8(String),
    Write(String),
    MissingFinalNewline,
    EmptyLine(usize),
    UnsupportedSchema(String),
}

impl fmt::Display for SemanticDatasetError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "semantic witness dataset rejected state: {self:?}"
        )
    }
}

impl std::error::Error for SemanticDatasetError {}
