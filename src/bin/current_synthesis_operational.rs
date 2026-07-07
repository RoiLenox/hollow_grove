use std::io;
use std::path::{Path, PathBuf};

#[path = "../current_synthesis_support.rs"]
mod current_synthesis_support;

use current_synthesis_support::{
    CURRENT_SYNTHESIS_CONTRACT_ARTIFACT_PATH, CURRENT_SYNTHESIS_OPERATIONAL_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_PREVIEW_ARTIFACT_PATH, build_current_synthesis_operational_from_artifacts,
    read_artifact, write_artifact,
};

fn artifact_path() -> PathBuf {
    PathBuf::from(CURRENT_SYNTHESIS_OPERATIONAL_ARTIFACT_PATH)
}

fn main() -> io::Result<()> {
    let current_synthesis_preview =
        read_artifact(Path::new(CURRENT_SYNTHESIS_PREVIEW_ARTIFACT_PATH))?;
    let current_synthesis_contract =
        read_artifact(Path::new(CURRENT_SYNTHESIS_CONTRACT_ARTIFACT_PATH))?;
    let current_synthesis_operational = build_current_synthesis_operational_from_artifacts(
        &current_synthesis_preview,
        &current_synthesis_contract,
    );
    let artifact_path = artifact_path();

    write_artifact(&artifact_path, &current_synthesis_operational)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::current_synthesis_support::{
        build_current_synthesis_operational_from_artifacts, write_artifact,
    };

    #[test]
    fn current_synthesis_operational_reads_existing_artifacts() {
        let current_synthesis_preview = "# Current Synthesis Preview\n\npreview";
        let current_synthesis_contract = "# Current Synthesis Contract\n\ncontract";

        assert_eq!(
            build_current_synthesis_operational_from_artifacts(
                current_synthesis_preview,
                current_synthesis_contract
            ),
            "# Current Synthesis Operational View\n\n\
             ## `PLEB` Side\n\n\
             - Clouseau belongs to `PLEB` as the clue-side client.\n\
             - `PLEB` remains the straight-side occupancy described by Current Synthesis.\n\
             - HAL does not occupy `PLEB`.\n\
             - no traversal or execution occurs here yet\n\n\
             ## `META` Side\n\n\
             - HAL belongs to `META` as the watch-side client.\n\
             - `META` remains the bent-side occupancy described by Current Synthesis.\n\
             - Clouseau does not occupy `META`.\n\
             - no traversal or execution occurs here yet\n\n\
             ## Shared Limits\n\n\
             - no automation\n\
             - no movement\n\
             - no runtime state\n\
             - no path execution\n\
             - no feedback into Hollow Grove\n\n\
             ## Artifact Inputs\n\n\
             Current Synthesis preview bytes: 36.\n\
             Current Synthesis contract bytes: 38.\n\n\
             ## Boundary Reminder\n\n\
             Operational meaning belongs to Current Synthesis. Hollow Grove remains unchanged.\n"
        );
    }

    #[test]
    fn current_synthesis_operational_writes_a_deterministic_file() {
        let current_synthesis_operational =
            build_current_synthesis_operational_from_artifacts("preview", "contract");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir =
            std::env::temp_dir().join(format!("current-synthesis-operational-{nonce}"));
        let artifact_path = artifact_dir.join("current_synthesis_operational.md");

        write_artifact(&artifact_path, &current_synthesis_operational)
            .expect("current synthesis operational artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path)
                .expect("current synthesis operational artifact should be readable"),
            current_synthesis_operational
        );

        fs::remove_file(&artifact_path)
            .expect("current synthesis operational artifact should be removable");
        fs::remove_dir(&artifact_dir)
            .expect("current synthesis operational directory should be removable");
    }
}
