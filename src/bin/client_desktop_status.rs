use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use hollow_grove::{
    DESKTOP_STATUS_ARTIFACT_PATH, KernelPass, Point, build_desktop_status_output,
    run_kernel_cycle,
};

fn build_desktop_status_from_client(kernel_pass: &KernelPass) -> String {
    build_desktop_status_output(kernel_pass)
}

fn write_desktop_status_artifact(artifact_path: &Path, contents: &str) -> io::Result<()> {
    if let Some(parent) = artifact_path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(artifact_path, contents)
}

fn artifact_path() -> PathBuf {
    PathBuf::from(DESKTOP_STATUS_ARTIFACT_PATH)
}

fn main() -> io::Result<()> {
    let kernel_pass = run_kernel_cycle(Point);
    let desktop_status = build_desktop_status_from_client(&kernel_pass);
    let artifact_path = artifact_path();

    write_desktop_status_artifact(&artifact_path, &desktop_status)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::{build_desktop_status_from_client, write_desktop_status_artifact};
    use hollow_grove::{Point, run_kernel_cycle};

    #[test]
    fn desktop_status_reads_the_completed_kernel_pass() {
        let kernel_pass = run_kernel_cycle(Point);

        assert_eq!(
            build_desktop_status_from_client(&kernel_pass),
            "Hollow Grove status: one completed witnessed recursion\n\n\
             Canonical witness:\n\
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
             landed Point\n\n\
             Note: read-only desktop artifact\n\
             Note: niri/river configs untouched\n"
        );
        assert_eq!(
            kernel_pass.to_string(),
            "start Point\n↓\nTriway\n↓\nHollowGrove\n↓\nCurrentSeam\n↓\nAuraBeam\n↓\nlanded Point"
        );
    }

    #[test]
    fn desktop_status_writes_a_deterministic_file() {
        let kernel_pass = run_kernel_cycle(Point);
        let desktop_status = build_desktop_status_from_client(&kernel_pass);
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir = std::env::temp_dir().join(format!("hollow-grove-desktop-{nonce}"));
        let artifact_path = artifact_dir.join("desktop_status.txt");

        write_desktop_status_artifact(&artifact_path, &desktop_status)
            .expect("desktop status artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path).expect("desktop status artifact should be readable"),
            desktop_status
        );

        fs::remove_file(&artifact_path).expect("desktop status artifact should be removable");
        fs::remove_dir(&artifact_dir).expect("desktop status directory should be removable");
    }
}
