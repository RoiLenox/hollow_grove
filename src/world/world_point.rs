//! Hollow Grove binding for the universal world-scale oriented Point.
//!
//! Proper names are attached only here, above the universal kernel. Rendering,
//! map rotation, and camera state are not inputs to constitutional polarity.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::composition::{
    ExpandedPointField, OrientedPoint, OrientedPointError, PhysicalPosition, RelativePolarity,
    SpatialEvidenceId, SpatialRegionId,
};

pub const WORLD_POINT_ID: &str = "point.world.hollow-grove";
pub const WORLD_FIELD_ID: &str = "field.world.hollow-grove";
pub const WORLD_POSITIVE_POLE_ID: &str = "pole.world.positive";
pub const WORLD_NEGATIVE_POLE_ID: &str = "pole.world.negative";
pub const LIGHT_AURA_REGION_ID: &str = "region.light-aura";
pub const CENTRAL_JUNCTION_REGION_ID: &str = "region.central-junction";
pub const DARK_AURA_REGION_ID: &str = "region.dark-aura";

macro_rules! world_point_id {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, WorldPointBindingError> {
                let value = value.into();
                if value.is_empty()
                    || !value.bytes().all(|byte| {
                        byte.is_ascii_lowercase()
                            || byte.is_ascii_digit()
                            || matches!(byte, b'.' | b'-')
                    })
                {
                    return Err(WorldPointBindingError::InvalidIdentifier(value));
                }
                Ok(Self(value))
            }

            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl TryFrom<String> for $name {
            type Error = WorldPointBindingError;

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

world_point_id!(WorldFieldRelationshipId);
world_point_id!(WorldFieldSubjectId);
world_point_id!(WorldMigrationDecisionId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum WorldCardinalOrientation {
    PositiveNorthTopNegativeSouthBottom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum WorldFieldRelation {
    PositiveRegionPlacement,
    CentralRegionPlacement,
    NegativeRegionPlacement,
    MovementTowardPositive,
    MovementTowardNegative,
    TransverseOrCircumferential,
    CrossingThroughCenter,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldFieldRelationship {
    pub relationship_id: WorldFieldRelationshipId,
    pub subject_id: WorldFieldSubjectId,
    pub relation: WorldFieldRelation,
    pub region_id: Option<SpatialRegionId>,
    pub evidence_ids: BTreeSet<SpatialEvidenceId>,
}

impl WorldFieldRelationship {
    pub fn validate(&self) -> Result<(), WorldPointBindingError> {
        if self.evidence_ids.is_empty() {
            return Err(WorldPointBindingError::InvalidRelationship(
                self.relationship_id.clone(),
            ));
        }
        let expected_region = match self.relation {
            WorldFieldRelation::PositiveRegionPlacement => Some(LIGHT_AURA_REGION_ID),
            WorldFieldRelation::CentralRegionPlacement => Some(CENTRAL_JUNCTION_REGION_ID),
            WorldFieldRelation::NegativeRegionPlacement => Some(DARK_AURA_REGION_ID),
            _ => None,
        };
        if let Some(expected) = expected_region
            && self.region_id.as_ref().map(SpatialRegionId::as_str) != Some(expected)
        {
            return Err(WorldPointBindingError::InvalidRelationship(
                self.relationship_id.clone(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ConstitutionalLawfulness {
    Lawful,
    Unlawful,
    Undetermined,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolarityLawfulnessObservation {
    pub polarity: RelativePolarity,
    pub lawfulness: ConstitutionalLawfulness,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HollowGroveWorldPointBinding {
    pub point: OrientedPoint,
    pub field: ExpandedPointField,
    pub cardinal_orientation: WorldCardinalOrientation,
    pub light_aura_region_id: SpatialRegionId,
    pub central_junction_region_id: SpatialRegionId,
    pub dark_aura_region_id: SpatialRegionId,
    pub lawfulness_requires_separate_determination: bool,
    pub presentation_may_change_constitutional_polarity: bool,
    pub evidence_ids: BTreeSet<SpatialEvidenceId>,
}

impl HollowGroveWorldPointBinding {
    pub fn validate(&self) -> Result<(), WorldPointBindingError> {
        self.point.validate()?;
        self.field.validate_against(&self.point)?;
        if self.point.point_id.as_str() != WORLD_POINT_ID
            || self.point.positive_pole_id.as_str() != WORLD_POSITIVE_POLE_ID
            || self.point.negative_pole_id.as_str() != WORLD_NEGATIVE_POLE_ID
            || self.field.field_id.as_str() != WORLD_FIELD_ID
            || self.light_aura_region_id.as_str() != LIGHT_AURA_REGION_ID
            || self.central_junction_region_id.as_str() != CENTRAL_JUNCTION_REGION_ID
            || self.dark_aura_region_id.as_str() != DARK_AURA_REGION_ID
            || self.field.positive_region_id != self.light_aura_region_id
            || self.field.center_region_id != self.central_junction_region_id
            || self.field.negative_region_id != self.dark_aura_region_id
            || self.cardinal_orientation
                != WorldCardinalOrientation::PositiveNorthTopNegativeSouthBottom
            || !self.lawfulness_requires_separate_determination
            || self.presentation_may_change_constitutional_polarity
            || self.evidence_ids.is_empty()
        {
            return Err(WorldPointBindingError::InvalidWorldBinding);
        }
        Ok(())
    }

    #[must_use]
    pub fn classify(&self, position: PhysicalPosition) -> RelativePolarity {
        self.point.classify(position)
    }

    #[must_use]
    pub fn observe_lawfulness(
        &self,
        polarity: RelativePolarity,
        lawfulness: ConstitutionalLawfulness,
    ) -> PolarityLawfulnessObservation {
        PolarityLawfulnessObservation {
            polarity,
            lawfulness,
        }
    }
}

#[derive(Debug)]
pub enum WorldPointBindingError {
    InvalidIdentifier(String),
    InvalidWorldBinding,
    InvalidRelationship(WorldFieldRelationshipId),
    Kernel(OrientedPointError),
}

impl fmt::Display for WorldPointBindingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "Hollow Grove world Point rejected state: {self:?}"
        )
    }
}

impl std::error::Error for WorldPointBindingError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Kernel(error) => Some(error),
            _ => None,
        }
    }
}

impl From<OrientedPointError> for WorldPointBindingError {
    fn from(value: OrientedPointError) -> Self {
        Self::Kernel(value)
    }
}
