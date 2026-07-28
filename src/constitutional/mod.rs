//! Constitutional runtime for lawful Current/Aura history.
//!
//! This layer sits above the recursion kernel. It consumes completed kernel,
//! recipe, institution, and observation evidence through typed references; it
//! does not alter kernel mechanics or permit House law to choose kernel output.

mod adapters;
mod application;
mod bond;
mod canonical_calendar;
mod grove_phase;
mod house_synthesis_semantics;
mod houses;
mod ids;
mod interfaces;
mod model;
mod persistence;
mod regional;
mod regional_persistence;
mod runtime;
mod scenarios;
mod trace;
mod tui;
mod visual_identity;

pub use adapters::*;
pub use application::*;
pub use bond::*;
pub use canonical_calendar::*;
pub use grove_phase::*;
pub use house_synthesis_semantics::*;
pub use houses::*;
pub use ids::*;
pub use interfaces::*;
pub use model::*;
pub use persistence::*;
pub use regional::*;
pub use regional_persistence::*;
pub use runtime::*;
pub use scenarios::*;
pub use trace::*;
pub use tui::*;
pub use visual_identity::*;
