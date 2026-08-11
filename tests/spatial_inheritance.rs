use std::collections::BTreeSet;

use hollow_grove::composition::{
    NormalizedCoordinate, NormalizedPoint, NormalizedRect, SpatialAuthorityId, SpatialFrameId,
    SpatialPathGeometry, SpatialPathId, SpatialRegionId,
};
use hollow_grove::hollow_grove_contract::House;
use hollow_grove::world::geography::ConstitutionalRouteId;
use hollow_grove::world::spatial_inheritance::{
    AURA_RIDGE_EAST_PROJECTION_ID, CANONICAL_SPATIAL_TEMPLATE_ID, CanonicalSpatialAnchor,
    CanonicalSpatialRegion, HollowGroveSpatialDeclaration, HollowGroveSpatialLevel,
    HollowGroveSpatialScope, SHARED_MAP_AUTHORITY_ID, canonical_hollow_grove_spatial_map,
    canonical_spatial_evidence, decode_hollow_grove_spatial_archive,
    encode_hollow_grove_spatial_archive, frame_id, hollow_grove_spatial_replay_digest, region_id,
    route_id,
};

#[test]
fn one_template_is_inherited_by_world_and_every_house_city() {
    let map = canonical_hollow_grove_spatial_map().unwrap();
    assert_eq!(map.template_id().as_str(), CANONICAL_SPATIAL_TEMPLATE_ID);
    assert_eq!(map.catalog().template_count(), 1);
    assert_eq!(map.catalog().frame_count(), 5);

    for scope in [
        HollowGroveSpatialScope::World,
        HollowGroveSpatialScope::House(House::Stonebend),
        HollowGroveSpatialScope::House(House::Sandmanor),
        HollowGroveSpatialScope::House(House::Glaushouse),
        HollowGroveSpatialScope::House(House::Flynt),
    ] {
        let frame = map.catalog().frame(&frame_id(scope).unwrap()).unwrap();
        assert_eq!(frame.template_id(), map.template_id());
    }
}

#[test]
fn mt_aura_and_every_corresponding_viewpoint_keep_local_identity_at_each_scale() {
    let map = canonical_hollow_grove_spatial_map().unwrap();
    let world_route = map
        .resolve_route(
            HollowGroveSpatialScope::World,
            ConstitutionalRouteId::MntAura,
        )
        .unwrap();
    assert_eq!(world_route.geometry, SpatialPathGeometry::Round);

    for house in [
        House::Stonebend,
        House::Sandmanor,
        House::Glaushouse,
        House::Flynt,
    ] {
        let scope = HollowGroveSpatialScope::House(house);
        let route = map
            .resolve_route(scope, ConstitutionalRouteId::MntAura)
            .unwrap();
        assert_eq!(route.local_from, world_route.local_from);
        assert_eq!(route.local_to, world_route.local_to);
        assert_eq!(route.geometry, world_route.geometry);
        assert_eq!(route.band_width, world_route.band_width);
        assert_eq!(route.local_control_points, world_route.local_control_points);

        for anchor in CanonicalSpatialAnchor::ALL {
            assert_eq!(
                map.resolve_anchor(scope, anchor).unwrap().local_position,
                map.resolve_anchor(HollowGroveSpatialScope::World, anchor)
                    .unwrap()
                    .local_position
            );
        }
        for region in CanonicalSpatialRegion::ALL {
            assert_eq!(
                map.resolve_region(scope, region).unwrap().local_polygon,
                map.resolve_region(HollowGroveSpatialScope::World, region)
                    .unwrap()
                    .local_polygon
            );
        }
    }
}

#[test]
fn static_screen_contract_exactly_projects_the_kernel_plan() {
    let map = canonical_hollow_grove_spatial_map().unwrap();
    let report = map.validate_conformance().unwrap();
    assert_eq!(
        (report.anchor_count, report.path_count, report.region_count),
        (6, 11, 3)
    );
    let template = map.catalog().template(map.template_id()).unwrap();
    let artifact: serde_json::Value =
        serde_json::from_str(include_str!("../artifacts/hueman_screen_map.json")).unwrap();

    for anchor in template.anchors() {
        let artifact_id = anchor
            .id
            .as_str()
            .strip_prefix("anchor.")
            .unwrap()
            .replace('-', "_");
        let artifact_anchor = if artifact_id == "ranina" {
            &artifact["center_anchor"]
        } else {
            artifact["nodes"]
                .as_array()
                .unwrap()
                .iter()
                .find(|node| node["id"] == artifact_id)
                .unwrap()
        };
        assert_eq!(
            (anchor.position.x.parts(), anchor.position.y.parts()),
            (
                json_normalized_parts(&artifact_anchor["x"]),
                json_normalized_parts(&artifact_anchor["y"]),
            )
        );
    }

    let mut projected_routes = BTreeSet::new();
    for (collection, expected_geometry) in [
        ("straight_routes", SpatialPathGeometry::Straight),
        ("curved_routes", SpatialPathGeometry::Round),
    ] {
        for route in artifact[collection].as_array().unwrap() {
            let Some(constitutional_id) = route["constitutional_route_id"].as_str() else {
                continue;
            };
            assert!(
                projected_routes.insert(constitutional_id.to_owned()),
                "duplicate constitutional route projection: {constitutional_id}"
            );
            let path = template
                .path(&SpatialPathId::new(constitutional_id).unwrap())
                .unwrap();
            assert_eq!(path.geometry, expected_geometry);
            assert_eq!(
                path.band_width.parts(),
                json_normalized_parts(&route["band_width"])
            );
            let artifact_endpoints = [
                route["from"].as_str().unwrap(),
                route["to"].as_str().unwrap(),
            ]
            .into_iter()
            .collect::<BTreeSet<_>>();
            let kernel_endpoints = [&path.from, &path.to]
                .into_iter()
                .map(|id| id.as_str().strip_prefix("anchor.").unwrap())
                .collect::<BTreeSet<_>>();
            assert_eq!(artifact_endpoints, kernel_endpoints);

            let artifact_controls = route
                .get("control")
                .into_iter()
                .map(|control| {
                    (
                        json_normalized_parts(&control["x"]),
                        json_normalized_parts(&control["y"]),
                    )
                })
                .collect::<Vec<_>>();
            let kernel_controls = path
                .control_points
                .iter()
                .map(|point| (point.x.parts(), point.y.parts()))
                .collect::<Vec<_>>();
            assert_eq!(artifact_controls, kernel_controls);
        }
    }
    assert_eq!(
        projected_routes,
        ConstitutionalRouteId::ALL
            .into_iter()
            .map(|route| route.stable_id().to_owned())
            .collect()
    );

    let local_projection = artifact["straight_routes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|route| route["id"] == "aura_ridge_east")
        .unwrap();
    assert_eq!(
        local_projection["projection_of"],
        ConstitutionalRouteId::AuraRidge.stable_id()
    );
    assert_eq!(local_projection["local_only"], true);
    assert_eq!(local_projection["from"], "aura_ridge_junction");
    assert_eq!(local_projection["to"], "sandmanor");
    let local_path = template
        .path(&SpatialPathId::new(AURA_RIDGE_EAST_PROJECTION_ID).unwrap())
        .unwrap();
    assert_eq!(local_path.from.as_str(), "anchor.aura-ridge-junction");
    assert_eq!(local_path.to.as_str(), "anchor.sandmanor");
    assert_eq!(local_path.geometry, SpatialPathGeometry::Straight);
    assert!(local_path.control_points.is_empty());
    assert_eq!(
        local_path.band_width.parts(),
        json_normalized_parts(&local_projection["band_width"])
    );

    for surface in artifact["surfaces"].as_array().unwrap() {
        let stable_id = format!(
            "region.{}",
            surface["id"].as_str().unwrap().replace('_', "-")
        );
        let region = template
            .region(&SpatialRegionId::new(stable_id).unwrap())
            .unwrap();
        let artifact_polygon = surface["polygon"]
            .as_array()
            .unwrap()
            .iter()
            .map(|point| {
                (
                    json_normalized_parts(&point["x"]),
                    json_normalized_parts(&point["y"]),
                )
            })
            .collect::<Vec<_>>();
        let kernel_polygon = region
            .polygon
            .iter()
            .map(|point| (point.x.parts(), point.y.parts()))
            .collect::<Vec<_>>();
        assert_eq!(artifact_polygon, kernel_polygon);
    }
    for region in CanonicalSpatialRegion::ALL {
        assert!(template.region(&region_id(region).unwrap()).is_some());
    }
}

#[test]
fn a_deeper_map_resolves_directly_to_the_root_and_back_to_its_viewpoint() {
    let mut map = canonical_hollow_grove_spatial_map().unwrap();
    let district_id = SpatialFrameId::new("frame.district.fixture").unwrap();
    map.inherit_nested_map(HollowGroveSpatialDeclaration::inherited(
        district_id.clone(),
        frame_id(HollowGroveSpatialScope::House(House::Stonebend)).unwrap(),
        map.template_id().clone(),
        HollowGroveSpatialLevel::District,
        NormalizedRect::new(point(250, 250), point(750, 750)).unwrap(),
        SpatialAuthorityId::new(SHARED_MAP_AUTHORITY_ID).unwrap(),
        canonical_spatial_evidence().unwrap(),
        None,
    ))
    .unwrap();

    let center = map
        .catalog()
        .resolve_viewpoint(&district_id, NormalizedPoint::center())
        .unwrap();
    assert_eq!(center.depth, 2);
    assert_eq!(map.catalog().lineage(&district_id).unwrap().len(), 3);

    let reverse = map.deepest_viewpoint(center.root_position).unwrap();
    assert_eq!(reverse.frame_id, district_id);
    assert_eq!(reverse.local_position, NormalizedPoint::center());
}

#[test]
fn spatial_kernel_remains_free_of_hollow_grove_geography() {
    const KERNEL: &str = include_str!("../hollow-grove-semantic-kernel/src/spatial_inheritance.rs");
    for forbidden in [
        "Hollow Grove",
        "Ranina",
        "Stonebend",
        "Sandmanor",
        "Glaushouse",
        "Glaüshouse",
        "Flynt",
        "Aura Ridge",
        "Mt. Aura",
        "MntAura",
        "Boardwalk",
        "Glausbahn",
    ] {
        assert!(
            !KERNEL.contains(forbidden),
            "neutral spatial kernel contains {forbidden}"
        );
    }
}

#[test]
fn complete_scale_lineage_reaches_repeatable_last_detail() {
    let mut map = canonical_hollow_grove_spatial_map().unwrap();
    let template_id = map.template_id().clone();
    let authority = SpatialAuthorityId::new(SHARED_MAP_AUTHORITY_ID).unwrap();
    let evidence = canonical_spatial_evidence().unwrap();
    let mut parent = frame_id(HollowGroveSpatialScope::House(House::Stonebend)).unwrap();
    for (id, level) in [
        (
            "frame.district.detail-proof",
            HollowGroveSpatialLevel::District,
        ),
        ("frame.site.detail-proof", HollowGroveSpatialLevel::Site),
        ("frame.room.detail-proof", HollowGroveSpatialLevel::Room),
        ("frame.object.detail-proof", HollowGroveSpatialLevel::Object),
        ("frame.detail.detail-proof", HollowGroveSpatialLevel::Detail),
        (
            "frame.detail.detail-proof.recursive",
            HollowGroveSpatialLevel::Detail,
        ),
    ] {
        let frame = SpatialFrameId::new(id).unwrap();
        map.inherit_nested_map(HollowGroveSpatialDeclaration::inherited(
            frame.clone(),
            parent,
            template_id.clone(),
            level,
            NormalizedRect::new(point(200, 200), point(800, 800)).unwrap(),
            authority.clone(),
            evidence.clone(),
            None,
        ))
        .unwrap();
        parent = frame;
    }

    let report = map.validate_conformance().unwrap();
    assert_eq!(report.maximum_depth, 7);
    assert_eq!(report.frame_count, 11);
    assert_eq!(
        map.catalog().lineage(&parent).unwrap().len(),
        report.maximum_depth + 1
    );
    let mt_aura = map
        .catalog()
        .resolve_path(&parent, &route_id(ConstitutionalRouteId::MntAura).unwrap())
        .unwrap();
    assert_eq!(mt_aura.geometry, SpatialPathGeometry::Round);
    assert_eq!(mt_aura.depth, 7);
}

#[test]
fn spatial_archive_restart_replays_the_same_identity_geometry_and_digest() {
    let map = canonical_hollow_grove_spatial_map().unwrap();
    let before = hollow_grove_spatial_replay_digest(&map).unwrap();
    let archive = encode_hollow_grove_spatial_archive(&map).unwrap();
    let replayed = decode_hollow_grove_spatial_archive(&archive).unwrap();
    let after = hollow_grove_spatial_replay_digest(&replayed).unwrap();
    assert_eq!(before, after);
    assert_eq!(
        replayed.validate_conformance().unwrap(),
        map.validate_conformance().unwrap()
    );
    assert_eq!(
        encode_hollow_grove_spatial_archive(&replayed).unwrap(),
        archive
    );
}

#[test]
fn spatial_archive_rejects_canonical_placement_tampering() {
    let map = canonical_hollow_grove_spatial_map().unwrap();
    let archive = encode_hollow_grove_spatial_archive(&map).unwrap();
    let mut json: serde_json::Value = serde_json::from_slice(&archive).unwrap();
    let declarations = json["declarations"].as_array_mut().unwrap();
    let stonebend = declarations
        .iter_mut()
        .find(|declaration| declaration["frame_id"] == "frame.city.stonebend")
        .unwrap();
    stonebend["bounds_in_parent"]["min"]["x"] = serde_json::json!(420_000_000_u64);
    let tampered = serde_json::to_vec(&json).unwrap();
    assert!(decode_hollow_grove_spatial_archive(&tampered).is_err());
}

fn point(x_per_mille: u64, y_per_mille: u64) -> NormalizedPoint {
    NormalizedPoint::new(
        NormalizedCoordinate::from_ratio(x_per_mille, 1_000).unwrap(),
        NormalizedCoordinate::from_ratio(y_per_mille, 1_000).unwrap(),
    )
}

fn json_normalized_parts(value: &serde_json::Value) -> u64 {
    (value.as_f64().unwrap() * 1_000_000_000.0).round() as u64
}
