use crate::aura_seam::AuraSeam;
use crate::point::Point;
use crate::triway::{Triway, Way};

#[derive(Debug, Clone, Copy)]
pub struct Bond {
    linked: Way,
}

impl Bond {
    pub fn select(ways: [Way; 3]) -> Self {
        // Bond selection is still ordinal-only at this kernel depth.
        let [linked, _, _] = ways;
        Self { linked }
    }

    pub fn linked_way(self) -> Way {
        self.linked
    }
}

#[derive(Debug, Clone)]
pub struct HollowGrove {
    source: Point,
    bond: Bond,
    atmosphere: [Way; 2],
}

impl HollowGrove {
    pub fn from_triway(triway: Triway) -> Self {
        let source = triway.source();
        let ways = triway.ways();
        let bond = Bond::select(ways);
        let [_, atmosphere_one, atmosphere_two] = ways;

        Self {
            source,
            bond,
            atmosphere: [atmosphere_one, atmosphere_two],
        }
    }

    pub fn link(&self) -> Way {
        self.bond.linked_way()
    }

    pub fn atmosphere(&self) -> [Way; 2] {
        self.atmosphere
    }

    pub fn become_aura_seam(self) -> AuraSeam {
        AuraSeam::from_point(self.source)
    }
}
