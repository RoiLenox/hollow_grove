use std::io;
use std::path::Path;

use hollow_grove::hueman_support::{
    CURRENT_SYNTHESIS_COLLISION_RELAY_ARTIFACT_PATH, HUEMAN_AURA_BEHAVIOR_ARTIFACT_PATH,
    HUEMAN_START_PATHS_ARTIFACT_PATH, build_hueman_path_crossovers_from_artifacts,
    hueman_path_crossovers_artifact_path,
};
use hollow_grove::{read_text_artifact, write_text_artifact};

fn main() -> io::Result<()> {
    let hueman_start_paths = read_text_artifact(Path::new(HUEMAN_START_PATHS_ARTIFACT_PATH))?;
    let hueman_aura_behavior = read_text_artifact(Path::new(HUEMAN_AURA_BEHAVIOR_ARTIFACT_PATH))?;
    let current_synthesis_collision_relay =
        read_text_artifact(Path::new(CURRENT_SYNTHESIS_COLLISION_RELAY_ARTIFACT_PATH))?;
    let hueman_path_crossovers = build_hueman_path_crossovers_from_artifacts(
        &hueman_start_paths,
        &hueman_aura_behavior,
        &current_synthesis_collision_relay,
    );
    let artifact_path = hueman_path_crossovers_artifact_path();

    write_text_artifact(&artifact_path, &hueman_path_crossovers)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use hollow_grove::hueman_support::build_hueman_path_crossovers_from_artifacts;
    use hollow_grove::write_text_artifact;

    #[test]
    fn hueman_path_crossovers_reads_existing_artifacts() {
        let output = build_hueman_path_crossovers_from_artifacts("paths", "aura", "relay");
        assert!(output.contains("## Relay Junction"));
        assert!(output.contains("Current Synthesis collision relay bytes: 5."));
    }

    #[test]
    fn hueman_path_crossovers_writes_a_deterministic_file() {
        let hueman_path_crossovers =
            build_hueman_path_crossovers_from_artifacts("paths", "aura", "relay");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir = std::env::temp_dir().join(format!("hueman-path-crossovers-{nonce}"));
        let artifact_path = artifact_dir.join("hueman_path_crossovers.md");

        write_text_artifact(&artifact_path, &hueman_path_crossovers)
            .expect("hueman path crossovers artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path)
                .expect("hueman path crossovers artifact should read"),
            hueman_path_crossovers
        );

        fs::remove_file(&artifact_path)
            .expect("hueman path crossovers artifact should be removable");
        fs::remove_dir(&artifact_dir)
            .expect("hueman path crossovers directory should be removable");
    }
}
