use officials_and_outlaws::*;

fn entity_official(key: &str) -> ConstitutionalEntityId {
    OfficialId::new(key).unwrap().into()
}

fn entity_outlaw(key: &str) -> ConstitutionalEntityId {
    OutlawId::new(key).unwrap().into()
}

fn sorted<'a>(values: impl Iterator<Item = &'a str>) -> Vec<&'a str> {
    let mut values: Vec<_> = values.collect();
    values.sort_unstable();
    values
}

fn registry_before_mastery() -> (
    OfficialsOutlawsRegistry,
    ExecutiveMastery,
    ConstitutionalRecognition,
    LawfulAccession,
) {
    let mut entries = canonical_entries();
    let mastery = entries.executive_masteries.pop().unwrap();
    let recognition = entries.recognitions.pop().unwrap();
    let accession = entries.accessions.pop().unwrap();
    (
        OfficialsOutlawsRegistry::from_entries(entries).unwrap(),
        mastery,
        recognition,
        accession,
    )
}

#[test]
fn manticorp_and_werewolves_are_symmetric_mirrors() {
    let registry = canonical_registry().unwrap();
    assert_eq!(
        registry.mirror_of(&entity_official(OFFICIAL_MANTICORP)),
        Some(entity_outlaw(OUTLAW_WEREWOLVES))
    );
    assert_eq!(
        registry.mirror_of(&entity_outlaw(OUTLAW_WEREWOLVES)),
        Some(entity_official(OFFICIAL_MANTICORP))
    );
}

#[test]
fn mystery_men_and_gallows_are_symmetric_mirrors() {
    let registry = canonical_registry().unwrap();
    assert_eq!(
        registry.mirror_of(&entity_official(OFFICIAL_MYSTERY_MEN)),
        Some(entity_outlaw(OUTLAW_GALLOWS))
    );
    assert_eq!(
        registry.mirror_of(&entity_outlaw(OUTLAW_GALLOWS)),
        Some(entity_official(OFFICIAL_MYSTERY_MEN))
    );
}

#[test]
fn mysteryguard_and_mermen_are_symmetric_mirrors() {
    let registry = canonical_registry().unwrap();
    assert_eq!(
        registry.mirror_of(&entity_official(OFFICIAL_MYSTERYGUARD)),
        Some(entity_outlaw(OUTLAW_MERMEN))
    );
    assert_eq!(
        registry.mirror_of(&entity_outlaw(OUTLAW_MERMEN)),
        Some(entity_official(OFFICIAL_MYSTERYGUARD))
    );
}

#[test]
fn gargoyle_gallows_manticorp_and_chimera_keep_their_entity_kinds_distinct() {
    let registry = canonical_registry().unwrap();
    let gargoyle = LineageId::new(LINEAGE_GARGOYLE).unwrap();
    let gallows = OutlawId::new(OUTLAW_GALLOWS).unwrap();
    let manticorp_institution = OfficialId::new(OFFICIAL_MANTICORP).unwrap();
    let manticorp_form = FormId::new(FORM_MANTICORP).unwrap();
    let chimera_form = FormId::new(FORM_CHIMERA).unwrap();
    let chimera_office = ConstitutionalOfficeId::new(OFFICE_CHIMERA).unwrap();

    assert_ne!(gargoyle.as_str(), gallows.as_str());
    assert_ne!(manticorp_institution.as_str(), manticorp_form.as_str());
    assert_ne!(chimera_form.as_str(), chimera_office.as_str());
    assert!(FormId::new(OFFICIAL_MANTICORP).is_err());
    assert!(ConstitutionalOfficeId::new(FORM_CHIMERA).is_err());
    assert!(PersonId::new(OFFICE_TROSS).is_err());

    let relation = registry
        .lineage_relations()
        .iter()
        .find(|entry| entry.outlaw == gallows)
        .unwrap();
    assert_eq!(relation.lineage, gargoyle);
    assert_eq!(relation.kind, LineageRelationKind::Influence);
}

#[test]
fn chimera_synthesis_is_exact_and_kernel_backed() {
    let registry = canonical_registry().unwrap();
    let recipe_id = SynthesisRecipeId::new(RECIPE_CHIMERA).unwrap();
    let recipe = registry.recipe(&recipe_id).unwrap();

    assert_eq!(
        sorted(recipe.sources.iter().map(FormId::as_str)),
        sorted([FORM_GARGOYLE, FORM_WEREWOLF, FORM_MERMAN].into_iter())
    );
    assert_eq!(recipe.result.as_str(), FORM_CHIMERA);

    let kernel_record = registry
        .composition_catalog()
        .record(recipe_id.as_kernel())
        .unwrap();
    assert_eq!(kernel_record.sources.len(), 3);
    assert_eq!(kernel_record.result.as_str(), FORM_CHIMERA);
}

#[test]
fn executive_mastery_records_lion_eagle_and_hydra_as_perfected_aspects() {
    let registry = canonical_registry().unwrap();
    let mastery = registry.executive_masteries().first().unwrap();
    assert_eq!(mastery.candidate.as_str(), PERSON_CANONICAL_TROSS_CANDIDATE);
    assert_eq!(mastery.completed_chimera.as_str(), FORM_CHIMERA);
    assert_eq!(mastery.refinement.source.as_str(), FORM_CHIMERA);
    assert_eq!(mastery.refinement.result.as_str(), FORM_MANTICORP);
    assert_eq!(mastery.resulting_manticorp.as_str(), FORM_MANTICORP);
    assert_eq!(
        mastery.refinement.perfected_aspects,
        vec![
            ManticorpMasteryAspect::Lion,
            ManticorpMasteryAspect::Eagle,
            ManticorpMasteryAspect::Hydra,
        ]
    );
}

#[test]
fn manticorp_cannot_be_synthesized_directly_from_lion_eagle_and_hydra() {
    let (mut registry, mastery, _, _) = registry_before_mastery();
    let direct_recipe = SynthesisRecipe {
        id: SynthesisRecipeId::new("officials-outlaws.recipe.direct-manticorp").unwrap(),
        sources: ["lion", "eagle", "hydra"]
            .into_iter()
            .map(|aspect| FormId::new(format!("officials-outlaws.form.{aspect}")).unwrap())
            .collect(),
        result: FormId::new(FORM_MANTICORP).unwrap(),
    };
    assert!(matches!(
        registry.register_recipe(direct_recipe),
        Err(RegistryError::ManticorpCannotBeDirectlySynthesized)
    ));
    assert_eq!(registry.recipes().len(), 1);
    assert!(
        registry
            .composition_catalog()
            .records_producing_result(FormId::new(FORM_MANTICORP).unwrap().as_kernel())
            .is_empty()
    );
    assert_eq!(
        mastery.refinement.perfected_aspects,
        vec![
            ManticorpMasteryAspect::Lion,
            ManticorpMasteryAspect::Eagle,
            ManticorpMasteryAspect::Hydra,
        ]
    );
}

#[test]
fn completed_chimera_is_required_for_executive_mastery() {
    let (mut registry, mut mastery, _, _) = registry_before_mastery();
    mastery.completed_chimera = FormId::new(FORM_GARGOYLE).unwrap();
    assert!(matches!(
        registry.register_executive_mastery(mastery),
        Err(RegistryError::InvalidExecutiveMastery)
    ));

    let mut entries = canonical_entries();
    let mastery = entries.executive_masteries.pop().unwrap();
    entries.recognitions.clear();
    entries.accessions.clear();
    entries.recipes.clear();
    let mut registry = OfficialsOutlawsRegistry::from_entries(entries).unwrap();
    assert!(matches!(
        registry.register_executive_mastery(mastery),
        Err(RegistryError::ChimeraSynthesisRequired)
    ));
}

#[test]
fn recognition_without_manticorp_mastery_is_rejected() {
    let (mut registry, _, recognition, _) = registry_before_mastery();
    assert!(matches!(
        registry.register_recognition(recognition),
        Err(RegistryError::RecognitionRequiresMastery(_))
    ));
    assert!(registry.recognitions().is_empty());
}

#[test]
fn mastery_does_not_automatically_grant_the_tross_office() {
    let (mut registry, mastery, _, _) = registry_before_mastery();
    let candidate = mastery.candidate.clone();
    registry.register_executive_mastery(mastery).unwrap();
    let tross = ConstitutionalOfficeId::new(OFFICE_TROSS).unwrap();

    assert!(registry.recognitions().is_empty());
    assert!(registry.accessions().is_empty());
    assert!(!registry.lawfully_holds_office(&candidate, &tross));
}

#[test]
fn recognition_is_required_before_lawful_tross_accession() {
    let (mut registry, mastery, recognition, accession) = registry_before_mastery();
    let candidate = mastery.candidate.clone();
    let tross = ConstitutionalOfficeId::new(OFFICE_TROSS).unwrap();
    registry.register_executive_mastery(mastery).unwrap();

    assert!(matches!(
        registry.register_accession(accession.clone()),
        Err(RegistryError::AccessionRequiresRecognition(_))
    ));
    registry.register_recognition(recognition).unwrap();
    assert!(!registry.lawfully_holds_office(&candidate, &tross));
    registry.register_accession(accession).unwrap();
    assert!(registry.lawfully_holds_office(&candidate, &tross));
}

#[test]
fn recognition_acknowledges_mastery_without_creating_achievement() {
    let (mut registry, mastery, recognition, _) = registry_before_mastery();
    let manticorp = FormId::new(FORM_MANTICORP).unwrap();
    assert!(
        registry
            .composition_catalog()
            .records_producing_result(manticorp.as_kernel())
            .is_empty()
    );

    registry.register_executive_mastery(mastery).unwrap();
    registry.register_recognition(recognition).unwrap();

    assert!(
        registry
            .composition_catalog()
            .records_producing_result(manticorp.as_kernel())
            .is_empty()
    );
    assert!(registry.accessions().is_empty());
}

#[test]
fn tross_office_is_not_a_composition_node_or_result() {
    let registry = canonical_registry().unwrap();
    assert!(FormId::new(OFFICE_TROSS).is_err());
    let tross_node = hollow_grove_kernel::CompositionNodeId::new(OFFICE_TROSS).unwrap();
    assert!(registry.composition_catalog().node(&tross_node).is_none());
    assert!(
        registry
            .composition_catalog()
            .records_producing_result(&tross_node)
            .is_empty()
    );
}

#[test]
fn opposite_registration_order_preserves_ids_mirrors_and_synthesis() {
    let forward = canonical_registry().unwrap();
    let mut reverse_entries = canonical_entries();
    reverse_entries.officials.reverse();
    reverse_entries.outlaws.reverse();
    reverse_entries.mirrors.reverse();
    reverse_entries.lineages.reverse();
    reverse_entries.forms.reverse();
    reverse_entries.synthesis_bases.reverse();
    reverse_entries.lineage_relations.reverse();
    reverse_entries.offices.reverse();
    reverse_entries.recipes.reverse();
    reverse_entries.executive_masteries.reverse();
    reverse_entries.recognitions.reverse();
    reverse_entries.accessions.reverse();
    for recipe in &mut reverse_entries.recipes {
        recipe.sources.reverse();
    }
    for mastery in &mut reverse_entries.executive_masteries {
        mastery.refinement.perfected_aspects.reverse();
        mastery.refinement.evidence.reverse();
    }
    let reverse = OfficialsOutlawsRegistry::from_entries(reverse_entries).unwrap();
    reverse.validate().unwrap();

    assert_eq!(
        sorted(forward.officials().iter().map(|entry| entry.id.as_str())),
        sorted(reverse.officials().iter().map(|entry| entry.id.as_str()))
    );
    assert_eq!(
        sorted(forward.outlaws().iter().map(|entry| entry.id.as_str())),
        sorted(reverse.outlaws().iter().map(|entry| entry.id.as_str()))
    );
    assert_eq!(
        sorted(forward.mirror_pairs().iter().map(|entry| entry.id.as_str())),
        sorted(reverse.mirror_pairs().iter().map(|entry| entry.id.as_str()))
    );
    assert_eq!(
        sorted(forward.lineages().iter().map(|entry| entry.id.as_str())),
        sorted(reverse.lineages().iter().map(|entry| entry.id.as_str()))
    );
    assert_eq!(
        sorted(forward.forms().iter().map(|entry| entry.id.as_str())),
        sorted(reverse.forms().iter().map(|entry| entry.id.as_str()))
    );
    assert_eq!(
        sorted(forward.offices().iter().map(|entry| entry.id.as_str())),
        sorted(reverse.offices().iter().map(|entry| entry.id.as_str()))
    );
    assert_eq!(
        sorted(forward.recipes().iter().map(|entry| entry.id.as_str())),
        sorted(reverse.recipes().iter().map(|entry| entry.id.as_str()))
    );
    for official in [
        OFFICIAL_MANTICORP,
        OFFICIAL_MYSTERY_MEN,
        OFFICIAL_MYSTERYGUARD,
    ] {
        assert_eq!(
            forward.mirror_of(&entity_official(official)),
            reverse.mirror_of(&entity_official(official))
        );
    }
    let recipe = SynthesisRecipeId::new(RECIPE_CHIMERA).unwrap();
    assert_eq!(forward.recipe(&recipe), reverse.recipe(&recipe));
}

#[test]
fn opposite_source_order_is_the_same_canonical_chimera_synthesis() {
    let forward = canonical_registry().unwrap();
    let mut reversed_entries = canonical_entries();
    let chimera = reversed_entries
        .recipes
        .iter_mut()
        .find(|recipe| recipe.id.as_str() == RECIPE_CHIMERA)
        .unwrap();
    chimera.sources.reverse();
    let reversed = OfficialsOutlawsRegistry::from_entries(reversed_entries).unwrap();
    reversed.validate().unwrap();

    let id = SynthesisRecipeId::new(RECIPE_CHIMERA).unwrap();
    assert_eq!(forward.recipe(&id), reversed.recipe(&id));
    assert_eq!(
        forward.composition_catalog().record(id.as_kernel()),
        reversed.composition_catalog().record(id.as_kernel())
    );
}

#[test]
fn mirror_registration_creates_neither_containment_nor_synthesis() {
    let entries = canonical_entries();
    let mut registry = OfficialsOutlawsRegistry::new();
    for official in entries.officials {
        registry.register_official(official).unwrap();
    }
    for outlaw in entries.outlaws {
        registry.register_outlaw(outlaw).unwrap();
    }
    for mirror in entries.mirrors {
        registry.register_mirror_pair(mirror).unwrap();
    }

    assert_eq!(registry.mirror_pairs().len(), 3);
    assert!(registry.recipes().is_empty());
    assert!(registry.forms().is_empty());
    let gargoyle_node = hollow_grove_kernel::CompositionNodeId::new(FORM_GARGOYLE).unwrap();
    assert!(
        registry
            .composition_catalog()
            .node(&gargoyle_node)
            .is_none()
    );
}

#[test]
fn canonical_synthesis_does_not_create_containment() {
    let registry = canonical_registry().unwrap();
    for form in registry.forms() {
        assert!(
            registry
                .composition_catalog()
                .direct_containers(form.id.as_kernel())
                .is_empty()
        );
        assert!(
            registry
                .composition_catalog()
                .direct_members(form.id.as_kernel())
                .is_empty()
        );
    }
}

#[test]
fn duplicate_and_reverse_duplicate_mirrors_are_rejected() {
    let entries = canonical_entries();
    let mut registry = OfficialsOutlawsRegistry::new();
    for official in entries.officials {
        registry.register_official(official).unwrap();
    }
    for outlaw in entries.outlaws {
        registry.register_outlaw(outlaw).unwrap();
    }
    registry
        .register_mirror_pair(entries.mirrors[0].clone())
        .unwrap();

    let reverse_duplicate = registry.register_mirror_between(
        MirrorPairId::new("officials-outlaws.mirror.reverse-duplicate").unwrap(),
        entity_outlaw(OUTLAW_WEREWOLVES),
        entity_official(OFFICIAL_MANTICORP),
        SharedFunction::MartialForce,
        "reverse duplicate",
    );
    assert!(matches!(
        reverse_duplicate,
        Err(RegistryError::DuplicateMirrorPair { .. })
    ));

    let duplicate_id = registry.register_mirror_between(
        MirrorPairId::new(MIRROR_MANTICORP_WEREWOLVES).unwrap(),
        entity_official(OFFICIAL_MYSTERY_MEN),
        entity_outlaw(OUTLAW_GALLOWS),
        SharedFunction::HiddenThreatHuntingInvestigationAndControl,
        "duplicate ID",
    );
    assert!(matches!(
        duplicate_id,
        Err(RegistryError::DuplicateMirrorId(_))
    ));
}

#[test]
fn self_mirrors_and_same_orientation_mirrors_are_rejected() {
    let entries = canonical_entries();
    let mut registry = OfficialsOutlawsRegistry::new();
    for official in entries.officials {
        registry.register_official(official).unwrap();
    }
    for outlaw in entries.outlaws {
        registry.register_outlaw(outlaw).unwrap();
    }
    let manticorp = entity_official(OFFICIAL_MANTICORP);
    assert!(matches!(
        registry.register_mirror_between(
            MirrorPairId::new("officials-outlaws.mirror.self").unwrap(),
            manticorp.clone(),
            manticorp,
            SharedFunction::MartialForce,
            "invalid",
        ),
        Err(RegistryError::SameMirrorEntity(_))
    ));
    assert!(matches!(
        registry.register_mirror_between(
            MirrorPairId::new("officials-outlaws.mirror.same-orientation").unwrap(),
            entity_official(OFFICIAL_MANTICORP),
            entity_official(OFFICIAL_MYSTERY_MEN),
            SharedFunction::MartialForce,
            "invalid",
        ),
        Err(RegistryError::MirrorRequiresOppositeOrientations)
    ));
}
