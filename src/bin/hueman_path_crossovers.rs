use std::io;
use std::path::Path;

use hollow_grove::hueman_support::{
    HUEMAN_AURA_BEHAVIOR_ARTIFACT_PATH, HUEMAN_START_PATHS_ARTIFACT_PATH,
    build_hueman_path_crossovers_from_artifacts, hueman_path_crossovers_artifact_path,
};
use hollow_grove::{read_text_artifact, write_text_artifact};

fn main() -> io::Result<()> {
    let hueman_start_paths = read_text_artifact(Path::new(HUEMAN_START_PATHS_ARTIFACT_PATH))?;
    let hueman_aura_behavior = read_text_artifact(Path::new(HUEMAN_AURA_BEHAVIOR_ARTIFACT_PATH))?;
    let hueman_path_crossovers =
        build_hueman_path_crossovers_from_artifacts(&hueman_start_paths, &hueman_aura_behavior);
    let artifact_path = hueman_path_crossovers_artifact_path();

    write_text_artifact(&artifact_path, &hueman_path_crossovers)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use hollow_grove::hueman_support::build_hueman_path_crossovers_from_artifacts;
    use hollow_grove::write_text_artifact;

    #[test]
    fn hueman_path_crossovers_reads_existing_artifacts() {
        assert_eq!(
            build_hueman_path_crossovers_from_artifacts("paths", "aura"),
            "# Hueman Path Crossovers\n\n\
             ## Structural Rule\n\n\
             Different starts may enter AuraTriad differently while still crossing through shared regions and shared world pressure.\n\n\
             ## Shared Entry Crossovers\n\n\
             - Glaushouse and Sandmanor cross immediately at Aura Beach.\n\
             - Flynt and Stonebend do not share first entry, but they both begin inland before reaching the coast.\n\n\
             ## Interior Crossovers\n\n\
             - Flynt and Sandmanor cross at Aura Basin.\n\
             - Stonebend and Glaushouse cross at Aura Fields.\n\
             - Stonebend and Sandmanor cross at Aura Basin after different openings.\n\n\
             ## Aura Ridge Trade Legs\n\n\
             - free trade follows the straight Aura Ridge rather than the underground inverse circle.\n\
             - Stonebend and Glaushouse hold a declared straight trade leg along the ridge.\n\
             - Glaushouse and Sandmanor hold a declared straight trade leg along the ridge.\n\
             - Glaushouse acts as the visible hinge where the right-angle trade body turns.\n\n\
             ## Full-Triad Convergence\n\n\
             - all four starts eventually touch Aura Basin\n\
             - all four starts eventually touch Aura Fields\n\
             - all four starts eventually touch Aura Beach\n\
             - the difference is order, not exclusion\n\n\
             ## Meaning\n\n\
             - crossover means the world can feel shared without erasing start identity\n\
             - shared regions carry different descriptive pressure depending on entry order\n\
             - the coast is the earliest common threshold for the western and southern starts\n\
             - inland turns remain the main crossover pressure for the northern and eastern starts\n\n\
             ## Status\n\n\
             - crossovers are descriptive-only for now\n\
             - no meeting mechanics or shared events are active\n\
             - start-path order remains unchanged\n\
             - archetype lens remains interpretive above the crossover map\n\
             - no feedback into Current Synthesis\n\
             - no feedback into Hollow Grove\n\n\
             ## Artifact Inputs\n\n\
             Hueman Start Paths bytes: 5.\n\
             Hueman Aura Behavior bytes: 4.\n\n\
             ## Boundary Reminder\n\n\
             Path crossovers declare where starts can meaningfully overlap in AuraTriad. They do not create procedural encounters or alter lower-layer routing.\n"
        );
    }

    #[test]
    fn hueman_path_crossovers_writes_a_deterministic_file() {
        let hueman_path_crossovers = build_hueman_path_crossovers_from_artifacts("paths", "aura");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir = std::env::temp_dir().join(format!("hueman-path-crossovers-{nonce}"));
        let artifact_path = artifact_dir.join("hueman_path_crossovers.md");

        write_text_artifact(&artifact_path, &hueman_path_crossovers)
            .expect("hueman path crossovers artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path)
                .expect("hueman path crossovers artifact should read"),
            hueman_path_crossovers
        );

        fs::remove_file(&artifact_path)
            .expect("hueman path crossovers artifact should be removable");
        fs::remove_dir(&artifact_dir)
            .expect("hueman path crossovers directory should be removable");
    }
}
