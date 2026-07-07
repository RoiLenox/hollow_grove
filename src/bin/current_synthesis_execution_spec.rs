use std::io;
use std::path::{Path, PathBuf};

#[path = "../current_synthesis_support.rs"]
mod current_synthesis_support;

use current_synthesis_support::{
    CURRENT_SYNTHESIS_CONSEQUENCE_ARTIFACT_PATH, CURRENT_SYNTHESIS_EXECUTION_SPEC_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_READINESS_ARTIFACT_PATH,
    build_current_synthesis_execution_spec_from_artifacts, read_artifact, write_artifact,
};

fn artifact_path() -> PathBuf {
    PathBuf::from(CURRENT_SYNTHESIS_EXECUTION_SPEC_ARTIFACT_PATH)
}

fn main() -> io::Result<()> {
    let current_synthesis_readiness =
        read_artifact(Path::new(CURRENT_SYNTHESIS_READINESS_ARTIFACT_PATH))?;
    let current_synthesis_consequence =
        read_artifact(Path::new(CURRENT_SYNTHESIS_CONSEQUENCE_ARTIFACT_PATH))?;
    let current_synthesis_execution_spec = build_current_synthesis_execution_spec_from_artifacts(
        &current_synthesis_readiness,
        &current_synthesis_consequence,
    );
    let artifact_path = artifact_path();

    write_artifact(&artifact_path, &current_synthesis_execution_spec)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::current_synthesis_support::{
        build_current_synthesis_execution_spec_from_artifacts, write_artifact,
    };

    #[test]
    fn current_synthesis_execution_spec_reads_existing_artifacts() {
        let current_synthesis_readiness = "# Current Synthesis Readiness\n\nreadiness";
        let current_synthesis_consequence = "# Current Synthesis Consequence\n\nconsequence";

        assert_eq!(
            build_current_synthesis_execution_spec_from_artifacts(
                current_synthesis_readiness,
                current_synthesis_consequence
            ),
            "# Current Synthesis Execution Spec\n\n\
             ## Preconditions\n\n\
             - route execution rules must be defined explicitly\n\
             - `PLEB` and `META` behavior must be specified explicitly\n\
             - runtime state must be introduced deliberately\n\
             - HAL permissions must be declared before automation\n\
             - Clouseau live interpretation rules must be declared before observation\n\n\
             ## `PLEB` Execution Would Need\n\n\
             - a defined `PLEB` route step model\n\
             - a defined transition rule between joints\n\
             - a defined boundary for clue production\n\n\
             ## `META` Execution Would Need\n\n\
             - a defined `META` route step model\n\
             - a defined watch or traversal rule between joints\n\
             - a defined boundary for complementary occupancy\n\n\
             ## HAL Would Need Before Automation\n\n\
             - explicit automation scope\n\
             - explicit allowed actions\n\
             - explicit prohibition on Hollow Grove mutation\n\n\
             ## Clouseau Would Need Before Live Interpretation\n\n\
             - explicit observation scope\n\
             - explicit clue or residue inputs\n\
             - explicit prohibition on control or automation\n\n\
             ## Activation Status\n\n\
             - not active\n\
             - no traversal\n\
             - no movement\n\
             - no automation\n\
             - no live interpretation\n\
             - no runtime state\n\n\
             ## Artifact Inputs\n\n\
             Current Synthesis readiness bytes: 40.\n\
             Current Synthesis consequence bytes: 44.\n\n\
             ## Boundary Reminder\n\n\
             Execution spec belongs to Current Synthesis. Hollow Grove remains unchanged.\n"
        );
    }

    #[test]
    fn current_synthesis_execution_spec_writes_a_deterministic_file() {
        let current_synthesis_execution_spec =
            build_current_synthesis_execution_spec_from_artifacts("readiness", "consequence");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir =
            std::env::temp_dir().join(format!("current-synthesis-execution-spec-{nonce}"));
        let artifact_path = artifact_dir.join("current_synthesis_execution_spec.md");

        write_artifact(&artifact_path, &current_synthesis_execution_spec)
            .expect("current synthesis execution spec artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path)
                .expect("current synthesis execution spec artifact should be readable"),
            current_synthesis_execution_spec
        );

        fs::remove_file(&artifact_path)
            .expect("current synthesis execution spec artifact should be removable");
        fs::remove_dir(&artifact_dir)
            .expect("current synthesis execution spec directory should be removable");
    }
}
