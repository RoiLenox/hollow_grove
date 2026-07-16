use std::io;
use std::path::Path;

use hollow_grove::hueman_progression::resolve_active_vertical_slice_at;
use hollow_grove::hueman_support::{
    HUEMAN_AURA_BEHAVIOR_ARTIFACT_PATH, HUEMAN_BOUNDARY_ARTIFACT_PATH,
    HUEMAN_PROCEDURAL_UPLIFT_ARTIFACT_PATH, HUEMAN_START_CHOICES_ARTIFACT_PATH,
    build_hueman_vertical_slice_for_spec_from_artifacts, hueman_vertical_slice_artifact_path,
};
use hollow_grove::{read_text_artifact, write_text_artifact};

fn main() -> io::Result<()> {
    let hueman_boundary = read_text_artifact(Path::new(HUEMAN_BOUNDARY_ARTIFACT_PATH))?;
    let hueman_start_choices = read_text_artifact(Path::new(HUEMAN_START_CHOICES_ARTIFACT_PATH))?;
    let hueman_aura_behavior = read_text_artifact(Path::new(HUEMAN_AURA_BEHAVIOR_ARTIFACT_PATH))?;
    let hueman_procedural_uplift =
        read_text_artifact(Path::new(HUEMAN_PROCEDURAL_UPLIFT_ARTIFACT_PATH))?;
    let active_slice = resolve_active_vertical_slice_at(Path::new("."))?;
    let hueman_vertical_slice = build_hueman_vertical_slice_for_spec_from_artifacts(
        active_slice,
        &hueman_boundary,
        &hueman_start_choices,
        &hueman_aura_behavior,
        &hueman_procedural_uplift,
    );
    let artifact_path = hueman_vertical_slice_artifact_path();

    write_text_artifact(&artifact_path, &hueman_vertical_slice)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use hollow_grove::hueman_support::build_hueman_vertical_slice_from_artifacts;
    use hollow_grove::write_text_artifact;

    #[test]
    fn hueman_vertical_slice_reads_existing_artifacts() {
        let output =
            build_hueman_vertical_slice_from_artifacts("boundary", "choices", "behavior", "uplift");

        assert!(output.starts_with("# Hueman Vertical Slice"));
        assert!(output.contains("Aura Ridge Opal Oil Starter Loop"));
        assert!(output.contains("`aura_ridge_opal_oil_gremlin`"));
        assert!(output.contains("Current Form path: Gremlin"));
        assert!(output.contains("Route Stabilization (`route`)"));
        assert!(output.contains("Flock Defense (`defense`)"));
        assert!(output.contains("proof: Bench proof must verify hinge-post stability"));
        assert!(output.contains("field output: Hinge Seal Charge x1"));
        assert!(output.contains("credential: Flockline Trust"));
        assert!(output.contains("unlocked next task: Route Hinge Survey"));
        assert!(output.contains("unlocked next task: Shelterline Night Watch"));
        assert!(output.contains("Hinge Seal Charge to two weaker posts"));
        assert!(output.contains("Ward Flare Charge to the recovery shelter edge"));
        assert!(output.contains("failure: Defense failure occurs if the anchor slips"));
        assert!(output.contains("Hueman Procedural Uplift bytes: 6."));
    }

    #[test]
    fn hueman_vertical_slice_writes_a_deterministic_file() {
        let hueman_vertical_slice =
            build_hueman_vertical_slice_from_artifacts("boundary", "choices", "behavior", "uplift");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir = std::env::temp_dir().join(format!("hueman-vertical-slice-{nonce}"));
        let artifact_path = artifact_dir.join("hueman_vertical_slice.md");

        write_text_artifact(&artifact_path, &hueman_vertical_slice)
            .expect("hueman vertical slice artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path).expect("hueman vertical slice artifact should read"),
            hueman_vertical_slice
        );

        fs::remove_file(&artifact_path)
            .expect("hueman vertical slice artifact should be removable");
        fs::remove_dir(&artifact_dir).expect("hueman vertical slice directory should be removable");
    }
}
