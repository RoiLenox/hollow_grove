use crate::{Aim, ContactOutcome, CurrentPrism, FrameState, Point, PrismDelta, SynthesisScript};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KissLanding {
    before: FrameState,
    point_squared: FrameState,
    applied_scripts: Vec<SynthesisScript>,
}

impl KissLanding {
    #[must_use]
    pub fn before(&self) -> &FrameState {
        &self.before
    }

    #[must_use]
    pub fn point_squared(&self) -> &FrameState {
        &self.point_squared
    }

    #[must_use]
    pub fn applied_scripts(&self) -> &[SynthesisScript] {
        &self.applied_scripts
    }

    #[must_use]
    pub fn next_point(&self) -> Point {
        Point::new(self.point_squared.clone())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LandingOutcome {
    Miss { frame_state: FrameState },
    Kiss(KissLanding),
}

impl LandingOutcome {
    #[must_use]
    pub const fn contact(&self) -> ContactOutcome {
        match self {
            Self::Miss { .. } => ContactOutcome::Miss,
            Self::Kiss(_) => ContactOutcome::Kiss,
        }
    }

    #[must_use]
    pub fn frame_state(&self) -> &FrameState {
        match self {
            Self::Miss { frame_state } => frame_state,
            Self::Kiss(kiss) => kiss.point_squared(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScriptApplicationError {
    PrismUnderflow,
    PrismOverflow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LandingFaultCut {
    BeforeScript(usize),
    DuringScript(usize),
    BetweenScripts(usize),
    AfterFinalScriptStaging,
    BeforeCommit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LandingFaultError {
    Script(ScriptApplicationError),
    Injected(LandingFaultCut),
}

pub fn land_contact(
    start: &FrameState,
    aim: &Aim,
    outcome: ContactOutcome,
) -> Result<LandingOutcome, ScriptApplicationError> {
    match outcome {
        ContactOutcome::Miss => Ok(LandingOutcome::Miss {
            frame_state: start.clone(),
        }),
        ContactOutcome::Kiss => apply_kiss(start, aim),
    }
}

fn apply_kiss(start: &FrameState, aim: &Aim) -> Result<LandingOutcome, ScriptApplicationError> {
    apply_kiss_with_fault(start, aim, None).map_err(|error| match error {
        LandingFaultError::Script(error) => error,
        LandingFaultError::Injected(_) => unreachable!("fault injection is disabled"),
    })
}

pub(crate) fn apply_kiss_with_fault(
    start: &FrameState,
    aim: &Aim,
    fault: Option<LandingFaultCut>,
) -> Result<LandingOutcome, LandingFaultError> {
    let mut working = start.clone();

    for (index, script) in aim.scripts().iter().enumerate() {
        if fault == Some(LandingFaultCut::BeforeScript(index)) {
            return Err(LandingFaultError::Injected(LandingFaultCut::BeforeScript(
                index,
            )));
        }

        apply_script(&mut working, script).map_err(LandingFaultError::Script)?;

        if fault == Some(LandingFaultCut::DuringScript(index)) {
            return Err(LandingFaultError::Injected(LandingFaultCut::DuringScript(
                index,
            )));
        }

        if index + 1 < aim.scripts().len() && fault == Some(LandingFaultCut::BetweenScripts(index))
        {
            return Err(LandingFaultError::Injected(
                LandingFaultCut::BetweenScripts(index),
            ));
        }
    }

    if fault == Some(LandingFaultCut::AfterFinalScriptStaging) {
        return Err(LandingFaultError::Injected(
            LandingFaultCut::AfterFinalScriptStaging,
        ));
    }

    if fault == Some(LandingFaultCut::BeforeCommit) {
        return Err(LandingFaultError::Injected(LandingFaultCut::BeforeCommit));
    }

    Ok(LandingOutcome::Kiss(KissLanding {
        before: start.clone(),
        point_squared: working,
        applied_scripts: aim.scripts().to_vec(),
    }))
}

fn apply_script(
    state: &mut FrameState,
    script: &SynthesisScript,
) -> Result<(), ScriptApplicationError> {
    match script {
        SynthesisScript::ApplyPrismDelta(delta) => apply_prism_delta(state, delta),
        SynthesisScript::AddFlow(flow_id) => {
            state.learn_flow(*flow_id);
            Ok(())
        }
        SynthesisScript::AddGlow(glow_id) => {
            state.learn_glow(*glow_id);
            Ok(())
        }
        SynthesisScript::SetFrame(frame_id) => {
            state.set_frame(*frame_id);
            Ok(())
        }
    }
}

fn apply_prism_delta(
    state: &mut FrameState,
    delta: &PrismDelta,
) -> Result<(), ScriptApplicationError> {
    let prism = state.prism();
    let next_prism = CurrentPrism::new(
        apply_channel_delta(prism.body(), delta.body())?,
        apply_channel_delta(prism.spirit(), delta.spirit())?,
        apply_channel_delta(prism.mind(), delta.mind())?,
        apply_channel_delta(prism.soul_interior(), delta.soul_interior())?,
        apply_channel_delta(prism.soul_exterior(), delta.soul_exterior())?,
    );
    state.set_prism(next_prism);
    Ok(())
}

fn apply_channel_delta(value: u16, delta: i16) -> Result<u16, ScriptApplicationError> {
    let result = i32::from(value) + i32::from(delta);
    if result < 0 {
        return Err(ScriptApplicationError::PrismUnderflow);
    }

    u16::try_from(result).map_err(|_| ScriptApplicationError::PrismOverflow)
}
#[cfg(test)]
mod tests {
    use crate::aim::{canonical_aim_bond, gremlin_tinker_aim, pixy_confusion_aim};
    use crate::{
        BeingId, ContactOutcome, CurrentPrism, FlowId, FrameId, FrameState, GlowId, Manager,
        PrismDelta, SynthesisRecipe, SynthesisScript, compile_recipe, construct_aim, fire,
        gremlin_tinker_recipe, pixy_confusion_recipe,
    };

    use super::{LandingOutcome, ScriptApplicationError, land_contact};

    #[test]
    fn canonical_recipe_to_scripts_to_aim_to_fire_to_kiss_transforms_hueman_into_pixy_confusion() {
        let start = FrameState::origin();
        let recipe = pixy_confusion_recipe();
        let scripts = compile_recipe(&recipe).expect("recipe should compile");
        let aim = pixy_confusion_aim(&recipe, scripts.clone()).expect("aim should build");
        let outcome = land_contact(&start, &aim, fire(&aim)).expect("kiss should land");
        let LandingOutcome::Kiss(kiss) = outcome else {
            panic!("expected kiss landing");
        };

        assert_eq!(kiss.applied_scripts(), scripts.as_slice());
        assert_eq!(kiss.before().being(), BeingId::Hueman);
        assert_eq!(kiss.before().frame(), FrameId::Hueman);
        assert!(kiss.before().glow_learnset().is_empty());
        assert_eq!(kiss.point_squared().being(), BeingId::Hueman);
        assert_eq!(kiss.point_squared().frame(), FrameId::Pixy);
        assert_eq!(kiss.before().prism().mind(), 1);
        assert_eq!(kiss.point_squared().prism().mind(), 3);
        assert_eq!(kiss.point_squared().prism().body(), 1);
        assert_eq!(kiss.point_squared().prism().spirit(), 1);
        assert_eq!(kiss.point_squared().prism().soul_interior(), 1);
        assert_eq!(kiss.point_squared().prism().soul_exterior(), 1);
        assert!(kiss.point_squared().flow_learnset().is_empty());
        assert_eq!(kiss.point_squared().glow_learnset(), &[GlowId::Confusion]);
        assert_eq!(start, FrameState::origin());
        assert_ne!(kiss.point_squared(), &start);
    }

    #[test]
    fn canonical_recipe_to_scripts_to_aim_to_fire_to_kiss_transforms_hueman_into_gremlin_tinker() {
        let start = FrameState::origin();
        let recipe = gremlin_tinker_recipe();
        let scripts = compile_recipe(&recipe).expect("recipe should compile");
        let aim = gremlin_tinker_aim(&recipe, scripts.clone()).expect("aim should build");
        let aim_before = aim.clone();
        let outcome = land_contact(&start, &aim, fire(&aim)).expect("kiss should land");
        let LandingOutcome::Kiss(kiss) = outcome else {
            panic!("expected kiss landing");
        };

        assert_eq!(kiss.applied_scripts(), scripts.as_slice());
        assert_eq!(kiss.before().being(), BeingId::Hueman);
        assert_eq!(kiss.before().frame(), FrameId::Hueman);
        assert!(kiss.before().flow_learnset().is_empty());
        assert_eq!(kiss.point_squared().being(), BeingId::Hueman);
        assert_eq!(kiss.point_squared().frame(), FrameId::Gremlin);
        assert_eq!(kiss.before().prism().body(), 1);
        assert_eq!(kiss.point_squared().prism().body(), 3);
        assert_eq!(kiss.point_squared().prism().spirit(), 1);
        assert_eq!(kiss.point_squared().prism().mind(), 1);
        assert_eq!(kiss.point_squared().prism().soul_interior(), 1);
        assert_eq!(kiss.point_squared().prism().soul_exterior(), 1);
        assert_eq!(kiss.point_squared().flow_learnset(), &[FlowId::TinkerGrip]);
        assert!(kiss.point_squared().glow_learnset().is_empty());
        assert_eq!(aim, aim_before);
        assert_eq!(start, FrameState::origin());
        assert_ne!(kiss.point_squared(), &start);
    }

    #[test]
    fn next_pass_invariant_preserves_frame_prism_flow_and_glow_from_point_squared() {
        let start = FrameState::origin();
        let recipe = pixy_confusion_recipe();
        let scripts = compile_recipe(&recipe).expect("recipe should compile");
        let aim = pixy_confusion_aim(&recipe, scripts).expect("aim should build");
        let outcome = land_contact(&start, &aim, fire(&aim)).expect("kiss should land");
        let LandingOutcome::Kiss(kiss) = outcome else {
            panic!("expected kiss landing");
        };
        let second_pass = crate::run_kernel_cycle(crate::Symptom::new(kiss.next_point()));

        assert_eq!(second_pass.start_frame_state(), kiss.point_squared());
        assert_eq!(second_pass.start_frame_state().frame(), FrameId::Pixy);
        assert_eq!(second_pass.start_frame_state().prism().body(), 1);
        assert_eq!(second_pass.start_frame_state().prism().spirit(), 1);
        assert_eq!(second_pass.start_frame_state().prism().mind(), 3);
        assert_eq!(second_pass.start_frame_state().prism().soul_interior(), 1);
        assert_eq!(second_pass.start_frame_state().prism().soul_exterior(), 1);
        assert!(second_pass.start_frame_state().flow_learnset().is_empty());
        assert_eq!(
            second_pass.start_frame_state().glow_learnset(),
            &[GlowId::Confusion]
        );
    }

    #[test]
    fn gremlin_point_squared_becomes_the_next_point_without_losing_flow_or_prism() {
        let start = FrameState::origin();
        let recipe = gremlin_tinker_recipe();
        let scripts = compile_recipe(&recipe).expect("recipe should compile");
        let aim = gremlin_tinker_aim(&recipe, scripts).expect("aim should build");
        let outcome = land_contact(&start, &aim, fire(&aim)).expect("kiss should land");
        let LandingOutcome::Kiss(kiss) = outcome else {
            panic!("expected kiss landing");
        };
        let second_pass = crate::run_kernel_cycle(crate::Symptom::new(kiss.next_point()));

        assert_eq!(second_pass.start_frame_state(), kiss.point_squared());
        assert_eq!(second_pass.start_frame_state().frame(), FrameId::Gremlin);
        assert_eq!(second_pass.start_frame_state().prism().body(), 3);
        assert_eq!(second_pass.start_frame_state().prism().spirit(), 1);
        assert_eq!(second_pass.start_frame_state().prism().mind(), 1);
        assert_eq!(second_pass.start_frame_state().prism().soul_interior(), 1);
        assert_eq!(second_pass.start_frame_state().prism().soul_exterior(), 1);
        assert_eq!(
            second_pass.start_frame_state().flow_learnset(),
            &[FlowId::TinkerGrip]
        );
        assert!(second_pass.start_frame_state().glow_learnset().is_empty());
    }

    #[test]
    fn switching_frames_preserves_legally_learned_flow_and_glow() {
        let gremlin_recipe = gremlin_tinker_recipe();
        let gremlin_scripts = compile_recipe(&gremlin_recipe).expect("recipe should compile");
        let gremlin_aim =
            gremlin_tinker_aim(&gremlin_recipe, gremlin_scripts).expect("aim should build");
        let first_outcome = land_contact(&FrameState::origin(), &gremlin_aim, fire(&gremlin_aim))
            .expect("first kiss should land");
        let LandingOutcome::Kiss(first_kiss) = first_outcome else {
            panic!("expected Gremlin kiss landing");
        };

        let pixy_recipe = pixy_confusion_recipe();
        let pixy_scripts = compile_recipe(&pixy_recipe).expect("recipe should compile");
        let pixy_aim = pixy_confusion_aim(&pixy_recipe, pixy_scripts).expect("aim should build");
        let second_outcome = land_contact(first_kiss.point_squared(), &pixy_aim, fire(&pixy_aim))
            .expect("second kiss should land");
        let LandingOutcome::Kiss(second_kiss) = second_outcome else {
            panic!("expected Pixy kiss landing");
        };

        assert_eq!(second_kiss.before().being(), BeingId::Hueman);
        assert_eq!(second_kiss.before().frame(), FrameId::Gremlin);
        assert_eq!(second_kiss.before().flow_learnset(), &[FlowId::TinkerGrip]);
        assert!(second_kiss.before().glow_learnset().is_empty());
        assert_eq!(second_kiss.point_squared().being(), BeingId::Hueman);
        assert_eq!(second_kiss.point_squared().frame(), FrameId::Pixy);
        assert_eq!(
            second_kiss.point_squared().flow_learnset(),
            &[FlowId::TinkerGrip]
        );
        assert_eq!(
            second_kiss.point_squared().glow_learnset(),
            &[GlowId::Confusion]
        );
    }

    #[test]
    fn miss_invariant_leaves_starting_frame_state_unchanged_and_produces_no_changed_point_squared()
    {
        let start = FrameState::origin();
        let recipe = pixy_confusion_recipe();
        let scripts = compile_recipe(&recipe).expect("recipe should compile");
        let aim = pixy_confusion_aim(&recipe, scripts.clone()).expect("aim should build");
        let outcome =
            land_contact(&start, &aim, ContactOutcome::Miss).expect("miss should be preserved");

        let LandingOutcome::Miss { frame_state } = outcome else {
            panic!("expected miss landing");
        };

        assert_eq!(frame_state, start);
        assert_eq!(frame_state.frame(), FrameId::Hueman);
        assert_eq!(frame_state.prism().mind(), 1);
        assert!(frame_state.glow_learnset().is_empty());
        assert_eq!(aim.scripts(), scripts.as_slice());
    }

    #[test]
    fn prism_underflow_is_rejected_transactionally() {
        let start = FrameState::origin();
        let recipe = SynthesisRecipe::new("underflow", "Underflow", vec![]);
        let scripts = vec![
            SynthesisScript::ApplyPrismDelta(PrismDelta::new(1, 0, 0, 0, 0)),
            SynthesisScript::ApplyPrismDelta(PrismDelta::new(-3, 0, 0, 0, 0)),
        ];
        let aim = construct_aim(
            "underflow_aim",
            &recipe,
            scripts,
            Manager::Hal,
            canonical_aim_bond(),
            None,
        )
        .expect("aim should build");

        assert_eq!(
            land_contact(&start, &aim, ContactOutcome::Kiss),
            Err(ScriptApplicationError::PrismUnderflow)
        );
        assert_eq!(start, FrameState::origin());
        assert_eq!(aim.scripts().len(), 2);
    }

    #[test]
    fn prism_overflow_is_rejected_transactionally() {
        let start = FrameState::new(
            FrameId::Hueman,
            CurrentPrism::new(u16::MAX, 1, 1, 1, 1),
            Vec::new(),
            Vec::new(),
        );
        let recipe = SynthesisRecipe::new("overflow", "Overflow", vec![]);
        let aim = construct_aim(
            "overflow_aim",
            &recipe,
            vec![SynthesisScript::ApplyPrismDelta(PrismDelta::new(
                1, 0, 0, 0, 0,
            ))],
            Manager::Hal,
            canonical_aim_bond(),
            None,
        )
        .expect("aim should build");

        assert_eq!(
            land_contact(&start, &aim, ContactOutcome::Kiss),
            Err(ScriptApplicationError::PrismOverflow)
        );
        assert_eq!(start.prism().body(), u16::MAX);
    }

    #[test]
    fn duplicate_add_glow_and_add_flow_are_idempotent() {
        let start = FrameState::origin();
        let recipe = SynthesisRecipe::new("duplicates", "Duplicates", vec![]);
        let aim = construct_aim(
            "duplicate_aim",
            &recipe,
            vec![
                SynthesisScript::AddGlow(GlowId::Confusion),
                SynthesisScript::AddGlow(GlowId::Confusion),
                SynthesisScript::AddFlow(FlowId::Stonefold),
                SynthesisScript::AddFlow(FlowId::Stonefold),
            ],
            Manager::Hal,
            canonical_aim_bond(),
            None,
        )
        .expect("aim should build");
        let outcome = land_contact(&start, &aim, ContactOutcome::Kiss).expect("kiss should land");
        let LandingOutcome::Kiss(kiss) = outcome else {
            panic!("expected kiss landing");
        };

        assert_eq!(kiss.point_squared().glow_learnset(), &[GlowId::Confusion]);
        assert_eq!(kiss.point_squared().flow_learnset(), &[FlowId::Stonefold]);
    }

    #[test]
    fn set_frame_preserves_unrelated_state_fields() {
        let start = FrameState::new(
            FrameId::Hueman,
            CurrentPrism::new(2, 3, 4, 5, 6),
            vec![FlowId::Stonefold],
            vec![GlowId::Projection],
        );
        let recipe = SynthesisRecipe::new("set_frame", "Set Frame", vec![]);
        let aim = construct_aim(
            "set_frame_aim",
            &recipe,
            vec![SynthesisScript::SetFrame(FrameId::Pixy)],
            Manager::Hal,
            canonical_aim_bond(),
            None,
        )
        .expect("aim should build");
        let outcome = land_contact(&start, &aim, ContactOutcome::Kiss).expect("kiss should land");
        let LandingOutcome::Kiss(kiss) = outcome else {
            panic!("expected kiss landing");
        };

        assert_eq!(kiss.point_squared().frame(), FrameId::Pixy);
        assert_eq!(kiss.point_squared().prism(), start.prism());
        assert_eq!(kiss.point_squared().flow_learnset(), start.flow_learnset());
        assert_eq!(kiss.point_squared().glow_learnset(), start.glow_learnset());
    }
}
