use std::io;
use std::path::{Path, PathBuf};

use hollow_grove::hueman_support::{
    CURRENT_SYNTHESIS_ACTIVATION_GATE_ARTIFACT_PATH, CURRENT_SYNTHESIS_BASE_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_OPERATIONAL_ARTIFACT_PATH, build_hueman_boundary_from_artifacts,
    build_hueman_fourway_from_artifacts, build_hueman_motion_map_from_artifacts,
    build_hueman_start_choices_from_artifacts, hueman_boundary_artifact_path,
    hueman_fourway_artifact_path, hueman_motion_map_artifact_path, hueman_start_choices_artifact_path,
};
use hollow_grove::{read_text_artifact, write_text_artifact};

fn run_hueman_at(root: &Path) -> io::Result<[PathBuf; 4]> {
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

    let hueman_start_choices =
        build_hueman_start_choices_from_artifacts(&hueman_fourway, &hueman_motion_map);
    let start_choices_path = root.join(hueman_start_choices_artifact_path());
    write_text_artifact(&start_choices_path, &hueman_start_choices)?;

    Ok([boundary_path, motion_map_path, fourway_path, start_choices_path])
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
        CURRENT_SYNTHESIS_OPERATIONAL_ARTIFACT_PATH, build_hueman_boundary_from_artifacts,
        build_hueman_fourway_from_artifacts, build_hueman_motion_map_from_artifacts,
        build_hueman_start_choices_from_artifacts, hueman_boundary_artifact_path,
        hueman_fourway_artifact_path, hueman_motion_map_artifact_path, hueman_start_choices_artifact_path,
    };
    use hollow_grove::{read_text_artifact, write_text_artifact};

    use super::run_hueman_at;

    #[test]
    fn hueman_runner_regenerates_boundary_motion_map_fourway_and_start_choices_in_order() {
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

        let [boundary_path, motion_map_path, fourway_path, start_choices_path] =
            run_hueman_at(&artifact_root).expect("hueman should run");
        let hueman_boundary = build_hueman_boundary_from_artifacts("base", "gate");
        let hueman_motion_map = build_hueman_motion_map_from_artifacts(&hueman_boundary, "ops");
        let hueman_fourway =
            build_hueman_fourway_from_artifacts(&hueman_boundary, &hueman_motion_map);
        let hueman_start_choices =
            build_hueman_start_choices_from_artifacts(&hueman_fourway, &hueman_motion_map);

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
            start_choices_path,
            artifact_root.join(hueman_start_choices_artifact_path())
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
            read_text_artifact(&start_choices_path)
                .expect("hueman start choices artifact should read"),
            hueman_start_choices
        );

        fs::remove_file(&artifact_root.join(CURRENT_SYNTHESIS_BASE_ARTIFACT_PATH))
            .expect("current synthesis base fixture should be removable");
        fs::remove_file(&artifact_root.join(CURRENT_SYNTHESIS_ACTIVATION_GATE_ARTIFACT_PATH))
            .expect("current synthesis activation gate fixture should be removable");
        fs::remove_file(&artifact_root.join(CURRENT_SYNTHESIS_OPERATIONAL_ARTIFACT_PATH))
            .expect("current synthesis operational fixture should be removable");
        fs::remove_file(&boundary_path).expect("hueman boundary artifact should be removable");
        fs::remove_file(&motion_map_path).expect("hueman motion map artifact should be removable");
        fs::remove_file(&fourway_path).expect("hueman fourway artifact should be removable");
        fs::remove_file(&start_choices_path)
            .expect("hueman start choices artifact should be removable");
        fs::remove_dir_all(artifact_root.join("artifacts"))
            .expect("artifact fixture directory should be removable");
        fs::remove_dir(&artifact_root).expect("artifact root should be removable");
    }
}
