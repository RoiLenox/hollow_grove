use hollow_grove::constitutional::{
    CausalPosition, ConstitutionalRegion, ParticipantId, RegionalBeingId, RegionalStandingKind,
    RuleSetId, V2_RULE_SET, scenario_regional_metadata, scenario_regional_registration,
};
use hollow_grove::gameplay::{
    ActiveIncarnationRef, BeingContinuityId, CardinalDirection, GameApplicationService,
    GameplayCommand, GameplayEvent, GameplayEventId, GameplayEventMetadata, GameplayIdentityError,
    GameplayRuntimeError, StarterInteractionId, TilePosition,
};
use hollow_grove::institution::InstitutionalBeingId;
use hollow_grove::lineage_contract::SandmanorForm;

fn rules() -> RuleSetId {
    RuleSetId::new(V2_RULE_SET).expect("V2 rule set ID")
}

fn continuity(value: &str) -> BeingContinuityId {
    BeingContinuityId::new(value).expect("test continuity ID")
}

fn participant(value: &str) -> ParticipantId {
    ParticipantId::new(value).expect("test participant ID")
}

fn institutional(value: &str) -> InstitutionalBeingId {
    InstitutionalBeingId::new(value).expect("test institutional Being ID")
}

fn metadata(value: &str, at: u64) -> GameplayEventMetadata {
    GameplayEventMetadata {
        id: GameplayEventId::new(value).expect("test gameplay event ID"),
        causal_position: CausalPosition::new(at),
    }
}

fn hueman_command() -> GameplayCommand {
    GameplayCommand::EstablishHuemanIdentity {
        continuity: continuity("being-continuity.hueman"),
        participant: participant("participant.hueman"),
        institutional: institutional("being.hueman"),
    }
}

#[test]
fn application_establishes_one_permanent_hueman_identity() {
    let mut service = GameApplicationService::new(rules());
    let event = service
        .execute(metadata("game-event.hueman", 1), hueman_command())
        .expect("Hueman identity must be established");
    assert_eq!(event.sequence, 0);
    assert_eq!(event.revision.get(), 1);
    assert!(matches!(
        event.payload,
        GameplayEvent::HuemanIdentityEstablished { .. }
    ));

    let hueman = service.hueman().expect("Hueman continuity");
    assert_eq!(hueman.id().as_str(), "being-continuity.hueman");
    assert_eq!(hueman.incarnation(), &ActiveIncarnationRef::Hueman);
    assert_eq!(
        hueman.incarnation().legacy_being(),
        Some(hollow_grove::BeingId::Hueman)
    );
    assert_eq!(service.revision().get(), 1);
}

#[test]
fn hueman_movement_is_authoritative_collision_aware_and_replayable() {
    let mut service = GameApplicationService::new(rules());
    service
        .execute(metadata("game-event.walk.hueman", 1), hueman_command())
        .unwrap();
    assert_eq!(
        service.runtime().hueman_position(),
        Some(TilePosition {
            x: 9,
            y: 15,
            facing: CardinalDirection::North,
        })
    );

    let movement = service
        .execute(
            metadata("game-event.walk.north", 2),
            GameplayCommand::MoveHueman {
                direction: CardinalDirection::North,
            },
        )
        .unwrap();
    assert!(matches!(
        movement.payload,
        GameplayEvent::HuemanMovementResolved {
            from: TilePosition { x: 9, y: 15, .. },
            to: TilePosition { x: 9, y: 14, .. },
        }
    ));
    assert_eq!(service.revision().get(), 2);

    let expected_events = service.events().to_vec();
    let replayed = GameApplicationService::replay(rules(), expected_events.clone()).unwrap();
    assert_eq!(replayed.events(), expected_events);
    assert_eq!(
        replayed.runtime().hueman_position(),
        service.runtime().hueman_position()
    );
}

#[test]
fn faced_interaction_is_authoritative_replayable_and_cleared_by_movement() {
    let mut service = GameApplicationService::new(rules());
    service
        .execute(metadata("game-event.interact.hueman", 1), hueman_command())
        .unwrap();
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
        service
            .execute(
                metadata(
                    &format!("game-event.interact.step-{index}"),
                    u64::try_from(index).unwrap() + 2,
                ),
                GameplayCommand::MoveHueman { direction },
            )
            .unwrap();
    }
    assert_eq!(
        service.runtime().hueman_position(),
        Some(TilePosition {
            x: 5,
            y: 14,
            facing: CardinalDirection::North,
        })
    );

    let interaction = service
        .execute(
            metadata("game-event.interact.guide", 8),
            GameplayCommand::InteractHueman,
        )
        .unwrap();
    assert!(matches!(
        interaction.payload,
        GameplayEvent::HuemanInteractionOpened {
            target: StarterInteractionId::RidgefolkGuide,
            ..
        }
    ));
    assert_eq!(
        service.runtime().active_interaction(),
        Some(StarterInteractionId::RidgefolkGuide)
    );

    let expected_events = service.events().to_vec();
    let replayed = GameApplicationService::replay(rules(), expected_events.clone()).unwrap();
    assert_eq!(replayed.events(), expected_events);
    assert_eq!(
        replayed.runtime().active_interaction(),
        Some(StarterInteractionId::RidgefolkGuide)
    );

    service
        .execute(
            metadata("game-event.interact.leave", 9),
            GameplayCommand::MoveHueman {
                direction: CardinalDirection::East,
            },
        )
        .unwrap();
    assert_eq!(service.runtime().active_interaction(), None);
}

#[test]
fn interaction_without_a_faced_target_fails_without_mutation() {
    let mut service = GameApplicationService::new(rules());
    service
        .execute(metadata("game-event.empty.hueman", 1), hueman_command())
        .unwrap();
    let before_events = service.events().to_vec();
    let before_revision = service.revision();

    let error = service
        .execute(
            metadata("game-event.empty.interact", 2),
            GameplayCommand::InteractHueman,
        )
        .unwrap_err();
    assert_eq!(error, GameplayRuntimeError::NoInteractionTarget);
    assert_eq!(service.events(), before_events);
    assert_eq!(service.revision(), before_revision);
    assert_eq!(service.runtime().active_interaction(), None);
}

#[test]
fn second_hueman_is_rejected_without_changing_authoritative_state() {
    let mut service = GameApplicationService::new(rules());
    service
        .execute(metadata("game-event.only-hueman", 1), hueman_command())
        .unwrap();
    let before = service.events().to_vec();

    let error = service
        .execute(
            metadata("game-event.second-hueman", 2),
            GameplayCommand::EstablishHuemanIdentity {
                continuity: continuity("being-continuity.second-hueman"),
                participant: participant("participant.second-hueman"),
                institutional: institutional("being.second-hueman"),
            },
        )
        .unwrap_err();
    assert!(matches!(
        error,
        GameplayRuntimeError::Identity(GameplayIdentityError::HuemanAlreadyEstablished { .. })
    ));
    assert_eq!(service.events(), before);
    assert_eq!(service.revision().get(), 1);
}

#[test]
fn exact_event_retry_is_idempotent_and_conflicting_reuse_fails_closed() {
    let mut service = GameApplicationService::new(rules());
    let event_metadata = metadata("game-event.idempotent-hueman", 1);
    let command = hueman_command();
    let expected = service
        .execute(event_metadata.clone(), command.clone())
        .unwrap()
        .clone();

    let retried = service.execute(event_metadata.clone(), command).unwrap();
    assert_eq!(retried, &expected);
    assert_eq!(service.events().len(), 1);
    assert_eq!(service.revision().get(), 1);

    let error = service
        .execute(
            event_metadata,
            GameplayCommand::EstablishHuemanIdentity {
                continuity: continuity("being-continuity.conflicting-hueman"),
                participant: participant("participant.conflicting-hueman"),
                institutional: institutional("being.conflicting-hueman"),
            },
        )
        .unwrap_err();
    assert!(matches!(error, GameplayRuntimeError::EventIdConflict(_)));
    assert_eq!(service.events(), [expected]);
}

#[test]
fn regional_registration_and_identity_commit_as_one_gameplay_event() {
    let world = hollow_grove::world::institutional_access_fixture();
    let regional = RegionalBeingId::new("being.gameplay.gnome").unwrap();
    let registration = scenario_regional_registration(
        &world.catalog,
        regional.clone(),
        SandmanorForm::Gnome,
        ConstitutionalRegion::AuraFields,
        RegionalStandingKind::Established,
    )
    .unwrap();
    let mut service = GameApplicationService::new(rules());
    service
        .execute(
            metadata("game-event.gnome", 2),
            GameplayCommand::RegisterRegionalBeing {
                continuity: continuity("being-continuity.gnome"),
                participant: participant("participant.gnome"),
                institutional: institutional("being.gnome"),
                regional_metadata: scenario_regional_metadata("gameplay.gnome", 2),
                registration: Box::new(registration),
            },
        )
        .unwrap();

    let identity = service
        .identity_for_regional(&regional)
        .expect("regional identity bridge");
    assert_eq!(identity.id().as_str(), "being-continuity.gnome");
    assert_eq!(identity.incarnation().regional_being(), Some(&regional));
    assert!(service.runtime().regional().being(&regional).is_some());
    assert_eq!(service.events().len(), 1);
    assert_eq!(service.runtime().regional().events().len(), 1);
}

#[test]
fn identity_collision_rolls_back_the_already_validated_regional_candidate() {
    let world = hollow_grove::world::institutional_access_fixture();
    let regional = RegionalBeingId::new("being.gameplay.rollback-gnome").unwrap();
    let registration = scenario_regional_registration(
        &world.catalog,
        regional.clone(),
        SandmanorForm::Gnome,
        ConstitutionalRegion::AuraFields,
        RegionalStandingKind::Established,
    )
    .unwrap();
    let mut service = GameApplicationService::new(rules());
    service
        .execute(metadata("game-event.rollback.hueman", 1), hueman_command())
        .unwrap();
    let before_events = service.events().to_vec();
    let before_revision = service.revision();

    let error = service
        .execute(
            metadata("game-event.rollback.gnome", 2),
            GameplayCommand::RegisterRegionalBeing {
                continuity: continuity("being-continuity.rollback-gnome"),
                participant: participant("participant.rollback-gnome"),
                institutional: institutional("being.hueman"),
                regional_metadata: scenario_regional_metadata("gameplay.rollback-gnome", 2),
                registration: Box::new(registration),
            },
        )
        .unwrap_err();
    assert!(matches!(
        error,
        GameplayRuntimeError::Identity(
            GameplayIdentityError::InstitutionalBeingAlreadyMapped { .. }
        )
    ));
    assert_eq!(service.events(), before_events);
    assert_eq!(service.revision(), before_revision);
    assert!(service.runtime().regional().events().is_empty());
    assert!(service.runtime().regional().being(&regional).is_none());
}

#[test]
fn child_rule_set_mismatch_fails_before_regional_or_identity_mutation() {
    let world = hollow_grove::world::institutional_access_fixture();
    let regional = RegionalBeingId::new("being.gameplay.wrong-rule-gnome").unwrap();
    let registration = scenario_regional_registration(
        &world.catalog,
        regional.clone(),
        SandmanorForm::Gnome,
        ConstitutionalRegion::AuraFields,
        RegionalStandingKind::Established,
    )
    .unwrap();
    let mut regional_metadata = scenario_regional_metadata("gameplay.wrong-rule-gnome", 2);
    regional_metadata.rule_set = RuleSetId::new("rules.wrong").unwrap();
    let mut service = GameApplicationService::new(rules());

    let error = service
        .execute(
            metadata("game-event.wrong-rule-gnome", 2),
            GameplayCommand::RegisterRegionalBeing {
                continuity: continuity("being-continuity.wrong-rule-gnome"),
                participant: participant("participant.wrong-rule-gnome"),
                institutional: institutional("being.wrong-rule-gnome"),
                regional_metadata,
                registration: Box::new(registration),
            },
        )
        .unwrap_err();
    assert!(matches!(
        error,
        GameplayRuntimeError::RuleSetMismatch { .. }
    ));
    assert!(service.events().is_empty());
    assert!(service.runtime().regional().events().is_empty());
    assert!(service.identity_for_regional(&regional).is_none());
}

#[test]
fn gameplay_history_replays_hueman_and_regional_identity_exactly() {
    let world = hollow_grove::world::institutional_access_fixture();
    let regional = RegionalBeingId::new("being.gameplay.replay-gnome").unwrap();
    let registration = scenario_regional_registration(
        &world.catalog,
        regional.clone(),
        SandmanorForm::Gnome,
        ConstitutionalRegion::AuraFields,
        RegionalStandingKind::Established,
    )
    .unwrap();
    let mut service = GameApplicationService::new(rules());
    service
        .execute(metadata("game-event.replay.hueman", 1), hueman_command())
        .unwrap();
    service
        .execute(
            metadata("game-event.replay.gnome", 2),
            GameplayCommand::RegisterRegionalBeing {
                continuity: continuity("being-continuity.replay-gnome"),
                participant: participant("participant.replay-gnome"),
                institutional: institutional("being.replay-gnome"),
                regional_metadata: scenario_regional_metadata("gameplay.replay-gnome", 2),
                registration: Box::new(registration),
            },
        )
        .unwrap();

    let expected_events = service.events().to_vec();
    let replayed = GameApplicationService::replay(rules(), expected_events.clone()).unwrap();
    assert_eq!(replayed.events(), expected_events);
    assert_eq!(replayed.revision(), service.revision());
    assert_eq!(replayed.hueman(), service.hueman());
    assert_eq!(
        replayed.identity_for_regional(&regional),
        service.identity_for_regional(&regional)
    );
    assert_eq!(
        replayed.runtime().regional().events(),
        service.runtime().regional().events()
    );
}

#[test]
fn recursion_kernels_do_not_import_the_gameplay_layer() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for relative in [
        "src/hollow_grove.rs",
        "src/kernel_pass.rs",
        "hollow-grove-kernel/src/lib.rs",
    ] {
        let source = std::fs::read_to_string(root.join(relative)).unwrap();
        assert!(!source.contains("crate::gameplay"), "{relative}");
        assert!(!source.contains("godot"), "{relative}");
    }
}
