use crate::point::Point;

#[derive(Debug, Clone)]
pub struct AuraBeam {
    point: Point,
}

impl AuraBeam {
    pub fn from_point(point: Point) -> Self {
        Self { point }
    }

    pub fn land_point(self) -> Point {
        self.point
    }
}
