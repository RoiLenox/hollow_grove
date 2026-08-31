use std::fs;
use std::future::Future;
use std::task::{Context, Poll, Waker};
use std::time::{SystemTime, UNIX_EPOCH};

use hollow_grove::composition::{ExternalRef, Position3, ScaleKey, SemanticWitnessDecision};
use hollow_grove::runtime_rampart::{
    DeterministicSemanticInterpreter, InterpreterError, LocalRampartCycleInput,
    ModelSemanticProposal, ProposedSemanticOperation, RampartApiCycleRequest, RampartApiDecision,
    RampartHttpApi,
};
use hollow_grove::semantic::encode_semantic_provenance_snapshot;

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

fn unique_temp_root(label: &str) -> std::path::PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("{label}-{}-{nonce}", std::process::id()))
}

fn request(intent: &str) -> RampartApiCycleRequest {
    RampartApiCycleRequest {
        point_ref: "point.hueman.origin".into(),
        scale_key: "structural-repair".into(),
        intent: intent.into(),
        context_ref: None,
    }
}

fn proposal_for(
    request: &RampartApiCycleRequest,
    position: Position3,
    cite_evidence: bool,
) -> ModelSemanticProposal {
    let point_ref = ExternalRef::new("point", "hueman.origin").unwrap();
    let input = LocalRampartCycleInput::new(
        point_ref,
        ScaleKey::new(&request.scale_key).unwrap(),
        &request.intent,
    )
    .unwrap();
    ModelSemanticProposal {
        scale_key: request.scale_key.clone(),
        proposed_position: position,
        operation: ProposedSemanticOperation::Repair,
        evidence_refs: if cite_evidence {
            let evidence = input.evidence_ref().unwrap();
            vec![format!("{}.{}", evidence.namespace, evidence.key)]
        } else {
            Vec::new()
        },
        semantic_tags: vec!["repair".into(), "support".into()],
        summary: "The request restores embodied structural support.".into(),
        confidence_millis: 870,
    }
}

#[test]
fn api_confirms_all_four_phases_and_durably_replays_the_result() {
    let root = unique_temp_root("hollow-grove-rampart-http-confirmed");
    let state = root.join("state.json");
    let request = request("Restore the supporting frame through Festival.");
    let position = Position3::from_raw(-120, 0, 580).unwrap();
    let interpreter =
        DeterministicSemanticInterpreter::responding_with(proposal_for(&request, position, true));
    let mut api = RampartHttpApi::from_state_path(interpreter, &state).unwrap();

    let response = block_on(api.execute(request)).unwrap();
    assert_eq!(response.schema_version, "hollow-grove-rampart-api-v1");
    assert_eq!(response.canonical_name, "The Rampart of Roi and Roselina");
    assert_eq!(
        response.phases,
        [
            "The Way Back",
            "The Initiation",
            "The Gathering",
            "The Festival"
        ]
    );
    assert!(matches!(
        response.decision,
        RampartApiDecision::Confirmed {
            confirmed_position,
            ..
        } if confirmed_position == position
    ));
    assert_eq!(response.next_position, Some(position));
    assert!(state.is_file());

    let replayed =
        RampartHttpApi::from_state_path(DeterministicSemanticInterpreter::unavailable(), &state)
            .unwrap();
    let point_ref = ExternalRef::new("point", "hueman.origin").unwrap();
    let scope = hollow_grove::composition::SemanticScope::new(
        ScaleKey::new("structural-repair").unwrap(),
        None,
    );
    assert_eq!(
        replayed
            .catalog()
            .current_semantic_position(&point_ref, &scope)
            .unwrap()
            .position,
        position
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn rejected_cycle_is_witnessed_but_never_becomes_current_position() {
    let root = unique_temp_root("hollow-grove-rampart-http-rejected");
    let state = root.join("state.json");
    let request = request("Repair without citing the supplied evidence.");
    let interpreter = DeterministicSemanticInterpreter::responding_with(proposal_for(
        &request,
        Position3::from_raw(0, 0, 500).unwrap(),
        false,
    ));
    let mut api = RampartHttpApi::from_state_path(interpreter, &state).unwrap();

    let response = block_on(api.execute(request)).unwrap();
    assert!(matches!(
        response.decision,
        RampartApiDecision::Rejected { .. }
    ));
    assert_eq!(response.next_position, None);
    assert!(api.catalog().semantic_position_records().is_empty());
    assert_eq!(api.catalog().semantic_witnesses().len(), 1);
    assert_eq!(
        api.catalog().semantic_witnesses()[0].decision,
        SemanticWitnessDecision::Rejected
    );
    assert!(state.is_file());
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn persistence_failure_rolls_back_the_in_memory_catalog() {
    let root = unique_temp_root("hollow-grove-rampart-http-rollback");
    fs::create_dir_all(&root).unwrap();
    let unwritable_target = root.join("state.json");
    let request = request("Restore support, but fail the durable commit.");
    let interpreter = DeterministicSemanticInterpreter::responding_with(proposal_for(
        &request,
        Position3::from_raw(0, 0, 700).unwrap(),
        true,
    ));
    let mut api = RampartHttpApi::from_state_path(interpreter, &unwritable_target).unwrap();
    let before = encode_semantic_provenance_snapshot(api.catalog()).unwrap();
    fs::create_dir(&unwritable_target).unwrap();

    let error = block_on(api.execute(request)).unwrap_err();
    assert_eq!(error.http_status(), 500);
    assert_eq!(
        encode_semantic_provenance_snapshot(api.catalog()).unwrap(),
        before
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn interpreter_failures_have_stable_gateway_statuses_and_no_mutation() {
    let cases = [
        (
            DeterministicSemanticInterpreter::timeout(),
            504,
            "interpreter_timeout",
        ),
        (
            DeterministicSemanticInterpreter::unavailable(),
            503,
            "interpreter_unavailable",
        ),
        (
            DeterministicSemanticInterpreter::failing_with(
                InterpreterError::MalformedModelResponse("not JSON".into()),
            ),
            502,
            "invalid_interpreter_response",
        ),
    ];
    for (index, (interpreter, expected_status, expected_type)) in cases.into_iter().enumerate() {
        let root = unique_temp_root(&format!("hollow-grove-rampart-http-error-{index}"));
        let state = root.join("state.json");
        let mut api = RampartHttpApi::from_state_path(interpreter, &state).unwrap();
        let before = encode_semantic_provenance_snapshot(api.catalog()).unwrap();
        let error = block_on(api.execute(request("Interpret without mutation."))).unwrap_err();
        assert_eq!(error.http_status(), expected_status);
        assert_eq!(error.error_type(), expected_type);
        assert_eq!(
            encode_semantic_provenance_snapshot(api.catalog()).unwrap(),
            before
        );
        assert!(!state.exists());
    }
}

#[test]
fn request_schema_rejects_unknown_fields() {
    let error = serde_json::from_value::<RampartApiCycleRequest>(serde_json::json!({
        "point_ref": "point.hueman.origin",
        "scale_key": "structural-repair",
        "intent": "Restore the frame.",
        "canonical_position": {"x": 0, "y": 0, "z": 1000}
    }))
    .unwrap_err();
    assert!(error.to_string().contains("unknown field"));
}
