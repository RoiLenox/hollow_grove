use std::io;
use std::path::Path;

use hollow_grove::hueman_support::{
    CURRENT_SYNTHESIS_COLLISION_RELAY_ARTIFACT_PATH, CURRENT_SYNTHESIS_CONTRACT_ARTIFACT_PATH,
    HUEMAN_GLAUSHOUSE_ROLES_ARTIFACT_PATH, HUEMAN_INVERSE_CIRCLE_ARTIFACT_PATH,
    HUEMAN_LINK_PHYSICS_ARTIFACT_PATH, HUEMAN_SANDMANOR_ROLES_ARTIFACT_PATH,
    HUEMAN_SCENE_PRESENCE_ARTIFACT_PATH, HUEMAN_STONEBEND_ROLES_ARTIFACT_PATH,
    HUEMAN_TROSS_HELPERS_ARTIFACT_PATH, build_hueman_scene_intent_from_artifacts,
    hueman_scene_intent_artifact_path,
};
use hollow_grove::{read_text_artifact, write_text_artifact};

fn main() -> io::Result<()> {
    let hueman_scene_presence = read_text_artifact(Path::new(HUEMAN_SCENE_PRESENCE_ARTIFACT_PATH))?;
    let hueman_link_physics = read_text_artifact(Path::new(HUEMAN_LINK_PHYSICS_ARTIFACT_PATH))?;
    let current_synthesis_collision_relay =
        read_text_artifact(Path::new(CURRENT_SYNTHESIS_COLLISION_RELAY_ARTIFACT_PATH))?;
    let current_synthesis_contract =
        read_text_artifact(Path::new(CURRENT_SYNTHESIS_CONTRACT_ARTIFACT_PATH))?;
    let hueman_stonebend_roles =
        read_text_artifact(Path::new(HUEMAN_STONEBEND_ROLES_ARTIFACT_PATH))?;
    let hueman_tross_helpers = read_text_artifact(Path::new(HUEMAN_TROSS_HELPERS_ARTIFACT_PATH))?;
    let hueman_glaushouse_roles =
        read_text_artifact(Path::new(HUEMAN_GLAUSHOUSE_ROLES_ARTIFACT_PATH))?;
    let hueman_sandmanor_roles =
        read_text_artifact(Path::new(HUEMAN_SANDMANOR_ROLES_ARTIFACT_PATH))?;
    let hueman_inverse_circle = read_text_artifact(Path::new(HUEMAN_INVERSE_CIRCLE_ARTIFACT_PATH))?;
    let hueman_scene_intent = build_hueman_scene_intent_from_artifacts(
        &hueman_scene_presence,
        &hueman_link_physics,
        &current_synthesis_collision_relay,
        &current_synthesis_contract,
        &hueman_stonebend_roles,
        &hueman_tross_helpers,
        &hueman_glaushouse_roles,
        &hueman_sandmanor_roles,
        &hueman_inverse_circle,
    );
    let artifact_path = hueman_scene_intent_artifact_path();

    write_text_artifact(&artifact_path, &hueman_scene_intent)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use hollow_grove::hueman_support::build_hueman_scene_intent_from_artifacts;
    use hollow_grove::write_text_artifact;

    #[test]
    fn hueman_scene_intent_reads_existing_artifacts() {
        let output = build_hueman_scene_intent_from_artifacts(
            "presence",
            "physics",
            "relay",
            "contract",
            "roles",
            "tross",
            "glaushouse",
            "sandmanor",
            "inverse",
        );
        assert!(output.contains("shared confirmation pressure packet"));
        assert!(output.contains("Current Synthesis collision relay bytes: 5."));
    }

    #[test]
    fn hueman_scene_intent_writes_a_deterministic_file() {
        let hueman_scene_intent = build_hueman_scene_intent_from_artifacts(
            "presence",
            "physics",
            "relay",
            "contract",
            "roles",
            "tross",
            "glaushouse",
            "sandmanor",
            "inverse",
        );
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir = std::env::temp_dir().join(format!("hueman-scene-intent-{nonce}"));
        let artifact_path = artifact_dir.join("hueman_scene_intent.md");

        write_text_artifact(&artifact_path, &hueman_scene_intent)
            .expect("hueman scene intent artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path).expect("hueman scene intent artifact should read"),
            hueman_scene_intent
        );

        fs::remove_file(&artifact_path).expect("hueman scene intent artifact should be removable");
        fs::remove_dir(&artifact_dir).expect("hueman scene intent directory should be removable");
    }
}
