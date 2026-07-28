//! Versioned persistence and replay for the Hollow Grove world Point binding.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::composition::{
    AxisHandedness, CompositionRecordId, ExpandedPointField, FieldId, OrientedPoint,
    OrientedPointError, PhysicalExtent, PhysicalPosition, PointCenterId, PointId, PointInversion,
    PointScaling, PolarityAxis, PoleId, ScaleKey, SpatialEvidenceId, SpatialRegionId, invert_point,
    lawfully_scale_point,
};

use super::world_point::{
    HollowGroveWorldPointBinding, WorldCardinalOrientation, WorldFieldRelationship,
    WorldFieldRelationshipId, WorldMigrationDecisionId, WorldPointBindingError,
};

pub const WORLD_POINT_ARCHIVE_FORMAT: &str = "HGPNT";
pub const WORLD_POINT_ARCHIVE_VERSION: u16 = 1;
pub const WORLD_POINT_LEGACY_ARCHIVE_VERSION: u16 = 0;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldPointArchivePayload {
    pub source_point: OrientedPoint,
    pub scaling: PointScaling,
    pub binding: HollowGroveWorldPointBinding,
    pub relationships: Vec<WorldFieldRelationship>,
    pub explicit_inversion_probe: PointInversion,
}

impl WorldPointArchivePayload {
    #[must_use]
    pub fn canonicalized(&self) -> Self {
        let mut payload = self.clone();
        payload
            .relationships
            .sort_by(|left, right| left.relationship_id.cmp(&right.relationship_id));
        payload
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorldPointReplayState {
    pub source_point: OrientedPoint,
    pub binding: HollowGroveWorldPointBinding,
    pub relationships: BTreeMap<WorldFieldRelationshipId, WorldFieldRelationship>,
    pub explicit_inversion_probe_result: OrientedPoint,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodedWorldPointArchive {
    pub archive_version: u16,
    pub checksum: String,
    pub payload: WorldPointArchivePayload,
    pub state: WorldPointReplayState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ArchiveHeader {
    format: String,
    archive_version: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct WorldPointArchiveEnvelope {
    format: String,
    archive_version: u16,
    checksum: String,
    payload: WorldPointArchivePayload,
}

pub fn replay_world_point_payload(
    payload: &WorldPointArchivePayload,
) -> Result<WorldPointReplayState, WorldPointArchiveError> {
    let payload = payload.canonicalized();
    payload.source_point.validate()?;
    let world_point = lawfully_scale_point(&payload.source_point, &payload.scaling)?;
    if world_point != payload.binding.point {
        return Err(WorldPointArchiveError::ScalingBindingMismatch);
    }
    payload.binding.validate()?;

    let mut relationships = BTreeMap::new();
    for relationship in &payload.relationships {
        relationship.validate()?;
        if relationships
            .insert(relationship.relationship_id.clone(), relationship.clone())
            .is_some()
        {
            return Err(WorldPointArchiveError::DuplicateRelationship(
                relationship.relationship_id.clone(),
            ));
        }
    }

    let inverted = invert_point(&payload.binding.point, &payload.explicit_inversion_probe)?;
    if inverted.point_id == payload.binding.point.point_id
        || inverted.positive_pole_id != payload.binding.point.negative_pole_id
        || inverted.negative_pole_id != payload.binding.point.positive_pole_id
        || inverted.orientation != payload.binding.point.orientation.inverted()
    {
        return Err(WorldPointArchiveError::InvalidInversionProbe);
    }

    Ok(WorldPointReplayState {
        source_point: payload.source_point,
        binding: payload.binding,
        relationships,
        explicit_inversion_probe_result: inverted,
    })
}

pub fn encode_world_point_archive(
    payload: &WorldPointArchivePayload,
) -> Result<Vec<u8>, WorldPointArchiveError> {
    let payload = payload.canonicalized();
    replay_world_point_payload(&payload)?;
    let envelope = WorldPointArchiveEnvelope {
        format: WORLD_POINT_ARCHIVE_FORMAT.into(),
        archive_version: WORLD_POINT_ARCHIVE_VERSION,
        checksum: checksum(&payload)?,
        payload,
    };
    serde_json::to_vec(&envelope).map_err(|error| WorldPointArchiveError::Json(error.to_string()))
}

pub fn decode_world_point_archive(
    bytes: &[u8],
) -> Result<DecodedWorldPointArchive, WorldPointArchiveError> {
    let header: ArchiveHeader = serde_json::from_slice(bytes)
        .map_err(|error| WorldPointArchiveError::Json(error.to_string()))?;
    if header.format != WORLD_POINT_ARCHIVE_FORMAT {
        return Err(WorldPointArchiveError::UnsupportedFormat(header.format));
    }
    match header.archive_version {
        WORLD_POINT_ARCHIVE_VERSION => decode_current(bytes),
        WORLD_POINT_LEGACY_ARCHIVE_VERSION => decode_legacy(bytes),
        version => Err(WorldPointArchiveError::UnsupportedVersion(version)),
    }
}

fn decode_current(bytes: &[u8]) -> Result<DecodedWorldPointArchive, WorldPointArchiveError> {
    let envelope: WorldPointArchiveEnvelope = serde_json::from_slice(bytes)
        .map_err(|error| WorldPointArchiveError::Json(error.to_string()))?;
    let payload = envelope.payload.canonicalized();
    if envelope.checksum != checksum(&payload)? {
        return Err(WorldPointArchiveError::ChecksumMismatch);
    }
    let state = replay_world_point_payload(&payload)?;
    Ok(DecodedWorldPointArchive {
        archive_version: envelope.archive_version,
        checksum: envelope.checksum,
        payload,
        state,
    })
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct LegacyUnorientedPoint {
    point_id: PointId,
    center_id: PointCenterId,
    center: PhysicalPosition,
    positive_pole_id: PoleId,
    negative_pole_id: PoleId,
    scale: ScaleKey,
    extent: PhysicalExtent,
    evidence_ids: BTreeSet<SpatialEvidenceId>,
    provenance_ids: BTreeSet<CompositionRecordId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LegacyWorldOrientationDecision {
    PositiveNorthTopNegativeSouthBottom,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct LegacyOrientationMigration {
    decision_id: WorldMigrationDecisionId,
    decision: LegacyWorldOrientationDecision,
    evidence_ids: BTreeSet<SpatialEvidenceId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct LegacyWorldPointPayloadV0 {
    source_point: LegacyUnorientedPoint,
    scaling: PointScaling,
    field_id: FieldId,
    center_region_id: SpatialRegionId,
    positive_region_id: SpatialRegionId,
    negative_region_id: SpatialRegionId,
    field_evidence_ids: BTreeSet<SpatialEvidenceId>,
    binding_evidence_ids: BTreeSet<SpatialEvidenceId>,
    relationships: Vec<WorldFieldRelationship>,
    explicit_inversion_probe: PointInversion,
    migration: LegacyOrientationMigration,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct LegacyWorldPointEnvelopeV0 {
    format: String,
    archive_version: u16,
    checksum: String,
    payload: LegacyWorldPointPayloadV0,
}

pub fn encode_legacy_world_point_archive_v0(
    payload: &WorldPointArchivePayload,
) -> Result<Vec<u8>, WorldPointArchiveError> {
    let payload = payload.canonicalized();
    replay_world_point_payload(&payload)?;
    let legacy = LegacyWorldPointPayloadV0 {
        source_point: LegacyUnorientedPoint {
            point_id: payload.source_point.point_id,
            center_id: payload.source_point.center_id,
            center: payload.source_point.center,
            positive_pole_id: payload.source_point.positive_pole_id,
            negative_pole_id: payload.source_point.negative_pole_id,
            scale: payload.source_point.scale,
            extent: payload.source_point.extent,
            evidence_ids: payload.source_point.evidence_ids,
            provenance_ids: payload.source_point.provenance_ids,
        },
        scaling: payload.scaling,
        field_id: payload.binding.field.field_id,
        center_region_id: payload.binding.field.center_region_id,
        positive_region_id: payload.binding.field.positive_region_id,
        negative_region_id: payload.binding.field.negative_region_id,
        field_evidence_ids: payload.binding.field.evidence_ids,
        binding_evidence_ids: payload.binding.evidence_ids,
        relationships: payload.relationships,
        explicit_inversion_probe: payload.explicit_inversion_probe,
        migration: LegacyOrientationMigration {
            decision_id: WorldMigrationDecisionId::new(
                "migration.world-point.orientation-v0-to-v1",
            )?,
            decision: LegacyWorldOrientationDecision::PositiveNorthTopNegativeSouthBottom,
            evidence_ids: [SpatialEvidenceId::new(
                "evidence.migration.world-point.north-positive",
            )?]
            .into_iter()
            .collect(),
        },
    };
    let envelope = LegacyWorldPointEnvelopeV0 {
        format: WORLD_POINT_ARCHIVE_FORMAT.into(),
        archive_version: WORLD_POINT_LEGACY_ARCHIVE_VERSION,
        checksum: legacy_checksum(&legacy)?,
        payload: legacy,
    };
    serde_json::to_vec(&envelope).map_err(|error| WorldPointArchiveError::Json(error.to_string()))
}

fn decode_legacy(bytes: &[u8]) -> Result<DecodedWorldPointArchive, WorldPointArchiveError> {
    let envelope: LegacyWorldPointEnvelopeV0 = serde_json::from_slice(bytes)
        .map_err(|error| WorldPointArchiveError::Json(error.to_string()))?;
    if envelope.checksum != legacy_checksum(&envelope.payload)? {
        return Err(WorldPointArchiveError::ChecksumMismatch);
    }
    if envelope.payload.migration.evidence_ids.is_empty()
        || envelope.payload.migration.decision
            != LegacyWorldOrientationDecision::PositiveNorthTopNegativeSouthBottom
    {
        return Err(WorldPointArchiveError::AmbiguousLegacyOrientation);
    }
    let legacy = envelope.payload;
    let orientation = PolarityAxis::new([0, 1, 0], AxisHandedness::RightHanded)?;
    let source_point = OrientedPoint {
        point_id: legacy.source_point.point_id,
        center_id: legacy.source_point.center_id,
        center: legacy.source_point.center,
        orientation,
        positive_pole_id: legacy.source_point.positive_pole_id,
        negative_pole_id: legacy.source_point.negative_pole_id,
        scale: legacy.source_point.scale,
        extent: legacy.source_point.extent,
        evidence_ids: legacy
            .source_point
            .evidence_ids
            .into_iter()
            .chain(legacy.migration.evidence_ids)
            .collect(),
        provenance_ids: legacy.source_point.provenance_ids,
    };
    let point = lawfully_scale_point(&source_point, &legacy.scaling)?;
    let field = ExpandedPointField {
        field_id: legacy.field_id,
        source_point_id: point.point_id.clone(),
        scale: point.scale.clone(),
        center_region_id: legacy.center_region_id.clone(),
        positive_region_id: legacy.positive_region_id.clone(),
        negative_region_id: legacy.negative_region_id.clone(),
        axis: orientation,
        evidence_ids: legacy.field_evidence_ids,
        provenance_ids: [legacy.scaling.scaling_id.clone()].into_iter().collect(),
    };
    let binding = HollowGroveWorldPointBinding {
        point,
        field,
        cardinal_orientation: WorldCardinalOrientation::PositiveNorthTopNegativeSouthBottom,
        light_aura_region_id: legacy.positive_region_id,
        central_junction_region_id: legacy.center_region_id,
        dark_aura_region_id: legacy.negative_region_id,
        lawfulness_requires_separate_determination: true,
        presentation_may_change_constitutional_polarity: false,
        evidence_ids: legacy.binding_evidence_ids,
    };
    let payload = WorldPointArchivePayload {
        source_point,
        scaling: legacy.scaling,
        binding,
        relationships: legacy.relationships,
        explicit_inversion_probe: legacy.explicit_inversion_probe,
    }
    .canonicalized();
    let state = replay_world_point_payload(&payload)?;
    Ok(DecodedWorldPointArchive {
        archive_version: WORLD_POINT_LEGACY_ARCHIVE_VERSION,
        checksum: envelope.checksum,
        payload,
        state,
    })
}

pub fn migrate_world_point_archive(bytes: &[u8]) -> Result<Vec<u8>, WorldPointArchiveError> {
    let decoded = decode_world_point_archive(bytes)?;
    encode_world_point_archive(&decoded.payload)
}

fn checksum(payload: &WorldPointArchivePayload) -> Result<String, WorldPointArchiveError> {
    let bytes = serde_json::to_vec(payload)
        .map_err(|error| WorldPointArchiveError::Json(error.to_string()))?;
    Ok(fnv1a64(&bytes))
}

fn legacy_checksum(payload: &LegacyWorldPointPayloadV0) -> Result<String, WorldPointArchiveError> {
    let bytes = serde_json::to_vec(payload)
        .map_err(|error| WorldPointArchiveError::Json(error.to_string()))?;
    Ok(fnv1a64(&bytes))
}

fn fnv1a64(bytes: &[u8]) -> String {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("{hash:016x}")
}

#[derive(Debug)]
pub enum WorldPointArchiveError {
    Json(String),
    UnsupportedFormat(String),
    UnsupportedVersion(u16),
    ChecksumMismatch,
    ScalingBindingMismatch,
    DuplicateRelationship(WorldFieldRelationshipId),
    InvalidInversionProbe,
    AmbiguousLegacyOrientation,
    StableKey(crate::composition::StableKeyError),
    Kernel(OrientedPointError),
    Binding(WorldPointBindingError),
}

impl fmt::Display for WorldPointArchiveError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "world Point archive rejected state: {self:?}")
    }
}

impl std::error::Error for WorldPointArchiveError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::StableKey(error) => Some(error),
            Self::Kernel(error) => Some(error),
            Self::Binding(error) => Some(error),
            _ => None,
        }
    }
}

impl From<crate::composition::StableKeyError> for WorldPointArchiveError {
    fn from(value: crate::composition::StableKeyError) -> Self {
        Self::StableKey(value)
    }
}

impl From<OrientedPointError> for WorldPointArchiveError {
    fn from(value: OrientedPointError) -> Self {
        Self::Kernel(value)
    }
}

impl From<WorldPointBindingError> for WorldPointArchiveError {
    fn from(value: WorldPointBindingError) -> Self {
        Self::Binding(value)
    }
}
