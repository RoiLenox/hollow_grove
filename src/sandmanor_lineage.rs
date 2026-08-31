//! Sandmanor's parallel people, Form, social-role, institutional, and sovereign
//! ontology.
//!
//! These axes are deliberately distinct. In particular, a mythological Form
//! does not create a social role, attendant office, sovereign Hand, or the
//! Sandman office. Detailed biological and metaphysical transition mechanics
//! remain open canon.

use crate::frame_state::FrameId;
use crate::hollow_grove_contract::House;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SandmanorLineage {
    Minorian,
    Minoan,
}

impl SandmanorLineage {
    #[must_use]
    pub const fn people_name(self) -> &'static str {
        match self {
            Self::Minorian => "Minorian",
            Self::Minoan => "Minoan",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SandmanorStage {
    Base,
    Intermediate,
    Mature,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SandmanorDomain {
    Interior,
    Exterior,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SandmanorForm {
    Gnome,
    Faun,
    Minotaur,
    Elf,
    Satyr,
    Centaur,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SandmanorSocialRole {
    Husbandman,
    Householder,
    Hearthkeeper,
    BeachBum,
    PartyBoy,
    Socialite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SandmanorCulturalDomain {
    ContentFarm,
    AuraField,
    AuraBeach,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SandmanorAttendantClass {
    Geomancer,
    Labyrinthian,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SandmanorSovereignHand {
    Hecaton,
    Pegasus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SandmanorSovereignIdentity {
    SandmanDemiurge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanonStatus {
    Locked,
    Open,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SandmanorSideDefinition {
    pub people: SandmanorLineage,
    pub forms: [SandmanorForm; 3],
    pub cultural_domains: &'static [SandmanorCulturalDomain],
    pub social_roles: [SandmanorSocialRole; 3],
    pub attendant_class: SandmanorAttendantClass,
    pub sovereign_hand: SandmanorSovereignHand,
}

const MINORIAN_CULTURAL_DOMAINS: &[SandmanorCulturalDomain] = &[
    SandmanorCulturalDomain::ContentFarm,
    SandmanorCulturalDomain::AuraField,
];
const MINOAN_CULTURAL_DOMAINS: &[SandmanorCulturalDomain] = &[SandmanorCulturalDomain::AuraBeach];

pub const MINORIAN_SIDE: SandmanorSideDefinition = SandmanorSideDefinition {
    people: SandmanorLineage::Minorian,
    forms: [
        SandmanorForm::Gnome,
        SandmanorForm::Faun,
        SandmanorForm::Minotaur,
    ],
    cultural_domains: MINORIAN_CULTURAL_DOMAINS,
    social_roles: [
        SandmanorSocialRole::Husbandman,
        SandmanorSocialRole::Householder,
        SandmanorSocialRole::Hearthkeeper,
    ],
    attendant_class: SandmanorAttendantClass::Geomancer,
    sovereign_hand: SandmanorSovereignHand::Hecaton,
};

pub const MINOAN_SIDE: SandmanorSideDefinition = SandmanorSideDefinition {
    people: SandmanorLineage::Minoan,
    forms: [
        SandmanorForm::Elf,
        SandmanorForm::Satyr,
        SandmanorForm::Centaur,
    ],
    cultural_domains: MINOAN_CULTURAL_DOMAINS,
    social_roles: [
        SandmanorSocialRole::BeachBum,
        SandmanorSocialRole::PartyBoy,
        SandmanorSocialRole::Socialite,
    ],
    attendant_class: SandmanorAttendantClass::Labyrinthian,
    sovereign_hand: SandmanorSovereignHand::Pegasus,
};

pub const SANDMANOR_SOVEREIGN: SandmanorSovereignIdentity =
    SandmanorSovereignIdentity::SandmanDemiurge;
pub const HECATON_APPOINTMENT_STATUS: CanonStatus = CanonStatus::Open;
pub const PEGASUS_APPOINTMENT_STATUS: CanonStatus = CanonStatus::Open;
pub const SOVEREIGN_HAND_SUCCESSION_STATUS: CanonStatus = CanonStatus::Open;
pub const DETAILED_FORM_TRANSITION_MECHANICS_STATUS: CanonStatus = CanonStatus::Open;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SandmanorTransitionError {
    NotSandmanorForm,
    CrossLineage,
    IllegalStageSkip,
}

impl SandmanorForm {
    #[must_use]
    pub const fn frame(self) -> FrameId {
        match self {
            Self::Gnome => FrameId::Gnome,
            Self::Faun => FrameId::Faun,
            Self::Minotaur => FrameId::Minotaur,
            Self::Elf => FrameId::Elf,
            Self::Satyr => FrameId::Satyr,
            Self::Centaur => FrameId::Centaur,
        }
    }

    #[must_use]
    pub const fn from_frame(frame: FrameId) -> Option<Self> {
        match frame {
            FrameId::Gnome => Some(Self::Gnome),
            FrameId::Faun => Some(Self::Faun),
            FrameId::Minotaur => Some(Self::Minotaur),
            FrameId::Elf => Some(Self::Elf),
            FrameId::Satyr => Some(Self::Satyr),
            FrameId::Centaur => Some(Self::Centaur),
            _ => None,
        }
    }

    #[must_use]
    pub const fn house(self) -> House {
        House::Sandmanor
    }

    #[must_use]
    pub const fn lineage(self) -> SandmanorLineage {
        match self {
            Self::Gnome | Self::Faun | Self::Minotaur => SandmanorLineage::Minorian,
            Self::Elf | Self::Satyr | Self::Centaur => SandmanorLineage::Minoan,
        }
    }

    #[must_use]
    pub const fn stage(self) -> SandmanorStage {
        match self {
            Self::Gnome | Self::Elf => SandmanorStage::Base,
            Self::Faun | Self::Satyr => SandmanorStage::Intermediate,
            Self::Minotaur | Self::Centaur => SandmanorStage::Mature,
        }
    }

    #[must_use]
    pub const fn domain(self) -> SandmanorDomain {
        match self.lineage() {
            SandmanorLineage::Minorian => SandmanorDomain::Interior,
            SandmanorLineage::Minoan => SandmanorDomain::Exterior,
        }
    }

    #[must_use]
    pub const fn previous_form(self) -> Option<Self> {
        match self {
            Self::Gnome | Self::Elf => None,
            Self::Faun => Some(Self::Gnome),
            Self::Minotaur => Some(Self::Faun),
            Self::Satyr => Some(Self::Elf),
            Self::Centaur => Some(Self::Satyr),
        }
    }

    #[must_use]
    pub const fn next_legal_form(self) -> Option<Self> {
        match self {
            Self::Gnome => Some(Self::Faun),
            Self::Faun => Some(Self::Minotaur),
            Self::Elf => Some(Self::Satyr),
            Self::Satyr => Some(Self::Centaur),
            Self::Minotaur | Self::Centaur => None,
        }
    }

    #[must_use]
    pub const fn matched_counterpart(self) -> Self {
        match self {
            Self::Gnome => Self::Elf,
            Self::Faun => Self::Satyr,
            Self::Minotaur => Self::Centaur,
            Self::Elf => Self::Gnome,
            Self::Satyr => Self::Faun,
            Self::Centaur => Self::Minotaur,
        }
    }
}

/// Validates documented adjacency without defining the underlying biological
/// or metaphysical mechanism.
pub fn validate_sandmanor_transition(
    from: FrameId,
    to: FrameId,
) -> Result<(), SandmanorTransitionError> {
    let Some(from) = SandmanorForm::from_frame(from) else {
        return Err(SandmanorTransitionError::NotSandmanorForm);
    };
    let Some(to) = SandmanorForm::from_frame(to) else {
        return Err(SandmanorTransitionError::NotSandmanorForm);
    };
    if from.lineage() != to.lineage() {
        return Err(SandmanorTransitionError::CrossLineage);
    }
    if from.next_legal_form() != Some(to) {
        return Err(SandmanorTransitionError::IllegalStageSkip);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forms_are_parallel_without_cross_people_evolution() {
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
            SandmanorForm::Faun.matched_counterpart(),
            SandmanorForm::Satyr
        );
        assert_eq!(
            SandmanorForm::Minotaur.matched_counterpart(),
            SandmanorForm::Centaur
        );
        assert_eq!(
            validate_sandmanor_transition(FrameId::Gnome, FrameId::Elf),
            Err(SandmanorTransitionError::CrossLineage)
        );
    }

    #[test]
    fn social_attendant_hand_and_sovereign_axes_are_distinct() {
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
        assert_eq!(HECATON_APPOINTMENT_STATUS, CanonStatus::Open);
        assert_eq!(PEGASUS_APPOINTMENT_STATUS, CanonStatus::Open);
    }

    #[test]
    fn only_adjacent_form_transitions_are_documented() {
        for (from, to) in [
            (FrameId::Gnome, FrameId::Faun),
            (FrameId::Faun, FrameId::Minotaur),
            (FrameId::Elf, FrameId::Satyr),
            (FrameId::Satyr, FrameId::Centaur),
        ] {
            assert_eq!(validate_sandmanor_transition(from, to), Ok(()));
        }
        assert_eq!(
            validate_sandmanor_transition(FrameId::Minotaur, FrameId::Hecaton),
            Err(SandmanorTransitionError::NotSandmanorForm)
        );
        assert_eq!(
            validate_sandmanor_transition(FrameId::Centaur, FrameId::Pegasus),
            Err(SandmanorTransitionError::NotSandmanorForm)
        );
    }
}
