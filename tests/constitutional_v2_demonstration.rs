use hollow_grove::constitutional::*;
use hollow_grove::institution::OfficeId;
use hollow_grove::lineage_contract::SandmanorForm;

#[test]
fn all_four_polarities_round_trip_through_persistence_and_replay() {
    let cases = [
        (
            Sign::Positive,
            Sign::Positive,
            ConstitutionalPolarity::PositiveCurrentPositiveAura,
        ),
        (
            Sign::Positive,
            Sign::Negative,
            ConstitutionalPolarity::PositiveCurrentNegativeAura,
        ),
        (
            Sign::Negative,
            Sign::Positive,
            ConstitutionalPolarity::NegativeCurrentPositiveAura,
        ),
        (
            Sign::Negative,
            Sign::Negative,
            ConstitutionalPolarity::NegativeCurrentNegativeAura,
        ),
    ];
    for (current, aura, expected) in cases {
        let scenario = run_polarity_scenario(current, aura).unwrap();
        assert_eq!(scenario.polarity, expected);
        let archive = encode_constitutional_archive(&scenario.runtime).unwrap();
        let decoded = decode_constitutional_archive(&archive).unwrap();
        let replayed = ConstitutionalRuntime::replay(
            scenario.runtime.waves().cloned(),
            scenario.runtime.events().iter().cloned(),
        )
        .unwrap();
        assert_eq!(decoded.events(), scenario.runtime.events());
        assert_eq!(replayed.events(), scenario.runtime.events());
        assert_eq!(encode_constitutional_archive(&decoded).unwrap(), archive);
    }
}

#[test]
fn every_house_function_rejects_an_office_from_another_house() {
    let world = hollow_grove::world::institutional_access_fixture();
    let cases = [
        (HouseFunction::Name, "office.sandmanor.sandman"),
        (HouseFunction::Prove, "office.stonebend.hypergiant"),
        (HouseFunction::Recognize, "office.glaushouse.prima-donna"),
        (HouseFunction::Clear, "office.flynt.tross"),
        (HouseFunction::Resolve, "office.sandmanor.sandman"),
    ];
    for (index, (function, office_value)) in cases.into_iter().enumerate() {
        let office = OfficeId::new(office_value).unwrap();
        let holder = world
            .catalog
            .office_holders
            .iter()
            .find(|holder| holder.active && holder.office == office)
            .unwrap();
        let result = HouseDecision::from_catalog(
            &world.catalog,
            HouseDecisionDraft {
                id: HouseDecisionId::new(format!("decision.wrong-house.{index}")).unwrap(),
                function,
                office,
                actor: AuthorityActorId::new(holder.being.as_str()).unwrap(),
                outcome: HouseDecisionOutcome::Accepted,
                evidence: vec![scenario_evidence("wrong-house")],
                causal_position: CausalPosition::new(1),
            },
        );
        assert!(matches!(result, Err(HouseLawError::WrongHouse { .. })));
    }
}

#[test]
fn ordinary_scenario_proves_tombstone_toke_renewal_and_inheritance() {
    let scenario = run_ordinary_lifecycle().unwrap();
    let successor = scenario.successor.as_ref().unwrap();
    assert_eq!(
        scenario.runtime.bond(&scenario.bond).unwrap().phase(),
        BondPhase::Resolved
    );
    assert_eq!(
        scenario.runtime.bond(successor).unwrap().phase(),
        BondPhase::Formed
    );
    assert!(scenario.runtime.events().iter().any(|event| {
        event.bond == scenario.bond && matches!(event.payload, BondEvent::TombstoneFormed(_))
    }));
    assert!(scenario.runtime.events().iter().any(|event| {
        event.bond == scenario.bond && matches!(event.payload, BondEvent::TokeRecorded(_))
    }));
    let BondEvent::Formed(successor_formation) = &scenario
        .runtime
        .events()
        .iter()
        .find(|event| &event.bond == successor && event.sequence == 0)
        .unwrap()
        .payload
    else {
        panic!("successor begins with formation");
    };
    assert_eq!(successor_formation.parent_bonds, vec![scenario.bond]);
    assert!(!successor_formation.inheritance_evidence.is_empty());
    scenario.runtime.verify_successor_integrity().unwrap();
}

#[test]
fn default_challenge_and_terminal_failures_are_demonstrable() {
    let scenario = run_default_challenge_scenario().unwrap();
    assert_eq!(
        scenario.runtime.bond(&scenario.bond).unwrap().phase(),
        BondPhase::Mature
    );
    assert!(scenario.runtime.events().iter().any(|event| matches!(
        event.payload,
        BondEvent::DefaultResolved(BondDefaultResolution {
            outcome: DefaultOutcome::Cured,
            ..
        })
    )));
    assert!(scenario.runtime.events().iter().any(|event| matches!(
        event.payload,
        BondEvent::ChallengeResolved(BondChallengeResolution {
            outcome: ChallengeOutcome::Rejected,
            ..
        })
    )));

    let premature = run_premature_maturity_scenario().unwrap();
    assert!(matches!(
        premature.error,
        ConstitutionalRuntimeError::Bond(BondStateError::PrematureMaturity)
    ));
    assert_eq!(premature.event_count_before, premature.event_count_after);
    assert!(!trace_rejected_bond_scenario(&premature).transitions[0].state_changed);

    let terminal = run_terminal_renewal_rejection().unwrap();
    assert!(matches!(
        terminal.error,
        ConstitutionalRuntimeError::Bond(BondStateError::WrongResolutionPrecondition {
            phase: BondPhase::Resolved,
            ..
        })
    ));
    assert_eq!(terminal.event_count_before, terminal.event_count_after);
}

#[test]
fn identities_are_caller_supplied_and_not_insertion_order_derived() {
    let first = run_gnome_minotaur_scenario().unwrap();
    let second = run_gnome_minotaur_scenario().unwrap();
    assert_eq!(first.predecessor, second.predecessor);
    assert_eq!(first.result, second.result);
    assert_eq!(first.synthesis, second.synthesis);
    assert_eq!(first.runtime.events(), second.runtime.events());
    assert_eq!(first.archive, second.archive);
}

#[test]
fn regional_roles_are_reducer_outputs_not_location_inference() {
    let world = hollow_grove::world::institutional_access_fixture();
    let gnome = RegionalBeingId::new("being.unsynthesized.gnome").unwrap();
    let elf = RegionalBeingId::new("being.unsynthesized.elf").unwrap();
    let mut runtime = RegionalSynthesisRuntime::new();
    runtime
        .register_being(
            scenario_regional_metadata("unsynthesized.gnome", 1),
            scenario_regional_registration(
                &world.catalog,
                gnome.clone(),
                SandmanorForm::Gnome,
                ConstitutionalRegion::AuraFields,
                RegionalStandingKind::Established,
            )
            .unwrap(),
        )
        .unwrap();
    runtime
        .register_being(
            scenario_regional_metadata("unsynthesized.elf", 2),
            scenario_regional_registration(
                &world.catalog,
                elf.clone(),
                SandmanorForm::Elf,
                ConstitutionalRegion::AuraBeach,
                RegionalStandingKind::Established,
            )
            .unwrap(),
        )
        .unwrap();
    assert!(runtime.stewardship(&gnome).is_none());
    assert!(runtime.guardianship(&elf).is_none());
    assert!(runtime.beach_occupation(&elf).is_none());
}

#[test]
fn end_to_end_kernel_bond_and_both_regional_lineages_replay_exactly() {
    let kernel = run_kernel_wave_scenario().unwrap();
    assert_eq!(kernel.constitutional_event_count, 0);
    assert!(kernel.runtime.wave(&kernel.wave).is_some());

    let bond = run_ordinary_lifecycle().unwrap();
    let bond_trace = trace_bond_scenario(&bond).unwrap();
    assert!(bond_trace.live_replay_equivalent);
    assert!(bond_trace.canonical_persistence);

    let minotaur = run_gnome_minotaur_scenario().unwrap();
    let centaur = run_elf_centaur_scenario().unwrap();
    for scenario in [&minotaur, &centaur] {
        let replayed =
            RegionalSynthesisRuntime::replay(scenario.runtime.events().iter().cloned()).unwrap();
        let persisted = decode_regional_archive(&scenario.archive).unwrap();
        assert_eq!(replayed, scenario.runtime);
        assert_eq!(persisted, scenario.runtime);
        assert_eq!(
            replayed.lineage(&scenario.result),
            scenario.runtime.lineage(&scenario.result)
        );
        let event_count = scenario.runtime.events().len();
        let trace = trace_regional_scenario(scenario).unwrap();
        assert_eq!(scenario.runtime.events().len(), event_count);
        assert!(trace.live_replay_equivalent);
    }
    assert_eq!(
        minotaur.runtime.being(&minotaur.result).unwrap().form,
        SandmanorForm::Minotaur
    );
    assert_eq!(
        centaur.runtime.being(&centaur.result).unwrap().form,
        SandmanorForm::Centaur
    );
    assert!(minotaur.runtime.stewardship(&minotaur.result).is_some());
    assert!(centaur.runtime.guardianship(&centaur.result).is_some());

    for illegal in ["gnome-centaur", "elf-minotaur"] {
        let rejected = run_rejected_regional_scenario(illegal).unwrap();
        assert_eq!(rejected.event_count_before, rejected.event_count_after);
        assert!(rejected.runtime.being(&rejected.attempted_result).is_none());
        let trace = trace_rejected_regional_scenario(&rejected);
        assert!(!trace.transitions[0].state_changed);
    }
}
