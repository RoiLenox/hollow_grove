use std::collections::{BTreeMap, BTreeSet, VecDeque};

use hollow_grove::constitutional::{
    CausalPosition, ConstitutionalRuntime, ParticipantId, RuleSetId, V2_RULE_SET,
};
use hollow_grove::gameplay::{
    BeingContinuityId, CardinalDirection, CharacterCondition, DeepPressureError, DeepPressureEvent,
    DeepPressureEvidenceId, DeepPressureOutcomeId, DeepPressurePersonId, DeepPressurePhase,
    DeepPressureSettlementChoice, DeepPressureState, DeepPressureStatementId,
    GameApplicationService, GameplayCommand, GameplayEvent, GameplayEventId, GameplayEventMetadata,
    InteractionId, LivingCaseChoice, LivingCaseId, LivingWorldState, MAP_HEIGHT, MAP_WIDTH,
    PartyActionId, PartyActorId, RecruitmentCandidateId, RecruitmentPath, SpeechClassification,
    TilePosition, WorldMapId, deep_pressure_functional_lore, map_definition,
    scheduled_people_on_map,
};
use hollow_grove::institution::InstitutionalBeingId;
use hollow_grove::world::extraction::ExtractionSiteId;
use hollow_grove::world::geography::ConstitutionalRouteId;
use hollow_grove::world::session::WorldSession;

fn rules() -> RuleSetId {
    RuleSetId::new(V2_RULE_SET).unwrap()
}

fn authority_world() -> WorldSession {
    WorldSession::from_persisted_output(include_str!("../artifacts/institutional_state.txt"))
        .unwrap()
}

fn metadata(sequence: u64) -> GameplayEventMetadata {
    GameplayEventMetadata {
        id: GameplayEventId::new(format!("game-event.deep-pressure.{sequence}")).unwrap(),
        causal_position: CausalPosition::new(sequence),
    }
}

fn establish(service: &mut GameApplicationService) {
    service
        .execute(
            metadata(1),
            GameplayCommand::EstablishHuemanIdentity {
                continuity: BeingContinuityId::new("being-continuity.hueman").unwrap(),
                participant: ParticipantId::new("participant.hueman").unwrap(),
                institutional: InstitutionalBeingId::new("being.hueman").unwrap(),
            },
        )
        .unwrap();
}

fn integrate_case(
    living: &mut LivingWorldState,
    campaign: &mut DeepPressureState,
    case_id: LivingCaseId,
    choice: LivingCaseChoice,
    campaign_events: &mut Vec<DeepPressureEvent>,
) {
    for evidence in case_id.required_evidence() {
        let event = living.observe(case_id, *evidence).unwrap();
        let linked = campaign
            .observe_living_event(&event, living.clock)
            .unwrap()
            .unwrap();
        campaign_events.push(linked);
    }
    let support = living.support(case_id, choice).unwrap();
    assert!(
        campaign
            .observe_living_event(&support, living.clock)
            .unwrap()
            .is_none()
    );
    let resolved = living.commit_duty_decision(case_id).unwrap();
    let linked = campaign
        .observe_living_event(&resolved, living.clock)
        .unwrap()
        .unwrap();
    campaign_events.push(linked);
}

fn complete_operational_arc(
    riptide_choice: LivingCaseChoice,
    certification_choice: LivingCaseChoice,
) -> (LivingWorldState, DeepPressureState, Vec<DeepPressureEvent>) {
    let mut living = LivingWorldState::canonical().unwrap();
    let mut campaign = DeepPressureState::new();
    let mut events = Vec::new();
    integrate_case(
        &mut living,
        &mut campaign,
        LivingCaseId::RiptideWellBlowout,
        riptide_choice,
        &mut events,
    );
    assert_eq!(campaign.phase(), DeepPressurePhase::ShorelineAftermath);
    integrate_case(
        &mut living,
        &mut campaign,
        LivingCaseId::AuraBeachStormRescue,
        LivingCaseChoice::CloseAndShelter,
        &mut events,
    );
    integrate_case(
        &mut living,
        &mut campaign,
        LivingCaseId::AuraBasinInjuredBeing,
        LivingCaseChoice::TransferToCare,
        &mut events,
    );
    assert_eq!(campaign.phase(), DeepPressurePhase::BurdenOfRepair);
    integrate_case(
        &mut living,
        &mut campaign,
        LivingCaseId::AuraFieldDroughtAllocation,
        LivingCaseChoice::EquitableRation,
        &mut events,
    );
    integrate_case(
        &mut living,
        &mut campaign,
        LivingCaseId::MntAuraRoofFall,
        LivingCaseChoice::ReinforceAndContinue,
        &mut events,
    );
    integrate_case(
        &mut living,
        &mut campaign,
        LivingCaseId::HighwayToHellGasPocket,
        LivingCaseChoice::SealAndVent,
        &mut events,
    );
    assert_eq!(campaign.phase(), DeepPressurePhase::DepthCertification);
    integrate_case(
        &mut living,
        &mut campaign,
        LivingCaseId::CurrentSeaWellCertification,
        certification_choice,
        &mut events,
    );
    assert_eq!(campaign.phase(), DeepPressurePhase::GatherAffectedVoices);
    (living, campaign, events)
}

fn gather_required_statements(
    living: &LivingWorldState,
    campaign: &mut DeepPressureState,
    events: &mut Vec<DeepPressureEvent>,
) {
    for statement in DeepPressureStatementId::REQUIRED {
        let event = campaign
            .observe_statement(statement, living.clock)
            .unwrap()
            .unwrap();
        events.push(event);
    }
}

#[test]
fn functional_lore_contract_has_all_twelve_fields_and_classified_speech() {
    deep_pressure_functional_lore().validate().unwrap();
    let mut state = DeepPressureState::new();
    let clock = LivingWorldState::canonical().unwrap().clock;
    state
        .observe_statement(DeepPressureStatementId::BoardwalkPimpPitch, clock)
        .unwrap();
    state
        .observe_statement(
            DeepPressureStatementId::Person(DeepPressurePersonId::TessBreakwater),
            clock,
        )
        .unwrap();
    assert_eq!(
        state
            .journal
            .iter()
            .find(|record| {
                record.evidence_id
                    == DeepPressureEvidenceId::Statement(
                        DeepPressureStatementId::BoardwalkPimpPitch,
                    )
            })
            .unwrap()
            .speech_classification,
        Some(SpeechClassification::DeliberateDeception)
    );
    assert_eq!(
        state
            .journal
            .iter()
            .find(|record| record.source == "Tess Breakwater")
            .unwrap()
            .speech_classification,
        Some(SpeechClassification::Rumor)
    );
}

#[test]
fn the_full_arc_forms_a_finite_recovery_bond_and_replays_exactly() {
    let (living, mut campaign, mut events) = complete_operational_arc(
        LivingCaseChoice::ShutInAndRetrieve,
        LivingCaseChoice::CertifyReducedRate,
    );
    gather_required_statements(&living, &mut campaign, &mut events);
    assert_eq!(campaign.phase(), DeepPressurePhase::BoardwalkSettlement);
    assert!(campaign.ready_for_settlement_support());

    let support = campaign
        .support_settlement(DeepPressureSettlementChoice::SharedBurdenCompact)
        .unwrap();
    events.push(support);
    let mut constitutional = ConstitutionalRuntime::new();
    let committed = campaign
        .commit_settlement(
            CausalPosition::new(500),
            &rules(),
            &mut constitutional,
            &authority_world(),
        )
        .unwrap();
    events.push(committed);

    assert_eq!(campaign.phase(), DeepPressurePhase::PersistentAftermath);
    let outcome = campaign.outcome.as_ref().unwrap();
    assert_eq!(outcome.id, DeepPressureOutcomeId::SharedBurdenCompactV1);
    assert_eq!(outcome.four_house_acts.len(), 4);
    assert_eq!(
        outcome
            .four_house_acts
            .iter()
            .map(|act| act.function.as_str())
            .collect::<Vec<_>>(),
        vec!["Name", "Prove", "Clear", "Recognize"]
    );
    let bond = outcome.recovery_bond.as_ref().unwrap();
    assert_eq!(bond.participants.len(), DeepPressurePersonId::ALL.len());
    assert_eq!(constitutional.events().len(), 3);
    assert!(outcome.aftermath.crew_care > 50);
    assert!(outcome.aftermath.coast_recovery > 50);
    assert!(
        campaign
            .relationships
            .values()
            .all(|memory| memory.constitutional_bond.as_deref() == Some(&bond.bond_id))
    );

    let mut replayed = DeepPressureState::new();
    for event in &events {
        replayed.apply(event).unwrap();
    }
    assert_eq!(replayed, campaign);
}

#[test]
fn compromised_riptide_evidence_bars_only_the_production_ending_transactionally() {
    let (living, mut campaign, mut events) = complete_operational_arc(
        LivingCaseChoice::RescueCrewFirst,
        LivingCaseChoice::CertifyReducedRate,
    );
    gather_required_statements(&living, &mut campaign, &mut events);
    assert!(campaign.contested_certificate());
    assert!(
        !campaign.settlement_choice_available(DeepPressureSettlementChoice::ProductionUnderReview)
    );
    let before = campaign.clone();
    assert_eq!(
        campaign
            .support_settlement(DeepPressureSettlementChoice::ProductionUnderReview)
            .unwrap_err(),
        DeepPressureError::CompromisedEvidenceBarsProductionAccord
    );
    assert_eq!(campaign, before);

    campaign
        .support_settlement(DeepPressureSettlementChoice::ProtectedRefusal)
        .unwrap();
    let mut constitutional = ConstitutionalRuntime::new();
    campaign
        .commit_settlement(
            CausalPosition::new(600),
            &rules(),
            &mut constitutional,
            &authority_world(),
        )
        .unwrap();
    let outcome = campaign.outcome.as_ref().unwrap();
    assert_eq!(outcome.id, DeepPressureOutcomeId::ProtectedRefusalV1);
    assert!(outcome.recovery_bond.is_none());
    assert!(outcome.aftermath.contested_certificate);
    assert_eq!(constitutional.events().len(), 0);
}

#[test]
fn local_choices_change_named_people_instead_of_only_changing_scores() {
    let (_, campaign, _) = complete_operational_arc(
        LivingCaseChoice::RescueCrewFirst,
        LivingCaseChoice::SuspendForRepair,
    );
    let corin = campaign
        .relationships
        .get(&DeepPressurePersonId::CorinWake)
        .unwrap();
    assert_eq!(corin.condition, CharacterCondition::Exposed);
    assert!(
        corin
            .unresolved_promises
            .contains("return to Riptide and complete the shut-in")
    );
    let iona = campaign
        .relationships
        .get(&DeepPressurePersonId::IonaDepth)
        .unwrap();
    assert_eq!(iona.reliability, 3);
    assert!(
        campaign
            .relationships
            .get(&DeepPressurePersonId::TessBreakwater)
            .unwrap()
            .remembered_outcomes
            .len()
            >= 3
    );
}

#[test]
fn scheduled_people_move_between_real_maps_on_shift_boundaries() {
    let mut living = LivingWorldState::canonical().unwrap();
    let riptide = WorldMapId::ExtractionSite(ExtractionSiteId::RiptideRecoveryRig);
    let beach = WorldMapId::AuraBeachCoastalCommons;
    let dawn_rig = scheduled_people_on_map(&living, riptide);
    assert!(
        dawn_rig
            .iter()
            .any(|person| person.person_id == DeepPressurePersonId::CorinWake)
    );
    assert_eq!(
        dawn_rig
            .iter()
            .map(|person| (person.position.x, person.position.y))
            .collect::<BTreeSet<_>>()
            .len(),
        dawn_rig.len()
    );
    for _ in 0..2 {
        living.advance_shift().unwrap();
    }
    let dusk_rig = scheduled_people_on_map(&living, riptide);
    let dusk_beach = scheduled_people_on_map(&living, beach);
    assert!(
        !dusk_rig
            .iter()
            .any(|person| person.person_id == DeepPressurePersonId::CorinWake)
    );
    assert!(
        dusk_beach
            .iter()
            .any(|person| person.person_id == DeepPressurePersonId::CorinWake)
    );
}

fn path_to_adjacent(
    service: &GameApplicationService,
    target: TilePosition,
) -> (Vec<CardinalDirection>, CardinalDirection) {
    let map = map_definition(service.runtime().hueman_map());
    let start = service.runtime().hueman_position().unwrap();
    let occupied = scheduled_people_on_map(
        service.runtime().living_world(),
        service.runtime().hueman_map(),
    )
    .into_iter()
    .map(|person| (person.position.x, person.position.y))
    .collect::<BTreeSet<_>>();
    let approaches = [
        (
            (target.x, target.y.saturating_add(1)),
            CardinalDirection::North,
        ),
        (
            (target.x.saturating_sub(1), target.y),
            CardinalDirection::East,
        ),
        (
            (target.x.saturating_add(1), target.y),
            CardinalDirection::West,
        ),
        (
            (target.x, target.y.saturating_sub(1)),
            CardinalDirection::South,
        ),
    ]
    .into_iter()
    .filter(|((x, y), _)| *x < MAP_WIDTH && *y < MAP_HEIGHT)
    .filter(|((x, y), _)| {
        matches!(
            map.rows[usize::from(*y)].as_bytes()[usize::from(*x)],
            b'.' | b'='
        ) && !occupied.contains(&(*x, *y))
    })
    .collect::<BTreeMap<_, _>>();

    let start_xy = (start.x, start.y);
    let mut frontier = VecDeque::from([start_xy]);
    let mut previous: BTreeMap<(u16, u16), ((u16, u16), CardinalDirection)> = BTreeMap::new();
    let mut visited = BTreeSet::from([start_xy]);
    let mut destination = None;
    while let Some(position) = frontier.pop_front() {
        if approaches.contains_key(&position) {
            destination = Some(position);
            break;
        }
        for (next, direction) in [
            (
                (position.0, position.1.wrapping_sub(1)),
                CardinalDirection::North,
            ),
            (
                (position.0.saturating_add(1), position.1),
                CardinalDirection::East,
            ),
            (
                (position.0, position.1.saturating_add(1)),
                CardinalDirection::South,
            ),
            (
                (position.0.wrapping_sub(1), position.1),
                CardinalDirection::West,
            ),
        ] {
            if next.0 >= MAP_WIDTH || next.1 >= MAP_HEIGHT || occupied.contains(&next) {
                continue;
            }
            let tile = map.rows[usize::from(next.1)].as_bytes()[usize::from(next.0)];
            if matches!(tile, b'.' | b'=') && visited.insert(next) {
                previous.insert(next, (position, direction));
                frontier.push_back(next);
            }
        }
    }
    let destination = destination.expect("a scheduled person has a reachable approach");
    let facing = approaches[&destination];
    let mut directions = Vec::new();
    let mut cursor = destination;
    while cursor != start_xy {
        let (parent, direction) = previous[&cursor];
        directions.push(direction);
        cursor = parent;
    }
    directions.reverse();
    (directions, facing)
}

fn enter_map(service: &mut GameApplicationService, sequence: &mut u64, map: WorldMapId) {
    service
        .execute(metadata(*sequence), GameplayCommand::EnterMap { map })
        .unwrap();
    *sequence += 1;
}

fn interact_at(
    service: &mut GameApplicationService,
    sequence: &mut u64,
    target_x: u16,
    target_y: u16,
) {
    let target = TilePosition {
        x: target_x,
        y: target_y,
        facing: CardinalDirection::South,
    };
    let (path, facing) = path_to_adjacent(service, target);
    for direction in path {
        service
            .execute(
                metadata(*sequence),
                GameplayCommand::MoveHueman { direction },
            )
            .unwrap();
        *sequence += 1;
    }
    service
        .execute(
            metadata(*sequence),
            GameplayCommand::MoveHueman { direction: facing },
        )
        .unwrap();
    *sequence += 1;
    service
        .execute(metadata(*sequence), GameplayCommand::InteractHueman)
        .unwrap();
    *sequence += 1;
}

fn hear_person(
    service: &mut GameApplicationService,
    sequence: &mut u64,
    person_id: DeepPressurePersonId,
) {
    let person = scheduled_people_on_map(
        service.runtime().living_world(),
        service.runtime().hueman_map(),
    )
    .into_iter()
    .find(|person| person.person_id == person_id)
    .unwrap_or_else(|| panic!("{person_id:?} is not present on the current map"));
    interact_at(service, sequence, person.position.x, person.position.y);
}

fn resolve_runtime_case(
    service: &mut GameApplicationService,
    sequence: &mut u64,
    case_id: LivingCaseId,
    choice: LivingCaseChoice,
) {
    service
        .execute(
            metadata(*sequence),
            GameplayCommand::SupportLivingCase { case_id, choice },
        )
        .unwrap();
    *sequence += 1;
    service
        .execute(
            metadata(*sequence),
            GameplayCommand::AskLivingDutyOfficerToDecide { case_id },
        )
        .unwrap();
    *sequence += 1;
}

#[test]
fn a_visible_scheduled_person_can_be_heard_and_the_journal_survives_archive_replay() {
    let mut service = GameApplicationService::new(rules());
    establish(&mut service);
    service
        .execute(
            metadata(2),
            GameplayCommand::EnterMap {
                map: WorldMapId::for_route(ConstitutionalRouteId::Riptide),
            },
        )
        .unwrap();
    service
        .execute(
            metadata(3),
            GameplayCommand::EnterMap {
                map: WorldMapId::ExtractionSite(ExtractionSiteId::RiptideRecoveryRig),
            },
        )
        .unwrap();
    let corin = scheduled_people_on_map(
        service.runtime().living_world(),
        service.runtime().hueman_map(),
    )
    .into_iter()
    .find(|person| person.person_id == DeepPressurePersonId::CorinWake)
    .unwrap();
    let (path, facing) = path_to_adjacent(&service, corin.position);
    let mut sequence = 4;
    for direction in path {
        service
            .execute(
                metadata(sequence),
                GameplayCommand::MoveHueman { direction },
            )
            .unwrap();
        sequence += 1;
    }
    service
        .execute(
            metadata(sequence),
            GameplayCommand::MoveHueman { direction: facing },
        )
        .unwrap();
    sequence += 1;
    let event = service
        .execute(metadata(sequence), GameplayCommand::InteractHueman)
        .unwrap();
    assert!(matches!(
        event.payload,
        GameplayEvent::HuemanInteractionOpened {
            target: InteractionId::DeepPressurePerson(DeepPressurePersonId::CorinWake),
            deep_pressure_event: Some(DeepPressureEvent::EvidenceJournaled { .. }),
            ..
        }
    ));
    assert!(
        service
            .runtime()
            .deep_pressure()
            .journal_contains(DeepPressureEvidenceId::Statement(
                DeepPressureStatementId::Person(DeepPressurePersonId::CorinWake)
            ))
    );

    let archive = service.encode_archive().unwrap();
    let replayed = GameApplicationService::from_archive(&archive).unwrap();
    assert_eq!(
        replayed.runtime().deep_pressure(),
        service.runtime().deep_pressure()
    );
    assert_eq!(replayed.encode_archive().unwrap(), archive);
}

#[test]
fn the_entire_playable_campaign_commits_and_replays_through_the_gameplay_archive() {
    let mut service = GameApplicationService::with_world_session(rules(), authority_world());
    establish(&mut service);
    let mut sequence = 2;

    enter_map(
        &mut service,
        &mut sequence,
        WorldMapId::for_route(ConstitutionalRouteId::Riptide),
    );
    enter_map(
        &mut service,
        &mut sequence,
        WorldMapId::ExtractionSite(ExtractionSiteId::RiptideRecoveryRig),
    );
    hear_person(&mut service, &mut sequence, DeepPressurePersonId::CorinWake);
    for target in [(9, 4), (5, 10), (14, 10)] {
        interact_at(&mut service, &mut sequence, target.0, target.1);
    }
    resolve_runtime_case(
        &mut service,
        &mut sequence,
        LivingCaseId::RiptideWellBlowout,
        LivingCaseChoice::ShutInAndRetrieve,
    );

    enter_map(
        &mut service,
        &mut sequence,
        WorldMapId::for_route(ConstitutionalRouteId::Riptide),
    );
    enter_map(
        &mut service,
        &mut sequence,
        WorldMapId::AuraBasinCollisionGrounds,
    );
    hear_person(
        &mut service,
        &mut sequence,
        DeepPressurePersonId::HarrowVale,
    );
    for target in [(8, 13), (2, 15), (5, 15)] {
        interact_at(&mut service, &mut sequence, target.0, target.1);
    }
    resolve_runtime_case(
        &mut service,
        &mut sequence,
        LivingCaseId::AuraBasinInjuredBeing,
        LivingCaseChoice::TransferToCare,
    );

    enter_map(
        &mut service,
        &mut sequence,
        WorldMapId::for_route(ConstitutionalRouteId::Riptide),
    );
    enter_map(
        &mut service,
        &mut sequence,
        WorldMapId::for_route(ConstitutionalRouteId::AuraRidge),
    );
    enter_map(
        &mut service,
        &mut sequence,
        WorldMapId::AuraBeachCoastalCommons,
    );
    hear_person(
        &mut service,
        &mut sequence,
        DeepPressurePersonId::TessBreakwater,
    );
    hear_person(
        &mut service,
        &mut sequence,
        DeepPressurePersonId::SellaWindward,
    );
    for target in [(12, 3), (8, 3), (14, 7)] {
        interact_at(&mut service, &mut sequence, target.0, target.1);
    }
    resolve_runtime_case(
        &mut service,
        &mut sequence,
        LivingCaseId::AuraBeachStormRescue,
        LivingCaseChoice::CloseAndShelter,
    );

    enter_map(
        &mut service,
        &mut sequence,
        WorldMapId::for_route(ConstitutionalRouteId::AuraRidge),
    );
    enter_map(
        &mut service,
        &mut sequence,
        WorldMapId::AuraFieldWorkingLand,
    );
    hear_person(
        &mut service,
        &mut sequence,
        DeepPressurePersonId::BrindleReed,
    );
    for target in [(7, 4), (11, 5), (14, 11)] {
        interact_at(&mut service, &mut sequence, target.0, target.1);
    }
    resolve_runtime_case(
        &mut service,
        &mut sequence,
        LivingCaseId::AuraFieldDroughtAllocation,
        LivingCaseChoice::EquitableRation,
    );

    enter_map(
        &mut service,
        &mut sequence,
        WorldMapId::for_route(ConstitutionalRouteId::MntAura),
    );
    enter_map(
        &mut service,
        &mut sequence,
        WorldMapId::ExtractionSite(ExtractionSiteId::MntAuraHighMine),
    );
    hear_person(&mut service, &mut sequence, DeepPressurePersonId::OrenPike);
    hear_person(&mut service, &mut sequence, DeepPressurePersonId::PelMarrow);
    for target in [(3, 2), (3, 10), (14, 10)] {
        interact_at(&mut service, &mut sequence, target.0, target.1);
    }
    resolve_runtime_case(
        &mut service,
        &mut sequence,
        LivingCaseId::MntAuraRoofFall,
        LivingCaseChoice::ReinforceAndContinue,
    );

    enter_map(
        &mut service,
        &mut sequence,
        WorldMapId::for_route(ConstitutionalRouteId::MntAura),
    );
    enter_map(
        &mut service,
        &mut sequence,
        WorldMapId::for_route(ConstitutionalRouteId::StairwayToHeaven),
    );
    enter_map(
        &mut service,
        &mut sequence,
        WorldMapId::ExtractionSite(ExtractionSiteId::StairwayBurdenMine),
    );
    hear_person(
        &mut service,
        &mut sequence,
        DeepPressurePersonId::BramBurden,
    );
    enter_map(
        &mut service,
        &mut sequence,
        WorldMapId::ExtractionSite(ExtractionSiteId::HighwayToHellDeepworks),
    );
    hear_person(
        &mut service,
        &mut sequence,
        DeepPressurePersonId::MaelaDownroad,
    );
    for target in [(3, 10), (3, 5), (14, 10)] {
        interact_at(&mut service, &mut sequence, target.0, target.1);
    }
    resolve_runtime_case(
        &mut service,
        &mut sequence,
        LivingCaseId::HighwayToHellGasPocket,
        LivingCaseChoice::SealAndVent,
    );

    enter_map(
        &mut service,
        &mut sequence,
        WorldMapId::ExtractionSite(ExtractionSiteId::StairwayBurdenMine),
    );
    enter_map(
        &mut service,
        &mut sequence,
        WorldMapId::for_route(ConstitutionalRouteId::StairwayToHeaven),
    );
    enter_map(
        &mut service,
        &mut sequence,
        WorldMapId::for_route(ConstitutionalRouteId::CurrentSea),
    );
    enter_map(
        &mut service,
        &mut sequence,
        WorldMapId::ExtractionSite(ExtractionSiteId::CurrentSeaDepthRig),
    );
    hear_person(&mut service, &mut sequence, DeepPressurePersonId::IonaDepth);
    for target in [(9, 4), (9, 12), (14, 14)] {
        interact_at(&mut service, &mut sequence, target.0, target.1);
    }
    resolve_runtime_case(
        &mut service,
        &mut sequence,
        LivingCaseId::CurrentSeaWellCertification,
        LivingCaseChoice::CertifyReducedRate,
    );

    enter_map(
        &mut service,
        &mut sequence,
        WorldMapId::for_route(ConstitutionalRouteId::CurrentSea),
    );
    enter_map(
        &mut service,
        &mut sequence,
        WorldMapId::for_route(ConstitutionalRouteId::Boardwalk),
    );
    interact_at(&mut service, &mut sequence, 4, 4);
    interact_at(&mut service, &mut sequence, 14, 4);
    assert_eq!(
        service.runtime().deep_pressure().phase(),
        DeepPressurePhase::BoardwalkSettlement
    );
    service
        .execute(
            metadata(sequence),
            GameplayCommand::SupportDeepPressureSettlement {
                choice: DeepPressureSettlementChoice::SharedBurdenCompact,
            },
        )
        .unwrap();
    sequence += 1;
    service
        .execute(
            metadata(sequence),
            GameplayCommand::AskDeepPressureAssemblyToCommit,
        )
        .unwrap();
    sequence += 1;

    assert_eq!(
        service.runtime().deep_pressure().phase(),
        DeepPressurePhase::PersistentAftermath
    );
    assert_eq!(service.runtime().constitutional().events().len(), 3);

    // The completed campaign now opens bounded recruitment. Brindle remains
    // physically present at Dawn until she decides to join; after acceptance
    // she leaves the work schedule projection and becomes a party member.
    enter_map(
        &mut service,
        &mut sequence,
        WorldMapId::for_route(ConstitutionalRouteId::CurrentSea),
    );
    enter_map(
        &mut service,
        &mut sequence,
        WorldMapId::for_route(ConstitutionalRouteId::StairwayToHeaven),
    );
    enter_map(
        &mut service,
        &mut sequence,
        WorldMapId::for_route(ConstitutionalRouteId::MntAura),
    );
    enter_map(
        &mut service,
        &mut sequence,
        WorldMapId::AuraFieldWorkingLand,
    );
    hear_person(
        &mut service,
        &mut sequence,
        DeepPressurePersonId::BrindleReed,
    );
    service
        .execute(
            metadata(sequence),
            GameplayCommand::RecruitPartyCandidate {
                candidate: RecruitmentCandidateId::FieldEngagementSteward,
                path: RecruitmentPath::SharedWork,
            },
        )
        .unwrap();
    sequence += 1;
    assert!(
        service
            .runtime()
            .party()
            .is_recruited(RecruitmentCandidateId::FieldEngagementSteward)
    );
    assert!(
        !service
            .runtime()
            .scheduled_people()
            .iter()
            .any(|presence| presence.person_id == DeepPressurePersonId::BrindleReed)
    );
    let brindle = PartyActorId::Companion(RecruitmentCandidateId::FieldEngagementSteward);
    service
        .execute(
            metadata(sequence),
            GameplayCommand::SwitchPartyLead { actor: brindle },
        )
        .unwrap();
    sequence += 1;
    service
        .execute(
            metadata(sequence),
            GameplayCommand::UsePartyAction {
                actor: brindle,
                action: PartyActionId::ReadEngagementWork,
                target_continuity_id: None,
            },
        )
        .unwrap();
    assert!(
        service.runtime().party().field_actions[0]
            .finding
            .contains("Engagement Farm")
    );

    let archive = service.encode_archive().unwrap();
    let replayed = GameApplicationService::from_archive(&archive).unwrap();
    assert_eq!(
        replayed.runtime().deep_pressure(),
        service.runtime().deep_pressure()
    );
    assert_eq!(
        replayed.runtime().living_world(),
        service.runtime().living_world()
    );
    assert_eq!(
        replayed.runtime().constitutional().events(),
        service.runtime().constitutional().events()
    );
    assert_eq!(replayed.runtime().party(), service.runtime().party());
    assert_eq!(replayed.encode_archive().unwrap(), archive);
}
