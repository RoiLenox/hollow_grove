use hollow_grove::constitutional::{CausalPosition, ParticipantId, RuleSetId};
use hollow_grove::gameplay::{
    BeingContinuityId, GameApplicationService, GameProtocolService, GameplayCommand,
    GameplayEventId, GameplayEventMetadata, GameplayIntent, HOLLOW_GROVE_GAMEPLAY_PROTOCOL_VERSION,
    ProtocolRequestEnvelope, ProtocolResponseStatus, SympioteAdaptation, SympioteBoardwalkCamp,
    SympioteFoundationAction as A, SympioteFoundationPhase as P, SympiotePracticeDirection,
};
use hollow_grove::institution::InstitutionalBeingId;
use hollow_grove::kernel_invariants::audit_sympiote_foundation;
use hollow_grove::world::session::WorldSession;
use hollow_grove::world::sympiote::{
    SympiotePhase, decode_sympiote_history, encode_sympiote_history,
};
use hollow_grove::world::sympiote_foundation::SympioteFoundationState;

fn rules() -> RuleSetId {
    RuleSetId::new("rule-set.sympiote-foundation.tests.v1").unwrap()
}

#[test]
fn long_assessment_history_has_bounded_read_only_projection_and_valid_handoffs() {
    let mut service = application();
    for _ in 0..80 {
        step(&mut service, A::RequestAssessment);
        step(&mut service, A::ConsentToFoundation);
    }
    let before = service.encode_archive().unwrap();
    let view = hollow_grove::gameplay::SympioteFoundationView::from_runtime(service.runtime());
    assert_eq!(view.clinical_receipt_count, 80);
    assert_eq!(view.clinical_receipts.len(), 32);
    assert!(serde_json::to_vec(&view).unwrap().len() < 32 * 1024);
    let inspection =
        hollow_grove::gameplay::LoreInspection::from_runtime(Some(service.runtime())).unwrap();
    assert_eq!(inspection.schema_version, 2);
    assert_eq!(
        inspection
            .sympiote_foundation_contract
            .declared_handoffs
            .len(),
        3
    );
    assert!(
        !inspection
            .sympiote_foundation
            .as_ref()
            .unwrap()
            .form_established
    );
    assert_eq!(service.encode_archive().unwrap(), before);
    let game_view = hollow_grove::gameplay::GameView::from_runtime(service.runtime(), Vec::new());
    let mut old_view = serde_json::to_value(game_view).unwrap();
    old_view
        .as_object_mut()
        .unwrap()
        .remove("sympiote_foundation");
    let old_view: hollow_grove::gameplay::GameView = serde_json::from_value(old_view).unwrap();
    assert!(!old_view.sympiote_foundation.form_established);
    assert!(old_view.sympiote_foundation.profile.is_none());
}
fn live_world() -> WorldSession {
    WorldSession::load_or_canonical_at(std::path::Path::new(env!("CARGO_MANIFEST_DIR"))).unwrap()
}

fn application() -> GameApplicationService {
    let mut service = GameApplicationService::with_world_session(rules(), live_world());
    service
        .execute(
            GameplayEventMetadata {
                id: GameplayEventId::new("event.sympiote.identity").unwrap(),
                causal_position: CausalPosition::new(1),
            },
            GameplayCommand::EstablishHuemanIdentity {
                continuity: BeingContinuityId::new("continuity.sympiote.patient").unwrap(),
                participant: ParticipantId::new("participant.sympiote.patient").unwrap(),
                institutional: InstitutionalBeingId::new("being.sympiote.patient").unwrap(),
            },
        )
        .unwrap();
    service
}
fn step(service: &mut GameApplicationService, action: A) {
    let at = service.revision().get() + 1;
    service
        .execute(
            GameplayEventMetadata {
                id: GameplayEventId::new(format!("event.sympiote.{at}")).unwrap(),
                causal_position: CausalPosition::new(at),
            },
            GameplayCommand::AdvanceSympioteFoundation { action },
        )
        .unwrap();
}
fn establish(service: &mut GameApplicationService) {
    for action in [
        A::RequestAssessment,
        A::ConsentToFoundation,
        A::AskClinicianToEstablish,
    ] {
        step(service, action);
    }
}
fn discharge(service: &mut GameApplicationService) {
    establish(service);
    step(service, A::Rest);
    step(service, A::AskClinicianToDischarge);
}

#[test]
fn clinical_form_requires_identity_consent_authority_and_recovery() {
    let mut empty = GameApplicationService::new(rules());
    assert!(
        empty
            .execute(
                GameplayEventMetadata {
                    id: GameplayEventId::new("event.sympiote.no-identity").unwrap(),
                    causal_position: CausalPosition::new(1)
                },
                GameplayCommand::AdvanceSympioteFoundation {
                    action: A::RequestAssessment
                }
            )
            .is_err()
    );
    assert_eq!(empty.revision().get(), 0);
    let mut service = application();
    step(&mut service, A::RequestAssessment);
    let before = service.encode_archive().unwrap();
    assert!(
        service
            .execute(
                GameplayEventMetadata {
                    id: GameplayEventId::new("event.sympiote.skip-consent").unwrap(),
                    causal_position: CausalPosition::new(3)
                },
                GameplayCommand::AdvanceSympioteFoundation {
                    action: A::AskClinicianToEstablish
                }
            )
            .is_err()
    );
    assert_eq!(before, service.encode_archive().unwrap());
    step(&mut service, A::ConsentToFoundation);
    step(&mut service, A::AskClinicianToEstablish);
    let state = service.runtime().sympiote_foundation();
    assert_eq!(state.phase(), P::Recovering);
    assert!(state.form_established());
    assert!(state.needs_rest());
    assert_eq!(
        state.graft().unwrap().phase,
        SympiotePhase::ReciprocallyIntegrated
    );
    assert_eq!(state.graft().unwrap().events.len(), 7);
    assert!(
        !state
            .available_actions()
            .contains(&A::AskClinicianToDischarge)
    );
    assert_eq!(
        state.profile().unwrap().subject_id,
        "being.sympiote.patient"
    );
    assert!(
        state
            .clinical_receipts()
            .iter()
            .all(|receipt| receipt.office == "office.glaushouse.prima-donna"
                && receipt.authority_actor == "being.glaushouse.current-prima-donna")
    );
    let identity = service.runtime().hueman().unwrap().clone();
    step(&mut service, A::Rest);
    step(&mut service, A::AskClinicianToDischarge);
    assert_eq!(
        service.runtime().sympiote_foundation().phase(),
        P::Discharged
    );
    assert_eq!(service.runtime().hueman().unwrap(), &identity);
    let graft = service.runtime().sympiote_foundation().graft().unwrap();
    assert_eq!(
        decode_sympiote_history(&encode_sympiote_history(graft).unwrap()).unwrap(),
        *graft
    );
}

#[test]
fn withdrawal_is_preserved_without_form_or_debt_and_can_be_reconsidered() {
    let mut service = application();
    step(&mut service, A::RequestAssessment);
    step(&mut service, A::ConsentToFoundation);
    step(&mut service, A::WithdrawConsent);
    let state = service.runtime().sympiote_foundation();
    assert_eq!(state.phase(), P::Refused);
    assert!(!state.form_established());
    assert!(state.graft().is_none());
    assert!(service.runtime().constitutional().events().is_empty());
    let archive = service.encode_archive().unwrap();
    let replayed = GameApplicationService::from_archive(&archive).unwrap();
    assert_eq!(replayed.runtime().sympiote_foundation(), state);
    step(&mut service, A::RequestAssessment);
    assert_eq!(
        service.runtime().sympiote_foundation().phase(),
        P::AwaitingConsent
    );
}

#[test]
fn repeated_practice_requires_release_rest_and_same_adaptation() {
    let mut service = application();
    discharge(&mut service);
    step(
        &mut service,
        A::Practice {
            adaptation: SympioteAdaptation::Balance,
        },
    );
    let before = service.encode_archive().unwrap();
    let at = service.revision().get() + 1;
    assert!(
        service
            .execute(
                GameplayEventMetadata {
                    id: GameplayEventId::new("event.sympiote.skip-rest").unwrap(),
                    causal_position: CausalPosition::new(at)
                },
                GameplayCommand::AdvanceSympioteFoundation {
                    action: A::Practice {
                        adaptation: SympioteAdaptation::Balance
                    }
                }
            )
            .is_err()
    );
    assert_eq!(before, service.encode_archive().unwrap());
    step(&mut service, A::Rest);
    step(
        &mut service,
        A::Practice {
            adaptation: SympioteAdaptation::Tension,
        },
    );
    step(&mut service, A::Rest);
    assert!(!service.runtime().sympiote_foundation().comfortable_use());
    step(
        &mut service,
        A::Practice {
            adaptation: SympioteAdaptation::Tension,
        },
    );
    assert!(!service.runtime().sympiote_foundation().comfortable_use());
    step(&mut service, A::Rest);
    assert!(service.runtime().sympiote_foundation().comfortable_use());
    assert!(audit_sympiote_foundation(service.runtime().sympiote_foundation()).passed());
}

#[test]
fn strain_preserves_the_form_and_demands_clinical_review_before_return() {
    let mut service = application();
    discharge(&mut service);
    step(&mut service, A::ReportStrain);
    let state = service.runtime().sympiote_foundation();
    assert!(state.form_established());
    assert_eq!(state.phase(), P::CareRequired);
    assert_eq!(state.available_actions(), vec![A::Rest]);
    step(&mut service, A::Rest);
    assert_eq!(
        service.runtime().sympiote_foundation().available_actions(),
        vec![A::AskClinicianToReview]
    );
    step(&mut service, A::AskClinicianToReview);
    assert_eq!(
        service.runtime().sympiote_foundation().phase(),
        P::Recovering
    );
    step(&mut service, A::Rest);
    step(&mut service, A::AskClinicianToDischarge);
    assert_eq!(
        service.runtime().sympiote_foundation().phase(),
        P::Discharged
    );
}

#[test]
fn absent_or_changed_clinician_cannot_reuse_old_scope() {
    let world = live_world();
    let mut state = SympioteFoundationState::default();
    state
        .advance("being.sympiote.patient", A::RequestAssessment, &world, 1)
        .unwrap();
    state
        .advance("being.sympiote.patient", A::ConsentToFoundation, &world, 2)
        .unwrap();
    let before = state.clone();
    let mut institutions = world.institutional().clone();
    institutions
        .catalog
        .office_holders
        .retain(|holder| holder.office.as_str() != "office.glaushouse.prima-donna");
    let absent = WorldSession::from_institutional_state(institutions).unwrap();
    assert!(
        state
            .advance(
                "being.sympiote.patient",
                A::AskClinicianToEstablish,
                &absent,
                3
            )
            .is_err()
    );
    assert_eq!(state, before);
    let mut institutions = world.institutional().clone();
    institutions
        .catalog
        .office_holders
        .iter_mut()
        .find(|holder| holder.active && holder.office.as_str() == "office.glaushouse.prima-donna")
        .unwrap()
        .being = InstitutionalBeingId::new("being.test.replacement-clinician").unwrap();
    let changed = WorldSession::from_institutional_state(institutions).unwrap();
    assert!(
        state
            .advance(
                "being.sympiote.patient",
                A::AskClinicianToEstablish,
                &changed,
                3
            )
            .is_err()
    );
    assert_eq!(state, before);
    assert!(
        state
            .advance("being.other-patient", A::AskClinicianToEstablish, &world, 3)
            .is_err()
    );
    assert_eq!(state, before);
}

#[test]
fn all_directions_and_both_offers_preserve_identity_without_granting_other_forms() {
    let mut service = application();
    discharge(&mut service);
    let identity = service.runtime().hueman().unwrap().clone();
    let graft = service
        .runtime()
        .sympiote_foundation()
        .graft()
        .unwrap()
        .clone();
    for direction in SympiotePracticeDirection::ALL {
        step(&mut service, A::ExploreDirection { direction });
    }
    for camp in [
        SympioteBoardwalkCamp::PimpAndTrooners,
        SympioteBoardwalkCamp::GimpAndGooners,
    ] {
        step(&mut service, A::ConsiderBoardwalkOffer { camp });
    }
    assert_eq!(service.runtime().hueman().unwrap(), &identity);
    assert_eq!(
        service.runtime().sympiote_foundation().graft().unwrap(),
        &graft
    );
    assert_eq!(
        service
            .runtime()
            .sympiote_foundation()
            .considered_offers()
            .len(),
        2
    );
    assert_eq!(service.runtime().regional().events().len(), 0);
    assert_eq!(service.runtime().constitutional().events().len(), 0);
    assert_eq!(
        service.runtime().winged_gargoyle_pilot().form(),
        hollow_grove::gameplay::WingedGargoylePilotForm::Hueman
    );
    let replayed =
        GameApplicationService::from_archive(&service.encode_archive().unwrap()).unwrap();
    assert_eq!(
        replayed.runtime().sympiote_foundation(),
        service.runtime().sympiote_foundation()
    );
}

#[test]
fn every_checkpoint_replays_and_tampered_receipts_fail_even_with_valid_checksum() {
    let mut service = application();
    for action in [
        A::RequestAssessment,
        A::ConsentToFoundation,
        A::AskClinicianToEstablish,
        A::Rest,
        A::AskClinicianToDischarge,
        A::Practice {
            adaptation: SympioteAdaptation::Coordination,
        },
        A::Rest,
    ] {
        step(&mut service, action);
        let replayed =
            GameApplicationService::from_archive(&service.encode_archive().unwrap()).unwrap();
        assert_eq!(
            replayed.runtime().sympiote_foundation(),
            service.runtime().sympiote_foundation()
        );
    }
    let mut events = service.events().to_vec();
    let hollow_grove::gameplay::GameplayEvent::SympioteFoundationChanged { event } =
        &mut events[1].payload
    else {
        panic!("assessment event expected")
    };
    event.clinical_receipt.as_mut().unwrap().authority_actor = "being.boardwalk.pimp".into();
    let altered = hollow_grove::gameplay::encode_gameplay_archive(
        &rules(),
        &events,
        service.runtime().world(),
    )
    .unwrap();
    assert!(GameApplicationService::from_archive(&altered).is_err());
    let mut value: serde_json::Value =
        serde_json::from_str(&service.encode_archive().unwrap()).unwrap();
    value["payload"]["events"][1]["payload"]["event"]["form_established"] = true.into();
    assert!(GameApplicationService::from_archive(&serde_json::to_string(&value).unwrap()).is_err());
}

fn request(id: &str, revision: u64, intent: GameplayIntent) -> ProtocolRequestEnvelope {
    ProtocolRequestEnvelope {
        protocol_version: HOLLOW_GROVE_GAMEPLAY_PROTOCOL_VERSION,
        session_id: "session.sympiote".into(),
        request_id: id.into(),
        expected_revision: revision,
        intent,
    }
}

#[test]
fn overflow_and_client_supplied_form_overrides_leave_no_partial_graft() {
    let world = live_world();
    let mut state = SympioteFoundationState::default();
    state
        .advance("being.sympiote.patient", A::RequestAssessment, &world, 1)
        .unwrap();
    state
        .advance("being.sympiote.patient", A::ConsentToFoundation, &world, 2)
        .unwrap();
    let before = state.clone();
    assert!(
        state
            .advance(
                "being.sympiote.patient",
                A::AskClinicianToEstablish,
                &world,
                u64::MAX
            )
            .is_err()
    );
    assert_eq!(state, before);
    let encoded = r#"{"type":"AdvanceSympioteFoundationIntent","action":{"Practice":{"adaptation":"Balance","new_form":"Chimera"}}}"#;
    assert!(serde_json::from_str::<GameplayIntent>(encoded).is_err());
}

#[test]
fn service_projects_actions_audit_and_retries_without_repeating_the_graft() {
    let save_root = std::env::temp_dir().join(format!(
        "hollow-grove-sympiote-protocol-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let mut service = GameProtocolService::new_with_roots(
        "session.sympiote",
        rules(),
        save_root,
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")),
    )
    .unwrap();
    let entered = service.handle(request(
        "request.enter",
        0,
        GameplayIntent::EnterHollowGroveIntent,
    ));
    assert_eq!(entered.status, ProtocolResponseStatus::Completed);
    let response = service.handle(request(
        "request.identity",
        entered.view.as_ref().unwrap().revision,
        GameplayIntent::EstablishHuemanIntent {
            continuity_id: "continuity.sympiote.patient".into(),
            participant_id: "participant.sympiote.patient".into(),
            institutional_being_id: "being.sympiote.patient".into(),
        },
    ));
    assert_eq!(response.status, ProtocolResponseStatus::Completed);
    let mut revision = response.view.as_ref().unwrap().revision;
    for (index, action) in [
        A::RequestAssessment,
        A::ConsentToFoundation,
        A::AskClinicianToEstablish,
        A::Rest,
        A::AskClinicianToDischarge,
    ]
    .into_iter()
    .enumerate()
    {
        let req = request(
            &format!("request.sympiote.{index}"),
            revision,
            GameplayIntent::AdvanceSympioteFoundationIntent { action },
        );
        let response = service.handle(req.clone());
        assert_eq!(response.status, ProtocolResponseStatus::Completed);
        assert!(response.view.as_ref().unwrap().kernel_audit.passed);
        assert_eq!(service.handle(req), response);
        revision = response.view.as_ref().unwrap().revision;
    }
    let response = service.handle(request(
        "request.sync",
        revision,
        GameplayIntent::SyncIntent,
    ));
    assert!(
        response
            .view
            .as_ref()
            .unwrap()
            .sympiote_foundation
            .form_established
    );
    assert_eq!(
        response
            .view
            .as_ref()
            .unwrap()
            .sympiote_foundation
            .clinical_receipts
            .len(),
        3
    );
    assert_eq!(
        response.view.as_ref().unwrap().sympiote_foundation.phase,
        P::Discharged
    );
    assert!(
        response
            .view
            .as_ref()
            .unwrap()
            .kernel_audit
            .checked_invariants
            .iter()
            .any(|name| name == "sympiote-clinical-consent-and-authority-preserved")
    );
}
