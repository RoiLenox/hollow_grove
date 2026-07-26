use std::io;
use std::path::Path;

use hollow_grove::hueman_support::{
    HUEMAN_FOURWAY_ARTIFACT_PATH, HUEMAN_START_CHOICES_ARTIFACT_PATH,
    build_hueman_flynt_constitution_from_artifacts, hueman_flynt_constitution_artifact_path,
};
use hollow_grove::{read_text_artifact, write_text_artifact};

fn main() -> io::Result<()> {
    let hueman_start_choices = read_text_artifact(Path::new(HUEMAN_START_CHOICES_ARTIFACT_PATH))?;
    let hueman_fourway = read_text_artifact(Path::new(HUEMAN_FOURWAY_ARTIFACT_PATH))?;
    let hueman_flynt_constitution =
        build_hueman_flynt_constitution_from_artifacts(&hueman_start_choices, &hueman_fourway);
    let artifact_path = hueman_flynt_constitution_artifact_path();

    write_text_artifact(&artifact_path, &hueman_flynt_constitution)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use hollow_grove::hueman_support::build_hueman_flynt_constitution_from_artifacts;
    use hollow_grove::write_text_artifact;

    #[test]
    fn hueman_flynt_constitution_reads_existing_artifacts() {
        let output = build_hueman_flynt_constitution_from_artifacts("start", "fourway");
        assert!(output.starts_with("# Hueman Flynt Constitution"));
        assert!(output.contains("Tross is the sovereign executive of Flynt"));
        assert!(output.contains("There is exactly one constitutional Chimera"));
        assert!(
            output.contains("Manticorp is a distinct maintained Synthesis Form beyond Chimera")
        );
        assert!(output.contains("The Mystery Man and Mr. X are aliases or operational identities"));
        assert!(output.contains("The Gallows is Flynt's underground Yakuza/mafia-like body"));
        assert!(output.contains("We Fairy Men is a coalition of distinct Basin traditions"));
        assert!(output.contains("Bro White and the 7 Brothas"));
        assert!(output.contains("Cinderellaman and His Midnight Crew"));
        assert!(output.contains("The Beauty and His Beasts"));
        assert!(output.contains("The Gallowry is the hidden headquarters"));
        assert!(output.contains("## Regional Goods"));
        assert!(output.contains("Flyntian Dagger = Flynt Opal"));
        assert!(output.contains("Hueman Fourway bytes: 7."));
    }

    #[test]
    fn hueman_flynt_constitution_writes_a_deterministic_file() {
        let hueman_flynt_constitution =
            build_hueman_flynt_constitution_from_artifacts("start", "fourway");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir = std::env::temp_dir().join(format!("hueman-flynt-constitution-{nonce}"));
        let artifact_path = artifact_dir.join("hueman_flynt_constitution.md");

        write_text_artifact(&artifact_path, &hueman_flynt_constitution)
            .expect("Hueman Flynt constitution artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path)
                .expect("Hueman Flynt constitution artifact should read"),
            hueman_flynt_constitution
        );

        fs::remove_file(&artifact_path)
            .expect("Hueman Flynt constitution artifact should be removable");
        fs::remove_dir(&artifact_dir)
            .expect("Hueman Flynt constitution directory should be removable");
    }
}
