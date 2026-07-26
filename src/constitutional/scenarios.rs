//! Reusable constitutional V2 scenarios.
//!
//! These fixtures submit public commands to the production reducers. They are
//! shared by examples, conformance tests, trace generation, and benchmarks.

use std::fmt;

use crate::composition::ExternalRef;
use crate::hollow_grove_contract::House;
use crate::institution::{InstitutionCatalog, OfficeId};
use crate::lineage_contract::SandmanorForm;
use crate::world;
use crate::world::house_institutions::stonebend_constitution_id;

use super::*;

pub const V2_RULE_SET: &str = "rules.hollow-grove-v2";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScenarioExpectation {
    Accepted,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScenarioCategory {
    BondLifecycle,
    Polarity,
    Failure,
    Persistence,
    KernelAdapter,
    RegionalSynthesis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScenarioDescriptor {
    pub name: &'static str,
    pub category: ScenarioCategory,
    pub expectation: ScenarioExpectation,
    pub summary: &'static str,
}

pub const SCENARIO_CATALOG: &[ScenarioDescriptor] = &[
    ScenarioDescriptor {
        name: "ordinary-lifecycle",
        category: ScenarioCategory::BondLifecycle,
        expectation: ScenarioExpectation::Accepted,
        summary: "complete House-backed Bond lifecycle with renewal and successor lineage",
    },
    ScenarioDescriptor {
        name: "positive-positive",
        category: ScenarioCategory::Polarity,
        expectation: ScenarioExpectation::Accepted,
        summary: "Positive Current bonded to Positive Aura",
    },
    ScenarioDescriptor {
        name: "positive-negative",
        category: ScenarioCategory::Polarity,
        expectation: ScenarioExpectation::Accepted,
        summary: "Positive Current bonded to Negative Aura",
    },
    ScenarioDescriptor {
        name: "negative-positive",
        category: ScenarioCategory::Polarity,
        expectation: ScenarioExpectation::Accepted,
        summary: "Negative Current bonded to Positive Aura",
    },
    ScenarioDescriptor {
        name: "negative-negative",
        category: ScenarioCategory::Polarity,
        expectation: ScenarioExpectation::Accepted,
        summary: "Negative Current bonded to Negative Aura",
    },
    ScenarioDescriptor {
        name: "default-challenge",
        category: ScenarioCategory::BondLifecycle,
        expectation: ScenarioExpectation::Accepted,
        summary: "lawful default cure and Sandmanor-proven rejected challenge before maturity",
    },
    ScenarioDescriptor {
        name: "premature-maturity",
        category: ScenarioCategory::Failure,
        expectation: ScenarioExpectation::Rejected,
        summary: "fully evaluated Bond attempts maturity before its finite term",
    },
    ScenarioDescriptor {
        name: "renewal-after-terminal",
        category: ScenarioCategory::Failure,
        expectation: ScenarioExpectation::Rejected,
        summary: "resolved Bond attempts a second renewal resolution",
    },
    ScenarioDescriptor {
        name: "gnome-minotaur",
        category: ScenarioCategory::RegionalSynthesis,
        expectation: ScenarioExpectation::Accepted,
        summary: "Gnome to Minotaur in the Aura Field with typed stewardship",
    },
    ScenarioDescriptor {
        name: "elf-centaur",
        category: ScenarioCategory::RegionalSynthesis,
        expectation: ScenarioExpectation::Accepted,
        summary: "Elf to Centaur on the Aura Beach with Aura Sea guardianship",
    },
    ScenarioDescriptor {
        name: "gnome-centaur",
        category: ScenarioCategory::Failure,
        expectation: ScenarioExpectation::Rejected,
        summary: "unratified Gnome to Centaur cross-lineage Synthesis",
    },
    ScenarioDescriptor {
        name: "elf-minotaur",
        category: ScenarioCategory::Failure,
        expectation: ScenarioExpectation::Rejected,
        summary: "unratified Elf to Minotaur cross-lineage Synthesis",
    },
    ScenarioDescriptor {
        name: "gnome-minotaur-wrong-region",
        category: ScenarioCategory::Failure,
        expectation: ScenarioExpectation::Rejected,
        summary: "Gnome without Aura Field standing attempts Minotaur Synthesis",
    },
    ScenarioDescriptor {
        name: "elf-centaur-wrong-region",
        category: ScenarioCategory::Failure,
        expectation: ScenarioExpectation::Rejected,
        summary: "Elf without Aura Beach standing attempts Centaur Synthesis",
    },
    ScenarioDescriptor {
        name: "synthesis-without-authority",
        category: ScenarioCategory::Failure,
        expectation: ScenarioExpectation::Rejected,
        summary: "regional Synthesis with a rejected resolution decision",
    },
    ScenarioDescriptor {
        name: "synthesis-without-evidence",
        category: ScenarioCategory::Failure,
        expectation: ScenarioExpectation::Rejected,
        summary: "regional Synthesis without subject-bound result evidence",
    },
    ScenarioDescriptor {
        name: "synthesis-mismatched-evidence",
        category: ScenarioCategory::Failure,
        expectation: ScenarioExpectation::Rejected,
        summary: "regional Synthesis using evidence for another Being",
    },
    ScenarioDescriptor {
        name: "minotaur-sea-claim",
        category: ScenarioCategory::Failure,
        expectation: ScenarioExpectation::Rejected,
        summary: "Minotaur claims Centaur-only Aura Sea guardianship",
    },
    ScenarioDescriptor {
        name: "centaur-fields-claim",
        category: ScenarioCategory::Failure,
        expectation: ScenarioExpectation::Rejected,
        summary: "Centaur claims Minotaur-only Aura Field stewardship",
    },
    ScenarioDescriptor {
        name: "kernel-wave",
        category: ScenarioCategory::KernelAdapter,
        expectation: ScenarioExpectation::Accepted,
        summary: "completed recursion pass becomes a Wave without moving Current",
    },
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioError(pub String);

impl fmt::Display for ScenarioError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for ScenarioError {}

fn scenario_error(error: impl fmt::Display) -> ScenarioError {
    ScenarioError(error.to_string())
}

fn stable<T>(
    value: &str,
    constructor: impl FnOnce(String) -> Result<T, ConstitutionalIdError>,
) -> T {
    constructor(value.to_owned()).expect("scenario identifiers are canonical constants")
}

#[must_use]
pub fn scenario_evidence(key: &str) -> EvidenceRef {
    EvidenceRef(ExternalRef::new("scenario", key).expect("scenario evidence key is nonempty"))
}

#[must_use]
pub fn scenario_event_metadata(label: &str, at: u64) -> EventMetadata {
    EventMetadata {
        id: stable(&format!("event.{label}"), ConstitutionalEventId::new),
        causal_position: CausalPosition::new(at),
        rule_set: stable(V2_RULE_SET, RuleSetId::new),
    }
}

#[must_use]
pub fn scenario_regional_metadata(label: &str, at: u64) -> RegionalEventMetadata {
    RegionalEventMetadata {
        id: stable(&format!("regional-event.{label}"), RegionalEventId::new),
        causal_position: CausalPosition::new(at),
        rule_set: stable(V2_RULE_SET, RuleSetId::new),
    }
}

fn office_for(function: HouseFunction) -> &'static str {
    match function {
        HouseFunction::Name => "office.stonebend.hypergiant",
        HouseFunction::Prove => "office.sandmanor.sandman",
        HouseFunction::Clear | HouseFunction::Resolve => "office.glaushouse.prima-donna",
        HouseFunction::Recognize => "office.flynt.tross",
    }
}

pub fn scenario_house_decision(
    catalog: &InstitutionCatalog,
    label: &str,
    function: HouseFunction,
    at: u64,
) -> Result<HouseDecision, ScenarioError> {
    let office = OfficeId::new(office_for(function)).expect("canonical scenario office");
    let holder = catalog
        .office_holders
        .iter()
        .find(|holder| holder.active && holder.office == office)
        .ok_or_else(|| ScenarioError(format!("missing active holder for {office:?}")))?;
    HouseDecision::from_catalog(
        catalog,
        HouseDecisionDraft {
            id: stable(&format!("decision.{label}"), HouseDecisionId::new),
            function,
            office,
            actor: stable(holder.being.as_str(), AuthorityActorId::new),
            outcome: HouseDecisionOutcome::Accepted,
            evidence: vec![scenario_evidence(&format!("authority.{label}"))],
            causal_position: CausalPosition::new(at),
        },
    )
    .map_err(scenario_error)
}

fn participant() -> BondParticipant {
    BondParticipant {
        id: stable("being.scenario-owner", ParticipantId::new),
        kind: ParticipantKind::Huemen,
        roles: vec![stable("role.owner", RoleId::new)],
    }
}

#[derive(Debug, Clone)]
pub struct BondScenario {
    pub name: &'static str,
    pub runtime: ConstitutionalRuntime,
    pub bond: BondId,
    pub formation: BondFormation,
    pub polarity: ConstitutionalPolarity,
    pub successor: Option<BondId>,
}

fn formed_bond(
    label: &str,
    current_sign: Sign,
    current_magnitude: u128,
) -> Result<(ConstitutionalRuntime, BondId, BondFormation), ScenarioError> {
    let world = world::institutional_access_fixture();
    let mut runtime = ConstitutionalRuntime::new();
    let wave = stable(&format!("wave.{label}"), WaveId::new);
    runtime
        .record_wave(WaveRecord {
            id: wave.clone(),
            origin: scenario_evidence(&format!("wave.{label}")),
            causal_position: CausalPosition::new(0),
        })
        .map_err(scenario_error)?;
    let owner = participant();
    let current_unit = stable("unit.current", UnitId::new);
    let institution = stonebend_constitution_id();
    let formation = BondFormation {
        id: stable(&format!("bond.{label}"), BondId::new),
        initiating_wave: wave,
        governing_house: House::Stonebend,
        governing_institution: institution.clone(),
        jurisdiction: InstitutionalJurisdictionSnapshot::from_catalog(
            &world.catalog,
            &institution,
            CausalPosition::new(1),
            vec![scenario_evidence(&format!("jurisdiction.{label}"))],
        )
        .map_err(scenario_error)?,
        parent_bonds: vec![],
        inheritance_evidence: vec![],
        participants: vec![owner.clone()],
        obligations: vec![stable("obligation.complete", ObligationId::new)],
        permissions: vec![stable("permission.circulate", PermissionId::new)],
        term: BondTerm::Finite {
            end: CausalPosition::new(10),
        },
        current_unit: current_unit.clone(),
        aura_unit: stable("unit.aura", UnitId::new),
        starting_current: vec![InitialCurrent {
            owner: owner.id.clone(),
            custodian: owner.id,
            quantity: SignedQuantity::new(current_sign, current_magnitude, current_unit)
                .map_err(scenario_error)?,
            evidence: vec![scenario_evidence(&format!("starting-current.{label}"))],
        }],
        initial_aura: vec![],
        evidence: vec![scenario_evidence(&format!("formation.{label}"))],
        stonebend_naming: scenario_house_decision(
            &world.catalog,
            &format!("{label}.name"),
            HouseFunction::Name,
            1,
        )?,
    };
    let bond = formation.id.clone();
    runtime
        .append(
            bond.clone(),
            scenario_event_metadata(&format!("{label}.form"), 1),
            BondEvent::Formed(formation.clone()),
        )
        .map_err(scenario_error)?;
    runtime
        .append(
            bond.clone(),
            scenario_event_metadata(&format!("{label}.prove"), 2),
            BondEvent::Validated(BondValidation {
                sandmanor_proof: scenario_house_decision(
                    &world.catalog,
                    &format!("{label}.prove"),
                    HouseFunction::Prove,
                    2,
                )?,
                evidence: vec![scenario_evidence(&format!("validation.{label}"))],
            }),
        )
        .map_err(scenario_error)?;
    runtime
        .append(
            bond.clone(),
            scenario_event_metadata(&format!("{label}.activate"), 3),
            BondEvent::Activated(BondActivation {
                evidence: vec![scenario_evidence(&format!("activation.{label}"))],
            }),
        )
        .map_err(scenario_error)?;
    Ok((runtime, bond, formation))
}

fn run_living_chemistry(
    runtime: &mut ConstitutionalRuntime,
    label: &str,
    bond: &BondId,
    formation: &BondFormation,
    current_sign: Sign,
    aura_sign: Sign,
) -> Result<ConstitutionalPolarity, ScenarioError> {
    runtime
        .append(
            bond.clone(),
            scenario_event_metadata(&format!("{label}.current"), 4),
            BondEvent::CurrentMoved(CurrentTransaction {
                id: stable(&format!("transaction.{label}"), CurrentTransactionId::new),
                wave: formation.initiating_wave.clone(),
                operation: CurrentOperation::Enter,
                edges: vec![CurrentEdge {
                    source: CurrentAccount::External("scenario-boundary".into()),
                    destination: CurrentAccount::Participant(formation.participants[0].id.clone()),
                    quantity: SignedQuantity::new(current_sign, 4, formation.current_unit.clone())
                        .map_err(scenario_error)?,
                }],
                evidence: vec![scenario_evidence(&format!("current.{label}"))],
            }),
        )
        .map_err(scenario_error)?;
    runtime
        .accumulate_current(
            bond,
            scenario_event_metadata(&format!("{label}.accumulate"), 5),
        )
        .map_err(scenario_error)?;
    runtime
        .append(
            bond.clone(),
            scenario_event_metadata(&format!("{label}.aura"), 6),
            BondEvent::AuraObserved(AuraObservation {
                id: stable(&format!("observation.{label}"), AuraObservationId::new),
                observer: formation.participants[0].id.clone(),
                quantity: SignedQuantity::new(aura_sign, 6, formation.aura_unit.clone())
                    .map_err(scenario_error)?,
                subject: scenario_evidence(&format!("current.{label}")),
                evidence: vec![scenario_evidence(&format!("aura.{label}"))],
            }),
        )
        .map_err(scenario_error)?;
    runtime
        .evaluate(
            bond,
            scenario_event_metadata(&format!("{label}.evaluate"), 7),
            stable(&format!("evaluation.{label}"), EvaluationId::new),
            vec![scenario_evidence(&format!("evaluation.{label}"))],
        )
        .map_err(scenario_error)?;
    runtime
        .bond(bond)
        .ok_or_else(|| ScenarioError("scenario Bond disappeared".into()))?
        .calculated_evaluation(
            stable(&format!("evaluation.{label}.inspect"), EvaluationId::new),
            vec![scenario_evidence(&format!("evaluation.{label}.inspect"))],
        )
        .map(|evaluation| evaluation.polarity)
        .map_err(scenario_error)
}

pub fn run_polarity_scenario(
    current_sign: Sign,
    aura_sign: Sign,
) -> Result<BondScenario, ScenarioError> {
    let name = match (current_sign, aura_sign) {
        (Sign::Positive, Sign::Positive) => "positive-positive",
        (Sign::Positive, Sign::Negative) => "positive-negative",
        (Sign::Negative, Sign::Positive) => "negative-positive",
        (Sign::Negative, Sign::Negative) => "negative-negative",
    };
    let (mut runtime, bond, formation) = formed_bond(name, current_sign, 10)?;
    let polarity = run_living_chemistry(
        &mut runtime,
        name,
        &bond,
        &formation,
        current_sign,
        aura_sign,
    )?;
    Ok(BondScenario {
        name,
        runtime,
        bond,
        formation,
        polarity,
        successor: None,
    })
}

/// Runs the complete proof-producing lifecycle and reserves/forms its renewal.
pub fn run_ordinary_lifecycle() -> Result<BondScenario, ScenarioError> {
    let name = "ordinary-lifecycle";
    let world = world::institutional_access_fixture();
    let (mut runtime, bond, formation) = formed_bond(name, Sign::Positive, 10)?;
    let polarity = run_living_chemistry(
        &mut runtime,
        name,
        &bond,
        &formation,
        Sign::Positive,
        Sign::Positive,
    )?;
    runtime
        .append(
            bond.clone(),
            scenario_event_metadata("ordinary.mature", 10),
            BondEvent::Matured(BondMaturity {
                trigger: MaturityTrigger::FiniteTermCompleted,
                evidence: vec![scenario_evidence("ordinary.maturity")],
            }),
        )
        .map_err(scenario_error)?;
    runtime
        .calculate_excess(&bond, scenario_event_metadata("ordinary.excess", 11))
        .map_err(scenario_error)?;
    runtime
        .append(
            bond.clone(),
            scenario_event_metadata("ordinary.clear", 12),
            BondEvent::CondensationDecided(CondensationDecision {
                status: CondensationStatus::Eligible,
                glaushouse_clearance: Some(scenario_house_decision(
                    &world.catalog,
                    "ordinary.clear",
                    HouseFunction::Clear,
                    12,
                )?),
                evidence: vec![scenario_evidence("ordinary.clearance")],
            }),
        )
        .map_err(scenario_error)?;
    let tombstone = stable("tombstone.ordinary", TombstoneId::new);
    let constitutional_excess = runtime
        .bond(&bond)
        .ok_or_else(|| ScenarioError("ordinary lifecycle Bond disappeared".into()))?
        .calculated_excess()
        .map_err(scenario_error)?;
    runtime
        .append(
            bond.clone(),
            scenario_event_metadata("ordinary.tombstone", 13),
            BondEvent::TombstoneFormed(Tombstone {
                id: tombstone.clone(),
                source_bond: bond.clone(),
                governing_house: formation.governing_house,
                governing_institution: formation.governing_institution.clone(),
                participants: formation.participants.clone(),
                constitutional_excess,
                polarity,
                completed_obligations: formation.obligations.clone(),
                remaining_obligations: vec![],
                evidence: vec![scenario_evidence("ordinary.tombstone")],
            }),
        )
        .map_err(scenario_error)?;
    let digest = constitutional_bond_replay_digest(&runtime, &bond).map_err(scenario_error)?;
    runtime
        .append(
            bond.clone(),
            scenario_event_metadata("ordinary.tombstone-validation", 14),
            BondEvent::TombstoneValidated(TombstoneValidation {
                validator: stable("being.scenario-validator", AuthorityActorId::new),
                validation_basis: scenario_evidence("ordinary.validation-basis"),
                replay_digest: digest,
                evidence: vec![scenario_evidence("ordinary.validation")],
            }),
        )
        .map_err(scenario_error)?;
    runtime
        .append(
            bond.clone(),
            scenario_event_metadata("ordinary.recognize", 15),
            BondEvent::FlyntRecognized(scenario_house_decision(
                &world.catalog,
                "ordinary.recognize",
                HouseFunction::Recognize,
                15,
            )?),
        )
        .map_err(scenario_error)?;
    runtime
        .append(
            bond.clone(),
            scenario_event_metadata("ordinary.toke", 16),
            BondEvent::TokeRecorded(Toke {
                id: stable("toke.ordinary", TokeId::new),
                tombstone,
                index_key: "scenario/ordinary-lifecycle".into(),
                evidence: vec![scenario_evidence("ordinary.toke")],
            }),
        )
        .map_err(scenario_error)?;
    let successor = stable("bond.ordinary-renewal", BondId::new);
    runtime
        .append(
            bond.clone(),
            scenario_event_metadata("ordinary.resolve", 17),
            BondEvent::Resolved(BondResolution {
                id: stable("resolution.ordinary", ResolutionId::new),
                disposition: ResolutionDisposition::Renew,
                successor_bonds: vec![successor.clone()],
                glaushouse_resolution: scenario_house_decision(
                    &world.catalog,
                    "ordinary.resolve",
                    HouseFunction::Resolve,
                    17,
                )?,
                evidence: vec![scenario_evidence("ordinary.resolution")],
            }),
        )
        .map_err(scenario_error)?;
    let successor_wave = stable("wave.ordinary-renewal", WaveId::new);
    runtime
        .record_wave(WaveRecord {
            id: successor_wave.clone(),
            origin: scenario_evidence("ordinary.successor-wave"),
            causal_position: CausalPosition::new(17),
        })
        .map_err(scenario_error)?;
    let mut successor_formation = formation.clone();
    successor_formation.id = successor.clone();
    successor_formation.initiating_wave = successor_wave;
    successor_formation.parent_bonds = vec![bond.clone()];
    successor_formation.inheritance_evidence = vec![scenario_evidence("ordinary.inheritance")];
    successor_formation.term = BondTerm::Finite {
        end: CausalPosition::new(30),
    };
    successor_formation.jurisdiction.observed_at = CausalPosition::new(18);
    successor_formation.stonebend_naming = scenario_house_decision(
        &world.catalog,
        "ordinary.successor-name",
        HouseFunction::Name,
        18,
    )?;
    runtime
        .append(
            successor.clone(),
            scenario_event_metadata("ordinary.successor-form", 18),
            BondEvent::Formed(successor_formation),
        )
        .map_err(scenario_error)?;
    runtime
        .verify_successor_integrity()
        .map_err(scenario_error)?;
    Ok(BondScenario {
        name,
        runtime,
        bond,
        formation,
        polarity,
        successor: Some(successor),
    })
}

/// Exercises a cured default and a challenge denied by Sandmanor proof, then
/// re-evaluates and matures the unchanged historical chemistry.
pub fn run_default_challenge_scenario() -> Result<BondScenario, ScenarioError> {
    let name = "default-challenge";
    let world = world::institutional_access_fixture();
    let (mut runtime, bond, formation) = formed_bond(name, Sign::Positive, 10)?;
    let polarity = run_living_chemistry(
        &mut runtime,
        name,
        &bond,
        &formation,
        Sign::Positive,
        Sign::Negative,
    )?;
    let default = stable("default.default-challenge", DefaultId::new);
    runtime
        .append(
            bond.clone(),
            scenario_event_metadata("default-challenge.default", 8),
            BondEvent::DefaultDeclared(BondDefault {
                id: default.clone(),
                participant: formation.participants[0].id.clone(),
                obligation: formation.obligations[0].clone(),
                evidence: vec![scenario_evidence("default-challenge.default")],
            }),
        )
        .map_err(scenario_error)?;
    let challenge = stable("challenge.default-challenge", ChallengeId::new);
    runtime
        .append(
            bond.clone(),
            scenario_event_metadata("default-challenge.challenge", 8),
            BondEvent::ChallengeFiled(BondChallenge {
                id: challenge.clone(),
                challenger: formation.participants[0].id.clone(),
                challenged_evidence: scenario_evidence("default-challenge.current"),
                evidence: vec![scenario_evidence("default-challenge.challenge")],
            }),
        )
        .map_err(scenario_error)?;
    runtime
        .append(
            bond.clone(),
            scenario_event_metadata("default-challenge.default-resolved", 9),
            BondEvent::DefaultResolved(BondDefaultResolution {
                default,
                outcome: DefaultOutcome::Cured,
                evidence: vec![scenario_evidence("default-challenge.cure")],
            }),
        )
        .map_err(scenario_error)?;
    runtime
        .append(
            bond.clone(),
            scenario_event_metadata("default-challenge.challenge-resolved", 9),
            BondEvent::ChallengeResolved(BondChallengeResolution {
                challenge,
                outcome: ChallengeOutcome::Rejected,
                sandmanor_proof: scenario_house_decision(
                    &world.catalog,
                    "default-challenge.challenge-proof",
                    HouseFunction::Prove,
                    9,
                )?,
                evidence: vec![scenario_evidence("default-challenge.denial")],
            }),
        )
        .map_err(scenario_error)?;
    runtime
        .evaluate(
            &bond,
            scenario_event_metadata("default-challenge.reevaluate", 9),
            stable("evaluation.default-challenge.final", EvaluationId::new),
            vec![scenario_evidence("default-challenge.reevaluation")],
        )
        .map_err(scenario_error)?;
    runtime
        .append(
            bond.clone(),
            scenario_event_metadata("default-challenge.mature", 10),
            BondEvent::Matured(BondMaturity {
                trigger: MaturityTrigger::FiniteTermCompleted,
                evidence: vec![scenario_evidence("default-challenge.maturity")],
            }),
        )
        .map_err(scenario_error)?;
    Ok(BondScenario {
        name,
        runtime,
        bond,
        formation,
        polarity,
        successor: None,
    })
}

#[derive(Debug, Clone)]
pub struct RejectedBondScenario {
    pub name: &'static str,
    pub runtime: ConstitutionalRuntime,
    pub bond: BondId,
    pub error: ConstitutionalRuntimeError,
    pub failure_code: &'static str,
    pub event_count_before: usize,
    pub event_count_after: usize,
}

pub fn run_premature_maturity_scenario() -> Result<RejectedBondScenario, ScenarioError> {
    let name = "premature-maturity";
    let (mut runtime, bond, formation) = formed_bond(name, Sign::Positive, 10)?;
    run_living_chemistry(
        &mut runtime,
        name,
        &bond,
        &formation,
        Sign::Positive,
        Sign::Positive,
    )?;
    let event_count_before = runtime.events().len();
    let error = runtime
        .append(
            bond.clone(),
            scenario_event_metadata("premature-maturity.attempt", 9),
            BondEvent::Matured(BondMaturity {
                trigger: MaturityTrigger::FiniteTermCompleted,
                evidence: vec![scenario_evidence("premature-maturity.attempt")],
            }),
        )
        .expect_err("maturity before term must fail");
    let event_count_after = runtime.events().len();
    Ok(RejectedBondScenario {
        name,
        runtime,
        bond,
        error,
        failure_code: "BOND_PREMATURE_MATURITY",
        event_count_before,
        event_count_after,
    })
}

pub fn run_terminal_renewal_rejection() -> Result<RejectedBondScenario, ScenarioError> {
    let name = "renewal-after-terminal";
    let world = world::institutional_access_fixture();
    let mut scenario = run_ordinary_lifecycle()?;
    let event_count_before = scenario.runtime.events().len();
    let error = scenario
        .runtime
        .append(
            scenario.bond.clone(),
            scenario_event_metadata("renewal-after-terminal.attempt", 19),
            BondEvent::Resolved(BondResolution {
                id: stable("resolution.terminal-retry", ResolutionId::new),
                disposition: ResolutionDisposition::Renew,
                successor_bonds: vec![stable("bond.illegal-second-renewal", BondId::new)],
                glaushouse_resolution: scenario_house_decision(
                    &world.catalog,
                    "renewal-after-terminal.resolve",
                    HouseFunction::Resolve,
                    19,
                )?,
                evidence: vec![scenario_evidence("renewal-after-terminal")],
            }),
        )
        .expect_err("resolved Bond cannot resolve again");
    let event_count_after = scenario.runtime.events().len();
    Ok(RejectedBondScenario {
        name,
        runtime: scenario.runtime,
        bond: scenario.bond,
        error,
        failure_code: "BOND_TERMINAL_STATE",
        event_count_before,
        event_count_after,
    })
}

#[derive(Debug, Clone)]
pub struct RegionalScenario {
    pub name: &'static str,
    pub runtime: RegionalSynthesisRuntime,
    pub predecessor: RegionalBeingId,
    pub result: RegionalBeingId,
    pub synthesis: RegionalSynthesisId,
    pub retry_event_count: usize,
    pub archive: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct RejectedRegionalScenario {
    pub name: &'static str,
    pub runtime: RegionalSynthesisRuntime,
    pub predecessor: RegionalBeingId,
    pub attempted_result: RegionalBeingId,
    pub error: RegionalSynthesisError,
    pub event_count_before: usize,
    pub event_count_after: usize,
}

#[derive(Debug, Clone)]
pub struct RejectedRegionalAssignmentScenario {
    pub name: &'static str,
    pub runtime: RegionalSynthesisRuntime,
    pub being: RegionalBeingId,
    pub error: RegionalSynthesisError,
    pub event_count_before: usize,
    pub event_count_after: usize,
}

pub fn scenario_subject_evidence(subject: &RegionalBeingId, key: &str) -> SubjectEvidence {
    SubjectEvidence {
        subject: subject.clone(),
        reference: scenario_evidence(key),
    }
}

pub fn scenario_regional_registration(
    catalog: &InstitutionCatalog,
    id: RegionalBeingId,
    form: SandmanorForm,
    region: ConstitutionalRegion,
    kind: RegionalStandingKind,
) -> Result<RegionalBeingRegistration, ScenarioError> {
    Ok(RegionalBeingRegistration {
        standing: RegionalStanding {
            region,
            kind,
            jurisdiction: RegionalJurisdictionSnapshot::from_catalog(
                catalog,
                region,
                CausalPosition::new(1),
                vec![scenario_evidence(&format!("jurisdiction.{}", id.as_str()))],
            )
            .map_err(scenario_error)?,
            evidence: vec![scenario_subject_evidence(&id, "regional.standing")],
        },
        evidence: vec![scenario_subject_evidence(&id, "regional.registration")],
        id,
        form,
    })
}

pub fn scenario_regional_authority(
    catalog: &InstitutionCatalog,
    label: &str,
    at: u64,
) -> Result<RegionalSynthesisAuthority, ScenarioError> {
    Ok(RegionalSynthesisAuthority {
        sandmanor_proof: scenario_house_decision(
            catalog,
            &format!("{label}.regional-proof"),
            HouseFunction::Prove,
            at,
        )?,
        glaushouse_resolution: scenario_house_decision(
            catalog,
            &format!("{label}.regional-resolution"),
            HouseFunction::Resolve,
            at,
        )?,
    })
}

#[allow(clippy::too_many_arguments)]
pub fn scenario_regional_command(
    catalog: &InstitutionCatalog,
    label: &str,
    predecessor: &RegionalBeingId,
    result: RegionalBeingId,
    predecessor_form: SandmanorForm,
    result_form: SandmanorForm,
    region: ConstitutionalRegion,
    function: RegionalFunction,
) -> Result<RegionalSynthesisCommand, ScenarioError> {
    Ok(RegionalSynthesisCommand {
        id: stable(&format!("synthesis.{label}"), RegionalSynthesisId::new),
        predecessor: predecessor.clone(),
        result,
        expected_predecessor_form: predecessor_form,
        requested_result_form: result_form,
        requested_region: region,
        requested_function: function,
        prerequisites: RegionalSynthesisPrerequisites {
            standing: scenario_subject_evidence(
                predecessor,
                &format!("{label}.prerequisite.standing"),
            ),
            lineage: scenario_subject_evidence(
                predecessor,
                &format!("{label}.prerequisite.lineage"),
            ),
            readiness: scenario_subject_evidence(
                predecessor,
                &format!("{label}.prerequisite.readiness"),
            ),
            constitutional_rule: scenario_subject_evidence(
                predecessor,
                &format!("{label}.prerequisite.rule"),
            ),
            supporting: vec![scenario_subject_evidence(
                predecessor,
                &format!("{label}.prerequisite.supporting"),
            )],
        },
        authority: scenario_regional_authority(catalog, label, 2)?,
        evidence: vec![scenario_subject_evidence(
            predecessor,
            &format!("{label}.synthesis-evidence"),
        )],
    })
}

fn run_accepted_regional(
    name: &'static str,
    predecessor_form: SandmanorForm,
    result_form: SandmanorForm,
    region: ConstitutionalRegion,
    function: RegionalFunction,
) -> Result<RegionalScenario, ScenarioError> {
    let world = world::institutional_access_fixture();
    let predecessor = stable(&format!("being.{name}.origin"), RegionalBeingId::new);
    let result = stable(&format!("being.{name}.result"), RegionalBeingId::new);
    let mut runtime = RegionalSynthesisRuntime::new();
    runtime
        .register_being(
            scenario_regional_metadata(&format!("{name}.register"), 1),
            scenario_regional_registration(
                &world.catalog,
                predecessor.clone(),
                predecessor_form,
                region,
                RegionalStandingKind::Established,
            )?,
        )
        .map_err(scenario_error)?;
    let metadata = scenario_regional_metadata(&format!("{name}.synthesize"), 2);
    let command = scenario_regional_command(
        &world.catalog,
        name,
        &predecessor,
        result.clone(),
        predecessor_form,
        result_form,
        region,
        function,
    )?;
    let synthesis = command.id.clone();
    runtime
        .synthesize(metadata.clone(), command.clone())
        .map_err(scenario_error)?;
    let retry_event_count = runtime.events().len();
    runtime
        .synthesize(metadata, command)
        .map_err(scenario_error)?;
    if runtime.events().len() != retry_event_count {
        return Err(ScenarioError(
            "idempotent Synthesis retry duplicated history".into(),
        ));
    }
    let replayed = RegionalSynthesisRuntime::replay(runtime.events().iter().cloned())
        .map_err(scenario_error)?;
    if replayed != runtime {
        return Err(ScenarioError("regional live/replay divergence".into()));
    }
    let archive = encode_regional_archive(&runtime).map_err(scenario_error)?;
    let decoded = decode_regional_archive(&archive).map_err(scenario_error)?;
    if decoded != runtime || encode_regional_archive(&decoded).map_err(scenario_error)? != archive {
        return Err(ScenarioError(
            "regional persistence is not canonical".into(),
        ));
    }
    Ok(RegionalScenario {
        name,
        runtime,
        predecessor,
        result,
        synthesis,
        retry_event_count,
        archive,
    })
}

pub fn run_gnome_minotaur_scenario() -> Result<RegionalScenario, ScenarioError> {
    run_accepted_regional(
        "gnome-minotaur",
        SandmanorForm::Gnome,
        SandmanorForm::Minotaur,
        ConstitutionalRegion::AuraFields,
        RegionalFunction::AuraFieldsStewardshipAndDefense,
    )
}

pub fn run_elf_centaur_scenario() -> Result<RegionalScenario, ScenarioError> {
    run_accepted_regional(
        "elf-centaur",
        SandmanorForm::Elf,
        SandmanorForm::Centaur,
        ConstitutionalRegion::AuraBeach,
        RegionalFunction::AuraBeachPatrolAndAuraSeaGuardianship,
    )
}

fn rejected_regional(
    name: &'static str,
    predecessor_form: SandmanorForm,
    result_form: SandmanorForm,
    standing_region: ConstitutionalRegion,
    requested_region: ConstitutionalRegion,
    function: RegionalFunction,
    mutate: impl FnOnce(&RegionalBeingId, &mut RegionalSynthesisCommand),
) -> Result<RejectedRegionalScenario, ScenarioError> {
    let world = world::institutional_access_fixture();
    let predecessor = stable(&format!("being.{name}.origin"), RegionalBeingId::new);
    let attempted_result = stable(&format!("being.{name}.result"), RegionalBeingId::new);
    let mut runtime = RegionalSynthesisRuntime::new();
    runtime
        .register_being(
            scenario_regional_metadata(&format!("{name}.register"), 1),
            scenario_regional_registration(
                &world.catalog,
                predecessor.clone(),
                predecessor_form,
                standing_region,
                RegionalStandingKind::Established,
            )?,
        )
        .map_err(scenario_error)?;
    let mut command = scenario_regional_command(
        &world.catalog,
        name,
        &predecessor,
        attempted_result.clone(),
        predecessor_form,
        result_form,
        requested_region,
        function,
    )?;
    mutate(&predecessor, &mut command);
    let event_count_before = runtime.events().len();
    let error = runtime
        .synthesize(
            scenario_regional_metadata(&format!("{name}.synthesize"), 2),
            command,
        )
        .expect_err("rejected regional scenario must fail closed");
    let event_count_after = runtime.events().len();
    Ok(RejectedRegionalScenario {
        name,
        runtime,
        predecessor,
        attempted_result,
        error,
        event_count_before,
        event_count_after,
    })
}

pub fn run_rejected_regional_scenario(
    name: &str,
) -> Result<RejectedRegionalScenario, ScenarioError> {
    match name {
        "gnome-centaur" => rejected_regional(
            "gnome-centaur",
            SandmanorForm::Gnome,
            SandmanorForm::Centaur,
            ConstitutionalRegion::AuraFields,
            ConstitutionalRegion::AuraBeach,
            RegionalFunction::AuraBeachPatrolAndAuraSeaGuardianship,
            |_, _| {},
        ),
        "elf-minotaur" => rejected_regional(
            "elf-minotaur",
            SandmanorForm::Elf,
            SandmanorForm::Minotaur,
            ConstitutionalRegion::AuraBeach,
            ConstitutionalRegion::AuraFields,
            RegionalFunction::AuraFieldsStewardshipAndDefense,
            |_, _| {},
        ),
        "gnome-minotaur-wrong-region" => rejected_regional(
            "gnome-minotaur-wrong-region",
            SandmanorForm::Gnome,
            SandmanorForm::Minotaur,
            ConstitutionalRegion::AuraBeach,
            ConstitutionalRegion::AuraFields,
            RegionalFunction::AuraFieldsStewardshipAndDefense,
            |_, _| {},
        ),
        "elf-centaur-wrong-region" => rejected_regional(
            "elf-centaur-wrong-region",
            SandmanorForm::Elf,
            SandmanorForm::Centaur,
            ConstitutionalRegion::AuraFields,
            ConstitutionalRegion::AuraBeach,
            RegionalFunction::AuraBeachPatrolAndAuraSeaGuardianship,
            |_, _| {},
        ),
        "synthesis-without-authority" => rejected_regional(
            "synthesis-without-authority",
            SandmanorForm::Gnome,
            SandmanorForm::Minotaur,
            ConstitutionalRegion::AuraFields,
            ConstitutionalRegion::AuraFields,
            RegionalFunction::AuraFieldsStewardshipAndDefense,
            |_, command| {
                command.authority.glaushouse_resolution.outcome = HouseDecisionOutcome::Rejected;
            },
        ),
        "synthesis-without-evidence" => rejected_regional(
            "synthesis-without-evidence",
            SandmanorForm::Gnome,
            SandmanorForm::Minotaur,
            ConstitutionalRegion::AuraFields,
            ConstitutionalRegion::AuraFields,
            RegionalFunction::AuraFieldsStewardshipAndDefense,
            |_, command| command.evidence.clear(),
        ),
        "synthesis-mismatched-evidence" => rejected_regional(
            "synthesis-mismatched-evidence",
            SandmanorForm::Gnome,
            SandmanorForm::Minotaur,
            ConstitutionalRegion::AuraFields,
            ConstitutionalRegion::AuraFields,
            RegionalFunction::AuraFieldsStewardshipAndDefense,
            |_, command| {
                command.evidence[0].subject = stable("being.unrelated", RegionalBeingId::new);
            },
        ),
        _ => Err(ScenarioError(format!("unknown rejected scenario: {name}"))),
    }
}

pub fn run_rejected_assignment_scenario(
    name: &str,
) -> Result<RejectedRegionalAssignmentScenario, ScenarioError> {
    let (name, scenario, guardianship_claim) = match name {
        "minotaur-sea-claim" => ("minotaur-sea-claim", run_gnome_minotaur_scenario()?, true),
        "centaur-fields-claim" => ("centaur-fields-claim", run_elf_centaur_scenario()?, false),
        _ => return Err(ScenarioError(format!("unknown assignment claim: {name}"))),
    };
    let event_count_before = scenario.runtime.events().len();
    let error = if guardianship_claim {
        scenario
            .runtime
            .require_guardianship(&scenario.result)
            .expect_err("Minotaur must not hold Aura Sea guardianship")
    } else {
        scenario
            .runtime
            .require_stewardship(&scenario.result)
            .expect_err("Centaur must not hold Aura Field stewardship")
    };
    let event_count_after = scenario.runtime.events().len();
    Ok(RejectedRegionalAssignmentScenario {
        name,
        runtime: scenario.runtime,
        being: scenario.result,
        error,
        event_count_before,
        event_count_after,
    })
}

#[derive(Debug, Clone)]
pub struct KernelWaveScenario {
    pub runtime: ConstitutionalRuntime,
    pub wave: WaveId,
    pub constitutional_event_count: usize,
}

pub fn run_kernel_wave_scenario() -> Result<KernelWaveScenario, ScenarioError> {
    let pass = crate::run_kernel_cycle(crate::Symptom::origin());
    let mut runtime = ConstitutionalRuntime::new();
    let wave = stable("wave.kernel-demo", WaveId::new);
    record_kernel_wave(
        &mut runtime,
        wave.clone(),
        &stable("artifact.kernel-demo", ArtifactId::new),
        CausalPosition::new(0),
        &pass,
    )
    .map_err(scenario_error)?;
    Ok(KernelWaveScenario {
        constitutional_event_count: runtime.events().len(),
        runtime,
        wave,
    })
}
