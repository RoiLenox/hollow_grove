//! Read-only constitutional observability.
//!
//! Trace records are projections of accepted event envelopes or returned
//! reducer errors. They contain no transition function and cannot mutate law.

use std::fmt;

use crate::hollow_grove_contract::House;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraceDisposition {
    Accepted,
    Rejected,
}

impl TraceDisposition {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "Accepted",
            Self::Rejected => "Rejected",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitionTrace {
    pub sequence: u64,
    pub event_id: String,
    pub command: &'static str,
    pub prior_state: String,
    pub proposed_state: String,
    pub acting_house: Option<House>,
    pub acting_institution: Option<String>,
    pub authority: Vec<String>,
    pub evidence: Vec<String>,
    pub current_polarity: Option<Sign>,
    pub aura_polarity: Option<Sign>,
    pub region: Option<ConstitutionalRegion>,
    pub predecessor_being: Option<String>,
    pub resulting_being: Option<String>,
    pub synthesis_rule: Option<&'static str>,
    pub regional_function: Option<RegionalFunction>,
    pub stewardship: Vec<&'static str>,
    pub guardianship: Vec<&'static str>,
    pub prevalidation_digest: Option<String>,
    pub disposition: TraceDisposition,
    pub failure_code: Option<String>,
    pub state_changed: bool,
}

impl fmt::Display for TransitionTrace {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(formatter, "[{:<03}] {}", self.sequence, self.command)?;
        writeln!(formatter, "Event: {}", self.event_id)?;
        writeln!(formatter, "Prior State: {}", self.prior_state)?;
        writeln!(formatter, "Proposed State: {}", self.proposed_state)?;
        if let Some(house) = self.acting_house {
            writeln!(formatter, "House: {}", house_name(house))?;
        }
        if let Some(institution) = &self.acting_institution {
            writeln!(formatter, "Institution: {institution}")?;
        }
        if !self.authority.is_empty() {
            writeln!(formatter, "Authority: {}", self.authority.join(", "))?;
        }
        if !self.evidence.is_empty() {
            writeln!(formatter, "Evidence: {}", self.evidence.join(", "))?;
        }
        if let Some(current) = self.current_polarity {
            writeln!(formatter, "Current: {}", sign_name(current))?;
        }
        if let Some(aura) = self.aura_polarity {
            writeln!(formatter, "Aura: {}", sign_name(aura))?;
        }
        if let Some(region) = self.region {
            writeln!(formatter, "Region: {}", region.as_str())?;
        }
        if let Some(predecessor) = &self.predecessor_being {
            writeln!(formatter, "Predecessor: {predecessor}")?;
        }
        if let Some(result) = &self.resulting_being {
            writeln!(formatter, "Resulting Being: {result}")?;
        }
        if let Some(rule) = self.synthesis_rule {
            writeln!(formatter, "Synthesis Rule: {rule}")?;
        }
        if let Some(function) = self.regional_function {
            writeln!(formatter, "Regional Function: {}", function.as_str())?;
        }
        if !self.stewardship.is_empty() {
            writeln!(formatter, "Stewardship: {}", self.stewardship.join("; "))?;
        }
        if !self.guardianship.is_empty() {
            writeln!(formatter, "Guardianship: {}", self.guardianship.join("; "))?;
        }
        if let Some(digest) = &self.prevalidation_digest {
            writeln!(formatter, "Prevalidation Digest: {digest}")?;
        }
        writeln!(formatter, "Result: {}", self.disposition.as_str())?;
        if let Some(code) = &self.failure_code {
            writeln!(formatter, "Failure: {code}")?;
        }
        write!(
            formatter,
            "State Changed: {}",
            if self.state_changed { "Yes" } else { "No" }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstitutionalTrace {
    pub scenario: String,
    pub transitions: Vec<TransitionTrace>,
    pub live_replay_equivalent: bool,
    pub canonical_persistence: bool,
    pub archive_digest: Option<String>,
}

impl fmt::Display for ConstitutionalTrace {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(formatter, "Scenario: {}", self.scenario)?;
        for transition in &self.transitions {
            writeln!(formatter, "\n{transition}")?;
        }
        writeln!(
            formatter,
            "\nReplay: {}",
            if self.live_replay_equivalent {
                "Equivalent"
            } else {
                "Divergent"
            }
        )?;
        writeln!(
            formatter,
            "Persistence: {}",
            if self.canonical_persistence {
                "Canonical"
            } else {
                "Unavailable"
            }
        )?;
        if let Some(digest) = &self.archive_digest {
            writeln!(formatter, "Archive Digest: {digest}")?;
        }
        Ok(())
    }
}

fn evidence_label(evidence: &EvidenceRef) -> String {
    format!("{}:{}", evidence.0.namespace, evidence.0.key)
}

fn house_name(house: House) -> &'static str {
    match house {
        House::Stonebend => "Stonebend",
        House::Sandmanor => "Sandmanor",
        House::Glaushouse => "Glaüshouse",
        House::Flynt => "Flynt",
    }
}

fn sign_name(sign: Sign) -> &'static str {
    match sign {
        Sign::Positive => "Positive",
        Sign::Negative => "Negative",
    }
}

fn bond_phase_name(phase: BondPhase) -> &'static str {
    match phase {
        BondPhase::Formed => "Formed",
        BondPhase::Validated => "Validated",
        BondPhase::Active => "Active",
        BondPhase::Mature => "Mature",
        BondPhase::ExcessCalculated => "Excess Calculated",
        BondPhase::EligibilityDecided => "Eligibility Decided",
        BondPhase::TombstoneFormed => "Tombstone Formed",
        BondPhase::TombstoneValidated => "Tombstone Validated",
        BondPhase::Recorded => "Recorded",
        BondPhase::Resolved => "Resolved",
    }
}

fn decision_projection(
    decision: &HouseDecision,
) -> (Option<House>, Option<String>, Vec<String>, Vec<String>) {
    (
        Some(decision.authority.house),
        decision
            .authority
            .institution
            .as_ref()
            .map(|institution| institution.as_str().to_owned()),
        vec![
            decision.id.as_str().to_owned(),
            decision.authority.office.as_str().to_owned(),
        ],
        decision.evidence.iter().map(evidence_label).collect(),
    )
}

fn bond_event_name(event: &BondEvent) -> &'static str {
    match event {
        BondEvent::Formed(_) => "FORM_BOND",
        BondEvent::Validated(_) => "PROVE_BOND",
        BondEvent::Activated(_) => "ACTIVATE_BOND",
        BondEvent::CurrentMoved(_) => "MOVE_CURRENT",
        BondEvent::CurrentAccumulated(_) => "ACCUMULATE_CURRENT",
        BondEvent::AuraObserved(_) => "OBSERVE_AURA",
        BondEvent::Evaluated(_) => "EVALUATE_CURRENT_AURA",
        BondEvent::Matured(_) => "MATURE_BOND",
        BondEvent::ExcessCalculated(_) => "CALCULATE_EXCESS",
        BondEvent::CondensationDecided(_) => "DECIDE_CONDENSATION",
        BondEvent::TombstoneFormed(_) => "FORM_TOMBSTONE",
        BondEvent::TombstoneOmitted(_) => "OMIT_TOMBSTONE",
        BondEvent::TombstoneValidated(_) => "VALIDATE_TOMBSTONE",
        BondEvent::TombstoneValidationOmitted(_) => "OMIT_TOMBSTONE_VALIDATION",
        BondEvent::FlyntRecognized(_) => "RECOGNIZE_TOMBSTONE",
        BondEvent::TokeRecorded(_) => "RECORD_TOKE",
        BondEvent::TokeOmitted(_) => "OMIT_TOKE",
        BondEvent::ChallengeFiled(_) => "FILE_CHALLENGE",
        BondEvent::ChallengeResolved(_) => "RESOLVE_CHALLENGE",
        BondEvent::DefaultDeclared(_) => "DECLARE_DEFAULT",
        BondEvent::DefaultResolved(_) => "RESOLVE_DEFAULT",
        BondEvent::Resolved(_) => "RESOLVE_BOND",
    }
}

fn bond_state_label(event: &BondEvent) -> &'static str {
    match event {
        BondEvent::Formed(_) => "Formed",
        BondEvent::Validated(_) => "Validated",
        BondEvent::Activated(_) => "Active",
        BondEvent::CurrentMoved(_) => "Active / Current Moved",
        BondEvent::CurrentAccumulated(_) => "Active / Accumulated",
        BondEvent::AuraObserved(_) => "Active / Aura Observed",
        BondEvent::Evaluated(_) => "Active / Evaluated",
        BondEvent::Matured(_) => "Mature",
        BondEvent::ExcessCalculated(_) => "Excess Calculated",
        BondEvent::CondensationDecided(_) => "Eligibility Decided",
        BondEvent::TombstoneFormed(_) => "Tombstone Formed",
        BondEvent::TombstoneOmitted(_) => "Tombstone Omitted",
        BondEvent::TombstoneValidated(_) => "Tombstone Validated",
        BondEvent::TombstoneValidationOmitted(_) => "Validation Omitted",
        BondEvent::FlyntRecognized(_) => "Flynt Recognized",
        BondEvent::TokeRecorded(_) => "Recorded",
        BondEvent::TokeOmitted(_) => "Toke Omitted",
        BondEvent::ChallengeFiled(_) => "Challenged",
        BondEvent::ChallengeResolved(_) => "Challenge Resolved",
        BondEvent::DefaultDeclared(_) => "Defaulted",
        BondEvent::DefaultResolved(_) => "Default Resolved",
        BondEvent::Resolved(_) => "Resolved",
    }
}

pub fn trace_bond_scenario(scenario: &BondScenario) -> Result<ConstitutionalTrace, ScenarioError> {
    let replayed = ConstitutionalRuntime::replay(
        scenario.runtime.waves().cloned(),
        scenario.runtime.events().iter().cloned(),
    )
    .map_err(|error| ScenarioError(error.to_string()))?;
    let encoded = encode_constitutional_archive(&scenario.runtime)
        .map_err(|error| ScenarioError(error.to_string()))?;
    let canonical_persistence = decode_constitutional_archive(&encoded)
        .and_then(|decoded| encode_constitutional_archive(&decoded))
        .is_ok_and(|canonical| canonical == encoded);
    let digest = constitutional_replay_digest(&scenario.runtime)
        .map_err(|error| ScenarioError(error.to_string()))?;
    let mut prior = "Absent".to_owned();
    let mut transitions = Vec::new();
    for event in scenario
        .runtime
        .events()
        .iter()
        .filter(|event| event.bond == scenario.bond)
    {
        let (house, institution, authority, mut evidence) = match &event.payload {
            BondEvent::Formed(value) => decision_projection(&value.stonebend_naming),
            BondEvent::Validated(value) => decision_projection(&value.sandmanor_proof),
            BondEvent::CondensationDecided(value) => value
                .glaushouse_clearance
                .as_ref()
                .map_or((None, None, vec![], vec![]), decision_projection),
            BondEvent::FlyntRecognized(value) => decision_projection(value),
            BondEvent::Resolved(value) => decision_projection(&value.glaushouse_resolution),
            _ => (None, None, vec![], vec![]),
        };
        let (current_polarity, aura_polarity) = match &event.payload {
            BondEvent::Evaluated(value) => {
                evidence.extend(value.evidence.iter().map(evidence_label));
                (
                    Some(value.polarity.current_sign()),
                    Some(value.polarity.aura_sign()),
                )
            }
            BondEvent::TombstoneFormed(value) => {
                evidence.extend(value.evidence.iter().map(evidence_label));
                (
                    Some(value.polarity.current_sign()),
                    Some(value.polarity.aura_sign()),
                )
            }
            _ => (None, None),
        };
        let proposed = bond_state_label(&event.payload).to_owned();
        transitions.push(TransitionTrace {
            sequence: event.sequence,
            event_id: event.id.as_str().to_owned(),
            command: bond_event_name(&event.payload),
            prior_state: prior,
            proposed_state: proposed.clone(),
            acting_house: house,
            acting_institution: institution,
            authority,
            evidence,
            current_polarity,
            aura_polarity,
            region: None,
            predecessor_being: None,
            resulting_being: None,
            synthesis_rule: None,
            regional_function: None,
            stewardship: vec![],
            guardianship: vec![],
            prevalidation_digest: match &event.payload {
                BondEvent::TombstoneValidated(value) => Some(value.replay_digest.clone()),
                _ => None,
            },
            disposition: TraceDisposition::Accepted,
            failure_code: None,
            state_changed: true,
        });
        prior = proposed;
    }
    Ok(ConstitutionalTrace {
        scenario: scenario.name.into(),
        transitions,
        live_replay_equivalent: replayed.events() == scenario.runtime.events(),
        canonical_persistence,
        archive_digest: Some(digest),
    })
}

pub fn trace_regional_scenario(
    scenario: &RegionalScenario,
) -> Result<ConstitutionalTrace, ScenarioError> {
    let replayed = RegionalSynthesisRuntime::replay(scenario.runtime.events().iter().cloned())
        .map_err(|error| ScenarioError(error.to_string()))?;
    let decoded = decode_regional_archive(&scenario.archive)
        .map_err(|error| ScenarioError(error.to_string()))?;
    let mut transitions = Vec::new();
    for event in scenario.runtime.events() {
        let trace = match &event.payload {
            RegionalEvent::BeingRegistered(registration) => TransitionTrace {
                sequence: event.sequence,
                event_id: event.id.as_str().to_owned(),
                command: "REGISTER_BEING",
                prior_state: "Absent".into(),
                proposed_state: format!("Active {}", sandmanor_form_name(registration.form)),
                acting_house: Some(registration.standing.jurisdiction.house),
                acting_institution: Some(
                    registration
                        .standing
                        .jurisdiction
                        .institution
                        .as_str()
                        .to_owned(),
                ),
                authority: vec![registration.standing.jurisdiction.site.as_str().to_owned()],
                evidence: registration
                    .evidence
                    .iter()
                    .map(|value| evidence_label(&value.reference))
                    .collect(),
                current_polarity: None,
                aura_polarity: None,
                region: Some(registration.standing.region),
                predecessor_being: Some(registration.id.as_str().to_owned()),
                resulting_being: None,
                synthesis_rule: None,
                regional_function: None,
                stewardship: vec![],
                guardianship: vec![],
                prevalidation_digest: None,
                disposition: TraceDisposition::Accepted,
                failure_code: None,
                state_changed: true,
            },
            RegionalEvent::SynthesisCompleted(record) => {
                let (stewardship, guardianship) = match &record.result.assignment {
                    Some(RegionalAssignment::Minotaur { stewardship }) => (
                        stewardship
                            .duties
                            .iter()
                            .map(|duty| duty.as_str())
                            .collect(),
                        vec![],
                    ),
                    Some(RegionalAssignment::Centaur {
                        beach_occupation,
                        sea_guardianship,
                    }) => (
                        beach_occupation
                            .duties
                            .iter()
                            .map(|duty| duty.as_str())
                            .collect(),
                        sea_guardianship
                            .duties
                            .iter()
                            .map(|duty| duty.as_str())
                            .collect(),
                    ),
                    None => (vec![], vec![]),
                };
                let proof = &record.command.authority.sandmanor_proof;
                let resolution = &record.command.authority.glaushouse_resolution;
                TransitionTrace {
                    sequence: event.sequence,
                    event_id: event.id.as_str().to_owned(),
                    command: "SYNTHESIZE_BEING",
                    prior_state: format!(
                        "Active {}",
                        sandmanor_form_name(record.command.expected_predecessor_form)
                    ),
                    proposed_state: format!("Active {}", sandmanor_form_name(record.result.form)),
                    acting_house: Some(resolution.authority.house),
                    acting_institution: resolution
                        .authority
                        .institution
                        .as_ref()
                        .map(|value| value.as_str().to_owned()),
                    authority: vec![
                        proof.id.as_str().to_owned(),
                        resolution.id.as_str().to_owned(),
                    ],
                    evidence: record
                        .command
                        .evidence
                        .iter()
                        .map(|value| evidence_label(&value.reference))
                        .collect(),
                    current_polarity: None,
                    aura_polarity: None,
                    region: Some(record.rule.region()),
                    predecessor_being: Some(record.command.predecessor.as_str().to_owned()),
                    resulting_being: Some(record.result.id.as_str().to_owned()),
                    synthesis_rule: Some(record.rule.as_str()),
                    regional_function: Some(record.rule.function()),
                    stewardship,
                    guardianship,
                    prevalidation_digest: None,
                    disposition: TraceDisposition::Accepted,
                    failure_code: None,
                    state_changed: true,
                }
            }
            RegionalEvent::BeingTombstoned(record) => TransitionTrace {
                sequence: event.sequence,
                event_id: event.id.as_str().to_owned(),
                command: "TOMBSTONE_BEING",
                prior_state: "Active".into(),
                proposed_state: "Tombstoned".into(),
                acting_house: None,
                acting_institution: None,
                authority: vec![],
                evidence: record
                    .evidence
                    .iter()
                    .map(|value| evidence_label(&value.reference))
                    .collect(),
                current_polarity: None,
                aura_polarity: None,
                region: None,
                predecessor_being: Some(record.being.as_str().to_owned()),
                resulting_being: None,
                synthesis_rule: None,
                regional_function: None,
                stewardship: vec![],
                guardianship: vec![],
                prevalidation_digest: None,
                disposition: TraceDisposition::Accepted,
                failure_code: None,
                state_changed: true,
            },
        };
        transitions.push(trace);
    }
    Ok(ConstitutionalTrace {
        scenario: scenario.name.into(),
        transitions,
        live_replay_equivalent: replayed == scenario.runtime,
        canonical_persistence: decoded == scenario.runtime,
        archive_digest: Some(format!(
            "fnv1a64-regional-v1:{:016x}",
            regional_archive_digest(&scenario.runtime)
                .map_err(|error| ScenarioError(error.to_string()))?
        )),
    })
}

#[must_use]
pub fn trace_rejected_regional_scenario(
    scenario: &RejectedRegionalScenario,
) -> ConstitutionalTrace {
    ConstitutionalTrace {
        scenario: scenario.name.into(),
        transitions: vec![TransitionTrace {
            sequence: u64::try_from(scenario.event_count_before).unwrap_or(u64::MAX),
            event_id: format!("regional-event.{}.synthesize", scenario.name),
            command: "SYNTHESIZE_BEING",
            prior_state: "Active predecessor".into(),
            proposed_state: "Requested regional result".into(),
            acting_house: None,
            acting_institution: None,
            authority: vec![],
            evidence: vec![],
            current_polarity: None,
            aura_polarity: None,
            region: scenario
                .runtime
                .being(&scenario.predecessor)
                .map(|being| being.standing.region),
            predecessor_being: Some(scenario.predecessor.as_str().to_owned()),
            resulting_being: Some(scenario.attempted_result.as_str().to_owned()),
            synthesis_rule: None,
            regional_function: None,
            stewardship: vec![],
            guardianship: vec![],
            prevalidation_digest: None,
            disposition: TraceDisposition::Rejected,
            failure_code: Some(scenario.error.code().to_owned()),
            state_changed: scenario.event_count_after != scenario.event_count_before,
        }],
        live_replay_equivalent: RegionalSynthesisRuntime::replay(
            scenario.runtime.events().iter().cloned(),
        )
        .is_ok_and(|replayed| replayed == scenario.runtime),
        canonical_persistence: true,
        archive_digest: None,
    }
}

#[must_use]
pub fn trace_rejected_bond_scenario(scenario: &RejectedBondScenario) -> ConstitutionalTrace {
    ConstitutionalTrace {
        scenario: scenario.name.into(),
        transitions: vec![TransitionTrace {
            sequence: u64::try_from(
                scenario
                    .runtime
                    .events()
                    .iter()
                    .filter(|event| event.bond == scenario.bond)
                    .count(),
            )
            .unwrap_or(u64::MAX),
            event_id: format!("event.{}.attempt", scenario.name),
            command: if scenario.name == "premature-maturity" {
                "MATURE_BOND"
            } else {
                "RESOLVE_BOND"
            },
            prior_state: scenario.runtime.bond(&scenario.bond).map_or_else(
                || "Unknown".into(),
                |bond| bond_phase_name(bond.phase()).into(),
            ),
            proposed_state: if scenario.name == "premature-maturity" {
                "Mature"
            } else {
                "Second renewal resolution"
            }
            .into(),
            acting_house: None,
            acting_institution: None,
            authority: vec![],
            evidence: vec![],
            current_polarity: None,
            aura_polarity: None,
            region: None,
            predecessor_being: None,
            resulting_being: None,
            synthesis_rule: None,
            regional_function: None,
            stewardship: vec![],
            guardianship: vec![],
            prevalidation_digest: None,
            disposition: TraceDisposition::Rejected,
            failure_code: Some(scenario.failure_code.into()),
            state_changed: scenario.event_count_before != scenario.event_count_after,
        }],
        live_replay_equivalent: ConstitutionalRuntime::replay(
            scenario.runtime.waves().cloned(),
            scenario.runtime.events().iter().cloned(),
        )
        .is_ok_and(|replayed| replayed.events() == scenario.runtime.events()),
        canonical_persistence: encode_constitutional_archive(&scenario.runtime)
            .and_then(|archive| decode_constitutional_archive(&archive))
            .is_ok_and(|decoded| decoded.events() == scenario.runtime.events()),
        archive_digest: None,
    }
}

#[must_use]
pub fn trace_rejected_assignment_scenario(
    scenario: &RejectedRegionalAssignmentScenario,
) -> ConstitutionalTrace {
    ConstitutionalTrace {
        scenario: scenario.name.into(),
        transitions: vec![TransitionTrace {
            sequence: u64::try_from(scenario.event_count_before).unwrap_or(u64::MAX),
            event_id: format!("claim.{}", scenario.name),
            command: "VERIFY_REGIONAL_ASSIGNMENT_CLAIM",
            prior_state: "Accepted regional Being".into(),
            proposed_state: if scenario.name == "minotaur-sea-claim" {
                "Aura Sea guardian"
            } else {
                "Aura Field steward"
            }
            .into(),
            acting_house: None,
            acting_institution: None,
            authority: vec![],
            evidence: vec![],
            current_polarity: None,
            aura_polarity: None,
            region: Some(if scenario.name == "minotaur-sea-claim" {
                ConstitutionalRegion::AuraSea
            } else {
                ConstitutionalRegion::AuraFields
            }),
            predecessor_being: Some(scenario.being.as_str().into()),
            resulting_being: None,
            synthesis_rule: None,
            regional_function: None,
            stewardship: vec![],
            guardianship: vec![],
            prevalidation_digest: None,
            disposition: TraceDisposition::Rejected,
            failure_code: Some(scenario.error.code().into()),
            state_changed: scenario.event_count_before != scenario.event_count_after,
        }],
        live_replay_equivalent: RegionalSynthesisRuntime::replay(
            scenario.runtime.events().iter().cloned(),
        )
        .is_ok_and(|replayed| replayed == scenario.runtime),
        canonical_persistence: encode_regional_archive(&scenario.runtime)
            .and_then(|archive| decode_regional_archive(&archive))
            .is_ok_and(|decoded| decoded == scenario.runtime),
        archive_digest: None,
    }
}
