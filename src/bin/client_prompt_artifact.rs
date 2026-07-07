use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use hollow_grove::{KernelPass, Point, run_kernel_cycle};

const SNAPSHOT_ARTIFACT_PATH: &str = "artifacts/kernel_pass_snapshot.json";
const PROMPT_ARTIFACT_PATH: &str = "artifacts/consumer_prompt.md";
const INVERSE_PATH_QUESTION: &str =
    "What does this completed pass reveal about the inverse path of the end use?";
const BOUNDARY_REMINDER: &str = "Do not mutate the kernel. Interpret only.";

fn build_prompt_artifact_from_client(kernel_pass: &KernelPass) -> String {
    format!(
        "# Consumer Prompt\n\n\
         ## Canonical Witness\n\n\
         ```text\n\
         {}\n\
         ```\n\n\
         ## Structured Snapshot Reference\n\n\
         `{SNAPSHOT_ARTIFACT_PATH}`\n\n\
         ## Inverse-Path Question\n\n\
         {INVERSE_PATH_QUESTION}\n\n\
         ## Boundary Reminder\n\n\
         {BOUNDARY_REMINDER}\n",
        kernel_pass
    )
}

fn write_prompt_artifact(artifact_path: &Path, contents: &str) -> io::Result<()> {
    if let Some(parent) = artifact_path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(artifact_path, contents)
}

fn artifact_path() -> PathBuf {
    PathBuf::from(PROMPT_ARTIFACT_PATH)
}

fn main() -> io::Result<()> {
    let kernel_pass = run_kernel_cycle(Point);
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
    use hollow_grove::{Point, run_kernel_cycle};

    #[test]
    fn prompt_artifact_reads_the_completed_kernel_pass() {
        let kernel_pass = run_kernel_cycle(Point);

        assert_eq!(
            build_prompt_artifact_from_client(&kernel_pass),
            "# Consumer Prompt\n\n\
             ## Canonical Witness\n\n\
             ```text\n\
             start Point\n\
             ↓\n\
             Triway\n\
             ↓\n\
             HollowGrove\n\
             ↓\n\
             CurrentSeam\n\
             ↓\n\
             AuraBeam\n\
             ↓\n\
             landed Point\n\
             ```\n\n\
             ## Structured Snapshot Reference\n\n\
             `artifacts/kernel_pass_snapshot.json`\n\n\
             ## Inverse-Path Question\n\n\
             What does this completed pass reveal about the inverse path of the end use?\n\n\
             ## Boundary Reminder\n\n\
             Do not mutate the kernel. Interpret only.\n"
        );
        assert_eq!(
            kernel_pass.to_string(),
            "start Point\n↓\nTriway\n↓\nHollowGrove\n↓\nCurrentSeam\n↓\nAuraBeam\n↓\nlanded Point"
        );
    }

    #[test]
    fn prompt_artifact_writes_a_deterministic_file() {
        let kernel_pass = run_kernel_cycle(Point);
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
