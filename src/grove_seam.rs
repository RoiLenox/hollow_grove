use crate::hollow_beam::HollowBeam;
use crate::symptom::Symptom;

#[derive(Debug, Clone)]
pub struct GroveSeam {
    symptom: Symptom,
}

impl GroveSeam {
    pub fn from_symptom(symptom: Symptom) -> Self {
        Self { symptom }
    }

    pub fn achieve_hollow_beam(self) -> HollowBeam {
        HollowBeam::from_symptom(self.symptom)
    }
}
