use std::fs;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use hollow_grove::{
    Position3, SemanticDatasetDecision, SemanticDatasetScope, SemanticTrainingCategory,
    SemanticWitnessDatasetRecord, format_semantic_dataset_record, inspect_semantic_dataset,
    semantic_dataset_history, semantic_dataset_nearby,
};

fn record(
    witness: &str,
    point: &str,
    scale: &str,
    decision: SemanticDatasetDecision,
    position: Position3,
) -> SemanticWitnessDatasetRecord {
    let prior = Position3::from_raw(-80, 0, -310).unwrap();
    SemanticWitnessDatasetRecord {
        schema_version: "hollow-grove-semantic-witness-v1".into(),
        witness_ref: format!("witness.{witness}"),
        proposal_ref: format!("proposal.{witness}"),
        cycle_id: format!("cycle.{witness}"),
        point_ref: point.into(),
        input: "Restore the broken supporting frame.".into(),
        scope: SemanticDatasetScope {
            scale_key: scale.into(),
            context_ref: Some("context.inspection.041".into()),
        },
        prior_position: Some(prior),
        proposed_position: position,
        translation: Some(prior.translation_to(position).unwrap()),
        confirmed_position: (decision == SemanticDatasetDecision::Confirmed).then_some(position),
        operation: "repair".into(),
        decision,
        rejection_reasons: if decision == SemanticDatasetDecision::Rejected {
            vec!["Validation(InsufficientEvidence)".into()]
        } else {
            Vec::new()
        },
        evidence_refs: vec!["evidence.inspection.041".into()],
        constitutional_refs: vec!["constitution.semantic-kernel-v1".into()],
        semantic_tags: vec!["repair".into()],
        training_categories: vec![if decision == SemanticDatasetDecision::Confirmed {
            SemanticTrainingCategory::Translation
        } else {
            SemanticTrainingCategory::EvidenceRejection
        }],
        summary: "A concise witnessed decision.".into(),
        prompt_version: "hollow-grove-semantic-interpreter-v1".into(),
        interpreter_version: "deterministic-semantic-interpreter-v1".into(),
    }
}

#[test]
fn confirmed_and_rejected_states_are_visibly_distinct() {
    let confirmed = record(
        "a-confirmed",
        "point.object.041",
        "structural-repair",
        SemanticDatasetDecision::Confirmed,
        Position3::from_raw(-120, 0, 580).unwrap(),
    );
    let rejected = record(
        "b-rejected",
        "point.object.041",
        "structural-repair",
        SemanticDatasetDecision::Rejected,
        Position3::from_raw(0, 0, 500).unwrap(),
    );
    let confirmed_text = format_semantic_dataset_record(&confirmed);
    assert!(confirmed_text.contains("Current position:"));
    assert!(confirmed_text.contains("Proposed position:"));
    assert!(confirmed_text.contains("Status:\nConfirmed"));
    assert!(confirmed_text.contains("Confirmed position:"));
    assert!(confirmed_text.contains("DX -40"));
    assert!(confirmed_text.contains("DZ +890"));
    assert!(confirmed_text.contains("CrossesOriginAxis"));
    assert!(confirmed_text.contains("Glaüshouse"));

    let rejected_text = format_semantic_dataset_record(&rejected);
    assert!(rejected_text.contains("Status:\nRejected"));
    assert!(rejected_text.contains("Reasons:"));
    assert!(rejected_text.contains("InsufficientEvidence"));
    assert!(!rejected_text.contains("Confirmed position:"));
}

#[test]
fn inspect_and_history_are_scoped_by_point_and_scale() {
    let records = vec![
        record(
            "a-confirmed",
            "point.object.041",
            "structural-repair",
            SemanticDatasetDecision::Confirmed,
            Position3::from_raw(-120, 0, 580).unwrap(),
        ),
        record(
            "b-rejected",
            "point.object.041",
            "structural-repair",
            SemanticDatasetDecision::Rejected,
            Position3::from_raw(0, 0, 500).unwrap(),
        ),
        record(
            "c-other",
            "point.object.041",
            "medical-diagnosis",
            SemanticDatasetDecision::Confirmed,
            Position3::from_raw(0, 0, -500).unwrap(),
        ),
    ];
    assert_eq!(
        inspect_semantic_dataset(&records, "point.object.041", "structural-repair")
            .unwrap()
            .witness_ref,
        "witness.b-rejected"
    );
    assert_eq!(
        semantic_dataset_history(&records, "point.object.041", "structural-repair").len(),
        2
    );
    assert!(inspect_semantic_dataset(&records, "point.missing", "structural-repair").is_err());
}

#[test]
fn nearby_presentation_uses_confirmed_positions_and_stable_order() {
    let records = vec![
        record(
            "z-other-close",
            "point.other",
            "medical-diagnosis",
            SemanticDatasetDecision::Confirmed,
            Position3::from_raw(1, 0, 0).unwrap(),
        ),
        record(
            "b-same",
            "point.same-b",
            "structural-repair",
            SemanticDatasetDecision::Confirmed,
            Position3::from_raw(100, 0, 0).unwrap(),
        ),
        record(
            "a-same",
            "point.same-a",
            "structural-repair",
            SemanticDatasetDecision::Confirmed,
            Position3::from_raw(-100, 0, 0).unwrap(),
        ),
        record(
            "rejected",
            "point.rejected",
            "structural-repair",
            SemanticDatasetDecision::Rejected,
            Position3::ORIGIN,
        ),
    ];
    let nearby = semantic_dataset_nearby(&records, "structural-repair", Position3::ORIGIN, 3);
    assert_eq!(
        nearby
            .iter()
            .map(|reading| reading.point_ref.as_str())
            .collect::<Vec<_>>(),
        ["point.same-a", "point.same-b", "point.other"]
    );
}

#[test]
fn semantic_cli_inspects_witnesses_without_mutating_them() {
    let source = record(
        "cli-confirmed",
        "point.object.041",
        "structural-repair",
        SemanticDatasetDecision::Confirmed,
        Position3::from_raw(-120, 0, 580).unwrap(),
    );
    let bytes = format!("{}\n", serde_json::to_string(&source).unwrap());
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let root = std::env::temp_dir().join(format!("hollow-grove-semantic-cli-{nonce}"));
    fs::create_dir_all(&root).unwrap();
    let path = root.join("semantic-witnesses-v1.jsonl");
    fs::write(&path, bytes.as_bytes()).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_hollow_grove_semantic"))
        .args([
            "semantic",
            "inspect",
            path.to_str().unwrap(),
            "point.object.041",
            "structural-repair",
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Status:\nConfirmed"));
    assert!(stdout.contains("Witness:\nwitness.cli-confirmed"));
    assert_eq!(fs::read(&path).unwrap(), bytes.as_bytes());

    fs::remove_dir_all(root).unwrap();
}
