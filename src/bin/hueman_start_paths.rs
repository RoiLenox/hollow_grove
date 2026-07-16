use std::io;
use std::path::Path;

use hollow_grove::hueman_support::{
    HUEMAN_ARCHETYPE_LENS_ARTIFACT_PATH, HUEMAN_START_CHOICES_ARTIFACT_PATH,
    build_hueman_start_paths_from_artifacts, hueman_start_paths_artifact_path,
};
use hollow_grove::{read_text_artifact, write_text_artifact};

fn main() -> io::Result<()> {
    let hueman_start_choices = read_text_artifact(Path::new(HUEMAN_START_CHOICES_ARTIFACT_PATH))?;
    let hueman_archetype_lens = read_text_artifact(Path::new(HUEMAN_ARCHETYPE_LENS_ARTIFACT_PATH))?;
    let hueman_start_paths =
        build_hueman_start_paths_from_artifacts(&hueman_start_choices, &hueman_archetype_lens);
    let artifact_path = hueman_start_paths_artifact_path();

    write_text_artifact(&artifact_path, &hueman_start_paths)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use hollow_grove::hueman_support::build_hueman_start_paths_from_artifacts;
    use hollow_grove::write_text_artifact;

    #[test]
    fn hueman_start_paths_reads_existing_artifacts() {
        let output = build_hueman_start_paths_from_artifacts("start", "lens");
        assert!(output.starts_with("# Hueman Start Paths"));
        assert!(output.contains("player still begins as Hueman near Aura Ridge"));
        assert!(
            output.contains("Sandmanor-facing approach = Aura Beach -> Aura Basin -> Aura Fields")
        );
        assert!(output.contains("Hueman Archetype Lens bytes: 4."));
    }

    #[test]
    fn hueman_start_paths_writes_a_deterministic_file() {
        let hueman_start_paths = build_hueman_start_paths_from_artifacts("start", "lens");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir = std::env::temp_dir().join(format!("hueman-start-paths-{nonce}"));
        let artifact_path = artifact_dir.join("hueman_start_paths.md");

        write_text_artifact(&artifact_path, &hueman_start_paths)
            .expect("hueman start paths artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path).expect("hueman start paths artifact should read"),
            hueman_start_paths
        );

        fs::remove_file(&artifact_path).expect("hueman start paths artifact should be removable");
        fs::remove_dir(&artifact_dir).expect("hueman start paths directory should be removable");
    }
}
