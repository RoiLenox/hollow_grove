use std::io;
use std::path::{Path, PathBuf};

#[path = "../current_synthesis_support.rs"]
mod current_synthesis_support;

use current_synthesis_support::{
    CURRENT_SYNTHESIS_CHOICE_ARTIFACT_PATH, CURRENT_SYNTHESIS_CLIENTS_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_TOPOLOGY_ARTIFACT_PATH, build_current_synthesis_choice_from_artifacts,
    read_artifact, write_artifact,
};

fn artifact_path() -> PathBuf {
    PathBuf::from(CURRENT_SYNTHESIS_CHOICE_ARTIFACT_PATH)
}

fn main() -> io::Result<()> {
    let current_synthesis_clients =
        read_artifact(Path::new(CURRENT_SYNTHESIS_CLIENTS_ARTIFACT_PATH))?;
    let current_synthesis_topology =
        read_artifact(Path::new(CURRENT_SYNTHESIS_TOPOLOGY_ARTIFACT_PATH))?;
    let current_synthesis_choice = build_current_synthesis_choice_from_artifacts(
        &current_synthesis_clients,
        &current_synthesis_topology,
    );
    let artifact_path = artifact_path();

    write_artifact(&artifact_path, &current_synthesis_choice)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::current_synthesis_support::{
        build_current_synthesis_choice_from_artifacts, write_artifact,
    };

    #[test]
    fn current_synthesis_choice_reads_existing_artifacts() {
        let current_synthesis_clients = "# Current Synthesis Clients\n\nclients";
        let current_synthesis_topology = "# Current Synthesis Topology\n\ntopology";

        assert_eq!(
            build_current_synthesis_choice_from_artifacts(
                current_synthesis_clients,
                current_synthesis_topology
            ),
            "# Current Synthesis Choice\n\n\
             ## Available Sides\n\n\
             - `PLEB` is available.\n\
             - `META` is available.\n\n\
             ## Placement Lock\n\n\
             - HAL remains assigned to `META`.\n\
             - Clouseau remains assigned to `PLEB`.\n\n\
             ## Choice Status\n\n\
             - user path choice is not executing yet\n\
             - no traversal\n\
             - no movement\n\
             - no automation\n\
             - no runtime state\n\n\
             ## Artifact Inputs\n\n\
             Current Synthesis clients bytes: 36.\n\
             Current Synthesis topology bytes: 38.\n\n\
             ## Boundary Reminder\n\n\
             Path choice belongs to Current Synthesis. Hollow Grove remains unchanged.\n"
        );
    }

    #[test]
    fn current_synthesis_choice_writes_a_deterministic_file() {
        let current_synthesis_choice =
            build_current_synthesis_choice_from_artifacts("clients", "topology");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir = std::env::temp_dir().join(format!("current-synthesis-choice-{nonce}"));
        let artifact_path = artifact_dir.join("current_synthesis_choice.md");

        write_artifact(&artifact_path, &current_synthesis_choice)
            .expect("current synthesis choice artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path)
                .expect("current synthesis choice artifact should be readable"),
            current_synthesis_choice
        );

        fs::remove_file(&artifact_path)
            .expect("current synthesis choice artifact should be removable");
        fs::remove_dir(&artifact_dir)
            .expect("current synthesis choice directory should be removable");
    }
}
