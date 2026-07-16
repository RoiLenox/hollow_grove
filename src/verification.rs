use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::fs;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::str::FromStr;
use std::time::Duration;
use std::time::Instant;

use crate::aim::{gremlin_tinker_aim, pixy_confusion_aim};
use crate::build_snapshot_boundary_output;
use crate::decision_engine::{
    DecisionChoiceTrace, DecisionEvaluationTrace, DecisionExecution,
    DecisionGeneratedCandidateTrace, DecisionObservation, DecisionRecipeBridgeTrace, DecisionTrace,
    DecisionTraceReasonCode, DecisionTraceTieBreakReason, build_decision_plan,
    build_decision_trace_without_execution, execute_observed_decision, observe_decision,
    observe_decision_with_geometry, replay_trace_for_observation,
};
use crate::kernel_pass_output::build_snapshot_output;
use crate::landing::{LandingFaultCut, LandingFaultError, apply_kiss_with_fault, land_contact};
use crate::{
    ContactOutcome, CurrentPrism, DecisionCandidateId, DecisionIntent, ExteriorShape, FlowId,
    FrameId, FrameState, GlowId, KernelInput, Manager, ManagerGeometry, Point, PrismDelta,
    SnapshotBoundary, SynthesisOrientation, SynthesisRecipe, SynthesisScript, compile_recipe,
    execute_synthesis_recipe, generate_decision_candidates, manager_domain_lock,
    observe_kernel_pass_decision, resolve_candidate_recipe, run_kernel_cycle_with_input,
};

const FRAMES: [FrameId; 19] = [
    FrameId::Hueman,
    FrameId::Gremlin,
    FrameId::Goblin,
    FrameId::Ghoul,
    FrameId::Troll,
    FrameId::Ork,
    FrameId::Ogre,
    FrameId::Troglodyte,
    FrameId::Pixy,
    FrameId::Sprite,
    FrameId::Faerie,
    FrameId::Nymph,
    FrameId::Siren,
    FrameId::Muse,
    FrameId::Werewolf,
    FrameId::Gargoyle,
    FrameId::Merman,
    FrameId::Chimera,
    FrameId::Manticore,
];
const FLOWS: [FlowId; 7] = [
    FlowId::TinkerGrip,
    FlowId::Stonefold,
    FlowId::PressureRelocation,
    FlowId::PackRelay,
    FlowId::Moonrush,
    FlowId::MeteorDrop,
    FlowId::RiptideSwim,
];
const GLOWS: [GlowId; 6] = [
    GlowId::Confusion,
    GlowId::Projection,
    GlowId::Recognition,
    GlowId::SpriteCall,
    GlowId::FaerieVeil,
    GlowId::MuseChorus,
];
const INTENTS: [DecisionIntent; 3] = [
    DecisionIntent::FavorCurrent,
    DecisionIntent::FavorAura,
    DecisionIntent::Neutral,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationProfile {
    Fast,
    Full,
    Overnight,
}

impl VerificationProfile {
    pub const fn property_cases(self) -> u64 {
        match self {
            Self::Fast => 2_048,
            Self::Full => 100_000,
            Self::Overnight => 100_000,
        }
    }

    pub const fn differential_cases(self) -> u64 {
        match self {
            Self::Fast => 10_000,
            Self::Full => 1_000_000,
            Self::Overnight => 1_000_000,
        }
    }

    pub const fn benchmark_iterations(self) -> usize {
        match self {
            Self::Fast => 512,
            Self::Full => 4_096,
            Self::Overnight => 4_096,
        }
    }

    pub const fn synthetic_include_extreme(self) -> bool {
        matches!(self, Self::Overnight)
    }

    pub const fn semantic_hash_cases(self) -> u64 {
        match self {
            Self::Fast => 10_000,
            Self::Full => 100_000,
            Self::Overnight => 100_000,
        }
    }
}

impl FromStr for VerificationProfile {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "fast" | "FAST" => Ok(Self::Fast),
            "full" | "FULL" => Ok(Self::Full),
            "overnight" | "OVERNIGHT" => Ok(Self::Overnight),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct VerificationReport {
    pub profile: VerificationProfile,
    pub seed: u64,
    pub exhaustive: ExhaustiveSummary,
    pub property: PropertySummary,
    pub metamorphic: MetamorphicSummary,
    pub differential: DifferentialSummary,
    pub rollback: RollbackSummary,
    pub trace_corruption: TraceCorruptionSummary,
    pub semantic_hash: SemanticHashSummary,
    pub benchmarks: BenchmarkSummary,
    pub synthetic_scale: Vec<SyntheticScaleStats>,
    pub skipped: Vec<SkippedStage>,
}

#[derive(Debug, Clone)]
pub struct SkippedStage {
    pub stage: &'static str,
    pub reason: String,
}

#[derive(Debug, Clone)]
pub struct ExhaustiveSummary {
    pub rows: Vec<ExhaustiveMatrixRow>,
    pub total_enumerated_states: usize,
    pub total_legal_states: usize,
    pub total_rejected_states: usize,
    pub unique_decision_outcomes: usize,
    pub unique_trace_shapes: usize,
    pub unexplained_states: usize,
    pub nondeterministic_states: usize,
    pub invariant_failures: usize,
    pub equivalence_classes: BTreeMap<String, usize>,
}

#[derive(Debug, Clone)]
pub struct ExhaustiveMatrixRow {
    pub observation_identity: String,
    pub frame: String,
    pub flows: String,
    pub glows: String,
    pub intent: String,
    pub route_geometry: String,
    pub candidate_scores: String,
    pub penalties: String,
    pub tie_status: bool,
    pub tie_break_reason: String,
    pub chosen_candidate: String,
    pub recipe_id: String,
    pub execution_summary: String,
}

#[derive(Debug, Clone)]
pub struct PropertySummary {
    pub cases: u64,
    pub seed: u64,
}

#[derive(Debug, Clone)]
pub struct MetamorphicSummary {
    pub checks: usize,
}

#[derive(Debug, Clone)]
pub struct DifferentialSummary {
    pub cases: u64,
    pub seed: u64,
    pub mismatches: usize,
}

#[derive(Debug, Clone)]
pub struct RollbackSummary {
    pub cut_count: usize,
    pub recipes: usize,
}

#[derive(Debug, Clone)]
pub struct TraceCorruptionSummary {
    pub corruption_count: usize,
    pub false_acceptances: usize,
}

#[derive(Debug, Clone)]
pub struct SemanticHashSummary {
    pub corpus_cases: u64,
    pub seed: u64,
    pub hash_hex: String,
    pub comparison_label: Option<String>,
    pub comparison_hash: Option<String>,
    pub mismatch_count: usize,
}

#[derive(Debug, Clone)]
pub struct BenchmarkSummary {
    pub stages: Vec<StageStats>,
}

#[derive(Debug, Clone)]
pub struct StageStats {
    pub stage: &'static str,
    pub iterations: usize,
    pub mean_us: f64,
    pub median_us: f64,
    pub p95_us: f64,
    pub p99_us: f64,
    pub stddev_us: f64,
    pub throughput_per_sec: f64,
    pub trace_size_bytes: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct SyntheticScaleStats {
    pub tier: &'static str,
    pub candidates: usize,
    pub facts: usize,
    pub generate_us: f64,
    pub evaluate_us: f64,
    pub choose_us: f64,
    pub trace_us: f64,
    pub replay_us: f64,
    pub trace_size_bytes: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ObservationSurface {
    PointOnly,
    KernelStraight,
    KernelCurved,
    SyntheticInverted,
}

#[derive(Debug, Clone)]
struct VerificationInput {
    frame: FrameId,
    flows: Vec<FlowId>,
    glows: Vec<GlowId>,
    intent: DecisionIntent,
    surface: ObservationSurface,
}

#[derive(Debug, Clone)]
struct ReferenceDecision {
    generation: Vec<DecisionGeneratedCandidateTrace>,
    evaluations: Vec<DecisionEvaluationTrace>,
    choice: DecisionChoiceTrace,
    recipe_bridge: DecisionRecipeBridgeTrace,
}

#[derive(Debug, Clone, Copy)]
struct DeterministicRng {
    state: u64,
}

impl DeterministicRng {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn next_usize(&mut self, upper: usize) -> usize {
        if upper <= 1 {
            0
        } else {
            (self.next_u64() as usize) % upper
        }
    }

    fn next_bool(&mut self) -> bool {
        self.next_u64() & 1 == 0
    }
}

pub fn run_profile(
    profile: VerificationProfile,
    seed: u64,
    comparison_label: Option<String>,
    comparison_hash: Option<String>,
) -> VerificationReport {
    run_profile_with_overrides(
        profile,
        seed,
        comparison_label,
        comparison_hash,
        None,
        None,
        None,
    )
}

pub fn run_profile_with_overrides(
    profile: VerificationProfile,
    seed: u64,
    comparison_label: Option<String>,
    comparison_hash: Option<String>,
    property_cases_override: Option<u64>,
    differential_cases_override: Option<u64>,
    hash_cases_override: Option<u64>,
) -> VerificationReport {
    let exhaustive = exhaustive_state_space();
    let property_cases = property_cases_override.unwrap_or(profile.property_cases());
    let differential_cases = differential_cases_override.unwrap_or(profile.differential_cases());
    let hash_cases = hash_cases_override.unwrap_or(profile.semantic_hash_cases());
    let property = run_property_suite(property_cases, seed);
    let metamorphic = run_metamorphic_suite();
    let differential = run_differential_suite(differential_cases, seed);
    let rollback = run_rollback_suite();
    let trace_corruption = run_trace_corruption_suite();
    let semantic_hash = semantic_hash_summary(hash_cases, seed, comparison_label, comparison_hash);
    let benchmarks = benchmark_stages(profile.benchmark_iterations(), seed);
    let synthetic_scale = benchmark_synthetic_scale(profile.synthetic_include_extreme(), seed);

    VerificationReport {
        profile,
        seed,
        exhaustive,
        property,
        metamorphic,
        differential,
        rollback,
        trace_corruption,
        semantic_hash,
        benchmarks,
        synthetic_scale,
        skipped: vec![
            SkippedStage {
                stage: "fuzz",
                reason:
                    "cargo-fuzz is not configured in this milestone; the harness uses deterministic hostile generated cases instead."
                        .to_string(),
            },
            SkippedStage {
                stage: "mutation",
                reason:
                    "mutation tooling is documented through the wrapper script, but no local mutation engine is bundled in this workspace."
                        .to_string(),
            },
            SkippedStage {
                stage: "miri_sanitizers",
                reason:
                    "Miri and sanitizer runs remain external-tool dependent and are reported by wrapper scripts when available."
                        .to_string(),
            },
            SkippedStage {
                stage: "long_soak",
                reason:
                    "long-duration soak infrastructure is exposed through soak-local.sh and is not run inside the fast/full report path."
                        .to_string(),
            },
        ],
    }
}

pub fn semantic_hash_only(cases: u64, seed: u64) -> String {
    semantic_hash_summary(cases, seed, None, None).hash_hex
}

pub fn render_exhaustive_matrix(rows: &[ExhaustiveMatrixRow]) -> String {
    let mut out = String::from(
        "observation_identity\tframe\tflows\tglows\tintent\troute_geometry\tcandidate_scores\tpenalties\ttie_status\ttie_break_reason\tchosen_candidate\trecipe_id\texecution_summary\n",
    );
    for row in rows {
        let _ = writeln!(
            out,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            row.observation_identity,
            row.frame,
            row.flows,
            row.glows,
            row.intent,
            row.route_geometry,
            row.candidate_scores,
            row.penalties,
            row.tie_status,
            row.tie_break_reason,
            row.chosen_candidate,
            row.recipe_id,
            row.execution_summary,
        );
    }
    out
}

pub fn report_to_json(report: &VerificationReport) -> String {
    let mut json = String::new();
    json.push_str("{\n");
    push_json_field(&mut json, "profile", profile_name(report.profile), true);
    push_json_u64(&mut json, "seed", report.seed, true);
    json.push_str("  \"exhaustive\": {\n");
    push_json_usize(
        &mut json,
        "total_enumerated_states",
        report.exhaustive.total_enumerated_states,
        true,
    );
    push_json_usize(
        &mut json,
        "total_legal_states",
        report.exhaustive.total_legal_states,
        true,
    );
    push_json_usize(
        &mut json,
        "total_rejected_states",
        report.exhaustive.total_rejected_states,
        true,
    );
    push_json_usize(
        &mut json,
        "unique_decision_outcomes",
        report.exhaustive.unique_decision_outcomes,
        true,
    );
    push_json_usize(
        &mut json,
        "unique_trace_shapes",
        report.exhaustive.unique_trace_shapes,
        true,
    );
    push_json_usize(
        &mut json,
        "unexplained_states",
        report.exhaustive.unexplained_states,
        true,
    );
    push_json_usize(
        &mut json,
        "nondeterministic_states",
        report.exhaustive.nondeterministic_states,
        true,
    );
    push_json_usize(
        &mut json,
        "invariant_failures",
        report.exhaustive.invariant_failures,
        false,
    );
    json.push_str("  },\n");
    json.push_str("  \"property\": {\n");
    push_json_u64(&mut json, "cases", report.property.cases, true);
    push_json_u64(&mut json, "seed", report.property.seed, false);
    json.push_str("  },\n");
    json.push_str("  \"metamorphic\": {\n");
    push_json_usize(&mut json, "checks", report.metamorphic.checks, false);
    json.push_str("  },\n");
    json.push_str("  \"differential\": {\n");
    push_json_u64(&mut json, "cases", report.differential.cases, true);
    push_json_u64(&mut json, "seed", report.differential.seed, true);
    push_json_usize(
        &mut json,
        "mismatches",
        report.differential.mismatches,
        false,
    );
    json.push_str("  },\n");
    json.push_str("  \"rollback\": {\n");
    push_json_usize(&mut json, "cut_count", report.rollback.cut_count, true);
    push_json_usize(&mut json, "recipes", report.rollback.recipes, false);
    json.push_str("  },\n");
    json.push_str("  \"trace_corruption\": {\n");
    push_json_usize(
        &mut json,
        "corruption_count",
        report.trace_corruption.corruption_count,
        true,
    );
    push_json_usize(
        &mut json,
        "false_acceptances",
        report.trace_corruption.false_acceptances,
        false,
    );
    json.push_str("  },\n");
    json.push_str("  \"semantic_hash\": {\n");
    push_json_u64(
        &mut json,
        "corpus_cases",
        report.semantic_hash.corpus_cases,
        true,
    );
    push_json_u64(&mut json, "seed", report.semantic_hash.seed, true);
    push_json_field(&mut json, "hash_hex", &report.semantic_hash.hash_hex, true);
    push_json_optional_field(
        &mut json,
        "comparison_label",
        report.semantic_hash.comparison_label.as_deref(),
        true,
    );
    push_json_optional_field(
        &mut json,
        "comparison_hash",
        report.semantic_hash.comparison_hash.as_deref(),
        true,
    );
    push_json_usize(
        &mut json,
        "mismatch_count",
        report.semantic_hash.mismatch_count,
        false,
    );
    json.push_str("  },\n");
    json.push_str("  \"skipped\": [\n");
    for (index, skipped) in report.skipped.iter().enumerate() {
        json.push_str("    {");
        let _ = write!(
            json,
            "\"stage\":\"{}\",\"reason\":\"{}\"",
            json_escape(skipped.stage),
            json_escape(&skipped.reason)
        );
        json.push('}');
        if index + 1 != report.skipped.len() {
            json.push(',');
        }
        json.push('\n');
    }
    json.push_str("  ]\n");
    json.push('}');
    json
}

pub fn report_to_markdown(report: &VerificationReport) -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "# Hollow Grove Adversarial Verification ({})",
        profile_name(report.profile)
    );
    let _ = writeln!(out);
    let _ = writeln!(out, "- seed: `{}`", report.seed);
    let _ = writeln!(
        out,
        "- exhaustive states: `{}` legal / `{}` rejected / `{}` total",
        report.exhaustive.total_legal_states,
        report.exhaustive.total_rejected_states,
        report.exhaustive.total_enumerated_states
    );
    let _ = writeln!(
        out,
        "- exhaustive failures: unexplained `{}`, nondeterministic `{}`, invariant failures `{}`",
        report.exhaustive.unexplained_states,
        report.exhaustive.nondeterministic_states,
        report.exhaustive.invariant_failures
    );
    let _ = writeln!(out, "- property cases: `{}`", report.property.cases);
    let _ = writeln!(out, "- metamorphic checks: `{}`", report.metamorphic.checks);
    let _ = writeln!(
        out,
        "- differential cases: `{}` with mismatches `{}`",
        report.differential.cases, report.differential.mismatches
    );
    let _ = writeln!(
        out,
        "- rollback cuts: `{}` across `{}` canonical recipes",
        report.rollback.cut_count, report.rollback.recipes
    );
    let _ = writeln!(
        out,
        "- trace corruptions: `{}` with false acceptances `{}`",
        report.trace_corruption.corruption_count, report.trace_corruption.false_acceptances
    );
    let _ = writeln!(
        out,
        "- semantic corpus hash: `{}`",
        report.semantic_hash.hash_hex
    );
    if let Some(label) = report.semantic_hash.comparison_label.as_deref() {
        let hash = report
            .semantic_hash
            .comparison_hash
            .as_deref()
            .unwrap_or("unknown");
        let _ = writeln!(
            out,
            "- cross-build comparison (`{label}`): `{hash}` with mismatch count `{}`",
            report.semantic_hash.mismatch_count
        );
    }
    let _ = writeln!(out);
    let _ = writeln!(out, "## Benchmarks");
    for stage in &report.benchmarks.stages {
        let _ = writeln!(
            out,
            "- `{}`: mean `{:.2} us`, median `{:.2} us`, p95 `{:.2} us`, p99 `{:.2} us`, throughput `{:.0}/s`",
            stage.stage,
            stage.mean_us,
            stage.median_us,
            stage.p95_us,
            stage.p99_us,
            stage.throughput_per_sec
        );
    }
    let _ = writeln!(out);
    let _ = writeln!(out, "## Synthetic Scale");
    for scale in &report.synthetic_scale {
        let _ = writeln!(
            out,
            "- `{}`: {} candidates / {} facts, choose `{:.2} us`, replay `{:.2} us`, trace bytes `{}`",
            scale.tier,
            scale.candidates,
            scale.facts,
            scale.choose_us,
            scale.replay_us,
            scale.trace_size_bytes
        );
    }
    let _ = writeln!(out);
    let _ = writeln!(out, "## Skipped");
    for skipped in &report.skipped {
        let _ = writeln!(out, "- `{}`: {}", skipped.stage, skipped.reason);
    }
    out
}

fn exhaustive_state_space() -> ExhaustiveSummary {
    let flow_sets = flow_variants();
    let glow_sets = glow_variants();
    let surfaces = [
        ObservationSurface::PointOnly,
        ObservationSurface::KernelStraight,
        ObservationSurface::KernelCurved,
        ObservationSurface::SyntheticInverted,
    ];
    let mut rows = Vec::new();
    let mut total_enumerated_states = 0usize;
    let mut total_legal_states = 0usize;
    let mut total_rejected_states = 0usize;
    let mut unique_decision_outcomes = BTreeSet::new();
    let mut unique_trace_shapes = BTreeSet::new();
    let mut equivalence_classes = BTreeMap::new();
    let mut unexplained_states = 0usize;
    let mut nondeterministic_states = 0usize;
    let invariant_failures = 0usize;

    for frame in FRAMES {
        for flows in &flow_sets {
            for glows in &glow_sets {
                for intent in INTENTS {
                    for surface in surfaces {
                        total_enumerated_states += 1;
                        if surface == ObservationSurface::KernelStraight
                            || surface == ObservationSurface::KernelCurved
                            || surface == ObservationSurface::PointOnly
                            || surface == ObservationSurface::SyntheticInverted
                        {
                            total_legal_states += 1;
                        } else {
                            total_rejected_states += 1;
                            continue;
                        }

                        let input = VerificationInput {
                            frame,
                            flows: flows.clone(),
                            glows: glows.clone(),
                            intent,
                            surface,
                        };
                        let point = build_point(frame, flows, glows);
                        let observation = build_observation(&input, &point);
                        let first = execute_observed_decision(observation.clone())
                            .expect("decision should execute");
                        let second = execute_observed_decision(observation)
                            .expect("decision should execute");
                        if first.trace() != second.trace()
                            || first.execution() != second.execution()
                        {
                            nondeterministic_states += 1;
                        }

                        let row = build_matrix_row(surface, &first);
                        unique_decision_outcomes.insert(format!(
                            "{}|{}|{}",
                            row.chosen_candidate, row.recipe_id, row.execution_summary
                        ));
                        unique_trace_shapes.insert(trace_shape_key(first.trace()));

                        let eq = equivalence_class(&input);
                        *equivalence_classes.entry(eq).or_insert(0usize) += 1;

                        if row.chosen_candidate.is_empty() {
                            unexplained_states += 1;
                        }

                        rows.push(row);
                    }
                }
            }
        }
    }

    ExhaustiveSummary {
        rows,
        total_enumerated_states,
        total_legal_states,
        total_rejected_states,
        unique_decision_outcomes: unique_decision_outcomes.len(),
        unique_trace_shapes: unique_trace_shapes.len(),
        unexplained_states,
        nondeterministic_states,
        invariant_failures,
        equivalence_classes,
    }
}

fn run_property_suite(cases: u64, seed: u64) -> PropertySummary {
    let mut rng = DeterministicRng::new(seed);

    for _ in 0..cases {
        let input = random_input(&mut rng);
        let point = build_point(input.frame, &input.flows, &input.glows);
        let before_point = point.clone();
        let observation = build_observation(&input, &point);
        let first =
            execute_observed_decision(observation.clone()).expect("decision should execute");
        let second =
            execute_observed_decision(observation.clone()).expect("decision should execute");
        assert_eq!(
            first.chosen().candidate().candidate_id(),
            second.chosen().candidate().candidate_id()
        );
        assert_eq!(first.trace(), second.trace());
        assert_eq!(point, before_point);
        assert_eq!(
            first.observation().frame_state(),
            before_point.frame_state()
        );
        assert!(
            first
                .candidates()
                .iter()
                .any(|candidate| candidate.candidate_id()
                    == first.chosen().candidate().candidate_id())
        );
        assert_eq!(
            resolve_candidate_recipe(first.chosen().candidate().candidate_id()).recipe_id(),
            first.recipe().recipe_id()
        );
        for evaluation in first.trace().evaluations() {
            assert_eq!(
                evaluation.intent_score() - evaluation.realized_state_penalty(),
                evaluation.final_score()
            );
        }
        replay_trace_for_observation(observation, first.trace())
            .expect("trace replay should succeed");
        assert_eq!(point, before_point);

        let miss = land_contact(
            point.frame_state(),
            first.execution().aim(),
            ContactOutcome::Miss,
        )
        .expect("miss should preserve state");
        assert_eq!(miss.contact(), ContactOutcome::Miss);
        assert_eq!(miss.frame_state(), point.frame_state());
    }

    PropertySummary { cases, seed }
}

fn run_metamorphic_suite() -> MetamorphicSummary {
    let mut checks = 0usize;
    let base = build_point(FrameId::Hueman, &[], &[]);

    for frame in [
        FrameId::Hueman,
        FrameId::Gremlin,
        FrameId::Pixy,
        FrameId::Goblin,
    ] {
        for intent in INTENTS {
            let point = build_point(frame, &[], &[]);
            let base_observation = observe_decision(&point, intent);
            let base_trace = execute_observed_decision(base_observation.clone())
                .expect("decision should execute")
                .trace()
                .clone();

            let with_flow = build_point(frame, &[FlowId::Stonefold], &[]);
            let flow_trace = execute_observed_decision(observe_decision(&with_flow, intent))
                .expect("decision should execute")
                .trace()
                .clone();
            assert_eq!(
                base_trace.choice().chosen_candidate(),
                flow_trace.choice().chosen_candidate()
            );
            checks += 1;

            let with_glow = build_point(frame, &[], &[GlowId::Projection]);
            let glow_trace = execute_observed_decision(observe_decision(&with_glow, intent))
                .expect("decision should execute")
                .trace()
                .clone();
            assert_eq!(
                base_trace.choice().chosen_candidate(),
                glow_trace.choice().chosen_candidate()
            );
            checks += 1;
        }
    }

    for intent in [DecisionIntent::FavorCurrent, DecisionIntent::FavorAura] {
        let point = base.clone();
        let plain = execute_observed_decision(observe_decision(&point, intent))
            .expect("decision should execute");
        let straight = execute_observed_decision(observe_decision_with_geometry(
            &point,
            intent,
            Some(ManagerGeometry::Straight),
        ))
        .expect("decision should execute");
        let curved = execute_observed_decision(observe_decision_with_geometry(
            &point,
            intent,
            Some(ManagerGeometry::Curved),
        ))
        .expect("decision should execute");
        assert_eq!(
            plain.chosen().candidate().candidate_id(),
            straight.chosen().candidate().candidate_id()
        );
        assert_eq!(
            plain.chosen().candidate().candidate_id(),
            curved.chosen().candidate().candidate_id()
        );
        checks += 2;
    }

    let neutral_plain = execute_observed_decision(observe_decision(&base, DecisionIntent::Neutral))
        .expect("decision should execute");
    let neutral_straight = execute_observed_decision(observe_decision_with_geometry(
        &base,
        DecisionIntent::Neutral,
        Some(ManagerGeometry::Straight),
    ))
    .expect("decision should execute");
    let neutral_curved = execute_observed_decision(observe_decision_with_geometry(
        &base,
        DecisionIntent::Neutral,
        Some(ManagerGeometry::Curved),
    ))
    .expect("decision should execute");
    let neutral_inverted = execute_observed_decision(observe_decision_with_geometry(
        &base,
        DecisionIntent::Neutral,
        Some(ManagerGeometry::Inverted),
    ))
    .expect("decision should execute");
    assert_eq!(
        neutral_plain.chosen().candidate().candidate_id(),
        DecisionCandidateId::GremlinTinker
    );
    assert_eq!(
        neutral_straight.chosen().candidate().candidate_id(),
        DecisionCandidateId::GremlinTinker
    );
    assert_eq!(
        neutral_curved.chosen().candidate().candidate_id(),
        DecisionCandidateId::PixyConfusion
    );
    assert_eq!(
        neutral_inverted.chosen().candidate().candidate_id(),
        DecisionCandidateId::GremlinTinker
    );
    assert_eq!(
        neutral_inverted.trace().choice().tie_break_reason(),
        DecisionTraceTieBreakReason::CanonicalGenerateOrder
    );
    checks += 4;

    let rendered_once = render_trace_text(neutral_plain.trace());
    let rendered_twice = render_trace_text(neutral_plain.trace());
    assert_eq!(rendered_once, rendered_twice);
    replay_trace_for_observation(
        observe_decision(&base, DecisionIntent::Neutral),
        neutral_plain.trace(),
    )
    .expect("replay should succeed");
    replay_trace_for_observation(
        observe_decision(&base, DecisionIntent::Neutral),
        neutral_plain.trace(),
    )
    .expect("replay should remain stable");
    checks += 3;

    MetamorphicSummary { checks }
}

fn run_differential_suite(cases: u64, seed: u64) -> DifferentialSummary {
    let mut rng = DeterministicRng::new(seed ^ 0xD1FF_EE11);
    let mut mismatches = 0usize;

    for _ in 0..cases {
        let input = random_input(&mut rng);
        let point = build_point(input.frame, &input.flows, &input.glows);
        let observation = build_observation(&input, &point);
        let plan = build_decision_plan(observation).expect("plan should build");
        let trace = build_decision_trace_without_execution(&plan);
        let reference = reference_decision(&plan.observation);

        if trace.generation() != reference.generation
            || trace.evaluations() != reference.evaluations
            || trace.choice() != &reference.choice
            || trace.recipe_bridge() != &reference.recipe_bridge
        {
            mismatches += 1;
        }
    }

    DifferentialSummary {
        cases,
        seed,
        mismatches,
    }
}

fn run_rollback_suite() -> RollbackSummary {
    let recipes = [
        resolve_candidate_recipe(DecisionCandidateId::GremlinTinker),
        resolve_candidate_recipe(DecisionCandidateId::PixyConfusion),
    ];
    let mut cut_count = 0usize;

    for recipe in recipes {
        let start = FrameState::origin();
        let scripts = compile_recipe(&recipe).expect("recipe should compile");
        let aim = build_aim(&recipe, scripts.clone());
        for cut in landing_fault_cuts(scripts.len()) {
            let before = start.clone();
            let result = apply_kiss_with_fault(&start, &aim, Some(cut));
            assert_eq!(start, before);
            assert!(matches!(result, Err(LandingFaultError::Injected(_))));

            let success =
                apply_kiss_with_fault(&start, &aim, None).expect("landing should recover");
            assert_eq!(success.contact(), ContactOutcome::Kiss);
            cut_count += 1;
        }
    }

    RollbackSummary {
        cut_count,
        recipes: 2,
    }
}

fn run_trace_corruption_suite() -> TraceCorruptionSummary {
    let traces = canonical_traces();
    let mut corruption_count = 0usize;
    let mut false_acceptances = 0usize;

    for (observation, trace) in traces {
        for (name, corrupted) in corrupt_trace_variants(&trace) {
            corruption_count += 1;
            if replay_trace_for_observation(observation.clone(), &corrupted).is_ok() {
                eprintln!("false acceptance: {name}");
                false_acceptances += 1;
            }
        }
    }

    TraceCorruptionSummary {
        corruption_count,
        false_acceptances,
    }
}

fn semantic_hash_summary(
    cases: u64,
    seed: u64,
    comparison_label: Option<String>,
    comparison_hash: Option<String>,
) -> SemanticHashSummary {
    let mut rng = DeterministicRng::new(seed ^ 0x5EED_FACE);
    let mut hasher = StableHasher::new();
    for _ in 0..cases {
        let input = random_input(&mut rng);
        let point = build_point(input.frame, &input.flows, &input.glows);
        let observation = build_observation(&input, &point);
        let execution = execute_observed_decision(observation).expect("decision should execute");
        hasher.write_str(&semantic_input_key(&input));
        hasher.write_str(&semantic_trace_key(execution.trace()));
        hasher.write_str(execution.recipe().recipe_id());
    }
    let hash_hex = hasher.finish_hex();
    let mismatch_count = comparison_hash
        .as_deref()
        .map(|other| usize::from(other != hash_hex))
        .unwrap_or(0);

    SemanticHashSummary {
        corpus_cases: cases,
        seed,
        hash_hex,
        comparison_label,
        comparison_hash,
        mismatch_count,
    }
}

fn benchmark_stages(iterations: usize, seed: u64) -> BenchmarkSummary {
    let corpus = benchmark_corpus(iterations, seed);
    let mut stages = Vec::new();

    stages.push(measure_stage("observe", iterations, None, || {
        for input in &corpus {
            let point = build_point(input.frame, &input.flows, &input.glows);
            let _ = build_observation(input, &point);
        }
    }));

    stages.push(measure_stage("generate", iterations, None, || {
        for input in &corpus {
            let point = build_point(input.frame, &input.flows, &input.glows);
            let observation = build_observation(input, &point);
            let _ = generate_decision_candidates(&observation);
        }
    }));

    stages.push(measure_stage(
        "evaluate_one_candidate",
        iterations,
        None,
        || {
            for input in &corpus {
                let point = build_point(input.frame, &input.flows, &input.glows);
                let observation = build_observation(input, &point);
                let candidates = generate_decision_candidates(&observation);
                let _ = crate::evaluate_decision_candidate(&observation, &candidates[0]);
            }
        },
    ));

    stages.push(measure_stage(
        "evaluate_complete_candidate_set",
        iterations,
        None,
        || {
            for input in &corpus {
                let point = build_point(input.frame, &input.flows, &input.glows);
                let observation = build_observation(input, &point);
                let candidates = generate_decision_candidates(&observation);
                for candidate in &candidates {
                    let _ = crate::evaluate_decision_candidate(&observation, candidate);
                }
            }
        },
    ));

    stages.push(measure_stage("choose", iterations, None, || {
        for input in &corpus {
            let point = build_point(input.frame, &input.flows, &input.glows);
            let observation = build_observation(input, &point);
            let candidates = generate_decision_candidates(&observation);
            let evaluations = candidates
                .iter()
                .map(|candidate| crate::evaluate_decision_candidate(&observation, candidate))
                .collect::<Vec<_>>();
            let _ = crate::choose_decision_for_observation(
                Some(&observation),
                &candidates,
                &evaluations,
            )
            .expect("choice should work");
        }
    }));

    let avg_trace_size = Cell::new(0usize);
    stages.push(measure_stage(
        "trace_construction",
        iterations,
        None,
        || {
            for input in &corpus {
                let point = build_point(input.frame, &input.flows, &input.glows);
                let observation = build_observation(input, &point);
                let plan = build_decision_plan(observation).expect("plan should build");
                let trace = build_decision_trace_without_execution(&plan);
                avg_trace_size.set(avg_trace_size.get() + render_trace_text(&trace).len());
            }
        },
    ));
    if let Some(last) = stages.last_mut() {
        last.trace_size_bytes = Some(avg_trace_size.get() / iterations.max(1));
    }

    stages.push(measure_stage(
        "replay_verification",
        iterations,
        None,
        || {
            for input in &corpus {
                let point = build_point(input.frame, &input.flows, &input.glows);
                let observation = build_observation(input, &point);
                let execution =
                    execute_observed_decision(observation.clone()).expect("execution should work");
                replay_trace_for_observation(observation, execution.trace())
                    .expect("replay should work");
            }
        },
    ));

    stages.push(measure_stage("recipe_resolution", iterations, None, || {
        for input in &corpus {
            let point = build_point(input.frame, &input.flows, &input.glows);
            let observation = build_observation(input, &point);
            let plan = build_decision_plan(observation).expect("plan should build");
            let _ = resolve_candidate_recipe(plan.chosen.candidate().candidate_id());
        }
    }));

    stages.push(measure_stage(
        "version_1_1_execution",
        iterations,
        None,
        || {
            for input in &corpus {
                let point = build_point(input.frame, &input.flows, &input.glows);
                let recipe = resolve_candidate_recipe(DecisionCandidateId::GremlinTinker);
                let _ = execute_synthesis_recipe(&point, &recipe).expect("execution should work");
            }
        },
    ));

    stages.push(measure_stage(
        "full_decision_without_witness",
        iterations,
        None,
        || {
            for input in &corpus {
                let point = build_point(input.frame, &input.flows, &input.glows);
                let observation = build_observation(input, &point);
                let _ = execute_observed_decision(observation).expect("decision should work");
            }
        },
    ));

    let rendered_bytes = Cell::new(0usize);
    stages.push(measure_stage(
        "full_decision_with_trace",
        iterations,
        None,
        || {
            for input in &corpus {
                let point = build_point(input.frame, &input.flows, &input.glows);
                let observation = build_observation(input, &point);
                let execution =
                    execute_observed_decision(observation).expect("decision should work");
                rendered_bytes
                    .set(rendered_bytes.get() + render_trace_text(execution.trace()).len());
            }
        },
    ));
    if let Some(last) = stages.last_mut() {
        last.trace_size_bytes = Some(rendered_bytes.get() / iterations.max(1));
    }

    stages.push(measure_stage("witness_rendering", iterations, None, || {
        for input in &corpus {
            let point = build_point(input.frame, &input.flows, &input.glows);
            let observation = build_observation(input, &point);
            let execution = execute_observed_decision(observation).expect("decision should work");
            let _ = render_trace_text(execution.trace());
        }
    }));

    BenchmarkSummary { stages }
}

fn benchmark_synthetic_scale(include_extreme: bool, seed: u64) -> Vec<SyntheticScaleStats> {
    let mut rng = DeterministicRng::new(seed ^ 0x5151_CA1E);
    let mut tiers = vec![
        ("TINY", 2usize, 5usize),
        ("SMALL", 32usize, 16usize),
        ("MEDIUM", 1_000usize, 64usize),
        ("LARGE", 10_000usize, 256usize),
    ];
    if include_extreme {
        tiers.push(("EXTREME", 100_000usize, 1_000usize));
    }

    tiers
        .into_iter()
        .map(|(tier, candidates, facts)| synthetic_scale_stat(tier, candidates, facts, &mut rng))
        .collect()
}

fn synthetic_scale_stat(
    tier: &'static str,
    candidate_count: usize,
    fact_count: usize,
    rng: &mut DeterministicRng,
) -> SyntheticScaleStats {
    let facts = (0..fact_count)
        .map(|index| (index as u32, rng.next_bool()))
        .collect::<Vec<_>>();
    let start = Instant::now();
    let candidates = (0..candidate_count)
        .map(|index| SyntheticCandidate {
            id: index,
            geometry: match index % 3 {
                0 => ManagerGeometry::Straight,
                1 => ManagerGeometry::Curved,
                _ => ManagerGeometry::Inverted,
            },
            base_score: (index % 5) as u16,
        })
        .collect::<Vec<_>>();
    let generate_us = start.elapsed().as_secs_f64() * 1_000_000.0;

    let eval_start = Instant::now();
    let evaluated = candidates
        .iter()
        .map(|candidate| SyntheticEvaluation {
            id: candidate.id,
            score: candidate.base_score + u16::from(facts[candidate.id % facts.len()].1),
            geometry: candidate.geometry,
        })
        .collect::<Vec<_>>();
    let evaluate_us = eval_start.elapsed().as_secs_f64() * 1_000_000.0;

    let choose_start = Instant::now();
    let chosen = evaluated
        .iter()
        .max_by_key(|evaluation| {
            (
                evaluation.score,
                synthetic_geometry_rank(evaluation.geometry),
            )
        })
        .expect("synthetic candidate should exist");
    let choose_us = choose_start.elapsed().as_secs_f64() * 1_000_000.0;

    let trace_start = Instant::now();
    let trace = format!(
        "{tier}|candidates={candidate_count}|facts={fact_count}|chosen={}|score={}",
        chosen.id, chosen.score
    );
    let trace_us = trace_start.elapsed().as_secs_f64() * 1_000_000.0;

    let replay_start = Instant::now();
    assert!(trace.contains("chosen="));
    let replay_us = replay_start.elapsed().as_secs_f64() * 1_000_000.0;

    SyntheticScaleStats {
        tier,
        candidates: candidate_count,
        facts: fact_count,
        generate_us,
        evaluate_us,
        choose_us,
        trace_us,
        replay_us,
        trace_size_bytes: trace.len(),
    }
}

fn random_input(rng: &mut DeterministicRng) -> VerificationInput {
    VerificationInput {
        frame: FRAMES[rng.next_usize(FRAMES.len())],
        flows: random_subset_flows(rng),
        glows: random_subset_glows(rng),
        intent: INTENTS[rng.next_usize(INTENTS.len())],
        surface: match rng.next_usize(4) {
            0 => ObservationSurface::PointOnly,
            1 => ObservationSurface::KernelStraight,
            2 => ObservationSurface::KernelCurved,
            _ => ObservationSurface::SyntheticInverted,
        },
    }
}

fn benchmark_corpus(iterations: usize, seed: u64) -> Vec<VerificationInput> {
    let mut rng = DeterministicRng::new(seed ^ 0xBEEF_0001);
    (0..iterations).map(|_| random_input(&mut rng)).collect()
}

fn random_subset_flows(rng: &mut DeterministicRng) -> Vec<FlowId> {
    FLOWS.iter().copied().filter(|_| rng.next_bool()).collect()
}

fn random_subset_glows(rng: &mut DeterministicRng) -> Vec<GlowId> {
    GLOWS.iter().copied().filter(|_| rng.next_bool()).collect()
}

fn flow_variants() -> Vec<Vec<FlowId>> {
    vec![
        Vec::new(),
        vec![FlowId::TinkerGrip],
        vec![FlowId::Stonefold],
        vec![FlowId::Stonefold, FlowId::TinkerGrip],
    ]
}

fn glow_variants() -> Vec<Vec<GlowId>> {
    vec![
        Vec::new(),
        vec![GlowId::Confusion],
        vec![GlowId::Projection],
        vec![GlowId::Projection, GlowId::Confusion],
    ]
}

fn build_point(frame: FrameId, flows: &[FlowId], glows: &[GlowId]) -> Point {
    Point::new(FrameState::new(
        frame,
        CurrentPrism::origin(),
        flows.to_vec(),
        glows.to_vec(),
    ))
}

fn build_observation(input: &VerificationInput, point: &Point) -> DecisionObservation {
    match input.surface {
        ObservationSurface::PointOnly => observe_decision(point, input.intent),
        ObservationSurface::KernelStraight => observe_kernel_pass_decision(
            &run_kernel_cycle_with_input(
                crate::Symptom::new(point.clone()),
                KernelInput::default(),
            ),
            input.intent,
        ),
        ObservationSurface::KernelCurved => observe_kernel_pass_decision(
            &run_kernel_cycle_with_input(
                crate::Symptom::new(point.clone()),
                KernelInput {
                    routing: crate::PlebMetaInput {
                        exterior_shape: ExteriorShape::Curved,
                        pleb_mode: crate::Mode::Pathos,
                        meta_mode: crate::Mode::Logos,
                    },
                },
            ),
            input.intent,
        ),
        ObservationSurface::SyntheticInverted => {
            observe_decision_with_geometry(point, input.intent, Some(ManagerGeometry::Inverted))
        }
    }
}

fn build_matrix_row(
    surface: ObservationSurface,
    execution: &DecisionExecution,
) -> ExhaustiveMatrixRow {
    let trace = execution.trace();
    ExhaustiveMatrixRow {
        observation_identity: match surface {
            ObservationSurface::PointOnly => "point-only".to_string(),
            ObservationSurface::KernelStraight => "kernel-pass-straight".to_string(),
            ObservationSurface::KernelCurved => "kernel-pass-curved".to_string(),
            ObservationSurface::SyntheticInverted => "synthetic-inverted".to_string(),
        },
        frame: format!("{:?}", trace.observation().frame()),
        flows: join_debug(trace.observation().flows()),
        glows: join_debug(trace.observation().glows()),
        intent: trace.observation().intent().as_str().to_string(),
        route_geometry: trace
            .observation()
            .route_geometry()
            .map(|geometry| format!("{geometry:?}"))
            .unwrap_or_else(|| "None".to_string()),
        candidate_scores: trace
            .evaluations()
            .iter()
            .map(|evaluation| {
                format!(
                    "{:?}={}",
                    evaluation.candidate_id(),
                    evaluation.final_score()
                )
            })
            .collect::<Vec<_>>()
            .join(","),
        penalties: trace
            .evaluations()
            .iter()
            .map(|evaluation| {
                format!(
                    "{:?}={}",
                    evaluation.candidate_id(),
                    evaluation.realized_state_penalty()
                )
            })
            .collect::<Vec<_>>()
            .join(","),
        tie_status: trace.choice().tie_occurred(),
        tie_break_reason: trace.choice().tie_break_reason().as_str().to_string(),
        chosen_candidate: format!("{:?}", trace.choice().chosen_candidate()),
        recipe_id: trace.recipe_bridge().recipe_id().to_string(),
        execution_summary: format!(
            "{:?}|frame={:?}|flow={}|glow={}|point_squared={}",
            trace.execution().contact(),
            trace.execution().landed_frame(),
            join_debug(trace.execution().added_flow()),
            join_debug(trace.execution().added_glow()),
            trace.execution().point_squared_produced()
        ),
    }
}

fn trace_shape_key(trace: &DecisionTrace) -> String {
    format!(
        "{}|{}|{}|{}",
        trace.choice().tie_break_reason().as_str(),
        trace.choice().chosen_candidate().as_str(),
        trace.recipe_bridge().recipe_id(),
        trace.execution().contact().as_str()
    )
}

fn equivalence_class(input: &VerificationInput) -> String {
    let frame_class = match input.frame {
        FrameId::Hueman => "Hueman",
        FrameId::Gremlin => "Gremlin",
        FrameId::Pixy => "Pixy",
        _ => "Other",
    };
    format!(
        "frame={frame_class}|tinker={}|confusion={}|intent={}|surface={:?}",
        input.flows.contains(&FlowId::TinkerGrip),
        input.glows.contains(&GlowId::Confusion),
        input.intent.as_str(),
        input.surface
    )
}

fn reference_decision(observation: &DecisionObservation) -> ReferenceDecision {
    let generation = vec![
        DecisionGeneratedCandidateTrace {
            candidate_id: DecisionCandidateId::GremlinTinker,
            manager: Manager::Clouseau,
            manager_geometry: manager_domain_lock(Manager::Clouseau).geometry(),
            orientation: SynthesisOrientation::Current,
        },
        DecisionGeneratedCandidateTrace {
            candidate_id: DecisionCandidateId::PixyConfusion,
            manager: Manager::Hal,
            manager_geometry: manager_domain_lock(Manager::Hal).geometry(),
            orientation: SynthesisOrientation::Aura,
        },
    ];
    let evaluations = generation
        .iter()
        .map(|candidate| reference_evaluation(observation, *candidate))
        .collect::<Vec<_>>();
    let highest = evaluations
        .iter()
        .map(DecisionEvaluationTrace::final_score)
        .max()
        .expect("evaluation should exist");
    let tied_candidates = evaluations
        .iter()
        .filter(|evaluation| evaluation.final_score() == highest)
        .map(|evaluation| evaluation.candidate_id())
        .collect::<Vec<_>>();
    let tie_occurred = tied_candidates.len() > 1;
    let geometry_matching_candidate = if tie_occurred {
        observation.route_geometry().and_then(|geometry| {
            tied_candidates.iter().copied().find(|candidate_id| {
                generation.iter().any(|candidate| {
                    candidate.candidate_id() == *candidate_id
                        && candidate.manager_geometry() == geometry
                })
            })
        })
    } else {
        None
    };
    let chosen_candidate = if let Some(candidate_id) = geometry_matching_candidate {
        candidate_id
    } else {
        evaluations
            .first()
            .and_then(|_| {
                evaluations
                    .iter()
                    .find(|evaluation| evaluation.final_score() == highest)
            })
            .expect("winner should exist")
            .candidate_id()
    };
    let tie_break_reason = if !tie_occurred {
        DecisionTraceTieBreakReason::NoTie
    } else if geometry_matching_candidate.is_some() {
        DecisionTraceTieBreakReason::ObservedRouteGeometryMatch
    } else {
        DecisionTraceTieBreakReason::CanonicalGenerateOrder
    };

    ReferenceDecision {
        generation,
        evaluations,
        choice: DecisionChoiceTrace {
            highest_score: highest,
            tied_candidates: if tie_occurred {
                tied_candidates.clone()
            } else {
                Vec::new()
            },
            tie_occurred,
            observed_route_geometry: observation.route_geometry(),
            manager_geometry_matched: geometry_matching_candidate.is_some(),
            geometry_matching_candidate,
            tie_break_reason,
            generate_order_resolved: matches!(
                tie_break_reason,
                DecisionTraceTieBreakReason::CanonicalGenerateOrder
            ),
            chosen_candidate,
        },
        recipe_bridge: DecisionRecipeBridgeTrace {
            chosen_candidate,
            recipe_id: resolve_candidate_recipe(chosen_candidate)
                .recipe_id()
                .to_string(),
            handed_to_execution_facade: true,
        },
    }
}

fn reference_evaluation(
    observation: &DecisionObservation,
    candidate: DecisionGeneratedCandidateTrace,
) -> DecisionEvaluationTrace {
    let mut reason_codes = Vec::new();
    let intent_score = match observation.intent() {
        DecisionIntent::FavorCurrent => {
            if matches!(candidate.orientation(), SynthesisOrientation::Current) {
                reason_codes.push(DecisionTraceReasonCode::PreferredCurrentOrientation);
                2
            } else {
                reason_codes.push(DecisionTraceReasonCode::NonPreferredCurrentOrientation);
                1
            }
        }
        DecisionIntent::FavorAura => {
            if matches!(candidate.orientation(), SynthesisOrientation::Aura) {
                reason_codes.push(DecisionTraceReasonCode::PreferredAuraOrientation);
                2
            } else {
                reason_codes.push(DecisionTraceReasonCode::NonPreferredAuraOrientation);
                1
            }
        }
        DecisionIntent::Neutral => {
            reason_codes.push(DecisionTraceReasonCode::NeutralBaseScore);
            1
        }
    };

    let realized_state_penalty = if matches!(observation.intent(), DecisionIntent::Neutral) {
        match candidate.candidate_id() {
            DecisionCandidateId::GremlinTinker => {
                let mut penalty = 0;
                if observation.frame_state().frame() == FrameId::Gremlin {
                    reason_codes.push(DecisionTraceReasonCode::AlreadyCanonicalFrame);
                    penalty = 1;
                }
                if observation
                    .frame_state()
                    .flow_learnset()
                    .contains(&FlowId::TinkerGrip)
                {
                    reason_codes.push(DecisionTraceReasonCode::AlreadyKnowsCanonicalFlow);
                    penalty = 1;
                }
                penalty
            }
            DecisionCandidateId::PixyConfusion => {
                let mut penalty = 0;
                if observation.frame_state().frame() == FrameId::Pixy {
                    reason_codes.push(DecisionTraceReasonCode::AlreadyCanonicalFrame);
                    penalty = 1;
                }
                if observation
                    .frame_state()
                    .glow_learnset()
                    .contains(&GlowId::Confusion)
                {
                    reason_codes.push(DecisionTraceReasonCode::AlreadyKnowsCanonicalGlow);
                    penalty = 1;
                }
                penalty
            }
        }
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

fn build_aim(recipe: &SynthesisRecipe, scripts: Vec<SynthesisScript>) -> crate::Aim {
    match recipe.recipe_id() {
        "gremlin_tinker" => gremlin_tinker_aim(recipe, scripts).expect("gremlin aim should build"),
        "pixy_confusion" => pixy_confusion_aim(recipe, scripts).expect("pixy aim should build"),
        other => panic!("unsupported canonical recipe for rollback verification: {other}"),
    }
}

fn landing_fault_cuts(script_count: usize) -> Vec<LandingFaultCut> {
    let mut cuts = Vec::new();
    for index in 0..script_count {
        cuts.push(LandingFaultCut::BeforeScript(index));
        cuts.push(LandingFaultCut::DuringScript(index));
        if index + 1 < script_count {
            cuts.push(LandingFaultCut::BetweenScripts(index));
        }
    }
    cuts.push(LandingFaultCut::AfterFinalScriptStaging);
    cuts.push(LandingFaultCut::BeforeCommit);
    cuts
}

fn canonical_traces() -> Vec<(DecisionObservation, DecisionTrace)> {
    let cases = vec![
        VerificationInput {
            frame: FrameId::Hueman,
            flows: Vec::new(),
            glows: Vec::new(),
            intent: DecisionIntent::FavorCurrent,
            surface: ObservationSurface::PointOnly,
        },
        VerificationInput {
            frame: FrameId::Hueman,
            flows: Vec::new(),
            glows: Vec::new(),
            intent: DecisionIntent::FavorAura,
            surface: ObservationSurface::PointOnly,
        },
        VerificationInput {
            frame: FrameId::Hueman,
            flows: Vec::new(),
            glows: Vec::new(),
            intent: DecisionIntent::Neutral,
            surface: ObservationSurface::PointOnly,
        },
        VerificationInput {
            frame: FrameId::Hueman,
            flows: Vec::new(),
            glows: Vec::new(),
            intent: DecisionIntent::Neutral,
            surface: ObservationSurface::KernelStraight,
        },
        VerificationInput {
            frame: FrameId::Hueman,
            flows: Vec::new(),
            glows: Vec::new(),
            intent: DecisionIntent::Neutral,
            surface: ObservationSurface::KernelCurved,
        },
        VerificationInput {
            frame: FrameId::Gremlin,
            flows: vec![FlowId::TinkerGrip],
            glows: Vec::new(),
            intent: DecisionIntent::Neutral,
            surface: ObservationSurface::PointOnly,
        },
        VerificationInput {
            frame: FrameId::Pixy,
            flows: Vec::new(),
            glows: vec![GlowId::Confusion],
            intent: DecisionIntent::Neutral,
            surface: ObservationSurface::PointOnly,
        },
    ];

    cases
        .into_iter()
        .map(|input| {
            let point = build_point(input.frame, &input.flows, &input.glows);
            let observation = build_observation(&input, &point);
            let trace = execute_observed_decision(observation.clone())
                .expect("decision should execute")
                .trace()
                .clone();
            (observation, trace)
        })
        .collect()
}

fn corrupt_trace_variants(trace: &DecisionTrace) -> Vec<(String, DecisionTrace)> {
    let mut variants = Vec::new();

    let mut push = |name: &str, mutator: fn(&mut DecisionTrace)| {
        let mut corrupted = trace.clone();
        mutator(&mut corrupted);
        variants.push((name.to_string(), corrupted));
    };

    push("observed_frame", |trace| {
        trace.observation.frame = alt_frame(trace.observation.frame);
    });
    push("observed_flow", |trace| {
        if trace.observation.flows.contains(&FlowId::TinkerGrip) {
            trace
                .observation
                .flows
                .retain(|flow| *flow != FlowId::TinkerGrip);
        } else {
            trace.observation.flows.push(FlowId::TinkerGrip);
        }
    });
    push("observed_glow", |trace| {
        if trace.observation.glows.contains(&GlowId::Confusion) {
            trace
                .observation
                .glows
                .retain(|glow| *glow != GlowId::Confusion);
        } else {
            trace.observation.glows.push(GlowId::Confusion);
        }
    });
    push("intent", |trace| {
        trace.observation.intent = alt_intent(trace.observation.intent);
    });
    push("route_geometry", |trace| {
        trace.observation.route_geometry = Some(alt_geometry(
            trace
                .observation
                .route_geometry
                .unwrap_or(ManagerGeometry::Straight),
        ));
    });
    push("candidate_order", |trace| {
        trace.generation.reverse();
    });
    push("manager_identity", |trace| {
        trace.generation[0].manager = alt_manager(trace.generation[0].manager);
    });
    push("manager_geometry", |trace| {
        trace.generation[0].manager_geometry = alt_geometry(trace.generation[0].manager_geometry);
    });
    push("candidate_orientation", |trace| {
        trace.generation[0].orientation = alt_orientation(trace.generation[0].orientation);
    });
    push("intent_score", |trace| {
        trace.evaluations[0].intent_score += 1;
    });
    push("realized_penalty", |trace| {
        trace.evaluations[0].realized_state_penalty ^= 1;
    });
    push("final_score", |trace| {
        trace.evaluations[0].final_score += 1;
    });
    push("evaluation_reason", |trace| {
        trace.evaluations[0].reason_codes[0] = alt_reason(trace.evaluations[0].reason_codes[0]);
    });
    push("highest_score", |trace| {
        trace.choice.highest_score += 1;
    });
    push("tie_flag", |trace| {
        trace.choice.tie_occurred = !trace.choice.tie_occurred;
    });
    push("tied_candidate_set", |trace| {
        trace.choice.tied_candidates = vec![alt_candidate(trace.choice.chosen_candidate)];
    });
    push("tie_break_reason", |trace| {
        trace.choice.tie_break_reason = alt_tie_reason(trace.choice.tie_break_reason);
    });
    push("geometry_match_evidence", |trace| {
        trace.choice.manager_geometry_matched = !trace.choice.manager_geometry_matched;
    });
    push("chosen_candidate", |trace| {
        trace.choice.chosen_candidate = alt_candidate(trace.choice.chosen_candidate);
    });
    push("recipe_identity", |trace| {
        trace.recipe_bridge.recipe_id = if trace.recipe_bridge.recipe_id == "gremlin_tinker" {
            "pixy_confusion".to_string()
        } else {
            "gremlin_tinker".to_string()
        };
    });
    push("handoff_flag", |trace| {
        trace.recipe_bridge.handed_to_execution_facade =
            !trace.recipe_bridge.handed_to_execution_facade;
    });
    push("miss_kiss", |trace| {
        trace.execution.contact = alt_contact(trace.execution.contact);
    });
    push("landed_frame", |trace| {
        trace.execution.landed_frame = Some(alt_frame(
            trace.execution.landed_frame.unwrap_or(FrameId::Hueman),
        ));
    });
    push("current_prism_delta", |trace| {
        trace.execution.prism_delta = PrismDelta::new(
            trace.execution.prism_delta.body() + 1,
            trace.execution.prism_delta.spirit(),
            trace.execution.prism_delta.mind(),
            trace.execution.prism_delta.soul_interior(),
            trace.execution.prism_delta.soul_exterior(),
        );
    });
    push("flow_addition", |trace| {
        trace.execution.added_flow = vec![FlowId::Stonefold];
    });
    push("glow_addition", |trace| {
        trace.execution.added_glow = vec![GlowId::Projection];
    });
    push("point_squared_flag", |trace| {
        trace.execution.point_squared_produced = !trace.execution.point_squared_produced;
    });

    variants
}

fn render_trace_text(trace: &DecisionTrace) -> String {
    format!(
        "observe:{}|{:?}|{:?}\nchoose:{}|{:?}|{}\nrecipe:{}\nexecute:{:?}|{:?}|{:?}|{:?}|{}\n",
        trace.observation.intent().as_str(),
        trace.observation.frame(),
        trace.observation.route_geometry(),
        trace.choice().tie_break_reason().as_str(),
        trace.choice().chosen_candidate(),
        trace.choice().highest_score(),
        trace.recipe_bridge().recipe_id(),
        trace.execution().contact(),
        trace.execution().landed_frame(),
        trace.execution().added_flow(),
        trace.execution().added_glow(),
        trace.execution().point_squared_produced()
    )
}

fn semantic_input_key(input: &VerificationInput) -> String {
    format!(
        "{:?}|{}|{}|{}|{:?}",
        input.frame,
        join_debug(&input.flows),
        join_debug(&input.glows),
        input.intent.as_str(),
        input.surface
    )
}

fn semantic_trace_key(trace: &DecisionTrace) -> String {
    let mut key = String::new();
    let _ = write!(
        key,
        "{:?}|{}|{:?}|",
        trace.observation().frame(),
        trace.observation().intent().as_str(),
        trace.observation().route_geometry()
    );
    for generated in trace.generation() {
        let _ = write!(
            key,
            "{:?}:{:?}:{:?}:{:?};",
            generated.candidate_id(),
            generated.manager(),
            generated.manager_geometry(),
            generated.orientation()
        );
    }
    for evaluation in trace.evaluations() {
        let _ = write!(
            key,
            "{:?}:{}:{}:{}:{:?};",
            evaluation.candidate_id(),
            evaluation.intent_score(),
            evaluation.realized_state_penalty(),
            evaluation.final_score(),
            evaluation.reason_codes()
        );
    }
    let _ = write!(
        key,
        "choice:{:?}:{:?}:{}:{:?}:{}:{}|recipe:{}|exec:{:?}:{:?}:{:?}:{:?}:{}",
        trace.choice().chosen_candidate(),
        trace.choice().tied_candidates(),
        trace.choice().highest_score(),
        trace.choice().tie_break_reason(),
        trace.choice().manager_geometry_matched(),
        trace.choice().generate_order_resolved(),
        trace.recipe_bridge().recipe_id(),
        trace.execution().contact(),
        trace.execution().landed_frame(),
        trace.execution().prism_delta(),
        trace.execution().added_flow(),
        trace.execution().point_squared_produced()
    );
    key
}

fn measure_stage(
    stage: &'static str,
    iterations: usize,
    trace_size_accumulator: Option<&mut usize>,
    mut body: impl FnMut(),
) -> StageStats {
    let mut samples = Vec::with_capacity(iterations);
    for _ in 0..iterations {
        let start = Instant::now();
        body();
        samples.push(start.elapsed().as_secs_f64() * 1_000_000.0);
    }
    let total = samples.iter().sum::<f64>();
    let mut sorted = samples.clone();
    sorted.sort_by(|left, right| left.partial_cmp(right).expect("finite durations"));
    let mean_us = total / iterations.max(1) as f64;
    let median_us = percentile(&sorted, 0.50);
    let p95_us = percentile(&sorted, 0.95);
    let p99_us = percentile(&sorted, 0.99);
    let variance = samples
        .iter()
        .map(|sample| {
            let diff = sample - mean_us;
            diff * diff
        })
        .sum::<f64>()
        / iterations.max(1) as f64;
    let throughput_per_sec = if total > 0.0 {
        iterations as f64 / (total / 1_000_000.0)
    } else {
        0.0
    };

    StageStats {
        stage,
        iterations,
        mean_us,
        median_us,
        p95_us,
        p99_us,
        stddev_us: variance.sqrt(),
        throughput_per_sec,
        trace_size_bytes: trace_size_accumulator.map(|value| *value / iterations.max(1)),
    }
}

fn percentile(sorted: &[f64], percentile: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    let index = ((sorted.len() - 1) as f64 * percentile).round() as usize;
    sorted[index]
}

fn profile_name(profile: VerificationProfile) -> &'static str {
    match profile {
        VerificationProfile::Fast => "FAST",
        VerificationProfile::Full => "FULL",
        VerificationProfile::Overnight => "OVERNIGHT",
    }
}

fn push_json_field(json: &mut String, key: &str, value: &str, comma: bool) {
    let _ = writeln!(
        json,
        "  \"{}\": \"{}\"{}",
        json_escape(key),
        json_escape(value),
        if comma { "," } else { "" }
    );
}

fn push_json_optional_field(json: &mut String, key: &str, value: Option<&str>, comma: bool) {
    match value {
        Some(value) => push_json_field(json, key, value, comma),
        None => {
            let _ = writeln!(
                json,
                "  \"{}\": null{}",
                json_escape(key),
                if comma { "," } else { "" }
            );
        }
    }
}

fn push_json_u64(json: &mut String, key: &str, value: u64, comma: bool) {
    let _ = writeln!(
        json,
        "  \"{}\": {}{}",
        json_escape(key),
        value,
        if comma { "," } else { "" }
    );
}

fn push_json_usize(json: &mut String, key: &str, value: usize, comma: bool) {
    let _ = writeln!(
        json,
        "  \"{}\": {}{}",
        json_escape(key),
        value,
        if comma { "," } else { "" }
    );
}

fn json_escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}

fn join_debug<T: std::fmt::Debug>(values: &[T]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values
            .iter()
            .map(|value| format!("{value:?}"))
            .collect::<Vec<_>>()
            .join("+")
    }
}

fn alt_frame(frame: FrameId) -> FrameId {
    match frame {
        FrameId::Hueman => FrameId::Gremlin,
        FrameId::Gremlin => FrameId::Pixy,
        _ => FrameId::Hueman,
    }
}

fn alt_intent(intent: DecisionIntent) -> DecisionIntent {
    match intent {
        DecisionIntent::FavorCurrent => DecisionIntent::FavorAura,
        DecisionIntent::FavorAura => DecisionIntent::Neutral,
        DecisionIntent::Neutral => DecisionIntent::FavorCurrent,
    }
}

fn alt_geometry(geometry: ManagerGeometry) -> ManagerGeometry {
    match geometry {
        ManagerGeometry::Straight => ManagerGeometry::Curved,
        ManagerGeometry::Curved => ManagerGeometry::Inverted,
        ManagerGeometry::Inverted => ManagerGeometry::Straight,
    }
}

fn alt_manager(manager: Manager) -> Manager {
    match manager {
        Manager::Hal => Manager::Clouseau,
        Manager::Clouseau => Manager::Cleopatra,
        Manager::Cleopatra => Manager::Hal,
    }
}

fn alt_orientation(orientation: SynthesisOrientation) -> SynthesisOrientation {
    match orientation {
        SynthesisOrientation::Current => SynthesisOrientation::Aura,
        SynthesisOrientation::Aura => SynthesisOrientation::Current,
    }
}

fn alt_candidate(candidate: DecisionCandidateId) -> DecisionCandidateId {
    match candidate {
        DecisionCandidateId::GremlinTinker => DecisionCandidateId::PixyConfusion,
        DecisionCandidateId::PixyConfusion => DecisionCandidateId::GremlinTinker,
    }
}

fn alt_reason(reason: DecisionTraceReasonCode) -> DecisionTraceReasonCode {
    match reason {
        DecisionTraceReasonCode::PreferredCurrentOrientation => {
            DecisionTraceReasonCode::NonPreferredCurrentOrientation
        }
        DecisionTraceReasonCode::NonPreferredCurrentOrientation => {
            DecisionTraceReasonCode::PreferredCurrentOrientation
        }
        DecisionTraceReasonCode::PreferredAuraOrientation => {
            DecisionTraceReasonCode::NonPreferredAuraOrientation
        }
        DecisionTraceReasonCode::NonPreferredAuraOrientation => {
            DecisionTraceReasonCode::PreferredAuraOrientation
        }
        DecisionTraceReasonCode::NeutralBaseScore => DecisionTraceReasonCode::AlreadyCanonicalFrame,
        DecisionTraceReasonCode::AlreadyCanonicalFrame => DecisionTraceReasonCode::NeutralBaseScore,
        DecisionTraceReasonCode::AlreadyKnowsCanonicalFlow => {
            DecisionTraceReasonCode::AlreadyKnowsCanonicalGlow
        }
        DecisionTraceReasonCode::AlreadyKnowsCanonicalGlow => {
            DecisionTraceReasonCode::AlreadyKnowsCanonicalFlow
        }
    }
}

fn alt_tie_reason(reason: DecisionTraceTieBreakReason) -> DecisionTraceTieBreakReason {
    match reason {
        DecisionTraceTieBreakReason::NoTie => DecisionTraceTieBreakReason::CanonicalGenerateOrder,
        DecisionTraceTieBreakReason::ObservedRouteGeometryMatch => {
            DecisionTraceTieBreakReason::CanonicalGenerateOrder
        }
        DecisionTraceTieBreakReason::CanonicalGenerateOrder => {
            DecisionTraceTieBreakReason::ObservedRouteGeometryMatch
        }
    }
}

fn alt_contact(contact: ContactOutcome) -> ContactOutcome {
    match contact {
        ContactOutcome::Miss => ContactOutcome::Kiss,
        ContactOutcome::Kiss => ContactOutcome::Miss,
    }
}

struct StableHasher {
    value: u64,
}

impl StableHasher {
    fn new() -> Self {
        Self {
            value: 0xcbf2_9ce4_8422_2325,
        }
    }

    fn write_str(&mut self, value: &str) {
        for byte in value.as_bytes() {
            self.value ^= u64::from(*byte);
            self.value = self.value.wrapping_mul(0x0000_0100_0000_01B3);
        }
    }

    fn finish_hex(&self) -> String {
        format!("{:016x}", self.value)
    }
}

#[derive(Debug, Clone, Copy)]
struct SyntheticCandidate {
    id: usize,
    geometry: ManagerGeometry,
    base_score: u16,
}

#[derive(Debug, Clone, Copy)]
struct SyntheticEvaluation {
    id: usize,
    geometry: ManagerGeometry,
    score: u16,
}

fn synthetic_geometry_rank(geometry: ManagerGeometry) -> u8 {
    match geometry {
        ManagerGeometry::Inverted => 0,
        ManagerGeometry::Straight => 1,
        ManagerGeometry::Curved => 2,
    }
}

pub const DEFAULT_VERIFICATION_SEED: u64 = 0xC0FF_EE42;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SoakConfig {
    pub duration_seconds: u64,
    pub seed: u64,
    pub report_interval_seconds: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SoakReport {
    pub duration_requested_seconds: u64,
    pub duration_completed_seconds: f64,
    pub seed: u64,
    pub workload_distribution: Vec<(&'static str, u8)>,
    pub workload_counts: BTreeMap<String, u64>,
    pub total_operations: u64,
    pub operations_per_second: f64,
    pub latency_min_us: f64,
    pub latency_mean_us: f64,
    pub latency_p50_us: f64,
    pub latency_p95_us: f64,
    pub latency_p99_us: f64,
    pub latency_max_us: f64,
    pub semantic_hash_accumulator: String,
    pub differential_mismatches: u64,
    pub semantic_hash_mismatches: u64,
    pub replay_false_acceptances: u64,
    pub rollback_failures: u64,
    pub partial_commits: u64,
    pub panic_count: u64,
    pub invariant_failures: u64,
    pub source_state_mutations: u64,
    pub unexplained_decisions: u64,
    pub corrupted_candidate_order: u64,
    pub rss_start_kb: Option<u64>,
    pub rss_end_kb: Option<u64>,
    pub rss_peak_kb: Option<u64>,
    pub fd_start: Option<u64>,
    pub fd_end: Option<u64>,
    pub fd_peak: Option<u64>,
    pub memory_samples: Vec<SoakMemorySample>,
    pub memory_trend: String,
    pub interrupted: bool,
    pub failure_reasons: Vec<String>,
}

impl SoakReport {
    #[must_use]
    pub fn succeeded(&self) -> bool {
        self.invariant_failures == 0
            && self.differential_mismatches == 0
            && self.semantic_hash_mismatches == 0
            && self.replay_false_acceptances == 0
            && self.rollback_failures == 0
            && self.partial_commits == 0
            && self.panic_count == 0
            && self.source_state_mutations == 0
            && self.unexplained_decisions == 0
            && self.corrupted_candidate_order == 0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SoakMemorySample {
    pub elapsed_seconds: u64,
    pub operations: u64,
    pub rss_kb: Option<u64>,
    pub fd_count: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CandidateArrangement {
    Canonical,
    DuplicateCandidate,
    MissingEvaluation,
    DuplicateEvaluation,
    UnknownEvaluation,
    Empty,
}

impl CandidateArrangement {
    fn from_byte(value: u8) -> Self {
        match value % 6 {
            0 => Self::Canonical,
            1 => Self::DuplicateCandidate,
            2 => Self::MissingEvaluation,
            3 => Self::DuplicateEvaluation,
            4 => Self::UnknownEvaluation,
            _ => Self::Empty,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SoakWorkload {
    DecisionEvaluation,
    RouteAwareTie,
    RealizedStateSwitch,
    ValidReplay,
    CorruptedReplay,
    Differential,
    CanonicalExecution,
    Rollback,
    SemanticHash,
}

impl SoakWorkload {
    fn as_str(self) -> &'static str {
        match self {
            Self::DecisionEvaluation => "decision_evaluation",
            Self::RouteAwareTie => "route_aware_tie",
            Self::RealizedStateSwitch => "realized_state_switch",
            Self::ValidReplay => "valid_replay",
            Self::CorruptedReplay => "corrupted_replay",
            Self::Differential => "differential",
            Self::CanonicalExecution => "canonical_execution",
            Self::Rollback => "rollback",
            Self::SemanticHash => "semantic_hash",
        }
    }
}

#[derive(Debug, Clone)]
struct LatencyReservoir {
    seen: u64,
    total_us: f64,
    min_us: f64,
    max_us: f64,
    samples: Vec<f64>,
}

impl LatencyReservoir {
    fn new() -> Self {
        Self {
            seen: 0,
            total_us: 0.0,
            min_us: f64::MAX,
            max_us: 0.0,
            samples: Vec::with_capacity(4_096),
        }
    }

    fn record(&mut self, sample_us: f64, rng: &mut DeterministicRng) {
        self.seen += 1;
        self.total_us += sample_us;
        self.min_us = self.min_us.min(sample_us);
        self.max_us = self.max_us.max(sample_us);
        if self.samples.len() < 4_096 {
            self.samples.push(sample_us);
            return;
        }

        let slot = rng.next_u64() % self.seen;
        if let Ok(index) = usize::try_from(slot)
            && index < self.samples.len()
        {
            self.samples[index] = sample_us;
        }
    }

    fn min_us(&self) -> f64 {
        if self.seen == 0 { 0.0 } else { self.min_us }
    }

    fn mean_us(&self) -> f64 {
        if self.seen == 0 {
            0.0
        } else {
            self.total_us / self.seen as f64
        }
    }

    fn max_us(&self) -> f64 {
        if self.seen == 0 { 0.0 } else { self.max_us }
    }

    fn percentile(&self, value: f64) -> f64 {
        percentile_samples(&self.samples, value)
    }
}

pub fn fuzz_decision_input_bytes(data: &[u8]) {
    let input = decode_fuzz_input(data);
    let point = build_point(input.frame, &input.flows, &input.glows);
    let before_point = point.clone();
    let observation = build_observation(&input, &point);
    assert_canonical_candidate_order(&observation);
    assert_candidate_arrangement_behavior(
        &observation,
        CandidateArrangement::from_byte(byte(data, 5)),
    );

    let first =
        execute_observed_decision(observation.clone()).expect("decision input should execute");
    let second = execute_observed_decision(observation.clone())
        .expect("decision input should remain deterministic");

    assert_eq!(
        first.chosen().candidate().candidate_id(),
        second.chosen().candidate().candidate_id()
    );
    assert_eq!(first.trace(), second.trace());
    assert_eq!(point, before_point);
    assert_eq!(
        first.observation().frame_state(),
        before_point.frame_state()
    );
    assert!(
        first
            .candidates()
            .iter()
            .any(|candidate| candidate.candidate_id() == first.chosen().candidate().candidate_id())
    );
    assert_eq!(
        resolve_candidate_recipe(first.chosen().candidate().candidate_id()).recipe_id(),
        first.recipe().recipe_id()
    );
    assert!(first.trace().recipe_bridge().handed_to_execution_facade());

    if observation.route_geometry().is_some() {
        let strict_winner = has_strict_winner(first.trace());
        let point_only = execute_observed_decision(observe_decision(&point, input.intent))
            .expect("point-only comparison should execute");
        if strict_winner {
            assert_eq!(
                first.chosen().candidate().candidate_id(),
                point_only.chosen().candidate().candidate_id()
            );
        }
    }
}

pub fn fuzz_decision_trace_replay_bytes(data: &[u8]) {
    let traces = canonical_traces();
    let index = usize::from(byte(data, 0)) % traces.len();
    let (observation, trace) = traces[index].clone();
    let before = observation.point().clone();

    if byte(data, 1) & 1 == 0 {
        replay_trace_for_observation(observation.clone(), &trace)
            .expect("unchanged trace should replay");
        replay_trace_for_observation(observation.clone(), &trace)
            .expect("unchanged trace should replay deterministically");
        assert_eq!(*observation.point(), before);
        return;
    }

    let corruptions = fuzz_trace_corruptions(&trace);
    let (name, corrupted) = &corruptions[usize::from(byte(data, 2)) % corruptions.len()];
    let replay = replay_trace_for_observation(observation.clone(), corrupted);
    assert!(replay.is_err(), "corruption `{name}` was falsely accepted");
    assert_eq!(*observation.point(), before);
}

pub fn fuzz_recipe_compiler_bytes(data: &[u8]) {
    let recipe = decode_fuzz_recipe(data);
    let start = Point::origin();
    let before = start.clone();
    let compiled = compile_recipe(&recipe);
    assert_eq!(compiled, compile_recipe(&recipe));
    assert_eq!(start, before);

    let Ok(scripts) = compiled else {
        return;
    };
    assert_eq!(
        scripts,
        compile_recipe(&recipe).expect("recipe compile should remain deterministic")
    );

    let exact_canonical = exact_canonical_recipe(&recipe);
    if let Some(recipe) = exact_canonical {
        let aim = build_aim(&recipe, scripts.clone());
        let cuts = landing_fault_cuts(scripts.len());
        if !cuts.is_empty() && byte(data, 12) & 1 == 0 {
            let cut = cuts[usize::from(byte(data, 13)) % cuts.len()];
            let start_state = FrameState::origin();
            let before_state = start_state.clone();
            let failed = apply_kiss_with_fault(&start_state, &aim, Some(cut));
            assert!(matches!(failed, Err(LandingFaultError::Injected(_))));
            assert_eq!(start_state, before_state);
            let success = apply_kiss_with_fault(&start_state, &aim, None)
                .expect("canonical landing should recover");
            assert_eq!(success.contact(), ContactOutcome::Kiss);
            return;
        }

        let execution =
            execute_synthesis_recipe(&start, &recipe).expect("canonical recipe should execute");
        assert_eq!(start, before);
        assert_eq!(execution.contact(), ContactOutcome::Kiss);
        return;
    }

    assert_eq!(start, before);
}

pub fn fuzz_snapshot_boundary_bytes(data: &[u8]) {
    let straight = canonical_snapshot(false);
    let curved = canonical_snapshot(true);
    match byte(data, 0) % 4 {
        0 => {
            let boundary =
                SnapshotBoundary::parse(&straight).expect("straight snapshot should parse");
            assert_eq!(
                build_snapshot_boundary_output(&boundary),
                build_snapshot_boundary_output(&boundary)
            );
        }
        1 => {
            let boundary = SnapshotBoundary::parse(&curved).expect("curved snapshot should parse");
            assert_eq!(
                build_snapshot_boundary_output(&boundary),
                build_snapshot_boundary_output(&boundary)
            );
        }
        2 => {
            let corrupted = corrupt_snapshot(&straight, byte(data, 1), byte(data, 2));
            assert!(
                SnapshotBoundary::parse(&corrupted).is_err(),
                "mutated snapshot was accepted: {corrupted}"
            );
        }
        _ => {
            let payload = String::from_utf8_lossy(&data[..data.len().min(256)]).into_owned();
            let _ = SnapshotBoundary::parse(&payload);
        }
    }
}

pub fn run_soak(config: SoakConfig) -> SoakReport {
    run_soak_with_checkpoint(config, &mut |_| {})
}

pub fn run_soak_with_checkpoint(
    config: SoakConfig,
    checkpoint: &mut dyn FnMut(&SoakReport),
) -> SoakReport {
    let duration_seconds = config.duration_seconds.max(1);
    let report_interval_seconds = config.report_interval_seconds.max(1);
    let distribution = soak_distribution();
    let start = Instant::now();
    let mut rng = DeterministicRng::new(config.seed ^ 0x50A6_1234);
    let mut semantic_hasher = StableHasher::new();
    let mut latency = LatencyReservoir::new();
    let mut workload_counts = BTreeMap::new();
    let mut total_operations = 0u64;
    let mut differential_mismatches = 0u64;
    let mut semantic_hash_mismatches = 0u64;
    let mut replay_false_acceptances = 0u64;
    let mut rollback_failures = 0u64;
    let mut partial_commits = 0u64;
    let mut panic_count = 0u64;
    let mut invariant_failures = 0u64;
    let mut source_state_mutations = 0u64;
    let mut unexplained_decisions = 0u64;
    let mut corrupted_candidate_order = 0u64;
    let mut failure_reasons = Vec::new();
    let mut memory_samples = Vec::new();
    let rss_start = current_rss_kb();
    let fd_start = current_fd_count();
    let mut rss_peak = rss_start;
    let mut fd_peak = fd_start;
    let mut next_report = Duration::from_secs(report_interval_seconds);

    while start.elapsed() < Duration::from_secs(duration_seconds) {
        let workload = select_soak_workload(&mut rng);
        let op_start = Instant::now();
        let outcome = catch_unwind(AssertUnwindSafe(|| {
            execute_soak_workload(workload, &mut rng)
        }));
        let elapsed_us = op_start.elapsed().as_secs_f64() * 1_000_000.0;
        latency.record(elapsed_us, &mut rng);
        total_operations += 1;
        *workload_counts
            .entry(workload.as_str().to_string())
            .or_insert(0) += 1;

        match outcome {
            Ok(Ok(summary)) => {
                semantic_hasher.write_str(&summary);
            }
            Ok(Err(error)) => {
                invariant_failures += 1;
                match error.kind {
                    SoakFailureKind::DifferentialMismatch => differential_mismatches += 1,
                    SoakFailureKind::SemanticHashMismatch => semantic_hash_mismatches += 1,
                    SoakFailureKind::ReplayFalseAcceptance => replay_false_acceptances += 1,
                    SoakFailureKind::RollbackFailure => rollback_failures += 1,
                    SoakFailureKind::PartialCommit => partial_commits += 1,
                    SoakFailureKind::SourceStateMutation => source_state_mutations += 1,
                    SoakFailureKind::UnexplainedDecision => unexplained_decisions += 1,
                    SoakFailureKind::CandidateOrderCorruption => corrupted_candidate_order += 1,
                    SoakFailureKind::InvariantFailure => {}
                }
                failure_reasons.push(error.message);
                break;
            }
            Err(_) => {
                panic_count += 1;
                invariant_failures += 1;
                failure_reasons.push(format!(
                    "panic during soak workload `{}`",
                    workload.as_str()
                ));
                break;
            }
        }

        if start.elapsed() >= next_report {
            let sample = SoakMemorySample {
                elapsed_seconds: start.elapsed().as_secs(),
                operations: total_operations,
                rss_kb: current_rss_kb(),
                fd_count: current_fd_count(),
            };
            rss_peak = option_max(rss_peak, sample.rss_kb);
            fd_peak = option_max(fd_peak, sample.fd_count);
            let sample_rss = sample.rss_kb;
            let sample_fd = sample.fd_count;
            memory_samples.push(sample);
            let checkpoint_report = build_soak_report(
                config.seed,
                duration_seconds,
                &distribution,
                start.elapsed().as_secs_f64(),
                &workload_counts,
                total_operations,
                &latency,
                &semantic_hasher,
                differential_mismatches,
                semantic_hash_mismatches,
                replay_false_acceptances,
                rollback_failures,
                partial_commits,
                panic_count,
                invariant_failures,
                source_state_mutations,
                unexplained_decisions,
                corrupted_candidate_order,
                rss_start,
                sample_rss,
                rss_peak,
                fd_start,
                sample_fd,
                fd_peak,
                memory_samples.clone(),
                false,
                failure_reasons.clone(),
            );
            checkpoint(&checkpoint_report);
            next_report += Duration::from_secs(report_interval_seconds);
        }
    }

    let rss_end = current_rss_kb();
    let fd_end = current_fd_count();
    rss_peak = option_max(rss_peak, rss_end);
    fd_peak = option_max(fd_peak, fd_end);
    if memory_samples.last().map(|sample| sample.elapsed_seconds) != Some(start.elapsed().as_secs())
    {
        memory_samples.push(SoakMemorySample {
            elapsed_seconds: start.elapsed().as_secs(),
            operations: total_operations,
            rss_kb: rss_end,
            fd_count: fd_end,
        });
    }

    build_soak_report(
        config.seed,
        duration_seconds,
        &distribution,
        start.elapsed().as_secs_f64(),
        &workload_counts,
        total_operations,
        &latency,
        &semantic_hasher,
        differential_mismatches,
        semantic_hash_mismatches,
        replay_false_acceptances,
        rollback_failures,
        partial_commits,
        panic_count,
        invariant_failures,
        source_state_mutations,
        unexplained_decisions,
        corrupted_candidate_order,
        rss_start,
        rss_end,
        rss_peak,
        fd_start,
        fd_end,
        fd_peak,
        memory_samples,
        false,
        failure_reasons,
    )
}

pub fn soak_report_to_json(report: &SoakReport) -> String {
    let mut json = String::new();
    json.push_str("{\n");
    push_json_u64(
        &mut json,
        "duration_requested_seconds",
        report.duration_requested_seconds,
        true,
    );
    let _ = writeln!(
        json,
        "  \"duration_completed_seconds\": {:.3},",
        report.duration_completed_seconds
    );
    push_json_u64(&mut json, "seed", report.seed, true);
    let _ = writeln!(json, "  \"total_operations\": {},", report.total_operations);
    let _ = writeln!(
        json,
        "  \"operations_per_second\": {:.3},",
        report.operations_per_second
    );
    let _ = writeln!(
        json,
        "  \"semantic_hash_accumulator\": \"{}\",",
        json_escape(&report.semantic_hash_accumulator)
    );
    json.push_str("  \"workload_counts\": {\n");
    for (index, (name, count)) in report.workload_counts.iter().enumerate() {
        let _ = writeln!(
            json,
            "    \"{}\": {}{}",
            json_escape(name),
            count,
            if index + 1 == report.workload_counts.len() {
                ""
            } else {
                ","
            }
        );
    }
    json.push_str("  },\n");
    json.push_str("  \"memory_samples\": [\n");
    for (index, sample) in report.memory_samples.iter().enumerate() {
        let _ = writeln!(
            json,
            "    {{\"elapsed_seconds\":{},\"operations\":{},\"rss_kb\":{},\"fd_count\":{}}}{}",
            sample.elapsed_seconds,
            sample.operations,
            optional_u64_json(sample.rss_kb),
            optional_u64_json(sample.fd_count),
            if index + 1 == report.memory_samples.len() {
                ""
            } else {
                ","
            }
        );
    }
    json.push_str("  ],\n");
    let _ = writeln!(
        json,
        "  \"latency\": {{\"min_us\": {:.3}, \"mean_us\": {:.3}, \"p50_us\": {:.3}, \"p95_us\": {:.3}, \"p99_us\": {:.3}, \"max_us\": {:.3}}},",
        report.latency_min_us,
        report.latency_mean_us,
        report.latency_p50_us,
        report.latency_p95_us,
        report.latency_p99_us,
        report.latency_max_us
    );
    let _ = writeln!(
        json,
        "  \"counters\": {{\"differential_mismatches\": {}, \"semantic_hash_mismatches\": {}, \"replay_false_acceptances\": {}, \"rollback_failures\": {}, \"partial_commits\": {}, \"panic_count\": {}, \"invariant_failures\": {}, \"source_state_mutations\": {}, \"unexplained_decisions\": {}, \"corrupted_candidate_order\": {}}},",
        report.differential_mismatches,
        report.semantic_hash_mismatches,
        report.replay_false_acceptances,
        report.rollback_failures,
        report.partial_commits,
        report.panic_count,
        report.invariant_failures,
        report.source_state_mutations,
        report.unexplained_decisions,
        report.corrupted_candidate_order
    );
    let _ = writeln!(
        json,
        "  \"rss_start_kb\": {},",
        optional_u64_json(report.rss_start_kb)
    );
    let _ = writeln!(
        json,
        "  \"rss_end_kb\": {},",
        optional_u64_json(report.rss_end_kb)
    );
    let _ = writeln!(
        json,
        "  \"rss_peak_kb\": {},",
        optional_u64_json(report.rss_peak_kb)
    );
    let _ = writeln!(
        json,
        "  \"fd_start\": {},",
        optional_u64_json(report.fd_start)
    );
    let _ = writeln!(json, "  \"fd_end\": {},", optional_u64_json(report.fd_end));
    let _ = writeln!(
        json,
        "  \"fd_peak\": {},",
        optional_u64_json(report.fd_peak)
    );
    push_json_field(&mut json, "memory_trend", &report.memory_trend, true);
    let _ = writeln!(
        json,
        "  \"interrupted\": {},",
        if report.interrupted { "true" } else { "false" }
    );
    json.push_str("  \"failure_reasons\": [\n");
    for (index, reason) in report.failure_reasons.iter().enumerate() {
        let _ = writeln!(
            json,
            "    \"{}\"{}",
            json_escape(reason),
            if index + 1 == report.failure_reasons.len() {
                ""
            } else {
                ","
            }
        );
    }
    json.push_str("  ]\n");
    json.push('}');
    json
}

pub fn soak_report_to_markdown(report: &SoakReport) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "# Hollow Grove Soak");
    let _ = writeln!(out);
    let _ = writeln!(out, "- seed: `{}`", report.seed);
    let _ = writeln!(
        out,
        "- duration: requested `{}`s / completed `{:.2}`s",
        report.duration_requested_seconds, report.duration_completed_seconds
    );
    let _ = writeln!(
        out,
        "- operations: `{}` total at `{:.2}` ops/s",
        report.total_operations, report.operations_per_second
    );
    let _ = writeln!(
        out,
        "- latency us: min `{:.2}`, mean `{:.2}`, p50 `{:.2}`, p95 `{:.2}`, p99 `{:.2}`, max `{:.2}`",
        report.latency_min_us,
        report.latency_mean_us,
        report.latency_p50_us,
        report.latency_p95_us,
        report.latency_p99_us,
        report.latency_max_us
    );
    let _ = writeln!(
        out,
        "- semantic hash accumulator: `{}`",
        report.semantic_hash_accumulator
    );
    let _ = writeln!(
        out,
        "- counters: invariant `{}`, differential `{}`, semantic hash `{}`, replay false acceptances `{}`, rollback `{}`, partial commits `{}`, panics `{}`",
        report.invariant_failures,
        report.differential_mismatches,
        report.semantic_hash_mismatches,
        report.replay_false_acceptances,
        report.rollback_failures,
        report.partial_commits,
        report.panic_count
    );
    let _ = writeln!(
        out,
        "- RSS kB: start `{}`, end `{}`, peak `{}`",
        optional_u64_display(report.rss_start_kb),
        optional_u64_display(report.rss_end_kb),
        optional_u64_display(report.rss_peak_kb)
    );
    let _ = writeln!(
        out,
        "- FD count: start `{}`, end `{}`, peak `{}`",
        optional_u64_display(report.fd_start),
        optional_u64_display(report.fd_end),
        optional_u64_display(report.fd_peak)
    );
    let _ = writeln!(out, "- memory trend: `{}`", report.memory_trend);
    let _ = writeln!(out);
    let _ = writeln!(out, "## Workloads");
    for (name, weight) in &report.workload_distribution {
        let count = report.workload_counts.get(*name).copied().unwrap_or(0);
        let _ = writeln!(out, "- `{name}`: weight `{weight}%`, operations `{count}`");
    }
    if !report.failure_reasons.is_empty() {
        let _ = writeln!(out);
        let _ = writeln!(out, "## Failures");
        for reason in &report.failure_reasons {
            let _ = writeln!(out, "- {reason}");
        }
    }
    out
}

fn decode_fuzz_input(data: &[u8]) -> VerificationInput {
    let frame = FRAMES[usize::from(byte(data, 0)) % FRAMES.len()];
    let flow_mask = byte(data, 1);
    let glow_mask = byte(data, 2);
    let intent = INTENTS[usize::from(byte(data, 3)) % INTENTS.len()];
    let surface = match byte(data, 4) % 4 {
        0 => ObservationSurface::PointOnly,
        1 => ObservationSurface::KernelStraight,
        2 => ObservationSurface::KernelCurved,
        _ => ObservationSurface::SyntheticInverted,
    };

    VerificationInput {
        frame,
        flows: decode_subset(&FLOWS, flow_mask),
        glows: decode_subset(&GLOWS, glow_mask),
        intent,
        surface,
    }
}

fn decode_fuzz_recipe(data: &[u8]) -> SynthesisRecipe {
    match byte(data, 6) % 4 {
        0 => resolve_candidate_recipe(DecisionCandidateId::GremlinTinker),
        1 => resolve_candidate_recipe(DecisionCandidateId::PixyConfusion),
        _ => {
            let id = bounded_ascii(data, 16, 7);
            let display_name = bounded_ascii(data, 16, 32);
            let intent_count = usize::from(byte(data, 8) % 4);
            let mut intents = Vec::with_capacity(intent_count);
            for index in 0..intent_count {
                intents.push(recipe_intent_from_bytes(data, 48 + index * 4));
            }
            SynthesisRecipe::new(id, display_name, intents)
        }
    }
}

fn recipe_intent_from_bytes(data: &[u8], start: usize) -> crate::RecipeIntent {
    match byte(data, start) % 4 {
        0 => crate::RecipeIntent::ModifyPrism(PrismDelta::new(
            i16::from(byte(data, start + 1) as i8),
            i16::from(byte(data, start + 2) as i8),
            i16::from(byte(data, start + 3) as i8),
            0,
            0,
        )),
        1 => {
            crate::RecipeIntent::LearnFlow(FLOWS[usize::from(byte(data, start + 1)) % FLOWS.len()])
        }
        2 => {
            crate::RecipeIntent::LearnGlow(GLOWS[usize::from(byte(data, start + 1)) % GLOWS.len()])
        }
        _ => crate::RecipeIntent::ChangeFrame(
            FRAMES[usize::from(byte(data, start + 1)) % FRAMES.len()],
        ),
    }
}

fn exact_canonical_recipe(recipe: &SynthesisRecipe) -> Option<SynthesisRecipe> {
    let gremlin = resolve_candidate_recipe(DecisionCandidateId::GremlinTinker);
    if recipe == &gremlin {
        return Some(gremlin);
    }

    let pixy = resolve_candidate_recipe(DecisionCandidateId::PixyConfusion);
    if recipe == &pixy {
        return Some(pixy);
    }

    None
}

fn canonical_snapshot(curved: bool) -> String {
    let kernel_pass = if curved {
        run_kernel_cycle_with_input(
            crate::Symptom::origin(),
            KernelInput {
                routing: crate::PlebMetaInput {
                    exterior_shape: ExteriorShape::Curved,
                    pleb_mode: crate::Mode::Pathos,
                    meta_mode: crate::Mode::Logos,
                },
            },
        )
    } else {
        run_kernel_cycle_with_input(crate::Symptom::origin(), KernelInput::default())
    };
    build_snapshot_output(&kernel_pass)
}

fn corrupt_snapshot(snapshot: &str, variant: u8, payload: u8) -> String {
    match variant % 7 {
        0 => replace_first_of(
            snapshot,
            &[
                "  \"grove_seam_route\": \"PlebExterior\",\n",
                "  \"grove_seam_route\": \"MetaExterior\",\n",
            ],
            "",
        ),
        1 => replace_first_of(
            snapshot,
            &["PlebExterior", "MetaExterior"],
            "BrokenExterior",
        ),
        2 => snapshot.replace(
            "\"landed_point\": \"Point²\"",
            "\"landed_point\": \"PointX\"",
        ),
        3 => replace_first_of(
            snapshot,
            &[
                "  \"landing_route\": \"BlepArrival\"",
                "  \"landing_route\": \"AtemArrival\"",
            ],
            "  \"landing_route\": \"BlepArrival\",\n  \"landing_route\": \"AtemArrival\"",
        ),
        4 => snapshot.replace("Point² (Landed Point)", "PointX (Landed Point)"),
        5 => {
            let oversized = "X".repeat(usize::from(payload) + 64);
            snapshot.replace(
                "\"canonical_witness\": \"",
                &format!("\"canonical_witness\": \"{oversized}"),
            )
        }
        _ => snapshot.replacen("{\n", "{\n  \"schema_version\": \"2\",\n", 1),
    }
}

fn replace_first_of(snapshot: &str, needles: &[&str], replacement: &str) -> String {
    for needle in needles {
        if snapshot.contains(needle) {
            return snapshot.replacen(needle, replacement, 1);
        }
    }
    snapshot.to_string()
}

fn fuzz_trace_corruptions(trace: &DecisionTrace) -> Vec<(String, DecisionTrace)> {
    let mut variants = corrupt_trace_variants(trace);
    let mut duplicate_candidate = trace.clone();
    duplicate_candidate.generation[1].candidate_id = duplicate_candidate.generation[0].candidate_id;
    variants.push((
        "duplicate_candidate_identity".to_string(),
        duplicate_candidate,
    ));

    let mut contradictory_scores = trace.clone();
    contradictory_scores.evaluations[0].intent_score += 1;
    variants.push(("contradictory_scores".to_string(), contradictory_scores));

    variants
}

fn assert_canonical_candidate_order(observation: &DecisionObservation) {
    assert!(canonical_candidate_order_matches(observation));
}

fn canonical_candidate_order_matches(observation: &DecisionObservation) -> bool {
    let candidates = generate_decision_candidates(observation);
    candidates
        .iter()
        .map(|candidate| candidate.candidate_id())
        .collect::<Vec<_>>()
        == expected_candidate_order()
}

fn expected_candidate_order() -> Vec<DecisionCandidateId> {
    vec![
        DecisionCandidateId::GremlinTinker,
        DecisionCandidateId::PixyConfusion,
    ]
}

fn assert_candidate_arrangement_behavior(
    observation: &DecisionObservation,
    arrangement: CandidateArrangement,
) {
    let canonical = generate_decision_candidates(observation);
    let evaluations = canonical
        .iter()
        .map(|candidate| crate::evaluate_decision_candidate(observation, candidate))
        .collect::<Vec<_>>();

    match arrangement {
        CandidateArrangement::Canonical => {
            let chosen =
                crate::choose_decision_for_observation(Some(observation), &canonical, &evaluations)
                    .expect("canonical candidate arrangement should choose");
            assert!(
                canonical
                    .iter()
                    .any(|candidate| candidate.candidate_id() == chosen.candidate().candidate_id())
            );
        }
        CandidateArrangement::DuplicateCandidate => {
            let candidates = vec![canonical[0], canonical[0], canonical[1]];
            let error = crate::choose_decision_for_observation(
                Some(observation),
                &candidates,
                &evaluations,
            )
            .expect_err("duplicate candidate ids should fail");
            assert!(matches!(
                error,
                crate::DecisionChooseError::DuplicateCandidate(DecisionCandidateId::GremlinTinker)
            ));
        }
        CandidateArrangement::MissingEvaluation => {
            let error = crate::choose_decision_for_observation(
                Some(observation),
                &canonical,
                &evaluations[..1],
            )
            .expect_err("missing evaluation should fail");
            assert!(matches!(
                error,
                crate::DecisionChooseError::MissingEvaluation(DecisionCandidateId::PixyConfusion)
            ));
        }
        CandidateArrangement::DuplicateEvaluation => {
            let duplicate = vec![evaluations[0], evaluations[0], evaluations[1]];
            let error =
                crate::choose_decision_for_observation(Some(observation), &canonical, &duplicate)
                    .expect_err("duplicate evaluation should fail");
            assert!(matches!(
                error,
                crate::DecisionChooseError::DuplicateEvaluation(DecisionCandidateId::GremlinTinker)
            ));
        }
        CandidateArrangement::UnknownEvaluation => {
            let error = crate::choose_decision_for_observation(
                Some(observation),
                &canonical[..1],
                &evaluations,
            )
            .expect_err("unknown evaluation should fail");
            assert!(matches!(
                error,
                crate::DecisionChooseError::UnknownEvaluation(DecisionCandidateId::PixyConfusion)
            ));
        }
        CandidateArrangement::Empty => {
            let error = crate::choose_decision_for_observation(Some(observation), &[], &[])
                .expect_err("empty candidate set should fail");
            assert_eq!(error, crate::DecisionChooseError::NoCandidates);
        }
    }
}

fn has_strict_winner(trace: &DecisionTrace) -> bool {
    let highest = trace
        .evaluations()
        .iter()
        .map(DecisionEvaluationTrace::final_score)
        .max()
        .unwrap_or(0);
    trace
        .evaluations()
        .iter()
        .filter(|evaluation| evaluation.final_score() == highest)
        .count()
        == 1
}

#[derive(Debug, Clone)]
struct SoakError {
    kind: SoakFailureKind,
    message: String,
}

#[derive(Debug, Clone, Copy)]
enum SoakFailureKind {
    DifferentialMismatch,
    SemanticHashMismatch,
    ReplayFalseAcceptance,
    RollbackFailure,
    PartialCommit,
    SourceStateMutation,
    UnexplainedDecision,
    CandidateOrderCorruption,
    InvariantFailure,
}

fn execute_soak_workload(
    workload: SoakWorkload,
    rng: &mut DeterministicRng,
) -> Result<String, SoakError> {
    match workload {
        SoakWorkload::DecisionEvaluation => soak_decision_evaluation(rng),
        SoakWorkload::RouteAwareTie => soak_route_aware_tie(rng),
        SoakWorkload::RealizedStateSwitch => soak_realized_state_switch(rng),
        SoakWorkload::ValidReplay => soak_valid_replay(rng),
        SoakWorkload::CorruptedReplay => soak_corrupted_replay(rng),
        SoakWorkload::Differential => soak_differential(rng),
        SoakWorkload::CanonicalExecution => soak_canonical_execution(rng),
        SoakWorkload::Rollback => soak_rollback(rng),
        SoakWorkload::SemanticHash => soak_semantic_hash(rng),
    }
}

fn soak_decision_evaluation(rng: &mut DeterministicRng) -> Result<String, SoakError> {
    let input = random_input(rng);
    let point = build_point(input.frame, &input.flows, &input.glows);
    let before = point.clone();
    let observation = build_observation(&input, &point);
    if !canonical_candidate_order_matches(&observation) {
        return Err(soak_error(
            SoakFailureKind::CandidateOrderCorruption,
            "generate_decision_candidates no longer returns canonical order",
        ));
    }
    let execution = execute_observed_decision(observation)
        .map_err(|error| invariant_error(format!("{error:?}")))?;
    if point != before {
        return Err(soak_error(
            SoakFailureKind::SourceStateMutation,
            "decision evaluation mutated the source point",
        ));
    }
    if execution
        .trace()
        .choice()
        .chosen_candidate()
        .as_str()
        .is_empty()
    {
        return Err(soak_error(
            SoakFailureKind::UnexplainedDecision,
            "decision evaluation produced an empty candidate id",
        ));
    }
    Ok(semantic_trace_key(execution.trace()))
}

fn soak_route_aware_tie(rng: &mut DeterministicRng) -> Result<String, SoakError> {
    let surface = match rng.next_usize(3) {
        0 => ObservationSurface::KernelStraight,
        1 => ObservationSurface::KernelCurved,
        _ => ObservationSurface::SyntheticInverted,
    };
    let point = build_point(FrameId::Hueman, &[], &[]);
    let input = VerificationInput {
        frame: FrameId::Hueman,
        flows: Vec::new(),
        glows: Vec::new(),
        intent: DecisionIntent::Neutral,
        surface,
    };
    let execution = execute_observed_decision(build_observation(&input, &point))
        .map_err(|error| invariant_error(format!("{error:?}")))?;
    let expected = match surface {
        ObservationSurface::KernelStraight => DecisionCandidateId::GremlinTinker,
        ObservationSurface::KernelCurved => DecisionCandidateId::PixyConfusion,
        ObservationSurface::SyntheticInverted => DecisionCandidateId::GremlinTinker,
        ObservationSurface::PointOnly => DecisionCandidateId::GremlinTinker,
    };
    if execution.chosen().candidate().candidate_id() != expected {
        return Err(soak_error(
            SoakFailureKind::InvariantFailure,
            format!(
                "route-aware tie chose {:?} instead of {:?}",
                execution.chosen().candidate().candidate_id(),
                expected
            ),
        ));
    }
    Ok(trace_shape_key(execution.trace()))
}

fn soak_realized_state_switch(rng: &mut DeterministicRng) -> Result<String, SoakError> {
    let (frame, flows, glows, expected) = if rng.next_bool() {
        (
            FrameId::Gremlin,
            vec![FlowId::TinkerGrip],
            Vec::new(),
            DecisionCandidateId::PixyConfusion,
        )
    } else {
        (
            FrameId::Pixy,
            Vec::new(),
            vec![GlowId::Confusion],
            DecisionCandidateId::GremlinTinker,
        )
    };
    let point = build_point(frame, &flows, &glows);
    let execution = execute_observed_decision(observe_decision(&point, DecisionIntent::Neutral))
        .map_err(|error| invariant_error(format!("{error:?}")))?;
    if execution.chosen().candidate().candidate_id() != expected {
        return Err(soak_error(
            SoakFailureKind::InvariantFailure,
            "realized-state switch did not change the neutral winner",
        ));
    }
    Ok(semantic_trace_key(execution.trace()))
}

fn soak_valid_replay(rng: &mut DeterministicRng) -> Result<String, SoakError> {
    let traces = canonical_traces();
    let (observation, trace) = traces[rng.next_usize(traces.len())].clone();
    replay_trace_for_observation(observation, &trace)
        .map_err(|error| invariant_error(format!("valid replay failed: {error:?}")))?;
    Ok(semantic_trace_key(&trace))
}

fn soak_corrupted_replay(rng: &mut DeterministicRng) -> Result<String, SoakError> {
    let traces = canonical_traces();
    let (observation, trace) = traces[rng.next_usize(traces.len())].clone();
    let corruptions = fuzz_trace_corruptions(&trace);
    let (name, corrupted) = &corruptions[rng.next_usize(corruptions.len())];
    if replay_trace_for_observation(observation, corrupted).is_ok() {
        return Err(soak_error(
            SoakFailureKind::ReplayFalseAcceptance,
            format!("corrupted replay was accepted for `{name}`"),
        ));
    }
    Ok(name.clone())
}

fn soak_differential(rng: &mut DeterministicRng) -> Result<String, SoakError> {
    let input = random_input(rng);
    let point = build_point(input.frame, &input.flows, &input.glows);
    let observation = build_observation(&input, &point);
    let plan =
        build_decision_plan(observation).map_err(|error| invariant_error(format!("{error:?}")))?;
    let trace = build_decision_trace_without_execution(&plan);
    let reference = reference_decision(&plan.observation);
    if trace.generation() != reference.generation
        || trace.evaluations() != reference.evaluations
        || trace.choice() != &reference.choice
        || trace.recipe_bridge() != &reference.recipe_bridge
    {
        return Err(soak_error(
            SoakFailureKind::DifferentialMismatch,
            "production and reference decision logic diverged",
        ));
    }
    Ok(semantic_trace_key(&trace))
}

fn soak_canonical_execution(rng: &mut DeterministicRng) -> Result<String, SoakError> {
    if rng.next_u64().is_multiple_of(3) {
        let recipe = resolve_candidate_recipe(DecisionCandidateId::GremlinTinker);
        let scripts = compile_recipe(&recipe).expect("canonical recipe should compile");
        let aim = build_aim(&recipe, scripts);
        let miss = land_contact(Point::origin().frame_state(), &aim, ContactOutcome::Miss)
            .map_err(|error| invariant_error(format!("{error:?}")))?;
        if miss.contact() != ContactOutcome::Miss {
            return Err(soak_error(
                SoakFailureKind::InvariantFailure,
                "miss workload did not preserve a miss outcome",
            ));
        }
        return Ok("miss".to_string());
    }

    let recipe = if rng.next_bool() {
        resolve_candidate_recipe(DecisionCandidateId::GremlinTinker)
    } else {
        resolve_candidate_recipe(DecisionCandidateId::PixyConfusion)
    };
    let execution = execute_synthesis_recipe(&Point::origin(), &recipe)
        .map_err(|error| invariant_error(format!("{error:?}")))?;
    if execution.contact() != ContactOutcome::Kiss {
        return Err(soak_error(
            SoakFailureKind::InvariantFailure,
            "canonical execution did not land with Kiss",
        ));
    }
    Ok(execution.recipe().recipe_id().to_string())
}

fn soak_rollback(rng: &mut DeterministicRng) -> Result<String, SoakError> {
    let recipe = if rng.next_bool() {
        resolve_candidate_recipe(DecisionCandidateId::GremlinTinker)
    } else {
        resolve_candidate_recipe(DecisionCandidateId::PixyConfusion)
    };
    let scripts = compile_recipe(&recipe).expect("canonical recipe should compile");
    let aim = build_aim(&recipe, scripts.clone());
    let cuts = landing_fault_cuts(scripts.len());
    let cut = cuts[rng.next_usize(cuts.len())];
    let start = FrameState::origin();
    let before = start.clone();
    let failed = apply_kiss_with_fault(&start, &aim, Some(cut));
    if !matches!(failed, Err(LandingFaultError::Injected(_))) {
        return Err(soak_error(
            SoakFailureKind::RollbackFailure,
            "rollback cut did not inject the expected failure",
        ));
    }
    if start != before {
        return Err(soak_error(
            SoakFailureKind::PartialCommit,
            "rollback cut mutated the source frame state",
        ));
    }
    let success = apply_kiss_with_fault(&start, &aim, None).map_err(|_| {
        soak_error(
            SoakFailureKind::RollbackFailure,
            "post-failure recovery failed",
        )
    })?;
    if success.contact() != ContactOutcome::Kiss {
        return Err(soak_error(
            SoakFailureKind::RollbackFailure,
            "post-failure recovery did not land successfully",
        ));
    }
    Ok(format!("{cut:?}"))
}

fn soak_semantic_hash(rng: &mut DeterministicRng) -> Result<String, SoakError> {
    let cases = 32 + (rng.next_u64() % 32);
    let seed = rng.next_u64();
    let first = semantic_hash_only(cases, seed);
    let second = semantic_hash_only(cases, seed);
    if first != second {
        return Err(soak_error(
            SoakFailureKind::SemanticHashMismatch,
            "semantic hash workload produced inconsistent hashes",
        ));
    }
    Ok(first)
}

fn select_soak_workload(rng: &mut DeterministicRng) -> SoakWorkload {
    match rng.next_u64() % 100 {
        0..=29 => SoakWorkload::DecisionEvaluation,
        30..=44 => SoakWorkload::RouteAwareTie,
        45..=54 => SoakWorkload::RealizedStateSwitch,
        55..=64 => SoakWorkload::ValidReplay,
        65..=74 => SoakWorkload::CorruptedReplay,
        75..=84 => SoakWorkload::Differential,
        85..=92 => SoakWorkload::CanonicalExecution,
        93..=97 => SoakWorkload::Rollback,
        _ => SoakWorkload::SemanticHash,
    }
}

fn soak_distribution() -> Vec<(&'static str, u8)> {
    vec![
        ("decision_evaluation", 30),
        ("route_aware_tie", 15),
        ("realized_state_switch", 10),
        ("valid_replay", 10),
        ("corrupted_replay", 10),
        ("differential", 10),
        ("canonical_execution", 8),
        ("rollback", 5),
        ("semantic_hash", 2),
    ]
}

#[allow(clippy::too_many_arguments)]
fn build_soak_report(
    seed: u64,
    duration_requested_seconds: u64,
    distribution: &[(&'static str, u8)],
    duration_completed_seconds: f64,
    workload_counts: &BTreeMap<String, u64>,
    total_operations: u64,
    latency: &LatencyReservoir,
    semantic_hasher: &StableHasher,
    differential_mismatches: u64,
    semantic_hash_mismatches: u64,
    replay_false_acceptances: u64,
    rollback_failures: u64,
    partial_commits: u64,
    panic_count: u64,
    invariant_failures: u64,
    source_state_mutations: u64,
    unexplained_decisions: u64,
    corrupted_candidate_order: u64,
    rss_start_kb: Option<u64>,
    rss_end_kb: Option<u64>,
    rss_peak_kb: Option<u64>,
    fd_start: Option<u64>,
    fd_end: Option<u64>,
    fd_peak: Option<u64>,
    memory_samples: Vec<SoakMemorySample>,
    interrupted: bool,
    failure_reasons: Vec<String>,
) -> SoakReport {
    let operations_per_second = if duration_completed_seconds > 0.0 {
        total_operations as f64 / duration_completed_seconds
    } else {
        0.0
    };
    SoakReport {
        duration_requested_seconds,
        duration_completed_seconds,
        seed,
        workload_distribution: distribution.to_vec(),
        workload_counts: workload_counts.clone(),
        total_operations,
        operations_per_second,
        latency_min_us: latency.min_us(),
        latency_mean_us: latency.mean_us(),
        latency_p50_us: latency.percentile(0.50),
        latency_p95_us: latency.percentile(0.95),
        latency_p99_us: latency.percentile(0.99),
        latency_max_us: latency.max_us(),
        semantic_hash_accumulator: semantic_hasher.finish_hex(),
        differential_mismatches,
        semantic_hash_mismatches,
        replay_false_acceptances,
        rollback_failures,
        partial_commits,
        panic_count,
        invariant_failures,
        source_state_mutations,
        unexplained_decisions,
        corrupted_candidate_order,
        rss_start_kb,
        rss_end_kb,
        rss_peak_kb,
        fd_start,
        fd_end,
        fd_peak,
        memory_samples: memory_samples.clone(),
        memory_trend: classify_memory_trend(&memory_samples),
        interrupted,
        failure_reasons,
    }
}

fn current_rss_kb() -> Option<u64> {
    let contents = fs::read_to_string("/proc/self/status").ok()?;
    contents.lines().find_map(|line| {
        let value = line.strip_prefix("VmRSS:")?.trim();
        value.split_whitespace().next()?.parse::<u64>().ok()
    })
}

fn current_fd_count() -> Option<u64> {
    Some(fs::read_dir("/proc/self/fd").ok()?.count() as u64)
}

fn classify_memory_trend(samples: &[SoakMemorySample]) -> String {
    let values = samples
        .iter()
        .filter_map(|sample| sample.rss_kb)
        .collect::<Vec<_>>();
    if values.len() < 3 {
        return "insufficient_samples".to_string();
    }
    let monotonic = values.windows(2).all(|window| window[1] >= window[0]);
    let initial = values.first().copied().unwrap_or(0);
    let final_value = values.last().copied().unwrap_or(initial);
    if monotonic && final_value > initial.saturating_add(initial / 10).saturating_add(1_024) {
        return "sustained_monotonic_growth".to_string();
    }
    if final_value > initial.saturating_add(initial / 5).saturating_add(2_048) {
        return "growth_without_clear_stabilization".to_string();
    }
    "stable_after_warmup".to_string()
}

fn byte(data: &[u8], index: usize) -> u8 {
    data.get(index).copied().unwrap_or(0)
}

fn bounded_ascii(data: &[u8], max_len: usize, start: usize) -> String {
    let len = usize::from(byte(data, start)) % (max_len + 1);
    (0..len)
        .map(|offset| match byte(data, start + 1 + offset) % 37 {
            0..=9 => char::from(b'0' + byte(data, start + 1 + offset) % 10),
            10..=35 => char::from(b'a' + (byte(data, start + 1 + offset) % 26)),
            _ => '_',
        })
        .collect()
}

fn decode_subset<T: Copy>(values: &[T], mask: u8) -> Vec<T> {
    values
        .iter()
        .copied()
        .enumerate()
        .filter_map(|(index, value)| {
            if mask & (1 << (index % 8)) != 0 {
                Some(value)
            } else {
                None
            }
        })
        .collect()
}

fn percentile_samples(samples: &[f64], percentile_value: f64) -> f64 {
    if samples.is_empty() {
        return 0.0;
    }
    let mut sorted = samples.to_vec();
    sorted.sort_by(|left, right| left.partial_cmp(right).expect("finite values"));
    percentile(&sorted, percentile_value)
}

fn optional_u64_json(value: Option<u64>) -> String {
    value
        .map(|value| value.to_string())
        .unwrap_or_else(|| "null".to_string())
}

fn optional_u64_display(value: Option<u64>) -> String {
    value
        .map(|value| value.to_string())
        .unwrap_or_else(|| "unavailable".to_string())
}

fn option_max(left: Option<u64>, right: Option<u64>) -> Option<u64> {
    match (left, right) {
        (Some(left), Some(right)) => Some(left.max(right)),
        (Some(left), None) => Some(left),
        (None, Some(right)) => Some(right),
        (None, None) => None,
    }
}

fn soak_error(kind: SoakFailureKind, message: impl Into<String>) -> SoakError {
    SoakError {
        kind,
        message: message.into(),
    }
}

fn invariant_error(message: impl Into<String>) -> SoakError {
    soak_error(SoakFailureKind::InvariantFailure, message)
}

#[cfg(test)]
mod tests {
    use super::{
        DEFAULT_VERIFICATION_SEED, SoakConfig, VerificationProfile, exhaustive_state_space,
        fuzz_decision_input_bytes, fuzz_decision_trace_replay_bytes, fuzz_recipe_compiler_bytes,
        fuzz_snapshot_boundary_bytes, render_trace_text, run_differential_suite,
        run_metamorphic_suite, run_profile, run_property_suite, run_rollback_suite, run_soak,
        run_trace_corruption_suite, semantic_hash_only,
    };

    #[test]
    fn exhaustive_enumeration_is_deterministic_and_explained() {
        let summary = exhaustive_state_space();
        assert_eq!(summary.unexplained_states, 0);
        assert_eq!(summary.nondeterministic_states, 0);
        assert_eq!(summary.invariant_failures, 0);
        assert!(summary.total_legal_states > 0);
        assert!(!summary.rows.is_empty());
    }

    #[test]
    fn property_and_metamorphic_campaigns_hold_for_small_local_runs() {
        let property = run_property_suite(256, 0xC0FFEE42);
        let metamorphic = run_metamorphic_suite();
        assert_eq!(property.cases, 256);
        assert!(metamorphic.checks > 0);
    }

    #[test]
    fn differential_reference_agrees_with_production() {
        let summary = run_differential_suite(10_000, 0xC0FFEE42);
        assert_eq!(summary.mismatches, 0);
    }

    #[test]
    fn rollback_cuts_and_trace_corruptions_hold() {
        let rollback = run_rollback_suite();
        let corruption = run_trace_corruption_suite();
        assert!(rollback.cut_count > 0);
        assert_eq!(corruption.false_acceptances, 0);
        assert!(corruption.corruption_count > 0);
    }

    #[test]
    fn semantic_hash_is_stable_for_identical_inputs() {
        let first = semantic_hash_only(8_192, 0xC0FFEE42);
        let second = semantic_hash_only(8_192, 0xC0FFEE42);
        assert_eq!(first, second);
    }

    #[test]
    fn fuzz_entrypoints_accept_canonical_seed_inputs() {
        fuzz_decision_input_bytes(b"hueman-neutral");
        fuzz_decision_trace_replay_bytes(b"canonical-trace");
        fuzz_recipe_compiler_bytes(b"canonical-recipe");
        fuzz_snapshot_boundary_bytes(b"canonical-snapshot");
    }

    #[test]
    fn soak_smoke_holds_for_a_short_run() {
        let report = run_soak(SoakConfig {
            duration_seconds: 1,
            seed: DEFAULT_VERIFICATION_SEED,
            report_interval_seconds: 1,
        });
        assert!(report.total_operations > 0);
        assert!(report.succeeded(), "{:?}", report.failure_reasons);
    }

    #[test]
    fn fast_profile_generates_nonempty_reports() {
        let report = run_profile(VerificationProfile::Fast, 0xC0FFEE42, None, None);
        assert!(report.exhaustive.total_legal_states > 0);
        assert!(
            report
                .benchmarks
                .stages
                .iter()
                .all(|stage| stage.iterations > 0)
        );
        let rendered = render_trace_text(
            &super::canonical_traces()
                .into_iter()
                .next()
                .expect("trace should exist")
                .1,
        );
        assert!(!rendered.is_empty());
    }
}
