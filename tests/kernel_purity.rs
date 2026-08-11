use std::fs;
use std::path::PathBuf;

#[test]
fn kernel_sources_do_not_contain_named_world_geography() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let kernel_files = [
        "src/lib.rs",
        "src/point.rs",
        "src/symptom.rs",
        "src/triway.rs",
        "src/hollow_grove.rs",
        "src/grove_seam.rs",
        "src/hollow_beam.rs",
        "src/kernel_pass.rs",
        "hollow-grove-recursion-kernel/src/pleb_meta/mod.rs",
        "hollow-grove-recursion-kernel/src/pleb_meta/sequence.rs",
        "hollow-grove-recursion-kernel/src/pleb_meta/mode.rs",
        "hollow-grove-recursion-kernel/src/pleb_meta/operator.rs",
        "hollow-grove-recursion-kernel/src/pleb_meta/route.rs",
        "hollow-grove-recursion-kernel/src/pleb_meta/bond.rs",
        "hollow-grove-recursion-kernel/src/pleb_meta/grammar.rs",
        "hollow-grove-recursion-kernel/src/pleb_meta/routing_pass.rs",
    ];
    let banned = [
        "Stonebend",
        "Sandmanor",
        "Glaushouse",
        "Glaüshouse",
        "Flynt",
        "BasinMotorSpeedway",
        "StairwayToHeaven",
        "Boardwalk",
        "Riptide",
        "Glausbahn",
        "CurrentSeanad",
        "AuraWay",
        "MountAura",
    ];

    for file in kernel_files {
        let source = fs::read_to_string(root.join(file)).unwrap();
        for name in banned {
            assert!(
                !source.contains(name),
                "{file} should not contain world name {name}"
            );
        }
    }
}
