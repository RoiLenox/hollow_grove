use crate::triway::Triway;

#[derive(Debug, Clone)]
pub struct Point;

impl Point {
    pub fn become_triway(self) -> Triway {
        Triway::from_point(self)
    }
}
