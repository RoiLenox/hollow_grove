use hollow_grove::ai_personas::{AiPersona, canonical_returning_goon_case, evaluate_persona};
use hollow_grove::gameplay::{GAMEPLAY_FEDERATION_COMPONENT_ID, GameplayFederationBinding};
use hollow_grove::guide_identity::{DualGuideSession, StableGuideSessionId};
use hollow_grove::kernel_invariants::{
    audit_gameplay_federation_binding, audit_guide_context, central_junction_is_coordination_only,
};

#[test]
fn canonical_federation_binding_passes_kernel_audit() {
    let binding =
        GameplayFederationBinding::canonical(GAMEPLAY_FEDERATION_COMPONENT_ID).with_advisory(
            evaluate_persona(AiPersona::Cleopatra, canonical_returning_goon_case()),
        );
    let report = audit_gameplay_federation_binding(&binding);
    assert!(report.passed(), "{report:?}");
    assert!(report.checked_invariants.len() >= 7);
}

#[test]
fn committed_advisory_fails_kernel_audit() {
    let mut advisory = evaluate_persona(AiPersona::Hal, canonical_returning_goon_case());
    advisory.world_state_committed = true;
    let binding = GameplayFederationBinding::canonical(GAMEPLAY_FEDERATION_COMPONENT_ID)
        .with_advisory(advisory);
    let report = audit_gameplay_federation_binding(&binding);
    assert!(!report.passed());
}

#[test]
fn canonical_guides_pass_oversight_audit() {
    let session_id = StableGuideSessionId::new("session.kernel-audit").unwrap();
    let session = DualGuideSession::inside(session_id);
    let context = session.guide_context_v1().unwrap();
    let report = audit_guide_context(&context);
    assert!(report.passed(), "{report:?}");
    assert!(central_junction_is_coordination_only());
}
