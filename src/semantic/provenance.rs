//! Transactional semantic confirmation over the existing composition catalog.

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::composition::{
    CompositionCatalog, CompositionCatalogError, CompositionNodeId, CompositionRecord,
    CompositionRecordId, ExternalRef, SemanticOperation, SemanticPositionRecord,
    SemanticRecordError, SemanticWitnessDecision, SemanticWitnessRecord,
};

pub const SEMANTIC_PROVENANCE_SCHEMA_VERSION: &str = "hollow-grove-semantic-provenance-v1";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticConfirmationReceipt {
    pub position_record_id: CompositionRecordId,
    pub composition_ref: ExternalRef,
    pub witness_ref: ExternalRef,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConfirmedSemanticProvenance {
    pub position_record: SemanticPositionRecord,
    pub composition_record: CompositionRecord,
    pub witness: SemanticWitnessRecord,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticProvenanceSnapshot {
    pub schema_version: String,
    pub confirmed: Vec<ConfirmedSemanticProvenance>,
    pub rejected: Vec<SemanticWitnessRecord>,
}

impl SemanticProvenanceSnapshot {
    #[must_use]
    pub fn canonicalized(&self) -> Self {
        let mut value = self.clone();
        value.confirmed = value
            .confirmed
            .iter()
            .map(|entry| ConfirmedSemanticProvenance {
                position_record: entry.position_record.canonicalized(),
                composition_record: entry.composition_record.clone(),
                witness: entry.witness.canonicalized(),
            })
            .collect();
        value.confirmed.sort_by(|left, right| {
            left.position_record
                .record_id
                .cmp(&right.position_record.record_id)
        });
        value.rejected = value
            .rejected
            .iter()
            .map(SemanticWitnessRecord::canonicalized)
            .collect();
        value
            .rejected
            .sort_by(|left, right| left.witness_ref.cmp(&right.witness_ref));
        value
    }
}

pub fn confirm_semantic_transition(
    catalog: &mut CompositionCatalog,
    position_record: SemanticPositionRecord,
    witness: SemanticWitnessRecord,
) -> Result<SemanticConfirmationReceipt, SemanticProvenanceError> {
    let position_record = position_record.canonicalized();
    let witness = witness.canonicalized();
    let point_node = unique_point_node(catalog, &position_record.point_ref)?;
    let composition_ref =
        ExternalRef::new("composition-record", position_record.record_id.as_str())
            .map_err(|error| SemanticProvenanceError::ExternalReference(error.to_string()))?;
    let operation = match &position_record.operation {
        SemanticOperation::PositionAssertion { .. } => {
            ExternalRef::new("semantic-operation", "position-assertion-v1")
        }
        SemanticOperation::SpatialTranslation { .. } => {
            ExternalRef::new("semantic-operation", "spatial-translation-v1")
        }
    }
    .map_err(|error| SemanticProvenanceError::ExternalReference(error.to_string()))?;
    let composition_record = CompositionRecord {
        id: position_record.record_id.clone(),
        sources: vec![point_node.clone()],
        result: point_node,
        operation,
        evidence: position_record.evidence_refs.first().cloned(),
    };
    let position_record_id = position_record.record_id.clone();
    let witness_ref = witness.witness_ref.clone();
    confirm_prepared_semantic_transition(catalog, position_record, composition_record, witness)?;
    Ok(SemanticConfirmationReceipt {
        position_record_id,
        composition_ref,
        witness_ref,
    })
}

fn confirm_prepared_semantic_transition(
    catalog: &mut CompositionCatalog,
    position_record: SemanticPositionRecord,
    composition_record: CompositionRecord,
    witness: SemanticWitnessRecord,
) -> Result<(), SemanticProvenanceError> {
    let position_record = position_record.canonicalized();
    let witness = witness.canonicalized();
    position_record
        .validate_shape()
        .map_err(SemanticProvenanceError::SemanticRecord)?;
    witness
        .validate_shape()
        .map_err(SemanticProvenanceError::SemanticRecord)?;
    validate_confirmed_witness(&position_record, &composition_record, &witness)?;

    let point_node = unique_point_node(catalog, &position_record.point_ref)?;
    let expected_operation = match position_record.operation {
        SemanticOperation::PositionAssertion { .. } => "position-assertion-v1",
        SemanticOperation::SpatialTranslation { .. } => "spatial-translation-v1",
    };
    if composition_record.id != position_record.record_id
        || composition_record.sources != [point_node.clone()]
        || composition_record.result != point_node
        || composition_record.operation.namespace != "semantic-operation"
        || composition_record.operation.key != expected_operation
        || composition_record.evidence != position_record.evidence_refs.first().cloned()
    {
        return Err(SemanticProvenanceError::ProvenanceMismatch(
            position_record.record_id,
        ));
    }

    let mut staged = catalog.clone();
    staged
        .insert_record(composition_record)
        .map_err(SemanticProvenanceError::Composition)?;
    staged
        .record_semantic_position(position_record)
        .map_err(SemanticProvenanceError::SemanticRecord)?;
    staged
        .record_semantic_witness(witness)
        .map_err(SemanticProvenanceError::SemanticRecord)?;
    *catalog = staged;
    Ok(())
}

pub fn record_rejected_semantic_witness(
    catalog: &mut CompositionCatalog,
    witness: SemanticWitnessRecord,
) -> Result<(), SemanticProvenanceError> {
    if witness.decision != SemanticWitnessDecision::Rejected {
        return Err(SemanticProvenanceError::ExpectedRejectedWitness(
            witness.witness_ref,
        ));
    }
    let mut staged = catalog.clone();
    staged
        .record_semantic_witness(witness)
        .map_err(SemanticProvenanceError::SemanticRecord)?;
    *catalog = staged;
    Ok(())
}

pub fn semantic_provenance_snapshot(
    catalog: &CompositionCatalog,
) -> Result<SemanticProvenanceSnapshot, SemanticProvenanceError> {
    let mut confirmed = Vec::new();
    for position_record in catalog.semantic_position_records() {
        let composition_record = catalog.record(&position_record.record_id).ok_or_else(|| {
            SemanticProvenanceError::MissingComposition(position_record.record_id.clone())
        })?;
        let witness = catalog
            .semantic_witness(&position_record.witness_ref)
            .ok_or_else(|| {
                SemanticProvenanceError::MissingWitness(position_record.witness_ref.clone())
            })?;
        confirmed.push(ConfirmedSemanticProvenance {
            position_record: position_record.clone(),
            composition_record: composition_record.clone(),
            witness: witness.clone(),
        });
    }
    let rejected = catalog
        .semantic_witnesses()
        .iter()
        .filter(|witness| witness.decision == SemanticWitnessDecision::Rejected)
        .cloned()
        .collect();
    Ok(SemanticProvenanceSnapshot {
        schema_version: SEMANTIC_PROVENANCE_SCHEMA_VERSION.into(),
        confirmed,
        rejected,
    }
    .canonicalized())
}

pub fn encode_semantic_provenance_snapshot(
    catalog: &CompositionCatalog,
) -> Result<Vec<u8>, SemanticProvenanceError> {
    serde_json::to_vec(&semantic_provenance_snapshot(catalog)?)
        .map_err(|error| SemanticProvenanceError::Json(error.to_string()))
}

pub fn replay_semantic_provenance_snapshot(
    snapshot: &SemanticProvenanceSnapshot,
    catalog: &mut CompositionCatalog,
) -> Result<(), SemanticProvenanceError> {
    if snapshot.schema_version != SEMANTIC_PROVENANCE_SCHEMA_VERSION {
        return Err(SemanticProvenanceError::UnsupportedSchema(
            snapshot.schema_version.clone(),
        ));
    }
    let mut staged = catalog.clone();
    let mut pending = snapshot.canonicalized().confirmed;
    while !pending.is_empty() {
        if let Some(existing) = pending
            .iter()
            .position(|entry| provenance_is_already_present(entry, &staged))
        {
            pending.remove(existing);
            continue;
        }
        let ready = pending
            .iter()
            .position(|entry| provenance_is_ready(entry, &staged));
        let Some(ready) = ready else {
            return Err(SemanticProvenanceError::ReplayMismatch(
                pending
                    .iter()
                    .map(|entry| entry.position_record.record_id.clone())
                    .collect(),
            ));
        };
        let entry = pending.remove(ready);
        confirm_prepared_semantic_transition(
            &mut staged,
            entry.position_record,
            entry.composition_record,
            entry.witness,
        )?;
    }
    for witness in snapshot.canonicalized().rejected {
        match staged.semantic_witness(&witness.witness_ref) {
            Some(existing) if existing == &witness => {}
            Some(_) => {
                return Err(SemanticProvenanceError::WitnessReplayMismatch(
                    witness.witness_ref,
                ));
            }
            None => record_rejected_semantic_witness(&mut staged, witness)?,
        }
    }
    *catalog = staged;
    Ok(())
}

pub fn decode_semantic_provenance_snapshot(
    bytes: &[u8],
    catalog: &mut CompositionCatalog,
) -> Result<SemanticProvenanceSnapshot, SemanticProvenanceError> {
    let snapshot: SemanticProvenanceSnapshot = serde_json::from_slice(bytes)
        .map_err(|error| SemanticProvenanceError::Json(error.to_string()))?;
    replay_semantic_provenance_snapshot(&snapshot, catalog)?;
    Ok(snapshot.canonicalized())
}

fn provenance_is_ready(entry: &ConfirmedSemanticProvenance, catalog: &CompositionCatalog) -> bool {
    match &entry.position_record.previous_record_id {
        None => catalog
            .current_semantic_position(
                &entry.position_record.point_ref,
                &entry.position_record.scope,
            )
            .is_none(),
        Some(previous) => catalog
            .current_semantic_position(
                &entry.position_record.point_ref,
                &entry.position_record.scope,
            )
            .is_some_and(|current| &current.record_id == previous),
    }
}

fn provenance_is_already_present(
    entry: &ConfirmedSemanticProvenance,
    catalog: &CompositionCatalog,
) -> bool {
    catalog
        .semantic_position_record(&entry.position_record.record_id)
        .is_some_and(|stored| stored == &entry.position_record)
        && catalog
            .record(&entry.composition_record.id)
            .is_some_and(|stored| stored == &entry.composition_record)
        && catalog
            .semantic_witness(&entry.witness.witness_ref)
            .is_some_and(|stored| stored == &entry.witness)
}

fn unique_point_node(
    catalog: &CompositionCatalog,
    point_ref: &ExternalRef,
) -> Result<CompositionNodeId, SemanticProvenanceError> {
    let nodes = catalog.nodes_for_object(point_ref);
    match nodes.as_slice() {
        [] => Err(SemanticProvenanceError::UnknownPoint(point_ref.clone())),
        [node] => Ok(node.id.clone()),
        _ => Err(SemanticProvenanceError::AmbiguousPointProjection {
            point_ref: point_ref.clone(),
            node_ids: nodes.iter().map(|node| node.id.clone()).collect(),
        }),
    }
}

fn validate_confirmed_witness(
    position_record: &SemanticPositionRecord,
    composition_record: &CompositionRecord,
    witness: &SemanticWitnessRecord,
) -> Result<(), SemanticProvenanceError> {
    let prior_position = match &position_record.operation {
        SemanticOperation::PositionAssertion { .. } => None,
        SemanticOperation::SpatialTranslation { from, .. } => Some(*from),
    };
    let translation = match &position_record.operation {
        SemanticOperation::PositionAssertion { .. } => None,
        SemanticOperation::SpatialTranslation { translation, .. } => Some(*translation),
    };
    let expected_composition_ref =
        ExternalRef::new("composition-record", composition_record.id.as_str())
            .map_err(|error| SemanticProvenanceError::ExternalReference(error.to_string()))?;
    if witness.decision != SemanticWitnessDecision::Confirmed
        || witness.witness_ref != position_record.witness_ref
        || witness.cycle_ref != position_record.cycle_ref
        || witness.point_ref != position_record.point_ref
        || witness.scope != position_record.scope
        || witness.prior_position != prior_position
        || witness.proposed_position != position_record.position
        || witness.translation != translation
        || witness.confirmed_position != Some(position_record.position)
        || witness.evidence_refs != position_record.canonicalized().evidence_refs
        || witness.constitutional_refs != position_record.canonicalized().constitutional_refs
        || witness.composition_ref.as_ref() != Some(&expected_composition_ref)
    {
        return Err(SemanticProvenanceError::WitnessMismatch(
            witness.witness_ref.clone(),
        ));
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemanticProvenanceError {
    ExternalReference(String),
    UnknownPoint(ExternalRef),
    AmbiguousPointProjection {
        point_ref: ExternalRef,
        node_ids: Vec<CompositionNodeId>,
    },
    ExpectedRejectedWitness(ExternalRef),
    WitnessMismatch(ExternalRef),
    ProvenanceMismatch(CompositionRecordId),
    MissingComposition(CompositionRecordId),
    MissingWitness(ExternalRef),
    Composition(CompositionCatalogError),
    SemanticRecord(SemanticRecordError),
    UnsupportedSchema(String),
    ReplayMismatch(Vec<CompositionRecordId>),
    WitnessReplayMismatch(ExternalRef),
    Json(String),
}

impl fmt::Display for SemanticProvenanceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "semantic provenance rejected state: {self:?}")
    }
}

impl std::error::Error for SemanticProvenanceError {}
