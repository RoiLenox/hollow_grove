use std::collections::BTreeSet;

use hollow_grove::hollow_grove_contract::House;
use hollow_grove::world::{house_institutions, stonebend};

#[test]
fn principal_authorities_have_one_constitutional_place() {
    stonebend::validate_principal_authorities().unwrap();
    assert_eq!(
        stonebend::PRINCIPAL_AUTHORITIES
            .iter()
            .filter(|entry| {
                entry.placement == stonebend::ConstitutionalPlacement::SingularHighestOffice
            })
            .count(),
        1
    );
    assert_eq!(
        stonebend::PRINCIPAL_AUTHORITIES
            .iter()
            .find(|entry| {
                entry.placement == stonebend::ConstitutionalPlacement::SingularHighestOffice
            })
            .unwrap()
            .authority,
        stonebend::PrincipalAuthority::Hypergiant
    );
}

#[test]
fn neutral_catalog_preserves_people_network_institution_and_offices() {
    let catalog = house_institutions::canonical_house_institutions();
    catalog.validate().unwrap();

    let institutions = catalog
        .institutions
        .iter()
        .filter(|entry| entry.house == Some(House::Stonebend))
        .map(|entry| entry.id.as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        institutions,
        BTreeSet::from([
            "institution.stonebend.constitution",
            "institution.stonebend.proliteriate",
            "institution.stonebend.freemason",
        ])
    );

    let offices = catalog
        .offices
        .iter()
        .filter(|entry| entry.house == Some(House::Stonebend))
        .collect::<Vec<_>>();
    assert_eq!(offices.len(), 2);
    assert!(
        offices
            .iter()
            .any(|entry| entry.id == stonebend::hypergiant_office_id() && entry.singular)
    );
    assert!(offices.iter().any(|entry| {
        entry.id == stonebend::high_freemason_office_id()
            && entry.singular
            && entry.institution == Some(stonebend::freemason_institution_id())
    }));

    assert!(catalog.roles.iter().any(|entry| {
        entry.id.as_str() == "role.stonebend.gerald"
            && entry.institution == stonebend::stonebend_constitution_id()
    }));
    assert!(catalog.roles.iter().any(|entry| {
        entry.id.as_str() == "role.stonebend.proliteriate-representative"
            && entry.institution == stonebend::proliteriate_id()
    }));
    assert!(catalog.roles.iter().any(|entry| {
        entry.id.as_str() == "role.stonebend.freemason-member"
            && entry.institution == stonebend::freemason_institution_id()
    }));
    assert!(!catalog.roles.iter().any(|entry| {
        matches!(
            entry.name.as_str(),
            "Hypergiant" | "Proliteriate" | "Freemason"
        )
    }));
}

#[test]
fn neutral_relationships_do_not_make_three_peer_sovereigns() {
    let catalog = house_institutions::canonical_house_institutions();
    let hypergiant = hollow_grove::InstitutionalEntityId::Office(stonebend::hypergiant_office_id());
    let high_freemason =
        hollow_grove::InstitutionalEntityId::Office(stonebend::high_freemason_office_id());
    let freemason =
        hollow_grove::InstitutionalEntityId::Institution(stonebend::freemason_institution_id());
    let proliteriate =
        hollow_grove::InstitutionalEntityId::Institution(stonebend::proliteriate_id());

    assert!(catalog.relationships.iter().any(|entry| {
        entry.source == high_freemason
            && entry.kind == hollow_grove::RelationshipKind::Coordinates
            && entry.target == hypergiant
    }));
    assert!(catalog.relationships.iter().any(|entry| {
        entry.source == high_freemason
            && entry.kind == hollow_grove::RelationshipKind::Commands
            && entry.target == freemason
    }));
    assert!(!catalog.relationships.iter().any(|entry| {
        entry.source == hypergiant
            && entry.kind == hollow_grove::RelationshipKind::Commands
            && entry.target == proliteriate
    }));
}

#[test]
fn action_practice_projection_contains_no_constitutional_office_variant() {
    use hollow_grove::flow_glow_grammar::StonebendPractice;

    for practice in [
        StonebendPractice::StructuralPractice,
        StonebendPractice::CivicWitness,
        StonebendPractice::IdentityCustodian,
    ] {
        assert!(!matches!(
            practice.as_str(),
            "Hypergiant" | "Proliteriate" | "Freemason"
        ));
    }
}
