use std::io;
use std::path::Path;

use hollow_grove::hueman_support::{
    build_hueman_scene_intent_from_artifacts, hueman_scene_intent_artifact_path,
    HUEMAN_LINK_PHYSICS_ARTIFACT_PATH, HUEMAN_SCENE_PRESENCE_ARTIFACT_PATH,
};
use hollow_grove::{read_text_artifact, write_text_artifact};

fn main() -> io::Result<()> {
    let hueman_scene_presence = read_text_artifact(Path::new(HUEMAN_SCENE_PRESENCE_ARTIFACT_PATH))?;
    let hueman_link_physics = read_text_artifact(Path::new(HUEMAN_LINK_PHYSICS_ARTIFACT_PATH))?;
    let hueman_scene_intent =
        build_hueman_scene_intent_from_artifacts(&hueman_scene_presence, &hueman_link_physics);
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
        assert_eq!(
            build_hueman_scene_intent_from_artifacts("presence", "physics"),
            "# Hueman Scene Intent\n\n\
             ## Structural Rule\n\n\
             Each scene presence carries a dominant descriptive intent before any encounter or dialogue system exists.\n\n\
             ## Intent Sets\n\n\
             ### Seam Market\n\n\
             - wants exchange\n\
             - wants rumor flow\n\
             - wants salvage circulation\n\
             - wants temporary trust without permanence\n\n\
             ### Threshold Weather\n\n\
             - wants warning\n\
             - wants drift\n\
             - wants exposure of unstable boundaries\n\
             - wants movement onward rather than settlement\n\n\
             ### Pressure Shelter\n\n\
             - wants concealment\n\
             - wants storage and continuity\n\
             - wants guarded warmth\n\
             - wants selective admission\n\n\
             ### Split Trace\n\n\
             - wants witness without resolution\n\
             - wants ambiguity to remain active\n\
             - wants multiple readings to coexist\n\
             - wants the route to stay half-open and half-hidden\n\n\
             ## Bias Reading\n\n\
             - `current` bias intensifies exchange, storage, and guarded continuity\n\
             - `aura` bias intensifies warning, drift, shimmer, and ambiguity\n\
             - mixed bias keeps intent unstable and scene-readable from multiple angles\n\n\
             ## Status\n\n\
             - scene intent is descriptive-only for now\n\
             - no AI, NPC, or quest logic is active\n\
             - scene presence remains the upstream occupancy layer\n\
             - link physics remains the upstream bias layer\n\
             - no feedback into Current Synthesis\n\
             - no feedback into Hollow Grove\n\n\
             ## Artifact Inputs\n\n\
             Hueman Scene Presence bytes: 8.\n\
             Hueman Link Physics bytes: 7.\n\n\
             ## Boundary Reminder\n\n\
             Scene intent says what a scene is trying to do atmospherically. It does not create tasks, dialogue trees, or procedural outcomes.\n"
        );
    }

    #[test]
    fn hueman_scene_intent_writes_a_deterministic_file() {
        let hueman_scene_intent = build_hueman_scene_intent_from_artifacts("presence", "physics");
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
