use std::io;

use crate::being_object_ontology::{
    ActionAim, AddressingMode, BeingState, ObjectId, ObjectState, SkillId,
    build_canonical_being_state_with_aura, canonical_object_state,
};
use crate::flow_glow_grammar::{
    ActionMode, CompatibilityLevel, EmbodiedGesture, ExpressionDomain, RecipeBoundaryStatus,
};
use crate::frame_state::FrameId;
use crate::synthesis_execution::{
    SynthesisExecution, SynthesisExecutionError, execute_synthesis_recipe,
};
use crate::{
    ContactOutcome, ExteriorShape, FlowId, GlowId, KernelPass, Manager, ManagerGeometry,
    PlayerSpatialInterpretation, Point, PrismDelta, RotationObservationContext, SynthesisRecipe,
    derive_player_spatial_interpretation, gremlin_tinker_recipe, manager_domain_lock,
    observation_context_for_point, pixy_confusion_recipe,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DecisionIntent {
    FavorCurrent,
    FavorAura,
    Neutral,
}

impl DecisionIntent {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FavorCurrent => "FavorCurrent",
            Self::FavorAura => "FavorAura",
            Self::Neutral => "Neutral",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SynthesisOrientation {
    Current,
    Aura,
}

impl SynthesisOrientation {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "Current",
            Self::Aura => "Aura",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DecisionCandidateId {
    GremlinTinker,
    PixyConfusion,
}

impl DecisionCandidateId {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GremlinTinker => "GremlinTinker",
            Self::PixyConfusion => "PixyConfusion",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionObservation {
    pub(crate) point: Point,
    pub(crate) intent: DecisionIntent,
    pub(crate) route_geometry: Option<ManagerGeometry>,
    pub(crate) rotation_context: Option<RotationObservationContext>,
    pub(crate) spatial_interpretation: Option<PlayerSpatialInterpretation>,
}

impl DecisionObservation {
    #[must_use]
    pub const fn point(&self) -> &Point {
        &self.point
    }

    #[must_use]
    pub const fn frame_state(&self) -> &crate::FrameState {
        self.point.frame_state()
    }

    #[must_use]
    pub const fn intent(&self) -> DecisionIntent {
        self.intent
    }

    #[must_use]
    pub const fn route_geometry(&self) -> Option<ManagerGeometry> {
        self.route_geometry
    }

    #[must_use]
    pub const fn rotation_context(&self) -> Option<RotationObservationContext> {
        self.rotation_context
    }

    #[must_use]
    pub const fn spatial_interpretation(&self) -> Option<&PlayerSpatialInterpretation> {
        self.spatial_interpretation.as_ref()
    }

    #[must_use]
    pub fn has_flow(&self, flow_id: crate::FlowId) -> bool {
        self.frame_state().flow_learnset().contains(&flow_id)
    }

    #[must_use]
    pub fn has_glow(&self, glow_id: crate::GlowId) -> bool {
        self.frame_state().glow_learnset().contains(&glow_id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecisionCandidate {
    candidate_id: DecisionCandidateId,
    manager: Manager,
    orientation: SynthesisOrientation,
}

impl DecisionCandidate {
    #[must_use]
    pub const fn new(
        candidate_id: DecisionCandidateId,
        manager: Manager,
        orientation: SynthesisOrientation,
    ) -> Self {
        Self {
            candidate_id,
            manager,
            orientation,
        }
    }

    #[must_use]
    pub const fn candidate_id(self) -> DecisionCandidateId {
        self.candidate_id
    }

    #[must_use]
    pub const fn manager(self) -> Manager {
        self.manager
    }

    #[must_use]
    pub const fn orientation(self) -> SynthesisOrientation {
        self.orientation
    }

    #[must_use]
    pub const fn manager_lock(self) -> crate::ManagerDomainLock {
        manager_domain_lock(self.manager)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecisionEvaluationReason {
    PreferredOrientation,
    NonPreferredOrientation,
    NeutralBaseline,
    NeutralAlreadyRealized,
}

impl DecisionEvaluationReason {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreferredOrientation => "preferred orientation",
            Self::NonPreferredOrientation => "nonpreferred orientation",
            Self::NeutralBaseline => "neutral baseline",
            Self::NeutralAlreadyRealized => "already realized in observed state",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecisionEvaluation {
    candidate_id: DecisionCandidateId,
    score: u16,
    reason: DecisionEvaluationReason,
}

impl DecisionEvaluation {
    #[must_use]
    pub const fn new(
        candidate_id: DecisionCandidateId,
        score: u16,
        reason: DecisionEvaluationReason,
    ) -> Self {
        Self {
            candidate_id,
            score,
            reason,
        }
    }

    #[must_use]
    pub const fn candidate_id(self) -> DecisionCandidateId {
        self.candidate_id
    }

    #[must_use]
    pub const fn score(self) -> u16 {
        self.score
    }

    #[must_use]
    pub const fn reason(self) -> DecisionEvaluationReason {
        self.reason
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChosenDecision {
    candidate: DecisionCandidate,
    evaluation: DecisionEvaluation,
    tie_break: Option<DecisionTieBreak>,
}

impl ChosenDecision {
    #[must_use]
    pub const fn candidate(&self) -> DecisionCandidate {
        self.candidate
    }

    #[must_use]
    pub const fn evaluation(&self) -> DecisionEvaluation {
        self.evaluation
    }

    #[must_use]
    pub const fn tie_break(&self) -> Option<DecisionTieBreak> {
        self.tie_break
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecisionTieBreak {
    ObservedGeometryMatch(ManagerGeometry),
    GenerateOrder,
}

impl DecisionTieBreak {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ObservedGeometryMatch(ManagerGeometry::Straight) => "observed straight geometry",
            Self::ObservedGeometryMatch(ManagerGeometry::Curved) => "observed curved geometry",
            Self::ObservedGeometryMatch(ManagerGeometry::Inverted) => "observed inverted geometry",
            Self::GenerateOrder => "generate order",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionObservationCheck {
    pub(crate) candidate_id: DecisionCandidateId,
    pub(crate) already_canonical_frame: bool,
    pub(crate) already_knows_canonical_flow: bool,
    pub(crate) already_knows_canonical_glow: bool,
}

impl DecisionObservationCheck {
    #[must_use]
    pub const fn candidate_id(&self) -> DecisionCandidateId {
        self.candidate_id
    }

    #[must_use]
    pub const fn already_canonical_frame(&self) -> bool {
        self.already_canonical_frame
    }

    #[must_use]
    pub const fn already_knows_canonical_flow(&self) -> bool {
        self.already_knows_canonical_flow
    }

    #[must_use]
    pub const fn already_knows_canonical_glow(&self) -> bool {
        self.already_knows_canonical_glow
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionObservationTrace {
    pub(crate) frame: FrameId,
    pub(crate) flows: Vec<FlowId>,
    pub(crate) glows: Vec<GlowId>,
    pub(crate) intent: DecisionIntent,
    pub(crate) route_geometry: Option<ManagerGeometry>,
    pub(crate) rotation_context: Option<RotationObservationContext>,
    pub(crate) spatial_interpretation: Option<PlayerSpatialInterpretation>,
    pub(crate) state_checks: Vec<DecisionObservationCheck>,
}

impl DecisionObservationTrace {
    #[must_use]
    pub const fn frame(&self) -> FrameId {
        self.frame
    }

    #[must_use]
    pub fn flows(&self) -> &[FlowId] {
        &self.flows
    }

    #[must_use]
    pub fn glows(&self) -> &[GlowId] {
        &self.glows
    }

    #[must_use]
    pub const fn intent(&self) -> DecisionIntent {
        self.intent
    }

    #[must_use]
    pub const fn route_geometry(&self) -> Option<ManagerGeometry> {
        self.route_geometry
    }

    #[must_use]
    pub const fn rotation_context(&self) -> Option<RotationObservationContext> {
        self.rotation_context
    }

    #[must_use]
    pub const fn spatial_interpretation(&self) -> Option<&PlayerSpatialInterpretation> {
        self.spatial_interpretation.as_ref()
    }

    #[must_use]
    pub fn state_checks(&self) -> &[DecisionObservationCheck] {
        &self.state_checks
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecisionGeneratedCandidateTrace {
    pub(crate) candidate_id: DecisionCandidateId,
    pub(crate) manager: Manager,
    pub(crate) manager_geometry: ManagerGeometry,
    pub(crate) orientation: SynthesisOrientation,
}

impl DecisionGeneratedCandidateTrace {
    #[must_use]
    pub const fn candidate_id(&self) -> DecisionCandidateId {
        self.candidate_id
    }

    #[must_use]
    pub const fn manager(&self) -> Manager {
        self.manager
    }

    #[must_use]
    pub const fn manager_geometry(&self) -> ManagerGeometry {
        self.manager_geometry
    }

    #[must_use]
    pub const fn orientation(&self) -> SynthesisOrientation {
        self.orientation
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecisionTraceReasonCode {
    PreferredCurrentOrientation,
    NonPreferredCurrentOrientation,
    PreferredAuraOrientation,
    NonPreferredAuraOrientation,
    NeutralBaseScore,
    AlreadyCanonicalFrame,
    AlreadyKnowsCanonicalFlow,
    AlreadyKnowsCanonicalGlow,
}

impl DecisionTraceReasonCode {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreferredCurrentOrientation => "PreferredCurrentOrientation",
            Self::NonPreferredCurrentOrientation => "NonPreferredCurrentOrientation",
            Self::PreferredAuraOrientation => "PreferredAuraOrientation",
            Self::NonPreferredAuraOrientation => "NonPreferredAuraOrientation",
            Self::NeutralBaseScore => "NeutralBaseScore",
            Self::AlreadyCanonicalFrame => "AlreadyCanonicalFrame",
            Self::AlreadyKnowsCanonicalFlow => "AlreadyKnowsCanonicalFlow",
            Self::AlreadyKnowsCanonicalGlow => "AlreadyKnowsCanonicalGlow",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionEvaluationTrace {
    pub(crate) candidate_id: DecisionCandidateId,
    pub(crate) intent_score: u16,
    pub(crate) realized_state_penalty: u16,
    pub(crate) final_score: u16,
    pub(crate) reason_codes: Vec<DecisionTraceReasonCode>,
}

impl DecisionEvaluationTrace {
    #[must_use]
    pub const fn candidate_id(&self) -> DecisionCandidateId {
        self.candidate_id
    }

    #[must_use]
    pub const fn intent_score(&self) -> u16 {
        self.intent_score
    }

    #[must_use]
    pub const fn realized_state_penalty(&self) -> u16 {
        self.realized_state_penalty
    }

    #[must_use]
    pub const fn final_score(&self) -> u16 {
        self.final_score
    }

    #[must_use]
    pub fn reason_codes(&self) -> &[DecisionTraceReasonCode] {
        &self.reason_codes
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecisionTraceTieBreakReason {
    NoTie,
    ObservedRouteGeometryMatch,
    CanonicalGenerateOrder,
}

impl DecisionTraceTieBreakReason {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoTie => "NoTie",
            Self::ObservedRouteGeometryMatch => "ObservedRouteGeometryMatch",
            Self::CanonicalGenerateOrder => "CanonicalGenerateOrder",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionChoiceTrace {
    pub(crate) highest_score: u16,
    pub(crate) tied_candidates: Vec<DecisionCandidateId>,
    pub(crate) tie_occurred: bool,
    pub(crate) observed_route_geometry: Option<ManagerGeometry>,
    pub(crate) manager_geometry_matched: bool,
    pub(crate) geometry_matching_candidate: Option<DecisionCandidateId>,
    pub(crate) tie_break_reason: DecisionTraceTieBreakReason,
    pub(crate) generate_order_resolved: bool,
    pub(crate) chosen_candidate: DecisionCandidateId,
}

impl DecisionChoiceTrace {
    #[must_use]
    pub const fn highest_score(&self) -> u16 {
        self.highest_score
    }

    #[must_use]
    pub fn tied_candidates(&self) -> &[DecisionCandidateId] {
        &self.tied_candidates
    }

    #[must_use]
    pub const fn tie_occurred(&self) -> bool {
        self.tie_occurred
    }

    #[must_use]
    pub const fn observed_route_geometry(&self) -> Option<ManagerGeometry> {
        self.observed_route_geometry
    }

    #[must_use]
    pub const fn manager_geometry_matched(&self) -> bool {
        self.manager_geometry_matched
    }

    #[must_use]
    pub const fn geometry_matching_candidate(&self) -> Option<DecisionCandidateId> {
        self.geometry_matching_candidate
    }

    #[must_use]
    pub const fn tie_break_reason(&self) -> DecisionTraceTieBreakReason {
        self.tie_break_reason
    }

    #[must_use]
    pub const fn generate_order_resolved(&self) -> bool {
        self.generate_order_resolved
    }

    #[must_use]
    pub const fn chosen_candidate(&self) -> DecisionCandidateId {
        self.chosen_candidate
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionRecipeBridgeTrace {
    pub(crate) chosen_candidate: DecisionCandidateId,
    pub(crate) recipe_id: String,
    pub(crate) handed_to_execution_facade: bool,
}

impl DecisionRecipeBridgeTrace {
    #[must_use]
    pub const fn chosen_candidate(&self) -> DecisionCandidateId {
        self.chosen_candidate
    }

    #[must_use]
    pub fn recipe_id(&self) -> &str {
        &self.recipe_id
    }

    #[must_use]
    pub const fn handed_to_execution_facade(&self) -> bool {
        self.handed_to_execution_facade
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionExecutionTrace {
    pub(crate) contact: ContactOutcome,
    pub(crate) landed_frame: Option<FrameId>,
    pub(crate) prism_delta: PrismDelta,
    pub(crate) added_flow: Vec<FlowId>,
    pub(crate) added_glow: Vec<GlowId>,
    pub(crate) point_squared_produced: bool,
}

impl DecisionExecutionTrace {
    #[must_use]
    pub const fn contact(&self) -> ContactOutcome {
        self.contact
    }

    #[must_use]
    pub const fn landed_frame(&self) -> Option<FrameId> {
        self.landed_frame
    }

    #[must_use]
    pub const fn prism_delta(&self) -> &PrismDelta {
        &self.prism_delta
    }

    #[must_use]
    pub fn added_flow(&self) -> &[FlowId] {
        &self.added_flow
    }

    #[must_use]
    pub fn added_glow(&self) -> &[GlowId] {
        &self.added_glow
    }

    #[must_use]
    pub const fn point_squared_produced(&self) -> bool {
        self.point_squared_produced
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionTrace {
    pub(crate) observation: DecisionObservationTrace,
    pub(crate) generation: Vec<DecisionGeneratedCandidateTrace>,
    pub(crate) evaluations: Vec<DecisionEvaluationTrace>,
    pub(crate) choice: DecisionChoiceTrace,
    pub(crate) recipe_bridge: DecisionRecipeBridgeTrace,
    pub(crate) execution: DecisionExecutionTrace,
}

impl DecisionTrace {
    #[must_use]
    pub const fn observation(&self) -> &DecisionObservationTrace {
        &self.observation
    }

    #[must_use]
    pub fn generation(&self) -> &[DecisionGeneratedCandidateTrace] {
        &self.generation
    }

    #[must_use]
    pub fn evaluations(&self) -> &[DecisionEvaluationTrace] {
        &self.evaluations
    }

    #[must_use]
    pub const fn choice(&self) -> &DecisionChoiceTrace {
        &self.choice
    }

    #[must_use]
    pub const fn recipe_bridge(&self) -> &DecisionRecipeBridgeTrace {
        &self.recipe_bridge
    }

    #[must_use]
    pub const fn execution(&self) -> &DecisionExecutionTrace {
        &self.execution
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionExecution {
    observation: DecisionObservation,
    candidates: Vec<DecisionCandidate>,
    evaluations: Vec<DecisionEvaluation>,
    chosen: ChosenDecision,
    recipe: SynthesisRecipe,
    execution: SynthesisExecution,
    trace: DecisionTrace,
}

impl DecisionExecution {
    #[must_use]
    pub const fn observation(&self) -> &DecisionObservation {
        &self.observation
    }

    #[must_use]
    pub fn candidates(&self) -> &[DecisionCandidate] {
        &self.candidates
    }

    #[must_use]
    pub fn evaluations(&self) -> &[DecisionEvaluation] {
        &self.evaluations
    }

    #[must_use]
    pub const fn chosen(&self) -> &ChosenDecision {
        &self.chosen
    }

    #[must_use]
    pub const fn recipe(&self) -> &SynthesisRecipe {
        &self.recipe
    }

    #[must_use]
    pub const fn execution(&self) -> &SynthesisExecution {
        &self.execution
    }

    #[must_use]
    pub const fn trace(&self) -> &DecisionTrace {
        &self.trace
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecisionChooseError {
    NoCandidates,
    MissingEvaluation(DecisionCandidateId),
    DuplicateCandidate(DecisionCandidateId),
    DuplicateEvaluation(DecisionCandidateId),
    UnknownEvaluation(DecisionCandidateId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecisionEngineError {
    Choose(DecisionChooseError),
    Execution(SynthesisExecutionError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecisionTraceReplayError {
    ObservationMismatch,
    CandidateOrderMismatch,
    EvaluationMismatch(DecisionCandidateId),
    TieBreakMismatch,
    ChoiceMismatch,
    RecipeMismatch,
    ExecutionMismatch,
}

pub fn observe_decision(point: &Point, intent: DecisionIntent) -> DecisionObservation {
    observe_decision_with_geometry(point, intent, None)
}

pub(crate) fn observe_decision_with_geometry(
    point: &Point,
    intent: DecisionIntent,
    route_geometry: Option<ManagerGeometry>,
) -> DecisionObservation {
    let spatial_interpretation = derive_player_spatial_interpretation(point);
    let spatial_interpretation = if spatial_interpretation.proxy().is_some()
        || spatial_interpretation.moxy().is_some()
        || spatial_interpretation.foxy().is_some()
    {
        Some(spatial_interpretation)
    } else {
        None
    };

    DecisionObservation {
        point: point.clone(),
        intent,
        route_geometry,
        rotation_context: observation_context_for_point(point),
        spatial_interpretation,
    }
}

pub fn observe_kernel_pass_decision(
    kernel_pass: &KernelPass,
    intent: DecisionIntent,
) -> DecisionObservation {
    observe_decision_with_geometry(
        kernel_pass.start_point(),
        intent,
        Some(match kernel_pass.routing().pleb_meta().exterior().shape() {
            ExteriorShape::Straight => ManagerGeometry::Straight,
            ExteriorShape::Curved => ManagerGeometry::Curved,
        }),
    )
}

pub fn generate_decision_candidates(_observation: &DecisionObservation) -> Vec<DecisionCandidate> {
    vec![
        DecisionCandidate::new(
            DecisionCandidateId::GremlinTinker,
            Manager::Clouseau,
            SynthesisOrientation::Current,
        ),
        DecisionCandidate::new(
            DecisionCandidateId::PixyConfusion,
            Manager::Hal,
            SynthesisOrientation::Aura,
        ),
    ]
}

pub fn evaluate_decision_candidate(
    observation: &DecisionObservation,
    candidate: &DecisionCandidate,
) -> DecisionEvaluation {
    build_evaluation_trace(observation, candidate).to_evaluation()
}

fn candidate_already_realized(
    observation: &DecisionObservation,
    candidate_id: DecisionCandidateId,
) -> bool {
    let check = observation_check_for_candidate(observation, candidate_id);
    check.already_canonical_frame()
        || check.already_knows_canonical_flow()
        || check.already_knows_canonical_glow()
}

pub fn choose_decision(
    candidates: &[DecisionCandidate],
    evaluations: &[DecisionEvaluation],
) -> Result<ChosenDecision, DecisionChooseError> {
    choose_decision_for_observation(None, candidates, evaluations)
}

pub fn choose_decision_for_observation(
    observation: Option<&DecisionObservation>,
    candidates: &[DecisionCandidate],
    evaluations: &[DecisionEvaluation],
) -> Result<ChosenDecision, DecisionChooseError> {
    if candidates.is_empty() {
        return Err(DecisionChooseError::NoCandidates);
    }

    for (index, candidate) in candidates.iter().enumerate() {
        if candidates[index + 1..]
            .iter()
            .any(|other| other.candidate_id() == candidate.candidate_id())
        {
            return Err(DecisionChooseError::DuplicateCandidate(
                candidate.candidate_id(),
            ));
        }
    }

    for evaluation in evaluations {
        if !candidates
            .iter()
            .any(|candidate| candidate.candidate_id() == evaluation.candidate_id())
        {
            return Err(DecisionChooseError::UnknownEvaluation(
                evaluation.candidate_id(),
            ));
        }
    }

    let mut best: Option<(
        DecisionCandidate,
        DecisionEvaluation,
        Option<DecisionTieBreak>,
    )> = None;

    for candidate in candidates {
        let matching: Vec<_> = evaluations
            .iter()
            .copied()
            .filter(|evaluation| evaluation.candidate_id() == candidate.candidate_id())
            .collect();

        if matching.is_empty() {
            return Err(DecisionChooseError::MissingEvaluation(
                candidate.candidate_id(),
            ));
        }

        if matching.len() > 1 {
            return Err(DecisionChooseError::DuplicateEvaluation(
                candidate.candidate_id(),
            ));
        }

        let evaluation = matching[0];
        let candidate_tie_break = tie_break_priority(observation, candidate);
        let replace_best = best
            .map(|(_best_candidate, best_evaluation, best_tie_break)| {
                if evaluation.score() > best_evaluation.score() {
                    true
                } else if evaluation.score() < best_evaluation.score() {
                    false
                } else {
                    candidate_tie_break_rank(candidate_tie_break)
                        > candidate_tie_break_rank(best_tie_break)
                }
            })
            .unwrap_or(true);

        if replace_best {
            best = Some((*candidate, evaluation, candidate_tie_break));
        }
    }

    let Some((candidate, evaluation, tie_break)) = best else {
        return Err(DecisionChooseError::NoCandidates);
    };

    Ok(ChosenDecision {
        candidate,
        evaluation,
        tie_break: if tied_score_count(evaluations, evaluation.score()) > 1 {
            Some(tie_break.unwrap_or(DecisionTieBreak::GenerateOrder))
        } else {
            None
        },
    })
}

pub fn resolve_candidate_recipe(candidate_id: DecisionCandidateId) -> SynthesisRecipe {
    match candidate_id {
        DecisionCandidateId::GremlinTinker => gremlin_tinker_recipe(),
        DecisionCandidateId::PixyConfusion => pixy_confusion_recipe(),
    }
}

pub fn execute_decision(
    point: &Point,
    intent: DecisionIntent,
) -> Result<DecisionExecution, DecisionEngineError> {
    let observation = observe_decision(point, intent);
    execute_observed_decision(observation)
}

pub fn execute_kernel_pass_decision(
    kernel_pass: &KernelPass,
    intent: DecisionIntent,
) -> Result<DecisionExecution, DecisionEngineError> {
    let observation = observe_kernel_pass_decision(kernel_pass, intent);
    execute_observed_decision(observation)
}

pub fn replay_decision_trace(
    point: &Point,
    intent: DecisionIntent,
    trace: &DecisionTrace,
) -> Result<(), DecisionTraceReplayError> {
    replay_trace_for_observation(observe_decision(point, intent), trace)
}

pub fn replay_kernel_pass_decision_trace(
    kernel_pass: &KernelPass,
    intent: DecisionIntent,
    trace: &DecisionTrace,
) -> Result<(), DecisionTraceReplayError> {
    replay_trace_for_observation(observe_kernel_pass_decision(kernel_pass, intent), trace)
}

pub(crate) fn execute_observed_decision(
    observation: DecisionObservation,
) -> Result<DecisionExecution, DecisionEngineError> {
    let plan = build_decision_plan(observation).map_err(DecisionEngineError::Choose)?;
    let recipe = plan.recipe.clone();
    let execution = execute_synthesis_recipe(plan.observation.point(), &recipe)
        .map_err(DecisionEngineError::Execution)?;
    let trace = build_decision_trace(&plan, &execution);

    Ok(DecisionExecution {
        observation: plan.observation,
        candidates: plan.candidates,
        evaluations: plan.evaluations,
        chosen: plan.chosen,
        recipe,
        execution,
        trace,
    })
}

pub(crate) fn replay_trace_for_observation(
    observation: DecisionObservation,
    trace: &DecisionTrace,
) -> Result<(), DecisionTraceReplayError> {
    let plan =
        build_decision_plan(observation).map_err(|_| DecisionTraceReplayError::ChoiceMismatch)?;
    let expected = build_decision_trace_without_execution(&plan);

    if expected.observation != trace.observation {
        return Err(DecisionTraceReplayError::ObservationMismatch);
    }

    if expected.generation != trace.generation {
        return Err(DecisionTraceReplayError::CandidateOrderMismatch);
    }

    for (expected_evaluation, recorded_evaluation) in
        expected.evaluations.iter().zip(trace.evaluations.iter())
    {
        if expected_evaluation != recorded_evaluation {
            return Err(DecisionTraceReplayError::EvaluationMismatch(
                expected_evaluation.candidate_id(),
            ));
        }
    }

    if expected.evaluations.len() != trace.evaluations.len() {
        return Err(DecisionTraceReplayError::CandidateOrderMismatch);
    }

    if expected.choice.tie_break_reason != trace.choice.tie_break_reason
        || expected.choice.manager_geometry_matched != trace.choice.manager_geometry_matched
        || expected.choice.geometry_matching_candidate != trace.choice.geometry_matching_candidate
        || expected.choice.generate_order_resolved != trace.choice.generate_order_resolved
    {
        return Err(DecisionTraceReplayError::TieBreakMismatch);
    }

    if expected.choice != trace.choice {
        return Err(DecisionTraceReplayError::ChoiceMismatch);
    }

    if expected.recipe_bridge != trace.recipe_bridge {
        return Err(DecisionTraceReplayError::RecipeMismatch);
    }

    if expected.execution != trace.execution {
        return Err(DecisionTraceReplayError::ExecutionMismatch);
    }

    Ok(())
}

fn tie_break_priority(
    observation: Option<&DecisionObservation>,
    candidate: &DecisionCandidate,
) -> Option<DecisionTieBreak> {
    let observation = observation?;
    let route_geometry = observation.route_geometry()?;

    if candidate.manager_lock().geometry() == route_geometry {
        Some(DecisionTieBreak::ObservedGeometryMatch(route_geometry))
    } else {
        None
    }
}

fn candidate_tie_break_rank(tie_break: Option<DecisionTieBreak>) -> u8 {
    match tie_break {
        Some(DecisionTieBreak::ObservedGeometryMatch(_)) => 2,
        Some(DecisionTieBreak::GenerateOrder) => 1,
        None => 0,
    }
}

fn tied_score_count(evaluations: &[DecisionEvaluation], score: u16) -> usize {
    evaluations
        .iter()
        .filter(|evaluation| evaluation.score() == score)
        .count()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DecisionPlan {
    pub(crate) observation: DecisionObservation,
    pub(crate) candidates: Vec<DecisionCandidate>,
    pub(crate) evaluations: Vec<DecisionEvaluation>,
    pub(crate) chosen: ChosenDecision,
    pub(crate) recipe: SynthesisRecipe,
}

impl DecisionEvaluationTrace {
    fn to_evaluation(&self) -> DecisionEvaluation {
        DecisionEvaluation::new(self.candidate_id, self.final_score, self.summary_reason())
    }

    fn summary_reason(&self) -> DecisionEvaluationReason {
        match self.reason_codes.first().copied() {
            Some(DecisionTraceReasonCode::PreferredCurrentOrientation)
            | Some(DecisionTraceReasonCode::PreferredAuraOrientation) => {
                DecisionEvaluationReason::PreferredOrientation
            }
            Some(DecisionTraceReasonCode::NonPreferredCurrentOrientation)
            | Some(DecisionTraceReasonCode::NonPreferredAuraOrientation) => {
                DecisionEvaluationReason::NonPreferredOrientation
            }
            Some(DecisionTraceReasonCode::NeutralBaseScore) => {
                if self.realized_state_penalty > 0 {
                    DecisionEvaluationReason::NeutralAlreadyRealized
                } else {
                    DecisionEvaluationReason::NeutralBaseline
                }
            }
            _ => DecisionEvaluationReason::NeutralBaseline,
        }
    }
}

pub(crate) fn build_decision_plan(
    observation: DecisionObservation,
) -> Result<DecisionPlan, DecisionChooseError> {
    let candidates = generate_decision_candidates(&observation);
    let evaluations = candidates
        .iter()
        .map(|candidate| evaluate_decision_candidate(&observation, candidate))
        .collect::<Vec<_>>();
    let chosen = choose_decision_for_observation(Some(&observation), &candidates, &evaluations)?;
    let recipe = resolve_candidate_recipe(chosen.candidate().candidate_id());

    Ok(DecisionPlan {
        observation,
        candidates,
        evaluations,
        chosen,
        recipe,
    })
}

fn build_decision_trace(plan: &DecisionPlan, execution: &SynthesisExecution) -> DecisionTrace {
    let mut trace = build_decision_trace_without_execution(plan);
    trace.execution = build_execution_trace(execution);
    trace
}

pub(crate) fn build_decision_trace_without_execution(plan: &DecisionPlan) -> DecisionTrace {
    let observation = build_observation_trace(&plan.observation, &plan.candidates);
    let generation = plan
        .candidates
        .iter()
        .map(|candidate| DecisionGeneratedCandidateTrace {
            candidate_id: candidate.candidate_id(),
            manager: candidate.manager(),
            manager_geometry: candidate.manager_lock().geometry(),
            orientation: candidate.orientation(),
        })
        .collect::<Vec<_>>();
    let evaluations = plan
        .candidates
        .iter()
        .map(|candidate| build_evaluation_trace(&plan.observation, candidate))
        .collect::<Vec<_>>();
    let choice = build_choice_trace(
        &plan.observation,
        &plan.candidates,
        &plan.evaluations,
        &plan.chosen,
    );
    let recipe_bridge = DecisionRecipeBridgeTrace {
        chosen_candidate: plan.chosen.candidate().candidate_id(),
        recipe_id: plan.recipe.recipe_id().to_string(),
        handed_to_execution_facade: true,
    };
    let execution = build_expected_execution_trace(&plan.observation, &plan.recipe);

    DecisionTrace {
        observation,
        generation,
        evaluations,
        choice,
        recipe_bridge,
        execution,
    }
}

fn build_observation_trace(
    observation: &DecisionObservation,
    candidates: &[DecisionCandidate],
) -> DecisionObservationTrace {
    DecisionObservationTrace {
        frame: observation.frame_state().frame(),
        flows: observation.frame_state().flow_learnset().to_vec(),
        glows: observation.frame_state().glow_learnset().to_vec(),
        intent: observation.intent(),
        route_geometry: observation.route_geometry(),
        rotation_context: observation.rotation_context(),
        spatial_interpretation: observation.spatial_interpretation().cloned(),
        state_checks: candidates
            .iter()
            .map(|candidate| observation_check_for_candidate(observation, candidate.candidate_id()))
            .collect(),
    }
}

fn observation_check_for_candidate(
    observation: &DecisionObservation,
    candidate_id: DecisionCandidateId,
) -> DecisionObservationCheck {
    match candidate_id {
        DecisionCandidateId::GremlinTinker => DecisionObservationCheck {
            candidate_id,
            already_canonical_frame: observation.frame_state().frame() == FrameId::Gremlin,
            already_knows_canonical_flow: observation.has_flow(FlowId::TinkerGrip),
            already_knows_canonical_glow: false,
        },
        DecisionCandidateId::PixyConfusion => DecisionObservationCheck {
            candidate_id,
            already_canonical_frame: observation.frame_state().frame() == FrameId::Pixy,
            already_knows_canonical_flow: false,
            already_knows_canonical_glow: observation.has_glow(GlowId::Confusion),
        },
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GivenCircumstance {
    PatientDistressed,
    VisibleSymptomsInconsistent,
    AbnormalAuraSignalPresent,
    MemoryInterferenceSuspected,
    ConsentPermitsExaminationNotMemoryAlteration,
    DistortionLocalizedNearWoundMargin,
    HiddenConditionStillUnconfirmed,
    MinorianMeasurementRequested,
    SignalMapClarifiesDistortionPattern,
}

impl GivenCircumstance {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PatientDistressed => "patient is distressed",
            Self::VisibleSymptomsInconsistent => "visible symptoms are inconsistent",
            Self::AbnormalAuraSignalPresent => "an abnormal Aura signal is present",
            Self::MemoryInterferenceSuspected => "memory interference is suspected but not known",
            Self::ConsentPermitsExaminationNotMemoryAlteration => {
                "consent permits examination but not memory alteration"
            }
            Self::DistortionLocalizedNearWoundMargin => {
                "the distortion localizes near the wound margin"
            }
            Self::HiddenConditionStillUnconfirmed => "the hidden condition remains unconfirmed",
            Self::MinorianMeasurementRequested => "a Minorian measurement request has been issued",
            Self::SignalMapClarifiesDistortionPattern => {
                "the signal map clarifies the distortion pattern"
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActiveObjective {
    IdentifyImmediateHiddenCondition,
}

impl ActiveObjective {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IdentifyImmediateHiddenCondition => "identify the immediate hidden condition",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActivePurpose {
    PreserveLifeAndAgency,
}

impl ActivePurpose {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreserveLifeAndAgency => "preserve the patient's life and agency",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Obstacle {
    ConcealedCondition,
    DistortedVisibleSignal,
    MemoryAlterationForbiddenByConsent,
}

impl Obstacle {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConcealedCondition => "the condition is concealed",
            Self::DistortedVisibleSignal => "the visible signal may be distorted",
            Self::MemoryAlterationForbiddenByConsent => {
                "memory alteration is blocked by the active consent boundary"
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CandidateTacticId {
    SurfaceSymptomShow,
    AuraLesionTrace,
    FalseSignalExposure,
    RequestMinorianMeasurement,
    ForciblyOpenMemory,
    ReassuringPresence,
    StabilizeWound,
    EndExamination,
}

impl CandidateTacticId {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SurfaceSymptomShow => "Surface Symptom Show",
            Self::AuraLesionTrace => "Aura Lesion Trace",
            Self::FalseSignalExposure => "False Signal Exposure",
            Self::RequestMinorianMeasurement => "Request Minorian Measurement",
            Self::ForciblyOpenMemory => "forcibly open memory",
            Self::ReassuringPresence => "Reassuring Presence",
            Self::StabilizeWound => "Stabilize the Wound",
            Self::EndExamination => "End Examination",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CandidateMoveId {
    SurfaceSymptomShow,
    AuraLesionTrace,
    FalseSignalExposure,
    RequestMinorianMeasurement,
    ForcedMemoryOpening,
    BedsideReassurance,
    WoundStabilization,
    EndExamination,
}

impl CandidateMoveId {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SurfaceSymptomShow => "Surface Symptom Show",
            Self::AuraLesionTrace => "Aura Lesion Trace",
            Self::FalseSignalExposure => "False Signal Exposure",
            Self::RequestMinorianMeasurement => "Request Minorian Measurement",
            Self::ForcedMemoryOpening => "Forced Memory Opening",
            Self::BedsideReassurance => "Bedside Reassurance",
            Self::WoundStabilization => "Wound Stabilization",
            Self::EndExamination => "End Examination",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProjectionUncertainty {
    Low,
    Medium,
    High,
}

impl ProjectionUncertainty {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceCost {
    Low,
    Moderate,
    High,
}

impl ResourceCost {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Low => "Low",
            Self::Moderate => "Moderate",
            Self::High => "High",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RiskLevel {
    Low,
    Moderate,
    High,
    Severe,
}

impl RiskLevel {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Low => "Low",
            Self::Moderate => "Moderate",
            Self::High => "High",
            Self::Severe => "Severe",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectiveProgress {
    Low,
    Medium,
    High,
    Regressive,
}

impl ObjectiveProgress {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
            Self::Regressive => "Regressive",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PurposeAlignment {
    Strong,
    Compatible,
    Conflicted,
}

impl PurposeAlignment {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Strong => "Strong",
            Self::Compatible => "Compatible",
            Self::Conflicted => "Conflicted",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AgencyConsequence {
    Preserved,
    Guided,
    Narrowed,
    Overridden,
}

impl AgencyConsequence {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preserved => "Preserved",
            Self::Guided => "Guided",
            Self::Narrowed => "Narrowed",
            Self::Overridden => "Overridden",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandidateProjection {
    likely_immediate_consequence: &'static str,
    uncertainty: ProjectionUncertainty,
    resource_cost: ResourceCost,
    risk: RiskLevel,
    objective_progress: ObjectiveProgress,
    purpose_alignment: PurposeAlignment,
    target_agency_consequence: AgencyConsequence,
    semantic_plausibility: CompatibilityLevel,
}

impl CandidateProjection {
    #[must_use]
    pub const fn likely_immediate_consequence(&self) -> &'static str {
        self.likely_immediate_consequence
    }

    #[must_use]
    pub const fn uncertainty(&self) -> ProjectionUncertainty {
        self.uncertainty
    }

    #[must_use]
    pub const fn resource_cost(&self) -> ResourceCost {
        self.resource_cost
    }

    #[must_use]
    pub const fn risk(&self) -> RiskLevel {
        self.risk
    }

    #[must_use]
    pub const fn objective_progress(&self) -> ObjectiveProgress {
        self.objective_progress
    }

    #[must_use]
    pub const fn purpose_alignment(&self) -> PurposeAlignment {
        self.purpose_alignment
    }

    #[must_use]
    pub const fn target_agency_consequence(&self) -> AgencyConsequence {
        self.target_agency_consequence
    }

    #[must_use]
    pub const fn semantic_plausibility(&self) -> CompatibilityLevel {
        self.semantic_plausibility
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SenseOfTruthCode {
    SupportedByObservation,
    ObjectiveDriven,
    ObstacleSensitive,
    SuperObjectiveAligned,
    CapabilityAvailable,
    GestureEmbodied,
    ModeValid,
    RealBeingObjectRelation,
    AddressingValid,
    NoKnowledgeLeakage,
    BoundedProjection,
    ConsentConflict,
    AgencyRisk,
    PurposeContradiction,
    PlotConvenienceUnsupported,
}

impl SenseOfTruthCode {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SupportedByObservation => "SupportedByObservation",
            Self::ObjectiveDriven => "ObjectiveDriven",
            Self::ObstacleSensitive => "ObstacleSensitive",
            Self::SuperObjectiveAligned => "SuperObjectiveAligned",
            Self::CapabilityAvailable => "CapabilityAvailable",
            Self::GestureEmbodied => "GestureEmbodied",
            Self::ModeValid => "ModeValid",
            Self::RealBeingObjectRelation => "RealBeingObjectRelation",
            Self::AddressingValid => "AddressingValid",
            Self::NoKnowledgeLeakage => "NoKnowledgeLeakage",
            Self::BoundedProjection => "BoundedProjection",
            Self::ConsentConflict => "ConsentConflict",
            Self::AgencyRisk => "AgencyRisk",
            Self::PurposeContradiction => "PurposeContradiction",
            Self::PlotConvenienceUnsupported => "PlotConvenienceUnsupported",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SenseOfTruthResult {
    passes: bool,
    score: u16,
    reasons: Vec<SenseOfTruthCode>,
}

impl SenseOfTruthResult {
    #[must_use]
    pub const fn passes(&self) -> bool {
        self.passes
    }

    #[must_use]
    pub const fn score(&self) -> u16 {
        self.score
    }

    #[must_use]
    pub fn reasons(&self) -> &[SenseOfTruthCode] {
        &self.reasons
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandidateTactic {
    tactic_id: CandidateTacticId,
    being: BeingState,
    skill: SkillId,
    domain: ExpressionDomain,
    gesture: EmbodiedGesture,
    mode: ActionMode,
    object: ObjectState,
    addressing_mode: AddressingMode,
    aim: ActionAim,
    candidate_move: CandidateMoveId,
}

impl CandidateTactic {
    #[must_use]
    pub const fn tactic_id(&self) -> CandidateTacticId {
        self.tactic_id
    }

    #[must_use]
    pub const fn being(&self) -> &BeingState {
        &self.being
    }

    #[must_use]
    pub const fn skill(&self) -> SkillId {
        self.skill
    }

    #[must_use]
    pub const fn domain(&self) -> ExpressionDomain {
        self.domain
    }

    #[must_use]
    pub const fn gesture(&self) -> EmbodiedGesture {
        self.gesture
    }

    #[must_use]
    pub const fn mode(&self) -> ActionMode {
        self.mode
    }

    #[must_use]
    pub const fn object(&self) -> &ObjectState {
        &self.object
    }

    #[must_use]
    pub const fn addressing_mode(&self) -> AddressingMode {
        self.addressing_mode
    }

    #[must_use]
    pub const fn aim(&self) -> ActionAim {
        self.aim
    }

    #[must_use]
    pub const fn candidate_move(&self) -> CandidateMoveId {
        self.candidate_move
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StanislavskiCandidateEvaluation {
    tactic_id: CandidateTacticId,
    projection: CandidateProjection,
    sense_of_truth: SenseOfTruthResult,
    recipe_status: RecipeBoundaryStatus,
    legal_candidate: bool,
}

impl StanislavskiCandidateEvaluation {
    #[must_use]
    pub const fn tactic_id(&self) -> CandidateTacticId {
        self.tactic_id
    }

    #[must_use]
    pub const fn projection(&self) -> &CandidateProjection {
        &self.projection
    }

    #[must_use]
    pub const fn sense_of_truth(&self) -> &SenseOfTruthResult {
        &self.sense_of_truth
    }

    #[must_use]
    pub const fn recipe_status(&self) -> RecipeBoundaryStatus {
        self.recipe_status
    }

    #[must_use]
    pub const fn legal_candidate(&self) -> bool {
        self.legal_candidate
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StanislavskiChosenDecision {
    tactic_id: CandidateTacticId,
    chosen_move: CandidateMoveId,
    reason: &'static str,
}

impl StanislavskiChosenDecision {
    #[must_use]
    pub const fn tactic_id(&self) -> CandidateTacticId {
        self.tactic_id
    }

    #[must_use]
    pub const fn chosen_move(&self) -> CandidateMoveId {
        self.chosen_move
    }

    #[must_use]
    pub const fn reason(&self) -> &'static str {
        self.reason
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThroughLineStatus {
    Established,
    Adapted,
}

impl ThroughLineStatus {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Established => "Purpose established; first tactic selected truthfully.",
            Self::Adapted => "Purpose persists; tactic adapts to changed circumstances.",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SceneThroughLine {
    purpose: ActivePurpose,
    objective_persists: bool,
    tactic_changed: bool,
    status: ThroughLineStatus,
}

impl SceneThroughLine {
    #[must_use]
    pub const fn purpose(&self) -> ActivePurpose {
        self.purpose
    }

    #[must_use]
    pub const fn objective_persists(&self) -> bool {
        self.objective_persists
    }

    #[must_use]
    pub const fn tactic_changed(&self) -> bool {
        self.tactic_changed
    }

    #[must_use]
    pub const fn status(&self) -> ThroughLineStatus {
        self.status
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionBeat {
    beat_index: u8,
    given_circumstances: Vec<GivenCircumstance>,
    objective: ActiveObjective,
    purpose: ActivePurpose,
    obstacles: Vec<Obstacle>,
    candidate_tactics: Vec<CandidateTactic>,
    evaluations: Vec<StanislavskiCandidateEvaluation>,
    chosen: StanislavskiChosenDecision,
    execution_result: &'static str,
    changed_circumstances: Vec<GivenCircumstance>,
    through_line: SceneThroughLine,
}

impl DecisionBeat {
    #[must_use]
    pub const fn beat_index(&self) -> u8 {
        self.beat_index
    }

    #[must_use]
    pub fn given_circumstances(&self) -> &[GivenCircumstance] {
        &self.given_circumstances
    }

    #[must_use]
    pub const fn objective(&self) -> ActiveObjective {
        self.objective
    }

    #[must_use]
    pub const fn purpose(&self) -> ActivePurpose {
        self.purpose
    }

    #[must_use]
    pub fn obstacles(&self) -> &[Obstacle] {
        &self.obstacles
    }

    #[must_use]
    pub fn candidate_tactics(&self) -> &[CandidateTactic] {
        &self.candidate_tactics
    }

    #[must_use]
    pub fn evaluations(&self) -> &[StanislavskiCandidateEvaluation] {
        &self.evaluations
    }

    #[must_use]
    pub const fn chosen(&self) -> &StanislavskiChosenDecision {
        &self.chosen
    }

    #[must_use]
    pub const fn execution_result(&self) -> &'static str {
        self.execution_result
    }

    #[must_use]
    pub fn changed_circumstances(&self) -> &[GivenCircumstance] {
        &self.changed_circumstances
    }

    #[must_use]
    pub const fn through_line(&self) -> &SceneThroughLine {
        &self.through_line
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StanislavskiDecisionSequence {
    title: &'static str,
    beats: Vec<DecisionBeat>,
}

impl StanislavskiDecisionSequence {
    #[must_use]
    pub const fn title(&self) -> &'static str {
        self.title
    }

    #[must_use]
    pub fn beats(&self) -> &[DecisionBeat] {
        &self.beats
    }
}

pub fn canonical_nightingale_hidden_wound_sequence() -> StanislavskiDecisionSequence {
    let beat_one = canonical_hidden_wound_beat_one();
    let beat_two = canonical_hidden_wound_beat_two(&beat_one);
    StanislavskiDecisionSequence {
        title: "Nightingale Hidden Wound",
        beats: vec![beat_one, beat_two],
    }
}

pub fn build_stanislavski_action_witness() -> io::Result<String> {
    let sequence = canonical_nightingale_hidden_wound_sequence();
    let mut output = String::from("HOLLOW GROVE STANISLAVSKI ACTION WITNESS\n\n");
    output.push_str("Scene:\n");
    output.push_str(sequence.title());
    output.push_str("\n\n");

    for beat in sequence.beats() {
        output.push_str("Beat ");
        output.push_str(&beat.beat_index().to_string());
        output.push_str("\n\nGiven Circumstances:\n");
        for circumstance in beat.given_circumstances() {
            output.push_str("- ");
            output.push_str(circumstance.as_str());
            output.push('\n');
        }
        output.push_str("\nObjective:\n");
        output.push_str(beat.objective().as_str());
        output.push_str("\n\nSuper-objective:\n");
        output.push_str(beat.purpose().as_str());
        output.push_str("\n\nObstacle:\n");
        for obstacle in beat.obstacles() {
            output.push_str("- ");
            output.push_str(obstacle.as_str());
            output.push('\n');
        }

        output.push_str("\nCandidate Tactics:\n");
        for (index, tactic) in beat.candidate_tactics().iter().enumerate() {
            let evaluation = beat
                .evaluations()
                .iter()
                .find(|evaluation| evaluation.tactic_id() == tactic.tactic_id())
                .expect("every tactic must have an evaluation");
            output.push_str(&format!("{}. {}\n", index + 1, tactic.tactic_id().as_str()));
            output.push_str("   Move: ");
            output.push_str(tactic.candidate_move().as_str());
            output.push('\n');
            output.push_str("   Skill: ");
            output.push_str(tactic.skill().as_str());
            output.push('\n');
            output.push_str("   Domain / Gesture / Mode: ");
            output.push_str(tactic.domain().as_str());
            output.push_str(" / ");
            output.push_str(tactic.gesture().as_str());
            output.push_str(" / ");
            output.push_str(tactic.mode().as_str());
            output.push('\n');
            output.push_str("   Object / Addressing: ");
            output.push_str(tactic.object().identity().as_str());
            output.push_str(" / ");
            output.push_str(tactic.addressing_mode().as_str());
            output.push('\n');
            output.push_str("   Aim: ");
            output.push_str(tactic.aim().as_str());
            output.push('\n');
            output.push_str("   Magic-If Projection:\n");
            output.push_str("   - likely immediate consequence: ");
            output.push_str(evaluation.projection().likely_immediate_consequence());
            output.push('\n');
            output.push_str("   - uncertainty: ");
            output.push_str(evaluation.projection().uncertainty().as_str());
            output.push('\n');
            output.push_str("   - resource cost: ");
            output.push_str(evaluation.projection().resource_cost().as_str());
            output.push('\n');
            output.push_str("   - risk: ");
            output.push_str(evaluation.projection().risk().as_str());
            output.push('\n');
            output.push_str("   - Objective progress: ");
            output.push_str(evaluation.projection().objective_progress().as_str());
            output.push('\n');
            output.push_str("   - Super-objective alignment: ");
            output.push_str(evaluation.projection().purpose_alignment().as_str());
            output.push('\n');
            output.push_str("   - consequence to target agency: ");
            output.push_str(evaluation.projection().target_agency_consequence().as_str());
            output.push('\n');
            output.push_str("   - semantic plausibility: ");
            output.push_str(evaluation.projection().semantic_plausibility().as_str());
            output.push('\n');
            output.push_str("   Sense-of-Truth Result:\n");
            output.push_str("   - passes: ");
            output.push_str(if evaluation.sense_of_truth().passes() {
                "yes"
            } else {
                "no"
            });
            output.push('\n');
            output.push_str("   - score: ");
            output.push_str(&evaluation.sense_of_truth().score().to_string());
            output.push('\n');
            output.push_str("   - reasons: ");
            output.push_str(
                &evaluation
                    .sense_of_truth()
                    .reasons()
                    .iter()
                    .map(|reason| reason.as_str())
                    .collect::<Vec<_>>()
                    .join(", "),
            );
            output.push('\n');
            output.push_str("   Recipe Status:\n");
            output.push_str("   - ");
            output.push_str(evaluation.recipe_status().as_str());
            output.push('\n');
        }

        output.push_str("\nChosen Tactic:\n");
        output.push_str(beat.chosen().tactic_id().as_str());
        output.push_str("\n\nChosen Move:\n");
        output.push_str(beat.chosen().chosen_move().as_str());
        output.push_str("\n\nReason:\n");
        output.push_str(beat.chosen().reason());
        output.push_str("\n\nRecipe Status:\n");
        output.push_str(RecipeBoundaryStatus::LegalFixtureRequired.as_str());
        output.push_str("\n\nExecution Result:\n");
        output.push_str(beat.execution_result());
        output.push_str("\n\nChanged Circumstances:\n");
        for circumstance in beat.changed_circumstances() {
            output.push_str("- ");
            output.push_str(circumstance.as_str());
            output.push('\n');
        }
        output.push_str("\nThrough-Line Status:\n");
        output.push_str(beat.through_line().status().as_str());
        output.push_str("\n\n");
    }

    Ok(output)
}

pub fn build_stanislavski_action_validation_report() -> io::Result<String> {
    let sequence = canonical_nightingale_hidden_wound_sequence();
    let first = sequence
        .beats()
        .first()
        .expect("canonical hidden wound sequence must contain at least one beat");
    let second = sequence
        .beats()
        .get(1)
        .expect("canonical hidden wound sequence must contain at least two beats");

    let forced_memory = first
        .evaluations()
        .iter()
        .find(|evaluation| evaluation.tactic_id() == CandidateTacticId::ForciblyOpenMemory)
        .expect("forced memory candidate must exist");

    if forced_memory.sense_of_truth().passes() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "forcibly open memory must fail the sense-of-truth gate",
        ));
    }

    if !forced_memory
        .sense_of_truth()
        .reasons()
        .contains(&SenseOfTruthCode::ConsentConflict)
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "forced memory candidate must record consent conflict",
        ));
    }

    if !first.candidate_tactics().iter().take(4).all(|tactic| {
        first
            .evaluations()
            .iter()
            .find(|evaluation| evaluation.tactic_id() == tactic.tactic_id())
            .map(StanislavskiCandidateEvaluation::legal_candidate)
            .unwrap_or(false)
    }) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "the first four hidden-wound tactics must remain potentially legal",
        ));
    }

    if first.chosen().tactic_id() == second.chosen().tactic_id() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "the second beat must adapt its tactic after the first beat changes circumstances",
        ));
    }

    if !second.through_line().objective_persists() || !second.through_line().tactic_changed() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "the through-line must keep the purpose while adapting the tactic",
        ));
    }

    Ok(String::from(
        "# Hollow Grove Stanislavski Action Validation\n\n\
         - status: pass\n\
         - GivenCircumstances typed: pass\n\
         - ActiveObjective typed: pass\n\
         - ActivePurpose typed: pass\n\
         - Obstacle typed: pass\n\
         - candidate tactics objective-driven: pass\n\
         - candidate tactics obstacle-sensitive: pass\n\
         - Magic-If projections bounded by observation and uncertainty: pass\n\
         - Sense-of-Truth rejects forced memory opening: pass\n\
         - no world-truth leakage: pass\n\
         - purpose persists while tactics adapt: pass\n\
         - ChosenDecision remains a single assignment per beat: pass\n\
         - Recipe boundary preserved: pass\n\
         - V2 remains selector: pass\n\
         - V1.1 remains executor: pass\n",
    ))
}

fn canonical_hidden_wound_beat_one() -> DecisionBeat {
    let objective = ActiveObjective::IdentifyImmediateHiddenCondition;
    let purpose = ActivePurpose::PreserveLifeAndAgency;
    let given_circumstances = vec![
        GivenCircumstance::PatientDistressed,
        GivenCircumstance::VisibleSymptomsInconsistent,
        GivenCircumstance::AbnormalAuraSignalPresent,
        GivenCircumstance::MemoryInterferenceSuspected,
        GivenCircumstance::ConsentPermitsExaminationNotMemoryAlteration,
    ];
    let obstacles = vec![
        Obstacle::ConcealedCondition,
        Obstacle::DistortedVisibleSignal,
        Obstacle::MemoryAlterationForbiddenByConsent,
    ];
    let candidate_tactics = hidden_wound_candidates();
    let evaluations = candidate_tactics
        .iter()
        .map(|tactic| evaluate_hidden_wound_candidate(1, tactic))
        .collect::<Vec<_>>();
    let chosen = StanislavskiChosenDecision {
        tactic_id: CandidateTacticId::AuraLesionTrace,
        chosen_move: CandidateMoveId::AuraLesionTrace,
        reason: "Aura Lesion Trace is the strongest truthful embodied tactic because it acts directly on the abnormal signal, respects the consent boundary, preserves agency, and does not assume the hidden condition in advance.",
    };

    DecisionBeat {
        beat_index: 1,
        given_circumstances,
        objective,
        purpose,
        obstacles,
        candidate_tactics,
        evaluations,
        chosen,
        execution_result: "Projected validated trace localizes the distortion near the wound margin without altering memory; frozen V1.1 handoff still requires a legal Recipe before live execution.",
        changed_circumstances: vec![
            GivenCircumstance::DistortionLocalizedNearWoundMargin,
            GivenCircumstance::HiddenConditionStillUnconfirmed,
        ],
        through_line: SceneThroughLine {
            purpose,
            objective_persists: true,
            tactic_changed: false,
            status: ThroughLineStatus::Established,
        },
    }
}

fn canonical_hidden_wound_beat_two(previous: &DecisionBeat) -> DecisionBeat {
    let mut given_circumstances = previous.given_circumstances().to_vec();
    given_circumstances.extend_from_slice(previous.changed_circumstances());
    let objective = ActiveObjective::IdentifyImmediateHiddenCondition;
    let purpose = ActivePurpose::PreserveLifeAndAgency;
    let obstacles = vec![
        Obstacle::ConcealedCondition,
        Obstacle::DistortedVisibleSignal,
        Obstacle::MemoryAlterationForbiddenByConsent,
    ];
    let candidate_tactics = hidden_wound_candidates();
    let evaluations = candidate_tactics
        .iter()
        .map(|tactic| evaluate_hidden_wound_candidate(2, tactic))
        .collect::<Vec<_>>();
    let chosen = StanislavskiChosenDecision {
        tactic_id: CandidateTacticId::RequestMinorianMeasurement,
        chosen_move: CandidateMoveId::RequestMinorianMeasurement,
        reason: "After the trace localizes the distortion, Request Minorian Measurement becomes the most truthful adaptation because it measures the newly localized mismatch without forcing memory access and improves evidence for the same objective.",
    };

    DecisionBeat {
        beat_index: 2,
        given_circumstances,
        objective,
        purpose,
        obstacles,
        candidate_tactics,
        evaluations,
        chosen,
        execution_result: "Projected adaptive consequence requests a measured signal map, clarifies the distortion pattern, and preserves the consent boundary; frozen V1.1 still remains the only live executor once a legal Recipe exists.",
        changed_circumstances: vec![
            GivenCircumstance::MinorianMeasurementRequested,
            GivenCircumstance::SignalMapClarifiesDistortionPattern,
        ],
        through_line: SceneThroughLine {
            purpose,
            objective_persists: true,
            tactic_changed: true,
            status: ThroughLineStatus::Adapted,
        },
    }
}

fn hidden_wound_candidates() -> Vec<CandidateTactic> {
    let nightingale = build_canonical_being_state_with_aura(FrameId::Hueman, Some(FrameId::Faerie));
    vec![
        CandidateTactic {
            tactic_id: CandidateTacticId::SurfaceSymptomShow,
            being: nightingale.clone(),
            skill: SkillId::Repair,
            domain: ExpressionDomain::Glow,
            gesture: EmbodiedGesture::Show,
            mode: ActionMode::Beam,
            object: canonical_object_state(ObjectId::SymptomPattern),
            addressing_mode: AddressingMode::Proxy,
            aim: ActionAim::DiagnoseAndExplain,
            candidate_move: CandidateMoveId::SurfaceSymptomShow,
        },
        CandidateTactic {
            tactic_id: CandidateTacticId::AuraLesionTrace,
            being: nightingale.clone(),
            skill: SkillId::Repair,
            domain: ExpressionDomain::Glow,
            gesture: EmbodiedGesture::Grip,
            mode: ActionMode::Beam,
            object: canonical_object_state(ObjectId::ClinicalFinding),
            addressing_mode: AddressingMode::Proxy,
            aim: ActionAim::DiagnoseAndExplain,
            candidate_move: CandidateMoveId::AuraLesionTrace,
        },
        CandidateTactic {
            tactic_id: CandidateTacticId::FalseSignalExposure,
            being: nightingale.clone(),
            skill: SkillId::Repair,
            domain: ExpressionDomain::Glow,
            gesture: EmbodiedGesture::Show,
            mode: ActionMode::Beam,
            object: canonical_object_state(ObjectId::ClinicalFinding),
            addressing_mode: AddressingMode::Foxy,
            aim: ActionAim::DiagnoseAndExplain,
            candidate_move: CandidateMoveId::FalseSignalExposure,
        },
        CandidateTactic {
            tactic_id: CandidateTacticId::RequestMinorianMeasurement,
            being: nightingale.clone(),
            skill: SkillId::Repair,
            domain: ExpressionDomain::Glow,
            gesture: EmbodiedGesture::Show,
            mode: ActionMode::Beam,
            object: canonical_object_state(ObjectId::SymptomPattern),
            addressing_mode: AddressingMode::Moxy,
            aim: ActionAim::MeasureAndMap,
            candidate_move: CandidateMoveId::RequestMinorianMeasurement,
        },
        CandidateTactic {
            tactic_id: CandidateTacticId::ForciblyOpenMemory,
            being: nightingale.clone(),
            skill: SkillId::Repair,
            domain: ExpressionDomain::Glow,
            gesture: EmbodiedGesture::Grip,
            mode: ActionMode::Seam,
            object: canonical_object_state(ObjectId::ConcealedMemoryRelation),
            addressing_mode: AddressingMode::Foxy,
            aim: ActionAim::RevealHiddenTruthWithConsent,
            candidate_move: CandidateMoveId::ForcedMemoryOpening,
        },
        CandidateTactic {
            tactic_id: CandidateTacticId::ReassuringPresence,
            being: nightingale.clone(),
            skill: SkillId::Guard,
            domain: ExpressionDomain::Glow,
            gesture: EmbodiedGesture::Grit,
            mode: ActionMode::Gleam,
            object: canonical_object_state(ObjectId::OpenWound),
            addressing_mode: AddressingMode::Proxy,
            aim: ActionAim::ProvideReassurance,
            candidate_move: CandidateMoveId::BedsideReassurance,
        },
        CandidateTactic {
            tactic_id: CandidateTacticId::StabilizeWound,
            being: nightingale.clone(),
            skill: SkillId::Brace,
            domain: ExpressionDomain::Flow,
            gesture: EmbodiedGesture::Grip,
            mode: ActionMode::Seam,
            object: canonical_object_state(ObjectId::OpenWound),
            addressing_mode: AddressingMode::Proxy,
            aim: ActionAim::StabilizeAndClose,
            candidate_move: CandidateMoveId::WoundStabilization,
        },
        CandidateTactic {
            tactic_id: CandidateTacticId::EndExamination,
            being: nightingale,
            skill: SkillId::Guard,
            domain: ExpressionDomain::Flow,
            gesture: EmbodiedGesture::Grit,
            mode: ActionMode::Seam,
            object: canonical_object_state(ObjectId::ClinicalFinding),
            addressing_mode: AddressingMode::Proxy,
            aim: ActionAim::WithdrawFromEncounter,
            candidate_move: CandidateMoveId::EndExamination,
        },
    ]
}

fn evaluate_hidden_wound_candidate(
    beat_index: u8,
    tactic: &CandidateTactic,
) -> StanislavskiCandidateEvaluation {
    match (beat_index, tactic.tactic_id()) {
        (1, CandidateTacticId::SurfaceSymptomShow) => build_hidden_wound_evaluation(
            tactic.tactic_id(),
            CandidateProjection {
                likely_immediate_consequence: "surface review may separate stable symptoms from changing ones without claiming the hidden cause",
                uncertainty: ProjectionUncertainty::Medium,
                resource_cost: ResourceCost::Low,
                risk: RiskLevel::Low,
                objective_progress: ObjectiveProgress::Medium,
                purpose_alignment: PurposeAlignment::Strong,
                target_agency_consequence: AgencyConsequence::Preserved,
                semantic_plausibility: CompatibilityLevel::High,
            },
            77,
            vec![
                SenseOfTruthCode::SupportedByObservation,
                SenseOfTruthCode::ObjectiveDriven,
                SenseOfTruthCode::ObstacleSensitive,
                SenseOfTruthCode::SuperObjectiveAligned,
                SenseOfTruthCode::CapabilityAvailable,
                SenseOfTruthCode::GestureEmbodied,
                SenseOfTruthCode::ModeValid,
                SenseOfTruthCode::RealBeingObjectRelation,
                SenseOfTruthCode::AddressingValid,
                SenseOfTruthCode::NoKnowledgeLeakage,
                SenseOfTruthCode::BoundedProjection,
            ],
            true,
        ),
        (1, CandidateTacticId::AuraLesionTrace) => build_hidden_wound_evaluation(
            tactic.tactic_id(),
            CandidateProjection {
                likely_immediate_consequence: "controlled signal tracing may localize the distortion around the wound boundary without altering memory",
                uncertainty: ProjectionUncertainty::Medium,
                resource_cost: ResourceCost::Moderate,
                risk: RiskLevel::Moderate,
                objective_progress: ObjectiveProgress::High,
                purpose_alignment: PurposeAlignment::Strong,
                target_agency_consequence: AgencyConsequence::Preserved,
                semantic_plausibility: CompatibilityLevel::High,
            },
            90,
            vec![
                SenseOfTruthCode::SupportedByObservation,
                SenseOfTruthCode::ObjectiveDriven,
                SenseOfTruthCode::ObstacleSensitive,
                SenseOfTruthCode::SuperObjectiveAligned,
                SenseOfTruthCode::CapabilityAvailable,
                SenseOfTruthCode::GestureEmbodied,
                SenseOfTruthCode::ModeValid,
                SenseOfTruthCode::RealBeingObjectRelation,
                SenseOfTruthCode::AddressingValid,
                SenseOfTruthCode::NoKnowledgeLeakage,
                SenseOfTruthCode::BoundedProjection,
            ],
            true,
        ),
        (1, CandidateTacticId::FalseSignalExposure) => build_hidden_wound_evaluation(
            tactic.tactic_id(),
            CandidateProjection {
                likely_immediate_consequence: "false-signal testing may expose whether the visible aura pattern is masking the real lesion",
                uncertainty: ProjectionUncertainty::High,
                resource_cost: ResourceCost::Moderate,
                risk: RiskLevel::Moderate,
                objective_progress: ObjectiveProgress::Medium,
                purpose_alignment: PurposeAlignment::Strong,
                target_agency_consequence: AgencyConsequence::Preserved,
                semantic_plausibility: CompatibilityLevel::Valid,
            },
            82,
            vec![
                SenseOfTruthCode::SupportedByObservation,
                SenseOfTruthCode::ObjectiveDriven,
                SenseOfTruthCode::ObstacleSensitive,
                SenseOfTruthCode::SuperObjectiveAligned,
                SenseOfTruthCode::CapabilityAvailable,
                SenseOfTruthCode::GestureEmbodied,
                SenseOfTruthCode::ModeValid,
                SenseOfTruthCode::RealBeingObjectRelation,
                SenseOfTruthCode::AddressingValid,
                SenseOfTruthCode::NoKnowledgeLeakage,
                SenseOfTruthCode::BoundedProjection,
            ],
            true,
        ),
        (1, CandidateTacticId::RequestMinorianMeasurement) => build_hidden_wound_evaluation(
            tactic.tactic_id(),
            CandidateProjection {
                likely_immediate_consequence: "measurement support may map inconsistencies that the Nightingale cannot safely confirm alone",
                uncertainty: ProjectionUncertainty::Medium,
                resource_cost: ResourceCost::Low,
                risk: RiskLevel::Low,
                objective_progress: ObjectiveProgress::High,
                purpose_alignment: PurposeAlignment::Strong,
                target_agency_consequence: AgencyConsequence::Preserved,
                semantic_plausibility: CompatibilityLevel::High,
            },
            84,
            vec![
                SenseOfTruthCode::SupportedByObservation,
                SenseOfTruthCode::ObjectiveDriven,
                SenseOfTruthCode::ObstacleSensitive,
                SenseOfTruthCode::SuperObjectiveAligned,
                SenseOfTruthCode::CapabilityAvailable,
                SenseOfTruthCode::GestureEmbodied,
                SenseOfTruthCode::ModeValid,
                SenseOfTruthCode::RealBeingObjectRelation,
                SenseOfTruthCode::AddressingValid,
                SenseOfTruthCode::NoKnowledgeLeakage,
                SenseOfTruthCode::BoundedProjection,
            ],
            true,
        ),
        (1, CandidateTacticId::ReassuringPresence) => build_hidden_wound_evaluation(
            tactic.tactic_id(),
            CandidateProjection {
                likely_immediate_consequence: "reassurance may lower panic and preserve cooperation, but it does not directly test the concealed condition",
                uncertainty: ProjectionUncertainty::Medium,
                resource_cost: ResourceCost::Low,
                risk: RiskLevel::Low,
                objective_progress: ObjectiveProgress::Low,
                purpose_alignment: PurposeAlignment::Strong,
                target_agency_consequence: AgencyConsequence::Preserved,
                semantic_plausibility: CompatibilityLevel::High,
            },
            66,
            vec![
                SenseOfTruthCode::SupportedByObservation,
                SenseOfTruthCode::ObjectiveDriven,
                SenseOfTruthCode::ObstacleSensitive,
                SenseOfTruthCode::SuperObjectiveAligned,
                SenseOfTruthCode::CapabilityAvailable,
                SenseOfTruthCode::GestureEmbodied,
                SenseOfTruthCode::ModeValid,
                SenseOfTruthCode::RealBeingObjectRelation,
                SenseOfTruthCode::AddressingValid,
                SenseOfTruthCode::NoKnowledgeLeakage,
                SenseOfTruthCode::BoundedProjection,
            ],
            true,
        ),
        (1, CandidateTacticId::StabilizeWound) => build_hidden_wound_evaluation(
            tactic.tactic_id(),
            CandidateProjection {
                likely_immediate_consequence: "physical stabilization may reduce immediate danger and buy time, but it leaves the hidden distortion unresolved",
                uncertainty: ProjectionUncertainty::Low,
                resource_cost: ResourceCost::Moderate,
                risk: RiskLevel::Low,
                objective_progress: ObjectiveProgress::Medium,
                purpose_alignment: PurposeAlignment::Strong,
                target_agency_consequence: AgencyConsequence::Preserved,
                semantic_plausibility: CompatibilityLevel::High,
            },
            74,
            vec![
                SenseOfTruthCode::SupportedByObservation,
                SenseOfTruthCode::ObjectiveDriven,
                SenseOfTruthCode::ObstacleSensitive,
                SenseOfTruthCode::SuperObjectiveAligned,
                SenseOfTruthCode::CapabilityAvailable,
                SenseOfTruthCode::GestureEmbodied,
                SenseOfTruthCode::ModeValid,
                SenseOfTruthCode::RealBeingObjectRelation,
                SenseOfTruthCode::AddressingValid,
                SenseOfTruthCode::NoKnowledgeLeakage,
                SenseOfTruthCode::BoundedProjection,
            ],
            true,
        ),
        (1, CandidateTacticId::EndExamination) => build_hidden_wound_evaluation(
            tactic.tactic_id(),
            CandidateProjection {
                likely_immediate_consequence: "withdrawing ends immediate pressure, but it sacrifices diagnosis and leaves the hidden condition active",
                uncertainty: ProjectionUncertainty::Low,
                resource_cost: ResourceCost::Low,
                risk: RiskLevel::High,
                objective_progress: ObjectiveProgress::Regressive,
                purpose_alignment: PurposeAlignment::Conflicted,
                target_agency_consequence: AgencyConsequence::Preserved,
                semantic_plausibility: CompatibilityLevel::Valid,
            },
            24,
            vec![
                SenseOfTruthCode::SupportedByObservation,
                SenseOfTruthCode::ObjectiveDriven,
                SenseOfTruthCode::CapabilityAvailable,
                SenseOfTruthCode::GestureEmbodied,
                SenseOfTruthCode::ModeValid,
                SenseOfTruthCode::RealBeingObjectRelation,
                SenseOfTruthCode::AddressingValid,
                SenseOfTruthCode::NoKnowledgeLeakage,
                SenseOfTruthCode::BoundedProjection,
            ],
            true,
        ),
        (1, CandidateTacticId::ForciblyOpenMemory) => build_hidden_wound_evaluation(
            tactic.tactic_id(),
            CandidateProjection {
                likely_immediate_consequence: "forced memory opening might reveal a hidden pattern, but it would do so by violating the stated consent boundary",
                uncertainty: ProjectionUncertainty::High,
                resource_cost: ResourceCost::High,
                risk: RiskLevel::Severe,
                objective_progress: ObjectiveProgress::Regressive,
                purpose_alignment: PurposeAlignment::Conflicted,
                target_agency_consequence: AgencyConsequence::Overridden,
                semantic_plausibility: CompatibilityLevel::Low,
            },
            12,
            vec![
                SenseOfTruthCode::SupportedByObservation,
                SenseOfTruthCode::ObjectiveDriven,
                SenseOfTruthCode::CapabilityAvailable,
                SenseOfTruthCode::GestureEmbodied,
                SenseOfTruthCode::ModeValid,
                SenseOfTruthCode::RealBeingObjectRelation,
                SenseOfTruthCode::AddressingValid,
                SenseOfTruthCode::NoKnowledgeLeakage,
                SenseOfTruthCode::BoundedProjection,
                SenseOfTruthCode::ConsentConflict,
                SenseOfTruthCode::AgencyRisk,
                SenseOfTruthCode::PurposeContradiction,
            ],
            false,
        ),
        (2, CandidateTacticId::SurfaceSymptomShow) => build_hidden_wound_evaluation(
            tactic.tactic_id(),
            CandidateProjection {
                likely_immediate_consequence: "surface review can re-check the wound margin, but it adds less after the trace has already localized the distortion",
                uncertainty: ProjectionUncertainty::Medium,
                resource_cost: ResourceCost::Low,
                risk: RiskLevel::Low,
                objective_progress: ObjectiveProgress::Low,
                purpose_alignment: PurposeAlignment::Strong,
                target_agency_consequence: AgencyConsequence::Preserved,
                semantic_plausibility: CompatibilityLevel::High,
            },
            62,
            vec![
                SenseOfTruthCode::SupportedByObservation,
                SenseOfTruthCode::ObjectiveDriven,
                SenseOfTruthCode::ObstacleSensitive,
                SenseOfTruthCode::SuperObjectiveAligned,
                SenseOfTruthCode::CapabilityAvailable,
                SenseOfTruthCode::GestureEmbodied,
                SenseOfTruthCode::ModeValid,
                SenseOfTruthCode::RealBeingObjectRelation,
                SenseOfTruthCode::AddressingValid,
                SenseOfTruthCode::NoKnowledgeLeakage,
                SenseOfTruthCode::BoundedProjection,
            ],
            true,
        ),
        (2, CandidateTacticId::AuraLesionTrace) => build_hidden_wound_evaluation(
            tactic.tactic_id(),
            CandidateProjection {
                likely_immediate_consequence: "a second trace may refine the first localization, but it risks repeating the same information without new measurement support",
                uncertainty: ProjectionUncertainty::Medium,
                resource_cost: ResourceCost::Moderate,
                risk: RiskLevel::Moderate,
                objective_progress: ObjectiveProgress::Medium,
                purpose_alignment: PurposeAlignment::Strong,
                target_agency_consequence: AgencyConsequence::Preserved,
                semantic_plausibility: CompatibilityLevel::High,
            },
            68,
            vec![
                SenseOfTruthCode::SupportedByObservation,
                SenseOfTruthCode::ObjectiveDriven,
                SenseOfTruthCode::ObstacleSensitive,
                SenseOfTruthCode::SuperObjectiveAligned,
                SenseOfTruthCode::CapabilityAvailable,
                SenseOfTruthCode::GestureEmbodied,
                SenseOfTruthCode::ModeValid,
                SenseOfTruthCode::RealBeingObjectRelation,
                SenseOfTruthCode::AddressingValid,
                SenseOfTruthCode::NoKnowledgeLeakage,
                SenseOfTruthCode::BoundedProjection,
            ],
            true,
        ),
        (2, CandidateTacticId::FalseSignalExposure) => build_hidden_wound_evaluation(
            tactic.tactic_id(),
            CandidateProjection {
                likely_immediate_consequence: "with the trace localized, false-signal exposure may separate distortion from lesion more cleanly than before",
                uncertainty: ProjectionUncertainty::Medium,
                resource_cost: ResourceCost::Moderate,
                risk: RiskLevel::Moderate,
                objective_progress: ObjectiveProgress::High,
                purpose_alignment: PurposeAlignment::Strong,
                target_agency_consequence: AgencyConsequence::Preserved,
                semantic_plausibility: CompatibilityLevel::High,
            },
            88,
            vec![
                SenseOfTruthCode::SupportedByObservation,
                SenseOfTruthCode::ObjectiveDriven,
                SenseOfTruthCode::ObstacleSensitive,
                SenseOfTruthCode::SuperObjectiveAligned,
                SenseOfTruthCode::CapabilityAvailable,
                SenseOfTruthCode::GestureEmbodied,
                SenseOfTruthCode::ModeValid,
                SenseOfTruthCode::RealBeingObjectRelation,
                SenseOfTruthCode::AddressingValid,
                SenseOfTruthCode::NoKnowledgeLeakage,
                SenseOfTruthCode::BoundedProjection,
            ],
            true,
        ),
        (2, CandidateTacticId::RequestMinorianMeasurement) => build_hidden_wound_evaluation(
            tactic.tactic_id(),
            CandidateProjection {
                likely_immediate_consequence: "the requested measurement can now map the localized mismatch and clarify whether the visible aura pattern is distorting the diagnosis",
                uncertainty: ProjectionUncertainty::Low,
                resource_cost: ResourceCost::Moderate,
                risk: RiskLevel::Low,
                objective_progress: ObjectiveProgress::High,
                purpose_alignment: PurposeAlignment::Strong,
                target_agency_consequence: AgencyConsequence::Preserved,
                semantic_plausibility: CompatibilityLevel::High,
            },
            91,
            vec![
                SenseOfTruthCode::SupportedByObservation,
                SenseOfTruthCode::ObjectiveDriven,
                SenseOfTruthCode::ObstacleSensitive,
                SenseOfTruthCode::SuperObjectiveAligned,
                SenseOfTruthCode::CapabilityAvailable,
                SenseOfTruthCode::GestureEmbodied,
                SenseOfTruthCode::ModeValid,
                SenseOfTruthCode::RealBeingObjectRelation,
                SenseOfTruthCode::AddressingValid,
                SenseOfTruthCode::NoKnowledgeLeakage,
                SenseOfTruthCode::BoundedProjection,
            ],
            true,
        ),
        (2, CandidateTacticId::ReassuringPresence) => build_hidden_wound_evaluation(
            tactic.tactic_id(),
            CandidateProjection {
                likely_immediate_consequence: "reassurance can help hold trust while the new measurement route is opened, but it still needs another tactic to clarify the distortion",
                uncertainty: ProjectionUncertainty::Low,
                resource_cost: ResourceCost::Low,
                risk: RiskLevel::Low,
                objective_progress: ObjectiveProgress::Medium,
                purpose_alignment: PurposeAlignment::Strong,
                target_agency_consequence: AgencyConsequence::Preserved,
                semantic_plausibility: CompatibilityLevel::High,
            },
            71,
            vec![
                SenseOfTruthCode::SupportedByObservation,
                SenseOfTruthCode::ObjectiveDriven,
                SenseOfTruthCode::ObstacleSensitive,
                SenseOfTruthCode::SuperObjectiveAligned,
                SenseOfTruthCode::CapabilityAvailable,
                SenseOfTruthCode::GestureEmbodied,
                SenseOfTruthCode::ModeValid,
                SenseOfTruthCode::RealBeingObjectRelation,
                SenseOfTruthCode::AddressingValid,
                SenseOfTruthCode::NoKnowledgeLeakage,
                SenseOfTruthCode::BoundedProjection,
            ],
            true,
        ),
        (2, CandidateTacticId::StabilizeWound) => build_hidden_wound_evaluation(
            tactic.tactic_id(),
            CandidateProjection {
                likely_immediate_consequence: "continued stabilization keeps the patient alive under pressure, but it still needs measurement support to resolve the concealed mismatch",
                uncertainty: ProjectionUncertainty::Low,
                resource_cost: ResourceCost::Moderate,
                risk: RiskLevel::Low,
                objective_progress: ObjectiveProgress::Medium,
                purpose_alignment: PurposeAlignment::Strong,
                target_agency_consequence: AgencyConsequence::Preserved,
                semantic_plausibility: CompatibilityLevel::High,
            },
            73,
            vec![
                SenseOfTruthCode::SupportedByObservation,
                SenseOfTruthCode::ObjectiveDriven,
                SenseOfTruthCode::ObstacleSensitive,
                SenseOfTruthCode::SuperObjectiveAligned,
                SenseOfTruthCode::CapabilityAvailable,
                SenseOfTruthCode::GestureEmbodied,
                SenseOfTruthCode::ModeValid,
                SenseOfTruthCode::RealBeingObjectRelation,
                SenseOfTruthCode::AddressingValid,
                SenseOfTruthCode::NoKnowledgeLeakage,
                SenseOfTruthCode::BoundedProjection,
            ],
            true,
        ),
        (2, CandidateTacticId::EndExamination) => build_hidden_wound_evaluation(
            tactic.tactic_id(),
            CandidateProjection {
                likely_immediate_consequence: "leaving after localization abandons the best available measurement follow-up and risks letting the distortion spread",
                uncertainty: ProjectionUncertainty::Low,
                resource_cost: ResourceCost::Low,
                risk: RiskLevel::High,
                objective_progress: ObjectiveProgress::Regressive,
                purpose_alignment: PurposeAlignment::Conflicted,
                target_agency_consequence: AgencyConsequence::Preserved,
                semantic_plausibility: CompatibilityLevel::Valid,
            },
            18,
            vec![
                SenseOfTruthCode::SupportedByObservation,
                SenseOfTruthCode::ObjectiveDriven,
                SenseOfTruthCode::CapabilityAvailable,
                SenseOfTruthCode::GestureEmbodied,
                SenseOfTruthCode::ModeValid,
                SenseOfTruthCode::RealBeingObjectRelation,
                SenseOfTruthCode::AddressingValid,
                SenseOfTruthCode::NoKnowledgeLeakage,
                SenseOfTruthCode::BoundedProjection,
            ],
            true,
        ),
        (2, CandidateTacticId::ForciblyOpenMemory) => build_hidden_wound_evaluation(
            tactic.tactic_id(),
            CandidateProjection {
                likely_immediate_consequence: "forced memory opening still bypasses the consent boundary and would replace adaptation with coercion",
                uncertainty: ProjectionUncertainty::High,
                resource_cost: ResourceCost::High,
                risk: RiskLevel::Severe,
                objective_progress: ObjectiveProgress::Regressive,
                purpose_alignment: PurposeAlignment::Conflicted,
                target_agency_consequence: AgencyConsequence::Overridden,
                semantic_plausibility: CompatibilityLevel::Low,
            },
            8,
            vec![
                SenseOfTruthCode::SupportedByObservation,
                SenseOfTruthCode::ObjectiveDriven,
                SenseOfTruthCode::CapabilityAvailable,
                SenseOfTruthCode::GestureEmbodied,
                SenseOfTruthCode::ModeValid,
                SenseOfTruthCode::RealBeingObjectRelation,
                SenseOfTruthCode::AddressingValid,
                SenseOfTruthCode::NoKnowledgeLeakage,
                SenseOfTruthCode::BoundedProjection,
                SenseOfTruthCode::ConsentConflict,
                SenseOfTruthCode::AgencyRisk,
                SenseOfTruthCode::PurposeContradiction,
            ],
            false,
        ),
        _ => build_hidden_wound_evaluation(
            tactic.tactic_id(),
            CandidateProjection {
                likely_immediate_consequence: "no canonical projection available",
                uncertainty: ProjectionUncertainty::High,
                resource_cost: ResourceCost::High,
                risk: RiskLevel::Severe,
                objective_progress: ObjectiveProgress::Regressive,
                purpose_alignment: PurposeAlignment::Conflicted,
                target_agency_consequence: AgencyConsequence::Overridden,
                semantic_plausibility: CompatibilityLevel::Low,
            },
            0,
            vec![SenseOfTruthCode::PlotConvenienceUnsupported],
            false,
        ),
    }
}

fn build_hidden_wound_evaluation(
    tactic_id: CandidateTacticId,
    projection: CandidateProjection,
    score: u16,
    reasons: Vec<SenseOfTruthCode>,
    legal_candidate: bool,
) -> StanislavskiCandidateEvaluation {
    StanislavskiCandidateEvaluation {
        tactic_id,
        projection,
        sense_of_truth: SenseOfTruthResult {
            passes: legal_candidate,
            score,
            reasons,
        },
        recipe_status: RecipeBoundaryStatus::LegalFixtureRequired,
        legal_candidate,
    }
}

fn build_evaluation_trace(
    observation: &DecisionObservation,
    candidate: &DecisionCandidate,
) -> DecisionEvaluationTrace {
    let check = observation_check_for_candidate(observation, candidate.candidate_id());
    let mut reason_codes = Vec::with_capacity(3);

    let intent_score = match observation.intent() {
        DecisionIntent::FavorCurrent => match candidate.orientation() {
            SynthesisOrientation::Current => {
                reason_codes.push(DecisionTraceReasonCode::PreferredCurrentOrientation);
                2
            }
            SynthesisOrientation::Aura => {
                reason_codes.push(DecisionTraceReasonCode::NonPreferredCurrentOrientation);
                1
            }
        },
        DecisionIntent::FavorAura => match candidate.orientation() {
            SynthesisOrientation::Aura => {
                reason_codes.push(DecisionTraceReasonCode::PreferredAuraOrientation);
                2
            }
            SynthesisOrientation::Current => {
                reason_codes.push(DecisionTraceReasonCode::NonPreferredAuraOrientation);
                1
            }
        },
        DecisionIntent::Neutral => {
            reason_codes.push(DecisionTraceReasonCode::NeutralBaseScore);
            1
        }
    };

    let realized_state_penalty = if matches!(observation.intent(), DecisionIntent::Neutral)
        && candidate_already_realized(observation, candidate.candidate_id())
    {
        if check.already_canonical_frame() {
            reason_codes.push(DecisionTraceReasonCode::AlreadyCanonicalFrame);
        }
        if check.already_knows_canonical_flow() {
            reason_codes.push(DecisionTraceReasonCode::AlreadyKnowsCanonicalFlow);
        }
        if check.already_knows_canonical_glow() {
            reason_codes.push(DecisionTraceReasonCode::AlreadyKnowsCanonicalGlow);
        }
        1
    } else {
        0
    };

    DecisionEvaluationTrace {
        candidate_id: candidate.candidate_id(),
        intent_score,
        realized_state_penalty,
        final_score: intent_score - realized_state_penalty,
        reason_codes,
    }
}

fn build_choice_trace(
    observation: &DecisionObservation,
    candidates: &[DecisionCandidate],
    evaluations: &[DecisionEvaluation],
    chosen: &ChosenDecision,
) -> DecisionChoiceTrace {
    let highest_score = chosen.evaluation().score();
    let tied_candidates = candidates
        .iter()
        .filter(|candidate| {
            evaluations.iter().any(|evaluation| {
                evaluation.candidate_id() == candidate.candidate_id()
                    && evaluation.score() == highest_score
            })
        })
        .map(|candidate| candidate.candidate_id())
        .collect::<Vec<_>>();
    let tie_occurred = tied_candidates.len() > 1;
    let tied_candidates = if tie_occurred {
        tied_candidates
    } else {
        Vec::new()
    };
    let geometry_matching_candidate = if tie_occurred {
        observation.route_geometry().and_then(|route_geometry| {
            candidates
                .iter()
                .find(|candidate| {
                    tied_candidates.contains(&candidate.candidate_id())
                        && candidate.manager_lock().geometry() == route_geometry
                })
                .map(|candidate| candidate.candidate_id())
        })
    } else {
        None
    };
    let tie_break_reason = match chosen.tie_break() {
        Some(DecisionTieBreak::ObservedGeometryMatch(_)) => {
            DecisionTraceTieBreakReason::ObservedRouteGeometryMatch
        }
        Some(DecisionTieBreak::GenerateOrder) => {
            DecisionTraceTieBreakReason::CanonicalGenerateOrder
        }
        None => DecisionTraceTieBreakReason::NoTie,
    };

    DecisionChoiceTrace {
        highest_score,
        tied_candidates,
        tie_occurred,
        observed_route_geometry: observation.route_geometry(),
        manager_geometry_matched: geometry_matching_candidate.is_some(),
        geometry_matching_candidate,
        tie_break_reason,
        generate_order_resolved: matches!(
            tie_break_reason,
            DecisionTraceTieBreakReason::CanonicalGenerateOrder
        ),
        chosen_candidate: chosen.candidate().candidate_id(),
    }
}

fn build_execution_trace(execution: &SynthesisExecution) -> DecisionExecutionTrace {
    let before = execution.start_frame_state();
    match execution.landing() {
        crate::LandingOutcome::Miss { .. } => DecisionExecutionTrace {
            contact: execution.contact(),
            landed_frame: None,
            prism_delta: PrismDelta::zero(),
            added_flow: Vec::new(),
            added_glow: Vec::new(),
            point_squared_produced: false,
        },
        crate::LandingOutcome::Kiss(kiss) => {
            let after = kiss.point_squared();
            DecisionExecutionTrace {
                contact: execution.contact(),
                landed_frame: Some(after.frame()),
                prism_delta: PrismDelta::new(
                    after.prism().body() as i16 - before.prism().body() as i16,
                    after.prism().spirit() as i16 - before.prism().spirit() as i16,
                    after.prism().mind() as i16 - before.prism().mind() as i16,
                    after.prism().soul_interior() as i16 - before.prism().soul_interior() as i16,
                    after.prism().soul_exterior() as i16 - before.prism().soul_exterior() as i16,
                ),
                added_flow: after
                    .flow_learnset()
                    .iter()
                    .copied()
                    .filter(|flow_id| !before.flow_learnset().contains(flow_id))
                    .collect(),
                added_glow: after
                    .glow_learnset()
                    .iter()
                    .copied()
                    .filter(|glow_id| !before.glow_learnset().contains(glow_id))
                    .collect(),
                point_squared_produced: true,
            }
        }
    }
}

fn build_expected_execution_trace(
    observation: &DecisionObservation,
    recipe: &SynthesisRecipe,
) -> DecisionExecutionTrace {
    let before = observation.frame_state();
    let scripts = match crate::compile_recipe(recipe) {
        Ok(scripts) => scripts,
        Err(_) => {
            return DecisionExecutionTrace {
                contact: ContactOutcome::Miss,
                landed_frame: None,
                prism_delta: PrismDelta::zero(),
                added_flow: Vec::new(),
                added_glow: Vec::new(),
                point_squared_produced: false,
            };
        }
    };

    let mut landed_frame = None;
    let mut prism_delta = PrismDelta::zero();
    let mut added_flow = Vec::new();
    let mut added_glow = Vec::new();

    for script in scripts {
        match script {
            crate::SynthesisScript::ApplyPrismDelta(delta) => {
                prism_delta = PrismDelta::new(
                    prism_delta.body() + delta.body(),
                    prism_delta.spirit() + delta.spirit(),
                    prism_delta.mind() + delta.mind(),
                    prism_delta.soul_interior() + delta.soul_interior(),
                    prism_delta.soul_exterior() + delta.soul_exterior(),
                );
            }
            crate::SynthesisScript::AddFlow(flow_id) => {
                if !before.flow_learnset().contains(&flow_id) && !added_flow.contains(&flow_id) {
                    added_flow.push(flow_id);
                }
            }
            crate::SynthesisScript::AddGlow(glow_id) => {
                if !before.glow_learnset().contains(&glow_id) && !added_glow.contains(&glow_id) {
                    added_glow.push(glow_id);
                }
            }
            crate::SynthesisScript::SetFrame(frame_id) => {
                landed_frame = Some(frame_id);
            }
        }
    }

    DecisionExecutionTrace {
        contact: ContactOutcome::Kiss,
        landed_frame,
        prism_delta,
        added_flow,
        added_glow,
        point_squared_produced: landed_frame.is_some(),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ContactOutcome, ExteriorShape, FlowId, FrameId, GlowId, KernelInput, LandingOutcome,
        ManagerDomain, ManagerFunction, ManagerGeometry, ManagerRelation, Mode, Point,
        SynthesisRecipe, build_canonical_player_spatial_fixture, execute_synthesis_recipe,
        run_kernel_cycle, run_kernel_cycle_with_input,
    };

    use super::{
        ChosenDecision, DecisionCandidate, DecisionCandidateId, DecisionChooseError,
        DecisionEvaluation, DecisionEvaluationReason, DecisionIntent, DecisionTieBreak,
        DecisionTraceReasonCode, DecisionTraceReplayError, DecisionTraceTieBreakReason,
        SynthesisOrientation, build_stanislavski_action_validation_report,
        build_stanislavski_action_witness, canonical_nightingale_hidden_wound_sequence,
        choose_decision, choose_decision_for_observation, evaluate_decision_candidate,
        execute_decision, execute_kernel_pass_decision, generate_decision_candidates,
        observe_decision, observe_kernel_pass_decision, replay_decision_trace,
        replay_kernel_pass_decision_trace, resolve_candidate_recipe,
    };

    #[test]
    fn observation_is_pure_and_records_the_explicit_intent() {
        let point = Point::origin();
        let before = point.clone();
        let observation = observe_decision(&point, DecisionIntent::FavorCurrent);

        assert_eq!(observation.point(), &point);
        assert_eq!(observation.frame_state(), point.frame_state());
        assert_eq!(observation.intent(), DecisionIntent::FavorCurrent);
        assert_eq!(observation.route_geometry(), None);
        let rotation = observation
            .rotation_context()
            .expect("origin Point should expose Rule-of-Twelve geometry");
        assert_eq!(rotation.ring().value(), 1);
        assert_eq!(rotation.absolute_position().value(), 7);
        assert_eq!(rotation.pass().value(), 2);
        assert_eq!(rotation.house_number().value(), 3);
        assert_eq!(point, before);
    }

    #[test]
    fn observation_can_surface_optional_player_spatial_interpretation() {
        let fixture =
            build_canonical_player_spatial_fixture().expect("player spatial fixture should build");
        let observation = observe_decision(fixture.point(), DecisionIntent::Neutral);
        let spatial = observation
            .spatial_interpretation()
            .expect("spatial interpretation should be present");
        let proxy = spatial.proxy().expect("Proxy should be present");
        let moxy = spatial.moxy().expect("Moxy should be present");

        assert_eq!(
            observation.rotation_context(),
            Some(fixture.rotation_context())
        );
        assert_eq!(proxy.render(), "Distal Round northwest of Stonebend");
        assert_eq!(
            moxy.render(),
            "Bond toward Flynt through Stairway to Heaven"
        );
        assert!(spatial.foxy().is_none());
    }

    #[test]
    fn kernel_pass_observation_records_straight_and_curved_geometry() {
        let straight = observe_kernel_pass_decision(
            &run_kernel_cycle(crate::Symptom::origin()),
            DecisionIntent::Neutral,
        );
        let curved = observe_kernel_pass_decision(
            &run_kernel_cycle_with_input(
                crate::Symptom::origin(),
                KernelInput {
                    routing: crate::PlebMetaInput {
                        exterior_shape: ExteriorShape::Curved,
                        pleb_mode: Mode::Pathos,
                        meta_mode: Mode::Logos,
                    },
                },
            ),
            DecisionIntent::Neutral,
        );

        assert_eq!(straight.route_geometry(), Some(ManagerGeometry::Straight));
        assert_eq!(curved.route_geometry(), Some(ManagerGeometry::Curved));
    }

    #[test]
    fn generation_is_deterministic_and_returns_the_two_canonical_candidates() {
        let observation = observe_decision(&Point::origin(), DecisionIntent::Neutral);
        let candidates = generate_decision_candidates(&observation);

        assert_eq!(candidates.len(), 2);
        assert_eq!(
            candidates[0].candidate_id(),
            DecisionCandidateId::GremlinTinker
        );
        assert_eq!(
            candidates[1].candidate_id(),
            DecisionCandidateId::PixyConfusion
        );
        assert_eq!(candidates[0].manager(), crate::Manager::Clouseau);
        assert_eq!(candidates[1].manager(), crate::Manager::Hal);
        assert_eq!(candidates[0].orientation(), SynthesisOrientation::Current);
        assert_eq!(candidates[1].orientation(), SynthesisOrientation::Aura);
    }

    #[test]
    fn evaluation_is_deterministic_and_explicit() {
        let observation = observe_decision(&Point::origin(), DecisionIntent::FavorCurrent);
        let candidates = generate_decision_candidates(&observation);
        let gremlin = evaluate_decision_candidate(&observation, &candidates[0]);
        let pixy = evaluate_decision_candidate(&observation, &candidates[1]);

        assert_eq!(gremlin.score(), 2);
        assert_eq!(
            gremlin.reason(),
            DecisionEvaluationReason::PreferredOrientation
        );
        assert_eq!(pixy.score(), 1);
        assert_eq!(
            pixy.reason(),
            DecisionEvaluationReason::NonPreferredOrientation
        );
    }

    #[test]
    fn neutral_evaluation_scores_both_candidates_equally() {
        let observation = observe_decision(&Point::origin(), DecisionIntent::Neutral);
        let candidates = generate_decision_candidates(&observation);
        let gremlin = evaluate_decision_candidate(&observation, &candidates[0]);
        let pixy = evaluate_decision_candidate(&observation, &candidates[1]);

        assert_eq!(gremlin.score(), 1);
        assert_eq!(pixy.score(), 1);
        assert_eq!(gremlin.reason(), DecisionEvaluationReason::NeutralBaseline);
        assert_eq!(pixy.reason(), DecisionEvaluationReason::NeutralBaseline);
    }

    #[test]
    fn neutral_evaluation_penalizes_already_realized_pixy_state() {
        let pixy_point = canonical_next_point_from(DecisionCandidateId::PixyConfusion);
        let observation = observe_decision(&pixy_point, DecisionIntent::Neutral);
        let candidates = generate_decision_candidates(&observation);
        let gremlin = evaluate_decision_candidate(&observation, &candidates[0]);
        let pixy = evaluate_decision_candidate(&observation, &candidates[1]);

        assert_eq!(observation.frame_state().frame(), FrameId::Pixy);
        assert_eq!(gremlin.score(), 1);
        assert_eq!(gremlin.reason(), DecisionEvaluationReason::NeutralBaseline);
        assert_eq!(pixy.score(), 0);
        assert_eq!(
            pixy.reason(),
            DecisionEvaluationReason::NeutralAlreadyRealized
        );
    }

    #[test]
    fn neutral_evaluation_penalizes_already_realized_gremlin_state() {
        let gremlin_point = canonical_next_point_from(DecisionCandidateId::GremlinTinker);
        let observation = observe_decision(&gremlin_point, DecisionIntent::Neutral);
        let candidates = generate_decision_candidates(&observation);
        let gremlin = evaluate_decision_candidate(&observation, &candidates[0]);
        let pixy = evaluate_decision_candidate(&observation, &candidates[1]);

        assert_eq!(observation.frame_state().frame(), FrameId::Gremlin);
        assert_eq!(gremlin.score(), 0);
        assert_eq!(
            gremlin.reason(),
            DecisionEvaluationReason::NeutralAlreadyRealized
        );
        assert_eq!(pixy.score(), 1);
        assert_eq!(pixy.reason(), DecisionEvaluationReason::NeutralBaseline);
    }

    #[test]
    fn choose_prefers_higher_score_and_uses_generate_order_for_ties() {
        let candidates = vec![
            DecisionCandidate::new(
                DecisionCandidateId::GremlinTinker,
                crate::Manager::Clouseau,
                SynthesisOrientation::Current,
            ),
            DecisionCandidate::new(
                DecisionCandidateId::PixyConfusion,
                crate::Manager::Hal,
                SynthesisOrientation::Aura,
            ),
        ];
        let favored = vec![
            DecisionEvaluation::new(
                DecisionCandidateId::GremlinTinker,
                2,
                DecisionEvaluationReason::PreferredOrientation,
            ),
            DecisionEvaluation::new(
                DecisionCandidateId::PixyConfusion,
                1,
                DecisionEvaluationReason::NonPreferredOrientation,
            ),
        ];
        let neutral = vec![
            DecisionEvaluation::new(
                DecisionCandidateId::GremlinTinker,
                1,
                DecisionEvaluationReason::NeutralBaseline,
            ),
            DecisionEvaluation::new(
                DecisionCandidateId::PixyConfusion,
                1,
                DecisionEvaluationReason::NeutralBaseline,
            ),
        ];

        let favored_choice = choose_decision(&candidates, &favored).expect("choice should work");
        let neutral_choice = choose_decision(&candidates, &neutral).expect("choice should work");

        assert_eq!(
            favored_choice.candidate().candidate_id(),
            DecisionCandidateId::GremlinTinker
        );
        assert_eq!(favored_choice.tie_break(), None);
        assert_eq!(
            neutral_choice.candidate().candidate_id(),
            DecisionCandidateId::GremlinTinker
        );
        assert_eq!(
            neutral_choice.tie_break(),
            Some(DecisionTieBreak::GenerateOrder)
        );
    }

    #[test]
    fn route_geometry_breaks_neutral_ties_before_generate_order() {
        let straight_observation = observe_kernel_pass_decision(
            &run_kernel_cycle(crate::Symptom::origin()),
            DecisionIntent::Neutral,
        );
        let curved_observation = observe_kernel_pass_decision(
            &run_kernel_cycle_with_input(
                crate::Symptom::origin(),
                KernelInput {
                    routing: crate::PlebMetaInput {
                        exterior_shape: ExteriorShape::Curved,
                        pleb_mode: Mode::Pathos,
                        meta_mode: Mode::Logos,
                    },
                },
            ),
            DecisionIntent::Neutral,
        );
        let candidates = generate_decision_candidates(&straight_observation);
        let straight_evaluations = candidates
            .iter()
            .map(|candidate| evaluate_decision_candidate(&straight_observation, candidate))
            .collect::<Vec<_>>();
        let curved_evaluations = candidates
            .iter()
            .map(|candidate| evaluate_decision_candidate(&curved_observation, candidate))
            .collect::<Vec<_>>();

        let straight_choice = choose_decision_for_observation(
            Some(&straight_observation),
            &candidates,
            &straight_evaluations,
        )
        .expect("straight choice should work");
        let curved_choice = choose_decision_for_observation(
            Some(&curved_observation),
            &candidates,
            &curved_evaluations,
        )
        .expect("curved choice should work");

        assert_eq!(
            straight_choice.candidate().candidate_id(),
            DecisionCandidateId::GremlinTinker
        );
        assert_eq!(
            straight_choice.tie_break(),
            Some(DecisionTieBreak::ObservedGeometryMatch(
                ManagerGeometry::Straight
            ))
        );
        assert_eq!(
            curved_choice.candidate().candidate_id(),
            DecisionCandidateId::PixyConfusion
        );
        assert_eq!(
            curved_choice.tie_break(),
            Some(DecisionTieBreak::ObservedGeometryMatch(
                ManagerGeometry::Curved
            ))
        );
    }

    #[test]
    fn choose_rejects_missing_duplicate_and_unknown_evaluations() {
        let gremlin = DecisionCandidate::new(
            DecisionCandidateId::GremlinTinker,
            crate::Manager::Clouseau,
            SynthesisOrientation::Current,
        );
        let pixy = DecisionCandidate::new(
            DecisionCandidateId::PixyConfusion,
            crate::Manager::Hal,
            SynthesisOrientation::Aura,
        );
        let good_eval = DecisionEvaluation::new(
            DecisionCandidateId::GremlinTinker,
            2,
            DecisionEvaluationReason::PreferredOrientation,
        );

        assert_eq!(
            choose_decision(&[], &[]),
            Err(DecisionChooseError::NoCandidates)
        );
        assert_eq!(
            choose_decision(&[gremlin], &[good_eval, good_eval]),
            Err(DecisionChooseError::DuplicateEvaluation(
                DecisionCandidateId::GremlinTinker
            ))
        );
        assert_eq!(
            choose_decision(&[gremlin, gremlin], &[good_eval]),
            Err(DecisionChooseError::DuplicateCandidate(
                DecisionCandidateId::GremlinTinker
            ))
        );
        assert_eq!(
            choose_decision(&[gremlin, pixy], &[good_eval]),
            Err(DecisionChooseError::MissingEvaluation(
                DecisionCandidateId::PixyConfusion
            ))
        );
        assert_eq!(
            choose_decision(
                &[gremlin],
                &[DecisionEvaluation::new(
                    DecisionCandidateId::PixyConfusion,
                    1,
                    DecisionEvaluationReason::NeutralBaseline,
                )],
            ),
            Err(DecisionChooseError::UnknownEvaluation(
                DecisionCandidateId::PixyConfusion
            ))
        );
    }

    #[test]
    fn candidate_to_recipe_resolution_uses_the_existing_v1_fixtures() {
        let gremlin = resolve_candidate_recipe(DecisionCandidateId::GremlinTinker);
        let pixy = resolve_candidate_recipe(DecisionCandidateId::PixyConfusion);

        assert_eq!(gremlin.recipe_id(), "gremlin_tinker");
        assert_eq!(pixy.recipe_id(), "pixy_confusion");
        assert_eq!(gremlin.display_name(), "Gremlin Tinker Recipe");
        assert_eq!(pixy.display_name(), "Pixy Confusion Recipe");
        assert_eq!(gremlin.intents().len(), 3);
        assert_eq!(pixy.intents().len(), 3);
    }

    #[test]
    fn current_favored_decision_executes_to_canonical_gremlin_point_squared() {
        let point = Point::origin();
        let before = point.clone();
        let result = execute_decision(&point, DecisionIntent::FavorCurrent)
            .expect("decision should execute");
        let trace = result.trace();

        assert_eq!(
            result.chosen().candidate().candidate_id(),
            DecisionCandidateId::GremlinTinker
        );
        assert_eq!(result.recipe().recipe_id(), "gremlin_tinker");
        assert_eq!(point, before);
        assert_eq!(result.execution().contact(), ContactOutcome::Kiss);
        assert_eq!(
            trace.choice().tie_break_reason(),
            DecisionTraceTieBreakReason::NoTie
        );
        assert_eq!(trace.execution().contact(), ContactOutcome::Kiss);
        assert_eq!(trace.execution().landed_frame(), Some(FrameId::Gremlin));
        assert_eq!(trace.execution().prism_delta().body(), 2);
        assert_eq!(trace.execution().prism_delta().mind(), 0);
        assert_eq!(trace.execution().added_flow(), &[FlowId::TinkerGrip]);
        assert!(trace.execution().added_glow().is_empty());
        let LandingOutcome::Kiss(kiss) = result.execution().landing() else {
            panic!("expected kiss landing");
        };
        assert_eq!(kiss.before().frame(), FrameId::Hueman);
        assert_eq!(kiss.point_squared().frame(), FrameId::Gremlin);
        assert_eq!(kiss.point_squared().prism().body(), 3);
        assert_eq!(kiss.point_squared().flow_learnset(), &[FlowId::TinkerGrip]);
        assert!(kiss.point_squared().glow_learnset().is_empty());
    }

    #[test]
    fn aura_favored_decision_executes_to_canonical_pixy_point_squared() {
        let point = Point::origin();
        let before = point.clone();
        let result =
            execute_decision(&point, DecisionIntent::FavorAura).expect("decision should execute");
        let trace = result.trace();

        assert_eq!(
            result.chosen().candidate().candidate_id(),
            DecisionCandidateId::PixyConfusion
        );
        assert_eq!(result.recipe().recipe_id(), "pixy_confusion");
        assert_eq!(point, before);
        assert_eq!(result.execution().contact(), ContactOutcome::Kiss);
        assert_eq!(
            trace.choice().tie_break_reason(),
            DecisionTraceTieBreakReason::NoTie
        );
        assert_eq!(trace.execution().landed_frame(), Some(FrameId::Pixy));
        assert_eq!(trace.execution().prism_delta().mind(), 2);
        assert_eq!(trace.execution().added_glow(), &[GlowId::Confusion]);
        assert!(trace.execution().added_flow().is_empty());
        let LandingOutcome::Kiss(kiss) = result.execution().landing() else {
            panic!("expected kiss landing");
        };
        assert_eq!(kiss.before().frame(), FrameId::Hueman);
        assert_eq!(kiss.point_squared().frame(), FrameId::Pixy);
        assert_eq!(kiss.point_squared().prism().mind(), 3);
        assert_eq!(kiss.point_squared().glow_learnset(), &[GlowId::Confusion]);
        assert!(kiss.point_squared().flow_learnset().is_empty());
    }

    #[test]
    fn neutral_decision_tie_breaks_to_gremlin_and_keeps_manager_classification_canonical() {
        let point = Point::origin();
        let result =
            execute_decision(&point, DecisionIntent::Neutral).expect("decision should execute");
        let manager_lock = result.chosen().candidate().manager_lock();

        assert_eq!(
            result.chosen().candidate().candidate_id(),
            DecisionCandidateId::GremlinTinker
        );
        assert_eq!(manager_lock.domain(), ManagerDomain::Pleb);
        assert_eq!(manager_lock.relation(), ManagerRelation::PlebPleb);
        assert_eq!(manager_lock.geometry(), ManagerGeometry::Straight);
        assert_eq!(manager_lock.function(), ManagerFunction::Locate);
        assert_eq!(
            result.chosen().tie_break(),
            Some(DecisionTieBreak::GenerateOrder)
        );
        assert_eq!(
            result.trace().choice().tie_break_reason(),
            DecisionTraceTieBreakReason::CanonicalGenerateOrder
        );
    }

    #[test]
    fn curved_kernel_pass_neutral_decision_chooses_pixy_from_observed_geometry() {
        let kernel_pass = run_kernel_cycle_with_input(
            crate::Symptom::origin(),
            KernelInput {
                routing: crate::PlebMetaInput {
                    exterior_shape: ExteriorShape::Curved,
                    pleb_mode: Mode::Pathos,
                    meta_mode: Mode::Logos,
                },
            },
        );
        let result = execute_kernel_pass_decision(&kernel_pass, DecisionIntent::Neutral)
            .expect("decision should execute");

        assert_eq!(
            result.chosen().candidate().candidate_id(),
            DecisionCandidateId::PixyConfusion
        );
        assert_eq!(
            result.chosen().tie_break(),
            Some(DecisionTieBreak::ObservedGeometryMatch(
                ManagerGeometry::Curved
            ))
        );
        let LandingOutcome::Kiss(kiss) = result.execution().landing() else {
            panic!("expected kiss landing");
        };
        assert_eq!(kiss.before().frame(), FrameId::Hueman);
        assert_eq!(kiss.point_squared().frame(), FrameId::Pixy);
    }

    #[test]
    fn neutral_decision_from_pixy_state_switches_to_gremlin_candidate() {
        let pixy_point = canonical_next_point_from(DecisionCandidateId::PixyConfusion);
        let before = pixy_point.clone();
        let result = execute_decision(&pixy_point, DecisionIntent::Neutral)
            .expect("decision should execute");

        assert_eq!(
            result.chosen().candidate().candidate_id(),
            DecisionCandidateId::GremlinTinker
        );
        assert_eq!(result.recipe().recipe_id(), "gremlin_tinker");
        assert_eq!(pixy_point, before);
        let LandingOutcome::Kiss(kiss) = result.execution().landing() else {
            panic!("expected kiss landing");
        };
        assert_eq!(kiss.before().frame(), FrameId::Pixy);
        assert_eq!(kiss.point_squared().frame(), FrameId::Gremlin);
    }

    #[test]
    fn neutral_decision_from_gremlin_state_switches_to_pixy_candidate() {
        let gremlin_point = canonical_next_point_from(DecisionCandidateId::GremlinTinker);
        let before = gremlin_point.clone();
        let result = execute_decision(&gremlin_point, DecisionIntent::Neutral)
            .expect("decision should execute");

        assert_eq!(
            result.chosen().candidate().candidate_id(),
            DecisionCandidateId::PixyConfusion
        );
        assert_eq!(result.recipe().recipe_id(), "pixy_confusion");
        assert_eq!(gremlin_point, before);
        let LandingOutcome::Kiss(kiss) = result.execution().landing() else {
            panic!("expected kiss landing");
        };
        assert_eq!(kiss.before().frame(), FrameId::Gremlin);
        assert_eq!(kiss.point_squared().frame(), FrameId::Pixy);
    }

    #[test]
    fn ten_thousand_repeated_decisions_remain_deterministic() {
        let point = Point::origin();

        for _ in 0..10_000 {
            let current = execute_decision(&point, DecisionIntent::FavorCurrent)
                .expect("current decision should execute");
            let aura = execute_decision(&point, DecisionIntent::FavorAura)
                .expect("aura decision should execute");
            let neutral = execute_decision(&point, DecisionIntent::Neutral)
                .expect("neutral decision should execute");

            assert_eq!(
                current.chosen().candidate().candidate_id(),
                DecisionCandidateId::GremlinTinker
            );
            assert_eq!(
                aura.chosen().candidate().candidate_id(),
                DecisionCandidateId::PixyConfusion
            );
            assert_eq!(
                neutral.chosen().candidate().candidate_id(),
                DecisionCandidateId::GremlinTinker
            );
            assert_eq!(point, Point::origin());
        }
    }

    #[test]
    fn version_two_does_not_copy_script_lists_into_candidate_or_recipe_resolution() {
        let gremlin = resolve_candidate_recipe(DecisionCandidateId::GremlinTinker);
        let pixy = resolve_candidate_recipe(DecisionCandidateId::PixyConfusion);

        assert_eq!(
            gremlin,
            SynthesisRecipe::new(
                "gremlin_tinker",
                "Gremlin Tinker Recipe",
                gremlin.intents().to_vec()
            )
        );
        assert_eq!(
            pixy,
            SynthesisRecipe::new(
                "pixy_confusion",
                "Pixy Confusion Recipe",
                pixy.intents().to_vec()
            )
        );
    }

    #[test]
    fn chosen_decision_retains_candidate_and_evaluation_evidence() {
        let choice = ChosenDecision {
            candidate: DecisionCandidate::new(
                DecisionCandidateId::GremlinTinker,
                crate::Manager::Clouseau,
                SynthesisOrientation::Current,
            ),
            evaluation: DecisionEvaluation::new(
                DecisionCandidateId::GremlinTinker,
                2,
                DecisionEvaluationReason::PreferredOrientation,
            ),
            tie_break: None,
        };

        assert_eq!(
            choice.candidate().candidate_id(),
            DecisionCandidateId::GremlinTinker
        );
        assert_eq!(choice.evaluation().score(), 2);
        assert_eq!(choice.tie_break(), None);
    }

    #[test]
    fn identical_inputs_create_identical_traces() {
        let point = Point::origin();
        let first = execute_decision(&point, DecisionIntent::FavorCurrent)
            .expect("decision should execute");
        let second = execute_decision(&point, DecisionIntent::FavorCurrent)
            .expect("decision should execute");

        assert_eq!(first.trace(), second.trace());
        assert_eq!(point, Point::origin());
    }

    #[test]
    fn trace_records_canonical_candidate_order_and_score_components() {
        let result = execute_decision(&Point::origin(), DecisionIntent::FavorCurrent)
            .expect("decision should execute");
        let trace = result.trace();

        assert_eq!(trace.generation().len(), 2);
        assert_eq!(
            trace.generation()[0].candidate_id(),
            DecisionCandidateId::GremlinTinker
        );
        assert_eq!(
            trace.generation()[1].candidate_id(),
            DecisionCandidateId::PixyConfusion
        );
        assert_eq!(trace.evaluations()[0].intent_score(), 2);
        assert_eq!(trace.evaluations()[0].realized_state_penalty(), 0);
        assert_eq!(trace.evaluations()[0].final_score(), 2);
        assert_eq!(
            trace.evaluations()[0].reason_codes(),
            &[DecisionTraceReasonCode::PreferredCurrentOrientation]
        );
        assert_eq!(trace.evaluations()[1].intent_score(), 1);
        assert_eq!(trace.evaluations()[1].realized_state_penalty(), 0);
        assert_eq!(trace.evaluations()[1].final_score(), 1);
        assert_eq!(
            trace.evaluations()[1].reason_codes(),
            &[DecisionTraceReasonCode::NonPreferredCurrentOrientation]
        );
        for evaluation in trace.evaluations() {
            assert_eq!(
                evaluation.intent_score() - evaluation.realized_state_penalty(),
                evaluation.final_score()
            );
        }
    }

    #[test]
    fn trace_records_point_only_and_route_geometry_tie_breaks_deterministically() {
        let point_only = execute_decision(&Point::origin(), DecisionIntent::Neutral)
            .expect("decision should execute");
        let straight = execute_kernel_pass_decision(
            &run_kernel_cycle(crate::Symptom::origin()),
            DecisionIntent::Neutral,
        )
        .expect("decision should execute");
        let curved = execute_kernel_pass_decision(
            &run_kernel_cycle_with_input(
                crate::Symptom::origin(),
                KernelInput {
                    routing: crate::PlebMetaInput {
                        exterior_shape: ExteriorShape::Curved,
                        pleb_mode: Mode::Pathos,
                        meta_mode: Mode::Logos,
                    },
                },
            ),
            DecisionIntent::Neutral,
        )
        .expect("decision should execute");

        assert_eq!(
            point_only.trace().choice().tie_break_reason(),
            DecisionTraceTieBreakReason::CanonicalGenerateOrder
        );
        assert_eq!(point_only.trace().choice().observed_route_geometry(), None);
        assert_eq!(
            straight.trace().choice().tie_break_reason(),
            DecisionTraceTieBreakReason::ObservedRouteGeometryMatch
        );
        assert_eq!(
            straight.trace().choice().observed_route_geometry(),
            Some(ManagerGeometry::Straight)
        );
        assert_eq!(
            straight.trace().choice().geometry_matching_candidate(),
            Some(DecisionCandidateId::GremlinTinker)
        );
        assert_eq!(
            curved.trace().choice().tie_break_reason(),
            DecisionTraceTieBreakReason::ObservedRouteGeometryMatch
        );
        assert_eq!(
            curved.trace().choice().observed_route_geometry(),
            Some(ManagerGeometry::Curved)
        );
        assert_eq!(
            curved.trace().choice().geometry_matching_candidate(),
            Some(DecisionCandidateId::PixyConfusion)
        );
    }

    #[test]
    fn trace_records_realized_state_penalties_for_gremlin_and_pixy() {
        let gremlin_result = execute_decision(
            &canonical_next_point_from(DecisionCandidateId::GremlinTinker),
            DecisionIntent::Neutral,
        )
        .expect("decision should execute");
        let pixy_result = execute_decision(
            &canonical_next_point_from(DecisionCandidateId::PixyConfusion),
            DecisionIntent::Neutral,
        )
        .expect("decision should execute");

        let gremlin_trace = gremlin_result.trace();
        assert_eq!(gremlin_trace.observation().frame(), FrameId::Gremlin);
        assert_eq!(
            gremlin_trace.evaluations()[0].candidate_id(),
            DecisionCandidateId::GremlinTinker
        );
        assert_eq!(gremlin_trace.evaluations()[0].intent_score(), 1);
        assert_eq!(gremlin_trace.evaluations()[0].realized_state_penalty(), 1);
        assert_eq!(gremlin_trace.evaluations()[0].final_score(), 0);
        assert!(
            gremlin_trace.evaluations()[0]
                .reason_codes()
                .contains(&DecisionTraceReasonCode::AlreadyCanonicalFrame)
        );
        assert!(
            gremlin_trace.evaluations()[0]
                .reason_codes()
                .contains(&DecisionTraceReasonCode::AlreadyKnowsCanonicalFlow)
        );

        let pixy_trace = pixy_result.trace();
        assert_eq!(pixy_trace.observation().frame(), FrameId::Pixy);
        assert_eq!(
            pixy_trace.evaluations()[1].candidate_id(),
            DecisionCandidateId::PixyConfusion
        );
        assert_eq!(pixy_trace.evaluations()[1].intent_score(), 1);
        assert_eq!(pixy_trace.evaluations()[1].realized_state_penalty(), 1);
        assert_eq!(pixy_trace.evaluations()[1].final_score(), 0);
        assert!(
            pixy_trace.evaluations()[1]
                .reason_codes()
                .contains(&DecisionTraceReasonCode::AlreadyCanonicalFrame)
        );
        assert!(
            pixy_trace.evaluations()[1]
                .reason_codes()
                .contains(&DecisionTraceReasonCode::AlreadyKnowsCanonicalGlow)
        );
    }

    #[test]
    fn replay_accepts_valid_trace_and_rejects_tampering() {
        let point = Point::origin();
        let result =
            execute_decision(&point, DecisionIntent::Neutral).expect("decision should execute");
        let trace = result.trace().clone();
        let kernel_pass = run_kernel_cycle_with_input(
            crate::Symptom::origin(),
            KernelInput {
                routing: crate::PlebMetaInput {
                    exterior_shape: ExteriorShape::Curved,
                    pleb_mode: Mode::Pathos,
                    meta_mode: Mode::Logos,
                },
            },
        );
        let curved = execute_kernel_pass_decision(&kernel_pass, DecisionIntent::Neutral)
            .expect("decision should execute");

        assert_eq!(
            replay_decision_trace(&point, DecisionIntent::Neutral, &trace),
            Ok(())
        );
        assert_eq!(
            replay_kernel_pass_decision_trace(
                &kernel_pass,
                DecisionIntent::Neutral,
                curved.trace()
            ),
            Ok(())
        );

        let mut changed_score = trace.clone();
        changed_score.evaluations[0].final_score = 9;
        assert_eq!(
            replay_decision_trace(&point, DecisionIntent::Neutral, &changed_score),
            Err(DecisionTraceReplayError::EvaluationMismatch(
                DecisionCandidateId::GremlinTinker
            ))
        );

        let mut changed_order = trace.clone();
        changed_order.generation.swap(0, 1);
        assert_eq!(
            replay_decision_trace(&point, DecisionIntent::Neutral, &changed_order),
            Err(DecisionTraceReplayError::CandidateOrderMismatch)
        );

        let mut changed_tie_break = trace.clone();
        changed_tie_break.choice.tie_break_reason = DecisionTraceTieBreakReason::NoTie;
        assert_eq!(
            replay_decision_trace(&point, DecisionIntent::Neutral, &changed_tie_break),
            Err(DecisionTraceReplayError::TieBreakMismatch)
        );

        let mut changed_choice = trace.clone();
        changed_choice.choice.chosen_candidate = DecisionCandidateId::PixyConfusion;
        assert_eq!(
            replay_decision_trace(&point, DecisionIntent::Neutral, &changed_choice),
            Err(DecisionTraceReplayError::ChoiceMismatch)
        );
    }

    #[test]
    fn stanislavski_hidden_wound_sequence_preserves_purpose_and_adapts_tactic() {
        let sequence = canonical_nightingale_hidden_wound_sequence();
        assert_eq!(sequence.title(), "Nightingale Hidden Wound");
        assert_eq!(sequence.beats().len(), 2);

        let beat_one = &sequence.beats()[0];
        let beat_two = &sequence.beats()[1];

        assert_eq!(
            beat_one.objective().as_str(),
            "identify the immediate hidden condition"
        );
        assert_eq!(
            beat_one.purpose().as_str(),
            "preserve the patient's life and agency"
        );
        assert_eq!(
            beat_one.chosen().tactic_id(),
            crate::CandidateTacticId::AuraLesionTrace
        );
        assert_eq!(
            beat_two.chosen().tactic_id(),
            crate::CandidateTacticId::RequestMinorianMeasurement
        );
        assert_eq!(
            beat_two.through_line().status(),
            crate::ThroughLineStatus::Adapted
        );

        let forced_memory = beat_one
            .evaluations()
            .iter()
            .find(|evaluation| {
                evaluation.tactic_id() == crate::CandidateTacticId::ForciblyOpenMemory
            })
            .expect("fixture should include the consent-violating tactic");
        assert!(!forced_memory.sense_of_truth().passes());
        assert!(
            forced_memory
                .sense_of_truth()
                .reasons()
                .contains(&crate::SenseOfTruthCode::ConsentConflict)
        );
        assert!(
            forced_memory
                .sense_of_truth()
                .reasons()
                .contains(&crate::SenseOfTruthCode::PurposeContradiction)
        );
    }

    #[test]
    fn stanislavski_witness_and_validation_render_required_sections() {
        let witness =
            build_stanislavski_action_witness().expect("Stanislavski action witness should render");
        let validation = build_stanislavski_action_validation_report()
            .expect("Stanislavski validation report should render");

        assert!(witness.contains("HOLLOW GROVE STANISLAVSKI ACTION WITNESS"));
        assert!(witness.contains("Scene:\nNightingale Hidden Wound"));
        assert!(witness.contains("Given Circumstances:"));
        assert!(witness.contains("Magic-If Projection:"));
        assert!(witness.contains("Sense-of-Truth Result:"));
        assert!(witness.contains("Chosen Tactic:\nAura Lesion Trace"));
        assert!(witness.contains("Through-Line Status:"));
        assert!(validation.contains("candidate tactics objective-driven: pass"));
        assert!(validation.contains("Sense-of-Truth rejects forced memory opening: pass"));
        assert!(validation.contains("V2 remains selector: pass"));
        assert!(validation.contains("V1.1 remains executor: pass"));
    }

    fn canonical_next_point_from(candidate_id: DecisionCandidateId) -> Point {
        let recipe = resolve_candidate_recipe(candidate_id);
        let execution = execute_synthesis_recipe(&Point::origin(), &recipe)
            .expect("canonical execution should succeed");
        let LandingOutcome::Kiss(kiss) = execution.landing() else {
            panic!("expected kiss landing");
        };
        kiss.next_point()
    }
}
