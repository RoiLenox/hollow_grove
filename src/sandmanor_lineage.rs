//! Sandmanor's matched Frame lineages.
//!
//! This is a typed legality and form-description contract. It reuses the
//! existing `FrameId` grammar and does not select actions or create a separate
//! transformation engine.

use crate::frame_state::FrameId;
use crate::hollow_grove_contract::House;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SandmanorLineage {
    Minorian,
    Minoan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SandmanorStage {
    Origin,
    Synthesis,
    Mastery,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SandmanorDomain {
    Interior,
    Exterior,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SandmanorSynthesisSource {
    GnomeAndBull,
    ElfAndHorse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SandmanorFormTrait {
    Interior,
    Exterior,
    Craft,
    Cultivation,
    Precision,
    Bull,
    Strength,
    Fieldwork,
    Endurance,
    FourArmed,
    ParallelAction,
    Construction,
    Harvest,
    Defense,
    MaterialMastery,
    Travel,
    Perception,
    Adaptability,
    Horse,
    Speed,
    CoastalRange,
    Winged,
    Flight,
    HorizonTravel,
    AerialRange,
    MovementMastery,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SandmanorForm {
    Gnome,
    Minotaur,
    Hecaton,
    Elf,
    Centaur,
    Pegasus,
}

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
            Self::Minotaur => FrameId::Minotaur,
            Self::Hecaton => FrameId::Hecaton,
            Self::Elf => FrameId::Elf,
            Self::Centaur => FrameId::Centaur,
            Self::Pegasus => FrameId::Pegasus,
        }
    }

    #[must_use]
    pub const fn from_frame(frame: FrameId) -> Option<Self> {
        match frame {
            FrameId::Gnome => Some(Self::Gnome),
            FrameId::Minotaur => Some(Self::Minotaur),
            FrameId::Hecaton => Some(Self::Hecaton),
            FrameId::Elf => Some(Self::Elf),
            FrameId::Centaur => Some(Self::Centaur),
            FrameId::Pegasus => Some(Self::Pegasus),
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
            Self::Gnome | Self::Minotaur | Self::Hecaton => SandmanorLineage::Minorian,
            Self::Elf | Self::Centaur | Self::Pegasus => SandmanorLineage::Minoan,
        }
    }

    #[must_use]
    pub const fn stage(self) -> SandmanorStage {
        match self {
            Self::Gnome | Self::Elf => SandmanorStage::Origin,
            Self::Minotaur | Self::Centaur => SandmanorStage::Synthesis,
            Self::Hecaton | Self::Pegasus => SandmanorStage::Mastery,
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
            Self::Minotaur => Some(Self::Gnome),
            Self::Hecaton => Some(Self::Minotaur),
            Self::Centaur => Some(Self::Elf),
            Self::Pegasus => Some(Self::Centaur),
        }
    }

    #[must_use]
    pub const fn next_legal_form(self) -> Option<Self> {
        match self {
            Self::Gnome => Some(Self::Minotaur),
            Self::Minotaur => Some(Self::Hecaton),
            Self::Elf => Some(Self::Centaur),
            Self::Centaur => Some(Self::Pegasus),
            Self::Hecaton | Self::Pegasus => None,
        }
    }

    #[must_use]
    pub const fn matched_counterpart(self) -> Self {
        match self {
            Self::Gnome => Self::Elf,
            Self::Minotaur => Self::Centaur,
            Self::Hecaton => Self::Pegasus,
            Self::Elf => Self::Gnome,
            Self::Centaur => Self::Minotaur,
            Self::Pegasus => Self::Hecaton,
        }
    }

    #[must_use]
    pub const fn synthesis_source(self) -> Option<SandmanorSynthesisSource> {
        match self {
            Self::Gnome | Self::Elf => None,
            Self::Minotaur | Self::Hecaton => Some(SandmanorSynthesisSource::GnomeAndBull),
            Self::Centaur | Self::Pegasus => Some(SandmanorSynthesisSource::ElfAndHorse),
        }
    }

    #[must_use]
    pub const fn arm_count(self) -> Option<u8> {
        match self {
            Self::Hecaton => Some(4),
            _ => None,
        }
    }

    #[must_use]
    pub const fn traits(self) -> &'static [SandmanorFormTrait] {
        use SandmanorFormTrait::*;
        match self {
            Self::Gnome => &[Interior, Craft, Cultivation, Precision],
            Self::Minotaur => &[Bull, Strength, Fieldwork, Endurance],
            Self::Hecaton => &[
                FourArmed,
                ParallelAction,
                Construction,
                Harvest,
                Defense,
                MaterialMastery,
            ],
            Self::Elf => &[Exterior, Travel, Perception, Adaptability],
            Self::Centaur => &[Horse, Speed, CoastalRange, Endurance],
            Self::Pegasus => &[Winged, Flight, HorizonTravel, AerialRange, MovementMastery],
        }
    }
}

/// Validates only the two adjacent transitions in either matched lineage.
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
    fn matched_lineages_are_legal_and_equal_at_mastery() {
        assert_eq!(SandmanorForm::Gnome.lineage(), SandmanorLineage::Minorian);
        assert_eq!(SandmanorForm::Elf.lineage(), SandmanorLineage::Minoan);
        assert_eq!(
            SandmanorForm::Hecaton.stage(),
            SandmanorForm::Pegasus.stage()
        );
        assert_eq!(SandmanorForm::Hecaton.stage(), SandmanorStage::Mastery);
        assert_eq!(
            SandmanorForm::Hecaton.matched_counterpart(),
            SandmanorForm::Pegasus
        );
    }

    #[test]
    fn each_lineage_accepts_only_its_adjacent_transition() {
        assert_eq!(
            validate_sandmanor_transition(FrameId::Gnome, FrameId::Minotaur),
            Ok(())
        );
        assert_eq!(
            validate_sandmanor_transition(FrameId::Minotaur, FrameId::Hecaton),
            Ok(())
        );
        assert_eq!(
            validate_sandmanor_transition(FrameId::Elf, FrameId::Centaur),
            Ok(())
        );
        assert_eq!(
            validate_sandmanor_transition(FrameId::Centaur, FrameId::Pegasus),
            Ok(())
        );
        assert_eq!(
            validate_sandmanor_transition(FrameId::Gnome, FrameId::Centaur),
            Err(SandmanorTransitionError::CrossLineage)
        );
        assert_eq!(
            validate_sandmanor_transition(FrameId::Minotaur, FrameId::Pegasus),
            Err(SandmanorTransitionError::CrossLineage)
        );
    }

    #[test]
    fn hecaton_is_four_armed_capacity_not_literal_hundred_arms() {
        assert_eq!(SandmanorForm::Hecaton.arm_count(), Some(4));
        assert!(
            SandmanorForm::Hecaton
                .traits()
                .contains(&SandmanorFormTrait::FourArmed)
        );
        assert!(
            !SandmanorForm::Hecaton
                .traits()
                .contains(&SandmanorFormTrait::Flight)
        );
    }
}
