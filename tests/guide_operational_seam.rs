use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use hollow_grove::composition::ExternalRef;
use hollow_grove::constitutional::{RuleSetId, V2_RULE_SET};
use hollow_grove::gameplay::{
    GameProtocolService, GameplayEventKind, GameplayIntent, ProtocolRejectionCode,
    ProtocolRequestEnvelope, ProtocolResponseStatus, guide_transition_journal_root,
};
use hollow_grove::guide_identity::{
    DualGuideSession, GuideIdentity, GuideTransitionDirection, GuideTransitionEvidence,
    PairedGuideTransition, StableGuideSessionId, StableTransitionId, WorldLayer,
};
use hollow_grove::guide_state_machine::{
    DurableGuideTransitionCoordinator, GuideTransitionEventKind, GuideTransitionJournalStore,
    OutsideDualSeerSession,
};

const SESSION_ID: &str = "session.operational-seam";

fn rules() -> RuleSetId {
    RuleSetId::new(V2_RULE_SET).expect("V2 rules are canonical")
}

fn temp_dir(label: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should follow Unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "hollow-grove-operational-seam-{label}-{}-{nonce}",
        std::process::id()
    ))
}

fn live_service(root: &Path) -> GameProtocolService {
    GameProtocolService::new_with_roots(
        SESSION_ID,
        rules(),
        root,
        Path::new(env!("CARGO_MANIFEST_DIR")),
    )
    .expect("live service should initialize")
}

fn request(
    request_id: &str,
    expected_revision: u64,
    intent: GameplayIntent,
) -> ProtocolRequestEnvelope {
    ProtocolRequestEnvelope {
        protocol_version: 1,
        session_id: SESSION_ID.into(),
        request_id: request_id.into(),
        expected_revision,
        intent,
    }
}

fn active_names(response: &hollow_grove::gameplay::ProtocolResponseEnvelope) -> Vec<&str> {
    response
        .view
        .as_ref()
        .expect("Guide responses include a view")
        .guide_context
        .guides
        .iter()
        .map(|guide| guide.display_name.as_str())
        .collect()
}

#[test]
fn live_entry_and_exit_share_one_revision_and_preserve_two_identities() {
    let root = temp_dir("entry-exit");
    let mut service = live_service(&root);

    let sync = service.handle(request(
        "request.sync-outside",
        0,
        GameplayIntent::SyncIntent,
    ));
    assert_eq!(sync.status, ProtocolResponseStatus::Completed);
    assert_eq!(sync.revision, 0);
    assert_eq!(
        sync.view.as_ref().unwrap().guide_context.layer,
        WorldLayer::OutsideHollowGrove
    );
    assert_eq!(active_names(&sync), ["Roï", "Rosaliña"]);

    let blocked = service.handle(request(
        "request.outside-establish",
        0,
        GameplayIntent::EstablishHuemanIntent {
            continuity_id: "being-continuity.hueman".into(),
            participant_id: "participant.hueman".into(),
            institutional_being_id: "being.hueman".into(),
        },
    ));
    assert_eq!(blocked.status, ProtocolResponseStatus::Rejected);
    assert_eq!(
        blocked.rejection.unwrap().code,
        ProtocolRejectionCode::CapabilityUnavailable
    );
    assert_eq!(service.revision(), 0);

    let entry_request = request("request.enter", 0, GameplayIntent::EnterHollowGroveIntent);
    let entered = service.handle(entry_request.clone());
    assert_eq!(entered.status, ProtocolResponseStatus::Completed);
    assert_eq!(entered.revision, 1);
    assert_eq!(active_names(&entered), ["Roi", "Roselina"]);
    assert_eq!(
        entered
            .view
            .as_ref()
            .unwrap()
            .guide_transition_events
            .last()
            .unwrap()
            .kind,
        GuideTransitionEventKind::Confirmed
    );
    assert!(
        entered
            .view
            .as_ref()
            .unwrap()
            .guide_transition_events
            .last()
            .unwrap()
            .render_tui()
            .unwrap()
            .contains("active: Roi + Roselina")
    );
    assert_eq!(
        service
            .guide_session()
            .guide_context_v1()
            .unwrap()
            .guides
            .iter()
            .map(|guide| guide.identity)
            .collect::<Vec<_>>(),
        [GuideIdentity::Roi, GuideIdentity::Roselina]
    );

    let stale = service.handle(request(
        "request.stale-client",
        0,
        GameplayIntent::EstablishHuemanIntent {
            continuity_id: "being-continuity.hueman".into(),
            participant_id: "participant.hueman".into(),
            institutional_being_id: "being.hueman".into(),
        },
    ));
    assert_eq!(stale.status, ProtocolResponseStatus::Rejected);
    assert_eq!(
        stale.rejection.unwrap().code,
        ProtocolRejectionCode::StaleRevision
    );

    let established = service.handle(request(
        "request.establish",
        1,
        GameplayIntent::EstablishHuemanIntent {
            continuity_id: "being-continuity.hueman".into(),
            participant_id: "participant.hueman".into(),
            institutional_being_id: "being.hueman".into(),
        },
    ));
    assert_eq!(established.status, ProtocolResponseStatus::Completed);
    assert_eq!(established.revision, 2);
    assert_eq!(established.events[0].revision, 1);
    assert_eq!(
        established.events[0].kind,
        GameplayEventKind::IdentityEstablished
    );
    assert_eq!(established.view.unwrap().revision, 2);

    let exited = service.handle(request(
        "request.leave",
        2,
        GameplayIntent::LeaveHollowGroveIntent,
    ));
    assert_eq!(exited.status, ProtocolResponseStatus::Completed);
    assert_eq!(exited.revision, 3);
    assert_eq!(active_names(&exited), ["Roï", "Rosaliña"]);

    assert_eq!(service.handle(entry_request), entered);

    drop(service);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn startup_recovers_layer_state_and_excludes_a_second_service_process() {
    let root = temp_dir("restart-lock");
    let mut first = live_service(&root);
    let entered = first.handle(request(
        "request.restart-entry",
        0,
        GameplayIntent::EnterHollowGroveIntent,
    ));
    assert_eq!(entered.status, ProtocolResponseStatus::Completed);

    let competing = GameProtocolService::new_with_roots(
        SESSION_ID,
        rules(),
        &root,
        Path::new(env!("CARGO_MANIFEST_DIR")),
    )
    .unwrap_err();
    assert!(competing.to_string().contains("active gameplay service"));

    drop(first);
    let mut restarted = live_service(&root);
    let restored = restarted.handle(request(
        "request.restart-sync",
        0,
        GameplayIntent::SyncIntent,
    ));
    assert_eq!(restored.revision, 1);
    assert_eq!(active_names(&restored), ["Roi", "Roselina"]);

    let exited = restarted.handle(request(
        "request.restart-exit",
        1,
        GameplayIntent::LeaveHollowGroveIntent,
    ));
    assert_eq!(exited.status, ProtocolResponseStatus::Completed);
    drop(restarted);

    let mut returned = live_service(&root);
    let outside = returned.handle(request(
        "request.returned-sync",
        0,
        GameplayIntent::SyncIntent,
    ));
    assert_eq!(outside.revision, 2);
    assert_eq!(active_names(&outside), ["Roï", "Rosaliña"]);

    drop(returned);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn a_prepared_crossing_is_completed_before_the_service_accepts_requests() {
    let root = temp_dir("prepared-recovery");
    let session_id = StableGuideSessionId::new(SESSION_ID).unwrap();
    let outside_session = DualGuideSession::outside(session_id);
    let transition = PairedGuideTransition::proposed(
        &outside_session,
        StableTransitionId::new("transition.request.crash-entry").unwrap(),
        GuideTransitionDirection::EnterHollowGrove,
        GuideTransitionEvidence {
            roi_continuity_ref: ExternalRef::new("witness", "roi-continuity").unwrap(),
            roi_embodiment_ref: ExternalRef::new("witness", "roi-embodiment").unwrap(),
            roselina_continuity_ref: ExternalRef::new("witness", "roselina-continuity").unwrap(),
            roselina_embodiment_ref: ExternalRef::new("witness", "roselina-embodiment").unwrap(),
            source_witness_ref: Some(ExternalRef::new("witness", "source").unwrap()),
        },
    )
    .unwrap();
    let pending = OutsideDualSeerSession::new(outside_session)
        .unwrap()
        .begin_entry(transition)
        .unwrap();
    let coordinator = DurableGuideTransitionCoordinator::new(GuideTransitionJournalStore::new(
        guide_transition_journal_root(&root, SESSION_ID),
    ));
    coordinator.prepare_entry(&pending).unwrap();
    drop(coordinator);

    let mut recovered = live_service(&root);
    assert_eq!(
        recovered.guide_session().current_layer,
        WorldLayer::HollowGrove
    );
    assert_eq!(recovered.revision(), 1);
    let sync = recovered.handle(request(
        "request.recovery-sync",
        0,
        GameplayIntent::SyncIntent,
    ));
    let transition_events = &sync.view.as_ref().unwrap().guide_transition_events;
    assert!(
        transition_events
            .iter()
            .any(|event| event.kind == GuideTransitionEventKind::Recovered)
    );
    assert_eq!(
        transition_events.last().unwrap().kind,
        GuideTransitionEventKind::Confirmed
    );
    assert_eq!(active_names(&sync), ["Roi", "Roselina"]);

    drop(recovered);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn save_and_load_restore_gameplay_and_guide_form_as_one_archive() {
    let root = temp_dir("save-load");
    let mut service = GameProtocolService::new_with_save_root(SESSION_ID, rules(), &root).unwrap();
    let left = service.handle(request(
        "request.save-exit",
        0,
        GameplayIntent::LeaveHollowGroveIntent,
    ));
    assert_eq!(left.status, ProtocolResponseStatus::Completed);
    assert_eq!(active_names(&left), ["Roï", "Rosaliña"]);
    let saved = service.handle(request(
        "request.save-outside",
        1,
        GameplayIntent::SaveIntent {
            slot_id: "slot-a".into(),
        },
    ));
    assert_eq!(saved.status, ProtocolResponseStatus::Completed);

    let entered = service.handle(request(
        "request.save-reenter",
        1,
        GameplayIntent::EnterHollowGroveIntent,
    ));
    assert_eq!(entered.revision, 2);
    assert_eq!(active_names(&entered), ["Roi", "Roselina"]);

    let loaded = service.handle(request(
        "request.load-outside",
        2,
        GameplayIntent::LoadIntent {
            slot_id: "slot-a".into(),
        },
    ));
    assert_eq!(loaded.status, ProtocolResponseStatus::Completed);
    assert_eq!(loaded.revision, 1);
    assert_eq!(active_names(&loaded), ["Roï", "Rosaliña"]);
    assert_eq!(
        loaded
            .view
            .as_ref()
            .unwrap()
            .guide_context
            .guides
            .iter()
            .map(|guide| guide.identity)
            .collect::<Vec<_>>(),
        [GuideIdentity::Roi, GuideIdentity::Roselina]
    );

    drop(service);
    let mut restarted = live_service(&root);
    let restored = restarted.handle(request(
        "request.loaded-restart-sync",
        0,
        GameplayIntent::SyncIntent,
    ));
    assert_eq!(restored.revision, 1);
    assert_eq!(active_names(&restored), ["Roï", "Rosaliña"]);
    assert!(
        restored
            .view
            .as_ref()
            .unwrap()
            .guide_transition_events
            .is_empty(),
        "a transition after the loaded save must not replay on restart"
    );
    drop(restarted);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn godot_enters_from_the_kernel_layer_and_only_renders_supplied_guide_names() {
    let source = include_str!("../hueman_godot/scripts/retro_overworld.gd");
    assert!(source.contains("\"EnterHollowGroveIntent\""));
    assert!(
        source.contains("if str(guide_context.get(\"layer\", \"\")) == \"outside_hollow_grove\":")
    );
    assert!(source.contains("_send_enter_hollow_grove()"));
    assert!(source.contains("guide_context = _dict_or_empty(view.get(\"guide_context\"))"));
    assert!(source.contains("guide_transition_events = view.get(\"guide_transition_events\", [])"));
    assert!(!source.contains("\"display_name\": \"Roi\""));
    assert!(!source.contains("\"display_name\": \"Roï\""));
}
