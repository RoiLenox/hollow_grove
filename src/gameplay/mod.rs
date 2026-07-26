//! Authoritative gameplay orchestration above Hollow Grove's domain reducers.
//!
//! This module is the presentation-neutral seam for the compact party RPG. It
//! owns gameplay continuity and coordinates existing constitutional runtimes;
//! it does not move rule, House, Bond, regional Synthesis, or recursion-kernel
//! authority into a presentation client.

mod application;
mod archive;
mod boardwalk;
mod deep_pressure;
mod identity;
mod living_world;
mod party;
mod protocol;
mod runtime;
mod stonebend;
mod view;
mod world;

pub use application::*;
pub use archive::*;
pub use boardwalk::*;
pub use deep_pressure::*;
pub use identity::*;
pub use living_world::*;
pub use party::*;
pub use protocol::*;
pub use runtime::*;
pub use stonebend::*;
pub use view::*;
pub use world::*;
