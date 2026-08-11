use crate::pleb_meta::Mode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlebMetaBond {
    pleb_mode: Mode,
    meta_mode: Mode,
}

impl PlebMetaBond {
    pub(crate) const fn new(pleb_mode: Mode, meta_mode: Mode) -> Self {
        Self {
            pleb_mode,
            meta_mode,
        }
    }

    pub const fn pleb_mode(self) -> Mode {
        self.pleb_mode
    }

    pub const fn meta_mode(self) -> Mode {
        self.meta_mode
    }
}
