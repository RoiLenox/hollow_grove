use hollow_grove::constitutional::{BondPhase, ConstitutionalPolarity, EvidenceRef};
use hollow_grove::lineage_contract::SandmanorForm;
use hollow_grove::world::hueman_faculties::*;
use hollow_grove::world::{flynt, institutional_access_fixture};
use hollow_grove::{
    FlowId, FrameId, GlowId, RecipeIntent, SynthesisRecipe, SynthesisScript, compile_recipe,
};

fn evidence(key: &str) -> EvidenceRef {
    EvidenceRef::new("hueman-faculty-test", key).unwrap()
}

fn uncertainty(faculty: HuemanFaculty) -> FacultyUncertainty {
    FacultyUncertainty {
        basis: format!("present evidence bounded for {faculty:?}"),
        confidence_basis_points: 6_000,
        guaranteed: false,
        alternatives_preserved: faculty == HuemanFaculty::Prefog,
    }
}

fn bounds(
    frame: FrameId,
    flow: Option<FlowId>,
    glow: Option<GlowId>,
    bond_phase: Option<BondPhase>,
    region: Option<FacultyRegion>,
) -> FacultyBounds {
    FacultyBounds {
        frame,
        required_flow: flow,
        required_glow: glow,
        allowed_polarities: vec![ConstitutionalPolarity::PositiveCurrentPositiveAura],
        bond_phase,
        regional_jurisdiction: region,
        institutional_recognition: None,
    }
}

fn manifestation(faculty: HuemanFaculty) -> FacultyManifestation {
    let (domain, authority, trigger, expression, bounds) = match faculty {
        HuemanFaculty::Presynce => (
            HuemanFacultyDomain::Body,
            FacultyAuthority::Stonebend,
            FacultyTrigger::PhysicalEmergence,
            FacultyExpression::EmbodiedAnticipation,
            bounds(FrameId::Gremlin, Some(FlowId::TinkerGrip), None, None, None),
        ),
        HuemanFaculty::Resynce => (
            HuemanFacultyDomain::Spirit,
            FacultyAuthority::Flynt,
            FacultyTrigger::RelationalEmergence,
            FacultyExpression::RelationalSynchronization,
            FacultyBounds {
                institutional_recognition: Some(flynt::gallows_id()),
                ..bounds(
                    FrameId::Werewolf,
                    Some(FlowId::PackRelay),
                    None,
                    Some(BondPhase::Active),
                    Some(FacultyRegion::AuraRidge),
                )
            },
        ),
        HuemanFaculty::Precog => (
            HuemanFacultyDomain::Mind,
            FacultyAuthority::Glaushouse,
            FacultyTrigger::EvidencePattern,
            FacultyExpression::ProbableContinuation,
            bounds(FrameId::Sprite, None, Some(GlowId::Projection), None, None),
        ),
        HuemanFaculty::Prefog => (
            HuemanFacultyDomain::SoulInterior,
            FacultyAuthority::SandmanorMinorian,
            FacultyTrigger::OpenPossibility,
            FacultyExpression::CandidatePossibilities {
                legal_candidate_count: 3,
            },
            bounds(
                FrameId::Gnome,
                None,
                None,
                None,
                Some(FacultyRegion::AuraFields),
            ),
        ),
        HuemanFaculty::Prefig => (
            HuemanFacultyDomain::SoulExterior,
            FacultyAuthority::SandmanorMinoan,
            FacultyTrigger::SelectedLegalCandidate,
            FacultyExpression::ProvisionalEmbodiment {
                status: PrefigEmbodimentStatus::Demonstrable,
            },
            bounds(
                FrameId::Elf,
                None,
                None,
                None,
                Some(FacultyRegion::AuraBeachAndCurrentSea),
            ),
        ),
    };
    FacultyManifestation {
        faculty,
        domain,
        authority,
        trigger,
        expression,
        uncertainty: uncertainty(faculty),
        evidence_requirements: vec![evidence(&format!("{faculty:?}"))],
        bounds,
    }
}

fn all_manifestations() -> Vec<FacultyManifestation> {
    [
        HuemanFaculty::Presynce,
        HuemanFaculty::Resynce,
        HuemanFaculty::Precog,
        HuemanFaculty::Prefog,
        HuemanFaculty::Prefig,
    ]
    .into_iter()
    .map(manifestation)
    .collect()
}

#[test]
fn canonical_faculty_ownership_and_decision_posture_are_exact() {
    assert_eq!(FACULTY_DEFINITIONS.len(), 5);
    assert_eq!(
        HuemanFaculty::Presynce.definition().authority,
        FacultyAuthority::Stonebend
    );
    assert_eq!(
        HuemanFaculty::Resynce.definition().authority,
        FacultyAuthority::Flynt
    );
    assert_eq!(
        HuemanFaculty::Precog.definition().authority,
        FacultyAuthority::Glaushouse
    );
    assert_eq!(
        HuemanFaculty::Prefog.definition().authority,
        FacultyAuthority::SandmanorMinorian
    );
    assert_eq!(
        HuemanFaculty::Prefig.definition().authority,
        FacultyAuthority::SandmanorMinoan
    );
    assert_eq!(
        HuemanFaculty::Presynce.decision_posture(),
        FacultyDecisionPosture::Observe
    );
    assert_eq!(
        HuemanFaculty::Resynce.decision_posture(),
        FacultyDecisionPosture::Observe
    );
    assert_eq!(
        HuemanFaculty::Prefog.decision_posture(),
        FacultyDecisionPosture::Generate
    );
    assert_eq!(
        HuemanFaculty::Precog.decision_posture(),
        FacultyDecisionPosture::Evaluate
    );
    assert_eq!(
        HuemanFaculty::Prefig.decision_posture(),
        FacultyDecisionPosture::ExecuteOrDemonstrate
    );
}

#[test]
fn all_five_faculties_serialize_and_replay_deterministically() {
    let manifestations = all_manifestations();
    let encoded = encode_faculty_manifestations(&manifestations).unwrap();
    assert!(String::from_utf8_lossy(&encoded).contains("\"format\":\"HGFAC\""));
    let decoded = decode_faculty_manifestations(&encoded).unwrap();
    let replayed = replay_faculty_manifestations(&manifestations).unwrap();
    assert_eq!(decoded, manifestations);
    assert_eq!(replayed, manifestations);
    assert_eq!(encode_faculty_manifestations(&replayed).unwrap(), encoded);
}

#[test]
fn ownership_trigger_and_expression_cannot_be_reassigned() {
    let mut prefog = manifestation(HuemanFaculty::Prefog);
    prefog.authority = FacultyAuthority::SandmanorMinoan;
    assert_eq!(
        validate_faculty_manifestation(&prefog),
        Err(FacultyLawError::OwnershipMismatch(HuemanFaculty::Prefog))
    );

    let mut prefig = manifestation(HuemanFaculty::Prefig);
    prefig.trigger = FacultyTrigger::OpenPossibility;
    assert_eq!(
        validate_faculty_manifestation(&prefig),
        Err(FacultyLawError::TriggerMismatch(HuemanFaculty::Prefig))
    );

    let mut precog = manifestation(HuemanFaculty::Precog);
    precog.expression = FacultyExpression::EmbodiedAnticipation;
    assert_eq!(
        validate_faculty_manifestation(&precog),
        Err(FacultyLawError::ExpressionMismatch(HuemanFaculty::Precog))
    );
}

#[test]
fn recipe_stores_optional_faculties_but_compilation_emits_only_existing_scripts() {
    let legacy = SynthesisRecipe::new(
        "faculty-free-recipe",
        "Faculty-free Recipe",
        vec![RecipeIntent::ChangeFrame(FrameId::Gremlin)],
    );
    assert!(legacy.faculty_manifestations().is_empty());

    let recipe = SynthesisRecipe::new(
        "faculty-aware-recipe",
        "Faculty-aware Recipe",
        vec![RecipeIntent::ChangeFrame(FrameId::Gremlin)],
    )
    .with_faculty_manifestations(all_manifestations());
    let scripts = compile_recipe(&recipe).unwrap();
    assert_eq!(recipe.faculty_manifestations().len(), 5);
    assert_eq!(scripts, vec![SynthesisScript::SetFrame(FrameId::Gremlin)]);
}

#[test]
fn prefog_and_prefig_cannot_directly_create_proof_or_transformation() {
    for faculty in [HuemanFaculty::Prefog, HuemanFaculty::Prefig] {
        let manifestation = manifestation(faculty);
        assert!(!manifestation.can_create_proof());
        assert!(!manifestation.independently_executes_transformation());
    }
    assert!(matches!(
        manifestation(HuemanFaculty::Prefig).expression,
        FacultyExpression::ProvisionalEmbodiment {
            status: PrefigEmbodimentStatus::Provisional | PrefigEmbodimentStatus::Demonstrable
        }
    ));
}

#[test]
fn failed_prefig_evidence_returns_to_prefog_without_erasure() {
    let failed = evidence("failed-prefig");
    let revised = evidence("revision-preserves-failure");
    let cycle = SandmanorSoulCycle {
        prefog: manifestation(HuemanFaculty::Prefog),
        prefig: manifestation(HuemanFaculty::Prefig),
        outcome: SoulCycleOutcome::Failure(vec![failed.clone()]),
        revision_evidence: vec![failed.clone(), revised],
        returns_to_prefog: true,
    };
    validate_sandmanor_soul_cycle(&cycle).unwrap();
    let mut erased_history = cycle.clone();
    erased_history.revision_evidence.clear();
    assert_eq!(
        validate_sandmanor_soul_cycle(&erased_history),
        Err(FacultyLawError::InvalidSoulCycle)
    );
    let SoulCycleOutcome::Failure(history) = cycle.outcome else {
        panic!("expected preserved failed Prefig evidence");
    };
    assert_eq!(history, vec![failed]);
    assert!(sandmanor_soul_halves_equal());
}

#[test]
fn minotaur_and_centaur_preserve_exact_regional_soul_manifestations() {
    let minotaur = RegionalSoulManifestation {
        form: SandmanorForm::Minotaur,
        faculty: HuemanFaculty::Prefog,
        authority: FacultyAuthority::SandmanorMinorian,
        region: FacultyRegion::AuraFields,
        expression: MatureSoulExpression::CultivatedPrefog,
        replaces_people_or_authority: false,
    };
    let centaur = RegionalSoulManifestation {
        form: SandmanorForm::Centaur,
        faculty: HuemanFaculty::Prefig,
        authority: FacultyAuthority::SandmanorMinoan,
        region: FacultyRegion::AuraBeachAndCurrentSea,
        expression: MatureSoulExpression::EmbodiedPrefig,
        replaces_people_or_authority: false,
    };
    validate_regional_soul_manifestation(minotaur).unwrap();
    validate_regional_soul_manifestation(centaur).unwrap();
    assert_eq!(
        canonical_regional_soul_manifestations(),
        [minotaur, centaur]
    );

    let wrong = RegionalSoulManifestation {
        region: FacultyRegion::AuraFields,
        ..centaur
    };
    assert_eq!(
        validate_regional_soul_manifestation(wrong),
        Err(FacultyLawError::InvalidRegionalSoulManifestation)
    );
}

#[test]
fn full_current_form_ladder_is_presynce_and_resynce_never_replaces_it() {
    assert_eq!(
        CURRENT_FORM_PRESYNCE_LADDER,
        [
            FrameId::Gremlin,
            FrameId::Goblin,
            FrameId::Ghoul,
            FrameId::Spectre,
            FrameId::Troll,
            FrameId::Ork,
            FrameId::Ogre,
            FrameId::Troglodyte,
        ]
    );
    assert_eq!(
        resynce_preserves_current_form(FrameId::Ghoul, &manifestation(HuemanFaculty::Resynce))
            .unwrap(),
        FrameId::Ghoul
    );
}

#[test]
fn we_fairy_men_and_gallows_are_distinct_resynce_jurisdictions() {
    let cultures = canonical_resynce_cultures();
    validate_resynce_cultures(&cultures).unwrap();
    let frontier = cultures
        .iter()
        .find(|entry| entry.culture == ResynceCulture::WeFairyMenAuraRidge)
        .unwrap();
    let civic = cultures
        .iter()
        .find(|entry| entry.culture == ResynceCulture::GallowsFlyntCivicRecognition)
        .unwrap();
    assert_eq!(frontier.region, FacultyRegion::AuraRidge);
    assert_eq!(
        frontier.function,
        ResynceCultureFunction::MobileCommunalRelation
    );
    assert!(!frontier.formal_flynt_state_authority);
    assert_eq!(civic.region, FacultyRegion::FlyntCivic);
    assert_eq!(
        civic.function,
        ResynceCultureFunction::ConsequentialCivicRecognition
    );
    assert!(civic.formal_flynt_state_authority);
    assert_ne!(frontier.entity, civic.entity);
    assert_eq!(WE_FAIRY_MEN_FRONTIER_ROLES.len(), 8);
    assert_eq!(GALLOWS_CIVIC_RECOGNITION_DOMAINS.len(), 7);
}

#[test]
fn faculties_remain_bounded_and_never_claim_certainty() {
    let mut precog = manifestation(HuemanFaculty::Precog);
    precog.uncertainty.guaranteed = true;
    assert_eq!(
        validate_faculty_manifestation(&precog),
        Err(FacultyLawError::InvalidUncertainty(HuemanFaculty::Precog))
    );

    let mut presynce = manifestation(HuemanFaculty::Presynce);
    presynce.evidence_requirements.clear();
    assert_eq!(
        validate_faculty_manifestation(&presynce),
        Err(FacultyLawError::MissingEvidence(HuemanFaculty::Presynce))
    );

    let mut prefog = manifestation(HuemanFaculty::Prefog);
    prefog.expression = FacultyExpression::CandidatePossibilities {
        legal_candidate_count: 1,
    };
    assert_eq!(
        validate_faculty_manifestation(&prefog),
        Err(FacultyLawError::PrefogWithoutMultiplePossibilities)
    );
}

#[test]
fn legacy_membership_never_infers_faculty_or_mastery() {
    let state = institutional_access_fixture();
    assert!(migrate_legacy_faculty_manifestations(&state.memberships).is_empty());
}
