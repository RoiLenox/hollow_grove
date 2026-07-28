use hollow_grove::build_canonical_point_squared_fixture;
use hollow_grove::composition::{PhysicalPosition, PointId, PositiveScaleFactor, RelativePolarity};
use hollow_grove::world::world_point::{
    CENTRAL_JUNCTION_REGION_ID, ConstitutionalLawfulness, DARK_AURA_REGION_ID,
    LIGHT_AURA_REGION_ID, WORLD_NEGATIVE_POLE_ID, WORLD_POINT_ID, WORLD_POSITIVE_POLE_ID,
};
use hollow_grove::world::world_point_archive::{
    WORLD_POINT_ARCHIVE_VERSION, WORLD_POINT_LEGACY_ARCHIVE_VERSION, WorldPointArchiveError,
    decode_world_point_archive, encode_legacy_world_point_archive_v0, encode_world_point_archive,
    migrate_world_point_archive,
};
use hollow_grove::world::world_point_fixture::canonical_world_point_archive_fixture;

#[test]
fn oriented_point_has_center_axis_and_two_stable_serialized_poles() {
    let payload = canonical_world_point_archive_fixture();
    let point = &payload.binding.point;
    point.validate().unwrap();
    assert_eq!(point.point_id.as_str(), WORLD_POINT_ID);
    assert_eq!(point.positive_pole_id.as_str(), WORLD_POSITIVE_POLE_ID);
    assert_eq!(point.negative_pole_id.as_str(), WORLD_NEGATIVE_POLE_ID);
    assert_ne!(point.positive_pole_id, point.negative_pole_id);
    assert_ne!(point.center_id.as_str(), point.positive_pole_id.as_str());

    let json = serde_json::to_vec(point).unwrap();
    let decoded: hollow_grove::composition::OrientedPoint = serde_json::from_slice(&json).unwrap();
    assert_eq!(decoded.positive_pole_id, point.positive_pole_id);
    assert_eq!(decoded.negative_pole_id, point.negative_pole_id);
    assert_eq!(decoded.orientation, point.orientation);
}

#[test]
fn scaling_preserves_center_poles_axis_and_rejects_numeric_inversion() {
    let payload = canonical_world_point_archive_fixture();
    let source = &payload.source_point;
    let world = &payload.binding.point;
    assert_ne!(source.point_id, world.point_id);
    assert_eq!(source.center_id, world.center_id);
    assert_eq!(source.center, world.center);
    assert_eq!(source.positive_pole_id, world.positive_pole_id);
    assert_eq!(source.negative_pole_id, world.negative_pole_id);
    assert_eq!(source.orientation, world.orientation);
    assert!(world.extent.get() > source.extent.get());
    assert!(PositiveScaleFactor::new(-1).is_err());
}

#[test]
fn explicit_inversion_is_recorded_and_does_not_mutate_world_binding() {
    let decoded = decode_world_point_archive(
        &encode_world_point_archive(&canonical_world_point_archive_fixture()).unwrap(),
    )
    .unwrap();
    let world = &decoded.state.binding.point;
    let inverted = &decoded.state.explicit_inversion_probe_result;
    assert_ne!(inverted.point_id, world.point_id);
    assert_eq!(inverted.positive_pole_id, world.negative_pole_id);
    assert_eq!(inverted.negative_pole_id, world.positive_pole_id);
    assert_eq!(inverted.orientation, world.orientation.inverted());
    assert_eq!(world.point_id.as_str(), WORLD_POINT_ID);
}

#[test]
fn hollow_grove_world_binding_maps_north_center_and_south_physically() {
    let binding = canonical_world_point_archive_fixture().binding;
    binding.validate().unwrap();
    assert_eq!(
        binding.classify(PhysicalPosition { x: 0, y: 10, z: 0 }),
        RelativePolarity::Positive
    );
    assert_eq!(
        binding.classify(PhysicalPosition::origin()),
        RelativePolarity::Center
    );
    assert_eq!(
        binding.classify(PhysicalPosition { x: 0, y: -10, z: 0 }),
        RelativePolarity::Negative
    );
    assert_eq!(binding.light_aura_region_id.as_str(), LIGHT_AURA_REGION_ID);
    assert_eq!(
        binding.central_junction_region_id.as_str(),
        CENTRAL_JUNCTION_REGION_ID
    );
    assert_eq!(binding.dark_aura_region_id.as_str(), DARK_AURA_REGION_ID);
}

#[test]
fn polarity_and_constitutional_lawfulness_are_independent() {
    let binding = canonical_world_point_archive_fixture().binding;
    let positive_unlawful = binding.observe_lawfulness(
        RelativePolarity::Positive,
        ConstitutionalLawfulness::Unlawful,
    );
    let negative_lawful =
        binding.observe_lawfulness(RelativePolarity::Negative, ConstitutionalLawfulness::Lawful);
    assert_eq!(
        positive_unlawful.lawfulness,
        ConstitutionalLawfulness::Unlawful
    );
    assert_eq!(negative_lawful.lawfulness, ConstitutionalLawfulness::Lawful);
    assert!(binding.lawfulness_requires_separate_determination);
}

#[test]
fn map_rotation_and_camera_inversion_are_not_polarity_inputs() {
    let binding = canonical_world_point_archive_fixture().binding;
    let position = PhysicalPosition { x: 0, y: 25, z: 0 };
    let before = binding.classify(position);
    let _presentation_rotation_degrees = 180_i32;
    let _camera_y_is_inverted = true;
    let after = binding.classify(position);
    assert_eq!(before, RelativePolarity::Positive);
    assert_eq!(after, before);
    assert!(!binding.presentation_may_change_constitutional_polarity);
}

#[test]
fn archive_replay_is_deterministic_and_order_independent() {
    let payload = canonical_world_point_archive_fixture();
    let canonical = encode_world_point_archive(&payload).unwrap();
    let first = decode_world_point_archive(&canonical).unwrap();
    let second = decode_world_point_archive(&canonical).unwrap();
    assert_eq!(first.archive_version, WORLD_POINT_ARCHIVE_VERSION);
    assert_eq!(first.state, second.state);

    let mut reverse = payload;
    reverse.relationships.reverse();
    assert_eq!(encode_world_point_archive(&reverse).unwrap(), canonical);
}

#[test]
fn legacy_unoriented_world_point_requires_recorded_migration_decision() {
    let legacy =
        encode_legacy_world_point_archive_v0(&canonical_world_point_archive_fixture()).unwrap();
    let decoded_legacy = decode_world_point_archive(&legacy).unwrap();
    assert_eq!(
        decoded_legacy.archive_version,
        WORLD_POINT_LEGACY_ARCHIVE_VERSION
    );
    assert_eq!(
        decoded_legacy.state.binding.point.orientation.components(),
        [0, 1, 0]
    );
    let migrated = migrate_world_point_archive(&legacy).unwrap();
    let decoded = decode_world_point_archive(&migrated).unwrap();
    assert_eq!(decoded.archive_version, WORLD_POINT_ARCHIVE_VERSION);
    assert_eq!(
        decoded.state.binding.point.point_id.as_str(),
        WORLD_POINT_ID
    );
}

#[test]
fn checksum_and_binding_validation_detect_polarity_tampering() {
    let payload = canonical_world_point_archive_fixture();
    let bytes = encode_world_point_archive(&payload).unwrap();
    let text = String::from_utf8(bytes).unwrap();
    let tampered = text.replace(WORLD_POSITIVE_POLE_ID, WORLD_NEGATIVE_POLE_ID);
    assert!(matches!(
        decode_world_point_archive(tampered.as_bytes()),
        Err(WorldPointArchiveError::ChecksumMismatch)
    ));

    let mut swapped = payload;
    std::mem::swap(
        &mut swapped.binding.field.positive_region_id,
        &mut swapped.binding.field.negative_region_id,
    );
    assert!(matches!(
        encode_world_point_archive(&swapped),
        Err(WorldPointArchiveError::Binding(_)) | Err(WorldPointArchiveError::Kernel(_))
    ));
}

#[test]
fn result_identity_is_caller_supplied_not_derived_from_order() {
    let payload = canonical_world_point_archive_fixture();
    assert_eq!(
        payload.scaling.result_point_id,
        PointId::new(WORLD_POINT_ID).unwrap()
    );
    assert_ne!(
        payload.source_point.point_id,
        payload.scaling.result_point_id
    );
}

#[test]
fn universal_kernel_has_no_hollow_grove_proper_name_binding() {
    const KERNEL: &str = include_str!("../hollow-grove-kernel/src/lib.rs");
    const ORIENTED: &str = include_str!("../hollow-grove-kernel/src/oriented_point.rs");
    for forbidden in [
        "Central Junction",
        "Light Aura",
        "Dark Aura",
        "Stonebend",
        "Sandmanor",
        "Glaüshouse",
        "Flynt",
    ] {
        assert!(!KERNEL.contains(forbidden), "kernel contains {forbidden}");
        assert!(!ORIENTED.contains(forbidden), "kernel contains {forbidden}");
    }
}

#[test]
fn existing_point_squared_progression_preserves_oriented_point_state() {
    let fixture = build_canonical_point_squared_fixture().unwrap();
    assert_eq!(
        fixture.point_before().physical(),
        fixture.first_application().stabilized_point().physical()
    );
    assert_eq!(
        fixture.first_application().stabilized_point().physical(),
        fixture.second_application().stabilized_point().physical()
    );
}
