use std::collections::VecDeque;
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io;
use std::os::fd::AsRawFd;
use std::os::unix::fs::{FileTypeExt, OpenOptionsExt, PermissionsExt};
use std::os::unix::net::UnixDatagram;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::{Instant, UNIX_EPOCH};

use hollow_niri::{
    Direction, HeightFlow, HeightIntentOutcome, HeightIntentState, HeightStep, MoveMessage,
    NiriConnection, TargetHeight, execute_height_cycle, execute_height_flow, execute_height_step,
    execute_move, execute_move_with_connection,
};
use serde::Serialize;

mod asset_watch;
mod broker;

const MAX_PENDING_COMMANDS: usize = 8;
const LOCK_EXCLUSIVE: i32 = 2;

fn usage() -> &'static str {
    "Usage:\n  hollow-niri move {left|right|left-top|left-bottom|right-top|right-bottom|up|down|top|bottom} [height]\n  hollow-niri height-step {grow|shrink}\n  hollow-niri height-cycle\n  hollow-niri height-flow {down|up}\n  hollow-niri daemon\n  hollow-niri broker [--root PATH] [--preview-fps N]\n  hollow-niri asset-watch [--root PATH]\n  hollow-niri ping\n  hollow-niri doctor [--json]\n  hollow-niri benchmark [iterations] [--json]\n  hollow-niri renderer-report [--json]"
}

fn runtime_root() -> Result<PathBuf, String> {
    let root = env::var_os("XDG_RUNTIME_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(format!("/run/user/{}", unsafe { libc_getuid() })));
    if !root.is_dir() {
        return Err(format!(
            "runtime directory is unavailable: {}",
            root.display()
        ));
    }
    Ok(root.join("hollow-niri"))
}

// SAFETY: getuid has no preconditions and does not modify memory.
unsafe fn libc_getuid() -> u32 {
    unsafe extern "C" {
        fn getuid() -> u32;
    }
    unsafe { getuid() }
}

fn daemon_socket_path() -> Result<PathBuf, String> {
    Ok(runtime_root()?.join("hollow-niri.sock"))
}

struct HeightIntentLock {
    _file: File,
}

fn acquire_height_intent_lock(path: &Path) -> Result<HeightIntentLock, String> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_file() => {
            return Err(format!(
                "refusing unsafe height-intent lock: {}",
                path.display()
            ));
        }
        Ok(_) => {}
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => {
            return Err(format!(
                "could not inspect height-intent lock {}: {error}",
                path.display()
            ));
        }
    }
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .mode(0o600)
        .open(path)
        .map_err(|error| {
            format!(
                "could not open height-intent lock {}: {error}",
                path.display()
            )
        })?;
    fs::set_permissions(path, fs::Permissions::from_mode(0o600)).map_err(|error| {
        format!(
            "could not secure height-intent lock {}: {error}",
            path.display()
        )
    })?;
    if unsafe { libc_flock(file.as_raw_fd(), LOCK_EXCLUSIVE) } != 0 {
        return Err(format!(
            "could not lock height intent {}: {}",
            path.display(),
            io::Error::last_os_error()
        ));
    }
    Ok(HeightIntentLock { _file: file })
}

fn load_height_intent(path: &Path) -> Result<Option<HeightIntentState>, String> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_file() => {
            return Err(format!(
                "refusing unsafe height-intent state: {}",
                path.display()
            ));
        }
        Ok(_) => {}
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(format!(
                "could not inspect height-intent state {}: {error}",
                path.display()
            ));
        }
    }
    let bytes = fs::read(path).map_err(|error| {
        format!(
            "could not read height-intent state {}: {error}",
            path.display()
        )
    })?;
    Ok(serde_json::from_slice(&bytes).ok())
}

fn store_height_intent(path: &Path, state: &HeightIntentState) -> Result<(), String> {
    let bytes = serde_json::to_vec(state)
        .map_err(|error| format!("could not encode height-intent state: {error}"))?;
    let temporary = path.with_extension(format!("tmp-{}", std::process::id()));
    fs::write(&temporary, bytes).map_err(|error| {
        format!(
            "could not write height-intent state {}: {error}",
            temporary.display()
        )
    })?;
    fs::set_permissions(&temporary, fs::Permissions::from_mode(0o600)).map_err(|error| {
        format!(
            "could not secure height-intent state {}: {error}",
            temporary.display()
        )
    })?;
    fs::rename(&temporary, path).map_err(|error| {
        format!(
            "could not commit height-intent state {}: {error}",
            path.display()
        )
    })
}

fn run_height_intent(
    operation: impl FnOnce(Option<HeightIntentState>) -> Result<HeightIntentOutcome, String>,
) -> Result<(), String> {
    let root = runtime_root()?;
    ensure_runtime_root(&root)?;
    let _lock = acquire_height_intent_lock(&root.join("height-intent.lock"))?;
    let state_path = root.join("height-intent.json");
    let previous = load_height_intent(&state_path)?;
    let outcome = operation(previous)?;
    if let Some(state) = outcome.state {
        store_height_intent(&state_path, &state)?;
    }
    Ok(())
}

// SAFETY: flock only reads the supplied file descriptor and operation.
unsafe fn libc_flock(file_descriptor: i32, operation: i32) -> i32 {
    unsafe extern "C" {
        fn flock(file_descriptor: i32, operation: i32) -> i32;
    }
    unsafe { flock(file_descriptor, operation) }
}

fn parse_move(args: &[String]) -> Result<MoveMessage, String> {
    let [direction, remaining @ ..] = args else {
        return Err(String::from("move requires a direction"));
    };
    let direction = Direction::parse(direction)?;
    let target_height = match remaining {
        [] if matches!(direction, Direction::Top | Direction::Bottom) => {
            Some(TargetHeight::default())
        }
        [] => None,
        [height] if direction.accepts_height() => Some(TargetHeight::parse(height)?),
        [_] => return Err(String::from("up and down do not accept a height")),
        _ => return Err(String::from("move accepts at most one height")),
    };
    let niri_socket = env::var("NIRI_SOCKET")
        .map_err(|_| String::from("NIRI_SOCKET is not set; run this command inside Niri"))?;
    Ok(MoveMessage::new(niri_socket, direction, target_height))
}

fn ensure_runtime_root(path: &Path) -> Result<(), String> {
    if path.exists() {
        let metadata = fs::symlink_metadata(path)
            .map_err(|error| format!("could not inspect {}: {error}", path.display()))?;
        if metadata.file_type().is_symlink() || !metadata.is_dir() {
            return Err(format!("refusing unsafe runtime path: {}", path.display()));
        }
    } else {
        fs::create_dir(path)
            .map_err(|error| format!("could not create {}: {error}", path.display()))?;
    }
    fs::set_permissions(path, fs::Permissions::from_mode(0o700))
        .map_err(|error| format!("could not secure {}: {error}", path.display()))
}

fn probe_daemon(path: &Path) -> Result<(), String> {
    let socket = UnixDatagram::unbound()
        .map_err(|error| format!("could not open health-probe socket: {error}"))?;
    socket
        .send_to(b"{\"ping\":true}", path)
        .map(|_| ())
        .map_err(|error| format!("could not reach {}: {error}", path.display()))
}

fn send_to_daemon(message: &MoveMessage) -> Result<(), String> {
    let path = daemon_socket_path()?;
    let payload = serde_json::to_vec(message)
        .map_err(|error| format!("could not encode movement command: {error}"))?;
    let socket = UnixDatagram::unbound()
        .map_err(|error| format!("could not open daemon client socket: {error}"))?;
    socket
        .send_to(&payload, &path)
        .map_err(|error| format!("could not send to {}: {error}", path.display()))?;
    Ok(())
}

#[derive(Default)]
struct QueueState {
    pending: VecDeque<MoveMessage>,
    coalesced: u64,
}

type WorkQueue = Arc<(Mutex<QueueState>, Condvar)>;

fn enqueue(queue: &WorkQueue, message: MoveMessage) {
    let (lock, ready) = &**queue;
    let mut state = lock.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    if state.pending.len() >= MAX_PENDING_COMMANDS {
        if state.pending.back() == Some(&message) {
            state.coalesced += 1;
            return;
        }
        state.pending.pop_front();
        state.coalesced += 1;
    }
    state.pending.push_back(message);
    ready.notify_one();
}

fn worker(queue: WorkQueue) {
    let mut connection: Option<NiriConnection> = None;
    loop {
        let message = {
            let (lock, ready) = &*queue;
            let mut state = lock.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
            while state.pending.is_empty() {
                state = ready
                    .wait(state)
                    .unwrap_or_else(|poisoned| poisoned.into_inner());
            }
            state
                .pending
                .pop_front()
                .expect("queue was checked as non-empty")
        };

        let needs_connection = connection
            .as_ref()
            .is_none_or(|niri| niri.path() != Path::new(&message.niri_socket));
        if needs_connection {
            connection = match NiriConnection::connect(&message.niri_socket) {
                Ok(value) => Some(value),
                Err(error) => {
                    eprintln!("hollow-niri: could not connect to Niri: {error}");
                    None
                }
            };
        }
        let Some(niri) = connection.as_mut() else {
            continue;
        };
        if let Err(error) = execute_move_with_connection(niri, &message) {
            eprintln!("hollow-niri: movement failed: {error}");
            connection = None;
        }
    }
}

fn run_daemon() -> Result<(), String> {
    let root = runtime_root()?;
    ensure_runtime_root(&root)?;
    let path = root.join("hollow-niri.sock");

    if path.exists() {
        let metadata = fs::symlink_metadata(&path)
            .map_err(|error| format!("could not inspect {}: {error}", path.display()))?;
        if !metadata.file_type().is_socket() {
            return Err(format!("refusing non-socket path: {}", path.display()));
        }
        if probe_daemon(&path).is_ok() {
            return Err(String::from(
                "another Hollow Niri daemon is already running",
            ));
        }
        fs::remove_file(&path)
            .map_err(|error| format!("could not remove stale socket: {error}"))?;
    }

    let socket = UnixDatagram::bind(&path)
        .map_err(|error| format!("could not bind {}: {error}", path.display()))?;
    fs::set_permissions(&path, fs::Permissions::from_mode(0o600))
        .map_err(|error| format!("could not secure {}: {error}", path.display()))?;

    let queue = Arc::new((Mutex::new(QueueState::default()), Condvar::new()));
    let worker_queue = Arc::clone(&queue);
    thread::Builder::new()
        .name(String::from("hollow-niri-ipc"))
        .spawn(move || worker(worker_queue))
        .map_err(|error| format!("could not start IPC worker: {error}"))?;

    let mut buffer = [0_u8; 4096];
    loop {
        let bytes = socket
            .recv(&mut buffer)
            .map_err(|error| format!("daemon receive failed: {error}"))?;
        let Ok(message) = serde_json::from_slice::<MoveMessage>(&buffer[..bytes]) else {
            // Health probes intentionally use a tiny non-command payload.
            continue;
        };
        if let Err(error) = message.validate() {
            eprintln!("hollow-niri: rejected command: {error}");
            continue;
        }
        enqueue(&queue, message);
    }
}

#[derive(Serialize)]
struct DoctorReport {
    status: &'static str,
    daemon_reachable: bool,
    daemon_socket: String,
    daemon_error: Option<String>,
    niri_socket: Option<String>,
    niri_version: Option<String>,
    windows: Option<usize>,
    round_trip_micros: Option<u128>,
    error: Option<String>,
}

fn doctor(json_output: bool) -> Result<(), String> {
    let daemon_path = daemon_socket_path()?;
    let daemon_probe = probe_daemon(&daemon_path);
    let daemon_reachable = daemon_probe.is_ok();
    let niri_socket = env::var("NIRI_SOCKET").ok();
    let mut report = DoctorReport {
        status: "fail",
        daemon_reachable,
        daemon_socket: daemon_path.display().to_string(),
        daemon_error: daemon_probe.err(),
        niri_socket: niri_socket.clone(),
        niri_version: None,
        windows: None,
        round_trip_micros: None,
        error: None,
    };

    if let Some(path) = niri_socket {
        let started = Instant::now();
        match NiriConnection::connect(&path) {
            Ok(mut niri) => match (niri.version(), niri.windows()) {
                (Ok(version), Ok(windows)) => {
                    report.status = if daemon_reachable { "pass" } else { "degraded" };
                    report.niri_version = Some(version);
                    report.windows = Some(windows.len());
                    report.round_trip_micros = Some(started.elapsed().as_micros());
                }
                (Err(error), _) | (_, Err(error)) => report.error = Some(error),
            },
            Err(error) => report.error = Some(format!("could not connect to Niri: {error}")),
        }
    } else {
        report.error = Some(String::from("NIRI_SOCKET is not set"));
    }

    if json_output {
        println!(
            "{}",
            serde_json::to_string_pretty(&report)
                .map_err(|error| format!("could not encode doctor report: {error}"))?
        );
    } else {
        println!("Hollow Niri: {}", report.status);
        println!(
            "  daemon: {}",
            if daemon_reachable {
                "ready"
            } else {
                "unavailable"
            }
        );
        if let Some(error) = &report.daemon_error {
            println!("  daemon error: {error}");
        }
        if let Some(version) = &report.niri_version {
            println!("  niri: {version}");
        }
        if let Some(windows) = report.windows {
            println!("  windows: {windows}");
        }
        if let Some(micros) = report.round_trip_micros {
            println!("  version + windows round trip: {micros} µs");
        }
        if let Some(error) = &report.error {
            println!("  error: {error}");
        }
    }
    if report.status == "pass" {
        Ok(())
    } else {
        Err(report
            .error
            .unwrap_or_else(|| String::from("movement daemon is unavailable")))
    }
}

#[derive(Serialize)]
struct BenchmarkReport {
    iterations: usize,
    p50_micros: u128,
    p95_micros: u128,
    max_micros: u128,
}

fn benchmark(iterations: usize, json_output: bool) -> Result<(), String> {
    if iterations == 0 || iterations > 100_000 {
        return Err(String::from("iterations must be between 1 and 100000"));
    }
    let path = env::var("NIRI_SOCKET")
        .map_err(|_| String::from("NIRI_SOCKET is not set; benchmark inside Niri"))?;
    let mut niri = NiriConnection::connect(path)
        .map_err(|error| format!("could not connect to Niri: {error}"))?;
    let mut samples = Vec::with_capacity(iterations);
    for _ in 0..iterations {
        let started = Instant::now();
        niri.windows()?;
        samples.push(started.elapsed().as_micros());
    }
    samples.sort_unstable();
    let percentile = |numerator: usize| {
        let index = ((samples.len() - 1) * numerator) / 100;
        samples[index]
    };
    let report = BenchmarkReport {
        iterations,
        p50_micros: percentile(50),
        p95_micros: percentile(95),
        max_micros: *samples.last().expect("iterations is non-zero"),
    };
    if json_output {
        println!(
            "{}",
            serde_json::to_string_pretty(&report)
                .map_err(|error| format!("could not encode benchmark: {error}"))?
        );
    } else {
        println!("Persistent Niri IPC · {} window snapshots", iterations);
        println!("  p50: {} µs", report.p50_micros);
        println!("  p95: {} µs", report.p95_micros);
        println!("  max: {} µs", report.max_micros);
    }
    Ok(())
}

#[derive(Serialize)]
struct RendererReport {
    niri_version: String,
    renderer_errors: usize,
    gl_invalid_value: usize,
    config_path: String,
    renderer_errors_since_config_change: Option<usize>,
    gl_invalid_value_since_config_change: Option<usize>,
    recent_error_lines: Vec<String>,
    visible_windows: Vec<RendererWindow>,
}

#[derive(Serialize)]
struct RendererWindow {
    id: u64,
    app_id: Option<String>,
    title: Option<String>,
    pid: Option<i32>,
    workspace_id: Option<u64>,
    layout_position: Option<(usize, usize)>,
    window_size: Option<(i32, i32)>,
}

fn renderer_report(json_output: bool) -> Result<(), String> {
    let version = Command::new("niri")
        .arg("--version")
        .output()
        .map_err(|error| format!("could not run niri --version: {error}"))?;
    let niri_version = String::from_utf8_lossy(&version.stdout).trim().to_owned();
    let journal = Command::new("journalctl")
        .args([
            "--user",
            "-b",
            "_COMM=niri",
            "--no-pager",
            "-o",
            "short-iso",
        ])
        .output()
        .map_err(|error| format!("could not read the Niri journal: {error}"))?;
    if !journal.status.success() {
        return Err(String::from_utf8_lossy(&journal.stderr).trim().to_owned());
    }
    let text = String::from_utf8_lossy(&journal.stdout);
    let renderer_lines: Vec<&str> = text
        .lines()
        .filter(|line| line.contains("smithay::backend::renderer") || line.contains("[GL]"))
        .collect();
    let gl_invalid_value = renderer_lines
        .iter()
        .filter(|line| line.contains("GL_INVALID_VALUE"))
        .count();

    let config_path = env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")))
        .unwrap_or_else(|| PathBuf::from(".config"))
        .join("niri/config.kdl");
    let errors_since_config_change = fs::metadata(&config_path)
        .and_then(|metadata| metadata.modified())
        .and_then(|modified| {
            modified
                .duration_since(UNIX_EPOCH)
                .map_err(io::Error::other)
        })
        .ok()
        .and_then(|modified| {
            let since = format!("@{}", modified.as_secs());
            Command::new("journalctl")
                .args([
                    "--user",
                    "-b",
                    "_COMM=niri",
                    "--since",
                    &since,
                    "--no-pager",
                    "-o",
                    "cat",
                ])
                .output()
                .ok()
        })
        .filter(|output| output.status.success())
        .map(|output| {
            let text = String::from_utf8_lossy(&output.stdout);
            let renderer_errors = text
                .lines()
                .filter(|line| line.contains("smithay::backend::renderer") || line.contains("[GL]"))
                .count();
            let gl_invalid_value = text
                .lines()
                .filter(|line| line.contains("GL_INVALID_VALUE"))
                .count();
            (renderer_errors, gl_invalid_value)
        });
    let recent_error_lines = renderer_lines
        .iter()
        .rev()
        .take(12)
        .rev()
        .map(|line| (*line).to_owned())
        .collect();

    let visible_windows = env::var("NIRI_SOCKET")
        .ok()
        .and_then(|path| NiriConnection::connect(path).ok())
        .and_then(|mut niri| niri.windows().ok())
        .unwrap_or_default()
        .into_iter()
        .map(|window| RendererWindow {
            id: window.id,
            app_id: window.app_id,
            title: window.title,
            pid: window.pid,
            workspace_id: window.workspace_id,
            layout_position: window.layout.pos_in_scrolling_layout,
            window_size: window.layout.window_size,
        })
        .collect();

    let report = RendererReport {
        niri_version,
        renderer_errors: renderer_lines.len(),
        gl_invalid_value,
        config_path: config_path.display().to_string(),
        renderer_errors_since_config_change: errors_since_config_change.map(|value| value.0),
        gl_invalid_value_since_config_change: errors_since_config_change.map(|value| value.1),
        recent_error_lines,
        visible_windows,
    };
    if json_output {
        println!(
            "{}",
            serde_json::to_string_pretty(&report)
                .map_err(|error| format!("could not encode renderer report: {error}"))?
        );
    } else {
        println!("Niri renderer report");
        println!("  version: {}", report.niri_version);
        println!("  renderer errors this boot: {}", report.renderer_errors);
        println!("  GL_INVALID_VALUE: {}", report.gl_invalid_value);
        if let Some(errors) = report.renderer_errors_since_config_change {
            println!(
                "  renderer errors since {} changed: {}",
                report.config_path, errors
            );
        }
        println!(
            "  currently visible clients: {}",
            report.visible_windows.len()
        );
        for line in &report.recent_error_lines {
            println!("  {line}");
        }
    }
    Ok(())
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.as_slice() {
        [command] if command == "daemon" => run_daemon(),
        [command, rest @ ..] if command == "broker" => {
            broker::run(broker::BrokerConfig::parse(rest)?)
        }
        [command, rest @ ..] if command == "asset-watch" => asset_watch::run(rest),
        [command] if command == "ping" => {
            probe_daemon(&daemon_socket_path()?)?;
            println!("hollow-niri: ready");
            Ok(())
        }
        [command, rest @ ..] if command == "move" => {
            let message = parse_move(rest)?;
            if send_to_daemon(&message).is_ok() {
                Ok(())
            } else {
                execute_move(&message).map(|_| ())
            }
        }
        [command, direction] if command == "height-step" => {
            let step = HeightStep::parse(direction)?;
            let niri_socket = env::var("NIRI_SOCKET").map_err(|_| {
                String::from("NIRI_SOCKET is not set; run this command inside Niri")
            })?;
            execute_height_step(&niri_socket, step).map(|_| ())
        }
        [command] if command == "height-cycle" => {
            let niri_socket = env::var("NIRI_SOCKET").map_err(|_| {
                String::from("NIRI_SOCKET is not set; run this command inside Niri")
            })?;
            run_height_intent(|previous| execute_height_cycle(&niri_socket, previous))
        }
        [command, direction] if command == "height-flow" => {
            let flow = HeightFlow::parse(direction)?;
            let niri_socket = env::var("NIRI_SOCKET").map_err(|_| {
                String::from("NIRI_SOCKET is not set; run this command inside Niri")
            })?;
            run_height_intent(|previous| execute_height_flow(&niri_socket, flow, previous))
        }
        [command] if command == "doctor" => doctor(false),
        [command, flag] if command == "doctor" && flag == "--json" => doctor(true),
        [command] if command == "benchmark" => benchmark(100, false),
        [command, flag] if command == "benchmark" && flag == "--json" => benchmark(100, true),
        [command, count] if command == "benchmark" => benchmark(
            count
                .parse()
                .map_err(|_| format!("invalid iteration count: {count}"))?,
            false,
        ),
        [command, count, flag] if command == "benchmark" && flag == "--json" => benchmark(
            count
                .parse()
                .map_err(|_| format!("invalid iteration count: {count}"))?,
            true,
        ),
        [command] if command == "renderer-report" => renderer_report(false),
        [command, flag] if command == "renderer-report" && flag == "--json" => {
            renderer_report(true)
        }
        [command] if command == "help" || command == "--help" || command == "-h" => {
            println!("{}", usage());
            Ok(())
        }
        _ => Err(String::from(usage())),
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("hollow-niri: {error}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounded_queue_coalesces_overflowing_repeat() {
        let queue = Arc::new((Mutex::new(QueueState::default()), Condvar::new()));
        let message = MoveMessage::new(
            String::from("/run/user/1000/niri.sock"),
            Direction::Left,
            None,
        );
        for _ in 0..32 {
            enqueue(&queue, message.clone());
        }
        let state = queue.0.lock().unwrap();
        assert_eq!(state.pending.len(), MAX_PENDING_COMMANDS);
        assert_eq!(state.coalesced, 24);
    }
}
