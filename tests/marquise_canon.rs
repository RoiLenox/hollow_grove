use std::fs;
use std::path::Path;

use flynt_constitution::{
    EXTENSION_MARQUISE, INSTITUTION_MYSTERY_MEN, OFFICE_TROSS, canonical_constitution,
};
use hollow_grove::institution::{InstitutionalEntityId, RelationshipKind};
use hollow_grove::world::flynt::{
    canonical_flynt_institutions, corsair_role_id, marquise_group_id, marquise_office_id,
    mystery_men_id, privateer_role_id, tross_office_id,
};
use hollow_grove::{
    MarquiseMeaning, MarquiseStatusCategory, canonical_marquise_lore, validate_marquise_lore,
};

const CANON: &str = include_str!("../FLYNT_MARQUISE_CANON_V1.md");

#[test]
fn marquise_is_a_maritime_child_of_mystery_men_under_the_tross() {
    let lore = canonical_marquise_lore();
    assert!(validate_marquise_lore(&lore).is_empty());
    assert_eq!(lore.parent_tradition, "Mystery Men");
    assert_eq!(lore.relation_to_parent, "Maritime extension");
    assert_eq!(
        lore.command_chain,
        ["The Tross", "Mystery Men", "The Marquise"]
    );
    assert_eq!(
        lore.meanings,
        [MarquiseMeaning::Organization, MarquiseMeaning::LeaderTitle]
    );

    let constitution = canonical_constitution().unwrap();
    assert_eq!(
        constitution
            .authority_chain(&flynt_constitution::FlyntNodeId::new(EXTENSION_MARQUISE).unwrap())
            .unwrap()
            .into_iter()
            .map(|node| node.id.as_str())
            .collect::<Vec<_>>(),
        [EXTENSION_MARQUISE, INSTITUTION_MYSTERY_MEN, OFFICE_TROSS]
    );
}

#[test]
fn neutral_projection_uses_a_mystery_men_group_not_a_sibling_institution() {
    let flynt = canonical_flynt_institutions();
    let marquise = flynt
        .catalog
        .groups
        .iter()
        .find(|group| group.id == marquise_group_id())
        .unwrap();
    assert_eq!(marquise.institution, mystery_men_id());
    assert!(
        flynt
            .catalog
            .institutions
            .iter()
            .all(|institution| institution.name != "The Marquise")
    );
    assert!(flynt.catalog.relationships.iter().any(|relationship| {
        relationship.source == InstitutionalEntityId::Office(tross_office_id())
            && relationship.kind == RelationshipKind::Commands
            && relationship.target == InstitutionalEntityId::Institution(mystery_men_id())
    }));
    assert!(flynt.catalog.relationships.iter().any(|relationship| {
        relationship.source == InstitutionalEntityId::Institution(mystery_men_id())
            && relationship.kind == RelationshipKind::Commands
            && relationship.target == InstitutionalEntityId::Group(marquise_group_id())
    }));
    assert!(
        flynt
            .catalog
            .offices
            .iter()
            .any(|office| office.id == marquise_office_id()
                && office.name == "The Marquise"
                && office.institution.as_ref() == Some(&mystery_men_id()))
    );
    assert!(
        flynt
            .catalog
            .roles
            .iter()
            .any(|role| { role.id == privateer_role_id() && role.institution == mystery_men_id() })
    );
    assert!(
        flynt
            .catalog
            .roles
            .iter()
            .any(|role| { role.id == corsair_role_id() && role.institution == mystery_men_id() })
    );
}

#[test]
fn progression_and_house_civic_identities_are_exact() {
    let lore = canonical_marquise_lore();
    assert_eq!(
        lore.progression
            .iter()
            .map(|record| (record.name.as_str(), record.category))
            .collect::<Vec<_>>(),
        [
            ("Warden", MarquiseStatusCategory::Citizen),
            (
                "Privateer",
                MarquiseStatusCategory::PractitionerOperationalMember
            ),
            ("Corsair", MarquiseStatusCategory::ApexRank),
            ("The Marquise", MarquiseStatusCategory::LeadershipOffice),
        ]
    );
    assert_eq!(
        lore.house_civic_identities
            .iter()
            .map(|record| (record.people.as_str(), record.house.as_str()))
            .collect::<Vec<_>>(),
        [
            ("Wardens", "Flynt"),
            ("Geralds", "Stonebend"),
            ("Sandmen", "Sandmanor"),
        ]
    );
}

#[test]
fn canonical_document_and_repository_do_not_invent_forbidden_rank_names() {
    for required in [
        "THE TROSS → MYSTERY MEN → THE MARQUISE",
        "Wardens = Flynt",
        "Geralds = Stonebend",
        "Sandmen = Sandmanor",
        "A Corsair pilots systems.",
        "Merman ↔ Mermaid",
        "At sea, The Push becomes navigation.",
    ] {
        assert!(CANON.contains(required), "missing canon phrase: {required}");
    }

    let source =
        fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/world/marquise.rs"))
            .unwrap();
    for forbidden in [
        "Grand Corsair",
        "Sea Marshal",
        "Fleetmaster",
        "Merman vs Mermaid",
        "sibling_of",
    ] {
        assert!(
            !source.contains(forbidden),
            "Marquise source invents or encodes forbidden term: {forbidden}"
        );
    }
}
