use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventMetadata {
    pub id: ConstitutionalEventId,
    pub causal_position: CausalPosition,
    pub rule_set: RuleSetId,
}

/// Append-only constitutional runtime. Aggregate state is a replayable cache;
/// events and caller-controlled identities are the authority.
#[derive(Debug, Clone, Default)]
pub struct ConstitutionalRuntime {
    waves: BTreeMap<WaveId, WaveRecord>,
    events: Vec<ConstitutionalEvent>,
    event_ids: BTreeSet<ConstitutionalEventId>,
    bonds: BTreeMap<BondId, BondAggregate>,
    tombstones: BTreeMap<TombstoneId, BondId>,
    tokes: BTreeMap<TokeId, TombstoneId>,
}

impl ConstitutionalRuntime {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_wave(&mut self, wave: WaveRecord) -> Result<(), ConstitutionalRuntimeError> {
        if let Some(existing) = self.waves.get(&wave.id) {
            return if existing == &wave {
                Ok(())
            } else {
                Err(ConstitutionalRuntimeError::WaveIdConflict(wave.id))
            };
        }
        self.waves.insert(wave.id.clone(), wave);
        Ok(())
    }

    pub fn append(
        &mut self,
        bond: BondId,
        metadata: EventMetadata,
        payload: BondEvent,
    ) -> Result<&ConstitutionalEvent, ConstitutionalRuntimeError> {
        if self.event_ids.contains(&metadata.id) {
            let existing = self
                .events
                .iter()
                .find(|event| event.id == metadata.id)
                .expect("the event ID index and event log must agree");
            return if existing.bond == bond
                && existing.causal_position == metadata.causal_position
                && existing.rule_set == metadata.rule_set
                && existing.payload == payload
            {
                Ok(existing)
            } else {
                Err(ConstitutionalRuntimeError::EventIdConflict(metadata.id))
            };
        }
        self.validate_wave_references(&payload, metadata.causal_position)?;
        self.validate_parent_references(&bond, &payload)?;
        self.validate_global_identities(&payload)?;
        if let BondEvent::TombstoneValidated(validation) = &payload {
            let expected = constitutional_bond_replay_digest(self, &bond)?;
            if validation.replay_digest != expected {
                return Err(ConstitutionalRuntimeError::ReplayDigestMismatch {
                    expected,
                    actual: validation.replay_digest.clone(),
                });
            }
        }

        let sequence = self.bonds.get(&bond).map_or(Ok(0), |aggregate| {
            aggregate
                .last_sequence()
                .checked_add(1)
                .ok_or(ConstitutionalRuntimeError::SequenceOverflow)
        })?;
        let event = ConstitutionalEvent {
            id: metadata.id,
            bond: bond.clone(),
            sequence,
            causal_position: metadata.causal_position,
            rule_set: metadata.rule_set,
            payload,
        };

        let aggregate = if let Some(existing) = self.bonds.get(&bond) {
            let mut candidate = existing.clone();
            candidate.apply(&event)?;
            candidate
        } else {
            BondAggregate::replay(std::iter::once(&event))?
        };

        self.index_projection(&event)?;
        self.event_ids.insert(event.id.clone());
        self.events.push(event);
        self.bonds.insert(bond, aggregate);
        Ok(self.events.last().expect("event was just appended"))
    }

    fn validate_wave_references(
        &self,
        payload: &BondEvent,
        at: CausalPosition,
    ) -> Result<(), ConstitutionalRuntimeError> {
        let wave = match payload {
            BondEvent::Formed(formation) => Some(&formation.initiating_wave),
            BondEvent::CurrentMoved(transaction) => Some(&transaction.wave),
            _ => None,
        };
        if let Some(wave) = wave {
            let record = self
                .waves
                .get(wave)
                .ok_or_else(|| ConstitutionalRuntimeError::UnknownWave(wave.clone()))?;
            if record.causal_position >= at {
                return Err(ConstitutionalRuntimeError::WaveDoesNotPrecedeEvent {
                    wave: wave.clone(),
                    wave_position: record.causal_position,
                    event_position: at,
                });
            }
        }
        Ok(())
    }

    fn validate_parent_references(
        &self,
        bond: &BondId,
        payload: &BondEvent,
    ) -> Result<(), ConstitutionalRuntimeError> {
        let BondEvent::Formed(formation) = payload else {
            return Ok(());
        };
        for parent_id in &formation.parent_bonds {
            let parent = self
                .bonds
                .get(parent_id)
                .ok_or_else(|| ConstitutionalRuntimeError::UnknownParentBond(parent_id.clone()))?;
            let resolution = parent.resolution().ok_or_else(|| {
                ConstitutionalRuntimeError::UnresolvedParentBond(parent_id.clone())
            })?;
            if !resolution
                .successor_bonds
                .iter()
                .any(|successor| successor == bond)
            {
                return Err(ConstitutionalRuntimeError::UnreservedSuccessor {
                    parent: parent_id.clone(),
                    successor: bond.clone(),
                });
            }
        }
        Ok(())
    }

    fn validate_global_identities(
        &self,
        payload: &BondEvent,
    ) -> Result<(), ConstitutionalRuntimeError> {
        match payload {
            BondEvent::TombstoneFormed(tombstone)
                if self.tombstones.contains_key(&tombstone.id) =>
            {
                Err(ConstitutionalRuntimeError::DuplicateTombstone(
                    tombstone.id.clone(),
                ))
            }
            BondEvent::TokeRecorded(toke) if self.tokes.contains_key(&toke.id) => {
                Err(ConstitutionalRuntimeError::DuplicateToke(toke.id.clone()))
            }
            _ => Ok(()),
        }
    }

    fn index_projection(
        &mut self,
        event: &ConstitutionalEvent,
    ) -> Result<(), ConstitutionalRuntimeError> {
        match &event.payload {
            BondEvent::TombstoneFormed(tombstone) => {
                self.tombstones
                    .insert(tombstone.id.clone(), event.bond.clone());
            }
            BondEvent::TokeRecorded(toke) => {
                if !self.tombstones.contains_key(&toke.tombstone) {
                    return Err(ConstitutionalRuntimeError::UnknownTombstone(
                        toke.tombstone.clone(),
                    ));
                }
                self.tokes.insert(toke.id.clone(), toke.tombstone.clone());
            }
            _ => {}
        }
        Ok(())
    }

    pub fn accumulate_current(
        &mut self,
        bond: &BondId,
        metadata: EventMetadata,
    ) -> Result<&ConstitutionalEvent, ConstitutionalRuntimeError> {
        let aggregate = self
            .bonds
            .get(bond)
            .ok_or_else(|| ConstitutionalRuntimeError::UnknownBond(bond.clone()))?;
        let accumulation = CurrentAccumulation {
            accounting: aggregate.current_accounting()?,
            through_transaction: aggregate.last_transaction_id().cloned(),
        };
        self.append(
            bond.clone(),
            metadata,
            BondEvent::CurrentAccumulated(accumulation),
        )
    }

    pub fn evaluate(
        &mut self,
        bond: &BondId,
        metadata: EventMetadata,
        evaluation_id: EvaluationId,
        evidence: Vec<EvidenceRef>,
    ) -> Result<&ConstitutionalEvent, ConstitutionalRuntimeError> {
        let evaluation = self
            .bonds
            .get(bond)
            .ok_or_else(|| ConstitutionalRuntimeError::UnknownBond(bond.clone()))?
            .calculated_evaluation(evaluation_id, evidence)?;
        self.append(bond.clone(), metadata, BondEvent::Evaluated(evaluation))
    }

    pub fn calculate_excess(
        &mut self,
        bond: &BondId,
        metadata: EventMetadata,
    ) -> Result<&ConstitutionalEvent, ConstitutionalRuntimeError> {
        let excess = self
            .bonds
            .get(bond)
            .ok_or_else(|| ConstitutionalRuntimeError::UnknownBond(bond.clone()))?
            .calculated_excess()?;
        self.append(bond.clone(), metadata, BondEvent::ExcessCalculated(excess))
    }

    pub fn replay(
        waves: impl IntoIterator<Item = WaveRecord>,
        events: impl IntoIterator<Item = ConstitutionalEvent>,
    ) -> Result<Self, ConstitutionalRuntimeError> {
        let mut runtime = Self::new();
        for wave in waves {
            runtime.record_wave(wave)?;
        }
        for event in events {
            let expected = runtime.append(
                event.bond.clone(),
                EventMetadata {
                    id: event.id.clone(),
                    causal_position: event.causal_position,
                    rule_set: event.rule_set.clone(),
                },
                event.payload.clone(),
            )?;
            if expected.sequence != event.sequence {
                return Err(ConstitutionalRuntimeError::ReplaySequenceMismatch {
                    expected: expected.sequence,
                    actual: event.sequence,
                });
            }
        }
        Ok(runtime)
    }

    #[must_use]
    pub fn bond(&self, id: &BondId) -> Option<&BondAggregate> {
        self.bonds.get(id)
    }

    #[must_use]
    pub fn wave(&self, id: &WaveId) -> Option<&WaveRecord> {
        self.waves.get(id)
    }

    #[must_use]
    pub fn events(&self) -> &[ConstitutionalEvent] {
        &self.events
    }

    pub fn waves(&self) -> impl Iterator<Item = &WaveRecord> {
        self.waves.values()
    }

    pub fn events_for<'a>(
        &'a self,
        bond: &'a BondId,
    ) -> impl Iterator<Item = &'a ConstitutionalEvent> {
        self.events.iter().filter(move |event| &event.bond == bond)
    }

    #[must_use]
    pub fn tombstone_bond(&self, id: &TombstoneId) -> Option<&BondId> {
        self.tombstones.get(id)
    }

    #[must_use]
    pub fn toke_tombstone(&self, id: &TokeId) -> Option<&TombstoneId> {
        self.tokes.get(id)
    }

    /// Verifies that every successor reserved by a resolved Bond has formed
    /// and names that Bond as a parent. Partial live histories may defer this
    /// check; finalized archives must pass it.
    pub fn verify_successor_integrity(&self) -> Result<(), ConstitutionalRuntimeError> {
        for (parent_id, parent) in &self.bonds {
            let Some(resolution) = parent.resolution() else {
                continue;
            };
            for successor_id in &resolution.successor_bonds {
                let successor = self.bonds.get(successor_id).ok_or_else(|| {
                    ConstitutionalRuntimeError::MissingReservedSuccessor {
                        parent: parent_id.clone(),
                        successor: successor_id.clone(),
                    }
                })?;
                if !successor
                    .formation()
                    .parent_bonds
                    .iter()
                    .any(|candidate| candidate == parent_id)
                {
                    return Err(ConstitutionalRuntimeError::SuccessorMissingParent {
                        parent: parent_id.clone(),
                        successor: successor_id.clone(),
                    });
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConstitutionalRuntimeError {
    WaveIdConflict(WaveId),
    UnknownWave(WaveId),
    WaveDoesNotPrecedeEvent {
        wave: WaveId,
        wave_position: CausalPosition,
        event_position: CausalPosition,
    },
    EventIdConflict(ConstitutionalEventId),
    UnknownBond(BondId),
    UnknownParentBond(BondId),
    UnresolvedParentBond(BondId),
    UnreservedSuccessor {
        parent: BondId,
        successor: BondId,
    },
    MissingReservedSuccessor {
        parent: BondId,
        successor: BondId,
    },
    SuccessorMissingParent {
        parent: BondId,
        successor: BondId,
    },
    DuplicateTombstone(TombstoneId),
    UnknownTombstone(TombstoneId),
    DuplicateToke(TokeId),
    SequenceOverflow,
    ReplaySequenceMismatch {
        expected: u64,
        actual: u64,
    },
    ReplayDigestMismatch {
        expected: String,
        actual: String,
    },
    Archive(String),
    Bond(BondStateError),
}

impl From<BondStateError> for ConstitutionalRuntimeError {
    fn from(value: BondStateError) -> Self {
        Self::Bond(value)
    }
}

impl From<ConstitutionalArchiveError> for ConstitutionalRuntimeError {
    fn from(value: ConstitutionalArchiveError) -> Self {
        Self::Archive(value.to_string())
    }
}

impl fmt::Display for ConstitutionalRuntimeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "constitutional runtime error: {self:?}")
    }
}

impl std::error::Error for ConstitutionalRuntimeError {}
