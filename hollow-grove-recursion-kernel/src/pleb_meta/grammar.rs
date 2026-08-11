use crate::pleb_meta::{
    ExteriorShape, ExteriorState, Mode, PlebMetaBond, PlebMetaInput, PlebMetaRouting, StrandState,
};
use crate::pleb_meta::{Operator, Sequence};

#[derive(Debug, Default, Clone, Copy)]
pub struct PlebMetaGrammar;

impl PlebMetaGrammar {
    pub const fn exterior_state(input: PlebMetaInput) -> ExteriorState {
        match input.exterior_shape {
            ExteriorShape::Straight => {
                ExteriorState::new(ExteriorShape::Straight, Sequence::Pleb, Operator::Clouseau)
            }
            ExteriorShape::Curved => {
                ExteriorState::new(ExteriorShape::Curved, Sequence::Meta, Operator::Hal)
            }
        }
    }

    pub const fn route(input: PlebMetaInput) -> PlebMetaRouting {
        let pleb = StrandState::new(Sequence::Pleb, input.pleb_mode);
        let blep = StrandState::new(Sequence::Blep, input.pleb_mode.complement());
        let meta = StrandState::new(Sequence::Meta, input.meta_mode);
        let atem = StrandState::new(Sequence::Atem, input.meta_mode.complement());
        let exterior = Self::exterior_state(input);
        let interior = exterior.interior();
        let bond = PlebMetaBond::new(input.pleb_mode, input.meta_mode);

        PlebMetaRouting::new(bond, pleb, meta, blep, atem, exterior, interior)
    }
}

pub const fn normal_response(incoming: Mode) -> Mode {
    incoming.complement()
}
