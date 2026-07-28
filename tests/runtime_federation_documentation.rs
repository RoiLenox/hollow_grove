use std::fs;
use std::path::{Path, PathBuf};

fn repository_file(path: &str) -> String {
    fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join(path))
        .unwrap_or_else(|error| panic!("failed to read {path}: {error}"))
}

fn rust_sources(root: &Path) -> Vec<PathBuf> {
    let mut pending = vec![root.to_path_buf()];
    let mut result = Vec::new();
    while let Some(path) = pending.pop() {
        for entry in fs::read_dir(path).expect("kernel source directory") {
            let path = entry.expect("kernel source entry").path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().and_then(|value| value.to_str()) == Some("rs") {
                result.push(path);
            }
        }
    }
    result.sort();
    result
}

#[test]
fn runtime_federation_has_one_exact_canonical_name_and_identity() {
    let canon = repository_file("THE_RUNTIME_FEDERATION_V1.md");
    assert!(canon.contains("# The Runtime Federation V1"));
    assert!(canon.contains("# **The Runtime Federation**"));
    assert!(canon.contains("`runtime-federation.hollow-grove.v1`"));
    assert!(canon.contains("archive.runtime-federation.hollow-grove.v1"));
    assert!(canon.contains("\nHGRF\n"));
}

#[test]
fn runtime_federation_is_continuity_without_new_sovereignty() {
    let canon = repository_file("THE_RUNTIME_FEDERATION_V1.md");
    for required in [
        "a fifth House",
        "a second Hollow Grove Constitution",
        "a new universal kernel",
        "permission for one archive to rewrite another archive's history",
        "Federation participation grants no House",
    ] {
        assert!(
            canon.contains(required),
            "Runtime Federation canon omitted {required}"
        );
    }
}

#[test]
fn kernel_facing_cycle_is_exact_and_continuous() {
    let canon = repository_file("THE_RUNTIME_FEDERATION_V1.md");
    let cycle = "The Way Back\n    → The Initiation\n    → The Gathering\n    → The Festival\n    → The Way Back";
    assert!(canon.contains(cycle));
    assert!(canon.contains(
        "Return is re-entry with consequence, not restoration to an untouched starting\nstate."
    ));
    assert!(canon.contains(
        "Only accepted domain events may alter canonical state. Rejected attempts may\nretain evidence and history but cannot falsely produce their intended result."
    ));
}

#[test]
fn kernel_and_architecture_documents_name_the_same_boundary() {
    let kernel = repository_file("KERNEL_v0.1.2.md");
    let architecture = repository_file("HOLLOW_GROVE_CONSTITUTIONAL_ARCHITECTURE_V1.md");
    let foundation = repository_file("HOLLOW_GROVE_SEMANTIC_FOUNDATION_V1.md");
    for document in [&kernel, &architecture, &foundation] {
        assert!(document.contains("The Runtime Federation"));
        assert!(document.contains("THE_RUNTIME_FEDERATION_V1.md"));
    }
    assert!(kernel.contains("universal kernel still computes one bounded pass"));
    assert!(architecture.contains("canonically established; implementation milestone frozen"));
}

#[test]
fn federation_canon_keeps_domain_reducers_and_archives_distinct() {
    let canon = repository_file("THE_RUNTIME_FEDERATION_V1.md");
    for law in [
        "Law of retained identity",
        "Law of domain custody",
        "Law of causal reference",
        "Law of accepted consequence",
        "Law of exact replay",
        "Law of contradiction",
        "Law of canonical order",
        "Law of migration",
        "Law of presentation non-authority",
        "Law of constitutional non-sovereignty",
    ] {
        assert!(canon.contains(law), "missing {law}");
    }
    assert!(canon.contains(
        "federation archive binds components; it does not flatten them into one\nambiguous event enum."
    ));
}

#[test]
fn universal_kernel_contains_no_runtime_federation_dependency() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("hollow-grove-kernel/src");
    for source in rust_sources(&root) {
        let text = fs::read_to_string(&source).expect("kernel source");
        for forbidden in [
            "RuntimeFederation",
            "Runtime Federation",
            "runtime-federation",
            "HGRF",
            "Central Junction",
            "Function Junction",
            "Stonebend",
            "Sandmanor",
            "Glaüshouse",
            "Glaushouse",
            "Flynt",
        ] {
            assert!(
                !text.contains(forbidden),
                "{} imported forbidden Hollow Grove runtime lore: {forbidden}",
                source.display()
            );
        }
    }
}

#[test]
fn implementation_claim_points_to_the_separate_executable_milestone() {
    let canon = repository_file("THE_RUNTIME_FEDERATION_V1.md");
    let implementation = repository_file("THE_RUNTIME_FEDERATION_IMPLEMENTATION_V1.md");
    assert!(canon.contains(
        "The separate\nexecutable milestone is recorded in\n`THE_RUNTIME_FEDERATION_IMPLEMENTATION_V1.md`."
    ));
    assert!(implementation.contains("Status: executable milestone complete"));
    assert!(implementation.contains("# **The Runtime Federation**"));
    assert!(implementation.contains("`runtime-federation.hollow-grove.v1`"));
    assert!(implementation.contains("`HGRF` archive version 1"));
    assert!(implementation.contains("gameplay archive schema V3"));
}
