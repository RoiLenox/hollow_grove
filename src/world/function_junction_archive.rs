//! Versioned archive for Function Junction, House Seasons, synchronization,
//! and Permanence.
//!
//! `HGFJP` composes the frozen `HGSEA` payload. It does not migrate or rewrite
//! the seasonal, route, Tournament, or world-Point archives beneath it.

use std::collections::BTreeMap;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::constitutional::CanonicalYearId;
use crate::constitutional::{GroveCycleError, GroveCycleRecord, GroveCycleRuntime};

use super::function_junction::{
    FunctionJunctionError, FunctionJunctionRecord, FunctionJunctionRuntime, HouseSeasonRecord,
    PracticalJokeTransition,
};
use super::permanence::{
    PermanenceAttestation, PermanenceError, PermanencePetition, PermanenceRuntime, PermanenceSeal,
    PermanenceTombstone, PermanentChangeRecord,
};
use super::seasonal_functions_archive::{
    CanonicalAnnualCycleState, SeasonalArchiveError, SeasonalArchivePayload, replay_payload_records,
};

pub const FUNCTION_JUNCTION_ARCHIVE_FORMAT: &str = "HGFJP";
pub const FUNCTION_JUNCTION_ARCHIVE_VERSION: u16 = 1;
pub const FUNCTION_JUNCTION_LEGACY_ARCHIVE_VERSION: u16 = 0;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FunctionJunctionAnnualRecord {
    pub canonical_year_id: CanonicalYearId,
    pub seasonal_archive: SeasonalArchivePayload,
    pub junctions: Vec<FunctionJunctionRecord>,
    pub seasons: Vec<HouseSeasonRecord>,
    pub practical_jokes: Vec<PracticalJokeTransition>,
    pub grove_cycles: Vec<GroveCycleRecord>,
    pub permanence_attestations: Vec<PermanenceAttestation>,
    pub permanence_petitions: Vec<PermanencePetition>,
    pub permanence_seals: Vec<PermanenceSeal>,
    pub permanence_changes: Vec<PermanentChangeRecord>,
    pub permanence_tombstones: Vec<PermanenceTombstone>,
}

impl FunctionJunctionAnnualRecord {
    #[must_use]
    pub fn canonicalized(&self) -> Self {
        let mut record = self.clone();
        record.seasonal_archive = record.seasonal_archive.canonicalized();
        record
            .junctions
            .sort_by(|left, right| left.junction_id.cmp(&right.junction_id));
        record
            .seasons
            .sort_by(|left, right| left.season_id.cmp(&right.season_id));
        record
            .practical_jokes
            .sort_by(|left, right| left.joke_id.cmp(&right.joke_id));
        record
            .grove_cycles
            .sort_by(|left, right| left.cycle_id.cmp(&right.cycle_id));
        record
            .permanence_attestations
            .sort_by(|left, right| left.attestation_id.cmp(&right.attestation_id));
        record
            .permanence_petitions
            .sort_by(|left, right| left.petition_id.cmp(&right.petition_id));
        record
            .permanence_seals
            .sort_by(|left, right| left.seal_id.cmp(&right.seal_id));
        record
            .permanence_changes
            .sort_by(|left, right| left.change_id.cmp(&right.change_id));
        record
            .permanence_tombstones
            .sort_by(|left, right| left.tombstone_id.cmp(&right.tombstone_id));
        record
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FunctionJunctionArchivePayload {
    pub annual_records: Vec<FunctionJunctionAnnualRecord>,
}

impl FunctionJunctionArchivePayload {
    #[must_use]
    pub fn canonicalized(&self) -> Self {
        let mut payload = self.clone();
        payload.annual_records = payload
            .annual_records
            .iter()
            .map(FunctionJunctionAnnualRecord::canonicalized)
            .collect();
        payload
            .annual_records
            .sort_by(|left, right| left.canonical_year_id.cmp(&right.canonical_year_id));
        payload
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionJunctionAnnualState {
    pub canonical_year_id: CanonicalYearId,
    pub annual_cycle: CanonicalAnnualCycleState,
    pub function_junction_runtime: FunctionJunctionRuntime,
    pub grove_cycle_runtime: GroveCycleRuntime,
    pub permanence_runtime: PermanenceRuntime,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodedFunctionJunctionArchive {
    pub archive_version: u16,
    pub checksum: String,
    pub payload: FunctionJunctionArchivePayload,
    pub annual_states: BTreeMap<CanonicalYearId, FunctionJunctionAnnualState>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct FunctionJunctionArchiveEnvelope {
    format: String,
    archive_version: u16,
    checksum: String,
    payload: FunctionJunctionArchivePayload,
}

pub fn encode_function_junction_archive(
    payload: &FunctionJunctionArchivePayload,
) -> Result<Vec<u8>, FunctionJunctionArchiveError> {
    encode_with_version(payload, FUNCTION_JUNCTION_ARCHIVE_VERSION)
}

pub fn encode_legacy_function_junction_archive_v0(
    payload: &FunctionJunctionArchivePayload,
) -> Result<Vec<u8>, FunctionJunctionArchiveError> {
    let payload = payload.canonicalized();
    replay_function_junction_payload(&payload)?;
    let bytes = encode_unvalidated(&payload, FUNCTION_JUNCTION_LEGACY_ARCHIVE_VERSION)?;
    let text = String::from_utf8(bytes)
        .map_err(|error| FunctionJunctionArchiveError::Json(error.to_string()))?
        .replace("\"TheWayBack\"", "\"Return\"")
        .replace("\"TheInitiation\"", "\"Incarnate\"")
        .replace("\"TheGathering\"", "\"Commune\"")
        .replace("\"TheFestival\"", "\"Confirm\"");
    Ok(text.into_bytes())
}

fn encode_with_version(
    payload: &FunctionJunctionArchivePayload,
    version: u16,
) -> Result<Vec<u8>, FunctionJunctionArchiveError> {
    let payload = payload.canonicalized();
    replay_function_junction_payload(&payload)?;
    encode_unvalidated(&payload, version)
}

fn encode_unvalidated(
    payload: &FunctionJunctionArchivePayload,
    version: u16,
) -> Result<Vec<u8>, FunctionJunctionArchiveError> {
    let envelope = FunctionJunctionArchiveEnvelope {
        format: FUNCTION_JUNCTION_ARCHIVE_FORMAT.into(),
        archive_version: version,
        checksum: checksum(payload)?,
        payload: payload.clone(),
    };
    serde_json::to_vec(&envelope)
        .map_err(|error| FunctionJunctionArchiveError::Json(error.to_string()))
}

pub fn decode_function_junction_archive(
    bytes: &[u8],
) -> Result<DecodedFunctionJunctionArchive, FunctionJunctionArchiveError> {
    let envelope: FunctionJunctionArchiveEnvelope = serde_json::from_slice(bytes)
        .map_err(|error| FunctionJunctionArchiveError::Json(error.to_string()))?;
    if envelope.format != FUNCTION_JUNCTION_ARCHIVE_FORMAT {
        return Err(FunctionJunctionArchiveError::UnsupportedFormat(
            envelope.format,
        ));
    }
    if !matches!(
        envelope.archive_version,
        FUNCTION_JUNCTION_LEGACY_ARCHIVE_VERSION | FUNCTION_JUNCTION_ARCHIVE_VERSION
    ) {
        return Err(FunctionJunctionArchiveError::UnsupportedVersion(
            envelope.archive_version,
        ));
    }
    if envelope.checksum != checksum(&envelope.payload)? {
        return Err(FunctionJunctionArchiveError::ChecksumMismatch);
    }
    let payload = envelope.payload.canonicalized();
    let annual_states = replay_function_junction_payload(&payload)?;
    Ok(DecodedFunctionJunctionArchive {
        archive_version: envelope.archive_version,
        checksum: envelope.checksum,
        payload,
        annual_states,
    })
}

pub fn migrate_function_junction_archive(
    bytes: &[u8],
) -> Result<Vec<u8>, FunctionJunctionArchiveError> {
    let decoded = decode_function_junction_archive(bytes)?;
    encode_function_junction_archive(&decoded.payload)
}

pub fn replay_function_junction_payload(
    payload: &FunctionJunctionArchivePayload,
) -> Result<BTreeMap<CanonicalYearId, FunctionJunctionAnnualState>, FunctionJunctionArchiveError> {
    let payload = payload.canonicalized();
    if payload.annual_records.is_empty() {
        return Err(FunctionJunctionArchiveError::NoAnnualRecords);
    }
    let mut result = BTreeMap::new();
    for record in payload.annual_records {
        if result.contains_key(&record.canonical_year_id) {
            return Err(FunctionJunctionArchiveError::DuplicateAnnualRecord(
                record.canonical_year_id,
            ));
        }
        let seasonal_states = replay_payload_records(&record.seasonal_archive)?;
        if seasonal_states.len() != 1 {
            return Err(FunctionJunctionArchiveError::SeasonalArchiveScope(
                record.canonical_year_id,
            ));
        }
        let annual_cycle = seasonal_states
            .get(&record.canonical_year_id)
            .ok_or_else(|| {
                FunctionJunctionArchiveError::SeasonalArchiveScope(record.canonical_year_id.clone())
            })?
            .clone();
        let function_junction_runtime = FunctionJunctionRuntime::replay(
            &annual_cycle,
            &record.junctions,
            &record.seasons,
            &record.practical_jokes,
        )?;
        let grove_cycle_runtime = GroveCycleRuntime::replay(&record.grove_cycles)?;
        let permanence_runtime = PermanenceRuntime::replay(
            &record.canonical_year_id,
            &record.permanence_attestations,
            &record.permanence_petitions,
            &record.permanence_seals,
            &record.permanence_changes,
            &record.permanence_tombstones,
        )?;
        result.insert(
            record.canonical_year_id.clone(),
            FunctionJunctionAnnualState {
                canonical_year_id: record.canonical_year_id,
                annual_cycle,
                function_junction_runtime,
                grove_cycle_runtime,
                permanence_runtime,
            },
        );
    }
    Ok(result)
}

fn checksum(
    payload: &FunctionJunctionArchivePayload,
) -> Result<String, FunctionJunctionArchiveError> {
    let bytes = serde_json::to_vec(payload)
        .map_err(|error| FunctionJunctionArchiveError::Json(error.to_string()))?;
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for byte in bytes {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    Ok(format!("{hash:016x}"))
}

#[derive(Debug)]
pub enum FunctionJunctionArchiveError {
    Json(String),
    UnsupportedFormat(String),
    UnsupportedVersion(u16),
    ChecksumMismatch,
    NoAnnualRecords,
    DuplicateAnnualRecord(CanonicalYearId),
    SeasonalArchiveScope(CanonicalYearId),
    Seasonal(SeasonalArchiveError),
    Junction(FunctionJunctionError),
    GroveCycle(GroveCycleError),
    Permanence(PermanenceError),
}

impl fmt::Display for FunctionJunctionArchiveError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "Function Junction archive rejected state: {self:?}"
        )
    }
}

impl std::error::Error for FunctionJunctionArchiveError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Seasonal(error) => Some(error),
            Self::Junction(error) => Some(error),
            Self::GroveCycle(error) => Some(error),
            Self::Permanence(error) => Some(error),
            _ => None,
        }
    }
}

impl From<SeasonalArchiveError> for FunctionJunctionArchiveError {
    fn from(value: SeasonalArchiveError) -> Self {
        Self::Seasonal(value)
    }
}

impl From<FunctionJunctionError> for FunctionJunctionArchiveError {
    fn from(value: FunctionJunctionError) -> Self {
        Self::Junction(value)
    }
}

impl From<GroveCycleError> for FunctionJunctionArchiveError {
    fn from(value: GroveCycleError) -> Self {
        Self::GroveCycle(value)
    }
}

impl From<PermanenceError> for FunctionJunctionArchiveError {
    fn from(value: PermanenceError) -> Self {
        Self::Permanence(value)
    }
}
