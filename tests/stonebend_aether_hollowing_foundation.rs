use std::collections::BTreeSet;

use hollow_grove::HollowingRefinement;
use hollow_grove::institution::IdentityId;
use hollow_grove::world::geography::ConstitutionalRouteId;
use hollow_grove::world::stonebend::foundation::{
    AURA_WAY_STAGE_ORDER, AdvancementLandmark, AdvancementRouteKind, AuraWayPath, AuraWayStage,
    AuraWayStageEvidence, BurdenState, CurrentBatch, FractionRole, HollowingAuthorization,
    HollowingError, HollowingFraction, HollowingOperationOutcome, HollowingRequest, MediumState,
    PhysicalManifestation, ProofEvidence, ProofStatus, RefractiveProperty, SANDMANOR_TO_STONEBEND,
    StoneBehavior, StoneFormationContext, VerticalLandmark, VerticalPole, diamond_profile,
    hollow_current, opal_profile, quartz_profile, recognize_aura_way_completion, refract_aether,
    validate_foundation,
};
use hollow_grove::world::stonebend::{DecisionRecordId, EvidenceRecordId, SealRecordId};

fn identity(value: &str) -> IdentityId {
    IdentityId::new(value).expect("stable fixture identity")
}

fn evidence(value: &str) -> EvidenceRecordId {
    EvidenceRecordId::new(value).expect("stable fixture evidence")
}

fn decision(value: &str) -> DecisionRecordId {
    DecisionRecordId::new(value).expect("stable fixture decision")
}

fn seal(value: &str) -> SealRecordId {
    SealRecordId::new(value).expect("stable fixture seal")
}

fn aura_way_path(reverse: bool, complete: bool) -> AuraWayPath {
    let mut stages = AURA_WAY_STAGE_ORDER
        .iter()
        .copied()
        .filter(|stage| complete || *stage != AuraWayStage::Examination)
        .map(|stage| AuraWayStageEvidence {
            stage,
            evidence: evidence(&format!(
                "evidence.stonebend.aura-way.{}",
                stage.semantic_order()
            )),
            supervising_authority: identity("being.stonebend.fixture-supervisor"),
        })
        .collect::<Vec<_>>();
    if reverse {
        stages.reverse();
    }
    AuraWayPath {
        id: identity("path.stonebend.fixture-profession"),
        candidate: identity("being.stonebend.fixture-candidate"),
        profession: "material surveyor".into(),
        route_kind: AdvancementRouteKind::StandardAuraWay,
        stage_evidence: stages,
    }
}

fn fraction(id: &str, role: FractionRole, quantity: u64) -> HollowingFraction {
    HollowingFraction {
        id: identity(id),
        role,
        quantity,
        description: id.replace('.', " "),
    }
}

fn current_batch(reverse: bool) -> CurrentBatch {
    let mut fractions = vec![
        fraction(
            "fraction.current.fixture-essential",
            FractionRole::Essential,
            60,
        ),
        fraction("fraction.current.fixture-inert", FractionRole::Inert, 25),
        fraction(
            "fraction.current.fixture-contaminant",
            FractionRole::Contaminant,
            15,
        ),
    ];
    if reverse {
        fractions.reverse();
    }
    CurrentBatch {
        id: identity("batch.current.fixture-one"),
        medium_lineage: identity("medium.aether-current.fixture-one"),
        source: identity("source.current.fixture-well"),
        quantity: 100,
        burden: BurdenState::Heavy,
        fractions,
        lawful_custodian: identity("being.stonebend.fixture-custodian"),
        extraction_evidence: vec![evidence("evidence.current.fixture-extraction")],
    }
}

fn authorization(active: bool) -> HollowingAuthorization {
    HollowingAuthorization {
        authority: decision("decision.stonebend.fixture-hollowing"),
        source_current: identity("batch.current.fixture-one"),
        operator: identity("being.stonebend.fixture-hollower"),
        allowed_removed_fractions: BTreeSet::from([
            identity("fraction.current.fixture-inert"),
            identity("fraction.current.fixture-contaminant"),
        ]),
        minimum_preserved_quantity: 60,
        evidence: vec![evidence("evidence.stonebend.fixture-authorization")],
        active,
    }
}

fn request(removals: Vec<&str>) -> HollowingRequest {
    HollowingRequest {
        process: identity("process.stonebend.fixture-hollowing"),
        result_aether: identity("batch.aether.fixture-one"),
        declared_source_current: identity("batch.current.fixture-one"),
        refinement: HollowingRefinement::MaterialLightening,
        requested_removals: removals.into_iter().map(identity).collect(),
        outcome: HollowingOperationOutcome::Completed,
        proof: ProofEvidence {
            measurement: evidence("evidence.aether.fixture-measurement"),
            process: evidence("evidence.aether.fixture-process"),
            status: ProofStatus::Recognized,
        },
        seal: seal("seal.stonebend.fixture-aether"),
    }
}

fn formation(
    geography: &str,
    pressure: &str,
    heat: &str,
    impurities: &str,
) -> StoneFormationContext {
    StoneFormationContext {
        geography: identity(geography),
        pressure: pressure.into(),
        heat: heat.into(),
        impurities: impurities.into(),
        environmental_history: format!("{geography} recorded formation history"),
    }
}

fn recognized_aether(reverse: bool) -> hollow_grove::world::stonebend::foundation::AetherBatch {
    hollow_current(
        &current_batch(reverse),
        &authorization(true),
        &request(vec![
            "fraction.current.fixture-inert",
            "fraction.current.fixture-contaminant",
        ]),
    )
    .expect("lawful fixture Hollowing")
}

#[test]
fn vertical_law_maps_landmarks_poles_and_physical_manifestations() {
    validate_foundation().expect("canonical foundation");
    assert_eq!(VerticalPole::Aether.landmark(), VerticalLandmark::MtAura);
    assert_eq!(VerticalPole::Bathos.landmark(), VerticalLandmark::Riptide);
    assert_eq!(
        VerticalPole::Aether.physical_manifestation(),
        PhysicalManifestation::Aura
    );
    assert_eq!(
        VerticalPole::Bathos.physical_manifestation(),
        PhysicalManifestation::Current
    );
    assert_eq!(
        VerticalLandmark::MtAura.route(),
        ConstitutionalRouteId::MntAura
    );
    assert_eq!(
        VerticalLandmark::Riptide.route(),
        ConstitutionalRouteId::Riptide
    );
}

#[test]
fn neither_vertical_pole_is_owned_by_stonebend_or_becomes_a_house() {
    assert_eq!(VerticalLandmark::MtAura.constitutional_owner(), None);
    assert_eq!(VerticalLandmark::Riptide.constitutional_owner(), None);
}

#[test]
fn sandmanor_to_stonebend_route_has_explicit_semantic_order() {
    assert_eq!(
        SANDMANOR_TO_STONEBEND,
        [
            AdvancementLandmark::Sandmanor,
            AdvancementLandmark::AuraWay,
            AdvancementLandmark::MtAura,
            AdvancementLandmark::StonebendGate,
            AdvancementLandmark::Stonebend,
        ]
    );
    assert!(
        SANDMANOR_TO_STONEBEND
            .windows(2)
            .all(|pair| { pair[0].semantic_order() < pair[1].semantic_order() })
    );
}

#[test]
fn aether_and_current_are_distinct_states_of_one_lineage() {
    let current = current_batch(false);
    let aether = recognized_aether(false);
    assert_ne!(MediumState::Current, aether.state);
    assert_eq!(aether.state, MediumState::Aether);
    assert_eq!(current.medium_lineage, aether.medium_lineage);
    assert_eq!(current.id, aether.source_current);
    assert!(MediumState::Current.accepts_burden(BurdenState::Heavy));
    assert!(MediumState::Aether.accepts_burden(BurdenState::Refined));
    assert!(!MediumState::Aether.accepts_burden(BurdenState::Heavy));
}

#[test]
fn aura_way_is_the_explicit_standard_route() {
    let path = aura_way_path(false, true);
    path.validate().expect("complete standard Aura Way");
    assert!(path.is_recognition_eligible());
    assert_eq!(path.route_kind, AdvancementRouteKind::StandardAuraWay);

    let mut exceptional = path;
    exceptional.route_kind = AdvancementRouteKind::ExceptionalAlternative;
    assert!(!exceptional.is_recognition_eligible());
}

#[test]
fn aura_way_semantic_order_ignores_record_insertion_order() {
    let forward = aura_way_path(false, true);
    let reverse = aura_way_path(true, true);
    let forward_stages = forward
        .semantic_evidence()
        .into_iter()
        .map(|record| record.stage)
        .collect::<Vec<_>>();
    let reverse_stages = reverse
        .semantic_evidence()
        .into_iter()
        .map(|record| record.stage)
        .collect::<Vec<_>>();
    assert_eq!(forward_stages, AURA_WAY_STAGE_ORDER);
    assert_eq!(forward_stages, reverse_stages);
}

#[test]
fn a_missing_aura_way_stage_prevents_recognition() {
    let path = aura_way_path(false, false);
    assert!(!path.is_recognition_eligible());
    assert!(
        recognize_aura_way_completion(
            &path,
            identity("recognition.stonebend.fixture-incomplete"),
            seal("seal.stonebend.fixture-incomplete"),
        )
        .is_err()
    );
}

#[test]
fn stonebend_recognition_never_declares_metaphysical_perfection() {
    let path = aura_way_path(true, true);
    let recognition = recognize_aura_way_completion(
        &path,
        identity("recognition.stonebend.fixture-complete"),
        seal("seal.stonebend.fixture-complete"),
    )
    .expect("lawful recognition");
    assert_eq!(recognition.candidate, path.candidate);
    assert!(!recognition.declares_metaphysical_perfection);
}

#[test]
fn lawful_hollowing_preserves_essential_fraction_provenance_and_seal() {
    let source = current_batch(false);
    let aether = recognized_aether(false);
    assert!(aether.is_recognized());
    assert_eq!(aether.id, identity("batch.aether.fixture-one"));
    assert_eq!(aether.source_current, source.id);
    assert_eq!(aether.provenance.source_current, source.id);
    assert_eq!(aether.provenance.medium_lineage, source.medium_lineage);
    assert_eq!(aether.seal, seal("seal.stonebend.fixture-aether"));
    assert!(aether.preserved_fractions.iter().any(|fraction| {
        fraction.id == identity("fraction.current.fixture-essential")
            && fraction.role == FractionRole::Essential
    }));
}

#[test]
fn hollowing_result_is_independent_of_fraction_insertion_order() {
    let forward = recognized_aether(false);
    let reverse = recognized_aether(true);
    assert_eq!(forward, reverse);
}

#[test]
fn unauthorized_hollowing_is_illegal_and_does_not_replace_source() {
    let source = current_batch(false);
    let source_snapshot = source.clone();
    let error = hollow_current(
        &source,
        &authorization(false),
        &request(vec!["fraction.current.fixture-inert"]),
    )
    .expect_err("inactive authorization must fail");
    assert_eq!(error, HollowingError::UnauthorizedExtraction);
    assert!(error.is_illegal_hollowing());
    assert_eq!(source, source_snapshot);
}

#[test]
fn essential_fraction_removal_is_illegal() {
    let source = current_batch(false);
    let mut authorization = authorization(true);
    authorization
        .allowed_removed_fractions
        .insert(identity("fraction.current.fixture-essential"));
    let error = hollow_current(
        &source,
        &authorization,
        &request(vec!["fraction.current.fixture-essential"]),
    )
    .expect_err("the whole-protecting fraction cannot be removed");
    assert!(matches!(error, HollowingError::EssentialFractionRemoval(_)));
    assert!(error.is_illegal_hollowing());
}

#[test]
fn falsified_provenance_is_illegal() {
    let source = current_batch(false);
    let mut false_request = request(vec!["fraction.current.fixture-inert"]);
    false_request.declared_source_current = identity("batch.current.falsified-source");
    let error = hollow_current(&source, &authorization(true), &false_request)
        .expect_err("false provenance must fail");
    assert_eq!(error, HollowingError::FalsifiedProvenance);
    assert!(error.is_illegal_hollowing());
}

#[test]
fn current_refinement_reuses_the_existing_material_lightening_abstraction() {
    let source = current_batch(false);
    let mut wrong_refinement = request(vec!["fraction.current.fixture-inert"]);
    wrong_refinement.refinement = HollowingRefinement::Precision;
    let error = hollow_current(&source, &authorization(true), &wrong_refinement)
        .expect_err("being-oriented precision is not Current batch refinement");
    assert_eq!(error, HollowingError::UnsupportedMaterialRefinement);
    assert!(!error.is_illegal_hollowing());
}

#[test]
fn exceeded_scope_is_illegal() {
    let source = current_batch(false);
    let mut narrow = authorization(true);
    narrow
        .allowed_removed_fractions
        .remove(&identity("fraction.current.fixture-contaminant"));
    let error = hollow_current(
        &source,
        &narrow,
        &request(vec!["fraction.current.fixture-contaminant"]),
    )
    .expect_err("scope must bind removal");
    assert!(matches!(error, HollowingError::ExceedsAuthorizedScope(_)));
}

#[test]
fn over_hollowing_is_distinct_from_successful_refinement() {
    let source = current_batch(false);
    let mut protective = authorization(true);
    protective.minimum_preserved_quantity = 90;
    let error = hollow_current(
        &source,
        &protective,
        &request(vec!["fraction.current.fixture-inert"]),
    )
    .expect_err("removal beyond the preserved whole must fail");
    assert_eq!(error, HollowingError::OverHollowing);
    assert!(error.is_illegal_hollowing());
    assert!(recognized_aether(false).is_recognized());
}

#[test]
fn lawful_process_failure_is_not_fraud_or_illegal_hollowing() {
    let source = current_batch(false);
    let mut failed = request(vec!["fraction.current.fixture-inert"]);
    failed.outcome = HollowingOperationOutcome::FailedWithoutMisconduct;
    let error = hollow_current(&source, &authorization(true), &failed)
        .expect_err("declared operational failure has no output");
    assert_eq!(error, HollowingError::LawfulProcessFailure);
    assert!(!error.is_illegal_hollowing());
}

#[test]
fn proof_preserves_measurement_and_evidentiary_meanings() {
    let aether = recognized_aether(false);
    assert_eq!(
        aether.proof.measurement,
        evidence("evidence.aether.fixture-measurement")
    );
    assert_eq!(
        aether.proof.process,
        evidence("evidence.aether.fixture-process")
    );
    assert_eq!(aether.proof.status, ProofStatus::Recognized);
}

#[test]
fn same_aether_refracts_into_distinct_aura_behaviors() {
    let aether = recognized_aether(false);
    let highland = formation(
        "geography.stone.fixture-highland",
        "high",
        "cool",
        "mixed-water",
    );
    let opal = opal_profile(identity("stone.profile.fixture-opal"), highland.clone());
    let diamond = diamond_profile(identity("stone.profile.fixture-diamond"), highland.clone());
    let quartz = quartz_profile(identity("stone.profile.fixture-quartz"), highland);
    let opal_aura = refract_aether(&aether, &opal, identity("aura.manifestation.fixture-opal"))
        .expect("opal refraction");
    let diamond_aura = refract_aether(
        &aether,
        &diamond,
        identity("aura.manifestation.fixture-diamond"),
    )
    .expect("diamond refraction");
    let quartz_aura = refract_aether(
        &aether,
        &quartz,
        identity("aura.manifestation.fixture-quartz"),
    )
    .expect("quartz refraction");

    for aura in [&opal_aura, &diamond_aura, &quartz_aura] {
        assert_eq!(aura.source_aether, aether.id);
        assert_eq!(aura.source_current, aether.source_current);
        assert_eq!(aura.medium_lineage, aether.medium_lineage);
        assert_eq!(aura.physical_state, PhysicalManifestation::Aura);
    }
    assert_eq!(opal_aura.behavior, StoneBehavior::Variable);
    assert_eq!(diamond_aura.behavior, StoneBehavior::Concentrating);
    assert_eq!(quartz_aura.behavior, StoneBehavior::Resonant);
    assert_ne!(opal_aura.properties, diamond_aura.properties);
    assert_ne!(diamond_aura.properties, quartz_aura.properties);
}

#[test]
fn geographic_formation_context_changes_stone_profile_and_aura() {
    let aether = recognized_aether(false);
    let coastal = opal_profile(
        identity("stone.profile.fixture-coastal-opal"),
        formation(
            "geography.stone.fixture-coast",
            "tidal",
            "warm",
            "salt-water",
        ),
    );
    let alpine = quartz_profile(
        identity("stone.profile.fixture-alpine-quartz"),
        formation(
            "geography.stone.fixture-alpine",
            "compressive",
            "cold",
            "iron-trace",
        ),
    );
    let coastal_aura = refract_aether(
        &aether,
        &coastal,
        identity("aura.manifestation.fixture-coastal"),
    )
    .expect("coastal refraction");
    let alpine_aura = refract_aether(
        &aether,
        &alpine,
        identity("aura.manifestation.fixture-alpine"),
    )
    .expect("alpine refraction");

    assert_ne!(coastal.formation.geography, alpine.formation.geography);
    assert_ne!(coastal.id, alpine.id);
    assert_ne!(coastal_aura.behavior, alpine_aura.behavior);
    assert_eq!(coastal_aura.source_aether, alpine_aura.source_aether);
}

#[test]
fn foundational_stones_have_behavior_beyond_color_and_no_house_lock() {
    let context = formation(
        "geography.stone.fixture-neutral",
        "layered",
        "temperate",
        "mixed",
    );
    let opal = opal_profile(identity("stone.profile.fixture-opal"), context.clone());
    let diamond = diamond_profile(identity("stone.profile.fixture-diamond"), context.clone());
    let quartz = quartz_profile(identity("stone.profile.fixture-quartz"), context);
    assert!(opal.properties.contains(&RefractiveProperty::MixedBands));
    assert!(
        diamond
            .properties
            .contains(&RefractiveProperty::Concentration)
    );
    assert!(quartz.properties.contains(&RefractiveProperty::Resonance));
    for stone in [&opal, &diamond, &quartz] {
        assert_eq!(stone.final_house_assignment(), None);
        assert!(!stone.requires_melting_for_aura());
    }
}

#[test]
fn aether_is_not_automatically_aura_and_stone_cannot_create_it() {
    let aether = recognized_aether(false);
    assert_eq!(aether.state, MediumState::Aether);
    let stone = quartz_profile(
        identity("stone.profile.fixture-quartz"),
        formation(
            "geography.stone.fixture-neutral",
            "layered",
            "temperate",
            "mixed",
        ),
    );
    assert_ne!(
        aether.state,
        MediumState::Current,
        "refinement changed the burden state"
    );
    let aura = refract_aether(&aether, &stone, identity("aura.manifestation.fixture"))
        .expect("stone reveals supplied Aether");
    assert_eq!(aura.source_aether, aether.id);
}
