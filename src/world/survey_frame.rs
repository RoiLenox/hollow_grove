//! A bounded Stonebend object case. Trials, custody and resident decisions
//! remain separate from House authority and from the player's Form.
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::constitutional::{HouseFunction, scenario_house_decision};
use crate::institution::IdentityId;
use crate::world::session::WorldSession;
use crate::world::stonebend::{EvidenceRecordId, NameClass, NameRecord, NameRecordId, NameStatus};

pub const SURVEY_FRAME_SOURCE: &str = "STONEBEND_SURVEY_FRAME_FIRST_CASE_V1.md";
pub const SURVEY_FRAME_CASE_ID: &str = "case.stonebend.survey-frame-doorway.v1";
pub const SURVEY_FRAME_LORE_ID: &str = "lore.stonebend.survey-frame-doorway";
pub const SURVEY_FRAME_OBJECT_ID: &str = "object.stonebend.repair-bench.survey-frame";
pub const SURVEY_FRAME_WORKSHOP: &str = "site.stonebend.aura-way.repair-bench";
pub const SURVEY_FRAME_RESIDENT: &str = "being.current-sea.mercy-deep";
pub const SURVEY_FRAME_KEEPER: &str = "being.stonebend.tess-splice";
pub const SURVEY_FRAME_RECORDER: &str = "being.sandmanor.ivo-mark";
pub const SURVEY_FRAME_SAFE_ROUTE: &str = "access.survey-frame.mercy-deep.alternate";
const DAMAGED_JOINT: &str = "part.survey-frame.retained-damaged-joint";
const REPLACEMENT_JOINT: &str = "part.survey-frame.traceable-replacement-joint";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SurveyFramePhase {
    AwaitingNeed,
    NeedRecorded,
    Recovered,
    MeasuringAvailable,
    RigPrepared,
    RigIsolated,
    Repaired,
    RigProven,
    RecoveryRequired,
    AwaitingProtectedReturn,
    Delivered,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SurveyFrameUse {
    MeasuringOnly,
    TemporaryRig,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SurveyFrameResidentChoice {
    AcceptMeasurements,
    AcceptTemporaryRig,
    WaitSafely,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SurveyFrameDisposition {
    MeasurementsDelivered,
    TemporaryRigDelivered,
    ProtectedDelayDelivered,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum SurveyFrameAction {
    ReceiveNeed,
    RecoverParts,
    AskStonebendToRegister,
    AskReviewerToMeasure,
    PrepareRig,
    OfferUse { use_scope: SurveyFrameUse },
    AskReviewerToTrialSupport,
    SupportResidentChoice { choice: SurveyFrameResidentChoice },
    AskResidentToDecide,
    RepairJoint,
    DeliverDisposition { disposition: SurveyFrameDisposition },
    AskResidentToWithdraw,
    RecoverDeployedRig,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SurveyFrameClaim {
    ArtifactIdentityAndDisclosedCustody,
    MeasurementOnly,
    FailedSupportTrial,
    ScopedTemporarySupport,
}

/// Prove may accept the record of a failed trial. Only ScopedTemporarySupport
/// attests this case's positive support claim, never a universal load rating.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SurveyFrameAuthorityReceipt {
    pub decision_id: String,
    pub function: String,
    pub office_id: String,
    pub authority_actor: String,
    pub causal_position: u64,
    pub object_id: String,
    pub object_revision: u64,
    pub claim: SurveyFrameClaim,
    pub scope: String,
    pub evidence: Vec<String>,
    pub independent_recorder: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SurveyFrameRevision {
    pub number: u64,
    pub change: String,
    pub source_parts: Vec<String>,
    pub unresolved_provenance: Vec<String>,
    pub retained_parts: Vec<String>,
    pub measurement_receipts: Vec<SurveyFrameAuthorityReceipt>,
    pub measurement_valid: bool,
    pub support_receipt: Option<SurveyFrameAuthorityReceipt>,
    pub support_passed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SurveyFrameOffer {
    pub id: String,
    pub revision: u64,
    pub use_scope: SurveyFrameUse,
    pub proof_at_offer: Option<String>,
    pub causal_position: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SurveyFrameResidentDecision {
    pub id: String,
    pub resident_id: String,
    pub participant_id: String,
    pub choice: SurveyFrameResidentChoice,
    pub use_scope: Option<SurveyFrameUse>,
    pub revision: u64,
    pub offer_id: Option<String>,
    pub proof_decision_id: Option<String>,
    pub causal_position: u64,
    pub withdrawal: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SurveyFrameDelivery {
    pub id: String,
    pub disposition: SurveyFrameDisposition,
    pub revision: u64,
    pub executor: String,
    pub recorder: String,
    pub recipient: String,
    pub custodian: String,
    pub recovery_owner: String,
    pub safe_route: String,
    pub outstanding_work: Vec<String>,
    pub causal_position: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SurveyFrameEvent {
    pub action: SurveyFrameAction,
    pub assisting_hueman: String,
    pub actor_id: String,
    pub from: SurveyFramePhase,
    pub to: SurveyFramePhase,
    pub object_revision: u64,
    pub authority: Option<SurveyFrameAuthorityReceipt>,
    pub resident_decision: Option<SurveyFrameResidentDecision>,
    pub delivery: Option<SurveyFrameDelivery>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SurveyFrameState {
    assisting_hueman: Option<String>,
    need_received: bool,
    registration: Option<SurveyFrameAuthorityReceipt>,
    artifact_name: Option<NameRecord>,
    revisions: Vec<SurveyFrameRevision>,
    offers: Vec<SurveyFrameOffer>,
    current_offer: Option<usize>,
    supported_choice: Option<SurveyFrameResidentChoice>,
    resident_decisions: Vec<SurveyFrameResidentDecision>,
    deliveries: Vec<SurveyFrameDelivery>,
    deployed: bool,
    recovery_required: bool,
    last_position: Option<u64>,
}

impl SurveyFrameState {
    #[must_use]
    pub fn phase(&self) -> SurveyFramePhase {
        if self.recovery_required {
            return SurveyFramePhase::RecoveryRequired;
        }
        if self.complete() {
            return SurveyFramePhase::Delivered;
        }
        if !self.deliveries.is_empty() {
            return SurveyFramePhase::AwaitingProtectedReturn;
        }
        if self.support_proven() {
            return SurveyFramePhase::RigProven;
        }
        if self.object_revision() == 3 {
            return SurveyFramePhase::Repaired;
        }
        if self
            .current()
            .is_some_and(|r| r.support_receipt.is_some() && !r.support_passed)
        {
            return SurveyFramePhase::RigIsolated;
        }
        if self.object_revision() == 2 {
            return SurveyFramePhase::RigPrepared;
        }
        if self.measurement_proven() {
            return SurveyFramePhase::MeasuringAvailable;
        }
        if !self.revisions.is_empty() {
            return SurveyFramePhase::Recovered;
        }
        if self.need_received {
            SurveyFramePhase::NeedRecorded
        } else {
            SurveyFramePhase::AwaitingNeed
        }
    }

    #[must_use]
    pub fn object_revision(&self) -> u64 {
        self.revisions.last().map_or(0, |r| r.number)
    }
    fn current(&self) -> Option<&SurveyFrameRevision> {
        self.revisions.last()
    }
    #[must_use]
    pub fn measurement_proven(&self) -> bool {
        self.current().is_some_and(|r| r.measurement_valid)
    }
    #[must_use]
    pub fn support_proven(&self) -> bool {
        self.current().is_some_and(|r| r.support_passed)
    }
    #[must_use]
    pub fn complete(&self) -> bool {
        self.deliveries.last().is_some_and(|delivery| {
            !self.recovery_required
                && !self
                    .resident_decisions
                    .last()
                    .is_some_and(|d| d.withdrawal && d.causal_position > delivery.causal_position)
        })
    }
    #[must_use]
    pub fn revisions(&self) -> &[SurveyFrameRevision] {
        &self.revisions
    }
    #[must_use]
    pub fn offers(&self) -> &[SurveyFrameOffer] {
        &self.offers
    }
    #[must_use]
    pub fn resident_decisions(&self) -> &[SurveyFrameResidentDecision] {
        &self.resident_decisions
    }
    #[must_use]
    pub fn deliveries(&self) -> &[SurveyFrameDelivery] {
        &self.deliveries
    }
    #[must_use]
    pub fn registration(&self) -> Option<&SurveyFrameAuthorityReceipt> {
        self.registration.as_ref()
    }
    #[must_use]
    pub fn artifact_name(&self) -> Option<&NameRecord> {
        self.artifact_name.as_ref()
    }
    #[must_use]
    pub fn recovery_required(&self) -> bool {
        self.recovery_required
    }
    #[must_use]
    pub fn deployed(&self) -> bool {
        self.deployed
    }
    #[must_use]
    pub fn supported_choice(&self) -> Option<SurveyFrameResidentChoice> {
        self.supported_choice
    }

    fn has_acceptance(&self, choice: SurveyFrameResidentChoice) -> bool {
        let use_scope = if choice == SurveyFrameResidentChoice::AcceptMeasurements {
            SurveyFrameUse::MeasuringOnly
        } else {
            SurveyFrameUse::TemporaryRig
        };
        self.resident_decisions
            .iter()
            .rev()
            .find(|d| {
                d.revision == self.object_revision()
                    && (d.choice == choice
                        || (d.choice == SurveyFrameResidentChoice::WaitSafely
                            && (d.use_scope.is_none() || d.use_scope == Some(use_scope))))
            })
            .is_some_and(|d| d.choice == choice && !d.withdrawal)
    }

    /// This bounded case has at most three revisions, six offers, two measuring
    /// trials per revision, and two deliveries. The full parent journal retains
    /// advisory changes without adding another unbounded child journal to clone.
    #[must_use]
    pub fn available_actions(&self) -> Vec<SurveyFrameAction> {
        use SurveyFrameAction as A;
        use SurveyFrameDisposition as D;
        use SurveyFrameResidentChoice as C;
        if self.recovery_required {
            return vec![A::RecoverDeployedRig];
        }
        if self.complete() {
            return if self.deployed {
                vec![A::AskResidentToWithdraw]
            } else {
                Vec::new()
            };
        }
        if !self.need_received {
            return vec![A::ReceiveNeed];
        }
        // Following withdrawal, recovery is mandatory before protected return.
        if !self.deliveries.is_empty() {
            return vec![A::DeliverDisposition {
                disposition: D::ProtectedDelayDelivered,
            }];
        }
        let mut actions = Vec::new();
        if self.revisions.is_empty() {
            actions.push(A::RecoverParts);
        }
        if !self.revisions.is_empty() && self.registration.is_none() {
            actions.push(A::AskStonebendToRegister);
        }
        if self.registration.is_some() && !self.measurement_proven() {
            actions.push(A::AskReviewerToMeasure);
        }
        if self.object_revision() == 1 && self.measurement_proven() {
            actions.push(A::PrepareRig);
        }
        for use_scope in [SurveyFrameUse::MeasuringOnly, SurveyFrameUse::TemporaryRig] {
            if self.measurement_proven()
                && (use_scope == SurveyFrameUse::MeasuringOnly || self.object_revision() >= 2)
                && !self
                    .offers
                    .iter()
                    .any(|o| o.revision == self.object_revision() && o.use_scope == use_scope)
            {
                actions.push(A::OfferUse { use_scope });
            }
        }
        if self.object_revision() >= 2
            && self.measurement_proven()
            && self.current().is_some_and(|r| r.support_receipt.is_none())
        {
            actions.push(A::AskReviewerToTrialSupport);
        }
        if self.object_revision() == 2
            && self
                .current()
                .is_some_and(|r| r.support_receipt.is_some() && !r.support_passed)
        {
            actions.push(A::RepairJoint);
        }
        let offer = self
            .current_offer
            .and_then(|i| self.offers.get(i))
            .filter(|o| o.revision == self.object_revision());
        for choice in [C::AcceptMeasurements, C::AcceptTemporaryRig, C::WaitSafely] {
            let eligible = match choice {
                C::AcceptMeasurements => {
                    offer.is_some_and(|o| o.use_scope == SurveyFrameUse::MeasuringOnly)
                        && self.measurement_proven()
                }
                C::AcceptTemporaryRig => {
                    offer.is_some_and(|o| o.use_scope == SurveyFrameUse::TemporaryRig)
                        && self.support_proven()
                }
                C::WaitSafely => true,
            };
            let already = self.resident_decisions.iter().any(|d| {
                d.revision == self.object_revision()
                    && d.offer_id.as_deref() == offer.map(|o| o.id.as_str())
                    && !d.withdrawal
            });
            if eligible && !already && self.supported_choice != Some(choice) {
                actions.push(A::SupportResidentChoice { choice });
            }
        }
        if self.supported_choice.is_some() {
            actions.push(A::AskResidentToDecide);
        }
        if self.measurement_proven() && self.has_acceptance(C::AcceptMeasurements) {
            actions.push(A::DeliverDisposition {
                disposition: D::MeasurementsDelivered,
            });
        }
        if self.support_proven() && self.has_acceptance(C::AcceptTemporaryRig) {
            actions.push(A::DeliverDisposition {
                disposition: D::TemporaryRigDelivered,
            });
        }
        if self
            .resident_decisions
            .last()
            .is_some_and(|d| d.revision == self.object_revision() && d.choice == C::WaitSafely)
        {
            actions.push(A::DeliverDisposition {
                disposition: D::ProtectedDelayDelivered,
            });
        }
        actions
    }

    pub fn advance(
        &mut self,
        assisting_hueman: &str,
        action: SurveyFrameAction,
        world: &WorldSession,
        at: u64,
    ) -> Result<SurveyFrameEvent, SurveyFrameError> {
        let mut candidate = self.clone();
        let event = candidate.advance_in_place(assisting_hueman, action, world, at)?;
        candidate.validate()?;
        *self = candidate;
        Ok(event)
    }

    fn advance_in_place(
        &mut self,
        assisting_hueman: &str,
        action: SurveyFrameAction,
        world: &WorldSession,
        at: u64,
    ) -> Result<SurveyFrameEvent, SurveyFrameError> {
        if crate::institution::InstitutionalBeingId::new(assisting_hueman).is_err() {
            return Err(SurveyFrameError::MissingHueman);
        }
        if [
            SURVEY_FRAME_RESIDENT,
            SURVEY_FRAME_KEEPER,
            SURVEY_FRAME_RECORDER,
        ]
        .contains(&assisting_hueman)
        {
            return Err(SurveyFrameError::AssistingActorConflict);
        }
        if self
            .assisting_hueman
            .as_deref()
            .is_some_and(|id| id != assisting_hueman)
        {
            return Err(SurveyFrameError::HuemanChanged);
        }
        if self.last_position.is_some_and(|old| at <= old) {
            return Err(SurveyFrameError::CausalRegression);
        }
        if !self.available_actions().contains(&action) {
            return Err(SurveyFrameError::UnavailableAction(action));
        }
        let from = self.phase();
        let mut authority = None;
        let mut resident_decision = None;
        let mut delivery = None;
        use SurveyFrameAction as A;
        let actor = match action {
            A::ReceiveNeed => {
                self.need_received = true;
                self.assisting_hueman = Some(assisting_hueman.into());
                SURVEY_FRAME_RESIDENT
            }
            A::RecoverParts => {
                self.revisions.push(SurveyFrameRevision {
                    number: 1,
                    change: "recovered square measuring frame; former support rating unknown"
                        .into(),
                    source_parts: vec![
                        "part.survey-frame.rack-left-member".into(),
                        "part.survey-frame.rack-right-member".into(),
                        DAMAGED_JOINT.into(),
                    ],
                    unresolved_provenance: vec![
                        "damaged joint's earlier source and load rating remain unknown".into(),
                    ],
                    retained_parts: Vec::new(),
                    measurement_receipts: Vec::new(),
                    measurement_valid: false,
                    support_receipt: None,
                    support_passed: false,
                });
                SURVEY_FRAME_KEEPER
            }
            A::AskStonebendToRegister => {
                let receipt = authority_receipt(
                    world,
                    self.object_revision(),
                    SurveyFrameClaim::ArtifactIdentityAndDisclosedCustody,
                    at,
                )?;
                self.artifact_name = Some(NameRecord {
                    id: NameRecordId::new("name.survey-frame.artifact.v1").map_err(|_| SurveyFrameError::InvalidState)?,
                    subject: IdentityId::new(SURVEY_FRAME_OBJECT_ID).map_err(|_| SurveyFrameError::InvalidState)?,
                    name: "Repair-bench survey frame".into(), class: NameClass::Artifact,
                    scope: "continuing object identity and disclosed repair-bench custody; no load rating or Title".into(),
                    exclusive: true, status: NameStatus::Active,
                    evidence: vec![EvidenceRecordId::new("evidence.survey-frame.recovered-parts.v1").map_err(|_| SurveyFrameError::InvalidState)?],
                    former_names: Vec::new(),
                });
                self.registration = Some(receipt.clone());
                authority = Some(receipt);
                SURVEY_FRAME_RECORDER
            }
            A::AskReviewerToMeasure => {
                let receipt = authority_receipt(
                    world,
                    self.object_revision(),
                    SurveyFrameClaim::MeasurementOnly,
                    at,
                )?;
                let revision = self
                    .revisions
                    .last_mut()
                    .ok_or(SurveyFrameError::InvalidState)?;
                revision.measurement_receipts.push(receipt.clone());
                revision.measurement_valid = true;
                authority = Some(receipt);
                SURVEY_FRAME_RECORDER
            }
            A::PrepareRig | A::RepairJoint => {
                let old = self
                    .current()
                    .ok_or(SurveyFrameError::InvalidState)?
                    .clone();
                let repairing = action == A::RepairJoint;
                let mut parts = old.source_parts;
                if repairing {
                    parts.retain(|p| p != DAMAGED_JOINT);
                    parts.push(REPLACEMENT_JOINT.into());
                } else {
                    parts.push("part.survey-frame.traceable-tension-line".into());
                }
                self.revisions.push(SurveyFrameRevision {
                    number: old.number + 1,
                    change: if repairing { "joint replaced; old failure and damaged joint retained; fresh proof required" } else { "removable tension line, adjustable rig and disclosed release added at bench; fresh measuring proof required" }.into(),
                    source_parts: parts,
                    unresolved_provenance: if repairing { Vec::new() } else { old.unresolved_provenance },
                    retained_parts: if repairing { vec![DAMAGED_JOINT.into()] } else { Vec::new() },
                    measurement_receipts: Vec::new(), measurement_valid: false, support_receipt: None, support_passed: false,
                });
                self.current_offer = None;
                self.supported_choice = None;
                SURVEY_FRAME_KEEPER
            }
            A::OfferUse { use_scope } => {
                let proof = self
                    .current()
                    .and_then(|r| match use_scope {
                        SurveyFrameUse::MeasuringOnly => r.measurement_receipts.last(),
                        SurveyFrameUse::TemporaryRig => {
                            r.support_receipt.as_ref().filter(|_| r.support_passed)
                        }
                    })
                    .map(|r| r.decision_id.clone());
                self.offers.push(SurveyFrameOffer {
                    id: format!("offer.survey-frame.{}.{}", self.object_revision(), at),
                    revision: self.object_revision(),
                    use_scope,
                    proof_at_offer: proof,
                    causal_position: at,
                });
                self.current_offer = Some(self.offers.len() - 1);
                self.supported_choice = None;
                SURVEY_FRAME_KEEPER
            }
            A::AskReviewerToTrialSupport => {
                let passed = self.object_revision() == 3;
                let receipt = authority_receipt(
                    world,
                    self.object_revision(),
                    if passed {
                        SurveyFrameClaim::ScopedTemporarySupport
                    } else {
                        SurveyFrameClaim::FailedSupportTrial
                    },
                    at,
                )?;
                let revision = self
                    .revisions
                    .last_mut()
                    .ok_or(SurveyFrameError::InvalidState)?;
                revision.support_receipt = Some(receipt.clone());
                revision.support_passed = passed;
                if !passed {
                    revision.measurement_valid = false;
                    self.supported_choice = None;
                }
                authority = Some(receipt);
                SURVEY_FRAME_RECORDER
            }
            A::SupportResidentChoice { choice } => {
                self.supported_choice = Some(choice);
                assisting_hueman
            }
            A::AskResidentToDecide => {
                let choice = self
                    .supported_choice
                    .take()
                    .ok_or(SurveyFrameError::InvalidState)?;
                // Revalidate the proposed use at commitment, not only at advice.
                let eligible = match choice {
                    SurveyFrameResidentChoice::AcceptMeasurements => self.measurement_proven(),
                    SurveyFrameResidentChoice::AcceptTemporaryRig => self.support_proven(),
                    SurveyFrameResidentChoice::WaitSafely => true,
                };
                if !eligible {
                    return Err(SurveyFrameError::InvalidState);
                }
                let decision = self.resident_decision(choice, at, false);
                self.resident_decisions.push(decision.clone());
                resident_decision = Some(decision);
                SURVEY_FRAME_RESIDENT
            }
            A::DeliverDisposition { disposition } => {
                let receipt = SurveyFrameDelivery {
                    id: format!("delivery.survey-frame.{at}"), disposition, revision: self.object_revision(),
                    executor: SURVEY_FRAME_KEEPER.into(), recorder: SURVEY_FRAME_RECORDER.into(), recipient: SURVEY_FRAME_RESIDENT.into(),
                    custodian: SURVEY_FRAME_KEEPER.into(), recovery_owner: SURVEY_FRAME_KEEPER.into(), safe_route: SURVEY_FRAME_SAFE_ROUTE.into(),
                    outstanding_work: match disposition {
                        SurveyFrameDisposition::MeasurementsDelivered => vec!["doorway support remains undelivered; any support use needs exact proof and acceptance".into()],
                        SurveyFrameDisposition::TemporaryRigDelivered => vec!["temporary scope, visible release, custody and recovery duty continue; withdrawal stops deployment".into()],
                        SurveyFrameDisposition::ProtectedDelayDelivered => vec!["access need, uncertain work and retained evidence remain addressable; no forced deployment".into()],
                    }, causal_position: at,
                };
                self.deployed = disposition == SurveyFrameDisposition::TemporaryRigDelivered;
                self.deliveries.push(receipt.clone());
                delivery = Some(receipt);
                SURVEY_FRAME_KEEPER
            }
            A::AskResidentToWithdraw => {
                let decision =
                    self.resident_decision(SurveyFrameResidentChoice::WaitSafely, at, true);
                self.resident_decisions.push(decision.clone());
                resident_decision = Some(decision);
                self.deployed = false;
                self.recovery_required = true;
                self.supported_choice = None;
                SURVEY_FRAME_RESIDENT
            }
            A::RecoverDeployedRig => {
                self.recovery_required = false;
                SURVEY_FRAME_KEEPER
            }
        };
        self.last_position = Some(at);
        Ok(SurveyFrameEvent {
            action,
            assisting_hueman: assisting_hueman.into(),
            actor_id: actor.into(),
            from,
            to: self.phase(),
            object_revision: self.object_revision(),
            authority,
            resident_decision,
            delivery,
        })
    }

    fn resident_decision(
        &self,
        choice: SurveyFrameResidentChoice,
        at: u64,
        withdrawal: bool,
    ) -> SurveyFrameResidentDecision {
        SurveyFrameResidentDecision {
            id: format!("choice.survey-frame.mercy-deep.{at}"),
            resident_id: SURVEY_FRAME_RESIDENT.into(),
            participant_id: "participant.current-sea.mercy-deep".into(),
            choice,
            use_scope: if withdrawal {
                None
            } else {
                self.current_offer
                    .and_then(|i| self.offers.get(i))
                    .map(|o| o.use_scope)
            },
            revision: self.object_revision(),
            offer_id: self
                .current_offer
                .and_then(|i| self.offers.get(i))
                .map(|o| o.id.clone()),
            proof_decision_id: match choice {
                SurveyFrameResidentChoice::AcceptMeasurements => self
                    .current()
                    .and_then(|r| r.measurement_receipts.last())
                    .map(|r| r.decision_id.clone()),
                SurveyFrameResidentChoice::AcceptTemporaryRig => self
                    .current()
                    .and_then(|r| r.support_receipt.as_ref())
                    .map(|r| r.decision_id.clone()),
                SurveyFrameResidentChoice::WaitSafely => None,
            },
            causal_position: at,
            withdrawal,
        }
    }

    pub fn validate(&self) -> Result<(), SurveyFrameError> {
        if self.revisions.len() > 3
            || self.offers.len() > 6
            || self.resident_decisions.len() > 13
            || self.deliveries.len() > 2
        {
            return Err(SurveyFrameError::InvalidState);
        }
        for (index, revision) in self.revisions.iter().enumerate() {
            if revision.number != index as u64 + 1
                || revision.measurement_receipts.len() > 2
                || (revision.measurement_valid && revision.measurement_receipts.is_empty())
                || (revision.support_passed
                    && !revision.support_receipt.as_ref().is_some_and(|r| {
                        r.claim == SurveyFrameClaim::ScopedTemporarySupport && revision.number == 3
                    }))
                || revision.measurement_receipts.iter().any(|r| {
                    r.object_revision != revision.number
                        || r.claim != SurveyFrameClaim::MeasurementOnly
                })
                || revision
                    .support_receipt
                    .as_ref()
                    .is_some_and(|r| r.object_revision != revision.number)
            {
                return Err(SurveyFrameError::InvalidState);
            }
        }
        if self.registration.is_some() != self.artifact_name.is_some()
            || (self.revisions.is_empty() && self.registration.is_some())
            || self.artifact_name.as_ref().is_some_and(|n| {
                n.subject.as_str() != SURVEY_FRAME_OBJECT_ID
                    || n.class != NameClass::Artifact
                    || n.status != NameStatus::Active
            })
            || (self.deployed
                && (!self.support_proven()
                    || !self.has_acceptance(SurveyFrameResidentChoice::AcceptTemporaryRig)))
            || (self.recovery_required && self.deployed)
        {
            return Err(SurveyFrameError::InvalidState);
        }
        Ok(())
    }
}

fn authority_receipt(
    world: &WorldSession,
    revision: u64,
    claim: SurveyFrameClaim,
    at: u64,
) -> Result<SurveyFrameAuthorityReceipt, SurveyFrameError> {
    let function = if claim == SurveyFrameClaim::ArtifactIdentityAndDisclosedCustody {
        HouseFunction::Name
    } else {
        HouseFunction::Prove
    };
    let claim_key = match claim {
        SurveyFrameClaim::ArtifactIdentityAndDisclosedCustody => "artifact-registration",
        SurveyFrameClaim::MeasurementOnly => "measurement",
        SurveyFrameClaim::FailedSupportTrial => "failed-support-trial",
        SurveyFrameClaim::ScopedTemporarySupport => "scoped-temporary-support",
    };
    let label = format!("survey-frame.r{revision}.{claim_key}.{at}");
    let decision = scenario_house_decision(&world.institutional().catalog, &label, function, at)
        .map_err(|e| SurveyFrameError::Authority(e.to_string()))?;
    decision
        .require_accepted(function)
        .map_err(|e| SurveyFrameError::Authority(e.to_string()))?;
    if [SURVEY_FRAME_KEEPER, SURVEY_FRAME_RESIDENT].contains(&decision.authority.actor.as_str()) {
        return Err(SurveyFrameError::Authority(
            "case executor or recipient cannot independently certify this frame".into(),
        ));
    }
    Ok(SurveyFrameAuthorityReceipt {
        decision_id: decision.id.as_str().into(), function: format!("{function:?}"), office_id: decision.authority.office.as_str().into(),
        authority_actor: decision.authority.actor.as_str().into(), causal_position: at, object_id: SURVEY_FRAME_OBJECT_ID.into(), object_revision: revision, claim,
        scope: match claim {
            SurveyFrameClaim::ArtifactIdentityAndDisclosedCustody => "artifact identity and Tess's disclosed custody; unknown history stays unknown",
            SurveyFrameClaim::MeasurementOnly => "this exact frame revision measures this doorway; no support rating",
            SurveyFrameClaim::FailedSupportTrial => "negative trial result: the revision-two rig connection slips; isolate, retain evidence and remeasure",
            SurveyFrameClaim::ScopedTemporarySupport => "this repaired revision's temporary use at Mercy Deep's doorway, with release and recovery; no general rating",
        }.into(),
        evidence: decision.evidence.iter().map(|e| format!("{:?}", e.0)).collect(), independent_recorder: SURVEY_FRAME_RECORDER.into(),
    })
}

#[must_use]
pub fn survey_frame_lore() -> crate::world::lived_lore::FunctionalLoreDefinition {
    use crate::world::lived_lore::FunctionalLoreDefinition;
    FunctionalLoreDefinition {
        id: SURVEY_FRAME_LORE_ID,
        house: crate::hollow_grove_contract::House::Stonebend,
        authority_function: HouseFunction::Name,
        authority_class: HouseFunction::Name.required_authority(),
        route: crate::world::geography::ConstitutionalRouteId::AuraWay,
        location: "Stonebend Aura Way repair bench and Mercy Deep's damaged doorway",
        jurisdiction: "Stonebend artifact identity/custody; Sandmanor exact-revision proof; resident's voluntary use",
        entities: &[
            "Mercy Deep",
            "Tess Splice",
            "Ivo Mark",
            "repairable survey frame",
            "retained damaged joint",
            "alternate safe route",
        ],
        dominant_verb: "Name",
        trigger: "Mercy Deep asks for a survey before choosing temporary support",
        evidence: &[
            "recovered-member provenance and unknown joint history",
            "exact-revision measuring proof",
            "retained failed support trial",
            "traceable repair",
            "resident decision",
            "delivery and recovery receipt",
        ],
        uncertainty: &[
            "former load rating is unknown",
            "measurement is not support",
            "trial is a bounded case demonstration, not universal engineering simulation",
        ],
        player_choices: &[
            "assist recovery",
            "request independent trials",
            "support a resident choice without deciding it",
            "repair",
            "deliver scoped result",
            "request withdrawal and recovery",
        ],
        lawful_state_changes: &[
            "same artifact recovered and registered",
            "revision measured",
            "negative support result retained",
            "refusal protected",
            "repaired revision requires fresh proof",
            "bounded delivery and withdrawal recovery",
        ],
        persistence_and_replay: "gameplay V6 records typed actions, exact results and authority; older saves invent no frame or consent",
        presentation: "read-only workshop view and typed client intent; existing palette and map authority retained",
        failure_and_refusal: &[
            "vacant House authority",
            "proof from the wrong revision",
            "advice treated as acceptance",
            "refused or withdrawn use",
            "forged authority or results",
            "missing recovery",
        ],
        affected_actor: "Mercy Deep",
        decision_owner: "Stonebend registry authority",
        executor: "Tess Splice",
        witness: "Ivo Mark",
        ordered_handoffs: &[
            "Stonebend registry authority→Tess Splice: Which continuing artifact and disclosed custody are recorded?",
            "Tess Splice→Ivo Mark: Which exact revision, negative trial, repair and independently proved use occurred?",
            "Ivo Mark→Mercy Deep: Which voluntary disposition, access, evidence and recovery obligations are delivered?",
        ],
        completion_condition: "an actual scoped delivery to Mercy Deep with exact proof/acceptance for use or protected delay; withdrawal requires recovery and protected return",
        restitution_owner: "Tess Splice",
        recipient: "Mercy Deep",
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SurveyFrameError {
    MissingHueman,
    HuemanChanged,
    AssistingActorConflict,
    CausalRegression,
    UnavailableAction(SurveyFrameAction),
    Authority(String),
    InvalidState,
    ReplayDivergence,
}
impl fmt::Display for SurveyFrameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "survey-frame action rejected: {self:?}")
    }
}
impl std::error::Error for SurveyFrameError {}
