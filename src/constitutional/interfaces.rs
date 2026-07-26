//! Shared constitutional-interface events.
//!
//! This module adapts cross-domain records to the existing Constitutional
//! Runtime's deterministic identity, evidence, replay, migration, and
//! historical-preservation grammar. It is not a second runtime or Bond
//! reducer. Domain aggregates retain their own IDs and semantics; explicit
//! identity references preserve provenance at the interface.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::hollow_grove_contract::House;

const INTERFACE_ARCHIVE_FORMAT: &str = "hollow-grove-shared-interface";
pub const SHARED_INTERFACE_ARCHIVE_VERSION: u16 = 1;

fn valid_stable_id(value: &str) -> bool {
    !value.is_empty()
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'.' | b'-')
        })
}

macro_rules! interface_id {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, SharedInterfaceError> {
                let value = value.into();
                if valid_stable_id(&value) {
                    Ok(Self(value))
                } else {
                    Err(SharedInterfaceError::InvalidIdentity(value))
                }
            }

            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(formatter)
            }
        }
    };
}

interface_id!(InterfaceEventId);
interface_id!(InterfaceAggregateId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum InterfaceIdentityKind {
    Person,
    Participant,
    CaseParty,
    Institution,
    House,
    RegionalBeing,
    Title,
    Gate,
    CourtCase,
    Amendment,
    Remedy,
    CentralJunctionRecord,
    Evidence,
    Authority,
    Seal,
    Tombstone,
    Other,
}

impl InterfaceIdentityKind {
    const fn wire_name(self) -> &'static str {
        match self {
            Self::Person => "person",
            Self::Participant => "participant",
            Self::CaseParty => "case-party",
            Self::Institution => "institution",
            Self::House => "house",
            Self::RegionalBeing => "regional-being",
            Self::Title => "title",
            Self::Gate => "gate",
            Self::CourtCase => "court-case",
            Self::Amendment => "amendment",
            Self::Remedy => "remedy",
            Self::CentralJunctionRecord => "central-junction-record",
            Self::Evidence => "evidence",
            Self::Authority => "authority",
            Self::Seal => "seal",
            Self::Tombstone => "tombstone",
            Self::Other => "other",
        }
    }

    fn parse(value: &str) -> Option<Self> {
        Some(match value {
            "person" => Self::Person,
            "participant" => Self::Participant,
            "case-party" => Self::CaseParty,
            "institution" => Self::Institution,
            "house" => Self::House,
            "regional-being" => Self::RegionalBeing,
            "title" => Self::Title,
            "gate" => Self::Gate,
            "court-case" => Self::CourtCase,
            "amendment" => Self::Amendment,
            "remedy" => Self::Remedy,
            "central-junction-record" => Self::CentralJunctionRecord,
            "evidence" => Self::Evidence,
            "authority" => Self::Authority,
            "seal" => Self::Seal,
            "tombstone" => Self::Tombstone,
            "other" => Self::Other,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct InterfaceIdentityRef {
    pub kind: InterfaceIdentityKind,
    stable_id: String,
    authority_source: String,
}

impl InterfaceIdentityRef {
    pub fn new(
        kind: InterfaceIdentityKind,
        stable_id: impl Into<String>,
        authority_source: impl Into<String>,
    ) -> Result<Self, SharedInterfaceError> {
        let stable_id = stable_id.into();
        let authority_source = authority_source.into();
        if !valid_stable_id(&stable_id) || authority_source.trim().is_empty() {
            return Err(SharedInterfaceError::InvalidIdentity(stable_id));
        }
        Ok(Self {
            kind,
            stable_id,
            authority_source,
        })
    }

    #[must_use]
    pub fn stable_id(&self) -> &str {
        &self.stable_id
    }

    #[must_use]
    pub fn authority_source(&self) -> &str {
        &self.authority_source
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum InterfaceDomain {
    Universal,
    Bond,
    House(House),
    MinoanCourt,
    CentralJunction,
    StonebendGate,
    Amendment,
    Restitution,
    EmergencyTransfer,
    Hueman,
}

impl InterfaceDomain {
    fn wire_name(self) -> String {
        match self {
            Self::Universal => "universal".into(),
            Self::Bond => "bond".into(),
            Self::House(house) => format!("house.{}", house.as_str().to_ascii_lowercase()),
            Self::MinoanCourt => "minoan-court".into(),
            Self::CentralJunction => "central-junction".into(),
            Self::StonebendGate => "stonebend-gate".into(),
            Self::Amendment => "amendment".into(),
            Self::Restitution => "restitution".into(),
            Self::EmergencyTransfer => "emergency-transfer".into(),
            Self::Hueman => "hueman".into(),
        }
    }

    fn parse(value: &str) -> Option<Self> {
        Some(match value {
            "universal" => Self::Universal,
            "bond" => Self::Bond,
            "house.stonebend" => Self::House(House::Stonebend),
            "house.sandmanor" => Self::House(House::Sandmanor),
            "house.glaushouse" => Self::House(House::Glaushouse),
            "house.flynt" => Self::House(House::Flynt),
            "minoan-court" => Self::MinoanCourt,
            "central-junction" => Self::CentralJunction,
            "stonebend-gate" => Self::StonebendGate,
            "amendment" => Self::Amendment,
            "restitution" => Self::Restitution,
            "emergency-transfer" => Self::EmergencyTransfer,
            "hueman" => Self::Hueman,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum InterfaceEventKind {
    CourtCaseOpened,
    CourtEvidenceSubmitted,
    CourtStageTransition,
    CourtProtectiveOrder,
    CourtJudgment,
    CourtAppealDisposition,
    CourtConstitutionalDisposition,
    RestitutionOrdered,
    RestitutionDelivered,
    RestitutionFailed,
    RestitutionRecurrence,
    CourtEquilibriumConfirmed,
    CourtCaseClosed,
    AmendmentProposed,
    AmendmentPublicNotice,
    AmendmentTestimony,
    AmendmentProcessCertified,
    AmendmentHouseRatified,
    AmendmentStonebendSealed,
    AmendmentImplemented,
    AmendmentRestitutionReviewed,
    CentralJunctionRecord,
    GateDecision,
    EmergencyTransferOpened,
    EmergencyTransferAcknowledged,
    EmergencyTransferCustodyChanged,
    EmergencyTransferDischarged,
    HouseRemedyExecuted,
    HouseContinuityEvent,
    TombstoneLinked,
}

impl InterfaceEventKind {
    #[must_use]
    pub const fn wire_name(self) -> &'static str {
        match self {
            Self::CourtCaseOpened => "court-case-opened",
            Self::CourtEvidenceSubmitted => "court-evidence-submitted",
            Self::CourtStageTransition => "court-stage-transition",
            Self::CourtProtectiveOrder => "court-protective-order",
            Self::CourtJudgment => "court-judgment",
            Self::CourtAppealDisposition => "court-appeal-disposition",
            Self::CourtConstitutionalDisposition => "court-constitutional-disposition",
            Self::RestitutionOrdered => "restitution-ordered",
            Self::RestitutionDelivered => "restitution-delivered",
            Self::RestitutionFailed => "restitution-failed",
            Self::RestitutionRecurrence => "restitution-recurrence",
            Self::CourtEquilibriumConfirmed => "court-equilibrium-confirmed",
            Self::CourtCaseClosed => "court-case-closed",
            Self::AmendmentProposed => "amendment-proposed",
            Self::AmendmentPublicNotice => "amendment-public-notice",
            Self::AmendmentTestimony => "amendment-testimony",
            Self::AmendmentProcessCertified => "amendment-process-certified",
            Self::AmendmentHouseRatified => "amendment-house-ratified",
            Self::AmendmentStonebendSealed => "amendment-stonebend-sealed",
            Self::AmendmentImplemented => "amendment-implemented",
            Self::AmendmentRestitutionReviewed => "amendment-restitution-reviewed",
            Self::CentralJunctionRecord => "central-junction-record",
            Self::GateDecision => "gate-decision",
            Self::EmergencyTransferOpened => "emergency-transfer-opened",
            Self::EmergencyTransferAcknowledged => "emergency-transfer-acknowledged",
            Self::EmergencyTransferCustodyChanged => "emergency-transfer-custody-changed",
            Self::EmergencyTransferDischarged => "emergency-transfer-discharged",
            Self::HouseRemedyExecuted => "house-remedy-executed",
            Self::HouseContinuityEvent => "house-continuity-event",
            Self::TombstoneLinked => "tombstone-linked",
        }
    }

    fn parse(value: &str) -> Option<Self> {
        Some(match value {
            "court-case-opened" => Self::CourtCaseOpened,
            "court-evidence-submitted" => Self::CourtEvidenceSubmitted,
            "court-stage-transition" => Self::CourtStageTransition,
            "court-protective-order" => Self::CourtProtectiveOrder,
            "court-judgment" => Self::CourtJudgment,
            "court-appeal-disposition" => Self::CourtAppealDisposition,
            "court-constitutional-disposition" => Self::CourtConstitutionalDisposition,
            "restitution-ordered" => Self::RestitutionOrdered,
            "restitution-delivered" => Self::RestitutionDelivered,
            "restitution-failed" => Self::RestitutionFailed,
            "restitution-recurrence" => Self::RestitutionRecurrence,
            "court-equilibrium-confirmed" => Self::CourtEquilibriumConfirmed,
            "court-case-closed" => Self::CourtCaseClosed,
            "amendment-proposed" => Self::AmendmentProposed,
            "amendment-public-notice" => Self::AmendmentPublicNotice,
            "amendment-testimony" => Self::AmendmentTestimony,
            "amendment-process-certified" => Self::AmendmentProcessCertified,
            "amendment-house-ratified" => Self::AmendmentHouseRatified,
            "amendment-stonebend-sealed" => Self::AmendmentStonebendSealed,
            "amendment-implemented" => Self::AmendmentImplemented,
            "amendment-restitution-reviewed" => Self::AmendmentRestitutionReviewed,
            "central-junction-record" => Self::CentralJunctionRecord,
            "gate-decision" => Self::GateDecision,
            "emergency-transfer-opened" => Self::EmergencyTransferOpened,
            "emergency-transfer-acknowledged" => Self::EmergencyTransferAcknowledged,
            "emergency-transfer-custody-changed" => Self::EmergencyTransferCustodyChanged,
            "emergency-transfer-discharged" => Self::EmergencyTransferDischarged,
            "house-remedy-executed" => Self::HouseRemedyExecuted,
            "house-continuity-event" => Self::HouseContinuityEvent,
            "tombstone-linked" => Self::TombstoneLinked,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterfaceFailureState {
    None,
    Rejected { code: String },
    RemediationRequired { code: String },
    Recurrence { code: String },
}

impl InterfaceFailureState {
    fn wire_parts(&self) -> (&'static str, &str) {
        match self {
            Self::None => ("none", ""),
            Self::Rejected { code } => ("rejected", code),
            Self::RemediationRequired { code } => ("remediation-required", code),
            Self::Recurrence { code } => ("recurrence", code),
        }
    }

    fn from_wire(kind: &str, code: String) -> Option<Self> {
        Some(match kind {
            "none" if code.is_empty() => Self::None,
            "rejected" if !code.is_empty() => Self::Rejected { code },
            "remediation-required" if !code.is_empty() => Self::RemediationRequired { code },
            "recurrence" if !code.is_empty() => Self::Recurrence { code },
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceEventEnvelope {
    pub event_id: InterfaceEventId,
    pub source_aggregate: InterfaceAggregateId,
    pub source_domain: InterfaceDomain,
    pub receiving_aggregate: InterfaceAggregateId,
    pub receiving_domain: InterfaceDomain,
    pub authority: InterfaceIdentityRef,
    pub subjects: Vec<InterfaceIdentityRef>,
    pub evidence: Vec<InterfaceIdentityRef>,
    pub kind: InterfaceEventKind,
    pub semantic_sequence: u64,
    pub causal_parent: Option<InterfaceEventId>,
    pub related_case: Option<InterfaceAggregateId>,
    pub related_amendment: Option<InterfaceAggregateId>,
    pub responsible_remedy_institution: Option<InterfaceIdentityRef>,
    pub failure_state: InterfaceFailureState,
    pub migration_version: u16,
    pub historical_links: Vec<InterfaceEventId>,
    pub payload: BTreeMap<String, String>,
}

impl InterfaceEventEnvelope {
    pub fn validate(&self) -> Result<(), SharedInterfaceError> {
        if self.migration_version != SHARED_INTERFACE_ARCHIVE_VERSION {
            return Err(SharedInterfaceError::UnsupportedVersion(
                self.migration_version,
            ));
        }
        if self.payload.keys().any(|key| !valid_stable_id(key))
            || self.payload.values().any(|value| value.trim().is_empty())
        {
            return Err(SharedInterfaceError::InvalidPayload(self.event_id.clone()));
        }
        let mut identities = BTreeSet::new();
        for identity in self.subjects.iter().chain(&self.evidence) {
            let key = (identity.kind, identity.stable_id());
            if !identities.insert(key) {
                return Err(SharedInterfaceError::DuplicateIdentityReference(
                    identity.stable_id().into(),
                ));
            }
        }
        if self
            .historical_links
            .iter()
            .any(|link| link == &self.event_id)
        {
            return Err(SharedInterfaceError::CircularHistory(self.event_id.clone()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterfaceApplyOutcome {
    Applied,
    Idempotent,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SharedInterfaceRuntime {
    events: BTreeMap<InterfaceEventId, InterfaceEventEnvelope>,
    aggregate_sequences: BTreeMap<InterfaceAggregateId, BTreeMap<u64, InterfaceEventId>>,
}

impl SharedInterfaceRuntime {
    #[must_use]
    pub fn event(&self, event_id: &InterfaceEventId) -> Option<&InterfaceEventEnvelope> {
        self.events.get(event_id)
    }

    #[must_use]
    pub fn events(&self) -> Vec<&InterfaceEventEnvelope> {
        let mut events = self.events.values().collect::<Vec<_>>();
        events.sort_by_key(|event| (event.semantic_sequence, event.event_id.as_str()));
        events
    }

    #[must_use]
    pub fn aggregate_history(
        &self,
        aggregate: &InterfaceAggregateId,
    ) -> Vec<&InterfaceEventEnvelope> {
        self.aggregate_sequences
            .get(aggregate)
            .into_iter()
            .flat_map(|records| records.values())
            .filter_map(|event_id| self.events.get(event_id))
            .collect()
    }

    pub fn apply(
        &mut self,
        event: InterfaceEventEnvelope,
    ) -> Result<InterfaceApplyOutcome, SharedInterfaceError> {
        event.validate()?;
        if let Some(existing) = self.events.get(&event.event_id) {
            return if existing == &event {
                Ok(InterfaceApplyOutcome::Idempotent)
            } else {
                Err(SharedInterfaceError::ConflictingEventIdentity(
                    event.event_id,
                ))
            };
        }
        if let Some(parent) = &event.causal_parent {
            let parent_event = self
                .events
                .get(parent)
                .ok_or_else(|| SharedInterfaceError::MissingCausalParent(parent.clone()))?;
            if parent_event.semantic_sequence >= event.semantic_sequence {
                return Err(SharedInterfaceError::InvalidCausalOrder(event.event_id));
            }
        }
        let sequence = self
            .aggregate_sequences
            .entry(event.source_aggregate.clone())
            .or_default();
        if let Some(existing) = sequence.get(&event.semantic_sequence) {
            return Err(SharedInterfaceError::DuplicateSemanticSequence {
                aggregate: event.source_aggregate,
                sequence: event.semantic_sequence,
                existing: existing.clone(),
            });
        }
        sequence.insert(event.semantic_sequence, event.event_id.clone());
        self.events.insert(event.event_id.clone(), event);
        Ok(InterfaceApplyOutcome::Applied)
    }

    pub fn replay(
        events: impl IntoIterator<Item = InterfaceEventEnvelope>,
    ) -> Result<Self, SharedInterfaceError> {
        let mut ordered = events.into_iter().collect::<Vec<_>>();
        ordered.sort_by_key(|event| (event.semantic_sequence, event.event_id.as_str().to_owned()));
        let mut runtime = Self::default();
        for event in ordered {
            runtime.apply(event)?;
        }
        Ok(runtime)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SharedInterfaceError {
    InvalidIdentity(String),
    InvalidPayload(InterfaceEventId),
    DuplicateIdentityReference(String),
    CircularHistory(InterfaceEventId),
    UnsupportedVersion(u16),
    ConflictingEventIdentity(InterfaceEventId),
    MissingCausalParent(InterfaceEventId),
    InvalidCausalOrder(InterfaceEventId),
    DuplicateSemanticSequence {
        aggregate: InterfaceAggregateId,
        sequence: u64,
        existing: InterfaceEventId,
    },
    InvalidArchive(String),
}

impl fmt::Display for SharedInterfaceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "shared constitutional interface error: {self:?}")
    }
}

impl std::error::Error for SharedInterfaceError {}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct WireIdentity {
    kind: String,
    stable_id: String,
    authority_source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct WireEvent {
    event_id: String,
    source_aggregate: String,
    source_domain: String,
    receiving_aggregate: String,
    receiving_domain: String,
    authority: WireIdentity,
    subjects: Vec<WireIdentity>,
    evidence: Vec<WireIdentity>,
    kind: String,
    semantic_sequence: u64,
    causal_parent: Option<String>,
    related_case: Option<String>,
    related_amendment: Option<String>,
    responsible_remedy_institution: Option<WireIdentity>,
    failure_kind: String,
    failure_code: String,
    migration_version: u16,
    historical_links: Vec<String>,
    payload: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct WireArchive {
    format: String,
    schema_version: u16,
    events: Vec<WireEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LegacyWireEventV0 {
    event_id: String,
    source_aggregate: String,
    source_domain: String,
    receiving_aggregate: String,
    authority: WireIdentity,
    subjects: Vec<WireIdentity>,
    evidence: Vec<WireIdentity>,
    kind: String,
    semantic_sequence: u64,
    causal_parent: Option<String>,
    related_case: Option<String>,
    related_amendment: Option<String>,
    responsible_remedy_institution: Option<WireIdentity>,
    failure_kind: String,
    failure_code: String,
    payload: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LegacyWireArchiveV0 {
    format: String,
    schema_version: u16,
    events: Vec<LegacyWireEventV0>,
}

fn identity_to_wire(identity: &InterfaceIdentityRef) -> WireIdentity {
    WireIdentity {
        kind: identity.kind.wire_name().into(),
        stable_id: identity.stable_id.clone(),
        authority_source: identity.authority_source.clone(),
    }
}

fn identity_from_wire(
    identity: WireIdentity,
) -> Result<InterfaceIdentityRef, SharedInterfaceError> {
    let kind = InterfaceIdentityKind::parse(&identity.kind)
        .ok_or_else(|| SharedInterfaceError::InvalidArchive("identity kind".into()))?;
    InterfaceIdentityRef::new(kind, identity.stable_id, identity.authority_source)
}

fn event_to_wire(event: &InterfaceEventEnvelope) -> WireEvent {
    let (failure_kind, failure_code) = event.failure_state.wire_parts();
    WireEvent {
        event_id: event.event_id.as_str().into(),
        source_aggregate: event.source_aggregate.as_str().into(),
        source_domain: event.source_domain.wire_name(),
        receiving_aggregate: event.receiving_aggregate.as_str().into(),
        receiving_domain: event.receiving_domain.wire_name(),
        authority: identity_to_wire(&event.authority),
        subjects: event.subjects.iter().map(identity_to_wire).collect(),
        evidence: event.evidence.iter().map(identity_to_wire).collect(),
        kind: event.kind.wire_name().into(),
        semantic_sequence: event.semantic_sequence,
        causal_parent: event
            .causal_parent
            .as_ref()
            .map(|value| value.as_str().into()),
        related_case: event
            .related_case
            .as_ref()
            .map(|value| value.as_str().into()),
        related_amendment: event
            .related_amendment
            .as_ref()
            .map(|value| value.as_str().into()),
        responsible_remedy_institution: event
            .responsible_remedy_institution
            .as_ref()
            .map(identity_to_wire),
        failure_kind: failure_kind.into(),
        failure_code: failure_code.into(),
        migration_version: event.migration_version,
        historical_links: event
            .historical_links
            .iter()
            .map(|value| value.as_str().into())
            .collect(),
        payload: event.payload.clone(),
    }
}

fn event_from_wire(event: WireEvent) -> Result<InterfaceEventEnvelope, SharedInterfaceError> {
    let result = InterfaceEventEnvelope {
        event_id: InterfaceEventId::new(event.event_id)?,
        source_aggregate: InterfaceAggregateId::new(event.source_aggregate)?,
        source_domain: InterfaceDomain::parse(&event.source_domain)
            .ok_or_else(|| SharedInterfaceError::InvalidArchive("source domain".into()))?,
        receiving_aggregate: InterfaceAggregateId::new(event.receiving_aggregate)?,
        receiving_domain: InterfaceDomain::parse(&event.receiving_domain)
            .ok_or_else(|| SharedInterfaceError::InvalidArchive("receiving domain".into()))?,
        authority: identity_from_wire(event.authority)?,
        subjects: event
            .subjects
            .into_iter()
            .map(identity_from_wire)
            .collect::<Result<_, _>>()?,
        evidence: event
            .evidence
            .into_iter()
            .map(identity_from_wire)
            .collect::<Result<_, _>>()?,
        kind: InterfaceEventKind::parse(&event.kind)
            .ok_or_else(|| SharedInterfaceError::InvalidArchive("event kind".into()))?,
        semantic_sequence: event.semantic_sequence,
        causal_parent: event.causal_parent.map(InterfaceEventId::new).transpose()?,
        related_case: event
            .related_case
            .map(InterfaceAggregateId::new)
            .transpose()?,
        related_amendment: event
            .related_amendment
            .map(InterfaceAggregateId::new)
            .transpose()?,
        responsible_remedy_institution: event
            .responsible_remedy_institution
            .map(identity_from_wire)
            .transpose()?,
        failure_state: InterfaceFailureState::from_wire(&event.failure_kind, event.failure_code)
            .ok_or_else(|| SharedInterfaceError::InvalidArchive("failure state".into()))?,
        migration_version: event.migration_version,
        historical_links: event
            .historical_links
            .into_iter()
            .map(InterfaceEventId::new)
            .collect::<Result<_, _>>()?,
        payload: event.payload,
    };
    result.validate()?;
    Ok(result)
}

pub fn encode_shared_interface_archive(
    runtime: &SharedInterfaceRuntime,
) -> Result<Vec<u8>, SharedInterfaceError> {
    let archive = WireArchive {
        format: INTERFACE_ARCHIVE_FORMAT.into(),
        schema_version: SHARED_INTERFACE_ARCHIVE_VERSION,
        events: runtime.events().into_iter().map(event_to_wire).collect(),
    };
    serde_json::to_vec(&archive)
        .map_err(|error| SharedInterfaceError::InvalidArchive(error.to_string()))
}

pub fn decode_shared_interface_archive(
    bytes: &[u8],
) -> Result<SharedInterfaceRuntime, SharedInterfaceError> {
    let archive: WireArchive = serde_json::from_slice(bytes)
        .map_err(|error| SharedInterfaceError::InvalidArchive(error.to_string()))?;
    if archive.format != INTERFACE_ARCHIVE_FORMAT {
        return Err(SharedInterfaceError::InvalidArchive("format".into()));
    }
    if archive.schema_version != SHARED_INTERFACE_ARCHIVE_VERSION {
        return Err(SharedInterfaceError::UnsupportedVersion(
            archive.schema_version,
        ));
    }
    SharedInterfaceRuntime::replay(
        archive
            .events
            .into_iter()
            .map(event_from_wire)
            .collect::<Result<Vec<_>, _>>()?,
    )
}

/// Produces the historical V0 fixture used by migration conformance tests.
pub fn encode_legacy_shared_interface_archive_v0(
    runtime: &SharedInterfaceRuntime,
) -> Result<Vec<u8>, SharedInterfaceError> {
    let events = runtime
        .events()
        .into_iter()
        .map(event_to_wire)
        .map(|event| LegacyWireEventV0 {
            event_id: event.event_id,
            source_aggregate: event.source_aggregate,
            source_domain: event.source_domain,
            receiving_aggregate: event.receiving_aggregate,
            authority: event.authority,
            subjects: event.subjects,
            evidence: event.evidence,
            kind: event.kind,
            semantic_sequence: event.semantic_sequence,
            causal_parent: event.causal_parent,
            related_case: event.related_case,
            related_amendment: event.related_amendment,
            responsible_remedy_institution: event.responsible_remedy_institution,
            failure_kind: event.failure_kind,
            failure_code: event.failure_code,
            payload: event.payload,
        })
        .collect();
    serde_json::to_vec(&LegacyWireArchiveV0 {
        format: INTERFACE_ARCHIVE_FORMAT.into(),
        schema_version: 0,
        events,
    })
    .map_err(|error| SharedInterfaceError::InvalidArchive(error.to_string()))
}

pub fn migrate_shared_interface_archive(bytes: &[u8]) -> Result<Vec<u8>, SharedInterfaceError> {
    let header: serde_json::Value = serde_json::from_slice(bytes)
        .map_err(|error| SharedInterfaceError::InvalidArchive(error.to_string()))?;
    let version = header
        .get("schema_version")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| SharedInterfaceError::InvalidArchive("schema version".into()))?;
    match u16::try_from(version)
        .map_err(|_| SharedInterfaceError::InvalidArchive("schema version".into()))?
    {
        SHARED_INTERFACE_ARCHIVE_VERSION => {
            encode_shared_interface_archive(&decode_shared_interface_archive(bytes)?)
        }
        0 => {
            let legacy: LegacyWireArchiveV0 = serde_json::from_slice(bytes)
                .map_err(|error| SharedInterfaceError::InvalidArchive(error.to_string()))?;
            if legacy.format != INTERFACE_ARCHIVE_FORMAT {
                return Err(SharedInterfaceError::InvalidArchive("format".into()));
            }
            let events = legacy
                .events
                .into_iter()
                .map(|event| {
                    event_from_wire(WireEvent {
                        event_id: event.event_id,
                        source_aggregate: event.source_aggregate,
                        source_domain: event.source_domain.clone(),
                        receiving_aggregate: event.receiving_aggregate,
                        receiving_domain: event.source_domain,
                        authority: event.authority,
                        subjects: event.subjects,
                        evidence: event.evidence,
                        kind: event.kind,
                        semantic_sequence: event.semantic_sequence,
                        causal_parent: event.causal_parent,
                        related_case: event.related_case,
                        related_amendment: event.related_amendment,
                        responsible_remedy_institution: event.responsible_remedy_institution,
                        failure_kind: event.failure_kind,
                        failure_code: event.failure_code,
                        migration_version: SHARED_INTERFACE_ARCHIVE_VERSION,
                        historical_links: Vec::new(),
                        payload: event.payload,
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;
            encode_shared_interface_archive(&SharedInterfaceRuntime::replay(events)?)
        }
        other => Err(SharedInterfaceError::UnsupportedVersion(other)),
    }
}
