use hollow_grove::constitutional::{
    GroveCycleResolution, GroveCycleRuntime, GrovePhase, LegacyGroveCycleRecord, LegacyGrovePhase,
    SeasonalAnchor, migrate_legacy_grove_cycles,
};
use hollow_grove::hollow_grove_contract::House;
use hollow_grove::world::central_junction_seasonal_functions::{
    FunctionActivity, GreatFunctionKind,
};
use hollow_grove::world::current_sea_passage::{
    BOARDWALK_ROUTE_ID, CURRENT_SEA_REGION_ID, CurrentSeaForce,
};
use hollow_grove::world::function_junction::{
    CheckpointStatus, SynchronizationPhase, WorldLayer, seasonal_handoff,
};
use hollow_grove::world::function_junction_archive::{
    FUNCTION_JUNCTION_ARCHIVE_VERSION, decode_function_junction_archive,
    encode_function_junction_archive, encode_legacy_function_junction_archive_v0,
    migrate_function_junction_archive, replay_function_junction_payload,
};
use hollow_grove::world::function_junction_fixture::{
    canonical_function_junction_archive_fixture, canonical_grove_cycles_fixture,
};
use hollow_grove::world::permanence::{
    AttestationStatus, PermanenceLaw, PermanentChangeKind, PermanentSubjectId,
};
use hollow_grove::world::service_tournament::canonical_war_of_a_thousand_hues;
use hollow_grove::world::way_back::{WayBackDirection, WayBackExpression};
use hollow_grove::world::world_point::{
    CENTRAL_JUNCTION_REGION_ID, ConstitutionalLawfulness, DARK_AURA_REGION_ID, LIGHT_AURA_REGION_ID,
};
use hollow_grove::{PhysicalPosition, RelativePolarity};

fn decoded() -> hollow_grove::world::function_junction_archive::DecodedFunctionJunctionArchive {
    let payload = canonical_function_junction_archive_fixture();
    let bytes = encode_function_junction_archive(&payload).unwrap();
    decode_function_junction_archive(&bytes).unwrap()
}

#[test]
fn grove_phase_has_exactly_four_variants_in_the_fixed_loop() {
    assert_eq!(
        GrovePhase::ALL,
        [
            GrovePhase::TheWayBack,
            GrovePhase::TheInitiation,
            GrovePhase::TheGathering,
            GrovePhase::TheFestival,
        ]
    );
    assert_eq!(GrovePhase::TheWayBack.next(), GrovePhase::TheInitiation);
    assert_eq!(GrovePhase::TheInitiation.next(), GrovePhase::TheGathering);
    assert_eq!(GrovePhase::TheGathering.next(), GrovePhase::TheFestival);
    assert_eq!(GrovePhase::TheFestival.next(), GrovePhase::TheWayBack);
}

#[test]
fn invalid_direct_phase_transitions_are_rejected() {
    let mut cycle = canonical_grove_cycles_fixture()[0].clone();
    cycle.current_phase = GrovePhase::TheWayBack;
    cycle.phase_history = vec![GrovePhase::TheWayBack];
    cycle.attempted_state_id = None;
    cycle.confirmed_state_id = None;
    cycle.next_way_back_state_id = None;
    cycle.resolution = GroveCycleResolution::Pending;
    cycle.completed_at = None;
    cycle.validate().unwrap();
    assert!(cycle.transition_to(GrovePhase::TheGathering).is_err());
    cycle.transition_to(GrovePhase::TheInitiation).unwrap();
    assert!(cycle.transition_to(GrovePhase::TheFestival).is_err());
    cycle.transition_to(GrovePhase::TheGathering).unwrap();
    assert!(cycle.transition_to(GrovePhase::TheWayBack).is_err());
}

#[test]
fn rejected_festival_records_attempt_without_false_state_mutation() {
    let cycles = canonical_grove_cycles_fixture();
    let rejected = cycles
        .iter()
        .find(|cycle| cycle.resolution == GroveCycleResolution::Rejected)
        .unwrap();
    assert_eq!(rejected.current_phase, GrovePhase::TheFestival);
    assert_ne!(
        rejected.attempted_state_id.as_ref(),
        Some(&rejected.prior_state_id)
    );
    assert_eq!(
        rejected.confirmed_state_id.as_ref(),
        Some(&rejected.prior_state_id)
    );
    assert_eq!(
        rejected.next_way_back_state_id.as_ref(),
        Some(&rejected.prior_state_id)
    );
}

#[test]
fn successful_festival_result_becomes_next_way_back_input() {
    let cycles = canonical_grove_cycles_fixture();
    let accepted = cycles
        .iter()
        .find(|cycle| cycle.resolution == GroveCycleResolution::Accepted)
        .unwrap();
    assert_eq!(accepted.current_phase, GrovePhase::TheFestival);
    assert_eq!(accepted.confirmed_state_id, accepted.attempted_state_id);
    assert_eq!(accepted.next_way_back_state_id, accepted.confirmed_state_id);
    assert_ne!(
        accepted.confirmed_state_id.as_ref(),
        Some(&accepted.prior_state_id)
    );
}

#[test]
fn continuous_cycles_reenter_with_the_prior_confirmed_result() {
    let runtime = GroveCycleRuntime::replay(&canonical_grove_cycles_fixture()).unwrap();
    let cycles = runtime.cycles().values().collect::<Vec<_>>();
    assert_eq!(cycles.len(), 2);
    let rejected = cycles
        .iter()
        .find(|cycle| cycle.resolution == GroveCycleResolution::Rejected)
        .unwrap();
    let accepted = cycles
        .iter()
        .find(|cycle| cycle.resolution == GroveCycleResolution::Accepted)
        .unwrap();
    assert_eq!(
        rejected.next_way_back_state_id.as_ref(),
        Some(&accepted.prior_state_id)
    );
}

#[test]
fn old_abstract_phase_names_migrate_without_duplicate_cycles() {
    let cycle = canonical_grove_cycles_fixture()[0].clone();
    let migrated = migrate_legacy_grove_cycles(&[
        LegacyGroveCycleRecord {
            cycle: cycle.clone(),
            legacy_phase: LegacyGrovePhase::Confirm,
        },
        LegacyGroveCycleRecord {
            cycle,
            legacy_phase: LegacyGrovePhase::Witness,
        },
    ]);
    assert_eq!(migrated.len(), 1);
    assert_eq!(migrated[0].current_phase, GrovePhase::TheFestival);
    assert_eq!(LegacyGrovePhase::Return.migrate(), GrovePhase::TheWayBack);
    assert_eq!(
        LegacyGrovePhase::Incarnate.migrate(),
        GrovePhase::TheInitiation
    );
    assert_eq!(
        LegacyGrovePhase::Commune.migrate(),
        GrovePhase::TheGathering
    );
    assert_eq!(
        LegacyGrovePhase::Transform.migrate(),
        GrovePhase::TheFestival
    );
}

#[test]
fn seasonal_functions_map_to_short_kernel_facing_phases() {
    let decoded = decoded();
    let runtime = &decoded
        .annual_states
        .values()
        .next()
        .unwrap()
        .function_junction_runtime;
    assert_eq!(
        runtime
            .junction_at_anchor(SeasonalAnchor::SummerSolstice)
            .unwrap()
            .grove_phase,
        GrovePhase::TheGathering
    );
    assert_eq!(
        runtime
            .junction_at_anchor(SeasonalAnchor::AutumnEquinox)
            .unwrap()
            .grove_phase,
        GrovePhase::TheFestival
    );
}

#[test]
fn festival_of_mirrors_keeps_its_full_seasonal_name() {
    let decoded = decoded();
    let festival = decoded
        .annual_states
        .values()
        .next()
        .unwrap()
        .annual_cycle
        .seasonal_runtime
        .function_at_anchor(SeasonalAnchor::AutumnEquinox)
        .unwrap();
    assert_eq!(festival.canonical_name, "The Festival of Mirrors");
    assert_eq!(festival.kind, GreatFunctionKind::FestivalOfMirrors);
}

#[test]
fn central_junction_is_world_center_light_is_positive_and_dark_is_negative() {
    let decoded = decoded();
    let binding = &decoded
        .annual_states
        .values()
        .next()
        .unwrap()
        .annual_cycle
        .world_point_state
        .binding;
    assert_eq!(
        binding.central_junction_region_id.as_str(),
        CENTRAL_JUNCTION_REGION_ID
    );
    assert_eq!(binding.light_aura_region_id.as_str(), LIGHT_AURA_REGION_ID);
    assert_eq!(binding.dark_aura_region_id.as_str(), DARK_AURA_REGION_ID);
    assert_eq!(
        binding.classify(PhysicalPosition { x: 0, y: 10, z: 0 }),
        RelativePolarity::Positive
    );
    assert_eq!(
        binding.classify(PhysicalPosition::origin()),
        RelativePolarity::Center
    );
    assert_eq!(
        binding.classify(PhysicalPosition { x: 0, y: -10, z: 0 }),
        RelativePolarity::Negative
    );
}

#[test]
fn rendering_cannot_invert_orientation_and_polarity_is_not_lawfulness() {
    let decoded = decoded();
    let binding = &decoded
        .annual_states
        .values()
        .next()
        .unwrap()
        .annual_cycle
        .world_point_state
        .binding;
    assert!(!binding.presentation_may_change_constitutional_polarity);
    assert_eq!(
        binding
            .observe_lawfulness(
                RelativePolarity::Positive,
                ConstitutionalLawfulness::Unlawful
            )
            .lawfulness,
        ConstitutionalLawfulness::Unlawful
    );
    assert_eq!(
        binding
            .observe_lawfulness(RelativePolarity::Negative, ConstitutionalLawfulness::Lawful)
            .lawfulness,
        ConstitutionalLawfulness::Lawful
    );
}

#[test]
fn four_function_junctions_close_and_open_the_exact_house_seasons() {
    let decoded = decoded();
    let runtime = &decoded
        .annual_states
        .values()
        .next()
        .unwrap()
        .function_junction_runtime;
    let expected = [
        (
            SeasonalAnchor::WinterSolstice,
            House::Flynt,
            House::Glaushouse,
        ),
        (
            SeasonalAnchor::SpringEquinox,
            House::Glaushouse,
            House::Stonebend,
        ),
        (
            SeasonalAnchor::SummerSolstice,
            House::Stonebend,
            House::Sandmanor,
        ),
        (
            SeasonalAnchor::AutumnEquinox,
            House::Sandmanor,
            House::Flynt,
        ),
    ];
    assert_eq!(runtime.junctions().len(), 4);
    assert_eq!(runtime.seasons().len(), 4);
    for (anchor, outgoing, incoming) in expected {
        let junction = runtime.junction_at_anchor(anchor).unwrap();
        assert_eq!(junction.outgoing_house, outgoing);
        assert_eq!(junction.incoming_house, incoming);
        assert!(junction.outgoing_season_closed);
        assert!(junction.incoming_season_opened);
        assert!(junction.completed);
        assert_eq!(seasonal_handoff(anchor).0, outgoing);
    }
}

#[test]
fn function_junction_is_not_central_junction_or_a_fifth_function() {
    let decoded = decoded();
    let state = decoded.annual_states.values().next().unwrap();
    assert_eq!(state.annual_cycle.seasonal_runtime.functions().len(), 4);
    assert!(
        state
            .function_junction_runtime
            .junctions()
            .values()
            .all(|junction| {
                junction.physical_place_id == CENTRAL_JUNCTION_REGION_ID
                    && !junction.is_geographic_location
                    && !junction.is_great_function
            })
    );
}

#[test]
fn derrick_resolves_to_gathering_and_opens_sandmanor_season() {
    let decoded = decoded();
    let state = decoded.annual_states.values().next().unwrap();
    assert_eq!(
        state
            .annual_cycle
            .seasonal_runtime
            .resolve_name("Derrick")
            .unwrap()
            .kind,
        GreatFunctionKind::Gathering
    );
    assert_eq!(
        state
            .function_junction_runtime
            .junction_at_anchor(SeasonalAnchor::SummerSolstice)
            .unwrap()
            .incoming_house,
        House::Sandmanor
    );
}

#[test]
fn four_functions_have_the_exact_synchronization_phases() {
    let decoded = decoded();
    let runtime = &decoded
        .annual_states
        .values()
        .next()
        .unwrap()
        .function_junction_runtime;
    let expected = [
        (
            SeasonalAnchor::WinterSolstice,
            SynchronizationPhase::PhysicalToDigitalReturn,
        ),
        (
            SeasonalAnchor::SpringEquinox,
            SynchronizationPhase::DigitalToPhysicalIncarnation,
        ),
        (
            SeasonalAnchor::SummerSolstice,
            SynchronizationPhase::BidirectionalParticipation,
        ),
        (
            SeasonalAnchor::AutumnEquinox,
            SynchronizationPhase::ComparisonAndConfirmation,
        ),
    ];
    for (anchor, phase) in expected {
        assert_eq!(
            runtime
                .junction_at_anchor(anchor)
                .unwrap()
                .synchronization_phase,
            phase
        );
    }
}

#[test]
fn physical_and_digital_checkpoints_survive_replay_separately() {
    let decoded = decoded();
    for junction in decoded
        .annual_states
        .values()
        .next()
        .unwrap()
        .function_junction_runtime
        .junctions()
        .values()
    {
        assert_eq!(junction.physical_checkpoint.layer, WorldLayer::Physical);
        assert_eq!(junction.digital_checkpoint.layer, WorldLayer::Digital);
        assert_ne!(
            junction.physical_checkpoint.checkpoint_id,
            junction.digital_checkpoint.checkpoint_id
        );
        assert_eq!(
            junction.physical_checkpoint.status,
            CheckpointStatus::Completed
        );
        assert_eq!(
            junction.digital_checkpoint.status,
            CheckpointStatus::Completed
        );
    }
}

#[test]
fn practical_jokes_are_witnessed_cues_not_transition_authority() {
    let decoded = decoded();
    let jokes = decoded
        .annual_states
        .values()
        .next()
        .unwrap()
        .function_junction_runtime
        .practical_jokes();
    assert_eq!(jokes.len(), 4);
    assert!(
        jokes
            .values()
            .all(|joke| { !joke.replaces_astronomical_anchor && !joke.replaces_function_junction })
    );
    assert!(jokes.values().any(|joke| {
        joke.question == "So…who won?" && joke.answer.as_deref() == Some("Whoever cleans this up.")
    }));
}

#[test]
fn way_back_direction_and_stairway_ascent_remain_frozen() {
    let decoded = decoded();
    let passages = decoded
        .annual_states
        .values()
        .next()
        .unwrap()
        .annual_cycle
        .way_back_runtime
        .passages();
    assert!(passages.values().any(|passage| {
        passage.direction == WayBackDirection::DescendingFromStonebend
            && passage.origin_house == House::Stonebend
            && passage.destination_house == House::Flynt
    }));
    assert!(passages.values().any(|passage| {
        passage.direction == WayBackDirection::AscendingFromFlynt
            && passage.origin_house == House::Flynt
            && passage.destination_house == House::Stonebend
            && passage.expression == WayBackExpression::StairwayToHeaven
    }));
}

#[test]
fn current_sea_riptide_undertow_and_boardwalk_remain_distinct() {
    let decoded = decoded();
    let current = &decoded
        .annual_states
        .values()
        .next()
        .unwrap()
        .annual_cycle
        .current_sea_runtime;
    assert!(
        current
            .events()
            .values()
            .all(|event| { event.region_id.as_str() == CURRENT_SEA_REGION_ID })
    );
    assert!(
        current
            .events()
            .values()
            .any(|event| event.force == Some(CurrentSeaForce::Riptide))
    );
    assert!(
        current
            .events()
            .values()
            .any(|event| event.force == Some(CurrentSeaForce::Undertow))
    );
    assert!(
        current
            .passages()
            .values()
            .all(|passage| passage.route_id.as_str() == BOARDWALK_ROUTE_ID
                && !passage.grants_automatic_recog)
    );
}

#[test]
fn ceremony_and_function_venues_are_nested_in_the_right_functions() {
    let decoded = decoded();
    let seasonal = &decoded
        .annual_states
        .values()
        .next()
        .unwrap()
        .annual_cycle
        .seasonal_runtime;
    let initiation = seasonal
        .function_at_anchor(SeasonalAnchor::SpringEquinox)
        .unwrap();
    let gathering = seasonal
        .function_at_anchor(SeasonalAnchor::SummerSolstice)
        .unwrap();
    let festival = seasonal
        .function_at_anchor(SeasonalAnchor::AutumnEquinox)
        .unwrap();
    assert!(
        initiation
            .venue_ids
            .iter()
            .any(|venue| venue.as_str() == "venue.aura-field")
    );
    assert!(gathering.activities.contains(&FunctionActivity::Ceremony));
    assert!(
        gathering
            .venue_ids
            .iter()
            .any(|venue| venue.as_str() == "venue.aura-beach")
    );
    assert!(
        festival
            .venue_ids
            .iter()
            .any(|venue| venue.as_str() == "venue.aura-basin")
    );
}

#[test]
fn tournament_and_nonlethal_war_remain_nested_under_gathering() {
    let decoded = decoded();
    let annual = &decoded.annual_states.values().next().unwrap().annual_cycle;
    let gathering = annual
        .seasonal_runtime
        .function_at_anchor(SeasonalAnchor::SummerSolstice)
        .unwrap();
    assert!(
        gathering
            .activities
            .contains(&FunctionActivity::ServiceTournament)
    );
    assert!(gathering.activities.len() > 1);
    assert!(canonical_war_of_a_thousand_hues().nonlethal);
}

#[test]
fn four_laws_have_exact_house_owners() {
    assert_eq!(PermanenceLaw::Identity.authority_house(), House::Stonebend);
    assert_eq!(PermanenceLaw::Pattern.authority_house(), House::Sandmanor);
    assert_eq!(
        PermanenceLaw::Integrity.authority_house(),
        House::Glaushouse
    );
    assert_eq!(PermanenceLaw::Recognition.authority_house(), House::Flynt);
}

#[test]
fn only_stonebend_may_issue_the_final_permanence_seal() {
    let mut payload = canonical_function_junction_archive_fixture();
    payload.annual_records[0].permanence_seals[0].issuing_house = House::Flynt;
    assert!(encode_function_junction_archive(&payload).is_err());
}

#[test]
fn missing_proof_prevents_permanence() {
    let mut payload = canonical_function_junction_archive_fixture();
    let missing = payload.annual_records[0]
        .permanence_attestations
        .pop()
        .unwrap();
    assert!(
        payload.annual_records[0].permanence_petitions[0]
            .attestation_ids
            .contains(&missing.attestation_id)
    );
    assert!(encode_function_junction_archive(&payload).is_err());
}

#[test]
fn rejected_proof_prevents_sealing_without_erasing_the_petition() {
    let mut payload = canonical_function_junction_archive_fixture();
    payload.annual_records[0].permanence_attestations[0].status = AttestationStatus::Rejected;
    assert!(encode_function_junction_archive(&payload).is_err());
    payload.annual_records[0].permanence_seals.clear();
    payload.annual_records[0].permanence_changes.clear();
    payload.annual_records[0].permanence_tombstones.clear();
    let states = replay_function_junction_payload(&payload).unwrap();
    let permanence = &states.values().next().unwrap().permanence_runtime;
    assert_eq!(permanence.petitions().len(), 1);
    assert!(permanence.seals().is_empty());
}

#[test]
fn conflicting_subject_identities_prevent_permanence() {
    let mut payload = canonical_function_junction_archive_fixture();
    payload.annual_records[0].permanence_attestations[0].subject_id =
        PermanentSubjectId::new("subject.conflicting-copy").unwrap();
    assert!(encode_function_junction_archive(&payload).is_err());
}

#[test]
fn lawful_amendment_preserves_prior_version_history() {
    let decoded = decoded();
    let history = decoded
        .annual_states
        .values()
        .next()
        .unwrap()
        .permanence_runtime
        .histories()
        .values()
        .next()
        .unwrap();
    assert_eq!(history.versions.len(), 4);
    assert_eq!(history.versions[0].as_str(), "version.permanence.bridge.v1");
    assert_eq!(
        history.versions[1].as_str(),
        "version.permanence.bridge.v2-amended"
    );
}

#[test]
fn dissolution_creates_a_tombstone_instead_of_silent_deletion() {
    let decoded = decoded();
    let permanence = &decoded
        .annual_states
        .values()
        .next()
        .unwrap()
        .permanence_runtime;
    assert_eq!(permanence.tombstones().len(), 1);
    assert!(
        permanence
            .tombstones()
            .values()
            .all(|tombstone| !tombstone.silently_deletes_history)
    );
    assert!(
        permanence
            .histories()
            .values()
            .all(|history| history.tombstone_id.is_some())
    );
}

#[test]
fn permanence_allows_lawful_succession_and_is_not_immutability() {
    let decoded = decoded();
    let permanence = &decoded
        .annual_states
        .values()
        .next()
        .unwrap()
        .permanence_runtime;
    assert!(
        permanence
            .changes()
            .values()
            .any(|change| change.kind == PermanentChangeKind::Succession)
    );
    assert!(permanence.seals().values().all(|seal| !seal.immutable));
}

#[test]
fn unauthorized_permanent_change_is_illegal_hollowing() {
    let mut payload = canonical_function_junction_archive_fixture();
    payload.annual_records[0].permanence_changes[0].authorizing_house = House::Sandmanor;
    payload.annual_records[0].permanence_changes[0].stonebend_authority_id = None;
    assert!(encode_function_junction_archive(&payload).is_err());
}

#[test]
fn no_great_function_or_junction_transfers_sovereignty() {
    let decoded = decoded();
    let state = decoded.annual_states.values().next().unwrap();
    assert!(
        state
            .annual_cycle
            .seasonal_runtime
            .functions()
            .values()
            .all(|function| !function.transfers_permanent_sovereignty)
    );
    assert!(
        state
            .function_junction_runtime
            .junctions()
            .values()
            .all(|junction| !junction.transfers_sovereignty)
    );
}

#[test]
fn opposite_insertion_order_produces_identical_identity_and_bytes() {
    let payload = canonical_function_junction_archive_fixture();
    let mut opposite = payload.clone();
    let record = &mut opposite.annual_records[0];
    record.junctions.reverse();
    record.seasons.reverse();
    record.practical_jokes.reverse();
    record.grove_cycles.reverse();
    record.permanence_attestations.reverse();
    record.permanence_petitions.reverse();
    record.permanence_seals.reverse();
    record.permanence_changes.reverse();
    record.permanence_tombstones.reverse();
    assert_eq!(
        encode_function_junction_archive(&payload).unwrap(),
        encode_function_junction_archive(&opposite).unwrap()
    );
}

#[test]
fn archive_replay_is_deterministic_and_checksum_tampering_is_detected() {
    let payload = canonical_function_junction_archive_fixture();
    let bytes = encode_function_junction_archive(&payload).unwrap();
    let first = decode_function_junction_archive(&bytes).unwrap();
    let replayed_bytes = encode_function_junction_archive(&first.payload).unwrap();
    let second = decode_function_junction_archive(&replayed_bytes).unwrap();
    assert_eq!(first.payload, second.payload);
    assert_eq!(first.annual_states, second.annual_states);

    let mut text = String::from_utf8(bytes).unwrap();
    text = text.replacen(
        "\"issuing_house\":\"Stonebend\"",
        "\"issuing_house\":\"Flynt\"",
        1,
    );
    assert!(decode_function_junction_archive(text.as_bytes()).is_err());
}

#[test]
fn legacy_outer_archive_migrates_without_rewriting_nested_seasonal_identity() {
    let payload = canonical_function_junction_archive_fixture();
    let legacy = encode_legacy_function_junction_archive_v0(&payload).unwrap();
    let legacy_text = String::from_utf8(legacy.clone()).unwrap();
    assert!(legacy_text.contains("\"Return\""));
    assert!(legacy_text.contains("\"Incarnate\""));
    assert!(legacy_text.contains("\"Commune\""));
    assert!(legacy_text.contains("\"Confirm\""));
    let migrated = migrate_function_junction_archive(&legacy).unwrap();
    let migrated_text = String::from_utf8(migrated.clone()).unwrap();
    assert!(migrated_text.contains("\"TheWayBack\""));
    assert!(migrated_text.contains("\"TheFestival\""));
    let decoded = decode_function_junction_archive(&migrated).unwrap();
    assert_eq!(decoded.archive_version, FUNCTION_JUNCTION_ARCHIVE_VERSION);
    assert_eq!(decoded.payload, payload.canonicalized());
    assert_eq!(
        decoded.payload.annual_records[0].seasonal_archive,
        payload.annual_records[0].seasonal_archive.canonicalized()
    );
}

#[test]
fn universal_kernel_remains_free_of_hollow_grove_and_house_names() {
    let kernel = concat!(
        include_str!("../hollow-grove-kernel/src/lib.rs"),
        include_str!("../hollow-grove-kernel/src/oriented_point.rs")
    );
    for forbidden in [
        "Central Junction",
        "Light Aura",
        "Dark Aura",
        "Stonebend",
        "Sandmanor",
        "Glaushouse",
        "Flynt",
        "Function Junction",
        "Permanence Seal",
    ] {
        assert!(
            !kernel.contains(forbidden),
            "{forbidden} entered the kernel"
        );
    }
}
