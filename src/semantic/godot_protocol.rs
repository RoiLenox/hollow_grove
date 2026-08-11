//! Rust-owned, presentation-only semantic events for Godot and other clients.

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::composition::{ExternalRef, Position3, Translation3, vertical_origin_relation};
use crate::runtime_federation::{
    FestivalResult, ProposalStatus, SemanticDecision, SemanticProposal, SemanticRejectionReason,
};

use hollow_grove_semantic_kernel::OriginRelation;

pub const SEMANTIC_EVENT_SCHEMA_VERSION: &str = "hollow-grove-semantic-event-v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticGodotEventEnvelope {
    pub schema_version: String,
    #[serde(flatten)]
    pub event: SemanticGodotEvent,
}

impl SemanticGodotEventEnvelope {
    #[must_use]
    pub fn new(event: SemanticGodotEvent) -> Self {
        Self {
            schema_version: SEMANTIC_EVENT_SCHEMA_VERSION.into(),
            event,
        }
    }
}

pub fn decode_semantic_godot_event(
    bytes: &[u8],
) -> Result<SemanticGodotEventEnvelope, SemanticEventDecodeError> {
    let envelope: SemanticGodotEventEnvelope = serde_json::from_slice(bytes)
        .map_err(|error| SemanticEventDecodeError::MalformedJson(error.to_string()))?;
    if envelope.schema_version != SEMANTIC_EVENT_SCHEMA_VERSION {
        return Err(SemanticEventDecodeError::UnsupportedSchema(
            envelope.schema_version,
        ));
    }
    Ok(envelope)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "event", rename_all = "snake_case", deny_unknown_fields)]
pub enum SemanticGodotEvent {
    SemanticPositionProposed {
        cycle_id: String,
        proposal_ref: String,
        point_ref: String,
        scale_key: String,
        proposed_position: Position3,
        status: ProposalStatus,
    },
    SemanticPositionConfirmed {
        cycle_id: String,
        proposal_ref: String,
        point_ref: String,
        scale_key: String,
        confirmed_position: Position3,
        witness_ref: String,
    },
    SemanticPositionRejected {
        cycle_id: String,
        proposal_ref: String,
        point_ref: String,
        scale_key: String,
        proposed_position: Position3,
        witness_ref: String,
        reasons: Vec<String>,
    },
    SemanticTranslationProposed {
        cycle_id: String,
        proposal_ref: String,
        point_ref: String,
        scale_key: String,
        from: Position3,
        translation: Translation3,
        to: Position3,
        status: ProposalStatus,
    },
    SemanticTranslationConfirmed {
        cycle_id: String,
        proposal_ref: String,
        point_ref: String,
        scale_key: String,
        from: Position3,
        translation: Translation3,
        to: Position3,
        crossed_vertical_origin: bool,
        witness_ref: String,
    },
    SemanticTranslationRejected {
        cycle_id: String,
        proposal_ref: String,
        point_ref: String,
        scale_key: String,
        from: Position3,
        translation: Translation3,
        to: Position3,
        witness_ref: String,
        reasons: Vec<String>,
    },
    CentralJunctionTouched {
        cycle_id: String,
        point_ref: String,
        from: Position3,
        to: Position3,
        witness_ref: String,
    },
    CentralJunctionCrossed {
        cycle_id: String,
        point_ref: String,
        from: Position3,
        to: Position3,
        witness_ref: String,
    },
    FestivalCompleted {
        cycle_id: String,
        proposal_ref: String,
        point_ref: String,
        status: ProposalStatus,
        witness_ref: String,
        next_state_refs: Vec<String>,
    },
}

pub fn semantic_proposal_events(
    proposal: &SemanticProposal,
) -> Result<Vec<SemanticGodotEventEnvelope>, SemanticEventProjectionError> {
    let common = event_common(proposal);
    let event = match (proposal.previous_position, proposal.translation) {
        (None, None) => SemanticGodotEvent::SemanticPositionProposed {
            cycle_id: common.cycle_id,
            proposal_ref: common.proposal_ref,
            point_ref: common.point_ref,
            scale_key: common.scale_key,
            proposed_position: proposal.proposed_position,
            status: ProposalStatus::Proposed,
        },
        (Some(from), Some(translation)) => {
            hollow_grove_semantic_kernel::verify_translation(
                from,
                translation,
                proposal.proposed_position,
            )
            .map_err(|_| SemanticEventProjectionError::TranslationMismatch)?;
            SemanticGodotEvent::SemanticTranslationProposed {
                cycle_id: common.cycle_id,
                proposal_ref: common.proposal_ref,
                point_ref: common.point_ref,
                scale_key: common.scale_key,
                from,
                translation,
                to: proposal.proposed_position,
                status: ProposalStatus::Proposed,
            }
        }
        _ => return Err(SemanticEventProjectionError::IncompleteTranslation),
    };
    Ok(vec![SemanticGodotEventEnvelope::new(event)])
}

pub fn semantic_festival_events(
    proposal: &SemanticProposal,
    festival: &FestivalResult,
) -> Result<Vec<SemanticGodotEventEnvelope>, SemanticEventProjectionError> {
    let mut events = Vec::new();
    let cycle_id = proposal.cycle_id.as_str().to_owned();
    let proposal_ref = external_label(&proposal.proposal_id);
    let point_ref = external_label(&proposal.point_ref);
    let scale_key = proposal.scope.scale.as_str().to_owned();

    let (status, witness_ref) = match &festival.decision {
        SemanticDecision::Confirmed(confirmed) => {
            ensure_decision_identity(
                proposal,
                &confirmed.proposal_id,
                &confirmed.point_ref,
                &confirmed.scope,
            )?;
            let witness_ref = external_label(&confirmed.witness_ref);
            match (confirmed.previous_position, confirmed.translation) {
                (None, None) => events.push(SemanticGodotEventEnvelope::new(
                    SemanticGodotEvent::SemanticPositionConfirmed {
                        cycle_id: cycle_id.clone(),
                        proposal_ref: proposal_ref.clone(),
                        point_ref: point_ref.clone(),
                        scale_key: scale_key.clone(),
                        confirmed_position: confirmed.confirmed_position,
                        witness_ref: witness_ref.clone(),
                    },
                )),
                (Some(from), Some(translation)) => {
                    hollow_grove_semantic_kernel::verify_translation(
                        from,
                        translation,
                        confirmed.confirmed_position,
                    )
                    .map_err(|_| SemanticEventProjectionError::TranslationMismatch)?;
                    let origin_relation =
                        vertical_origin_relation(from, confirmed.confirmed_position);
                    events.push(SemanticGodotEventEnvelope::new(
                        SemanticGodotEvent::SemanticTranslationConfirmed {
                            cycle_id: cycle_id.clone(),
                            proposal_ref: proposal_ref.clone(),
                            point_ref: point_ref.clone(),
                            scale_key: scale_key.clone(),
                            from,
                            translation,
                            to: confirmed.confirmed_position,
                            crossed_vertical_origin: matches!(
                                origin_relation,
                                OriginRelation::CrossesOriginAxis
                            ),
                            witness_ref: witness_ref.clone(),
                        },
                    ));
                    if matches!(origin_relation, OriginRelation::TouchesOriginAxis) {
                        events.push(SemanticGodotEventEnvelope::new(
                            SemanticGodotEvent::CentralJunctionTouched {
                                cycle_id: cycle_id.clone(),
                                point_ref: point_ref.clone(),
                                from,
                                to: confirmed.confirmed_position,
                                witness_ref: witness_ref.clone(),
                            },
                        ));
                    } else if matches!(origin_relation, OriginRelation::CrossesOriginAxis) {
                        events.push(SemanticGodotEventEnvelope::new(
                            SemanticGodotEvent::CentralJunctionCrossed {
                                cycle_id: cycle_id.clone(),
                                point_ref: point_ref.clone(),
                                from,
                                to: confirmed.confirmed_position,
                                witness_ref: witness_ref.clone(),
                            },
                        ));
                    }
                }
                _ => return Err(SemanticEventProjectionError::IncompleteTranslation),
            }
            (ProposalStatus::Confirmed, witness_ref)
        }
        SemanticDecision::Rejected(rejected) => {
            ensure_decision_identity(
                proposal,
                &rejected.proposal_id,
                &rejected.point_ref,
                &rejected.scope,
            )?;
            let witness_ref = external_label(&rejected.witness_ref);
            let reasons = rejected
                .reasons
                .iter()
                .map(rejection_reason)
                .collect::<Vec<_>>();
            match (proposal.previous_position, proposal.translation) {
                (None, None) => events.push(SemanticGodotEventEnvelope::new(
                    SemanticGodotEvent::SemanticPositionRejected {
                        cycle_id: cycle_id.clone(),
                        proposal_ref: proposal_ref.clone(),
                        point_ref: point_ref.clone(),
                        scale_key: scale_key.clone(),
                        proposed_position: proposal.proposed_position,
                        witness_ref: witness_ref.clone(),
                        reasons,
                    },
                )),
                (Some(from), Some(translation)) => events.push(SemanticGodotEventEnvelope::new(
                    SemanticGodotEvent::SemanticTranslationRejected {
                        cycle_id: cycle_id.clone(),
                        proposal_ref: proposal_ref.clone(),
                        point_ref: point_ref.clone(),
                        scale_key: scale_key.clone(),
                        from,
                        translation,
                        to: proposal.proposed_position,
                        witness_ref: witness_ref.clone(),
                        reasons,
                    },
                )),
                _ => return Err(SemanticEventProjectionError::IncompleteTranslation),
            }
            (ProposalStatus::Rejected, witness_ref)
        }
    };

    events.push(SemanticGodotEventEnvelope::new(
        SemanticGodotEvent::FestivalCompleted {
            cycle_id,
            proposal_ref,
            point_ref,
            status,
            witness_ref,
            next_state_refs: festival
                .next_state_refs
                .iter()
                .map(external_label)
                .collect(),
        },
    ));
    Ok(events)
}

struct EventCommon {
    cycle_id: String,
    proposal_ref: String,
    point_ref: String,
    scale_key: String,
}

fn event_common(proposal: &SemanticProposal) -> EventCommon {
    EventCommon {
        cycle_id: proposal.cycle_id.as_str().to_owned(),
        proposal_ref: external_label(&proposal.proposal_id),
        point_ref: external_label(&proposal.point_ref),
        scale_key: proposal.scope.scale.as_str().to_owned(),
    }
}

fn ensure_decision_identity(
    proposal: &SemanticProposal,
    decision_proposal_ref: &ExternalRef,
    decision_point_ref: &ExternalRef,
    decision_scope: &hollow_grove_semantic_kernel::SemanticScope,
) -> Result<(), SemanticEventProjectionError> {
    if decision_proposal_ref != &proposal.proposal_id
        || decision_point_ref != &proposal.point_ref
        || decision_scope != &proposal.scope
    {
        return Err(SemanticEventProjectionError::DecisionIdentityMismatch);
    }
    Ok(())
}

fn rejection_reason(reason: &SemanticRejectionReason) -> String {
    format!("{reason:?}")
}

fn external_label(reference: &ExternalRef) -> String {
    format!("{}.{}", reference.namespace, reference.key)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemanticEventProjectionError {
    IncompleteTranslation,
    TranslationMismatch,
    DecisionIdentityMismatch,
}

impl fmt::Display for SemanticEventProjectionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "semantic event projection failed: {self:?}")
    }
}

impl std::error::Error for SemanticEventProjectionError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemanticEventDecodeError {
    MalformedJson(String),
    UnsupportedSchema(String),
}

impl fmt::Display for SemanticEventDecodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "semantic event decode failed: {self:?}")
    }
}

impl std::error::Error for SemanticEventDecodeError {}
