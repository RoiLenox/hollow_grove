//! Glaüshouse's bounded, host-specific Sympiote foundation and lived practice.
//! Authority stays in the live House catalog; grafting reuses the existing
//! Sympiote reducer. Nothing in this module belongs to the neutral kernel.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::constitutional::{HouseFunction, scenario_house_decision};
use crate::world::session::WorldSession;
use crate::world::sympiote::{
    SympioteAction, SympioteActionRecord, SympioteGraft, SympioteIntegrationOutcome, SympiotePhase,
};

pub const SYMPIOTE_FOUNDATION_SOURCE: &str = "HOLLOW_GROVE_SYMPIOTE_FOUNDATION_V1.md";
pub const SYMPIOTE_FOUNDATION_FORM_ID: &str = "form.sympiote.foundation.v1";
pub const SYMPIOTE_FOUNDATION_CASE_ID: &str = "case.glaushouse.sympiote-foundation.v1";
pub const SYMPIOTE_FOUNDATION_FACILITY: &str = "institution.glaushouse.glauspitals";

/// Separate additive case; the historical twelve-route roster remains exact.
#[must_use]
pub fn sympiote_foundation_lore() -> crate::world::lived_lore::FunctionalLoreDefinition {
    crate::world::lived_lore::FunctionalLoreDefinition {
        id: "lore.glaushouse.sympiote-foundation",
        house: crate::hollow_grove_contract::House::Glaushouse,
        authority_function: HouseFunction::Clear,
        authority_class: HouseFunction::Clear.required_authority(),
        route: crate::world::geography::ConstitutionalRouteId::Boardwalk,
        location: "Glauspitals clinical establishment and recovery; voluntary Boardwalk return",
        jurisdiction: "Glaüshouse clinical scope; later traditions retain their own authority",
        entities: &[
            "continuing Hueman",
            "live Glaüshouse clinician",
            "clinical recorder",
            "Pimp and Trooners",
            "Gimp and Gooners",
        ],
        dominant_verb: "Clear",
        trigger: "Hueman requests a host-specific Sympiote foundation assessment",
        evidence: &[
            "disclosed host-bound profile",
            "personal consent or withdrawal",
            "live clinical authority receipt",
            "ordered graft result",
            "rest and discharge",
            "bounded adaptation, release, and rest",
        ],
        uncertainty: &[
            "later Forms are not guaranteed by foundation practice",
            "this bounded case does not supply a general clinical patient model",
            "exact inventories and later path mechanisms remain open",
        ],
        player_choices: &[
            "request assessment",
            "consent",
            "withdraw",
            "request establishment",
            "rest",
            "request discharge",
            "practice",
            "report strain",
            "request review",
            "explore a direction",
            "consider a Boardwalk offer",
        ],
        lawful_state_changes: &[
            "host-bound profile recorded",
            "specific consent recorded",
            "refusal preserved with no new Form",
            "actual reciprocal graft establishes foundation",
            "rest obligation met",
            "clinical return recorded",
            "limited practice recorded",
            "care required with Form retained",
            "review and renewed recovery plan",
            "interest recorded without another Form",
            "offer observation without purchase or affiliation",
        ],
        persistence_and_replay: "gameplay archive V5 replays parent actions, clinical receipts, and graft child history; V1-V4 migration invents no foundation progress",
        presentation: "read-only GameView and lore inspection; dedicated clinic map and new sprites remain future presentation work",
        failure_and_refusal: &[
            "missing identity or authority",
            "withdrawn consent",
            "changed establishment operator",
            "practice before rest",
            "strain requiring clinical review",
            "tampered or causally divergent replay",
        ],
        affected_actor: "Continuing Hueman",
        decision_owner: "Glaüshouse clinical authority",
        executor: "Qualified clinical practitioner",
        witness: "Clinical and practice recorder",
        ordered_handoffs: &[
            "Glaüshouse clinical authority→Qualified clinical practitioner: Is this host-bound establishment consented and within the disclosed clinical scope?",
            "Qualified clinical practitioner→Clinical and practice recorder: What actual graft result, recovery, and bounded practice occurred?",
            "Clinical and practice recorder→Continuing Hueman: What Form lives now, what care remains, and is voluntary return and comfortable use evidenced?",
        ],
        completion_condition: "same host, actual reciprocal foundation, clinical discharge, and two uses of one adaptation separated by rest and followed by recovery; no later Form implied",
        restitution_owner: "Glaüshouse clinical recovery team",
        recipient: "Continuing Hueman",
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SympioteFoundationPhase {
    AwaitingAssessment,
    AwaitingConsent,
    Consented,
    Recovering,
    Discharged,
    CareRequired,
    Refused,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SympioteAdaptation {
    Balance,
    Tension,
    Coordination,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SympiotePracticeDirection {
    Stonebend,
    Minorian,
    Minoan,
    Glaushouse,
    Flynt,
}

impl SympiotePracticeDirection {
    pub const ALL: [Self; 5] = [
        Self::Stonebend,
        Self::Minorian,
        Self::Minoan,
        Self::Glaushouse,
        Self::Flynt,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SympioteBoardwalkCamp {
    PimpAndTrooners,
    GimpAndGooners,
}

/// Requests and personal choices, never client-supplied clinical truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum SympioteFoundationAction {
    RequestAssessment,
    ConsentToFoundation,
    WithdrawConsent,
    AskClinicianToEstablish,
    Rest,
    AskClinicianToDischarge,
    Practice {
        adaptation: SympioteAdaptation,
    },
    ReportStrain,
    AskClinicianToReview,
    ExploreDirection {
        direction: SympiotePracticeDirection,
    },
    ConsiderBoardwalkOffer {
        camp: SympioteBoardwalkCamp,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SympioteFoundationProfile {
    pub form_id: String,
    pub subject_id: String,
    pub facility: String,
    pub appearance: String,
    pub capacity_scope: String,
    pub maintenance: Vec<String>,
    pub limits: Vec<String>,
}

impl SympioteFoundationProfile {
    fn for_subject(subject: &str) -> Self {
        Self {
            form_id: SYMPIOTE_FOUNDATION_FORM_ID.into(),
            subject_id: subject.into(),
            facility: SYMPIOTE_FOUNDATION_FACILITY.into(),
            appearance: "recognizable continuous person; functional changes in posture, movement, and tissue response".into(),
            capacity_scope: "small controlled adjustments of balance, tension, or coordination; release after one bounded exercise".into(),
            maintenance: vec!["appropriate nourishment".into(), "rest between exercises".into(), "clinical follow-up for changed behavior or strain".into()],
            limits: vec!["no unrestricted shapeshifting or selected power package".into(), "practice does not grant a later Form, Aura move, faculty, office, or crew affiliation".into(), "stop before repeating an exercise while rest is needed".into()],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SympioteClinicalReceipt {
    pub decision_id: String,
    pub authority_actor: String,
    pub office: String,
    pub function: String,
    pub subject_id: String,
    pub scope: String,
    pub causal_position: u64,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SympioteFoundationEvent {
    pub action: SympioteFoundationAction,
    pub subject_id: String,
    pub from: SympioteFoundationPhase,
    pub to: SympioteFoundationPhase,
    pub form_established: bool,
    pub needs_rest: bool,
    pub comfortable_use: bool,
    pub practice_count: u32,
    pub clinical_receipt: Option<SympioteClinicalReceipt>,
    pub evidence_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SympioteFoundationState {
    phase: SympioteFoundationPhase,
    profile: Option<SympioteFoundationProfile>,
    graft: Option<SympioteGraft>,
    assessment_actor: Option<String>,
    consented: bool,
    needs_rest: bool,
    rested: bool,
    practice_count: u32,
    practiced_adaptation: Option<SympioteAdaptation>,
    comfortable_use: bool,
    direction: Option<SympiotePracticeDirection>,
    considered_offers: BTreeSet<SympioteBoardwalkCamp>,
    clinical_receipts: Vec<SympioteClinicalReceipt>,
    last_position: Option<u64>,
}

impl Default for SympioteFoundationState {
    fn default() -> Self {
        Self {
            phase: SympioteFoundationPhase::AwaitingAssessment,
            profile: None,
            graft: None,
            assessment_actor: None,
            consented: false,
            needs_rest: false,
            rested: false,
            practice_count: 0,
            practiced_adaptation: None,
            comfortable_use: false,
            direction: None,
            considered_offers: BTreeSet::new(),
            clinical_receipts: Vec::new(),
            last_position: None,
        }
    }
}

impl SympioteFoundationState {
    pub fn phase(&self) -> SympioteFoundationPhase {
        self.phase
    }
    pub fn profile(&self) -> Option<&SympioteFoundationProfile> {
        self.profile.as_ref()
    }
    pub fn graft(&self) -> Option<&SympioteGraft> {
        self.graft.as_ref()
    }
    pub fn needs_rest(&self) -> bool {
        self.needs_rest
    }
    pub fn comfortable_use(&self) -> bool {
        self.comfortable_use
    }
    pub fn practice_count(&self) -> u32 {
        self.practice_count
    }
    pub fn direction(&self) -> Option<SympiotePracticeDirection> {
        self.direction
    }
    pub fn considered_offers(&self) -> &BTreeSet<SympioteBoardwalkCamp> {
        &self.considered_offers
    }
    pub fn clinical_receipts(&self) -> &[SympioteClinicalReceipt] {
        &self.clinical_receipts
    }
    pub fn form_established(&self) -> bool {
        self.graft
            .as_ref()
            .is_some_and(SympioteGraft::successful_reciprocal_synthesis)
    }

    pub fn available_actions(&self) -> Vec<SympioteFoundationAction> {
        use SympioteFoundationAction as A;
        use SympioteFoundationPhase as P;
        match self.phase {
            P::AwaitingAssessment | P::Refused => vec![A::RequestAssessment],
            P::AwaitingConsent => vec![A::ConsentToFoundation, A::WithdrawConsent],
            P::Consented => vec![
                A::AskClinicianToEstablish,
                A::WithdrawConsent,
                A::RequestAssessment,
            ],
            P::Recovering => {
                if self.needs_rest {
                    vec![A::Rest, A::ReportStrain]
                } else {
                    vec![A::AskClinicianToDischarge, A::ReportStrain]
                }
            }
            P::CareRequired => {
                if self.rested {
                    vec![A::AskClinicianToReview]
                } else {
                    vec![A::Rest]
                }
            }
            P::Discharged => {
                let mut actions = if self.needs_rest {
                    vec![A::Rest, A::ReportStrain]
                } else {
                    vec![
                        A::Practice {
                            adaptation: SympioteAdaptation::Balance,
                        },
                        A::Practice {
                            adaptation: SympioteAdaptation::Tension,
                        },
                        A::Practice {
                            adaptation: SympioteAdaptation::Coordination,
                        },
                        A::ReportStrain,
                    ]
                };
                actions.extend(
                    SympiotePracticeDirection::ALL
                        .map(|direction| A::ExploreDirection { direction }),
                );
                actions.extend(
                    [
                        SympioteBoardwalkCamp::PimpAndTrooners,
                        SympioteBoardwalkCamp::GimpAndGooners,
                    ]
                    .map(|camp| A::ConsiderBoardwalkOffer { camp }),
                );
                actions
            }
        }
    }

    /// Clone-and-commit preserves every field on a failed request or overflow.
    pub fn advance(
        &mut self,
        subject: &str,
        action: SympioteFoundationAction,
        world: &WorldSession,
        at: u64,
    ) -> Result<SympioteFoundationEvent, SympioteFoundationError> {
        let mut candidate = self.clone();
        let event = candidate.advance_in_place(subject, action, world, at)?;
        candidate.validate()?;
        *self = candidate;
        Ok(event)
    }

    fn advance_in_place(
        &mut self,
        subject: &str,
        action: SympioteFoundationAction,
        world: &WorldSession,
        at: u64,
    ) -> Result<SympioteFoundationEvent, SympioteFoundationError> {
        if subject.trim().is_empty()
            || crate::institution::InstitutionalBeingId::new(subject).is_err()
        {
            return Err(SympioteFoundationError::MissingSubject);
        }
        if self
            .profile
            .as_ref()
            .is_some_and(|profile| profile.subject_id != subject)
        {
            return Err(SympioteFoundationError::SubjectChanged);
        }
        if self.last_position.is_some_and(|previous| at <= previous) {
            return Err(SympioteFoundationError::CausalRegression);
        }
        if !self.available_actions().contains(&action) {
            return Err(SympioteFoundationError::UnavailableAction(action));
        }
        let from = self.phase;
        let mut clinical_receipt = None;
        use SympioteFoundationAction as A;
        use SympioteFoundationPhase as P;
        match action {
            A::RequestAssessment => {
                let receipt = clinical_receipt_for(
                    world,
                    subject,
                    "assessment-and-disclosed-foundation-scope",
                    at,
                )?;
                self.assessment_actor = Some(receipt.authority_actor.clone());
                self.profile = Some(SympioteFoundationProfile::for_subject(subject));
                self.consented = false;
                self.phase = P::AwaitingConsent;
                clinical_receipt = Some(receipt);
            }
            A::ConsentToFoundation => {
                self.consented = true;
                self.phase = P::Consented;
            }
            A::WithdrawConsent => {
                self.consented = false;
                self.phase = P::Refused;
            }
            A::AskClinicianToEstablish => {
                if !self.consented {
                    return Err(SympioteFoundationError::ConsentRequired);
                }
                let receipt = clinical_receipt_for(
                    world,
                    subject,
                    "host-specific-foundation-establishment",
                    at,
                )?;
                if self.assessment_actor.as_deref() != Some(receipt.authority_actor.as_str()) {
                    return Err(SympioteFoundationError::OperatorChanged);
                }
                let first = at
                    .checked_mul(8)
                    .ok_or(SympioteFoundationError::CausalOverflow)?;
                let mut graft =
                    SympioteGraft::new(format!("graft.sympiote-foundation.{subject}"), subject);
                let actions = [
                    SympioteAction::SampleHost,
                    SympioteAction::ReadHostCurrentAuraRecipe,
                    SympioteAction::CultivateLivingTissue,
                    SympioteAction::CraftForHost {
                        requested_power_package: None,
                    },
                    SympioteAction::GraftWithConsent,
                    SympioteAction::BeginCompatibilityMonitoring,
                    SympioteAction::ResolveIntegration {
                        outcome: SympioteIntegrationOutcome::ReciprocalSynthesis,
                        emergent_form: Some(SYMPIOTE_FOUNDATION_FORM_ID.into()),
                    },
                ];
                for (index, graft_action) in actions.into_iter().enumerate() {
                    graft
                        .apply(SympioteActionRecord {
                            causal_position: first
                                .checked_add(index as u64)
                                .ok_or(SympioteFoundationError::CausalOverflow)?,
                            evidence: format!(
                                "evidence.sympiote-foundation.{subject}.{at}.graft-{index}"
                            ),
                            action: graft_action,
                        })
                        .map_err(|error| SympioteFoundationError::Graft(error.to_string()))?;
                }
                self.graft = Some(graft);
                self.phase = P::Recovering;
                self.needs_rest = true;
                self.rested = false;
                clinical_receipt = Some(receipt);
            }
            A::Rest => {
                self.needs_rest = false;
                self.rested = true;
                if self.phase == P::Discharged && self.practice_count >= 2 {
                    self.comfortable_use = true;
                }
            }
            A::AskClinicianToDischarge => {
                if self.needs_rest || !self.rested {
                    return Err(SympioteFoundationError::RecoveryRequired);
                }
                clinical_receipt = Some(clinical_receipt_for(
                    world,
                    subject,
                    "recovery-teaching-and-voluntary-discharge",
                    at,
                )?);
                self.phase = P::Discharged;
            }
            A::Practice { adaptation } => {
                self.comfortable_use = false;
                self.practice_count = if self.practiced_adaptation == Some(adaptation) {
                    self.practice_count
                        .checked_add(1)
                        .ok_or(SympioteFoundationError::PracticeOverflow)?
                } else {
                    1
                };
                self.practiced_adaptation = Some(adaptation);
                self.needs_rest = true;
                self.rested = false;
            }
            A::ReportStrain => {
                self.phase = P::CareRequired;
                self.needs_rest = true;
                self.rested = false;
                self.comfortable_use = false;
                self.practice_count = 0;
                self.practiced_adaptation = None;
            }
            A::AskClinicianToReview => {
                clinical_receipt = Some(clinical_receipt_for(
                    world,
                    subject,
                    "strain-review-and-renewed-care-plan",
                    at,
                )?);
                self.phase = P::Recovering;
                self.needs_rest = true;
                self.rested = false;
            }
            A::ExploreDirection { direction } => self.direction = Some(direction),
            A::ConsiderBoardwalkOffer { camp } => {
                self.considered_offers.insert(camp);
            }
        }
        if let Some(receipt) = &clinical_receipt {
            self.clinical_receipts.push(receipt.clone());
        }
        self.last_position = Some(at);
        Ok(SympioteFoundationEvent {
            action,
            subject_id: subject.into(),
            from,
            to: self.phase,
            form_established: self.form_established(),
            needs_rest: self.needs_rest,
            comfortable_use: self.comfortable_use,
            practice_count: self.practice_count,
            clinical_receipt,
            evidence_id: format!("evidence.sympiote-foundation.{subject}.{at}"),
        })
    }

    pub fn validate(&self) -> Result<(), SympioteFoundationError> {
        if let Some(profile) = &self.profile {
            if profile.form_id != SYMPIOTE_FOUNDATION_FORM_ID
                || profile.facility != SYMPIOTE_FOUNDATION_FACILITY
            {
                return Err(SympioteFoundationError::InvalidEstablishedForm);
            }
            if self.clinical_receipts.iter().any(|receipt| {
                receipt.subject_id != profile.subject_id
                    || receipt.office != "office.glaushouse.prima-donna"
                    || receipt.function != "Clear"
                    || receipt.authority_actor.is_empty()
                    || receipt.evidence.is_empty()
                    || self
                        .last_position
                        .is_none_or(|at| receipt.causal_position > at)
            }) {
                return Err(SympioteFoundationError::InvalidEstablishedForm);
            }
            if self
                .clinical_receipts
                .windows(2)
                .any(|pair| pair[0].causal_position >= pair[1].causal_position)
            {
                return Err(SympioteFoundationError::CausalRegression);
            }
        }
        if self.form_established() {
            let profile = self
                .profile
                .as_ref()
                .ok_or(SympioteFoundationError::MissingSubject)?;
            let graft = self
                .graft
                .as_ref()
                .ok_or(SympioteFoundationError::MissingSubject)?;
            if !self.consented
                || graft.host_id != profile.subject_id
                || graft.phase != SympiotePhase::ReciprocallyIntegrated
                || graft.emergent_form.as_deref() != Some(SYMPIOTE_FOUNDATION_FORM_ID)
                || self.clinical_receipts.len() < 2
            {
                return Err(SympioteFoundationError::InvalidEstablishedForm);
            }
        } else if matches!(
            self.phase,
            SympioteFoundationPhase::Recovering
                | SympioteFoundationPhase::Discharged
                | SympioteFoundationPhase::CareRequired
        ) {
            return Err(SympioteFoundationError::InvalidEstablishedForm);
        }
        if self.comfortable_use
            && (self.practice_count < 2 || self.phase != SympioteFoundationPhase::Discharged)
        {
            return Err(SympioteFoundationError::InvalidMilestone);
        }
        Ok(())
    }
}

fn clinical_receipt_for(
    world: &WorldSession,
    subject: &str,
    scope: &str,
    at: u64,
) -> Result<SympioteClinicalReceipt, SympioteFoundationError> {
    let decision = scenario_house_decision(
        &world.institutional().catalog,
        &format!("sympiote-foundation.{subject}.{at}.{scope}"),
        HouseFunction::Clear,
        at,
    )
    .map_err(|error| SympioteFoundationError::ClinicalAuthority(error.to_string()))?;
    decision
        .require_accepted(HouseFunction::Clear)
        .map_err(|error| SympioteFoundationError::ClinicalAuthority(error.to_string()))?;
    Ok(SympioteClinicalReceipt {
        decision_id: decision.id.as_str().into(),
        authority_actor: decision.authority.actor.as_str().into(),
        office: decision.authority.office.as_str().into(),
        function: "Clear".into(),
        subject_id: subject.into(),
        scope: scope.into(),
        causal_position: at,
        evidence: vec![format!(
            "evidence.sympiote-foundation.{subject}.{at}.{scope}"
        )],
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SympioteFoundationError {
    MissingSubject,
    SubjectChanged,
    CausalRegression,
    CausalOverflow,
    PracticeOverflow,
    ConsentRequired,
    RecoveryRequired,
    OperatorChanged,
    InvalidEstablishedForm,
    InvalidMilestone,
    UnavailableAction(SympioteFoundationAction),
    ClinicalAuthority(String),
    Graft(String),
    ReplayDivergence,
}

impl fmt::Display for SympioteFoundationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Sympiote foundation rejected: {self:?}")
    }
}
impl std::error::Error for SympioteFoundationError {}
