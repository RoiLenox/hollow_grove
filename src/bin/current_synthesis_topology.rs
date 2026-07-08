use std::io;
use std::path::{Path, PathBuf};

#[path = "../current_synthesis_support.rs"]
mod current_synthesis_support;

use current_synthesis_support::{
    CURRENT_SYNTHESIS_SEQUENCE_ARTIFACT_PATH, CURRENT_SYNTHESIS_STATE_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_TOPOLOGY_ARTIFACT_PATH, build_current_synthesis_topology_from_artifacts,
    read_artifact, write_artifact,
};

fn artifact_path() -> PathBuf {
    PathBuf::from(CURRENT_SYNTHESIS_TOPOLOGY_ARTIFACT_PATH)
}

fn main() -> io::Result<()> {
    let current_synthesis_sequence =
        read_artifact(Path::new(CURRENT_SYNTHESIS_SEQUENCE_ARTIFACT_PATH))?;
    let current_synthesis_state = read_artifact(Path::new(CURRENT_SYNTHESIS_STATE_ARTIFACT_PATH))?;
    let current_synthesis_topology = build_current_synthesis_topology_from_artifacts(
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
        let current_synthesis_sequence = "# Current Synthesis Sequence\n\nsequence";
        let current_synthesis_state = "# Current Synthesis State\n\nstate";

        assert_eq!(
            build_current_synthesis_topology_from_artifacts(
                current_synthesis_sequence,
                current_synthesis_state
            ),
            "# Current Synthesis Topology\n\n\
             ## Joint Order\n\n\
             ```text\n\
             P/M\n\
             ↓\n\
             L/E\n\
             ↓\n\
             E/T\n\
             ↓\n\
             B/A\n\
             ```\n\n\
             ## Adjacency\n\n\
             - `P/M` connects to `L/E`.\n\
             - `L/E` connects to `E/T`.\n\
             - `E/T` connects to `B/A`.\n\n\
             ## Side Assignment\n\n\
             - HAL is assigned to the `META` side of each joint.\n\
             - Clouseau is assigned to the `PLEB` side of each joint.\n\n\
             ## Inverse Curved Route\n\n\
             The inverse curved route runs beneath the plains as downstream Current Synthesis geography.\n\n\
             ```text\n\
             Aura Basin\n\
             ↓\n\
             Aura Fields\n\
             ↓\n\
             Aura Beach\n\
             ```\n\n\
             These remain route regions and route stations, not Hollow Grove layers.\n\n\
             ## Route Material Families\n\n\
             - route material may present as `dark current` or `hollow current`\n\
             - route material may present as `reflective aura` or `holographic aura`\n\
             - subtype presence does not change joint order or side assignment\n\n\
             ## Deferral\n\n\
             - traversal deferred\n\
             - `PLEB`/`META` execution deferred\n\
             - HAL behavior deferred\n\
             - Clouseau behavior deferred\n\
             - `niri`/`river` integration deferred\n\n\
             ## Artifact Inputs\n\n\
             Current Synthesis sequence bytes: 38.\n\
             Current Synthesis state bytes: 32.\n\n\
             ## Boundary Reminder\n\n\
             Topology belongs to Current Synthesis. Hollow Grove remains unchanged.\n"
        );
    }

    #[test]
    fn current_synthesis_topology_writes_a_deterministic_file() {
        let current_synthesis_topology =
            build_current_synthesis_topology_from_artifacts("sequence", "state");
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
