use std::fmt;

use crate::frame_state::FrameState;
use crate::grove_seam::SeamRoute;
use crate::pleb_meta::{RoutingPass, Sequence};
use crate::point::Point;
use crate::symptom::Symptom;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BeamRoute {
    BlepReturn,
    AtemReturn,
}

impl fmt::Display for BeamRoute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BlepReturn => f.write_str("BlepReturn"),
            Self::AtemReturn => f.write_str("AtemReturn"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LandingRoute {
    BlepArrival,
    AtemArrival,
}

impl fmt::Display for LandingRoute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BlepArrival => f.write_str("BlepArrival"),
            Self::AtemArrival => f.write_str("AtemArrival"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LandedSymptom {
    symptom: Symptom,
    route: LandingRoute,
}

impl LandedSymptom {
    const fn new(symptom: Symptom, route: LandingRoute) -> Self {
        Self { symptom, route }
    }

    pub const fn route(&self) -> LandingRoute {
        self.route
    }

    pub fn symptom(&self) -> &Symptom {
        &self.symptom
    }

    pub fn point(&self) -> &Point {
        self.symptom.point()
    }

    pub const fn frame_state(&self) -> &FrameState {
        self.symptom.frame_state()
    }

    pub fn next_point(&self) -> Point {
        self.point().clone()
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct HollowBeam {
    symptom: Symptom,
    routing: RoutingPass,
    seam_route: SeamRoute,
    route: BeamRoute,
}

impl HollowBeam {
    pub fn from_symptom(
        symptom: Symptom,
        routing: RoutingPass,
        seam_route: SeamRoute,
        interior_sequence: Sequence,
    ) -> Self {
        let route = match interior_sequence {
            Sequence::Blep => BeamRoute::BlepReturn,
            Sequence::Atem => BeamRoute::AtemReturn,
            Sequence::Pleb | Sequence::Meta => unreachable!("interior route must be complementary"),
        };

        Self {
            symptom,
            routing,
            seam_route,
            route,
        }
    }

    pub fn land_symptom(self) -> LandedSymptom {
        let Self {
            symptom,
            routing: _,
            seam_route: _,
            route,
        } = self;

        let landing_route = match route {
            BeamRoute::BlepReturn => LandingRoute::BlepArrival,
            BeamRoute::AtemReturn => LandingRoute::AtemArrival,
        };

        LandedSymptom::new(symptom, landing_route)
    }

    pub const fn routing(&self) -> &RoutingPass {
        &self.routing
    }

    pub const fn seam_route(&self) -> SeamRoute {
        self.seam_route
    }

    pub const fn route(&self) -> BeamRoute {
        self.route
    }
}

impl fmt::Debug for HollowBeam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HollowBeam")
            .field("symptom", &self.symptom)
            .finish()
    }
}
