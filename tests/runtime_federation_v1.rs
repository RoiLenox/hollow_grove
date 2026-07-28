use hollow_grove::constitutional::{RuleSetId, V2_RULE_SET};
use hollow_grove::gameplay::{
    GAMEPLAY_ARCHIVE_SCHEMA_VERSION, GAMEPLAY_FEDERATION_COMPONENT_ID, GameApplicationService,
    decode_gameplay_archive_with_metadata, encode_legacy_gameplay_archive_v2,
    migrate_gameplay_archive,
};
use hollow_grove::runtime_federation::{
    FederationComponentKind, RUNTIME_FEDERATION_ARCHIVE_FORMAT,
    RUNTIME_FEDERATION_ARCHIVE_IDENTITY, RUNTIME_FEDERATION_ARCHIVE_VERSION,
    RUNTIME_FEDERATION_CANONICAL_NAME, RUNTIME_FEDERATION_IDENTITY, RuntimeFederation,
    RuntimeFederationError, component_digest, decode_runtime_federation_archive,
    encode_runtime_federation_archive, migrate_runtime_federation_archive,
};
use hollow_grove::runtime_federation_fixture::{
    FEDERATION_GAMEPLAY_COMPONENT_ID, FEDERATION_KERNEL_COMPONENT_ID,
    canonical_runtime_federation_payload,
};
use hollow_grove::world::session::WorldSession;

#[test]
fn canonical_fixture_makes_the_formal_name_executable() {
    let payload = canonical_runtime_federation_payload().unwrap();
    let runtime = RuntimeFederation::replay(&payload).unwrap();
    let manifest = runtime.manifest();
    assert_eq!(manifest.canonical_name, RUNTIME_FEDERATION_CANONICAL_NAME);
    assert_eq!(manifest.canonical_name, "The Runtime Federation");
    assert_eq!(manifest.federation_identity, RUNTIME_FEDERATION_IDENTITY);
    assert_eq!(
        manifest.archive_identity,
        RUNTIME_FEDERATION_ARCHIVE_IDENTITY
    );
    assert_eq!(manifest.archive_version, RUNTIME_FEDERATION_ARCHIVE_VERSION);
    assert_eq!(
        runtime.component_count(),
        FederationComponentKind::REQUIRED.len()
    );
    for kind in FederationComponentKind::REQUIRED {
        assert!(
            manifest
                .components
                .iter()
                .any(|component| component.kind == kind),
            "missing {}",
            kind.display_name()
        );
    }
}

#[test]
fn hgrf_encoding_replays_every_component_and_is_deterministic() {
    let payload = canonical_runtime_federation_payload().unwrap();
    let first = encode_runtime_federation_archive(&payload).unwrap();
    let (decoded, runtime) = decode_runtime_federation_archive(&first).unwrap();
    assert_eq!(
        runtime.component_count(),
        FederationComponentKind::REQUIRED.len()
    );
    assert_eq!(
        runtime
            .component(FEDERATION_KERNEL_COMPONENT_ID)
            .unwrap()
            .component_id,
        FEDERATION_KERNEL_COMPONENT_ID
    );
    assert_eq!(encode_runtime_federation_archive(&decoded).unwrap(), first);
    assert_eq!(migrate_runtime_federation_archive(&first).unwrap(), first);

    let mut reversed = payload;
    reversed.manifest.components.reverse();
    reversed.manifest.events.reverse();
    reversed.component_archives.reverse();
    assert_eq!(encode_runtime_federation_archive(&reversed).unwrap(), first);

    let envelope: serde_json::Value = serde_json::from_slice(&first).unwrap();
    assert_eq!(envelope["format"], RUNTIME_FEDERATION_ARCHIVE_FORMAT);
    assert_eq!(
        envelope["archive_version"],
        RUNTIME_FEDERATION_ARCHIVE_VERSION
    );
}

#[test]
fn component_tampering_and_dependency_contradictions_fail_closed() {
    let mut tampered = canonical_runtime_federation_payload().unwrap();
    tampered
        .component_archives
        .iter_mut()
        .find(|archive| archive.component_id == FEDERATION_KERNEL_COMPONENT_ID)
        .unwrap()
        .archive_bytes
        .push(b'!');
    assert!(matches!(
        RuntimeFederation::replay(&tampered),
        Err(RuntimeFederationError::ComponentDigestMismatch { .. })
    ));

    let mut checksum_only = canonical_runtime_federation_payload().unwrap();
    let kernel_archive = checksum_only
        .component_archives
        .iter_mut()
        .find(|archive| archive.component_id == FEDERATION_KERNEL_COMPONENT_ID)
        .unwrap();
    let mut kernel: serde_json::Value =
        serde_json::from_slice(&kernel_archive.archive_bytes).unwrap();
    kernel["federation_aware"] = true.into();
    kernel_archive.archive_bytes = serde_json::to_vec(&kernel).unwrap();
    checksum_only
        .manifest
        .components
        .iter_mut()
        .find(|component| component.component_id == FEDERATION_KERNEL_COMPONENT_ID)
        .unwrap()
        .digest = component_digest(&kernel_archive.archive_bytes);
    checksum_only.manifest.refresh_aggregate_digest().unwrap();
    assert!(matches!(
        RuntimeFederation::replay(&checksum_only),
        Err(RuntimeFederationError::ComponentReplay { .. })
    ));

    let mut missing = canonical_runtime_federation_payload().unwrap();
    missing
        .manifest
        .components
        .iter_mut()
        .find(|component| component.component_id == FEDERATION_GAMEPLAY_COMPONENT_ID)
        .unwrap()
        .dependencies
        .push("component.missing".into());
    missing.manifest.refresh_aggregate_digest().unwrap();
    assert!(matches!(
        RuntimeFederation::replay(&missing),
        Err(RuntimeFederationError::MissingDependency { .. })
    ));

    let mut cyclical = canonical_runtime_federation_payload().unwrap();
    cyclical
        .manifest
        .components
        .iter_mut()
        .find(|component| component.component_id == FEDERATION_KERNEL_COMPONENT_ID)
        .unwrap()
        .dependencies
        .push(FEDERATION_GAMEPLAY_COMPONENT_ID.into());
    cyclical.manifest.refresh_aggregate_digest().unwrap();
    assert!(matches!(
        RuntimeFederation::replay(&cyclical),
        Err(RuntimeFederationError::CyclicalDependency(_))
    ));

    let mut duplicate = canonical_runtime_federation_payload().unwrap();
    let dependency = duplicate
        .manifest
        .components
        .iter()
        .find(|component| component.component_id == FEDERATION_GAMEPLAY_COMPONENT_ID)
        .unwrap()
        .dependencies[0]
        .clone();
    duplicate
        .manifest
        .components
        .iter_mut()
        .find(|component| component.component_id == FEDERATION_GAMEPLAY_COMPONENT_ID)
        .unwrap()
        .dependencies
        .push(dependency);
    duplicate.manifest.refresh_aggregate_digest().unwrap();
    assert!(matches!(
        RuntimeFederation::replay(&duplicate),
        Err(RuntimeFederationError::DuplicateDependency(_))
    ));
}

#[test]
fn hgrf_envelope_checksum_detects_manifest_tampering() {
    let bytes = encode_runtime_federation_archive(&canonical_runtime_federation_payload().unwrap())
        .unwrap();
    let mut envelope: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    envelope["payload"]["manifest"]["confirmed_state_id"] = "state.tampered".into();
    let tampered = serde_json::to_vec(&envelope).unwrap();
    assert!(matches!(
        decode_runtime_federation_archive(&tampered),
        Err(RuntimeFederationError::ChecksumMismatch { .. })
    ));
}

#[test]
fn rejected_history_cannot_falsely_mutate_the_confirmed_state() {
    let payload = canonical_runtime_federation_payload().unwrap();
    let runtime = RuntimeFederation::replay(&payload).unwrap();
    let manifest = runtime.manifest();
    let proof = &manifest.first_playable_proof;
    let rejected = runtime.event(&proof.rejected_attempt_event_id).unwrap();
    let accepted = runtime.event(&proof.accepted_result_event_id).unwrap();
    assert!(!rejected.accepted);
    assert!(rejected.result_state_id.is_none());
    assert!(!rejected.evidence_ids.is_empty());
    assert!(accepted.accepted);
    assert_eq!(
        accepted.result_state_id.as_deref(),
        Some(manifest.confirmed_state_id.as_str())
    );
    assert_eq!(manifest.confirmed_state_id, manifest.next_way_back_state_id);
    assert_ne!(
        manifest.prior_confirmed_state_id,
        manifest.confirmed_state_id
    );
}

#[test]
fn first_playable_proof_covers_the_complete_contained_operation() {
    let payload = canonical_runtime_federation_payload().unwrap();
    let runtime = RuntimeFederation::replay(&payload).unwrap();
    let proof = &runtime.manifest().first_playable_proof;
    assert_eq!(proof.phase_records.len(), 4);
    assert_eq!(
        proof
            .phase_records
            .iter()
            .map(|record| record.phase)
            .collect::<Vec<_>>(),
        hollow_grove::constitutional::GrovePhase::ALL
    );
    assert_eq!(proof.permanence_proof_ids.len(), 4);
    assert!(proof.nonlethal);
    assert!(proof.presentation_reads_only);
    assert!(
        !runtime
            .event(&proof.cross_domain_event_id)
            .unwrap()
            .caused_by
            .is_empty()
    );
    assert!(!runtime.manifest().transfers_sovereignty);
    assert!(!runtime.manifest().presentation_authoritative);
    let view = runtime.first_playable_view();
    assert_eq!(view.operation_id, proof.operation_id);
    assert_eq!(
        view.accepted_state_id,
        runtime.manifest().confirmed_state_id
    );
    assert_eq!(view.rejected_attempt_count, 1);
    assert!(view.real_emergency_recorded);
    assert!(!view.presentation_may_mutate);
}

#[test]
fn read_only_subject_and_evidence_indexes_do_not_create_authority() {
    let runtime =
        RuntimeFederation::replay(&canonical_runtime_federation_payload().unwrap()).unwrap();
    let subject_events = runtime.events_for_subject("subject.central-junction.four-house-bridge");
    assert_eq!(
        subject_events.len(),
        runtime.manifest().events.len(),
        "every contained-operation event retains the same subject"
    );
    let event = runtime
        .event("federation-event.operation.real-emergency")
        .unwrap();
    assert_eq!(
        runtime.events_for_evidence(&event.evidence_ids[0]),
        &["federation-event.operation.real-emergency".to_owned()]
    );
}

#[test]
fn gameplay_schema_v3_is_federation_aware_and_v2_migration_is_idempotent() {
    let rule_set = RuleSetId::new(V2_RULE_SET).unwrap();
    let application = GameApplicationService::new(rule_set.clone());
    let encoded = application.encode_archive().unwrap();
    let decoded = decode_gameplay_archive_with_metadata(&encoded).unwrap();
    assert_eq!(
        decoded.source_schema_version,
        GAMEPLAY_ARCHIVE_SCHEMA_VERSION
    );
    assert_eq!(
        decoded.federation_binding.federation_identity,
        RUNTIME_FEDERATION_IDENTITY
    );
    assert_eq!(
        decoded.federation_binding.component_id,
        GAMEPLAY_FEDERATION_COMPONENT_ID
    );
    assert!(decoded.migration_history.is_empty());
    assert_eq!(migrate_gameplay_archive(&encoded).unwrap(), encoded);

    let v2 = encode_legacy_gameplay_archive_v2(
        &rule_set,
        application.events(),
        &WorldSession::canonical(),
    )
    .unwrap();
    let migrated = migrate_gameplay_archive(&v2).unwrap();
    let migrated_decoded = decode_gameplay_archive_with_metadata(&migrated).unwrap();
    assert_eq!(
        migrated_decoded.source_schema_version,
        GAMEPLAY_ARCHIVE_SCHEMA_VERSION
    );
    assert_eq!(migrated_decoded.events, decoded.events);
    assert_eq!(migrated_decoded.migration_history.len(), 1);
    assert_eq!(
        migrated_decoded.migration_history[0].migration_id,
        "migration.gameplay.v2-to-v3.runtime-federation"
    );
    assert_eq!(migrate_gameplay_archive(&migrated).unwrap(), migrated);
}
