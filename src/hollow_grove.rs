use crate::aura_seam::AuraSeam;
use crate::triway::Triway;

#[derive(Debug, Clone)]
pub struct HollowGrove {
    triway: Triway,
}

impl HollowGrove {
    pub fn from_triway(triway: Triway) -> Self {
        Self { triway }
    }

    pub fn become_aura_seam(self) -> AuraSeam {
        AuraSeam::from_point(self.triway.into_point())
    }
}
