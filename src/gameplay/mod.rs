//! Authoritative gameplay orchestration above Hollow Grove's domain reducers.
//!
//! This module is the presentation-neutral seam for the compact party RPG. It
//! owns gameplay continuity and coordinates existing constitutional runtimes;
//! it does not move rule, House, Bond, regional Synthesis, or recursion-kernel
//! authority into a presentation client.

mod application;
mod archive;
mod boardwalk;
mod boardwalk_territory;
mod deep_pressure;
mod identity;
mod living_world;
mod lore_binding;
mod party;
mod protocol;
mod runtime;
mod stonebend;
mod synthesis_pilot;
mod view;
mod world;

pub use crate::world::survey_frame::{
    SurveyFrameAction, SurveyFrameAuthorityReceipt, SurveyFrameClaim, SurveyFrameDelivery,
    SurveyFrameDisposition, SurveyFrameError, SurveyFrameEvent, SurveyFrameOffer, SurveyFramePhase,
    SurveyFrameResidentChoice, SurveyFrameResidentDecision, SurveyFrameRevision, SurveyFrameState,
    SurveyFrameUse,
};
pub use crate::world::sympiote_foundation::{
    SympioteAdaptation, SympioteBoardwalkCamp, SympioteFoundationAction, SympioteFoundationError,
    SympioteFoundationEvent, SympioteFoundationPhase, SympioteFoundationState,
    SympiotePracticeDirection,
};
pub use application::*;
pub use archive::*;
pub use boardwalk::*;
pub use boardwalk_territory::*;
pub use deep_pressure::*;
pub use identity::*;
pub use living_world::*;
pub use lore_binding::*;
pub use party::*;
pub use protocol::*;
pub use runtime::*;
pub use stonebend::*;
pub use synthesis_pilot::*;
pub use view::*;
pub use world::*;
