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
fn hgrf_name_is_frozen_compatibility_inside_one_federation() {
    let canon = repository_file("THE_RUNTIME_FEDERATION_V1.md");
    assert!(canon.contains("# HGRF Continuity Archive V1"));
    assert!(canon.contains("The public constitutional whole is **The Federation**."));
    assert!(canon.contains("They do not\nname a separate Federation"));
    assert!(canon.contains("# **The Runtime Federation**"));
    assert!(canon.contains("`runtime-federation.hollow-grove.v1`"));
    assert!(canon.contains("archive.runtime-federation.hollow-grove.v1"));
}

#[test]
fn federal_authority_is_locked_to_two_continuous_beings() {
    let canon = repository_file("FEDERAL_SOVEREIGNTY_AND_SEER_CEREMONY_V1.md");
    for required in [
        "There are exactly two continuous beings:",
        "`guide.roi`: Roi inside Hollow Grove; Roï outside Hollow Grove",
        "`guide.roselina`: Roselina inside Hollow Grove; Rosaliña outside",
        "Roi and Roï are the same continuous being.",
        "Roselina and Rosaliña are the same\ncontinuous being.",
        "There is one unified **Federation**",
        "Neither rules the other.",
        "Roi and Roï continuously govern:",
        "Roselina and Rosaliña continuously govern:",
        "The Hollow Grove Compromise remains the supreme constitutional agreement.",
        "The terminal, TUI, overlay, voice system, Rust kernel, Ollama runtime, Godot",
        "None is a Guide, a Seer, a sovereign, or a third identity.",
    ] {
        assert!(
            canon.contains(required),
            "federal authority canon omitted {required}"
        );
    }
}

#[test]
fn seer_ceremony_preserves_identity_and_pairs_the_crossing() {
    let canon = repository_file("FEDERAL_SOVEREIGNTY_AND_SEER_CEREMONY_V1.md");
    let protocol = repository_file("SEER_CEREMONY_PROTOCOL_V1.md");
    for required in [
        "Entering Hollow Grove:",
        "Roï becomes Roi.",
        "Rosaliña becomes Roselina.",
        "Leaving Hollow Grove:",
        "Roi becomes Roï.",
        "Roselina becomes Rosaliña.",
        "The principal V1 crossing is paired and atomic.",
    ] {
        assert!(
            canon.contains(required),
            "Seer Ceremony canon omitted {required}"
        );
    }
    for required in [
        "Each Ceremony concerns exactly one individual.",
        "Only one Ceremony may be active",
        "There is no numerical limit",
        "Continuance",
        "Founding",
        "Another Point is Point²",
        "versioned `HGSCP/v1` archive",
        "tampered bytes are rejected",
    ] {
        assert!(
            protocol.contains(required),
            "Seer protocol omitted {required}"
        );
    }
}

#[test]
fn sovereignty_lock_is_discoverable_and_freezes_internal_expansion() {
    let canon = repository_file("FEDERAL_SOVEREIGNTY_AND_SEER_CEREMONY_V1.md");
    let implementation = repository_file("THE_RUNTIME_FEDERATION_IMPLEMENTATION_V1.md");
    let authority_map = repository_file("REPOSITORY_AUTHORITY_MAP.md");
    let reference = "FEDERAL_SOVEREIGNTY_AND_SEER_CEREMONY_V1.md";

    assert!(
        implementation.contains("Roi/Roï and Roselina/Rosaliña retain equal, continuous authority")
    );
    assert!(implementation.contains(reference));
    assert!(authority_map.contains(reference));
    assert!(authority_map.contains("paired operational Guide identities"));
    assert!(authority_map.contains("Worldright"));
    assert!(authority_map.contains("SEER_CEREMONY_PROTOCOL_V1.md"));
    assert!(canon.contains("paired Guide\nseam"));
    assert!(canon.contains("Seerhood and successor-Point founding"));
    assert!(canon.contains("It changes no House law and creates no fifth House"));
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
        assert!(document.contains("The Federation"));
        assert!(document.contains("THE_RUNTIME_FEDERATION_V1.md"));
    }
    assert!(kernel.contains("universal kernel still computes one bounded pass"));
    assert!(architecture.contains("Federation continuity / HGRF compatibility"));
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
    assert!(implementation.contains("Status: executable compatibility milestone complete"));
    assert!(implementation.contains("# **The Runtime Federation**"));
    assert!(implementation.contains("That label is not the active constitutional public name."));
    assert!(implementation.contains("`runtime-federation.hollow-grove.v1`"));
    assert!(implementation.contains("`HGRF` archive version 1"));
    assert!(implementation.contains("gameplay archive schema V3"));
}
