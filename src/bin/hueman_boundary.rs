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
        assert_eq!(
            build_hueman_boundary_from_artifacts("base", "gate"),
            "# Hueman Boundary\n\n\
             ## Stack\n\n\
             ```text\n\
             Hollow Grove\n\
             ↓\n\
             KernelPass\n\
             ↓\n\
             Artifacts\n\
             ↓\n\
             Current Synthesis\n\
             ↓\n\
             Hueman\n\
             ```\n\n\
             ## Layer Role\n\n\
             - Hueman is the later persistent 32-bit collaboration/world layer.\n\
             - Current Synthesis remains the operating-system layer beneath it.\n\
             - Hollow Grove remains the recursive core beneath both.\n\n\
             ## Movement Distinction\n\n\
             - Hollow Grove moves active context through the locked field.\n\
             - Hueman moves the character sprite through the same locked field.\n\
             - Human Core remains the operator anchor.\n\n\
             ## Declared World Surface\n\n\
             - Flynt, Stonebend, Glaushouse, and Sandmanor are declared as Hueman-facing world anchors.\n\
             - civic roles, helper lines, kingdom roles, scene reading, and procedural uplift may be described above Current Synthesis.\n\
             - species logic is deferred.\n\
             - no feedback into Current Synthesis\n\
             - no feedback into Hollow Grove\n\n\
             ## Activation Status\n\n\
             - Current Synthesis activation remains denied.\n\
             - Hueman world activation is not enabled.\n\
             - collaborative persistence is not enabled.\n\
             - visual world mapping is not enabled.\n\n\
             ## Artifact Inputs\n\n\
             Current Synthesis base bytes: 4.\n\
             Current Synthesis activation gate bytes: 4.\n\n\
             ## Boundary Reminder\n\n\
             Hueman may consume Current Synthesis. Current Synthesis does not know Hueman exists.\n"
        );
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
