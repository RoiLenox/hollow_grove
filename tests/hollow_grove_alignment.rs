use hollow_grove::hollow_grove_contract::{
    CareFunction, CareLevel, CareLevelClaim, CivilizationModel, EntityCategory, EntityClaim,
    FrameGrammarClaim, GnomeProgressionClaim, HollowGroveAlignmentInput, HollowingClaim,
    HollowingMode, House, HouseAnchor, HouseAnchorClaim, HouseRock, HouseRockClaim, Lineage,
    NightingaleClaim, NightingaleFunction, NightingaleIdentity, Profession, ProfessionAccessRule,
    ProfessionClaim, SandmanorPeople, SandmanorPeopleClaim, Substance, SubstanceIdentityClaim,
    build_hollow_grove_alignment_validation_report, build_hollow_grove_alignment_witness,
    canonical_hollowing_fixture, canonical_house_goods_fixture, canonical_material_fixture,
    canonical_medicine_fixture, canonical_ontology_fixture, canonical_root_alignment_fixture,
    perform_canonical_hollowing, validate_current_world_context_alignment,
    validate_hollow_grove_alignment,
};

#[test]
fn alignment_witness_prints_the_root_laws() {
    let witness = build_hollow_grove_alignment_witness();

    assert!(witness.contains("HOLLOW GROVE ALIGNMENT WITNESS"));
    assert!(witness.contains("Current = blood"));
    assert!(witness.contains("Hollow = pus"));
    assert!(witness.contains("Aura = air / pressure / light"));
    assert!(witness.contains("Whole\n→ Hollowing\n→ Hollow + Hollowed"));
    assert!(witness.contains("Frame = named living mech only"));
    assert!(witness.contains("Diamond claims"));
    assert!(witness.contains("Glaüshouse owns professional medicine."));
}

#[test]
fn current_world_context_document_stays_aligned() {
    let diagnostics = validate_current_world_context_alignment();
    assert!(
        diagnostics.is_empty(),
        "world context drifted: {diagnostics:?}"
    );
}

#[test]
fn canonical_hollowing_fixture_preserves_outer_form_and_extracts_hollow() {
    let fixture = canonical_hollowing_fixture();
    let result = perform_canonical_hollowing(&fixture);

    assert!(result.hollow_extracted);
    assert!(result.hollowed_remainder_preserved);
}

#[test]
fn canonical_material_fixture_validates() {
    let diagnostics = validate_hollow_grove_alignment(&canonical_material_fixture());
    assert!(diagnostics.is_empty(), "{diagnostics:?}");
}

#[test]
fn canonical_ontology_fixture_validates() {
    let diagnostics = validate_hollow_grove_alignment(&canonical_ontology_fixture());
    assert!(diagnostics.is_empty(), "{diagnostics:?}");
}

#[test]
fn canonical_house_goods_fixture_validates() {
    let diagnostics = validate_hollow_grove_alignment(&canonical_house_goods_fixture());
    assert!(diagnostics.is_empty(), "{diagnostics:?}");
}

#[test]
fn canonical_medicine_fixture_validates_without_profession_locks() {
    let diagnostics = validate_hollow_grove_alignment(&canonical_medicine_fixture());
    assert!(diagnostics.is_empty(), "{diagnostics:?}");
}

#[test]
fn canonical_root_alignment_fixture_validates() {
    let diagnostics = validate_hollow_grove_alignment(&canonical_root_alignment_fixture());
    assert!(diagnostics.is_empty(), "{diagnostics:?}");
}

#[test]
fn contradiction_frame_cannot_be_hospital() {
    let input = HollowGroveAlignmentInput {
        entity_claims: vec![EntityClaim {
            name: String::from("GlaushouseHospital"),
            category: EntityCategory::Frame,
        }],
        ..HollowGroveAlignmentInput::default()
    };

    let diagnostics = validate_hollow_grove_alignment(&input);
    assert!(!diagnostics.is_empty());
}

#[test]
fn contradiction_hollow_cannot_be_clear_blood() {
    let input = HollowGroveAlignmentInput {
        substance_identity_claims: vec![SubstanceIdentityClaim {
            substance: Substance::Hollow,
            identity: String::from("clear blood"),
        }],
        ..HollowGroveAlignmentInput::default()
    };

    let diagnostics = validate_hollow_grove_alignment(&input);
    assert!(!diagnostics.is_empty());
}

#[test]
fn contradiction_hollowing_cannot_be_current_containment() {
    let input = HollowGroveAlignmentInput {
        hollowing_claims: vec![HollowingClaim {
            mode: HollowingMode::Containment,
            extracts_useful_interior: false,
            preserves_outer_form: false,
        }],
        ..HollowGroveAlignmentInput::default()
    };

    let diagnostics = validate_hollow_grove_alignment(&input);
    assert!(!diagnostics.is_empty());
}

#[test]
fn contradiction_nightingale_cannot_be_generic_nurse_species() {
    let input = HollowGroveAlignmentInput {
        nightingale_claims: vec![NightingaleClaim {
            identity: NightingaleIdentity::GenericNurseSpecies,
            origin: House::Glaushouse,
            medium: Substance::Aura,
            functions: vec![NightingaleFunction::PrepareTissueForRecovery],
        }],
        ..HollowGroveAlignmentInput::default()
    };

    let diagnostics = validate_hollow_grove_alignment(&input);
    assert!(!diagnostics.is_empty());
}

#[test]
fn contradiction_minorian_cannot_be_elf() {
    let input = HollowGroveAlignmentInput {
        sandmanor_people_claims: vec![SandmanorPeopleClaim {
            people: SandmanorPeople::Minorian,
            lineage: Lineage::Elf,
        }],
        ..HollowGroveAlignmentInput::default()
    };

    let diagnostics = validate_hollow_grove_alignment(&input);
    assert!(!diagnostics.is_empty());
}

#[test]
fn contradiction_minoan_cannot_be_gnome() {
    let input = HollowGroveAlignmentInput {
        sandmanor_people_claims: vec![SandmanorPeopleClaim {
            people: SandmanorPeople::Minoan,
            lineage: Lineage::Gnome,
        }],
        ..HollowGroveAlignmentInput::default()
    };

    let diagnostics = validate_hollow_grove_alignment(&input);
    assert!(!diagnostics.is_empty());
}

#[test]
fn contradiction_gnomes_cannot_transform_into_gargoyles() {
    let input = HollowGroveAlignmentInput {
        gnome_progression_claims: vec![GnomeProgressionClaim {
            has_evolution_ladder: true,
            evolves_into_gargoyle: true,
        }],
        ..HollowGroveAlignmentInput::default()
    };

    let diagnostics = validate_hollow_grove_alignment(&input);
    assert!(!diagnostics.is_empty());
}

#[test]
fn sandmanor_gnome_lineage_is_not_rejected_as_a_gargoyle_path() {
    let input = HollowGroveAlignmentInput {
        gnome_progression_claims: vec![GnomeProgressionClaim {
            has_evolution_ladder: true,
            evolves_into_gargoyle: false,
        }],
        ..HollowGroveAlignmentInput::default()
    };

    assert!(validate_hollow_grove_alignment(&input).is_empty());
}

#[test]
fn contradiction_only_native_glaushouse_species_may_be_doctors_fails() {
    let input = HollowGroveAlignmentInput {
        profession_claims: vec![ProfessionClaim {
            species: String::from("Elf"),
            profession: Profession::GeneralDoctor,
            medical_discipline: House::Glaushouse,
            access_rule: ProfessionAccessRule::LockedToNativeHouse,
        }],
        ..HollowGroveAlignmentInput::default()
    };

    let diagnostics = validate_hollow_grove_alignment(&input);
    assert!(!diagnostics.is_empty());
}

#[test]
fn contradiction_marrow_cannot_move_out_of_stonebend() {
    let input = HollowGroveAlignmentInput {
        house_anchor_claims: vec![HouseAnchorClaim {
            anchor: HouseAnchor::Marrow,
            primary_house: House::Sandmanor,
        }],
        ..HollowGroveAlignmentInput::default()
    };

    let diagnostics = validate_hollow_grove_alignment(&input);
    assert!(!diagnostics.is_empty());
}

#[test]
fn contradiction_isolated_houses_fail_alignment() {
    let input = HollowGroveAlignmentInput {
        civilization_claims: vec![CivilizationModel::IsolatedHouses],
        ..HollowGroveAlignmentInput::default()
    };

    let diagnostics = validate_hollow_grove_alignment(&input);
    assert!(!diagnostics.is_empty());
}

#[test]
fn contradiction_house_rocks_cannot_replace_primary_substances() {
    let input = HollowGroveAlignmentInput {
        house_rock_claims: vec![HouseRockClaim {
            house: House::Stonebend,
            rock: HouseRock::Diamond,
            operation: String::from("claims"),
            replaces_substance: Some(Substance::Current),
            visible_style: None,
        }],
        ..HollowGroveAlignmentInput::default()
    };

    let diagnostics = validate_hollow_grove_alignment(&input);
    assert!(!diagnostics.is_empty());
}

#[test]
fn pass_examples_cover_frame_scene_structure_and_system() {
    let input = HollowGroveAlignmentInput {
        entity_claims: vec![
            EntityClaim {
                name: String::from("Gargoyle"),
                category: EntityCategory::Frame,
            },
            EntityClaim {
                name: String::from("Surgery"),
                category: EntityCategory::Scene,
            },
            EntityClaim {
                name: String::from("Hospital"),
                category: EntityCategory::Structure,
            },
            EntityClaim {
                name: String::from("MedicalNetwork"),
                category: EntityCategory::System,
            },
        ],
        frame_grammar_claims: vec![FrameGrammarClaim {
            subject: String::from("Gargoyle"),
            category: EntityCategory::Frame,
            uses_frame_flow_glow: true,
        }],
        ..canonical_material_fixture()
    };

    let diagnostics = validate_hollow_grove_alignment(&input);
    assert!(diagnostics.is_empty(), "{diagnostics:?}");
}

#[test]
fn pass_examples_allow_mixed_species_medical_professions() {
    let input = HollowGroveAlignmentInput {
        profession_claims: vec![
            ProfessionClaim {
                species: String::from("Elf"),
                profession: Profession::Radiologist,
                medical_discipline: House::Glaushouse,
                access_rule: ProfessionAccessRule::Open,
            },
            ProfessionClaim {
                species: String::from("Gargoyle"),
                profession: Profession::Surgeon,
                medical_discipline: House::Glaushouse,
                access_rule: ProfessionAccessRule::Open,
            },
            ProfessionClaim {
                species: String::from("Werewolf"),
                profession: Profession::Nurse,
                medical_discipline: House::Glaushouse,
                access_rule: ProfessionAccessRule::Open,
            },
            ProfessionClaim {
                species: String::from("Gnome"),
                profession: Profession::EmergencyPhysician,
                medical_discipline: House::Glaushouse,
                access_rule: ProfessionAccessRule::Open,
            },
        ],
        care_level_claims: vec![
            CareLevelClaim {
                care_level: CareLevel::LocalCareCenter,
                functions: vec![
                    CareFunction::Rest,
                    CareFunction::Stabilize,
                    CareFunction::Clean,
                    CareFunction::Replenish,
                    CareFunction::Return,
                    CareFunction::ReferSeriousCases,
                ],
            },
            CareLevelClaim {
                care_level: CareLevel::AdvancedGlaushouseFacility,
                functions: vec![
                    CareFunction::Diagnose,
                    CareFunction::Clear,
                    CareFunction::Reconstruct,
                    CareFunction::Rehabilitate,
                    CareFunction::Transform,
                ],
            },
        ],
        civilization_claims: vec![CivilizationModel::Connected],
        ..HollowGroveAlignmentInput::default()
    };

    let diagnostics = validate_hollow_grove_alignment(&input);
    assert!(diagnostics.is_empty(), "{diagnostics:?}");
}

#[test]
fn validation_report_stays_green() {
    let report = build_hollow_grove_alignment_validation_report();
    assert!(report.contains("status: pass"), "{report}");
}
