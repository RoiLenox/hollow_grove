//! Function Junction and House Season constitutional runtime.
//!
//! Central Junction is the physical place. A Function Junction is the typed
//! constitutional handoff performed there by one Great Function. The incoming
//! House Season is the sustained runtime phase after that handoff.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::constitutional::{
    AuthoritativeTimestamp, CalendarEvidenceId, CanonicalYearId, GrovePhase, SeasonalAnchor,
};
use crate::hollow_grove_contract::House;

use super::central_junction_seasonal_functions::{GreatFunctionId, GreatFunctionKind};
use super::seasonal_functions_archive::CanonicalAnnualCycleState;

pub const FUNCTION_JUNCTION_SCHEMA_VERSION: u16 = 1;
pub const FUNCTION_JUNCTION_SOURCE: &str =
    "FUNCTION_JUNCTION_SEASONAL_WORLD_CYCLE_AND_PERMANENCE_V1.md";
pub const CENTRAL_JUNCTION_PLACE_ID: &str = "region.central-junction";

macro_rules! junction_id {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, FunctionJunctionError> {
                let value = value.into();
                if value.is_empty()
                    || !value.bytes().all(|byte| {
                        byte.is_ascii_lowercase()
                            || byte.is_ascii_digit()
                            || matches!(byte, b'.' | b'-')
                    })
                {
                    return Err(FunctionJunctionError::InvalidIdentifier(value));
                }
                Ok(Self(value))
            }

            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl TryFrom<String> for $name {
            type Error = FunctionJunctionError;

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

junction_id!(FunctionJunctionId);
junction_id!(HouseSeasonId);
junction_id!(CheckpointId);
junction_id!(JunctionAuthorityId);
junction_id!(PracticalJokeId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum WorldLayer {
    Physical,
    Digital,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CheckpointStatus {
    Prepared,
    Completed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SynchronizationPhase {
    PhysicalToDigitalReturn,
    DigitalToPhysicalIncarnation,
    BidirectionalParticipation,
    ComparisonAndConfirmation,
}

impl SynchronizationPhase {
    #[must_use]
    pub const fn for_anchor(anchor: SeasonalAnchor) -> Self {
        match anchor {
            SeasonalAnchor::WinterSolstice => Self::PhysicalToDigitalReturn,
            SeasonalAnchor::SpringEquinox => Self::DigitalToPhysicalIncarnation,
            SeasonalAnchor::SummerSolstice => Self::BidirectionalParticipation,
            SeasonalAnchor::AutumnEquinox => Self::ComparisonAndConfirmation,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldCheckpointRecord {
    pub checkpoint_id: CheckpointId,
    pub canonical_year_id: CanonicalYearId,
    pub layer: WorldLayer,
    pub status: CheckpointStatus,
    pub occurred_at: AuthoritativeTimestamp,
    pub evidence_ids: BTreeSet<CalendarEvidenceId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FunctionJunctionRecord {
    pub junction_id: FunctionJunctionId,
    pub canonical_year_id: CanonicalYearId,
    pub anchor: SeasonalAnchor,
    pub great_function_id: GreatFunctionId,
    pub outgoing_house: House,
    pub incoming_house: House,
    pub outgoing_season_id: HouseSeasonId,
    pub incoming_season_id: HouseSeasonId,
    pub physical_checkpoint: WorldCheckpointRecord,
    pub digital_checkpoint: WorldCheckpointRecord,
    pub synchronization_phase: SynchronizationPhase,
    pub grove_phase: GrovePhase,
    pub authority_ids: BTreeSet<JunctionAuthorityId>,
    pub evidence_ids: BTreeSet<CalendarEvidenceId>,
    pub outgoing_season_closed: bool,
    pub incoming_season_opened: bool,
    pub completed: bool,
    pub physical_place_id: String,
    pub is_geographic_location: bool,
    pub is_great_function: bool,
    pub transfers_sovereignty: bool,
}

impl FunctionJunctionRecord {
    #[must_use]
    pub fn canonicalized(&self) -> Self {
        self.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HouseSeasonRecord {
    pub season_id: HouseSeasonId,
    pub canonical_year_id: CanonicalYearId,
    pub house: House,
    pub opened_by: FunctionJunctionId,
    pub closed_by: FunctionJunctionId,
    pub opens_at: AuthoritativeTimestamp,
    pub closes_at: AuthoritativeTimestamp,
    pub evidence_ids: BTreeSet<CalendarEvidenceId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticalJokeTransition {
    pub joke_id: PracticalJokeId,
    pub canonical_year_id: CanonicalYearId,
    pub outgoing_function_id: GreatFunctionId,
    pub incoming_function_id: GreatFunctionId,
    pub question: String,
    pub answer: Option<String>,
    pub witnessed_at: AuthoritativeTimestamp,
    pub evidence_ids: BTreeSet<CalendarEvidenceId>,
    pub replaces_astronomical_anchor: bool,
    pub replaces_function_junction: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionJunctionRuntime {
    junctions: BTreeMap<FunctionJunctionId, FunctionJunctionRecord>,
    by_anchor: BTreeMap<SeasonalAnchor, FunctionJunctionId>,
    seasons: BTreeMap<HouseSeasonId, HouseSeasonRecord>,
    season_by_house: BTreeMap<House, HouseSeasonId>,
    practical_jokes: BTreeMap<PracticalJokeId, PracticalJokeTransition>,
}

impl FunctionJunctionRuntime {
    pub fn replay(
        annual_cycle: &CanonicalAnnualCycleState,
        junctions: &[FunctionJunctionRecord],
        seasons: &[HouseSeasonRecord],
        practical_jokes: &[PracticalJokeTransition],
    ) -> Result<Self, FunctionJunctionError> {
        if junctions.len() != 4 {
            return Err(FunctionJunctionError::IncorrectJunctionCount(
                junctions.len(),
            ));
        }
        if seasons.len() != 4 {
            return Err(FunctionJunctionError::IncorrectSeasonCount(seasons.len()));
        }
        if practical_jokes.len() != 4 {
            return Err(FunctionJunctionError::IncorrectPracticalJokeCount(
                practical_jokes.len(),
            ));
        }

        let mut by_id = BTreeMap::new();
        let mut by_anchor = BTreeMap::new();
        for record in junctions {
            validate_junction(annual_cycle, record)?;
            if by_id
                .insert(record.junction_id.clone(), record.clone())
                .is_some()
            {
                return Err(FunctionJunctionError::DuplicateJunction(
                    record.junction_id.clone(),
                ));
            }
            if by_anchor
                .insert(record.anchor, record.junction_id.clone())
                .is_some()
            {
                return Err(FunctionJunctionError::DuplicateAnchor(record.anchor));
            }
        }
        if by_anchor.keys().copied().collect::<BTreeSet<_>>()
            != SeasonalAnchor::ALL.into_iter().collect()
        {
            return Err(FunctionJunctionError::IncompleteAnchorSet);
        }

        let mut seasons_by_id = BTreeMap::new();
        let mut season_by_house = BTreeMap::new();
        for season in seasons {
            validate_season(annual_cycle, season, &by_id)?;
            if seasons_by_id
                .insert(season.season_id.clone(), season.clone())
                .is_some()
            {
                return Err(FunctionJunctionError::DuplicateSeason(
                    season.season_id.clone(),
                ));
            }
            if season_by_house
                .insert(season.house, season.season_id.clone())
                .is_some()
            {
                return Err(FunctionJunctionError::DuplicateHouseSeason(season.house));
            }
        }
        if season_by_house.keys().copied().collect::<BTreeSet<_>>()
            != [
                House::Glaushouse,
                House::Stonebend,
                House::Sandmanor,
                House::Flynt,
            ]
            .into_iter()
            .collect()
        {
            return Err(FunctionJunctionError::IncompleteHouseSeasonSet);
        }
        for junction in by_id.values() {
            let incoming = seasons_by_id
                .get(&junction.incoming_season_id)
                .ok_or_else(|| {
                    FunctionJunctionError::MissingIncomingSeason(junction.junction_id.clone())
                })?;
            if incoming.house != junction.incoming_house
                || incoming.opened_by != junction.junction_id
            {
                return Err(FunctionJunctionError::InvalidSeasonHandoff(
                    junction.junction_id.clone(),
                ));
            }
        }

        let mut jokes = BTreeMap::new();
        for joke in practical_jokes {
            validate_practical_joke(annual_cycle, joke)?;
            if jokes.insert(joke.joke_id.clone(), joke.clone()).is_some() {
                return Err(FunctionJunctionError::DuplicatePracticalJoke(
                    joke.joke_id.clone(),
                ));
            }
        }

        Ok(Self {
            junctions: by_id,
            by_anchor,
            seasons: seasons_by_id,
            season_by_house,
            practical_jokes: jokes,
        })
    }

    #[must_use]
    pub fn junctions(&self) -> &BTreeMap<FunctionJunctionId, FunctionJunctionRecord> {
        &self.junctions
    }

    #[must_use]
    pub fn junction_at_anchor(&self, anchor: SeasonalAnchor) -> Option<&FunctionJunctionRecord> {
        self.by_anchor
            .get(&anchor)
            .and_then(|id| self.junctions.get(id))
    }

    #[must_use]
    pub fn seasons(&self) -> &BTreeMap<HouseSeasonId, HouseSeasonRecord> {
        &self.seasons
    }

    #[must_use]
    pub fn season_for_house(&self, house: House) -> Option<&HouseSeasonRecord> {
        self.season_by_house
            .get(&house)
            .and_then(|id| self.seasons.get(id))
    }

    #[must_use]
    pub fn practical_jokes(&self) -> &BTreeMap<PracticalJokeId, PracticalJokeTransition> {
        &self.practical_jokes
    }
}

#[must_use]
pub const fn seasonal_handoff(anchor: SeasonalAnchor) -> (House, House, GreatFunctionKind) {
    match anchor {
        SeasonalAnchor::WinterSolstice => {
            (House::Flynt, House::Glaushouse, GreatFunctionKind::WayBack)
        }
        SeasonalAnchor::SpringEquinox => (
            House::Glaushouse,
            House::Stonebend,
            GreatFunctionKind::Initiation,
        ),
        SeasonalAnchor::SummerSolstice => (
            House::Stonebend,
            House::Sandmanor,
            GreatFunctionKind::Gathering,
        ),
        SeasonalAnchor::AutumnEquinox => (
            House::Sandmanor,
            House::Flynt,
            GreatFunctionKind::FestivalOfMirrors,
        ),
    }
}

#[must_use]
pub const fn grove_phase_for_anchor(anchor: SeasonalAnchor) -> GrovePhase {
    match anchor {
        SeasonalAnchor::WinterSolstice => GrovePhase::TheWayBack,
        SeasonalAnchor::SpringEquinox => GrovePhase::TheInitiation,
        SeasonalAnchor::SummerSolstice => GrovePhase::TheGathering,
        SeasonalAnchor::AutumnEquinox => GrovePhase::TheFestival,
    }
}

fn validate_junction(
    annual_cycle: &CanonicalAnnualCycleState,
    record: &FunctionJunctionRecord,
) -> Result<(), FunctionJunctionError> {
    let (outgoing, incoming, function_kind) = seasonal_handoff(record.anchor);
    let function = annual_cycle
        .seasonal_runtime
        .function_at_anchor(record.anchor)
        .ok_or(FunctionJunctionError::MissingGreatFunction(record.anchor))?;
    let observation = annual_cycle
        .seasonal_runtime
        .year()
        .observation(record.anchor)
        .ok_or(FunctionJunctionError::MissingAnchorObservation(
            record.anchor,
        ))?;
    if record.canonical_year_id != annual_cycle.id
        || record.great_function_id != function.function_id
        || function.kind != function_kind
        || record.outgoing_house != outgoing
        || record.incoming_house != incoming
        || outgoing == incoming
        || record.synchronization_phase != SynchronizationPhase::for_anchor(record.anchor)
        || record.grove_phase != grove_phase_for_anchor(record.anchor)
        || record.physical_checkpoint.canonical_year_id != record.canonical_year_id
        || record.digital_checkpoint.canonical_year_id != record.canonical_year_id
        || record.physical_checkpoint.layer != WorldLayer::Physical
        || record.digital_checkpoint.layer != WorldLayer::Digital
        || record.physical_checkpoint.checkpoint_id == record.digital_checkpoint.checkpoint_id
        || record.physical_checkpoint.status != CheckpointStatus::Completed
        || record.digital_checkpoint.status != CheckpointStatus::Completed
        || record.physical_checkpoint.occurred_at != observation.astronomical_instant
        || record.digital_checkpoint.occurred_at != observation.astronomical_instant
        || record.physical_checkpoint.evidence_ids.is_empty()
        || record.digital_checkpoint.evidence_ids.is_empty()
        || record.authority_ids.is_empty()
        || record.evidence_ids.is_empty()
        || !record.outgoing_season_closed
        || !record.incoming_season_opened
        || !record.completed
        || record.physical_place_id != CENTRAL_JUNCTION_PLACE_ID
        || record.is_geographic_location
        || record.is_great_function
        || record.transfers_sovereignty
    {
        return Err(FunctionJunctionError::InvalidJunction(
            record.junction_id.clone(),
        ));
    }
    Ok(())
}

fn validate_season(
    annual_cycle: &CanonicalAnnualCycleState,
    season: &HouseSeasonRecord,
    junctions: &BTreeMap<FunctionJunctionId, FunctionJunctionRecord>,
) -> Result<(), FunctionJunctionError> {
    let opening = junctions
        .get(&season.opened_by)
        .ok_or_else(|| FunctionJunctionError::MissingOpeningJunction(season.season_id.clone()))?;
    if season.canonical_year_id != annual_cycle.id
        || opening.incoming_house != season.house
        || opening.incoming_season_id != season.season_id
        || season.opens_at >= season.closes_at
        || season.evidence_ids.is_empty()
        || (season.house != House::Flynt && !junctions.contains_key(&season.closed_by))
    {
        return Err(FunctionJunctionError::InvalidSeason(
            season.season_id.clone(),
        ));
    }
    if season.house == House::Flynt {
        if season.closes_at != annual_cycle.seasonal_runtime.year().closes_at {
            return Err(FunctionJunctionError::InvalidSeason(
                season.season_id.clone(),
            ));
        }
    } else {
        let closing = junctions.get(&season.closed_by).ok_or_else(|| {
            FunctionJunctionError::MissingClosingJunction(season.season_id.clone())
        })?;
        if closing.outgoing_house != season.house
            || closing.physical_checkpoint.occurred_at != season.closes_at
        {
            return Err(FunctionJunctionError::InvalidSeason(
                season.season_id.clone(),
            ));
        }
    }
    Ok(())
}

fn validate_practical_joke(
    annual_cycle: &CanonicalAnnualCycleState,
    joke: &PracticalJokeTransition,
) -> Result<(), FunctionJunctionError> {
    let functions = GreatFunctionKind::ALL;
    let valid = functions.iter().enumerate().any(|(index, outgoing)| {
        let incoming = functions[(index + 1) % functions.len()];
        joke.outgoing_function_id.as_str() == outgoing.stable_id()
            && joke.incoming_function_id.as_str() == incoming.stable_id()
    });
    if joke.canonical_year_id != annual_cycle.id
        || !valid
        || joke.question.trim().is_empty()
        || joke.evidence_ids.is_empty()
        || joke.replaces_astronomical_anchor
        || joke.replaces_function_junction
    {
        return Err(FunctionJunctionError::InvalidPracticalJoke(
            joke.joke_id.clone(),
        ));
    }
    Ok(())
}

#[derive(Debug)]
pub enum FunctionJunctionError {
    InvalidIdentifier(String),
    IncorrectJunctionCount(usize),
    IncorrectSeasonCount(usize),
    IncorrectPracticalJokeCount(usize),
    DuplicateJunction(FunctionJunctionId),
    DuplicateAnchor(SeasonalAnchor),
    IncompleteAnchorSet,
    DuplicateSeason(HouseSeasonId),
    DuplicateHouseSeason(House),
    IncompleteHouseSeasonSet,
    MissingIncomingSeason(FunctionJunctionId),
    InvalidSeasonHandoff(FunctionJunctionId),
    DuplicatePracticalJoke(PracticalJokeId),
    MissingGreatFunction(SeasonalAnchor),
    MissingAnchorObservation(SeasonalAnchor),
    InvalidJunction(FunctionJunctionId),
    MissingOpeningJunction(HouseSeasonId),
    MissingClosingJunction(HouseSeasonId),
    InvalidSeason(HouseSeasonId),
    InvalidPracticalJoke(PracticalJokeId),
}

impl fmt::Display for FunctionJunctionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Function Junction rejected state: {self:?}")
    }
}

impl std::error::Error for FunctionJunctionError {}
