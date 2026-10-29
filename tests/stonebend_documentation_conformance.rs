const CONSTITUTION: &str = include_str!("../STONEBEND_CONSTITUTION_V2.md");
const SUPERSEDED_DRAFT: &str = include_str!("../STONEBEND_CONSTITUTION_V1_DRAFT.md");
const AUTHORITY_MAP: &str = include_str!("../REPOSITORY_AUTHORITY_MAP.md");
const WORLD_CONTEXT: &str =
    include_str!("../CURRENT_SYNTHESIS_HOLLOW_GROVE_WORLD_CONTEXT_v0.1.0.md");
const CORE: &str = include_str!("../HOLLOW_GROVE_CORE_v1.0.0.md");
const IMPLEMENTATION: &str = include_str!("../src/world/stonebend.rs");
const INSTITUTION_PROJECTION: &str = include_str!("../src/world/house_institutions.rs");
const HUEMAN_PROJECTION: &str = include_str!("../artifacts/hueman_stonebend_roles.md");
const KERNEL: &str = include_str!("../hollow-grove-semantic-kernel/src/lib.rs");

#[test]
fn constitution_preserves_all_twenty_one_articles() {
    let articles = [
        "Article I",
        "Article II",
        "Article III",
        "Article IV",
        "Article V",
        "Article VI",
        "Article VII",
        "Article VIII",
        "Article IX",
        "Article X",
        "Article XI",
        "Article XII",
        "Article XIII",
        "Article XIV",
        "Article XV",
        "Article XVI",
        "Article XVII",
        "Article XVIII",
        "Article XIX",
        "Article XX",
        "Article XXI",
    ];
    for article in articles {
        assert!(CONSTITUTION.contains(article), "missing {article}");
    }
}

#[test]
fn canonical_maxims_and_institutions_are_exact() {
    for term in [
        "Stonebend names.",
        "A Name identifies.",
        "A Title authorizes.",
        "A Mirror verifies.",
        "A Seal endures.",
        "Geralds",
        "Hypergiant",
        "Proliteriate",
        "High Freemason",
        "Illegal Hollowing",
        "Basin Motor Speedway",
        "Mt. Aura",
    ] {
        assert!(CONSTITUTION.contains(term), "constitution omits {term}");
    }
    for term in [
        "STONEBEND_GOVERNING_VERB",
        "Illegal Hollowing",
        "Hypergiant",
        "Proliteriate",
        "High Freemason",
    ] {
        assert!(
            IMPLEMENTATION.contains(term) || INSTITUTION_PROJECTION.contains(term),
            "implementation omits {term}"
        );
    }
}

#[test]
fn public_authority_documents_point_to_v2() {
    assert!(SUPERSEDED_DRAFT.contains("Status: historical redirect"));
    assert!(SUPERSEDED_DRAFT.contains("STONEBEND_CONSTITUTION_V2.md"));
    assert!(AUTHORITY_MAP.contains("`STONEBEND_CONSTITUTION_V2.md`"));
    assert!(AUTHORITY_MAP.contains("`src/world/stonebend.rs`"));
    assert!(WORLD_CONTEXT.contains("Proliteriate — the elected professional office"));
    assert!(CORE.contains("Hypergiant — the singular highest office"));
}

#[test]
fn active_stonebend_surfaces_contain_no_obsolete_peer_role_law() {
    for (name, document) in [
        ("constitution", CONSTITUTION),
        ("authority map", AUTHORITY_MAP),
        ("world context", WORLD_CONTEXT),
        ("core", CORE),
        ("implementation", IMPLEMENTATION),
        ("institution projection", INSTITUTION_PROJECTION),
        ("Hueman projection", HUEMAN_PROJECTION),
    ] {
        for obsolete in [
            "Proletariat",
            "not a higher authority than the others",
            "either office may take the Crown",
            "Title, Labor, and Craft balance",
        ] {
            assert!(!document.contains(obsolete), "{name} contains `{obsolete}`");
        }
    }
}

#[test]
fn generated_hueman_projection_is_presentation_only_and_current() {
    for term in [
        "the Geralds are Stonebend's constitutional people",
        "the Hypergiant is the singular highest Stonebend office",
        "the Proliteriate is one elected professional office",
        "the High Freemason seal belongs to the single elected Freemason",
        "Illegal Hollowing is Stonebend's signature constitutional offense",
        "Stonebend has exactly three bidirectional constitutional gates",
        "Hypergiant is Stonebend's sovereign title",
        "Diamond's current role is STATUS: OPEN",
        "The Acclamation is the intended sovereign selection",
        "Godot and Hueman artifacts are presentation only",
    ] {
        assert!(HUEMAN_PROJECTION.contains(term), "projection omits {term}");
    }
}

#[test]
fn stonebend_remains_out_of_the_recursion_kernel() {
    for forbidden in [
        "STONEBEND_CONSTITUTION_V2",
        "Proliteriate",
        "Hypergiant",
        "Illegal Hollowing",
        "world::stonebend",
    ] {
        assert!(!KERNEL.contains(forbidden));
    }
}

#[test]
fn contemporary_design_and_role_projection_exclude_retired_accession_name() {
    for (name, document) in [
        (
            "constitution",
            include_str!("../STONEBEND_CONSTITUTION_V2.md"),
        ),
        (
            "Rev100",
            include_str!("../STONEBEND_REV100_GOVERNANCE_AMENDMENT.md"),
        ),
        ("core", include_str!("../HOLLOW_GROVE_CORE_v1.0.0.md")),
        (
            "context",
            include_str!("../CURRENT_SYNTHESIS_HOLLOW_GROVE_WORLD_CONTEXT_v0.1.0.md"),
        ),
        (
            "context mirror",
            include_str!("../artifacts/current_synthesis_world_context.md"),
        ),
        ("roles", HUEMAN_PROJECTION),
        (
            "authority map",
            include_str!("../REPOSITORY_AUTHORITY_MAP.md"),
        ),
        (
            "semantic summary",
            include_str!("../HOLLOW_GROVE_SEMANTIC_FOUNDATION_V1.md"),
        ),
        (
            "capabilities",
            include_str!("../V2_CAPABILITY_INVENTORY.md"),
        ),
        (
            "capability matrix",
            include_str!("../V2_CAPABILITY_MATRIX.md"),
        ),
        ("artifact index", include_str!("../artifacts/index.md")),
        (
            "geography annex",
            include_str!(
                "../docs/STONEBEND_FLYNT_CORRIDOR_HIPPO_DOME_HIPPODRONES_GEOGRAPHY_SUPPLEMENT_V2_2026-10-26.md"
            ),
        ),
    ] {
        assert!(!document.to_lowercase().contains("lazerhorn"), "{name}");
    }
    let generated = hollow_grove::hueman_support::build_hueman_stonebend_roles_from_artifacts(
        "start", "fourway",
    );
    assert!(!generated.to_lowercase().contains("lazerhorn"));
    assert!(generated.contains("The Acclamation is the intended sovereign selection"));
    assert!(generated.contains("H.T.T. — Highway to Hell / S.T.H. — Stairway to Heaven"));
}
