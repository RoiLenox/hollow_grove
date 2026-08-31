use std::env;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;
use std::thread;
use std::time::Duration;

use hollow_grove::composition::{NORMALIZED_UNIT, NormalizedCoordinate, NormalizedPoint};
use hollow_grove::world::spatial_inheritance::{
    HollowGroveSpatialMap, canonical_hollow_grove_spatial_map, hollow_grove_spatial_replay_digest,
};
use hollow_grove::{read_text_artifact, write_text_artifact};
use hollow_niri::NiriConnection;
use serde_json::{Value, json};

const RUNTIME_MEMORY_ARTIFACT_PATH: &str = "artifacts/runtime_memory.txt";
const NIRI_BRIDGE_MEMORY_ARTIFACT_PATH: &str = "artifacts/niri_bridge_memory.txt";
const NIRI_BRIDGE_STATUS_ARTIFACT_PATH: &str = "artifacts/niri_bridge_status.md";
const SCREEN_MAP_STATE_ARTIFACT_PATH: &str = "artifacts/screen_map_state.json";
const HOLLOW_GROVE_WORKSPACE_NAME: &str = "HollowGrove";
const DEFAULT_INTERVAL_MS: u64 = 1_000;

struct LiveSpatialContract {
    map: HollowGroveSpatialMap,
    catalog_digest: String,
    template_digest: String,
    replay_digest: String,
}

static CANONICAL_SPATIAL_MAP: OnceLock<Result<LiveSpatialContract, String>> = OnceLock::new();

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

#[derive(Debug, Clone, Copy, PartialEq)]
struct Rect {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

#[derive(Debug, Clone, PartialEq)]
struct FocusedWindow {
    id: Option<usize>,
    title: Option<String>,
    app_id: Option<String>,
    output: Option<String>,
    rect: Rect,
}

#[derive(Debug, Clone, PartialEq)]
struct NiriOutput {
    name: String,
    is_focused: bool,
    rect: Rect,
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

fn parse_json_f64(value: &str, field: &str) -> io::Result<f64> {
    value.parse::<f64>().map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("expected f64 for {field}, got {value}"),
        )
    })
}

fn optional_json_field<'a>(object: &'a str, key: &str) -> Option<&'a str> {
    find_json_field(object, key).ok()
}

fn first_json_string_field(object: &str, keys: &[&str]) -> io::Result<Option<String>> {
    for key in keys {
        if let Some(value) = optional_json_field(object, key) {
            return parse_json_string(value);
        }
    }
    Ok(None)
}

fn first_json_bool_field(object: &str, keys: &[&str]) -> io::Result<Option<bool>> {
    for key in keys {
        if let Some(value) = optional_json_field(object, key) {
            return parse_json_bool(value).map(Some);
        }
    }
    Ok(None)
}

fn first_json_usize_field(object: &str, keys: &[&str]) -> io::Result<Option<usize>> {
    for key in keys {
        if let Some(value) = optional_json_field(object, key) {
            return parse_json_usize(value, key).map(Some);
        }
    }
    Ok(None)
}

fn first_json_f64_field(object: &str, keys: &[&str]) -> io::Result<Option<f64>> {
    for key in keys {
        if let Some(value) = optional_json_field(object, key) {
            return parse_json_f64(value, key).map(Some);
        }
    }
    Ok(None)
}

fn rect_from_field_sets(
    object: &str,
    x_keys: &[&str],
    y_keys: &[&str],
    width_keys: &[&str],
    height_keys: &[&str],
) -> io::Result<Option<Rect>> {
    let x = first_json_f64_field(object, x_keys)?;
    let y = first_json_f64_field(object, y_keys)?;
    let width = first_json_f64_field(object, width_keys)?;
    let height = first_json_f64_field(object, height_keys)?;

    match (x, y, width, height) {
        (Some(x), Some(y), Some(width), Some(height)) => Ok(Some(Rect {
            x,
            y,
            width,
            height,
        })),
        _ => Ok(None),
    }
}

fn parse_rect_from_object(object: &str) -> io::Result<Rect> {
    if let Some(rect) = rect_from_field_sets(
        object,
        &["x", "pos_x", "left"],
        &["y", "pos_y", "top"],
        &["width", "w"],
        &["height", "h"],
    )? {
        return Ok(rect);
    }

    for key in ["logical", "geometry", "rect", "window_rect", "layout"] {
        if let Some(value) = optional_json_field(object, key)
            && value.trim_start().starts_with('{')
            && let Some(rect) = rect_from_field_sets(
                value,
                &["x", "pos_x", "left"],
                &["y", "pos_y", "top"],
                &["width", "w"],
                &["height", "h"],
            )?
        {
            return Ok(rect);
        }
    }

    if let (Some(position), Some(size)) = (
        optional_json_field(object, "position"),
        optional_json_field(object, "size"),
    ) && position.trim_start().starts_with('{')
        && size.trim_start().starts_with('{')
    {
        let x = first_json_f64_field(position, &["x", "pos_x", "left"])?;
        let y = first_json_f64_field(position, &["y", "pos_y", "top"])?;
        let width = first_json_f64_field(size, &["width", "w"])?;
        let height = first_json_f64_field(size, &["height", "h"])?;
        if let (Some(x), Some(y), Some(width), Some(height)) = (x, y, width, height) {
            return Ok(Rect {
                x,
                y,
                width,
                height,
            });
        }
    }

    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "unable to parse rectangle from niri json object",
    ))
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

fn parse_outputs_json(contents: &str) -> io::Result<Vec<NiriOutput>> {
    let mut outputs = Vec::new();

    for object in split_top_level_objects(contents)? {
        let name = first_json_string_field(object, &["name"])?.ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "output object missing name")
        })?;
        let rect = parse_rect_from_object(object)?;
        let is_focused = first_json_bool_field(object, &["is_focused"])?.unwrap_or(false);

        outputs.push(NiriOutput {
            name,
            is_focused,
            rect,
        });
    }

    Ok(outputs)
}

fn parse_focused_window_json(contents: &str) -> io::Result<Option<FocusedWindow>> {
    let trimmed = contents.trim();
    if trimmed == "null" {
        return Ok(None);
    }
    if !trimmed.starts_with('{') || !trimmed.ends_with('}') {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "focused-window json is not an object",
        ));
    }

    Ok(Some(FocusedWindow {
        id: first_json_usize_field(trimmed, &["id"])?,
        title: first_json_string_field(trimmed, &["title"])?,
        app_id: first_json_string_field(trimmed, &["app_id", "app-id"])?,
        output: first_json_string_field(trimmed, &["output", "output_name"])?,
        rect: parse_rect_from_object(trimmed)?,
    }))
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

struct LiveNiri {
    socket: Option<PathBuf>,
    connection: Option<NiriConnection>,
    cli_fallback: bool,
}

impl LiveNiri {
    fn connect_from_environment() -> io::Result<Self> {
        let cli_fallback = env::var_os("HOLLOW_GROVE_NIRI_CLI")
            .is_some_and(|value| value == "1" || value == "true");
        let socket = env::var_os("NIRI_SOCKET").map(PathBuf::from);
        if socket.is_none() && !cli_fallback {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "NIRI_SOCKET is not set; set HOLLOW_GROVE_NIRI_CLI=1 only for a CLI compatibility environment",
            ));
        }
        Ok(Self {
            socket,
            connection: None,
            cli_fallback,
        })
    }

    fn cli_output(args: &[&str]) -> io::Result<String> {
        let output = Command::new("niri").args(args).output()?;
        if !output.status.success() {
            return Err(io::Error::other(format!(
                "niri {} exited with {}: {}",
                args.join(" "),
                output.status,
                String::from_utf8_lossy(&output.stderr).trim()
            )));
        }
        String::from_utf8(output.stdout).map_err(|error| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("niri output was not UTF-8: {error}"),
            )
        })
    }

    fn cli_request(request: &Value) -> io::Result<Value> {
        let request = request.as_str().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "Niri request was not a string")
        })?;
        match request {
            "Workspaces" => Ok(json!({
                "Workspaces": serde_json::from_str::<Value>(&Self::cli_output(&["msg", "-j", "workspaces"])?)?
            })),
            "Outputs" => Ok(json!({
                "Outputs": serde_json::from_str::<Value>(&Self::cli_output(&["msg", "-j", "outputs"])?)?
            })),
            "FocusedWindow" => Ok(json!({
                "FocusedWindow": serde_json::from_str::<Value>(&Self::cli_output(&["msg", "-j", "focused-window"])?)?
            })),
            "OverviewState" => Ok(json!({
                "OverviewState": {
                    "is_open": parse_overview_state(&Self::cli_output(&["msg", "overview-state"])?)? == OverviewState::Open
                }
            })),
            other => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("unsupported Niri CLI compatibility request: {other}"),
            )),
        }
    }

    fn request(&mut self, request: &Value) -> io::Result<Value> {
        if self.cli_fallback {
            return Self::cli_request(request);
        }
        let socket = self
            .socket
            .as_ref()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "NIRI_SOCKET is not set"))?;
        if self.connection.is_none() {
            self.connection = Some(NiriConnection::connect(socket)?);
        }
        let result = self
            .connection
            .as_mut()
            .expect("connection was initialized")
            .request(request);
        match result {
            Ok(value) => Ok(value),
            Err(first_error) => {
                self.connection = Some(NiriConnection::connect(socket)?);
                self.connection
                    .as_mut()
                    .expect("connection was reinitialized")
                    .request(request)
                    .map_err(|second_error| {
                        io::Error::other(format!(
                            "Niri IPC request failed after reconnect: {first_error}; {second_error}"
                        ))
                    })
            }
        }
    }

    fn payload(&mut self, request: &str, key: &str) -> io::Result<Value> {
        let response = self.request(&json!(request))?;
        response.get(key).cloned().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Niri {request} response omitted {key}: {response}"),
            )
        })
    }

    fn workspaces(&mut self) -> io::Result<Vec<NiriWorkspace>> {
        parse_workspaces_json(&self.payload("Workspaces", "Workspaces")?.to_string())
    }

    fn overview_state(&mut self) -> io::Result<OverviewState> {
        let payload = self.payload("OverviewState", "OverviewState")?;
        let is_open = payload
            .get("is_open")
            .and_then(Value::as_bool)
            .or_else(|| payload.as_bool())
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("unexpected Niri overview response: {payload}"),
                )
            })?;
        Ok(if is_open {
            OverviewState::Open
        } else {
            OverviewState::Closed
        })
    }

    fn outputs(&mut self) -> io::Result<Vec<NiriOutput>> {
        parse_outputs_json(&self.payload("Outputs", "Outputs")?.to_string())
    }

    fn focused_window(&mut self) -> io::Result<Option<FocusedWindow>> {
        parse_focused_window_json(&self.payload("FocusedWindow", "FocusedWindow")?.to_string())
    }
}

fn escape_json(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}

fn format_json_number(value: f64) -> String {
    let rendered = format!("{value:.6}");
    rendered
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_owned()
}

fn normalized_point(x: f64, y: f64) -> Option<NormalizedPoint> {
    fn coordinate(value: f64) -> Option<NormalizedCoordinate> {
        if !value.is_finite() || !(0.0..=1.0).contains(&value) {
            return None;
        }
        let parts = (value * NORMALIZED_UNIT as f64).round() as u64;
        NormalizedCoordinate::new(parts).ok()
    }

    Some(NormalizedPoint::new(coordinate(x)?, coordinate(y)?))
}

fn normalized_coordinate_json(coordinate: NormalizedCoordinate) -> String {
    format_json_number(coordinate.parts() as f64 / NORMALIZED_UNIT as f64)
}

fn build_spatial_viewpoint_json(center_x: f64, center_y: f64) -> String {
    let Some(root_position) = normalized_point(center_x, center_y) else {
        return String::from("null");
    };
    let spatial_contract = match CANONICAL_SPATIAL_MAP.get_or_init(|| {
        let map = canonical_hollow_grove_spatial_map().map_err(|error| error.to_string())?;
        let report = map
            .validate_conformance()
            .map_err(|error| error.to_string())?;
        let replay_digest =
            hollow_grove_spatial_replay_digest(&map).map_err(|error| error.to_string())?;
        Ok(LiveSpatialContract {
            map,
            catalog_digest: report.catalog_digest,
            template_digest: report.template_digest,
            replay_digest,
        })
    }) {
        Ok(spatial_contract) => spatial_contract,
        Err(_) => return String::from("null"),
    };
    let Some(selection) = spatial_contract
        .map
        .catalog()
        .select_viewpoint(root_position)
    else {
        return String::from("null");
    };
    let viewpoint = selection.selected;

    format!(
        "{{\"frame_id\":\"{}\",\"template_id\":\"{}\",\"scale\":\"{}\",\"depth\":{},\"rule_set\":\"{}\",\"selection_policy\":\"{}\",\"candidate_count\":{},\"catalog_digest\":\"{}\",\"template_digest\":\"{}\",\"replay_digest\":\"{}\",\"root\":{{\"x\":{},\"y\":{}}},\"local\":{{\"x\":{},\"y\":{}}}}}",
        escape_json(viewpoint.frame_id.as_str()),
        escape_json(viewpoint.template_id.as_str()),
        escape_json(viewpoint.scale.as_str()),
        viewpoint.depth,
        escape_json(spatial_contract.map.catalog().rule_set().as_str()),
        escape_json(selection.policy_id),
        selection.candidates.len(),
        escape_json(&spatial_contract.catalog_digest),
        escape_json(&spatial_contract.template_digest),
        escape_json(&spatial_contract.replay_digest),
        normalized_coordinate_json(viewpoint.root_position.x),
        normalized_coordinate_json(viewpoint.root_position.y),
        normalized_coordinate_json(viewpoint.local_position.x),
        normalized_coordinate_json(viewpoint.local_position.y),
    )
}

fn build_screen_map_state_output(
    focused_window: Option<&FocusedWindow>,
    active_output: Option<&NiriOutput>,
    status: &str,
    error: Option<&str>,
) -> String {
    let map_contract_path = "artifacts/hueman_screen_map.json";
    let intent_path = "artifacts/screen_map_intent.json";

    let focused_window_json = match focused_window {
        Some(window) => format!(
            "{{\"id\":{},\"title\":{},\"app_id\":{},\"output\":{},\"rect\":{{\"x\":{},\"y\":{},\"width\":{},\"height\":{}}}}}",
            window
                .id
                .map(|value| value.to_string())
                .unwrap_or_else(|| String::from("null")),
            window
                .title
                .as_deref()
                .map(|value| format!("\"{}\"", escape_json(value)))
                .unwrap_or_else(|| String::from("null")),
            window
                .app_id
                .as_deref()
                .map(|value| format!("\"{}\"", escape_json(value)))
                .unwrap_or_else(|| String::from("null")),
            window
                .output
                .as_deref()
                .map(|value| format!("\"{}\"", escape_json(value)))
                .unwrap_or_else(|| String::from("null")),
            format_json_number(window.rect.x),
            format_json_number(window.rect.y),
            format_json_number(window.rect.width),
            format_json_number(window.rect.height)
        ),
        None => String::from("null"),
    };

    let active_output_json = match active_output {
        Some(output) => format!(
            "{{\"name\":\"{}\",\"is_focused\":{},\"rect\":{{\"x\":{},\"y\":{},\"width\":{},\"height\":{}}}}}",
            escape_json(&output.name),
            output.is_focused,
            format_json_number(output.rect.x),
            format_json_number(output.rect.y),
            format_json_number(output.rect.width),
            format_json_number(output.rect.height)
        ),
        None => String::from("null"),
    };

    let (normalized_json, spatial_viewpoint_json) = match (focused_window, active_output) {
        (Some(window), Some(output)) => {
            let center_x =
                (window.rect.x - output.rect.x + (window.rect.width / 2.0)) / output.rect.width;
            let center_y =
                (window.rect.y - output.rect.y + (window.rect.height / 2.0)) / output.rect.height;
            let rect_x = (window.rect.x - output.rect.x) / output.rect.width;
            let rect_y = (window.rect.y - output.rect.y) / output.rect.height;
            let rect_width = window.rect.width / output.rect.width;
            let rect_height = window.rect.height / output.rect.height;

            (
                format!(
                    "{{\"center\":{{\"x\":{},\"y\":{}}},\"rect\":{{\"x\":{},\"y\":{},\"width\":{},\"height\":{}}}}}",
                    format_json_number(center_x),
                    format_json_number(center_y),
                    format_json_number(rect_x),
                    format_json_number(rect_y),
                    format_json_number(rect_width),
                    format_json_number(rect_height)
                ),
                build_spatial_viewpoint_json(center_x, center_y),
            )
        }
        _ => (String::from("null"), String::from("null")),
    };

    let error_json = error
        .map(|value| format!("\"{}\"", escape_json(value)))
        .unwrap_or_else(|| String::from("null"));

    format!(
        "{{\n  \"schema_version\": \"0.3.0\",\n  \"status\": \"{status}\",\n  \"window_probe\": \"focused_window_center\",\n  \"map_contract_path\": \"{map_contract_path}\",\n  \"intent_path\": \"{intent_path}\",\n  \"focused_window\": {focused_window_json},\n  \"active_output\": {active_output_json},\n  \"normalized\": {normalized_json},\n  \"spatial_viewpoint\": {spatial_viewpoint_json},\n  \"error\": {error_json}\n}}\n"
    )
}

fn write_screen_map_state_at(root: &Path, live: &mut LiveNiri) -> io::Result<PathBuf> {
    let path = root.join(SCREEN_MAP_STATE_ARTIFACT_PATH);

    let output = match (live.focused_window(), live.outputs()) {
        (Ok(focused_window), Ok(outputs)) => {
            let active_output = match focused_window
                .as_ref()
                .and_then(|window| window.output.as_deref())
            {
                Some(output_name) => outputs.iter().find(|output| output.name == output_name),
                None => outputs
                    .iter()
                    .find(|output| output.is_focused)
                    .or_else(|| outputs.first()),
            };

            let status = match (&focused_window, active_output) {
                (Some(_), Some(_)) => "ok",
                (None, _) => "no-focused-window",
                (Some(_), None) => "no-matching-output",
            };

            build_screen_map_state_output(focused_window.as_ref(), active_output, status, None)
        }
        (Err(error), _) => build_screen_map_state_output(
            None,
            None,
            "unavailable",
            Some(&format!("focused-window read failed: {error}")),
        ),
        (_, Err(error)) => build_screen_map_state_output(
            None,
            None,
            "unavailable",
            Some(&format!("outputs read failed: {error}")),
        ),
    };

    write_text_artifact(&path, &output)?;
    Ok(path)
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
         - bridge_memory_contract: `{NIRI_BRIDGE_MEMORY_ARTIFACT_PATH}`\n\
         - screen_map_state_contract: `{SCREEN_MAP_STATE_ARTIFACT_PATH}`\n\n\
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

fn run_bridge_tick(
    root: &Path,
    config: BridgeConfig,
    live: &mut LiveNiri,
) -> io::Result<BridgeTickResult> {
    write_screen_map_state_at(root, live)?;
    let runtime_memory = read_runtime_memory_at(root)?;
    let previous_bridge_memory = read_bridge_memory_at(root)?;

    let runtime_memory = match runtime_memory {
        Some(memory) => memory,
        None => {
            let status = format!(
                "# Hollow Grove Niri Bridge\n\n## Bridge Status\n\n- apply_enabled: false\n- result_kind: waiting\n- result_message: waiting for runtime memory\n- screen_map_state_contract: `{SCREEN_MAP_STATE_ARTIFACT_PATH}`\n"
            );
            let status_path = root.join(NIRI_BRIDGE_STATUS_ARTIFACT_PATH);
            write_text_artifact(&status_path, &status)?;
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

    let workspaces = live.workspaces()?;
    let overview = live.overview_state()?;
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
        live.workspaces()?
    } else {
        workspaces
    };
    let status_overview = if config.apply && !plan.commands.is_empty() {
        live.overview_state()?
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
    let mut live = LiveNiri::connect_from_environment()?;

    loop {
        tick_count += 1;
        let result = run_bridge_tick(Path::new("."), config, &mut live)?;

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
        build_plan, build_screen_map_state_output, build_spatial_viewpoint_json, parse_bridge_cli,
        parse_bridge_memory, parse_focused_window_json, parse_outputs_json, parse_overview_state,
        parse_runtime_memory, parse_workspaces_json, usage, workspace_summary,
    };

    const SCREEN_LIVE_STATE_V0_3_FIXTURE: &str =
        include_str!("../../tests/fixtures/compatibility/screen_live_state_v0_3.json");

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
             last_witness: Point\\n↓\\nTriway\n",
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
    fn outputs_json_parser_reads_logical_rectangles() {
        let outputs = parse_outputs_json(
            r#"[{"name":"DP-2","is_focused":true,"logical":{"x":0,"y":0,"width":2560,"height":1440}}]"#,
        )
        .expect("output parse");
        assert_eq!(outputs.len(), 1);
        assert_eq!(outputs[0].name, "DP-2");
        assert!(outputs[0].is_focused);
        assert_eq!(outputs[0].rect.width, 2560.0);
        assert_eq!(outputs[0].rect.height, 1440.0);
    }

    #[test]
    fn focused_window_parser_reads_geometry() {
        let focused_window = parse_focused_window_json(
            r#"{"id":4,"title":"Terminal","app_id":"kitty","output":"DP-2","geometry":{"x":320,"y":180,"width":1280,"height":720}}"#,
        )
        .expect("focused window parse")
        .expect("focused window should exist");
        assert_eq!(focused_window.id, Some(4));
        assert_eq!(focused_window.output.as_deref(), Some("DP-2"));
        assert_eq!(focused_window.rect.x, 320.0);
        assert_eq!(focused_window.rect.height, 720.0);
    }

    #[test]
    fn screen_map_state_output_renders_normalized_geometry() {
        let focused_window = parse_focused_window_json(
            r#"{"id":4,"title":"Terminal","app_id":"kitty","output":"DP-2","geometry":{"x":320,"y":180,"width":1280,"height":720}}"#,
        )
        .expect("focused window parse")
        .expect("focused window should exist");
        let output = parse_outputs_json(
            r#"[{"name":"DP-2","is_focused":true,"logical":{"x":0,"y":0,"width":2560,"height":1440}}]"#,
        )
        .expect("output parse")
        .pop()
        .expect("one output");

        let rendered =
            build_screen_map_state_output(Some(&focused_window), Some(&output), "ok", None);
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&rendered).expect("live state should parse"),
            serde_json::from_str::<serde_json::Value>(SCREEN_LIVE_STATE_V0_3_FIXTURE)
                .expect("live-state fixture should parse")
        );
        assert!(rendered.contains("\"schema_version\": \"0.3.0\""));
        assert!(rendered.contains("\"status\": \"ok\""));
        assert!(rendered.contains("\"center\":{\"x\":0.375,\"y\":0.375}"));
        assert!(
            rendered.contains("\"rect\":{\"x\":0.125,\"y\":0.125,\"width\":0.5,\"height\":0.5}")
        );
        assert!(rendered.contains("\"frame_id\":\"frame.world.hollow-grove\""));
        assert!(rendered.contains("\"template_id\":\"template.hollow-grove.main-map\""));
        assert!(rendered.contains("\"scale\":\"scale.world\""));
        assert!(rendered.contains("\"rule_set\":\"ruleset.hollow-grove.spatial-inheritance-v1\""));
        assert!(rendered.contains("\"selection_policy\":\"deepest-then-stable-frame-id-v1\""));
        assert!(rendered.contains("\"catalog_digest\":\"fnv1a64-spatial-v1:"));
        assert!(rendered.contains("\"replay_digest\":\"fnv1a64-hollow-grove-spatial-v1:"));
    }

    #[test]
    fn screen_map_state_resolves_house_center_as_inherited_city_viewpoint() {
        let rendered = build_spatial_viewpoint_json(0.5, 0.14);
        assert!(rendered.contains("\"frame_id\":\"frame.city.stonebend\""));
        assert!(rendered.contains("\"scale\":\"scale.city\""));
        assert!(rendered.contains("\"depth\":1"));
        assert!(rendered.contains("\"candidate_count\":2"));
        assert!(rendered.contains("\"local\":{\"x\":0.5,\"y\":0.5}"));
    }

    #[test]
    fn screen_map_state_rejects_offscreen_points_from_spatial_inheritance() {
        assert_eq!(build_spatial_viewpoint_json(-0.01, 0.5), "null");
        assert_eq!(build_spatial_viewpoint_json(0.5, 1.01), "null");
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
