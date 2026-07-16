use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

#[path = "../current_synthesis_support.rs"]
mod current_synthesis_support;

use current_synthesis_support::{
    ARTIFACT_INDEX_PATH, CURRENT_SYNTHESIS_ACTIVATION_GATE_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_BASE_ARTIFACT_PATH, CURRENT_SYNTHESIS_BEHAVIOR_RULES_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_CHOICE_ARTIFACT_PATH, CURRENT_SYNTHESIS_CLIENTS_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_COLLISION_RELAY_ARTIFACT_PATH, CURRENT_SYNTHESIS_CONSEQUENCE_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_CONTRACT_ARTIFACT_PATH, CURRENT_SYNTHESIS_EXECUTION_SPEC_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_OPERATIONAL_ARTIFACT_PATH, CURRENT_SYNTHESIS_PREVIEW_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_READINESS_ARTIFACT_PATH, CURRENT_SYNTHESIS_SELECTION_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_SEQUENCE_ARTIFACT_PATH, CURRENT_SYNTHESIS_STATE_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_TOPOLOGY_ARTIFACT_PATH, CURRENT_SYNTHESIS_TRANSITION_PM_TO_LE_ARTIFACT_PATH,
    DESKTOP_STATUS_ARTIFACT_PATH, PROMPT_ARTIFACT_PATH, SNAPSHOT_ARTIFACT_PATH,
    build_current_synthesis_activation_gate_from_artifacts,
    build_current_synthesis_base_from_boundary,
    build_current_synthesis_behavior_rules_from_artifacts,
    build_current_synthesis_choice_from_artifacts, build_current_synthesis_clients_from_boundary,
    build_current_synthesis_collision_relay_from_boundary,
    build_current_synthesis_consequence_from_artifacts,
    build_current_synthesis_contract_from_artifacts,
    build_current_synthesis_execution_spec_from_artifacts,
    build_current_synthesis_operational_from_artifacts,
    build_current_synthesis_preview_from_artifacts,
    build_current_synthesis_readiness_from_artifacts,
    build_current_synthesis_selection_from_artifacts,
    build_current_synthesis_sequence_from_artifacts, build_current_synthesis_state_from_artifacts,
    build_current_synthesis_topology_from_boundary,
    build_current_synthesis_transition_pm_to_le_from_boundary, ensure_artifact_index,
    load_artifact_index,
};
use hollow_grove::SnapshotBoundary;
use hollow_grove::hueman_support::{
    HUEMAN_ARCHETYPE_LENS_ARTIFACT_PATH, HUEMAN_AURA_BEHAVIOR_ARTIFACT_PATH,
    HUEMAN_AURA_TRIAD_ARTIFACT_PATH, HUEMAN_BOUNDARY_ARTIFACT_PATH,
    HUEMAN_CROSSOVER_SCENES_ARTIFACT_PATH, HUEMAN_FOURWAY_ARTIFACT_PATH,
    HUEMAN_GLAUSHOUSE_ROLES_ARTIFACT_PATH, HUEMAN_INVERSE_CIRCLE_ARTIFACT_PATH,
    HUEMAN_LINK_PHYSICS_ARTIFACT_PATH, HUEMAN_MOTION_MAP_ARTIFACT_PATH,
    HUEMAN_PATH_CROSSOVERS_ARTIFACT_PATH, HUEMAN_PROCEDURAL_UPLIFT_ARTIFACT_PATH,
    HUEMAN_SANDMANOR_ROLES_ARTIFACT_PATH, HUEMAN_SCENE_DRIFT_ARTIFACT_PATH,
    HUEMAN_SCENE_INTENT_ARTIFACT_PATH, HUEMAN_SCENE_PRESENCE_ARTIFACT_PATH,
    HUEMAN_START_CHOICES_ARTIFACT_PATH, HUEMAN_START_PATHS_ARTIFACT_PATH,
    HUEMAN_STONEBEND_ROLES_ARTIFACT_PATH, HUEMAN_TROSS_HELPERS_ARTIFACT_PATH,
    VERTICAL_INTEGRATION_STACK_ARTIFACT_PATH, build_hueman_archetype_lens_from_artifacts,
    build_hueman_aura_behavior_from_artifacts, build_hueman_aura_triad_from_artifacts,
    build_hueman_boundary_from_artifacts, build_hueman_crossover_scenes_from_artifacts,
    build_hueman_fourway_from_artifacts, build_hueman_glaushouse_roles_from_artifacts,
    build_hueman_inverse_circle_from_artifacts, build_hueman_link_physics_from_artifacts,
    build_hueman_motion_map_from_artifacts, build_hueman_path_crossovers_from_artifacts,
    build_hueman_procedural_uplift_from_artifacts, build_hueman_sandmanor_roles_from_artifacts,
    build_hueman_scene_drift_from_artifacts, build_hueman_scene_intent_from_artifacts,
    build_hueman_scene_presence_from_artifacts, build_hueman_start_choices_from_artifacts,
    build_hueman_start_paths_from_artifacts, build_hueman_stonebend_roles_from_artifacts,
    build_hueman_tross_helpers_from_artifacts, build_vertical_integration_stack_from_artifacts,
};
use hollow_grove::{
    ArtifactSession, CANONICAL_WITNESS, Symptom, build_desktop_status_output,
    build_prompt_artifact_output, build_snapshot_output, run_kernel_cycle, write_text_artifact,
};

const BENCHMARK_REPORT_ARTIFACT_PATH: &str = "artifacts/current_synthesis_benchmark.md";
const BENCHMARK_SNAPSHOT_ARTIFACT_PATH: &str = "artifacts/current_synthesis_benchmark.json";
const BENCHMARK_RELEASE_ARTIFACT_PATH: &str = "artifacts/current_synthesis_benchmark_release.md";
const DEFAULT_WARMUP: usize = 5;
const DEFAULT_SAMPLES: usize = 25;
const HOLLOW_GROVE_MAIN_BINARY_NAME: &str = "hollow-grove";
const TOP_OUTLIER_COUNT: usize = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArtifactWriteMode {
    Disk,
    Memory,
}

impl ArtifactWriteMode {
    fn as_str(self) -> &'static str {
        match self {
            Self::Disk => "disk",
            Self::Memory => "memory",
        }
    }

    fn flushes_to_disk(self) -> bool {
        matches!(self, Self::Disk)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BenchmarkConfig {
    warmup: usize,
    samples: usize,
    artifact_write_mode: ArtifactWriteMode,
    quiet: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum BenchmarkCli {
    Help,
    Run(BenchmarkConfig),
}

#[derive(Debug, Clone)]
struct BenchmarkEnvironment {
    profile: &'static str,
    os: &'static str,
    arch: &'static str,
    cpu_parallelism: usize,
    benchmark_binary_bytes: u64,
    hollow_grove_binary_bytes: Option<u64>,
}

#[derive(Debug, Clone)]
struct BenchmarkPaths {
    artifact_index: PathBuf,
    snapshot: PathBuf,
    prompt: PathBuf,
    desktop_status: PathBuf,
    current_synthesis_base: PathBuf,
    current_synthesis_state: PathBuf,
    current_synthesis_sequence: PathBuf,
    current_synthesis_topology: PathBuf,
    current_synthesis_clients: PathBuf,
    current_synthesis_choice: PathBuf,
    current_synthesis_contract: PathBuf,
    current_synthesis_preview: PathBuf,
    current_synthesis_operational: PathBuf,
    current_synthesis_selection: PathBuf,
    current_synthesis_consequence: PathBuf,
    current_synthesis_readiness: PathBuf,
    current_synthesis_execution_spec: PathBuf,
    current_synthesis_behavior_rules: PathBuf,
    current_synthesis_transition_pm_to_le: PathBuf,
    current_synthesis_collision_relay: PathBuf,
    current_synthesis_activation_gate: PathBuf,
    hueman_boundary: PathBuf,
    hueman_motion_map: PathBuf,
    hueman_fourway: PathBuf,
    hueman_aura_triad: PathBuf,
    hueman_start_choices: PathBuf,
    hueman_stonebend_roles: PathBuf,
    hueman_tross_helpers: PathBuf,
    hueman_glaushouse_roles: PathBuf,
    hueman_sandmanor_roles: PathBuf,
    hueman_procedural_uplift: PathBuf,
    hueman_aura_behavior: PathBuf,
    hueman_archetype_lens: PathBuf,
    hueman_start_paths: PathBuf,
    hueman_path_crossovers: PathBuf,
    hueman_link_physics: PathBuf,
    hueman_inverse_circle: PathBuf,
    hueman_crossover_scenes: PathBuf,
    hueman_scene_presence: PathBuf,
    hueman_scene_intent: PathBuf,
    hueman_scene_drift: PathBuf,
    vertical_integration_stack: PathBuf,
}

impl BenchmarkPaths {
    fn new(root: &Path) -> Self {
        Self {
            artifact_index: root.join(ARTIFACT_INDEX_PATH),
            snapshot: root.join(SNAPSHOT_ARTIFACT_PATH),
            prompt: root.join(PROMPT_ARTIFACT_PATH),
            desktop_status: root.join(DESKTOP_STATUS_ARTIFACT_PATH),
            current_synthesis_base: root.join(CURRENT_SYNTHESIS_BASE_ARTIFACT_PATH),
            current_synthesis_state: root.join(CURRENT_SYNTHESIS_STATE_ARTIFACT_PATH),
            current_synthesis_sequence: root.join(CURRENT_SYNTHESIS_SEQUENCE_ARTIFACT_PATH),
            current_synthesis_topology: root.join(CURRENT_SYNTHESIS_TOPOLOGY_ARTIFACT_PATH),
            current_synthesis_clients: root.join(CURRENT_SYNTHESIS_CLIENTS_ARTIFACT_PATH),
            current_synthesis_choice: root.join(CURRENT_SYNTHESIS_CHOICE_ARTIFACT_PATH),
            current_synthesis_contract: root.join(CURRENT_SYNTHESIS_CONTRACT_ARTIFACT_PATH),
            current_synthesis_preview: root.join(CURRENT_SYNTHESIS_PREVIEW_ARTIFACT_PATH),
            current_synthesis_operational: root.join(CURRENT_SYNTHESIS_OPERATIONAL_ARTIFACT_PATH),
            current_synthesis_selection: root.join(CURRENT_SYNTHESIS_SELECTION_ARTIFACT_PATH),
            current_synthesis_consequence: root.join(CURRENT_SYNTHESIS_CONSEQUENCE_ARTIFACT_PATH),
            current_synthesis_readiness: root.join(CURRENT_SYNTHESIS_READINESS_ARTIFACT_PATH),
            current_synthesis_execution_spec: root
                .join(CURRENT_SYNTHESIS_EXECUTION_SPEC_ARTIFACT_PATH),
            current_synthesis_behavior_rules: root
                .join(CURRENT_SYNTHESIS_BEHAVIOR_RULES_ARTIFACT_PATH),
            current_synthesis_transition_pm_to_le: root
                .join(CURRENT_SYNTHESIS_TRANSITION_PM_TO_LE_ARTIFACT_PATH),
            current_synthesis_collision_relay: root
                .join(CURRENT_SYNTHESIS_COLLISION_RELAY_ARTIFACT_PATH),
            current_synthesis_activation_gate: root
                .join(CURRENT_SYNTHESIS_ACTIVATION_GATE_ARTIFACT_PATH),
            hueman_boundary: root.join(HUEMAN_BOUNDARY_ARTIFACT_PATH),
            hueman_motion_map: root.join(HUEMAN_MOTION_MAP_ARTIFACT_PATH),
            hueman_fourway: root.join(HUEMAN_FOURWAY_ARTIFACT_PATH),
            hueman_aura_triad: root.join(HUEMAN_AURA_TRIAD_ARTIFACT_PATH),
            hueman_start_choices: root.join(HUEMAN_START_CHOICES_ARTIFACT_PATH),
            hueman_stonebend_roles: root.join(HUEMAN_STONEBEND_ROLES_ARTIFACT_PATH),
            hueman_tross_helpers: root.join(HUEMAN_TROSS_HELPERS_ARTIFACT_PATH),
            hueman_glaushouse_roles: root.join(HUEMAN_GLAUSHOUSE_ROLES_ARTIFACT_PATH),
            hueman_sandmanor_roles: root.join(HUEMAN_SANDMANOR_ROLES_ARTIFACT_PATH),
            hueman_procedural_uplift: root.join(HUEMAN_PROCEDURAL_UPLIFT_ARTIFACT_PATH),
            hueman_aura_behavior: root.join(HUEMAN_AURA_BEHAVIOR_ARTIFACT_PATH),
            hueman_archetype_lens: root.join(HUEMAN_ARCHETYPE_LENS_ARTIFACT_PATH),
            hueman_start_paths: root.join(HUEMAN_START_PATHS_ARTIFACT_PATH),
            hueman_path_crossovers: root.join(HUEMAN_PATH_CROSSOVERS_ARTIFACT_PATH),
            hueman_link_physics: root.join(HUEMAN_LINK_PHYSICS_ARTIFACT_PATH),
            hueman_inverse_circle: root.join(HUEMAN_INVERSE_CIRCLE_ARTIFACT_PATH),
            hueman_crossover_scenes: root.join(HUEMAN_CROSSOVER_SCENES_ARTIFACT_PATH),
            hueman_scene_presence: root.join(HUEMAN_SCENE_PRESENCE_ARTIFACT_PATH),
            hueman_scene_intent: root.join(HUEMAN_SCENE_INTENT_ARTIFACT_PATH),
            hueman_scene_drift: root.join(HUEMAN_SCENE_DRIFT_ARTIFACT_PATH),
            vertical_integration_stack: root.join(VERTICAL_INTEGRATION_STACK_ARTIFACT_PATH),
        }
    }
}

#[derive(Debug, Clone)]
struct StageSample {
    sample_index: usize,
    name: &'static str,
    group: &'static str,
    artifact_path: Option<PathBuf>,
    build_elapsed: Duration,
    write_elapsed: Duration,
    total_elapsed: Duration,
    output_bytes: usize,
}

#[derive(Debug, Clone)]
struct PipelineSample {
    total_elapsed: Duration,
    stages: Vec<StageSample>,
    witness: String,
    activation_gate: String,
    scene_drift: String,
}

#[derive(Debug, Clone)]
struct StageAggregate {
    name: &'static str,
    group: &'static str,
    output_bytes: usize,
    samples: Vec<StageSample>,
}

#[derive(Debug, Clone)]
struct StageStats {
    name: &'static str,
    group: &'static str,
    output_bytes: usize,
    sample_count: usize,
    build_avg_ns: u128,
    write_avg_ns: u128,
    total_avg_ns: u128,
    total_min_ns: u128,
    total_max_ns: u128,
    total_p95_ns: u128,
    stddev_ns: f64,
    coefficient_of_variation: f64,
    write_share_of_stage: f64,
    max_sample_index: usize,
    max_over_avg_ratio: f64,
}

#[derive(Debug, Clone)]
struct GroupStats {
    name: &'static str,
    build_avg_ns: u128,
    write_avg_ns: u128,
    total_avg_ns: u128,
    share_of_total: f64,
}

#[derive(Debug, Clone)]
struct WeakPoint {
    title: String,
    detail: String,
    fix_hint: String,
}

#[derive(Debug, Clone)]
struct ReleaseGate {
    title: &'static str,
    passed: bool,
    detail: String,
}

#[derive(Debug, Clone)]
struct OutlierRecord {
    stage_name: &'static str,
    group: &'static str,
    sample_index: usize,
    total_ns: u128,
    avg_ns: u128,
    multiplier: f64,
    write_share_of_stage: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PublishSignal {
    Strong,
    Promising,
    Weak,
}

impl PublishSignal {
    fn as_str(self) -> &'static str {
        match self {
            Self::Strong => "strong",
            Self::Promising => "promising",
            Self::Weak => "weak",
        }
    }
}

#[derive(Debug, Clone)]
struct BenchmarkReport {
    environment: BenchmarkEnvironment,
    generated_unix_time_s: u64,
    warmup: usize,
    samples: usize,
    artifact_write_mode: ArtifactWriteMode,
    total_avg_ns: u128,
    total_min_ns: u128,
    total_max_ns: u128,
    total_p95_ns: u128,
    stage_stats: Vec<StageStats>,
    group_stats: Vec<GroupStats>,
    drift_count: usize,
    publish_signal: PublishSignal,
    release_gates: Vec<ReleaseGate>,
    weak_points: Vec<WeakPoint>,
    outliers: Vec<OutlierRecord>,
    release_headline: String,
    internal_caution: String,
}

#[derive(Debug, Clone)]
struct SeriesStats {
    avg_ns: u128,
    min_ns: u128,
    max_ns: u128,
    p95_ns: u128,
    stddev_ns: f64,
    coefficient_of_variation: f64,
}

fn parse_benchmark_cli<I>(args: I) -> Result<BenchmarkCli, String>
where
    I: IntoIterator<Item = String>,
{
    let mut warmup = DEFAULT_WARMUP;
    let mut samples = DEFAULT_SAMPLES;
    let mut artifact_write_mode = ArtifactWriteMode::Disk;
    let mut quiet = false;
    let mut args = args.into_iter();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--help" | "-h" => return Ok(BenchmarkCli::Help),
            "--no-write" => artifact_write_mode = ArtifactWriteMode::Memory,
            "--quiet" => quiet = true,
            "--warmup" => {
                let value = args
                    .next()
                    .ok_or_else(|| String::from("missing value for --warmup"))?;
                warmup = parse_positive_usize("--warmup", &value)?;
            }
            "--samples" => {
                let value = args
                    .next()
                    .ok_or_else(|| String::from("missing value for --samples"))?;
                samples = parse_positive_usize("--samples", &value)?;
            }
            other => return Err(format!("unknown argument: {other}")),
        }
    }

    Ok(BenchmarkCli::Run(BenchmarkConfig {
        warmup,
        samples,
        artifact_write_mode,
        quiet,
    }))
}

fn parse_positive_usize(flag: &str, value: &str) -> Result<usize, String> {
    let parsed = value
        .parse::<usize>()
        .map_err(|_| format!("invalid usize for {flag}: {value}"))?;
    if parsed == 0 {
        return Err(format!("{flag} must be greater than zero"));
    }
    Ok(parsed)
}

fn usage() -> &'static str {
    "Usage: current_synthesis_benchmark [--warmup N] [--samples N] [--no-write] [--quiet]"
}

fn unix_time_seconds() -> io::Result<u64> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| io::Error::other(format!("system time error: {error}")))?
        .as_secs())
}

fn benchmark_profile() -> &'static str {
    if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    }
}

fn file_size(path: &Path) -> Option<u64> {
    fs::metadata(path).ok().map(|metadata| metadata.len())
}

fn capture_environment() -> io::Result<BenchmarkEnvironment> {
    let current_exe = env::current_exe()?;
    let benchmark_binary_bytes = file_size(&current_exe).unwrap_or(0);
    let hollow_grove_binary_bytes = current_exe
        .parent()
        .map(|parent| parent.join(HOLLOW_GROVE_MAIN_BINARY_NAME))
        .and_then(|path| file_size(&path));

    Ok(BenchmarkEnvironment {
        profile: benchmark_profile(),
        os: env::consts::OS,
        arch: env::consts::ARCH,
        cpu_parallelism: std::thread::available_parallelism()
            .map(|parallelism| parallelism.get())
            .unwrap_or(1),
        benchmark_binary_bytes,
        hollow_grove_binary_bytes,
    })
}

fn combine_durations(left: Duration, right: Duration) -> Duration {
    left + right
}

fn measure_value_stage<T, F, B>(
    sample_index: usize,
    name: &'static str,
    group: &'static str,
    byte_len: B,
    operation: F,
) -> io::Result<(T, StageSample)>
where
    F: FnOnce() -> io::Result<T>,
    B: Fn(&T) -> usize,
{
    let build_started = Instant::now();
    let value = operation()?;
    let build_elapsed = build_started.elapsed();
    let output_bytes = byte_len(&value);

    Ok((
        value,
        StageSample {
            sample_index,
            name,
            group,
            artifact_path: None,
            build_elapsed,
            write_elapsed: Duration::ZERO,
            total_elapsed: build_elapsed,
            output_bytes,
        },
    ))
}

fn measure_artifact_stage<F>(
    session: &mut ArtifactSession,
    path: &Path,
    sample_index: usize,
    name: &'static str,
    group: &'static str,
    build: F,
) -> io::Result<(String, StageSample)>
where
    F: FnOnce() -> io::Result<String>,
{
    let build_started = Instant::now();
    let output = build()?;
    let build_elapsed = build_started.elapsed();
    let output_bytes = output.len();
    session.stage_text_artifact(path, &output);

    Ok((
        output,
        StageSample {
            sample_index,
            name,
            group,
            artifact_path: Some(path.to_path_buf()),
            build_elapsed,
            write_elapsed: Duration::ZERO,
            total_elapsed: build_elapsed,
            output_bytes,
        },
    ))
}

fn run_pipeline_sample_with_paths(
    paths: &BenchmarkPaths,
    sample_index: usize,
    artifact_write_mode: ArtifactWriteMode,
) -> io::Result<PipelineSample> {
    ensure_artifact_index(&paths.artifact_index)?;

    let total_started = Instant::now();
    let mut stages = Vec::new();
    let mut session = ArtifactSession::new();

    let (kernel_pass, stage) = measure_value_stage(
        sample_index,
        "kernel_pass",
        "kernel",
        |kernel_pass: &hollow_grove::KernelPass| kernel_pass.canonical_witness().len(),
        || Ok(run_kernel_cycle(Symptom::origin())),
    )?;
    stages.push(stage);

    let (snapshot, stage) = measure_artifact_stage(
        &mut session,
        &paths.snapshot,
        sample_index,
        "client_snapshot",
        "clients",
        || Ok(build_snapshot_output(&kernel_pass)),
    )?;
    stages.push(stage);

    let (prompt, stage) = measure_artifact_stage(
        &mut session,
        &paths.prompt,
        sample_index,
        "client_prompt_artifact",
        "clients",
        || Ok(build_prompt_artifact_output(&kernel_pass)),
    )?;
    stages.push(stage);

    let (desktop_status, stage) = measure_artifact_stage(
        &mut session,
        &paths.desktop_status,
        sample_index,
        "client_desktop_status",
        "clients",
        || Ok(build_desktop_status_output(&kernel_pass)),
    )?;
    stages.push(stage);

    let snapshot_boundary = SnapshotBoundary::parse(&snapshot)?;

    let artifact_index = load_artifact_index(&paths.artifact_index)?;

    let (current_synthesis_base, stage) = measure_artifact_stage(
        &mut session,
        &paths.current_synthesis_base,
        sample_index,
        "current_synthesis_base",
        "current_synthesis",
        || {
            build_current_synthesis_base_from_boundary(
                &snapshot_boundary,
                snapshot.len(),
                &prompt,
                &desktop_status,
            )
        },
    )?;
    stages.push(stage);

    let (current_synthesis_state, stage) = measure_artifact_stage(
        &mut session,
        &paths.current_synthesis_state,
        sample_index,
        "current_synthesis_state",
        "current_synthesis",
        || {
            Ok(build_current_synthesis_state_from_artifacts(
                &current_synthesis_base,
                &artifact_index,
            ))
        },
    )?;
    stages.push(stage);

    let (current_synthesis_sequence, stage) = measure_artifact_stage(
        &mut session,
        &paths.current_synthesis_sequence,
        sample_index,
        "current_synthesis_sequence",
        "current_synthesis",
        || {
            Ok(build_current_synthesis_sequence_from_artifacts(
                &current_synthesis_base,
                &current_synthesis_state,
            ))
        },
    )?;
    stages.push(stage);

    let (current_synthesis_topology, stage) = measure_artifact_stage(
        &mut session,
        &paths.current_synthesis_topology,
        sample_index,
        "current_synthesis_topology",
        "current_synthesis",
        || {
            Ok(build_current_synthesis_topology_from_boundary(
                &snapshot_boundary,
                snapshot.len(),
                &current_synthesis_sequence,
                &current_synthesis_state,
            ))
        },
    )?;
    stages.push(stage);

    let (current_synthesis_clients, stage) = measure_artifact_stage(
        &mut session,
        &paths.current_synthesis_clients,
        sample_index,
        "current_synthesis_clients",
        "current_synthesis",
        || {
            Ok(build_current_synthesis_clients_from_boundary(
                &snapshot_boundary,
                snapshot.len(),
                &current_synthesis_topology,
                &current_synthesis_sequence,
            ))
        },
    )?;
    stages.push(stage);

    let (current_synthesis_choice, stage) = measure_artifact_stage(
        &mut session,
        &paths.current_synthesis_choice,
        sample_index,
        "current_synthesis_choice",
        "current_synthesis",
        || {
            Ok(build_current_synthesis_choice_from_artifacts(
                &current_synthesis_clients,
                &current_synthesis_topology,
            ))
        },
    )?;
    stages.push(stage);

    let (current_synthesis_contract, stage) = measure_artifact_stage(
        &mut session,
        &paths.current_synthesis_contract,
        sample_index,
        "current_synthesis_contract",
        "current_synthesis",
        || {
            Ok(build_current_synthesis_contract_from_artifacts(
                &current_synthesis_choice,
                &current_synthesis_clients,
            ))
        },
    )?;
    stages.push(stage);

    let (current_synthesis_preview, stage) = measure_artifact_stage(
        &mut session,
        &paths.current_synthesis_preview,
        sample_index,
        "current_synthesis_preview",
        "current_synthesis",
        || {
            Ok(build_current_synthesis_preview_from_artifacts(
                &current_synthesis_contract,
                &current_synthesis_sequence,
            ))
        },
    )?;
    stages.push(stage);

    let (current_synthesis_operational, stage) = measure_artifact_stage(
        &mut session,
        &paths.current_synthesis_operational,
        sample_index,
        "current_synthesis_operational",
        "current_synthesis",
        || {
            Ok(build_current_synthesis_operational_from_artifacts(
                &current_synthesis_preview,
                &current_synthesis_contract,
            ))
        },
    )?;
    stages.push(stage);

    let (current_synthesis_selection, stage) = measure_artifact_stage(
        &mut session,
        &paths.current_synthesis_selection,
        sample_index,
        "current_synthesis_selection",
        "current_synthesis",
        || {
            Ok(build_current_synthesis_selection_from_artifacts(
                &current_synthesis_choice,
                &current_synthesis_operational,
            ))
        },
    )?;
    stages.push(stage);

    let (current_synthesis_consequence, stage) = measure_artifact_stage(
        &mut session,
        &paths.current_synthesis_consequence,
        sample_index,
        "current_synthesis_consequence",
        "current_synthesis",
        || {
            Ok(build_current_synthesis_consequence_from_artifacts(
                &current_synthesis_selection,
                &current_synthesis_operational,
            ))
        },
    )?;
    stages.push(stage);

    let (current_synthesis_readiness, stage) = measure_artifact_stage(
        &mut session,
        &paths.current_synthesis_readiness,
        sample_index,
        "current_synthesis_readiness",
        "current_synthesis",
        || {
            Ok(build_current_synthesis_readiness_from_artifacts(
                &current_synthesis_consequence,
                &current_synthesis_selection,
            ))
        },
    )?;
    stages.push(stage);

    let (current_synthesis_execution_spec, stage) = measure_artifact_stage(
        &mut session,
        &paths.current_synthesis_execution_spec,
        sample_index,
        "current_synthesis_execution_spec",
        "current_synthesis",
        || {
            Ok(build_current_synthesis_execution_spec_from_artifacts(
                &current_synthesis_readiness,
                &current_synthesis_consequence,
            ))
        },
    )?;
    stages.push(stage);

    let (current_synthesis_behavior_rules, stage) = measure_artifact_stage(
        &mut session,
        &paths.current_synthesis_behavior_rules,
        sample_index,
        "current_synthesis_behavior_rules",
        "current_synthesis",
        || {
            Ok(build_current_synthesis_behavior_rules_from_artifacts(
                &current_synthesis_execution_spec,
                &current_synthesis_selection,
            ))
        },
    )?;
    stages.push(stage);

    let (current_synthesis_transition_pm_to_le, stage) = measure_artifact_stage(
        &mut session,
        &paths.current_synthesis_transition_pm_to_le,
        sample_index,
        "current_synthesis_transition_pm_to_le",
        "current_synthesis",
        || {
            Ok(build_current_synthesis_transition_pm_to_le_from_boundary(
                &current_synthesis_behavior_rules,
                &current_synthesis_topology,
                &snapshot_boundary,
                snapshot.len(),
            ))
        },
    )?;
    stages.push(stage);

    let (current_synthesis_collision_relay, stage) = measure_artifact_stage(
        &mut session,
        &paths.current_synthesis_collision_relay,
        sample_index,
        "current_synthesis_collision_relay",
        "current_synthesis",
        || {
            Ok(build_current_synthesis_collision_relay_from_boundary(
                &snapshot_boundary,
                snapshot.len(),
                &current_synthesis_contract,
                &current_synthesis_operational,
                &current_synthesis_transition_pm_to_le,
            ))
        },
    )?;
    stages.push(stage);

    let (current_synthesis_activation_gate, stage) = measure_artifact_stage(
        &mut session,
        &paths.current_synthesis_activation_gate,
        sample_index,
        "current_synthesis_activation_gate",
        "current_synthesis",
        || {
            Ok(build_current_synthesis_activation_gate_from_artifacts(
                &current_synthesis_transition_pm_to_le,
                &current_synthesis_collision_relay,
                &current_synthesis_readiness,
            ))
        },
    )?;
    stages.push(stage);

    let (hueman_boundary, stage) = measure_artifact_stage(
        &mut session,
        &paths.hueman_boundary,
        sample_index,
        "hueman_boundary",
        "hueman",
        || {
            Ok(build_hueman_boundary_from_artifacts(
                &current_synthesis_base,
                &current_synthesis_activation_gate,
            ))
        },
    )?;
    stages.push(stage);

    let (hueman_motion_map, stage) = measure_artifact_stage(
        &mut session,
        &paths.hueman_motion_map,
        sample_index,
        "hueman_motion_map",
        "hueman",
        || {
            Ok(build_hueman_motion_map_from_artifacts(
                &hueman_boundary,
                &current_synthesis_operational,
            ))
        },
    )?;
    stages.push(stage);

    let (hueman_fourway, stage) = measure_artifact_stage(
        &mut session,
        &paths.hueman_fourway,
        sample_index,
        "hueman_fourway",
        "hueman",
        || {
            Ok(build_hueman_fourway_from_artifacts(
                &hueman_boundary,
                &hueman_motion_map,
            ))
        },
    )?;
    stages.push(stage);

    let (hueman_aura_triad, stage) = measure_artifact_stage(
        &mut session,
        &paths.hueman_aura_triad,
        sample_index,
        "hueman_aura_triad",
        "hueman",
        || {
            Ok(build_hueman_aura_triad_from_artifacts(
                &hueman_fourway,
                &current_synthesis_topology,
            ))
        },
    )?;
    stages.push(stage);

    let (hueman_start_choices, stage) = measure_artifact_stage(
        &mut session,
        &paths.hueman_start_choices,
        sample_index,
        "hueman_start_choices",
        "hueman",
        || {
            Ok(build_hueman_start_choices_from_artifacts(
                &hueman_fourway,
                &hueman_aura_triad,
            ))
        },
    )?;
    stages.push(stage);

    let (hueman_stonebend_roles, stage) = measure_artifact_stage(
        &mut session,
        &paths.hueman_stonebend_roles,
        sample_index,
        "hueman_stonebend_roles",
        "hueman",
        || {
            Ok(build_hueman_stonebend_roles_from_artifacts(
                &hueman_start_choices,
                &hueman_fourway,
            ))
        },
    )?;
    stages.push(stage);

    let (hueman_tross_helpers, stage) = measure_artifact_stage(
        &mut session,
        &paths.hueman_tross_helpers,
        sample_index,
        "hueman_tross_helpers",
        "hueman",
        || {
            Ok(build_hueman_tross_helpers_from_artifacts(
                &hueman_start_choices,
                &hueman_fourway,
            ))
        },
    )?;
    stages.push(stage);

    let (hueman_glaushouse_roles, stage) = measure_artifact_stage(
        &mut session,
        &paths.hueman_glaushouse_roles,
        sample_index,
        "hueman_glaushouse_roles",
        "hueman",
        || {
            Ok(build_hueman_glaushouse_roles_from_artifacts(
                &hueman_start_choices,
                &hueman_fourway,
            ))
        },
    )?;
    stages.push(stage);

    let (hueman_sandmanor_roles, stage) = measure_artifact_stage(
        &mut session,
        &paths.hueman_sandmanor_roles,
        sample_index,
        "hueman_sandmanor_roles",
        "hueman",
        || {
            Ok(build_hueman_sandmanor_roles_from_artifacts(
                &hueman_start_choices,
                &hueman_fourway,
            ))
        },
    )?;
    stages.push(stage);

    let (hueman_procedural_uplift, stage) = measure_artifact_stage(
        &mut session,
        &paths.hueman_procedural_uplift,
        sample_index,
        "hueman_procedural_uplift",
        "hueman",
        || {
            Ok(build_hueman_procedural_uplift_from_artifacts(
                &current_synthesis_execution_spec,
                &current_synthesis_behavior_rules,
                &current_synthesis_transition_pm_to_le,
                &current_synthesis_collision_relay,
                &current_synthesis_selection,
                &current_synthesis_consequence,
                &current_synthesis_activation_gate,
                &hueman_stonebend_roles,
                &hueman_tross_helpers,
                &hueman_glaushouse_roles,
                &hueman_sandmanor_roles,
            ))
        },
    )?;
    stages.push(stage);

    let (hueman_aura_behavior, stage) = measure_artifact_stage(
        &mut session,
        &paths.hueman_aura_behavior,
        sample_index,
        "hueman_aura_behavior",
        "hueman",
        || {
            Ok(build_hueman_aura_behavior_from_artifacts(
                &hueman_aura_triad,
                &hueman_start_choices,
            ))
        },
    )?;
    stages.push(stage);

    let (hueman_archetype_lens, stage) = measure_artifact_stage(
        &mut session,
        &paths.hueman_archetype_lens,
        sample_index,
        "hueman_archetype_lens",
        "hueman",
        || {
            Ok(build_hueman_archetype_lens_from_artifacts(
                &hueman_start_choices,
                &hueman_aura_behavior,
                &hueman_stonebend_roles,
                &hueman_sandmanor_roles,
            ))
        },
    )?;
    stages.push(stage);

    let (hueman_start_paths, stage) = measure_artifact_stage(
        &mut session,
        &paths.hueman_start_paths,
        sample_index,
        "hueman_start_paths",
        "hueman",
        || {
            Ok(build_hueman_start_paths_from_artifacts(
                &hueman_start_choices,
                &hueman_archetype_lens,
            ))
        },
    )?;
    stages.push(stage);

    let (hueman_path_crossovers, stage) = measure_artifact_stage(
        &mut session,
        &paths.hueman_path_crossovers,
        sample_index,
        "hueman_path_crossovers",
        "hueman",
        || {
            Ok(build_hueman_path_crossovers_from_artifacts(
                &hueman_start_paths,
                &hueman_aura_behavior,
                &current_synthesis_collision_relay,
            ))
        },
    )?;
    stages.push(stage);

    let (hueman_link_physics, stage) = measure_artifact_stage(
        &mut session,
        &paths.hueman_link_physics,
        sample_index,
        "hueman_link_physics",
        "hueman",
        || {
            Ok(build_hueman_link_physics_from_artifacts(
                &current_synthesis_sequence,
                &hueman_path_crossovers,
                &current_synthesis_collision_relay,
            ))
        },
    )?;
    stages.push(stage);

    let (hueman_inverse_circle, stage) = measure_artifact_stage(
        &mut session,
        &paths.hueman_inverse_circle,
        sample_index,
        "hueman_inverse_circle",
        "hueman",
        || {
            Ok(build_hueman_inverse_circle_from_artifacts(
                &hueman_fourway,
                &hueman_link_physics,
            ))
        },
    )?;
    stages.push(stage);

    let (hueman_crossover_scenes, stage) = measure_artifact_stage(
        &mut session,
        &paths.hueman_crossover_scenes,
        sample_index,
        "hueman_crossover_scenes",
        "hueman",
        || {
            Ok(build_hueman_crossover_scenes_from_artifacts(
                &hueman_path_crossovers,
                &hueman_link_physics,
                &current_synthesis_collision_relay,
            ))
        },
    )?;
    stages.push(stage);

    let (hueman_scene_presence, stage) = measure_artifact_stage(
        &mut session,
        &paths.hueman_scene_presence,
        sample_index,
        "hueman_scene_presence",
        "hueman",
        || {
            Ok(build_hueman_scene_presence_from_artifacts(
                &hueman_crossover_scenes,
                &hueman_archetype_lens,
                &hueman_stonebend_roles,
                &hueman_tross_helpers,
                &hueman_glaushouse_roles,
                &hueman_sandmanor_roles,
                &hueman_inverse_circle,
                &current_synthesis_collision_relay,
            ))
        },
    )?;
    stages.push(stage);

    let (hueman_scene_intent, stage) = measure_artifact_stage(
        &mut session,
        &paths.hueman_scene_intent,
        sample_index,
        "hueman_scene_intent",
        "hueman",
        || {
            Ok(build_hueman_scene_intent_from_artifacts(
                &hueman_scene_presence,
                &hueman_link_physics,
                &current_synthesis_collision_relay,
                &current_synthesis_contract,
                &hueman_stonebend_roles,
                &hueman_tross_helpers,
                &hueman_glaushouse_roles,
                &hueman_sandmanor_roles,
                &hueman_inverse_circle,
            ))
        },
    )?;
    stages.push(stage);

    let (hueman_scene_drift, stage) = measure_artifact_stage(
        &mut session,
        &paths.hueman_scene_drift,
        sample_index,
        "hueman_scene_drift",
        "hueman",
        || {
            Ok(build_hueman_scene_drift_from_artifacts(
                &hueman_scene_intent,
                &hueman_link_physics,
                &current_synthesis_collision_relay,
            ))
        },
    )?;
    stages.push(stage);

    let (_vertical_integration_stack, stage) = measure_artifact_stage(
        &mut session,
        &paths.vertical_integration_stack,
        sample_index,
        "vertical_integration_stack",
        "hueman",
        || {
            Ok(build_vertical_integration_stack_from_artifacts(
                &current_synthesis_base,
                &current_synthesis_collision_relay,
                &hueman_boundary,
                &hueman_glaushouse_roles,
                &hueman_sandmanor_roles,
                &hueman_inverse_circle,
                &hueman_procedural_uplift,
                &hueman_scene_presence,
                &hueman_scene_intent,
                &hueman_scene_drift,
            ))
        },
    )?;
    stages.push(stage);

    let mut flush_elapsed_by_path = HashMap::new();
    if artifact_write_mode.flushes_to_disk() {
        let flush_records = session.commit_timed()?;
        flush_elapsed_by_path = HashMap::with_capacity(flush_records.len());
        for record in flush_records {
            flush_elapsed_by_path.insert(record.path, record.elapsed);
        }
    }

    for stage in &mut stages {
        let Some(path) = stage.artifact_path.as_ref() else {
            continue;
        };
        let write_elapsed = flush_elapsed_by_path.remove(path).unwrap_or(Duration::ZERO);
        stage.write_elapsed = write_elapsed;
        stage.total_elapsed = combine_durations(stage.build_elapsed, write_elapsed);
    }

    Ok(PipelineSample {
        total_elapsed: total_started.elapsed(),
        stages,
        witness: kernel_pass.canonical_witness().to_owned(),
        activation_gate: current_synthesis_activation_gate,
        scene_drift: hueman_scene_drift,
    })
}

#[cfg(test)]
fn run_pipeline_sample(
    root: &Path,
    sample_index: usize,
    artifact_write_mode: ArtifactWriteMode,
) -> io::Result<PipelineSample> {
    let paths = BenchmarkPaths::new(root);
    run_pipeline_sample_with_paths(&paths, sample_index, artifact_write_mode)
}

fn fold_samples(samples: &[PipelineSample]) -> Vec<StageAggregate> {
    let mut aggregates = Vec::<StageAggregate>::new();

    for sample in samples {
        if aggregates.is_empty() {
            aggregates = sample
                .stages
                .iter()
                .cloned()
                .map(|stage| StageAggregate {
                    name: stage.name,
                    group: stage.group,
                    output_bytes: stage.output_bytes,
                    samples: vec![stage],
                })
                .collect();
            continue;
        }

        for (aggregate, stage) in aggregates.iter_mut().zip(sample.stages.iter()) {
            aggregate.samples.push(stage.clone());
            aggregate.output_bytes = stage.output_bytes;
        }
    }

    aggregates
}

fn compute_series_stats(values: &[u128]) -> SeriesStats {
    let mut sorted = values.to_vec();
    sorted.sort_unstable();
    let sample_count = sorted.len();
    let sum_ns = sorted.iter().copied().sum::<u128>();
    let avg_ns = sum_ns / sample_count as u128;
    let min_ns = sorted[0];
    let max_ns = sorted[sample_count - 1];
    let p95_index = ((sample_count * 95).div_ceil(100)).saturating_sub(1);
    let p95_ns = sorted[p95_index];
    let mean = avg_ns as f64;
    let variance = sorted
        .iter()
        .map(|sample| {
            let delta = *sample as f64 - mean;
            delta * delta
        })
        .sum::<f64>()
        / sample_count as f64;
    let stddev_ns = variance.sqrt();
    let coefficient_of_variation = if mean == 0.0 { 0.0 } else { stddev_ns / mean };

    SeriesStats {
        avg_ns,
        min_ns,
        max_ns,
        p95_ns,
        stddev_ns,
        coefficient_of_variation,
    }
}

fn compute_stage_stats(aggregate: StageAggregate) -> StageStats {
    let build_values = aggregate
        .samples
        .iter()
        .map(|sample| sample.build_elapsed.as_nanos())
        .collect::<Vec<_>>();
    let write_values = aggregate
        .samples
        .iter()
        .map(|sample| sample.write_elapsed.as_nanos())
        .collect::<Vec<_>>();
    let total_values = aggregate
        .samples
        .iter()
        .map(|sample| sample.total_elapsed.as_nanos())
        .collect::<Vec<_>>();

    let build_avg_ns = build_values.iter().copied().sum::<u128>() / build_values.len() as u128;
    let write_avg_ns = write_values.iter().copied().sum::<u128>() / write_values.len() as u128;
    let total_stats = compute_series_stats(&total_values);
    let write_share_of_stage = if total_stats.avg_ns == 0 {
        0.0
    } else {
        write_avg_ns as f64 / total_stats.avg_ns as f64
    };
    let max_sample = aggregate
        .samples
        .iter()
        .max_by_key(|sample| sample.total_elapsed.as_nanos())
        .expect("stage samples should not be empty");
    let max_over_avg_ratio = if total_stats.avg_ns == 0 {
        0.0
    } else {
        max_sample.total_elapsed.as_nanos() as f64 / total_stats.avg_ns as f64
    };

    StageStats {
        name: aggregate.name,
        group: aggregate.group,
        output_bytes: aggregate.output_bytes,
        sample_count: aggregate.samples.len(),
        build_avg_ns,
        write_avg_ns,
        total_avg_ns: total_stats.avg_ns,
        total_min_ns: total_stats.min_ns,
        total_max_ns: total_stats.max_ns,
        total_p95_ns: total_stats.p95_ns,
        stddev_ns: total_stats.stddev_ns,
        coefficient_of_variation: total_stats.coefficient_of_variation,
        write_share_of_stage,
        max_sample_index: max_sample.sample_index,
        max_over_avg_ratio,
    }
}

fn compute_group_stats(stage_stats: &[StageStats], total_avg_ns: u128) -> Vec<GroupStats> {
    let groups = ["kernel", "clients", "current_synthesis", "hueman"];

    groups
        .into_iter()
        .map(|group| {
            let build_avg_ns = stage_stats
                .iter()
                .filter(|stage| stage.group == group)
                .map(|stage| stage.build_avg_ns)
                .sum::<u128>();
            let write_avg_ns = stage_stats
                .iter()
                .filter(|stage| stage.group == group)
                .map(|stage| stage.write_avg_ns)
                .sum::<u128>();
            let total_stage_avg_ns = stage_stats
                .iter()
                .filter(|stage| stage.group == group)
                .map(|stage| stage.total_avg_ns)
                .sum::<u128>();
            let share_of_total = if total_avg_ns == 0 {
                0.0
            } else {
                total_stage_avg_ns as f64 / total_avg_ns as f64
            };

            GroupStats {
                name: group,
                build_avg_ns,
                write_avg_ns,
                total_avg_ns: total_stage_avg_ns,
                share_of_total,
            }
        })
        .collect()
}

fn compute_total_stats(samples: &[PipelineSample]) -> SeriesStats {
    let totals = samples
        .iter()
        .map(|sample| sample.total_elapsed.as_nanos())
        .collect::<Vec<_>>();
    compute_series_stats(&totals)
}

fn count_drift(samples: &[PipelineSample]) -> usize {
    let Some(first) = samples.first() else {
        return 0;
    };

    samples
        .iter()
        .skip(1)
        .filter(|sample| {
            sample.witness != first.witness
                || sample.activation_gate != first.activation_gate
                || sample.scene_drift != first.scene_drift
        })
        .count()
}

fn collect_outliers(stage_stats: &[StageStats]) -> Vec<OutlierRecord> {
    let mut outliers = stage_stats
        .iter()
        .map(|stage| OutlierRecord {
            stage_name: stage.name,
            group: stage.group,
            sample_index: stage.max_sample_index,
            total_ns: stage.total_max_ns,
            avg_ns: stage.total_avg_ns,
            multiplier: stage.max_over_avg_ratio,
            write_share_of_stage: stage.write_share_of_stage,
        })
        .collect::<Vec<_>>();

    outliers.sort_by(|left, right| {
        right
            .multiplier
            .partial_cmp(&left.multiplier)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    outliers.truncate(TOP_OUTLIER_COUNT);
    outliers
}

fn stage_fix_surface(stage: &StageStats) -> &'static str {
    if stage.write_share_of_stage >= 0.35 {
        "filesystem write churn"
    } else {
        "string construction or allocation churn"
    }
}

fn identify_weak_points(
    stage_stats: &[StageStats],
    group_stats: &[GroupStats],
    total_avg_ns: u128,
    drift_count: usize,
) -> Vec<WeakPoint> {
    let mut weak_points = Vec::new();
    let Some(slowest_stage) = stage_stats.iter().max_by_key(|stage| stage.total_avg_ns) else {
        return weak_points;
    };
    let Some(noisiest_stage) = stage_stats.iter().max_by(|left, right| {
        left.coefficient_of_variation
            .partial_cmp(&right.coefficient_of_variation)
            .unwrap_or(std::cmp::Ordering::Equal)
    }) else {
        return weak_points;
    };
    let Some(largest_output_stage) = stage_stats.iter().max_by_key(|stage| stage.output_bytes)
    else {
        return weak_points;
    };
    let Some(heaviest_group) = group_stats.iter().max_by_key(|group| group.total_avg_ns) else {
        return weak_points;
    };

    let slowest_share = if total_avg_ns == 0 {
        0.0
    } else {
        slowest_stage.total_avg_ns as f64 / total_avg_ns as f64
    };

    weak_points.push(if slowest_share < 0.10 {
        WeakPoint {
            title: String::from("Latency Spread"),
            detail: format!(
                "No single stage dominates the route. The slowest stage is `{}` at {} us, which is only {:.1}% of the full pipeline cycle.",
                slowest_stage.name,
                ns_to_us_string(slowest_stage.total_avg_ns),
                slowest_share * 100.0
            ),
            fix_hint: format!(
                "Treat this as a route-width problem instead of a single-function problem. Reduce artifact churn across the `{}` group before micro-tuning `{}` in isolation.",
                heaviest_group.name, slowest_stage.name
            ),
        }
    } else {
        WeakPoint {
            title: format!("Latency Concentration: {}", slowest_stage.name),
            detail: format!(
                "`{}` averages {} us and consumes {:.1}% of the full pipeline cycle.",
                slowest_stage.name,
                ns_to_us_string(slowest_stage.total_avg_ns),
                slowest_share * 100.0
            ),
            fix_hint: format!(
                "Profile `{}` first. Most of its cost currently looks like {}.",
                slowest_stage.name,
                stage_fix_surface(slowest_stage)
            ),
        }
    });

    weak_points.push(WeakPoint {
        title: format!("Variance Pressure: {}", noisiest_stage.name),
        detail: format!(
            "`{}` has the highest timing variance with a {:.1}% coefficient of variation, a p95 of {} us, and a worst-sample spike of {:.1}x its average.",
            noisiest_stage.name,
            noisiest_stage.coefficient_of_variation * 100.0,
            ns_to_us_string(noisiest_stage.total_p95_ns),
            noisiest_stage.max_over_avg_ratio
        ),
        fix_hint: format!(
            "Investigate {} inside `{}` first; its worst sample came from benchmark sample `{}`.",
            stage_fix_surface(noisiest_stage),
            noisiest_stage.name,
            noisiest_stage.max_sample_index
        ),
    });

    weak_points.push(WeakPoint {
        title: format!("Artifact Weight: {}", largest_output_stage.name),
        detail: format!(
            "`{}` emits the largest single output at {} bytes.",
            largest_output_stage.name, largest_output_stage.output_bytes
        ),
        fix_hint: format!(
            "If publication depends on responsiveness and portability, check whether `{}` is carrying descriptive text that can be condensed or generated on demand.",
            largest_output_stage.name
        ),
    });

    weak_points.push(WeakPoint {
        title: format!("Group Load: {}", heaviest_group.name),
        detail: format!(
            "The `{}` group takes the largest share of the route at {:.1}% of average runtime, with {} us spent building and {} us spent writing.",
            heaviest_group.name,
            heaviest_group.share_of_total * 100.0,
            ns_to_us_string(heaviest_group.build_avg_ns),
            ns_to_us_string(heaviest_group.write_avg_ns)
        ),
        fix_hint: format!(
            "Treat the `{}` group as the primary optimization surface before chasing isolated micro-stages.",
            heaviest_group.name
        ),
    });

    weak_points.push(if drift_count == 0 {
        WeakPoint {
            title: String::from("Determinism Check"),
            detail: String::from(
                "No witness or downstream artifact drift was observed across measured samples.",
            ),
            fix_hint: String::from(
                "Keep this invariant. If later changes introduce drift, block performance marketing until the source is isolated.",
            ),
        }
    } else {
        WeakPoint {
            title: String::from("Determinism Break"),
            detail: format!(
                "{} measured samples drifted from the first benchmark witness or downstream artifact set.",
                drift_count
            ),
            fix_hint: String::from(
                "Fix determinism before publishing; drift will invalidate every performance claim in the report.",
            ),
        }
    });

    weak_points
}

fn build_release_gates(
    artifact_write_mode: ArtifactWriteMode,
    environment: &BenchmarkEnvironment,
    total_stats: &SeriesStats,
    drift_count: usize,
    highest_cv: f64,
) -> Vec<ReleaseGate> {
    let release_profile = environment.profile == "release";
    let deterministic = drift_count == 0;
    let p95_gate = total_stats.p95_ns <= 1_000_000;
    let variance_gate = highest_cv <= 0.50;
    let footprint_gate = environment
        .hollow_grove_binary_bytes
        .map(|bytes| bytes <= 1_000_000)
        .unwrap_or(false);
    let artifact_flush_gate = artifact_write_mode.flushes_to_disk();

    vec![
        ReleaseGate {
            title: "Artifact Flush",
            passed: artifact_flush_gate,
            detail: if artifact_flush_gate {
                String::from("Pipeline artifacts were flushed to disk during each sample.")
            } else {
                String::from(
                    "Pipeline artifacts were staged in memory only; flush cost was excluded.",
                )
            },
        },
        ReleaseGate {
            title: "Release Profile",
            passed: release_profile,
            detail: format!("Benchmark was executed in `{}` mode.", environment.profile),
        },
        ReleaseGate {
            title: "Determinism",
            passed: deterministic,
            detail: format!("Observed drift count: `{}`.", drift_count),
        },
        ReleaseGate {
            title: "P95 Runtime",
            passed: p95_gate,
            detail: format!(
                "Full-route p95 is `{}` ms.",
                ns_to_ms_string(total_stats.p95_ns)
            ),
        },
        ReleaseGate {
            title: "Variance Control",
            passed: variance_gate,
            detail: format!(
                "Highest stage coefficient of variation is `{:.1}%`.",
                highest_cv * 100.0
            ),
        },
        ReleaseGate {
            title: "Binary Footprint",
            passed: footprint_gate,
            detail: match environment.hollow_grove_binary_bytes {
                Some(bytes) => format!("Integrated `hollow-grove` binary is `{}` bytes.", bytes),
                None => String::from("Integrated `hollow-grove` binary size is unavailable."),
            },
        },
    ]
}

fn classify_publish_signal(release_gates: &[ReleaseGate]) -> PublishSignal {
    let pass_count = release_gates.iter().filter(|gate| gate.passed).count();
    if pass_count == release_gates.len() {
        PublishSignal::Strong
    } else if pass_count >= 3 {
        PublishSignal::Promising
    } else {
        PublishSignal::Weak
    }
}

fn build_release_headline(
    publish_signal: PublishSignal,
    artifact_write_mode: ArtifactWriteMode,
    environment: &BenchmarkEnvironment,
    total_stats: &SeriesStats,
    drift_count: usize,
) -> String {
    let prefix = match publish_signal {
        PublishSignal::Strong => "Release headline candidate",
        PublishSignal::Promising => "Release-supporting claim",
        PublishSignal::Weak => "Internal-only claim",
    };

    format!(
        "{}: Hollow Grove completes deterministic {} regeneration in {} ms average / {} ms p95 across {}-profile samples with drift count `{}`.",
        prefix,
        if artifact_write_mode.flushes_to_disk() {
            "full-route"
        } else {
            "in-memory staged-route"
        },
        ns_to_ms_string(total_stats.avg_ns),
        ns_to_ms_string(total_stats.p95_ns),
        environment.profile,
        drift_count
    )
}

fn build_internal_caution(
    publish_signal: PublishSignal,
    artifact_write_mode: ArtifactWriteMode,
    outliers: &[OutlierRecord],
    heaviest_group: &GroupStats,
) -> String {
    let caution_prefix = match publish_signal {
        PublishSignal::Strong => "Do not oversell it",
        PublishSignal::Promising => "Do not headline performance alone",
        PublishSignal::Weak => "Do not publish performance claims yet",
    };

    let outlier_stage = outliers
        .first()
        .map(|outlier| outlier.stage_name)
        .unwrap_or("(none)");

    format!(
        "{}: latency is still spread across the route and the biggest outlier pressure currently shows up in `{}` while `{}` remains the heaviest group. {}",
        caution_prefix,
        outlier_stage,
        heaviest_group.name,
        if artifact_write_mode.flushes_to_disk() {
            "These numbers include artifact flush cost."
        } else {
            "These numbers exclude artifact flush cost, so compare them against disk mode instead of replacing it."
        }
    )
}

fn build_benchmark_report(
    environment: BenchmarkEnvironment,
    generated_unix_time_s: u64,
    config: BenchmarkConfig,
    samples: &[PipelineSample],
) -> BenchmarkReport {
    let stage_stats = fold_samples(samples)
        .into_iter()
        .map(compute_stage_stats)
        .collect::<Vec<_>>();
    let total_stats = compute_total_stats(samples);
    let group_stats = compute_group_stats(&stage_stats, total_stats.avg_ns);
    let drift_count = count_drift(samples);
    let highest_cv = stage_stats
        .iter()
        .map(|stage| stage.coefficient_of_variation)
        .fold(0.0, f64::max);
    let release_gates = build_release_gates(
        config.artifact_write_mode,
        &environment,
        &total_stats,
        drift_count,
        highest_cv,
    );
    let publish_signal = classify_publish_signal(&release_gates);
    let weak_points =
        identify_weak_points(&stage_stats, &group_stats, total_stats.avg_ns, drift_count);
    let outliers = collect_outliers(&stage_stats);
    let heaviest_group = group_stats
        .iter()
        .max_by_key(|group| group.total_avg_ns)
        .expect("group stats should not be empty");
    let release_headline = build_release_headline(
        publish_signal,
        config.artifact_write_mode,
        &environment,
        &total_stats,
        drift_count,
    );
    let internal_caution = build_internal_caution(
        publish_signal,
        config.artifact_write_mode,
        &outliers,
        heaviest_group,
    );

    BenchmarkReport {
        environment,
        generated_unix_time_s,
        warmup: config.warmup,
        samples: config.samples,
        artifact_write_mode: config.artifact_write_mode,
        total_avg_ns: total_stats.avg_ns,
        total_min_ns: total_stats.min_ns,
        total_max_ns: total_stats.max_ns,
        total_p95_ns: total_stats.p95_ns,
        stage_stats,
        group_stats,
        drift_count,
        publish_signal,
        release_gates,
        weak_points,
        outliers,
        release_headline,
        internal_caution,
    }
}

fn ns_to_us_string(ns: u128) -> String {
    format!("{:.3}", ns as f64 / 1_000.0)
}

fn ns_to_ms_string(ns: u128) -> String {
    format!("{:.3}", ns as f64 / 1_000_000.0)
}

fn bytes_to_kib_string(bytes: u64) -> String {
    format!("{:.1}", bytes as f64 / 1024.0)
}

fn escape_json(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}

fn render_benchmark_markdown(report: &BenchmarkReport) -> String {
    let stage_table = report
        .stage_stats
        .iter()
        .map(|stage| {
            format!(
                "| `{}` | `{}` | {} | {} | {} | {} | {} | {:.1}% | {:.1}% | {} |",
                stage.name,
                stage.group,
                ns_to_us_string(stage.build_avg_ns),
                ns_to_us_string(stage.write_avg_ns),
                ns_to_us_string(stage.total_avg_ns),
                ns_to_us_string(stage.total_p95_ns),
                ns_to_us_string(stage.total_max_ns),
                stage.coefficient_of_variation * 100.0,
                stage.write_share_of_stage * 100.0,
                stage.output_bytes
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let group_table = report
        .group_stats
        .iter()
        .map(|group| {
            format!(
                "| `{}` | {} | {} | {} | {:.1}% |",
                group.name,
                ns_to_us_string(group.build_avg_ns),
                ns_to_us_string(group.write_avg_ns),
                ns_to_us_string(group.total_avg_ns),
                group.share_of_total * 100.0
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let release_gates = report
        .release_gates
        .iter()
        .map(|gate| {
            format!(
                "| {} | {} | {} |",
                gate.title,
                if gate.passed { "pass" } else { "fail" },
                gate.detail
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let outlier_table = report
        .outliers
        .iter()
        .map(|outlier| {
            format!(
                "| `{}` | `{}` | `{}` | {} | {:.1}x | {:.1}% |",
                outlier.stage_name,
                outlier.group,
                outlier.sample_index,
                ns_to_us_string(outlier.total_ns),
                outlier.multiplier,
                outlier.write_share_of_stage * 100.0
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let weak_points = report
        .weak_points
        .iter()
        .map(|weak_point| {
            format!(
                "### {}\n\n- Detail: {}\n- Fix path: {}\n",
                weak_point.title, weak_point.detail, weak_point.fix_hint
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "# Current Synthesis Benchmark\n\n\
         Date: unix `{}`\n\n\
         ## Release Headline\n\n\
         {}\n\n\
         ## Internal Caution\n\n\
         {}\n\n\
         ## Environment\n\n\
         - profile: `{}`\n\
         - artifact write mode: `{}`\n\
         - os: `{}`\n\
         - arch: `{}`\n\
         - logical cpus: `{}`\n\
         - benchmark binary size: `{}` bytes (`{}` KiB)\n\
         - hollow-grove binary size: {}\n\n\
         ## Benchmark Base\n\n\
         Hollow Grove was measured as a staged route rather than a single CLI print.\n\
         The benchmark follows the same downstream order the runtime uses:\n\n\
         ```text\n\
         kernel\n\
         ↓\n\
         clients\n\
         ↓\n\
         current_synthesis\n\
         ↓\n\
         hueman\n\
         ```\n\n\
         ## Sequence\n\n\
         - Warmup cycles: `{}`\n\
         - Measured cycles: `{}`\n\
         - Canonical witness locked: yes\n\
         - Canonical witness bytes: `{}`\n\
         - Drift count: `{}`\n\n\
         ## Release Gates\n\n\
         | Gate | Result | Detail |\n\
         | --- | --- | --- |\n\
         {}\n\n\
         ## Full Cycle\n\n\
         - Average runtime: `{}` ms\n\
         - P95 runtime: `{}` ms\n\
         - Min runtime: `{}` ms\n\
         - Max runtime: `{}` ms\n\
         - Local publish signal: `{}`\n\n\
         ## Stage Pressure\n\n\
         | Stage | Group | Build avg us | Write avg us | Total avg us | P95 us | Max us | CV | Write share | Output bytes |\n\
         | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |\n\
         {}\n\n\
         ## Group Load\n\n\
         | Group | Build avg us | Write avg us | Total avg us | Share of full cycle |\n\
         | --- | ---: | ---: | ---: | ---: |\n\
         {}\n\n\
         ## Outlier Watch\n\n\
         | Stage | Group | Worst sample | Worst us | Worst / avg | Write share |\n\
         | --- | --- | ---: | ---: | ---: | ---: |\n\
         {}\n\n\
         ## Weak Points\n\n\
         {}\n\
         ## Readiness\n\n\
         - `strong` means the route is ready to support a performance headline.\n\
         - `promising` means the route is publishable as supporting evidence, but not yet as the main performance story.\n\
         - `weak` means determinism, profile discipline, or timing stability are not ready for public claims.\n\n\
         ## Boundary Reminder\n\n\
         This report measures local systems behavior.\n\
         It does not prove audience demand, product fit, or cultural impact on its own.\n",
        report.generated_unix_time_s,
        report.release_headline,
        report.internal_caution,
        report.environment.profile,
        report.artifact_write_mode.as_str(),
        report.environment.os,
        report.environment.arch,
        report.environment.cpu_parallelism,
        report.environment.benchmark_binary_bytes,
        bytes_to_kib_string(report.environment.benchmark_binary_bytes),
        report
            .environment
            .hollow_grove_binary_bytes
            .map(|bytes| format!("`{}` bytes (`{}` KiB)", bytes, bytes_to_kib_string(bytes)))
            .unwrap_or_else(|| String::from("unavailable")),
        report.warmup,
        report.samples,
        CANONICAL_WITNESS.len(),
        report.drift_count,
        release_gates,
        ns_to_ms_string(report.total_avg_ns),
        ns_to_ms_string(report.total_p95_ns),
        ns_to_ms_string(report.total_min_ns),
        ns_to_ms_string(report.total_max_ns),
        report.publish_signal.as_str(),
        stage_table,
        group_table,
        outlier_table,
        weak_points
    )
}

fn render_benchmark_json(report: &BenchmarkReport) -> String {
    let stage_stats = report
        .stage_stats
        .iter()
        .map(|stage| {
            format!(
                "{{\"name\":\"{}\",\"group\":\"{}\",\"sample_count\":{},\"build_avg_ns\":{},\"write_avg_ns\":{},\"total_avg_ns\":{},\"total_p95_ns\":{},\"total_min_ns\":{},\"total_max_ns\":{},\"stddev_ns\":{:.3},\"cv\":{:.6},\"write_share_of_stage\":{:.6},\"output_bytes\":{},\"max_sample_index\":{},\"max_over_avg_ratio\":{:.6}}}",
                escape_json(stage.name),
                escape_json(stage.group),
                stage.sample_count,
                stage.build_avg_ns,
                stage.write_avg_ns,
                stage.total_avg_ns,
                stage.total_p95_ns,
                stage.total_min_ns,
                stage.total_max_ns,
                stage.stddev_ns,
                stage.coefficient_of_variation,
                stage.write_share_of_stage,
                stage.output_bytes,
                stage.max_sample_index,
                stage.max_over_avg_ratio
            )
        })
        .collect::<Vec<_>>()
        .join(",");

    let group_stats = report
        .group_stats
        .iter()
        .map(|group| {
            format!(
                "{{\"name\":\"{}\",\"build_avg_ns\":{},\"write_avg_ns\":{},\"total_avg_ns\":{},\"share_of_total\":{:.6}}}",
                escape_json(group.name),
                group.build_avg_ns,
                group.write_avg_ns,
                group.total_avg_ns,
                group.share_of_total
            )
        })
        .collect::<Vec<_>>()
        .join(",");

    let release_gates = report
        .release_gates
        .iter()
        .map(|gate| {
            format!(
                "{{\"title\":\"{}\",\"passed\":{},\"detail\":\"{}\"}}",
                escape_json(gate.title),
                gate.passed,
                escape_json(&gate.detail)
            )
        })
        .collect::<Vec<_>>()
        .join(",");

    let weak_points = report
        .weak_points
        .iter()
        .map(|weak_point| {
            format!(
                "{{\"title\":\"{}\",\"detail\":\"{}\",\"fix_hint\":\"{}\"}}",
                escape_json(&weak_point.title),
                escape_json(&weak_point.detail),
                escape_json(&weak_point.fix_hint)
            )
        })
        .collect::<Vec<_>>()
        .join(",");

    let outliers = report
        .outliers
        .iter()
        .map(|outlier| {
            format!(
                "{{\"stage_name\":\"{}\",\"group\":\"{}\",\"sample_index\":{},\"total_ns\":{},\"avg_ns\":{},\"multiplier\":{:.6},\"write_share_of_stage\":{:.6}}}",
                escape_json(outlier.stage_name),
                escape_json(outlier.group),
                outlier.sample_index,
                outlier.total_ns,
                outlier.avg_ns,
                outlier.multiplier,
                outlier.write_share_of_stage
            )
        })
        .collect::<Vec<_>>()
        .join(",");

    format!(
        "{{\"generated_unix_time_s\":{},\"environment\":{{\"profile\":\"{}\",\"artifact_write_mode\":\"{}\",\"os\":\"{}\",\"arch\":\"{}\",\"cpu_parallelism\":{},\"benchmark_binary_bytes\":{},\"hollow_grove_binary_bytes\":{}}},\"warmup\":{},\"samples\":{},\"drift_count\":{},\"publish_signal\":\"{}\",\"release_headline\":\"{}\",\"internal_caution\":\"{}\",\"total_avg_ns\":{},\"total_p95_ns\":{},\"total_min_ns\":{},\"total_max_ns\":{},\"stage_stats\":[{}],\"group_stats\":[{}],\"release_gates\":[{}],\"outliers\":[{}],\"weak_points\":[{}]}}",
        report.generated_unix_time_s,
        escape_json(report.environment.profile),
        report.artifact_write_mode.as_str(),
        escape_json(report.environment.os),
        escape_json(report.environment.arch),
        report.environment.cpu_parallelism,
        report.environment.benchmark_binary_bytes,
        report
            .environment
            .hollow_grove_binary_bytes
            .map(|bytes| bytes.to_string())
            .unwrap_or_else(|| String::from("null")),
        report.warmup,
        report.samples,
        report.drift_count,
        report.publish_signal.as_str(),
        escape_json(&report.release_headline),
        escape_json(&report.internal_caution),
        report.total_avg_ns,
        report.total_p95_ns,
        report.total_min_ns,
        report.total_max_ns,
        stage_stats,
        group_stats,
        release_gates,
        outliers,
        weak_points
    )
}

fn render_benchmark_release_markdown(report: &BenchmarkReport) -> String {
    let release_gates = report
        .release_gates
        .iter()
        .map(|gate| {
            format!(
                "- {}: {} ({})",
                gate.title,
                if gate.passed { "pass" } else { "fail" },
                gate.detail
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let weak_points = report
        .weak_points
        .iter()
        .map(|weak_point| format!("- {}: {}", weak_point.title, weak_point.detail))
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "# Current Synthesis Benchmark Release Summary\n\n\
         ## Claim\n\n\
         {}\n\n\
         ## Caution\n\n\
         {}\n\n\
         ## Release Gates\n\n\
         {}\n\n\
         ## Broadcast Metrics\n\n\
         - profile: `{}`\n\
         - artifact write mode: `{}`\n\
         - average full-route runtime: `{}` ms\n\
         - p95 full-route runtime: `{}` ms\n\
         - max full-route runtime: `{}` ms\n\
         - drift count: `{}`\n\
         - publish signal: `{}`\n\n\
         ## Main Weak Points\n\n\
         {}\n\n\
         ## Positioning\n\n\
         Use this benchmark as evidence of deterministic systems discipline.\n\
         Do not treat it as proof of mass appeal by itself.\n",
        report.release_headline,
        report.internal_caution,
        release_gates,
        report.environment.profile,
        report.artifact_write_mode.as_str(),
        ns_to_ms_string(report.total_avg_ns),
        ns_to_ms_string(report.total_p95_ns),
        ns_to_ms_string(report.total_max_ns),
        report.drift_count,
        report.publish_signal.as_str(),
        weak_points
    )
}

fn write_benchmark_artifacts(root: &Path, report: &BenchmarkReport) -> io::Result<[PathBuf; 3]> {
    let markdown_path = root.join(BENCHMARK_REPORT_ARTIFACT_PATH);
    let json_path = root.join(BENCHMARK_SNAPSHOT_ARTIFACT_PATH);
    let release_path = root.join(BENCHMARK_RELEASE_ARTIFACT_PATH);

    write_text_artifact(&markdown_path, &render_benchmark_markdown(report))?;
    write_text_artifact(&json_path, &render_benchmark_json(report))?;
    write_text_artifact(&release_path, &render_benchmark_release_markdown(report))?;

    Ok([markdown_path, json_path, release_path])
}

fn main() -> io::Result<()> {
    let cli = parse_benchmark_cli(env::args().skip(1))
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidInput, error))?;

    let config = match cli {
        BenchmarkCli::Help => {
            println!("{}", usage());
            return Ok(());
        }
        BenchmarkCli::Run(config) => config,
    };

    let root = Path::new(".");
    let paths = BenchmarkPaths::new(root);
    ensure_artifact_index(&paths.artifact_index)?;

    for sample_index in 0..config.warmup {
        run_pipeline_sample_with_paths(&paths, sample_index, config.artifact_write_mode)?;
    }

    let mut samples = Vec::with_capacity(config.samples);
    for sample_offset in 0..config.samples {
        let sample_index = config.warmup + sample_offset;
        let sample =
            run_pipeline_sample_with_paths(&paths, sample_index, config.artifact_write_mode)?;
        if !config.quiet {
            println!(
                "benchmark sample {}/{} complete in {} us",
                sample_offset + 1,
                config.samples,
                ns_to_us_string(sample.total_elapsed.as_nanos())
            );
        }
        samples.push(sample);
    }

    let environment = capture_environment()?;
    let report = build_benchmark_report(environment, unix_time_seconds()?, config, &samples);
    let [markdown_path, json_path, release_path] = write_benchmark_artifacts(root, &report)?;

    if !config.quiet {
        println!("{}", markdown_path.display());
        println!("{}", json_path.display());
        println!("{}", release_path.display());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        ArtifactWriteMode, BenchmarkCli, BenchmarkConfig, DEFAULT_SAMPLES, DEFAULT_WARMUP,
        build_benchmark_report, capture_environment, parse_benchmark_cli,
        render_benchmark_markdown, render_benchmark_release_markdown, run_pipeline_sample, usage,
    };
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_temp_dir(prefix: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("{prefix}-{nonce}"))
    }

    #[test]
    fn benchmark_cli_defaults_and_help_remain_stable() {
        let cli = parse_benchmark_cli(std::iter::empty::<String>()).expect("cli should parse");
        assert_eq!(
            cli,
            BenchmarkCli::Run(BenchmarkConfig {
                warmup: DEFAULT_WARMUP,
                samples: DEFAULT_SAMPLES,
                artifact_write_mode: ArtifactWriteMode::Disk,
                quiet: false,
            })
        );

        let cli = parse_benchmark_cli([String::from("--help")]).expect("help should parse");
        assert_eq!(cli, BenchmarkCli::Help);
        assert_eq!(
            usage(),
            "Usage: current_synthesis_benchmark [--warmup N] [--samples N] [--no-write] [--quiet]"
        );
    }

    #[test]
    fn benchmark_cli_parses_custom_values() {
        let cli = parse_benchmark_cli([
            String::from("--warmup"),
            String::from("2"),
            String::from("--samples"),
            String::from("7"),
            String::from("--no-write"),
            String::from("--quiet"),
        ])
        .expect("cli should parse");

        assert_eq!(
            cli,
            BenchmarkCli::Run(BenchmarkConfig {
                warmup: 2,
                samples: 7,
                artifact_write_mode: ArtifactWriteMode::Memory,
                quiet: true,
            })
        );
    }

    #[test]
    fn pipeline_sample_writes_current_synthesis_and_hueman_outputs() {
        let root = unique_temp_dir("hollow-grove-benchmark-sample");
        fs::create_dir_all(&root).expect("temp root should create");

        let sample = run_pipeline_sample(&root, 0, ArtifactWriteMode::Disk)
            .expect("pipeline sample should run");
        assert!(sample.witness.contains("AuraBeam"));
        assert!(sample.witness.contains("Point² (Landed Point)"));
        assert!(
            sample
                .activation_gate
                .contains("# Current Synthesis Activation Gate")
        );
        assert!(sample.scene_drift.contains("# Hueman Scene Drift"));
        assert!(
            sample
                .stages
                .iter()
                .any(|stage| stage.write_elapsed > std::time::Duration::ZERO)
        );

        fs::remove_dir_all(&root).expect("temp cleanup should succeed");
    }

    #[test]
    fn pipeline_sample_can_skip_artifact_flushes() {
        let root = unique_temp_dir("hollow-grove-benchmark-sample-no-write");
        fs::create_dir_all(&root).expect("temp root should create");

        let sample = run_pipeline_sample(&root, 0, ArtifactWriteMode::Memory)
            .expect("pipeline sample should run without flush");

        assert!(sample.witness.contains("AuraBeam"));
        assert!(sample.witness.contains("Point² (Landed Point)"));
        assert!(
            sample
                .stages
                .iter()
                .all(|stage| stage.write_elapsed == std::time::Duration::ZERO)
        );

        fs::remove_dir_all(&root).expect("temp cleanup should succeed");
    }

    #[test]
    fn benchmark_report_surfaces_release_sections_and_weak_points() {
        let root = unique_temp_dir("hollow-grove-benchmark-report");
        fs::create_dir_all(&root).expect("temp root should create");

        let sample_one = run_pipeline_sample(&root, 0, ArtifactWriteMode::Disk)
            .expect("first sample should run");
        let sample_two = run_pipeline_sample(&root, 1, ArtifactWriteMode::Disk)
            .expect("second sample should run");
        let report = build_benchmark_report(
            capture_environment().expect("environment should capture"),
            super::unix_time_seconds().expect("time should work"),
            BenchmarkConfig {
                warmup: 0,
                samples: 2,
                artifact_write_mode: ArtifactWriteMode::Disk,
                quiet: true,
            },
            &[sample_one, sample_two],
        );
        let markdown = render_benchmark_markdown(&report);
        let release_markdown = render_benchmark_release_markdown(&report);

        assert!(markdown.contains("# Current Synthesis Benchmark"));
        assert!(markdown.contains("## Release Gates"));
        assert!(markdown.contains("## Outlier Watch"));
        assert!(markdown.contains("## Weak Points"));
        assert!(markdown.contains("- artifact write mode: `disk`"));
        assert!(markdown.contains("Latency Concentration") || markdown.contains("Latency Spread"));
        assert!(markdown.contains("Determinism Check"));
        assert!(release_markdown.contains("# Current Synthesis Benchmark Release Summary"));
        assert!(release_markdown.contains("## Claim"));
        assert!(release_markdown.contains("## Main Weak Points"));
        assert!(release_markdown.contains("- artifact write mode: `disk`"));

        fs::remove_dir_all(&root).expect("temp cleanup should succeed");
    }
}
