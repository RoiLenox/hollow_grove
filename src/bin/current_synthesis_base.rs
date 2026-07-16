use std::io;
use std::path::{Path, PathBuf};

#[path = "../current_synthesis_support.rs"]
mod current_synthesis_support;

use current_synthesis_support::{
    CURRENT_SYNTHESIS_BASE_ARTIFACT_PATH, DESKTOP_STATUS_ARTIFACT_PATH, PROMPT_ARTIFACT_PATH,
    SNAPSHOT_ARTIFACT_PATH, build_current_synthesis_base_from_artifacts, read_artifact,
    write_artifact,
};

fn artifact_path() -> PathBuf {
    PathBuf::from(CURRENT_SYNTHESIS_BASE_ARTIFACT_PATH)
}

fn main() -> io::Result<()> {
    let snapshot = read_artifact(Path::new(SNAPSHOT_ARTIFACT_PATH))?;
    let prompt = read_artifact(Path::new(PROMPT_ARTIFACT_PATH))?;
    let desktop_status = read_artifact(Path::new(DESKTOP_STATUS_ARTIFACT_PATH))?;
    let current_synthesis_base =
        build_current_synthesis_base_from_artifacts(&snapshot, &prompt, &desktop_status)?;
    let artifact_path = artifact_path();

    write_artifact(&artifact_path, &current_synthesis_base)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use hollow_grove::{
        Symptom, build_desktop_status_output, build_prompt_artifact_output, build_snapshot_output,
        run_kernel_cycle,
    };

    use super::current_synthesis_support::{
        build_current_synthesis_base_from_artifacts, write_artifact,
    };

    #[test]
    fn current_synthesis_base_reads_existing_hollow_grove_artifacts() {
        let kernel_pass = run_kernel_cycle(Symptom::origin());
        let snapshot = build_snapshot_output(&kernel_pass);
        let prompt = build_prompt_artifact_output(&kernel_pass);
        let desktop_status = build_desktop_status_output(&kernel_pass);
        let output =
            build_current_synthesis_base_from_artifacts(&snapshot, &prompt, &desktop_status)
                .expect("current synthesis base should build");

        assert!(output.contains("## Canonical Witness"));
        assert!(output.contains("Point² (Landed Point) [BlepArrival]"));
        assert!(output.contains("Fourway"));
        assert!(output.contains("CurrentSeam [PlebExterior]"));
        assert!(output.contains("AuraBeam [BlepReturn]"));
        assert!(output.contains("- universal landed point: `Point²`"));
    }

    #[test]
    fn current_synthesis_base_writes_a_deterministic_file() {
        let snapshot = "{\n\
                        \x20\x20\"grove_seam_route\": \"PlebExterior\",\n\
                        \x20\x20\"hollow_beam_route\": \"BlepReturn\",\n\
                        \x20\x20\"landing_route\": \"BlepArrival\",\n\
                        \x20\x20\"landed_point\": \"Point²\",\n\
                        \x20\x20\"canonical_witness\": \"Point\\n↓\\nTriway\\n↓\\nFourway\\n↓\\nHollowGrove\\n↓\\nCurrentSeam [PlebExterior]\\n↓\\nAuraBeam [BlepReturn]\\n↓\\nPoint² (Landed Point) [BlepArrival]\"\n\
                        }";
        let prompt = "# Consumer Prompt\n\n\
                      ## Canonical Witness\n\n\
                      ```text\n\
                      Point\n\
                      ↓\n\
                      Triway\n\
                      ↓\n\
                      Fourway\n\
                      ↓\n\
                      HollowGrove\n\
                      ↓\n\
                      CurrentSeam [PlebExterior]\n\
                      ↓\n\
                      AuraBeam [BlepReturn]\n\
                      ↓\n\
                      Point² (Landed Point) [BlepArrival]\n\
                      ```\n\n\
                      ## Structured Snapshot Reference\n\n\
                      `artifacts/kernel_pass_snapshot.json`\n\n\
                      ## Inverse-Path Question\n\n\
                      What does this completed pass reveal about the inverse path of the end use?\n\n\
                      ## Boundary Reminder\n\n\
                      Do not mutate the kernel. Interpret only.\n";
        let desktop_status = "Hollow Grove status: one completed witnessed recursion\n\nCanonical witness:\nPoint\n↓\nTriway\n↓\nFourway\n↓\nHollowGrove\n↓\nCurrentSeam [PlebExterior]\n↓\nAuraBeam [BlepReturn]\n↓\nPoint² (Landed Point) [BlepArrival]\n\nNote: read-only desktop artifact\nNote: niri/river configs untouched\n";
        let current_synthesis_base =
            build_current_synthesis_base_from_artifacts(snapshot, prompt, desktop_status)
                .expect("current synthesis base should build");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir = std::env::temp_dir().join(format!("current-synthesis-base-{nonce}"));
        let artifact_path = artifact_dir.join("current_synthesis_base.md");

        write_artifact(&artifact_path, &current_synthesis_base)
            .expect("current synthesis base artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path)
                .expect("current synthesis base artifact should be readable"),
            current_synthesis_base
        );

        fs::remove_file(&artifact_path)
            .expect("current synthesis base artifact should be removable");
        fs::remove_dir(&artifact_dir)
            .expect("current synthesis base directory should be removable");
    }

    #[test]
    fn current_synthesis_base_rejects_prompt_witness_mismatch() {
        let kernel_pass = run_kernel_cycle(Symptom::origin());
        let snapshot = build_snapshot_output(&kernel_pass);
        let prompt = build_prompt_artifact_output(&kernel_pass)
            .replace("CurrentSeam [PlebExterior]", "CurrentSeam [MetaExterior]");
        let desktop_status = build_desktop_status_output(&kernel_pass);

        let error =
            build_current_synthesis_base_from_artifacts(&snapshot, &prompt, &desktop_status)
                .expect_err("mismatched prompt witness should fail");

        assert_eq!(
            error.to_string(),
            "prompt artifact canonical witness does not match snapshot boundary"
        );
    }
}
