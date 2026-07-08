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
        assert_eq!(
            build_hueman_glaushouse_roles_from_artifacts("start", "fourway"),
            "# Hueman Glaushouse Roles\n\n\
             ## Structural Rule\n\n\
             Glaushouse carries a scene-facing court and care order inside Hueman's world layer, where leadership, succession, and nursing remain socially visible without leaving the Fourway boundary.\n\n\
             ## Canonical Anchor\n\n\
             - Glaushouse remains the South-facing `pixy` start on the Fourway.\n\
             - Glaushouse holds the luminous southern threshold of the roster.\n\
             - Glaushouse mines jades.\n\
             - Glaushouse exports Jadomer as its main outward trade good, formed from current and jades.\n\
\n\
             ## Glaushouse Order\n\n\
             - Prima Donna is the leader.\n\
             - Persephone is the assistant and step-down continuity.\n\
             - Nightengales are the nurses and the common people of Glaushouse.\n\
\n\
             ## Social Balance\n\n\
             - Prima Donna sets tone, command, and public face.\n\
             - Persephone carries relay authority and may step down from the lead into continuity duty.\n\
             - Nightengales keep recovery, bedside care, and the lived body of the kingdom.\n\
             - leadership does not erase the people; the people remain visible through the Nightengales.\n\n\
             ## Status\n\n\
             - Glaushouse roles are descriptive-only for now\n\
             - no court resolver or succession engine is active\n\
             - no nurse AI or care loop is active\n\
             - no feedback into Current Synthesis\n\
             - no feedback into Hollow Grove\n\n\
             ## Artifact Inputs\n\n\
             Hueman Start Choices bytes: 5.\n\
             Hueman Fourway bytes: 7.\n\n\
             ## Boundary Reminder\n\n\
             Glaushouse roles belong to Hueman's kingdom layer. They do not replace scene logic, procedural care systems, or any Current Synthesis client boundary.\n"
        );
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
