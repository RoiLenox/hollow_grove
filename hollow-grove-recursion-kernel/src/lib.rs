//! Pure, domain-neutral routing grammar for one Hollow Grove recursion pass.
//!
//! This crate owns only bounded cycle topology and Pleb/Meta routing. It has no
//! dependencies and deliberately knows nothing about Houses, characters,
//! evidence, persistence, presentation, or application-owned Point state.

pub mod pleb_meta;

pub use pleb_meta::{
    ExteriorShape, ExteriorState, InteriorState, Mode, Operator, PlebMetaBond, PlebMetaGrammar,
    PlebMetaInput, PlebMetaRouting, RoutingPass, Sequence, StrandState, normal_response,
};

/// Domain-neutral stages in one complete recursion pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CycleStage {
    Point,
    Triway,
    Fourway,
    HollowGrove,
    CurrentSeam,
    AuraBeam,
    LandedPoint,
}

pub const CANONICAL_CYCLE_STAGES: [CycleStage; 7] = [
    CycleStage::Point,
    CycleStage::Triway,
    CycleStage::Fourway,
    CycleStage::HollowGrove,
    CycleStage::CurrentSeam,
    CycleStage::AuraBeam,
    CycleStage::LandedPoint,
];

#[cfg(test)]
mod tests {
    use super::{CANONICAL_CYCLE_STAGES, CycleStage};

    #[test]
    fn canonical_cycle_is_bounded_and_lands_once() {
        assert_eq!(CANONICAL_CYCLE_STAGES.len(), 7);
        assert_eq!(CANONICAL_CYCLE_STAGES[0], CycleStage::Point);
        assert_eq!(CANONICAL_CYCLE_STAGES[6], CycleStage::LandedPoint);
        assert_eq!(
            CANONICAL_CYCLE_STAGES
                .iter()
                .filter(|stage| **stage == CycleStage::LandedPoint)
                .count(),
            1
        );
    }
}
