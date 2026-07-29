use hollow_grove::{
    CanonRetriever, CanonStatus, CatalogCanonRetriever, CompositionCatalog, CompositionNode,
    CompositionNodeId, CompositionRecordId, ExternalRef, MAX_NEARBY_SEMANTIC_EXAMPLES,
    MAX_RETRIEVED_CANON_FRAGMENTS, Position3, RetrievalError, RetrievalRequest,
    RetrievedCanonFragment, ScaleKey, SemanticOperation, SemanticPositionRecord, SemanticScope,
    SemanticWitnessDecision, SemanticWitnessRecord, nearby_semantic_examples,
};

fn reference(namespace: &str, key: &str) -> ExternalRef {
    ExternalRef::new(namespace, key).unwrap()
}

fn scope(scale: &str) -> SemanticScope {
    SemanticScope::new(ScaleKey::new(scale).unwrap(), None)
}

fn insert_position(
    catalog: &mut CompositionCatalog,
    id: &str,
    point: &str,
    selected_scope: SemanticScope,
    position: Position3,
    tags: &[&str],
) {
    let point_ref = reference("point", point);
    catalog
        .insert_node(CompositionNode {
            id: CompositionNodeId::new(format!("node.{point}")).unwrap(),
            object: point_ref.clone(),
            scale: ScaleKey::new("scale.point").unwrap(),
        })
        .unwrap();
    let record_id = CompositionRecordId::new(id).unwrap();
    let witness_ref = reference("witness", id);
    let cycle_ref = reference("cycle", id);
    let record = SemanticPositionRecord {
        record_id: record_id.clone(),
        point_ref: point_ref.clone(),
        scope: selected_scope.clone(),
        position,
        operation: SemanticOperation::PositionAssertion {
            scope: selected_scope.clone(),
            position,
        },
        previous_record_id: None,
        evidence_refs: vec![reference("evidence", id)],
        constitutional_refs: vec![reference("constitution", "semantic-kernel-v1")],
        witness_ref: witness_ref.clone(),
        cycle_ref: cycle_ref.clone(),
    };
    catalog.assert_semantic_position(record).unwrap();
    catalog
        .record_semantic_witness(SemanticWitnessRecord {
            witness_ref,
            proposal_ref: reference("proposal", id),
            cycle_ref,
            actor_ref: reference("actor", "retrieval-test"),
            point_ref,
            scope: selected_scope,
            input: "Place a witnessed example.".into(),
            prior_position: None,
            proposed_position: position,
            translation: None,
            confirmed_position: Some(position),
            operation: reference("semantic-intent", "locate"),
            decision: SemanticWitnessDecision::Confirmed,
            evidence_refs: vec![reference("evidence", id)],
            constitutional_refs: vec![reference("constitution", "semantic-kernel-v1")],
            composition_ref: Some(reference("composition-record", record_id.as_str())),
            rejection_reasons: vec![],
            semantic_tags: tags.iter().map(|tag| (*tag).into()).collect(),
            summary: "A concise nearby example.".into(),
            prompt_version: "hollow-grove-semantic-interpreter-v1".into(),
            interpreter_version: "deterministic-v1".into(),
        })
        .unwrap();
}

fn request(center: Option<Position3>) -> RetrievalRequest {
    RetrievalRequest {
        point_ref: reference("point", "query"),
        scope: scope("structural-repair"),
        coordinate: center,
        constitutional_refs: vec![reference("constitution", "semantic-kernel-v1")],
        maximum_canon_fragments: 8,
        maximum_nearby_examples: 8,
        semantic_tags: vec![],
    }
}

fn canon() -> Vec<RetrievedCanonFragment> {
    vec![RetrievedCanonFragment {
        reference: reference("constitution", "semantic-kernel-v1"),
        source_document: "docs/SEMANTIC_KERNEL_V1.md".into(),
        section: "Core constitutional statement".into(),
        status: CanonStatus::Constitutional,
        version: Some("v1".into()),
        content: "Central Junction is Origin. Y remains reserved in V1.".into(),
    }]
}

#[test]
fn nearby_order_is_same_scale_then_distance_then_stable_record_id() {
    let mut catalog = CompositionCatalog::new();
    insert_position(
        &mut catalog,
        "record.semantic.same-b",
        "same-b",
        scope("structural-repair"),
        Position3::from_raw(-900, 0, 0).unwrap(),
        &["support"],
    );
    insert_position(
        &mut catalog,
        "record.semantic.other-close",
        "other-close",
        scope("medical-diagnosis"),
        Position3::from_raw(10, 0, 0).unwrap(),
        &["support"],
    );
    insert_position(
        &mut catalog,
        "record.semantic.same-a",
        "same-a",
        scope("structural-repair"),
        Position3::from_raw(900, 0, 0).unwrap(),
        &["support"],
    );

    let examples = nearby_semantic_examples(&catalog, &request(Some(Position3::ORIGIN))).unwrap();
    assert_eq!(
        examples
            .iter()
            .map(|example| example.record_ref.key.as_str())
            .collect::<Vec<_>>(),
        [
            "record.semantic.same-a",
            "record.semantic.same-b",
            "record.semantic.other-close"
        ]
    );
}

#[test]
fn tags_and_result_limits_bound_the_prompt_neighborhood() {
    let mut catalog = CompositionCatalog::new();
    insert_position(
        &mut catalog,
        "record.semantic.repair",
        "repair",
        scope("structural-repair"),
        Position3::from_raw(10, 0, 20).unwrap(),
        &["repair"],
    );
    insert_position(
        &mut catalog,
        "record.semantic.design",
        "design",
        scope("structural-repair"),
        Position3::from_raw(5, 0, 10).unwrap(),
        &["design"],
    );
    let mut selected = request(Some(Position3::ORIGIN));
    selected.semantic_tags = vec!["repair".into()];
    selected.maximum_nearby_examples = 1;
    let examples = nearby_semantic_examples(&catalog, &selected).unwrap();
    assert_eq!(examples.len(), 1);
    assert_eq!(examples[0].semantic_tags, ["repair"]);
}

#[test]
fn absent_query_position_returns_no_examples_instead_of_using_origin() {
    let mut catalog = CompositionCatalog::new();
    insert_position(
        &mut catalog,
        "record.semantic.existing",
        "existing",
        scope("structural-repair"),
        Position3::ORIGIN,
        &["junction"],
    );
    assert!(
        nearby_semantic_examples(&catalog, &request(None))
            .unwrap()
            .is_empty()
    );
}

#[test]
fn exact_canon_resolution_is_bounded_sorted_and_fail_closed() {
    let catalog = CompositionCatalog::new();
    let retriever = CatalogCanonRetriever::new(&catalog, canon()).unwrap();
    let bundle = retriever
        .retrieve(&request(Some(Position3::ORIGIN)))
        .unwrap();
    assert_eq!(bundle.canon.len(), 1);
    assert_eq!(
        bundle.canon[0].reference,
        reference("constitution", "semantic-kernel-v1")
    );

    let mut unknown = request(Some(Position3::ORIGIN));
    unknown.constitutional_refs = vec![reference("constitution", "invented")];
    assert!(matches!(
        retriever.retrieve(&unknown),
        Err(RetrievalError::UnknownCanonReference(_))
    ));
}

#[test]
fn excessive_retrieval_limits_are_rejected() {
    let catalog = CompositionCatalog::new();
    let retriever = CatalogCanonRetriever::new(&catalog, canon()).unwrap();
    let mut too_much_canon = request(Some(Position3::ORIGIN));
    too_much_canon.maximum_canon_fragments = MAX_RETRIEVED_CANON_FRAGMENTS + 1;
    assert!(matches!(
        retriever.retrieve(&too_much_canon),
        Err(RetrievalError::CanonLimitExceeded { .. })
    ));
    let mut too_many_examples = request(Some(Position3::ORIGIN));
    too_many_examples.maximum_nearby_examples = MAX_NEARBY_SEMANTIC_EXAMPLES + 1;
    assert!(matches!(
        retriever.retrieve(&too_many_examples),
        Err(RetrievalError::NearbyLimitExceeded { .. })
    ));
}

#[test]
fn insertion_order_does_not_change_retrieval_results() {
    let build = |reverse: bool| {
        let mut catalog = CompositionCatalog::new();
        let entries = if reverse {
            [("record.semantic.b", "b"), ("record.semantic.a", "a")]
        } else {
            [("record.semantic.a", "a"), ("record.semantic.b", "b")]
        };
        for (id, point) in entries {
            insert_position(
                &mut catalog,
                id,
                point,
                scope("structural-repair"),
                Position3::from_raw(100, 0, 100).unwrap(),
                &["support"],
            );
        }
        nearby_semantic_examples(&catalog, &request(Some(Position3::ORIGIN))).unwrap()
    };
    assert_eq!(build(false), build(true));
}
