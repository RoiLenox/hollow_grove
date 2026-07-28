use std::collections::BTreeSet;

use hollow_grove::constitutional::SeasonalAnchor;
use hollow_grove::hollow_grove_contract::House;
use hollow_grove::world::central_junction_seasonal_functions::{
    CANONICAL_CEREMONIAL_SEQUENCE, FunctionActivity, FunctionIntensity, GreatFunctionKind,
    IncarnationalPrinciple, SacredMotion, UNIVERSAL_HUMAN_SEQUENCE, UniversalHumanMotion,
};
use hollow_grove::world::seasonal_functions_archive::{
    SEASONAL_ARCHIVE_VERSION, SeasonalArchiveError, decode_seasonal_archive,
    encode_legacy_seasonal_archive_v0, encode_seasonal_archive, migrate_seasonal_archive,
};
use hollow_grove::world::seasonal_functions_fixture::{
    canonical_calendar_year_fixture, canonical_seasonal_archive_fixture,
};

const CANON: &str = include_str!("../CENTRAL_JUNCTION_SEASONAL_FUNCTIONS_V1.md");

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
fn canonical_year_begins_at_winter_solstice_and_closes_at_the_next_one() {
    let year = canonical_calendar_year_fixture();
    year.validate().unwrap();
    let winter = year.observation(SeasonalAnchor::WinterSolstice).unwrap();
    assert_eq!(year.opens_at, winter.astronomical_instant);
    assert_eq!(year.closes_at, year.boundary.next_winter_solstice);
    assert!(year.boundary.closes_previous_and_opens_next);
    assert_ne!(year.id, year.boundary.next_year_id);
}

#[test]
fn exactly_four_great_functions_have_fixed_anchor_house_and_name() {
    let archive = decoded();
    let runtime = &cycle(&archive).seasonal_runtime;
    assert_eq!(runtime.functions().len(), 4);
    for (kind, house, name) in [
        (
            GreatFunctionKind::WayBack,
            House::Glaushouse,
            "The Way Back",
        ),
        (
            GreatFunctionKind::Initiation,
            House::Stonebend,
            "The Initiation",
        ),
        (
            GreatFunctionKind::Gathering,
            House::Sandmanor,
            "The Gathering",
        ),
        (
            GreatFunctionKind::FestivalOfMirrors,
            House::Flynt,
            "The Festival of Mirrors",
        ),
    ] {
        let function = runtime.function_at_anchor(kind.anchor()).unwrap();
        assert_eq!(function.kind, kind);
        assert_eq!(function.presiding_house, house);
        assert_eq!(function.canonical_name, name);
    }
}

#[test]
fn derrick_resolves_to_the_gathering_identity_without_a_fifth_function() {
    let archive = decoded();
    let runtime = &cycle(&archive).seasonal_runtime;
    let gathering = runtime.resolve_name("The Gathering").unwrap();
    let derrick = runtime.resolve_name("Derrick").unwrap();
    assert_eq!(gathering.function_id, derrick.function_id);
    assert_eq!(gathering.kind, GreatFunctionKind::Gathering);
    assert_eq!(runtime.functions().len(), 4);
}

#[test]
fn sacred_and_human_sequences_are_typed_and_canonically_described() {
    assert_eq!(
        SacredMotion::ALL,
        [
            SacredMotion::Return,
            SacredMotion::Incarnate,
            SacredMotion::Commune,
            SacredMotion::Confirm,
        ]
    );
    assert_eq!(
        SacredMotion::ALL.map(SacredMotion::human_motion),
        [
            UniversalHumanMotion::Heal,
            UniversalHumanMotion::Belong,
            UniversalHumanMotion::Participate,
            UniversalHumanMotion::Serve,
        ]
    );
    assert_eq!(
        CANONICAL_CEREMONIAL_SEQUENCE,
        "Return → Incarnate → Commune → Confirm"
    );
    assert_eq!(
        UNIVERSAL_HUMAN_SEQUENCE,
        "Heal → Belong → Participate → Serve"
    );
    assert!(CANON.contains("Heal → Belong → Participate → Serve"));
}

#[test]
fn all_dimension_intensities_match_the_frozen_matrix() {
    let archive = decoded();
    let runtime = &cycle(&archive).seasonal_runtime;
    for (kind, celebration, ritual, competition) in [
        (
            GreatFunctionKind::WayBack,
            FunctionIntensity::VeryHigh,
            FunctionIntensity::VeryHigh,
            FunctionIntensity::Light,
        ),
        (
            GreatFunctionKind::Initiation,
            FunctionIntensity::High,
            FunctionIntensity::VeryHigh,
            FunctionIntensity::Moderate,
        ),
        (
            GreatFunctionKind::Gathering,
            FunctionIntensity::VeryHigh,
            FunctionIntensity::High,
            FunctionIntensity::VeryHigh,
        ),
        (
            GreatFunctionKind::FestivalOfMirrors,
            FunctionIntensity::High,
            FunctionIntensity::VeryHigh,
            FunctionIntensity::Moderate,
        ),
    ] {
        let dimensions = runtime
            .function_at_anchor(kind.anchor())
            .unwrap()
            .dimensions;
        assert_eq!(dimensions.celebration, celebration);
        assert_eq!(dimensions.ritual, ritual);
        assert_eq!(dimensions.competition, competition);
    }
}

#[test]
fn service_tournament_is_nested_in_but_does_not_replace_the_gathering() {
    let archive = decoded();
    let state = cycle(&archive);
    let gathering = state
        .seasonal_runtime
        .function_at_anchor(SeasonalAnchor::SummerSolstice)
        .unwrap();
    assert_eq!(state.nesting.gathering_function_id, gathering.function_id);
    assert!(
        gathering
            .activities
            .contains(&FunctionActivity::ServiceTournament)
    );
    for activity in [
        FunctionActivity::CommunalMeal,
        FunctionActivity::Music,
        FunctionActivity::Market,
        FunctionActivity::Performance,
        FunctionActivity::CulturalExchange,
        FunctionActivity::FamilyGathering,
        FunctionActivity::SharedCommunion,
        FunctionActivity::Diplomacy,
        FunctionActivity::HouseExhibition,
        FunctionActivity::PublicSynthesisPresentation,
        FunctionActivity::AthleticCivicEvent,
    ] {
        assert!(gathering.activities.contains(&activity));
    }
    assert!(gathering.activities.len() > 10);
}

#[test]
fn nested_war_of_a_thousand_hues_remains_nonlethal() {
    let archive = decoded();
    let state = cycle(&archive);
    assert_eq!(
        state.nesting.war_id.as_str(),
        "war.central-junction.thousand-hues"
    );
    assert!(
        state
            .tournament_years
            .values()
            .all(|year| year.tournament_runtime.war().nonlethal)
    );
}

#[test]
fn eternal_christmas_is_incarnational_and_not_a_fixed_winter_date() {
    let archive = decoded();
    let initiation = cycle(&archive)
        .seasonal_runtime
        .function_at_anchor(SeasonalAnchor::SpringEquinox)
        .unwrap();
    assert_eq!(
        initiation.incarnational_principle,
        Some(IncarnationalPrinciple::EternalChristmas)
    );
    assert!(
        !initiation
            .incarnational_principle
            .unwrap()
            .is_fixed_winter_date()
    );
    assert_eq!(initiation.anchor, SeasonalAnchor::SpringEquinox);
    assert!(CANON.contains("eternal, incarnational sense"));
}

#[test]
fn seasonal_venues_consume_the_world_point_field_binding() {
    let archive = decoded();
    let state = cycle(&archive);
    let runtime = &state.seasonal_runtime;
    let world_subjects = state
        .world_point_state
        .relationships
        .values()
        .map(|relationship| relationship.subject_id.as_str())
        .collect::<BTreeSet<_>>();

    let initiation = runtime.resolve_name("The Initiation").unwrap();
    assert!(
        initiation
            .venue_ids
            .iter()
            .any(|venue| venue.as_str() == "venue.aura-field")
    );
    let gathering = runtime.resolve_name("The Gathering").unwrap();
    assert!(
        gathering
            .venue_ids
            .iter()
            .any(|venue| venue.as_str() == "venue.aura-beach")
    );
    assert!(
        gathering
            .event_ids
            .iter()
            .any(|event| event.as_str() == "event.canonical-year.gathering.ceremony")
    );
    assert!(gathering.activities.contains(&FunctionActivity::Ceremony));
    let festival = runtime.resolve_name("The Festival of Mirrors").unwrap();
    assert!(
        festival
            .venue_ids
            .iter()
            .any(|venue| venue.as_str() == "venue.aura-basin")
    );
    assert!(runtime.functions().values().all(|function| {
        function
            .venue_ids
            .iter()
            .all(|venue| world_subjects.contains(venue.as_str()))
    }));
}

#[test]
fn removing_a_seasonal_venue_world_binding_is_rejected() {
    let mut payload = canonical_seasonal_archive_fixture();
    payload.annual_cycles[0]
        .world_point_archive
        .relationships
        .retain(|relationship| relationship.subject_id.as_str() != "venue.aura-field");
    assert!(matches!(
        encode_seasonal_archive(&payload),
        Err(SeasonalArchiveError::InvalidPassageCycle(_))
    ));
}

#[test]
fn festival_recognizes_service_mark_edge_and_glass_without_sovereignty() {
    let archive = decoded();
    let state = cycle(&archive);
    let festival = state
        .seasonal_runtime
        .function_at_anchor(SeasonalAnchor::AutumnEquinox)
        .unwrap();
    assert_eq!(festival.kind, GreatFunctionKind::FestivalOfMirrors);
    assert!(!festival.transfers_permanent_sovereignty);
    assert!(!festival.presiding_house_owns_central_junction);
    assert_eq!(state.recognitions.len(), 3);
    assert!(
        state
            .recognitions
            .values()
            .all(|recognition| !recognition.grants_permanent_sovereignty)
    );
}

#[test]
fn every_apex_is_inside_its_function_and_functions_are_chronological() {
    let archive = decoded();
    let runtime = &cycle(&archive).seasonal_runtime;
    let ordered = SeasonalAnchor::ALL
        .into_iter()
        .map(|anchor| runtime.function_at_anchor(anchor).unwrap())
        .collect::<Vec<_>>();
    assert!(
        ordered
            .iter()
            .all(|function| function.opens_at < function.apex_at
                && function.apex_at < function.closes_at)
    );
    assert!(
        ordered
            .windows(2)
            .all(|pair| pair[0].apex_at < pair[1].apex_at)
    );
}

#[test]
fn archive_replay_preserves_aliases_canonical_identity_and_checksum() {
    let payload = canonical_seasonal_archive_fixture();
    let bytes = encode_seasonal_archive(&payload).unwrap();
    let first = decode_seasonal_archive(&bytes).unwrap();
    let second = decode_seasonal_archive(&bytes).unwrap();
    assert_eq!(first.archive_version, SEASONAL_ARCHIVE_VERSION);
    assert_eq!(first.checksum.len(), 16);
    assert_eq!(first.annual_cycles, second.annual_cycles);
    let runtime = &cycle(&first).seasonal_runtime;
    assert_eq!(
        runtime.resolve_name("Derrick").unwrap().function_id,
        runtime.resolve_name("The Gathering").unwrap().function_id
    );
}

#[test]
fn obsolete_working_names_migrate_without_duplicate_functions() {
    let payload = canonical_seasonal_archive_fixture();
    let legacy = encode_legacy_seasonal_archive_v0(&payload).unwrap();
    let decoded_legacy = decode_seasonal_archive(&legacy).unwrap();
    let runtime = &cycle(&decoded_legacy).seasonal_runtime;
    assert_eq!(runtime.functions().len(), 4);
    assert!(runtime.resolve_name("The Way Back").is_some());
    assert!(runtime.resolve_name("The Initiation").is_some());
    assert!(runtime.resolve_name("The Gathering").is_some());
    assert!(runtime.resolve_name("The Festival of Mirrors").is_some());

    let migrated = migrate_seasonal_archive(&legacy).unwrap();
    let decoded_migrated = decode_seasonal_archive(&migrated).unwrap();
    assert_eq!(decoded_migrated.archive_version, SEASONAL_ARCHIVE_VERSION);
    assert_eq!(
        cycle(&decoded_migrated).seasonal_runtime.functions().len(),
        4
    );
}

#[test]
fn opposite_insertion_order_does_not_change_archive_or_replay() {
    let payload = canonical_seasonal_archive_fixture();
    let canonical = encode_seasonal_archive(&payload).unwrap();
    let mut reversed = payload;
    reversed.annual_cycles.reverse();
    let cycle = &mut reversed.annual_cycles[0];
    cycle.calendar.anchor_observations.reverse();
    cycle.functions.reverse();
    for function in &mut cycle.functions {
        function.phases.reverse();
    }
    cycle.recognitions.reverse();
    cycle.world_point_archive.relationships.reverse();
    cycle.way_back_passages.reverse();
    cycle.current_sea_events.reverse();
    cycle.glaushouse_clearances.reverse();
    cycle.boardwalk_passages.reverse();
    cycle.flynt_resynce_events.reverse();
    cycle.flynt_recog_events.reverse();
    cycle.tournament_archive.years.reverse();
    let tournament = &mut cycle.tournament_archive.years[0];
    tournament.representatives.reverse();
    tournament.events.reverse();
    tournament.scenario_phases.reverse();
    tournament.alliances.reverse();
    tournament.real_emergencies.reverse();
    tournament.scoring_events.reverse();
    tournament.constitutional_violations.reverse();
    tournament.artifact_refinements.reverse();
    tournament.prize_awards.reverse();
    tournament.synthesis_semantic_events.reverse();
    assert_eq!(encode_seasonal_archive(&reversed).unwrap(), canonical);
}

#[test]
fn archive_tampering_and_a_fifth_function_are_rejected() {
    let payload = canonical_seasonal_archive_fixture();
    let bytes = encode_seasonal_archive(&payload).unwrap();
    let text = String::from_utf8(bytes).unwrap();
    let tampered = text.replace("\"The Gathering\"", "\"The GatherinG\"");
    assert!(matches!(
        decode_seasonal_archive(tampered.as_bytes()),
        Err(SeasonalArchiveError::ChecksumMismatch)
    ));

    let mut fifth = payload;
    let duplicate = fifth.annual_cycles[0].functions[0].clone();
    fifth.annual_cycles[0].functions.push(duplicate);
    assert!(matches!(
        encode_seasonal_archive(&fifth),
        Err(SeasonalArchiveError::Seasonal(_))
    ));
}

#[test]
fn all_four_great_functions_are_nonsovereign_and_anchor_complete() {
    let archive = decoded();
    let runtime = &cycle(&archive).seasonal_runtime;
    assert!(
        runtime
            .functions()
            .values()
            .all(|function| !function.transfers_permanent_sovereignty
                && !function.presiding_house_owns_central_junction)
    );
    assert_eq!(
        runtime
            .functions()
            .values()
            .map(|function| function.anchor)
            .collect::<BTreeSet<_>>(),
        SeasonalAnchor::ALL.into_iter().collect()
    );
}

#[test]
fn universal_recursion_kernel_remains_free_of_seasonal_house_lore() {
    const KERNEL: &str = include_str!("../hollow-grove-kernel/src/lib.rs");
    for forbidden in [
        "The Way Back",
        "The Initiation",
        "The Gathering",
        "Derrick",
        "The Festival of Mirrors",
        "WinterSolstice",
        "GreatFunctionKind",
    ] {
        assert!(!KERNEL.contains(forbidden), "kernel contains {forbidden}");
    }
}
