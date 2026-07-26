use flynt_constitution::{
    EXPRESSION_MYSTERY_MAN, EXPRESSION_WE_FAIRY_MEN, FORM_CHIMERA, FORM_MANTICORP,
    INSTITUTION_GALLOWS, OFFICE_TROSS, canonical_constitution,
};
use hollow_grove::institution::InstitutionalEntityId;
use hollow_grove::world::flynt::{
    constitutional_chimera_id, gallowry_site_id, gallows_id, manticorp_id, mystery_men_id,
    tross_office_id,
};
use hollow_grove::world::{canonical_institutional_world_state, flynt};

#[test]
fn domain_audit_proves_unique_authority_and_complete_superiors() {
    let constitution = canonical_constitution().unwrap();
    let audit = constitution.audit().unwrap();

    assert_eq!(audit.sovereign_executive.as_str(), OFFICE_TROSS);
    assert_eq!(audit.constitutional_chimera_count, 1);
    assert_eq!(audit.chimera_recipe_count, 1);
    assert_eq!(audit.duplicate_authority_count, 0);
    assert!(audit.all_non_root_nodes_have_one_superior);
    assert!(audit.hierarchy_is_acyclic);
    assert!(audit.all_authority_reaches_tross);
    assert!(audit.gallowry_is_distinct_from_gallows);
    assert!(audit.founding_union_is_complete);
}

#[test]
fn canonical_urban_and_rural_chains_are_complementary() {
    let constitution = canonical_constitution().unwrap();
    let chain = |leaf: &str| {
        constitution
            .authority_chain(&flynt_constitution::FlyntNodeId::new(leaf).unwrap())
            .unwrap()
            .into_iter()
            .map(|node| node.id.as_str())
            .collect::<Vec<_>>()
    };

    assert_eq!(
        chain(EXPRESSION_MYSTERY_MAN),
        [EXPRESSION_MYSTERY_MAN, INSTITUTION_GALLOWS, OFFICE_TROSS,]
    );
    assert_eq!(
        chain(EXPRESSION_WE_FAIRY_MEN),
        [EXPRESSION_WE_FAIRY_MEN, INSTITUTION_GALLOWS, OFFICE_TROSS,]
    );
}

#[test]
fn world_projection_preserves_domain_authority_without_duplicate_hierarchy() {
    let flynt = flynt::canonical_flynt_institutions();
    flynt.validate().unwrap();

    assert!(flynt.catalog.institution(&manticorp_id()).is_some());
    assert!(flynt.catalog.institution(&mystery_men_id()).is_some());
    assert!(flynt.catalog.institution(&gallows_id()).is_some());
    assert!(
        flynt
            .catalog
            .sites
            .iter()
            .any(|site| site.id == gallowry_site_id()
                && site.controlled_by.as_ref() == Some(&gallows_id()))
    );
    assert!(
        flynt
            .catalog
            .offices
            .iter()
            .all(|office| office.name != "Chimera")
    );
}

#[test]
fn tross_publicly_and_underground_commands_are_distinct_faces_of_one_sovereign() {
    let flynt = flynt::canonical_flynt_institutions();
    let relationships = &flynt.catalog.relationships;

    assert!(relationships.iter().any(|relationship| {
        relationship.source == InstitutionalEntityId::Office(tross_office_id())
            && relationship.target == InstitutionalEntityId::Being(constitutional_chimera_id())
    }));
    for branch in [manticorp_id(), gallows_id()] {
        assert!(relationships.iter().any(|relationship| {
            relationship.source == InstitutionalEntityId::Office(tross_office_id())
                && relationship.target == InstitutionalEntityId::Institution(branch.clone())
        }));
    }
}

#[test]
fn manticorp_form_and_institution_are_distinct() {
    let constitution = canonical_constitution().unwrap();
    assert_eq!(constitution.chimera().form.as_str(), FORM_CHIMERA);
    assert!(
        constitution
            .forms()
            .iter()
            .any(|form| form.id.as_str() == FORM_MANTICORP)
    );
    assert_eq!(
        constitution.manticorp_recipe().recipe.result.as_str(),
        FORM_MANTICORP
    );
}

#[test]
fn aggregate_world_contains_exact_flynt_institutions() {
    let world = canonical_institutional_world_state();
    world.validate().unwrap();
    assert!(world.catalog.institution(&manticorp_id()).is_some());
    assert!(world.catalog.institution(&mystery_men_id()).is_some());
    assert!(world.catalog.institution(&gallows_id()).is_some());
}
