use hollow_grove::NormalizedCoordinate;
use hollow_grove::constitutional::SeasonalAnchor;
use hollow_grove::world::adaptive_topology_seasonal::apply_completed_function_junction;
use hollow_grove::world::adaptive_topology_state::{
    AdaptiveRouteAttachment, AdaptiveTopologyStateV1, adaptive_topology_state_digest,
    decode_adaptive_topology_state, encode_adaptive_topology_state,
    encode_adaptive_topology_state_pretty,
};
use hollow_grove::world::function_junction_archive::replay_function_junction_payload;
use hollow_grove::world::function_junction_fixture::canonical_function_junction_archive_fixture;
use hollow_grove::world::spatial_topology_v2::TopologyV2RouteId;

pub(crate) fn tool_main() -> Result<(), Box<dyn std::error::Error>> {
    let mut state = AdaptiveTopologyStateV1::circular();
    let traveler = AdaptiveRouteAttachment::new(
        "traveler.audit.current-sea",
        TopologyV2RouteId::CurrentSea,
        NormalizedCoordinate::midpoint(),
    )?;

    println!("# Hollow Grove Adaptive Topology State V1 Audit\n");
    println!("## Initial persisted state\n");
    print!("{}", encode_adaptive_topology_state_pretty(&state)?);
    println!("digest: {}", adaptive_topology_state_digest(&state)?);

    let annual_state =
        replay_function_junction_payload(&canonical_function_junction_archive_fixture())?
            .into_values()
            .next()
            .expect("canonical fixture contains one annual state");
    let runtime = annual_state.function_junction_runtime;
    let winter = runtime
        .junction_at_anchor(SeasonalAnchor::WinterSolstice)
        .expect("canonical Winter Function Junction")
        .junction_id
        .clone();
    let spring = runtime
        .junction_at_anchor(SeasonalAnchor::SpringEquinox)
        .expect("canonical Spring Function Junction")
        .junction_id
        .clone();
    let summer = runtime
        .junction_at_anchor(SeasonalAnchor::SummerSolstice)
        .expect("canonical Summer Function Junction")
        .junction_id
        .clone();
    let autumn = runtime
        .junction_at_anchor(SeasonalAnchor::AutumnEquinox)
        .expect("canonical Autumn Function Junction")
        .junction_id
        .clone();

    println!("\n## Validated Function Junction bindings\n");
    println!(
        "- Winter / The Way Back: {:?}",
        apply_completed_function_junction(&mut state, &runtime, &winter)?
    );
    println!(
        "- Spring / The Initiation: {:?}",
        apply_completed_function_junction(&mut state, &runtime, &spring)?
    );

    println!("\n## Initiation prepare, rebuild, commit sequence\n");
    while let Some(prepared) = state.prepare_next_step()? {
        let evidence = prepared.acknowledge_rebuilds(
            format!("collision.snapshot.audit.r{}", prepared.next_revision),
            format!("navigation.snapshot.audit.r{}", prepared.next_revision),
        )?;
        let commit = state.commit_prepared_step(&prepared, evidence)?;

        // Exercise the same persistence boundary a real world-state owner uses.
        let encoded = encode_adaptive_topology_state(&state)?;
        state = decode_adaptive_topology_state(&encoded)?;
        let projection = state.project_traveler(&traveler)?;
        println!(
            "- revision {}: rectangularity={} collision={} navigation={} traveler={}@{} -> ({}, {}) complete={}",
            commit.revision,
            commit.rectangularity.parts(),
            commit.collision_snapshot_id,
            commit.navigation_snapshot_id,
            projection.route.stable_id(),
            projection.route_progress.parts(),
            projection.position.x.parts(),
            projection.position.y.parts(),
            commit.transition_completed,
        );
    }

    println!(
        "\n- Summer / The Gathering: {:?}",
        apply_completed_function_junction(&mut state, &runtime, &summer)?
    );
    println!(
        "- Autumn / The Festival of Mirrors: {:?}",
        apply_completed_function_junction(&mut state, &runtime, &autumn)?
    );

    println!("\n## Final persisted state\n");
    print!("{}", encode_adaptive_topology_state_pretty(&state)?);
    println!("phase: {:?}", state.phase());
    println!("digest: {}", adaptive_topology_state_digest(&state)?);
    Ok(())
}
