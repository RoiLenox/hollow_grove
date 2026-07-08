use std::io;
use std::path::Path;

use hollow_grove::hueman_support::{
    CURRENT_SYNTHESIS_SEQUENCE_ARTIFACT_PATH, HUEMAN_PATH_CROSSOVERS_ARTIFACT_PATH,
    build_hueman_link_physics_from_artifacts, hueman_link_physics_artifact_path,
};
use hollow_grove::{read_text_artifact, write_text_artifact};

fn main() -> io::Result<()> {
    let current_synthesis_sequence =
        read_text_artifact(Path::new(CURRENT_SYNTHESIS_SEQUENCE_ARTIFACT_PATH))?;
    let hueman_path_crossovers =
        read_text_artifact(Path::new(HUEMAN_PATH_CROSSOVERS_ARTIFACT_PATH))?;
    let hueman_link_physics = build_hueman_link_physics_from_artifacts(
        &current_synthesis_sequence,
        &hueman_path_crossovers,
    );
    let artifact_path = hueman_link_physics_artifact_path();

    write_text_artifact(&artifact_path, &hueman_link_physics)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use hollow_grove::hueman_support::build_hueman_link_physics_from_artifacts;
    use hollow_grove::write_text_artifact;

    #[test]
    fn hueman_link_physics_reads_existing_artifacts() {
        assert_eq!(
            build_hueman_link_physics_from_artifacts("sequence", "cross"),
            "# Hueman Link Physics\n\n\
             ## Structural Rule\n\n\
             Links that do not get bonded may later resolve into `current` or `aura` according to downstream physics.\n\n\
             ## Bond Split\n\n\
             - bonded link stays the selected route\n\
             - unbonded links remain available as unresolved world material\n\
             - unresolved material is not empty; it carries later directional bias\n\n\
             ## Arm Weight Reading\n\n\
             - each `META` letter and its `PLEB` counterpart carry three arms across the same joint\n\
             - one arm per side bonds into the selected link while the remaining arm weight stays unresolved\n\
             - retained heavier continuity pressure tends to read as `current`\n\
             - lighter exposed spill tends to read as `aura`\n\
             - Hueman reads that unresolved weight upward as `current` or `aura` while Hollow Grove keeps the same event as the lower witness simultaneously\n\
             - simultaneous reading does not grant Hueman authority to rewrite the kernel witness\n\n\
             ## Current Bias Physics\n\n\
             - continuity pressure favors `current`\n\
             - occupancy load favors `current`\n\
             - inland persistence favors `current`\n\
             - repeat traversal favors `current`\n\n\
             ## Aura Bias Physics\n\n\
             - exposure pressure favors `aura`\n\
             - threshold bleed favors `aura`\n\
             - atmospheric spill favors `aura`\n\
             - edge drift favors `aura`\n\n\
             ## Element Names\n\n\
             - `current` may also be called Bathos or dark water.\n\
             - `current` appears as dark current or hollow current.\n\
             - `aura` may also be called Aether or air.\n\
             - `aura` appears as reflective aura or holographic aura.\n\n\
             ## Crossover Reading\n\n\
             - shared starts can touch the same unresolved material with different bias\n\
             - the same region may feel more `current` from one route and more `aura` from another\n\
             - crossover zones are where the physics split becomes most visible in Hueman\n\
             - Aura Ridge trade legs keep exchange visible on straight lines while unresolved bias still moves beneath them\n\n\
             ## Status\n\n\
             - link physics is descriptive-only for now\n\
             - no procedural resolver chooses `current` or `aura` yet\n\
             - bond selection remains kernel-simple underneath this layer\n\
             - no feedback into Current Synthesis\n\
             - no feedback into Hollow Grove\n\n\
             ## Artifact Inputs\n\n\
             Current Synthesis sequence bytes: 8.\n\
             Hueman Path Crossovers bytes: 5.\n\n\
             ## Boundary Reminder\n\n\
             Link physics explains how unbonded links may later read as `current` or `aura`. It does not rewrite Bond, HollowGrove, or Current Synthesis sequence ownership.\n"
        );
    }

    #[test]
    fn hueman_link_physics_writes_a_deterministic_file() {
        let hueman_link_physics = build_hueman_link_physics_from_artifacts("sequence", "cross");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir = std::env::temp_dir().join(format!("hueman-link-physics-{nonce}"));
        let artifact_path = artifact_dir.join("hueman_link_physics.md");

        write_text_artifact(&artifact_path, &hueman_link_physics)
            .expect("hueman link physics artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path).expect("hueman link physics artifact should read"),
            hueman_link_physics
        );

        fs::remove_file(&artifact_path).expect("hueman link physics artifact should be removable");
        fs::remove_dir(&artifact_dir).expect("hueman link physics directory should be removable");
    }
}
