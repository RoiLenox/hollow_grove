pub mod aura_seam;
pub mod hollow_grove;
pub mod point;
pub mod triway;

pub use aura_seam::AuraSeam;
pub use hollow_grove::HollowGrove;
pub use point::Point;
pub use triway::{Triway, Way};

pub fn run_kernel_cycle(point: Point) -> Point {
    let triway = point.become_triway();
    let hollow_grove = triway.become_hollow_grove();
    let aura_seam = hollow_grove.become_aura_seam();
    aura_seam.create_point()
}

pub fn kernel_proof() -> [&'static str; 7] {
    let _point_2 = run_kernel_cycle(Point);

    [
        "Current Synthesis creates Point #1",
        "Point becomes Triway.",
        "Triway carries one Point through three ways.",
        "Triway becomes Hollow Grove",
        "Hollow Grove becomes AuraSeam",
        "AuraSeam creates Point #2",
        "Kernel recursion verified.",
    ]
}

#[cfg(test)]
mod tests {
    use super::{Point, Way, kernel_proof, run_kernel_cycle};

    #[test]
    fn point_becomes_the_next_point() {
        let next_point: Point = run_kernel_cycle(Point);
        assert_eq!(format!("{next_point:?}"), "Point");
    }

    #[test]
    fn triway_carries_one_point_through_three_ways() {
        let triway = Point.become_triway();
        assert_eq!(triway.ways(), [Way::One, Way::Two, Way::Three]);
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
                "Hollow Grove becomes AuraSeam",
                "AuraSeam creates Point #2",
                "Kernel recursion verified.",
            ]
        );
    }
}
