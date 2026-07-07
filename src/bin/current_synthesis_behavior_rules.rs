use std::io;
use std::path::{Path, PathBuf};

#[path = "../current_synthesis_support.rs"]
mod current_synthesis_support;

use current_synthesis_support::{
    CURRENT_SYNTHESIS_BEHAVIOR_RULES_ARTIFACT_PATH, CURRENT_SYNTHESIS_EXECUTION_SPEC_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_SELECTION_ARTIFACT_PATH,
    build_current_synthesis_behavior_rules_from_artifacts, read_artifact, write_artifact,
};

fn artifact_path() -> PathBuf {
    PathBuf::from(CURRENT_SYNTHESIS_BEHAVIOR_RULES_ARTIFACT_PATH)
}

fn main() -> io::Result<()> {
    let current_synthesis_execution_spec =
        read_artifact(Path::new(CURRENT_SYNTHESIS_EXECUTION_SPEC_ARTIFACT_PATH))?;
    let current_synthesis_selection =
        read_artifact(Path::new(CURRENT_SYNTHESIS_SELECTION_ARTIFACT_PATH))?;
    let current_synthesis_behavior_rules = build_current_synthesis_behavior_rules_from_artifacts(
        &current_synthesis_execution_spec,
        &current_synthesis_selection,
    );
    let artifact_path = artifact_path();

    write_artifact(&artifact_path, &current_synthesis_behavior_rules)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::current_synthesis_support::{
        build_current_synthesis_behavior_rules_from_artifacts, write_artifact,
    };

    #[test]
    fn current_synthesis_behavior_rules_reads_existing_artifacts() {
        let current_synthesis_execution_spec = "# Current Synthesis Execution Spec\n\nspec";
        let current_synthesis_selection = "# Current Synthesis Selection\n\nselection";

        assert_eq!(
            build_current_synthesis_behavior_rules_from_artifacts(
                current_synthesis_execution_spec,
                current_synthesis_selection
            ),
            "# Current Synthesis Behavior Rules\n\n\
             ## Rule 1: Occupancy\n\n\
             - the selected side remains occupied by its assigned client\n\
             - the complementary side remains occupied by its assigned client\n\n\
             ## Rule 2: Joint Order\n\n\
             - any future route behavior must follow `P/M -> L/E -> E/T -> B/A`\n\
             - no joint may be skipped\n\n\
             ## Rule 3: `PLEB`\n\n\
             - `PLEB` remains the straight-side route context\n\
             - Clouseau remains the `PLEB` client\n\
             - clue production stays within Current Synthesis\n\n\
             ## Rule 4: `META`\n\n\
             - `META` remains the bent-side route context\n\
             - HAL remains the `META` client\n\
             - complementary occupancy stays within Current Synthesis\n\n\
             ## Rule 5: HAL Scope\n\n\
             - HAL may act only within explicit Current Synthesis permissions\n\
             - HAL never mutates Hollow Grove\n\
\n\
             ## Rule 6: Clouseau Scope\n\n\
             - Clouseau may interpret only within explicit Current Synthesis permissions\n\
             - Clouseau never controls route execution\n\
\n\
             ## Activation Status\n\n\
             - rules defined\n\
             - not active\n\
             - no traversal\n\
             - no movement\n\
             - no automation\n\
             - no live interpretation\n\
             - no runtime state\n\n\
             ## Artifact Inputs\n\n\
             Current Synthesis execution spec bytes: 40.\n\
             Current Synthesis selection bytes: 40.\n\n\
             ## Boundary Reminder\n\n\
             Behavior rules belong to Current Synthesis. Hollow Grove remains unchanged.\n"
        );
    }

    #[test]
    fn current_synthesis_behavior_rules_writes_a_deterministic_file() {
        let current_synthesis_behavior_rules =
            build_current_synthesis_behavior_rules_from_artifacts("spec", "selection");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir =
            std::env::temp_dir().join(format!("current-synthesis-behavior-rules-{nonce}"));
        let artifact_path = artifact_dir.join("current_synthesis_behavior_rules.md");

        write_artifact(&artifact_path, &current_synthesis_behavior_rules)
            .expect("current synthesis behavior rules artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path)
                .expect("current synthesis behavior rules artifact should be readable"),
            current_synthesis_behavior_rules
        );

        fs::remove_file(&artifact_path)
            .expect("current synthesis behavior rules artifact should be removable");
        fs::remove_dir(&artifact_dir)
            .expect("current synthesis behavior rules directory should be removable");
    }
}
