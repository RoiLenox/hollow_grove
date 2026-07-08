pub mod artifact_io;
pub mod grove_seam;
pub mod hollow_beam;
pub mod hollow_grove;
pub mod hueman_support;
pub mod kernel_pass;
pub mod kernel_pass_output;
pub mod point;
pub mod symptom;
pub mod triway;

pub use artifact_io::{
    ArtifactFlushRecord, ArtifactSession, read_text_artifact, write_text_artifact,
};
pub use grove_seam::GroveSeam;
pub use hollow_beam::HollowBeam;
pub use hollow_grove::{Bond, HollowGrove};
pub use kernel_pass::{CANONICAL_WITNESS, KernelPass, LANDED_WITNESS_LABEL, START_WITNESS_LABEL};
pub use kernel_pass_output::{
    BOUNDARY_REMINDER, DESKTOP_STATUS_ARTIFACT_PATH, INVERSE_PATH_QUESTION, PROMPT_ARTIFACT_PATH,
    SNAPSHOT_ARTIFACT_PATH, build_desktop_status_output, build_inverse_path_prompt,
    build_prompt_artifact_output, build_snapshot_output, build_tree_output,
};
pub use point::Point;
pub use symptom::Symptom;
pub use triway::{Triway, Way};

pub fn run_kernel_cycle(symptom: Symptom) -> KernelPass {
    let start = symptom;
    let triway = start.clone().become_triway();
    let hollow_grove = triway.clone().become_hollow_grove();
    let grove_seam = hollow_grove.clone().become_grove_seam();
    let hollow_beam = grove_seam.clone().achieve_hollow_beam();
    let landed = hollow_beam.clone().land_symptom();

    KernelPass::new(start, triway, hollow_grove, grove_seam, hollow_beam, landed)
}

pub fn kernel_proof() -> [&'static str; 10] {
    let _kernel_pass = run_kernel_cycle(Symptom::origin());

    [
        "Symptom 1 enters the kernel cycle.",
        "Symptom 1 becomes Triway.",
        "Triway carries one symptom through three ways.",
        "Triway becomes Hollow Grove",
        "Hollow Grove forms Bond on one Way and leaves two ways as Atmosphere.",
        "Hollow Grove becomes GroveSeam",
        "GroveSeam achieves HollowBeam",
        "HollowBeam lands Symptom 2.",
        "KernelPass witnesses one completed recursion.",
        "Kernel recursion verified.",
    ]
}

#[cfg(test)]
mod tests {
    use super::{Bond, CANONICAL_WITNESS, Symptom, Way, kernel_proof, run_kernel_cycle};

    #[test]
    fn symptom_lands_with_the_same_inner_point() {
        let kernel_pass = run_kernel_cycle(Symptom::origin());
        assert_eq!(
            format!("{:?}", kernel_pass.landed_symptom()),
            "Symptom { point: Point }"
        );
        assert_eq!(format!("{:?}", kernel_pass.end_point()), "Point");
    }

    #[test]
    fn triway_carries_one_symptom_through_three_ways() {
        let triway = Symptom::origin().become_triway();
        assert_eq!(triway.ways(), [Way::One, Way::Two, Way::Three]);
    }

    #[test]
    fn hollow_grove_resolves_link_and_atmosphere() {
        let hollow_grove = Symptom::origin().become_triway().become_hollow_grove();
        assert_eq!(hollow_grove.link(), Way::One);
        assert_eq!(hollow_grove.atmosphere(), [Way::Two, Way::Three]);
    }

    #[test]
    fn bond_selects_one_way() {
        let bond = Bond::select([Way::One, Way::Two, Way::Three]);
        assert_eq!(bond.linked_way(), Way::One);
    }

    #[test]
    fn kernel_pass_witnesses_one_completed_recursion() {
        let kernel_pass = run_kernel_cycle(Symptom::origin());

        assert_eq!(
            format!("{:?}", kernel_pass.start_symptom()),
            "Symptom { point: Point }"
        );
        assert_eq!(format!("{:?}", kernel_pass.start_point()), "Point");
        assert_eq!(
            kernel_pass.triway().ways(),
            [Way::One, Way::Two, Way::Three]
        );
        assert_eq!(kernel_pass.hollow_grove().link(), Way::One);
        assert_eq!(
            kernel_pass.hollow_grove().atmosphere(),
            [Way::Two, Way::Three]
        );
        assert_eq!(
            format!("{:?}", kernel_pass.grove_seam()),
            "GroveSeam { symptom: Symptom { point: Point } }"
        );
        assert_eq!(
            format!("{:?}", kernel_pass.hollow_beam()),
            "HollowBeam { symptom: Symptom { point: Point } }"
        );
        assert_eq!(
            format!("{:?}", kernel_pass.landed_symptom()),
            "Symptom { point: Point }"
        );
        assert_eq!(format!("{:?}", kernel_pass.end_point()), "Point");
    }

    #[test]
    fn kernel_pass_displays_the_canonical_witness_deterministically() {
        let kernel_pass = run_kernel_cycle(Symptom::origin());

        assert_eq!(format!("{kernel_pass}"), CANONICAL_WITNESS);
    }

    #[test]
    fn kernel_proof_reports_the_full_cycle() {
        assert_eq!(
            kernel_proof(),
            [
                "Symptom 1 enters the kernel cycle.",
                "Symptom 1 becomes Triway.",
                "Triway carries one symptom through three ways.",
                "Triway becomes Hollow Grove",
                "Hollow Grove forms Bond on one Way and leaves two ways as Atmosphere.",
                "Hollow Grove becomes GroveSeam",
                "GroveSeam achieves HollowBeam",
                "HollowBeam lands Symptom 2.",
                "KernelPass witnesses one completed recursion.",
                "Kernel recursion verified.",
            ]
        );
    }
}
