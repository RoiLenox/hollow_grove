use hollow_grove::{
    CompositionCatalog, CompositionNode, CompositionNodeId, CompositionRecordId, ExternalRef,
    Position3, ScaleKey, SemanticOperation, SemanticPositionRecord, SemanticScope,
    SemanticWitnessDecision, SemanticWitnessRecord, Translation3, confirm_semantic_transition,
    decode_semantic_provenance_snapshot, encode_semantic_provenance_snapshot,
    record_rejected_semantic_witness,
};

fn reference(namespace: &str, key: impl Into<String>) -> ExternalRef {
    ExternalRef::new(namespace, key).unwrap()
}

fn point_node() -> CompositionNode {
    CompositionNode {
        id: CompositionNodeId::new("node.point.support-column-041").unwrap(),
        object: reference("point", "object.support-column.041"),
        scale: ScaleKey::new("scale.point").unwrap(),
    }
}

fn scope() -> SemanticScope {
    SemanticScope::new(
        ScaleKey::new("structural-repair").unwrap(),
        Some(reference("context", "inspection.session.041")),
    )
}

fn assertion_record() -> SemanticPositionRecord {
    let scope = scope();
    let position = Position3::from_raw(-80, 0, -310).unwrap();
    SemanticPositionRecord {
        record_id: CompositionRecordId::new("record.semantic.support-column-041.initial").unwrap(),
        point_ref: point_node().object,
        scope: scope.clone(),
        position,
        operation: SemanticOperation::PositionAssertion { scope, position },
        previous_record_id: None,
        evidence_refs: vec![reference("evidence", "inspection.041")],
        constitutional_refs: vec![reference("constitution", "semantic-kernel-v1")],
        witness_ref: reference("witness", "semantic.support-column-041.initial"),
        cycle_ref: reference("cycle", "semantic.support-column-041.initial"),
    }
}

fn translation_record() -> SemanticPositionRecord {
    let initial = assertion_record();
    let scope = scope();
    let from = initial.position;
    let to = Position3::from_raw(-120, 0, 580).unwrap();
    let translation = from.translation_to(to).unwrap();
    SemanticPositionRecord {
        record_id: CompositionRecordId::new("record.semantic.support-column-041.repair").unwrap(),
        point_ref: point_node().object,
        scope: scope.clone(),
        position: to,
        operation: SemanticOperation::SpatialTranslation {
            scope,
            from,
            translation,
            to,
        },
        previous_record_id: Some(initial.record_id),
        evidence_refs: vec![reference("evidence", "inspection.041")],
        constitutional_refs: vec![reference("constitution", "semantic-kernel-v1")],
        witness_ref: reference("witness", "semantic.support-column-041.repair"),
        cycle_ref: reference("cycle", "semantic.support-column-041.repair"),
    }
}

fn confirmed_witness(record: &SemanticPositionRecord, input: &str) -> SemanticWitnessRecord {
    let (prior_position, translation) = match record.operation {
        SemanticOperation::PositionAssertion { .. } => (None, None),
        SemanticOperation::SpatialTranslation {
            from, translation, ..
        } => (Some(from), Some(translation)),
    };
    SemanticWitnessRecord {
        witness_ref: record.witness_ref.clone(),
        proposal_ref: reference("proposal", record.record_id.as_str()),
        cycle_ref: record.cycle_ref.clone(),
        actor_ref: reference("actor", "tester"),
        point_ref: record.point_ref.clone(),
        scope: record.scope.clone(),
        input: input.into(),
        prior_position,
        proposed_position: record.position,
        translation,
        confirmed_position: Some(record.position),
        decision: SemanticWitnessDecision::Confirmed,
        evidence_refs: record.evidence_refs.clone(),
        constitutional_refs: record.constitutional_refs.clone(),
        composition_ref: Some(reference("composition-record", record.record_id.as_str())),
        rejection_reasons: vec![],
        semantic_tags: vec!["repair".into(), "embodiment".into()],
        summary: "A concise witnessed semantic decision.".into(),
        prompt_version: "hollow-grove-semantic-interpreter-v1".into(),
        interpreter_version: "deterministic-v1".into(),
    }
}

fn rejected_witness() -> SemanticWitnessRecord {
    SemanticWitnessRecord {
        witness_ref: reference("witness", "semantic.support-column-041.rejected"),
        proposal_ref: reference("proposal", "semantic.support-column-041.rejected"),
        cycle_ref: reference("cycle", "semantic.support-column-041.rejected"),
        actor_ref: reference("actor", "tester"),
        point_ref: point_node().object,
        scope: scope(),
        input: "Move without evidence.".into(),
        prior_position: Some(Position3::from_raw(-80, 0, -310).unwrap()),
        proposed_position: Position3::from_raw(0, 0, 500).unwrap(),
        translation: None,
        confirmed_position: None,
        decision: SemanticWitnessDecision::Rejected,
        evidence_refs: vec![],
        constitutional_refs: vec![reference("constitution", "semantic-kernel-v1")],
        composition_ref: None,
        rejection_reasons: vec!["InsufficientEvidence".into()],
        semantic_tags: vec!["repair".into()],
        summary: "The proposal lacked required inspection evidence.".into(),
        prompt_version: "hollow-grove-semantic-interpreter-v1".into(),
        interpreter_version: "deterministic-v1".into(),
    }
}

fn catalog() -> CompositionCatalog {
    let mut catalog = CompositionCatalog::new();
    catalog.insert_node(point_node()).unwrap();
    catalog
}

#[test]
fn confirmed_assertion_and_translation_create_causal_provenance_without_containment() {
    let mut catalog = catalog();
    let node_id = point_node().id;
    let identity_before = catalog.node(&node_id).unwrap().object.clone();

    let assertion = assertion_record();
    confirm_semantic_transition(
        &mut catalog,
        assertion.clone(),
        confirmed_witness(&assertion, "Locate the damaged support."),
    )
    .unwrap();
    let translation = translation_record();
    confirm_semantic_transition(
        &mut catalog,
        translation.clone(),
        confirmed_witness(&translation, "Restore the broken supporting frame."),
    )
    .unwrap();

    assert_eq!(catalog.node(&node_id).unwrap().object, identity_before);
    assert_eq!(
        catalog
            .current_semantic_position(&translation.point_ref, &translation.scope)
            .unwrap()
            .position,
        Position3::from_raw(-120, 0, 580).unwrap()
    );
    assert_eq!(
        catalog
            .semantic_position_history(&translation.point_ref, &translation.scope)
            .len(),
        2
    );
    assert_eq!(catalog.records_producing_result(&node_id).len(), 2);
    assert!(catalog.direct_containers(&node_id).is_empty());
    assert!(catalog.direct_members(&node_id).is_empty());
}

#[test]
fn stale_confirmation_and_rejection_leave_canonical_state_unchanged() {
    let mut catalog = catalog();
    let assertion = assertion_record();
    confirm_semantic_transition(
        &mut catalog,
        assertion.clone(),
        confirmed_witness(&assertion, "Locate the damaged support."),
    )
    .unwrap();
    let canonical_before = encode_semantic_provenance_snapshot(&catalog).unwrap();

    let mut stale = translation_record();
    let stale_from = Position3::ORIGIN;
    let stale_to = stale.position;
    stale.operation = SemanticOperation::SpatialTranslation {
        scope: stale.scope.clone(),
        from: stale_from,
        translation: stale_from.translation_to(stale_to).unwrap(),
        to: stale_to,
    };
    let stale_witness = confirmed_witness(&stale, "Use an obsolete source state.");
    assert!(confirm_semantic_transition(&mut catalog, stale, stale_witness).is_err());
    assert_eq!(
        encode_semantic_provenance_snapshot(&catalog).unwrap(),
        canonical_before
    );

    record_rejected_semantic_witness(&mut catalog, rejected_witness()).unwrap();
    assert_eq!(
        catalog
            .current_semantic_position(&assertion.point_ref, &assertion.scope)
            .unwrap()
            .position,
        assertion.position
    );
    assert_eq!(catalog.records_producing_result(&point_node().id).len(), 1);
    assert_eq!(catalog.semantic_witnesses().len(), 2);
}

#[test]
fn causal_provenance_replay_is_deterministic_and_idempotent() {
    let mut source = catalog();
    let assertion = assertion_record();
    confirm_semantic_transition(
        &mut source,
        assertion.clone(),
        confirmed_witness(&assertion, "Locate the damaged support."),
    )
    .unwrap();
    let translation = translation_record();
    confirm_semantic_transition(
        &mut source,
        translation.clone(),
        confirmed_witness(&translation, "Restore the broken supporting frame."),
    )
    .unwrap();
    record_rejected_semantic_witness(&mut source, rejected_witness()).unwrap();
    let bytes = encode_semantic_provenance_snapshot(&source).unwrap();

    let mut replayed = catalog();
    decode_semantic_provenance_snapshot(&bytes, &mut replayed).unwrap();
    assert_eq!(
        encode_semantic_provenance_snapshot(&replayed).unwrap(),
        bytes
    );
    decode_semantic_provenance_snapshot(&bytes, &mut replayed).unwrap();
    assert_eq!(
        encode_semantic_provenance_snapshot(&replayed).unwrap(),
        bytes
    );
    assert!(bytes.ends_with(b"}"));
}

#[test]
fn duplicate_stable_ids_and_ambiguous_point_projections_are_rejected() {
    let mut selected_catalog = catalog();
    let assertion = assertion_record();
    confirm_semantic_transition(
        &mut selected_catalog,
        assertion.clone(),
        confirmed_witness(&assertion, "Locate the damaged support."),
    )
    .unwrap();
    assert!(
        confirm_semantic_transition(
            &mut selected_catalog,
            assertion.clone(),
            confirmed_witness(&assertion, "Locate the damaged support."),
        )
        .is_err()
    );

    let mut ambiguous = catalog();
    ambiguous
        .insert_node(CompositionNode {
            id: CompositionNodeId::new("node.point.support-column-041.second").unwrap(),
            object: point_node().object,
            scale: ScaleKey::new("scale.second-projection").unwrap(),
        })
        .unwrap();
    let assertion = assertion_record();
    assert!(
        confirm_semantic_transition(
            &mut ambiguous,
            assertion.clone(),
            confirmed_witness(&assertion, "Locate the damaged support."),
        )
        .is_err()
    );
    assert!(ambiguous.semantic_position_records().is_empty());
}

#[test]
fn translation_witness_matches_checked_integer_movement() {
    let record = translation_record();
    let witness = confirmed_witness(&record, "Restore the broken supporting frame.");
    assert_eq!(
        witness.translation,
        Some(Translation3::from_raw(-40, 0, 890).unwrap())
    );
}
