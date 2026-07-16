use std::fmt;

use crate::frame_state::{BeingId, FrameState};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Point {
    frame_state: FrameState,
}

impl Point {
    pub fn new(frame_state: FrameState) -> Self {
        Self { frame_state }
    }

    pub fn origin() -> Self {
        Self::new(FrameState::origin())
    }

    pub const fn frame_state(&self) -> &FrameState {
        &self.frame_state
    }

    pub const fn being(&self) -> BeingId {
        self.frame_state.being()
    }

    pub fn into_frame_state(self) -> FrameState {
        self.frame_state
    }
}

impl Default for Point {
    fn default() -> Self {
        Self::origin()
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Point")
    }
}
