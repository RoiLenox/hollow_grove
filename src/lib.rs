pub mod aim;
pub mod artifact_io;
pub mod being_object_ontology;
pub mod civic_body;
pub mod current_grip_inheritance;
pub mod current_synthesis_engine;
pub mod current_synthesis_scenario;
pub mod decision_engine;
pub mod fire;
pub mod flow_glow_grammar;
pub mod frame_state;
pub mod grove_seam;
pub mod hollow_beam;
pub mod hollow_grove;
pub mod hollow_grove_content;
pub mod hollow_grove_contract;
pub mod hueman_progression;
pub mod hueman_slice;
pub mod hueman_support;
pub mod kernel_pass;
pub mod kernel_pass_output;
pub mod landing;
pub mod manager_domain;
pub mod pleb_meta;
pub mod point;
pub mod point_progression;
pub mod snapshot_boundary;
pub mod symptom;
pub mod synthesis_execution;
pub mod synthesis_recipe;
pub mod triway;
pub mod verification;
pub mod world_map_geometry;

pub use aim::{Aim, AimBuildError, construct_aim};
pub use artifact_io::{
    ArtifactFlushRecord, ArtifactSession, read_text_artifact, write_text_artifact,
};
pub use being_object_ontology::{
    ActionAim, AddressingMode, BeingEmbodiment, BeingInheritance, BeingObjectAction,
    BeingObjectContractInput, BeingObjectMoveResolution, BeingObjectObservation,
    BeingObjectSynthesisSpec, BeingState, HollowingOperation, HollowingRefinement, HollowingTarget,
    ObjectCondition, ObjectConnection, ObjectFamily, ObjectFunction, ObjectId, ObjectKind,
    ObjectMaterial, ObjectState, PracticeContext, PressureExposure, ResolvedMoveId, SkillId,
    SkillPractice, TraitTransferRule, build_being_object_validation_report,
    build_being_object_witness, build_being_state, build_move_witness,
    canonical_being_object_contract_fixture, canonical_cross_boundary_synthesis_fixture,
    canonical_foxy_repair_fixture, canonical_gremlin_proxy_action_fixture,
    canonical_hollow_being_fixture, canonical_hollow_object_fixture, canonical_moxy_repair_fixture,
    canonical_object_state, canonical_troglodyte_proxy_action_fixture, observe_being_object_action,
    resolve_being_object_action, validate_being_object_contract,
};
pub use civic_body::{
    CivicAction, CivicBodyContractInput, CivicBodyDefinition, CivicBodyDiagnostic,
    CivicBodyDiagnosticCode, CivicBodyRole, CivicFailure, CivicPeople, CrisisRole,
    build_civic_body_validation_report, build_civic_body_witness, build_civic_crisis_witness,
    canonical_civic_body_contract_fixture, canonical_civic_body_definitions, canonical_civic_chant,
    canonical_civic_crisis_steps, civic_body_definition, validate_civic_body_contract,
};
pub use current_grip_inheritance::{
    CurrentGripActionRequest, CurrentGripInheritanceContractInput, CurrentGripResolution,
    CurrentRequirement, GripExpressionId, GripExpressionScore, GripPracticeEvent,
    GripPracticeProfile, PressureRequirement, SkillRoot,
    build_current_inheritance_validation_report, build_current_inheritance_witness,
    build_grip_witness, canonical_current_grip_inheritance_contract_fixture,
    canonical_freemason_tendency_fixture, canonical_goblin_weapongrip_fixture,
    canonical_gremlin_foxy_tinkergrip_fixture, canonical_gremlin_tinkergrip_fixture,
    canonical_hypergiant_tendency_fixture, canonical_ogre_siegegrip_fixture,
    canonical_ork_formationgrip_fixture, canonical_proletariat_tendency_fixture,
    canonical_troglodyte_precision_fixture, canonical_troglodyte_worldgrip_fixture,
    canonical_troll_bridgegrip_fixture, canonical_troll_moxy_bridgegrip_fixture,
    project_grip_practice_profile, resolve_current_grip_action,
    validate_current_grip_inheritance_contract,
};
pub use decision_engine::{
    ChosenDecision, DecisionCandidate, DecisionCandidateId, DecisionChoiceTrace,
    DecisionChooseError, DecisionEngineError, DecisionEvaluation, DecisionEvaluationReason,
    DecisionEvaluationTrace, DecisionExecution, DecisionExecutionTrace,
    DecisionGeneratedCandidateTrace, DecisionIntent, DecisionObservation, DecisionObservationCheck,
    DecisionObservationTrace, DecisionRecipeBridgeTrace, DecisionTieBreak, DecisionTrace,
    DecisionTraceReasonCode, DecisionTraceReplayError, DecisionTraceTieBreakReason,
    SynthesisOrientation, choose_decision, choose_decision_for_observation,
    evaluate_decision_candidate, execute_decision, execute_kernel_pass_decision,
    generate_decision_candidates, observe_decision, observe_kernel_pass_decision,
    replay_decision_trace, replay_kernel_pass_decision_trace, resolve_candidate_recipe,
};
pub use fire::{ContactOutcome, FireContext, fire, fire_with_context};
pub use flow_glow_grammar::{
    ActionMode, CompatibilityLevel, EmbodiedActionRequest, EmbodiedGesture, EmbodiedMoveId,
    EmbodiedMoveResolution, EmbodiedPracticeRecord, EmbodiedRole, ExpressionDomain,
    FlowGlowContractInput, RecipeBoundaryStatus, build_embodied_action_witness,
    build_flow_glow_validation_report, build_flow_glow_witness,
    canonical_cross_pair_show_seam_fixture, canonical_flow_glow_contract_fixture,
    canonical_freemason_flow_beam_fixture, canonical_hypergiant_flow_gleam_fixture,
    canonical_muse_show_beam_fixture, canonical_nightingale_grip_seam_fixture,
    canonical_nightingale_show_beam_fixture, canonical_proletariat_flow_seam_fixture,
    canonical_siren_grit_gleam_fixture, resolve_embodied_action, validate_flow_glow_contract,
};
pub use frame_state::{BeingId, CurrentPrism, FlowId, FrameId, FrameState, GlowId};
pub use grove_seam::{GroveSeam, SeamRoute};
pub use hollow_beam::{BeamRoute, HollowBeam, LandedSymptom, LandingRoute};
pub use hollow_grove::{Bond, HollowGrove};
pub use hollow_grove_content::{
    build_hollow_grove_foundation_verification_report, build_hollow_grove_vertical_witness,
};
pub use hollow_grove_contract::{
    AuraIlluminationClaim, CurrentDepthIdentityClaim, CurrentDepthOwnershipClaim,
    CurrentRelationClaim, CurrentSpeedSemanticClaim, DepthIdentityValue,
    HollowGroveProgressionContractInput, PointSquaredSemanticClaim,
    build_hollow_grove_progression_witness, canonical_progression_contract_fixture,
    validate_hollow_grove_progression_contract,
};
pub use kernel_pass::{
    AURA_BEAM_WITNESS_LABEL, CANONICAL_WITNESS, CURRENT_SEAM_WITNESS_LABEL, FOURWAY_WITNESS_LABEL,
    KernelInput, KernelPass, LANDED_WITNESS_DESCRIPTION, LANDED_WITNESS_LABEL, START_WITNESS_LABEL,
};
pub use kernel_pass_output::{
    BOUNDARY_REMINDER, DESKTOP_STATUS_ARTIFACT_PATH, INVERSE_PATH_QUESTION, PROMPT_ARTIFACT_PATH,
    SNAPSHOT_ARTIFACT_PATH, build_desktop_status_output, build_inverse_path_prompt,
    build_prompt_artifact_output, build_snapshot_output, build_tree_output,
};
pub use landing::{KissLanding, LandingOutcome, ScriptApplicationError, land_contact};
pub use manager_domain::{
    Manager, ManagerDomain, ManagerDomainLock, ManagerFunction, ManagerGeometry, ManagerRelation,
    build_manager_language_validation_report, build_manager_language_witness,
    canonical_manager_language_contract_fixture, compact_manager_domain_law, manager_domain_lock,
    routing_respects_manager_domain_lock, validate_manager_language_contract,
};
pub use pleb_meta::{
    ExteriorShape, ExteriorState, InteriorState, Mode, Operator, PlebMetaBond, PlebMetaGrammar,
    PlebMetaInput, PlebMetaRouting, RoutingPass, Sequence, StrandState, normal_response,
};
pub use point::Point;
pub use point_progression::{
    AuraDevelopment, AuraDimensionId, CanonicalHorizonId, CanonicalRouteId,
    CurrentDepthDevelopment, CurrentDepthId, HuemanCapacities, PointProgressionDiagnostic,
    PointProgressionDiagnosticCode, PointProgressionState, PointSquaredApplication,
    PointSquaredApplicationStatus, PointSquaredAscension, PointSquaredPrepareError,
    PointWorldConsequence, ReachableWorldState, apply_point_squared_ascension,
    build_canonical_point_squared_fixture, build_point_progression_state_output,
    build_point_squared_witness, build_progression_validation_report, build_progression_witness,
    parse_point_progression_state, prepare_point_squared_ascension, validate_point_progression,
};
pub use snapshot_boundary::{SnapshotBoundary, build_snapshot_boundary_output};
pub use symptom::Symptom;
pub use synthesis_execution::{
    SynthesisExecution, SynthesisExecutionError, execute_synthesis_recipe,
};
pub use synthesis_recipe::{
    PrismDelta, RecipeIntent, SynthesisRecipe, SynthesisRecipeCompileError, SynthesisScript,
    compile_recipe, gremlin_tinker_recipe, pixy_confusion_recipe,
};
pub use triway::{Triway, Way};
pub use world_map_geometry::{
    Foxy, FoxySource, FoxySourceKind, HollowGroveRotationContractInput, HouseNumber,
    HousePositionKind, Moxy, MoxyRelation, PlayerSpatialContractInput, PlayerSpatialFixture,
    PlayerSpatialInterpretation, PointGeometryState, PointLocation, PositionThreshold, Proximity,
    Proxy, ReflectionKind, RelativeDirection, RotationObservationContext, RotationPass,
    RotationPosition, RotationalCoordinate, RuleOfTwelveContractInput, RuleOfTwelvePosition,
    SpatialGeometry, SpiralTransition, ThresholdKind, WorldCenterId, WorldRing,
    build_canonical_player_spatial_fixture, build_map_artifact, build_map_validation_report,
    build_map_witness, build_player_location_witness, build_rule_of_twelve_validation_report,
    build_rule_of_twelve_witness, canonical_player_spatial_contract_fixture,
    canonical_rotation_contract_fixture, canonical_rule_of_twelve_contract_fixture,
    derive_player_spatial_interpretation, glaushouse_anchor_position,
    glaushouse_threshold_position, house_anchor_for_position, house_for_position,
    house_number_for_position, is_primary_house_anchor, is_rotation_complete, next_position,
    observation_context_for_point, opposite_position, pass_for_position, position_identity,
    previous_position, select_canonical_spiral_transition, stonebend_anchor_position,
    threshold_at_position, validate_hollow_grove_rotation_contract,
    validate_player_spatial_contract, validate_rule_of_twelve_contract,
};

pub fn run_kernel_cycle(symptom: Symptom) -> KernelPass {
    run_kernel_cycle_with_input(symptom, KernelInput::default())
}

pub fn run_kernel_cycle_with_input(symptom: Symptom, input: KernelInput) -> KernelPass {
    let start = symptom;
    let triway = start.clone().become_triway();
    let hollow_grove = triway.clone().become_hollow_grove();
    let grove_seam = hollow_grove.clone().become_grove_seam(input.routing);
    let hollow_beam = grove_seam.clone().achieve_hollow_beam();
    let landed = hollow_beam.clone().land_symptom();

    KernelPass::new(start, triway, hollow_grove, grove_seam, hollow_beam, landed)
}

pub fn kernel_proof() -> [&'static str; 10] {
    let _kernel_pass = run_kernel_cycle(Symptom::origin());

    [
        "Point enters the kernel cycle.",
        "Point becomes Triway.",
        "Triway carries one Point through three ways.",
        "Triway is publicly witnessed through Fourway.",
        "Fourway resolves into HollowGrove.",
        "HollowGrove becomes CurrentSeam.",
        "CurrentSeam achieves AuraBeam.",
        "AuraBeam lands Point².",
        "Point² is the Landed Point and next Point source.",
        "Kernel recursion verified.",
    ]
}

#[cfg(test)]
mod tests {
    use super::{
        BeamRoute, Bond, CANONICAL_WITNESS, CurrentPrism, ExteriorShape, FrameId, KernelInput,
        LandingRoute, Mode, SeamRoute, Sequence, Symptom, Way, kernel_proof, run_kernel_cycle,
        run_kernel_cycle_with_input,
    };

    #[test]
    fn symptom_lands_with_the_same_inner_point() {
        let kernel_pass = run_kernel_cycle(Symptom::origin());
        assert_eq!(
            format!("{:?}", kernel_pass.landed_symptom()),
            "Symptom { point: Point }"
        );
        assert_eq!(format!("{:?}", kernel_pass.landed().point()), "Point");
        assert_eq!(format!("{:?}", kernel_pass.end_point()), "Point");
    }

    #[test]
    fn triway_carries_one_symptom_through_three_ways() {
        let triway = Symptom::origin().become_triway();
        assert_eq!(triway.ways(), [Way::One, Way::Two, Way::Three]);
    }

    #[test]
    fn hollow_grove_resolves_link_and_atmosphere() {
        let hollow_grove = Symptom::origin().become_triway().become_hollow_grove();
        assert_eq!(hollow_grove.link(), Way::One);
        assert_eq!(hollow_grove.atmosphere(), [Way::Two, Way::Three]);
    }

    #[test]
    fn bond_selects_one_way() {
        let bond = Bond::select([Way::One, Way::Two, Way::Three]);
        assert_eq!(bond.linked_way(), Way::One);
    }

    #[test]
    fn kernel_pass_witnesses_one_completed_recursion() {
        let kernel_pass = run_kernel_cycle(Symptom::origin());

        assert_eq!(
            format!("{:?}", kernel_pass.start_symptom()),
            "Symptom { point: Point }"
        );
        assert_eq!(format!("{:?}", kernel_pass.start_point()), "Point");
        assert_eq!(
            kernel_pass.triway().ways(),
            [Way::One, Way::Two, Way::Three]
        );
        assert_eq!(kernel_pass.hollow_grove().link(), Way::One);
        assert_eq!(
            kernel_pass.hollow_grove().atmosphere(),
            [Way::Two, Way::Three]
        );
        assert_eq!(
            format!("{:?}", kernel_pass.grove_seam()),
            "GroveSeam { symptom: Symptom { point: Point } }"
        );
        assert_eq!(
            format!("{:?}", kernel_pass.hollow_beam()),
            "HollowBeam { symptom: Symptom { point: Point } }"
        );
        assert_eq!(
            format!("{:?}", kernel_pass.landed_symptom()),
            "Symptom { point: Point }"
        );
        assert_eq!(format!("{:?}", kernel_pass.end_point()), "Point");
        assert_eq!(
            kernel_pass
                .routing()
                .pleb_meta()
                .exterior()
                .foreground_sequence(),
            Sequence::Pleb
        );
        assert_eq!(
            kernel_pass.grove_seam().routing().pleb_meta().exterior(),
            kernel_pass.routing().pleb_meta().exterior()
        );
        assert_eq!(kernel_pass.grove_seam().route(), SeamRoute::PlebExterior);
        assert_eq!(
            kernel_pass.hollow_beam().routing().pleb_meta().exterior(),
            kernel_pass.routing().pleb_meta().exterior()
        );
        assert_eq!(
            kernel_pass.hollow_beam().seam_route(),
            SeamRoute::PlebExterior
        );
        assert_eq!(kernel_pass.hollow_beam().route(), BeamRoute::BlepReturn);
        assert_eq!(kernel_pass.landed().route(), LandingRoute::BlepArrival);
    }

    #[test]
    fn kernel_pass_contains_exactly_one_routing_pass() {
        let kernel_pass = run_kernel_cycle_with_input(
            Symptom::origin(),
            KernelInput {
                routing: super::PlebMetaInput {
                    exterior_shape: ExteriorShape::Curved,
                    pleb_mode: Mode::Pathos,
                    meta_mode: Mode::Logos,
                },
            },
        );

        let routing = kernel_pass.routing();
        assert_eq!(routing.pleb_meta().pleb().sequence(), Sequence::Pleb);
        assert_eq!(routing.pleb_meta().blep().sequence(), Sequence::Blep);
        assert_eq!(routing.pleb_meta().meta().sequence(), Sequence::Meta);
        assert_eq!(routing.pleb_meta().atem().sequence(), Sequence::Atem);
        assert_eq!(
            routing.pleb_meta().exterior().foreground_sequence(),
            Sequence::Meta
        );
        assert_eq!(routing.pleb_meta().interior().sequence(), Sequence::Atem);
        assert_eq!(kernel_pass.grove_seam().route(), SeamRoute::MetaExterior);
        assert_eq!(
            kernel_pass.hollow_beam().seam_route(),
            SeamRoute::MetaExterior
        );
        assert_eq!(kernel_pass.hollow_beam().route(), BeamRoute::AtemReturn);
        assert_eq!(kernel_pass.landed().route(), LandingRoute::AtemArrival);
    }

    #[test]
    fn landed_witness_preserves_universal_symptom_for_both_route_paths() {
        let straight = run_kernel_cycle(Symptom::origin());
        let curved = run_kernel_cycle_with_input(
            Symptom::origin(),
            KernelInput {
                routing: super::PlebMetaInput {
                    exterior_shape: ExteriorShape::Curved,
                    pleb_mode: Mode::Pathos,
                    meta_mode: Mode::Logos,
                },
            },
        );

        assert_eq!(
            format!("{:?}", straight.landed().symptom()),
            "Symptom { point: Point }"
        );
        assert_eq!(
            format!("{:?}", curved.landed().symptom()),
            "Symptom { point: Point }"
        );
        assert_eq!(format!("{:?}", straight.landed().point()), "Point");
        assert_eq!(format!("{:?}", curved.landed().point()), "Point");
    }

    #[test]
    fn kernel_pass_displays_the_canonical_witness_deterministically() {
        let kernel_pass = run_kernel_cycle(Symptom::origin());

        assert_eq!(format!("{kernel_pass}"), CANONICAL_WITNESS);
        assert_eq!(format!("{kernel_pass}"), format!("{kernel_pass}"));
        assert!(format!("{kernel_pass}").contains("Fourway"));
        assert!(format!("{kernel_pass}").contains("CurrentSeam"));
        assert!(format!("{kernel_pass}").contains("AuraBeam"));
        assert!(format!("{kernel_pass}").contains("PlebExterior"));
        assert!(format!("{kernel_pass}").contains("BlepReturn"));
        assert!(format!("{kernel_pass}").contains("BlepArrival"));
        assert!(format!("{kernel_pass}").contains("Point² (Landed Point)"));
        assert!(!format!("{kernel_pass}").contains("MetaExterior"));
        assert!(!format!("{kernel_pass}").contains("AtemReturn"));
        assert!(!format!("{kernel_pass}").contains("AtemArrival"));
    }

    #[test]
    fn origin_symptom_begins_with_the_first_frame_fixture() {
        let frame_state = Symptom::origin().frame_state().clone();

        assert_eq!(frame_state.frame(), FrameId::Hueman);
        assert_eq!(frame_state.prism(), &CurrentPrism::origin());
        assert!(frame_state.flow_learnset().is_empty());
        assert!(frame_state.glow_learnset().is_empty());
    }

    #[test]
    fn frame_state_survives_both_kernel_route_variants_unchanged() {
        let straight = run_kernel_cycle(Symptom::origin());
        let curved = run_kernel_cycle_with_input(
            Symptom::origin(),
            KernelInput {
                routing: super::PlebMetaInput {
                    exterior_shape: ExteriorShape::Curved,
                    pleb_mode: Mode::Pathos,
                    meta_mode: Mode::Logos,
                },
            },
        );
        let expected = super::FrameState::origin();

        assert_eq!(straight.start_frame_state(), &expected);
        assert_eq!(straight.end_frame_state(), &expected);
        assert_eq!(curved.start_frame_state(), &expected);
        assert_eq!(curved.end_frame_state(), &expected);
    }

    #[test]
    fn curved_kernel_pass_displays_the_route_shaped_canonical_witness() {
        let kernel_pass = run_kernel_cycle_with_input(
            Symptom::origin(),
            KernelInput {
                routing: super::PlebMetaInput {
                    exterior_shape: ExteriorShape::Curved,
                    pleb_mode: Mode::Pathos,
                    meta_mode: Mode::Logos,
                },
            },
        );

        let witness = format!("{kernel_pass}");
        assert_eq!(witness, format!("{kernel_pass}"));
        assert!(witness.contains("Fourway"));
        assert!(witness.contains("CurrentSeam"));
        assert!(witness.contains("AuraBeam"));
        assert!(witness.contains("MetaExterior"));
        assert!(witness.contains("AtemReturn"));
        assert!(witness.contains("AtemArrival"));
        assert!(witness.contains("Point² (Landed Point)"));
        assert!(!witness.contains("PlebExterior"));
        assert!(!witness.contains("BlepReturn"));
        assert!(!witness.contains("BlepArrival"));
    }

    #[test]
    fn public_witness_retains_the_complete_canonical_route() {
        let witness = run_kernel_cycle(Symptom::origin()).to_string();

        assert!(
            witness.starts_with("Point\n↓\nTriway\n↓\nFourway\n↓\nHollowGrove\n↓\nCurrentSeam")
        );
        assert!(witness.contains("AuraBeam"));
        assert!(witness.ends_with("Point² (Landed Point) [BlepArrival]"));
        assert!(!witness.contains("Landed Point\n↓\nPoint²"));
    }

    #[test]
    fn point_squared_becomes_the_next_point_for_the_next_pass() {
        let first_pass = run_kernel_cycle(Symptom::origin());
        let next_point = first_pass.landed().next_point();
        let second_pass = run_kernel_cycle(Symptom::new(next_point));

        assert_eq!(
            second_pass.start_frame_state(),
            first_pass.end_frame_state()
        );
        assert_eq!(format!("{:?}", second_pass.start_point()), "Point");
    }

    #[test]
    fn kernel_proof_reports_the_full_cycle() {
        assert_eq!(
            kernel_proof(),
            [
                "Point enters the kernel cycle.",
                "Point becomes Triway.",
                "Triway carries one Point through three ways.",
                "Triway is publicly witnessed through Fourway.",
                "Fourway resolves into HollowGrove.",
                "HollowGrove becomes CurrentSeam.",
                "CurrentSeam achieves AuraBeam.",
                "AuraBeam lands Point².",
                "Point² is the Landed Point and next Point source.",
                "Kernel recursion verified.",
            ]
        );
    }
}
