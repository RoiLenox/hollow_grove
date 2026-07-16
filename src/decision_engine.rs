use crate::synthesis_execution::{
    SynthesisExecution, SynthesisExecutionError, execute_synthesis_recipe,
};
use crate::{
    ContactOutcome, ExteriorShape, FlowId, FrameId, GlowId, KernelPass, Manager, ManagerGeometry,
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
        SynthesisOrientation, choose_decision, choose_decision_for_observation,
        evaluate_decision_candidate, execute_decision, execute_kernel_pass_decision,
        generate_decision_candidates, observe_decision, observe_kernel_pass_decision,
        replay_decision_trace, replay_kernel_pass_decision_trace, resolve_candidate_recipe,
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
