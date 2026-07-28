use hollow_grove::constitutional::{CausalPosition, ParticipantId, RuleSetId, V2_RULE_SET};
use hollow_grove::gameplay::{
    BeingContinuityId, CardinalDirection, GameApplicationService, GameView, GameplayCommand,
    GameplayEventId, GameplayEventMetadata, InteractionId, MAP_HEIGHT, MAP_WIDTH,
    ROUND_ROUTE_MAP_ROWS, TilePosition, WorldMapId, map_definition,
};
use hollow_grove::institution::InstitutionalBeingId;
use hollow_grove::world::geography::ConstitutionalRouteId;
use hollow_grove::world::route_network::{RouteGeometryClass, RouteNetwork};

fn rules() -> RuleSetId {
    RuleSetId::new(V2_RULE_SET).unwrap()
}

fn metadata(sequence: u64) -> GameplayEventMetadata {
    GameplayEventMetadata {
        id: GameplayEventId::new(format!("game-event.route-network.{sequence}")).unwrap(),
        causal_position: CausalPosition::new(sequence),
    }
}

#[test]
fn canonical_network_has_five_straight_and_five_round_routes() {
    let network = RouteNetwork::canonical().unwrap();
    assert_eq!(network.segments().len(), 10);
    assert_eq!(
        network
            .routes_by_geometry(RouteGeometryClass::Straight)
            .len(),
        5
    );
    assert_eq!(
        network.routes_by_geometry(RouteGeometryClass::Round).len(),
        5
    );
    assert_eq!(
        network.routes_by_geometry(RouteGeometryClass::Straight),
        vec![
            ConstitutionalRouteId::Boardwalk,
            ConstitutionalRouteId::CurrentSea,
            ConstitutionalRouteId::AuraRidge,
            ConstitutionalRouteId::AuraWay,
            ConstitutionalRouteId::BasinMotorspeedway,
        ]
    );
    assert_eq!(
        network.routes_by_geometry(RouteGeometryClass::Round),
        vec![
            ConstitutionalRouteId::Riptide,
            ConstitutionalRouteId::Glausbahn,
            ConstitutionalRouteId::CurrentSeanad,
            ConstitutionalRouteId::MntAura,
            ConstitutionalRouteId::StairwayToHeaven,
        ]
    );
    for segment in network.segments() {
        assert_ne!(segment.endpoints[0], segment.endpoints[1]);
        assert!(segment.start.x_per_mille <= 1_000);
        assert!(segment.start.y_per_mille <= 1_000);
        assert!(segment.end.x_per_mille <= 1_000);
        assert!(segment.end.y_per_mille <= 1_000);
    }
}

#[test]
fn every_constitutional_route_has_one_canonical_playable_map() {
    for route in ConstitutionalRouteId::ALL {
        let id = WorldMapId::for_route(route);
        assert!(id.is_canonical());
        assert_eq!(id.route(), Some(route));
        assert_eq!(WorldMapId::from_wire(id.as_str()).unwrap(), id);
        let map = map_definition(id);
        assert_eq!(map.id, id);
        assert_eq!(map.rows.len(), usize::from(MAP_HEIGHT));
        assert!(
            map.rows
                .iter()
                .all(|row| row.len() == usize::from(MAP_WIDTH)),
            "{} has malformed authored rows",
            id.as_str()
        );
    }
}

#[test]
fn glausbahn_uses_round_road_geometry_and_current_sea_uses_a_civic_crowd_map() {
    let glausbahn = map_definition(WorldMapId::for_route(ConstitutionalRouteId::Glausbahn));
    assert_eq!(glausbahn.rows, &ROUND_ROUTE_MAP_ROWS);

    let current_sea = map_definition(WorldMapId::for_route(ConstitutionalRouteId::CurrentSea));
    assert!(current_sea.rows.iter().all(|row| !row.contains('~')));
    assert!(current_sea.rows.iter().any(|row| row.contains('c')));
}

#[test]
fn generic_route_maps_expose_their_exact_constitutional_witness() {
    for route in [
        ConstitutionalRouteId::Riptide,
        ConstitutionalRouteId::Glausbahn,
        ConstitutionalRouteId::CurrentSeanad,
        ConstitutionalRouteId::AuraWay,
        ConstitutionalRouteId::MntAura,
        ConstitutionalRouteId::BasinMotorspeedway,
        ConstitutionalRouteId::StairwayToHeaven,
    ] {
        let map = map_definition(WorldMapId::for_route(route));
        assert_eq!(
            map.interaction_in_front_with_cases(
                TilePosition {
                    x: 9,
                    y: 5,
                    facing: CardinalDirection::North,
                },
                None,
                None,
            ),
            Some(InteractionId::ConstitutionalRouteWitness(route))
        );
        let dialogue = hollow_grove::gameplay::InteractionView::from_target(
            InteractionId::ConstitutionalRouteWitness(route),
        );
        assert_eq!(dialogue.speaker, route.display_name().to_uppercase());
        assert!(
            dialogue
                .pages
                .iter()
                .any(|page| page.contains("GEOMETRY IS NOT AUTHORITY"))
        );
    }
}

#[test]
fn route_transfers_follow_shared_house_endpoints_and_replay() {
    let route_cycle = [
        ConstitutionalRouteId::CurrentSea,
        ConstitutionalRouteId::AuraWay,
        ConstitutionalRouteId::MntAura,
        ConstitutionalRouteId::BasinMotorspeedway,
        ConstitutionalRouteId::StairwayToHeaven,
        ConstitutionalRouteId::Boardwalk,
        ConstitutionalRouteId::Riptide,
        ConstitutionalRouteId::Glausbahn,
        ConstitutionalRouteId::CurrentSeanad,
        ConstitutionalRouteId::AuraRidge,
    ];
    let network = RouteNetwork::canonical().unwrap();
    let mut prior = ConstitutionalRouteId::AuraRidge;
    for route in route_cycle {
        assert!(network.can_transfer(prior, route));
        assert!(network.transfer_house(prior, route).is_some());
        prior = route;
    }

    let mut service = GameApplicationService::new(rules());
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
    let mut sequence = 2;
    for route in route_cycle {
        service
            .execute(
                metadata(sequence),
                GameplayCommand::EnterMap {
                    map: WorldMapId::for_route(route),
                },
            )
            .unwrap();
        let view = GameView::from_runtime(service.runtime(), vec![]);
        let route_view = view.route.unwrap();
        assert_eq!(route_view.route_id, route.stable_id());
        assert_eq!(route_view.display_name, route.display_name());
        sequence += 1;
    }

    let archive = service.encode_archive().unwrap();
    let replayed = GameApplicationService::from_archive(&archive).unwrap();
    assert_eq!(replayed.events(), service.events());
    assert_eq!(
        replayed.runtime().hueman_map(),
        WorldMapId::AuraRidgeGroveApproach
    );
}

#[test]
fn disconnected_route_jump_fails_without_mutating_gameplay() {
    let mut service = GameApplicationService::new(rules());
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
    service
        .execute(
            metadata(2),
            GameplayCommand::EnterMap {
                map: WorldMapId::for_route(ConstitutionalRouteId::AuraWay),
            },
        )
        .unwrap();
    let events = service.events().to_vec();
    let revision = service.revision();
    let error = service
        .execute(
            metadata(3),
            GameplayCommand::EnterMap {
                map: WorldMapId::for_route(ConstitutionalRouteId::Boardwalk),
            },
        )
        .unwrap_err();
    assert!(matches!(
        error,
        hollow_grove::gameplay::GameplayRuntimeError::DisconnectedRouteTransfer {
            from: ConstitutionalRouteId::AuraWay,
            to: ConstitutionalRouteId::Boardwalk,
        }
    ));
    assert_eq!(service.events(), events);
    assert_eq!(service.revision(), revision);
}

#[test]
fn route_skeleton_claims_all_three_implemented_interior_surfaces() {
    assert_eq!(
        WorldMapId::from_wire("aura-field").unwrap(),
        WorldMapId::AuraFieldWorkingLand
    );
    assert_eq!(
        WorldMapId::from_wire("aura-beach").unwrap(),
        WorldMapId::AuraBeachCoastalCommons
    );
    assert_eq!(
        WorldMapId::from_wire("aura-basin").unwrap(),
        WorldMapId::AuraBasinCollisionGrounds
    );
    let client = include_str!("../hueman_godot/scripts/retro_overworld.gd");
    for map in [
        "aura-way.design-corridor",
        "aura-field.working-land",
        "aura-beach.coastal-commons",
        "aura-basin.collision-grounds",
        "mnt-aura.aspiration-path",
        "basin-motor-speedway.production-circuit",
        "stairway-to-heaven.ascent-path",
        "riptide.emergency-intake",
        "glausbahn.refinement-span",
        "current-seanad.deliberation-chamber",
    ] {
        assert!(client.contains(map), "client route cycle missing {map}");
    }
}
