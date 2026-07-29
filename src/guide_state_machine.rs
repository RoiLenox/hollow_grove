//! Typed, durable Guide crossing state machine.
//!
//! V1 records remain readable, while this module is the authoritative path for
//! new crossings. Its public wrappers expose only canonical outside, inside,
//! entering, and leaving states.

use std::collections::BTreeSet;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::artifact_io::write_bytes_atomically;
use crate::composition::ExternalRef;
use crate::guide_identity::{
    CENTRAL_JUNCTION_ORIGIN, DualGuideSession, GuideContextV1, GuideFestivalDecision,
    GuidePresenceError, GuideTransitionDirection, GuideTransitionError, GuideTransitionWitness,
    JunctionTransitionState, PairedGuideTransition, PairedGuideTransitionOutcome,
    RejectedGuideTransition, StableTransitionId, WorldLayer, perform_paired_guide_transition,
    stage_paired_guide_transition,
};

pub const GUIDE_STATE_MACHINE_SCHEMA_VERSION: &str = "hollow-grove-guide-state-machine-v2";
pub const GUIDE_TRANSITION_EVENT_SCHEMA_VERSION: &str = "hollow-grove-guide-transition-event-v1";
pub const DEFAULT_GUIDE_TRANSITION_JOURNAL_DIR: &str = "artifacts/guide-transitions-v2";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutsideDualSeerSession {
    session: DualGuideSession,
}

impl OutsideDualSeerSession {
    pub fn new(session: DualGuideSession) -> Result<Self, GuideStateMachineError> {
        validate_stable_session(&session, WorldLayer::OutsideHollowGrove)?;
        Ok(Self { session })
    }

    #[must_use]
    pub const fn as_session(&self) -> &DualGuideSession {
        &self.session
    }

    #[must_use]
    pub fn into_session(self) -> DualGuideSession {
        self.session
    }

    pub fn begin_entry(
        self,
        transition: PairedGuideTransition,
    ) -> Result<EnteringAtJunction, GuideStateMachineError> {
        if transition.direction != GuideTransitionDirection::EnterHollowGrove {
            return Err(GuideStateMachineError::WrongTransitionDirection {
                expected: GuideTransitionDirection::EnterHollowGrove,
                actual: transition.direction,
            });
        }
        let mut staged_session = self.session.clone();
        stage_paired_guide_transition(&mut staged_session, &transition)?;
        Ok(EnteringAtJunction {
            source: self,
            staged_session,
            transition,
            junction_state: JunctionTransitionState::Approaching,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InsideDualGuideSession {
    session: DualGuideSession,
}

impl InsideDualGuideSession {
    pub fn new(session: DualGuideSession) -> Result<Self, GuideStateMachineError> {
        validate_stable_session(&session, WorldLayer::HollowGrove)?;
        Ok(Self { session })
    }

    #[must_use]
    pub const fn as_session(&self) -> &DualGuideSession {
        &self.session
    }

    #[must_use]
    pub fn into_session(self) -> DualGuideSession {
        self.session
    }

    pub fn begin_exit(
        self,
        transition: PairedGuideTransition,
    ) -> Result<LeavingAtJunction, GuideStateMachineError> {
        if transition.direction != GuideTransitionDirection::LeaveHollowGrove {
            return Err(GuideStateMachineError::WrongTransitionDirection {
                expected: GuideTransitionDirection::LeaveHollowGrove,
                actual: transition.direction,
            });
        }
        let mut staged_session = self.session.clone();
        stage_paired_guide_transition(&mut staged_session, &transition)?;
        Ok(LeavingAtJunction {
            source: self,
            staged_session,
            transition,
            junction_state: JunctionTransitionState::Approaching,
        })
    }
}

fn validate_stable_session(
    session: &DualGuideSession,
    expected_layer: WorldLayer,
) -> Result<(), GuideStateMachineError> {
    session.validate()?;
    if session.current_layer != expected_layer {
        return Err(GuideStateMachineError::WrongStableLayer {
            expected: expected_layer,
            actual: session.current_layer,
        });
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnteringAtJunction {
    source: OutsideDualSeerSession,
    staged_session: DualGuideSession,
    transition: PairedGuideTransition,
    junction_state: JunctionTransitionState,
}

impl EnteringAtJunction {
    #[must_use]
    pub const fn source(&self) -> &OutsideDualSeerSession {
        &self.source
    }

    #[must_use]
    pub const fn transition(&self) -> &PairedGuideTransition {
        &self.transition
    }

    #[must_use]
    pub const fn junction_state(&self) -> JunctionTransitionState {
        self.junction_state
    }

    pub fn at_origin(mut self) -> Result<Self, GuideStateMachineError> {
        advance_junction_state(
            &mut self.junction_state,
            JunctionTransitionState::Approaching,
            JunctionTransitionState::AtOrigin,
        )?;
        Ok(self)
    }

    pub fn form_changing(mut self) -> Result<Self, GuideStateMachineError> {
        advance_junction_state(
            &mut self.junction_state,
            JunctionTransitionState::AtOrigin,
            JunctionTransitionState::FormChanging,
        )?;
        Ok(self)
    }

    pub fn guide_context_v1(&self) -> Result<GuideContextV1, GuideStateMachineError> {
        Ok(GuideContextV1::during_transition(
            &self.staged_session,
            &self.transition,
            self.junction_state,
        )?)
    }

    pub fn confirm(self) -> EntryFestivalOutcome {
        let Self {
            source,
            mut staged_session,
            transition,
            ..
        } = self;
        match perform_paired_guide_transition(&mut staged_session, &transition) {
            PairedGuideTransitionOutcome::Confirmed(witness) => {
                match InsideDualGuideSession::new(staged_session) {
                    Ok(session) => EntryFestivalOutcome::Confirmed { session, witness },
                    Err(error) => EntryFestivalOutcome::Rejected {
                        session: source,
                        rejection: RejectedGuideTransition {
                            transition_id: transition.transition_id,
                            direction: transition.direction,
                            session_ref: transition.session_ref,
                            decision: GuideFestivalDecision::Rejected,
                            reasons: vec![GuideTransitionError::InvalidPresence(
                                GuidePresenceError::Serialization(error.to_string()),
                            )],
                        },
                    },
                }
            }
            PairedGuideTransitionOutcome::Rejected(rejection) => EntryFestivalOutcome::Rejected {
                session: source,
                rejection,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeavingAtJunction {
    source: InsideDualGuideSession,
    staged_session: DualGuideSession,
    transition: PairedGuideTransition,
    junction_state: JunctionTransitionState,
}

impl LeavingAtJunction {
    #[must_use]
    pub const fn source(&self) -> &InsideDualGuideSession {
        &self.source
    }

    #[must_use]
    pub const fn transition(&self) -> &PairedGuideTransition {
        &self.transition
    }

    #[must_use]
    pub const fn junction_state(&self) -> JunctionTransitionState {
        self.junction_state
    }

    pub fn at_origin(mut self) -> Result<Self, GuideStateMachineError> {
        advance_junction_state(
            &mut self.junction_state,
            JunctionTransitionState::Approaching,
            JunctionTransitionState::AtOrigin,
        )?;
        Ok(self)
    }

    pub fn form_changing(mut self) -> Result<Self, GuideStateMachineError> {
        advance_junction_state(
            &mut self.junction_state,
            JunctionTransitionState::AtOrigin,
            JunctionTransitionState::FormChanging,
        )?;
        Ok(self)
    }

    pub fn guide_context_v1(&self) -> Result<GuideContextV1, GuideStateMachineError> {
        Ok(GuideContextV1::during_transition(
            &self.staged_session,
            &self.transition,
            self.junction_state,
        )?)
    }

    pub fn confirm(self) -> ExitFestivalOutcome {
        let Self {
            source,
            mut staged_session,
            transition,
            ..
        } = self;
        match perform_paired_guide_transition(&mut staged_session, &transition) {
            PairedGuideTransitionOutcome::Confirmed(witness) => {
                match OutsideDualSeerSession::new(staged_session) {
                    Ok(session) => ExitFestivalOutcome::Confirmed { session, witness },
                    Err(error) => ExitFestivalOutcome::Rejected {
                        session: source,
                        rejection: RejectedGuideTransition {
                            transition_id: transition.transition_id,
                            direction: transition.direction,
                            session_ref: transition.session_ref,
                            decision: GuideFestivalDecision::Rejected,
                            reasons: vec![GuideTransitionError::InvalidPresence(
                                GuidePresenceError::Serialization(error.to_string()),
                            )],
                        },
                    },
                }
            }
            PairedGuideTransitionOutcome::Rejected(rejection) => ExitFestivalOutcome::Rejected {
                session: source,
                rejection,
            },
        }
    }
}

fn advance_junction_state(
    current: &mut JunctionTransitionState,
    expected: JunctionTransitionState,
    next: JunctionTransitionState,
) -> Result<(), GuideStateMachineError> {
    if *current != expected {
        return Err(GuideStateMachineError::InvalidJunctionAdvance {
            expected,
            actual: *current,
            requested: next,
        });
    }
    *current = next;
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntryFestivalOutcome {
    Confirmed {
        session: InsideDualGuideSession,
        witness: Box<GuideTransitionWitness>,
    },
    Rejected {
        session: OutsideDualSeerSession,
        rejection: RejectedGuideTransition,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExitFestivalOutcome {
    Confirmed {
        session: OutsideDualSeerSession,
        witness: Box<GuideTransitionWitness>,
    },
    Rejected {
        session: InsideDualGuideSession,
        rejection: RejectedGuideTransition,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CanonicalGuideSession {
    Outside(OutsideDualSeerSession),
    Inside(InsideDualGuideSession),
}

impl CanonicalGuideSession {
    pub fn restore(session: DualGuideSession) -> Result<Self, GuideStateMachineError> {
        match session.current_layer {
            WorldLayer::OutsideHollowGrove => {
                Ok(Self::Outside(OutsideDualSeerSession::new(session)?))
            }
            WorldLayer::HollowGrove => Ok(Self::Inside(InsideDualGuideSession::new(session)?)),
            WorldLayer::CentralJunction => Err(GuideStateMachineError::UnfinishedJunctionSession),
        }
    }

    #[must_use]
    pub const fn as_session(&self) -> &DualGuideSession {
        match self {
            Self::Outside(session) => session.as_session(),
            Self::Inside(session) => session.as_session(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GuideTransitionEventKind {
    Proposed,
    JournalPrepared,
    Approaching,
    AtOrigin,
    FormChanging,
    Confirmed,
    Rejected,
    Recovered,
    Replayed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GuideTransitionEvent {
    pub schema_version: String,
    pub transition_ref: ExternalRef,
    pub session_ref: ExternalRef,
    pub direction: GuideTransitionDirection,
    pub kind: GuideTransitionEventKind,
    pub junction_origin: Option<crate::composition::Position3>,
    pub guide_context: GuideContextV1,
}

impl GuideTransitionEvent {
    fn stable(
        transition: &PairedGuideTransition,
        kind: GuideTransitionEventKind,
        guide_context: GuideContextV1,
    ) -> Self {
        Self {
            schema_version: GUIDE_TRANSITION_EVENT_SCHEMA_VERSION.into(),
            transition_ref: transition.external_ref(),
            session_ref: transition.session_ref.clone(),
            direction: transition.direction,
            kind,
            junction_origin: matches!(
                kind,
                GuideTransitionEventKind::AtOrigin | GuideTransitionEventKind::FormChanging
            )
            .then_some(CENTRAL_JUNCTION_ORIGIN),
            guide_context,
        }
    }

    pub fn validate(&self) -> Result<(), GuideStateMachineError> {
        if self.schema_version != GUIDE_TRANSITION_EVENT_SCHEMA_VERSION {
            return Err(GuideStateMachineError::UnsupportedEventSchema(
                self.schema_version.clone(),
            ));
        }
        self.guide_context.validate()?;
        if self.session_ref != self.guide_context.session_ref {
            return Err(GuideStateMachineError::EventSessionMismatch);
        }
        if matches!(
            self.kind,
            GuideTransitionEventKind::AtOrigin | GuideTransitionEventKind::FormChanging
        ) && self.junction_origin != Some(CENTRAL_JUNCTION_ORIGIN)
        {
            return Err(GuideStateMachineError::JunctionOriginMismatch);
        }
        let layer_is_valid = match self.kind {
            GuideTransitionEventKind::Proposed
            | GuideTransitionEventKind::JournalPrepared
            | GuideTransitionEventKind::Rejected
            | GuideTransitionEventKind::Recovered => {
                self.guide_context.layer == self.direction.source().1
            }
            GuideTransitionEventKind::Approaching
            | GuideTransitionEventKind::AtOrigin
            | GuideTransitionEventKind::FormChanging => {
                self.guide_context.layer == WorldLayer::CentralJunction
            }
            GuideTransitionEventKind::Confirmed => {
                self.guide_context.layer == self.direction.destination().1
            }
            GuideTransitionEventKind::Replayed => {
                self.guide_context.layer == self.direction.source().1
                    || self.guide_context.layer == self.direction.destination().1
            }
        };
        if !layer_is_valid {
            return Err(GuideStateMachineError::EventLayerMismatch {
                kind: self.kind,
                layer: self.guide_context.layer,
            });
        }
        Ok(())
    }

    pub fn render_tui(&self) -> Result<String, GuideStateMachineError> {
        self.validate()?;
        let names = self
            .guide_context
            .guides
            .iter()
            .map(|guide| guide.display_name.as_str())
            .collect::<Vec<_>>()
            .join(" + ");
        Ok(format!(
            "GUIDE CROSSING {:?}\ntransition: {}.{}\nsession: {}.{}\nactive: {names}\n",
            self.kind,
            self.transition_ref.namespace,
            self.transition_ref.key,
            self.session_ref.namespace,
            self.session_ref.key,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GuideTransitionJournalStatus {
    Prepared,
    Confirmed,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GuideTransitionJournal {
    pub schema_version: String,
    pub transition_id: StableTransitionId,
    pub status: GuideTransitionJournalStatus,
    pub source_session: DualGuideSession,
    pub transition: PairedGuideTransition,
    pub resulting_session: Option<DualGuideSession>,
    pub witness: Option<Box<GuideTransitionWitness>>,
    pub rejection: Option<RejectedGuideTransition>,
    pub events: Vec<GuideTransitionEvent>,
}

impl GuideTransitionJournal {
    fn prepared(
        source_session: DualGuideSession,
        transition: PairedGuideTransition,
    ) -> Result<Self, GuideStateMachineError> {
        let source_context = source_session.guide_context_v1()?;
        let events = vec![
            GuideTransitionEvent::stable(
                &transition,
                GuideTransitionEventKind::Proposed,
                source_context.clone(),
            ),
            GuideTransitionEvent::stable(
                &transition,
                GuideTransitionEventKind::JournalPrepared,
                source_context,
            ),
        ];
        let journal = Self {
            schema_version: GUIDE_STATE_MACHINE_SCHEMA_VERSION.into(),
            transition_id: transition.transition_id.clone(),
            status: GuideTransitionJournalStatus::Prepared,
            source_session,
            transition,
            resulting_session: None,
            witness: None,
            rejection: None,
            events,
        };
        journal.validate()?;
        Ok(journal)
    }

    pub fn validate(&self) -> Result<(), GuideStateMachineError> {
        if self.schema_version != GUIDE_STATE_MACHINE_SCHEMA_VERSION {
            return Err(GuideStateMachineError::UnsupportedJournalSchema(
                self.schema_version.clone(),
            ));
        }
        if self.transition_id != self.transition.transition_id {
            return Err(GuideStateMachineError::JournalTransitionMismatch);
        }
        self.source_session.validate()?;
        let mut staged = self.source_session.clone();
        stage_paired_guide_transition(&mut staged, &self.transition)?;
        for event in &self.events {
            event.validate()?;
            if event.transition_ref != self.transition.external_ref()
                || event.direction != self.transition.direction
            {
                return Err(GuideStateMachineError::JournalEventMismatch);
            }
        }
        let event_kinds = self
            .events
            .iter()
            .map(|event| event.kind)
            .collect::<Vec<_>>();
        if !event_kinds.starts_with(&[
            GuideTransitionEventKind::Proposed,
            GuideTransitionEventKind::JournalPrepared,
        ]) {
            return Err(GuideStateMachineError::JournalEventSequence);
        }
        match self.status {
            GuideTransitionJournalStatus::Prepared => {
                if self.resulting_session.is_some()
                    || self.witness.is_some()
                    || self.rejection.is_some()
                    || event_kinds.len() != 2
                {
                    return Err(GuideStateMachineError::InvalidPreparedJournal);
                }
            }
            GuideTransitionJournalStatus::Confirmed => {
                let resulting = self
                    .resulting_session
                    .as_ref()
                    .ok_or(GuideStateMachineError::MissingConfirmedResult)?;
                let witness = self
                    .witness
                    .as_ref()
                    .ok_or(GuideStateMachineError::MissingConfirmedResult)?;
                if self.rejection.is_some() {
                    return Err(GuideStateMachineError::MissingConfirmedResult);
                }
                resulting.validate()?;
                if resulting.session_id != self.source_session.session_id
                    || resulting.current_layer != self.transition.direction.destination().1
                    || resulting.focused_object_ref != self.source_session.focused_object_ref
                    || witness.transition_id != self.transition_id
                    || witness.session_ref != self.source_session.external_ref()
                    || witness.decision != GuideFestivalDecision::Confirmed
                    || witness.roi_transition != self.transition.roi_transition
                    || witness.roselina_transition != self.transition.roselina_transition
                    || resulting.current_witness_ref.as_ref() != Some(&witness.witness_ref)
                    || resulting.roi_state.transition_witness_ref.as_ref()
                        != Some(&witness.witness_ref)
                    || resulting.roselina_state.transition_witness_ref.as_ref()
                        != Some(&witness.witness_ref)
                {
                    return Err(GuideStateMachineError::ConfirmedResultMismatch);
                }
                if event_kinds.last() != Some(&GuideTransitionEventKind::Confirmed) {
                    return Err(GuideStateMachineError::JournalEventSequence);
                }
                witness.roi_transition.validate()?;
                witness.roselina_transition.validate()?;
            }
            GuideTransitionJournalStatus::Rejected => {
                let rejection = self
                    .rejection
                    .as_ref()
                    .ok_or(GuideStateMachineError::MissingRejectedResult)?;
                if self.resulting_session.is_some() || self.witness.is_some() {
                    return Err(GuideStateMachineError::MissingRejectedResult);
                }
                if rejection.transition_id != self.transition_id
                    || rejection.session_ref != self.source_session.external_ref()
                    || rejection.decision != GuideFestivalDecision::Rejected
                {
                    return Err(GuideStateMachineError::RejectedResultMismatch);
                }
                if event_kinds.last() != Some(&GuideTransitionEventKind::Rejected) {
                    return Err(GuideStateMachineError::JournalEventSequence);
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuideTransitionJournalStore {
    root: PathBuf,
}

impl GuideTransitionJournalStore {
    #[must_use]
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    #[must_use]
    pub fn journal_path(&self, transition_id: &StableTransitionId) -> PathBuf {
        self.root.join(format!("{}.json", transition_id.as_str()))
    }

    pub fn load(
        &self,
        transition_id: &StableTransitionId,
    ) -> Result<Option<GuideTransitionJournal>, GuideStateMachineError> {
        let path = self.journal_path(transition_id);
        let bytes = match fs::read(&path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(error) => return Err(GuideStateMachineError::Io(error.to_string())),
        };
        let journal = serde_json::from_slice::<GuideTransitionJournal>(&bytes)
            .map_err(|error| GuideStateMachineError::Serialization(error.to_string()))?;
        journal.validate()?;
        Ok(Some(journal))
    }

    fn persist(&self, journal: &GuideTransitionJournal) -> Result<(), GuideStateMachineError> {
        journal.validate()?;
        let bytes = serde_json::to_vec_pretty(journal)
            .map_err(|error| GuideStateMachineError::Serialization(error.to_string()))?;
        write_bytes_atomically(&self.journal_path(&journal.transition_id), &bytes)
            .map_err(|error| GuideStateMachineError::Io(error.to_string()))
    }

    fn prepare(
        &self,
        source_session: &DualGuideSession,
        transition: &PairedGuideTransition,
    ) -> Result<(GuideTransitionJournal, bool), GuideStateMachineError> {
        if let Some(existing) = self.load(&transition.transition_id)? {
            if existing.source_session != *source_session || existing.transition != *transition {
                return Err(GuideStateMachineError::IdempotencyConflict(
                    transition.transition_id.clone(),
                ));
            }
            return Ok((existing, true));
        }
        let journal = GuideTransitionJournal::prepared(source_session.clone(), transition.clone())?;
        self.persist(&journal)?;
        Ok((journal, false))
    }

    pub fn journals(&self) -> Result<Vec<GuideTransitionJournal>, GuideStateMachineError> {
        let entries = match fs::read_dir(&self.root) {
            Ok(entries) => entries,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(error) => return Err(GuideStateMachineError::Io(error.to_string())),
        };
        let mut paths = entries
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| {
                path.extension()
                    .is_some_and(|extension| extension == "json")
            })
            .collect::<Vec<_>>();
        paths.sort();
        paths
            .into_iter()
            .map(|path| {
                let bytes = fs::read(&path)
                    .map_err(|error| GuideStateMachineError::Io(error.to_string()))?;
                let journal = serde_json::from_slice::<GuideTransitionJournal>(&bytes)
                    .map_err(|error| GuideStateMachineError::Serialization(error.to_string()))?;
                journal.validate()?;
                Ok(journal)
            })
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PendingTypedTransition {
    Entering(EnteringAtJunction),
    Leaving(LeavingAtJunction),
}

impl PendingTypedTransition {
    fn source_session(&self) -> &DualGuideSession {
        match self {
            Self::Entering(pending) => pending.source.as_session(),
            Self::Leaving(pending) => pending.source.as_session(),
        }
    }

    fn transition(&self) -> &PairedGuideTransition {
        match self {
            Self::Entering(pending) => &pending.transition,
            Self::Leaving(pending) => &pending.transition,
        }
    }

    fn staged_session(&self) -> &DualGuideSession {
        match self {
            Self::Entering(pending) => &pending.staged_session,
            Self::Leaving(pending) => &pending.staged_session,
        }
    }

    fn into_staged_session(self) -> DualGuideSession {
        match self {
            Self::Entering(pending) => pending.staged_session,
            Self::Leaving(pending) => pending.staged_session,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DurableGuideTransitionOutcome {
    Confirmed {
        session: CanonicalGuideSession,
        witness: Box<GuideTransitionWitness>,
        events: Vec<GuideTransitionEvent>,
        replayed: bool,
        recovered: bool,
    },
    Rejected {
        session: CanonicalGuideSession,
        rejection: RejectedGuideTransition,
        events: Vec<GuideTransitionEvent>,
        replayed: bool,
        recovered: bool,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DurableGuideTransitionCoordinator {
    store: GuideTransitionJournalStore,
}

impl DurableGuideTransitionCoordinator {
    #[must_use]
    pub fn new(store: GuideTransitionJournalStore) -> Self {
        Self { store }
    }

    #[must_use]
    pub const fn store(&self) -> &GuideTransitionJournalStore {
        &self.store
    }

    pub fn prepare_entry(
        &self,
        pending: &EnteringAtJunction,
    ) -> Result<(), GuideStateMachineError> {
        self.store
            .prepare(pending.source.as_session(), &pending.transition)?;
        Ok(())
    }

    pub fn prepare_exit(&self, pending: &LeavingAtJunction) -> Result<(), GuideStateMachineError> {
        self.store
            .prepare(pending.source.as_session(), &pending.transition)?;
        Ok(())
    }

    pub fn execute_entry(
        &self,
        pending: EnteringAtJunction,
    ) -> Result<DurableGuideTransitionOutcome, GuideStateMachineError> {
        self.execute(PendingTypedTransition::Entering(pending))
    }

    pub fn execute_exit(
        &self,
        pending: LeavingAtJunction,
    ) -> Result<DurableGuideTransitionOutcome, GuideStateMachineError> {
        self.execute(PendingTypedTransition::Leaving(pending))
    }

    pub fn reject_entry(
        &self,
        pending: EnteringAtJunction,
        reason: GuideTransitionError,
    ) -> Result<DurableGuideTransitionOutcome, GuideStateMachineError> {
        self.reject(PendingTypedTransition::Entering(pending), reason)
    }

    pub fn reject_exit(
        &self,
        pending: LeavingAtJunction,
        reason: GuideTransitionError,
    ) -> Result<DurableGuideTransitionOutcome, GuideStateMachineError> {
        self.reject(PendingTypedTransition::Leaving(pending), reason)
    }

    pub fn recover(
        &self,
        transition_id: &StableTransitionId,
    ) -> Result<DurableGuideTransitionOutcome, GuideStateMachineError> {
        let journal = self
            .store
            .load(transition_id)?
            .ok_or_else(|| GuideStateMachineError::JournalNotFound(transition_id.clone()))?;
        if journal.status != GuideTransitionJournalStatus::Prepared {
            return self.outcome_from_terminal(journal, true, false);
        }
        let pending = match journal.transition.direction {
            GuideTransitionDirection::EnterHollowGrove => {
                let source = OutsideDualSeerSession::new(journal.source_session.clone())?;
                PendingTypedTransition::Entering(source.begin_entry(journal.transition.clone())?)
            }
            GuideTransitionDirection::LeaveHollowGrove => {
                let source = InsideDualGuideSession::new(journal.source_session.clone())?;
                PendingTypedTransition::Leaving(source.begin_exit(journal.transition.clone())?)
            }
        };
        self.execute_with_recovery(pending, true)
    }

    fn execute(
        &self,
        pending: PendingTypedTransition,
    ) -> Result<DurableGuideTransitionOutcome, GuideStateMachineError> {
        self.execute_with_recovery(pending, false)
    }

    fn reject(
        &self,
        pending: PendingTypedTransition,
        reason: GuideTransitionError,
    ) -> Result<DurableGuideTransitionOutcome, GuideStateMachineError> {
        let source_session = pending.source_session().clone();
        let transition = pending.transition().clone();
        let (mut journal, existed) = self.store.prepare(&source_session, &transition)?;
        if journal.status != GuideTransitionJournalStatus::Prepared {
            return self.outcome_from_terminal(journal, true, false);
        }
        if existed {
            journal.events.push(GuideTransitionEvent::stable(
                &transition,
                GuideTransitionEventKind::Recovered,
                source_session.guide_context_v1()?,
            ));
        }
        let rejection = RejectedGuideTransition {
            transition_id: transition.transition_id.clone(),
            direction: transition.direction,
            session_ref: transition.session_ref.clone(),
            decision: GuideFestivalDecision::Rejected,
            reasons: vec![reason],
        };
        journal.status = GuideTransitionJournalStatus::Rejected;
        journal.rejection = Some(rejection.clone());
        journal.events.push(GuideTransitionEvent::stable(
            &transition,
            GuideTransitionEventKind::Rejected,
            source_session.guide_context_v1()?,
        ));
        self.store.persist(&journal)?;
        Ok(DurableGuideTransitionOutcome::Rejected {
            session: CanonicalGuideSession::restore(source_session)?,
            rejection,
            events: journal.events,
            replayed: false,
            recovered: existed,
        })
    }

    fn execute_with_recovery(
        &self,
        pending: PendingTypedTransition,
        recovery_requested: bool,
    ) -> Result<DurableGuideTransitionOutcome, GuideStateMachineError> {
        let source_session = pending.source_session().clone();
        let transition = pending.transition().clone();
        let (mut journal, existed) = self.store.prepare(&source_session, &transition)?;
        if journal.status != GuideTransitionJournalStatus::Prepared {
            return self.outcome_from_terminal(journal, true, false);
        }
        let recovered = recovery_requested || existed;
        if recovered {
            journal.events.push(GuideTransitionEvent::stable(
                &transition,
                GuideTransitionEventKind::Recovered,
                source_session.guide_context_v1()?,
            ));
        }
        for (kind, state) in [
            (
                GuideTransitionEventKind::Approaching,
                JunctionTransitionState::Approaching,
            ),
            (
                GuideTransitionEventKind::AtOrigin,
                JunctionTransitionState::AtOrigin,
            ),
            (
                GuideTransitionEventKind::FormChanging,
                JunctionTransitionState::FormChanging,
            ),
        ] {
            journal.events.push(GuideTransitionEvent::stable(
                &transition,
                kind,
                GuideContextV1::during_transition(pending.staged_session(), &transition, state)?,
            ));
        }

        let mut staged_session = pending.into_staged_session();
        match perform_paired_guide_transition(&mut staged_session, &transition) {
            PairedGuideTransitionOutcome::Confirmed(witness) => {
                let session = CanonicalGuideSession::restore(staged_session.clone())?;
                journal.status = GuideTransitionJournalStatus::Confirmed;
                journal.resulting_session = Some(staged_session.clone());
                journal.witness = Some(witness.clone());
                journal.events.push(GuideTransitionEvent::stable(
                    &transition,
                    GuideTransitionEventKind::Confirmed,
                    staged_session.guide_context_v1()?,
                ));
                self.store.persist(&journal)?;
                Ok(DurableGuideTransitionOutcome::Confirmed {
                    session,
                    witness,
                    events: journal.events,
                    replayed: false,
                    recovered,
                })
            }
            PairedGuideTransitionOutcome::Rejected(rejection) => {
                journal.status = GuideTransitionJournalStatus::Rejected;
                journal.rejection = Some(rejection.clone());
                journal.events.push(GuideTransitionEvent::stable(
                    &transition,
                    GuideTransitionEventKind::Rejected,
                    source_session.guide_context_v1()?,
                ));
                self.store.persist(&journal)?;
                Ok(DurableGuideTransitionOutcome::Rejected {
                    session: CanonicalGuideSession::restore(source_session)?,
                    rejection,
                    events: journal.events,
                    replayed: false,
                    recovered,
                })
            }
        }
    }

    fn outcome_from_terminal(
        &self,
        mut journal: GuideTransitionJournal,
        replayed: bool,
        recovered: bool,
    ) -> Result<DurableGuideTransitionOutcome, GuideStateMachineError> {
        journal.validate()?;
        let context = journal
            .resulting_session
            .as_ref()
            .unwrap_or(&journal.source_session)
            .guide_context_v1()?;
        journal.events.push(GuideTransitionEvent::stable(
            &journal.transition,
            GuideTransitionEventKind::Replayed,
            context,
        ));
        match journal.status {
            GuideTransitionJournalStatus::Confirmed => {
                Ok(DurableGuideTransitionOutcome::Confirmed {
                    session: CanonicalGuideSession::restore(
                        journal
                            .resulting_session
                            .ok_or(GuideStateMachineError::MissingConfirmedResult)?,
                    )?,
                    witness: journal
                        .witness
                        .ok_or(GuideStateMachineError::MissingConfirmedResult)?,
                    events: journal.events,
                    replayed,
                    recovered,
                })
            }
            GuideTransitionJournalStatus::Rejected => Ok(DurableGuideTransitionOutcome::Rejected {
                session: CanonicalGuideSession::restore(journal.source_session)?,
                rejection: journal
                    .rejection
                    .ok_or(GuideStateMachineError::MissingRejectedResult)?,
                events: journal.events,
                replayed,
                recovered,
            }),
            GuideTransitionJournalStatus::Prepared => {
                Err(GuideStateMachineError::IncompletePreparedJournal)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GuideAuditSeverity {
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GuideAuditFinding {
    pub severity: GuideAuditSeverity,
    pub code: String,
    pub subject: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GuideInvariantAuditReport {
    pub schema_version: String,
    pub journal_root: String,
    pub journal_count: usize,
    pub prepared_count: usize,
    pub confirmed_count: usize,
    pub rejected_count: usize,
    pub findings: Vec<GuideAuditFinding>,
}

impl GuideInvariantAuditReport {
    #[must_use]
    pub fn is_clean(&self) -> bool {
        !self
            .findings
            .iter()
            .any(|finding| finding.severity == GuideAuditSeverity::Error)
    }

    #[must_use]
    pub fn render_tui(&self) -> String {
        let result = if self.is_clean() { "PASS" } else { "FAIL" };
        let mut output = format!(
            "GUIDE STATE AUDIT — {result}\n\n\
             journal root: {}\n\
             journals: {}\n\
             prepared: {}\n\
             confirmed: {}\n\
             rejected: {}\n\
             stable identities: guide.roi, guide.roselina\n\
             Central Junction: [0,0,0]\n",
            self.journal_root,
            self.journal_count,
            self.prepared_count,
            self.confirmed_count,
            self.rejected_count,
        );
        if self.findings.is_empty() {
            output.push_str("findings: none\n");
        } else {
            output.push_str("\nfindings:\n");
            for finding in &self.findings {
                output.push_str(&format!(
                    "- {:?} {} [{}]: {}\n",
                    finding.severity, finding.code, finding.subject, finding.message
                ));
            }
        }
        output
    }
}

#[must_use]
pub fn audit_guide_transition_store(root: &Path) -> GuideInvariantAuditReport {
    let mut report = GuideInvariantAuditReport {
        schema_version: GUIDE_STATE_MACHINE_SCHEMA_VERSION.into(),
        journal_root: root.display().to_string(),
        journal_count: 0,
        prepared_count: 0,
        confirmed_count: 0,
        rejected_count: 0,
        findings: Vec::new(),
    };
    let entries = match fs::read_dir(root) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return report,
        Err(error) => {
            report.findings.push(GuideAuditFinding {
                severity: GuideAuditSeverity::Error,
                code: "journal_root_unreadable".into(),
                subject: root.display().to_string(),
                message: error.to_string(),
            });
            return report;
        }
    };
    let mut seen = BTreeSet::new();
    let mut paths = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "json")
        })
        .collect::<Vec<_>>();
    paths.sort();
    for path in paths {
        report.journal_count += 1;
        let subject = path.display().to_string();
        let journal = fs::read(&path)
            .map_err(|error| error.to_string())
            .and_then(|bytes| {
                serde_json::from_slice::<GuideTransitionJournal>(&bytes)
                    .map_err(|error| error.to_string())
            });
        let journal = match journal {
            Ok(journal) => journal,
            Err(message) => {
                report.findings.push(GuideAuditFinding {
                    severity: GuideAuditSeverity::Error,
                    code: "journal_unreadable".into(),
                    subject,
                    message,
                });
                continue;
            }
        };
        if !seen.insert(journal.transition_id.clone()) {
            report.findings.push(GuideAuditFinding {
                severity: GuideAuditSeverity::Error,
                code: "duplicate_transition_id".into(),
                subject: journal.transition_id.to_string(),
                message: "more than one journal claims this stable transition identity".into(),
            });
        }
        match journal.status {
            GuideTransitionJournalStatus::Prepared => {
                report.prepared_count += 1;
                report.findings.push(GuideAuditFinding {
                    severity: GuideAuditSeverity::Error,
                    code: "incomplete_crossing".into(),
                    subject: journal.transition_id.to_string(),
                    message: "prepared crossing requires recovery before it is canonical".into(),
                });
            }
            GuideTransitionJournalStatus::Confirmed => report.confirmed_count += 1,
            GuideTransitionJournalStatus::Rejected => report.rejected_count += 1,
        }
        if let Err(error) = journal.validate() {
            report.findings.push(GuideAuditFinding {
                severity: GuideAuditSeverity::Error,
                code: "constitutional_violation".into(),
                subject,
                message: error.to_string(),
            });
        }
    }
    report
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GuideStateMachineError {
    InvalidPresence(GuidePresenceError),
    InvalidTransition(GuideTransitionError),
    WrongStableLayer {
        expected: WorldLayer,
        actual: WorldLayer,
    },
    WrongTransitionDirection {
        expected: GuideTransitionDirection,
        actual: GuideTransitionDirection,
    },
    InvalidJunctionAdvance {
        expected: JunctionTransitionState,
        actual: JunctionTransitionState,
        requested: JunctionTransitionState,
    },
    UnfinishedJunctionSession,
    UnsupportedEventSchema(String),
    UnsupportedJournalSchema(String),
    EventSessionMismatch,
    EventLayerMismatch {
        kind: GuideTransitionEventKind,
        layer: WorldLayer,
    },
    JunctionOriginMismatch,
    JournalTransitionMismatch,
    JournalEventMismatch,
    JournalEventSequence,
    InvalidPreparedJournal,
    MissingConfirmedResult,
    ConfirmedResultMismatch,
    MissingRejectedResult,
    RejectedResultMismatch,
    IdempotencyConflict(StableTransitionId),
    JournalNotFound(StableTransitionId),
    IncompletePreparedJournal,
    Serialization(String),
    Io(String),
}

impl From<GuidePresenceError> for GuideStateMachineError {
    fn from(error: GuidePresenceError) -> Self {
        Self::InvalidPresence(error)
    }
}

impl From<GuideTransitionError> for GuideStateMachineError {
    fn from(error: GuideTransitionError) -> Self {
        Self::InvalidTransition(error)
    }
}

impl fmt::Display for GuideStateMachineError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Guide state-machine operation failed: {self:?}")
    }
}

impl std::error::Error for GuideStateMachineError {}
