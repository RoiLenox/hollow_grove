use std::io;
use std::path::Path;

use hollow_grove::hueman_support::{
    CURRENT_SYNTHESIS_COLLISION_RELAY_ARTIFACT_PATH, HUEMAN_ARCHETYPE_LENS_ARTIFACT_PATH,
    HUEMAN_CROSSOVER_SCENES_ARTIFACT_PATH, HUEMAN_GLAUSHOUSE_ROLES_ARTIFACT_PATH,
    HUEMAN_INVERSE_CIRCLE_ARTIFACT_PATH, HUEMAN_SANDMANOR_ROLES_ARTIFACT_PATH,
    HUEMAN_STONEBEND_ROLES_ARTIFACT_PATH, HUEMAN_TROSS_HELPERS_ARTIFACT_PATH,
    build_hueman_scene_presence_from_artifacts, hueman_scene_presence_artifact_path,
};
use hollow_grove::{read_text_artifact, write_text_artifact};

fn main() -> io::Result<()> {
    let hueman_crossover_scenes =
        read_text_artifact(Path::new(HUEMAN_CROSSOVER_SCENES_ARTIFACT_PATH))?;
    let hueman_archetype_lens = read_text_artifact(Path::new(HUEMAN_ARCHETYPE_LENS_ARTIFACT_PATH))?;
    let hueman_stonebend_roles =
        read_text_artifact(Path::new(HUEMAN_STONEBEND_ROLES_ARTIFACT_PATH))?;
    let hueman_tross_helpers = read_text_artifact(Path::new(HUEMAN_TROSS_HELPERS_ARTIFACT_PATH))?;
    let hueman_glaushouse_roles =
        read_text_artifact(Path::new(HUEMAN_GLAUSHOUSE_ROLES_ARTIFACT_PATH))?;
    let hueman_sandmanor_roles =
        read_text_artifact(Path::new(HUEMAN_SANDMANOR_ROLES_ARTIFACT_PATH))?;
    let hueman_inverse_circle = read_text_artifact(Path::new(HUEMAN_INVERSE_CIRCLE_ARTIFACT_PATH))?;
    let current_synthesis_collision_relay =
        read_text_artifact(Path::new(CURRENT_SYNTHESIS_COLLISION_RELAY_ARTIFACT_PATH))?;
    let hueman_scene_presence = build_hueman_scene_presence_from_artifacts(
        &hueman_crossover_scenes,
        &hueman_archetype_lens,
        &hueman_stonebend_roles,
        &hueman_tross_helpers,
        &hueman_glaushouse_roles,
        &hueman_sandmanor_roles,
        &hueman_inverse_circle,
        &current_synthesis_collision_relay,
    );
    let artifact_path = hueman_scene_presence_artifact_path();

    write_text_artifact(&artifact_path, &hueman_scene_presence)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use hollow_grove::hueman_support::build_hueman_scene_presence_from_artifacts;
    use hollow_grove::write_text_artifact;

    #[test]
    fn hueman_scene_presence_reads_existing_artifacts() {
        let output = build_hueman_scene_presence_from_artifacts(
            "scene",
            "lens",
            "roles",
            "tross",
            "glaushouse",
            "sandmanor",
            "inverse",
            "relay",
        );
        assert!(output.contains("## Relay Packet Presence"));
        assert!(output.contains("boardwalk-casino hunting capital"));
        assert!(output.contains("Aura Beach: the Minoan-facing coastal court"));
        assert!(output.contains("Current Synthesis collision relay bytes: 5."));
    }

    #[test]
    fn hueman_scene_presence_writes_a_deterministic_file() {
        let hueman_scene_presence = build_hueman_scene_presence_from_artifacts(
            "scene",
            "lens",
            "roles",
            "tross",
            "glaushouse",
            "sandmanor",
            "inverse",
            "relay",
        );
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir = std::env::temp_dir().join(format!("hueman-scene-presence-{nonce}"));
        let artifact_path = artifact_dir.join("hueman_scene_presence.md");

        write_text_artifact(&artifact_path, &hueman_scene_presence)
            .expect("hueman scene presence artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path).expect("hueman scene presence artifact should read"),
            hueman_scene_presence
        );

        fs::remove_file(&artifact_path)
            .expect("hueman scene presence artifact should be removable");
        fs::remove_dir(&artifact_dir).expect("hueman scene presence directory should be removable");
    }
}
