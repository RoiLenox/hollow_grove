use std::io;
use std::path::Path;

use hollow_grove::hueman_support::{
    CURRENT_SYNTHESIS_COLLISION_RELAY_ARTIFACT_PATH, HUEMAN_LINK_PHYSICS_ARTIFACT_PATH,
    HUEMAN_PATH_CROSSOVERS_ARTIFACT_PATH, build_hueman_crossover_scenes_from_artifacts,
    hueman_crossover_scenes_artifact_path,
};
use hollow_grove::{read_text_artifact, write_text_artifact};

fn main() -> io::Result<()> {
    let hueman_path_crossovers =
        read_text_artifact(Path::new(HUEMAN_PATH_CROSSOVERS_ARTIFACT_PATH))?;
    let hueman_link_physics = read_text_artifact(Path::new(HUEMAN_LINK_PHYSICS_ARTIFACT_PATH))?;
    let current_synthesis_collision_relay =
        read_text_artifact(Path::new(CURRENT_SYNTHESIS_COLLISION_RELAY_ARTIFACT_PATH))?;
    let hueman_crossover_scenes = build_hueman_crossover_scenes_from_artifacts(
        &hueman_path_crossovers,
        &hueman_link_physics,
        &current_synthesis_collision_relay,
    );
    let artifact_path = hueman_crossover_scenes_artifact_path();

    write_text_artifact(&artifact_path, &hueman_crossover_scenes)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use hollow_grove::hueman_support::build_hueman_crossover_scenes_from_artifacts;
    use hollow_grove::write_text_artifact;

    #[test]
    fn hueman_crossover_scenes_reads_existing_artifacts() {
        let output = build_hueman_crossover_scenes_from_artifacts("cross", "physics", "relay");
        assert!(output.contains("## Relay Scene Use"));
        assert!(output.contains("Current Synthesis collision relay bytes: 5."));
    }

    #[test]
    fn hueman_crossover_scenes_writes_a_deterministic_file() {
        let hueman_crossover_scenes =
            build_hueman_crossover_scenes_from_artifacts("cross", "physics", "relay");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir = std::env::temp_dir().join(format!("hueman-crossover-scenes-{nonce}"));
        let artifact_path = artifact_dir.join("hueman_crossover_scenes.md");

        write_text_artifact(&artifact_path, &hueman_crossover_scenes)
            .expect("hueman crossover scenes artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path)
                .expect("hueman crossover scenes artifact should read"),
            hueman_crossover_scenes
        );

        fs::remove_file(&artifact_path)
            .expect("hueman crossover scenes artifact should be removable");
        fs::remove_dir(&artifact_dir)
            .expect("hueman crossover scenes directory should be removable");
    }
}
