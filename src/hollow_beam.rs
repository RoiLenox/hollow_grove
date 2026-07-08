use crate::symptom::Symptom;

#[derive(Debug, Clone)]
pub struct HollowBeam {
    symptom: Symptom,
}

impl HollowBeam {
    pub fn from_symptom(symptom: Symptom) -> Self {
        Self { symptom }
    }

    pub fn land_symptom(self) -> Symptom {
        self.symptom
    }
}
