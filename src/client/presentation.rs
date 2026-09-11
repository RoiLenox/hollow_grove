//! Renderer-facing projections with no wgpu or window types.

use std::fmt;

use crate::gameplay::CardinalDirection;

use super::assets::{AtlasRegion, SpriteKey, resolve_region};
use super::model::{
    ActorBodyIdentity, ActorState, ClientWorldState, LOGICAL_HEIGHT, LOGICAL_WIDTH, MovementState,
    TILE_SIZE, TileKind, WalkFrame,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Camera2d {
    pub origin_x: i32,
    pub origin_y: i32,
    pub viewport_width: u32,
    pub viewport_height: u32,
}

impl Camera2d {
    #[must_use]
    pub fn following(actor: &ActorState, world_width: u32, world_height: u32) -> Self {
        let actor_center_x =
            i32::from(actor.position.x) * TILE_SIZE as i32 + (TILE_SIZE / 2) as i32;
        let actor_center_y =
            i32::from(actor.position.y) * TILE_SIZE as i32 + (TILE_SIZE / 2) as i32;
        let max_x = world_width.saturating_sub(LOGICAL_WIDTH) as i32;
        let max_y = world_height.saturating_sub(LOGICAL_HEIGHT) as i32;
        Self {
            origin_x: (actor_center_x - LOGICAL_WIDTH as i32 / 2).clamp(0, max_x),
            origin_y: (actor_center_y - LOGICAL_HEIGHT as i32 / 2).clamp(0, max_y),
            viewport_width: LOGICAL_WIDTH,
            viewport_height: LOGICAL_HEIGHT,
        }
    }

    #[must_use]
    pub const fn world_to_screen(self, world_x: i32, world_y: i32) -> (i32, i32) {
        (world_x - self.origin_x, world_y - self.origin_y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PhysicalViewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub scale: f32,
    pub integer_scaled: bool,
}

#[must_use]
pub fn logical_viewport(physical_width: u32, physical_height: u32) -> PhysicalViewport {
    let fit_x = physical_width as f32 / LOGICAL_WIDTH as f32;
    let fit_y = physical_height as f32 / LOGICAL_HEIGHT as f32;
    let fit = fit_x.min(fit_y);
    let (scale, integer_scaled) = if fit >= 1.0 {
        (fit.floor(), true)
    } else {
        (fit.max(0.01), false)
    };
    let width = LOGICAL_WIDTH as f32 * scale;
    let height = LOGICAL_HEIGHT as f32 * scale;
    let (x, y) = if integer_scaled {
        (
            physical_width.saturating_sub(width as u32) as f32 / 2.0,
            physical_height.saturating_sub(height as u32) as f32 / 2.0,
        )
    } else {
        (
            (physical_width as f32 - width) * 0.5,
            (physical_height as f32 - height) * 0.5,
        )
    };
    PhysicalViewport {
        x: x.floor(),
        y: y.floor(),
        width,
        height,
        scale,
        integer_scaled,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActorPresentationState {
    pub sprite_key: SpriteKey,
    pub world_x: i32,
    pub world_y: i32,
    pub facing: CardinalDirection,
    pub movement: MovementState,
    pub walk_frame: WalkFrame,
    pub visual_width: u32,
    pub visual_height: u32,
}

impl ActorPresentationState {
    pub fn resolve(actor: &ActorState) -> Result<Self, PresentationError> {
        let sprite_key = match &actor.identity.body {
            ActorBodyIdentity::Hueman => SpriteKey::HuemanOverworldPrototype,
            ActorBodyIdentity::Nature { nature_id } => {
                return Err(PresentationError::UnresolvedNature(nature_id.clone()));
            }
        };
        Ok(Self {
            sprite_key,
            world_x: i32::from(actor.position.x) * TILE_SIZE as i32,
            world_y: i32::from(actor.position.y) * TILE_SIZE as i32,
            facing: actor.position.facing,
            movement: actor.movement,
            walk_frame: actor.walk_frame,
            visual_width: TILE_SIZE,
            visual_height: TILE_SIZE,
        })
    }

    #[must_use]
    pub fn atlas_region(self) -> AtlasRegion {
        resolve_region(self.sprite_key, self.facing, self.movement, self.walk_frame)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpriteDraw {
    pub key: SpriteKey,
    pub screen_x: i32,
    pub screen_y: i32,
    pub width: u32,
    pub height: u32,
    pub source: AtlasRegion,
    pub order: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenePresentation {
    pub camera: Camera2d,
    pub sprites: Vec<SpriteDraw>,
}

impl ScenePresentation {
    pub fn resolve(state: &ClientWorldState) -> Result<Self, PresentationError> {
        let camera = Camera2d::following(
            &state.hueman,
            state.world.width_pixels(),
            state.world.height_pixels(),
        );
        let first_x = (camera.origin_x / TILE_SIZE as i32).max(0);
        let first_y = (camera.origin_y / TILE_SIZE as i32).max(0);
        let last_x = ((camera.origin_x + camera.viewport_width as i32) / TILE_SIZE as i32 + 1)
            .min(i32::from(state.world.width_tiles));
        let last_y = ((camera.origin_y + camera.viewport_height as i32) / TILE_SIZE as i32 + 1)
            .min(i32::from(state.world.height_tiles));

        let mut sprites = Vec::new();
        for y in first_y..last_y {
            for x in first_x..last_x {
                let key = match state.world.tile(x, y) {
                    TileKind::Ground => SpriteKey::TestGround,
                    TileKind::Path => SpriteKey::TestPath,
                    TileKind::Blocked => SpriteKey::TestBlocked,
                };
                let (screen_x, screen_y) =
                    camera.world_to_screen(x * TILE_SIZE as i32, y * TILE_SIZE as i32);
                sprites.push(SpriteDraw {
                    key,
                    screen_x,
                    screen_y,
                    width: TILE_SIZE,
                    height: TILE_SIZE,
                    source: resolve_region(
                        key,
                        CardinalDirection::South,
                        MovementState::Idle,
                        WalkFrame::A,
                    ),
                    order: 0,
                });
            }
        }

        let actor = ActorPresentationState::resolve(&state.hueman)?;
        let (screen_x, screen_y) = camera.world_to_screen(actor.world_x, actor.world_y);
        sprites.push(SpriteDraw {
            key: actor.sprite_key,
            screen_x,
            screen_y,
            width: actor.visual_width,
            height: actor.visual_height,
            source: actor.atlas_region(),
            order: 1,
        });
        sprites.sort_by_key(|sprite| sprite.order);
        Ok(Self { camera, sprites })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PresentationError {
    UnresolvedNature(String),
}

impl fmt::Display for PresentationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnresolvedNature(nature) => {
                write!(
                    formatter,
                    "no overworld sprite key is registered for Nature `{nature}`"
                )
            }
        }
    }
}

impl std::error::Error for PresentationError {}
