use std::fmt;

use crate::composition::{
    CompositionCatalog, CompositionRecordId, SEMANTIC_KERNEL_SCHEMA_VERSION,
    SemanticPositionRecord, SemanticPositionSnapshot, SemanticRecordError,
};

pub fn encode_semantic_position_archive(
    catalog: &CompositionCatalog,
) -> Result<Vec<u8>, SemanticArchiveError> {
    serde_json::to_vec(&catalog.semantic_position_snapshot())
        .map_err(|error| SemanticArchiveError::Json(error.to_string()))
}

pub fn decode_semantic_position_archive(
    bytes: &[u8],
    catalog: &mut CompositionCatalog,
) -> Result<SemanticPositionSnapshot, SemanticArchiveError> {
    let snapshot: SemanticPositionSnapshot = serde_json::from_slice(bytes)
        .map_err(|error| SemanticArchiveError::Json(error.to_string()))?;
    replay_semantic_position_snapshot(&snapshot, catalog)?;
    Ok(snapshot.canonicalized())
}

pub fn replay_semantic_position_snapshot(
    snapshot: &SemanticPositionSnapshot,
    catalog: &mut CompositionCatalog,
) -> Result<(), SemanticArchiveError> {
    if snapshot.schema_version != SEMANTIC_KERNEL_SCHEMA_VERSION {
        return Err(SemanticArchiveError::UnsupportedSchema(
            snapshot.schema_version.clone(),
        ));
    }

    let mut staged = catalog.clone();
    let mut pending = snapshot.canonicalized().records;
    while !pending.is_empty() {
        if let Some(existing) = pending.iter().position(|record| {
            staged
                .semantic_position_record(&record.record_id)
                .is_some_and(|stored| stored == record)
        }) {
            pending.remove(existing);
            continue;
        }
        let ready = pending
            .iter()
            .position(|record| record_is_ready(record, &staged));
        let Some(ready) = ready else {
            return Err(SemanticArchiveError::ReplayBlocked(
                pending
                    .iter()
                    .map(|record| record.record_id.clone())
                    .collect(),
            ));
        };
        let record = pending.remove(ready);
        staged
            .record_semantic_position(record)
            .map_err(SemanticArchiveError::Record)?;
    }
    *catalog = staged;
    Ok(())
}

fn record_is_ready(record: &SemanticPositionRecord, catalog: &CompositionCatalog) -> bool {
    match &record.previous_record_id {
        None => true,
        Some(previous) => catalog
            .current_semantic_position(&record.point_ref, &record.scope)
            .is_some_and(|current| &current.record_id == previous),
    }
}

pub fn migrate_optional_semantic_position_archive(
    bytes: Option<&[u8]>,
    catalog: &mut CompositionCatalog,
) -> Result<Vec<u8>, SemanticArchiveError> {
    if let Some(bytes) = bytes {
        decode_semantic_position_archive(bytes, catalog)?;
    }
    encode_semantic_position_archive(catalog)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemanticArchiveError {
    Json(String),
    UnsupportedSchema(String),
    ReplayBlocked(Vec<CompositionRecordId>),
    Record(SemanticRecordError),
}

impl fmt::Display for SemanticArchiveError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "semantic archive rejected state: {self:?}")
    }
}

impl std::error::Error for SemanticArchiveError {}

#[cfg(test)]
mod tests {
    use crate::composition::{
        CompositionNode, CompositionNodeId, ExternalRef, Position3, ScaleKey, SemanticOperation,
        SemanticScope,
    };

    use super::*;

    fn reference(namespace: &str, key: &str) -> ExternalRef {
        ExternalRef::new(namespace, key).unwrap()
    }

    fn scope(scale: &str) -> SemanticScope {
        SemanticScope::new(ScaleKey::new(scale).unwrap(), None)
    }

    fn node(id: &str, point: &str) -> CompositionNode {
        CompositionNode {
            id: CompositionNodeId::new(id).unwrap(),
            object: reference("point", point),
            scale: ScaleKey::new("scale.point").unwrap(),
        }
    }

    fn assertion(id: &str, point: &str, position: Position3) -> SemanticPositionRecord {
        let scope = scope("scale.test");
        SemanticPositionRecord {
            record_id: CompositionRecordId::new(id).unwrap(),
            point_ref: reference("point", point),
            scope: scope.clone(),
            position,
            operation: SemanticOperation::PositionAssertion { scope, position },
            previous_record_id: None,
            evidence_refs: vec![reference("evidence", point)],
            constitutional_refs: vec![reference("constitution", "semantic-kernel-v1")],
            witness_ref: reference("witness", id),
            cycle_ref: reference("cycle", id),
        }
    }

    #[test]
    fn absent_semantic_archive_preserves_existing_points_without_backfill() {
        let mut catalog = CompositionCatalog::new();
        let point = node("node.point.legacy", "legacy");
        catalog.insert_node(point.clone()).unwrap();
        let migrated = migrate_optional_semantic_position_archive(None, &mut catalog).unwrap();
        assert_eq!(catalog.node(&point.id), Some(&point));
        assert!(catalog.semantic_position_records().is_empty());
        let snapshot: SemanticPositionSnapshot = serde_json::from_slice(&migrated).unwrap();
        assert_eq!(snapshot.schema_version, SEMANTIC_KERNEL_SCHEMA_VERSION);
        assert!(snapshot.records.is_empty());
    }

    #[test]
    fn archive_is_byte_identical_across_insertion_orders() {
        let mut forward = CompositionCatalog::new();
        forward.insert_node(node("node.point.a", "a")).unwrap();
        forward.insert_node(node("node.point.b", "b")).unwrap();
        forward
            .assert_semantic_position(assertion(
                "record.semantic.b",
                "b",
                Position3::from_raw(20, 0, 30).unwrap(),
            ))
            .unwrap();
        forward
            .assert_semantic_position(assertion(
                "record.semantic.a",
                "a",
                Position3::from_raw(-20, 0, -30).unwrap(),
            ))
            .unwrap();

        let mut reverse = CompositionCatalog::new();
        reverse.insert_node(node("node.point.b", "b")).unwrap();
        reverse.insert_node(node("node.point.a", "a")).unwrap();
        reverse
            .assert_semantic_position(assertion(
                "record.semantic.a",
                "a",
                Position3::from_raw(-20, 0, -30).unwrap(),
            ))
            .unwrap();
        reverse
            .assert_semantic_position(assertion(
                "record.semantic.b",
                "b",
                Position3::from_raw(20, 0, 30).unwrap(),
            ))
            .unwrap();

        assert_eq!(
            encode_semantic_position_archive(&forward).unwrap(),
            encode_semantic_position_archive(&reverse).unwrap()
        );
    }

    #[test]
    fn replay_is_transactional_and_rejects_unknown_schema() {
        let mut catalog = CompositionCatalog::new();
        catalog.insert_node(node("node.point.a", "a")).unwrap();
        let before = encode_semantic_position_archive(&catalog).unwrap();
        let future = SemanticPositionSnapshot {
            schema_version: "hollow-grove-semantic-kernel-v2".into(),
            records: vec![assertion("record.semantic.a", "a", Position3::ORIGIN)],
        };
        assert!(matches!(
            replay_semantic_position_snapshot(&future, &mut catalog),
            Err(SemanticArchiveError::UnsupportedSchema(_))
        ));
        assert_eq!(encode_semantic_position_archive(&catalog).unwrap(), before);
    }

    #[test]
    fn migration_is_idempotent_for_an_already_replayed_archive() {
        let mut catalog = CompositionCatalog::new();
        catalog.insert_node(node("node.point.a", "a")).unwrap();
        catalog
            .assert_semantic_position(assertion("record.semantic.a", "a", Position3::ORIGIN))
            .unwrap();
        let bytes = encode_semantic_position_archive(&catalog).unwrap();
        assert_eq!(
            migrate_optional_semantic_position_archive(Some(&bytes), &mut catalog).unwrap(),
            bytes
        );
        assert_eq!(
            migrate_optional_semantic_position_archive(Some(&bytes), &mut catalog).unwrap(),
            bytes
        );
        assert_eq!(catalog.semantic_position_records().len(), 1);
    }

    #[test]
    fn strict_archive_and_reference_parsing_reject_unknown_or_invalid_data() {
        let unknown =
            br#"{"schema_version":"hollow-grove-semantic-kernel-v1","records":[],"extra":true}"#;
        let mut catalog = CompositionCatalog::new();
        assert!(decode_semantic_position_archive(unknown, &mut catalog).is_err());
        assert!(
            serde_json::from_str::<ExternalRef>(r#"{"namespace":"Not Stable","key":"value"}"#)
                .is_err()
        );
        assert!(
            serde_json::from_str::<ExternalRef>(
                r#"{"namespace":"point","key":"value","extra":true}"#
            )
            .is_err()
        );
    }
}
