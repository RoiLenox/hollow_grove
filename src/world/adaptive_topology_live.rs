//! Durable producer/consumer bridge for live adaptive-topology rebuilds.
//!
//! Rust owns state preparation and commit. Godot consumes a kernel-sampled
//! corridor command and may acknowledge it only after creating collision and
//! navigation geometry for the exact command identity and digest.

use std::fmt;
use std::fs;
use std::io;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::artifact_io::write_bytes_atomically;
use crate::composition::{NORMALIZED_UNIT, NormalizedCoordinate, NormalizedPoint};

use super::adaptive_topology_seasonal::{
    AdaptiveTopologySeasonalError, SeasonalAdaptiveTopologyDisposition,
    apply_completed_function_junction,
};
use super::adaptive_topology_state::{
    ADAPTIVE_TOPOLOGY_STATE_ARTIFACT_PATH, AdaptiveTopologyCommitRecord,
    AdaptiveTopologyStateArchiveError, AdaptiveTopologyStateError, AdaptiveTopologyStateV1,
    decode_adaptive_topology_state, project_adaptive_route_position,
};
use super::function_junction::FunctionJunctionId;
use super::function_junction_archive::{
    FunctionJunctionArchiveError, decode_function_junction_archive,
};
use super::spatial_topology_v2::TopologyV2RouteId;

pub const ADAPTIVE_TOPOLOGY_LIVE_SCHEMA_VERSION: u16 = 1;
pub const ADAPTIVE_TOPOLOGY_LIVE_RULE_SET_ID: &str =
    "ruleset.hollow-grove.adaptive-topology-live-v1";
pub const ADAPTIVE_TOPOLOGY_SEASONAL_EVENT_ARTIFACT_PATH: &str =
    "artifacts/hollow_grove_adaptive_topology_seasonal_event_v1.json";
pub const ADAPTIVE_TOPOLOGY_REBUILD_COMMAND_ARTIFACT_PATH: &str =
    "artifacts/hollow_grove_adaptive_topology_rebuild_command_v1.json";
pub const ADAPTIVE_TOPOLOGY_REBUILD_ACK_ARTIFACT_PATH: &str =
    "artifacts/hollow_grove_adaptive_topology_rebuild_ack_v1.json";
pub const ADAPTIVE_TOPOLOGY_ROUTE_SAMPLE_SEGMENTS: u32 = 32;
pub const ADAPTIVE_TOPOLOGY_CORRIDOR_HALF_WIDTH_PARTS: u64 = 8_000_000;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdaptiveTopologySeasonalEventV1 {
    pub schema_version: u16,
    pub rule_set: String,
    pub junction_id: String,
    pub function_junction_archive: serde_json::Value,
}

impl AdaptiveTopologySeasonalEventV1 {
    pub fn from_archive(
        archive_bytes: &[u8],
        junction_id: &str,
    ) -> Result<Self, AdaptiveTopologyLiveError> {
        decode_function_junction_archive(archive_bytes)?;
        let event = Self {
            schema_version: ADAPTIVE_TOPOLOGY_LIVE_SCHEMA_VERSION,
            rule_set: ADAPTIVE_TOPOLOGY_LIVE_RULE_SET_ID.to_owned(),
            junction_id: junction_id.to_owned(),
            function_junction_archive: serde_json::from_slice(archive_bytes)?,
        };
        event.validated_junction()?;
        Ok(event)
    }

    fn validated_junction(
        &self,
    ) -> Result<
        (
            super::function_junction::FunctionJunctionRuntime,
            FunctionJunctionId,
        ),
        AdaptiveTopologyLiveError,
    > {
        if self.schema_version != ADAPTIVE_TOPOLOGY_LIVE_SCHEMA_VERSION {
            return Err(AdaptiveTopologyLiveError::UnsupportedSchema {
                expected: ADAPTIVE_TOPOLOGY_LIVE_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.rule_set != ADAPTIVE_TOPOLOGY_LIVE_RULE_SET_ID {
            return Err(AdaptiveTopologyLiveError::RuleSetMismatch);
        }
        let archive_bytes = serde_json::to_vec(&self.function_junction_archive)?;
        let decoded = decode_function_junction_archive(&archive_bytes)?;
        let junction_id = FunctionJunctionId::new(&self.junction_id)
            .map_err(|error| AdaptiveTopologyLiveError::FunctionJunction(error.to_string()))?;
        let mut matches = decoded
            .annual_states
            .into_values()
            .filter(|annual| {
                annual
                    .function_junction_runtime
                    .junctions()
                    .contains_key(&junction_id)
            })
            .map(|annual| annual.function_junction_runtime);
        let runtime = matches.next().ok_or_else(|| {
            AdaptiveTopologyLiveError::UnknownFunctionJunction(self.junction_id.clone())
        })?;
        if matches.next().is_some() {
            return Err(AdaptiveTopologyLiveError::AmbiguousFunctionJunction(
                self.junction_id.clone(),
            ));
        }
        Ok((runtime, junction_id))
    }

    pub fn apply(
        &self,
        state: &mut AdaptiveTopologyStateV1,
    ) -> Result<SeasonalAdaptiveTopologyDisposition, AdaptiveTopologyLiveError> {
        let (runtime, junction_id) = self.validated_junction()?;
        Ok(apply_completed_function_junction(
            state,
            &runtime,
            &junction_id,
        )?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdaptiveTopologyRebuildCommandStatus {
    Committed,
    Prepared,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdaptiveTopologyRouteCorridorV1 {
    pub route_id: String,
    pub points: Vec<NormalizedPoint>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdaptiveTopologyGeometryV1 {
    pub normalization_unit: u64,
    pub rectangularity: NormalizedCoordinate,
    pub corridor_half_width_parts: u64,
    pub sample_segments: u32,
    pub geometry_digest: String,
    pub routes: Vec<AdaptiveTopologyRouteCorridorV1>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PreparedAdaptiveTopologyRebuildV1 {
    pub transition_id: String,
    pub trigger_id: String,
    pub base_revision: u64,
    pub next_revision: u64,
    pub tick: u32,
    pub duration_ticks: u32,
    pub transition_completed: bool,
    pub collision_snapshot_id: String,
    pub navigation_snapshot_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdaptiveTopologyRebuildCommandV1 {
    pub schema_version: u16,
    pub rule_set: String,
    pub command_id: String,
    pub status: AdaptiveTopologyRebuildCommandStatus,
    pub state_revision: u64,
    pub geometry: AdaptiveTopologyGeometryV1,
    pub prepared: Option<PreparedAdaptiveTopologyRebuildV1>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdaptiveTopologyRebuildAcknowledgementV1 {
    pub schema_version: u16,
    pub rule_set: String,
    pub command_id: String,
    pub transition_id: String,
    pub base_revision: u64,
    pub next_revision: u64,
    pub rectangularity: NormalizedCoordinate,
    pub geometry_digest: String,
    pub collision_snapshot_id: String,
    pub navigation_snapshot_id: String,
    pub route_count: usize,
    pub collision_polygon_count: usize,
    pub navigation_region_count: usize,
}

impl AdaptiveTopologyRebuildAcknowledgementV1 {
    pub fn for_command(
        command: &AdaptiveTopologyRebuildCommandV1,
        collision_polygon_count: usize,
        navigation_region_count: usize,
    ) -> Result<Self, AdaptiveTopologyLiveError> {
        let prepared = command
            .prepared
            .as_ref()
            .ok_or(AdaptiveTopologyLiveError::AcknowledgementForCommittedGeometry)?;
        Ok(Self {
            schema_version: ADAPTIVE_TOPOLOGY_LIVE_SCHEMA_VERSION,
            rule_set: ADAPTIVE_TOPOLOGY_LIVE_RULE_SET_ID.to_owned(),
            command_id: command.command_id.clone(),
            transition_id: prepared.transition_id.clone(),
            base_revision: prepared.base_revision,
            next_revision: prepared.next_revision,
            rectangularity: command.geometry.rectangularity,
            geometry_digest: command.geometry.geometry_digest.clone(),
            collision_snapshot_id: prepared.collision_snapshot_id.clone(),
            navigation_snapshot_id: prepared.navigation_snapshot_id.clone(),
            route_count: command.geometry.routes.len(),
            collision_polygon_count,
            navigation_region_count,
        })
    }
}

pub fn build_adaptive_topology_rebuild_command(
    state: &AdaptiveTopologyStateV1,
) -> Result<AdaptiveTopologyRebuildCommandV1, AdaptiveTopologyLiveError> {
    state.validate()?;
    let prepared_step = state.prepare_next_step()?;
    let rectangularity = prepared_step
        .as_ref()
        .map_or_else(|| state.rectangularity(), |step| step.rectangularity);
    let geometry = build_geometry(rectangularity)?;
    let digest_suffix = geometry
        .geometry_digest
        .rsplit_once(':')
        .map_or(geometry.geometry_digest.as_str(), |(_, suffix)| suffix);
    let (status, command_revision, prepared) = if let Some(step) = prepared_step {
        let collision_snapshot_id = format!(
            "collision.snapshot.adaptive-topology.r{}.{digest_suffix}",
            step.next_revision
        );
        let navigation_snapshot_id = format!(
            "navigation.snapshot.adaptive-topology.r{}.{digest_suffix}",
            step.next_revision
        );
        (
            AdaptiveTopologyRebuildCommandStatus::Prepared,
            step.next_revision,
            Some(PreparedAdaptiveTopologyRebuildV1 {
                transition_id: step.transition_id,
                trigger_id: step.trigger_id,
                base_revision: step.base_revision,
                next_revision: step.next_revision,
                tick: step.tick,
                duration_ticks: step.duration_ticks,
                transition_completed: step.transition_completed,
                collision_snapshot_id,
                navigation_snapshot_id,
            }),
        )
    } else {
        (
            AdaptiveTopologyRebuildCommandStatus::Committed,
            state.revision(),
            None,
        )
    };
    Ok(AdaptiveTopologyRebuildCommandV1 {
        schema_version: ADAPTIVE_TOPOLOGY_LIVE_SCHEMA_VERSION,
        rule_set: ADAPTIVE_TOPOLOGY_LIVE_RULE_SET_ID.to_owned(),
        command_id: format!(
            "adaptive-topology-rebuild.{status:?}.r{command_revision}.{digest_suffix}"
        )
        .to_ascii_lowercase(),
        status,
        state_revision: state.revision(),
        geometry,
        prepared,
    })
}

fn build_geometry(
    rectangularity: NormalizedCoordinate,
) -> Result<AdaptiveTopologyGeometryV1, AdaptiveTopologyLiveError> {
    let mut routes = Vec::with_capacity(TopologyV2RouteId::ALL.len());
    for route in TopologyV2RouteId::ALL {
        let mut points = Vec::with_capacity(ADAPTIVE_TOPOLOGY_ROUTE_SAMPLE_SEGMENTS as usize + 1);
        for segment in 0..=ADAPTIVE_TOPOLOGY_ROUTE_SAMPLE_SEGMENTS {
            let progress = NormalizedCoordinate::from_ratio(
                u64::from(segment),
                u64::from(ADAPTIVE_TOPOLOGY_ROUTE_SAMPLE_SEGMENTS),
            )
            .map_err(AdaptiveTopologyStateError::from)?;
            points.push(project_adaptive_route_position(
                route,
                progress,
                rectangularity,
            )?);
        }
        routes.push(AdaptiveTopologyRouteCorridorV1 {
            route_id: route.stable_id().to_owned(),
            points,
        });
    }
    let hash = geometry_hash(rectangularity, &routes);
    Ok(AdaptiveTopologyGeometryV1 {
        normalization_unit: NORMALIZED_UNIT,
        rectangularity,
        corridor_half_width_parts: ADAPTIVE_TOPOLOGY_CORRIDOR_HALF_WIDTH_PARTS,
        sample_segments: ADAPTIVE_TOPOLOGY_ROUTE_SAMPLE_SEGMENTS,
        geometry_digest: format!("fnv1a32-adaptive-topology-geometry-v1:{hash:08x}"),
        routes,
    })
}

fn geometry_hash(
    rectangularity: NormalizedCoordinate,
    routes: &[AdaptiveTopologyRouteCorridorV1],
) -> u32 {
    let mut canonical_parts = vec![
        String::from("v1"),
        rectangularity.parts().to_string(),
        ADAPTIVE_TOPOLOGY_CORRIDOR_HALF_WIDTH_PARTS.to_string(),
        ADAPTIVE_TOPOLOGY_ROUTE_SAMPLE_SEGMENTS.to_string(),
    ];
    for route in routes {
        canonical_parts.push(route.route_id.clone());
        for point in &route.points {
            canonical_parts.push(point.x.parts().to_string());
            canonical_parts.push(point.y.parts().to_string());
        }
    }
    let mut hash = 0x811c9dc5_u32;
    let bytes = canonical_parts.join("|").into_bytes();
    for byte in bytes {
        hash ^= u32::from(byte);
        hash = hash.wrapping_mul(0x01000193);
    }
    hash
}

pub fn commit_adaptive_topology_rebuild(
    state: &mut AdaptiveTopologyStateV1,
    acknowledgement: &AdaptiveTopologyRebuildAcknowledgementV1,
) -> Result<AdaptiveTopologyCommitRecord, AdaptiveTopologyLiveError> {
    let command = build_adaptive_topology_rebuild_command(state)?;
    let prepared_command = command
        .prepared
        .as_ref()
        .ok_or(AdaptiveTopologyLiveError::NoPreparedRebuild)?;
    let prepared_step = state
        .prepare_next_step()?
        .ok_or(AdaptiveTopologyLiveError::NoPreparedRebuild)?;
    if acknowledgement.schema_version != ADAPTIVE_TOPOLOGY_LIVE_SCHEMA_VERSION
        || acknowledgement.rule_set != ADAPTIVE_TOPOLOGY_LIVE_RULE_SET_ID
        || acknowledgement.command_id != command.command_id
        || acknowledgement.transition_id != prepared_command.transition_id
        || acknowledgement.base_revision != prepared_command.base_revision
        || acknowledgement.next_revision != prepared_command.next_revision
        || acknowledgement.rectangularity != command.geometry.rectangularity
        || acknowledgement.geometry_digest != command.geometry.geometry_digest
        || acknowledgement.collision_snapshot_id != prepared_command.collision_snapshot_id
        || acknowledgement.navigation_snapshot_id != prepared_command.navigation_snapshot_id
        || acknowledgement.route_count != command.geometry.routes.len()
        || acknowledgement.collision_polygon_count < acknowledgement.route_count
        || acknowledgement.navigation_region_count == 0
    {
        return Err(AdaptiveTopologyLiveError::RebuildAcknowledgementMismatch);
    }
    let evidence = prepared_step.acknowledge_rebuilds(
        &acknowledgement.collision_snapshot_id,
        &acknowledgement.navigation_snapshot_id,
    )?;
    Ok(state.commit_prepared_step(&prepared_step, evidence)?)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdaptiveTopologyLiveStepReport {
    pub state_revision: u64,
    pub rectangularity: NormalizedCoordinate,
    pub seasonal_disposition: Option<String>,
    pub committed_revision: Option<u64>,
    pub acknowledged_command_id: Option<String>,
    pub acknowledged_geometry_digest: Option<String>,
    pub collision_polygon_count: Option<usize>,
    pub navigation_region_count: Option<usize>,
    pub stale_acknowledgement_ignored: bool,
    pub command_id: String,
    pub command_status: AdaptiveTopologyRebuildCommandStatus,
}

pub fn submit_adaptive_topology_seasonal_event(
    root: &Path,
    archive_bytes: &[u8],
    junction_id: &str,
) -> Result<AdaptiveTopologySeasonalEventV1, AdaptiveTopologyLiveError> {
    let event = AdaptiveTopologySeasonalEventV1::from_archive(archive_bytes, junction_id)?;
    write_json_if_changed(
        &root.join(ADAPTIVE_TOPOLOGY_SEASONAL_EVENT_ARTIFACT_PATH),
        &event,
    )?;
    Ok(event)
}

pub fn process_adaptive_topology_live_once(
    root: &Path,
) -> Result<AdaptiveTopologyLiveStepReport, AdaptiveTopologyLiveError> {
    let state_path = root.join(ADAPTIVE_TOPOLOGY_STATE_ARTIFACT_PATH);
    let mut state = match fs::read(&state_path) {
        Ok(bytes) => decode_adaptive_topology_state(&bytes)?,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            AdaptiveTopologyStateV1::circular()
        }
        Err(error) => return Err(error.into()),
    };
    let mut committed_revision = None;
    let mut acknowledged_command_id = None;
    let mut acknowledged_geometry_digest = None;
    let mut collision_polygon_count = None;
    let mut navigation_region_count = None;
    let mut stale_acknowledgement_ignored = false;

    let event_path = root.join(ADAPTIVE_TOPOLOGY_SEASONAL_EVENT_ARTIFACT_PATH);
    let seasonal_disposition = match fs::read(&event_path) {
        Ok(bytes) => {
            let event: AdaptiveTopologySeasonalEventV1 = serde_json::from_slice(&bytes)?;
            Some(format!("{:?}", event.apply(&mut state)?))
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => None,
        Err(error) => return Err(error.into()),
    };

    let ack_path = root.join(ADAPTIVE_TOPOLOGY_REBUILD_ACK_ARTIFACT_PATH);
    match fs::read(&ack_path) {
        Ok(bytes) => {
            let acknowledgement: AdaptiveTopologyRebuildAcknowledgementV1 =
                serde_json::from_slice(&bytes)?;
            let expected = build_adaptive_topology_rebuild_command(&state)?;
            if acknowledgement.command_id == expected.command_id && expected.prepared.is_some() {
                acknowledged_command_id = Some(acknowledgement.command_id.clone());
                acknowledged_geometry_digest = Some(acknowledgement.geometry_digest.clone());
                collision_polygon_count = Some(acknowledgement.collision_polygon_count);
                navigation_region_count = Some(acknowledgement.navigation_region_count);
                let record = commit_adaptive_topology_rebuild(&mut state, &acknowledgement)?;
                committed_revision = Some(record.revision);
            } else {
                stale_acknowledgement_ignored = true;
            }
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => return Err(error.into()),
    }

    write_json_if_changed(&state_path, &state)?;
    let command = build_adaptive_topology_rebuild_command(&state)?;
    write_json_if_changed(
        &root.join(ADAPTIVE_TOPOLOGY_REBUILD_COMMAND_ARTIFACT_PATH),
        &command,
    )?;
    Ok(AdaptiveTopologyLiveStepReport {
        state_revision: state.revision(),
        rectangularity: state.rectangularity(),
        seasonal_disposition,
        committed_revision,
        acknowledged_command_id,
        acknowledged_geometry_digest,
        collision_polygon_count,
        navigation_region_count,
        stale_acknowledgement_ignored,
        command_id: command.command_id,
        command_status: command.status,
    })
}

fn write_json_if_changed(
    path: &Path,
    value: &impl Serialize,
) -> Result<(), AdaptiveTopologyLiveError> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    if fs::read(path).is_ok_and(|existing| existing == bytes) {
        return Ok(());
    }
    write_bytes_atomically(path, &bytes)?;
    Ok(())
}

#[derive(Debug)]
pub enum AdaptiveTopologyLiveError {
    Io(io::Error),
    Json(serde_json::Error),
    State(AdaptiveTopologyStateError),
    StateArchive(AdaptiveTopologyStateArchiveError),
    Seasonal(AdaptiveTopologySeasonalError),
    FunctionJunctionArchive(FunctionJunctionArchiveError),
    FunctionJunction(String),
    UnsupportedSchema { expected: u16, actual: u16 },
    RuleSetMismatch,
    UnknownFunctionJunction(String),
    AmbiguousFunctionJunction(String),
    AcknowledgementForCommittedGeometry,
    NoPreparedRebuild,
    RebuildAcknowledgementMismatch,
}

macro_rules! live_error_from {
    ($source:ty, $variant:ident) => {
        impl From<$source> for AdaptiveTopologyLiveError {
            fn from(value: $source) -> Self {
                Self::$variant(value)
            }
        }
    };
}

live_error_from!(io::Error, Io);
live_error_from!(serde_json::Error, Json);
live_error_from!(AdaptiveTopologyStateError, State);
live_error_from!(AdaptiveTopologyStateArchiveError, StateArchive);
live_error_from!(AdaptiveTopologySeasonalError, Seasonal);
live_error_from!(FunctionJunctionArchiveError, FunctionJunctionArchive);

impl fmt::Display for AdaptiveTopologyLiveError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => error.fmt(formatter),
            Self::Json(error) => write!(formatter, "adaptive topology live JSON error: {error}"),
            Self::State(error) => error.fmt(formatter),
            Self::StateArchive(error) => error.fmt(formatter),
            Self::Seasonal(error) => error.fmt(formatter),
            Self::FunctionJunctionArchive(error) => error.fmt(formatter),
            Self::FunctionJunction(error) => formatter.write_str(error),
            Self::UnsupportedSchema { expected, actual } => write!(
                formatter,
                "unsupported adaptive topology live schema {actual}; expected {expected}"
            ),
            Self::RuleSetMismatch => formatter.write_str("adaptive topology live rule-set mismatch"),
            Self::UnknownFunctionJunction(id) => write!(formatter, "unknown Function Junction: {id}"),
            Self::AmbiguousFunctionJunction(id) => write!(formatter, "Function Junction identity appears in multiple annual states: {id}"),
            Self::AcknowledgementForCommittedGeometry => formatter.write_str("committed geometry does not require a rebuild acknowledgement"),
            Self::NoPreparedRebuild => formatter.write_str("no prepared adaptive topology rebuild is active"),
            Self::RebuildAcknowledgementMismatch => formatter.write_str("Godot collision/navigation acknowledgement does not match the prepared topology command"),
        }
    }
}

impl std::error::Error for AdaptiveTopologyLiveError {}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;
    use crate::constitutional::SeasonalAnchor;
    use crate::world::function_junction_archive::encode_function_junction_archive;
    use crate::world::function_junction_fixture::canonical_function_junction_archive_fixture;

    fn archive_and_spring() -> (Vec<u8>, String) {
        let payload = canonical_function_junction_archive_fixture();
        let spring = payload.annual_records[0]
            .junctions
            .iter()
            .find(|junction| junction.anchor == SeasonalAnchor::SpringEquinox)
            .unwrap()
            .junction_id
            .as_str()
            .to_owned();
        (encode_function_junction_archive(&payload).unwrap(), spring)
    }

    fn temp_root() -> std::path::PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("adaptive-topology-live-{nonce}"))
    }

    #[test]
    fn committed_and_prepared_commands_contain_all_kernel_sampled_corridors() {
        let mut state = AdaptiveTopologyStateV1::circular();
        let committed = build_adaptive_topology_rebuild_command(&state).unwrap();
        assert_eq!(
            committed.status,
            AdaptiveTopologyRebuildCommandStatus::Committed
        );
        assert_eq!(committed.geometry.routes.len(), 9);
        assert!(committed.prepared.is_none());
        assert!(
            committed
                .geometry
                .routes
                .iter()
                .all(|route| route.points.len() == 33)
        );

        let (archive, spring) = archive_and_spring();
        AdaptiveTopologySeasonalEventV1::from_archive(&archive, &spring)
            .unwrap()
            .apply(&mut state)
            .unwrap();
        let prepared = build_adaptive_topology_rebuild_command(&state).unwrap();
        assert_eq!(
            prepared.status,
            AdaptiveTopologyRebuildCommandStatus::Prepared
        );
        assert_eq!(prepared.geometry.rectangularity.parts(), 250_000_000);
        assert!(prepared.prepared.is_some());
    }

    #[test]
    fn acknowledgement_must_match_geometry_and_both_rebuild_counts() {
        let (archive, spring) = archive_and_spring();
        let event = AdaptiveTopologySeasonalEventV1::from_archive(&archive, &spring).unwrap();
        let mut state = AdaptiveTopologyStateV1::circular();
        event.apply(&mut state).unwrap();
        let command = build_adaptive_topology_rebuild_command(&state).unwrap();
        let mut bad =
            AdaptiveTopologyRebuildAcknowledgementV1::for_command(&command, 9, 9).unwrap();
        bad.navigation_region_count = 0;
        let before = state.clone();
        assert!(commit_adaptive_topology_rebuild(&mut state, &bad).is_err());
        assert_eq!(state, before);

        let good = AdaptiveTopologyRebuildAcknowledgementV1::for_command(&command, 9, 9).unwrap();
        commit_adaptive_topology_rebuild(&mut state, &good).unwrap();
        assert_eq!(state.rectangularity().parts(), 250_000_000);
    }

    #[test]
    fn filesystem_bridge_runs_spring_through_four_atomic_handshakes() {
        let root = temp_root();
        fs::create_dir_all(root.join("artifacts")).unwrap();
        let (archive, spring) = archive_and_spring();
        submit_adaptive_topology_seasonal_event(&root, &archive, &spring).unwrap();
        process_adaptive_topology_live_once(&root).unwrap();

        for expected in [250_000_000, 500_000_000, 750_000_000, 1_000_000_000] {
            let command: AdaptiveTopologyRebuildCommandV1 = serde_json::from_slice(
                &fs::read(root.join(ADAPTIVE_TOPOLOGY_REBUILD_COMMAND_ARTIFACT_PATH)).unwrap(),
            )
            .unwrap();
            assert_eq!(
                command.status,
                AdaptiveTopologyRebuildCommandStatus::Prepared
            );
            let acknowledgement =
                AdaptiveTopologyRebuildAcknowledgementV1::for_command(&command, 9, 9).unwrap();
            write_json_if_changed(
                &root.join(ADAPTIVE_TOPOLOGY_REBUILD_ACK_ARTIFACT_PATH),
                &acknowledgement,
            )
            .unwrap();
            process_adaptive_topology_live_once(&root).unwrap();
            let state = decode_adaptive_topology_state(
                &fs::read(root.join(ADAPTIVE_TOPOLOGY_STATE_ARTIFACT_PATH)).unwrap(),
            )
            .unwrap();
            assert_eq!(state.rectangularity().parts(), expected);
        }

        let command: AdaptiveTopologyRebuildCommandV1 = serde_json::from_slice(
            &fs::read(root.join(ADAPTIVE_TOPOLOGY_REBUILD_COMMAND_ARTIFACT_PATH)).unwrap(),
        )
        .unwrap();
        assert_eq!(
            command.status,
            AdaptiveTopologyRebuildCommandStatus::Committed
        );
        assert!(command.prepared.is_none());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn tampered_function_junction_archive_cannot_submit_an_event() {
        let (archive, spring) = archive_and_spring();
        let mut json: serde_json::Value = serde_json::from_slice(&archive).unwrap();
        json["checksum"] = serde_json::json!("tampered");
        assert!(
            AdaptiveTopologySeasonalEventV1::from_archive(
                &serde_json::to_vec(&json).unwrap(),
                &spring,
            )
            .is_err()
        );
    }
}
