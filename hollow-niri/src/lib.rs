use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::io::{self, BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

pub const PROTOCOL_VERSION: u8 = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Direction {
    Left,
    Right,
    LeftTop,
    LeftBottom,
    RightTop,
    RightBottom,
    Up,
    Down,
    Top,
    Bottom,
}

impl Direction {
    pub fn parse(value: &str) -> Result<Self, String> {
        match value {
            "left" => Ok(Self::Left),
            "right" => Ok(Self::Right),
            "left-top" => Ok(Self::LeftTop),
            "left-bottom" => Ok(Self::LeftBottom),
            "right-top" => Ok(Self::RightTop),
            "right-bottom" => Ok(Self::RightBottom),
            "up" => Ok(Self::Up),
            "down" => Ok(Self::Down),
            "top" => Ok(Self::Top),
            "bottom" => Ok(Self::Bottom),
            _ => Err(format!("invalid direction: {value}")),
        }
    }

    pub fn accepts_height(self) -> bool {
        matches!(
            self,
            Self::Left
                | Self::Right
                | Self::LeftTop
                | Self::LeftBottom
                | Self::RightTop
                | Self::RightBottom
                | Self::Top
                | Self::Bottom
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HeightStep {
    Grow,
    Shrink,
}

impl HeightStep {
    pub fn parse(value: &str) -> Result<Self, String> {
        match value {
            "grow" => Ok(Self::Grow),
            "shrink" => Ok(Self::Shrink),
            _ => Err(format!("invalid height step: {value}")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HeightFlow {
    Down,
    Up,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct HeightIntentState {
    pub window_id: u64,
    pub target_percentage: f64,
    pub updated_at_millis: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HeightIntentOutcome {
    pub actions: usize,
    pub state: Option<HeightIntentState>,
}

impl HeightFlow {
    pub fn parse(value: &str) -> Result<Self, String> {
        match value {
            "down" => Ok(Self::Down),
            "up" => Ok(Self::Up),
            _ => Err(format!("invalid height flow: {value}")),
        }
    }

    fn anchor(self) -> Direction {
        match self {
            Self::Down => Direction::Top,
            Self::Up => Direction::Bottom,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetHeight {
    Fixed(i32),
    Proportion(f64),
}

impl TargetHeight {
    pub fn parse(value: &str) -> Result<Self, String> {
        if let Some(percent) = value.strip_suffix('%') {
            let number = percent
                .parse::<f64>()
                .map_err(|_| format!("invalid height: {value}"))?;
            if !number.is_finite() || !(0.0..=100.0).contains(&number) {
                return Err(format!(
                    "height percentage must be between 0% and 100%: {value}"
                ));
            }
            // Niri's IPC SizeChange representation keeps percentage points:
            // `50%` is `SetProportion(50.0)`, not a normalized 0.5 fraction.
            Ok(Self::Proportion(number))
        } else {
            let pixels = value
                .parse::<i32>()
                .map_err(|_| format!("invalid height: {value}"))?;
            if pixels < 0 {
                return Err(format!("height must not be negative: {value}"));
            }
            Ok(Self::Fixed(pixels))
        }
    }

    fn niri_value(self) -> Value {
        match self {
            Self::Fixed(value) => json!({ "SetFixed": value }),
            Self::Proportion(value) => json!({ "SetProportion": value }),
        }
    }
}

impl Default for TargetHeight {
    fn default() -> Self {
        Self::Proportion(50.0)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MoveMessage {
    pub protocol: u8,
    pub niri_socket: String,
    pub direction: Direction,
    pub target_height: Option<TargetHeight>,
}

impl MoveMessage {
    pub fn new(
        niri_socket: String,
        direction: Direction,
        target_height: Option<TargetHeight>,
    ) -> Self {
        Self {
            protocol: PROTOCOL_VERSION,
            niri_socket,
            direction,
            target_height,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.protocol != PROTOCOL_VERSION {
            return Err(format!(
                "unsupported Hollow Niri protocol {}; expected {}",
                self.protocol, PROTOCOL_VERSION
            ));
        }
        if self.niri_socket.is_empty() {
            return Err(String::from("NIRI_SOCKET is empty"));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct WindowInfo {
    pub id: u64,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub app_id: Option<String>,
    #[serde(default)]
    pub pid: Option<i32>,
    pub workspace_id: Option<u64>,
    pub is_focused: bool,
    pub is_floating: bool,
    pub layout: WindowLayout,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct WindowLayout {
    pub pos_in_scrolling_layout: Option<(usize, usize)>,
    #[serde(default)]
    pub window_size: Option<(i32, i32)>,
    #[serde(default)]
    pub tile_size: Option<(f64, f64)>,
    #[serde(default)]
    pub tile_pos_in_workspace_view: Option<(f64, f64)>,
    #[serde(default)]
    pub window_offset_in_tile: Option<(f64, f64)>,
}

#[derive(Debug, Clone, Deserialize)]
struct WorkspaceOutputInfo {
    id: u64,
    #[serde(default)]
    output: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct HeightOutputInfo {
    logical: HeightLogicalOutput,
}

#[derive(Debug, Clone, Deserialize)]
struct HeightLogicalOutput {
    height: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PlannedAction {
    MoveColumnLeft,
    MoveColumnRight,
    MoveColumnToFirst,
    MoveColumnToLast,
    MoveWindowUp,
    MoveWindowDown,
    ConsumeOrExpelLeft { id: u64 },
    ConsumeOrExpelRight { id: u64 },
    SetWindowHeight { id: u64, height: TargetHeight },
}

impl PlannedAction {
    pub fn request(&self) -> Value {
        let action = match self {
            Self::MoveColumnLeft => json!({ "MoveColumnLeft": {} }),
            Self::MoveColumnRight => json!({ "MoveColumnRight": {} }),
            Self::MoveColumnToFirst => json!({ "MoveColumnToFirst": {} }),
            Self::MoveColumnToLast => json!({ "MoveColumnToLast": {} }),
            Self::MoveWindowUp => json!({ "MoveWindowUp": {} }),
            Self::MoveWindowDown => json!({ "MoveWindowDown": {} }),
            Self::ConsumeOrExpelLeft { id } => {
                json!({ "ConsumeOrExpelWindowLeft": { "id": id } })
            }
            Self::ConsumeOrExpelRight { id } => {
                json!({ "ConsumeOrExpelWindowRight": { "id": id } })
            }
            Self::SetWindowHeight { id, height } => json!({
                "SetWindowHeight": {
                    "id": id,
                    "change": height.niri_value()
                }
            }),
        };
        json!({ "Action": action })
    }
}

pub fn plan_move(
    windows: &[WindowInfo],
    direction: Direction,
    target_height: Option<TargetHeight>,
) -> Vec<PlannedAction> {
    let Some(focused) = windows.iter().find(|window| window.is_focused) else {
        return Vec::new();
    };
    if focused.is_floating {
        return Vec::new();
    }
    let (Some(workspace_id), Some((source_col, _))) =
        (focused.workspace_id, focused.layout.pos_in_scrolling_layout)
    else {
        return Vec::new();
    };

    let mut workspace_windows: Vec<&WindowInfo> = windows
        .iter()
        .filter(|window| {
            !window.is_floating
                && window.workspace_id == Some(workspace_id)
                && window.layout.pos_in_scrolling_layout.is_some()
        })
        .collect();
    workspace_windows.sort_by_key(|window| window.layout.pos_in_scrolling_layout);

    match direction {
        Direction::Left | Direction::Right => plan_horizontal(
            &workspace_windows,
            focused.id,
            source_col,
            direction,
            None,
            false,
            target_height,
        ),
        Direction::LeftTop
        | Direction::LeftBottom
        | Direction::RightTop
        | Direction::RightBottom => {
            let (side, edge) = match direction {
                Direction::LeftTop => (Direction::Left, Direction::Top),
                Direction::LeftBottom => (Direction::Left, Direction::Bottom),
                Direction::RightTop => (Direction::Right, Direction::Top),
                Direction::RightBottom => (Direction::Right, Direction::Bottom),
                _ => unreachable!(),
            };
            plan_horizontal(
                &workspace_windows,
                focused.id,
                source_col,
                side,
                Some(edge),
                true,
                target_height,
            )
        }
        Direction::Top | Direction::Bottom => plan_stack_edge(
            &workspace_windows,
            focused.id,
            source_col,
            direction,
            target_height.unwrap_or_default(),
        ),
        Direction::Up | Direction::Down => {
            plan_vertical(&workspace_windows, focused.id, source_col, direction)
        }
    }
}

fn column_windows<'a>(windows: &'a [&WindowInfo], column: usize) -> Vec<&'a WindowInfo> {
    let mut result: Vec<&WindowInfo> = windows
        .iter()
        .copied()
        .filter(|window| {
            window
                .layout
                .pos_in_scrolling_layout
                .is_some_and(|(col, _)| col == column)
        })
        .collect();
    result.sort_by_key(|window| window.layout.pos_in_scrolling_layout.map(|(_, row)| row));
    result
}

fn plan_horizontal(
    windows: &[&WindowInfo],
    focused_id: u64,
    source_col: usize,
    direction: Direction,
    target_edge: Option<Direction>,
    force_join: bool,
    requested_height: Option<TargetHeight>,
) -> Vec<PlannedAction> {
    let preserved_height = windows
        .iter()
        .find(|window| window.id == focused_id)
        .and_then(|window| window.layout.window_size.map(|(_, height)| height))
        .map(TargetHeight::Fixed);
    let target_height = requested_height.or(preserved_height);

    let columns: Vec<usize> = windows
        .iter()
        .filter_map(|window| window.layout.pos_in_scrolling_layout.map(|(col, _)| col))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    if columns.len() <= 1 {
        return requested_height
            .map(|height| {
                vec![PlannedAction::SetWindowHeight {
                    id: focused_id,
                    height,
                }]
            })
            .unwrap_or_default();
    }

    let Some(current_index) = columns.iter().position(|column| *column == source_col) else {
        return Vec::new();
    };
    let target_index = match direction {
        Direction::Left => (current_index + columns.len() - 1) % columns.len(),
        Direction::Right => (current_index + 1) % columns.len(),
        _ => unreachable!("horizontal planner received a non-horizontal direction"),
    };
    let target_col = columns[target_index];
    let source = column_windows(windows, source_col);
    let target = column_windows(windows, target_col);
    let Some(source_index) = source.iter().position(|window| window.id == focused_id) else {
        return Vec::new();
    };

    let mut actions = Vec::new();
    if !force_join && windows.len() == 2 && source.len() == 1 && target.len() == 1 {
        actions.push(match (direction, current_index) {
            (Direction::Left, 0) => PlannedAction::MoveColumnToLast,
            (Direction::Left, _) => PlannedAction::MoveColumnLeft,
            (Direction::Right, index) if index == columns.len() - 1 => {
                PlannedAction::MoveColumnToFirst
            }
            (Direction::Right, _) => PlannedAction::MoveColumnRight,
            _ => unreachable!(),
        });
        if let Some(height) = target_height {
            actions.push(PlannedAction::SetWindowHeight {
                id: focused_id,
                height,
            });
        }
        return actions;
    }

    let at_edge = matches!(direction, Direction::Left) && current_index == 0
        || matches!(direction, Direction::Right) && current_index == columns.len() - 1;
    let consume = || match direction {
        Direction::Left => PlannedAction::ConsumeOrExpelLeft { id: focused_id },
        Direction::Right => PlannedAction::ConsumeOrExpelRight { id: focused_id },
        _ => unreachable!(),
    };

    if at_edge {
        if source.len() > 1 {
            actions.push(consume());
        }
        actions.push(match direction {
            Direction::Left => PlannedAction::MoveColumnToLast,
            Direction::Right => PlannedAction::MoveColumnToFirst,
            _ => unreachable!(),
        });
        actions.push(consume());
    } else if source.len() > 1 {
        actions.push(consume());
        actions.push(consume());
    } else {
        actions.push(consume());
    }

    if !target.is_empty() {
        let desired_index = match target_edge {
            Some(Direction::Top) => 0,
            Some(Direction::Bottom) => target.len(),
            None if source.len() <= 1 => 0,
            None => {
                ((source_index * target.len()) as f64 / (source.len() - 1) as f64).round() as usize
            }
            _ => unreachable!("horizontal target edge must be top or bottom"),
        };
        for _ in 0..target.len().saturating_sub(desired_index) {
            actions.push(PlannedAction::MoveWindowUp);
        }
    }
    if let Some(height) = target_height {
        actions.push(PlannedAction::SetWindowHeight {
            id: focused_id,
            height,
        });
    }
    actions
}

fn plan_stack_edge(
    windows: &[&WindowInfo],
    focused_id: u64,
    source_col: usize,
    direction: Direction,
    target_height: TargetHeight,
) -> Vec<PlannedAction> {
    let column = column_windows(windows, source_col);
    let Some(current_index) = column.iter().position(|window| window.id == focused_id) else {
        return Vec::new();
    };
    let move_count = match direction {
        Direction::Top => current_index,
        Direction::Bottom => column.len().saturating_sub(current_index + 1),
        _ => unreachable!("stack-edge planner received another direction"),
    };
    let movement = match direction {
        Direction::Top => PlannedAction::MoveWindowUp,
        Direction::Bottom => PlannedAction::MoveWindowDown,
        _ => unreachable!(),
    };
    let mut actions = vec![movement; move_count];
    actions.push(PlannedAction::SetWindowHeight {
        id: focused_id,
        height: target_height,
    });
    actions
}

fn plan_vertical(
    windows: &[&WindowInfo],
    focused_id: u64,
    source_col: usize,
    direction: Direction,
) -> Vec<PlannedAction> {
    let column = column_windows(windows, source_col);
    let Some(current_index) = column.iter().position(|window| window.id == focused_id) else {
        return Vec::new();
    };
    match direction {
        Direction::Down if current_index == column.len() - 1 => {
            vec![PlannedAction::MoveWindowUp; column.len().saturating_sub(1)]
        }
        Direction::Down => vec![PlannedAction::MoveWindowDown],
        Direction::Up if current_index == 0 => {
            vec![PlannedAction::MoveWindowDown; column.len().saturating_sub(1)]
        }
        Direction::Up => vec![PlannedAction::MoveWindowUp],
        _ => unreachable!("vertical planner received another direction"),
    }
}

const QUARTER_HEIGHTS: [f64; 4] = [25.0, 50.0, 75.0, 100.0];
const HEIGHT_STEP_TOLERANCE: f64 = 0.5;
const HEIGHT_INTENT_GRACE_MILLIS: u64 = 1_500;

pub fn next_quarter_height(current_percentage: f64, step: HeightStep) -> Option<f64> {
    if !current_percentage.is_finite() {
        return None;
    }
    match step {
        HeightStep::Grow => QUARTER_HEIGHTS
            .into_iter()
            .find(|height| *height > current_percentage + HEIGHT_STEP_TOLERANCE),
        HeightStep::Shrink => QUARTER_HEIGHTS
            .into_iter()
            .rev()
            .find(|height| *height < current_percentage - HEIGHT_STEP_TOLERANCE),
    }
}

fn focused_height_percentage(
    windows: &[WindowInfo],
    output_height: f64,
) -> Option<(&WindowInfo, f64)> {
    if !output_height.is_finite() || output_height <= 0.0 {
        return None;
    }
    let focused = windows
        .iter()
        .find(|window| window.is_focused && !window.is_floating)?;
    let current_height = focused
        .layout
        .tile_size
        .map(|(_, height)| height)
        .or_else(|| {
            focused
                .layout
                .window_size
                .map(|(_, height)| f64::from(height))
        })
        .filter(|height| height.is_finite())?;
    Some((focused, current_height * 100.0 / output_height))
}

pub fn plan_height_step(
    windows: &[WindowInfo],
    output_height: f64,
    step: HeightStep,
) -> Vec<PlannedAction> {
    let Some((_, current_percentage)) = focused_height_percentage(windows, output_height) else {
        return Vec::new();
    };
    let Some(target) = next_quarter_height(current_percentage, step) else {
        return Vec::new();
    };
    plan_move(
        windows,
        Direction::Top,
        Some(TargetHeight::Proportion(target)),
    )
}

fn effective_height_percentage(
    window_id: u64,
    current_percentage: f64,
    previous: Option<HeightIntentState>,
    now_millis: u64,
) -> f64 {
    previous
        .filter(|state| {
            let target_is_valid = QUARTER_HEIGHTS.contains(&state.target_percentage);
            let geometry_reached_target =
                (state.target_percentage - current_percentage).abs() <= HEIGHT_STEP_TOLERANCE;
            let command_is_recent = now_millis
                .checked_sub(state.updated_at_millis)
                .is_some_and(|age| age <= HEIGHT_INTENT_GRACE_MILLIS);
            state.window_id == window_id
                && target_is_valid
                && (geometry_reached_target || command_is_recent)
        })
        .map_or(current_percentage, |state| state.target_percentage)
}

pub fn plan_height_cycle(
    windows: &[WindowInfo],
    output_height: f64,
    previous: Option<HeightIntentState>,
    now_millis: u64,
) -> (Vec<PlannedAction>, Option<HeightIntentState>) {
    let Some((focused, current_percentage)) = focused_height_percentage(windows, output_height)
    else {
        return (Vec::new(), None);
    };
    let effective_percentage =
        effective_height_percentage(focused.id, current_percentage, previous, now_millis);
    let target = if (effective_percentage - 75.0).abs() <= HEIGHT_STEP_TOLERANCE {
        25.0
    } else {
        75.0
    };
    (
        plan_move(
            windows,
            Direction::Top,
            Some(TargetHeight::Proportion(target)),
        ),
        Some(HeightIntentState {
            window_id: focused.id,
            target_percentage: target,
            updated_at_millis: now_millis,
        }),
    )
}

pub fn next_height_flow_target(current_percentage: f64) -> Option<f64> {
    if !current_percentage.is_finite() {
        return None;
    }
    Some(next_quarter_height(current_percentage, HeightStep::Grow).unwrap_or(25.0))
}

pub fn plan_height_flow(
    windows: &[WindowInfo],
    output_height: f64,
    flow: HeightFlow,
    previous: Option<HeightIntentState>,
    now_millis: u64,
) -> (Vec<PlannedAction>, Option<HeightIntentState>) {
    let Some((focused, current_percentage)) = focused_height_percentage(windows, output_height)
    else {
        return (Vec::new(), None);
    };
    let effective_percentage =
        effective_height_percentage(focused.id, current_percentage, previous, now_millis);
    let Some(target) = next_height_flow_target(effective_percentage) else {
        return (Vec::new(), None);
    };
    (
        plan_move(
            windows,
            flow.anchor(),
            Some(TargetHeight::Proportion(target)),
        ),
        Some(HeightIntentState {
            window_id: focused.id,
            target_percentage: target,
            updated_at_millis: now_millis,
        }),
    )
}

pub struct NiriConnection {
    path: PathBuf,
    reader: BufReader<UnixStream>,
}

pub struct NiriEventStream {
    reader: BufReader<UnixStream>,
}

impl fmt::Debug for NiriEventStream {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("NiriEventStream")
            .finish_non_exhaustive()
    }
}

impl fmt::Debug for NiriConnection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("NiriConnection")
            .field("path", &self.path)
            .finish_non_exhaustive()
    }
}

impl NiriConnection {
    pub fn connect(path: impl AsRef<Path>) -> io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        let stream = UnixStream::connect(&path)?;
        stream.set_read_timeout(Some(Duration::from_secs(2)))?;
        stream.set_write_timeout(Some(Duration::from_secs(2)))?;
        Ok(Self {
            path,
            reader: BufReader::new(stream),
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn request(&mut self, request: &Value) -> Result<Value, String> {
        {
            let stream = self.reader.get_mut();
            serde_json::to_writer(&mut *stream, request)
                .map_err(|error| format!("could not serialize Niri request: {error}"))?;
            stream
                .write_all(b"\n")
                .map_err(|error| format!("could not write Niri request: {error}"))?;
            stream
                .flush()
                .map_err(|error| format!("could not flush Niri request: {error}"))?;
        }

        let mut line = String::new();
        let bytes = self
            .reader
            .read_line(&mut line)
            .map_err(|error| format!("could not read Niri reply: {error}"))?;
        if bytes == 0 {
            return Err(String::from("Niri closed its IPC socket"));
        }
        let reply: Value = serde_json::from_str(&line)
            .map_err(|error| format!("Niri returned invalid JSON: {error}"))?;
        if let Some(error) = reply.get("Err") {
            return Err(format!("Niri rejected the request: {error}"));
        }
        reply
            .get("Ok")
            .cloned()
            .ok_or_else(|| String::from("Niri reply contained neither Ok nor Err"))
    }

    pub fn version(&mut self) -> Result<String, String> {
        let response = self.request(&json!("Version"))?;
        response
            .get("Version")
            .and_then(Value::as_str)
            .map(str::to_owned)
            .ok_or_else(|| format!("unexpected Niri version response: {response}"))
    }

    pub fn windows(&mut self) -> Result<Vec<WindowInfo>, String> {
        let response = self.request(&json!("Windows"))?;
        let windows = response
            .get("Windows")
            .cloned()
            .ok_or_else(|| format!("unexpected Niri windows response: {response}"))?;
        serde_json::from_value(windows)
            .map_err(|error| format!("could not decode Niri windows: {error}"))
    }

    fn output_height_for_workspace(&mut self, workspace_id: u64) -> Result<f64, String> {
        let response = self.request(&json!("Workspaces"))?;
        let workspaces = response
            .get("Workspaces")
            .cloned()
            .ok_or_else(|| format!("unexpected Niri workspaces response: {response}"))?;
        let workspaces: Vec<WorkspaceOutputInfo> = serde_json::from_value(workspaces)
            .map_err(|error| format!("could not decode Niri workspaces: {error}"))?;
        let output = workspaces
            .into_iter()
            .find(|workspace| workspace.id == workspace_id)
            .and_then(|workspace| workspace.output)
            .ok_or_else(|| format!("workspace {workspace_id} has no output"))?;

        let response = self.request(&json!("Outputs"))?;
        let outputs = response
            .get("Outputs")
            .cloned()
            .ok_or_else(|| format!("unexpected Niri outputs response: {response}"))?;
        let outputs: BTreeMap<String, HeightOutputInfo> = serde_json::from_value(outputs)
            .map_err(|error| format!("could not decode Niri outputs: {error}"))?;
        let height = outputs
            .get(&output)
            .map(|info| info.logical.height)
            .filter(|height| *height > 0)
            .ok_or_else(|| format!("output {output} has no usable logical height"))?;
        Ok(f64::from(height))
    }

    pub fn action(&mut self, action: &PlannedAction) -> Result<(), String> {
        let response = self.request(&action.request())?;
        if response == json!("Handled") {
            Ok(())
        } else {
            Err(format!("unexpected Niri action response: {response}"))
        }
    }

    pub fn into_event_stream(mut self, wake_interval: Duration) -> Result<NiriEventStream, String> {
        let response = self.request(&json!("EventStream"))?;
        if response != json!("Handled") {
            return Err(format!("unexpected Niri event-stream response: {response}"));
        }
        self.reader
            .get_mut()
            .set_read_timeout(Some(wake_interval))
            .map_err(|error| format!("could not configure Niri event stream: {error}"))?;
        Ok(NiriEventStream {
            reader: self.reader,
        })
    }
}

impl NiriEventStream {
    pub fn next_event(&mut self) -> Result<Option<Value>, String> {
        let mut line = String::new();
        match self.reader.read_line(&mut line) {
            Ok(0) => Err(String::from("Niri closed its event stream")),
            Ok(_) => serde_json::from_str(&line)
                .map(Some)
                .map_err(|error| format!("Niri returned an invalid event: {error}")),
            Err(error)
                if matches!(
                    error.kind(),
                    io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut
                ) =>
            {
                Ok(None)
            }
            Err(error) => Err(format!("could not read Niri event: {error}")),
        }
    }
}

pub fn execute_move(message: &MoveMessage) -> Result<usize, String> {
    message.validate()?;
    let mut niri = NiriConnection::connect(&message.niri_socket)
        .map_err(|error| format!("could not connect to Niri: {error}"))?;
    execute_move_with_connection(&mut niri, message)
}

pub fn execute_move_with_connection(
    niri: &mut NiriConnection,
    message: &MoveMessage,
) -> Result<usize, String> {
    if niri.path() != Path::new(&message.niri_socket) {
        return Err(String::from(
            "the cached Niri socket no longer matches the command",
        ));
    }
    let windows = niri.windows()?;
    let actions = plan_move(&windows, message.direction, message.target_height);
    for action in &actions {
        niri.action(action)?;
    }
    Ok(actions.len())
}

pub fn execute_height_step(niri_socket: &str, step: HeightStep) -> Result<usize, String> {
    let mut niri = NiriConnection::connect(niri_socket)
        .map_err(|error| format!("could not connect to Niri: {error}"))?;
    let windows = niri.windows()?;
    let Some(workspace_id) = windows
        .iter()
        .find(|window| window.is_focused && !window.is_floating)
        .and_then(|window| window.workspace_id)
    else {
        return Ok(0);
    };
    let output_height = niri.output_height_for_workspace(workspace_id)?;
    let actions = plan_height_step(&windows, output_height, step);
    for action in &actions {
        niri.action(action)?;
    }
    Ok(actions.len())
}

fn height_intent_now_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .try_into()
        .unwrap_or(u64::MAX)
}

pub fn execute_height_cycle(
    niri_socket: &str,
    previous: Option<HeightIntentState>,
) -> Result<HeightIntentOutcome, String> {
    let mut niri = NiriConnection::connect(niri_socket)
        .map_err(|error| format!("could not connect to Niri: {error}"))?;
    let windows = niri.windows()?;
    let Some(workspace_id) = windows
        .iter()
        .find(|window| window.is_focused && !window.is_floating)
        .and_then(|window| window.workspace_id)
    else {
        return Ok(HeightIntentOutcome {
            actions: 0,
            state: None,
        });
    };
    let output_height = niri.output_height_for_workspace(workspace_id)?;
    let (actions, state) = plan_height_cycle(
        &windows,
        output_height,
        previous,
        height_intent_now_millis(),
    );
    for action in &actions {
        niri.action(action)?;
    }
    Ok(HeightIntentOutcome {
        actions: actions.len(),
        state,
    })
}

pub fn execute_height_flow(
    niri_socket: &str,
    flow: HeightFlow,
    previous: Option<HeightIntentState>,
) -> Result<HeightIntentOutcome, String> {
    let mut niri = NiriConnection::connect(niri_socket)
        .map_err(|error| format!("could not connect to Niri: {error}"))?;
    let windows = niri.windows()?;
    let Some(workspace_id) = windows
        .iter()
        .find(|window| window.is_focused && !window.is_floating)
        .and_then(|window| window.workspace_id)
    else {
        return Ok(HeightIntentOutcome {
            actions: 0,
            state: None,
        });
    };
    let output_height = niri.output_height_for_workspace(workspace_id)?;
    let (actions, state) = plan_height_flow(
        &windows,
        output_height,
        flow,
        previous,
        height_intent_now_millis(),
    );
    for action in &actions {
        niri.action(action)?;
    }
    Ok(HeightIntentOutcome {
        actions: actions.len(),
        state,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn window(id: u64, column: usize, row: usize, focused: bool) -> WindowInfo {
        WindowInfo {
            id,
            title: None,
            app_id: None,
            pid: None,
            workspace_id: Some(7),
            is_focused: focused,
            is_floating: false,
            layout: WindowLayout {
                pos_in_scrolling_layout: Some((column, row)),
                window_size: Some((800, 600)),
                tile_size: None,
                tile_pos_in_workspace_view: None,
                window_offset_in_tile: None,
            },
        }
    }

    fn window_at_height(
        id: u64,
        column: usize,
        row: usize,
        focused: bool,
        height: f64,
    ) -> WindowInfo {
        let mut value = window(id, column, row, focused);
        value.layout.tile_size = Some((800.0, height));
        value
    }

    #[test]
    fn parses_heights() {
        assert_eq!(
            TargetHeight::parse("33.33%").unwrap(),
            TargetHeight::Proportion(33.33)
        );
        assert_eq!(
            TargetHeight::parse("720").unwrap(),
            TargetHeight::Fixed(720)
        );
        assert!(TargetHeight::parse("101%").is_err());
        assert!(TargetHeight::parse("-1").is_err());
    }

    #[test]
    fn wraps_vertical_stack_without_a_sleep() {
        let windows = vec![
            window(1, 1, 1, true),
            window(2, 1, 2, false),
            window(3, 1, 3, false),
        ];
        assert_eq!(
            plan_move(&windows, Direction::Up, None),
            vec![PlannedAction::MoveWindowDown, PlannedAction::MoveWindowDown]
        );
    }

    #[test]
    fn swaps_two_solo_columns_and_sets_requested_height() {
        let windows = vec![window(1, 1, 1, true), window(2, 2, 1, false)];
        assert_eq!(
            plan_move(
                &windows,
                Direction::Left,
                Some(TargetHeight::Proportion(33.33))
            ),
            vec![
                PlannedAction::MoveColumnToLast,
                PlannedAction::SetWindowHeight {
                    id: 1,
                    height: TargetHeight::Proportion(33.33)
                }
            ]
        );
    }

    #[test]
    fn ordinary_horizontal_move_preserves_current_height() {
        let windows = vec![window(1, 1, 1, true), window(2, 2, 1, false)];
        assert_eq!(
            plan_move(&windows, Direction::Right, None),
            vec![
                PlannedAction::MoveColumnRight,
                PlannedAction::SetWindowHeight {
                    id: 1,
                    height: TargetHeight::Fixed(600)
                }
            ]
        );
    }

    #[test]
    fn corner_moves_join_solo_columns_instead_of_swapping_them() {
        let windows = vec![window(1, 1, 1, false), window(2, 2, 1, true)];
        assert_eq!(
            plan_move(&windows, Direction::LeftTop, None),
            vec![
                PlannedAction::ConsumeOrExpelLeft { id: 2 },
                PlannedAction::MoveWindowUp,
                PlannedAction::SetWindowHeight {
                    id: 2,
                    height: TargetHeight::Fixed(600)
                }
            ]
        );
        assert_eq!(
            plan_move(&windows, Direction::LeftBottom, None),
            vec![
                PlannedAction::ConsumeOrExpelLeft { id: 2 },
                PlannedAction::SetWindowHeight {
                    id: 2,
                    height: TargetHeight::Fixed(600)
                }
            ]
        );
    }

    #[test]
    fn right_corner_moves_select_the_requested_stack_edge() {
        let windows = vec![
            window(1, 1, 1, true),
            window(2, 2, 1, false),
            window(3, 2, 2, false),
        ];
        assert_eq!(
            plan_move(&windows, Direction::RightTop, None),
            vec![
                PlannedAction::ConsumeOrExpelRight { id: 1 },
                PlannedAction::MoveWindowUp,
                PlannedAction::MoveWindowUp,
                PlannedAction::SetWindowHeight {
                    id: 1,
                    height: TargetHeight::Fixed(600)
                }
            ]
        );
        assert_eq!(
            plan_move(&windows, Direction::RightBottom, None),
            vec![
                PlannedAction::ConsumeOrExpelRight { id: 1 },
                PlannedAction::SetWindowHeight {
                    id: 1,
                    height: TargetHeight::Fixed(600)
                }
            ]
        );
    }

    #[test]
    fn snaps_to_bottom_and_sets_height() {
        let windows = vec![
            window(1, 1, 1, true),
            window(2, 1, 2, false),
            window(3, 1, 3, false),
        ];
        assert_eq!(
            plan_move(
                &windows,
                Direction::Bottom,
                Some(TargetHeight::Proportion(96.0))
            ),
            vec![
                PlannedAction::MoveWindowDown,
                PlannedAction::MoveWindowDown,
                PlannedAction::SetWindowHeight {
                    id: 1,
                    height: TargetHeight::Proportion(96.0)
                }
            ]
        );
    }

    #[test]
    fn ignores_floating_window() {
        let mut focused = window(1, 1, 1, true);
        focused.is_floating = true;
        assert!(plan_move(&[focused], Direction::Right, None).is_empty());
    }

    #[test]
    fn serializes_niri_26_04_action_shape() {
        let request = PlannedAction::SetWindowHeight {
            id: 42,
            height: TargetHeight::Proportion(50.0),
        }
        .request();
        assert_eq!(
            request,
            json!({
                "Action": {
                    "SetWindowHeight": {
                        "id": 42,
                        "change": { "SetProportion": 50.0 }
                    }
                }
            })
        );
    }

    #[test]
    fn quarter_height_steps_clamp_and_snap() {
        assert_eq!(next_quarter_height(25.0, HeightStep::Grow), Some(50.0));
        assert_eq!(next_quarter_height(50.0, HeightStep::Grow), Some(75.0));
        assert_eq!(next_quarter_height(75.0, HeightStep::Grow), Some(100.0));
        assert_eq!(next_quarter_height(100.0, HeightStep::Grow), None);

        assert_eq!(next_quarter_height(100.0, HeightStep::Shrink), Some(75.0));
        assert_eq!(next_quarter_height(75.0, HeightStep::Shrink), Some(50.0));
        assert_eq!(next_quarter_height(50.0, HeightStep::Shrink), Some(25.0));
        assert_eq!(next_quarter_height(25.0, HeightStep::Shrink), None);

        assert_eq!(next_quarter_height(63.0, HeightStep::Grow), Some(75.0));
        assert_eq!(next_quarter_height(63.0, HeightStep::Shrink), Some(50.0));
    }

    #[test]
    fn height_step_keeps_the_window_at_the_top() {
        let windows = vec![
            window_at_height(1, 1, 1, false, 864.0),
            window_at_height(2, 1, 2, true, 864.0),
        ];
        assert_eq!(
            plan_height_step(&windows, 1728.0, HeightStep::Grow),
            vec![
                PlannedAction::MoveWindowUp,
                PlannedAction::SetWindowHeight {
                    id: 2,
                    height: TargetHeight::Proportion(75.0),
                },
            ]
        );
    }

    #[test]
    fn height_step_does_nothing_beyond_an_endpoint() {
        let at_top = vec![window_at_height(1, 1, 1, true, 1728.0)];
        let at_bottom = vec![window_at_height(1, 1, 1, true, 432.0)];
        assert!(plan_height_step(&at_top, 1728.0, HeightStep::Grow).is_empty());
        assert!(plan_height_step(&at_bottom, 1728.0, HeightStep::Shrink).is_empty());
    }

    #[test]
    fn height_cycle_toggles_between_25_and_75() {
        let output_height = 1728.0;
        let expected = [75.0, 25.0, 75.0, 25.0];
        let mut current = 25.0;
        let mut state = None;
        let mut now_millis = 1_000;
        for target in expected {
            let windows = vec![window_at_height(
                1,
                1,
                1,
                true,
                output_height * current / 100.0,
            )];
            let (actions, next_state) =
                plan_height_cycle(&windows, output_height, state, now_millis);
            assert_eq!(
                actions,
                vec![PlannedAction::SetWindowHeight {
                    id: 1,
                    height: TargetHeight::Proportion(target),
                }]
            );
            state = next_state;
            current = target;
            now_millis += 100;
        }
    }

    #[test]
    fn height_cycle_defaults_to_focus_and_bookmarks_only_from_focus() {
        let unrelated_height = vec![window_at_height(1, 1, 1, true, 1728.0)];
        let focused_height = vec![window_at_height(1, 1, 1, true, 1296.0)];

        let (focus_actions, _) = plan_height_cycle(&unrelated_height, 1728.0, None, 1_000);
        let (bookmark_actions, _) = plan_height_cycle(&focused_height, 1728.0, None, 1_000);

        assert_eq!(
            focus_actions,
            vec![PlannedAction::SetWindowHeight {
                id: 1,
                height: TargetHeight::Proportion(75.0),
            }]
        );
        assert_eq!(
            bookmark_actions,
            vec![PlannedAction::SetWindowHeight {
                id: 1,
                height: TargetHeight::Proportion(25.0),
            }]
        );
    }

    #[test]
    fn height_flows_advance_relative_to_the_actual_height() {
        assert_eq!(next_height_flow_target(10.0), Some(25.0));
        assert_eq!(next_height_flow_target(25.0), Some(50.0));
        assert_eq!(next_height_flow_target(50.0), Some(75.0));
        assert_eq!(next_height_flow_target(63.0), Some(75.0));
        assert_eq!(next_height_flow_target(75.0), Some(100.0));
        assert_eq!(next_height_flow_target(100.0), Some(25.0));
        assert_eq!(next_height_flow_target(f64::NAN), None);
    }

    #[test]
    fn downward_height_flow_anchors_at_the_top() {
        let windows = vec![
            window_at_height(1, 1, 1, false, 864.0),
            window_at_height(2, 1, 2, true, 864.0),
        ];
        let (actions, _) = plan_height_flow(&windows, 1728.0, HeightFlow::Down, None, 1_000);
        assert_eq!(
            actions,
            vec![
                PlannedAction::MoveWindowUp,
                PlannedAction::SetWindowHeight {
                    id: 2,
                    height: TargetHeight::Proportion(75.0),
                },
            ]
        );
    }

    #[test]
    fn upward_height_flow_anchors_at_the_bottom() {
        let windows = vec![
            window_at_height(1, 1, 1, true, 864.0),
            window_at_height(2, 1, 2, false, 864.0),
        ];
        let (actions, _) = plan_height_flow(&windows, 1728.0, HeightFlow::Up, None, 1_000);
        assert_eq!(
            actions,
            vec![
                PlannedAction::MoveWindowDown,
                PlannedAction::SetWindowHeight {
                    id: 1,
                    height: TargetHeight::Proportion(75.0),
                },
            ]
        );
    }

    #[test]
    fn height_flow_wraps_from_100_back_to_the_first_25_frame() {
        let windows = vec![window_at_height(1, 1, 1, true, 1728.0)];
        let (actions, _) = plan_height_flow(&windows, 1728.0, HeightFlow::Down, None, 1_000);
        assert_eq!(
            actions,
            vec![PlannedAction::SetWindowHeight {
                id: 1,
                height: TargetHeight::Proportion(25.0),
            }]
        );
    }

    #[test]
    fn rapid_flow_press_advances_past_an_in_flight_height() {
        let windows = vec![window_at_height(1, 1, 1, true, 604.8)];
        let in_flight = HeightIntentState {
            window_id: 1,
            target_percentage: 50.0,
            updated_at_millis: 1_000,
        };

        let (actions, state) =
            plan_height_flow(&windows, 1728.0, HeightFlow::Up, Some(in_flight), 1_200);

        assert_eq!(
            actions,
            vec![PlannedAction::SetWindowHeight {
                id: 1,
                height: TargetHeight::Proportion(75.0),
            }]
        );
        assert_eq!(state.unwrap().target_percentage, 75.0);
    }

    #[test]
    fn rapid_focus_press_uses_the_shared_in_flight_target() {
        let windows = vec![window_at_height(1, 1, 1, true, 864.0)];
        let in_flight = HeightIntentState {
            window_id: 1,
            target_percentage: 75.0,
            updated_at_millis: 1_000,
        };

        let (actions, state) = plan_height_cycle(&windows, 1728.0, Some(in_flight), 1_200);

        assert_eq!(
            actions,
            vec![PlannedAction::SetWindowHeight {
                id: 1,
                height: TargetHeight::Proportion(25.0),
            }]
        );
        assert_eq!(state.unwrap().target_percentage, 25.0);
    }

    #[test]
    fn stale_height_intent_yields_to_external_geometry() {
        let windows = vec![window_at_height(1, 1, 1, true, 1088.64)];
        let stale = HeightIntentState {
            window_id: 1,
            target_percentage: 25.0,
            updated_at_millis: 1_000,
        };

        let (actions, _) = plan_height_flow(&windows, 1728.0, HeightFlow::Down, Some(stale), 3_000);

        assert_eq!(
            actions,
            vec![PlannedAction::SetWindowHeight {
                id: 1,
                height: TargetHeight::Proportion(75.0),
            }]
        );
    }

    #[test]
    fn height_intent_never_crosses_to_another_window() {
        let windows = vec![window_at_height(2, 1, 1, true, 864.0)];
        let other_window = HeightIntentState {
            window_id: 1,
            target_percentage: 25.0,
            updated_at_millis: 1_000,
        };

        let (actions, _) =
            plan_height_flow(&windows, 1728.0, HeightFlow::Up, Some(other_window), 1_100);

        assert_eq!(
            actions,
            vec![PlannedAction::SetWindowHeight {
                id: 2,
                height: TargetHeight::Proportion(75.0),
            }]
        );
    }

    #[test]
    fn future_height_intent_yields_to_current_geometry() {
        let windows = vec![window_at_height(1, 1, 1, true, 1088.64)];
        let future = HeightIntentState {
            window_id: 1,
            target_percentage: 25.0,
            updated_at_millis: 2_000,
        };

        let (actions, _) =
            plan_height_flow(&windows, 1728.0, HeightFlow::Down, Some(future), 1_000);

        assert_eq!(
            actions,
            vec![PlannedAction::SetWindowHeight {
                id: 1,
                height: TargetHeight::Proportion(75.0),
            }]
        );
    }
}
