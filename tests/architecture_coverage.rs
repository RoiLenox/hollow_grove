use std::path::Path;

use hollow_grove::architecture_audit::{
    ArchitectureCoverageReport, audit_architecture_links, canonical_architecture_links,
};
use hollow_grove::constitutional::{
    CausalPosition, HouseFunction, ParticipantId, RuleSetId, V2_RULE_SET,
};
use hollow_grove::gameplay::{
    BeingContinuityId, CardinalDirection, GameApplicationService, GameplayCommand, GameplayEventId,
    GameplayEventMetadata,
};
use hollow_grove::institution::InstitutionalBeingId;

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}
fn metadata(at: u64) -> GameplayEventMetadata {
    GameplayEventMetadata {
        id: GameplayEventId::new(format!("event.architecture-check.{at}")).unwrap(),
        causal_position: CausalPosition::new(at),
    }
}
fn identity(suffix: &str) -> GameplayCommand {
    GameplayCommand::EstablishHuemanIdentity {
        continuity: BeingContinuityId::new(format!("continuity.architecture.{suffix}")).unwrap(),
        participant: ParticipantId::new(format!("participant.architecture.{suffix}")).unwrap(),
        institutional: InstitutionalBeingId::new(format!("being.architecture.{suffix}")).unwrap(),
    }
}

#[test]
fn coverage_separates_source_links_bindings_and_execution() {
    let report = ArchitectureCoverageReport::inspect(root(), None).unwrap();
    assert!(report.source_links_passed, "{:?}", report.violations);
    assert_eq!(report.components.len(), 23);
    assert_eq!(report.canonical_route_contracts.len(), 12);
    assert_eq!(report.verified_binding_count, 14);
    assert_eq!(report.unbound_canonical_route_count, 0);
    assert_eq!(report.backend_handoff_binding_count, 10);
    assert_eq!(report.unbound_gameplay_route_count, 10);
    assert!(report.replayed_save_revision.is_none());
    assert!(
        report
            .canonical_route_contracts
            .iter()
            .all(|entry| entry.execution_complete_in_supplied_save.is_none())
    );
    assert!(
        report
            .additional_foundation_contract
            .execution_complete_in_supplied_save
            .is_none()
    );
    let json = serde_json::to_value(report).unwrap();
    assert_eq!(json["schema_version"], 3);
    assert!(
        json["scope"]
            .as_str()
            .unwrap()
            .contains("do not attest execution")
    );
}

#[test]
fn source_audit_rejects_duplicate_ownership_house_substitution_and_escaping_links() {
    let links = canonical_architecture_links();
    let mut duplicate = links.clone();
    duplicate.push(links[0].clone());
    assert!(!audit_architecture_links(root(), &duplicate).passed());
    let mut duplicate_contract = links.clone();
    duplicate_contract[1].authority_source = links[0].authority_source;
    assert!(!audit_architecture_links(root(), &duplicate_contract).passed());
    let mut wrong_house = links.clone();
    wrong_house
        .iter_mut()
        .find(|link| link.id == "architecture.stonebend")
        .unwrap()
        .principal_function = Some(HouseFunction::Recognize);
    assert!(!audit_architecture_links(root(), &wrong_house).passed());
    let mut escaping = links.clone();
    escaping[0].implementation = "../hollow_grove/src/lib.rs";
    assert!(!audit_architecture_links(root(), &escaping).passed());
    let missing_house: Vec<_> = links
        .into_iter()
        .filter(|link| link.id != "architecture.flynt")
        .collect();
    assert!(!audit_architecture_links(root(), &missing_house).passed());
}

#[test]
fn missing_checkout_reports_gaps_without_inventing_execution() {
    let report = ArchitectureCoverageReport::inspect(
        &std::env::temp_dir().join("hollow-grove-missing-architecture-root"),
        None,
    )
    .unwrap();
    assert!(!report.source_links_passed);
    assert!(!report.violations.is_empty());
    assert!(report.replayed_save_revision.is_none());
}

#[test]
fn long_history_retains_retries_atomic_failure_and_replay_bytes() {
    let mut service = GameApplicationService::new(RuleSetId::new(V2_RULE_SET).unwrap());
    service.execute(metadata(1), identity("hueman")).unwrap();
    for at in 2..=1025 {
        service
            .execute(
                metadata(at),
                GameplayCommand::MoveHueman {
                    direction: if at % 2 == 0 {
                        CardinalDirection::East
                    } else {
                        CardinalDirection::West
                    },
                },
            )
            .unwrap();
    }
    let before = service.encode_archive().unwrap();
    let first = service.events()[0].clone();
    assert_eq!(
        service.execute(metadata(1), identity("hueman")).unwrap(),
        &first
    );
    assert_eq!(service.encode_archive().unwrap(), before);
    assert!(service.execute(metadata(1), identity("other")).is_err());
    assert!(service.execute(metadata(1026), identity("other")).is_err());
    assert_eq!(service.encode_archive().unwrap(), before);
    let old_events = service.events().to_vec();
    service
        .execute(
            metadata(1026),
            GameplayCommand::MoveHueman {
                direction: CardinalDirection::North,
            },
        )
        .unwrap();
    assert_eq!(&service.events()[..old_events.len()], old_events);
    let after = service.encode_archive().unwrap();
    let replayed = GameApplicationService::from_archive(&after).unwrap();
    assert_eq!(replayed.encode_archive().unwrap(), after);
    let inspection = ArchitectureCoverageReport::inspect(root(), Some(replayed.runtime())).unwrap();
    assert!(inspection.source_links_passed);
    assert_eq!(inspection.replayed_save_revision, Some(1026));
    assert_eq!(
        inspection
            .additional_foundation_contract
            .execution_complete_in_supplied_save,
        Some(false)
    );
    assert_eq!(replayed.encode_archive().unwrap(), after);
}

#[test]
fn foundation_coverage_requires_actual_rested_repeatable_use() {
    use hollow_grove::gameplay::SympioteAdaptation;
    use hollow_grove::gameplay::SympioteFoundationAction as A;
    let world = hollow_grove::world::session::WorldSession::load_or_canonical_at(root()).unwrap();
    let mut service =
        GameApplicationService::with_world_session(RuleSetId::new(V2_RULE_SET).unwrap(), world);
    service.execute(metadata(1), identity("hueman")).unwrap();
    let actions = [
        A::RequestAssessment,
        A::ConsentToFoundation,
        A::AskClinicianToEstablish,
        A::Rest,
        A::AskClinicianToDischarge,
        A::Practice {
            adaptation: SympioteAdaptation::Balance,
        },
        A::Rest,
        A::Practice {
            adaptation: SympioteAdaptation::Balance,
        },
        A::Rest,
    ];
    for (index, action) in actions.into_iter().enumerate() {
        service
            .execute(
                metadata(index as u64 + 2),
                GameplayCommand::AdvanceSympioteFoundation { action },
            )
            .unwrap();
        let before = service.encode_archive().unwrap();
        let report = ArchitectureCoverageReport::inspect(root(), Some(service.runtime())).unwrap();
        assert_eq!(
            report
                .additional_foundation_contract
                .execution_complete_in_supplied_save,
            Some(index == actions.len() - 1)
        );
        assert_eq!(service.encode_archive().unwrap(), before);
    }
}

#[test]
fn architecture_cli_reads_saves_and_fails_with_a_missing_checkout() {
    use std::process::Command;
    let temp = std::env::temp_dir().join(format!(
        "hollow-grove-architecture-cli-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(&temp).unwrap();
    let mut service = GameApplicationService::new(RuleSetId::new(V2_RULE_SET).unwrap());
    service.execute(metadata(1), identity("hueman")).unwrap();
    let archive = service.encode_archive().unwrap();
    let save = temp.join("save.json");
    std::fs::write(&save, &archive).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_hollow-grove"))
        .args(["architecture", "--json", "--save"])
        .arg(&save)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["replayed_save_revision"], 1);
    assert_eq!(std::fs::read_to_string(&save).unwrap(), archive);
    let failed = Command::new(env!("CARGO_BIN_EXE_hollow-grove"))
        .args(["architecture", "--json", "--source-root"])
        .arg(&temp)
        .output()
        .unwrap();
    assert!(!failed.status.success());
    let report: serde_json::Value = serde_json::from_slice(&failed.stdout).unwrap();
    assert_eq!(report["source_links_passed"], false);
    assert!(!report["violations"].as_array().unwrap().is_empty());
    std::fs::write(
        &save,
        archive.replace(
            "continuity.architecture.hueman",
            "continuity.architecture.forged",
        ),
    )
    .unwrap();
    let tampered = Command::new(env!("CARGO_BIN_EXE_hollow-grove"))
        .args(["architecture", "--json", "--save"])
        .arg(&save)
        .output()
        .unwrap();
    assert!(!tampered.status.success());
    assert!(tampered.stdout.is_empty());
    std::fs::remove_dir_all(temp).unwrap();
}
