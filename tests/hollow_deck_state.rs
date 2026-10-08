use std::collections::BTreeSet;
use std::fs;
use std::sync::atomic::{AtomicU64, Ordering};

use hollow_grove::hollow_deck::{Recipe, StatBlock, canonical_hollow_deck};
use hollow_grove::hollow_deck_state::{
    DEFAULT_HOLLOW_DECK_STATE_PATH, HOLLOW_DECK_STATE_FORMAT, HOLLOW_DECK_STATE_VERSION,
    HollowDeckState, HollowDeckStateError, decode_hollow_deck_state, encode_hollow_deck_state,
    load_hollow_deck_state, save_hollow_deck_state, validate_hollow_deck_state,
};
use hollow_grove::hollow_deck_tui::{render_cultivation_state, render_nature_card_with_state};
use hollow_grove::runtime_rampart::component_digest;

fn temp_path(label: &str) -> std::path::PathBuf {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    std::env::temp_dir().join(format!(
        "hollow-deck-{label}-{}-{}.json",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ))
}

fn legacy_envelope(state: &HollowDeckState) -> Vec<u8> {
    let state_bytes = serde_json::to_vec(state).expect("serialize legacy state payload");
    serde_json::to_vec_pretty(&serde_json::json!({
        "format": HOLLOW_DECK_STATE_FORMAT,
        "version": HOLLOW_DECK_STATE_VERSION,
        "checksum": component_digest(&state_bytes),
        "state": state,
    }))
    .expect("serialize legacy state envelope")
}

#[test]
fn catalog_amendment_preserves_exact_v1_flynt_cultivation_payload() {
    let catalog = canonical_hollow_deck();
    let mut state = HollowDeckState::default();
    state.cultivate(&catalog, "manticorp", 4_000).unwrap();
    state.cultivate(&catalog, "chimera", 7_000).unwrap();
    let legacy = legacy_envelope(&state);
    let restored = decode_hollow_deck_state(&catalog, &legacy).unwrap();
    assert_eq!(restored, state);
    let reencoded = encode_hollow_deck_state(&catalog, &restored).unwrap();
    let original: serde_json::Value = serde_json::from_slice(&legacy).unwrap();
    let current: serde_json::Value = serde_json::from_str(&reencoded).unwrap();
    assert_eq!(original, current);
    assert_eq!(current["version"], 1);
}

#[test]
fn current_and_aura_progress_independently_and_overlay_canon() {
    let catalog = canonical_hollow_deck();
    let mut state = HollowDeckState::default();

    let stats = state
        .cultivate(&catalog, "gargoyle", 5_000)
        .expect("valid cultivation");
    assert_eq!(stats, StatBlock::new(70, 47, 69, 75));
    state
        .set_growth_stage(&catalog, "gargoyle", "Winglets")
        .expect("canonical growth stage");
    assert!(state.aura_sets.is_empty(), "Current does not imply Aura");

    state
        .learn_aura_move(&catalog, "werewolf", "scent")
        .expect("compatible design-example Aura move may be cultivated");
    state
        .equip_aura_move(&catalog, "werewolf", "scent")
        .expect("learned Aura move may be equipped");
    assert!(
        !state.current_forms.contains_key("werewolf"),
        "Aura does not imply a Current Form"
    );

    let form = state
        .current_form_for(&catalog, "gargoyle")
        .expect("valid state")
        .expect("cultivated form");
    assert_eq!(form.computed_stats, stats);
    assert_eq!(form.current_growth_stage.as_deref(), Some("winglets"));
    assert_eq!(
        form.achieved_growth_stages,
        ["fangs", "claws", "horns", "wing-buds", "winglets"]
    );
    assert_eq!(
        catalog
            .current_form("gargoyle")
            .unwrap()
            .development_basis_points,
        10_000
    );
    assert_eq!(
        catalog
            .current_form("gargoyle")
            .unwrap()
            .current_growth_stage
            .as_deref(),
        Some("white-opal-statue-mastery"),
        "the canonical catalog is never mutated"
    );
    assert_eq!(
        state.current_forms["gargoyle"].development_basis_points,
        5_000
    );
    assert_eq!(
        state.current_forms["gargoyle"].growth_stage.as_deref(),
        Some("winglets")
    );
}

#[test]
fn invalid_stages_and_aura_equipment_are_rejected() {
    let catalog = canonical_hollow_deck();
    let mut state = HollowDeckState::default();
    assert!(
        state
            .set_growth_stage(&catalog, "gargoyle", "Perfect Wings")
            .is_err()
    );
    assert!(
        state
            .set_current_feature_stage(&catalog, "gargoyle", "claws", "Claws")
            .is_err(),
        "features without canonical stages remain TODO rather than accepting invented ranks"
    );
    assert!(
        state
            .equip_aura_move(&catalog, "werewolf", "scent")
            .is_err()
    );
    assert!(state.learn_aura_move(&catalog, "vampire", "scent").is_err());
    assert!(
        state
            .set_growth_stage(&catalog, "werewolf", "Wings")
            .is_err(),
        "a Nature without the Gargoyle path cannot store its milestones"
    );

    state
        .set_current_technique_stage(&catalog, "werewolf", "siphone", "Frenzy")
        .expect("Frenzy is a canonical Siphone stage");
    assert_eq!(
        state.current_forms["werewolf"].technique_stages["siphone"],
        "Frenzy"
    );
}

#[test]
fn winged_gargoyle_requires_discovery_and_wings_or_later_growth() {
    let catalog = canonical_hollow_deck();
    let mut state = HollowDeckState::default();
    assert!(
        state
            .record_synthesis(&catalog, "gargoyle", "white-opal-statue", &[])
            .is_err(),
        "White Opal Statue is no longer a Synthesis"
    );
    state
        .cultivate(&catalog, "gargoyle", 1_000)
        .expect("begin Gargoyle Current cultivation");
    assert!(
        state
            .record_synthesis(&catalog, "gargoyle", "winged-gargoyle-safe-flight", &[])
            .is_err(),
        "source pattern must be discovered"
    );
    state
        .discover_synthesis_recipe(&catalog, "Winged Gargoyle")
        .expect("canonical pattern");
    state
        .set_growth_stage(&catalog, "gargoyle", "Winglets")
        .expect("pre-Wings growth");
    assert!(
        state
            .record_synthesis(&catalog, "gargoyle", "winged-gargoyle-safe-flight", &[])
            .is_err(),
        "Winglets do not satisfy mature Wings"
    );
    state
        .set_growth_stage(&catalog, "gargoyle", "Wings")
        .expect("mature biological Wings");
    state
        .record_synthesis(&catalog, "gargoyle", "winged-gargoyle-safe-flight", &[])
        .expect("Wings satisfy the safe-flight Synthesis prerequisite");
    assert!(
        state
            .known_syntheses
            .contains("winged-gargoyle-safe-flight")
    );

    for later_stage in ["Mature Fortification", "White Opal Statue Mastery"] {
        let mut later = HollowDeckState::default();
        later.cultivate(&catalog, "gargoyle", 1_000).unwrap();
        later
            .discover_synthesis_recipe(&catalog, "Winged Gargoyle")
            .unwrap();
        later
            .set_growth_stage(&catalog, "gargoyle", later_stage)
            .unwrap();
        later
            .record_synthesis(&catalog, "gargoyle", "winged-gargoyle-safe-flight", &[])
            .unwrap_or_else(|error| panic!("{later_stage} must satisfy Wings: {error}"));
    }
}

#[test]
fn legacy_gargoyle_state_normalizes_after_checksum_and_preserves_unrelated_state() {
    let mut catalog = canonical_hollow_deck();
    catalog.recipes.push(Recipe {
        id: "test-legacy-recipe".into(),
        name: "Test Legacy Recipe".into(),
        category: "test".into(),
        description: String::new(),
        ingredients: Vec::new(),
        tools: Vec::new(),
        station: None,
        requirements: Vec::new(),
        result_item: "test-result".into(),
        result_quantity: 1,
        discovery_source: None,
        known: false,
    });
    let mut legacy = HollowDeckState::default();
    legacy.cultivate(&catalog, "gargoyle", 4_321).unwrap();
    legacy
        .current_forms
        .get_mut("gargoyle")
        .unwrap()
        .feature_stages
        .insert("wings".into(), "Winglets".into());
    legacy.cultivate(&catalog, "werewolf", 7_500).unwrap();
    legacy
        .set_current_technique_stage(&catalog, "werewolf", "siphone", "Rush")
        .unwrap();
    legacy
        .set_current_feature_stage(&catalog, "pegasus", "wings", "Winglets")
        .unwrap();
    legacy
        .learn_aura_move(&catalog, "werewolf", "scent")
        .unwrap();
    legacy
        .equip_aura_move(&catalog, "werewolf", "scent")
        .unwrap();
    legacy.known_recipes.insert(catalog.recipes[0].id.clone());
    legacy
        .discovered_synthesis_recipes
        .insert(catalog.synthesis_recipes[0].id.clone());
    legacy.known_syntheses = BTreeSet::from([
        "white-opal-statue".into(),
        "winged-gargoyle-safe-flight".into(),
    ]);

    let encoded = legacy_envelope(&legacy);
    let path = temp_path("legacy-normalization");
    fs::write(&path, &encoded).expect("write legacy V1 save");
    let loaded = load_hollow_deck_state(&catalog, &path).expect("load legacy V1 save");

    assert_eq!(
        fs::read(&path).expect("reread legacy save"),
        encoded,
        "loading normalizes in memory and does not rewrite the V1 file"
    );
    assert_eq!(
        loaded.current_forms["gargoyle"].development_basis_points,
        4_321
    );
    assert_eq!(
        loaded.current_forms["gargoyle"].growth_stage.as_deref(),
        Some("white-opal-statue-mastery"),
        "legacy White Opal Statue advances beyond the Winglets migration"
    );
    assert!(loaded.current_forms["gargoyle"].feature_stages.is_empty());
    assert_eq!(
        loaded.current_forms["werewolf"],
        legacy.current_forms["werewolf"]
    );
    assert_eq!(
        loaded.current_forms["pegasus"], legacy.current_forms["pegasus"],
        "an unrelated Nature's independent feature stages survive"
    );
    assert_eq!(loaded.aura_sets, legacy.aura_sets);
    assert_eq!(loaded.known_recipes, legacy.known_recipes);
    assert_eq!(
        loaded.discovered_synthesis_recipes,
        legacy.discovered_synthesis_recipes
    );
    assert!(!loaded.known_syntheses.contains("white-opal-statue"));
    assert!(
        loaded
            .known_syntheses
            .contains("winged-gargoyle-safe-flight")
    );

    let mut tampered: serde_json::Value = serde_json::from_slice(&encoded).unwrap();
    tampered["state"]["current_forms"]["gargoyle"]["feature_stages"]["wings"] = "Wings".into();
    assert!(matches!(
        decode_hollow_deck_state(&catalog, &serde_json::to_vec(&tampered).unwrap()),
        Err(HollowDeckStateError::ChecksumMismatch)
    ));
    fs::remove_file(path).expect("remove legacy test artifact");
}

#[test]
fn each_legacy_wings_value_maps_to_the_corresponding_unified_stage() {
    let catalog = canonical_hollow_deck();
    for (legacy_stage, expected_stage) in [
        ("Buds", "wing-buds"),
        ("Winglets", "winglets"),
        ("Wings", "wings"),
    ] {
        let mut legacy = HollowDeckState::default();
        legacy.cultivate(&catalog, "gargoyle", 6_789).unwrap();
        legacy
            .current_forms
            .get_mut("gargoyle")
            .unwrap()
            .feature_stages
            .insert("wings".into(), legacy_stage.into());
        let loaded = decode_hollow_deck_state(&catalog, &legacy_envelope(&legacy)).unwrap();
        assert_eq!(
            loaded.current_forms["gargoyle"].growth_stage.as_deref(),
            Some(expected_stage)
        );
        assert_eq!(
            loaded.current_forms["gargoyle"].development_basis_points,
            6_789
        );
        assert!(loaded.current_forms["gargoyle"].feature_stages.is_empty());
    }
}

#[test]
fn state_round_trips_atomically_and_rejects_tampering() {
    let catalog = canonical_hollow_deck();
    assert_eq!(HOLLOW_DECK_STATE_VERSION, 1);
    assert_eq!(
        DEFAULT_HOLLOW_DECK_STATE_PATH,
        "artifacts/gameplay-saves/hollow-deck-state-v1.json"
    );
    let mut state = HollowDeckState::default();
    state
        .cultivate(&catalog, "werewolf", 7_500)
        .expect("valid cultivation");
    state
        .set_current_technique_stage(&catalog, "werewolf", "siphone", "Rush")
        .expect("valid technique stage");

    let path = temp_path("round-trip");
    save_hollow_deck_state(&catalog, &state, &path).expect("atomic state save");
    let loaded = load_hollow_deck_state(&catalog, &path).expect("state load");
    assert_eq!(loaded, state);
    validate_hollow_deck_state(&catalog, &loaded).expect("round-tripped state validates");

    let encoded = encode_hollow_deck_state(&catalog, &state).expect("state encoding");
    let mut envelope: serde_json::Value = serde_json::from_str(&encoded).unwrap();
    assert_eq!(envelope["version"], 1);
    assert_eq!(envelope["state"]["schema_version"], 1);
    envelope["state"]["current_forms"]["werewolf"]["development_basis_points"] = 8_000.into();
    let tampered = serde_json::to_vec(&envelope).unwrap();
    assert!(matches!(
        decode_hollow_deck_state(&catalog, &tampered),
        Err(HollowDeckStateError::ChecksumMismatch)
    ));
    fs::remove_file(path).expect("remove test artifact");
}

#[test]
fn state_views_name_the_active_layer_explicitly() {
    let catalog = canonical_hollow_deck();
    let mut state = HollowDeckState::default();
    state.cultivate(&catalog, "gargoyle", 5_000).unwrap();
    let card = render_nature_card_with_state(&catalog, &state, "gargoyle").unwrap();
    assert!(card.contains("ACTIVE CULTIVATION"));
    assert!(card.contains("50.00%"));
    assert!(card.contains("TOTAL 261"));
    let overview = render_cultivation_state(&catalog, &state);
    assert!(overview.contains("CURRENT FORMS  /  BODY"));
    assert!(overview.contains("AURA SETS  /  MIND"));
}
