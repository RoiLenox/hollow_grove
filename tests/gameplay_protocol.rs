use std::io::Write;
use std::process::{Command, Stdio};

use hollow_grove::constitutional::{RuleSetId, V2_RULE_SET};
use hollow_grove::gameplay::{
    CardinalDirection, EventAuthority, GameProtocolService, GameplayEventKind, GameplayIntent,
    HOLLOW_GROVE_GAMEPLAY_PROTOCOL_VERSION, ProtocolRejectionCode, ProtocolRequestEnvelope,
    ProtocolResponseEnvelope, ProtocolResponseStatus,
};

const SYNC_FIXTURE: &str = include_str!("../hueman_godot/protocol/fixtures/sync_request_v1.json");
const SYNC_RESPONSE_FIXTURE: &str =
    include_str!("../hueman_godot/protocol/fixtures/sync_response_v1.json");

fn rules() -> RuleSetId {
    RuleSetId::new(V2_RULE_SET).unwrap()
}

fn service() -> GameProtocolService {
    GameProtocolService::new("session.fixture", rules()).unwrap()
}

fn request(request_id: &str, revision: u64, intent: GameplayIntent) -> ProtocolRequestEnvelope {
    ProtocolRequestEnvelope {
        protocol_version: HOLLOW_GROVE_GAMEPLAY_PROTOCOL_VERSION,
        session_id: "session.fixture".into(),
        request_id: request_id.into(),
        expected_revision: revision,
        intent,
    }
}

fn establish_hueman(request_id: &str, revision: u64) -> ProtocolRequestEnvelope {
    request(
        request_id,
        revision,
        GameplayIntent::EstablishHuemanIntent {
            continuity_id: "being-continuity.hueman".into(),
            participant_id: "participant.hueman".into(),
            institutional_being_id: "being.hueman".into(),
        },
    )
}

fn assert_json_subset(expected: &serde_json::Value, actual: &serde_json::Value) {
    match (expected, actual) {
        (serde_json::Value::Object(expected), serde_json::Value::Object(actual)) => {
            for (key, expected_value) in expected {
                let actual_value = actual
                    .get(key)
                    .unwrap_or_else(|| panic!("missing additive-protocol fixture key: {key}"));
                assert_json_subset(expected_value, actual_value);
            }
        }
        (serde_json::Value::Array(expected), serde_json::Value::Array(actual)) => {
            assert!(
                actual.len() >= expected.len(),
                "protocol array removed fixture entries"
            );
            for (expected_value, actual_value) in expected.iter().zip(actual) {
                assert_json_subset(expected_value, actual_value);
            }
        }
        _ => assert_eq!(expected, actual),
    }
}

#[test]
fn shared_sync_fixture_round_trips_and_returns_an_authoritative_snapshot_view() {
    let request: ProtocolRequestEnvelope = serde_json::from_str(SYNC_FIXTURE).unwrap();
    assert_eq!(request.intent, GameplayIntent::SyncIntent);
    assert_eq!(
        serde_json::from_str::<ProtocolRequestEnvelope>(&serde_json::to_string(&request).unwrap())
            .unwrap(),
        request
    );

    let mut service = service();
    let response = service.handle(request);
    assert_eq!(response.status, ProtocolResponseStatus::Completed);
    assert_eq!(response.revision, 0);
    assert_eq!(response.events.len(), 1);
    assert_eq!(response.events[0].kind, GameplayEventKind::SnapshotLoaded);
    assert_eq!(response.events[0].authority, EventAuthority::Projection);
    let view = response.view.as_ref().unwrap();
    assert_eq!(view.revision, 0);
    assert_eq!(view.identity_count, 0);
    assert!(view.hueman.is_none());
    assert!(
        view.capabilities
            .iter()
            .any(|entry| entry.intent_type == "MoveIntent" && !entry.available)
    );
    assert_eq!(
        serde_json::from_str::<ProtocolResponseEnvelope>(
            &serde_json::to_string(&response).unwrap()
        )
        .unwrap(),
        response
    );
    // Protocol V1 permits additive view fields and capabilities. The shared
    // fixture remains the mandatory minimum projection and must stay an exact
    // subset of the richer living-world response.
    assert_json_subset(
        &serde_json::from_str::<serde_json::Value>(SYNC_RESPONSE_FIXTURE).unwrap(),
        &serde_json::to_value(&response).unwrap(),
    );
}

#[test]
fn hueman_intent_commits_one_canonical_event_and_updates_the_view() {
    let mut service = service();
    let response = service.handle(establish_hueman("request.create-hueman", 0));
    assert_eq!(response.status, ProtocolResponseStatus::Completed);
    assert_eq!(response.revision, 1);
    assert_eq!(response.events.len(), 1);
    assert_eq!(
        response.events[0].kind,
        GameplayEventKind::IdentityEstablished
    );
    assert_eq!(response.events[0].authority, EventAuthority::Canonical);
    assert_eq!(response.events[0].revision, 1);
    let view = response.view.as_ref().unwrap();
    let hueman = view.hueman.as_ref().unwrap();
    assert_eq!(hueman.continuity_id, "being-continuity.hueman");
    assert_eq!(hueman.incarnation_kind, "Hueman");
    assert!(view.capabilities.iter().any(|entry| {
        entry.intent_type == "EstablishHuemanIntent"
            && !entry.available
            && entry.unavailable_reason.is_some()
    }));
    assert_eq!(service.application().events().len(), 1);
}

#[test]
fn party_projection_opens_without_mutation_and_recruitment_fails_closed_before_the_arc() {
    let mut service = service();
    assert_eq!(
        service
            .handle(establish_hueman("request.party-hueman", 0))
            .status,
        ProtocolResponseStatus::Completed
    );
    let opened = service.handle(request(
        "request.open-party",
        1,
        GameplayIntent::OpenPartyIntent,
    ));
    assert_eq!(opened.status, ProtocolResponseStatus::Completed);
    assert_eq!(opened.revision, 1);
    assert!(opened.events.is_empty());
    let party = &opened.view.as_ref().unwrap().party;
    assert_eq!(party.member_count, 1);
    assert_eq!(party.max_members, 6);
    assert_eq!(party.lead_continuity_id, "being-continuity.hueman");

    let before_events = service.application().events().len();
    let rejected = service.handle(request(
        "request.early-recruit",
        1,
        GameplayIntent::RecruitIntent {
            target_id: "recruitment-candidate.riptide-pressure-keeper".into(),
            recruitment_path: "shared-work".into(),
        },
    ));
    assert_eq!(rejected.status, ProtocolResponseStatus::Rejected);
    assert_eq!(
        rejected.rejection.unwrap().code,
        ProtocolRejectionCode::RuntimeRejected
    );
    assert_eq!(rejected.revision, 1);
    assert_eq!(service.application().events().len(), before_events);
}

#[test]
fn movement_intent_updates_the_projected_overworld() {
    let mut service = service();
    assert_eq!(
        service
            .handle(establish_hueman("request.walk-hueman", 0))
            .status,
        ProtocolResponseStatus::Completed
    );

    let response = service.handle(request(
        "request.walk-north",
        1,
        GameplayIntent::MoveIntent {
            direction: CardinalDirection::North,
        },
    ));
    assert_eq!(response.status, ProtocolResponseStatus::Completed);
    assert_eq!(response.revision, 2);
    assert_eq!(response.events[0].kind, GameplayEventKind::MovementAccepted);
    let view = response.view.unwrap();
    let overworld = view.overworld.unwrap();
    assert_eq!(
        (overworld.width, overworld.height, overworld.tile_size),
        (20, 18, 8)
    );
    assert_eq!((overworld.player.x, overworld.player.y), (9, 14));
    assert_eq!(overworld.player.facing, CardinalDirection::North);
    assert!(
        view.capabilities
            .iter()
            .any(|entry| { entry.intent_type == "MoveIntent" && entry.available })
    );
}

#[test]
fn interaction_intent_opens_authoritative_handheld_dialogue() {
    let mut service = service();
    assert_eq!(
        service
            .handle(establish_hueman("request.talk-hueman", 0))
            .status,
        ProtocolResponseStatus::Completed
    );
    for (index, direction) in [
        CardinalDirection::West,
        CardinalDirection::West,
        CardinalDirection::West,
        CardinalDirection::West,
        CardinalDirection::North,
        CardinalDirection::North,
    ]
    .into_iter()
    .enumerate()
    {
        let expected_revision = u64::try_from(index).unwrap() + 1;
        let response = service.handle(request(
            &format!("request.talk-step-{index}"),
            expected_revision,
            GameplayIntent::MoveIntent { direction },
        ));
        assert_eq!(response.status, ProtocolResponseStatus::Completed);
    }

    let response = service.handle(request(
        "request.talk-guide",
        7,
        GameplayIntent::InteractIntent,
    ));
    assert_eq!(response.status, ProtocolResponseStatus::Completed);
    assert_eq!(response.revision, 8);
    assert_eq!(
        response.events[0].kind,
        GameplayEventKind::InteractionOpened
    );
    let interaction = response.events[0].interaction.as_ref().unwrap();
    assert_eq!(
        interaction.target_id,
        "interaction.aura-ridge.ridgefolk-guide"
    );
    assert_eq!(interaction.speaker, "RIDGEFOLK GUIDE");
    assert_eq!(
        response.view.as_ref().unwrap().interaction,
        Some(interaction.clone())
    );
    assert!(
        response
            .view
            .unwrap()
            .capabilities
            .iter()
            .any(|entry| { entry.intent_type == "InteractIntent" && entry.available })
    );
}

#[test]
fn request_retry_is_exact_and_conflicting_reuse_fails_closed() {
    let mut service = service();
    let request_envelope = establish_hueman("request.idempotent", 0);
    let expected = service.handle(request_envelope.clone());
    assert_eq!(service.handle(request_envelope), expected);
    assert_eq!(service.application().events().len(), 1);

    let conflict = service.handle(request("request.idempotent", 1, GameplayIntent::SyncIntent));
    assert_eq!(conflict.status, ProtocolResponseStatus::Rejected);
    assert_eq!(
        conflict.rejection.unwrap().code,
        ProtocolRejectionCode::RequestIdConflict
    );
    assert_eq!(service.application().events().len(), 1);
}

#[test]
fn stale_mutation_is_rejected_while_sync_recovers_the_current_snapshot() {
    let mut service = service();
    assert_eq!(
        service
            .handle(establish_hueman("request.first", 0))
            .revision,
        1
    );

    let stale = service.handle(request(
        "request.stale-move",
        0,
        GameplayIntent::MoveIntent {
            direction: CardinalDirection::North,
        },
    ));
    assert_eq!(stale.status, ProtocolResponseStatus::Rejected);
    assert_eq!(
        stale.rejection.unwrap().code,
        ProtocolRejectionCode::StaleRevision
    );
    assert_eq!(stale.view.unwrap().revision, 1);

    let sync = service.handle(request("request.recover", 0, GameplayIntent::SyncIntent));
    assert_eq!(sync.status, ProtocolResponseStatus::Completed);
    assert_eq!(sync.revision, 1);
    assert!(sync.view.unwrap().hueman.is_some());
    assert_eq!(service.application().events().len(), 1);
}

#[test]
fn movement_requires_hueman_without_mutating_the_runtime() {
    let mut service = service();
    let response = service.handle(request(
        "request.move",
        0,
        GameplayIntent::MoveIntent {
            direction: CardinalDirection::East,
        },
    ));
    assert_eq!(response.status, ProtocolResponseStatus::Rejected);
    assert_eq!(
        response.rejection.unwrap().code,
        ProtocolRejectionCode::CapabilityUnavailable
    );
    assert_eq!(response.revision, 0);
    assert!(service.application().events().is_empty());
}

#[test]
fn version_session_and_malformed_requests_fail_without_exposing_mutable_state() {
    let mut service = service();
    let mut wrong_version = request("request.version", 0, GameplayIntent::SyncIntent);
    wrong_version.protocol_version += 1;
    let response = service.handle(wrong_version);
    assert_eq!(
        response.rejection.unwrap().code,
        ProtocolRejectionCode::UnsupportedProtocolVersion
    );
    assert!(response.view.is_none());

    let mut wrong_session = request("request.session", 0, GameplayIntent::SyncIntent);
    wrong_session.session_id = "session.other".into();
    let response = service.handle(wrong_session);
    assert_eq!(
        response.rejection.unwrap().code,
        ProtocolRejectionCode::InvalidSession
    );
    assert!(response.view.is_none());

    let response: ProtocolResponseEnvelope =
        serde_json::from_str(&service.handle_json_line("{not-json}")).unwrap();
    assert_eq!(
        response.rejection.unwrap().code,
        ProtocolRejectionCode::MalformedRequest
    );
    assert_eq!(service.revision(), 0);
}

#[test]
fn unknown_fields_and_oversized_lines_are_malformed_without_mutation() {
    let mut service = service();
    let mut unknown = serde_json::from_str::<serde_json::Value>(SYNC_FIXTURE).unwrap();
    unknown["unexpected"] = serde_json::json!(true);
    let response: ProtocolResponseEnvelope =
        serde_json::from_str(&service.handle_json_line(&serde_json::to_string(&unknown).unwrap()))
            .unwrap();
    assert_eq!(
        response.rejection.unwrap().code,
        ProtocolRejectionCode::MalformedRequest
    );

    let oversized = "x".repeat(hollow_grove::gameplay::MAX_GAMEPLAY_MESSAGE_BYTES + 1);
    let response: ProtocolResponseEnvelope =
        serde_json::from_str(&service.handle_json_line(&oversized)).unwrap();
    assert_eq!(
        response.rejection.unwrap().code,
        ProtocolRejectionCode::MalformedRequest
    );
    assert_eq!(service.revision(), 0);
    assert!(service.application().events().is_empty());
}

#[test]
fn stdio_binary_serves_the_same_shared_fixture() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_hollow_grove_game_service"))
        .args(["--stdio", "--session", "session.fixture"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(SYNC_FIXTURE.as_bytes())
        .unwrap();
    child.stdin.take();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "{:?}", output.stderr);
    let response: ProtocolResponseEnvelope =
        serde_json::from_slice(output.stdout.strip_suffix(b"\n").unwrap()).unwrap();
    assert_eq!(response.status, ProtocolResponseStatus::Completed);
    assert_eq!(response.events[0].kind, GameplayEventKind::SnapshotLoaded);
}

#[test]
fn godot_client_is_transport_only_and_uses_the_shared_protocol_fields() {
    let source = include_str!("../hueman_godot/scripts/runtime_client.gd");
    for required in [
        "StreamPeerTCP",
        "protocol_version",
        "session_id",
        "request_id",
        "expected_revision",
        "intent",
        "response_received",
    ] {
        assert!(source.contains(required), "missing {required}");
    }
    for forbidden in [
        "BondAggregate",
        "RegionalSynthesisRuntime",
        "ConstitutionalRuntime",
        "RandomNumberGenerator",
        "screen_map_intent.json",
    ] {
        assert!(
            !source.contains(forbidden),
            "Godot client contains {forbidden}"
        );
    }
}
