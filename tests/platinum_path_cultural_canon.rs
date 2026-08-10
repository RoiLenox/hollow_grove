#[test]
fn platinum_path_canon_preserves_one_path_with_local_definitions() {
    let canon = include_str!("../PLATINUM_PATH_CULTURAL_CANON_V1.md");
    for required in [
        "next version of the Golden Rule",
        "Continuity",
        "Consent",
        "Witness",
        "Custody",
        "Return",
        "Roi and Roselina",
        "Twofold Witness",
        "Stonebend",
        "Sandmanor",
        "Glaüshouse",
        "Flynt",
        "The Gimp of the Underway",
        "The Pimp of the Boardwalk",
        "Clouseau and Current Synthesis",
        "not a new sovereign layer",
    ] {
        assert!(
            canon.contains(required),
            "Platinum Path canon omits {required}"
        );
    }
    assert!(!canon.contains("the Guides"));
    assert!(canon.contains("not a formal Hollow"));
}
