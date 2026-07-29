//! Scoped contextual positions stored by the neutral composition catalog.

use std::fmt;

use serde::{Deserialize, Serialize};

use super::{
    CompositionCatalog, CompositionRecordId, CoordinateError, ExternalRef, Position3, ScaleKey,
    Translation3, verify_translation,
};

pub const SEMANTIC_KERNEL_SCHEMA_VERSION: &str = "hollow-grove-semantic-kernel-v1";
pub const SEMANTIC_WITNESS_SCHEMA_VERSION: &str = "hollow-grove-semantic-witness-v1";

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticScope {
    pub scale: ScaleKey,
    pub context_ref: Option<ExternalRef>,
}

impl SemanticScope {
    #[must_use]
    pub const fn new(scale: ScaleKey, context_ref: Option<ExternalRef>) -> Self {
        Self { scale, context_ref }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum SemanticOperation {
    PositionAssertion {
        scope: SemanticScope,
        position: Position3,
    },
    SpatialTranslation {
        scope: SemanticScope,
        from: Position3,
        translation: Translation3,
        to: Position3,
    },
}

impl SemanticOperation {
    #[must_use]
    pub const fn scope(&self) -> &SemanticScope {
        match self {
            Self::PositionAssertion { scope, .. } | Self::SpatialTranslation { scope, .. } => scope,
        }
    }

    #[must_use]
    pub const fn destination(&self) -> Position3 {
        match self {
            Self::PositionAssertion { position, .. } => *position,
            Self::SpatialTranslation { to, .. } => *to,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticPositionRecord {
    pub record_id: CompositionRecordId,
    pub point_ref: ExternalRef,
    pub scope: SemanticScope,
    pub position: Position3,
    pub operation: SemanticOperation,
    pub previous_record_id: Option<CompositionRecordId>,
    pub evidence_refs: Vec<ExternalRef>,
    pub constitutional_refs: Vec<ExternalRef>,
    pub witness_ref: ExternalRef,
    pub cycle_ref: ExternalRef,
}

impl SemanticPositionRecord {
    #[must_use]
    pub fn canonicalized(&self) -> Self {
        let mut value = self.clone();
        value.evidence_refs.sort();
        value.evidence_refs.dedup();
        value.constitutional_refs.sort();
        value.constitutional_refs.dedup();
        value
    }

    pub fn validate_shape(&self) -> Result<(), SemanticRecordError> {
        if self.scope != *self.operation.scope()
            || self.position != self.operation.destination()
            || self.evidence_refs.is_empty()
            || self.constitutional_refs.is_empty()
        {
            return Err(SemanticRecordError::InvalidRecord(self.record_id.clone()));
        }
        validate_external_ref(&self.point_ref)?;
        validate_external_ref(&self.witness_ref)?;
        validate_external_ref(&self.cycle_ref)?;
        if let Some(context) = &self.scope.context_ref {
            validate_external_ref(context)?;
        }
        for reference in self
            .evidence_refs
            .iter()
            .chain(self.constitutional_refs.iter())
        {
            validate_external_ref(reference)?;
        }
        match &self.operation {
            SemanticOperation::PositionAssertion { .. } => {
                if self.previous_record_id.is_some() {
                    return Err(SemanticRecordError::AssertionHasPredecessor(
                        self.record_id.clone(),
                    ));
                }
            }
            SemanticOperation::SpatialTranslation {
                from,
                translation,
                to,
                ..
            } => {
                if self.previous_record_id.is_none() {
                    return Err(SemanticRecordError::TranslationMissingPredecessor(
                        self.record_id.clone(),
                    ));
                }
                verify_translation(*from, *translation, *to)
                    .map_err(SemanticRecordError::Coordinate)?;
            }
        }
        Ok(())
    }
}

fn validate_external_ref(reference: &ExternalRef) -> Result<(), SemanticRecordError> {
    ExternalRef::new(reference.namespace.clone(), reference.key.clone())
        .map(|_| ())
        .map_err(|_| SemanticRecordError::InvalidExternalReference(reference.clone()))
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticPositionSnapshot {
    pub schema_version: String,
    pub records: Vec<SemanticPositionRecord>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticWitnessDecision {
    Confirmed,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticWitnessRecord {
    pub witness_ref: ExternalRef,
    pub proposal_ref: ExternalRef,
    pub cycle_ref: ExternalRef,
    pub actor_ref: ExternalRef,
    pub point_ref: ExternalRef,
    pub scope: SemanticScope,
    pub input: String,
    pub prior_position: Option<Position3>,
    pub proposed_position: Position3,
    pub translation: Option<Translation3>,
    pub confirmed_position: Option<Position3>,
    pub decision: SemanticWitnessDecision,
    pub evidence_refs: Vec<ExternalRef>,
    pub constitutional_refs: Vec<ExternalRef>,
    pub composition_ref: Option<ExternalRef>,
    pub rejection_reasons: Vec<String>,
    pub semantic_tags: Vec<String>,
    pub summary: String,
    pub prompt_version: String,
    pub interpreter_version: String,
}

impl SemanticWitnessRecord {
    #[must_use]
    pub fn canonicalized(&self) -> Self {
        let mut value = self.clone();
        value.evidence_refs.sort();
        value.evidence_refs.dedup();
        value.constitutional_refs.sort();
        value.constitutional_refs.dedup();
        value.semantic_tags.sort();
        value.semantic_tags.dedup();
        value
    }

    pub fn validate_shape(&self) -> Result<(), SemanticRecordError> {
        for reference in [
            &self.witness_ref,
            &self.proposal_ref,
            &self.cycle_ref,
            &self.actor_ref,
            &self.point_ref,
        ]
        .into_iter()
        .chain(self.scope.context_ref.iter())
        .chain(self.evidence_refs.iter())
        .chain(self.constitutional_refs.iter())
        .chain(self.composition_ref.iter())
        {
            validate_external_ref(reference)?;
        }
        if self.input.trim().is_empty()
            || self.summary.trim().is_empty()
            || self.prompt_version.trim().is_empty()
            || self.interpreter_version.trim().is_empty()
        {
            return Err(SemanticRecordError::InvalidWitness(
                self.witness_ref.clone(),
            ));
        }
        if let Some(translation) = self.translation {
            let Some(prior) = self.prior_position else {
                return Err(SemanticRecordError::InvalidWitness(
                    self.witness_ref.clone(),
                ));
            };
            verify_translation(prior, translation, self.proposed_position)
                .map_err(SemanticRecordError::Coordinate)?;
        }
        match self.decision {
            SemanticWitnessDecision::Confirmed => {
                if self.confirmed_position != Some(self.proposed_position)
                    || self.composition_ref.is_none()
                    || !self.rejection_reasons.is_empty()
                {
                    return Err(SemanticRecordError::InvalidWitness(
                        self.witness_ref.clone(),
                    ));
                }
            }
            SemanticWitnessDecision::Rejected => {
                if self.confirmed_position.is_some()
                    || self.composition_ref.is_some()
                    || self.rejection_reasons.is_empty()
                {
                    return Err(SemanticRecordError::InvalidWitness(
                        self.witness_ref.clone(),
                    ));
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticWitnessSnapshot {
    pub schema_version: String,
    pub witnesses: Vec<SemanticWitnessRecord>,
}

impl SemanticWitnessSnapshot {
    #[must_use]
    pub fn canonicalized(&self) -> Self {
        let mut value = self.clone();
        value.witnesses = value
            .witnesses
            .iter()
            .map(SemanticWitnessRecord::canonicalized)
            .collect();
        value
            .witnesses
            .sort_by(|left, right| left.witness_ref.cmp(&right.witness_ref));
        value
    }
}

impl SemanticPositionSnapshot {
    #[must_use]
    pub fn canonicalized(&self) -> Self {
        let mut value = self.clone();
        value.records = value
            .records
            .iter()
            .map(SemanticPositionRecord::canonicalized)
            .collect();
        value
            .records
            .sort_by(|left, right| left.record_id.cmp(&right.record_id));
        value
    }
}

impl CompositionCatalog {
    pub fn assert_semantic_position(
        &mut self,
        record: SemanticPositionRecord,
    ) -> Result<(), SemanticRecordError> {
        record.validate_shape()?;
        if !matches!(
            record.operation,
            SemanticOperation::PositionAssertion { .. }
        ) {
            return Err(SemanticRecordError::ExpectedPositionAssertion(
                record.record_id,
            ));
        }
        self.record_semantic_position(record)
    }

    pub fn record_semantic_position(
        &mut self,
        record: SemanticPositionRecord,
    ) -> Result<(), SemanticRecordError> {
        let record = record.canonicalized();
        record.validate_shape()?;
        if self
            .semantic_record_positions
            .contains_key(&record.record_id)
        {
            return Err(SemanticRecordError::DuplicateStableId(record.record_id));
        }
        if !self
            .nodes
            .iter()
            .any(|node| node.object == record.point_ref)
        {
            return Err(SemanticRecordError::UnknownPoint(record.point_ref));
        }

        let key = (record.point_ref.clone(), record.scope.clone());
        let current = self
            .current_semantic_positions
            .get(&key)
            .and_then(|position| self.semantic_positions.get(*position));
        match &record.operation {
            SemanticOperation::PositionAssertion { .. } => {
                if let Some(current) = current {
                    return Err(SemanticRecordError::PositionAlreadyExists {
                        point_ref: record.point_ref,
                        scope: record.scope,
                        current_record_id: current.record_id.clone(),
                    });
                }
            }
            SemanticOperation::SpatialTranslation { from, .. } => {
                let current =
                    current.ok_or_else(|| SemanticRecordError::MissingSourcePosition {
                        point_ref: record.point_ref.clone(),
                        scope: record.scope.clone(),
                    })?;
                if current.position != *from
                    || record.previous_record_id.as_ref() != Some(&current.record_id)
                {
                    return Err(SemanticRecordError::StaleSourcePosition {
                        expected_record_id: current.record_id.clone(),
                        expected: current.position,
                        supplied_record_id: record.previous_record_id,
                        supplied: *from,
                    });
                }
            }
        }

        let position = self.semantic_positions.len();
        self.semantic_record_positions
            .insert(record.record_id.clone(), position);
        self.semantic_history
            .entry(key.clone())
            .or_default()
            .push(position);
        self.current_semantic_positions.insert(key, position);
        self.semantic_positions.push(record);
        Ok(())
    }

    #[must_use]
    pub fn semantic_position_record(
        &self,
        id: &CompositionRecordId,
    ) -> Option<&SemanticPositionRecord> {
        self.semantic_record_positions
            .get(id)
            .and_then(|position| self.semantic_positions.get(*position))
    }

    #[must_use]
    pub fn current_semantic_position(
        &self,
        point_ref: &ExternalRef,
        scope: &SemanticScope,
    ) -> Option<&SemanticPositionRecord> {
        self.current_semantic_positions
            .get(&(point_ref.clone(), scope.clone()))
            .and_then(|position| self.semantic_positions.get(*position))
    }

    #[must_use]
    pub fn semantic_position_history(
        &self,
        point_ref: &ExternalRef,
        scope: &SemanticScope,
    ) -> Vec<&SemanticPositionRecord> {
        self.semantic_history
            .get(&(point_ref.clone(), scope.clone()))
            .into_iter()
            .flatten()
            .filter_map(|position| self.semantic_positions.get(*position))
            .collect()
    }

    #[must_use]
    pub fn semantic_position_records(&self) -> &[SemanticPositionRecord] {
        &self.semantic_positions
    }

    #[must_use]
    pub fn semantic_position_snapshot(&self) -> SemanticPositionSnapshot {
        SemanticPositionSnapshot {
            schema_version: SEMANTIC_KERNEL_SCHEMA_VERSION.into(),
            records: self.semantic_positions.clone(),
        }
        .canonicalized()
    }

    pub fn record_semantic_witness(
        &mut self,
        witness: SemanticWitnessRecord,
    ) -> Result<(), SemanticRecordError> {
        let witness = witness.canonicalized();
        witness.validate_shape()?;
        if self
            .semantic_witness_positions
            .contains_key(&witness.witness_ref)
        {
            return Err(SemanticRecordError::DuplicateWitnessRef(
                witness.witness_ref,
            ));
        }
        let position = self.semantic_witnesses.len();
        self.semantic_witness_positions
            .insert(witness.witness_ref.clone(), position);
        self.semantic_witnesses.push(witness);
        Ok(())
    }

    #[must_use]
    pub fn semantic_witness(&self, witness_ref: &ExternalRef) -> Option<&SemanticWitnessRecord> {
        self.semantic_witness_positions
            .get(witness_ref)
            .and_then(|position| self.semantic_witnesses.get(*position))
    }

    #[must_use]
    pub fn semantic_witnesses(&self) -> &[SemanticWitnessRecord] {
        &self.semantic_witnesses
    }

    #[must_use]
    pub fn semantic_witness_snapshot(&self) -> SemanticWitnessSnapshot {
        SemanticWitnessSnapshot {
            schema_version: SEMANTIC_WITNESS_SCHEMA_VERSION.into(),
            witnesses: self.semantic_witnesses.clone(),
        }
        .canonicalized()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemanticRecordError {
    InvalidRecord(CompositionRecordId),
    InvalidWitness(ExternalRef),
    InvalidExternalReference(ExternalRef),
    AssertionHasPredecessor(CompositionRecordId),
    TranslationMissingPredecessor(CompositionRecordId),
    ExpectedPositionAssertion(CompositionRecordId),
    DuplicateStableId(CompositionRecordId),
    DuplicateWitnessRef(ExternalRef),
    UnknownPoint(ExternalRef),
    PositionAlreadyExists {
        point_ref: ExternalRef,
        scope: SemanticScope,
        current_record_id: CompositionRecordId,
    },
    MissingSourcePosition {
        point_ref: ExternalRef,
        scope: SemanticScope,
    },
    StaleSourcePosition {
        expected_record_id: CompositionRecordId,
        expected: Position3,
        supplied_record_id: Option<CompositionRecordId>,
        supplied: Position3,
    },
    UnsupportedSchema(String),
    ReplayBlocked(Vec<CompositionRecordId>),
    Coordinate(CoordinateError),
}

impl fmt::Display for SemanticRecordError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "semantic position rejected state: {self:?}")
    }
}

impl std::error::Error for SemanticRecordError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CompositionNode, CompositionNodeId};

    fn reference(namespace: &str, key: &str) -> ExternalRef {
        ExternalRef::new(namespace, key).unwrap()
    }

    fn scope(scale: &str, context: Option<&str>) -> SemanticScope {
        SemanticScope::new(
            ScaleKey::new(scale).unwrap(),
            context.map(|key| reference("context", key)),
        )
    }

    fn point_node(id: &str, object: &str) -> CompositionNode {
        CompositionNode {
            id: CompositionNodeId::new(id).unwrap(),
            object: reference("point", object),
            scale: ScaleKey::new("scale.point").unwrap(),
        }
    }

    fn assertion(
        id: &str,
        point: &str,
        scope: SemanticScope,
        position: Position3,
    ) -> SemanticPositionRecord {
        SemanticPositionRecord {
            record_id: CompositionRecordId::new(id).unwrap(),
            point_ref: reference("point", point),
            scope: scope.clone(),
            position,
            operation: SemanticOperation::PositionAssertion { scope, position },
            previous_record_id: None,
            evidence_refs: vec![reference("evidence", "inspection.001")],
            constitutional_refs: vec![reference("constitution", "semantic-kernel-v1")],
            witness_ref: reference("witness", id),
            cycle_ref: reference("cycle", id),
        }
    }

    #[test]
    fn two_points_may_share_one_coordinate_without_sharing_identity() {
        let mut catalog = CompositionCatalog::new();
        catalog
            .insert_node(point_node("node.point.first", "first"))
            .unwrap();
        catalog
            .insert_node(point_node("node.point.second", "second"))
            .unwrap();
        let shared = Position3::from_raw(-250, 0, 600).unwrap();
        let selected_scope = scope("scale.structural-repair", None);
        catalog
            .assert_semantic_position(assertion(
                "record.semantic.first",
                "first",
                selected_scope.clone(),
                shared,
            ))
            .unwrap();
        catalog
            .assert_semantic_position(assertion(
                "record.semantic.second",
                "second",
                selected_scope.clone(),
                shared,
            ))
            .unwrap();
        assert_eq!(
            catalog
                .current_semantic_position(&reference("point", "first"), &selected_scope)
                .unwrap()
                .position,
            catalog
                .current_semantic_position(&reference("point", "second"), &selected_scope)
                .unwrap()
                .position
        );
        assert_ne!(reference("point", "first"), reference("point", "second"));
    }

    #[test]
    fn one_point_may_hold_different_positions_by_scale_and_context() {
        let mut catalog = CompositionCatalog::new();
        catalog
            .insert_node(point_node("node.point.frame", "frame"))
            .unwrap();
        let construction = scope("scale.construction", Some("document.build"));
        let diagnosis = scope("scale.medical-diagnosis", Some("inspection.041"));
        catalog
            .assert_semantic_position(assertion(
                "record.semantic.frame.construction",
                "frame",
                construction.clone(),
                Position3::from_raw(0, 0, 700).unwrap(),
            ))
            .unwrap();
        catalog
            .assert_semantic_position(assertion(
                "record.semantic.frame.diagnosis",
                "frame",
                diagnosis.clone(),
                Position3::from_raw(-50, 0, -600).unwrap(),
            ))
            .unwrap();
        assert_ne!(
            catalog
                .current_semantic_position(&reference("point", "frame"), &construction)
                .unwrap()
                .position,
            catalog
                .current_semantic_position(&reference("point", "frame"), &diagnosis)
                .unwrap()
                .position
        );
    }

    #[test]
    fn assertion_requires_known_identity_evidence_and_no_existing_state() {
        let mut catalog = CompositionCatalog::new();
        let selected_scope = scope("scale.test", None);
        let record = assertion(
            "record.semantic.missing",
            "missing",
            selected_scope.clone(),
            Position3::ORIGIN,
        );
        assert!(matches!(
            catalog.assert_semantic_position(record),
            Err(SemanticRecordError::UnknownPoint(_))
        ));

        catalog
            .insert_node(point_node("node.point.present", "present"))
            .unwrap();
        let record = assertion(
            "record.semantic.present",
            "present",
            selected_scope.clone(),
            Position3::ORIGIN,
        );
        catalog.assert_semantic_position(record.clone()).unwrap();
        assert!(matches!(
            catalog.assert_semantic_position(record),
            Err(SemanticRecordError::DuplicateStableId(_))
        ));
        let second = assertion(
            "record.semantic.present.second",
            "present",
            selected_scope,
            Position3::from_raw(1, 0, 0).unwrap(),
        );
        assert!(matches!(
            catalog.assert_semantic_position(second),
            Err(SemanticRecordError::PositionAlreadyExists { .. })
        ));
    }

    #[test]
    fn position_does_not_create_containment_or_change_point_identity() {
        let mut catalog = CompositionCatalog::new();
        let point = point_node("node.point.independent", "independent");
        catalog.insert_node(point.clone()).unwrap();
        catalog
            .assert_semantic_position(assertion(
                "record.semantic.independent",
                "independent",
                scope("scale.test", None),
                Position3::from_raw(300, 0, -200).unwrap(),
            ))
            .unwrap();
        assert!(catalog.direct_containers(&point.id).is_empty());
        assert_eq!(catalog.node(&point.id).unwrap(), &point);
    }

    #[test]
    fn snapshots_are_sorted_by_stable_id_and_empty_state_is_valid() {
        let empty = CompositionCatalog::new().semantic_position_snapshot();
        assert_eq!(empty.schema_version, SEMANTIC_KERNEL_SCHEMA_VERSION);
        assert!(empty.records.is_empty());

        let mut catalog = CompositionCatalog::new();
        catalog
            .insert_node(point_node("node.point.zed", "zed"))
            .unwrap();
        catalog
            .insert_node(point_node("node.point.alpha", "alpha"))
            .unwrap();
        catalog
            .assert_semantic_position(assertion(
                "record.semantic.zed",
                "zed",
                scope("scale.test", None),
                Position3::from_raw(1, 0, 0).unwrap(),
            ))
            .unwrap();
        catalog
            .assert_semantic_position(assertion(
                "record.semantic.alpha",
                "alpha",
                scope("scale.test", None),
                Position3::from_raw(-1, 0, 0).unwrap(),
            ))
            .unwrap();
        let ids = catalog
            .semantic_position_snapshot()
            .records
            .into_iter()
            .map(|record| record.record_id)
            .collect::<Vec<_>>();
        assert_eq!(
            ids,
            vec![
                CompositionRecordId::new("record.semantic.alpha").unwrap(),
                CompositionRecordId::new("record.semantic.zed").unwrap(),
            ]
        );
    }
}
