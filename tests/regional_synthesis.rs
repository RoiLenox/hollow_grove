use hollow_grove::constitutional::*;
use hollow_grove::lineage_contract::{SandmanorForm, SandmanorTransitionError};

fn regional_id(value: &str) -> RegionalBeingId {
    RegionalBeingId::new(value).expect("test regional Being ID")
}

fn setup_origin(
    label: &str,
    form: SandmanorForm,
    region: ConstitutionalRegion,
) -> (
    hollow_grove::institution_affiliation::InstitutionalWorldState,
    RegionalSynthesisRuntime,
    RegionalBeingId,
) {
    let world = hollow_grove::world::institutional_access_fixture();
    let origin = regional_id(&format!("being.test.{label}.origin"));
    let mut runtime = RegionalSynthesisRuntime::new();
    runtime
        .register_being(
            scenario_regional_metadata(&format!("test.{label}.register"), 1),
            scenario_regional_registration(
                &world.catalog,
                origin.clone(),
                form,
                region,
                RegionalStandingKind::Established,
            )
            .unwrap(),
        )
        .unwrap();
    (world, runtime, origin)
}

#[test]
fn gnome_synthesizes_to_minotaur_with_complete_field_stewardship() {
    let scenario = run_gnome_minotaur_scenario().unwrap();
    let source = scenario.runtime.being(&scenario.predecessor).unwrap();
    let minotaur = scenario.runtime.being(&scenario.result).unwrap();
    assert_eq!(source.form, SandmanorForm::Gnome);
    assert_eq!(
        source.status,
        RegionalBeingStatus::SynthesizedInto(scenario.result.clone())
    );
    assert_eq!(minotaur.form, SandmanorForm::Minotaur);
    assert_eq!(minotaur.predecessor.as_ref(), Some(&scenario.predecessor));
    assert_eq!(minotaur.standing.region, ConstitutionalRegion::AuraFields);
    assert_eq!(scenario.runtime.lineage(&scenario.result).unwrap().len(), 2);

    let stewardship = scenario.runtime.stewardship(&scenario.result).unwrap();
    assert_eq!(stewardship.region, ConstitutionalRegion::AuraFields);
    assert_eq!(stewardship.steward, scenario.result);
    assert_eq!(stewardship.duties.len(), 7);
    for duty in [
        AuraFieldsDuty::TendAuraCrops,
        AuraFieldsDuty::GuardFieldBoundary,
        AuraFieldsDuty::CarryFieldLoad,
        AuraFieldsDuty::MaintainFieldRoute,
        AuraFieldsDuty::GuardHarvest,
        AuraFieldsDuty::ProtectFieldWorker,
        AuraFieldsDuty::StabilizeFieldCurrent,
    ] {
        assert!(stewardship.duties.contains(&duty));
    }
    assert!(scenario.runtime.guardianship(&scenario.result).is_none());
    assert!(
        scenario
            .runtime
            .beach_occupation(&scenario.result)
            .is_none()
    );
}

#[test]
fn elf_synthesizes_to_centaur_with_beach_patrol_and_sea_guardianship() {
    let scenario = run_elf_centaur_scenario().unwrap();
    let centaur = scenario.runtime.being(&scenario.result).unwrap();
    assert_eq!(centaur.form, SandmanorForm::Centaur);
    assert_eq!(centaur.predecessor.as_ref(), Some(&scenario.predecessor));
    assert_eq!(centaur.standing.region, ConstitutionalRegion::AuraBeach);

    let beach = scenario.runtime.beach_occupation(&scenario.result).unwrap();
    assert_eq!(beach.region, ConstitutionalRegion::AuraBeach);
    assert_eq!(beach.duties.len(), 8);
    for duty in [
        AuraBeachDuty::RoamAuraBeach,
        AuraBeachDuty::PatrolShoreline,
        AuraBeachDuty::GuardAuraSeaAccess,
        AuraBeachDuty::WatchCoastalRoute,
        AuraBeachDuty::EscortTraveler,
        AuraBeachDuty::RecognizeHorizonChange,
        AuraBeachDuty::DefendCoastalIncursion,
        AuraBeachDuty::MaintainLandSeaBoundary,
    ] {
        assert!(beach.duties.contains(&duty));
    }

    let guardianship = scenario.runtime.guardianship(&scenario.result).unwrap();
    assert_eq!(guardianship.region, ConstitutionalRegion::AuraSea);
    assert_eq!(guardianship.guardian, scenario.result);
    assert_eq!(guardianship.duties.len(), 4);
    assert!(scenario.runtime.stewardship(&scenario.result).is_none());
}

#[test]
fn regional_persistence_replay_and_migration_are_canonical() {
    for scenario in [
        run_gnome_minotaur_scenario().unwrap(),
        run_elf_centaur_scenario().unwrap(),
    ] {
        let replayed =
            RegionalSynthesisRuntime::replay(scenario.runtime.events().iter().cloned()).unwrap();
        assert_eq!(replayed, scenario.runtime);
        let encoded = encode_regional_archive(&scenario.runtime).unwrap();
        let decoded = decode_regional_archive(&encoded).unwrap();
        assert_eq!(decoded, scenario.runtime);
        assert_eq!(encode_regional_archive(&decoded).unwrap(), encoded);
        assert_eq!(
            regional_archive_digest(&decoded).unwrap(),
            regional_archive_digest(&scenario.runtime).unwrap()
        );

        let legacy = encode_legacy_regional_archive_v0(&scenario.runtime).unwrap();
        let migrated = migrate_regional_archive(&legacy).unwrap();
        assert_eq!(migrated, encoded);
        assert_eq!(
            decode_regional_archive(&migrated).unwrap(),
            scenario.runtime
        );
    }
}

#[test]
fn idempotent_synthesis_retry_has_one_effect_and_stable_digest() {
    let scenario = run_gnome_minotaur_scenario().unwrap();
    assert_eq!(scenario.retry_event_count, 2);
    assert_eq!(scenario.runtime.events().len(), 2);
    assert_eq!(
        regional_archive_digest(&scenario.runtime).unwrap(),
        regional_archive_digest(&decode_regional_archive(&scenario.archive).unwrap()).unwrap()
    );
    assert_eq!(
        scenario
            .runtime
            .events()
            .iter()
            .filter(|event| matches!(event.payload, RegionalEvent::SynthesisCompleted(_)))
            .count(),
        1
    );
}

#[test]
fn reversed_and_cross_lineage_transformations_fail_without_mutation() {
    for name in ["gnome-centaur", "elf-minotaur"] {
        let scenario = run_rejected_regional_scenario(name).unwrap();
        assert_eq!(scenario.event_count_before, scenario.event_count_after);
        assert!(scenario.runtime.being(&scenario.attempted_result).is_none());
        assert!(matches!(
            scenario.error,
            RegionalSynthesisError::Lineage(
                SandmanorTransitionError::CrossLineage | SandmanorTransitionError::IllegalStageSkip
            )
        ));
    }
}

#[test]
fn correct_forms_in_wrong_regions_fail_without_mutation() {
    for name in ["gnome-minotaur-wrong-region", "elf-centaur-wrong-region"] {
        let scenario = run_rejected_regional_scenario(name).unwrap();
        assert_eq!(scenario.event_count_before, scenario.event_count_after);
        assert!(matches!(
            scenario.error,
            RegionalSynthesisError::InsufficientRegionalStanding { .. }
        ));
    }
}

#[test]
fn authority_and_subject_evidence_fail_closed() {
    let no_authority = run_rejected_regional_scenario("synthesis-without-authority").unwrap();
    assert!(matches!(
        no_authority.error,
        RegionalSynthesisError::HouseLaw(HouseLawError::DecisionNotAccepted(_))
    ));
    let no_evidence = run_rejected_regional_scenario("synthesis-without-evidence").unwrap();
    assert!(matches!(
        no_evidence.error,
        RegionalSynthesisError::MissingEvidence("regional Synthesis")
    ));
    let mismatched = run_rejected_regional_scenario("synthesis-mismatched-evidence").unwrap();
    assert!(matches!(
        mismatched.error,
        RegionalSynthesisError::EvidenceSubjectMismatch { .. }
    ));
    for scenario in [no_authority, no_evidence, mismatched] {
        assert_eq!(scenario.event_count_before, scenario.event_count_after);
        assert!(scenario.runtime.being(&scenario.attempted_result).is_none());
    }
}

#[test]
fn minotaur_and_centaur_cannot_claim_each_others_regional_authority() {
    for name in ["minotaur-sea-claim", "centaur-fields-claim"] {
        let scenario = run_rejected_assignment_scenario(name).unwrap();
        assert!(matches!(
            scenario.error,
            RegionalSynthesisError::AssignmentNotHeld { .. }
        ));
        assert_eq!(scenario.error.code(), "REGIONAL_ASSIGNMENT_NOT_HELD");
        assert_eq!(scenario.event_count_before, scenario.event_count_after);
        assert!(!trace_rejected_assignment_scenario(&scenario).transitions[0].state_changed);
    }
}

#[test]
fn synthesis_after_tombstone_is_rejected_and_cannot_resurrect_source() {
    let (world, mut runtime, origin) = setup_origin(
        "tombstone",
        SandmanorForm::Gnome,
        ConstitutionalRegion::AuraFields,
    );
    runtime
        .tombstone_being(
            scenario_regional_metadata("test.tombstone.close", 2),
            RegionalTombstoneRecord {
                being: origin.clone(),
                tombstone: TombstoneId::new("tombstone.regional-origin").unwrap(),
                evidence: vec![scenario_subject_evidence(&origin, "regional.tombstone")],
            },
        )
        .unwrap();
    let result = regional_id("being.test.tombstone.result");
    let command = scenario_regional_command(
        &world.catalog,
        "test.tombstone",
        &origin,
        result.clone(),
        SandmanorForm::Gnome,
        SandmanorForm::Minotaur,
        ConstitutionalRegion::AuraFields,
        RegionalFunction::AuraFieldsStewardshipAndDefense,
    )
    .unwrap();
    let count = runtime.events().len();
    let error = runtime
        .synthesize(
            scenario_regional_metadata("test.tombstone.synthesize", 3),
            command,
        )
        .unwrap_err();
    assert!(matches!(error, RegionalSynthesisError::BeingNotActive(_)));
    assert_eq!(runtime.events().len(), count);
    assert!(runtime.being(&result).is_none());
    assert!(matches!(
        runtime.being(&origin).unwrap().status,
        RegionalBeingStatus::Tombstoned(_)
    ));
}

#[test]
fn duplicate_non_idempotent_synthesis_is_rejected() {
    let mut scenario = run_gnome_minotaur_scenario().unwrap();
    let record = scenario
        .runtime
        .synthesis(&scenario.synthesis)
        .unwrap()
        .clone();
    let count = scenario.runtime.events().len();
    let error = scenario
        .runtime
        .synthesize(
            scenario_regional_metadata("test.duplicate-synthesis", 3),
            record.command,
        )
        .unwrap_err();
    assert!(matches!(
        error,
        RegionalSynthesisError::SynthesisIdConflict(_)
    ));
    assert_eq!(scenario.runtime.events().len(), count);
}

#[test]
fn result_identity_function_and_institution_cannot_be_forged() {
    let (world, runtime, origin) = setup_origin(
        "forgery",
        SandmanorForm::Gnome,
        ConstitutionalRegion::AuraFields,
    );
    let base = scenario_regional_command(
        &world.catalog,
        "test.forgery",
        &origin,
        regional_id("being.test.forgery.result"),
        SandmanorForm::Gnome,
        SandmanorForm::Minotaur,
        ConstitutionalRegion::AuraFields,
        RegionalFunction::AuraFieldsStewardshipAndDefense,
    )
    .unwrap();

    let mut same_identity = base.clone();
    same_identity.result = origin.clone();
    assert!(matches!(
        runtime
            .clone()
            .synthesize(
                scenario_regional_metadata("test.forgery.identity", 2),
                same_identity
            )
            .unwrap_err(),
        RegionalSynthesisError::ResultReusesPredecessorIdentity
    ));

    let mut wrong_function = base.clone();
    wrong_function.requested_function = RegionalFunction::AuraBeachPatrolAndAuraSeaGuardianship;
    assert!(matches!(
        runtime
            .clone()
            .synthesize(
                scenario_regional_metadata("test.forgery.function", 2),
                wrong_function
            )
            .unwrap_err(),
        RegionalSynthesisError::WrongRegionalFunction { .. }
    ));

    let mut wrong_institution = base;
    wrong_institution
        .authority
        .glaushouse_resolution
        .authority
        .institution = None;
    assert!(matches!(
        runtime
            .clone()
            .synthesize(
                scenario_regional_metadata("test.forgery.institution", 2),
                wrong_institution
            )
            .unwrap_err(),
        RegionalSynthesisError::WrongAuthorityInstitution { .. }
    ));
    assert_eq!(runtime.events().len(), 1);
}

#[test]
fn evolved_forms_cannot_be_registered_without_predecessor_lineage() {
    let world = hollow_grove::world::institutional_access_fixture();
    for (index, form) in [SandmanorForm::Minotaur, SandmanorForm::Centaur]
        .into_iter()
        .enumerate()
    {
        let id = regional_id(&format!("being.test.unlined.{index}"));
        let region = if form == SandmanorForm::Minotaur {
            ConstitutionalRegion::AuraFields
        } else {
            ConstitutionalRegion::AuraBeach
        };
        let registration = scenario_regional_registration(
            &world.catalog,
            id,
            form,
            region,
            RegionalStandingKind::Established,
        )
        .unwrap();
        let mut runtime = RegionalSynthesisRuntime::new();
        assert!(matches!(
            runtime
                .register_being(
                    scenario_regional_metadata(&format!("test.unlined.{index}"), 1),
                    registration
                )
                .unwrap_err(),
            RegionalSynthesisError::OriginRegistrationRequired(actual) if actual == form
        ));
        assert!(runtime.events().is_empty());
    }
}

#[test]
fn aura_sea_is_guardianship_not_a_primary_synthesis_location() {
    let world = hollow_grove::world::institutional_access_fixture();
    assert!(matches!(
        RegionalJurisdictionSnapshot::from_catalog(
            &world.catalog,
            ConstitutionalRegion::AuraSea,
            CausalPosition::new(1),
            vec![scenario_evidence("aura-sea-primary")]
        ),
        Err(RegionalSynthesisError::RegionCannotBePrimaryStanding(
            ConstitutionalRegion::AuraSea
        ))
    ));
}

#[test]
fn altered_replay_history_is_detected() {
    let scenario = run_elf_centaur_scenario().unwrap();
    let mut events = scenario.runtime.events().to_vec();
    let RegionalEvent::SynthesisCompleted(record) = &mut events[1].payload else {
        panic!("second regional event must be Synthesis");
    };
    record.result.predecessor = None;
    assert!(matches!(
        RegionalSynthesisRuntime::replay(events),
        Err(RegionalSynthesisError::ReplayDivergence(_))
    ));
}

#[test]
fn unsupported_archive_versions_fail_closed() {
    let scenario = run_gnome_minotaur_scenario().unwrap();
    let mut archive = scenario.archive;
    archive[8..10].copy_from_slice(&999_u16.to_le_bytes());
    assert!(matches!(
        decode_regional_archive(&archive),
        Err(RegionalArchiveError::UnsupportedVersion(999))
    ));
}

#[test]
fn traces_report_and_tui_events_serialize_without_deciding() {
    for scenario in [
        run_gnome_minotaur_scenario().unwrap(),
        run_elf_centaur_scenario().unwrap(),
    ] {
        let event_count = scenario.runtime.events().len();
        let trace = trace_regional_scenario(&scenario).unwrap();
        assert!(trace.live_replay_equivalent);
        assert!(trace.canonical_persistence);
        assert_eq!(scenario.runtime.events().len(), event_count);
        let events = tui_events_from_trace(&trace);
        assert!(
            events
                .iter()
                .any(|event| event.kind == TuiEventKind::LineagePreserved)
        );
        for event in events {
            assert_eq!(TuiEvent::decode_line(&event.encode_line()).unwrap(), event);
        }
        assert_eq!(scenario.runtime.events().len(), event_count);
    }

    let rejected = run_rejected_regional_scenario("gnome-centaur").unwrap();
    let trace = trace_rejected_regional_scenario(&rejected);
    assert!(!trace.transitions[0].state_changed);
    assert!(
        tui_events_from_trace(&trace)
            .iter()
            .any(|event| event.kind == TuiEventKind::SynthesisRejected)
    );
    assert!(rejected.runtime.being(&rejected.attempted_result).is_none());
}

#[test]
fn every_regional_catalog_scenario_matches_its_declared_expectation() {
    for descriptor in SCENARIO_CATALOG.iter().filter(|descriptor| {
        descriptor.category == ScenarioCategory::RegionalSynthesis
            || matches!(
                descriptor.name,
                "gnome-centaur"
                    | "elf-minotaur"
                    | "gnome-minotaur-wrong-region"
                    | "elf-centaur-wrong-region"
                    | "synthesis-without-authority"
                    | "synthesis-without-evidence"
                    | "synthesis-mismatched-evidence"
                    | "minotaur-sea-claim"
                    | "centaur-fields-claim"
            )
    }) {
        match descriptor.expectation {
            ScenarioExpectation::Accepted if descriptor.name == "gnome-minotaur" => {
                run_gnome_minotaur_scenario().unwrap();
            }
            ScenarioExpectation::Accepted if descriptor.name == "elf-centaur" => {
                run_elf_centaur_scenario().unwrap();
            }
            ScenarioExpectation::Rejected => {
                if matches!(
                    descriptor.name,
                    "minotaur-sea-claim" | "centaur-fields-claim"
                ) {
                    let scenario = run_rejected_assignment_scenario(descriptor.name).unwrap();
                    assert_eq!(scenario.event_count_before, scenario.event_count_after);
                } else {
                    let scenario = run_rejected_regional_scenario(descriptor.name).unwrap();
                    assert_eq!(scenario.event_count_before, scenario.event_count_after);
                }
            }
            _ => panic!("unhandled catalog descriptor: {}", descriptor.name),
        }
    }
}
