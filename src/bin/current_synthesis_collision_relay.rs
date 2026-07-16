use std::io;
use std::path::{Path, PathBuf};

use hollow_grove::SnapshotBoundary;

#[path = "../current_synthesis_support.rs"]
mod current_synthesis_support;

use current_synthesis_support::{
    CURRENT_SYNTHESIS_COLLISION_RELAY_ARTIFACT_PATH, CURRENT_SYNTHESIS_CONTRACT_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_OPERATIONAL_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_TRANSITION_PM_TO_LE_ARTIFACT_PATH, SNAPSHOT_ARTIFACT_PATH,
    build_current_synthesis_collision_relay_from_boundary, read_artifact, write_artifact,
};

fn artifact_path() -> PathBuf {
    PathBuf::from(CURRENT_SYNTHESIS_COLLISION_RELAY_ARTIFACT_PATH)
}

fn build_current_synthesis_collision_relay_at(root: &Path) -> io::Result<String> {
    let snapshot = read_artifact(&root.join(SNAPSHOT_ARTIFACT_PATH))?;
    let snapshot_boundary = SnapshotBoundary::parse(&snapshot)?;
    let current_synthesis_contract =
        read_artifact(&root.join(CURRENT_SYNTHESIS_CONTRACT_ARTIFACT_PATH))?;
    let current_synthesis_operational =
        read_artifact(&root.join(CURRENT_SYNTHESIS_OPERATIONAL_ARTIFACT_PATH))?;
    let current_synthesis_transition_pm_to_le =
        read_artifact(&root.join(CURRENT_SYNTHESIS_TRANSITION_PM_TO_LE_ARTIFACT_PATH))?;

    Ok(build_current_synthesis_collision_relay_from_boundary(
        &snapshot_boundary,
        snapshot.len(),
        &current_synthesis_contract,
        &current_synthesis_operational,
        &current_synthesis_transition_pm_to_le,
    ))
}

fn main() -> io::Result<()> {
    let current_synthesis_collision_relay =
        build_current_synthesis_collision_relay_at(Path::new("."))?;
    let artifact_path = artifact_path();

    write_artifact(&artifact_path, &current_synthesis_collision_relay)?;
    println!("{}", artifact_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io;
    use std::path::Path;
    use std::time::{SystemTime, UNIX_EPOCH};

    use hollow_grove::{
        ExteriorShape, KernelInput, Mode, PlebMetaInput, Symptom, build_snapshot_output,
        run_kernel_cycle, run_kernel_cycle_with_input,
    };

    use super::build_current_synthesis_collision_relay_at;
    use super::current_synthesis_support::{
        build_current_synthesis_collision_relay_from_artifacts, write_artifact,
    };

    fn write_fixture(root: &Path, relative_path: &str, contents: &str) {
        let path = root.join(relative_path);
        write_artifact(&path, contents).expect("fixture should write");
    }

    #[test]
    fn current_synthesis_collision_relay_reads_existing_artifacts() {
        let snapshot = build_snapshot_output(&run_kernel_cycle(Symptom::origin()));
        let output = build_current_synthesis_collision_relay_from_artifacts(
            &snapshot,
            "contract",
            "operational",
            "transition",
        )
        .expect("collision relay should build");
        assert!(output.starts_with("# Current Synthesis Collision Relay"));
        assert!(output.contains("## Frozen Kernel Collision Boundary"));
        assert!(output.contains("- exterior ingress: `PlebExterior`"));
        assert!(output.contains("- complementary return: `BlepReturn`"));
        assert!(output.contains("- landed route: `BlepArrival`"));
        assert!(output.contains("- universal landed point: `Point²`"));
        assert!(output.contains("## Packet Schema"));
        assert!(output.contains("packet id: `cleo_hal_pm_le_confirmation`"));
        assert!(output.contains("## Hueman Consumption Targets"));
    }

    #[test]
    fn current_synthesis_collision_relay_writes_a_deterministic_file() {
        let snapshot = build_snapshot_output(&run_kernel_cycle(Symptom::origin()));
        let current_synthesis_collision_relay =
            build_current_synthesis_collision_relay_from_artifacts(
                &snapshot,
                "contract",
                "operational",
                "transition",
            )
            .expect("collision relay should build");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_dir =
            std::env::temp_dir().join(format!("current-synthesis-collision-relay-{nonce}"));
        let artifact_path = artifact_dir.join("current_synthesis_collision_relay.md");

        write_artifact(&artifact_path, &current_synthesis_collision_relay)
            .expect("current synthesis collision relay artifact should write");

        assert_eq!(
            fs::read_to_string(&artifact_path)
                .expect("current synthesis collision relay artifact should be readable"),
            current_synthesis_collision_relay
        );

        fs::remove_file(&artifact_path)
            .expect("current synthesis collision relay artifact should be removable");
        fs::remove_dir(&artifact_dir)
            .expect("current synthesis collision relay directory should be removable");
    }

    #[test]
    fn current_synthesis_collision_relay_reads_the_curved_boundary_directly() {
        let snapshot = build_snapshot_output(&run_kernel_cycle_with_input(
            Symptom::origin(),
            KernelInput {
                routing: PlebMetaInput {
                    exterior_shape: ExteriorShape::Curved,
                    pleb_mode: Mode::Pathos,
                    meta_mode: Mode::Logos,
                },
            },
        ));
        let output = build_current_synthesis_collision_relay_from_artifacts(
            &snapshot,
            "contract",
            "operational",
            "PlebExterior BlepReturn BlepArrival",
        )
        .expect("curved collision relay should build");

        assert!(output.contains("- exterior ingress: `MetaExterior`"));
        assert!(output.contains("- complementary return: `AtemReturn`"));
        assert!(output.contains("- landed route: `AtemArrival`"));
        assert!(output.contains("- universal landed point: `Point²`"));
        assert!(!output.contains("- exterior ingress: `PlebExterior`"));
    }

    #[test]
    fn current_synthesis_collision_relay_rejects_invalid_snapshots() {
        let error = build_current_synthesis_collision_relay_from_artifacts(
            "{\"canonical_witness\": \"x\"}",
            "contract",
            "operational",
            "transition",
        )
        .expect_err("invalid snapshot should fail");

        assert_eq!(
            error.to_string(),
            "snapshot missing string field `grove_seam_route`"
        );
    }

    #[test]
    fn current_synthesis_collision_relay_rejects_contradictory_route_chains() {
        let error = build_current_synthesis_collision_relay_from_artifacts(
            "{\n\
             \x20\x20\"grove_seam_route\": \"PlebExterior\",\n\
             \x20\x20\"hollow_beam_route\": \"AtemReturn\",\n\
             \x20\x20\"landing_route\": \"AtemArrival\",\n\
             \x20\x20\"landed_point\": \"Point²\",\n\
             \x20\x20\"canonical_witness\": \"Point\\n↓\\nTriway\\n↓\\nFourway\\n↓\\nHollowGrove\\n↓\\nCurrentSeam [PlebExterior]\\n↓\\nAuraBeam [AtemReturn]\\n↓\\nPoint² (Landed Point) [AtemArrival]\"\n\
             }",
            "contract",
            "operational",
            "transition",
        )
        .expect_err("contradictory route chain should fail");

        assert_eq!(
            error.to_string(),
            "snapshot boundary route chain is contradictory: PlebExterior -> AtemReturn -> AtemArrival"
        );
    }

    #[test]
    fn current_synthesis_collision_relay_does_not_fall_back_to_transition_or_prose_when_snapshot_is_missing()
     {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let artifact_root =
            std::env::temp_dir().join(format!("current-synthesis-collision-relay-missing-{nonce}"));

        write_fixture(
            &artifact_root,
            "artifacts/current_synthesis_contract.md",
            "contract",
        );
        write_fixture(
            &artifact_root,
            "artifacts/current_synthesis_operational.md",
            "operational",
        );
        write_fixture(
            &artifact_root,
            "artifacts/current_synthesis_transition_pm_to_le.md",
            "PlebExterior BlepReturn BlepArrival Point²",
        );

        let error = build_current_synthesis_collision_relay_at(&artifact_root)
            .expect_err("missing snapshot should fail");

        assert_eq!(error.kind(), io::ErrorKind::NotFound);

        fs::remove_file(&artifact_root.join("artifacts/current_synthesis_contract.md"))
            .expect("contract fixture should be removable");
        fs::remove_file(&artifact_root.join("artifacts/current_synthesis_operational.md"))
            .expect("operational fixture should be removable");
        fs::remove_file(&artifact_root.join("artifacts/current_synthesis_transition_pm_to_le.md"))
            .expect("transition fixture should be removable");
        fs::remove_dir(&artifact_root.join("artifacts"))
            .expect("artifact directory should be removable");
        fs::remove_dir(&artifact_root).expect("fixture root should be removable");
    }
}
