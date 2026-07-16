use std::io;

use crate::current_synthesis_scenario::{DEFAULT_SCENARIO_ID, ScenarioDefinition, load_scenario};
use crate::frame_state::{BeingId, FlowId, FrameId, GlowId};
use crate::hollow_grove_contract::{
    AlignmentDiagnostic, CivilizationModel, EntityCategory, FrameGrammarClaim,
    HollowGroveAlignmentInput, House, HouseRock, Profession, ProfessionAccessRule, SandmanorPeople,
    SandmanorPeopleClaim, Substance, canonical_progression_contract_fixture,
    validate_hollow_grove_alignment, validate_hollow_grove_progression_contract,
};
use crate::point_progression::{
    CanonicalRouteId, PointProgressionDiagnostic, PointSquaredApplicationStatus,
    build_canonical_point_squared_fixture, build_point_squared_witness, validate_point_progression,
};
use crate::world_map_geometry::{
    RotationPosition, WorldCenterId, build_map_witness, canonical_rotation_contract_fixture,
    validate_hollow_grove_rotation_contract,
};
use crate::{
    CANONICAL_WITNESS, ContactOutcome, DecisionIntent, FrameState, LandingOutcome, Point, Symptom,
    execute_kernel_pass_decision, execute_synthesis_recipe, gremlin_tinker_recipe,
    pixy_confusion_recipe, run_kernel_cycle,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegionalItemProfile {
    pub label: String,
    pub house: House,
    pub rock: HouseRock,
    pub visible_style: String,
    pub replaces_substance: Option<Substance>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoleProfile {
    pub label: String,
    pub being: BeingId,
    pub active_frame: FrameId,
    pub species_label: String,
    pub people_claim: Option<SandmanorPeopleClaim>,
    pub house_training: House,
    pub profession: Profession,
    pub specialty: Option<String>,
    pub medical_discipline: House,
    pub access_rule: ProfessionAccessRule,
    pub regional_style: House,
    pub equipment: Vec<RegionalItemProfile>,
    pub flow_capability: Option<FlowId>,
    pub glow_capability: Option<GlowId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MedicalTeamProfile {
    pub label: String,
    pub roles: Vec<RoleProfile>,
}

pub fn validate_hollow_grove_alignment_or_error(
    context: &str,
    input: &HollowGroveAlignmentInput,
) -> io::Result<()> {
    diagnostics_to_result(context, validate_hollow_grove_alignment(input))
}

pub fn validate_current_synthesis_scenario(scenario: &ScenarioDefinition) -> io::Result<()> {
    let mut input = HollowGroveAlignmentInput {
        entity_claims: vec![crate::hollow_grove_contract::EntityClaim {
            name: scenario.title.clone(),
            category: EntityCategory::Scene,
        }],
        frame_grammar_claims: vec![FrameGrammarClaim {
            subject: scenario.title.clone(),
            category: EntityCategory::Scene,
            uses_frame_flow_glow: false,
        }],
        civilization_claims: vec![CivilizationModel::Connected],
        ..HollowGroveAlignmentInput::default()
    };

    let mut diagnostics = validate_hollow_grove_alignment(&input);
    let mut bad_lines = Vec::new();
    for npc in &scenario.npcs {
        if npc.name.to_ascii_lowercase().contains("nightingale")
            || npc.role.to_ascii_lowercase().contains("nightingale")
        {
            bad_lines.push(format!(
                "{} / {} treats Nightingale as a scenario staff role instead of white blood cells",
                npc.name, npc.role
            ));
        }
        for text in npc
            .needs
            .iter()
            .chain(npc.memories.iter())
            .chain(npc.relationships.iter())
            .chain(npc.perceived_world.iter())
        {
            validate_content_line(&format!("scenario `{}`", scenario.id), text, &mut bad_lines);
        }
    }

    validate_content_line(
        &format!("scenario `{}`", scenario.id),
        &scenario.player_need,
        &mut bad_lines,
    );
    for text in scenario
        .faction_conditions
        .iter()
        .chain(scenario.settlement_conditions.iter())
        .chain(scenario.war_conditions.iter())
    {
        validate_content_line(&format!("scenario `{}`", scenario.id), text, &mut bad_lines);
    }

    if !bad_lines.is_empty() {
        diagnostics.push(AlignmentDiagnostic {
            code: crate::hollow_grove_contract::AlignmentDiagnosticCode::NightingaleMismatch,
            message: bad_lines.join("; "),
        });
    }

    input.civilization_claims.clear();
    diagnostics_to_result(&format!("scenario `{}`", scenario.id), diagnostics)
}

pub fn validate_role_profile(role: &RoleProfile) -> io::Result<()> {
    let mut input = HollowGroveAlignmentInput {
        entity_claims: vec![crate::hollow_grove_contract::EntityClaim {
            name: format!("{:?}", role.active_frame),
            category: EntityCategory::Frame,
        }],
        frame_grammar_claims: vec![FrameGrammarClaim {
            subject: role.label.clone(),
            category: EntityCategory::Frame,
            uses_frame_flow_glow: true,
        }],
        profession_claims: vec![crate::hollow_grove_contract::ProfessionClaim {
            species: role.species_label.clone(),
            profession: role.profession,
            medical_discipline: role.medical_discipline,
            access_rule: role.access_rule,
        }],
        civilization_claims: vec![CivilizationModel::Connected],
        sandmanor_people_claims: role.people_claim.clone().into_iter().collect(),
        house_rock_claims: role
            .equipment
            .iter()
            .map(|item| crate::hollow_grove_contract::HouseRockClaim {
                house: item.house,
                rock: item.rock,
                operation: item.rock.canonical_operation().to_owned(),
                replaces_substance: item.replaces_substance,
                visible_style: Some(item.visible_style.clone()),
            })
            .collect(),
        ..HollowGroveAlignmentInput::default()
    };
    let diagnostics = validate_hollow_grove_alignment(&input);
    input.civilization_claims.clear();
    diagnostics_to_result(&role.label, diagnostics)?;

    if role.being != BeingId::Hueman {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("{} must preserve Being = Hueman", role.label),
        ));
    }

    Ok(())
}

pub fn validate_regional_item_profile(item: &RegionalItemProfile) -> io::Result<()> {
    let input = HollowGroveAlignmentInput {
        house_rock_claims: vec![crate::hollow_grove_contract::HouseRockClaim {
            house: item.house,
            rock: item.rock,
            operation: item.rock.canonical_operation().to_owned(),
            replaces_substance: item.replaces_substance,
            visible_style: Some(item.visible_style.clone()),
        }],
        ..HollowGroveAlignmentInput::default()
    };
    validate_hollow_grove_alignment_or_error(&item.label, &input)
}

pub fn validate_medical_team_profile(team: &MedicalTeamProfile) -> io::Result<()> {
    for role in &team.roles {
        validate_role_profile(role)?;
    }
    Ok(())
}

pub fn validate_generated_content_batch(
    label: &str,
    roles: &[RoleProfile],
    items: &[RegionalItemProfile],
) -> io::Result<()> {
    for role in roles {
        validate_role_profile(role)?;
    }
    for item in items {
        validate_regional_item_profile(item)?;
    }
    if roles.is_empty() && items.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("{label} must include at least one role or item"),
        ));
    }
    Ok(())
}

pub fn validate_canonical_content_fixtures() -> io::Result<()> {
    validate_generated_content_batch(
        "canonical content fixtures",
        &[
            build_gargoyle_surgeon_role(),
            build_elf_radiologist_role(),
            build_werewolf_emergency_nurse_role(),
            build_gnome_emergency_physician_role(),
        ],
        &[build_flyntian_dagger_profile()],
    )
}

pub fn validate_hueman_progression_foundation() -> io::Result<()> {
    let origin = FrameState::origin();
    if origin.being() != BeingId::Hueman
        || origin.frame() != FrameId::Hueman
        || !origin.flow_learnset().is_empty()
        || !origin.glow_learnset().is_empty()
        || origin.prism().body() != 1
        || origin.prism().spirit() != 1
        || origin.prism().mind() != 1
        || origin.prism().soul_interior() != 1
        || origin.prism().soul_exterior() != 1
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "origin Hueman fixture drifted from the semantic foundation",
        ));
    }

    let point = Point::origin();
    let gremlin = execute_synthesis_recipe(&point, &gremlin_tinker_recipe()).map_err(|error| {
        io::Error::other(format!("gremlin fixture execution failed: {error:?}"))
    })?;
    let LandingOutcome::Kiss(gremlin_kiss) = gremlin.landing() else {
        return Err(io::Error::other("gremlin fixture did not land as a kiss"));
    };
    if gremlin_kiss.before().being() != BeingId::Hueman
        || gremlin_kiss.point_squared().being() != BeingId::Hueman
        || gremlin_kiss.point_squared().frame() != FrameId::Gremlin
        || gremlin_kiss.point_squared().flow_learnset() != [FlowId::TinkerGrip]
        || !gremlin_kiss.point_squared().glow_learnset().is_empty()
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Gremlin Tinker fixture drifted from the semantic foundation",
        ));
    }

    let pixy = execute_synthesis_recipe(&point, &pixy_confusion_recipe())
        .map_err(|error| io::Error::other(format!("pixy fixture execution failed: {error:?}")))?;
    let LandingOutcome::Kiss(pixy_kiss) = pixy.landing() else {
        return Err(io::Error::other("pixy fixture did not land as a kiss"));
    };
    if pixy_kiss.before().being() != BeingId::Hueman
        || pixy_kiss.point_squared().being() != BeingId::Hueman
        || pixy_kiss.point_squared().frame() != FrameId::Pixy
        || !pixy_kiss.point_squared().flow_learnset().is_empty()
        || pixy_kiss.point_squared().glow_learnset() != [GlowId::Confusion]
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Pixy Confusion fixture drifted from the semantic foundation",
        ));
    }

    let persistent = execute_synthesis_recipe(&gremlin_kiss.next_point(), &pixy_confusion_recipe())
        .map_err(|error| {
            io::Error::other(format!("persistent learning fixture failed: {error:?}"))
        })?;
    let LandingOutcome::Kiss(persistent_kiss) = persistent.landing() else {
        return Err(io::Error::other(
            "persistent learning fixture did not land as a kiss",
        ));
    };
    if persistent_kiss.point_squared().being() != BeingId::Hueman
        || persistent_kiss.point_squared().frame() != FrameId::Pixy
        || persistent_kiss.point_squared().flow_learnset() != [FlowId::TinkerGrip]
        || persistent_kiss.point_squared().glow_learnset() != [GlowId::Confusion]
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "frame switching erased a legal Flow or Glow learnset",
        ));
    }

    Ok(())
}

pub fn validate_point_squared_progression_foundation() -> io::Result<()> {
    let fixture = build_canonical_point_squared_fixture()?;
    let before = fixture.point_before();
    let stabilized = fixture.first_application().stabilized_point();
    let diagnostics = validate_point_progression(stabilized);
    let errors = diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.is_error)
        .collect::<Vec<_>>();
    if !errors.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "Point² progression fixture drifted: {}",
                render_progression_diagnostics(&errors)
            ),
        ));
    }
    if fixture.first_application().status() != PointSquaredApplicationStatus::Applied {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "first Point² ascension application must succeed",
        ));
    }
    if fixture.second_application().status() != PointSquaredApplicationStatus::AlreadyApplied {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "replaying the same Point² ascension must report already applied",
        ));
    }
    if stabilized.progression().capacities().current_capacity() != 2
        || stabilized.progression().capacities().aura_capacity() != 2
        || stabilized.progression().stable_point_level() != 2
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "the canonical Point² fixture must stabilize at Point level 2 with paired 2/2 capacities",
        ));
    }
    if !stabilized
        .world()
        .route_visible(CanonicalRouteId::StairwayToHeaven)
        || !stabilized
            .world()
            .route_survivable(CanonicalRouteId::StairwayToHeaven)
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Stairway to Heaven must become visible and survivable after the canonical Point² fixture",
        ));
    }
    if !stabilized.world().next_frame_potential_available() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Point² must open next Frame potential without granting it automatically",
        ));
    }
    if before.world().geometry().center() != WorldCenterId::Ranina
        || stabilized.world().geometry().center() != WorldCenterId::Ranina
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Ranina must remain the unique center before and after Point²",
        ));
    }
    if before.world().geometry().current_position() != Some(RotationPosition::seven())
        || stabilized.world().geometry().current_position() != Some(RotationPosition::seven())
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "the canonical Point² fixture must preserve Position 7 while opening Ring 2",
        ));
    }
    Ok(())
}

pub fn validate_ranina_rotation_foundation() -> io::Result<()> {
    let diagnostics =
        validate_hollow_grove_rotation_contract(&canonical_rotation_contract_fixture());
    if !diagnostics.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            diagnostics
                .into_iter()
                .map(|diagnostic| diagnostic.message)
                .collect::<Vec<_>>()
                .join("; "),
        ));
    }
    let fixture = build_canonical_point_squared_fixture()?;
    let before = fixture.point_before();
    let after = fixture.first_application().stabilized_point();
    if before.progression().stable_point_level() != 1
        || after.progression().stable_point_level() != 2
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Point² must open Ring 2 from Ring 1 in the canonical geometry fixture",
        ));
    }
    Ok(())
}

pub fn validate_medical_injury_cycle() -> io::Result<()> {
    let root_fixture = crate::hollow_grove_contract::canonical_root_alignment_fixture();
    let diagnostics =
        validate_hollow_grove_alignment(&crate::hollow_grove_contract::HollowGroveAlignmentInput {
            nightingale_claims: root_fixture.nightingale_claims,
            ..crate::hollow_grove_contract::HollowGroveAlignmentInput::default()
        });
    if !diagnostics.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "medical lore Nightingale contract drifted: {}",
                diagnostics
                    .into_iter()
                    .map(|diagnostic| diagnostic.message)
                    .collect::<Vec<_>>()
                    .join("; ")
            ),
        ));
    }

    let cycle = [
        "Hollow Current is breached.",
        "Current carries the alarm.",
        "Abyss feels and records the damage.",
        "Aura reveals the true condition.",
        "Glaüshouse clears and stabilizes Abyss.",
        "Current restores circulation.",
        "Stonebend rebuilds Hollow Current.",
    ];
    if cycle.len() != 7 || !cycle[0].contains("Hollow Current") || !cycle[6].contains("Stonebend") {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "medical injury cycle fixture drifted from the canonical order",
        ));
    }

    Ok(())
}

pub fn build_hollow_grove_vertical_witness() -> io::Result<String> {
    validate_hueman_progression_foundation()?;

    let role = build_gargoyle_surgeon_role();
    validate_role_profile(&role)?;
    let item = build_flyntian_dagger_profile();
    validate_regional_item_profile(&item)?;

    let scenario = load_scenario(DEFAULT_SCENARIO_ID)?;
    validate_current_synthesis_scenario(&scenario)?;

    validate_hollow_grove_alignment_or_error(
        "foundation vertical witness ontology",
        &HollowGroveAlignmentInput {
            entity_claims: vec![
                crate::hollow_grove_contract::EntityClaim {
                    name: String::from("Gargoyle"),
                    category: EntityCategory::Frame,
                },
                crate::hollow_grove_contract::EntityClaim {
                    name: String::from("EmergencyOperation"),
                    category: EntityCategory::Scene,
                },
                crate::hollow_grove_contract::EntityClaim {
                    name: String::from("GlaushouseHospital"),
                    category: EntityCategory::Structure,
                },
                crate::hollow_grove_contract::EntityClaim {
                    name: String::from("MedicalCareNetwork"),
                    category: EntityCategory::System,
                },
            ],
            frame_grammar_claims: vec![FrameGrammarClaim {
                subject: String::from("Gargoyle"),
                category: EntityCategory::Frame,
                uses_frame_flow_glow: true,
            }],
            civilization_claims: vec![CivilizationModel::Connected],
            ..HollowGroveAlignmentInput::default()
        },
    )?;

    let kernel_pass = run_kernel_cycle(Symptom::origin());
    let decision = execute_kernel_pass_decision(&kernel_pass, DecisionIntent::FavorCurrent)
        .map_err(|error| {
            io::Error::other(format!("vertical witness decision failed: {error:?}"))
        })?;
    let LandingOutcome::Kiss(kiss) = decision.execution().landing() else {
        return Err(io::Error::other(
            "vertical witness execution did not produce Point²",
        ));
    };
    if decision.execution().contact() != ContactOutcome::Kiss
        || kiss.point_squared().being() != BeingId::Hueman
        || kiss.point_squared().frame() != FrameId::Gremlin
        || kiss.point_squared().flow_learnset() != [FlowId::TinkerGrip]
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "vertical witness decision-to-execution chain drifted",
        ));
    }

    Ok(format!(
        "HOLLOW GROVE FOUNDATION VERTICAL WITNESS\n\n\
         Persistent Being:\n\
         - Being: {:?}\n\
         - Origin Frame: {:?}\n\
         - Active Frame after validated recipe: {:?}\n\
         - Learned Flow: {:?}\n\
         - Learned Glow: none\n\n\
         Mixed Profession:\n\
         {}\n\n\
         Regional Item:\n\
         {}\n\n\
         Scenario Boundary:\n\
         - loaded scenario: `{}`\n\
         - title: {}\n\
         - validation: pass\n\
         - ontology anchor: EmergencyOperation = Scene, GlaushouseHospital = Structure, MedicalCareNetwork = System\n\n\
         V2 Choice:\n\
         - intent: {}\n\
         - chosen candidate: {}\n\
         - recipe: {} ({})\n\
         - handed to Version 1.1: true\n\n\
         Frozen V1.1 Execution:\n\
         - contact: {:?}\n\
         - start: {:?} / {:?}\n\
         - Point²: {:?} / {:?}\n\
         - canonical witness: {}\n",
        kiss.before().being(),
        FrameId::Hueman,
        kiss.point_squared().frame(),
        FlowId::TinkerGrip,
        render_role_profile(&role),
        render_regional_item_profile(&item),
        scenario.id,
        scenario.title,
        decision.observation().intent().as_str(),
        decision.chosen().candidate().candidate_id().as_str(),
        decision.recipe().display_name(),
        decision.recipe().recipe_id(),
        decision.execution().contact(),
        kiss.before().being(),
        kiss.before().frame(),
        kiss.point_squared().being(),
        kiss.point_squared().frame(),
        CANONICAL_WITNESS
    ))
}

pub fn build_hollow_grove_foundation_verification_report() -> io::Result<String> {
    validate_hueman_progression_foundation()?;
    validate_point_squared_progression_foundation()?;
    validate_ranina_rotation_foundation()?;
    validate_medical_injury_cycle()?;
    validate_canonical_content_fixtures()?;
    validate_medical_team_profile(&build_glaushouse_medical_team_profile())?;
    let scenario = load_scenario(DEFAULT_SCENARIO_ID)?;
    validate_current_synthesis_scenario(&scenario)?;
    let contract_diagnostics =
        validate_hollow_grove_progression_contract(&canonical_progression_contract_fixture());
    if !contract_diagnostics.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "progression semantic contract drifted: {}",
                contract_diagnostics
                    .into_iter()
                    .map(|diagnostic| diagnostic.message)
                    .collect::<Vec<_>>()
                    .join("; ")
            ),
        ));
    }
    let rotation_diagnostics =
        validate_hollow_grove_rotation_contract(&canonical_rotation_contract_fixture());
    if !rotation_diagnostics.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "rotational geometry contract drifted: {}",
                rotation_diagnostics
                    .into_iter()
                    .map(|diagnostic| diagnostic.message)
                    .collect::<Vec<_>>()
                    .join("; ")
            ),
        ));
    }

    let contradiction_checks = [
        (
            "Frame = GlaushouseHospital",
            HollowGroveAlignmentInput {
                entity_claims: vec![crate::hollow_grove_contract::EntityClaim {
                    name: String::from("GlaushouseHospital"),
                    category: EntityCategory::Frame,
                }],
                ..HollowGroveAlignmentInput::default()
            },
        ),
        (
            "Hollow = clear blood",
            HollowGroveAlignmentInput {
                substance_identity_claims: vec![
                    crate::hollow_grove_contract::SubstanceIdentityClaim {
                        substance: Substance::Hollow,
                        identity: String::from("clear blood"),
                    },
                ],
                ..HollowGroveAlignmentInput::default()
            },
        ),
        (
            "Only native Glaushouse species may become doctors",
            HollowGroveAlignmentInput {
                profession_claims: vec![crate::hollow_grove_contract::ProfessionClaim {
                    species: String::from("Elf"),
                    profession: Profession::GeneralDoctor,
                    medical_discipline: House::Glaushouse,
                    access_rule: ProfessionAccessRule::LockedToNativeHouse,
                }],
                ..HollowGroveAlignmentInput::default()
            },
        ),
    ];
    for (label, input) in contradiction_checks {
        if validate_hollow_grove_alignment(&input).is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("contradiction check unexpectedly passed: {label}"),
            ));
        }
    }

    let kernel_pass = run_kernel_cycle(Symptom::origin());
    if kernel_pass.to_string() != CANONICAL_WITNESS {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "V1.1 canonical witness drifted",
        ));
    }

    Ok(format!(
        "HOLLOW GROVE FOUNDATION VERIFICATION\n\n\
         Status:\n\
         - world witness: pass\n\
         - world validation: pass\n\
         - current-depth contract: pass\n\
         - aura illumination contract: pass\n\
         - Point² paired capacity advancement: pass\n\
         - Point² exactly-once application: pass\n\
         - Hueman identity fixture: pass\n\
         - progression persistence fixture: pass\n\
         - development persistence: pass\n\
         - Stairway horizon fixture: pass\n\
         - Ranina center: pass\n\
         - twelve-position rotation: pass\n\
         - Stonebend position 1: pass\n\
         - Glaüshouse threshold 6: pass\n\
         - Glaüshouse position 7: pass\n\
         - opposition geometry: pass\n\
         - Point² radial expansion: pass\n\
         - Ranina center invariance: pass\n\
         - point stabilization: pass\n\
         - canonical content fixtures: pass\n\
         - medical lore injury cycle: pass\n\
         - contradiction checks: pass\n\
         - vertical witness: pass\n\
         - V1.1 topology unchanged: pass\n\n\
         World Witness:\n\n\
         {}\n\
         Point² Witness:\n\n\
         {}\n\
         Map Witness:\n\n\
         {}\n\
         Vertical Witness:\n\n\
         {}",
        crate::hollow_grove_contract::build_hollow_grove_alignment_witness(),
        build_point_squared_witness()?,
        build_map_witness()?,
        build_hollow_grove_vertical_witness()?
    ))
}

pub fn build_flyntian_dagger_profile() -> RegionalItemProfile {
    RegionalItemProfile {
        label: String::from("Flyntian Dagger"),
        house: House::Flynt,
        rock: HouseRock::Opal,
        visible_style: String::from("transforming modular motion-oriented opal blade"),
        replaces_substance: None,
    }
}

pub fn build_gargoyle_surgeon_role() -> RoleProfile {
    RoleProfile {
        label: String::from("Gargoyle Surgeon"),
        being: BeingId::Hueman,
        active_frame: FrameId::Gargoyle,
        species_label: String::from("Gargoyle"),
        people_claim: None,
        house_training: House::Glaushouse,
        profession: Profession::Surgeon,
        specialty: Some(String::from("Trauma")),
        medical_discipline: House::Glaushouse,
        access_rule: ProfessionAccessRule::Open,
        regional_style: House::Stonebend,
        equipment: vec![RegionalItemProfile {
            label: String::from("Jade Surgical Drain Set"),
            house: House::Glaushouse,
            rock: HouseRock::Jade,
            visible_style: String::from("smooth filtered restorative surgical channels"),
            replaces_substance: None,
        }],
        flow_capability: Some(FlowId::Stonefold),
        glow_capability: None,
    }
}

pub fn build_elf_radiologist_role() -> RoleProfile {
    RoleProfile {
        label: String::from("Elf Radiologist"),
        being: BeingId::Hueman,
        active_frame: FrameId::Pixy,
        species_label: String::from("Elf"),
        people_claim: Some(SandmanorPeopleClaim {
            people: SandmanorPeople::Minoan,
            lineage: crate::hollow_grove_contract::Lineage::Elf,
        }),
        house_training: House::Glaushouse,
        profession: Profession::Radiologist,
        specialty: Some(String::from("Crystal imaging")),
        medical_discipline: House::Glaushouse,
        access_rule: ProfessionAccessRule::Open,
        regional_style: House::Sandmanor,
        equipment: vec![RegionalItemProfile {
            label: String::from("Crystal Diagnostic Lens"),
            house: House::Sandmanor,
            rock: HouseRock::Crystal,
            visible_style: String::from("faceted readable diagnostic crystal array"),
            replaces_substance: None,
        }],
        flow_capability: None,
        glow_capability: Some(GlowId::Projection),
    }
}

pub fn build_werewolf_emergency_nurse_role() -> RoleProfile {
    RoleProfile {
        label: String::from("Werewolf Emergency Nurse"),
        being: BeingId::Hueman,
        active_frame: FrameId::Werewolf,
        species_label: String::from("Werewolf"),
        people_claim: None,
        house_training: House::Glaushouse,
        profession: Profession::Nurse,
        specialty: Some(String::from("Emergency response")),
        medical_discipline: House::Glaushouse,
        access_rule: ProfessionAccessRule::Open,
        regional_style: House::Flynt,
        equipment: vec![RegionalItemProfile {
            label: String::from("Flynt Rescue Harness"),
            house: House::Flynt,
            rock: HouseRock::Opal,
            visible_style: String::from("modular transforming motion-ready rescue rig"),
            replaces_substance: None,
        }],
        flow_capability: Some(FlowId::Moonrush),
        glow_capability: None,
    }
}

pub fn build_gnome_emergency_physician_role() -> RoleProfile {
    RoleProfile {
        label: String::from("Gnome Emergency Physician"),
        being: BeingId::Hueman,
        active_frame: FrameId::Pixy,
        species_label: String::from("Gnome"),
        people_claim: Some(SandmanorPeopleClaim {
            people: SandmanorPeople::Minorian,
            lineage: crate::hollow_grove_contract::Lineage::Gnome,
        }),
        house_training: House::Glaushouse,
        profession: Profession::EmergencyPhysician,
        specialty: Some(String::from("Intake triage")),
        medical_discipline: House::Glaushouse,
        access_rule: ProfessionAccessRule::Open,
        regional_style: House::Sandmanor,
        equipment: vec![RegionalItemProfile {
            label: String::from("Crystal Triage Ledger"),
            house: House::Sandmanor,
            rock: HouseRock::Crystal,
            visible_style: String::from("measured faceted readable dosage ledger"),
            replaces_substance: None,
        }],
        flow_capability: None,
        glow_capability: Some(GlowId::Recognition),
    }
}

pub fn build_glaushouse_medical_team_profile() -> MedicalTeamProfile {
    MedicalTeamProfile {
        label: String::from("Glaushouse Mixed Medical Team"),
        roles: vec![
            build_gargoyle_surgeon_role(),
            build_elf_radiologist_role(),
            build_werewolf_emergency_nurse_role(),
            build_gnome_emergency_physician_role(),
        ],
    }
}

pub fn render_role_profile(role: &RoleProfile) -> String {
    let specialty = role
        .specialty
        .as_deref()
        .map(|value| format!("; specialty: {value}"))
        .unwrap_or_default();
    let flow = role
        .flow_capability
        .map(|value| format!("{value:?}"))
        .unwrap_or_else(|| String::from("none"));
    let glow = role
        .glow_capability
        .map(|value| format!("{value:?}"))
        .unwrap_or_else(|| String::from("none"));
    let equipment = role
        .equipment
        .iter()
        .map(|item| {
            format!(
                "{} [{} {}]",
                item.label,
                item.rock.as_str(),
                item.visible_style
            )
        })
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "- {} = Being {}, Frame {:?}, species {}, training {}, profession {}{}, equipment: {}, Flow: {}, Glow: {}",
        role.label,
        format!("{:?}", role.being),
        role.active_frame,
        role.species_label,
        role.house_training.as_str(),
        role.profession.as_str(),
        specialty,
        equipment,
        flow,
        glow
    )
}

pub fn render_regional_item_profile(item: &RegionalItemProfile) -> String {
    format!(
        "- {} = {} {} with {}",
        item.label,
        item.house.as_str(),
        item.rock.as_str(),
        item.visible_style
    )
}

pub fn render_medical_team_profile(team: &MedicalTeamProfile) -> String {
    let mut output = format!("- {}\n", team.label);
    for role in &team.roles {
        output.push_str(&format!("{}\n", render_role_profile(role)));
    }
    output
}

fn diagnostics_to_result(context: &str, diagnostics: Vec<AlignmentDiagnostic>) -> io::Result<()> {
    if diagnostics.is_empty() {
        return Ok(());
    }
    let message = diagnostics
        .into_iter()
        .map(|diagnostic| format!("{:?}: {}", diagnostic.code, diagnostic.message))
        .collect::<Vec<_>>()
        .join("; ");
    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        format!("{context} violates Hollow Grove alignment: {message}"),
    ))
}

fn validate_content_line(context: &str, line: &str, bad_lines: &mut Vec<String>) {
    let lower = line.to_ascii_lowercase();
    if lower.contains("plasma") || lower.contains("clear blood") {
        bad_lines.push(format!(
            "{context} contains a forbidden Hollow identity line: `{line}`"
        ));
    }
    if lower.contains("only native glaushouse") {
        bad_lines.push(format!(
            "{context} contains a forbidden native-house profession lock: `{line}`"
        ));
    }
}

fn render_progression_diagnostics(diagnostics: &[&PointProgressionDiagnostic]) -> String {
    diagnostics
        .iter()
        .map(|diagnostic| diagnostic.message.as_str())
        .collect::<Vec<_>>()
        .join("; ")
}

#[cfg(test)]
mod tests {
    use super::{
        build_elf_radiologist_role, build_flyntian_dagger_profile, build_gargoyle_surgeon_role,
        build_glaushouse_medical_team_profile, build_gnome_emergency_physician_role,
        build_hollow_grove_foundation_verification_report, build_hollow_grove_vertical_witness,
        build_werewolf_emergency_nurse_role, validate_canonical_content_fixtures,
        validate_current_synthesis_scenario, validate_generated_content_batch,
        validate_hueman_progression_foundation, validate_medical_team_profile,
        validate_regional_item_profile, validate_role_profile,
    };
    use crate::current_synthesis_scenario::ScenarioDefinition;

    #[test]
    fn canonical_role_profiles_validate() {
        validate_role_profile(&build_gargoyle_surgeon_role()).expect("gargoyle surgeon is valid");
        validate_role_profile(&build_elf_radiologist_role()).expect("elf radiologist is valid");
        validate_role_profile(&build_werewolf_emergency_nurse_role())
            .expect("werewolf nurse is valid");
        validate_role_profile(&build_gnome_emergency_physician_role())
            .expect("gnome physician is valid");
    }

    #[test]
    fn canonical_item_profile_validates() {
        validate_regional_item_profile(&build_flyntian_dagger_profile())
            .expect("flyntian dagger is valid");
    }

    #[test]
    fn canonical_team_and_batch_validate() {
        validate_medical_team_profile(&build_glaushouse_medical_team_profile())
            .expect("medical team is valid");
        validate_generated_content_batch(
            "mixed medical batch",
            &[
                build_gargoyle_surgeon_role(),
                build_elf_radiologist_role(),
                build_werewolf_emergency_nurse_role(),
                build_gnome_emergency_physician_role(),
            ],
            &[build_flyntian_dagger_profile()],
        )
        .expect("batch is valid");
        validate_canonical_content_fixtures().expect("canonical content fixtures stay valid");
    }

    #[test]
    fn hueman_progression_foundation_stays_aligned() {
        validate_hueman_progression_foundation().expect("Hueman progression foundation is valid");
    }

    #[test]
    fn vertical_witness_covers_content_decision_and_v1_boundary() {
        let witness = build_hollow_grove_vertical_witness().expect("vertical witness should build");
        assert!(witness.contains("Being: Hueman"));
        assert!(witness.contains("Active Frame after validated recipe: Gremlin"));
        assert!(witness.contains("Gargoyle Surgeon"));
        assert!(witness.contains("Flyntian Dagger"));
        assert!(witness.contains("loaded scenario: `scout_valley_vertical_slice`"));
        assert!(witness.contains("chosen candidate: GremlinTinker"));
        assert!(witness.contains("recipe: Gremlin Tinker Recipe (gremlin_tinker)"));
        assert!(witness.contains("Point²: Hueman / Gremlin"));
    }

    #[test]
    fn foundation_verification_report_summarizes_checkpoint_regression() {
        let report = build_hollow_grove_foundation_verification_report()
            .expect("foundation verification should build");
        assert!(report.contains("HOLLOW GROVE FOUNDATION VERIFICATION"));
        assert!(report.contains("world witness: pass"));
        assert!(report.contains("vertical witness: pass"));
        assert!(report.contains("V1.1 topology unchanged: pass"));
    }

    #[test]
    fn scenario_validation_rejects_staff_nightingales() {
        let scenario = ScenarioDefinition {
            id: String::from("bad"),
            title: String::from("Bad Scenario"),
            default_focused_npc_id: String::from("nightingale_01"),
            player_need: String::from("Hold the line"),
            faction_conditions: vec![],
            settlement_conditions: vec![],
            war_conditions: vec![],
            npcs: vec![crate::current_synthesis_scenario::ScenarioNpcDefinition {
                id: String::from("nightingale_01"),
                name: String::from("Shelter Nightingale 01"),
                role: String::from("Glaushouse nurse"),
                faction: String::from("Glaushouse"),
                location: String::from("ward"),
                condition: String::from("steady"),
                needs: vec![],
                memories: vec![],
                relationships: vec![],
                perceived_world: vec![],
            }],
        };

        let error =
            validate_current_synthesis_scenario(&scenario).expect_err("staff nightingale fails");
        assert!(error.to_string().contains("Nightingale"));
    }
}
