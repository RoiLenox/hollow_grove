use std::io;
use std::path::Path;

use hollow_grove::hueman_support::{
    CURRENT_SYNTHESIS_ACTIVATION_GATE_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_BEHAVIOR_RULES_ARTIFACT_PATH, CURRENT_SYNTHESIS_CONSEQUENCE_ARTIFACT_PATH,
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
        assert_eq!(
            build_hueman_procedural_uplift_from_artifacts(
                "execution",
                "rules",
                "transition",
                "selection",
                "consequence",
                "gate",
                "stonebend",
                "tross",
                "glaushouse",
                "sandmanor"
            ),
            "# Hueman Procedural Uplift\n\n\
             ## Structural Rule\n\n\
             Hueman may lift procedural contracts from Current Synthesis into world-facing behavior surfaces without moving lower-layer ownership upward.\n\n\
             ## Shared Contract\n\n\
             - Current Synthesis still owns execution spec, behavior rules, transition rules, selection, consequence, and activation gating.\n\
             - Hueman consumes those lower contracts as kingdom-facing procedures.\n\
             - no uplifted procedure may mutate Hollow Grove or rewrite Current Synthesis ownership.\n\n\
             ## Stonebend Procedure\n\n\
             - Proliteriate, Hypergiant, and Freemason enter any civic decision as an equal-power triad.\n\
             - Hypergiant may present first as the public face, but may not finalize alone.\n\
             - Geralds provide the witnessed public body that confirms a civic shift without taking triad power.\n\
             - diamond extraction stays inside Stonebend's civic balance instead of becoming a private sovereign right.\n\
             - hollow current combines with diamond yield into mercury mirror under the same civic balance.\n\
             - until activation changes, Stonebend procedure remains declared rather than executed.\n\n\
             ## Flynt Procedure\n\n\
             - Tross holds the Flynt line from North -> South as the procedural spine.\n\
             - Juvenile checks the North head before Delinquent checks the South end.\n\
             - the four White Dwarfs keep close guard around Tross while Wardens hold the broader line body.\n\
             - opal extraction follows the guarded line body rather than an unbounded field claim.\n\
             - hollow current carries opal yield outward as Opal Oil without breaking the guarded line body.\n\
             - transition pressure may be read through Current Synthesis route order, but no autonomous traversal is enabled.\n\n\
             ## Glaushouse Procedure\n\n\
             - Prima Donna sets command tone and first issuance.\n\
             - Persephone relays or inherits continuity when command steps down.\n\
             - Nightengales run the care loop and stabilize the common body without taking sovereign lead.\n\
             - current combines with jade extraction into Jadomer without displacing Nightengales care duty.\n\
             - succession and care remain procedurally specified but still gated.\n\n\
             ## Sandmanor Procedure\n\n\
             - selection identifies the rival public frame and consequence names the witnessed improvement result.\n\
             - a Minoan winner takes Sandmanite; a Minorian winner takes Sandmanorian.\n\
             - Sandmen bind the crowd witness that legitimizes the Sandman office.\n\
             - current combines with crystal extraction into Crystoleum as part of the stewarded public export both rival houses must carry.\n\
             - reciprocal teaching remains the basis of rule instead of inheritance.\n\n\
             ## Activation Status\n\n\
             - procedural uplift is defined\n\
             - Current Synthesis activation still denies live execution\n\
             - no autonomous NPC state, contest loop, care loop, or guard traversal is active\n\
             - no feedback into Current Synthesis\n\
             - no feedback into Hollow Grove\n\n\
             ## Artifact Inputs\n\n\
             Current Synthesis execution spec bytes: 9.\n\
             Current Synthesis behavior rules bytes: 5.\n\
             Current Synthesis transition bytes: 10.\n\
             Current Synthesis selection bytes: 9.\n\
             Current Synthesis consequence bytes: 11.\n\
             Current Synthesis activation gate bytes: 4.\n\
             Hueman Stonebend Roles bytes: 9.\n\
             Hueman Tross Helpers bytes: 5.\n\
             Hueman Glaushouse Roles bytes: 10.\n\
             Hueman Sandmanor Roles bytes: 9.\n\n\
             ## Boundary Reminder\n\n\
             Procedural uplift makes Hueman ready to consume lower-layer procedures. It does not activate those procedures or grant Hueman authority over Current Synthesis.\n"
        );
    }

    #[test]
    fn hueman_procedural_uplift_writes_a_deterministic_file() {
        let hueman_procedural_uplift = build_hueman_procedural_uplift_from_artifacts(
            "execution",
            "rules",
            "transition",
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
