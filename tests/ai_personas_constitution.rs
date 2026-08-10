use hollow_grove::ai_personas::{
    AdvisoryStatus, AiPersona, PersonaAdvisoryRecord, canonical_returning_goon_case,
    encode_advisory, evaluate_persona, replay_advisory,
};
use hollow_grove::constitutional::{RuleSetId, V2_RULE_SET};
use hollow_grove::gameplay::{
    BoardwalkCase, GameplayFederationBinding, decode_gameplay_archive_with_metadata,
    encode_gameplay_archive_with_federation_binding,
};
use hollow_grove::world::session::WorldSession;

#[test]
fn returning_goon_fixture_keeps_cleopatra_and_hal_advisory() {
    let input = canonical_returning_goon_case();
    let cleopatra = evaluate_persona(AiPersona::Cleopatra, input.clone());
    let hal = evaluate_persona(AiPersona::Hal, input);

    assert_eq!(cleopatra.status, AdvisoryStatus::AcceptedAdvisory);
    assert_eq!(hal.status, AdvisoryStatus::AcceptedAdvisory);
    assert_eq!(cleopatra.associated_title, "Gimp of the Underway");
    assert_eq!(hal.associated_title, "Pimp of the Boardwalk");
    assert!(!cleopatra.world_state_committed);
    assert!(!hal.world_state_committed);
    assert!(cleopatra.next_handoff.contains("Stonebend"));
    assert!(hal.next_handoff.contains("Flynt"));
}

#[test]
fn clouseau_is_the_pleb_pleb_witness_without_bond_authority() {
    let record = evaluate_persona(AiPersona::Clouseau, canonical_returning_goon_case());
    assert_eq!(record.associated_title, "PLEB-PLEB witness");
    assert!(record.finding.contains("does not create"));
    assert!(record.next_handoff.contains("Current Synthesis"));
    assert!(!record.world_state_committed);
}

#[test]
fn advisory_records_round_trip_and_replay_exactly() {
    for persona in [AiPersona::Cleopatra, AiPersona::Hal, AiPersona::Clouseau] {
        let record = evaluate_persona(persona, canonical_returning_goon_case());
        let bytes = encode_advisory(&record).unwrap();
        let replayed = replay_advisory(&bytes).unwrap();
        assert_eq!(record, replayed);
        assert_eq!(encode_advisory(&replayed).unwrap(), bytes);
    }
}

#[test]
fn tampering_or_attempted_commit_fails_closed() {
    let record = evaluate_persona(AiPersona::Hal, canonical_returning_goon_case());
    let mut value: serde_json::Value =
        serde_json::from_slice(&encode_advisory(&record).unwrap()).unwrap();
    value["finding"] = serde_json::Value::String("Hal grants the Pimp title".into());
    let tampered = serde_json::to_vec(&value).unwrap();
    assert!(replay_advisory(&tampered).is_err());

    let mut committed: PersonaAdvisoryRecord = record;
    committed.world_state_committed = true;
    let committed_bytes = encode_advisory(&committed).unwrap();
    assert!(replay_advisory(&committed_bytes).is_err());
}

#[test]
fn advisory_records_federate_through_the_existing_gameplay_component() {
    let rule_set = RuleSetId::new(V2_RULE_SET).unwrap();
    let cleopatra = evaluate_persona(AiPersona::Cleopatra, canonical_returning_goon_case());
    let hal = evaluate_persona(AiPersona::Hal, canonical_returning_goon_case());
    let binding = GameplayFederationBinding::canonical(
        hollow_grove::gameplay::GAMEPLAY_FEDERATION_COMPONENT_ID,
    )
    .with_advisory(cleopatra)
    .with_advisory(hal)
    .with_shared_interface_record(hollow_grove::ai_personas::SharedInterfaceRecord {
        kind: hollow_grove::ai_personas::SharedInterfaceRecordKind::Restitution,
        record_id: "restitution.boardwalk.returning-goon.v1".into(),
        subject_id: "being.boardwalk.returning-goon.v1".into(),
        authority_id: "authority.minoan-county-court.v1".into(),
        status: "equilibrium-confirmed".into(),
        causal_position: 48,
        provenance_id: "provenance.restitution.boardwalk.returning-goon.v1".into(),
    });
    let archive = encode_gameplay_archive_with_federation_binding(
        &rule_set,
        &[],
        &WorldSession::canonical(),
        binding,
    )
    .unwrap();
    let decoded = decode_gameplay_archive_with_metadata(&archive).unwrap();
    assert_eq!(decoded.federation_binding.advisory_records.len(), 2);
    assert_eq!(decoded.federation_binding.shared_interface_records.len(), 1);
    assert!(
        decoded
            .federation_binding
            .advisory_records
            .iter()
            .all(|record| !record.world_state_committed)
    );
}

#[test]
fn returning_goon_case_supplies_the_same_advisories_to_federation() {
    let case = BoardwalkCase::new();
    let records = case.constitutional_advisories();
    assert_eq!(records.len(), 3);
    assert_eq!(records[0].persona, AiPersona::Cleopatra);
    assert_eq!(records[1].persona, AiPersona::Hal);
    assert_eq!(records[2].persona, AiPersona::Clouseau);
}
