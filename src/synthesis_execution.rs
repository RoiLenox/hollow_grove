use crate::aim::{gremlin_tinker_aim, pixy_confusion_aim};
use crate::{
    Aim, AimBuildError, ContactOutcome, FrameState, LandingOutcome, Point, ScriptApplicationError,
    SynthesisRecipe, SynthesisRecipeCompileError, SynthesisScript, compile_recipe, fire,
    land_contact,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SynthesisExecution {
    start_frame_state: FrameState,
    recipe: SynthesisRecipe,
    scripts: Vec<SynthesisScript>,
    aim: Aim,
    contact: ContactOutcome,
    landing: LandingOutcome,
}

impl SynthesisExecution {
    #[must_use]
    pub const fn start_frame_state(&self) -> &FrameState {
        &self.start_frame_state
    }

    #[must_use]
    pub const fn recipe(&self) -> &SynthesisRecipe {
        &self.recipe
    }

    #[must_use]
    pub fn scripts(&self) -> &[SynthesisScript] {
        &self.scripts
    }

    #[must_use]
    pub const fn aim(&self) -> &Aim {
        &self.aim
    }

    #[must_use]
    pub const fn contact(&self) -> ContactOutcome {
        self.contact
    }

    #[must_use]
    pub const fn landing(&self) -> &LandingOutcome {
        &self.landing
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SynthesisExecutionError {
    UnknownRecipe,
    Compile(SynthesisRecipeCompileError),
    Aim(AimBuildError),
    Landing(ScriptApplicationError),
}

pub fn execute_synthesis_recipe(
    start: &Point,
    recipe: &SynthesisRecipe,
) -> Result<SynthesisExecution, SynthesisExecutionError> {
    let scripts = compile_recipe(recipe).map_err(SynthesisExecutionError::Compile)?;
    let aim = build_canonical_aim(recipe, scripts.clone())?;
    let contact = fire(&aim);
    let landing = land_contact(start.frame_state(), &aim, contact)
        .map_err(SynthesisExecutionError::Landing)?;

    Ok(SynthesisExecution {
        start_frame_state: start.frame_state().clone(),
        recipe: recipe.clone(),
        scripts,
        aim,
        contact,
        landing,
    })
}

fn build_canonical_aim(
    recipe: &SynthesisRecipe,
    scripts: Vec<SynthesisScript>,
) -> Result<Aim, SynthesisExecutionError> {
    match recipe.recipe_id() {
        "pixy_confusion" => {
            pixy_confusion_aim(recipe, scripts).map_err(SynthesisExecutionError::Aim)
        }
        "gremlin_tinker" => {
            gremlin_tinker_aim(recipe, scripts).map_err(SynthesisExecutionError::Aim)
        }
        _ => Err(SynthesisExecutionError::UnknownRecipe),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ContactOutcome, FlowId, FrameId, GlowId, LandingOutcome, Point, gremlin_tinker_recipe,
        pixy_confusion_recipe,
    };

    use super::{SynthesisExecutionError, execute_synthesis_recipe};

    #[test]
    fn canonical_pixy_recipe_executes_through_the_frozen_v1_path() {
        let point = Point::origin();
        let execution = execute_synthesis_recipe(&point, &pixy_confusion_recipe())
            .expect("execution should work");

        assert_eq!(execution.start_frame_state().frame(), FrameId::Hueman);
        assert_eq!(execution.contact(), ContactOutcome::Kiss);
        let LandingOutcome::Kiss(kiss) = execution.landing() else {
            panic!("expected kiss landing");
        };
        assert_eq!(kiss.point_squared().frame(), FrameId::Pixy);
        assert_eq!(kiss.point_squared().prism().mind(), 3);
        assert_eq!(kiss.point_squared().glow_learnset(), &[GlowId::Confusion]);
        assert!(kiss.point_squared().flow_learnset().is_empty());
    }

    #[test]
    fn canonical_gremlin_recipe_executes_through_the_frozen_v1_path() {
        let point = Point::origin();
        let execution = execute_synthesis_recipe(&point, &gremlin_tinker_recipe())
            .expect("execution should work");

        assert_eq!(execution.start_frame_state().frame(), FrameId::Hueman);
        assert_eq!(execution.contact(), ContactOutcome::Kiss);
        let LandingOutcome::Kiss(kiss) = execution.landing() else {
            panic!("expected kiss landing");
        };
        assert_eq!(kiss.point_squared().frame(), FrameId::Gremlin);
        assert_eq!(kiss.point_squared().prism().body(), 3);
        assert_eq!(kiss.point_squared().flow_learnset(), &[FlowId::TinkerGrip]);
        assert!(kiss.point_squared().glow_learnset().is_empty());
    }

    #[test]
    fn execution_rejects_unknown_recipe_ids() {
        let point = Point::origin();
        let unknown = crate::SynthesisRecipe::new(
            "unknown",
            "Unknown",
            vec![crate::RecipeIntent::ChangeFrame(crate::FrameId::Pixy)],
        );

        assert_eq!(
            execute_synthesis_recipe(&point, &unknown),
            Err(SynthesisExecutionError::UnknownRecipe)
        );
    }
}
