use std::io;
use std::path::Path;

use hollow_grove::hueman_support::{
    HUEMAN_FOURWAY_ARTIFACT_PATH, HUEMAN_START_CHOICES_ARTIFACT_PATH,
    build_hueman_glaushouse_roles_from_artifacts, hueman_glaushouse_roles_artifact_path,
};
use hollow_grove::{read_text_artifact, write_text_artifact};

fn main() -> io::Result<()> {
    let hueman_start_choices = read_text_artifact(Path::new(HUEMAN_START_CHOICES_ARTIFACT_PATH))?;
    let hueman_fourway = read_text_artifact(Path::new(HUEMAN_FOURWAY_ARTIFACT_PATH))?;
    let hueman_glaushouse_roles =
        build_hueman_glaushouse_roles_from_artifacts(&hueman_start_choices, &hueman_fourway);
    let artifact_path = hueman_glaushouse_roles_artifact_path();

    write_text_artifact(&artifact_path, &hueman_glaushouse_roles)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use hollow_grove::hueman_support::build_hueman_glaushouse_roles_from_artifacts;
    use hollow_grove::write_text_artifact;

    #[test]
    fn hueman_glaushouse_roles_reads_existing_artifacts() {
        let output = build_hueman_glaushouse_roles_from_artifacts("start", "fourway");
        assert!(output.starts_with("# Hueman Glaushouse Roles"));
        assert!(output.contains("Pixy -> Sprite -> Farie -> Nymph -> Siren -> Muse"));
        assert!(output.contains("exclusive Synthesis"));
        assert!(output.contains("mechanical-industrial medical capital"));
        assert!(output.contains("whoever holds final public Clearance is Prima Donna"));
        assert!(output.contains("Muse is the highest care form in Glaushouse"));
        assert!(output.contains("mines jades and refines Glaus Gel"));
        assert!(
            output.contains("Persephone and the recovery floor may each oppose false Clearance")
        );
        assert!(output.contains("## Composed Medical Roles"));
        assert!(output.contains("Gargoyle Surgeon = Being Hueman"));
        assert!(output.contains("Elf Radiologist = Being Hueman"));
        assert!(output.contains("Werewolf Emergency Nurse = Being Hueman"));
        assert!(output.contains("Gnome Emergency Physician = Being Hueman"));
        assert!(output.contains("Hueman Fourway bytes: 7."));
    }

    #[test]
    fn hueman_glaushouse_roles_writes_a_deterministic_file() {
        let hueman_glaushouse_roles =
            build_hueman_glaushouse_roles_from_artifacts("start", "fourway");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir = std::env::temp_dir().join(format!("hueman-glaushouse-roles-{nonce}"));
        let artifact_path = artifact_dir.join("hueman_glaushouse_roles.md");

        write_text_artifact(&artifact_path, &hueman_glaushouse_roles)
            .expect("hueman glaushouse roles artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path)
                .expect("hueman glaushouse roles artifact should read"),
            hueman_glaushouse_roles
        );

        fs::remove_file(&artifact_path)
            .expect("hueman glaushouse roles artifact should be removable");
        fs::remove_dir(&artifact_dir)
            .expect("hueman glaushouse roles directory should be removable");
    }
}
