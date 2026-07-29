use std::collections::BTreeSet;
use std::fs;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use hollow_grove::{
    Position3, SemanticDatasetDecision, SemanticDatasetScope, SemanticTrainingCategory,
    SemanticWitnessDatasetRecord, decode_semantic_witness_jsonl, split_semantic_witness_jsonl,
};

fn record(index: usize) -> SemanticWitnessDatasetRecord {
    SemanticWitnessDatasetRecord {
        schema_version: "hollow-grove-semantic-witness-v1".into(),
        witness_ref: format!("witness.semantic.{index:03}"),
        proposal_ref: format!("proposal.semantic.{index:03}"),
        cycle_id: format!("cycle.semantic.{index:03}"),
        point_ref: format!("point.object.{index:03}"),
        input: format!("Witnessed semantic example {index}."),
        scope: SemanticDatasetScope {
            scale_key: "training-preparation".into(),
            context_ref: None,
        },
        prior_position: None,
        proposed_position: Position3::from_raw((index as i16 % 100) - 50, 0, 500).unwrap(),
        translation: None,
        confirmed_position: Some(Position3::from_raw((index as i16 % 100) - 50, 0, 500).unwrap()),
        operation: "locate".into(),
        decision: SemanticDatasetDecision::Confirmed,
        rejection_reasons: vec![],
        evidence_refs: vec![format!("evidence.training.{index:03}")],
        constitutional_refs: vec!["constitution.semantic-kernel-v1".into()],
        semantic_tags: vec!["training-preparation".into()],
        training_categories: vec![SemanticTrainingCategory::PositionAssertion],
        summary: "A concise witnessed example.".into(),
        prompt_version: "hollow-grove-semantic-interpreter-v1".into(),
        interpreter_version: "deterministic-semantic-interpreter-v1".into(),
    }
}

fn dataset(count: usize, reverse: bool) -> Vec<u8> {
    let mut records = (0..count).map(record).collect::<Vec<_>>();
    if reverse {
        records.reverse();
    }
    let mut output = Vec::new();
    for record in records {
        output.extend(serde_json::to_vec(&record).unwrap());
        output.push(b'\n');
    }
    output
}

#[test]
fn split_is_seeded_stable_ordered_and_byte_identical() {
    let first = split_semantic_witness_jsonl(&dataset(100, false), 410_041, 100).unwrap();
    let repeated = split_semantic_witness_jsonl(&dataset(100, false), 410_041, 100).unwrap();
    let reversed = split_semantic_witness_jsonl(&dataset(100, true), 410_041, 100).unwrap();
    let different_seed = split_semantic_witness_jsonl(&dataset(100, false), 410_042, 100).unwrap();
    assert_eq!(first, repeated);
    assert_eq!(first, reversed);
    assert!(
        first.train_bytes != different_seed.train_bytes
            || first.validation_bytes != different_seed.validation_bytes
    );
    assert!(first.train_bytes.is_empty() || first.train_bytes.ends_with(b"\n"));
    assert!(first.validation_bytes.is_empty() || first.validation_bytes.ends_with(b"\n"));
    assert_eq!(first.train_records + first.validation_records, 100);
}

#[test]
fn split_preserves_every_witness_exactly_once() {
    let split = split_semantic_witness_jsonl(&dataset(100, false), 7, 250).unwrap();
    let mut witnesses = decode_semantic_witness_jsonl(&split.train_bytes)
        .unwrap()
        .into_iter()
        .map(|record| record.witness_ref)
        .collect::<BTreeSet<_>>();
    for record in decode_semantic_witness_jsonl(&split.validation_bytes).unwrap() {
        assert!(witnesses.insert(record.witness_ref));
    }
    assert_eq!(witnesses.len(), 100);
}

#[test]
fn split_rejects_invalid_share_and_duplicate_witnesses() {
    assert!(split_semantic_witness_jsonl(&dataset(1, false), 1, 1001).is_err());
    let one = dataset(1, false);
    let duplicate = [one.as_slice(), one.as_slice()].concat();
    assert!(split_semantic_witness_jsonl(&duplicate, 1, 100).is_err());
}

#[test]
fn semantic_cli_writes_the_seeded_split() {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let root = std::env::temp_dir().join(format!("hollow-grove-semantic-split-{nonce}"));
    fs::create_dir_all(&root).unwrap();
    let input = root.join("input.jsonl");
    let train = root.join("train.jsonl");
    let validation = root.join("validation.jsonl");
    fs::write(&input, dataset(40, false)).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_hollow_grove_semantic"))
        .args([
            "dataset",
            "split-semantic",
            input.to_str().unwrap(),
            train.to_str().unwrap(),
            validation.to_str().unwrap(),
            "410041",
            "100",
        ])
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    let expected = split_semantic_witness_jsonl(&dataset(40, false), 410_041, 100).unwrap();
    assert_eq!(fs::read(&train).unwrap(), expected.train_bytes);
    assert_eq!(fs::read(&validation).unwrap(), expected.validation_bytes);

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn shared_adapter_template_is_disabled_and_not_house_personas() {
    let template = fs::read_to_string("ai/training/hollow-grove-adapter-v1.example.json").unwrap();
    let value: serde_json::Value = serde_json::from_str(&template).unwrap();
    assert_eq!(value["architecture"], "one_shared_adapter");
    assert_eq!(value["training_enabled"], false);
    assert_eq!(value["reserved_y_value"], 0);
    assert!(!template.contains("Stonebend chatbot"));
}
