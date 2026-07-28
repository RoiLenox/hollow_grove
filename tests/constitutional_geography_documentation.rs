use hollow_grove::world::geography::{ConstitutionalRouteId, canonical_constitutional_geography};

const SPECIFICATION: &str = include_str!("../HOLLOW_GROVE_CONSTITUTIONAL_GEOGRAPHY_V2.md");
const SCREEN_MAP: &str = include_str!("../HUEMAN_SCREEN_MAP_v0.1.0.md");
const CROSSOVERS: &str = include_str!("../HUEMAN_PATH_CROSSOVERS_v0.1.0.md");
const CORE: &str = include_str!("../HOLLOW_GROVE_CORE_v1.0.0.md");
const GEOGRAPHY_SOURCE: &str = include_str!("../src/world/geography.rs");
const GENERATED_CROSSOVERS: &str = include_str!("../artifacts/hueman_path_crossovers.md");
const GENERATED_SCENE_PRESENCE: &str = include_str!("../artifacts/hueman_scene_presence.md");
const GENERATED_SCENE_INTENT: &str = include_str!("../artifacts/hueman_scene_intent.md");
const GENERATED_STACK: &str = include_str!("../artifacts/vertical_integration_stack.md");

#[test]
fn canonical_document_names_every_route_id_name_and_verb() {
    let geography = canonical_constitutional_geography().unwrap();
    for route in geography.routes() {
        assert!(SPECIFICATION.contains(route.id.stable_id()));
        assert!(SPECIFICATION.contains(route.id.display_name()));
        assert!(SPECIFICATION.contains(route.verb.as_str()));
        assert!(SPECIFICATION.contains(route.purpose));
        for stage in route.process {
            assert!(
                SPECIFICATION.contains(stage),
                "specification omits {} process stage {stage}",
                route.id.display_name()
            );
        }
    }
    for boundary in geography.boundaries() {
        assert!(SPECIFICATION.contains(boundary.boundary.stable_id()));
        assert!(SPECIFICATION.contains(boundary.inward_flow));
        assert!(SPECIFICATION.contains(boundary.outward_flow));
    }
}

#[test]
fn public_world_documents_use_the_same_complete_route_roster() {
    for route in ConstitutionalRouteId::ALL {
        for (name, document) in [
            ("screen map", SCREEN_MAP),
            ("path crossovers", CROSSOVERS),
            ("core", CORE),
        ] {
            assert!(
                document.contains(route.display_name()),
                "{name} omits {}",
                route.display_name()
            );
        }
    }
}

#[test]
fn generated_world_projections_match_the_constitutional_route_law() {
    let geography = canonical_constitutional_geography().unwrap();
    for route in geography.routes() {
        assert!(GENERATED_CROSSOVERS.contains(route.id.display_name()));
        assert!(GENERATED_CROSSOVERS.contains(route.verb.as_str()));
    }
    assert!(GENERATED_SCENE_PRESENCE.contains("Riptide / Retrieve"));
    assert!(GENERATED_SCENE_PRESENCE.contains("Current Seanad / Deliberate"));
    assert!(GENERATED_SCENE_INTENT.contains("Stairway to Heaven / Ascend"));
    assert!(GENERATED_SCENE_INTENT.contains("Mt. Aura / Aspire"));
    assert!(
        GENERATED_STACK.contains(
            "constitutional geography owns the world-facing route-purpose roster above Current Synthesis"
        )
    );
}

#[test]
fn current_sea_and_current_seanad_are_documented_as_distinct() {
    assert!(SPECIFICATION.contains("geography.route.current-sea"));
    assert!(SPECIFICATION.contains("geography.route.current-seanad"));
    assert!(SPECIFICATION.contains("Current Sea and Current Seanad remain distinct"));
    assert!(SCREEN_MAP.contains("Current Sea is a Straight civic certification concourse"));
    assert!(SCREEN_MAP.contains("Glausbahn is the Round road sector"));
}

#[test]
fn canonical_world_surfaces_have_no_route_name_drift() {
    for (name, document) in [
        ("specification", SPECIFICATION),
        ("screen map", SCREEN_MAP),
        ("path crossovers", CROSSOVERS),
        ("core", CORE),
        ("neutral projection", GEOGRAPHY_SOURCE),
        ("generated crossovers", GENERATED_CROSSOVERS),
        ("generated scene presence", GENERATED_SCENE_PRESENCE),
        ("generated scene intent", GENERATED_SCENE_INTENT),
        ("generated stack", GENERATED_STACK),
    ] {
        for drift in ["Mount Aura", "Mt Aura", "Mnt. Aura", "Basin Motorspeedway"] {
            assert!(!document.contains(drift), "{name} contains drift `{drift}`");
        }
    }
}

#[test]
fn specification_contains_visual_gameplay_and_godot_recommendations() {
    for section in [
        "## Visual world recommendations",
        "## Godot scene hierarchy recommendation",
        "ConstitutionalRouteLayer",
        "RouteTerrainPresentation",
        "RouteEncounterAnchor",
    ] {
        assert!(SPECIFICATION.contains(section));
    }
    for visual_term in [
        "Terrain",
        "Vegetation",
        "Lighting",
        "Traffic",
        "Architecture",
        "shoreline",
        "mountain",
        "coastline",
        "sky",
        "skybox",
        "Landmark",
        "District transition",
        "Gameplay",
    ] {
        assert!(
            SPECIFICATION
                .to_lowercase()
                .contains(&visual_term.to_lowercase()),
            "visual recommendation omits {visual_term}"
        );
    }
}

#[test]
fn frozen_glaushouse_law_is_repeated_without_reinterpretation() {
    for term in [
        "Doctor Ratchet (Prima Donna)",
        "Nurse House (Persephone)",
        "The Nightingales",
        "Glauspitals",
        "Chromacord",
    ] {
        assert!(SPECIFICATION.contains(term));
    }
    assert!(SPECIFICATION.contains("Route law may present"));
    assert!(SPECIFICATION.contains("may not change their constitutional place"));
}
