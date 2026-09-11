//! Phase II native graphical client boundary.
//!
//! Canon and gameplay state remain upstream of renderer-neutral presentation;
//! winit and wgpu are confined to the window and render modules.

mod app;
pub mod assets;
pub mod input;
pub mod model;
pub mod presentation;
mod render;

pub use app::{ClientError, ClientOptions, run_client};
