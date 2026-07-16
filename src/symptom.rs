use crate::frame_state::FrameState;
use crate::point::Point;
use crate::triway::Triway;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symptom {
    point: Point,
}

impl Symptom {
    pub fn new(point: Point) -> Self {
        Self { point }
    }

    pub fn origin() -> Self {
        Self::new(Point::origin())
    }

    pub fn point(&self) -> &Point {
        &self.point
    }

    pub const fn frame_state(&self) -> &FrameState {
        self.point.frame_state()
    }

    pub fn into_point(self) -> Point {
        self.point
    }

    pub fn become_triway(self) -> Triway {
        Triway::from_symptom(self)
    }
}
