//! Bounded, replayable Tross succession process.
//!
//! This records candidacy and gate completion without changing the active
//! Flynt office. A successful case still requires an external Flynt/Stonebend
//! office reducer to install the sealed successor.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TrossSuccessionTrigger {
    Death,
    Abdication,
    LawfulRemoval,
    IrreversibleIncapacity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TrossSuccessionStage {
    Opened,
    StonebendNamed,
    SandmanorProven,
    GlaushouseCleared,
    FlyntRecognized,
    CourtReviewed,
    StonebendSealed,
    Rejected,
}

impl TrossSuccessionStage {
    const ORDER: [Self; 7] = [
        Self::Opened,
        Self::StonebendNamed,
        Self::SandmanorProven,
        Self::GlaushouseCleared,
        Self::FlyntRecognized,
        Self::CourtReviewed,
        Self::StonebendSealed,
    ];
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TrossSuccessionRecord {
    pub case_id: String,
    pub incumbent_id: String,
    pub candidate_id: String,
    pub trigger: TrossSuccessionTrigger,
    pub stage: TrossSuccessionStage,
    pub house_records: BTreeSet<String>,
    pub court_review_id: Option<String>,
    pub active_tross_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrossSuccessionError {
    InvalidIdentity,
    InvalidTransition,
    MissingHouseRecord(&'static str),
    MissingCourtReview,
}

impl TrossSuccessionRecord {
    pub fn open(
        case_id: impl Into<String>,
        incumbent_id: impl Into<String>,
        candidate_id: impl Into<String>,
        trigger: TrossSuccessionTrigger,
    ) -> Result<Self, TrossSuccessionError> {
        let record = Self {
            case_id: case_id.into(),
            incumbent_id: incumbent_id.into(),
            candidate_id: candidate_id.into(),
            trigger,
            stage: TrossSuccessionStage::Opened,
            house_records: BTreeSet::new(),
            court_review_id: None,
            active_tross_id: "being.flynt.tross".into(),
        };
        record.validate_identity()?;
        Ok(record)
    }

    pub fn advance(
        &mut self,
        next: TrossSuccessionStage,
        record_id: Option<String>,
    ) -> Result<(), TrossSuccessionError> {
        if next == TrossSuccessionStage::Rejected {
            self.stage = next;
            return Ok(());
        }
        let current = TrossSuccessionStage::ORDER
            .iter()
            .position(|stage| *stage == self.stage)
            .ok_or(TrossSuccessionError::InvalidTransition)?;
        let expected = TrossSuccessionStage::ORDER
            .get(current + 1)
            .copied()
            .ok_or(TrossSuccessionError::InvalidTransition)?;
        if next != expected {
            return Err(TrossSuccessionError::InvalidTransition);
        }
        match next {
            TrossSuccessionStage::StonebendNamed => {
                self.require_record(record_id, "stonebend")?;
            }
            TrossSuccessionStage::SandmanorProven => {
                self.require_record(record_id, "sandmanor")?;
            }
            TrossSuccessionStage::GlaushouseCleared => {
                self.require_record(record_id, "glaüshouse")?;
            }
            TrossSuccessionStage::FlyntRecognized => {
                self.require_record(record_id, "flynt")?;
            }
            TrossSuccessionStage::CourtReviewed => {
                let id = record_id.ok_or(TrossSuccessionError::MissingCourtReview)?;
                self.court_review_id = Some(id);
            }
            TrossSuccessionStage::StonebendSealed => {
                for required in ["stonebend", "sandmanor", "glaüshouse", "flynt"] {
                    if !self.house_records.iter().any(|id| id.starts_with(required)) {
                        return Err(TrossSuccessionError::MissingHouseRecord(required));
                    }
                }
                if self.court_review_id.is_none() {
                    return Err(TrossSuccessionError::MissingCourtReview);
                }
            }
            TrossSuccessionStage::Opened | TrossSuccessionStage::Rejected => {}
        }
        self.stage = next;
        Ok(())
    }

    fn require_record(
        &mut self,
        record_id: Option<String>,
        prefix: &'static str,
    ) -> Result<(), TrossSuccessionError> {
        let id = record_id.ok_or(TrossSuccessionError::MissingHouseRecord(prefix))?;
        if !id.starts_with(prefix) {
            return Err(TrossSuccessionError::MissingHouseRecord(prefix));
        }
        self.house_records.insert(id);
        Ok(())
    }

    fn validate_identity(&self) -> Result<(), TrossSuccessionError> {
        if self.case_id.is_empty()
            || self.incumbent_id.is_empty()
            || self.candidate_id.is_empty()
            || self.active_tross_id != "being.flynt.tross"
            || self.incumbent_id == self.candidate_id
        {
            return Err(TrossSuccessionError::InvalidIdentity);
        }
        Ok(())
    }
}
