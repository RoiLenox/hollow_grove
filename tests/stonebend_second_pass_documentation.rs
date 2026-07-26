const SECOND_PASS: &str = include_str!("../STONEBEND_THREE_GATES_OFFICES_AND_TITLE_SCOPE_V1.md");
const CONSTITUTION: &str = include_str!("../STONEBEND_CONSTITUTION_V2.md");
const CORE: &str = include_str!("../HOLLOW_GROVE_CORE_v1.0.0.md");
const COMPROMISE: &str = include_str!("../HOLLOW_GROVE_COMPROMISE_V1_DRAFT.md");
const GEOGRAPHY: &str = include_str!("../HOLLOW_GROVE_CONSTITUTIONAL_GEOGRAPHY_V2.md");
const SEMANTIC: &str = include_str!("../HOLLOW_GROVE_SEMANTIC_FOUNDATION_V1.md");
const SPECIFICATION: &str = include_str!("../HOLLOW_GROVE_V2_CONSTITUTIONAL_SPECIFICATION.md");
const WORLD_CONTEXT: &str =
    include_str!("../CURRENT_SYNTHESIS_HOLLOW_GROVE_WORLD_CONTEXT_v0.1.0.md");
const WORLD_CONTEXT_MIRROR: &str = include_str!("../artifacts/current_synthesis_world_context.md");
const AUTHORITY_MAP: &str = include_str!("../REPOSITORY_AUTHORITY_MAP.md");
const CAPABILITY_INVENTORY: &str = include_str!("../V2_CAPABILITY_INVENTORY.md");
const ARTIFACT_INDEX: &str = include_str!("../artifacts/index.md");
const HUEMAN_PROJECTION: &str = include_str!("../artifacts/hueman_stonebend_roles.md");
const IMPLEMENTATION: &str = include_str!("../src/world/stonebend/second_pass.rs");
const KERNEL: &str = include_str!("../hollow-grove-kernel/src/lib.rs");

#[test]
fn dedicated_second_pass_document_has_all_canonical_statements() {
    for statement in [
        "Stonebend has three constitutional gates: one facing Flynt, one facing Central",
        "A name may be lawful without being universally authorized.",
        "The Freemason forges the Claim.",
        "The Hypergiant bears the Title.",
        "The Proliteriate shields the Yield.",
        "The Claim proves that power can stand. The Title permits power to act. The",
        "Diamond belongs to Stonebend. The Hypergiant bears its weight.",
        "The Hypergiant holds the Title. The Title does not become the Hypergiant's",
        "The Freemason may prove that a crown can stand. The Freemason may not place it",
        "Every burden must have a witness. Every Yield must return to those who bore",
        "The network is permanent. Its voice is temporary.",
        "The Proliteriate does not elect a ruler. It raises a witness.",
        "Spartacus is not king of the people. He is the person through whom the people",
        "One may challenge. Two may remove. None may rule alone.",
        "The voice may be recalled. The people may not be abolished.",
        "No Hypergiant claims Diamond without climbing The Lazerhorn.",
        "A Hypergiant may prepare an heir. Diamond recognizes no automatic",
        "Diamond may be lost. The Way back may not be shortened.",
        "Stonebend proves the Form can bear its name. Flynt proves the named Form can",
    ] {
        assert!(
            SECOND_PASS.contains(statement),
            "second-pass canon omits `{statement}`"
        );
    }
}

#[test]
fn three_gate_topology_and_routes_are_publicly_projected() {
    for document in [SECOND_PASS, CONSTITUTION, CORE, COMPROMISE, GEOGRAPHY] {
        for term in [
            "Flynt-facing",
            "Central Junction-facing",
            "Sandmanor-facing",
            "Stairway to Heaven",
            "Basin Motor Speedway",
            "Craft Corridor",
            "Aura Way",
            "Mt. Aura",
        ] {
            assert!(document.contains(term), "projection omits `{term}`");
        }
    }
    assert!(SECOND_PASS.contains("Central Junction is a district, not"));
    assert!(GEOGRAPHY.contains("Central Junction remains a district rather than a"));
}

#[test]
fn generic_single_gate_wording_is_explicitly_superseded() {
    assert!(SECOND_PASS.contains("Earlier references to one universal"));
    assert!(SECOND_PASS.contains("are superseded by these three facings"));
    assert!(CONSTITUTION.contains("Mt. Aura is an ideal and route landmark rather than a gate"));
    assert!(GEOGRAPHY.contains("Mt. Aura remains an ideal and route landmark rather than a gate"));
}

#[test]
fn title_scope_and_gate_failure_boundaries_are_canonical() {
    for term in [
        "FormationRecognition",
        "PublicCirculation",
        "OperationalDeployment",
        "Recognition at one gate implies nothing at another",
        "Honest failure",
        "negligence",
        "fraud",
        "illegality",
        "constitutional hollowness",
    ] {
        assert!(SECOND_PASS.contains(term), "missing `{term}`");
    }
    for term in [
        "StonebendTitleCore",
        "GateScopeRecognition",
        "GateEvidenceTransfer",
        "ReturnedEvidenceDisposition",
    ] {
        assert!(IMPLEMENTATION.contains(term), "typed model omits `{term}`");
    }
}

#[test]
fn diamond_and_hypergiant_are_never_flattened_together() {
    for document in [SECOND_PASS, CONSTITUTION, WORLD_CONTEXT, HUEMAN_PROJECTION] {
        assert!(document.contains("Diamond"));
        assert!(document.contains("Hypergiant"));
        assert!(document.contains("sovereign Title"));
        assert!(document.contains("temporary"));
    }
    for term in [
        "SovereignTitle",
        "DiamondState",
        "DiamondTenure",
        "DiamondAlreadyBorne",
        "DiamondAlreadyVacant",
    ] {
        assert!(IMPLEMENTATION.contains(term), "typed model omits `{term}`");
    }
}

#[test]
fn proliteriate_is_network_not_chamber_or_fourth_sovereign() {
    for document in [
        SECOND_PASS,
        CONSTITUTION,
        CORE,
        WORLD_CONTEXT,
        HUEMAN_PROJECTION,
    ] {
        assert!(document.contains("distributed"));
        assert!(document.contains("network"));
    }
    for obsolete in ["the civic chamber", "Proliteriate chamber"] {
        for (name, document) in [
            ("constitution", CONSTITUTION),
            ("core", CORE),
            ("world context", WORLD_CONTEXT),
            ("Hueman projection", HUEMAN_PROJECTION),
        ] {
            assert!(!document.contains(obsolete), "{name} retains `{obsolete}`");
        }
    }
    assert!(SECOND_PASS.contains("Spartacus is constitutional rhetoric and an archetype"));
    assert!(SECOND_PASS.contains("not a permanent fourth"));
}

#[test]
fn node_mandate_witness_and_recall_model_is_public() {
    for term in [
        "district nodes",
        "guild or workshop nodes",
        "labor crew or worksite nodes",
        "inherited or commonwealth community nodes",
        "stable identity",
        "recall",
        "completion",
    ] {
        assert!(SECOND_PASS.contains(term), "missing `{term}`");
    }
    for term in [
        "ProliteriateNodeKind",
        "NetworkMembership",
        "NetworkMandate",
        "RaisedWitness",
        "WitnessRecall",
    ] {
        assert!(IMPLEMENTATION.contains(term), "typed model omits `{term}`");
    }
}

#[test]
fn one_challenge_two_remove_and_tombstones_are_canonical() {
    for term in [
        "One power cannot count twice",
        "Duplicate",
        "Freemason plus Proliteriate",
        "Hypergiant plus Proliteriate",
        "Hypergiant plus Freemason",
        "Tombstone",
    ] {
        assert!(SECOND_PASS.contains(term), "missing `{term}`");
    }
    for term in [
        "ConstitutionalChallenge",
        "ConstitutionalConcurrence",
        "RemovalAuthorization",
        "RemovalDisposition",
        "OfficeTombstone",
    ] {
        assert!(IMPLEMENTATION.contains(term), "typed model omits `{term}`");
    }
}

#[test]
fn lazerhorn_path_replaces_obsolete_summit_vigil_selection() {
    for document in [
        SECOND_PASS,
        CONSTITUTION,
        CORE,
        WORLD_CONTEXT,
        HUEMAN_PROJECTION,
    ] {
        assert!(document.contains("Lazerhorn"));
    }
    for obsolete in [
        "supermajority finalist",
        "Current Summit vigil",
        "FinalistSelectedBySupermajority",
    ] {
        for (name, document) in [
            ("constitution", CONSTITUTION),
            ("core", CORE),
            ("world context", WORLD_CONTEXT),
            ("Hueman projection", HUEMAN_PROJECTION),
            ("implementation", IMPLEMENTATION),
        ] {
            assert!(!document.contains(obsolete), "{name} retains `{obsolete}`");
        }
    }
}

#[test]
fn first_pass_vertical_and_material_law_remains_present() {
    for statement in [
        "Mt. Aura is Aether.",
        "Riptide is Bathos.",
        "Aura is Aether revealed through Form.",
        "Current is Bathos embodied through weight.",
        "Aether is weightless Current.",
        "Current is the heaviest Aether.",
        "Current flows. Hollowing refines. Aether rises. Stone refracts. Aura reveals.",
    ] {
        assert!(SECOND_PASS.contains(statement), "missing `{statement}`");
    }
    for forbidden in [
        "AuraGlass",
        "PublicProofScale",
        "HouseStoneAssignment",
        "HollowingGuild",
    ] {
        assert!(!IMPLEMENTATION.contains(forbidden));
    }
}

#[test]
fn authority_and_capability_maps_point_to_second_pass() {
    for document in [
        AUTHORITY_MAP,
        CAPABILITY_INVENTORY,
        ARTIFACT_INDEX,
        SPECIFICATION,
        SEMANTIC,
    ] {
        assert!(document.contains("STONEBEND_THREE_GATES_OFFICES_AND_TITLE_SCOPE_V1.md"));
    }
    assert!(AUTHORITY_MAP.contains("`src/world/stonebend/second_pass.rs`"));
    assert!(CAPABILITY_INVENTORY.contains("Stonebend second-pass constitutional inventory"));
}

#[test]
fn generated_world_context_mirror_is_byte_identical() {
    assert_eq!(WORLD_CONTEXT.as_bytes(), WORLD_CONTEXT_MIRROR.as_bytes());
}

#[test]
fn second_pass_does_not_enter_the_recursion_kernel() {
    for forbidden in [
        "StonebendGateFacing",
        "DiamondTenure",
        "ProliteriateNetwork",
        "Lazerhorn",
        "stonebend::second_pass",
    ] {
        assert!(!KERNEL.contains(forbidden));
    }
}
