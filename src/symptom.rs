use crate::point::Point;
use crate::triway::Triway;

#[derive(Debug, Clone)]
pub struct Symptom {
    point: Point,
}

impl Symptom {
    pub const fn new(point: Point) -> Self {
        Self { point }
    }

    pub const fn origin() -> Self {
        Self::new(Point)
    }

    pub fn point(&self) -> &Point {
        &self.point
    }

    pub fn into_point(self) -> Point {
        self.point
    }

    pub fn become_triway(self) -> Triway {
        Triway::from_symptom(self)
    }
}
