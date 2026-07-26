use hollow_grove::constitutional::*;

fn request(id: &str, command: TuiCommand) -> TuiRequest {
    TuiRequest {
        id: id.into(),
        command,
    }
}

fn run(name: &str) -> TuiCommand {
    TuiCommand::RunScenario {
        scenario: name.into(),
    }
}

fn has_kind(response: &TuiResponse, kind: TuiEventKind) -> bool {
    response.events.iter().any(|event| event.kind == kind)
}

#[test]
fn catalog_starts_session_once_and_uses_deterministic_wire_records() {
    let mut service = ConstitutionalApplicationService::new("session.catalog").unwrap();
    let first = service
        .execute(request("request.catalog", TuiCommand::Catalog))
        .unwrap();
    assert_eq!(first.status, ApplicationResponseStatus::Completed);
    assert_eq!(first.events[0].kind, TuiEventKind::SessionStarted);
    assert_eq!(
        first
            .events
            .iter()
            .filter(|event| event.kind == TuiEventKind::CatalogEntry)
            .count(),
        SCENARIO_CATALOG.len()
    );
    assert_eq!(
        first.events.last().unwrap().kind,
        TuiEventKind::CatalogCompleted
    );
    for (sequence, event) in first.events.iter().enumerate() {
        assert_eq!(event.sequence, u64::try_from(sequence).unwrap());
        assert_eq!(TuiEvent::decode_line(&event.encode_line()).unwrap(), *event);
    }

    let audit = service
        .execute(request("request.audit", TuiCommand::Audit))
        .unwrap();
    assert!(!has_kind(&audit, TuiEventKind::SessionStarted));
}

#[test]
fn streaming_boundary_emits_the_exact_idempotent_response_in_sequence() {
    let mut service = ConstitutionalApplicationService::new("session.stream").unwrap();
    let scenario_request = request("request.stream", run("gnome-minotaur"));
    let expected = service.execute(scenario_request.clone()).unwrap();
    let mut streamed = Vec::new();
    let status = service
        .execute_streaming(scenario_request, |event| streamed.push(event.clone()))
        .unwrap();
    assert_eq!(status, expected.status);
    assert_eq!(streamed, expected.events);
    assert_eq!(service.regional_event_count(), 2);
}

#[test]
fn gnome_minotaur_runs_through_service_and_assigns_typed_stewardship() {
    let mut service = ConstitutionalApplicationService::new("session.gnome").unwrap();
    let response = service
        .execute(request("request.gnome.run", run("gnome-minotaur")))
        .unwrap();
    assert_eq!(response.status, ApplicationResponseStatus::Completed);
    for required in [
        TuiEventKind::SynthesisProposed,
        TuiEventKind::AuthorityChecked,
        TuiEventKind::EvidenceChecked,
        TuiEventKind::SynthesisAccepted,
        TuiEventKind::StateChanged,
        TuiEventKind::LineagePreserved,
        TuiEventKind::AuraFieldsAssigned,
        TuiEventKind::StewardshipGranted,
    ] {
        assert!(has_kind(&response, required), "missing {required:?}");
    }
    assert!(!has_kind(&response, TuiEventKind::AuraBeachAssigned));
    assert!(!has_kind(&response, TuiEventKind::GuardianshipGranted));
    assert_eq!(service.selected_scenario(), Some("gnome-minotaur"));
    assert_eq!(service.constitutional_event_count(), 0);
    assert_eq!(service.regional_event_count(), 2);
    let archive = service.selected_archive_metadata().unwrap();
    assert_eq!(archive.runtime, "RegionalSynthesis");
    assert!(archive.byte_length > 0);
    assert!(archive.digest.starts_with("fnv1a64-regional-v1:"));

    let before = service.regional_event_count();
    let stewardship = service
        .execute(request(
            "request.gnome.stewardship",
            TuiCommand::InspectStewardship {
                scenario: "gnome-minotaur".into(),
            },
        ))
        .unwrap();
    assert_eq!(service.regional_event_count(), before);
    let event = stewardship
        .events
        .iter()
        .find(|event| event.kind == TuiEventKind::StewardshipGranted)
        .unwrap();
    assert_eq!(event.fields["region"], "Aura Field");
    assert!(event.fields["duties"].contains("tend Aura crops"));
}

#[test]
fn elf_centaur_exposes_beach_region_lineage_and_sea_guardianship() {
    let mut service = ConstitutionalApplicationService::new("session.elf").unwrap();
    let run_response = service
        .execute(request("request.elf.run", run("elf-centaur")))
        .unwrap();
    assert!(has_kind(&run_response, TuiEventKind::AuraBeachAssigned));
    assert!(has_kind(&run_response, TuiEventKind::GuardianshipGranted));
    assert!(!has_kind(&run_response, TuiEventKind::AuraFieldsAssigned));
    assert!(!has_kind(&run_response, TuiEventKind::StewardshipGranted));

    let lineage = service
        .execute(request(
            "request.elf.lineage",
            TuiCommand::InspectLineage {
                scenario: "elf-centaur".into(),
            },
        ))
        .unwrap();
    let forms: Vec<_> = lineage
        .events
        .iter()
        .filter(|event| event.kind == TuiEventKind::LineagePreserved)
        .map(|event| event.fields["form"].as_str())
        .collect();
    assert_eq!(forms, ["Elf", "Centaur"]);

    let region = service
        .execute(request(
            "request.elf.region",
            TuiCommand::InspectRegion {
                scenario: "elf-centaur".into(),
            },
        ))
        .unwrap();
    assert_eq!(region.events[0].fields["region"], "Aura Beach");

    let guardianship = service
        .execute(request(
            "request.elf.guardianship",
            TuiCommand::InspectGuardianship {
                scenario: "elf-centaur".into(),
            },
        ))
        .unwrap();
    assert_eq!(
        guardianship.events[0].kind,
        TuiEventKind::GuardianshipGranted
    );
    assert_eq!(guardianship.events[0].fields["region"], "Aura Sea");
    assert!(guardianship.events[0].fields["duties"].contains("guard access to the Aura Sea"));
}

#[test]
fn replay_persistence_and_migration_use_the_selected_owned_archive() {
    let mut service = ConstitutionalApplicationService::new("session.archive").unwrap();
    service
        .execute(request("request.archive.run", run("elf-centaur")))
        .unwrap();

    let replay = service
        .execute(request(
            "request.archive.replay",
            TuiCommand::ReplayScenario {
                scenario: "elf-centaur".into(),
            },
        ))
        .unwrap();
    let completed = replay.events.last().unwrap();
    assert_eq!(completed.kind, TuiEventKind::ReplayCompleted);
    assert_eq!(completed.fields["equivalent"], "Yes");

    let persisted = service
        .execute(request(
            "request.archive.persist",
            TuiCommand::PersistScenario {
                scenario: "elf-centaur".into(),
            },
        ))
        .unwrap();
    assert_eq!(persisted.events[0].kind, TuiEventKind::Persisted);
    assert_eq!(persisted.events[0].fields["canonical"], "Yes");

    let migrated = service
        .execute(request(
            "request.archive.migrate",
            TuiCommand::MigrateScenario {
                scenario: "elf-centaur".into(),
            },
        ))
        .unwrap();
    assert_eq!(
        migrated.events.last().unwrap().kind,
        TuiEventKind::MigrationCompleted
    );
    assert_eq!(migrated.events.last().unwrap().fields["canonical"], "Yes");
}

#[test]
fn reversed_synthesis_is_a_typed_successful_application_rejection() {
    let mut service = ConstitutionalApplicationService::new("session.rejection").unwrap();
    let response = service
        .execute(request("request.reject.run", run("gnome-centaur")))
        .unwrap();
    assert_eq!(
        response.status,
        ApplicationResponseStatus::ConstitutionallyRejected
    );
    let rejected = response
        .events
        .iter()
        .find(|event| event.kind == TuiEventKind::SynthesisRejected)
        .unwrap();
    assert_eq!(
        rejected.fields["failure"],
        "REGIONAL_ILLEGAL_LINEAGE_TRANSITION"
    );
    assert!(!has_kind(&response, TuiEventKind::StateChanged));
    assert_eq!(service.regional_event_count(), 1);
}

#[test]
fn exact_request_retry_is_stable_and_conflicting_reuse_fails_closed() {
    let mut service = ConstitutionalApplicationService::new("session.retry").unwrap();
    let original_request = request("request.same", run("gnome-minotaur"));
    let first = service.execute(original_request.clone()).unwrap();
    let count = service.regional_event_count();
    let second = service.execute(original_request).unwrap();
    assert_eq!(second, first);
    assert_eq!(service.regional_event_count(), count);

    let conflict = service.execute(request("request.same", run("elf-centaur")));
    assert!(matches!(
        conflict,
        Err(ApplicationServiceError::RequestIdConflict(id)) if id == "request.same"
    ));
    assert_eq!(service.selected_scenario(), Some("gnome-minotaur"));
}

#[test]
fn cancellation_occurs_before_a_request_and_never_exposes_partial_state() {
    let mut service = ConstitutionalApplicationService::new("session.cancel").unwrap();
    let cancellation = service
        .execute(request(
            "request.cancel.command",
            TuiCommand::Cancel {
                request_id: "request.cancel.target".into(),
            },
        ))
        .unwrap();
    assert!(has_kind(&cancellation, TuiEventKind::CancellationAccepted));

    let cancelled = service
        .execute(request("request.cancel.target", run("gnome-minotaur")))
        .unwrap();
    assert_eq!(cancelled.status, ApplicationResponseStatus::Cancelled);
    assert!(has_kind(&cancelled, TuiEventKind::RequestCancelled));
    assert_eq!(service.selected_scenario(), None);
    assert_eq!(service.regional_event_count(), 0);
}

#[test]
fn inspections_require_an_explicitly_selected_scenario() {
    let mut service = ConstitutionalApplicationService::new("session.selection").unwrap();
    let result = service.execute(request(
        "request.selection.inspect",
        TuiCommand::InspectLineage {
            scenario: "gnome-minotaur".into(),
        },
    ));
    assert!(matches!(
        result,
        Err(ApplicationServiceError::ScenarioNotSelected(name)) if name == "gnome-minotaur"
    ));
}

#[test]
fn audit_executes_every_catalog_scenario_without_installing_one() {
    let mut service = ConstitutionalApplicationService::new("session.audit").unwrap();
    let response = service
        .execute(request("request.audit.all", TuiCommand::Audit))
        .unwrap();
    let audit = response
        .events
        .iter()
        .find(|event| event.kind == TuiEventKind::AuditCompleted)
        .unwrap();
    assert_eq!(
        audit.fields["scenario_count"],
        SCENARIO_CATALOG.len().to_string()
    );
    assert_eq!(audit.fields["replay_equivalent"], "Yes");
    assert_eq!(audit.fields["persistence_canonical"], "Yes");
    assert_eq!(service.selected_scenario(), None);
}
