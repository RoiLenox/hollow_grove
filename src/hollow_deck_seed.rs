//! Canonical seed projection for [`crate::hollow_deck`].

use crate::hollow_deck::*;
use crate::hollow_grove_contract::House;

pub(crate) fn seed_catalog() -> HollowDeckCatalog {
    let houses = vec![
        HouseEntry {
            id: House::Stonebend,
            display_name: "Stonebend".into(),
            dominions: vec![Dominion::Presynce],
            current_type: CurrentType::Hollow,
            aura_materials: vec![AuraMaterial::Diamond],
            direction: Some("North".into()),
            organization: "8-form progression".into(),
        },
        HouseEntry {
            id: House::Sandmanor,
            display_name: "Sandmanor".into(),
            dominions: vec![Dominion::Prefog, Dominion::Prefig],
            current_type: CurrentType::Regular,
            aura_materials: vec![AuraMaterial::Crystal],
            direction: Some("East".into()),
            organization: "2 complete parallel cultural paths".into(),
        },
        HouseEntry {
            id: House::Glaushouse,
            display_name: "Glaüshouse".into(),
            dominions: vec![Dominion::Precog],
            current_type: CurrentType::Hollow,
            aura_materials: vec![AuraMaterial::Jade, AuraMaterial::Emerald],
            direction: Some("South".into()),
            organization: "5 seats with 2 equal Precog branch progressions".into(),
        },
        HouseEntry {
            id: House::Flynt,
            display_name: "Flynt".into(),
            dominions: vec![Dominion::Resynce],
            current_type: CurrentType::Regular,
            aura_materials: vec![AuraMaterial::Opal],
            direction: Some("West".into()),
            organization: "4 reciprocal Nature pairs".into(),
        },
    ];

    let glaushouse_structure = GlaushouseStructure {
        direction: "South".into(),
        umbrella_faculty: Dominion::Precog,
        seats: GlaushouseSeat::ALL.to_vec(),
        entry_title: "Nightingale de <Seat>".into(),
        branch_paths: vec![
            GlaushouseBranchPath {
                branch: GlaushouseBranch::Presensor,
                evolution_path_id: "glaushouse-presensor".into(),
            },
            GlaushouseBranchPath {
                branch: GlaushouseBranch::Preceptor,
                evolution_path_id: "glaushouse-preceptor".into(),
            },
        ],
    };
    let institutional_axes = institutional_axis_model();
    let sandmanor_institution = sandmanor_institutional_hierarchy();
    let flynt_institution = flynt_institutional_hierarchy();
    let stonebend_institution = stonebend_institutional_hierarchy();

    let primary_traits = vec![
        trait_entry(
            "scavenging",
            "Scavenging",
            "Exploit loose objects, debris, and openings.",
        ),
        trait_entry(
            "rigging",
            "Rigging",
            "Use the environment as traps, tools, and mechanisms.",
        ),
        trait_entry(
            "devouring",
            "Devouring",
            "Convert physical material or resources into endurance.",
        ),
        trait_entry(
            "phasing",
            "Phasing",
            "Partially bypass physical boundaries.",
        ),
        trait_entry(
            "regeneration",
            "Regeneration",
            "Rebuild a damaged Hollow Current and Diamond form.",
        ),
        trait_entry(
            "sundering",
            "Sundering",
            "Break guards, armor, and structure.",
        ),
        trait_entry(
            "crushing",
            "Crushing",
            "Apply overwhelming physical force and displacement.",
        ),
        trait_entry(
            "hollowing",
            "Hollowing",
            "Manipulate cavities, internal space, mass distribution, Hollow Current, and absence within matter.",
        ),
        trait_entry("detection", "Detection", "Notice what matters."),
        trait_entry("prediction", "Prediction", "Anticipate what comes next."),
        trait_entry("mending", "Mending", "Perform localized repair."),
        trait_entry(
            "projection",
            "Projection",
            "Extend awareness beyond normal boundaries.",
        ),
        trait_entry(
            "deflection",
            "Deflection",
            "Redirect incoming action or force.",
        ),
        trait_entry(
            "restoration",
            "Restoration",
            "Restore integrity at greater depth.",
        ),
        trait_entry("compulsion", "Compulsion", "Impose direction or decision."),
        trait_entry("hope", "Hope", "Create possibility."),
        trait_entry(
            "grip",
            "Grip",
            "Develop working hands and secure physical grip.",
        ),
        trait_entry(
            "traction",
            "Traction",
            "Develop footing and controlled contact with the ground.",
        ),
        trait_entry(
            "leverage",
            "Leverage",
            "Coordinate the whole body around mechanical advantage.",
        ),
        trait_entry(
            "multiplication",
            "Multiplication",
            "Multiply working anatomy, especially arms, hands, and work points.",
        ),
        trait_entry(
            "balance",
            "Balance",
            "Maintain bodily equilibrium and poised movement.",
        ),
        trait_entry(
            "agility",
            "Agility",
            "Develop specialized legs and responsive movement.",
        ),
        trait_entry(
            "momentum",
            "Momentum",
            "Build and direct stride-driven motion.",
        ),
        trait_entry(
            "flight",
            "Flight",
            "Achieve vertical mobility and sustained flight.",
        ),
        trait_entry(
            "fortification",
            "Fortification",
            "Voluntarily control one's own stillness for protection, rest, recovery, and defensive commitment.",
        ),
        trait_entry(
            "petrification",
            "Petrification",
            "Impose stillness onto another body.",
        ),
        trait_entry(
            "siphone",
            "Siphone",
            "Build and accelerate Current through connection and physical action.",
        ),
        trait_entry(
            "transfusion",
            "Transfusion",
            "Drain, store, transfer, or inject Current between bodies.",
        ),
        trait_entry(
            "anchoring",
            "Anchoring",
            "Control or fix physical position relative to oneself.",
        ),
        trait_entry(
            "attraction",
            "Attraction",
            "Influence where another being wants to move or focus attention.",
        ),
        trait_entry(
            "convergence",
            "Convergence",
            "Coordinate many parts or systems into one controlled configuration.",
        ),
        trait_entry(
            "divergence",
            "Divergence",
            "Redistribute one body's specialization among multiple configurations without exceeding its total budget.",
        ),
    ];

    let natures = vec![
        nature(
            "gremlin",
            "Gremlin",
            House::Stonebend,
            Dominion::Presynce,
            "scavenging",
            StatBlock::new(55, 70, 50, 45),
            Some(("stonebend", 0)),
            None,
            None,
            None,
            Some("Exploit loose objects, debris, and openings."),
        ),
        nature(
            "goblin",
            "Goblin",
            House::Stonebend,
            Dominion::Presynce,
            "rigging",
            StatBlock::new(58, 68, 60, 49),
            Some(("stonebend", 1)),
            None,
            None,
            None,
            Some("Use the environment as traps, tools, and mechanisms."),
        ),
        nature(
            "ghoul",
            "Ghoul",
            House::Stonebend,
            Dominion::Presynce,
            "devouring",
            StatBlock::new(66, 52, 63, 69),
            Some(("stonebend", 2)),
            None,
            None,
            None,
            Some("Convert physical material and resources into endurance."),
        ),
        nature(
            "spectre",
            "Spectre",
            House::Stonebend,
            Dominion::Presynce,
            "phasing",
            StatBlock::new(45, 88, 65, 67),
            Some(("stonebend", 3)),
            None,
            None,
            Some("Clear Hollow-Current body with Diamond structural anchors."),
            Some("Partially bypass physical boundaries."),
        ),
        nature(
            "troll",
            "Troll",
            House::Stonebend,
            Dominion::Presynce,
            "regeneration",
            StatBlock::new(78, 42, 65, 95),
            Some(("stonebend", 4)),
            None,
            None,
            None,
            Some("Rebuild damaged Hollow Current and Diamond form."),
        ),
        nature(
            "orc",
            "Orc",
            House::Stonebend,
            Dominion::Presynce,
            "sundering",
            StatBlock::new(88, 62, 95, 50),
            Some(("stonebend", 5)),
            None,
            None,
            None,
            Some("Break guards, armor, and structure."),
        ),
        nature(
            "ogre",
            "Ogre",
            House::Stonebend,
            Dominion::Presynce,
            "crushing",
            StatBlock::new(100, 45, 95, 70),
            Some(("stonebend", 6)),
            None,
            None,
            None,
            Some("Overwhelming physical force and displacement."),
        ),
        nature(
            "troglodyte",
            "Troglodyte",
            House::Stonebend,
            Dominion::Presynce,
            "hollowing",
            StatBlock::new(95, 45, 95, 85),
            Some(("stonebend", 7)),
            None,
            None,
            None,
            Some("Hollowing creates absence."),
        ),
        nature(
            "pixie",
            "Pixie",
            House::Glaushouse,
            Dominion::Precog,
            "detection",
            StatBlock::new(35, 85, 60, 40),
            Some(("glaushouse-presensor", 0)),
            None,
            None,
            None,
            Some("Notice what matters."),
        ),
        nature(
            "sprite",
            "Sprite",
            House::Glaushouse,
            Dominion::Precog,
            "prediction",
            StatBlock::new(36, 88, 70, 41),
            Some(("glaushouse-preceptor", 0)),
            None,
            None,
            None,
            Some("Anticipate what comes next."),
        ),
        nature(
            "fairy",
            "Fairy",
            House::Glaushouse,
            Dominion::Precog,
            "mending",
            StatBlock::new(40, 82, 58, 70),
            Some(("glaushouse-presensor", 1)),
            None,
            None,
            None,
            Some("Localized repair."),
        ),
        nature(
            "spirit",
            "Spirit",
            House::Glaushouse,
            Dominion::Precog,
            "projection",
            StatBlock::new(35, 78, 87, 65),
            Some(("glaushouse-preceptor", 1)),
            None,
            None,
            None,
            Some("Extend awareness beyond normal boundaries."),
        ),
        nature(
            "sylph",
            "Sylph",
            House::Glaushouse,
            Dominion::Precog,
            "deflection",
            StatBlock::new(42, 100, 68, 70),
            Some(("glaushouse-preceptor", 2)),
            None,
            None,
            None,
            Some("Redirect incoming action or force."),
        ),
        nature(
            "nymph",
            "Nymph",
            House::Glaushouse,
            Dominion::Precog,
            "restoration",
            StatBlock::new(50, 80, 70, 95),
            Some(("glaushouse-presensor", 2)),
            None,
            None,
            None,
            Some("Deeper restoration of integrity."),
        ),
        nature(
            "siren",
            "Siren",
            House::Glaushouse,
            Dominion::Precog,
            "compulsion",
            StatBlock::new(45, 85, 100, 80),
            Some(("glaushouse-preceptor", 3)),
            None,
            None,
            None,
            Some("Impose direction or decision."),
        ),
        nature(
            "muse",
            "Muse",
            House::Glaushouse,
            Dominion::Precog,
            "hope",
            StatBlock::new(45, 95, 85, 95),
            Some(("glaushouse-presensor", 3)),
            None,
            None,
            None,
            Some("Hope creates possibility."),
        ),
        nature(
            "gnome",
            "Gnome",
            House::Sandmanor,
            Dominion::Prefog,
            "grip",
            StatBlock::new(48, 58, 54, 60),
            Some(("sandmanor-minorian", 0)),
            None,
            None,
            Some("Working hands and grip."),
            None,
        ),
        nature(
            "faun",
            "Faun",
            House::Sandmanor,
            Dominion::Prefog,
            "traction",
            StatBlock::new(62, 75, 60, 53),
            Some(("sandmanor-minorian", 1)),
            None,
            None,
            Some("Footing and traction."),
            None,
        ),
        nature(
            "minotaur",
            "Minotaur",
            House::Sandmanor,
            Dominion::Prefog,
            "leverage",
            StatBlock::new(92, 58, 72, 58),
            Some(("sandmanor-minorian", 2)),
            None,
            None,
            Some("Whole-body leverage."),
            None,
        ),
        nature(
            "hecaton",
            "Hecaton",
            House::Sandmanor,
            Dominion::Prefog,
            "multiplication",
            StatBlock::new(100, 55, 90, 75),
            Some(("sandmanor-minorian", 3)),
            None,
            None,
            Some("Multiple arms, hands, and work points."),
            Some("Hecaton is multiplication of working anatomy, not merely a larger Minotaur."),
        ),
        nature(
            "elf",
            "Elf",
            House::Sandmanor,
            Dominion::Prefig,
            "balance",
            StatBlock::new(44, 70, 60, 46),
            Some(("sandmanor-minoan", 0)),
            None,
            None,
            Some("Balance."),
            None,
        ),
        nature(
            "satyr",
            "Satyr",
            House::Sandmanor,
            Dominion::Prefig,
            "agility",
            StatBlock::new(54, 82, 68, 46),
            Some(("sandmanor-minoan", 1)),
            None,
            None,
            Some("Specialized legs and agility."),
            None,
        ),
        nature(
            "centaur",
            "Centaur",
            House::Sandmanor,
            Dominion::Prefig,
            "momentum",
            StatBlock::new(75, 92, 70, 43),
            Some(("sandmanor-minoan", 2)),
            None,
            None,
            Some("Stride and momentum."),
            None,
        ),
        nature(
            "pegasus",
            "Pegasus",
            House::Sandmanor,
            Dominion::Prefig,
            "flight",
            StatBlock::new(65, 100, 95, 60),
            Some(("sandmanor-minoan", 3)),
            None,
            None,
            Some("Wings and vertical mobility."),
            None,
        ),
        nature(
            "gargoyle",
            "Gargoyle",
            House::Flynt,
            Dominion::Resynce,
            "fortification",
            StatBlock::new(89, 44, 87, 100),
            None,
            Some("medusa"),
            Some("Stillness"),
            Some(
                "Dark void-blue Regular Current body with pale or white Opal hands, claws, talons, fangs, horns, wing structures, and other specialized appendages. The full-white body is reserved for White Opal Statue.",
            ),
            Some("Gargoyle chooses its own stillness."),
        ),
        nature(
            "medusa",
            "Medusa",
            House::Flynt,
            Dominion::Resynce,
            "petrification",
            StatBlock::new(55, 75, 100, 90),
            None,
            Some("gargoyle"),
            Some("Stillness"),
            None,
            Some("Medusa imposes stillness onto others."),
        ),
        nature(
            "werewolf",
            "Werewolf",
            House::Flynt,
            Dominion::Resynce,
            "siphone",
            StatBlock::new(95, 82, 90, 53),
            None,
            Some("vampire"),
            Some("Circulation"),
            Some("Claws, jaws, pursuit legs, musculature, Opal working surfaces, and Opal joints."),
            Some("Find → pursue → connect → Siphone → escalate."),
        ),
        nature(
            "vampire",
            "Vampire",
            House::Flynt,
            Dominion::Resynce,
            "transfusion",
            StatBlock::new(60, 100, 96, 64),
            None,
            Some("werewolf"),
            Some("Circulation"),
            Some(
                "Sleek void-blue body with precise Opal fangs, nails, eyes, and vein-like accents.",
            ),
            Some(
                "Transfusion can drain Siphone pressure; excessive Werewolf Current may cause configurable Overfill instability.",
            ),
        ),
        nature(
            "mariner",
            "Mariner",
            House::Flynt,
            Dominion::Resynce,
            "anchoring",
            StatBlock::new(90, 60, 75, 95),
            None,
            Some("mermaid"),
            Some("Position"),
            Some(
                "Aquatic propulsion, grip, leverage, swimming anatomy, displacement resistance, dragging, and grappling.",
            ),
            Some(
                "I decide where I remain; later, I determine where something remains relative to me.",
            ),
        ),
        nature(
            "mermaid",
            "Mermaid",
            House::Flynt,
            Dominion::Resynce,
            "attraction",
            StatBlock::new(55, 100, 95, 70),
            None,
            Some("mariner"),
            Some("Position"),
            None,
            Some("Influences where another being wants to move or focus attention."),
        ),
        nature(
            "manticorp",
            "Manticorp",
            House::Flynt,
            Dominion::Resynce,
            "convergence",
            StatBlock::new(85, 70, 85, 80),
            None,
            Some("chimera"),
            Some("Form"),
            None,
            Some(
                "Many become one; coordinates multiple parts or systems into one controlled configuration.",
            ),
        ),
        nature(
            "chimera",
            "Chimera",
            House::Flynt,
            Dominion::Resynce,
            "divergence",
            StatBlock::new(70, 80, 90, 80),
            None,
            Some("manticorp"),
            Some("Form"),
            None,
            Some("One becomes many; temporary redistribution never exceeds the 320-point total."),
        ),
    ];

    let evolution_paths = vec![
        path(
            "stonebend",
            "Stonebend",
            House::Stonebend,
            Dominion::Presynce,
            &[
                "gremlin",
                "goblin",
                "ghoul",
                "spectre",
                "troll",
                "orc",
                "ogre",
                "troglodyte",
            ],
        ),
        path(
            "glaushouse-presensor",
            "Glaüshouse — Presensor / Matron / Jade",
            House::Glaushouse,
            Dominion::Precog,
            &["pixie", "fairy", "nymph", "muse"],
        ),
        path(
            "glaushouse-preceptor",
            "Glaüshouse — Preceptor / Marshall / Emerald",
            House::Glaushouse,
            Dominion::Precog,
            &["sprite", "spirit", "sylph", "siren"],
        ),
        path(
            "sandmanor-minorian",
            "Sandmanor — Minorian",
            House::Sandmanor,
            Dominion::Prefog,
            &["gnome", "faun", "minotaur", "hecaton"],
        ),
        path(
            "sandmanor-minoan",
            "Sandmanor — Minoan",
            House::Sandmanor,
            Dominion::Prefig,
            &["elf", "satyr", "centaur", "pegasus"],
        ),
    ];

    let pair_relationships = vec![
        PairRelationship {
            nature_a: "gargoyle".into(),
            nature_b: "medusa".into(),
            shared_principle: "Stillness".into(),
            primary_trait_a: "fortification".into(),
            primary_trait_b: "petrification".into(),
            special_interactions: vec![SpecialInteraction {
                name: "White Opal Statue control".into(),
                mechanical_effect: "Medusa can force Gargoyle into White Opal Statue, removing agency and denying the restoration granted by voluntary Fortification.".into(),
                counterplay: Some("Gargoyle may voluntarily Fortify before Petrification resolves.".into()),
                configurable_notes: None,
            }],
        },
        PairRelationship {
            nature_a: "werewolf".into(),
            nature_b: "vampire".into(),
            shared_principle: "Circulation".into(),
            primary_trait_a: "siphone".into(),
            primary_trait_b: "transfusion".into(),
            special_interactions: vec![SpecialInteraction {
                name: "Siphone pressure and Overfill".into(),
                mechanical_effect: "Transfusion can drain Siphone pressure, but excessive Werewolf Current can overfill and destabilize the Vampire.".into(),
                counterplay: None,
                configurable_notes: Some("Exact Overfill thresholds and penalties remain configurable/TODO.".into()),
            }],
        },
        PairRelationship {
            nature_a: "mariner".into(),
            nature_b: "mermaid".into(),
            shared_principle: "Position".into(),
            primary_trait_a: "anchoring".into(),
            primary_trait_b: "attraction".into(),
            special_interactions: vec![SpecialInteraction {
                name: "Fixed and drawn position".into(),
                mechanical_effect: "Anchoring fixes position while Attraction draws position or attention.".into(),
                counterplay: None,
                configurable_notes: None,
            }],
        },
        PairRelationship {
            nature_a: "manticorp".into(),
            nature_b: "chimera".into(),
            shared_principle: "Form".into(),
            primary_trait_a: "convergence".into(),
            primary_trait_b: "divergence".into(),
            special_interactions: vec![SpecialInteraction {
                name: "Configuration lock".into(),
                mechanical_effect: "Convergence may later lock Chimera into a chosen configuration; Divergence may redistribute but never increase the 320-point budget.".into(),
                counterplay: None,
                configurable_notes: Some("Exact timing and lock rules remain TODO.".into()),
            }],
        },
    ];

    let budgets = [220, 235, 250, 265, 280, 295, 310, 320];
    let stonebend = [
        "gremlin",
        "goblin",
        "ghoul",
        "spectre",
        "troll",
        "orc",
        "ogre",
        "troglodyte",
    ];
    let glaushouse = [
        "pixie", "sprite", "fairy", "spirit", "sylph", "nymph", "siren", "muse",
    ];
    let cross_house_counterparts = stonebend
        .iter()
        .zip(glaushouse)
        .zip(budgets)
        .enumerate()
        .map(|(index, ((left, right), budget))| CrossHouseCounterpart {
            nature_a: (*left).into(),
            nature_b: right.into(),
            tier: index + 1,
            stat_budget: budget,
            conceptual_contrast: (index == 7)
                .then(|| "Hollowing creates absence; Hope creates possibility.".into()),
        })
        .collect::<Vec<_>>();

    let flynt_visual_identities = flynt_visual_identity_profiles();
    let flynt_visual_languages = flynt_institutional_visual_languages();

    let mut current_forms = natures
        .iter()
        .map(|nature| CurrentForm {
            nature_id: nature.id.clone(),
            current_features: Vec::new(),
            current_techniques: Vec::new(),
            growth_path_id: None,
            current_growth_stage: None,
            achieved_growth_stages: Vec::new(),
            development_basis_points: 10_000,
            computed_stats: nature.mature_stats,
        })
        .collect::<Vec<_>>();
    let growth_paths = vec![UnifiedGrowthPath {
        id: "gargoyle-unified-growth".into(),
        nature_id: "gargoyle".into(),
        name: "Gargoyle Unified Biological Cultivation".into(),
        milestones: vec![
            growth(
                "fangs",
                "Fangs",
                "White Opal fangs emerge from the void-blue Current body.",
            ),
            growth(
                "claws",
                "Claws",
                "White Opal claws become functional working anatomy.",
            ),
            growth(
                "horns",
                "Horns",
                "White Opal horns complete the early specialized anatomy.",
            ),
            growth(
                "wing-buds",
                "Wing Buds",
                "Developed wing roots and white Opal structural buds emerge.",
            ),
            growth(
                "winglets",
                "Winglets",
                "Wing roots extend into inherited white Opal winglets.",
            ),
            growth(
                "wings",
                "Wings",
                "Mature wings use a white Opal structural frame with void-blue Current membrane.",
            ),
            growth(
                "mature-fortification",
                "Mature Fortification",
                "Fortification can overtake the cultivated mobile anatomy in controlled stillness.",
            ),
            growth(
                "white-opal-statue-mastery",
                "White Opal Statue Mastery",
                "The entire body, including mature wing structures and membranes as appropriate, becomes full white Opal for voluntary protection, rest, and restoration.",
            ),
        ],
    }];
    let gargoyle_growth = &growth_paths[0];
    let gargoyle_form = current_forms
        .iter_mut()
        .find(|form| form.nature_id == "gargoyle")
        .expect("seeded Gargoyle Current Form");
    gargoyle_form.growth_path_id = Some(gargoyle_growth.id.clone());
    gargoyle_form.current_growth_stage = Some("white-opal-statue-mastery".into());
    gargoyle_form.achieved_growth_stages = gargoyle_growth
        .milestones
        .iter()
        .map(|milestone| milestone.id.clone())
        .collect();
    set_form(
        &mut current_forms,
        "gargoyle",
        vec![
            feature(
                "wings",
                "Wings",
                CurrentFeatureCategory::Locomotion,
                &[],
                "Inherited anatomy: white Opal structural frame with void-blue Current membrane. Stage is owned by the unified growth path.",
            ),
            feature(
                "claws",
                "Claws",
                CurrentFeatureCategory::WorkingSurface,
                &[],
                "White Opal claws.",
            ),
            feature(
                "fangs",
                "Fangs",
                CurrentFeatureCategory::Anatomy,
                &[],
                "White Opal fangs.",
            ),
            feature(
                "horns",
                "Horns",
                CurrentFeatureCategory::Anatomy,
                &[],
                "White Opal horns.",
            ),
        ],
        vec![],
    );
    set_form(&mut current_forms, "werewolf", vec![
        feature("claws", "Claws", CurrentFeatureCategory::WorkingSurface, &[], "Opal working surfaces."),
        feature("jaws", "Jaws", CurrentFeatureCategory::Anatomy, &[], "Hunting anatomy."),
        feature("pursuit-legs", "Pursuit Legs", CurrentFeatureCategory::Locomotion, &[], "Pursuit anatomy and Opal joints."),
        feature("musculature", "Musculature", CurrentFeatureCategory::Structure, &[], "Circulation-driven hunting musculature."),
    ], vec![CurrentTechnique {
        id: "siphone".into(),
        name: "Siphone".into(),
        stages: strings(&["Siphone", "Pump", "Surge", "Rush", "Frenzy"]),
        current_stage: None,
        description: "Frenzy grows from the initial Siphone circulation action; it is not an unrelated ability.".into(),
    }]);
    set_form(
        &mut current_forms,
        "vampire",
        vec![
            feature(
                "fangs",
                "Fangs",
                CurrentFeatureCategory::Anatomy,
                &[],
                "Precise Opal fangs.",
            ),
            feature(
                "nails",
                "Nails",
                CurrentFeatureCategory::WorkingSurface,
                &[],
                "Precise Opal nails.",
            ),
            feature(
                "eyes",
                "Eyes",
                CurrentFeatureCategory::Anatomy,
                &[],
                "Precise Opal eyes.",
            ),
            feature(
                "vein-accents",
                "Vein-like Accents",
                CurrentFeatureCategory::Structure,
                &[],
                "Fine Opal accents.",
            ),
        ],
        vec![CurrentTechnique {
            id: "transfusion".into(),
            name: "Transfusion".into(),
            stages: strings(&["Drain", "Store", "Transfer", "Inject"]),
            current_stage: None,
            description: "Manipulates Current between bodies.".into(),
        }],
    );
    set_form(
        &mut current_forms,
        "mariner",
        vec![
            feature(
                "aquatic-propulsion",
                "Aquatic Propulsion",
                CurrentFeatureCategory::Locomotion,
                &[],
                "Swimming anatomy.",
            ),
            feature(
                "grip",
                "Grip",
                CurrentFeatureCategory::WorkingSurface,
                &[],
                "Grip and leverage for anchoring, dragging, and grappling.",
            ),
            feature(
                "displacement-resistance",
                "Displacement Resistance",
                CurrentFeatureCategory::BodyState,
                &[],
                "Physical resistance to forced movement.",
            ),
        ],
        vec![],
    );
    set_form(
        &mut current_forms,
        "gnome",
        vec![feature(
            "working-hands",
            "Working Hands",
            CurrentFeatureCategory::WorkingSurface,
            &[],
            "Grip-specialized hands.",
        )],
        vec![],
    );
    set_form(
        &mut current_forms,
        "faun",
        vec![feature(
            "traction-feet",
            "Traction Feet",
            CurrentFeatureCategory::Locomotion,
            &[],
            "Footing-specialized anatomy.",
        )],
        vec![],
    );
    set_form(
        &mut current_forms,
        "minotaur",
        vec![feature(
            "leverage-frame",
            "Leverage Frame",
            CurrentFeatureCategory::Structure,
            &[],
            "Whole-body leverage.",
        )],
        vec![],
    );
    set_form(
        &mut current_forms,
        "hecaton",
        vec![feature(
            "working-anatomy",
            "Working Anatomy",
            CurrentFeatureCategory::Anatomy,
            &[],
            "Multiple arms, hands, and work points.",
        )],
        vec![],
    );
    set_form(
        &mut current_forms,
        "elf",
        vec![feature(
            "balanced-frame",
            "Balanced Frame",
            CurrentFeatureCategory::BodyState,
            &[],
            "Balance-specialized body.",
        )],
        vec![],
    );
    set_form(
        &mut current_forms,
        "satyr",
        vec![feature(
            "agility-legs",
            "Agility Legs",
            CurrentFeatureCategory::Locomotion,
            &[],
            "Legs specialized for agility.",
        )],
        vec![],
    );
    set_form(
        &mut current_forms,
        "centaur",
        vec![feature(
            "momentum-stride",
            "Momentum Stride",
            CurrentFeatureCategory::Locomotion,
            &[],
            "Stride-specialized body.",
        )],
        vec![],
    );
    set_form(
        &mut current_forms,
        "pegasus",
        vec![feature(
            "wings",
            "Wings",
            CurrentFeatureCategory::Locomotion,
            &["Buds", "Winglets", "Wings"],
            "Flight and vertical mobility.",
        )],
        vec![],
    );
    set_form(
        &mut current_forms,
        "spectre",
        vec![feature(
            "diamond-anchors",
            "Diamond Structural Anchors",
            CurrentFeatureCategory::Structure,
            &[],
            "Diamond anchors within a clear Hollow-Current body.",
        )],
        vec![],
    );

    let aura_moves = design_aura_moves();
    let aura_sets = natures
        .iter()
        .map(|nature| AuraSet {
            nature_id: nature.id.clone(),
            learned_moves: Vec::new(),
            equipped_moves: Vec::new(),
            design_examples: match nature.id.as_str() {
                "werewolf" => strings(&["scent", "track", "pursuit", "fixation", "pack-sense"]),
                "vampire" => strings(&["sense", "veil", "mesmerize", "pulse", "silence"]),
                "mariner" => strings(&["bearing", "depth", "mark", "hold", "current-sense"]),
                "mermaid" => strings(&["lure", "call", "charm", "focus", "beguile"]),
                "manticorp" => strings(&["command", "lock", "order", "focus", "synchronize"]),
                _ => Vec::new(),
            },
        })
        .collect();

    let synthesis_recipes = vec![SynthesisRecipe {
        id: "recipe.flynt.winged-gargoyle.v1".into(),
        name: "Winged Gargoyle".into(),
        description: "The existing multi-House treasure-hunt pattern for safe flight and grounded return after a Gargoyle has already cultivated mature biological Wings.".into(),
        source_type: SynthesisRecipeSourceType::Pattern,
        source_description: "Six ordered cultural and physical fragments followed by a stable trial and separate clinical clearance.".into(),
        is_discovered: false,
        rarity: None,
        unique_source: true,
        compatible_natures: vec!["gargoyle".into()],
        required_current_features: Vec::new(),
        required_current_techniques: Vec::new(),
        required_growth_stage: Some(GrowthStageRequirement {
            nature_id: "gargoyle".into(),
            milestone_id: "wings".into(),
        }),
        required_aura_moves: Vec::new(),
        optional_reagents: Vec::new(),
        consumed_reagents: Vec::new(),
        result_type: SynthesisResultType::Characteristic,
        result_synthesis: "winged-gargoyle-safe-flight".into(),
        mechanical_effect: "Teaches the bounded safe-flight and grounded-return characteristic after mature biological Wings plus the existing evidence, trial, and clearance gates; it never grows Wings.".into(),
        lore_text: "Indexed from the repository's existing RecipeDiscovery::winged_gargoyle authority; the Deck does not duplicate its execution state machine.".into(),
        tags: strings(&["flynt", "gargoyle", "current", "treasure-hunt", "clearance"]),
    }];

    let syntheses = vec![Synthesis {
            id: "winged-gargoyle-safe-flight".into(),
            name: "Winged Gargoyle Safe Flight".into(),
            result_type: SynthesisResultType::Characteristic,
            source_recipe_id: Some("recipe.flynt.winged-gargoyle.v1".into()),
            compatible_natures: vec!["gargoyle".into()],
            current_requirements: Vec::new(),
            required_growth_stage: Some(GrowthStageRequirement {
                nature_id: "gargoyle".into(),
                milestone_id: "wings".into(),
            }),
            aura_requirements: Vec::new(),
            reagent_requirements: Vec::new(),
            effects: vec!["Safe bounded flight and grounded return after existing clearance gates.".into()],
            description: "Deck projection of the existing winged Gargoyle pilot result. It grants safe-flight behavior, never ordinary biological Wings.".into(),
            lore_notes: Some("Execution and discovery remain owned by recipe_discovery.rs.".into()),
        }];

    let banes = vec![Bane {
        id: "garlic".into(),
        name: "Garlic".into(),
        target_natures: vec!["gargoyle".into()],
        target_primary_traits: vec!["fortification".into()],
        effect_type: "Primary Trait disruption".into(),
        mechanical_effect: "Disrupts Fortification, White Opal Statue, restoration while Fortified, and the ability to safely maintain the full Opal state.".into(),
        duration: None,
        delivery_method: None,
        strength: None,
        lore_notes: Some("Garlic is a targeted counter, not a generic Gargoyle damage bonus.".into()),
    }];

    HollowDeckCatalog {
        schema_version: HOLLOW_DECK_SCHEMA_VERSION.into(),
        hueman_stats: HUEMAN_STATS,
        institutional_axes,
        houses,
        glaushouse_structure,
        sandmanor_institution,
        flynt_institution,
        stonebend_institution,
        dominions: vec![
            Dominion::Presynce,
            Dominion::Prefog,
            Dominion::Prefig,
            Dominion::Precog,
            Dominion::Resynce,
        ],
        natures,
        primary_traits,
        evolution_paths,
        pair_relationships,
        cross_house_counterparts,
        flynt_visual_identities,
        flynt_visual_languages,
        current_forms,
        growth_paths,
        aura_moves,
        aura_sets,
        recipes: Vec::new(),
        synthesis_recipes,
        syntheses,
        banes,
    }
}

fn institutional_axis_model() -> InstitutionalAxisModel {
    InstitutionalAxisModel {
        biology: "What is the body?".into(),
        biological_growth: "What has that body biologically developed into?".into(),
        citizenship: "What is the person's social baseline?".into(),
        institutional_service: "What institution has the person entered or served?".into(),
        institutional_mastery: "What level of institutional authority or mastery has the person earned?".into(),
        specialist_discipline: "Which specialist sect, branch, or order does the person practice?".into(),
        synthesis_apex: "What convergent, synthetic, or apex attainment has the person completed?".into(),
        operating_organization: "Where and how is the person operating?".into(),
        social_alias: "What does that social world call the person?".into(),
        visual_presentation: "How do clothing, equipment, insignia, animation, environment, and VFX express those answers?".into(),
        renderer_invariant: "Presentation axes may alter clothing, equipment, silhouette, accessories, insignia, animation, environment, and VFX; they never automatically mutate biological Nature.".into(),
    }
}

fn sandmanor_institutional_hierarchy() -> SandmanorInstitutionalHierarchy {
    SandmanorInstitutionalHierarchy {
        direction: "East".into(),
        current_type: CurrentType::Regular,
        aura_material: AuraMaterial::Crystal,
        physical_philosophy: "The Way".into(),
        visual_character: strings(&[
            "bright",
            "open",
            "aquamarine",
            "sky-blue",
            "blue-white",
            "Crystal refraction",
            "coast",
            "distance",
            "movement",
            "possibility",
        ]),
        cultures: vec![
            SandmanorCulturePath {
                culture: SandmanorCulture::Minorian,
                orientation: strings(&[
                    "interior",
                    "household",
                    "material stewardship",
                    "continuity",
                    "Prefog",
                ]),
                faculty: SandmanorFacultyModel {
                    faculty: Dominion::Prefog,
                    core_question: "What is going to be needed?".into(),
                    function: "Anticipates pressure, need, shortage, support requirements, and continuity demands.".into(),
                    examples: strings(&[
                        "resource depletion",
                        "structural strain",
                        "household needs",
                        "land needs",
                        "environmental pressure",
                        "future support requirements",
                    ]),
                    excluded_interpretation: "Prefog is not generic prophecy.".into(),
                },
                social_path: vec![
                    SandmanorSocialStage {
                        identity: SandmanorSocialIdentity::Hearthkeeper,
                        tier: SandmanorMasteryTier::Baseline,
                        meaning: "Maintain immediate continuity through hearth, routine, basic needs, care, and household rhythm.".into(),
                        visual_direction: strings(&[
                            "practical domestic continuity",
                            "hearth and work tools",
                            "modest durable clothing",
                        ]),
                        avoid: Vec::new(),
                    },
                    SandmanorSocialStage {
                        identity: SandmanorSocialIdentity::Householder,
                        tier: SandmanorMasteryTier::Intermediate,
                        meaning: "Sustain and organize a full household through property, stores, dependents, tools, obligations, and contingency.".into(),
                        visual_direction: strings(&[
                            "property and household authority",
                            "organized equipment",
                            "storage and management motifs",
                        ]),
                        avoid: Vec::new(),
                    },
                    SandmanorSocialStage {
                        identity: SandmanorSocialIdentity::Husbandman,
                        tier: SandmanorMasteryTier::Apex,
                        meaning: "Practice mature stewardship of husbandry, land, animals, resources, family, production, planning, and long-term continuity.".into(),
                        visual_direction: strings(&[
                            "mature stewardship",
                            "land, resource, animal, and household integration",
                            "practical authority",
                        ]),
                        avoid: strings(&["caricature farmer", "beginner farmer shorthand"]),
                    },
                ],
                social_progression: strings(&["maintain", "organize", "steward"]),
                aura_path: vec![
                    SandmanorAuraStage {
                        identity: SandmanorAuraMastery::Geomancer,
                        tier: SandmanorMasteryTier::Baseline,
                        meaning: "Reads local physical and resource conditions.".into(),
                        question: "What does this place permit?".into(),
                        examples: strings(&[
                            "soil",
                            "stone",
                            "water",
                            "grade",
                            "resource",
                            "boundary",
                            "orientation",
                            "immediate environmental constraints",
                        ]),
                        visual_direction: strings(&[
                            "local terrain and resource reading",
                            "Crystal-ground relationship",
                        ]),
                        avoid: Vec::new(),
                    },
                    SandmanorAuraStage {
                        identity: SandmanorAuraMastery::Labyrinthian,
                        tier: SandmanorMasteryTier::Intermediate,
                        meaning: "Understands systems of conditions and dependencies.".into(),
                        question: "If this changes, where does the effect travel?".into(),
                        examples: strings(&[
                            "routes",
                            "storage",
                            "thresholds",
                            "bottlenecks",
                            "resource chains",
                            "family relationships",
                            "contingencies",
                            "nested systems",
                        ]),
                        visual_direction: strings(&[
                            "routes and dependencies",
                            "system overlays",
                            "maze and system motifs",
                        ]),
                        avoid: strings(&["literal maze costume"]),
                    },
                    SandmanorAuraStage {
                        identity: SandmanorAuraMastery::Demiurge,
                        tier: SandmanorMasteryTier::Apex,
                        meaning: "Arranges conditions into a functioning, sustainable world or system; this is mature systems construction, not omnipotent creation of a universe.".into(),
                        question: "What conditions will let this system sustain itself?".into(),
                        examples: strings(&[
                            "viable conditions",
                            "sustainable systems",
                            "world-building",
                        ]),
                        visual_direction: strings(&[
                            "resolved systems mastery",
                            "mature Crystal world-building identity",
                        ]),
                        avoid: strings(&["omnipotent universe creator"]),
                    },
                ],
                aura_progression: strings(&["reads place", "reads system", "makes system"]),
                nature_path_id: "sandmanor-minorian".into(),
            },
            SandmanorCulturePath {
                culture: SandmanorCulture::Minoan,
                orientation: strings(&[
                    "coastal",
                    "mobile",
                    "social",
                    "possibility",
                    "Prefig",
                ]),
                faculty: SandmanorFacultyModel {
                    faculty: Dominion::Prefig,
                    core_question: "What is this becoming?".into(),
                    function: "Anticipates form, trajectory, possibility, social movement, and emerging opportunity.".into(),
                    examples: strings(&[
                        "routes opening",
                        "social groups forming",
                        "events becoming important",
                        "opportunities emerging",
                        "trends gaining momentum",
                        "trajectories becoming viable",
                    ]),
                    excluded_interpretation: "Prefig is not generic fortune-telling.".into(),
                },
                social_path: vec![
                    SandmanorSocialStage {
                        identity: SandmanorSocialIdentity::BeachBum,
                        tier: SandmanorMasteryTier::Baseline,
                        meaning: "Lives lightly with baseline freedom of movement, low attachment, and adaptability across open coastal environments.".into(),
                        visual_direction: strings(&[
                            "low-attachment coastal freedom",
                            "practical relaxed clothing",
                            "movement-ready",
                        ]),
                        avoid: Vec::new(),
                    },
                    SandmanorSocialStage {
                        identity: SandmanorSocialIdentity::PartyBoy,
                        tier: SandmanorMasteryTier::Intermediate,
                        meaning: "Activates social environments through timing, attention, group energy, invitation, and connection, turning unrelated people into an event.".into(),
                        visual_direction: strings(&[
                            "social activation",
                            "rhythm",
                            "expressive movement",
                            "attention and connection",
                        ]),
                        avoid: Vec::new(),
                    },
                    SandmanorSocialStage {
                        identity: SandmanorSocialIdentity::Socialite,
                        tier: SandmanorMasteryTier::Apex,
                        meaning: "Practices mature social mobility across networks, groups, places, and relationships, carrying reputation and access between environments.".into(),
                        visual_direction: strings(&[
                            "access",
                            "polish",
                            "network mobility",
                            "recognizable status",
                        ]),
                        avoid: strings(&["fashion caricature"]),
                    },
                ],
                social_progression: strings(&[
                    "move freely",
                    "create connection",
                    "master networks",
                ]),
                aura_path: vec![
                    SandmanorAuraStage {
                        identity: SandmanorAuraMastery::Acolyte,
                        tier: SandmanorMasteryTier::Baseline,
                        meaning: "An initiated Prefig practitioner learning discipline, orientation, balance, and recognition of emerging possibility.".into(),
                        question: "What possibility is opening?".into(),
                        examples: strings(&[
                            "discipline",
                            "orientation",
                            "balance",
                            "emerging possibility",
                        ]),
                        visual_direction: strings(&[
                            "disciplined Prefig initiation",
                            "poised",
                            "observant",
                            "Crystal possibility motifs",
                        ]),
                        avoid: strings(&["generic religious follower"]),
                    },
                    SandmanorAuraStage {
                        identity: SandmanorAuraMastery::Navigator,
                        tier: SandmanorMasteryTier::Intermediate,
                        meaning: "Selects and maintains a trajectory through changing possibility.".into(),
                        question: "Which route do we take, and how do we stay on it?".into(),
                        examples: strings(&[
                            "social routes",
                            "geographic routes",
                            "opportunity",
                            "negotiation",
                            "coast and water",
                            "battle",
                            "relationship networks",
                            "changing conditions",
                        ]),
                        visual_direction: strings(&[
                            "route and trajectory language",
                            "maps",
                            "direction",
                            "movement",
                            "Crystal lines and refraction",
                        ]),
                        avoid: Vec::new(),
                    },
                    SandmanorAuraStage {
                        identity: SandmanorAuraMastery::Ascendant,
                        tier: SandmanorMasteryTier::Apex,
                        meaning: "Exceeds the limits of existing trajectories and possibility structures.".into(),
                        question: "What if the existing route is no longer the limit?".into(),
                        examples: strings(&[
                            "surpassing existing possibility",
                            "boundary-breaking trajectory",
                        ]),
                        visual_direction: strings(&[
                            "lightness",
                            "openness",
                            "boundary-breaking",
                            "aerial and vertical visual logic",
                            "resolved rather than cluttered",
                        ]),
                        avoid: Vec::new(),
                    },
                ],
                aura_progression: strings(&[
                    "recognizes possibility",
                    "moves through possibility",
                    "surpasses existing possibility",
                ]),
                nature_path_id: "sandmanor-minoan".into(),
            },
        ],
        social_pairings: vec![
            SandmanorSocialPairing {
                minorian: SandmanorSocialIdentity::Hearthkeeper,
                minoan: SandmanorSocialIdentity::BeachBum,
                contrast: "stay and maintain ↔ move and adapt".into(),
            },
            SandmanorSocialPairing {
                minorian: SandmanorSocialIdentity::Householder,
                minoan: SandmanorSocialIdentity::PartyBoy,
                contrast: "organize a household ↔ organize a gathering".into(),
            },
            SandmanorSocialPairing {
                minorian: SandmanorSocialIdentity::Husbandman,
                minoan: SandmanorSocialIdentity::Socialite,
                contrast: "steward material and resource relationships ↔ steward social and network relationships".into(),
            },
        ],
        aura_pairings: vec![
            SandmanorAuraPairing {
                minorian: SandmanorAuraMastery::Geomancer,
                minoan: SandmanorAuraMastery::Acolyte,
                contrast: "reads the world and place ↔ learns orientation within possibility".into(),
            },
            SandmanorAuraPairing {
                minorian: SandmanorAuraMastery::Labyrinthian,
                minoan: SandmanorAuraMastery::Navigator,
                contrast: "understands the system ↔ moves through the system".into(),
            },
            SandmanorAuraPairing {
                minorian: SandmanorAuraMastery::Demiurge,
                minoan: SandmanorAuraMastery::Ascendant,
                contrast: "constructs viable conditions and world ↔ exceeds the limits of existing conditions and world".into(),
            },
        ],
        sovereign_synthesis: SandmanorSovereignSynthesis {
            peer_apexes: vec![
                SandmanorAuraMastery::Demiurge,
                SandmanorAuraMastery::Ascendant,
            ],
            synthesis_identity: SandmanorSovereignIdentity::Transcendent,
            synthesis_formula: "Demiurge + Ascendant → The Transcendent".into(),
            sovereign_identity: SandmanorSovereignIdentity::Sandman,
            sovereign_embodiment_of: SandmanorSovereignIdentity::Transcendent,
            sovereign_formula: "The Transcendent → The Sandman".into(),
            meaning: "Masters The Way between constructing and sustaining a viable world and exceeding the limits of an existing world.".into(),
            core_principle: strings(&[
                "The Minorian says: build the house.",
                "The Minoan says: leave the house.",
                "The Transcendent knows when to do either.",
            ]),
            social_boundary: "The Transcendent is not an ordinary social rank.".into(),
            nature_boundary: "The Transcendent and The Sandman are not Nature rungs and are not inserted into either Nature path.".into(),
            visual_direction: strings(&[
                "Demiurge world-making and sustained conditions",
                "Ascendant world-exceeding and open possibility",
                "sovereign mastery of The Way between stability and transcendence",
                "folkloric Sandman archetype elevated into Sandmanor sovereignty",
            ]),
            avoid: strings(&[
                "more powerful Pegasus",
                "bigger Hecaton",
                "forced Morpheus or Hypnos deity identity",
            ]),
        },
        axis_invariant: "Culture, social and Current identity, Aura mastery, Nature, and sovereign synthesis are separate typed dimensions.".into(),
    }
}

fn flynt_institutional_hierarchy() -> FlyntInstitutionalHierarchy {
    FlyntInstitutionalHierarchy {
        citizen_baseline: FlyntCitizenship::Warden,
        service_status: FlyntServiceStatus::Corpman,
        service_prerequisite: FlyntCitizenship::Warden,
        capacity_requirement: "Prove Capacity".into(),
        elite_status: FlyntEliteStatus::MysteryMan,
        elite_prerequisite: FlyntServiceStatus::Corpman,
        sect_prerequisite: FlyntEliteStatus::MysteryMan,
        sects: vec![
            FlyntSectTradition {
                sect: FlyntSect::Hunter,
                nature_id: "gargoyle".into(),
                primary_trait_id: "fortification".into(),
                vocation: strings(&[
                    "supernatural hunter",
                    "prepared specialist",
                    "surveillance",
                    "containment",
                    "survival",
                    "patient pursuit",
                    "Van Helsing-like field discipline",
                ]),
                visual_direction: strings(&[
                    "prepared",
                    "observant",
                    "containment",
                    "protection",
                    "field tools",
                    "Gargoyle / Opal / Fortification language",
                ]),
            },
            FlyntSectTradition {
                sect: FlyntSect::Chief,
                nature_id: "werewolf".into(),
                primary_trait_id: "siphone".into(),
                vocation: strings(&[
                    "pack leadership",
                    "territory",
                    "command",
                    "pursuit",
                    "group organization",
                    "living force",
                ]),
                visual_direction: strings(&[
                    "command",
                    "pack",
                    "territory",
                    "physical authority",
                    "movement",
                    "Werewolf / Siphone language",
                ]),
            },
            FlyntSectTradition {
                sect: FlyntSect::Mariner,
                nature_id: "mariner".into(),
                primary_trait_id: "anchoring".into(),
                vocation: strings(&[
                    "navigation",
                    "positional control",
                    "water",
                    "rigging",
                    "routes",
                    "maritime fieldcraft",
                ]),
                visual_direction: strings(&[
                    "maritime fieldcraft",
                    "navigation",
                    "rigging",
                    "coats",
                    "rope",
                    "positional equipment",
                    "Mariner / Anchoring language",
                ]),
            },
        ],
        convergence: FlyntConvergenceMastery {
            required_sects: FlyntSect::ALL.to_vec(),
            convergence_identity: "Chimera".into(),
            convergence_nature_id: "chimera".into(),
            convergence_meaning:
                "Convergent synthesis and mastery of Hunter, Chief, and Mariner; not a fourth ordinary Mystery Men sect."
                    .into(),
            convergence_visual_direction:
                "A disciplined composite of Hunter structure, Chief force, and Mariner positional control; never random monster clutter."
                    .into(),
            perfected_from_identity: "Chimera".into(),
            perfected_identity: "Manticorp".into(),
            perfected_nature_id: "manticorp".into(),
            perfection_meaning:
                "The perfected and stabilized apex of Chimera: integrated, stable, sovereign, resolved, and singular."
                    .into(),
            perfection_visual_direction:
                "Perfected integration rather than visible accumulation: controlled, sovereign, resolved, and singular."
                    .into(),
            apex_association: "The Tross".into(),
            legacy_pair_compatibility:
                "The existing reciprocal Form pair between the two apex Natures and its Convergence/Divergence combat mechanics remain archive-compatible peer mechanics; they do not replace this directional institutional mastery."
                    .into(),
        },
        sorcery: FlyntSorcerySystem {
            domain: "Flynt synthesis manufacturing and formulation".into(),
            paired_ladder: vec![
                FlyntSorceryPair {
                    male_identity: FlyntSorceryIdentity::Wizard,
                    female_identity: FlyntSorceryIdentity::Witch,
                    function: FlyntSorceryFunction::EstablishedManufacture,
                    capabilities: strings(&[
                        "manufacture established Synthesis Recipes",
                        "manufacture established Synthesis products",
                    ]),
                },
                FlyntSorceryPair {
                    male_identity: FlyntSorceryIdentity::Warlock,
                    female_identity: FlyntSorceryIdentity::Enchantress,
                    function: FlyntSorceryFunction::PatternModification,
                    capabilities: strings(&[
                        "combine",
                        "counterfeit",
                        "corrupt",
                        "strengthen",
                        "weaken",
                        "repurpose",
                        "reformulate",
                    ]),
                },
                FlyntSorceryPair {
                    male_identity: FlyntSorceryIdentity::Sorcerer,
                    female_identity: FlyntSorceryIdentity::Sorceress,
                    function: FlyntSorceryFunction::PatternAuthorship,
                    capabilities: strings(&[
                        "author new Synthesis Recipes",
                        "author new Synthesis patterns",
                    ]),
                },
            ],
            ecosystem_roles: vec![
                FlyntEcosystemRoleProfile {
                    role: FlyntEcosystemRole::Tross,
                    function: "sovereign direction and governance".into(),
                },
                FlyntEcosystemRoleProfile {
                    role: FlyntEcosystemRole::MysteryMen,
                    function: "enforcement, interdiction, investigation, and regulation".into(),
                },
                FlyntEcosystemRoleProfile {
                    role: FlyntEcosystemRole::Sorcery,
                    function:
                        "manufacture, formulation, pattern modification, and pattern authorship"
                            .into(),
                },
                FlyntEcosystemRoleProfile {
                    role: FlyntEcosystemRole::Gallows,
                    function: "urban vice, leverage, distribution, and commerce".into(),
                },
                FlyntEcosystemRoleProfile {
                    role: FlyntEcosystemRole::WeFairyMen,
                    function: "rural public legitimacy, mutual aid, and protection".into(),
                },
                FlyntEcosystemRoleProfile {
                    role: FlyntEcosystemRole::Marquise,
                    function: "maritime operating and routing extension".into(),
                },
            ],
            governance: "The Tross governs the wider Flynt vice and synthesis ecosystem."
                .into(),
            systemic_danger: "Uncontrolled synthesis is the key systemic danger.".into(),
            sanctioned_practitioner_boundary:
                "Sanctioned sorcery practitioners are not inherently criminal, hostile, villainous, or anti-Tross."
                    .into(),
            axis_boundary:
                "Sorcery is neither a Sandmanor Aura ladder, a Nature progression, nor a Mystery Men sect or rank."
                    .into(),
        },
        operating_contexts: vec![
            FlyntOperatingContext {
                organization: FlyntOperatingOrganization::WeFairyMen,
                social_basis: strings(&[
                    "mutual aid",
                    "protection",
                    "reputation",
                    "favors given",
                ]),
                environment: strings(&["countryside", "roads", "fields", "forest", "river"]),
                tradition_affinity: None,
                relationship_to_hierarchy:
                    "Rural public-facing folk operation, never a formal service-ladder step."
                        .into(),
            },
            FlyntOperatingContext {
                organization: FlyntOperatingOrganization::Gallows,
                social_basis: strings(&[
                    "favors owed",
                    "debts",
                    "leverage",
                    "access",
                ]),
                environment: strings(&[
                    "docks",
                    "clubs",
                    "warehouses",
                    "streets",
                    "back rooms",
                ]),
                tradition_affinity: None,
                relationship_to_hierarchy:
                    "Urban syndicate operation, never a formal service-ladder step.".into(),
            },
            FlyntOperatingContext {
                organization: FlyntOperatingOrganization::Marquise,
                social_basis: strings(&[
                    "maritime extension",
                    "routes",
                    "navigation",
                    "institutional operation",
                ]),
                environment: strings(&["waterways", "ships", "ports", "maritime routes"]),
                tradition_affinity: Some(FlyntSect::Mariner),
                relationship_to_hierarchy:
                    "Maritime institutional and operating extension strongly connected to the Mariner tradition, never a career-ladder step."
                        .into(),
            },
        ],
        citizen_visual_direction: strings(&[
            "ordinary Flynt citizen",
            "social baseline",
            "no implied elite or specialist authority",
        ]),
        service_visual_direction: strings(&[
            "practical",
            "general-purpose",
            "field-ready",
            "disciplined",
            "shared Flynt service identity",
            "minimal specialist symbolism",
        ]),
        elite_visual_direction: strings(&[
            "elite threshold",
            "personalized field kit",
            "Mystery Men recognition",
            "permission to display specialist identity",
            "increased material and supernatural confidence",
        ]),
    }
}

fn stonebend_institutional_hierarchy() -> StonebendInstitutionalHierarchy {
    StonebendInstitutionalHierarchy {
        citizen_baseline: StonebendCitizenship::Gerald,
        training_entry: StonebendTrainingStatus::Apprentice,
        training_prerequisite: StonebendCitizenship::Gerald,
        competency_prerequisite: StonebendTrainingStatus::Apprentice,
        competencies: vec![
            StonebendCompetencyProfile {
                competency: StonebendCompetency::Mason,
                channel: InstitutionalChannel::Current,
                aura_material: None,
                concepts: strings(&[
                    "physical material work",
                    "placement",
                    "cutting",
                    "fitting",
                    "joining",
                    "bracing",
                    "tool use",
                    "direct manipulation of physical structure",
                ]),
                visual_direction: strings(&[
                    "tools",
                    "measurement",
                    "fitting",
                    "physical material contact",
                    "structured workwear",
                ]),
                avoid: strings(&["generic fantasy class costume"]),
            },
            StonebendCompetencyProfile {
                competency: StonebendCompetency::Mage,
                channel: InstitutionalChannel::Aura,
                aura_material: Some(AuraMaterial::Diamond),
                concepts: strings(&[
                    "geometry",
                    "alignment",
                    "planes",
                    "angle",
                    "measurement",
                    "structure",
                    "load path",
                    "fault",
                    "intended form",
                ]),
                visual_direction: strings(&[
                    "Diamond geometry",
                    "alignment",
                    "measurement",
                    "planes",
                    "structural reading",
                    "pattern and perception",
                ]),
                avoid: strings(&["generic fantasy wizard robes"]),
            },
            StonebendCompetencyProfile {
                competency: StonebendCompetency::Pugilist,
                channel: InstitutionalChannel::Current,
                aura_material: None,
                concepts: strings(&[
                    "generate force",
                    "absorb force",
                    "push",
                    "pull",
                    "carry",
                    "strike",
                    "brace",
                    "grapple",
                    "compress",
                    "endure load",
                ]),
                visual_direction: strings(&[
                    "body under load",
                    "bracing",
                    "leverage",
                    "impact",
                    "practical protection",
                    "muscular force",
                ]),
                avoid: strings(&["generic fighter-class shorthand"]),
            },
            StonebendCompetencyProfile {
                competency: StonebendCompetency::Foreman,
                channel: InstitutionalChannel::Aura,
                aura_material: Some(AuraMaterial::Diamond),
                concepts: strings(&[
                    "organization of force",
                    "load distribution",
                    "sequence",
                    "coordination",
                    "support",
                    "dependency",
                    "directing bodies, tools, and work through a physical system",
                ]),
                visual_direction: strings(&[
                    "coordination",
                    "worksite authority",
                    "measurement",
                    "load distribution",
                    "planning",
                    "Diamond structural overlays and indicators",
                ]),
                avoid: strings(&["generic mind control", "psychic commander archetype"]),
            },
        ],
        orders: vec![
            StonebendOrderPath {
                mastery: StonebendMastery::Freemason,
                cultural_identity: "Superstar".into(),
                current_competency: StonebendCompetency::Mason,
                aura_competency: StonebendCompetency::Mage,
                synthesis_meaning:
                    "Mason Current plus Mage Diamond Aura resolved as one completed structural mastery."
                        .into(),
                visual_direction:
                    "Resolve physical craftsmanship and Diamond structural perception as one complete identity, never two layered costumes."
                        .into(),
            },
            StonebendOrderPath {
                mastery: StonebendMastery::Proliteriate,
                cultural_identity: "Rockstar".into(),
                current_competency: StonebendCompetency::Pugilist,
                aura_competency: StonebendCompetency::Foreman,
                synthesis_meaning:
                    "Pugilist Current plus Foreman Diamond Aura resolved as force made productive."
                        .into(),
                visual_direction:
                    "Resolve bodily capacity and organization of load as one productive mastery, never two layered costumes."
                        .into(),
            },
        ],
        apex: StonebendApexMastery {
            peer_inputs: StonebendMastery::ALL.to_vec(),
            required_cultural_identities: strings(&["Superstar", "Rockstar"]),
            identity: "Hypergiant".into(),
            meaning:
                "Perfected Stonebend integration of structural mastery with capacity and load mastery."
                    .into(),
            visual_direction: strings(&[
                "monumental",
                "controlled",
                "load-bearing",
                "precise",
                "complete",
            ]),
            avoid: strings(&["merely a bigger Ogre"]),
        },
        philosophy: StonebendPhysicalPhilosophy {
            faculty: Dominion::Presynce,
            current_type: CurrentType::Hollow,
            aura_material: AuraMaterial::Diamond,
            philosophy: "The Cut".into(),
            process: "Hollowing".into(),
            presynce_meaning:
                "Apprehending physical presence as matter, structure, support, load, fault, and relationship without inflating perception into omniscience."
                    .into(),
            presynce_questions: strings(&[
                "What is physically here?",
                "What is it made from?",
                "Where is its weight?",
                "What supports it?",
                "Where is its fault?",
                "What bears load?",
                "What moves if this moves?",
                "What remains if this is cut?",
            ]),
            cut_meaning:
                "Selecting the exact physical boundary where one material state, relationship, or structure becomes another."
                    .into(),
            cut_examples: strings(&[
                "Mason: raw material → Cut → fitted material",
                "Pugilist: static load → Cut → displaced load",
                "Orc: whole structure → Cut → sundered structure",
                "Troglodyte: solid mass → Cut → controlled hollow",
            ]),
            hollowing_boundary:
                "Hollowing remains the terminal Stonebend Nature and process expression; this institutional projection does not redefine it."
                    .into(),
        },
        citizen_visual_direction: strings(&["normal Stonebend citizen presentation"]),
        apprentice_visual_direction: strings(&[
            "practical work clothing",
            "tools",
            "training equipment",
            "minimal order distinction",
            "Diamond and Hollow Current hints",
        ]),
        constitutional_compatibility:
            "This Deck training/mastery projection does not replace Stonebend's separate constitutional Claim/Yield/Title authorities: Freemason, Proliteriate, and Hypergiant retain their existing civic and office meanings outside the Deck."
                .into(),
    }
}

fn flynt_visual_identity_profiles() -> Vec<FlyntVisualIdentityProfile> {
    vec![
        FlyntVisualIdentityProfile {
            nature_id: "gargoyle".into(),
            mystery_men_identity: social_identity(
                "hunter",
                "Hunter",
                FlyntOrganization::MysteryMen,
                "A hidden, disciplined supernatural hunter whose tools and field preparation reveal professional specialization rather than wizardry.",
            ),
            we_fairy_men_identity: social_identity(
                "bro-white",
                "Bro White",
                FlyntOrganization::WeFairyMen,
                "A rural folk protector and supernatural hunter shaped by roads, local trust, mutual aid, and an earned legendary silhouette.",
            ),
            gallows_identity: social_identity(
                "johnny-apple",
                "Johnny Apple",
                FlyntOrganization::Gallows,
                "The same Gargoyle translated into a sharper, denser, street-oriented Gallows presence while remaining recognizably Bro White.",
            ),
            symbol_anchor: "Apple".into(),
            mythic_source: "Snow White's poisoned apple".into(),
            secondary_echoes: strings(&["Johnny Appleseed"]),
            motif_direction: "Keep the apple discoverable through restrained personal or environmental detail; never dress the character as Snow White.".into(),
        },
        FlyntVisualIdentityProfile {
            nature_id: "werewolf".into(),
            mystery_men_identity: social_identity(
                "chief",
                "Chief",
                FlyntOrganization::MysteryMen,
                "A prepared, disciplined pack professional whose leadership is expressed through practical field command and specialized gear.",
            ),
            we_fairy_men_identity: social_identity(
                "beauty-the-beast",
                "Beauty the Beast",
                FlyntOrganization::WeFairyMen,
                "A rural pack leader and folk figure whose protection, reputation, reciprocity, and local relationships carry his legend.",
            ),
            gallows_identity: social_identity(
                "tommy-rose",
                "Tommy Rose",
                FlyntOrganization::Gallows,
                "The same Werewolf in the Gallows: composed, dangerous, socially connected, and marked by subtle rose-associated detail.",
            ),
            symbol_anchor: "Rose".into(),
            mythic_source: "Beauty and the Beast".into(),
            secondary_echoes: Vec::new(),
            motif_direction: "Use restrained rose details in jewelry, embroidery, tools, tattoos, or rooms; never dress the character as Belle or the Beast.".into(),
        },
        FlyntVisualIdentityProfile {
            nature_id: "mariner".into(),
            mystery_men_identity: social_identity(
                "mariner",
                "Mariner",
                FlyntOrganization::MysteryMen,
                "An occult maritime professional defined by navigation discipline, prepared tools, field competence, and practical supernatural work.",
            ),
            we_fairy_men_identity: social_identity(
                "cindarellaman",
                "Cindarellaman",
                FlyntOrganization::WeFairyMen,
                "A rural and seafaring folk hero shaped by roads, rivers, weather, mutual aid, and the Midnight Crew echo.",
            ),
            gallows_identity: social_identity(
                "jimmy-pumps",
                "Jimmy Pumps",
                FlyntOrganization::Gallows,
                "The same Mariner rendered as urban, maritime, and underworld-connected, carrying subtle shoe, midnight, and carriage symbolism.",
            ),
            symbol_anchor: "Shoe / pump".into(),
            mythic_source: "Cinderella's glass slipper".into(),
            secondary_echoes: strings(&["Pumpkin carriage", "Midnight", "Midnight Crew"]),
            motif_direction: "Keep shoe, pump, midnight, and carriage echoes in restrained objects or environments; never dress the character as Cinderella.".into(),
        },
    ]
}

fn flynt_institutional_visual_languages() -> Vec<FlyntInstitutionalVisualLanguage> {
    vec![
        FlyntInstitutionalVisualLanguage {
            organization: FlyntOrganization::MysteryMen,
            visual_character: strings(&[
                "hidden",
                "disciplined",
                "occult",
                "professional",
                "prepared",
                "specialized",
            ]),
            social_power: strings(&[
                "tools",
                "discipline",
                "field gear",
                "specialization",
                "House symbols",
                "practical supernatural work",
            ]),
            wardrobe: strings(&[
                "specialized field gear",
                "practical tools",
                "restrained House symbols",
            ]),
            environment: strings(&[
                "operational field sites",
                "prepared workspaces",
                "occult evidence environments",
            ]),
            animation_direction:
                "Prepared, economical professional movement with tool-aware discipline.".into(),
            avoid: strings(&["generic wizard robes"]),
        },
        FlyntInstitutionalVisualLanguage {
            organization: FlyntOrganization::WeFairyMen,
            visual_character: strings(&[
                "Robin Hood energy",
                "local legend",
                "countryside protector",
                "outlaw folk hero",
                "mutual aid",
                "reputation",
                "reciprocity",
            ]),
            social_power: strings(&[
                "favors given",
                "protection",
                "stories",
                "local trust",
                "movement through rural Flynt",
            ]),
            wardrobe: strings(&[
                "worn practical clothing",
                "rural workwear",
                "travel gear",
                "field tools",
                "personal insignia",
                "handmade or inherited objects",
            ]),
            environment: strings(&[
                "open terrain",
                "roads",
                "fields",
                "forest",
                "river",
                "farms",
                "small settlements",
                "campfires",
                "daylight and weather",
            ]),
            animation_direction:
                "Open, weather-aware movement shaped by travel, reputation, and public aid."
                    .into(),
            avoid: strings(&[
                "uniformed-army presentation",
                "new biological Nature designs",
            ]),
        },
        FlyntInstitutionalVisualLanguage {
            organization: FlyntOrganization::Gallows,
            visual_character: strings(&[
                "lived-in",
                "contemporary",
                "urban",
                "underworld-connected",
                "obligation-based",
            ]),
            social_power: strings(&[
                "favors owed",
                "debt",
                "leverage",
                "access",
                "routes",
                "docks",
                "clubs",
                "back rooms",
                "contracts",
                "city relationships",
            ]),
            wardrobe: strings(&[
                "sharper tailoring",
                "urban workwear",
                "coats",
                "situational suits",
                "dock or industrial clothing",
                "sparing jewelry",
                "symbolic personal items",
            ]),
            environment: strings(&[
                "streets",
                "alleys",
                "docks",
                "clubs",
                "warehouses",
                "apartments",
                "offices",
                "back rooms",
                "neon, street, and harbor light",
            ]),
            animation_direction:
                "Controlled urban movement attentive to enclosed space, access, leverage, and social consequence."
                    .into(),
            avoid: strings(&[
                "stereotypical gangster costumes",
                "costume-drama mafia",
                "literal fairy-tale cosplay",
                "new biological Nature designs",
            ]),
        },
    ]
}

fn social_identity(
    id: &str,
    name: &str,
    organization: FlyntOrganization,
    visual_direction: &str,
) -> FlyntSocialIdentity {
    FlyntSocialIdentity {
        id: id.into(),
        name: name.into(),
        organization,
        visual_direction: visual_direction.into(),
    }
}

fn trait_entry(id: &str, name: &str, mechanical_effect: &str) -> PrimaryTrait {
    PrimaryTrait {
        id: id.into(),
        name: name.into(),
        mechanical_effect: mechanical_effect.into(),
        lore_notes: None,
    }
}

#[allow(clippy::too_many_arguments)]
fn nature(
    id: &str,
    name: &str,
    house_id: House,
    dominion_id: Dominion,
    primary_trait_id: &str,
    mature_stats: StatBlock,
    evolution: Option<(&str, usize)>,
    paired_nature_id: Option<&str>,
    shared_pair_principle: Option<&str>,
    visual_description: Option<&str>,
    notes: Option<&str>,
) -> Nature {
    let (current_type, aura_material) = match house_id {
        House::Stonebend => (CurrentType::Hollow, AuraMaterial::Diamond),
        House::Sandmanor => (CurrentType::Regular, AuraMaterial::Crystal),
        House::Glaushouse if evolution.is_some_and(|(path, _)| path == "glaushouse-preceptor") => {
            (CurrentType::Hollow, AuraMaterial::Emerald)
        }
        House::Glaushouse => (CurrentType::Hollow, AuraMaterial::Jade),
        House::Flynt => (CurrentType::Regular, AuraMaterial::Opal),
    };
    Nature {
        id: id.into(),
        name: name.into(),
        house_id,
        dominion_id,
        current_type,
        aura_material,
        primary_trait_id: primary_trait_id.into(),
        mature_stats,
        evolution_path_id: evolution.map(|(path, _)| path.into()),
        evolution_index: evolution.map(|(_, index)| index),
        paired_nature_id: paired_nature_id.map(Into::into),
        shared_pair_principle: shared_pair_principle.map(Into::into),
        visual_description: visual_description.map(Into::into),
        notes: notes.map(Into::into),
    }
}

fn path(
    id: &str,
    name: &str,
    house_id: House,
    dominion_id: Dominion,
    nature_ids: &[&str],
) -> EvolutionPath {
    EvolutionPath {
        id: id.into(),
        name: name.into(),
        house_id,
        dominion_id,
        nature_ids: strings(nature_ids),
    }
}

fn feature(
    id: &str,
    name: &str,
    category: CurrentFeatureCategory,
    stages: &[&str],
    visual_effects: &str,
) -> CurrentFeature {
    CurrentFeature {
        id: id.into(),
        name: name.into(),
        category,
        stages: strings(stages),
        current_stage: None,
        stat_effects: None,
        visual_effects: Some(visual_effects.into()),
        requirements: Vec::new(),
    }
}

fn growth(id: &str, name: &str, visual_effect: &str) -> GrowthMilestone {
    GrowthMilestone {
        id: id.into(),
        name: name.into(),
        visual_effect: visual_effect.into(),
    }
}

fn set_form(
    forms: &mut [CurrentForm],
    nature_id: &str,
    current_features: Vec<CurrentFeature>,
    current_techniques: Vec<CurrentTechnique>,
) {
    let form = forms
        .iter_mut()
        .find(|form| form.nature_id == nature_id)
        .expect("seeded Nature must have a Current Form");
    form.current_features = current_features;
    form.current_techniques = current_techniques;
}

fn design_aura_moves() -> Vec<AuraMove> {
    let definitions = [
        (
            "werewolf",
            &["scent", "track", "pursuit", "fixation", "pack-sense"][..],
        ),
        (
            "vampire",
            &["sense", "veil", "mesmerize", "pulse", "silence"][..],
        ),
        (
            "mariner",
            &["bearing", "depth", "mark", "hold", "current-sense"][..],
        ),
        (
            "mermaid",
            &["lure", "call", "charm", "focus", "beguile"][..],
        ),
        (
            "manticorp",
            &["command", "lock", "order", "focus", "synchronize"][..],
        ),
    ];
    let mut moves = Vec::new();
    for (nature_id, names) in definitions {
        for id in names {
            if moves.iter().any(|item: &AuraMove| item.id == *id) {
                continue;
            }
            moves.push(AuraMove {
                id: (*id).into(),
                name: title_case(id),
                description: None,
                mode: None,
                compatible_natures: vec![nature_id.into()],
                compatible_dominions: vec![Dominion::Resynce],
                effects: Vec::new(),
                requirements: Vec::new(),
                canon_status: CanonStatus::DesignExample,
            });
        }
    }
    moves
}

fn title_case(value: &str) -> String {
    value
        .split('-')
        .map(|part| {
            let mut chars = part.chars();
            chars.next().map_or_else(String::new, |first| {
                first.to_uppercase().collect::<String>() + chars.as_str()
            })
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).into()).collect()
}
