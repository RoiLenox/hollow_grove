use std::io;
use std::path::{Path, PathBuf};

use hollow_grove::hueman_progression::resolve_active_vertical_slice_at;
use hollow_grove::hueman_support::{
    CURRENT_SYNTHESIS_ACTIVATION_GATE_ARTIFACT_PATH, CURRENT_SYNTHESIS_BASE_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_BEHAVIOR_RULES_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_COLLISION_RELAY_ARTIFACT_PATH, CURRENT_SYNTHESIS_CONSEQUENCE_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_CONTRACT_ARTIFACT_PATH, CURRENT_SYNTHESIS_EXECUTION_SPEC_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_OPERATIONAL_ARTIFACT_PATH, CURRENT_SYNTHESIS_SELECTION_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_SEQUENCE_ARTIFACT_PATH, CURRENT_SYNTHESIS_TOPOLOGY_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_TRANSITION_PM_TO_LE_ARTIFACT_PATH,
    build_hueman_archetype_lens_from_artifacts, build_hueman_aura_behavior_from_artifacts,
    build_hueman_aura_triad_from_artifacts, build_hueman_boundary_from_artifacts,
    build_hueman_crossover_scenes_from_artifacts, build_hueman_fourway_from_artifacts,
    build_hueman_glaushouse_roles_from_artifacts, build_hueman_inverse_circle_from_artifacts,
    build_hueman_link_physics_from_artifacts, build_hueman_motion_map_from_artifacts,
    build_hueman_path_crossovers_from_artifacts, build_hueman_procedural_uplift_from_artifacts,
    build_hueman_sandmanor_roles_from_artifacts, build_hueman_scene_drift_from_artifacts,
    build_hueman_scene_intent_from_artifacts, build_hueman_scene_presence_from_artifacts,
    build_hueman_start_choices_from_artifacts, build_hueman_start_paths_from_artifacts,
    build_hueman_stonebend_roles_from_artifacts, build_hueman_tross_helpers_from_artifacts,
    build_hueman_vertical_slice_for_spec_from_artifacts,
    build_vertical_integration_stack_from_artifacts, hueman_archetype_lens_artifact_path,
    hueman_aura_behavior_artifact_path, hueman_aura_triad_artifact_path,
    hueman_boundary_artifact_path, hueman_crossover_scenes_artifact_path,
    hueman_fourway_artifact_path, hueman_glaushouse_roles_artifact_path,
    hueman_inverse_circle_artifact_path, hueman_link_physics_artifact_path,
    hueman_motion_map_artifact_path, hueman_path_crossovers_artifact_path,
    hueman_procedural_uplift_artifact_path, hueman_sandmanor_roles_artifact_path,
    hueman_scene_drift_artifact_path, hueman_scene_intent_artifact_path,
    hueman_scene_presence_artifact_path, hueman_start_choices_artifact_path,
    hueman_start_paths_artifact_path, hueman_stonebend_roles_artifact_path,
    hueman_tross_helpers_artifact_path, hueman_vertical_slice_artifact_path,
    vertical_integration_stack_artifact_path,
};
use hollow_grove::{read_text_artifact, write_text_artifact};

fn run_hueman_at(root: &Path) -> io::Result<[PathBuf; 22]> {
    let current_synthesis_base =
        read_text_artifact(&root.join(CURRENT_SYNTHESIS_BASE_ARTIFACT_PATH))?;
    let current_synthesis_activation_gate =
        read_text_artifact(&root.join(CURRENT_SYNTHESIS_ACTIVATION_GATE_ARTIFACT_PATH))?;
    let hueman_boundary = build_hueman_boundary_from_artifacts(
        &current_synthesis_base,
        &current_synthesis_activation_gate,
    );
    let boundary_path = root.join(hueman_boundary_artifact_path());
    write_text_artifact(&boundary_path, &hueman_boundary)?;

    let current_synthesis_operational =
        read_text_artifact(&root.join(CURRENT_SYNTHESIS_OPERATIONAL_ARTIFACT_PATH))?;
    let hueman_motion_map =
        build_hueman_motion_map_from_artifacts(&hueman_boundary, &current_synthesis_operational);
    let motion_map_path = root.join(hueman_motion_map_artifact_path());
    write_text_artifact(&motion_map_path, &hueman_motion_map)?;

    let hueman_fourway = build_hueman_fourway_from_artifacts(&hueman_boundary, &hueman_motion_map);
    let fourway_path = root.join(hueman_fourway_artifact_path());
    write_text_artifact(&fourway_path, &hueman_fourway)?;

    let current_synthesis_topology =
        read_text_artifact(&root.join(CURRENT_SYNTHESIS_TOPOLOGY_ARTIFACT_PATH))?;
    let hueman_aura_triad =
        build_hueman_aura_triad_from_artifacts(&hueman_fourway, &current_synthesis_topology);
    let aura_triad_path = root.join(hueman_aura_triad_artifact_path());
    write_text_artifact(&aura_triad_path, &hueman_aura_triad)?;

    let hueman_start_choices =
        build_hueman_start_choices_from_artifacts(&hueman_fourway, &hueman_aura_triad);
    let start_choices_path = root.join(hueman_start_choices_artifact_path());
    write_text_artifact(&start_choices_path, &hueman_start_choices)?;

    let hueman_stonebend_roles =
        build_hueman_stonebend_roles_from_artifacts(&hueman_start_choices, &hueman_fourway);
    let stonebend_roles_path = root.join(hueman_stonebend_roles_artifact_path());
    write_text_artifact(&stonebend_roles_path, &hueman_stonebend_roles)?;

    let hueman_tross_helpers =
        build_hueman_tross_helpers_from_artifacts(&hueman_start_choices, &hueman_fourway);
    let tross_helpers_path = root.join(hueman_tross_helpers_artifact_path());
    write_text_artifact(&tross_helpers_path, &hueman_tross_helpers)?;

    let hueman_glaushouse_roles =
        build_hueman_glaushouse_roles_from_artifacts(&hueman_start_choices, &hueman_fourway);
    let glaushouse_roles_path = root.join(hueman_glaushouse_roles_artifact_path());
    write_text_artifact(&glaushouse_roles_path, &hueman_glaushouse_roles)?;

    let hueman_sandmanor_roles =
        build_hueman_sandmanor_roles_from_artifacts(&hueman_start_choices, &hueman_fourway);
    let sandmanor_roles_path = root.join(hueman_sandmanor_roles_artifact_path());
    write_text_artifact(&sandmanor_roles_path, &hueman_sandmanor_roles)?;

    let current_synthesis_collision_relay =
        read_text_artifact(&root.join(CURRENT_SYNTHESIS_COLLISION_RELAY_ARTIFACT_PATH))?;
    let current_synthesis_execution_spec =
        read_text_artifact(&root.join(CURRENT_SYNTHESIS_EXECUTION_SPEC_ARTIFACT_PATH))?;
    let current_synthesis_behavior_rules =
        read_text_artifact(&root.join(CURRENT_SYNTHESIS_BEHAVIOR_RULES_ARTIFACT_PATH))?;
    let current_synthesis_transition_pm_to_le =
        read_text_artifact(&root.join(CURRENT_SYNTHESIS_TRANSITION_PM_TO_LE_ARTIFACT_PATH))?;
    let current_synthesis_selection =
        read_text_artifact(&root.join(CURRENT_SYNTHESIS_SELECTION_ARTIFACT_PATH))?;
    let current_synthesis_consequence =
        read_text_artifact(&root.join(CURRENT_SYNTHESIS_CONSEQUENCE_ARTIFACT_PATH))?;
    let hueman_procedural_uplift = build_hueman_procedural_uplift_from_artifacts(
        &current_synthesis_execution_spec,
        &current_synthesis_behavior_rules,
        &current_synthesis_transition_pm_to_le,
        &current_synthesis_collision_relay,
        &current_synthesis_selection,
        &current_synthesis_consequence,
        &current_synthesis_activation_gate,
        &hueman_stonebend_roles,
        &hueman_tross_helpers,
        &hueman_glaushouse_roles,
        &hueman_sandmanor_roles,
    );
    let procedural_uplift_path = root.join(hueman_procedural_uplift_artifact_path());
    write_text_artifact(&procedural_uplift_path, &hueman_procedural_uplift)?;

    let hueman_aura_behavior =
        build_hueman_aura_behavior_from_artifacts(&hueman_aura_triad, &hueman_start_choices);
    let aura_behavior_path = root.join(hueman_aura_behavior_artifact_path());
    write_text_artifact(&aura_behavior_path, &hueman_aura_behavior)?;

    let active_slice = resolve_active_vertical_slice_at(root)?;
    let hueman_vertical_slice = build_hueman_vertical_slice_for_spec_from_artifacts(
        active_slice,
        &hueman_boundary,
        &hueman_start_choices,
        &hueman_aura_behavior,
        &hueman_procedural_uplift,
    );
    let vertical_slice_path = root.join(hueman_vertical_slice_artifact_path());
    write_text_artifact(&vertical_slice_path, &hueman_vertical_slice)?;

    let hueman_archetype_lens = build_hueman_archetype_lens_from_artifacts(
        &hueman_start_choices,
        &hueman_aura_behavior,
        &hueman_stonebend_roles,
        &hueman_sandmanor_roles,
    );
    let archetype_lens_path = root.join(hueman_archetype_lens_artifact_path());
    write_text_artifact(&archetype_lens_path, &hueman_archetype_lens)?;

    let hueman_start_paths =
        build_hueman_start_paths_from_artifacts(&hueman_start_choices, &hueman_archetype_lens);
    let start_paths_path = root.join(hueman_start_paths_artifact_path());
    write_text_artifact(&start_paths_path, &hueman_start_paths)?;

    let hueman_path_crossovers = build_hueman_path_crossovers_from_artifacts(
        &hueman_start_paths,
        &hueman_aura_behavior,
        &current_synthesis_collision_relay,
    );
    let path_crossovers_path = root.join(hueman_path_crossovers_artifact_path());
    write_text_artifact(&path_crossovers_path, &hueman_path_crossovers)?;

    let current_synthesis_sequence =
        read_text_artifact(&root.join(CURRENT_SYNTHESIS_SEQUENCE_ARTIFACT_PATH))?;
    let hueman_link_physics = build_hueman_link_physics_from_artifacts(
        &current_synthesis_sequence,
        &hueman_path_crossovers,
        &current_synthesis_collision_relay,
    );
    let link_physics_path = root.join(hueman_link_physics_artifact_path());
    write_text_artifact(&link_physics_path, &hueman_link_physics)?;

    let hueman_inverse_circle =
        build_hueman_inverse_circle_from_artifacts(&hueman_fourway, &hueman_link_physics);
    let inverse_circle_path = root.join(hueman_inverse_circle_artifact_path());
    write_text_artifact(&inverse_circle_path, &hueman_inverse_circle)?;

    let hueman_crossover_scenes = build_hueman_crossover_scenes_from_artifacts(
        &hueman_path_crossovers,
        &hueman_link_physics,
        &current_synthesis_collision_relay,
    );
    let crossover_scenes_path = root.join(hueman_crossover_scenes_artifact_path());
    write_text_artifact(&crossover_scenes_path, &hueman_crossover_scenes)?;

    let current_synthesis_contract =
        read_text_artifact(&root.join(CURRENT_SYNTHESIS_CONTRACT_ARTIFACT_PATH))?;
    let hueman_scene_presence = build_hueman_scene_presence_from_artifacts(
        &hueman_crossover_scenes,
        &hueman_archetype_lens,
        &hueman_stonebend_roles,
        &hueman_tross_helpers,
        &hueman_glaushouse_roles,
        &hueman_sandmanor_roles,
        &hueman_inverse_circle,
        &current_synthesis_collision_relay,
    );
    let scene_presence_path = root.join(hueman_scene_presence_artifact_path());
    write_text_artifact(&scene_presence_path, &hueman_scene_presence)?;

    let hueman_scene_intent = build_hueman_scene_intent_from_artifacts(
        &hueman_scene_presence,
        &hueman_link_physics,
        &current_synthesis_collision_relay,
        &current_synthesis_contract,
        &hueman_stonebend_roles,
        &hueman_tross_helpers,
        &hueman_glaushouse_roles,
        &hueman_sandmanor_roles,
        &hueman_inverse_circle,
    );
    let scene_intent_path = root.join(hueman_scene_intent_artifact_path());
    write_text_artifact(&scene_intent_path, &hueman_scene_intent)?;

    let hueman_scene_drift = build_hueman_scene_drift_from_artifacts(
        &hueman_scene_intent,
        &hueman_link_physics,
        &current_synthesis_collision_relay,
    );
    let scene_drift_path = root.join(hueman_scene_drift_artifact_path());
    write_text_artifact(&scene_drift_path, &hueman_scene_drift)?;

    let vertical_integration_stack = build_vertical_integration_stack_from_artifacts(
        &current_synthesis_base,
        &current_synthesis_collision_relay,
        &hueman_boundary,
        &hueman_glaushouse_roles,
        &hueman_sandmanor_roles,
        &hueman_inverse_circle,
        &hueman_procedural_uplift,
        &hueman_scene_presence,
        &hueman_scene_intent,
        &hueman_scene_drift,
    );
    let vertical_integration_stack_path = root.join(vertical_integration_stack_artifact_path());
    write_text_artifact(
        &vertical_integration_stack_path,
        &vertical_integration_stack,
    )?;

    Ok([
        boundary_path,
        motion_map_path,
        fourway_path,
        aura_triad_path,
        start_choices_path,
        stonebend_roles_path,
        tross_helpers_path,
        glaushouse_roles_path,
        sandmanor_roles_path,
        procedural_uplift_path,
        aura_behavior_path,
        vertical_slice_path,
        archetype_lens_path,
        start_paths_path,
        path_crossovers_path,
        link_physics_path,
        inverse_circle_path,
        crossover_scenes_path,
        scene_presence_path,
        scene_intent_path,
        scene_drift_path,
        vertical_integration_stack_path,
    ])
}

fn main() -> io::Result<()> {
    for artifact_path in run_hueman_at(Path::new("."))? {
        println!("{}", artifact_path.display());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use hollow_grove::hueman_support::{
        CURRENT_SYNTHESIS_ACTIVATION_GATE_ARTIFACT_PATH, CURRENT_SYNTHESIS_BASE_ARTIFACT_PATH,
        CURRENT_SYNTHESIS_BEHAVIOR_RULES_ARTIFACT_PATH,
        CURRENT_SYNTHESIS_COLLISION_RELAY_ARTIFACT_PATH,
        CURRENT_SYNTHESIS_CONSEQUENCE_ARTIFACT_PATH, CURRENT_SYNTHESIS_CONTRACT_ARTIFACT_PATH,
        CURRENT_SYNTHESIS_EXECUTION_SPEC_ARTIFACT_PATH,
        CURRENT_SYNTHESIS_OPERATIONAL_ARTIFACT_PATH, CURRENT_SYNTHESIS_SELECTION_ARTIFACT_PATH,
        CURRENT_SYNTHESIS_SEQUENCE_ARTIFACT_PATH, CURRENT_SYNTHESIS_TOPOLOGY_ARTIFACT_PATH,
        CURRENT_SYNTHESIS_TRANSITION_PM_TO_LE_ARTIFACT_PATH,
        build_hueman_archetype_lens_from_artifacts, build_hueman_aura_behavior_from_artifacts,
        build_hueman_aura_triad_from_artifacts, build_hueman_boundary_from_artifacts,
        build_hueman_crossover_scenes_from_artifacts, build_hueman_fourway_from_artifacts,
        build_hueman_glaushouse_roles_from_artifacts, build_hueman_inverse_circle_from_artifacts,
        build_hueman_link_physics_from_artifacts, build_hueman_motion_map_from_artifacts,
        build_hueman_path_crossovers_from_artifacts, build_hueman_procedural_uplift_from_artifacts,
        build_hueman_sandmanor_roles_from_artifacts, build_hueman_scene_drift_from_artifacts,
        build_hueman_scene_intent_from_artifacts, build_hueman_scene_presence_from_artifacts,
        build_hueman_start_choices_from_artifacts, build_hueman_start_paths_from_artifacts,
        build_hueman_stonebend_roles_from_artifacts, build_hueman_tross_helpers_from_artifacts,
        build_hueman_vertical_slice_from_artifacts,
        build_vertical_integration_stack_from_artifacts, hueman_archetype_lens_artifact_path,
        hueman_aura_behavior_artifact_path, hueman_aura_triad_artifact_path,
        hueman_boundary_artifact_path, hueman_crossover_scenes_artifact_path,
        hueman_fourway_artifact_path, hueman_glaushouse_roles_artifact_path,
        hueman_inverse_circle_artifact_path, hueman_link_physics_artifact_path,
        hueman_motion_map_artifact_path, hueman_path_crossovers_artifact_path,
        hueman_procedural_uplift_artifact_path, hueman_sandmanor_roles_artifact_path,
        hueman_scene_drift_artifact_path, hueman_scene_intent_artifact_path,
        hueman_scene_presence_artifact_path, hueman_start_choices_artifact_path,
        hueman_start_paths_artifact_path, hueman_stonebend_roles_artifact_path,
        hueman_tross_helpers_artifact_path, hueman_vertical_slice_artifact_path,
        vertical_integration_stack_artifact_path,
    };
    use hollow_grove::{read_text_artifact, write_text_artifact};

    use super::run_hueman_at;

    #[test]
    fn hueman_runner_regenerates_boundary_motion_map_fourway_aura_triad_start_choices_stonebend_roles_tross_helpers_aura_behavior_vertical_slice_archetype_lens_start_paths_path_crossovers_link_physics_crossover_scenes_scene_presence_scene_intent_scene_drift_and_vertical_integration_stack_in_order()
     {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_root = std::env::temp_dir().join(format!("hueman-runner-{nonce}"));

        write_text_artifact(
            &artifact_root.join(CURRENT_SYNTHESIS_BASE_ARTIFACT_PATH),
            "base",
        )
        .expect("current synthesis base fixture should write");
        write_text_artifact(
            &artifact_root.join(CURRENT_SYNTHESIS_ACTIVATION_GATE_ARTIFACT_PATH),
            "gate",
        )
        .expect("current synthesis activation gate fixture should write");
        write_text_artifact(
            &artifact_root.join(CURRENT_SYNTHESIS_OPERATIONAL_ARTIFACT_PATH),
            "ops",
        )
        .expect("current synthesis operational fixture should write");
        write_text_artifact(
            &artifact_root.join(CURRENT_SYNTHESIS_TOPOLOGY_ARTIFACT_PATH),
            "topology",
        )
        .expect("current synthesis topology fixture should write");
        write_text_artifact(
            &artifact_root.join(CURRENT_SYNTHESIS_SEQUENCE_ARTIFACT_PATH),
            "sequence",
        )
        .expect("current synthesis sequence fixture should write");
        write_text_artifact(
            &artifact_root.join(CURRENT_SYNTHESIS_EXECUTION_SPEC_ARTIFACT_PATH),
            "execution",
        )
        .expect("current synthesis execution spec fixture should write");
        write_text_artifact(
            &artifact_root.join(CURRENT_SYNTHESIS_BEHAVIOR_RULES_ARTIFACT_PATH),
            "rules",
        )
        .expect("current synthesis behavior rules fixture should write");
        write_text_artifact(
            &artifact_root.join(CURRENT_SYNTHESIS_TRANSITION_PM_TO_LE_ARTIFACT_PATH),
            "transition",
        )
        .expect("current synthesis transition fixture should write");
        write_text_artifact(
            &artifact_root.join(CURRENT_SYNTHESIS_SELECTION_ARTIFACT_PATH),
            "selection",
        )
        .expect("current synthesis selection fixture should write");
        write_text_artifact(
            &artifact_root.join(CURRENT_SYNTHESIS_CONTRACT_ARTIFACT_PATH),
            "contract",
        )
        .expect("current synthesis contract fixture should write");
        write_text_artifact(
            &artifact_root.join(CURRENT_SYNTHESIS_COLLISION_RELAY_ARTIFACT_PATH),
            "relay",
        )
        .expect("current synthesis collision relay fixture should write");
        write_text_artifact(
            &artifact_root.join(CURRENT_SYNTHESIS_CONSEQUENCE_ARTIFACT_PATH),
            "consequence",
        )
        .expect("current synthesis consequence fixture should write");

        let [
            boundary_path,
            motion_map_path,
            fourway_path,
            aura_triad_path,
            start_choices_path,
            stonebend_roles_path,
            tross_helpers_path,
            glaushouse_roles_path,
            sandmanor_roles_path,
            procedural_uplift_path,
            aura_behavior_path,
            vertical_slice_path,
            archetype_lens_path,
            start_paths_path,
            path_crossovers_path,
            link_physics_path,
            inverse_circle_path,
            crossover_scenes_path,
            scene_presence_path,
            scene_intent_path,
            scene_drift_path,
            vertical_integration_stack_path,
        ] = run_hueman_at(&artifact_root).expect("hueman should run");
        let hueman_boundary = build_hueman_boundary_from_artifacts("base", "gate");
        let hueman_motion_map = build_hueman_motion_map_from_artifacts(&hueman_boundary, "ops");
        let hueman_fourway =
            build_hueman_fourway_from_artifacts(&hueman_boundary, &hueman_motion_map);
        let hueman_aura_triad = build_hueman_aura_triad_from_artifacts(&hueman_fourway, "topology");
        let hueman_start_choices =
            build_hueman_start_choices_from_artifacts(&hueman_fourway, &hueman_aura_triad);
        let hueman_stonebend_roles =
            build_hueman_stonebend_roles_from_artifacts(&hueman_start_choices, &hueman_fourway);
        let hueman_tross_helpers =
            build_hueman_tross_helpers_from_artifacts(&hueman_start_choices, &hueman_fourway);
        let hueman_glaushouse_roles =
            build_hueman_glaushouse_roles_from_artifacts(&hueman_start_choices, &hueman_fourway);
        let hueman_sandmanor_roles =
            build_hueman_sandmanor_roles_from_artifacts(&hueman_start_choices, &hueman_fourway);
        let hueman_procedural_uplift = build_hueman_procedural_uplift_from_artifacts(
            "execution",
            "rules",
            "transition",
            "relay",
            "selection",
            "consequence",
            "gate",
            &hueman_stonebend_roles,
            &hueman_tross_helpers,
            &hueman_glaushouse_roles,
            &hueman_sandmanor_roles,
        );
        let hueman_aura_behavior =
            build_hueman_aura_behavior_from_artifacts(&hueman_aura_triad, &hueman_start_choices);
        let hueman_vertical_slice = build_hueman_vertical_slice_from_artifacts(
            &hueman_boundary,
            &hueman_start_choices,
            &hueman_aura_behavior,
            &hueman_procedural_uplift,
        );
        let hueman_archetype_lens = build_hueman_archetype_lens_from_artifacts(
            &hueman_start_choices,
            &hueman_aura_behavior,
            &hueman_stonebend_roles,
            &hueman_sandmanor_roles,
        );
        let hueman_start_paths =
            build_hueman_start_paths_from_artifacts(&hueman_start_choices, &hueman_archetype_lens);
        let hueman_path_crossovers = build_hueman_path_crossovers_from_artifacts(
            &hueman_start_paths,
            &hueman_aura_behavior,
            "relay",
        );
        let hueman_link_physics =
            build_hueman_link_physics_from_artifacts("sequence", &hueman_path_crossovers, "relay");
        let hueman_inverse_circle =
            build_hueman_inverse_circle_from_artifacts(&hueman_fourway, &hueman_link_physics);
        let hueman_crossover_scenes = build_hueman_crossover_scenes_from_artifacts(
            &hueman_path_crossovers,
            &hueman_link_physics,
            "relay",
        );
        let hueman_scene_presence = build_hueman_scene_presence_from_artifacts(
            &hueman_crossover_scenes,
            &hueman_archetype_lens,
            &hueman_stonebend_roles,
            &hueman_tross_helpers,
            &hueman_glaushouse_roles,
            &hueman_sandmanor_roles,
            &hueman_inverse_circle,
            "relay",
        );
        let hueman_scene_intent = build_hueman_scene_intent_from_artifacts(
            &hueman_scene_presence,
            &hueman_link_physics,
            "relay",
            "contract",
            &hueman_stonebend_roles,
            &hueman_tross_helpers,
            &hueman_glaushouse_roles,
            &hueman_sandmanor_roles,
            &hueman_inverse_circle,
        );
        let hueman_scene_drift = build_hueman_scene_drift_from_artifacts(
            &hueman_scene_intent,
            &hueman_link_physics,
            "relay",
        );
        let vertical_integration_stack = build_vertical_integration_stack_from_artifacts(
            "base",
            "relay",
            &hueman_boundary,
            &hueman_glaushouse_roles,
            &hueman_sandmanor_roles,
            &hueman_inverse_circle,
            &hueman_procedural_uplift,
            &hueman_scene_presence,
            &hueman_scene_intent,
            &hueman_scene_drift,
        );

        assert_eq!(
            boundary_path,
            artifact_root.join(hueman_boundary_artifact_path())
        );
        assert_eq!(
            motion_map_path,
            artifact_root.join(hueman_motion_map_artifact_path())
        );
        assert_eq!(
            fourway_path,
            artifact_root.join(hueman_fourway_artifact_path())
        );
        assert_eq!(
            aura_triad_path,
            artifact_root.join(hueman_aura_triad_artifact_path())
        );
        assert_eq!(
            start_choices_path,
            artifact_root.join(hueman_start_choices_artifact_path())
        );
        assert_eq!(
            stonebend_roles_path,
            artifact_root.join(hueman_stonebend_roles_artifact_path())
        );
        assert_eq!(
            tross_helpers_path,
            artifact_root.join(hueman_tross_helpers_artifact_path())
        );
        assert_eq!(
            glaushouse_roles_path,
            artifact_root.join(hueman_glaushouse_roles_artifact_path())
        );
        assert_eq!(
            sandmanor_roles_path,
            artifact_root.join(hueman_sandmanor_roles_artifact_path())
        );
        assert_eq!(
            procedural_uplift_path,
            artifact_root.join(hueman_procedural_uplift_artifact_path())
        );
        assert_eq!(
            aura_behavior_path,
            artifact_root.join(hueman_aura_behavior_artifact_path())
        );
        assert_eq!(
            vertical_slice_path,
            artifact_root.join(hueman_vertical_slice_artifact_path())
        );
        assert_eq!(
            archetype_lens_path,
            artifact_root.join(hueman_archetype_lens_artifact_path())
        );
        assert_eq!(
            start_paths_path,
            artifact_root.join(hueman_start_paths_artifact_path())
        );
        assert_eq!(
            path_crossovers_path,
            artifact_root.join(hueman_path_crossovers_artifact_path())
        );
        assert_eq!(
            link_physics_path,
            artifact_root.join(hueman_link_physics_artifact_path())
        );
        assert_eq!(
            inverse_circle_path,
            artifact_root.join(hueman_inverse_circle_artifact_path())
        );
        assert_eq!(
            crossover_scenes_path,
            artifact_root.join(hueman_crossover_scenes_artifact_path())
        );
        assert_eq!(
            scene_presence_path,
            artifact_root.join(hueman_scene_presence_artifact_path())
        );
        assert_eq!(
            scene_intent_path,
            artifact_root.join(hueman_scene_intent_artifact_path())
        );
        assert_eq!(
            scene_drift_path,
            artifact_root.join(hueman_scene_drift_artifact_path())
        );
        assert_eq!(
            vertical_integration_stack_path,
            artifact_root.join(vertical_integration_stack_artifact_path())
        );
        assert_eq!(
            read_text_artifact(&boundary_path).expect("hueman boundary artifact should read"),
            hueman_boundary
        );
        assert_eq!(
            read_text_artifact(&motion_map_path).expect("hueman motion map artifact should read"),
            hueman_motion_map
        );
        assert_eq!(
            read_text_artifact(&fourway_path).expect("hueman fourway artifact should read"),
            hueman_fourway
        );
        assert_eq!(
            read_text_artifact(&aura_triad_path).expect("hueman aura triad artifact should read"),
            hueman_aura_triad
        );
        assert_eq!(
            read_text_artifact(&start_choices_path)
                .expect("hueman start choices artifact should read"),
            hueman_start_choices
        );
        assert_eq!(
            read_text_artifact(&stonebend_roles_path)
                .expect("hueman stonebend roles artifact should read"),
            hueman_stonebend_roles
        );
        assert_eq!(
            read_text_artifact(&tross_helpers_path)
                .expect("hueman tross helpers artifact should read"),
            hueman_tross_helpers
        );
        assert_eq!(
            read_text_artifact(&glaushouse_roles_path)
                .expect("hueman glaushouse roles artifact should read"),
            hueman_glaushouse_roles
        );
        assert_eq!(
            read_text_artifact(&sandmanor_roles_path)
                .expect("hueman sandmanor roles artifact should read"),
            hueman_sandmanor_roles
        );
        assert_eq!(
            read_text_artifact(&procedural_uplift_path)
                .expect("hueman procedural uplift artifact should read"),
            hueman_procedural_uplift
        );
        assert_eq!(
            read_text_artifact(&aura_behavior_path)
                .expect("hueman aura behavior artifact should read"),
            hueman_aura_behavior
        );
        assert_eq!(
            read_text_artifact(&vertical_slice_path)
                .expect("hueman vertical slice artifact should read"),
            hueman_vertical_slice
        );
        assert_eq!(
            read_text_artifact(&archetype_lens_path)
                .expect("hueman archetype lens artifact should read"),
            hueman_archetype_lens
        );
        assert_eq!(
            read_text_artifact(&start_paths_path).expect("hueman start paths artifact should read"),
            hueman_start_paths
        );
        assert_eq!(
            read_text_artifact(&path_crossovers_path)
                .expect("hueman path crossovers artifact should read"),
            hueman_path_crossovers
        );
        assert_eq!(
            read_text_artifact(&link_physics_path)
                .expect("hueman link physics artifact should read"),
            hueman_link_physics
        );
        assert_eq!(
            read_text_artifact(&inverse_circle_path)
                .expect("hueman inverse circle artifact should read"),
            hueman_inverse_circle
        );
        assert_eq!(
            read_text_artifact(&crossover_scenes_path)
                .expect("hueman crossover scenes artifact should read"),
            hueman_crossover_scenes
        );
        assert_eq!(
            read_text_artifact(&scene_presence_path)
                .expect("hueman scene presence artifact should read"),
            hueman_scene_presence
        );
        assert_eq!(
            read_text_artifact(&scene_intent_path)
                .expect("hueman scene intent artifact should read"),
            hueman_scene_intent
        );
        assert_eq!(
            read_text_artifact(&scene_drift_path).expect("hueman scene drift artifact should read"),
            hueman_scene_drift
        );
        assert_eq!(
            read_text_artifact(&vertical_integration_stack_path)
                .expect("vertical integration stack artifact should read"),
            vertical_integration_stack
        );

        fs::remove_file(&artifact_root.join(CURRENT_SYNTHESIS_BASE_ARTIFACT_PATH))
            .expect("current synthesis base fixture should be removable");
        fs::remove_file(&artifact_root.join(CURRENT_SYNTHESIS_ACTIVATION_GATE_ARTIFACT_PATH))
            .expect("current synthesis activation gate fixture should be removable");
        fs::remove_file(&artifact_root.join(CURRENT_SYNTHESIS_OPERATIONAL_ARTIFACT_PATH))
            .expect("current synthesis operational fixture should be removable");
        fs::remove_file(&artifact_root.join(CURRENT_SYNTHESIS_TOPOLOGY_ARTIFACT_PATH))
            .expect("current synthesis topology fixture should be removable");
        fs::remove_file(&artifact_root.join(CURRENT_SYNTHESIS_SEQUENCE_ARTIFACT_PATH))
            .expect("current synthesis sequence fixture should be removable");
        fs::remove_file(&artifact_root.join(CURRENT_SYNTHESIS_EXECUTION_SPEC_ARTIFACT_PATH))
            .expect("current synthesis execution spec fixture should be removable");
        fs::remove_file(&artifact_root.join(CURRENT_SYNTHESIS_BEHAVIOR_RULES_ARTIFACT_PATH))
            .expect("current synthesis behavior rules fixture should be removable");
        fs::remove_file(&artifact_root.join(CURRENT_SYNTHESIS_TRANSITION_PM_TO_LE_ARTIFACT_PATH))
            .expect("current synthesis transition fixture should be removable");
        fs::remove_file(&artifact_root.join(CURRENT_SYNTHESIS_SELECTION_ARTIFACT_PATH))
            .expect("current synthesis selection fixture should be removable");
        fs::remove_file(&artifact_root.join(CURRENT_SYNTHESIS_CONSEQUENCE_ARTIFACT_PATH))
            .expect("current synthesis consequence fixture should be removable");
        fs::remove_file(&boundary_path).expect("hueman boundary artifact should be removable");
        fs::remove_file(&motion_map_path).expect("hueman motion map artifact should be removable");
        fs::remove_file(&fourway_path).expect("hueman fourway artifact should be removable");
        fs::remove_file(&aura_triad_path).expect("hueman aura triad artifact should be removable");
        fs::remove_file(&start_choices_path)
            .expect("hueman start choices artifact should be removable");
        fs::remove_file(&stonebend_roles_path)
            .expect("hueman stonebend roles artifact should be removable");
        fs::remove_file(&tross_helpers_path)
            .expect("hueman tross helpers artifact should be removable");
        fs::remove_file(&sandmanor_roles_path)
            .expect("hueman sandmanor roles artifact should be removable");
        fs::remove_file(&procedural_uplift_path)
            .expect("hueman procedural uplift artifact should be removable");
        fs::remove_file(&aura_behavior_path)
            .expect("hueman aura behavior artifact should be removable");
        fs::remove_file(&vertical_slice_path)
            .expect("hueman vertical slice artifact should be removable");
        fs::remove_file(&archetype_lens_path)
            .expect("hueman archetype lens artifact should be removable");
        fs::remove_file(&start_paths_path)
            .expect("hueman start paths artifact should be removable");
        fs::remove_file(&path_crossovers_path)
            .expect("hueman path crossovers artifact should be removable");
        fs::remove_file(&link_physics_path)
            .expect("hueman link physics artifact should be removable");
        fs::remove_file(&inverse_circle_path)
            .expect("hueman inverse circle artifact should be removable");
        fs::remove_file(&crossover_scenes_path)
            .expect("hueman crossover scenes artifact should be removable");
        fs::remove_file(&scene_presence_path)
            .expect("hueman scene presence artifact should be removable");
        fs::remove_file(&scene_intent_path)
            .expect("hueman scene intent artifact should be removable");
        fs::remove_file(&scene_drift_path)
            .expect("hueman scene drift artifact should be removable");
        fs::remove_file(&vertical_integration_stack_path)
            .expect("vertical integration stack artifact should be removable");
        fs::remove_dir_all(artifact_root.join("artifacts"))
            .expect("artifact fixture directory should be removable");
        fs::remove_dir(&artifact_root).expect("artifact root should be removable");
    }
}
