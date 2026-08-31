use std::error::Error;
use std::io;

use hollow_grove::composition::ExternalRef;
use hollow_grove::guide_identity::{
    DualGuideSession, GuideTransitionDirection, GuideTransitionEvidence, PairedGuideTransition,
    PairedGuideTransitionOutcome, StableGuideSessionId, StableTransitionId, WorldLayer,
    run_paired_guide_transition_cycle,
};

fn reference(key: &str) -> ExternalRef {
    ExternalRef::new("witness", key).expect("static audit witness reference is valid")
}

fn transition(
    session: &DualGuideSession,
    direction: GuideTransitionDirection,
    transition_id: &str,
) -> Result<PairedGuideTransition, Box<dyn Error>> {
    Ok(PairedGuideTransition::proposed(
        session,
        StableTransitionId::new(transition_id)?,
        direction,
        GuideTransitionEvidence {
            roi_continuity_ref: reference("roi-continuity"),
            roi_embodiment_ref: reference("roi-embodiment"),
            roselina_continuity_ref: reference("roselina-continuity"),
            roselina_embodiment_ref: reference("roselina-embodiment"),
            source_witness_ref: None,
        },
    )?)
}

fn confirmed_names(
    outcome: PairedGuideTransitionOutcome,
) -> Result<(String, String), Box<dyn Error>> {
    match outcome {
        PairedGuideTransitionOutcome::Confirmed(witness) => Ok((
            witness.resulting_session.roi_name,
            witness.resulting_session.roselina_name,
        )),
        PairedGuideTransitionOutcome::Rejected(rejection) => Err(io::Error::other(format!(
            "Guide transition audit rejected: {:?}",
            rejection.reasons
        ))
        .into()),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let session_id = StableGuideSessionId::new("session.guide-transition-audit")?;
    let mut session = DualGuideSession::outside(session_id.clone());

    let entry = transition(
        &session,
        GuideTransitionDirection::EnterHollowGrove,
        "transition.audit-entry",
    )?;
    let entry_cycle = run_paired_guide_transition_cycle(&mut session, &entry)?;
    let entry_names = confirmed_names(entry_cycle.festival)?;
    if session.current_layer != WorldLayer::HollowGrove
        || entry_names != ("Roi".into(), "Roselina".into())
    {
        return Err(io::Error::other("entry names or layer were not canonical").into());
    }

    let exit = transition(
        &session,
        GuideTransitionDirection::LeaveHollowGrove,
        "transition.audit-exit",
    )?;
    let exit_cycle = run_paired_guide_transition_cycle(&mut session, &exit)?;
    let exit_names = confirmed_names(exit_cycle.festival)?;
    if session.session_id != session_id
        || session.current_layer != WorldLayer::OutsideHollowGrove
        || exit_names != ("Roï".into(), "Rosaliña".into())
    {
        return Err(io::Error::other("exit did not preserve the canonical session").into());
    }

    println!("The Rampart of Roi and Roselina — Central Junction Guide Transition V1");
    println!("entry: Roï → Roi; Rosaliña → Roselina");
    println!("exit: Roi → Roï; Roselina → Rosaliña");
    println!("session: {}", session.session_id);
    println!("identity: guide.roi preserved");
    println!("identity: guide.roselina preserved");
    println!("result: confirmed");
    Ok(())
}
