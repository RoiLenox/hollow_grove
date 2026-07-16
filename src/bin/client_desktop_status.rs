use std::io;
use std::path::{Path, PathBuf};

use hollow_grove::{
    DESKTOP_STATUS_ARTIFACT_PATH, KernelPass, Symptom, build_desktop_status_output,
    run_kernel_cycle, write_text_artifact,
};

fn build_desktop_status_from_client(kernel_pass: &KernelPass) -> String {
    build_desktop_status_output(kernel_pass)
}

fn write_desktop_status_artifact(artifact_path: &Path, contents: &str) -> io::Result<()> {
    write_text_artifact(artifact_path, contents)
}

fn artifact_path() -> PathBuf {
    PathBuf::from(DESKTOP_STATUS_ARTIFACT_PATH)
}

fn main() -> io::Result<()> {
    let kernel_pass = run_kernel_cycle(Symptom::origin());
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
    use hollow_grove::{CANONICAL_WITNESS, Symptom, run_kernel_cycle};

    #[test]
    fn desktop_status_reads_the_completed_kernel_pass() {
        let kernel_pass = run_kernel_cycle(Symptom::origin());
        let output = build_desktop_status_from_client(&kernel_pass);

        assert!(output.contains("Canonical witness:\nPoint"));
        assert!(output.contains("Fourway"));
        assert!(output.contains("CurrentSeam [PlebExterior]"));
        assert!(output.contains("AuraBeam [BlepReturn]"));
        assert!(output.contains("Point² (Landed Point) [BlepArrival]"));
        assert!(output.contains("STARTING POINT"));
        assert!(output.contains("POINT²\nLanded Point"));
        assert!(output.contains("STARTING POINT\n\nFrame:\n  Hueman\n\nCURRENT PRISM"));
        assert!(output.contains("POINT²\nLanded Point\n\nFrame:\n  Pixy\n\nCURRENT PRISM"));
        assert!(output.contains("POINT²\nLanded Point\n\nFrame:\n  Gremlin\n\nCURRENT PRISM"));
        assert_eq!(output.matches("FLOW\n\n  none").count(), 2);
        assert!(output.contains("FLOW\n\n  TinkerGrip"));
        assert_eq!(output.matches("GLOW\n\n  none").count(), 2);
        assert_eq!(output.matches("GLOW\n\n  Confusion").count(), 1);
        assert!(output.contains("SYNTHESIS RECIPE\n\n  ID:\n    pixy_confusion"));
        assert!(output.contains("Pixy Confusion Recipe"));
        assert!(output.contains("GREMLIN TINKER RECIPE\n\n  ID:\n    gremlin_tinker"));
        assert!(output.contains("Gremlin Tinker Recipe"));
        assert!(output.contains("SYNTHESIS SCRIPTS\n\n  status:\n    ready"));
        assert!(output.contains("1. ApplyPrismDelta"));
        assert!(output.contains("2. AddGlow\n    Confusion"));
        assert!(output.contains("3. SetFrame\n    Pixy"));
        assert!(output.contains("2. AddFlow\n    TinkerGrip"));
        assert!(output.contains("3. SetFrame\n    Gremlin"));
        assert!(output.contains("SYNTHESIS AIM\n\n  ID:\n    pixy_confusion_aim"));
        assert!(output.contains("SYNTHESIS AIM\n\n  ID:\n    gremlin_tinker_aim"));
        assert!(output.contains("Source Recipe:\n    pixy_confusion"));
        assert!(output.contains("Source Recipe:\n    gremlin_tinker"));
        assert!(output.contains("Manager:\n    HAL"));
        assert!(output.contains("Manager:\n    Clouseau"));
        assert!(output.contains("Domain:\n    META"));
        assert!(output.contains("Domain:\n    PLEB"));
        assert!(output.contains("Relation:\n    PLEB ↔ META"));
        assert!(output.contains("Relation:\n    PLEB ↔ PLEB"));
        assert!(output.contains("Geometry:\n    curved"));
        assert!(output.contains("Geometry:\n    straight"));
        assert!(output.contains("Function:\n    Information From Beyond"));
        assert!(output.contains("Function:\n    Bond"));
        assert!(output.contains("Bond:\n    One"));
        assert!(output.contains("Named Route:\n    unset"));
        assert!(output.contains("Status:\n    prepared"));
        assert!(output.contains("FIRE\n\n  status:\n    committed"));
        assert!(output.contains("Contact:\n    Kiss"));
        assert!(output.contains("KISS LANDING\n\n  Scripts applied:\n    yes"));
        assert_eq!(
            output
                .matches("KISS LANDING\n\n  Scripts applied:\n    yes")
                .count(),
            2
        );
        assert!(output.contains("Starting Frame:\n    Hueman"));
        assert!(output.contains("Point² Frame:\n    Pixy"));
        assert!(output.contains("Point² Frame:\n    Gremlin"));
        assert!(output.contains("Glaüshouse / Mind:\n    3"));
        assert!(output.contains("Stonebend / Body:\n    3"));
        assert!(output.contains("GLOW LEARNED:\n    Confusion"));
        assert!(output.contains("GLOW LEARNED:\n    none"));
        assert!(output.contains("FLOW LEARNED:\n    none"));
        assert!(output.contains("FLOW LEARNED:\n    TinkerGrip"));
        assert!(output.contains("FrameState changed:\n    true"));
        assert!(output.contains("Point² produced:\n    true"));
        assert!(output.contains("MISS LANDING\n\n  Contact:\n    Miss"));
        assert_eq!(
            output
                .matches("MISS LANDING\n\n  Contact:\n    Miss")
                .count(),
            2
        );
        assert!(output.contains("Scripts applied:\n    no"));
        assert!(output.contains("VERSION 2 DECISION"));
        assert!(output.contains("Decision Trace"));
        assert!(output.contains("CURRENT-FAVORED"));
        assert!(output.contains("AURA-FAVORED"));
        assert!(output.contains("NEUTRAL"));
        assert!(output.contains("Observe:\n  Hueman, Intent FavorCurrent"));
        assert!(output.contains("Observe:\n  Hueman, Intent FavorAura"));
        assert!(output.contains("Observe:\n  Hueman, Intent Neutral"));
        assert!(output.contains("Flow: none"));
        assert!(output.contains("Glow: none"));
        assert!(output.contains("Route Geometry: straight"));
        assert!(
            output.contains("State checks:\n  GremlinTinker: frame=false, flow=false, glow=false")
        );
        assert!(output.contains(
            "GremlinTinker\n    Manager: Clouseau\n    Geometry: straight\n    Orientation: Current"
        ));
        assert!(output.contains(
            "PixyConfusion\n    Manager: HAL\n    Geometry: curved\n    Orientation: Aura"
        ));
        assert!(output.contains("Evaluate:\n  GremlinTinker = 2, preferred orientation"));
        assert!(output.contains("Intent score: 2"));
        assert!(output.contains("Realized penalty: 0"));
        assert!(output.contains("Reasons: PreferredCurrentOrientation"));
        assert!(output.contains("PixyConfusion = 2, preferred orientation"));
        assert!(output.contains("Reasons: PreferredAuraOrientation"));
        assert!(output.contains(
            "Tie-break:\n  observed straight geometry\n  Reason: ObservedRouteGeometryMatch"
        ));
        assert!(output.contains(
            "Recipe:\n  Gremlin Tinker Recipe\n  ID: gremlin_tinker\n  Handed to Version 1.1: true"
        ));
        assert!(output.contains(
            "Recipe:\n  Pixy Confusion Recipe\n  ID: pixy_confusion\n  Handed to Version 1.1: true"
        ));
        assert!(output.contains("Version 1.1 Execution:\n    Hueman → Kiss → Gremlin Point²"));
        assert!(output.contains("Version 1.1 Execution:\n    Hueman → Kiss → Pixy Point²"));
        assert!(output.contains("Point² produced: true"));
        assert!(output.contains("Body 1 → 3"));
        assert!(output.contains("Mind 1 → 3"));
        assert!(output.contains("Flow +TinkerGrip"));
        assert!(output.contains("Glow +Confusion"));
        assert!(!output.contains("SYNTHESIS BEAM"));
        assert!(!output.contains("Transmission:\n"));
        assert_eq!(kernel_pass.to_string(), CANONICAL_WITNESS);
    }

    #[test]
    fn desktop_status_writes_a_deterministic_file() {
        let kernel_pass = run_kernel_cycle(Symptom::origin());
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
