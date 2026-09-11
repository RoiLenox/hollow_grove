use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant, SystemTime};

use hollow_niri::{NiriConnection, WindowInfo};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

const EVENT_WAKE_INTERVAL: Duration = Duration::from_millis(500);
const RECONNECT_DELAY: Duration = Duration::from_millis(500);

#[derive(Debug, Clone)]
pub struct BrokerConfig {
    root: PathBuf,
    preview_interval: Option<Duration>,
}

impl BrokerConfig {
    pub fn parse(args: &[String]) -> Result<Self, String> {
        let mut root = env::var_os("HOLLOW_GROVE_ROOT")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."));
        let mut preview_fps = 1.0_f64;
        let mut index = 0;
        while index < args.len() {
            match args[index].as_str() {
                "--root" => {
                    index += 1;
                    root = args
                        .get(index)
                        .map(PathBuf::from)
                        .ok_or_else(|| String::from("missing path for --root"))?;
                }
                "--preview-fps" => {
                    index += 1;
                    preview_fps = args
                        .get(index)
                        .ok_or_else(|| String::from("missing value for --preview-fps"))?
                        .parse::<f64>()
                        .map_err(|_| String::from("--preview-fps must be a number"))?;
                    if !preview_fps.is_finite() || !(0.0..=10.0).contains(&preview_fps) {
                        return Err(String::from("--preview-fps must be between 0 and 10"));
                    }
                }
                unknown => return Err(format!("unknown broker argument: {unknown}")),
            }
            index += 1;
        }
        let root = root.canonicalize().map_err(|error| {
            format!("could not resolve broker root {}: {error}", root.display())
        })?;
        let preview_interval =
            (preview_fps > 0.0).then(|| Duration::from_secs_f64(1.0 / preview_fps));
        Ok(Self {
            root,
            preview_interval,
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
struct WorkspaceInfo {
    id: u64,
    #[serde(default)]
    output: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
struct LogicalOutput {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    scale: f64,
    transform: String,
}

#[derive(Debug, Clone, Deserialize)]
struct OutputInfo {
    name: String,
    logical: LogicalOutput,
    #[serde(default)]
    vrr_supported: bool,
    #[serde(default)]
    vrr_enabled: bool,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq)]
struct ScreenRect {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

#[derive(Debug, Default)]
struct DesktopState {
    windows: BTreeMap<u64, WindowInfo>,
    workspaces: BTreeMap<u64, WorkspaceInfo>,
    outputs: BTreeMap<String, OutputInfo>,
}

impl DesktopState {
    fn replace_windows(&mut self, windows: Vec<WindowInfo>) {
        self.windows = windows
            .into_iter()
            .map(|window| (window.id, window))
            .collect();
    }

    fn replace_workspaces(&mut self, workspaces: Vec<WorkspaceInfo>) {
        self.workspaces = workspaces
            .into_iter()
            .map(|workspace| (workspace.id, workspace))
            .collect();
    }

    fn output_for_window(&self, window: &WindowInfo) -> Option<&OutputInfo> {
        let workspace = self.workspaces.get(&window.workspace_id?)?;
        self.outputs.get(workspace.output.as_deref()?)
    }

    fn window_rect(&self, window: &WindowInfo) -> Option<ScreenRect> {
        let output = self.output_for_window(window)?;
        let (tile_x, tile_y) = window.layout.tile_pos_in_workspace_view?;
        let (offset_x, offset_y) = window.layout.window_offset_in_tile.unwrap_or((0.0, 0.0));
        let (width, height) = window.layout.window_size?;
        (width > 1 && height > 1).then_some(ScreenRect {
            x: f64::from(output.logical.x) + tile_x + offset_x,
            y: f64::from(output.logical.y) + tile_y + offset_y,
            width: f64::from(width),
            height: f64::from(height),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
struct WindowProjection {
    app_id: Option<String>,
    workspace_id: Option<u64>,
    is_focused: bool,
    is_floating: bool,
    position: Option<(usize, usize)>,
    size: Option<(i32, i32)>,
    tile_position: Option<(u64, u64)>,
    offset: Option<(u64, u64)>,
}

fn finite_pair(value: Option<(f64, f64)>) -> Option<(u64, u64)> {
    value.map(|(x, y)| (x.to_bits(), y.to_bits()))
}

fn window_projection(window: &WindowInfo) -> WindowProjection {
    WindowProjection {
        app_id: window.app_id.clone(),
        workspace_id: window.workspace_id,
        is_focused: window.is_focused,
        is_floating: window.is_floating,
        position: window.layout.pos_in_scrolling_layout,
        size: window.layout.window_size,
        tile_position: finite_pair(window.layout.tile_pos_in_workspace_view),
        offset: finite_pair(window.layout.window_offset_in_tile),
    }
}

fn value_payload(response: Value, key: &str) -> Result<Value, String> {
    response
        .get(key)
        .cloned()
        .ok_or_else(|| format!("Niri {key} response omitted its payload: {response}"))
}

fn refresh_windows(
    connection: &mut NiriConnection,
    state: &mut DesktopState,
) -> Result<(), String> {
    let payload = value_payload(connection.request(&json!("Windows"))?, "Windows")?;
    let windows = serde_json::from_value(payload)
        .map_err(|error| format!("could not decode Niri windows: {error}"))?;
    state.replace_windows(windows);
    Ok(())
}

fn refresh_workspaces(
    connection: &mut NiriConnection,
    state: &mut DesktopState,
) -> Result<(), String> {
    let payload = value_payload(connection.request(&json!("Workspaces"))?, "Workspaces")?;
    let workspaces = serde_json::from_value(payload)
        .map_err(|error| format!("could not decode Niri workspaces: {error}"))?;
    state.replace_workspaces(workspaces);
    Ok(())
}

fn refresh_outputs(
    connection: &mut NiriConnection,
    state: &mut DesktopState,
) -> Result<(), String> {
    let payload = value_payload(connection.request(&json!("Outputs"))?, "Outputs")?;
    state.outputs = serde_json::from_value(payload)
        .map_err(|error| format!("could not decode Niri outputs: {error}"))?;
    Ok(())
}

fn read_json(path: &Path) -> Result<Value, String> {
    let bytes =
        fs::read(path).map_err(|error| format!("could not read {}: {error}", path.display()))?;
    serde_json::from_slice(&bytes)
        .map_err(|error| format!("could not decode {}: {error}", path.display()))
}

fn revision_without_counter(value: &Value) -> Value {
    let mut value = value.clone();
    if let Some(object) = value.as_object_mut() {
        object.remove("revision");
    }
    value
}

fn write_bytes_if_changed(path: &Path, bytes: &[u8]) -> Result<bool, String> {
    if fs::read(path).ok().as_deref() == Some(bytes) {
        return Ok(false);
    }
    let parent = path
        .parent()
        .ok_or_else(|| format!("{} has no parent directory", path.display()))?;
    fs::create_dir_all(parent)
        .map_err(|error| format!("could not create {}: {error}", parent.display()))?;
    let pending = parent.join(format!(
        ".{}.broker-{}",
        path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("state"),
        std::process::id()
    ));
    let mut file = fs::File::create(&pending)
        .map_err(|error| format!("could not create {}: {error}", pending.display()))?;
    file.write_all(bytes)
        .map_err(|error| format!("could not write {}: {error}", pending.display()))?;
    drop(file);
    fs::rename(&pending, path)
        .map_err(|error| format!("could not publish {}: {error}", path.display()))?;
    Ok(true)
}

fn publish_revisioned(path: &Path, mut value: Value) -> Result<bool, String> {
    let existing = read_json(path).ok();
    if existing.as_ref().is_some_and(|current| {
        revision_without_counter(current) == revision_without_counter(&value)
    }) {
        return Ok(false);
    }
    let revision = existing
        .as_ref()
        .and_then(|current| current.get("revision"))
        .and_then(Value::as_u64)
        .unwrap_or(0)
        .saturating_add(1);
    value
        .as_object_mut()
        .ok_or_else(|| String::from("broker output must be a JSON object"))?
        .insert(String::from("revision"), json!(revision));
    let mut bytes = serde_json::to_vec_pretty(&value)
        .map_err(|error| format!("could not encode {}: {error}", path.display()))?;
    bytes.push(b'\n');
    write_bytes_if_changed(path, &bytes)
}

fn output_contract(output: Option<&OutputInfo>) -> Value {
    output.map_or(Value::Null, |output| {
        json!({
            "name": output.name,
            "is_focused": false,
            "rect": output.logical,
            "vrr_supported": output.vrr_supported,
            "vrr_enabled": output.vrr_enabled,
        })
    })
}

fn window_contract(state: &DesktopState, window: &WindowInfo) -> Value {
    let output = state.output_for_window(window);
    let rect = state.window_rect(window);
    json!({
        "id": window.id,
        "title": window.title,
        "app_id": window.app_id,
        "output": output.map(|value| value.name.as_str()),
        "workspace_id": window.workspace_id,
        "rect": rect,
    })
}

fn normalized_contract(state: &DesktopState, window: &WindowInfo) -> Option<Value> {
    let output = state.output_for_window(window)?;
    let rect = state.window_rect(window)?;
    let width = f64::from(output.logical.width);
    let height = f64::from(output.logical.height);
    if width <= 0.0 || height <= 0.0 {
        return None;
    }
    let relative_x = rect.x - f64::from(output.logical.x);
    let relative_y = rect.y - f64::from(output.logical.y);
    Some(json!({
        "center": {
            "x": (relative_x + rect.width / 2.0) / width,
            "y": (relative_y + rect.height / 2.0) / height,
        },
        "rect": {
            "x": relative_x / width,
            "y": relative_y / height,
            "width": rect.width / width,
            "height": rect.height / height,
        }
    }))
}

fn nested_value<'a>(value: &'a Value, path: &[&str]) -> Option<&'a Value> {
    path.iter()
        .try_fold(value, |current, key| current.get(*key))
}

fn publish_pair_state(config: &BrokerConfig, state: &DesktopState) -> Result<Option<u64>, String> {
    let path = config.root.join("artifacts/hueman_pair_state.json");
    let mut pair = match read_json(&path) {
        Ok(value) => value,
        Err(_) if !path.exists() => return Ok(None),
        Err(error) => return Err(error),
    };
    let pair_id = pair
        .get("focused_window")
        .and_then(|window| window.get("id"))
        .and_then(Value::as_u64);
    let object = pair
        .as_object_mut()
        .ok_or_else(|| String::from("Hueman pair state must be a JSON object"))?;
    object.insert(String::from("schema_version"), json!("0.3.0"));
    object.insert(String::from("paired_window_mode"), json!(true));
    object.insert(String::from("sticky_actor"), json!(true));

    let Some(pair_id) = pair_id else {
        object.insert(String::from("binding_status"), json!("waiting"));
        object.insert(String::from("normalized"), Value::Null);
        publish_revisioned(&path, pair)?;
        return Ok(None);
    };
    let Some(window) = state.windows.get(&pair_id) else {
        object.insert(String::from("binding_status"), json!("window-missing"));
        object.insert(String::from("normalized"), Value::Null);
        object.insert(String::from("active_output"), Value::Null);
        publish_revisioned(&path, pair)?;
        return Ok(Some(pair_id));
    };
    let output = state.output_for_window(window);
    let actual_normalized = normalized_contract(state, window);
    let application_anchor = nested_value(
        &pair,
        &["application_attachment", "world_anchor", "normalized"],
    )
    .cloned();
    let normalized = match (application_anchor, actual_normalized) {
        (Some(anchor), Some(mut actual)) => {
            if let Some(actual_object) = actual.as_object_mut() {
                actual_object.insert(String::from("center"), anchor);
            }
            actual
        }
        (Some(anchor), None) => json!({"center": anchor, "rect": null}),
        (None, Some(actual)) => actual,
        (None, None) => Value::Null,
    };
    let attached = pair
        .get("application_attachment")
        .is_some_and(|value| !value.is_null());
    let status = if output.is_none() {
        "no-matching-output"
    } else if state.window_rect(window).is_none() {
        "geometry-unavailable"
    } else if attached {
        "attached"
    } else {
        "ok"
    };
    let object = pair.as_object_mut().expect("pair object was validated");
    object.insert(
        String::from("probe_source"),
        json!(if attached {
            "application_world_anchor"
        } else {
            "paired_window_center"
        }),
    );
    object.insert(String::from("binding_status"), json!(status));
    object.insert(
        String::from("focused_window"),
        window_contract(state, window),
    );
    object.insert(String::from("active_output"), output_contract(output));
    object.insert(String::from("normalized"), normalized);
    publish_revisioned(&path, pair)?;
    Ok(Some(pair_id))
}

fn publish_screen_map(config: &BrokerConfig, state: &DesktopState) -> Result<(), String> {
    let path = config.root.join("artifacts/screen_map_state.json");
    let focused = state.windows.values().find(|window| window.is_focused);
    let (status, window, output, normalized, error) = match focused {
        None => (
            "no-focused-window",
            Value::Null,
            Value::Null,
            Value::Null,
            Value::Null,
        ),
        Some(window) => {
            let output = state.output_for_window(window);
            let normalized = normalized_contract(state, window);
            let status = if output.is_none() {
                "no-matching-output"
            } else if normalized.is_none() {
                "geometry-unavailable"
            } else {
                "ok"
            };
            (
                status,
                window_contract(state, window),
                output_contract(output),
                normalized.unwrap_or(Value::Null),
                if status == "geometry-unavailable" {
                    json!("Niri did not expose tile_pos_in_workspace_view for this window")
                } else {
                    Value::Null
                },
            )
        }
    };
    publish_revisioned(
        &path,
        json!({
            "schema_version": "0.4.0",
            "status": status,
            "window_probe": "niri_event_stream",
            "focused_window": window,
            "active_output": output,
            "normalized": normalized,
            "spatial_viewpoint": null,
            "error": error,
        }),
    )?;
    Ok(())
}

fn capture_allowed(pair: &Value) -> bool {
    let managed = pair
        .get("focused_window")
        .and_then(|window| window.get("app_id"))
        .and_then(Value::as_str)
        .is_some_and(|app_id| app_id.starts_with("hollow-grove."));
    !managed
        || nested_value(
            pair,
            &["application_attachment", "privacy", "capture_allowed"],
        )
        .and_then(Value::as_bool)
        .unwrap_or(false)
}

fn publish_preview_status(config: &BrokerConfig, status: &str, detail: &str) -> Result<(), String> {
    publish_revisioned(
        &config.root.join("artifacts/hueman_pair_preview_state.json"),
        json!({
            "schema_version": "0.3.0",
            "status": status,
            "detail": detail,
        }),
    )?;
    Ok(())
}

fn maybe_capture_preview(
    config: &BrokerConfig,
    state: &DesktopState,
    pair_id: Option<u64>,
    dirty: &mut bool,
    last_capture: &mut Option<Instant>,
) -> Result<(), String> {
    if !*dirty {
        return Ok(());
    }
    let Some(interval) = config.preview_interval else {
        publish_preview_status(
            config,
            "disabled",
            "preview capture disabled by broker configuration",
        )?;
        *dirty = false;
        return Ok(());
    };
    if last_capture.is_some_and(|last| last.elapsed() < interval) {
        return Ok(());
    }
    let pair_path = config.root.join("artifacts/hueman_pair_state.json");
    let pair = match read_json(&pair_path) {
        Ok(value) => value,
        Err(_) => {
            publish_preview_status(config, "waiting", "pair state not found")?;
            *dirty = false;
            return Ok(());
        }
    };
    let preview_path = config.root.join("artifacts/hueman_pair_preview.png");
    if !capture_allowed(&pair) {
        let _ = fs::remove_file(&preview_path);
        publish_preview_status(
            config,
            "masked",
            "managed surface uses semantic-only projection",
        )?;
        *dirty = false;
        return Ok(());
    }
    let Some(window) = pair_id.and_then(|id| state.windows.get(&id)) else {
        publish_preview_status(config, "waiting", "paired window is unavailable")?;
        *dirty = false;
        return Ok(());
    };
    let Some(rect) = state.window_rect(window) else {
        publish_preview_status(
            config,
            "unavailable",
            "Niri did not expose capture geometry for this tiled window",
        )?;
        *dirty = false;
        return Ok(());
    };
    let geometry = format!(
        "{},{} {}x{}",
        rect.x.floor() as i64,
        rect.y.floor() as i64,
        rect.width.floor() as i64,
        rect.height.floor() as i64
    );
    let pending = preview_path.with_extension(format!("broker-{}.png", std::process::id()));
    let result = Command::new("grim")
        .args(["-g", &geometry])
        .arg(&pending)
        .status();
    match result {
        Ok(status) if status.success() => {
            fs::rename(&pending, &preview_path)
                .map_err(|error| format!("could not publish preview: {error}"))?;
            publish_preview_status(config, "ok", &format!("captured {geometry}"))?;
            *last_capture = Some(Instant::now());
        }
        Ok(status) => {
            let _ = fs::remove_file(&pending);
            publish_preview_status(config, "unavailable", &format!("grim exited with {status}"))?;
        }
        Err(error) => {
            publish_preview_status(
                config,
                "unavailable",
                &format!("could not start grim: {error}"),
            )?;
        }
    }
    *dirty = false;
    Ok(())
}

fn pair_modified_time(config: &BrokerConfig) -> Option<SystemTime> {
    fs::metadata(config.root.join("artifacts/hueman_pair_state.json"))
        .and_then(|metadata| metadata.modified())
        .ok()
}

fn handle_event(
    event: Value,
    connection: &mut NiriConnection,
    state: &mut DesktopState,
    pair_id: Option<u64>,
) -> Result<(bool, bool), String> {
    let Some((kind, payload)) = event.as_object().and_then(|object| object.iter().next()) else {
        return Ok((false, false));
    };
    match kind.as_str() {
        "WindowsChanged" => {
            let windows = payload
                .get("windows")
                .cloned()
                .ok_or_else(|| String::from("WindowsChanged omitted windows"))?;
            state.replace_windows(
                serde_json::from_value(windows)
                    .map_err(|error| format!("could not decode WindowsChanged: {error}"))?,
            );
            Ok((true, true))
        }
        "WindowOpenedOrChanged" => {
            let window: WindowInfo = serde_json::from_value(
                payload
                    .get("window")
                    .cloned()
                    .ok_or_else(|| String::from("WindowOpenedOrChanged omitted window"))?,
            )
            .map_err(|error| format!("could not decode WindowOpenedOrChanged: {error}"))?;
            let relevant_change = state
                .windows
                .get(&window.id)
                .is_none_or(|previous| window_projection(previous) != window_projection(&window));
            let preview_dirty = pair_id == Some(window.id);
            state.windows.insert(window.id, window);
            Ok((relevant_change, preview_dirty))
        }
        "WindowClosed" => {
            let id = payload.get("id").and_then(Value::as_u64);
            let removed = id.and_then(|id| state.windows.remove(&id)).is_some();
            Ok((removed, id == pair_id))
        }
        "WorkspacesChanged" => {
            let workspaces = payload
                .get("workspaces")
                .cloned()
                .ok_or_else(|| String::from("WorkspacesChanged omitted workspaces"))?;
            state.replace_workspaces(
                serde_json::from_value(workspaces)
                    .map_err(|error| format!("could not decode WorkspacesChanged: {error}"))?,
            );
            Ok((true, true))
        }
        kind if kind.starts_with("Workspace") => {
            refresh_workspaces(connection, state)?;
            refresh_windows(connection, state)?;
            Ok((true, true))
        }
        kind if kind.starts_with("Output") || kind == "ConfigLoaded" => {
            refresh_outputs(connection, state)?;
            refresh_workspaces(connection, state)?;
            refresh_windows(connection, state)?;
            Ok((true, true))
        }
        _ => Ok((false, false)),
    }
}

fn run_session(config: &BrokerConfig) -> Result<(), String> {
    let socket = env::var("NIRI_SOCKET")
        .map_err(|_| String::from("NIRI_SOCKET is not set; run the broker inside Niri"))?;
    let mut connection = NiriConnection::connect(&socket)
        .map_err(|error| format!("could not connect to Niri: {error}"))?;
    let event_connection = NiriConnection::connect(&socket)
        .map_err(|error| format!("could not connect to Niri event stream: {error}"))?;
    let mut events = event_connection.into_event_stream(EVENT_WAKE_INTERVAL)?;
    let mut state = DesktopState::default();
    refresh_outputs(&mut connection, &mut state)?;
    refresh_workspaces(&mut connection, &mut state)?;
    refresh_windows(&mut connection, &mut state)?;

    let mut pair_id = publish_pair_state(config, &state)?;
    publish_screen_map(config, &state)?;
    let mut preview_dirty = true;
    let mut last_capture = None;
    let mut pair_mtime = pair_modified_time(config);

    loop {
        if let Some(event) = events.next_event()? {
            let (state_dirty, capture_dirty) =
                handle_event(event, &mut connection, &mut state, pair_id)?;
            preview_dirty |= capture_dirty;
            if state_dirty {
                pair_id = publish_pair_state(config, &state)?;
                publish_screen_map(config, &state)?;
                pair_mtime = pair_modified_time(config);
            }
        }

        let current_pair_mtime = pair_modified_time(config);
        if current_pair_mtime != pair_mtime {
            pair_id = publish_pair_state(config, &state)?;
            pair_mtime = pair_modified_time(config);
            preview_dirty = true;
        }
        maybe_capture_preview(
            config,
            &state,
            pair_id,
            &mut preview_dirty,
            &mut last_capture,
        )?;
    }
}

pub fn run(config: BrokerConfig) -> Result<(), String> {
    loop {
        if let Err(error) = run_session(&config) {
            eprintln!("hollow-niri broker: {error}; reconnecting");
            thread::sleep(RECONNECT_DELAY);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn revision_comparison_ignores_only_the_revision_counter() {
        let left = json!({"revision": 1, "status": "ok"});
        let right = json!({"revision": 99, "status": "ok"});
        assert_eq!(
            revision_without_counter(&left),
            revision_without_counter(&right)
        );
    }

    #[test]
    fn broker_cli_accepts_disabled_preview() {
        let root = env::current_dir().expect("current directory");
        let config = BrokerConfig::parse(&[
            String::from("--root"),
            root.display().to_string(),
            String::from("--preview-fps"),
            String::from("0"),
        ])
        .expect("broker configuration");
        assert!(config.preview_interval.is_none());
    }
}
