use std::io;
use std::path::{Path, PathBuf};

#[path = "../current_synthesis_support.rs"]
mod current_synthesis_support;

use current_synthesis_support::{
    CURRENT_SYNTHESIS_CHOICE_ARTIFACT_PATH, CURRENT_SYNTHESIS_OPERATIONAL_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_SELECTION_ARTIFACT_PATH, build_current_synthesis_selection_from_artifacts,
    read_artifact, write_artifact,
};

fn artifact_path() -> PathBuf {
    PathBuf::from(CURRENT_SYNTHESIS_SELECTION_ARTIFACT_PATH)
}

fn main() -> io::Result<()> {
    let current_synthesis_choice =
        read_artifact(Path::new(CURRENT_SYNTHESIS_CHOICE_ARTIFACT_PATH))?;
    let current_synthesis_operational =
        read_artifact(Path::new(CURRENT_SYNTHESIS_OPERATIONAL_ARTIFACT_PATH))?;
    let current_synthesis_selection = build_current_synthesis_selection_from_artifacts(
        &current_synthesis_choice,
        &current_synthesis_operational,
    );
    let artifact_path = artifact_path();

    write_artifact(&artifact_path, &current_synthesis_selection)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::current_synthesis_support::{
        build_current_synthesis_selection_from_artifacts, write_artifact,
    };

    #[test]
    fn current_synthesis_selection_reads_existing_artifacts() {
        let current_synthesis_choice = "# Current Synthesis Choice\n\nchoice";
        let current_synthesis_operational = "# Current Synthesis Operational View\n\noperational";

        assert_eq!(
            build_current_synthesis_selection_from_artifacts(
                current_synthesis_choice,
                current_synthesis_operational
            ),
            "# Current Synthesis Selection\n\n\
             ## Selected Side\n\n\
             - `PLEB`\n\n\
             ## Complementary Side\n\n\
             - `META`\n\n\
             ## Placement Lock\n\n\
             - HAL remains on `META`.\n\
             - Clouseau remains on `PLEB`.\n\n\
             ## Selection Status\n\n\
             - deterministic read-only selection\n\
             - no traversal\n\
             - no movement\n\
             - no automation\n\
             - no runtime state\n\
             - no execution\n\n\
             ## Artifact Inputs\n\n\
             Current Synthesis choice bytes: 34.\n\
             Current Synthesis operational bytes: 49.\n\n\
             ## Boundary Reminder\n\n\
             Selection belongs to Current Synthesis. Hollow Grove remains unchanged.\n"
        );
    }

    #[test]
    fn current_synthesis_selection_writes_a_deterministic_file() {
        let current_synthesis_selection =
            build_current_synthesis_selection_from_artifacts("choice", "operational");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir =
            std::env::temp_dir().join(format!("current-synthesis-selection-{nonce}"));
        let artifact_path = artifact_dir.join("current_synthesis_selection.md");

        write_artifact(&artifact_path, &current_synthesis_selection)
            .expect("current synthesis selection artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path)
                .expect("current synthesis selection artifact should be readable"),
            current_synthesis_selection
        );

        fs::remove_file(&artifact_path)
            .expect("current synthesis selection artifact should be removable");
        fs::remove_dir(&artifact_dir)
            .expect("current synthesis selection directory should be removable");
    }
}
