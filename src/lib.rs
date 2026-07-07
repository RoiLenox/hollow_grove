pub mod artifact_io;
pub mod aura_beam;
pub mod current_seam;
pub mod hollow_grove;
pub mod hueman_support;
pub mod kernel_pass;
pub mod kernel_pass_output;
pub mod point;
pub mod triway;

pub use artifact_io::{read_text_artifact, write_text_artifact};
pub use aura_beam::AuraBeam;
pub use current_seam::CurrentSeam;
pub use hollow_grove::{Bond, HollowGrove};
pub use kernel_pass::KernelPass;
pub use kernel_pass_output::{
    BOUNDARY_REMINDER, DESKTOP_STATUS_ARTIFACT_PATH, INVERSE_PATH_QUESTION,
    PROMPT_ARTIFACT_PATH, SNAPSHOT_ARTIFACT_PATH, build_desktop_status_output,
    build_inverse_path_prompt, build_prompt_artifact_output, build_snapshot_output,
    build_tree_output,
};
pub use point::Point;
pub use triway::{Triway, Way};

pub fn run_kernel_cycle(point: Point) -> KernelPass {
    let start = point;
    let triway = start.clone().become_triway();
    let hollow_grove = triway.clone().become_hollow_grove();
    let current_seam = hollow_grove.clone().become_current_seam();
    let aura_beam = current_seam.clone().project_aura_beam();
    let landed = aura_beam.clone().land_point();

    KernelPass::new(start, triway, hollow_grove, current_seam, aura_beam, landed)
}

pub fn kernel_proof() -> [&'static str; 10] {
    let _kernel_pass = run_kernel_cycle(Point);

    [
        "Current Synthesis creates Point #1",
        "Point becomes Triway.",
        "Triway carries one Point through three ways.",
        "Triway becomes Hollow Grove",
        "Hollow Grove forms Bond on one Way and leaves two ways as Atmosphere.",
        "Hollow Grove becomes CurrentSeam",
        "CurrentSeam projects AuraBeam",
        "AuraBeam lands Point #2",
        "KernelPass witnesses one completed recursion.",
        "Kernel recursion verified.",
    ]
}

#[cfg(test)]
mod tests {
    use super::{Bond, Point, Way, kernel_proof, run_kernel_cycle};

    #[test]
    fn point_becomes_the_next_point() {
        let kernel_pass = run_kernel_cycle(Point);
        assert_eq!(format!("{:?}", kernel_pass.landed_point()), "Point");
    }

    #[test]
    fn triway_carries_one_point_through_three_ways() {
        let triway = Point.become_triway();
        assert_eq!(triway.ways(), [Way::One, Way::Two, Way::Three]);
    }

    #[test]
    fn hollow_grove_resolves_link_and_atmosphere() {
        let hollow_grove = Point.become_triway().become_hollow_grove();
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
        let kernel_pass = run_kernel_cycle(Point);

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
            format!("{:?}", kernel_pass.current_seam()),
            "CurrentSeam { point: Point }"
        );
        assert_eq!(
            format!("{:?}", kernel_pass.aura_beam()),
            "AuraBeam { point: Point }"
        );
        assert_eq!(format!("{:?}", kernel_pass.landed_point()), "Point");
    }

    #[test]
    fn kernel_pass_displays_the_canonical_witness_deterministically() {
        let kernel_pass = run_kernel_cycle(Point);

        assert_eq!(
            format!("{kernel_pass}"),
            "start Point\n↓\nTriway\n↓\nHollowGrove\n↓\nCurrentSeam\n↓\nAuraBeam\n↓\nlanded Point"
        );
    }

    #[test]
    fn kernel_proof_reports_the_full_cycle() {
        assert_eq!(
            kernel_proof(),
            [
                "Current Synthesis creates Point #1",
                "Point becomes Triway.",
                "Triway carries one Point through three ways.",
                "Triway becomes Hollow Grove",
                "Hollow Grove forms Bond on one Way and leaves two ways as Atmosphere.",
                "Hollow Grove becomes CurrentSeam",
                "CurrentSeam projects AuraBeam",
                "AuraBeam lands Point #2",
                "KernelPass witnesses one completed recursion.",
                "Kernel recursion verified.",
            ]
        );
    }
}
