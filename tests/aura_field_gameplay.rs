use std::collections::{BTreeSet, VecDeque};

use hollow_grove::constitutional::{
    CausalPosition, ConstitutionalRegion, ParticipantId, RuleSetId, V2_RULE_SET,
};
use hollow_grove::gameplay::{
    BeingContinuityId, CardinalDirection, GameApplicationService, GameView, GameplayCommand,
    GameplayEventId, GameplayEventMetadata, GameplayRuntimeError, InteractionId, MAP_HEIGHT,
    MAP_WIDTH, TilePosition, WorldMapId, map_definition,
};
use hollow_grove::institution::InstitutionalBeingId;
use hollow_grove::world::aura_field::{
    AURA_FIELD_MAP_ID, AuraFieldFacilityId, AuraFieldFacilityKind, canonical_aura_field,
};
use hollow_grove::world::geography::ConstitutionalRouteId;
use hollow_grove::world::house_institutions::canonical_house_institutions;
use hollow_grove::world::interior_surface::InteriorSurfaceId;

fn rules() -> RuleSetId {
    RuleSetId::new(V2_RULE_SET).unwrap()
}

fn metadata(sequence: u64) -> GameplayEventMetadata {
    GameplayEventMetadata {
        id: GameplayEventId::new(format!("game-event.aura-field.{sequence}")).unwrap(),
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

#[test]
fn the_canon_contains_one_aura_field_with_multiple_farms_and_full_field_works() {
    assert!(InteriorSurfaceId::ALL.contains(&InteriorSurfaceId::AuraField));
    let field = canonical_aura_field().unwrap();
    assert_eq!(field.id.display_name(), "Aura Field");
    assert_eq!(field.id.stable_id(), "aura-field");
    assert_eq!(field.map_id, AURA_FIELD_MAP_ID);
    assert_eq!(field.facilities.len(), AuraFieldFacilityId::ALL.len());
    assert_eq!(
        field
            .facilities
            .iter()
            .filter(|facility| facility.kind == AuraFieldFacilityKind::AuraFarm)
            .count(),
        2
    );
    let engagement = field
        .facility(AuraFieldFacilityId::EngagementFarm)
        .expect("Engagement Farm is a distinct Aura Field constituent");
    assert_eq!(engagement.kind, AuraFieldFacilityKind::EngagementFarm);
    assert!(engagement.function.contains("leave-without-debt"));
    let kinds: BTreeSet<_> = field
        .facilities
        .iter()
        .map(|facility| facility.kind)
        .collect();
    assert!(
        AuraFieldFacilityKind::REQUIRED
            .into_iter()
            .all(|kind| kinds.contains(&kind))
    );
    assert_eq!(
        field.access_routes,
        [
            ConstitutionalRouteId::AuraRidge,
            ConstitutionalRouteId::AuraWay,
            ConstitutionalRouteId::MntAura,
        ]
    );
    assert_eq!(field.house_roles.len(), 4);
}

#[test]
fn plural_machine_symbols_project_only_the_singular_geographic_name() {
    assert_eq!(ConstitutionalRegion::AuraFields.as_str(), "Aura Field");
    let catalog = canonical_house_institutions();
    let site = catalog
        .sites
        .iter()
        .find(|site| site.id.as_str() == "site.sandmanor.aura-fields")
        .expect("legacy save-compatible Aura Field site ID");
    assert_eq!(site.name, "Aura Field");
}

#[test]
fn aura_field_map_exposes_every_working_place() {
    let id = WorldMapId::from_wire(AURA_FIELD_MAP_ID).unwrap();
    assert_eq!(id, WorldMapId::AuraFieldWorkingLand);
    assert_eq!(id.route(), None);
    assert_eq!(id.surface(), Some(InteriorSurfaceId::AuraField));
    let map = map_definition(id);
    assert_eq!(map.rows.len(), usize::from(MAP_HEIGHT));
    assert!(
        map.rows
            .iter()
            .all(|row| row.len() == usize::from(MAP_WIDTH))
    );

    let approaches = [
        (AuraFieldFacilityId::Orchard, 3, 4, CardinalDirection::North),
        (
            AuraFieldFacilityId::Paddock,
            12,
            4,
            CardinalDirection::North,
        ),
        (AuraFieldFacilityId::Apiary, 14, 4, CardinalDirection::North),
        (
            AuraFieldFacilityId::IrrigationWorks,
            7,
            5,
            CardinalDirection::North,
        ),
        (
            AuraFieldFacilityId::ProvingPlots,
            11,
            4,
            CardinalDirection::South,
        ),
        (
            AuraFieldFacilityId::UpperAuraFarm,
            3,
            8,
            CardinalDirection::North,
        ),
        (
            AuraFieldFacilityId::EastAuraFarm,
            11,
            8,
            CardinalDirection::North,
        ),
        (
            AuraFieldFacilityId::EngagementFarm,
            16,
            12,
            CardinalDirection::South,
        ),
        (
            AuraFieldFacilityId::Windbreak,
            15,
            6,
            CardinalDirection::East,
        ),
        (AuraFieldFacilityId::Barn, 6, 12, CardinalDirection::North),
        (
            AuraFieldFacilityId::Granary,
            14,
            12,
            CardinalDirection::North,
        ),
        (
            AuraFieldFacilityId::CompostYard,
            6,
            12,
            CardinalDirection::East,
        ),
        (
            AuraFieldFacilityId::Farmstead,
            5,
            12,
            CardinalDirection::South,
        ),
        (
            AuraFieldFacilityId::ProduceMarket,
            12,
            12,
            CardinalDirection::South,
        ),
        (
            AuraFieldFacilityId::ToolShed,
            5,
            16,
            CardinalDirection::West,
        ),
        (
            AuraFieldFacilityId::SeedHouse,
            12,
            16,
            CardinalDirection::East,
        ),
    ];
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
    for (facility, x, y, facing) in approaches {
        assert!(reachable.contains(&(x, y)), "{facility:?} is unreachable");
        let interaction =
            map.interaction_in_front_with_cases(TilePosition { x, y, facing }, None, None);
        assert_eq!(
            interaction,
            Some(InteractionId::AuraFieldFacility(facility))
        );
        let dialogue = hollow_grove::gameplay::InteractionView::from_target(interaction.unwrap());
        assert!(dialogue.pages[0].contains("ONE AURA FIELD"));
        assert!(dialogue.pages[2].contains("DOES NOT CREATE TITLE"));
    }
}

#[test]
fn aura_field_is_a_surface_view_and_route_connected_working_region() {
    let mut service = GameApplicationService::new(rules());
    establish(&mut service);
    service
        .execute(
            metadata(2),
            GameplayCommand::EnterMap {
                map: WorldMapId::for_route(ConstitutionalRouteId::AuraWay),
            },
        )
        .unwrap();
    service
        .execute(
            metadata(3),
            GameplayCommand::EnterMap {
                map: WorldMapId::AuraFieldWorkingLand,
            },
        )
        .unwrap();

    let view = GameView::from_runtime(service.runtime(), vec![]);
    assert!(view.route.is_none());
    let surface = view.surface.unwrap();
    assert_eq!(surface.surface_id, "aura-field");
    assert_eq!(surface.display_name, "Aura Field");
    assert_eq!(surface.dominant_house, "Sandmanor");
    assert!(surface.regional_attribution.contains("Minorian interior"));
    assert!(surface.singular_region);
    assert_eq!(surface.facilities.len(), AuraFieldFacilityId::ALL.len());
    assert_eq!(surface.boundary.len(), 3);

    service
        .execute(
            metadata(4),
            GameplayCommand::EnterMap {
                map: WorldMapId::for_route(ConstitutionalRouteId::MntAura),
            },
        )
        .unwrap();
    let archive = service.encode_archive().unwrap();
    let replayed = GameApplicationService::from_archive(&archive).unwrap();
    assert_eq!(replayed.events(), service.events());
    assert_eq!(
        replayed.runtime().hueman_map(),
        WorldMapId::for_route(ConstitutionalRouteId::MntAura)
    );
}

#[test]
fn disconnected_routes_cannot_jump_into_aura_field() {
    let mut service = GameApplicationService::new(rules());
    establish(&mut service);
    service
        .execute(
            metadata(2),
            GameplayCommand::EnterMap {
                map: WorldMapId::for_route(ConstitutionalRouteId::Boardwalk),
            },
        )
        .unwrap();
    let revision = service.revision();
    let events = service.events().to_vec();
    let error = service
        .execute(
            metadata(3),
            GameplayCommand::EnterMap {
                map: WorldMapId::AuraFieldWorkingLand,
            },
        )
        .unwrap_err();
    assert!(matches!(
        error,
        GameplayRuntimeError::DisconnectedMapTransfer { ref from, ref to }
            if from == "boardwalk.return-vestibule" && to == AURA_FIELD_MAP_ID
    ));
    assert_eq!(service.revision(), revision);
    assert_eq!(service.events(), events);
}
