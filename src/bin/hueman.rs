use std::io;
use std::path::{Path, PathBuf};

use hollow_grove::hueman_support::{
    CURRENT_SYNTHESIS_ACTIVATION_GATE_ARTIFACT_PATH, CURRENT_SYNTHESIS_BASE_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_OPERATIONAL_ARTIFACT_PATH, CURRENT_SYNTHESIS_TOPOLOGY_ARTIFACT_PATH,
    build_hueman_aura_behavior_from_artifacts, build_hueman_aura_triad_from_artifacts,
    build_hueman_boundary_from_artifacts, build_hueman_fourway_from_artifacts,
    build_hueman_motion_map_from_artifacts, build_hueman_start_choices_from_artifacts,
    hueman_aura_behavior_artifact_path, hueman_aura_triad_artifact_path,
    hueman_boundary_artifact_path, hueman_fourway_artifact_path,
    hueman_motion_map_artifact_path, hueman_start_choices_artifact_path,
};
use hollow_grove::{read_text_artifact, write_text_artifact};

fn run_hueman_at(root: &Path) -> io::Result<[PathBuf; 6]> {
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

    Ok([
        boundary_path,
        motion_map_path,
        fourway_path,
        aura_triad_path,
        start_choices_path,
        aura_behavior_path,
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
        CURRENT_SYNTHESIS_OPERATIONAL_ARTIFACT_PATH, CURRENT_SYNTHESIS_TOPOLOGY_ARTIFACT_PATH,
        build_hueman_aura_behavior_from_artifacts, build_hueman_aura_triad_from_artifacts,
        build_hueman_boundary_from_artifacts, build_hueman_fourway_from_artifacts,
        build_hueman_motion_map_from_artifacts, build_hueman_start_choices_from_artifacts,
        hueman_aura_behavior_artifact_path, hueman_aura_triad_artifact_path,
        hueman_boundary_artifact_path, hueman_fourway_artifact_path,
        hueman_motion_map_artifact_path, hueman_start_choices_artifact_path,
    };
    use hollow_grove::{read_text_artifact, write_text_artifact};

    use super::run_hueman_at;

    #[test]
    fn hueman_runner_regenerates_boundary_motion_map_fourway_aura_triad_start_choices_and_aura_behavior_in_order() {
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

        let [
            boundary_path,
            motion_map_path,
            fourway_path,
            aura_triad_path,
            start_choices_path,
            aura_behavior_path,
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

        fs::remove_file(&artifact_root.join(CURRENT_SYNTHESIS_BASE_ARTIFACT_PATH))
            .expect("current synthesis base fixture should be removable");
        fs::remove_file(&artifact_root.join(CURRENT_SYNTHESIS_ACTIVATION_GATE_ARTIFACT_PATH))
            .expect("current synthesis activation gate fixture should be removable");
        fs::remove_file(&artifact_root.join(CURRENT_SYNTHESIS_OPERATIONAL_ARTIFACT_PATH))
            .expect("current synthesis operational fixture should be removable");
        fs::remove_file(&artifact_root.join(CURRENT_SYNTHESIS_TOPOLOGY_ARTIFACT_PATH))
            .expect("current synthesis topology fixture should be removable");
        fs::remove_file(&boundary_path).expect("hueman boundary artifact should be removable");
        fs::remove_file(&motion_map_path).expect("hueman motion map artifact should be removable");
        fs::remove_file(&fourway_path).expect("hueman fourway artifact should be removable");
        fs::remove_file(&aura_triad_path).expect("hueman aura triad artifact should be removable");
        fs::remove_file(&start_choices_path)
            .expect("hueman start choices artifact should be removable");
        fs::remove_file(&aura_behavior_path)
            .expect("hueman aura behavior artifact should be removable");
        fs::remove_dir_all(artifact_root.join("artifacts"))
            .expect("artifact fixture directory should be removable");
        fs::remove_dir(&artifact_root).expect("artifact root should be removable");
    }
}
