use std::collections::{BTreeMap, BTreeSet};

use hollow_grove::world::hueman_faculties::{FacultyAuthority, HuemanFaculty};
use hollow_grove::world::power_recipes::{
    AuraPowerRoot, CurrentPowerRoot, MAX_PREPARED_POWER_RECIPES, PowerOutcome, PowerRecipeError,
    PowerWheelProgress, PreparedPowerSet, PreparedPowerSetError, RecipeRestriction,
    UnlockRequirement, decode_power_recipe_catalog, draft_power_recipe_catalog,
    draft_power_recipe_institutions, draft_power_wheel, encode_power_recipe_catalog,
    faculty_power_root, replay_power_recipe_catalog, validate_power_recipe,
    validate_power_recipe_catalog, validate_power_recipe_institutions, validate_power_wheel,
};
use hollow_grove::world::sympiote::{
    StonebendProvingAssessment, StonebendProvingJudgment, SympioteAction, SympioteActionRecord,
    SympioteError, SympioteGraft, SympioteIntegrationOutcome, SympiotePhase,
    decode_sympiote_history, encode_sympiote_history, gremlin_gargoyle_diversion_witness,
    judge_sympiote_integration,
};

const POWER_RECIPE_DOCUMENT: &str = include_str!("../HOLLOW_GROVE_POWER_RECIPE_CONSTITUTION_V1.md");
const COMPROMISE_DOCUMENT: &str = include_str!("../HOLLOW_GROVE_COMPROMISE_V1_DRAFT.md");

#[test]
fn executable_recipe_constitution_has_a_conforming_public_document() {
    for required in [
        "Powers are outcomes. Recipes are identities.",
        "Somatokinesis",
        "Psychometry",
        "Generation",
        "Dreamwalking",
        "Shapeshifting",
        "Illusion",
        "Biokinesis",
        "Clairvoyance",
        "Telekinesis",
        "Telepathy",
        "seventeen distinct methods",
        "Manticorp Academy",
        "HGPRC",
        "HGSYM",
    ] {
        assert!(
            POWER_RECIPE_DOCUMENT.contains(required),
            "missing recipe-constitution documentation fragment: {required}",
        );
    }
    for required in [
        "Power Recipe constitution milestone status",
        "`src/world/power_recipes.rs`",
        "`src/world/sympiote.rs`",
        "`HOLLOW_GROVE_POWER_RECIPE_CONSTITUTION_V1.md`",
    ] {
        assert!(
            COMPROMISE_DOCUMENT.contains(required),
            "missing Compromise integration fragment: {required}",
        );
    }
}

#[test]
fn five_faculties_keep_their_exact_current_and_aura_roots() {
    let expected = [
        (
            HuemanFaculty::Presynce,
            FacultyAuthority::Stonebend,
            CurrentPowerRoot::Somatokinesis,
            AuraPowerRoot::Psychometry,
        ),
        (
            HuemanFaculty::Prefog,
            FacultyAuthority::SandmanorMinorian,
            CurrentPowerRoot::Generation,
            AuraPowerRoot::Dreamwalking,
        ),
        (
            HuemanFaculty::Prefig,
            FacultyAuthority::SandmanorMinoan,
            CurrentPowerRoot::Shapeshifting,
            AuraPowerRoot::Illusion,
        ),
        (
            HuemanFaculty::Precog,
            FacultyAuthority::Glaushouse,
            CurrentPowerRoot::Biokinesis,
            AuraPowerRoot::Clairvoyance,
        ),
        (
            HuemanFaculty::Resynce,
            FacultyAuthority::Flynt,
            CurrentPowerRoot::Telekinesis,
            AuraPowerRoot::Telepathy,
        ),
    ];

    for (faculty, authority, current, aura) in expected {
        let root = faculty_power_root(faculty);
        assert_eq!(root.authority, authority);
        assert_eq!(root.current, current);
        assert_eq!(root.aura, aura);
        assert!(!root.principle.is_empty());
    }
}

#[test]
fn draft_catalog_embodies_all_seventeen_competing_shared_methods() {
    let catalog = draft_power_recipe_catalog();
    validate_power_recipe_catalog(&catalog).unwrap();
    assert_eq!(catalog.len(), 17);

    let counts = catalog.iter().fold(BTreeMap::new(), |mut counts, recipe| {
        *counts.entry(recipe.outcome).or_insert(0_usize) += 1;
        counts
    });
    assert_eq!(counts[&PowerOutcome::Flight], 5);
    assert_eq!(counts[&PowerOutcome::Invisibility], 4);
    assert_eq!(counts[&PowerOutcome::Healing], 4);
    assert_eq!(counts[&PowerOutcome::SuperSpeed], 4);

    for recipe in &catalog {
        assert!(!recipe.outcome_grants_house_ownership());
        assert!(
            recipe
                .restrictions
                .contains(&RecipeRestriction::NoOutcomeOwnership)
        );
        assert!(!recipe.strengths.is_empty());
        assert!(!recipe.weaknesses.is_empty());
        assert!(!recipe.trace.is_empty());
        assert!(!recipe.risks.is_empty());
    }
}

#[test]
fn methods_for_one_outcome_retain_distinct_processors_traces_and_tradeoffs() {
    let catalog = draft_power_recipe_catalog();
    for outcome in PowerOutcome::ALL {
        let methods = catalog
            .iter()
            .filter(|recipe| recipe.outcome == outcome)
            .collect::<Vec<_>>();
        let traces = methods
            .iter()
            .map(|recipe| recipe.trace.as_str())
            .collect::<BTreeSet<_>>();
        let processors = methods
            .iter()
            .flat_map(|recipe| recipe.processors())
            .collect::<Vec<_>>();
        assert_eq!(traces.len(), methods.len());
        assert!(processors.len() >= 3);
    }
}

#[test]
fn four_recipe_institutions_cover_every_faculty_without_owning_outcomes() {
    let institutions = draft_power_recipe_institutions();
    validate_power_recipe_institutions(&institutions).unwrap();
    assert_eq!(institutions.len(), 4);
    assert!(
        institutions
            .iter()
            .all(|institution| !institution.may_claim_an_outcome)
    );
    assert!(
        institutions
            .iter()
            .find(|institution| institution.name == "Manticorp Academy")
            .unwrap()
            .functions
            .iter()
            .any(|function| function.contains("Telekinesis"))
    );
}

#[test]
fn a_recipe_cannot_claim_another_facultys_roots() {
    let mut recipe = draft_power_recipe_catalog().remove(0);
    recipe.contributions[0].current_root = CurrentPowerRoot::Generation;
    assert_eq!(
        validate_power_recipe(&recipe),
        Err(PowerRecipeError::CurrentRootMismatch(
            recipe.id.clone(),
            HuemanFaculty::Resynce,
        ))
    );
}

#[test]
fn draft_power_wheel_places_every_recipe_once_and_preserves_five_sectors() {
    let catalog = draft_power_recipe_catalog();
    let wheel = draft_power_wheel(&catalog);
    validate_power_wheel(&wheel, &catalog).unwrap();
    assert_eq!(wheel.sectors.len(), 5);
    assert_eq!(wheel.cross_house_bridges.len(), 2);

    let represented = wheel
        .sectors
        .iter()
        .flat_map(|sector| sector.nodes.iter())
        .chain(wheel.cross_house_bridges.iter())
        .filter_map(|node| node.recipe_id.as_deref())
        .collect::<BTreeSet<_>>();
    assert_eq!(represented.len(), catalog.len());
}

fn complete_progress(
    catalog: &[hollow_grove::world::power_recipes::PowerRecipeDefinition],
) -> PowerWheelProgress {
    PowerWheelProgress {
        capacity: 4,
        proof: true,
        recognition: true,
        compatible_form_and_frame: true,
        discovered_or_taught_recipes: catalog.iter().map(|recipe| recipe.id.clone()).collect(),
    }
}

#[test]
fn prepared_set_requires_every_unlock_and_enforces_small_active_loadout() {
    let catalog = draft_power_recipe_catalog();
    let mut prepared = PreparedPowerSet::default();
    let missing = prepared
        .equip(&catalog[0].id, &catalog, &PowerWheelProgress::default())
        .unwrap_err();
    let PreparedPowerSetError::MissingRequirements { missing, .. } = missing else {
        panic!("expected explicit unlock requirements");
    };
    assert_eq!(
        missing,
        vec![
            UnlockRequirement::Capacity,
            UnlockRequirement::Proof,
            UnlockRequirement::Recognition,
            UnlockRequirement::CompatibleFormAndFrame,
            UnlockRequirement::DiscoveredOrTaughtRecipe,
        ]
    );

    let progress = complete_progress(&catalog);
    for recipe in catalog.iter().take(MAX_PREPARED_POWER_RECIPES) {
        prepared.equip(&recipe.id, &catalog, &progress).unwrap();
    }
    assert_eq!(prepared.recipe_ids().len(), MAX_PREPARED_POWER_RECIPES);
    assert_eq!(
        prepared.equip(&catalog[MAX_PREPARED_POWER_RECIPES].id, &catalog, &progress,),
        Err(PreparedPowerSetError::PreparedSetFull)
    );
    assert!(prepared.unequip(&catalog[0].id));
    prepared
        .equip(&catalog[MAX_PREPARED_POWER_RECIPES].id, &catalog, &progress)
        .unwrap();
}

#[test]
fn recipe_catalog_serializes_and_replays_deterministically() {
    let catalog = draft_power_recipe_catalog();
    let first = encode_power_recipe_catalog(&catalog).unwrap();
    let second = encode_power_recipe_catalog(&catalog).unwrap();
    assert_eq!(first, second);
    assert_eq!(decode_power_recipe_catalog(&first).unwrap(), catalog);
    assert_eq!(replay_power_recipe_catalog(&catalog).unwrap(), catalog);

    let mut envelope: serde_json::Value = serde_json::from_slice(&first).unwrap();
    envelope["checksum"] = serde_json::Value::String("tampered".into());
    let tampered = serde_json::to_vec(&envelope).unwrap();
    assert_eq!(
        decode_power_recipe_catalog(&tampered),
        Err(PowerRecipeError::ChecksumMismatch)
    );
}

fn apply(graft: &mut SympioteGraft, causal_position: u64, evidence: &str, action: SympioteAction) {
    graft
        .apply(SympioteActionRecord {
            causal_position,
            evidence: evidence.into(),
            action,
        })
        .unwrap();
}

fn integrated_graft() -> SympioteGraft {
    let mut graft = SympioteGraft::new("sympiote.case-a", "being.host-a");
    apply(&mut graft, 1, "evidence.sample", SympioteAction::SampleHost);
    apply(
        &mut graft,
        2,
        "evidence.recipe",
        SympioteAction::ReadHostCurrentAuraRecipe,
    );
    apply(
        &mut graft,
        3,
        "evidence.cultivation",
        SympioteAction::CultivateLivingTissue,
    );
    apply(
        &mut graft,
        4,
        "evidence.host-craft",
        SympioteAction::CraftForHost {
            requested_power_package: None,
        },
    );
    apply(
        &mut graft,
        5,
        "evidence.consent",
        SympioteAction::GraftWithConsent,
    );
    apply(
        &mut graft,
        6,
        "evidence.monitor",
        SympioteAction::BeginCompatibilityMonitoring,
    );
    apply(
        &mut graft,
        7,
        "evidence.integration",
        SympioteAction::ResolveIntegration {
            outcome: SympioteIntegrationOutcome::ReciprocalSynthesis,
            emergent_form: Some("Gargoyle".into()),
        },
    );
    graft
}

#[test]
fn sympiote_follows_the_full_glaushouse_sequence_without_selected_bias() {
    let graft = integrated_graft();
    assert_eq!(graft.phase, SympiotePhase::ReciprocallyIntegrated);
    assert_eq!(graft.emergent_form.as_deref(), Some("Gargoyle"));
    assert!(graft.successful_reciprocal_synthesis());
    assert_eq!(graft.player_selected_bias(), None);
    assert!(!graft.classification.is_sympian_lineage());
    assert_eq!(graft.events.len(), 7);
}

#[test]
fn requested_sympiote_power_package_fails_without_partial_effects() {
    let mut graft = SympioteGraft::new("sympiote.case-b", "being.host-b");
    apply(&mut graft, 1, "sample", SympioteAction::SampleHost);
    apply(
        &mut graft,
        2,
        "recipe",
        SympioteAction::ReadHostCurrentAuraRecipe,
    );
    apply(
        &mut graft,
        3,
        "tissue",
        SympioteAction::CultivateLivingTissue,
    );
    let before = graft.clone();
    assert_eq!(
        graft.apply(SympioteActionRecord {
            causal_position: 4,
            evidence: "menu-choice".into(),
            action: SympioteAction::CraftForHost {
                requested_power_package: Some("flight".into()),
            },
        }),
        Err(SympioteError::PlayerSelectedBiasForbidden)
    );
    assert_eq!(graft, before);
}

#[test]
fn sympiote_history_is_checksummed_and_replays_exactly() {
    let graft = integrated_graft();
    let first = encode_sympiote_history(&graft).unwrap();
    let second = encode_sympiote_history(&graft).unwrap();
    assert_eq!(first, second);
    assert_eq!(decode_sympiote_history(&first).unwrap(), graft);
}

#[test]
fn host_and_sympiote_rejection_resolve_to_distinct_terminal_states() {
    for (case, outcome, terminal) in [
        (
            "host-rejection",
            SympioteIntegrationOutcome::HostRejection,
            SympiotePhase::HostRejected,
        ),
        (
            "sympiote-rejection",
            SympioteIntegrationOutcome::SympioteRejection,
            SympiotePhase::SympioteRejected,
        ),
    ] {
        let mut graft = SympioteGraft::new(format!("sympiote.{case}"), "being.test-host");
        apply(&mut graft, 1, "sample", SympioteAction::SampleHost);
        apply(
            &mut graft,
            2,
            "recipe",
            SympioteAction::ReadHostCurrentAuraRecipe,
        );
        apply(
            &mut graft,
            3,
            "tissue",
            SympioteAction::CultivateLivingTissue,
        );
        apply(
            &mut graft,
            4,
            "craft",
            SympioteAction::CraftForHost {
                requested_power_package: None,
            },
        );
        apply(&mut graft, 5, "consent", SympioteAction::GraftWithConsent);
        apply(
            &mut graft,
            6,
            "monitor",
            SympioteAction::BeginCompatibilityMonitoring,
        );
        apply(
            &mut graft,
            7,
            "rejection evidence",
            SympioteAction::ResolveIntegration {
                outcome,
                emergent_form: None,
            },
        );
        assert_eq!(graft.phase, terminal);
    }
}

fn proving_assessment() -> StonebendProvingAssessment {
    StonebendProvingAssessment {
        candidate_id: "candidate.sympiote.case-a".into(),
        integration: SympioteIntegrationOutcome::ReciprocalSynthesis,
        stable_form: true,
        restraint: true,
        repeatable: true,
        reciprocal_control: true,
        identity_survives_pressure: true,
        glaushouse_clearance: true,
        coercive: false,
        actively_destructive: false,
        evidence: vec!["evidence.arena-three-repetitions".into()],
    }
}

#[test]
fn stonebend_recognizes_only_stable_restrained_reciprocal_integration() {
    let recognized = judge_sympiote_integration(proving_assessment()).unwrap();
    assert_eq!(recognized.judgment, StonebendProvingJudgment::Recognized);
    assert!(!recognized.power_grants_title_or_office);

    let mut provisional = proving_assessment();
    provisional.repeatable = false;
    assert_eq!(
        judge_sympiote_integration(provisional).unwrap().judgment,
        StonebendProvingJudgment::Provisional
    );

    let mut referred = proving_assessment();
    referred.glaushouse_clearance = false;
    assert_eq!(
        judge_sympiote_integration(referred).unwrap().judgment,
        StonebendProvingJudgment::ReferredToGlaushouse
    );
}

#[test]
fn stonebend_orders_severance_only_for_coercive_or_destructive_state() {
    let mut ordinary_failure = proving_assessment();
    ordinary_failure.integration = SympioteIntegrationOutcome::FailedIntegration;
    ordinary_failure.stable_form = false;
    ordinary_failure.identity_survives_pressure = false;
    assert_eq!(
        judge_sympiote_integration(ordinary_failure)
            .unwrap()
            .judgment,
        StonebendProvingJudgment::Rejected
    );

    let mut coercive = proving_assessment();
    coercive.coercive = true;
    assert_eq!(
        judge_sympiote_integration(coercive).unwrap().judgment,
        StonebendProvingJudgment::Severance
    );
}

#[test]
fn iconic_gargoyle_diversion_is_a_nonexecuting_proposed_witness() {
    let witness = gremlin_gargoyle_diversion_witness();
    assert_eq!(witness.starting_form, "Gremlin");
    assert_eq!(witness.expected_progression, "Goblin");
    assert_eq!(witness.emergent_synthesis_form, "Gargoyle");
    assert!(!witness.executes_progression);
    assert!(witness.status.contains("proposed"));
    assert!(witness.status.contains("Stonebend"));
}
