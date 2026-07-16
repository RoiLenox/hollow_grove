use std::io;
use std::path::{Path, PathBuf};

use hollow_grove::{
    KernelPass, SNAPSHOT_ARTIFACT_PATH, Symptom, build_snapshot_output, run_kernel_cycle,
    write_text_artifact,
};

fn build_snapshot_from_client(kernel_pass: &KernelPass) -> String {
    build_snapshot_output(kernel_pass)
}

fn write_snapshot_artifact(artifact_path: &Path, contents: &str) -> io::Result<()> {
    write_text_artifact(artifact_path, contents)
}

fn artifact_path() -> PathBuf {
    PathBuf::from(SNAPSHOT_ARTIFACT_PATH)
}

fn main() -> io::Result<()> {
    let kernel_pass = run_kernel_cycle(Symptom::origin());
    let snapshot = build_snapshot_from_client(&kernel_pass);
    let artifact_path = artifact_path();

    write_snapshot_artifact(&artifact_path, &snapshot)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::{build_snapshot_from_client, write_snapshot_artifact};
    use hollow_grove::{CANONICAL_WITNESS, Symptom, run_kernel_cycle};

    #[test]
    fn snapshot_client_reads_the_completed_kernel_pass() {
        let kernel_pass = run_kernel_cycle(Symptom::origin());
        let output = build_snapshot_from_client(&kernel_pass);

        assert!(output.contains("\"start\": \"Point\""));
        assert!(output.contains("\"fourway\": \"Fourway\""));
        assert!(output.contains("\"grove_seam\": \"CurrentSeam\""));
        assert!(output.contains("\"hollow_beam\": \"AuraBeam\""));
        assert!(output.contains("\"landed\": \"Landed Point\""));
        assert!(output.contains("\"landed_point\": \"Point²\""));
        assert!(output.contains("Point² (Landed Point) [BlepArrival]"));
        assert_eq!(kernel_pass.to_string(), CANONICAL_WITNESS);
    }

    #[test]
    fn snapshot_client_writes_a_deterministic_artifact() {
        let kernel_pass = run_kernel_cycle(Symptom::origin());
        let snapshot = build_snapshot_from_client(&kernel_pass);
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir = std::env::temp_dir().join(format!("hollow-grove-snapshot-{nonce}"));
        let artifact_path = artifact_dir.join("kernel_pass_snapshot.json");

        write_snapshot_artifact(&artifact_path, &snapshot).expect("snapshot artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path).expect("snapshot artifact should be readable"),
            snapshot
        );

        fs::remove_file(&artifact_path).expect("snapshot artifact should be removable");
        fs::remove_dir(&artifact_dir).expect("snapshot directory should be removable");
    }
}
