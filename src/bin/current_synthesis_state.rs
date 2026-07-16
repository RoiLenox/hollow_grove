use std::io;
use std::path::{Path, PathBuf};

#[path = "../current_synthesis_support.rs"]
mod current_synthesis_support;

use current_synthesis_support::{
    ARTIFACT_INDEX_PATH, CURRENT_SYNTHESIS_BASE_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_STATE_ARTIFACT_PATH, build_current_synthesis_state_from_artifacts,
    load_artifact_index, read_artifact, write_artifact,
};

fn artifact_path() -> PathBuf {
    PathBuf::from(CURRENT_SYNTHESIS_STATE_ARTIFACT_PATH)
}

fn main() -> io::Result<()> {
    let current_synthesis_base = read_artifact(Path::new(CURRENT_SYNTHESIS_BASE_ARTIFACT_PATH))?;
    let artifact_index = load_artifact_index(Path::new(ARTIFACT_INDEX_PATH))?;
    let current_synthesis_state =
        build_current_synthesis_state_from_artifacts(&current_synthesis_base, &artifact_index);
    let artifact_path = artifact_path();

    write_artifact(&artifact_path, &current_synthesis_state)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::current_synthesis_support::{
        build_current_synthesis_state_from_artifacts, write_artifact,
    };

    #[test]
    fn current_synthesis_state_reads_existing_artifacts() {
        let current_synthesis_base = "# Current Synthesis Base\n\nbase";
        let artifact_index = "# Artifact Index\n\nindex";

        assert_eq!(
            build_current_synthesis_state_from_artifacts(current_synthesis_base, artifact_index),
            "# Current Synthesis State\n\n\
             ## Current Mode\n\n\
             Current Synthesis Base\n\n\
             ## Available Artifacts\n\n\
             - `artifacts/kernel_pass_snapshot.json`\n\
             - `artifacts/consumer_prompt.md`\n\
             - `artifacts/desktop_status.txt`\n\
             - `artifacts/current_synthesis_base.md`\n\
             - `artifacts/current_synthesis_state.md`\n\
             - `artifacts/current_synthesis_sequence.md`\n\
             - `artifacts/current_synthesis_topology.md`\n\
             - `artifacts/current_synthesis_clients.md`\n\
             - `artifacts/current_synthesis_choice.md`\n\
             - `artifacts/current_synthesis_contract.md`\n\
             - `artifacts/current_synthesis_preview.md`\n\
             - `artifacts/current_synthesis_operational.md`\n\
             - `artifacts/current_synthesis_selection.md`\n\
             - `artifacts/current_synthesis_consequence.md`\n\
             - `artifacts/current_synthesis_readiness.md`\n\
             - `artifacts/current_synthesis_execution_spec.md`\n\
             - `artifacts/current_synthesis_behavior_rules.md`\n\
             - `artifacts/current_synthesis_transition_pm_to_le.md`\n\
             - `artifacts/current_synthesis_collision_relay.md`\n\
             - `artifacts/current_synthesis_activation_gate.md`\n\
             - `artifacts/index.md`\n\n\
             ## Active Clients\n\n\
             - `current_synthesis_base`\n\
             - `current_synthesis_state`\n\
             - `current_synthesis_sequence`\n\
             - `current_synthesis_topology`\n\
             - `current_synthesis_clients`\n\
             - `current_synthesis_choice`\n\
             - `current_synthesis_contract`\n\
             - `current_synthesis_preview`\n\
             - `current_synthesis_operational`\n\
             - `current_synthesis_selection`\n\
             - `current_synthesis_consequence`\n\
             - `current_synthesis_readiness`\n\
             - `current_synthesis_execution_spec`\n\
             - `current_synthesis_behavior_rules`\n\
             - `current_synthesis_transition_pm_to_le`\n\
             - `current_synthesis_collision_relay`\n\
             - `current_synthesis_activation_gate`\n\
             - `current_synthesis`\n\n\
             ## Next Possible Action\n\n\
             Keep the route preview, operational view, selection, consequence, readiness, execution spec, behavior rules, transition rule, collision relay, and activation gate read-only until Current Synthesis is explicitly activated for behavior.\n\n\
             ## Deferred Status\n\n\
             - HAL status: deferred\n\
             - Clouseau status: deferred\n\
             - `PLEB`/`META` execution: deferred\n\
             - `niri`/`river` integration: deferred\n\n\
             ## Artifact Inputs\n\n\
             Current Synthesis base bytes: 30.\n\
             Artifact index bytes: 23.\n\n\
             ## Boundary Reminder\n\n\
             Current Synthesis consumes Hollow Grove; Hollow Grove does not know Current Synthesis exists.\n"
        );
    }

    #[test]
    fn current_synthesis_state_writes_a_deterministic_file() {
        let current_synthesis_state = build_current_synthesis_state_from_artifacts("base", "index");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir = std::env::temp_dir().join(format!("current-synthesis-state-{nonce}"));
        let artifact_path = artifact_dir.join("current_synthesis_state.md");

        write_artifact(&artifact_path, &current_synthesis_state)
            .expect("current synthesis state artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path)
                .expect("current synthesis state artifact should be readable"),
            current_synthesis_state
        );

        fs::remove_file(&artifact_path)
            .expect("current synthesis state artifact should be removable");
        fs::remove_dir(&artifact_dir)
            .expect("current synthesis state directory should be removable");
    }
}
