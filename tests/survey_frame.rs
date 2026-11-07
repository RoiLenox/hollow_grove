use std::path::Path;

use hollow_grove::constitutional::{CausalPosition, ParticipantId, RuleSetId, V2_RULE_SET};
use hollow_grove::gameplay::{
    BeingContinuityId, GameApplicationService, GameProtocolService, GameView, GameplayCommand,
    GameplayEvent, GameplayEventId, GameplayEventMetadata, GameplayIntent, ProtocolRequestEnvelope,
    ProtocolResponseStatus, SurveyFrameAction as A, SurveyFrameClaim, SurveyFrameDisposition as D,
    SurveyFramePhase as P, SurveyFrameResidentChoice as C, SurveyFrameUse as U,
    encode_gameplay_archive,
};
use hollow_grove::institution::InstitutionalBeingId;
use hollow_grove::world::session::WorldSession;
use hollow_grove::world::survey_frame::{
    SURVEY_FRAME_KEEPER, SURVEY_FRAME_OBJECT_ID, SURVEY_FRAME_RECORDER, SURVEY_FRAME_RESIDENT,
};

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}
fn rules() -> RuleSetId {
    RuleSetId::new(V2_RULE_SET).unwrap()
}
fn app_with(world: WorldSession) -> GameApplicationService {
    let mut service = GameApplicationService::with_world_session(rules(), world);
    service
        .execute(
            meta(1),
            GameplayCommand::EstablishHuemanIdentity {
                continuity: BeingContinuityId::new("continuity.survey-frame.player").unwrap(),
                participant: ParticipantId::new("participant.survey-frame.player").unwrap(),
                institutional: InstitutionalBeingId::new("being.survey-frame.player").unwrap(),
            },
        )
        .unwrap();
    service
}
fn app() -> GameApplicationService {
    app_with(WorldSession::load_or_canonical_at(root()).unwrap())
}
fn meta(at: u64) -> GameplayEventMetadata {
    GameplayEventMetadata {
        id: GameplayEventId::new(format!("event.survey-frame.test.{at}")).unwrap(),
        causal_position: CausalPosition::new(at),
    }
}
fn step(service: &mut GameApplicationService, action: A) {
    service
        .execute(
            meta(service.events().len() as u64 + 1),
            GameplayCommand::AdvanceSurveyFrame { action },
        )
        .unwrap();
    assert!(
        hollow_grove::kernel_invariants::audit_survey_frame(service.runtime().survey_frame())
            .passed()
    );
}
fn reject(service: &mut GameApplicationService, action: A) {
    let before = service.encode_archive().unwrap();
    assert!(
        service
            .execute(
                meta(service.events().len() as u64 + 1),
                GameplayCommand::AdvanceSurveyFrame { action }
            )
            .is_err()
    );
    assert_eq!(service.encode_archive().unwrap(), before);
}
fn measuring(service: &mut GameApplicationService) {
    for action in [
        A::ReceiveNeed,
        A::RecoverParts,
        A::AskStonebendToRegister,
        A::AskReviewerToMeasure,
    ] {
        step(service, action);
    }
}
fn failed_rig(service: &mut GameApplicationService) {
    measuring(service);
    for action in [
        A::PrepareRig,
        A::AskReviewerToMeasure,
        A::OfferUse {
            use_scope: U::TemporaryRig,
        },
        A::AskReviewerToTrialSupport,
    ] {
        step(service, action);
    }
}
fn decide(service: &mut GameApplicationService, choice: C) {
    step(service, A::SupportResidentChoice { choice });
    step(service, A::AskResidentToDecide);
}
fn repaired_rig(service: &mut GameApplicationService) {
    failed_rig(service);
    decide(service, C::WaitSafely);
    for action in [
        A::RepairJoint,
        A::AskReviewerToMeasure,
        A::AskReviewerToTrialSupport,
        A::OfferUse {
            use_scope: U::TemporaryRig,
        },
    ] {
        step(service, action);
    }
}

#[test]
fn artifact_registration_and_measurement_do_not_grant_support_or_player_form() {
    let mut service = app();
    let hueman = service.runtime().hueman().unwrap().clone();
    let foundation = service.runtime().sympiote_foundation().clone();
    let pilot = service.runtime().winged_gargoyle_pilot().clone();
    let dormant = GameView::from_runtime(service.runtime(), vec![]).survey_frame;
    assert!(dormant.object_id.is_none() && dormant.registration.is_none() && !dormant.complete);
    measuring(&mut service);
    let frame = service.runtime().survey_frame();
    assert!(frame.measurement_proven() && !frame.support_proven());
    let name = frame.artifact_name().unwrap();
    assert_eq!(name.subject.as_str(), SURVEY_FRAME_OBJECT_ID);
    assert_eq!(
        name.class,
        hollow_grove::world::stonebend::NameClass::Artifact
    );
    assert_eq!(frame.registration().unwrap().function, "Name");
    assert_eq!(
        frame.revisions()[0].measurement_receipts[0].function,
        "Prove"
    );
    reject(
        &mut service,
        A::DeliverDisposition {
            disposition: D::TemporaryRigDelivered,
        },
    );
    reject(
        &mut service,
        A::SupportResidentChoice {
            choice: C::AcceptTemporaryRig,
        },
    );
    assert_eq!(service.runtime().hueman().unwrap(), &hueman);
    assert_eq!(service.runtime().sympiote_foundation(), &foundation);
    assert_eq!(service.runtime().winged_gargoyle_pilot(), &pilot);
}

#[test]
fn negative_trial_is_committed_and_repair_requires_fresh_proof_and_new_offer() {
    let mut service = app();
    failed_rig(&mut service);
    let frame = service.runtime().survey_frame();
    assert_eq!(frame.phase(), P::RigIsolated);
    assert!(!frame.measurement_proven() && !frame.support_proven());
    assert_eq!(
        frame.revisions()[1].support_receipt.as_ref().unwrap().claim,
        SurveyFrameClaim::FailedSupportTrial
    );
    let before = service.events().to_vec();
    decide(&mut service, C::WaitSafely);
    step(&mut service, A::RepairJoint);
    assert_eq!(&service.events()[..before.len()], before);
    let frame = service.runtime().survey_frame();
    assert_eq!(frame.object_revision(), 3);
    assert!(
        frame.revisions()[2]
            .retained_parts
            .iter()
            .any(|p| p.contains("damaged-joint"))
    );
    assert!(frame.revisions()[1].support_receipt.is_some());
    assert!(!frame.measurement_proven() && !frame.support_proven());
    reject(&mut service, A::AskReviewerToTrialSupport);
    reject(
        &mut service,
        A::SupportResidentChoice {
            choice: C::AcceptTemporaryRig,
        },
    );
    step(&mut service, A::AskReviewerToMeasure);
    step(&mut service, A::AskReviewerToTrialSupport);
    reject(
        &mut service,
        A::SupportResidentChoice {
            choice: C::AcceptTemporaryRig,
        },
    );
    step(
        &mut service,
        A::OfferUse {
            use_scope: U::TemporaryRig,
        },
    );
    let last_decision = service
        .runtime()
        .survey_frame()
        .resident_decisions()
        .last()
        .unwrap();
    assert_eq!(last_decision.choice, C::WaitSafely);
    assert_eq!(
        last_decision.revision, 2,
        "repair does not rewrite a refusal"
    );
    reject(
        &mut service,
        A::DeliverDisposition {
            disposition: D::TemporaryRigDelivered,
        },
    );
}

#[test]
fn player_advice_is_not_resident_acceptance_and_all_three_endings_replay() {
    for outcome in [
        D::MeasurementsDelivered,
        D::TemporaryRigDelivered,
        D::ProtectedDelayDelivered,
    ] {
        let mut service = app();
        let choice = match outcome {
            D::MeasurementsDelivered => {
                measuring(&mut service);
                step(
                    &mut service,
                    A::OfferUse {
                        use_scope: U::MeasuringOnly,
                    },
                );
                C::AcceptMeasurements
            }
            D::TemporaryRigDelivered => {
                repaired_rig(&mut service);
                C::AcceptTemporaryRig
            }
            D::ProtectedDelayDelivered => {
                failed_rig(&mut service);
                C::WaitSafely
            }
        };
        let count = service.runtime().survey_frame().resident_decisions().len();
        step(&mut service, A::SupportResidentChoice { choice });
        assert_eq!(
            service.runtime().survey_frame().resident_decisions().len(),
            count
        );
        reject(
            &mut service,
            A::DeliverDisposition {
                disposition: outcome,
            },
        );
        step(&mut service, A::AskResidentToDecide);
        let decision = service
            .runtime()
            .survey_frame()
            .resident_decisions()
            .last()
            .unwrap();
        assert_eq!(decision.resident_id, SURVEY_FRAME_RESIDENT);
        assert_eq!(
            decision.participant_id,
            "participant.current-sea.mercy-deep"
        );
        if choice != C::WaitSafely {
            assert!(decision.proof_decision_id.is_some());
        }
        step(
            &mut service,
            A::DeliverDisposition {
                disposition: outcome,
            },
        );
        let state = service.runtime().survey_frame();
        assert!(state.complete());
        let delivery = state.deliveries().last().unwrap();
        assert_eq!(delivery.disposition, outcome);
        assert_eq!(delivery.custodian, SURVEY_FRAME_KEEPER);
        assert_eq!(delivery.recovery_owner, SURVEY_FRAME_KEEPER);
        assert_eq!(delivery.recorder, SURVEY_FRAME_RECORDER);
        assert_eq!(delivery.recipient, SURVEY_FRAME_RESIDENT);
        let archive = service.encode_archive().unwrap();
        let replayed = GameApplicationService::from_archive(&archive).unwrap();
        assert_eq!(replayed.runtime().survey_frame(), state);
        assert_eq!(replayed.encode_archive().unwrap(), archive);
        let inspection =
            hollow_grove::gameplay::LoreInspection::from_runtime(Some(replayed.runtime())).unwrap();
        assert_eq!(inspection.schema_version, 4);
        assert!(inspection.survey_frame.unwrap().complete);
        let coverage = hollow_grove::architecture_audit::ArchitectureCoverageReport::inspect(
            root(),
            Some(replayed.runtime()),
        )
        .unwrap();
        assert!(coverage.source_links_passed);
        assert_eq!(coverage.schema_version, 3);
        assert_eq!(coverage.verified_binding_count, 14);
        assert_eq!(coverage.unbound_canonical_route_count, 0);
        assert_eq!(coverage.backend_handoff_binding_count, 10);
        assert_eq!(coverage.unbound_gameplay_route_count, 10);
        assert!(
            coverage
                .canonical_route_contracts
                .iter()
                .all(|c| c.execution_complete_in_supplied_handoff.is_none())
        );
        assert_eq!(
            coverage
                .additional_survey_frame_contract
                .execution_complete_in_supplied_save,
            Some(true)
        );
    }
}

#[test]
fn safe_pause_can_end_without_inventing_an_artifact_or_authority() {
    let mut service = app_with(WorldSession::canonical());
    step(&mut service, A::ReceiveNeed);
    decide(&mut service, C::WaitSafely);
    step(
        &mut service,
        A::DeliverDisposition {
            disposition: D::ProtectedDelayDelivered,
        },
    );
    let frame = service.runtime().survey_frame();
    assert!(frame.complete());
    assert_eq!(frame.object_revision(), 0);
    assert!(frame.registration().is_none() && frame.artifact_name().is_none());
    assert!(
        GameView::from_runtime(service.runtime(), vec![])
            .survey_frame
            .object_id
            .is_none()
    );
}

#[test]
fn missing_authority_and_early_commands_fail_atomically_without_losing_work() {
    let mut service = app_with(WorldSession::canonical());
    reject(&mut service, A::RecoverParts);
    step(&mut service, A::ReceiveNeed);
    step(&mut service, A::RecoverParts);
    reject(&mut service, A::AskStonebendToRegister);
    assert_eq!(service.runtime().survey_frame().object_revision(), 1);
    assert!(service.runtime().survey_frame().registration().is_none());
    let missing = GameView::from_runtime(service.runtime(), vec![])
        .survey_frame
        .missing_authorities;
    assert!(missing.contains(&"office.stonebend.hypergiant".to_owned()));
    assert!(missing.contains(&"office.sandmanor.sandman".to_owned()));
    decide(&mut service, C::WaitSafely);
    step(
        &mut service,
        A::DeliverDisposition {
            disposition: D::ProtectedDelayDelivered,
        },
    );
}

#[test]
fn refusing_support_preserves_separately_accepted_measuring_scope() {
    let mut service = app();
    measuring(&mut service);
    step(&mut service, A::PrepareRig);
    step(&mut service, A::AskReviewerToMeasure);
    step(
        &mut service,
        A::OfferUse {
            use_scope: U::MeasuringOnly,
        },
    );
    decide(&mut service, C::AcceptMeasurements);
    step(
        &mut service,
        A::OfferUse {
            use_scope: U::TemporaryRig,
        },
    );
    decide(&mut service, C::WaitSafely);
    assert!(
        service
            .runtime()
            .survey_frame()
            .available_actions()
            .contains(&A::DeliverDisposition {
                disposition: D::MeasurementsDelivered
            })
    );
    reject(
        &mut service,
        A::SupportResidentChoice {
            choice: C::AcceptTemporaryRig,
        },
    );
    step(
        &mut service,
        A::DeliverDisposition {
            disposition: D::MeasurementsDelivered,
        },
    );
    assert!(!service.runtime().survey_frame().deployed());
}

#[test]
fn withdrawal_stops_deployment_and_withholds_completion_until_recovery_and_return() {
    let mut service = app();
    repaired_rig(&mut service);
    decide(&mut service, C::AcceptTemporaryRig);
    step(
        &mut service,
        A::DeliverDisposition {
            disposition: D::TemporaryRigDelivered,
        },
    );
    let before = service.events().to_vec();
    step(&mut service, A::AskResidentToWithdraw);
    assert!(!service.runtime().survey_frame().deployed());
    assert!(!service.runtime().survey_frame().complete());
    assert!(service.runtime().survey_frame().recovery_required());
    reject(
        &mut service,
        A::DeliverDisposition {
            disposition: D::ProtectedDelayDelivered,
        },
    );
    step(&mut service, A::RecoverDeployedRig);
    assert!(!service.runtime().survey_frame().complete());
    assert_eq!(
        service.runtime().survey_frame().phase(),
        P::AwaitingProtectedReturn
    );
    step(
        &mut service,
        A::DeliverDisposition {
            disposition: D::ProtectedDelayDelivered,
        },
    );
    assert!(service.runtime().survey_frame().complete());
    assert_eq!(service.runtime().survey_frame().deliveries().len(), 2);
    assert_eq!(&service.events()[..before.len()], before);
    let archive = service.encode_archive().unwrap();
    let restored = GameApplicationService::from_archive(&archive).unwrap();
    assert_eq!(
        restored.runtime().survey_frame(),
        service.runtime().survey_frame()
    );
}

#[test]
fn retry_does_not_repeat_a_physical_trial_and_forged_results_fail_checksummed_replay() {
    let mut service = app();
    failed_rig(&mut service);
    let event = service.events().last().unwrap().clone();
    let before = service.encode_archive().unwrap();
    assert_eq!(
        service
            .execute(
                GameplayEventMetadata {
                    id: event.id.clone(),
                    causal_position: event.causal_position
                },
                GameplayCommand::AdvanceSurveyFrame {
                    action: A::AskReviewerToTrialSupport
                }
            )
            .unwrap(),
        &event
    );
    assert_eq!(service.encode_archive().unwrap(), before);
    assert!(
        service
            .execute(
                GameplayEventMetadata {
                    id: event.id,
                    causal_position: event.causal_position
                },
                GameplayCommand::AdvanceSurveyFrame {
                    action: A::RepairJoint
                }
            )
            .is_err()
    );
    for kind in 0..3 {
        let mut events = service.events().to_vec();
        let GameplayEvent::SurveyFrameChanged { event } = &mut events.last_mut().unwrap().payload
        else {
            panic!("trial")
        };
        match kind {
            0 => event.authority.as_mut().unwrap().authority_actor = "being.forged.reviewer".into(),
            1 => event.authority.as_mut().unwrap().claim = SurveyFrameClaim::ScopedTemporarySupport,
            _ => event.object_revision = 3,
        }
        let forged = encode_gameplay_archive(
            service.runtime().rule_set(),
            &events,
            service.runtime().world_session(),
        )
        .unwrap();
        assert!(GameApplicationService::from_archive(&forged).is_err());
    }
}

#[test]
fn protocol_restart_preserves_frame_history_and_rejects_client_supplied_truth() {
    let temp = std::env::temp_dir().join(format!(
        "hollow-grove-frame-protocol-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(&temp).unwrap();
    let session = "session.survey-frame.protocol";
    let mut service = GameProtocolService::new_with_roots(session, rules(), &temp, root()).unwrap();
    let mut next = 0u64;
    let mut revision = 0u64;
    let mut request = |service: &mut GameProtocolService, intent| {
        next += 1;
        let response = service.handle(ProtocolRequestEnvelope {
            protocol_version: 1,
            session_id: session.into(),
            request_id: format!("request.frame.{next}"),
            expected_revision: revision,
            intent,
        });
        assert_eq!(
            response.status,
            ProtocolResponseStatus::Completed,
            "{response:?}"
        );
        revision = response.revision;
        assert!(response.view.as_ref().unwrap().kernel_audit.passed);
        response.view.unwrap()
    };
    request(&mut service, GameplayIntent::EnterHollowGroveIntent);
    request(
        &mut service,
        GameplayIntent::EstablishHuemanIntent {
            continuity_id: "continuity.frame.protocol".into(),
            participant_id: "participant.frame.protocol".into(),
            institutional_being_id: "being.frame.protocol".into(),
        },
    );
    for action in [
        A::ReceiveNeed,
        A::RecoverParts,
        A::AskStonebendToRegister,
        A::AskReviewerToMeasure,
        A::PrepareRig,
        A::AskReviewerToMeasure,
        A::OfferUse {
            use_scope: U::TemporaryRig,
        },
        A::AskReviewerToTrialSupport,
        A::SupportResidentChoice {
            choice: C::WaitSafely,
        },
        A::AskResidentToDecide,
        A::RepairJoint,
        A::AskReviewerToMeasure,
        A::AskReviewerToTrialSupport,
        A::OfferUse {
            use_scope: U::TemporaryRig,
        },
        A::SupportResidentChoice {
            choice: C::AcceptTemporaryRig,
        },
        A::AskResidentToDecide,
        A::DeliverDisposition {
            disposition: D::TemporaryRigDelivered,
        },
    ] {
        request(
            &mut service,
            GameplayIntent::AdvanceSurveyFrameIntent { action },
        );
    }
    let before = request(
        &mut service,
        GameplayIntent::SaveIntent {
            slot_id: "slot.frame".into(),
        },
    )
    .survey_frame;
    drop(service);
    let mut service = GameProtocolService::new_with_roots(session, rules(), &temp, root()).unwrap();
    request(&mut service, GameplayIntent::SyncIntent);
    let after = request(
        &mut service,
        GameplayIntent::LoadIntent {
            slot_id: "slot.frame".into(),
        },
    )
    .survey_frame;
    assert_eq!(before, after);
    assert!(
        serde_json::to_vec(&GameView::from_runtime(
            service.application().runtime(),
            vec![]
        ))
        .unwrap()
        .len()
            < 65536
    );
    let malformed = serde_json::json!({"protocol_version":1,"session_id":session,"request_id":"request.forged-trial","expected_revision":0,
        "intent":{"type":"AdvanceSurveyFrameIntent","action":{"AskReviewerToTrialSupport":{"passed":true}}}});
    assert!(serde_json::from_value::<ProtocolRequestEnvelope>(malformed).is_err());
    drop(service);
    std::fs::remove_dir_all(temp).unwrap();
}

#[test]
fn workshop_projection_and_role_bindings_remain_read_only_and_renderer_owned() {
    let mut service = app();
    repaired_rig(&mut service);
    let before = service.encode_archive().unwrap();
    let view = GameView::from_runtime(service.runtime(), vec![]).survey_frame;
    assert_eq!(view.object_revision, 3);
    assert!(view.support_proven && !view.complete);
    assert_eq!(view.revisions.len(), 3);
    assert!(
        hollow_grove::world::survey_frame::survey_frame_lore()
            .operational_handoffs()
            .is_ok()
    );
    assert_eq!(before, service.encode_archive().unwrap());
    let gd =
        std::fs::read_to_string(root().join("hueman_godot/scripts/retro_overworld.gd")).unwrap();
    for required in [
        "KEY_O",
        "AdvanceSurveyFrameIntent",
        "available_actions",
        "ADVICE IS NOT MERCY'S DECISION",
    ] {
        assert!(gd.contains(required));
    }
    for forbidden in [
        "SurveyFrameState",
        "scenario_house_decision",
        "ScopedTemporarySupport",
    ] {
        assert!(!gd.contains(forbidden));
    }
}

#[test]
fn actual_schema_five_foundation_archive_retains_history_and_has_no_frame_progress() {
    let old = include_str!("fixtures/compatibility/gameplay_v5_sympiote.json");
    let decoded = hollow_grove::gameplay::decode_gameplay_archive_with_metadata(old).unwrap();
    assert_eq!(decoded.source_schema_version, 5);
    let service = GameApplicationService::from_archive(old).unwrap();
    assert_eq!(service.events(), decoded.events);
    assert!(service.runtime().sympiote_foundation().form_established());
    let view = GameView::from_runtime(service.runtime(), vec![]).survey_frame;
    assert_eq!(view.phase, P::AwaitingNeed);
    assert!(view.object_id.is_none() && view.registration.is_none() && view.deliveries.is_empty());
    let current = service.encode_archive().unwrap();
    let decoded_current =
        hollow_grove::gameplay::decode_gameplay_archive_with_metadata(&current).unwrap();
    assert_eq!(
        decoded_current.source_schema_version,
        hollow_grove::gameplay::GAMEPLAY_ARCHIVE_SCHEMA_VERSION
    );
    assert!(
        decoded_current
            .migration_history
            .iter()
            .any(|m| m.migration_id == "migration.gameplay.v5-to-v6.survey-frame")
    );
    let replayed = GameApplicationService::from_archive(&current).unwrap();
    assert_eq!(
        replayed.runtime().sympiote_foundation(),
        service.runtime().sympiote_foundation()
    );
    assert_eq!(replayed.runtime().hueman(), service.runtime().hueman());
    assert_eq!(replayed.encode_archive().unwrap(), current);
}

#[test]
fn frame_events_cannot_be_backported_into_old_checksummed_envelopes() {
    let mut service = app();
    step(&mut service, A::ReceiveNeed);
    let mut value: serde_json::Value =
        serde_json::from_str(&service.encode_archive().unwrap()).unwrap();
    for version in 2..=5 {
        value["schema_version"] = version.into();
        assert!(
            GameApplicationService::from_archive(&serde_json::to_string(&value).unwrap()).is_err()
        );
    }
}
