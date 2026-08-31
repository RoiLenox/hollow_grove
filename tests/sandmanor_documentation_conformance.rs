const CONSTITUTION: &str = include_str!("../SANDMANOR_CONSTITUTION_V2.md");
const AUDIT: &str = include_str!("../SANDMANOR_CONSTITUTIONAL_AUDIT_V2.md");
const SUPERSEDED_DRAFT: &str = include_str!("../SANDMANOR_CONSTITUTION_V1_DRAFT.md");
const AUTHORITY_MAP: &str = include_str!("../REPOSITORY_AUTHORITY_MAP.md");
const CORE: &str = include_str!("../HOLLOW_GROVE_CORE_v1.0.0.md");
const WORLD_CONTEXT: &str =
    include_str!("../CURRENT_SYNTHESIS_HOLLOW_GROVE_WORLD_CONTEXT_v0.1.0.md");
const CIVIC_BODY: &str = include_str!("../CIVIC_BODY_CORRESPONDENCE_V1.md");
const IMPLEMENTATION: &str = include_str!("../src/world/sandmanor.rs");
const FACULTIES: &str = include_str!("../HUEMAN_FACULTIES_V1.md");
const FACULTY_IMPLEMENTATION: &str = include_str!("../src/world/hueman_faculties.rs");
const RECIPE_IMPLEMENTATION: &str = include_str!("../src/synthesis_recipe.rs");
const INSTITUTIONS: &str = include_str!("../src/world/house_institutions.rs");
const HUEMAN: &str = include_str!("../artifacts/hueman_sandmanor_roles.md");
const VERTICAL_STACK: &str = include_str!("../artifacts/vertical_integration_stack.md");
const LINEAGE_LOCK: &str = include_str!("../SANDMANOR_LINEAGE_LOCK_v1.md");
const GUARDIAN_SUPPLEMENT: &str = include_str!("../SANDMANOR_GUARDIAN_AND_SUCCESSION_V1.md");
const SYNTHESIS_CANON: &str = include_str!("../HOLLOW_GROVE_SYNTHESIS_CANON_V1.md");
const KERNEL: &str = include_str!("../hollow-grove-semantic-kernel/src/lib.rs");

#[test]
fn constitution_preserves_all_forty_four_articles() {
    for heading in [
        "## Article I —",
        "## Article II —",
        "## Article III —",
        "## Article IV —",
        "## Article V —",
        "## Article VI —",
        "## Article VII —",
        "## Article VIII —",
        "## Article IX —",
        "## Article X —",
        "## Article XI —",
        "## Article XII —",
        "## Article XIII —",
        "## Article XIV —",
        "## Article XV —",
        "## Article XVI —",
        "## Article XVII —",
        "## Article XVIII —",
        "## Article XIX —",
        "## Article XX —",
        "## Article XXI —",
        "## Article XXII —",
        "## Article XXIII —",
        "## Article XXIV —",
        "## Article XXV —",
        "## Article XXVI —",
        "## Article XXVII —",
        "## Article XXVIII —",
        "## Article XXIX —",
        "## Article XXX —",
        "## Article XXXI —",
        "## Article XXXII —",
        "## Article XXXIII —",
        "## Article XXXIV —",
        "## Article XXXV —",
        "## Article XXXVI —",
        "## Article XXXVII —",
        "## Article XXXVIII —",
        "## Article XXXIX —",
        "## Article XL —",
        "## Article XLI —",
        "## Article XLII —",
        "## Article XLIII —",
        "## Article XLIV —",
    ] {
        assert!(CONSTITUTION.contains(heading), "missing {heading}");
    }
}

#[test]
fn additive_hueman_soul_law_is_exact_and_proof_bounded() {
    for term in [
        "Sandmanor is the Soul of the Hueman",
        "Minorian / Soul Interior / Prefog",
        "Minoan / Soul Exterior / Prefig",
        "Gnome → Faun → Minotaur",
        "Elf → Satyr → Centaur",
        "Husbandman → Householder → Hearthkeeper",
        "Beach Bum → Party Boy → Socialite",
        "The Sandman = Demiurge",
        "institution.sandmanor.sandmen",
    ] {
        assert!(CONSTITUTION.contains(term), "constitution omits {term}");
    }
    for term in [
        "Presynce recognizes present, material, embodied reality in relation to Identity.",
        "Resynce recognizes Capacity through response, change, pressure, or load.",
        "Precog recognizes trajectory or condition in relation to Integrity.",
        "Prefog recognizes Pattern that already exists but is obscured.",
        "Prefig recognizes or projects Pattern before manifestation.",
        "We Fairy Men",
        "The Gallows",
    ] {
        assert!(FACULTIES.contains(term), "faculty law omits {term}");
    }
    assert!(FACULTY_IMPLEMENTATION.contains("FacultyManifestation"));
    assert!(FACULTY_IMPLEMENTATION.contains("CURRENT_FORM_PRESYNCE_LADDER"));
    assert!(FACULTY_IMPLEMENTATION.contains("FACULTY_ARCHIVE_FORMAT"));
    assert!(RECIPE_IMPLEMENTATION.contains("faculty_manifestations"));
}

#[test]
fn canonical_people_office_syntheses_crime_and_palette_are_present() {
    for term in [
        "Sandmanor proves.",
        "Minorians",
        "Minoans",
        "Gnome → Faun → Minotaur",
        "Elf → Satyr → Centaur",
        "Geomancer",
        "Labyrinthian",
        "Hecaton",
        "Pegasus",
        "The Sandman = Demiurge",
        "The Sandman",
        "Contest of Improvement",
        "Fraudulent Design",
        "Husbandman → Householder → Hearthkeeper",
        "Beach Bum → Party Boy → Socialite",
        "Content Farm cultivates what a family has and makes",
        "Aura Field",
        "Aura Beach",
        "Current Sea",
        "#5A1F2A",
        "#A6404F",
        "#E07A86",
    ] {
        assert!(CONSTITUTION.contains(term), "constitution omits {term}");
    }
}

#[test]
fn unresolved_canon_is_explicitly_open_not_invented() {
    for term in [
        "exact appointment mechanism for Hecaton",
        "exact appointment mechanism for Pegasus",
        "succession and removal mechanics for Hecaton and Pegasus",
        "detailed Geomancer powers",
        "detailed Labyrinthian powers",
        "deeper biological or metaphysical mechanics",
    ] {
        assert!(
            CONSTITUTION.contains(term),
            "open-canon registry omits {term}"
        );
    }
}

#[test]
fn public_authority_surfaces_point_to_v2_and_exact_placements() {
    assert!(SUPERSEDED_DRAFT.contains("Status: historical redirect"));
    assert!(SUPERSEDED_DRAFT.contains("SANDMANOR_CONSTITUTION_V2.md"));
    assert!(AUTHORITY_MAP.contains("`SANDMANOR_CONSTITUTION_V2.md`"));
    assert!(AUTHORITY_MAP.contains("`SANDMANOR_CONSTITUTIONAL_AUDIT_V2.md`"));
    assert!(AUTHORITY_MAP.contains("`src/world/sandmanor.rs`"));
    assert!(CORE.contains("The compatible Contest of\nImprovement"));
    assert!(WORLD_CONTEXT.contains("Sandmanor proves through design"));
    assert!(CIVIC_BODY.contains("Gnome") && CIVIC_BODY.contains("Minotaur"));
    assert!(IMPLEMENTATION.contains("SandmanorRegistry"));
    assert!(IMPLEMENTATION.contains("SandmanAuthorityOrigin"));
    assert!(INSTITUTIONS.contains("sandmanor::sandman_office_id()"));
    assert!(INSTITUTIONS.contains("sandmanor::proof_civilization_id()"));
}

#[test]
fn audit_maps_authority_regional_law_and_legacy_to_executable_surfaces() {
    for term in [
        "office.sandmanor.sandman",
        "institution.sandmanor.sandmen",
        "role.sandmanor.minorian",
        "role.sandmanor.minoan",
        "Gnome → Faun → Minotaur",
        "Elf → Satyr → Centaur",
        "Hecaton",
        "Pegasus",
        "tests/sandmanor_constitutional_architecture.rs",
        "tests/sandmanor_documentation_conformance.rs",
        "src/bin/sandmanor_constitutional_audit.rs",
        "universal recursion kernel",
    ] {
        assert!(AUDIT.contains(term), "audit omits {term}");
    }
}

#[test]
fn generated_hueman_projection_is_presentation_only_and_current() {
    for term in [
        "Minorians are Sandmanor's Prefog / Hearth / interior-family people",
        "Minoans are Sandmanor's Prefig / Current Sea / outward-social people",
        "Gnome -> Faun -> Minotaur",
        "Elf -> Satyr -> Centaur",
        "failure is evidence and remains recorded",
        "Contest of Improvement",
        "The Sandman = Demiurge",
        "Hecaton is the Minorian Hand",
        "Fraudulent Design is Sandmanor's signature constitutional offense",
        "Sandmanor is the Hueman Soul",
        "Prefog -> Prefig -> Proof -> Evidence or Failure -> Revision -> Prefog",
        "only the existing proof lifecycle may advance a Prefig source",
        "faculties orthogonal to Current, Aura, Synthesis, and institutional rank",
        "Hueman and Godot may present Sandmanor records but may not create proof",
        "the universal recursion kernel remains isolated",
    ] {
        assert!(HUEMAN.contains(term), "Hueman projection omits {term}");
    }
    assert!(VERTICAL_STACK.contains("evidence-judged Contest of Improvement"));
}

#[test]
fn active_surfaces_contain_no_obsolete_sandmanor_authority_law() {
    for (name, document) in [
        ("constitution", CONSTITUTION),
        ("authority map", AUTHORITY_MAP),
        ("core", CORE),
        ("implementation", IMPLEMENTATION),
        ("institutions", INSTITUTIONS),
        ("Hueman projection", HUEMAN),
        ("vertical stack", VERTICAL_STACK),
        ("lineage lock", LINEAGE_LOCK),
        ("guardian supplement", GUARDIAN_SUPPLEMENT),
        ("Synthesis canon", SYNTHESIS_CANON),
        ("world context", WORLD_CONTEXT),
    ] {
        for obsolete in [
            "Sandmanor roles are descriptive-only for now",
            "no contest resolver or crowd AI is active",
            "crowd-judged Sandman contest",
            "Gnomes do not evolve through a formal ladder",
            "CrowdRecognition",
            "Sandmen are the people and witness body",
            "Husbandman + Geomancer → Gnome",
            "Warden + Witch → Elf",
            "Gnome → Minotaur → Hecaton",
            "Elf → Centaur → Pegasus",
            "The Transcendent is Sandmanor's absolute apex",
            "Beach Bum → Party Boy → Influencer",
            "Gnome = Geomancer",
            "Minotaur = Labyrinthian",
            "Hecaton = Demiurge",
            "Gnome → Elf",
            "Minorian → Minoan",
            "Geomancer → Labyrinthian → Demiurge",
        ] {
            assert!(!document.contains(obsolete), "{name} contains `{obsolete}`");
        }
    }
}

#[test]
fn sandmanor_specific_law_remains_out_of_the_recursion_kernel() {
    for forbidden in [
        "SANDMANOR_CONSTITUTION_V2",
        "ContestOfImprovement",
        "Fraudulent Design",
        "SandmanorRegistry",
        "world::sandmanor",
        "HuemanFaculty",
        "Prefog",
        "Prefig",
    ] {
        assert!(!KERNEL.contains(forbidden), "kernel contains {forbidden}");
    }
}
