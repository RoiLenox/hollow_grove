use crate::{FlowId, FrameId, GlowId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PrismDelta {
    body: i16,
    spirit: i16,
    mind: i16,
    soul_interior: i16,
    soul_exterior: i16,
}

impl PrismDelta {
    pub const fn new(
        body: i16,
        spirit: i16,
        mind: i16,
        soul_interior: i16,
        soul_exterior: i16,
    ) -> Self {
        Self {
            body,
            spirit,
            mind,
            soul_interior,
            soul_exterior,
        }
    }

    pub const fn zero() -> Self {
        Self::new(0, 0, 0, 0, 0)
    }

    pub const fn body(&self) -> i16 {
        self.body
    }

    pub const fn spirit(&self) -> i16 {
        self.spirit
    }

    pub const fn mind(&self) -> i16 {
        self.mind
    }

    pub const fn soul_interior(&self) -> i16 {
        self.soul_interior
    }

    pub const fn soul_exterior(&self) -> i16 {
        self.soul_exterior
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RecipeIntent {
    ModifyPrism(PrismDelta),
    LearnFlow(FlowId),
    LearnGlow(GlowId),
    ChangeFrame(FrameId),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SynthesisRecipe {
    recipe_id: String,
    display_name: String,
    intents: Vec<RecipeIntent>,
}

impl SynthesisRecipe {
    pub fn new(
        recipe_id: impl Into<String>,
        display_name: impl Into<String>,
        intents: Vec<RecipeIntent>,
    ) -> Self {
        Self {
            recipe_id: recipe_id.into(),
            display_name: display_name.into(),
            intents,
        }
    }

    pub fn recipe_id(&self) -> &str {
        &self.recipe_id
    }

    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    pub fn intents(&self) -> &[RecipeIntent] {
        &self.intents
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SynthesisScript {
    ApplyPrismDelta(PrismDelta),
    AddFlow(FlowId),
    AddGlow(GlowId),
    SetFrame(FrameId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SynthesisRecipeCompileError {
    EmptyRecipeId,
    EmptyDisplayName,
    NoIntents,
}

pub fn compile_recipe(
    recipe: &SynthesisRecipe,
) -> Result<Vec<SynthesisScript>, SynthesisRecipeCompileError> {
    if recipe.recipe_id().trim().is_empty() {
        return Err(SynthesisRecipeCompileError::EmptyRecipeId);
    }

    if recipe.display_name().trim().is_empty() {
        return Err(SynthesisRecipeCompileError::EmptyDisplayName);
    }

    if recipe.intents().is_empty() {
        return Err(SynthesisRecipeCompileError::NoIntents);
    }

    Ok(recipe
        .intents()
        .iter()
        .map(|intent| match intent {
            RecipeIntent::ModifyPrism(delta) => SynthesisScript::ApplyPrismDelta(*delta),
            RecipeIntent::LearnFlow(flow_id) => SynthesisScript::AddFlow(*flow_id),
            RecipeIntent::LearnGlow(glow_id) => SynthesisScript::AddGlow(*glow_id),
            RecipeIntent::ChangeFrame(frame_id) => SynthesisScript::SetFrame(*frame_id),
        })
        .collect())
}

pub fn pixy_confusion_recipe() -> SynthesisRecipe {
    SynthesisRecipe::new(
        "pixy_confusion",
        "Pixy Confusion Recipe",
        vec![
            RecipeIntent::ModifyPrism(PrismDelta::new(0, 0, 2, 0, 0)),
            RecipeIntent::LearnGlow(GlowId::Confusion),
            RecipeIntent::ChangeFrame(FrameId::Pixy),
        ],
    )
}

pub fn gremlin_tinker_recipe() -> SynthesisRecipe {
    SynthesisRecipe::new(
        "gremlin_tinker",
        "Gremlin Tinker Recipe",
        vec![
            RecipeIntent::ModifyPrism(PrismDelta::new(2, 0, 0, 0, 0)),
            RecipeIntent::LearnFlow(FlowId::TinkerGrip),
            RecipeIntent::ChangeFrame(FrameId::Gremlin),
        ],
    )
}

#[cfg(test)]
mod tests {
    use crate::{FrameState, KernelInput, Symptom, run_kernel_cycle, run_kernel_cycle_with_input};

    use super::{
        PrismDelta, RecipeIntent, SynthesisRecipe, SynthesisRecipeCompileError, SynthesisScript,
        compile_recipe, gremlin_tinker_recipe, pixy_confusion_recipe,
    };

    #[test]
    fn pixy_confusion_recipe_compiles_successfully() {
        let scripts = compile_recipe(&pixy_confusion_recipe()).expect("recipe should compile");

        assert_eq!(scripts.len(), 3);
        assert_eq!(
            scripts,
            vec![
                SynthesisScript::ApplyPrismDelta(PrismDelta::new(0, 0, 2, 0, 0)),
                SynthesisScript::AddGlow(crate::GlowId::Confusion),
                SynthesisScript::SetFrame(crate::FrameId::Pixy),
            ]
        );
    }

    #[test]
    fn gremlin_tinker_recipe_compiles_successfully() {
        let scripts = compile_recipe(&gremlin_tinker_recipe()).expect("recipe should compile");

        assert_eq!(scripts.len(), 3);
        assert_eq!(
            scripts,
            vec![
                SynthesisScript::ApplyPrismDelta(PrismDelta::new(2, 0, 0, 0, 0)),
                SynthesisScript::AddFlow(crate::FlowId::TinkerGrip),
                SynthesisScript::SetFrame(crate::FrameId::Gremlin),
            ]
        );
    }

    #[test]
    fn pixy_confusion_recipe_scripts_keep_the_expected_order() {
        let scripts = compile_recipe(&pixy_confusion_recipe()).expect("recipe should compile");

        assert!(matches!(
            scripts[0],
            SynthesisScript::ApplyPrismDelta(PrismDelta {
                body: 0,
                spirit: 0,
                mind: 2,
                soul_interior: 0,
                soul_exterior: 0,
            })
        ));
        assert!(matches!(
            scripts[1],
            SynthesisScript::AddGlow(crate::GlowId::Confusion)
        ));
        assert!(matches!(
            scripts[2],
            SynthesisScript::SetFrame(crate::FrameId::Pixy)
        ));
    }

    #[test]
    fn gremlin_tinker_recipe_scripts_keep_the_expected_order() {
        let scripts = compile_recipe(&gremlin_tinker_recipe()).expect("recipe should compile");

        assert!(matches!(
            scripts[0],
            SynthesisScript::ApplyPrismDelta(PrismDelta {
                body: 2,
                spirit: 0,
                mind: 0,
                soul_interior: 0,
                soul_exterior: 0,
            })
        ));
        assert!(matches!(
            scripts[1],
            SynthesisScript::AddFlow(crate::FlowId::TinkerGrip)
        ));
        assert!(matches!(
            scripts[2],
            SynthesisScript::SetFrame(crate::FrameId::Gremlin)
        ));
    }

    #[test]
    fn prism_delta_keeps_mind_plus_two_and_other_channels_zero() {
        let scripts = compile_recipe(&pixy_confusion_recipe()).expect("recipe should compile");
        let SynthesisScript::ApplyPrismDelta(delta) = scripts[0] else {
            panic!("expected first script to be ApplyPrismDelta");
        };

        assert_eq!(delta.body(), 0);
        assert_eq!(delta.spirit(), 0);
        assert_eq!(delta.mind(), 2);
        assert_eq!(delta.soul_interior(), 0);
        assert_eq!(delta.soul_exterior(), 0);
    }

    #[test]
    fn gremlin_prism_delta_keeps_body_plus_two_and_other_channels_zero() {
        let scripts = compile_recipe(&gremlin_tinker_recipe()).expect("recipe should compile");
        let SynthesisScript::ApplyPrismDelta(delta) = scripts[0] else {
            panic!("expected first script to be ApplyPrismDelta");
        };

        assert_eq!(delta.body(), 2);
        assert_eq!(delta.spirit(), 0);
        assert_eq!(delta.mind(), 0);
        assert_eq!(delta.soul_interior(), 0);
        assert_eq!(delta.soul_exterior(), 0);
    }

    #[test]
    fn invalid_empty_recipe_data_returns_explicit_errors() {
        let empty_id = SynthesisRecipe::new(
            "",
            "Pixy Confusion Recipe",
            vec![RecipeIntent::ModifyPrism(PrismDelta::zero())],
        );
        let empty_name = SynthesisRecipe::new(
            "pixy_confusion",
            "   ",
            vec![RecipeIntent::LearnGlow(crate::GlowId::Confusion)],
        );
        let no_intents = SynthesisRecipe::new("pixy_confusion", "Pixy Confusion Recipe", vec![]);

        assert_eq!(
            compile_recipe(&empty_id),
            Err(SynthesisRecipeCompileError::EmptyRecipeId)
        );
        assert_eq!(
            compile_recipe(&empty_name),
            Err(SynthesisRecipeCompileError::EmptyDisplayName)
        );
        assert_eq!(
            compile_recipe(&no_intents),
            Err(SynthesisRecipeCompileError::NoIntents)
        );
    }

    #[test]
    fn compiling_recipe_does_not_mutate_starting_or_landed_frame_state() {
        let straight = run_kernel_cycle(Symptom::origin());
        let curved = run_kernel_cycle_with_input(
            Symptom::origin(),
            KernelInput {
                routing: crate::PlebMetaInput {
                    exterior_shape: crate::ExteriorShape::Curved,
                    pleb_mode: crate::Mode::Pathos,
                    meta_mode: crate::Mode::Logos,
                },
            },
        );
        let start_before = straight.start_frame_state().clone();
        let end_before = straight.end_frame_state().clone();
        let curved_start_before = curved.start_frame_state().clone();
        let curved_end_before = curved.end_frame_state().clone();

        let scripts = compile_recipe(&pixy_confusion_recipe()).expect("recipe should compile");

        assert_eq!(scripts.len(), 3);
        assert_eq!(straight.start_frame_state(), &start_before);
        assert_eq!(straight.end_frame_state(), &end_before);
        assert_eq!(curved.start_frame_state(), &curved_start_before);
        assert_eq!(curved.end_frame_state(), &curved_end_before);
        assert_eq!(straight.start_frame_state(), straight.end_frame_state());
        assert_eq!(curved.start_frame_state(), curved.end_frame_state());
        assert_eq!(straight.start_frame_state(), &FrameState::origin());
    }
}
