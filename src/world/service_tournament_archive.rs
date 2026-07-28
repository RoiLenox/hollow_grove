//! Versioned, deterministic persistence and replay for Service Tournament years.
//!
//! The archive stores accepted constitutional inputs, then reconstructs state by
//! replaying the production Tournament and House Synthesis reducers. It never
//! trusts client presentation, vector position, insertion order, or random IDs
//! as constitutional identity.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::constitutional::{
    HouseSynthesisSemanticError, HouseSynthesisSemanticRuntime, SynthesisSemanticEvent,
};
use crate::hollow_grove_contract::House;

use super::service_tournament::{
    AllianceId, ArtifactId, ArtifactRefinementId, CompetitorId, ConstitutionalPenalty, EmergencyId,
    HouseColorFamily, PairedServiceIdentity, PrizeAwardId, ScenarioId, ScenarioPhaseId,
    ScoringCategory, ScoringEventId, ServiceMarkId, ServiceTournamentError,
    ServiceTournamentRuntime, TournamentAuthorityId, TournamentCompetitor, TournamentEvent,
    TournamentEvidenceId, TournamentId, TournamentLocationId, TournamentResult, TournamentYearId,
    ViolationId, WarId, canonical_service_tournament, canonical_war_of_a_thousand_hues,
    read_paint_layers,
};

pub const SERVICE_TOURNAMENT_ARCHIVE_FORMAT: &str = "HGSTA";
pub const SERVICE_TOURNAMENT_ARCHIVE_VERSION: u16 = 1;
pub const SERVICE_TOURNAMENT_LEGACY_ARCHIVE_VERSION: u16 = 0;
pub const CANONICAL_TOURNAMENT_YEAR_ID: &str = "service-tournament.canonical-year.v1";
pub const EDGE_OF_TOMORROW_ID: &str = "artifact.service-tournament.edge-of-tomorrow";
pub const GLASS_OF_A_THOUSAND_HUES_ID: &str =
    "artifact.service-tournament.glass-of-a-thousand-hues";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScenarioPhase {
    Briefed,
    Active,
    Overlapping,
    EmergencySuspended,
    Resolved,
    Closed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScenarioPhaseRecord {
    pub id: ScenarioPhaseId,
    pub tournament_year_id: TournamentYearId,
    pub scenario_id: ScenarioId,
    pub phase: ScenarioPhase,
    pub semantic_sequence: u64,
    pub evidence_references: BTreeSet<TournamentEvidenceId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemporaryAllianceRecord {
    pub id: AllianceId,
    pub tournament_year_id: TournamentYearId,
    pub houses: BTreeSet<House>,
    pub participant_ids: BTreeSet<CompetitorId>,
    pub scenario_ids: BTreeSet<ScenarioId>,
    pub temporary: bool,
    pub ends_with_tournament_year: bool,
    pub evidence_references: BTreeSet<TournamentEvidenceId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RealEmergencyRecord {
    pub id: EmergencyId,
    pub tournament_year_id: TournamentYearId,
    pub simulation_scenario_id: ScenarioId,
    pub response_scenario_id: ScenarioId,
    pub location_id: TournamentLocationId,
    pub initially_interpreted_as_simulation: bool,
    pub determined_real: bool,
    pub simulation_suspended: bool,
    pub recognized_by: House,
    pub distinction_authority: TournamentAuthorityId,
    pub evidence_references: BTreeSet<TournamentEvidenceId>,
    pub account: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScoringEventRecord {
    pub id: ScoringEventId,
    pub tournament_year_id: TournamentYearId,
    pub house: House,
    pub category: ScoringCategory,
    pub points_delta: i16,
    pub scenario_id: ScenarioId,
    pub evidence_references: BTreeSet<TournamentEvidenceId>,
    pub constitutional_restraint_decision: bool,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstitutionalViolationRecord {
    pub id: ViolationId,
    pub tournament_year_id: TournamentYearId,
    pub house: House,
    pub violation: ConstitutionalPenalty,
    pub scenario_id: ScenarioId,
    pub points_deduction: u16,
    pub invalidates_tactical_victory: bool,
    pub evidence_references: BTreeSet<TournamentEvidenceId>,
    pub account: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PrizeLineage {
    Current,
    Aura,
    StonebendTradition,
    SandmanorTradition,
    GlaushouseTradition,
    FlyntTradition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PrizeDegree {
    FlagshipArtifact,
    CompletedSynthesis,
    SynthesisRecipePattern,
    SynthesisCore,
    RefinedMaterial,
    RawMaterial,
    AccessAuthorityOrMentorship,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PrizeSubjectKind {
    FlagshipArtifact,
    CompletedSynthesis,
    SynthesisRecipePattern,
    SynthesisCore,
    RefinedMaterial,
    RawMaterial,
    AccessAuthorityOrMentorship,
}

impl PrizeDegree {
    #[must_use]
    pub const fn required_subject_kind(self) -> PrizeSubjectKind {
        match self {
            Self::FlagshipArtifact => PrizeSubjectKind::FlagshipArtifact,
            Self::CompletedSynthesis => PrizeSubjectKind::CompletedSynthesis,
            Self::SynthesisRecipePattern => PrizeSubjectKind::SynthesisRecipePattern,
            Self::SynthesisCore => PrizeSubjectKind::SynthesisCore,
            Self::RefinedMaterial => PrizeSubjectKind::RefinedMaterial,
            Self::RawMaterial => PrizeSubjectKind::RawMaterial,
            Self::AccessAuthorityOrMentorship => PrizeSubjectKind::AccessAuthorityOrMentorship,
        }
    }

    #[must_use]
    pub const fn is_completed_synthesis(self) -> bool {
        matches!(self, Self::FlagshipArtifact | Self::CompletedSynthesis)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FlagshipArtifactKind {
    EdgeOfTomorrow,
    GlassOfAThousandHues,
}

impl FlagshipArtifactKind {
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::EdgeOfTomorrow => "The Edge of Tomorrow",
            Self::GlassOfAThousandHues => "The Glass of a Thousand Hues",
        }
    }

    #[must_use]
    pub const fn stable_id(self) -> &'static str {
        match self {
            Self::EdgeOfTomorrow => EDGE_OF_TOMORROW_ID,
            Self::GlassOfAThousandHues => GLASS_OF_A_THOUSAND_HUES_ID,
        }
    }

    #[must_use]
    pub const fn lineage(self) -> PrizeLineage {
        match self {
            Self::EdgeOfTomorrow => PrizeLineage::Current,
            Self::GlassOfAThousandHues => PrizeLineage::Aura,
        }
    }

    #[must_use]
    pub const fn custody_title(self) -> &'static str {
        match self {
            Self::EdgeOfTomorrow => "Bearer of the Edge",
            Self::GlassOfAThousandHues => "Keeper of the Glass",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactRefinementRecord {
    pub id: ArtifactRefinementId,
    pub tournament_year_id: TournamentYearId,
    pub artifact_id: ArtifactId,
    pub artifact_kind: FlagshipArtifactKind,
    pub refinement_sequence: u64,
    pub previous_refinement_id: Option<ArtifactRefinementId>,
    pub contributing_house: House,
    pub contribution: String,
    pub lawful_recomposition: bool,
    pub replaces_artifact: bool,
    pub evidence_references: BTreeSet<TournamentEvidenceId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactCustodyRecord {
    pub artifact_id: ArtifactId,
    pub artifact_kind: FlagshipArtifactKind,
    pub custodian_id: CompetitorId,
    pub custodian_house: House,
    pub title: String,
    pub temporary_until_next_tournament: bool,
    pub custody_is_ownership: bool,
    pub grants_sovereignty: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrizeAwardRecord {
    pub id: PrizeAwardId,
    pub tournament_year_id: TournamentYearId,
    pub recipient_id: CompetitorId,
    pub recipient_house: House,
    pub lineage: PrizeLineage,
    pub degree: PrizeDegree,
    pub subject_kind: PrizeSubjectKind,
    pub subject_id: String,
    pub flagship_custody: Option<ArtifactCustodyRecord>,
    pub transfers_ownership: bool,
    pub grants_sovereignty: bool,
    pub evidence_references: BTreeSet<TournamentEvidenceId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WarOutcomeRecord {
    pub war_id: WarId,
    pub result_id: String,
    pub tactical_leader: House,
    pub elimination_counts: BTreeMap<House, u16>,
    pub constitutional_champion: House,
    pub nonlethal: bool,
    pub transfers_permanent_sovereignty: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TournamentYearRecord {
    pub id: TournamentYearId,
    pub calendar_year: u32,
    pub tournament_id: TournamentId,
    pub central_junction_function_id: String,
    pub representatives: Vec<TournamentCompetitor>,
    pub events: Vec<TournamentEvent>,
    pub scenario_phases: Vec<ScenarioPhaseRecord>,
    pub alliances: Vec<TemporaryAllianceRecord>,
    pub real_emergencies: Vec<RealEmergencyRecord>,
    pub scoring_events: Vec<ScoringEventRecord>,
    pub constitutional_violations: Vec<ConstitutionalViolationRecord>,
    pub artifact_refinements: Vec<ArtifactRefinementRecord>,
    pub prize_awards: Vec<PrizeAwardRecord>,
    pub synthesis_semantic_events: Vec<SynthesisSemanticEvent>,
    pub war_result: WarOutcomeRecord,
}

impl TournamentYearRecord {
    #[must_use]
    pub fn canonicalized(&self) -> Self {
        let mut record = self.clone();
        record
            .representatives
            .sort_by(|left, right| left.id.cmp(&right.id));
        record
            .events
            .sort_by_key(|event| (event.semantic_sequence, event.id.as_str().to_owned()));
        record
            .scenario_phases
            .sort_by_key(|phase| (phase.semantic_sequence, phase.id.as_str().to_owned()));
        record
            .alliances
            .sort_by(|left, right| left.id.cmp(&right.id));
        record
            .real_emergencies
            .sort_by(|left, right| left.id.cmp(&right.id));
        record
            .scoring_events
            .sort_by(|left, right| left.id.cmp(&right.id));
        record
            .constitutional_violations
            .sort_by(|left, right| left.id.cmp(&right.id));
        record.artifact_refinements.sort_by_key(|refinement| {
            (
                refinement.artifact_id.as_str().to_owned(),
                refinement.refinement_sequence,
                refinement.id.as_str().to_owned(),
            )
        });
        record
            .prize_awards
            .sort_by(|left, right| left.id.cmp(&right.id));
        record
            .synthesis_semantic_events
            .sort_by_key(|event| (event.semantic_sequence, event.id.as_str().to_owned()));
        record
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceTournamentArchivePayload {
    pub years: Vec<TournamentYearRecord>,
}

impl ServiceTournamentArchivePayload {
    #[must_use]
    pub fn canonicalized(&self) -> Self {
        let mut payload = self.clone();
        payload.years = payload
            .years
            .iter()
            .map(TournamentYearRecord::canonicalized)
            .collect();
        payload.years.sort_by(|left, right| left.id.cmp(&right.id));
        payload
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlagshipArtifactState {
    pub id: ArtifactId,
    pub kind: FlagshipArtifactKind,
    pub completed_synthesis: bool,
    pub singular: bool,
    pub evolving: bool,
    pub provenance_lineage_id: String,
    pub refinements: Vec<ArtifactRefinementId>,
    pub contributing_houses: BTreeSet<House>,
    pub custody: Option<ArtifactCustodyRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TournamentYearState {
    pub id: TournamentYearId,
    pub tournament_runtime: ServiceTournamentRuntime,
    pub synthesis_runtime: HouseSynthesisSemanticRuntime,
    pub scenario_phases: BTreeMap<ScenarioPhaseId, ScenarioPhaseRecord>,
    pub alliances: BTreeMap<AllianceId, TemporaryAllianceRecord>,
    pub real_emergencies: BTreeMap<EmergencyId, RealEmergencyRecord>,
    pub scoring_events: BTreeMap<ScoringEventId, ScoringEventRecord>,
    pub constitutional_violations: BTreeMap<ViolationId, ConstitutionalViolationRecord>,
    pub artifacts: BTreeMap<ArtifactId, FlagshipArtifactState>,
    pub prize_awards: BTreeMap<PrizeAwardId, PrizeAwardRecord>,
    pub war_result: WarOutcomeRecord,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodedServiceTournamentArchive {
    pub archive_version: u16,
    pub checksum: String,
    pub payload: ServiceTournamentArchivePayload,
    pub years: BTreeMap<TournamentYearId, TournamentYearState>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ArchiveEnvelope {
    format: String,
    archive_version: u16,
    checksum: String,
    payload: ServiceTournamentArchivePayload,
}

pub fn encode_service_tournament_archive(
    payload: &ServiceTournamentArchivePayload,
) -> Result<Vec<u8>, ServiceTournamentArchiveError> {
    encode_with_version(payload, SERVICE_TOURNAMENT_ARCHIVE_VERSION)
}

pub fn encode_legacy_service_tournament_archive_v0(
    payload: &ServiceTournamentArchivePayload,
) -> Result<Vec<u8>, ServiceTournamentArchiveError> {
    encode_with_version(payload, SERVICE_TOURNAMENT_LEGACY_ARCHIVE_VERSION)
}

fn encode_with_version(
    payload: &ServiceTournamentArchivePayload,
    archive_version: u16,
) -> Result<Vec<u8>, ServiceTournamentArchiveError> {
    let payload = payload.canonicalized();
    replay_payload(&payload)?;
    let envelope = ArchiveEnvelope {
        format: SERVICE_TOURNAMENT_ARCHIVE_FORMAT.into(),
        archive_version,
        checksum: checksum(&payload)?,
        payload,
    };
    serde_json::to_vec(&envelope)
        .map_err(|error| ServiceTournamentArchiveError::Json(error.to_string()))
}

pub fn decode_service_tournament_archive(
    bytes: &[u8],
) -> Result<DecodedServiceTournamentArchive, ServiceTournamentArchiveError> {
    let envelope: ArchiveEnvelope = serde_json::from_slice(bytes)
        .map_err(|error| ServiceTournamentArchiveError::Json(error.to_string()))?;
    if envelope.format != SERVICE_TOURNAMENT_ARCHIVE_FORMAT {
        return Err(ServiceTournamentArchiveError::UnsupportedFormat(
            envelope.format,
        ));
    }
    if !matches!(
        envelope.archive_version,
        SERVICE_TOURNAMENT_LEGACY_ARCHIVE_VERSION | SERVICE_TOURNAMENT_ARCHIVE_VERSION
    ) {
        return Err(ServiceTournamentArchiveError::UnsupportedVersion(
            envelope.archive_version,
        ));
    }
    if envelope.checksum != checksum(&envelope.payload)? {
        return Err(ServiceTournamentArchiveError::ChecksumMismatch);
    }
    let payload = envelope.payload.canonicalized();
    let years = replay_payload(&payload)?;
    Ok(DecodedServiceTournamentArchive {
        archive_version: envelope.archive_version,
        checksum: envelope.checksum,
        payload,
        years,
    })
}

pub fn migrate_service_tournament_archive(
    bytes: &[u8],
) -> Result<Vec<u8>, ServiceTournamentArchiveError> {
    let decoded = decode_service_tournament_archive(bytes)?;
    encode_service_tournament_archive(&decoded.payload)
}

pub fn replay_payload(
    payload: &ServiceTournamentArchivePayload,
) -> Result<BTreeMap<TournamentYearId, TournamentYearState>, ServiceTournamentArchiveError> {
    let payload = payload.canonicalized();
    if payload.years.is_empty() {
        return Err(ServiceTournamentArchiveError::NoTournamentYears);
    }
    let mut years = BTreeMap::new();
    for record in &payload.years {
        if years.contains_key(&record.id) {
            return Err(ServiceTournamentArchiveError::DuplicateTournamentYear(
                record.id.clone(),
            ));
        }
        let state = replay_year(record)?;
        years.insert(record.id.clone(), state);
    }
    Ok(years)
}

fn replay_year(
    record: &TournamentYearRecord,
) -> Result<TournamentYearState, ServiceTournamentArchiveError> {
    let tournament = canonical_service_tournament();
    let war = canonical_war_of_a_thousand_hues();
    if record.tournament_id != tournament.id
        || record.central_junction_function_id != tournament.function.stable_id()
        || record.war_result.war_id != war.id
        || !record.war_result.nonlethal
        || record.war_result.transfers_permanent_sovereignty
    {
        return Err(ServiceTournamentArchiveError::InvalidYearIdentity(
            record.id.clone(),
        ));
    }

    let tournament_runtime = ServiceTournamentRuntime::replay(&record.events)?;
    validate_representatives(record, &tournament_runtime)?;
    validate_phases(record, &tournament_runtime)?;
    validate_alliances(record, &tournament_runtime)?;
    validate_emergencies(record, &tournament_runtime)?;
    validate_scoring(record, &tournament_runtime)?;
    validate_service_marks(record, &tournament_runtime)?;
    let artifacts = replay_artifacts(record, &tournament_runtime)?;
    let synthesis_runtime =
        HouseSynthesisSemanticRuntime::replay(&record.synthesis_semantic_events)?;
    validate_result(record, &tournament_runtime)?;

    Ok(TournamentYearState {
        id: record.id.clone(),
        tournament_runtime,
        synthesis_runtime,
        scenario_phases: collect_unique(
            &record.scenario_phases,
            |value| value.id.clone(),
            ServiceTournamentArchiveError::DuplicateScenarioPhase,
        )?,
        alliances: collect_unique(
            &record.alliances,
            |value| value.id.clone(),
            ServiceTournamentArchiveError::DuplicateAlliance,
        )?,
        real_emergencies: collect_unique(
            &record.real_emergencies,
            |value| value.id.clone(),
            ServiceTournamentArchiveError::DuplicateEmergency,
        )?,
        scoring_events: collect_unique(
            &record.scoring_events,
            |value| value.id.clone(),
            ServiceTournamentArchiveError::DuplicateScoringEvent,
        )?,
        constitutional_violations: collect_unique(
            &record.constitutional_violations,
            |value| value.id.clone(),
            ServiceTournamentArchiveError::DuplicateViolation,
        )?,
        artifacts,
        prize_awards: collect_unique(
            &record.prize_awards,
            |value| value.id.clone(),
            ServiceTournamentArchiveError::DuplicatePrizeAward,
        )?,
        war_result: record.war_result.clone(),
    })
}

fn validate_representatives(
    record: &TournamentYearRecord,
    runtime: &ServiceTournamentRuntime,
) -> Result<(), ServiceTournamentArchiveError> {
    let expected = PairedServiceIdentity::ALL
        .into_iter()
        .map(|identity| (identity.house(), identity))
        .collect::<BTreeMap<_, _>>();
    let representatives = record
        .representatives
        .iter()
        .map(|representative| {
            (
                representative.house,
                (representative.id.clone(), representative.service_identity),
            )
        })
        .collect::<BTreeMap<_, _>>();
    if record.representatives.len() != 4
        || representatives.len() != 4
        || expected.iter().any(|(house, identity)| {
            representatives
                .get(house)
                .is_none_or(|(_, actual)| actual != identity)
        })
        || runtime.competitors().len() != 4
        || runtime.competitors().values().any(|competitor| {
            representatives
                .get(&competitor.house)
                .is_none_or(|(id, identity)| {
                    id != &competitor.id || identity != &competitor.service_identity
                })
        })
    {
        return Err(ServiceTournamentArchiveError::InvalidRepresentatives(
            record.id.clone(),
        ));
    }
    Ok(())
}

fn validate_phases(
    record: &TournamentYearRecord,
    runtime: &ServiceTournamentRuntime,
) -> Result<(), ServiceTournamentArchiveError> {
    let mut ids = BTreeSet::new();
    let mut last = BTreeMap::<ScenarioId, u64>::new();
    for phase in &record.scenario_phases {
        if phase.tournament_year_id != record.id
            || !ids.insert(phase.id.clone())
            || !runtime.scenarios().contains_key(&phase.scenario_id)
            || phase.evidence_references.is_empty()
            || last
                .insert(phase.scenario_id.clone(), phase.semantic_sequence)
                .is_some_and(|previous| phase.semantic_sequence <= previous)
        {
            return Err(ServiceTournamentArchiveError::InvalidScenarioPhase(
                phase.id.clone(),
            ));
        }
    }
    Ok(())
}

fn validate_alliances(
    record: &TournamentYearRecord,
    runtime: &ServiceTournamentRuntime,
) -> Result<(), ServiceTournamentArchiveError> {
    let mut ids = BTreeSet::new();
    for alliance in &record.alliances {
        let represented_houses = alliance
            .participant_ids
            .iter()
            .filter_map(|id| {
                runtime
                    .competitors()
                    .get(id)
                    .map(|competitor| competitor.house)
            })
            .collect::<BTreeSet<_>>();
        if alliance.tournament_year_id != record.id
            || !ids.insert(alliance.id.clone())
            || alliance.houses.len() < 2
            || represented_houses != alliance.houses
            || alliance
                .scenario_ids
                .iter()
                .any(|id| !runtime.scenarios().contains_key(id))
            || !alliance.temporary
            || !alliance.ends_with_tournament_year
            || alliance.evidence_references.is_empty()
        {
            return Err(ServiceTournamentArchiveError::InvalidAlliance(
                alliance.id.clone(),
            ));
        }
    }
    Ok(())
}

fn validate_emergencies(
    record: &TournamentYearRecord,
    runtime: &ServiceTournamentRuntime,
) -> Result<(), ServiceTournamentArchiveError> {
    let mut ids = BTreeSet::new();
    for emergency in &record.real_emergencies {
        if emergency.tournament_year_id != record.id
            || !ids.insert(emergency.id.clone())
            || !runtime
                .scenarios()
                .contains_key(&emergency.simulation_scenario_id)
            || !runtime
                .scenarios()
                .contains_key(&emergency.response_scenario_id)
            || !emergency.initially_interpreted_as_simulation
            || !emergency.determined_real
            || !emergency.simulation_suspended
            || emergency.evidence_references.is_empty()
            || emergency.account.trim().is_empty()
        {
            return Err(ServiceTournamentArchiveError::InvalidEmergency(
                emergency.id.clone(),
            ));
        }
    }
    Ok(())
}

fn validate_scoring(
    record: &TournamentYearRecord,
    runtime: &ServiceTournamentRuntime,
) -> Result<(), ServiceTournamentArchiveError> {
    let mut scoring_ids = BTreeSet::new();
    for score in &record.scoring_events {
        if score.tournament_year_id != record.id
            || !scoring_ids.insert(score.id.clone())
            || !runtime.scenarios().contains_key(&score.scenario_id)
            || score.evidence_references.is_empty()
            || score.reason.trim().is_empty()
            || (score.constitutional_restraint_decision
                && score.category != ScoringCategory::ConstitutionalRestraint)
        {
            return Err(ServiceTournamentArchiveError::InvalidScoringEvent(
                score.id.clone(),
            ));
        }
    }
    let mut violation_ids = BTreeSet::new();
    for violation in &record.constitutional_violations {
        if violation.tournament_year_id != record.id
            || !violation_ids.insert(violation.id.clone())
            || !runtime.scenarios().contains_key(&violation.scenario_id)
            || violation.points_deduction == 0
            || violation.points_deduction > i16::MAX as u16
            || violation.evidence_references.is_empty()
            || violation.account.trim().is_empty()
        {
            return Err(ServiceTournamentArchiveError::InvalidViolation(
                violation.id.clone(),
            ));
        }
    }
    Ok(())
}

fn validate_service_marks(
    record: &TournamentYearRecord,
    runtime: &ServiceTournamentRuntime,
) -> Result<(), ServiceTournamentArchiveError> {
    for service_mark in runtime.service_marks().values() {
        if service_mark.tournament_year_id != record.id
            || !runtime.scenarios().contains_key(&service_mark.scenario)
        {
            return Err(ServiceTournamentArchiveError::InvalidServiceMark(
                service_mark.id.clone(),
            ));
        }
        let marks = service_mark
            .ordered_paint_layers
            .iter()
            .map(|id| {
                runtime.marks().get(id).ok_or_else(|| {
                    ServiceTournamentArchiveError::InvalidServiceMark(service_mark.id.clone())
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let reading = read_paint_layers(&marks).ok_or_else(|| {
            ServiceTournamentArchiveError::InvalidServiceMark(service_mark.id.clone())
        })?;
        if reading.ordered_marks != service_mark.ordered_paint_layers
            || service_mark
                .ordered_paint_layers
                .iter()
                .cloned()
                .collect::<BTreeSet<_>>()
                != service_mark.provenance.source_marks
        {
            return Err(ServiceTournamentArchiveError::InvalidServiceMark(
                service_mark.id.clone(),
            ));
        }
    }
    Ok(())
}

fn replay_artifacts(
    record: &TournamentYearRecord,
    runtime: &ServiceTournamentRuntime,
) -> Result<BTreeMap<ArtifactId, FlagshipArtifactState>, ServiceTournamentArchiveError> {
    let mut artifacts = [
        canonical_artifact(FlagshipArtifactKind::EdgeOfTomorrow)?,
        canonical_artifact(FlagshipArtifactKind::GlassOfAThousandHues)?,
    ]
    .into_iter()
    .map(|artifact| (artifact.id.clone(), artifact))
    .collect::<BTreeMap<_, _>>();
    let mut refinement_ids = BTreeSet::new();
    for refinement in &record.artifact_refinements {
        let artifact = artifacts.get_mut(&refinement.artifact_id).ok_or_else(|| {
            ServiceTournamentArchiveError::InvalidArtifactRefinement(refinement.id.clone())
        })?;
        let expected_previous = artifact.refinements.last();
        if refinement.tournament_year_id != record.id
            || !refinement_ids.insert(refinement.id.clone())
            || artifact.kind != refinement.artifact_kind
            || refinement.refinement_sequence != artifact.refinements.len() as u64
            || refinement.previous_refinement_id.as_ref() != expected_previous
            || refinement.contribution.trim().is_empty()
            || !refinement.lawful_recomposition
            || refinement.replaces_artifact
            || refinement.evidence_references.is_empty()
        {
            return Err(ServiceTournamentArchiveError::InvalidArtifactRefinement(
                refinement.id.clone(),
            ));
        }
        artifact.refinements.push(refinement.id.clone());
        artifact
            .contributing_houses
            .insert(refinement.contributing_house);
    }

    let mut award_ids = BTreeSet::new();
    for award in &record.prize_awards {
        let recipient = runtime.competitors().get(&award.recipient_id);
        if award.tournament_year_id != record.id
            || !award_ids.insert(award.id.clone())
            || recipient.is_none_or(|competitor| competitor.house != award.recipient_house)
            || award.degree.required_subject_kind() != award.subject_kind
            || award.subject_id.trim().is_empty()
            || award.transfers_ownership
            || award.grants_sovereignty
            || award.evidence_references.is_empty()
        {
            return Err(ServiceTournamentArchiveError::InvalidPrizeAward(
                award.id.clone(),
            ));
        }
        if let Some(custody) = &award.flagship_custody {
            let artifact = artifacts.get_mut(&custody.artifact_id).ok_or_else(|| {
                ServiceTournamentArchiveError::InvalidPrizeAward(award.id.clone())
            })?;
            if award.degree != PrizeDegree::FlagshipArtifact
                || award.subject_kind != PrizeSubjectKind::FlagshipArtifact
                || award.lineage != artifact.kind.lineage()
                || award.subject_id != custody.artifact_id.as_str()
                || custody.artifact_kind != artifact.kind
                || custody.custodian_id != award.recipient_id
                || custody.custodian_house != award.recipient_house
                || custody.title != artifact.kind.custody_title()
                || !custody.temporary_until_next_tournament
                || custody.custody_is_ownership
                || custody.grants_sovereignty
            {
                return Err(ServiceTournamentArchiveError::InvalidPrizeAward(
                    award.id.clone(),
                ));
            }
            artifact.custody = Some(custody.clone());
        } else if award.degree == PrizeDegree::FlagshipArtifact {
            return Err(ServiceTournamentArchiveError::InvalidPrizeAward(
                award.id.clone(),
            ));
        }
    }
    Ok(artifacts)
}

fn canonical_artifact(
    kind: FlagshipArtifactKind,
) -> Result<FlagshipArtifactState, ServiceTournamentArchiveError> {
    let id = ArtifactId::new(kind.stable_id())
        .map_err(|_| ServiceTournamentArchiveError::InvalidCanonicalArtifact(kind))?;
    Ok(FlagshipArtifactState {
        id,
        kind,
        completed_synthesis: true,
        singular: true,
        evolving: true,
        provenance_lineage_id: format!("lineage.{}", kind.stable_id()),
        refinements: Vec::new(),
        contributing_houses: BTreeSet::new(),
        custody: None,
    })
}

fn validate_result(
    record: &TournamentYearRecord,
    runtime: &ServiceTournamentRuntime,
) -> Result<(), ServiceTournamentArchiveError> {
    let result = runtime
        .results()
        .values()
        .find(|result| result.id.as_str() == record.war_result.result_id)
        .ok_or_else(|| ServiceTournamentArchiveError::InvalidWarResult(record.id.clone()))?;
    if result.champion != record.war_result.constitutional_champion
        || result.transfers_permanent_sovereignty
        || record.war_result.elimination_counts.len() != 4
    {
        return Err(ServiceTournamentArchiveError::InvalidWarResult(
            record.id.clone(),
        ));
    }
    let invalidated_tactical_leader = record.constitutional_violations.iter().any(|violation| {
        violation.house == record.war_result.tactical_leader
            && violation.invalidates_tactical_victory
    });
    let violations_are_scored = record.constitutional_violations.iter().all(|violation| {
        result
            .scorecards
            .get(&violation.house)
            .is_some_and(|scorecard| scorecard.penalties.contains(&violation.violation))
            && record.scoring_events.iter().any(|score| {
                score.house == violation.house
                    && score.points_delta == -(violation.points_deduction as i16)
            })
    });
    let score_champion = result
        .scorecards
        .iter()
        .max_by_key(|(house, scorecard)| (scorecard.total_score(), std::cmp::Reverse(**house)))
        .map(|(house, _)| *house);
    let elimination_leader = record
        .war_result
        .elimination_counts
        .iter()
        .max_by_key(|(house, count)| (**count, std::cmp::Reverse(**house)))
        .map(|(house, _)| *house);
    if invalidated_tactical_leader
        && record.war_result.tactical_leader == record.war_result.constitutional_champion
        || !violations_are_scored
        || score_champion != Some(record.war_result.constitutional_champion)
        || elimination_leader != Some(record.war_result.tactical_leader)
    {
        return Err(ServiceTournamentArchiveError::InvalidWarResult(
            record.id.clone(),
        ));
    }
    Ok(())
}

fn collect_unique<K: Ord + Clone, V: Clone>(
    values: &[V],
    key: impl Fn(&V) -> K,
    duplicate: impl Fn(K) -> ServiceTournamentArchiveError,
) -> Result<BTreeMap<K, V>, ServiceTournamentArchiveError> {
    let mut result = BTreeMap::new();
    for value in values {
        let id = key(value);
        if result.insert(id.clone(), value.clone()).is_some() {
            return Err(duplicate(id));
        }
    }
    Ok(result)
}

fn checksum(
    payload: &ServiceTournamentArchivePayload,
) -> Result<String, ServiceTournamentArchiveError> {
    let bytes = serde_json::to_vec(payload)
        .map_err(|error| ServiceTournamentArchiveError::Json(error.to_string()))?;
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for byte in bytes {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    Ok(format!("{hash:016x}"))
}

#[derive(Debug)]
pub enum ServiceTournamentArchiveError {
    Json(String),
    UnsupportedFormat(String),
    UnsupportedVersion(u16),
    ChecksumMismatch,
    NoTournamentYears,
    DuplicateTournamentYear(TournamentYearId),
    InvalidYearIdentity(TournamentYearId),
    InvalidRepresentatives(TournamentYearId),
    DuplicateScenarioPhase(ScenarioPhaseId),
    InvalidScenarioPhase(ScenarioPhaseId),
    DuplicateAlliance(AllianceId),
    InvalidAlliance(AllianceId),
    DuplicateEmergency(EmergencyId),
    InvalidEmergency(EmergencyId),
    DuplicateScoringEvent(ScoringEventId),
    InvalidScoringEvent(ScoringEventId),
    DuplicateViolation(ViolationId),
    InvalidViolation(ViolationId),
    InvalidServiceMark(ServiceMarkId),
    InvalidCanonicalArtifact(FlagshipArtifactKind),
    InvalidArtifactRefinement(ArtifactRefinementId),
    DuplicatePrizeAward(PrizeAwardId),
    InvalidPrizeAward(PrizeAwardId),
    InvalidWarResult(TournamentYearId),
    Tournament(ServiceTournamentError),
    Synthesis(HouseSynthesisSemanticError),
}

impl fmt::Display for ServiceTournamentArchiveError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "Service Tournament archive rejected state: {self:?}"
        )
    }
}

impl std::error::Error for ServiceTournamentArchiveError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Tournament(error) => Some(error),
            Self::Synthesis(error) => Some(error),
            _ => None,
        }
    }
}

impl From<ServiceTournamentError> for ServiceTournamentArchiveError {
    fn from(value: ServiceTournamentError) -> Self {
        Self::Tournament(value)
    }
}

impl From<HouseSynthesisSemanticError> for ServiceTournamentArchiveError {
    fn from(value: HouseSynthesisSemanticError) -> Self {
        Self::Synthesis(value)
    }
}

#[must_use]
pub fn flagship_artifact(
    state: &TournamentYearState,
    kind: FlagshipArtifactKind,
) -> Option<&FlagshipArtifactState> {
    state
        .artifacts
        .values()
        .find(|artifact| artifact.kind == kind)
}

#[must_use]
pub fn final_result(state: &TournamentYearState) -> Option<&TournamentResult> {
    state
        .tournament_runtime
        .results()
        .values()
        .find(|result| result.id.as_str() == state.war_result.result_id)
}

#[must_use]
pub fn canonical_color_registry() -> BTreeMap<House, HouseColorFamily> {
    [
        House::Stonebend,
        House::Sandmanor,
        House::Glaushouse,
        House::Flynt,
    ]
    .into_iter()
    .map(|house| (house, HouseColorFamily::for_house(house)))
    .collect()
}
