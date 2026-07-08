use std::env;
use std::io;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

#[path = "../current_synthesis_support.rs"]
mod current_synthesis_support;

use current_synthesis_support::{
    ARTIFACT_INDEX_PATH, CURRENT_SYNTHESIS_ACTIVATION_GATE_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_BASE_ARTIFACT_PATH, CURRENT_SYNTHESIS_BEHAVIOR_RULES_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_CHOICE_ARTIFACT_PATH, CURRENT_SYNTHESIS_CLIENTS_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_CONSEQUENCE_ARTIFACT_PATH, CURRENT_SYNTHESIS_CONTRACT_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_EXECUTION_SPEC_ARTIFACT_PATH, CURRENT_SYNTHESIS_OPERATIONAL_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_PREVIEW_ARTIFACT_PATH, CURRENT_SYNTHESIS_READINESS_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_SELECTION_ARTIFACT_PATH, CURRENT_SYNTHESIS_SEQUENCE_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_STATE_ARTIFACT_PATH, CURRENT_SYNTHESIS_TOPOLOGY_ARTIFACT_PATH,
    CURRENT_SYNTHESIS_TRANSITION_PM_TO_LE_ARTIFACT_PATH, DESKTOP_STATUS_ARTIFACT_PATH,
    PROMPT_ARTIFACT_PATH, SNAPSHOT_ARTIFACT_PATH,
    build_current_synthesis_activation_gate_from_artifacts,
    build_current_synthesis_base_from_artifacts,
    build_current_synthesis_behavior_rules_from_artifacts,
    build_current_synthesis_choice_from_artifacts, build_current_synthesis_clients_from_artifacts,
    build_current_synthesis_consequence_from_artifacts,
    build_current_synthesis_contract_from_artifacts,
    build_current_synthesis_execution_spec_from_artifacts,
    build_current_synthesis_operational_from_artifacts,
    build_current_synthesis_preview_from_artifacts,
    build_current_synthesis_readiness_from_artifacts,
    build_current_synthesis_selection_from_artifacts,
    build_current_synthesis_sequence_from_artifacts, build_current_synthesis_state_from_artifacts,
    build_current_synthesis_topology_from_artifacts,
    build_current_synthesis_transition_pm_to_le_from_artifacts, ensure_artifact_index,
    read_artifact,
};
use hollow_grove::hueman_support::{
    build_hueman_archetype_lens_from_artifacts, build_hueman_aura_behavior_from_artifacts,
    build_hueman_aura_triad_from_artifacts, build_hueman_boundary_from_artifacts,
    build_hueman_crossover_scenes_from_artifacts, build_hueman_fourway_from_artifacts,
    build_hueman_glaushouse_roles_from_artifacts, build_hueman_inverse_circle_from_artifacts,
    build_hueman_link_physics_from_artifacts, build_hueman_motion_map_from_artifacts,
    build_hueman_path_crossovers_from_artifacts, build_hueman_procedural_uplift_from_artifacts,
    build_hueman_sandmanor_roles_from_artifacts, build_hueman_scene_drift_from_artifacts,
    build_hueman_scene_intent_from_artifacts, build_hueman_scene_presence_from_artifacts,
    build_hueman_start_choices_from_artifacts, build_hueman_start_paths_from_artifacts,
    build_hueman_stonebend_roles_from_artifacts, build_hueman_tross_helpers_from_artifacts,
    build_vertical_integration_stack_from_artifacts, hueman_archetype_lens_artifact_path,
    hueman_aura_behavior_artifact_path, hueman_aura_triad_artifact_path,
    hueman_boundary_artifact_path, hueman_crossover_scenes_artifact_path,
    hueman_fourway_artifact_path, hueman_glaushouse_roles_artifact_path,
    hueman_inverse_circle_artifact_path, hueman_link_physics_artifact_path,
    hueman_motion_map_artifact_path, hueman_path_crossovers_artifact_path,
    hueman_procedural_uplift_artifact_path, hueman_sandmanor_roles_artifact_path,
    hueman_scene_drift_artifact_path, hueman_scene_intent_artifact_path,
    hueman_scene_presence_artifact_path, hueman_start_choices_artifact_path,
    hueman_start_paths_artifact_path, hueman_stonebend_roles_artifact_path,
    hueman_tross_helpers_artifact_path, vertical_integration_stack_artifact_path,
};
use hollow_grove::{
    ArtifactSession, Symptom, build_desktop_status_output, build_prompt_artifact_output,
    build_snapshot_output, read_text_artifact, run_kernel_cycle, write_text_artifact,
};

const RUNTIME_INPUT_ARTIFACT_PATH: &str = "artifacts/runtime_input.txt";
const RUNTIME_MEMORY_ARTIFACT_PATH: &str = "artifacts/runtime_memory.txt";
const RUNTIME_LOOP_STATUS_ARTIFACT_PATH: &str = "artifacts/runtime_loop_status.md";
const DEFAULT_INTERVAL_MS: u64 = 1_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RuntimeConfig {
    cycles: Option<usize>,
    interval: Duration,
    quiet: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum RuntimeCli {
    Help,
    Run(RuntimeConfig),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RuntimeMode {
    Run,
    Hold,
    Stop,
}

impl RuntimeMode {
    fn as_str(self) -> &'static str {
        match self {
            Self::Run => "run",
            Self::Hold => "hold",
            Self::Stop => "stop",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RuntimeInput {
    mode: RuntimeMode,
    origin: String,
    operator_note: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RuntimeMemory {
    last_cycle: usize,
    last_unix_time_s: u64,
    last_runtime_mode: RuntimeMode,
    last_action_taken: String,
    last_origin: String,
    last_operator_note: String,
    last_should_stop: bool,
    last_witness: Option<String>,
}

#[derive(Debug)]
struct RuntimeCycleResult {
    cycle_number: usize,
    elapsed: Duration,
    mode: RuntimeMode,
    action_taken: &'static str,
    should_stop: bool,
    status_path: PathBuf,
}

fn parse_runtime_cli<I>(args: I) -> Result<RuntimeCli, String>
where
    I: IntoIterator<Item = String>,
{
    let mut cycles = None;
    let mut interval = Duration::from_millis(DEFAULT_INTERVAL_MS);
    let mut quiet = false;
    let mut args = args.into_iter();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--help" | "-h" => return Ok(RuntimeCli::Help),
            "--quiet" => quiet = true,
            "--cycles" => {
                let value = args
                    .next()
                    .ok_or_else(|| String::from("missing value for --cycles"))?;
                let parsed = value
                    .parse::<usize>()
                    .map_err(|_| format!("invalid usize for --cycles: {value}"))?;
                if parsed == 0 {
                    return Err(String::from("--cycles must be greater than zero"));
                }
                cycles = Some(parsed);
            }
            "--interval-ms" => {
                let value = args
                    .next()
                    .ok_or_else(|| String::from("missing value for --interval-ms"))?;
                let parsed = value
                    .parse::<u64>()
                    .map_err(|_| format!("invalid u64 for --interval-ms: {value}"))?;
                interval = Duration::from_millis(parsed);
            }
            other => return Err(format!("unknown argument: {other}")),
        }
    }

    Ok(RuntimeCli::Run(RuntimeConfig {
        cycles,
        interval,
        quiet,
    }))
}

fn usage() -> &'static str {
    "Usage: hollow_grove_runtime [--cycles N] [--interval-ms N] [--quiet]"
}

fn system_time_unix_seconds() -> io::Result<u64> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| io::Error::other(format!("system time error: {error}")))?
        .as_secs())
}

fn build_runtime_input_template() -> String {
    String::from(
        "# Hollow Grove Runtime Input\n\
         # runtime_mode: run | hold | stop\n\
         # origin: symptom-origin\n\n\
         runtime_mode: run\n\
         origin: symptom-origin\n\
         operator_note: default open loop\n",
    )
}

fn parse_runtime_mode(value: &str) -> io::Result<RuntimeMode> {
    match value {
        "run" => Ok(RuntimeMode::Run),
        "hold" => Ok(RuntimeMode::Hold),
        "stop" => Ok(RuntimeMode::Stop),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("invalid runtime_mode: {value}"),
        )),
    }
}

fn parse_runtime_input(contents: &str) -> io::Result<RuntimeInput> {
    let mut mode = None;
    let mut origin = None;
    let mut operator_note = None;
    let mut seen_unknown = Vec::new();

    for raw_line in contents.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let (key, value) = line.split_once(':').ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("runtime input line is missing ':' separator: {line}"),
            )
        })?;
        let key = key.trim();
        let value = value.trim();

        match key {
            "runtime_mode" => {
                if mode.is_some() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "runtime input contains duplicate runtime_mode",
                    ));
                }
                mode = Some(parse_runtime_mode(value)?);
            }
            "origin" => {
                if origin.is_some() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "runtime input contains duplicate origin",
                    ));
                }
                origin = Some(value.to_owned());
            }
            "operator_note" => {
                if operator_note.is_some() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "runtime input contains duplicate operator_note",
                    ));
                }
                operator_note = Some(value.to_owned());
            }
            other => seen_unknown.push(other.to_owned()),
        }
    }

    if let Some(unknown_key) = seen_unknown.first() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("runtime input contains unknown key: {unknown_key}"),
        ));
    }

    let origin = origin.unwrap_or_else(|| String::from("symptom-origin"));
    if origin != "symptom-origin" {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("invalid origin: {origin}"),
        ));
    }

    Ok(RuntimeInput {
        mode: mode.unwrap_or(RuntimeMode::Run),
        origin,
        operator_note: operator_note.unwrap_or_else(|| String::from("default open loop")),
    })
}

fn read_or_create_runtime_input_at(root: &Path) -> io::Result<RuntimeInput> {
    let input_path = root.join(RUNTIME_INPUT_ARTIFACT_PATH);

    match read_text_artifact(&input_path) {
        Ok(contents) => parse_runtime_input(&contents),
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            let template = build_runtime_input_template();
            write_text_artifact(&input_path, &template)?;
            parse_runtime_input(&template)
        }
        Err(error) => Err(error),
    }
}

fn read_existing_witness_at(root: &Path) -> io::Result<Option<String>> {
    match read_text_artifact(&root.join(DESKTOP_STATUS_ARTIFACT_PATH)) {
        Ok(desktop_status) => current_synthesis_support::extract_canonical_witness(&desktop_status)
            .map(|witness| Some(witness.to_owned())),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error),
    }
}

fn resolve_visible_witness(
    root: &Path,
    previous_memory: Option<&RuntimeMemory>,
) -> io::Result<Option<String>> {
    let current_witness = read_existing_witness_at(root)?;
    Ok(current_witness.or_else(|| previous_memory.and_then(|memory| memory.last_witness.clone())))
}

fn escape_runtime_memory_value(value: &str) -> String {
    value.replace('\\', "\\\\").replace('\n', "\\n")
}

fn unescape_runtime_memory_value(value: &str) -> String {
    let mut output = String::new();
    let mut chars = value.chars();

    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some('n') => output.push('\n'),
                Some('\\') => output.push('\\'),
                Some(other) => {
                    output.push('\\');
                    output.push(other);
                }
                None => output.push('\\'),
            }
        } else {
            output.push(ch);
        }
    }

    output
}

fn parse_runtime_bool(value: &str, field: &str) -> io::Result<bool> {
    match value {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("invalid bool for {field}: {value}"),
        )),
    }
}

fn build_runtime_memory_output(memory: &RuntimeMemory) -> String {
    let witness = memory
        .last_witness
        .as_deref()
        .map(escape_runtime_memory_value)
        .unwrap_or_else(|| String::from("(none)"));

    format!(
        "# Hollow Grove Runtime Memory\n\
         last_cycle: {}\n\
         last_unix_time_s: {}\n\
         last_runtime_mode: {}\n\
         last_action_taken: {}\n\
         last_origin: {}\n\
         last_operator_note: {}\n\
         last_should_stop: {}\n\
         last_witness: {}\n",
        memory.last_cycle,
        memory.last_unix_time_s,
        memory.last_runtime_mode.as_str(),
        escape_runtime_memory_value(&memory.last_action_taken),
        escape_runtime_memory_value(&memory.last_origin),
        escape_runtime_memory_value(&memory.last_operator_note),
        memory.last_should_stop,
        witness
    )
}

fn parse_runtime_memory(contents: &str) -> io::Result<RuntimeMemory> {
    let mut last_cycle = None;
    let mut last_unix_time_s = None;
    let mut last_runtime_mode = None;
    let mut last_action_taken = None;
    let mut last_origin = None;
    let mut last_operator_note = None;
    let mut last_should_stop = None;
    let mut last_witness = None;
    let mut seen_unknown = Vec::new();

    for raw_line in contents.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let (key, value) = line.split_once(':').ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("runtime memory line is missing ':' separator: {line}"),
            )
        })?;
        let key = key.trim();
        let value = value.trim();

        match key {
            "last_cycle" => {
                if last_cycle.is_some() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "runtime memory contains duplicate last_cycle",
                    ));
                }
                last_cycle = Some(value.parse::<usize>().map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("invalid usize for last_cycle: {value}"),
                    )
                })?)
            }
            "last_unix_time_s" => {
                if last_unix_time_s.is_some() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "runtime memory contains duplicate last_unix_time_s",
                    ));
                }
                last_unix_time_s = Some(value.parse::<u64>().map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("invalid u64 for last_unix_time_s: {value}"),
                    )
                })?)
            }
            "last_runtime_mode" => {
                if last_runtime_mode.is_some() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "runtime memory contains duplicate last_runtime_mode",
                    ));
                }
                last_runtime_mode = Some(parse_runtime_mode(value)?);
            }
            "last_action_taken" => {
                if last_action_taken.is_some() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "runtime memory contains duplicate last_action_taken",
                    ));
                }
                last_action_taken = Some(unescape_runtime_memory_value(value));
            }
            "last_origin" => {
                if last_origin.is_some() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "runtime memory contains duplicate last_origin",
                    ));
                }
                last_origin = Some(unescape_runtime_memory_value(value));
            }
            "last_operator_note" => {
                if last_operator_note.is_some() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "runtime memory contains duplicate last_operator_note",
                    ));
                }
                last_operator_note = Some(unescape_runtime_memory_value(value));
            }
            "last_should_stop" => {
                if last_should_stop.is_some() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "runtime memory contains duplicate last_should_stop",
                    ));
                }
                last_should_stop = Some(parse_runtime_bool(value, key)?);
            }
            "last_witness" => {
                if last_witness.is_some() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "runtime memory contains duplicate last_witness",
                    ));
                }
                last_witness = Some(if value == "(none)" {
                    None
                } else {
                    Some(unescape_runtime_memory_value(value))
                })
            }
            other => seen_unknown.push(other.to_owned()),
        }
    }

    if let Some(unknown_key) = seen_unknown.first() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("runtime memory contains unknown key: {unknown_key}"),
        ));
    }

    Ok(RuntimeMemory {
        last_cycle: last_cycle.ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "runtime memory missing last_cycle",
            )
        })?,
        last_unix_time_s: last_unix_time_s.ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "runtime memory missing last_unix_time_s",
            )
        })?,
        last_runtime_mode: last_runtime_mode.ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "runtime memory missing last_runtime_mode",
            )
        })?,
        last_action_taken: last_action_taken.ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "runtime memory missing last_action_taken",
            )
        })?,
        last_origin: last_origin.ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "runtime memory missing last_origin",
            )
        })?,
        last_operator_note: last_operator_note.ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "runtime memory missing last_operator_note",
            )
        })?,
        last_should_stop: last_should_stop.ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "runtime memory missing last_should_stop",
            )
        })?,
        last_witness: last_witness.ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "runtime memory missing last_witness",
            )
        })?,
    })
}

fn read_runtime_memory_at(root: &Path) -> io::Result<Option<RuntimeMemory>> {
    match read_text_artifact(&root.join(RUNTIME_MEMORY_ARTIFACT_PATH)) {
        Ok(contents) => parse_runtime_memory(&contents).map(Some),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error),
    }
}

fn write_runtime_memory_at(root: &Path, memory: &RuntimeMemory) -> io::Result<PathBuf> {
    let path = root.join(RUNTIME_MEMORY_ARTIFACT_PATH);
    let output = build_runtime_memory_output(memory);
    write_text_artifact(&path, &output)?;
    Ok(path)
}

fn runtime_resume_cycle_at(root: &Path) -> io::Result<usize> {
    Ok(read_runtime_memory_at(root)?
        .map(|memory| memory.last_cycle)
        .unwrap_or(0))
}

fn build_runtime_status_output(
    cycle_number: usize,
    elapsed: Duration,
    timestamp_unix_seconds: u64,
    runtime_input: &RuntimeInput,
    action_taken: &str,
    witness: Option<&str>,
    previous_memory: Option<&RuntimeMemory>,
) -> String {
    let witness = witness.unwrap_or("(no completed witness available)");
    let coverage = match runtime_input.mode {
        RuntimeMode::Run => {
            "- kernel artifacts refreshed\n\
             - Current Synthesis artifacts refreshed\n\
             - Hueman artifacts refreshed"
        }
        RuntimeMode::Hold | RuntimeMode::Stop => {
            "- kernel artifacts unchanged\n\
             - Current Synthesis artifacts unchanged\n\
             - Hueman artifacts unchanged"
        }
    };
    let memory_summary = match previous_memory {
        Some(memory) => format!(
            "- previous_cycle: {}\n\
             - previous_mode: {}\n\
             - previous_action: {}\n\
             - previous_witness_available: {}",
            memory.last_cycle,
            memory.last_runtime_mode.as_str(),
            memory.last_action_taken,
            if memory.last_witness.is_some() {
                "yes"
            } else {
                "no"
            }
        ),
        None => String::from(
            "- previous_cycle: none\n\
             - previous_mode: none\n\
             - previous_action: none\n\
             - previous_witness_available: no",
        ),
    };

    format!(
        "# Hollow Grove Runtime Loop\n\n\
         ## Status\n\n\
         - cycle: {cycle_number}\n\
         - unix_time_s: {timestamp_unix_seconds}\n\
         - elapsed_ms: {}\n\
         - runtime_mode: {}\n\
         - action_taken: {action_taken}\n\
         - origin: {}\n\
         - input_contract: `{RUNTIME_INPUT_ARTIFACT_PATH}`\n\
         - memory_contract: `{RUNTIME_MEMORY_ARTIFACT_PATH}`\n\
         - root_origin: `Symptom::origin()`\n\n\
         ## Previous Memory\n\n\
         {memory_summary}\n\n\
         ## Operator Note\n\n\
         {}\n\n\
         ## Canonical Witness\n\n\
         ```text\n\
         {witness}\n\
         ```\n\n\
         ## Coverage\n\n\
        {coverage}\n",
        elapsed.as_millis(),
        runtime_input.mode.as_str(),
        runtime_input.origin,
        runtime_input.operator_note
    )
}

fn stage_artifact(session: &mut ArtifactSession, path: &Path, contents: impl Into<String>) {
    session.stage_text_artifact(path, contents);
}

fn read_artifact_in_session(
    session: &ArtifactSession,
    root: &Path,
    relative_path: impl AsRef<Path>,
) -> io::Result<String> {
    session.read_text_artifact(&root.join(relative_path))
}

fn run_current_synthesis_at(
    root: &Path,
    session: &mut ArtifactSession,
) -> io::Result<[PathBuf; 16]> {
    let snapshot = read_artifact_in_session(session, root, SNAPSHOT_ARTIFACT_PATH)?;
    let prompt = read_artifact_in_session(session, root, PROMPT_ARTIFACT_PATH)?;
    let desktop_status = read_artifact_in_session(session, root, DESKTOP_STATUS_ARTIFACT_PATH)?;
    let current_synthesis_base =
        build_current_synthesis_base_from_artifacts(&snapshot, &prompt, &desktop_status)?;
    let base_path = root.join(CURRENT_SYNTHESIS_BASE_ARTIFACT_PATH);
    stage_artifact(session, &base_path, current_synthesis_base.clone());

    let artifact_index_path = root.join(ARTIFACT_INDEX_PATH);
    ensure_artifact_index(&artifact_index_path)?;
    let artifact_index = read_artifact(&artifact_index_path)?;
    let current_synthesis_state =
        build_current_synthesis_state_from_artifacts(&current_synthesis_base, &artifact_index);
    let state_path = root.join(CURRENT_SYNTHESIS_STATE_ARTIFACT_PATH);
    stage_artifact(session, &state_path, current_synthesis_state.clone());

    let current_synthesis_sequence = build_current_synthesis_sequence_from_artifacts(
        &current_synthesis_base,
        &current_synthesis_state,
    );
    let sequence_path = root.join(CURRENT_SYNTHESIS_SEQUENCE_ARTIFACT_PATH);
    stage_artifact(session, &sequence_path, current_synthesis_sequence.clone());

    let current_synthesis_topology = build_current_synthesis_topology_from_artifacts(
        &current_synthesis_sequence,
        &current_synthesis_state,
    );
    let topology_path = root.join(CURRENT_SYNTHESIS_TOPOLOGY_ARTIFACT_PATH);
    stage_artifact(session, &topology_path, current_synthesis_topology.clone());

    let current_synthesis_clients = build_current_synthesis_clients_from_artifacts(
        &current_synthesis_topology,
        &current_synthesis_sequence,
    );
    let clients_path = root.join(CURRENT_SYNTHESIS_CLIENTS_ARTIFACT_PATH);
    stage_artifact(session, &clients_path, current_synthesis_clients.clone());

    let current_synthesis_choice = build_current_synthesis_choice_from_artifacts(
        &current_synthesis_clients,
        &current_synthesis_topology,
    );
    let choice_path = root.join(CURRENT_SYNTHESIS_CHOICE_ARTIFACT_PATH);
    stage_artifact(session, &choice_path, current_synthesis_choice.clone());

    let current_synthesis_contract = build_current_synthesis_contract_from_artifacts(
        &current_synthesis_choice,
        &current_synthesis_clients,
    );
    let contract_path = root.join(CURRENT_SYNTHESIS_CONTRACT_ARTIFACT_PATH);
    stage_artifact(session, &contract_path, current_synthesis_contract.clone());

    let current_synthesis_preview = build_current_synthesis_preview_from_artifacts(
        &current_synthesis_contract,
        &current_synthesis_sequence,
    );
    let preview_path = root.join(CURRENT_SYNTHESIS_PREVIEW_ARTIFACT_PATH);
    stage_artifact(session, &preview_path, current_synthesis_preview.clone());

    let current_synthesis_operational = build_current_synthesis_operational_from_artifacts(
        &current_synthesis_preview,
        &current_synthesis_contract,
    );
    let operational_path = root.join(CURRENT_SYNTHESIS_OPERATIONAL_ARTIFACT_PATH);
    stage_artifact(
        session,
        &operational_path,
        current_synthesis_operational.clone(),
    );

    let current_synthesis_selection = build_current_synthesis_selection_from_artifacts(
        &current_synthesis_choice,
        &current_synthesis_operational,
    );
    let selection_path = root.join(CURRENT_SYNTHESIS_SELECTION_ARTIFACT_PATH);
    stage_artifact(
        session,
        &selection_path,
        current_synthesis_selection.clone(),
    );

    let current_synthesis_consequence = build_current_synthesis_consequence_from_artifacts(
        &current_synthesis_selection,
        &current_synthesis_operational,
    );
    let consequence_path = root.join(CURRENT_SYNTHESIS_CONSEQUENCE_ARTIFACT_PATH);
    stage_artifact(
        session,
        &consequence_path,
        current_synthesis_consequence.clone(),
    );

    let current_synthesis_readiness = build_current_synthesis_readiness_from_artifacts(
        &current_synthesis_consequence,
        &current_synthesis_selection,
    );
    let readiness_path = root.join(CURRENT_SYNTHESIS_READINESS_ARTIFACT_PATH);
    stage_artifact(
        session,
        &readiness_path,
        current_synthesis_readiness.clone(),
    );

    let current_synthesis_execution_spec = build_current_synthesis_execution_spec_from_artifacts(
        &current_synthesis_readiness,
        &current_synthesis_consequence,
    );
    let execution_spec_path = root.join(CURRENT_SYNTHESIS_EXECUTION_SPEC_ARTIFACT_PATH);
    stage_artifact(
        session,
        &execution_spec_path,
        current_synthesis_execution_spec.clone(),
    );

    let current_synthesis_behavior_rules = build_current_synthesis_behavior_rules_from_artifacts(
        &current_synthesis_execution_spec,
        &current_synthesis_selection,
    );
    let behavior_rules_path = root.join(CURRENT_SYNTHESIS_BEHAVIOR_RULES_ARTIFACT_PATH);
    stage_artifact(
        session,
        &behavior_rules_path,
        current_synthesis_behavior_rules.clone(),
    );

    let current_synthesis_transition_pm_to_le =
        build_current_synthesis_transition_pm_to_le_from_artifacts(
            &current_synthesis_behavior_rules,
            &current_synthesis_topology,
        );
    let transition_pm_to_le_path = root.join(CURRENT_SYNTHESIS_TRANSITION_PM_TO_LE_ARTIFACT_PATH);
    stage_artifact(
        session,
        &transition_pm_to_le_path,
        current_synthesis_transition_pm_to_le.clone(),
    );

    let current_synthesis_activation_gate = build_current_synthesis_activation_gate_from_artifacts(
        &current_synthesis_transition_pm_to_le,
        &current_synthesis_readiness,
    );
    let activation_gate_path = root.join(CURRENT_SYNTHESIS_ACTIVATION_GATE_ARTIFACT_PATH);
    stage_artifact(
        session,
        &activation_gate_path,
        current_synthesis_activation_gate,
    );

    Ok([
        base_path,
        state_path,
        sequence_path,
        topology_path,
        clients_path,
        choice_path,
        contract_path,
        preview_path,
        operational_path,
        selection_path,
        consequence_path,
        readiness_path,
        execution_spec_path,
        behavior_rules_path,
        transition_pm_to_le_path,
        activation_gate_path,
    ])
}

fn run_hueman_at(root: &Path, session: &mut ArtifactSession) -> io::Result<[PathBuf; 21]> {
    let current_synthesis_base =
        read_artifact_in_session(session, root, CURRENT_SYNTHESIS_BASE_ARTIFACT_PATH)?;
    let current_synthesis_activation_gate = read_artifact_in_session(
        session,
        root,
        CURRENT_SYNTHESIS_ACTIVATION_GATE_ARTIFACT_PATH,
    )?;
    let hueman_boundary = build_hueman_boundary_from_artifacts(
        &current_synthesis_base,
        &current_synthesis_activation_gate,
    );
    let boundary_path = root.join(hueman_boundary_artifact_path());
    stage_artifact(session, &boundary_path, hueman_boundary.clone());

    let current_synthesis_operational =
        read_artifact_in_session(session, root, CURRENT_SYNTHESIS_OPERATIONAL_ARTIFACT_PATH)?;
    let hueman_motion_map =
        build_hueman_motion_map_from_artifacts(&hueman_boundary, &current_synthesis_operational);
    let motion_map_path = root.join(hueman_motion_map_artifact_path());
    stage_artifact(session, &motion_map_path, hueman_motion_map.clone());

    let hueman_fourway = build_hueman_fourway_from_artifacts(&hueman_boundary, &hueman_motion_map);
    let fourway_path = root.join(hueman_fourway_artifact_path());
    stage_artifact(session, &fourway_path, hueman_fourway.clone());

    let current_synthesis_topology =
        read_artifact_in_session(session, root, CURRENT_SYNTHESIS_TOPOLOGY_ARTIFACT_PATH)?;
    let hueman_aura_triad =
        build_hueman_aura_triad_from_artifacts(&hueman_fourway, &current_synthesis_topology);
    let aura_triad_path = root.join(hueman_aura_triad_artifact_path());
    stage_artifact(session, &aura_triad_path, hueman_aura_triad.clone());

    let hueman_start_choices =
        build_hueman_start_choices_from_artifacts(&hueman_fourway, &hueman_aura_triad);
    let start_choices_path = root.join(hueman_start_choices_artifact_path());
    stage_artifact(session, &start_choices_path, hueman_start_choices.clone());

    let hueman_stonebend_roles =
        build_hueman_stonebend_roles_from_artifacts(&hueman_start_choices, &hueman_fourway);
    let stonebend_roles_path = root.join(hueman_stonebend_roles_artifact_path());
    stage_artifact(
        session,
        &stonebend_roles_path,
        hueman_stonebend_roles.clone(),
    );

    let hueman_tross_helpers =
        build_hueman_tross_helpers_from_artifacts(&hueman_start_choices, &hueman_fourway);
    let tross_helpers_path = root.join(hueman_tross_helpers_artifact_path());
    stage_artifact(session, &tross_helpers_path, hueman_tross_helpers.clone());

    let hueman_glaushouse_roles =
        build_hueman_glaushouse_roles_from_artifacts(&hueman_start_choices, &hueman_fourway);
    let glaushouse_roles_path = root.join(hueman_glaushouse_roles_artifact_path());
    stage_artifact(
        session,
        &glaushouse_roles_path,
        hueman_glaushouse_roles.clone(),
    );

    let hueman_sandmanor_roles =
        build_hueman_sandmanor_roles_from_artifacts(&hueman_start_choices, &hueman_fourway);
    let sandmanor_roles_path = root.join(hueman_sandmanor_roles_artifact_path());
    stage_artifact(
        session,
        &sandmanor_roles_path,
        hueman_sandmanor_roles.clone(),
    );

    let current_synthesis_execution_spec = read_artifact_in_session(
        session,
        root,
        CURRENT_SYNTHESIS_EXECUTION_SPEC_ARTIFACT_PATH,
    )?;
    let current_synthesis_behavior_rules = read_artifact_in_session(
        session,
        root,
        CURRENT_SYNTHESIS_BEHAVIOR_RULES_ARTIFACT_PATH,
    )?;
    let current_synthesis_transition_pm_to_le = read_artifact_in_session(
        session,
        root,
        CURRENT_SYNTHESIS_TRANSITION_PM_TO_LE_ARTIFACT_PATH,
    )?;
    let current_synthesis_selection =
        read_artifact_in_session(session, root, CURRENT_SYNTHESIS_SELECTION_ARTIFACT_PATH)?;
    let current_synthesis_consequence =
        read_artifact_in_session(session, root, CURRENT_SYNTHESIS_CONSEQUENCE_ARTIFACT_PATH)?;
    let hueman_procedural_uplift = build_hueman_procedural_uplift_from_artifacts(
        &current_synthesis_execution_spec,
        &current_synthesis_behavior_rules,
        &current_synthesis_transition_pm_to_le,
        &current_synthesis_selection,
        &current_synthesis_consequence,
        &current_synthesis_activation_gate,
        &hueman_stonebend_roles,
        &hueman_tross_helpers,
        &hueman_glaushouse_roles,
        &hueman_sandmanor_roles,
    );
    let procedural_uplift_path = root.join(hueman_procedural_uplift_artifact_path());
    stage_artifact(
        session,
        &procedural_uplift_path,
        hueman_procedural_uplift.clone(),
    );

    let hueman_aura_behavior =
        build_hueman_aura_behavior_from_artifacts(&hueman_aura_triad, &hueman_start_choices);
    let aura_behavior_path = root.join(hueman_aura_behavior_artifact_path());
    stage_artifact(session, &aura_behavior_path, hueman_aura_behavior.clone());

    let hueman_archetype_lens = build_hueman_archetype_lens_from_artifacts(
        &hueman_start_choices,
        &hueman_aura_behavior,
        &hueman_stonebend_roles,
        &hueman_sandmanor_roles,
    );
    let archetype_lens_path = root.join(hueman_archetype_lens_artifact_path());
    stage_artifact(session, &archetype_lens_path, hueman_archetype_lens.clone());

    let hueman_start_paths =
        build_hueman_start_paths_from_artifacts(&hueman_start_choices, &hueman_archetype_lens);
    let start_paths_path = root.join(hueman_start_paths_artifact_path());
    stage_artifact(session, &start_paths_path, hueman_start_paths.clone());

    let hueman_path_crossovers =
        build_hueman_path_crossovers_from_artifacts(&hueman_start_paths, &hueman_aura_behavior);
    let path_crossovers_path = root.join(hueman_path_crossovers_artifact_path());
    stage_artifact(
        session,
        &path_crossovers_path,
        hueman_path_crossovers.clone(),
    );

    let current_synthesis_sequence =
        read_artifact_in_session(session, root, CURRENT_SYNTHESIS_SEQUENCE_ARTIFACT_PATH)?;
    let hueman_link_physics = build_hueman_link_physics_from_artifacts(
        &current_synthesis_sequence,
        &hueman_path_crossovers,
    );
    let link_physics_path = root.join(hueman_link_physics_artifact_path());
    stage_artifact(session, &link_physics_path, hueman_link_physics.clone());

    let hueman_inverse_circle =
        build_hueman_inverse_circle_from_artifacts(&hueman_fourway, &hueman_link_physics);
    let inverse_circle_path = root.join(hueman_inverse_circle_artifact_path());
    stage_artifact(session, &inverse_circle_path, hueman_inverse_circle.clone());

    let hueman_crossover_scenes =
        build_hueman_crossover_scenes_from_artifacts(&hueman_path_crossovers, &hueman_link_physics);
    let crossover_scenes_path = root.join(hueman_crossover_scenes_artifact_path());
    stage_artifact(
        session,
        &crossover_scenes_path,
        hueman_crossover_scenes.clone(),
    );

    let current_synthesis_contract =
        read_artifact_in_session(session, root, CURRENT_SYNTHESIS_CONTRACT_ARTIFACT_PATH)?;
    let hueman_scene_presence = build_hueman_scene_presence_from_artifacts(
        &hueman_crossover_scenes,
        &hueman_archetype_lens,
        &hueman_stonebend_roles,
        &hueman_tross_helpers,
        &hueman_glaushouse_roles,
        &hueman_sandmanor_roles,
        &hueman_inverse_circle,
    );
    let scene_presence_path = root.join(hueman_scene_presence_artifact_path());
    stage_artifact(session, &scene_presence_path, hueman_scene_presence.clone());

    let hueman_scene_intent = build_hueman_scene_intent_from_artifacts(
        &hueman_scene_presence,
        &hueman_link_physics,
        &current_synthesis_contract,
        &hueman_stonebend_roles,
        &hueman_tross_helpers,
        &hueman_glaushouse_roles,
        &hueman_sandmanor_roles,
        &hueman_inverse_circle,
    );
    let scene_intent_path = root.join(hueman_scene_intent_artifact_path());
    stage_artifact(session, &scene_intent_path, hueman_scene_intent.clone());

    let hueman_scene_drift =
        build_hueman_scene_drift_from_artifacts(&hueman_scene_intent, &hueman_link_physics);
    let scene_drift_path = root.join(hueman_scene_drift_artifact_path());
    stage_artifact(session, &scene_drift_path, hueman_scene_drift.clone());

    let vertical_integration_stack = build_vertical_integration_stack_from_artifacts(
        &current_synthesis_base,
        &hueman_boundary,
        &hueman_glaushouse_roles,
        &hueman_sandmanor_roles,
        &hueman_inverse_circle,
        &hueman_procedural_uplift,
        &hueman_scene_presence,
        &hueman_scene_intent,
        &hueman_scene_drift,
    );
    let vertical_integration_stack_path = root.join(vertical_integration_stack_artifact_path());
    stage_artifact(
        session,
        &vertical_integration_stack_path,
        vertical_integration_stack,
    );

    Ok([
        boundary_path,
        motion_map_path,
        fourway_path,
        aura_triad_path,
        start_choices_path,
        stonebend_roles_path,
        tross_helpers_path,
        glaushouse_roles_path,
        sandmanor_roles_path,
        procedural_uplift_path,
        aura_behavior_path,
        archetype_lens_path,
        start_paths_path,
        path_crossovers_path,
        link_physics_path,
        inverse_circle_path,
        crossover_scenes_path,
        scene_presence_path,
        scene_intent_path,
        scene_drift_path,
        vertical_integration_stack_path,
    ])
}

fn run_runtime_cycle_at(root: &Path, cycle_number: usize) -> io::Result<RuntimeCycleResult> {
    let cycle_started = Instant::now();
    let previous_memory = read_runtime_memory_at(root)?;
    let runtime_input = read_or_create_runtime_input_at(root)?;
    let (action_taken, should_stop, witness) = match runtime_input.mode {
        RuntimeMode::Run => {
            let kernel_pass = run_kernel_cycle(Symptom::origin());
            let mut session = ArtifactSession::new();

            let snapshot = build_snapshot_output(&kernel_pass);
            stage_artifact(&mut session, &root.join(SNAPSHOT_ARTIFACT_PATH), snapshot);

            let prompt = build_prompt_artifact_output(&kernel_pass);
            stage_artifact(&mut session, &root.join(PROMPT_ARTIFACT_PATH), prompt);

            let desktop_status = build_desktop_status_output(&kernel_pass);
            stage_artifact(
                &mut session,
                &root.join(DESKTOP_STATUS_ARTIFACT_PATH),
                desktop_status,
            );

            run_current_synthesis_at(root, &mut session)?;
            run_hueman_at(root, &mut session)?;
            session.commit()?;

            ("refreshed pipeline", false, Some(kernel_pass.to_string()))
        }
        RuntimeMode::Hold => (
            "held pipeline",
            false,
            resolve_visible_witness(root, previous_memory.as_ref())?,
        ),
        RuntimeMode::Stop => (
            "stop requested",
            true,
            resolve_visible_witness(root, previous_memory.as_ref())?,
        ),
    };
    let timestamp_unix_seconds = system_time_unix_seconds()?;
    let runtime_memory = RuntimeMemory {
        last_cycle: cycle_number,
        last_unix_time_s: timestamp_unix_seconds,
        last_runtime_mode: runtime_input.mode,
        last_action_taken: String::from(action_taken),
        last_origin: runtime_input.origin.clone(),
        last_operator_note: runtime_input.operator_note.clone(),
        last_should_stop: should_stop,
        last_witness: witness.clone(),
    };
    write_runtime_memory_at(root, &runtime_memory)?;

    let status_path = root.join(RUNTIME_LOOP_STATUS_ARTIFACT_PATH);
    let runtime_status = build_runtime_status_output(
        cycle_number,
        cycle_started.elapsed(),
        timestamp_unix_seconds,
        &runtime_input,
        action_taken,
        witness.as_deref(),
        previous_memory.as_ref(),
    );
    write_text_artifact(&status_path, &runtime_status)?;

    Ok(RuntimeCycleResult {
        cycle_number,
        elapsed: cycle_started.elapsed(),
        mode: runtime_input.mode,
        action_taken,
        should_stop,
        status_path,
    })
}

fn main() -> io::Result<()> {
    let cli = parse_runtime_cli(env::args().skip(1))
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidInput, error))?;

    let config = match cli {
        RuntimeCli::Help => {
            println!("{}", usage());
            return Ok(());
        }
        RuntimeCli::Run(config) => config,
    };

    let mut cycle_number = runtime_resume_cycle_at(Path::new("."))?;

    loop {
        cycle_number += 1;
        let cycle = run_runtime_cycle_at(Path::new("."), cycle_number)?;

        if !config.quiet {
            println!(
                "cycle {} [{}/{}] complete in {} ms -> {}",
                cycle.cycle_number,
                cycle.mode.as_str(),
                cycle.action_taken,
                cycle.elapsed.as_millis(),
                cycle.status_path.display()
            );
        }

        if cycle.should_stop {
            break;
        }

        if let Some(max_cycles) = config.cycles
            && cycle_number >= max_cycles
        {
            break;
        }

        thread::sleep(config.interval);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    use super::{
        DEFAULT_INTERVAL_MS, RUNTIME_INPUT_ARTIFACT_PATH, RUNTIME_LOOP_STATUS_ARTIFACT_PATH,
        RUNTIME_MEMORY_ARTIFACT_PATH, RuntimeCli, RuntimeConfig, RuntimeInput, RuntimeMemory,
        RuntimeMode, build_runtime_input_template, build_runtime_memory_output,
        build_runtime_status_output, parse_runtime_cli, parse_runtime_input, parse_runtime_memory,
        run_runtime_cycle_at, runtime_resume_cycle_at, usage,
    };
    use crate::current_synthesis_support::{
        ARTIFACT_INDEX_PATH, DESKTOP_STATUS_ARTIFACT_PATH, PROMPT_ARTIFACT_PATH,
        SNAPSHOT_ARTIFACT_PATH,
    };
    use hollow_grove::hueman_support::{
        hueman_boundary_artifact_path, hueman_scene_drift_artifact_path,
    };
    use hollow_grove::{CANONICAL_WITNESS, read_text_artifact, write_text_artifact};

    fn unique_artifact_root(prefix: &str) -> std::path::PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("{prefix}-{nonce}"))
    }

    fn write_fixture(root: &Path, relative_path: &str, contents: &str) {
        write_text_artifact(&root.join(relative_path), contents).expect("fixture should write");
    }

    #[test]
    fn runtime_status_output_is_readable_and_stable() {
        let runtime_input = RuntimeInput {
            mode: RuntimeMode::Run,
            origin: String::from("symptom-origin"),
            operator_note: String::from("default open loop"),
        };
        let previous_memory = RuntimeMemory {
            last_cycle: 2,
            last_unix_time_s: 1_234_567_889,
            last_runtime_mode: RuntimeMode::Hold,
            last_action_taken: String::from("held pipeline"),
            last_origin: String::from("symptom-origin"),
            last_operator_note: String::from("freeze"),
            last_should_stop: false,
            last_witness: None,
        };
        let output = build_runtime_status_output(
            3,
            Duration::from_millis(12),
            1_234_567_890,
            &runtime_input,
            "refreshed pipeline",
            Some(CANONICAL_WITNESS),
            Some(&previous_memory),
        );

        assert!(output.contains("# Hollow Grove Runtime Loop"));
        assert!(output.contains("- cycle: 3"));
        assert!(output.contains("- unix_time_s: 1234567890"));
        assert!(output.contains("- elapsed_ms: 12"));
        assert!(output.contains("- runtime_mode: run"));
        assert!(output.contains("- action_taken: refreshed pipeline"));
        assert!(output.contains("- previous_cycle: 2"));
        assert!(output.contains("- previous_action: held pipeline"));
        assert!(output.contains("default open loop"));
        assert!(output.contains(CANONICAL_WITNESS));
    }

    #[test]
    fn runtime_cli_parses_cycles_interval_and_quiet() {
        let cli = parse_runtime_cli([
            String::from("--cycles"),
            String::from("5"),
            String::from("--interval-ms"),
            String::from("250"),
            String::from("--quiet"),
        ])
        .expect("cli should parse");

        assert_eq!(
            cli,
            RuntimeCli::Run(RuntimeConfig {
                cycles: Some(5),
                interval: Duration::from_millis(250),
                quiet: true,
            })
        );
    }

    #[test]
    fn runtime_cli_defaults_to_open_loop_with_one_second_interval() {
        let cli = parse_runtime_cli(std::iter::empty::<String>()).expect("cli should parse");

        assert_eq!(
            cli,
            RuntimeCli::Run(RuntimeConfig {
                cycles: None,
                interval: Duration::from_millis(DEFAULT_INTERVAL_MS),
                quiet: false,
            })
        );
    }

    #[test]
    fn runtime_cli_reports_invalid_arguments() {
        let error = parse_runtime_cli([String::from("--cycles"), String::from("0")])
            .expect_err("zero cycles should fail");
        assert_eq!(error, "--cycles must be greater than zero");

        let error =
            parse_runtime_cli([String::from("--wat")]).expect_err("unknown flag should fail");
        assert_eq!(error, "unknown argument: --wat");
    }

    #[test]
    fn runtime_cli_supports_help() {
        let cli = parse_runtime_cli([String::from("--help")]).expect("help should parse");
        assert_eq!(cli, RuntimeCli::Help);
        assert_eq!(
            usage(),
            "Usage: hollow_grove_runtime [--cycles N] [--interval-ms N] [--quiet]"
        );
    }

    #[test]
    fn runtime_input_template_and_parser_remain_compatible() {
        let template = build_runtime_input_template();
        let runtime_input = parse_runtime_input(&template).expect("template should parse");

        assert_eq!(
            runtime_input,
            RuntimeInput {
                mode: RuntimeMode::Run,
                origin: String::from("symptom-origin"),
                operator_note: String::from("default open loop"),
            }
        );
    }

    #[test]
    fn runtime_input_parser_accepts_hold_and_stop_modes() {
        let runtime_input = parse_runtime_input(
            "runtime_mode: hold\norigin: symptom-origin\noperator_note: freeze\n",
        )
        .expect("hold input should parse");
        assert_eq!(runtime_input.mode, RuntimeMode::Hold);

        let runtime_input = parse_runtime_input(
            "runtime_mode: stop\norigin: symptom-origin\noperator_note: stop now\n",
        )
        .expect("stop input should parse");
        assert_eq!(runtime_input.mode, RuntimeMode::Stop);
    }

    #[test]
    fn runtime_input_parser_rejects_invalid_contracts() {
        let error =
            parse_runtime_input("runtime_mode: drift\n").expect_err("invalid mode should fail");
        assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);

        let error =
            parse_runtime_input("origin: wrong-place\n").expect_err("invalid origin should fail");
        assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);

        let error =
            parse_runtime_input("runtime_mode: run\nruntime_mode: hold\norigin: symptom-origin\n")
                .expect_err("duplicate mode should fail");
        assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);

        let error = parse_runtime_input(
            "runtime_mode: run\norigin: symptom-origin\noperator_note: ok\nextra: value\n",
        )
        .expect_err("unknown key should fail");
        assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
    }

    #[test]
    fn runtime_memory_output_and_parser_remain_compatible() {
        let memory = RuntimeMemory {
            last_cycle: 9,
            last_unix_time_s: 1_234_567_890,
            last_runtime_mode: RuntimeMode::Run,
            last_action_taken: String::from("refreshed pipeline"),
            last_origin: String::from("symptom-origin"),
            last_operator_note: String::from("resume from memory"),
            last_should_stop: false,
            last_witness: Some(String::from(CANONICAL_WITNESS)),
        };
        let output = build_runtime_memory_output(&memory);
        let parsed = parse_runtime_memory(&output).expect("memory should parse");

        assert_eq!(parsed, memory);
    }

    #[test]
    fn runtime_memory_parser_rejects_duplicates_and_unknown_keys() {
        let error = parse_runtime_memory(
            "last_cycle: 1\nlast_cycle: 2\nlast_unix_time_s: 1\nlast_runtime_mode: run\nlast_action_taken: ok\nlast_origin: symptom-origin\nlast_operator_note: note\nlast_should_stop: false\nlast_witness: (none)\n",
        )
        .expect_err("duplicate cycle should fail");
        assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);

        let error = parse_runtime_memory(
            "last_cycle: 1\nlast_unix_time_s: 1\nlast_runtime_mode: run\nlast_action_taken: ok\nlast_origin: symptom-origin\nlast_operator_note: note\nlast_should_stop: false\nlast_witness: (none)\nextra: value\n",
        )
        .expect_err("unknown key should fail");
        assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
    }

    #[test]
    fn runtime_resume_cycle_reads_previous_memory() {
        let artifact_root = unique_artifact_root("hollow-grove-runtime-resume");
        let memory = RuntimeMemory {
            last_cycle: 7,
            last_unix_time_s: 1_234_567_890,
            last_runtime_mode: RuntimeMode::Run,
            last_action_taken: String::from("refreshed pipeline"),
            last_origin: String::from("symptom-origin"),
            last_operator_note: String::from("resume"),
            last_should_stop: false,
            last_witness: Some(String::from(CANONICAL_WITNESS)),
        };
        write_fixture(
            &artifact_root,
            RUNTIME_MEMORY_ARTIFACT_PATH,
            &build_runtime_memory_output(&memory),
        );

        let cycle_number = runtime_resume_cycle_at(&artifact_root).expect("resume should read");
        assert_eq!(cycle_number, 7);

        fs::remove_dir_all(&artifact_root).expect("temp dir cleanup should succeed");
    }

    #[test]
    fn runtime_cycle_refreshes_kernel_current_synthesis_and_hueman_artifacts() {
        let artifact_root = unique_artifact_root("hollow-grove-runtime");
        write_fixture(
            &artifact_root,
            ARTIFACT_INDEX_PATH,
            "# Artifact Index\n\nindex",
        );

        let cycle = run_runtime_cycle_at(&artifact_root, 1).expect("runtime cycle should run");

        assert_eq!(
            cycle.status_path,
            artifact_root.join(RUNTIME_LOOP_STATUS_ARTIFACT_PATH)
        );

        let snapshot =
            read_text_artifact(&artifact_root.join(SNAPSHOT_ARTIFACT_PATH)).expect("snapshot");
        let prompt = read_text_artifact(&artifact_root.join(PROMPT_ARTIFACT_PATH)).expect("prompt");
        let desktop_status = read_text_artifact(&artifact_root.join(DESKTOP_STATUS_ARTIFACT_PATH))
            .expect("desktop status");
        let runtime_input = read_text_artifact(&artifact_root.join(RUNTIME_INPUT_ARTIFACT_PATH))
            .expect("runtime input");
        let runtime_memory = read_text_artifact(&artifact_root.join(RUNTIME_MEMORY_ARTIFACT_PATH))
            .expect("runtime memory");
        let runtime_status = read_text_artifact(&cycle.status_path).expect("runtime status");
        let hueman_boundary =
            read_text_artifact(&artifact_root.join(hueman_boundary_artifact_path()))
                .expect("hueman boundary");
        let hueman_scene_drift =
            read_text_artifact(&artifact_root.join(hueman_scene_drift_artifact_path()))
                .expect("hueman scene drift");

        assert!(snapshot.contains("\"start\": \"Symptom 1\""));
        assert!(snapshot.contains("\"canonical_witness\":"));
        assert!(prompt.contains("start Symptom 1"));
        assert!(desktop_status.contains(CANONICAL_WITNESS));
        assert!(runtime_input.contains("runtime_mode: run"));
        assert!(runtime_memory.contains("last_cycle: 1"));
        assert!(runtime_memory.contains("last_runtime_mode: run"));
        assert!(runtime_status.contains(CANONICAL_WITNESS));
        assert!(runtime_status.contains("refreshed pipeline"));
        assert!(hueman_boundary.contains("# Hueman Boundary"));
        assert!(hueman_scene_drift.contains("# Hueman Scene Drift"));

        fs::remove_dir_all(&artifact_root).expect("temp dir cleanup should succeed");
    }

    #[test]
    fn runtime_cycle_bootstraps_artifact_index_when_missing() {
        let artifact_root = unique_artifact_root("hollow-grove-runtime-bootstrap");

        let cycle = run_runtime_cycle_at(&artifact_root, 1).expect("runtime cycle should run");
        let artifact_index = read_text_artifact(&artifact_root.join(ARTIFACT_INDEX_PATH))
            .expect("artifact index should be created");
        let runtime_status = read_text_artifact(&cycle.status_path).expect("runtime status");

        assert!(artifact_index.contains("Symptom -> Triway -> HollowGrove"));
        assert!(runtime_status.contains("refreshed pipeline"));

        fs::remove_dir_all(&artifact_root).expect("temp dir cleanup should succeed");
    }

    #[test]
    fn runtime_cycle_holds_when_requested() {
        let artifact_root = unique_artifact_root("hollow-grove-runtime-hold");
        let memory = RuntimeMemory {
            last_cycle: 4,
            last_unix_time_s: 1_234_567_890,
            last_runtime_mode: RuntimeMode::Run,
            last_action_taken: String::from("refreshed pipeline"),
            last_origin: String::from("symptom-origin"),
            last_operator_note: String::from("prior run"),
            last_should_stop: false,
            last_witness: Some(String::from(CANONICAL_WITNESS)),
        };
        write_fixture(
            &artifact_root,
            RUNTIME_MEMORY_ARTIFACT_PATH,
            &build_runtime_memory_output(&memory),
        );
        write_fixture(
            &artifact_root,
            RUNTIME_INPUT_ARTIFACT_PATH,
            "runtime_mode: hold\norigin: symptom-origin\noperator_note: hold still\n",
        );

        let cycle = run_runtime_cycle_at(&artifact_root, 5).expect("hold cycle should run");
        let runtime_status = read_text_artifact(&cycle.status_path).expect("runtime status");
        let runtime_memory = read_text_artifact(&artifact_root.join(RUNTIME_MEMORY_ARTIFACT_PATH))
            .expect("runtime memory");

        assert_eq!(cycle.mode, RuntimeMode::Hold);
        assert!(!cycle.should_stop);
        assert!(runtime_status.contains("- runtime_mode: hold"));
        assert!(runtime_status.contains("- action_taken: held pipeline"));
        assert!(runtime_status.contains(CANONICAL_WITNESS));
        assert!(runtime_status.contains("- kernel artifacts unchanged"));
        assert!(runtime_status.contains("- previous_cycle: 4"));
        assert!(runtime_memory.contains("last_cycle: 5"));
        assert!(runtime_memory.contains("last_runtime_mode: hold"));

        fs::remove_dir_all(&artifact_root).expect("temp dir cleanup should succeed");
    }

    #[test]
    fn runtime_cycle_stops_when_requested() {
        let artifact_root = unique_artifact_root("hollow-grove-runtime-stop");
        write_fixture(
            &artifact_root,
            RUNTIME_INPUT_ARTIFACT_PATH,
            "runtime_mode: stop\norigin: symptom-origin\noperator_note: shut down\n",
        );

        let cycle = run_runtime_cycle_at(&artifact_root, 1).expect("stop cycle should run");
        let runtime_status = read_text_artifact(&cycle.status_path).expect("runtime status");
        let runtime_memory = read_text_artifact(&artifact_root.join(RUNTIME_MEMORY_ARTIFACT_PATH))
            .expect("runtime memory");

        assert_eq!(cycle.mode, RuntimeMode::Stop);
        assert!(cycle.should_stop);
        assert!(runtime_status.contains("- runtime_mode: stop"));
        assert!(runtime_status.contains("- action_taken: stop requested"));
        assert!(runtime_status.contains("- kernel artifacts unchanged"));
        assert!(runtime_memory.contains("last_should_stop: true"));

        fs::remove_dir_all(&artifact_root).expect("temp dir cleanup should succeed");
    }
}
