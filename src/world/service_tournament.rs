//! Canonical Service Tournament law and deterministic scenario runtime.
//!
//! The Tournament is a Central Junction Function conducted through nonlethal
//! simulation. The paired real-world services are external reference models
//! for one identity per House; they do not create institutions, offices,
//! military commands, or additional sovereign factions inside Hollow Grove.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::hollow_grove_contract::House;

use super::central_junction::CentralJunctionFunction;

pub const SERVICE_TOURNAMENT_SOURCE: &str = "SERVICE_TOURNAMENT_CENTRAL_JUNCTION_CANON_V1.md";
pub const SERVICE_TOURNAMENT_SCHEMA_VERSION: &str = "1.0.0";
pub const SERVICE_TOURNAMENT_NAME: &str = "The Service Tournament";
pub const WAR_OF_A_THOUSAND_HUES_NAME: &str = "The War of a Thousand Hues";
pub const WAR_OF_A_THOUSAND_HUES_INFORMAL_NAME: &str = "the Thousand Hues";
pub const SERVICE_TOURNAMENT_PURPOSE: &str =
    "Which House can remain fully itself while still serving the whole of Hollow Grove?";
pub const SERVICE_TOURNAMENT_OPENING: &str = "Let the Houses take their colors. Let Central Junction bear witness. The War of a Thousand Hues has begun.";
pub const THOUSAND_HUES_MAXIM: &str =
    "Four Houses enter with four colors. Central Junction leaves wearing a thousand hues.";
pub const TOURNAMENT_CHARACTER_MAXIM: &str =
    "War performed safely enough to become culture, but seriously enough to expose character.";
pub const CONTAINMENT_JOKE: &str = "This year, the scenarios will remain contained.";

macro_rules! tournament_id {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, ServiceTournamentIdError> {
                let value = value.into();
                if value.is_empty()
                    || !value.bytes().all(|byte| {
                        byte.is_ascii_lowercase()
                            || byte.is_ascii_digit()
                            || matches!(byte, b'.' | b'-')
                    })
                {
                    return Err(ServiceTournamentIdError(value));
                }
                Ok(Self(value))
            }

            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(formatter)
            }
        }

        impl TryFrom<String> for $name {
            type Error = ServiceTournamentIdError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }
    };
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceTournamentIdError(String);

impl fmt::Display for ServiceTournamentIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "invalid Service Tournament stable identifier: {}",
            self.0
        )
    }
}

impl std::error::Error for ServiceTournamentIdError {}

tournament_id!(TournamentId);
tournament_id!(TournamentYearId);
tournament_id!(WarId);
tournament_id!(ScenarioId);
tournament_id!(MarkId);
tournament_id!(ServiceMarkId);
tournament_id!(CompetitorId);
tournament_id!(ObjectiveId);
tournament_id!(ResultId);
tournament_id!(TournamentEventId);
tournament_id!(TournamentLocationId);
tournament_id!(TournamentEvidenceId);
tournament_id!(TournamentAuthorityId);
tournament_id!(ScenarioPhaseId);
tournament_id!(AllianceId);
tournament_id!(EmergencyId);
tournament_id!(ScoringEventId);
tournament_id!(ViolationId);
tournament_id!(PrizeAwardId);
tournament_id!(ArtifactId);
tournament_id!(ArtifactRefinementId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PairedServiceIdentity {
    FlyntAtfArmy,
    StonebendDeaAirForce,
    SandmanorCiaNavy,
    GlaushouseFbiMarines,
}

impl PairedServiceIdentity {
    pub const ALL: [Self; 4] = [
        Self::FlyntAtfArmy,
        Self::StonebendDeaAirForce,
        Self::SandmanorCiaNavy,
        Self::GlaushouseFbiMarines,
    ];

    #[must_use]
    pub const fn house(self) -> House {
        match self {
            Self::FlyntAtfArmy => House::Flynt,
            Self::StonebendDeaAirForce => House::Stonebend,
            Self::SandmanorCiaNavy => House::Sandmanor,
            Self::GlaushouseFbiMarines => House::Glaushouse,
        }
    }

    #[must_use]
    pub const fn house_display_name(self) -> &'static str {
        match self {
            Self::FlyntAtfArmy => "Flynt, MI",
            Self::StonebendDeaAirForce => "Stonebend",
            Self::SandmanorCiaNavy => "Sandmanor",
            Self::GlaushouseFbiMarines => "Glaüshouse",
        }
    }

    #[must_use]
    pub const fn agency_reference(self) -> &'static str {
        match self {
            Self::FlyntAtfArmy => "ATF",
            Self::StonebendDeaAirForce => "DEA",
            Self::SandmanorCiaNavy => "CIA",
            Self::GlaushouseFbiMarines => "FBI",
        }
    }

    #[must_use]
    pub const fn armed_service_reference(self) -> &'static str {
        match self {
            Self::FlyntAtfArmy => "Army",
            Self::StonebendDeaAirForce => "Air Force",
            Self::SandmanorCiaNavy => "Navy",
            Self::GlaushouseFbiMarines => "Marines",
        }
    }

    #[must_use]
    pub const fn paired_reference(self) -> &'static str {
        match self {
            Self::FlyntAtfArmy => "ATF & Army",
            Self::StonebendDeaAirForce => "DEA & Air Force",
            Self::SandmanorCiaNavy => "CIA & Navy",
            Self::GlaushouseFbiMarines => "FBI & Marines",
        }
    }

    #[must_use]
    pub const fn emphases(self) -> &'static [&'static str] {
        match self {
            Self::FlyntAtfArmy => &[
                "combat engineering",
                "controlled mechanisms",
                "breaching",
                "explosives knowledge",
                "fortification",
                "route creation",
                "territorial control",
                "field improvisation",
                "lawful recognition of force",
            ],
            Self::StonebendDeaAirForce => &[
                "classification",
                "chain of custody",
                "tracking substances, Current, identities, and objects",
                "aerial observation",
                "precision",
                "strategic command",
                "boundary control",
                "lawful containment",
                "interdiction",
            ],
            Self::SandmanorCiaNavy => &[
                "intelligence",
                "secrecy",
                "navigation",
                "maritime movement",
                "foreign contact",
                "source cultivation",
                "indirect influence",
                "currents and routes",
                "coastal operations",
                "interpretation of incomplete information",
            ],
            Self::GlaushouseFbiMarines => &[
                "investigation",
                "evidence",
                "forensics",
                "diagnosis",
                "internal defense",
                "rapid deployment",
                "rescue",
                "casualty extraction",
                "close intervention",
                "securing dangerous scenes",
            ],
        }
    }

    #[must_use]
    pub const fn constitutional_compatibility(self) -> &'static [&'static str] {
        match self {
            Self::FlyntAtfArmy => &[
                "Tross",
                "Manticorp",
                "Mystery Men",
                "The Gallows as the unlawful or unrecognized mirror",
            ],
            Self::StonebendDeaAirForce => &[
                "Title and lawful identity",
                "naming and structure",
                "boundary and permanence",
                "lawful Hollowing",
                "the prohibition against Illegal Hollowing",
            ],
            Self::SandmanorCiaNavy => &[
                "Minorians and Minoans",
                "Gnomes, Minotaurs, Elves, and Centaurs",
                "Aura Fields and Aura Beach",
                "Current Sea",
                "the Sandman and the Contest of Improvement",
            ],
            Self::GlaushouseFbiMarines => &[
                "Prima Donna / Doctor Ratchet",
                "Persephone / Nurse House",
                "Nightingales",
                "Chromacord",
                "the Recovery Ward",
                "the prohibition against Illegal Synthesis",
            ],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct HouseServiceProfile {
    pub identity: PairedServiceIdentity,
    pub one_complete_cultural_government_identity: bool,
    pub external_reference_models_only: bool,
    pub preserves_house_government: bool,
    pub creates_separate_service_teams: bool,
}

#[must_use]
pub const fn canonical_house_service_profiles() -> [HouseServiceProfile; 4] {
    [
        HouseServiceProfile {
            identity: PairedServiceIdentity::FlyntAtfArmy,
            one_complete_cultural_government_identity: true,
            external_reference_models_only: true,
            preserves_house_government: true,
            creates_separate_service_teams: false,
        },
        HouseServiceProfile {
            identity: PairedServiceIdentity::StonebendDeaAirForce,
            one_complete_cultural_government_identity: true,
            external_reference_models_only: true,
            preserves_house_government: true,
            creates_separate_service_teams: false,
        },
        HouseServiceProfile {
            identity: PairedServiceIdentity::SandmanorCiaNavy,
            one_complete_cultural_government_identity: true,
            external_reference_models_only: true,
            preserves_house_government: true,
            creates_separate_service_teams: false,
        },
        HouseServiceProfile {
            identity: PairedServiceIdentity::GlaushouseFbiMarines,
            one_complete_cultural_government_identity: true,
            external_reference_models_only: true,
            preserves_house_government: true,
            creates_separate_service_teams: false,
        },
    ]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum HouseColorFamily {
    Blue,
    Red,
    Green,
    Black,
}

impl HouseColorFamily {
    pub const ALL: [Self; 4] = [Self::Blue, Self::Red, Self::Green, Self::Black];

    #[must_use]
    pub const fn for_house(house: House) -> Self {
        match house {
            House::Stonebend => Self::Blue,
            House::Sandmanor => Self::Red,
            House::Glaushouse => Self::Green,
            House::Flynt => Self::Black,
        }
    }

    #[must_use]
    pub const fn house(self) -> House {
        match self {
            Self::Blue => House::Stonebend,
            Self::Red => House::Sandmanor,
            Self::Green => House::Glaushouse,
            Self::Black => House::Flynt,
        }
    }

    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::Blue => "Blue",
            Self::Red => "Red",
            Self::Green => "Green",
            Self::Black => "Black",
        }
    }

    #[must_use]
    pub const fn palette_color_ids(self) -> &'static [&'static str] {
        match self {
            Self::Blue => &[
                "stonebend.prussian_blue",
                "stonebend.lapis_lazuli",
                "stonebend.air_superiority_blue",
            ],
            Self::Red => &["sandmanor.wine", "sandmanor.redwood", "sandmanor.blush"],
            Self::Green => &[
                "glaushouse.brunswick_green",
                "glaushouse.viridian",
                "glaushouse.eton_blue",
            ],
            Self::Black => &[
                "flynt.rich_black_blue",
                "flynt.gunmetal",
                "flynt.powder_blue",
                "flynt.tropical_indigo",
            ],
        }
    }

    #[must_use]
    pub fn accepts_palette_color(self, color_id: &str) -> bool {
        self.palette_color_ids().contains(&color_id)
    }

    #[must_use]
    pub const fn named_variations(self) -> &'static [&'static str] {
        match self {
            Self::Blue => &["Prussian Blue", "Lapis Lazuli", "Air Superiority Blue"],
            Self::Red => &["Wine", "Redwood", "Blush"],
            Self::Green => &["Brunswick Green", "Viridian", "Eton Blue"],
            Self::Black => &[
                "Onyx",
                "Obsidian",
                "Gunmetal",
                "Rich Black Blue",
                "weathered blue-black",
            ],
        }
    }

    #[must_use]
    pub const fn permits_featureless_pure_black(self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MarkGrammar {
    Handprint,
    StraightLine,
    Circle,
    Cross,
    Burst,
    Arrow,
    FourOverlappingHouseMarks,
}

impl MarkGrammar {
    pub const ALL: [Self; 7] = [
        Self::Handprint,
        Self::StraightLine,
        Self::Circle,
        Self::Cross,
        Self::Burst,
        Self::Arrow,
        Self::FourOverlappingHouseMarks,
    ];

    #[must_use]
    pub const fn meaning(self) -> &'static str {
        match self {
            Self::Handprint => "We were here.",
            Self::StraightLine => "Route secured.",
            Self::Circle => "Area defended or protected.",
            Self::Cross => "Rescue, treatment, or medical extraction.",
            Self::Burst => "Breach completed.",
            Self::Arrow => "Advance, pursuit, or directed movement.",
            Self::FourOverlappingHouseMarks => "Joint operation or shared constitutional action.",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MarkAction {
    Reached,
    Held,
    Breached,
    Defended,
    Rescued,
    CrossedRoute,
    AllianceFormed,
    TerritoryChangedHands,
    ArrivedFirst,
    RemainedLast,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SimulationSystem {
    PaintballStyleWeapon,
    MarkingRound,
    ColoredSmoke,
    MockExplosive,
    TrainingVehicle,
    FalseDocument,
    StagedEvidence,
    TemporaryIdentity,
    CodedMessage,
    DroneOrObservationTool,
    MovableBarricade,
    ScenarioCasualty,
    ControlledEnvironmentalChange,
    RescueAndExtractionEquipment,
}

impl SimulationSystem {
    pub const ALL: [Self; 14] = [
        Self::PaintballStyleWeapon,
        Self::MarkingRound,
        Self::ColoredSmoke,
        Self::MockExplosive,
        Self::TrainingVehicle,
        Self::FalseDocument,
        Self::StagedEvidence,
        Self::TemporaryIdentity,
        Self::CodedMessage,
        Self::DroneOrObservationTool,
        Self::MovableBarricade,
        Self::ScenarioCasualty,
        Self::ControlledEnvironmentalChange,
        Self::RescueAndExtractionEquipment,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ScenarioType {
    TerritorialControl,
    BridgeDefense,
    ConvoyEscort,
    EvidenceRecovery,
    WitnessProtection,
    StagedContrabandInvestigation,
    MockKidnapping,
    SabotageDiscovery,
    RescueOperation,
    CasualtyExtraction,
    RouteClearance,
    ControlledBreach,
    AerialObservation,
    NavalOrCanalNavigation,
    Infiltration,
    FalseEmergency,
    DisasterResponse,
    ConstitutionalCrimeInvestigation,
    MultiHouseJointMission,
}

impl ScenarioType {
    pub const ALL: [Self; 19] = [
        Self::TerritorialControl,
        Self::BridgeDefense,
        Self::ConvoyEscort,
        Self::EvidenceRecovery,
        Self::WitnessProtection,
        Self::StagedContrabandInvestigation,
        Self::MockKidnapping,
        Self::SabotageDiscovery,
        Self::RescueOperation,
        Self::CasualtyExtraction,
        Self::RouteClearance,
        Self::ControlledBreach,
        Self::AerialObservation,
        Self::NavalOrCanalNavigation,
        Self::Infiltration,
        Self::FalseEmergency,
        Self::DisasterResponse,
        Self::ConstitutionalCrimeInvestigation,
        Self::MultiHouseJointMission,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ScoringCategory {
    MissionCompletion,
    TrueScenarioRecognition,
    CivilianProtection,
    RescueSuccess,
    EvidenceIntegrity,
    Engineering,
    Mobility,
    TerritorialControl,
    ConstitutionalRestraint,
    Adaptability,
    Cooperation,
    CorrectHouseAuthority,
    ChangedConditionRecognition,
}

impl ScoringCategory {
    pub const ALL: [Self; 13] = [
        Self::MissionCompletion,
        Self::TrueScenarioRecognition,
        Self::CivilianProtection,
        Self::RescueSuccess,
        Self::EvidenceIntegrity,
        Self::Engineering,
        Self::Mobility,
        Self::TerritorialControl,
        Self::ConstitutionalRestraint,
        Self::Adaptability,
        Self::Cooperation,
        Self::CorrectHouseAuthority,
        Self::ChangedConditionRecognition,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ConstitutionalPenalty {
    CivilianEndangerment,
    EvidenceFalsification,
    JurisdictionViolation,
    ExcessiveForce,
    UnnecessaryJunctionDamage,
    RealEmergencyMisclassified,
    ObjectiveDistractionFailure,
    UnconstitutionalCompletion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TournamentAward {
    Honor,
    CeremonialTitle,
    FutureFunctionStewardship,
    TemporaryOperationalResponsibility,
    TournamentStandardOrCupCustody,
    FirstMarkInNextWar,
    PublicRecognition,
}

impl TournamentAward {
    #[must_use]
    pub const fn transfers_permanent_sovereignty(self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ServiceMarkSignificance {
    ImpossibleDefense,
    FamousRescue,
    DecisiveBreach,
    FirstMarkOfWar,
    CompetitionChangingRoute,
    CivilianContribution,
    JointHouseOperation,
    FourHouseCollision,
    ChampionFinalAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TournamentToneStage {
    CeremonialArrival,
    FormalOpening,
    StructuredCompetition,
    RivalriesAndTemporaryAlliances,
    ScenarioSpillover,
    InformalCivilianRoles,
    CityWideImprovisedNetworks,
    FinalConstitutionalTest,
}

impl TournamentToneStage {
    pub const ALL: [Self; 8] = [
        Self::CeremonialArrival,
        Self::FormalOpening,
        Self::StructuredCompetition,
        Self::RivalriesAndTemporaryAlliances,
        Self::ScenarioSpillover,
        Self::InformalCivilianRoles,
        Self::CityWideImprovisedNetworks,
        Self::FinalConstitutionalTest,
    ];
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceTournamentDefinition {
    pub id: TournamentId,
    pub name: &'static str,
    pub function: CentralJunctionFunction,
    pub location_stable_id: &'static str,
    pub central_war: WarId,
    pub representatives: [PairedServiceIdentity; 4],
    pub largest_public_function_at_central_junction: bool,
    pub temporary_shared_operational_arena: bool,
    pub scenarios_may_overlap: bool,
    pub nonlethal_simulation_only: bool,
    pub transfers_permanent_sovereignty: bool,
    pub purpose: &'static str,
    pub constitutional_sources: BTreeSet<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WarDefinition {
    pub id: WarId,
    pub tournament: TournamentId,
    pub official_name: &'static str,
    pub informal_name: &'static str,
    pub nonlethal: bool,
    pub paint_records_presence_and_action: bool,
    pub color_owners: BTreeMap<House, HouseColorFamily>,
}

#[must_use]
pub fn canonical_service_tournament() -> ServiceTournamentDefinition {
    ServiceTournamentDefinition {
        id: TournamentId::new("function.central-junction.service-tournament")
            .expect("canonical Tournament ID"),
        name: SERVICE_TOURNAMENT_NAME,
        function: CentralJunctionFunction::ServiceTournament,
        location_stable_id: "district.central-junction",
        central_war: WarId::new("war.central-junction.thousand-hues")
            .expect("canonical Thousand Hues ID"),
        representatives: PairedServiceIdentity::ALL,
        largest_public_function_at_central_junction: true,
        temporary_shared_operational_arena: true,
        scenarios_may_overlap: true,
        nonlethal_simulation_only: true,
        transfers_permanent_sovereignty: false,
        purpose: SERVICE_TOURNAMENT_PURPOSE,
        constitutional_sources: [
            "HOLLOW_GROVE_COMPROMISE_V1_DRAFT.md",
            "FLYNT_CONSTITUTION_V2.md",
            "STONEBEND_CONSTITUTION_V2.md",
            "SANDMANOR_CONSTITUTION_V2.md",
            "GLAUSHOUSE_CONSTITUTION_V2.md",
        ]
        .into_iter()
        .collect(),
    }
}

#[must_use]
pub fn canonical_war_of_a_thousand_hues() -> WarDefinition {
    let tournament = canonical_service_tournament();
    WarDefinition {
        id: tournament.central_war,
        tournament: tournament.id,
        official_name: WAR_OF_A_THOUSAND_HUES_NAME,
        informal_name: WAR_OF_A_THOUSAND_HUES_INFORMAL_NAME,
        nonlethal: true,
        paint_records_presence_and_action: true,
        color_owners: [
            (House::Stonebend, HouseColorFamily::Blue),
            (House::Sandmanor, HouseColorFamily::Red),
            (House::Glaushouse, HouseColorFamily::Green),
            (House::Flynt, HouseColorFamily::Black),
        ]
        .into_iter()
        .collect(),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TournamentCompetitor {
    pub id: CompetitorId,
    pub tournament: TournamentId,
    pub house: House,
    pub service_identity: PairedServiceIdentity,
    pub public_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TournamentObjective {
    pub id: ObjectiveId,
    pub category: ScoringCategory,
    pub description: String,
    pub available_points: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TournamentScenario {
    pub id: ScenarioId,
    pub tournament: TournamentId,
    pub war: Option<WarId>,
    pub scenario_type: ScenarioType,
    pub operational_zone: TournamentLocationId,
    pub objectives: Vec<TournamentObjective>,
    pub simulation_systems: BTreeSet<SimulationSystem>,
    pub nonlethal: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaintMarkProvenance {
    pub originating_event: TournamentEventId,
    pub competitor: CompetitorId,
    pub evidence: BTreeSet<TournamentEvidenceId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaintMark {
    pub id: MarkId,
    pub war: WarId,
    pub scenario: Option<ScenarioId>,
    pub location: TournamentLocationId,
    pub house: House,
    pub color_family: HouseColorFamily,
    pub palette_sources: BTreeSet<String>,
    pub hue_description: String,
    pub action: MarkAction,
    pub grammar: Option<MarkGrammar>,
    pub layer_sequence: u64,
    pub provenance: PaintMarkProvenance,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceMarkProvenance {
    pub preservation_event: TournamentEventId,
    pub source_marks: BTreeSet<MarkId>,
    pub source_action_events: BTreeSet<TournamentEventId>,
    pub authorized_by: TournamentAuthorityId,
    pub evidence: BTreeSet<TournamentEvidenceId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceMark {
    pub id: ServiceMarkId,
    pub tournament_year_id: TournamentYearId,
    pub war: WarId,
    pub year: u32,
    pub scenario: ScenarioId,
    pub location: TournamentLocationId,
    pub houses: BTreeSet<House>,
    pub operation_name: String,
    pub participants: BTreeSet<CompetitorId>,
    pub significance: ServiceMarkSignificance,
    pub ordered_paint_layers: Vec<MarkId>,
    pub constitutional_significance: String,
    pub account: String,
    pub provenance: ServiceMarkProvenance,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HouseScorecard {
    pub scores: BTreeMap<ScoringCategory, u16>,
    pub penalties: BTreeSet<ConstitutionalPenalty>,
}

impl HouseScorecard {
    #[must_use]
    pub fn total_score(&self) -> u32 {
        self.scores.values().copied().map(u32::from).sum()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TournamentResult {
    pub id: ResultId,
    pub tournament: TournamentId,
    pub champion: House,
    pub scorecards: BTreeMap<House, HouseScorecard>,
    pub award: TournamentAward,
    pub transfers_permanent_sovereignty: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TournamentEventKind {
    CompetitorRegistered(TournamentCompetitor),
    TournamentOpened,
    ScenarioRegistered(TournamentScenario),
    MarkRecorded(PaintMark),
    ServiceMarkPreserved(ServiceMark),
    ResultRecorded(TournamentResult),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TournamentEvent {
    pub id: TournamentEventId,
    pub tournament: TournamentId,
    pub semantic_sequence: u64,
    pub kind: TournamentEventKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LayerMeaning {
    SuccessivePresence,
    FlyntBreachThenStonebendSecurity,
    GlaushouseRescueInsideFlyntControl,
    SandmanorInfiltrationOfStonebend,
    FourHouseContestOrCooperation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LayerReading {
    pub ordered_marks: Vec<MarkId>,
    pub ordered_houses: Vec<House>,
    pub meaning: LayerMeaning,
}

#[must_use]
pub fn read_paint_layers(marks: &[&PaintMark]) -> Option<LayerReading> {
    if marks.is_empty() {
        return None;
    }
    let mut ordered = marks.to_vec();
    ordered.sort_by_key(|mark| (mark.layer_sequence, mark.id.as_str().to_owned()));
    let ordered_houses = ordered.iter().map(|mark| mark.house).collect::<Vec<_>>();
    let houses = ordered_houses.iter().copied().collect::<BTreeSet<_>>();
    let meaning = if houses.len() == 4 {
        LayerMeaning::FourHouseContestOrCooperation
    } else if ordered_houses
        .windows(2)
        .any(|pair| pair == [House::Flynt, House::Stonebend])
    {
        LayerMeaning::FlyntBreachThenStonebendSecurity
    } else if ordered_houses
        .windows(2)
        .any(|pair| pair == [House::Flynt, House::Glaushouse])
    {
        LayerMeaning::GlaushouseRescueInsideFlyntControl
    } else if ordered_houses
        .windows(2)
        .any(|pair| pair == [House::Stonebend, House::Sandmanor])
    {
        LayerMeaning::SandmanorInfiltrationOfStonebend
    } else {
        LayerMeaning::SuccessivePresence
    };
    Some(LayerReading {
        ordered_marks: ordered.iter().map(|mark| mark.id.clone()).collect(),
        ordered_houses,
        meaning,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceTournamentError {
    InvalidCanonicalDefinition,
    WrongTournament,
    WrongWar,
    EventIdConflict(TournamentEventId),
    UnexpectedSequence { expected: u64, actual: u64 },
    RosterClosed,
    DuplicateCompetitor(CompetitorId),
    DuplicateHouseRepresentative(House),
    ServiceIdentityMismatch(CompetitorId),
    IncompleteFourHouseRoster,
    TournamentAlreadyOpen,
    TournamentNotOpen,
    DuplicateScenario(ScenarioId),
    DuplicateObjective(ObjectiveId),
    MissingObjectives(ScenarioId),
    NonlethalSimulationRequired,
    UnknownScenario(ScenarioId),
    DuplicateMark(MarkId),
    UnknownCompetitor(CompetitorId),
    MarkHouseMismatch(MarkId),
    ColorFamilyMismatch(MarkId),
    InvalidPaletteSource(MarkId, String),
    MissingPaintProvenance(MarkId),
    LayerSequenceConflict(TournamentLocationId, u64),
    DuplicateServiceMark(ServiceMarkId),
    UnknownMark(MarkId),
    InvalidServiceMarkProvenance(ServiceMarkId),
    DuplicateResult(ResultId),
    IncompleteScorecard(ResultId),
    InvalidChampion(House),
    PermanentSovereigntyForbidden,
}

impl fmt::Display for ServiceTournamentError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Service Tournament validation failed: {self:?}")
    }
}

impl std::error::Error for ServiceTournamentError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceTournamentRuntime {
    tournament: ServiceTournamentDefinition,
    war: WarDefinition,
    competitors: BTreeMap<CompetitorId, TournamentCompetitor>,
    house_representatives: BTreeMap<House, CompetitorId>,
    scenarios: BTreeMap<ScenarioId, TournamentScenario>,
    objectives: BTreeMap<ObjectiveId, TournamentObjective>,
    marks: BTreeMap<MarkId, PaintMark>,
    service_marks: BTreeMap<ServiceMarkId, ServiceMark>,
    results: BTreeMap<ResultId, TournamentResult>,
    events: BTreeMap<TournamentEventId, TournamentEvent>,
    opened: bool,
}

impl ServiceTournamentRuntime {
    pub fn canonical() -> Result<Self, ServiceTournamentError> {
        Self::new(
            canonical_service_tournament(),
            canonical_war_of_a_thousand_hues(),
        )
    }

    pub fn new(
        tournament: ServiceTournamentDefinition,
        war: WarDefinition,
    ) -> Result<Self, ServiceTournamentError> {
        validate_definition(&tournament, &war)?;
        Ok(Self {
            tournament,
            war,
            competitors: BTreeMap::new(),
            house_representatives: BTreeMap::new(),
            scenarios: BTreeMap::new(),
            objectives: BTreeMap::new(),
            marks: BTreeMap::new(),
            service_marks: BTreeMap::new(),
            results: BTreeMap::new(),
            events: BTreeMap::new(),
            opened: false,
        })
    }

    pub fn replay(events: &[TournamentEvent]) -> Result<Self, ServiceTournamentError> {
        let mut ordered = events.to_vec();
        ordered.sort_by_key(|event| (event.semantic_sequence, event.id.as_str().to_owned()));
        let mut runtime = Self::canonical()?;
        for event in ordered {
            runtime.apply_event(event)?;
        }
        Ok(runtime)
    }

    pub fn apply_event(&mut self, event: TournamentEvent) -> Result<(), ServiceTournamentError> {
        if event.tournament != self.tournament.id {
            return Err(ServiceTournamentError::WrongTournament);
        }
        if self.events.contains_key(&event.id) {
            return Err(ServiceTournamentError::EventIdConflict(event.id));
        }
        let expected = self
            .events
            .values()
            .map(|existing| existing.semantic_sequence)
            .max()
            .map_or(0, |sequence| sequence + 1);
        if event.semantic_sequence != expected {
            return Err(ServiceTournamentError::UnexpectedSequence {
                expected,
                actual: event.semantic_sequence,
            });
        }

        match &event.kind {
            TournamentEventKind::CompetitorRegistered(competitor) => {
                self.register_competitor(competitor)?;
            }
            TournamentEventKind::TournamentOpened => self.open()?,
            TournamentEventKind::ScenarioRegistered(scenario) => {
                self.register_scenario(scenario)?;
            }
            TournamentEventKind::MarkRecorded(mark) => {
                self.record_mark(&event.id, mark)?;
            }
            TournamentEventKind::ServiceMarkPreserved(service_mark) => {
                self.preserve_service_mark(&event.id, service_mark)?;
            }
            TournamentEventKind::ResultRecorded(result) => self.record_result(result)?,
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    #[must_use]
    pub const fn tournament(&self) -> &ServiceTournamentDefinition {
        &self.tournament
    }

    #[must_use]
    pub const fn war(&self) -> &WarDefinition {
        &self.war
    }

    #[must_use]
    pub const fn is_open(&self) -> bool {
        self.opened
    }

    #[must_use]
    pub fn competitors(&self) -> &BTreeMap<CompetitorId, TournamentCompetitor> {
        &self.competitors
    }

    #[must_use]
    pub fn scenarios(&self) -> &BTreeMap<ScenarioId, TournamentScenario> {
        &self.scenarios
    }

    #[must_use]
    pub fn objectives(&self) -> &BTreeMap<ObjectiveId, TournamentObjective> {
        &self.objectives
    }

    #[must_use]
    pub fn marks(&self) -> &BTreeMap<MarkId, PaintMark> {
        &self.marks
    }

    #[must_use]
    pub fn service_marks(&self) -> &BTreeMap<ServiceMarkId, ServiceMark> {
        &self.service_marks
    }

    #[must_use]
    pub fn results(&self) -> &BTreeMap<ResultId, TournamentResult> {
        &self.results
    }

    fn register_competitor(
        &mut self,
        competitor: &TournamentCompetitor,
    ) -> Result<(), ServiceTournamentError> {
        if self.opened {
            return Err(ServiceTournamentError::RosterClosed);
        }
        if competitor.tournament != self.tournament.id {
            return Err(ServiceTournamentError::WrongTournament);
        }
        if competitor.service_identity.house() != competitor.house
            || competitor.public_name.trim().is_empty()
        {
            return Err(ServiceTournamentError::ServiceIdentityMismatch(
                competitor.id.clone(),
            ));
        }
        if self.competitors.contains_key(&competitor.id) {
            return Err(ServiceTournamentError::DuplicateCompetitor(
                competitor.id.clone(),
            ));
        }
        if self.house_representatives.contains_key(&competitor.house) {
            return Err(ServiceTournamentError::DuplicateHouseRepresentative(
                competitor.house,
            ));
        }
        self.competitors
            .insert(competitor.id.clone(), competitor.clone());
        self.house_representatives
            .insert(competitor.house, competitor.id.clone());
        Ok(())
    }

    fn open(&mut self) -> Result<(), ServiceTournamentError> {
        if self.opened {
            return Err(ServiceTournamentError::TournamentAlreadyOpen);
        }
        let canonical_houses = PairedServiceIdentity::ALL
            .into_iter()
            .map(PairedServiceIdentity::house)
            .collect::<BTreeSet<_>>();
        let registered_houses = self
            .house_representatives
            .keys()
            .copied()
            .collect::<BTreeSet<_>>();
        let registered_identities = self
            .competitors
            .values()
            .map(|competitor| competitor.service_identity)
            .collect::<BTreeSet<_>>();
        if self.competitors.len() != 4
            || registered_houses != canonical_houses
            || registered_identities
                != PairedServiceIdentity::ALL
                    .into_iter()
                    .collect::<BTreeSet<_>>()
        {
            return Err(ServiceTournamentError::IncompleteFourHouseRoster);
        }
        self.opened = true;
        Ok(())
    }

    fn register_scenario(
        &mut self,
        scenario: &TournamentScenario,
    ) -> Result<(), ServiceTournamentError> {
        self.require_open()?;
        if scenario.tournament != self.tournament.id {
            return Err(ServiceTournamentError::WrongTournament);
        }
        if scenario.war.as_ref().is_some_and(|war| war != &self.war.id) {
            return Err(ServiceTournamentError::WrongWar);
        }
        if !scenario.nonlethal {
            return Err(ServiceTournamentError::NonlethalSimulationRequired);
        }
        if scenario.objectives.is_empty() {
            return Err(ServiceTournamentError::MissingObjectives(
                scenario.id.clone(),
            ));
        }
        if self.scenarios.contains_key(&scenario.id) {
            return Err(ServiceTournamentError::DuplicateScenario(
                scenario.id.clone(),
            ));
        }
        let mut ids = BTreeSet::new();
        for objective in &scenario.objectives {
            if objective.description.trim().is_empty()
                || objective.available_points == 0
                || !ids.insert(objective.id.clone())
                || self.objectives.contains_key(&objective.id)
            {
                return Err(ServiceTournamentError::DuplicateObjective(
                    objective.id.clone(),
                ));
            }
        }
        for objective in &scenario.objectives {
            self.objectives
                .insert(objective.id.clone(), objective.clone());
        }
        self.scenarios.insert(scenario.id.clone(), scenario.clone());
        Ok(())
    }

    fn record_mark(
        &mut self,
        event_id: &TournamentEventId,
        mark: &PaintMark,
    ) -> Result<(), ServiceTournamentError> {
        self.require_open()?;
        if mark.war != self.war.id {
            return Err(ServiceTournamentError::WrongWar);
        }
        if let Some(scenario) = &mark.scenario
            && !self.scenarios.contains_key(scenario)
        {
            return Err(ServiceTournamentError::UnknownScenario(scenario.clone()));
        }
        if self.marks.contains_key(&mark.id) {
            return Err(ServiceTournamentError::DuplicateMark(mark.id.clone()));
        }
        let competitor = self
            .competitors
            .get(&mark.provenance.competitor)
            .ok_or_else(|| {
                ServiceTournamentError::UnknownCompetitor(mark.provenance.competitor.clone())
            })?;
        if competitor.house != mark.house {
            return Err(ServiceTournamentError::MarkHouseMismatch(mark.id.clone()));
        }
        if mark.color_family != HouseColorFamily::for_house(mark.house) {
            return Err(ServiceTournamentError::ColorFamilyMismatch(mark.id.clone()));
        }
        for color in &mark.palette_sources {
            if !mark.color_family.accepts_palette_color(color) {
                return Err(ServiceTournamentError::InvalidPaletteSource(
                    mark.id.clone(),
                    color.clone(),
                ));
            }
        }
        if mark.provenance.originating_event != *event_id
            || mark.provenance.evidence.is_empty()
            || mark.palette_sources.is_empty()
            || mark.hue_description.trim().is_empty()
        {
            return Err(ServiceTournamentError::MissingPaintProvenance(
                mark.id.clone(),
            ));
        }
        if self.marks.values().any(|existing| {
            existing.location == mark.location && existing.layer_sequence == mark.layer_sequence
        }) {
            return Err(ServiceTournamentError::LayerSequenceConflict(
                mark.location.clone(),
                mark.layer_sequence,
            ));
        }
        self.marks.insert(mark.id.clone(), mark.clone());
        Ok(())
    }

    fn preserve_service_mark(
        &mut self,
        event_id: &TournamentEventId,
        service_mark: &ServiceMark,
    ) -> Result<(), ServiceTournamentError> {
        self.require_open()?;
        if service_mark.war != self.war.id {
            return Err(ServiceTournamentError::WrongWar);
        }
        if self.service_marks.contains_key(&service_mark.id) {
            return Err(ServiceTournamentError::DuplicateServiceMark(
                service_mark.id.clone(),
            ));
        }
        if service_mark.provenance.source_marks.is_empty() {
            return Err(ServiceTournamentError::InvalidServiceMarkProvenance(
                service_mark.id.clone(),
            ));
        }

        let mut houses = BTreeSet::new();
        let mut participants = BTreeSet::new();
        let mut source_events = BTreeSet::new();
        let mut ordered_layers = Vec::new();
        for mark_id in &service_mark.provenance.source_marks {
            let mark = self
                .marks
                .get(mark_id)
                .ok_or_else(|| ServiceTournamentError::UnknownMark(mark_id.clone()))?;
            if mark.location != service_mark.location {
                return Err(ServiceTournamentError::InvalidServiceMarkProvenance(
                    service_mark.id.clone(),
                ));
            }
            if mark.scenario.as_ref() != Some(&service_mark.scenario) {
                return Err(ServiceTournamentError::InvalidServiceMarkProvenance(
                    service_mark.id.clone(),
                ));
            }
            houses.insert(mark.house);
            participants.insert(mark.provenance.competitor.clone());
            source_events.insert(mark.provenance.originating_event.clone());
            ordered_layers.push(mark);
        }
        ordered_layers.sort_by_key(|mark| (mark.layer_sequence, mark.id.as_str().to_owned()));
        let ordered_layer_ids = ordered_layers
            .into_iter()
            .map(|mark| mark.id.clone())
            .collect::<Vec<_>>();
        if service_mark.provenance.preservation_event != *event_id
            || service_mark.provenance.source_action_events != source_events
            || service_mark.provenance.evidence.is_empty()
            || service_mark.houses != houses
            || !participants.is_subset(&service_mark.participants)
            || service_mark.ordered_paint_layers != ordered_layer_ids
            || service_mark.operation_name.trim().is_empty()
            || service_mark.constitutional_significance.trim().is_empty()
            || service_mark.account.trim().is_empty()
        {
            return Err(ServiceTournamentError::InvalidServiceMarkProvenance(
                service_mark.id.clone(),
            ));
        }
        self.service_marks
            .insert(service_mark.id.clone(), service_mark.clone());
        Ok(())
    }

    fn record_result(&mut self, result: &TournamentResult) -> Result<(), ServiceTournamentError> {
        self.require_open()?;
        if result.tournament != self.tournament.id {
            return Err(ServiceTournamentError::WrongTournament);
        }
        if self.results.contains_key(&result.id) {
            return Err(ServiceTournamentError::DuplicateResult(result.id.clone()));
        }
        if result.transfers_permanent_sovereignty || result.award.transfers_permanent_sovereignty()
        {
            return Err(ServiceTournamentError::PermanentSovereigntyForbidden);
        }
        if !self.house_representatives.contains_key(&result.champion) {
            return Err(ServiceTournamentError::InvalidChampion(result.champion));
        }
        let houses = self
            .house_representatives
            .keys()
            .copied()
            .collect::<BTreeSet<_>>();
        if result.scorecards.keys().copied().collect::<BTreeSet<_>>() != houses {
            return Err(ServiceTournamentError::IncompleteScorecard(
                result.id.clone(),
            ));
        }
        let categories = ScoringCategory::ALL.into_iter().collect::<BTreeSet<_>>();
        if result.scorecards.values().any(|scorecard| {
            scorecard.scores.keys().copied().collect::<BTreeSet<_>>() != categories
        }) {
            return Err(ServiceTournamentError::IncompleteScorecard(
                result.id.clone(),
            ));
        }
        self.results.insert(result.id.clone(), result.clone());
        Ok(())
    }

    fn require_open(&self) -> Result<(), ServiceTournamentError> {
        if self.opened {
            Ok(())
        } else {
            Err(ServiceTournamentError::TournamentNotOpen)
        }
    }
}

fn validate_definition(
    tournament: &ServiceTournamentDefinition,
    war: &WarDefinition,
) -> Result<(), ServiceTournamentError> {
    let representatives = tournament
        .representatives
        .into_iter()
        .collect::<BTreeSet<_>>();
    let houses = tournament
        .representatives
        .into_iter()
        .map(PairedServiceIdentity::house)
        .collect::<BTreeSet<_>>();
    let profiles = canonical_house_service_profiles();
    let references_are_external = profiles.iter().all(|profile| {
        profile.one_complete_cultural_government_identity
            && profile.external_reference_models_only
            && profile.preserves_house_government
            && !profile.creates_separate_service_teams
    });
    let required_constitutions = [
        "HOLLOW_GROVE_COMPROMISE_V1_DRAFT.md",
        "FLYNT_CONSTITUTION_V2.md",
        "STONEBEND_CONSTITUTION_V2.md",
        "SANDMANOR_CONSTITUTION_V2.md",
        "GLAUSHOUSE_CONSTITUTION_V2.md",
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();
    let visual_constitution = crate::constitutional::canonical_visual_color_constitution();
    let palette_matches = HouseColorFamily::ALL.into_iter().all(|family| {
        let Some(palette) = visual_constitution.house_palette(family.house()) else {
            return false;
        };
        palette
            .canonical_color_ids()
            .into_iter()
            .collect::<BTreeSet<_>>()
            == family
                .palette_color_ids()
                .iter()
                .copied()
                .collect::<BTreeSet<_>>()
    });
    if tournament.name != SERVICE_TOURNAMENT_NAME
        || tournament.function != CentralJunctionFunction::ServiceTournament
        || tournament.id.as_str() != tournament.function.stable_id()
        || tournament.location_stable_id != "district.central-junction"
        || tournament.central_war != war.id
        || representatives.len() != 4
        || houses.len() != 4
        || !references_are_external
        || tournament.constitutional_sources != required_constitutions
        || !palette_matches
        || !tournament.largest_public_function_at_central_junction
        || !tournament.temporary_shared_operational_arena
        || !tournament.scenarios_may_overlap
        || !tournament.nonlethal_simulation_only
        || tournament.transfers_permanent_sovereignty
        || war.tournament != tournament.id
        || war.official_name != WAR_OF_A_THOUSAND_HUES_NAME
        || !war.nonlethal
        || !war.paint_records_presence_and_action
        || war.color_owners.len() != 4
        || war
            .color_owners
            .iter()
            .any(|(house, color)| color.house() != *house)
    {
        return Err(ServiceTournamentError::InvalidCanonicalDefinition);
    }
    Ok(())
}
