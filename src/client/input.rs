//! Winit keyboard events translated into renderer-neutral gameplay intents.

use crate::gameplay::CardinalDirection;
use winit::keyboard::KeyCode;

#[must_use]
pub const fn direction_for_key(code: KeyCode) -> Option<CardinalDirection> {
    match code {
        KeyCode::ArrowUp | KeyCode::KeyW => Some(CardinalDirection::North),
        KeyCode::ArrowRight | KeyCode::KeyD => Some(CardinalDirection::East),
        KeyCode::ArrowDown | KeyCode::KeyS => Some(CardinalDirection::South),
        KeyCode::ArrowLeft | KeyCode::KeyA => Some(CardinalDirection::West),
        _ => None,
    }
}
