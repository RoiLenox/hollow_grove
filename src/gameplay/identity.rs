use std::collections::BTreeMap;
use std::fmt;

use crate::constitutional::{ParticipantId, RegionalBeingId};
use crate::frame_state::BeingId;
use crate::institution::InstitutionalBeingId;

fn is_stable_gameplay_id(value: &str) -> bool {
    !value.is_empty()
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'.' | b'-')
        })
}

macro_rules! stable_gameplay_id {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, GameplayIdError> {
                let value = value.into();
                if is_stable_gameplay_id(&value) {
                    Ok(Self(value))
                } else {
                    Err(GameplayIdError::Invalid(value))
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameplayIdError {
    Invalid(String),
}

impl fmt::Display for GameplayIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid(value) => write!(formatter, "invalid gameplay identifier: {value}"),
        }
    }
}

impl std::error::Error for GameplayIdError {}

stable_gameplay_id!(BeingContinuityId);
stable_gameplay_id!(GameplayEventId);

/// The currently embodied identity behind one stable gameplay continuity.
///
/// Regional Synthesis may replace `Regional` with a new `RegionalBeingId` in a
/// later event. The continuity ID remains stable while predecessor and result
/// identities remain distinct in the regional runtime.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActiveIncarnationRef {
    Hueman,
    Regional(RegionalBeingId),
}

impl ActiveIncarnationRef {
    #[must_use]
    pub const fn legacy_being(&self) -> Option<BeingId> {
        match self {
            Self::Hueman => Some(BeingId::Hueman),
            Self::Regional(_) => None,
        }
    }

    #[must_use]
    pub const fn regional_being(&self) -> Option<&RegionalBeingId> {
        match self {
            Self::Hueman => None,
            Self::Regional(being) => Some(being),
        }
    }
}

/// Typed cross-domain references for one gameplay Being.
///
/// These references are deliberately not collapsed into one universal ID.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BeingDomainRefs {
    participant: ParticipantId,
    institutional: InstitutionalBeingId,
}

impl BeingDomainRefs {
    #[must_use]
    pub const fn participant(&self) -> &ParticipantId {
        &self.participant
    }

    #[must_use]
    pub const fn institutional(&self) -> &InstitutionalBeingId {
        &self.institutional
    }
}

/// Stable gameplay continuity plus the active domain incarnation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BeingContinuityRecord {
    id: BeingContinuityId,
    incarnation: ActiveIncarnationRef,
    domain_refs: BeingDomainRefs,
}

impl BeingContinuityRecord {
    pub(super) fn hueman(
        id: BeingContinuityId,
        participant: ParticipantId,
        institutional: InstitutionalBeingId,
    ) -> Self {
        Self {
            id,
            incarnation: ActiveIncarnationRef::Hueman,
            domain_refs: BeingDomainRefs {
                participant,
                institutional,
            },
        }
    }

    pub(super) fn regional(
        id: BeingContinuityId,
        regional: RegionalBeingId,
        participant: ParticipantId,
        institutional: InstitutionalBeingId,
    ) -> Self {
        Self {
            id,
            incarnation: ActiveIncarnationRef::Regional(regional),
            domain_refs: BeingDomainRefs {
                participant,
                institutional,
            },
        }
    }

    #[must_use]
    pub const fn id(&self) -> &BeingContinuityId {
        &self.id
    }

    #[must_use]
    pub const fn incarnation(&self) -> &ActiveIncarnationRef {
        &self.incarnation
    }

    #[must_use]
    pub const fn domain_refs(&self) -> &BeingDomainRefs {
        &self.domain_refs
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(super) struct IdentityRegistry {
    records: BTreeMap<BeingContinuityId, BeingContinuityRecord>,
    hueman: Option<BeingContinuityId>,
    regional: BTreeMap<RegionalBeingId, BeingContinuityId>,
    participants: BTreeMap<ParticipantId, BeingContinuityId>,
    institutional: BTreeMap<InstitutionalBeingId, BeingContinuityId>,
}

impl IdentityRegistry {
    pub(super) fn insert(
        &mut self,
        record: BeingContinuityRecord,
    ) -> Result<(), GameplayIdentityError> {
        if self.records.contains_key(record.id()) {
            return Err(GameplayIdentityError::ContinuityIdConflict(
                record.id().clone(),
            ));
        }
        if let Some(existing) = self.participants.get(record.domain_refs().participant()) {
            return Err(GameplayIdentityError::ParticipantAlreadyMapped {
                participant: record.domain_refs().participant().clone(),
                existing: existing.clone(),
            });
        }
        if let Some(existing) = self.institutional.get(record.domain_refs().institutional()) {
            return Err(GameplayIdentityError::InstitutionalBeingAlreadyMapped {
                institutional: record.domain_refs().institutional().clone(),
                existing: existing.clone(),
            });
        }

        match record.incarnation() {
            ActiveIncarnationRef::Hueman => {
                if let Some(existing) = &self.hueman {
                    return Err(GameplayIdentityError::HuemanAlreadyEstablished {
                        existing: existing.clone(),
                        attempted: record.id().clone(),
                    });
                }
            }
            ActiveIncarnationRef::Regional(regional) => {
                if let Some(existing) = self.regional.get(regional) {
                    return Err(GameplayIdentityError::RegionalBeingAlreadyMapped {
                        regional: regional.clone(),
                        existing: existing.clone(),
                    });
                }
            }
        }

        let id = record.id().clone();
        self.participants
            .insert(record.domain_refs().participant().clone(), id.clone());
        self.institutional
            .insert(record.domain_refs().institutional().clone(), id.clone());
        match record.incarnation() {
            ActiveIncarnationRef::Hueman => self.hueman = Some(id.clone()),
            ActiveIncarnationRef::Regional(regional) => {
                self.regional.insert(regional.clone(), id.clone());
            }
        }
        self.records.insert(id, record);
        Ok(())
    }

    pub(super) fn get(&self, id: &BeingContinuityId) -> Option<&BeingContinuityRecord> {
        self.records.get(id)
    }

    pub(super) fn hueman(&self) -> Option<&BeingContinuityRecord> {
        self.records.get(self.hueman.as_ref()?)
    }

    pub(super) fn by_regional(&self, regional: &RegionalBeingId) -> Option<&BeingContinuityRecord> {
        self.records.get(self.regional.get(regional)?)
    }

    pub(super) fn len(&self) -> usize {
        self.records.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameplayIdentityError {
    ContinuityIdConflict(BeingContinuityId),
    HuemanAlreadyEstablished {
        existing: BeingContinuityId,
        attempted: BeingContinuityId,
    },
    RegionalBeingAlreadyMapped {
        regional: RegionalBeingId,
        existing: BeingContinuityId,
    },
    ParticipantAlreadyMapped {
        participant: ParticipantId,
        existing: BeingContinuityId,
    },
    InstitutionalBeingAlreadyMapped {
        institutional: InstitutionalBeingId,
        existing: BeingContinuityId,
    },
}

impl fmt::Display for GameplayIdentityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "gameplay identity rejected: {self:?}")
    }
}

impl std::error::Error for GameplayIdentityError {}
