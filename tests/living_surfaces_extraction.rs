use std::collections::{BTreeMap, BTreeSet, VecDeque};

use hollow_grove::constitutional::{CausalPosition, ParticipantId, RuleSetId, V2_RULE_SET};
use hollow_grove::gameplay::{
    BeingContinuityId, CardinalDirection, GameApplicationService, GameplayCommand, GameplayEventId,
    GameplayEventMetadata, GameplayRuntimeError, InteractionId, LivingCaseChoice, LivingCaseId,
    LivingWorldError, LivingWorldState, MAP_HEIGHT, MAP_WIDTH, TilePosition, WorldMapDefinition,
    WorldMapId, living_case_definition, map_definition,
};
use hollow_grove::institution::InstitutionalBeingId;
use hollow_grove::world::extraction::{
    CURRENT_SEA_RIG_MAP_ID, ExtractedResource, ExtractionFacilityId, ExtractionMethod,
    ExtractionSiteId, HIGHWAY_TO_HELL_MAP_ID, MNT_AURA_MINE_MAP_ID, RIPTIDE_RIG_MAP_ID,
    STAIRWAY_MINE_MAP_ID, canonical_extraction_sites,
};
use hollow_grove::world::geography::ConstitutionalRouteId;

fn rules() -> RuleSetId {
    RuleSetId::new(V2_RULE_SET).unwrap()
}

fn metadata(sequence: u64) -> GameplayEventMetadata {
    GameplayEventMetadata {
        id: GameplayEventId::new(format!("game-event.living-extraction.{sequence}")).unwrap(),
        causal_position: CausalPosition::new(sequence),
    }
}

fn establish(service: &mut GameApplicationService) {
    service
        .execute(
            metadata(1),
            GameplayCommand::EstablishHuemanIdentity {
                continuity: BeingContinuityId::new("being-continuity.hueman").unwrap(),
                participant: ParticipantId::new("participant.hueman").unwrap(),
                institutional: InstitutionalBeingId::new("being.hueman").unwrap(),
            },
        )
        .unwrap();
}

fn observe_all(state: &mut LivingWorldState, case_id: LivingCaseId) {
    for evidence in case_id.required_evidence() {
        state.observe(case_id, *evidence).unwrap();
    }
}

fn support_and_commit(
    state: &mut LivingWorldState,
    case_id: LivingCaseId,
    choice: LivingCaseChoice,
) {
    state.support(case_id, choice).unwrap();
    state.commit_duty_decision(case_id).unwrap();
}

fn reachable_tiles(map: WorldMapDefinition) -> BTreeSet<(u16, u16)> {
    let mut reachable = BTreeSet::from([(map.spawn.x, map.spawn.y)]);
    let mut frontier = VecDeque::from([(map.spawn.x, map.spawn.y)]);
    while let Some((x, y)) = frontier.pop_front() {
        for (next_x, next_y) in [
            (x.wrapping_sub(1), y),
            (x.saturating_add(1), y),
            (x, y.wrapping_sub(1)),
            (x, y.saturating_add(1)),
        ] {
            if next_x >= MAP_WIDTH || next_y >= MAP_HEIGHT {
                continue;
            }
            let tile = map.rows[next_y as usize].as_bytes()[next_x as usize];
            if matches!(tile, b'.' | b'=') && reachable.insert((next_x, next_y)) {
                frontier.push_back((next_x, next_y));
            }
        }
    }
    reachable
}

fn walk_to(service: &mut GameApplicationService, sequence: &mut u64, destination: (u16, u16)) {
    let map = map_definition(service.runtime().hueman_map());
    let start = service.runtime().hueman_position().unwrap();
    let start = (start.x, start.y);
    let mut frontier = VecDeque::from([start]);
    let mut previous = BTreeMap::new();
    let mut visited = BTreeSet::from([start]);
    while let Some(position) = frontier.pop_front() {
        if position == destination {
            break;
        }
        let (x, y) = position;
        for next in [
            (x.wrapping_sub(1), y),
            (x.saturating_add(1), y),
            (x, y.wrapping_sub(1)),
            (x, y.saturating_add(1)),
        ] {
            if next.0 >= MAP_WIDTH || next.1 >= MAP_HEIGHT {
                continue;
            }
            let tile = map.rows[next.1 as usize].as_bytes()[next.0 as usize];
            if matches!(tile, b'.' | b'=') && visited.insert(next) {
                previous.insert(next, position);
                frontier.push_back(next);
            }
        }
    }
    assert!(visited.contains(&destination), "destination is unreachable");
    let mut path = vec![destination];
    while *path.last().unwrap() != start {
        let current = *path.last().expect("path has a current tile");
        path.push(previous[&current]);
    }
    path.reverse();
    for edge in path.windows(2) {
        let direction = match (
            i32::from(edge[1].0) - i32::from(edge[0].0),
            i32::from(edge[1].1) - i32::from(edge[0].1),
        ) {
            (-1, 0) => CardinalDirection::West,
            (1, 0) => CardinalDirection::East,
            (0, -1) => CardinalDirection::North,
            (0, 1) => CardinalDirection::South,
            other => panic!("invalid path edge: {other:?}"),
        };
        service
            .execute(
                metadata(*sequence),
                GameplayCommand::MoveHueman { direction },
            )
            .unwrap();
        *sequence += 1;
    }
}

fn face_and_interact(
    service: &mut GameApplicationService,
    sequence: &mut u64,
    direction: CardinalDirection,
) {
    service
        .execute(
            metadata(*sequence),
            GameplayCommand::MoveHueman { direction },
        )
        .unwrap();
    *sequence += 1;
    service
        .execute(metadata(*sequence), GameplayCommand::InteractHueman)
        .unwrap();
    *sequence += 1;
}

#[test]
fn solid_mines_and_current_wells_are_distinct_route_bound_workplaces() {
    let sites = canonical_extraction_sites().unwrap();
    assert_eq!(sites.len(), ExtractionSiteId::ALL.len());
    assert_eq!(
        sites
            .iter()
            .filter(|site| site.method == ExtractionMethod::SolidSeamMining)
            .count(),
        3
    );
    assert_eq!(
        sites
            .iter()
            .filter(|site| site.method == ExtractionMethod::OffshoreCurrentWell)
            .count(),
        2
    );

    let highway = sites
        .iter()
        .find(|site| site.id == ExtractionSiteId::HighwayToHellDeepworks)
        .unwrap();
    assert!(highway.id.is_nested_descent());
    assert_eq!(highway.id.route(), ConstitutionalRouteId::StairwayToHeaven);
    assert!(highway.route_limit.contains("never an eleventh"));

    let riptide = sites
        .iter()
        .find(|site| site.id == ExtractionSiteId::RiptideRecoveryRig)
        .unwrap();
    let current_sea = sites
        .iter()
        .find(|site| site.id == ExtractionSiteId::CurrentSeaDepthRig)
        .unwrap();
    assert_eq!(riptide.resource, ExtractedResource::RecoveredCurrentBrine);
    assert_eq!(
        current_sea.resource,
        ExtractedResource::CertifiedCurrentBrine
    );
    assert!(riptide.constitutional_function.contains("retrieve"));
    assert!(
        current_sea
            .constitutional_function
            .contains("certification")
    );
}

#[test]
fn every_extraction_map_is_fixed_reachable_and_has_all_working_facilities() {
    let maps = [
        (
            ExtractionSiteId::MntAuraHighMine,
            MNT_AURA_MINE_MAP_ID,
            vec![
                (
                    ExtractionFacilityId::SurveyOffice,
                    3,
                    1,
                    CardinalDirection::South,
                ),
                (
                    ExtractionFacilityId::Headframe,
                    14,
                    1,
                    CardinalDirection::South,
                ),
                (
                    ExtractionFacilityId::VentilationHouse,
                    3,
                    4,
                    CardinalDirection::South,
                ),
                (
                    ExtractionFacilityId::HoistAndCage,
                    14,
                    4,
                    CardinalDirection::South,
                ),
                (
                    ExtractionFacilityId::PumpStation,
                    9,
                    7,
                    CardinalDirection::South,
                ),
                (
                    ExtractionFacilityId::WorkingFace,
                    3,
                    9,
                    CardinalDirection::South,
                ),
                (
                    ExtractionFacilityId::RefugeChamber,
                    14,
                    9,
                    CardinalDirection::South,
                ),
                (
                    ExtractionFacilityId::GradeAndCustodyYard,
                    3,
                    12,
                    CardinalDirection::South,
                ),
            ],
        ),
        (
            ExtractionSiteId::RiptideRecoveryRig,
            RIPTIDE_RIG_MAP_ID,
            vec![
                (
                    ExtractionFacilityId::Derrick,
                    8,
                    3,
                    CardinalDirection::North,
                ),
                (
                    ExtractionFacilityId::PressureControl,
                    9,
                    5,
                    CardinalDirection::North,
                ),
                (
                    ExtractionFacilityId::DrillFloor,
                    5,
                    5,
                    CardinalDirection::South,
                ),
                (
                    ExtractionFacilityId::CurrentSeparator,
                    14,
                    5,
                    CardinalDirection::South,
                ),
                (
                    ExtractionFacilityId::SpillBoomDepot,
                    5,
                    9,
                    CardinalDirection::South,
                ),
                (
                    ExtractionFacilityId::DiveAndRescueBay,
                    14,
                    9,
                    CardinalDirection::South,
                ),
                (
                    ExtractionFacilityId::CertificationLaboratory,
                    9,
                    13,
                    CardinalDirection::North,
                ),
                (
                    ExtractionFacilityId::TransferManifold,
                    14,
                    13,
                    CardinalDirection::South,
                ),
            ],
        ),
    ];

    for (site, map_id, approaches) in maps {
        let map = map_definition(WorldMapId::ExtractionSite(site));
        assert_eq!(map.id.as_str(), map_id);
        assert_eq!(map.rows.len(), usize::from(MAP_HEIGHT));
        assert!(
            map.rows
                .iter()
                .all(|row| row.len() == usize::from(MAP_WIDTH))
        );
        let reachable = reachable_tiles(map);
        for (facility, x, y, facing) in approaches {
            assert!(
                reachable.contains(&(x, y)),
                "{site:?}/{facility:?} has no walkable approach"
            );
            assert_eq!(
                map.interaction_in_front_with_cases(TilePosition { x, y, facing }, None, None),
                Some(InteractionId::ExtractionFacility { site, facility })
            );
        }
    }

    for (site, map_id) in [
        (ExtractionSiteId::StairwayBurdenMine, STAIRWAY_MINE_MAP_ID),
        (
            ExtractionSiteId::HighwayToHellDeepworks,
            HIGHWAY_TO_HELL_MAP_ID,
        ),
        (ExtractionSiteId::CurrentSeaDepthRig, CURRENT_SEA_RIG_MAP_ID),
    ] {
        assert_eq!(
            WorldMapId::from_wire(map_id).unwrap().extraction(),
            Some(site)
        );
    }
}

#[test]
fn cases_fail_closed_then_apply_cross_region_consequences_and_custody() {
    let mut state = LivingWorldState::canonical().unwrap();
    let initial_field_water = state.field.irrigation_reserve;
    let initial_dune = state.beach.dune_integrity;
    let initial_fish = state.beach.fish_stock;
    let initial_basin_contamination = state.basin.contamination;

    let before = state.clone();
    let error = state
        .support(
            LivingCaseId::AuraFieldDroughtAllocation,
            LivingCaseChoice::EquitableRation,
        )
        .unwrap_err();
    assert_eq!(
        error,
        LivingWorldError::MissingEvidence(LivingCaseId::AuraFieldDroughtAllocation)
    );
    assert_eq!(state, before);

    observe_all(&mut state, LivingCaseId::AuraFieldDroughtAllocation);
    support_and_commit(
        &mut state,
        LivingCaseId::AuraFieldDroughtAllocation,
        LivingCaseChoice::EquitableRation,
    );
    assert!(state.field.irrigation_reserve < initial_field_water);

    observe_all(&mut state, LivingCaseId::MntAuraRoofFall);
    support_and_commit(
        &mut state,
        LivingCaseId::MntAuraRoofFall,
        LivingCaseChoice::ReinforceAndContinue,
    );
    assert!(state.field.irrigation_reserve > initial_field_water - 8);
    assert!(state.beach.dune_integrity > initial_dune);

    observe_all(&mut state, LivingCaseId::RiptideWellBlowout);
    support_and_commit(
        &mut state,
        LivingCaseId::RiptideWellBlowout,
        LivingCaseChoice::ShutInAndRetrieve,
    );
    assert!(state.beach.fish_stock > initial_fish);
    assert!(state.basin.contamination < initial_basin_contamination);
    assert_eq!(state.custody.len(), 2);
    assert!(state.custody.iter().all(|lot| lot.living_blood_excluded));
    assert!(state.custody.iter().all(|lot| {
        lot.provenance
            .iter()
            .any(|line| line.contains("never blood"))
    }));
}

#[test]
fn unsafe_extraction_rescue_and_salvage_choices_are_transactional_refusals() {
    for (case_id, forbidden) in [
        (
            LivingCaseId::AuraBeachStormRescue,
            LivingCaseChoice::KeepShoreOpen,
        ),
        (
            LivingCaseId::AuraBasinInjuredBeing,
            LivingCaseChoice::SalvageTheSubject,
        ),
        (
            LivingCaseId::MntAuraRoofFall,
            LivingCaseChoice::BlastThroughFall,
        ),
        (
            LivingCaseId::HighwayToHellGasPocket,
            LivingCaseChoice::ContinueCutting,
        ),
        (
            LivingCaseId::RiptideWellBlowout,
            LivingCaseChoice::ContinueFlow,
        ),
        (
            LivingCaseId::CurrentSeaWellCertification,
            LivingCaseChoice::BypassCertification,
        ),
    ] {
        let mut state = LivingWorldState::canonical().unwrap();
        observe_all(&mut state, case_id);
        let before = state.clone();
        assert_eq!(
            state.support(case_id, forbidden),
            Err(LivingWorldError::ForbiddenChoice {
                case_id,
                choice: forbidden,
            })
        );
        assert_eq!(state, before);
    }
}

#[test]
fn clock_weather_people_and_unresolved_incidents_are_deterministic() {
    let mut left = LivingWorldState::canonical().unwrap();
    let mut right = LivingWorldState::canonical().unwrap();
    let initial_fish = left.beach.fish_stock;
    let initial_contamination = left.basin.contamination;
    for _ in 0..7 {
        assert_eq!(
            left.advance_shift().unwrap(),
            right.advance_shift().unwrap()
        );
    }
    assert_eq!(left, right);
    assert_eq!(left.revision, 7);
    assert!(left.beach.fish_stock < initial_fish);
    assert!(left.basin.contamination > initial_contamination);
    assert_eq!(left.people.len(), 10);
    assert!(
        left.extraction
            .get(&ExtractionSiteId::StairwayBurdenMine)
            .unwrap()
            .output_units
            > 0
    );
    assert!(
        left.custody
            .iter()
            .any(|lot| lot.origin == ExtractionSiteId::StairwayBurdenMine)
    );
    assert!(
        left.people
            .iter()
            .all(|person| !person.authority_limit.is_empty())
    );
}

#[test]
fn physical_exits_bind_mines_to_routes_and_require_the_exit_tile() {
    let mut service = GameApplicationService::new(rules());
    establish(&mut service);
    service
        .execute(
            metadata(2),
            GameplayCommand::TraverseMapExit {
                map: WorldMapId::for_route(ConstitutionalRouteId::MntAura),
            },
        )
        .unwrap();
    service
        .execute(
            metadata(3),
            GameplayCommand::TraverseMapExit {
                map: WorldMapId::ExtractionSite(ExtractionSiteId::MntAuraHighMine),
            },
        )
        .unwrap();
    assert_eq!(
        service.runtime().hueman_map(),
        WorldMapId::ExtractionSite(ExtractionSiteId::MntAuraHighMine)
    );
    service
        .execute(
            metadata(4),
            GameplayCommand::MoveHueman {
                direction: CardinalDirection::North,
            },
        )
        .unwrap();
    let before_revision = service.revision();
    let before_events = service.events().len();
    let error = service
        .execute(
            metadata(5),
            GameplayCommand::TraverseMapExit {
                map: WorldMapId::for_route(ConstitutionalRouteId::MntAura),
            },
        )
        .unwrap_err();
    assert!(matches!(
        error,
        GameplayRuntimeError::PhysicalExitRequired { .. }
    ));
    assert_eq!(service.revision(), before_revision);
    assert_eq!(service.events().len(), before_events);

    let mut stairway = GameApplicationService::new(rules());
    establish(&mut stairway);
    stairway
        .execute(
            metadata(2),
            GameplayCommand::TraverseMapExit {
                map: WorldMapId::for_route(ConstitutionalRouteId::StairwayToHeaven),
            },
        )
        .unwrap();
    assert!(matches!(
        stairway
            .execute(
                metadata(3),
                GameplayCommand::TraverseMapExit {
                    map: WorldMapId::ExtractionSite(ExtractionSiteId::HighwayToHellDeepworks),
                },
            )
            .unwrap_err(),
        GameplayRuntimeError::DisconnectedMapTransfer { .. }
    ));
    stairway
        .execute(
            metadata(4),
            GameplayCommand::TraverseMapExit {
                map: WorldMapId::ExtractionSite(ExtractionSiteId::StairwayBurdenMine),
            },
        )
        .unwrap();
    stairway
        .execute(
            metadata(5),
            GameplayCommand::TraverseMapExit {
                map: WorldMapId::ExtractionSite(ExtractionSiteId::HighwayToHellDeepworks),
            },
        )
        .unwrap();
}

#[test]
fn living_world_events_persist_and_replay_inside_the_gameplay_archive() {
    let mut service = GameApplicationService::new(rules());
    establish(&mut service);
    for sequence in 2..=6 {
        service
            .execute(metadata(sequence), GameplayCommand::AdvanceLivingWorldShift)
            .unwrap();
    }
    let expected = service.runtime().living_world().clone();
    let archive = service.encode_archive().unwrap();
    let replayed = GameApplicationService::from_archive(&archive).unwrap();
    assert_eq!(replayed.runtime().living_world(), &expected);
    assert_eq!(replayed.encode_archive().unwrap(), archive);
}

#[test]
fn facility_evidence_support_and_duty_decision_flow_through_the_runtime() {
    let mut service = GameApplicationService::new(rules());
    establish(&mut service);
    service
        .execute(
            metadata(2),
            GameplayCommand::EnterMap {
                map: WorldMapId::AuraFieldWorkingLand,
            },
        )
        .unwrap();
    let mut sequence = 3;
    for (approach, facing) in [
        ((7, 5), CardinalDirection::North),
        ((11, 4), CardinalDirection::South),
        ((14, 12), CardinalDirection::North),
    ] {
        walk_to(&mut service, &mut sequence, approach);
        face_and_interact(&mut service, &mut sequence, facing);
    }
    let case = service
        .runtime()
        .living_world()
        .cases
        .get(&LivingCaseId::AuraFieldDroughtAllocation)
        .unwrap();
    assert!(case.ready());
    assert!(case.supported_choice.is_none());

    service
        .execute(
            metadata(sequence),
            GameplayCommand::SupportLivingCase {
                case_id: LivingCaseId::AuraFieldDroughtAllocation,
                choice: LivingCaseChoice::ProtectSeedReserve,
            },
        )
        .unwrap();
    sequence += 1;
    let case = service
        .runtime()
        .living_world()
        .cases
        .get(&LivingCaseId::AuraFieldDroughtAllocation)
        .unwrap();
    assert_eq!(
        case.supported_choice,
        Some(LivingCaseChoice::ProtectSeedReserve)
    );
    assert!(case.resolved_choice.is_none());

    service
        .execute(
            metadata(sequence),
            GameplayCommand::AskLivingDutyOfficerToDecide {
                case_id: LivingCaseId::AuraFieldDroughtAllocation,
            },
        )
        .unwrap();
    let case = service
        .runtime()
        .living_world()
        .cases
        .get(&LivingCaseId::AuraFieldDroughtAllocation)
        .unwrap();
    assert_eq!(
        case.resolved_choice,
        Some(LivingCaseChoice::ProtectSeedReserve)
    );
    assert!(case.player_support_is_nonbinding);

    let archive = service.encode_archive().unwrap();
    let replayed = GameApplicationService::from_archive(&archive).unwrap();
    assert_eq!(
        replayed.runtime().living_world(),
        service.runtime().living_world()
    );
}

#[test]
fn all_lawful_case_paths_name_a_nonplayer_decision_maker() {
    let lawful = [
        (
            LivingCaseId::AuraFieldDroughtAllocation,
            LivingCaseChoice::ProtectSeedReserve,
        ),
        (
            LivingCaseId::AuraBeachStormRescue,
            LivingCaseChoice::GuidedRescue,
        ),
        (
            LivingCaseId::AuraBasinInjuredBeing,
            LivingCaseChoice::TransferToCare,
        ),
        (
            LivingCaseId::MntAuraRoofFall,
            LivingCaseChoice::WithdrawCrew,
        ),
        (
            LivingCaseId::HighwayToHellGasPocket,
            LivingCaseChoice::SealAndVent,
        ),
        (
            LivingCaseId::RiptideWellBlowout,
            LivingCaseChoice::RescueCrewFirst,
        ),
        (
            LivingCaseId::CurrentSeaWellCertification,
            LivingCaseChoice::CertifyReducedRate,
        ),
    ];
    let mut state = LivingWorldState::canonical().unwrap();
    for (case_id, choice) in lawful {
        let contract = living_case_definition(case_id).unwrap();
        assert_eq!(contract.evidence.len(), 3);
        assert!(!contract.dominant_verb.is_empty());
        assert!(!contract.failure_or_refusal.is_empty());
        observe_all(&mut state, case_id);
        support_and_commit(&mut state, case_id, choice);
        let case = state.cases.get(&case_id).unwrap();
        assert!(case.player_support_is_nonbinding);
        assert_eq!(case.decision_maker, case_id.decision_maker());
        assert_eq!(case.authority_class, contract.authority_class);
        assert_eq!(case.resolved_choice, Some(choice));
        assert!(
            case.outcome_id
                .as_deref()
                .unwrap()
                .starts_with("outcome.case.")
        );
    }
}
