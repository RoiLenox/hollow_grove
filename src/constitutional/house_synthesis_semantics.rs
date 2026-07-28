//! House-scoped Synthesis meaning above the universal recursion kernel.
//!
//! These types separate the inward operation performed during Aim from the
//! outward manifestation that only an accepted Kiss may apply. They describe
//! constitutional meaning; they do not alter recursion, compile a Recipe, or
//! choose a kernel result.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::hollow_grove_contract::House;

pub const HOUSE_SYNTHESIS_SEMANTICS_SOURCE: &str =
    "SERVICE_TOURNAMENT_ARCHIVE_AND_CANONICAL_YEAR_FIXTURE_V1.md";
pub const HOUSE_SYNTHESIS_SEMANTICS_VERSION: u16 = 1;

macro_rules! semantic_id {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, HouseSynthesisSemanticError> {
                let value = value.into();
                if value.is_empty()
                    || !value.bytes().all(|byte| {
                        byte.is_ascii_lowercase()
                            || byte.is_ascii_digit()
                            || matches!(byte, b'.' | b'-')
                    })
                {
                    return Err(HouseSynthesisSemanticError::InvalidIdentifier(value));
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

        impl TryFrom<String> for $name {
            type Error = HouseSynthesisSemanticError;

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

semantic_id!(SynthesisAttemptId);
semantic_id!(SynthesisSemanticEventId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FlyntOperation {
    Resynce,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FlyntManifestation {
    Recog,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StonebendOperation {
    Presynce,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StonebendManifestation {
    Precog,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SandmanorOperation {
    Prefog,
    Prefig,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SandmanorSourceLineage {
    Gnome,
    Elf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SandmanorJurisdiction {
    AuraFields,
    AuraBeachAndCurrentSea,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SandmanorManifestation {
    Minotaur,
    Centaur,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GlaushouseOperation {
    Precog,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GlaushouseManifestation {
    Sympiote,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HouseSynthesisPath {
    Flynt,
    Stonebend,
    SandmanorMinorian,
    SandmanorMinoan,
    Glaushouse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HouseInwardOperation {
    Flynt(FlyntOperation),
    Stonebend(StonebendOperation),
    Sandmanor(SandmanorOperation),
    Glaushouse(GlaushouseOperation),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HouseOutwardManifestation {
    Flynt(FlyntManifestation),
    Stonebend(StonebendManifestation),
    Sandmanor(SandmanorManifestation),
    Glaushouse(GlaushouseManifestation),
}

impl HouseSynthesisPath {
    #[must_use]
    pub const fn house(self) -> House {
        match self {
            Self::Flynt => House::Flynt,
            Self::Stonebend => House::Stonebend,
            Self::SandmanorMinorian | Self::SandmanorMinoan => House::Sandmanor,
            Self::Glaushouse => House::Glaushouse,
        }
    }

    #[must_use]
    pub const fn inward_operation(self) -> HouseInwardOperation {
        match self {
            Self::Flynt => HouseInwardOperation::Flynt(FlyntOperation::Resynce),
            Self::Stonebend => HouseInwardOperation::Stonebend(StonebendOperation::Presynce),
            Self::SandmanorMinorian => HouseInwardOperation::Sandmanor(SandmanorOperation::Prefog),
            Self::SandmanorMinoan => HouseInwardOperation::Sandmanor(SandmanorOperation::Prefig),
            Self::Glaushouse => HouseInwardOperation::Glaushouse(GlaushouseOperation::Precog),
        }
    }

    #[must_use]
    pub const fn outward_manifestation(self) -> HouseOutwardManifestation {
        match self {
            Self::Flynt => HouseOutwardManifestation::Flynt(FlyntManifestation::Recog),
            Self::Stonebend => HouseOutwardManifestation::Stonebend(StonebendManifestation::Precog),
            Self::SandmanorMinorian => {
                HouseOutwardManifestation::Sandmanor(SandmanorManifestation::Minotaur)
            }
            Self::SandmanorMinoan => {
                HouseOutwardManifestation::Sandmanor(SandmanorManifestation::Centaur)
            }
            Self::Glaushouse => {
                HouseOutwardManifestation::Glaushouse(GlaushouseManifestation::Sympiote)
            }
        }
    }

    #[must_use]
    pub const fn sandmanor_source_lineage(self) -> Option<SandmanorSourceLineage> {
        match self {
            Self::SandmanorMinorian => Some(SandmanorSourceLineage::Gnome),
            Self::SandmanorMinoan => Some(SandmanorSourceLineage::Elf),
            _ => None,
        }
    }

    #[must_use]
    pub const fn sandmanor_jurisdiction(self) -> Option<SandmanorJurisdiction> {
        match self {
            Self::SandmanorMinorian => Some(SandmanorJurisdiction::AuraFields),
            Self::SandmanorMinoan => Some(SandmanorJurisdiction::AuraBeachAndCurrentSea),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SemanticContactOutcome {
    Miss,
    Kiss,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PointSquaredRelationshipRecord {
    pub record_id: String,
    pub relationship_id: String,
    pub authority_id: String,
    pub location_id: String,
    pub recipe_id: String,
    pub provenance_id: String,
    pub result_id: String,
    pub evidence_references: BTreeSet<String>,
    pub manifestation: HouseOutwardManifestation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SynthesisSemanticEventKind {
    Aim {
        attempt_id: SynthesisAttemptId,
        path: HouseSynthesisPath,
        operation: HouseInwardOperation,
        recipe_id: String,
        compiler_id: String,
        script_id: String,
        aim_id: String,
        manifestation_before: Option<HouseOutwardManifestation>,
        evidence_references: BTreeSet<String>,
    },
    Fire {
        attempt_id: SynthesisAttemptId,
        fire_id: String,
    },
    Contact {
        attempt_id: SynthesisAttemptId,
        outcome: SemanticContactOutcome,
        accepted_by: String,
        evidence_references: BTreeSet<String>,
    },
    PointSquared {
        attempt_id: SynthesisAttemptId,
        record: PointSquaredRelationshipRecord,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SynthesisSemanticEvent {
    pub id: SynthesisSemanticEventId,
    pub semantic_sequence: u64,
    pub kind: SynthesisSemanticEventKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SynthesisAttemptState {
    pub path: HouseSynthesisPath,
    pub operation: HouseInwardOperation,
    pub manifestation_before: Option<HouseOutwardManifestation>,
    pub canonical_manifestation: Option<HouseOutwardManifestation>,
    pub fired: bool,
    pub contact: Option<SemanticContactOutcome>,
    pub point_squared: Option<PointSquaredRelationshipRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HouseSynthesisSemanticRuntime {
    attempts: BTreeMap<SynthesisAttemptId, SynthesisAttemptState>,
    events: BTreeMap<SynthesisSemanticEventId, SynthesisSemanticEvent>,
}

impl HouseSynthesisSemanticRuntime {
    pub fn replay(events: &[SynthesisSemanticEvent]) -> Result<Self, HouseSynthesisSemanticError> {
        let mut ordered = events.to_vec();
        ordered.sort_by_key(|event| (event.semantic_sequence, event.id.as_str().to_owned()));
        let mut runtime = Self::default();
        for event in ordered {
            runtime.apply(event)?;
        }
        Ok(runtime)
    }

    pub fn apply(
        &mut self,
        event: SynthesisSemanticEvent,
    ) -> Result<(), HouseSynthesisSemanticError> {
        if self.events.contains_key(&event.id) {
            return Err(HouseSynthesisSemanticError::DuplicateEvent(event.id));
        }
        let expected = self.events.len() as u64;
        if event.semantic_sequence != expected {
            return Err(HouseSynthesisSemanticError::UnexpectedSequence {
                expected,
                actual: event.semantic_sequence,
            });
        }
        match &event.kind {
            SynthesisSemanticEventKind::Aim {
                attempt_id,
                path,
                operation,
                recipe_id,
                compiler_id,
                script_id,
                aim_id,
                manifestation_before,
                evidence_references,
            } => {
                if self.attempts.contains_key(attempt_id) {
                    return Err(HouseSynthesisSemanticError::DuplicateAttempt(
                        attempt_id.clone(),
                    ));
                }
                if *operation != path.inward_operation()
                    || [
                        recipe_id.as_str(),
                        compiler_id.as_str(),
                        script_id.as_str(),
                        aim_id.as_str(),
                    ]
                    .into_iter()
                    .any(str::is_empty)
                    || evidence_references.is_empty()
                {
                    return Err(HouseSynthesisSemanticError::InvalidAim(attempt_id.clone()));
                }
                self.attempts.insert(
                    attempt_id.clone(),
                    SynthesisAttemptState {
                        path: *path,
                        operation: *operation,
                        manifestation_before: *manifestation_before,
                        canonical_manifestation: *manifestation_before,
                        fired: false,
                        contact: None,
                        point_squared: None,
                    },
                );
            }
            SynthesisSemanticEventKind::Fire {
                attempt_id,
                fire_id,
            } => {
                let attempt = self.attempts.get_mut(attempt_id).ok_or_else(|| {
                    HouseSynthesisSemanticError::UnknownAttempt(attempt_id.clone())
                })?;
                if attempt.fired || fire_id.is_empty() {
                    return Err(HouseSynthesisSemanticError::InvalidFire(attempt_id.clone()));
                }
                attempt.fired = true;
            }
            SynthesisSemanticEventKind::Contact {
                attempt_id,
                outcome,
                accepted_by,
                evidence_references,
            } => {
                let attempt = self.attempts.get_mut(attempt_id).ok_or_else(|| {
                    HouseSynthesisSemanticError::UnknownAttempt(attempt_id.clone())
                })?;
                if !attempt.fired
                    || attempt.contact.is_some()
                    || accepted_by.is_empty()
                    || evidence_references.is_empty()
                {
                    return Err(HouseSynthesisSemanticError::InvalidContact(
                        attempt_id.clone(),
                    ));
                }
                attempt.contact = Some(*outcome);
                if *outcome == SemanticContactOutcome::Kiss {
                    attempt.canonical_manifestation = Some(attempt.path.outward_manifestation());
                }
            }
            SynthesisSemanticEventKind::PointSquared { attempt_id, record } => {
                let attempt = self.attempts.get_mut(attempt_id).ok_or_else(|| {
                    HouseSynthesisSemanticError::UnknownAttempt(attempt_id.clone())
                })?;
                if attempt.contact != Some(SemanticContactOutcome::Kiss)
                    || attempt.point_squared.is_some()
                    || attempt.canonical_manifestation != Some(record.manifestation)
                    || [
                        record.record_id.as_str(),
                        record.relationship_id.as_str(),
                        record.authority_id.as_str(),
                        record.location_id.as_str(),
                        record.recipe_id.as_str(),
                        record.provenance_id.as_str(),
                        record.result_id.as_str(),
                    ]
                    .into_iter()
                    .any(str::is_empty)
                    || record.evidence_references.is_empty()
                {
                    return Err(HouseSynthesisSemanticError::InvalidPointSquared(
                        attempt_id.clone(),
                    ));
                }
                attempt.point_squared = Some(record.clone());
            }
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    #[must_use]
    pub fn attempts(&self) -> &BTreeMap<SynthesisAttemptId, SynthesisAttemptState> {
        &self.attempts
    }

    #[must_use]
    pub fn events(&self) -> &BTreeMap<SynthesisSemanticEventId, SynthesisSemanticEvent> {
        &self.events
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HouseSynthesisSemanticError {
    InvalidIdentifier(String),
    DuplicateEvent(SynthesisSemanticEventId),
    UnexpectedSequence { expected: u64, actual: u64 },
    DuplicateAttempt(SynthesisAttemptId),
    UnknownAttempt(SynthesisAttemptId),
    InvalidAim(SynthesisAttemptId),
    InvalidFire(SynthesisAttemptId),
    InvalidContact(SynthesisAttemptId),
    InvalidPointSquared(SynthesisAttemptId),
}

impl fmt::Display for HouseSynthesisSemanticError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "House Synthesis semantic law rejected state: {self:?}"
        )
    }
}

impl std::error::Error for HouseSynthesisSemanticError {}
