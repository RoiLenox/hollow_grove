use std::any::TypeId;

use hollow_grove::House;
use hollow_grove::hollow_deck::EffectChannel;
use hollow_grove::hollow_deck::{
    AuraMatchup, AuraMaterial, AuraSet, CurrentForm, CurrentType, Dominion,
    FULL_EXPRESSION_STAT_CEILING, FlyntArtSelection, FlyntCitizenship, FlyntEcosystemRole,
    FlyntEliteStatus, FlyntOperatingOrganization, FlyntOrganization, FlyntSect, FlyntServiceStatus,
    FlyntSorceryFunction, FlyntSorceryIdentity, GlaushouseBranch, GlaushouseSeat,
    HOLLOW_DECK_SCHEMA_VERSION, HUEMAN_STATS, InstitutionalChannel, Nature, Recipe,
    SandmanorAuraMastery, SandmanorCulture, SandmanorMasteryTier, SandmanorSocialIdentity,
    SandmanorSovereignIdentity, StatBlock, StonebendCitizenship, StonebendCompetency,
    StonebendMastery, StonebendTrainingStatus, SynthesisRecipe, SynthesisRecipeSourceType,
    SynthesisResultType, achieved_growth_stages, canonical_hollow_deck, compute_transformed_stats,
    get_applicable_aura_modifier, get_aura_matchup, get_cross_house_counterpart,
    get_evolution_path, get_house_for_nature, get_pair_relationship, get_primary_trait,
    resolve_flynt_art_identity, resolve_flynt_principal_institutional_stack,
    resolve_stonebend_order_mastery, validate_hollow_deck, validate_synthesis_requirements,
};
use hollow_grove::hollow_deck_tui::{
    NatureQuery, render_alchemy, render_compare, render_counterparts, render_flynt_identities,
    render_glaushouse, render_home, render_institutions, render_nature_card, render_nature_index,
    render_progression, render_recipes, render_sandmanor, render_synthesis_recipes,
};

fn names(catalog: &hollow_grove::hollow_deck::HollowDeckCatalog, path_id: &str) -> Vec<String> {
    catalog
        .evolution_path(path_id)
        .expect("canonical path")
        .nature_ids
        .iter()
        .map(|id| catalog.nature(id).expect("canonical Nature").name.clone())
        .collect()
}

#[test]
fn canonical_catalog_validates_and_keeps_hueman_neutral() {
    let catalog = canonical_hollow_deck();
    validate_hollow_deck(&catalog).expect("canonical Hollow Deck should validate");
    assert_eq!(HOLLOW_DECK_SCHEMA_VERSION, "1.6.0");
    assert_eq!(catalog.schema_version, HOLLOW_DECK_SCHEMA_VERSION);
    assert_eq!(catalog.hueman_stats, StatBlock::new(50, 50, 50, 50));
    assert_eq!(catalog.hueman_stats, HUEMAN_STATS);
    assert_eq!(catalog.hueman_stats.computed_total(), 200);
    assert_eq!(catalog.houses.len(), 4);
    assert_eq!(catalog.dominions.len(), 5);
    assert_eq!(catalog.natures.len(), 32);
    assert_eq!(catalog.primary_traits.len(), 32);
}

#[test]
fn every_canonical_stat_is_in_range_and_under_the_shared_ceiling() {
    let catalog = canonical_hollow_deck();
    for nature in &catalog.natures {
        for stat in [
            nature.mature_stats.strength,
            nature.mature_stats.speed,
            nature.mature_stats.attack,
            nature.mature_stats.defense,
        ] {
            assert!(stat <= 100, "{} has an out-of-range stat", nature.name);
        }
        assert!(
            nature.mature_stats.computed_total() <= FULL_EXPRESSION_STAT_CEILING,
            "{} exceeds the shared ceiling",
            nature.name
        );
        assert_ne!(
            nature.mature_stats,
            StatBlock::new(100, 100, 100, 100),
            "no apex becomes universally perfect"
        );
    }
}

#[test]
fn house_current_and_coequal_glaushouse_aura_matrix_is_exact() {
    let catalog = canonical_hollow_deck();
    let expected = [
        (
            House::Stonebend,
            CurrentType::Hollow,
            vec![AuraMaterial::Diamond],
        ),
        (
            House::Sandmanor,
            CurrentType::Regular,
            vec![AuraMaterial::Crystal],
        ),
        (
            House::Glaushouse,
            CurrentType::Hollow,
            vec![AuraMaterial::Jade, AuraMaterial::Emerald],
        ),
        (House::Flynt, CurrentType::Regular, vec![AuraMaterial::Opal]),
    ];
    for (house, current, aura) in expected {
        let entry = catalog
            .houses
            .iter()
            .find(|entry| entry.id == house)
            .expect("canonical House");
        assert_eq!(entry.current_type, current);
        assert_eq!(entry.aura_materials, aura);
    }
    assert_eq!(
        catalog.nature("Gremlin").unwrap().house_id,
        House::Stonebend
    );
    for nature in [
        "Pixie", "Sprite", "Fairy", "Spirit", "Sylph", "Nymph", "Siren", "Muse",
    ] {
        assert_eq!(catalog.nature(nature).unwrap().house_id, House::Glaushouse);
    }
    let directions = [
        (House::Stonebend, "North"),
        (House::Sandmanor, "East"),
        (House::Glaushouse, "South"),
        (House::Flynt, "West"),
    ];
    for (house, direction) in directions {
        assert_eq!(
            catalog
                .houses
                .iter()
                .find(|entry| entry.id == house)
                .and_then(|entry| entry.direction.as_deref()),
            Some(direction)
        );
    }
}

#[test]
fn glaushouse_branches_titles_seats_and_traits_are_exact() {
    let catalog = canonical_hollow_deck();
    assert!(AuraMaterial::ALL.contains(&AuraMaterial::Emerald));
    assert_eq!(catalog.glaushouse_structure.seats, GlaushouseSeat::ALL);
    assert_eq!(
        GlaushouseSeat::ALL.map(GlaushouseSeat::as_str),
        ["Calli", "Cleo", "Melony", "Teri", "Talia"]
    );
    assert_eq!(
        GlaushouseSeat::Melony.entry_title(),
        "Nightingale de Melony"
    );

    let expected = [
        (
            GlaushouseBranch::Presensor,
            "Matron",
            AuraMaterial::Jade,
            "Matron de Melony",
            ["Pixie", "Fairy", "Nymph", "Muse"],
            ["Detection", "Mending", "Restoration", "Hope"],
        ),
        (
            GlaushouseBranch::Preceptor,
            "Marshall",
            AuraMaterial::Emerald,
            "Marshall de Melony",
            ["Sprite", "Spirit", "Sylph", "Siren"],
            ["Prediction", "Projection", "Deflection", "Compulsion"],
        ),
    ];
    for (branch, role, material, title, nature_names, trait_names) in expected {
        assert_eq!(branch.role_family(), role);
        assert_eq!(branch.aura_material(), material);
        assert_eq!(branch.achieved_title(GlaushouseSeat::Melony), title);
        let branch_path = catalog
            .glaushouse_structure
            .branch_paths
            .iter()
            .find(|path| path.branch == branch)
            .expect("canonical Glaüshouse branch");
        assert_eq!(
            names(&catalog, &branch_path.evolution_path_id),
            nature_names
        );
        assert_eq!(
            nature_names.map(|nature| {
                get_primary_trait(&catalog, nature)
                    .expect("branch Nature trait")
                    .name
                    .as_str()
            }),
            trait_names
        );
    }

    let rendered = render_glaushouse(&catalog);
    assert!(rendered.contains("Nightingale de Melony"));
    assert!(rendered.contains("Matron de Melony"));
    assert!(rendered.contains("Marshall de Melony"));

    let mut invalid = catalog.clone();
    invalid
        .natures
        .iter_mut()
        .find(|nature| nature.id == "pixie")
        .unwrap()
        .aura_material = AuraMaterial::Emerald;
    assert!(
        validate_hollow_deck(&invalid).is_err(),
        "a Presensor Nature cannot claim the Preceptor material"
    );
}

#[test]
fn flynt_traits_budgets_and_pair_principles_are_exact() {
    let catalog = canonical_hollow_deck();
    let expected = [
        ("Gargoyle", "Fortification", "Medusa", "Stillness"),
        ("Medusa", "Petrification", "Gargoyle", "Stillness"),
        ("Werewolf", "Siphone", "Vampire", "Circulation"),
        ("Vampire", "Transfusion", "Werewolf", "Circulation"),
        ("Mariner", "Anchoring", "Mermaid", "Position"),
        ("Mermaid", "Attraction", "Mariner", "Position"),
        ("Manticorp", "Convergence", "Chimera", "Form"),
        ("Chimera", "Divergence", "Manticorp", "Form"),
    ];
    for (nature_name, trait_name, pair_name, principle) in expected {
        let nature = catalog.nature(nature_name).expect("Flynt Nature");
        assert_eq!(nature.house_id, House::Flynt);
        assert_eq!(nature.dominion_id, Dominion::Resynce);
        assert_eq!(nature.mature_stats.computed_total(), 320);
        assert_eq!(
            get_primary_trait(&catalog, nature_name).unwrap().name,
            trait_name
        );
        assert_eq!(
            catalog
                .nature(nature.paired_nature_id.as_deref().unwrap())
                .unwrap()
                .name,
            pair_name
        );
        assert_eq!(nature.shared_pair_principle.as_deref(), Some(principle));
        assert_eq!(
            get_pair_relationship(&catalog, nature_name)
                .unwrap()
                .shared_principle,
            principle
        );
    }
}

#[test]
fn flynt_contextual_identity_matrix_is_visual_not_biological_data() {
    let catalog = canonical_hollow_deck();
    let expected = [
        ("gargoyle", "Hunter", "Bro White", "Johnny Apple", "Apple"),
        (
            "werewolf",
            "Chief",
            "Beauty the Beast",
            "Tommy Rose",
            "Rose",
        ),
        (
            "mariner",
            "Mariner",
            "Cindarellaman",
            "Jimmy Pumps",
            "Shoe / pump",
        ),
    ];
    assert_eq!(catalog.flynt_visual_identities.len(), expected.len());
    for (nature_id, mystery, rural, gallows, symbol) in expected {
        let profile = catalog
            .flynt_visual_identity(nature_id)
            .expect("canonical Flynt visual identity");
        assert_eq!(profile.nature_id, nature_id);
        assert_eq!(profile.mystery_men_identity.name, mystery);
        assert_eq!(profile.we_fairy_men_identity.name, rural);
        assert_eq!(profile.gallows_identity.name, gallows);
        assert_eq!(profile.symbol_anchor, symbol);
    }
    for alias in [
        "Hunter",
        "Bro White",
        "Johnny Apple",
        "Chief",
        "Beauty the Beast",
        "Tommy Rose",
        "Cindarellaman",
        "Jimmy Pumps",
    ] {
        assert!(
            catalog.nature(alias).is_none(),
            "{alias} must not be a Nature"
        );
        assert!(
            catalog.growth_path(alias).is_none(),
            "{alias} must not own a biological growth path"
        );
    }

    let resolved = resolve_flynt_art_identity(
        &catalog,
        &FlyntArtSelection {
            nature_id: "Gargoyle".into(),
            growth_stage: Some("Wings".into()),
            organization: FlyntOrganization::Gallows,
            outfit: Some("urban dock coat".into()),
            accessories: vec!["restrained apple pin".into()],
            environment: Some("harbor back room".into()),
            animation_state: Some("grounded return".into()),
        },
    )
    .expect("canonical contextual render selection");
    assert_eq!(resolved.nature_id, "gargoyle");
    assert_eq!(resolved.growth_stage.as_deref(), Some("wings"));
    assert_eq!(resolved.citizenship, FlyntCitizenship::Warden);
    assert_eq!(resolved.service_history, FlyntServiceStatus::Corpman);
    assert_eq!(resolved.elite_order, FlyntEliteStatus::MysteryMan);
    assert_eq!(resolved.sect, FlyntSect::Hunter);
    assert_eq!(resolved.social_identity_id, "johnny-apple");
    assert_eq!(resolved.social_identity, "Johnny Apple");
    assert_eq!(resolved.outfit, "urban dock coat");
    assert_eq!(resolved.accessories, ["restrained apple pin"]);
    assert_eq!(resolved.environment, "harbor back room");
    assert_eq!(resolved.animation_state, "grounded return");

    let invalid_nature = FlyntArtSelection {
        nature_id: "Johnny Apple".into(),
        growth_stage: None,
        organization: FlyntOrganization::Gallows,
        outfit: None,
        accessories: Vec::new(),
        environment: None,
        animation_state: None,
    };
    assert!(resolve_flynt_art_identity(&catalog, &invalid_nature).is_err());
    let board = render_flynt_identities(&catalog, None).unwrap();
    assert!(board.contains("Gargoyle     Hunter         Bro White"));
    assert!(board.contains("Johnny Apple"));
    assert!(board.contains("Beauty the Beast"));
    assert!(board.contains("Cindarellaman"));
    assert!(board.contains("literal fairy-tale cosplay"));

    let matches = hollow_grove::hollow_deck_tui::query_natures(
        &catalog,
        &NatureQuery {
            search: Some("Jimmy Pumps".into()),
            ..NatureQuery::default()
        },
    );
    assert_eq!(
        matches
            .iter()
            .map(|nature| nature.id.as_str())
            .collect::<Vec<_>>(),
        ["mariner"]
    );
}

#[test]
fn flynt_service_sects_convergence_and_principal_stacks_are_exact() {
    let catalog = canonical_hollow_deck();
    let hierarchy = &catalog.flynt_institution;
    assert_eq!(hierarchy.citizen_baseline, FlyntCitizenship::Warden);
    assert_eq!(hierarchy.service_status, FlyntServiceStatus::Corpman);
    assert_eq!(hierarchy.service_prerequisite, FlyntCitizenship::Warden);
    assert_eq!(hierarchy.capacity_requirement, "Prove Capacity");
    assert_eq!(hierarchy.elite_status, FlyntEliteStatus::MysteryMan);
    assert_eq!(hierarchy.elite_prerequisite, FlyntServiceStatus::Corpman);
    assert_eq!(hierarchy.sect_prerequisite, FlyntEliteStatus::MysteryMan);
    assert_ne!(
        hierarchy.service_status.as_str(),
        hierarchy.convergence.perfected_identity
    );

    let expected = [
        (
            FlyntSect::Hunter,
            "gargoyle",
            "fortification",
            "Bro White",
            "Johnny Apple",
        ),
        (
            FlyntSect::Chief,
            "werewolf",
            "siphone",
            "Beauty the Beast",
            "Tommy Rose",
        ),
        (
            FlyntSect::Mariner,
            "mariner",
            "anchoring",
            "Cindarellaman",
            "Jimmy Pumps",
        ),
    ];
    assert_eq!(
        hierarchy
            .sects
            .iter()
            .map(|tradition| tradition.sect)
            .collect::<Vec<_>>(),
        FlyntSect::ALL
    );
    for (sect, nature_id, trait_id, rural, urban) in expected {
        let tradition = catalog.flynt_sect(sect).expect("parallel Flynt sect");
        assert_eq!(tradition.nature_id, nature_id);
        assert_eq!(tradition.primary_trait_id, trait_id);
        let nature = catalog.nature(nature_id).unwrap();
        assert_eq!(nature.primary_trait_id, trait_id);

        let stack = resolve_flynt_principal_institutional_stack(&catalog, nature_id).unwrap();
        assert_eq!(stack.nature_id, nature_id);
        assert_eq!(stack.citizenship, FlyntCitizenship::Warden);
        assert_eq!(stack.service_history, FlyntServiceStatus::Corpman);
        assert_eq!(stack.elite_order, FlyntEliteStatus::MysteryMan);
        assert_eq!(stack.sect, sect);

        let identity = catalog.flynt_visual_identity(nature_id).unwrap();
        assert_eq!(identity.we_fairy_men_identity.name, rural);
        assert_eq!(identity.gallows_identity.name, urban);
        assert!(catalog.nature(rural).is_none());
        assert!(catalog.nature(urban).is_none());
    }
    assert!(
        hierarchy
            .sects
            .iter()
            .all(|tradition| tradition.nature_id != "chimera"),
        "Synthesis attainment is not a fourth ordinary sect"
    );
    assert_eq!(hierarchy.convergence.required_sects, FlyntSect::ALL);
    assert_eq!(hierarchy.convergence.convergence_identity, "Manticorp");
    assert_eq!(hierarchy.convergence.convergence_nature_id, "manticorp");
    assert_eq!(hierarchy.convergence.perfected_from_identity, "Manticorp");
    assert_eq!(hierarchy.convergence.perfected_identity, "Chimera");
    assert_eq!(hierarchy.convergence.perfected_nature_id, "chimera");
    assert_eq!(hierarchy.convergence.apex_association, "The Tross");

    let pair = get_pair_relationship(&catalog, "manticorp").unwrap();
    assert_eq!(pair.nature_a, "manticorp");
    assert_eq!(pair.nature_b, "chimera");
    assert_eq!(pair.primary_trait_a, "convergence");
    assert_eq!(pair.primary_trait_b, "divergence");
    assert!(
        hierarchy
            .convergence
            .legacy_pair_compatibility
            .contains("peer mechanics")
    );

    let marquise = hierarchy
        .operating_contexts
        .iter()
        .find(|context| context.organization == FlyntOperatingOrganization::Marquise)
        .unwrap();
    assert_eq!(marquise.tradition_affinity, Some(FlyntSect::Mariner));
    assert!(
        hierarchy
            .operating_contexts
            .iter()
            .all(|context| context.relationship_to_hierarchy.contains("never"))
    );

    let gargoyle_growth = catalog.growth_path("gargoyle").unwrap();
    assert_eq!(
        gargoyle_growth
            .milestones
            .iter()
            .map(|milestone| milestone.name.as_str())
            .collect::<Vec<_>>(),
        [
            "Fangs",
            "Claws",
            "Horns",
            "Wing Buds",
            "Winglets",
            "Wings",
            "Mature Fortification",
            "White Opal Statue Mastery",
        ]
    );
    let rendered = render_institutions(&catalog, Some("flynt")).unwrap();
    assert!(rendered.contains("Warden"));
    assert!(rendered.contains("Corpman"));
    assert!(rendered.contains("Hunter + Chief + Mariner → Manticorp"));
    assert!(rendered.contains("Manticorp → Chimera"));
    assert!(rendered.contains("CONTEXTUAL IDENTITIES — NOT CAREER STEPS"));
}

#[test]
fn flynt_deck_rejects_superseded_mastery_without_changing_peer_mechanics() {
    use hollow_grove::world::practitioner_ontology::{
        FlyntSynthesisState, HIGHER_SYNTHESIS_RULES, HigherSynthesisNode,
    };

    let mut catalog = canonical_hollow_deck();
    let perfected = HIGHER_SYNTHESIS_RULES
        .iter()
        .find(|rule| rule.result == HigherSynthesisNode::FlyntState(FlyntSynthesisState::Chimera))
        .unwrap();
    assert_eq!(
        perfected.sources,
        &[HigherSynthesisNode::FlyntState(
            FlyntSynthesisState::Manticorp
        )]
    );
    assert_eq!(
        catalog.flynt_institution.convergence.convergence_identity,
        "Manticorp"
    );
    assert_eq!(
        catalog.flynt_institution.convergence.perfected_identity,
        "Chimera"
    );
    let peer_pair = get_pair_relationship(&catalog, "manticorp")
        .unwrap()
        .clone();

    let convergence = &mut catalog.flynt_institution.convergence;
    convergence.convergence_identity = "Chimera".into();
    convergence.convergence_nature_id = "chimera".into();
    convergence.perfected_from_identity = "Chimera".into();
    convergence.perfected_identity = "Manticorp".into();
    convergence.perfected_nature_id = "manticorp".into();
    assert!(validate_hollow_deck(&catalog).is_err());
    assert_eq!(
        get_pair_relationship(&catalog, "manticorp").unwrap(),
        &peer_pair
    );
}

#[test]
fn flynt_sorcery_ladders_pairings_and_ecosystem_are_exact() {
    let catalog = canonical_hollow_deck();
    let sorcery = &catalog.flynt_institution.sorcery;
    assert_eq!(
        sorcery.domain,
        "Flynt synthesis manufacturing and formulation"
    );

    let expected = [
        (
            FlyntSorceryIdentity::Wizard,
            FlyntSorceryIdentity::Witch,
            FlyntSorceryFunction::EstablishedManufacture,
            vec![
                "manufacture established Synthesis Recipes",
                "manufacture established Synthesis products",
            ],
        ),
        (
            FlyntSorceryIdentity::Warlock,
            FlyntSorceryIdentity::Enchantress,
            FlyntSorceryFunction::PatternModification,
            vec![
                "combine",
                "counterfeit",
                "corrupt",
                "strengthen",
                "weaken",
                "repurpose",
                "reformulate",
            ],
        ),
        (
            FlyntSorceryIdentity::Sorcerer,
            FlyntSorceryIdentity::Sorceress,
            FlyntSorceryFunction::PatternAuthorship,
            vec![
                "author new Synthesis Recipes",
                "author new Synthesis patterns",
            ],
        ),
    ];
    assert_eq!(sorcery.paired_ladder.len(), expected.len());
    for (pair, (male, female, function, capabilities)) in sorcery.paired_ladder.iter().zip(expected)
    {
        assert_eq!(pair.male_identity, male);
        assert_eq!(pair.female_identity, female);
        assert_eq!(pair.function, function);
        assert_eq!(
            pair.capabilities
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            capabilities
        );
        assert!(catalog.nature(male.as_str()).is_none());
        assert!(catalog.nature(female.as_str()).is_none());
    }

    assert_eq!(
        sorcery
            .ecosystem_roles
            .iter()
            .map(|profile| profile.role)
            .collect::<Vec<_>>(),
        FlyntEcosystemRole::ALL
    );
    assert_eq!(
        sorcery.governance,
        "The Tross governs the wider Flynt vice and synthesis ecosystem."
    );
    assert_eq!(
        sorcery.systemic_danger,
        "Uncontrolled synthesis is the key systemic danger."
    );
    assert!(
        sorcery
            .sanctioned_practitioner_boundary
            .contains("not inherently criminal")
    );
    assert!(
        sorcery
            .axis_boundary
            .contains("nor a Mystery Men sect or rank")
    );

    let rendered = render_institutions(&catalog, Some("flynt")).unwrap();
    assert!(rendered.contains("Wizard       ↔ Witch"));
    assert!(rendered.contains("Warlock      ↔ Enchantress"));
    assert!(rendered.contains("Sorcerer     ↔ Sorceress"));
    assert!(rendered.contains("Uncontrolled synthesis is the key systemic danger"));
    assert!(rendered.contains("FLYNT  /  FORMAL SERVICE"));

    let mut invalid = catalog.clone();
    invalid.flynt_institution.sorcery.paired_ladder.swap(0, 2);
    assert!(validate_hollow_deck(&invalid).is_err());
}

#[test]
fn stonebend_paired_competencies_masteries_and_apex_remain_non_biological() {
    let catalog = canonical_hollow_deck();
    let hierarchy = &catalog.stonebend_institution;
    assert_eq!(hierarchy.citizen_baseline, StonebendCitizenship::Gerald);
    assert_eq!(
        hierarchy.training_entry,
        StonebendTrainingStatus::Apprentice
    );
    assert_eq!(
        hierarchy.training_prerequisite,
        StonebendCitizenship::Gerald
    );
    assert_eq!(
        hierarchy.competency_prerequisite,
        StonebendTrainingStatus::Apprentice
    );
    assert!(!hierarchy.contains_active_identity("Applicant"));

    let expected_competencies = [
        (
            StonebendCompetency::Mason,
            InstitutionalChannel::Current,
            None,
        ),
        (
            StonebendCompetency::Mage,
            InstitutionalChannel::Aura,
            Some(AuraMaterial::Diamond),
        ),
        (
            StonebendCompetency::Pugilist,
            InstitutionalChannel::Current,
            None,
        ),
        (
            StonebendCompetency::Foreman,
            InstitutionalChannel::Aura,
            Some(AuraMaterial::Diamond),
        ),
    ];
    for (competency, channel, aura) in expected_competencies {
        let profile = catalog.stonebend_competency(competency).unwrap();
        assert_eq!(profile.channel, channel);
        assert_eq!(profile.aura_material, aura);
    }

    let freemason = catalog
        .stonebend_order(StonebendMastery::Freemason)
        .unwrap();
    assert_eq!(freemason.current_competency, StonebendCompetency::Mason);
    assert_eq!(freemason.aura_competency, StonebendCompetency::Mage);
    assert_eq!(freemason.cultural_identity, "Superstar");
    let proliteriate = catalog
        .stonebend_order(StonebendMastery::Proliteriate)
        .unwrap();
    assert_eq!(
        proliteriate.current_competency,
        StonebendCompetency::Pugilist
    );
    assert_eq!(proliteriate.aura_competency, StonebendCompetency::Foreman);
    assert_eq!(proliteriate.cultural_identity, "Rockstar");

    for mastery in StonebendMastery::ALL {
        let troll = resolve_stonebend_order_mastery(&catalog, "Troll", mastery).unwrap();
        assert_eq!(troll.nature_id, "troll");
        assert_eq!(troll.citizenship, StonebendCitizenship::Gerald);
        assert_eq!(troll.training_status, StonebendTrainingStatus::Apprentice);
        assert_eq!(troll.synthesis_mastery, mastery);
    }
    assert!(
        resolve_stonebend_order_mastery(&catalog, "Gargoyle", StonebendMastery::Freemason).is_err()
    );

    assert_eq!(hierarchy.apex.peer_inputs, StonebendMastery::ALL);
    assert_eq!(
        hierarchy.apex.required_cultural_identities,
        ["Superstar", "Rockstar"]
    );
    assert_eq!(hierarchy.apex.identity, "Hypergiant");
    assert!(catalog.nature("Hypergiant").is_none());
    assert_eq!(
        names(&catalog, "stonebend"),
        [
            "Gremlin",
            "Goblin",
            "Ghoul",
            "Spectre",
            "Troll",
            "Orc",
            "Ogre",
            "Troglodyte",
        ]
    );
    assert_eq!(hierarchy.philosophy.faculty, Dominion::Presynce);
    assert_eq!(hierarchy.philosophy.philosophy, "The Cut");
    assert_eq!(hierarchy.philosophy.process, "Hollowing");

    let rendered = render_institutions(&catalog, Some("stonebend")).unwrap();
    assert!(rendered.contains("Gerald"));
    assert!(!hierarchy.contains_active_identity("Mage"));
    assert!(hierarchy.contains_active_identity("Justice"));
    assert!(rendered.contains("Journeyman → Brickmason → Proliteriate"));
    assert!(rendered.contains("Craftsman → Artisan → Freemason"));
    assert!(!rendered.contains("Mage"));
    assert!(rendered.contains("Squire → Justice → Hypergiant"));
    assert!(rendered.contains("not a ninth Nature rung"));
}

#[test]
fn sandmanor_cultures_social_ladders_and_pairings_are_exact() {
    let catalog = canonical_hollow_deck();
    let hierarchy = &catalog.sandmanor_institution;
    assert_eq!(
        hierarchy
            .cultures
            .iter()
            .map(|path| path.culture)
            .collect::<Vec<_>>(),
        SandmanorCulture::ALL
    );

    let minorian = catalog
        .sandmanor_culture(SandmanorCulture::Minorian)
        .unwrap();
    assert_eq!(
        minorian
            .social_path
            .iter()
            .map(|stage| stage.identity)
            .collect::<Vec<_>>(),
        [
            SandmanorSocialIdentity::Hearthkeeper,
            SandmanorSocialIdentity::Householder,
            SandmanorSocialIdentity::Husbandman,
        ]
    );
    assert_eq!(
        minorian
            .social_path
            .iter()
            .map(|stage| stage.tier)
            .collect::<Vec<_>>(),
        [
            SandmanorMasteryTier::Baseline,
            SandmanorMasteryTier::Intermediate,
            SandmanorMasteryTier::Apex,
        ]
    );
    assert_eq!(
        catalog
            .sandmanor_social_stage(SandmanorSocialIdentity::Husbandman)
            .unwrap()
            .tier,
        SandmanorMasteryTier::Apex,
        "Husbandman is mature apex stewardship, never the entry farmer rank"
    );

    let minoan = catalog.sandmanor_culture(SandmanorCulture::Minoan).unwrap();
    assert_eq!(
        minoan
            .social_path
            .iter()
            .map(|stage| stage.identity)
            .collect::<Vec<_>>(),
        [
            SandmanorSocialIdentity::BeachBum,
            SandmanorSocialIdentity::PartyBoy,
            SandmanorSocialIdentity::Socialite,
        ]
    );
    assert_eq!(
        hierarchy
            .social_pairings
            .iter()
            .map(|pairing| (pairing.minorian, pairing.minoan))
            .collect::<Vec<_>>(),
        [
            (
                SandmanorSocialIdentity::Hearthkeeper,
                SandmanorSocialIdentity::BeachBum,
            ),
            (
                SandmanorSocialIdentity::Householder,
                SandmanorSocialIdentity::PartyBoy,
            ),
            (
                SandmanorSocialIdentity::Husbandman,
                SandmanorSocialIdentity::Socialite,
            ),
        ]
    );

    let mut stale_order = catalog.clone();
    stale_order.sandmanor_institution.cultures[0]
        .social_path
        .swap(0, 2);
    assert!(
        validate_hollow_deck(&stale_order).is_err(),
        "the retired Husbandman → Householder → Hearthkeeper order must fail validation"
    );
}

#[test]
fn sandmanor_aura_faculties_and_sovereign_synthesis_are_exact() {
    let catalog = canonical_hollow_deck();
    let hierarchy = &catalog.sandmanor_institution;
    let minorian = catalog
        .sandmanor_culture(SandmanorCulture::Minorian)
        .unwrap();
    let minoan = catalog.sandmanor_culture(SandmanorCulture::Minoan).unwrap();
    assert_eq!(minorian.faculty.faculty, Dominion::Prefog);
    assert_eq!(
        minorian.faculty.core_question,
        "What is going to be needed?"
    );
    assert_eq!(
        minorian.faculty.function,
        "Anticipates pressure, need, shortage, support requirements, and continuity demands."
    );
    assert_eq!(minoan.faculty.faculty, Dominion::Prefig);
    assert_eq!(minoan.faculty.core_question, "What is this becoming?");
    assert_eq!(
        minoan.faculty.function,
        "Anticipates form, trajectory, possibility, social movement, and emerging opportunity."
    );
    assert_eq!(
        minorian
            .aura_path
            .iter()
            .map(|stage| stage.identity)
            .collect::<Vec<_>>(),
        [
            SandmanorAuraMastery::Geomancer,
            SandmanorAuraMastery::Labyrinthian,
            SandmanorAuraMastery::Demiurge,
        ]
    );
    assert_eq!(
        minoan
            .aura_path
            .iter()
            .map(|stage| stage.identity)
            .collect::<Vec<_>>(),
        [
            SandmanorAuraMastery::Acolyte,
            SandmanorAuraMastery::Navigator,
            SandmanorAuraMastery::Ascendant,
        ]
    );
    let demiurge = catalog
        .sandmanor_aura_stage(SandmanorAuraMastery::Demiurge)
        .unwrap();
    let ascendant = catalog
        .sandmanor_aura_stage(SandmanorAuraMastery::Ascendant)
        .unwrap();
    assert_eq!(demiurge.tier, SandmanorMasteryTier::Apex);
    assert_eq!(ascendant.tier, SandmanorMasteryTier::Apex);
    assert!(demiurge.meaning.contains("functioning, sustainable world"));
    assert!(demiurge.meaning.contains("not omnipotent"));
    assert!(ascendant.meaning.contains("Exceeds the limits"));

    let synthesis = &hierarchy.sovereign_synthesis;
    assert_eq!(
        synthesis.peer_apexes,
        [
            SandmanorAuraMastery::Demiurge,
            SandmanorAuraMastery::Ascendant,
        ]
    );
    assert_eq!(
        synthesis.synthesis_identity,
        SandmanorSovereignIdentity::Transcendent
    );
    assert_eq!(
        synthesis.synthesis_formula,
        "Demiurge + Ascendant → The Transcendent"
    );
    assert_eq!(
        synthesis.sovereign_identity,
        SandmanorSovereignIdentity::Sandman
    );
    assert_eq!(
        synthesis.sovereign_embodiment_of,
        SandmanorSovereignIdentity::Transcendent
    );
    assert_eq!(
        synthesis.sovereign_formula,
        "The Transcendent → The Sandman"
    );

    let mut collapsed_apex = catalog.clone();
    collapsed_apex
        .sandmanor_institution
        .sovereign_synthesis
        .peer_apexes = vec![SandmanorAuraMastery::Demiurge];
    assert!(validate_hollow_deck(&collapsed_apex).is_err());
}

#[test]
fn sandmanor_nature_social_aura_and_sovereign_axes_remain_independent() {
    let catalog = canonical_hollow_deck();
    assert_ne!(
        TypeId::of::<Nature>(),
        TypeId::of::<SandmanorSocialIdentity>()
    );
    assert_ne!(
        TypeId::of::<SandmanorSocialIdentity>(),
        TypeId::of::<SandmanorAuraMastery>()
    );
    assert_eq!(
        names(&catalog, "sandmanor-minorian"),
        ["Gnome", "Faun", "Minotaur", "Hecaton"]
    );
    assert_eq!(
        names(&catalog, "sandmanor-minoan"),
        ["Elf", "Satyr", "Centaur", "Pegasus"]
    );
    assert_eq!(
        get_primary_trait(&catalog, "Hecaton").unwrap().name,
        "Multiplication"
    );
    assert_eq!(
        get_primary_trait(&catalog, "Pegasus").unwrap().name,
        "Flight"
    );
    let pegasus_wings = catalog
        .current_form("Pegasus")
        .unwrap()
        .current_features
        .iter()
        .find(|feature| feature.id == "wings")
        .unwrap();
    assert_eq!(pegasus_wings.stages, ["Buds", "Winglets", "Wings"]);

    for non_nature in [
        "Hearthkeeper",
        "Householder",
        "Husbandman",
        "Beach Bum",
        "Party Boy",
        "Socialite",
        "Geomancer",
        "Labyrinthian",
        "Demiurge",
        "Acolyte",
        "Navigator",
        "Ascendant",
        "The Transcendent",
        "The Sandman",
    ] {
        assert!(
            catalog.nature(non_nature).is_none(),
            "{non_nature} is not a Nature"
        );
        assert!(
            catalog.evolution_paths.iter().all(|path| {
                path.nature_ids
                    .iter()
                    .all(|nature_id| nature_id != non_nature)
            }),
            "{non_nature} cannot be inserted into a Nature path"
        );
    }
}

#[test]
fn sandmanor_board_uses_current_social_aura_and_sovereign_terms() {
    let catalog = canonical_hollow_deck();
    let direct = render_sandmanor(&catalog);
    let institutional = render_institutions(&catalog, Some("sandmanor")).unwrap();
    assert_eq!(
        direct, institutional,
        "sandmanor is one shared board projection"
    );
    for expected in [
        "Hearthkeeper → Householder → Husbandman",
        "Beach Bum → Party Boy → Socialite",
        "Geomancer → Labyrinthian → Demiurge",
        "Acolyte → Navigator → Ascendant",
        "Gnome → Faun → Minotaur → Hecaton",
        "Elf → Satyr → Centaur → Pegasus",
        "Demiurge + Ascendant → The Transcendent",
        "The Transcendent → The Sandman",
    ] {
        assert!(
            direct.contains(expected),
            "Sandmanor board missing {expected}"
        );
    }
    let current_aura_names = catalog
        .sandmanor_culture(SandmanorCulture::Minoan)
        .unwrap()
        .aura_path
        .iter()
        .map(|stage| stage.identity.as_str())
        .collect::<Vec<_>>();
    for retired in ["Witch", "Wizard", "Sorcerer", "Warlock"] {
        assert!(!current_aura_names.contains(&retired));
    }
}

#[test]
fn house_institutional_grammars_remain_structurally_asymmetric() {
    let catalog = canonical_hollow_deck();
    let all = render_institutions(&catalog, None).unwrap();
    assert!(all.contains("GLAÜSHOUSE  /  SEAT THEN COEQUAL BRANCH"));
    assert!(all.contains("FLYNT  /  FORMAL SERVICE"));
    assert!(all.contains("STONEBEND  /  THREE PATHS"));
    assert!(all.contains("SANDMANOR  /  TWO COMPLETE WAYS OF LIFE"));
    assert_eq!(
        catalog.glaushouse_structure.entry_title,
        "Nightingale de <Seat>"
    );
    assert_eq!(catalog.glaushouse_structure.branch_paths.len(), 2);
    assert_eq!(catalog.flynt_institution.sects.len(), 3);
    assert_eq!(catalog.stonebend_institution.orders.len(), 2);
    assert_eq!(catalog.sandmanor_institution.cultures.len(), 2);

    let serialized = serde_json::to_string(&catalog).unwrap();
    for axis in [
        "biology",
        "biological_growth",
        "citizenship",
        "institutional_service",
        "institutional_mastery",
        "specialist_discipline",
        "synthesis_apex",
        "operating_organization",
        "social_alias",
        "visual_presentation",
    ] {
        assert!(serialized.contains(&format!("\"{axis}\"")));
    }

    let mut invalid_flynt = catalog.clone();
    invalid_flynt.flynt_institution.sects[0].nature_id = "werewolf".into();
    assert!(validate_hollow_deck(&invalid_flynt).is_err());
    let mut invalid_stonebend = catalog.clone();
    invalid_stonebend.stonebend_institution.orders[0].aura_competency =
        StonebendCompetency::Foreman;
    assert!(validate_hollow_deck(&invalid_stonebend).is_err());
}

#[test]
fn evolution_orders_and_budgets_are_exact() {
    let catalog = canonical_hollow_deck();
    assert_eq!(
        names(&catalog, "stonebend"),
        [
            "Gremlin",
            "Goblin",
            "Ghoul",
            "Spectre",
            "Troll",
            "Orc",
            "Ogre",
            "Troglodyte"
        ]
    );
    assert_eq!(
        names(&catalog, "glaushouse-presensor"),
        ["Pixie", "Fairy", "Nymph", "Muse"]
    );
    assert_eq!(
        names(&catalog, "glaushouse-preceptor"),
        ["Sprite", "Spirit", "Sylph", "Siren"]
    );
    assert!(catalog.evolution_path("glaushouse").is_none());
    let obsolete_chain = [
        "pixie", "sprite", "fairy", "spirit", "sylph", "nymph", "siren", "muse",
    ];
    assert!(
        catalog
            .evolution_paths
            .iter()
            .all(|path| path.nature_ids != obsolete_chain),
        "the former interleaved eight-form chain is not canonical"
    );
    assert_eq!(
        names(&catalog, "sandmanor-minorian"),
        ["Gnome", "Faun", "Minotaur", "Hecaton"]
    );
    assert_eq!(
        names(&catalog, "sandmanor-minoan"),
        ["Elf", "Satyr", "Centaur", "Pegasus"]
    );

    let path = catalog.evolution_path("stonebend").unwrap();
    assert_eq!(
        path.nature_ids
            .iter()
            .map(|id| catalog.nature(id).unwrap().mature_stats.computed_total())
            .collect::<Vec<_>>(),
        [220, 235, 250, 265, 280, 295, 310, 320]
    );
    for path_id in ["sandmanor-minorian", "sandmanor-minoan"] {
        let path = catalog.evolution_path(path_id).unwrap();
        assert_eq!(
            path.nature_ids
                .iter()
                .map(|id| catalog.nature(id).unwrap().mature_stats.computed_total())
                .collect::<Vec<_>>(),
            [220, 250, 280, 320]
        );
    }
    assert!(get_evolution_path(&catalog, "Gargoyle").is_none());
}

#[test]
fn cross_house_counterparts_keep_separate_ownership_and_matched_budgets() {
    let catalog = canonical_hollow_deck();
    let expected = [
        ("Gremlin", "Pixie", 220),
        ("Goblin", "Sprite", 235),
        ("Ghoul", "Fairy", 250),
        ("Spectre", "Spirit", 265),
        ("Troll", "Sylph", 280),
        ("Orc", "Nymph", 295),
        ("Ogre", "Siren", 310),
        ("Troglodyte", "Muse", 320),
    ];
    for (left, right, budget) in expected {
        let counterpart = get_cross_house_counterpart(&catalog, left).expect("counterpart");
        assert_eq!(counterpart.name, right);
        assert_eq!(get_house_for_nature(&catalog, left), Some(House::Stonebend));
        assert_eq!(counterpart.house_id, House::Glaushouse);
        assert_eq!(counterpart.mature_stats.computed_total(), budget);
    }
}

#[test]
fn five_materials_share_four_alchemy_slots_with_exact_relationships() {
    assert_eq!(
        AuraMaterial::Jade.alchemy_slot(),
        AuraMaterial::Emerald.alchemy_slot()
    );
    for (source, target) in [
        (AuraMaterial::Diamond, AuraMaterial::Jade),
        (AuraMaterial::Diamond, AuraMaterial::Emerald),
        (AuraMaterial::Jade, AuraMaterial::Opal),
        (AuraMaterial::Emerald, AuraMaterial::Opal),
        (AuraMaterial::Opal, AuraMaterial::Crystal),
        (AuraMaterial::Crystal, AuraMaterial::Diamond),
    ] {
        assert_eq!(get_aura_matchup(source, target), AuraMatchup::Favorable);
        assert_eq!(get_aura_matchup(target, source), AuraMatchup::Unfavorable);
    }
    for (left, right) in [
        (AuraMaterial::Jade, AuraMaterial::Emerald),
        (AuraMaterial::Jade, AuraMaterial::Crystal),
        (AuraMaterial::Emerald, AuraMaterial::Crystal),
        (AuraMaterial::Diamond, AuraMaterial::Opal),
    ] {
        assert_eq!(get_aura_matchup(left, right), AuraMatchup::Neutral);
        assert_eq!(get_aura_matchup(right, left), AuraMatchup::Neutral);
    }
    assert_eq!(
        get_aura_matchup(AuraMaterial::Diamond, AuraMaterial::Opal),
        AuraMatchup::Neutral
    );
    assert_eq!(
        get_aura_matchup(AuraMaterial::Crystal, AuraMaterial::Jade),
        AuraMatchup::Neutral
    );
    for material in AuraMaterial::ALL {
        assert_eq!(get_aura_matchup(material, material), AuraMatchup::Neutral);
    }
    assert_eq!(
        get_applicable_aura_modifier(
            AuraMaterial::Diamond,
            AuraMaterial::Jade,
            EffectChannel::Aura
        ),
        1.10
    );
    assert_eq!(
        get_applicable_aura_modifier(
            AuraMaterial::Diamond,
            AuraMaterial::Jade,
            EffectChannel::CurrentPhysical
        ),
        1.00
    );
}

#[test]
fn current_form_and_aura_set_are_independent_types_and_states() {
    let catalog = canonical_hollow_deck();
    let form = catalog.current_form("Gargoyle").unwrap().clone();
    let set = catalog.aura_set("Gargoyle").unwrap().clone();
    let form_without_aura: (CurrentForm, Option<AuraSet>) = (form, None);
    let aura_without_form: (Option<CurrentForm>, AuraSet) = (None, set);
    assert!(form_without_aura.1.is_none());
    assert!(aura_without_form.0.is_none());
    assert_eq!(form_without_aura.0.nature_id, aura_without_form.1.nature_id);
}

#[test]
fn ordinary_and_synthesis_recipes_are_distinct_and_requirements_can_be_one_sided() {
    assert_ne!(TypeId::of::<Recipe>(), TypeId::of::<SynthesisRecipe>());
    let catalog = canonical_hollow_deck();
    let current_only = catalog.synthesis_recipes.first().unwrap();
    assert!(current_only.required_current_features.is_empty());
    assert!(current_only.required_growth_stage.is_some());
    assert!(current_only.required_aura_moves.is_empty());
    let report = validate_synthesis_requirements(
        current_only,
        "gargoyle",
        catalog.current_form("gargoyle"),
        None,
        &[],
    );
    assert!(report.compatible);
    assert!(report.missing_current_features.is_empty());
    assert!(report.missing_aura_moves.is_empty());

    let aura_only = SynthesisRecipe {
        id: "test-aura-only".into(),
        name: "Test Aura-only Pattern".into(),
        description: String::new(),
        source_type: SynthesisRecipeSourceType::Pattern,
        source_description: String::new(),
        is_discovered: true,
        rarity: None,
        unique_source: false,
        compatible_natures: vec!["werewolf".into()],
        required_current_features: Vec::new(),
        required_current_techniques: Vec::new(),
        required_growth_stage: None,
        required_aura_moves: vec!["scent".into()],
        optional_reagents: Vec::new(),
        consumed_reagents: Vec::new(),
        result_type: SynthesisResultType::Ability,
        result_synthesis: "test".into(),
        mechanical_effect: String::new(),
        lore_text: String::new(),
        tags: Vec::new(),
    };
    let learned = AuraSet {
        nature_id: "werewolf".into(),
        learned_moves: vec!["scent".into()],
        equipped_moves: vec!["scent".into()],
        design_examples: Vec::new(),
    };
    let report = validate_synthesis_requirements(&aura_only, "werewolf", None, Some(&learned), &[]);
    assert!(report.compatible);
    assert!(report.missing_current_features.is_empty());
    assert!(report.missing_aura_moves.is_empty());
}

#[test]
fn gargoyle_growth_is_ordered_cumulative_and_gates_winged_gargoyle() {
    let catalog = canonical_hollow_deck();
    let path = catalog
        .growth_path("gargoyle")
        .expect("Gargoyle growth path");
    let ids = path
        .milestones
        .iter()
        .map(|stage| stage.id.as_str())
        .collect::<Vec<_>>();
    let names = path
        .milestones
        .iter()
        .map(|stage| stage.name.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        names,
        [
            "Fangs",
            "Claws",
            "Horns",
            "Wing Buds",
            "Winglets",
            "Wings",
            "Mature Fortification",
            "White Opal Statue Mastery",
        ]
    );
    assert_eq!(achieved_growth_stages(path, "Wings").unwrap(), ids[..=5]);
    assert_eq!(
        achieved_growth_stages(path, "White Opal Statue Mastery").unwrap(),
        ids
    );

    assert!(
        catalog
            .syntheses
            .iter()
            .all(|synthesis| synthesis.id != "white-opal-statue")
    );
    let recipe = catalog
        .synthesis_recipes
        .iter()
        .find(|recipe| recipe.name == "Winged Gargoyle")
        .expect("Winged Gargoyle recipe");
    assert_eq!(
        recipe
            .required_growth_stage
            .as_ref()
            .map(|requirement| requirement.milestone_id.as_str()),
        Some("wings")
    );
    assert!(recipe.required_current_features.is_empty());

    for (stage, should_pass) in [
        ("winglets", false),
        ("wings", true),
        ("mature-fortification", true),
        ("white-opal-statue-mastery", true),
    ] {
        let mut form = catalog.current_form("gargoyle").unwrap().clone();
        form.current_growth_stage = Some(stage.into());
        form.achieved_growth_stages = achieved_growth_stages(path, stage).unwrap();
        let report = validate_synthesis_requirements(recipe, "gargoyle", Some(&form), None, &[]);
        assert_eq!(
            report.missing_growth_stage.is_none(),
            should_pass,
            "{stage}"
        );
    }
}

#[test]
fn transformed_stats_interpolate_without_changing_targets() {
    let catalog = canonical_hollow_deck();
    assert_eq!(
        compute_transformed_stats(&catalog, "Gargoyle", 0).unwrap(),
        HUEMAN_STATS
    );
    assert_eq!(
        compute_transformed_stats(&catalog, "Gargoyle", 10_000).unwrap(),
        catalog.nature("Gargoyle").unwrap().mature_stats
    );
    assert_eq!(
        compute_transformed_stats(&catalog, "Goblin", 0).unwrap(),
        catalog.nature("Gremlin").unwrap().mature_stats
    );
    assert!(compute_transformed_stats(&catalog, "Gargoyle", 10_001).is_err());
}

#[test]
fn hollow_deck_views_use_canonical_terms_and_keep_recipe_surfaces_separate() {
    let catalog = canonical_hollow_deck();
    let home = render_home(&catalog);
    assert!(home.contains("HOLLOW DECK"));
    assert!(home.contains("GLAÜSHOUSE"));
    assert!(home.contains("Jade"));

    let index = render_nature_index(
        &catalog,
        &NatureQuery {
            house: Some("Flynt".into()),
            principle: Some("Stillness".into()),
            ..NatureQuery::default()
        },
    );
    assert!(index.contains("Gargoyle"));
    assert!(index.contains("Medusa"));
    assert!(!index.contains("Werewolf"));

    let card = render_nature_card(&catalog, "Gargoyle").unwrap();
    for section in [
        "CURRENT FORM",
        "AURA SET",
        "EVOLUTION / PROGRESSION",
        "SYNTHESIS RECIPES",
        "BANES",
        "ALCHEMY MATCHUP",
    ] {
        assert!(card.contains(section), "Nature card missing {section}");
    }
    assert!(card.contains("White Opal Statue"));
    assert!(render_progression(&catalog).contains(&["Manticorp", "Chimera"].join(" ↔ ")));
    assert!(render_counterparts(&catalog).contains("DIFFERENT Houses"));
    assert!(
        render_compare(&catalog, "Gargoyle", "Medusa")
            .unwrap()
            .contains("Stillness")
    );
    assert!(render_alchemy().contains("Glaüshouse (Jade / Emerald)"));
    assert!(render_recipes(&catalog).contains("Ordinary real-world crafting"));
    assert!(render_synthesis_recipes(&catalog).contains("Collectible supernatural patterns"));
}
