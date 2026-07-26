use std::collections::BTreeSet;

use hollow_grove::hollow_grove_contract::House;
use hollow_grove::institution::IdentityId;
use hollow_grove::world::{glaushouse, house_institutions};

fn identity(value: &str) -> IdentityId {
    IdentityId::new(value).unwrap()
}

fn rank_slug(rank: glaushouse::ClinicalRank) -> &'static str {
    match rank {
        glaushouse::ClinicalRank::Nightingale => "nightingale",
        glaushouse::ClinicalRank::Matron => "matron",
        glaushouse::ClinicalRank::Marshal => "marshal",
        glaushouse::ClinicalRank::Persephone => "persephone",
    }
}

fn rank_evidence(rank: glaushouse::ClinicalRank) -> BTreeSet<glaushouse::AdvancementEvidenceKind> {
    use glaushouse::AdvancementEvidenceKind as Evidence;
    match rank {
        glaushouse::ClinicalRank::Nightingale => BTreeSet::from([Evidence::NightingaleCare]),
        glaushouse::ClinicalRank::Matron => BTreeSet::from([
            Evidence::NightingaleCare,
            Evidence::AuraSensitiveObservation,
            Evidence::IdentityContinuity,
            Evidence::MaintenanceAndRenewal,
            Evidence::RecoveryAndRehabilitation,
        ]),
        glaushouse::ClinicalRank::Marshal => BTreeSet::from([
            Evidence::NightingaleCare,
            Evidence::CurrentSensitiveStabilization,
            Evidence::BodilyContinuity,
            Evidence::MaintenanceAndRenewal,
            Evidence::RecoveryAndRehabilitation,
        ]),
        glaushouse::ClinicalRank::Persephone => BTreeSet::from([
            Evidence::NightingaleCare,
            Evidence::AuraSensitiveObservation,
            Evidence::CurrentSensitiveStabilization,
            Evidence::IdentityContinuity,
            Evidence::BodilyContinuity,
            Evidence::MaintenanceAndRenewal,
            Evidence::RecoveryAndRehabilitation,
            Evidence::SafeRegression,
            Evidence::DischargeJudgment,
            Evidence::ReconciledEvidence,
        ]),
    }
}

fn standing(
    clinician: IdentityId,
    token_prefix: &str,
    ranks: &[glaushouse::ClinicalRank],
) -> (
    glaushouse::ClinicalStanding,
    Vec<glaushouse::AdvancementToken>,
) {
    let tokens = ranks
        .iter()
        .copied()
        .map(|rank| glaushouse::AdvancementToken {
            id: glaushouse::AdvancementTokenId::new(format!(
                "token.{token_prefix}.{}",
                rank_slug(rank)
            ))
            .unwrap(),
            clinician: clinician.clone(),
            recognized_rank: rank,
            evidence: rank_evidence(rank),
            clinical_experience: vec![format!("{} clinical experience", rank_slug(rank))],
            authority_granted: vec![format!("{} clinical authority", rank_slug(rank))],
            education_access: vec![format!("{} education access", rank_slug(rank))],
            patient_responsibility: vec![format!("{} patient responsibility", rank_slug(rank))],
            objective_benefit: format!(
                "{} work produced a documented patient benefit",
                rank_slug(rank)
            ),
        })
        .collect::<Vec<_>>();
    (
        glaushouse::ClinicalStanding {
            clinician,
            earned_ranks: ranks.iter().copied().collect(),
            active_branch_emphasis: None,
            advancement_tokens: tokens.iter().map(|token| token.id.clone()).collect(),
        },
        tokens,
    )
}

fn qualified_standing(
    clinician: IdentityId,
    token_prefix: &str,
) -> (
    glaushouse::ClinicalStanding,
    Vec<glaushouse::AdvancementToken>,
) {
    standing(
        clinician,
        token_prefix,
        &[
            glaushouse::ClinicalRank::Nightingale,
            glaushouse::ClinicalRank::Matron,
            glaushouse::ClinicalRank::Marshal,
            glaushouse::ClinicalRank::Persephone,
        ],
    )
}

fn candidacy_evidence() -> BTreeSet<glaushouse::PrimaDonnaEvidenceKind> {
    use glaushouse::PrimaDonnaEvidenceKind as Evidence;
    BTreeSet::from([
        Evidence::NightingaleFoundation,
        Evidence::MatronMastery,
        Evidence::MarshalMastery,
        Evidence::PersephoneService,
        Evidence::PhysicianMastery,
        Evidence::PatientOutcomes,
        Evidence::TechnicalViabilityJudgment,
        Evidence::LivedViabilityJudgment,
        Evidence::DiagnosticMastery,
        Evidence::RecipeAuthorshipOrRevision,
        Evidence::SurgicalAuthority,
        Evidence::SynthesisLedgerStewardship,
        Evidence::TeachingAndCultivation,
        Evidence::AdvancementPathsRemainOpen,
    ])
}

fn succession_sources() -> BTreeSet<glaushouse::PrimaDonnaSuccessionSource> {
    use glaushouse::PrimaDonnaSuccessionSource as Source;
    BTreeSet::from([
        Source::NightingaleTestimony,
        Source::MatronRecordOrTestimony,
        Source::MarshalRecordOrTestimony,
        Source::PersephoneRecordOrTestimony,
        Source::TreatedHuemanTestimony,
        Source::LivingLedger,
        Source::RecipeLedger,
        Source::ClinicalOutcomes,
        Source::TeachingHistory,
    ])
}

fn candidacy(id: &str, candidate: IdentityId) -> glaushouse::PrimaDonnaCandidacy {
    glaushouse::PrimaDonnaCandidacy {
        id: glaushouse::PrimaDonnaCandidacyId::new(id).unwrap(),
        candidate,
        evidence: candidacy_evidence(),
        succession_sources: succession_sources(),
        testimony: vec![
            "Nightingale testimony".into(),
            "Matron and Marshal testimony".into(),
            "Persephone and treated-Hueman testimony".into(),
            "Living Ledger, Recipe Ledger, outcomes, and teaching history".into(),
        ],
        eligible: true,
    }
}

fn accession(
    id: &str,
    holder: IdentityId,
    candidacy: &glaushouse::PrimaDonnaCandidacy,
) -> glaushouse::AccessionRecord {
    glaushouse::AccessionRecord {
        id: glaushouse::AccessionRecordId::new(id).unwrap(),
        office: glaushouse::ClinicalOffice::PrimaDonna,
        holder,
        active: true,
        tombstoned: false,
        origin: glaushouse::AuthorityOrigin::ClinicalAccession,
        clinical_competence_reviewed: true,
        nightingale_testimony_recorded: true,
        persephone_review_completed: true,
        flynt_recognition_recorded: true,
        stonebend_title_recorded: true,
        evidence: vec!["complete sealed accession".into()],
        candidacy: candidacy.id.clone(),
        advancement_paths_open: true,
    }
}

fn technical_viability() -> glaushouse::TechnicalViability {
    glaushouse::TechnicalViability {
        recipe_possible: true,
        materials_available: true,
        compatibility_understood: true,
        intended_form_described: true,
        risks_known: true,
        lawful_path_exists: true,
    }
}

fn lived_viability() -> glaushouse::LivedViability {
    glaushouse::LivedViability {
        consent_meaningful: true,
        consequences_understood: true,
        maintenance_resources_exist: true,
        recovery_resources_exist: true,
        continuing_care_available: true,
        expected_form_livable: true,
        coercion_excluded: true,
    }
}

fn continuance() -> glaushouse::SynthesisContinuance {
    glaushouse::SynthesisContinuance {
        recipe: glaushouse::SynthesisRecipeReference {
            name: "ratified regional recipe".into(),
            revision: "v3 maintained-continuance".into(),
        },
        ways: vec![glaushouse::SynthesisWay {
            name: "regional maintained Form Ways".into(),
            practices: vec![
                "daily bodily discipline".into(),
                "scheduled Recipe practice".into(),
            ],
        }],
        maintenance: vec!["Current and Aura calibration".into()],
        renewal: vec!["scheduled Sympiote renewal".into()],
        environmental_conditions: vec!["compatible pressure and temperature".into()],
        institutional_care: vec![glaushouse::glauspitals_id()],
        conditions: glaushouse::ContinuanceConditions {
            maintenance_current: true,
            renewal_current: true,
            recipe_practiced: true,
            environment_compatible: true,
            institutional_care_available: true,
            ways_known_and_practiced: true,
        },
        expected_continuance: glaushouse::ContinuanceHorizon::NaturalLifePossible,
    }
}

fn valid_registry() -> glaushouse::GlaushouseRegistry {
    let prima_donna = glaushouse::doctor_ratchet_identity_id();
    let persephone_one = glaushouse::nurse_house_identity_id();
    let persephone_two = identity("being.glaushouse.second-persephone");
    let patient = identity("being.glaushouse.test-patient");
    let material = identity("material.glaushouse.test-gel");
    let source = identity("being.glaushouse.test-donor");
    let nightingale = identity("being.glaushouse.test-nightingale");
    let capacity = glaushouse::CapacityRecordId::new("capacity.glaushouse.test").unwrap();
    let consent = glaushouse::ConsentRecordId::new("consent.glaushouse.test").unwrap();
    let diagnosis = glaushouse::DiagnosisRecordId::new("diagnosis.glaushouse.test").unwrap();
    let recovery = glaushouse::RecoveryPlanId::new("recovery.glaushouse.test").unwrap();
    let clearance = glaushouse::ClearanceRecordId::new("clearance.glaushouse.test").unwrap();
    let privilege = glaushouse::PrivilegeRecordId::new("privilege.glaushouse.test").unwrap();
    let material_record =
        glaushouse::MaterialRecordId::new("material-record.glaushouse.test").unwrap();

    let (prima_standing, mut tokens) =
        qualified_standing(prima_donna.clone(), "glaushouse.doctor-ratchet");
    let (persephone_one_standing, more_tokens) =
        qualified_standing(persephone_one.clone(), "glaushouse.nurse-house");
    tokens.extend(more_tokens);
    let (persephone_two_standing, more_tokens) =
        qualified_standing(persephone_two.clone(), "glaushouse.second-persephone");
    tokens.extend(more_tokens);
    let (nightingale_standing, more_tokens) = standing(
        nightingale.clone(),
        "glaushouse.test-nightingale",
        &[glaushouse::ClinicalRank::Nightingale],
    );
    tokens.extend(more_tokens);

    let prima_candidacy = candidacy("candidacy.glaushouse.doctor-ratchet", prima_donna.clone());

    glaushouse::GlaushouseRegistry {
        subjects: [
            (prima_donna.clone(), "Doctor Ratchet"),
            (persephone_one.clone(), "Nurse House"),
            (persephone_two.clone(), "second Persephone"),
            (patient.clone(), "patient identity before Synthesis"),
            (material.clone(), "Glaus Gel material identity"),
            (source.clone(), "lawful material source"),
            (nightingale.clone(), "Nightingale witness identity"),
        ]
        .into_iter()
        .map(|(id, history)| glaushouse::ClinicalSubjectRecord {
            id,
            prior_identity_history: vec![history.into()],
            tombstoned: false,
        })
        .collect(),
        clinical_standings: vec![
            prima_standing,
            persephone_one_standing,
            persephone_two_standing,
            nightingale_standing,
        ],
        advancement_tokens: tokens,
        prima_donna_candidacies: vec![prima_candidacy.clone()],
        accessions: vec![accession(
            "accession.glaushouse.test-prima-donna",
            prima_donna.clone(),
            &prima_candidacy,
        )],
        diagnoses: vec![glaushouse::DiagnosisRecord {
            id: diagnosis.clone(),
            subject: patient.clone(),
            status: glaushouse::DiagnosisStatus::TransformationReadiness,
            findings: "regional adaptation is indicated and stable".into(),
            evidence: vec!["observed readiness record".into()],
            uncertainty_disclosed: true,
            operator: prima_donna.clone(),
        }],
        capacities: vec![glaushouse::CapacityRecord {
            id: capacity.clone(),
            subject: patient.clone(),
            understands_information: true,
            appreciates_consequences: true,
            compares_options: true,
            communicates_choice: true,
            support_offered: vec!["plain-language explanation".into()],
            assessed_at: 1,
            expires_at: 20,
        }],
        consents: vec![glaushouse::ConsentRecord {
            id: consent.clone(),
            subject: patient.clone(),
            scope: glaushouse::ConsentScope::Synthesis,
            procedure: "regional Synthesis".into(),
            capacity: capacity.clone(),
            origin: glaushouse::ConsentOrigin::Explicit,
            informed: true,
            voluntary: true,
            specific: true,
            comprehensible: true,
            current: true,
            material_risks_disclosed: true,
            alternatives_disclosed: true,
            recovery_disclosed: true,
            experimental_status_disclosed: false,
            withdrawn_at: None,
        }],
        clearances: vec![glaushouse::ClearanceRecord {
            id: clearance.clone(),
            subject: patient.clone(),
            diagnosis,
            procedure: "regional Synthesis".into(),
            operator: prima_donna.clone(),
            facility: "Glauspitals regional Synthesis hall".into(),
            scope: "one named subject and one regional transformation".into(),
            class: glaushouse::ClearanceClass::Synthesis,
            consent: consent.clone(),
            capacity,
            required_equipment: vec!["Current stabilizer".into()],
            required_witnesses: vec!["Nightingale".into()],
            emergency_plan: "stop, stabilize, and transfer to recovery".into(),
            recovery_plan: Some(recovery.clone()),
            stopping_conditions: vec!["Aura or Current instability".into()],
            issued_at: 2,
            expires_at: 20,
            status: glaushouse::ClearanceStatus::Active,
            review_authority: glaushouse::PrincipalAuthority::PrimaDonna,
        }],
        privileges: vec![glaushouse::OperatorPrivilegeRecord {
            id: privilege.clone(),
            operator: prima_donna.clone(),
            permitted_procedures: vec!["regional Synthesis".into()],
            facility: "Glauspitals regional Synthesis hall".into(),
            status: glaushouse::PrivilegeStatus::Active,
            valid_until: 20,
        }],
        materials: vec![glaushouse::SynthesisMaterialRecord {
            id: material_record.clone(),
            material,
            source,
            provenance: vec!["lawful source and procedure record".into()],
            custody_chain: vec!["source -> Glauspitals pharmacy -> operator".into()],
            lawfully_obtained: true,
            illegally_hollowed: false,
        }],
        recovery_plans: vec![glaushouse::RecoveryPlan {
            id: recovery.clone(),
            subject: patient.clone(),
            stabilization: vec!["Current and Aura stabilization".into()],
            monitoring: vec!["continuous adverse-event monitoring".into()],
            rehabilitation: vec!["new Frame movement training".into()],
            identity_support: vec!["prior identity continuity review".into()],
            discharge_conditions: vec!["stable and informed".into()],
            responsible_institution: glaushouse::glauspitals_id(),
        }],
        syntheses: vec![glaushouse::SynthesisRecord {
            id: glaushouse::SynthesisRecordId::new("synthesis.glaushouse.test").unwrap(),
            subject: patient.clone(),
            class: glaushouse::SynthesisClass::Regional,
            depth: glaushouse::SynthesisDepth::Transfiguration,
            recipe: "ratified regional recipe".into(),
            intended_result: "lawful regional Frame".into(),
            clearance,
            consent,
            operator_privilege: privilege,
            materials: vec![material_record],
            recovery_plan: Some(recovery),
            started_at: 10,
            irreversible_threshold_at: Some(12),
            experimental_marked: false,
            nightingale_witness: Some(nightingale),
            emergency_post_event_review: None,
            actual_outcome: glaushouse::SynthesisOutcome::Successful,
            recorded_outcome: glaushouse::SynthesisOutcome::Successful,
            lifecycle: glaushouse::SynthesisLifecycle {
                history: vec![
                    glaushouse::SynthesisLifecycleState::Established,
                    glaushouse::SynthesisLifecycleState::Integrated,
                    glaushouse::SynthesisLifecycleState::Maintained,
                ],
            },
            continuance: continuance(),
            rejection: None,
            technical_viability: Some(technical_viability()),
            lived_viability: Some(lived_viability()),
            prior_identity_history_preserved: true,
            resulting_title_or_office: false,
            stabilized: true,
            recovery_status_recorded: true,
        }],
        living_ledger: vec![glaushouse::LivingLedgerEntry {
            id: glaushouse::LedgerEntryId::new("ledger.living.test").unwrap(),
            subject: patient,
            matron: glaushouse::MatronEvidence {
                cognition: "coherent".into(),
                perception: "adapted".into(),
                consent: "meaningful and current".into(),
                emotion: "livable".into(),
                identity: "continuous".into(),
                aura_coherence: "stable".into(),
                patient_testimony: "the Form remains mine".into(),
                lived_adaptation: "daily life supported".into(),
            },
            marshal: glaushouse::MarshalEvidence {
                current_flow: "stable".into(),
                structural_condition: "integrated".into(),
                graft_integrity: "intact".into(),
                mobility: "functional".into(),
                physical_tolerance: "within range".into(),
                containment: "not required".into(),
                bodily_adaptation: "maintained".into(),
            },
            overseen_by_persephones: vec![persephone_one, persephone_two],
            continuities_form_one_viable_life: true,
        }],
        recipe_ledger: vec![glaushouse::RecipeLedgerEntry {
            id: glaushouse::LedgerEntryId::new("ledger.recipe.test").unwrap(),
            recipe: glaushouse::SynthesisRecipeReference {
                name: "ratified regional recipe".into(),
                revision: "v3 maintained-continuance".into(),
            },
            architecture: vec!["host-specific integration architecture".into()],
            revision_history: vec!["renewal protocol clarified".into()],
            authorized_by_prima_donna: prima_donna,
        }],
        ..glaushouse::GlaushouseRegistry::default()
    }
}

#[test]
fn institutional_projection_has_one_office_and_four_open_ranks() {
    glaushouse::validate_principal_authorities().unwrap();
    let catalog = house_institutions::canonical_house_institutions();
    catalog.validate().unwrap();

    let offices = catalog
        .offices
        .iter()
        .filter(|entry| entry.house == Some(House::Glaushouse))
        .collect::<Vec<_>>();
    assert_eq!(offices.len(), 1);
    assert_eq!(offices[0].id, glaushouse::prima_donna_office_id());
    assert!(offices[0].singular);

    let ranks = catalog
        .roles
        .iter()
        .filter(|entry| entry.institution == glaushouse::nightingales_id())
        .map(|entry| entry.id.as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        ranks,
        BTreeSet::from([
            "role.glaushouse.nightingale",
            "role.glaushouse.matron",
            "role.glaushouse.marshal",
            "role.glaushouse.persephone",
        ])
    );
    let definitions = glaushouse::PRINCIPAL_AUTHORITIES;
    let matron = definitions
        .iter()
        .find(|entry| entry.authority == glaushouse::PrincipalAuthority::Matron)
        .unwrap();
    let marshal = definitions
        .iter()
        .find(|entry| entry.authority == glaushouse::PrincipalAuthority::Marshal)
        .unwrap();
    assert!(!matron.singular);
    assert!(!marshal.singular);
    assert_eq!(
        glaushouse::ClinicalRank::Matron.advancement_level(),
        glaushouse::ClinicalRank::Marshal.advancement_level()
    );
}

#[test]
fn complete_clinical_and_synthesis_record_validates_with_multiple_persephones() {
    let registry = valid_registry();
    registry.validate().unwrap();
    assert_eq!(
        registry
            .clinical_standings
            .iter()
            .filter(|standing| standing.is_qualified_persephone())
            .count(),
        3
    );
    assert_eq!(
        registry
            .accessions
            .iter()
            .filter(|accession| accession.active)
            .count(),
        1
    );
}

#[test]
fn nightingale_may_choose_either_equal_branch_and_cross_train() {
    for (name, ranks) in [
        (
            "matron",
            vec![
                glaushouse::ClinicalRank::Nightingale,
                glaushouse::ClinicalRank::Matron,
            ],
        ),
        (
            "marshal",
            vec![
                glaushouse::ClinicalRank::Nightingale,
                glaushouse::ClinicalRank::Marshal,
            ],
        ),
        (
            "cross-trained",
            vec![
                glaushouse::ClinicalRank::Nightingale,
                glaushouse::ClinicalRank::Matron,
                glaushouse::ClinicalRank::Marshal,
            ],
        ),
    ] {
        let clinician = identity(&format!("being.glaushouse.{name}"));
        let (standing, tokens) = standing(clinician.clone(), &format!("glaushouse.{name}"), &ranks);
        let registry = glaushouse::GlaushouseRegistry {
            subjects: vec![glaushouse::ClinicalSubjectRecord {
                id: clinician,
                prior_identity_history: vec!["one stable clinician".into()],
                tombstoned: false,
            }],
            clinical_standings: vec![standing],
            advancement_tokens: tokens,
            ..glaushouse::GlaushouseRegistry::default()
        };
        registry.validate().unwrap();
    }
}

#[test]
fn persephone_requires_both_domains_and_preserves_one_identity() {
    let clinician = identity("being.glaushouse.unbalanced-persephone");
    let ranks = [
        glaushouse::ClinicalRank::Nightingale,
        glaushouse::ClinicalRank::Matron,
        glaushouse::ClinicalRank::Persephone,
    ];
    let (standing, tokens) = standing(
        clinician.clone(),
        "glaushouse.unbalanced-persephone",
        &ranks,
    );
    let registry = glaushouse::GlaushouseRegistry {
        subjects: vec![glaushouse::ClinicalSubjectRecord {
            id: clinician.clone(),
            prior_identity_history: vec!["same person before advancement".into()],
            tombstoned: false,
        }],
        clinical_standings: vec![standing],
        advancement_tokens: tokens,
        ..glaushouse::GlaushouseRegistry::default()
    };
    assert_eq!(
        registry.validate(),
        Err(glaushouse::GlaushouseValidationError::UnbalancedPersephone(
            clinician
        ))
    );
}

#[test]
fn existing_rank_holders_do_not_close_new_candidacy() {
    let mut registry = valid_registry();
    let newcomer = identity("being.glaushouse.new-matron");
    let (standing, tokens) = standing(
        newcomer.clone(),
        "glaushouse.new-matron",
        &[
            glaushouse::ClinicalRank::Nightingale,
            glaushouse::ClinicalRank::Matron,
        ],
    );
    registry.subjects.push(glaushouse::ClinicalSubjectRecord {
        id: newcomer,
        prior_identity_history: vec!["new clinical candidate".into()],
        tombstoned: false,
    });
    registry.clinical_standings.push(standing);
    registry.advancement_tokens.extend(tokens);
    registry.validate().unwrap();
}

#[test]
fn multiple_active_prima_donnas_are_rejected_but_vacancy_is_valid() {
    let mut vacancy = valid_registry();
    vacancy.accessions[0].active = false;
    vacancy.validate().unwrap();

    let mut duplicate = valid_registry();
    let second = identity("being.glaushouse.second-prima-candidate");
    let (standing, tokens) =
        qualified_standing(second.clone(), "glaushouse.second-prima-candidate");
    let second_candidacy = candidacy("candidacy.glaushouse.second-prima", second.clone());
    duplicate.subjects.push(glaushouse::ClinicalSubjectRecord {
        id: second.clone(),
        prior_identity_history: vec!["qualified Persephone candidate".into()],
        tombstoned: false,
    });
    duplicate.clinical_standings.push(standing);
    duplicate.advancement_tokens.extend(tokens);
    duplicate
        .prima_donna_candidacies
        .push(second_candidacy.clone());
    duplicate.accessions.push(accession(
        "accession.glaushouse.second-prima",
        second,
        &second_candidacy,
    ));
    assert_eq!(
        duplicate.validate(),
        Err(glaushouse::GlaushouseValidationError::ActivePrimaDonnaCount(2))
    );
}

#[test]
fn prima_donna_requires_qualified_persephone_and_open_paths() {
    let mut unqualified = valid_registry();
    unqualified
        .clinical_standings
        .iter_mut()
        .find(|standing| standing.clinician == glaushouse::doctor_ratchet_identity_id())
        .unwrap()
        .earned_ranks
        .remove(&glaushouse::ClinicalRank::Marshal);
    assert!(matches!(
        unqualified.validate(),
        Err(glaushouse::GlaushouseValidationError::UnbalancedPersephone(
            _
        )) | Err(glaushouse::GlaushouseValidationError::UnqualifiedPrimaDonnaCandidate(_))
    ));

    let mut closed = valid_registry();
    closed.accessions[0].advancement_paths_open = false;
    assert!(matches!(
        closed.validate(),
        Err(glaushouse::GlaushouseValidationError::InvalidAccession(_))
    ));

    let mut missing_voice = valid_registry();
    missing_voice.prima_donna_candidacies[0]
        .succession_sources
        .remove(&glaushouse::PrimaDonnaSuccessionSource::TreatedHuemanTestimony);
    assert!(matches!(
        missing_voice.validate(),
        Err(glaushouse::GlaushouseValidationError::UnqualifiedPrimaDonnaCandidate(_))
    ));
}

#[test]
fn advancement_tokens_create_experience_authority_access_and_responsibility() {
    let registry = valid_registry();
    for token in &registry.advancement_tokens {
        assert!(!token.clinical_experience.is_empty());
        assert!(!token.authority_granted.is_empty());
        assert!(!token.education_access.is_empty());
        assert!(!token.patient_responsibility.is_empty());
        assert!(!token.objective_benefit.is_empty());
    }
}

#[test]
fn maintained_form_can_continue_for_life_and_renew() {
    let plan = continuance();
    assert!(plan.may_continue_for_natural_life());
    let lifecycle = glaushouse::SynthesisLifecycle {
        history: vec![
            glaushouse::SynthesisLifecycleState::Established,
            glaushouse::SynthesisLifecycleState::Integrated,
            glaushouse::SynthesisLifecycleState::Maintained,
            glaushouse::SynthesisLifecycleState::Renewed,
            glaushouse::SynthesisLifecycleState::Maintained,
            glaushouse::SynthesisLifecycleState::Refined,
        ],
    };
    lifecycle.validate().unwrap();
}

#[test]
fn refinement_revision_regression_and_collapse_are_distinct_transitions() {
    for final_state in [
        glaushouse::SynthesisLifecycleState::Refined,
        glaushouse::SynthesisLifecycleState::Revised,
        glaushouse::SynthesisLifecycleState::Regressed,
        glaushouse::SynthesisLifecycleState::SafelyDiscontinued,
        glaushouse::SynthesisLifecycleState::CatastrophicallyCollapsed,
    ] {
        let lifecycle = glaushouse::SynthesisLifecycle {
            history: vec![
                glaushouse::SynthesisLifecycleState::Established,
                glaushouse::SynthesisLifecycleState::Integrated,
                glaushouse::SynthesisLifecycleState::Maintained,
                final_state,
            ],
        };
        lifecycle.validate().unwrap();
    }
    assert_ne!(
        glaushouse::SynthesisLifecycleState::Regressed,
        glaushouse::SynthesisLifecycleState::CatastrophicallyCollapsed
    );
    assert_ne!(
        glaushouse::SynthesisOutcome::SafeRegression,
        glaushouse::SynthesisOutcome::CatastrophicCollapse
    );
}

#[test]
fn overgrowth_is_an_emergency_failure_not_an_intended_depth() {
    assert!(!glaushouse::SynthesisDepth::Overgrowth.is_intended_clinical_depth());
    let mut registry = valid_registry();
    registry.syntheses[0].depth = glaushouse::SynthesisDepth::Overgrowth;
    assert!(matches!(
        registry.validate(),
        Err(glaushouse::GlaushouseValidationError::OvergrowthTreatedAsIntendedForm(_))
    ));
}

#[test]
fn host_and_sympiote_rejection_are_distinct() {
    let host = glaushouse::RejectionRecord::Host {
        mechanisms: vec![glaushouse::HostRejectionMechanism::Expel],
    };
    let sympiote = glaushouse::RejectionRecord::Sympiote {
        mechanisms: vec![glaushouse::SympioteRejectionMechanism::ConstructUnconsentedBody],
    };
    assert_ne!(host, sympiote);

    let mut registry = valid_registry();
    registry.syntheses[0].actual_outcome = glaushouse::SynthesisOutcome::HostRejection;
    registry.syntheses[0].recorded_outcome = glaushouse::SynthesisOutcome::HostRejection;
    registry.syntheses[0].rejection = Some(sympiote);
    assert!(matches!(
        registry.validate(),
        Err(glaushouse::GlaushouseValidationError::RejectionDirectionMismatch(_))
    ));
}

#[test]
fn major_synthesis_requires_technical_and_lived_viability() {
    for technical_missing in [true, false] {
        let mut registry = valid_registry();
        if technical_missing {
            registry.syntheses[0].technical_viability = None;
        } else {
            registry.syntheses[0].lived_viability = None;
        }
        assert!(matches!(
            registry.validate(),
            Err(glaushouse::GlaushouseValidationError::MajorSynthesisWithoutViability(_))
        ));
    }
}

#[test]
fn living_and_recipe_ledgers_are_distinct_authority_records() {
    let mut registry = valid_registry();
    registry.recipe_ledger[0].id = registry.living_ledger[0].id.clone();
    assert_eq!(
        registry.validate(),
        Err(glaushouse::GlaushouseValidationError::LedgerLayersCollapsed)
    );
    assert_ne!(
        glaushouse::LedgerLayer::Living,
        glaushouse::LedgerLayer::Recipe
    );
}

#[test]
fn invalid_consent_clearance_and_material_still_block_synthesis() {
    let mut invalid_consent = valid_registry();
    invalid_consent.consents[0].origin = glaushouse::ConsentOrigin::Recognition;
    assert!(matches!(
        invalid_consent.validate(),
        Err(glaushouse::GlaushouseValidationError::InvalidConsent(_))
    ));

    let mut expired = valid_registry();
    expired.clearances[0].expires_at = 9;
    assert!(matches!(
        expired.validate(),
        Err(glaushouse::GlaushouseValidationError::SynthesisWithoutClearance(_))
    ));

    let mut illegal_material = valid_registry();
    illegal_material.materials[0].illegally_hollowed = true;
    assert!(matches!(
        illegal_material.validate(),
        Err(glaushouse::GlaushouseValidationError::UnlawfulMaterial(_))
    ));
}

#[test]
fn transformation_never_creates_office_or_erases_identity_history() {
    let mut office = valid_registry();
    office.syntheses[0].resulting_title_or_office = true;
    assert!(matches!(
        office.validate(),
        Err(glaushouse::GlaushouseValidationError::TransformationCreatedAuthority(_))
    ));

    let mut erased = valid_registry();
    erased.syntheses[0].prior_identity_history_preserved = false;
    assert!(matches!(
        erased.validate(),
        Err(glaushouse::GlaushouseValidationError::IdentityHistoryErased(_))
    ));
}

#[test]
fn ordered_selection_process_requires_every_evidence_phase() {
    let mut process =
        glaushouse::PrimaDonnaSelectionProcess::open(glaushouse::doctor_ratchet_identity_id());
    while let Some(next) = process.phase().next() {
        process
            .advance(next, format!("constitutional evidence for {next:?}"))
            .unwrap();
    }
    process.require_complete().unwrap();
}
