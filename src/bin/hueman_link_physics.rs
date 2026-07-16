use std::io;
use std::path::Path;

use hollow_grove::hueman_support::{
    CURRENT_SYNTHESIS_COLLISION_RELAY_ARTIFACT_PATH, CURRENT_SYNTHESIS_SEQUENCE_ARTIFACT_PATH,
    HUEMAN_PATH_CROSSOVERS_ARTIFACT_PATH, build_hueman_link_physics_from_artifacts,
    hueman_link_physics_artifact_path,
};
use hollow_grove::{read_text_artifact, write_text_artifact};

fn main() -> io::Result<()> {
    let current_synthesis_sequence =
        read_text_artifact(Path::new(CURRENT_SYNTHESIS_SEQUENCE_ARTIFACT_PATH))?;
    let hueman_path_crossovers =
        read_text_artifact(Path::new(HUEMAN_PATH_CROSSOVERS_ARTIFACT_PATH))?;
    let current_synthesis_collision_relay =
        read_text_artifact(Path::new(CURRENT_SYNTHESIS_COLLISION_RELAY_ARTIFACT_PATH))?;
    let hueman_link_physics = build_hueman_link_physics_from_artifacts(
        &current_synthesis_sequence,
        &hueman_path_crossovers,
        &current_synthesis_collision_relay,
    );
    let artifact_path = hueman_link_physics_artifact_path();

    write_text_artifact(&artifact_path, &hueman_link_physics)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use hollow_grove::hueman_support::build_hueman_link_physics_from_artifacts;
    use hollow_grove::write_text_artifact;

    #[test]
    fn hueman_link_physics_reads_existing_artifacts() {
        let output = build_hueman_link_physics_from_artifacts("sequence", "cross", "relay");
        assert!(output.contains("## Relay Packet Reading"));
        assert!(output.contains("Current Synthesis collision relay bytes: 5."));
    }

    #[test]
    fn hueman_link_physics_writes_a_deterministic_file() {
        let hueman_link_physics =
            build_hueman_link_physics_from_artifacts("sequence", "cross", "relay");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir = std::env::temp_dir().join(format!("hueman-link-physics-{nonce}"));
        let artifact_path = artifact_dir.join("hueman_link_physics.md");

        write_text_artifact(&artifact_path, &hueman_link_physics)
            .expect("hueman link physics artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path).expect("hueman link physics artifact should read"),
            hueman_link_physics
        );

        fs::remove_file(&artifact_path).expect("hueman link physics artifact should be removable");
        fs::remove_dir(&artifact_dir).expect("hueman link physics directory should be removable");
    }
}
