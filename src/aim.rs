use crate::{
    Bond, Manager, ManagerDomainLock, SynthesisRecipe, SynthesisScript, Way, manager_domain_lock,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Aim {
    aim_id: String,
    source_recipe_id: String,
    scripts: Vec<SynthesisScript>,
    manager_lock: ManagerDomainLock,
    bond: Bond,
    named_route: Option<String>,
}

impl Aim {
    #[must_use]
    pub fn aim_id(&self) -> &str {
        &self.aim_id
    }

    #[must_use]
    pub fn source_recipe_id(&self) -> &str {
        &self.source_recipe_id
    }

    #[must_use]
    pub fn scripts(&self) -> &[SynthesisScript] {
        &self.scripts
    }

    #[must_use]
    pub const fn manager_lock(&self) -> ManagerDomainLock {
        self.manager_lock
    }

    #[must_use]
    pub const fn bond(&self) -> Bond {
        self.bond
    }

    #[must_use]
    pub fn named_route(&self) -> Option<&str> {
        self.named_route.as_deref()
    }

    #[must_use]
    pub const fn status_label(&self) -> &'static str {
        "prepared"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AimBuildError {
    EmptyAimId,
    EmptyRecipeId,
    NoScripts,
}

#[must_use]
pub fn construct_aim(
    aim_id: impl Into<String>,
    recipe: &SynthesisRecipe,
    scripts: Vec<SynthesisScript>,
    manager: Manager,
    bond: Bond,
    named_route: Option<String>,
) -> Result<Aim, AimBuildError> {
    let aim_id = aim_id.into();
    if aim_id.trim().is_empty() {
        return Err(AimBuildError::EmptyAimId);
    }

    if recipe.recipe_id().trim().is_empty() {
        return Err(AimBuildError::EmptyRecipeId);
    }

    if scripts.is_empty() {
        return Err(AimBuildError::NoScripts);
    }

    Ok(Aim {
        aim_id,
        source_recipe_id: recipe.recipe_id().to_string(),
        scripts,
        manager_lock: manager_domain_lock(manager),
        bond,
        named_route,
    })
}

#[must_use]
pub(crate) fn canonical_aim_bond() -> Bond {
    Bond::select([Way::One, Way::Two, Way::Three])
}

#[must_use]
pub(crate) fn pixy_confusion_aim(
    recipe: &SynthesisRecipe,
    scripts: Vec<SynthesisScript>,
) -> Result<Aim, AimBuildError> {
    construct_aim(
        "pixy_confusion_aim",
        recipe,
        scripts,
        Manager::Hal,
        canonical_aim_bond(),
        None,
    )
}

#[must_use]
pub(crate) fn gremlin_tinker_aim(
    recipe: &SynthesisRecipe,
    scripts: Vec<SynthesisScript>,
) -> Result<Aim, AimBuildError> {
    construct_aim(
        "gremlin_tinker_aim",
        recipe,
        scripts,
        Manager::Clouseau,
        canonical_aim_bond(),
        None,
    )
}

#[cfg(test)]
mod tests {
    use crate::{
        ExteriorShape, FrameState, KernelInput, ManagerDomain, ManagerFunction, ManagerGeometry,
        ManagerRelation, Mode, PlebMetaInput, Symptom, compile_recipe, gremlin_tinker_recipe,
        pixy_confusion_recipe, run_kernel_cycle, run_kernel_cycle_with_input,
    };

    use super::{
        AimBuildError, canonical_aim_bond, construct_aim, gremlin_tinker_aim, pixy_confusion_aim,
    };

    #[test]
    fn pixy_confusion_aim_constructs_successfully() {
        let recipe = pixy_confusion_recipe();
        let scripts = compile_recipe(&recipe).expect("recipe should compile");
        let aim = pixy_confusion_aim(&recipe, scripts.clone()).expect("aim should build");

        assert_eq!(aim.aim_id(), "pixy_confusion_aim");
        assert_eq!(aim.source_recipe_id(), "pixy_confusion");
        assert_eq!(aim.scripts(), scripts.as_slice());
        assert_eq!(aim.scripts().len(), 3);
        assert_eq!(aim.manager_lock().manager(), crate::Manager::Hal);
        assert_eq!(aim.manager_lock().domain(), ManagerDomain::Meta);
        assert_eq!(aim.manager_lock().relation(), ManagerRelation::PlebMeta);
        assert_eq!(aim.manager_lock().geometry(), ManagerGeometry::Curved);
        assert_eq!(aim.manager_lock().function(), ManagerFunction::Connect);
        assert_eq!(aim.bond(), canonical_aim_bond());
        assert_eq!(aim.bond().linked_way(), crate::Way::One);
        assert_eq!(aim.named_route(), None);
        assert_eq!(aim.status_label(), "prepared");
    }

    #[test]
    fn gremlin_tinker_aim_constructs_successfully() {
        let recipe = gremlin_tinker_recipe();
        let scripts = compile_recipe(&recipe).expect("recipe should compile");
        let aim = gremlin_tinker_aim(&recipe, scripts.clone()).expect("aim should build");

        assert_eq!(aim.aim_id(), "gremlin_tinker_aim");
        assert_eq!(aim.source_recipe_id(), "gremlin_tinker");
        assert_eq!(aim.scripts(), scripts.as_slice());
        assert_eq!(aim.scripts().len(), 3);
        assert_eq!(aim.manager_lock().manager(), crate::Manager::Clouseau);
        assert_eq!(aim.manager_lock().domain(), ManagerDomain::Pleb);
        assert_eq!(aim.manager_lock().relation(), ManagerRelation::PlebPleb);
        assert_eq!(aim.manager_lock().geometry(), ManagerGeometry::Straight);
        assert_eq!(aim.manager_lock().function(), ManagerFunction::Locate);
        assert_eq!(aim.bond(), canonical_aim_bond());
        assert_eq!(aim.bond().linked_way(), crate::Way::One);
        assert_eq!(aim.named_route(), None);
        assert_eq!(aim.status_label(), "prepared");
    }

    #[test]
    fn aim_preserves_compiled_script_order() {
        let recipe = pixy_confusion_recipe();
        let scripts = compile_recipe(&recipe).expect("recipe should compile");
        let aim = pixy_confusion_aim(&recipe, scripts).expect("aim should build");

        assert!(matches!(
            aim.scripts()[0],
            crate::SynthesisScript::ApplyPrismDelta(_)
        ));
        assert!(matches!(
            aim.scripts()[1],
            crate::SynthesisScript::AddGlow(crate::GlowId::Confusion)
        ));
        assert!(matches!(
            aim.scripts()[2],
            crate::SynthesisScript::SetFrame(crate::FrameId::Pixy)
        ));
    }

    #[test]
    fn gremlin_aim_preserves_compiled_script_order() {
        let recipe = gremlin_tinker_recipe();
        let scripts = compile_recipe(&recipe).expect("recipe should compile");
        let aim = gremlin_tinker_aim(&recipe, scripts).expect("aim should build");

        assert!(matches!(
            aim.scripts()[0],
            crate::SynthesisScript::ApplyPrismDelta(_)
        ));
        assert!(matches!(
            aim.scripts()[1],
            crate::SynthesisScript::AddFlow(crate::FlowId::TinkerGrip)
        ));
        assert!(matches!(
            aim.scripts()[2],
            crate::SynthesisScript::SetFrame(crate::FrameId::Gremlin)
        ));
    }

    #[test]
    fn aim_build_rejects_empty_ids_and_missing_scripts() {
        let recipe = pixy_confusion_recipe();
        let scripts = compile_recipe(&recipe).expect("recipe should compile");

        assert_eq!(
            construct_aim(
                "",
                &recipe,
                scripts.clone(),
                crate::Manager::Hal,
                canonical_aim_bond(),
                None,
            ),
            Err(AimBuildError::EmptyAimId)
        );

        let empty_recipe =
            crate::SynthesisRecipe::new("   ", "Pixy Confusion Recipe", recipe.intents().to_vec());
        assert_eq!(
            construct_aim(
                "pixy_confusion_aim",
                &empty_recipe,
                scripts.clone(),
                crate::Manager::Hal,
                canonical_aim_bond(),
                None,
            ),
            Err(AimBuildError::EmptyRecipeId)
        );

        assert_eq!(
            construct_aim(
                "pixy_confusion_aim",
                &recipe,
                Vec::new(),
                crate::Manager::Hal,
                canonical_aim_bond(),
                None,
            ),
            Err(AimBuildError::NoScripts)
        );
    }

    #[test]
    fn aim_construction_does_not_mutate_starting_or_landed_frame_state() {
        let straight = run_kernel_cycle(Symptom::origin());
        let curved = run_kernel_cycle_with_input(
            Symptom::origin(),
            KernelInput {
                routing: PlebMetaInput {
                    exterior_shape: ExteriorShape::Curved,
                    pleb_mode: Mode::Pathos,
                    meta_mode: Mode::Logos,
                },
            },
        );
        let start_before = straight.start_frame_state().clone();
        let end_before = straight.end_frame_state().clone();
        let curved_start_before = curved.start_frame_state().clone();
        let curved_end_before = curved.end_frame_state().clone();
        let recipe = pixy_confusion_recipe();
        let scripts = compile_recipe(&recipe).expect("recipe should compile");
        let current_recipe = gremlin_tinker_recipe();
        let current_scripts = compile_recipe(&current_recipe).expect("recipe should compile");

        let aim = pixy_confusion_aim(&recipe, scripts).expect("aim should build");
        let current_aim =
            gremlin_tinker_aim(&current_recipe, current_scripts).expect("aim should build");

        assert_eq!(aim.status_label(), "prepared");
        assert_eq!(current_aim.status_label(), "prepared");
        assert_eq!(straight.start_frame_state(), &start_before);
        assert_eq!(straight.end_frame_state(), &end_before);
        assert_eq!(curved.start_frame_state(), &curved_start_before);
        assert_eq!(curved.end_frame_state(), &curved_end_before);
        assert_eq!(straight.start_frame_state(), straight.end_frame_state());
        assert_eq!(curved.start_frame_state(), curved.end_frame_state());
        assert_eq!(straight.start_frame_state(), &FrameState::origin());
    }
}
