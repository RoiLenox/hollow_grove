use std::env;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use std::time::Duration;

use hollow_grove::{read_text_artifact, write_text_artifact};

const RUNTIME_MEMORY_ARTIFACT_PATH: &str = "artifacts/runtime_memory.txt";
const NIRI_BRIDGE_MEMORY_ARTIFACT_PATH: &str = "artifacts/niri_bridge_memory.txt";
const NIRI_BRIDGE_STATUS_ARTIFACT_PATH: &str = "artifacts/niri_bridge_status.md";
const HOLLOW_GROVE_WORKSPACE_NAME: &str = "HollowGrove";
const DEFAULT_INTERVAL_MS: u64 = 1_000;

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
struct RuntimeMemory {
    last_cycle: usize,
    last_runtime_mode: RuntimeMode,
    last_action_taken: String,
    last_origin: String,
    last_operator_note: String,
    last_should_stop: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct NiriWorkspace {
    id: usize,
    idx: usize,
    name: Option<String>,
    output: String,
    is_focused: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OverviewState {
    Open,
    Closed,
}

impl OverviewState {
    fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Closed => "closed",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum NiriCommand {
    OpenOverview,
    CloseOverview,
    SetWorkspaceName { name: String },
    FocusWorkspace { reference: String },
}

impl NiriCommand {
    fn signature(&self) -> String {
        match self {
            Self::OpenOverview => String::from("open-overview"),
            Self::CloseOverview => String::from("close-overview"),
            Self::SetWorkspaceName { name } => format!("set-workspace-name:{name}"),
            Self::FocusWorkspace { reference } => format!("focus-workspace:{reference}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BridgePlan {
    commands: Vec<NiriCommand>,
    desired_workspace: Option<String>,
    result_message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BridgeMemory {
    last_runtime_cycle: usize,
    last_runtime_mode: RuntimeMode,
    last_command_signature: String,
    last_bridge_result: String,
    last_workspace_target: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BridgeResultKind {
    Applied,
    DryRun,
    Noop,
    Waiting,
}

impl BridgeResultKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Applied => "applied",
            Self::DryRun => "dry-run",
            Self::Noop => "no-op",
            Self::Waiting => "waiting",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BridgeConfig {
    apply: bool,
    cycles: Option<usize>,
    interval: Duration,
    quiet: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum BridgeCli {
    Help,
    Run(BridgeConfig),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BridgeTickResult {
    runtime_cycle: Option<usize>,
    runtime_mode: Option<RuntimeMode>,
    command_count: usize,
    result_kind: BridgeResultKind,
    result_message: String,
    status_path: PathBuf,
}

fn parse_runtime_mode(value: &str) -> io::Result<RuntimeMode> {
    match value {
        "run" => Ok(RuntimeMode::Run),
        "hold" => Ok(RuntimeMode::Hold),
        "stop" => Ok(RuntimeMode::Stop),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("invalid runtime mode: {value}"),
        )),
    }
}

fn parse_bridge_cli<I>(args: I) -> Result<BridgeCli, String>
where
    I: IntoIterator<Item = String>,
{
    let mut apply = false;
    let mut cycles = Some(1usize);
    let mut interval = Duration::from_millis(DEFAULT_INTERVAL_MS);
    let mut quiet = false;
    let mut args = args.into_iter();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--help" | "-h" => return Ok(BridgeCli::Help),
            "--apply" => apply = true,
            "--quiet" => quiet = true,
            "--watch" => cycles = None,
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

    Ok(BridgeCli::Run(BridgeConfig {
        apply,
        cycles,
        interval,
        quiet,
    }))
}

fn usage() -> &'static str {
    "Usage: hollow_grove_niri_bridge [--apply] [--watch | --cycles N] [--interval-ms N] [--quiet]"
}

fn parse_kv(contents: &str) -> io::Result<Vec<(String, String)>> {
    let mut pairs = Vec::new();

    for raw_line in contents.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let (key, value) = line.split_once(':').ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("line is missing ':' separator: {line}"),
            )
        })?;
        pairs.push((key.trim().to_owned(), value.trim().to_owned()));
    }

    Ok(pairs)
}

fn parse_runtime_memory(contents: &str) -> io::Result<RuntimeMemory> {
    let mut last_cycle = None;
    let mut last_runtime_mode = None;
    let mut last_action_taken = None;
    let mut last_origin = None;
    let mut last_operator_note = None;
    let mut last_should_stop = None;

    for (key, value) in parse_kv(contents)? {
        match key.as_str() {
            "last_cycle" => {
                last_cycle = Some(value.parse::<usize>().map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("invalid usize for last_cycle: {value}"),
                    )
                })?)
            }
            "last_runtime_mode" => last_runtime_mode = Some(parse_runtime_mode(&value)?),
            "last_action_taken" => last_action_taken = Some(value),
            "last_origin" => last_origin = Some(value),
            "last_operator_note" => last_operator_note = Some(value),
            "last_should_stop" => {
                last_should_stop = Some(match value.as_str() {
                    "true" => true,
                    "false" => false,
                    _ => {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("invalid bool for last_should_stop: {value}"),
                        ));
                    }
                })
            }
            _ => {}
        }
    }

    Ok(RuntimeMemory {
        last_cycle: last_cycle.ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "runtime memory missing last_cycle",
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
    })
}

fn parse_bridge_memory(contents: &str) -> io::Result<BridgeMemory> {
    let mut last_runtime_cycle = None;
    let mut last_runtime_mode = None;
    let mut last_command_signature = None;
    let mut last_bridge_result = None;
    let mut last_workspace_target = None;

    for (key, value) in parse_kv(contents)? {
        match key.as_str() {
            "last_runtime_cycle" => {
                last_runtime_cycle = Some(value.parse::<usize>().map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("invalid usize for last_runtime_cycle: {value}"),
                    )
                })?)
            }
            "last_runtime_mode" => last_runtime_mode = Some(parse_runtime_mode(&value)?),
            "last_command_signature" => last_command_signature = Some(value),
            "last_niri_action" => last_command_signature = Some(value),
            "last_bridge_result" => last_bridge_result = Some(value),
            "last_workspace_target" => last_workspace_target = Some(value),
            _ => {}
        }
    }

    Ok(BridgeMemory {
        last_runtime_cycle: last_runtime_cycle.ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "bridge memory missing last_runtime_cycle",
            )
        })?,
        last_runtime_mode: last_runtime_mode.ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "bridge memory missing last_runtime_mode",
            )
        })?,
        last_command_signature: last_command_signature.ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "bridge memory missing last_command_signature",
            )
        })?,
        last_bridge_result: last_bridge_result.ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "bridge memory missing last_bridge_result",
            )
        })?,
        last_workspace_target: last_workspace_target.unwrap_or_else(|| String::from("(none)")),
    })
}

fn read_runtime_memory_at(root: &Path) -> io::Result<Option<RuntimeMemory>> {
    match read_text_artifact(&root.join(RUNTIME_MEMORY_ARTIFACT_PATH)) {
        Ok(contents) => parse_runtime_memory(&contents).map(Some),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error),
    }
}

fn read_bridge_memory_at(root: &Path) -> io::Result<Option<BridgeMemory>> {
    match read_text_artifact(&root.join(NIRI_BRIDGE_MEMORY_ARTIFACT_PATH)) {
        Ok(contents) => parse_bridge_memory(&contents).map(Some),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error),
    }
}

fn write_bridge_memory_at(root: &Path, bridge_memory: &BridgeMemory) -> io::Result<PathBuf> {
    let output = format!(
        "# Hollow Grove Niri Bridge Memory\n\
         last_runtime_cycle: {}\n\
         last_runtime_mode: {}\n\
         last_command_signature: {}\n\
         last_bridge_result: {}\n\
         last_workspace_target: {}\n",
        bridge_memory.last_runtime_cycle,
        bridge_memory.last_runtime_mode.as_str(),
        bridge_memory.last_command_signature,
        bridge_memory.last_bridge_result,
        bridge_memory.last_workspace_target
    );
    let path = root.join(NIRI_BRIDGE_MEMORY_ARTIFACT_PATH);
    write_text_artifact(&path, &output)?;
    Ok(path)
}

fn split_top_level_objects(json: &str) -> io::Result<Vec<&str>> {
    let trimmed = json.trim();
    if !trimmed.starts_with('[') || !trimmed.ends_with(']') {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "workspace json is not an array",
        ));
    }

    let mut objects = Vec::new();
    let mut depth = 0usize;
    let mut start = None;
    let mut in_string = false;
    let mut escaped = false;

    for (index, ch) in trimmed.char_indices() {
        if in_string {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }

        match ch {
            '"' => in_string = true,
            '{' => {
                if depth == 0 {
                    start = Some(index);
                }
                depth += 1;
            }
            '}' => {
                if depth == 0 {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "workspace json has unmatched closing brace",
                    ));
                }
                depth -= 1;
                if depth == 0 {
                    let object_start = start.ok_or_else(|| {
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            "workspace json object start missing",
                        )
                    })?;
                    objects.push(&trimmed[object_start..=index]);
                }
            }
            _ => {}
        }
    }

    if depth != 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "workspace json has unmatched opening brace",
        ));
    }

    Ok(objects)
}

fn find_json_field<'a>(object: &'a str, key: &str) -> io::Result<&'a str> {
    let pattern = format!("\"{key}\":");
    let start = object.find(&pattern).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("workspace object missing field {key}"),
        )
    })? + pattern.len();

    let mut in_string = false;
    let mut escaped = false;
    let mut depth = 0usize;
    let rest = &object[start..];

    for (index, ch) in rest.char_indices() {
        if in_string {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }

        match ch {
            '"' => in_string = true,
            '[' | '{' => depth += 1,
            ']' | '}' => {
                if depth == 0 {
                    return Ok(rest[..index].trim());
                }
                depth -= 1;
            }
            ',' if depth == 0 => return Ok(rest[..index].trim()),
            _ => {}
        }
    }

    Ok(rest.trim())
}

fn parse_json_string(value: &str) -> io::Result<Option<String>> {
    if value == "null" {
        return Ok(None);
    }
    if !(value.starts_with('"') && value.ends_with('"')) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("expected json string or null, got {value}"),
        ));
    }

    Ok(Some(value[1..value.len() - 1].replace("\\\"", "\"")))
}

fn parse_json_bool(value: &str) -> io::Result<bool> {
    match value {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("expected json bool, got {value}"),
        )),
    }
}

fn parse_json_usize(value: &str, field: &str) -> io::Result<usize> {
    value.parse::<usize>().map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("expected usize for {field}, got {value}"),
        )
    })
}

fn parse_workspaces_json(contents: &str) -> io::Result<Vec<NiriWorkspace>> {
    let mut workspaces = Vec::new();

    for object in split_top_level_objects(contents)? {
        workspaces.push(NiriWorkspace {
            id: parse_json_usize(find_json_field(object, "id")?, "id")?,
            idx: parse_json_usize(find_json_field(object, "idx")?, "idx")?,
            name: parse_json_string(find_json_field(object, "name")?)?,
            output: parse_json_string(find_json_field(object, "output")?)?.ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "workspace output cannot be null",
                )
            })?,
            is_focused: parse_json_bool(find_json_field(object, "is_focused")?)?,
        });
    }

    Ok(workspaces)
}

fn parse_overview_state(contents: &str) -> io::Result<OverviewState> {
    match contents.trim() {
        "Overview is open." => Ok(OverviewState::Open),
        "Overview is closed." => Ok(OverviewState::Closed),
        other => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("unexpected overview state output: {other}"),
        )),
    }
}

fn read_live_workspaces() -> io::Result<Vec<NiriWorkspace>> {
    let output = Command::new("niri")
        .args(["msg", "-j", "workspaces"])
        .output()?;
    if !output.status.success() {
        return Err(io::Error::other(format!(
            "niri msg -j workspaces exited with status {}",
            output.status
        )));
    }
    parse_workspaces_json(&String::from_utf8_lossy(&output.stdout))
}

fn read_live_overview_state() -> io::Result<OverviewState> {
    let output = Command::new("niri")
        .args(["msg", "overview-state"])
        .output()?;
    if !output.status.success() {
        return Err(io::Error::other(format!(
            "niri msg overview-state exited with status {}",
            output.status
        )));
    }
    parse_overview_state(&String::from_utf8_lossy(&output.stdout))
}

fn build_plan(
    runtime_mode: RuntimeMode,
    workspaces: &[NiriWorkspace],
    overview: OverviewState,
) -> BridgePlan {
    let mut commands = Vec::new();
    let focused_workspace = workspaces.iter().find(|workspace| workspace.is_focused);
    let hollow_grove_workspace = workspaces
        .iter()
        .find(|workspace| workspace.name.as_deref() == Some(HOLLOW_GROVE_WORKSPACE_NAME));

    match runtime_mode {
        RuntimeMode::Run => {
            if let Some(workspace) = hollow_grove_workspace {
                if !workspace.is_focused {
                    commands.push(NiriCommand::FocusWorkspace {
                        reference: String::from(HOLLOW_GROVE_WORKSPACE_NAME),
                    });
                }
            } else if let Some(workspace) = focused_workspace {
                if workspace.name.as_deref() != Some(HOLLOW_GROVE_WORKSPACE_NAME) {
                    commands.push(NiriCommand::SetWorkspaceName {
                        name: String::from(HOLLOW_GROVE_WORKSPACE_NAME),
                    });
                }
            } else {
                return BridgePlan {
                    commands: Vec::new(),
                    desired_workspace: Some(String::from(HOLLOW_GROVE_WORKSPACE_NAME)),
                    result_message: String::from("no focused workspace available"),
                };
            }

            if overview == OverviewState::Open {
                commands.push(NiriCommand::CloseOverview);
            }

            let message = if commands.is_empty() {
                String::from("HollowGrove workspace is already focused and overview is closed")
            } else {
                String::from("taking over the focused workspace for HollowGrove run mode")
            };

            BridgePlan {
                commands,
                desired_workspace: Some(String::from(HOLLOW_GROVE_WORKSPACE_NAME)),
                result_message: message,
            }
        }
        RuntimeMode::Hold => {
            if overview == OverviewState::Closed {
                commands.push(NiriCommand::OpenOverview);
            }

            BridgePlan {
                commands,
                desired_workspace: hollow_grove_workspace
                    .map(|_| String::from(HOLLOW_GROVE_WORKSPACE_NAME)),
                result_message: if overview == OverviewState::Open {
                    String::from("overview is already open for hold mode")
                } else {
                    String::from("opening overview for hold mode")
                },
            }
        }
        RuntimeMode::Stop => {
            if overview == OverviewState::Open {
                commands.push(NiriCommand::CloseOverview);
            }

            BridgePlan {
                commands,
                desired_workspace: hollow_grove_workspace
                    .map(|_| String::from(HOLLOW_GROVE_WORKSPACE_NAME)),
                result_message: if overview == OverviewState::Closed {
                    String::from("overview is already closed for stop mode")
                } else {
                    String::from("closing overview for stop mode")
                },
            }
        }
    }
}

fn apply_niri_command(command: &NiriCommand) -> io::Result<()> {
    let status = match command {
        NiriCommand::OpenOverview => Command::new("niri")
            .args(["msg", "action", "open-overview"])
            .status()?,
        NiriCommand::CloseOverview => Command::new("niri")
            .args(["msg", "action", "close-overview"])
            .status()?,
        NiriCommand::FocusWorkspace { reference } => Command::new("niri")
            .args(["msg", "action", "focus-workspace", reference])
            .status()?,
        NiriCommand::SetWorkspaceName { name } => Command::new("niri")
            .args(["msg", "action", "set-workspace-name", name])
            .status()?,
    };

    if status.success() {
        Ok(())
    } else {
        Err(io::Error::other(format!(
            "niri command {} exited with status {status}",
            command.signature()
        )))
    }
}

fn command_signature(commands: &[NiriCommand]) -> String {
    if commands.is_empty() {
        String::from("none")
    } else {
        commands
            .iter()
            .map(NiriCommand::signature)
            .collect::<Vec<_>>()
            .join(",")
    }
}

fn workspace_summary(workspaces: &[NiriWorkspace]) -> String {
    if workspaces.is_empty() {
        return String::from("- none");
    }

    workspaces
        .iter()
        .map(|workspace| {
            format!(
                "- idx {} on {} name={} focused={}",
                workspace.idx,
                workspace.output,
                workspace.name.as_deref().unwrap_or("(none)"),
                workspace.is_focused
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn build_bridge_status_output(
    apply_enabled: bool,
    runtime_memory: Option<&RuntimeMemory>,
    previous_bridge_memory: Option<&BridgeMemory>,
    workspaces: &[NiriWorkspace],
    overview: OverviewState,
    plan: &BridgePlan,
    result_kind: BridgeResultKind,
) -> String {
    let runtime_section = match runtime_memory {
        Some(memory) => format!(
            "- last_cycle: {}\n\
             - last_runtime_mode: {}\n\
             - last_action_taken: {}\n\
             - last_origin: {}\n\
             - last_should_stop: {}",
            memory.last_cycle,
            memory.last_runtime_mode.as_str(),
            memory.last_action_taken,
            memory.last_origin,
            memory.last_should_stop
        ),
        None => String::from(
            "- last_cycle: none\n\
             - last_runtime_mode: none\n\
             - last_action_taken: none\n\
             - last_origin: none\n\
             - last_should_stop: none",
        ),
    };

    let previous_section = match previous_bridge_memory {
        Some(memory) => format!(
            "- last_runtime_cycle: {}\n\
             - last_runtime_mode: {}\n\
             - last_command_signature: {}\n\
             - last_bridge_result: {}\n\
             - last_workspace_target: {}",
            memory.last_runtime_cycle,
            memory.last_runtime_mode.as_str(),
            memory.last_command_signature,
            memory.last_bridge_result,
            memory.last_workspace_target
        ),
        None => String::from(
            "- last_runtime_cycle: none\n\
             - last_runtime_mode: none\n\
             - last_command_signature: none\n\
             - last_bridge_result: none\n\
             - last_workspace_target: none",
        ),
    };

    let planned_commands = if plan.commands.is_empty() {
        String::from("- none")
    } else {
        plan.commands
            .iter()
            .map(|command| format!("- {}", command.signature()))
            .collect::<Vec<_>>()
            .join("\n")
    };

    let desired_workspace = plan.desired_workspace.as_deref().unwrap_or("(none)");

    format!(
        "# Hollow Grove Niri Bridge\n\n\
         ## Bridge Status\n\n\
         - apply_enabled: {apply_enabled}\n\
         - result_kind: {}\n\
         - overview_state: {}\n\
         - desired_workspace: {desired_workspace}\n\
         - result_message: {}\n\
         - runtime_memory_contract: `{RUNTIME_MEMORY_ARTIFACT_PATH}`\n\
         - bridge_memory_contract: `{NIRI_BRIDGE_MEMORY_ARTIFACT_PATH}`\n\n\
         ## Planned Commands\n\n\
         {planned_commands}\n\n\
         ## Runtime Memory\n\n\
         {runtime_section}\n\n\
         ## Previous Bridge Memory\n\n\
         {previous_section}\n\n\
         ## Live Workspaces\n\n\
         {}\n",
        result_kind.as_str(),
        overview.as_str(),
        plan.result_message,
        workspace_summary(workspaces)
    )
}

fn run_bridge_tick(root: &Path, config: BridgeConfig) -> io::Result<BridgeTickResult> {
    let runtime_memory = read_runtime_memory_at(root)?;
    let previous_bridge_memory = read_bridge_memory_at(root)?;

    let runtime_memory = match runtime_memory {
        Some(memory) => memory,
        None => {
            let status = "# Hollow Grove Niri Bridge\n\n## Bridge Status\n\n- apply_enabled: false\n- result_kind: waiting\n- result_message: waiting for runtime memory\n";
            let status_path = root.join(NIRI_BRIDGE_STATUS_ARTIFACT_PATH);
            write_text_artifact(&status_path, status)?;
            return Ok(BridgeTickResult {
                runtime_cycle: None,
                runtime_mode: None,
                command_count: 0,
                result_kind: BridgeResultKind::Waiting,
                result_message: String::from("waiting for runtime memory"),
                status_path,
            });
        }
    };

    let workspaces = read_live_workspaces()?;
    let overview = read_live_overview_state()?;
    let plan = build_plan(runtime_memory.last_runtime_mode, &workspaces, overview);
    let signature = command_signature(&plan.commands);

    let result_kind = if plan.commands.is_empty() {
        BridgeResultKind::Noop
    } else if config.apply {
        for command in &plan.commands {
            apply_niri_command(command)?;
        }
        BridgeResultKind::Applied
    } else {
        BridgeResultKind::DryRun
    };

    let status_workspaces = if config.apply && !plan.commands.is_empty() {
        read_live_workspaces()?
    } else {
        workspaces
    };
    let status_overview = if config.apply && !plan.commands.is_empty() {
        read_live_overview_state()?
    } else {
        overview
    };

    let bridge_memory = BridgeMemory {
        last_runtime_cycle: runtime_memory.last_cycle,
        last_runtime_mode: runtime_memory.last_runtime_mode,
        last_command_signature: signature,
        last_bridge_result: String::from(result_kind.as_str()),
        last_workspace_target: plan
            .desired_workspace
            .clone()
            .unwrap_or_else(|| String::from("(none)")),
    };
    write_bridge_memory_at(root, &bridge_memory)?;

    let status_output = build_bridge_status_output(
        config.apply,
        Some(&runtime_memory),
        previous_bridge_memory.as_ref(),
        &status_workspaces,
        status_overview,
        &plan,
        result_kind,
    );
    let status_path = root.join(NIRI_BRIDGE_STATUS_ARTIFACT_PATH);
    write_text_artifact(&status_path, &status_output)?;

    Ok(BridgeTickResult {
        runtime_cycle: Some(runtime_memory.last_cycle),
        runtime_mode: Some(runtime_memory.last_runtime_mode),
        command_count: plan.commands.len(),
        result_kind,
        result_message: plan.result_message,
        status_path,
    })
}

fn main() -> io::Result<()> {
    let cli = parse_bridge_cli(env::args().skip(1))
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidInput, error))?;

    let config = match cli {
        BridgeCli::Help => {
            println!("{}", usage());
            return Ok(());
        }
        BridgeCli::Run(config) => config,
    };

    let mut tick_count = 0usize;

    loop {
        tick_count += 1;
        let result = run_bridge_tick(Path::new("."), config)?;

        if !config.quiet {
            println!(
                "bridge tick {} [{}] commands={} -> {}",
                tick_count,
                result.result_kind.as_str(),
                result.command_count,
                result.status_path.display()
            );
        }

        if let Some(max_cycles) = config.cycles
            && tick_count >= max_cycles
        {
            break;
        }

        thread::sleep(config.interval);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        BridgeCli, BridgeConfig, BridgeResultKind, DEFAULT_INTERVAL_MS,
        HOLLOW_GROVE_WORKSPACE_NAME, NiriCommand, NiriWorkspace, OverviewState, RuntimeMode,
        build_plan, parse_bridge_cli, parse_bridge_memory, parse_overview_state,
        parse_runtime_memory, parse_workspaces_json, usage, workspace_summary,
    };

    fn sample_workspaces() -> &'static str {
        r#"[{"id":4,"idx":2,"name":null,"output":"DP-2","is_urgent":false,"is_active":false,"is_focused":false,"active_window_id":null},{"id":1,"idx":1,"name":null,"output":"DP-2","is_urgent":false,"is_active":true,"is_focused":true,"active_window_id":4}]"#
    }

    #[test]
    fn bridge_cli_defaults_to_single_dry_run_tick() {
        let cli = parse_bridge_cli(std::iter::empty::<String>()).expect("cli should parse");
        assert_eq!(
            cli,
            BridgeCli::Run(BridgeConfig {
                apply: false,
                cycles: Some(1),
                interval: std::time::Duration::from_millis(DEFAULT_INTERVAL_MS),
                quiet: false,
            })
        );
    }

    #[test]
    fn bridge_cli_parses_apply_watch_and_quiet() {
        let cli = parse_bridge_cli([
            String::from("--apply"),
            String::from("--watch"),
            String::from("--interval-ms"),
            String::from("250"),
            String::from("--quiet"),
        ])
        .expect("cli should parse");
        assert_eq!(
            cli,
            BridgeCli::Run(BridgeConfig {
                apply: true,
                cycles: None,
                interval: std::time::Duration::from_millis(250),
                quiet: true,
            })
        );
    }

    #[test]
    fn bridge_cli_supports_help() {
        let cli = parse_bridge_cli([String::from("--help")]).expect("help should parse");
        assert_eq!(cli, BridgeCli::Help);
        assert_eq!(
            usage(),
            "Usage: hollow_grove_niri_bridge [--apply] [--watch | --cycles N] [--interval-ms N] [--quiet]"
        );
    }

    #[test]
    fn runtime_memory_parser_reads_runtime_contract() {
        let memory = parse_runtime_memory(
            "# Hollow Grove Runtime Memory\n\
             last_cycle: 4\n\
             last_unix_time_s: 1234567890\n\
             last_runtime_mode: hold\n\
             last_action_taken: refreshed pipeline\n\
             last_origin: symptom-origin\n\
             last_operator_note: fixture\n\
             last_should_stop: false\n\
             last_witness: start Symptom 1\\n↓\\nTriway\n",
        )
        .expect("runtime memory parse");
        assert_eq!(memory.last_cycle, 4);
        assert_eq!(memory.last_runtime_mode, RuntimeMode::Hold);
        assert_eq!(memory.last_origin, "symptom-origin");
    }

    #[test]
    fn bridge_memory_parser_reads_bridge_contract() {
        let bridge_memory = parse_bridge_memory(
            "# Hollow Grove Niri Bridge Memory\n\
             last_runtime_cycle: 3\n\
             last_runtime_mode: hold\n\
             last_command_signature: open-overview\n\
             last_bridge_result: applied\n\
             last_workspace_target: HollowGrove\n",
        )
        .expect("bridge memory parse");
        assert_eq!(bridge_memory.last_runtime_cycle, 3);
        assert_eq!(bridge_memory.last_runtime_mode, RuntimeMode::Hold);
        assert_eq!(bridge_memory.last_command_signature, "open-overview");
    }

    #[test]
    fn workspaces_json_parser_reads_live_shape() {
        let workspaces = parse_workspaces_json(sample_workspaces()).expect("workspace parse");
        assert_eq!(workspaces.len(), 2);
        assert_eq!(workspaces[0].idx, 2);
        assert_eq!(workspaces[1].idx, 1);
        assert!(workspaces[1].is_focused);
        assert_eq!(workspaces[1].name, None);
    }

    #[test]
    fn overview_state_parser_reads_cli_output() {
        assert_eq!(
            parse_overview_state("Overview is open.\n").expect("open parse"),
            OverviewState::Open
        );
        assert_eq!(
            parse_overview_state("Overview is closed.\n").expect("closed parse"),
            OverviewState::Closed
        );
    }

    #[test]
    fn run_mode_claims_unnamed_focused_workspace() {
        let workspaces = parse_workspaces_json(sample_workspaces()).expect("workspace parse");
        let plan = build_plan(RuntimeMode::Run, &workspaces, OverviewState::Open);

        assert_eq!(
            plan.commands,
            vec![
                NiriCommand::SetWorkspaceName {
                    name: String::from(HOLLOW_GROVE_WORKSPACE_NAME),
                },
                NiriCommand::CloseOverview,
            ]
        );
        assert_eq!(
            plan.desired_workspace,
            Some(String::from(HOLLOW_GROVE_WORKSPACE_NAME))
        );
        assert!(
            plan.result_message
                .contains("taking over the focused workspace for HollowGrove run mode")
        );
    }

    #[test]
    fn run_mode_focuses_existing_hollow_grove_workspace() {
        let workspaces = vec![
            NiriWorkspace {
                id: 1,
                idx: 1,
                name: Some(String::from(HOLLOW_GROVE_WORKSPACE_NAME)),
                output: String::from("DP-2"),
                is_focused: false,
            },
            NiriWorkspace {
                id: 2,
                idx: 2,
                name: None,
                output: String::from("DP-2"),
                is_focused: true,
            },
        ];
        let plan = build_plan(RuntimeMode::Run, &workspaces, OverviewState::Open);

        assert_eq!(
            plan.commands,
            vec![
                NiriCommand::FocusWorkspace {
                    reference: String::from(HOLLOW_GROVE_WORKSPACE_NAME),
                },
                NiriCommand::CloseOverview,
            ]
        );
    }

    #[test]
    fn run_mode_renames_named_focused_workspace_to_hollow_grove() {
        let workspaces = vec![NiriWorkspace {
            id: 9,
            idx: 1,
            name: Some(String::from("Work")),
            output: String::from("DP-2"),
            is_focused: true,
        }];
        let plan = build_plan(RuntimeMode::Run, &workspaces, OverviewState::Closed);

        assert_eq!(
            plan.commands,
            vec![NiriCommand::SetWorkspaceName {
                name: String::from(HOLLOW_GROVE_WORKSPACE_NAME),
            }]
        );
        assert!(
            plan.result_message
                .contains("taking over the focused workspace for HollowGrove run mode")
        );
    }

    #[test]
    fn hold_mode_opens_overview() {
        let workspaces = parse_workspaces_json(sample_workspaces()).expect("workspace parse");
        let plan = build_plan(RuntimeMode::Hold, &workspaces, OverviewState::Closed);
        assert_eq!(plan.commands, vec![NiriCommand::OpenOverview]);
    }

    #[test]
    fn stop_mode_closes_overview() {
        let workspaces = parse_workspaces_json(sample_workspaces()).expect("workspace parse");
        let plan = build_plan(RuntimeMode::Stop, &workspaces, OverviewState::Open);
        assert_eq!(plan.commands, vec![NiriCommand::CloseOverview]);
    }

    #[test]
    fn workspace_summary_is_readable() {
        let workspaces = parse_workspaces_json(sample_workspaces()).expect("workspace parse");
        let summary = workspace_summary(&workspaces);
        assert!(summary.contains("idx 1"));
        assert!(summary.contains("focused=true"));
    }

    #[test]
    fn status_kinds_remain_stable() {
        assert_eq!(BridgeResultKind::Applied.as_str(), "applied");
        assert_eq!(BridgeResultKind::DryRun.as_str(), "dry-run");
        assert_eq!(BridgeResultKind::Noop.as_str(), "no-op");
        assert_eq!(BridgeResultKind::Waiting.as_str(), "waiting");
    }
}
