use crate::aura_seam::AuraSeam;
use crate::point::Point;
use crate::triway::{Triway, Way};

#[derive(Debug, Clone)]
pub struct HollowGrove {
    source: Point,
    link: Way,
    atmosphere: [Way; 2],
}

impl HollowGrove {
    pub fn from_triway(triway: Triway) -> Self {
        let source = triway.source();
        let [link, atmosphere_one, atmosphere_two] = triway.ways();

        Self {
            source,
            link,
            atmosphere: [atmosphere_one, atmosphere_two],
        }
    }

    pub fn link(&self) -> Way {
        self.link
    }

    pub fn atmosphere(&self) -> [Way; 2] {
        self.atmosphere
    }

    pub fn become_aura_seam(self) -> AuraSeam {
        AuraSeam::from_point(self.source)
    }
}
