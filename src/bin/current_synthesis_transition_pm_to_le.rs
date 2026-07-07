use std::io;
use std::path::{Path, PathBuf};

#[path = "../current_synthesis_support.rs"]
mod current_synthesis_support;

use current_synthesis_support::{
    CURRENT_SYNTHESIS_BEHAVIOR_RULES_ARTIFACT_PATH, CURRENT_SYNTHESIS_TOPOLOGY_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_TRANSITION_PM_TO_LE_ARTIFACT_PATH,
    build_current_synthesis_transition_pm_to_le_from_artifacts, read_artifact, write_artifact,
};

fn artifact_path() -> PathBuf {
    PathBuf::from(CURRENT_SYNTHESIS_TRANSITION_PM_TO_LE_ARTIFACT_PATH)
}

fn main() -> io::Result<()> {
    let current_synthesis_behavior_rules =
        read_artifact(Path::new(CURRENT_SYNTHESIS_BEHAVIOR_RULES_ARTIFACT_PATH))?;
    let current_synthesis_topology =
        read_artifact(Path::new(CURRENT_SYNTHESIS_TOPOLOGY_ARTIFACT_PATH))?;
    let current_synthesis_transition_pm_to_le =
        build_current_synthesis_transition_pm_to_le_from_artifacts(
            &current_synthesis_behavior_rules,
            &current_synthesis_topology,
        );
    let artifact_path = artifact_path();

    write_artifact(&artifact_path, &current_synthesis_transition_pm_to_le)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::current_synthesis_support::{
        build_current_synthesis_transition_pm_to_le_from_artifacts, write_artifact,
    };

    #[test]
    fn current_synthesis_transition_pm_to_le_reads_existing_artifacts() {
        let current_synthesis_behavior_rules = "# Current Synthesis Behavior Rules\n\nrules";
        let current_synthesis_topology = "# Current Synthesis Topology\n\ntopology";

        assert_eq!(
            build_current_synthesis_transition_pm_to_le_from_artifacts(
                current_synthesis_behavior_rules,
                current_synthesis_topology
            ),
            "# Current Synthesis Transition Rule `P/M -> L/E`\n\n\
             ## Transition Condition\n\n\
             - the joint order must remain `P/M -> L/E -> E/T -> B/A`\n\
             - `PLEB` and `META` occupancy must remain locked\n\
             - this rule remains declarative only\n\n\
             ## `PLEB` Occupancy\n\n\
             - Clouseau remains on `PLEB`\n\
             - straight-side occupancy carries from `P/M` to `L/E`\n\n\
             ## `META` Occupancy\n\n\
             - HAL remains on `META`\n\
             - bent-side occupancy carries from `P/M` to `L/E`\n\n\
             ## HAL Observation\n\n\
             - HAL may observe complementary alignment at `P/M` and `L/E`\n\
             - HAL may not automate movement\n\n\
             ## Clouseau Observation\n\n\
             - Clouseau may observe clue continuity at `P/M` and `L/E`\n\
             - Clouseau may not control movement\n\n\
             ## Still Forbidden\n\n\
             - route traversal\n\
             - route movement\n\
             - automation\n\
             - live interpretation\n\
             - runtime state\n\
             - feedback into Hollow Grove\n\n\
             ## Artifact Inputs\n\n\
             Current Synthesis behavior rules bytes: 41.\n\
             Current Synthesis topology bytes: 38.\n\n\
             ## Boundary Reminder\n\n\
             Transition rules belong to Current Synthesis. Hollow Grove remains unchanged.\n"
        );
    }

    #[test]
    fn current_synthesis_transition_pm_to_le_writes_a_deterministic_file() {
        let current_synthesis_transition_pm_to_le =
            build_current_synthesis_transition_pm_to_le_from_artifacts("rules", "topology");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir =
            std::env::temp_dir().join(format!("current-synthesis-transition-pm-to-le-{nonce}"));
        let artifact_path = artifact_dir.join("current_synthesis_transition_pm_to_le.md");

        write_artifact(&artifact_path, &current_synthesis_transition_pm_to_le)
            .expect("current synthesis transition rule artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path)
                .expect("current synthesis transition rule artifact should be readable"),
            current_synthesis_transition_pm_to_le
        );

        fs::remove_file(&artifact_path)
            .expect("current synthesis transition rule artifact should be removable");
        fs::remove_dir(&artifact_dir)
            .expect("current synthesis transition rule directory should be removable");
    }
}
