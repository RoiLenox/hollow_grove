use std::io;
use std::path::{Path, PathBuf};

#[path = "../current_synthesis_support.rs"]
mod current_synthesis_support;

use current_synthesis_support::{
    CURRENT_SYNTHESIS_SEQUENCE_ARTIFACT_PATH, CURRENT_SYNTHESIS_STATE_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_TOPOLOGY_ARTIFACT_PATH, SNAPSHOT_ARTIFACT_PATH,
    build_current_synthesis_topology_from_artifacts, read_artifact, write_artifact,
};

fn artifact_path() -> PathBuf {
    PathBuf::from(CURRENT_SYNTHESIS_TOPOLOGY_ARTIFACT_PATH)
}

fn main() -> io::Result<()> {
    let snapshot = read_artifact(Path::new(SNAPSHOT_ARTIFACT_PATH))?;
    let current_synthesis_sequence =
        read_artifact(Path::new(CURRENT_SYNTHESIS_SEQUENCE_ARTIFACT_PATH))?;
    let current_synthesis_state = read_artifact(Path::new(CURRENT_SYNTHESIS_STATE_ARTIFACT_PATH))?;
    let current_synthesis_topology = build_current_synthesis_topology_from_artifacts(
        &snapshot,
        &current_synthesis_sequence,
        &current_synthesis_state,
    );
    let artifact_path = artifact_path();

    write_artifact(&artifact_path, &current_synthesis_topology)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::current_synthesis_support::{
        build_current_synthesis_topology_from_artifacts, write_artifact,
    };

    #[test]
    fn current_synthesis_topology_reads_existing_artifacts() {
        let snapshot = "{\n\
                        \x20\x20\"grove_seam_route\": \"PlebExterior\",\n\
                        \x20\x20\"hollow_beam_route\": \"BlepReturn\",\n\
                        \x20\x20\"landing_route\": \"BlepArrival\",\n\
                        \x20\x20\"landed_point\": \"Point²\",\n\
                        \x20\x20\"canonical_witness\": \"Point\\n↓\\nTriway\\n↓\\nFourway\\n↓\\nHollowGrove\\n↓\\nCurrentSeam [PlebExterior]\\n↓\\nAuraBeam [BlepReturn]\\n↓\\nPoint² (Landed Point) [BlepArrival]\"\n\
                        }";
        let current_synthesis_sequence = "# Current Synthesis Sequence\n\nsequence";
        let current_synthesis_state = "# Current Synthesis State\n\nstate";
        let output = build_current_synthesis_topology_from_artifacts(
            snapshot,
            current_synthesis_sequence,
            current_synthesis_state,
        );

        assert!(output.contains("## Frozen Kernel Entry Boundary"));
        assert!(output.contains("- universal landed point: `Point²`"));
        assert!(output.contains("Snapshot bytes: "));
        assert!(output.contains("Current Synthesis sequence bytes: 38."));
        assert!(output.contains("Current Synthesis state bytes: 32."));
    }

    #[test]
    fn current_synthesis_topology_writes_a_deterministic_file() {
        let current_synthesis_topology = build_current_synthesis_topology_from_artifacts(
            "{\n\
                 \x20\x20\"grove_seam_route\": \"PlebExterior\",\n\
                 \x20\x20\"hollow_beam_route\": \"BlepReturn\",\n\
                 \x20\x20\"landing_route\": \"BlepArrival\",\n\
                 \x20\x20\"landed_point\": \"Point²\",\n\
                 \x20\x20\"canonical_witness\": \"Point\\n↓\\nTriway\\n↓\\nFourway\\n↓\\nHollowGrove\\n↓\\nCurrentSeam [PlebExterior]\\n↓\\nAuraBeam [BlepReturn]\\n↓\\nPoint² (Landed Point) [BlepArrival]\"\n\
                 }",
            "sequence",
            "state",
        );
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir = std::env::temp_dir().join(format!("current-synthesis-topology-{nonce}"));
        let artifact_path = artifact_dir.join("current_synthesis_topology.md");

        write_artifact(&artifact_path, &current_synthesis_topology)
            .expect("current synthesis topology artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path)
                .expect("current synthesis topology artifact should be readable"),
            current_synthesis_topology
        );

        fs::remove_file(&artifact_path)
            .expect("current synthesis topology artifact should be removable");
        fs::remove_dir(&artifact_dir)
            .expect("current synthesis topology directory should be removable");
    }
}
