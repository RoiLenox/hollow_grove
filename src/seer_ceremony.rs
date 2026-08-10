//! Singular, sequential Seer Ceremony state machine.
//!
//! The operational Guide seam remains paired (Roi/Roselina). This module is
//! the separate constitutional layer for an individual becoming a Seer and
//! choosing Continuance or Founding.

use serde::{Deserialize, Serialize};
use std::fmt;

pub const SEER_CEREMONY_SCHEMA_VERSION: &str = "hollow-grove-seer-ceremony-v1";
pub const SEER_CEREMONY_ARCHIVE_FORMAT: &str = "HGSCP";
pub const SEER_CEREMONY_ARCHIVE_VERSION: u16 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SeerCeremony {
    pub schema_version: String,
    pub ceremony_id: String,
    pub individual_id: String,
    pub source_point_id: String,
    pub movement: CeremonyMovement,
    pub path: Option<SeerPath>,
    pub created_point_id: Option<String>,
    pub status: CeremonyStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CeremonyMovement {
    Gathering,
    Naming,
    Convergence,
    Recognition,
    Squaring,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SeerPath {
    Continuance,
    Founding,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CeremonyStatus {
    Active,
    Suspended,
    Completed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeerCeremonyCoordinator {
    active: Option<SeerCeremony>,
    completed: Vec<SeerCeremony>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SeerCeremonyArchive {
    pub format: String,
    pub version: u16,
    pub active: Option<SeerCeremony>,
    pub completed: Vec<SeerCeremony>,
    pub checksum: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SeerCeremonyOutcome {
    Continued {
        seer_id: String,
        point_id: String,
    },
    Founded {
        seer_id: String,
        source_point_id: String,
        created_point_id: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SeerCeremonyError {
    EmptyIdentifier(&'static str),
    CeremonyAlreadyActive,
    NoActiveCeremony,
    CeremonySuspended,
    CeremonyAlreadyCompleted,
    WrongMovement {
        expected: CeremonyMovement,
        actual: CeremonyMovement,
    },
    PathAlreadyChosen,
    PathRequired,
    FoundingPointCollision(String),
    InvalidArchive(String),
    ArchiveSerialization(String),
}

impl fmt::Display for SeerCeremonyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for SeerCeremonyError {}

impl SeerCeremonyCoordinator {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            active: None,
            completed: Vec::new(),
        }
    }

    #[must_use]
    pub const fn active(&self) -> Option<&SeerCeremony> {
        self.active.as_ref()
    }

    #[must_use]
    pub fn completed(&self) -> &[SeerCeremony] {
        &self.completed
    }

    pub fn encode_archive(&self) -> Result<Vec<u8>, SeerCeremonyError> {
        let mut archive = self.archive_without_checksum();
        archive.checksum = archive_checksum(&archive)?;
        serde_json::to_vec_pretty(&archive)
            .map_err(|error| SeerCeremonyError::ArchiveSerialization(error.to_string()))
    }

    pub fn decode_archive(bytes: &[u8]) -> Result<Self, SeerCeremonyError> {
        let archive: SeerCeremonyArchive = serde_json::from_slice(bytes)
            .map_err(|error| SeerCeremonyError::ArchiveSerialization(error.to_string()))?;
        validate_archive(&archive)?;
        Ok(Self {
            active: archive.active,
            completed: archive.completed,
        })
    }

    #[must_use]
    pub fn archive(&self) -> SeerCeremonyArchive {
        self.archive_without_checksum()
    }

    fn archive_without_checksum(&self) -> SeerCeremonyArchive {
        SeerCeremonyArchive {
            format: SEER_CEREMONY_ARCHIVE_FORMAT.into(),
            version: SEER_CEREMONY_ARCHIVE_VERSION,
            active: self.active.clone(),
            completed: self.completed.clone(),
            checksum: String::new(),
        }
    }

    pub fn begin(
        &mut self,
        ceremony_id: impl Into<String>,
        individual_id: impl Into<String>,
        source_point_id: impl Into<String>,
    ) -> Result<&SeerCeremony, SeerCeremonyError> {
        if self.active.is_some() {
            return Err(SeerCeremonyError::CeremonyAlreadyActive);
        }
        let ceremony_id = ceremony_id.into();
        let individual_id = individual_id.into();
        let source_point_id = source_point_id.into();
        validate_identifier(&ceremony_id, "ceremony_id")?;
        validate_identifier(&individual_id, "individual_id")?;
        validate_identifier(&source_point_id, "source_point_id")?;
        self.active = Some(SeerCeremony {
            schema_version: SEER_CEREMONY_SCHEMA_VERSION.into(),
            ceremony_id,
            individual_id,
            source_point_id,
            movement: CeremonyMovement::Gathering,
            path: None,
            created_point_id: None,
            status: CeremonyStatus::Active,
        });
        Ok(self
            .active
            .as_ref()
            .expect("active ceremony was just stored"))
    }

    pub fn advance(&mut self) -> Result<CeremonyMovement, SeerCeremonyError> {
        let ceremony = self
            .active
            .as_mut()
            .ok_or(SeerCeremonyError::NoActiveCeremony)?;
        if ceremony.status == CeremonyStatus::Suspended {
            return Err(SeerCeremonyError::CeremonySuspended);
        }
        if ceremony.status == CeremonyStatus::Completed {
            return Err(SeerCeremonyError::CeremonyAlreadyCompleted);
        }
        let next = match ceremony.movement {
            CeremonyMovement::Gathering => CeremonyMovement::Naming,
            CeremonyMovement::Naming => CeremonyMovement::Convergence,
            CeremonyMovement::Convergence => CeremonyMovement::Recognition,
            CeremonyMovement::Recognition => CeremonyMovement::Squaring,
            CeremonyMovement::Squaring => {
                return Err(SeerCeremonyError::WrongMovement {
                    expected: CeremonyMovement::Squaring,
                    actual: CeremonyMovement::Squaring,
                });
            }
        };
        ceremony.movement = next;
        Ok(next)
    }

    pub fn choose_path(&mut self, path: SeerPath) -> Result<(), SeerCeremonyError> {
        let ceremony = self
            .active
            .as_mut()
            .ok_or(SeerCeremonyError::NoActiveCeremony)?;
        if ceremony.movement != CeremonyMovement::Squaring {
            return Err(SeerCeremonyError::WrongMovement {
                expected: CeremonyMovement::Squaring,
                actual: ceremony.movement,
            });
        }
        if ceremony.path.replace(path).is_some() {
            return Err(SeerCeremonyError::PathAlreadyChosen);
        }
        Ok(())
    }

    pub fn suspend(&mut self) -> Result<(), SeerCeremonyError> {
        let ceremony = self
            .active
            .as_mut()
            .ok_or(SeerCeremonyError::NoActiveCeremony)?;
        if ceremony.status == CeremonyStatus::Completed {
            return Err(SeerCeremonyError::CeremonyAlreadyCompleted);
        }
        ceremony.status = CeremonyStatus::Suspended;
        Ok(())
    }

    pub fn complete(&mut self) -> Result<SeerCeremonyOutcome, SeerCeremonyError> {
        let mut ceremony = self
            .active
            .take()
            .ok_or(SeerCeremonyError::NoActiveCeremony)?;
        if ceremony.status == CeremonyStatus::Suspended {
            self.active = Some(ceremony);
            return Err(SeerCeremonyError::CeremonySuspended);
        }
        if ceremony.movement != CeremonyMovement::Squaring {
            let actual = ceremony.movement;
            self.active = Some(ceremony);
            return Err(SeerCeremonyError::WrongMovement {
                expected: CeremonyMovement::Squaring,
                actual,
            });
        }
        let path = ceremony.path.ok_or_else(|| {
            self.active = Some(ceremony.clone());
            SeerCeremonyError::PathRequired
        })?;
        let outcome = match path {
            SeerPath::Continuance => SeerCeremonyOutcome::Continued {
                seer_id: ceremony.individual_id.clone(),
                point_id: ceremony.source_point_id.clone(),
            },
            SeerPath::Founding => {
                let created = format!("{}.{}", ceremony.source_point_id, ceremony.ceremony_id);
                if self
                    .completed
                    .iter()
                    .any(|item| item.created_point_id.as_deref() == Some(&created))
                {
                    self.active = Some(ceremony);
                    return Err(SeerCeremonyError::FoundingPointCollision(created));
                }
                ceremony.created_point_id = Some(created.clone());
                SeerCeremonyOutcome::Founded {
                    seer_id: ceremony.individual_id.clone(),
                    source_point_id: ceremony.source_point_id.clone(),
                    created_point_id: created,
                }
            }
        };
        ceremony.status = CeremonyStatus::Completed;
        self.completed.push(ceremony);
        Ok(outcome)
    }
}

impl Default for SeerCeremonyCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

fn validate_identifier(value: &str, field: &'static str) -> Result<(), SeerCeremonyError> {
    if value.trim().is_empty() {
        Err(SeerCeremonyError::EmptyIdentifier(field))
    } else {
        Ok(())
    }
}

fn archive_checksum(archive: &SeerCeremonyArchive) -> Result<String, SeerCeremonyError> {
    let mut unsigned = archive.clone();
    unsigned.checksum.clear();
    let bytes = serde_json::to_vec(&unsigned)
        .map_err(|error| SeerCeremonyError::ArchiveSerialization(error.to_string()))?;
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in bytes {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    Ok(format!("fnv1a64:hgscp-v1:{hash:016x}"))
}

fn validate_archive(archive: &SeerCeremonyArchive) -> Result<(), SeerCeremonyError> {
    if archive.format != SEER_CEREMONY_ARCHIVE_FORMAT {
        return Err(SeerCeremonyError::InvalidArchive(format!(
            "unsupported format {}",
            archive.format
        )));
    }
    if archive.version != SEER_CEREMONY_ARCHIVE_VERSION {
        return Err(SeerCeremonyError::InvalidArchive(format!(
            "unsupported version {}",
            archive.version
        )));
    }
    let expected = archive_checksum(archive)?;
    if archive.checksum != expected {
        return Err(SeerCeremonyError::InvalidArchive(format!(
            "checksum mismatch: expected {}, got {}",
            expected, archive.checksum
        )));
    }
    let mut ids = std::collections::BTreeSet::new();
    for ceremony in archive.completed.iter().chain(archive.active.iter()) {
        if ceremony.schema_version != SEER_CEREMONY_SCHEMA_VERSION {
            return Err(SeerCeremonyError::InvalidArchive(format!(
                "unsupported ceremony schema {}",
                ceremony.schema_version
            )));
        }
        if ceremony.status == CeremonyStatus::Completed && !archive.completed.contains(ceremony) {
            return Err(SeerCeremonyError::InvalidArchive(
                "completed ceremony outside completed archive".into(),
            ));
        }
        if !ids.insert(ceremony.ceremony_id.clone()) {
            return Err(SeerCeremonyError::InvalidArchive(format!(
                "duplicate ceremony {}",
                ceremony.ceremony_id
            )));
        }
    }
    if let Some(active) = &archive.active {
        if active.status == CeremonyStatus::Completed {
            return Err(SeerCeremonyError::InvalidArchive(
                "active ceremony cannot be completed".into(),
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn complete(coordinator: &mut SeerCeremonyCoordinator, path: SeerPath) {
        for _ in 0..4 {
            coordinator.advance().unwrap();
        }
        coordinator.choose_path(path).unwrap();
    }

    #[test]
    fn one_individual_completes_a_continuance_ceremony() {
        let mut coordinator = SeerCeremonyCoordinator::new();
        coordinator
            .begin("ceremony.1", "person.1", "point.1")
            .unwrap();
        complete(&mut coordinator, SeerPath::Continuance);
        assert_eq!(
            coordinator.complete().unwrap(),
            SeerCeremonyOutcome::Continued {
                seer_id: "person.1".into(),
                point_id: "point.1".into()
            }
        );
        assert_eq!(coordinator.completed().len(), 1);
    }

    #[test]
    fn founding_creates_a_successor_point_and_allows_the_next_ceremony() {
        let mut coordinator = SeerCeremonyCoordinator::new();
        coordinator
            .begin("ceremony.1", "person.1", "point.1")
            .unwrap();
        complete(&mut coordinator, SeerPath::Founding);
        assert_eq!(
            coordinator.complete().unwrap(),
            SeerCeremonyOutcome::Founded {
                seer_id: "person.1".into(),
                source_point_id: "point.1".into(),
                created_point_id: "point.1.ceremony.1".into()
            }
        );
        coordinator
            .begin("ceremony.2", "person.2", "point.1.ceremony.1")
            .unwrap();
        assert!(coordinator.active().is_some());
    }

    #[test]
    fn ceremonies_are_individual_and_non_overlapping_but_unlimited() {
        let mut coordinator = SeerCeremonyCoordinator::new();
        coordinator
            .begin("ceremony.1", "person.1", "point.1")
            .unwrap();
        assert_eq!(
            coordinator
                .begin("ceremony.2", "person.2", "point.1")
                .unwrap_err(),
            SeerCeremonyError::CeremonyAlreadyActive
        );
    }

    #[test]
    fn archive_round_trips_and_rejects_tampering() {
        let mut coordinator = SeerCeremonyCoordinator::new();
        coordinator
            .begin("ceremony.1", "person.1", "point.1")
            .unwrap();
        let bytes = coordinator.encode_archive().unwrap();
        let restored = SeerCeremonyCoordinator::decode_archive(&bytes).unwrap();
        assert_eq!(restored, coordinator);

        let mut tampered = bytes;
        let index = tampered
            .iter()
            .position(|byte| *byte == b'1')
            .expect("archive contains an identifier digit");
        tampered[index] = b'9';
        assert!(matches!(
            SeerCeremonyCoordinator::decode_archive(&tampered),
            Err(SeerCeremonyError::InvalidArchive(_))
        ));
    }

    #[test]
    fn completed_archive_can_resume_with_another_individual() {
        let mut coordinator = SeerCeremonyCoordinator::new();
        coordinator
            .begin("ceremony.1", "person.1", "point.1")
            .unwrap();
        complete(&mut coordinator, SeerPath::Continuance);
        coordinator.complete().unwrap();
        let restored =
            SeerCeremonyCoordinator::decode_archive(&coordinator.encode_archive().unwrap())
                .unwrap();
        assert_eq!(restored.completed().len(), 1);
        assert!(restored.active().is_none());
    }
}
