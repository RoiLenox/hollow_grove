use std::collections::BTreeMap;
use std::fmt;

use crate::composition::ExternalRef;

use super::{ParticipantId, UnitId, WaveId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CausalPosition(u64);

impl CausalPosition {
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Sign {
    Positive,
    Negative,
}

impl Sign {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Positive => "positive",
            Self::Negative => "negative",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuantityError {
    ZeroMagnitude,
    Overflow,
    UnitMismatch {
        expected: UnitId,
        actual: UnitId,
    },
    InsufficientCurrent {
        participant: ParticipantId,
        sign: Sign,
        available: u128,
        requested: u128,
    },
}

impl fmt::Display for QuantityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroMagnitude => {
                formatter.write_str("signed quantity magnitude must be non-zero")
            }
            Self::Overflow => formatter.write_str("exact constitutional accounting overflowed"),
            Self::UnitMismatch { expected, actual } => {
                write!(
                    formatter,
                    "quantity unit {actual} does not match {expected}"
                )
            }
            Self::InsufficientCurrent {
                participant,
                sign,
                available,
                requested,
            } => write!(
                formatter,
                "participant {participant} has {available} {} Current but {requested} was requested",
                sign.as_str()
            ),
        }
    }
}

impl std::error::Error for QuantityError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedQuantity {
    pub sign: Sign,
    pub magnitude: u128,
    pub unit: UnitId,
}

impl SignedQuantity {
    pub fn new(sign: Sign, magnitude: u128, unit: UnitId) -> Result<Self, QuantityError> {
        if magnitude == 0 {
            return Err(QuantityError::ZeroMagnitude);
        }
        Ok(Self {
            sign,
            magnitude,
            unit,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedTotals {
    pub positive: u128,
    pub negative: u128,
    pub unit: UnitId,
}

impl SignedTotals {
    #[must_use]
    pub fn zero(unit: UnitId) -> Self {
        Self {
            positive: 0,
            negative: 0,
            unit,
        }
    }

    pub fn add(&mut self, quantity: &SignedQuantity) -> Result<(), QuantityError> {
        if quantity.unit != self.unit {
            return Err(QuantityError::UnitMismatch {
                expected: self.unit.clone(),
                actual: quantity.unit.clone(),
            });
        }
        let target = match quantity.sign {
            Sign::Positive => &mut self.positive,
            Sign::Negative => &mut self.negative,
        };
        *target = target
            .checked_add(quantity.magnitude)
            .ok_or(QuantityError::Overflow)?;
        Ok(())
    }

    #[must_use]
    pub fn net(&self) -> NetExcess {
        match self.positive.cmp(&self.negative) {
            std::cmp::Ordering::Greater => NetExcess {
                sign: Some(Sign::Positive),
                magnitude: self.positive - self.negative,
                unit: self.unit.clone(),
            },
            std::cmp::Ordering::Less => NetExcess {
                sign: Some(Sign::Negative),
                magnitude: self.negative - self.positive,
                unit: self.unit.clone(),
            },
            std::cmp::Ordering::Equal => NetExcess {
                sign: None,
                magnitude: 0,
                unit: self.unit.clone(),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetExcess {
    pub sign: Option<Sign>,
    pub magnitude: u128,
    pub unit: UnitId,
}

impl NetExcess {
    #[must_use]
    pub const fn is_zero(&self) -> bool {
        self.magnitude == 0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BondTerm {
    Finite { end: CausalPosition },
    Perpetual,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceRef(pub ExternalRef);

impl EvidenceRef {
    pub fn new(
        namespace: impl Into<String>,
        key: impl Into<String>,
    ) -> Result<Self, crate::composition::ExternalRefError> {
        ExternalRef::new(namespace, key).map(Self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaveRecord {
    pub id: WaveId,
    pub origin: EvidenceRef,
    pub causal_position: CausalPosition,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CurrentAccount {
    Participant(ParticipantId),
    External(String),
    Sink(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurrentOperation {
    Enter,
    Leave,
    Transfer,
    Split,
    Merge,
    Branch,
    Reverse,
    Circulate,
    Consume,
    Decay,
    Expire,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrentEdge {
    pub source: CurrentAccount,
    pub destination: CurrentAccount,
    pub quantity: SignedQuantity,
}

pub(crate) type HoldingKey = (ParticipantId, Sign);
pub(crate) type Holdings = BTreeMap<HoldingKey, u128>;
