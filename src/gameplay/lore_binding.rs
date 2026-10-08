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
    HollowGroveGameRuntime, HuemanFaculty, MERCY_DEEP_BEING_ID, MERCY_DEEP_PARTICIPANT_ID,
    RETURNING_GOON_BEING_ID, RETURNING_GOON_PARTICIPANT_ID, StonebendCasePhase,
    StonebendContinuityChoice, StonebendEvidence, StonebendOutcomeId, StonebendOutcomeView,
};

pub const BOARDWALK_RETURN_LORE_ID: &str = "lore.flynt.boardwalk-return-recognition";
pub const CURRENT_SEA_CONTINUITY_LORE_ID: &str = "lore.stonebend.current-sea-continuity";
pub const LORE_INSPECTION_SCHEMA_VERSION: u16 = 4;

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
#[serde(rename_all = "snake_case")]
pub enum StonebendLoreStopReason {
    MissingEvidence(StonebendEvidence),
    MissingFaculty(HuemanFaculty),
    MissingAuthority(&'static str),
    PlayerSupportRequired,
    HouseDeterminationRequired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StonebendLoreExecution {
    pub case_id: &'static str,
    pub affected_being_id: &'static str,
    pub subject_participant_id: &'static str,
    pub phase: StonebendCasePhase,
    pub evidence: Vec<StonebendEvidence>,
    pub supported_choice: Option<StonebendContinuityChoice>,
    pub committed_choice: Option<StonebendContinuityChoice>,
    pub stops: Vec<StonebendLoreStopReason>,
    pub events: Vec<LoreEventWitness>,
    pub decisions: Vec<LoreDecisionWitness>,
    pub outcome: Option<StonebendOutcomeView>,
    pub complete: bool,
    pub completion_scope: &'static str,
    pub high_review_pending: bool,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stonebend_execution: Option<StonebendLoreExecution>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LoreInspection {
    pub schema_version: u16,
    pub authority: &'static str,
    pub sources: Vec<LoreRuleSource>,
    pub definitions: Vec<LoreDefinitionInspection>,
    pub sympiote_foundation: Option<super::SympioteFoundationView>,
    pub sympiote_foundation_contract: LoreDefinitionInspection,
    pub survey_frame: Option<super::SurveyFrameView>,
    pub survey_frame_contract: LoreDefinitionInspection,
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
        let frame = crate::world::survey_frame::survey_frame_lore();
        Ok(Self {
            schema_version: LORE_INSPECTION_SCHEMA_VERSION,
            survey_frame: runtime.map(super::SurveyFrameView::from_runtime),
            survey_frame_contract: LoreDefinitionInspection {
                lore_id: frame.id,
                house: frame.house,
                trigger: frame.trigger,
                player_choices: frame.player_choices,
                uncertainty: frame.uncertainty,
                failure_and_refusal: frame.failure_and_refusal,
                completion_condition: frame.completion_condition,
                authority_source: crate::world::survey_frame::SURVEY_FRAME_SOURCE,
                declared_handoffs: frame
                    .operational_handoffs()
                    .map_err(LoreInspectionError::Definition)?,
                gameplay_binding: Some(crate::world::survey_frame::SURVEY_FRAME_CASE_ID),
                execution: None,
                stonebend_execution: None,
            },
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
                stonebend_execution: None,
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
                    path: "STONEBEND_CURRENT_SEA_VERTICAL_SLICE_V1.md",
                    status: "implemented bounded continuity case; typed Name and Seal records",
                    consumer: "src/gameplay/stonebend.rs",
                },
                LoreRuleSource {
                    path: crate::world::sympiote_foundation::SYMPIOTE_FOUNDATION_SOURCE,
                    status: "adopted Sympiote foundation; bounded clinical and practice runtime",
                    consumer: "src/world/sympiote_foundation.rs",
                },
                LoreRuleSource {
                    path: crate::world::survey_frame::SURVEY_FRAME_SOURCE,
                    status: "adopted bounded object case; resident choice, exact proof and recovery",
                    consumer: "src/world/survey_frame.rs",
                },
            ],
            definitions: definitions
                .iter()
                .map(|definition| {
                    let boardwalk = definition.id == BOARDWALK_RETURN_LORE_ID;
                    let stonebend = definition.id == CURRENT_SEA_CONTINUITY_LORE_ID;
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
                        gameplay_binding: if boardwalk {
                            Some(super::BOARDWALK_RETURN_CASE_ID)
                        } else if stonebend {
                            Some(super::CURRENT_SEA_CONTINUITY_CASE_ID)
                        } else {
                            None
                        },
                        execution: if boardwalk {
                            runtime.map(boardwalk_execution).transpose()?.flatten()
                        } else {
                            None
                        },
                        stonebend_execution: if stonebend {
                            runtime.map(stonebend_execution).transpose()?.flatten()
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
                definition.gameplay_binding.unwrap_or_else(|| {
                    if crate::world::lore_handoffs::ROUTES
                        .iter()
                        .any(|r| r.lore_id() == definition.lore_id)
                    {
                        "headless backend handoff; gameplay integration deferred"
                    } else {
                        "definition only; no verified execution binding"
                    }
                })
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
            } else if let Some(execution) = &definition.stonebend_execution {
                let _ = writeln!(
                    output,
                    "\nCase: `{}`; affected Being: `{}`\nPhase: {:?}; bounded disposition complete: {}; high review pending: {}\nCompletion scope: {}\nNext: {}\n",
                    execution.case_id,
                    execution.affected_being_id,
                    execution.phase,
                    execution.complete,
                    execution.high_review_pending,
                    execution.completion_scope,
                    execution.next_step
                );
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
                if let Some(outcome) = &execution.outcome {
                    let _ = writeln!(
                        output,
                        "\nConsequence: {}\nDecision record: `{}`; Seal: `{}`\nActive Names: {}\nProvisional Names: {}\nTitle granted: {}\nUncertainty: {}\nFailure/refusal terms: {}",
                        outcome.lawful_state_change,
                        outcome.decision_record_id,
                        outcome.seal_record_id,
                        outcome.active_names.join("; "),
                        outcome.provisional_names.join("; "),
                        outcome.title_granted,
                        outcome.faculty_uncertainties.join("; "),
                        outcome.failure_and_refusal.join("; ")
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
        if let Some(frame) = &self.survey_frame {
            let _ = writeln!(
                output,
                "\n## Survey frame\n\nCase: `{}`\nPhase: {:?}; object revision: {}; disposition complete: {}\nMeasuring proof: {}; support proof: {}; recovery required: {}\nResident: `{}`; keeper: `{}`; recorder: `{}`\n",
                frame.case_id,
                frame.phase,
                frame.object_revision,
                frame.complete,
                frame.measurement_proven,
                frame.support_proven,
                frame.recovery_required,
                frame.resident,
                frame.keeper,
                frame.recorder
            );
            for revision in &frame.revisions {
                let _ = writeln!(
                    output,
                    "- Revision {}: {}; retained parts: {}",
                    revision.number,
                    revision.change,
                    revision.retained_parts.join("; ")
                );
                if let Some(proof) = &revision.support_receipt {
                    let _ = writeln!(
                        output,
                        "- Support record: {:?}, `{}` by `{}`",
                        proof.claim, proof.decision_id, proof.authority_actor
                    );
                }
            }
            for decision in &frame.resident_decisions {
                let _ = writeln!(
                    output,
                    "- Resident decision: {:?}, `{}`, revision {}; withdrawal: {}",
                    decision.choice, decision.id, decision.revision, decision.withdrawal
                );
            }
            for delivery in &frame.deliveries {
                let _ = writeln!(
                    output,
                    "- Delivered: {:?}, `{}` to `{}`; remaining: {}",
                    delivery.disposition,
                    delivery.id,
                    delivery.recipient,
                    delivery.outstanding_work.join("; ")
                );
            }
        } else {
            output.push_str("\n## Survey frame\n\nAdopted object-case contract; no gameplay archive supplied.\n");
        }
        output
    }
}

fn stonebend_execution(
    runtime: &HollowGroveGameRuntime,
) -> Result<Option<StonebendLoreExecution>, LoreInspectionError> {
    let Some(case) = runtime.stonebend_case() else {
        return Ok(None);
    };
    let mut stops = Vec::new();
    if case.committed_choice().is_none() {
        for evidence in StonebendEvidence::ALL {
            if !case.evidence().contains(&evidence) {
                stops.push(StonebendLoreStopReason::MissingEvidence(evidence));
            }
        }
        for faculty in HuemanFaculty::ALL {
            if !case.faculties().contains(&faculty) {
                stops.push(StonebendLoreStopReason::MissingFaculty(faculty));
            }
        }
        if !runtime
            .world_session()
            .institutional()
            .catalog
            .office_holders
            .iter()
            .any(|holder| holder.active && holder.office.as_str() == "office.stonebend.hypergiant")
        {
            stops.push(StonebendLoreStopReason::MissingAuthority(
                "office.stonebend.hypergiant",
            ));
        }
        stops.push(if case.supported_choice().is_some() {
            StonebendLoreStopReason::HouseDeterminationRequired
        } else {
            StonebendLoreStopReason::PlayerSupportRequired
        });
    }
    let mut execution = StonebendLoreExecution {
        case_id: case.id().as_str(),
        affected_being_id: MERCY_DEEP_BEING_ID,
        subject_participant_id: MERCY_DEEP_PARTICIPANT_ID,
        phase: case.phase(),
        evidence: case.evidence().iter().copied().collect(),
        supported_choice: case.supported_choice(),
        committed_choice: case.committed_choice(),
        stops,
        events: Vec::new(),
        decisions: Vec::new(),
        outcome: None,
        complete: false,
        completion_scope: "bounded Current Sea disposition; referral does not complete high review or create a final Name",
        high_review_pending: false,
        next_step: if case.supported_choice().is_some() {
            "Stonebend must record its scoped Name determination; player support remains advisory."
        } else {
            "Disclose subject-bound evidence and faculties before offering advisory support."
        },
    };
    let mut commitment = None;
    for event in runtime.events() {
        let kind = match &event.payload {
            GameplayEvent::HuemanInteractionOpened {
                stonebend_evidence: Some(_),
                ..
            } => "case evidence",
            GameplayEvent::StonebendFacultyDisclosed { .. } => "faculty disclosure",
            GameplayEvent::StonebendContinuityOptionSupported { .. } => "nonbinding player support",
            GameplayEvent::StonebendContinuityDeterminationCommitted {
                subject,
                choice,
                outcome_id,
                decision_id,
                authority_actor,
            } => {
                let outcome = case.outcome().ok_or(LoreInspectionError::Execution(
                    "Stonebend commitment lacks an outcome",
                ))?;
                if commitment.is_some()
                    || subject.as_str() != MERCY_DEEP_PARTICIPANT_ID
                    || Some(*choice) != case.committed_choice()
                    || *outcome_id != StonebendOutcomeId::for_choice(*choice)
                    || *outcome_id != outcome.id
                    || *decision_id != outcome.stonebend_naming.id
                    || *authority_actor != outcome.stonebend_naming.authority.actor
                    || outcome.stonebend_naming.causal_position != event.causal_position
                {
                    return Err(LoreInspectionError::Execution(
                        "Stonebend commitment does not match subject, decision and outcome",
                    ));
                }
                commitment = Some(event);
                "Stonebend determination and scoped Name/Seal disposition"
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
        commitment.ok_or(LoreInspectionError::Execution(
            "Stonebend outcome without a committed event is not completion",
        ))?;
        if !case.is_ready()
            || case.supported_choice() != Some(outcome.choice)
            || !outcome.player_support_is_nonbinding
            || outcome.title_granted
            || outcome.subject != case.subject()
            || outcome.case != case.id()
        {
            return Err(LoreInspectionError::Execution(
                "Stonebend outcome lacks subject-bound evidence or exceeds its scope",
            ));
        }
        outcome
            .determination
            .validate(outcome.choice, &outcome.stonebend_naming)
            .map_err(|_| {
                LoreInspectionError::Execution("Stonebend Name/Seal records are invalid")
            })?;
        execution.decisions.push(LoreDecisionWitness::from_decision(
            &outcome.stonebend_naming,
            HouseFunction::Name,
        )?);
        execution.outcome = Some(StonebendOutcomeView::from_record(outcome));
        execution.complete = true;
        execution.high_review_pending =
            outcome.choice == StonebendContinuityChoice::ReferIdentityConflict;
        execution.next_step = if execution.high_review_pending {
            "The sealed referral preserves evidence and challenge; high review remains pending and no final Name or Title is granted."
        } else {
            "Mercy Deep receives the scoped Name/Seal disposition; challenge and former-Name history remain available, with no Title grant."
        };
    }
    Ok(Some(execution))
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
