use crate::aura_beam::AuraBeam;
use crate::point::Point;

#[derive(Debug, Clone)]
pub struct CurrentSeam {
    point: Point,
}

impl CurrentSeam {
    pub fn from_point(point: Point) -> Self {
        Self { point }
    }

    pub fn project_aura_beam(self) -> AuraBeam {
        AuraBeam::from_point(self.point)
    }
}
