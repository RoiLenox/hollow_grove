use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use hollow_grove::{KernelPass, Point, Way, run_kernel_cycle};

const ARTIFACT_PATH: &str = "artifacts/kernel_pass_snapshot.json";

fn way_name(way: Way) -> &'static str {
    match way {
        Way::One => "One",
        Way::Two => "Two",
        Way::Three => "Three",
    }
}

fn build_snapshot_from_client(kernel_pass: &KernelPass) -> String {
    let triway = kernel_pass.triway();
    let [way_one, way_two, way_three] = triway.ways();

    let hollow_grove = kernel_pass.hollow_grove();
    let [atmosphere_one, atmosphere_two] = hollow_grove.atmosphere();

    format!(
        "{{\n\
         \x20\x20\"start\": \"{:?}\",\n\
         \x20\x20\"triway\": {{\n\
         \x20\x20\x20\x20\"ways\": [\"{}\", \"{}\", \"{}\"]\n\
         \x20\x20}},\n\
         \x20\x20\"hollow_grove\": {{\n\
         \x20\x20\x20\x20\"bond\": \"{}\",\n\
         \x20\x20\x20\x20\"atmosphere\": [\"{}\", \"{}\"]\n\
         \x20\x20}},\n\
         \x20\x20\"current_seam\": \"CurrentSeam\",\n\
         \x20\x20\"aura_beam\": \"AuraBeam\",\n\
         \x20\x20\"landed\": \"{:?}\",\n\
         \x20\x20\"canonical_witness\": \"{}\"\n\
         }}",
        kernel_pass.start_point(),
        way_name(way_one),
        way_name(way_two),
        way_name(way_three),
        way_name(hollow_grove.link()),
        way_name(atmosphere_one),
        way_name(atmosphere_two),
        kernel_pass.landed_point(),
        kernel_pass.to_string().replace('\n', "\\n")
    )
}

fn write_snapshot_artifact(artifact_path: &Path, contents: &str) -> io::Result<()> {
    if let Some(parent) = artifact_path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(artifact_path, contents)
}

fn artifact_path() -> PathBuf {
    PathBuf::from(ARTIFACT_PATH)
}

fn main() -> io::Result<()> {
    let kernel_pass = run_kernel_cycle(Point);
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
    use hollow_grove::{Point, run_kernel_cycle};

    #[test]
    fn snapshot_client_reads_the_completed_kernel_pass() {
        let kernel_pass = run_kernel_cycle(Point);

        assert_eq!(
            build_snapshot_from_client(&kernel_pass),
            "{\n\
             \x20\x20\"start\": \"Point\",\n\
             \x20\x20\"triway\": {\n\
             \x20\x20\x20\x20\"ways\": [\"One\", \"Two\", \"Three\"]\n\
             \x20\x20},\n\
             \x20\x20\"hollow_grove\": {\n\
             \x20\x20\x20\x20\"bond\": \"One\",\n\
             \x20\x20\x20\x20\"atmosphere\": [\"Two\", \"Three\"]\n\
             \x20\x20},\n\
             \x20\x20\"current_seam\": \"CurrentSeam\",\n\
             \x20\x20\"aura_beam\": \"AuraBeam\",\n\
             \x20\x20\"landed\": \"Point\",\n\
             \x20\x20\"canonical_witness\": \"start Point\\n↓\\nTriway\\n↓\\nHollowGrove\\n↓\\nCurrentSeam\\n↓\\nAuraBeam\\n↓\\nlanded Point\"\n\
             }"
        );
        assert_eq!(
            kernel_pass.to_string(),
            "start Point\n↓\nTriway\n↓\nHollowGrove\n↓\nCurrentSeam\n↓\nAuraBeam\n↓\nlanded Point"
        );
    }

    #[test]
    fn snapshot_client_writes_a_deterministic_artifact() {
        let kernel_pass = run_kernel_cycle(Point);
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
