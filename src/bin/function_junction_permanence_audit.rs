use std::collections::BTreeSet;

use hollow_grove::constitutional::{
    GROVE_PHASE_SEQUENCE, GroveCycleResolution, GrovePhase, SeasonalAnchor,
};
use hollow_grove::hollow_grove_contract::House;
use hollow_grove::world::central_junction_seasonal_functions::{GreatFunctionKind, SacredMotion};
use hollow_grove::world::current_sea_passage::{
    BOARDWALK_ROUTE_ID, CURRENT_SEA_REGION_ID, CurrentSeaForce,
};
use hollow_grove::world::function_junction::{SynchronizationPhase, seasonal_handoff};
use hollow_grove::world::function_junction_archive::{
    FUNCTION_JUNCTION_ARCHIVE_VERSION, decode_function_junction_archive,
    encode_function_junction_archive,
};
use hollow_grove::world::function_junction_fixture::canonical_function_junction_archive_fixture;
use hollow_grove::world::permanence::{
    PERMANENCE_CYCLE, PERMANENCE_MAXIM, PermanenceLaw, PermanentChangeKind,
};
use hollow_grove::world::seasonal_functions_archive::{
    decode_seasonal_archive, encode_seasonal_archive,
};
use hollow_grove::world::service_tournament::canonical_war_of_a_thousand_hues;
use hollow_grove::world::way_back::{WayBackDirection, WayBackExpression};
use hollow_grove::world::world_point::{
    CENTRAL_JUNCTION_REGION_ID, DARK_AURA_REGION_ID, LIGHT_AURA_REGION_ID,
};

fn report(label: &str, condition: bool, failures: &mut usize) {
    if condition {
        println!("PASS {label}");
    } else {
        println!("FAIL {label}");
        *failures += 1;
    }
}

fn main() {
    let payload = canonical_function_junction_archive_fixture();
    let bytes = match encode_function_junction_archive(&payload) {
        Ok(bytes) => bytes,
        Err(error) => {
            eprintln!("FAIL canonical archive encoding: {error}");
            std::process::exit(1);
        }
    };
    let decoded = match decode_function_junction_archive(&bytes) {
        Ok(decoded) => decoded,
        Err(error) => {
            eprintln!("FAIL canonical archive decoding: {error}");
            std::process::exit(1);
        }
    };
    let state = decoded
        .annual_states
        .values()
        .next()
        .expect("canonical fixture has one annual state");
    let junction_runtime = &state.function_junction_runtime;
    let annual_cycle = &state.annual_cycle;
    let permanence = &state.permanence_runtime;
    let mut failures = 0;

    report(
        &format!(
            "archive version={} canonical-year={}",
            decoded.archive_version, state.canonical_year_id
        ),
        decoded.archive_version == FUNCTION_JUNCTION_ARCHIVE_VERSION,
        &mut failures,
    );
    report(
        "world Point binds Light Aura / Central Junction / Dark Aura",
        annual_cycle
            .world_point_state
            .binding
            .light_aura_region_id
            .as_str()
            == LIGHT_AURA_REGION_ID
            && annual_cycle
                .world_point_state
                .binding
                .central_junction_region_id
                .as_str()
                == CENTRAL_JUNCTION_REGION_ID
            && annual_cycle
                .world_point_state
                .binding
                .dark_aura_region_id
                .as_str()
                == DARK_AURA_REGION_ID,
        &mut failures,
    );
    report(
        "exactly four Great Functions, Function Junctions, and House Seasons",
        annual_cycle.seasonal_runtime.functions().len() == 4
            && junction_runtime.junctions().len() == 4
            && junction_runtime.seasons().len() == 4,
        &mut failures,
    );
    report(
        "Function Junction is a transition, not Central Junction or a fifth Function",
        junction_runtime.junctions().values().all(|junction| {
            !junction.is_geographic_location
                && !junction.is_great_function
                && junction.physical_place_id == CENTRAL_JUNCTION_REGION_ID
        }),
        &mut failures,
    );

    let expected = [
        (
            SeasonalAnchor::WinterSolstice,
            House::Flynt,
            House::Glaushouse,
            SynchronizationPhase::PhysicalToDigitalReturn,
        ),
        (
            SeasonalAnchor::SpringEquinox,
            House::Glaushouse,
            House::Stonebend,
            SynchronizationPhase::DigitalToPhysicalIncarnation,
        ),
        (
            SeasonalAnchor::SummerSolstice,
            House::Stonebend,
            House::Sandmanor,
            SynchronizationPhase::BidirectionalParticipation,
        ),
        (
            SeasonalAnchor::AutumnEquinox,
            House::Sandmanor,
            House::Flynt,
            SynchronizationPhase::ComparisonAndConfirmation,
        ),
    ];
    report(
        "seasonal handoffs and physical–digital synchronization are exact",
        expected.iter().all(|(anchor, outgoing, incoming, phase)| {
            let junction = junction_runtime
                .junction_at_anchor(*anchor)
                .expect("canonical junction");
            seasonal_handoff(*anchor).0 == *outgoing
                && junction.outgoing_house == *outgoing
                && junction.incoming_house == *incoming
                && junction.synchronization_phase == *phase
                && junction.physical_checkpoint.status
                    == hollow_grove::world::function_junction::CheckpointStatus::Completed
                && junction.digital_checkpoint.status
                    == hollow_grove::world::function_junction::CheckpointStatus::Completed
        }),
        &mut failures,
    );
    report(
        &format!("exact four-phase loop: {GROVE_PHASE_SEQUENCE}"),
        GrovePhase::ALL.len() == 4
            && GrovePhase::ALL
                == [
                    GrovePhase::TheWayBack,
                    GrovePhase::TheInitiation,
                    GrovePhase::TheGathering,
                    GrovePhase::TheFestival,
                ]
            && GrovePhase::TheFestival.next() == GrovePhase::TheWayBack
            && expected.iter().all(|(anchor, _, _, _)| {
                junction_runtime
                    .junction_at_anchor(*anchor)
                    .is_some_and(|junction| {
                        junction.grove_phase
                            == hollow_grove::world::function_junction::grove_phase_for_anchor(
                                *anchor,
                            )
                    })
            }),
        &mut failures,
    );
    let cycles = &state.grove_cycle_runtime;
    report(
        "Festival records acceptance or rejection and feeds the next Way Back",
        cycles.cycles().values().all(|cycle| {
            cycle.current_phase == GrovePhase::TheFestival
                && cycle.next_way_back_state_id == cycle.confirmed_state_id
                && !cycle.rendering_may_advance_phase
                && match cycle.resolution {
                    GroveCycleResolution::Rejected => {
                        cycle.confirmed_state_id.as_ref() == Some(&cycle.prior_state_id)
                    }
                    GroveCycleResolution::Accepted => {
                        cycle.confirmed_state_id != Some(cycle.prior_state_id.clone())
                    }
                    GroveCycleResolution::Pending => false,
                }
        }),
        &mut failures,
    );
    report(
        "The Gathering/Derrick opens the Season of Sandmanor",
        annual_cycle
            .seasonal_runtime
            .resolve_name("Derrick")
            .is_some_and(|function| {
                function.kind == GreatFunctionKind::Gathering
                    && junction_runtime
                        .junction_at_anchor(SeasonalAnchor::SummerSolstice)
                        .is_some_and(|junction| junction.incoming_house == House::Sandmanor)
            }),
        &mut failures,
    );
    report(
        "ceremonial motion glosses remain subordinate: Return / Incarnate / Commune / Confirm",
        GreatFunctionKind::ALL
            .into_iter()
            .map(GreatFunctionKind::sacred_motion)
            .collect::<Vec<_>>()
            == vec![
                SacredMotion::Return,
                SacredMotion::Incarnate,
                SacredMotion::Commune,
                SacredMotion::Confirm,
            ],
        &mut failures,
    );
    report(
        "The Practical Joke cues transitions without replacing anchors",
        junction_runtime.practical_jokes().len() == 4
            && junction_runtime
                .practical_jokes()
                .values()
                .all(|joke| !joke.replaces_astronomical_anchor && !joke.replaces_function_junction),
        &mut failures,
    );

    let way_back = &annual_cycle.way_back_runtime;
    report(
        "The Way Back descends Stonebend→Flynt and ascends Flynt→Stonebend",
        way_back.passages().values().any(|passage| {
            passage.direction == WayBackDirection::DescendingFromStonebend
                && passage.origin_house == House::Stonebend
                && passage.destination_house == House::Flynt
        }) && way_back.passages().values().any(|passage| {
            passage.direction == WayBackDirection::AscendingFromFlynt
                && passage.origin_house == House::Flynt
                && passage.destination_house == House::Stonebend
                && passage.expression == WayBackExpression::StairwayToHeaven
        }),
        &mut failures,
    );
    let current = &annual_cycle.current_sea_runtime;
    report(
        "Current Sea, Riptide, Undertow, and Boardwalk remain distinct",
        current.events().values().any(|event| {
            event.region_id.as_str() == CURRENT_SEA_REGION_ID
                && event.force == Some(CurrentSeaForce::Riptide)
        }) && current
            .events()
            .values()
            .any(|event| event.force == Some(CurrentSeaForce::Undertow))
            && current
                .passages()
                .values()
                .all(|passage| passage.route_id.as_str() == BOARDWALK_ROUTE_ID),
        &mut failures,
    );
    report(
        "War of a Thousand Hues remains nonlethal",
        canonical_war_of_a_thousand_hues().nonlethal,
        &mut failures,
    );

    report(
        "all four Permanence laws are attested by their fixed Houses",
        permanence
            .attestations()
            .values()
            .map(|attestation| (attestation.law, attestation.authority_house))
            .collect::<BTreeSet<_>>()
            == PermanenceLaw::ALL
                .into_iter()
                .map(|law| (law, law.authority_house()))
                .collect(),
        &mut failures,
    );
    report(
        "only Stonebend issues the final non-immutable Permanence Seal",
        permanence.seals().values().all(|seal| {
            seal.issuing_house == House::Stonebend
                && !seal.immutable
                && seal.supporting_attestation_ids.len() == 4
        }),
        &mut failures,
    );
    report(
        "Permanence keeps amendment, succession, and dissolution history",
        permanence
            .changes()
            .values()
            .map(|change| change.kind)
            .collect::<BTreeSet<_>>()
            == [
                PermanentChangeKind::Amendment,
                PermanentChangeKind::Succession,
                PermanentChangeKind::Dissolution,
            ]
            .into_iter()
            .collect()
            && permanence
                .histories()
                .values()
                .all(|history| history.versions.len() == 4 && history.tombstone_id.is_some()),
        &mut failures,
    );
    report(
        &format!("{PERMANENCE_CYCLE}; {PERMANENCE_MAXIM}"),
        !permanence.petitions().is_empty() && !permanence.tombstones().is_empty(),
        &mut failures,
    );
    report(
        "no Great Function or Function Junction transfers sovereignty",
        annual_cycle
            .seasonal_runtime
            .functions()
            .values()
            .all(|function| !function.transfers_permanent_sovereignty)
            && junction_runtime
                .junctions()
                .values()
                .all(|junction| !junction.transfers_sovereignty),
        &mut failures,
    );

    let seasonal_bytes = encode_seasonal_archive(&payload.annual_records[0].seasonal_archive)
        .expect("canonical nested seasonal archive encodes");
    let seasonal = decode_seasonal_archive(&seasonal_bytes)
        .expect("canonical nested seasonal archive decodes");
    report(
        &format!(
            "outer checksum={} nested-HGSEA-checksum={}",
            decoded.checksum, seasonal.checksum
        ),
        decode_function_junction_archive(
            &encode_function_junction_archive(&decoded.payload)
                .expect("re-encoding decoded payload"),
        )
        .is_ok_and(|replayed| replayed.payload == decoded.payload),
        &mut failures,
    );

    if failures > 0 {
        eprintln!("Function Junction/Permanence audit failed with {failures} invariant(s)");
        std::process::exit(1);
    }
}
