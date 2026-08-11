#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Sequence {
    Pleb,
    Blep,
    Meta,
    Atem,
}

impl Sequence {
    pub const fn complement(self) -> Self {
        match self {
            Self::Pleb => Self::Blep,
            Self::Blep => Self::Pleb,
            Self::Meta => Self::Atem,
            Self::Atem => Self::Meta,
        }
    }
}
