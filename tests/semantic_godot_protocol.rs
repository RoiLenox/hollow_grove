use hollow_grove::constitutional::GroveCycleId;
use hollow_grove::runtime_federation::{
    ConfirmedSemanticTransition, FestivalResult, ProposalStatus, ProposedSemanticOperation,
    RejectedSemanticProposal, SemanticDecision, SemanticProposal, SemanticRejectionReason,
    SemanticValidationError,
};
use hollow_grove::{
    ExternalRef, Position3, SEMANTIC_EVENT_SCHEMA_VERSION, ScaleKey, SemanticGodotEvent,
    SemanticGodotEventEnvelope, SemanticScope, decode_semantic_godot_event,
    semantic_festival_events, semantic_proposal_events,
};

fn reference(namespace: &str, key: &str) -> ExternalRef {
    ExternalRef::new(namespace, key).unwrap()
}

fn scope() -> SemanticScope {
    SemanticScope::new(ScaleKey::new("structural-repair").unwrap(), None)
}

fn translation_proposal() -> SemanticProposal {
    let from = Position3::from_raw(-80, 0, -310).unwrap();
    let to = Position3::from_raw(-120, 0, 580).unwrap();
    SemanticProposal {
        proposal_id: reference("proposal", "semantic.repair.041"),
        cycle_id: GroveCycleId::new("cycle.semantic.repair.041").unwrap(),
        actor_ref: reference("actor", "repairer.041"),
        point_ref: reference("point", "object.support-column.041"),
        scope: scope(),
        previous_position: Some(from),
        proposed_position: to,
        translation: Some(from.translation_to(to).unwrap()),
        operation: ProposedSemanticOperation::Repair,
        evidence_refs: vec![reference("evidence", "inspection.041")],
        semantic_tags: vec!["repair".into()],
        summary: "Restore embodied support.".into(),
        confidence_millis: 870,
        status: ProposalStatus::Gathering,
    }
}

#[test]
fn proposed_translation_is_explicitly_unconfirmed() {
    let events = semantic_proposal_events(&translation_proposal()).unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].schema_version, SEMANTIC_EVENT_SCHEMA_VERSION);
    match &events[0].event {
        SemanticGodotEvent::SemanticTranslationProposed {
            status,
            translation,
            ..
        } => {
            assert_eq!(*status, ProposalStatus::Proposed);
            assert_eq!(translation.dy().get(), 0);
        }
        other => panic!("unexpected event: {other:?}"),
    }
}

#[test]
fn confirmed_translation_emits_crossing_and_festival_events() {
    let proposal = translation_proposal();
    let decision = ConfirmedSemanticTransition {
        proposal_id: proposal.proposal_id.clone(),
        point_ref: proposal.point_ref.clone(),
        scope: proposal.scope.clone(),
        previous_position: proposal.previous_position,
        confirmed_position: proposal.proposed_position,
        translation: proposal.translation,
        composition_ref: reference("composition-record", "semantic.repair.041"),
        witness_ref: reference("witness", "semantic.repair.041"),
    };
    let festival = FestivalResult {
        decision: SemanticDecision::Confirmed(decision),
        next_state_refs: vec![reference("semantic-state", "repair.041")],
    };
    let events = semantic_festival_events(&proposal, &festival).unwrap();
    assert_eq!(events.len(), 3);
    assert!(matches!(
        events[0].event,
        SemanticGodotEvent::SemanticTranslationConfirmed {
            crossed_vertical_origin: true,
            ..
        }
    ));
    assert!(matches!(
        events[1].event,
        SemanticGodotEvent::CentralJunctionCrossed { .. }
    ));
    assert!(matches!(
        events[2].event,
        SemanticGodotEvent::FestivalCompleted {
            status: ProposalStatus::Confirmed,
            ..
        }
    ));
}

#[test]
fn rejection_projects_reasons_without_presenting_canonical_mutation() {
    let proposal = translation_proposal();
    let festival = FestivalResult {
        decision: SemanticDecision::Rejected(RejectedSemanticProposal {
            proposal_id: proposal.proposal_id.clone(),
            point_ref: proposal.point_ref.clone(),
            scope: proposal.scope.clone(),
            proposed_position: proposal.proposed_position,
            reasons: vec![SemanticRejectionReason::Validation(
                SemanticValidationError::InsufficientEvidence,
            )],
            witness_ref: reference("witness", "semantic.rejected.041"),
        }),
        next_state_refs: vec![],
    };
    let events = semantic_festival_events(&proposal, &festival).unwrap();
    assert_eq!(events.len(), 2);
    match &events[0].event {
        SemanticGodotEvent::SemanticTranslationRejected { reasons, .. } => {
            assert!(reasons[0].contains("InsufficientEvidence"));
        }
        other => panic!("unexpected event: {other:?}"),
    }
    assert!(matches!(
        events[1].event,
        SemanticGodotEvent::FestivalCompleted {
            status: ProposalStatus::Rejected,
            ..
        }
    ));
}

#[test]
fn event_json_is_strict_and_round_trips() {
    let event = semantic_proposal_events(&translation_proposal())
        .unwrap()
        .remove(0);
    let bytes = serde_json::to_vec(&event).unwrap();
    let decoded = decode_semantic_godot_event(&bytes).unwrap();
    assert_eq!(event, decoded);

    let mut value = serde_json::to_value(&event).unwrap();
    value["unexpected"] = true.into();
    assert!(serde_json::from_value::<SemanticGodotEventEnvelope>(value).is_err());

    let mut value = serde_json::to_value(&event).unwrap();
    value["schema_version"] = "hollow-grove-semantic-event-v2".into();
    assert!(decode_semantic_godot_event(&serde_json::to_vec(&value).unwrap()).is_err());
}

#[test]
fn mismatched_decision_identity_is_not_projected() {
    let proposal = translation_proposal();
    let festival = FestivalResult {
        decision: SemanticDecision::Confirmed(ConfirmedSemanticTransition {
            proposal_id: reference("proposal", "different"),
            point_ref: proposal.point_ref.clone(),
            scope: proposal.scope.clone(),
            previous_position: proposal.previous_position,
            confirmed_position: proposal.proposed_position,
            translation: proposal.translation,
            composition_ref: reference("composition-record", "semantic.repair.041"),
            witness_ref: reference("witness", "semantic.repair.041"),
        }),
        next_state_refs: vec![],
    };
    assert!(semantic_festival_events(&proposal, &festival).is_err());
}
