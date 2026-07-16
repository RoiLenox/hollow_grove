use std::io;
use std::path::Path;

use hollow_grove::hueman_support::{
    HUEMAN_FOURWAY_ARTIFACT_PATH, HUEMAN_LINK_PHYSICS_ARTIFACT_PATH,
    build_hueman_inverse_circle_from_artifacts, hueman_inverse_circle_artifact_path,
};
use hollow_grove::{read_text_artifact, write_text_artifact};

fn main() -> io::Result<()> {
    let hueman_fourway = read_text_artifact(Path::new(HUEMAN_FOURWAY_ARTIFACT_PATH))?;
    let hueman_link_physics = read_text_artifact(Path::new(HUEMAN_LINK_PHYSICS_ARTIFACT_PATH))?;
    let hueman_inverse_circle =
        build_hueman_inverse_circle_from_artifacts(&hueman_fourway, &hueman_link_physics);
    let artifact_path = hueman_inverse_circle_artifact_path();

    write_text_artifact(&artifact_path, &hueman_inverse_circle)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use hollow_grove::hueman_support::build_hueman_inverse_circle_from_artifacts;
    use hollow_grove::write_text_artifact;

    #[test]
    fn hueman_inverse_circle_reads_existing_artifacts() {
        let output = build_hueman_inverse_circle_from_artifacts("fourway", "physics");
        assert!(output.starts_with("# Hueman Inverse Circle"));
        assert!(output.contains("Merman"));
        assert!(output.contains("Mnt. Aura"));
        assert!(output.contains("Hueman Link Physics bytes: 7."));
    }

    #[test]
    fn hueman_inverse_circle_writes_a_deterministic_file() {
        let hueman_inverse_circle =
            build_hueman_inverse_circle_from_artifacts("fourway", "physics");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir = std::env::temp_dir().join(format!("hueman-inverse-circle-{nonce}"));
        let artifact_path = artifact_dir.join("hueman_inverse_circle.md");

        write_text_artifact(&artifact_path, &hueman_inverse_circle)
            .expect("hueman inverse circle artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path).expect("hueman inverse circle artifact should read"),
            hueman_inverse_circle
        );

        fs::remove_file(&artifact_path)
            .expect("hueman inverse circle artifact should be removable");
        fs::remove_dir(&artifact_dir).expect("hueman inverse circle directory should be removable");
    }
}
