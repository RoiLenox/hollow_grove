use std::io;
use std::path::Path;

use hollow_grove::hueman_support::{
    CURRENT_SYNTHESIS_TOPOLOGY_ARTIFACT_PATH, HUEMAN_FOURWAY_ARTIFACT_PATH,
    build_hueman_aura_triad_from_artifacts, hueman_aura_triad_artifact_path,
};
use hollow_grove::{read_text_artifact, write_text_artifact};

fn main() -> io::Result<()> {
    let hueman_fourway = read_text_artifact(Path::new(HUEMAN_FOURWAY_ARTIFACT_PATH))?;
    let current_synthesis_topology =
        read_text_artifact(Path::new(CURRENT_SYNTHESIS_TOPOLOGY_ARTIFACT_PATH))?;
    let hueman_aura_triad =
        build_hueman_aura_triad_from_artifacts(&hueman_fourway, &current_synthesis_topology);
    let artifact_path = hueman_aura_triad_artifact_path();

    write_text_artifact(&artifact_path, &hueman_aura_triad)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use hollow_grove::hueman_support::build_hueman_aura_triad_from_artifacts;
    use hollow_grove::write_text_artifact;

    #[test]
    fn hueman_aura_triad_reads_existing_artifacts() {
        assert_eq!(
            build_hueman_aura_triad_from_artifacts("fourway", "topology"),
            "# Hueman Aura Triad\n\n\
             ## Structural Rule\n\n\
             AuraTriad is the three-region resolution beneath Fourway and above Triway.\n\n\
             ## Stack\n\n\
             ```text\n\
             Hueman\n\
             ↓\n\
             Fourway\n\
             ↓\n\
             AuraTriad\n\
             ↓\n\
             Triway\n\
             ↓\n\
             Hollow Grove\n\
             ```\n\n\
             ## Triad\n\n\
             ```text\n\
             Aura Basin\n\
             ↓\n\
             Aura Fields\n\
             ↓\n\
             Aura Beach\n\
             ```\n\n\
             ## Meaning\n\n\
             - AuraTriad is the world-facing three-region route body beneath Fourway.\n\
             - Current Synthesis already records these as inverse-route regions.\n\
             - Hueman reads them as the triadic resolution of the world map.\n\
             - Triway remains the lower recursive split after this layer.\n\n\
             ## PLEB and META\n\n\
             - `PLEB` and `META` remain Current Synthesis occupancy semantics.\n\
             - AuraTriad does not move `PLEB` or `META` into the kernel.\n\
             - AuraTriad does not redefine Triway.\n\n\
             ## Boundary\n\n\
             - AuraTriad belongs to Hueman as world reading.\n\
             - the source geography remains readable from Current Synthesis.\n\
             - no feedback into Current Synthesis\n\
             - no feedback into Hollow Grove\n\n\
             ## Artifact Inputs\n\n\
             Hueman Fourway bytes: 7.\n\
             Current Synthesis topology bytes: 8.\n\n\
             ## Boundary Reminder\n\n\
             AuraTriad is the bridge between Hueman Fourway and Hollow Grove Triway. It is not itself a kernel structure.\n"
        );
    }

    #[test]
    fn hueman_aura_triad_writes_a_deterministic_file() {
        let hueman_aura_triad = build_hueman_aura_triad_from_artifacts("fourway", "topology");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir = std::env::temp_dir().join(format!("hueman-aura-triad-{nonce}"));
        let artifact_path = artifact_dir.join("hueman_aura_triad.md");

        write_text_artifact(&artifact_path, &hueman_aura_triad)
            .expect("hueman aura triad artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path).expect("hueman aura triad artifact should read"),
            hueman_aura_triad
        );

        fs::remove_file(&artifact_path).expect("hueman aura triad artifact should be removable");
        fs::remove_dir(&artifact_dir).expect("hueman aura triad directory should be removable");
    }
}
