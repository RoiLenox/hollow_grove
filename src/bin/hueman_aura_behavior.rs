use std::io;
use std::path::Path;

use hollow_grove::hueman_support::{
    HUEMAN_AURA_TRIAD_ARTIFACT_PATH, HUEMAN_START_CHOICES_ARTIFACT_PATH,
    build_hueman_aura_behavior_from_artifacts, hueman_aura_behavior_artifact_path,
};
use hollow_grove::{read_text_artifact, write_text_artifact};

fn main() -> io::Result<()> {
    let hueman_aura_triad = read_text_artifact(Path::new(HUEMAN_AURA_TRIAD_ARTIFACT_PATH))?;
    let hueman_start_choices = read_text_artifact(Path::new(HUEMAN_START_CHOICES_ARTIFACT_PATH))?;
    let hueman_aura_behavior =
        build_hueman_aura_behavior_from_artifacts(&hueman_aura_triad, &hueman_start_choices);
    let artifact_path = hueman_aura_behavior_artifact_path();

    write_text_artifact(&artifact_path, &hueman_aura_behavior)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use hollow_grove::hueman_support::build_hueman_aura_behavior_from_artifacts;
    use hollow_grove::write_text_artifact;

    #[test]
    fn hueman_aura_behavior_reads_existing_artifacts() {
        assert_eq!(
            build_hueman_aura_behavior_from_artifacts("triad", "start"),
            "# Hueman Aura Behavior\n\n\
             ## Structural Rule\n\n\
             After the start choice is placed on the Fourway, Hueman reads AuraTriad as three descriptive region states.\n\n\
             ## Entry Order\n\n\
             ```text\n\
             Start Choice\n\
             ↓\n\
             Aura Basin\n\
             ↓\n\
             Aura Fields\n\
             ↓\n\
             Aura Beach\n\
             ```\n\n\
             ## Region States\n\n\
             ### Aura Basin\n\n\
             - movement reads as inward and narrowing\n\
             - encounter tone reads as close, muffled, and formative\n\
             - world description favors pressure, shelter, and accumulation\n\n\
             ### Aura Fields\n\n\
             - movement reads as lateral and exposed\n\
             - encounter tone reads as social, visible, and negotiable\n\
             - world description favors weather, distance, and traversal\n\n\
             ### Aura Beach\n\n\
             - movement reads as outward and threshold-facing\n\
             - encounter tone reads as reflective, sparse, and releasing\n\
             - world description favors edge, horizon, and departure\n\n\
             ## Status\n\n\
             - AuraTriad behavior is descriptive-only for now\n\
             - movement pressure is declarative, not simulated\n\
             - encounter tone is declarative, not procedural\n\
             - the Fourway roster remains unchanged\n\
             - species mechanics are not active yet\n\
             - no feedback into Current Synthesis\n\
             - no feedback into Hollow Grove\n\n\
             ## Artifact Inputs\n\n\
             Hueman AuraTriad bytes: 5.\n\
             Hueman Start Choices bytes: 5.\n\n\
             ## Boundary Reminder\n\n\
             Aura behavior is a Hueman-facing reading of the world after start choice. It does not alter Current Synthesis occupancy or Hollow Grove recursion.\n"
        );
    }

    #[test]
    fn hueman_aura_behavior_writes_a_deterministic_file() {
        let hueman_aura_behavior = build_hueman_aura_behavior_from_artifacts("triad", "start");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir = std::env::temp_dir().join(format!("hueman-aura-behavior-{nonce}"));
        let artifact_path = artifact_dir.join("hueman_aura_behavior.md");

        write_text_artifact(&artifact_path, &hueman_aura_behavior)
            .expect("hueman aura behavior artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path).expect("hueman aura behavior artifact should read"),
            hueman_aura_behavior
        );

        fs::remove_file(&artifact_path).expect("hueman aura behavior artifact should be removable");
        fs::remove_dir(&artifact_dir).expect("hueman aura behavior directory should be removable");
    }
}
