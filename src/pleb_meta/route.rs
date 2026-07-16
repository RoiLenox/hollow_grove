use crate::pleb_meta::{Mode, Operator, Sequence};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExteriorShape {
    Straight,
    Curved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExteriorState {
    shape: ExteriorShape,
    foreground_sequence: Sequence,
    operator: Operator,
}

impl ExteriorState {
    pub(crate) const fn new(
        shape: ExteriorShape,
        foreground_sequence: Sequence,
        operator: Operator,
    ) -> Self {
        Self {
            shape,
            foreground_sequence,
            operator,
        }
    }

    pub const fn interior(self) -> InteriorState {
        InteriorState::new(
            self.foreground_sequence,
            self.foreground_sequence.complement(),
            Operator::Cleopatra,
        )
    }

    pub const fn shape(self) -> ExteriorShape {
        self.shape
    }

    pub const fn foreground_sequence(self) -> Sequence {
        self.foreground_sequence
    }

    pub const fn operator(self) -> Operator {
        self.operator
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InteriorState {
    complement_of: Sequence,
    sequence: Sequence,
    operator: Operator,
}

impl InteriorState {
    pub(crate) const fn new(
        complement_of: Sequence,
        sequence: Sequence,
        operator: Operator,
    ) -> Self {
        Self {
            complement_of,
            sequence,
            operator,
        }
    }

    pub const fn complement_of(self) -> Sequence {
        self.complement_of
    }

    pub const fn sequence(self) -> Sequence {
        self.sequence
    }

    pub const fn operator(self) -> Operator {
        self.operator
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StrandState {
    sequence: Sequence,
    mode: Mode,
    operator: Operator,
}

impl StrandState {
    pub const fn new(sequence: Sequence, mode: Mode) -> Self {
        let operator = match sequence {
            Sequence::Pleb => Operator::Clouseau,
            Sequence::Meta => Operator::Hal,
            Sequence::Blep | Sequence::Atem => Operator::Cleopatra,
        };

        Self {
            sequence,
            mode,
            operator,
        }
    }

    pub const fn sequence(self) -> Sequence {
        self.sequence
    }

    pub const fn mode(self) -> Mode {
        self.mode
    }

    pub const fn operator(self) -> Operator {
        self.operator
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlebMetaInput {
    pub exterior_shape: ExteriorShape,
    pub pleb_mode: Mode,
    pub meta_mode: Mode,
}

impl Default for PlebMetaInput {
    fn default() -> Self {
        Self {
            exterior_shape: ExteriorShape::Straight,
            pleb_mode: Mode::Pathos,
            meta_mode: Mode::Logos,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlebMetaRouting {
    bond: crate::pleb_meta::PlebMetaBond,
    pleb: StrandState,
    meta: StrandState,
    blep: StrandState,
    atem: StrandState,
    exterior: ExteriorState,
    interior: InteriorState,
}

impl PlebMetaRouting {
    pub(crate) const fn new(
        bond: crate::pleb_meta::PlebMetaBond,
        pleb: StrandState,
        meta: StrandState,
        blep: StrandState,
        atem: StrandState,
        exterior: ExteriorState,
        interior: InteriorState,
    ) -> Self {
        Self {
            bond,
            pleb,
            meta,
            blep,
            atem,
            exterior,
            interior,
        }
    }

    pub const fn bond(self) -> crate::pleb_meta::PlebMetaBond {
        self.bond
    }

    pub const fn pleb(self) -> StrandState {
        self.pleb
    }

    pub const fn meta(self) -> StrandState {
        self.meta
    }

    pub const fn blep(self) -> StrandState {
        self.blep
    }

    pub const fn atem(self) -> StrandState {
        self.atem
    }

    pub const fn exterior(self) -> ExteriorState {
        self.exterior
    }

    pub const fn interior(self) -> InteriorState {
        self.interior
    }
}
