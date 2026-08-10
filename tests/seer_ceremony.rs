use hollow_grove::{
    CeremonyMovement, SeerCeremonyCoordinator, SeerCeremonyError, SeerCeremonyOutcome, SeerPath,
};

fn advance_to_squaring(coordinator: &mut SeerCeremonyCoordinator) {
    assert_eq!(coordinator.advance().unwrap(), CeremonyMovement::Naming);
    assert_eq!(
        coordinator.advance().unwrap(),
        CeremonyMovement::Convergence
    );
    assert_eq!(
        coordinator.advance().unwrap(),
        CeremonyMovement::Recognition
    );
    assert_eq!(coordinator.advance().unwrap(), CeremonyMovement::Squaring);
}

#[test]
fn one_individual_must_choose_continuance_or_founding() {
    let mut coordinator = SeerCeremonyCoordinator::new();
    coordinator
        .begin("ceremony.1", "person.1", "point.1")
        .unwrap();
    advance_to_squaring(&mut coordinator);
    coordinator.choose_path(SeerPath::Continuance).unwrap();
    assert_eq!(
        coordinator.complete().unwrap(),
        SeerCeremonyOutcome::Continued {
            seer_id: "person.1".into(),
            point_id: "point.1".into(),
        }
    );
}

#[test]
fn foundation_creates_a_point_and_allows_unlimited_sequential_ceremonies() {
    let mut coordinator = SeerCeremonyCoordinator::new();
    coordinator
        .begin("ceremony.1", "person.1", "point.1")
        .unwrap();
    advance_to_squaring(&mut coordinator);
    coordinator.choose_path(SeerPath::Founding).unwrap();
    assert_eq!(
        coordinator.complete().unwrap(),
        SeerCeremonyOutcome::Founded {
            seer_id: "person.1".into(),
            source_point_id: "point.1".into(),
            created_point_id: "point.1.ceremony.1".into(),
        }
    );

    coordinator
        .begin("ceremony.2", "person.2", "point.1.ceremony.1")
        .unwrap();
    assert!(coordinator.active().is_some());
}

#[test]
fn ceremonies_cannot_overlap_but_are_not_globally_limited() {
    let mut coordinator = SeerCeremonyCoordinator::new();
    coordinator
        .begin("ceremony.1", "person.1", "point.1")
        .unwrap();
    assert_eq!(
        coordinator
            .begin("ceremony.2", "person.2", "point.1")
            .unwrap_err(),
        SeerCeremonyError::CeremonyAlreadyActive
    );
}
