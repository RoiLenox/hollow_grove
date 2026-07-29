use std::fs;
use std::future::Future;
use std::process::Command;
use std::task::{Context, Poll, Waker};
use std::time::{SystemTime, UNIX_EPOCH};

use hollow_grove::composition::{
    CompositionCatalog, ExternalRef, Position3, ScaleKey, SemanticWitnessDecision,
};
use hollow_grove::constitutional::GrovePhase;
use hollow_grove::runtime_federation::{
    DeterministicSemanticInterpreter, LocalFederationCycleInput, ModelSemanticProposal,
    ProposedSemanticOperation, SemanticDecision, run_local_federation_cycle,
};
use hollow_grove::world::composition_witnesses::canonical_composition_witness_catalog;
use hollow_grove::{decode_semantic_provenance_snapshot, encode_semantic_provenance_snapshot};

fn block_on<F: Future>(future: F) -> F::Output {
    let waker = Waker::noop();
    let mut context = Context::from_waker(waker);
    let mut future = std::pin::pin!(future);
    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(output) => return output,
            Poll::Pending => std::thread::yield_now(),
        }
    }
}

fn point(key: &str) -> ExternalRef {
    ExternalRef::new("point", key).unwrap()
}

fn input(intent: &str) -> LocalFederationCycleInput {
    LocalFederationCycleInput::new(
        point("hueman.origin"),
        ScaleKey::new("structural-repair").unwrap(),
        intent,
    )
    .unwrap()
}

fn proposal(
    input: &LocalFederationCycleInput,
    position: Position3,
    cite_evidence: bool,
) -> ModelSemanticProposal {
    ModelSemanticProposal {
        scale_key: input.scale.as_str().into(),
        proposed_position: position,
        operation: ProposedSemanticOperation::Repair,
        evidence_refs: if cite_evidence {
            let evidence = input.evidence_ref().unwrap();
            vec![format!("{}.{}", evidence.namespace, evidence.key)]
        } else {
            Vec::new()
        },
        semantic_tags: vec!["repair".into(), "support".into()],
        summary: "The intent restores an embodied supporting structure.".into(),
        confidence_millis: 850,
    }
}

fn catalog() -> CompositionCatalog {
    canonical_composition_witness_catalog().unwrap()
}

fn unique_temp_root(label: &str) -> std::path::PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("{label}-{}-{nonce}", std::process::id()))
}

#[test]
fn local_command_runs_all_phases_and_confirms_into_the_existing_catalog() {
    let mut catalog = catalog();
    let input = input("Restore the supporting frame.");
    let position = Position3::from_raw(-120, 0, 580).unwrap();
    let result = block_on(run_local_federation_cycle(
        DeterministicSemanticInterpreter::responding_with(proposal(&input, position, true)),
        &input,
        &mut catalog,
    ))
    .unwrap();

    assert_eq!(result.phase_trace, GrovePhase::ALL);
    let SemanticDecision::Confirmed(confirmed) = result.festival.decision else {
        panic!("resolved evidence and authority should confirm");
    };
    assert_eq!(confirmed.confirmed_position, position);
    assert_eq!(result.next_way_back.current_position, Some(position));
    assert_eq!(catalog.semantic_position_records().len(), 1);
    assert_eq!(catalog.semantic_witnesses().len(), 1);
    let witness = &catalog.semantic_witnesses()[0];
    assert_eq!(witness.decision, SemanticWitnessDecision::Confirmed);
    assert_eq!(
        witness.constitutional_refs,
        [ExternalRef::new("constitution", "semantic-kernel-v1").unwrap()]
    );
}

#[test]
fn unsupported_model_proposal_is_witnessed_without_position_mutation() {
    let mut catalog = catalog();
    let input = input("Repair without citing the supplied request evidence.");
    let result = block_on(run_local_federation_cycle(
        DeterministicSemanticInterpreter::responding_with(proposal(
            &input,
            Position3::from_raw(0, 0, 500).unwrap(),
            false,
        )),
        &input,
        &mut catalog,
    ))
    .unwrap();

    assert!(matches!(
        result.festival.decision,
        SemanticDecision::Rejected(_)
    ));
    assert!(catalog.semantic_position_records().is_empty());
    assert!(
        catalog
            .composition_records()
            .iter()
            .all(|record| { !record.id.as_str().starts_with("record.semantic.") })
    );
    assert_eq!(catalog.semantic_witnesses().len(), 1);
    assert_eq!(
        catalog.semantic_witnesses()[0].decision,
        SemanticWitnessDecision::Rejected
    );
}

#[test]
fn unknown_point_and_interpreter_failure_do_not_mutate_state() {
    let mut unknown_catalog = catalog();
    let unknown_input = LocalFederationCycleInput::new(
        point("not-in-catalog"),
        ScaleKey::new("structural-repair").unwrap(),
        "Locate an unknown Point.",
    )
    .unwrap();
    let before = encode_semantic_provenance_snapshot(&unknown_catalog).unwrap();
    assert!(
        block_on(run_local_federation_cycle(
            DeterministicSemanticInterpreter::responding_with(proposal(
                &unknown_input,
                Position3::ORIGIN,
                true,
            )),
            &unknown_input,
            &mut unknown_catalog,
        ))
        .is_err()
    );
    assert_eq!(
        encode_semantic_provenance_snapshot(&unknown_catalog).unwrap(),
        before
    );

    let mut failed_catalog = catalog();
    let valid_input = input("Interpret a malformed response.");
    let before = encode_semantic_provenance_snapshot(&failed_catalog).unwrap();
    assert!(
        block_on(run_local_federation_cycle(
            DeterministicSemanticInterpreter::from_json(b"not-json"),
            &valid_input,
            &mut failed_catalog,
        ))
        .is_err()
    );
    assert_eq!(
        encode_semantic_provenance_snapshot(&failed_catalog).unwrap(),
        before
    );
}

#[test]
fn federation_cli_persists_and_replays_a_confirmed_cycle_without_ollama() {
    let root = unique_temp_root("hollow-grove-federation-cli");
    fs::create_dir_all(&root).unwrap();
    let state = root.join("semantic-state.json");
    let proposal_path = root.join("proposal.json");
    let input = input("Restore the supporting frame through Festival.");
    let destination = Position3::from_raw(-120, 0, 580).unwrap();
    fs::write(
        &proposal_path,
        serde_json::to_vec(&proposal(&input, destination, true)).unwrap(),
    )
    .unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_hollow_grove_semantic"))
        .args([
            "federation",
            "run",
            "point.hueman.origin",
            "structural-repair",
            &input.user_intent,
            "--state",
            state.to_str().unwrap(),
            "--proposal-json",
            proposal_path.to_str().unwrap(),
            "--json",
        ])
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    let rendered: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(
        rendered["schema_version"],
        "hollow-grove-federation-cli-result-v1"
    );
    assert_eq!(
        rendered["phases"],
        serde_json::json!([
            "The Way Back",
            "The Initiation",
            "The Gathering",
            "The Festival"
        ])
    );
    assert_eq!(rendered["decision"]["status"], "confirmed");
    assert_eq!(
        rendered["decision"]["confirmed_position"],
        serde_json::json!({"x": -120, "y": 0, "z": 580})
    );

    let mut replayed = catalog();
    decode_semantic_provenance_snapshot(&fs::read(&state).unwrap(), &mut replayed).unwrap();
    assert_eq!(
        replayed
            .current_semantic_position(&input.point_ref, &input.scope())
            .unwrap()
            .position,
        destination
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn root_cli_delegates_federation_help_without_contacting_ollama() {
    let output = Command::new(env!("CARGO_BIN_EXE_hollow-grove"))
        .args(["federation", "--help"])
        .output()
        .unwrap();

    assert!(output.status.success(), "{output:?}");
    assert!(
        String::from_utf8_lossy(&output.stdout)
            .contains("federation run <point-ref> <scale> <intent>")
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("federation serve"));
}
