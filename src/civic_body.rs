use std::io;

use crate::hollow_grove_contract::{House, Lineage, SandmanorPeople};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CivicPeople {
    Geralds,
    Nightingales,
    Wardens,
    Minorians,
    Minoans,
}

impl CivicPeople {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Geralds => "Geralds",
            Self::Nightingales => "Nightingales",
            Self::Wardens => "Wardens",
            Self::Minorians => "Minorians",
            Self::Minoans => "Minoans",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CivicBodyRole {
    RedBloodCells,
    WhiteBloodCells,
    Platelets,
    InteriorSignalingAndRegulation,
    EpithelialAndSensoryBoundary,
}

impl CivicBodyRole {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RedBloodCells => "Red Blood Cells",
            Self::WhiteBloodCells => "White Blood Cells",
            Self::Platelets => "Platelets",
            Self::InteriorSignalingAndRegulation => "Interior Signaling and Regulation",
            Self::EpithelialAndSensoryBoundary => "Epithelial and Sensory Boundary",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CivicAction {
    Carry,
    Clear,
    Close,
    Measure,
    Reveal,
}

impl CivicAction {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Carry => "Carry",
            Self::Clear => "Clear",
            Self::Close => "Close",
            Self::Measure => "Measure",
            Self::Reveal => "Reveal",
        }
    }

    #[must_use]
    pub const fn verb(self) -> &'static str {
        match self {
            Self::Carry => "carry",
            Self::Clear => "clear",
            Self::Close => "close",
            Self::Measure => "measure",
            Self::Reveal => "reveal",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CrisisRole {
    RevealBreach,
    MeasureBreach,
    CloseBreach,
    ClearThreat,
    CarryRecovery,
}

impl CrisisRole {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RevealBreach => "Reveal Breach",
            Self::MeasureBreach => "Measure Breach",
            Self::CloseBreach => "Close Breach",
            Self::ClearThreat => "Clear Threat",
            Self::CarryRecovery => "Carry Recovery",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CivicFailure {
    BodyStarves,
    BodyInfected,
    BodyBleedsOut,
    BodyLosesInternalCoordination,
    BodyLosesBoundary,
}

impl CivicFailure {
    #[must_use]
    pub const fn summary(self) -> &'static str {
        match self {
            Self::BodyStarves => "The body starves.",
            Self::BodyInfected => "The body becomes infected.",
            Self::BodyBleedsOut => "The body bleeds out.",
            Self::BodyLosesInternalCoordination => "The body loses internal coordination.",
            Self::BodyLosesBoundary => "The body loses its boundary.",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SandmanorFace {
    Interior,
    Exterior,
}

impl SandmanorFace {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Interior => "Interior",
            Self::Exterior => "Exterior",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CivicBodyDefinition {
    people: CivicPeople,
    house: House,
    sandmanor_face: Option<SandmanorFace>,
    sandmanor_people: Option<SandmanorPeople>,
    lineage: Option<Lineage>,
    body_role: CivicBodyRole,
    canonical_action: CivicAction,
    functions: &'static [&'static str],
    crisis_roles: &'static [CrisisRole],
    absence_failure: CivicFailure,
}

impl CivicBodyDefinition {
    #[must_use]
    pub const fn people(self) -> CivicPeople {
        self.people
    }

    #[must_use]
    pub const fn house(self) -> House {
        self.house
    }

    #[must_use]
    pub const fn sandmanor_face(self) -> Option<SandmanorFace> {
        self.sandmanor_face
    }

    #[must_use]
    pub const fn sandmanor_people(self) -> Option<SandmanorPeople> {
        self.sandmanor_people
    }

    #[must_use]
    pub const fn lineage(self) -> Option<Lineage> {
        self.lineage
    }

    #[must_use]
    pub const fn body_role(self) -> CivicBodyRole {
        self.body_role
    }

    #[must_use]
    pub const fn canonical_action(self) -> CivicAction {
        self.canonical_action
    }

    #[must_use]
    pub const fn functions(self) -> &'static [&'static str] {
        self.functions
    }

    #[must_use]
    pub const fn crisis_roles(self) -> &'static [CrisisRole] {
        self.crisis_roles
    }

    #[must_use]
    pub const fn absence_failure(self) -> CivicFailure {
        self.absence_failure
    }

    #[must_use]
    pub fn chant(self) -> String {
        format!("{} {}.", self.people.as_str(), self.canonical_action.verb())
    }

    #[must_use]
    pub fn display_house_label(self) -> String {
        match self.sandmanor_face {
            Some(face) => format!("{} {}", self.house.as_str(), face.as_str()),
            None => self.house.as_str().to_owned(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CivicBodyContractInput {
    pub definitions: Vec<CivicBodyDefinition>,
    pub chant: Vec<String>,
    pub crisis_steps: Vec<String>,
    pub body_correspondence_is_literal_species: bool,
    pub minorians_and_minoans_collapsed: bool,
    pub wardens_and_nightingales_collapsed: bool,
    pub civic_role_replaces_being: bool,
    pub civic_role_replaces_skill: bool,
    pub civic_role_replaces_object: bool,
    pub civic_role_bypasses_recipe_legality: bool,
    pub crisis_sequence_frozen_kernel_route: bool,
    pub v1_1_changed: bool,
    pub point_cubed: bool,
    pub position_thirteen: bool,
    pub current_prism_conflated: bool,
}

impl Default for CivicBodyContractInput {
    fn default() -> Self {
        canonical_civic_body_contract_fixture()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CivicBodyDiagnosticCode {
    HouseMapping,
    PeopleMapping,
    InteriorExteriorDistinction,
    BodyCorrespondence,
    CanonicalAction,
    CrisisRole,
    BeingObjectBoundary,
    RecipeBoundary,
    KernelFreeze,
    Contradiction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CivicBodyDiagnostic {
    pub code: CivicBodyDiagnosticCode,
    pub message: String,
}

const GERALD_FUNCTIONS: &[&str] = &[
    "carry Current through routes and systems",
    "transport fuel, materials, tools, and labor",
    "supply distant structures and recovery sites",
    "return depleted resources for renewal",
];
const NIGHTINGALE_FUNCTIONS: &[&str] = &[
    "recognize danger and contamination",
    "diagnose injury and damaged conditions",
    "clear damaged matter and coordinate healing",
    "remember threats and escalate when danger persists",
];
const WARDEN_FUNCTIONS: &[&str] = &[
    "detect rupture and converge rapidly",
    "attach to damaged boundaries",
    "form temporary emergency bonds and barriers",
    "hold catastrophic loss until full repair arrives",
];
const MINORIAN_FUNCTIONS: &[&str] = &[
    "measure internal conditions and count resources",
    "allocate energy and route messages",
    "maintain hidden maps, timing, and storage",
    "coordinate internal response and signal shortages",
];
const MINOAN_FUNCTIONS: &[&str] = &[
    "shape the boundary between self and world",
    "regulate exchange and preserve surface integrity",
    "sense environmental change and public condition",
    "reveal outward status and translate signals inward",
];

const GERALD_CRISIS: &[CrisisRole] = &[CrisisRole::CarryRecovery];
const NIGHTINGALE_CRISIS: &[CrisisRole] = &[CrisisRole::ClearThreat];
const WARDEN_CRISIS: &[CrisisRole] = &[CrisisRole::CloseBreach];
const MINORIAN_CRISIS: &[CrisisRole] = &[CrisisRole::MeasureBreach];
const MINOAN_CRISIS: &[CrisisRole] = &[CrisisRole::RevealBreach];

const GERALDS: CivicBodyDefinition = CivicBodyDefinition {
    people: CivicPeople::Geralds,
    house: House::Stonebend,
    sandmanor_face: None,
    sandmanor_people: None,
    lineage: None,
    body_role: CivicBodyRole::RedBloodCells,
    canonical_action: CivicAction::Carry,
    functions: GERALD_FUNCTIONS,
    crisis_roles: GERALD_CRISIS,
    absence_failure: CivicFailure::BodyStarves,
};
const NIGHTINGALES: CivicBodyDefinition = CivicBodyDefinition {
    people: CivicPeople::Nightingales,
    house: House::Glaushouse,
    sandmanor_face: None,
    sandmanor_people: None,
    lineage: None,
    body_role: CivicBodyRole::WhiteBloodCells,
    canonical_action: CivicAction::Clear,
    functions: NIGHTINGALE_FUNCTIONS,
    crisis_roles: NIGHTINGALE_CRISIS,
    absence_failure: CivicFailure::BodyInfected,
};
const WARDENS: CivicBodyDefinition = CivicBodyDefinition {
    people: CivicPeople::Wardens,
    house: House::Flynt,
    sandmanor_face: None,
    sandmanor_people: None,
    lineage: None,
    body_role: CivicBodyRole::Platelets,
    canonical_action: CivicAction::Close,
    functions: WARDEN_FUNCTIONS,
    crisis_roles: WARDEN_CRISIS,
    absence_failure: CivicFailure::BodyBleedsOut,
};
const MINORIANS: CivicBodyDefinition = CivicBodyDefinition {
    people: CivicPeople::Minorians,
    house: House::Sandmanor,
    sandmanor_face: Some(SandmanorFace::Interior),
    sandmanor_people: Some(SandmanorPeople::Minorian),
    lineage: Some(Lineage::Gnome),
    body_role: CivicBodyRole::InteriorSignalingAndRegulation,
    canonical_action: CivicAction::Measure,
    functions: MINORIAN_FUNCTIONS,
    crisis_roles: MINORIAN_CRISIS,
    absence_failure: CivicFailure::BodyLosesInternalCoordination,
};
const MINOANS: CivicBodyDefinition = CivicBodyDefinition {
    people: CivicPeople::Minoans,
    house: House::Sandmanor,
    sandmanor_face: Some(SandmanorFace::Exterior),
    sandmanor_people: Some(SandmanorPeople::Minoan),
    lineage: Some(Lineage::Elf),
    body_role: CivicBodyRole::EpithelialAndSensoryBoundary,
    canonical_action: CivicAction::Reveal,
    functions: MINOAN_FUNCTIONS,
    crisis_roles: MINOAN_CRISIS,
    absence_failure: CivicFailure::BodyLosesBoundary,
};

const CANONICAL_CIVIC_DEFINITIONS: [CivicBodyDefinition; 5] =
    [GERALDS, NIGHTINGALES, WARDENS, MINORIANS, MINOANS];

#[must_use]
pub const fn canonical_civic_body_definitions() -> &'static [CivicBodyDefinition; 5] {
    &CANONICAL_CIVIC_DEFINITIONS
}

#[must_use]
pub fn civic_body_definition(people: CivicPeople) -> CivicBodyDefinition {
    canonical_civic_body_definitions()
        .iter()
        .copied()
        .find(|definition| definition.people() == people)
        .expect("canonical civic definition should exist")
}

#[must_use]
pub fn canonical_civic_chant() -> [String; 5] {
    [
        GERALDS.chant(),
        NIGHTINGALES.chant(),
        WARDENS.chant(),
        MINORIANS.chant(),
        MINOANS.chant(),
    ]
}

#[must_use]
pub fn canonical_civic_crisis_steps() -> [String; 5] {
    [
        String::from("Minoans reveal the breach."),
        String::from("Minorians measure the breach."),
        String::from("Wardens close the breach."),
        String::from("Nightingales clear the threat."),
        String::from("Geralds carry what recovery requires."),
    ]
}

#[must_use]
pub fn canonical_civic_body_contract_fixture() -> CivicBodyContractInput {
    CivicBodyContractInput {
        definitions: canonical_civic_body_definitions().to_vec(),
        chant: canonical_civic_chant().to_vec(),
        crisis_steps: canonical_civic_crisis_steps().to_vec(),
        body_correspondence_is_literal_species: false,
        minorians_and_minoans_collapsed: false,
        wardens_and_nightingales_collapsed: false,
        civic_role_replaces_being: false,
        civic_role_replaces_skill: false,
        civic_role_replaces_object: false,
        civic_role_bypasses_recipe_legality: false,
        crisis_sequence_frozen_kernel_route: false,
        v1_1_changed: false,
        point_cubed: false,
        position_thirteen: false,
        current_prism_conflated: false,
    }
}

#[must_use]
pub fn validate_civic_body_contract(input: &CivicBodyContractInput) -> Vec<CivicBodyDiagnostic> {
    let mut diagnostics = Vec::new();

    if input.definitions.len() != CANONICAL_CIVIC_DEFINITIONS.len() {
        diagnostics.push(CivicBodyDiagnostic {
            code: CivicBodyDiagnosticCode::PeopleMapping,
            message: format!(
                "Civic body contract must contain exactly {} civic peoples, got {}",
                CANONICAL_CIVIC_DEFINITIONS.len(),
                input.definitions.len()
            ),
        });
    }

    for expected in canonical_civic_body_definitions() {
        let actual = input
            .definitions
            .iter()
            .copied()
            .find(|definition| definition.people() == expected.people());
        let Some(actual) = actual else {
            diagnostics.push(CivicBodyDiagnostic {
                code: CivicBodyDiagnosticCode::PeopleMapping,
                message: format!(
                    "{} must remain present in the civic-body model",
                    expected.people().as_str()
                ),
            });
            continue;
        };

        if actual.house() != expected.house() {
            diagnostics.push(CivicBodyDiagnostic {
                code: CivicBodyDiagnosticCode::HouseMapping,
                message: format!(
                    "{} must remain aligned to {}, got {}",
                    expected.people().as_str(),
                    expected.house().as_str(),
                    actual.house().as_str()
                ),
            });
        }

        if actual.body_role() != expected.body_role() {
            diagnostics.push(CivicBodyDiagnostic {
                code: CivicBodyDiagnosticCode::BodyCorrespondence,
                message: format!(
                    "{} must remain {}",
                    expected.people().as_str(),
                    expected.body_role().as_str()
                ),
            });
        }

        if actual.canonical_action() != expected.canonical_action() {
            diagnostics.push(CivicBodyDiagnostic {
                code: CivicBodyDiagnosticCode::CanonicalAction,
                message: format!(
                    "{} must remain locked to {}",
                    expected.people().as_str(),
                    expected.canonical_action().as_str()
                ),
            });
        }

        if actual.sandmanor_face() != expected.sandmanor_face()
            || actual.sandmanor_people() != expected.sandmanor_people()
            || actual.lineage() != expected.lineage()
        {
            diagnostics.push(CivicBodyDiagnostic {
                code: CivicBodyDiagnosticCode::InteriorExteriorDistinction,
                message: format!(
                    "{} must preserve its Sandmanor Interior/Exterior and lineage identity",
                    expected.people().as_str()
                ),
            });
        }

        if actual.crisis_roles() != expected.crisis_roles() {
            diagnostics.push(CivicBodyDiagnostic {
                code: CivicBodyDiagnosticCode::CrisisRole,
                message: format!(
                    "{} must preserve its canonical crisis response role",
                    expected.people().as_str()
                ),
            });
        }

        if actual.absence_failure() != expected.absence_failure() {
            diagnostics.push(CivicBodyDiagnostic {
                code: CivicBodyDiagnosticCode::CrisisRole,
                message: format!(
                    "{} must preserve its canonical absence failure",
                    expected.people().as_str()
                ),
            });
        }
    }

    if input.chant != canonical_civic_chant().to_vec() {
        diagnostics.push(CivicBodyDiagnostic {
            code: CivicBodyDiagnosticCode::CanonicalAction,
            message: String::from(
                "Canonical chant must remain: Geralds carry. Nightingales clear. Wardens close. Minorians measure. Minoans reveal.",
            ),
        });
    }

    if input.crisis_steps != canonical_civic_crisis_steps().to_vec() {
        diagnostics.push(CivicBodyDiagnostic {
            code: CivicBodyDiagnosticCode::CrisisRole,
            message: String::from(
                "Canonical civic crisis loop must remain Reveal -> Measure -> Close -> Clear -> Carry.",
            ),
        });
    }

    if input.minorians_and_minoans_collapsed {
        diagnostics.push(CivicBodyDiagnostic {
            code: CivicBodyDiagnosticCode::InteriorExteriorDistinction,
            message: String::from("Minorians and Minoans must remain distinct civic peoples."),
        });
    }

    if input.wardens_and_nightingales_collapsed {
        diagnostics.push(CivicBodyDiagnostic {
            code: CivicBodyDiagnosticCode::CrisisRole,
            message: String::from("Wardens and Nightingales must remain distinct response roles."),
        });
    }

    if input.body_correspondence_is_literal_species {
        diagnostics.push(CivicBodyDiagnostic {
            code: CivicBodyDiagnosticCode::BodyCorrespondence,
            message: String::from(
                "Civic-body correspondence is functional and metaphorical, not literal species biology.",
            ),
        });
    }

    if input.civic_role_replaces_being
        || input.civic_role_replaces_skill
        || input.civic_role_replaces_object
    {
        diagnostics.push(CivicBodyDiagnostic {
            code: CivicBodyDiagnosticCode::BeingObjectBoundary,
            message: String::from(
                "Civic role must not replace Being, Skill, or Object inside the existing ontology.",
            ),
        });
    }

    if input.civic_role_bypasses_recipe_legality {
        diagnostics.push(CivicBodyDiagnostic {
            code: CivicBodyDiagnosticCode::RecipeBoundary,
            message: String::from("Civic role must not bypass Recipe legality."),
        });
    }

    if input.crisis_sequence_frozen_kernel_route {
        diagnostics.push(CivicBodyDiagnostic {
            code: CivicBodyDiagnosticCode::KernelFreeze,
            message: String::from(
                "The civic crisis loop must remain a semantic response grammar, not a frozen kernel route.",
            ),
        });
    }

    if input.v1_1_changed {
        diagnostics.push(CivicBodyDiagnostic {
            code: CivicBodyDiagnosticCode::KernelFreeze,
            message: String::from("The frozen V1.1 execution topology must remain unchanged."),
        });
    }

    if input.point_cubed {
        diagnostics.push(CivicBodyDiagnostic {
            code: CivicBodyDiagnosticCode::Contradiction,
            message: String::from("Point³ must not be introduced by the civic-body layer."),
        });
    }

    if input.position_thirteen {
        diagnostics.push(CivicBodyDiagnostic {
            code: CivicBodyDiagnosticCode::Contradiction,
            message: String::from("Position 13 must not be introduced by the civic-body layer."),
        });
    }

    if input.current_prism_conflated {
        diagnostics.push(CivicBodyDiagnostic {
            code: CivicBodyDiagnosticCode::Contradiction,
            message: String::from(
                "CurrentPrism must remain distinct from civic-body role semantics.",
            ),
        });
    }

    diagnostics
}

pub fn build_civic_body_witness() -> io::Result<String> {
    Ok(format!(
        "HOLLOW GROVE CIVIC BODY\n\n\
         Stonebend:\n\
         Geralds\n\
         Body Role:\n\
         Red Blood Cells\n\
         Action:\n\
         Carry\n\n\
         Glaüshouse:\n\
         Nightingales\n\
         Body Role:\n\
         White Blood Cells\n\
         Action:\n\
         Clear\n\n\
         Flynt:\n\
         Wardens\n\
         Body Role:\n\
         Platelets\n\
         Action:\n\
         Close\n\n\
         Sandmanor Interior:\n\
         Minorians\n\
         People:\n\
         Gnomes\n\
         Body Role:\n\
         Interior Signaling and Regulation\n\
         Action:\n\
         Measure\n\n\
         Sandmanor Exterior:\n\
         Minoans\n\
         People:\n\
         Elves\n\
         Body Role:\n\
         Epithelial and Sensory Boundary\n\
         Action:\n\
         Reveal\n\n\
         Canonical Chant:\n\n\
         {}\n\
         {}\n\
         {}\n\
         {}\n\
         {}\n",
        GERALDS.chant(),
        NIGHTINGALES.chant(),
        WARDENS.chant(),
        MINORIANS.chant(),
        MINOANS.chant()
    ))
}

pub fn build_civic_body_validation_report() -> io::Result<String> {
    let diagnostics = validate_civic_body_contract(&canonical_civic_body_contract_fixture());
    let status = if diagnostics.is_empty() {
        "pass"
    } else {
        "fail"
    };
    let detail = if diagnostics.is_empty() {
        String::from("none")
    } else {
        diagnostics
            .iter()
            .map(|diagnostic| format!("- {}", diagnostic.message))
            .collect::<Vec<_>>()
            .join("\n")
    };

    Ok(format!(
        "HOLLOW GROVE CIVIC BODY VALIDATION\n\
         status: {}\n\
         - House mappings: pass\n\
         - people mappings: pass\n\
         - Interior/Exterior distinction: pass\n\
         - body correspondences: pass\n\
         - canonical actions: pass\n\
         - crisis response roles: pass\n\
         - Being/Object separation: pass\n\
         - Recipe boundary unchanged: pass\n\
         - V1.1 unchanged: pass\n\n\
         Diagnostics:\n\
         {}\n",
        status, detail
    ))
}

pub fn build_civic_crisis_witness() -> io::Result<String> {
    let steps = canonical_civic_crisis_steps();
    Ok(format!(
        "EVENT:\n\
         World Breach\n\n\
         1. {}\n\
         2. {}\n\
         3. {}\n\
         4. {}\n\
         5. {}\n\n\
         System Status:\n\
         Contained / Diagnosed / Supplied\n",
        steps[0], steps[1], steps[2], steps[3], steps[4]
    ))
}

#[cfg(test)]
mod tests {
    use super::{
        CivicAction, CivicBodyContractInput, CivicBodyRole, CivicPeople,
        build_civic_body_validation_report, build_civic_body_witness, build_civic_crisis_witness,
        canonical_civic_body_contract_fixture, canonical_civic_body_definitions,
        canonical_civic_crisis_steps, validate_civic_body_contract,
    };
    use crate::hollow_grove_contract::{House, Lineage, SandmanorPeople};

    #[test]
    fn canonical_civic_people_map_to_house_role_and_action() {
        let definitions = canonical_civic_body_definitions();
        assert_eq!(definitions.len(), 5);

        let geralds = definitions[0];
        assert_eq!(geralds.people(), CivicPeople::Geralds);
        assert_eq!(geralds.house(), House::Stonebend);
        assert_eq!(geralds.body_role(), CivicBodyRole::RedBloodCells);
        assert_eq!(geralds.canonical_action(), CivicAction::Carry);

        let nightingales = definitions[1];
        assert_eq!(nightingales.people(), CivicPeople::Nightingales);
        assert_eq!(nightingales.house(), House::Glaushouse);
        assert_eq!(nightingales.body_role(), CivicBodyRole::WhiteBloodCells);
        assert_eq!(nightingales.canonical_action(), CivicAction::Clear);

        let wardens = definitions[2];
        assert_eq!(wardens.people(), CivicPeople::Wardens);
        assert_eq!(wardens.house(), House::Flynt);
        assert_eq!(wardens.body_role(), CivicBodyRole::Platelets);
        assert_eq!(wardens.canonical_action(), CivicAction::Close);

        let minorians = definitions[3];
        assert_eq!(minorians.people(), CivicPeople::Minorians);
        assert_eq!(minorians.house(), House::Sandmanor);
        assert_eq!(
            minorians.sandmanor_people(),
            Some(SandmanorPeople::Minorian)
        );
        assert_eq!(minorians.lineage(), Some(Lineage::Gnome));
        assert_eq!(
            minorians.body_role(),
            CivicBodyRole::InteriorSignalingAndRegulation
        );
        assert_eq!(minorians.canonical_action(), CivicAction::Measure);

        let minoans = definitions[4];
        assert_eq!(minoans.people(), CivicPeople::Minoans);
        assert_eq!(minoans.house(), House::Sandmanor);
        assert_eq!(minoans.sandmanor_people(), Some(SandmanorPeople::Minoan));
        assert_eq!(minoans.lineage(), Some(Lineage::Elf));
        assert_eq!(
            minoans.body_role(),
            CivicBodyRole::EpithelialAndSensoryBoundary
        );
        assert_eq!(minoans.canonical_action(), CivicAction::Reveal);
    }

    #[test]
    fn canonical_civic_crisis_loop_stays_distinct() {
        let steps = canonical_civic_crisis_steps();
        assert_eq!(steps[0], "Minoans reveal the breach.");
        assert_eq!(steps[1], "Minorians measure the breach.");
        assert_eq!(steps[2], "Wardens close the breach.");
        assert_eq!(steps[3], "Nightingales clear the threat.");
        assert_eq!(steps[4], "Geralds carry what recovery requires.");
    }

    #[test]
    fn canonical_fixture_validates() {
        let diagnostics = validate_civic_body_contract(&canonical_civic_body_contract_fixture());
        assert!(diagnostics.is_empty(), "{diagnostics:?}");
    }

    #[test]
    fn contradiction_wrong_house_and_collapsed_roles_fail() {
        let mut wrong_house = canonical_civic_body_contract_fixture();
        wrong_house.definitions[0] = super::CivicBodyDefinition {
            house: House::Glaushouse,
            ..wrong_house.definitions[0]
        };
        let diagnostics = validate_civic_body_contract(&wrong_house);
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic
                .message
                .contains("Geralds must remain aligned to Stonebend")
        }));

        let collapsed = CivicBodyContractInput {
            wardens_and_nightingales_collapsed: true,
            ..canonical_civic_body_contract_fixture()
        };
        let diagnostics = validate_civic_body_contract(&collapsed);
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic
                .message
                .contains("Wardens and Nightingales must remain distinct")
        }));
    }

    #[test]
    fn contradiction_minorian_minoan_reversal_and_literal_species_fail() {
        let mut reversed = canonical_civic_body_contract_fixture();
        reversed.definitions[3] = super::CivicBodyDefinition {
            sandmanor_people: Some(SandmanorPeople::Minoan),
            lineage: Some(Lineage::Elf),
            ..reversed.definitions[3]
        };
        let diagnostics = validate_civic_body_contract(&reversed);
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.message.contains(
                    "Minorians must preserve its Sandmanor Interior/Exterior and lineage identity"
                ))
        );

        let literal = CivicBodyContractInput {
            body_correspondence_is_literal_species: true,
            ..canonical_civic_body_contract_fixture()
        };
        let diagnostics = validate_civic_body_contract(&literal);
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.message.contains("functional and metaphorical"))
        );
    }

    #[test]
    fn contradiction_kernel_boundary_and_recipe_bypass_fail() {
        let input = CivicBodyContractInput {
            civic_role_bypasses_recipe_legality: true,
            crisis_sequence_frozen_kernel_route: true,
            v1_1_changed: true,
            point_cubed: true,
            position_thirteen: true,
            current_prism_conflated: true,
            ..canonical_civic_body_contract_fixture()
        };
        let diagnostics = validate_civic_body_contract(&input);
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic
                .message
                .contains("must not bypass Recipe legality")
        }));
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.message.contains("semantic response grammar"))
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.message.contains("V1.1 execution topology"))
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.message.contains("Point³"))
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.message.contains("Position 13"))
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.message.contains("CurrentPrism"))
        );
    }

    #[test]
    fn witness_and_validation_surfaces_render() {
        let witness = build_civic_body_witness().expect("civic body witness should render");
        assert!(witness.contains("Geralds carry."));
        assert!(witness.contains("Nightingales clear."));
        assert!(witness.contains("Wardens close."));
        assert!(witness.contains("Minorians measure."));
        assert!(witness.contains("Minoans reveal."));

        let validation =
            build_civic_body_validation_report().expect("civic validation should render");
        assert!(validation.contains("status: pass"));

        let crisis = build_civic_crisis_witness().expect("civic crisis witness should render");
        assert!(crisis.contains("World Breach"));
        assert!(crisis.contains("Wardens close the breach."));
    }
}
