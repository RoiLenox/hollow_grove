use serde::{Deserialize, Serialize};

use crate::ai_personas::{
    PersonaAdvisoryRecord, SharedInterfaceRecord, encode_advisory, replay_advisory,
};
use crate::constitutional::{
    AuthorityActorId, BondId, CausalPosition, HouseDecisionId, ParticipantId, RuleSetId,
};
use crate::institution::InstitutionalBeingId;
use crate::world::session::WorldSession;

use super::{
    ActiveIncarnationRef, BeingContinuityId, BeingContinuityRecord, BoardwalkChoice,
    BoardwalkEvidence, BoardwalkOutcomeId, DeepPressureEvent, GameRevision, GameplayEvent,
    GameplayEventEnvelope, GameplayEventId, HuemanFaculty, InteractionId, LivingWorldEvent,
    PartyEvent, StonebendContinuityChoice, StonebendEvidence, StonebendOutcomeId, SurveyFrameEvent,
    SympioteFoundationEvent, TilePosition, WingedGargoylePilotEvent, WorldMapId,
};

pub const GAMEPLAY_ARCHIVE_FORMAT: &str = "hollow-grove-gameplay";
pub const GAMEPLAY_ARCHIVE_SCHEMA_VERSION: u16 = 6;
const PRE_SURVEY_FRAME_GAMEPLAY_ARCHIVE_SCHEMA_VERSION: u16 = 5;
const PRE_SYMPIOTE_GAMEPLAY_ARCHIVE_SCHEMA_VERSION: u16 = 4;
pub const GAMEPLAY_RAMPART_COMPONENT_ID: &str = "component.gameplay.authoritative";
#[deprecated(since = "0.1.0", note = "use GAMEPLAY_RAMPART_COMPONENT_ID")]
pub const GAMEPLAY_FEDERATION_COMPONENT_ID: &str = GAMEPLAY_RAMPART_COMPONENT_ID;
const PRE_SYNTHESIS_GAMEPLAY_ARCHIVE_SCHEMA_VERSION: u16 = 3;
const PRE_RAMPART_GAMEPLAY_ARCHIVE_SCHEMA_VERSION: u16 = 2;
const LEGACY_GAMEPLAY_ARCHIVE_SCHEMA_VERSION: u16 = 1;

/// Reports whether the gameplay decoder has an explicit migration path for a
/// schema version. Rampart component validation uses the same boundary as
/// the standalone archive decoder instead of assuming only the latest schema
/// can be replayed.
#[must_use]
pub const fn is_supported_gameplay_archive_schema_version(schema_version: u16) -> bool {
    matches!(
        schema_version,
        GAMEPLAY_ARCHIVE_SCHEMA_VERSION
            | PRE_SURVEY_FRAME_GAMEPLAY_ARCHIVE_SCHEMA_VERSION
            | PRE_SYMPIOTE_GAMEPLAY_ARCHIVE_SCHEMA_VERSION
            | PRE_SYNTHESIS_GAMEPLAY_ARCHIVE_SCHEMA_VERSION
            | PRE_RAMPART_GAMEPLAY_ARCHIVE_SCHEMA_VERSION
            | LEGACY_GAMEPLAY_ARCHIVE_SCHEMA_VERSION
    )
}
// Schema V1 formed Boardwalk Bonds against these test-era authority actors.
// Migration preserves that historical dependency explicitly so replay is
// exact; new sessions never load these actors and use live schema-V2 state.
const LEGACY_GAMEPLAY_AUTHORITY_MIGRATION: &str = "schema_version:2\n\
office-holder\toffice.stonebend.hypergiant\tbeing.stonebend.fixture-member\ttrue\n\
office-holder\toffice.sandmanor.sandman\tbeing.sandmanor.fixture-member\ttrue\n\
office-holder\toffice.glaushouse.prima-donna\tbeing.glaushouse.fixture-member\ttrue\n";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GameplayRampartBinding {
    #[serde(rename = "federation_identity")]
    pub rampart_identity: String,
    #[serde(rename = "federation_archive_format")]
    pub rampart_archive_format: String,
    #[serde(rename = "federation_archive_version")]
    pub rampart_archive_version: u16,
    pub component_id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub advisory_records: Vec<PersonaAdvisoryRecord>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub shared_interface_records: Vec<SharedInterfaceRecord>,
}

#[deprecated(since = "0.1.0", note = "use GameplayRampartBinding")]
pub type GameplayFederationBinding = GameplayRampartBinding;

impl GameplayRampartBinding {
    #[must_use]
    pub fn canonical(component_id: impl Into<String>) -> Self {
        Self {
            rampart_identity: crate::runtime_rampart::RUNTIME_RAMPART_IDENTITY.into(),
            rampart_archive_format: crate::runtime_rampart::RUNTIME_RAMPART_ARCHIVE_FORMAT.into(),
            rampart_archive_version: crate::runtime_rampart::RUNTIME_RAMPART_ARCHIVE_VERSION,
            component_id: component_id.into(),
            advisory_records: Vec::new(),
            shared_interface_records: Vec::new(),
        }
    }

    #[must_use]
    pub fn with_advisory(mut self, record: PersonaAdvisoryRecord) -> Self {
        self.advisory_records.push(record);
        self
    }

    #[must_use]
    pub fn with_shared_interface_record(mut self, record: SharedInterfaceRecord) -> Self {
        self.shared_interface_records.push(record);
        self
    }

    fn validate(&self) -> Result<(), GameplayArchiveError> {
        crate::kernel_invariants::audit_gameplay_rampart_binding(self)
            .ensure_passed()
            .map_err(|error| GameplayArchiveError::InvalidDomainValue(error.to_string()))?;
        if self.rampart_identity != crate::runtime_rampart::RUNTIME_RAMPART_IDENTITY
            && self.rampart_identity != crate::runtime_rampart::LEGACY_RUNTIME_FEDERATION_IDENTITY
        {
            return Err(GameplayArchiveError::InvalidRampartBinding);
        }
        if self.rampart_archive_format != crate::runtime_rampart::RUNTIME_RAMPART_ARCHIVE_FORMAT
            || self.rampart_archive_version
                != crate::runtime_rampart::RUNTIME_RAMPART_ARCHIVE_VERSION
            || self.component_id.is_empty()
            || self.component_id.chars().any(char::is_whitespace)
        {
            return Err(GameplayArchiveError::InvalidRampartBinding);
        }
        for record in &self.advisory_records {
            let bytes = encode_advisory(record)
                .map_err(|error| GameplayArchiveError::InvalidDomainValue(error.to_string()))?;
            replay_advisory(&bytes)
                .map_err(|error| GameplayArchiveError::InvalidDomainValue(error.to_string()))?;
        }
        for record in &self.shared_interface_records {
            record
                .validate()
                .map_err(|error| GameplayArchiveError::InvalidDomainValue(error.to_string()))?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GameplayArchiveMigrationRecord {
    pub migration_id: String,
    pub from_schema_version: u16,
    pub to_schema_version: u16,
    pub decision: String,
    pub provenance_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodedGameplayArchive {
    pub source_schema_version: u16,
    pub rule_set: RuleSetId,
    pub events: Vec<GameplayEventEnvelope>,
    pub institutional_state: Option<String>,
    pub rampart_binding: GameplayRampartBinding,
    pub migration_history: Vec<GameplayArchiveMigrationRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct GameplayArchiveEnvelopeV3 {
    format: String,
    schema_version: u16,
    checksum: String,
    payload: GameplayArchivePayloadV3,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct GameplayArchiveEnvelopeV4 {
    format: String,
    schema_version: u16,
    checksum: String,
    payload: GameplayArchivePayloadV4,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct GameplayArchivePayloadV4 {
    rule_set: String,
    events: Vec<WireGameplayEventEnvelope>,
    institutional_state: String,
    #[serde(rename = "federation_binding")]
    rampart_binding: GameplayRampartBinding,
    migration_history: Vec<GameplayArchiveMigrationRecord>,
}

// V5 adds an event variant; envelope fields and checksum input remain unchanged.
type GameplayArchiveEnvelopeV5 = GameplayArchiveEnvelopeV4;
type GameplayArchivePayloadV5 = GameplayArchivePayloadV4;
// V6 adds a typed object-case event; the envelope/checksum payload stays exact.
type GameplayArchiveEnvelopeV6 = GameplayArchiveEnvelopeV5;
type GameplayArchivePayloadV6 = GameplayArchivePayloadV5;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct GameplayArchivePayloadV3 {
    rule_set: String,
    events: Vec<WireGameplayEventEnvelope>,
    institutional_state: String,
    #[serde(rename = "federation_binding")]
    rampart_binding: GameplayRampartBinding,
    migration_history: Vec<GameplayArchiveMigrationRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct GameplayArchiveEnvelopeV2 {
    format: String,
    schema_version: u16,
    checksum: String,
    payload: GameplayArchivePayloadV2,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct GameplayArchivePayloadV2 {
    rule_set: String,
    events: Vec<WireGameplayEventEnvelope>,
    institutional_state: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct GameplayArchiveEnvelopeV1 {
    format: String,
    schema_version: u16,
    checksum: String,
    payload: GameplayArchivePayloadV1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct GameplayArchivePayloadV1 {
    rule_set: String,
    events: Vec<LegacyWireGameplayEventEnvelope>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct WireGameplayEventEnvelope {
    id: String,
    sequence: u64,
    revision: u64,
    causal_position: u64,
    payload: WireGameplayEvent,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct LegacyWireGameplayEventEnvelope {
    id: String,
    sequence: u64,
    revision: u64,
    causal_position: u64,
    payload: LegacyWireGameplayEvent,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", deny_unknown_fields)]
enum WireGameplayEvent {
    HuemanIdentityEstablished {
        continuity_id: String,
        participant_id: String,
        institutional_being_id: String,
    },
    HuemanMovementResolved {
        from: TilePosition,
        to: TilePosition,
    },
    HuemanInteractionOpened {
        at: TilePosition,
        target: InteractionId,
        evidence: Option<BoardwalkEvidence>,
        #[serde(default)]
        stonebend_evidence: Option<StonebendEvidence>,
        #[serde(default)]
        living_world_event: Option<LivingWorldEvent>,
        #[serde(default)]
        deep_pressure_event: Option<DeepPressureEvent>,
    },
    HuemanMapEntered {
        from: WorldMapId,
        to: WorldMapId,
        at: TilePosition,
    },
    BoardwalkFacultyDisclosed {
        faculty: HuemanFaculty,
    },
    BoardwalkOptionSupported {
        choice: BoardwalkChoice,
    },
    ReturningGoonChoiceCommitted {
        committed_by: String,
        choice: BoardwalkChoice,
        outcome_id: BoardwalkOutcomeId,
        relationship_bond: Option<String>,
    },
    StonebendFacultyDisclosed {
        faculty: HuemanFaculty,
    },
    StonebendContinuityOptionSupported {
        choice: StonebendContinuityChoice,
    },
    StonebendContinuityDeterminationCommitted {
        subject: String,
        choice: StonebendContinuityChoice,
        outcome_id: StonebendOutcomeId,
        decision_id: String,
        authority_actor: String,
    },
    LivingWorldChanged {
        event: LivingWorldEvent,
        #[serde(default)]
        deep_pressure_event: Option<DeepPressureEvent>,
        #[serde(default)]
        party_event: Option<PartyEvent>,
    },
    DeepPressureChanged {
        event: DeepPressureEvent,
    },
    PartyChanged {
        event: PartyEvent,
    },
    WingedGargoylePilotAdvanced {
        event: WingedGargoylePilotEvent,
    },
    SympioteFoundationChanged {
        event: SympioteFoundationEvent,
    },
    SurveyFrameChanged {
        event: SurveyFrameEvent,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", deny_unknown_fields)]
enum LegacyWireGameplayEvent {
    HuemanIdentityEstablished {
        continuity_id: String,
        participant_id: String,
        institutional_being_id: String,
    },
    HuemanMovementResolved {
        from: TilePosition,
        to: TilePosition,
    },
    HuemanInteractionOpened {
        at: TilePosition,
        target: InteractionId,
        evidence: Option<BoardwalkEvidence>,
    },
    HuemanMapEntered {
        from: WorldMapId,
        to: WorldMapId,
        at: TilePosition,
    },
    BoardwalkFacultyDisclosed {
        faculty: HuemanFaculty,
    },
    BoardwalkOptionSupported {
        choice: BoardwalkChoice,
    },
    ReturningGoonChoiceCommitted {
        committed_by: String,
        choice: BoardwalkChoice,
        goon_bond: Option<String>,
    },
}

pub fn encode_gameplay_archive(
    rule_set: &RuleSetId,
    events: &[GameplayEventEnvelope],
    world: &WorldSession,
) -> Result<String, GameplayArchiveError> {
    encode_gameplay_archive_for_component(rule_set, events, world, GAMEPLAY_RAMPART_COMPONENT_ID)
}

pub fn encode_gameplay_archive_for_component(
    rule_set: &RuleSetId,
    events: &[GameplayEventEnvelope],
    world: &WorldSession,
    component_id: &str,
) -> Result<String, GameplayArchiveError> {
    encode_gameplay_archive_with_history(
        rule_set,
        events,
        world,
        GameplayRampartBinding::canonical(component_id),
        Vec::new(),
    )
}

/// Encodes the authoritative gameplay component while carrying advisory-only
/// persona evidence through the existing Rampart binding.
pub fn encode_gameplay_archive_with_rampart_binding(
    rule_set: &RuleSetId,
    events: &[GameplayEventEnvelope],
    world: &WorldSession,
    rampart_binding: GameplayRampartBinding,
) -> Result<String, GameplayArchiveError> {
    encode_gameplay_archive_with_history(rule_set, events, world, rampart_binding, Vec::new())
}

#[deprecated(
    since = "0.1.0",
    note = "use encode_gameplay_archive_with_rampart_binding"
)]
pub fn encode_gameplay_archive_with_federation_binding(
    rule_set: &RuleSetId,
    events: &[GameplayEventEnvelope],
    world: &WorldSession,
    binding: GameplayRampartBinding,
) -> Result<String, GameplayArchiveError> {
    encode_gameplay_archive_with_rampart_binding(rule_set, events, world, binding)
}

pub(super) fn encode_gameplay_archive_with_history(
    rule_set: &RuleSetId,
    events: &[GameplayEventEnvelope],
    world: &WorldSession,
    rampart_binding: GameplayRampartBinding,
    mut migration_history: Vec<GameplayArchiveMigrationRecord>,
) -> Result<String, GameplayArchiveError> {
    rampart_binding.validate()?;
    migration_history.sort_by(|left, right| left.migration_id.cmp(&right.migration_id));
    let payload = GameplayArchivePayloadV6 {
        rule_set: rule_set.as_str().into(),
        events: events
            .iter()
            .map(WireGameplayEventEnvelope::try_from)
            .collect::<Result<_, _>>()?,
        institutional_state: world.persisted_state_output(),
        rampart_binding,
        migration_history,
    };
    let checksum = checksum_for(&payload)?;
    serde_json::to_string_pretty(&GameplayArchiveEnvelopeV6 {
        format: GAMEPLAY_ARCHIVE_FORMAT.into(),
        schema_version: GAMEPLAY_ARCHIVE_SCHEMA_VERSION,
        checksum,
        payload,
    })
    .map_err(|error| GameplayArchiveError::Serialization(error.to_string()))
}

pub fn encode_legacy_gameplay_archive_v2(
    rule_set: &RuleSetId,
    events: &[GameplayEventEnvelope],
    world: &WorldSession,
) -> Result<String, GameplayArchiveError> {
    let payload = GameplayArchivePayloadV2 {
        rule_set: rule_set.as_str().into(),
        events: events
            .iter()
            .map(WireGameplayEventEnvelope::try_from)
            .collect::<Result<_, _>>()?,
        institutional_state: world.persisted_state_output(),
    };
    let checksum = checksum_for(&payload)?;
    serde_json::to_string_pretty(&GameplayArchiveEnvelopeV2 {
        format: GAMEPLAY_ARCHIVE_FORMAT.into(),
        schema_version: PRE_RAMPART_GAMEPLAY_ARCHIVE_SCHEMA_VERSION,
        checksum,
        payload,
    })
    .map_err(|error| GameplayArchiveError::Serialization(error.to_string()))
}

/// Encodes the minimal schema-V1 archive retained by the compatibility suite.
///
/// V1 had no institutional snapshot, Rampart binding, or migration
/// history. New callers must write V4; this function exists only to preserve a
/// deterministic historical fixture for decoder and migration tests.
pub fn encode_empty_legacy_gameplay_archive_v1(
    rule_set: &RuleSetId,
) -> Result<String, GameplayArchiveError> {
    let payload = GameplayArchivePayloadV1 {
        rule_set: rule_set.as_str().into(),
        events: Vec::new(),
    };
    let checksum = checksum_for(&payload)?;
    serde_json::to_string_pretty(&GameplayArchiveEnvelopeV1 {
        format: GAMEPLAY_ARCHIVE_FORMAT.into(),
        schema_version: LEGACY_GAMEPLAY_ARCHIVE_SCHEMA_VERSION,
        checksum,
        payload,
    })
    .map_err(|error| GameplayArchiveError::Serialization(error.to_string()))
}

pub fn decode_gameplay_archive(
    encoded: &str,
) -> Result<(RuleSetId, Vec<GameplayEventEnvelope>, Option<String>), GameplayArchiveError> {
    let decoded = decode_gameplay_archive_with_metadata(encoded)?;
    Ok((
        decoded.rule_set,
        decoded.events,
        decoded.institutional_state,
    ))
}

pub fn decode_gameplay_archive_with_metadata(
    encoded: &str,
) -> Result<DecodedGameplayArchive, GameplayArchiveError> {
    let value: serde_json::Value = serde_json::from_str(encoded)
        .map_err(|error| GameplayArchiveError::Serialization(error.to_string()))?;
    let schema_version = value
        .get("schema_version")
        .and_then(serde_json::Value::as_u64)
        .and_then(|value| u16::try_from(value).ok())
        .ok_or_else(|| {
            GameplayArchiveError::Serialization("missing gameplay archive schema version".into())
        })?;
    let (rule_set, wire_events, institutional_state, rampart_binding, migration_history) =
        match schema_version {
            GAMEPLAY_ARCHIVE_SCHEMA_VERSION => {
                let envelope: GameplayArchiveEnvelopeV6 = serde_json::from_value(value)
                    .map_err(|error| GameplayArchiveError::Serialization(error.to_string()))?;
                validate_envelope(
                    &envelope.format,
                    envelope.schema_version,
                    &envelope.checksum,
                    &envelope.payload,
                )?;
                envelope.payload.rampart_binding.validate()?;
                (
                    envelope.payload.rule_set,
                    envelope.payload.events,
                    Some(envelope.payload.institutional_state),
                    envelope.payload.rampart_binding,
                    envelope.payload.migration_history,
                )
            }
            PRE_SURVEY_FRAME_GAMEPLAY_ARCHIVE_SCHEMA_VERSION => {
                let envelope: GameplayArchiveEnvelopeV5 = serde_json::from_value(value)
                    .map_err(|error| GameplayArchiveError::Serialization(error.to_string()))?;
                validate_envelope(
                    &envelope.format,
                    envelope.schema_version,
                    &envelope.checksum,
                    &envelope.payload,
                )?;
                envelope.payload.rampart_binding.validate()?;
                reject_pre_survey_frame_events(&envelope.payload.events)?;
                let mut migration_history = envelope.payload.migration_history;
                migration_history.push(survey_frame_migration_record());
                (
                    envelope.payload.rule_set,
                    envelope.payload.events,
                    Some(envelope.payload.institutional_state),
                    envelope.payload.rampart_binding,
                    migration_history,
                )
            }
            PRE_SYMPIOTE_GAMEPLAY_ARCHIVE_SCHEMA_VERSION => {
                let envelope: GameplayArchiveEnvelopeV4 = serde_json::from_value(value)
                    .map_err(|error| GameplayArchiveError::Serialization(error.to_string()))?;
                validate_envelope(
                    &envelope.format,
                    envelope.schema_version,
                    &envelope.checksum,
                    &envelope.payload,
                )?;
                envelope.payload.rampart_binding.validate()?;
                reject_pre_sympiote_events(&envelope.payload.events)?;
                let mut migration_history = envelope.payload.migration_history;
                migration_history.push(sympiote_foundation_migration_record());
                migration_history.push(survey_frame_migration_record());
                (
                    envelope.payload.rule_set,
                    envelope.payload.events,
                    Some(envelope.payload.institutional_state),
                    envelope.payload.rampart_binding,
                    migration_history,
                )
            }
            PRE_SYNTHESIS_GAMEPLAY_ARCHIVE_SCHEMA_VERSION => {
                let envelope: GameplayArchiveEnvelopeV3 = serde_json::from_value(value)
                    .map_err(|error| GameplayArchiveError::Serialization(error.to_string()))?;
                validate_envelope(
                    &envelope.format,
                    envelope.schema_version,
                    &envelope.checksum,
                    &envelope.payload,
                )?;
                envelope.payload.rampart_binding.validate()?;
                reject_pre_synthesis_events(&envelope.payload.events)?;
                let mut migration_history = envelope.payload.migration_history;
                migration_history.push(synthesis_pilot_migration_record());
                migration_history.push(sympiote_foundation_migration_record());
                migration_history.push(survey_frame_migration_record());
                (
                    envelope.payload.rule_set,
                    envelope.payload.events,
                    Some(envelope.payload.institutional_state),
                    envelope.payload.rampart_binding,
                    migration_history,
                )
            }
            PRE_RAMPART_GAMEPLAY_ARCHIVE_SCHEMA_VERSION => {
                let envelope: GameplayArchiveEnvelopeV2 = serde_json::from_value(value)
                    .map_err(|error| GameplayArchiveError::Serialization(error.to_string()))?;
                validate_envelope(
                    &envelope.format,
                    envelope.schema_version,
                    &envelope.checksum,
                    &envelope.payload,
                )?;
                reject_pre_synthesis_events(&envelope.payload.events)?;
                (
                    envelope.payload.rule_set,
                    envelope.payload.events,
                    Some(envelope.payload.institutional_state),
                    GameplayRampartBinding::canonical(GAMEPLAY_RAMPART_COMPONENT_ID),
                    vec![
                        rampart_migration_record(),
                        synthesis_pilot_migration_record(),
                        sympiote_foundation_migration_record(),
                        survey_frame_migration_record(),
                    ],
                )
            }
            LEGACY_GAMEPLAY_ARCHIVE_SCHEMA_VERSION => {
                let envelope: GameplayArchiveEnvelopeV1 = serde_json::from_value(value)
                    .map_err(|error| GameplayArchiveError::Serialization(error.to_string()))?;
                validate_envelope(
                    &envelope.format,
                    envelope.schema_version,
                    &envelope.checksum,
                    &envelope.payload,
                )?;
                (
                    envelope.payload.rule_set,
                    envelope
                        .payload
                        .events
                        .into_iter()
                        .map(LegacyWireGameplayEventEnvelope::into_current)
                        .collect(),
                    Some(LEGACY_GAMEPLAY_AUTHORITY_MIGRATION.into()),
                    GameplayRampartBinding::canonical(GAMEPLAY_RAMPART_COMPONENT_ID),
                    vec![
                        legacy_authority_migration_record(),
                        rampart_migration_record(),
                        synthesis_pilot_migration_record(),
                        sympiote_foundation_migration_record(),
                        survey_frame_migration_record(),
                    ],
                )
            }
            unsupported => return Err(GameplayArchiveError::UnsupportedSchema(unsupported)),
        };
    let rule_set = RuleSetId::new(rule_set)
        .map_err(|error| GameplayArchiveError::InvalidDomainValue(error.to_string()))?;
    let events = wire_events
        .into_iter()
        .map(|event| event.into_domain(&rule_set))
        .collect::<Result<_, _>>()?;
    Ok(DecodedGameplayArchive {
        source_schema_version: schema_version,
        rule_set,
        events,
        institutional_state,
        rampart_binding,
        migration_history,
    })
}

pub fn migrate_gameplay_archive(encoded: &str) -> Result<String, GameplayArchiveError> {
    let decoded = decode_gameplay_archive_with_metadata(encoded)?;
    if decoded.source_schema_version == GAMEPLAY_ARCHIVE_SCHEMA_VERSION {
        return encode_replayed_gameplay_archive(decoded);
    }
    encode_replayed_gameplay_archive(decoded)
}

fn encode_replayed_gameplay_archive(
    decoded: DecodedGameplayArchive,
) -> Result<String, GameplayArchiveError> {
    let world = decoded
        .institutional_state
        .as_deref()
        .map(WorldSession::from_persisted_output)
        .transpose()
        .map_err(|error| GameplayArchiveError::InvalidDomainValue(error.to_string()))?
        .unwrap_or_else(WorldSession::canonical);
    encode_gameplay_archive_with_history(
        &decoded.rule_set,
        &decoded.events,
        &world,
        decoded.rampart_binding,
        decoded.migration_history,
    )
}

fn legacy_authority_migration_record() -> GameplayArchiveMigrationRecord {
    GameplayArchiveMigrationRecord {
        migration_id: "migration.gameplay.v1-to-v2.authority-dependency".into(),
        from_schema_version: 1,
        to_schema_version: 2,
        decision:
            "preserve historical Boardwalk authority dependency without inventing live authority"
                .into(),
        provenance_id: "provenance.gameplay.migration.v1-to-v2".into(),
    }
}

fn rampart_migration_record() -> GameplayArchiveMigrationRecord {
    GameplayArchiveMigrationRecord {
        migration_id: "migration.gameplay.v2-to-v3.runtime-federation".into(),
        from_schema_version: 2,
        to_schema_version: 3,
        decision:
            "bind existing gameplay history to The Rampart of Roi and Roselina without inventing domain events"
                .into(),
        provenance_id: "provenance.gameplay.migration.v2-to-v3".into(),
    }
}

fn synthesis_pilot_migration_record() -> GameplayArchiveMigrationRecord {
    GameplayArchiveMigrationRecord {
        migration_id: "migration.gameplay.v3-to-v4.winged-gargoyle-pilot".into(),
        from_schema_version: 3,
        to_schema_version: 4,
        decision: "preserve pre-pilot gameplay history without inventing Synthesis progress".into(),
        provenance_id: "provenance.gameplay.migration.v3-to-v4".into(),
    }
}

fn reject_pre_synthesis_events(
    events: &[WireGameplayEventEnvelope],
) -> Result<(), GameplayArchiveError> {
    reject_pre_sympiote_events(events)?;
    if events.iter().any(|event| {
        matches!(
            event.payload,
            WireGameplayEvent::WingedGargoylePilotAdvanced { .. }
        )
    }) {
        return Err(GameplayArchiveError::InvalidDomainValue(
            "gameplay archive schemas before V4 cannot contain winged Gargoyle pilot events".into(),
        ));
    }
    Ok(())
}

fn sympiote_foundation_migration_record() -> GameplayArchiveMigrationRecord {
    GameplayArchiveMigrationRecord {
        migration_id: "migration.gameplay.v4-to-v5.sympiote-foundation".into(),
        from_schema_version: 4, to_schema_version: 5,
        decision: "preserve prior identity, Forms, and history without inventing a Sympiote graft, consent, or cultivation".into(),
        provenance_id: "provenance.gameplay.migration.v4-to-v5".into(),
    }
}

fn reject_pre_sympiote_events(
    events: &[WireGameplayEventEnvelope],
) -> Result<(), GameplayArchiveError> {
    reject_pre_survey_frame_events(events)?;
    if events.iter().any(|event| {
        matches!(
            event.payload,
            WireGameplayEvent::SympioteFoundationChanged { .. }
        )
    }) {
        return Err(GameplayArchiveError::InvalidDomainValue(
            "gameplay archives before V5 cannot contain Sympiote foundation events".into(),
        ));
    }
    Ok(())
}

fn survey_frame_migration_record() -> GameplayArchiveMigrationRecord {
    GameplayArchiveMigrationRecord {
        migration_id: "migration.gameplay.v5-to-v6.survey-frame".into(), from_schema_version: 5, to_schema_version: 6,
        decision: "preserve prior identity, Forms, consent and history without inventing an object, proof, trial, custody or resident decision".into(),
        provenance_id: "provenance.gameplay.migration.v5-to-v6".into(),
    }
}
fn reject_pre_survey_frame_events(
    events: &[WireGameplayEventEnvelope],
) -> Result<(), GameplayArchiveError> {
    if events
        .iter()
        .any(|e| matches!(e.payload, WireGameplayEvent::SurveyFrameChanged { .. }))
    {
        return Err(GameplayArchiveError::InvalidDomainValue(
            "gameplay archives before V6 cannot contain survey-frame events".into(),
        ));
    }
    Ok(())
}

fn validate_envelope(
    format: &str,
    schema_version: u16,
    expected_checksum: &str,
    payload: &impl Serialize,
) -> Result<(), GameplayArchiveError> {
    if format != GAMEPLAY_ARCHIVE_FORMAT {
        return Err(GameplayArchiveError::UnsupportedFormat(format.into()));
    }
    if !is_supported_gameplay_archive_schema_version(schema_version) {
        return Err(GameplayArchiveError::UnsupportedSchema(schema_version));
    }
    let actual_checksum = checksum_for(payload)?;
    if expected_checksum != actual_checksum {
        return Err(GameplayArchiveError::ChecksumMismatch {
            expected: expected_checksum.into(),
            actual: actual_checksum,
        });
    }
    Ok(())
}

impl LegacyWireGameplayEventEnvelope {
    fn into_current(self) -> WireGameplayEventEnvelope {
        let payload = match self.payload {
            LegacyWireGameplayEvent::HuemanIdentityEstablished {
                continuity_id,
                participant_id,
                institutional_being_id,
            } => WireGameplayEvent::HuemanIdentityEstablished {
                continuity_id,
                participant_id,
                institutional_being_id,
            },
            LegacyWireGameplayEvent::HuemanMovementResolved { from, to } => {
                WireGameplayEvent::HuemanMovementResolved { from, to }
            }
            LegacyWireGameplayEvent::HuemanInteractionOpened {
                at,
                target,
                evidence,
            } => WireGameplayEvent::HuemanInteractionOpened {
                at,
                target,
                evidence,
                stonebend_evidence: None,
                living_world_event: None,
                deep_pressure_event: None,
            },
            LegacyWireGameplayEvent::HuemanMapEntered { from, to, at } => {
                WireGameplayEvent::HuemanMapEntered { from, to, at }
            }
            LegacyWireGameplayEvent::BoardwalkFacultyDisclosed { faculty } => {
                WireGameplayEvent::BoardwalkFacultyDisclosed { faculty }
            }
            LegacyWireGameplayEvent::BoardwalkOptionSupported { choice } => {
                WireGameplayEvent::BoardwalkOptionSupported { choice }
            }
            LegacyWireGameplayEvent::ReturningGoonChoiceCommitted {
                committed_by,
                choice,
                goon_bond,
            } => WireGameplayEvent::ReturningGoonChoiceCommitted {
                committed_by,
                choice,
                outcome_id: BoardwalkOutcomeId::for_choice(choice),
                relationship_bond: goon_bond,
            },
        };
        WireGameplayEventEnvelope {
            id: self.id,
            sequence: self.sequence,
            revision: self.revision,
            causal_position: self.causal_position,
            payload,
        }
    }
}

impl TryFrom<&GameplayEventEnvelope> for WireGameplayEventEnvelope {
    type Error = GameplayArchiveError;

    fn try_from(event: &GameplayEventEnvelope) -> Result<Self, Self::Error> {
        let payload = match &event.payload {
            GameplayEvent::HuemanIdentityEstablished { identity } => {
                if identity.incarnation() != &ActiveIncarnationRef::Hueman {
                    return Err(GameplayArchiveError::IdentityPayloadMismatch);
                }
                WireGameplayEvent::HuemanIdentityEstablished {
                    continuity_id: identity.id().as_str().into(),
                    participant_id: identity.domain_refs().participant().as_str().into(),
                    institutional_being_id: identity.domain_refs().institutional().as_str().into(),
                }
            }
            GameplayEvent::RegionalBeingIdentityEstablished { .. } => {
                return Err(GameplayArchiveError::UnsupportedGameplayEvent(
                    "RegionalBeingIdentityEstablished".into(),
                ));
            }
            GameplayEvent::HuemanMovementResolved { from, to } => {
                WireGameplayEvent::HuemanMovementResolved {
                    from: *from,
                    to: *to,
                }
            }
            GameplayEvent::HuemanInteractionOpened {
                at,
                target,
                evidence,
                stonebend_evidence,
                living_world_event,
                deep_pressure_event,
            } => WireGameplayEvent::HuemanInteractionOpened {
                at: *at,
                target: *target,
                evidence: *evidence,
                stonebend_evidence: *stonebend_evidence,
                living_world_event: living_world_event.clone(),
                deep_pressure_event: deep_pressure_event.clone(),
            },
            GameplayEvent::HuemanMapEntered { from, to, at } => {
                WireGameplayEvent::HuemanMapEntered {
                    from: *from,
                    to: *to,
                    at: *at,
                }
            }
            GameplayEvent::BoardwalkFacultyDisclosed { faculty } => {
                WireGameplayEvent::BoardwalkFacultyDisclosed { faculty: *faculty }
            }
            GameplayEvent::BoardwalkOptionSupported { choice } => {
                WireGameplayEvent::BoardwalkOptionSupported { choice: *choice }
            }
            GameplayEvent::ReturningGoonChoiceCommitted {
                committed_by,
                choice,
                outcome_id,
                relationship_bond,
            } => WireGameplayEvent::ReturningGoonChoiceCommitted {
                committed_by: committed_by.as_str().into(),
                choice: *choice,
                outcome_id: *outcome_id,
                relationship_bond: relationship_bond.as_ref().map(|bond| bond.as_str().into()),
            },
            GameplayEvent::StonebendFacultyDisclosed { faculty } => {
                WireGameplayEvent::StonebendFacultyDisclosed { faculty: *faculty }
            }
            GameplayEvent::StonebendContinuityOptionSupported { choice } => {
                WireGameplayEvent::StonebendContinuityOptionSupported { choice: *choice }
            }
            GameplayEvent::StonebendContinuityDeterminationCommitted {
                subject,
                choice,
                outcome_id,
                decision_id,
                authority_actor,
            } => WireGameplayEvent::StonebendContinuityDeterminationCommitted {
                subject: subject.as_str().into(),
                choice: *choice,
                outcome_id: *outcome_id,
                decision_id: decision_id.as_str().into(),
                authority_actor: authority_actor.as_str().into(),
            },
            GameplayEvent::LivingWorldChanged {
                event,
                deep_pressure_event,
                party_event,
            } => WireGameplayEvent::LivingWorldChanged {
                event: event.clone(),
                deep_pressure_event: deep_pressure_event.clone(),
                party_event: party_event.clone(),
            },
            GameplayEvent::DeepPressureChanged { event } => {
                WireGameplayEvent::DeepPressureChanged {
                    event: event.clone(),
                }
            }
            GameplayEvent::PartyChanged { event } => WireGameplayEvent::PartyChanged {
                event: event.clone(),
            },
            GameplayEvent::WingedGargoylePilotAdvanced { event } => {
                WireGameplayEvent::WingedGargoylePilotAdvanced {
                    event: event.clone(),
                }
            }
            GameplayEvent::SympioteFoundationChanged { event } => {
                WireGameplayEvent::SympioteFoundationChanged {
                    event: event.clone(),
                }
            }
            GameplayEvent::SurveyFrameChanged { event } => WireGameplayEvent::SurveyFrameChanged {
                event: event.clone(),
            },
        };
        Ok(Self {
            id: event.id.as_str().into(),
            sequence: event.sequence,
            revision: event.revision.get(),
            causal_position: event.causal_position.get(),
            payload,
        })
    }
}

impl WireGameplayEventEnvelope {
    fn into_domain(
        self,
        rule_set: &RuleSetId,
    ) -> Result<GameplayEventEnvelope, GameplayArchiveError> {
        let payload = match self.payload {
            WireGameplayEvent::HuemanIdentityEstablished {
                continuity_id,
                participant_id,
                institutional_being_id,
            } => GameplayEvent::HuemanIdentityEstablished {
                identity: BeingContinuityRecord::hueman(
                    parse(continuity_id, BeingContinuityId::new)?,
                    parse(participant_id, ParticipantId::new)?,
                    parse(institutional_being_id, InstitutionalBeingId::new)?,
                ),
            },
            WireGameplayEvent::HuemanMovementResolved { from, to } => {
                GameplayEvent::HuemanMovementResolved { from, to }
            }
            WireGameplayEvent::HuemanInteractionOpened {
                at,
                target,
                evidence,
                stonebend_evidence,
                living_world_event,
                deep_pressure_event,
            } => GameplayEvent::HuemanInteractionOpened {
                at,
                target,
                evidence,
                stonebend_evidence,
                living_world_event,
                deep_pressure_event,
            },
            WireGameplayEvent::HuemanMapEntered { from, to, at } => {
                GameplayEvent::HuemanMapEntered { from, to, at }
            }
            WireGameplayEvent::BoardwalkFacultyDisclosed { faculty } => {
                GameplayEvent::BoardwalkFacultyDisclosed { faculty }
            }
            WireGameplayEvent::BoardwalkOptionSupported { choice } => {
                GameplayEvent::BoardwalkOptionSupported { choice }
            }
            WireGameplayEvent::ReturningGoonChoiceCommitted {
                committed_by,
                choice,
                outcome_id,
                relationship_bond,
            } => GameplayEvent::ReturningGoonChoiceCommitted {
                committed_by: parse(committed_by, ParticipantId::new)?,
                choice,
                outcome_id,
                relationship_bond: relationship_bond
                    .map(|value| parse(value, BondId::new))
                    .transpose()?,
            },
            WireGameplayEvent::StonebendFacultyDisclosed { faculty } => {
                GameplayEvent::StonebendFacultyDisclosed { faculty }
            }
            WireGameplayEvent::StonebendContinuityOptionSupported { choice } => {
                GameplayEvent::StonebendContinuityOptionSupported { choice }
            }
            WireGameplayEvent::StonebendContinuityDeterminationCommitted {
                subject,
                choice,
                outcome_id,
                decision_id,
                authority_actor,
            } => GameplayEvent::StonebendContinuityDeterminationCommitted {
                subject: parse(subject, ParticipantId::new)?,
                choice,
                outcome_id,
                decision_id: parse(decision_id, HouseDecisionId::new)?,
                authority_actor: parse(authority_actor, AuthorityActorId::new)?,
            },
            WireGameplayEvent::LivingWorldChanged {
                event,
                deep_pressure_event,
                party_event,
            } => GameplayEvent::LivingWorldChanged {
                event,
                deep_pressure_event,
                party_event,
            },
            WireGameplayEvent::DeepPressureChanged { event } => {
                GameplayEvent::DeepPressureChanged { event }
            }
            WireGameplayEvent::PartyChanged { event } => GameplayEvent::PartyChanged { event },
            WireGameplayEvent::WingedGargoylePilotAdvanced { event } => {
                GameplayEvent::WingedGargoylePilotAdvanced { event }
            }
            WireGameplayEvent::SympioteFoundationChanged { event } => {
                GameplayEvent::SympioteFoundationChanged { event }
            }
            WireGameplayEvent::SurveyFrameChanged { event } => {
                GameplayEvent::SurveyFrameChanged { event }
            }
        };
        Ok(GameplayEventEnvelope {
            id: parse(self.id, GameplayEventId::new)?,
            sequence: self.sequence,
            revision: GameRevision::from_archive(self.revision),
            causal_position: CausalPosition::new(self.causal_position),
            rule_set: rule_set.clone(),
            payload,
        })
    }
}

fn parse<T, E: std::fmt::Display>(
    value: String,
    constructor: impl FnOnce(String) -> Result<T, E>,
) -> Result<T, GameplayArchiveError> {
    constructor(value).map_err(|error| GameplayArchiveError::InvalidDomainValue(error.to_string()))
}

fn checksum_for(payload: &impl Serialize) -> Result<String, GameplayArchiveError> {
    let bytes = serde_json::to_vec(payload)
        .map_err(|error| GameplayArchiveError::Serialization(error.to_string()))?;
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in bytes {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    Ok(format!("fnv1a64:{hash:016x}"))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameplayArchiveError {
    Serialization(String),
    UnsupportedFormat(String),
    UnsupportedSchema(u16),
    ChecksumMismatch { expected: String, actual: String },
    UnsupportedGameplayEvent(String),
    InvalidDomainValue(String),
    IdentityPayloadMismatch,
    InvalidRampartBinding,
}

impl std::fmt::Display for GameplayArchiveError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "gameplay archive rejected: {self:?}")
    }
}

impl std::error::Error for GameplayArchiveError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constitutional::V2_RULE_SET;

    #[test]
    fn v4_migration_preserves_existing_form_and_adds_no_sympiote_history() {
        use crate::gameplay::{
            GameApplicationService, GameplayCommand, GameplayEventMetadata,
            WingedGargoylePilotAction as A,
        };
        let mut application = GameApplicationService::new(RuleSetId::new(V2_RULE_SET).unwrap());
        application
            .execute(
                GameplayEventMetadata {
                    id: GameplayEventId::new("event.fixture.identity").unwrap(),
                    causal_position: CausalPosition::new(1),
                },
                GameplayCommand::EstablishHuemanIdentity {
                    continuity: BeingContinuityId::new("continuity.fixture.hueman").unwrap(),
                    participant: ParticipantId::new("participant.fixture.hueman").unwrap(),
                    institutional: InstitutionalBeingId::new("being.fixture.hueman").unwrap(),
                },
            )
            .unwrap();
        for (index, action) in [
            A::RecoverStonebendStructure,
            A::RecoverSandmanorMeasurement,
            A::ObtainGlaushouseCompatibility,
            A::EarnFlyntWitness,
            A::RecoverStonebendMaterial,
            A::PreserveSandmanorMethod,
            A::RecordStableTrial,
        ]
        .into_iter()
        .enumerate()
        {
            let at = u64::try_from(index).unwrap() + 2;
            application
                .execute(
                    GameplayEventMetadata {
                        id: GameplayEventId::new(format!("event.fixture.pilot.{at}")).unwrap(),
                        causal_position: CausalPosition::new(at),
                    },
                    GameplayCommand::AdvanceWingedGargoylePilot { action },
                )
                .unwrap();
        }
        let mut before: GameplayArchiveEnvelopeV4 =
            serde_json::from_str(&application.encode_archive().unwrap()).unwrap();
        before.schema_version = PRE_SYMPIOTE_GAMEPLAY_ARCHIVE_SCHEMA_VERSION;
        let before = serde_json::to_string_pretty(&before).unwrap();
        let after = migrate_gameplay_archive(&before).unwrap();
        let decoded = decode_gameplay_archive_with_metadata(&after).unwrap();
        assert_eq!(decoded.source_schema_version, 6);
        assert_eq!(
            decoded.migration_history,
            vec![
                sympiote_foundation_migration_record(),
                survey_frame_migration_record()
            ]
        );
        let replayed = GameApplicationService::from_archive(&after).unwrap();
        assert_eq!(replayed.runtime().hueman(), application.runtime().hueman());
        assert_eq!(
            replayed.runtime().winged_gargoyle_pilot(),
            application.runtime().winged_gargoyle_pilot()
        );
        assert_eq!(
            replayed.runtime().sympiote_foundation(),
            &crate::world::sympiote_foundation::SympioteFoundationState::default()
        );
        assert_eq!(replayed.events(), application.events());
        assert_eq!(migrate_gameplay_archive(&after).unwrap(), after);
    }

    #[test]
    fn old_envelopes_cannot_smuggle_sympiote_events() {
        let payload = GameplayArchivePayloadV4 {
            rule_set: V2_RULE_SET.into(), institutional_state: WorldSession::canonical().persisted_state_output(),
            rampart_binding: GameplayRampartBinding::canonical(GAMEPLAY_RAMPART_COMPONENT_ID), migration_history: Vec::new(),
            events: vec![WireGameplayEventEnvelope { id: "event.fixture.false-foundation".into(), sequence: 0, revision: 1, causal_position: 1,
                payload: WireGameplayEvent::SympioteFoundationChanged { event: SympioteFoundationEvent {
                    action: crate::world::sympiote_foundation::SympioteFoundationAction::RequestAssessment,
                    subject_id: "being.fixture.hueman".into(), from: crate::world::sympiote_foundation::SympioteFoundationPhase::AwaitingAssessment,
                    to: crate::world::sympiote_foundation::SympioteFoundationPhase::AwaitingConsent,
                    form_established: false, needs_rest: false, comfortable_use: false, practice_count: 0, clinical_receipt: None,
                    evidence_id: "evidence.fixture.false-foundation".into(),
                } }
            }],
        };
        for version in [3, 4] {
            let envelope = GameplayArchiveEnvelopeV4 {
                format: GAMEPLAY_ARCHIVE_FORMAT.into(),
                schema_version: version,
                checksum: checksum_for(&payload).unwrap(),
                payload: payload.clone(),
            };
            let encoded = serde_json::to_string(&envelope).unwrap();
            assert!(matches!(
                decode_gameplay_archive_with_metadata(&encoded),
                Err(GameplayArchiveError::InvalidDomainValue(_))
            ));
        }
    }

    #[test]
    fn schema_three_archive_migrates_without_inventing_synthesis_progress() {
        let world = WorldSession::canonical();
        let payload = GameplayArchivePayloadV3 {
            rule_set: V2_RULE_SET.into(),
            events: Vec::new(),
            institutional_state: world.persisted_state_output(),
            rampart_binding: GameplayRampartBinding::canonical(GAMEPLAY_RAMPART_COMPONENT_ID),
            migration_history: Vec::new(),
        };
        let encoded = serde_json::to_string(&GameplayArchiveEnvelopeV3 {
            format: GAMEPLAY_ARCHIVE_FORMAT.into(),
            schema_version: PRE_SYNTHESIS_GAMEPLAY_ARCHIVE_SCHEMA_VERSION,
            checksum: checksum_for(&payload).unwrap(),
            payload,
        })
        .unwrap();
        let decoded = decode_gameplay_archive_with_metadata(&encoded).unwrap();
        assert_eq!(
            decoded.source_schema_version,
            PRE_SYNTHESIS_GAMEPLAY_ARCHIVE_SCHEMA_VERSION
        );
        assert!(decoded.events.is_empty());
        assert_eq!(
            decoded.migration_history,
            vec![
                synthesis_pilot_migration_record(),
                sympiote_foundation_migration_record(),
                survey_frame_migration_record()
            ]
        );

        let migrated = migrate_gameplay_archive(&encoded).unwrap();
        let migrated = decode_gameplay_archive_with_metadata(&migrated).unwrap();
        assert_eq!(
            migrated.source_schema_version,
            GAMEPLAY_ARCHIVE_SCHEMA_VERSION
        );
        assert!(migrated.events.is_empty());
        assert_eq!(
            migrated.migration_history,
            vec![
                synthesis_pilot_migration_record(),
                sympiote_foundation_migration_record(),
                survey_frame_migration_record()
            ]
        );

        let payload = GameplayArchivePayloadV3 {
            rule_set: V2_RULE_SET.into(),
            events: vec![WireGameplayEventEnvelope {
                id: "game-event.impossible-v3-pilot".into(),
                sequence: 0,
                revision: 1,
                causal_position: 1,
                payload: WireGameplayEvent::WingedGargoylePilotAdvanced {
                    event: WingedGargoylePilotEvent {
                        action: crate::gameplay::WingedGargoylePilotAction::RecordStableTrial,
                        stage: crate::recipe_discovery::GargoyleHuntStage::TrialSynthesis,
                        form: crate::gameplay::WingedGargoylePilotForm::Gargoyle,
                        fragment_count: 6,
                        capability_id: None,
                        power_recipe_id: None,
                        completed: false,
                    },
                },
            }],
            institutional_state: world.persisted_state_output(),
            rampart_binding: GameplayRampartBinding::canonical(GAMEPLAY_RAMPART_COMPONENT_ID),
            migration_history: Vec::new(),
        };
        let encoded = serde_json::to_string(&GameplayArchiveEnvelopeV3 {
            format: GAMEPLAY_ARCHIVE_FORMAT.into(),
            schema_version: PRE_SYNTHESIS_GAMEPLAY_ARCHIVE_SCHEMA_VERSION,
            checksum: checksum_for(&payload).unwrap(),
            payload,
        })
        .unwrap();
        assert!(matches!(
            decode_gameplay_archive_with_metadata(&encoded),
            Err(GameplayArchiveError::InvalidDomainValue(message))
                if message.contains("before V4")
        ));
    }

    #[test]
    fn schema_one_archive_migrates_its_historical_authority_dependency() {
        let payload = GameplayArchivePayloadV1 {
            rule_set: V2_RULE_SET.into(),
            events: vec![LegacyWireGameplayEventEnvelope {
                id: "game-event.legacy-boardwalk-choice".into(),
                sequence: 0,
                revision: 1,
                causal_position: 1,
                payload: LegacyWireGameplayEvent::ReturningGoonChoiceCommitted {
                    committed_by: super::super::RETURNING_GOON_PARTICIPANT_ID.into(),
                    choice: BoardwalkChoice::GoonBond,
                    goon_bond: Some(super::super::BOARDWALK_GOON_BOND_ID.into()),
                },
            }],
        };
        let encoded = serde_json::to_string(&GameplayArchiveEnvelopeV1 {
            format: GAMEPLAY_ARCHIVE_FORMAT.into(),
            schema_version: LEGACY_GAMEPLAY_ARCHIVE_SCHEMA_VERSION,
            checksum: checksum_for(&payload).unwrap(),
            payload,
        })
        .unwrap();
        let (rule_set, events, institutional_state) = decode_gameplay_archive(&encoded).unwrap();
        assert_eq!(rule_set.as_str(), V2_RULE_SET);
        assert_eq!(events.len(), 1);
        let GameplayEvent::ReturningGoonChoiceCommitted {
            outcome_id,
            relationship_bond,
            ..
        } = &events[0].payload
        else {
            panic!("legacy Boardwalk choice");
        };
        assert_eq!(*outcome_id, BoardwalkOutcomeId::GoonBondV1);
        assert_eq!(
            relationship_bond.as_ref().map(BondId::as_str),
            Some(super::super::BOARDWALK_GOON_BOND_ID)
        );
        let institutional_state = institutional_state.unwrap();
        assert!(institutional_state.starts_with("schema_version:2\n"));
        assert!(institutional_state.contains("being.stonebend.fixture-member"));

        let migrated = migrate_gameplay_archive(&encoded).unwrap();
        let migrated = decode_gameplay_archive_with_metadata(&migrated).unwrap();
        assert_eq!(
            migrated.source_schema_version,
            GAMEPLAY_ARCHIVE_SCHEMA_VERSION
        );
        assert_eq!(migrated.events.len(), 1);
        assert_eq!(migrated.migration_history.len(), 5);
        assert!(migrated.migration_history.iter().any(
            |record| record.migration_id == "migration.gameplay.v1-to-v2.authority-dependency"
        ));
        assert!(
            migrated
                .migration_history
                .iter()
                .any(|record| record.migration_id
                    == "migration.gameplay.v2-to-v3.runtime-federation")
        );
        assert!(migrated.migration_history.iter().any(|record| {
            record.migration_id == "migration.gameplay.v3-to-v4.winged-gargoyle-pilot"
        }));
    }
}
