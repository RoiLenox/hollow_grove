use std::io;
use std::path::{Path, PathBuf};

#[path = "../current_synthesis_support.rs"]
mod current_synthesis_support;

use current_synthesis_support::{
    CURRENT_SYNTHESIS_BASE_ARTIFACT_PATH, CURRENT_SYNTHESIS_SEQUENCE_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_STATE_ARTIFACT_PATH, build_current_synthesis_sequence_from_artifacts,
    read_artifact, write_artifact,
};

fn artifact_path() -> PathBuf {
    PathBuf::from(CURRENT_SYNTHESIS_SEQUENCE_ARTIFACT_PATH)
}

fn main() -> io::Result<()> {
    let current_synthesis_base = read_artifact(Path::new(CURRENT_SYNTHESIS_BASE_ARTIFACT_PATH))?;
    let current_synthesis_state = read_artifact(Path::new(CURRENT_SYNTHESIS_STATE_ARTIFACT_PATH))?;
    let current_synthesis_sequence = build_current_synthesis_sequence_from_artifacts(
        &current_synthesis_base,
        &current_synthesis_state,
    );
    let artifact_path = artifact_path();

    write_artifact(&artifact_path, &current_synthesis_sequence)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::current_synthesis_support::{
        build_current_synthesis_sequence_from_artifacts, write_artifact,
    };

    #[test]
    fn current_synthesis_sequence_reads_existing_artifacts() {
        let current_synthesis_base = "# Current Synthesis Base\n\nbase";
        let current_synthesis_state = "# Current Synthesis State\n\nstate";

        assert_eq!(
            build_current_synthesis_sequence_from_artifacts(
                current_synthesis_base,
                current_synthesis_state
            ),
            "# Current Synthesis Sequence\n\n\
             ## Sequence Lock\n\n\
             ```text\n\
             P/M\n\
             ↓\n\
             L/E\n\
             ↓\n\
             E/T\n\
             ↓\n\
             B/A\n\
             ```\n\n\
             ## Joint Model\n\n\
             Each paired joint has a `PLEB` side, a `META` side, three possible arms of movement on each side, one bonded arm, and unused arms that remain as clue context, environmental residue, or route material.\n\n\
             ## Client Sides\n\n\
             - HAL belongs to `META`.\n\
             - Clouseau belongs to `PLEB`.\n\n\
             ## Topology Status\n\n\
             Topology is downstream from this sequence.\n\n\
             ## Deferral\n\n\
             - `PLEB`/`META` execution deferred\n\
             - HAL behavior deferred\n\
             - Clouseau behavior deferred\n\
             - `niri`/`river` integration deferred\n\n\
             ## Artifact Inputs\n\n\
             Current Synthesis base bytes: 30.\n\
             Current Synthesis state bytes: 32.\n\n\
             ## Boundary Reminder\n\n\
             This sequence belongs to Current Synthesis. Hollow Grove remains unchanged.\n"
        );
    }

    #[test]
    fn current_synthesis_sequence_writes_a_deterministic_file() {
        let current_synthesis_sequence =
            build_current_synthesis_sequence_from_artifacts("base", "state");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir = std::env::temp_dir().join(format!("current-synthesis-sequence-{nonce}"));
        let artifact_path = artifact_dir.join("current_synthesis_sequence.md");

        write_artifact(&artifact_path, &current_synthesis_sequence)
            .expect("current synthesis sequence artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path)
                .expect("current synthesis sequence artifact should be readable"),
            current_synthesis_sequence
        );

        fs::remove_file(&artifact_path)
            .expect("current synthesis sequence artifact should be removable");
        fs::remove_dir(&artifact_dir)
            .expect("current synthesis sequence directory should be removable");
    }
}
