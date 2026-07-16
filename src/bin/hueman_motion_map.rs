use std::io;
use std::path::Path;

use hollow_grove::hueman_support::{
    CURRENT_SYNTHESIS_OPERATIONAL_ARTIFACT_PATH, HUEMAN_BOUNDARY_ARTIFACT_PATH,
    build_hueman_motion_map_from_artifacts, hueman_motion_map_artifact_path,
};
use hollow_grove::{read_text_artifact, write_text_artifact};

fn main() -> io::Result<()> {
    let hueman_boundary = read_text_artifact(Path::new(HUEMAN_BOUNDARY_ARTIFACT_PATH))?;
    let current_synthesis_operational =
        read_text_artifact(Path::new(CURRENT_SYNTHESIS_OPERATIONAL_ARTIFACT_PATH))?;
    let hueman_motion_map =
        build_hueman_motion_map_from_artifacts(&hueman_boundary, &current_synthesis_operational);
    let artifact_path = hueman_motion_map_artifact_path();

    write_text_artifact(&artifact_path, &hueman_motion_map)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use hollow_grove::hueman_support::build_hueman_motion_map_from_artifacts;
    use hollow_grove::write_text_artifact;

    #[test]
    fn hueman_motion_map_reads_existing_artifacts() {
        let output = build_hueman_motion_map_from_artifacts("boundary", "ops");
        assert!(output.starts_with("# Hueman Motion Map"));
        assert!(output.contains("the Hueman avatar moves through the field"));
        assert!(output.contains("Current Synthesis operational bytes: 3."));
    }

    #[test]
    fn hueman_motion_map_writes_a_deterministic_file() {
        let hueman_motion_map = build_hueman_motion_map_from_artifacts("boundary", "ops");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir = std::env::temp_dir().join(format!("hueman-motion-map-{nonce}"));
        let artifact_path = artifact_dir.join("hueman_motion_map.md");

        write_text_artifact(&artifact_path, &hueman_motion_map)
            .expect("hueman motion map artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path).expect("hueman motion map artifact should read"),
            hueman_motion_map
        );

        fs::remove_file(&artifact_path).expect("hueman motion map artifact should be removable");
        fs::remove_dir(&artifact_dir).expect("hueman motion map directory should be removable");
    }
}
