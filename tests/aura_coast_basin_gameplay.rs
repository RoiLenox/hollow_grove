use std::collections::{BTreeSet, VecDeque};

use hollow_grove::constitutional::{CausalPosition, ParticipantId, RuleSetId, V2_RULE_SET};
use hollow_grove::gameplay::{
    BeingContinuityId, CardinalDirection, GameApplicationService, GameView, GameplayCommand,
    GameplayEventId, GameplayEventMetadata, GameplayRuntimeError, InteractionId, MAP_HEIGHT,
    MAP_WIDTH, TilePosition, WorldMapDefinition, WorldMapId, map_definition,
};
use hollow_grove::hollow_grove_contract::House;
use hollow_grove::institution::InstitutionalBeingId;
use hollow_grove::world::aura_basin::{
    AURA_BASIN_MAP_ID, AuraBasinFacilityId, AuraBasinFacilityKind, canonical_aura_basin,
};
use hollow_grove::world::aura_beach::{
    AURA_BEACH_MAP_ID, AuraBeachFacilityId, AuraBeachFacilityKind, canonical_aura_beach,
};
use hollow_grove::world::geography::ConstitutionalRouteId;
use hollow_grove::world::interior_surface::InteriorSurfaceId;

fn rules() -> RuleSetId {
    RuleSetId::new(V2_RULE_SET).unwrap()
}

fn metadata(sequence: u64) -> GameplayEventMetadata {
    GameplayEventMetadata {
        id: GameplayEventId::new(format!("game-event.aura-coast-basin.{sequence}")).unwrap(),
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

#[test]
fn beach_and_basin_have_distinct_correct_regional_attribution() {
    assert_eq!(
        InteriorSurfaceId::ALL,
        [
            InteriorSurfaceId::AuraField,
            InteriorSurfaceId::AuraBeach,
            InteriorSurfaceId::AuraBasin,
        ]
    );

    let beach = canonical_aura_beach().unwrap();
    assert_eq!(beach.id.display_name(), "Aura Beach");
    assert_eq!(beach.dominant_house, House::Sandmanor);
    assert!(beach.regional_attribution.contains("Minoan exterior"));
    assert_eq!(beach.facilities.len(), AuraBeachFacilityId::ALL.len());
    assert_eq!(
        beach
            .facilities
            .iter()
            .map(|facility| facility.kind)
            .collect::<BTreeSet<_>>(),
        AuraBeachFacilityKind::REQUIRED.into_iter().collect()
    );
    assert_eq!(
        beach.access_routes.as_slice(),
        InteriorSurfaceId::AuraBeach.access_routes()
    );

    let basin = canonical_aura_basin().unwrap();
    assert_eq!(basin.id.display_name(), "Aura Basin");
    assert_eq!(basin.dominant_house, House::Flynt);
    assert!(basin.regional_attribution.starts_with("Flynt collision"));
    assert_eq!(basin.facilities.len(), AuraBasinFacilityId::ALL.len());
    assert_eq!(
        basin
            .facilities
            .iter()
            .map(|facility| facility.kind)
            .collect::<BTreeSet<_>>(),
        AuraBasinFacilityKind::REQUIRED.into_iter().collect()
    );
    assert_eq!(
        basin.access_routes.as_slice(),
        InteriorSurfaceId::AuraBasin.access_routes()
    );
}

#[test]
fn every_aura_beach_working_place_is_reachable_and_interactive() {
    let map = map_definition(WorldMapId::AuraBeachCoastalCommons);
    assert_eq!(map.id.as_str(), AURA_BEACH_MAP_ID);
    assert_eq!(map.rows.len(), usize::from(MAP_HEIGHT));
    assert!(
        map.rows
            .iter()
            .all(|row| row.len() == usize::from(MAP_WIDTH))
    );
    let reachable = reachable_tiles(map);
    let approaches = [
        (AuraBeachFacilityId::Beacon, 4, 2, CardinalDirection::South),
        (
            AuraBeachFacilityId::WeatherStation,
            8,
            2,
            CardinalDirection::South,
        ),
        (
            AuraBeachFacilityId::TideStation,
            12,
            2,
            CardinalDirection::South,
        ),
        (
            AuraBeachFacilityId::CoastalProvingStrand,
            9,
            5,
            CardinalDirection::North,
        ),
        (
            AuraBeachFacilityId::PublicApproach,
            8,
            6,
            CardinalDirection::North,
        ),
        (
            AuraBeachFacilityId::NavigationSchool,
            4,
            6,
            CardinalDirection::South,
        ),
        (
            AuraBeachFacilityId::RescuePost,
            14,
            6,
            CardinalDirection::South,
        ),
        (
            AuraBeachFacilityId::PublicPier,
            9,
            10,
            CardinalDirection::North,
        ),
        (
            AuraBeachFacilityId::ElfDesignYard,
            4,
            10,
            CardinalDirection::South,
        ),
        (
            AuraBeachFacilityId::CentaurRun,
            8,
            10,
            CardinalDirection::South,
        ),
        (
            AuraBeachFacilityId::DuneWard,
            12,
            10,
            CardinalDirection::South,
        ),
        (
            AuraBeachFacilityId::BoatLanding,
            9,
            12,
            CardinalDirection::South,
        ),
        (
            AuraBeachFacilityId::SalvageYard,
            4,
            15,
            CardinalDirection::North,
        ),
        (
            AuraBeachFacilityId::FishMarket,
            8,
            15,
            CardinalDirection::North,
        ),
        (
            AuraBeachFacilityId::RecoveryPavilion,
            12,
            15,
            CardinalDirection::North,
        ),
    ];
    for (facility, x, y, facing) in approaches {
        assert!(reachable.contains(&(x, y)), "{facility:?} is unreachable");
        let interaction =
            map.interaction_in_front_with_cases(TilePosition { x, y, facing }, None, None);
        assert_eq!(
            interaction,
            Some(InteractionId::AuraBeachFacility(facility))
        );
        let dialogue = hollow_grove::gameplay::InteractionView::from_target(interaction.unwrap());
        assert!(dialogue.pages[0].contains("INSIDE AURA BEACH"));
        assert!(dialogue.pages[2].contains("DOES NOT CREATE SYNTHESIS OR TITLE"));
    }
}

#[test]
fn every_aura_basin_working_place_is_reachable_and_interactive() {
    let map = map_definition(WorldMapId::AuraBasinCollisionGrounds);
    assert_eq!(map.id.as_str(), AURA_BASIN_MAP_ID);
    assert!(
        map.rows
            .iter()
            .all(|row| row.len() == usize::from(MAP_WIDTH))
    );
    let reachable = reachable_tiles(map);
    let approaches = [
        (
            AuraBasinFacilityId::HuntGround,
            1,
            2,
            CardinalDirection::East,
        ),
        (AuraBasinFacilityId::DenSeam, 12, 2, CardinalDirection::East),
        (
            AuraBasinFacilityId::WeaponsRange,
            2,
            4,
            CardinalDirection::South,
        ),
        (
            AuraBasinFacilityId::TransformationTrial,
            7,
            4,
            CardinalDirection::South,
        ),
        (
            AuraBasinFacilityId::RescueWatch,
            2,
            8,
            CardinalDirection::North,
        ),
        (
            AuraBasinFacilityId::ConflictBoundary,
            9,
            8,
            CardinalDirection::North,
        ),
        (
            AuraBasinFacilityId::TrailShelter,
            1,
            10,
            CardinalDirection::East,
        ),
        (
            AuraBasinFacilityId::HollowRecoveryYard,
            10,
            10,
            CardinalDirection::East,
        ),
        (
            AuraBasinFacilityId::FairyMenWaycamp,
            2,
            12,
            CardinalDirection::South,
        ),
        (
            AuraBasinFacilityId::FrameRecoveryGarage,
            5,
            12,
            CardinalDirection::South,
        ),
        (
            AuraBasinFacilityId::TriageEvacuationPoint,
            8,
            12,
            CardinalDirection::South,
        ),
        (
            AuraBasinFacilityId::CompetitionRing,
            11,
            12,
            CardinalDirection::South,
        ),
        (
            AuraBasinFacilityId::LawfulHollowingStation,
            2,
            14,
            CardinalDirection::South,
        ),
        (
            AuraBasinFacilityId::SalvageDepot,
            5,
            14,
            CardinalDirection::South,
        ),
        (
            AuraBasinFacilityId::HideWorks,
            8,
            14,
            CardinalDirection::South,
        ),
        (
            AuraBasinFacilityId::RaceSpur,
            11,
            14,
            CardinalDirection::South,
        ),
    ];
    for (facility, x, y, facing) in approaches {
        assert!(reachable.contains(&(x, y)), "{facility:?} is unreachable");
        let interaction =
            map.interaction_in_front_with_cases(TilePosition { x, y, facing }, None, None);
        assert_eq!(
            interaction,
            Some(InteractionId::AuraBasinFacility(facility))
        );
        let dialogue = hollow_grove::gameplay::InteractionView::from_target(interaction.unwrap());
        assert!(dialogue.pages[0].contains("INSIDE AURA BASIN"));
        assert!(dialogue.pages[2].contains("CREATES NO AUTHORITY"));
    }
}

#[test]
fn both_surface_views_expose_attribution_and_replay_route_entry() {
    for (map, incoming, outgoing, expected_house, expected_count) in [
        (
            WorldMapId::AuraBeachCoastalCommons,
            ConstitutionalRouteId::CurrentSea,
            ConstitutionalRouteId::Glausbahn,
            "Sandmanor",
            15,
        ),
        (
            WorldMapId::AuraBasinCollisionGrounds,
            ConstitutionalRouteId::Boardwalk,
            ConstitutionalRouteId::Riptide,
            "Flynt",
            16,
        ),
    ] {
        let mut service = GameApplicationService::new(rules());
        establish(&mut service);
        service
            .execute(
                metadata(2),
                GameplayCommand::EnterMap {
                    map: WorldMapId::for_route(incoming),
                },
            )
            .unwrap();
        service
            .execute(metadata(3), GameplayCommand::EnterMap { map })
            .unwrap();
        let view = GameView::from_runtime(service.runtime(), vec![]);
        assert!(view.route.is_none());
        let surface = view.surface.unwrap();
        assert_eq!(surface.dominant_house, expected_house);
        assert!(surface.singular_region);
        assert_eq!(surface.facilities.len(), expected_count);
        assert!(!surface.regional_attribution.is_empty());

        service
            .execute(
                metadata(4),
                GameplayCommand::EnterMap {
                    map: WorldMapId::for_route(outgoing),
                },
            )
            .unwrap();
        let archive = service.encode_archive().unwrap();
        let replayed = GameApplicationService::from_archive(&archive).unwrap();
        assert_eq!(replayed.events(), service.events());
    }
}

#[test]
fn incorrect_regional_approaches_fail_without_mutation() {
    for (route, map) in [
        (
            ConstitutionalRouteId::AuraWay,
            WorldMapId::AuraBeachCoastalCommons,
        ),
        (
            ConstitutionalRouteId::Glausbahn,
            WorldMapId::AuraBasinCollisionGrounds,
        ),
    ] {
        let mut service = GameApplicationService::new(rules());
        establish(&mut service);
        service
            .execute(
                metadata(2),
                GameplayCommand::EnterMap {
                    map: WorldMapId::for_route(route),
                },
            )
            .unwrap();
        let revision = service.revision();
        let events = service.events().to_vec();
        let error = service
            .execute(metadata(3), GameplayCommand::EnterMap { map })
            .unwrap_err();
        assert!(matches!(
            error,
            GameplayRuntimeError::DisconnectedMapTransfer { .. }
        ));
        assert_eq!(service.revision(), revision);
        assert_eq!(service.events(), events);
    }
}

#[test]
fn surface_documents_lock_attribution_and_the_compromise_tracks_completion() {
    let beach = include_str!("../AURA_BEACH_SURFACE_V1.md");
    let basin = include_str!("../AURA_BASIN_SURFACE_V1.md");
    let compromise = include_str!("../HOLLOW_GROVE_COMPROMISE_V1_DRAFT.md");
    let roadmap = include_str!("../HOLLOW_GROVE_PLAYABLE_EXPANSION_ROADMAP_V1.md");

    for term in [
        "Sandmanor's Minoan exterior tradition",
        "Minoan Navigation School",
        "Centaur Mobility Run",
        "Current Sea is Aura Beach's linked Minoan civic-circulation jurisdiction",
    ] {
        assert!(beach.contains(term), "Aura Beach contract missing {term}");
    }
    for term in [
        "serves Flynt first",
        "Glaüshouse Complementary Loop",
        "Stonebend Hollowing Station",
        "We Fairy Men Waycamp",
        "does not make Aura Basin the property",
    ] {
        assert!(basin.contains(term), "Aura Basin contract missing {term}");
    }
    assert!(compromise.contains("Aura Beach is one singular Sandmanor Minoan"));
    assert!(compromise.contains("Aura Basin is one singular surface serving Flynt first"));
    assert!(roadmap.contains("Aura Beach — implemented"));
    assert!(roadmap.contains("Aura Basin — implemented"));
}
