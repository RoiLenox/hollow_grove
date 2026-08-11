//! Generic physical orientation for the universal Point primitive.
//!
//! Positive and negative are relational polarity, never morality. This module
//! intentionally contains no world, region, House, or presentation names.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use super::{CompositionRecordId, ScaleKey, StableKeyError};

macro_rules! spatial_key {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, StableKeyError> {
                let value = value.into();
                if !value.is_empty()
                    && value.bytes().all(|byte| {
                        byte.is_ascii_lowercase()
                            || byte.is_ascii_digit()
                            || matches!(byte, b'.' | b'-')
                    })
                {
                    Ok(Self(value))
                } else {
                    Err(StableKeyError::Invalid(value))
                }
            }

            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl TryFrom<String> for $name {
            type Error = StableKeyError;

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

spatial_key!(PointId);
spatial_key!(PointCenterId);
spatial_key!(PoleId);
spatial_key!(FieldId);
spatial_key!(SpatialRegionId);
spatial_key!(SpatialAuthorityId);
spatial_key!(SpatialEvidenceId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PhysicalPosition {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl PhysicalPosition {
    #[must_use]
    pub const fn origin() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AxisHandedness {
    RightHanded,
    LeftHanded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PolarityAxis {
    components: [i64; 3],
    handedness: AxisHandedness,
}

impl PolarityAxis {
    pub fn new(
        components: [i64; 3],
        handedness: AxisHandedness,
    ) -> Result<Self, OrientedPointError> {
        if components == [0, 0, 0] {
            return Err(OrientedPointError::ZeroAxis);
        }
        Ok(Self {
            components,
            handedness,
        })
    }

    #[must_use]
    pub const fn components(self) -> [i64; 3] {
        self.components
    }

    #[must_use]
    pub const fn handedness(self) -> AxisHandedness {
        self.handedness
    }

    #[must_use]
    pub const fn inverted(self) -> Self {
        Self {
            components: [
                -self.components[0],
                -self.components[1],
                -self.components[2],
            ],
            handedness: self.handedness,
        }
    }

    #[must_use]
    pub fn classify(
        self,
        center: PhysicalPosition,
        position: PhysicalPosition,
    ) -> RelativePolarity {
        let displacement = [
            i128::from(position.x) - i128::from(center.x),
            i128::from(position.y) - i128::from(center.y),
            i128::from(position.z) - i128::from(center.z),
        ];
        let projection = displacement[0] * i128::from(self.components[0])
            + displacement[1] * i128::from(self.components[1])
            + displacement[2] * i128::from(self.components[2]);
        match projection.cmp(&0) {
            std::cmp::Ordering::Greater => RelativePolarity::Positive,
            std::cmp::Ordering::Equal => RelativePolarity::Center,
            std::cmp::Ordering::Less => RelativePolarity::Negative,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PhysicalExtent(u64);

impl PhysicalExtent {
    pub fn new(value: u64) -> Result<Self, OrientedPointError> {
        if value == 0 {
            Err(OrientedPointError::ZeroExtent)
        } else {
            Ok(Self(value))
        }
    }

    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PositiveScaleFactor(u64);

impl PositiveScaleFactor {
    pub fn new(value: i64) -> Result<Self, OrientedPointError> {
        let value = u64::try_from(value).map_err(|_| OrientedPointError::NonPositiveScale)?;
        if value == 0 {
            Err(OrientedPointError::NonPositiveScale)
        } else {
            Ok(Self(value))
        }
    }

    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RelativePolarity {
    Positive,
    Center,
    Negative,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PolarityTendency {
    OutwardExpression,
    Projection,
    Exposure,
    Emergence,
    Visibility,
    Expansion,
    InwardContainment,
    Depth,
    Absorption,
    Concealment,
    Incubation,
    Contraction,
    Crossing,
    Neutrality,
    Exchange,
    PolarityTransition,
    BalancedWitness,
    ZeroAxisDisplacement,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OrientedPoint {
    pub point_id: PointId,
    pub center_id: PointCenterId,
    pub center: PhysicalPosition,
    pub orientation: PolarityAxis,
    pub positive_pole_id: PoleId,
    pub negative_pole_id: PoleId,
    pub scale: ScaleKey,
    pub extent: PhysicalExtent,
    pub evidence_ids: BTreeSet<SpatialEvidenceId>,
    pub provenance_ids: BTreeSet<CompositionRecordId>,
}

impl OrientedPoint {
    pub fn validate(&self) -> Result<(), OrientedPointError> {
        if self.positive_pole_id == self.negative_pole_id {
            return Err(OrientedPointError::DuplicatePoles);
        }
        if self.center_id.as_str() == self.positive_pole_id.as_str()
            || self.center_id.as_str() == self.negative_pole_id.as_str()
        {
            return Err(OrientedPointError::CenterIsPole);
        }
        if self.orientation.components == [0, 0, 0] {
            return Err(OrientedPointError::ZeroAxis);
        }
        if self.extent.0 == 0 {
            return Err(OrientedPointError::ZeroExtent);
        }
        if self.evidence_ids.is_empty() {
            return Err(OrientedPointError::MissingEvidence);
        }
        Ok(())
    }

    #[must_use]
    pub fn classify(&self, position: PhysicalPosition) -> RelativePolarity {
        self.orientation.classify(self.center, position)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PointScaling {
    pub scaling_id: CompositionRecordId,
    pub source_point_id: PointId,
    pub result_point_id: PointId,
    pub source_scale: ScaleKey,
    pub result_scale: ScaleKey,
    pub factor: PositiveScaleFactor,
    pub authority_ids: BTreeSet<SpatialAuthorityId>,
    pub evidence_ids: BTreeSet<SpatialEvidenceId>,
}

pub fn lawfully_scale_point(
    source: &OrientedPoint,
    scaling: &PointScaling,
) -> Result<OrientedPoint, OrientedPointError> {
    source.validate()?;
    if scaling.source_point_id != source.point_id
        || scaling.source_scale != source.scale
        || scaling.result_point_id == source.point_id
        || scaling.authority_ids.is_empty()
        || scaling.evidence_ids.is_empty()
    {
        return Err(OrientedPointError::InvalidScaling);
    }
    let extent = source
        .extent
        .get()
        .checked_mul(scaling.factor.get())
        .ok_or(OrientedPointError::ExtentOverflow)?;
    let mut result = source.clone();
    result.point_id = scaling.result_point_id.clone();
    result.scale = scaling.result_scale.clone();
    result.extent = PhysicalExtent::new(extent)?;
    result
        .evidence_ids
        .extend(scaling.evidence_ids.iter().cloned());
    result.provenance_ids.insert(scaling.scaling_id.clone());
    result.validate()?;
    Ok(result)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PointInversion {
    pub inversion_id: CompositionRecordId,
    pub source_point_id: PointId,
    pub result_point_id: PointId,
    pub authority_ids: BTreeSet<SpatialAuthorityId>,
    pub evidence_ids: BTreeSet<SpatialEvidenceId>,
}

pub fn invert_point(
    source: &OrientedPoint,
    inversion: &PointInversion,
) -> Result<OrientedPoint, OrientedPointError> {
    source.validate()?;
    if inversion.source_point_id != source.point_id
        || inversion.result_point_id == source.point_id
        || inversion.authority_ids.is_empty()
        || inversion.evidence_ids.is_empty()
    {
        return Err(OrientedPointError::InvalidInversion);
    }
    let mut result = source.clone();
    result.point_id = inversion.result_point_id.clone();
    result.orientation = source.orientation.inverted();
    result.positive_pole_id = source.negative_pole_id.clone();
    result.negative_pole_id = source.positive_pole_id.clone();
    result
        .evidence_ids
        .extend(inversion.evidence_ids.iter().cloned());
    result.provenance_ids.insert(inversion.inversion_id.clone());
    result.validate()?;
    Ok(result)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpandedPointField {
    pub field_id: FieldId,
    pub source_point_id: PointId,
    pub scale: ScaleKey,
    pub center_region_id: SpatialRegionId,
    pub positive_region_id: SpatialRegionId,
    pub negative_region_id: SpatialRegionId,
    pub axis: PolarityAxis,
    pub evidence_ids: BTreeSet<SpatialEvidenceId>,
    pub provenance_ids: BTreeSet<CompositionRecordId>,
}

impl ExpandedPointField {
    pub fn validate_against(&self, point: &OrientedPoint) -> Result<(), OrientedPointError> {
        let regions = [
            self.center_region_id.as_str(),
            self.positive_region_id.as_str(),
            self.negative_region_id.as_str(),
        ]
        .into_iter()
        .collect::<BTreeSet<_>>();
        if self.source_point_id != point.point_id
            || self.scale != point.scale
            || self.axis != point.orientation
            || regions.len() != 3
            || self.evidence_ids.is_empty()
        {
            return Err(OrientedPointError::InvalidExpandedField);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrientedPointError {
    ZeroAxis,
    ZeroExtent,
    DuplicatePoles,
    CenterIsPole,
    MissingEvidence,
    NonPositiveScale,
    ExtentOverflow,
    InvalidScaling,
    InvalidInversion,
    InvalidExpandedField,
}

impl fmt::Display for OrientedPointError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "oriented Point rejected state: {self:?}")
    }
}

impl std::error::Error for OrientedPointError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn evidence(value: &str) -> SpatialEvidenceId {
        SpatialEvidenceId::new(value).unwrap()
    }

    fn point() -> OrientedPoint {
        OrientedPoint {
            point_id: PointId::new("point.test.source").unwrap(),
            center_id: PointCenterId::new("center.test.source").unwrap(),
            center: PhysicalPosition::origin(),
            orientation: PolarityAxis::new([2, 1, 0], AxisHandedness::RightHanded).unwrap(),
            positive_pole_id: PoleId::new("pole.test.positive").unwrap(),
            negative_pole_id: PoleId::new("pole.test.negative").unwrap(),
            scale: ScaleKey::new("scale.object").unwrap(),
            extent: PhysicalExtent::new(2).unwrap(),
            evidence_ids: [evidence("evidence.test.point")].into_iter().collect(),
            provenance_ids: BTreeSet::new(),
        }
    }

    #[test]
    fn point_has_stable_center_axis_and_two_distinct_poles() {
        let point = point();
        point.validate().unwrap();
        assert_ne!(point.positive_pole_id, point.negative_pole_id);
        assert_ne!(point.center_id.as_str(), point.positive_pole_id.as_str());
        assert_eq!(point.orientation.components(), [2, 1, 0]);
    }

    #[test]
    fn projection_onto_axis_classifies_without_cardinal_convention() {
        let point = point();
        assert_eq!(
            point.classify(PhysicalPosition { x: 1, y: 0, z: 0 }),
            RelativePolarity::Positive
        );
        assert_eq!(
            point.classify(PhysicalPosition { x: 0, y: 0, z: 7 }),
            RelativePolarity::Center
        );
        assert_eq!(
            point.classify(PhysicalPosition { x: -1, y: 0, z: 0 }),
            RelativePolarity::Negative
        );
    }

    #[test]
    fn lawful_scaling_preserves_center_axis_and_poles() {
        let source = point();
        let scaling = PointScaling {
            scaling_id: CompositionRecordId::new("record.test.scale").unwrap(),
            source_point_id: source.point_id.clone(),
            result_point_id: PointId::new("point.test.scaled").unwrap(),
            source_scale: source.scale.clone(),
            result_scale: ScaleKey::new("scale.region").unwrap(),
            factor: PositiveScaleFactor::new(10).unwrap(),
            authority_ids: [SpatialAuthorityId::new("authority.test.scale").unwrap()]
                .into_iter()
                .collect(),
            evidence_ids: [evidence("evidence.test.scale")].into_iter().collect(),
        };
        let scaled = lawfully_scale_point(&source, &scaling).unwrap();
        assert_eq!(scaled.center_id, source.center_id);
        assert_eq!(scaled.center, source.center);
        assert_eq!(scaled.orientation, source.orientation);
        assert_eq!(scaled.positive_pole_id, source.positive_pole_id);
        assert_eq!(scaled.negative_pole_id, source.negative_pole_id);
        assert_eq!(scaled.extent.get(), 20);
    }

    #[test]
    fn negative_or_zero_numeric_scale_cannot_invert() {
        assert!(matches!(
            PositiveScaleFactor::new(-1),
            Err(OrientedPointError::NonPositiveScale)
        ));
        assert!(matches!(
            PositiveScaleFactor::new(0),
            Err(OrientedPointError::NonPositiveScale)
        ));
    }

    #[test]
    fn inversion_is_explicit_distinct_and_provenanced() {
        let source = point();
        let inversion = PointInversion {
            inversion_id: CompositionRecordId::new("record.test.invert").unwrap(),
            source_point_id: source.point_id.clone(),
            result_point_id: PointId::new("point.test.inverted").unwrap(),
            authority_ids: [SpatialAuthorityId::new("authority.test.invert").unwrap()]
                .into_iter()
                .collect(),
            evidence_ids: [evidence("evidence.test.invert")].into_iter().collect(),
        };
        let inverted = invert_point(&source, &inversion).unwrap();
        assert_ne!(inverted.point_id, source.point_id);
        assert_eq!(inverted.positive_pole_id, source.negative_pole_id);
        assert_eq!(inverted.negative_pole_id, source.positive_pole_id);
        assert_eq!(inverted.orientation, source.orientation.inverted());
        assert!(inverted.provenance_ids.contains(&inversion.inversion_id));
    }

    #[test]
    fn open_scale_keys_cover_required_conceptual_scales() {
        for scale in [
            "scale.microscopic",
            "scale.material",
            "scale.object",
            "scale.entity",
            "scale.room",
            "scale.district",
            "scale.region",
            "scale.world",
        ] {
            assert_eq!(ScaleKey::new(scale).unwrap().as_str(), scale);
        }
    }
}
