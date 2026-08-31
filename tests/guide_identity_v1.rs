use std::process::Command;

use hollow_grove::composition::{ExternalRef, Position3};
use hollow_grove::guide_identity::{
    ActiveGuideContext, CENTRAL_JUNCTION_ORIGIN, DualGuideSession, DualSeerPresentation,
    GUIDE_IDENTITY_SCHEMA_VERSION, GUIDE_MODEL_PROMPT_LAW, GuideForm, GuideIdentity,
    GuideIdentityRecord, GuideMigrationError, GuidePresence, GuideSpeaker,
    GuideTransitionDirection, GuideTransitionError, GuideTransitionEvidence, JunctionGuidePresence,
    JunctionTransitionState, LegacyGuideIdentityRecord, PairedGuideTransition,
    PairedGuideTransitionOutcome, RAMPART_CANONICAL_NAME, StableGuideSessionId, StableTransitionId,
    WayBackGuideContext, WorldLayer, canonical_guide_identity_records, decode_dual_guide_session,
    encode_dual_guide_session, migrate_legacy_guide_identities, perform_paired_guide_transition,
    run_paired_guide_transition_cycle, stage_paired_guide_transition,
};
use hollow_grove::hollow_grove_contract::House;

fn reference(namespace: &str, key: &str) -> ExternalRef {
    ExternalRef::new(namespace, key).expect("test reference should be valid")
}

fn session_id() -> StableGuideSessionId {
    StableGuideSessionId::new("session.guide-continuity")
        .expect("test session identifier should be stable")
}

fn proposed_transition(
    session: &DualGuideSession,
    direction: GuideTransitionDirection,
    transition_id: &str,
) -> PairedGuideTransition {
    PairedGuideTransition::proposed(
        session,
        StableTransitionId::new(transition_id).expect("transition identifier should be stable"),
        direction,
        GuideTransitionEvidence {
            roi_continuity_ref: reference("witness", "roi-continuity"),
            roi_embodiment_ref: reference("witness", "roi-embodiment"),
            roselina_continuity_ref: reference("witness", "roselina-continuity"),
            roselina_embodiment_ref: reference("witness", "roselina-embodiment"),
            source_witness_ref: None,
        },
    )
    .expect("canonical transition proposal should be accepted")
}

#[test]
fn exactly_two_stable_guide_identities_exist() {
    assert_eq!(
        GuideIdentity::ALL,
        [GuideIdentity::Roi, GuideIdentity::Roselina]
    );
    assert_eq!(
        canonical_guide_identity_records(),
        [
            GuideIdentityRecord::canonical(GuideIdentity::Roi),
            GuideIdentityRecord::canonical(GuideIdentity::Roselina),
        ]
    );
    assert_eq!(GuideIdentity::Roi.stable_id(), "guide.roi");
    assert_eq!(GuideIdentity::Roselina.stable_id(), "guide.roselina");

    for interface_name in ["tui", "terminal", "ollama", "central_junction", "guide"] {
        assert!(
            serde_json::from_str::<GuideIdentity>(&format!("\"{interface_name}\"")).is_err(),
            "{interface_name} must not deserialize as a Guide identity"
        );
    }
    assert_eq!(RAMPART_CANONICAL_NAME, "The Rampart of Roi and Roselina");
}

#[test]
fn canonical_names_are_derived_from_identity_form_and_layer() {
    assert_eq!(
        GuidePresence::inside(GuideIdentity::Roi)
            .canonical_name()
            .unwrap(),
        "Roi"
    );
    assert_eq!(
        GuidePresence::outside(GuideIdentity::Roi)
            .canonical_name()
            .unwrap(),
        "Roï"
    );
    assert_eq!(
        GuidePresence::inside(GuideIdentity::Roselina)
            .canonical_name()
            .unwrap(),
        "Roselina"
    );
    assert_eq!(
        GuidePresence::outside(GuideIdentity::Roselina)
            .canonical_name()
            .unwrap(),
        "Rosaliña"
    );
}

#[test]
fn invalid_identity_form_layer_combinations_are_rejected() {
    assert!(
        GuidePresence::new(GuideIdentity::Roi, GuideForm::Seer, WorldLayer::HollowGrove,).is_err()
    );
    assert!(
        GuidePresence::new(
            GuideIdentity::Roselina,
            GuideForm::HollowGroveGuide,
            WorldLayer::OutsideHollowGrove,
        )
        .is_err()
    );
    assert!(
        GuidePresence::new(
            GuideIdentity::Roi,
            GuideForm::Seer,
            WorldLayer::CentralJunction,
        )
        .is_err()
    );
}

#[test]
fn central_junction_is_origin_and_not_a_house() {
    assert_eq!(CENTRAL_JUNCTION_ORIGIN, Position3::ORIGIN);
    assert!(serde_json::from_str::<House>("\"CentralJunction\"").is_err());

    let transitional = JunctionGuidePresence::new(
        GuideIdentity::Roi,
        GuideForm::Seer,
        reference("guide-transition", "entry-1"),
        JunctionTransitionState::AtOrigin,
    )
    .expect("an explicitly witnessed Central Junction state should be valid");
    assert_eq!(transitional.transitional_name().unwrap(), "Roï");
    assert!(
        JunctionGuidePresence::new(
            GuideIdentity::Roi,
            GuideForm::Seer,
            reference("guide-transition", "entry-1"),
            JunctionTransitionState::Completed,
        )
        .is_err()
    );
}

#[test]
fn central_junction_model_context_is_explicitly_transitional() {
    let session = DualGuideSession::outside(session_id());
    let transition = proposed_transition(
        &session,
        GuideTransitionDirection::EnterHollowGrove,
        "transition.context-1",
    );
    let context = ActiveGuideContext::during_transition(
        &session,
        &transition,
        JunctionTransitionState::AtOrigin,
    )
    .unwrap();
    assert_eq!(context.layer, WorldLayer::CentralJunction);
    assert_eq!(
        context.seam_transition,
        Some(JunctionTransitionState::AtOrigin)
    );
    assert_eq!(
        context.transition_ref.as_ref(),
        Some(&transition.external_ref())
    );
    assert_eq!(context.guides[0].display_name, "Roï");
    assert_eq!(context.guides[1].display_name, "Rosaliña");
    assert!(context.validate().is_ok());
}

#[test]
fn outside_and_inside_presentation_route_base_speakers_to_active_names() {
    let outside = DualGuideSession::outside(session_id())
        .active_context()
        .unwrap();
    assert_eq!(outside.display_name(GuideSpeaker::Roi).unwrap(), "Roï");
    assert_eq!(
        outside.display_name(GuideSpeaker::Roselina).unwrap(),
        "Rosaliña"
    );
    assert_eq!(
        outside.display_name(GuideSpeaker::Both).unwrap(),
        "ROÏ + ROSALIÑA"
    );

    let inside = DualGuideSession::inside(session_id())
        .active_context()
        .unwrap();
    assert_eq!(inside.display_name(GuideSpeaker::Roi).unwrap(), "Roi");
    assert_eq!(
        inside.display_name(GuideSpeaker::Roselina).unwrap(),
        "Roselina"
    );
    assert_eq!(
        inside.display_name(GuideSpeaker::Both).unwrap(),
        "ROI + ROSELINA"
    );
}

#[test]
fn entry_crossing_is_atomic_and_preserves_identity_and_session() {
    let mut session = DualGuideSession::outside(session_id());
    let original_session_id = session.session_id.clone();
    let roi_identity = session.roi_state.identity_ref.clone();
    let roselina_identity = session.roselina_state.identity_ref.clone();
    let transition = proposed_transition(
        &session,
        GuideTransitionDirection::EnterHollowGrove,
        "transition.entry-1",
    );

    let cycle = run_paired_guide_transition_cycle(&mut session, &transition).unwrap();
    assert_eq!(
        cycle.phase_trace,
        hollow_grove::constitutional::GrovePhase::ALL
    );
    assert!(cycle.gathering_valid);
    assert_eq!(
        cycle.way_back_before.current_layer,
        WorldLayer::OutsideHollowGrove
    );
    assert_eq!(cycle.way_back_after.current_layer, WorldLayer::HollowGrove);
    let witness = match cycle.festival {
        PairedGuideTransitionOutcome::Confirmed(witness) => witness,
        rejected => panic!("entry should confirm, got {rejected:?}"),
    };

    assert_eq!(session.session_id, original_session_id);
    assert_eq!(session.roi_state.identity_ref, roi_identity);
    assert_eq!(session.roselina_state.identity_ref, roselina_identity);
    assert_eq!(session.current_layer, WorldLayer::HollowGrove);
    assert_eq!(session.roi_state.canonical_name().unwrap(), "Roi");
    assert_eq!(session.roselina_state.canonical_name().unwrap(), "Roselina");
    assert_eq!(witness.roi_transition.from_name, "Roï");
    assert_eq!(witness.roi_transition.to_name, "Roi");
    assert_eq!(witness.roselina_transition.from_name, "Rosaliña");
    assert_eq!(witness.roselina_transition.to_name, "Roselina");
    assert_eq!(
        witness.junction_states,
        [
            JunctionTransitionState::Approaching,
            JunctionTransitionState::AtOrigin,
            JunctionTransitionState::FormChanging,
            JunctionTransitionState::Completed,
        ]
    );
}

#[test]
fn exit_crossing_is_atomic_and_preserves_identity_and_session() {
    let mut session = DualGuideSession::inside(session_id());
    let original = session.clone();
    let transition = proposed_transition(
        &session,
        GuideTransitionDirection::LeaveHollowGrove,
        "transition.exit-1",
    );

    let witness = match perform_paired_guide_transition(&mut session, &transition) {
        PairedGuideTransitionOutcome::Confirmed(witness) => witness,
        rejected => panic!("exit should confirm, got {rejected:?}"),
    };

    assert_eq!(session.session_id, original.session_id);
    assert_eq!(
        session.roi_state.identity_ref,
        original.roi_state.identity_ref
    );
    assert_eq!(
        session.roselina_state.identity_ref,
        original.roselina_state.identity_ref
    );
    assert_eq!(session.current_layer, WorldLayer::OutsideHollowGrove);
    assert_eq!(session.roi_state.canonical_name().unwrap(), "Roï");
    assert_eq!(session.roselina_state.canonical_name().unwrap(), "Rosaliña");
    assert_eq!(witness.roi_transition.from_name, "Roi");
    assert_eq!(witness.roi_transition.to_name, "Roï");
    assert_eq!(witness.roselina_transition.from_name, "Roselina");
    assert_eq!(witness.roselina_transition.to_name, "Rosaliña");
}

#[test]
fn rejected_or_partial_crossing_changes_neither_form() {
    let mut session = DualGuideSession::outside(session_id());
    let original = session.clone();
    let mut transition = proposed_transition(
        &session,
        GuideTransitionDirection::EnterHollowGrove,
        "transition.rejected-1",
    );
    transition.roselina_transition.embodiment_proof.confirmed = false;

    let rejection = match perform_paired_guide_transition(&mut session, &transition) {
        PairedGuideTransitionOutcome::Rejected(rejection) => rejection,
        confirmed => panic!("unproven crossing should reject, got {confirmed:?}"),
    };

    assert_eq!(session, original);
    assert!(
        rejection
            .reasons
            .contains(&GuideTransitionError::EmbodimentUnproven(
                GuideIdentity::Roselina
            ))
    );
}

#[test]
fn way_back_restores_a_staged_transition_and_rejection_clears_it() {
    let mut session = DualGuideSession::outside(session_id());
    let mut transition = proposed_transition(
        &session,
        GuideTransitionDirection::EnterHollowGrove,
        "transition.pending-1",
    );
    stage_paired_guide_transition(&mut session, &transition).unwrap();
    let pending_ref = transition.external_ref();
    assert_eq!(session.pending_transition_ref.as_ref(), Some(&pending_ref));

    let restored =
        decode_dual_guide_session(&encode_dual_guide_session(&session).unwrap()).unwrap();
    let way_back = WayBackGuideContext::restore(&restored).unwrap();
    assert_eq!(way_back.pending_transition.as_ref(), Some(&pending_ref));
    assert_eq!(way_back.roi_state.canonical_name().unwrap(), "Roï");
    assert_eq!(
        way_back.roselina_state.canonical_name().unwrap(),
        "Rosaliña"
    );

    transition.roi_transition.continuity_proof.confirmed = false;
    assert!(matches!(
        perform_paired_guide_transition(&mut session, &transition),
        PairedGuideTransitionOutcome::Rejected(_)
    ));
    assert!(session.pending_transition_ref.is_none());
    assert_eq!(session.roi_state.canonical_name().unwrap(), "Roï");
    assert_eq!(session.roselina_state.canonical_name().unwrap(), "Rosaliña");
}

#[test]
fn mismatched_pair_cannot_create_a_canonical_partial_crossing() {
    let mut session = DualGuideSession::outside(session_id());
    let original = session.clone();
    let mut transition = proposed_transition(
        &session,
        GuideTransitionDirection::EnterHollowGrove,
        "transition.mismatch-1",
    );
    transition.roselina_transition.identity_ref = GuideIdentity::Roi.external_ref();

    assert!(matches!(
        perform_paired_guide_transition(&mut session, &transition),
        PairedGuideTransitionOutcome::Rejected(_)
    ));
    assert_eq!(session, original);
}

#[test]
fn way_back_and_archive_restore_canonical_form_state_not_text() {
    let mut outside = DualGuideSession::outside(session_id());
    outside.focused_object_ref = Some(reference("file", "src/guide_identity.rs"));
    let encoded = encode_dual_guide_session(&outside).unwrap();
    let restored = decode_dual_guide_session(&encoded).unwrap();
    let way_back = WayBackGuideContext::restore(&restored).unwrap();

    assert_eq!(restored, outside);
    assert_eq!(way_back.session_ref, outside.external_ref());
    assert_eq!(way_back.current_layer, WorldLayer::OutsideHollowGrove);
    assert_eq!(way_back.roi_state.canonical_name().unwrap(), "Roï");
    assert_eq!(
        way_back.roselina_state.canonical_name().unwrap(),
        "Rosaliña"
    );

    let mut archive: serde_json::Value = serde_json::from_slice(&encoded).unwrap();
    archive["schema_version"] = serde_json::Value::String("unknown-guide-schema".into());
    assert!(decode_dual_guide_session(&serde_json::to_vec(&archive).unwrap()).is_err());
}

#[test]
fn migration_merges_alias_records_without_rewriting_history() {
    let records = vec![
        LegacyGuideIdentityRecord {
            legacy_identity_id: "legacy.roi.inside".into(),
            identity_hint: GuideIdentity::Roi,
            public_name: "Roi".into(),
            form: Some(GuideForm::HollowGroveGuide),
            layer: Some(WorldLayer::HollowGrove),
            historical_witness_text: Some("Roi entered the old scene.".into()),
        },
        LegacyGuideIdentityRecord {
            legacy_identity_id: "legacy.roi.outside".into(),
            identity_hint: GuideIdentity::Roi,
            public_name: "Roï".into(),
            form: Some(GuideForm::Seer),
            layer: Some(WorldLayer::OutsideHollowGrove),
            historical_witness_text: Some("Roï restored the old session.".into()),
        },
        LegacyGuideIdentityRecord {
            legacy_identity_id: "legacy.roselina.inside".into(),
            identity_hint: GuideIdentity::Roselina,
            public_name: "Roselina".into(),
            form: Some(GuideForm::HollowGroveGuide),
            layer: Some(WorldLayer::HollowGrove),
            historical_witness_text: Some("Roselina inspected the grove.".into()),
        },
        LegacyGuideIdentityRecord {
            legacy_identity_id: "legacy.rosalina.outside".into(),
            identity_hint: GuideIdentity::Roselina,
            public_name: "Rosalina".into(),
            form: Some(GuideForm::Seer),
            layer: Some(WorldLayer::OutsideHollowGrove),
            historical_witness_text: Some("Rosalina inspected the desktop.".into()),
        },
    ];

    let migrated =
        migrate_legacy_guide_identities(&records, WorldLayer::OutsideHollowGrove).unwrap();

    assert_eq!(migrated.schema_version, GUIDE_IDENTITY_SCHEMA_VERSION);
    assert_eq!(migrated.identity_records.len(), 2);
    assert_eq!(migrated.form_states.len(), 2);
    assert_eq!(migrated.metadata.len(), 2);
    assert!(
        migrated
            .metadata
            .iter()
            .flat_map(|metadata| &metadata.historical_witness_texts)
            .any(|text| text == "Roi entered the old scene.")
    );
    assert_eq!(
        migrated
            .form_states
            .iter()
            .map(|state| state.canonical_name().unwrap())
            .collect::<Vec<_>>(),
        ["Roï", "Rosaliña"]
    );
}

#[test]
fn migration_corrects_old_inside_accent_assumptions_without_destroying_audit_data() {
    let records = [
        LegacyGuideIdentityRecord {
            legacy_identity_id: "legacy.incorrect.roi".into(),
            identity_hint: GuideIdentity::Roi,
            public_name: "Roï".into(),
            form: Some(GuideForm::HollowGroveGuide),
            layer: Some(WorldLayer::HollowGrove),
            historical_witness_text: Some("Roï was once labeled inside.".into()),
        },
        LegacyGuideIdentityRecord {
            legacy_identity_id: "legacy.incorrect.roselina".into(),
            identity_hint: GuideIdentity::Roselina,
            public_name: "Rosaliña".into(),
            form: Some(GuideForm::HollowGroveGuide),
            layer: Some(WorldLayer::HollowGrove),
            historical_witness_text: Some("Rosaliña was once labeled inside.".into()),
        },
    ];

    let migrated = migrate_legacy_guide_identities(&records, WorldLayer::HollowGrove).unwrap();
    assert_eq!(
        migrated
            .form_states
            .iter()
            .map(|state| state.canonical_name().unwrap())
            .collect::<Vec<_>>(),
        ["Roi", "Roselina"]
    );
    assert!(
        migrated
            .metadata
            .iter()
            .all(|metadata| metadata.corrected_source_conflicts.len() == 1)
    );
    assert!(
        migrated
            .metadata
            .iter()
            .flat_map(|metadata| &metadata.historical_witness_texts)
            .any(|text| text == "Roï was once labeled inside.")
    );
}

#[test]
fn migration_rejects_unknown_people_and_ambiguous_junction_state() {
    let unknown = [LegacyGuideIdentityRecord {
        legacy_identity_id: "legacy.third-guide".into(),
        identity_hint: GuideIdentity::Roi,
        public_name: "The Guide".into(),
        form: None,
        layer: None,
        historical_witness_text: None,
    }];
    assert!(matches!(
        migrate_legacy_guide_identities(&unknown, WorldLayer::OutsideHollowGrove),
        Err(GuideMigrationError::UnknownLegacyName(name)) if name == "The Guide"
    ));
    let unrelated_same_name = [LegacyGuideIdentityRecord {
        legacy_identity_id: "person.unrelated.roi".into(),
        identity_hint: GuideIdentity::Roselina,
        public_name: "Roi".into(),
        form: None,
        layer: None,
        historical_witness_text: None,
    }];
    assert!(matches!(
        migrate_legacy_guide_identities(&unrelated_same_name, WorldLayer::OutsideHollowGrove),
        Err(GuideMigrationError::IdentityEvidenceMismatch { .. })
    ));

    let minimal = [
        LegacyGuideIdentityRecord {
            legacy_identity_id: "legacy.roi".into(),
            identity_hint: GuideIdentity::Roi,
            public_name: "Roi".into(),
            form: None,
            layer: None,
            historical_witness_text: None,
        },
        LegacyGuideIdentityRecord {
            legacy_identity_id: "legacy.roselina".into(),
            identity_hint: GuideIdentity::Roselina,
            public_name: "Roselina".into(),
            form: None,
            layer: None,
            historical_witness_text: None,
        },
    ];
    assert_eq!(
        migrate_legacy_guide_identities(&minimal, WorldLayer::CentralJunction),
        Err(GuideMigrationError::AmbiguousTransitionLayer)
    );
}

#[test]
fn model_context_gets_kernel_selected_names_and_exact_prompt_law() {
    let context = DualGuideSession::outside(session_id())
        .active_context()
        .unwrap();
    let value: serde_json::Value =
        serde_json::from_str(&context.model_context_json().unwrap()).unwrap();

    assert_eq!(value["layer"], "outside_hollow_grove");
    assert_eq!(value["guides"][0]["identity"], "roi");
    assert_eq!(value["guides"][0]["form"], "seer");
    assert_eq!(value["guides"][0]["display_name"], "Roï");
    assert_eq!(value["guides"][0]["jurisdiction"], "when");
    assert_eq!(value["guides"][1]["identity"], "roselina");
    assert_eq!(value["guides"][1]["display_name"], "Rosaliña");
    assert_eq!(value["guides"][1]["jurisdiction"], "where_what");

    assert!(GUIDE_MODEL_PROMPT_LAW.contains("Roi and Roï are the same being."));
    assert!(GUIDE_MODEL_PROMPT_LAW.contains("Roselina and Rosaliña are the same being."));
    assert!(GUIDE_MODEL_PROMPT_LAW.contains("Do not invent a generic third Guide."));
    assert!(
        GUIDE_MODEL_PROMPT_LAW
            .ends_with("Guide form is supplied and confirmed by the Rust kernel.")
    );
}

#[test]
fn malformed_active_context_cannot_override_kernel_names() {
    let mut context: ActiveGuideContext = DualGuideSession::outside(session_id())
        .active_context()
        .unwrap();
    context.guides[0].display_name = "Roi".into();
    assert!(context.validate().is_err());
}

#[test]
fn outside_tui_is_an_interface_for_the_two_state_derived_seer_names() {
    let rendered = DualSeerPresentation::outside(session_id())
        .render_tui()
        .unwrap();
    assert!(rendered.starts_with("THE SEERS\n"));
    assert!(rendered.contains("\nRoï\n"));
    assert!(rendered.contains("\nRosaliña\n"));
    assert!(rendered.contains("interface: TUI"));
    assert!(rendered.contains("authority: The Rampart of Roi and Roselina"));
    assert!(!rendered.contains("The Guide"));
}

#[test]
fn central_junction_transition_audit_confirms_the_round_trip() {
    let output = Command::new(env!("CARGO_BIN_EXE_guide_transition_audit"))
        .output()
        .expect("Guide transition audit should run");
    assert!(
        output.status.success(),
        "Guide transition audit failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("entry: Roï → Roi; Rosaliña → Roselina"));
    assert!(stdout.contains("exit: Roi → Roï; Roselina → Rosaliña"));
    assert!(stdout.contains("identity: guide.roi preserved"));
    assert!(stdout.contains("identity: guide.roselina preserved"));
    assert!(stdout.contains("result: confirmed"));
}
