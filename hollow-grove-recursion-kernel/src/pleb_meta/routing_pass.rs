use crate::pleb_meta::{PlebMetaGrammar, PlebMetaInput, PlebMetaRouting};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RoutingPass {
    pleb_meta: PlebMetaRouting,
}

impl RoutingPass {
    pub const fn new(input: PlebMetaInput) -> Self {
        Self {
            pleb_meta: PlebMetaGrammar::route(input),
        }
    }

    pub const fn pleb_meta(&self) -> &PlebMetaRouting {
        &self.pleb_meta
    }
}
