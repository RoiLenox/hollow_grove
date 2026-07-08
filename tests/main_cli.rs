use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use hollow_grove::CANONICAL_WITNESS;

fn unique_temp_dir(prefix: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("{prefix}-{nonce}"))
}

fn create_executable_script(path: &Path, contents: &str) {
    fs::write(path, contents).expect("script should write");
    let mut perms = fs::metadata(path).expect("script metadata").permissions();
    #[cfg(unix)]
    {
        perms.set_mode(0o755);
    }
    fs::set_permissions(path, perms).expect("script permissions should update");
}

fn wait_for_path(path: &Path) -> bool {
    for _ in 0..20 {
        if path.exists() {
            return true;
        }
        thread::sleep(Duration::from_millis(100));
    }
    false
}

#[test]
fn main_binary_defaults_to_kernel_witness() {
    let output = Command::new(env!("CARGO_BIN_EXE_hollow-grove"))
        .output()
        .expect("main binary should run");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        CANONICAL_WITNESS
    );
}

#[test]
fn main_binary_reports_integrated_help() {
    let output = Command::new(env!("CARGO_BIN_EXE_hollow-grove"))
        .arg("--help")
        .output()
        .expect("help should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Usage: hollow-grove [command] [args]"));
    assert!(stdout.contains("runtime [args]"));
    assert!(stdout.contains("bridge [args]"));
    assert!(stdout.contains("desktop [args]"));
    assert!(stdout.contains("benchmark [args]"));
}

#[test]
fn main_binary_delegates_runtime_help() {
    let output = Command::new(env!("CARGO_BIN_EXE_hollow-grove"))
        .args(["runtime", "--help"])
        .output()
        .expect("runtime help should run");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        "Usage: hollow_grove_runtime [--cycles N] [--interval-ms N] [--quiet]"
    );
}

#[test]
fn main_binary_delegates_benchmark_help() {
    let output = Command::new(env!("CARGO_BIN_EXE_hollow-grove"))
        .args(["benchmark", "--help"])
        .output()
        .expect("benchmark help should run");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        "Usage: current_synthesis_benchmark [--warmup N] [--samples N] [--no-write] [--quiet]"
    );
}

#[test]
fn main_binary_runs_benchmark_and_writes_report() {
    let temp_root = unique_temp_dir("hollow-grove-main-benchmark");
    fs::create_dir_all(temp_root.join("artifacts")).expect("artifact root should create");

    let status = Command::new(env!("CARGO_BIN_EXE_hollow-grove"))
        .current_dir(&temp_root)
        .args(["benchmark", "--warmup", "1", "--samples", "2", "--quiet"])
        .status()
        .expect("benchmark should run");

    assert!(status.success());
    assert!(wait_for_path(
        &temp_root.join("artifacts/current_synthesis_benchmark.md")
    ));
    assert!(wait_for_path(
        &temp_root.join("artifacts/current_synthesis_benchmark.json")
    ));
    assert!(wait_for_path(
        &temp_root.join("artifacts/current_synthesis_benchmark_release.md")
    ));

    let report = fs::read_to_string(temp_root.join("artifacts/current_synthesis_benchmark.md"))
        .expect("benchmark report should exist");
    let release_summary =
        fs::read_to_string(temp_root.join("artifacts/current_synthesis_benchmark_release.md"))
            .expect("benchmark release summary should exist");
    assert!(report.contains("# Current Synthesis Benchmark"));
    assert!(report.contains("## Weak Points"));
    assert!(release_summary.contains("# Current Synthesis Benchmark Release Summary"));

    fs::remove_dir_all(&temp_root).expect("temp cleanup should succeed");
}

#[test]
fn main_binary_launches_desktop_runtime_and_bridge() {
    let temp_root = unique_temp_dir("hollow-grove-main-desktop");
    let temp_bin = temp_root.join("bin");
    let temp_artifacts = temp_root.join("artifacts");
    fs::create_dir_all(&temp_bin).expect("temp bin should create");
    fs::create_dir_all(&temp_artifacts).expect("temp artifacts should create");
    fs::write(
        temp_artifacts.join("index.md"),
        "# Artifact Index\n\nindex\n",
    )
    .expect("artifact index should write");

    let niri_log = temp_root.join("niri.log");
    create_executable_script(
        &temp_bin.join("niri"),
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\necho \"$@\" >> \"{}\"\nif [[ \"$1\" == \"msg\" && \"$2\" == \"-j\" && \"${{3-}}\" == \"workspaces\" ]]; then\n  printf '%s\\n' '[{{\"id\":1,\"idx\":1,\"name\":null,\"output\":\"DP-2\",\"is_urgent\":false,\"is_active\":true,\"is_focused\":true,\"active_window_id\":4}}]'\nelif [[ \"$1\" == \"msg\" && \"$2\" == \"overview-state\" ]]; then\n  printf '%s\\n' 'Overview is open.'\nelif [[ \"$1\" == \"msg\" && \"$2\" == \"action\" && \"${{3-}}\" == \"open-overview\" ]]; then\n  :\nelif [[ \"$1\" == \"msg\" && \"$2\" == \"action\" && \"${{3-}}\" == \"close-overview\" ]]; then\n  :\nelif [[ \"$1\" == \"msg\" && \"$2\" == \"action\" && \"${{3-}}\" == \"focus-workspace\"* ]]; then\n  :\nelif [[ \"$1\" == \"msg\" && \"$2\" == \"action\" && \"${{3-}}\" == \"set-workspace-name\"* ]]; then\n  :\nelse\n  printf '%s\\n' \"unexpected niri invocation: $*\" >&2\n  exit 1\nfi\n",
            niri_log.display()
        ),
    );

    let status = Command::new(env!("CARGO_BIN_EXE_hollow-grove"))
        .current_dir(&temp_root)
        .env(
            "PATH",
            format!(
                "{}:{}",
                temp_bin.display(),
                std::env::var("PATH").unwrap_or_default()
            ),
        )
        .args([
            "desktop",
            "--cycles",
            "2",
            "--interval-ms",
            "1500",
            "--quiet",
        ])
        .status()
        .expect("desktop launcher should run");

    assert!(status.success());
    assert!(wait_for_path(
        &temp_artifacts.join("runtime_loop_status.md")
    ));
    assert!(wait_for_path(&temp_artifacts.join("niri_bridge_status.md")));
    assert!(wait_for_path(&niri_log));

    let runtime_status = fs::read_to_string(temp_artifacts.join("runtime_loop_status.md"))
        .expect("runtime status should exist");
    let bridge_status = fs::read_to_string(temp_artifacts.join("niri_bridge_status.md"))
        .expect("bridge status should exist");
    let niri_log_contents = fs::read_to_string(&niri_log).expect("niri log should exist");

    assert!(runtime_status.contains("cycle: 2"));
    assert!(runtime_status.contains("refreshed pipeline"));
    assert!(bridge_status.contains("apply_enabled: true"));
    assert!(niri_log_contents.contains("msg -j workspaces"));
    assert!(niri_log_contents.contains("msg overview-state"));
    assert!(niri_log_contents.contains("msg action set-workspace-name HollowGrove"));

    fs::remove_dir_all(&temp_root).expect("temp cleanup should succeed");
}
