//! Headless House handoffs. A scoped decision, work, observation and receipt
//! are distinct. This service does not operate the game or grant Forms/offices.
use super::session::WorldSession;
use crate::composition::ExternalRef;
use crate::constitutional::{
    AuthorityActorId, CausalPosition, EvidenceRef, HouseDecision, HouseDecisionDraft,
    HouseDecisionId, HouseDecisionOutcome, HouseFunction,
};
use crate::institution::{InstitutionalBeingId, OfficeId};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const SOURCE: &str = "docs/BACKEND_HOUSE_HANDOFFS_V1.md";
pub const IMPLEMENTATION: &str = "src/world/lore_handoffs.rs";
pub const PROOF_SOURCE: &str = "tests/backend_house_handoffs.rs";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Route {
    Stairway,
    Basin,
    PublicName,
    Hollowing,
    PublicDesign,
    RecoveryDesign,
    CoastalProof,
    EmergencyIntake,
    HighRiskClearance,
    RecoveryDischarge,
}
pub const ROUTES: [Route; 10] = [
    Route::Stairway,
    Route::Basin,
    Route::PublicName,
    Route::Hollowing,
    Route::PublicDesign,
    Route::RecoveryDesign,
    Route::CoastalProof,
    Route::EmergencyIntake,
    Route::HighRiskClearance,
    Route::RecoveryDischarge,
];
impl Route {
    pub const fn lore_id(self) -> &'static str {
        match self {
            Self::Stairway => "lore.flynt.stairway-functional-recognition",
            Self::Basin => "lore.flynt.basin-dual-expression",
            Self::PublicName => "lore.stonebend.aura-ridge-public-name",
            Self::Hollowing => "lore.stonebend.mnt-aura-illegal-hollowing",
            Self::PublicDesign => "lore.sandmanor.aura-way-public-design",
            Self::RecoveryDesign => "lore.sandmanor.glausbahn-recovery-design",
            Self::CoastalProof => "lore.sandmanor.aura-beach-current-sea-role-proof",
            Self::EmergencyIntake => "lore.glaushouse.riptide-emergency-intake",
            Self::HighRiskClearance => "lore.glaushouse.current-seanad-high-risk-clearance",
            Self::RecoveryDischarge => "lore.glaushouse.aura-ridge-recovery-discharge",
        }
    }
    pub const fn function(self) -> HouseFunction {
        match self {
            Self::Stairway | Self::Basin => HouseFunction::Recognize,
            Self::PublicName | Self::Hollowing => HouseFunction::Name,
            Self::PublicDesign | Self::RecoveryDesign | Self::CoastalProof => HouseFunction::Prove,
            _ => HouseFunction::Clear,
        }
    }
    pub fn definition(self) -> super::lived_lore::FunctionalLoreDefinition {
        super::lived_lore::canonical_functional_lore_definitions()
            .into_iter()
            .find(|d| d.id == self.lore_id())
            .expect("canonical handoff")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Actors {
    pub executor: String,
    pub witness: String,
    pub recipient: String,
    pub recovery_owner: String,
}
impl Actors {
    fn validate(&self) -> Result<(), Error> {
        for actor in [
            &self.executor,
            &self.witness,
            &self.recipient,
            &self.recovery_owner,
        ] {
            InstitutionalBeingId::new(actor.clone())
                .map_err(|_| Error::Invalid("invalid actor ID"))?;
        }
        require(
            self.executor != self.witness
                && self.witness != self.recipient
                && self.witness != self.recovery_owner
                && self.executor != self.recipient,
            "executor, independent witness and recipient must differ",
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Capacity {
    Capable,
    Unknown,
    Unable,
}

/// Reported domain facts, attributed to the independent recorder. These are
/// not sensor truth or clinical certification; decisions retain their owner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Facts {
    Stairway {
        work: Vec<String>,
        contributions: Vec<String>,
        scope: String,
    },
    Basin {
        cargo: String,
        custodian: String,
        urban_scope: String,
        rural_scope: String,
        contraband_uncertainty: String,
    },
    PublicName {
        former_names: Vec<String>,
        requested_name: String,
        liabilities: Vec<String>,
    },
    Hollowing {
        sample: String,
        subject_custodian: String,
        sample_custodian: String,
        missing_custody_history: String,
        restoration_uncertainty: String,
    },
    PublicDesign {
        design: String,
        interior_result: String,
        exterior_result: String,
        passed: bool,
        maintenance_uncertainty: String,
    },
    RecoveryDesign {
        method: String,
        lab_result: String,
        moving_failure: String,
        revised_moving_result: Option<String>,
        revised_moving_passed: bool,
        capacity: Capacity,
    },
    CoastalProof {
        original_body: String,
        reproducing_body: String,
        reproduced: bool,
        safer_within_conditions: bool,
        result: String,
    },
    EmergencyIntake {
        capacity: Capacity,
        immediate_stabilization_required: bool,
        urgent_threat: String,
        stabilization_scope: String,
    },
    HighRiskClearance {
        capacity: Capacity,
        proven_claim: String,
        proof_supported: bool,
        reversal_uncertainty: String,
        conditions: Vec<String>,
    },
    RecoveryDischarge {
        capacity: Capacity,
        travel_result: String,
        travel_proven_within_limits: bool,
        duty_limits: Vec<String>,
        full_load_uncertainty: String,
        recovery_plan: String,
    },
}
impl Facts {
    fn route(&self) -> Route {
        match self {
            Self::Stairway { .. } => Route::Stairway,
            Self::Basin { .. } => Route::Basin,
            Self::PublicName { .. } => Route::PublicName,
            Self::Hollowing { .. } => Route::Hollowing,
            Self::PublicDesign { .. } => Route::PublicDesign,
            Self::RecoveryDesign { .. } => Route::RecoveryDesign,
            Self::CoastalProof { .. } => Route::CoastalProof,
            Self::EmergencyIntake { .. } => Route::EmergencyIntake,
            Self::HighRiskClearance { .. } => Route::HighRiskClearance,
            Self::RecoveryDischarge { .. } => Route::RecoveryDischarge,
        }
    }
    fn capacity(&self) -> Option<Capacity> {
        match self {
            Self::RecoveryDesign { capacity, .. }
            | Self::EmergencyIntake { capacity, .. }
            | Self::HighRiskClearance { capacity, .. }
            | Self::RecoveryDischarge { capacity, .. } => Some(*capacity),
            _ => None,
        }
    }
    fn validate(&self, subject: &str) -> Result<(), Error> {
        match self {
            Self::Stairway {
                work,
                contributions,
                scope,
            } => {
                strings(work)?;
                strings(contributions)?;
                text(scope)?;
            }
            Self::Basin {
                cargo,
                custodian,
                urban_scope,
                rural_scope,
                contraband_uncertainty,
            } => {
                for s in [
                    cargo,
                    custodian,
                    urban_scope,
                    rural_scope,
                    contraband_uncertainty,
                ] {
                    text(s)?;
                }
                require(cargo == subject, "cargo must be the continuing subject")?;
            }
            Self::PublicName {
                former_names,
                requested_name,
                liabilities,
            } => {
                strings(former_names)?;
                text(requested_name)?;
                optional_strings(liabilities)?;
            }
            Self::Hollowing {
                sample,
                subject_custodian,
                sample_custodian,
                missing_custody_history,
                restoration_uncertainty,
            } => {
                for s in [
                    sample,
                    subject_custodian,
                    sample_custodian,
                    missing_custody_history,
                    restoration_uncertainty,
                ] {
                    text(s)?;
                }
                require(
                    sample != subject,
                    "subject identity and extracted sample must be distinct",
                )?;
            }
            Self::PublicDesign {
                design,
                interior_result,
                exterior_result,
                maintenance_uncertainty,
                ..
            } => {
                require(design == subject, "design must be the continuing subject")?;
                for s in [
                    design,
                    interior_result,
                    exterior_result,
                    maintenance_uncertainty,
                ] {
                    text(s)?;
                }
            }
            Self::RecoveryDesign {
                method,
                lab_result,
                moving_failure,
                revised_moving_result,
                ..
            } => {
                for s in [method, lab_result, moving_failure] {
                    text(s)?;
                }
                if let Some(s) = revised_moving_result {
                    text(s)?;
                }
            }
            Self::CoastalProof {
                original_body,
                reproducing_body,
                result,
                ..
            } => {
                for s in [original_body, reproducing_body, result] {
                    text(s)?;
                }
                require(
                    original_body != reproducing_body,
                    "reproduction must be independent",
                )?;
            }
            Self::EmergencyIntake {
                urgent_threat,
                stabilization_scope,
                ..
            } => {
                text(urgent_threat)?;
                text(stabilization_scope)?;
            }
            Self::HighRiskClearance {
                proven_claim,
                reversal_uncertainty,
                conditions,
                ..
            } => {
                text(proven_claim)?;
                text(reversal_uncertainty)?;
                strings(conditions)?;
            }
            Self::RecoveryDischarge {
                travel_result,
                duty_limits,
                full_load_uncertainty,
                recovery_plan,
                ..
            } => {
                for s in [travel_result, full_load_uncertainty, recovery_plan] {
                    text(s)?;
                }
                strings(duty_limits)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Evidence {
    pub revision: u32,
    pub references: Vec<String>,
    pub tested_conditions: Vec<String>,
    pub negative_results: Vec<String>,
    pub facts: Facts,
}
impl Evidence {
    fn validate(&self, route: Route, subject: &str) -> Result<(), Error> {
        require(
            self.facts.route() == route && self.revision > 0,
            "evidence route/revision mismatch",
        )?;
        strings(&self.references)?;
        strings(&self.tested_conditions)?;
        optional_strings(&self.negative_results)?;
        self.facts.validate(subject)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Function {
    Name,
    Prove,
    Clear,
    Recognize,
}
impl Function {
    pub const fn house_function(self) -> HouseFunction {
        match self {
            Self::Name => HouseFunction::Name,
            Self::Prove => HouseFunction::Prove,
            Self::Clear => HouseFunction::Clear,
            Self::Recognize => HouseFunction::Recognize,
        }
    }
    fn office(self) -> &'static str {
        match self {
            Self::Name => "office.stonebend.hypergiant",
            Self::Prove => "office.sandmanor.sandman",
            Self::Clear => "office.glaushouse.prima-donna",
            Self::Recognize => "office.flynt.tross",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorityReceipt {
    pub id: String,
    pub actor: String,
    pub office: String,
    pub function: Function,
    pub revision: u32,
    pub evidence: Vec<String>,
    pub conditions: Vec<String>,
    pub at: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Action {
    RecordEvidence {
        actor: String,
        evidence: Evidence,
    },
    Choose {
        actor: String,
        alternative: u8,
    },
    EmergencyStabilization {
        actor: String,
    },
    Decide {
        actor: String,
        function: Function,
    },
    Execute {
        actor: String,
    },
    Observe {
        actor: String,
        accepts_scope: bool,
    },
    Deliver {
        actor: String,
    },
    Stop {
        actor: String,
        reason: String,
    },
    Recover {
        actor: String,
        evidence: Vec<String>,
    },
    ObserveRecovery {
        actor: String,
    },
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Phase {
    AwaitingEvidence,
    AwaitingChoice,
    AwaitingDecision,
    Authorized,
    Executed,
    Observed,
    Stopped,
    RecoveryRecorded,
    Recovered,
    Delivered,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Consequence {
    FunctionalRecognition {
        scope: Option<String>,
        contributions: Vec<String>,
        attribution_review: bool,
    },
    DualExpression {
        cargo: String,
        custodian: String,
        urban_scope: String,
        rural_scope: String,
        selected_expression: Option<String>,
        contraband_uncertainty: String,
        sovereign_office: String,
    },
    PublicReference {
        former_names: Vec<String>,
        provisional_name: Option<String>,
        liabilities: Vec<String>,
        challenge_pending: bool,
    },
    ProtectedCustody {
        subject: String,
        subject_custodian: String,
        sample: String,
        sample_custodian: String,
        emergency_seal: bool,
        investigation_pending: bool,
        destructive_inspection_authorized: bool,
    },
    ShelterProof {
        design: String,
        proven_within_conditions: bool,
        revision_required: bool,
        claim_rejected: bool,
    },
    RecoveryMethod {
        method: String,
        revised: bool,
        laboratory_only: bool,
        testing_stopped: bool,
        failure_retained: String,
    },
    CoastalRole {
        reproduced_role_proven: bool,
        wider_reproduction_required: bool,
        predecessor: String,
        result: String,
        transition_performed: bool,
    },
    EmergencyCare {
        stabilization_recorded: bool,
        elective_intervention_authorized: bool,
        nightingale_stop: bool,
    },
    ClinicalDisposition {
        conditional_clearance: bool,
        reversible_alternative_requested: bool,
        synthesis_refused: bool,
        proof_decision: Option<String>,
        clearance_decision: String,
        realization_performed: bool,
    },
    RecoveryTransfer {
        limited_transfer: bool,
        duty_limits: Vec<String>,
        full_duty_authorized: bool,
        recovery_continues: bool,
        receiving_conditions_refused: bool,
        recovery_plan: String,
    },
    ProtectedReturn {
        subject: String,
        reason: String,
    },
}

/// Typed consequence preserves the actual domain facts. The chosen alternative
/// is the exact canonical contract text; it is not a universal pass/fail flag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Outcome {
    pub route: Route,
    pub subject: String,
    pub revision: u32,
    pub alternative: Option<String>,
    pub facts: Option<Facts>,
    pub consequence: Consequence,
    pub conditions: Vec<String>,
    pub negative_results: Vec<String>,
    pub consent_recorded: bool,
    pub protected_stop: Option<String>,
    pub remaining_obligations: Vec<String>,
    pub grants_form: bool,
    pub grants_office: bool,
    pub grants_affiliation: bool,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub phase: Phase,
    pub evidence: Option<Evidence>,
    pub alternative: Option<u8>,
    pub emergency_without_consent: bool,
    pub decisions: Vec<AuthorityReceipt>,
    pub outcome: Option<Outcome>,
    pub stop_reason: Option<String>,
    pub recovery_required: bool,
    pub delivery_recipient: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Event {
    pub id: String,
    pub at: u64,
    pub action: Action,
    pub result: State,
}

#[derive(Debug, Clone)]
pub struct HandoffService {
    route: Route,
    subject: String,
    actors: Actors,
    world: WorldSession,
    state: State,
    events: Vec<Event>,
    index: BTreeMap<String, usize>,
}
impl HandoffService {
    pub fn new(
        route: Route,
        subject: String,
        actors: Actors,
        world: WorldSession,
    ) -> Result<Self, Error> {
        text(&subject)?;
        actors.validate()?;
        if matches!(
            route,
            Route::Stairway
                | Route::PublicName
                | Route::Hollowing
                | Route::RecoveryDesign
                | Route::CoastalProof
                | Route::EmergencyIntake
                | Route::HighRiskClearance
                | Route::RecoveryDischarge
        ) {
            require(
                subject == actors.recipient,
                "living subject must receive their own disposition",
            )?;
        }
        Ok(Self {
            route,
            subject,
            actors,
            world,
            state: State {
                phase: Phase::AwaitingEvidence,
                evidence: None,
                alternative: None,
                emergency_without_consent: false,
                decisions: vec![],
                outcome: None,
                stop_reason: None,
                recovery_required: false,
                delivery_recipient: None,
            },
            events: vec![],
            index: BTreeMap::new(),
        })
    }
    pub fn route(&self) -> Route {
        self.route
    }
    pub fn state(&self) -> &State {
        &self.state
    }
    pub fn events(&self) -> &[Event] {
        &self.events
    }
    pub fn complete(&self) -> bool {
        self.state.phase == Phase::Delivered
    }
    pub fn execute(&mut self, id: String, at: u64, action: Action) -> Result<&Event, Error> {
        text(&id)?;
        if let Some(&index) = self.index.get(&id) {
            let event = &self.events[index];
            require(
                event.at == at && event.action == action,
                "conflicting retry",
            )?;
            return Ok(event);
        }
        require(
            at > self.events.last().map_or(0, |e| e.at),
            "causal position must increase",
        )?;
        let mut candidate = self.state.clone(); // bounded current state; never clone the journal
        self.apply(&mut candidate, at, &id, &action)?;
        let index = self.events.len();
        self.events.push(Event {
            id: id.clone(),
            at,
            action,
            result: candidate.clone(),
        });
        self.index.insert(id, index);
        self.state = candidate;
        Ok(&self.events[index])
    }
    fn apply(&self, s: &mut State, at: u64, id: &str, action: &Action) -> Result<(), Error> {
        require(s.phase != Phase::Delivered, "delivered scope is closed")?;
        match action {
            Action::RecordEvidence { actor, evidence } => {
                require(
                    actor == &self.actors.witness,
                    "only the independent recorder records evidence",
                )?;
                require(
                    !matches!(
                        s.phase,
                        Phase::Executed
                            | Phase::Observed
                            | Phase::Stopped
                            | Phase::RecoveryRecorded
                            | Phase::Recovered
                    ),
                    "recover or close performed work before changing scope",
                )?;
                evidence.validate(self.route, &self.subject)?;
                require(
                    evidence.revision == s.evidence.as_ref().map_or(1, |e| e.revision + 1),
                    "revision must advance exactly once",
                )?;
                require(evidence.revision <= 32, "bounded case revision limit")?;
                if let Some(old) = &s.evidence {
                    require(
                        old.negative_results
                            .iter()
                            .all(|n| evidence.negative_results.contains(n)),
                        "negative history cannot be removed",
                    )?;
                    if let (
                        Facts::PublicName {
                            former_names,
                            liabilities,
                            ..
                        },
                        Facts::PublicName {
                            former_names: new_names,
                            liabilities: new_debts,
                            ..
                        },
                    ) = (&old.facts, &evidence.facts)
                    {
                        require(
                            former_names.iter().all(|n| new_names.contains(n))
                                && liabilities.iter().all(|n| new_debts.contains(n)),
                            "Name revision cannot erase history or liability",
                        )?;
                    }
                }
                s.evidence = Some(evidence.clone());
                s.alternative = None;
                s.decisions.clear();
                s.emergency_without_consent = false;
                s.outcome = None;
                s.phase = Phase::AwaitingChoice;
            }
            Action::Choose { actor, alternative } => {
                require(
                    actor == &self.actors.recipient && s.phase == Phase::AwaitingChoice,
                    "independent recipient choice required",
                )?;
                require(*alternative < 3, "unknown canonical alternative")?;
                if *alternative == 0 {
                    require(
                        s.evidence
                            .as_ref()
                            .unwrap()
                            .facts
                            .capacity()
                            .is_none_or(|c| c == Capacity::Capable),
                        "unknown capacity is not elective consent",
                    )?;
                }
                s.alternative = Some(*alternative);
                s.phase = Phase::AwaitingDecision;
            }
            Action::EmergencyStabilization { actor } => {
                require(
                    actor == &self.actors.executor
                        && self.route == Route::EmergencyIntake
                        && s.phase == Phase::AwaitingChoice,
                    "emergency operator and intake evidence required",
                )?;
                require(
                    s.evidence.as_ref().unwrap().facts.capacity() != Some(Capacity::Capable),
                    "capable patient makes their own choice",
                )?;
                require(
                    matches!(
                        s.evidence.as_ref().unwrap().facts,
                        Facts::EmergencyIntake {
                            immediate_stabilization_required: true,
                            ..
                        }
                    ),
                    "no immediate emergency need has been recorded",
                )?;
                s.alternative = Some(0);
                s.emergency_without_consent = true;
                s.phase = Phase::AwaitingDecision;
            }
            Action::Decide { actor, function } => {
                require(
                    s.phase == Phase::AwaitingDecision,
                    "choice and current evidence required",
                )?;
                let principal = self.route.function();
                let secondary = match self.route {
                    Route::RecoveryDesign if s.alternative != Some(2) => Some(HouseFunction::Clear),
                    Route::HighRiskClearance if s.alternative == Some(0) => {
                        Some(HouseFunction::Prove)
                    }
                    _ => None,
                };
                let f = function.house_function();
                require(
                    f == principal || Some(f) == secondary,
                    "mismatched House function",
                )?;
                require(
                    !s.decisions.iter().any(|d| d.function == *function),
                    "decision already recorded for revision",
                )?;
                if f == principal {
                    if let Some(required) = secondary {
                        require(
                            s.decisions
                                .iter()
                                .any(|d| d.function.house_function() == required),
                            "independent prerequisite decision missing",
                        )?;
                    }
                    self.check_positive_claim(s)?;
                }
                let evidence = s.evidence.as_ref().unwrap();
                let office = OfficeId::new(function.office())
                    .map_err(|_| Error::Invalid("invalid office"))?;
                let decision = HouseDecision::from_catalog(
                    &self.world.institutional().catalog,
                    HouseDecisionDraft {
                        id: HouseDecisionId::new(format!("decision.handoff.{id}"))
                            .map_err(|_| Error::Invalid("invalid decision ID"))?,
                        function: f,
                        office,
                        actor: AuthorityActorId::new(actor.clone())
                            .map_err(|_| Error::Invalid("invalid authority actor"))?,
                        outcome: HouseDecisionOutcome::Accepted,
                        evidence: evidence
                            .references
                            .iter()
                            .map(|r| {
                                EvidenceRef(
                                    ExternalRef::new("backend-handoff", r.clone())
                                        .expect("validated evidence reference"),
                                )
                            })
                            .collect(),
                        causal_position: CausalPosition::new(at),
                    },
                )
                .map_err(|_| Error::Invalid("live House authority unavailable or mismatched"))?;
                decision
                    .require_accepted(f)
                    .map_err(|_| Error::Invalid("unaccepted decision"))?;
                require(
                    ![
                        &self.actors.executor,
                        &self.actors.witness,
                        &self.actors.recipient,
                    ]
                    .contains(&actor),
                    "decision owner must be independent of ordinary work",
                )?;
                s.decisions.push(AuthorityReceipt {
                    id: decision.id.as_str().into(),
                    actor: actor.clone(),
                    office: function.office().into(),
                    function: *function,
                    revision: evidence.revision,
                    evidence: evidence.references.clone(),
                    conditions: evidence.tested_conditions.clone(),
                    at,
                });
                if f == principal {
                    s.phase = Phase::Authorized;
                }
            }
            Action::Execute { actor } => {
                require(
                    actor == &self.actors.executor && s.phase == Phase::Authorized,
                    "scoped authorization and bound executor required",
                )?;
                s.outcome = Some(self.outcome(s));
                s.phase = Phase::Executed;
            }
            Action::Observe {
                actor,
                accepts_scope,
            } => {
                require(
                    actor == &self.actors.witness && s.phase == Phase::Executed && *accepts_scope,
                    "independent observation must accept the exact performed scope",
                )?;
                s.phase = Phase::Observed;
            }
            Action::Deliver { actor } => {
                require(
                    actor == &self.actors.recipient
                        && (s.phase == Phase::Observed || s.phase == Phase::Recovered),
                    "actual recipient receipt and witnessed work or recovered stop required",
                )?;
                require(!s.recovery_required, "recovery obligations remain open")?;
                if s.outcome.is_none() {
                    s.outcome = Some(self.outcome(s));
                }
                s.delivery_recipient = Some(actor.clone());
                s.phase = Phase::Delivered;
            }
            Action::Stop { actor, reason } => {
                require(
                    actor == &self.actors.recipient
                        || (self.route == Route::EmergencyIntake && actor == &self.actors.witness),
                    "subject or Nightingale stop owner required",
                )?;
                require(
                    !matches!(
                        s.phase,
                        Phase::Stopped | Phase::RecoveryRecorded | Phase::Recovered
                    ),
                    "already stopped",
                )?;
                text(reason)?;
                s.recovery_required = matches!(s.phase, Phase::Executed | Phase::Observed);
                s.stop_reason = Some(reason.clone());
                s.phase = Phase::Stopped;
                s.outcome = Some(self.outcome(s));
            }
            Action::Recover { actor, evidence } => {
                require(
                    actor == &self.actors.recovery_owner && s.phase == Phase::Stopped,
                    "bound recovery owner and stopped scope required",
                )?;
                strings(evidence)?;
                s.phase = Phase::RecoveryRecorded;
            }
            Action::ObserveRecovery { actor } => {
                require(
                    actor == &self.actors.witness && s.phase == Phase::RecoveryRecorded,
                    "independent recovery observation required",
                )?;
                s.recovery_required = false;
                s.phase = Phase::Recovered;
                s.outcome = Some(self.outcome(s));
            }
        }
        Ok(())
    }
    fn check_positive_claim(&self, s: &State) -> Result<(), Error> {
        if s.alternative != Some(0) {
            return Ok(());
        }
        let e = s.evidence.as_ref().unwrap();
        match &e.facts {
            Facts::PublicDesign { passed, .. } => require(
                *passed,
                "failed shelter result cannot become positive proof",
            ),
            Facts::RecoveryDesign {
                revised_moving_result,
                revised_moving_passed,
                ..
            } => require(
                e.revision >= 2 && revised_moving_result.is_some() && *revised_moving_passed,
                "failed moving method requires an actual revised result",
            ),
            Facts::CoastalProof {
                reproduced,
                safer_within_conditions,
                ..
            } => require(
                *reproduced && *safer_within_conditions,
                "regional proof requires independent reproduction",
            ),
            Facts::HighRiskClearance {
                proof_supported, ..
            } => require(
                *proof_supported,
                "negative or unproven design cannot support positive clinical clearance",
            ),
            Facts::RecoveryDischarge {
                travel_proven_within_limits,
                ..
            } => require(
                *travel_proven_within_limits,
                "unproven travel cannot support positive restricted transfer",
            ),
            _ => Ok(()),
        }
    }
    fn consequence(&self, s: &State) -> Consequence {
        if let Some(reason) = &s.stop_reason {
            return Consequence::ProtectedReturn {
                subject: self.subject.clone(),
                reason: reason.clone(),
            };
        }
        let e = s
            .evidence
            .as_ref()
            .expect("authorized consequence has evidence");
        let choice = s.alternative.expect("authorized consequence has a choice");
        match &e.facts {
            Facts::Stairway {
                work: _,
                contributions,
                scope,
            } => Consequence::FunctionalRecognition {
                scope: (choice == 0).then(|| scope.clone()),
                contributions: contributions.clone(),
                attribution_review: choice == 2,
            },
            Facts::Basin {
                cargo,
                custodian,
                urban_scope,
                rural_scope,
                contraband_uncertainty,
            } => Consequence::DualExpression {
                cargo: cargo.clone(),
                custodian: custodian.clone(),
                urban_scope: urban_scope.clone(),
                rural_scope: rural_scope.clone(),
                selected_expression: match choice {
                    0 => Some(urban_scope.clone()),
                    1 => Some(rural_scope.clone()),
                    _ => None,
                },
                contraband_uncertainty: contraband_uncertainty.clone(),
                sovereign_office: "office.flynt.tross".into(),
            },
            Facts::PublicName {
                former_names,
                requested_name,
                liabilities,
            } => Consequence::PublicReference {
                former_names: former_names.clone(),
                provisional_name: (choice == 0).then(|| requested_name.clone()),
                liabilities: liabilities.clone(),
                challenge_pending: choice == 2,
            },
            Facts::Hollowing {
                sample,
                subject_custodian,
                sample_custodian,
                ..
            } => Consequence::ProtectedCustody {
                subject: self.subject.clone(),
                subject_custodian: subject_custodian.clone(),
                sample: sample.clone(),
                sample_custodian: sample_custodian.clone(),
                emergency_seal: choice == 0,
                investigation_pending: true,
                destructive_inspection_authorized: false,
            },
            Facts::PublicDesign { design, .. } => Consequence::ShelterProof {
                design: design.clone(),
                proven_within_conditions: choice == 0,
                revision_required: choice == 1,
                claim_rejected: choice == 2,
            },
            Facts::RecoveryDesign {
                method,
                moving_failure,
                ..
            } => Consequence::RecoveryMethod {
                method: method.clone(),
                revised: choice == 0,
                laboratory_only: choice == 1,
                testing_stopped: choice == 2,
                failure_retained: moving_failure.clone(),
            },
            Facts::CoastalProof { .. } => Consequence::CoastalRole {
                reproduced_role_proven: choice == 0,
                wider_reproduction_required: choice == 1,
                predecessor: "Satyr".into(),
                result: "Centaur".into(),
                transition_performed: false,
            },
            Facts::EmergencyIntake { .. } => Consequence::EmergencyCare {
                stabilization_recorded: choice == 0,
                elective_intervention_authorized: false,
                nightingale_stop: choice == 2,
            },
            Facts::HighRiskClearance { .. } => Consequence::ClinicalDisposition {
                conditional_clearance: choice == 0,
                reversible_alternative_requested: choice == 1,
                synthesis_refused: choice == 2,
                proof_decision: s
                    .decisions
                    .iter()
                    .find(|d| d.function == Function::Prove)
                    .map(|d| d.id.clone()),
                clearance_decision: s
                    .decisions
                    .iter()
                    .find(|d| d.function == Function::Clear)
                    .expect("principal decision")
                    .id
                    .clone(),
                realization_performed: false,
            },
            Facts::RecoveryDischarge {
                duty_limits,
                recovery_plan,
                ..
            } => Consequence::RecoveryTransfer {
                limited_transfer: choice == 0,
                duty_limits: duty_limits.clone(),
                full_duty_authorized: false,
                recovery_continues: true,
                receiving_conditions_refused: choice == 2,
                recovery_plan: recovery_plan.clone(),
            },
        }
    }
    fn obligations(&self, s: &State) -> Vec<String> {
        let owner = &self.actors.recovery_owner;
        let mut obligations = vec![format!(
            "{owner} owns continuing recovery within the disclosed scope"
        )];
        if s.recovery_required {
            obligations.push(format!(
                "{owner} must complete protected return before delivery"
            ));
        }
        if s.stop_reason.is_some() {
            return obligations;
        }
        let pending = match (self.route, s.alternative.unwrap()) {
            (Route::Stairway, 2) => Some("attribution review"),
            (Route::Basin, _) => Some("separate cargo custody and contraband review"),
            (Route::PublicName, 0) => {
                Some("provisional-reference review while retaining former history and liabilities")
            }
            (Route::PublicName, 2) => Some("conflicting Name claim review"),
            (Route::Hollowing, _) => Some("missing-custody investigation and restoration review"),
            (Route::PublicDesign, 1) => Some("design revision and fresh scoped proof"),
            (Route::PublicDesign, _) => {
                Some("untested seasonal performance and maintenance review")
            }
            (Route::RecoveryDesign, _) => Some(
                "clinical recovery and method review without deleting the moving-route failure",
            ),
            (Route::CoastalProof, 1) => Some("wider independent reproduction"),
            (Route::CoastalProof, _) => {
                Some("untested weather review; any biological transition remains separate")
            }
            (Route::EmergencyIntake, _) => {
                Some("continuing clinical review without inferred elective consent")
            }
            (Route::HighRiskClearance, _) => {
                Some("separate clinical procedure, alternatives and recovery review")
            }
            (Route::RecoveryDischarge, _) => {
                Some("restricted duty, follow-up and supported return")
            }
            _ => None,
        };
        if let Some(pending) = pending {
            obligations.push(format!("{owner} remains answerable for {pending}; reserved House decisions retain their authority"));
        }
        obligations
    }
    fn outcome(&self, s: &State) -> Outcome {
        let e = s.evidence.as_ref();
        let stopped = s.stop_reason.is_some();
        Outcome {
            route: self.route,
            subject: self.subject.clone(),
            revision: e.map_or(0, |e| e.revision),
            alternative: s
                .alternative
                .map(|i| self.route.definition().player_choices[i as usize].into()),
            facts: e.map(|e| e.facts.clone()),
            consequence: self.consequence(s),
            conditions: e.map_or_else(Vec::new, |e| e.tested_conditions.clone()),
            negative_results: e.map_or_else(Vec::new, |e| e.negative_results.clone()),
            consent_recorded: !stopped
                && !s.emergency_without_consent
                && s.alternative == Some(0)
                && e.is_some_and(|e| e.facts.capacity() == Some(Capacity::Capable)),
            protected_stop: s.stop_reason.clone(),
            remaining_obligations: self.obligations(s),
            grants_form: false,
            grants_office: false,
            grants_affiliation: false,
        }
    }
    pub fn encode(&self) -> Result<String, Error> {
        let payload = Payload {
            route: self.route,
            subject: self.subject.clone(),
            actors: self.actors.clone(),
            institutional_state: self.world.persisted_state_output(),
            events: self.events.clone(),
        };
        let checksum = checksum(&payload)?;
        serde_json::to_string_pretty(&Archive {
            format: "hollow-grove-backend-handoff".into(),
            schema_version: 1,
            checksum,
            payload,
        })
        .map_err(|e| Error::Archive(e.to_string()))
    }
    pub fn decode(contents: &str) -> Result<Self, Error> {
        let archive: Archive =
            serde_json::from_str(contents).map_err(|e| Error::Archive(e.to_string()))?;
        require(
            archive.format == "hollow-grove-backend-handoff" && archive.schema_version == 1,
            "unsupported handoff archive; game/catalog archives are not handoff histories",
        )?;
        require(
            archive.checksum == checksum(&archive.payload)?,
            "handoff checksum mismatch",
        )?;
        let p = archive.payload;
        let world = WorldSession::from_persisted_output(&p.institutional_state)
            .map_err(|e| Error::Archive(e.to_string()))?;
        let mut service = Self::new(p.route, p.subject, p.actors, world)?;
        for expected in p.events {
            require(
                !service.index.contains_key(&expected.id),
                "duplicate event in archive",
            )?;
            let actual =
                service.execute(expected.id.clone(), expected.at, expected.action.clone())?;
            require(
                actual == &expected,
                "replayed handoff differs from recorded consequence",
            )?;
        }
        Ok(service)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Payload {
    route: Route,
    subject: String,
    actors: Actors,
    institutional_state: String,
    events: Vec<Event>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Archive {
    format: String,
    schema_version: u16,
    checksum: String,
    payload: Payload,
}
fn checksum(value: &impl Serialize) -> Result<String, Error> {
    let canonical = serde_json::to_value(value).map_err(|e| Error::Archive(e.to_string()))?;
    let bytes = serde_json::to_vec(&canonical).map_err(|e| Error::Archive(e.to_string()))?;
    let mut hash = 0xcbf29ce484222325u64;
    for b in bytes {
        hash ^= u64::from(b);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    Ok(format!("fnv1a64:{hash:016x}")) // corruption check; not a signature
}
fn require(condition: bool, message: &'static str) -> Result<(), Error> {
    if condition {
        Ok(())
    } else {
        Err(Error::Invalid(message))
    }
}
fn text(s: &str) -> Result<(), Error> {
    require(
        !s.trim().is_empty() && s.len() <= 4096,
        "missing or oversized text",
    )
}
fn strings(values: &[String]) -> Result<(), Error> {
    require(
        !values.is_empty() && values.len() <= 64,
        "missing or excessive evidence/conditions",
    )?;
    for s in values {
        text(s)?;
    }
    Ok(())
}
fn optional_strings(values: &[String]) -> Result<(), Error> {
    require(values.len() <= 64, "excessive history")?;
    for s in values {
        text(s)?;
    }
    Ok(())
}
#[derive(Debug)]
pub enum Error {
    Invalid(&'static str),
    Archive(String),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(s) => f.write_str(s),
            Self::Archive(s) => f.write_str(s),
        }
    }
}
impl std::error::Error for Error {}
