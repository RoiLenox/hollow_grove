//! Seasonal authority adapter for Adaptive Topology State V1.
//!
//! A validated Function Junction is the only input. The astronomical calendar
//! and player location cannot request a topology change directly.

use std::fmt;

use crate::constitutional::SeasonalAnchor;

use super::adaptive_topology_state::{
    AdaptiveTopologyStateError, AdaptiveTopologyStateV1, AdaptiveTopologyTarget,
    AdaptiveTopologyTransitionRequest, AdaptiveTopologyTrigger, AdaptiveTopologyTriggerSource,
};
use super::function_junction::{FunctionJunctionId, FunctionJunctionRuntime};

pub const SEASONAL_ADAPTIVE_TRANSITION_TICKS: u32 = 4;
pub const SEASONAL_ADAPTIVE_TRANSITION_PREFIX: &str = "transition.adaptive-topology.";
pub const SEASONAL_ADAPTIVE_TRIGGER_PREFIX: &str = "trigger.adaptive-topology.";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SeasonalAdaptiveTopologyDisposition {
    NotApplicable {
        anchor: SeasonalAnchor,
    },
    AlreadyAtTargetRecorded {
        transition_id: String,
        target: AdaptiveTopologyTarget,
    },
    TransitionStarted {
        transition_id: String,
        target: AdaptiveTopologyTarget,
        duration_ticks: u32,
    },
    TransitionInProgress {
        transition_id: String,
        target: AdaptiveTopologyTarget,
    },
    TransitionAlreadyApplied {
        transition_id: String,
        target: AdaptiveTopologyTarget,
    },
}

pub fn apply_completed_function_junction(
    state: &mut AdaptiveTopologyStateV1,
    runtime: &FunctionJunctionRuntime,
    junction_id: &FunctionJunctionId,
) -> Result<SeasonalAdaptiveTopologyDisposition, AdaptiveTopologySeasonalError> {
    state.validate()?;
    let junction = runtime.junctions().get(junction_id).ok_or_else(|| {
        AdaptiveTopologySeasonalError::UnknownFunctionJunction(junction_id.as_str().to_owned())
    })?;
    let Some(target) = target_for_seasonal_anchor(junction.anchor) else {
        return Ok(SeasonalAdaptiveTopologyDisposition::NotApplicable {
            anchor: junction.anchor,
        });
    };

    let transition_id = format!(
        "{SEASONAL_ADAPTIVE_TRANSITION_PREFIX}{}",
        junction.junction_id.as_str()
    );
    if state.has_completed_transition(&transition_id) {
        return Ok(
            SeasonalAdaptiveTopologyDisposition::TransitionAlreadyApplied {
                transition_id,
                target,
            },
        );
    }
    if let Some(active) = state.active_transition() {
        if active.transition_id == transition_id && active.target == target {
            return Ok(SeasonalAdaptiveTopologyDisposition::TransitionInProgress {
                transition_id,
                target,
            });
        }
        return Err(AdaptiveTopologySeasonalError::TransitionConflict {
            active_transition_id: active.transition_id.clone(),
            requested_transition_id: transition_id,
        });
    }

    let request = AdaptiveTopologyTransitionRequest::linear(
        transition_id.clone(),
        AdaptiveTopologyTrigger::new(
            format!(
                "{SEASONAL_ADAPTIVE_TRIGGER_PREFIX}{}",
                junction.junction_id.as_str()
            ),
            AdaptiveTopologyTriggerSource::WorldAuthority,
        )?,
        target,
        SEASONAL_ADAPTIVE_TRANSITION_TICKS,
    )?;
    if state.rectangularity() == target.rectangularity() {
        state.record_already_satisfied_transition(request)?;
        return Ok(
            SeasonalAdaptiveTopologyDisposition::AlreadyAtTargetRecorded {
                transition_id,
                target,
            },
        );
    }

    state.request_transition(request)?;
    Ok(SeasonalAdaptiveTopologyDisposition::TransitionStarted {
        transition_id,
        target,
        duration_ticks: SEASONAL_ADAPTIVE_TRANSITION_TICKS,
    })
}

#[must_use]
pub const fn target_for_seasonal_anchor(anchor: SeasonalAnchor) -> Option<AdaptiveTopologyTarget> {
    match anchor {
        SeasonalAnchor::WinterSolstice => Some(AdaptiveTopologyTarget::Circular),
        SeasonalAnchor::SpringEquinox => Some(AdaptiveTopologyTarget::Rectangular),
        SeasonalAnchor::SummerSolstice | SeasonalAnchor::AutumnEquinox => None,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdaptiveTopologySeasonalError {
    State(AdaptiveTopologyStateError),
    UnknownFunctionJunction(String),
    TransitionConflict {
        active_transition_id: String,
        requested_transition_id: String,
    },
}

impl From<AdaptiveTopologyStateError> for AdaptiveTopologySeasonalError {
    fn from(value: AdaptiveTopologyStateError) -> Self {
        Self::State(value)
    }
}

impl fmt::Display for AdaptiveTopologySeasonalError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::State(error) => error.fmt(formatter),
            Self::UnknownFunctionJunction(id) => {
                write!(formatter, "unknown completed Function Junction: {id}")
            }
            Self::TransitionConflict {
                active_transition_id,
                requested_transition_id,
            } => write!(
                formatter,
                "seasonal topology request {requested_transition_id} conflicts with active transition {active_transition_id}"
            ),
        }
    }
}

impl std::error::Error for AdaptiveTopologySeasonalError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::function_junction_archive::replay_function_junction_payload;
    use crate::world::function_junction_fixture::canonical_function_junction_archive_fixture;

    fn runtime() -> FunctionJunctionRuntime {
        replay_function_junction_payload(&canonical_function_junction_archive_fixture())
            .unwrap()
            .into_values()
            .next()
            .unwrap()
            .function_junction_runtime
    }

    fn junction_id(
        runtime: &FunctionJunctionRuntime,
        anchor: SeasonalAnchor,
    ) -> FunctionJunctionId {
        runtime
            .junction_at_anchor(anchor)
            .unwrap()
            .junction_id
            .clone()
    }

    fn commit_next(state: &mut AdaptiveTopologyStateV1) {
        let prepared = state.prepare_next_step().unwrap().unwrap();
        let evidence = prepared
            .acknowledge_rebuilds(
                format!("collision.snapshot.seasonal.r{}", prepared.next_revision),
                format!("navigation.snapshot.seasonal.r{}", prepared.next_revision),
            )
            .unwrap();
        state.commit_prepared_step(&prepared, evidence).unwrap();
    }

    #[test]
    fn initiation_incarnates_the_rectangle_at_approved_quarter_poses() {
        let runtime = runtime();
        let spring = junction_id(&runtime, SeasonalAnchor::SpringEquinox);
        let mut state = AdaptiveTopologyStateV1::circular();
        let disposition = apply_completed_function_junction(&mut state, &runtime, &spring).unwrap();
        assert!(matches!(
            disposition,
            SeasonalAdaptiveTopologyDisposition::TransitionStarted {
                target: AdaptiveTopologyTarget::Rectangular,
                duration_ticks: 4,
                ..
            }
        ));
        for expected in [250_000_000, 500_000_000, 750_000_000, 1_000_000_000] {
            commit_next(&mut state);
            assert_eq!(state.rectangularity().parts(), expected);
        }
    }

    #[test]
    fn way_back_returns_the_rectangle_to_the_circle_in_four_ticks() {
        let runtime = runtime();
        let winter = junction_id(&runtime, SeasonalAnchor::WinterSolstice);
        let mut state = AdaptiveTopologyStateV1::rectangular();
        let disposition = apply_completed_function_junction(&mut state, &runtime, &winter).unwrap();
        assert!(matches!(
            disposition,
            SeasonalAdaptiveTopologyDisposition::TransitionStarted {
                target: AdaptiveTopologyTarget::Circular,
                duration_ticks: 4,
                ..
            }
        ));
        for expected in [750_000_000, 500_000_000, 250_000_000, 0] {
            commit_next(&mut state);
            assert_eq!(state.rectangularity().parts(), expected);
        }
    }

    #[test]
    fn gathering_and_festival_preserve_the_current_communal_form() {
        let runtime = runtime();
        let mut state = AdaptiveTopologyStateV1::rectangular();
        let before = state.clone();
        for anchor in [
            SeasonalAnchor::SummerSolstice,
            SeasonalAnchor::AutumnEquinox,
        ] {
            let id = junction_id(&runtime, anchor);
            assert_eq!(
                apply_completed_function_junction(&mut state, &runtime, &id).unwrap(),
                SeasonalAdaptiveTopologyDisposition::NotApplicable { anchor }
            );
            assert_eq!(state, before);
        }
    }

    #[test]
    fn already_satisfied_and_active_requests_are_durable_idempotent_replays() {
        let runtime = runtime();
        let winter = junction_id(&runtime, SeasonalAnchor::WinterSolstice);
        let spring = junction_id(&runtime, SeasonalAnchor::SpringEquinox);
        let mut state = AdaptiveTopologyStateV1::circular();

        assert!(matches!(
            apply_completed_function_junction(&mut state, &runtime, &winter).unwrap(),
            SeasonalAdaptiveTopologyDisposition::AlreadyAtTargetRecorded { .. }
        ));
        let after_winter = state.clone();
        assert!(matches!(
            apply_completed_function_junction(&mut state, &runtime, &winter).unwrap(),
            SeasonalAdaptiveTopologyDisposition::TransitionAlreadyApplied { .. }
        ));
        assert_eq!(state, after_winter);

        apply_completed_function_junction(&mut state, &runtime, &spring).unwrap();
        let active = state.clone();
        assert!(matches!(
            apply_completed_function_junction(&mut state, &runtime, &spring).unwrap(),
            SeasonalAdaptiveTopologyDisposition::TransitionInProgress { .. }
        ));
        assert_eq!(state, active);
        while state.active_transition().is_some() {
            commit_next(&mut state);
        }
        let rectangular = state.clone();
        assert!(matches!(
            apply_completed_function_junction(&mut state, &runtime, &winter).unwrap(),
            SeasonalAdaptiveTopologyDisposition::TransitionAlreadyApplied { .. }
        ));
        assert_eq!(
            state, rectangular,
            "old Winter replay must not reverse Spring"
        );
    }

    #[test]
    fn unknown_junction_and_competing_active_transition_fail_without_mutation() {
        let runtime = runtime();
        let spring = junction_id(&runtime, SeasonalAnchor::SpringEquinox);
        let winter = junction_id(&runtime, SeasonalAnchor::WinterSolstice);
        let unknown = FunctionJunctionId::new("function-junction.unknown").unwrap();
        let mut state = AdaptiveTopologyStateV1::circular();
        let before = state.clone();
        assert!(apply_completed_function_junction(&mut state, &runtime, &unknown).is_err());
        assert_eq!(state, before);

        apply_completed_function_junction(&mut state, &runtime, &spring).unwrap();
        let active = state.clone();
        assert!(apply_completed_function_junction(&mut state, &runtime, &winter).is_err());
        assert_eq!(state, active);
    }
}
