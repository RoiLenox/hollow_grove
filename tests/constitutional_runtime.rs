use hollow_grove::composition::ExternalRef;
use hollow_grove::constitutional::*;
use hollow_grove::hollow_grove_contract::House;
use hollow_grove::institution::{InstitutionId, OfficeId};

fn id<T>(value: &str, build: impl FnOnce(String) -> Result<T, ConstitutionalIdError>) -> T {
    build(value.into()).expect("stable test ID")
}

fn evidence(key: &str) -> EvidenceRef {
    EvidenceRef(ExternalRef::new("test", key).expect("test evidence"))
}

fn authority(
    house: House,
    office: &str,
    actor: &str,
    authority: &str,
    at: u64,
) -> AuthoritySnapshot {
    AuthoritySnapshot {
        actor: id(actor, AuthorityActorId::new),
        office: OfficeId::new(office).unwrap(),
        institution: None,
        house,
        authorities: vec![authority.into()],
        observed_at: CausalPosition::new(at),
    }
}

fn decision(
    id_value: &str,
    function: HouseFunction,
    office: &str,
    actor: &str,
    at: u64,
) -> HouseDecision {
    HouseDecision {
        id: id(id_value, HouseDecisionId::new),
        function,
        authority: authority(
            function.constitutional_house(),
            office,
            actor,
            function.required_authority(),
            at,
        ),
        outcome: HouseDecisionOutcome::Accepted,
        evidence: vec![evidence(id_value)],
        causal_position: CausalPosition::new(at),
    }
}

fn metadata(number: u64) -> EventMetadata {
    EventMetadata {
        id: id(&format!("event.{number}"), ConstitutionalEventId::new),
        causal_position: CausalPosition::new(number),
        rule_set: id("rules.hollow-grove-v2", RuleSetId::new),
    }
}

fn participant(value: &str) -> BondParticipant {
    BondParticipant {
        id: id(value, ParticipantId::new),
        kind: ParticipantKind::Huemen,
        roles: vec![id("role.owner", RoleId::new)],
    }
}

fn formed_runtime(
    current_sign: Sign,
    current_magnitude: u128,
) -> (ConstitutionalRuntime, BondId, BondFormation) {
    let mut runtime = ConstitutionalRuntime::new();
    let wave_id = id("wave.origin", WaveId::new);
    runtime
        .record_wave(WaveRecord {
            id: wave_id.clone(),
            origin: evidence("wave-origin"),
            causal_position: CausalPosition::new(0),
        })
        .unwrap();
    let bond_id = id("bond.integration", BondId::new);
    let owner = participant("being.owner");
    let current_unit = id("unit.current", UnitId::new);
    let formation = BondFormation {
        id: bond_id.clone(),
        initiating_wave: wave_id,
        governing_house: House::Stonebend,
        governing_institution: InstitutionId::new("institution.stonebend.constitution").unwrap(),
        jurisdiction: InstitutionalJurisdictionSnapshot {
            institution: InstitutionId::new("institution.stonebend.constitution").unwrap(),
            house: House::Stonebend,
            observed_at: CausalPosition::new(1),
            evidence: vec![evidence("jurisdiction")],
        },
        parent_bonds: vec![],
        inheritance_evidence: vec![],
        participants: vec![owner.clone()],
        obligations: vec![id("obligation.finish", ObligationId::new)],
        permissions: vec![id("permission.circulate", PermissionId::new)],
        term: BondTerm::Finite {
            end: CausalPosition::new(10),
        },
        current_unit: current_unit.clone(),
        aura_unit: id("unit.aura", UnitId::new),
        starting_current: vec![InitialCurrent {
            owner: owner.id.clone(),
            custodian: owner.id.clone(),
            quantity: SignedQuantity::new(current_sign, current_magnitude, current_unit).unwrap(),
            evidence: vec![evidence("starting-current")],
        }],
        initial_aura: vec![],
        evidence: vec![evidence("formation")],
        stonebend_naming: decision(
            "decision.stonebend.name",
            HouseFunction::Name,
            "office.stonebend.hypergiant",
            "being.hypergiant",
            1,
        ),
    };
    runtime
        .append(
            bond_id.clone(),
            metadata(1),
            BondEvent::Formed(formation.clone()),
        )
        .unwrap();
    runtime
        .append(
            bond_id.clone(),
            metadata(2),
            BondEvent::Validated(BondValidation {
                sandmanor_proof: decision(
                    "decision.sandmanor.prove",
                    HouseFunction::Prove,
                    "office.sandmanor.sandman",
                    "being.sandman",
                    2,
                ),
                evidence: vec![evidence("validation")],
            }),
        )
        .unwrap();
    runtime
        .append(
            bond_id.clone(),
            metadata(3),
            BondEvent::Activated(BondActivation {
                evidence: vec![evidence("activation")],
            }),
        )
        .unwrap();
    (runtime, bond_id, formation)
}

fn complete_living_history(
    runtime: &mut ConstitutionalRuntime,
    bond: &BondId,
    formation: &BondFormation,
    entering_sign: Sign,
    entering_magnitude: u128,
    aura_sign: Sign,
    aura_magnitude: u128,
) {
    runtime
        .append(
            bond.clone(),
            metadata(4),
            BondEvent::CurrentMoved(CurrentTransaction {
                id: id("transaction.entry", CurrentTransactionId::new),
                wave: formation.initiating_wave.clone(),
                operation: CurrentOperation::Enter,
                edges: vec![CurrentEdge {
                    source: CurrentAccount::External("world-boundary".into()),
                    destination: CurrentAccount::Participant(formation.participants[0].id.clone()),
                    quantity: SignedQuantity::new(
                        entering_sign,
                        entering_magnitude,
                        formation.current_unit.clone(),
                    )
                    .unwrap(),
                }],
                evidence: vec![evidence("current-entry")],
            }),
        )
        .unwrap();
    runtime.accumulate_current(bond, metadata(5)).unwrap();
    runtime
        .append(
            bond.clone(),
            metadata(6),
            BondEvent::AuraObserved(AuraObservation {
                id: id("observation.aura", AuraObservationId::new),
                observer: formation.participants[0].id.clone(),
                quantity: SignedQuantity::new(
                    aura_sign,
                    aura_magnitude,
                    formation.aura_unit.clone(),
                )
                .unwrap(),
                subject: evidence("current-entry"),
                evidence: vec![evidence("aura-observation")],
            }),
        )
        .unwrap();
    runtime
        .evaluate(
            bond,
            metadata(7),
            id("evaluation.final", EvaluationId::new),
            vec![evidence("evaluation")],
        )
        .unwrap();
}

#[test]
fn positive_current_bonds_to_negative_aura_and_replays_identically() {
    let (mut runtime, bond, formation) = formed_runtime(Sign::Positive, 10);
    complete_living_history(
        &mut runtime,
        &bond,
        &formation,
        Sign::Negative,
        3,
        Sign::Negative,
        4,
    );
    let aggregate = runtime.bond(&bond).unwrap();
    let evaluation = aggregate
        .calculated_evaluation(
            id("evaluation.check", EvaluationId::new),
            vec![evidence("check")],
        )
        .unwrap();
    assert_eq!(
        evaluation.polarity,
        ConstitutionalPolarity::PositiveCurrentNegativeAura
    );

    let replayed =
        ConstitutionalRuntime::replay(runtime.waves().cloned(), runtime.events().iter().cloned())
            .unwrap();
    assert_eq!(replayed.events(), runtime.events());
    assert_eq!(replayed.bond(&bond), runtime.bond(&bond));
}

#[test]
fn negative_current_bonds_to_positive_aura() {
    let (mut runtime, bond, formation) = formed_runtime(Sign::Negative, 12);
    complete_living_history(
        &mut runtime,
        &bond,
        &formation,
        Sign::Positive,
        2,
        Sign::Positive,
        5,
    );
    let evaluation = runtime
        .bond(&bond)
        .unwrap()
        .calculated_evaluation(
            id("evaluation.check", EvaluationId::new),
            vec![evidence("check")],
        )
        .unwrap();
    assert_eq!(
        evaluation.polarity,
        ConstitutionalPolarity::NegativeCurrentPositiveAura
    );
}

#[test]
fn full_proof_lifecycle_requires_all_four_house_functions() {
    let (mut runtime, bond, formation) = formed_runtime(Sign::Positive, 10);
    complete_living_history(
        &mut runtime,
        &bond,
        &formation,
        Sign::Negative,
        3,
        Sign::Negative,
        4,
    );
    runtime
        .append(
            bond.clone(),
            metadata(10),
            BondEvent::Matured(BondMaturity {
                trigger: MaturityTrigger::FiniteTermCompleted,
                evidence: vec![evidence("term-completed")],
            }),
        )
        .unwrap();
    runtime.calculate_excess(&bond, metadata(11)).unwrap();
    runtime
        .append(
            bond.clone(),
            metadata(12),
            BondEvent::CondensationDecided(CondensationDecision {
                status: CondensationStatus::Eligible,
                glaushouse_clearance: Some(decision(
                    "decision.glaushouse.clear",
                    HouseFunction::Clear,
                    "office.glaushouse.prima-donna",
                    "being.prima-donna",
                    12,
                )),
                evidence: vec![evidence("eligibility")],
            }),
        )
        .unwrap();
    let tombstone_id = id("tombstone.integration", TombstoneId::new);
    runtime
        .append(
            bond.clone(),
            metadata(13),
            BondEvent::TombstoneFormed(Tombstone {
                id: tombstone_id.clone(),
                source_bond: bond.clone(),
                governing_house: formation.governing_house,
                governing_institution: formation.governing_institution.clone(),
                participants: formation.participants.clone(),
                constitutional_excess: runtime.bond(&bond).unwrap().calculated_excess().unwrap(),
                polarity: ConstitutionalPolarity::PositiveCurrentNegativeAura,
                completed_obligations: formation.obligations.clone(),
                remaining_obligations: vec![],
                evidence: vec![evidence("tombstone")],
            }),
        )
        .unwrap();
    let tombstone_replay_digest = constitutional_bond_replay_digest(&runtime, &bond).unwrap();
    let invalid_validation = runtime.append(
        bond.clone(),
        metadata(14),
        BondEvent::TombstoneValidated(TombstoneValidation {
            validator: id("being.independent-validator", AuthorityActorId::new),
            validation_basis: evidence("independent-validator"),
            replay_digest: "fnv1a64-bond-v1:0000000000000000".into(),
            evidence: vec![evidence("validation-evidence")],
        }),
    );
    assert!(matches!(
        invalid_validation,
        Err(ConstitutionalRuntimeError::ReplayDigestMismatch { .. })
    ));
    runtime
        .append(
            bond.clone(),
            metadata(14),
            BondEvent::TombstoneValidated(TombstoneValidation {
                validator: id("being.independent-validator", AuthorityActorId::new),
                validation_basis: evidence("independent-validator"),
                replay_digest: tombstone_replay_digest,
                evidence: vec![evidence("validation-evidence")],
            }),
        )
        .unwrap();

    let premature_toke = runtime.append(
        bond.clone(),
        metadata(15),
        BondEvent::TokeRecorded(Toke {
            id: id("toke.integration", TokeId::new),
            tombstone: tombstone_id.clone(),
            index_key: "history/integration".into(),
            evidence: vec![evidence("toke")],
        }),
    );
    assert!(matches!(
        premature_toke,
        Err(ConstitutionalRuntimeError::Bond(
            BondStateError::FlyntRecognitionRequired
        ))
    ));

    runtime
        .append(
            bond.clone(),
            metadata(15),
            BondEvent::FlyntRecognized(decision(
                "decision.flynt.recognize",
                HouseFunction::Recognize,
                "office.flynt.tross",
                "being.tross",
                15,
            )),
        )
        .unwrap();
    let toke_id = id("toke.integration", TokeId::new);
    runtime
        .append(
            bond.clone(),
            metadata(16),
            BondEvent::TokeRecorded(Toke {
                id: toke_id.clone(),
                tombstone: tombstone_id.clone(),
                index_key: "history/integration".into(),
                evidence: vec![evidence("toke")],
            }),
        )
        .unwrap();
    let successor_id = id("bond.integration-renewal", BondId::new);
    runtime
        .append(
            bond.clone(),
            metadata(17),
            BondEvent::Resolved(BondResolution {
                id: id("resolution.integration", ResolutionId::new),
                disposition: ResolutionDisposition::Renew,
                successor_bonds: vec![successor_id.clone()],
                glaushouse_resolution: decision(
                    "decision.glaushouse.resolve",
                    HouseFunction::Resolve,
                    "office.glaushouse.prima-donna",
                    "being.prima-donna",
                    17,
                ),
                evidence: vec![evidence("resolution")],
            }),
        )
        .unwrap();

    assert_eq!(runtime.bond(&bond).unwrap().phase(), BondPhase::Resolved);
    assert_eq!(runtime.tombstone_bond(&tombstone_id), Some(&bond));
    assert_eq!(runtime.toke_tombstone(&toke_id), Some(&tombstone_id));
    assert!(matches!(
        runtime.verify_successor_integrity(),
        Err(ConstitutionalRuntimeError::MissingReservedSuccessor { .. })
    ));

    let successor_wave = id("wave.successor", WaveId::new);
    runtime
        .record_wave(WaveRecord {
            id: successor_wave.clone(),
            origin: evidence("successor-wave"),
            causal_position: CausalPosition::new(17),
        })
        .unwrap();
    let mut successor_formation = formation.clone();
    successor_formation.id = successor_id.clone();
    successor_formation.initiating_wave = successor_wave;
    successor_formation.parent_bonds = vec![bond.clone()];
    successor_formation.inheritance_evidence = vec![evidence("renewal-inheritance")];
    successor_formation.term = BondTerm::Finite {
        end: CausalPosition::new(30),
    };
    successor_formation.stonebend_naming = decision(
        "decision.stonebend.name-renewal",
        HouseFunction::Name,
        "office.stonebend.hypergiant",
        "being.hypergiant",
        18,
    );
    successor_formation.jurisdiction.observed_at = CausalPosition::new(18);
    runtime
        .append(
            successor_id.clone(),
            metadata(18),
            BondEvent::Formed(successor_formation),
        )
        .unwrap();
    runtime.verify_successor_integrity().unwrap();
    assert_eq!(
        runtime.bond(&successor_id).unwrap().phase(),
        BondPhase::Formed
    );
    let archived = encode_constitutional_archive(&runtime).unwrap();
    let replayed = decode_constitutional_archive(&archived).unwrap();
    assert_eq!(replayed.events(), runtime.events());
    assert_eq!(replayed.bond(&bond), runtime.bond(&bond));
}

#[test]
fn reserved_house_law_is_rejected_instead_of_invented() {
    assert_eq!(
        invoke_reserved_procedure(ReservedHouseProcedure::SandmanorContestSuccession),
        Err(HouseLawError::ReservedProcedure(
            ReservedHouseProcedure::SandmanorContestSuccession
        ))
    );
}

#[test]
fn canonical_archive_round_trips_through_replay() {
    let (mut runtime, bond, formation) = formed_runtime(Sign::Positive, 10);
    complete_living_history(
        &mut runtime,
        &bond,
        &formation,
        Sign::Negative,
        3,
        Sign::Negative,
        4,
    );
    let encoded = encode_constitutional_archive(&runtime).unwrap();
    let decoded = decode_constitutional_archive(&encoded).unwrap();
    assert_eq!(decoded.events(), runtime.events());
    assert_eq!(decoded.bond(&bond), runtime.bond(&bond));
    assert_eq!(
        encode_constitutional_archive(&decoded).unwrap(),
        encoded,
        "canonical re-encoding must be byte-identical"
    );
    assert_eq!(
        constitutional_replay_digest(&decoded).unwrap(),
        constitutional_replay_digest(&runtime).unwrap()
    );
    assert_eq!(migrate_constitutional_archive(&encoded).unwrap(), encoded);
}

#[test]
fn archive_rejects_unknown_schema_instead_of_guessing_a_migration() {
    let (runtime, _, _) = formed_runtime(Sign::Positive, 10);
    let mut encoded = encode_constitutional_archive(&runtime).unwrap();
    encoded[8..10].copy_from_slice(&999_u16.to_le_bytes());
    assert!(matches!(
        decode_constitutional_archive(&encoded),
        Err(ConstitutionalArchiveError::UnsupportedVersion(999))
    ));
}

#[test]
fn command_retry_is_idempotent_but_identity_conflicts_are_rejected() {
    let (mut runtime, bond, formation) = formed_runtime(Sign::Positive, 10);
    let event_count = runtime.events().len();
    let retried = runtime
        .append(bond.clone(), metadata(1), BondEvent::Formed(formation))
        .unwrap();
    assert_eq!(retried.sequence, 0);
    assert_eq!(runtime.events().len(), event_count);

    let wave = runtime
        .wave(&id("wave.origin", WaveId::new))
        .unwrap()
        .clone();
    runtime.record_wave(wave).unwrap();

    let conflict = runtime.append(
        bond,
        EventMetadata {
            id: id("event.1", ConstitutionalEventId::new),
            causal_position: CausalPosition::new(2),
            rule_set: id("rules.hollow-grove-v2", RuleSetId::new),
        },
        BondEvent::Activated(BondActivation {
            evidence: vec![evidence("conflict")],
        }),
    );
    assert!(matches!(
        conflict,
        Err(ConstitutionalRuntimeError::EventIdConflict(_))
    ));
}

#[test]
fn house_decisions_are_derived_from_the_existing_institution_catalog() {
    let world = hollow_grove::world::institutional_access_fixture();
    let functions = [
        (HouseFunction::Name, "office.stonebend.hypergiant"),
        (HouseFunction::Prove, "office.sandmanor.sandman"),
        (HouseFunction::Clear, "office.glaushouse.prima-donna"),
        (HouseFunction::Resolve, "office.glaushouse.prima-donna"),
        (HouseFunction::Recognize, "office.flynt.tross"),
    ];
    for (index, (function, office_value)) in functions.into_iter().enumerate() {
        let office = OfficeId::new(office_value).unwrap();
        let holder = world
            .catalog
            .office_holders
            .iter()
            .find(|holder| holder.active && holder.office == office)
            .expect("every integration authority must have an active fixture holder");
        let decision = HouseDecision::from_catalog(
            &world.catalog,
            HouseDecisionDraft {
                id: id(&format!("decision.catalog.{index}"), HouseDecisionId::new),
                function,
                office,
                actor: id(holder.being.as_str(), AuthorityActorId::new),
                outcome: HouseDecisionOutcome::Accepted,
                evidence: vec![evidence("catalog-authority")],
                causal_position: CausalPosition::new(1),
            },
        )
        .unwrap();
        assert_eq!(decision.authority.house, function.constitutional_house());
        assert!(decision.authority.grants(function.required_authority()));
    }

    let inactive = HouseDecision::from_catalog(
        &world.catalog,
        HouseDecisionDraft {
            id: id("decision.stonebend.impostor", HouseDecisionId::new),
            function: HouseFunction::Name,
            office: OfficeId::new("office.stonebend.hypergiant").unwrap(),
            actor: id("being.not-the-holder", AuthorityActorId::new),
            outcome: HouseDecisionOutcome::Accepted,
            evidence: vec![evidence("impostor")],
            causal_position: CausalPosition::new(1),
        },
    );
    assert!(matches!(
        inactive,
        Err(HouseLawError::InactiveAuthorityActor(_))
    ));
}

#[test]
fn completed_kernel_pass_records_a_wave_without_moving_current() {
    let pass = hollow_grove::run_kernel_cycle(hollow_grove::Symptom::origin());
    let mut runtime = ConstitutionalRuntime::new();
    let wave = id("wave.kernel-pass", WaveId::new);
    record_kernel_wave(
        &mut runtime,
        wave.clone(),
        &id("artifact.kernel-pass", ArtifactId::new),
        CausalPosition::new(0),
        &pass,
    )
    .unwrap();
    assert!(runtime.wave(&wave).is_some());
    assert!(runtime.events().is_empty());
}

#[test]
fn pending_challenge_blocks_maturity_and_resolution_requires_sandmanor_proof() {
    let (mut runtime, bond, formation) = formed_runtime(Sign::Positive, 10);
    complete_living_history(
        &mut runtime,
        &bond,
        &formation,
        Sign::Negative,
        3,
        Sign::Negative,
        4,
    );
    let challenge = id("challenge.accounting", ChallengeId::new);
    runtime
        .append(
            bond.clone(),
            metadata(8),
            BondEvent::ChallengeFiled(BondChallenge {
                id: challenge.clone(),
                challenger: formation.participants[0].id.clone(),
                challenged_evidence: evidence("current-entry"),
                evidence: vec![evidence("challenge-filing")],
            }),
        )
        .unwrap();
    let maturity = runtime.append(
        bond.clone(),
        metadata(10),
        BondEvent::Matured(BondMaturity {
            trigger: MaturityTrigger::FiniteTermCompleted,
            evidence: vec![evidence("term-completed")],
        }),
    );
    assert!(matches!(
        maturity,
        Err(ConstitutionalRuntimeError::Bond(
            BondStateError::EvaluationRequired | BondStateError::PendingChallenge
        ))
    ));
    runtime
        .append(
            bond.clone(),
            metadata(9),
            BondEvent::ChallengeResolved(BondChallengeResolution {
                challenge: challenge.clone(),
                outcome: ChallengeOutcome::Clarified,
                sandmanor_proof: decision(
                    "decision.sandmanor.challenge-proof",
                    HouseFunction::Prove,
                    "office.sandmanor.sandman",
                    "being.sandman",
                    9,
                ),
                evidence: vec![evidence("challenge-resolution")],
            }),
        )
        .unwrap();
    runtime
        .evaluate(
            &bond,
            EventMetadata {
                id: id("event.9-evaluation", ConstitutionalEventId::new),
                causal_position: CausalPosition::new(9),
                rule_set: id("rules.hollow-grove-v2", RuleSetId::new),
            },
            id("evaluation.after-challenge", EvaluationId::new),
            vec![evidence("evaluation-after-challenge")],
        )
        .unwrap();
    runtime
        .append(
            bond.clone(),
            metadata(10),
            BondEvent::Matured(BondMaturity {
                trigger: MaturityTrigger::FiniteTermCompleted,
                evidence: vec![evidence("term-completed")],
            }),
        )
        .unwrap();
    assert_eq!(runtime.bond(&bond).unwrap().phase(), BondPhase::Mature);
    assert!(matches!(
        appeal_challenge(&challenge),
        Err(HouseLawError::ReservedProcedure(
            ReservedHouseProcedure::HouseAppealCourt
        ))
    ));
    let archived = encode_constitutional_archive(&runtime).unwrap();
    assert_eq!(
        decode_constitutional_archive(&archived).unwrap().events(),
        runtime.events()
    );
}

#[test]
fn current_and_aura_cannot_change_after_the_finite_term_boundary() {
    let (mut runtime, bond, formation) = formed_runtime(Sign::Positive, 10);
    let error = runtime
        .append(
            bond,
            metadata(10),
            BondEvent::CurrentMoved(CurrentTransaction {
                id: id("transaction.late", CurrentTransactionId::new),
                wave: formation.initiating_wave,
                operation: CurrentOperation::Enter,
                edges: vec![CurrentEdge {
                    source: CurrentAccount::External("world-boundary".into()),
                    destination: CurrentAccount::Participant(formation.participants[0].id.clone()),
                    quantity: SignedQuantity::new(Sign::Positive, 1, formation.current_unit)
                        .unwrap(),
                }],
                evidence: vec![evidence("late-current")],
            }),
        )
        .unwrap_err();
    assert!(matches!(
        error,
        ConstitutionalRuntimeError::Bond(BondStateError::CurrentOutsideActiveTerm)
    ));
}

#[test]
fn confirmed_default_remains_visible_in_the_tombstone() {
    let (mut runtime, bond, formation) = formed_runtime(Sign::Positive, 10);
    complete_living_history(
        &mut runtime,
        &bond,
        &formation,
        Sign::Negative,
        3,
        Sign::Negative,
        4,
    );
    let default_id = id("default.finish", DefaultId::new);
    runtime
        .append(
            bond.clone(),
            metadata(8),
            BondEvent::DefaultDeclared(BondDefault {
                id: default_id.clone(),
                participant: formation.participants[0].id.clone(),
                obligation: formation.obligations[0].clone(),
                evidence: vec![evidence("default-declared")],
            }),
        )
        .unwrap();
    runtime
        .append(
            bond.clone(),
            metadata(9),
            BondEvent::DefaultResolved(BondDefaultResolution {
                default: default_id,
                outcome: DefaultOutcome::Confirmed,
                evidence: vec![evidence("default-confirmed")],
            }),
        )
        .unwrap();
    runtime
        .evaluate(
            &bond,
            EventMetadata {
                id: id("event.9-evaluation", ConstitutionalEventId::new),
                causal_position: CausalPosition::new(9),
                rule_set: id("rules.hollow-grove-v2", RuleSetId::new),
            },
            id("evaluation.after-default", EvaluationId::new),
            vec![evidence("evaluation-after-default")],
        )
        .unwrap();
    runtime
        .append(
            bond.clone(),
            metadata(10),
            BondEvent::Matured(BondMaturity {
                trigger: MaturityTrigger::FiniteTermCompleted,
                evidence: vec![evidence("term-completed")],
            }),
        )
        .unwrap();
    runtime.calculate_excess(&bond, metadata(11)).unwrap();
    runtime
        .append(
            bond.clone(),
            metadata(12),
            BondEvent::CondensationDecided(CondensationDecision {
                status: CondensationStatus::Eligible,
                glaushouse_clearance: Some(decision(
                    "decision.glaushouse.clear-default",
                    HouseFunction::Clear,
                    "office.glaushouse.prima-donna",
                    "being.prima-donna",
                    12,
                )),
                evidence: vec![evidence("default-eligibility")],
            }),
        )
        .unwrap();
    let tombstone = Tombstone {
        id: id("tombstone.default", TombstoneId::new),
        source_bond: bond.clone(),
        governing_house: formation.governing_house,
        governing_institution: formation.governing_institution,
        participants: formation.participants,
        constitutional_excess: runtime.bond(&bond).unwrap().calculated_excess().unwrap(),
        polarity: ConstitutionalPolarity::PositiveCurrentNegativeAura,
        completed_obligations: vec![],
        remaining_obligations: formation.obligations,
        evidence: vec![evidence("default-tombstone")],
    };
    runtime
        .append(
            bond,
            metadata(13),
            BondEvent::TombstoneFormed(tombstone.clone()),
        )
        .unwrap();
    let archived = encode_constitutional_archive(&runtime).unwrap();
    let decoded = decode_constitutional_archive(&archived).unwrap();
    assert!(decoded.events().iter().any(|event| {
        matches!(
            &event.payload,
            BondEvent::TombstoneFormed(recorded) if recorded == &tombstone
        )
    }));
}

#[test]
fn no_stage_can_be_skipped() {
    let (mut runtime, bond, _) = formed_runtime(Sign::Positive, 10);
    let error = runtime
        .append(
            bond,
            metadata(10),
            BondEvent::Matured(BondMaturity {
                trigger: MaturityTrigger::FiniteTermCompleted,
                evidence: vec![evidence("term-completed")],
            }),
        )
        .unwrap_err();
    assert!(matches!(
        error,
        ConstitutionalRuntimeError::Bond(BondStateError::AccumulationRequired)
    ));
}
