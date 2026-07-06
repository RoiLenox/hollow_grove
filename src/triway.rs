use crate::point::Point;
use crate::hollow_grove::HollowGrove;

#[derive(Debug, Clone)]
pub struct Triway {
    point: Point,
}

impl Triway {
    pub fn from_point(point: Point) -> Self {
        Self { point }
    }

    pub fn become_hollow_grove(self) -> HollowGrove {
        HollowGrove::from_triway(self)
    }

    pub fn into_point(self) -> Point {
        self.point
    }
}
