const CONSTITUTION: &str = include_str!("../GLAUSHOUSE_CONSTITUTION_V2.md");
const AUDIT: &str = include_str!("../GLAUSHOUSE_CONSTITUTIONAL_AUDIT_V2.md");
const SUPERSEDED_DRAFT: &str = include_str!("../GLAUSHOUSE_CONSTITUTION_V1_DRAFT.md");
const AUTHORITY_MAP: &str = include_str!("../REPOSITORY_AUTHORITY_MAP.md");
const CORE: &str = include_str!("../HOLLOW_GROVE_CORE_v1.0.0.md");
const WORLD_CONTEXT: &str =
    include_str!("../CURRENT_SYNTHESIS_HOLLOW_GROVE_WORLD_CONTEXT_v0.1.0.md");
const IMPLEMENTATION: &str = include_str!("../src/world/glaushouse.rs");
const INSTITUTIONS: &str = include_str!("../src/world/house_institutions.rs");
const HUEMAN: &str = include_str!("../artifacts/hueman_glaushouse_roles.md");
const COMPROMISE: &str = include_str!("../HOLLOW_GROVE_COMPROMISE_V1_DRAFT.md");
const HUEMAN_CONSTITUTION: &str = include_str!("../HUEMAN_v0.1.0.md");
const FLYNT: &str = include_str!("../FLYNT_CONSTITUTION_V2.md");
const MANTICORP: &str = include_str!("../FLYNT_DUAL_LEADERSHIP_AND_MANTICORP_RECIPE_V1.md");
const POWER_RECIPE: &str = include_str!("../HOLLOW_GROVE_POWER_RECIPE_CONSTITUTION_V1.md");
const WORLD_CONTEXT_PROJECTION: &str =
    include_str!("../artifacts/current_synthesis_world_context.md");
const KERNEL: &str = include_str!("../hollow-grove-kernel/src/lib.rs");

#[test]
fn constitution_preserves_all_thirty_articles() {
    for article in [
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
        "Article XXII",
        "Article XXIII",
        "Article XXIV",
        "Article XXV",
        "Article XXVI",
        "Article XXVII",
        "Article XXVIII",
        "Article XXIX",
        "Article XXX",
    ] {
        assert!(CONSTITUTION.contains(article), "missing {article}");
    }
}

#[test]
fn canonical_maxims_authorities_institutions_and_palette_are_present() {
    for term in [
        "Glaüshouse clears.",
        "Diagnosis identifies condition.",
        "Consent authorizes participation.",
        "Clearance permits procedure.",
        "Care preserves the subject.",
        "Synthesis transforms.",
        "Recovery completes the act.",
        "Illegal Synthesis",
        "Doctor Ratchet is its frozen current",
        "Nurse House is a frozen current Persephone identity",
        "The Nightingales",
        "The Matron reads the patient.",
        "The Marshal holds the patient.",
        "Persephone preserves the whole patient.",
        "Synthesis is not permanence. Synthesis is Continuance through renewal.",
        "Synthesis is medicine first and transformation second.",
        "Technical viability plus lived viability equals lawful Synthesis.",
        "Glauspitals",
        "Chromacord",
        "Swanshee",
        "#163C35",
        "#2F7A62",
        "#78C9A4",
        "Grip",
        "Show",
        "Grit",
    ] {
        assert!(CONSTITUTION.contains(term), "constitution omits {term}");
    }
}

#[test]
fn public_authority_surfaces_point_to_v2_and_exact_placements() {
    assert!(SUPERSEDED_DRAFT.contains("Status: historical redirect"));
    assert!(SUPERSEDED_DRAFT.contains("GLAUSHOUSE_CONSTITUTION_V2.md"));
    assert!(AUTHORITY_MAP.contains("`GLAUSHOUSE_CONSTITUTION_V2.md`"));
    assert!(AUTHORITY_MAP.contains("`GLAUSHOUSE_CONSTITUTIONAL_AUDIT_V2.md`"));
    assert!(AUTHORITY_MAP.contains("`src/world/glaushouse.rs`"));
    assert!(CORE.contains("The Nightingales — the universal clinical foundation"));
    assert!(WORLD_CONTEXT.contains("constitutional nursing and clinical-care"));
    assert!(IMPLEMENTATION.contains("SingularHighestClinicalOffice"));
    assert!(IMPLEMENTATION.contains("UniversalClinicalFoundation"));
    assert!(IMPLEMENTATION.contains("EqualAuraForwardBranch"));
    assert!(IMPLEMENTATION.contains("EqualCurrentForwardBranch"));
    assert!(IMPLEMENTATION.contains("MultipleBalancedRank"));
    assert!(IMPLEMENTATION.contains("institution.glaushouse.glauspitals"));
    assert!(IMPLEMENTATION.contains("institution.glaushouse.chromacord"));
    assert!(IMPLEMENTATION.contains("institution.glaushouse.nightingales"));
    assert!(INSTITUTIONS.contains("glaushouse::glauspitals_id()"));
    assert!(INSTITUTIONS.contains("glaushouse::chromacord_id()"));
    assert!(INSTITUTIONS.contains("glaushouse::nightingales_id()"));
}

#[test]
fn audit_maps_each_authority_and_legacy_transition_to_executable_surfaces() {
    for term in [
        "office.glaushouse.prima-donna",
        "role.glaushouse.persephone",
        "role.glaushouse.matron",
        "role.glaushouse.marshal",
        "institution.glaushouse.nightingales",
        "institution.glaushouse.glauspitals",
        "institution.glaushouse.chromacord",
        "PublicClearance",
        "FinalJudgmentAnswerability",
        "tests/glaushouse_constitutional_architecture.rs",
        "src/bin/glaushouse_constitutional_audit.rs",
        "universal recursion kernel",
    ] {
        assert!(AUDIT.contains(term), "audit omits {term}");
    }
}

#[test]
fn generated_hueman_projection_is_presentation_only_and_current() {
    for term in [
        "Doctor Ratchet is its frozen current holder identity",
        "Nurse House is one frozen current Persephone identity",
        "The Nightingales are the universal clinical foundation",
        "Matron and Marshal are equal",
        "multiple Persephones may serve",
        "Synthesis is Continuance through renewal",
        "Glauspitals operates clinical facilities",
        "Chromacord preserves clinical records and evidence",
        "Illegal Synthesis is Glaushouse's signature constitutional offense",
        "Hueman and Godot may present records but may not create consent",
        "the universal recursion kernel remains isolated",
    ] {
        assert!(HUEMAN.contains(term), "Hueman projection omits {term}");
    }
}

#[test]
fn active_surfaces_contain_no_obsolete_glaushouse_authority_law() {
    for (name, document) in [
        ("constitution", CONSTITUTION),
        ("authority map", AUTHORITY_MAP),
        ("core", CORE),
        ("implementation", IMPLEMENTATION),
        ("institutions", INSTITUTIONS),
        ("Hueman projection", HUEMAN),
        ("world-context projection", WORLD_CONTEXT_PROJECTION),
        ("Flynt constitution", FLYNT),
        ("Manticorp constitution", MANTICORP),
    ] {
        for obsolete in [
            "no court resolver or succession engine is active",
            "Glaushouse roles are descriptive-only for now",
            "Nightingales: Glaushouse white-blood-cell civic people studied",
            "singular Persephone",
            "Persephone is the singular",
            "office.glaushouse.persephone",
            "permanent Synthesis Form",
            "permanent Form beyond Chimera",
        ] {
            assert!(!document.contains(obsolete), "{name} contains `{obsolete}`");
        }
    }
}

#[test]
fn continuity_ladder_succession_and_gremlin_way_are_projected_across_canon() {
    for (name, document) in [
        ("constitution", CONSTITUTION),
        ("compromise", COMPROMISE),
        ("Hueman constitution", HUEMAN_CONSTITUTION),
    ] {
        for term in [
            "Nightingale",
            "Matron",
            "Marshal",
            "Persephone",
            "Prima Donna",
            "Continuance",
            "maintenance",
            "renewal",
        ] {
            assert!(document.contains(term), "{name} omits {term}");
        }
    }
    assert!(CONSTITUTION.contains(
        "Stonebend prepares three offices to remove one leader. Glaüshouse requires one leader to prepare many people to rise."
    ));
    assert!(MANTICORP.contains("Gremlincoin = the Gremlin Way."));
    assert!(MANTICORP.contains("Gargoyle maintenance and renewal"));
    assert!(FLYNT.contains("presently maintained living holder"));
    assert!(FLYNT.contains("Manticorp Form distinct from the Manticorp institution"));
    assert!(POWER_RECIPE.contains("host rejection / Sympiote rejection"));
    assert!(
        POWER_RECIPE
            .contains("Is the Sympiote integrating with the Hueman, or replacing the Hueman?")
    );
    assert_eq!(WORLD_CONTEXT, WORLD_CONTEXT_PROJECTION);
}

#[test]
fn glaushouse_specific_law_remains_out_of_the_recursion_kernel() {
    for forbidden in [
        "GLAUSHOUSE_CONSTITUTION_V2",
        "PrimaDonna",
        "Persephone",
        "Nightingales",
        "Illegal Synthesis",
        "world::glaushouse",
    ] {
        assert!(!KERNEL.contains(forbidden), "kernel contains {forbidden}");
    }
}
