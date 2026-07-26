use flynt_constitution::{
    COMPANION_CHIMERA, CREW_BRO_WHITE, CREW_CINDERELLAMAN, CREW_THE_BEAUTY, ConstitutionError,
    EXPRESSION_MYSTERY_MAN, EXPRESSION_WE_FAIRY_MEN, FORM_CHIMERA, FORM_GARGOYLE, FORM_MANTICORP,
    FORM_MERMAN, FORM_WEREWOLF, FlyntConstitution, FlyntNodeId, FlyntNodeKind, GREMLINCOIN_MEANING,
    GargoyleContinuance, GremlincoinRecord, IDENTITY_MR_X, IDENTITY_MYSTERY_MAN,
    INSTITUTION_GALLOWS, INSTITUTION_MANTICORP, INSTITUTION_MYSTERY_MEN,
    ManticorpContinuanceRequirement, OFFICE_BRO_WHITE, OFFICE_CINDERELLAMAN, OFFICE_THE_BEAUTY,
    OFFICE_TROSS, PERSON_TROSS, RECIPE_GARGOYLE_CONTINUANCE, RECIPE_MANTICORP, SITE_GALLOWRY,
    TOKEN_GREMLINCOIN, WAY_GREMLIN, canonical_constitution, canonical_gremlin_way_practices,
    canonical_hierarchy_rows, canonical_parts, canonical_superior_map,
};

fn id(value: &str) -> FlyntNodeId {
    FlyntNodeId::new(value).unwrap()
}

#[test]
fn canonical_hierarchy_has_one_sovereign_and_one_chimera() {
    let constitution = canonical_constitution().unwrap();
    let audit = constitution.audit().unwrap();

    assert_eq!(audit.sovereign_executive.as_str(), OFFICE_TROSS);
    assert_eq!(audit.constitutional_chimera_count, 1);
    assert_eq!(audit.duplicate_authority_count, 0);
    assert!(audit.all_non_root_nodes_have_one_superior);
    assert!(audit.hierarchy_is_acyclic);
    assert!(audit.all_authority_reaches_tross);
    assert!(audit.founding_union_is_complete);
}

#[test]
fn urban_expression_is_exact() {
    let constitution = canonical_constitution().unwrap();
    let mystery_man = id(EXPRESSION_MYSTERY_MAN);
    let chain: Vec<_> = constitution
        .authority_chain(&mystery_man)
        .unwrap()
        .into_iter()
        .map(|node| node.id.as_str())
        .collect();

    assert_eq!(
        chain,
        [EXPRESSION_MYSTERY_MAN, INSTITUTION_GALLOWS, OFFICE_TROSS,]
    );
    assert_eq!(
        constitution
            .node_by_key(INSTITUTION_MANTICORP)
            .unwrap()
            .kind,
        FlyntNodeKind::MilitaryInstitution
    );
    assert_eq!(
        constitution
            .node_by_key(INSTITUTION_MYSTERY_MEN)
            .unwrap()
            .kind,
        FlyntNodeKind::InvestigativeBureau
    );
}

#[test]
fn rural_expression_is_exact() {
    let constitution = canonical_constitution().unwrap();
    for founder in [OFFICE_BRO_WHITE, OFFICE_CINDERELLAMAN, OFFICE_THE_BEAUTY] {
        let chain: Vec<_> = constitution
            .authority_chain(&id(founder))
            .unwrap()
            .into_iter()
            .map(|node| node.id.as_str())
            .collect();
        assert_eq!(
            chain,
            [
                founder,
                EXPRESSION_WE_FAIRY_MEN,
                INSTITUTION_GALLOWS,
                OFFICE_TROSS,
            ]
        );
    }

    let superior_map = canonical_superior_map();
    assert_eq!(superior_map[CREW_BRO_WHITE], Some(OFFICE_BRO_WHITE));
    assert_eq!(superior_map[CREW_CINDERELLAMAN], Some(OFFICE_CINDERELLAMAN));
    assert_eq!(superior_map[CREW_THE_BEAUTY], Some(OFFICE_THE_BEAUTY));
    let union = constitution.founding_union();
    assert_eq!(union.folk_expression.as_str(), EXPRESSION_WE_FAIRY_MEN);
    assert_eq!(union.institutional_home.as_str(), INSTITUTION_GALLOWS);
    assert_eq!(
        union.constitutional_expression_of.as_str(),
        COMPANION_CHIMERA
    );
}

#[test]
fn chimera_is_the_unique_three_people_synthesis() {
    let constitution = canonical_constitution().unwrap();
    let recipe = constitution.chimera_recipe();
    let sources: std::collections::HashSet<_> =
        recipe.sources.iter().map(|id| id.as_str()).collect();

    assert_eq!(recipe.result.as_str(), FORM_CHIMERA);
    assert_eq!(
        sources,
        [FORM_GARGOYLE, FORM_MERMAN, FORM_WEREWOLF]
            .into_iter()
            .collect()
    );
    assert_eq!(
        constitution
            .composition_catalog()
            .records_producing_result(recipe.result.as_kernel())
            .len(),
        1
    );
}

#[test]
fn manticorp_form_and_institution_are_distinct_but_canonical() {
    let constitution = canonical_constitution().unwrap();
    assert_eq!(
        constitution
            .node_by_key(INSTITUTION_MANTICORP)
            .unwrap()
            .kind,
        FlyntNodeKind::MilitaryInstitution
    );
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
    assert_eq!(
        constitution.manticorp_recipe().recipe.id.as_str(),
        RECIPE_MANTICORP
    );
}

#[test]
fn tross_mystery_man_mr_x_and_manticorp_bearer_share_one_identity_lock() {
    let constitution = canonical_constitution().unwrap();
    let identity = constitution.sovereign_identity();
    assert_eq!(identity.person.as_str(), PERSON_TROSS);
    assert_eq!(identity.public_title.as_str(), OFFICE_TROSS);
    assert_eq!(
        identity
            .underground_identities
            .iter()
            .map(|id| id.as_str())
            .collect::<Vec<_>>(),
        [IDENTITY_MYSTERY_MAN, IDENTITY_MR_X]
    );
    assert_eq!(identity.maintained_form.as_str(), FORM_MANTICORP);
    assert_eq!(
        identity
            .continuance_requirements
            .iter()
            .copied()
            .collect::<std::collections::BTreeSet<_>>(),
        std::collections::BTreeSet::from([
            ManticorpContinuanceRequirement::BodilyDiscipline,
            ManticorpContinuanceRequirement::RecipeRenewal,
            ManticorpContinuanceRequirement::DividedBasinKnowledge,
            ManticorpContinuanceRequirement::InstitutionalRecognition,
            ManticorpContinuanceRequirement::SpecializedGlaushouseCare,
        ])
    );
    assert_eq!(identity.public_institution.as_str(), INSTITUTION_MANTICORP);
    assert_eq!(
        identity.underground_institution.as_str(),
        INSTITUTION_GALLOWS
    );
}

#[test]
fn gremlincoin_is_the_gremlin_way_and_gargoyle_requires_continuance() {
    assert_eq!(GREMLINCOIN_MEANING, WAY_GREMLIN);
    let hueman = id("flynt.person.gremlin-candidate");
    let gremlincoin = GremlincoinRecord {
        hueman: hueman.clone(),
        token: TOKEN_GREMLINCOIN,
        way: WAY_GREMLIN,
        practices: canonical_gremlin_way_practices(),
        objective_value_created: vec!["a discarded structure became useful territory".into()],
    };
    let evidence = GargoyleContinuance {
        hueman,
        gremlincoin,
        recipe: RECIPE_GARGOYLE_CONTINUANCE,
        recipe_viable: true,
        synthesis_established: true,
        maintained_structure: true,
        territory: true,
        responsibility: true,
        maintenance_current: true,
        renewal_current: true,
    };
    assert!(evidence.validates());

    let mut numeric_threshold_only = evidence.clone();
    numeric_threshold_only.maintenance_current = false;
    assert!(!numeric_threshold_only.validates());
}

#[test]
fn divided_manticorp_recipe_has_three_indispensable_basin_custodians() {
    let constitution = canonical_constitution().unwrap();
    assert_eq!(constitution.manticorp_recipe().custody.len(), 3);
    assert_eq!(
        constitution
            .manticorp_recipe()
            .custody
            .iter()
            .map(|entry| entry.custodian.as_str())
            .collect::<std::collections::HashSet<_>>()
            .len(),
        3
    );
}

#[test]
fn gallowry_is_the_hidden_home_not_the_gallows() {
    let constitution = canonical_constitution().unwrap();
    assert_eq!(constitution.gallowry().id.as_str(), SITE_GALLOWRY);
    assert_eq!(
        constitution.gallowry().controlled_by.as_str(),
        INSTITUTION_GALLOWS
    );
    assert_ne!(
        constitution.gallowry().id,
        constitution.node_by_key(INSTITUTION_GALLOWS).unwrap().id
    );
    assert!(
        constitution
            .audit()
            .unwrap()
            .gallowry_is_distinct_from_gallows
    );
}

#[test]
fn missing_node_is_rejected() {
    let mut parts = canonical_parts();
    parts
        .nodes
        .retain(|node| node.id.as_str() != INSTITUTION_MYSTERY_MEN);
    let constitution = FlyntConstitution::from_parts(parts).unwrap();

    assert_eq!(
        constitution.validate(),
        Err(ConstitutionError::MissingCanonicalNode(
            INSTITUTION_MYSTERY_MEN
        ))
    );
}

#[test]
fn duplicate_authority_is_rejected() {
    let mut parts = canonical_parts();
    parts.nodes.push(parts.nodes[0].clone());
    let duplicate = parts.nodes[0].id.clone();
    let constitution = FlyntConstitution::from_parts(parts).unwrap();

    assert_eq!(
        constitution.validate(),
        Err(ConstitutionError::DuplicateNode(duplicate))
    );
}

#[test]
fn hierarchy_mutation_is_rejected() {
    let mut parts = canonical_parts();
    parts
        .nodes
        .iter_mut()
        .find(|node| node.id.as_str() == INSTITUTION_MANTICORP)
        .unwrap()
        .superior = Some(id(COMPANION_CHIMERA));
    let constitution = FlyntConstitution::from_parts(parts).unwrap();

    assert_eq!(
        constitution.validate(),
        Err(ConstitutionError::CanonicalNodeMismatch(id(
            INSTITUTION_MANTICORP
        )))
    );
}

#[test]
fn incomplete_chimera_recipe_is_rejected() {
    let mut parts = canonical_parts();
    parts.chimera_recipe.sources.pop();
    let constitution = FlyntConstitution::from_parts(parts).unwrap();

    assert_eq!(
        constitution.validate(),
        Err(ConstitutionError::InvalidChimeraRecipe)
    );
}

#[test]
fn canonical_rows_are_deterministic_and_have_no_duplicate_children() {
    let rows = canonical_hierarchy_rows();
    let children: std::collections::HashSet<_> = rows.iter().map(|row| row.0).collect();
    assert_eq!(rows.len(), children.len());
    assert_eq!(rows, canonical_hierarchy_rows());
}

#[test]
fn canonical_validation_is_independent_of_roster_insertion_order() {
    let mut parts = canonical_parts();
    parts.nodes.reverse();
    parts.lineages.reverse();
    parts.forms.reverse();
    parts.chimera_recipe.sources.reverse();

    FlyntConstitution::from_parts(parts)
        .unwrap()
        .validate()
        .unwrap();
}
