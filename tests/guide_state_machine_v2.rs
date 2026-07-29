use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use hollow_grove::composition::ExternalRef;
use hollow_grove::guide_identity::{
    DualGuideSession, GUIDE_CONTEXT_SCHEMA_VERSION, GUIDE_IDENTITY_MIGRATION_SCHEMA_VERSION,
    GuideContextV1, GuideForm, GuideIdentity, GuideMigrationError, GuideTransitionDirection,
    GuideTransitionEvidence, JunctionTransitionState, LegacyGuideIdentityRecord,
    PairedGuideTransition, StableGuideSessionId, StableTransitionId, WorldLayer,
    decode_dual_guide_session, decode_guide_identity_migration, encode_dual_guide_session,
    guide_context_v1_json_schema, migrate_legacy_guide_identities,
};
use hollow_grove::guide_state_machine::{
    CanonicalGuideSession, DurableGuideTransitionCoordinator, DurableGuideTransitionOutcome,
    GuideStateMachineError, GuideTransitionEventKind, GuideTransitionJournalStatus,
    GuideTransitionJournalStore, InsideDualGuideSession, OutsideDualSeerSession,
    audit_guide_transition_store,
};

fn temp_dir(label: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should follow Unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("hollow-grove-{label}-{nonce}"))
}

fn reference(namespace: &str, key: &str) -> ExternalRef {
    ExternalRef::new(namespace, key).expect("fixture reference should be valid")
}

fn session_id() -> StableGuideSessionId {
    StableGuideSessionId::new("session.guide-state-machine-v2")
        .expect("fixture session ID should be stable")
}

fn transition(
    session: &DualGuideSession,
    id: &str,
    direction: GuideTransitionDirection,
) -> PairedGuideTransition {
    PairedGuideTransition::proposed(
        session,
        StableTransitionId::new(id).expect("fixture transition ID should be stable"),
        direction,
        GuideTransitionEvidence {
            roi_continuity_ref: reference("witness", "roi-continuity"),
            roi_embodiment_ref: reference("witness", "roi-embodiment"),
            roselina_continuity_ref: reference("witness", "roselina-continuity"),
            roselina_embodiment_ref: reference("witness", "roselina-embodiment"),
            source_witness_ref: Some(reference("witness", "source")),
        },
    )
    .expect("canonical transition should be proposed")
}

#[test]
fn typed_stable_states_reject_the_wrong_layer_and_direction() {
    assert!(OutsideDualSeerSession::new(DualGuideSession::inside(session_id())).is_err());
    assert!(InsideDualGuideSession::new(DualGuideSession::outside(session_id())).is_err());

    let outside = DualGuideSession::outside(session_id());
    let exit = transition(
        &DualGuideSession::inside(session_id()),
        "transition.wrong-direction",
        GuideTransitionDirection::LeaveHollowGrove,
    );
    assert!(matches!(
        OutsideDualSeerSession::new(outside)
            .unwrap()
            .begin_entry(exit),
        Err(GuideStateMachineError::WrongTransitionDirection { .. })
    ));
}

#[test]
fn junction_progression_is_ordered_and_exposes_versioned_context() {
    let session = DualGuideSession::outside(session_id());
    let entry = transition(
        &session,
        "transition.ordered-entry",
        GuideTransitionDirection::EnterHollowGrove,
    );
    let approaching = OutsideDualSeerSession::new(session)
        .unwrap()
        .begin_entry(entry)
        .unwrap();
    let context = approaching.guide_context_v1().unwrap();
    assert_eq!(context.schema_version, GUIDE_CONTEXT_SCHEMA_VERSION);
    assert_eq!(context.layer, WorldLayer::CentralJunction);
    assert_eq!(
        context.seam_transition,
        Some(JunctionTransitionState::Approaching)
    );
    assert_eq!(context.guides[0].display_name, "Roï");
    assert!(approaching.clone().form_changing().is_err());

    let changing = approaching.at_origin().unwrap().form_changing().unwrap();
    assert_eq!(
        changing.guide_context_v1().unwrap().seam_transition,
        Some(JunctionTransitionState::FormChanging)
    );
}

#[test]
fn versioned_context_schema_and_inside_outside_fixtures_are_kernel_generated() {
    let checked_in_schema: serde_json::Value =
        serde_json::from_str(include_str!("../schemas/guide-context-v1.schema.json")).unwrap();
    assert_eq!(guide_context_v1_json_schema(), checked_in_schema);

    let outside_fixture: GuideContextV1 =
        serde_json::from_str(include_str!("fixtures/guide-context-v1-outside.json")).unwrap();
    let inside_fixture: GuideContextV1 =
        serde_json::from_str(include_str!("fixtures/guide-context-v1-inside.json")).unwrap();
    let fixture_id = StableGuideSessionId::new("session.contract-fixture").unwrap();
    assert_eq!(
        outside_fixture,
        DualGuideSession::outside(fixture_id.clone())
            .guide_context_v1()
            .unwrap()
    );
    assert_eq!(
        inside_fixture,
        DualGuideSession::inside(fixture_id)
            .guide_context_v1()
            .unwrap()
    );
    outside_fixture.validate().unwrap();
    inside_fixture.validate().unwrap();
}

#[test]
fn durable_round_trip_survives_restart_and_preserves_session_and_object() {
    let root = temp_dir("guide-round-trip");
    let coordinator =
        DurableGuideTransitionCoordinator::new(GuideTransitionJournalStore::new(&root));
    let mut raw = DualGuideSession::outside(session_id());
    raw.focused_object_ref = Some(reference("object", "focused-work"));
    let stable_session = raw.session_id.clone();
    let roi_identity = raw.roi_state.identity_ref.clone();
    let roselina_identity = raw.roselina_state.identity_ref.clone();

    let entry = transition(
        &raw,
        "transition.durable-entry",
        GuideTransitionDirection::EnterHollowGrove,
    );
    let entry_outcome = coordinator
        .execute_entry(
            OutsideDualSeerSession::new(raw)
                .unwrap()
                .begin_entry(entry)
                .unwrap(),
        )
        .unwrap();
    let inside = match entry_outcome {
        DurableGuideTransitionOutcome::Confirmed {
            session: CanonicalGuideSession::Inside(session),
            events,
            replayed,
            recovered,
            ..
        } => {
            assert!(!replayed);
            assert!(!recovered);
            assert_eq!(
                events.iter().map(|event| event.kind).collect::<Vec<_>>(),
                vec![
                    GuideTransitionEventKind::Proposed,
                    GuideTransitionEventKind::JournalPrepared,
                    GuideTransitionEventKind::Approaching,
                    GuideTransitionEventKind::AtOrigin,
                    GuideTransitionEventKind::FormChanging,
                    GuideTransitionEventKind::Confirmed,
                ]
            );
            assert!(
                events
                    .last()
                    .unwrap()
                    .render_tui()
                    .unwrap()
                    .contains("active: Roi + Roselina")
            );
            session
        }
        other => panic!("entry should confirm inside, got {other:?}"),
    };

    let archive = encode_dual_guide_session(inside.as_session()).unwrap();
    let restarted = decode_dual_guide_session(&archive).unwrap();
    let inside = InsideDualGuideSession::new(restarted).unwrap();
    let exit = transition(
        inside.as_session(),
        "transition.durable-exit",
        GuideTransitionDirection::LeaveHollowGrove,
    );
    let outside = match coordinator
        .execute_exit(inside.begin_exit(exit).unwrap())
        .unwrap()
    {
        DurableGuideTransitionOutcome::Confirmed {
            session: CanonicalGuideSession::Outside(session),
            ..
        } => session,
        other => panic!("exit should confirm outside, got {other:?}"),
    };

    assert_eq!(outside.as_session().session_id, stable_session);
    assert_eq!(outside.as_session().roi_state.identity_ref, roi_identity);
    assert_eq!(
        outside.as_session().roselina_state.identity_ref,
        roselina_identity
    );
    assert_eq!(
        outside.as_session().focused_object_ref,
        Some(reference("object", "focused-work"))
    );
    assert_eq!(
        outside.as_session().roi_state.canonical_name().unwrap(),
        "Roï"
    );
    assert_eq!(
        outside
            .as_session()
            .roselina_state
            .canonical_name()
            .unwrap(),
        "Rosaliña"
    );
    assert!(audit_guide_transition_store(&root).is_clean());
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn prepared_journal_recovers_once_after_a_simulated_crash() {
    let root = temp_dir("guide-recovery");
    let store = GuideTransitionJournalStore::new(&root);
    let coordinator = DurableGuideTransitionCoordinator::new(store.clone());
    let raw = DualGuideSession::outside(session_id());
    let entry = transition(
        &raw,
        "transition.crash-recovery",
        GuideTransitionDirection::EnterHollowGrove,
    );
    let id = entry.transition_id.clone();
    let pending = OutsideDualSeerSession::new(raw)
        .unwrap()
        .begin_entry(entry)
        .unwrap();
    coordinator.prepare_entry(&pending).unwrap();

    let prepared = store.load(&id).unwrap().unwrap();
    assert_eq!(prepared.status, GuideTransitionJournalStatus::Prepared);
    let before = audit_guide_transition_store(&root);
    assert!(!before.is_clean());
    assert_eq!(before.prepared_count, 1);

    drop(coordinator);
    let restarted = DurableGuideTransitionCoordinator::new(store.clone());
    match restarted.recover(&id).unwrap() {
        DurableGuideTransitionOutcome::Confirmed {
            session: CanonicalGuideSession::Inside(session),
            recovered,
            replayed,
            events,
            ..
        } => {
            assert!(recovered);
            assert!(!replayed);
            assert_eq!(session.as_session().current_layer, WorldLayer::HollowGrove);
            assert!(
                events
                    .iter()
                    .any(|event| event.kind == GuideTransitionEventKind::Recovered)
            );
        }
        other => panic!("recovery should confirm once, got {other:?}"),
    }
    assert!(audit_guide_transition_store(&root).is_clean());
    assert_eq!(
        store.load(&id).unwrap().unwrap().status,
        GuideTransitionJournalStatus::Confirmed
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn repeated_execution_replays_one_witness_and_conflicting_reuse_is_rejected() {
    let root = temp_dir("guide-idempotency");
    let coordinator =
        DurableGuideTransitionCoordinator::new(GuideTransitionJournalStore::new(&root));
    let raw = DualGuideSession::outside(session_id());
    let entry = transition(
        &raw,
        "transition.idempotent-entry",
        GuideTransitionDirection::EnterHollowGrove,
    );
    let first_pending = OutsideDualSeerSession::new(raw.clone())
        .unwrap()
        .begin_entry(entry.clone())
        .unwrap();
    coordinator.execute_entry(first_pending).unwrap();

    let replay = coordinator
        .execute_entry(
            OutsideDualSeerSession::new(raw)
                .unwrap()
                .begin_entry(entry)
                .unwrap(),
        )
        .unwrap();
    match replay {
        DurableGuideTransitionOutcome::Confirmed {
            replayed,
            recovered,
            events,
            ..
        } => {
            assert!(replayed);
            assert!(!recovered);
            assert_eq!(
                events
                    .iter()
                    .filter(|event| event.kind == GuideTransitionEventKind::Confirmed)
                    .count(),
                1
            );
            assert_eq!(
                events
                    .iter()
                    .filter(|event| event.kind == GuideTransitionEventKind::Replayed)
                    .count(),
                1
            );
        }
        other => panic!("replay should return the original confirmation, got {other:?}"),
    }

    let different_session = DualGuideSession::outside(
        StableGuideSessionId::new("session.guide-state-machine-conflict").unwrap(),
    );
    let conflict = transition(
        &different_session,
        "transition.idempotent-entry",
        GuideTransitionDirection::EnterHollowGrove,
    );
    let error = coordinator
        .execute_entry(
            OutsideDualSeerSession::new(different_session)
                .unwrap()
                .begin_entry(conflict)
                .unwrap(),
        )
        .unwrap_err();
    assert!(matches!(
        error,
        GuideStateMachineError::IdempotencyConflict(_)
    ));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn durable_festival_rejection_rolls_back_both_guides_and_becomes_idempotent() {
    let root = temp_dir("guide-rejection");
    let store = GuideTransitionJournalStore::new(&root);
    let coordinator = DurableGuideTransitionCoordinator::new(store.clone());
    let raw = DualGuideSession::outside(session_id());
    let entry = transition(
        &raw,
        "transition.rejected-entry",
        GuideTransitionDirection::EnterHollowGrove,
    );
    let id = entry.transition_id.clone();
    let pending = OutsideDualSeerSession::new(raw.clone())
        .unwrap()
        .begin_entry(entry.clone())
        .unwrap();
    let outcome = coordinator
        .reject_entry(
            pending,
            hollow_grove::guide_identity::GuideTransitionError::ConstitutionalRejection(
                "Gathering did not establish a valid destination embodiment.".into(),
            ),
        )
        .unwrap();
    match outcome {
        DurableGuideTransitionOutcome::Rejected {
            session: CanonicalGuideSession::Outside(session),
            replayed,
            ..
        } => {
            assert!(!replayed);
            assert_eq!(session.as_session(), &raw);
            assert_eq!(
                session.as_session().roi_state.canonical_name().unwrap(),
                "Roï"
            );
            assert_eq!(
                session
                    .as_session()
                    .roselina_state
                    .canonical_name()
                    .unwrap(),
                "Rosaliña"
            );
        }
        other => panic!("Festival should reject without changing either Guide, got {other:?}"),
    }
    assert_eq!(
        store.load(&id).unwrap().unwrap().status,
        GuideTransitionJournalStatus::Rejected
    );

    let replay = coordinator
        .execute_entry(
            OutsideDualSeerSession::new(raw)
                .unwrap()
                .begin_entry(entry)
                .unwrap(),
        )
        .unwrap();
    assert!(matches!(
        replay,
        DurableGuideTransitionOutcome::Rejected { replayed: true, .. }
    ));
    assert!(audit_guide_transition_store(&root).is_clean());
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn audit_reports_corruption_without_accepting_it_as_canonical() {
    let root = temp_dir("guide-audit-corrupt");
    fs::create_dir_all(&root).unwrap();
    fs::write(root.join("broken.json"), b"{not-json").unwrap();
    let report = audit_guide_transition_store(&root);
    assert!(!report.is_clean());
    assert_eq!(report.journal_count, 1);
    assert!(
        report
            .findings
            .iter()
            .any(|finding| finding.code == "journal_unreadable")
    );
    assert!(report.render_tui().starts_with("GUIDE STATE AUDIT — FAIL"));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn audit_detects_duplicate_transition_ids_and_tampered_witness_names() {
    let root = temp_dir("guide-audit-tamper");
    let store = GuideTransitionJournalStore::new(&root);
    let coordinator = DurableGuideTransitionCoordinator::new(store.clone());
    let raw = DualGuideSession::outside(session_id());
    let entry = transition(
        &raw,
        "transition.audit-tamper",
        GuideTransitionDirection::EnterHollowGrove,
    );
    let id = entry.transition_id.clone();
    coordinator
        .execute_entry(
            OutsideDualSeerSession::new(raw)
                .unwrap()
                .begin_entry(entry)
                .unwrap(),
        )
        .unwrap();

    let canonical_path = store.journal_path(&id);
    let mut value: serde_json::Value =
        serde_json::from_slice(&fs::read(&canonical_path).unwrap()).unwrap();
    value["witness"]["roi_transition"]["to_name"] = "Roï".into();
    fs::write(
        root.join("duplicate-transition.json"),
        serde_json::to_vec_pretty(&value).unwrap(),
    )
    .unwrap();

    let report = audit_guide_transition_store(&root);
    assert!(!report.is_clean());
    assert!(
        report
            .findings
            .iter()
            .any(|finding| finding.code == "duplicate_transition_id")
    );
    assert!(
        report
            .findings
            .iter()
            .any(|finding| finding.code == "constitutional_violation")
    );
    fs::remove_dir_all(root).unwrap();
}

fn legacy_records() -> Vec<LegacyGuideIdentityRecord> {
    vec![
        LegacyGuideIdentityRecord {
            legacy_identity_id: "legacy.roi".into(),
            identity_hint: GuideIdentity::Roi,
            public_name: "Roï".into(),
            form: Some(GuideForm::Seer),
            layer: Some(WorldLayer::OutsideHollowGrove),
            historical_witness_text: Some("Roï restored the session.".into()),
        },
        LegacyGuideIdentityRecord {
            legacy_identity_id: "legacy.roselina".into(),
            identity_hint: GuideIdentity::Roselina,
            public_name: "Rosaliña".into(),
            form: Some(GuideForm::Seer),
            layer: Some(WorldLayer::OutsideHollowGrove),
            historical_witness_text: Some("Rosaliña inspected the workspace.".into()),
        },
    ]
}

#[test]
fn migration_v2_is_idempotent_and_reads_v1_artifacts_without_losing_history() {
    let first =
        migrate_legacy_guide_identities(&legacy_records(), WorldLayer::OutsideHollowGrove).unwrap();
    let second =
        migrate_legacy_guide_identities(&legacy_records(), WorldLayer::OutsideHollowGrove).unwrap();
    assert_eq!(first, second);
    first.validate().unwrap();
    assert_eq!(
        first.migration_schema_version,
        GUIDE_IDENTITY_MIGRATION_SCHEMA_VERSION
    );

    let mut old_value = serde_json::to_value(&first).unwrap();
    old_value
        .as_object_mut()
        .unwrap()
        .remove("migration_schema_version");
    old_value
        .as_object_mut()
        .unwrap()
        .remove("applied_migrations");
    let upgraded =
        decode_guide_identity_migration(&serde_json::to_vec(&old_value).unwrap()).unwrap();
    assert_eq!(upgraded.identity_records, first.identity_records);
    assert_eq!(upgraded.metadata, first.metadata);
    assert_eq!(
        upgraded.applied_migrations,
        vec![GUIDE_IDENTITY_MIGRATION_SCHEMA_VERSION]
    );

    let mut future = serde_json::to_value(&first).unwrap();
    future["migration_schema_version"] = "hollow-grove-guide-identity-migration-v99".into();
    assert!(matches!(
        decode_guide_identity_migration(&serde_json::to_vec(&future).unwrap()),
        Err(GuideMigrationError::UnsupportedMigrationSchema { .. })
    ));
}
