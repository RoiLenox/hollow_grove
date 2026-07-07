use std::io;
use std::path::Path;

use hollow_grove::hueman_support::{
    HUEMAN_BOUNDARY_ARTIFACT_PATH, HUEMAN_MOTION_MAP_ARTIFACT_PATH,
    build_hueman_start_choices_from_artifacts, hueman_start_choices_artifact_path,
};
use hollow_grove::{read_text_artifact, write_text_artifact};

fn main() -> io::Result<()> {
    let hueman_boundary = read_text_artifact(Path::new(HUEMAN_BOUNDARY_ARTIFACT_PATH))?;
    let hueman_motion_map = read_text_artifact(Path::new(HUEMAN_MOTION_MAP_ARTIFACT_PATH))?;
    let hueman_start_choices =
        build_hueman_start_choices_from_artifacts(&hueman_boundary, &hueman_motion_map);
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
        assert_eq!(
            build_hueman_start_choices_from_artifacts("boundary", "motion"),
            "# Hueman Start Choices\n\n\
             ## End User Archetypes\n\n\
             - `goblin`\n\
             - `gremlin`\n\
             - `pixy`\n\
             - `sprite`\n\n\
             ## Starting Places\n\n\
             - Flynt\n\
             - Stonebend\n\
             - Glaushouse\n\
             - Sandmanor\n\n\
             ## Initial Start Roster\n\n\
             - `goblin` starts in Flynt\n\
             - `gremlin` starts in Stonebend\n\
             - `pixy` starts in Glaushouse\n\
             - `sprite` starts in Sandmanor\n\n\
             ## Status\n\n\
             - the end user may choose one archetype\n\
             - the starting place follows the initial Hueman roster\n\
             - world behavior is not active yet\n\
             - species mechanics are not active yet\n\
             - no feedback into Current Synthesis\n\
             - no feedback into Hollow Grove\n\n\
             ## Artifact Inputs\n\n\
             Hueman boundary bytes: 8.\n\
             Hueman motion map bytes: 6.\n\n\
             ## Boundary Reminder\n\n\
             This is a Hueman-layer start declaration only. It does not change Current Synthesis or Hollow Grove.\n"
        );
    }

    #[test]
    fn hueman_start_choices_writes_a_deterministic_file() {
        let hueman_start_choices =
            build_hueman_start_choices_from_artifacts("boundary", "motion");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir = std::env::temp_dir().join(format!("hueman-start-choices-{nonce}"));
        let artifact_path = artifact_dir.join("hueman_start_choices.md");

        write_text_artifact(&artifact_path, &hueman_start_choices)
            .expect("hueman start choices artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path)
                .expect("hueman start choices artifact should read"),
            hueman_start_choices
        );

        fs::remove_file(&artifact_path)
            .expect("hueman start choices artifact should be removable");
        fs::remove_dir(&artifact_dir)
            .expect("hueman start choices directory should be removable");
    }
}
