use crate::hollow_grove::HollowGrove;
use crate::point::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Way {
    One,
    Two,
    Three,
}

#[derive(Debug, Clone)]
pub struct Triway {
    source: Point,
    ways: [Way; 3],
}

impl Triway {
    pub fn from_point(point: Point) -> Self {
        Self {
            source: point,
            ways: [Way::One, Way::Two, Way::Three],
        }
    }

    pub fn become_hollow_grove(self) -> HollowGrove {
        HollowGrove::from_triway(self)
    }

    pub fn ways(&self) -> [Way; 3] {
        self.ways
    }

    pub fn source(&self) -> Point {
        self.source.clone()
    }

    pub fn into_point(self) -> Point {
        self.source
    }
}
