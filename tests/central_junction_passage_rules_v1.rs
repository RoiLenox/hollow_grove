use hollow_grove::hollow_grove_contract::House;
use hollow_grove::world::current_sea_passage::{
    BOARDWALK_ROUTE_ID, CURRENT_SEA_REGION_ID, CurrentSeaForce, CurrentSeaForceDirection,
    CurrentSeaState, RIPTIDE_FORCE_ID, UNDERTOW_FORCE_ID,
};
use hollow_grove::world::seasonal_functions_archive::{
    SeasonalArchiveError, decode_seasonal_archive, encode_seasonal_archive,
};
use hollow_grove::world::seasonal_functions_fixture::canonical_seasonal_archive_fixture;
use hollow_grove::world::way_back::{
    AURA_WAY_ROUTE_ID, STAIRWAY_TO_HEAVEN_SEGMENT_ID, THE_WAY_BACK_RULE_ID, WayBackDirection,
    WayBackExpression, WayBackPassageId, WayBackRuntime, WayBackSupportRole,
};

fn decoded() -> hollow_grove::world::seasonal_functions_archive::DecodedSeasonalArchive {
    let bytes = encode_seasonal_archive(&canonical_seasonal_archive_fixture()).unwrap();
    decode_seasonal_archive(&bytes).unwrap()
}

fn cycle(
    archive: &hollow_grove::world::seasonal_functions_archive::DecodedSeasonalArchive,
) -> &hollow_grove::world::seasonal_functions_archive::CanonicalAnnualCycleState {
    archive.annual_cycles.values().next().unwrap()
}

#[test]
fn way_back_route_rule_function_and_stairway_have_distinct_stable_ids() {
    let archive = decoded();
    let state = cycle(&archive);
    let route = state.way_back_runtime.route();
    let winter = state.seasonal_runtime.resolve_name("The Way Back").unwrap();
    assert_eq!(route.route_id.as_str(), AURA_WAY_ROUTE_ID);
    assert_eq!(route.rule_id.as_str(), THE_WAY_BACK_RULE_ID);
    assert_eq!(
        route.stairway_segment_id.as_str(),
        STAIRWAY_TO_HEAVEN_SEGMENT_ID
    );
    assert_ne!(route.route_id.as_str(), route.rule_id.as_str());
    assert_ne!(route.route_id.as_str(), winter.function_id.as_str());
    assert_ne!(route.rule_id.as_str(), winter.function_id.as_str());
    assert!(route.valid_outside_winter_function);
}

#[test]
fn way_back_descends_stonebend_to_flynt_and_ascends_flynt_to_stonebend() {
    let archive = decoded();
    let passages = cycle(&archive).way_back_runtime.passages();
    let descent = passages
        .values()
        .find(|passage| passage.direction == WayBackDirection::DescendingFromStonebend)
        .unwrap();
    let ascent = passages
        .values()
        .find(|passage| passage.direction == WayBackDirection::AscendingFromFlynt)
        .unwrap();
    assert_eq!(descent.origin_house, House::Stonebend);
    assert_eq!(descent.destination_house, House::Flynt);
    assert_eq!(descent.expression, WayBackExpression::AuraWay);
    assert_eq!(ascent.origin_house, House::Flynt);
    assert_eq!(ascent.destination_house, House::Stonebend);
    assert_eq!(ascent.expression, WayBackExpression::StairwayToHeaven);
}

#[test]
fn stairway_to_heaven_is_ascent_only_and_rejects_descent() {
    let mut payload = canonical_seasonal_archive_fixture();
    let cycle = &mut payload.annual_cycles[0];
    let descent = cycle
        .way_back_passages
        .iter_mut()
        .find(|passage| passage.direction == WayBackDirection::DescendingFromStonebend)
        .unwrap();
    descent.expression = WayBackExpression::StairwayToHeaven;
    assert!(matches!(
        encode_seasonal_archive(&payload),
        Err(SeasonalArchiveError::WayBack(_))
    ));
}

#[test]
fn aura_way_supports_both_lawful_directions_without_inference() {
    let payload = canonical_seasonal_archive_fixture();
    let cycle = &payload.annual_cycles[0];
    let mut passages = cycle.way_back_passages.clone();
    let ascent = passages
        .iter_mut()
        .find(|passage| passage.direction == WayBackDirection::AscendingFromFlynt)
        .unwrap();
    ascent.passage_id = WayBackPassageId::new("passage.test.aura-way-ascent").unwrap();
    ascent.expression = WayBackExpression::AuraWay;
    WayBackRuntime::replay(cycle.way_back_route.clone(), &passages).unwrap();
}

#[test]
fn assisting_houses_tend_and_arrange_without_route_ownership() {
    let archive = decoded();
    let runtime = &cycle(&archive).way_back_runtime;
    assert_eq!(runtime.route().route_owner, None);
    assert!(runtime.passages().values().any(|passage| {
        passage.support.iter().any(|support| {
            support.house == House::Glaushouse
                && support.role == WayBackSupportRole::GlaushouseCareAndClearance
                && !support.claims_route_ownership
                && !passage.clearance_ids.is_empty()
        })
    }));
    assert!(runtime.passages().values().any(|passage| {
        passage.support.iter().any(|support| {
            support.house == House::Sandmanor
                && support.role == WayBackSupportRole::SandmanorArrangement
                && !support.claims_route_ownership
        })
    }));
}

#[test]
fn way_back_direction_origin_destination_and_provenance_survive_replay() {
    let first = decoded();
    let bytes = encode_seasonal_archive(&first.payload).unwrap();
    let second = decode_seasonal_archive(&bytes).unwrap();
    assert_eq!(
        cycle(&first).way_back_runtime,
        cycle(&second).way_back_runtime
    );
    assert!(
        cycle(&second)
            .way_back_runtime
            .passages()
            .values()
            .all(|passage| !passage.evidence_ids.is_empty()
                && !passage.provenance_id.as_str().is_empty())
    );
}

#[test]
fn way_back_direction_and_endpoint_tampering_are_rejected() {
    let mut wrong_direction = canonical_seasonal_archive_fixture();
    wrong_direction.annual_cycles[0].way_back_passages[0].direction =
        WayBackDirection::AscendingFromFlynt;
    assert!(matches!(
        encode_seasonal_archive(&wrong_direction),
        Err(SeasonalArchiveError::WayBack(_))
    ));

    let mut wrong_endpoint = canonical_seasonal_archive_fixture();
    wrong_endpoint.annual_cycles[0].way_back_passages[0].destination_house = House::Stonebend;
    assert!(matches!(
        encode_seasonal_archive(&wrong_endpoint),
        Err(SeasonalArchiveError::WayBack(_))
    ));
}

#[test]
fn current_sea_body_riptide_undertow_and_boardwalk_are_type_distinct() {
    assert_ne!(CURRENT_SEA_REGION_ID, RIPTIDE_FORCE_ID);
    assert_ne!(CURRENT_SEA_REGION_ID, BOARDWALK_ROUTE_ID);
    assert_ne!(RIPTIDE_FORCE_ID, BOARDWALK_ROUTE_ID);
    assert_ne!(RIPTIDE_FORCE_ID, UNDERTOW_FORCE_ID);
    assert_ne!(CurrentSeaForce::Riptide, CurrentSeaForce::Undertow);
}

#[test]
fn current_sea_sets_in_glaushouse_and_rises_toward_flynt_by_riptide() {
    let archive = decoded();
    let runtime = &cycle(&archive).current_sea_runtime;
    let setting = runtime
        .events()
        .values()
        .find(|event| event.state == CurrentSeaState::Setting)
        .unwrap();
    assert_eq!(setting.region_id.as_str(), CURRENT_SEA_REGION_ID);
    assert_eq!(setting.origin_house, House::Glaushouse);
    assert_eq!(setting.destination_house, None);

    for rise in runtime
        .events()
        .values()
        .filter(|event| event.state == CurrentSeaState::Rising)
    {
        assert_eq!(rise.force, Some(CurrentSeaForce::Riptide));
        assert_eq!(rise.force_id.as_ref().unwrap().as_str(), RIPTIDE_FORCE_ID);
        assert_eq!(rise.origin_house, House::Glaushouse);
        assert_eq!(rise.destination_house, Some(House::Flynt));
        assert_eq!(
            rise.force_direction,
            Some(CurrentSeaForceDirection::FromGlaushouseTowardFlynt)
        );
        assert!(rise.is_successful_rise());
    }
}

#[test]
fn riptide_can_rise_without_a_boardwalk_traveler() {
    let archive = decoded();
    let runtime = &cycle(&archive).current_sea_runtime;
    let associated = runtime
        .passages()
        .values()
        .filter_map(|passage| passage.associated_current_event_id.as_ref())
        .collect::<std::collections::BTreeSet<_>>();
    assert!(runtime.events().values().any(|event| {
        event.force == Some(CurrentSeaForce::Riptide) && !associated.contains(&event.event_id)
    }));
}

#[test]
fn boardwalk_carries_the_traveler_and_may_reference_but_never_be_riptide() {
    let archive = decoded();
    let runtime = &cycle(&archive).current_sea_runtime;
    let passage = runtime.passages().values().next().unwrap();
    assert_eq!(passage.route_id.as_str(), BOARDWALK_ROUTE_ID);
    assert_eq!(passage.origin_house, House::Glaushouse);
    assert_eq!(passage.destination_house, House::Flynt);
    let event = runtime
        .events()
        .get(passage.associated_current_event_id.as_ref().unwrap())
        .unwrap();
    assert_eq!(event.force, Some(CurrentSeaForce::Riptide));
    assert_ne!(passage.passage_id.as_str(), event.event_id.as_str());
}

#[test]
fn arrival_and_glaushouse_clearance_do_not_automatically_grant_recog() {
    let archive = decoded();
    let runtime = &cycle(&archive).current_sea_runtime;
    let passage = runtime.passages().values().next().unwrap();
    assert!(passage.arrived_at_destination);
    assert!(!passage.grants_automatic_recog);
    assert!(
        runtime
            .clearances()
            .values()
            .all(|clearance| !clearance.grants_flynt_recog)
    );

    let mut without_recog = canonical_seasonal_archive_fixture();
    without_recog.annual_cycles[0].flynt_recog_events.clear();
    let bytes = encode_seasonal_archive(&without_recog).unwrap();
    let replayed = decode_seasonal_archive(&bytes).unwrap();
    assert!(
        cycle(&replayed)
            .current_sea_runtime
            .recog_events()
            .is_empty()
    );
    assert!(!cycle(&replayed).current_sea_runtime.passages().is_empty());
}

#[test]
fn lawful_recog_requires_separate_accepted_resynce_and_manifestation_conditions() {
    let archive = decoded();
    let runtime = &cycle(&archive).current_sea_runtime;
    assert_eq!(runtime.resynce_events().len(), 1);
    assert_eq!(runtime.recog_events().len(), 1);
    assert!(
        runtime
            .resynce_events()
            .values()
            .all(|event| event.accepted)
    );
    assert!(
        runtime
            .recog_events()
            .values()
            .all(|event| { event.accepted && !event.manifestation_condition_ids.is_empty() })
    );

    let mut invalid = canonical_seasonal_archive_fixture();
    invalid.annual_cycles[0].flynt_recog_events[0]
        .manifestation_condition_ids
        .clear();
    assert!(matches!(
        encode_seasonal_archive(&invalid),
        Err(SeasonalArchiveError::CurrentSea(_))
    ));
}

#[test]
fn undertow_is_inverse_hazard_and_never_successful_emergence() {
    let archive = decoded();
    let hazard = cycle(&archive)
        .current_sea_runtime
        .events()
        .values()
        .find(|event| event.force == Some(CurrentSeaForce::Undertow))
        .unwrap();
    assert_eq!(hazard.state, CurrentSeaState::Disturbed);
    assert_eq!(
        hazard.force_direction,
        Some(CurrentSeaForceDirection::DownwardOrBeneath)
    );
    assert_eq!(hazard.destination_house, None);
    assert!(!hazard.is_successful_rise());
}

#[test]
fn current_force_and_boardwalk_identity_survive_archive_and_replay() {
    let first = decoded();
    let bytes = encode_seasonal_archive(&first.payload).unwrap();
    let second = decode_seasonal_archive(&bytes).unwrap();
    assert_eq!(
        cycle(&first).current_sea_runtime,
        cycle(&second).current_sea_runtime
    );
}

#[test]
fn force_direction_and_checksum_tampering_are_detected() {
    let mut force = canonical_seasonal_archive_fixture();
    let rise = force.annual_cycles[0]
        .current_sea_events
        .iter_mut()
        .find(|event| event.state == CurrentSeaState::Rising)
        .unwrap();
    rise.force = Some(CurrentSeaForce::Undertow);
    assert!(matches!(
        encode_seasonal_archive(&force),
        Err(SeasonalArchiveError::CurrentSea(_))
    ));

    let mut direction = canonical_seasonal_archive_fixture();
    let rise = direction.annual_cycles[0]
        .current_sea_events
        .iter_mut()
        .find(|event| event.state == CurrentSeaState::Rising)
        .unwrap();
    rise.force_direction = Some(CurrentSeaForceDirection::DownwardOrBeneath);
    assert!(matches!(
        encode_seasonal_archive(&direction),
        Err(SeasonalArchiveError::CurrentSea(_))
    ));

    let bytes = encode_seasonal_archive(&canonical_seasonal_archive_fixture()).unwrap();
    let text = String::from_utf8(bytes).unwrap();
    let tampered = text.replace("\"Riptide\"", "\"Undertow\"");
    assert!(matches!(
        decode_seasonal_archive(tampered.as_bytes()),
        Err(SeasonalArchiveError::ChecksumMismatch)
    ));
}

#[test]
fn opposite_insertion_order_does_not_merge_force_route_or_reverse_passages() {
    let payload = canonical_seasonal_archive_fixture();
    let canonical = encode_seasonal_archive(&payload).unwrap();
    let mut reverse = payload;
    let cycle = &mut reverse.annual_cycles[0];
    cycle.way_back_passages.reverse();
    cycle.current_sea_events.reverse();
    cycle.glaushouse_clearances.reverse();
    cycle.boardwalk_passages.reverse();
    cycle.flynt_resynce_events.reverse();
    cycle.flynt_recog_events.reverse();
    assert_eq!(encode_seasonal_archive(&reverse).unwrap(), canonical);
}

#[test]
fn universal_kernel_contains_no_house_specific_passage_lore() {
    const KERNEL: &str = include_str!("../hollow-grove-kernel/src/lib.rs");
    const ORIENTED: &str = include_str!("../hollow-grove-kernel/src/oriented_point.rs");
    for forbidden in [
        "Way Back",
        "Aura Way",
        "Stairway to Heaven",
        "Current Sea",
        "Riptide",
        "Undertow",
        "Boardwalk",
        "Glaüshouse",
        "Flynt",
    ] {
        assert!(!KERNEL.contains(forbidden));
        assert!(!ORIENTED.contains(forbidden));
    }
}
