const CANON: &str = include_str!("../CENTRAL_JUNCTION_FOUR_POLE_ECONOMY_V1.md");
const MODEL: &str = include_str!("../src/world/central_junction.rs");
const GEOGRAPHY: &str = include_str!("../HOLLOW_GROVE_CONSTITUTIONAL_GEOGRAPHY_V2.md");
const GEOGRAPHY_MODEL: &str = include_str!("../src/world/geography.rs");
const LIVED_LORE: &str = include_str!("../src/world/lived_lore.rs");
const CORE: &str = include_str!("../HOLLOW_GROVE_CORE_v1.0.0.md");
const HUEMAN: &str = include_str!("../HUEMAN_v0.1.0.md");
const COMPROMISE: &str = include_str!("../HOLLOW_GROVE_COMPROMISE_V1_DRAFT.md");
const AUTHORITY_MAP: &str = include_str!("../REPOSITORY_AUTHORITY_MAP.md");
const INVENTORY: &str = include_str!("../V2_CAPABILITY_INVENTORY.md");
const WORLD_CONTEXT: &str =
    include_str!("../CURRENT_SYNTHESIS_HOLLOW_GROVE_WORLD_CONTEXT_v0.1.0.md");
const WORLD_CONTEXT_PROJECTION: &str =
    include_str!("../artifacts/current_synthesis_world_context.md");
const PATH_CROSSOVERS: &str = include_str!("../artifacts/hueman_path_crossovers.md");
const FLYNT: &str = include_str!("../FLYNT_DUAL_LEADERSHIP_AND_MANTICORP_RECIPE_V1.md");
const GLAUSHOUSE: &str = include_str!("../GLAUSHOUSE_CONSTITUTION_V2.md");
const KERNEL: &str = include_str!("../hollow-grove-kernel/src/lib.rs");

#[test]
fn canon_records_all_required_public_statements() {
    for statement in [
        "Sandmanor designs the Form.",
        "Flynt engineers the Function.",
        "Stonebend makes the Form endure.",
        "Glaüshouse keeps the Function alive.",
        "One currency measures exchange. The four poles determine value. Toke/Tokens prove what was earned.",
        "Reroute what still has somewhere to go.",
        "Release what has reached its end.",
        "The Houses do not own the indexes. They fill them. Aura Ridge measures what the world currently values.",
        "The Sector Hall is where work enters public life. Central Junction is where the world decides what that work is worth.",
        "Central Junction is where Hollow Grove meets through capital, evidence, work, and public life.",
        "Design shows what the world wants to become. Engineering shows what it can make work. Craft shows what it can make endure. Repair shows what it refuses to lose.",
        "Current Haze is unresolved possibility.",
        "Equal Gaze is reconciled perspective.",
        "Aura Beam reveals or transmits the visible shared future.",
    ] {
        assert!(CANON.contains(statement), "canon omits `{statement}`");
    }
}

#[test]
fn canon_separates_currency_tokens_shares_positions_indexes_and_gremlincoin() {
    for term in [
        "Glaus is not a Glaüshouse currency.",
        "Flynt is not a Flynt currency.",
        "Mark is not a Stonebend currency.",
        "Mino is not a Sandmanor currency.",
        "The four indexes are not currencies.",
        "Gremlincoin is not spendable money.",
        "The public name of the standard currency is not yet locked.",
        "`StandardCurrencyAmount`",
        "Currency measures ordinary exchange.",
        "Toke/Tokens record earned work",
        "Shares represent enterprise participation",
        "Event contracts represent positions",
    ] {
        assert!(CANON.contains(term), "currency law omits `{term}`");
    }
}

#[test]
fn junction_district_exchange_authorities_corridors_halls_and_indexes_are_named() {
    for term in [
        "CENTRAL JUNCTION",
        "district, not a single building",
        "South Ridge Exchange",
        "Junction Board",
        "Clearing House",
        "Junction Wire",
        "Stonebend Craft Corridor",
        "Glaüshouse Repair Corridor",
        "Sandmanor Design Corridor",
        "Flynt Engineering Ring",
        "Stonebend Craft Hall",
        "Sandmanor Design Hall",
        "Flynt Engineering Hall",
        "Glaüshouse Repair Hall",
        "Sandmanor Design Index",
        "Flynt Engineering Index",
        "Stonebend Craft Index",
        "Glaüshouse Repair Index",
    ] {
        assert!(
            CANON.contains(term),
            "Central Junction canon omits `{term}`"
        );
    }
}

#[test]
fn active_market_surfaces_do_not_institutionalize_summit_concepts() {
    for (name, surface) in [
        ("canon", CANON),
        ("model", MODEL),
        ("geography", GEOGRAPHY),
        ("core", CORE),
        ("Hueman", HUEMAN),
        ("compromise", COMPROMISE),
        ("world context", WORLD_CONTEXT),
    ] {
        for forbidden in [
            "CurrentHazeAuthority",
            "Current Haze office",
            "Current Haze department",
            "EqualGazeOffice",
            "Equal Gaze regulator",
            "Equal Gaze court",
            "AuraBeamExchange",
            "Aura Beam ticker",
            "Aura Beam financial",
        ] {
            assert!(
                !surface.contains(forbidden),
                "{name} institutionalizes Summit language as `{forbidden}`"
            );
        }
    }
    assert!(CANON.contains("“The contract is still in the Current Haze”"));
    assert!(CANON.contains("“The ruling has reached Equal Gaze”"));
    assert!(MODEL.contains("MarketLifecycleState"));
    assert!(!MODEL.contains("CurrentHazeAuthority"));
    assert!(!MODEL.contains("EqualGazeOffice"));
    assert!(!MODEL.contains("AuraBeamExchange"));
}

#[test]
fn central_exchange_wording_is_retired_from_active_geography_and_projection() {
    for (name, surface) in [
        ("geography canon", GEOGRAPHY),
        ("geography model", GEOGRAPHY_MODEL),
        ("path crossover projection", PATH_CROSSOVERS),
        ("lived-lore projection", LIVED_LORE),
    ] {
        assert!(
            !surface.to_ascii_lowercase().contains("central exchange"),
            "{name} retains obsolete Central Exchange wording"
        );
        assert!(surface.contains("Central Junction"));
    }
}

#[test]
fn authority_map_inventory_and_compromise_point_to_executable_proof() {
    for term in [
        "`CENTRAL_JUNCTION_FOUR_POLE_ECONOMY_V1.md`",
        "`src/world/central_junction.rs`",
        "`tests/central_junction_constitutional_architecture.rs`",
        "`tests/central_junction_documentation_conformance.rs`",
        "`src/bin/central_junction_constitutional_audit.rs`",
    ] {
        assert!(AUTHORITY_MAP.contains(term), "authority map omits `{term}`");
    }
    for term in [
        "Aura Ridge And Central Junction Public Economy",
        "StandardCurrencyAmount",
        "Blackroot event proof",
        "Summit terminology",
    ] {
        assert!(INVENTORY.contains(term), "inventory omits `{term}`");
    }
    assert!(COMPROMISE.contains("Aura Ridge and Central Junction market milestone status"));
}

#[test]
fn generated_world_context_is_exact_and_projects_the_public_economy() {
    assert_eq!(WORLD_CONTEXT, WORLD_CONTEXT_PROJECTION);
    for term in [
        "Aura Ridge, Central Junction, and the Four-Pole Economy",
        "one unnamed standard ordinary currency",
        "Current Haze is unresolved possibility.",
        "Equal Gaze is reconciled perspective.",
        "Aura Beam reveals or transmits the visible shared future.",
        "Do not use Aura Beam as a financial ticker; use the Junction Wire.",
    ] {
        assert!(WORLD_CONTEXT.contains(term), "world context omits `{term}`");
    }
}

#[test]
fn prior_flynt_glaushouse_and_kernel_boundaries_remain_locked() {
    for term in [
        "The Tross, the Mystery Man, and Mr. X are the same individual.",
        "Manticorp Form",
        "Manticorp Institution",
        "Gremlincoin = the Gremlin Way.",
        "Gargoyle maintenance and renewal",
    ] {
        assert!(FLYNT.contains(term), "Flynt lock omits `{term}`");
    }
    for term in [
        "Nightingale",
        "Matron",
        "Marshal",
        "Persephone",
        "Prima Donna",
        "Synthesis is not permanence. Synthesis is Continuance through renewal.",
    ] {
        assert!(GLAUSHOUSE.contains(term), "Glaüshouse lock omits `{term}`");
    }
    for forbidden in [
        "CentralJunction",
        "JunctionBoard",
        "ClearingHouse",
        "JunctionWire",
        "EconomicPole",
        "EventContract",
    ] {
        assert!(
            !KERNEL.contains(forbidden),
            "public-economy law leaked into the recursion kernel as `{forbidden}`"
        );
    }
}
