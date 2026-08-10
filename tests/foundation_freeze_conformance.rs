use hollow_grove::kernel_invariants::{
    CENTRAL_JUNCTION_COORDINATION_IDENTITY, PLATINUM_PATH_ID, PLATINUM_PATH_NAME,
    PLATINUM_PATH_SHORT_NAME, central_junction_is_coordination_only,
};

#[test]
fn foundation_freeze_names_the_art_safe_contract() {
    let freeze = include_str!("../FOUNDATION_FREEZE_V1.md");
    for required in [
        "universal kernel",
        "four House jurisdictions",
        "one Federation",
        "Roi and Roselina",
        "archive checksums",
        "refusal",
        "kernel_audit",
        "renderer-owned assets",
    ] {
        assert!(
            freeze.contains(required),
            "foundation freeze omits {required}"
        );
    }
}

#[test]
fn foundation_freeze_constants_remain_canonical() {
    assert_eq!(
        CENTRAL_JUNCTION_COORDINATION_IDENTITY,
        "region.central-junction"
    );
    assert_eq!(PLATINUM_PATH_ID, "path.platinum.v1");
    assert_eq!(PLATINUM_PATH_NAME, "Platinum Path");
    assert_eq!(PLATINUM_PATH_SHORT_NAME, "the Plath");
    assert!(central_junction_is_coordination_only());
}
