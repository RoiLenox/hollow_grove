//! Central Junction and Four-Pole economic law.
//!
//! This module is a deterministic constitutional model, not a trading loop or
//! gameplay reducer. It does not move actors, execute combat, transform a
//! Being, alter party state, or enter the recursion kernel.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::hollow_grove_contract::House;

pub const CENTRAL_JUNCTION_SOURCE: &str = "CENTRAL_JUNCTION_FOUR_POLE_ECONOMY_V1.md";
pub const CENTRAL_JUNCTION_NAME: &str = "CENTRAL JUNCTION";
pub const CENTRAL_JUNCTION_SHORT_NAME: &str = "The Junction";
pub const STANDARD_CURRENCY_PUBLIC_NAME: Option<&str> = None;

macro_rules! junction_id {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, JunctionIdError> {
                let value = value.into();
                if value.is_empty()
                    || !value.bytes().all(|byte| {
                        byte.is_ascii_lowercase()
                            || byte.is_ascii_digit()
                            || byte == b'.'
                            || byte == b'-'
                    })
                {
                    return Err(JunctionIdError(value));
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
    };
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JunctionIdError(String);

impl fmt::Display for JunctionIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "invalid Central Junction stable identifier: {}",
            self.0
        )
    }
}

impl std::error::Error for JunctionIdError {}

junction_id!(MarketIndexId);
junction_id!(EnterpriseId);
junction_id!(ProjectId);
junction_id!(EventContractId);
junction_id!(SettlementEvidenceId);
junction_id!(MarketActorId);
junction_id!(MarketPositionId);
junction_id!(BoardDecisionId);
junction_id!(SettlementId);
junction_id!(PublicationId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum WorkObject {
    Form,
    Function,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum WorkLifecycle {
    Creation,
    Continuance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EconomicPole {
    Design,
    Engineering,
    Craft,
    Repair,
}

impl EconomicPole {
    pub const ALL: [Self; 4] = [Self::Design, Self::Engineering, Self::Craft, Self::Repair];

    #[must_use]
    pub const fn house(self) -> House {
        match self {
            Self::Design => House::Sandmanor,
            Self::Engineering => House::Flynt,
            Self::Craft => House::Stonebend,
            Self::Repair => House::Glaushouse,
        }
    }

    #[must_use]
    pub const fn work_object(self) -> WorkObject {
        match self {
            Self::Design | Self::Craft => WorkObject::Form,
            Self::Engineering | Self::Repair => WorkObject::Function,
        }
    }

    #[must_use]
    pub const fn lifecycle(self) -> WorkLifecycle {
        match self {
            Self::Design | Self::Engineering => WorkLifecycle::Creation,
            Self::Craft | Self::Repair => WorkLifecycle::Continuance,
        }
    }
}

#[must_use]
pub const fn classify_economic_pole(object: WorkObject, lifecycle: WorkLifecycle) -> EconomicPole {
    match (object, lifecycle) {
        (WorkObject::Form, WorkLifecycle::Creation) => EconomicPole::Design,
        (WorkObject::Function, WorkLifecycle::Creation) => EconomicPole::Engineering,
        (WorkObject::Form, WorkLifecycle::Continuance) => EconomicPole::Craft,
        (WorkObject::Function, WorkLifecycle::Continuance) => EconomicPole::Repair,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WorkDisposition {
    Act,
    Cultivate,
    Reroute,
    Release,
}

#[must_use]
pub const fn classify_work_disposition(important: bool, urgent: bool) -> WorkDisposition {
    match (important, urgent) {
        (true, true) => WorkDisposition::Act,
        (true, false) => WorkDisposition::Cultivate,
        (false, true) => WorkDisposition::Reroute,
        (false, false) => WorkDisposition::Release,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StandardCurrencyAmount {
    minor_units: u64,
}

impl StandardCurrencyAmount {
    #[must_use]
    pub const fn from_minor_units(minor_units: u64) -> Self {
        Self { minor_units }
    }

    #[must_use]
    pub const fn minor_units(self) -> u64 {
        self.minor_units
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValueInstrument {
    StandardCurrency,
    TokeToken,
    EnterpriseShare,
    EventContractPosition,
    Gremlincoin,
    SectorIndex,
}

impl ValueInstrument {
    #[must_use]
    pub const fn is_ordinary_currency(self) -> bool {
        matches!(self, Self::StandardCurrency)
    }

    #[must_use]
    pub const fn is_spendable_money(self) -> bool {
        matches!(self, Self::StandardCurrency)
    }

    #[must_use]
    pub const fn is_earned_evidence(self) -> bool {
        matches!(self, Self::TokeToken | Self::Gremlincoin)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CentralJunctionInstitution {
    SouthRidgeExchange,
    JunctionBoard,
    ClearingHouse,
    JunctionWire,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MarketAuthorityFunction {
    EnterpriseListings,
    ShareTrading,
    ProjectFinancing,
    EventContractTrading,
    OfficialIndexCalculation,
    TransactionProcessing,
    ListingStandards,
    IndexMembership,
    IndexMethodology,
    DisclosureRequirements,
    ProfessionalEvidenceAudits,
    ConflictOfInterestRules,
    MarketInvestigations,
    DisputedClassifications,
    DisputedHouseAssessments,
    PublicMarketRules,
    CompletedTradeSettlement,
    ShareTransfers,
    ProjectContractSettlement,
    EventContractSettlement,
    Payouts,
    VoidedContracts,
    AuthorizedReversals,
    FinalBalances,
    IndexPublication,
    PricePublication,
    ListingPublication,
    DisclosurePublication,
    ProjectUpdatePublication,
    EmploymentDemandPublication,
    MarketWarningPublication,
    RecognizedFailurePublication,
    SettledOutcomePublication,
    PublicNotices,
}

impl CentralJunctionInstitution {
    pub const ALL: [Self; 4] = [
        Self::SouthRidgeExchange,
        Self::JunctionBoard,
        Self::ClearingHouse,
        Self::JunctionWire,
    ];

    #[must_use]
    pub const fn stable_id(self) -> &'static str {
        match self {
            Self::SouthRidgeExchange => "institution.central-junction.south-ridge-exchange",
            Self::JunctionBoard => "institution.central-junction.junction-board",
            Self::ClearingHouse => "institution.central-junction.clearing-house",
            Self::JunctionWire => "institution.central-junction.junction-wire",
        }
    }

    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::SouthRidgeExchange => "South Ridge Exchange",
            Self::JunctionBoard => "Junction Board",
            Self::ClearingHouse => "Clearing House",
            Self::JunctionWire => "Junction Wire",
        }
    }

    #[must_use]
    pub const fn governs(self, function: MarketAuthorityFunction) -> bool {
        use MarketAuthorityFunction::{
            AuthorizedReversals, CompletedTradeSettlement, ConflictOfInterestRules,
            DisclosurePublication, DisclosureRequirements, DisputedClassifications,
            DisputedHouseAssessments, EmploymentDemandPublication, EnterpriseListings,
            EventContractSettlement, EventContractTrading, FinalBalances, IndexMembership,
            IndexMethodology, IndexPublication, ListingPublication, ListingStandards,
            MarketInvestigations, MarketWarningPublication, OfficialIndexCalculation, Payouts,
            PricePublication, ProfessionalEvidenceAudits, ProjectContractSettlement,
            ProjectFinancing, ProjectUpdatePublication, PublicMarketRules, PublicNotices,
            RecognizedFailurePublication, SettledOutcomePublication, ShareTrading, ShareTransfers,
            TransactionProcessing, VoidedContracts,
        };
        match self {
            Self::SouthRidgeExchange => matches!(
                function,
                EnterpriseListings
                    | ShareTrading
                    | ProjectFinancing
                    | EventContractTrading
                    | OfficialIndexCalculation
                    | TransactionProcessing
            ),
            Self::JunctionBoard => matches!(
                function,
                ListingStandards
                    | IndexMembership
                    | IndexMethodology
                    | DisclosureRequirements
                    | ProfessionalEvidenceAudits
                    | ConflictOfInterestRules
                    | MarketInvestigations
                    | DisputedClassifications
                    | DisputedHouseAssessments
                    | PublicMarketRules
            ),
            Self::ClearingHouse => matches!(
                function,
                CompletedTradeSettlement
                    | ShareTransfers
                    | ProjectContractSettlement
                    | EventContractSettlement
                    | Payouts
                    | VoidedContracts
                    | AuthorizedReversals
                    | FinalBalances
            ),
            Self::JunctionWire => matches!(
                function,
                IndexPublication
                    | PricePublication
                    | ListingPublication
                    | DisclosurePublication
                    | ProjectUpdatePublication
                    | EmploymentDemandPublication
                    | MarketWarningPublication
                    | RecognizedFailurePublication
                    | SettledOutcomePublication
                    | PublicNotices
            ),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum HouseSectorHall {
    StonebendCraftHall,
    SandmanorDesignHall,
    FlyntEngineeringHall,
    GlaushouseRepairHall,
}

impl HouseSectorHall {
    pub const ALL: [Self; 4] = [
        Self::StonebendCraftHall,
        Self::SandmanorDesignHall,
        Self::FlyntEngineeringHall,
        Self::GlaushouseRepairHall,
    ];

    #[must_use]
    pub const fn stable_id(self) -> &'static str {
        match self {
            Self::StonebendCraftHall => "institution.sector-hall.stonebend-craft",
            Self::SandmanorDesignHall => "institution.sector-hall.sandmanor-design",
            Self::FlyntEngineeringHall => "institution.sector-hall.flynt-engineering",
            Self::GlaushouseRepairHall => "institution.sector-hall.glaushouse-repair",
        }
    }

    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::StonebendCraftHall => "Stonebend Craft Hall",
            Self::SandmanorDesignHall => "Sandmanor Design Hall",
            Self::FlyntEngineeringHall => "Flynt Engineering Hall",
            Self::GlaushouseRepairHall => "Glaüshouse Repair Hall",
        }
    }

    #[must_use]
    pub const fn pole(self) -> EconomicPole {
        match self {
            Self::StonebendCraftHall => EconomicPole::Craft,
            Self::SandmanorDesignHall => EconomicPole::Design,
            Self::FlyntEngineeringHall => EconomicPole::Engineering,
            Self::GlaushouseRepairHall => EconomicPole::Repair,
        }
    }

    #[must_use]
    pub const fn house(self) -> House {
        self.pole().house()
    }

    #[must_use]
    pub const fn sets_market_price(self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum JunctionApproach {
    CraftCorridor,
    RepairCorridor,
    DesignCorridor,
    FlyntEngineeringRing,
}

impl JunctionApproach {
    pub const ALL: [Self; 4] = [
        Self::CraftCorridor,
        Self::RepairCorridor,
        Self::DesignCorridor,
        Self::FlyntEngineeringRing,
    ];

    #[must_use]
    pub const fn stable_id(self) -> &'static str {
        match self {
            Self::CraftCorridor => "geography.central-junction.craft-corridor",
            Self::RepairCorridor => "geography.central-junction.repair-corridor",
            Self::DesignCorridor => "geography.central-junction.design-corridor",
            Self::FlyntEngineeringRing => "geography.central-junction.flynt-engineering-ring",
        }
    }

    #[must_use]
    pub const fn toward_house(self) -> House {
        match self {
            Self::CraftCorridor => House::Stonebend,
            Self::RepairCorridor => House::Glaushouse,
            Self::DesignCorridor => House::Sandmanor,
            Self::FlyntEngineeringRing => House::Flynt,
        }
    }

    #[must_use]
    pub const fn is_interior_spoke(self) -> bool {
        !matches!(self, Self::FlyntEngineeringRing)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CentralJunctionFunction {
    ServiceTournament,
}

impl CentralJunctionFunction {
    pub const ALL: [Self; 1] = [Self::ServiceTournament];

    #[must_use]
    pub const fn stable_id(self) -> &'static str {
        match self {
            Self::ServiceTournament => "function.central-junction.service-tournament",
        }
    }

    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::ServiceTournament => "The Service Tournament",
        }
    }

    #[must_use]
    pub const fn is_largest_public_function(self) -> bool {
        matches!(self, Self::ServiceTournament)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CentralJunctionDistrict {
    pub stable_id: &'static str,
    pub formal_name: &'static str,
    pub short_name: &'static str,
    pub institutions: BTreeSet<CentralJunctionInstitution>,
    pub approaches: BTreeSet<JunctionApproach>,
    pub sector_halls: BTreeSet<HouseSectorHall>,
    pub public_functions: BTreeSet<CentralJunctionFunction>,
    pub district_not_single_building: bool,
}

#[must_use]
pub fn canonical_central_junction() -> CentralJunctionDistrict {
    CentralJunctionDistrict {
        stable_id: "district.central-junction",
        formal_name: CENTRAL_JUNCTION_NAME,
        short_name: CENTRAL_JUNCTION_SHORT_NAME,
        institutions: CentralJunctionInstitution::ALL.into_iter().collect(),
        approaches: JunctionApproach::ALL.into_iter().collect(),
        sector_halls: HouseSectorHall::ALL.into_iter().collect(),
        public_functions: CentralJunctionFunction::ALL.into_iter().collect(),
        district_not_single_building: true,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SummitConcept {
    CurrentHaze,
    EqualGaze,
    AuraBeam,
}

impl SummitConcept {
    #[must_use]
    pub const fn canonical_statement(self) -> &'static str {
        match self {
            Self::CurrentHaze => "Current Haze is unresolved possibility.",
            Self::EqualGaze => "Equal Gaze is reconciled perspective.",
            Self::AuraBeam => "Aura Beam reveals or transmits the visible shared future.",
        }
    }

    #[must_use]
    pub const fn is_market_institution(self) -> bool {
        false
    }

    #[must_use]
    pub const fn is_financial_ticker(self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MarketLifecycleState {
    Proposed,
    Open,
    UnderReview,
    Recognized,
    Disputed,
    Settled,
    Voided,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MarketProcessStage {
    HouseSectorHall,
    JunctionBoard,
    ClearingHouse,
    JunctionWire,
}

impl MarketProcessStage {
    pub const ALL: [Self; 4] = [
        Self::HouseSectorHall,
        Self::JunctionBoard,
        Self::ClearingHouse,
        Self::JunctionWire,
    ];
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarketIndexDefinition {
    pub id: MarketIndexId,
    pub name: &'static str,
    pub pole: EconomicPole,
    pub owner: Option<House>,
    pub methodology_authority: CentralJunctionInstitution,
    pub calculation_authority: CentralJunctionInstitution,
    pub publication_authority: CentralJunctionInstitution,
    pub currency: bool,
}

#[must_use]
pub fn canonical_market_indexes() -> Vec<MarketIndexDefinition> {
    [
        (
            "index.central-junction.sandmanor-design",
            "Sandmanor Design Index",
            EconomicPole::Design,
        ),
        (
            "index.central-junction.flynt-engineering",
            "Flynt Engineering Index",
            EconomicPole::Engineering,
        ),
        (
            "index.central-junction.stonebend-craft",
            "Stonebend Craft Index",
            EconomicPole::Craft,
        ),
        (
            "index.central-junction.glaushouse-repair",
            "Glaüshouse Repair Index",
            EconomicPole::Repair,
        ),
    ]
    .into_iter()
    .map(|(id, name, pole)| MarketIndexDefinition {
        id: MarketIndexId::new(id).expect("canonical index ID"),
        name,
        pole,
        owner: None,
        methodology_authority: CentralJunctionInstitution::JunctionBoard,
        calculation_authority: CentralJunctionInstitution::SouthRidgeExchange,
        publication_authority: CentralJunctionInstitution::JunctionWire,
        currency: false,
    })
    .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexMethodology {
    pub approved_by: CentralJunctionInstitution,
    pub base_millipoints: i64,
    pub productive_unit_weight: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifiedProductiveEvidence {
    pub id: SettlementEvidenceId,
    pub hall: HouseSectorHall,
    pub pole: EconomicPole,
    pub net_productive_units: i64,
    pub board_recognized: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OfficialIndexValue {
    pub index: MarketIndexId,
    pub value_millipoints: i64,
    pub calculated_by: CentralJunctionInstitution,
    pub published_by: CentralJunctionInstitution,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PublicIndexBoardField {
    OfficialSectorIndexValue,
    PriceMovement,
    VerifiedProductiveEvidence,
    EmploymentDemand,
    MajorActiveProjects,
    Warnings,
    PublicDisclosures,
}

impl PublicIndexBoardField {
    pub const ALL: [Self; 7] = [
        Self::OfficialSectorIndexValue,
        Self::PriceMovement,
        Self::VerifiedProductiveEvidence,
        Self::EmploymentDemand,
        Self::MajorActiveProjects,
        Self::Warnings,
        Self::PublicDisclosures,
    ];
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicIndexBoard {
    pub stable_id: &'static str,
    pub hall: HouseSectorHall,
    pub index: MarketIndexId,
    pub connected_to: CentralJunctionInstitution,
    pub fields: BTreeSet<PublicIndexBoardField>,
}

#[must_use]
pub fn canonical_public_index_boards() -> Vec<PublicIndexBoard> {
    canonical_market_indexes()
        .into_iter()
        .map(|index| {
            let (stable_id, hall) = match index.pole {
                EconomicPole::Design => (
                    "board.sector-hall.sandmanor-design-index",
                    HouseSectorHall::SandmanorDesignHall,
                ),
                EconomicPole::Engineering => (
                    "board.sector-hall.flynt-engineering-index",
                    HouseSectorHall::FlyntEngineeringHall,
                ),
                EconomicPole::Craft => (
                    "board.sector-hall.stonebend-craft-index",
                    HouseSectorHall::StonebendCraftHall,
                ),
                EconomicPole::Repair => (
                    "board.sector-hall.glaushouse-repair-index",
                    HouseSectorHall::GlaushouseRepairHall,
                ),
            };
            PublicIndexBoard {
                stable_id,
                hall,
                index: index.id,
                connected_to: CentralJunctionInstitution::JunctionWire,
                fields: PublicIndexBoardField::ALL.into_iter().collect(),
            }
        })
        .collect()
}

pub fn calculate_official_index(
    index: &MarketIndexDefinition,
    methodology: &IndexMethodology,
    evidence: &[VerifiedProductiveEvidence],
    expectation_delta_millipoints: i64,
) -> Result<OfficialIndexValue, CentralJunctionError> {
    validate_market_index(index)?;
    if methodology.approved_by != CentralJunctionInstitution::JunctionBoard {
        return Err(CentralJunctionError::UnlawfulIndexMethodology);
    }
    let mut seen = BTreeSet::new();
    let mut productive_units = 0_i64;
    for record in evidence {
        if !seen.insert(&record.id)
            || !record.board_recognized
            || record.pole != index.pole
            || record.hall.pole() != index.pole
        {
            return Err(CentralJunctionError::InvalidIndexEvidence);
        }
        productive_units = productive_units
            .checked_add(record.net_productive_units)
            .ok_or(CentralJunctionError::ArithmeticOverflow)?;
    }
    let evidence_value = productive_units
        .checked_mul(methodology.productive_unit_weight)
        .ok_or(CentralJunctionError::ArithmeticOverflow)?;
    let value_millipoints = methodology
        .base_millipoints
        .checked_add(evidence_value)
        .and_then(|value| value.checked_add(expectation_delta_millipoints))
        .ok_or(CentralJunctionError::ArithmeticOverflow)?;
    Ok(OfficialIndexValue {
        index: index.id.clone(),
        value_millipoints,
        calculated_by: CentralJunctionInstitution::SouthRidgeExchange,
        published_by: CentralJunctionInstitution::JunctionWire,
    })
}

fn validate_market_index(index: &MarketIndexDefinition) -> Result<(), CentralJunctionError> {
    if index.owner.is_some()
        || index.currency
        || index.methodology_authority != CentralJunctionInstitution::JunctionBoard
        || index.calculation_authority != CentralJunctionInstitution::SouthRidgeExchange
        || index.publication_authority != CentralJunctionInstitution::JunctionWire
    {
        return Err(CentralJunctionError::InvalidOfficialIndex(index.id.clone()));
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SectorExposure {
    pub pole: EconomicPole,
    pub basis_points: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EconomicClassification {
    pub primary: SectorExposure,
    pub secondary: Vec<SectorExposure>,
    pub evidence: Vec<String>,
}

impl EconomicClassification {
    pub fn validate(&self) -> Result<(), CentralJunctionError> {
        if self.primary.basis_points == 0
            || self.evidence.is_empty()
            || self.evidence.iter().any(|entry| entry.trim().is_empty())
        {
            return Err(CentralJunctionError::InvalidEconomicClassification);
        }
        let mut poles = BTreeSet::from([self.primary.pole]);
        let mut total = u32::from(self.primary.basis_points);
        for exposure in &self.secondary {
            if exposure.basis_points == 0
                || exposure.pole == self.primary.pole
                || exposure.basis_points >= self.primary.basis_points
                || !poles.insert(exposure.pole)
            {
                return Err(CentralJunctionError::InvalidEconomicClassification);
            }
            total += u32::from(exposure.basis_points);
        }
        if total != 10_000 {
            return Err(CentralJunctionError::InvalidEconomicClassification);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListedEnterprise {
    pub id: EnterpriseId,
    pub name: String,
    pub classification: EconomicClassification,
    pub shares_outstanding: u64,
    pub state: MarketLifecycleState,
}

impl ListedEnterprise {
    pub fn validate(&self) -> Result<(), CentralJunctionError> {
        if self.name.trim().is_empty() || self.shares_outstanding == 0 {
            return Err(CentralJunctionError::InvalidListedEnterprise(
                self.id.clone(),
            ));
        }
        self.classification.validate()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListedProject {
    pub id: ProjectId,
    pub name: String,
    pub classification: EconomicClassification,
    pub financing_target: StandardCurrencyAmount,
    pub state: MarketLifecycleState,
}

impl ListedProject {
    pub fn validate(&self) -> Result<(), CentralJunctionError> {
        if self.name.trim().is_empty() || self.financing_target.minor_units() == 0 {
            return Err(CentralJunctionError::InvalidListedProject(self.id.clone()));
        }
        self.classification.validate()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum WorkshopFaction {
    FactionA,
    FactionB,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum WorkshopCondition {
    ControlsWorkshop,
    ProductionOperational,
    StonebendRecognizesClaim,
    NoActiveLawfulChallenge,
}

impl WorkshopCondition {
    pub const ALL: [Self; 4] = [
        Self::ControlsWorkshop,
        Self::ProductionOperational,
        Self::StonebendRecognizesClaim,
        Self::NoActiveLawfulChallenge,
    ];

    #[must_use]
    pub const fn attesting_hall(self) -> HouseSectorHall {
        match self {
            Self::ProductionOperational => HouseSectorHall::FlyntEngineeringHall,
            Self::ControlsWorkshop
            | Self::StonebendRecognizesClaim
            | Self::NoActiveLawfulChallenge => HouseSectorHall::StonebendCraftHall,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EventOutcome {
    FactionA,
    FactionB,
    Draw,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventOutcomeDefinition {
    pub outcome: EventOutcome,
    pub conditions: BTreeSet<WorkshopCondition>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventContract {
    pub id: EventContractId,
    pub name: String,
    pub definitions_recorded_at: u64,
    pub opens_at: u64,
    pub closes_at: u64,
    pub definitions: Vec<EventOutcomeDefinition>,
    pub state: MarketLifecycleState,
}

impl EventContract {
    pub fn validate(&self) -> Result<(), CentralJunctionError> {
        if self.name.trim().is_empty()
            || self.definitions_recorded_at >= self.opens_at
            || self.opens_at >= self.closes_at
            || self.definitions.len() != 3
        {
            return Err(CentralJunctionError::InvalidEventContract(self.id.clone()));
        }
        let definitions = self
            .definitions
            .iter()
            .map(|definition| definition.outcome)
            .collect::<BTreeSet<_>>();
        if definitions
            != BTreeSet::from([
                EventOutcome::FactionA,
                EventOutcome::FactionB,
                EventOutcome::Draw,
            ])
            || self
                .definitions
                .iter()
                .any(|definition| match definition.outcome {
                    EventOutcome::FactionA | EventOutcome::FactionB => {
                        definition.conditions != WorkshopCondition::ALL.into_iter().collect()
                    }
                    EventOutcome::Draw => !definition.conditions.is_empty(),
                })
        {
            return Err(CentralJunctionError::InvalidEventContract(self.id.clone()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HouseAttestation {
    pub id: SettlementEvidenceId,
    pub contract: EventContractId,
    pub faction: WorkshopFaction,
    pub condition: WorkshopCondition,
    pub hall: HouseSectorHall,
    pub satisfied: bool,
    pub evidence: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MarketDuty {
    JunctionBoardReviewer,
    ClearingHouseOfficial,
    ConflictCommander,
    Trader,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConflictDisclosure {
    pub actor: MarketActorId,
    pub contract: EventContractId,
    pub duty: MarketDuty,
    pub material_interest: bool,
    pub disclosed: bool,
    pub recused: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarketPosition {
    pub id: MarketPositionId,
    pub contract: EventContractId,
    pub holder: MarketActorId,
    pub outcome: EventOutcome,
    pub stake: StandardCurrencyAmount,
    pub quoted_price_basis_points: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JunctionBoardDecision {
    pub id: BoardDecisionId,
    pub contract: EventContractId,
    pub outcome: Option<EventOutcome>,
    pub state: MarketLifecycleState,
    pub reviewed_by: MarketActorId,
    pub evidence: Vec<SettlementEvidenceId>,
    pub may_be_described_as_equal_gaze: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PositionSettlement {
    pub position: MarketPositionId,
    pub final_amount: StandardCurrencyAmount,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClearingSettlement {
    pub id: SettlementId,
    pub contract: EventContractId,
    pub decision: BoardDecisionId,
    pub outcome: EventOutcome,
    pub state: MarketLifecycleState,
    pub settled_by: MarketActorId,
    pub positions: Vec<PositionSettlement>,
    pub market_price_determined_outcome: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JunctionWirePublication {
    pub id: PublicationId,
    pub contract: EventContractId,
    pub settlement: SettlementId,
    pub published_by: CentralJunctionInstitution,
    pub recognized_outcome: EventOutcome,
    pub public_notice: String,
}

pub fn audit_event_contract(
    contract: &EventContract,
    attestations: &[HouseAttestation],
    reviewer: MarketActorId,
    positions: &[MarketPosition],
    disclosures: &[ConflictDisclosure],
    commanders: &[MarketActorId],
) -> Result<JunctionBoardDecision, CentralJunctionError> {
    contract.validate()?;
    validate_conflicts(
        contract,
        &reviewer,
        MarketDuty::JunctionBoardReviewer,
        positions,
        disclosures,
    )?;
    for commander in commanders {
        validate_conflicts(
            contract,
            commander,
            MarketDuty::ConflictCommander,
            positions,
            disclosures,
        )?;
    }
    let mut evidence_keys = BTreeSet::new();
    let mut satisfaction = BTreeMap::new();
    for attestation in attestations {
        if attestation.contract != contract.id
            || attestation.evidence.trim().is_empty()
            || attestation.hall != attestation.condition.attesting_hall()
            || !evidence_keys.insert((attestation.faction, attestation.condition))
        {
            return Err(CentralJunctionError::InvalidSettlementEvidence);
        }
        satisfaction.insert(
            (attestation.faction, attestation.condition),
            attestation.satisfied,
        );
    }
    if evidence_keys.len() != WorkshopFaction::ALL_COUNT * WorkshopCondition::ALL.len() {
        return Err(CentralJunctionError::IncompleteSettlementEvidence);
    }

    let complete = |faction| {
        WorkshopCondition::ALL.into_iter().all(|condition| {
            satisfaction
                .get(&(faction, condition))
                .copied()
                .unwrap_or(false)
        })
    };
    let faction_a = complete(WorkshopFaction::FactionA);
    let faction_b = complete(WorkshopFaction::FactionB);
    let (outcome, state) = match (faction_a, faction_b) {
        (true, false) => (
            Some(EventOutcome::FactionA),
            MarketLifecycleState::Recognized,
        ),
        (false, true) => (
            Some(EventOutcome::FactionB),
            MarketLifecycleState::Recognized,
        ),
        (false, false) => (Some(EventOutcome::Draw), MarketLifecycleState::Recognized),
        (true, true) => (None, MarketLifecycleState::Disputed),
    };
    Ok(JunctionBoardDecision {
        id: BoardDecisionId::new("decision.central-junction.blackroot-workshop")
            .expect("canonical decision ID"),
        contract: contract.id.clone(),
        outcome,
        state,
        reviewed_by: reviewer,
        evidence: attestations
            .iter()
            .map(|attestation| attestation.id.clone())
            .collect(),
        may_be_described_as_equal_gaze: state == MarketLifecycleState::Recognized,
    })
}

impl WorkshopFaction {
    const ALL_COUNT: usize = 2;
}

pub fn settle_event_contract(
    contract: &EventContract,
    decision: &JunctionBoardDecision,
    positions: &[MarketPosition],
    clearing_official: MarketActorId,
    disclosures: &[ConflictDisclosure],
) -> Result<ClearingSettlement, CentralJunctionError> {
    if decision.contract != contract.id
        || decision.state != MarketLifecycleState::Recognized
        || decision.outcome.is_none()
    {
        return Err(CentralJunctionError::UnrecognizedEventOutcome);
    }
    validate_conflicts(
        contract,
        &clearing_official,
        MarketDuty::ClearingHouseOfficial,
        positions,
        disclosures,
    )?;
    let outcome = decision.outcome.expect("recognized decision checked above");
    let mut position_ids = BTreeSet::new();
    let mut settled_positions = Vec::with_capacity(positions.len());
    for position in positions {
        if position.contract != contract.id
            || position.stake.minor_units() == 0
            || position.quoted_price_basis_points == 0
            || position.quoted_price_basis_points > 10_000
            || !position_ids.insert(&position.id)
        {
            return Err(CentralJunctionError::InvalidMarketPosition);
        }
        let final_units = if position.outcome == outcome {
            position
                .stake
                .minor_units()
                .checked_mul(2)
                .ok_or(CentralJunctionError::ArithmeticOverflow)?
        } else {
            0
        };
        settled_positions.push(PositionSettlement {
            position: position.id.clone(),
            final_amount: StandardCurrencyAmount::from_minor_units(final_units),
        });
    }
    Ok(ClearingSettlement {
        id: SettlementId::new("settlement.central-junction.blackroot-workshop")
            .expect("canonical settlement ID"),
        contract: contract.id.clone(),
        decision: decision.id.clone(),
        outcome,
        state: MarketLifecycleState::Settled,
        settled_by: clearing_official,
        positions: settled_positions,
        market_price_determined_outcome: false,
    })
}

fn validate_conflicts(
    contract: &EventContract,
    actor: &MarketActorId,
    duty: MarketDuty,
    positions: &[MarketPosition],
    disclosures: &[ConflictDisclosure],
) -> Result<(), CentralJunctionError> {
    let holds_position = positions
        .iter()
        .any(|position| position.contract == contract.id && position.holder == *actor);
    if !holds_position {
        return Ok(());
    }
    let disclosure = disclosures.iter().find(|disclosure| {
        disclosure.actor == *actor && disclosure.contract == contract.id && disclosure.duty == duty
    });
    if !disclosure
        .is_some_and(|record| record.material_interest && record.disclosed && record.recused)
    {
        return Err(CentralJunctionError::UndisclosedProhibitedInterest(
            actor.clone(),
        ));
    }
    Err(CentralJunctionError::RecusedOfficialCannotAct(
        actor.clone(),
    ))
}

#[must_use]
pub fn publish_settlement(
    settlement: &ClearingSettlement,
) -> Result<JunctionWirePublication, CentralJunctionError> {
    if settlement.state != MarketLifecycleState::Settled {
        return Err(CentralJunctionError::UnsettledPublication);
    }
    Ok(JunctionWirePublication {
        id: PublicationId::new("publication.central-junction.blackroot-workshop")
            .expect("canonical publication ID"),
        contract: settlement.contract.clone(),
        settlement: settlement.id.clone(),
        published_by: CentralJunctionInstitution::JunctionWire,
        recognized_outcome: settlement.outcome,
        public_notice: format!(
            "Blackroot Workshop Conflict settled as {:?}",
            settlement.outcome
        ),
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlackrootWorkshopProof {
    pub process: [MarketProcessStage; 4],
    pub contract: EventContract,
    pub attestations: Vec<HouseAttestation>,
    pub decision: JunctionBoardDecision,
    pub settlement: ClearingSettlement,
    pub publication: JunctionWirePublication,
}

pub fn blackroot_workshop_event_proof() -> Result<BlackrootWorkshopProof, CentralJunctionError> {
    let contract = canonical_blackroot_contract();
    let attestations = canonical_blackroot_attestations(&contract.id);
    let positions = canonical_blackroot_positions(&contract.id);
    let reviewer = MarketActorId::new("actor.junction-board.blackroot-reviewer")
        .expect("canonical reviewer ID");
    let clearing_official = MarketActorId::new("actor.clearing-house.blackroot-official")
        .expect("canonical clearing official ID");
    let commanders = [
        MarketActorId::new("actor.blackroot.faction-a-commander").expect("canonical commander ID"),
        MarketActorId::new("actor.blackroot.faction-b-commander").expect("canonical commander ID"),
    ];
    let decision = audit_event_contract(
        &contract,
        &attestations,
        reviewer,
        &positions,
        &[],
        &commanders,
    )?;
    let settlement =
        settle_event_contract(&contract, &decision, &positions, clearing_official, &[])?;
    let publication = publish_settlement(&settlement)?;
    Ok(BlackrootWorkshopProof {
        process: MarketProcessStage::ALL,
        contract,
        attestations,
        decision,
        settlement,
        publication,
    })
}

#[must_use]
pub fn canonical_blackroot_contract() -> EventContract {
    let winning_conditions = WorkshopCondition::ALL.into_iter().collect::<BTreeSet<_>>();
    EventContract {
        id: EventContractId::new("contract.event.blackroot-workshop")
            .expect("canonical contract ID"),
        name: "Blackroot Workshop Conflict".into(),
        definitions_recorded_at: 10,
        opens_at: 20,
        closes_at: 100,
        definitions: vec![
            EventOutcomeDefinition {
                outcome: EventOutcome::FactionA,
                conditions: winning_conditions.clone(),
            },
            EventOutcomeDefinition {
                outcome: EventOutcome::FactionB,
                conditions: winning_conditions,
            },
            EventOutcomeDefinition {
                outcome: EventOutcome::Draw,
                conditions: BTreeSet::new(),
            },
        ],
        state: MarketLifecycleState::UnderReview,
    }
}

fn canonical_blackroot_attestations(contract: &EventContractId) -> Vec<HouseAttestation> {
    let mut records = Vec::new();
    for faction in [WorkshopFaction::FactionA, WorkshopFaction::FactionB] {
        for condition in WorkshopCondition::ALL {
            let faction_slug = match faction {
                WorkshopFaction::FactionA => "faction-a",
                WorkshopFaction::FactionB => "faction-b",
            };
            let condition_slug = match condition {
                WorkshopCondition::ControlsWorkshop => "controls-workshop",
                WorkshopCondition::ProductionOperational => "production-operational",
                WorkshopCondition::StonebendRecognizesClaim => "stonebend-recognizes-claim",
                WorkshopCondition::NoActiveLawfulChallenge => "no-active-lawful-challenge",
            };
            records.push(HouseAttestation {
                id: SettlementEvidenceId::new(format!(
                    "evidence.blackroot.{faction_slug}.{condition_slug}"
                ))
                .expect("canonical evidence ID"),
                contract: contract.clone(),
                faction,
                condition,
                hall: condition.attesting_hall(),
                satisfied: faction == WorkshopFaction::FactionA,
                evidence: format!(
                    "{faction:?} {condition:?} recorded by {}",
                    condition.attesting_hall().display_name()
                ),
            });
        }
    }
    records
}

fn canonical_blackroot_positions(contract: &EventContractId) -> Vec<MarketPosition> {
    vec![
        MarketPosition {
            id: MarketPositionId::new("position.blackroot.trader-a")
                .expect("canonical position ID"),
            contract: contract.clone(),
            holder: MarketActorId::new("actor.blackroot.trader-a").expect("canonical actor ID"),
            outcome: EventOutcome::FactionA,
            stake: StandardCurrencyAmount::from_minor_units(500),
            quoted_price_basis_points: 6_000,
        },
        MarketPosition {
            id: MarketPositionId::new("position.blackroot.trader-b")
                .expect("canonical position ID"),
            contract: contract.clone(),
            holder: MarketActorId::new("actor.blackroot.trader-b").expect("canonical actor ID"),
            outcome: EventOutcome::FactionB,
            stake: StandardCurrencyAmount::from_minor_units(500),
            quoted_price_basis_points: 4_000,
        },
    ]
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CentralJunctionError {
    InvalidOfficialIndex(MarketIndexId),
    UnlawfulIndexMethodology,
    InvalidIndexEvidence,
    ArithmeticOverflow,
    InvalidEconomicClassification,
    InvalidListedEnterprise(EnterpriseId),
    InvalidListedProject(ProjectId),
    InvalidEventContract(EventContractId),
    InvalidSettlementEvidence,
    IncompleteSettlementEvidence,
    UndisclosedProhibitedInterest(MarketActorId),
    RecusedOfficialCannotAct(MarketActorId),
    UnrecognizedEventOutcome,
    InvalidMarketPosition,
    UnsettledPublication,
}

impl fmt::Display for CentralJunctionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Central Junction rejected state: {self:?}")
    }
}

impl std::error::Error for CentralJunctionError {}
