//! Hollow Grove's kernel-facing, scale-invariant four-phase grammar.
//!
//! This constitutional façade sits above the House-neutral recursion kernel.
//! Detailed lifecycle operations remain evidence-bearing substeps beneath these
//! four public phases.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::composition::ScaleKey;

use super::AuthoritativeTimestamp;

pub const GROVE_PHASE_SCHEMA_VERSION: u16 = 1;
pub const GROVE_PHASE_SEQUENCE: &str =
    "The Way Back → The Initiation → The Gathering → The Festival → The Way Back";
pub const GROVE_PHASE_DESCRIPTION: &str = "The Way Back carries the prior state forward. The Initiation gives it form. The Gathering places that form into relationship. The Festival determines what became true. What became true enters The Way Back again.";

macro_rules! cycle_id {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, GroveCycleError> {
                let value = value.into();
                if value.is_empty()
                    || !value.bytes().all(|byte| {
                        byte.is_ascii_lowercase()
                            || byte.is_ascii_digit()
                            || matches!(byte, b'.' | b'-')
                    })
                {
                    return Err(GroveCycleError::InvalidIdentifier(value));
                }
                Ok(Self(value))
            }

            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl TryFrom<String> for $name {
            type Error = GroveCycleError;

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

cycle_id!(GroveCycleId);
cycle_id!(GroveCycleSubjectId);
cycle_id!(GroveStateId);
cycle_id!(GroveCycleEvidenceId);
cycle_id!(GroveCycleAuthorityId);
cycle_id!(GroveCycleProvenanceId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum GrovePhase {
    #[serde(alias = "Return", alias = "Enter", alias = "ReturnAgain")]
    TheWayBack,
    #[serde(alias = "Incarnate", alias = "Encounter", alias = "Attempt")]
    TheInitiation,
    #[serde(alias = "Commune", alias = "Manifest")]
    TheGathering,
    #[serde(
        alias = "Confirm",
        alias = "Witness",
        alias = "Record",
        alias = "Transform"
    )]
    TheFestival,
}

impl GrovePhase {
    pub const ALL: [Self; 4] = [
        Self::TheWayBack,
        Self::TheInitiation,
        Self::TheGathering,
        Self::TheFestival,
    ];

    #[must_use]
    pub const fn next(self) -> Self {
        match self {
            Self::TheWayBack => Self::TheInitiation,
            Self::TheInitiation => Self::TheGathering,
            Self::TheGathering => Self::TheFestival,
            Self::TheFestival => Self::TheWayBack,
        }
    }

    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::TheWayBack => "The Way Back",
            Self::TheInitiation => "The Initiation",
            Self::TheGathering => "The Gathering",
            Self::TheFestival => "The Festival",
        }
    }

    #[must_use]
    pub const fn constitutional_question(self) -> &'static str {
        match self {
            Self::TheWayBack => "What has come back?",
            Self::TheInitiation => "What form will it take?",
            Self::TheGathering => "What happens when it meets the world?",
            Self::TheFestival => "What actually became true?",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum GroveCycleResolution {
    Pending,
    Accepted,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GroveCycleRecord {
    pub cycle_id: GroveCycleId,
    pub subject_id: GroveCycleSubjectId,
    pub scale: ScaleKey,
    pub current_phase: GrovePhase,
    pub phase_history: Vec<GrovePhase>,
    pub prior_state_id: GroveStateId,
    pub attempted_state_id: Option<GroveStateId>,
    pub confirmed_state_id: Option<GroveStateId>,
    pub next_way_back_state_id: Option<GroveStateId>,
    pub resolution: GroveCycleResolution,
    pub evidence_ids: BTreeSet<GroveCycleEvidenceId>,
    pub authority_ids: BTreeSet<GroveCycleAuthorityId>,
    pub provenance_ids: BTreeSet<GroveCycleProvenanceId>,
    pub opened_at: AuthoritativeTimestamp,
    pub completed_at: Option<AuthoritativeTimestamp>,
    pub rendering_may_advance_phase: bool,
}

impl GroveCycleRecord {
    pub fn validate(&self) -> Result<(), GroveCycleError> {
        if self.phase_history.is_empty()
            || self.phase_history[0] != GrovePhase::TheWayBack
            || self.phase_history.last().copied() != Some(self.current_phase)
            || self
                .phase_history
                .windows(2)
                .any(|window| window[0].next() != window[1])
            || self.evidence_ids.is_empty()
            || self.authority_ids.is_empty()
            || self.provenance_ids.is_empty()
            || self.rendering_may_advance_phase
            || self
                .completed_at
                .as_ref()
                .is_some_and(|completed_at| completed_at <= &self.opened_at)
        {
            return Err(GroveCycleError::InvalidCycle(self.cycle_id.clone()));
        }
        match self.resolution {
            GroveCycleResolution::Pending => {
                if self.current_phase == GrovePhase::TheFestival
                    || self.completed_at.is_some()
                    || self.confirmed_state_id.is_some()
                    || self.next_way_back_state_id.is_some()
                {
                    return Err(GroveCycleError::InvalidPendingCycle(self.cycle_id.clone()));
                }
            }
            GroveCycleResolution::Accepted => {
                let confirmed = self
                    .confirmed_state_id
                    .as_ref()
                    .ok_or_else(|| GroveCycleError::MissingConfirmedState(self.cycle_id.clone()))?;
                if self.phase_history != GrovePhase::ALL
                    || self.current_phase != GrovePhase::TheFestival
                    || self.completed_at.is_none()
                    || self.attempted_state_id.as_ref() != Some(confirmed)
                    || self.next_way_back_state_id.as_ref() != Some(confirmed)
                {
                    return Err(GroveCycleError::InvalidAcceptedCycle(self.cycle_id.clone()));
                }
            }
            GroveCycleResolution::Rejected => {
                if self.phase_history != GrovePhase::ALL
                    || self.current_phase != GrovePhase::TheFestival
                    || self.completed_at.is_none()
                    || self.attempted_state_id.is_none()
                    || self.confirmed_state_id.as_ref() != Some(&self.prior_state_id)
                    || self.next_way_back_state_id.as_ref() != Some(&self.prior_state_id)
                {
                    return Err(GroveCycleError::InvalidRejectedCycle(self.cycle_id.clone()));
                }
            }
        }
        Ok(())
    }

    pub fn transition_to(&mut self, next: GrovePhase) -> Result<(), GroveCycleError> {
        let expected = self.current_phase.next();
        if next != expected {
            return Err(GroveCycleError::InvalidTransition {
                cycle_id: self.cycle_id.clone(),
                from: self.current_phase,
                expected,
                attempted: next,
            });
        }
        self.current_phase = next;
        self.phase_history.push(next);
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroveCycleRuntime {
    cycles: BTreeMap<GroveCycleId, GroveCycleRecord>,
    by_subject: BTreeMap<GroveCycleSubjectId, Vec<GroveCycleId>>,
}

impl GroveCycleRuntime {
    pub fn replay(cycles: &[GroveCycleRecord]) -> Result<Self, GroveCycleError> {
        let mut ordered = cycles.to_vec();
        ordered.sort_by(|left, right| {
            left.subject_id
                .cmp(&right.subject_id)
                .then_with(|| left.opened_at.cmp(&right.opened_at))
                .then_with(|| left.cycle_id.cmp(&right.cycle_id))
        });
        let mut by_id = BTreeMap::new();
        let mut by_subject: BTreeMap<GroveCycleSubjectId, Vec<GroveCycleId>> = BTreeMap::new();
        let mut last_by_subject: BTreeMap<GroveCycleSubjectId, GroveStateId> = BTreeMap::new();
        for cycle in ordered {
            cycle.validate()?;
            if let Some(prior_confirmed) = last_by_subject.get(&cycle.subject_id)
                && prior_confirmed != &cycle.prior_state_id
            {
                return Err(GroveCycleError::BrokenContinuousCycle(
                    cycle.cycle_id.clone(),
                ));
            }
            if let Some(next) = cycle.next_way_back_state_id.clone() {
                last_by_subject.insert(cycle.subject_id.clone(), next);
            }
            if by_id
                .insert(cycle.cycle_id.clone(), cycle.clone())
                .is_some()
            {
                return Err(GroveCycleError::DuplicateCycle(cycle.cycle_id));
            }
            by_subject
                .entry(cycle.subject_id.clone())
                .or_default()
                .push(cycle.cycle_id);
        }
        Ok(Self {
            cycles: by_id,
            by_subject,
        })
    }

    #[must_use]
    pub fn cycles(&self) -> &BTreeMap<GroveCycleId, GroveCycleRecord> {
        &self.cycles
    }

    #[must_use]
    pub fn cycles_for_subject(&self, subject_id: &GroveCycleSubjectId) -> Option<&[GroveCycleId]> {
        self.by_subject.get(subject_id).map(Vec::as_slice)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LegacyGrovePhase {
    Return,
    Incarnate,
    Commune,
    Confirm,
    Enter,
    Encounter,
    Attempt,
    Manifest,
    Witness,
    Record,
    Transform,
    ReturnAgain,
}

impl LegacyGrovePhase {
    #[must_use]
    pub const fn migrate(self) -> GrovePhase {
        match self {
            Self::Return | Self::Enter | Self::ReturnAgain => GrovePhase::TheWayBack,
            Self::Incarnate | Self::Encounter | Self::Attempt => GrovePhase::TheInitiation,
            Self::Commune | Self::Manifest => GrovePhase::TheGathering,
            Self::Confirm | Self::Witness | Self::Record | Self::Transform => {
                GrovePhase::TheFestival
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LegacyGroveCycleRecord {
    pub cycle: GroveCycleRecord,
    pub legacy_phase: LegacyGrovePhase,
}

#[must_use]
pub fn migrate_legacy_grove_cycles(legacy: &[LegacyGroveCycleRecord]) -> Vec<GroveCycleRecord> {
    let mut by_id = BTreeMap::new();
    for record in legacy {
        let mut cycle = record.cycle.clone();
        cycle.current_phase = record.legacy_phase.migrate();
        by_id.entry(cycle.cycle_id.clone()).or_insert(cycle);
    }
    by_id.into_values().collect()
}

#[derive(Debug)]
pub enum GroveCycleError {
    InvalidIdentifier(String),
    InvalidCycle(GroveCycleId),
    InvalidPendingCycle(GroveCycleId),
    MissingConfirmedState(GroveCycleId),
    InvalidAcceptedCycle(GroveCycleId),
    InvalidRejectedCycle(GroveCycleId),
    InvalidTransition {
        cycle_id: GroveCycleId,
        from: GrovePhase,
        expected: GrovePhase,
        attempted: GrovePhase,
    },
    DuplicateCycle(GroveCycleId),
    BrokenContinuousCycle(GroveCycleId),
}

impl fmt::Display for GroveCycleError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Grove phase cycle rejected state: {self:?}")
    }
}

impl std::error::Error for GroveCycleError {}
