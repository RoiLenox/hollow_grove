use std::io;
use std::path::{Path, PathBuf};

use hollow_grove::hueman_support::{
    CURRENT_SYNTHESIS_ACTIVATION_GATE_ARTIFACT_PATH, CURRENT_SYNTHESIS_BASE_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_OPERATIONAL_ARTIFACT_PATH, CURRENT_SYNTHESIS_SEQUENCE_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_TOPOLOGY_ARTIFACT_PATH,
    build_hueman_archetype_lens_from_artifacts, build_hueman_aura_behavior_from_artifacts,
    build_hueman_aura_triad_from_artifacts, build_hueman_boundary_from_artifacts,
    build_hueman_crossover_scenes_from_artifacts,
    build_hueman_fourway_from_artifacts, build_hueman_motion_map_from_artifacts,
    build_hueman_link_physics_from_artifacts,
    build_hueman_path_crossovers_from_artifacts,
    build_hueman_start_choices_from_artifacts, build_hueman_start_paths_from_artifacts,
    hueman_archetype_lens_artifact_path, hueman_aura_behavior_artifact_path,
    hueman_crossover_scenes_artifact_path,
    hueman_link_physics_artifact_path,
    hueman_path_crossovers_artifact_path,
    hueman_aura_triad_artifact_path, hueman_boundary_artifact_path,
    hueman_fourway_artifact_path, hueman_motion_map_artifact_path,
    hueman_start_choices_artifact_path, hueman_start_paths_artifact_path,
};
use hollow_grove::{read_text_artifact, write_text_artifact};

fn run_hueman_at(root: &Path) -> io::Result<[PathBuf; 11]> {
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

    let hueman_aura_behavior =
        build_hueman_aura_behavior_from_artifacts(&hueman_aura_triad, &hueman_start_choices);
    let aura_behavior_path = root.join(hueman_aura_behavior_artifact_path());
    write_text_artifact(&aura_behavior_path, &hueman_aura_behavior)?;

    let hueman_archetype_lens =
        build_hueman_archetype_lens_from_artifacts(&hueman_start_choices, &hueman_aura_behavior);
    let archetype_lens_path = root.join(hueman_archetype_lens_artifact_path());
    write_text_artifact(&archetype_lens_path, &hueman_archetype_lens)?;

    let hueman_start_paths =
        build_hueman_start_paths_from_artifacts(&hueman_start_choices, &hueman_archetype_lens);
    let start_paths_path = root.join(hueman_start_paths_artifact_path());
    write_text_artifact(&start_paths_path, &hueman_start_paths)?;

    let hueman_path_crossovers =
        build_hueman_path_crossovers_from_artifacts(&hueman_start_paths, &hueman_aura_behavior);
    let path_crossovers_path = root.join(hueman_path_crossovers_artifact_path());
    write_text_artifact(&path_crossovers_path, &hueman_path_crossovers)?;

    let current_synthesis_sequence =
        read_text_artifact(&root.join(CURRENT_SYNTHESIS_SEQUENCE_ARTIFACT_PATH))?;
    let hueman_link_physics =
        build_hueman_link_physics_from_artifacts(&current_synthesis_sequence, &hueman_path_crossovers);
    let link_physics_path = root.join(hueman_link_physics_artifact_path());
    write_text_artifact(&link_physics_path, &hueman_link_physics)?;

    let hueman_crossover_scenes =
        build_hueman_crossover_scenes_from_artifacts(&hueman_path_crossovers, &hueman_link_physics);
    let crossover_scenes_path = root.join(hueman_crossover_scenes_artifact_path());
    write_text_artifact(&crossover_scenes_path, &hueman_crossover_scenes)?;

    Ok([
        boundary_path,
        motion_map_path,
        fourway_path,
        aura_triad_path,
        start_choices_path,
        aura_behavior_path,
        archetype_lens_path,
        start_paths_path,
        path_crossovers_path,
        link_physics_path,
        crossover_scenes_path,
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
        CURRENT_SYNTHESIS_OPERATIONAL_ARTIFACT_PATH, CURRENT_SYNTHESIS_SEQUENCE_ARTIFACT_PATH,
        CURRENT_SYNTHESIS_TOPOLOGY_ARTIFACT_PATH,
        build_hueman_archetype_lens_from_artifacts, build_hueman_aura_behavior_from_artifacts,
        build_hueman_aura_triad_from_artifacts, build_hueman_boundary_from_artifacts,
        build_hueman_crossover_scenes_from_artifacts,
        build_hueman_fourway_from_artifacts, build_hueman_motion_map_from_artifacts,
        build_hueman_link_physics_from_artifacts,
        build_hueman_path_crossovers_from_artifacts,
        build_hueman_start_choices_from_artifacts, build_hueman_start_paths_from_artifacts,
        hueman_archetype_lens_artifact_path, hueman_aura_behavior_artifact_path,
        hueman_crossover_scenes_artifact_path,
        hueman_link_physics_artifact_path,
        hueman_path_crossovers_artifact_path,
        hueman_aura_triad_artifact_path, hueman_boundary_artifact_path,
        hueman_fourway_artifact_path, hueman_motion_map_artifact_path,
        hueman_start_choices_artifact_path, hueman_start_paths_artifact_path,
    };
    use hollow_grove::{read_text_artifact, write_text_artifact};

    use super::run_hueman_at;

    #[test]
    fn hueman_runner_regenerates_boundary_motion_map_fourway_aura_triad_start_choices_aura_behavior_archetype_lens_start_paths_path_crossovers_link_physics_and_crossover_scenes_in_order() {
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

        let [
            boundary_path,
            motion_map_path,
            fourway_path,
            aura_triad_path,
            start_choices_path,
            aura_behavior_path,
            archetype_lens_path,
            start_paths_path,
            path_crossovers_path,
            link_physics_path,
            crossover_scenes_path,
        ] =
            run_hueman_at(&artifact_root).expect("hueman should run");
        let hueman_boundary = build_hueman_boundary_from_artifacts("base", "gate");
        let hueman_motion_map = build_hueman_motion_map_from_artifacts(&hueman_boundary, "ops");
        let hueman_fourway =
            build_hueman_fourway_from_artifacts(&hueman_boundary, &hueman_motion_map);
        let hueman_aura_triad =
            build_hueman_aura_triad_from_artifacts(&hueman_fourway, "topology");
        let hueman_start_choices =
            build_hueman_start_choices_from_artifacts(&hueman_fourway, &hueman_aura_triad);
        let hueman_aura_behavior =
            build_hueman_aura_behavior_from_artifacts(&hueman_aura_triad, &hueman_start_choices);
        let hueman_archetype_lens =
            build_hueman_archetype_lens_from_artifacts(&hueman_start_choices, &hueman_aura_behavior);
        let hueman_start_paths =
            build_hueman_start_paths_from_artifacts(&hueman_start_choices, &hueman_archetype_lens);
        let hueman_path_crossovers =
            build_hueman_path_crossovers_from_artifacts(&hueman_start_paths, &hueman_aura_behavior);
        let hueman_link_physics =
            build_hueman_link_physics_from_artifacts("sequence", &hueman_path_crossovers);
        let hueman_crossover_scenes =
            build_hueman_crossover_scenes_from_artifacts(&hueman_path_crossovers, &hueman_link_physics);

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
            aura_behavior_path,
            artifact_root.join(hueman_aura_behavior_artifact_path())
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
            crossover_scenes_path,
            artifact_root.join(hueman_crossover_scenes_artifact_path())
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
            read_text_artifact(&aura_behavior_path)
                .expect("hueman aura behavior artifact should read"),
            hueman_aura_behavior
        );
        assert_eq!(
            read_text_artifact(&archetype_lens_path)
                .expect("hueman archetype lens artifact should read"),
            hueman_archetype_lens
        );
        assert_eq!(
            read_text_artifact(&start_paths_path)
                .expect("hueman start paths artifact should read"),
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
            read_text_artifact(&crossover_scenes_path)
                .expect("hueman crossover scenes artifact should read"),
            hueman_crossover_scenes
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
        fs::remove_file(&boundary_path).expect("hueman boundary artifact should be removable");
        fs::remove_file(&motion_map_path).expect("hueman motion map artifact should be removable");
        fs::remove_file(&fourway_path).expect("hueman fourway artifact should be removable");
        fs::remove_file(&aura_triad_path).expect("hueman aura triad artifact should be removable");
        fs::remove_file(&start_choices_path)
            .expect("hueman start choices artifact should be removable");
        fs::remove_file(&aura_behavior_path)
            .expect("hueman aura behavior artifact should be removable");
        fs::remove_file(&archetype_lens_path)
            .expect("hueman archetype lens artifact should be removable");
        fs::remove_file(&start_paths_path)
            .expect("hueman start paths artifact should be removable");
        fs::remove_file(&path_crossovers_path)
            .expect("hueman path crossovers artifact should be removable");
        fs::remove_file(&link_physics_path)
            .expect("hueman link physics artifact should be removable");
        fs::remove_file(&crossover_scenes_path)
            .expect("hueman crossover scenes artifact should be removable");
        fs::remove_dir_all(artifact_root.join("artifacts"))
            .expect("artifact fixture directory should be removable");
        fs::remove_dir(&artifact_root).expect("artifact root should be removable");
    }
}
