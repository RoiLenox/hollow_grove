//! Typed Hollow Grove practitioner ontology.
//!
//! This module is semantic House law above the neutral recursion kernel. It is
//! intentionally separate from the V1 constitutional `Current`/`Aura`
//! quantities, power-root archives, and replay fields, whose names retain their
//! historical physical meaning for compatibility.

use crate::hollow_grove_contract::House;
use crate::world::hueman_faculties::HuemanFaculty;
use crate::world::permanence::PermanenceLaw;

pub const PRACTITIONER_ONTOLOGY_SOURCE: &str = "HOLLOW_GROVE_SYNTHESIS_CANON_V1.md";
pub const FLYNT_SYNTHESIS_AMENDMENT_SOURCE: &str =
    "HOLLOW_GROVE_CANON_AMENDMENT_FLYNT_SYNTHESIS_SANDMANOR_HANDS_V1.md";
pub const PRACTITIONER_LEVEL_ONE_EQUATION: &str = "CURRENT + AURA → SYNTHESIS";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SemanticStatus {
    Locked,
    Working,
    Open,
    Legacy,
    Deprecated,
    Superseded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PractitionerAxis {
    Current,
    Aura,
    Synthesis,
}

pub const PRACTITIONER_INPUT_AXES: [PractitionerAxis; 2] =
    [PractitionerAxis::Current, PractitionerAxis::Aura];
pub const PRACTITIONER_SYNTHESIS_OUTPUT: PractitionerAxis = PractitionerAxis::Synthesis;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PractitionerDefinition {
    pub refers_to_whole_person: bool,
    pub level_one_inputs: [PractitionerAxis; 2],
    pub level_one_output: PractitionerAxis,
    pub level_one_equation: &'static str,
    pub status: SemanticStatus,
}

pub const PRACTITIONER_DEFINITION: PractitionerDefinition = PractitionerDefinition {
    refers_to_whole_person: true,
    level_one_inputs: PRACTITIONER_INPUT_AXES,
    level_one_output: PRACTITIONER_SYNTHESIS_OUTPUT,
    level_one_equation: PRACTITIONER_LEVEL_ONE_EQUATION,
    status: SemanticStatus::Locked,
};

/// Returns the authoritative semantic definition used by runtime consumers.
///
/// This accessor keeps callers on the semantic registry rather than on legacy
/// physical/accounting representations that happen to retain Current/Aura
/// field names.
#[must_use]
pub fn practitioner_definition() -> &'static PractitionerDefinition {
    &PRACTITIONER_DEFINITION
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SynthesisLevel {
    Practitioner,
    HigherComposite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HouseDomain {
    Body,
    Soul,
    Mind,
    Spirit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HouseTopology {
    StonebendPolarityAndLadder,
    SandmanorParallelTraditions,
    GlaushouseSplitAndReunify,
    FlyntThreeTraditionsAndHigherSynthesis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SynthesisForm {
    Gremlin,
    Goblin,
    Ghoul,
    Spectre,
    Troll,
    Ork,
    Ogre,
    Troglodyte,
    Gnome,
    Faun,
    Minotaur,
    Elf,
    Satyr,
    Centaur,
    Pixie,
    Sprite,
    Fairy,
    Spirit,
    Siren,
    Sylph,
    Nymph,
    Muse,
    Gargoyle,
    Werewolf,
    Merman,
    Chimera,
    Manticore,
    Seer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LegacySynthesisForm {
    Pixy,
    Faerie,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SandmanorTradition {
    Minorian,
    Minoan,
}

pub const STONEBEND_ACTIVE_SYNTHESIS_LADDER: [SynthesisForm; 8] = [
    SynthesisForm::Gremlin,
    SynthesisForm::Goblin,
    SynthesisForm::Ghoul,
    SynthesisForm::Spectre,
    SynthesisForm::Troll,
    SynthesisForm::Ork,
    SynthesisForm::Ogre,
    SynthesisForm::Troglodyte,
];

pub const GLAUSHOUSE_ACTIVE_SYNTHESIS_LADDER: [SynthesisForm; 8] = [
    SynthesisForm::Pixie,
    SynthesisForm::Sprite,
    SynthesisForm::Fairy,
    SynthesisForm::Spirit,
    SynthesisForm::Siren,
    SynthesisForm::Sylph,
    SynthesisForm::Nymph,
    SynthesisForm::Muse,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SynthesisLevelPairing {
    pub level: u8,
    pub current: SynthesisForm,
    pub aura: SynthesisForm,
}

pub const HOLLOW_GROVE_SYNTHESIS_LEVEL_PAIRINGS: [SynthesisLevelPairing; 8] = [
    SynthesisLevelPairing {
        level: 1,
        current: SynthesisForm::Gremlin,
        aura: SynthesisForm::Pixie,
    },
    SynthesisLevelPairing {
        level: 2,
        current: SynthesisForm::Goblin,
        aura: SynthesisForm::Sprite,
    },
    SynthesisLevelPairing {
        level: 3,
        current: SynthesisForm::Ghoul,
        aura: SynthesisForm::Fairy,
    },
    SynthesisLevelPairing {
        level: 4,
        current: SynthesisForm::Spectre,
        aura: SynthesisForm::Spirit,
    },
    SynthesisLevelPairing {
        level: 5,
        current: SynthesisForm::Troll,
        aura: SynthesisForm::Siren,
    },
    SynthesisLevelPairing {
        level: 6,
        current: SynthesisForm::Ork,
        aura: SynthesisForm::Sylph,
    },
    SynthesisLevelPairing {
        level: 7,
        current: SynthesisForm::Ogre,
        aura: SynthesisForm::Nymph,
    },
    SynthesisLevelPairing {
        level: 8,
        current: SynthesisForm::Troglodyte,
        aura: SynthesisForm::Muse,
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GlaushouseBranch {
    Foundation,
    Biokinetic,
    Noomantic,
    Reunified,
    Apex,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GlaushouseCorrespondence {
    pub institution_or_rank: &'static str,
    pub synthesis: SynthesisForm,
    pub branch: GlaushouseBranch,
    pub advanced_synthesis: bool,
}

pub const GLAUSHOUSE_CORRESPONDENCES: [GlaushouseCorrespondence; 7] = [
    GlaushouseCorrespondence {
        institution_or_rank: "Nightingale",
        synthesis: SynthesisForm::Pixie,
        branch: GlaushouseBranch::Foundation,
        advanced_synthesis: false,
    },
    GlaushouseCorrespondence {
        institution_or_rank: "Marshal",
        synthesis: SynthesisForm::Sprite,
        branch: GlaushouseBranch::Biokinetic,
        advanced_synthesis: false,
    },
    GlaushouseCorrespondence {
        institution_or_rank: "Marshal",
        synthesis: SynthesisForm::Spirit,
        branch: GlaushouseBranch::Biokinetic,
        advanced_synthesis: true,
    },
    GlaushouseCorrespondence {
        institution_or_rank: "Matron",
        synthesis: SynthesisForm::Fairy,
        branch: GlaushouseBranch::Noomantic,
        advanced_synthesis: false,
    },
    GlaushouseCorrespondence {
        institution_or_rank: "Matron",
        synthesis: SynthesisForm::Nymph,
        branch: GlaushouseBranch::Noomantic,
        advanced_synthesis: true,
    },
    GlaushouseCorrespondence {
        institution_or_rank: "Persephone",
        synthesis: SynthesisForm::Siren,
        branch: GlaushouseBranch::Reunified,
        advanced_synthesis: true,
    },
    GlaushouseCorrespondence {
        institution_or_rank: "Prima Donna",
        synthesis: SynthesisForm::Muse,
        branch: GlaushouseBranch::Apex,
        advanced_synthesis: true,
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlyntPractitionerTradition {
    Warlock,
    Lycan,
    Mariner,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FlyntBaseSynthesis {
    pub tradition: FlyntPractitionerTradition,
    pub synthesis: SynthesisForm,
    pub working_function: &'static str,
    pub function_status: SemanticStatus,
    pub exact_level_one_equation: Option<&'static str>,
    pub equation_status: SemanticStatus,
}

pub const FLYNT_BASE_SYNTHESES: [FlyntBaseSynthesis; 3] = [
    FlyntBaseSynthesis {
        tradition: FlyntPractitionerTradition::Warlock,
        synthesis: SynthesisForm::Gargoyle,
        working_function: "HOLD",
        function_status: SemanticStatus::Working,
        exact_level_one_equation: None,
        equation_status: SemanticStatus::Open,
    },
    FlyntBaseSynthesis {
        tradition: FlyntPractitionerTradition::Lycan,
        synthesis: SynthesisForm::Werewolf,
        working_function: "DRIVE",
        function_status: SemanticStatus::Working,
        exact_level_one_equation: None,
        equation_status: SemanticStatus::Open,
    },
    FlyntBaseSynthesis {
        tradition: FlyntPractitionerTradition::Mariner,
        synthesis: SynthesisForm::Merman,
        working_function: "ADAPT",
        function_status: SemanticStatus::Working,
        exact_level_one_equation: None,
        equation_status: SemanticStatus::Open,
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HigherSynthesisNode {
    Form(SynthesisForm),
    FlyntState(FlyntSynthesisState),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlyntSynthesisState {
    Manticorp,
    Chimera,
}

impl FlyntSynthesisState {
    pub const COMBINED: Self = Self::Manticorp;
    pub const PERFECTED: Self = Self::Chimera;

    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::Manticorp => "Manticorp",
            Self::Chimera => "Chimera",
        }
    }

    #[must_use]
    pub const fn nature_id(self) -> &'static str {
        match self {
            Self::Manticorp => "manticorp",
            Self::Chimera => "chimera",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HigherSynthesisRule {
    pub sources: &'static [HigherSynthesisNode],
    pub result: HigherSynthesisNode,
    pub level: SynthesisLevel,
    pub relationship_status: SemanticStatus,
    pub mechanics_status: SemanticStatus,
    pub source: &'static str,
}

const FLYNT_MANTICORP_SOURCES: &[HigherSynthesisNode] = &[
    HigherSynthesisNode::Form(SynthesisForm::Gargoyle),
    HigherSynthesisNode::Form(SynthesisForm::Werewolf),
    HigherSynthesisNode::Form(SynthesisForm::Merman),
];
const FLYNT_CHIMERA_SOURCES: &[HigherSynthesisNode] = &[HigherSynthesisNode::FlyntState(
    FlyntSynthesisState::Manticorp,
)];
const SEER_SOURCES: &[HigherSynthesisNode] = &[
    HigherSynthesisNode::Form(SynthesisForm::Troglodyte),
    HigherSynthesisNode::Form(SynthesisForm::Muse),
];
pub const HIGHER_SYNTHESIS_RULES: [HigherSynthesisRule; 3] = [
    HigherSynthesisRule {
        sources: FLYNT_MANTICORP_SOURCES,
        result: HigherSynthesisNode::FlyntState(FlyntSynthesisState::Manticorp),
        level: SynthesisLevel::HigherComposite,
        relationship_status: SemanticStatus::Locked,
        mechanics_status: SemanticStatus::Open,
        source: FLYNT_SYNTHESIS_AMENDMENT_SOURCE,
    },
    HigherSynthesisRule {
        sources: FLYNT_CHIMERA_SOURCES,
        result: HigherSynthesisNode::FlyntState(FlyntSynthesisState::Chimera),
        level: SynthesisLevel::HigherComposite,
        relationship_status: SemanticStatus::Locked,
        mechanics_status: SemanticStatus::Open,
        source: FLYNT_SYNTHESIS_AMENDMENT_SOURCE,
    },
    HigherSynthesisRule {
        sources: SEER_SOURCES,
        result: HigherSynthesisNode::Form(SynthesisForm::Seer),
        level: SynthesisLevel::HigherComposite,
        relationship_status: SemanticStatus::Locked,
        mechanics_status: SemanticStatus::Open,
        source: PRACTITIONER_ONTOLOGY_SOURCE,
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HouseOntologyDefinition {
    pub house: House,
    pub domain: HouseDomain,
    pub law: PermanenceLaw,
    pub faculties: &'static [HuemanFaculty],
    pub topology: HouseTopology,
}

const STONEBEND_FACULTIES: &[HuemanFaculty] = &[HuemanFaculty::Presynce];
const SANDMANOR_FACULTIES: &[HuemanFaculty] = &[HuemanFaculty::Prefog, HuemanFaculty::Prefig];
const GLAUSHOUSE_FACULTIES: &[HuemanFaculty] = &[HuemanFaculty::Precog];
const FLYNT_FACULTIES: &[HuemanFaculty] = &[HuemanFaculty::Resynce];

pub const HOUSE_ONTOLOGIES: [HouseOntologyDefinition; 4] = [
    HouseOntologyDefinition {
        house: House::Stonebend,
        domain: HouseDomain::Body,
        law: PermanenceLaw::Identity,
        faculties: STONEBEND_FACULTIES,
        topology: HouseTopology::StonebendPolarityAndLadder,
    },
    HouseOntologyDefinition {
        house: House::Sandmanor,
        domain: HouseDomain::Soul,
        law: PermanenceLaw::Pattern,
        faculties: SANDMANOR_FACULTIES,
        topology: HouseTopology::SandmanorParallelTraditions,
    },
    HouseOntologyDefinition {
        house: House::Glaushouse,
        domain: HouseDomain::Mind,
        law: PermanenceLaw::Integrity,
        faculties: GLAUSHOUSE_FACULTIES,
        topology: HouseTopology::GlaushouseSplitAndReunify,
    },
    HouseOntologyDefinition {
        house: House::Flynt,
        domain: HouseDomain::Spirit,
        law: PermanenceLaw::Capacity,
        faculties: FLYNT_FACULTIES,
        topology: HouseTopology::FlyntThreeTraditionsAndHigherSynthesis,
    },
];

pub const STONEBEND_SOVEREIGN_TITLE: &str = "Hypergiant";
pub const STONEBEND_DIAMOND_STATUS: SemanticStatus = SemanticStatus::Open;
pub const HOLLOW_GROVE_LEVEL_PAIRING_STATUS: SemanticStatus = SemanticStatus::Locked;
pub const STONEBEND_CURRENT_AURA_MAPPING_STATUS: SemanticStatus = HOLLOW_GROVE_LEVEL_PAIRING_STATUS;
pub const SANDMANOR_FORM_TRANSITION_MECHANICS_STATUS: SemanticStatus = SemanticStatus::Open;
pub const FLYNT_CIVIC_PEOPLE_STATUS: SemanticStatus = SemanticStatus::Open;

#[must_use]
pub fn house_ontology(house: House) -> &'static HouseOntologyDefinition {
    HOUSE_ONTOLOGIES
        .iter()
        .find(|definition| definition.house == house)
        .expect("all four Houses have ontology definitions")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn universal_equation_does_not_invent_sandmanor_form_mappings() {
        let definition = practitioner_definition();
        assert!(definition.refers_to_whole_person);
        assert_eq!(definition.status, SemanticStatus::Locked);
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
    fn house_topologies_are_not_normalized() {
        let topologies = HOUSE_ONTOLOGIES
            .iter()
            .map(|definition| definition.topology)
            .collect::<std::collections::HashSet<_>>();
        assert_eq!(topologies.len(), 4);
    }

    #[test]
    fn open_equations_are_absent_instead_of_fabricated() {
        assert!(
            FLYNT_BASE_SYNTHESES
                .iter()
                .all(|entry| entry.exact_level_one_equation.is_none()
                    && entry.equation_status == SemanticStatus::Open)
        );
    }

    #[test]
    fn active_ladders_preserve_all_eight_locked_pairings() {
        assert_eq!(
            STONEBEND_ACTIVE_SYNTHESIS_LADDER,
            [
                SynthesisForm::Gremlin,
                SynthesisForm::Goblin,
                SynthesisForm::Ghoul,
                SynthesisForm::Spectre,
                SynthesisForm::Troll,
                SynthesisForm::Ork,
                SynthesisForm::Ogre,
                SynthesisForm::Troglodyte,
            ]
        );
        assert_eq!(
            GLAUSHOUSE_ACTIVE_SYNTHESIS_LADDER,
            [
                SynthesisForm::Pixie,
                SynthesisForm::Sprite,
                SynthesisForm::Fairy,
                SynthesisForm::Spirit,
                SynthesisForm::Siren,
                SynthesisForm::Sylph,
                SynthesisForm::Nymph,
                SynthesisForm::Muse,
            ]
        );
        for (index, pairing) in HOLLOW_GROVE_SYNTHESIS_LEVEL_PAIRINGS.iter().enumerate() {
            assert_eq!(pairing.level as usize, index + 1);
            assert_eq!(pairing.current, STONEBEND_ACTIVE_SYNTHESIS_LADDER[index]);
            assert_eq!(pairing.aura, GLAUSHOUSE_ACTIVE_SYNTHESIS_LADDER[index]);
        }
    }

    #[test]
    fn higher_synthesis_is_not_level_one() {
        assert!(
            HIGHER_SYNTHESIS_RULES
                .iter()
                .all(|rule| rule.level == SynthesisLevel::HigherComposite)
        );
    }
}
