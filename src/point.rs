use std::fmt;

use crate::composition::{
    AxisHandedness, OrientedPoint, PhysicalExtent, PhysicalPosition, PointCenterId, PointId,
    PolarityAxis, PoleId, ScaleKey, SpatialEvidenceId,
};
use crate::frame_state::{BeingId, FrameState};
use crate::point_progression::{PointProgressionState, ReachableWorldState};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Point {
    frame_state: FrameState,
    progression: PointProgressionState,
    world: ReachableWorldState,
    physical: OrientedPoint,
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
        Self::with_physical_domain_state(
            frame_state,
            progression,
            world,
            canonical_origin_physical_point(),
        )
    }

    pub fn with_physical_domain_state(
        frame_state: FrameState,
        progression: PointProgressionState,
        world: ReachableWorldState,
        physical: OrientedPoint,
    ) -> Self {
        physical
            .validate()
            .expect("a recursion Point requires lawful oriented physical state");
        Self {
            frame_state,
            progression,
            world,
            physical,
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

    pub const fn physical(&self) -> &OrientedPoint {
        &self.physical
    }

    pub const fn being(&self) -> BeingId {
        self.frame_state.being()
    }

    pub fn into_frame_state(self) -> FrameState {
        self.frame_state
    }

    pub fn with_frame_state_preserving_domain(&self, frame_state: FrameState) -> Self {
        Self::with_physical_domain_state(
            frame_state,
            self.progression.clone(),
            self.world.clone(),
            self.physical.clone(),
        )
    }
}

fn canonical_origin_physical_point() -> OrientedPoint {
    OrientedPoint {
        point_id: PointId::new("point.recursion.origin").expect("canonical recursion Point ID"),
        center_id: PointCenterId::new("center.recursion.origin")
            .expect("canonical recursion center ID"),
        center: PhysicalPosition::origin(),
        orientation: PolarityAxis::new([0, 1, 0], AxisHandedness::RightHanded)
            .expect("canonical recursion polarity axis"),
        positive_pole_id: PoleId::new("pole.recursion.positive")
            .expect("canonical recursion positive pole ID"),
        negative_pole_id: PoleId::new("pole.recursion.negative")
            .expect("canonical recursion negative pole ID"),
        scale: ScaleKey::new("scale.entity").expect("canonical recursion scale"),
        extent: PhysicalExtent::new(1).expect("canonical recursion physical extent"),
        evidence_ids: [
            SpatialEvidenceId::new("evidence.recursion.oriented-point-origin")
                .expect("canonical recursion Point evidence"),
        ]
        .into_iter()
        .collect(),
        provenance_ids: Default::default(),
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
