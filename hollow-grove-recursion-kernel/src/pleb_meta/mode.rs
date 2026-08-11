#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mode {
    Pathos,
    Logos,
    Ethos,
    Bathos,
}

impl Mode {
    pub const fn complement(self) -> Self {
        match self {
            Self::Pathos => Self::Bathos,
            Self::Bathos => Self::Pathos,
            Self::Logos => Self::Ethos,
            Self::Ethos => Self::Logos,
        }
    }
}
