use std::collections::BTreeSet;
use std::process::ExitCode;

use hollow_grove::constitutional::SeasonalAnchor;
use hollow_grove::hollow_grove_contract::House;
use hollow_grove::world::central_junction_seasonal_functions::{
    FunctionIntensity, GreatFunctionKind, SacredMotion,
};
use hollow_grove::world::current_sea_passage::{
    BOARDWALK_ROUTE_ID, CURRENT_SEA_REGION_ID, CurrentSeaForce, CurrentSeaForceDirection,
    CurrentSeaState, RIPTIDE_FORCE_ID, UNDERTOW_FORCE_ID,
};
use hollow_grove::world::seasonal_functions_archive::{
    SEASONAL_ARCHIVE_VERSION, decode_seasonal_archive, encode_seasonal_archive,
};
use hollow_grove::world::seasonal_functions_fixture::canonical_seasonal_archive_fixture;
use hollow_grove::world::way_back::{
    AURA_WAY_ROUTE_ID, STAIRWAY_TO_HEAVEN_SEGMENT_ID, THE_WAY_BACK_RULE_ID, WayBackDirection,
    WayBackExpression, WayBackSupportRole,
};
use hollow_grove::world::world_point::{
    CENTRAL_JUNCTION_REGION_ID, DARK_AURA_REGION_ID, LIGHT_AURA_REGION_ID, WORLD_NEGATIVE_POLE_ID,
    WORLD_POINT_ID, WORLD_POSITIVE_POLE_ID,
};

fn main() -> ExitCode {
    match audit() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("FAIL central-junction-seasonal-audit: {error}");
            ExitCode::FAILURE
        }
    }
}

fn require(condition: bool, label: &str) -> Result<(), String> {
    if condition {
        println!("PASS {label}");
        Ok(())
    } else {
        Err(label.into())
    }
}

fn audit() -> Result<(), String> {
    let fixture = canonical_seasonal_archive_fixture();
    let bytes = encode_seasonal_archive(&fixture).map_err(|error| error.to_string())?;
    let decoded = decode_seasonal_archive(&bytes).map_err(|error| error.to_string())?;
    let replayed = decode_seasonal_archive(&bytes).map_err(|error| error.to_string())?;
    let cycle = decoded
        .annual_cycles
        .values()
        .next()
        .ok_or_else(|| "canonical annual cycle missing".to_owned())?;
    let runtime = &cycle.seasonal_runtime;
    let winter = runtime
        .year()
        .observation(SeasonalAnchor::WinterSolstice)
        .ok_or_else(|| "Winter Solstice observation missing".to_owned())?;

    require(
        decoded.archive_version == SEASONAL_ARCHIVE_VERSION
            && runtime.year().opens_at == winter.astronomical_instant,
        &format!(
            "canonical year begins at Winter Solstice={} archive-version={}",
            runtime.year().opens_at,
            decoded.archive_version
        ),
    )?;
    require(
        runtime.functions().len() == 4,
        "exactly four Great Functions",
    )?;
    require(
        SeasonalAnchor::ALL.into_iter().all(|anchor| {
            runtime
                .functions()
                .values()
                .filter(|function| function.anchor == anchor)
                .count()
                == 1
        }),
        "each astronomical anchor occurs exactly once",
    )?;

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
        let function = runtime
            .function_at_anchor(kind.anchor())
            .ok_or_else(|| format!("{} missing", kind.canonical_name()))?;
        require(
            function.presiding_house == house && function.canonical_name == name,
            &format!("{name} presided by {}", house.as_str()),
        )?;
    }

    let gathering = runtime
        .resolve_name("The Gathering")
        .ok_or_else(|| "The Gathering missing".to_owned())?;
    let derrick = runtime
        .resolve_name("Derrick")
        .ok_or_else(|| "Derrick alias missing".to_owned())?;
    require(
        gathering.function_id == derrick.function_id && runtime.functions().len() == 4,
        "Derrick aliases The Gathering and is not a fifth Function",
    )?;

    let sacred_sequence = SeasonalAnchor::ALL
        .into_iter()
        .map(|anchor| runtime.function_at_anchor(anchor).unwrap().sacred_motion)
        .collect::<Vec<_>>();
    require(
        sacred_sequence
            == vec![
                SacredMotion::Return,
                SacredMotion::Incarnate,
                SacredMotion::Commune,
                SacredMotion::Confirm,
            ],
        "sacred motion sequence Return → Incarnate → Commune → Confirm",
    )?;

    let dimension_checks = [
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
    ];
    require(
        dimension_checks
            .into_iter()
            .all(|(kind, celebration, ritual, competition)| {
                let dimensions = runtime
                    .function_at_anchor(kind.anchor())
                    .unwrap()
                    .dimensions;
                dimensions.celebration == celebration
                    && dimensions.ritual == ritual
                    && dimensions.competition == competition
            }),
        "Celebration/Ritual/Competition intensity matrix",
    )?;
    require(
        cycle.nesting.gathering_function_id == gathering.function_id
            && cycle.nesting.service_tournament_id.as_str()
                == "function.central-junction.service-tournament",
        "Service Tournament nested under The Gathering",
    )?;
    require(
        cycle.nesting.war_id.as_str() == "war.central-junction.thousand-hues"
            && cycle
                .tournament_years
                .values()
                .all(|year| year.tournament_runtime.war().nonlethal),
        "War of a Thousand Hues nested and nonlethal",
    )?;
    let festival = runtime
        .function_at_anchor(SeasonalAnchor::AutumnEquinox)
        .unwrap();
    require(
        festival.kind == GreatFunctionKind::FestivalOfMirrors
            && !festival.transfers_permanent_sovereignty
            && cycle
                .recognitions
                .values()
                .all(|recognition| !recognition.grants_permanent_sovereignty),
        "Festival of Mirrors recognizes without sovereignty",
    )?;

    let way_back = &cycle.way_back_runtime;
    require(
        way_back.route().route_id.as_str() == AURA_WAY_ROUTE_ID
            && way_back.route().rule_id.as_str() == THE_WAY_BACK_RULE_ID
            && way_back.route().stairway_segment_id.as_str() == STAIRWAY_TO_HEAVEN_SEGMENT_ID
            && way_back.route().route_owner.is_none()
            && way_back.route().valid_outside_winter_function,
        "Aura Way is the permanent unowned route; The Way Back is directional; route and Function identities are distinct",
    )?;
    let descent = way_back
        .passages()
        .values()
        .find(|passage| passage.direction == WayBackDirection::DescendingFromStonebend);
    require(
        descent.is_some_and(|passage| {
            passage.origin_house == House::Stonebend
                && passage.destination_house == House::Flynt
                && passage.expression == WayBackExpression::AuraWay
        }),
        "Way Back descent begins at Stonebend and terminates at Flynt",
    )?;
    let ascent = way_back
        .passages()
        .values()
        .find(|passage| passage.direction == WayBackDirection::AscendingFromFlynt);
    require(
        ascent.is_some_and(|passage| {
            passage.origin_house == House::Flynt
                && passage.destination_house == House::Stonebend
                && passage.expression == WayBackExpression::StairwayToHeaven
        }) && way_back.passages().values().all(|passage| {
            passage.expression != WayBackExpression::StairwayToHeaven
                || passage.direction == WayBackDirection::AscendingFromFlynt
        }),
        "Way Back ascent begins at Flynt and Stairway to Heaven is ascent-only",
    )?;
    require(
        way_back.passages().values().all(|passage| {
            !passage.evidence_ids.is_empty() && !passage.provenance_id.as_str().is_empty()
        }),
        "Way Back passage provenance is valid",
    )?;
    require(
        way_back.passages().values().any(|passage| {
            passage.support.iter().any(|support| {
                support.house == House::Glaushouse
                    && support.role == WayBackSupportRole::GlaushouseCareAndClearance
                    && !support.claims_route_ownership
            })
        }),
        "Glaüshouse presides and tends but does not own Aura Way",
    )?;
    require(
        way_back.passages().values().any(|passage| {
            passage.support.iter().any(|support| {
                support.house == House::Sandmanor
                    && support.role == WayBackSupportRole::SandmanorArrangement
                    && !support.claims_route_ownership
            })
        }),
        "Sandmanor arranges passage but does not own Aura Way",
    )?;

    let current = &cycle.current_sea_runtime;
    require(
        current.events().values().any(|event| {
            event.region_id.as_str() == CURRENT_SEA_REGION_ID
                && event.state == CurrentSeaState::Setting
                && event.origin_house == House::Glaushouse
        }),
        "Current Sea sets in Glaüshouse",
    )?;
    require(
        current.events().values().any(|event| {
            event.state == CurrentSeaState::Rising
                && event.force == Some(CurrentSeaForce::Riptide)
                && event
                    .force_id
                    .as_ref()
                    .is_some_and(|id| id.as_str() == RIPTIDE_FORCE_ID)
                && event.force_direction
                    == Some(CurrentSeaForceDirection::FromGlaushouseTowardFlynt)
                && event.origin_house == House::Glaushouse
                && event.destination_house == Some(House::Flynt)
        }),
        "Riptide is the rising Current force from Glaüshouse toward Flynt",
    )?;
    require(
        current.events().values().any(|event| {
            event.force == Some(CurrentSeaForce::Undertow)
                && event
                    .force_id
                    .as_ref()
                    .is_some_and(|id| id.as_str() == UNDERTOW_FORCE_ID)
                && !event.is_successful_rise()
        }),
        "Undertow is the inverse hazard, not successful emergence",
    )?;
    require(
        current.passages().values().all(|passage| {
            passage.route_id.as_str() == BOARDWALK_ROUTE_ID
                && matches!(
                    (passage.origin_house, passage.destination_house),
                    (House::Glaushouse, House::Flynt) | (House::Flynt, House::Glaushouse)
                )
                && !passage.grants_automatic_recog
        }),
        "the traveler moves by the distinct lawful Boardwalk route without automatic Recog",
    )?;
    require(
        current
            .clearances()
            .values()
            .all(|clearance| !clearance.grants_flynt_recog)
            && !current.resynce_events().is_empty()
            && current
                .recog_events()
                .values()
                .all(|recog| !recog.manifestation_condition_ids.is_empty()),
        "Glaüshouse clearance, Flynt Resynce, and lawful Recog remain separate",
    )?;

    let world = &cycle.world_point_state.binding;
    require(
        world.point.point_id.as_str() == WORLD_POINT_ID
            && world.point.scale.as_str() == "scale.world"
            && world.point.positive_pole_id.as_str() == WORLD_POSITIVE_POLE_ID
            && world.point.negative_pole_id.as_str() == WORLD_NEGATIVE_POLE_ID,
        "world Point identity, scale, axis, and stable poles",
    )?;
    require(
        world.light_aura_region_id.as_str() == LIGHT_AURA_REGION_ID
            && world.central_junction_region_id.as_str() == CENTRAL_JUNCTION_REGION_ID
            && world.dark_aura_region_id.as_str() == DARK_AURA_REGION_ID,
        "Light Aura is positive, Central Junction is center, and Dark Aura is negative",
    )?;
    require(
        world.lawfulness_requires_separate_determination
            && !world.presentation_may_change_constitutional_polarity,
        "polarity is not morality and rendering cannot change constitutional orientation",
    )?;
    let world_subjects = cycle
        .world_point_state
        .relationships
        .values()
        .map(|relationship| relationship.subject_id.as_str())
        .collect::<BTreeSet<_>>();
    require(
        runtime.functions().values().all(|function| {
            function
                .venue_ids
                .iter()
                .all(|venue| world_subjects.contains(venue.as_str()))
        }) && runtime
            .resolve_name("The Initiation")
            .is_some_and(|function| {
                function
                    .venue_ids
                    .iter()
                    .any(|venue| venue.as_str() == "venue.aura-field")
            })
            && gathering
                .venue_ids
                .iter()
                .any(|venue| venue.as_str() == "venue.aura-beach")
            && festival
                .venue_ids
                .iter()
                .any(|venue| venue.as_str() == "venue.aura-basin"),
        "seasonal venues consume the world field: Aura Field, Aura Beach, and Aura Basin",
    )?;
    let ordered_apices = SeasonalAnchor::ALL
        .into_iter()
        .map(|anchor| runtime.function_at_anchor(anchor).unwrap().apex_at.clone())
        .collect::<Vec<_>>();
    require(
        ordered_apices.windows(2).all(|pair| pair[0] < pair[1]),
        "chronological order is valid",
    )?;
    require(
        decoded.annual_cycles == replayed.annual_cycles,
        "deterministic replay succeeds",
    )?;
    require(
        encode_seasonal_archive(&decoded.payload).map_err(|error| error.to_string())? == bytes,
        "archive checksum and canonical encoding are valid",
    )?;
    require(
        runtime
            .functions()
            .values()
            .map(|function| function.anchor)
            .collect::<BTreeSet<_>>()
            == SeasonalAnchor::ALL.into_iter().collect(),
        "annual anchor registry is complete",
    )?;
    println!("PASS archive checksum={}", decoded.checksum);
    Ok(())
}
