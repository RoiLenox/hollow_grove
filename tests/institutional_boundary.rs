use std::fs;
use std::path::PathBuf;

/// Institutional facts may be projected to scenes and Falloutman, but they
/// must not become a hidden decision engine or enter the typed routing kernel.
#[test]
fn institutional_domain_stays_outside_routing_and_decision_selection() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let protected_files = [
        "src/point.rs",
        "src/symptom.rs",
        "src/triway.rs",
        "src/hollow_grove.rs",
        "src/grove_seam.rs",
        "src/hollow_beam.rs",
        "src/kernel_pass.rs",
        "src/pleb_meta/mod.rs",
        "src/pleb_meta/sequence.rs",
        "src/pleb_meta/mode.rs",
        "src/pleb_meta/operator.rs",
        "src/pleb_meta/route.rs",
        "src/pleb_meta/bond.rs",
        "src/pleb_meta/grammar.rs",
        "src/pleb_meta/routing_pass.rs",
        "src/decision_engine.rs",
        "src/current_synthesis_engine.rs",
    ];
    let forbidden_dependencies = [
        "InstitutionalWorldState",
        "InstitutionalSceneContext",
        "InstitutionalAction",
        "InstitutionalVerb",
        "WorldSession",
        "present_institutional_access",
    ];

    for file in protected_files {
        let source = fs::read_to_string(root.join(file)).expect("protected source must exist");
        for dependency in forbidden_dependencies {
            assert!(
                !source.contains(dependency),
                "{file} must not depend on institutional scene or action state: {dependency}"
            );
        }
    }
}
