use std::io;
use std::path::{Path, PathBuf};

#[path = "../current_synthesis_support.rs"]
mod current_synthesis_support;

use current_synthesis_support::{
    CURRENT_SYNTHESIS_ACTIVATION_GATE_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_COLLISION_RELAY_ARTIFACT_PATH, CURRENT_SYNTHESIS_READINESS_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_TRANSITION_PM_TO_LE_ARTIFACT_PATH,
    build_current_synthesis_activation_gate_from_artifacts, read_artifact, write_artifact,
};

fn artifact_path() -> PathBuf {
    PathBuf::from(CURRENT_SYNTHESIS_ACTIVATION_GATE_ARTIFACT_PATH)
}

fn main() -> io::Result<()> {
    let current_synthesis_transition_pm_to_le = read_artifact(Path::new(
        CURRENT_SYNTHESIS_TRANSITION_PM_TO_LE_ARTIFACT_PATH,
    ))?;
    let current_synthesis_collision_relay =
        read_artifact(Path::new(CURRENT_SYNTHESIS_COLLISION_RELAY_ARTIFACT_PATH))?;
    let current_synthesis_readiness =
        read_artifact(Path::new(CURRENT_SYNTHESIS_READINESS_ARTIFACT_PATH))?;
    let current_synthesis_activation_gate = build_current_synthesis_activation_gate_from_artifacts(
        &current_synthesis_transition_pm_to_le,
        &current_synthesis_collision_relay,
        &current_synthesis_readiness,
    );
    let artifact_path = artifact_path();

    write_artifact(&artifact_path, &current_synthesis_activation_gate)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::current_synthesis_support::{
        build_current_synthesis_activation_gate_from_artifacts, write_artifact,
    };

    #[test]
    fn current_synthesis_activation_gate_reads_existing_artifacts() {
        let current_synthesis_transition_pm_to_le =
            "# Current Synthesis Transition Rule `P/M -> L/E`\n\nrule";
        let current_synthesis_collision_relay = "# Current Synthesis Collision Relay\n\nrelay";
        let current_synthesis_readiness = "# Current Synthesis Readiness\n\nreadiness";

        assert_eq!(
            build_current_synthesis_activation_gate_from_artifacts(
                current_synthesis_transition_pm_to_le,
                current_synthesis_collision_relay,
                current_synthesis_readiness
            ),
            "# Current Synthesis Activation Gate\n\n\
             ## Gate Result\n\n\
             - activation denied\n\
             - Current Synthesis remains read-only\n\n\
             ## Reason\n\n\
             - the `P/M -> L/E` transition rule is defined but not active\n\
             - the HAL/Cleo collision relay is defined but not enabled\n\
             - readiness confirms route behavior is not enabled\n\
             - HAL automation is not enabled\n\
             - Clouseau live interpretation is not enabled\n\
             - Cleo underground observation is not enabled\n\
             - runtime state has not been introduced\n\n\
             ## Allowed Now\n\n\
             - deterministic artifact generation\n\
             - boundary documentation\n\
             - read-only evaluation of Current Synthesis structure\n\n\
             ## Not Allowed Now\n\n\
             - route traversal\n\
             - route movement\n\
             - HAL automation\n\
             - Clouseau live interpretation\n\
             - Cleo underground observation\n\
             - HAL/Cleo collision relay\n\
             - feedback into Hollow Grove\n\n\
             ## Artifact Inputs\n\n\
             Current Synthesis transition rule bytes: 54.\n\
             Current Synthesis collision relay bytes: 42.\n\
             Current Synthesis readiness bytes: 40.\n\n\
             ## Boundary Reminder\n\n\
             Activation gating belongs to Current Synthesis. Hollow Grove remains unchanged.\n"
        );
    }

    #[test]
    fn current_synthesis_activation_gate_writes_a_deterministic_file() {
        let current_synthesis_activation_gate =
            build_current_synthesis_activation_gate_from_artifacts("spec", "relay", "readiness");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir =
            std::env::temp_dir().join(format!("current-synthesis-activation-gate-{nonce}"));
        let artifact_path = artifact_dir.join("current_synthesis_activation_gate.md");

        write_artifact(&artifact_path, &current_synthesis_activation_gate)
            .expect("current synthesis activation gate artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path)
                .expect("current synthesis activation gate artifact should be readable"),
            current_synthesis_activation_gate
        );

        fs::remove_file(&artifact_path)
            .expect("current synthesis activation gate artifact should be removable");
        fs::remove_dir(&artifact_dir)
            .expect("current synthesis activation gate directory should be removable");
    }
}
