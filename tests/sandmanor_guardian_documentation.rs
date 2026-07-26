const CONSTITUTION: &str = include_str!("../SANDMANOR_CONSTITUTION_V2.md");
const SUPPLEMENT: &str = include_str!("../SANDMANOR_GUARDIAN_AND_SUCCESSION_V1.md");
const COMPROMISE: &str = include_str!("../HOLLOW_GROVE_COMPROMISE_V1_DRAFT.md");
const GEOGRAPHY: &str = include_str!("../HOLLOW_GROVE_CONSTITUTIONAL_GEOGRAPHY_V2.md");
const WORLD: &str = include_str!("../CURRENT_SYNTHESIS_HOLLOW_GROVE_WORLD_CONTEXT_v0.1.0.md");
const AURA_FIELD: &str = include_str!("../AURA_FIELD_SURFACE_V1.md");
const AURA_BEACH: &str = include_str!("../AURA_BEACH_SURFACE_V1.md");
const AUTHORITY_MAP: &str = include_str!("../REPOSITORY_AUTHORITY_MAP.md");
const CAPABILITIES: &str = include_str!("../V2_CAPABILITY_INVENTORY.md");
const IMPLEMENTATION: &str = include_str!("../src/world/sandmanor/milestone.rs");
const PROJECTION: &str = include_str!("../artifacts/hueman_sandmanor_roles.md");
const KERNEL: &str = include_str!("../hollow-grove-kernel/src/lib.rs");

#[test]
fn all_required_canonical_maxims_are_ratified() {
    for maxim in [
        "The Minorian asks how a place is lived within.",
        "The Minoan asks how a place lives within the world.",
        "The Minorians cultivate what enters a life.",
        "The Minoans guard where life meets the world.",
        "Interior gives the world intimacy. Exterior gives intimacy a world.",
        "Food feeds the Body. Content feeds the Mind and Aura.",
        "A field can be overworked. So can an audience.",
        "The Gnome tends the field. The Minotaur answers for it.",
        "The Minotaur guards a field. Hecaton keeps the fields from failing one another.",
        "The Elf watches the shore. The Centaur answers when the shore breaks.",
        "The Centaur guards the beach. Pegasus guards the horizon.",
        "The Form remains a body. The mantle remains a trust.",
        "The Minorian learns to open the gate. The Minoan learns to tend what lies behind it.",
        "The crowd measures change, not rank.",
        "Long ago, he was called Aegon. Today, he is known as the Sandman.",
        "The Minoan County Courthouse determines what must answer to the law. Glaüshouse determines what can be repaired afterward.",
    ] {
        assert!(SUPPLEMENT.contains(maxim), "missing maxim: {maxim}");
    }
}

#[test]
fn farm_and_content_health_distinction_reach_public_canon() {
    for document in [SUPPLEMENT, AURA_FIELD, WORLD] {
        assert!(document.contains("Aura Fields"));
        assert!(document.contains("Content Farm"));
    }
    assert!(SUPPLEMENT.contains("Content production is not inherently corrupt"));
    assert!(IMPLEMENTATION.contains("ContentFarmAssessment"));
    assert!(IMPLEMENTATION.contains("is_exploitative"));
    assert!(IMPLEMENTATION.contains("is_healthy"));
}

#[test]
fn coast_courthouse_and_southern_law_order_are_exact() {
    let sequence = "Free Aura Beach\n→ Southern Coast\n→ Current Break\n→ Minoan County Courthouse\n→ Glaüshouse";
    for document in [SUPPLEMENT, GEOGRAPHY, AURA_BEACH] {
        assert!(document.contains(sequence), "coastal order drifted");
    }
    assert!(SUPPLEMENT.contains("Southern Law"));
    assert!(SUPPLEMENT.contains("neither a fifth\nHouse"));
    assert!(SUPPLEMENT.contains("The name County does not impose a world-wide county system"));
}

#[test]
fn current_break_never_transfers_manticorp_command() {
    for document in [SUPPLEMENT, COMPROMISE, AURA_BEACH, WORLD] {
        assert!(document.contains("Current Break"));
        assert!(document.contains("Manticorp"));
        assert!(document.contains("Flynt"));
    }
    assert!(
        SUPPLEMENT.contains("Current Break never creates, owns, or commands a second Manticorp")
    );
    assert!(IMPLEMENTATION.contains("creates_second_manticorp"));
    assert!(IMPLEMENTATION.contains("command_house != House::Flynt"));
}

#[test]
fn all_guardian_forms_and_mantles_are_public_and_distinct() {
    for term in [
        "Gnome → Minotaur → Hecaton",
        "Elf → Centaur → Pegasus",
        "GUARDIAN OF THE FIELDS",
        "GUARDIAN OF THE WHOLE FARM",
        "GUARDIAN OF THE BEACH",
        "GUARDIAN OF THE HORIZON",
        "Qualification, Recipe, Synthesis Form, mantle, jurisdiction, and present",
        "Loss of mantle and physical regression remain distinct events",
    ] {
        assert!(SUPPLEMENT.contains(term), "missing guardian law: {term}");
    }
    for term in [
        "GuardianQualification",
        "GuardianRecipeAuthorization",
        "GuardianSynthesisRecord",
        "GuardianInvestiture",
        "GuardianAuthorityState",
        "SynthesisContinuance",
        "SynthesisLifecycle",
    ] {
        assert!(IMPLEMENTATION.contains(term), "missing schema: {term}");
    }
}

#[test]
fn contest_trials_crowd_and_teaching_integrity_are_explicit() {
    for term in [
        "Aura Field Trial",
        "Content Farm Trial",
        "Liberty and Hospitality Trial",
        "Rescue and Horizon Trial",
        "Reciprocal Integration Trial",
        "own published baseline",
        "The crowd is the constitutional judge of improvement",
        "One recognized judgment may count per eligible stable identity",
        "A valid tie produces no arbitrary Sandman",
        "Teaching Integrity",
    ] {
        assert!(
            SUPPLEMENT.contains(term) || CONSTITUTION.contains(term),
            "missing Contest law: {term}"
        );
    }
    for term in [
        "TrialDomain",
        "CandidateBaseline",
        "ImprovementEvidence",
        "TeachingIntegrityFinding",
        "CrowdJudgment",
        "CrowdVerdict",
        "duplicate_voters",
    ] {
        assert!(
            IMPLEMENTATION.contains(term),
            "missing Contest schema: {term}"
        );
    }
}

#[test]
fn sandman_is_one_person_one_mantle_and_aegon_is_only_an_alias() {
    assert!(SUPPLEMENT.contains("The Sandman is not a physical fusion of two people"));
    assert!(SUPPLEMENT.contains("Only one active mantle exists"));
    assert!(SUPPLEMENT.contains("losing candidate\nretains their guardian Form and mantle"));
    assert!(SUPPLEMENT.contains("Aegon is a historical name"));
    assert!(SUPPLEMENT.contains("Aegaeon is not canonical terminology"));
    for forbidden in [
        "AegaeonForm",
        "AegaeonOffice",
        "AegonForm",
        "AegonOffice",
        "AegonToSandman",
    ] {
        assert!(!IMPLEMENTATION.contains(forbidden));
    }
}

#[test]
fn single_design_index_and_no_people_currency_are_preserved() {
    assert!(SUPPLEMENT.contains("one Sandmanor Design Index"));
    assert!(SUPPLEMENT.contains("No Minorian or Minoan currency"));
    assert!(SUPPLEMENT.contains("Interior and Cultivated Design"));
    assert!(SUPPLEMENT.contains("Exterior and Coastal Design"));
    assert!(CAPABILITIES.contains("Sandmanor Guardian and Reciprocal Succession"));
}

#[test]
fn authority_map_and_compromise_point_to_the_bounded_implementation() {
    for term in [
        "SANDMANOR_GUARDIAN_AND_SUCCESSION_V1.md",
        "src/world/sandmanor/milestone.rs",
    ] {
        assert!(AUTHORITY_MAP.contains(term));
        assert!(COMPROMISE.contains(term));
    }
    assert!(COMPROMISE.contains("without changing combat, movement"));
    assert!(COMPROMISE.contains("Central Junction market\nbehavior"));
    assert!(COMPROMISE.contains("recursion kernel"));
}

#[test]
fn generated_projection_contains_new_public_lore() {
    for term in [
        "Aura Fields and Content Farm",
        "Free Aura Beach -> Southern Coast -> Current Break",
        "Gnome -> Minotaur -> Hecaton",
        "Elf -> Centaur -> Pegasus",
        "the audited crowd measures improvement",
        "called Aegon",
        "one official Design Index",
    ] {
        assert!(PROJECTION.contains(term), "projection omits {term}");
    }
}

#[test]
fn sandmanor_expansion_does_not_enter_the_recursion_kernel() {
    for forbidden in [
        "GuardianMantle",
        "CurrentBreak",
        "ContentFarm",
        "ContestOfImprovementProof",
        "SandmanConvergence",
        "Aegon",
    ] {
        assert!(!KERNEL.contains(forbidden), "kernel contains {forbidden}");
    }
}
