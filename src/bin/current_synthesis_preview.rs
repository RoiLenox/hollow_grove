use std::io;
use std::path::{Path, PathBuf};

#[path = "../current_synthesis_support.rs"]
mod current_synthesis_support;

use current_synthesis_support::{
    CURRENT_SYNTHESIS_CONTRACT_ARTIFACT_PATH, CURRENT_SYNTHESIS_PREVIEW_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_SEQUENCE_ARTIFACT_PATH, build_current_synthesis_preview_from_artifacts,
    read_artifact, write_artifact,
};

fn artifact_path() -> PathBuf {
    PathBuf::from(CURRENT_SYNTHESIS_PREVIEW_ARTIFACT_PATH)
}

fn main() -> io::Result<()> {
    let current_synthesis_contract =
        read_artifact(Path::new(CURRENT_SYNTHESIS_CONTRACT_ARTIFACT_PATH))?;
    let current_synthesis_sequence =
        read_artifact(Path::new(CURRENT_SYNTHESIS_SEQUENCE_ARTIFACT_PATH))?;
    let current_synthesis_preview = build_current_synthesis_preview_from_artifacts(
        &current_synthesis_contract,
        &current_synthesis_sequence,
    );
    let artifact_path = artifact_path();

    write_artifact(&artifact_path, &current_synthesis_preview)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::current_synthesis_support::{
        build_current_synthesis_preview_from_artifacts, write_artifact,
    };

    #[test]
    fn current_synthesis_preview_reads_existing_artifacts() {
        let current_synthesis_contract = "# Current Synthesis Contract\n\ncontract";
        let current_synthesis_sequence = "# Current Synthesis Sequence\n\nsequence";

        assert_eq!(
            build_current_synthesis_preview_from_artifacts(
                current_synthesis_contract,
                current_synthesis_sequence
            ),
            "# Current Synthesis Preview\n\n\
             ## `PLEB` Chosen\n\n\
             - HAL remains on `META`.\n\
             - Clouseau occupies `PLEB`.\n\
             - Joint order remains `P/M -> L/E -> E/T -> B/A`.\n\n\
             ## `META` Chosen\n\n\
             - HAL remains on `META`.\n\
             - Clouseau remains on `PLEB`.\n\
             - Joint order remains `P/M -> L/E -> E/T -> B/A`.\n\n\
             ## Preview Status\n\n\
             - no traversal\n\
             - no movement\n\
             - no automation\n\
             - no runtime state\n\
             - no execution\n\n\
             ## Artifact Inputs\n\n\
             Current Synthesis contract bytes: 38.\n\
             Current Synthesis sequence bytes: 38.\n\n\
             ## Boundary Reminder\n\n\
             Route preview belongs to Current Synthesis. Hollow Grove remains unchanged.\n"
        );
    }

    #[test]
    fn current_synthesis_preview_writes_a_deterministic_file() {
        let current_synthesis_preview =
            build_current_synthesis_preview_from_artifacts("contract", "sequence");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir = std::env::temp_dir().join(format!("current-synthesis-preview-{nonce}"));
        let artifact_path = artifact_dir.join("current_synthesis_preview.md");

        write_artifact(&artifact_path, &current_synthesis_preview)
            .expect("current synthesis preview artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path)
                .expect("current synthesis preview artifact should be readable"),
            current_synthesis_preview
        );

        fs::remove_file(&artifact_path)
            .expect("current synthesis preview artifact should be removable");
        fs::remove_dir(&artifact_dir)
            .expect("current synthesis preview directory should be removable");
    }
}
