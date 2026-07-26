use std::fs;
use std::path::PathBuf;

#[test]
fn hollow_grove_kernel_has_no_flynt_constitution_dependency_or_terms() {
    let layer_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let kernel_root = layer_root.parent().unwrap();
    let kernel_files = [
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
    ];
    let banned_lowercase = [
        "flynt constitution",
        "flynt-constitution",
        "flynt",
        "manticorp",
        "mystery men",
        "gallows",
        "werewolf",
        "gargoyle",
        "chimera",
        "merman",
        "tross",
    ];

    for file in kernel_files {
        let source = fs::read_to_string(kernel_root.join(file)).unwrap();
        let lowercase_source = source.to_ascii_lowercase();
        for term in banned_lowercase {
            assert!(
                !lowercase_source.contains(term),
                "frozen kernel source {file} contains domain term {term}"
            );
        }
    }

    let neutral_kernel_manifest =
        fs::read_to_string(kernel_root.join("hollow-grove-kernel/Cargo.toml")).unwrap();
    assert!(!neutral_kernel_manifest.contains("flynt-constitution"));
    let application_manifest = fs::read_to_string(kernel_root.join("Cargo.toml")).unwrap();
    assert!(application_manifest.contains("flynt-constitution"));
    let layer_manifest = fs::read_to_string(layer_root.join("Cargo.toml")).unwrap();
    assert!(!layer_manifest.contains("hollow-grove = { path = \"..\" }"));
    assert!(layer_manifest.contains("hollow-grove-kernel"));
}
