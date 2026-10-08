//! Read-only links from authored lore contracts to reducer-owned execution.
//!
//! Definitions and declared handoffs never count as executed work. This module
//! consumes an already validated runtime and cannot commit a choice or decision.

use std::fmt;

use serde::Serialize;

use crate::composition::ExternalRef;
use crate::constitutional::{BondEvent, HouseDecision, HouseFunction};
use crate::hollow_grove_contract::House;
use crate::kernel_invariants::{KernelInvariantError, audit_functional_lore_definitions};
use crate::world::lived_lore::{
    FunctionalLoreError, OperationalLoreHandoff, canonical_functional_lore_definitions,
};

use super::{
    BoardwalkCasePhase, BoardwalkChoice, BoardwalkEvidence, BoardwalkOutcomeId, GameplayEvent,
    HollowGroveGameRuntime, HuemanFaculty, RETURNING_GOON_BEING_ID, RETURNING_GOON_PARTICIPANT_ID,
};

pub const BOARDWALK_RETURN_LORE_ID: &str = "lore.flynt.boardwalk-return-recognition";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LoreRuleSource {
    pub path: &'static str,
    pub status: &'static str,
    pub consumer: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BoardwalkLoreChoice {
    SeekFiniteAffiliation,
    AcceptScopedWork,
    ReturnIndependently,
}

impl BoardwalkLoreChoice {
    #[must_use]
    pub const fn for_choice(choice: BoardwalkChoice) -> Self {
        match choice {
            BoardwalkChoice::PimpPatronage | BoardwalkChoice::GoonBond => {
                Self::SeekFiniteAffiliation
            }
            BoardwalkChoice::LimitedCooperation => Self::AcceptScopedWork,
            BoardwalkChoice::IndependentReturn => Self::ReturnIndependently,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LoreStopReason {
    MissingEvidence(BoardwalkEvidence),
    MissingFaculty(HuemanFaculty),
    MissingAuthority(&'static str),
    PlayerSupportRequired,
    ActorCommitmentRequired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LoreEventWitness {
    pub event_id: String,
    pub sequence: u64,
    pub causal_position: u64,
    pub kind: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LoreDecisionWitness {
    pub decision_id: String,
    pub function: &'static str,
    pub house: House,
    pub office_id: String,
    pub actor_id: String,
    pub causal_position: u64,
    pub evidence: Vec<ExternalRef>,
}

impl LoreDecisionWitness {
    fn from_decision(
        decision: &HouseDecision,
        function: HouseFunction,
    ) -> Result<Self, LoreInspectionError> {
        decision.require_accepted(function).map_err(|_| {
            LoreInspectionError::Execution("case decision is not accepted in its own jurisdiction")
        })?;
        Ok(Self {
            decision_id: decision.id.as_str().into(),
            function: match function {
                HouseFunction::Name => "Name",
                HouseFunction::Prove => "Prove",
                HouseFunction::Clear => "Clear",
                HouseFunction::Recognize => "Recognize",
                HouseFunction::Resolve => "Resolve",
            },
            house: decision.authority.house,
            office_id: decision.authority.office.as_str().into(),
            actor_id: decision.authority.actor.as_str().into(),
            causal_position: decision.causal_position.get(),
            evidence: decision
                .evidence
                .iter()
                .map(|evidence| evidence.0.clone())
                .collect(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BoardwalkLoreExecution {
    pub case_id: &'static str,
    pub affected_being_id: &'static str,
    pub decision_maker_participant_id: &'static str,
    pub phase: BoardwalkCasePhase,
    pub evidence: Vec<BoardwalkEvidence>,
    pub supported_choice: Option<BoardwalkChoice>,
    pub committed_choice: Option<BoardwalkChoice>,
    pub catalog_choice: Option<BoardwalkLoreChoice>,
    pub stops: Vec<LoreStopReason>,
    pub events: Vec<LoreEventWitness>,
    pub decisions: Vec<LoreDecisionWitness>,
    pub outcome_id: Option<String>,
    pub lawful_state_change: Option<&'static str>,
    pub uncertainty: Vec<&'static str>,
    pub failure_and_refusal: Vec<&'static str>,
    pub complete: bool,
    pub protected_independent_return: bool,
    pub next_step: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LoreDefinitionInspection {
    pub lore_id: &'static str,
    pub house: House,
    pub authority_source: &'static str,
    pub trigger: &'static str,
    pub player_choices: &'static [&'static str],
    pub uncertainty: &'static [&'static str],
    pub failure_and_refusal: &'static [&'static str],
    pub completion_condition: &'static str,
    pub declared_handoffs: Vec<OperationalLoreHandoff>,
    pub gameplay_binding: Option<&'static str>,
    pub execution: Option<BoardwalkLoreExecution>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LoreInspection {
    pub schema_version: u16,
    pub authority: &'static str,
    pub sources: Vec<LoreRuleSource>,
    pub definitions: Vec<LoreDefinitionInspection>,
    pub sympiote_foundation: Option<super::SympioteFoundationView>,
    pub sympiote_foundation_contract: LoreDefinitionInspection,
}

impl LoreInspection {
    pub fn from_runtime(
        runtime: Option<&HollowGroveGameRuntime>,
    ) -> Result<Self, LoreInspectionError> {
        let definitions = canonical_functional_lore_definitions();
        audit_functional_lore_definitions(&definitions)
            .ensure_passed()
            .map_err(LoreInspectionError::Invariant)?;
        let foundation = crate::world::sympiote_foundation::sympiote_foundation_lore();
        Ok(Self {
            schema_version: 2,
            sympiote_foundation: runtime.map(super::SympioteFoundationView::from_runtime),
            sympiote_foundation_contract: LoreDefinitionInspection {
                lore_id: foundation.id,
                house: foundation.house,
                trigger: foundation.trigger,
                player_choices: foundation.player_choices,
                uncertainty: foundation.uncertainty,
                failure_and_refusal: foundation.failure_and_refusal,
                completion_condition: foundation.completion_condition,
                authority_source: crate::world::sympiote_foundation::SYMPIOTE_FOUNDATION_SOURCE,
                declared_handoffs: foundation
                    .operational_handoffs()
                    .map_err(LoreInspectionError::Definition)?,
                gameplay_binding: Some(
                    crate::world::sympiote_foundation::SYMPIOTE_FOUNDATION_CASE_ID,
                ),
                execution: None,
            },
            authority: "read-only projection; declared handoffs do not attest execution",
            sources: vec![
                LoreRuleSource {
                    path: "HOLLOW_GROVE_COMPROMISE_V1_DRAFT.md",
                    status: "authoritative constitution",
                    consumer: "src/world/lived_lore.rs",
                },
                LoreRuleSource {
                    path: "HOLLOW_GROVE_OPERATIONAL_LORE_KERNEL_V1_DRAFT.md",
                    status: "adopted operational contract",
                    consumer: "src/world/lived_lore.rs::FunctionalLoreDefinition::operational_handoffs",
                },
                LoreRuleSource {
                    path: "BOARDWALK_GOON_BOND_VERTICAL_SLICE_V1_DRAFT.md",
                    status: "bounded playable case",
                    consumer: "src/gameplay/boardwalk.rs",
                },
                LoreRuleSource {
                    path: crate::world::sympiote_foundation::SYMPIOTE_FOUNDATION_SOURCE,
                    status: "adopted Sympiote foundation; bounded clinical and practice runtime",
                    consumer: "src/world/sympiote_foundation.rs",
                },
            ],
            definitions: definitions
                .iter()
                .map(|definition| {
                    let bound = definition.id == BOARDWALK_RETURN_LORE_ID;
                    Ok(LoreDefinitionInspection {
                        lore_id: definition.id,
                        house: definition.house,
                        trigger: definition.trigger,
                        player_choices: definition.player_choices,
                        uncertainty: definition.uncertainty,
                        failure_and_refusal: definition.failure_and_refusal,
                        completion_condition: definition.completion_condition,
                        authority_source: match definition.house {
                            House::Stonebend => "STONEBEND_CONSTITUTION_V2.md",
                            House::Sandmanor => "SANDMANOR_CONSTITUTION_V2.md",
                            House::Glaushouse => "GLAUSHOUSE_CONSTITUTION_V2.md",
                            House::Flynt => "FLYNT_CONSTITUTION_V2.md",
                        },
                        declared_handoffs: definition
                            .operational_handoffs()
                            .map_err(LoreInspectionError::Definition)?,
                        gameplay_binding: bound.then_some(super::BOARDWALK_RETURN_CASE_ID),
                        execution: if bound {
                            runtime.map(boardwalk_execution).transpose()?.flatten()
                        } else {
                            None
                        },
                    })
                })
                .collect::<Result<_, LoreInspectionError>>()?,
        })
    }

    #[must_use]
    pub fn witness_markdown(&self) -> String {
        use std::fmt::Write;
        let mut output = String::from(
            "# Hollow Grove Lore Inspection\n\nAuthored handoffs are declarations. Execution below is derived from gameplay records.\n\n",
        );
        for source in &self.sources {
            let _ = writeln!(
                output,
                "- Source: `{}` ({}) → `{}`",
                source.path, source.status, source.consumer
            );
        }
        output.push('\n');
        for definition in &self.definitions {
            let _ = writeln!(
                output,
                "## {}\n\n- Authority source: `{}`",
                definition.lore_id, definition.authority_source
            );
            let _ = writeln!(
                output,
                "- Gameplay binding: {}",
                definition
                    .gameplay_binding
                    .unwrap_or("definition only; no verified execution binding")
            );
            let _ = writeln!(
                output,
                "- Trigger: {}\n- Choices: {}\n- Uncertainty: {}\n- Declared completion: {}",
                definition.trigger,
                definition.player_choices.join("; "),
                definition.uncertainty.join("; "),
                definition.completion_condition
            );
            let _ = writeln!(
                output,
                "- Declared refusal: {}",
                definition.failure_and_refusal.join("; ")
            );
            output.push_str("\nDeclared handoffs:\n\n");
            for handoff in &definition.declared_handoffs {
                let _ = writeln!(
                    output,
                    "- {} → {}: {}",
                    handoff.from.label, handoff.to.label, handoff.question
                );
            }
            if let Some(execution) = &definition.execution {
                let _ = writeln!(
                    output,
                    "\nCase: `{}`; affected Being: `{}`",
                    execution.case_id, execution.affected_being_id
                );
                let _ = writeln!(
                    output,
                    "Phase: {:?}; complete: {}; protected independent return: {}",
                    execution.phase, execution.complete, execution.protected_independent_return
                );
                let _ = writeln!(output, "Next: {}", execution.next_step);
                output.push('\n');
                for stop in &execution.stops {
                    let _ = writeln!(output, "- Pending: {stop:?}");
                }
                for decision in &execution.decisions {
                    let _ = writeln!(
                        output,
                        "- {}: `{}` by `{}` via `{}`",
                        decision.function,
                        decision.decision_id,
                        decision.actor_id,
                        decision.office_id
                    );
                }
                for event in &execution.events {
                    let _ = writeln!(
                        output,
                        "- {}: `{}` at sequence {}",
                        event.kind, event.event_id, event.sequence
                    );
                }
                if let Some(change) = execution.lawful_state_change {
                    let _ = writeln!(output, "\nConsequence: {change}");
                    let _ = writeln!(
                        output,
                        "Uncertainty: {}\nFailure/refusal terms: {}",
                        execution.uncertainty.join("; "),
                        execution.failure_and_refusal.join("; ")
                    );
                }
            } else if definition.gameplay_binding.is_some() {
                output.push_str("\nNo case execution was supplied or started.\n");
            }
            output.push('\n');
        }
        if let Some(foundation) = &self.sympiote_foundation {
            let _ = writeln!(
                output,
                "## Sympiote foundation\n\nCase: `{}`\n\n- Phase: {:?}\n- Form established: {}\n- Needs rest: {}\n- Comfortable use: {}\n- Clinical decisions: {}\n- Practice direction: {:?}\n",
                foundation.case_id,
                foundation.phase,
                foundation.form_established,
                foundation.needs_rest,
                foundation.comfortable_use,
                foundation.clinical_receipt_count,
                foundation.direction
            );
        } else {
            output.push_str("## Sympiote foundation\n\nAuthored clinical/practice contract; no gameplay archive was supplied.\n");
        }
        output
    }
}

fn boardwalk_execution(
    runtime: &HollowGroveGameRuntime,
) -> Result<Option<BoardwalkLoreExecution>, LoreInspectionError> {
    let Some(case) = runtime.boardwalk_case() else {
        return Ok(None);
    };
    let mut stops = Vec::new();
    if case.committed_choice().is_none() {
        for evidence in BoardwalkEvidence::ALL {
            if !case.evidence().contains(&evidence) {
                stops.push(LoreStopReason::MissingEvidence(evidence));
            }
        }
        for faculty in HuemanFaculty::ALL {
            if !case.faculties().contains(&faculty) {
                stops.push(LoreStopReason::MissingFaculty(faculty));
            }
        }
        let mut offices = vec!["office.glaushouse.prima-donna", "office.flynt.tross"];
        if case
            .supported_choice()
            .is_some_and(|choice| choice != BoardwalkChoice::IndependentReturn)
        {
            offices.extend(["office.stonebend.hypergiant", "office.sandmanor.sandman"]);
        }
        for office in offices {
            if !runtime
                .world_session()
                .institutional()
                .catalog
                .office_holders
                .iter()
                .any(|holder| holder.active && holder.office.as_str() == office)
            {
                stops.push(LoreStopReason::MissingAuthority(office));
            }
        }
        stops.push(if case.supported_choice().is_some() {
            LoreStopReason::ActorCommitmentRequired
        } else {
            LoreStopReason::PlayerSupportRequired
        });
    }
    let mut execution = BoardwalkLoreExecution {
        case_id: case.id().as_str(),
        affected_being_id: RETURNING_GOON_BEING_ID,
        decision_maker_participant_id: RETURNING_GOON_PARTICIPANT_ID,
        phase: case.phase(),
        evidence: case.evidence().iter().copied().collect(),
        supported_choice: case.supported_choice(),
        committed_choice: case.committed_choice(),
        catalog_choice: case.committed_choice().map(BoardwalkLoreChoice::for_choice),
        stops,
        events: Vec::new(),
        decisions: Vec::new(),
        outcome_id: None,
        lawful_state_change: None,
        uncertainty: Vec::new(),
        failure_and_refusal: Vec::new(),
        complete: false,
        protected_independent_return: false,
        next_step: if case.supported_choice().is_some() {
            "The resident may commit a choice; player support remains nonbinding."
        } else {
            "Disclose evidence and faculties, then offer nonbinding support. Independent return remains a choice."
        },
    };
    let mut commitment = None;
    for event in runtime.events() {
        let kind = match &event.payload {
            GameplayEvent::HuemanInteractionOpened {
                evidence: Some(_), ..
            } => "case evidence",
            GameplayEvent::BoardwalkFacultyDisclosed { .. } => "faculty disclosure",
            GameplayEvent::BoardwalkOptionSupported { .. } => "nonbinding player support",
            GameplayEvent::ReturningGoonChoiceCommitted {
                committed_by,
                choice,
                outcome_id,
                relationship_bond,
            } => {
                let outcome = case.outcome().ok_or(LoreInspectionError::Execution(
                    "commitment lacks an outcome",
                ))?;
                if commitment.is_some()
                    || committed_by.as_str() != RETURNING_GOON_PARTICIPANT_ID
                    || Some(*choice) != case.committed_choice()
                    || *outcome_id != BoardwalkOutcomeId::for_choice(*choice)
                    || *outcome_id != outcome.id
                    || relationship_bond.as_ref()
                        != outcome
                            .relationship
                            .as_ref()
                            .map(|relationship| &relationship.bond)
                {
                    return Err(LoreInspectionError::Execution(
                        "commitment does not match the affected actor and outcome",
                    ));
                }
                commitment = Some(event);
                "affected actor commitment and recorded outcome"
            }
            _ => continue,
        };
        execution.events.push(LoreEventWitness {
            event_id: event.id.as_str().into(),
            sequence: event.sequence,
            causal_position: event.causal_position.get(),
            kind,
        });
    }
    if let Some(outcome) = case.outcome() {
        let event = commitment.ok_or(LoreInspectionError::Execution(
            "an outcome without a commitment is not completion",
        ))?;
        if !case.is_ready()
            || case.supported_choice() != Some(outcome.choice)
            || !outcome.player_support_is_nonbinding
            || outcome.subject != case.returning_goon()
            || outcome.case != case.id()
        {
            return Err(LoreInspectionError::Execution(
                "outcome lacks subject-bound evidence and commitment",
            ));
        }
        execution.decisions.push(LoreDecisionWitness::from_decision(
            &outcome.glaushouse_discharge_clearance,
            HouseFunction::Clear,
        )?);
        execution.decisions.push(LoreDecisionWitness::from_decision(
            &outcome.flynt_recognition,
            HouseFunction::Recognize,
        )?);
        if let Some(relationship) = &outcome.relationship {
            let bond = runtime.constitutional().bond(&relationship.bond).ok_or(
                LoreInspectionError::Execution("relationship lacks its constitutional Bond"),
            )?;
            execution.decisions.push(LoreDecisionWitness::from_decision(
                &bond.formation().stonebend_naming,
                HouseFunction::Name,
            )?);
            let events = runtime
                .constitutional()
                .events_for(&relationship.bond)
                .collect::<Vec<_>>();
            let [formed, validated, activated, ..] = events.as_slice() else {
                return Err(LoreInspectionError::Execution(
                    "relationship lacks its ordered formation, proof, and activation",
                ));
            };
            if formed.id != relationship.formed_event
                || validated.id != relationship.validated_event
                || activated.id != relationship.activated_event
                || !matches!(formed.payload, BondEvent::Formed(_))
                || !matches!(activated.payload, BondEvent::Activated(_))
            {
                return Err(LoreInspectionError::Execution(
                    "relationship event references diverge",
                ));
            }
            let BondEvent::Validated(proof) = &validated.payload else {
                return Err(LoreInspectionError::Execution(
                    "relationship lacks Sandmanor proof",
                ));
            };
            execution.decisions.push(LoreDecisionWitness::from_decision(
                &proof.sandmanor_proof,
                HouseFunction::Prove,
            )?);
        } else if outcome.choice != BoardwalkChoice::IndependentReturn {
            return Err(LoreInspectionError::Execution(
                "a finite relationship choice lacks its Bond",
            ));
        }
        if execution
            .decisions
            .iter()
            .any(|decision| decision.causal_position > event.causal_position.get())
        {
            return Err(LoreInspectionError::Execution(
                "a decision cannot follow its committed consequence",
            ));
        }
        execution.outcome_id = Some(outcome.id.as_str().into());
        execution.lawful_state_change = Some(outcome.lawful_state_change);
        execution.uncertainty = outcome.faculty_uncertainties.clone();
        execution.failure_and_refusal = outcome.failure_and_refusal.clone();
        execution.complete = true;
        execution.protected_independent_return =
            outcome.choice == BoardwalkChoice::IndependentReturn;
        execution.next_step = "The resident receives the recorded return outcome. Existing refusal, exit, and review terms remain inspectable.";
    }
    Ok(Some(execution))
}

#[derive(Debug)]
pub enum LoreInspectionError {
    Definition(FunctionalLoreError),
    Invariant(KernelInvariantError),
    Execution(&'static str),
}

impl fmt::Display for LoreInspectionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Definition(error) => error.fmt(formatter),
            Self::Invariant(error) => error.fmt(formatter),
            Self::Execution(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for LoreInspectionError {}
