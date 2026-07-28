//! Constitutional semantics for the Current Sea, its forces, and Boardwalk.
//!
//! The body (`region.current-sea`), natural forces (`force.riptide` and
//! `force.undertow`), and lawful traveler route (`route.boardwalk`) are
//! deliberately type-distinct. Flynt Resynce and Recog remain separate lawful
//! events above the universal recursion kernel.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::constitutional::{AuthoritativeTimestamp, CalendarEvidenceId, CanonicalYearId};
use crate::hollow_grove_contract::House;

pub const CURRENT_SEA_REGION_ID: &str = "region.current-sea";
pub const RIPTIDE_FORCE_ID: &str = "force.riptide";
pub const UNDERTOW_FORCE_ID: &str = "force.undertow";
pub const BOARDWALK_ROUTE_ID: &str = "route.boardwalk";

macro_rules! current_sea_id {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, CurrentSeaValidationError> {
                let value = value.into();
                if value.is_empty()
                    || !value.bytes().all(|byte| {
                        byte.is_ascii_lowercase()
                            || byte.is_ascii_digit()
                            || matches!(byte, b'.' | b'-')
                    })
                {
                    return Err(CurrentSeaValidationError::InvalidIdentifier(value));
                }
                Ok(Self(value))
            }

            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl TryFrom<String> for $name {
            type Error = CurrentSeaValidationError;

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

current_sea_id!(CurrentSeaRegionId);
current_sea_id!(CurrentSeaEventId);
current_sea_id!(CurrentSeaForceId);
current_sea_id!(BoardwalkRouteId);
current_sea_id!(BoardwalkPassageId);
current_sea_id!(CurrentSeaTravelerId);
current_sea_id!(CurrentSeaClearanceId);
current_sea_id!(CurrentSeaAuthorityId);
current_sea_id!(CurrentSeaProvenanceId);
current_sea_id!(FlyntResynceEventId);
current_sea_id!(FlyntRecogEventId);
current_sea_id!(ManifestationConditionId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CurrentSeaState {
    Setting,
    Settled,
    Rising,
    Disturbed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CurrentSeaForce {
    Riptide,
    Undertow,
}

impl CurrentSeaForce {
    #[must_use]
    pub const fn stable_id(self) -> &'static str {
        match self {
            Self::Riptide => RIPTIDE_FORCE_ID,
            Self::Undertow => UNDERTOW_FORCE_ID,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CurrentSeaForceDirection {
    FromGlaushouseTowardFlynt,
    DownwardOrBeneath,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrentSeaEvent {
    pub event_id: CurrentSeaEventId,
    pub canonical_year_id: Option<CanonicalYearId>,
    pub region_id: CurrentSeaRegionId,
    pub state: CurrentSeaState,
    pub force: Option<CurrentSeaForce>,
    pub force_id: Option<CurrentSeaForceId>,
    pub force_direction: Option<CurrentSeaForceDirection>,
    pub origin_house: House,
    pub destination_house: Option<House>,
    pub evidence_ids: BTreeSet<CalendarEvidenceId>,
    pub constitutional_authority_id: CurrentSeaAuthorityId,
    pub provenance_id: CurrentSeaProvenanceId,
    pub occurred_at: AuthoritativeTimestamp,
}

impl CurrentSeaEvent {
    pub fn validate(&self) -> Result<(), CurrentSeaValidationError> {
        if self.region_id.as_str() != CURRENT_SEA_REGION_ID
            || self.origin_house != House::Glaushouse
            || self.evidence_ids.is_empty()
        {
            return Err(CurrentSeaValidationError::InvalidSeaEvent(
                self.event_id.clone(),
            ));
        }
        let valid_state = match self.state {
            CurrentSeaState::Setting | CurrentSeaState::Settled => {
                self.force.is_none()
                    && self.force_id.is_none()
                    && self.force_direction.is_none()
                    && self.destination_house.is_none()
            }
            CurrentSeaState::Rising => {
                self.force == Some(CurrentSeaForce::Riptide)
                    && self
                        .force_id
                        .as_ref()
                        .is_some_and(|id| id.as_str() == RIPTIDE_FORCE_ID)
                    && self.force_direction
                        == Some(CurrentSeaForceDirection::FromGlaushouseTowardFlynt)
                    && self.destination_house == Some(House::Flynt)
            }
            CurrentSeaState::Disturbed => {
                self.force == Some(CurrentSeaForce::Undertow)
                    && self
                        .force_id
                        .as_ref()
                        .is_some_and(|id| id.as_str() == UNDERTOW_FORCE_ID)
                    && self.force_direction == Some(CurrentSeaForceDirection::DownwardOrBeneath)
                    && self.destination_house.is_none()
            }
        };
        if !valid_state {
            return Err(CurrentSeaValidationError::InvalidSeaEvent(
                self.event_id.clone(),
            ));
        }
        Ok(())
    }

    #[must_use]
    pub const fn is_successful_rise(&self) -> bool {
        matches!(
            (
                self.state,
                self.force,
                self.force_direction,
                self.origin_house,
                self.destination_house,
            ),
            (
                CurrentSeaState::Rising,
                Some(CurrentSeaForce::Riptide),
                Some(CurrentSeaForceDirection::FromGlaushouseTowardFlynt),
                House::Glaushouse,
                Some(House::Flynt),
            )
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GlaushouseClearanceRecord {
    pub clearance_id: CurrentSeaClearanceId,
    pub traveler_id: CurrentSeaTravelerId,
    pub current_event_id: CurrentSeaEventId,
    pub evidence_ids: BTreeSet<CalendarEvidenceId>,
    pub constitutional_authority_id: CurrentSeaAuthorityId,
    pub provenance_id: CurrentSeaProvenanceId,
    pub cleared_at: AuthoritativeTimestamp,
    pub grants_flynt_recog: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoardwalkPassage {
    pub passage_id: BoardwalkPassageId,
    pub canonical_year_id: Option<CanonicalYearId>,
    pub traveler_id: CurrentSeaTravelerId,
    pub route_id: BoardwalkRouteId,
    pub origin_house: House,
    pub destination_house: House,
    pub associated_current_event_id: Option<CurrentSeaEventId>,
    pub clearance_ids: BTreeSet<CurrentSeaClearanceId>,
    pub evidence_ids: BTreeSet<CalendarEvidenceId>,
    pub constitutional_authority_id: CurrentSeaAuthorityId,
    pub provenance_id: CurrentSeaProvenanceId,
    pub opened_at: AuthoritativeTimestamp,
    pub completed_at: Option<AuthoritativeTimestamp>,
    pub arrived_at_destination: bool,
    pub grants_automatic_recog: bool,
}

impl BoardwalkPassage {
    fn validate_shape(&self) -> Result<(), CurrentSeaValidationError> {
        let valid_endpoints = matches!(
            (self.origin_house, self.destination_house),
            (House::Glaushouse, House::Flynt) | (House::Flynt, House::Glaushouse)
        );
        if self.route_id.as_str() != BOARDWALK_ROUTE_ID
            || !valid_endpoints
            || self.evidence_ids.is_empty()
            || self.grants_automatic_recog
            || self
                .completed_at
                .as_ref()
                .is_some_and(|completed| completed < &self.opened_at)
            || (self.arrived_at_destination && self.completed_at.is_none())
        {
            return Err(CurrentSeaValidationError::InvalidBoardwalkPassage(
                self.passage_id.clone(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlyntResynceRecord {
    pub event_id: FlyntResynceEventId,
    pub traveler_id: CurrentSeaTravelerId,
    pub boardwalk_passage_id: BoardwalkPassageId,
    pub evidence_ids: BTreeSet<CalendarEvidenceId>,
    pub constitutional_authority_id: CurrentSeaAuthorityId,
    pub provenance_id: CurrentSeaProvenanceId,
    pub occurred_at: AuthoritativeTimestamp,
    pub accepted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlyntRecogRecord {
    pub event_id: FlyntRecogEventId,
    pub traveler_id: CurrentSeaTravelerId,
    pub resynce_event_id: FlyntResynceEventId,
    pub manifestation_condition_ids: BTreeSet<ManifestationConditionId>,
    pub evidence_ids: BTreeSet<CalendarEvidenceId>,
    pub constitutional_authority_id: CurrentSeaAuthorityId,
    pub provenance_id: CurrentSeaProvenanceId,
    pub occurred_at: AuthoritativeTimestamp,
    pub accepted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrentSeaRuntime {
    events: BTreeMap<CurrentSeaEventId, CurrentSeaEvent>,
    clearances: BTreeMap<CurrentSeaClearanceId, GlaushouseClearanceRecord>,
    passages: BTreeMap<BoardwalkPassageId, BoardwalkPassage>,
    resynce_events: BTreeMap<FlyntResynceEventId, FlyntResynceRecord>,
    recog_events: BTreeMap<FlyntRecogEventId, FlyntRecogRecord>,
}

impl CurrentSeaRuntime {
    pub fn replay(
        events: &[CurrentSeaEvent],
        clearances: &[GlaushouseClearanceRecord],
        passages: &[BoardwalkPassage],
        resynce_events: &[FlyntResynceRecord],
        recog_events: &[FlyntRecogRecord],
    ) -> Result<Self, CurrentSeaValidationError> {
        let mut event_map = BTreeMap::new();
        for event in events {
            event.validate()?;
            if event_map
                .insert(event.event_id.clone(), event.clone())
                .is_some()
            {
                return Err(CurrentSeaValidationError::DuplicateRecord);
            }
        }

        let mut clearance_map = BTreeMap::new();
        for clearance in clearances {
            let event = event_map.get(&clearance.current_event_id).ok_or_else(|| {
                CurrentSeaValidationError::InvalidClearance(clearance.clearance_id.clone())
            })?;
            if event.origin_house != House::Glaushouse
                || clearance.evidence_ids.is_empty()
                || clearance.grants_flynt_recog
                || clearance.cleared_at < event.occurred_at
                || clearance_map
                    .insert(clearance.clearance_id.clone(), clearance.clone())
                    .is_some()
            {
                return Err(CurrentSeaValidationError::InvalidClearance(
                    clearance.clearance_id.clone(),
                ));
            }
        }

        let mut passage_map = BTreeMap::new();
        for passage in passages {
            passage.validate_shape()?;
            if passage.clearance_ids.iter().any(|id| {
                clearance_map.get(id).is_none_or(|clearance| {
                    clearance.traveler_id != passage.traveler_id
                        || clearance.cleared_at > passage.opened_at
                })
            }) {
                return Err(CurrentSeaValidationError::InvalidBoardwalkPassage(
                    passage.passage_id.clone(),
                ));
            }
            if let Some(event_id) = &passage.associated_current_event_id {
                let event = event_map.get(event_id).ok_or_else(|| {
                    CurrentSeaValidationError::InvalidBoardwalkPassage(passage.passage_id.clone())
                })?;
                if passage.origin_house != House::Glaushouse
                    || passage.destination_house != House::Flynt
                    || !event.is_successful_rise()
                    || event.occurred_at > passage.opened_at
                {
                    return Err(CurrentSeaValidationError::InvalidBoardwalkPassage(
                        passage.passage_id.clone(),
                    ));
                }
            }
            if passage_map
                .insert(passage.passage_id.clone(), passage.clone())
                .is_some()
            {
                return Err(CurrentSeaValidationError::DuplicateRecord);
            }
        }

        let mut resynce_map = BTreeMap::new();
        for resynce in resynce_events {
            let passage = passage_map
                .get(&resynce.boardwalk_passage_id)
                .ok_or_else(|| {
                    CurrentSeaValidationError::InvalidResynce(resynce.event_id.clone())
                })?;
            if resynce.traveler_id != passage.traveler_id
                || passage.destination_house != House::Flynt
                || !passage.arrived_at_destination
                || resynce.evidence_ids.is_empty()
                || passage
                    .completed_at
                    .as_ref()
                    .is_none_or(|completed| completed > &resynce.occurred_at)
                || resynce_map
                    .insert(resynce.event_id.clone(), resynce.clone())
                    .is_some()
            {
                return Err(CurrentSeaValidationError::InvalidResynce(
                    resynce.event_id.clone(),
                ));
            }
        }

        let mut recog_map = BTreeMap::new();
        for recog in recog_events {
            let resynce = resynce_map
                .get(&recog.resynce_event_id)
                .ok_or_else(|| CurrentSeaValidationError::InvalidRecog(recog.event_id.clone()))?;
            if recog.traveler_id != resynce.traveler_id
                || !resynce.accepted
                || recog.evidence_ids.is_empty()
                || recog.manifestation_condition_ids.is_empty()
                || recog.occurred_at < resynce.occurred_at
                || recog_map
                    .insert(recog.event_id.clone(), recog.clone())
                    .is_some()
            {
                return Err(CurrentSeaValidationError::InvalidRecog(
                    recog.event_id.clone(),
                ));
            }
        }

        Ok(Self {
            events: event_map,
            clearances: clearance_map,
            passages: passage_map,
            resynce_events: resynce_map,
            recog_events: recog_map,
        })
    }

    #[must_use]
    pub fn events(&self) -> &BTreeMap<CurrentSeaEventId, CurrentSeaEvent> {
        &self.events
    }

    #[must_use]
    pub fn clearances(&self) -> &BTreeMap<CurrentSeaClearanceId, GlaushouseClearanceRecord> {
        &self.clearances
    }

    #[must_use]
    pub fn passages(&self) -> &BTreeMap<BoardwalkPassageId, BoardwalkPassage> {
        &self.passages
    }

    #[must_use]
    pub fn resynce_events(&self) -> &BTreeMap<FlyntResynceEventId, FlyntResynceRecord> {
        &self.resynce_events
    }

    #[must_use]
    pub fn recog_events(&self) -> &BTreeMap<FlyntRecogEventId, FlyntRecogRecord> {
        &self.recog_events
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CurrentSeaValidationError {
    InvalidIdentifier(String),
    InvalidSeaEvent(CurrentSeaEventId),
    InvalidClearance(CurrentSeaClearanceId),
    InvalidBoardwalkPassage(BoardwalkPassageId),
    InvalidResynce(FlyntResynceEventId),
    InvalidRecog(FlyntRecogEventId),
    DuplicateRecord,
}

impl fmt::Display for CurrentSeaValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Current Sea rejected state: {self:?}")
    }
}

impl std::error::Error for CurrentSeaValidationError {}
