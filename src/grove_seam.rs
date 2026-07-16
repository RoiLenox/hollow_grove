use std::fmt;

use crate::hollow_beam::HollowBeam;
use crate::pleb_meta::{ExteriorShape, RoutingPass, Sequence};
use crate::symptom::Symptom;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeamRoute {
    PlebExterior,
    MetaExterior,
}

impl fmt::Display for SeamRoute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PlebExterior => f.write_str("PlebExterior"),
            Self::MetaExterior => f.write_str("MetaExterior"),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct GroveSeam {
    symptom: Symptom,
    routing: RoutingPass,
    route: SeamRoute,
}

impl GroveSeam {
    pub fn from_symptom(symptom: Symptom, routing: RoutingPass) -> Self {
        let route = match routing.pleb_meta().exterior().shape() {
            ExteriorShape::Straight => SeamRoute::PlebExterior,
            ExteriorShape::Curved => SeamRoute::MetaExterior,
        };

        Self {
            symptom,
            routing,
            route,
        }
    }

    pub fn achieve_hollow_beam(self) -> HollowBeam {
        let interior_sequence = match self.route {
            SeamRoute::PlebExterior => Sequence::Blep,
            SeamRoute::MetaExterior => Sequence::Atem,
        };

        HollowBeam::from_symptom(self.symptom, self.routing, self.route, interior_sequence)
    }

    pub const fn routing(&self) -> &RoutingPass {
        &self.routing
    }

    pub const fn route(&self) -> SeamRoute {
        self.route
    }
}

impl fmt::Debug for GroveSeam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GroveSeam")
            .field("symptom", &self.symptom)
            .finish()
    }
}
