use std::io;
use std::path::Path;

use hollow_grove::hueman_support::{
    HUEMAN_LINK_PHYSICS_ARTIFACT_PATH, HUEMAN_SCENE_INTENT_ARTIFACT_PATH,
    build_hueman_scene_drift_from_artifacts, hueman_scene_drift_artifact_path,
};
use hollow_grove::{read_text_artifact, write_text_artifact};

fn main() -> io::Result<()> {
    let hueman_scene_intent = read_text_artifact(Path::new(HUEMAN_SCENE_INTENT_ARTIFACT_PATH))?;
    let hueman_link_physics = read_text_artifact(Path::new(HUEMAN_LINK_PHYSICS_ARTIFACT_PATH))?;
    let hueman_scene_drift =
        build_hueman_scene_drift_from_artifacts(&hueman_scene_intent, &hueman_link_physics);
    let artifact_path = hueman_scene_drift_artifact_path();

    write_text_artifact(&artifact_path, &hueman_scene_drift)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use hollow_grove::hueman_support::build_hueman_scene_drift_from_artifacts;
    use hollow_grove::write_text_artifact;

    #[test]
    fn hueman_scene_drift_reads_existing_artifacts() {
        assert_eq!(
            build_hueman_scene_drift_from_artifacts("intent", "physics"),
            "# Hueman Scene Drift\n\n\
             ## Structural Rule\n\n\
             Scene intent can drift into a different scene type when bias pressure persists over time without a full system resolving it.\n\n\
             ## Drift Vectors\n\n\
             ### Seam Market\n\n\
             - drifts toward Pressure Shelter when exchange slows and stored continuity takes over\n\
             - drifts toward Threshold Weather when structures fail and exposure outruns arrangement\n\n\
             ### Threshold Weather\n\n\
             - drifts toward Split Trace when warning persists without settlement\n\
             - drifts toward Seam Market when repeated crossings stabilize the edge into exchange\n\n\
             ### Pressure Shelter\n\n\
             - drifts toward Seam Market when guarded stores reopen into circulation\n\
             - drifts toward Split Trace when shelter empties and only residue remains\n\n\
             ### Split Trace\n\n\
             - drifts toward Threshold Weather when ambiguity spills outward into exposure\n\
             - drifts toward Pressure Shelter when traces are hoarded, muffled, or enclosed\n\n\
             ## Drift Drivers\n\n\
             - sustained `current` accumulation pulls scenes toward storage, continuity, and reopened exchange\n\
             - sustained `aura` accumulation pulls scenes toward exposure, drift, shimmer, and unstable edges\n\
             - mixed unresolved pressure preserves Split Trace longer instead of forcing a clean resolution\n\
             - repeated crossings can stabilize a scene back into exchange even after warning or ambiguity\n\n\
             ## Status\n\n\
             - scene drift is descriptive-only for now\n\
             - no time simulation or procedural resolver is active\n\
             - scene intent remains the upstream atmospheric layer\n\
             - link physics remains the upstream bias layer\n\
             - no feedback into Current Synthesis\n\
             - no feedback into Hollow Grove\n\n\
             ## Artifact Inputs\n\n\
             Hueman Scene Intent bytes: 6.\n\
             Hueman Link Physics bytes: 7.\n\n\
             ## Boundary Reminder\n\n\
             Scene drift says how a scene may change if its pressure persists. It does not activate clocks, AI routines, or procedural world updates.\n"
        );
    }

    #[test]
    fn hueman_scene_drift_writes_a_deterministic_file() {
        let hueman_scene_drift = build_hueman_scene_drift_from_artifacts("intent", "physics");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir = std::env::temp_dir().join(format!("hueman-scene-drift-{nonce}"));
        let artifact_path = artifact_dir.join("hueman_scene_drift.md");

        write_text_artifact(&artifact_path, &hueman_scene_drift)
            .expect("hueman scene drift artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path).expect("hueman scene drift artifact should read"),
            hueman_scene_drift
        );

        fs::remove_file(&artifact_path).expect("hueman scene drift artifact should be removable");
        fs::remove_dir(&artifact_dir).expect("hueman scene drift directory should be removable");
    }
}
