use std::io;
use std::path::Path;

use hollow_grove::hueman_support::{
    HUEMAN_AURA_BEHAVIOR_ARTIFACT_PATH, HUEMAN_SANDMANOR_ROLES_ARTIFACT_PATH,
    HUEMAN_START_CHOICES_ARTIFACT_PATH, HUEMAN_STONEBEND_ROLES_ARTIFACT_PATH,
    build_hueman_archetype_lens_from_artifacts, hueman_archetype_lens_artifact_path,
};
use hollow_grove::{read_text_artifact, write_text_artifact};

fn main() -> io::Result<()> {
    let hueman_start_choices = read_text_artifact(Path::new(HUEMAN_START_CHOICES_ARTIFACT_PATH))?;
    let hueman_aura_behavior = read_text_artifact(Path::new(HUEMAN_AURA_BEHAVIOR_ARTIFACT_PATH))?;
    let hueman_stonebend_roles =
        read_text_artifact(Path::new(HUEMAN_STONEBEND_ROLES_ARTIFACT_PATH))?;
    let hueman_sandmanor_roles =
        read_text_artifact(Path::new(HUEMAN_SANDMANOR_ROLES_ARTIFACT_PATH))?;
    let hueman_archetype_lens = build_hueman_archetype_lens_from_artifacts(
        &hueman_start_choices,
        &hueman_aura_behavior,
        &hueman_stonebend_roles,
        &hueman_sandmanor_roles,
    );
    let artifact_path = hueman_archetype_lens_artifact_path();

    write_text_artifact(&artifact_path, &hueman_archetype_lens)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use hollow_grove::hueman_support::build_hueman_archetype_lens_from_artifacts;
    use hollow_grove::write_text_artifact;

    #[test]
    fn hueman_archetype_lens_reads_existing_artifacts() {
        let output =
            build_hueman_archetype_lens_from_artifacts("start", "aura", "roles", "sandmanor");
        assert!(output.starts_with("# Hueman Archetype Lens"));
        assert!(output.contains("confirmed origin path"));
        assert!(output.contains("### `sprite`"));
        assert!(output.contains("does not make forms into hereditary races"));
    }

    #[test]
    fn hueman_archetype_lens_writes_a_deterministic_file() {
        let hueman_archetype_lens =
            build_hueman_archetype_lens_from_artifacts("start", "aura", "roles", "sandmanor");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir = std::env::temp_dir().join(format!("hueman-archetype-lens-{nonce}"));
        let artifact_path = artifact_dir.join("hueman_archetype_lens.md");

        write_text_artifact(&artifact_path, &hueman_archetype_lens)
            .expect("hueman archetype lens artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path).expect("hueman archetype lens artifact should read"),
            hueman_archetype_lens
        );

        fs::remove_file(&artifact_path)
            .expect("hueman archetype lens artifact should be removable");
        fs::remove_dir(&artifact_dir).expect("hueman archetype lens directory should be removable");
    }
}
