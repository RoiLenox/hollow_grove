//! Four Great Functions at Central Junction.
//!
//! The canonical calendar supplies neutral astronomical observations. This
//! layer attaches the four fixed civic Functions, House presidencies, motions,
//! dimensions, activities, and lifecycle records without changing House
//! sovereignty or entering the recursion kernel.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::constitutional::{
    AuthoritativeTimestamp, CalendarEvidenceId, CanonicalCalendarError, CanonicalYearId,
    CanonicalYearRecord, SeasonalAnchor,
};
use crate::hollow_grove_contract::House;

pub const CENTRAL_JUNCTION_SEASONAL_FUNCTIONS_SOURCE: &str =
    "CENTRAL_JUNCTION_SEASONAL_FUNCTIONS_V1.md";
pub const SEASONAL_FUNCTIONS_SCHEMA_VERSION: u16 = 1;
pub const UNIVERSAL_HUMAN_SEQUENCE: &str = "Heal → Belong → Participate → Serve";
pub const CANONICAL_CEREMONIAL_SEQUENCE: &str = "Return → Incarnate → Commune → Confirm";

macro_rules! seasonal_id {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, SeasonalFunctionError> {
                let value = value.into();
                if value.is_empty()
                    || !value.bytes().all(|byte| {
                        byte.is_ascii_lowercase()
                            || byte.is_ascii_digit()
                            || matches!(byte, b'.' | b'-')
                    })
                {
                    return Err(SeasonalFunctionError::InvalidIdentifier(value));
                }
                Ok(Self(value))
            }

            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl TryFrom<String> for $name {
            type Error = SeasonalFunctionError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(formatter)
            }
        }
    };
}

seasonal_id!(GreatFunctionId);
seasonal_id!(FunctionPhaseId);
seasonal_id!(SeasonalEventId);
seasonal_id!(SeasonalRecognitionId);
seasonal_id!(SeasonalVenueId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum UniversalHumanMotion {
    Heal,
    Belong,
    Participate,
    Serve,
}

impl UniversalHumanMotion {
    pub const ALL: [Self; 4] = [Self::Heal, Self::Belong, Self::Participate, Self::Serve];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SacredMotion {
    Return,
    Incarnate,
    Commune,
    Confirm,
}

impl SacredMotion {
    pub const ALL: [Self; 4] = [Self::Return, Self::Incarnate, Self::Commune, Self::Confirm];

    #[must_use]
    pub const fn human_motion(self) -> UniversalHumanMotion {
        match self {
            Self::Return => UniversalHumanMotion::Heal,
            Self::Incarnate => UniversalHumanMotion::Belong,
            Self::Commune => UniversalHumanMotion::Participate,
            Self::Confirm => UniversalHumanMotion::Serve,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum FunctionIntensity {
    Light,
    Moderate,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FunctionDimensions {
    pub celebration: FunctionIntensity,
    pub ritual: FunctionIntensity,
    pub competition: FunctionIntensity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum GreatFunctionKind {
    WayBack,
    Initiation,
    Gathering,
    FestivalOfMirrors,
}

impl GreatFunctionKind {
    pub const ALL: [Self; 4] = [
        Self::WayBack,
        Self::Initiation,
        Self::Gathering,
        Self::FestivalOfMirrors,
    ];

    #[must_use]
    pub const fn stable_id(self) -> &'static str {
        match self {
            Self::WayBack => "function.central-junction.seasonal.way-back",
            Self::Initiation => "function.central-junction.seasonal.initiation",
            Self::Gathering => "function.central-junction.seasonal.gathering",
            Self::FestivalOfMirrors => "function.central-junction.seasonal.festival-of-mirrors",
        }
    }

    #[must_use]
    pub const fn canonical_name(self) -> &'static str {
        match self {
            Self::WayBack => "The Way Back",
            Self::Initiation => "The Initiation",
            Self::Gathering => "The Gathering",
            Self::FestivalOfMirrors => "The Festival of Mirrors",
        }
    }

    #[must_use]
    pub const fn aliases(self) -> &'static [&'static str] {
        match self {
            Self::Gathering => &["Derrick"],
            _ => &[],
        }
    }

    #[must_use]
    pub const fn anchor(self) -> SeasonalAnchor {
        match self {
            Self::WayBack => SeasonalAnchor::WinterSolstice,
            Self::Initiation => SeasonalAnchor::SpringEquinox,
            Self::Gathering => SeasonalAnchor::SummerSolstice,
            Self::FestivalOfMirrors => SeasonalAnchor::AutumnEquinox,
        }
    }

    #[must_use]
    pub const fn presiding_house(self) -> House {
        match self {
            Self::WayBack => House::Glaushouse,
            Self::Initiation => House::Stonebend,
            Self::Gathering => House::Sandmanor,
            Self::FestivalOfMirrors => House::Flynt,
        }
    }

    #[must_use]
    pub const fn sacred_motion(self) -> SacredMotion {
        match self {
            Self::WayBack => SacredMotion::Return,
            Self::Initiation => SacredMotion::Incarnate,
            Self::Gathering => SacredMotion::Commune,
            Self::FestivalOfMirrors => SacredMotion::Confirm,
        }
    }

    #[must_use]
    pub const fn dimensions(self) -> FunctionDimensions {
        match self {
            Self::WayBack => FunctionDimensions {
                celebration: FunctionIntensity::VeryHigh,
                ritual: FunctionIntensity::VeryHigh,
                competition: FunctionIntensity::Light,
            },
            Self::Initiation => FunctionDimensions {
                celebration: FunctionIntensity::High,
                ritual: FunctionIntensity::VeryHigh,
                competition: FunctionIntensity::Moderate,
            },
            Self::Gathering => FunctionDimensions {
                celebration: FunctionIntensity::VeryHigh,
                ritual: FunctionIntensity::High,
                competition: FunctionIntensity::VeryHigh,
            },
            Self::FestivalOfMirrors => FunctionDimensions {
                celebration: FunctionIntensity::High,
                ritual: FunctionIntensity::VeryHigh,
                competition: FunctionIntensity::Moderate,
            },
        }
    }

    #[must_use]
    pub const fn central_question(self) -> &'static str {
        match self {
            Self::WayBack => "What can find its way back?",
            Self::Initiation => "What is ready to enter the world and receive a place?",
            Self::Gathering => "What can we become together?",
            Self::FestivalOfMirrors => "What has proven worthy of being carried forward?",
        }
    }

    #[must_use]
    pub const fn venue_ids(self) -> &'static [&'static str] {
        match self {
            Self::WayBack => &["venue.central-junction"],
            Self::Initiation => &["venue.central-junction", "venue.aura-field"],
            Self::Gathering => &["venue.central-junction", "venue.aura-beach"],
            Self::FestivalOfMirrors => &["venue.central-junction", "venue.aura-basin"],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum FunctionDimension {
    Celebration,
    Ritual,
    Competition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum FunctionActivity {
    PublicJoy,
    CommunalMeal,
    Music,
    Decoration,
    FamilyGathering,
    Market,
    Parade,
    Performance,
    CulturalExchange,
    Hospitality,
    MemorialObservance,
    PublicHealing,
    Reconciliation,
    BondReview,
    LanternAndGlassLight,
    PublicNaming,
    LawfulInitiation,
    FoundationDedication,
    Chartering,
    ArchitectureExhibition,
    StructuralTrial,
    GiftExchange,
    SharedCommunion,
    Diplomacy,
    HouseExhibition,
    PublicSynthesisPresentation,
    AthleticCivicEvent,
    ServiceTournament,
    ConstitutionalReview,
    PublicRecognition,
    Commissioning,
    ServiceMarkPreservation,
    EngineeringDemonstration,
    RecipeRatification,
    ArtifactCustodyRecognition,
    FailureReflection,
    ArchivePreparation,
    GentleRecoveryTrial,
    Ceremony,
}

impl FunctionActivity {
    #[must_use]
    pub const fn dimension(self) -> FunctionDimension {
        match self {
            Self::PublicJoy
            | Self::CommunalMeal
            | Self::Music
            | Self::Decoration
            | Self::FamilyGathering
            | Self::Market
            | Self::Parade
            | Self::Performance
            | Self::CulturalExchange
            | Self::Hospitality
            | Self::LanternAndGlassLight
            | Self::GiftExchange
            | Self::HouseExhibition => FunctionDimension::Celebration,
            Self::MemorialObservance
            | Self::PublicHealing
            | Self::Reconciliation
            | Self::BondReview
            | Self::PublicNaming
            | Self::LawfulInitiation
            | Self::FoundationDedication
            | Self::Chartering
            | Self::SharedCommunion
            | Self::ConstitutionalReview
            | Self::PublicRecognition
            | Self::Commissioning
            | Self::ServiceMarkPreservation
            | Self::ArtifactCustodyRecognition
            | Self::FailureReflection
            | Self::ArchivePreparation
            | Self::Ceremony => FunctionDimension::Ritual,
            Self::ArchitectureExhibition
            | Self::StructuralTrial
            | Self::Diplomacy
            | Self::PublicSynthesisPresentation
            | Self::AthleticCivicEvent
            | Self::ServiceTournament
            | Self::EngineeringDemonstration
            | Self::RecipeRatification
            | Self::GentleRecoveryTrial => FunctionDimension::Competition,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum FunctionPhase {
    Preparation,
    Gathering,
    Eve,
    AstronomicalApex,
    Celebration,
    ReturnOrDeparture,
    Archive,
}

impl FunctionPhase {
    pub const ALL: [Self; 7] = [
        Self::Preparation,
        Self::Gathering,
        Self::Eve,
        Self::AstronomicalApex,
        Self::Celebration,
        Self::ReturnOrDeparture,
        Self::Archive,
    ];
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FunctionPhaseRecord {
    pub id: FunctionPhaseId,
    pub function_id: GreatFunctionId,
    pub phase: FunctionPhase,
    pub occurs_at: AuthoritativeTimestamp,
    pub evidence_ids: BTreeSet<CalendarEvidenceId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IncarnationalPrinciple {
    EternalChristmas,
}

impl IncarnationalPrinciple {
    #[must_use]
    pub const fn is_fixed_winter_date(self) -> bool {
        false
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GreatFunctionRecord {
    pub function_id: GreatFunctionId,
    pub canonical_year_id: CanonicalYearId,
    pub kind: GreatFunctionKind,
    pub anchor: SeasonalAnchor,
    pub presiding_house: House,
    pub canonical_name: String,
    pub aliases: BTreeSet<String>,
    pub sacred_motion: SacredMotion,
    pub dimensions: FunctionDimensions,
    pub opens_at: AuthoritativeTimestamp,
    pub apex_at: AuthoritativeTimestamp,
    pub closes_at: AuthoritativeTimestamp,
    pub phases: Vec<FunctionPhaseRecord>,
    pub activities: BTreeSet<FunctionActivity>,
    pub event_ids: BTreeSet<SeasonalEventId>,
    pub venue_ids: BTreeSet<SeasonalVenueId>,
    pub evidence_ids: BTreeSet<CalendarEvidenceId>,
    pub incarnational_principle: Option<IncarnationalPrinciple>,
    pub presiding_house_owns_central_junction: bool,
    pub transfers_permanent_sovereignty: bool,
}

impl GreatFunctionRecord {
    #[must_use]
    pub fn canonicalized(&self) -> Self {
        let mut record = self.clone();
        record
            .phases
            .sort_by_key(|phase| (phase.phase, phase.id.as_str().to_owned()));
        record
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CentralJunctionSeasonalRuntime {
    year: CanonicalYearRecord,
    functions: BTreeMap<GreatFunctionId, GreatFunctionRecord>,
    by_anchor: BTreeMap<SeasonalAnchor, GreatFunctionId>,
    name_registry: BTreeMap<String, GreatFunctionId>,
}

impl CentralJunctionSeasonalRuntime {
    pub fn replay(
        year: CanonicalYearRecord,
        functions: &[GreatFunctionRecord],
    ) -> Result<Self, SeasonalFunctionError> {
        year.validate()?;
        if functions.len() != GreatFunctionKind::ALL.len() {
            return Err(SeasonalFunctionError::IncorrectFunctionCount(
                functions.len(),
            ));
        }
        let mut ordered = functions
            .iter()
            .map(GreatFunctionRecord::canonicalized)
            .collect::<Vec<_>>();
        ordered.sort_by_key(|function| (function.anchor, function.function_id.as_str().to_owned()));
        let mut by_id = BTreeMap::new();
        let mut by_anchor = BTreeMap::new();
        let mut names = BTreeMap::new();
        for function in ordered {
            validate_function(&year, &function)?;
            if by_id
                .insert(function.function_id.clone(), function.clone())
                .is_some()
            {
                return Err(SeasonalFunctionError::DuplicateFunction(
                    function.function_id,
                ));
            }
            if by_anchor
                .insert(function.anchor, function.function_id.clone())
                .is_some()
            {
                return Err(SeasonalFunctionError::DuplicateAnchor(function.anchor));
            }
            for name in std::iter::once(function.canonical_name.as_str())
                .chain(function.aliases.iter().map(String::as_str))
            {
                if names
                    .insert(name.to_owned(), function.function_id.clone())
                    .is_some()
                {
                    return Err(SeasonalFunctionError::DuplicateName(name.to_owned()));
                }
            }
        }
        if by_anchor.keys().copied().collect::<BTreeSet<_>>()
            != SeasonalAnchor::ALL.into_iter().collect()
        {
            return Err(SeasonalFunctionError::IncompleteFunctionAnchors);
        }
        Ok(Self {
            year,
            functions: by_id,
            by_anchor,
            name_registry: names,
        })
    }

    #[must_use]
    pub const fn year(&self) -> &CanonicalYearRecord {
        &self.year
    }

    #[must_use]
    pub fn functions(&self) -> &BTreeMap<GreatFunctionId, GreatFunctionRecord> {
        &self.functions
    }

    #[must_use]
    pub fn function_at_anchor(&self, anchor: SeasonalAnchor) -> Option<&GreatFunctionRecord> {
        self.by_anchor
            .get(&anchor)
            .and_then(|id| self.functions.get(id))
    }

    #[must_use]
    pub fn resolve_name(&self, name: &str) -> Option<&GreatFunctionRecord> {
        self.name_registry
            .get(name)
            .and_then(|id| self.functions.get(id))
    }
}

fn validate_function(
    year: &CanonicalYearRecord,
    function: &GreatFunctionRecord,
) -> Result<(), SeasonalFunctionError> {
    let kind = function.kind;
    let expected_id = GreatFunctionId::new(kind.stable_id())?;
    let expected_aliases = kind
        .aliases()
        .iter()
        .map(|alias| (*alias).to_owned())
        .collect::<BTreeSet<_>>();
    let observation = year
        .observation(function.anchor)
        .ok_or(SeasonalFunctionError::MissingAnchor(function.anchor))?;
    let expected_venues = kind
        .venue_ids()
        .iter()
        .map(|id| SeasonalVenueId::new(*id))
        .collect::<Result<BTreeSet<_>, _>>()?;
    if function.canonical_year_id != year.id
        || function.function_id != expected_id
        || function.anchor != kind.anchor()
        || function.presiding_house != kind.presiding_house()
        || function.canonical_name != kind.canonical_name()
        || function.aliases != expected_aliases
        || function.sacred_motion != kind.sacred_motion()
        || function.dimensions != kind.dimensions()
        || function.apex_at != observation.astronomical_instant
        || function.opens_at >= function.apex_at
        || function.apex_at >= function.closes_at
        || (kind != GreatFunctionKind::WayBack && function.opens_at < year.opens_at)
        || function.closes_at > year.closes_at
        || function.evidence_ids.is_empty()
        || function.activities.is_empty()
        || function.venue_ids != expected_venues
        || function.presiding_house_owns_central_junction
        || function.transfers_permanent_sovereignty
    {
        return Err(SeasonalFunctionError::InvalidFunction(
            function.function_id.clone(),
        ));
    }
    if kind == GreatFunctionKind::Initiation {
        if function.incarnational_principle != Some(IncarnationalPrinciple::EternalChristmas) {
            return Err(SeasonalFunctionError::InvalidIncarnationalMeaning(
                function.function_id.clone(),
            ));
        }
    } else if function.incarnational_principle.is_some() {
        return Err(SeasonalFunctionError::InvalidIncarnationalMeaning(
            function.function_id.clone(),
        ));
    }
    let represented_dimensions = function
        .activities
        .iter()
        .map(|activity| activity.dimension())
        .collect::<BTreeSet<_>>();
    if represented_dimensions
        != [
            FunctionDimension::Celebration,
            FunctionDimension::Ritual,
            FunctionDimension::Competition,
        ]
        .into_iter()
        .collect()
    {
        return Err(SeasonalFunctionError::MissingFunctionDimension(
            function.function_id.clone(),
        ));
    }
    validate_phases(function)
}

fn validate_phases(function: &GreatFunctionRecord) -> Result<(), SeasonalFunctionError> {
    if function.phases.len() != FunctionPhase::ALL.len() {
        return Err(SeasonalFunctionError::IncompleteLifecycle(
            function.function_id.clone(),
        ));
    }
    let mut by_phase = BTreeMap::new();
    let mut ids = BTreeSet::new();
    for phase in &function.phases {
        if phase.function_id != function.function_id
            || !ids.insert(phase.id.clone())
            || by_phase.insert(phase.phase, phase).is_some()
            || phase.evidence_ids.is_empty()
            || phase.occurs_at < function.opens_at
            || phase.occurs_at > function.closes_at
        {
            return Err(SeasonalFunctionError::InvalidLifecyclePhase(
                phase.id.clone(),
            ));
        }
    }
    if by_phase.keys().copied().collect::<BTreeSet<_>>() != FunctionPhase::ALL.into_iter().collect()
    {
        return Err(SeasonalFunctionError::IncompleteLifecycle(
            function.function_id.clone(),
        ));
    }
    let timestamps = FunctionPhase::ALL.map(|phase| by_phase[&phase].occurs_at.clone());
    if !timestamps.windows(2).all(|pair| pair[0] < pair[1])
        || by_phase[&FunctionPhase::AstronomicalApex].occurs_at != function.apex_at
    {
        return Err(SeasonalFunctionError::LifecycleOrderViolation(
            function.function_id.clone(),
        ));
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SeasonalFunctionError {
    InvalidIdentifier(String),
    Calendar(CanonicalCalendarError),
    IncorrectFunctionCount(usize),
    DuplicateFunction(GreatFunctionId),
    DuplicateAnchor(SeasonalAnchor),
    DuplicateName(String),
    IncompleteFunctionAnchors,
    MissingAnchor(SeasonalAnchor),
    InvalidFunction(GreatFunctionId),
    InvalidIncarnationalMeaning(GreatFunctionId),
    MissingFunctionDimension(GreatFunctionId),
    IncompleteLifecycle(GreatFunctionId),
    InvalidLifecyclePhase(FunctionPhaseId),
    LifecycleOrderViolation(GreatFunctionId),
}

impl fmt::Display for SeasonalFunctionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "Central Junction seasonal law rejected state: {self:?}"
        )
    }
}

impl std::error::Error for SeasonalFunctionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Calendar(error) => Some(error),
            _ => None,
        }
    }
}

impl From<CanonicalCalendarError> for SeasonalFunctionError {
    fn from(value: CanonicalCalendarError) -> Self {
        Self::Calendar(value)
    }
}
