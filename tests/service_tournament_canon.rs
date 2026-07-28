use std::collections::{BTreeMap, BTreeSet};

use hollow_grove::hollow_grove_contract::House;
use hollow_grove::world::central_junction::{
    CentralJunctionFunction, CentralJunctionInstitution, EconomicPole, canonical_central_junction,
};
use hollow_grove::world::service_tournament::{
    self, CompetitorId, HouseColorFamily, HouseScorecard, LayerMeaning, MarkAction, MarkGrammar,
    MarkId, PaintMark, PaintMarkProvenance, PairedServiceIdentity, ResultId, ScenarioId,
    ScenarioType, ScoringCategory, ServiceMark, ServiceMarkId, ServiceMarkProvenance,
    ServiceMarkSignificance, ServiceTournamentError, ServiceTournamentRuntime, SimulationSystem,
    TournamentAuthorityId, TournamentAward, TournamentCompetitor, TournamentEvent,
    TournamentEventId, TournamentEventKind, TournamentEvidenceId, TournamentId,
    TournamentLocationId, TournamentObjective, TournamentResult, TournamentScenario,
    TournamentYearId,
};

const CANON: &str = include_str!("../SERVICE_TOURNAMENT_CENTRAL_JUNCTION_CANON_V1.md");

fn tournament_id() -> TournamentId {
    TournamentId::new("function.central-junction.service-tournament").unwrap()
}

fn competitor_id(house: House) -> CompetitorId {
    CompetitorId::new(format!(
        "competitor.service-tournament.{}",
        house.as_str().to_ascii_lowercase()
    ))
    .unwrap()
}

fn event(sequence: u64, kind: TournamentEventKind) -> TournamentEvent {
    TournamentEvent {
        id: TournamentEventId::new(format!("event.service-tournament.{sequence}")).unwrap(),
        tournament: tournament_id(),
        semantic_sequence: sequence,
        kind,
    }
}

fn competitor(identity: PairedServiceIdentity) -> TournamentCompetitor {
    TournamentCompetitor {
        id: competitor_id(identity.house()),
        tournament: tournament_id(),
        house: identity.house(),
        service_identity: identity,
        public_name: format!("{} delegation", identity.house_display_name()),
    }
}

fn opened_runtime() -> ServiceTournamentRuntime {
    let mut runtime = ServiceTournamentRuntime::canonical().unwrap();
    for (sequence, identity) in PairedServiceIdentity::ALL.into_iter().enumerate() {
        runtime
            .apply_event(event(
                sequence as u64,
                TournamentEventKind::CompetitorRegistered(competitor(identity)),
            ))
            .unwrap();
    }
    runtime
        .apply_event(event(4, TournamentEventKind::TournamentOpened))
        .unwrap();
    runtime
}

fn scenario() -> TournamentScenario {
    TournamentScenario {
        id: ScenarioId::new("scenario.thousand-hues.west-gate-breach").unwrap(),
        tournament: tournament_id(),
        war: Some(service_tournament::canonical_war_of_a_thousand_hues().id),
        scenario_type: ScenarioType::ControlledBreach,
        operational_zone: TournamentLocationId::new("location.central-junction.west-gate").unwrap(),
        objectives: vec![
            TournamentObjective {
                id: service_tournament::ObjectiveId::new(
                    "objective.thousand-hues.west-gate-breach",
                )
                .unwrap(),
                category: ScoringCategory::MissionCompletion,
                description: "Open the simulated gate without endangering the public.".into(),
                available_points: 20,
            },
            TournamentObjective {
                id: service_tournament::ObjectiveId::new(
                    "objective.thousand-hues.west-gate-restraint",
                )
                .unwrap(),
                category: ScoringCategory::ConstitutionalRestraint,
                description: "Use only the force authorized for the controlled breach.".into(),
                available_points: 20,
            },
        ],
        simulation_systems: [
            SimulationSystem::PaintballStyleWeapon,
            SimulationSystem::MockExplosive,
            SimulationSystem::MovableBarricade,
        ]
        .into_iter()
        .collect(),
        nonlethal: true,
    }
}

fn flynt_mark(mark_event: TournamentEventId) -> PaintMark {
    PaintMark {
        id: MarkId::new("mark.thousand-hues.west-gate.flynt-burst").unwrap(),
        war: service_tournament::canonical_war_of_a_thousand_hues().id,
        scenario: Some(ScenarioId::new("scenario.thousand-hues.west-gate-breach").unwrap()),
        location: TournamentLocationId::new("location.central-junction.west-gate").unwrap(),
        house: House::Flynt,
        color_family: HouseColorFamily::Black,
        palette_sources: ["flynt.rich_black_blue".into(), "flynt.gunmetal".into()]
            .into_iter()
            .collect(),
        hue_description: "weathered obsidian and gunmetal".into(),
        action: MarkAction::Breached,
        grammar: Some(MarkGrammar::Burst),
        layer_sequence: 0,
        provenance: PaintMarkProvenance {
            originating_event: mark_event,
            competitor: competitor_id(House::Flynt),
            evidence: [
                TournamentEvidenceId::new("evidence.thousand-hues.west-gate-breach").unwrap(),
            ]
            .into_iter()
            .collect(),
        },
    }
}

fn complete_scorecard(points: u16) -> HouseScorecard {
    HouseScorecard {
        scores: ScoringCategory::ALL
            .into_iter()
            .map(|category| (category, points))
            .collect(),
        penalties: BTreeSet::new(),
    }
}

fn result(transfers_permanent_sovereignty: bool) -> TournamentResult {
    TournamentResult {
        id: ResultId::new("result.service-tournament.canonical-final").unwrap(),
        tournament: tournament_id(),
        champion: House::Glaushouse,
        scorecards: [
            (House::Flynt, complete_scorecard(7)),
            (House::Stonebend, complete_scorecard(8)),
            (House::Sandmanor, complete_scorecard(9)),
            (House::Glaushouse, complete_scorecard(10)),
        ]
        .into_iter()
        .collect(),
        award: TournamentAward::FirstMarkInNextWar,
        transfers_permanent_sovereignty,
    }
}

#[test]
fn each_house_has_exactly_one_fixed_paired_service_identity() {
    let expected = [
        (
            PairedServiceIdentity::FlyntAtfArmy,
            House::Flynt,
            "Flynt, MI",
            "ATF",
            "Army",
        ),
        (
            PairedServiceIdentity::StonebendDeaAirForce,
            House::Stonebend,
            "Stonebend",
            "DEA",
            "Air Force",
        ),
        (
            PairedServiceIdentity::SandmanorCiaNavy,
            House::Sandmanor,
            "Sandmanor",
            "CIA",
            "Navy",
        ),
        (
            PairedServiceIdentity::GlaushouseFbiMarines,
            House::Glaushouse,
            "Glaüshouse",
            "FBI",
            "Marines",
        ),
    ];
    assert_eq!(PairedServiceIdentity::ALL.len(), 4);
    for (identity, house, display, agency, armed_service) in expected {
        assert_eq!(identity.house(), house);
        assert_eq!(identity.house_display_name(), display);
        assert_eq!(identity.agency_reference(), agency);
        assert_eq!(identity.armed_service_reference(), armed_service);
    }

    let profiles = service_tournament::canonical_house_service_profiles();
    assert!(profiles.iter().all(|profile| {
        profile.one_complete_cultural_government_identity
            && profile.external_reference_models_only
            && profile.preserves_house_government
            && !profile.creates_separate_service_teams
    }));
}

#[test]
fn tournament_has_exactly_four_house_representatives() {
    let tournament = service_tournament::canonical_service_tournament();
    assert_eq!(tournament.representatives.len(), 4);
    assert_eq!(
        tournament
            .representatives
            .into_iter()
            .map(PairedServiceIdentity::house)
            .collect::<BTreeSet<_>>()
            .len(),
        4
    );

    let runtime = opened_runtime();
    assert!(runtime.is_open());
    assert_eq!(runtime.competitors().len(), 4);
}

#[test]
fn war_of_a_thousand_hues_and_every_scenario_are_nonlethal() {
    let war = service_tournament::canonical_war_of_a_thousand_hues();
    assert_eq!(war.official_name, "The War of a Thousand Hues");
    assert!(war.nonlethal);

    let mut runtime = opened_runtime();
    let mut lethal = scenario();
    lethal.nonlethal = false;
    assert!(matches!(
        runtime.apply_event(event(5, TournamentEventKind::ScenarioRegistered(lethal))),
        Err(ServiceTournamentError::NonlethalSimulationRequired)
    ));
}

#[test]
fn stonebend_owns_blue() {
    assert_eq!(
        HouseColorFamily::for_house(House::Stonebend),
        HouseColorFamily::Blue
    );
}

#[test]
fn sandmanor_owns_red() {
    assert_eq!(
        HouseColorFamily::for_house(House::Sandmanor),
        HouseColorFamily::Red
    );
}

#[test]
fn glaushouse_owns_green() {
    assert_eq!(
        HouseColorFamily::for_house(House::Glaushouse),
        HouseColorFamily::Green
    );
}

#[test]
fn flynt_owns_a_varied_black_family_without_featureless_pure_black() {
    let family = HouseColorFamily::for_house(House::Flynt);
    assert_eq!(family, HouseColorFamily::Black);
    assert!(family.named_variations().contains(&"Onyx"));
    assert!(family.named_variations().contains(&"Obsidian"));
    assert!(family.named_variations().contains(&"Gunmetal"));
    assert!(family.palette_color_ids().contains(&"flynt.gunmetal"));
    assert!(!family.permits_featureless_pure_black());
}

#[test]
fn service_marks_preserve_source_mark_event_participant_and_evidence_provenance() {
    let mut runtime = opened_runtime();
    runtime
        .apply_event(event(
            5,
            TournamentEventKind::ScenarioRegistered(scenario()),
        ))
        .unwrap();
    let mark_event = TournamentEventId::new("event.service-tournament.6").unwrap();
    let mark = flynt_mark(mark_event.clone());
    runtime
        .apply_event(event(6, TournamentEventKind::MarkRecorded(mark.clone())))
        .unwrap();

    let preservation_event = TournamentEventId::new("event.service-tournament.7").unwrap();
    let service_mark = ServiceMark {
        id: ServiceMarkId::new("service-mark.2047.west-gate-breach").unwrap(),
        tournament_year_id: TournamentYearId::new("service-tournament.year.2047").unwrap(),
        war: mark.war.clone(),
        year: 2047,
        scenario: ScenarioId::new("scenario.thousand-hues.west-gate-breach").unwrap(),
        location: mark.location.clone(),
        houses: [House::Flynt].into_iter().collect(),
        operation_name: "West Gate Breach".into(),
        participants: [competitor_id(House::Flynt)].into_iter().collect(),
        significance: ServiceMarkSignificance::DecisiveBreach,
        ordered_paint_layers: vec![mark.id.clone()],
        constitutional_significance: "The breach preserved public access and restraint.".into(),
        account: "Flynt opened the controlled gate without crossing the civilian line.".into(),
        provenance: ServiceMarkProvenance {
            preservation_event: preservation_event.clone(),
            source_marks: [mark.id.clone()].into_iter().collect(),
            source_action_events: [mark_event.clone()].into_iter().collect(),
            authorized_by: TournamentAuthorityId::new(
                "authority.service-tournament.service-mark-keeper",
            )
            .unwrap(),
            evidence: [
                TournamentEvidenceId::new("evidence.service-mark.2047.west-gate-breach").unwrap(),
            ]
            .into_iter()
            .collect(),
        },
    };
    runtime
        .apply_event(event(
            7,
            TournamentEventKind::ServiceMarkPreserved(service_mark.clone()),
        ))
        .unwrap();

    let preserved = runtime.service_marks().get(&service_mark.id).unwrap();
    assert_eq!(
        preserved.provenance.source_marks,
        [mark.id].into_iter().collect()
    );
    assert_eq!(
        preserved.provenance.source_action_events,
        [mark_event].into_iter().collect()
    );
    assert!(!preserved.provenance.evidence.is_empty());
    assert_eq!(
        runtime
            .marks()
            .get(preserved.provenance.source_marks.first().unwrap())
            .unwrap()
            .provenance
            .competitor,
        competitor_id(House::Flynt)
    );
}

#[test]
fn tournament_result_cannot_transfer_permanent_sovereignty() {
    let mut valid_runtime = opened_runtime();
    valid_runtime
        .apply_event(event(5, TournamentEventKind::ResultRecorded(result(false))))
        .unwrap();
    assert_eq!(valid_runtime.results().len(), 1);
    assert!(
        valid_runtime
            .results()
            .values()
            .all(|record| !record.transfers_permanent_sovereignty)
    );

    let mut invalid_runtime = opened_runtime();
    assert!(matches!(
        invalid_runtime.apply_event(event(5, TournamentEventKind::ResultRecorded(result(true)))),
        Err(ServiceTournamentError::PermanentSovereigntyForbidden)
    ));
}

#[test]
fn scorecard_requires_every_service_and_constitutional_category() {
    let mut runtime = opened_runtime();
    let mut incomplete = result(false);
    incomplete
        .scorecards
        .get_mut(&House::Flynt)
        .unwrap()
        .scores
        .remove(&ScoringCategory::ConstitutionalRestraint);
    assert!(matches!(
        runtime.apply_event(event(5, TournamentEventKind::ResultRecorded(incomplete))),
        Err(ServiceTournamentError::IncompleteScorecard(_))
    ));
}

#[test]
fn paint_grammar_and_layer_order_form_a_readable_historical_chart() {
    let flynt = flynt_mark(TournamentEventId::new("event.mark.flynt").unwrap());
    let mut stonebend = flynt.clone();
    stonebend.id = MarkId::new("mark.thousand-hues.west-gate.stonebend-line").unwrap();
    stonebend.house = House::Stonebend;
    stonebend.color_family = HouseColorFamily::Blue;
    stonebend.palette_sources = ["stonebend.lapis_lazuli".into()].into_iter().collect();
    stonebend.hue_description = "lapis line over weathered black".into();
    stonebend.action = MarkAction::Held;
    stonebend.grammar = Some(MarkGrammar::StraightLine);
    stonebend.layer_sequence = 1;
    stonebend.provenance.competitor = competitor_id(House::Stonebend);

    let reading = service_tournament::read_paint_layers(&[&stonebend, &flynt]).unwrap();
    assert_eq!(reading.ordered_houses, vec![House::Flynt, House::Stonebend]);
    assert_eq!(
        reading.meaning,
        LayerMeaning::FlyntBreachThenStonebendSecurity
    );
    assert_eq!(MarkGrammar::Burst.meaning(), "Breach completed.");
    assert_eq!(
        MarkGrammar::Cross.meaning(),
        "Rescue, treatment, or medical extraction."
    );
}

#[test]
fn stable_caller_supplied_ids_make_replay_independent_of_input_collection_order() {
    let mut events = PairedServiceIdentity::ALL
        .into_iter()
        .enumerate()
        .map(|(sequence, identity)| {
            event(
                sequence as u64,
                TournamentEventKind::CompetitorRegistered(competitor(identity)),
            )
        })
        .collect::<Vec<_>>();
    events.push(event(4, TournamentEventKind::TournamentOpened));
    events.push(event(
        5,
        TournamentEventKind::ScenarioRegistered(scenario()),
    ));
    events.reverse();

    let replayed = ServiceTournamentRuntime::replay(&events).unwrap();
    assert!(replayed.is_open());
    assert!(
        replayed
            .scenarios()
            .contains_key(&ScenarioId::new("scenario.thousand-hues.west-gate-breach").unwrap())
    );
    assert!(
        replayed.objectives().contains_key(
            &service_tournament::ObjectiveId::new("objective.thousand-hues.west-gate-restraint")
                .unwrap()
        )
    );
}

#[test]
fn central_junction_and_existing_constitutional_architecture_remain_intact() {
    let district = canonical_central_junction();
    assert_eq!(district.stable_id, "district.central-junction");
    assert_eq!(
        district.institutions,
        CentralJunctionInstitution::ALL.into_iter().collect()
    );
    assert_eq!(
        district.public_functions,
        [CentralJunctionFunction::ServiceTournament]
            .into_iter()
            .collect()
    );
    assert!(CentralJunctionFunction::ServiceTournament.is_largest_public_function());
    assert_eq!(EconomicPole::Design.house(), House::Sandmanor);
    assert_eq!(EconomicPole::Engineering.house(), House::Flynt);
    assert_eq!(EconomicPole::Craft.house(), House::Stonebend);
    assert_eq!(EconomicPole::Repair.house(), House::Glaushouse);

    let tournament = service_tournament::canonical_service_tournament();
    assert!(!tournament.transfers_permanent_sovereignty);
    assert_eq!(tournament.constitutional_sources.len(), 5);
    assert!(
        tournament
            .constitutional_sources
            .contains("HOLLOW_GROVE_COMPROMISE_V1_DRAFT.md")
    );
}

#[test]
fn canonical_document_names_the_required_lore_and_runtime_contracts() {
    for required in [
        "The Service Tournament",
        "The War of a Thousand Hues",
        "Flynt, MI",
        "ATF & Army",
        "DEA & Air Force",
        "CIA & Navy",
        "FBI & Marines",
        "Stonebend | Blue",
        "Sandmanor | Red",
        "Glaüshouse | Green",
        "Flynt | Black",
        "This House was here.",
        "Service Marks",
        "nonlethal",
        "stable caller-supplied identifiers",
        "permanent sovereignty",
        "HOLLOW_GROVE_COMPROMISE_V1_DRAFT.md",
    ] {
        assert!(
            CANON.contains(required),
            "missing canonical fragment {required}"
        );
    }
    assert!(!CANON.contains("eight independent service factions"));
}

#[test]
fn incomplete_or_split_rosters_cannot_open() {
    let mut runtime = ServiceTournamentRuntime::canonical().unwrap();
    runtime
        .apply_event(event(
            0,
            TournamentEventKind::CompetitorRegistered(competitor(
                PairedServiceIdentity::FlyntAtfArmy,
            )),
        ))
        .unwrap();
    assert!(matches!(
        runtime.apply_event(event(1, TournamentEventKind::TournamentOpened)),
        Err(ServiceTournamentError::IncompleteFourHouseRoster)
    ));

    let mut duplicate = ServiceTournamentRuntime::canonical().unwrap();
    duplicate
        .apply_event(event(
            0,
            TournamentEventKind::CompetitorRegistered(competitor(
                PairedServiceIdentity::FlyntAtfArmy,
            )),
        ))
        .unwrap();
    let mut second_flynt = competitor(PairedServiceIdentity::FlyntAtfArmy);
    second_flynt.id = CompetitorId::new("competitor.service-tournament.flynt-army-split").unwrap();
    assert!(matches!(
        duplicate.apply_event(event(
            1,
            TournamentEventKind::CompetitorRegistered(second_flynt)
        )),
        Err(ServiceTournamentError::DuplicateHouseRepresentative(
            House::Flynt
        ))
    ));
}

#[test]
fn score_totals_are_multicategory_and_not_elimination_counts() {
    let scorecard = complete_scorecard(3);
    assert_eq!(scorecard.scores.len(), ScoringCategory::ALL.len(),);
    assert_eq!(
        scorecard.total_score(),
        3 * ScoringCategory::ALL.len() as u32
    );
    assert!(
        !scorecard
            .scores
            .keys()
            .any(|category| format!("{category:?}").contains("Elimination"))
    );
}

#[test]
fn fixed_registry_is_not_generated_from_map_or_insertion_position() {
    let identities = PairedServiceIdentity::ALL
        .into_iter()
        .map(|identity| (identity.house(), identity.paired_reference()))
        .collect::<BTreeMap<_, _>>();
    assert_eq!(identities[&House::Flynt], "ATF & Army");
    assert_eq!(identities[&House::Stonebend], "DEA & Air Force");
    assert_eq!(identities[&House::Sandmanor], "CIA & Navy");
    assert_eq!(identities[&House::Glaushouse], "FBI & Marines");
}
