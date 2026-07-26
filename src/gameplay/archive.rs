use serde::{Deserialize, Serialize};

use crate::constitutional::{
    AuthorityActorId, BondId, CausalPosition, HouseDecisionId, ParticipantId, RuleSetId,
};
use crate::institution::InstitutionalBeingId;
use crate::world::session::WorldSession;

use super::{
    ActiveIncarnationRef, BeingContinuityId, BeingContinuityRecord, BoardwalkChoice,
    BoardwalkEvidence, BoardwalkOutcomeId, DeepPressureEvent, GameRevision, GameplayEvent,
    GameplayEventEnvelope, GameplayEventId, HuemanFaculty, InteractionId, LivingWorldEvent,
    PartyEvent, StonebendContinuityChoice, StonebendEvidence, StonebendOutcomeId, TilePosition,
    WorldMapId,
};

pub const GAMEPLAY_ARCHIVE_FORMAT: &str = "hollow-grove-gameplay";
pub const GAMEPLAY_ARCHIVE_SCHEMA_VERSION: u16 = 2;
const LEGACY_GAMEPLAY_ARCHIVE_SCHEMA_VERSION: u16 = 1;
// Schema V1 formed Boardwalk Bonds against these test-era authority actors.
// Migration preserves that historical dependency explicitly so replay is
// exact; new sessions never load these actors and use live schema-V2 state.
const LEGACY_GAMEPLAY_AUTHORITY_MIGRATION: &str = "schema_version:2\n\
office-holder\toffice.stonebend.hypergiant\tbeing.stonebend.fixture-member\ttrue\n\
office-holder\toffice.sandmanor.sandman\tbeing.sandmanor.fixture-member\ttrue\n\
office-holder\toffice.glaushouse.prima-donna\tbeing.glaushouse.fixture-member\ttrue\n";

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
        schema_version: GAMEPLAY_ARCHIVE_SCHEMA_VERSION,
        checksum,
        payload,
    })
    .map_err(|error| GameplayArchiveError::Serialization(error.to_string()))
}

pub fn decode_gameplay_archive(
    encoded: &str,
) -> Result<(RuleSetId, Vec<GameplayEventEnvelope>, Option<String>), GameplayArchiveError> {
    let value: serde_json::Value = serde_json::from_str(encoded)
        .map_err(|error| GameplayArchiveError::Serialization(error.to_string()))?;
    let schema_version = value
        .get("schema_version")
        .and_then(serde_json::Value::as_u64)
        .and_then(|value| u16::try_from(value).ok())
        .ok_or_else(|| {
            GameplayArchiveError::Serialization("missing gameplay archive schema version".into())
        })?;
    let (rule_set, wire_events, institutional_state) = match schema_version {
        GAMEPLAY_ARCHIVE_SCHEMA_VERSION => {
            let envelope: GameplayArchiveEnvelopeV2 = serde_json::from_value(value)
                .map_err(|error| GameplayArchiveError::Serialization(error.to_string()))?;
            validate_envelope(
                &envelope.format,
                envelope.schema_version,
                &envelope.checksum,
                &envelope.payload,
            )?;
            (
                envelope.payload.rule_set,
                envelope.payload.events,
                Some(envelope.payload.institutional_state),
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
    Ok((rule_set, events, institutional_state))
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
    if !matches!(
        schema_version,
        GAMEPLAY_ARCHIVE_SCHEMA_VERSION | LEGACY_GAMEPLAY_ARCHIVE_SCHEMA_VERSION
    ) {
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
    }
}
