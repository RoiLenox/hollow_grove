use hollow_grove::FrameId;
use hollow_grove::lineage_contract::{
    MINOAN_SIDE, MINORIAN_SIDE, SANDMANOR_SOVEREIGN, SandmanorAttendantClass,
    SandmanorCulturalDomain, SandmanorForm, SandmanorLineage, SandmanorSocialRole,
    SandmanorSovereignHand, SandmanorSovereignIdentity,
};
use hollow_grove::world::glaushouse::{
    CLINICAL_SYNTHESIS_CORRESPONDENCES, ClinicalSynthesisForm, GLAUSHOUSE_AURA_EXPRESSION,
};
use hollow_grove::world::practitioner_ontology::{
    FLYNT_BASE_SYNTHESES, GLAUSHOUSE_ACTIVE_SYNTHESIS_LADDER, GlaushouseBranch,
    HIGHER_SYNTHESIS_RULES, HOLLOW_GROVE_LEVEL_PAIRING_STATUS,
    HOLLOW_GROVE_SYNTHESIS_LEVEL_PAIRINGS, HigherSynthesisNode, PRACTITIONER_INPUT_AXES,
    PRACTITIONER_LEVEL_ONE_EQUATION, PRACTITIONER_SYNTHESIS_OUTPUT, PractitionerAxis,
    SANDMANOR_FORM_TRANSITION_MECHANICS_STATUS, STONEBEND_ACTIVE_SYNTHESIS_LADDER,
    STONEBEND_SOVEREIGN_TITLE, SemanticStatus, SynthesisForm, SynthesisLevel,
    practitioner_definition,
};

#[test]
fn practitioner_contains_current_and_aura_and_manifests_level_one_synthesis() {
    assert!(practitioner_definition().refers_to_whole_person);
    assert_eq!(
        PRACTITIONER_INPUT_AXES,
        [PractitionerAxis::Current, PractitionerAxis::Aura]
    );
    assert_eq!(PRACTITIONER_SYNTHESIS_OUTPUT, PractitionerAxis::Synthesis);
    assert_eq!(
        PRACTITIONER_LEVEL_ONE_EQUATION,
        "CURRENT + AURA → SYNTHESIS"
    );
    assert_eq!(
        SANDMANOR_FORM_TRANSITION_MECHANICS_STATUS,
        SemanticStatus::Open
    );
}

#[test]
fn sandmanor_keeps_forms_roles_attendants_and_hands_on_distinct_axes() {
    assert_ne!(MINORIAN_SIDE.people, MINOAN_SIDE.people);
    assert_eq!(MINORIAN_SIDE.people, SandmanorLineage::Minorian);
    assert_eq!(MINOAN_SIDE.people, SandmanorLineage::Minoan);
    assert_eq!(
        MINORIAN_SIDE.forms,
        [
            SandmanorForm::Gnome,
            SandmanorForm::Faun,
            SandmanorForm::Minotaur
        ]
    );
    assert_eq!(
        MINOAN_SIDE.forms,
        [
            SandmanorForm::Elf,
            SandmanorForm::Satyr,
            SandmanorForm::Centaur
        ]
    );
    assert_eq!(
        MINORIAN_SIDE.social_roles,
        [
            SandmanorSocialRole::Husbandman,
            SandmanorSocialRole::Householder,
            SandmanorSocialRole::Hearthkeeper,
        ]
    );
    assert_eq!(
        MINOAN_SIDE.social_roles,
        [
            SandmanorSocialRole::BeachBum,
            SandmanorSocialRole::PartyBoy,
            SandmanorSocialRole::Socialite,
        ]
    );
    assert_eq!(
        MINORIAN_SIDE.cultural_domains,
        [
            SandmanorCulturalDomain::ContentFarm,
            SandmanorCulturalDomain::AuraField,
        ]
    );
    assert_eq!(
        MINOAN_SIDE.cultural_domains,
        [SandmanorCulturalDomain::AuraBeach]
    );
    assert_eq!(
        MINORIAN_SIDE.attendant_class,
        SandmanorAttendantClass::Geomancer
    );
    assert_eq!(
        MINOAN_SIDE.attendant_class,
        SandmanorAttendantClass::Labyrinthian
    );
    assert_eq!(
        MINORIAN_SIDE.sovereign_hand,
        SandmanorSovereignHand::Hecaton
    );
    assert_eq!(MINOAN_SIDE.sovereign_hand, SandmanorSovereignHand::Pegasus);
    assert_eq!(
        SANDMANOR_SOVEREIGN,
        SandmanorSovereignIdentity::SandmanDemiurge
    );
    assert!(FrameId::Hecaton.active_canonical_form().is_none());
    assert!(FrameId::Pegasus.active_canonical_form().is_none());
}

#[test]
fn glaushouse_uses_exact_spelling_and_branch_reunification() {
    assert_eq!(GLAUSHOUSE_AURA_EXPRESSION, "Noomancy");
    assert!(CLINICAL_SYNTHESIS_CORRESPONDENCES.iter().any(|entry| {
        entry.form == ClinicalSynthesisForm::Spirit && entry.advanced_branch_form
    }));
    assert!(
        CLINICAL_SYNTHESIS_CORRESPONDENCES
            .iter()
            .any(|entry| { entry.form == ClinicalSynthesisForm::Fairy })
    );
    assert!(
        HIGHER_SYNTHESIS_RULES
            .iter()
            .all(|rule| rule.level == SynthesisLevel::HigherComposite)
    );
    let branches = hollow_grove::world::practitioner_ontology::GLAUSHOUSE_CORRESPONDENCES;
    assert!(
        branches
            .iter()
            .any(|entry| entry.branch == GlaushouseBranch::Reunified)
    );
}

#[test]
fn all_eight_current_and_aura_levels_use_the_locked_pairing() {
    assert_eq!(STONEBEND_SOVEREIGN_TITLE, "Hypergiant");
    assert_eq!(HOLLOW_GROVE_LEVEL_PAIRING_STATUS, SemanticStatus::Locked);
    assert_eq!(STONEBEND_ACTIVE_SYNTHESIS_LADDER.len(), 8);
    assert_eq!(GLAUSHOUSE_ACTIVE_SYNTHESIS_LADDER.len(), 8);
    assert_eq!(HOLLOW_GROVE_SYNTHESIS_LEVEL_PAIRINGS.len(), 8);
    assert_eq!(
        FrameId::Spectre.active_canonical_form(),
        Some(FrameId::Spectre)
    );
    assert_eq!(FrameId::Pixy.active_canonical_form(), Some(FrameId::Pixie));
    assert_eq!(
        FrameId::Faerie.active_canonical_form(),
        Some(FrameId::Fairy)
    );
    for (index, pairing) in HOLLOW_GROVE_SYNTHESIS_LEVEL_PAIRINGS.iter().enumerate() {
        assert_eq!(pairing.level as usize, index + 1);
        assert_eq!(pairing.current, STONEBEND_ACTIVE_SYNTHESIS_LADDER[index]);
        assert_eq!(pairing.aura, GLAUSHOUSE_ACTIVE_SYNTHESIS_LADDER[index]);
    }
}

#[test]
fn flynt_open_equations_are_not_fabricated_and_base_forms_remain() {
    assert_eq!(FLYNT_BASE_SYNTHESES.len(), 3);
    assert!(FLYNT_BASE_SYNTHESES.iter().all(|entry| {
        entry.exact_level_one_equation.is_none() && entry.equation_status == SemanticStatus::Open
    }));
}

#[test]
fn seer_remains_higher_cross_house_synthesis() {
    let seer = HIGHER_SYNTHESIS_RULES
        .iter()
        .find(|rule| rule.result == HigherSynthesisNode::Form(SynthesisForm::Seer))
        .unwrap();
    assert_eq!(seer.level, SynthesisLevel::HigherComposite);
    assert_eq!(
        seer.sources,
        &[
            HigherSynthesisNode::Form(SynthesisForm::Troglodyte),
            HigherSynthesisNode::Form(SynthesisForm::Muse),
        ]
    );
}
