//! Versioned archive for complete Central Junction canonical years.
//!
//! `HGSEA` composes the unchanged Service Tournament `HGSTA` payload beneath
//! the four Great Functions. The outer archive owns annual chronology and
//! nesting; Tournament history continues to replay through its own reducer.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::constitutional::{CanonicalCalendarError, CanonicalYearId};

use super::central_junction_seasonal_functions::{
    CentralJunctionSeasonalRuntime, FunctionActivity, GreatFunctionId, GreatFunctionKind,
    GreatFunctionRecord, SeasonalEventId, SeasonalFunctionError, SeasonalRecognitionId,
};
use super::current_sea_passage::{
    BoardwalkPassage, CurrentSeaEvent, CurrentSeaRuntime, CurrentSeaValidationError,
    FlyntRecogRecord, FlyntResynceRecord, GlaushouseClearanceRecord,
};
use super::service_tournament::{
    ArtifactId, ResultId, ServiceMarkId, TournamentId, TournamentYearId, WarId,
    canonical_service_tournament, canonical_war_of_a_thousand_hues,
};
use super::service_tournament_archive::{
    DecodedServiceTournamentArchive, FlagshipArtifactKind, ServiceTournamentArchiveError,
    ServiceTournamentArchivePayload, TournamentYearState, decode_service_tournament_archive,
    encode_service_tournament_archive, flagship_artifact, replay_payload,
};
use super::way_back::{
    WayBackPassage, WayBackRouteDefinition, WayBackRuntime, WayBackValidationError,
};
use super::world_point_archive::{
    WorldPointArchiveError, WorldPointArchivePayload, WorldPointReplayState,
    replay_world_point_payload,
};

pub const SEASONAL_ARCHIVE_FORMAT: &str = "HGSEA";
pub const SEASONAL_ARCHIVE_VERSION: u16 = 1;
pub const SEASONAL_LEGACY_ARCHIVE_VERSION: u16 = 0;
pub const CANONICAL_ANNUAL_CYCLE_ID: &str = "central-junction.canonical-year.2047.v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SeasonalNestingRecord {
    pub gathering_function_id: GreatFunctionId,
    pub service_tournament_id: TournamentId,
    pub service_tournament_year_id: TournamentYearId,
    pub service_tournament_event_id: SeasonalEventId,
    pub war_id: WarId,
    pub tournament_result_id: ResultId,
    pub tournament_archive_checksum: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SeasonalRecognitionSubject {
    ServiceMark(ServiceMarkId),
    EdgeOfTomorrow(ArtifactId),
    GlassOfAThousandHues(ArtifactId),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SeasonalRecognitionRecord {
    pub id: SeasonalRecognitionId,
    pub canonical_year_id: CanonicalYearId,
    pub festival_function_id: GreatFunctionId,
    pub event_id: SeasonalEventId,
    pub subject: SeasonalRecognitionSubject,
    pub evidence_ids: BTreeSet<crate::constitutional::CalendarEvidenceId>,
    pub grants_permanent_sovereignty: bool,
    pub account: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalAnnualCycleRecord {
    pub id: CanonicalYearId,
    pub calendar: crate::constitutional::CanonicalYearRecord,
    pub functions: Vec<GreatFunctionRecord>,
    pub way_back_route: WayBackRouteDefinition,
    pub way_back_passages: Vec<WayBackPassage>,
    pub current_sea_events: Vec<CurrentSeaEvent>,
    pub glaushouse_clearances: Vec<GlaushouseClearanceRecord>,
    pub boardwalk_passages: Vec<BoardwalkPassage>,
    pub flynt_resynce_events: Vec<FlyntResynceRecord>,
    pub flynt_recog_events: Vec<FlyntRecogRecord>,
    pub world_point_archive: WorldPointArchivePayload,
    pub nesting: SeasonalNestingRecord,
    pub recognitions: Vec<SeasonalRecognitionRecord>,
    pub tournament_archive: ServiceTournamentArchivePayload,
}

impl CanonicalAnnualCycleRecord {
    #[must_use]
    pub fn canonicalized(&self) -> Self {
        let mut record = self.clone();
        record.calendar = record.calendar.canonicalized();
        record.functions = record
            .functions
            .iter()
            .map(GreatFunctionRecord::canonicalized)
            .collect();
        record
            .functions
            .sort_by_key(|function| (function.anchor, function.function_id.as_str().to_owned()));
        record.way_back_passages = record
            .way_back_passages
            .iter()
            .map(WayBackPassage::canonicalized)
            .collect();
        record
            .way_back_passages
            .sort_by(|left, right| left.passage_id.cmp(&right.passage_id));
        record
            .current_sea_events
            .sort_by(|left, right| left.event_id.cmp(&right.event_id));
        record
            .glaushouse_clearances
            .sort_by(|left, right| left.clearance_id.cmp(&right.clearance_id));
        record
            .boardwalk_passages
            .sort_by(|left, right| left.passage_id.cmp(&right.passage_id));
        record
            .flynt_resynce_events
            .sort_by(|left, right| left.event_id.cmp(&right.event_id));
        record
            .flynt_recog_events
            .sort_by(|left, right| left.event_id.cmp(&right.event_id));
        record.world_point_archive = record.world_point_archive.canonicalized();
        record
            .recognitions
            .sort_by(|left, right| left.id.cmp(&right.id));
        record.tournament_archive = record.tournament_archive.canonicalized();
        record
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SeasonalArchivePayload {
    pub annual_cycles: Vec<CanonicalAnnualCycleRecord>,
}

impl SeasonalArchivePayload {
    #[must_use]
    pub fn canonicalized(&self) -> Self {
        let mut payload = self.clone();
        payload.annual_cycles = payload
            .annual_cycles
            .iter()
            .map(CanonicalAnnualCycleRecord::canonicalized)
            .collect();
        payload
            .annual_cycles
            .sort_by(|left, right| left.id.cmp(&right.id));
        payload
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalAnnualCycleState {
    pub id: CanonicalYearId,
    pub seasonal_runtime: CentralJunctionSeasonalRuntime,
    pub way_back_runtime: WayBackRuntime,
    pub current_sea_runtime: CurrentSeaRuntime,
    pub world_point_state: WorldPointReplayState,
    pub tournament_years: BTreeMap<TournamentYearId, TournamentYearState>,
    pub nesting: SeasonalNestingRecord,
    pub recognitions: BTreeMap<SeasonalRecognitionId, SeasonalRecognitionRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodedSeasonalArchive {
    pub archive_version: u16,
    pub checksum: String,
    pub payload: SeasonalArchivePayload,
    pub annual_cycles: BTreeMap<CanonicalYearId, CanonicalAnnualCycleState>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct SeasonalArchiveEnvelope {
    format: String,
    archive_version: u16,
    checksum: String,
    payload: SeasonalArchivePayload,
}

pub fn encode_seasonal_archive(
    payload: &SeasonalArchivePayload,
) -> Result<Vec<u8>, SeasonalArchiveError> {
    encode_with_version(payload, SEASONAL_ARCHIVE_VERSION)
}

pub fn encode_legacy_seasonal_archive_v0(
    payload: &SeasonalArchivePayload,
) -> Result<Vec<u8>, SeasonalArchiveError> {
    let mut legacy = payload.canonicalized();
    apply_legacy_working_names(&mut legacy);
    encode_unvalidated(&legacy, SEASONAL_LEGACY_ARCHIVE_VERSION)
}

fn encode_with_version(
    payload: &SeasonalArchivePayload,
    version: u16,
) -> Result<Vec<u8>, SeasonalArchiveError> {
    let payload = payload.canonicalized();
    replay_payload_records(&payload)?;
    encode_unvalidated(&payload, version)
}

fn encode_unvalidated(
    payload: &SeasonalArchivePayload,
    version: u16,
) -> Result<Vec<u8>, SeasonalArchiveError> {
    let envelope = SeasonalArchiveEnvelope {
        format: SEASONAL_ARCHIVE_FORMAT.into(),
        archive_version: version,
        checksum: checksum(payload)?,
        payload: payload.clone(),
    };
    serde_json::to_vec(&envelope).map_err(|error| SeasonalArchiveError::Json(error.to_string()))
}

pub fn decode_seasonal_archive(
    bytes: &[u8],
) -> Result<DecodedSeasonalArchive, SeasonalArchiveError> {
    let envelope: SeasonalArchiveEnvelope = serde_json::from_slice(bytes)
        .map_err(|error| SeasonalArchiveError::Json(error.to_string()))?;
    if envelope.format != SEASONAL_ARCHIVE_FORMAT {
        return Err(SeasonalArchiveError::UnsupportedFormat(envelope.format));
    }
    if !matches!(
        envelope.archive_version,
        SEASONAL_LEGACY_ARCHIVE_VERSION | SEASONAL_ARCHIVE_VERSION
    ) {
        return Err(SeasonalArchiveError::UnsupportedVersion(
            envelope.archive_version,
        ));
    }
    if envelope.checksum != checksum(&envelope.payload)? {
        return Err(SeasonalArchiveError::ChecksumMismatch);
    }
    let mut payload = envelope.payload;
    if envelope.archive_version == SEASONAL_LEGACY_ARCHIVE_VERSION {
        migrate_working_names(&mut payload);
    }
    let payload = payload.canonicalized();
    let annual_cycles = replay_payload_records(&payload)?;
    Ok(DecodedSeasonalArchive {
        archive_version: envelope.archive_version,
        checksum: envelope.checksum,
        payload,
        annual_cycles,
    })
}

pub fn migrate_seasonal_archive(bytes: &[u8]) -> Result<Vec<u8>, SeasonalArchiveError> {
    let decoded = decode_seasonal_archive(bytes)?;
    encode_seasonal_archive(&decoded.payload)
}

pub fn replay_payload_records(
    payload: &SeasonalArchivePayload,
) -> Result<BTreeMap<CanonicalYearId, CanonicalAnnualCycleState>, SeasonalArchiveError> {
    let payload = payload.canonicalized();
    if payload.annual_cycles.is_empty() {
        return Err(SeasonalArchiveError::NoAnnualCycles);
    }
    let mut states = BTreeMap::new();
    for cycle in payload.annual_cycles {
        if states.contains_key(&cycle.id) {
            return Err(SeasonalArchiveError::DuplicateAnnualCycle(cycle.id));
        }
        let state = replay_annual_cycle(&cycle)?;
        states.insert(cycle.id.clone(), state);
    }
    Ok(states)
}

fn replay_annual_cycle(
    cycle: &CanonicalAnnualCycleRecord,
) -> Result<CanonicalAnnualCycleState, SeasonalArchiveError> {
    if cycle.id != cycle.calendar.id {
        return Err(SeasonalArchiveError::YearIdentityMismatch(cycle.id.clone()));
    }
    let seasonal_runtime =
        CentralJunctionSeasonalRuntime::replay(cycle.calendar.clone(), &cycle.functions)?;
    validate_passage_year_references(cycle, &seasonal_runtime)?;
    let way_back_runtime =
        WayBackRuntime::replay(cycle.way_back_route.clone(), &cycle.way_back_passages)?;
    let current_sea_runtime = CurrentSeaRuntime::replay(
        &cycle.current_sea_events,
        &cycle.glaushouse_clearances,
        &cycle.boardwalk_passages,
        &cycle.flynt_resynce_events,
        &cycle.flynt_recog_events,
    )?;
    let world_point_state = replay_world_point_payload(&cycle.world_point_archive)?;
    let tournament_years = replay_payload(&cycle.tournament_archive)?;
    validate_nesting(cycle, &seasonal_runtime, &tournament_years)?;
    let recognitions = validate_recognitions(cycle, &seasonal_runtime, &tournament_years)?;
    Ok(CanonicalAnnualCycleState {
        id: cycle.id.clone(),
        seasonal_runtime,
        way_back_runtime,
        current_sea_runtime,
        world_point_state,
        tournament_years,
        nesting: cycle.nesting.clone(),
        recognitions,
    })
}

fn validate_passage_year_references(
    cycle: &CanonicalAnnualCycleRecord,
    seasonal_runtime: &CentralJunctionSeasonalRuntime,
) -> Result<(), SeasonalArchiveError> {
    let winter = seasonal_runtime
        .function_at_anchor(crate::constitutional::SeasonalAnchor::WinterSolstice)
        .ok_or_else(|| SeasonalArchiveError::InvalidPassageCycle(cycle.id.clone()))?;
    let world_subjects = cycle
        .world_point_archive
        .relationships
        .iter()
        .map(|relationship| relationship.subject_id.as_str())
        .collect::<BTreeSet<_>>();
    if seasonal_runtime.functions().values().any(|function| {
        function
            .venue_ids
            .iter()
            .any(|venue_id| !world_subjects.contains(venue_id.as_str()))
    }) {
        return Err(SeasonalArchiveError::InvalidPassageCycle(cycle.id.clone()));
    }
    for passage in &cycle.way_back_passages {
        if passage
            .canonical_year_id
            .as_ref()
            .is_some_and(|year_id| year_id != &cycle.id)
            || passage
                .function_id
                .as_ref()
                .is_some_and(|function_id| function_id != &winter.function_id)
        {
            return Err(SeasonalArchiveError::InvalidPassageCycle(cycle.id.clone()));
        }
    }
    if cycle.current_sea_events.iter().any(|event| {
        event
            .canonical_year_id
            .as_ref()
            .is_some_and(|year_id| year_id != &cycle.id)
    }) || cycle.boardwalk_passages.iter().any(|passage| {
        passage
            .canonical_year_id
            .as_ref()
            .is_some_and(|year_id| year_id != &cycle.id)
    }) {
        return Err(SeasonalArchiveError::InvalidPassageCycle(cycle.id.clone()));
    }
    Ok(())
}

fn validate_nesting(
    cycle: &CanonicalAnnualCycleRecord,
    seasonal_runtime: &CentralJunctionSeasonalRuntime,
    tournament_years: &BTreeMap<TournamentYearId, TournamentYearState>,
) -> Result<(), SeasonalArchiveError> {
    let gathering = seasonal_runtime
        .function_at_anchor(crate::constitutional::SeasonalAnchor::SummerSolstice)
        .ok_or_else(|| SeasonalArchiveError::InvalidNesting(cycle.id.clone()))?;
    let tournament = canonical_service_tournament();
    let war = canonical_war_of_a_thousand_hues();
    let tournament_year = tournament_years
        .get(&cycle.nesting.service_tournament_year_id)
        .ok_or_else(|| SeasonalArchiveError::InvalidNesting(cycle.id.clone()))?;
    let tournament_bytes = encode_service_tournament_archive(&cycle.tournament_archive)?;
    let decoded_tournament: DecodedServiceTournamentArchive =
        decode_service_tournament_archive(&tournament_bytes)?;
    let result_exists = tournament_year
        .tournament_runtime
        .results()
        .keys()
        .any(|id| id == &cycle.nesting.tournament_result_id);
    if gathering.function_id != cycle.nesting.gathering_function_id
        || gathering.kind != GreatFunctionKind::Gathering
        || !gathering
            .activities
            .contains(&FunctionActivity::ServiceTournament)
        || !gathering
            .event_ids
            .contains(&cycle.nesting.service_tournament_event_id)
        || gathering.activities.len() <= 1
        || cycle.nesting.service_tournament_id != tournament.id
        || cycle.nesting.war_id != war.id
        || !war.nonlethal
        || tournament_year.tournament_runtime.war().id != war.id
        || !result_exists
        || cycle.nesting.tournament_archive_checksum != decoded_tournament.checksum
    {
        return Err(SeasonalArchiveError::InvalidNesting(cycle.id.clone()));
    }
    Ok(())
}

fn validate_recognitions(
    cycle: &CanonicalAnnualCycleRecord,
    seasonal_runtime: &CentralJunctionSeasonalRuntime,
    tournament_years: &BTreeMap<TournamentYearId, TournamentYearState>,
) -> Result<BTreeMap<SeasonalRecognitionId, SeasonalRecognitionRecord>, SeasonalArchiveError> {
    let festival = seasonal_runtime
        .function_at_anchor(crate::constitutional::SeasonalAnchor::AutumnEquinox)
        .ok_or_else(|| SeasonalArchiveError::InvalidRecognitionCycle(cycle.id.clone()))?;
    let tournament_year = tournament_years
        .get(&cycle.nesting.service_tournament_year_id)
        .ok_or_else(|| SeasonalArchiveError::InvalidRecognitionCycle(cycle.id.clone()))?;
    let mut result = BTreeMap::new();
    let mut subject_kinds = BTreeSet::new();
    for recognition in &cycle.recognitions {
        let subject_valid = match &recognition.subject {
            SeasonalRecognitionSubject::ServiceMark(id) => {
                subject_kinds.insert(0_u8);
                tournament_year
                    .tournament_runtime
                    .service_marks()
                    .contains_key(id)
            }
            SeasonalRecognitionSubject::EdgeOfTomorrow(id) => {
                subject_kinds.insert(1_u8);
                flagship_artifact(tournament_year, FlagshipArtifactKind::EdgeOfTomorrow)
                    .is_some_and(|artifact| &artifact.id == id)
            }
            SeasonalRecognitionSubject::GlassOfAThousandHues(id) => {
                subject_kinds.insert(2_u8);
                flagship_artifact(tournament_year, FlagshipArtifactKind::GlassOfAThousandHues)
                    .is_some_and(|artifact| &artifact.id == id)
            }
        };
        if recognition.canonical_year_id != cycle.id
            || recognition.festival_function_id != festival.function_id
            || !festival.event_ids.contains(&recognition.event_id)
            || recognition.evidence_ids.is_empty()
            || recognition.grants_permanent_sovereignty
            || recognition.account.trim().is_empty()
            || !subject_valid
            || result
                .insert(recognition.id.clone(), recognition.clone())
                .is_some()
        {
            return Err(SeasonalArchiveError::InvalidRecognition(
                recognition.id.clone(),
            ));
        }
    }
    if subject_kinds != [0_u8, 1, 2].into_iter().collect() {
        return Err(SeasonalArchiveError::IncompleteRecognitionCycle(
            cycle.id.clone(),
        ));
    }
    Ok(result)
}

fn apply_legacy_working_names(payload: &mut SeasonalArchivePayload) {
    for cycle in &mut payload.annual_cycles {
        for function in &mut cycle.functions {
            function.canonical_name = match function.kind {
                GreatFunctionKind::WayBack => "The Long Return",
                GreatFunctionKind::Initiation => "The First Naming",
                GreatFunctionKind::Gathering => "The Great Gathering",
                GreatFunctionKind::FestivalOfMirrors => "The Great Recognition",
            }
            .into();
        }
    }
}

fn migrate_working_names(payload: &mut SeasonalArchivePayload) {
    for cycle in &mut payload.annual_cycles {
        for function in &mut cycle.functions {
            function.canonical_name = match function.canonical_name.as_str() {
                "The Long Return" => "The Way Back",
                "The First Naming" => "The Initiation",
                "The Great Gathering" => "The Gathering",
                "The Great Recognition" => "The Festival of Mirrors",
                current => current,
            }
            .into();
            if function.kind == GreatFunctionKind::Gathering {
                function.aliases.insert("Derrick".into());
            }
        }
    }
}

fn checksum(payload: &SeasonalArchivePayload) -> Result<String, SeasonalArchiveError> {
    let bytes = serde_json::to_vec(payload)
        .map_err(|error| SeasonalArchiveError::Json(error.to_string()))?;
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for byte in bytes {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    Ok(format!("{hash:016x}"))
}

#[derive(Debug)]
pub enum SeasonalArchiveError {
    Json(String),
    UnsupportedFormat(String),
    UnsupportedVersion(u16),
    ChecksumMismatch,
    NoAnnualCycles,
    DuplicateAnnualCycle(CanonicalYearId),
    YearIdentityMismatch(CanonicalYearId),
    InvalidNesting(CanonicalYearId),
    InvalidRecognitionCycle(CanonicalYearId),
    InvalidRecognition(SeasonalRecognitionId),
    IncompleteRecognitionCycle(CanonicalYearId),
    InvalidPassageCycle(CanonicalYearId),
    Calendar(CanonicalCalendarError),
    Seasonal(SeasonalFunctionError),
    Tournament(ServiceTournamentArchiveError),
    WayBack(WayBackValidationError),
    CurrentSea(CurrentSeaValidationError),
    WorldPoint(WorldPointArchiveError),
}

impl fmt::Display for SeasonalArchiveError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "seasonal archive rejected state: {self:?}")
    }
}

impl std::error::Error for SeasonalArchiveError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Calendar(error) => Some(error),
            Self::Seasonal(error) => Some(error),
            Self::Tournament(error) => Some(error),
            Self::WayBack(error) => Some(error),
            Self::CurrentSea(error) => Some(error),
            Self::WorldPoint(error) => Some(error),
            _ => None,
        }
    }
}

impl From<CanonicalCalendarError> for SeasonalArchiveError {
    fn from(value: CanonicalCalendarError) -> Self {
        Self::Calendar(value)
    }
}

impl From<SeasonalFunctionError> for SeasonalArchiveError {
    fn from(value: SeasonalFunctionError) -> Self {
        Self::Seasonal(value)
    }
}

impl From<ServiceTournamentArchiveError> for SeasonalArchiveError {
    fn from(value: ServiceTournamentArchiveError) -> Self {
        Self::Tournament(value)
    }
}

impl From<WayBackValidationError> for SeasonalArchiveError {
    fn from(value: WayBackValidationError) -> Self {
        Self::WayBack(value)
    }
}

impl From<CurrentSeaValidationError> for SeasonalArchiveError {
    fn from(value: CurrentSeaValidationError) -> Self {
        Self::CurrentSea(value)
    }
}

impl From<WorldPointArchiveError> for SeasonalArchiveError {
    fn from(value: WorldPointArchiveError) -> Self {
        Self::WorldPoint(value)
    }
}
