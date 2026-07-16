use crate::{Aim, ManagerGeometry, ManagerRelation, Way, manager_domain_lock};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContactOutcome {
    Miss,
    Kiss,
}

impl ContactOutcome {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Miss => "Miss",
            Self::Kiss => "Kiss",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FireContext {
    required_relation: ManagerRelation,
    required_geometry: ManagerGeometry,
    required_way: Way,
}

impl FireContext {
    pub const fn new(
        required_relation: ManagerRelation,
        required_geometry: ManagerGeometry,
        required_way: Way,
    ) -> Self {
        Self {
            required_relation,
            required_geometry,
            required_way,
        }
    }

    pub fn for_aim(aim: &Aim) -> Self {
        let manager_lock = aim.manager_lock();
        Self::new(
            manager_lock.relation(),
            manager_lock.geometry(),
            aim.bond().linked_way(),
        )
    }

    pub const fn required_relation(self) -> ManagerRelation {
        self.required_relation
    }

    pub const fn required_geometry(self) -> ManagerGeometry {
        self.required_geometry
    }

    pub const fn required_way(self) -> Way {
        self.required_way
    }
}

pub fn fire(aim: &Aim) -> ContactOutcome {
    fire_with_context(aim, &FireContext::for_aim(aim))
}

pub fn fire_with_context(aim: &Aim, context: &FireContext) -> ContactOutcome {
    let manager_lock = aim.manager_lock();

    if aim.scripts().is_empty() {
        return ContactOutcome::Miss;
    }

    if manager_lock != manager_domain_lock(manager_lock.manager()) {
        return ContactOutcome::Miss;
    }

    if manager_lock.relation() != context.required_relation() {
        return ContactOutcome::Miss;
    }

    if manager_lock.geometry() != context.required_geometry() {
        return ContactOutcome::Miss;
    }

    if aim.bond().linked_way() != context.required_way() {
        return ContactOutcome::Miss;
    }

    ContactOutcome::Kiss
}

#[cfg(test)]
mod tests {
    use crate::aim::{canonical_aim_bond, gremlin_tinker_aim, pixy_confusion_aim};
    use crate::{
        ExteriorShape, FrameState, KernelInput, Manager, ManagerGeometry, ManagerRelation, Mode,
        PlebMetaInput, Symptom, compile_recipe, construct_aim, gremlin_tinker_recipe,
        pixy_confusion_recipe, run_kernel_cycle, run_kernel_cycle_with_input,
    };

    use super::{ContactOutcome, FireContext, fire, fire_with_context};

    #[test]
    fn prepared_pixy_aim_can_be_fired_and_returns_kiss() {
        let recipe = pixy_confusion_recipe();
        let scripts = compile_recipe(&recipe).expect("recipe should compile");
        let aim = pixy_confusion_aim(&recipe, scripts.clone()).expect("aim should build");

        assert_eq!(fire(&aim), ContactOutcome::Kiss);
        assert_eq!(aim.scripts(), scripts.as_slice());
    }

    #[test]
    fn prepared_gremlin_aim_can_be_fired_and_returns_kiss() {
        let recipe = gremlin_tinker_recipe();
        let scripts = compile_recipe(&recipe).expect("recipe should compile");
        let aim = gremlin_tinker_aim(&recipe, scripts.clone()).expect("aim should build");

        assert_eq!(fire(&aim), ContactOutcome::Kiss);
        assert_eq!(aim.scripts(), scripts.as_slice());
    }

    #[test]
    fn fire_does_not_mutate_aim_or_frame_state() {
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
        let aim = pixy_confusion_aim(&recipe, scripts.clone()).expect("aim should build");
        let aim_before = aim.clone();

        assert_eq!(fire(&aim), ContactOutcome::Kiss);
        assert_eq!(aim, aim_before);
        assert_eq!(aim.scripts(), scripts.as_slice());
        assert_eq!(straight.start_frame_state(), &start_before);
        assert_eq!(straight.end_frame_state(), &end_before);
        assert_eq!(curved.start_frame_state(), &curved_start_before);
        assert_eq!(curved.end_frame_state(), &curved_end_before);
        assert_eq!(straight.start_frame_state(), straight.end_frame_state());
        assert_eq!(curved.start_frame_state(), curved.end_frame_state());
        assert_eq!(straight.start_frame_state(), &FrameState::origin());
    }

    #[test]
    fn manager_relation_mismatch_produces_miss() {
        let straight = run_kernel_cycle(Symptom::origin());
        let start_before = straight.start_frame_state().clone();
        let end_before = straight.end_frame_state().clone();
        let recipe = pixy_confusion_recipe();
        let scripts = compile_recipe(&recipe).expect("recipe should compile");
        let aim = pixy_confusion_aim(&recipe, scripts.clone()).expect("aim should build");
        let aim_before = aim.clone();
        let miss_context = FireContext::new(
            ManagerRelation::PlebPleb,
            aim.manager_lock().geometry(),
            aim.bond().linked_way(),
        );

        assert_eq!(fire_with_context(&aim, &miss_context), ContactOutcome::Miss);
        assert_eq!(aim, aim_before);
        assert_eq!(aim.scripts(), scripts.as_slice());
        assert_eq!(straight.start_frame_state(), &start_before);
        assert_eq!(straight.end_frame_state(), &end_before);
        assert_eq!(straight.start_frame_state(), straight.end_frame_state());
    }

    #[test]
    fn geometry_mismatch_produces_miss() {
        let recipe = pixy_confusion_recipe();
        let scripts = compile_recipe(&recipe).expect("recipe should compile");
        let aim = pixy_confusion_aim(&recipe, scripts.clone()).expect("aim should build");
        let aim_before = aim.clone();
        let miss_context = FireContext::new(
            aim.manager_lock().relation(),
            ManagerGeometry::Straight,
            aim.bond().linked_way(),
        );

        assert_eq!(fire_with_context(&aim, &miss_context), ContactOutcome::Miss);
        assert_eq!(aim, aim_before);
        assert_eq!(aim.scripts(), scripts.as_slice());
    }

    #[test]
    fn aim_build_error_remains_distinct_from_contact_outcome() {
        let recipe = pixy_confusion_recipe();
        let scripts = compile_recipe(&recipe).expect("recipe should compile");
        let invalid = construct_aim(
            "",
            &recipe,
            scripts.clone(),
            Manager::Hal,
            canonical_aim_bond(),
            None,
        );
        let valid_aim = pixy_confusion_aim(&recipe, scripts).expect("aim should build");

        assert_eq!(invalid, Err(crate::AimBuildError::EmptyAimId));
        assert_eq!(fire(&valid_aim), ContactOutcome::Kiss);
    }

    #[test]
    fn fire_respects_hal_manager_domain_invariants() {
        let recipe = pixy_confusion_recipe();
        let scripts = compile_recipe(&recipe).expect("recipe should compile");
        let aim = pixy_confusion_aim(&recipe, scripts).expect("aim should build");
        let kiss_context = FireContext::new(
            ManagerRelation::PlebMeta,
            ManagerGeometry::Curved,
            aim.bond().linked_way(),
        );

        assert_eq!(aim.manager_lock().manager(), Manager::Hal);
        assert_eq!(fire_with_context(&aim, &kiss_context), ContactOutcome::Kiss);
    }

    #[test]
    fn fire_respects_clouseau_manager_domain_invariants() {
        let recipe = gremlin_tinker_recipe();
        let scripts = compile_recipe(&recipe).expect("recipe should compile");
        let aim = gremlin_tinker_aim(&recipe, scripts).expect("aim should build");
        let kiss_context = FireContext::new(
            ManagerRelation::PlebPleb,
            ManagerGeometry::Straight,
            aim.bond().linked_way(),
        );

        assert_eq!(aim.manager_lock().manager(), Manager::Clouseau);
        assert_eq!(fire_with_context(&aim, &kiss_context), ContactOutcome::Kiss);
    }
}
