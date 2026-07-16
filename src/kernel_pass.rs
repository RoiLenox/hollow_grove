use std::fmt;

use crate::frame_state::FrameState;
use crate::grove_seam::GroveSeam;
use crate::hollow_beam::{HollowBeam, LandedSymptom};
use crate::hollow_grove::HollowGrove;
use crate::pleb_meta::{PlebMetaInput, RoutingPass};
use crate::point::Point;
use crate::symptom::Symptom;
use crate::triway::Triway;

pub const START_WITNESS_LABEL: &str = "Point";
pub const FOURWAY_WITNESS_LABEL: &str = "Fourway";
pub const CURRENT_SEAM_WITNESS_LABEL: &str = "CurrentSeam";
pub const AURA_BEAM_WITNESS_LABEL: &str = "AuraBeam";
pub const LANDED_WITNESS_LABEL: &str = "Point²";
pub const LANDED_WITNESS_DESCRIPTION: &str = "Landed Point";
pub const CANONICAL_WITNESS: &str = "Point\n↓\nTriway\n↓\nFourway\n↓\nHollowGrove\n↓\nCurrentSeam [PlebExterior]\n↓\nAuraBeam [BlepReturn]\n↓\nPoint² (Landed Point) [BlepArrival]";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KernelInput {
    pub routing: PlebMetaInput,
}

impl Default for KernelInput {
    fn default() -> Self {
        Self {
            routing: PlebMetaInput::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KernelPass {
    start: Symptom,
    triway: Triway,
    hollow_grove: HollowGrove,
    grove_seam: GroveSeam,
    hollow_beam: HollowBeam,
    landed: LandedSymptom,
    routing: RoutingPass,
    canonical_witness: String,
}

fn render_canonical_witness(
    grove_seam_route: impl fmt::Display,
    hollow_beam_route: impl fmt::Display,
    landing_route: impl fmt::Display,
) -> String {
    format!(
        "{START_WITNESS_LABEL}\n\
         ↓\n\
         Triway\n\
         ↓\n\
         {FOURWAY_WITNESS_LABEL}\n\
         ↓\n\
         HollowGrove\n\
         ↓\n\
         {CURRENT_SEAM_WITNESS_LABEL} [{grove_seam_route}]\n\
         ↓\n\
         {AURA_BEAM_WITNESS_LABEL} [{hollow_beam_route}]\n\
         ↓\n\
         {LANDED_WITNESS_LABEL} ({LANDED_WITNESS_DESCRIPTION}) [{landing_route}]"
    )
}

impl KernelPass {
    pub fn new(
        start: Symptom,
        triway: Triway,
        hollow_grove: HollowGrove,
        grove_seam: GroveSeam,
        hollow_beam: HollowBeam,
        landed: LandedSymptom,
    ) -> Self {
        let routing = *hollow_beam.routing();
        let canonical_witness =
            render_canonical_witness(grove_seam.route(), hollow_beam.route(), landed.route());

        Self {
            start,
            triway,
            hollow_grove,
            grove_seam,
            hollow_beam,
            landed,
            routing,
            canonical_witness,
        }
    }

    pub fn start_symptom(&self) -> &Symptom {
        &self.start
    }

    pub fn start_point(&self) -> &Point {
        self.start.point()
    }

    pub const fn start_frame_state(&self) -> &FrameState {
        self.start.frame_state()
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

    pub fn landed(&self) -> &LandedSymptom {
        &self.landed
    }

    pub fn landed_symptom(&self) -> &Symptom {
        self.landed.symptom()
    }

    pub fn end_point(&self) -> &Point {
        self.landed.point()
    }

    pub const fn end_frame_state(&self) -> &FrameState {
        self.landed.frame_state()
    }

    pub fn routing(&self) -> &RoutingPass {
        &self.routing
    }

    pub fn canonical_witness(&self) -> &str {
        &self.canonical_witness
    }
}

impl fmt::Display for KernelPass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.canonical_witness())
    }
}
