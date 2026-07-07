use std::io;
use std::path::Path;

use hollow_grove::hueman_support::{
    HUEMAN_AURA_BEHAVIOR_ARTIFACT_PATH, HUEMAN_START_CHOICES_ARTIFACT_PATH,
    build_hueman_archetype_lens_from_artifacts, hueman_archetype_lens_artifact_path,
};
use hollow_grove::{read_text_artifact, write_text_artifact};

fn main() -> io::Result<()> {
    let hueman_start_choices =
        read_text_artifact(Path::new(HUEMAN_START_CHOICES_ARTIFACT_PATH))?;
    let hueman_aura_behavior =
        read_text_artifact(Path::new(HUEMAN_AURA_BEHAVIOR_ARTIFACT_PATH))?;
    let hueman_archetype_lens =
        build_hueman_archetype_lens_from_artifacts(&hueman_start_choices, &hueman_aura_behavior);
    let artifact_path = hueman_archetype_lens_artifact_path();

    write_text_artifact(&artifact_path, &hueman_archetype_lens)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use hollow_grove::hueman_support::build_hueman_archetype_lens_from_artifacts;
    use hollow_grove::write_text_artifact;

    #[test]
    fn hueman_archetype_lens_reads_existing_artifacts() {
        assert_eq!(
            build_hueman_archetype_lens_from_artifacts("start", "aura"),
            "# Hueman Archetype Lens\n\n\
             ## Structural Rule\n\n\
             After start choice and AuraTriad behavior are declared, each archetype reads the same regions through a different descriptive lens.\n\n\
             ## Archetype Readings\n\n\
             ### `goblin`\n\n\
             - Aura Basin reads as burrow, shelter, and kept stores\n\
             - Aura Fields reads as forage paths, routes, and workable ground\n\
             - Aura Beach reads as exposed salvage, tide risk, and thin cover\n\n\
             ### `gremlin`\n\n\
             - Aura Basin reads as stress seams, pressure joints, and hidden leverage\n\
             - Aura Fields reads as barter space, friction lines, and noisy crossings\n\
             - Aura Beach reads as scrap edge, discard flow, and threshold apparatus\n\n\
             ### `pixy`\n\n\
             - Aura Basin reads as hush, glow, and suspended potential\n\
             - Aura Fields reads as shimmer, weather play, and visible drift\n\
             - Aura Beach reads as glint, spray, and bright dispersal\n\n\
             ### `sprite`\n\n\
             - Aura Basin reads as root echo, sleep, and soft enclosure\n\
             - Aura Fields reads as current, sway, and open circulation\n\
             - Aura Beach reads as horizon pull, release, and farward motion\n\n\
             ## Status\n\n\
             - archetype lens is descriptive-only for now\n\
             - no procedural bonuses or penalties are active\n\
             - the Fourway start roster remains unchanged\n\
             - AuraTriad region behavior remains shared underneath the lens\n\
             - no feedback into Current Synthesis\n\
             - no feedback into Hollow Grove\n\n\
             ## Artifact Inputs\n\n\
             Hueman Start Choices bytes: 5.\n\
             Hueman Aura Behavior bytes: 4.\n\n\
             ## Boundary Reminder\n\n\
             The archetype lens changes interpretation, not rules. It is a Hueman-facing difference in reading the world after placement.\n"
        );
    }

    #[test]
    fn hueman_archetype_lens_writes_a_deterministic_file() {
        let hueman_archetype_lens =
            build_hueman_archetype_lens_from_artifacts("start", "aura");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir = std::env::temp_dir().join(format!("hueman-archetype-lens-{nonce}"));
        let artifact_path = artifact_dir.join("hueman_archetype_lens.md");

        write_text_artifact(&artifact_path, &hueman_archetype_lens)
            .expect("hueman archetype lens artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path)
                .expect("hueman archetype lens artifact should read"),
            hueman_archetype_lens
        );

        fs::remove_file(&artifact_path)
            .expect("hueman archetype lens artifact should be removable");
        fs::remove_dir(&artifact_dir)
            .expect("hueman archetype lens directory should be removable");
    }
}
