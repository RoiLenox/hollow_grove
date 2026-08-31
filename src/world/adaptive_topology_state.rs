//! Persistent world-state authority for the adaptive Hollow Grove topology.
//!
//! Geometry remains owned by Spatial Topology V2 and the semantic kernel.
//! This layer decides which pose is currently publishable and requires both
//! collision and navigation rebuild evidence before a moving pose commits.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::composition::{
    NORMALIZED_UNIT, NormalizedCoordinate, NormalizedPoint, SpatialInheritanceError,
};

use super::adaptive_midpoint_hinges::{
    AdaptiveMidpointHingeError, CanonicalMidpointHinge, canonical_adaptive_midpoint_hinges,
};
use super::spatial_inheritance::HollowGroveSpatialScope;
use super::spatial_topology_v2::{
    SpatialTopologyV2Error, TopologyV2RouteId, canonical_hollow_grove_spatial_topology_v2,
    topology_v2_route_id,
};

pub const ADAPTIVE_TOPOLOGY_STATE_SCHEMA_VERSION: u16 = 1;
pub const ADAPTIVE_TOPOLOGY_STATE_RULE_SET_ID: &str =
    "ruleset.hollow-grove.adaptive-topology-state-v1";
pub const ADAPTIVE_TOPOLOGY_STATE_ARTIFACT_PATH: &str =
    "artifacts/hollow_grove_adaptive_topology_state_v1.json";
pub const MAX_ADAPTIVE_TRANSITION_TICKS: u32 = 10_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdaptiveTopologyTarget {
    Circular,
    Rectangular,
}

impl AdaptiveTopologyTarget {
    #[must_use]
    pub const fn rectangularity(self) -> NormalizedCoordinate {
        match self {
            Self::Circular => NormalizedCoordinate::zero(),
            Self::Rectangular => NormalizedCoordinate::one(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdaptiveTopologyPhase {
    Circular,
    TransitioningToCircular,
    TransitioningToRectangle,
    Rectangular,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdaptiveTopologyEasing {
    LinearFixedPointV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdaptiveTopologyMovementPolicy {
    PreserveRouteIdentityAndProgress,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdaptiveTopologyRebuildPolicy {
    CollisionAndNavigationBeforeCommit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdaptiveTopologyTriggerSource {
    GameplayEvent,
    WorldAuthority,
    Developer,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdaptiveTopologyTrigger {
    pub trigger_id: String,
    pub source: AdaptiveTopologyTriggerSource,
}

impl AdaptiveTopologyTrigger {
    pub fn new(
        trigger_id: impl Into<String>,
        source: AdaptiveTopologyTriggerSource,
    ) -> Result<Self, AdaptiveTopologyStateError> {
        let trigger = Self {
            trigger_id: trigger_id.into(),
            source,
        };
        validate_identifier(&trigger.trigger_id, "trigger")?;
        Ok(trigger)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdaptiveTopologyTransitionRequest {
    pub transition_id: String,
    pub trigger: AdaptiveTopologyTrigger,
    pub target: AdaptiveTopologyTarget,
    pub duration_ticks: u32,
    pub easing: AdaptiveTopologyEasing,
}

impl AdaptiveTopologyTransitionRequest {
    pub fn linear(
        transition_id: impl Into<String>,
        trigger: AdaptiveTopologyTrigger,
        target: AdaptiveTopologyTarget,
        duration_ticks: u32,
    ) -> Result<Self, AdaptiveTopologyStateError> {
        let request = Self {
            transition_id: transition_id.into(),
            trigger,
            target,
            duration_ticks,
            easing: AdaptiveTopologyEasing::LinearFixedPointV1,
        };
        request.validate()?;
        Ok(request)
    }

    fn validate(&self) -> Result<(), AdaptiveTopologyStateError> {
        validate_identifier(&self.transition_id, "transition")?;
        validate_identifier(&self.trigger.trigger_id, "trigger")?;
        if self.duration_ticks == 0 || self.duration_ticks > MAX_ADAPTIVE_TRANSITION_TICKS {
            return Err(AdaptiveTopologyStateError::InvalidDuration(
                self.duration_ticks,
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActiveAdaptiveTopologyTransition {
    pub transition_id: String,
    pub trigger: AdaptiveTopologyTrigger,
    pub start_rectangularity: NormalizedCoordinate,
    pub target: AdaptiveTopologyTarget,
    pub duration_ticks: u32,
    pub committed_ticks: u32,
    pub easing: AdaptiveTopologyEasing,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdaptiveTopologyCommitRecord {
    pub transition_id: String,
    pub trigger_id: String,
    pub revision: u64,
    pub rectangularity: NormalizedCoordinate,
    pub collision_snapshot_id: String,
    pub navigation_snapshot_id: String,
    pub transition_completed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdaptiveTopologyStateV1 {
    schema_version: u16,
    rule_set: String,
    revision: u64,
    rectangularity: NormalizedCoordinate,
    movement_policy: AdaptiveTopologyMovementPolicy,
    rebuild_policy: AdaptiveTopologyRebuildPolicy,
    active_transition: Option<ActiveAdaptiveTopologyTransition>,
    completed_transition_ids: BTreeSet<String>,
    last_commit: Option<AdaptiveTopologyCommitRecord>,
}

impl AdaptiveTopologyStateV1 {
    #[must_use]
    pub fn circular() -> Self {
        Self {
            schema_version: ADAPTIVE_TOPOLOGY_STATE_SCHEMA_VERSION,
            rule_set: ADAPTIVE_TOPOLOGY_STATE_RULE_SET_ID.to_owned(),
            revision: 0,
            rectangularity: NormalizedCoordinate::zero(),
            movement_policy: AdaptiveTopologyMovementPolicy::PreserveRouteIdentityAndProgress,
            rebuild_policy: AdaptiveTopologyRebuildPolicy::CollisionAndNavigationBeforeCommit,
            active_transition: None,
            completed_transition_ids: BTreeSet::new(),
            last_commit: None,
        }
    }

    #[must_use]
    pub fn rectangular() -> Self {
        let mut state = Self::circular();
        state.rectangularity = NormalizedCoordinate::one();
        state
    }

    #[must_use]
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    #[must_use]
    pub fn rule_set(&self) -> &str {
        &self.rule_set
    }

    #[must_use]
    pub const fn revision(&self) -> u64 {
        self.revision
    }

    #[must_use]
    pub const fn rectangularity(&self) -> NormalizedCoordinate {
        self.rectangularity
    }

    #[must_use]
    pub const fn movement_policy(&self) -> AdaptiveTopologyMovementPolicy {
        self.movement_policy
    }

    #[must_use]
    pub const fn rebuild_policy(&self) -> AdaptiveTopologyRebuildPolicy {
        self.rebuild_policy
    }

    #[must_use]
    pub fn active_transition(&self) -> Option<&ActiveAdaptiveTopologyTransition> {
        self.active_transition.as_ref()
    }

    #[must_use]
    pub fn last_commit(&self) -> Option<&AdaptiveTopologyCommitRecord> {
        self.last_commit.as_ref()
    }

    #[must_use]
    pub fn has_completed_transition(&self, transition_id: &str) -> bool {
        self.completed_transition_ids.contains(transition_id)
    }

    #[must_use]
    pub fn phase(&self) -> AdaptiveTopologyPhase {
        match self
            .active_transition
            .as_ref()
            .map(|transition| transition.target)
        {
            Some(AdaptiveTopologyTarget::Circular) => {
                AdaptiveTopologyPhase::TransitioningToCircular
            }
            Some(AdaptiveTopologyTarget::Rectangular) => {
                AdaptiveTopologyPhase::TransitioningToRectangle
            }
            None if self.rectangularity == NormalizedCoordinate::zero() => {
                AdaptiveTopologyPhase::Circular
            }
            None => AdaptiveTopologyPhase::Rectangular,
        }
    }

    pub fn request_transition(
        &mut self,
        request: AdaptiveTopologyTransitionRequest,
    ) -> Result<(), AdaptiveTopologyStateError> {
        self.validate()?;
        request.validate()?;
        if let Some(active) = &self.active_transition {
            return Err(AdaptiveTopologyStateError::TransitionAlreadyActive(
                active.transition_id.clone(),
            ));
        }
        if self
            .completed_transition_ids
            .contains(&request.transition_id)
        {
            return Err(AdaptiveTopologyStateError::TransitionIdAlreadyApplied(
                request.transition_id,
            ));
        }
        if self.rectangularity == request.target.rectangularity() {
            return Err(AdaptiveTopologyStateError::AlreadyAtTarget(request.target));
        }

        let next_revision = self
            .revision
            .checked_add(1)
            .ok_or(AdaptiveTopologyStateError::RevisionOverflow)?;
        let mut candidate = self.clone();
        candidate.revision = next_revision;
        candidate.active_transition = Some(ActiveAdaptiveTopologyTransition {
            transition_id: request.transition_id,
            trigger: request.trigger,
            start_rectangularity: self.rectangularity,
            target: request.target,
            duration_ticks: request.duration_ticks,
            committed_ticks: 0,
            easing: request.easing,
        });
        candidate.validate()?;
        *self = candidate;
        Ok(())
    }

    pub(super) fn record_already_satisfied_transition(
        &mut self,
        request: AdaptiveTopologyTransitionRequest,
    ) -> Result<(), AdaptiveTopologyStateError> {
        self.validate()?;
        request.validate()?;
        if let Some(active) = &self.active_transition {
            return Err(AdaptiveTopologyStateError::TransitionAlreadyActive(
                active.transition_id.clone(),
            ));
        }
        if self
            .completed_transition_ids
            .contains(&request.transition_id)
        {
            return Err(AdaptiveTopologyStateError::TransitionIdAlreadyApplied(
                request.transition_id,
            ));
        }
        if self.rectangularity != request.target.rectangularity() {
            return Err(AdaptiveTopologyStateError::TargetNotSatisfied(
                request.target,
            ));
        }

        let mut candidate = self.clone();
        candidate.revision = self
            .revision
            .checked_add(1)
            .ok_or(AdaptiveTopologyStateError::RevisionOverflow)?;
        candidate
            .completed_transition_ids
            .insert(request.transition_id);
        candidate.validate()?;
        *self = candidate;
        Ok(())
    }

    pub fn prepare_next_step(
        &self,
    ) -> Result<Option<PreparedAdaptiveTopologyStep>, AdaptiveTopologyStateError> {
        self.validate()?;
        let Some(transition) = &self.active_transition else {
            return Ok(None);
        };
        let tick = transition.committed_ticks + 1;
        let next_revision = self
            .revision
            .checked_add(1)
            .ok_or(AdaptiveTopologyStateError::RevisionOverflow)?;
        let rectangularity = transition_rectangularity(transition, tick)?;
        Ok(Some(PreparedAdaptiveTopologyStep {
            transition_id: transition.transition_id.clone(),
            trigger_id: transition.trigger.trigger_id.clone(),
            base_revision: self.revision,
            next_revision,
            tick,
            duration_ticks: transition.duration_ticks,
            rectangularity,
            target: transition.target,
            movement_policy: self.movement_policy,
            rebuild_policy: self.rebuild_policy,
            transition_completed: tick == transition.duration_ticks,
        }))
    }

    pub fn commit_prepared_step(
        &mut self,
        prepared: &PreparedAdaptiveTopologyStep,
        evidence: AdaptiveTopologyRebuildEvidence,
    ) -> Result<AdaptiveTopologyCommitRecord, AdaptiveTopologyStateError> {
        let expected = self
            .prepare_next_step()?
            .ok_or(AdaptiveTopologyStateError::NoTransitionActive)?;
        if &expected != prepared {
            return Err(AdaptiveTopologyStateError::PreparedStepMismatch);
        }
        evidence.validate_for(prepared)?;

        let mut candidate = self.clone();
        candidate.revision = prepared.next_revision;
        candidate.rectangularity = prepared.rectangularity;
        let active = candidate
            .active_transition
            .as_mut()
            .ok_or(AdaptiveTopologyStateError::NoTransitionActive)?;
        active.committed_ticks = prepared.tick;
        if prepared.transition_completed {
            let completed = candidate
                .active_transition
                .take()
                .ok_or(AdaptiveTopologyStateError::NoTransitionActive)?;
            candidate
                .completed_transition_ids
                .insert(completed.transition_id);
        }
        let record = AdaptiveTopologyCommitRecord {
            transition_id: prepared.transition_id.clone(),
            trigger_id: prepared.trigger_id.clone(),
            revision: prepared.next_revision,
            rectangularity: prepared.rectangularity,
            collision_snapshot_id: evidence.collision_snapshot_id,
            navigation_snapshot_id: evidence.navigation_snapshot_id,
            transition_completed: prepared.transition_completed,
        };
        candidate.last_commit = Some(record.clone());
        candidate.validate()?;
        *self = candidate;
        Ok(record)
    }

    pub fn validate(&self) -> Result<(), AdaptiveTopologyStateError> {
        if self.schema_version != ADAPTIVE_TOPOLOGY_STATE_SCHEMA_VERSION {
            return Err(AdaptiveTopologyStateError::UnsupportedSchema {
                expected: ADAPTIVE_TOPOLOGY_STATE_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.rule_set != ADAPTIVE_TOPOLOGY_STATE_RULE_SET_ID {
            return Err(AdaptiveTopologyStateError::RuleSetMismatch);
        }
        if self.movement_policy != AdaptiveTopologyMovementPolicy::PreserveRouteIdentityAndProgress
            || self.rebuild_policy
                != AdaptiveTopologyRebuildPolicy::CollisionAndNavigationBeforeCommit
        {
            return Err(AdaptiveTopologyStateError::PolicyMismatch);
        }
        for transition_id in &self.completed_transition_ids {
            validate_identifier(transition_id, "completed transition")?;
        }
        if let Some(transition) = &self.active_transition {
            validate_identifier(&transition.transition_id, "active transition")?;
            validate_identifier(&transition.trigger.trigger_id, "trigger")?;
            if self
                .completed_transition_ids
                .contains(&transition.transition_id)
            {
                return Err(AdaptiveTopologyStateError::TransitionIdAlreadyApplied(
                    transition.transition_id.clone(),
                ));
            }
            if transition.duration_ticks == 0
                || transition.duration_ticks > MAX_ADAPTIVE_TRANSITION_TICKS
                || transition.committed_ticks >= transition.duration_ticks
            {
                return Err(AdaptiveTopologyStateError::InvalidTransitionProgress);
            }
            if transition.start_rectangularity == transition.target.rectangularity() {
                return Err(AdaptiveTopologyStateError::AlreadyAtTarget(
                    transition.target,
                ));
            }
            if self.rectangularity
                != transition_rectangularity(transition, transition.committed_ticks)?
            {
                return Err(AdaptiveTopologyStateError::RectangularityDrift);
            }
        } else if self.rectangularity != NormalizedCoordinate::zero()
            && self.rectangularity != NormalizedCoordinate::one()
        {
            return Err(AdaptiveTopologyStateError::UnownedIntermediatePose);
        }
        if let Some(commit) = &self.last_commit {
            validate_identifier(&commit.transition_id, "committed transition")?;
            validate_identifier(&commit.trigger_id, "committed trigger")?;
            validate_identifier(&commit.collision_snapshot_id, "collision snapshot")?;
            validate_identifier(&commit.navigation_snapshot_id, "navigation snapshot")?;
            if commit.revision > self.revision {
                return Err(AdaptiveTopologyStateError::CommitRevisionAhead);
            }
        }
        Ok(())
    }

    pub fn project_traveler(
        &self,
        attachment: &AdaptiveRouteAttachment,
    ) -> Result<AdaptiveTravelerProjection, AdaptiveTopologyStateError> {
        self.validate()?;
        attachment.validate()?;
        let position = project_adaptive_route_position(
            attachment.route,
            attachment.route_progress,
            self.rectangularity,
        )?;
        Ok(AdaptiveTravelerProjection {
            traveler_id: attachment.traveler_id.clone(),
            route: attachment.route,
            route_progress: attachment.route_progress,
            rectangularity: self.rectangularity,
            position,
        })
    }
}

pub fn project_adaptive_route_position(
    route: TopologyV2RouteId,
    route_progress: NormalizedCoordinate,
    rectangularity: NormalizedCoordinate,
) -> Result<NormalizedPoint, AdaptiveTopologyStateError> {
    let map = canonical_hollow_grove_spatial_topology_v2()?;
    if route == TopologyV2RouteId::AuraRidge {
        return Ok(map
            .template()
            .point_on_path(&topology_v2_route_id(route)?, route_progress)?);
    }

    let hinge_id = CanonicalMidpointHinge::ALL
        .into_iter()
        .find(|hinge| hinge.routes().contains(&route))
        .ok_or(AdaptiveTopologyStateError::RouteIsNotAdaptive(route))?;
    let structure = canonical_adaptive_midpoint_hinges()?;
    let resolved = structure.resolve(
        &map,
        HollowGroveSpatialScope::World,
        hinge_id,
        rectangularity,
    )?;
    Ok(resolved
        .local
        .slots
        .iter()
        .find(|slot| slot.path_id.as_str() == route.stable_id())
        .ok_or(AdaptiveTopologyStateError::RouteIsNotAdaptive(route))?
        .point_at_route_progress(route_progress))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparedAdaptiveTopologyStep {
    pub transition_id: String,
    pub trigger_id: String,
    pub base_revision: u64,
    pub next_revision: u64,
    pub tick: u32,
    pub duration_ticks: u32,
    pub rectangularity: NormalizedCoordinate,
    pub target: AdaptiveTopologyTarget,
    pub movement_policy: AdaptiveTopologyMovementPolicy,
    pub rebuild_policy: AdaptiveTopologyRebuildPolicy,
    pub transition_completed: bool,
}

impl PreparedAdaptiveTopologyStep {
    pub fn acknowledge_rebuilds(
        &self,
        collision_snapshot_id: impl Into<String>,
        navigation_snapshot_id: impl Into<String>,
    ) -> Result<AdaptiveTopologyRebuildEvidence, AdaptiveTopologyStateError> {
        let evidence = AdaptiveTopologyRebuildEvidence {
            transition_id: self.transition_id.clone(),
            base_revision: self.base_revision,
            next_revision: self.next_revision,
            rectangularity: self.rectangularity,
            collision_snapshot_id: collision_snapshot_id.into(),
            navigation_snapshot_id: navigation_snapshot_id.into(),
        };
        evidence.validate_for(self)?;
        Ok(evidence)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdaptiveTopologyRebuildEvidence {
    transition_id: String,
    base_revision: u64,
    next_revision: u64,
    rectangularity: NormalizedCoordinate,
    collision_snapshot_id: String,
    navigation_snapshot_id: String,
}

impl AdaptiveTopologyRebuildEvidence {
    fn validate_for(
        &self,
        prepared: &PreparedAdaptiveTopologyStep,
    ) -> Result<(), AdaptiveTopologyStateError> {
        validate_identifier(&self.collision_snapshot_id, "collision snapshot")?;
        validate_identifier(&self.navigation_snapshot_id, "navigation snapshot")?;
        if self.transition_id != prepared.transition_id
            || self.base_revision != prepared.base_revision
            || self.next_revision != prepared.next_revision
            || self.rectangularity != prepared.rectangularity
        {
            return Err(AdaptiveTopologyStateError::RebuildEvidenceMismatch);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdaptiveRouteAttachment {
    pub traveler_id: String,
    pub route: TopologyV2RouteId,
    pub route_progress: NormalizedCoordinate,
}

impl AdaptiveRouteAttachment {
    pub fn new(
        traveler_id: impl Into<String>,
        route: TopologyV2RouteId,
        route_progress: NormalizedCoordinate,
    ) -> Result<Self, AdaptiveTopologyStateError> {
        let attachment = Self {
            traveler_id: traveler_id.into(),
            route,
            route_progress,
        };
        attachment.validate()?;
        Ok(attachment)
    }

    fn validate(&self) -> Result<(), AdaptiveTopologyStateError> {
        validate_identifier(&self.traveler_id, "traveler")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdaptiveTravelerProjection {
    pub traveler_id: String,
    pub route: TopologyV2RouteId,
    pub route_progress: NormalizedCoordinate,
    pub rectangularity: NormalizedCoordinate,
    pub position: NormalizedPoint,
}

pub fn encode_adaptive_topology_state(
    state: &AdaptiveTopologyStateV1,
) -> Result<Vec<u8>, AdaptiveTopologyStateArchiveError> {
    state.validate()?;
    Ok(serde_json::to_vec(state)?)
}

pub fn encode_adaptive_topology_state_pretty(
    state: &AdaptiveTopologyStateV1,
) -> Result<String, AdaptiveTopologyStateArchiveError> {
    state.validate()?;
    let mut json = serde_json::to_string_pretty(state)?;
    json.push('\n');
    Ok(json)
}

pub fn decode_adaptive_topology_state(
    bytes: &[u8],
) -> Result<AdaptiveTopologyStateV1, AdaptiveTopologyStateArchiveError> {
    let state: AdaptiveTopologyStateV1 = serde_json::from_slice(bytes)?;
    state.validate()?;
    Ok(state)
}

pub fn adaptive_topology_state_digest(
    state: &AdaptiveTopologyStateV1,
) -> Result<String, AdaptiveTopologyStateArchiveError> {
    let bytes = encode_adaptive_topology_state(state)?;
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in bytes {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    Ok(format!("fnv1a64-adaptive-topology-state-v1:{hash:016x}"))
}

fn transition_rectangularity(
    transition: &ActiveAdaptiveTopologyTransition,
    tick: u32,
) -> Result<NormalizedCoordinate, AdaptiveTopologyStateError> {
    if tick > transition.duration_ticks {
        return Err(AdaptiveTopologyStateError::InvalidTransitionProgress);
    }
    let progress =
        NormalizedCoordinate::from_ratio(u64::from(tick), u64::from(transition.duration_ticks))?;
    Ok(lerp_coordinate(
        transition.start_rectangularity,
        transition.target.rectangularity(),
        progress,
    ))
}

fn lerp_coordinate(
    from: NormalizedCoordinate,
    to: NormalizedCoordinate,
    progress: NormalizedCoordinate,
) -> NormalizedCoordinate {
    let progress = u128::from(progress.parts());
    let complement = u128::from(NORMALIZED_UNIT) - progress;
    let parts = (u128::from(from.parts()) * complement
        + u128::from(to.parts()) * progress
        + u128::from(NORMALIZED_UNIT / 2))
        / u128::from(NORMALIZED_UNIT);
    NormalizedCoordinate::new(parts as u64).expect("interpolation remains normalized")
}

fn validate_identifier(value: &str, kind: &'static str) -> Result<(), AdaptiveTopologyStateError> {
    if value.is_empty()
        || value.len() > 160
        || !value.bytes().all(|byte| {
            byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-' | b'_' | b':' | b'/')
        })
    {
        return Err(AdaptiveTopologyStateError::InvalidIdentifier {
            kind,
            value: value.to_owned(),
        });
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdaptiveTopologyStateError {
    Kernel(SpatialInheritanceError),
    Hinge(String),
    Topology(String),
    UnsupportedSchema { expected: u16, actual: u16 },
    RuleSetMismatch,
    PolicyMismatch,
    InvalidIdentifier { kind: &'static str, value: String },
    InvalidDuration(u32),
    InvalidTransitionProgress,
    TransitionAlreadyActive(String),
    TransitionIdAlreadyApplied(String),
    AlreadyAtTarget(AdaptiveTopologyTarget),
    TargetNotSatisfied(AdaptiveTopologyTarget),
    NoTransitionActive,
    RevisionOverflow,
    PreparedStepMismatch,
    RebuildEvidenceMismatch,
    RectangularityDrift,
    UnownedIntermediatePose,
    CommitRevisionAhead,
    RouteIsNotAdaptive(TopologyV2RouteId),
}

impl From<SpatialInheritanceError> for AdaptiveTopologyStateError {
    fn from(value: SpatialInheritanceError) -> Self {
        Self::Kernel(value)
    }
}

impl From<AdaptiveMidpointHingeError> for AdaptiveTopologyStateError {
    fn from(value: AdaptiveMidpointHingeError) -> Self {
        Self::Hinge(value.to_string())
    }
}

impl From<SpatialTopologyV2Error> for AdaptiveTopologyStateError {
    fn from(value: SpatialTopologyV2Error) -> Self {
        Self::Topology(value.to_string())
    }
}

impl fmt::Display for AdaptiveTopologyStateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Kernel(error) => error.fmt(formatter),
            Self::Hinge(error) => write!(formatter, "adaptive hinge projection failed: {error}"),
            Self::Topology(error) => write!(formatter, "spatial topology failed: {error}"),
            Self::UnsupportedSchema { expected, actual } => write!(
                formatter,
                "unsupported adaptive topology state schema {actual}; expected {expected}"
            ),
            Self::RuleSetMismatch => formatter.write_str("adaptive topology rule-set mismatch"),
            Self::PolicyMismatch => formatter.write_str("adaptive topology policy mismatch"),
            Self::InvalidIdentifier { kind, value } => {
                write!(formatter, "invalid {kind} identifier: {value}")
            }
            Self::InvalidDuration(duration) => {
                write!(
                    formatter,
                    "invalid adaptive transition duration: {duration}"
                )
            }
            Self::InvalidTransitionProgress => {
                formatter.write_str("adaptive transition progress is invalid")
            }
            Self::TransitionAlreadyActive(id) => {
                write!(formatter, "adaptive transition is already active: {id}")
            }
            Self::TransitionIdAlreadyApplied(id) => {
                write!(
                    formatter,
                    "adaptive transition id was already applied: {id}"
                )
            }
            Self::AlreadyAtTarget(target) => {
                write!(formatter, "adaptive topology is already at {target:?}")
            }
            Self::TargetNotSatisfied(target) => {
                write!(
                    formatter,
                    "adaptive topology has not yet reached {target:?}"
                )
            }
            Self::NoTransitionActive => {
                formatter.write_str("no adaptive topology transition is active")
            }
            Self::RevisionOverflow => formatter.write_str("adaptive topology revision overflowed"),
            Self::PreparedStepMismatch => {
                formatter.write_str("prepared adaptive topology step is stale or altered")
            }
            Self::RebuildEvidenceMismatch => formatter.write_str(
                "collision/navigation evidence does not match the prepared topology step",
            ),
            Self::RectangularityDrift => {
                formatter.write_str("adaptive topology rectangularity drifted from its transition")
            }
            Self::UnownedIntermediatePose => formatter
                .write_str("an intermediate adaptive pose must retain its active transition"),
            Self::CommitRevisionAhead => {
                formatter.write_str("adaptive topology commit is ahead of state revision")
            }
            Self::RouteIsNotAdaptive(route) => {
                write!(
                    formatter,
                    "route {route:?} is not assigned to an adaptive hinge"
                )
            }
        }
    }
}

impl std::error::Error for AdaptiveTopologyStateError {}

#[derive(Debug)]
pub enum AdaptiveTopologyStateArchiveError {
    Json(serde_json::Error),
    State(AdaptiveTopologyStateError),
}

impl From<serde_json::Error> for AdaptiveTopologyStateArchiveError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

impl From<AdaptiveTopologyStateError> for AdaptiveTopologyStateArchiveError {
    fn from(value: AdaptiveTopologyStateError) -> Self {
        Self::State(value)
    }
}

impl fmt::Display for AdaptiveTopologyStateArchiveError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(error) => write!(formatter, "adaptive topology state JSON error: {error}"),
            Self::State(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for AdaptiveTopologyStateArchiveError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn request(
        id: &str,
        target: AdaptiveTopologyTarget,
        duration_ticks: u32,
    ) -> AdaptiveTopologyTransitionRequest {
        AdaptiveTopologyTransitionRequest::linear(
            id,
            AdaptiveTopologyTrigger::new(
                "trigger.test.world-event",
                AdaptiveTopologyTriggerSource::GameplayEvent,
            )
            .unwrap(),
            target,
            duration_ticks,
        )
        .unwrap()
    }

    fn commit_next(state: &mut AdaptiveTopologyStateV1) -> AdaptiveTopologyCommitRecord {
        let prepared = state.prepare_next_step().unwrap().unwrap();
        let evidence = prepared
            .acknowledge_rebuilds(
                format!("collision.snapshot.r{}", prepared.next_revision),
                format!("navigation.snapshot.r{}", prepared.next_revision),
            )
            .unwrap();
        state.commit_prepared_step(&prepared, evidence).unwrap()
    }

    #[test]
    fn transition_requires_rebuilds_and_commits_exact_fixed_point_steps() {
        let mut state = AdaptiveTopologyStateV1::circular();
        state
            .request_transition(request(
                "transition.test.rectangle",
                AdaptiveTopologyTarget::Rectangular,
                4,
            ))
            .unwrap();
        assert_eq!(
            state.phase(),
            AdaptiveTopologyPhase::TransitioningToRectangle
        );
        assert_eq!(state.revision(), 1);

        for expected in [250_000_000, 500_000_000, 750_000_000, 1_000_000_000] {
            let record = commit_next(&mut state);
            assert_eq!(record.rectangularity.parts(), expected);
        }
        assert_eq!(state.phase(), AdaptiveTopologyPhase::Rectangular);
        assert_eq!(state.revision(), 5);
        assert!(state.active_transition().is_none());
        assert!(state.last_commit().unwrap().transition_completed);
    }

    #[test]
    fn stale_preparation_and_missing_rebuild_identity_fail_transactionally() {
        let mut state = AdaptiveTopologyStateV1::circular();
        state
            .request_transition(request(
                "transition.test.stale",
                AdaptiveTopologyTarget::Rectangular,
                2,
            ))
            .unwrap();
        let prepared = state.prepare_next_step().unwrap().unwrap();
        let before = state.clone();
        assert!(
            prepared
                .acknowledge_rebuilds("", "navigation.snapshot.r2")
                .is_err()
        );
        assert_eq!(state, before);

        let evidence = prepared
            .acknowledge_rebuilds("collision.snapshot.r2", "navigation.snapshot.r2")
            .unwrap();
        state.commit_prepared_step(&prepared, evidence).unwrap();
        let before = state.clone();
        let stale_evidence = prepared
            .acknowledge_rebuilds("collision.snapshot.stale", "navigation.snapshot.stale")
            .unwrap();
        assert!(
            state
                .commit_prepared_step(&prepared, stale_evidence)
                .is_err()
        );
        assert_eq!(state, before);
    }

    #[test]
    fn archive_round_trip_is_deterministic_and_rejects_intermediate_drift() {
        let mut state = AdaptiveTopologyStateV1::circular();
        state
            .request_transition(request(
                "transition.test.archive",
                AdaptiveTopologyTarget::Rectangular,
                4,
            ))
            .unwrap();
        commit_next(&mut state);
        let bytes = encode_adaptive_topology_state(&state).unwrap();
        let replayed = decode_adaptive_topology_state(&bytes).unwrap();
        assert_eq!(replayed, state);
        assert_eq!(
            adaptive_topology_state_digest(&replayed).unwrap(),
            adaptive_topology_state_digest(&state).unwrap()
        );

        let mut json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        json["rectangularity"] = serde_json::json!(900_000_000_u64);
        assert!(decode_adaptive_topology_state(&serde_json::to_vec(&json).unwrap()).is_err());
    }

    #[test]
    fn traveler_keeps_route_and_progress_while_its_world_position_adapts() {
        let attachment = AdaptiveRouteAttachment::new(
            "traveler.test.hueman",
            TopologyV2RouteId::CurrentSea,
            NormalizedCoordinate::midpoint(),
        )
        .unwrap();
        let mut state = AdaptiveTopologyStateV1::circular();
        let circular = state.project_traveler(&attachment).unwrap();
        assert_eq!(circular.position.x.parts(), 673_750_000);
        assert_eq!(circular.position.y.parts(), 740_000_000);

        state
            .request_transition(request(
                "transition.test.traveler",
                AdaptiveTopologyTarget::Rectangular,
                2,
            ))
            .unwrap();
        commit_next(&mut state);
        let halfway = state.project_traveler(&attachment).unwrap();
        commit_next(&mut state);
        let rectangular = state.project_traveler(&attachment).unwrap();

        assert_eq!(halfway.route, attachment.route);
        assert_eq!(halfway.route_progress, attachment.route_progress);
        assert_eq!(halfway.position.x.parts(), 704_375_000);
        assert_eq!(halfway.position.y.parts(), 800_000_000);
        assert_eq!(rectangular.position.x.parts(), 735_000_000);
        assert_eq!(rectangular.position.y.parts(), 860_000_000);
    }

    #[test]
    fn completed_state_can_reverse_but_cannot_reuse_a_transition_identity() {
        let mut state = AdaptiveTopologyStateV1::circular();
        state
            .request_transition(request(
                "transition.test.outbound",
                AdaptiveTopologyTarget::Rectangular,
                1,
            ))
            .unwrap();
        commit_next(&mut state);
        assert!(
            state
                .request_transition(request(
                    "transition.test.outbound",
                    AdaptiveTopologyTarget::Circular,
                    1,
                ))
                .is_err()
        );
        state
            .request_transition(request(
                "transition.test.return",
                AdaptiveTopologyTarget::Circular,
                1,
            ))
            .unwrap();
        commit_next(&mut state);
        assert_eq!(state.phase(), AdaptiveTopologyPhase::Circular);
        assert_eq!(state.rectangularity(), NormalizedCoordinate::zero());
    }
}
