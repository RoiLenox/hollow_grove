use std::io;
use std::path::Path;

use hollow_grove::hueman_support::{
    CURRENT_SYNTHESIS_ACTIVATION_GATE_ARTIFACT_PATH, CURRENT_SYNTHESIS_BASE_ARTIFACT_PATH,
    build_hueman_boundary_from_artifacts, hueman_boundary_artifact_path,
};
use hollow_grove::{read_text_artifact, write_text_artifact};

fn main() -> io::Result<()> {
    let current_synthesis_base =
        read_text_artifact(Path::new(CURRENT_SYNTHESIS_BASE_ARTIFACT_PATH))?;
    let current_synthesis_activation_gate =
        read_text_artifact(Path::new(CURRENT_SYNTHESIS_ACTIVATION_GATE_ARTIFACT_PATH))?;
    let hueman_boundary = build_hueman_boundary_from_artifacts(
        &current_synthesis_base,
        &current_synthesis_activation_gate,
    );
    let artifact_path = hueman_boundary_artifact_path();

    write_text_artifact(&artifact_path, &hueman_boundary)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use hollow_grove::hueman_support::build_hueman_boundary_from_artifacts;
    use hollow_grove::write_text_artifact;

    #[test]
    fn hueman_boundary_reads_existing_artifacts() {
        let output = build_hueman_boundary_from_artifacts("base", "gate");
        assert!(output.starts_with("# Hueman Boundary"));
        assert!(output.contains("standalone civilization sandbox layer"));
        assert!(output.contains("Godot 4 and Aseprite are appropriate for Hueman."));
        assert!(output.contains("Current Synthesis activation gate bytes: 4."));
    }

    #[test]
    fn hueman_boundary_writes_a_deterministic_file() {
        let hueman_boundary = build_hueman_boundary_from_artifacts("base", "gate");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir = std::env::temp_dir().join(format!("hueman-boundary-{nonce}"));
        let artifact_path = artifact_dir.join("hueman_boundary.md");

        write_text_artifact(&artifact_path, &hueman_boundary)
            .expect("hueman boundary artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path).expect("hueman boundary artifact should read"),
            hueman_boundary
        );

        fs::remove_file(&artifact_path).expect("hueman boundary artifact should be removable");
        fs::remove_dir(&artifact_dir).expect("hueman boundary directory should be removable");
    }
}
