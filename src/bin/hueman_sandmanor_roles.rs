use std::io;
use std::path::Path;

use hollow_grove::hueman_support::{
    HUEMAN_FOURWAY_ARTIFACT_PATH, HUEMAN_START_CHOICES_ARTIFACT_PATH,
    build_hueman_sandmanor_roles_from_artifacts, hueman_sandmanor_roles_artifact_path,
};
use hollow_grove::{read_text_artifact, write_text_artifact};

fn main() -> io::Result<()> {
    let hueman_start_choices = read_text_artifact(Path::new(HUEMAN_START_CHOICES_ARTIFACT_PATH))?;
    let hueman_fourway = read_text_artifact(Path::new(HUEMAN_FOURWAY_ARTIFACT_PATH))?;
    let hueman_sandmanor_roles =
        build_hueman_sandmanor_roles_from_artifacts(&hueman_start_choices, &hueman_fourway);
    let artifact_path = hueman_sandmanor_roles_artifact_path();

    write_text_artifact(&artifact_path, &hueman_sandmanor_roles)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use hollow_grove::hueman_support::build_hueman_sandmanor_roles_from_artifacts;
    use hollow_grove::write_text_artifact;

    #[test]
    fn hueman_sandmanor_roles_reads_existing_artifacts() {
        let output = build_hueman_sandmanor_roles_from_artifacts("start", "fourway");
        assert!(output.starts_with("# Hueman Sandmanor Roles"));
        assert!(output.contains("Pixy is Sandmanor's confirmed Aura-origin path."));
        assert!(output.contains("Minorians are Sandmanor's interior Gnome tradition"));
        assert!(output.contains("Minoans are Sandmanor's exterior Elf tradition"));
        assert!(output.contains("Gnome -> Minotaur is the canonical Aura Field"));
        assert!(output.contains("Elf -> Centaur is the canonical Aura Beach"));
        assert!(output.contains("failure is evidence and remains recorded"));
        assert!(output.contains("Contest of Improvement"));
        assert!(output.contains("## People And Profession Composition"));
        assert!(output.contains("Elf Radiologist = Being Hueman"));
        assert!(output.contains("Gnome Emergency Physician = Being Hueman"));
        assert!(output.contains("The Sandman is the singular highest constitutional office"));
        assert!(output.contains("Fraudulent Design is Sandmanor's signature"));
        assert!(output.contains("Aura Fields and Content Farm"));
        assert!(output.contains("Free Aura Beach -> Southern Coast -> Current Break"));
        assert!(output.contains("Gnome -> Minotaur -> Hecaton"));
        assert!(output.contains("Elf -> Centaur -> Pegasus"));
        assert!(output.contains("the audited crowd measures improvement"));
        assert!(output.contains("called Aegon"));
        assert!(output.contains("Hueman Fourway bytes: 7."));
    }

    #[test]
    fn hueman_sandmanor_roles_writes_a_deterministic_file() {
        let hueman_sandmanor_roles =
            build_hueman_sandmanor_roles_from_artifacts("start", "fourway");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir = std::env::temp_dir().join(format!("hueman-sandmanor-roles-{nonce}"));
        let artifact_path = artifact_dir.join("hueman_sandmanor_roles.md");

        write_text_artifact(&artifact_path, &hueman_sandmanor_roles)
            .expect("hueman sandmanor roles artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path)
                .expect("hueman sandmanor roles artifact should read"),
            hueman_sandmanor_roles
        );

        fs::remove_file(&artifact_path)
            .expect("hueman sandmanor roles artifact should be removable");
        fs::remove_dir(&artifact_dir)
            .expect("hueman sandmanor roles directory should be removable");
    }
}
