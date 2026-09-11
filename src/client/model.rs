//! Renderer-neutral actor and temporary test-world state.

use std::time::Duration;

use crate::House;
use crate::gameplay::{CardinalDirection, TilePosition};

pub const LOGICAL_WIDTH: u32 = 640;
pub const LOGICAL_HEIGHT: u32 = 360;
pub const TILE_SIZE: u32 = 16;
pub const WALK_POSE_DURATION: Duration = Duration::from_millis(140);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActorBodyIdentity {
    Hueman,
    Nature { nature_id: String },
}

/// Independent canonical axes used to resolve presentation without encoding
/// biological/social combinations as new actor species.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActorVisualIdentity {
    pub body: ActorBodyIdentity,
    pub house: Option<House>,
    pub culture_id: Option<String>,
    pub current_form_id: Option<String>,
    pub aura_set_id: Option<String>,
    pub institution_id: Option<String>,
    pub status_id: Option<String>,
    pub sect_or_order_id: Option<String>,
    pub organization_id: Option<String>,
    pub alias_id: Option<String>,
}

impl ActorVisualIdentity {
    #[must_use]
    pub const fn hueman() -> Self {
        Self {
            body: ActorBodyIdentity::Hueman,
            house: None,
            culture_id: None,
            current_form_id: None,
            aura_set_id: None,
            institution_id: None,
            status_id: None,
            sect_or_order_id: None,
            organization_id: None,
            alias_id: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CollisionFootprint {
    pub width_tiles: u16,
    pub height_tiles: u16,
}

impl CollisionFootprint {
    pub const ONE_TILE: Self = Self {
        width_tiles: 1,
        height_tiles: 1,
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MovementState {
    Idle,
    Walking,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalkFrame {
    A,
    B,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveOutcome {
    Accepted,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActorState {
    pub identity: ActorVisualIdentity,
    pub position: TilePosition,
    pub movement: MovementState,
    pub walk_frame: WalkFrame,
    pub collision: CollisionFootprint,
    accepted_steps: u64,
}

impl ActorState {
    #[must_use]
    pub const fn hueman(position: TilePosition) -> Self {
        Self {
            identity: ActorVisualIdentity::hueman(),
            position,
            movement: MovementState::Idle,
            walk_frame: WalkFrame::A,
            collision: CollisionFootprint::ONE_TILE,
            accepted_steps: 0,
        }
    }

    pub fn try_move(&mut self, direction: CardinalDirection, world: &TestWorld) -> MoveOutcome {
        self.position.facing = direction;
        let (offset_x, offset_y) = direction.offset();
        let target_x = i32::from(self.position.x) + offset_x;
        let target_y = i32::from(self.position.y) + offset_y;
        if !world.footprint_is_walkable(target_x, target_y, self.collision) {
            self.movement = MovementState::Idle;
            return MoveOutcome::Blocked;
        }

        self.position = TilePosition {
            x: u16::try_from(target_x).expect("walkable x is nonnegative"),
            y: u16::try_from(target_y).expect("walkable y is nonnegative"),
            facing: direction,
        };
        self.accepted_steps += 1;
        self.walk_frame = if self.accepted_steps.is_multiple_of(2) {
            WalkFrame::B
        } else {
            WalkFrame::A
        };
        self.movement = MovementState::Walking;
        MoveOutcome::Accepted
    }

    pub fn finish_step(&mut self) {
        self.movement = MovementState::Idle;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileKind {
    Ground,
    Path,
    Blocked,
}

impl TileKind {
    #[must_use]
    pub const fn is_walkable(self) -> bool {
        !matches!(self, Self::Blocked)
    }
}

/// Small original test space. Its procedural layout is intentionally temporary
/// and contains no imported map art or lore authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TestWorld {
    pub width_tiles: u16,
    pub height_tiles: u16,
}

impl TestWorld {
    pub const WIDTH: u16 = 56;
    pub const HEIGHT: u16 = 36;
    pub const SPAWN: TilePosition = TilePosition {
        x: 28,
        y: 18,
        facing: CardinalDirection::South,
    };

    #[must_use]
    pub const fn new() -> Self {
        Self {
            width_tiles: Self::WIDTH,
            height_tiles: Self::HEIGHT,
        }
    }

    #[must_use]
    pub const fn width_pixels(self) -> u32 {
        self.width_tiles as u32 * TILE_SIZE
    }

    #[must_use]
    pub const fn height_pixels(self) -> u32 {
        self.height_tiles as u32 * TILE_SIZE
    }

    #[must_use]
    pub const fn tile(self, x: i32, y: i32) -> TileKind {
        if x < 0
            || y < 0
            || x >= self.width_tiles as i32
            || y >= self.height_tiles as i32
            || x == 0
            || y == 0
            || x == self.width_tiles as i32 - 1
            || y == self.height_tiles as i32 - 1
        {
            return TileKind::Blocked;
        }

        if (x == 18 && y >= 7 && y <= 14)
            || (x == 38 && y >= 21 && y <= 29)
            || (y == 9 && x >= 34 && x <= 42)
            || (y == 27 && x >= 11 && x <= 19)
        {
            return TileKind::Blocked;
        }

        if x == self.width_tiles as i32 / 2 || y == self.height_tiles as i32 / 2 {
            TileKind::Path
        } else {
            TileKind::Ground
        }
    }

    #[must_use]
    pub fn footprint_is_walkable(
        self,
        origin_x: i32,
        origin_y: i32,
        footprint: CollisionFootprint,
    ) -> bool {
        (0..i32::from(footprint.height_tiles)).all(|offset_y| {
            (0..i32::from(footprint.width_tiles)).all(|offset_x| {
                self.tile(origin_x + offset_x, origin_y + offset_y)
                    .is_walkable()
            })
        })
    }
}

impl Default for TestWorld {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientWorldState {
    pub world: TestWorld,
    pub hueman: ActorState,
}

impl ClientWorldState {
    #[must_use]
    pub const fn new() -> Self {
        let world = TestWorld::new();
        Self {
            world,
            hueman: ActorState::hueman(TestWorld::SPAWN),
        }
    }

    pub fn move_hueman(&mut self, direction: CardinalDirection) -> MoveOutcome {
        self.hueman.try_move(direction, &self.world)
    }

    pub fn finish_hueman_step(&mut self) {
        self.hueman.finish_step();
    }
}

impl Default for ClientWorldState {
    fn default() -> Self {
        Self::new()
    }
}
