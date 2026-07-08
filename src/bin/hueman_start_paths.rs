use std::io;
use std::path::Path;

use hollow_grove::hueman_support::{
    HUEMAN_ARCHETYPE_LENS_ARTIFACT_PATH, HUEMAN_START_CHOICES_ARTIFACT_PATH,
    build_hueman_start_paths_from_artifacts, hueman_start_paths_artifact_path,
};
use hollow_grove::{read_text_artifact, write_text_artifact};

fn main() -> io::Result<()> {
    let hueman_start_choices = read_text_artifact(Path::new(HUEMAN_START_CHOICES_ARTIFACT_PATH))?;
    let hueman_archetype_lens = read_text_artifact(Path::new(HUEMAN_ARCHETYPE_LENS_ARTIFACT_PATH))?;
    let hueman_start_paths =
        build_hueman_start_paths_from_artifacts(&hueman_start_choices, &hueman_archetype_lens);
    let artifact_path = hueman_start_paths_artifact_path();

    write_text_artifact(&artifact_path, &hueman_start_paths)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use hollow_grove::hueman_support::build_hueman_start_paths_from_artifacts;
    use hollow_grove::write_text_artifact;

    #[test]
    fn hueman_start_paths_reads_existing_artifacts() {
        assert_eq!(
            build_hueman_start_paths_from_artifacts("start", "lens"),
            "# Hueman Start Paths\n\n\
             ## Structural Rule\n\n\
             Each Fourway start enters AuraTriad through a first descriptive region before any procedural mechanics exist.\n\n\
             ## Route Order\n\n\
             - Flynt = `goblin` = Aura Basin -> Aura Fields -> Aura Beach\n\
             - Stonebend = `gremlin` = Aura Fields -> Aura Basin -> Aura Beach\n\
             - Glaushouse = `pixy` = Aura Beach -> Aura Fields -> Aura Basin\n\
             - Sandmanor = `sprite` = Aura Beach -> Aura Basin -> Aura Fields\n\n\
             ## First Entry\n\n\
             - Flynt enters Aura Basin first.\n\
             - Stonebend enters Aura Fields first.\n\
             - Glaushouse enters Aura Beach first.\n\
             - Sandmanor enters Aura Beach first.\n\n\
             ## Status\n\n\
             - start-path order is descriptive-only for now\n\
             - the first region is declared but not procedurally enforced\n\
             - archetype lens remains interpretive above the route order\n\
             - species mechanics are not active yet\n\
             - no feedback into Current Synthesis\n\
             - no feedback into Hollow Grove\n\n\
             ## Artifact Inputs\n\n\
             Hueman Start Choices bytes: 5.\n\
             Hueman Archetype Lens bytes: 4.\n\n\
             ## Boundary Reminder\n\n\
             Start paths declare which AuraTriad region a start naturally enters first. They do not add movement rules or alter lower-layer topology.\n"
        );
    }

    #[test]
    fn hueman_start_paths_writes_a_deterministic_file() {
        let hueman_start_paths = build_hueman_start_paths_from_artifacts("start", "lens");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir = std::env::temp_dir().join(format!("hueman-start-paths-{nonce}"));
        let artifact_path = artifact_dir.join("hueman_start_paths.md");

        write_text_artifact(&artifact_path, &hueman_start_paths)
            .expect("hueman start paths artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path).expect("hueman start paths artifact should read"),
            hueman_start_paths
        );

        fs::remove_file(&artifact_path).expect("hueman start paths artifact should be removable");
        fs::remove_dir(&artifact_dir).expect("hueman start paths directory should be removable");
    }
}
