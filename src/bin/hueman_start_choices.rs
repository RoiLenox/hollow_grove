use std::io;
use std::path::Path;

use hollow_grove::hueman_support::{
    HUEMAN_AURA_TRIAD_ARTIFACT_PATH, HUEMAN_FOURWAY_ARTIFACT_PATH,
    build_hueman_start_choices_from_artifacts, hueman_start_choices_artifact_path,
};
use hollow_grove::{read_text_artifact, write_text_artifact};

fn main() -> io::Result<()> {
    let hueman_fourway = read_text_artifact(Path::new(HUEMAN_FOURWAY_ARTIFACT_PATH))?;
    let hueman_aura_triad = read_text_artifact(Path::new(HUEMAN_AURA_TRIAD_ARTIFACT_PATH))?;
    let hueman_start_choices =
        build_hueman_start_choices_from_artifacts(&hueman_fourway, &hueman_aura_triad);
    let artifact_path = hueman_start_choices_artifact_path();

    write_text_artifact(&artifact_path, &hueman_start_choices)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use hollow_grove::hueman_support::build_hueman_start_choices_from_artifacts;
    use hollow_grove::write_text_artifact;

    #[test]
    fn hueman_start_choices_reads_existing_artifacts() {
        let output = build_hueman_start_choices_from_artifacts("fourway", "triad");
        assert!(output.starts_with("# Hueman Start Choices"));
        assert!(output.contains("the player begins as Hueman near Aura Ridge"));
        assert!(output.contains("`goblin` originates in Stonebend"));
        assert!(output.contains("Hueman AuraTriad bytes: 5."));
    }

    #[test]
    fn hueman_start_choices_writes_a_deterministic_file() {
        let hueman_start_choices = build_hueman_start_choices_from_artifacts("fourway", "triad");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir = std::env::temp_dir().join(format!("hueman-start-choices-{nonce}"));
        let artifact_path = artifact_dir.join("hueman_start_choices.md");

        write_text_artifact(&artifact_path, &hueman_start_choices)
            .expect("hueman start choices artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path).expect("hueman start choices artifact should read"),
            hueman_start_choices
        );

        fs::remove_file(&artifact_path).expect("hueman start choices artifact should be removable");
        fs::remove_dir(&artifact_dir).expect("hueman start choices directory should be removable");
    }
}
