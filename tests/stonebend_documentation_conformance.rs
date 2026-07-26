const CONSTITUTION: &str = include_str!("../STONEBEND_CONSTITUTION_V2.md");
const SUPERSEDED_DRAFT: &str = include_str!("../STONEBEND_CONSTITUTION_V1_DRAFT.md");
const AUTHORITY_MAP: &str = include_str!("../REPOSITORY_AUTHORITY_MAP.md");
const WORLD_CONTEXT: &str =
    include_str!("../CURRENT_SYNTHESIS_HOLLOW_GROVE_WORLD_CONTEXT_v0.1.0.md");
const CORE: &str = include_str!("../HOLLOW_GROVE_CORE_v1.0.0.md");
const IMPLEMENTATION: &str = include_str!("../src/world/stonebend.rs");
const INSTITUTION_PROJECTION: &str = include_str!("../src/world/house_institutions.rs");
const HUEMAN_PROJECTION: &str = include_str!("../artifacts/hueman_stonebend_roles.md");
const KERNEL: &str = include_str!("../hollow-grove-kernel/src/lib.rs");

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
    assert!(WORLD_CONTEXT.contains("Proliteriate — the permanent distributed public network"));
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
        "the Proliteriate is Stonebend's permanent distributed public network",
        "the High Freemason is the singular office",
        "Illegal Hollowing is Stonebend's signature constitutional offense",
        "Stonebend has exactly three bidirectional constitutional gates",
        "Diamond is Stonebend's continuing sovereign Title",
        "no Hypergiant claims Diamond without The Lazerhorn",
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
