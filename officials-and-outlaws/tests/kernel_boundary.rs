use std::fs;
use std::path::PathBuf;

#[test]
fn hollow_grove_kernels_have_no_flynt_constitution_dependency_or_terms() {
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
        "hollow-grove-recursion-kernel/src/pleb_meta/mod.rs",
        "hollow-grove-recursion-kernel/src/pleb_meta/sequence.rs",
        "hollow-grove-recursion-kernel/src/pleb_meta/mode.rs",
        "hollow-grove-recursion-kernel/src/pleb_meta/operator.rs",
        "hollow-grove-recursion-kernel/src/pleb_meta/route.rs",
        "hollow-grove-recursion-kernel/src/pleb_meta/bond.rs",
        "hollow-grove-recursion-kernel/src/pleb_meta/grammar.rs",
        "hollow-grove-recursion-kernel/src/pleb_meta/routing_pass.rs",
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

    let semantic_kernel_manifest =
        fs::read_to_string(kernel_root.join("hollow-grove-semantic-kernel/Cargo.toml")).unwrap();
    assert!(!semantic_kernel_manifest.contains("flynt-constitution"));
    let recursion_kernel_manifest =
        fs::read_to_string(kernel_root.join("hollow-grove-recursion-kernel/Cargo.toml")).unwrap();
    assert!(!recursion_kernel_manifest.contains("flynt-constitution"));
    assert!(!recursion_kernel_manifest.contains("serde"));
    let application_manifest = fs::read_to_string(kernel_root.join("Cargo.toml")).unwrap();
    assert!(application_manifest.contains("flynt-constitution"));
    let layer_manifest = fs::read_to_string(layer_root.join("Cargo.toml")).unwrap();
    assert!(!layer_manifest.contains("hollow-grove = { path = \"..\" }"));
    assert!(layer_manifest.contains("hollow-grove-semantic-kernel"));
}
