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
        let output = build_hueman_aura_behavior_from_artifacts("triad", "start");
        assert!(output.starts_with("# Hueman Aura Behavior"));
        assert!(output.contains("## Player Grammar"));
        assert!(output.contains("Aura Glow asks whether the player is getting closer"));
        assert!(output.contains("Hueman Start Choices bytes: 5."));
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
