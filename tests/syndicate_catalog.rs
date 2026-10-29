use hollow_grove::syndicate_catalog::{RANK_TITLES, SYNDICATE_CATALOG_VERSION};

#[test]
fn catalog_preserves_approved_three_by_three_rank_identity_and_provenance() {
    let expected = [
        (
            "syndicate.word",
            "Warlock",
            1,
            "syndicate.word.rung_1.warlock",
        ),
        (
            "syndicate.word",
            "Wizard",
            2,
            "syndicate.word.rung_2.wizard",
        ),
        (
            "syndicate.word",
            "Sorcerer",
            3,
            "syndicate.word.rung_3.sorcerer",
        ),
        (
            "syndicate.work",
            "Wiccan",
            1,
            "syndicate.work.rung_1.wiccan",
        ),
        ("syndicate.work", "Witch", 2, "syndicate.work.rung_2.witch"),
        (
            "syndicate.work",
            "Sorceress",
            3,
            "syndicate.work.rung_3.sorceress",
        ),
        (
            "syndicate.performance",
            "Mage",
            1,
            "syndicate.performance.rung_1.mage",
        ),
        (
            "syndicate.performance",
            "Magician",
            2,
            "syndicate.performance.rung_2.magician",
        ),
        (
            "syndicate.performance",
            "Mime",
            3,
            "syndicate.performance.rung_3.mime",
        ),
    ];

    assert_eq!(RANK_TITLES.len(), expected.len());
    for (rank, (discipline, title, rung, id)) in RANK_TITLES.iter().zip(expected) {
        assert_eq!(rank.discipline_id, discipline);
        assert_eq!(rank.title, title);
        assert_eq!(rank.rung, rung);
        assert_eq!(rank.id, id);
        assert_eq!(rank.catalog_version, SYNDICATE_CATALOG_VERSION);
        assert!(rank.source_ref.contains("HOLLOW_GROVE_REV_C.md §4.4"));
        assert!(
            rank.source_ref
                .contains("HOLLOW_GROVE_MASTER_CANON_WORKING_REV100_TUI_HANDOFF.md §6.5")
        );
    }

    for (i, rank) in RANK_TITLES.iter().enumerate() {
        assert!(RANK_TITLES[..i].iter().all(|prior| prior.id != rank.id));
    }
}

#[test]
fn syndicate_warlock_has_a_namespaced_id_separate_from_the_legacy_title() {
    let syndicate_warlock = &RANK_TITLES[0];
    assert_eq!(syndicate_warlock.id, "syndicate.word.rung_1.warlock");
    assert_ne!(syndicate_warlock.id, "Warlock");
}
