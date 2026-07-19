use std::collections::{BTreeMap, BTreeSet};

use crate::hollow_grove_contract::House;
use crate::institution::InstitutionId;

use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParticipantKind {
    Huemen,
    Npc,
    House,
    Institution,
    Material,
    Recipe,
    Object,
    Transformation,
    Process,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BondParticipant {
    pub id: ParticipantId,
    pub kind: ParticipantKind,
    pub roles: Vec<RoleId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InitialCurrent {
    pub owner: ParticipantId,
    pub custodian: ParticipantId,
    pub quantity: SignedQuantity,
    pub evidence: Vec<EvidenceRef>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BondFormation {
    pub id: BondId,
    pub initiating_wave: WaveId,
    pub governing_house: House,
    pub governing_institution: InstitutionId,
    pub jurisdiction: InstitutionalJurisdictionSnapshot,
    pub parent_bonds: Vec<BondId>,
    pub inheritance_evidence: Vec<EvidenceRef>,
    pub participants: Vec<BondParticipant>,
    pub obligations: Vec<ObligationId>,
    pub permissions: Vec<PermissionId>,
    pub term: BondTerm,
    pub current_unit: UnitId,
    pub aura_unit: UnitId,
    pub starting_current: Vec<InitialCurrent>,
    pub initial_aura: Vec<SignedQuantity>,
    pub evidence: Vec<EvidenceRef>,
    pub stonebend_naming: HouseDecision,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BondValidation {
    pub sandmanor_proof: HouseDecision,
    pub evidence: Vec<EvidenceRef>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BondActivation {
    pub evidence: Vec<EvidenceRef>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrentTransaction {
    pub id: CurrentTransactionId,
    pub wave: WaveId,
    pub operation: CurrentOperation,
    pub edges: Vec<CurrentEdge>,
    pub evidence: Vec<EvidenceRef>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrentAccounting {
    pub historical: SignedTotals,
    pub incoming: SignedTotals,
    pub outgoing: SignedTotals,
    pub transferred: SignedTotals,
    pub retained: SignedTotals,
    pub unresolved: SignedTotals,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrentAccumulation {
    pub accounting: CurrentAccounting,
    pub through_transaction: Option<CurrentTransactionId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuraObservation {
    pub id: AuraObservationId,
    pub observer: ParticipantId,
    pub quantity: SignedQuantity,
    pub subject: EvidenceRef,
    pub evidence: Vec<EvidenceRef>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstitutionalPolarity {
    PositiveCurrentPositiveAura,
    PositiveCurrentNegativeAura,
    NegativeCurrentPositiveAura,
    NegativeCurrentNegativeAura,
}

impl ConstitutionalPolarity {
    #[must_use]
    pub const fn current_sign(self) -> Sign {
        match self {
            Self::PositiveCurrentPositiveAura | Self::PositiveCurrentNegativeAura => Sign::Positive,
            Self::NegativeCurrentPositiveAura | Self::NegativeCurrentNegativeAura => Sign::Negative,
        }
    }

    #[must_use]
    pub const fn aura_sign(self) -> Sign {
        match self {
            Self::PositiveCurrentPositiveAura | Self::NegativeCurrentPositiveAura => Sign::Positive,
            Self::PositiveCurrentNegativeAura | Self::NegativeCurrentNegativeAura => Sign::Negative,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrentAuraEvaluation {
    pub id: EvaluationId,
    pub current: SignedTotals,
    pub aura: SignedTotals,
    pub polarity: ConstitutionalPolarity,
    pub evidence: Vec<EvidenceRef>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaturityTrigger {
    FiniteTermCompleted,
    PerpetualTermTerminated,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BondMaturity {
    pub trigger: MaturityTrigger,
    pub evidence: Vec<EvidenceRef>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BondChallenge {
    pub id: ChallengeId,
    pub challenger: ParticipantId,
    pub challenged_evidence: EvidenceRef,
    pub evidence: Vec<EvidenceRef>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChallengeOutcome {
    Sustained,
    Rejected,
    Clarified,
    Corrected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BondChallengeResolution {
    pub challenge: ChallengeId,
    pub outcome: ChallengeOutcome,
    pub sandmanor_proof: HouseDecision,
    pub evidence: Vec<EvidenceRef>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BondDefault {
    pub id: DefaultId,
    pub participant: ParticipantId,
    pub obligation: ObligationId,
    pub evidence: Vec<EvidenceRef>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefaultOutcome {
    Cured,
    Confirmed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BondDefaultResolution {
    pub default: DefaultId,
    pub outcome: DefaultOutcome,
    pub evidence: Vec<EvidenceRef>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IneligibilityReason {
    NoNetExcess,
    ClearanceRejected,
    ClearanceInconclusive,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CondensationStatus {
    Eligible,
    Ineligible(IneligibilityReason),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CondensationDecision {
    pub status: CondensationStatus,
    pub glaushouse_clearance: Option<HouseDecision>,
    pub evidence: Vec<EvidenceRef>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tombstone {
    pub id: TombstoneId,
    pub source_bond: BondId,
    pub governing_house: House,
    pub governing_institution: InstitutionId,
    pub participants: Vec<BondParticipant>,
    pub constitutional_excess: NetExcess,
    pub polarity: ConstitutionalPolarity,
    pub completed_obligations: Vec<ObligationId>,
    pub remaining_obligations: Vec<ObligationId>,
    pub evidence: Vec<EvidenceRef>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TombstoneValidation {
    pub validator: AuthorityActorId,
    pub validation_basis: EvidenceRef,
    pub replay_digest: String,
    pub evidence: Vec<EvidenceRef>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TombstoneOmission {
    pub reason: IneligibilityReason,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Toke {
    pub id: TokeId,
    pub tombstone: TombstoneId,
    pub index_key: String,
    pub evidence: Vec<EvidenceRef>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolutionDisposition {
    Complete,
    Renew,
    Merge,
    Branch,
    Split,
    Transfer,
    Dissolve,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BondResolution {
    pub id: ResolutionId,
    pub disposition: ResolutionDisposition,
    pub successor_bonds: Vec<BondId>,
    pub glaushouse_resolution: HouseDecision,
    pub evidence: Vec<EvidenceRef>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BondEvent {
    Formed(BondFormation),
    Validated(BondValidation),
    Activated(BondActivation),
    CurrentMoved(CurrentTransaction),
    CurrentAccumulated(CurrentAccumulation),
    AuraObserved(AuraObservation),
    Evaluated(CurrentAuraEvaluation),
    Matured(BondMaturity),
    ExcessCalculated(NetExcess),
    CondensationDecided(CondensationDecision),
    TombstoneFormed(Tombstone),
    TombstoneOmitted(TombstoneOmission),
    TombstoneValidated(TombstoneValidation),
    TombstoneValidationOmitted(TombstoneOmission),
    FlyntRecognized(HouseDecision),
    TokeRecorded(Toke),
    TokeOmitted(TombstoneOmission),
    ChallengeFiled(BondChallenge),
    ChallengeResolved(BondChallengeResolution),
    DefaultDeclared(BondDefault),
    DefaultResolved(BondDefaultResolution),
    Resolved(BondResolution),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstitutionalEvent {
    pub id: ConstitutionalEventId,
    pub bond: BondId,
    pub sequence: u64,
    pub causal_position: CausalPosition,
    pub rule_set: RuleSetId,
    pub payload: BondEvent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BondPhase {
    Formed,
    Validated,
    Active,
    Mature,
    ExcessCalculated,
    EligibilityDecided,
    TombstoneFormed,
    TombstoneValidated,
    Recorded,
    Resolved,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BondAggregate {
    formation: BondFormation,
    phase: BondPhase,
    rule_set: RuleSetId,
    last_sequence: u64,
    last_causal_position: CausalPosition,
    validation: Option<BondValidation>,
    holdings: Holdings,
    historical: SignedTotals,
    incoming: SignedTotals,
    outgoing: SignedTotals,
    transferred: SignedTotals,
    aura: SignedTotals,
    transaction_ids: BTreeSet<CurrentTransactionId>,
    observation_ids: BTreeSet<AuraObservationId>,
    house_decision_ids: BTreeSet<HouseDecisionId>,
    challenge_ids: BTreeSet<ChallengeId>,
    pending_challenges: BTreeSet<ChallengeId>,
    default_ids: BTreeSet<DefaultId>,
    pending_defaults: BTreeMap<DefaultId, ObligationId>,
    confirmed_default_obligations: BTreeSet<ObligationId>,
    last_transaction: Option<CurrentTransactionId>,
    accumulation: Option<CurrentAccumulation>,
    evaluation: Option<CurrentAuraEvaluation>,
    excess: Option<NetExcess>,
    condensation: Option<CondensationDecision>,
    tombstone: Option<Tombstone>,
    tombstone_validated: bool,
    flynt_recognition: Option<HouseDecision>,
    toke: Option<Toke>,
    omission_stage: u8,
    resolution: Option<BondResolution>,
}

impl BondAggregate {
    pub fn replay<'a>(
        events: impl IntoIterator<Item = &'a ConstitutionalEvent>,
    ) -> Result<Self, BondStateError> {
        let mut aggregate: Option<Self> = None;
        for event in events {
            match aggregate.as_mut() {
                Some(existing) => existing.apply(event)?,
                None => {
                    let BondEvent::Formed(formation) = &event.payload else {
                        return Err(BondStateError::FormationRequired);
                    };
                    let mut created = Self::from_formation(formation.clone(), event)?;
                    created.last_sequence = event.sequence;
                    created.last_causal_position = event.causal_position;
                    aggregate = Some(created);
                }
            }
        }
        aggregate.ok_or(BondStateError::FormationRequired)
    }

    fn from_formation(
        formation: BondFormation,
        event: &ConstitutionalEvent,
    ) -> Result<Self, BondStateError> {
        if event.sequence != 0 || event.bond != formation.id {
            return Err(BondStateError::InvalidFormationEnvelope);
        }
        validate_formation(&formation, event.causal_position)?;
        let mut holdings = BTreeMap::new();
        let mut historical = SignedTotals::zero(formation.current_unit.clone());
        let mut incoming = SignedTotals::zero(formation.current_unit.clone());
        for initial in &formation.starting_current {
            historical.add(&initial.quantity)?;
            incoming.add(&initial.quantity)?;
            credit_holding(&mut holdings, &initial.custodian, &initial.quantity)?;
        }
        let mut aura = SignedTotals::zero(formation.aura_unit.clone());
        for initial in &formation.initial_aura {
            aura.add(initial)?;
        }
        let house_decision_ids = BTreeSet::from([formation.stonebend_naming.id.clone()]);
        Ok(Self {
            formation,
            phase: BondPhase::Formed,
            rule_set: event.rule_set.clone(),
            last_sequence: 0,
            last_causal_position: event.causal_position,
            validation: None,
            holdings,
            historical,
            incoming,
            outgoing: SignedTotals::zero(event_current_unit(event)?),
            transferred: SignedTotals::zero(event_current_unit(event)?),
            aura,
            transaction_ids: BTreeSet::new(),
            observation_ids: BTreeSet::new(),
            house_decision_ids,
            challenge_ids: BTreeSet::new(),
            pending_challenges: BTreeSet::new(),
            default_ids: BTreeSet::new(),
            pending_defaults: BTreeMap::new(),
            confirmed_default_obligations: BTreeSet::new(),
            last_transaction: None,
            accumulation: None,
            evaluation: None,
            excess: None,
            condensation: None,
            tombstone: None,
            tombstone_validated: false,
            flynt_recognition: None,
            toke: None,
            omission_stage: 0,
            resolution: None,
        })
    }

    pub(crate) fn apply(&mut self, event: &ConstitutionalEvent) -> Result<(), BondStateError> {
        if event.bond != self.formation.id {
            return Err(BondStateError::WrongBond {
                expected: self.formation.id.clone(),
                actual: event.bond.clone(),
            });
        }
        let expected_sequence = self
            .last_sequence
            .checked_add(1)
            .ok_or(BondStateError::SequenceOverflow)?;
        if event.sequence != expected_sequence {
            return Err(BondStateError::SequenceMismatch {
                expected: expected_sequence,
                actual: event.sequence,
            });
        }
        if event.causal_position < self.last_causal_position {
            return Err(BondStateError::CausalRegression {
                previous: self.last_causal_position,
                actual: event.causal_position,
            });
        }
        if event.rule_set != self.rule_set {
            return Err(BondStateError::RuleSetMismatch {
                expected: self.rule_set.clone(),
                actual: event.rule_set.clone(),
            });
        }

        match &event.payload {
            BondEvent::Formed(_) => return Err(BondStateError::DuplicateFormation),
            BondEvent::Validated(validation) => self.apply_validation(validation, event)?,
            BondEvent::Activated(activation) => self.apply_activation(activation, event)?,
            BondEvent::CurrentMoved(transaction) => {
                self.apply_transaction(transaction, event.causal_position)?
            }
            BondEvent::CurrentAccumulated(accumulation) => self.apply_accumulation(accumulation)?,
            BondEvent::AuraObserved(observation) => {
                self.apply_aura(observation, event.causal_position)?
            }
            BondEvent::Evaluated(evaluation) => self.apply_evaluation(evaluation)?,
            BondEvent::Matured(maturity) => self.apply_maturity(maturity, event.causal_position)?,
            BondEvent::ExcessCalculated(excess) => self.apply_excess(excess)?,
            BondEvent::CondensationDecided(decision) => {
                self.apply_condensation(decision, event.causal_position)?
            }
            BondEvent::TombstoneFormed(tombstone) => self.apply_tombstone(tombstone)?,
            BondEvent::TombstoneOmitted(omission) => self.apply_omission(omission, 1)?,
            BondEvent::TombstoneValidated(validation) => {
                self.apply_tombstone_validation(validation)?
            }
            BondEvent::TombstoneValidationOmitted(omission) => self.apply_omission(omission, 2)?,
            BondEvent::FlyntRecognized(recognition) => {
                self.apply_recognition(recognition, event.causal_position)?
            }
            BondEvent::TokeRecorded(toke) => self.apply_toke(toke)?,
            BondEvent::TokeOmitted(omission) => self.apply_omission(omission, 3)?,
            BondEvent::ChallengeFiled(challenge) => self.apply_challenge(challenge)?,
            BondEvent::ChallengeResolved(resolution) => {
                self.apply_challenge_resolution(resolution, event.causal_position)?
            }
            BondEvent::DefaultDeclared(default) => self.apply_default(default)?,
            BondEvent::DefaultResolved(resolution) => self.apply_default_resolution(resolution)?,
            BondEvent::Resolved(resolution) => {
                self.apply_resolution(resolution, event.causal_position)?
            }
        }

        self.last_sequence = event.sequence;
        self.last_causal_position = event.causal_position;
        Ok(())
    }

    fn require_phase(&self, expected: BondPhase) -> Result<(), BondStateError> {
        if self.phase != expected {
            return Err(BondStateError::WrongPhase {
                expected,
                actual: self.phase,
            });
        }
        Ok(())
    }

    fn apply_validation(
        &mut self,
        validation: &BondValidation,
        event: &ConstitutionalEvent,
    ) -> Result<(), BondStateError> {
        self.require_phase(BondPhase::Formed)?;
        validation
            .sandmanor_proof
            .require_accepted(HouseFunction::Prove)?;
        self.register_house_decision(&validation.sandmanor_proof)?;
        if validation.evidence.is_empty() {
            return Err(BondStateError::MissingEvidence("Bond Validation"));
        }
        if validation.sandmanor_proof.causal_position > event.causal_position {
            return Err(BondStateError::DecisionFromFuture);
        }
        self.validation = Some(validation.clone());
        self.phase = BondPhase::Validated;
        Ok(())
    }

    fn apply_activation(
        &mut self,
        activation: &BondActivation,
        event: &ConstitutionalEvent,
    ) -> Result<(), BondStateError> {
        self.require_phase(BondPhase::Validated)?;
        if activation.evidence.is_empty() {
            return Err(BondStateError::MissingEvidence("Bond Activation"));
        }
        if let BondTerm::Finite { end } = self.formation.term
            && event.causal_position >= end
        {
            return Err(BondStateError::TermAlreadyCompleted);
        }
        self.phase = BondPhase::Active;
        Ok(())
    }

    fn apply_transaction(
        &mut self,
        transaction: &CurrentTransaction,
        at: CausalPosition,
    ) -> Result<(), BondStateError> {
        self.require_phase(BondPhase::Active)?;
        if let BondTerm::Finite { end } = self.formation.term
            && at >= end
        {
            return Err(BondStateError::CurrentOutsideActiveTerm);
        }
        if transaction.edges.is_empty() {
            return Err(BondStateError::EmptyCurrentTransaction);
        }
        if transaction.evidence.is_empty() {
            return Err(BondStateError::MissingEvidence("Current Circulation"));
        }
        if !self.transaction_ids.insert(transaction.id.clone()) {
            return Err(BondStateError::DuplicateCurrentTransaction(
                transaction.id.clone(),
            ));
        }
        validate_operation_shape(transaction)?;

        let participants: BTreeSet<_> = self
            .formation
            .participants
            .iter()
            .map(|participant| participant.id.clone())
            .collect();
        let mut holdings = self.holdings.clone();
        let mut historical = self.historical.clone();
        let mut incoming = self.incoming.clone();
        let mut outgoing = self.outgoing.clone();
        let mut transferred = self.transferred.clone();

        for edge in &transaction.edges {
            validate_account(&edge.source, &participants, true)?;
            validate_account(&edge.destination, &participants, false)?;
            if edge.source == edge.destination {
                return Err(BondStateError::SelfCurrentEdge);
            }
            if edge.quantity.unit != self.formation.current_unit {
                return Err(QuantityError::UnitMismatch {
                    expected: self.formation.current_unit.clone(),
                    actual: edge.quantity.unit.clone(),
                }
                .into());
            }
            historical.add(&edge.quantity)?;
            if let CurrentAccount::Participant(source) = &edge.source {
                debit_holding(&mut holdings, source, &edge.quantity)?;
            }
            if let CurrentAccount::Participant(destination) = &edge.destination {
                credit_holding(&mut holdings, destination, &edge.quantity)?;
            }
            match (&edge.source, &edge.destination) {
                (CurrentAccount::External(_), CurrentAccount::Participant(_)) => {
                    incoming.add(&edge.quantity)?;
                }
                (
                    CurrentAccount::Participant(_),
                    CurrentAccount::External(_) | CurrentAccount::Sink(_),
                ) => {
                    outgoing.add(&edge.quantity)?;
                }
                (CurrentAccount::Participant(_), CurrentAccount::Participant(_)) => {
                    transferred.add(&edge.quantity)?;
                }
                _ => return Err(BondStateError::EdgeDoesNotCrossBond),
            }
        }
        self.holdings = holdings;
        self.historical = historical;
        self.incoming = incoming;
        self.outgoing = outgoing;
        self.transferred = transferred;
        self.last_transaction = Some(transaction.id.clone());
        self.accumulation = None;
        self.evaluation = None;
        Ok(())
    }

    fn apply_accumulation(
        &mut self,
        accumulation: &CurrentAccumulation,
    ) -> Result<(), BondStateError> {
        self.require_phase(BondPhase::Active)?;
        if self.transaction_ids.is_empty() {
            return Err(BondStateError::CurrentCirculationRequired);
        }
        let expected = self.current_accounting()?;
        if accumulation.accounting != expected
            || accumulation.through_transaction != self.last_transaction
        {
            return Err(BondStateError::AccumulationMismatch);
        }
        self.accumulation = Some(accumulation.clone());
        Ok(())
    }

    fn apply_aura(
        &mut self,
        observation: &AuraObservation,
        at: CausalPosition,
    ) -> Result<(), BondStateError> {
        self.require_phase(BondPhase::Active)?;
        if let BondTerm::Finite { end } = self.formation.term
            && at >= end
        {
            return Err(BondStateError::AuraOutsideActiveTerm);
        }
        if !self.participant_exists(&observation.observer) {
            return Err(BondStateError::UnknownParticipant(
                observation.observer.clone(),
            ));
        }
        if observation.evidence.is_empty() {
            return Err(BondStateError::MissingEvidence("Aura Observation"));
        }
        if !self.observation_ids.insert(observation.id.clone()) {
            return Err(BondStateError::DuplicateAuraObservation(
                observation.id.clone(),
            ));
        }
        self.aura.add(&observation.quantity)?;
        self.evaluation = None;
        Ok(())
    }

    fn apply_evaluation(
        &mut self,
        evaluation: &CurrentAuraEvaluation,
    ) -> Result<(), BondStateError> {
        self.require_phase(BondPhase::Active)?;
        if self.accumulation.is_none() {
            return Err(BondStateError::AccumulationRequired);
        }
        if self.observation_ids.is_empty() {
            return Err(BondStateError::AuraObservationRequired);
        }
        if evaluation.evidence.is_empty() {
            return Err(BondStateError::MissingEvidence("Current/Aura Evaluation"));
        }
        let current = self.retained_totals()?;
        let expected_polarity = polarity(&current, &self.aura)?;
        if evaluation.current != current
            || evaluation.aura != self.aura
            || evaluation.polarity != expected_polarity
        {
            return Err(BondStateError::EvaluationMismatch);
        }
        self.evaluation = Some(evaluation.clone());
        Ok(())
    }

    fn apply_maturity(
        &mut self,
        maturity: &BondMaturity,
        at: CausalPosition,
    ) -> Result<(), BondStateError> {
        self.require_phase(BondPhase::Active)?;
        if self.accumulation.is_none() {
            return Err(BondStateError::AccumulationRequired);
        }
        if self.evaluation.is_none() {
            return Err(BondStateError::EvaluationRequired);
        }
        if !self.pending_challenges.is_empty() {
            return Err(BondStateError::PendingChallenge);
        }
        if !self.pending_defaults.is_empty() {
            return Err(BondStateError::PendingDefault);
        }
        if maturity.evidence.is_empty() {
            return Err(BondStateError::MissingEvidence("Maturity"));
        }
        match (&self.formation.term, maturity.trigger) {
            (BondTerm::Finite { end }, MaturityTrigger::FiniteTermCompleted) if at >= *end => {}
            (BondTerm::Perpetual, MaturityTrigger::PerpetualTermTerminated) => {}
            (BondTerm::Finite { .. }, MaturityTrigger::FiniteTermCompleted) => {
                return Err(BondStateError::PrematureMaturity);
            }
            _ => return Err(BondStateError::WrongMaturityTrigger),
        }
        self.phase = BondPhase::Mature;
        Ok(())
    }

    fn apply_excess(&mut self, excess: &NetExcess) -> Result<(), BondStateError> {
        self.require_phase(BondPhase::Mature)?;
        let expected = self.retained_totals()?.net();
        if excess != &expected {
            return Err(BondStateError::ExcessMismatch);
        }
        self.excess = Some(excess.clone());
        self.phase = BondPhase::ExcessCalculated;
        Ok(())
    }

    fn apply_condensation(
        &mut self,
        decision: &CondensationDecision,
        at: CausalPosition,
    ) -> Result<(), BondStateError> {
        self.require_phase(BondPhase::ExcessCalculated)?;
        if decision.evidence.is_empty() {
            return Err(BondStateError::MissingEvidence("Condensation Eligibility"));
        }
        let excess = self.excess.as_ref().ok_or(BondStateError::ExcessRequired)?;
        match (&decision.status, &decision.glaushouse_clearance) {
            (CondensationStatus::Eligible, Some(clearance)) => {
                if excess.is_zero() {
                    return Err(BondStateError::ZeroExcessCannotCondense);
                }
                clearance.require_accepted(HouseFunction::Clear)?;
                if clearance.causal_position > at {
                    return Err(BondStateError::DecisionFromFuture);
                }
                self.register_house_decision(clearance)?;
            }
            (CondensationStatus::Eligible, None) => {
                return Err(BondStateError::GlaushouseClearanceRequired);
            }
            (CondensationStatus::Ineligible(IneligibilityReason::NoNetExcess), None)
                if excess.is_zero() => {}
            (
                CondensationStatus::Ineligible(IneligibilityReason::ClearanceRejected),
                Some(clearance),
            ) if clearance.outcome == HouseDecisionOutcome::Rejected => {
                clearance.validate_for(HouseFunction::Clear)?;
                if clearance.causal_position > at {
                    return Err(BondStateError::DecisionFromFuture);
                }
                self.register_house_decision(clearance)?;
            }
            (
                CondensationStatus::Ineligible(IneligibilityReason::ClearanceInconclusive),
                Some(clearance),
            ) if clearance.outcome == HouseDecisionOutcome::Inconclusive => {
                clearance.validate_for(HouseFunction::Clear)?;
                if clearance.causal_position > at {
                    return Err(BondStateError::DecisionFromFuture);
                }
                self.register_house_decision(clearance)?;
            }
            _ => return Err(BondStateError::EligibilityMismatch),
        }
        self.condensation = Some(decision.clone());
        self.phase = BondPhase::EligibilityDecided;
        Ok(())
    }

    fn apply_tombstone(&mut self, tombstone: &Tombstone) -> Result<(), BondStateError> {
        self.require_phase(BondPhase::EligibilityDecided)?;
        if self.condensation.as_ref().map(|decision| &decision.status)
            != Some(&CondensationStatus::Eligible)
        {
            return Err(BondStateError::CondensationNotEligible);
        }
        let evaluation = self
            .evaluation
            .as_ref()
            .ok_or(BondStateError::EvaluationRequired)?;
        if tombstone.source_bond != self.formation.id
            || tombstone.governing_house != self.formation.governing_house
            || tombstone.governing_institution != self.formation.governing_institution
            || tombstone.participants != self.formation.participants
            || Some(&tombstone.constitutional_excess) != self.excess.as_ref()
            || tombstone.polarity != evaluation.polarity
            || tombstone.completed_obligations != self.completed_obligations()
            || tombstone.remaining_obligations != self.remaining_obligations()
            || tombstone.evidence.is_empty()
        {
            return Err(BondStateError::TombstoneMismatch);
        }
        self.tombstone = Some(tombstone.clone());
        self.phase = BondPhase::TombstoneFormed;
        Ok(())
    }

    fn apply_tombstone_validation(
        &mut self,
        validation: &TombstoneValidation,
    ) -> Result<(), BondStateError> {
        self.require_phase(BondPhase::TombstoneFormed)?;
        if validation.replay_digest.is_empty() || validation.evidence.is_empty() {
            return Err(BondStateError::InvalidTombstoneValidation);
        }
        if self
            .formation
            .participants
            .iter()
            .any(|participant| participant.id.as_str() == validation.validator.as_str())
        {
            return Err(BondStateError::ValidatorIsBondParticipant);
        }
        self.tombstone_validated = true;
        self.phase = BondPhase::TombstoneValidated;
        Ok(())
    }

    fn apply_recognition(
        &mut self,
        recognition: &HouseDecision,
        at: CausalPosition,
    ) -> Result<(), BondStateError> {
        self.require_phase(BondPhase::TombstoneValidated)?;
        recognition.require_accepted(HouseFunction::Recognize)?;
        if recognition.causal_position > at {
            return Err(BondStateError::DecisionFromFuture);
        }
        self.register_house_decision(recognition)?;
        self.flynt_recognition = Some(recognition.clone());
        Ok(())
    }

    fn apply_toke(&mut self, toke: &Toke) -> Result<(), BondStateError> {
        self.require_phase(BondPhase::TombstoneValidated)?;
        if self.flynt_recognition.is_none() {
            return Err(BondStateError::FlyntRecognitionRequired);
        }
        let tombstone = self
            .tombstone
            .as_ref()
            .ok_or(BondStateError::TombstoneRequired)?;
        if toke.tombstone != tombstone.id || toke.index_key.is_empty() || toke.evidence.is_empty() {
            return Err(BondStateError::TokeMismatch);
        }
        self.toke = Some(toke.clone());
        self.phase = BondPhase::Recorded;
        Ok(())
    }

    fn apply_omission(
        &mut self,
        omission: &TombstoneOmission,
        stage: u8,
    ) -> Result<(), BondStateError> {
        self.require_phase(BondPhase::EligibilityDecided)?;
        let Some(CondensationDecision {
            status: CondensationStatus::Ineligible(reason),
            ..
        }) = &self.condensation
        else {
            return Err(BondStateError::OmissionNotPermitted);
        };
        if reason != &omission.reason || stage != self.omission_stage + 1 {
            return Err(BondStateError::OmissionStageMismatch);
        }
        self.omission_stage = stage;
        Ok(())
    }

    fn apply_challenge(&mut self, challenge: &BondChallenge) -> Result<(), BondStateError> {
        self.require_phase(BondPhase::Active)?;
        if !self.participant_exists(&challenge.challenger) {
            return Err(BondStateError::UnknownParticipant(
                challenge.challenger.clone(),
            ));
        }
        if challenge.evidence.is_empty() {
            return Err(BondStateError::MissingEvidence("Challenge"));
        }
        if !self.challenge_ids.insert(challenge.id.clone()) {
            return Err(BondStateError::DuplicateChallenge(challenge.id.clone()));
        }
        self.pending_challenges.insert(challenge.id.clone());
        self.evaluation = None;
        Ok(())
    }

    fn apply_challenge_resolution(
        &mut self,
        resolution: &BondChallengeResolution,
        at: CausalPosition,
    ) -> Result<(), BondStateError> {
        self.require_phase(BondPhase::Active)?;
        if !self.pending_challenges.remove(&resolution.challenge) {
            return Err(BondStateError::UnknownPendingChallenge(
                resolution.challenge.clone(),
            ));
        }
        if resolution.evidence.is_empty() {
            return Err(BondStateError::MissingEvidence("Challenge Resolution"));
        }
        resolution
            .sandmanor_proof
            .require_accepted(HouseFunction::Prove)?;
        if resolution.sandmanor_proof.causal_position > at {
            return Err(BondStateError::DecisionFromFuture);
        }
        self.register_house_decision(&resolution.sandmanor_proof)?;
        self.evaluation = None;
        Ok(())
    }

    fn apply_default(&mut self, default: &BondDefault) -> Result<(), BondStateError> {
        self.require_phase(BondPhase::Active)?;
        if !self.participant_exists(&default.participant) {
            return Err(BondStateError::UnknownParticipant(
                default.participant.clone(),
            ));
        }
        if !self.formation.obligations.contains(&default.obligation) {
            return Err(BondStateError::UnknownObligation(
                default.obligation.clone(),
            ));
        }
        if default.evidence.is_empty() {
            return Err(BondStateError::MissingEvidence("Default"));
        }
        if !self.default_ids.insert(default.id.clone()) {
            return Err(BondStateError::DuplicateDefault(default.id.clone()));
        }
        self.pending_defaults
            .insert(default.id.clone(), default.obligation.clone());
        self.evaluation = None;
        Ok(())
    }

    fn apply_default_resolution(
        &mut self,
        resolution: &BondDefaultResolution,
    ) -> Result<(), BondStateError> {
        self.require_phase(BondPhase::Active)?;
        if resolution.evidence.is_empty() {
            return Err(BondStateError::MissingEvidence("Default Resolution"));
        }
        let obligation = self
            .pending_defaults
            .remove(&resolution.default)
            .ok_or_else(|| BondStateError::UnknownPendingDefault(resolution.default.clone()))?;
        match resolution.outcome {
            DefaultOutcome::Cured => {
                self.confirmed_default_obligations.remove(&obligation);
            }
            DefaultOutcome::Confirmed => {
                self.confirmed_default_obligations.insert(obligation);
            }
        }
        self.evaluation = None;
        Ok(())
    }

    fn apply_resolution(
        &mut self,
        resolution: &BondResolution,
        at: CausalPosition,
    ) -> Result<(), BondStateError> {
        match self.phase {
            BondPhase::Recorded => {}
            BondPhase::EligibilityDecided if self.omission_stage == 3 => {}
            _ => {
                return Err(BondStateError::WrongResolutionPrecondition {
                    phase: self.phase,
                    omission_stage: self.omission_stage,
                });
            }
        }
        resolution
            .glaushouse_resolution
            .require_accepted(HouseFunction::Resolve)?;
        if resolution.glaushouse_resolution.causal_position > at {
            return Err(BondStateError::DecisionFromFuture);
        }
        self.register_house_decision(&resolution.glaushouse_resolution)?;
        if resolution.evidence.is_empty() {
            return Err(BondStateError::MissingEvidence("Synthesis Resolution"));
        }
        match resolution.disposition {
            ResolutionDisposition::Complete | ResolutionDisposition::Dissolve => {
                if !resolution.successor_bonds.is_empty() {
                    return Err(BondStateError::UnexpectedSuccessorBonds);
                }
            }
            ResolutionDisposition::Renew
            | ResolutionDisposition::Merge
            | ResolutionDisposition::Transfer
                if resolution.successor_bonds.len() != 1 =>
            {
                return Err(BondStateError::ExactlyOneSuccessorBondRequired);
            }
            ResolutionDisposition::Split | ResolutionDisposition::Branch
                if resolution.successor_bonds.len() < 2 =>
            {
                return Err(BondStateError::MultipleSuccessorBondsRequired);
            }
            _ => {}
        }
        if resolution
            .successor_bonds
            .iter()
            .any(|id| id == &self.formation.id)
        {
            return Err(BondStateError::SelfSuccessorBond);
        }
        self.resolution = Some(resolution.clone());
        self.phase = BondPhase::Resolved;
        Ok(())
    }

    #[must_use]
    pub fn id(&self) -> &BondId {
        &self.formation.id
    }

    #[must_use]
    pub const fn phase(&self) -> BondPhase {
        self.phase
    }

    #[must_use]
    pub const fn last_sequence(&self) -> u64 {
        self.last_sequence
    }

    #[must_use]
    pub const fn last_causal_position(&self) -> CausalPosition {
        self.last_causal_position
    }

    #[must_use]
    pub fn formation(&self) -> &BondFormation {
        &self.formation
    }

    #[must_use]
    pub fn resolution(&self) -> Option<&BondResolution> {
        self.resolution.as_ref()
    }

    pub fn current_accounting(&self) -> Result<CurrentAccounting, BondStateError> {
        let retained = self.retained_totals()?;
        Ok(CurrentAccounting {
            historical: self.historical.clone(),
            incoming: self.incoming.clone(),
            outgoing: self.outgoing.clone(),
            transferred: self.transferred.clone(),
            retained: retained.clone(),
            unresolved: retained,
        })
    }

    #[must_use]
    pub fn aura_totals(&self) -> &SignedTotals {
        &self.aura
    }

    pub fn calculated_evaluation(
        &self,
        id: EvaluationId,
        evidence: Vec<EvidenceRef>,
    ) -> Result<CurrentAuraEvaluation, BondStateError> {
        let current = self.retained_totals()?;
        Ok(CurrentAuraEvaluation {
            id,
            polarity: polarity(&current, &self.aura)?,
            current,
            aura: self.aura.clone(),
            evidence,
        })
    }

    pub fn calculated_excess(&self) -> Result<NetExcess, BondStateError> {
        Ok(self.retained_totals()?.net())
    }

    pub(crate) fn last_transaction_id(&self) -> Option<&CurrentTransactionId> {
        self.last_transaction.as_ref()
    }

    fn retained_totals(&self) -> Result<SignedTotals, BondStateError> {
        let mut totals = SignedTotals::zero(self.formation.current_unit.clone());
        for ((_, sign), magnitude) in &self.holdings {
            let quantity =
                SignedQuantity::new(*sign, *magnitude, self.formation.current_unit.clone());
            if *magnitude != 0 {
                totals.add(&quantity?)?;
            }
        }
        Ok(totals)
    }

    fn participant_exists(&self, id: &ParticipantId) -> bool {
        self.formation
            .participants
            .iter()
            .any(|participant| &participant.id == id)
    }

    fn completed_obligations(&self) -> Vec<ObligationId> {
        self.formation
            .obligations
            .iter()
            .filter(|obligation| !self.confirmed_default_obligations.contains(*obligation))
            .cloned()
            .collect()
    }

    fn remaining_obligations(&self) -> Vec<ObligationId> {
        self.formation
            .obligations
            .iter()
            .filter(|obligation| self.confirmed_default_obligations.contains(*obligation))
            .cloned()
            .collect()
    }

    fn register_house_decision(&mut self, decision: &HouseDecision) -> Result<(), BondStateError> {
        if !self.house_decision_ids.insert(decision.id.clone()) {
            return Err(BondStateError::DuplicateHouseDecision(decision.id.clone()));
        }
        Ok(())
    }
}

fn event_current_unit(event: &ConstitutionalEvent) -> Result<UnitId, BondStateError> {
    match &event.payload {
        BondEvent::Formed(formation) => Ok(formation.current_unit.clone()),
        _ => Err(BondStateError::FormationRequired),
    }
}

fn validate_formation(formation: &BondFormation, at: CausalPosition) -> Result<(), BondStateError> {
    formation
        .stonebend_naming
        .require_accepted(HouseFunction::Name)?;
    if formation.stonebend_naming.causal_position > at {
        return Err(BondStateError::DecisionFromFuture);
    }
    if formation.jurisdiction.institution != formation.governing_institution
        || formation.jurisdiction.house != formation.governing_house
        || formation.jurisdiction.evidence.is_empty()
    {
        return Err(BondStateError::JurisdictionMismatch);
    }
    if formation.jurisdiction.observed_at > at {
        return Err(BondStateError::JurisdictionFromFuture);
    }
    if formation.participants.is_empty() {
        return Err(BondStateError::ParticipantsRequired);
    }
    if formation.evidence.is_empty() {
        return Err(BondStateError::MissingEvidence("Bond Formation"));
    }
    let parent_count = formation.parent_bonds.iter().collect::<BTreeSet<_>>().len();
    if parent_count != formation.parent_bonds.len() {
        return Err(BondStateError::DuplicateParentBond);
    }
    if formation
        .parent_bonds
        .iter()
        .any(|parent| parent == &formation.id)
    {
        return Err(BondStateError::SelfParentBond);
    }
    if formation.parent_bonds.is_empty() != formation.inheritance_evidence.is_empty() {
        return Err(BondStateError::InheritanceEvidenceMismatch);
    }
    let mut participant_ids = BTreeSet::new();
    for participant in &formation.participants {
        if participant.roles.is_empty() {
            return Err(BondStateError::RolesRequired(participant.id.clone()));
        }
        if !participant_ids.insert(participant.id.clone()) {
            return Err(BondStateError::DuplicateParticipant(participant.id.clone()));
        }
        let role_count = participant.roles.iter().collect::<BTreeSet<_>>().len();
        if role_count != participant.roles.len() {
            return Err(BondStateError::DuplicateRole(participant.id.clone()));
        }
    }
    if let BondTerm::Finite { end } = formation.term
        && end <= at
    {
        return Err(BondStateError::InvalidFiniteTerm);
    }
    for initial in &formation.starting_current {
        if !participant_ids.contains(&initial.owner) {
            return Err(BondStateError::UnknownParticipant(initial.owner.clone()));
        }
        if !participant_ids.contains(&initial.custodian) {
            return Err(BondStateError::UnknownParticipant(
                initial.custodian.clone(),
            ));
        }
        if initial.quantity.unit != formation.current_unit {
            return Err(QuantityError::UnitMismatch {
                expected: formation.current_unit.clone(),
                actual: initial.quantity.unit.clone(),
            }
            .into());
        }
        if initial.evidence.is_empty() {
            return Err(BondStateError::MissingEvidence("Starting Current"));
        }
    }
    for aura in &formation.initial_aura {
        if aura.unit != formation.aura_unit {
            return Err(QuantityError::UnitMismatch {
                expected: formation.aura_unit.clone(),
                actual: aura.unit.clone(),
            }
            .into());
        }
    }
    Ok(())
}

fn validate_operation_shape(transaction: &CurrentTransaction) -> Result<(), BondStateError> {
    let participant_sources = transaction
        .edges
        .iter()
        .filter_map(|edge| match &edge.source {
            CurrentAccount::Participant(id) => Some(id),
            _ => None,
        })
        .collect::<BTreeSet<_>>();
    let participant_destinations = transaction
        .edges
        .iter()
        .filter_map(|edge| match &edge.destination {
            CurrentAccount::Participant(id) => Some(id),
            _ => None,
        })
        .collect::<BTreeSet<_>>();
    let valid = match transaction.operation {
        CurrentOperation::Enter => transaction.edges.iter().all(|edge| {
            matches!(edge.source, CurrentAccount::External(_))
                && matches!(edge.destination, CurrentAccount::Participant(_))
        }),
        CurrentOperation::Leave => transaction.edges.iter().all(|edge| {
            matches!(edge.source, CurrentAccount::Participant(_))
                && matches!(edge.destination, CurrentAccount::External(_))
        }),
        CurrentOperation::Consume | CurrentOperation::Decay | CurrentOperation::Expire => {
            transaction.edges.iter().all(|edge| {
                matches!(edge.source, CurrentAccount::Participant(_))
                    && matches!(edge.destination, CurrentAccount::Sink(_))
            })
        }
        CurrentOperation::Split | CurrentOperation::Branch => {
            participant_sources.len() == 1 && participant_destinations.len() >= 2
        }
        CurrentOperation::Merge => {
            participant_sources.len() >= 2 && participant_destinations.len() == 1
        }
        CurrentOperation::Transfer | CurrentOperation::Reverse | CurrentOperation::Circulate => {
            transaction.edges.iter().all(|edge| {
                matches!(edge.source, CurrentAccount::Participant(_))
                    && matches!(edge.destination, CurrentAccount::Participant(_))
            })
        }
    };
    if valid {
        Ok(())
    } else {
        Err(BondStateError::OperationShapeMismatch(
            transaction.operation,
        ))
    }
}

fn validate_account(
    account: &CurrentAccount,
    participants: &BTreeSet<ParticipantId>,
    source: bool,
) -> Result<(), BondStateError> {
    match account {
        CurrentAccount::Participant(id) if !participants.contains(id) => {
            Err(BondStateError::UnknownParticipant(id.clone()))
        }
        CurrentAccount::Sink(_) if source => Err(BondStateError::SinkCannotSourceCurrent),
        CurrentAccount::External(value) | CurrentAccount::Sink(value) if value.is_empty() => {
            Err(BondStateError::EmptyCurrentAccount)
        }
        _ => Ok(()),
    }
}

fn credit_holding(
    holdings: &mut Holdings,
    participant: &ParticipantId,
    quantity: &SignedQuantity,
) -> Result<(), QuantityError> {
    let entry = holdings
        .entry((participant.clone(), quantity.sign))
        .or_default();
    *entry = entry
        .checked_add(quantity.magnitude)
        .ok_or(QuantityError::Overflow)?;
    Ok(())
}

fn debit_holding(
    holdings: &mut Holdings,
    participant: &ParticipantId,
    quantity: &SignedQuantity,
) -> Result<(), QuantityError> {
    let entry = holdings
        .entry((participant.clone(), quantity.sign))
        .or_default();
    if *entry < quantity.magnitude {
        return Err(QuantityError::InsufficientCurrent {
            participant: participant.clone(),
            sign: quantity.sign,
            available: *entry,
            requested: quantity.magnitude,
        });
    }
    *entry -= quantity.magnitude;
    Ok(())
}

fn dominant_sign(totals: &SignedTotals, domain: &'static str) -> Result<Sign, BondStateError> {
    match totals.positive.cmp(&totals.negative) {
        std::cmp::Ordering::Greater => Ok(Sign::Positive),
        std::cmp::Ordering::Less => Ok(Sign::Negative),
        std::cmp::Ordering::Equal => Err(BondStateError::UnevaluableSignedDomain(domain)),
    }
}

fn polarity(
    current: &SignedTotals,
    aura: &SignedTotals,
) -> Result<ConstitutionalPolarity, BondStateError> {
    match (
        dominant_sign(current, "Current")?,
        dominant_sign(aura, "Aura")?,
    ) {
        (Sign::Positive, Sign::Positive) => Ok(ConstitutionalPolarity::PositiveCurrentPositiveAura),
        (Sign::Positive, Sign::Negative) => Ok(ConstitutionalPolarity::PositiveCurrentNegativeAura),
        (Sign::Negative, Sign::Positive) => Ok(ConstitutionalPolarity::NegativeCurrentPositiveAura),
        (Sign::Negative, Sign::Negative) => Ok(ConstitutionalPolarity::NegativeCurrentNegativeAura),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BondStateError {
    FormationRequired,
    InvalidFormationEnvelope,
    DuplicateFormation,
    WrongBond {
        expected: BondId,
        actual: BondId,
    },
    SequenceOverflow,
    SequenceMismatch {
        expected: u64,
        actual: u64,
    },
    CausalRegression {
        previous: CausalPosition,
        actual: CausalPosition,
    },
    WrongPhase {
        expected: BondPhase,
        actual: BondPhase,
    },
    RuleSetMismatch {
        expected: RuleSetId,
        actual: RuleSetId,
    },
    ParticipantsRequired,
    JurisdictionMismatch,
    JurisdictionFromFuture,
    RolesRequired(ParticipantId),
    DuplicateParticipant(ParticipantId),
    DuplicateRole(ParticipantId),
    DuplicateParentBond,
    SelfParentBond,
    InheritanceEvidenceMismatch,
    UnknownParticipant(ParticipantId),
    InvalidFiniteTerm,
    TermAlreadyCompleted,
    CurrentOutsideActiveTerm,
    AuraOutsideActiveTerm,
    MissingEvidence(&'static str),
    DecisionFromFuture,
    EmptyCurrentTransaction,
    CurrentCirculationRequired,
    DuplicateCurrentTransaction(CurrentTransactionId),
    OperationShapeMismatch(CurrentOperation),
    SelfCurrentEdge,
    EdgeDoesNotCrossBond,
    SinkCannotSourceCurrent,
    EmptyCurrentAccount,
    AccumulationMismatch,
    AccumulationRequired,
    DuplicateAuraObservation(AuraObservationId),
    DuplicateHouseDecision(HouseDecisionId),
    AuraObservationRequired,
    EvaluationMismatch,
    EvaluationRequired,
    DuplicateChallenge(ChallengeId),
    UnknownPendingChallenge(ChallengeId),
    PendingChallenge,
    UnknownObligation(ObligationId),
    DuplicateDefault(DefaultId),
    UnknownPendingDefault(DefaultId),
    PendingDefault,
    UnevaluableSignedDomain(&'static str),
    PrematureMaturity,
    WrongMaturityTrigger,
    ExcessRequired,
    ExcessMismatch,
    ZeroExcessCannotCondense,
    GlaushouseClearanceRequired,
    EligibilityMismatch,
    CondensationNotEligible,
    TombstoneMismatch,
    TombstoneRequired,
    InvalidTombstoneValidation,
    ValidatorIsBondParticipant,
    FlyntRecognitionRequired,
    TokeMismatch,
    OmissionNotPermitted,
    OmissionStageMismatch,
    WrongResolutionPrecondition {
        phase: BondPhase,
        omission_stage: u8,
    },
    UnexpectedSuccessorBonds,
    ExactlyOneSuccessorBondRequired,
    MultipleSuccessorBondsRequired,
    SelfSuccessorBond,
    Quantity(QuantityError),
    HouseLaw(HouseLawError),
}

impl From<QuantityError> for BondStateError {
    fn from(value: QuantityError) -> Self {
        Self::Quantity(value)
    }
}

impl From<HouseLawError> for BondStateError {
    fn from(value: HouseLawError) -> Self {
        Self::HouseLaw(value)
    }
}

impl std::fmt::Display for BondStateError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "Bond constitutional state violation: {self:?}")
    }
}

impl std::error::Error for BondStateError {}
