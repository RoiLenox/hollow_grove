use std::io;
use std::path::{Path, PathBuf};

#[path = "../current_synthesis_support.rs"]
mod current_synthesis_support;

use current_synthesis_support::{
    CURRENT_SYNTHESIS_CONSEQUENCE_ARTIFACT_PATH, CURRENT_SYNTHESIS_READINESS_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_SELECTION_ARTIFACT_PATH, build_current_synthesis_readiness_from_artifacts,
    read_artifact, write_artifact,
};

fn artifact_path() -> PathBuf {
    PathBuf::from(CURRENT_SYNTHESIS_READINESS_ARTIFACT_PATH)
}

fn main() -> io::Result<()> {
    let current_synthesis_consequence =
        read_artifact(Path::new(CURRENT_SYNTHESIS_CONSEQUENCE_ARTIFACT_PATH))?;
    let current_synthesis_selection =
        read_artifact(Path::new(CURRENT_SYNTHESIS_SELECTION_ARTIFACT_PATH))?;
    let current_synthesis_readiness = build_current_synthesis_readiness_from_artifacts(
        &current_synthesis_consequence,
        &current_synthesis_selection,
    );
    let artifact_path = artifact_path();

    write_artifact(&artifact_path, &current_synthesis_readiness)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::current_synthesis_support::{
        build_current_synthesis_readiness_from_artifacts, write_artifact,
    };

    #[test]
    fn current_synthesis_readiness_reads_existing_artifacts() {
        let current_synthesis_consequence = "# Current Synthesis Consequence\n\nconsequence";
        let current_synthesis_selection = "# Current Synthesis Selection\n\nselection";

        assert_eq!(
            build_current_synthesis_readiness_from_artifacts(
                current_synthesis_consequence,
                current_synthesis_selection
            ),
            "# Current Synthesis Readiness\n\n\
             ## Locked\n\n\
             - sequence locked\n\
             - topology locked\n\
             - client placement locked\n\
             - choice locked\n\
             - contract locked\n\
             - preview locked\n\
             - operational view locked\n\
             - selection locked\n\
             - consequence locked\n\n\
             ## Missing Before Execution\n\n\
             - route execution rules are not defined\n\
             - `PLEB`/`META` behavior is not active\n\
             - HAL automation is not enabled\n\
             - Clouseau live interpretation is not enabled\n\
             - runtime state is not introduced\n\n\
             ## Current Readiness\n\n\
             - `PLEB` cannot act yet\n\
             - `META` cannot act yet\n\
             - HAL cannot automate yet\n\
             - Clouseau cannot interpret live behavior yet\n\
             - Current Synthesis remains read-only\n\n\
             ## Artifact Inputs\n\n\
             Current Synthesis consequence bytes: 44.\n\
             Current Synthesis selection bytes: 40.\n\n\
             ## Boundary Reminder\n\n\
             Readiness belongs to Current Synthesis. Hollow Grove remains unchanged.\n"
        );
    }

    #[test]
    fn current_synthesis_readiness_writes_a_deterministic_file() {
        let current_synthesis_readiness =
            build_current_synthesis_readiness_from_artifacts("consequence", "selection");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir =
            std::env::temp_dir().join(format!("current-synthesis-readiness-{nonce}"));
        let artifact_path = artifact_dir.join("current_synthesis_readiness.md");

        write_artifact(&artifact_path, &current_synthesis_readiness)
            .expect("current synthesis readiness artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path)
                .expect("current synthesis readiness artifact should be readable"),
            current_synthesis_readiness
        );

        fs::remove_file(&artifact_path)
            .expect("current synthesis readiness artifact should be removable");
        fs::remove_dir(&artifact_dir)
            .expect("current synthesis readiness directory should be removable");
    }
}
