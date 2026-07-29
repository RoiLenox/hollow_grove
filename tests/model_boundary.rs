use std::future::Future;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::time::Duration;

use hollow_grove::guide_identity::{
    DualGuideSession, GUIDE_MODEL_PROMPT_LAW, StableGuideSessionId,
};
use hollow_grove::runtime_federation::{
    HOLLOW_GROVE_SEMANTIC_PROMPT_V1, HOLLOW_GROVE_SEMANTIC_PROMPT_VERSION, InterpreterError,
    MAX_MODEL_RESPONSE_BYTES, ModelSemanticProposal, OllamaConfig, OllamaConfigError,
    OllamaSemanticInterpreter, OllamaTransport, SemanticInterpretationRequest, SemanticInterpreter,
    parse_model_semantic_proposal,
};
use hollow_grove::{ExternalRef, ScaleKey, SemanticScope};

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

fn valid_json() -> Vec<u8> {
    br#"{
        "scale_key":"structural-repair",
        "proposed_position":{"x":-120,"y":0,"z":580},
        "operation":"repair",
        "evidence_refs":["evidence.inspection.041"],
        "semantic_tags":["support","repair","embodiment"],
        "summary":"The request is structural and embodied, with engineering support.",
        "confidence_millis":870
    }"#
    .to_vec()
}

fn request() -> SemanticInterpretationRequest {
    SemanticInterpretationRequest {
        prompt_version: HOLLOW_GROVE_SEMANTIC_PROMPT_VERSION.into(),
        user_intent: "Restore the broken supporting frame.".into(),
        point_ref: ExternalRef::new("point", "object.support-column.041").unwrap(),
        scope: SemanticScope::new(ScaleKey::new("structural-repair").unwrap(), None),
        current_position: None,
        retrieved_canon: vec![],
        nearby_examples: vec![],
        evidence_refs: vec![ExternalRef::new("evidence", "inspection.041").unwrap()],
        active_guides: DualGuideSession::outside(
            StableGuideSessionId::new("session.model-boundary").unwrap(),
        )
        .guide_context_v1()
        .unwrap(),
        guide_prompt_law: GUIDE_MODEL_PROMPT_LAW.into(),
    }
}

#[test]
fn strict_model_parser_accepts_only_the_bounded_schema() {
    let parsed = parse_model_semantic_proposal(&valid_json()).unwrap();
    assert_eq!(parsed.proposed_position.y().get(), 0);
    assert_eq!(parsed.confidence_millis, 870);

    let unknown = String::from_utf8(valid_json()).unwrap().replace(
        "\"confidence_millis\":870",
        "\"confidence_millis\":870,\"extra\":true",
    );
    assert!(matches!(
        parse_model_semantic_proposal(unknown.as_bytes()),
        Err(InterpreterError::UnknownModelField(field)) if field == "extra"
    ));
    let guide_override = String::from_utf8(valid_json()).unwrap().replace(
        "\"confidence_millis\":870",
        "\"confidence_millis\":870,\"display_name\":\"Roi\",\"guide_form\":\"seer\"",
    );
    assert!(matches!(
        parse_model_semantic_proposal(guide_override.as_bytes()),
        Err(InterpreterError::UnknownModelField(field))
            if field == "display_name" || field == "guide_form"
    ));

    let missing = String::from_utf8(valid_json()).unwrap().replace(
        "\"summary\":\"The request is structural and embodied, with engineering support.\",",
        "",
    );
    assert!(matches!(
        parse_model_semantic_proposal(missing.as_bytes()),
        Err(InterpreterError::MissingModelField(field)) if field == "summary"
    ));
}

#[test]
fn coordinate_and_operation_attacks_fail_during_parsing() {
    for invalid in [
        String::from_utf8(valid_json())
            .unwrap()
            .replace("\"x\":-120", "\"x\":-1001"),
        String::from_utf8(valid_json())
            .unwrap()
            .replace("\"y\":0", "\"y\":1"),
        String::from_utf8(valid_json())
            .unwrap()
            .replace("\"z\":580", "\"z\":1001"),
        String::from_utf8(valid_json())
            .unwrap()
            .replace("\"x\":-120", "\"x\":1.5"),
        String::from_utf8(valid_json())
            .unwrap()
            .replace("\"operation\":\"repair\"", "\"operation\":\"shell\""),
    ] {
        assert!(parse_model_semantic_proposal(invalid.as_bytes()).is_err());
    }
}

#[test]
fn response_summary_tags_and_confidence_are_bounded() {
    let oversized_wire = vec![b' '; MAX_MODEL_RESPONSE_BYTES + 1];
    assert!(matches!(
        parse_model_semantic_proposal(&oversized_wire),
        Err(InterpreterError::ResponseTooLarge { .. })
    ));

    let mut value: serde_json::Value = serde_json::from_slice(&valid_json()).unwrap();
    value["summary"] = "x".repeat(2_049).into();
    assert!(matches!(
        parse_model_semantic_proposal(&serde_json::to_vec(&value).unwrap()),
        Err(InterpreterError::ModelFieldTooLarge {
            field: "summary",
            ..
        })
    ));
    value["summary"] = "bounded".into();
    value["semantic_tags"] =
        serde_json::Value::Array((0..33).map(|index| format!("tag-{index}").into()).collect());
    assert!(matches!(
        parse_model_semantic_proposal(&serde_json::to_vec(&value).unwrap()),
        Err(InterpreterError::TooManySemanticTags { .. })
    ));
    value["semantic_tags"] = serde_json::json!(["bounded"]);
    value["confidence_millis"] = 1001.into();
    assert!(matches!(
        parse_model_semantic_proposal(&serde_json::to_vec(&value).unwrap()),
        Err(InterpreterError::ConfidenceOutOfBounds(1001))
    ));
}

#[derive(Clone)]
struct MockTransport {
    response: Result<Vec<u8>, InterpreterError>,
    request_body: Arc<Mutex<Vec<u8>>>,
}

impl OllamaTransport for MockTransport {
    fn post_json<'a>(
        &'a self,
        _endpoint: &'a str,
        payload: &'a [u8],
        _timeout: Duration,
        _maximum_response_bytes: usize,
    ) -> std::pin::Pin<Box<dyn Future<Output = Result<Vec<u8>, InterpreterError>> + Send + 'a>>
    {
        Box::pin(async move {
            *self.request_body.lock().unwrap() = payload.to_vec();
            self.response.clone()
        })
    }
}

#[test]
fn ollama_adapter_sends_versioned_prompt_and_parses_only_nested_json() {
    let body = Arc::new(Mutex::new(Vec::new()));
    let response = serde_json::to_vec(&serde_json::json!({
        "model": "test-model",
        "done": true,
        "response": String::from_utf8(valid_json()).unwrap()
    }))
    .unwrap();
    let interpreter = OllamaSemanticInterpreter::with_transport(
        OllamaConfig::new(
            "http://127.0.0.1:11434/api/generate",
            "test-model",
            Duration::from_secs(1),
        )
        .unwrap(),
        MockTransport {
            response: Ok(response),
            request_body: Arc::clone(&body),
        },
    )
    .unwrap();
    let proposal = block_on(interpreter.propose(&request())).unwrap();
    assert_eq!(
        proposal,
        parse_model_semantic_proposal(&valid_json()).unwrap()
    );
    assert_eq!(interpreter.version(), "ollama:test-model");

    let sent: serde_json::Value = serde_json::from_slice(&body.lock().unwrap()).unwrap();
    assert_eq!(sent["model"], "test-model");
    assert_eq!(sent["stream"], false);
    assert_eq!(sent["format"], "json");
    let system = sent["system"].as_str().unwrap();
    assert!(system.starts_with(HOLLOW_GROVE_SEMANTIC_PROMPT_V1.trim_end()));
    assert!(system.contains(GUIDE_MODEL_PROMPT_LAW));
    assert!(system.contains("Do not invent a generic third Guide."));
    let prompt: serde_json::Value = serde_json::from_str(sent["prompt"].as_str().unwrap()).unwrap();
    assert_eq!(
        prompt["active_guide_context"]["guides"][0]["display_name"],
        "Roï"
    );
    assert_eq!(
        prompt["active_guide_context"]["guides"][1]["display_name"],
        "Rosaliña"
    );
    assert_eq!(prompt["guide_identity_law"], GUIDE_MODEL_PROMPT_LAW);
    assert!(
        sent["prompt"]
            .as_str()
            .unwrap()
            .contains("evidence.inspection.041")
    );
}

#[test]
fn ollama_failures_remain_interpreter_failures() {
    for error in [
        InterpreterError::InterpreterTimeout,
        InterpreterError::InterpreterUnavailable,
    ] {
        let interpreter = OllamaSemanticInterpreter::with_transport(
            OllamaConfig::new(
                "http://localhost:11434/api/generate",
                "test-model",
                Duration::from_secs(1),
            )
            .unwrap(),
            MockTransport {
                response: Err(error.clone()),
                request_body: Arc::new(Mutex::new(Vec::new())),
            },
        )
        .unwrap();
        assert_eq!(block_on(interpreter.propose(&request())), Err(error));
    }
}

#[test]
fn ollama_configuration_is_external_and_loopback_only() {
    assert!(matches!(
        OllamaConfig::new(
            "https://127.0.0.1:11434/api/generate",
            "model",
            Duration::from_secs(1)
        ),
        Err(OllamaConfigError::InvalidEndpoint(_))
    ));
    assert!(matches!(
        OllamaConfig::new(
            "http://example.com:11434/api/generate",
            "model",
            Duration::from_secs(1)
        ),
        Err(OllamaConfigError::NonLocalEndpoint(_))
    ));
    assert!(matches!(
        OllamaConfig::new(
            "http://127.0.0.1:11434/api/generate",
            "",
            Duration::from_secs(1)
        ),
        Err(OllamaConfigError::InvalidModel(_))
    ));
    assert!(
        HOLLOW_GROVE_SEMANTIC_PROMPT_V1
            .contains("Do not use Central Junction as a default for uncertainty.")
    );
}

#[test]
fn deterministic_interpreter_can_simulate_malformed_and_reserved_y_responses() {
    for bytes in [
        b"{".as_slice(),
        br#"{
            "scale_key":"structural-repair",
            "proposed_position":{"x":0,"y":9,"z":0},
            "operation":"repair",
            "evidence_refs":[],
            "semantic_tags":[],
            "summary":"invalid y",
            "confidence_millis":500
        }"#
        .as_slice(),
    ] {
        let interpreter =
            hollow_grove::runtime_federation::DeterministicSemanticInterpreter::from_json(bytes);
        assert!(block_on(interpreter.propose(&request())).is_err());
    }
}

#[test]
fn model_type_remains_a_proposal_not_an_authoritative_record() {
    let proposal: ModelSemanticProposal = parse_model_semantic_proposal(&valid_json()).unwrap();
    assert_eq!(proposal.scale_key, "structural-repair");
    assert!(proposal.summary.contains("structural"));
}
