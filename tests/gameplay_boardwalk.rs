use hollow_grove::constitutional::{
    BondId, BondPhase, BondTerm, CausalPosition, ConstitutionalRuntime, ParticipantId, RuleSetId,
    V2_RULE_SET,
};
use hollow_grove::gameplay::{
    BOARDWALK_GOON_BOND_ID, BOARDWALK_LIMITED_COOPERATION_BOND_ID, BOARDWALK_MAP_ROWS,
    BOARDWALK_PATRONAGE_BOND_ID, BOARDWALK_RETURN_CASE_ID, BoardwalkAuthorityClass, BoardwalkCase,
    BoardwalkCaseError, BoardwalkChoice, CardinalDirection, GameApplicationService,
    GameProtocolService, GameView, GameplayCommand, GameplayEventId, GameplayEventKind,
    GameplayEventMetadata, GameplayIntent, GameplayRuntimeError, HuemanFaculty, InteractionId,
    MAP_WIDTH, ProtocolRequestEnvelope, ProtocolResponseStatus, RETURNING_GOON_BEING_ID,
    RETURNING_GOON_PARTICIPANT_ID, WorldMapId,
};
use hollow_grove::institution::InstitutionalBeingId;
use hollow_grove::world::session::WorldSession;

fn rules() -> RuleSetId {
    RuleSetId::new(V2_RULE_SET).unwrap()
}

fn execute(service: &mut GameApplicationService, next: &mut u64, command: GameplayCommand) {
    let at = *next;
    service
        .execute(
            GameplayEventMetadata {
                id: GameplayEventId::new(format!("game-event.boardwalk.{at}")).unwrap(),
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

fn completed_goon_bond_application() -> GameApplicationService {
    let world =
        WorldSession::load_or_canonical_at(std::path::Path::new(env!("CARGO_MANIFEST_DIR")))
            .unwrap();
    let mut service = GameApplicationService::with_world_session(rules(), world);
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
            map: WorldMapId::BoardwalkReturnVestibule,
        },
    );

    // Discharge advocate at (3, 13), approached from (3, 14).
    walk(&mut service, &mut next, CardinalDirection::West, 6);
    walk(&mut service, &mut next, CardinalDirection::North, 1);
    face_north_and_interact(&mut service, &mut next);

    // Returning Goon at (9, 11), approached from (9, 12).
    walk(&mut service, &mut next, CardinalDirection::East, 6);
    walk(&mut service, &mut next, CardinalDirection::North, 2);
    face_north_and_interact(&mut service, &mut next);

    // Route around the Returning Goon to the faculty station.
    walk(&mut service, &mut next, CardinalDirection::West, 1);
    walk(&mut service, &mut next, CardinalDirection::North, 4);
    walk(&mut service, &mut next, CardinalDirection::East, 1);
    face_north_and_interact(&mut service, &mut next);

    // Pimp, Hoe, Goon, and Gimp testimony along the upper boardwalk.
    walk(&mut service, &mut next, CardinalDirection::West, 5);
    walk(&mut service, &mut next, CardinalDirection::North, 3);
    face_north_and_interact(&mut service, &mut next);
    walk(&mut service, &mut next, CardinalDirection::East, 3);
    face_north_and_interact(&mut service, &mut next);
    walk(&mut service, &mut next, CardinalDirection::East, 4);
    face_north_and_interact(&mut service, &mut next);
    walk(&mut service, &mut next, CardinalDirection::East, 3);
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
        GameplayCommand::SupportBoardwalkOption {
            choice: BoardwalkChoice::GoonBond,
        },
    );
    execute(
        &mut service,
        &mut next,
        GameplayCommand::AskReturningGoonToDecide,
    );
    service
}

#[test]
fn returning_goon_commits_a_finite_active_constitutional_bond() {
    let service = completed_goon_bond_application();
    let case = service.runtime().boardwalk_case().unwrap();
    assert_eq!(case.id().as_str(), BOARDWALK_RETURN_CASE_ID);
    assert_eq!(case.returning_goon().as_str(), RETURNING_GOON_BEING_ID);
    assert_eq!(case.committed_choice(), Some(BoardwalkChoice::GoonBond));
    assert!(case.is_ready());
    let commit = case.goon_bond().unwrap();
    assert_eq!(commit.bond.as_str(), BOARDWALK_GOON_BOND_ID);
    assert!(commit.term_end.get() > service.revision().get());

    let bond = service
        .runtime()
        .constitutional()
        .bond(&commit.bond)
        .unwrap();
    assert_eq!(bond.phase(), BondPhase::Active);
    assert!(matches!(bond.formation().term, BondTerm::Finite { .. }));
    assert!(
        bond.formation()
            .obligations
            .iter()
            .any(|obligation| obligation.as_str() == "obligation.boardwalk.no-ownership")
    );
    assert!(
        bond.formation()
            .permissions
            .iter()
            .any(|permission| permission.as_str() == "permission.boardwalk.leave")
    );
    assert_eq!(service.runtime().constitutional().events().len(), 3);
    assert_eq!(
        service.runtime().constitutional().events()[0].bond.as_str(),
        BOARDWALK_GOON_BOND_ID
    );

    let final_event = service.events().last().unwrap();
    let hollow_grove::gameplay::GameplayEvent::ReturningGoonChoiceCommitted {
        committed_by,
        choice,
        outcome_id,
        relationship_bond,
    } = &final_event.payload
    else {
        panic!("Returning Goon choice event");
    };
    assert_eq!(committed_by.as_str(), RETURNING_GOON_PARTICIPANT_ID);
    assert_eq!(*choice, BoardwalkChoice::GoonBond);
    assert_eq!(outcome_id.as_str(), "outcome.boardwalk.goon-bond.v1");
    assert_eq!(
        relationship_bond.as_ref().map(BondId::as_str),
        Some(BOARDWALK_GOON_BOND_ID)
    );
}

#[test]
fn case_entry_requires_hueman_and_evidence_but_independent_return_needs_no_bond() {
    let mut application = GameApplicationService::new(rules());
    let error = application
        .execute(
            GameplayEventMetadata {
                id: GameplayEventId::new("game-event.boardwalk.no-hueman").unwrap(),
                causal_position: CausalPosition::new(1),
            },
            GameplayCommand::EnterMap {
                map: WorldMapId::BoardwalkReturnVestibule,
            },
        )
        .unwrap_err();
    assert_eq!(error, GameplayRuntimeError::MapEntryRequiresHueman);

    let mut case = BoardwalkCase::new();
    assert_eq!(
        case.support(BoardwalkChoice::IndependentReturn),
        Err(BoardwalkCaseError::CaseNotReady)
    );
    for interaction in [
        InteractionId::BoardwalkDischargeAdvocate,
        InteractionId::BoardwalkPimp,
        InteractionId::BoardwalkHoeWitness,
        InteractionId::BoardwalkGimp,
        InteractionId::BoardwalkGoonWitness,
        InteractionId::BoardwalkReturningGoon,
    ] {
        case.observe_interaction(interaction);
    }
    for faculty in HuemanFaculty::ALL {
        case.disclose_faculty(faculty).unwrap();
    }
    case.support(BoardwalkChoice::IndependentReturn).unwrap();
    let mut constitutional = ConstitutionalRuntime::new();
    let world =
        WorldSession::load_or_canonical_at(std::path::Path::new(env!("CARGO_MANIFEST_DIR")))
            .unwrap();
    assert_eq!(
        case.commit_returning_goon_choice_with_authority(
            CausalPosition::new(1),
            &rules(),
            &mut constitutional,
            &world,
        )
        .unwrap(),
        BoardwalkChoice::IndependentReturn
    );
    assert!(constitutional.events().is_empty());
    assert!(case.goon_bond().is_none());
    assert!(case.outcome().unwrap().relationship.is_none());
}

#[test]
fn every_boardwalk_choice_has_a_typed_lawful_outcome_and_exact_refusal_path() {
    let world =
        WorldSession::load_or_canonical_at(std::path::Path::new(env!("CARGO_MANIFEST_DIR")))
            .unwrap();
    for (choice, authority_class, expected_bond) in [
        (
            BoardwalkChoice::PimpPatronage,
            BoardwalkAuthorityClass::VoluntaryPatronage,
            Some(BOARDWALK_PATRONAGE_BOND_ID),
        ),
        (
            BoardwalkChoice::GoonBond,
            BoardwalkAuthorityClass::FiniteGoonBond,
            Some(BOARDWALK_GOON_BOND_ID),
        ),
        (
            BoardwalkChoice::LimitedCooperation,
            BoardwalkAuthorityClass::ScopedCooperation,
            Some(BOARDWALK_LIMITED_COOPERATION_BOND_ID),
        ),
        (
            BoardwalkChoice::IndependentReturn,
            BoardwalkAuthorityClass::CivicSelfDirection,
            None,
        ),
    ] {
        let mut case = BoardwalkCase::new();
        for interaction in [
            InteractionId::BoardwalkDischargeAdvocate,
            InteractionId::BoardwalkPimp,
            InteractionId::BoardwalkHoeWitness,
            InteractionId::BoardwalkGimp,
            InteractionId::BoardwalkGoonWitness,
            InteractionId::BoardwalkReturningGoon,
        ] {
            case.observe_interaction(interaction);
        }
        for faculty in HuemanFaculty::ALL {
            case.disclose_faculty(faculty).unwrap();
        }
        case.support(choice).unwrap();
        let mut constitutional = ConstitutionalRuntime::new();
        case.commit_returning_goon_choice_with_authority(
            CausalPosition::new(20),
            &rules(),
            &mut constitutional,
            &world,
        )
        .unwrap();

        let outcome = case.outcome().unwrap();
        assert_eq!(outcome.choice, choice);
        assert_eq!(outcome.authority_class, authority_class);
        assert_eq!(outcome.evidence.len(), 6);
        assert!(outcome.player_support_is_nonbinding);
        assert!(!outcome.faculty_uncertainties.is_empty());
        assert!(!outcome.failure_and_refusal.is_empty());
        assert_eq!(
            outcome.glaushouse_discharge_clearance.function,
            hollow_grove::constitutional::HouseFunction::Clear
        );
        assert_eq!(
            outcome.flynt_recognition.function,
            hollow_grove::constitutional::HouseFunction::Recognize
        );
        assert!(
            !outcome
                .glaushouse_discharge_clearance
                .authority
                .actor
                .as_str()
                .contains("fixture")
        );

        assert_eq!(
            outcome
                .relationship
                .as_ref()
                .map(|relationship| relationship.bond.as_str()),
            expected_bond
        );
        if let Some(relationship) = &outcome.relationship {
            let bond = constitutional.bond(&relationship.bond).unwrap();
            assert_eq!(bond.phase(), BondPhase::Active);
            assert!(
                bond.formation()
                    .obligations
                    .iter()
                    .any(|obligation| obligation.as_str().contains("no-ownership"))
            );
            assert_eq!(constitutional.events().len(), 3);
        } else {
            assert!(constitutional.events().is_empty());
            assert!(outcome.lawful_state_change.contains("refuses"));
        }
    }
}

#[test]
fn boardwalk_resolution_fails_closed_without_live_house_incumbents() {
    let mut case = BoardwalkCase::new();
    for interaction in [
        InteractionId::BoardwalkDischargeAdvocate,
        InteractionId::BoardwalkPimp,
        InteractionId::BoardwalkHoeWitness,
        InteractionId::BoardwalkGimp,
        InteractionId::BoardwalkGoonWitness,
        InteractionId::BoardwalkReturningGoon,
    ] {
        case.observe_interaction(interaction);
    }
    for faculty in HuemanFaculty::ALL {
        case.disclose_faculty(faculty).unwrap();
    }
    case.support(BoardwalkChoice::GoonBond).unwrap();
    let mut constitutional = ConstitutionalRuntime::new();
    let error = case
        .commit_returning_goon_choice(CausalPosition::new(1), &rules(), &mut constitutional)
        .unwrap_err();
    assert!(matches!(error, BoardwalkCaseError::Constitutional(_)));
    assert!(case.outcome().is_none());
    assert!(case.committed_choice().is_none());
    assert!(constitutional.events().is_empty());
}

#[test]
fn boardwalk_layout_and_godot_controls_expose_the_authored_slice_only() {
    assert!(
        BOARDWALK_MAP_ROWS
            .iter()
            .all(|row| row.len() == usize::from(MAP_WIDTH))
    );
    let social_row = BOARDWALK_MAP_ROWS[4].as_bytes();
    assert_eq!(
        (
            social_row.iter().position(|tile| *tile == b'P').unwrap(),
            social_row.iter().position(|tile| *tile == b'O').unwrap(),
            social_row.iter().position(|tile| *tile == b'N').unwrap(),
            social_row.iter().position(|tile| *tile == b'G').unwrap(),
        ),
        (4, 7, 11, 14)
    );

    let godot = include_str!("../hueman_godot/scripts/retro_overworld.gd");
    for required in [
        "EnterRegionIntent",
        "DiscloseFacultyObservationIntent",
        "SupportBoardwalkOptionIntent",
        "AskReturningGoonToDecideIntent",
        "SaveIntent",
        "LoadIntent",
        "boardwalk_case",
    ] {
        assert!(
            godot.contains(required),
            "missing Godot control: {required}"
        );
    }
    for forbidden in [
        "BondFormation",
        "HouseDecision",
        "ConstitutionalRuntime",
        "commit_returning_goon_choice",
    ] {
        assert!(
            !godot.contains(forbidden),
            "Godot crossed authority boundary with {forbidden}"
        );
    }
}

#[test]
fn boardwalk_history_and_active_bond_replay_from_checksummed_archive() {
    let service = completed_goon_bond_application();
    let archive = service.encode_archive().unwrap();
    assert!(archive.contains("\"checksum\": \"fnv1a64:"));
    assert!(archive.contains("\"schema_version\": 3"));
    assert!(archive.contains("\"federation_binding\""));
    assert!(archive.contains("\"institutional_state\""));
    assert!(archive.contains("being.stonebend.current-hypergiant"));
    assert!(!archive.contains("being.stonebend.fixture-member"));

    let replayed = GameApplicationService::from_archive(&archive).unwrap();
    assert_eq!(replayed.events(), service.events());
    assert_eq!(replayed.revision(), service.revision());
    assert_eq!(
        replayed
            .runtime()
            .boardwalk_case()
            .unwrap()
            .committed_choice(),
        Some(BoardwalkChoice::GoonBond)
    );
    let bond = BondId::new(BOARDWALK_GOON_BOND_ID).unwrap();
    assert_eq!(
        replayed
            .runtime()
            .constitutional()
            .bond(&bond)
            .unwrap()
            .phase(),
        BondPhase::Active
    );

    let mut damaged: serde_json::Value = serde_json::from_str(&archive).unwrap();
    damaged["payload"]["events"][0]["causal_position"] = serde_json::json!(99);
    let damaged = serde_json::to_string(&damaged).unwrap();
    assert!(GameApplicationService::from_archive(&damaged).is_err());
}

#[test]
fn resolved_goon_bond_changes_collision_placement_and_revisit_dialogue() {
    let mut service = completed_goon_bond_application();
    let mut next = service.revision().get() + 1;
    walk(&mut service, &mut next, CardinalDirection::South, 1);
    walk(&mut service, &mut next, CardinalDirection::West, 2);
    face_north_and_interact(&mut service, &mut next);

    let view = GameView::from_runtime(service.runtime(), vec![]);
    let overworld = view.overworld.unwrap();
    assert_eq!(overworld.tile_rows[5].as_bytes()[12], b'R');
    assert_eq!(overworld.tile_rows[5].as_bytes()[13], b'N');
    assert_eq!((overworld.player.x, overworld.player.y), (12, 6));
    let dialogue = view.interaction.unwrap();
    assert_eq!(dialogue.target_id, "interaction.boardwalk.returning-goon");
    assert!(
        dialogue
            .pages
            .iter()
            .any(|page| page.contains("FINITE GOON BOND"))
    );
}

fn protocol_request(
    request_id: &str,
    revision: u64,
    intent: GameplayIntent,
) -> ProtocolRequestEnvelope {
    ProtocolRequestEnvelope {
        protocol_version: 1,
        session_id: "session.boardwalk-save".into(),
        request_id: request_id.into(),
        expected_revision: revision,
        intent,
    }
}

#[test]
fn protocol_save_survives_service_restart_and_loads_the_boardwalk_map() {
    let save_root = std::env::temp_dir().join(format!(
        "hollow-grove-boardwalk-save-{}",
        std::process::id()
    ));
    let mut first =
        GameProtocolService::new_with_save_root("session.boardwalk-save", rules(), &save_root)
            .unwrap();
    assert_eq!(
        first
            .handle(protocol_request(
                "request.boardwalk-save.establish",
                0,
                GameplayIntent::EstablishHuemanIntent {
                    continuity_id: "being-continuity.hueman".into(),
                    participant_id: "participant.hueman".into(),
                    institutional_being_id: "being.hueman".into(),
                },
            ))
            .status,
        ProtocolResponseStatus::Completed
    );
    let entered = first.handle(protocol_request(
        "request.boardwalk-save.enter",
        1,
        GameplayIntent::EnterRegionIntent {
            region_id: "boardwalk.return-vestibule".into(),
        },
    ));
    assert_eq!(entered.status, ProtocolResponseStatus::Completed);
    let saved = first.handle(protocol_request(
        "request.boardwalk-save.write",
        2,
        GameplayIntent::SaveIntent {
            slot_id: "slot-a".into(),
        },
    ));
    assert_eq!(saved.status, ProtocolResponseStatus::Completed);
    assert_eq!(saved.events[0].kind, GameplayEventKind::SnapshotSaved);

    let mut restarted =
        GameProtocolService::new_with_save_root("session.boardwalk-save", rules(), &save_root)
            .unwrap();
    let loaded = restarted.handle(protocol_request(
        "request.boardwalk-save.read",
        0,
        GameplayIntent::LoadIntent {
            slot_id: "slot-a".into(),
        },
    ));
    assert_eq!(loaded.status, ProtocolResponseStatus::Completed);
    assert_eq!(loaded.revision, 2);
    assert_eq!(
        loaded.view.unwrap().overworld.unwrap().map_id,
        "boardwalk.return-vestibule"
    );
}
