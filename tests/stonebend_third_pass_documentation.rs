use std::fs;

const CANON: &str = "STONEBEND_TITLE_LIFECYCLE_AND_CONSTITUTIONAL_CONTINUITY_V1.md";

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("failed to read {path}: {error}"))
}

#[test]
fn dedicated_third_pass_canon_exists() {
    let canon = read(CANON);
    assert!(canon.contains("# Stonebend Title Lifecycle and Constitutional Continuity V1"));
    assert!(canon.contains("Status: canonical Stonebend Third Pass"));
}

#[test]
fn lifecycle_and_activation_maxims_are_exact() {
    let canon = read(CANON);
    for statement in [
        "A Title is not praise. It is a bounded public Claim backed by evidence.",
        "Recognition begins a Title. Continuance sustains it.",
        "Recognition establishes the Title. Activation permits its exercise.",
        "A name may be lawful without being universally authorized.",
    ] {
        assert!(
            canon.contains(statement),
            "missing canonical statement: {statement}"
        );
    }
}

#[test]
fn restoration_and_hollowness_maxims_are_exact() {
    let canon = read(CANON);
    assert!(canon.contains("Restoration repairs the Title. It does not erase the break."));
    assert!(canon.contains("Do not uphold what is hollow. Do not hollow what is whole."));
}

#[test]
fn vacancy_forge_and_network_maxims_are_exact() {
    let canon = read(CANON);
    for statement in [
        "During vacancy, Stonebend continues. Diamond does not secretly change hands.",
        "Necessity may preserve the boundary. It may not inherit the crown.",
        "The Forge may fall silent. Its records must not disappear.",
        "The network is permanent. Its voice is temporary.",
        "The voice may be recalled. The people may not be abolished.",
    ] {
        assert!(
            canon.contains(statement),
            "missing canonical statement: {statement}"
        );
    }
}

#[test]
fn constitutional_triangle_remains_intact() {
    let canon = read(CANON);
    assert!(canon.contains(
        "The Claim proves that power can stand. The Title permits power to act. The Yield determines whether power deserves to continue."
    ));
}

#[test]
fn lifecycle_distinctions_and_failures_are_documented() {
    let canon = read(CANON);
    for term in [
        "Maintenance is not renewal.",
        "HonestFailure",
        "Negligence",
        "Fraud",
        "Illegality",
        "ConstitutionalHollowness",
        "Limitation",
        "Supervision",
        "Suspension",
        "Remediation",
    ] {
        assert!(
            canon.contains(term),
            "missing lifecycle distinction: {term}"
        );
    }
}

#[test]
fn all_terminal_dispositions_are_documented() {
    let canon = read(CANON);
    for term in [
        "HonorableCompletion",
        "Surrender",
        "Expiration",
        "Death",
        "EndOfForm",
        "Succession",
        "RemovalForFailure",
        "RemovalForFraud",
        "RemovalForIllegality",
        "ConstitutionalDissolution",
        "Supersession",
    ] {
        assert!(canon.contains(term), "missing terminal disposition: {term}");
    }
}

#[test]
fn forbidden_hidden_offices_are_rejected() {
    let canon = read(CANON);
    assert!(canon.contains(
        "No Regent, Acting Hypergiant, Interim Diamond,\npermanent Proliteriate Speaker, or fourth sovereign office exists."
    ));
    assert!(!read("src/world/stonebend/third_pass.rs").contains("struct Regent"));
}

#[test]
fn active_authority_documents_cross_reference_third_pass() {
    for path in [
        "STONEBEND_CONSTITUTION_V2.md",
        "STONEBEND_THREE_GATES_OFFICES_AND_TITLE_SCOPE_V1.md",
        "HOLLOW_GROVE_CORE_v1.0.0.md",
        "HOLLOW_GROVE_COMPROMISE_V1_DRAFT.md",
        "HOLLOW_GROVE_SEMANTIC_FOUNDATION_V1.md",
        "HOLLOW_GROVE_V2_CONSTITUTIONAL_SPECIFICATION.md",
        "REPOSITORY_AUTHORITY_MAP.md",
        "V2_CAPABILITY_INVENTORY.md",
        "artifacts/index.md",
    ] {
        assert!(
            read(path).contains(CANON),
            "{path} does not reference the Third Pass"
        );
    }
}

#[test]
fn public_role_projection_contains_lifecycle_continuity() {
    let roles = read("artifacts/hueman_stonebend_roles.md");
    assert!(roles.contains("## Title Lifecycle and Constitutional Continuity"));
    assert!(roles.contains("a Claim is not automatically a Title"));
    assert!(roles.contains("High Freemason replacement requires independent Forge review"));
    assert!(roles.contains("without a permanent speaker or locked numerical threshold"));
}

#[test]
fn world_context_preserves_domain_specific_recognition() {
    let world = read("CURRENT_SYNTHESIS_HOLLOW_GROVE_WORLD_CONTEXT_v0.1.0.md");
    assert!(world.contains("Flynt recognition is not a Stonebend Title."));
    assert!(world.contains("Stonebend recognition establishes that a\nbounded Title"));
}

#[test]
fn world_context_mirror_is_byte_identical() {
    assert_eq!(
        fs::read("CURRENT_SYNTHESIS_HOLLOW_GROVE_WORLD_CONTEXT_v0.1.0.md").unwrap(),
        fs::read("artifacts/current_synthesis_world_context.md").unwrap()
    );
}

#[test]
fn third_pass_has_no_recursion_kernel_dependency() {
    let implementation = read("src/world/stonebend/third_pass.rs");
    assert!(!implementation.contains("recursion_kernel"));
    assert!(!implementation.contains("hollow_grove_kernel"));
    assert!(implementation.contains("not replace the Bond aggregate"));
}
