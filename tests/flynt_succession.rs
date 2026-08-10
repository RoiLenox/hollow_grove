use hollow_grove::{TrossSuccessionRecord, TrossSuccessionStage, TrossSuccessionTrigger};

#[test]
fn tross_succession_requires_all_four_houses_and_court() {
    let mut case = TrossSuccessionRecord::open(
        "case.flynt.tross-succession.v1",
        "being.flynt.tross.current",
        "being.flynt.tross.candidate-a",
        TrossSuccessionTrigger::Abdication,
    )
    .unwrap();
    for (stage, record) in [
        (TrossSuccessionStage::StonebendNamed, "stonebend.name.v1"),
        (TrossSuccessionStage::SandmanorProven, "sandmanor.proof.v1"),
        (
            TrossSuccessionStage::GlaushouseCleared,
            "glaüshouse.clearance.v1",
        ),
        (
            TrossSuccessionStage::FlyntRecognized,
            "flynt.recognition.v1",
        ),
    ] {
        case.advance(stage, Some(record.into())).unwrap();
    }
    case.advance(
        TrossSuccessionStage::CourtReviewed,
        Some("court.review.v1".into()),
    )
    .unwrap();
    case.advance(TrossSuccessionStage::StonebendSealed, None)
        .unwrap();
    assert_eq!(case.stage, TrossSuccessionStage::StonebendSealed);
    assert_eq!(case.active_tross_id, "being.flynt.tross");
}

#[test]
fn tross_succession_cannot_skip_gates_or_self_select_from_manticorp() {
    let mut case = TrossSuccessionRecord::open(
        "case.flynt.tross-succession.v1",
        "being.flynt.tross.current",
        "being.flynt.manticorp",
        TrossSuccessionTrigger::IrreversibleIncapacity,
    )
    .unwrap();
    assert!(
        case.advance(TrossSuccessionStage::StonebendSealed, None)
            .is_err()
    );
    assert!(case.advance(TrossSuccessionStage::Rejected, None).is_ok());
}
