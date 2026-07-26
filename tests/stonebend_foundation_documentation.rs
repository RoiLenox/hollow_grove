const FOUNDATION: &str = include_str!("../STONEBEND_AURA_WAY_AETHER_HOLLOWING_FOUNDATION_V1.md");
const CONSTITUTION: &str = include_str!("../STONEBEND_CONSTITUTION_V2.md");
const GEOGRAPHY: &str = include_str!("../HOLLOW_GROVE_CONSTITUTIONAL_GEOGRAPHY_V2.md");
const COMPROMISE: &str = include_str!("../HOLLOW_GROVE_COMPROMISE_V1_DRAFT.md");
const CORE: &str = include_str!("../HOLLOW_GROVE_CORE_v1.0.0.md");
const WORLD_CONTEXT: &str =
    include_str!("../CURRENT_SYNTHESIS_HOLLOW_GROVE_WORLD_CONTEXT_v0.1.0.md");
const WORLD_CONTEXT_MIRROR: &str = include_str!("../artifacts/current_synthesis_world_context.md");
const AUTHORITY_MAP: &str = include_str!("../REPOSITORY_AUTHORITY_MAP.md");
const INVENTORY: &str = include_str!("../V2_CAPABILITY_INVENTORY.md");
const IMPLEMENTATION: &str = include_str!("../src/world/stonebend/foundation.rs");
const CURRENT_SYNTHESIS: &str = include_str!("../src/current_synthesis_engine.rs");
const CENTRAL_JUNCTION: &str = include_str!("../CENTRAL_JUNCTION_FOUR_POLE_ECONOMY_V1.md");
const STONEBEND_PROJECTION: &str = include_str!("../artifacts/hueman_stonebend_roles.md");

#[test]
fn vertical_law_is_exact_and_canonical() {
    for statement in [
        "Mt. Aura is Aether.",
        "Riptide is Bathos.",
        "Aura is Aether revealed through Form.",
        "Current is Bathos embodied through weight.",
        "Aether is weightless Current.",
        "Current is the heaviest Aether.",
        "Aether is Current without burden. Current is Aether bearing the world.",
        "Current repeats because weight teaches it where to return.",
        "Aether gives life something worth becoming. Bathos ensures becoming remains",
    ] {
        assert!(FOUNDATION.contains(statement), "missing `{statement}`");
    }
    assert!(
        FOUNDATION
            .contains("Current repeats. Aura reveals. Relativity bends. Synthesis transforms.")
    );
}

#[test]
fn neither_pole_is_flattened_into_morality_or_house_ownership() {
    assert!(FOUNDATION.contains("Neither pole is a moral absolute."));
    assert!(FOUNDATION.contains("No House owns the summit."));
    assert!(FOUNDATION.contains("Stonebend does not own Riptide or Bathos."));
    assert!(FOUNDATION.contains("Mt. Aura is not a fifth House"));
}

#[test]
fn aura_way_is_the_known_standard_and_not_a_shortcut() {
    for stage in [
        "prerequisite",
        "education",
        "supervised practice",
        "examination",
        "demonstrated responsibility",
        "recognition eligibility",
    ] {
        assert!(FOUNDATION.contains(stage), "missing stage `{stage}`");
    }
    assert!(
        FOUNDATION
            .contains("Aura Way does not promise success. It promises that the steps are known.")
    );
    assert!(
        FOUNDATION.contains(
            "The Houses teach the work. Aura Way organizes the path. Stonebend names the"
        )
    );
    assert!(IMPLEMENTATION.contains("AdvancementRouteKind::StandardAuraWay"));
    assert!(IMPLEMENTATION.contains("AuraWayCannotBeExceptional"));
}

#[test]
fn geographic_and_institutional_route_meanings_coexist() {
    assert!(FOUNDATION.contains(
        "Sandmanor\n→ Aura Way\n→ Mt. Aura\n→ Sandmanor-facing Stonebend gate\n→ Stonebend"
    ));
    assert!(
        GEOGRAPHY
            .contains("| `geography.route.aura-way` | Aura Way | Stonebend / Sandmanor | Design |")
    );
    assert!(
        GEOGRAPHY
            .contains("| `geography.route.mnt-aura` | Mt. Aura | Stonebend / Sandmanor | Aspire |")
    );
    assert!(
        GEOGRAPHY
            .contains("| `geography.route.riptide` | Riptide | Flynt / Glaüshouse | Retrieve |")
    );
    assert!(GEOGRAPHY.contains("This meaning does not make Riptide evil"));
    assert!(GEOGRAPHY.contains("does not require every profession to climb physically"));
}

#[test]
fn stonebend_recognition_is_not_perfection() {
    assert!(
        FOUNDATION.contains("Stonebend names what has been proven. It does not pretend proof is")
    );
    assert!(FOUNDATION.contains("Recognized proof is not metaphysical perfection."));
    assert!(IMPLEMENTATION.contains("declares_metaphysical_perfection: false"));
    assert!(CONSTITUTION.contains("without declaring perfection"));
}

#[test]
fn lawful_hollowing_preserves_identity_essential_fractions_and_provenance() {
    for term in [
        "CurrentBatch",
        "medium_lineage",
        "HollowingAuthorization",
        "allowed_removed_fractions",
        "EssentialFractionRemoval",
        "AetherBatch",
        "BatchProvenance",
        "source_current",
        "SealRecordId",
    ] {
        assert!(
            IMPLEMENTATION.contains(term),
            "implementation omits `{term}`"
        );
    }
    assert!(
        FOUNDATION
            .contains("Do not preserve what is merely waste. Do not remove what makes the whole.")
    );
    assert!(FOUNDATION.contains("Do not uphold what is hollow. Do not hollow what is whole."));
}

#[test]
fn honest_failure_remains_distinct_from_illegal_hollowing() {
    assert!(FOUNDATION.contains("An authorized process may fail without becoming fraud."));
    assert!(IMPLEMENTATION.contains("FailedWithoutMisconduct"));
    assert!(IMPLEMENTATION.contains("LawfulProcessFailure"));
    assert!(IMPLEMENTATION.contains("is_illegal_hollowing"));
}

#[test]
fn obsolete_all_hollowing_removes_the_useful_interior_rule_is_corrected() {
    for obsolete in [
        "Hollowing is the act of removing the useful interior while preserving or leaving behind the outer form.",
        "Hollowing extracts the usable interior resources of a thing while leaving its outer form behind.",
        "Hollowing is extraction of the useful interior while leaving the exterior.",
    ] {
        assert!(
            !WORLD_CONTEXT.contains(obsolete),
            "obsolete rule `{obsolete}`"
        );
    }
    assert!(
        WORLD_CONTEXT
            .contains("Extraction Hollowing removes a bounded interior resource while preserving")
    );
    assert!(
        WORLD_CONTEXT
            .contains("Refinement Hollowing removes authorized burden while preserving essential")
    );
}

#[test]
fn proof_retains_measurement_and_evidence_without_a_public_scale() {
    assert!(FOUNDATION.contains("Proof has two connected meanings:"));
    assert!(FOUNDATION.contains("measures concentration, refinement, or potency"));
    assert!(FOUNDATION.contains("genuine, authorized, and correctly"));
    assert!(FOUNDATION.contains("does not establish a public numeric scale"));
    for state in ["Unmeasured", "Measured", "Recognized", "Rejected"] {
        assert!(IMPLEMENTATION.contains(state));
    }
}

#[test]
fn prism_model_preserves_aether_and_stone_provenance() {
    for statement in [
        "Current flows. Hollowing refines. Aether rises. Stone refracts. Aura reveals.",
        "Current supplies the force. Crystal supplies the pattern. Aura is the",
        "Aether is universal. Aura is particular.",
        "Opal varies.",
        "Diamond concentrates.",
        "Quartz resonates.",
    ] {
        assert!(FOUNDATION.contains(statement), "missing `{statement}`");
    }
    for field in [
        "source_aether",
        "source_current",
        "medium_lineage",
        "stone_profile",
        "formation_geography",
    ] {
        assert!(IMPLEMENTATION.contains(field));
    }
}

#[test]
fn first_pass_locks_no_house_stone_melting_or_synthetic_recast_canon() {
    assert!(FOUNDATION.contains("This pass creates no final House-to-stone assignment."));
    assert!(IMPLEMENTATION.contains("final_house_assignment"));
    assert!(IMPLEMENTATION.contains("None"));
    assert!(FOUNDATION.contains("Melting is not the ordinary Aura process"));
    assert!(IMPLEMENTATION.contains("requires_melting_for_aura"));
    assert!(!FOUNDATION.contains("Aura Glass"));
    assert!(!IMPLEMENTATION.contains("AuraGlass"));
}

#[test]
fn established_house_rock_rhetoric_is_preserved_as_nonexclusive() {
    assert!(WORLD_CONTEXT.contains("Diamond claims"));
    assert!(WORLD_CONTEXT.contains("Crystal measures"));
    assert!(WORLD_CONTEXT.contains("Jade clears"));
    assert!(WORLD_CONTEXT.contains("Opal shimmers"));
    assert!(FOUNDATION.contains(
        "Earlier\nHouse material rhetoric and named resources do not convert these natural"
    ));
}

#[test]
fn summit_and_market_concepts_are_unchanged() {
    assert!(CENTRAL_JUNCTION.contains("Current Haze is unresolved possibility."));
    assert!(CENTRAL_JUNCTION.contains("Equal Gaze is reconciled perspective."));
    assert!(CENTRAL_JUNCTION.contains("Aura Beam reveals or transmits the visible shared future."));
    for prohibited in [
        "CurrentHazeAuthority",
        "EqualGazeOffice",
        "AuraBeamExchange",
    ] {
        assert!(!IMPLEMENTATION.contains(prohibited));
    }
}

#[test]
fn current_synthesis_residue_destinations_remain_intact() {
    assert!(CURRENT_SYNTHESIS.contains("ResidueDestination::Aether"));
    assert!(CURRENT_SYNTHESIS.contains("ResidueDestination::Bathos"));
    assert!(CURRENT_SYNTHESIS.contains("SemanticSide::Left => ResidueDestination::Aether"));
    assert!(CURRENT_SYNTHESIS.contains("SemanticSide::Right => ResidueDestination::Bathos"));
}

#[test]
fn authority_maps_and_public_projection_name_the_bounded_foundation() {
    for document in [AUTHORITY_MAP, INVENTORY, COMPROMISE, CORE] {
        assert!(
            document.contains("STONEBEND_AURA_WAY_AETHER_HOLLOWING_FOUNDATION_V1.md")
                || document.contains("First Stonebend foundation"),
            "foundation missing from authority surface"
        );
    }
    assert!(STONEBEND_PROJECTION.contains("## Aura Way, Aether, and Stone"));
    assert!(STONEBEND_PROJECTION.contains("Mt. Aura is Aether"));
    assert!(STONEBEND_PROJECTION.contains("ordinary Aura manifestation preserves"));
}

#[test]
fn generated_world_context_mirror_remains_byte_identical() {
    assert_eq!(WORLD_CONTEXT.as_bytes(), WORLD_CONTEXT_MIRROR.as_bytes());
}

#[test]
fn parent_stonebend_constitution_still_has_twenty_one_articles() {
    for article in 1..=21 {
        let roman = [
            "", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X", "XI", "XII", "XIII",
            "XIV", "XV", "XVI", "XVII", "XVIII", "XIX", "XX", "XXI",
        ][article];
        assert!(
            CONSTITUTION.contains(&format!("## Article {roman}")),
            "missing Article {roman}"
        );
    }
    assert!(!CONSTITUTION.contains("## Article XXII"));
}
