use std::fs;
use std::path::Path;

use hollow_grove::hollow_grove_contract::House;
use hollow_grove::world::geography::{
    ConstitutionalFlowDirection, ConstitutionalRouteId as Route, ConstitutionalRouteVerb as Verb,
    FrozenRuntimeRouteKey, HouseBoundary, canonical_constitutional_geography,
};

#[test]
fn every_major_route_has_one_unique_constitutional_verb() {
    let geography = canonical_constitutional_geography().unwrap();
    let audit = geography.audit().unwrap();

    assert_eq!(audit.route_count, 10);
    assert_eq!(audit.distinct_purpose_count, 10);
    assert_eq!(audit.distinct_verb_count, 10);
    assert_eq!(audit.duplicate_role_count, 0);
    assert!(audit.every_route_has_one_purpose);
    assert!(audit.every_route_has_process_flow);
    assert_eq!(
        geography
            .routes()
            .iter()
            .map(|route| (route.id, route.verb))
            .collect::<Vec<_>>(),
        [
            (Route::Boardwalk, Verb::Return),
            (Route::Riptide, Verb::Retrieve),
            (Route::CurrentSea, Verb::Certify),
            (Route::AuraRidge, Verb::Witness),
            (Route::Glausbahn, Verb::Refine),
            (Route::CurrentSeanad, Verb::Deliberate),
            (Route::AuraWay, Verb::Design),
            (Route::MntAura, Verb::Aspire),
            (Route::BasinMotorspeedway, Verb::Produce),
            (Route::StairwayToHeaven, Verb::Ascend),
        ]
    );
}

#[test]
fn every_house_boundary_has_exactly_two_routes_and_coherent_flow() {
    let geography = canonical_constitutional_geography().unwrap();
    let audit = geography.audit().unwrap();

    assert!(audit.every_boundary_has_two_routes);
    assert!(audit.every_boundary_has_inward_and_outward_flow);
    for boundary in HouseBoundary::ALL {
        assert_eq!(geography.routes_for_boundary(boundary).len(), 2);
        let law = geography
            .boundaries()
            .iter()
            .find(|law| law.boundary == boundary)
            .unwrap();
        assert!(!law.inward_flow.is_empty());
        assert!(!law.outward_flow.is_empty());
    }
}

#[test]
fn boardwalk_returns_while_riptide_retrieves() {
    let geography = canonical_constitutional_geography().unwrap();

    assert_eq!(
        geography.route(Route::Boardwalk).unwrap().direction,
        ConstitutionalFlowDirection::Directed {
            from: House::Glaushouse,
            to: House::Flynt,
        }
    );
    assert_eq!(
        geography.route(Route::Riptide).unwrap().direction,
        ConstitutionalFlowDirection::Directed {
            from: House::Flynt,
            to: House::Glaushouse,
        }
    );
}

#[test]
fn current_sea_and_current_seanad_cannot_be_conflated() {
    let geography = canonical_constitutional_geography().unwrap();
    let audit = geography.audit().unwrap();

    assert!(audit.current_sea_is_distinct_from_current_seanad);
    assert!(audit.frozen_runtime_projections_are_unique);
    assert_eq!(
        geography
            .route(Route::CurrentSea)
            .unwrap()
            .frozen_runtime_projection,
        None
    );
    assert_eq!(
        geography
            .route(Route::CurrentSeanad)
            .unwrap()
            .frozen_runtime_projection,
        Some(FrozenRuntimeRouteKey::CurrentSeanad)
    );
}

#[test]
fn established_route_names_and_meanings_remain_exact() {
    let geography = canonical_constitutional_geography().unwrap();
    assert_eq!(Route::MntAura.display_name(), "Mt. Aura");
    assert_eq!(geography.route(Route::MntAura).unwrap().verb, Verb::Aspire);
    assert_eq!(
        Route::BasinMotorspeedway.display_name(),
        "Basin Motor Speedway"
    );
    assert_eq!(
        geography.route(Route::BasinMotorspeedway).unwrap().verb,
        Verb::Produce
    );
}

#[test]
fn frozen_layers_do_not_import_constitutional_geography() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    for frozen_root in [
        root.join("hollow-grove-kernel"),
        root.join("officials-and-outlaws"),
        root.join("src/constitutional"),
    ] {
        assert_no_geography_import(&frozen_root);
    }
}

#[test]
fn geography_projection_does_not_import_frozen_authority_engines() {
    let source = include_str!("../src/world/geography.rs");
    assert!(!source.contains("flynt_constitution"));
    assert!(!source.contains("crate::constitutional"));
    assert!(!source.contains("hollow_grove_kernel"));
    assert!(!source.contains("current_synthesis_engine"));
}

fn assert_no_geography_import(path: &Path) {
    for entry in fs::read_dir(path).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            if path.file_name().and_then(|name| name.to_str()) == Some("target") {
                continue;
            }
            assert_no_geography_import(&path);
            continue;
        }
        if path.extension().and_then(|extension| extension.to_str()) != Some("rs") {
            continue;
        }
        let contents = fs::read_to_string(&path).unwrap();
        assert!(!contents.contains("world::geography"), "{}", path.display());
        assert!(
            !contents.contains("constitutional_geography"),
            "{}",
            path.display()
        );
    }
}
