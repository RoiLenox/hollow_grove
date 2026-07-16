use std::io;
use std::path::Path;

use hollow_grove::hueman_support::{
    HUEMAN_BOUNDARY_ARTIFACT_PATH, HUEMAN_MOTION_MAP_ARTIFACT_PATH,
    build_hueman_fourway_from_artifacts, hueman_fourway_artifact_path,
};
use hollow_grove::{read_text_artifact, write_text_artifact};

fn main() -> io::Result<()> {
    let hueman_boundary = read_text_artifact(Path::new(HUEMAN_BOUNDARY_ARTIFACT_PATH))?;
    let hueman_motion_map = read_text_artifact(Path::new(HUEMAN_MOTION_MAP_ARTIFACT_PATH))?;
    let hueman_fourway = build_hueman_fourway_from_artifacts(&hueman_boundary, &hueman_motion_map);
    let artifact_path = hueman_fourway_artifact_path();

    write_text_artifact(&artifact_path, &hueman_fourway)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use hollow_grove::hueman_support::build_hueman_fourway_from_artifacts;
    use hollow_grove::write_text_artifact;

    #[test]
    fn hueman_fourway_reads_existing_artifacts() {
        let output = build_hueman_fourway_from_artifacts("boundary", "motion");
        assert!(output.starts_with("# Hueman Fourway"));
        assert!(output.contains("West = Flynt"));
        assert!(output.contains("`sprite` originates in Glaushouse."));
        assert!(output.contains("Hueman motion map bytes: 6."));
    }

    #[test]
    fn hueman_fourway_writes_a_deterministic_file() {
        let hueman_fourway = build_hueman_fourway_from_artifacts("boundary", "motion");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir = std::env::temp_dir().join(format!("hueman-fourway-{nonce}"));
        let artifact_path = artifact_dir.join("hueman_fourway.md");

        write_text_artifact(&artifact_path, &hueman_fourway)
            .expect("hueman fourway artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path).expect("hueman fourway artifact should read"),
            hueman_fourway
        );

        fs::remove_file(&artifact_path).expect("hueman fourway artifact should be removable");
        fs::remove_dir(&artifact_dir).expect("hueman fourway directory should be removable");
    }
}
