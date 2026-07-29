use hollow_grove::{
    AxisDelta, AxisValue, CONSTITUTIONAL_HOUSE_ORDER, CoordinateError, House, KernelAnchor,
    Position3, Translation3, house_distances, squared_distance, verify_translation,
};

#[test]
fn all_five_constitutional_positions_are_immutable_constants() {
    let anchors = [
        (
            KernelAnchor::House(House::Stonebend),
            Position3::from_raw(0, 0, 1000).unwrap(),
        ),
        (
            KernelAnchor::House(House::Sandmanor),
            Position3::from_raw(1000, 0, 0).unwrap(),
        ),
        (
            KernelAnchor::House(House::Glaushouse),
            Position3::from_raw(0, 0, -1000).unwrap(),
        ),
        (
            KernelAnchor::House(House::Flynt),
            Position3::from_raw(-1000, 0, 0).unwrap(),
        ),
        (KernelAnchor::CentralJunction, Position3::ORIGIN),
    ];
    for (anchor, expected) in anchors {
        assert_eq!(anchor.position(), expected);
        assert_eq!(anchor.position().y(), AxisValue::ZERO);
    }
}

#[test]
fn coordinate_and_delta_validation_fail_closed() {
    assert_eq!(AxisValue::new(-1000).unwrap().get(), -1000);
    assert_eq!(AxisValue::new(1000).unwrap().get(), 1000);
    assert!(AxisValue::new(-1001).is_err());
    assert!(AxisValue::new(1001).is_err());
    assert_eq!(AxisDelta::new(-2000).unwrap().get(), -2000);
    assert_eq!(AxisDelta::new(2000).unwrap().get(), 2000);
    assert!(AxisDelta::new(-2001).is_err());
    assert!(AxisDelta::new(2001).is_err());
    assert!(Position3::from_raw(0, 0, 0).is_ok());
    assert!(Position3::from_raw(0, 1, 0).is_err());
    assert!(Position3::from_raw(0, -1, 0).is_err());
    assert!(Translation3::from_raw(0, 1, 0).is_err());
}

#[test]
fn movement_law_checks_source_translation_and_destination() {
    let source = KernelAnchor::House(House::Glaushouse).position();
    let destination = KernelAnchor::House(House::Stonebend).position();
    let translation = source.translation_to(destination).unwrap();
    assert_eq!(translation, Translation3::from_raw(0, 0, 2000).unwrap());
    assert_eq!(source.translated(translation).unwrap(), destination);
    verify_translation(source, translation, destination).unwrap();
    assert!(matches!(
        verify_translation(source, translation, Position3::ORIGIN),
        Err(CoordinateError::TranslationMismatch { .. })
    ));
}

#[test]
fn distance_relationships_are_complete_and_deterministic() {
    let origin = house_distances(Position3::ORIGIN);
    assert_eq!(origin.len(), 4);
    assert_eq!(
        origin.iter().map(|entry| entry.house).collect::<Vec<_>>(),
        CONSTITUTIONAL_HOUSE_ORDER
    );
    assert!(
        origin
            .iter()
            .all(|entry| entry.squared_distance == 1_000_000)
    );
    for house in CONSTITUTIONAL_HOUSE_ORDER {
        let anchor = KernelAnchor::House(house).position();
        assert_eq!(squared_distance(anchor, anchor), 0);
        assert_eq!(
            squared_distance(anchor, Position3::ORIGIN),
            squared_distance(Position3::ORIGIN, anchor)
        );
    }
}

#[test]
fn serialization_never_bypasses_reserved_y_or_unknown_field_checks() {
    let position = Position3::from_raw(-120, 0, 580).unwrap();
    let translation = Translation3::from_raw(-40, 0, 890).unwrap();
    assert_eq!(
        serde_json::from_str::<Position3>(&serde_json::to_string(&position).unwrap()).unwrap(),
        position
    );
    assert_eq!(
        serde_json::from_str::<Translation3>(&serde_json::to_string(&translation).unwrap())
            .unwrap(),
        translation
    );
    for invalid in [
        r#"{"x":0,"y":1,"z":0}"#,
        r#"{"x":0,"y":0,"z":0,"unknown":true}"#,
        r#"{"x":0.25,"y":0,"z":0}"#,
    ] {
        assert!(serde_json::from_str::<Position3>(invalid).is_err());
    }
}
