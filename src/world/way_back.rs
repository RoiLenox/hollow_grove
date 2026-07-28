//! Directional constitutional law for the permanent Way Back passage.
//!
//! Aura Way is the complete Stonebend–Flynt route. The recurring Winter
//! Function may foreground a passage without owning the route or limiting its
//! permanent availability. This House-specific law remains above the neutral
//! calendar and the universal recursion kernel.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::constitutional::{AuthoritativeTimestamp, CalendarEvidenceId, CanonicalYearId};
use crate::hollow_grove_contract::House;

use super::central_junction_seasonal_functions::{GreatFunctionId, GreatFunctionKind};

pub const AURA_WAY_ROUTE_ID: &str = "route.aura-way";
pub const THE_WAY_BACK_RULE_ID: &str = "passage.the-way-back";
pub const STAIRWAY_TO_HEAVEN_SEGMENT_ID: &str = "route-segment.stairway-to-heaven";

macro_rules! way_back_id {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, WayBackValidationError> {
                let value = value.into();
                if value.is_empty()
                    || !value.bytes().all(|byte| {
                        byte.is_ascii_lowercase()
                            || byte.is_ascii_digit()
                            || matches!(byte, b'.' | b'-')
                    })
                {
                    return Err(WayBackValidationError::InvalidIdentifier(value));
                }
                Ok(Self(value))
            }

            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl TryFrom<String> for $name {
            type Error = WayBackValidationError;

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

way_back_id!(WayBackRouteId);
way_back_id!(WayBackRuleId);
way_back_id!(WayBackRouteSegmentId);
way_back_id!(WayBackPassageId);
way_back_id!(WayBackTravelerId);
way_back_id!(WayBackClearanceId);
way_back_id!(WayBackAuthorityId);
way_back_id!(WayBackProvenanceId);
way_back_id!(WayBackSupportId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum WayBackDirection {
    DescendingFromStonebend,
    AscendingFromFlynt,
}

impl WayBackDirection {
    #[must_use]
    pub const fn origin(self) -> House {
        match self {
            Self::DescendingFromStonebend => House::Stonebend,
            Self::AscendingFromFlynt => House::Flynt,
        }
    }

    #[must_use]
    pub const fn destination(self) -> House {
        match self {
            Self::DescendingFromStonebend => House::Flynt,
            Self::AscendingFromFlynt => House::Stonebend,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum WayBackExpression {
    AuraWay,
    StairwayToHeaven,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WayBackRouteDefinition {
    pub route_id: WayBackRouteId,
    pub rule_id: WayBackRuleId,
    pub stairway_segment_id: WayBackRouteSegmentId,
    pub endpoints: [House; 2],
    pub route_owner: Option<House>,
    pub valid_outside_winter_function: bool,
}

impl WayBackRouteDefinition {
    pub fn validate(&self) -> Result<(), WayBackValidationError> {
        if self.route_id.as_str() != AURA_WAY_ROUTE_ID
            || self.rule_id.as_str() != THE_WAY_BACK_RULE_ID
            || self.stairway_segment_id.as_str() != STAIRWAY_TO_HEAVEN_SEGMENT_ID
            || self.endpoints != [House::Stonebend, House::Flynt]
            || self.route_owner.is_some()
            || !self.valid_outside_winter_function
        {
            return Err(WayBackValidationError::InvalidRouteDefinition);
        }
        Ok(())
    }
}

#[must_use]
pub fn canonical_way_back_route() -> WayBackRouteDefinition {
    WayBackRouteDefinition {
        route_id: WayBackRouteId::new(AURA_WAY_ROUTE_ID).expect("canonical Aura Way route ID"),
        rule_id: WayBackRuleId::new(THE_WAY_BACK_RULE_ID).expect("canonical Way Back rule ID"),
        stairway_segment_id: WayBackRouteSegmentId::new(STAIRWAY_TO_HEAVEN_SEGMENT_ID)
            .expect("canonical Stairway segment ID"),
        endpoints: [House::Stonebend, House::Flynt],
        route_owner: None,
        valid_outside_winter_function: true,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum WayBackSupportRole {
    GlaushouseCareAndClearance,
    SandmanorArrangement,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WayBackSupportRecord {
    pub support_id: WayBackSupportId,
    pub house: House,
    pub role: WayBackSupportRole,
    pub authority_id: WayBackAuthorityId,
    pub evidence_ids: BTreeSet<CalendarEvidenceId>,
    pub claims_route_ownership: bool,
}

impl WayBackSupportRecord {
    fn validate(&self) -> Result<(), WayBackValidationError> {
        let correct_house = matches!(
            (self.house, self.role),
            (
                House::Glaushouse,
                WayBackSupportRole::GlaushouseCareAndClearance
            ) | (House::Sandmanor, WayBackSupportRole::SandmanorArrangement)
        );
        if !correct_house || self.claims_route_ownership || self.evidence_ids.is_empty() {
            return Err(WayBackValidationError::InvalidSupport(
                self.support_id.clone(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WayBackPassage {
    pub passage_id: WayBackPassageId,
    pub canonical_year_id: Option<CanonicalYearId>,
    pub traveler_id: WayBackTravelerId,
    pub direction: WayBackDirection,
    pub route_id: WayBackRouteId,
    pub origin_house: House,
    pub destination_house: House,
    pub expression: WayBackExpression,
    pub function_id: Option<GreatFunctionId>,
    pub clearance_ids: BTreeSet<WayBackClearanceId>,
    pub evidence_ids: BTreeSet<CalendarEvidenceId>,
    pub constitutional_authority_id: WayBackAuthorityId,
    pub provenance_id: WayBackProvenanceId,
    pub support: Vec<WayBackSupportRecord>,
    pub opened_at: AuthoritativeTimestamp,
    pub completed_at: Option<AuthoritativeTimestamp>,
}

impl WayBackPassage {
    #[must_use]
    pub fn canonicalized(&self) -> Self {
        let mut passage = self.clone();
        passage
            .support
            .sort_by(|left, right| left.support_id.cmp(&right.support_id));
        passage
    }

    pub fn validate(&self, route: &WayBackRouteDefinition) -> Result<(), WayBackValidationError> {
        route.validate()?;
        if self.route_id != route.route_id
            || self.origin_house != self.direction.origin()
            || self.destination_house != self.direction.destination()
            || self.evidence_ids.is_empty()
            || self
                .completed_at
                .as_ref()
                .is_some_and(|completed| completed < &self.opened_at)
        {
            return Err(WayBackValidationError::InvalidPassage(
                self.passage_id.clone(),
            ));
        }
        if self.direction == WayBackDirection::DescendingFromStonebend
            && self.expression != WayBackExpression::AuraWay
        {
            return Err(WayBackValidationError::StairwayUsedForDescent(
                self.passage_id.clone(),
            ));
        }
        if self.expression == WayBackExpression::StairwayToHeaven
            && (self.direction != WayBackDirection::AscendingFromFlynt
                || self.origin_house != House::Flynt)
        {
            return Err(WayBackValidationError::InvalidStairwayExpression(
                self.passage_id.clone(),
            ));
        }
        if let Some(function_id) = &self.function_id {
            let winter_id = GreatFunctionKind::WayBack.stable_id();
            if function_id.as_str() != winter_id || self.canonical_year_id.is_none() {
                return Err(WayBackValidationError::InvalidFunctionAssociation(
                    self.passage_id.clone(),
                ));
            }
        }
        let mut support_ids = BTreeSet::new();
        for support in &self.support {
            support.validate()?;
            if !support_ids.insert(support.support_id.clone()) {
                return Err(WayBackValidationError::DuplicateSupport(
                    support.support_id.clone(),
                ));
            }
            if support.role == WayBackSupportRole::GlaushouseCareAndClearance
                && self.clearance_ids.is_empty()
            {
                return Err(WayBackValidationError::CareWithoutClearance(
                    self.passage_id.clone(),
                ));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WayBackRuntime {
    route: WayBackRouteDefinition,
    passages: BTreeMap<WayBackPassageId, WayBackPassage>,
}

impl WayBackRuntime {
    pub fn replay(
        route: WayBackRouteDefinition,
        passages: &[WayBackPassage],
    ) -> Result<Self, WayBackValidationError> {
        route.validate()?;
        let mut by_id = BTreeMap::new();
        for passage in passages {
            let passage = passage.canonicalized();
            passage.validate(&route)?;
            if by_id.insert(passage.passage_id.clone(), passage).is_some() {
                return Err(WayBackValidationError::DuplicatePassage);
            }
        }
        Ok(Self {
            route,
            passages: by_id,
        })
    }

    #[must_use]
    pub fn route(&self) -> &WayBackRouteDefinition {
        &self.route
    }

    #[must_use]
    pub fn passages(&self) -> &BTreeMap<WayBackPassageId, WayBackPassage> {
        &self.passages
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WayBackValidationError {
    InvalidIdentifier(String),
    InvalidRouteDefinition,
    InvalidPassage(WayBackPassageId),
    DuplicatePassage,
    StairwayUsedForDescent(WayBackPassageId),
    InvalidStairwayExpression(WayBackPassageId),
    InvalidFunctionAssociation(WayBackPassageId),
    InvalidSupport(WayBackSupportId),
    DuplicateSupport(WayBackSupportId),
    CareWithoutClearance(WayBackPassageId),
}

impl fmt::Display for WayBackValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Way Back rejected state: {self:?}")
    }
}

impl std::error::Error for WayBackValidationError {}
