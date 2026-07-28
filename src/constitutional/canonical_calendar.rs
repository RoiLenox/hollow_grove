//! Neutral canonical-year and astronomical-anchor records.
//!
//! This layer orders caller-supplied authoritative observations above the
//! constitutional runtime. It contains no House, festival, or presentation
//! semantics and performs no astronomical calculation.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};

pub const CANONICAL_CALENDAR_SOURCE: &str = "CENTRAL_JUNCTION_SEASONAL_FUNCTIONS_V1.md";
pub const CANONICAL_CALENDAR_SCHEMA_VERSION: u16 = 1;

macro_rules! calendar_id {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, CanonicalCalendarError> {
                let value = value.into();
                if value.is_empty()
                    || !value.bytes().all(|byte| {
                        byte.is_ascii_lowercase()
                            || byte.is_ascii_digit()
                            || matches!(byte, b'.' | b'-')
                    })
                {
                    return Err(CanonicalCalendarError::InvalidIdentifier(value));
                }
                Ok(Self(value))
            }

            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl TryFrom<String> for $name {
            type Error = CanonicalCalendarError;

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

calendar_id!(CanonicalYearId);
calendar_id!(AnchorObservationId);
calendar_id!(CalendarAuthorityId);
calendar_id!(CalendarEvidenceId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SeasonalAnchor {
    WinterSolstice,
    SpringEquinox,
    SummerSolstice,
    AutumnEquinox,
}

impl SeasonalAnchor {
    pub const ALL: [Self; 4] = [
        Self::WinterSolstice,
        Self::SpringEquinox,
        Self::SummerSolstice,
        Self::AutumnEquinox,
    ];

    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::WinterSolstice => "Winter Solstice",
            Self::SpringEquinox => "Spring Equinox",
            Self::SummerSolstice => "Summer Solstice",
            Self::AutumnEquinox => "Autumn Equinox",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct AuthoritativeTimestamp(String);

impl AuthoritativeTimestamp {
    pub fn new(value: impl Into<String>) -> Result<Self, CanonicalCalendarError> {
        let value = value.into();
        let bytes = value.as_bytes();
        let valid_shape = bytes.len() == 20
            && [4, 7].into_iter().all(|index| bytes[index] == b'-')
            && bytes[10] == b'T'
            && [13, 16].into_iter().all(|index| bytes[index] == b':')
            && bytes[19] == b'Z'
            && bytes.iter().enumerate().all(|(index, byte)| {
                matches!(index, 4 | 7 | 10 | 13 | 16 | 19) || byte.is_ascii_digit()
            });
        if !valid_shape {
            return Err(CanonicalCalendarError::InvalidTimestamp(value));
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for AuthoritativeTimestamp {
    type Error = CanonicalCalendarError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<AuthoritativeTimestamp> for String {
    fn from(value: AuthoritativeTimestamp) -> Self {
        value.0
    }
}

impl fmt::Display for AuthoritativeTimestamp {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct ObservedCivilDate(String);

impl ObservedCivilDate {
    pub fn new(value: impl Into<String>) -> Result<Self, CanonicalCalendarError> {
        let value = value.into();
        let bytes = value.as_bytes();
        let valid_shape = bytes.len() == 10
            && bytes[4] == b'-'
            && bytes[7] == b'-'
            && bytes
                .iter()
                .enumerate()
                .all(|(index, byte)| matches!(index, 4 | 7) || byte.is_ascii_digit());
        if !valid_shape {
            return Err(CanonicalCalendarError::InvalidCivilDate(value));
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for ObservedCivilDate {
    type Error = CanonicalCalendarError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<ObservedCivilDate> for String {
    fn from(value: ObservedCivilDate) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AstronomicalAnchorObservation {
    pub id: AnchorObservationId,
    pub canonical_year_id: CanonicalYearId,
    pub anchor: SeasonalAnchor,
    pub astronomical_instant: AuthoritativeTimestamp,
    pub observed_civil_date: ObservedCivilDate,
    pub supplied_by: CalendarAuthorityId,
    pub evidence_ids: BTreeSet<CalendarEvidenceId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalYearBoundary {
    pub next_year_id: CanonicalYearId,
    pub next_winter_solstice: AuthoritativeTimestamp,
    pub observed_civil_date: ObservedCivilDate,
    pub supplied_by: CalendarAuthorityId,
    pub evidence_ids: BTreeSet<CalendarEvidenceId>,
    pub closes_previous_and_opens_next: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalYearRecord {
    pub id: CanonicalYearId,
    pub opens_at: AuthoritativeTimestamp,
    pub closes_at: AuthoritativeTimestamp,
    pub anchor_observations: Vec<AstronomicalAnchorObservation>,
    pub boundary: CanonicalYearBoundary,
}

impl CanonicalYearRecord {
    pub fn validate(&self) -> Result<(), CanonicalCalendarError> {
        if self.opens_at >= self.closes_at
            || self.closes_at != self.boundary.next_winter_solstice
            || !self.boundary.closes_previous_and_opens_next
            || self.boundary.next_year_id == self.id
            || self.boundary.evidence_ids.is_empty()
        {
            return Err(CanonicalCalendarError::InvalidYearBoundary(self.id.clone()));
        }
        if self.anchor_observations.len() != SeasonalAnchor::ALL.len() {
            return Err(CanonicalCalendarError::IncompleteAnchorSet(self.id.clone()));
        }
        let mut by_anchor = BTreeMap::new();
        let mut ids = BTreeSet::new();
        for observation in &self.anchor_observations {
            if observation.canonical_year_id != self.id
                || !ids.insert(observation.id.clone())
                || by_anchor.insert(observation.anchor, observation).is_some()
                || observation.evidence_ids.is_empty()
                || observation.astronomical_instant < self.opens_at
                || observation.astronomical_instant >= self.closes_at
            {
                return Err(CanonicalCalendarError::InvalidAnchorObservation(
                    observation.id.clone(),
                ));
            }
        }
        let anchors = by_anchor.keys().copied().collect::<BTreeSet<_>>();
        if anchors != SeasonalAnchor::ALL.into_iter().collect() {
            return Err(CanonicalCalendarError::IncompleteAnchorSet(self.id.clone()));
        }
        let winter = by_anchor[&SeasonalAnchor::WinterSolstice];
        if winter.astronomical_instant != self.opens_at {
            return Err(CanonicalCalendarError::YearDoesNotOpenAtWinterSolstice(
                self.id.clone(),
            ));
        }
        let instants =
            SeasonalAnchor::ALL.map(|anchor| by_anchor[&anchor].astronomical_instant.clone());
        if !instants.windows(2).all(|pair| pair[0] < pair[1]) {
            return Err(CanonicalCalendarError::AnchorOrderViolation(
                self.id.clone(),
            ));
        }
        Ok(())
    }

    #[must_use]
    pub fn observation(&self, anchor: SeasonalAnchor) -> Option<&AstronomicalAnchorObservation> {
        self.anchor_observations
            .iter()
            .find(|observation| observation.anchor == anchor)
    }

    #[must_use]
    pub fn canonicalized(&self) -> Self {
        let mut year = self.clone();
        year.anchor_observations
            .sort_by_key(|observation| (observation.anchor, observation.id.as_str().to_owned()));
        year
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CanonicalCalendarError {
    InvalidIdentifier(String),
    InvalidTimestamp(String),
    InvalidCivilDate(String),
    InvalidYearBoundary(CanonicalYearId),
    IncompleteAnchorSet(CanonicalYearId),
    InvalidAnchorObservation(AnchorObservationId),
    YearDoesNotOpenAtWinterSolstice(CanonicalYearId),
    AnchorOrderViolation(CanonicalYearId),
}

impl fmt::Display for CanonicalCalendarError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "canonical calendar rejected state: {self:?}")
    }
}

impl std::error::Error for CanonicalCalendarError {}
