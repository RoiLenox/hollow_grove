use std::io;
use std::path::{Path, PathBuf};

use hollow_grove::SnapshotBoundary;

#[path = "../current_synthesis_support.rs"]
mod current_synthesis_support;

use current_synthesis_support::{
    CURRENT_SYNTHESIS_CLIENTS_ARTIFACT_PATH, CURRENT_SYNTHESIS_SEQUENCE_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_TOPOLOGY_ARTIFACT_PATH, SNAPSHOT_ARTIFACT_PATH,
    build_current_synthesis_clients_from_boundary, read_artifact, write_artifact,
};

fn artifact_path() -> PathBuf {
    PathBuf::from(CURRENT_SYNTHESIS_CLIENTS_ARTIFACT_PATH)
}

fn main() -> io::Result<()> {
    let snapshot = read_artifact(Path::new(SNAPSHOT_ARTIFACT_PATH))?;
    let snapshot_boundary = SnapshotBoundary::parse(&snapshot)?;
    let current_synthesis_topology =
        read_artifact(Path::new(CURRENT_SYNTHESIS_TOPOLOGY_ARTIFACT_PATH))?;
    let current_synthesis_sequence =
        read_artifact(Path::new(CURRENT_SYNTHESIS_SEQUENCE_ARTIFACT_PATH))?;
    let current_synthesis_clients = build_current_synthesis_clients_from_boundary(
        &snapshot_boundary,
        snapshot.len(),
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

    use hollow_grove::{Symptom, build_snapshot_output, run_kernel_cycle};

    use super::current_synthesis_support::{
        build_current_synthesis_clients_from_artifacts, write_artifact,
    };

    #[test]
    fn current_synthesis_clients_reads_existing_artifacts() {
        let snapshot = build_snapshot_output(&run_kernel_cycle(Symptom::origin()));
        let current_synthesis_topology = "# Current Synthesis Topology\n\ntopology";
        let current_synthesis_sequence = "# Current Synthesis Sequence\n\nsequence";
        let output = build_current_synthesis_clients_from_artifacts(
            &snapshot,
            current_synthesis_topology,
            current_synthesis_sequence,
        )
        .expect("current synthesis clients should build");

        assert!(output.contains("## Frozen Kernel Placement Boundary"));
        assert!(output.contains("- universal landed point: `Point²`"));
        assert!(output.contains("Snapshot bytes: 587."));
        assert!(output.contains("Current Synthesis topology bytes: 38."));
        assert!(output.contains("Current Synthesis sequence bytes: 38."));
    }

    #[test]
    fn current_synthesis_clients_writes_a_deterministic_file() {
        let snapshot = build_snapshot_output(&run_kernel_cycle(Symptom::origin()));
        let current_synthesis_clients =
            build_current_synthesis_clients_from_artifacts(&snapshot, "topology", "sequence")
                .expect("current synthesis clients should build");
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
