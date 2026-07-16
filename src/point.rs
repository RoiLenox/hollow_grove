use std::fmt;

use crate::frame_state::{BeingId, FrameState};
use crate::point_progression::{PointProgressionState, ReachableWorldState};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Point {
    frame_state: FrameState,
    progression: PointProgressionState,
    world: ReachableWorldState,
}

impl Point {
    pub fn new(frame_state: FrameState) -> Self {
        Self::with_domain_state(
            frame_state,
            PointProgressionState::origin(),
            ReachableWorldState::origin(),
        )
    }

    pub fn with_domain_state(
        frame_state: FrameState,
        progression: PointProgressionState,
        world: ReachableWorldState,
    ) -> Self {
        Self {
            frame_state,
            progression,
            world,
        }
    }

    pub fn origin() -> Self {
        Self::new(FrameState::origin())
    }

    pub const fn frame_state(&self) -> &FrameState {
        &self.frame_state
    }

    pub const fn progression(&self) -> &PointProgressionState {
        &self.progression
    }

    pub const fn world(&self) -> &ReachableWorldState {
        &self.world
    }

    pub const fn being(&self) -> BeingId {
        self.frame_state.being()
    }

    pub fn into_frame_state(self) -> FrameState {
        self.frame_state
    }

    pub fn with_frame_state_preserving_domain(&self, frame_state: FrameState) -> Self {
        Self::with_domain_state(frame_state, self.progression.clone(), self.world.clone())
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
