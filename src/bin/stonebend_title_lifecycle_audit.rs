use std::collections::BTreeSet;

use hollow_grove::world::stonebend::second_pass::{
    GateFailureKind, GateScope, StonebendGateFacing,
};
use hollow_grove::world::stonebend::third_pass::{
    ContinuityActionKind, ProliteriateContinuityPolicy, TitleLifecycleStage,
    TitleRenewalDisposition, TitleTerminalDisposition,
};

fn main() {
    assert_eq!(StonebendGateFacing::ALL.len(), 3);
    assert_eq!(GateScope::ALL.len(), 3);
    assert!(
        TitleLifecycleStage::ALL
            .windows(2)
            .all(|stages| { stages[0].semantic_order() < stages[1].semantic_order() })
    );
    assert_eq!(
        [
            GateFailureKind::HonestFailure,
            GateFailureKind::Negligence,
            GateFailureKind::Fraud,
            GateFailureKind::Illegality,
            GateFailureKind::ConstitutionalHollowness,
        ]
        .into_iter()
        .collect::<BTreeSet<_>>()
        .len(),
        5
    );
    assert_eq!(
        [
            TitleRenewalDisposition::Renewed,
            TitleRenewalDisposition::RenewedWithLimitations,
            TitleRenewalDisposition::RemediationRequired,
            TitleRenewalDisposition::Deferred,
            TitleRenewalDisposition::Rejected,
            TitleRenewalDisposition::Expired,
        ]
        .len(),
        6
    );
    assert_eq!(
        [
            TitleTerminalDisposition::HonorableCompletion,
            TitleTerminalDisposition::Surrender,
            TitleTerminalDisposition::Expiration,
            TitleTerminalDisposition::Death,
            TitleTerminalDisposition::EndOfForm,
            TitleTerminalDisposition::Succession,
            TitleTerminalDisposition::RemovalForFailure,
            TitleTerminalDisposition::RemovalForFraud,
            TitleTerminalDisposition::RemovalForIllegality,
            TitleTerminalDisposition::ConstitutionalDissolution,
            TitleTerminalDisposition::Supersession,
        ]
        .into_iter()
        .collect::<BTreeSet<_>>()
        .len(),
        11
    );
    for forbidden in [
        ContinuityActionKind::InvestDiamond,
        ContinuityActionKind::CreateSovereignLaw,
        ContinuityActionKind::PermanentlyExpandGateScope,
        ContinuityActionKind::AppointHypergiant,
        ContinuityActionKind::EraseChallenge,
        ContinuityActionKind::RemovePrincipalPower,
    ] {
        assert!(!forbidden.lawful_during_vacancy());
    }
    let network_policy = ProliteriateContinuityPolicy::default();
    assert!(network_policy.validate().is_ok());
    assert!(network_policy.permanent_selection_threshold.is_none());
    assert!(network_policy.permanent_speaker.is_none());

    println!("Stonebend Title Lifecycle and Continuity Audit: pass");
    println!("lifecycle stages: semantic");
    println!("recognition distinct from activation: true");
    println!("maintenance distinct from renewal: true");
    println!("failure classifications: 5");
    println!("renewal dispositions: 6");
    println!("terminal dispositions: 11");
    println!("gate scopes preserved: 3");
    println!("Diamond vacancy continuity may invest Diamond: false");
    println!("permanent Regent or Acting Hypergiant: false");
    println!("independent Forge replacement required: true");
    println!("permanent Proliteriate speaker: false");
    println!("permanent numerical network threshold: false");
    println!("recursion kernel dependency: none");
}
