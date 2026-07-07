use std::io;
use std::path::Path;

use hollow_grove::hueman_support::{
    HUEMAN_ARCHETYPE_LENS_ARTIFACT_PATH, HUEMAN_CROSSOVER_SCENES_ARTIFACT_PATH,
    build_hueman_scene_presence_from_artifacts, hueman_scene_presence_artifact_path,
};
use hollow_grove::{read_text_artifact, write_text_artifact};

fn main() -> io::Result<()> {
    let hueman_crossover_scenes =
        read_text_artifact(Path::new(HUEMAN_CROSSOVER_SCENES_ARTIFACT_PATH))?;
    let hueman_archetype_lens =
        read_text_artifact(Path::new(HUEMAN_ARCHETYPE_LENS_ARTIFACT_PATH))?;
    let hueman_scene_presence = build_hueman_scene_presence_from_artifacts(
        &hueman_crossover_scenes,
        &hueman_archetype_lens,
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
        assert_eq!(
            build_hueman_scene_presence_from_artifacts("scene", "lens"),
            "# Hueman Scene Presence\n\n\
             ## Structural Rule\n\n\
             Each crossover scene carries a characteristic kind of presence before any encounter mechanics exist.\n\n\
             ## Presence Sets\n\n\
             ### Seam Market\n\n\
             - rumor carriers\n\
             - salvage brokers\n\
             - porters and exchangers\n\
             - temporary stalls, marks, and signal cloths\n\n\
             ### Threshold Weather\n\n\
             - drifters and lookouts\n\
             - spray forms and kite traces\n\
             - warning markers at unstable edges\n\
             - bright debris that looks almost inhabited\n\n\
             ### Pressure Shelter\n\n\
             - keepers, hoarders, and wardens\n\
             - bundled stores and shielded caches\n\
             - muffled listeners and inward-facing fires\n\
             - stacked signs of prior occupation\n\n\
             ### Split Trace\n\n\
             - echoes, doubles, and uncertain witnesses\n\
             - abandoned signals that still feel recent\n\
             - partial camps, partial routes, partial stories\n\
             - clues that suggest more than one version of the scene\n\n\
             ## Archetype Pull\n\n\
             - `goblin` reads Pressure Shelter as nearest habitation\n\
             - `gremlin` reads Seam Market as the busiest adaptive presence\n\
             - `pixy` reads Threshold Weather as the brightest living atmosphere\n\
             - `sprite` reads Split Trace as the most permeable presence state\n\n\
             ## Status\n\n\
             - scene presence is descriptive-only for now\n\
             - no NPC system or occupancy resolver is active\n\
             - crossover scenes remain the upstream scene map\n\
             - archetype lens remains the upstream interpretive filter\n\
             - no feedback into Current Synthesis\n\
             - no feedback into Hollow Grove\n\n\
             ## Artifact Inputs\n\n\
             Hueman Crossover Scenes bytes: 5.\n\
             Hueman Archetype Lens bytes: 4.\n\n\
             ## Boundary Reminder\n\n\
             Scene presence says what kind of occupant or trace belongs in a scene. It does not create procedural actors, dialogue, or rewards.\n"
        );
    }

    #[test]
    fn hueman_scene_presence_writes_a_deterministic_file() {
        let hueman_scene_presence =
            build_hueman_scene_presence_from_artifacts("scene", "lens");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir = std::env::temp_dir().join(format!("hueman-scene-presence-{nonce}"));
        let artifact_path = artifact_dir.join("hueman_scene_presence.md");

        write_text_artifact(&artifact_path, &hueman_scene_presence)
            .expect("hueman scene presence artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path)
                .expect("hueman scene presence artifact should read"),
            hueman_scene_presence
        );

        fs::remove_file(&artifact_path)
            .expect("hueman scene presence artifact should be removable");
        fs::remove_dir(&artifact_dir)
            .expect("hueman scene presence directory should be removable");
    }
}
