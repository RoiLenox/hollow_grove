use std::path::Path;

use hollow_grove::{SNAPSHOT_ARTIFACT_PATH, SnapshotBoundary, build_snapshot_boundary_output};

fn main() -> std::io::Result<()> {
    let boundary = SnapshotBoundary::read_from_path(Path::new(SNAPSHOT_ARTIFACT_PATH))?;
    println!("{}", build_snapshot_boundary_output(&boundary));
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use hollow_grove::{
        ExteriorShape, KernelInput, Mode, PlebMetaInput, SnapshotBoundary, Symptom,
        build_snapshot_boundary_output, build_snapshot_output, run_kernel_cycle,
        run_kernel_cycle_with_input, write_text_artifact,
    };

    #[test]
    fn boundary_consumer_reads_the_frozen_straight_snapshot_artifact() {
        let kernel_pass = run_kernel_cycle(Symptom::origin());
        let boundary = SnapshotBoundary::parse(&build_snapshot_output(&kernel_pass))
            .expect("snapshot boundary should parse");
        let output = build_snapshot_boundary_output(&boundary);

        assert!(output.contains("landed_point: Point²"));
        assert!(output.contains("Fourway"));
        assert!(output.contains("CurrentSeam [PlebExterior]"));
        assert!(output.contains("AuraBeam [BlepReturn]"));
        assert!(output.contains("Point² (Landed Point) [BlepArrival]"));
    }

    #[test]
    fn boundary_consumer_reads_the_frozen_curved_snapshot_artifact() {
        let kernel_pass = run_kernel_cycle_with_input(
            Symptom::origin(),
            KernelInput {
                routing: PlebMetaInput {
                    exterior_shape: ExteriorShape::Curved,
                    pleb_mode: Mode::Pathos,
                    meta_mode: Mode::Logos,
                },
            },
        );
        let boundary = SnapshotBoundary::parse(&build_snapshot_output(&kernel_pass))
            .expect("snapshot boundary should parse");

        assert_eq!(boundary.grove_seam_route(), "MetaExterior");
        assert_eq!(boundary.hollow_beam_route(), "AtemReturn");
        assert_eq!(boundary.landing_route(), "AtemArrival");
        assert!(build_snapshot_boundary_output(&boundary).contains("landed_point: Point²"));
    }

    #[test]
    fn boundary_consumer_reads_a_snapshot_file_without_kernelpass_access() {
        let kernel_pass = run_kernel_cycle(Symptom::origin());
        let snapshot = build_snapshot_output(&kernel_pass);
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir = std::env::temp_dir().join(format!("hollow-grove-boundary-{nonce}"));
        let artifact_path = artifact_dir.join("kernel_pass_snapshot.json");

        write_text_artifact(&artifact_path, &snapshot).expect("snapshot artifact should write");

        let boundary =
            SnapshotBoundary::read_from_path(&artifact_path).expect("snapshot should parse");
        assert_eq!(boundary.grove_seam_route(), "PlebExterior");
        assert_eq!(boundary.hollow_beam_route(), "BlepReturn");
        assert_eq!(boundary.landing_route(), "BlepArrival");

        fs::remove_file(&artifact_path).expect("snapshot artifact should be removable");
        fs::remove_dir(&artifact_dir).expect("snapshot directory should be removable");
    }
}
