use std::collections::{BTreeMap, BTreeSet};

use hollow_grove::hollow_grove_contract::House;
use hollow_grove::institution::IdentityId;
use hollow_grove::lineage_contract::SandmanorForm;
use hollow_grove::world::central_junction::{EconomicPole, canonical_market_indexes};
use hollow_grove::world::glaushouse::{
    ContinuanceConditions, ContinuanceHorizon, SynthesisContinuance, SynthesisLifecycle,
    SynthesisLifecycleState, SynthesisRecipeReference, SynthesisWay,
};
use hollow_grove::world::sandmanor::milestone::*;
use hollow_grove::world::sandmanor::{ContestId, EvidenceId, RecipeId};

fn identity(value: &str) -> IdentityId {
    IdentityId::new(value).unwrap()
}

fn evidence(value: &str) -> EvidenceId {
    EvidenceId::new(value).unwrap()
}

fn recipe(value: &str) -> RecipeId {
    RecipeId::new(value).unwrap()
}

fn lifecycle() -> SynthesisLifecycle {
    SynthesisLifecycle {
        history: vec![
            SynthesisLifecycleState::Established,
            SynthesisLifecycleState::Integrated,
            SynthesisLifecycleState::Maintained,
            SynthesisLifecycleState::Renewed,
        ],
    }
}

fn continuance(name: &str) -> SynthesisContinuance {
    SynthesisContinuance {
        recipe: SynthesisRecipeReference {
            name: name.into(),
            revision: "1.0.0".into(),
        },
        ways: vec![SynthesisWay {
            name: format!("{name} Ways"),
            practices: vec!["service".into(), "renewal".into()],
        }],
        maintenance: vec!["bodily discipline".into(), "regional service".into()],
        renewal: vec!["Recipe renewal".into()],
        environmental_conditions: vec!["compatible Sandmanor domain".into()],
        institutional_care: vec![hollow_grove::world::glaushouse::medical_civilization_id()],
        conditions: ContinuanceConditions {
            maintenance_current: true,
            renewal_current: true,
            recipe_practiced: true,
            environment_compatible: true,
            institutional_care_available: true,
            ways_known_and_practiced: true,
        },
        expected_continuance: ContinuanceHorizon::NaturalLifePossible,
    }
}

fn qualification(
    id: &str,
    person: &IdentityId,
    people: SandmanorPeople,
    current_form: SandmanorForm,
    target_form: SandmanorForm,
) -> GuardianQualification {
    GuardianQualification {
        id: GuardianQualificationId::new(id).unwrap(),
        person: person.clone(),
        people,
        current_form,
        target_form,
        evidence: vec![evidence(&format!("evidence.{id}"))],
        sustained_service: true,
        long_term_health_preserved: true,
        accepts_responsibility_for_failure: true,
        qualified: true,
    }
}

fn authorization(
    id: &str,
    person: &IdentityId,
    qualification: &GuardianQualification,
    recipe_id: &str,
) -> GuardianRecipeAuthorization {
    GuardianRecipeAuthorization {
        id: GuardianAuthorizationId::new(id).unwrap(),
        person: person.clone(),
        qualification: qualification.id.clone(),
        recipe: recipe(recipe_id),
        from: qualification.current_form,
        to: qualification.target_form,
        sandmanor_proof_recorded: true,
        authorized: true,
    }
}

fn synthesis(
    id: &str,
    person: &IdentityId,
    authorization: &GuardianRecipeAuthorization,
) -> GuardianSynthesisRecord {
    GuardianSynthesisRecord {
        id: GuardianSynthesisId::new(id).unwrap(),
        person: person.clone(),
        authorization: authorization.id.clone(),
        recipe: authorization.recipe.clone(),
        from: authorization.from,
        to: authorization.to,
        glaushouse_compatibility_cleared: true,
        lawful_synthesis_completed: true,
        lifecycle: lifecycle(),
        continuance: continuance(id),
    }
}

fn investiture(
    id: &str,
    person: &IdentityId,
    synthesis: &GuardianSynthesisRecord,
    mantle: GuardianMantle,
    jurisdiction: &str,
) -> GuardianInvestiture {
    GuardianInvestiture {
        id: GuardianInvestitureId::new(id).unwrap(),
        person: person.clone(),
        synthesis: synthesis.id.clone(),
        mantle,
        jurisdiction: jurisdiction.into(),
        authority_state: GuardianAuthorityState::Active,
        renewal_current: true,
    }
}

fn event(id: &str, kind: GuardianEventKind) -> GuardianEvent {
    GuardianEvent {
        id: GuardianEventId::new(id).unwrap(),
        kind,
    }
}

fn complete_guardian_paths() -> (GuardianState, IdentityId, IdentityId) {
    let minorian = identity("being.sandmanor.hecaton-candidate");
    let minoan = identity("being.sandmanor.pegasus-candidate");

    let minotaur_q = qualification(
        "qualification.minorian.minotaur",
        &minorian,
        SandmanorPeople::Minorian,
        SandmanorForm::Gnome,
        SandmanorForm::Minotaur,
    );
    let minotaur_a = authorization(
        "authorization.minorian.minotaur",
        &minorian,
        &minotaur_q,
        "recipe.sandmanor.minotaur",
    );
    let minotaur_s = synthesis("synthesis.minorian.minotaur", &minorian, &minotaur_a);
    let minotaur_i = investiture(
        "investiture.minorian.fields",
        &minorian,
        &minotaur_s,
        GuardianMantle::GuardianOfTheFields,
        "a defined Aura Farm domain",
    );
    let hecaton_q = qualification(
        "qualification.minorian.hecaton",
        &minorian,
        SandmanorPeople::Minorian,
        SandmanorForm::Minotaur,
        SandmanorForm::Hecaton,
    );
    let hecaton_a = authorization(
        "authorization.minorian.hecaton",
        &minorian,
        &hecaton_q,
        "recipe.sandmanor.hecaton",
    );
    let hecaton_s = synthesis("synthesis.minorian.hecaton", &minorian, &hecaton_a);
    let hecaton_i = investiture(
        "investiture.minorian.whole-farm",
        &minorian,
        &hecaton_s,
        GuardianMantle::GuardianOfTheWholeFarm,
        "the whole Aura Farm",
    );

    let centaur_q = qualification(
        "qualification.minoan.centaur",
        &minoan,
        SandmanorPeople::Minoan,
        SandmanorForm::Elf,
        SandmanorForm::Centaur,
    );
    let centaur_a = authorization(
        "authorization.minoan.centaur",
        &minoan,
        &centaur_q,
        "recipe.sandmanor.centaur",
    );
    let centaur_s = synthesis("synthesis.minoan.centaur", &minoan, &centaur_a);
    let centaur_i = investiture(
        "investiture.minoan.beach",
        &minoan,
        &centaur_s,
        GuardianMantle::GuardianOfTheBeach,
        "a defined stretch of Aura Beach",
    );
    let pegasus_q = qualification(
        "qualification.minoan.pegasus",
        &minoan,
        SandmanorPeople::Minoan,
        SandmanorForm::Centaur,
        SandmanorForm::Pegasus,
    );
    let pegasus_a = authorization(
        "authorization.minoan.pegasus",
        &minoan,
        &pegasus_q,
        "recipe.sandmanor.pegasus",
    );
    let pegasus_s = synthesis("synthesis.minoan.pegasus", &minoan, &pegasus_a);
    let pegasus_i = investiture(
        "investiture.minoan.horizon",
        &minoan,
        &pegasus_s,
        GuardianMantle::GuardianOfTheHorizon,
        "the connected coast and Current Sea approaches",
    );

    let events = vec![
        event(
            "event.01.minotaur-qualification",
            GuardianEventKind::QualificationRecorded(minotaur_q),
        ),
        event(
            "event.02.minotaur-authorization",
            GuardianEventKind::RecipeAuthorized(minotaur_a),
        ),
        event(
            "event.03.minotaur-synthesis",
            GuardianEventKind::SynthesisRecorded(minotaur_s),
        ),
        event(
            "event.04.fields-investiture",
            GuardianEventKind::MantleInvested(minotaur_i.clone()),
        ),
        event(
            "event.05.hecaton-qualification",
            GuardianEventKind::QualificationRecorded(hecaton_q),
        ),
        event(
            "event.06.hecaton-authorization",
            GuardianEventKind::RecipeAuthorized(hecaton_a),
        ),
        event(
            "event.07.hecaton-synthesis",
            GuardianEventKind::SynthesisRecorded(hecaton_s),
        ),
        event(
            "event.08.fields-retirement",
            GuardianEventKind::MantleRetired(minotaur_i.id),
        ),
        event(
            "event.09.whole-farm-investiture",
            GuardianEventKind::MantleInvested(hecaton_i),
        ),
        event(
            "event.10.centaur-qualification",
            GuardianEventKind::QualificationRecorded(centaur_q),
        ),
        event(
            "event.11.centaur-authorization",
            GuardianEventKind::RecipeAuthorized(centaur_a),
        ),
        event(
            "event.12.centaur-synthesis",
            GuardianEventKind::SynthesisRecorded(centaur_s),
        ),
        event(
            "event.13.beach-investiture",
            GuardianEventKind::MantleInvested(centaur_i.clone()),
        ),
        event(
            "event.14.pegasus-qualification",
            GuardianEventKind::QualificationRecorded(pegasus_q),
        ),
        event(
            "event.15.pegasus-authorization",
            GuardianEventKind::RecipeAuthorized(pegasus_a),
        ),
        event(
            "event.16.pegasus-synthesis",
            GuardianEventKind::SynthesisRecorded(pegasus_s),
        ),
        event(
            "event.17.beach-retirement",
            GuardianEventKind::MantleRetired(centaur_i.id),
        ),
        event(
            "event.18.horizon-investiture",
            GuardianEventKind::MantleInvested(pegasus_i),
        ),
    ];
    (GuardianState::replay(&events).unwrap(), minorian, minoan)
}

fn scores(values: [u16; 5]) -> BTreeMap<TrialDomain, u16> {
    TrialDomain::ALL.into_iter().zip(values).collect()
}

fn contest_fixture(reverse: bool) -> (GuardianState, ContestOfImprovementProof) {
    let (guardians, hecaton, pegasus) = complete_guardian_paths();
    let hecaton_candidate = ImprovementCandidate {
        person: hecaton.clone(),
        people: SandmanorPeople::Minorian,
        form: SandmanorForm::Hecaton,
        mantle: GuardianInvestitureId::new("investiture.minorian.whole-farm").unwrap(),
        baseline: CandidateBaseline {
            candidate: hecaton.clone(),
            competencies: scores([90, 90, 20, 20, 40]),
            weaknesses: vec!["encloses public freedom".into()],
            evidence: vec![evidence("evidence.contest.hecaton.baseline")],
        },
        improvement: ImprovementEvidence {
            candidate: hecaton.clone(),
            final_competencies: scores([90, 90, 70, 75, 80]),
            evidence: vec![evidence("evidence.contest.hecaton.final")],
            integration_demonstrated: true,
            imitation_only: false,
            degraded_other_candidate: false,
        },
        unresolved_disqualifying_corruption: false,
    };
    let pegasus_candidate = ImprovementCandidate {
        person: pegasus.clone(),
        people: SandmanorPeople::Minoan,
        form: SandmanorForm::Pegasus,
        mantle: GuardianInvestitureId::new("investiture.minoan.horizon").unwrap(),
        baseline: CandidateBaseline {
            candidate: pegasus.clone(),
            competencies: scores([20, 20, 90, 90, 40]),
            weaknesses: vec!["does not plan a future harvest".into()],
            evidence: vec![evidence("evidence.contest.pegasus.baseline")],
        },
        improvement: ImprovementEvidence {
            candidate: pegasus.clone(),
            final_competencies: scores([70, 65, 90, 90, 75]),
            evidence: vec![evidence("evidence.contest.pegasus.final")],
            integration_demonstrated: true,
            imitation_only: false,
            degraded_other_candidate: false,
        },
        unresolved_disqualifying_corruption: false,
    };
    let candidates = if reverse {
        [pegasus_candidate, hecaton_candidate]
    } else {
        [hecaton_candidate, pegasus_candidate]
    };
    let trials = [
        (TrialDomain::AuraField, pegasus.clone(), hecaton.clone()),
        (TrialDomain::ContentFarm, pegasus.clone(), hecaton.clone()),
        (
            TrialDomain::LibertyHospitality,
            hecaton.clone(),
            pegasus.clone(),
        ),
        (TrialDomain::RescueHorizon, hecaton.clone(), pegasus.clone()),
        (
            TrialDomain::ReciprocalIntegration,
            pegasus.clone(),
            hecaton.clone(),
        ),
    ]
    .into_iter()
    .enumerate()
    .map(|(index, (domain, candidate, teacher))| PublicTrial {
        id: TrialId::new(format!("trial.contest.{index}")).unwrap(),
        domain,
        candidate,
        teacher,
        evidence: vec![evidence(&format!("evidence.trial.{index}"))],
        candidate_completed_work: true,
        valid: true,
    })
    .collect();
    let hecaton_improvement = candidates
        .iter()
        .find(|candidate| candidate.person == hecaton)
        .unwrap()
        .improvement_total()
        .unwrap();
    let pegasus_improvement = candidates
        .iter()
        .find(|candidate| candidate.person == pegasus)
        .unwrap()
        .improvement_total()
        .unwrap();
    let votes = [
        ("one", hecaton.clone(), hecaton_improvement),
        ("two", hecaton.clone(), hecaton_improvement),
        ("three", hecaton.clone(), hecaton_improvement),
        ("four", pegasus.clone(), pegasus_improvement),
        ("five", pegasus.clone(), pegasus_improvement),
    ]
    .into_iter()
    .map(|(suffix, candidate, assessed_improvement)| CrowdJudgment {
        id: CrowdJudgmentId::new(format!("judgment.contest.{suffix}")).unwrap(),
        voter: identity(&format!("being.sandmanor.voter-{suffix}")),
        candidate,
        assessed_improvement,
        eligible: true,
        conflicted: false,
        coerced: false,
    })
    .collect();
    (
        guardians,
        ContestOfImprovementProof {
            id: ContestId::new("contest.sandmanor.reciprocal-improvement").unwrap(),
            candidates,
            trials,
            teaching_integrity: vec![
                TeachingIntegrityFinding {
                    teacher: hecaton,
                    sincere: true,
                    complete_enough_for_safety: true,
                    deliberately_false: false,
                    deliberately_dangerous: false,
                },
                TeachingIntegrityFinding {
                    teacher: pegasus,
                    sincere: true,
                    complete_enough_for_safety: true,
                    deliberately_false: false,
                    deliberately_dangerous: false,
                },
            ],
            judgments: votes,
        },
    )
}

#[test]
fn two_equal_peoples_have_exact_base_and_guardian_forms() {
    assert_eq!(SandmanorPeople::Minorian.base_form(), SandmanorForm::Gnome);
    assert_eq!(SandmanorPeople::Minoan.base_form(), SandmanorForm::Elf);
    assert_eq!(
        SandmanorPeople::Minorian.senior_guardian_form(),
        SandmanorForm::Hecaton
    );
    assert_eq!(
        SandmanorPeople::Minoan.senior_guardian_form(),
        SandmanorForm::Pegasus
    );
}

#[test]
fn aura_farm_has_physical_and_cultural_halves_without_blanket_corruption() {
    assert_eq!(
        AURA_FARM_HALVES,
        [
            CultivationDomain::AuraFields,
            CultivationDomain::ContentFarm
        ]
    );
    let healthy = ContentFarmAssessment {
        practices: BTreeSet::from([
            ContentFarmPractice::Educates,
            ContentFarmPractice::PreservesMemory,
            ContentFarmPractice::NourishesAttention,
        ]),
    };
    assert!(healthy.is_healthy());
    assert!(!healthy.is_exploitative());
    let exploitative = ContentFarmAssessment {
        practices: BTreeSet::from([
            ContentFarmPractice::EntertainsResponsibly,
            ContentFarmPractice::HarvestsAttentionWithoutNourishment,
        ]),
    };
    assert!(exploitative.is_exploitative());
    assert!(!exploitative.is_healthy());
}

#[test]
fn coast_order_is_canonical_and_southern_law_remains_sandmanor_jurisdiction() {
    assert_eq!(
        NORTH_TO_SOUTH_COAST,
        [
            CoastalZone::FreeAuraBeach,
            CoastalZone::SouthernCoast,
            CoastalZone::CurrentBreak,
            CoastalZone::MinoanCountyCourthouse,
            CoastalZone::GlaushouseBorder,
        ]
    );
    assert!(coast_is_progressively_regulated());
    assert_eq!(
        CoastalZone::MinoanCountyCourthouse.governing_house(),
        House::Sandmanor
    );
    assert_eq!(
        CoastalZone::GlaushouseBorder.governing_house(),
        House::Glaushouse
    );
}

#[test]
fn courthouse_transfer_preserves_legal_and_clinical_authority_boundary() {
    CourthouseTransfer {
        id: CoastalTransferId::new("transfer.courthouse.injured-person").unwrap(),
        person: identity("being.sandmanor.injured-detainee"),
        from: CoastalZone::MinoanCountyCourthouse,
        to: CoastalZone::GlaushouseBorder,
        lawful_transfer: true,
        medical_or_clinical_reason: true,
        courthouse_authority_retained_by: House::Sandmanor,
        receiving_care_authority: House::Glaushouse,
    }
    .validate()
    .unwrap();

    let catalog = hollow_grove::world::house_institutions::canonical_house_institutions();
    catalog.validate().unwrap();
    let courthouse = catalog.institution(&minoan_county_courthouse_id()).unwrap();
    assert_eq!(courthouse.house, Some(House::Sandmanor));
    assert_eq!(
        courthouse.headquarters.as_ref(),
        Some(&minoan_county_courthouse_site_id())
    );
    let beach = catalog
        .sites
        .iter()
        .find(|site| site.id.as_str() == "site.sandmanor.aura-beach")
        .unwrap();
    assert!(
        beach
            .zones
            .iter()
            .any(|zone| zone.as_str() == "zone.sandmanor.aura-beach.current-break")
    );
}

#[test]
fn current_break_hosts_manticorp_without_owning_or_duplicating_it() {
    ManticorpCurrentBreakTraining {
        id: MaritimeTrainingId::new("training.current-break.manticorp-unit").unwrap(),
        manticorp_institution: hollow_grove::world::flynt::manticorp_id(),
        flynt_authorized_unit: true,
        sandmanor_authorized_access: true,
        minoan_coastal_instruction: true,
        command_house: House::Flynt,
        territorial_house: House::Sandmanor,
        creates_second_manticorp: false,
    }
    .validate()
    .unwrap();
}

#[test]
fn complete_minorian_and_minoan_guardian_recipe_paths_replay_deterministically() {
    let (state, minorian, minoan) = complete_guardian_paths();
    state.validate().unwrap();
    assert_eq!(state.qualifications.len(), 4);
    assert_eq!(state.authorizations.len(), 4);
    assert_eq!(state.syntheses.len(), 4);
    assert_eq!(state.investitures.len(), 4);
    assert!(
        state
            .syntheses
            .values()
            .all(|record| record.form_is_physically_present())
    );
    assert!(state.investitures.values().any(|record| {
        record.person == minorian
            && record.mantle == GuardianMantle::GuardianOfTheWholeFarm
            && record.may_exercise_authority()
    }));
    assert!(state.investitures.values().any(|record| {
        record.person == minoan
            && record.mantle == GuardianMantle::GuardianOfTheHorizon
            && record.may_exercise_authority()
    }));
}

#[test]
fn guardian_progression_rejects_automatic_or_cross_lineage_upgrade() {
    let person = identity("being.sandmanor.invalid-upgrade");
    let invalid = qualification(
        "qualification.invalid.auto-hecaton",
        &person,
        SandmanorPeople::Minorian,
        SandmanorForm::Gnome,
        SandmanorForm::Hecaton,
    );
    let result = GuardianState::replay(&[event(
        "event.invalid.auto-hecaton",
        GuardianEventKind::QualificationRecorded(invalid),
    )]);
    assert!(matches!(
        result,
        Err(SandmanorMilestoneError::InvalidQualification(_))
    ));
}

#[test]
fn mantle_removal_ends_authority_without_erasing_the_real_form() {
    let (mut state, minorian, _) = complete_guardian_paths();
    let mantle_id = GuardianInvestitureId::new("investiture.minorian.whole-farm").unwrap();
    state
        .apply_event(GuardianEventKind::MantleRemoved(mantle_id.clone()))
        .unwrap();
    let mantle = &state.investitures[&mantle_id];
    assert!(!mantle.may_exercise_authority());
    assert!(state.syntheses.values().any(|record| {
        record.person == minorian
            && record.to == SandmanorForm::Hecaton
            && record.form_is_physically_present()
    }));
}

#[test]
fn guardian_mantles_are_plural_while_sandman_is_singular() {
    let (mut state, minorian, minoan) = complete_guardian_paths();
    let hecaton_synthesis = state
        .syntheses
        .values()
        .find(|record| record.person == minorian && record.to == SandmanorForm::Hecaton)
        .unwrap()
        .clone();
    let pegasus_synthesis = state
        .syntheses
        .values()
        .find(|record| record.person == minoan && record.to == SandmanorForm::Pegasus)
        .unwrap()
        .clone();
    let first = investiture(
        "investiture.sandman.first",
        &minorian,
        &hecaton_synthesis,
        GuardianMantle::SandmanSovereign,
        "Sandmanor",
    );
    let second = investiture(
        "investiture.sandman.second",
        &minoan,
        &pegasus_synthesis,
        GuardianMantle::SandmanSovereign,
        "Sandmanor",
    );
    state.investitures.insert(first.id.clone(), first);
    state.investitures.insert(second.id.clone(), second);
    assert_eq!(
        state.validate(),
        Err(SandmanorMilestoneError::MultipleActiveSandmen)
    );
}

#[test]
fn contest_uses_all_five_trials_and_is_candidate_order_independent() {
    let (guardians, contest) = contest_fixture(false);
    let (_, forward) = contest.evaluate(&guardians).unwrap();
    let (guardians, reversed) = contest_fixture(true);
    let (_, reverse) = reversed.evaluate(&guardians).unwrap();
    let expected = CrowdVerdict::Winner(identity("being.sandmanor.hecaton-candidate"));
    assert_eq!(forward, expected);
    assert_eq!(reverse, expected);
}

#[test]
fn improvement_is_relative_to_baseline_not_absolute_mastery() {
    let (_, contest) = contest_fixture(false);
    let hecaton = contest
        .candidates
        .iter()
        .find(|candidate| candidate.form == SandmanorForm::Hecaton)
        .unwrap();
    let pegasus = contest
        .candidates
        .iter()
        .find(|candidate| candidate.form == SandmanorForm::Pegasus)
        .unwrap();
    assert_eq!(hecaton.baseline.competencies[&TrialDomain::AuraField], 90);
    assert_eq!(
        hecaton.improvement.final_competencies[&TrialDomain::AuraField],
        90
    );
    assert_eq!(
        pegasus.improvement.final_competencies[&TrialDomain::AuraField],
        70
    );
    assert_eq!(pegasus.baseline.competencies[&TrialDomain::AuraField], 20);
    assert_eq!(hecaton.improvement_total().unwrap(), 145);
    assert_eq!(pegasus.improvement_total().unwrap(), 130);
}

#[test]
fn duplicate_conflicted_and_coerced_judgments_do_not_count() {
    let (guardians, mut contest) = contest_fixture(false);
    let duplicate_voter = contest.judgments[0].voter.clone();
    let assessed = contest.judgments[0].assessed_improvement;
    contest.judgments.push(CrowdJudgment {
        id: CrowdJudgmentId::new("judgment.contest.duplicate").unwrap(),
        voter: duplicate_voter,
        candidate: contest.judgments[0].candidate.clone(),
        assessed_improvement: assessed,
        eligible: true,
        conflicted: false,
        coerced: false,
    });
    contest.judgments[1].conflicted = true;
    contest.judgments[2].coerced = true;
    let (tally, _) = contest.evaluate(&guardians).unwrap();
    assert_eq!(tally.excluded_judgments.len(), 4);
}

#[test]
fn valid_tie_leaves_sandman_unresolved() {
    let (guardians, mut contest) = contest_fixture(false);
    contest.judgments.remove(2);
    let (_, verdict) = contest.evaluate(&guardians).unwrap();
    assert_eq!(verdict, CrowdVerdict::Tie);
}

#[test]
fn teaching_sabotage_voids_the_saboteur_instead_of_rewarding_rank() {
    let (guardians, mut contest) = contest_fixture(false);
    contest.teaching_integrity[0].deliberately_false = true;
    let (_, verdict) = contest.evaluate(&guardians).unwrap();
    assert_eq!(
        verdict,
        CrowdVerdict::VoidForTeachingSabotage(identity("being.sandmanor.hecaton-candidate"))
    );
}

#[test]
fn unresolved_disqualifying_corruption_blocks_contest_entry() {
    let (guardians, mut contest) = contest_fixture(false);
    contest.candidates[0].unresolved_disqualifying_corruption = true;
    assert!(matches!(
        contest.evaluate(&guardians),
        Err(SandmanorMilestoneError::DisqualifiedContestCandidate(_))
    ));
}

#[test]
fn imitation_or_degrading_the_other_candidate_is_not_improvement() {
    let (guardians, mut contest) = contest_fixture(false);
    contest.candidates[0].improvement.imitation_only = true;
    assert!(matches!(
        contest.evaluate(&guardians),
        Err(SandmanorMilestoneError::InvalidImprovementEvidence(_))
    ));
    contest.candidates[0].improvement.imitation_only = false;
    contest.candidates[0].improvement.degraded_other_candidate = true;
    assert!(matches!(
        contest.evaluate(&guardians),
        Err(SandmanorMilestoneError::InvalidImprovementEvidence(_))
    ));
}

#[test]
fn winner_receives_sandman_convergence_and_loser_keeps_form_and_mantle() {
    let (guardians, contest) = contest_fixture(false);
    let (_, verdict) = contest.evaluate(&guardians).unwrap();
    let winner = identity("being.sandmanor.hecaton-candidate");
    let loser = identity("being.sandmanor.pegasus-candidate");
    let succession = SandmanSuccession {
        id: SuccessionId::new("succession.sandmanor.current").unwrap(),
        contest: contest.id,
        winner: winner.clone(),
        crowd_verdict: verdict,
        recipe_authorized: true,
        convergence: SandmanConvergence {
            person: winner,
            source_form: SandmanorForm::Hecaton,
            recipe: recipe("recipe.sandmanor.sandman"),
            integrates_other_way: true,
            glaushouse_compatibility_cleared: true,
            lifecycle: lifecycle(),
            continuance: continuance("Sandman convergence"),
            fused_with_other_person: false,
        },
        mantle: SandmanMantleInvestiture {
            id: GuardianInvestitureId::new("investiture.sandman.current").unwrap(),
            person: identity("being.sandmanor.hecaton-candidate"),
            mantle: GuardianMantle::SandmanSovereign,
            authority_state: GuardianAuthorityState::Active,
            renewal_current: true,
        },
        historical_names: BTreeSet::from([SANDMAN_HISTORICAL_NAME.into()]),
        losing_candidate: loser.clone(),
        loser_retains_form: true,
        loser_retains_mantle: true,
    };
    validate_sandman_successions(&[succession]).unwrap();
    assert!(guardians.investitures.values().any(|record| {
        record.person == loser
            && record.mantle == GuardianMantle::GuardianOfTheHorizon
            && record.may_exercise_authority()
    }));
    assert_eq!(SANDMAN_HISTORICAL_NAME, "Aegon");
    assert_eq!(SANDMAN_COMMON_NAME, "The Sandman");
}

#[test]
fn sandmanor_design_disciplines_share_one_central_junction_index() {
    SandmanorDesignExposure {
        interior_cultivated_basis_points: 4_500,
        exterior_coastal_basis_points: 5_500,
    }
    .validate()
    .unwrap();
    let design_indexes = canonical_market_indexes()
        .into_iter()
        .filter(|index| index.pole == EconomicPole::Design)
        .collect::<Vec<_>>();
    assert_eq!(design_indexes.len(), 1);
    assert_eq!(design_indexes[0].name, "Sandmanor Design Index");
    assert!(design_indexes[0].owner.is_none());
    assert!(!design_indexes[0].currency);
}
