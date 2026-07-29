use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use hollow_grove::{
    CompositionCatalog, ExternalRef, Position3, SEMANTIC_WITNESS_SCHEMA_VERSION, ScaleKey,
    SemanticDatasetDecision, SemanticScope, SemanticTrainingCategory, SemanticWitnessDecision,
    SemanticWitnessRecord, export_semantic_witness_jsonl, validate_semantic_witness_jsonl,
    write_semantic_witness_dataset,
};

fn reference(namespace: &str, key: &str) -> ExternalRef {
    ExternalRef::new(namespace, key).unwrap()
}

fn scope() -> SemanticScope {
    SemanticScope::new(
        ScaleKey::new("structural-repair").unwrap(),
        Some(reference("context", "inspection.session.041")),
    )
}

fn confirmed_witness(key: &str) -> SemanticWitnessRecord {
    SemanticWitnessRecord {
        witness_ref: reference("witness", key),
        proposal_ref: reference("proposal", key),
        cycle_ref: reference("cycle", key),
        actor_ref: reference("actor", "repairer.041"),
        point_ref: reference("point", "object.support-column.041"),
        scope: scope(),
        input: "Restore the broken supporting frame.".into(),
        prior_position: Some(Position3::from_raw(-80, 0, -310).unwrap()),
        proposed_position: Position3::from_raw(-120, 0, 580).unwrap(),
        translation: Some(
            Position3::from_raw(-80, 0, -310)
                .unwrap()
                .translation_to(Position3::from_raw(-120, 0, 580).unwrap())
                .unwrap(),
        ),
        confirmed_position: Some(Position3::from_raw(-120, 0, 580).unwrap()),
        operation: reference("semantic-intent", "repair"),
        decision: SemanticWitnessDecision::Confirmed,
        evidence_refs: vec![reference("evidence", "inspection.041")],
        constitutional_refs: vec![reference("constitution", "semantic-kernel-v1")],
        composition_ref: Some(reference("composition-record", key)),
        rejection_reasons: vec![],
        semantic_tags: vec!["support".into(), "repair".into(), "embodiment".into()],
        summary: "The internal condition became restored embodied support.".into(),
        prompt_version: "hollow-grove-semantic-interpreter-v1".into(),
        interpreter_version: "deterministic-semantic-interpreter-v1".into(),
    }
}

fn rejected_witness(key: &str) -> SemanticWitnessRecord {
    SemanticWitnessRecord {
        witness_ref: reference("witness", key),
        proposal_ref: reference("proposal", key),
        cycle_ref: reference("cycle", key),
        actor_ref: reference("actor", "repairer.041"),
        point_ref: reference("point", "object.support-column.041"),
        scope: scope(),
        input: "Repair without an inspection.".into(),
        prior_position: None,
        proposed_position: Position3::from_raw(0, 0, 500).unwrap(),
        translation: None,
        confirmed_position: None,
        operation: reference("semantic-intent", "repair"),
        decision: SemanticWitnessDecision::Rejected,
        evidence_refs: vec![],
        constitutional_refs: vec![reference("constitution", "semantic-kernel-v1")],
        composition_ref: None,
        rejection_reasons: vec!["Validation(InsufficientEvidence)".into()],
        semantic_tags: vec!["repair".into()],
        summary: "The proposal lacked inspection evidence.".into(),
        prompt_version: "hollow-grove-semantic-interpreter-v1".into(),
        interpreter_version: "deterministic-semantic-interpreter-v1".into(),
    }
}

fn catalog(reverse: bool) -> CompositionCatalog {
    let mut catalog = CompositionCatalog::new();
    let records = if reverse {
        [
            rejected_witness("semantic.z-rejected"),
            confirmed_witness("semantic.a-confirmed"),
        ]
    } else {
        [
            confirmed_witness("semantic.a-confirmed"),
            rejected_witness("semantic.z-rejected"),
        ]
    };
    for witness in records {
        catalog.record_semantic_witness(witness).unwrap();
    }
    catalog
}

#[test]
fn confirmed_and_rejected_witnesses_export_as_valid_jsonl() {
    let bytes = export_semantic_witness_jsonl(&catalog(false)).unwrap();
    assert!(bytes.ends_with(b"\n"));
    assert_eq!(validate_semantic_witness_jsonl(&bytes).unwrap(), 2);
    let lines = std::str::from_utf8(&bytes)
        .unwrap()
        .lines()
        .collect::<Vec<_>>();
    let confirmed: hollow_grove::SemanticWitnessDatasetRecord =
        serde_json::from_str(lines[0]).unwrap();
    let rejected: hollow_grove::SemanticWitnessDatasetRecord =
        serde_json::from_str(lines[1]).unwrap();
    assert_eq!(confirmed.schema_version, SEMANTIC_WITNESS_SCHEMA_VERSION);
    assert_eq!(confirmed.decision, SemanticDatasetDecision::Confirmed);
    assert_eq!(
        confirmed.training_categories,
        [SemanticTrainingCategory::Translation]
    );
    assert_eq!(rejected.decision, SemanticDatasetDecision::Rejected);
    assert_eq!(
        rejected.training_categories,
        [SemanticTrainingCategory::EvidenceRejection]
    );
    assert!(rejected.confirmed_position.is_none());
}

#[test]
fn export_is_byte_identical_and_independent_of_insertion_order() {
    let first = export_semantic_witness_jsonl(&catalog(false)).unwrap();
    let repeated = export_semantic_witness_jsonl(&catalog(false)).unwrap();
    let reversed = export_semantic_witness_jsonl(&catalog(true)).unwrap();
    assert_eq!(first, repeated);
    assert_eq!(first, reversed);
}

#[test]
fn dataset_contains_no_hidden_reasoning_fields() {
    let bytes = export_semantic_witness_jsonl(&catalog(false)).unwrap();
    let text = std::str::from_utf8(&bytes).unwrap().to_ascii_lowercase();
    assert!(!text.contains("chain_of_thought"));
    assert!(!text.contains("chain-of-thought"));
    assert!(!text.contains("hidden_reasoning"));
}

#[test]
fn validator_rejects_missing_newline_unknown_fields_and_schema_drift() {
    let bytes = export_semantic_witness_jsonl(&catalog(false)).unwrap();
    assert!(validate_semantic_witness_jsonl(&bytes[..bytes.len() - 1]).is_err());

    let first = std::str::from_utf8(&bytes).unwrap().lines().next().unwrap();
    let mut value: serde_json::Value = serde_json::from_str(first).unwrap();
    value["extra"] = true.into();
    let unknown = format!("{}\n", serde_json::to_string(&value).unwrap());
    assert!(validate_semantic_witness_jsonl(unknown.as_bytes()).is_err());
    value.as_object_mut().unwrap().remove("extra");
    value["schema_version"] = "hollow-grove-semantic-witness-v2".into();
    let future = format!("{}\n", serde_json::to_string(&value).unwrap());
    assert!(validate_semantic_witness_jsonl(future.as_bytes()).is_err());
}

#[test]
fn library_entry_point_writes_the_exact_export_bytes() {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let root = std::env::temp_dir().join(format!("hollow-grove-semantic-dataset-{nonce}"));
    let path = root.join("semantic-witnesses-v1.jsonl");
    write_semantic_witness_dataset(&path, &catalog(false)).unwrap();
    assert_eq!(
        fs::read(&path).unwrap(),
        export_semantic_witness_jsonl(&catalog(false)).unwrap()
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn pre_operation_witnesses_migrate_without_guessing_an_intent() {
    let witness = confirmed_witness("semantic.legacy");
    let mut value = serde_json::to_value(witness).unwrap();
    value.as_object_mut().unwrap().remove("operation");
    let migrated: SemanticWitnessRecord = serde_json::from_value(value).unwrap();
    assert_eq!(migrated.operation.namespace, "semantic-intent");
    assert_eq!(migrated.operation.key, "unspecified-v1");
}
