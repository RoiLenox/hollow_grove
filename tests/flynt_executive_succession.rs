use hollow_grove::institution::{InstitutionalBeingId, OfficeHolder};
use hollow_grove::world::canonical_institutional_world_state;
use hollow_grove::world::flynt::{
    FlyntInstitutions, FlyntValidationError, canonical_flynt_institutions, manticorps_id,
    tross_office_id,
};
use officials_and_outlaws::{
    CanonicalEntries, ConstitutionalOfficeId, ConstitutionalRecognition, ExecutiveMastery,
    FORM_CHIMERA, FORM_GARGOYLE, FORM_MANTICORP, FormId, LawfulAccession, OFFICE_TROSS,
    OfficialsOutlawsRegistry, PERSON_CANONICAL_TROSS_CANDIDATE, PersonId, RegistryError,
    canonical_entries,
};

fn candidate_being(value: &str) -> InstitutionalBeingId {
    InstitutionalBeingId::new(value).expect("test candidate must have a stable being ID")
}

fn entries_before_mastery() -> (
    CanonicalEntries,
    ExecutiveMastery,
    ConstitutionalRecognition,
    LawfulAccession,
) {
    let mut entries = canonical_entries();
    let mastery = entries.executive_masteries.pop().unwrap();
    let recognition = entries.recognitions.pop().unwrap();
    let accession = entries.accessions.pop().unwrap();
    (entries, mastery, recognition, accession)
}

fn assert_tross_activation_rejected(registry: OfficialsOutlawsRegistry, candidate: &str) {
    let mut flynt = FlyntInstitutions::from_succession_registry(registry);
    assert!(matches!(
        flynt.register_lawful_tross_holder(candidate_being(candidate)),
        Err(FlyntValidationError::LawfulTrossAccessionRequired(_))
    ));
    assert!(flynt.catalog.office_holders.is_empty());
}

#[test]
fn direct_tross_assignment_is_rejected_even_when_the_registry_is_canonical() {
    let registry = officials_and_outlaws::canonical_registry().unwrap();
    let mut flynt = FlyntInstitutions::from_succession_registry(registry);
    let candidate = candidate_being(PERSON_CANONICAL_TROSS_CANDIDATE);
    flynt.catalog.office_holders.push(OfficeHolder {
        office: tross_office_id(),
        being: candidate.clone(),
        active: true,
    });

    assert_eq!(
        flynt.validate(),
        Err(FlyntValidationError::DirectTrossAssignmentRejected(
            candidate
        ))
    );
}

#[test]
fn chimera_synthesis_is_required_before_executive_mastery_or_tross() {
    let (mut entries, mastery, _, _) = entries_before_mastery();
    entries.recipes.clear();
    let mut registry = OfficialsOutlawsRegistry::from_entries(entries).unwrap();

    assert!(matches!(
        registry.register_executive_mastery(mastery),
        Err(RegistryError::ChimeraSynthesisRequired)
    ));
    assert_tross_activation_rejected(registry, PERSON_CANONICAL_TROSS_CANDIDATE);
}

#[test]
fn candidate_specific_chimera_refinement_is_required() {
    let (entries, mut mastery, _, _) = entries_before_mastery();
    mastery.refinement.source = FormId::new(FORM_GARGOYLE).unwrap();
    let mut registry = OfficialsOutlawsRegistry::from_entries(entries).unwrap();

    assert!(matches!(
        registry.register_executive_mastery(mastery),
        Err(RegistryError::InvalidExecutiveMastery)
    ));
    assert_tross_activation_rejected(registry, PERSON_CANONICAL_TROSS_CANDIDATE);
}

#[test]
fn refinement_and_succession_records_are_candidate_specific() {
    let (entries, mut mastery, mut recognition, mut accession) = entries_before_mastery();
    let other_candidate = PersonId::new("officials-outlaws.person.other-candidate").unwrap();
    mastery.candidate = other_candidate.clone();
    recognition.candidate = other_candidate.clone();
    accession.candidate = other_candidate;
    let mut registry = OfficialsOutlawsRegistry::from_entries(entries).unwrap();
    registry.register_executive_mastery(mastery).unwrap();
    registry.register_recognition(recognition).unwrap();
    registry.register_accession(accession).unwrap();

    let mut flynt = FlyntInstitutions::from_succession_registry(registry);
    assert!(matches!(
        flynt.register_lawful_tross_holder(candidate_being(PERSON_CANONICAL_TROSS_CANDIDATE)),
        Err(FlyntValidationError::LawfulTrossAccessionRequired(_))
    ));
    flynt
        .register_lawful_tross_holder(candidate_being("officials-outlaws.person.other-candidate"))
        .unwrap();
    flynt.validate().unwrap();
}

#[test]
fn executive_mastery_is_required_and_recognition_cannot_create_it() {
    let (entries, _, recognition, _) = entries_before_mastery();
    let mut registry = OfficialsOutlawsRegistry::from_entries(entries).unwrap();

    assert!(matches!(
        registry.register_recognition(recognition),
        Err(RegistryError::RecognitionRequiresMastery(_))
    ));
    assert!(registry.executive_masteries().is_empty());
    assert!(registry.recognitions().is_empty());
    assert_tross_activation_rejected(registry, PERSON_CANONICAL_TROSS_CANDIDATE);
}

#[test]
fn mastery_alone_does_not_grant_the_tross_office() {
    let (entries, mastery, _, _) = entries_before_mastery();
    let mut registry = OfficialsOutlawsRegistry::from_entries(entries).unwrap();
    registry.register_executive_mastery(mastery).unwrap();

    assert!(registry.recognitions().is_empty());
    assert!(registry.accessions().is_empty());
    assert_tross_activation_rejected(registry, PERSON_CANONICAL_TROSS_CANDIDATE);
}

#[test]
fn recognition_is_required_before_accession() {
    let (entries, mastery, _, accession) = entries_before_mastery();
    let mut registry = OfficialsOutlawsRegistry::from_entries(entries).unwrap();
    registry.register_executive_mastery(mastery).unwrap();

    assert!(matches!(
        registry.register_accession(accession),
        Err(RegistryError::AccessionRequiresRecognition(_))
    ));
    assert_tross_activation_rejected(registry, PERSON_CANONICAL_TROSS_CANDIDATE);
}

#[test]
fn recognition_alone_does_not_grant_office_and_accession_is_required() {
    let (entries, mastery, recognition, _) = entries_before_mastery();
    let mut registry = OfficialsOutlawsRegistry::from_entries(entries).unwrap();
    registry.register_executive_mastery(mastery).unwrap();
    registry.register_recognition(recognition).unwrap();

    assert!(registry.accessions().is_empty());
    assert_tross_activation_rejected(registry, PERSON_CANONICAL_TROSS_CANDIDATE);
}

#[test]
fn lawful_accession_requires_the_existing_matching_recognition() {
    let (entries, mastery, recognition, mut accession) = entries_before_mastery();
    let mut registry = OfficialsOutlawsRegistry::from_entries(entries).unwrap();
    registry.register_executive_mastery(mastery).unwrap();
    registry.register_recognition(recognition).unwrap();
    accession.candidate = PersonId::new("officials-outlaws.person.other-candidate").unwrap();

    assert!(matches!(
        registry.register_accession(accession),
        Err(RegistryError::InvalidLawfulAccession)
    ));
    assert_tross_activation_rejected(registry, PERSON_CANONICAL_TROSS_CANDIDATE);
}

#[test]
fn canonical_world_reaches_tross_only_through_lawful_accession() {
    let flynt = canonical_flynt_institutions();
    flynt.validate().unwrap();
    let candidate = PersonId::new(PERSON_CANONICAL_TROSS_CANDIDATE).unwrap();
    let office = ConstitutionalOfficeId::new(OFFICE_TROSS).unwrap();
    assert!(
        flynt
            .succession_registry()
            .lawfully_holds_office(&candidate, &office)
    );
    assert!(flynt.catalog.office_holders.iter().any(|holder| {
        holder.active
            && holder.office == tross_office_id()
            && holder.being.as_str() == PERSON_CANONICAL_TROSS_CANDIDATE
    }));

    let world = canonical_institutional_world_state();
    assert!(world.catalog.office_holders.iter().any(|holder| {
        holder.active
            && holder.office == tross_office_id()
            && holder.being.as_str() == PERSON_CANONICAL_TROSS_CANDIDATE
    }));
}

#[test]
fn composition_catalog_produces_only_chimera_for_executive_succession() {
    let flynt = canonical_flynt_institutions();
    let registry = flynt.succession_registry();
    let chimera = FormId::new(FORM_CHIMERA).unwrap();
    let manticorp = FormId::new(FORM_MANTICORP).unwrap();
    let tross_node = hollow_grove::CompositionNodeId::new(OFFICE_TROSS).unwrap();

    assert_eq!(
        registry
            .composition_catalog()
            .records_producing_result(chimera.as_kernel())
            .len(),
        1
    );
    assert!(
        registry
            .composition_catalog()
            .records_producing_result(manticorp.as_kernel())
            .is_empty()
    );
    assert!(registry.composition_catalog().node(&tross_node).is_none());
    assert_ne!(manticorps_id().as_str(), manticorp.as_str());
}
