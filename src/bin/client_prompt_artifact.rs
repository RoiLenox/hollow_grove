use std::io;
use std::path::{Path, PathBuf};

use hollow_grove::{
    KernelPass, PROMPT_ARTIFACT_PATH, Symptom, build_prompt_artifact_output, run_kernel_cycle,
    write_text_artifact,
};

fn build_prompt_artifact_from_client(kernel_pass: &KernelPass) -> String {
    build_prompt_artifact_output(kernel_pass)
}

fn write_prompt_artifact(artifact_path: &Path, contents: &str) -> io::Result<()> {
    write_text_artifact(artifact_path, contents)
}

fn artifact_path() -> PathBuf {
    PathBuf::from(PROMPT_ARTIFACT_PATH)
}

fn main() -> io::Result<()> {
    let kernel_pass = run_kernel_cycle(Symptom::origin());
    let prompt_artifact = build_prompt_artifact_from_client(&kernel_pass);
    let artifact_path = artifact_path();

    write_prompt_artifact(&artifact_path, &prompt_artifact)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::{build_prompt_artifact_from_client, write_prompt_artifact};
    use hollow_grove::{CANONICAL_WITNESS, Symptom, run_kernel_cycle};

    #[test]
    fn prompt_artifact_reads_the_completed_kernel_pass() {
        let kernel_pass = run_kernel_cycle(Symptom::origin());
        let output = build_prompt_artifact_from_client(&kernel_pass);

        assert!(output.contains("## Canonical Witness"));
        assert!(output.contains("Point² (Landed Point) [BlepArrival]"));
        assert!(output.contains("Fourway"));
        assert!(output.contains("CurrentSeam [PlebExterior]"));
        assert!(output.contains("AuraBeam [BlepReturn]"));
        assert_eq!(kernel_pass.to_string(), CANONICAL_WITNESS);
    }

    #[test]
    fn prompt_artifact_writes_a_deterministic_file() {
        let kernel_pass = run_kernel_cycle(Symptom::origin());
        let prompt_artifact = build_prompt_artifact_from_client(&kernel_pass);
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir = std::env::temp_dir().join(format!("hollow-grove-prompt-{nonce}"));
        let artifact_path = artifact_dir.join("consumer_prompt.md");

        write_prompt_artifact(&artifact_path, &prompt_artifact)
            .expect("prompt artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path).expect("prompt artifact should be readable"),
            prompt_artifact
        );

        fs::remove_file(&artifact_path).expect("prompt artifact should be removable");
        fs::remove_dir(&artifact_dir).expect("prompt directory should be removable");
    }
}
