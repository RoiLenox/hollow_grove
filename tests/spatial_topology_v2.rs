use std::collections::BTreeSet;

use hollow_grove::world::adaptive_midpoint_hinges::{
    audit_adaptive_topology_morph, canonical_adaptive_morph_contract,
};
use hollow_grove::world::spatial_topology_v2::{
    LEGACY_CURRENT_SEANAD_ROUTE_ID, LEGACY_GLAUSBAHN_ROUTE_ID, TopologyV2RegionId,
    TopologyV2RouteId, canonical_hollow_grove_spatial_topology_v2, decode_spatial_topology_v2,
    migrate_spatial_v1_archive_to_topology_v2, spatial_topology_v2_digest,
};

const OVERLAY: &str = include_str!("../artifacts/hollow_grove_spatial_topology_v2_overlay.json");
const ADAPTIVE_MORPH: &str = include_str!("../artifacts/hollow_grove_adaptive_morph_v1.json");
const GODOT_SHELL: &str = include_str!("../hueman_godot/scripts/hueman_screen_shell.gd");
const CHECKPOINT_CAPTURE: &str =
    include_str!("../hueman_godot/scripts/morph_checkpoint_capture.gd");
const VISUAL_APPROVAL: &str = include_str!("../HOLLOW_GROVE_MORPH_VISUAL_APPROVAL_V1.md");
const SPEC: &str = include_str!("../HOLLOW_GROVE_SPATIAL_TOPOLOGY_V2.md");
const FROZEN_SPATIAL_V1_FIXTURE: &[u8] =
    include_bytes!("fixtures/compatibility/spatial_inheritance_v1.json");
const APPROVED_MORPH_RENDERS: [(&str, &[u8]); 5] = [
    (
        "hollow_grove_morph_000.png",
        include_bytes!("../artifacts/hollow_grove_morph_checkpoints/hollow_grove_morph_000.png"),
    ),
    (
        "hollow_grove_morph_025.png",
        include_bytes!("../artifacts/hollow_grove_morph_checkpoints/hollow_grove_morph_025.png"),
    ),
    (
        "hollow_grove_morph_050.png",
        include_bytes!("../artifacts/hollow_grove_morph_checkpoints/hollow_grove_morph_050.png"),
    ),
    (
        "hollow_grove_morph_075.png",
        include_bytes!("../artifacts/hollow_grove_morph_checkpoints/hollow_grove_morph_075.png"),
    ),
    (
        "hollow_grove_morph_100.png",
        include_bytes!("../artifacts/hollow_grove_morph_checkpoints/hollow_grove_morph_100.png"),
    ),
];

#[test]
fn frozen_v1_fixture_migrates_without_mutating_the_source_archive() {
    let before = FROZEN_SPATIAL_V1_FIXTURE.to_vec();
    let migrated = migrate_spatial_v1_archive_to_topology_v2(FROZEN_SPATIAL_V1_FIXTURE).unwrap();

    assert_eq!(FROZEN_SPATIAL_V1_FIXTURE, before.as_slice());
    let topology = decode_spatial_topology_v2(&migrated).unwrap();
    let audit = topology.validate().unwrap();
    assert_eq!(audit.route_count, 9);
    assert_eq!(audit.straight_route_count, 5);
    assert_eq!(audit.curved_route_count, 4);
    assert_eq!(
        spatial_topology_v2_digest(&topology).unwrap(),
        "fnv1a64-hollow-grove-spatial-topology-v2:936a8883839fe67f"
    );
}

#[test]
fn presentation_overlay_matches_the_corrected_route_roster() {
    let overlay: serde_json::Value = serde_json::from_str(OVERLAY).unwrap();
    assert_eq!(overlay["schema_version"], 2);
    let straight = overlay["straight_routes"].as_array().unwrap();
    let curved = overlay["curved_routes"].as_array().unwrap();
    let route_ids = straight
        .iter()
        .chain(curved)
        .filter_map(|route| route["constitutional_route_id"].as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(route_ids.len(), TopologyV2RouteId::ALL.len());
    for route in TopologyV2RouteId::ALL {
        assert!(
            route_ids.contains(route.stable_id()),
            "overlay missing {route:?}"
        );
    }
    assert!(!OVERLAY.contains("Current Seanad"));
    assert!(!route_ids.contains(LEGACY_CURRENT_SEANAD_ROUTE_ID));
    assert!(!route_ids.contains(LEGACY_GLAUSBAHN_ROUTE_ID));

    let glausbaughn = straight
        .iter()
        .find(|route| {
            route["constitutional_route_id"] == TopologyV2RouteId::Glausbaughn.stable_id()
        })
        .unwrap();
    assert!(glausbaughn.get("control").is_none());
    let current_sea = curved
        .iter()
        .find(|route| route["constitutional_route_id"] == TopologyV2RouteId::CurrentSea.stable_id())
        .unwrap();
    assert_eq!(current_sea["control"]["x"], 0.73);
    assert_eq!(current_sea["control"]["y"], 0.8);
}

#[test]
fn presentation_morph_is_derived_exactly_from_the_bottom_up_contract() {
    let checked_artifact: serde_json::Value = serde_json::from_str(ADAPTIVE_MORPH).unwrap();
    let canonical = serde_json::to_value(canonical_adaptive_morph_contract().unwrap()).unwrap();
    assert_eq!(checked_artifact, canonical);

    let audit = audit_adaptive_topology_morph().unwrap();
    assert_eq!(audit.sample_count, 21);
    assert!(audit.endpoints_remain_fixed);
    assert!(audit.slots_remain_isolated);
    assert!(audit.canonical_circle_is_preserved);
    assert!(audit.rectangular_symmetry_is_exact);
    assert!(audit.central_spine_remains_fixed);

    for required in [
        "ADAPTIVE_MORPH_CONTRACT_RELATIVE_PATH",
        "_adaptive_route_points",
        "_adaptive_midway_position",
        "_on_adaptive_rectangularity_changed",
        "adaptive_morph_slider",
    ] {
        assert!(
            GODOT_SHELL.contains(required),
            "Godot shell missing {required}"
        );
    }
}

#[test]
fn visual_approval_freezes_five_real_godot_checkpoints() {
    assert!(CHECKPOINT_CAPTURE.contains("const CHECKPOINTS := [0.0, 0.25, 0.5, 0.75, 1.0]"));
    assert!(CHECKPOINT_CAPTURE.contains("set_topology_review_mode"));
    for (name, bytes) in APPROVED_MORPH_RENDERS {
        assert!(
            VISUAL_APPROVAL.contains(name),
            "approval record missing {name}"
        );
        assert!(
            bytes.len() > 50_000,
            "approved render {name} is unexpectedly small"
        );
        assert_eq!(&bytes[..8], b"\x89PNG\r\n\x1a\n");
        assert_eq!(u32::from_be_bytes(bytes[16..20].try_into().unwrap()), 1280);
        assert_eq!(u32::from_be_bytes(bytes[20..24].try_into().unwrap()), 720);
    }
}

#[test]
fn presentation_overlay_matches_the_aura_farm_partition() {
    let overlay: serde_json::Value = serde_json::from_str(OVERLAY).unwrap();
    let surfaces = overlay["surfaces"].as_array().unwrap();
    for (id, region) in [
        ("aura_field", TopologyV2RegionId::AuraField),
        ("content_farm", TopologyV2RegionId::ContentFarm),
        ("aura_beach", TopologyV2RegionId::AuraBeach),
        ("aura_basin", TopologyV2RegionId::AuraBasin),
    ] {
        let actual = surfaces.iter().find(|surface| surface["id"] == id).unwrap()["polygon"]
            .as_array()
            .unwrap()
            .iter()
            .map(|point| {
                (
                    (point["x"].as_f64().unwrap() * 1_000.0).round() as u16,
                    (point["y"].as_f64().unwrap() * 1_000.0).round() as u16,
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(actual, region.polygon_per_mille());
    }
    assert!(GODOT_SHELL.contains("SPATIAL_TOPOLOGY_V2_OVERLAY_RELATIVE_PATH"));
    assert!(GODOT_SHELL.contains("_draw_surface(\"content_farm\""));
}

#[test]
fn v2_spec_and_runtime_agree_on_the_correction() {
    canonical_hollow_grove_spatial_topology_v2().unwrap();
    for required in [
        "exact equal-area triangles",
        "Glausbaughn | Straight",
        "Current Sea | Round",
        "Current Seanad` is not a route",
        "geography.route.current-seanad -> geography.route.current-sea",
    ] {
        assert!(SPEC.contains(required), "V2 spec missing {required}");
    }
}
