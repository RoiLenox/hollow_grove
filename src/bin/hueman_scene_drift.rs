use std::io;
use std::path::Path;

use hollow_grove::hueman_support::{
    CURRENT_SYNTHESIS_COLLISION_RELAY_ARTIFACT_PATH, HUEMAN_LINK_PHYSICS_ARTIFACT_PATH,
    HUEMAN_SCENE_INTENT_ARTIFACT_PATH, build_hueman_scene_drift_from_artifacts,
    hueman_scene_drift_artifact_path,
};
use hollow_grove::{read_text_artifact, write_text_artifact};

fn main() -> io::Result<()> {
    let hueman_scene_intent = read_text_artifact(Path::new(HUEMAN_SCENE_INTENT_ARTIFACT_PATH))?;
    let hueman_link_physics = read_text_artifact(Path::new(HUEMAN_LINK_PHYSICS_ARTIFACT_PATH))?;
    let current_synthesis_collision_relay =
        read_text_artifact(Path::new(CURRENT_SYNTHESIS_COLLISION_RELAY_ARTIFACT_PATH))?;
    let hueman_scene_drift = build_hueman_scene_drift_from_artifacts(
        &hueman_scene_intent,
        &hueman_link_physics,
        &current_synthesis_collision_relay,
    );
    let artifact_path = hueman_scene_drift_artifact_path();

    write_text_artifact(&artifact_path, &hueman_scene_drift)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use hollow_grove::hueman_support::build_hueman_scene_drift_from_artifacts;
    use hollow_grove::write_text_artifact;

    #[test]
    fn hueman_scene_drift_reads_existing_artifacts() {
        let output = build_hueman_scene_drift_from_artifacts("intent", "physics", "relay");
        assert!(output.contains("slows drift by holding one shared confirmation point"));
        assert!(output.contains("Current Synthesis collision relay bytes: 5."));
    }

    #[test]
    fn hueman_scene_drift_writes_a_deterministic_file() {
        let hueman_scene_drift =
            build_hueman_scene_drift_from_artifacts("intent", "physics", "relay");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir = std::env::temp_dir().join(format!("hueman-scene-drift-{nonce}"));
        let artifact_path = artifact_dir.join("hueman_scene_drift.md");

        write_text_artifact(&artifact_path, &hueman_scene_drift)
            .expect("hueman scene drift artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path).expect("hueman scene drift artifact should read"),
            hueman_scene_drift
        );

        fs::remove_file(&artifact_path).expect("hueman scene drift artifact should be removable");
        fs::remove_dir(&artifact_dir).expect("hueman scene drift directory should be removable");
    }
}
