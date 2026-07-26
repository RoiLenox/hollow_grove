use hollow_grove::constitutional::{
    CausalPosition, HouseFunction, ParticipantId, RuleSetId, V2_RULE_SET,
};
use hollow_grove::gameplay::{
    CURRENT_SEA_CONTINUITY_CASE_ID, CURRENT_SEA_MAP_ROWS, CardinalDirection,
    GameApplicationService, GameProtocolService, GameView, GameplayCommand, GameplayEvent,
    GameplayEventId, GameplayEventMetadata, GameplayIntent, HuemanFaculty, InteractionId,
    MAP_WIDTH, MERCY_DEEP_BEING_ID, MERCY_DEEP_EXISTING_NAME, MERCY_DEEP_PARTICIPANT_ID,
    MERCY_DEEP_TRANSFORMED_NAME, ProtocolRequestEnvelope, ProtocolResponseStatus,
    StonebendAuthorityClass, StonebendCaseError, StonebendContinuityCase,
    StonebendContinuityChoice, WorldMapId, map_definition,
};
use hollow_grove::institution::InstitutionalBeingId;
use hollow_grove::world::session::WorldSession;
use hollow_grove::world::stonebend::{NameStatus, PrincipalAuthority};

fn rules() -> RuleSetId {
    RuleSetId::new(V2_RULE_SET).unwrap()
}

fn live_world() -> WorldSession {
    WorldSession::load_or_canonical_at(std::path::Path::new(env!("CARGO_MANIFEST_DIR"))).unwrap()
}

fn ready_case() -> StonebendContinuityCase {
    let mut case = StonebendContinuityCase::new();
    for interaction in [
        InteractionId::CurrentSeaMercyDeep,
        InteractionId::CurrentSeaNameLedger,
        InteractionId::CurrentSeaRestorationArchive,
        InteractionId::CurrentSeaDepthWitness,
        InteractionId::CurrentSeaMercuryMirror,
    ] {
        assert!(case.observe_interaction(interaction).is_some());
    }
    for faculty in HuemanFaculty::ALL {
        case.disclose_faculty(faculty).unwrap();
    }
    assert!(case.is_ready());
    case
}

#[test]
fn all_current_sea_choices_create_exact_typed_stonebend_records_without_title() {
    for (choice, authority_class) in [
        (
            StonebendContinuityChoice::AffirmExistingName,
            StonebendAuthorityClass::ConstitutionalIdentity,
        ),
        (
            StonebendContinuityChoice::ProvisionalTransformedFormName,
            StonebendAuthorityClass::ProvisionalIdentityContinuity,
        ),
        (
            StonebendContinuityChoice::ReferIdentityConflict,
            StonebendAuthorityClass::HighIdentityReview,
        ),
    ] {
        let mut case = ready_case();
        case.support(choice).unwrap();
        case.commit_with_authority(CausalPosition::new(20), &live_world())
            .unwrap();

        let outcome = case.outcome().unwrap();
        assert_eq!(case.id().as_str(), CURRENT_SEA_CONTINUITY_CASE_ID);
        assert_eq!(case.subject().as_str(), MERCY_DEEP_BEING_ID);
        assert_eq!(outcome.authority_class, authority_class);
        assert_eq!(outcome.evidence.len(), 5);
        assert_eq!(outcome.stonebend_naming.function, HouseFunction::Name);
        assert_eq!(
            outcome.determination.decision.authority,
            PrincipalAuthority::Hypergiant
        );
        assert_eq!(
            outcome.determination.seal.issuing_authority,
            PrincipalAuthority::HighFreemason
        );
        assert!(!outcome.title_granted);
        assert!(outcome.player_support_is_nonbinding);
        assert!(
            !outcome
                .stonebend_naming
                .authority
                .actor
                .as_str()
                .contains("fixture")
        );
        outcome
            .determination
            .validate(choice, &outcome.stonebend_naming)
            .unwrap();

        match choice {
            StonebendContinuityChoice::AffirmExistingName => {
                assert_eq!(outcome.determination.name_records.len(), 1);
                assert_eq!(
                    outcome.determination.name_records[0].name,
                    MERCY_DEEP_EXISTING_NAME
                );
                assert_eq!(
                    outcome.determination.name_records[0].status,
                    NameStatus::Active
                );
            }
            StonebendContinuityChoice::ProvisionalTransformedFormName => {
                assert_eq!(outcome.determination.name_records.len(), 2);
                assert_eq!(
                    outcome.determination.name_records[1].name,
                    MERCY_DEEP_TRANSFORMED_NAME
                );
                assert_eq!(
                    outcome.determination.name_records[1].status,
                    NameStatus::Provisional
                );
                assert_eq!(
                    outcome.determination.name_records[1].former_names,
                    vec![outcome.determination.name_records[0].id.clone()]
                );
            }
            StonebendContinuityChoice::ReferIdentityConflict => {
                assert!(outcome.determination.name_records.is_empty());
                assert!(outcome.lawful_state_change.contains("no final Name"));
            }
        }
    }
}

#[test]
fn continuity_case_fails_closed_and_preserves_subject_challenge() {
    let mut case = StonebendContinuityCase::new();
    assert_eq!(
        case.support(StonebendContinuityChoice::ReferIdentityConflict),
        Err(StonebendCaseError::CaseNotReady)
    );

    let mut case = ready_case();
    case.support(StonebendContinuityChoice::ReferIdentityConflict)
        .unwrap();
    let error = case
        .commit_with_authority(CausalPosition::new(1), &WorldSession::canonical())
        .unwrap_err();
    assert!(matches!(error, StonebendCaseError::Constitutional(_)));
    assert!(case.outcome().is_none());
    assert!(case.committed_choice().is_none());
}

fn execute(service: &mut GameApplicationService, next: &mut u64, command: GameplayCommand) {
    let at = *next;
    service
        .execute(
            GameplayEventMetadata {
                id: GameplayEventId::new(format!("game-event.stonebend.{at}")).unwrap(),
                causal_position: CausalPosition::new(at),
            },
            command,
        )
        .unwrap();
    *next += 1;
}

fn walk(
    service: &mut GameApplicationService,
    next: &mut u64,
    direction: CardinalDirection,
    steps: usize,
) {
    for _ in 0..steps {
        execute(service, next, GameplayCommand::MoveHueman { direction });
    }
}

fn face_north_and_interact(service: &mut GameApplicationService, next: &mut u64) {
    execute(
        service,
        next,
        GameplayCommand::MoveHueman {
            direction: CardinalDirection::North,
        },
    );
    execute(service, next, GameplayCommand::InteractHueman);
}

fn completed_stonebend_application() -> GameApplicationService {
    let mut service = GameApplicationService::with_world_session(rules(), live_world());
    let mut next = 1;
    execute(
        &mut service,
        &mut next,
        GameplayCommand::EstablishHuemanIdentity {
            continuity: hollow_grove::gameplay::BeingContinuityId::new("being-continuity.hueman")
                .unwrap(),
            participant: ParticipantId::new("participant.hueman").unwrap(),
            institutional: InstitutionalBeingId::new("being.hueman").unwrap(),
        },
    );
    execute(
        &mut service,
        &mut next,
        GameplayCommand::EnterMap {
            map: WorldMapId::CurrentSeaDeepCertificationLanding,
        },
    );

    // Restoration archive at (4, 13), approached from (4, 14).
    walk(&mut service, &mut next, CardinalDirection::West, 5);
    walk(&mut service, &mut next, CardinalDirection::North, 1);
    face_north_and_interact(&mut service, &mut next);

    // Name ledger at (4, 9), approached from (4, 10).
    walk(&mut service, &mut next, CardinalDirection::East, 1);
    walk(&mut service, &mut next, CardinalDirection::North, 4);
    walk(&mut service, &mut next, CardinalDirection::West, 1);
    face_north_and_interact(&mut service, &mut next);

    // Mercury Mirror at (14, 9), approached from (14, 10).
    walk(&mut service, &mut next, CardinalDirection::East, 10);
    face_north_and_interact(&mut service, &mut next);

    // Depth witness at (13, 4), approached from (13, 5).
    walk(&mut service, &mut next, CardinalDirection::West, 1);
    walk(&mut service, &mut next, CardinalDirection::North, 5);
    face_north_and_interact(&mut service, &mut next);

    // Mercy Deep at (8, 4), approached from (8, 5).
    walk(&mut service, &mut next, CardinalDirection::West, 5);
    face_north_and_interact(&mut service, &mut next);

    for faculty in HuemanFaculty::ALL {
        execute(
            &mut service,
            &mut next,
            GameplayCommand::DiscloseFacultyObservation { faculty },
        );
    }
    execute(
        &mut service,
        &mut next,
        GameplayCommand::SupportStonebendContinuityOption {
            choice: StonebendContinuityChoice::ProvisionalTransformedFormName,
        },
    );
    execute(
        &mut service,
        &mut next,
        GameplayCommand::AskStonebendToDetermineContinuity,
    );
    service
}

#[test]
fn stonebend_map_case_and_authority_replay_from_checksummed_archive() {
    let service = completed_stonebend_application();
    let case = service.runtime().stonebend_case().unwrap();
    assert_eq!(
        case.committed_choice(),
        Some(StonebendContinuityChoice::ProvisionalTransformedFormName)
    );
    let GameplayEvent::StonebendContinuityDeterminationCommitted {
        subject,
        authority_actor,
        ..
    } = &service.events().last().unwrap().payload
    else {
        panic!("Stonebend determination event");
    };
    assert_eq!(subject.as_str(), MERCY_DEEP_PARTICIPANT_ID);
    assert!(!authority_actor.as_str().contains("fixture"));

    let view = GameView::from_runtime(service.runtime(), vec![]);
    let overworld = view.overworld.unwrap();
    assert_eq!(overworld.map_id, "current-sea.deep-certification-landing");
    assert_eq!(overworld.tile_rows[7].as_bytes()[8], b'=');
    assert_eq!(overworld.tile_rows[7].as_bytes()[7], b'x');
    let dialogue = view.interaction.unwrap();
    assert_eq!(dialogue.speaker, "MERCY DEEP");
    assert!(dialogue.pages.iter().any(|page| page.contains("AFTERTIDE")));
    let case_view = view.stonebend_case.unwrap();
    assert!(!case_view.outcome.unwrap().title_granted);

    let archive = service.encode_archive().unwrap();
    assert!(archive.contains("\"schema_version\": 2"));
    assert!(archive.contains("StonebendContinuityDeterminationCommitted"));
    assert!(archive.contains("being.stonebend.current-hypergiant"));
    let replayed = GameApplicationService::from_archive(&archive).unwrap();
    assert_eq!(replayed.events(), service.events());
    assert_eq!(
        replayed
            .runtime()
            .stonebend_case()
            .unwrap()
            .committed_choice(),
        case.committed_choice()
    );
}

#[test]
fn current_sea_layout_and_client_expose_stonebend_without_owning_law() {
    assert!(
        CURRENT_SEA_MAP_ROWS
            .iter()
            .all(|row| row.len() == usize::from(MAP_WIDTH))
    );
    assert_eq!(
        WorldMapId::from_wire("current-sea.deep-certification-landing").unwrap(),
        WorldMapId::CurrentSeaDeepCertificationLanding
    );
    let map = map_definition(WorldMapId::CurrentSeaDeepCertificationLanding);
    let projected =
        map.projected_rows_with_cases(None, Some(StonebendContinuityChoice::ReferIdentityConflict));
    assert_eq!(projected[7].as_bytes()[9], b'=');

    let godot = include_str!("../hueman_godot/scripts/retro_overworld.gd");
    for required in [
        "current-sea.deep-certification-landing",
        "SupportStonebendContinuityOptionIntent",
        "AskStonebendToDetermineContinuityIntent",
        "stonebend_case",
    ] {
        assert!(
            godot.contains(required),
            "missing Stonebend client projection: {required}"
        );
    }
    for forbidden in [
        "scenario_house_decision",
        "DecisionRecord",
        "SealRecord",
        "PrincipalAuthority",
    ] {
        assert!(
            !godot.contains(forbidden),
            "Godot crossed Stonebend authority boundary with {forbidden}"
        );
    }
}

fn protocol_request(
    request_id: &str,
    revision: u64,
    intent: GameplayIntent,
) -> ProtocolRequestEnvelope {
    ProtocolRequestEnvelope {
        protocol_version: 1,
        session_id: "session.stonebend".into(),
        request_id: request_id.into(),
        expected_revision: revision,
        intent,
    }
}

#[test]
fn protocol_projects_the_current_sea_case_and_rejects_premature_support() {
    let mut service = GameProtocolService::new("session.stonebend", rules()).unwrap();
    let established = service.handle(protocol_request(
        "request.stonebend.establish",
        0,
        GameplayIntent::EstablishHuemanIntent {
            continuity_id: "being-continuity.hueman".into(),
            participant_id: "participant.hueman".into(),
            institutional_being_id: "being.hueman".into(),
        },
    ));
    assert_eq!(established.status, ProtocolResponseStatus::Completed);

    let entered = service.handle(protocol_request(
        "request.stonebend.enter",
        1,
        GameplayIntent::EnterRegionIntent {
            region_id: "current-sea.deep-certification-landing".into(),
        },
    ));
    assert_eq!(entered.status, ProtocolResponseStatus::Completed);
    let view = entered.view.unwrap();
    assert!(view.stonebend_case.is_some());
    assert!(view.capabilities.iter().any(|capability| {
        capability.intent_type == "SupportStonebendContinuityOptionIntent" && !capability.available
    }));

    let rejected = service.handle(protocol_request(
        "request.stonebend.premature-support",
        2,
        GameplayIntent::SupportStonebendContinuityOptionIntent {
            choice: StonebendContinuityChoice::AffirmExistingName,
        },
    ));
    assert_eq!(rejected.status, ProtocolResponseStatus::Rejected);
    assert_eq!(rejected.revision, 2);
}
