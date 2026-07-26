use std::collections::BTreeSet;

use hollow_grove::hollow_grove_contract::House;
use hollow_grove::world::central_junction::{
    self, BoardDecisionId, CentralJunctionError, CentralJunctionInstitution, ConflictDisclosure,
    EconomicClassification, EconomicPole, EnterpriseId, EventOutcome, HouseSectorHall,
    IndexMethodology, JunctionApproach, MarketActorId, MarketDuty, MarketLifecycleState,
    MarketPosition, MarketPositionId, ProjectId, SectorExposure, SettlementEvidenceId,
    StandardCurrencyAmount, SummitConcept, ValueInstrument, VerifiedProductiveEvidence,
    WorkDisposition, WorkLifecycle, WorkObject,
};

fn market_position(
    id: &str,
    contract: &central_junction::EventContractId,
    holder: &str,
) -> MarketPosition {
    MarketPosition {
        id: MarketPositionId::new(id).unwrap(),
        contract: contract.clone(),
        holder: MarketActorId::new(holder).unwrap(),
        outcome: EventOutcome::FactionA,
        stake: StandardCurrencyAmount::from_minor_units(100),
        quoted_price_basis_points: 5_000,
    }
}

#[test]
fn four_poles_exhaustively_map_form_function_creation_and_continuance() {
    let expected = [
        (
            WorkObject::Form,
            WorkLifecycle::Creation,
            EconomicPole::Design,
            House::Sandmanor,
        ),
        (
            WorkObject::Function,
            WorkLifecycle::Creation,
            EconomicPole::Engineering,
            House::Flynt,
        ),
        (
            WorkObject::Form,
            WorkLifecycle::Continuance,
            EconomicPole::Craft,
            House::Stonebend,
        ),
        (
            WorkObject::Function,
            WorkLifecycle::Continuance,
            EconomicPole::Repair,
            House::Glaushouse,
        ),
    ];

    for (object, lifecycle, pole, house) in expected {
        assert_eq!(
            central_junction::classify_economic_pole(object, lifecycle),
            pole
        );
        assert_eq!(pole.work_object(), object);
        assert_eq!(pole.lifecycle(), lifecycle);
        assert_eq!(pole.house(), house);
    }
    assert_eq!(
        EconomicPole::ALL.into_iter().collect::<BTreeSet<_>>().len(),
        4
    );
}

#[test]
fn disposition_matrix_is_exact() {
    assert_eq!(
        central_junction::classify_work_disposition(true, true),
        WorkDisposition::Act
    );
    assert_eq!(
        central_junction::classify_work_disposition(true, false),
        WorkDisposition::Cultivate
    );
    assert_eq!(
        central_junction::classify_work_disposition(false, true),
        WorkDisposition::Reroute
    );
    assert_eq!(
        central_junction::classify_work_disposition(false, false),
        WorkDisposition::Release
    );
}

#[test]
fn one_unnamed_currency_is_distinct_from_indexes_tokens_and_gremlincoin() {
    assert_eq!(central_junction::STANDARD_CURRENCY_PUBLIC_NAME, None);
    let amount = StandardCurrencyAmount::from_minor_units(4_200);
    assert_eq!(amount.minor_units(), 4_200);

    for instrument in [
        ValueInstrument::TokeToken,
        ValueInstrument::EnterpriseShare,
        ValueInstrument::EventContractPosition,
        ValueInstrument::Gremlincoin,
        ValueInstrument::SectorIndex,
    ] {
        assert!(!instrument.is_ordinary_currency());
        assert!(!instrument.is_spendable_money());
    }
    assert!(ValueInstrument::StandardCurrency.is_ordinary_currency());
    assert!(ValueInstrument::StandardCurrency.is_spendable_money());
    assert!(ValueInstrument::TokeToken.is_earned_evidence());
    assert!(ValueInstrument::Gremlincoin.is_earned_evidence());
}

#[test]
fn central_junction_is_a_district_with_exact_institutions() {
    let junction = central_junction::canonical_central_junction();
    assert_eq!(junction.stable_id, "district.central-junction");
    assert_eq!(junction.formal_name, "CENTRAL JUNCTION");
    assert_eq!(junction.short_name, "The Junction");
    assert!(junction.district_not_single_building);
    assert_eq!(
        junction.institutions,
        CentralJunctionInstitution::ALL.into_iter().collect()
    );
    assert!(
        junction
            .institutions
            .contains(&CentralJunctionInstitution::SouthRidgeExchange)
    );
    assert!(
        junction
            .institutions
            .contains(&CentralJunctionInstitution::JunctionBoard)
    );
    assert!(
        junction
            .institutions
            .contains(&CentralJunctionInstitution::ClearingHouse)
    );
    assert!(
        junction
            .institutions
            .contains(&CentralJunctionInstitution::JunctionWire)
    );
}

#[test]
fn exchange_board_clearing_and_wire_have_nonoverlapping_practical_authority() {
    use central_junction::MarketAuthorityFunction as Function;

    let expected = [
        (
            CentralJunctionInstitution::SouthRidgeExchange,
            Function::EnterpriseListings,
        ),
        (
            CentralJunctionInstitution::SouthRidgeExchange,
            Function::OfficialIndexCalculation,
        ),
        (
            CentralJunctionInstitution::JunctionBoard,
            Function::IndexMethodology,
        ),
        (
            CentralJunctionInstitution::JunctionBoard,
            Function::ConflictOfInterestRules,
        ),
        (
            CentralJunctionInstitution::ClearingHouse,
            Function::EventContractSettlement,
        ),
        (
            CentralJunctionInstitution::ClearingHouse,
            Function::FinalBalances,
        ),
        (
            CentralJunctionInstitution::JunctionWire,
            Function::IndexPublication,
        ),
        (
            CentralJunctionInstitution::JunctionWire,
            Function::SettledOutcomePublication,
        ),
    ];
    for (authority, function) in expected {
        assert!(authority.governs(function));
        assert_eq!(
            CentralJunctionInstitution::ALL
                .into_iter()
                .filter(|candidate| candidate.governs(function))
                .count(),
            1
        );
    }
    assert!(!CentralJunctionInstitution::JunctionBoard.governs(Function::EventContractSettlement));
    assert!(!CentralJunctionInstitution::ClearingHouse.governs(Function::IndexMethodology));
}

#[test]
fn corridors_and_flynt_ring_reach_the_correct_houses() {
    let expected = [
        (JunctionApproach::CraftCorridor, House::Stonebend, true),
        (JunctionApproach::RepairCorridor, House::Glaushouse, true),
        (JunctionApproach::DesignCorridor, House::Sandmanor, true),
        (JunctionApproach::FlyntEngineeringRing, House::Flynt, false),
    ];
    for (approach, house, interior_spoke) in expected {
        assert_eq!(approach.toward_house(), house);
        assert_eq!(approach.is_interior_spoke(), interior_spoke);
    }
}

#[test]
fn each_house_has_one_sector_hall_and_halls_never_set_market_prices() {
    let expected = [
        (House::Stonebend, HouseSectorHall::StonebendCraftHall),
        (House::Sandmanor, HouseSectorHall::SandmanorDesignHall),
        (House::Flynt, HouseSectorHall::FlyntEngineeringHall),
        (House::Glaushouse, HouseSectorHall::GlaushouseRepairHall),
    ];
    for (house, hall) in expected {
        assert_eq!(hall.house(), house);
        assert!(!hall.sets_market_price());
    }
}

#[test]
fn summit_concepts_retain_vision_meanings_and_are_not_market_bureaucracy() {
    let expected = [
        (
            SummitConcept::CurrentHaze,
            "Current Haze is unresolved possibility.",
        ),
        (
            SummitConcept::EqualGaze,
            "Equal Gaze is reconciled perspective.",
        ),
        (
            SummitConcept::AuraBeam,
            "Aura Beam reveals or transmits the visible shared future.",
        ),
    ];
    for (concept, statement) in expected {
        assert_eq!(concept.canonical_statement(), statement);
        assert!(!concept.is_market_institution());
        assert!(!concept.is_financial_ticker());
    }
}

#[test]
fn four_indexes_are_independent_noncurrency_public_measurements() {
    let indexes = central_junction::canonical_market_indexes();
    assert_eq!(indexes.len(), 4);
    assert_eq!(
        indexes
            .iter()
            .map(|index| index.pole)
            .collect::<BTreeSet<_>>(),
        EconomicPole::ALL.into_iter().collect()
    );
    for index in &indexes {
        assert_eq!(index.owner, None);
        assert!(!index.currency);
        assert_eq!(
            index.methodology_authority,
            CentralJunctionInstitution::JunctionBoard
        );
        assert_eq!(
            index.calculation_authority,
            CentralJunctionInstitution::SouthRidgeExchange
        );
        assert_eq!(
            index.publication_authority,
            CentralJunctionInstitution::JunctionWire
        );
    }
}

#[test]
fn every_sector_hall_has_one_wire_connected_public_index_board() {
    let boards = central_junction::canonical_public_index_boards();
    assert_eq!(boards.len(), 4);
    assert_eq!(
        boards
            .iter()
            .map(|board| board.hall)
            .collect::<BTreeSet<_>>(),
        HouseSectorHall::ALL.into_iter().collect()
    );
    assert_eq!(
        boards
            .iter()
            .map(|board| board.index.as_str())
            .collect::<BTreeSet<_>>()
            .len(),
        4
    );
    for board in boards {
        assert_eq!(board.connected_to, CentralJunctionInstitution::JunctionWire);
        assert_eq!(
            board.fields,
            central_junction::PublicIndexBoardField::ALL
                .into_iter()
                .collect()
        );
    }
}

#[test]
fn official_index_uses_recognized_productive_evidence_and_wire_publication() {
    let index = central_junction::canonical_market_indexes()
        .into_iter()
        .find(|index| index.pole == EconomicPole::Repair)
        .unwrap();
    let evidence = [VerifiedProductiveEvidence {
        id: SettlementEvidenceId::new("evidence.index.repair-restoration").unwrap(),
        hall: HouseSectorHall::GlaushouseRepairHall,
        pole: EconomicPole::Repair,
        net_productive_units: 906,
        board_recognized: true,
    }];
    let value = central_junction::calculate_official_index(
        &index,
        &IndexMethodology {
            approved_by: CentralJunctionInstitution::JunctionBoard,
            base_millipoints: 100_000,
            productive_unit_weight: 10,
        },
        &evidence,
        3_700,
    )
    .unwrap();
    assert_eq!(value.value_millipoints, 112_760);
    assert_eq!(
        value.calculated_by,
        CentralJunctionInstitution::SouthRidgeExchange
    );
    assert_eq!(value.published_by, CentralJunctionInstitution::JunctionWire);
}

#[test]
fn listings_preserve_stable_identity_primary_pole_and_secondary_exposure() {
    let classification = EconomicClassification {
        primary: SectorExposure {
            pole: EconomicPole::Engineering,
            basis_points: 6_000,
        },
        secondary: vec![
            SectorExposure {
                pole: EconomicPole::Craft,
                basis_points: 2_500,
            },
            SectorExposure {
                pole: EconomicPole::Repair,
                basis_points: 1_500,
            },
        ],
        evidence: vec!["rail operation is the principal contribution".into()],
    };
    classification.validate().unwrap();

    let stable_id = EnterpriseId::new("enterprise.rail.shared-line").unwrap();
    let listing = central_junction::ListedEnterprise {
        id: stable_id.clone(),
        name: "Shared Line Rail".into(),
        classification,
        shares_outstanding: 10_000,
        state: MarketLifecycleState::Open,
    };
    listing.validate().unwrap();
    assert_eq!(listing.id, stable_id);
    assert_eq!(
        listing.classification.primary.pole,
        EconomicPole::Engineering
    );
    assert_eq!(listing.classification.secondary.len(), 2);
}

#[test]
fn projects_use_the_same_primary_and_secondary_classification_without_new_money() {
    let project = central_junction::ListedProject {
        id: ProjectId::new("project.central-junction.repair-transit-span").unwrap(),
        name: "Repair Transit Span".into(),
        classification: EconomicClassification {
            primary: SectorExposure {
                pole: EconomicPole::Repair,
                basis_points: 5_500,
            },
            secondary: vec![
                SectorExposure {
                    pole: EconomicPole::Engineering,
                    basis_points: 3_000,
                },
                SectorExposure {
                    pole: EconomicPole::Craft,
                    basis_points: 1_500,
                },
            ],
            evidence: vec!["restoration of failed transit Function is primary".into()],
        },
        financing_target: StandardCurrencyAmount::from_minor_units(75_000),
        state: MarketLifecycleState::Proposed,
    };
    project.validate().unwrap();
    assert_eq!(project.classification.primary.pole, EconomicPole::Repair);
    assert_eq!(project.financing_target.minor_units(), 75_000);
}

#[test]
fn blackroot_fixture_defines_outcomes_before_opening_and_settles_from_evidence() {
    let proof = central_junction::blackroot_workshop_event_proof().unwrap();
    assert!(proof.contract.definitions_recorded_at < proof.contract.opens_at);
    assert!(proof.contract.opens_at < proof.contract.closes_at);
    assert_eq!(proof.process, central_junction::MarketProcessStage::ALL);
    assert_eq!(proof.attestations.len(), 8);
    assert_eq!(proof.decision.state, MarketLifecycleState::Recognized);
    assert_eq!(proof.decision.outcome, Some(EventOutcome::FactionA));
    assert!(proof.decision.may_be_described_as_equal_gaze);
    assert_eq!(proof.settlement.outcome, EventOutcome::FactionA);
    assert_eq!(proof.settlement.state, MarketLifecycleState::Settled);
    assert!(!proof.settlement.market_price_determined_outcome);
    assert_eq!(
        proof.publication.published_by,
        CentralJunctionInstitution::JunctionWire
    );
    assert_eq!(proof.publication.recognized_outcome, EventOutcome::FactionA);
}

#[test]
fn settlement_price_and_constitutional_outcome_are_distinct() {
    let proof = central_junction::blackroot_workshop_event_proof().unwrap();
    let reviewer = MarketActorId::new("actor.junction-board.price-independent").unwrap();
    let positions = vec![MarketPosition {
        quoted_price_basis_points: 9_999,
        ..market_position(
            "position.blackroot.false-favorite",
            &proof.contract.id,
            "actor.blackroot.false-favorite",
        )
    }];
    let decision = central_junction::audit_event_contract(
        &proof.contract,
        &proof.attestations,
        reviewer,
        &positions,
        &[],
        &[],
    )
    .unwrap();
    assert_eq!(decision.outcome, Some(EventOutcome::FactionA));
}

#[test]
fn incomplete_or_wrong_domain_evidence_cannot_be_recognized() {
    let proof = central_junction::blackroot_workshop_event_proof().unwrap();
    let reviewer = MarketActorId::new("actor.junction-board.incomplete-review").unwrap();
    assert!(matches!(
        central_junction::audit_event_contract(
            &proof.contract,
            &proof.attestations[..7],
            reviewer.clone(),
            &[],
            &[],
            &[],
        ),
        Err(CentralJunctionError::IncompleteSettlementEvidence)
    ));

    let mut wrong_domain = proof.attestations;
    wrong_domain[0].hall = HouseSectorHall::SandmanorDesignHall;
    assert!(matches!(
        central_junction::audit_event_contract(
            &proof.contract,
            &wrong_domain,
            reviewer,
            &[],
            &[],
            &[],
        ),
        Err(CentralJunctionError::InvalidSettlementEvidence)
    ));
}

#[test]
fn conflicted_reviewer_clearing_official_and_commander_cannot_act_secretly() {
    let proof = central_junction::blackroot_workshop_event_proof().unwrap();

    let reviewer = MarketActorId::new("actor.junction-board.conflicted").unwrap();
    let reviewer_position = market_position(
        "position.blackroot.conflicted-reviewer",
        &proof.contract.id,
        reviewer.as_str(),
    );
    assert!(matches!(
        central_junction::audit_event_contract(
            &proof.contract,
            &proof.attestations,
            reviewer.clone(),
            std::slice::from_ref(&reviewer_position),
            &[],
            &[],
        ),
        Err(CentralJunctionError::UndisclosedProhibitedInterest(actor)) if actor == reviewer
    ));

    let clearing = MarketActorId::new("actor.clearing-house.conflicted").unwrap();
    let clearing_position = market_position(
        "position.blackroot.conflicted-clearing",
        &proof.contract.id,
        clearing.as_str(),
    );
    assert!(matches!(
        central_junction::settle_event_contract(
            &proof.contract,
            &proof.decision,
            std::slice::from_ref(&clearing_position),
            clearing.clone(),
            &[],
        ),
        Err(CentralJunctionError::UndisclosedProhibitedInterest(actor)) if actor == clearing
    ));

    let commander = MarketActorId::new("actor.blackroot.conflicted-commander").unwrap();
    let commander_position = market_position(
        "position.blackroot.conflicted-commander",
        &proof.contract.id,
        commander.as_str(),
    );
    assert!(matches!(
        central_junction::audit_event_contract(
            &proof.contract,
            &proof.attestations,
            MarketActorId::new("actor.junction-board.clean-reviewer").unwrap(),
            std::slice::from_ref(&commander_position),
            &[],
            std::slice::from_ref(&commander),
        ),
        Err(CentralJunctionError::UndisclosedProhibitedInterest(actor)) if actor == commander
    ));
}

#[test]
fn disclosure_and_recusal_remove_a_conflicted_official_from_the_action() {
    let proof = central_junction::blackroot_workshop_event_proof().unwrap();
    let reviewer = MarketActorId::new("actor.junction-board.recused").unwrap();
    let position = market_position(
        "position.blackroot.recused-reviewer",
        &proof.contract.id,
        reviewer.as_str(),
    );
    let disclosure = ConflictDisclosure {
        actor: reviewer.clone(),
        contract: proof.contract.id.clone(),
        duty: MarketDuty::JunctionBoardReviewer,
        material_interest: true,
        disclosed: true,
        recused: true,
    };
    assert!(matches!(
        central_junction::audit_event_contract(
            &proof.contract,
            &proof.attestations,
            reviewer.clone(),
            &[position],
            &[disclosure],
            &[],
        ),
        Err(CentralJunctionError::RecusedOfficialCannotAct(actor)) if actor == reviewer
    ));
}

#[test]
fn lifecycle_uses_ordinary_market_states_only() {
    let states = [
        MarketLifecycleState::Proposed,
        MarketLifecycleState::Open,
        MarketLifecycleState::UnderReview,
        MarketLifecycleState::Recognized,
        MarketLifecycleState::Disputed,
        MarketLifecycleState::Settled,
        MarketLifecycleState::Voided,
    ];
    assert_eq!(states.len(), 7);
    assert_eq!(
        BoardDecisionId::new("decision.central-junction.stable")
            .unwrap()
            .as_str(),
        "decision.central-junction.stable"
    );
}
