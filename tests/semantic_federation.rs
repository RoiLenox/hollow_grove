use std::future::Future;
use std::task::{Context, Poll, Waker};

use hollow_grove::composition::{
    CompositionCatalog, CompositionNode, CompositionNodeId, ExternalRef, Position3, ScaleKey,
    SemanticScope,
};
use hollow_grove::constitutional::{GroveCycleId, GrovePhase};
use hollow_grove::runtime_federation::{
    CatalogRoiContinuity, DeterministicSemanticInterpreter, FederationError,
    FederationOrchestrator, FederationRequest, KernelSemanticConstitution, ModelSemanticProposal,
    ProposedSemanticOperation, ReadOnlyRosalinaWorldState, RoiContinuity, RosalinaWorldState,
    SemanticConstitution, SemanticDecision, SemanticProposal, SemanticValidationError,
    ValidationReport, WayBackContext,
};

fn block_on<F: Future>(future: F) -> F::Output {
    let waker = Waker::noop();
    let mut context = Context::from_waker(waker);
    let mut future = std::pin::pin!(future);
    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(output) => return output,
            Poll::Pending => std::thread::yield_now(),
        }
    }
}

fn reference(namespace: &str, key: &str) -> ExternalRef {
    ExternalRef::new(namespace, key).unwrap()
}

fn scope() -> SemanticScope {
    SemanticScope::new(
        ScaleKey::new("structural-repair").unwrap(),
        Some(reference("context", "inspection.session.041")),
    )
}

fn catalog() -> CompositionCatalog {
    let mut catalog = CompositionCatalog::new();
    catalog
        .insert_node(CompositionNode {
            id: CompositionNodeId::new("node.point.support-column-041").unwrap(),
            object: reference("point", "object.support-column.041"),
            scale: ScaleKey::new("scale.point").unwrap(),
        })
        .unwrap();
    catalog
}

fn request(intent: &str) -> FederationRequest {
    FederationRequest {
        actor_ref: reference("actor", "repairer.041"),
        point_ref: reference("point", "object.support-column.041"),
        scope: scope(),
        user_intent: intent.into(),
        prompt_version: "hollow-grove-semantic-interpreter-v1".into(),
        evidence_refs: vec![reference("evidence", "inspection.041")],
        constitutional_refs: vec![reference("constitution", "semantic-kernel-v1")],
        authority_refs: vec![reference("authority", "repairer.041")],
        unresolved_condition_refs: vec![],
        world_state_refs: vec![reference("world-state", "support-column.041")],
        nearby_semantic_refs: vec![],
        retrieved_canon: vec![],
        nearby_examples: vec![],
    }
}

fn proposal(position: Position3) -> ModelSemanticProposal {
    ModelSemanticProposal {
        scale_key: "structural-repair".into(),
        proposed_position: position,
        operation: ProposedSemanticOperation::Repair,
        evidence_refs: vec!["evidence.inspection.041".into()],
        semantic_tags: vec!["support".into(), "repair".into(), "embodiment".into()],
        summary: "The request repairs damaged structure into embodied support.".into(),
        confidence_millis: 870,
    }
}

fn orchestrator(
    model: ModelSemanticProposal,
) -> FederationOrchestrator<
    CatalogRoiContinuity,
    ReadOnlyRosalinaWorldState,
    DeterministicSemanticInterpreter,
    KernelSemanticConstitution,
> {
    FederationOrchestrator::new(
        CatalogRoiContinuity,
        ReadOnlyRosalinaWorldState,
        DeterministicSemanticInterpreter::responding_with(model),
        KernelSemanticConstitution,
    )
}

#[test]
fn successful_cycle_uses_every_public_phase_and_returns_next_way_back_state() {
    let mut runtime = catalog();
    let position = Position3::from_raw(-80, 0, -310).unwrap();
    let result = block_on(orchestrator(proposal(position)).run_cycle(
        request("Locate the damaged supporting frame."),
        &mut runtime,
    ))
    .unwrap();

    assert_eq!(result.phase_trace, GrovePhase::ALL);
    let SemanticDecision::Confirmed(confirmed) = result.festival.decision else {
        panic!("lawful proposal must be confirmed");
    };
    assert_eq!(confirmed.confirmed_position, position);
    assert_eq!(result.next_way_back.current_position, Some(position));
    assert_eq!(runtime.semantic_position_records().len(), 1);
    assert_eq!(runtime.composition_records().len(), 1);
    assert_eq!(runtime.semantic_witnesses().len(), 1);
}

#[test]
fn complete_translation_cycle_preserves_identity_and_returns_confirmed_movement() {
    let mut runtime = catalog();
    let initial = Position3::from_raw(-80, 0, -310).unwrap();
    block_on(orchestrator(proposal(initial)).run_cycle(
        request("Locate the damaged supporting frame."),
        &mut runtime,
    ))
    .unwrap();
    let destination = Position3::from_raw(-120, 0, 580).unwrap();
    let result = block_on(orchestrator(proposal(destination)).run_cycle(
        request("Restore the broken supporting frame."),
        &mut runtime,
    ))
    .unwrap();
    let SemanticDecision::Confirmed(confirmed) = result.festival.decision else {
        panic!("lawful translation must be confirmed");
    };
    assert_eq!(confirmed.previous_position, Some(initial));
    assert_eq!(
        confirmed.translation.unwrap(),
        initial.translation_to(destination).unwrap()
    );
    assert_eq!(result.next_way_back.current_position, Some(destination));
    assert_eq!(
        runtime.nodes_for_object(&reference("point", "object.support-column.041"))[0].id,
        CompositionNodeId::new("node.point.support-column-041").unwrap()
    );
}

#[test]
fn missing_evidence_is_gathered_as_a_rejection_without_canonical_mutation() {
    let mut runtime = catalog();
    let before_nodes = runtime.nodes_for_object(&reference("point", "object.support-column.041"));
    assert_eq!(before_nodes.len(), 1);
    let mut request = request("Repair without an inspection.");
    request.evidence_refs.clear();
    let result = block_on(
        orchestrator(proposal(Position3::from_raw(0, 0, 400).unwrap()))
            .run_cycle(request, &mut runtime),
    )
    .unwrap();
    assert_eq!(result.phase_trace, GrovePhase::ALL);
    let SemanticDecision::Rejected(rejected) = result.festival.decision else {
        panic!("unsupported proposal must be rejected");
    };
    assert!(rejected.reasons.iter().any(|reason| {
        format!("{reason:?}").contains("UnknownEvidence")
            || format!("{reason:?}").contains("InsufficientEvidence")
    }));
    assert!(runtime.semantic_position_records().is_empty());
    assert!(runtime.composition_records().is_empty());
    assert_eq!(runtime.semantic_witnesses().len(), 1);
}

#[derive(Debug, Clone, Copy)]
struct StaleRoi;

impl RoiContinuity for StaleRoi {
    fn restore_history(
        &self,
        request: &FederationRequest,
        runtime: &CompositionCatalog,
        cycle_id: GroveCycleId,
    ) -> Result<WayBackContext, FederationError> {
        let mut context = CatalogRoiContinuity.restore_history(request, runtime, cycle_id)?;
        context.current_position = Some(Position3::ORIGIN);
        context.current_record_id = None;
        Ok(context)
    }

    fn resolve_causality(
        &self,
        point_ref: &ExternalRef,
        scope: &SemanticScope,
        runtime: &CompositionCatalog,
    ) -> Vec<ExternalRef> {
        CatalogRoiContinuity.resolve_causality(point_ref, scope, runtime)
    }

    fn verify_succession(
        &self,
        proposal: &SemanticProposal,
        context: &WayBackContext,
    ) -> Result<(), SemanticValidationError> {
        CatalogRoiContinuity.verify_succession(proposal, context)
    }

    fn prepare_replay(&self, proposal: &SemanticProposal) -> Result<(), SemanticValidationError> {
        CatalogRoiContinuity.prepare_replay(proposal)
    }
}

#[test]
fn festival_rechecks_source_and_rejects_stale_state() {
    let mut runtime = catalog();
    let destination = Position3::from_raw(100, 0, 500).unwrap();
    let runtime_cycle = FederationOrchestrator::new(
        StaleRoi,
        ReadOnlyRosalinaWorldState,
        DeterministicSemanticInterpreter::responding_with(proposal(destination)),
        KernelSemanticConstitution,
    );
    let result =
        block_on(runtime_cycle.run_cycle(request("Use an obsolete source state."), &mut runtime))
            .unwrap();
    let SemanticDecision::Rejected(rejected) = result.festival.decision else {
        panic!("stale proposal must be rejected");
    };
    assert!(
        rejected
            .reasons
            .iter()
            .any(|reason| format!("{reason:?}").contains("StaleSourcePosition"))
    );
    assert!(runtime.semantic_position_records().is_empty());
}

#[derive(Debug, Clone, Copy)]
struct DenyingConstitution;

impl SemanticConstitution for DenyingConstitution {
    fn validate(
        &self,
        _proposal: &SemanticProposal,
        _context: &WayBackContext,
    ) -> ValidationReport {
        ValidationReport {
            errors: vec![SemanticValidationError::ConstitutionalViolation(
                "test boundary".into(),
            )],
            warnings: vec![],
        }
    }
}

#[test]
fn constitutional_rejection_reaches_festival_but_never_changes_position() {
    let mut runtime = catalog();
    let runtime_cycle = FederationOrchestrator::new(
        CatalogRoiContinuity,
        ReadOnlyRosalinaWorldState,
        DeterministicSemanticInterpreter::responding_with(proposal(Position3::ORIGIN)),
        DenyingConstitution,
    );
    let result =
        block_on(runtime_cycle.run_cycle(request("Attempt an unlawful placement."), &mut runtime))
            .unwrap();
    assert_eq!(result.phase_trace, GrovePhase::ALL);
    assert!(matches!(
        result.festival.decision,
        SemanticDecision::Rejected(_)
    ));
    assert!(runtime.semantic_position_records().is_empty());
    assert!(runtime.composition_records().is_empty());
}

#[test]
fn initiation_failure_and_interpreter_unavailability_mutate_nothing() {
    for interpreter in [
        DeterministicSemanticInterpreter::failing_with(
            hollow_grove::runtime_federation::InterpreterError::MalformedModelResponse(
                "bad json".into(),
            ),
        ),
        DeterministicSemanticInterpreter::timeout(),
        DeterministicSemanticInterpreter::unavailable(),
    ] {
        let mut runtime = catalog();
        let runtime_cycle = FederationOrchestrator::new(
            CatalogRoiContinuity,
            ReadOnlyRosalinaWorldState,
            interpreter,
            KernelSemanticConstitution,
        );
        assert!(
            block_on(runtime_cycle.run_cycle(request("Interpret this."), &mut runtime)).is_err()
        );
        assert!(runtime.semantic_position_records().is_empty());
        assert!(runtime.composition_records().is_empty());
        assert!(runtime.semantic_witnesses().is_empty());
    }
}

#[derive(Debug, Clone, Copy)]
struct FailingPersistRosalina;

impl RosalinaWorldState for FailingPersistRosalina {
    fn inspect_world_state(
        &self,
        request: &FederationRequest,
    ) -> Result<Vec<ExternalRef>, FederationError> {
        Ok(request.world_state_refs.clone())
    }

    fn validate_embodiment(
        &self,
        _proposal: &SemanticProposal,
        _context: &WayBackContext,
    ) -> ValidationReport {
        ValidationReport::valid()
    }

    fn execute_confirmed_effect(
        &self,
        _proposal: &SemanticProposal,
    ) -> Result<Vec<ExternalRef>, FederationError> {
        Ok(vec![reference("world-result", "prepared.041")])
    }

    fn persist_world_result(&self, _result_refs: &[ExternalRef]) -> Result<(), FederationError> {
        Err(FederationError::WorldState(
            "simulated write failure".into(),
        ))
    }
}

#[test]
fn festival_catalog_commit_is_rolled_back_when_world_persistence_fails() {
    let mut runtime = catalog();
    let runtime_cycle = FederationOrchestrator::new(
        CatalogRoiContinuity,
        FailingPersistRosalina,
        DeterministicSemanticInterpreter::responding_with(proposal(Position3::ORIGIN)),
        KernelSemanticConstitution,
    );
    assert!(
        block_on(runtime_cycle.run_cycle(request("Prepare a failing effect."), &mut runtime))
            .is_err()
    );
    assert!(runtime.semantic_position_records().is_empty());
    assert!(runtime.composition_records().is_empty());
    assert!(runtime.semantic_witnesses().is_empty());
}

#[test]
fn deterministic_interpreter_and_runtime_ids_are_repeatable() {
    let model = proposal(Position3::from_raw(-250, 0, 600).unwrap());
    let mut first = catalog();
    let first_result =
        block_on(orchestrator(model.clone()).run_cycle(request("Place the overlap."), &mut first))
            .unwrap();
    let mut second = catalog();
    let second_result =
        block_on(orchestrator(model).run_cycle(request("Place the overlap."), &mut second))
            .unwrap();
    assert_eq!(first_result.festival, second_result.festival);
    assert_eq!(
        first.semantic_position_records(),
        second.semantic_position_records()
    );
    assert_eq!(first.semantic_witnesses(), second.semantic_witnesses());
}
