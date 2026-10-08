use hollow_grove::world::lore_handoffs::{
    Action as A, Actors, Capacity, Consequence as C, Evidence, Facts, Function as F,
    HandoffService, Phase, ROUTES, Route,
};
use hollow_grove::world::session::WorldSession;

fn actors() -> Actors {
    Actors {
        executor: "being.backend-fixture.executor".into(),
        witness: "being.backend-fixture.witness".into(),
        recipient: "being.backend-fixture.recipient".into(),
        recovery_owner: "being.backend-fixture.executor".into(),
    }
}
fn world() -> WorldSession {
    WorldSession::from_institutional_state(hollow_grove::world::institutional_access_fixture())
        .unwrap()
}
fn app(route: Route) -> HandoffService {
    let subject = match route {
        Route::Basin => "object.backend-fixture.cargo",
        Route::PublicDesign => "object.backend-fixture.shelter",
        _ => "being.backend-fixture.recipient",
    };
    HandoffService::new(route, subject.into(), actors(), world()).unwrap()
}
fn evidence(route: Route, revision: u32) -> Evidence {
    let facts = match route {
        Route::Stairway => Facts::Stairway {
            work: vec!["completed access repair".into()],
            contributions: vec!["independent worker fitted the rail".into()],
            scope: "rail maintenance on this stairway".into(),
        },
        Route::Basin => Facts::Basin {
            cargo: "object.backend-fixture.cargo".into(),
            custodian: "being.backend-fixture.executor".into(),
            urban_scope: "urban delivery".into(),
            rural_scope: "rural access".into(),
            contraband_uncertainty: "review is pending".into(),
        },
        Route::PublicName => Facts::PublicName {
            former_names: vec!["Earlier Civic Reference".into()],
            requested_name: "Requested Civic Reference".into(),
            liabilities: vec!["earlier finite repair obligation".into()],
        },
        Route::Hollowing => Facts::Hollowing {
            sample: "object.backend-fixture.sample".into(),
            subject_custodian: "being.backend-fixture.executor".into(),
            sample_custodian: "being.backend-fixture.witness".into(),
            missing_custody_history: "initial extraction custody absent".into(),
            restoration_uncertainty: "restoration feasibility unproven".into(),
        },
        Route::PublicDesign => Facts::PublicDesign {
            design: "object.backend-fixture.shelter".into(),
            interior_result: "usable within tested load".into(),
            exterior_result: "tested approach remains accessible".into(),
            passed: true,
            maintenance_uncertainty: "seasonal burden estimated".into(),
        },
        Route::RecoveryDesign => Facts::RecoveryDesign {
            method: "method.backend-fixture.mobility".into(),
            lab_result: "works in laboratory".into(),
            moving_failure: "failed on moving route".into(),
            revised_moving_result: (revision >= 2)
                .then(|| "revised movement works in tested conditions".into()),
            revised_moving_passed: revision >= 2,
            capacity: Capacity::Capable,
        },
        Route::CoastalProof => Facts::CoastalProof {
            original_body: "body.backend-fixture.original".into(),
            reproducing_body: "body.backend-fixture.independent".into(),
            reproduced: true,
            safer_within_conditions: true,
            result: "rescue result independently repeated".into(),
        },
        Route::EmergencyIntake => Facts::EmergencyIntake {
            capacity: Capacity::Capable,
            immediate_stabilization_required: true,
            urgent_threat: "recorded immediate instability".into(),
            stabilization_scope: "bounded stabilization only".into(),
        },
        Route::HighRiskClearance => Facts::HighRiskClearance {
            capacity: Capacity::Capable,
            proven_claim: "scoped design evidence".into(),
            proof_supported: true,
            reversal_uncertainty: "reversal may be incomplete".into(),
            conditions: vec![
                "clearance disposition only; procedure separately requires review".into(),
            ],
        },
        Route::RecoveryDischarge => Facts::RecoveryDischarge {
            capacity: Capacity::Capable,
            travel_result: "limited travel demonstrated".into(),
            travel_proven_within_limits: true,
            duty_limits: vec!["light duty and rest only".into()],
            full_load_uncertainty: "full load endurance unknown".into(),
            recovery_plan: "follow-up and supported return".into(),
        },
    };
    Evidence {
        revision,
        references: vec![format!("observation.backend-fixture.{route:?}.{revision}")],
        tested_conditions: vec!["disclosed conditions of this witnessed trial".into()],
        negative_results: vec!["earlier failed attempt remains attributable".into()],
        facts,
    }
}
fn step(s: &mut HandoffService, action: A) {
    let at = s.events().len() as u64 + 1;
    s.execute(format!("event.backend-fixture.{at}"), at, action)
        .unwrap();
}
fn reject(s: &mut HandoffService, action: A) {
    let before = s.encode().unwrap();
    let at = s.events().len() as u64 + 1;
    assert!(
        s.execute(format!("event.backend-fixture.{at}"), at, action)
            .is_err()
    );
    assert_eq!(s.encode().unwrap(), before);
}
fn primary(route: Route) -> F {
    match route {
        Route::Stairway | Route::Basin => F::Recognize,
        Route::PublicName | Route::Hollowing => F::Name,
        Route::PublicDesign | Route::RecoveryDesign | Route::CoastalProof => F::Prove,
        _ => F::Clear,
    }
}
fn decision(s: &mut HandoffService, function: F) {
    let office = match function {
        F::Name => "office.stonebend.hypergiant",
        F::Prove => "office.sandmanor.sandman",
        F::Clear => "office.glaushouse.prima-donna",
        F::Recognize => "office.flynt.tross",
    };
    let actor = world()
        .institutional()
        .catalog
        .office_holders
        .iter()
        .find(|h| h.office.as_str() == office && h.active)
        .unwrap()
        .being
        .as_str()
        .to_owned();
    step(s, A::Decide { actor, function });
}
fn choose(s: &mut HandoffService, alternative: u8) {
    let route = s.route();
    step(
        s,
        A::RecordEvidence {
            actor: actors().witness,
            evidence: evidence(route, 1),
        },
    );
    if route == Route::RecoveryDesign {
        step(
            s,
            A::RecordEvidence {
                actor: actors().witness,
                evidence: evidence(route, 2),
            },
        );
    }
    step(
        s,
        A::Choose {
            actor: actors().recipient,
            alternative,
        },
    );
}
fn authorize(s: &mut HandoffService) {
    match (s.route(), s.state().alternative.unwrap()) {
        (Route::RecoveryDesign, 0 | 1) => decision(s, F::Clear),
        (Route::HighRiskClearance, 0) => decision(s, F::Prove),
        _ => {}
    }
    decision(s, primary(s.route()));
}
fn delivery(s: &mut HandoffService) {
    step(
        s,
        A::Execute {
            actor: actors().executor,
        },
    );
    step(
        s,
        A::Observe {
            actor: actors().witness,
            accepts_scope: true,
        },
    );
    step(
        s,
        A::Deliver {
            actor: actors().recipient,
        },
    );
}
fn verify_route(route: Route) {
    for alternative in 0..3 {
        let mut s = app(route);
        choose(&mut s, alternative);
        authorize(&mut s);
        delivery(&mut s);
        assert!(s.complete());
        let outcome = s.state().outcome.as_ref().unwrap();
        assert_eq!(
            outcome.alternative.as_deref(),
            Some(route.definition().player_choices[alternative as usize])
        );
        assert!(!outcome.grants_form && !outcome.grants_office && !outcome.grants_affiliation);
        assert_eq!(
            s.state().delivery_recipient.as_deref(),
            Some(actors().recipient.as_str())
        );
        let before = s.encode().unwrap();
        let decoded = HandoffService::decode(&before).unwrap();
        assert_eq!(decoded.state(), s.state());
        assert_eq!(decoded.encode().unwrap(), before);
        let report = hollow_grove::architecture_audit::ArchitectureCoverageReport::inspect(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")),
            None,
        )
        .unwrap()
        .with_handoff(&decoded);
        assert!(report.source_links_passed, "{:?}", report.violations);
        assert_eq!(report.unbound_canonical_route_count, 0);
        assert_eq!(
            report
                .canonical_route_contracts
                .iter()
                .filter(|c| c.execution_complete_in_supplied_handoff == Some(true))
                .count(),
            1
        );
        if let Ok(directory) = std::env::var("HG_HANDOFF_FIXTURE_DIR") {
            std::fs::create_dir_all(&directory).unwrap();
            std::fs::write(
                std::path::Path::new(&directory).join(format!("{route:?}-{alternative}.json")),
                before,
            )
            .unwrap();
        }
    }
}
macro_rules! route_test {
    ($name:ident,$route:ident) => {
        #[test]
        fn $name() {
            verify_route(Route::$route);
        }
    };
}
route_test!(stairway_all_three_scoped_dispositions, Stairway);
route_test!(basin_all_three_scoped_dispositions, Basin);
route_test!(public_name_all_three_scoped_dispositions, PublicName);
route_test!(hollowing_all_three_scoped_dispositions, Hollowing);
route_test!(public_design_all_three_scoped_dispositions, PublicDesign);
route_test!(
    recovery_design_all_three_scoped_dispositions,
    RecoveryDesign
);
route_test!(coastal_proof_all_three_scoped_dispositions, CoastalProof);
route_test!(
    emergency_intake_all_three_scoped_dispositions,
    EmergencyIntake
);
route_test!(
    high_risk_clearance_all_three_scoped_dispositions,
    HighRiskClearance
);
route_test!(
    recovery_discharge_all_three_scoped_dispositions,
    RecoveryDischarge
);

#[test]
fn every_route_has_protected_refusal_and_witnessed_recovery_before_and_after_work() {
    for route in ROUTES {
        for performed in [false, true] {
            let mut s = app(route);
            if performed {
                choose(&mut s, 0);
                authorize(&mut s);
                step(
                    &mut s,
                    A::Execute {
                        actor: actors().executor,
                    },
                );
            }
            step(
                &mut s,
                A::Stop {
                    actor: actors().recipient,
                    reason: "withdrawn participation".into(),
                },
            );
            assert_eq!(s.state().recovery_required, performed);
            reject(
                &mut s,
                A::Execute {
                    actor: actors().executor,
                },
            );
            reject(
                &mut s,
                A::Deliver {
                    actor: actors().recipient,
                },
            );
            step(
                &mut s,
                A::Recover {
                    actor: actors().recovery_owner,
                    evidence: vec!["protected return completed".into()],
                },
            );
            reject(
                &mut s,
                A::Deliver {
                    actor: actors().recipient,
                },
            );
            step(
                &mut s,
                A::ObserveRecovery {
                    actor: actors().witness,
                },
            );
            step(
                &mut s,
                A::Deliver {
                    actor: actors().recipient,
                },
            );
            assert!(matches!(
                s.state().outcome.as_ref().unwrap().consequence,
                C::ProtectedReturn { .. }
            ));
            let bytes = s.encode().unwrap();
            assert_eq!(
                HandoffService::decode(&bytes).unwrap().encode().unwrap(),
                bytes
            );
        }
    }
}
#[test]
fn authority_is_live_correct_and_separate_from_ordinary_roles() {
    for route in ROUTES {
        let mut state = world().institutional().clone();
        state.catalog.office_holders.clear();
        let subject = match route {
            Route::Basin => "object.backend-fixture.cargo",
            Route::PublicDesign => "object.backend-fixture.shelter",
            _ => "being.backend-fixture.recipient",
        };
        let mut s = HandoffService::new(
            route,
            subject.into(),
            actors(),
            WorldSession::from_institutional_state(state).unwrap(),
        )
        .unwrap();
        choose(&mut s, 2);
        reject(
            &mut s,
            A::Decide {
                actor: "being.backend-fixture.fake-authority".into(),
                function: primary(route),
            },
        );
        let mut s = app(route);
        choose(&mut s, 2);
        reject(
            &mut s,
            A::Decide {
                actor: actors().executor,
                function: primary(route),
            },
        );
        reject(
            &mut s,
            A::Decide {
                actor: "being.stonebend.fixture-member".into(),
                function: if primary(route) == F::Name {
                    F::Recognize
                } else {
                    F::Name
                },
            },
        );
    }
}
#[test]
fn revision_invalidates_every_choice_and_decision_and_retains_negative_history() {
    for route in ROUTES {
        let mut s = app(route);
        choose(&mut s, 0);
        authorize(&mut s);
        let revision = s.state().evidence.as_ref().unwrap().revision + 1;
        let mut changed = evidence(route, revision);
        changed.negative_results.clear();
        reject(
            &mut s,
            A::RecordEvidence {
                actor: actors().witness,
                evidence: changed,
            },
        );
        step(
            &mut s,
            A::RecordEvidence {
                actor: actors().witness,
                evidence: evidence(route, revision),
            },
        );
        assert_eq!(s.state().phase, Phase::AwaitingChoice);
        assert!(s.state().decisions.is_empty());
        reject(
            &mut s,
            A::Execute {
                actor: actors().executor,
            },
        );
        step(
            &mut s,
            A::Choose {
                actor: actors().recipient,
                alternative: 0,
            },
        );
        authorize(&mut s);
        delivery(&mut s);
        assert_eq!(s.state().outcome.as_ref().unwrap().revision, revision);
        assert!(s.events().iter().any(|e| !e.result.decisions.is_empty()
            && e.result.evidence.as_ref().unwrap().revision < revision));
    }
}
#[test]
fn retries_conflicts_causal_order_and_wrong_actor_fail_without_mutation() {
    for route in ROUTES {
        let mut s = app(route);
        choose(&mut s, 2);
        let first = s.events()[0].clone();
        let before = s.encode().unwrap();
        assert_eq!(
            s.execute(first.id.clone(), first.at, first.action.clone())
                .unwrap(),
            &first
        );
        assert_eq!(s.encode().unwrap(), before);
        assert!(
            s.execute(first.id.clone(), first.at + 1, first.action)
                .is_err()
        );
        assert!(
            s.execute(
                "event.out-of-order".into(),
                1,
                A::Stop {
                    actor: actors().recipient,
                    reason: "stop".into()
                }
            )
            .is_err()
        );
        reject(
            &mut s,
            A::Execute {
                actor: actors().executor,
            },
        );
        assert_eq!(s.encode().unwrap(), before);
        authorize(&mut s);
        step(
            &mut s,
            A::Execute {
                actor: actors().executor,
            },
        );
        reject(
            &mut s,
            A::Observe {
                actor: actors().executor,
                accepts_scope: true,
            },
        );
        reject(
            &mut s,
            A::Observe {
                actor: actors().witness,
                accepts_scope: false,
            },
        );
        reject(
            &mut s,
            A::Deliver {
                actor: actors().recipient,
            },
        );
    }
}
#[test]
fn silence_unknown_capacity_and_emergency_do_not_grant_elective_consent() {
    for route in [
        Route::EmergencyIntake,
        Route::HighRiskClearance,
        Route::RecoveryDischarge,
        Route::RecoveryDesign,
    ] {
        let mut s = app(route);
        let mut e = evidence(route, 1);
        match &mut e.facts {
            Facts::EmergencyIntake { capacity, .. }
            | Facts::HighRiskClearance { capacity, .. }
            | Facts::RecoveryDischarge { capacity, .. }
            | Facts::RecoveryDesign { capacity, .. } => *capacity = Capacity::Unknown,
            _ => unreachable!(),
        }
        step(
            &mut s,
            A::RecordEvidence {
                actor: actors().witness,
                evidence: e,
            },
        );
        reject(
            &mut s,
            A::Choose {
                actor: actors().recipient,
                alternative: 0,
            },
        );
        if route == Route::EmergencyIntake {
            step(
                &mut s,
                A::EmergencyStabilization {
                    actor: actors().executor,
                },
            );
            authorize(&mut s);
            delivery(&mut s);
            let outcome = s.state().outcome.as_ref().unwrap();
            assert!(!outcome.consent_recorded);
            assert!(matches!(
                outcome.consequence,
                C::EmergencyCare {
                    stabilization_recorded: true,
                    elective_intervention_authorized: false,
                    ..
                }
            ));
        } else {
            reject(
                &mut s,
                A::EmergencyStabilization {
                    actor: actors().executor,
                },
            );
        }
    }
    let mut s = app(Route::EmergencyIntake);
    choose(&mut s, 0);
    authorize(&mut s);
    step(
        &mut s,
        A::Stop {
            actor: actors().witness,
            reason: "Nightingale protected stop".into(),
        },
    );
    assert_eq!(s.state().phase, Phase::Stopped);
}
#[test]
fn proof_and_clearance_are_independent_and_failed_or_unreproduced_claims_cannot_pass() {
    for route in [Route::HighRiskClearance, Route::RecoveryDesign] {
        let mut s = app(route);
        choose(&mut s, 0);
        let function = primary(route);
        let actor = if function == F::Clear {
            "being.glaushouse.fixture-member"
        } else {
            "being.sandmanor.fixture-member"
        };
        reject(
            &mut s,
            A::Decide {
                actor: actor.into(),
                function,
            },
        );
    }
    for route in [
        Route::PublicDesign,
        Route::CoastalProof,
        Route::RecoveryDesign,
    ] {
        let mut s = app(route);
        let mut e = evidence(route, 1);
        match &mut e.facts {
            Facts::PublicDesign { passed, .. } => *passed = false,
            Facts::CoastalProof { reproduced, .. } => *reproduced = false,
            _ => {}
        }
        step(
            &mut s,
            A::RecordEvidence {
                actor: actors().witness,
                evidence: e,
            },
        );
        step(
            &mut s,
            A::Choose {
                actor: actors().recipient,
                alternative: 0,
            },
        );
        if route == Route::RecoveryDesign {
            decision(&mut s, F::Clear);
        }
        reject(
            &mut s,
            A::Decide {
                actor: "being.sandmanor.fixture-member".into(),
                function: F::Prove,
            },
        );
    }
}
#[test]
fn names_custody_roles_and_restrictions_survive_their_actual_consequences() {
    for route in [
        Route::PublicName,
        Route::Hollowing,
        Route::CoastalProof,
        Route::RecoveryDischarge,
        Route::Basin,
    ] {
        let mut s = app(route);
        choose(&mut s, 0);
        authorize(&mut s);
        delivery(&mut s);
        match &s.state().outcome.as_ref().unwrap().consequence {
            C::PublicReference {
                former_names,
                liabilities,
                provisional_name,
                ..
            } => {
                assert_eq!(former_names.len(), 1);
                assert_eq!(liabilities.len(), 1);
                assert!(provisional_name.is_some());
            }
            C::ProtectedCustody {
                subject,
                sample,
                destructive_inspection_authorized,
                ..
            } => {
                assert_ne!(subject, sample);
                assert!(!destructive_inspection_authorized);
            }
            C::CoastalRole {
                predecessor,
                result,
                transition_performed,
                ..
            } => {
                assert_eq!(predecessor, "Satyr");
                assert_eq!(result, "Centaur");
                assert!(!transition_performed);
            }
            C::RecoveryTransfer {
                duty_limits,
                full_duty_authorized,
                recovery_continues,
                ..
            } => {
                assert_eq!(duty_limits.len(), 1);
                assert!(!full_duty_authorized);
                assert!(recovery_continues);
            }
            C::DualExpression {
                urban_scope,
                rural_scope,
                sovereign_office,
                ..
            } => {
                assert_ne!(urban_scope, rural_scope);
                assert_eq!(sovereign_office, "office.flynt.tross");
            }
            _ => unreachable!(),
        }
    }
}
#[test]
fn repeated_failure_negative_proof_and_unproven_travel_do_not_become_positive_clearance() {
    for route in [
        Route::RecoveryDesign,
        Route::CoastalProof,
        Route::HighRiskClearance,
        Route::RecoveryDischarge,
    ] {
        let mut s = app(route);
        step(
            &mut s,
            A::RecordEvidence {
                actor: actors().witness,
                evidence: evidence(route, 1),
            },
        );
        let mut e = evidence(route, 2);
        match &mut e.facts {
            Facts::RecoveryDesign {
                revised_moving_passed,
                ..
            } => *revised_moving_passed = false,
            Facts::CoastalProof {
                safer_within_conditions,
                ..
            } => *safer_within_conditions = false,
            Facts::HighRiskClearance {
                proof_supported, ..
            } => *proof_supported = false,
            Facts::RecoveryDischarge {
                travel_proven_within_limits,
                ..
            } => *travel_proven_within_limits = false,
            _ => unreachable!(),
        }
        step(
            &mut s,
            A::RecordEvidence {
                actor: actors().witness,
                evidence: e,
            },
        );
        step(
            &mut s,
            A::Choose {
                actor: actors().recipient,
                alternative: 0,
            },
        );
        match route {
            Route::RecoveryDesign => decision(&mut s, F::Clear),
            Route::HighRiskClearance => decision(&mut s, F::Prove),
            _ => {}
        }
        let function = primary(route);
        let actor = if function == F::Clear {
            "being.glaushouse.fixture-member"
        } else {
            "being.sandmanor.fixture-member"
        };
        reject(
            &mut s,
            A::Decide {
                actor: actor.into(),
                function,
            },
        );
    }
}
fn rehash(value: &mut serde_json::Value) {
    let mut hash = 0xcbf29ce484222325u64;
    let p = &value["payload"];
    for b in serde_json::to_vec(p).unwrap() {
        hash ^= u64::from(b);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    value["checksum"] = format!("fnv1a64:{hash:016x}").into();
}
#[test]
fn damaged_unknown_or_misclassified_archives_fail_closed() {
    for route in ROUTES {
        let mut s = app(route);
        choose(&mut s, 2);
        authorize(&mut s);
        delivery(&mut s);
        let encoded = s.encode().unwrap();
        let mut v: serde_json::Value = serde_json::from_str(&encoded).unwrap();
        v["payload"]["events"][0]["result"]["phase"] = "delivered".into();
        assert!(HandoffService::decode(&v.to_string()).is_err());
        rehash(&mut v);
        assert!(
            HandoffService::decode(&v.to_string())
                .unwrap_err()
                .to_string()
                .contains("replayed handoff differs")
        );
        let mut v: serde_json::Value = serde_json::from_str(&encoded).unwrap();
        v["schema_version"] = 0.into();
        assert!(HandoffService::decode(&v.to_string()).is_err());
        let mut v: serde_json::Value = serde_json::from_str(&encoded).unwrap();
        v["format"] = "hollow-grove-gameplay".into();
        assert!(HandoffService::decode(&v.to_string()).is_err());
    }
}
#[test]
fn changed_identity_erased_name_liability_self_witness_and_non_emergency_intake_are_rejected() {
    let mut bad_actors = actors();
    bad_actors.recovery_owner = bad_actors.witness.clone();
    assert!(
        HandoffService::new(Route::PublicName, actors().recipient, bad_actors, world()).is_err()
    );
    assert!(
        HandoffService::new(
            Route::PublicName,
            "being.another-subject".into(),
            actors(),
            world()
        )
        .is_err()
    );
    let mut s = app(Route::PublicName);
    step(
        &mut s,
        A::RecordEvidence {
            actor: actors().witness,
            evidence: evidence(Route::PublicName, 1),
        },
    );
    let mut revised = evidence(Route::PublicName, 2);
    if let Facts::PublicName {
        former_names,
        liabilities,
        ..
    } = &mut revised.facts
    {
        *former_names = vec!["unrelated history".into()];
        liabilities.clear();
    }
    reject(
        &mut s,
        A::RecordEvidence {
            actor: actors().witness,
            evidence: revised,
        },
    );
    let mut s = app(Route::PublicDesign);
    let mut wrong_subject = evidence(Route::PublicDesign, 1);
    if let Facts::PublicDesign { design, .. } = &mut wrong_subject.facts {
        *design = "another shelter".into();
    }
    reject(
        &mut s,
        A::RecordEvidence {
            actor: actors().witness,
            evidence: wrong_subject,
        },
    );
    let mut s = app(Route::EmergencyIntake);
    let mut not_urgent = evidence(Route::EmergencyIntake, 1);
    if let Facts::EmergencyIntake {
        capacity,
        immediate_stabilization_required,
        ..
    } = &mut not_urgent.facts
    {
        *capacity = Capacity::Unknown;
        *immediate_stabilization_required = false;
    }
    step(
        &mut s,
        A::RecordEvidence {
            actor: actors().witness,
            evidence: not_urgent,
        },
    );
    reject(
        &mut s,
        A::EmergencyStabilization {
            actor: actors().executor,
        },
    );
}
#[test]
fn cli_replays_only_the_supplied_handoff_without_writing_it() {
    let mut s = app(Route::PublicName);
    choose(&mut s, 0);
    authorize(&mut s);
    delivery(&mut s);
    let encoded = s.encode().unwrap();
    let path =
        std::env::temp_dir().join(format!("backend-handoff-cli-{}.json", std::process::id()));
    std::fs::write(&path, &encoded).unwrap();
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_hollow-grove"))
        .args(["architecture", "--json", "--handoff"])
        .arg(&path)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["schema_version"], 3);
    assert_eq!(report["verified_binding_count"], 14);
    assert_eq!(report["unbound_canonical_route_count"], 0);
    assert_eq!(
        report["replayed_handoff_route"],
        Route::PublicName.lore_id()
    );
    assert_eq!(std::fs::read_to_string(&path).unwrap(), encoded);
    std::fs::remove_file(path).unwrap();
}
