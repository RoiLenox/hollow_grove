use std::io;
use std::path::Path;

use hollow_grove::hueman_support::{
    CURRENT_SYNTHESIS_ACTIVATION_GATE_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_BEHAVIOR_RULES_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_COLLISION_RELAY_ARTIFACT_PATH, CURRENT_SYNTHESIS_CONSEQUENCE_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_EXECUTION_SPEC_ARTIFACT_PATH, CURRENT_SYNTHESIS_SELECTION_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_TRANSITION_PM_TO_LE_ARTIFACT_PATH, HUEMAN_GLAUSHOUSE_ROLES_ARTIFACT_PATH,
    HUEMAN_SANDMANOR_ROLES_ARTIFACT_PATH, HUEMAN_STONEBEND_ROLES_ARTIFACT_PATH,
    HUEMAN_TROSS_HELPERS_ARTIFACT_PATH, build_hueman_procedural_uplift_from_artifacts,
    hueman_procedural_uplift_artifact_path,
};
use hollow_grove::{read_text_artifact, write_text_artifact};

fn main() -> io::Result<()> {
    let current_synthesis_execution_spec =
        read_text_artifact(Path::new(CURRENT_SYNTHESIS_EXECUTION_SPEC_ARTIFACT_PATH))?;
    let current_synthesis_behavior_rules =
        read_text_artifact(Path::new(CURRENT_SYNTHESIS_BEHAVIOR_RULES_ARTIFACT_PATH))?;
    let current_synthesis_transition_pm_to_le = read_text_artifact(Path::new(
        CURRENT_SYNTHESIS_TRANSITION_PM_TO_LE_ARTIFACT_PATH,
    ))?;
    let current_synthesis_collision_relay =
        read_text_artifact(Path::new(CURRENT_SYNTHESIS_COLLISION_RELAY_ARTIFACT_PATH))?;
    let current_synthesis_selection =
        read_text_artifact(Path::new(CURRENT_SYNTHESIS_SELECTION_ARTIFACT_PATH))?;
    let current_synthesis_consequence =
        read_text_artifact(Path::new(CURRENT_SYNTHESIS_CONSEQUENCE_ARTIFACT_PATH))?;
    let current_synthesis_activation_gate =
        read_text_artifact(Path::new(CURRENT_SYNTHESIS_ACTIVATION_GATE_ARTIFACT_PATH))?;
    let hueman_stonebend_roles =
        read_text_artifact(Path::new(HUEMAN_STONEBEND_ROLES_ARTIFACT_PATH))?;
    let hueman_tross_helpers = read_text_artifact(Path::new(HUEMAN_TROSS_HELPERS_ARTIFACT_PATH))?;
    let hueman_glaushouse_roles =
        read_text_artifact(Path::new(HUEMAN_GLAUSHOUSE_ROLES_ARTIFACT_PATH))?;
    let hueman_sandmanor_roles =
        read_text_artifact(Path::new(HUEMAN_SANDMANOR_ROLES_ARTIFACT_PATH))?;
    let hueman_procedural_uplift = build_hueman_procedural_uplift_from_artifacts(
        &current_synthesis_execution_spec,
        &current_synthesis_behavior_rules,
        &current_synthesis_transition_pm_to_le,
        &current_synthesis_collision_relay,
        &current_synthesis_selection,
        &current_synthesis_consequence,
        &current_synthesis_activation_gate,
        &hueman_stonebend_roles,
        &hueman_tross_helpers,
        &hueman_glaushouse_roles,
        &hueman_sandmanor_roles,
    );
    let artifact_path = hueman_procedural_uplift_artifact_path();

    write_text_artifact(&artifact_path, &hueman_procedural_uplift)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use hollow_grove::hueman_support::build_hueman_procedural_uplift_from_artifacts;
    use hollow_grove::write_text_artifact;

    #[test]
    fn hueman_procedural_uplift_reads_existing_artifacts() {
        let output = build_hueman_procedural_uplift_from_artifacts(
            "execution",
            "rules",
            "transition",
            "relay",
            "selection",
            "consequence",
            "gate",
            "stonebend",
            "tross",
            "glaushouse",
            "sandmanor",
        );
        assert!(output.contains("## Bottom-Up Procedure Spine"));
        assert!(output.contains("## Relay Procedure"));
        assert!(output.contains(
            "Flynt knowledge gates open through puzzle trails, treasure-hunt clues, and route memory"
        ));
        assert!(output.contains(
            "a Flynt contender gathers ingredients in the field but still has to pass through Glaushouse synthesis"
        ));
        assert!(
            output.contains(
                "Chimera is the first true recombination body of those mastered branches"
            )
        );
        assert!(output.contains("Current Synthesis collision relay bytes: 5."));
    }

    #[test]
    fn hueman_procedural_uplift_writes_a_deterministic_file() {
        let hueman_procedural_uplift = build_hueman_procedural_uplift_from_artifacts(
            "execution",
            "rules",
            "transition",
            "relay",
            "selection",
            "consequence",
            "gate",
            "stonebend",
            "tross",
            "glaushouse",
            "sandmanor",
        );
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir = std::env::temp_dir().join(format!("hueman-procedural-uplift-{nonce}"));
        let artifact_path = artifact_dir.join("hueman_procedural_uplift.md");

        write_text_artifact(&artifact_path, &hueman_procedural_uplift)
            .expect("hueman procedural uplift artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path)
                .expect("hueman procedural uplift artifact should read"),
            hueman_procedural_uplift
        );

        fs::remove_file(&artifact_path)
            .expect("hueman procedural uplift artifact should be removable");
        fs::remove_dir(&artifact_dir)
            .expect("hueman procedural uplift directory should be removable");
    }
}
