use std::collections::{BTreeSet, VecDeque};

use hollow_grove::constitutional::{CausalPosition, ParticipantId, RuleSetId, V2_RULE_SET};
use hollow_grove::gameplay::*;
use hollow_grove::institution::InstitutionalBeingId;
use hollow_grove::kernel_invariants::audit_boardwalk_territory;
use hollow_grove::world::geography::ConstitutionalRouteId;

fn execute(service: &mut GameApplicationService, command: GameplayCommand) {
    let next = service.runtime().revision().get() + 1;
    service
        .execute(
            GameplayEventMetadata {
                id: GameplayEventId::new(format!("game-event.boardwalk-territory.{next}")).unwrap(),
                causal_position: CausalPosition::new(next),
            },
            command,
        )
        .unwrap();
}

fn application() -> GameApplicationService {
    let mut service = GameApplicationService::new(RuleSetId::new(V2_RULE_SET).unwrap());
    execute(
        &mut service,
        GameplayCommand::EstablishHuemanIdentity {
            continuity: BeingContinuityId::new("being-continuity.hueman").unwrap(),
            participant: ParticipantId::new("participant.hueman").unwrap(),
            institutional: InstitutionalBeingId::new("institutional-being.hueman").unwrap(),
        },
    );
    execute(
        &mut service,
        GameplayCommand::EnterMap {
            map: WorldMapId::BoardwalkReturnVestibule,
        },
    );
    service
}

fn view(service: &GameApplicationService) -> GameView {
    GameView::from_runtime(service.runtime(), vec![])
}

fn walk_to(service: &mut GameApplicationService, x: u16, y: u16) {
    let map = map_definition(service.runtime().hueman_map());
    let start = service.runtime().hueman_position().unwrap();
    let mut pending = VecDeque::from([(start, vec![])]);
    let mut visited = BTreeSet::new();
    let path = loop {
        let (position, steps) = pending.pop_front().expect("destination must be accessible");
        if (position.x, position.y) == (x, y) {
            break steps;
        }
        if !visited.insert((position.x, position.y)) {
            continue;
        }
        for direction in [
            CardinalDirection::North,
            CardinalDirection::East,
            CardinalDirection::South,
            CardinalDirection::West,
        ] {
            let next = map.move_actor(position, direction, None);
            if (position.x, position.y) != (next.x, next.y) {
                let mut path = steps.clone();
                path.push(direction);
                pending.push_back((next, path));
            }
        }
    };
    for direction in path {
        execute(service, GameplayCommand::MoveHueman { direction });
    }
    let position = service.runtime().hueman_position().unwrap();
    assert_eq!((position.x, position.y), (x, y));
}

#[test]
fn influence_and_occupation_can_change_without_sovereignty_or_conquest() {
    let clock = LivingClock {
        day: 1,
        shift: WorkShift::Dawn,
    };
    let factions = boardwalk_factions();
    assert_eq!(factions.len(), 2);
    for faction in &factions {
        assert_eq!(faction.faction_id.rival(), faction.rival_faction);
        assert!(faction.sovereign_house_affiliation.is_none());
        assert!(faction.syndicate_organization.is_none());
    }
    for level in [BoardwalkLevel::Upper, BoardwalkLevel::Lower] {
        let mut projections = vec![];
        for influence in [
            BoardwalkInfluence {
                trooners: 90,
                gooners: 10,
            },
            BoardwalkInfluence {
                trooners: 10,
                gooners: 90,
            },
            BoardwalkInfluence {
                trooners: 0,
                gooners: 0,
            },
        ] {
            let state = BoardwalkTerritoryView::with_influence(level, clock, influence);
            assert!(audit_boardwalk_territory(Some(&state)).passed());
            assert_eq!(state.dominant_faction, influence.dominant());
            assert!(state.formal_neutrality && state.public_access && state.contested);
            assert!(state.sovereign_faction.is_none());
            projections.push(state);
        }
        assert_eq!(projections[0].local_presence, projections[1].local_presence);
        let mut corrupted = projections[0].clone();
        corrupted.sovereign_faction = Some(BoardwalkFactionId::Trooners);
        assert!(!audit_boardwalk_territory(Some(&corrupted)).passed());
    }
}

#[test]
fn both_spans_have_accessible_stairs_signs_and_ambient_people_in_every_shift() {
    for level in [BoardwalkLevel::Upper, BoardwalkLevel::Lower] {
        let mut service = application();
        let map_id = WorldMapId::BoardwalkSpan(level);
        assert!(map_id.is_canonical());
        assert_eq!(WorldMapId::from_wire(map_id.as_str()).unwrap(), map_id);
        assert_eq!(map_id.route(), Some(ConstitutionalRouteId::Boardwalk));
        let map = map_definition(map_id);
        assert_eq!(map.rows.len(), usize::from(MAP_HEIGHT));
        assert!(
            map.rows
                .iter()
                .all(|row| row.len() == usize::from(MAP_WIDTH))
        );
        execute(
            &mut service,
            GameplayCommand::TraverseMapExit { map: map_id },
        );
        let original_influence = view(&service).boardwalk_territory.unwrap().influence;
        for _ in WorkShift::ALL {
            let projected = view(&service);
            let territory = projected.boardwalk_territory.unwrap();
            assert_eq!(territory.dominant_faction, Some(level.home_faction()));
            for exit in &projected.physical_exits {
                walk_to(&mut service, exit.exit_x, exit.exit_y);
            }
            for person in territory.local_presence {
                walk_to(&mut service, person.position.x, person.position.y + 1);
                execute(
                    &mut service,
                    GameplayCommand::MoveHueman {
                        direction: CardinalDirection::North,
                    },
                );
                execute(&mut service, GameplayCommand::InteractHueman);
                assert_eq!(
                    service.runtime().active_interaction(),
                    Some(InteractionId::BoardwalkTerritoryPerson {
                        level,
                        role: person.role
                    })
                );
                let speech = view(&service).interaction.unwrap().pages.join(" ");
                assert!(speech.contains("PUBLIC INFRASTRUCTURE"));
                assert!(speech.contains("WITHOUT JOINING"));
            }
            walk_to(&mut service, 11, 15);
            execute(
                &mut service,
                GameplayCommand::MoveHueman {
                    direction: CardinalDirection::North,
                },
            );
            execute(&mut service, GameplayCommand::InteractHueman);
            assert_eq!(
                service.runtime().active_interaction(),
                Some(InteractionId::BoardwalkPassageNotice(level))
            );
            execute(&mut service, GameplayCommand::AdvanceLivingWorldShift);
        }
        // The schedule cycles rather than ending in conquest.
        assert_eq!(
            view(&service).boardwalk_territory.unwrap().influence,
            original_influence
        );
        assert!(
            service
                .runtime()
                .boardwalk_case()
                .unwrap()
                .evidence()
                .is_empty()
        );
        assert!(
            service
                .runtime()
                .boardwalk_case()
                .unwrap()
                .committed_choice()
                .is_none()
        );
    }
}

#[test]
fn public_traversal_and_save_replay_preserve_network_and_institutional_state() {
    let mut service = application();
    let institutional_before = service.runtime().world().persisted_state_output();
    let vestibule_before = service.encode_archive().unwrap();
    let legacy_loaded = GameApplicationService::from_archive(&vestibule_before).unwrap();
    assert_eq!(view(&legacy_loaded), view(&service));
    let upper = WorldMapId::BoardwalkSpan(BoardwalkLevel::Upper);
    let lower = WorldMapId::BoardwalkSpan(BoardwalkLevel::Lower);
    execute(
        &mut service,
        GameplayCommand::TraverseMapExit { map: upper },
    );
    // No direct span-to-House-route shortcuts were introduced.
    assert_eq!(
        service.runtime().physical_exit_destinations(),
        vec![WorldMapId::BoardwalkReturnVestibule, lower]
    );
    export_view("upper", &service);
    walk_to(&mut service, 9, 2);
    execute(
        &mut service,
        GameplayCommand::TraverseMapExit { map: lower },
    );
    assert_eq!(
        service.runtime().hueman_position().unwrap(),
        map_entry_position(upper, lower)
    );
    execute(&mut service, GameplayCommand::AdvanceLivingWorldShift);
    export_view("lower", &service);
    let saved = service.encode_archive().unwrap();
    let loaded = GameApplicationService::from_archive(&saved).unwrap();
    assert_eq!(view(&service), view(&loaded));
    // At the far stair, the vestibule ramp cannot be used remotely.
    let next = service.runtime().revision().get() + 1;
    assert!(
        service
            .execute(
                GameplayEventMetadata {
                    id: GameplayEventId::new("game-event.boardwalk-territory.bad-exit").unwrap(),
                    causal_position: CausalPosition::new(next),
                },
                GameplayCommand::TraverseMapExit {
                    map: WorldMapId::BoardwalkReturnVestibule
                }
            )
            .is_err()
    );
    walk_to(&mut service, 9, 15);
    execute(
        &mut service,
        GameplayCommand::TraverseMapExit {
            map: WorldMapId::BoardwalkReturnVestibule,
        },
    );
    assert_eq!(
        institutional_before,
        service.runtime().world().persisted_state_output()
    );
    assert_eq!(service.runtime().regional().events().len(), 0);
    assert!(
        service
            .runtime()
            .boardwalk_case()
            .unwrap()
            .committed_choice()
            .is_none()
    );
    assert!(
        service
            .runtime()
            .physical_exit_destinations()
            .contains(&WorldMapId::AuraBasinCollisionGrounds)
    );
}

fn export_view(name: &str, service: &GameApplicationService) {
    if let Ok(directory) = std::env::var("BOARDWALK_VIEW_OUTPUT_DIR") {
        std::fs::write(
            std::path::Path::new(&directory).join(format!("{name}.json")),
            serde_json::to_vec_pretty(&view(service)).unwrap(),
        )
        .unwrap();
    }
}
