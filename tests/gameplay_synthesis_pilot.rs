use hollow_grove::constitutional::{CausalPosition, ParticipantId, RuleSetId, V2_RULE_SET};
use hollow_grove::gameplay::{
    BeingContinuityId, GameApplicationService, GameProtocolService, GameplayCommand,
    GameplayEventId, GameplayEventKind, GameplayEventMetadata, GameplayIntent,
    HOLLOW_GROVE_GAMEPLAY_PROTOCOL_VERSION, HollowGroveGameRuntime, ProtocolRequestEnvelope,
    ProtocolResponseStatus, WingedGargoylePilotAction, WingedGargoylePilotForm,
};
use hollow_grove::institution::InstitutionalBeingId;
use hollow_grove::recipe_discovery::GargoyleHuntStage;

const ACTIONS: [WingedGargoylePilotAction; 9] = [
    WingedGargoylePilotAction::RecoverStonebendStructure,
    WingedGargoylePilotAction::RecoverSandmanorMeasurement,
    WingedGargoylePilotAction::ObtainGlaushouseCompatibility,
    WingedGargoylePilotAction::EarnFlyntWitness,
    WingedGargoylePilotAction::RecoverStonebendMaterial,
    WingedGargoylePilotAction::PreserveSandmanorMethod,
    WingedGargoylePilotAction::RecordStableTrial,
    WingedGargoylePilotAction::GrantClinicalClearance,
    WingedGargoylePilotAction::CompleteSafeFlightAndReturn,
];

fn rules() -> RuleSetId {
    RuleSetId::new(V2_RULE_SET).unwrap()
}

fn execute(service: &mut GameApplicationService, at: u64, command: GameplayCommand) {
    service
        .execute(
            GameplayEventMetadata {
                id: GameplayEventId::new(format!("game-event.gargoyle-pilot.{at}")).unwrap(),
                causal_position: CausalPosition::new(at),
            },
            command,
        )
        .unwrap();
}

fn establish_hueman(service: &mut GameApplicationService) {
    execute(
        service,
        1,
        GameplayCommand::EstablishHuemanIdentity {
            continuity: BeingContinuityId::new("being-continuity.hueman").unwrap(),
            participant: ParticipantId::new("participant.hueman").unwrap(),
            institutional: InstitutionalBeingId::new("being.hueman").unwrap(),
        },
    );
}

#[test]
fn runtime_requires_hueman_and_keeps_out_of_order_progress_transactional() {
    let mut runtime = HollowGroveGameRuntime::new(rules());
    let before = runtime.winged_gargoyle_pilot().clone();
    let error = runtime
        .execute(
            GameplayEventMetadata {
                id: GameplayEventId::new("game-event.gargoyle-pilot.no-hueman").unwrap(),
                causal_position: CausalPosition::new(1),
            },
            GameplayCommand::AdvanceWingedGargoylePilot {
                action: WingedGargoylePilotAction::RecoverStonebendStructure,
            },
        )
        .unwrap_err();
    assert!(error.to_string().contains("SynthesisRequiresHueman"));
    assert_eq!(runtime.revision().get(), 0);
    assert_eq!(runtime.winged_gargoyle_pilot(), &before);

    let mut service = GameApplicationService::new(rules());
    establish_hueman(&mut service);
    let before = service.runtime().winged_gargoyle_pilot().clone();
    assert!(
        service
            .execute(
                GameplayEventMetadata {
                    id: GameplayEventId::new("game-event.gargoyle-pilot.out-of-order").unwrap(),
                    causal_position: CausalPosition::new(2),
                },
                GameplayCommand::AdvanceWingedGargoylePilot {
                    action: WingedGargoylePilotAction::RecordStableTrial,
                },
            )
            .is_err()
    );
    assert_eq!(service.revision().get(), 1);
    assert_eq!(service.runtime().winged_gargoyle_pilot(), &before);
}

#[test]
fn canonical_hunt_transforms_unlocks_and_returns_without_losing_capability() {
    let mut service = GameApplicationService::new(rules());
    establish_hueman(&mut service);
    for (index, action) in ACTIONS.into_iter().enumerate() {
        execute(
            &mut service,
            u64::try_from(index).unwrap() + 2,
            GameplayCommand::AdvanceWingedGargoylePilot { action },
        );
        let pilot = service.runtime().winged_gargoyle_pilot();
        match action {
            WingedGargoylePilotAction::RecordStableTrial => {
                assert_eq!(pilot.form(), WingedGargoylePilotForm::Gargoyle);
                assert_eq!(pilot.stage(), GargoyleHuntStage::TrialSynthesis);
            }
            WingedGargoylePilotAction::GrantClinicalClearance => {
                assert_eq!(pilot.form(), WingedGargoylePilotForm::WingedGargoyle);
                assert_eq!(pilot.stage(), GargoyleHuntStage::FinalClearance);
            }
            WingedGargoylePilotAction::CompleteSafeFlightAndReturn => {
                assert_eq!(pilot.form(), WingedGargoylePilotForm::Hueman);
                assert_eq!(pilot.stage(), GargoyleHuntStage::SafeFlightAndReturn);
            }
            _ => {}
        }
    }

    let pilot = service.runtime().winged_gargoyle_pilot();
    assert!(pilot.completed());
    assert_eq!(pilot.fragment_count(), 6);
    assert_eq!(pilot.capability_id(), Some("capability.flynt.telekinesis"));
    assert_eq!(
        pilot.power_recipe_id(),
        Some("recipe.flight.resynce-propulsion")
    );
    assert!(pilot.next_action().is_none());
}

#[test]
fn archive_replay_preserves_mid_hunt_and_completed_pilot_state() {
    let mut service = GameApplicationService::new(rules());
    establish_hueman(&mut service);
    for (index, action) in ACTIONS[..7].iter().copied().enumerate() {
        execute(
            &mut service,
            u64::try_from(index).unwrap() + 2,
            GameplayCommand::AdvanceWingedGargoylePilot { action },
        );
    }
    let archive = service.encode_archive().unwrap();
    let replayed = GameApplicationService::from_archive(&archive).unwrap();
    assert_eq!(
        replayed.runtime().winged_gargoyle_pilot(),
        service.runtime().winged_gargoyle_pilot()
    );
    assert_eq!(
        replayed.runtime().winged_gargoyle_pilot().form(),
        WingedGargoylePilotForm::Gargoyle
    );

    for (index, action) in ACTIONS[7..].iter().copied().enumerate() {
        execute(
            &mut service,
            u64::try_from(index).unwrap() + 9,
            GameplayCommand::AdvanceWingedGargoylePilot { action },
        );
    }
    let replayed =
        GameApplicationService::from_archive(&service.encode_archive().unwrap()).unwrap();
    assert!(replayed.runtime().winged_gargoyle_pilot().completed());
    assert_eq!(
        replayed.runtime().winged_gargoyle_pilot().capability_id(),
        Some("capability.flynt.telekinesis")
    );
}

fn protocol_request(id: &str, revision: u64, intent: GameplayIntent) -> ProtocolRequestEnvelope {
    ProtocolRequestEnvelope {
        protocol_version: HOLLOW_GROVE_GAMEPLAY_PROTOCOL_VERSION,
        session_id: "session.gargoyle-pilot".into(),
        request_id: id.into(),
        expected_revision: revision,
        intent,
    }
}

#[test]
fn protocol_projects_each_next_action_and_closes_capability_after_safe_return() {
    let mut service = GameProtocolService::new("session.gargoyle-pilot", rules()).unwrap();
    let established = service.handle(protocol_request(
        "request.gargoyle-pilot.establish",
        0,
        GameplayIntent::EstablishHuemanIntent {
            continuity_id: "being-continuity.hueman".into(),
            participant_id: "participant.hueman".into(),
            institutional_being_id: "being.hueman".into(),
        },
    ));
    assert_eq!(established.status, ProtocolResponseStatus::Completed);
    assert_eq!(
        established
            .view
            .as_ref()
            .unwrap()
            .winged_gargoyle_pilot
            .next_action,
        Some(ACTIONS[0])
    );

    let mut revision = established.revision;
    for (index, action) in ACTIONS.into_iter().enumerate() {
        let response = service.handle(protocol_request(
            &format!("request.gargoyle-pilot.step-{index}"),
            revision,
            GameplayIntent::AdvanceWingedGargoylePilotIntent { action },
        ));
        assert_eq!(response.status, ProtocolResponseStatus::Completed);
        assert_eq!(response.events.len(), 1);
        assert_eq!(
            response.events[0].kind,
            if index < 6 {
                GameplayEventKind::SynthesisAvailable
            } else {
                GameplayEventKind::FrameChanged
            }
        );
        revision = response.revision;
    }

    let view = service.handle(protocol_request(
        "request.gargoyle-pilot.sync",
        revision,
        GameplayIntent::SyncIntent,
    ));
    let view = view.view.unwrap();
    assert!(view.winged_gargoyle_pilot.completed);
    assert_eq!(
        view.winged_gargoyle_pilot.form,
        WingedGargoylePilotForm::Hueman
    );
    assert!(view.winged_gargoyle_pilot.next_action.is_none());
    assert!(view.capabilities.iter().any(|capability| {
        capability.intent_type == "AdvanceWingedGargoylePilotIntent" && !capability.available
    }));
}
