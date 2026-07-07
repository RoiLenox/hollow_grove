use std::fmt;

use crate::aura_beam::AuraBeam;
use crate::current_seam::CurrentSeam;
use crate::hollow_grove::HollowGrove;
use crate::point::Point;
use crate::triway::Triway;

#[derive(Debug, Clone)]
pub struct KernelPass {
    start: Point,
    triway: Triway,
    hollow_grove: HollowGrove,
    current_seam: CurrentSeam,
    aura_beam: AuraBeam,
    landed: Point,
}

impl KernelPass {
    pub fn new(
        start: Point,
        triway: Triway,
        hollow_grove: HollowGrove,
        current_seam: CurrentSeam,
        aura_beam: AuraBeam,
        landed: Point,
    ) -> Self {
        Self {
            start,
            triway,
            hollow_grove,
            current_seam,
            aura_beam,
            landed,
        }
    }

    pub fn start_point(&self) -> &Point {
        &self.start
    }

    pub fn triway(&self) -> &Triway {
        &self.triway
    }

    pub fn hollow_grove(&self) -> &HollowGrove {
        &self.hollow_grove
    }

    pub fn current_seam(&self) -> &CurrentSeam {
        &self.current_seam
    }

    pub fn aura_beam(&self) -> &AuraBeam {
        &self.aura_beam
    }

    pub fn landed_point(&self) -> &Point {
        &self.landed
    }
}

impl fmt::Display for KernelPass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "start Point\n↓\nTriway\n↓\nHollowGrove\n↓\nCurrentSeam\n↓\nAuraBeam\n↓\nlanded Point"
        )
    }
}
