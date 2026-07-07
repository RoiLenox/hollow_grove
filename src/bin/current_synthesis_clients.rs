use std::io;
use std::path::{Path, PathBuf};

#[path = "../current_synthesis_support.rs"]
mod current_synthesis_support;

use current_synthesis_support::{
    CURRENT_SYNTHESIS_CLIENTS_ARTIFACT_PATH, CURRENT_SYNTHESIS_SEQUENCE_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_TOPOLOGY_ARTIFACT_PATH, build_current_synthesis_clients_from_artifacts,
    read_artifact, write_artifact,
};

fn artifact_path() -> PathBuf {
    PathBuf::from(CURRENT_SYNTHESIS_CLIENTS_ARTIFACT_PATH)
}

fn main() -> io::Result<()> {
    let current_synthesis_topology =
        read_artifact(Path::new(CURRENT_SYNTHESIS_TOPOLOGY_ARTIFACT_PATH))?;
    let current_synthesis_sequence =
        read_artifact(Path::new(CURRENT_SYNTHESIS_SEQUENCE_ARTIFACT_PATH))?;
    let current_synthesis_clients = build_current_synthesis_clients_from_artifacts(
        &current_synthesis_topology,
        &current_synthesis_sequence,
    );
    let artifact_path = artifact_path();

    write_artifact(&artifact_path, &current_synthesis_clients)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::current_synthesis_support::{
        build_current_synthesis_clients_from_artifacts, write_artifact,
    };

    #[test]
    fn current_synthesis_clients_reads_existing_artifacts() {
        let current_synthesis_topology = "# Current Synthesis Topology\n\ntopology";
        let current_synthesis_sequence = "# Current Synthesis Sequence\n\nsequence";

        assert_eq!(
            build_current_synthesis_clients_from_artifacts(
                current_synthesis_topology,
                current_synthesis_sequence
            ),
            "# Current Synthesis Clients\n\n\
             ## Placement Lock\n\n\
             - HAL is placed on the `META` side of each joint.\n\
             - Clouseau is placed on the `PLEB` side of each joint.\n\n\
             ## Joint Placement\n\n\
             - `P/M`: HAL on `META`, Clouseau on `PLEB`.\n\
             - `L/E`: HAL on `META`, Clouseau on `PLEB`.\n\
             - `E/T`: HAL on `META`, Clouseau on `PLEB`.\n\
             - `B/A`: HAL on `META`, Clouseau on `PLEB`.\n\n\
             ## Behavior Status\n\n\
             - no movement\n\
             - no traversal\n\
             - no automation\n\
             - no `PLEB`/`META` execution\n\
             - no runtime state\n\n\
             ## Artifact Inputs\n\n\
             Current Synthesis topology bytes: 38.\n\
             Current Synthesis sequence bytes: 38.\n\n\
             ## Boundary Reminder\n\n\
             Client placement belongs to Current Synthesis. Hollow Grove remains unchanged.\n"
        );
    }

    #[test]
    fn current_synthesis_clients_writes_a_deterministic_file() {
        let current_synthesis_clients =
            build_current_synthesis_clients_from_artifacts("topology", "sequence");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir = std::env::temp_dir().join(format!("current-synthesis-clients-{nonce}"));
        let artifact_path = artifact_dir.join("current_synthesis_clients.md");

        write_artifact(&artifact_path, &current_synthesis_clients)
            .expect("current synthesis clients artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path)
                .expect("current synthesis clients artifact should be readable"),
            current_synthesis_clients
        );

        fs::remove_file(&artifact_path)
            .expect("current synthesis clients artifact should be removable");
        fs::remove_dir(&artifact_dir)
            .expect("current synthesis clients directory should be removable");
    }
}
