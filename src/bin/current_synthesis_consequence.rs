use std::io;
use std::path::{Path, PathBuf};

#[path = "../current_synthesis_support.rs"]
mod current_synthesis_support;

use current_synthesis_support::{
    CURRENT_SYNTHESIS_CONSEQUENCE_ARTIFACT_PATH, CURRENT_SYNTHESIS_OPERATIONAL_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_SELECTION_ARTIFACT_PATH, build_current_synthesis_consequence_from_artifacts,
    read_artifact, write_artifact,
};

fn artifact_path() -> PathBuf {
    PathBuf::from(CURRENT_SYNTHESIS_CONSEQUENCE_ARTIFACT_PATH)
}

fn main() -> io::Result<()> {
    let current_synthesis_selection =
        read_artifact(Path::new(CURRENT_SYNTHESIS_SELECTION_ARTIFACT_PATH))?;
    let current_synthesis_operational =
        read_artifact(Path::new(CURRENT_SYNTHESIS_OPERATIONAL_ARTIFACT_PATH))?;
    let current_synthesis_consequence = build_current_synthesis_consequence_from_artifacts(
        &current_synthesis_selection,
        &current_synthesis_operational,
    );
    let artifact_path = artifact_path();

    write_artifact(&artifact_path, &current_synthesis_consequence)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::current_synthesis_support::{
        build_current_synthesis_consequence_from_artifacts, write_artifact,
    };

    #[test]
    fn current_synthesis_consequence_reads_existing_artifacts() {
        let current_synthesis_selection = "# Current Synthesis Selection\n\nselection";
        let current_synthesis_operational = "# Current Synthesis Operational View\n\noperational";

        assert_eq!(
            build_current_synthesis_consequence_from_artifacts(
                current_synthesis_selection,
                current_synthesis_operational
            ),
            "# Current Synthesis Consequence\n\n\
             ## Selected Side Consequence\n\n\
             - `PLEB` remains the occupied selected side.\n\
             - Clouseau remains on `PLEB`.\n\
             - `PLEB` remains descriptive only.\n\n\
             ## Complementary Side Consequence\n\n\
             - `META` remains the complementary side.\n\
             - HAL remains on `META`.\n\
             - `META` remains descriptive only.\n\n\
             ## Still Deferred\n\n\
             - no traversal\n\
             - no movement\n\
             - no automation\n\
             - no runtime state\n\
             - no path execution\n\
             - no feedback into Hollow Grove\n\n\
             ## Cannot Happen Yet\n\n\
             - HAL does not automate\n\
             - Clouseau does not interpret live behavior\n\
             - `PLEB`/`META` do not execute as routes\n\n\
             ## Artifact Inputs\n\n\
             Current Synthesis selection bytes: 40.\n\
             Current Synthesis operational bytes: 49.\n\n\
             ## Boundary Reminder\n\n\
             Consequence belongs to Current Synthesis. Hollow Grove remains unchanged.\n"
        );
    }

    #[test]
    fn current_synthesis_consequence_writes_a_deterministic_file() {
        let current_synthesis_consequence =
            build_current_synthesis_consequence_from_artifacts("selection", "operational");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir =
            std::env::temp_dir().join(format!("current-synthesis-consequence-{nonce}"));
        let artifact_path = artifact_dir.join("current_synthesis_consequence.md");

        write_artifact(&artifact_path, &current_synthesis_consequence)
            .expect("current synthesis consequence artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path)
                .expect("current synthesis consequence artifact should be readable"),
            current_synthesis_consequence
        );

        fs::remove_file(&artifact_path)
            .expect("current synthesis consequence artifact should be removable");
        fs::remove_dir(&artifact_dir)
            .expect("current synthesis consequence directory should be removable");
    }
}
