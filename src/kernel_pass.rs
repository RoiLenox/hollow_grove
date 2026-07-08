use std::fmt;

use crate::grove_seam::GroveSeam;
use crate::hollow_beam::HollowBeam;
use crate::hollow_grove::HollowGrove;
use crate::point::Point;
use crate::symptom::Symptom;
use crate::triway::Triway;

pub const START_WITNESS_LABEL: &str = "Symptom 1";
pub const LANDED_WITNESS_LABEL: &str = "Symptom 2";
pub const CANONICAL_WITNESS: &str =
    "start Symptom 1\n↓\nTriway\n↓\nHollowGrove\n↓\nGroveSeam\n↓\nHollowBeam\n↓\nlanded Symptom 2";

#[derive(Debug, Clone)]
pub struct KernelPass {
    start: Symptom,
    triway: Triway,
    hollow_grove: HollowGrove,
    grove_seam: GroveSeam,
    hollow_beam: HollowBeam,
    landed: Symptom,
}

impl KernelPass {
    pub fn new(
        start: Symptom,
        triway: Triway,
        hollow_grove: HollowGrove,
        grove_seam: GroveSeam,
        hollow_beam: HollowBeam,
        landed: Symptom,
    ) -> Self {
        Self {
            start,
            triway,
            hollow_grove,
            grove_seam,
            hollow_beam,
            landed,
        }
    }

    pub fn start_symptom(&self) -> &Symptom {
        &self.start
    }

    pub fn start_point(&self) -> &Point {
        self.start.point()
    }

    pub fn triway(&self) -> &Triway {
        &self.triway
    }

    pub fn hollow_grove(&self) -> &HollowGrove {
        &self.hollow_grove
    }

    pub fn grove_seam(&self) -> &GroveSeam {
        &self.grove_seam
    }

    pub fn hollow_beam(&self) -> &HollowBeam {
        &self.hollow_beam
    }

    pub fn landed_symptom(&self) -> &Symptom {
        &self.landed
    }

    pub fn end_point(&self) -> &Point {
        self.landed.point()
    }
}

impl fmt::Display for KernelPass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{CANONICAL_WITNESS}")
    }
}
