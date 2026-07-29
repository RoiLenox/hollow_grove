//! Deterministic fixed-point geometry for contextual position.
//!
//! This module owns only bounded coordinates and translations. Domain layers
//! attach constitutional names and meaning to the five fixed positions.

use std::fmt;

use serde::{Deserialize, Serialize};

pub const AXIS_MIN: i16 = -1000;
pub const AXIS_MAX: i16 = 1000;
pub const DELTA_MIN: i32 = -2000;
pub const DELTA_MAX: i32 = 2000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "i16", into = "i16")]
pub struct AxisValue(i16);

impl AxisValue {
    pub const ZERO: Self = Self(0);
    pub const MIN: Self = Self(AXIS_MIN);
    pub const MAX: Self = Self(AXIS_MAX);

    pub fn new(value: i16) -> Result<Self, CoordinateError> {
        if (AXIS_MIN..=AXIS_MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(CoordinateError::AxisOutOfBounds {
                value,
                minimum: AXIS_MIN,
                maximum: AXIS_MAX,
            })
        }
    }

    #[must_use]
    pub const fn get(self) -> i16 {
        self.0
    }
}

impl TryFrom<i16> for AxisValue {
    type Error = CoordinateError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<AxisValue> for i16 {
    fn from(value: AxisValue) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "i32", into = "i32")]
pub struct AxisDelta(i32);

impl AxisDelta {
    pub const ZERO: Self = Self(0);

    pub fn new(value: i32) -> Result<Self, CoordinateError> {
        if (DELTA_MIN..=DELTA_MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(CoordinateError::DeltaOutOfBounds {
                value,
                minimum: DELTA_MIN,
                maximum: DELTA_MAX,
            })
        }
    }

    #[must_use]
    pub const fn get(self) -> i32 {
        self.0
    }
}

impl TryFrom<i32> for AxisDelta {
    type Error = CoordinateError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<AxisDelta> for i32 {
    fn from(value: AxisDelta) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "Position3Wire")]
pub struct Position3 {
    x: AxisValue,
    y: AxisValue,
    z: AxisValue,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Position3Wire {
    x: AxisValue,
    y: AxisValue,
    z: AxisValue,
}

impl TryFrom<Position3Wire> for Position3 {
    type Error = CoordinateError;

    fn try_from(value: Position3Wire) -> Result<Self, Self::Error> {
        Self::new(value.x, value.y, value.z)
    }
}

impl Position3 {
    pub const ORIGIN: Self = Self {
        x: AxisValue::ZERO,
        y: AxisValue::ZERO,
        z: AxisValue::ZERO,
    };
    pub const POSITIVE_Z_EXTREME: Self = Self {
        x: AxisValue::ZERO,
        y: AxisValue::ZERO,
        z: AxisValue::MAX,
    };
    pub const POSITIVE_X_EXTREME: Self = Self {
        x: AxisValue::MAX,
        y: AxisValue::ZERO,
        z: AxisValue::ZERO,
    };
    pub const NEGATIVE_Z_EXTREME: Self = Self {
        x: AxisValue::ZERO,
        y: AxisValue::ZERO,
        z: AxisValue::MIN,
    };
    pub const NEGATIVE_X_EXTREME: Self = Self {
        x: AxisValue::MIN,
        y: AxisValue::ZERO,
        z: AxisValue::ZERO,
    };

    pub fn new(x: AxisValue, y: AxisValue, z: AxisValue) -> Result<Self, CoordinateError> {
        if y != AxisValue::ZERO {
            return Err(CoordinateError::ReservedYAxisNonZero { value: y.get() });
        }
        Ok(Self { x, y, z })
    }

    pub fn from_raw(x: i16, y: i16, z: i16) -> Result<Self, CoordinateError> {
        Self::new(AxisValue::new(x)?, AxisValue::new(y)?, AxisValue::new(z)?)
    }

    #[must_use]
    pub const fn x(self) -> AxisValue {
        self.x
    }

    #[must_use]
    pub const fn y(self) -> AxisValue {
        self.y
    }

    #[must_use]
    pub const fn z(self) -> AxisValue {
        self.z
    }

    pub fn translated(self, translation: Translation3) -> Result<Self, CoordinateError> {
        let next_x = i32::from(self.x().get())
            .checked_add(translation.dx().get())
            .ok_or(CoordinateError::ArithmeticOverflow)?;
        let next_y = i32::from(self.y().get())
            .checked_add(translation.dy().get())
            .ok_or(CoordinateError::ArithmeticOverflow)?;
        let next_z = i32::from(self.z().get())
            .checked_add(translation.dz().get())
            .ok_or(CoordinateError::ArithmeticOverflow)?;

        let next_x = i16::try_from(next_x).map_err(|_| CoordinateError::ArithmeticOverflow)?;
        let next_y = i16::try_from(next_y).map_err(|_| CoordinateError::ArithmeticOverflow)?;
        let next_z = i16::try_from(next_z).map_err(|_| CoordinateError::ArithmeticOverflow)?;

        Self::new(
            AxisValue::new(next_x)?,
            AxisValue::new(next_y)?,
            AxisValue::new(next_z)?,
        )
    }

    pub fn translation_to(self, destination: Position3) -> Result<Translation3, CoordinateError> {
        let dx = i32::from(destination.x().get()) - i32::from(self.x().get());
        let dy = i32::from(destination.y().get()) - i32::from(self.y().get());
        let dz = i32::from(destination.z().get()) - i32::from(self.z().get());
        Translation3::new(
            AxisDelta::new(dx)?,
            AxisDelta::new(dy)?,
            AxisDelta::new(dz)?,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "Translation3Wire")]
pub struct Translation3 {
    dx: AxisDelta,
    dy: AxisDelta,
    dz: AxisDelta,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Translation3Wire {
    dx: AxisDelta,
    dy: AxisDelta,
    dz: AxisDelta,
}

impl TryFrom<Translation3Wire> for Translation3 {
    type Error = CoordinateError;

    fn try_from(value: Translation3Wire) -> Result<Self, Self::Error> {
        Self::new(value.dx, value.dy, value.dz)
    }
}

impl Translation3 {
    pub const ZERO: Self = Self {
        dx: AxisDelta::ZERO,
        dy: AxisDelta::ZERO,
        dz: AxisDelta::ZERO,
    };

    pub fn new(dx: AxisDelta, dy: AxisDelta, dz: AxisDelta) -> Result<Self, CoordinateError> {
        if dy != AxisDelta::ZERO {
            return Err(CoordinateError::ReservedYAxisTranslation { value: dy.get() });
        }
        Ok(Self { dx, dy, dz })
    }

    pub fn from_raw(dx: i32, dy: i32, dz: i32) -> Result<Self, CoordinateError> {
        Self::new(
            AxisDelta::new(dx)?,
            AxisDelta::new(dy)?,
            AxisDelta::new(dz)?,
        )
    }

    #[must_use]
    pub const fn dx(self) -> AxisDelta {
        self.dx
    }

    #[must_use]
    pub const fn dy(self) -> AxisDelta {
        self.dy
    }

    #[must_use]
    pub const fn dz(self) -> AxisDelta {
        self.dz
    }
}

pub fn verify_translation(
    source: Position3,
    translation: Translation3,
    destination: Position3,
) -> Result<(), CoordinateError> {
    let calculated = source.translated(translation)?;
    if calculated == destination {
        Ok(())
    } else {
        Err(CoordinateError::TranslationMismatch {
            expected: calculated,
            supplied: destination,
        })
    }
}

#[must_use]
pub fn squared_distance(first: Position3, second: Position3) -> i64 {
    let dx = i64::from(first.x().get()) - i64::from(second.x().get());
    let dy = i64::from(first.y().get()) - i64::from(second.y().get());
    let dz = i64::from(first.z().get()) - i64::from(second.z().get());
    dx * dx + dy * dy + dz * dz
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OriginRelation {
    DoesNotTouch,
    TouchesOriginAxis,
    CrossesOriginAxis,
}

#[must_use]
pub const fn vertical_origin_relation(from: Position3, to: Position3) -> OriginRelation {
    let from_z = from.z().get();
    let to_z = to.z().get();
    if (from_z < 0 && to_z > 0) || (from_z > 0 && to_z < 0) {
        OriginRelation::CrossesOriginAxis
    } else if from_z == 0 || to_z == 0 {
        OriginRelation::TouchesOriginAxis
    } else {
        OriginRelation::DoesNotTouch
    }
}

#[must_use]
pub const fn crosses_vertical_origin(from: Position3, to: Position3) -> bool {
    !matches!(
        vertical_origin_relation(from, to),
        OriginRelation::DoesNotTouch
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoordinateError {
    AxisOutOfBounds {
        value: i16,
        minimum: i16,
        maximum: i16,
    },
    DeltaOutOfBounds {
        value: i32,
        minimum: i32,
        maximum: i32,
    },
    ArithmeticOverflow,
    ReservedYAxisNonZero {
        value: i16,
    },
    ReservedYAxisTranslation {
        value: i32,
    },
    TranslationMismatch {
        expected: Position3,
        supplied: Position3,
    },
    MalformedCoordinate,
}

impl fmt::Display for CoordinateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AxisOutOfBounds {
                value,
                minimum,
                maximum,
            } => write!(
                formatter,
                "axis value {value} is outside {minimum} through {maximum}"
            ),
            Self::DeltaOutOfBounds {
                value,
                minimum,
                maximum,
            } => write!(
                formatter,
                "axis delta {value} is outside {minimum} through {maximum}"
            ),
            Self::ArithmeticOverflow => formatter.write_str("coordinate arithmetic overflowed"),
            Self::ReservedYAxisNonZero { value } => {
                write!(formatter, "reserved Y axis must be zero, received {value}")
            }
            Self::ReservedYAxisTranslation { value } => {
                write!(
                    formatter,
                    "reserved Y-axis translation must be zero, received {value}"
                )
            }
            Self::TranslationMismatch { expected, supplied } => write!(
                formatter,
                "translation destination mismatch: expected {expected:?}, supplied {supplied:?}"
            ),
            Self::MalformedCoordinate => formatter.write_str("malformed coordinate"),
        }
    }
}

impl std::error::Error for CoordinateError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn absolute_boundaries_are_inclusive_and_values_outside_are_rejected() {
        assert_eq!(AxisValue::new(-1000).unwrap(), AxisValue::MIN);
        assert_eq!(AxisValue::new(1000).unwrap(), AxisValue::MAX);
        assert!(matches!(
            AxisValue::new(-1001),
            Err(CoordinateError::AxisOutOfBounds { value: -1001, .. })
        ));
        assert!(matches!(
            AxisValue::new(1001),
            Err(CoordinateError::AxisOutOfBounds { value: 1001, .. })
        ));
    }

    #[test]
    fn version_one_y_axis_is_reserved() {
        assert!(Position3::from_raw(5, 0, -7).is_ok());
        assert!(matches!(
            Position3::from_raw(0, 1, 0),
            Err(CoordinateError::ReservedYAxisNonZero { value: 1 })
        ));
        assert!(matches!(
            Position3::from_raw(0, -1, 0),
            Err(CoordinateError::ReservedYAxisNonZero { value: -1 })
        ));
    }

    #[test]
    fn delta_boundaries_cover_the_full_field() {
        assert_eq!(AxisDelta::new(-2000).unwrap().get(), -2000);
        assert_eq!(AxisDelta::new(2000).unwrap().get(), 2000);
        assert!(AxisDelta::new(-2001).is_err());
        assert!(AxisDelta::new(2001).is_err());
        assert!(matches!(
            Translation3::from_raw(0, 1, 0),
            Err(CoordinateError::ReservedYAxisTranslation { value: 1 })
        ));
    }

    #[test]
    fn full_opposite_extreme_translation_succeeds() {
        let translation = Position3::NEGATIVE_Z_EXTREME
            .translation_to(Position3::POSITIVE_Z_EXTREME)
            .unwrap();
        assert_eq!(translation.dz().get(), 2000);
        assert_eq!(
            Position3::NEGATIVE_Z_EXTREME
                .translated(translation)
                .unwrap(),
            Position3::POSITIVE_Z_EXTREME
        );
    }

    #[test]
    fn translation_rejects_axis_escape_and_destination_mismatch() {
        let leaves_field = Translation3::from_raw(1, 0, 0).unwrap();
        assert!(matches!(
            Position3::POSITIVE_X_EXTREME.translated(leaves_field),
            Err(CoordinateError::AxisOutOfBounds { value: 1001, .. })
        ));
        assert!(matches!(
            verify_translation(
                Position3::ORIGIN,
                Translation3::from_raw(5, 0, 0).unwrap(),
                Position3::from_raw(6, 0, 0).unwrap()
            ),
            Err(CoordinateError::TranslationMismatch { .. })
        ));
    }

    #[test]
    fn translation_derivation_reconstructs_destination() {
        let source = Position3::from_raw(-80, 0, -310).unwrap();
        let destination = Position3::from_raw(-120, 0, 580).unwrap();
        let translation = source.translation_to(destination).unwrap();
        assert_eq!(translation, Translation3::from_raw(-40, 0, 890).unwrap());
        assert_eq!(source.translated(translation).unwrap(), destination);
    }

    #[test]
    fn distance_is_symmetric_zero_for_self_and_equal_at_origin() {
        let value = Position3::from_raw(-250, 0, 600).unwrap();
        assert_eq!(squared_distance(value, value), 0);
        assert_eq!(
            squared_distance(value, Position3::POSITIVE_X_EXTREME),
            squared_distance(Position3::POSITIVE_X_EXTREME, value)
        );
        let distances = [
            Position3::POSITIVE_Z_EXTREME,
            Position3::POSITIVE_X_EXTREME,
            Position3::NEGATIVE_Z_EXTREME,
            Position3::NEGATIVE_X_EXTREME,
        ]
        .map(|anchor| squared_distance(Position3::ORIGIN, anchor));
        assert_eq!(distances, [1_000_000; 4]);
    }

    #[test]
    fn crossing_and_touching_are_distinct() {
        let below = Position3::from_raw(200, 0, -1).unwrap();
        let above = Position3::from_raw(200, 0, 1).unwrap();
        assert_eq!(
            vertical_origin_relation(below, above),
            OriginRelation::CrossesOriginAxis
        );
        assert_eq!(
            vertical_origin_relation(above, Position3::ORIGIN),
            OriginRelation::TouchesOriginAxis
        );
        assert!(crosses_vertical_origin(below, above));
    }
}
