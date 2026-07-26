use crate::constitutional::{RegionalBeingId, RuleSetId};
use crate::world::session::WorldSession;

use super::{
    BeingContinuityId, BeingContinuityRecord, GameRevision, GameplayArchiveError, GameplayCommand,
    GameplayEventEnvelope, GameplayEventMetadata, GameplayRuntimeError, HollowGroveGameRuntime,
    decode_gameplay_archive, encode_gameplay_archive,
};

/// Presentation-neutral owner of the authoritative gameplay runtime.
///
/// Clients receive immutable records and have no mutable access to the child
/// constitutional, regional, or institutional runtimes.
#[derive(Debug)]
pub struct GameApplicationService {
    runtime: HollowGroveGameRuntime,
}

impl GameApplicationService {
    #[must_use]
    pub fn new(rule_set: RuleSetId) -> Self {
        Self {
            runtime: HollowGroveGameRuntime::new(rule_set),
        }
    }

    #[must_use]
    pub fn with_world_session(rule_set: RuleSetId, world: WorldSession) -> Self {
        Self {
            runtime: HollowGroveGameRuntime::with_world_session(rule_set, world),
        }
    }

    pub fn execute(
        &mut self,
        metadata: GameplayEventMetadata,
        command: GameplayCommand,
    ) -> Result<&GameplayEventEnvelope, GameplayRuntimeError> {
        self.runtime.execute(metadata, command)
    }

    pub fn replay(
        rule_set: RuleSetId,
        events: impl IntoIterator<Item = GameplayEventEnvelope>,
    ) -> Result<Self, GameplayRuntimeError> {
        Ok(Self {
            runtime: HollowGroveGameRuntime::replay(rule_set, events)?,
        })
    }

    pub fn replay_with_world_session(
        rule_set: RuleSetId,
        events: impl IntoIterator<Item = GameplayEventEnvelope>,
        world: WorldSession,
    ) -> Result<Self, GameplayRuntimeError> {
        Ok(Self {
            runtime: HollowGroveGameRuntime::replay_with_world_session(rule_set, events, world)?,
        })
    }

    pub fn encode_archive(&self) -> Result<String, GameplayArchiveError> {
        encode_gameplay_archive(
            self.runtime.rule_set(),
            self.runtime.events(),
            self.runtime.world_session(),
        )
    }

    pub fn from_archive(encoded: &str) -> Result<Self, GameApplicationArchiveError> {
        let (rule_set, events, persisted_world) = decode_gameplay_archive(encoded)?;
        let world = persisted_world
            .map(|contents| WorldSession::from_persisted_output(&contents))
            .transpose()
            .map_err(|error| GameApplicationArchiveError::InstitutionalState(error.to_string()))?
            .unwrap_or_else(WorldSession::canonical);
        Self::replay_with_world_session(rule_set, events, world)
            .map_err(GameApplicationArchiveError::Runtime)
    }

    pub fn from_archive_with_world_session(
        encoded: &str,
        world: WorldSession,
    ) -> Result<Self, GameApplicationArchiveError> {
        let (rule_set, events, persisted_world) = decode_gameplay_archive(encoded)?;
        let replay_world = persisted_world
            .map(|contents| WorldSession::from_persisted_output(&contents))
            .transpose()
            .map_err(|error| GameApplicationArchiveError::InstitutionalState(error.to_string()))?
            .unwrap_or(world);
        Self::replay_with_world_session(rule_set, events, replay_world)
            .map_err(GameApplicationArchiveError::Runtime)
    }

    #[must_use]
    pub const fn revision(&self) -> GameRevision {
        self.runtime.revision()
    }

    #[must_use]
    pub fn events(&self) -> &[GameplayEventEnvelope] {
        self.runtime.events()
    }

    #[must_use]
    pub fn identity(&self, id: &BeingContinuityId) -> Option<&BeingContinuityRecord> {
        self.runtime.identity(id)
    }

    #[must_use]
    pub fn hueman(&self) -> Option<&BeingContinuityRecord> {
        self.runtime.hueman()
    }

    #[must_use]
    pub fn identity_for_regional(&self, id: &RegionalBeingId) -> Option<&BeingContinuityRecord> {
        self.runtime.identity_for_regional(id)
    }

    #[must_use]
    pub const fn runtime(&self) -> &HollowGroveGameRuntime {
        &self.runtime
    }
}

#[derive(Debug)]
pub enum GameApplicationArchiveError {
    Archive(GameplayArchiveError),
    Runtime(GameplayRuntimeError),
    InstitutionalState(String),
}

impl From<GameplayArchiveError> for GameApplicationArchiveError {
    fn from(value: GameplayArchiveError) -> Self {
        Self::Archive(value)
    }
}

impl std::fmt::Display for GameApplicationArchiveError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "gameplay application archive rejected: {self:?}")
    }
}

impl std::error::Error for GameApplicationArchiveError {}
