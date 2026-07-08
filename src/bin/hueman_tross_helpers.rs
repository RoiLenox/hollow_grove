use std::io;
use std::path::Path;

use hollow_grove::hueman_support::{
    HUEMAN_FOURWAY_ARTIFACT_PATH, HUEMAN_START_CHOICES_ARTIFACT_PATH,
    build_hueman_tross_helpers_from_artifacts, hueman_tross_helpers_artifact_path,
};
use hollow_grove::{read_text_artifact, write_text_artifact};

fn main() -> io::Result<()> {
    let hueman_start_choices = read_text_artifact(Path::new(HUEMAN_START_CHOICES_ARTIFACT_PATH))?;
    let hueman_fourway = read_text_artifact(Path::new(HUEMAN_FOURWAY_ARTIFACT_PATH))?;
    let hueman_tross_helpers =
        build_hueman_tross_helpers_from_artifacts(&hueman_start_choices, &hueman_fourway);
    let artifact_path = hueman_tross_helpers_artifact_path();

    write_text_artifact(&artifact_path, &hueman_tross_helpers)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use hollow_grove::hueman_support::build_hueman_tross_helpers_from_artifacts;
    use hollow_grove::write_text_artifact;

    #[test]
    fn hueman_tross_helpers_reads_existing_artifacts() {
        assert_eq!(
            build_hueman_tross_helpers_from_artifacts("start", "fourway"),
            "# Hueman Tross Helpers\n\n\
             ## Structural Rule\n\n\
             Tross is anchored in Flynt and runs as a north-to-south helper line inside Hueman's world layer.\n\n\
             ## Anchor\n\n\
             - Tross is in Flynt.\n\
             - Flynt remains North = `goblin` on the Fourway roster.\n\
             - Wardens are the people of Flynt.\n\
             - Flynt mines opals.\n\
             - Flynt exports Opal Oil as its main outward trade good, formed from hollow current and opal yield.\n\
             - Tross runs North -> South rather than spanning the whole Fourway equally.\n\n\
             ## Helper Pair\n\n\
             - Delinquent\n\
             - Juvenile\n\n\
             ## Personal Guard\n\n\
             - The White Dwarfs are Tross's personal guard.\n\
             - there are four White Dwarfs.\n\
             - they keep close guard around Tross rather than taking directional posts from the helper pair.\n\n\
             ## North-South Guard\n\n\
             - Juvenile guards North at the Flynt-facing head of the line.\n\
             - Delinquent guards South.\n\
             - South remains Glaushouse = `pixy` on the Fourway roster.\n\
             - helper duty runs down the line from Flynt instead of behaving like sovereign rule.\n\n\
             ## Status\n\n\
             - Tross helpers are descriptive-only for now\n\
             - no helper AI or encounter resolver is active\n\
             - no automatic north or south event gate is active\n\
             - no feedback into Current Synthesis\n\
             - no feedback into Hollow Grove\n\n\
             ## Artifact Inputs\n\n\
             Hueman Start Choices bytes: 5.\n\
             Hueman Fourway bytes: 7.\n\n\
             ## Boundary Reminder\n\n\
             Tross helpers belong to Hueman's Flynt-anchored directional line. They do not replace Fourway placement, civic roles, or kernel ownership.\n"
        );
    }

    #[test]
    fn hueman_tross_helpers_writes_a_deterministic_file() {
        let hueman_tross_helpers = build_hueman_tross_helpers_from_artifacts("start", "fourway");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir = std::env::temp_dir().join(format!("hueman-tross-helpers-{nonce}"));
        let artifact_path = artifact_dir.join("hueman_tross_helpers.md");

        write_text_artifact(&artifact_path, &hueman_tross_helpers)
            .expect("hueman tross helpers artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path).expect("hueman tross helpers artifact should read"),
            hueman_tross_helpers
        );

        fs::remove_file(&artifact_path).expect("hueman tross helpers artifact should be removable");
        fs::remove_dir(&artifact_dir).expect("hueman tross helpers directory should be removable");
    }
}
