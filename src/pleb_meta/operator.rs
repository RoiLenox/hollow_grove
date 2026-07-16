use crate::pleb_meta::Sequence;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operator {
    Clouseau,
    Hal,
    Cleopatra,
}

impl Operator {
    pub const fn handles(self, sequence: Sequence) -> bool {
        match self {
            Self::Clouseau => matches!(sequence, Sequence::Pleb),
            Self::Hal => matches!(sequence, Sequence::Meta),
            Self::Cleopatra => matches!(sequence, Sequence::Blep | Sequence::Atem),
        }
    }
}
