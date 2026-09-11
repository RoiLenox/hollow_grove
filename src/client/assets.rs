//! Semantic sprite lookup and deterministic prototype atlas generation.

use crate::gameplay::CardinalDirection;

use super::model::{MovementState, WalkFrame};

pub const ATLAS_WIDTH: u32 = 64;
pub const ATLAS_HEIGHT: u32 = 64;
pub const SPRITE_FRAME_SIZE: u32 = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SpriteKey {
    /// Original programmer art; replace through this key rather than changing gameplay state.
    HuemanOverworldPrototype,
    TestGround,
    TestPath,
    TestBlocked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AtlasRegion {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl AtlasRegion {
    #[must_use]
    pub fn normalized(self) -> [f32; 4] {
        [
            self.x as f32 / ATLAS_WIDTH as f32,
            self.y as f32 / ATLAS_HEIGHT as f32,
            (self.x + self.width) as f32 / ATLAS_WIDTH as f32,
            (self.y + self.height) as f32 / ATLAS_HEIGHT as f32,
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrototypeAtlas {
    pub width: u32,
    pub height: u32,
    pub rgba8: Vec<u8>,
}

#[must_use]
pub fn resolve_region(
    key: SpriteKey,
    facing: CardinalDirection,
    movement: MovementState,
    walk_frame: WalkFrame,
) -> AtlasRegion {
    match key {
        SpriteKey::HuemanOverworldPrototype => {
            let column = match movement {
                MovementState::Idle => 0,
                MovementState::Walking => match walk_frame {
                    WalkFrame::A => 1,
                    WalkFrame::B => 2,
                },
            };
            let row = match facing {
                CardinalDirection::South => 0,
                CardinalDirection::West => 1,
                CardinalDirection::East => 2,
                CardinalDirection::North => 3,
            };
            AtlasRegion {
                x: column * SPRITE_FRAME_SIZE,
                y: row * SPRITE_FRAME_SIZE,
                width: SPRITE_FRAME_SIZE,
                height: SPRITE_FRAME_SIZE,
            }
        }
        SpriteKey::TestGround => tile_region(0),
        SpriteKey::TestPath => tile_region(1),
        SpriteKey::TestBlocked => tile_region(2),
    }
}

const fn tile_region(row: u32) -> AtlasRegion {
    AtlasRegion {
        x: 48,
        y: row * SPRITE_FRAME_SIZE,
        width: SPRITE_FRAME_SIZE,
        height: SPRITE_FRAME_SIZE,
    }
}

#[must_use]
pub fn build_prototype_atlas() -> PrototypeAtlas {
    let mut rgba8 = vec![0_u8; (ATLAS_WIDTH * ATLAS_HEIGHT * 4) as usize];
    for facing in [
        CardinalDirection::South,
        CardinalDirection::West,
        CardinalDirection::East,
        CardinalDirection::North,
    ] {
        for (column, pose) in [
            (0, MovementState::Idle),
            (1, MovementState::Walking),
            (2, MovementState::Walking),
        ] {
            let walk_frame = if column == 2 {
                WalkFrame::B
            } else {
                WalkFrame::A
            };
            let region = resolve_region(
                SpriteKey::HuemanOverworldPrototype,
                facing,
                pose,
                walk_frame,
            );
            draw_hueman_frame(&mut rgba8, region.x, region.y, facing, column);
        }
    }
    draw_tile(&mut rgba8, tile_region(0), TilePalette::Ground);
    draw_tile(&mut rgba8, tile_region(1), TilePalette::Path);
    draw_tile(&mut rgba8, tile_region(2), TilePalette::Blocked);

    PrototypeAtlas {
        width: ATLAS_WIDTH,
        height: ATLAS_HEIGHT,
        rgba8,
    }
}

#[derive(Debug, Clone, Copy)]
enum TilePalette {
    Ground,
    Path,
    Blocked,
}

fn draw_tile(bytes: &mut [u8], region: AtlasRegion, palette: TilePalette) {
    let (base, accent, detail) = match palette {
        TilePalette::Ground => (
            [91, 139, 105, 255],
            [109, 157, 119, 255],
            [71, 111, 87, 255],
        ),
        TilePalette::Path => (
            [169, 158, 124, 255],
            [188, 179, 144, 255],
            [134, 124, 101, 255],
        ),
        TilePalette::Blocked => ([39, 67, 82, 255], [57, 89, 101, 255], [22, 42, 60, 255]),
    };
    for y in 0..region.height {
        for x in 0..region.width {
            let color = if matches!(palette, TilePalette::Blocked)
                && (x == 0 || y == 0 || x + 1 == region.width || y + 1 == region.height)
            {
                detail
            } else if (x + y * 3).is_multiple_of(11) {
                accent
            } else if (x * 5 + y).is_multiple_of(17) {
                detail
            } else {
                base
            };
            set_pixel(bytes, region.x + x, region.y + y, color);
        }
    }
}

fn draw_hueman_frame(
    bytes: &mut [u8],
    origin_x: u32,
    origin_y: u32,
    facing: CardinalDirection,
    pose: u32,
) {
    const OUTLINE: [u8; 4] = [15, 31, 51, 255];
    const SKIN: [u8; 4] = [220, 187, 143, 255];
    const SHIRT: [u8; 4] = [67, 117, 139, 255];
    const TROUSERS: [u8; 4] = [48, 67, 81, 255];
    const HIGHLIGHT: [u8; 4] = [190, 224, 215, 255];

    let head_left = if matches!(facing, CardinalDirection::East) {
        6
    } else {
        5
    };
    fill_rect(bytes, origin_x, origin_y, head_left, 1, 6, 6, OUTLINE);
    fill_rect(bytes, origin_x, origin_y, head_left + 1, 2, 4, 4, SKIN);
    fill_rect(bytes, origin_x, origin_y, head_left + 1, 1, 4, 2, OUTLINE);

    match facing {
        CardinalDirection::South => {
            pixel(bytes, origin_x, origin_y, head_left + 1, 4, OUTLINE);
            pixel(bytes, origin_x, origin_y, head_left + 4, 4, OUTLINE);
        }
        CardinalDirection::North => {
            fill_rect(bytes, origin_x, origin_y, head_left + 1, 3, 4, 3, OUTLINE);
        }
        CardinalDirection::East => {
            pixel(bytes, origin_x, origin_y, head_left + 4, 4, OUTLINE);
            pixel(bytes, origin_x, origin_y, head_left + 6, 4, SKIN);
        }
        CardinalDirection::West => {
            pixel(bytes, origin_x, origin_y, head_left + 1, 4, OUTLINE);
            pixel(
                bytes,
                origin_x,
                origin_y,
                head_left.saturating_sub(1),
                4,
                SKIN,
            );
        }
    }

    fill_rect(bytes, origin_x, origin_y, 5, 7, 6, 5, OUTLINE);
    fill_rect(bytes, origin_x, origin_y, 6, 7, 4, 4, SHIRT);
    fill_rect(bytes, origin_x, origin_y, 7, 7, 2, 1, HIGHLIGHT);

    let (left_arm_top, right_arm_top) = match pose {
        1 => (7, 9),
        2 => (9, 7),
        _ => (8, 8),
    };
    fill_rect(bytes, origin_x, origin_y, 4, left_arm_top, 1, 4, OUTLINE);
    fill_rect(bytes, origin_x, origin_y, 11, right_arm_top, 1, 4, OUTLINE);

    let (left_leg_bottom, right_leg_bottom) = match pose {
        1 => (15, 14),
        2 => (14, 15),
        _ => (15, 15),
    };
    fill_rect(
        bytes,
        origin_x,
        origin_y,
        5,
        12,
        2,
        left_leg_bottom - 11,
        TROUSERS,
    );
    fill_rect(
        bytes,
        origin_x,
        origin_y,
        9,
        12,
        2,
        right_leg_bottom - 11,
        TROUSERS,
    );
    fill_rect(bytes, origin_x, origin_y, 4, left_leg_bottom, 3, 1, OUTLINE);
    fill_rect(
        bytes,
        origin_x,
        origin_y,
        9,
        right_leg_bottom,
        3,
        1,
        OUTLINE,
    );
}

fn fill_rect(
    bytes: &mut [u8],
    origin_x: u32,
    origin_y: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    color: [u8; 4],
) {
    for offset_y in 0..height {
        for offset_x in 0..width {
            pixel(bytes, origin_x, origin_y, x + offset_x, y + offset_y, color);
        }
    }
}

fn pixel(bytes: &mut [u8], origin_x: u32, origin_y: u32, x: u32, y: u32, color: [u8; 4]) {
    set_pixel(bytes, origin_x + x, origin_y + y, color);
}

fn set_pixel(bytes: &mut [u8], x: u32, y: u32, color: [u8; 4]) {
    let index = ((y * ATLAS_WIDTH + x) * 4) as usize;
    bytes[index..index + 4].copy_from_slice(&color);
}
