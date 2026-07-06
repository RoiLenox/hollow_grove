use crate::point::Point;

#[derive(Debug, Clone)]
pub struct AuraSeam {
    point: Point,
}

impl AuraSeam {
    pub fn from_point(point: Point) -> Self {
        Self { point }
    }

    pub fn create_point(self) -> Point {
        self.point
    }
}
