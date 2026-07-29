use hollow_grove::{
    CompositionCatalog, CompositionNode, CompositionNodeId, CompositionRecordId, Containment,
    ExternalRef, Position3, ScaleKey, SemanticOperation, SemanticPositionRecord, SemanticScope,
    decode_semantic_position_archive, encode_semantic_position_archive,
    migrate_optional_semantic_position_archive,
};

fn reference(namespace: &str, key: &str) -> ExternalRef {
    ExternalRef::new(namespace, key).unwrap()
}

fn point_node(id: &str, point: &str) -> CompositionNode {
    CompositionNode {
        id: CompositionNodeId::new(id).unwrap(),
        object: reference("point", point),
        scale: ScaleKey::new("scale.point").unwrap(),
    }
}

fn scope(scale: &str, context: Option<&str>) -> SemanticScope {
    SemanticScope::new(
        ScaleKey::new(scale).unwrap(),
        context.map(|key| reference("context", key)),
    )
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
        evidence_refs: vec![reference("evidence", id)],
        constitutional_refs: vec![reference("constitution", "semantic-kernel-v1")],
        witness_ref: reference("witness", id),
        cycle_ref: reference("cycle", id),
    }
}

#[test]
fn identity_position_causality_and_containment_remain_distinct() {
    let mut catalog = CompositionCatalog::new();
    let container = point_node("node.container", "container");
    let point = point_node("node.point.object", "object");
    catalog.insert_node(container.clone()).unwrap();
    catalog.insert_node(point.clone()).unwrap();
    catalog
        .add_containment(Containment {
            container: container.id.clone(),
            member: point.id.clone(),
        })
        .unwrap();

    let selected_scope = scope("scale.structural-repair", Some("inspection.041"));
    catalog
        .assert_semantic_position(assertion(
            "record.semantic.object.initial",
            "object",
            selected_scope.clone(),
            Position3::from_raw(-80, 0, -310).unwrap(),
        ))
        .unwrap();

    assert_eq!(catalog.node(&point.id), Some(&point));
    assert_eq!(catalog.direct_containers(&point.id), vec![&container]);
    assert!(
        catalog.records_producing_result(&point.id).is_empty(),
        "position assertion alone is not composition provenance"
    );
    assert_eq!(
        catalog
            .current_semantic_position(&reference("point", "object"), &selected_scope)
            .unwrap()
            .position,
        Position3::from_raw(-80, 0, -310).unwrap()
    );
}

#[test]
fn contextual_polysemy_is_indexed_by_scale_and_context() {
    let mut catalog = CompositionCatalog::new();
    catalog
        .insert_node(point_node("node.point.frame", "frame"))
        .unwrap();
    let construction = scope("scale.construction", Some("document.foundation"));
    let diagnosis = scope("scale.medical-diagnosis", Some("inspection.041"));
    catalog
        .assert_semantic_position(assertion(
            "record.semantic.frame.construction",
            "frame",
            construction.clone(),
            Position3::from_raw(50, 0, 750).unwrap(),
        ))
        .unwrap();
    catalog
        .assert_semantic_position(assertion(
            "record.semantic.frame.diagnosis",
            "frame",
            diagnosis.clone(),
            Position3::from_raw(-100, 0, -650).unwrap(),
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
fn archive_round_trip_replays_without_inventing_positions() {
    let point = point_node("node.point.object", "object");
    let mut source = CompositionCatalog::new();
    source.insert_node(point.clone()).unwrap();
    source
        .assert_semantic_position(assertion(
            "record.semantic.object",
            "object",
            scope("scale.test", None),
            Position3::from_raw(10, 0, 20).unwrap(),
        ))
        .unwrap();
    let bytes = encode_semantic_position_archive(&source).unwrap();

    let mut replayed = CompositionCatalog::new();
    replayed.insert_node(point).unwrap();
    decode_semantic_position_archive(&bytes, &mut replayed).unwrap();
    assert_eq!(encode_semantic_position_archive(&replayed).unwrap(), bytes);

    let legacy = point_node("node.point.legacy", "legacy");
    let mut legacy_catalog = CompositionCatalog::new();
    legacy_catalog.insert_node(legacy.clone()).unwrap();
    migrate_optional_semantic_position_archive(None, &mut legacy_catalog).unwrap();
    assert_eq!(legacy_catalog.node(&legacy.id), Some(&legacy));
    assert!(legacy_catalog.semantic_position_records().is_empty());
}
