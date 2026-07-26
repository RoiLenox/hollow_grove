use std::collections::BTreeSet;

use hollow_grove::hollow_grove_contract::House;
use hollow_grove::institution::IdentityId;
use hollow_grove::lineage_contract::SandmanorForm;
use hollow_grove::world::{house_institutions, sandmanor};

fn identity(value: &str) -> IdentityId {
    IdentityId::new(value).unwrap()
}

fn evidence_id(value: &str) -> sandmanor::EvidenceId {
    sandmanor::EvidenceId::new(value).unwrap()
}

fn valid_registry() -> sandmanor::SandmanorRegistry {
    let minorian = identity("being.sandmanor.test-minorian");
    let minoan = identity("being.sandmanor.test-minoan");
    let method = sandmanor::MethodId::new("method.sandmanor.test").unwrap();
    let design = sandmanor::DesignId::new("design.sandmanor.test-v1").unwrap();
    let claim = sandmanor::ClaimId::new("claim.sandmanor.test").unwrap();
    let baseline_minorian = evidence_id("evidence.sandmanor.minorian-baseline");
    let final_minorian = evidence_id("evidence.sandmanor.minorian-final");
    let baseline_minoan = evidence_id("evidence.sandmanor.minoan-baseline");
    let final_minoan = evidence_id("evidence.sandmanor.minoan-final");
    let demonstration = sandmanor::DemonstrationId::new("demonstration.sandmanor.test").unwrap();
    let proof = sandmanor::ProofJudgmentId::new("proof.sandmanor.test").unwrap();
    let minorian_teaches =
        sandmanor::TeachingRecordId::new("teaching.sandmanor.minorian-to-minoan").unwrap();
    let minoan_teaches =
        sandmanor::TeachingRecordId::new("teaching.sandmanor.minoan-to-minorian").unwrap();
    let contest = sandmanor::ContestId::new("contest.sandmanor.test").unwrap();

    let evidence = [
        (
            baseline_minorian.clone(),
            minorian.clone(),
            "Minorian baseline",
        ),
        (
            final_minorian.clone(),
            minorian.clone(),
            "Minorian learned exterior practice",
        ),
        (baseline_minoan.clone(), minoan.clone(), "Minoan baseline"),
        (
            final_minoan.clone(),
            minoan.clone(),
            "Minoan learned interior practice",
        ),
    ]
    .into_iter()
    .map(|(id, collector, content)| sandmanor::EvidenceRecord {
        id,
        claim: claim.clone(),
        class: sandmanor::EvidenceClass::Experimental,
        source: "witnessed Contest session".into(),
        collector: sandmanor::DesignAuthor::Being(collector),
        collected_at: 5,
        method: method.clone(),
        custody: vec!["Sandmanor evidence archive".into()],
        alterations: Vec::new(),
        content: content.into(),
        fabricated: false,
    })
    .collect();

    sandmanor::SandmanorRegistry {
        subjects: vec![
            sandmanor::SandmanorSubjectRecord {
                id: minorian.clone(),
                intellectual_lineage: vec!["Minorian Gnome practice".into()],
                tombstoned: false,
            },
            sandmanor::SandmanorSubjectRecord {
                id: minoan.clone(),
                intellectual_lineage: vec!["Minoan Elf practice".into()],
                tombstoned: false,
            },
        ],
        methods: vec![sandmanor::MethodRecord {
            id: method.clone(),
            objective: "test reciprocal design improvement".into(),
            sequence: vec!["baseline".into(), "teach".into(), "demonstrate".into()],
            materials: vec!["shared design model".into()],
            operators: vec![sandmanor::DesignAuthor::Institution(
                sandmanor::proof_civilization_id(),
            )],
            controls: vec!["same disclosed task".into()],
            comparison_basis: "documented learning from baseline".into(),
            measurements: vec!["new competence".into(), "corrected method".into()],
            recording_procedure: "append-only witnessed record".into(),
            stopping_conditions: vec!["sabotage or coercion".into()],
            exclusion_rules: vec!["popularity alone".into()],
            uncertainty: vec!["different traditions improve differently".into()],
            deviations: Vec::new(),
            analysis: "compare integration without ranking traditions".into(),
            changed_after_results_without_disclosure: false,
        }],
        designs: vec![sandmanor::DesignRecord {
            id: design.clone(),
            author: sandmanor::DesignAuthor::Institution(sandmanor::proof_civilization_id()),
            purpose: "reciprocal whole-design exercise".into(),
            intended_users: vec!["Minorians".into(), "Minoans".into()],
            problem: "interior and exterior reasoning are disconnected".into(),
            assumptions: vec!["both traditions have equal standing".into()],
            inputs: vec!["two baseline designs".into()],
            outputs: vec!["improved integrated design".into()],
            dependencies: vec!["reciprocal teaching".into()],
            constraints: vec!["no domination metric".into()],
            method: method.clone(),
            materials: vec!["model and evidence archive".into()],
            expected_result: "documented improvement".into(),
            known_risks: vec!["staged improvement".into()],
            failure_states: vec!["neither candidate learns".into()],
            maintenance: vec!["preserve revision history".into()],
            alternatives: vec!["joint applied-design challenge".into()],
            measurement_plan: "compare each candidate to their own baseline".into(),
            version: "1.0.0".into(),
            prior_version: None,
            materially_revised: false,
            inherits_prior_proof: false,
            conflicts_of_interest: vec!["none disclosed".into()],
            status: sandmanor::DesignStatus::Validated,
        }],
        claims: vec![sandmanor::ClaimRecord {
            id: claim.clone(),
            design,
            statement: "reciprocal teaching improves whole-design judgment".into(),
            scope: "this Contest and disclosed conditions".into(),
            requires_independent_reproduction: false,
            active: true,
            tombstoned: false,
        }],
        evidence,
        demonstrations: vec![sandmanor::DemonstrationRecord {
            id: demonstration.clone(),
            claim,
            method,
            environment: "public Sandmanor design floor".into(),
            operator: sandmanor::DesignAuthor::Institution(sandmanor::proof_civilization_id()),
            observers: vec![
                sandmanor::DesignAuthor::Being(minorian.clone()),
                sandmanor::DesignAuthor::Being(minoan.clone()),
            ],
            inputs: vec!["baseline work".into()],
            outputs: vec!["final work".into()],
            deviations: Vec::new(),
            measurements: vec!["documented improvement".into()],
            actual_result: "both candidates learned; Minorian integrated more".into(),
            complete_result_recorded: true,
            prototype: false,
            represented_as_completed_production: false,
            simulated: false,
            represented_as_direct_physical_performance: false,
            failure: None,
        }],
        proofs: vec![sandmanor::ProofJudgmentRecord {
            id: proof.clone(),
            claim: sandmanor::ClaimId::new("claim.sandmanor.test").unwrap(),
            scope: "this demonstrated Contest design".into(),
            status: sandmanor::ProofStatus::ProvenWithinScope,
            evidence: vec![
                baseline_minorian.clone(),
                final_minorian.clone(),
                baseline_minoan.clone(),
                final_minoan.clone(),
            ],
            demonstrations: vec![demonstration],
            reproductions: Vec::new(),
            criticism_considered: Vec::new(),
            issued_by: sandmanor::sandman_office_id(),
            active: true,
            emergency_expires_at: None,
            entered_ordinary_review: false,
            grants_title: false,
            grants_clearance: false,
            grants_recognition: false,
            prefig_source: None,
        }],
        teaching: vec![
            sandmanor::TeachingRecord {
                id: minorian_teaches.clone(),
                teacher: minorian.clone(),
                learner: minoan.clone(),
                teacher_tradition: sandmanor::CivicTradition::Minorian,
                learner_tradition: sandmanor::CivicTradition::Minoan,
                practice: "maintain a repeated interior system".into(),
                design_principle: "improve what must continue working".into(),
                method: "inspect from within".into(),
                observation: "maintenance burden".into(),
                criticism: "appearance concealed repair cost".into(),
                baseline_evidence: vec![baseline_minoan.clone()],
                final_evidence: vec![final_minoan.clone()],
                genuine: true,
                comprehensible: true,
                relevant: true,
                non_sabotaging: true,
            },
            sandmanor::TeachingRecord {
                id: minoan_teaches.clone(),
                teacher: minoan.clone(),
                learner: minorian.clone(),
                teacher_tradition: sandmanor::CivicTradition::Minoan,
                learner_tradition: sandmanor::CivicTradition::Minorian,
                practice: "orient an exterior approach".into(),
                design_principle: "discover what may become reachable".into(),
                method: "inspect from the horizon inward".into(),
                observation: "newcomer route legibility".into(),
                criticism: "interior efficiency concealed public exclusion".into(),
                baseline_evidence: vec![baseline_minorian.clone()],
                final_evidence: vec![final_minorian.clone()],
                genuine: true,
                comprehensible: true,
                relevant: true,
                non_sabotaging: true,
            },
        ],
        contests: vec![sandmanor::ContestOfImprovementRecord {
            id: contest.clone(),
            candidates: [
                sandmanor::ContestCandidate {
                    being: minorian.clone(),
                    tradition: sandmanor::CivicTradition::Minorian,
                    baseline_evidence: vec![baseline_minorian],
                    final_evidence: vec![final_minorian],
                    teaching_experience_demonstrated: true,
                    willingness_to_learn_demonstrated: true,
                    unresolved_fraudulent_design: false,
                },
                sandmanor::ContestCandidate {
                    being: minoan,
                    tradition: sandmanor::CivicTradition::Minoan,
                    baseline_evidence: vec![baseline_minoan],
                    final_evidence: vec![final_minoan],
                    teaching_experience_demonstrated: true,
                    willingness_to_learn_demonstrated: true,
                    unresolved_fraudulent_design: false,
                },
            ],
            teaching: vec![minorian_teaches, minoan_teaches],
            review_body: sandmanor::ContestReviewBody {
                minorian_reviewers: true,
                minoan_reviewers: true,
                teaching_representatives: true,
                ordinary_affected_citizens: true,
                evidence_stewards: true,
                conflict_reviewers: true,
            },
            audience_received_baselines: true,
            audience_received_process: true,
            audience_received_results: true,
            audience_could_question_candidates: true,
            conflicts_disclosed: true,
            outcome: sandmanor::ContestOutcome::Winner,
            winner: Some(minorian.clone()),
            joint_applied_design_challenge_completed: false,
            challenges_resolved: true,
            fraudulent: false,
            complete: true,
        }],
        accessions: vec![sandmanor::SandmanAccessionRecord {
            id: sandmanor::AccessionRecordId::new("accession.sandmanor.test").unwrap(),
            holder: minorian,
            contest,
            origin: sandmanor::SandmanAuthorityOrigin::ContestOfImprovement,
            stonebend_title_recorded: true,
            flynt_recognition_recorded: true,
            public_learning_statement: "I learned to make interior work meet the world.".into(),
            sealed: true,
            active: true,
            tombstoned: false,
        }],
        regional_proofs: vec![sandmanor::RegionalSynthesisProofRecord {
            id: sandmanor::RegionalProofId::new("regional-proof.sandmanor.gnome-minotaur").unwrap(),
            predecessor: SandmanorForm::Gnome,
            result: SandmanorForm::Minotaur,
            domain: sandmanor::RegionalProofDomain::AuraFields,
            function: sandmanor::RegionalFunction::AdvancedTendingAndFieldLabor,
            proof,
            grants_synthesis_clearance: false,
            grants_title_or_office: false,
        }],
        ..sandmanor::SandmanorRegistry::default()
    }
}

#[test]
fn canonical_institution_office_traditions_and_sites_are_exact() {
    sandmanor::validate_civic_traditions().unwrap();
    let catalog = house_institutions::canonical_house_institutions();
    catalog.validate().unwrap();
    let institutions = catalog
        .institutions
        .iter()
        .filter(|entry| entry.house == Some(House::Sandmanor))
        .map(|entry| entry.id.as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        institutions,
        BTreeSet::from([
            "institution.sandmanor.minoan-county-courthouse",
            "institution.sandmanor.sandmen",
        ])
    );
    let office = catalog
        .offices
        .iter()
        .find(|entry| entry.id == sandmanor::sandman_office_id())
        .unwrap();
    assert!(office.singular);
    assert!(
        office
            .authority
            .iter()
            .any(|value| value == "WitnessedImprovement")
    );
    assert!(
        office
            .authority
            .iter()
            .any(|value| value == "ProofDetermination")
    );
    assert!(
        !office
            .authority
            .iter()
            .any(|value| value == "CrowdRecognition")
    );
    assert_eq!(
        catalog
            .roles
            .iter()
            .filter(|entry| entry.id.as_str().starts_with("role.sandmanor."))
            .count(),
        2
    );
    assert_eq!(
        catalog
            .sites
            .iter()
            .filter(|entry| entry.house == House::Sandmanor)
            .count(),
        3
    );
    assert!(
        catalog
            .institution(&sandmanor::milestone::minoan_county_courthouse_id())
            .is_some()
    );
}

#[test]
fn complete_proof_contest_accession_and_regional_record_validate() {
    valid_registry().validate().unwrap();
}

#[test]
fn contest_requires_reciprocal_teaching_baselines_review_and_real_winner() {
    let mut registry = valid_registry();
    registry.contests[0].teaching.pop();
    assert!(matches!(
        registry.validate(),
        Err(sandmanor::SandmanorValidationError::ContestWithoutReciprocalTeaching(_))
    ));

    let mut registry = valid_registry();
    registry.contests[0].review_body.minoan_reviewers = false;
    assert!(matches!(
        registry.validate(),
        Err(sandmanor::SandmanorValidationError::InvalidContestReview(_))
    ));

    let mut registry = valid_registry();
    registry.contests[0].winner = Some(identity("being.sandmanor.not-a-candidate"));
    assert!(matches!(
        registry.validate(),
        Err(sandmanor::SandmanorValidationError::ContestWinnerMismatch(
            _
        ))
    ));
}

#[test]
fn office_never_arises_from_heredity_transformation_popularity_or_legacy_state() {
    for origin in [
        sandmanor::SandmanAuthorityOrigin::Heredity,
        sandmanor::SandmanAuthorityOrigin::Combat,
        sandmanor::SandmanAuthorityOrigin::Popularity,
        sandmanor::SandmanAuthorityOrigin::Transformation,
        sandmanor::SandmanAuthorityOrigin::RecognitionAlone,
        sandmanor::SandmanAuthorityOrigin::LegacyProgression,
    ] {
        let mut registry = valid_registry();
        registry.accessions[0].origin = origin;
        assert!(matches!(
            registry.validate(),
            Err(sandmanor::SandmanorValidationError::InvalidSandmanAccession(_))
        ));
    }
    let mut registry = valid_registry();
    registry.accessions.push(registry.accessions[0].clone());
    registry.accessions[1].id =
        sandmanor::AccessionRecordId::new("accession.sandmanor.second").unwrap();
    assert!(matches!(
        registry.validate(),
        Err(sandmanor::SandmanorValidationError::ActiveSandmanCount(2))
    ));
}

#[test]
fn proof_never_substitutes_for_title_clearance_or_recognition() {
    for field in 0..3 {
        let mut registry = valid_registry();
        match field {
            0 => registry.proofs[0].grants_title = true,
            1 => registry.proofs[0].grants_clearance = true,
            _ => registry.proofs[0].grants_recognition = true,
        }
        assert!(matches!(
            registry.validate(),
            Err(sandmanor::SandmanorValidationError::ProofSubstitutesOtherHouse(_))
        ));
    }
}

#[test]
fn prefig_advances_only_when_attached_to_the_existing_proof_lifecycle() {
    let mut registry = valid_registry();
    let source_evidence = registry.proofs[0].evidence[0].clone();
    registry.proofs[0].prefig_source = Some(sandmanor::PrefigProofSourceRecord {
        recipe_id: "recipe.sandmanor.prefig-demonstration".into(),
        status: hollow_grove::world::hueman_faculties::PrefigEmbodimentStatus::Demonstrable,
        evidence: vec![source_evidence.clone()],
    });
    registry.validate().unwrap();
    assert!(registry.proofs[0].advances_prefig_through_existing_proof());

    registry.proofs[0]
        .evidence
        .retain(|evidence| evidence != &source_evidence);
    assert!(matches!(
        registry.validate(),
        Err(sandmanor::SandmanorValidationError::InvalidPrefigProofSource(_))
    ));
}

#[test]
fn material_revision_prototype_simulation_and_failure_history_fail_closed() {
    let mut registry = valid_registry();
    registry.designs[0].materially_revised = true;
    registry.designs[0].inherits_prior_proof = true;
    assert!(matches!(
        registry.validate(),
        Err(sandmanor::SandmanorValidationError::BreakingRevisionInheritedProof(_))
    ));

    let mut registry = valid_registry();
    registry.demonstrations[0].prototype = true;
    registry.demonstrations[0].represented_as_completed_production = true;
    assert!(matches!(
        registry.validate(),
        Err(sandmanor::SandmanorValidationError::PrototypeAsProduction(
            _
        ))
    ));

    let mut registry = valid_registry();
    registry.demonstrations[0].simulated = true;
    registry.demonstrations[0].represented_as_direct_physical_performance = true;
    assert!(matches!(
        registry.validate(),
        Err(sandmanor::SandmanorValidationError::SimulationAsPhysical(_))
    ));
}

#[test]
fn required_reproduction_is_independent() {
    let mut registry = valid_registry();
    registry.claims[0].requires_independent_reproduction = true;
    assert!(matches!(
        registry.validate(),
        Err(sandmanor::SandmanorValidationError::ProofWithoutReproduction(_))
    ));

    let id = sandmanor::ReproductionId::new("reproduction.sandmanor.test").unwrap();
    let evidence = registry.proofs[0].evidence[0].clone();
    registry.reproductions.push(sandmanor::ReproductionRecord {
        id: id.clone(),
        claim: registry.claims[0].id.clone(),
        original_design_body: sandmanor::DesignAuthor::Institution(
            sandmanor::proof_civilization_id(),
        ),
        reproducing_body: sandmanor::DesignAuthor::Institution(sandmanor::proof_civilization_id()),
        evidence: vec![evidence],
        independent: true,
        result: "reproduced".into(),
        successful: true,
    });
    registry.proofs[0].reproductions.push(id);
    assert!(matches!(
        registry.validate(),
        Err(sandmanor::SandmanorValidationError::ReproductionNotIndependent(_))
    ));
}

#[test]
fn regional_proof_preserves_both_exact_syntheses_without_granting_other_authority() {
    let mut registry = valid_registry();
    registry
        .regional_proofs
        .push(sandmanor::RegionalSynthesisProofRecord {
            id: sandmanor::RegionalProofId::new("regional-proof.sandmanor.elf-centaur").unwrap(),
            predecessor: SandmanorForm::Elf,
            result: SandmanorForm::Centaur,
            domain: sandmanor::RegionalProofDomain::AuraBeachAndCurrentSea,
            function: sandmanor::RegionalFunction::RoamAuraBeachAndGuardCurrentSea,
            proof: registry.proofs[0].id.clone(),
            grants_synthesis_clearance: false,
            grants_title_or_office: false,
        });
    registry.validate().unwrap();

    registry.regional_proofs[1].domain = sandmanor::RegionalProofDomain::AuraFields;
    assert!(matches!(
        registry.validate(),
        Err(sandmanor::SandmanorValidationError::InvalidRegionalProof(_))
    ));
}
