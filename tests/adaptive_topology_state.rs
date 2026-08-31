use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use hollow_grove::NormalizedCoordinate;
use hollow_grove::constitutional::SeasonalAnchor;
use hollow_grove::world::adaptive_topology_live::{
    ADAPTIVE_TOPOLOGY_REBUILD_ACK_ARTIFACT_PATH, ADAPTIVE_TOPOLOGY_REBUILD_COMMAND_ARTIFACT_PATH,
    AdaptiveTopologyRebuildAcknowledgementV1, AdaptiveTopologyRebuildCommandStatus,
    AdaptiveTopologyRebuildCommandV1, build_adaptive_topology_rebuild_command,
    commit_adaptive_topology_rebuild, process_adaptive_topology_live_once,
    submit_adaptive_topology_seasonal_event,
};
use hollow_grove::world::adaptive_topology_seasonal::{
    SeasonalAdaptiveTopologyDisposition, apply_completed_function_junction,
};
use hollow_grove::world::adaptive_topology_state::{
    AdaptiveRouteAttachment, AdaptiveTopologyStateV1, decode_adaptive_topology_state,
    encode_adaptive_topology_state,
};
use hollow_grove::world::function_junction_archive::{
    encode_function_junction_archive, replay_function_junction_payload,
};
use hollow_grove::world::function_junction_fixture::canonical_function_junction_archive_fixture;
use hollow_grove::world::spatial_topology_v2::TopologyV2RouteId;

const CHECKED_STATE: &str =
    include_str!("../artifacts/hollow_grove_adaptive_topology_state_v1.json");
const GODOT_SHELL: &str = include_str!("../hueman_godot/scripts/hueman_screen_shell.gd");
const GODOT_LAUNCHER: &str = include_str!("../run-hueman-godot.sh");
const SPEC: &str = include_str!("../HOLLOW_GROVE_ADAPTIVE_TOPOLOGY_STATE_V1.md");
const LIVE_REHEARSAL_SPEC: &str =
    include_str!("../HOLLOW_GROVE_ADAPTIVE_TOPOLOGY_LIVE_REHEARSAL_V1.md");
const LIVE_REHEARSAL: &str =
    include_str!("../artifacts/hollow_grove_adaptive_topology_live_rehearsal_v1.json");
const REHEARSAL_ARCHIVE: &[u8] =
    include_bytes!("../artifacts/hollow_grove_function_junction_rehearsal_archive_v1.json");

#[test]
fn checked_artifact_is_the_canonical_valid_circular_state() {
    let checked: serde_json::Value = serde_json::from_str(CHECKED_STATE).unwrap();
    let canonical = AdaptiveTopologyStateV1::circular();
    assert_eq!(checked, serde_json::to_value(&canonical).unwrap());
    assert_eq!(
        decode_adaptive_topology_state(CHECKED_STATE.as_bytes()).unwrap(),
        canonical
    );
}

#[test]
fn persistence_and_traveler_attachment_survive_every_committed_pose() {
    let attachment = AdaptiveRouteAttachment::new(
        "traveler.integration.current-sea",
        TopologyV2RouteId::CurrentSea,
        NormalizedCoordinate::midpoint(),
    )
    .unwrap();
    let mut state = AdaptiveTopologyStateV1::circular();
    let annual_state =
        replay_function_junction_payload(&canonical_function_junction_archive_fixture())
            .unwrap()
            .into_values()
            .next()
            .unwrap();
    let runtime = annual_state.function_junction_runtime;
    let spring = &runtime
        .junction_at_anchor(SeasonalAnchor::SpringEquinox)
        .unwrap()
        .junction_id;
    assert!(matches!(
        apply_completed_function_junction(&mut state, &runtime, spring).unwrap(),
        SeasonalAdaptiveTopologyDisposition::TransitionStarted {
            duration_ticks: 4,
            ..
        }
    ));

    for expected in [250_000_000, 500_000_000, 750_000_000, 1_000_000_000] {
        let prepared = state.prepare_next_step().unwrap().unwrap();
        let evidence = prepared
            .acknowledge_rebuilds(
                format!("collision.snapshot.integration.r{}", prepared.next_revision),
                format!(
                    "navigation.snapshot.integration.r{}",
                    prepared.next_revision
                ),
            )
            .unwrap();
        state.commit_prepared_step(&prepared, evidence).unwrap();
        state = decode_adaptive_topology_state(&encode_adaptive_topology_state(&state).unwrap())
            .unwrap();

        let projection = state.project_traveler(&attachment).unwrap();
        assert_eq!(projection.route, TopologyV2RouteId::CurrentSea);
        assert_eq!(projection.route_progress, NormalizedCoordinate::midpoint());
        assert_eq!(projection.rectangularity.parts(), expected);
    }
}

#[test]
fn public_live_bridge_commits_four_kernel_geometry_handshakes() {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let root = std::env::temp_dir().join(format!(
        "hollow-grove-adaptive-topology-integration-{}-{nonce}",
        std::process::id()
    ));
    fs::create_dir_all(root.join("artifacts")).unwrap();

    let archive = canonical_function_junction_archive_fixture();
    let spring = archive.annual_records[0]
        .junctions
        .iter()
        .find(|junction| junction.anchor == SeasonalAnchor::SpringEquinox)
        .unwrap()
        .junction_id
        .as_str()
        .to_owned();
    submit_adaptive_topology_seasonal_event(
        &root,
        &encode_function_junction_archive(&archive).unwrap(),
        &spring,
    )
    .unwrap();
    process_adaptive_topology_live_once(&root).unwrap();

    for (revision, rectangularity) in [
        (2, 250_000_000),
        (3, 500_000_000),
        (4, 750_000_000),
        (5, 1_000_000_000),
    ] {
        let command: AdaptiveTopologyRebuildCommandV1 = serde_json::from_slice(
            &fs::read(root.join(ADAPTIVE_TOPOLOGY_REBUILD_COMMAND_ARTIFACT_PATH)).unwrap(),
        )
        .unwrap();
        assert_eq!(
            command.status,
            AdaptiveTopologyRebuildCommandStatus::Prepared
        );
        assert_eq!(command.geometry.rectangularity.parts(), rectangularity);
        assert_eq!(command.geometry.routes.len(), 9);
        assert!(
            command
                .geometry
                .routes
                .iter()
                .all(|route| route.points.len() == 33)
        );

        let acknowledgement =
            AdaptiveTopologyRebuildAcknowledgementV1::for_command(&command, 9, 1).unwrap();
        hollow_grove::write_bytes_atomically(
            &root.join(ADAPTIVE_TOPOLOGY_REBUILD_ACK_ARTIFACT_PATH),
            &serde_json::to_vec_pretty(&acknowledgement).unwrap(),
        )
        .unwrap();
        let report = process_adaptive_topology_live_once(&root).unwrap();
        assert_eq!(report.committed_revision, Some(revision));
    }

    let state = decode_adaptive_topology_state(
        &fs::read(root.join("artifacts/hollow_grove_adaptive_topology_state_v1.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(state.revision(), 5);
    assert_eq!(state.rectangularity().parts(), 1_000_000_000);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn checked_live_rehearsal_replays_all_five_recorded_geometry_poses() {
    let baseline: serde_json::Value = serde_json::from_str(LIVE_REHEARSAL).unwrap();
    assert_eq!(baseline["acceptance"]["result"], "passed");
    assert_eq!(
        baseline["visual_capture"]["fresh_capture_matches_approved_renders"],
        true
    );
    assert_eq!(
        REHEARSAL_ARCHIVE,
        encode_function_junction_archive(&canonical_function_junction_archive_fixture())
            .unwrap()
            .as_slice()
    );

    let poses = baseline["poses"].as_array().unwrap();
    assert_eq!(poses.len(), 5);
    assert_eq!(
        poses
            .iter()
            .map(|pose| pose["percentage"].as_u64().unwrap())
            .collect::<Vec<_>>(),
        [0, 25, 50, 75, 100]
    );
    assert!(poses.iter().all(|pose| {
        pose["collision_polygon_count"] == 9
            && pose["navigation_region_count"] == 1
            && pose["result"] == "passed"
    }));

    let mut state = AdaptiveTopologyStateV1::circular();
    let circle = build_adaptive_topology_rebuild_command(&state).unwrap();
    assert_eq!(poses[0]["command_id"], circle.command_id);
    assert_eq!(poses[0]["geometry_digest"], circle.geometry.geometry_digest);

    let annual_state =
        replay_function_junction_payload(&canonical_function_junction_archive_fixture())
            .unwrap()
            .into_values()
            .next()
            .unwrap();
    let runtime = annual_state.function_junction_runtime;
    let spring = &runtime
        .junction_at_anchor(SeasonalAnchor::SpringEquinox)
        .unwrap()
        .junction_id;
    apply_completed_function_junction(&mut state, &runtime, spring).unwrap();

    for pose in &poses[1..] {
        let command = build_adaptive_topology_rebuild_command(&state).unwrap();
        assert_eq!(pose["command_id"], command.command_id);
        assert_eq!(pose["geometry_digest"], command.geometry.geometry_digest);
        let acknowledgement =
            AdaptiveTopologyRebuildAcknowledgementV1::for_command(&command, 9, 1).unwrap();
        commit_adaptive_topology_rebuild(&mut state, &acknowledgement).unwrap();
        assert_eq!(pose["state_revision"], state.revision());
        assert_eq!(pose["rectangularity"], state.rectangularity().parts());
    }
    let rectangle = build_adaptive_topology_rebuild_command(&state).unwrap();
    assert_eq!(
        poses[4]["restored_command_id"],
        rectangle.command_id.as_str()
    );
    for required in [
        "All five poses were realized by Godot",
        "nine collision polygons and one baked navigation union",
        "five SHA-256 digests match the approved PNGs exactly",
        "checked canonical state remains the circular revision-0 state",
    ] {
        assert!(LIVE_REHEARSAL_SPEC.contains(required));
    }
}

#[test]
fn godot_consumes_world_state_without_conflating_it_with_developer_override() {
    for required in [
        "ADAPTIVE_TOPOLOGY_STATE_RELATIVE_PATH",
        "ADAPTIVE_TOPOLOGY_REBUILD_COMMAND_RELATIVE_PATH",
        "ADAPTIVE_TOPOLOGY_REBUILD_ACK_RELATIVE_PATH",
        "_apply_adaptive_topology_world_state",
        "adaptive_morph_developer_override = true",
        "_on_use_adaptive_world_state_pressed",
        "Use world state",
        "CollisionPolygon2D.new()",
        "NavigationRegion2D.new()",
        "NavigationServer2D.bake_from_source_geometry_data",
        "NavigationServer2D.map_force_update",
        "_adaptive_topology_geometry_digest",
        "DirAccess.rename_absolute",
    ] {
        assert!(
            GODOT_SHELL.contains(required),
            "Godot shell missing {required}"
        );
    }
    for required in [
        "adaptive_topology_live",
        "serve --root \"$SCRIPT_DIR\"",
        "HOLLOW_GROVE_TOPOLOGY_POLL_MS",
    ] {
        assert!(
            GODOT_LAUNCHER.contains(required),
            "Godot launcher missing {required}"
        );
    }
    for required in [
        "Collision and navigation systems rebuild",
        "fixed-point route progress",
        "does not rewrite frozen gameplay or compatibility archives",
        "never mutates the persisted state",
        "The Initiation",
        "The Gathering",
        "Festival of Mirrors",
        "The Way Back",
        "Live execution bridge",
        "33 fixed-point samples",
        "atomically replaces both the durable state and the next rebuild command",
    ] {
        assert!(
            SPEC.contains(required),
            "state specification missing {required}"
        );
    }
}
