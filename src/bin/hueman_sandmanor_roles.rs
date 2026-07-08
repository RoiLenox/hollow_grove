use std::io;
use std::path::Path;

use hollow_grove::hueman_support::{
    HUEMAN_FOURWAY_ARTIFACT_PATH, HUEMAN_START_CHOICES_ARTIFACT_PATH,
    build_hueman_sandmanor_roles_from_artifacts, hueman_sandmanor_roles_artifact_path,
};
use hollow_grove::{read_text_artifact, write_text_artifact};

fn main() -> io::Result<()> {
    let hueman_start_choices = read_text_artifact(Path::new(HUEMAN_START_CHOICES_ARTIFACT_PATH))?;
    let hueman_fourway = read_text_artifact(Path::new(HUEMAN_FOURWAY_ARTIFACT_PATH))?;
    let hueman_sandmanor_roles =
        build_hueman_sandmanor_roles_from_artifacts(&hueman_start_choices, &hueman_fourway);
    let artifact_path = hueman_sandmanor_roles_artifact_path();

    write_text_artifact(&artifact_path, &hueman_sandmanor_roles)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use hollow_grove::hueman_support::build_hueman_sandmanor_roles_from_artifacts;
    use hollow_grove::write_text_artifact;

    #[test]
    fn hueman_sandmanor_roles_reads_existing_artifacts() {
        assert_eq!(
            build_hueman_sandmanor_roles_from_artifacts("start", "fourway"),
            "# Hueman Sandmanor Roles\n\n\
             ## Structural Rule\n\n\
             Sandmanor carries a rival two-house rule inside Hueman's world layer, where northern accountancy and southern interior-song design compete through reciprocal teaching rather than inherited fixed sovereignty.\n\n\
             ## Canonical Anchor\n\n\
             - Sandmanor remains the West-facing `sprite` start on the Fourway.\n\
             - from Stonebend, Sandmanor sits on the far counter-arc.\n\
             - from Glaushouse, Sandmanor may read eastward across the relational arc without changing the canonical map.\n\n\
             - Sandmanor mines crystals.\n\
             - Sandmanor exports Crystoleum as its main outward trade good, formed from current and crystals.\n\n\
             ## Sandmanor Halves\n\n\
             - Sandmen are the people of Sandmanor.\n\
             - Minoans hold the South.\n\
             - Minorians hold the North.\n\n\
             ## Native Crafts\n\n\
             - Minoans design interiors, rooms, and atmospheres like a song.\n\
             - Minorians account, tally, and measure what Sandmanor can sustain.\n\
             - Minoans and Minorians are the rival houses inside the Sandmen.\n\
             - each side keeps its own people and its own craft pressure.\n\n\
             ## Rival Teaching Contract\n\n\
             - a Minorian must teach a Minoan to account.\n\
             - a Minoan must teach a Minorian to design like a song.\n\
             - each rival has to improve at the other's native discipline rather than remain pure.\n\n\
             ## Sandman Rule\n\n\
             - the crowd judges which rival is most improved by the opposing lesson.\n\
             - the office of rule is the Sandman.\n\
             - a Minoan winner is referred to as the Sandmanite.\n\
             - a Minorian winner is referred to as the Sandmanorian.\n\
             - the winning title-holder becomes ruler of Sandmanor until the contest turns again.\n\
             - rule is earned by witnessed improvement, not fixed inheritance.\n\n\
             ## Status\n\n\
             - Sandmanor roles are descriptive-only for now\n\
             - no contest resolver or crowd AI is active\n\
             - no automatic succession cycle is active\n\
             - no feedback into Current Synthesis\n\
             - no feedback into Hollow Grove\n\n\
             ## Artifact Inputs\n\n\
             Hueman Start Choices bytes: 5.\n\
             Hueman Fourway bytes: 7.\n\n\
             ## Boundary Reminder\n\n\
             Sandmanor roles belong to Hueman's kingdom layer. They do not replace Fourway placement, scene logic, or any Current Synthesis client boundary.\n"
        );
    }

    #[test]
    fn hueman_sandmanor_roles_writes_a_deterministic_file() {
        let hueman_sandmanor_roles =
            build_hueman_sandmanor_roles_from_artifacts("start", "fourway");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir = std::env::temp_dir().join(format!("hueman-sandmanor-roles-{nonce}"));
        let artifact_path = artifact_dir.join("hueman_sandmanor_roles.md");

        write_text_artifact(&artifact_path, &hueman_sandmanor_roles)
            .expect("hueman sandmanor roles artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path)
                .expect("hueman sandmanor roles artifact should read"),
            hueman_sandmanor_roles
        );

        fs::remove_file(&artifact_path)
            .expect("hueman sandmanor roles artifact should be removable");
        fs::remove_dir(&artifact_dir)
            .expect("hueman sandmanor roles directory should be removable");
    }
}
