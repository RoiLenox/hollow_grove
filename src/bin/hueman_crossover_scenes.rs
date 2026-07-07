use std::io;
use std::path::Path;

use hollow_grove::hueman_support::{
    HUEMAN_LINK_PHYSICS_ARTIFACT_PATH, HUEMAN_PATH_CROSSOVERS_ARTIFACT_PATH,
    build_hueman_crossover_scenes_from_artifacts, hueman_crossover_scenes_artifact_path,
};
use hollow_grove::{read_text_artifact, write_text_artifact};

fn main() -> io::Result<()> {
    let hueman_path_crossovers =
        read_text_artifact(Path::new(HUEMAN_PATH_CROSSOVERS_ARTIFACT_PATH))?;
    let hueman_link_physics =
        read_text_artifact(Path::new(HUEMAN_LINK_PHYSICS_ARTIFACT_PATH))?;
    let hueman_crossover_scenes = build_hueman_crossover_scenes_from_artifacts(
        &hueman_path_crossovers,
        &hueman_link_physics,
    );
    let artifact_path = hueman_crossover_scenes_artifact_path();

    write_text_artifact(&artifact_path, &hueman_crossover_scenes)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use hollow_grove::hueman_support::build_hueman_crossover_scenes_from_artifacts;
    use hollow_grove::write_text_artifact;

    #[test]
    fn hueman_crossover_scenes_reads_existing_artifacts() {
        assert_eq!(
            build_hueman_crossover_scenes_from_artifacts("cross", "physics"),
            "# Hueman Crossover Scenes\n\n\
             ## Structural Rule\n\n\
             When `current`-biased and `aura`-biased unresolved links appear at the same crossover, the world produces a named descriptive scene type.\n\n\
             ## Scene Types\n\n\
             ### Seam Market\n\n\
             - appears where `current` continuity and `aura` spill remain in balance\n\
             - feels like trade, rumor, salvage, and temporary arrangement\n\
             - fits shared Aura Fields crossings best\n\n\
             ### Threshold Weather\n\n\
             - appears where `aura` exposure outruns `current` continuity\n\
             - feels like spray, drift, shimmer, and unstable edges\n\
             - fits shared Aura Beach crossings best\n\n\
             ### Pressure Shelter\n\n\
             - appears where `current` persistence contains `aura` residue\n\
             - feels like storage, burrow heat, muffled exchange, and held tension\n\
             - fits shared Aura Basin crossings best\n\n\
             ### Split Trace\n\n\
             - appears where both biases are present but neither settles the scene\n\
             - feels like afterimage, contradictory clues, and route ambiguity\n\
             - fits delayed or secondary crossovers after different openings\n\n\
             ## Placement\n\n\
             - Aura Beach tends toward Threshold Weather first.\n\
             - Aura Fields tends toward Seam Market first.\n\
             - Aura Basin tends toward Pressure Shelter first.\n\
             - Split Trace can appear in any crossover zone where the bias remains unresolved.\n\n\
             ## Status\n\n\
             - crossover scenes are descriptive-only for now\n\
             - no encounter tables or event resolvers are active\n\
             - link physics remains the upstream explanation for the scene type\n\
             - path crossovers remain the upstream overlap map\n\
             - no feedback into Current Synthesis\n\
             - no feedback into Hollow Grove\n\n\
             ## Artifact Inputs\n\n\
             Hueman Path Crossovers bytes: 5.\n\
             Hueman Link Physics bytes: 7.\n\n\
             ## Boundary Reminder\n\n\
             Crossover scenes name what a shared biased overlap feels like. They do not create procedural meetings, rewards, or movement rules.\n"
        );
    }

    #[test]
    fn hueman_crossover_scenes_writes_a_deterministic_file() {
        let hueman_crossover_scenes =
            build_hueman_crossover_scenes_from_artifacts("cross", "physics");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir =
            std::env::temp_dir().join(format!("hueman-crossover-scenes-{nonce}"));
        let artifact_path = artifact_dir.join("hueman_crossover_scenes.md");

        write_text_artifact(&artifact_path, &hueman_crossover_scenes)
            .expect("hueman crossover scenes artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path)
                .expect("hueman crossover scenes artifact should read"),
            hueman_crossover_scenes
        );

        fs::remove_file(&artifact_path)
            .expect("hueman crossover scenes artifact should be removable");
        fs::remove_dir(&artifact_dir)
            .expect("hueman crossover scenes directory should be removable");
    }
}
