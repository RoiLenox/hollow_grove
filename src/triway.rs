use crate::hollow_grove::HollowGrove;
use crate::symptom::Symptom;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Way {
    One,
    Two,
    Three,
}

#[derive(Debug, Clone)]
pub struct Triway {
    source: Symptom,
    ways: [Way; 3],
}

impl Triway {
    pub fn from_symptom(symptom: Symptom) -> Self {
        Self {
            source: symptom,
            ways: [Way::One, Way::Two, Way::Three],
        }
    }

    pub fn become_hollow_grove(self) -> HollowGrove {
        HollowGrove::from_triway(self)
    }

    pub fn ways(&self) -> [Way; 3] {
        self.ways
    }

    pub fn source(&self) -> Symptom {
        self.source.clone()
    }

    pub fn into_symptom(self) -> Symptom {
        self.source
    }
}
