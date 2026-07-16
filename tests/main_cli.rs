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
    assert!(stdout.contains("hueman-slice"));
    assert!(stdout.contains("verify-foundation"));
    assert!(stdout.contains("world context"));
    assert!(stdout.contains("world witness"));
    assert!(stdout.contains("world validate"));
    assert!(stdout.contains("progression witness"));
    assert!(stdout.contains("progression validate"));
    assert!(stdout.contains("point-squared witness"));
    assert!(stdout.contains("map witness"));
    assert!(stdout.contains("map validate"));
    assert!(stdout.contains("rule-of-twelve witness"));
    assert!(stdout.contains("rule-of-twelve validate"));
    assert!(stdout.contains("manager-language witness"));
    assert!(stdout.contains("manager-language validate"));
    assert!(stdout.contains("player-location witness"));
    assert!(stdout.contains("being-object witness"));
    assert!(stdout.contains("being-object validate"));
    assert!(stdout.contains("move witness"));
    assert!(stdout.contains("civic-body witness"));
    assert!(stdout.contains("civic-body validate"));
    assert!(stdout.contains("civic-crisis witness"));
    assert!(stdout.contains("flow-glow witness"));
    assert!(stdout.contains("flow-glow validate"));
    assert!(stdout.contains("embodied-action witness"));
    assert!(stdout.contains("current-inheritance witness"));
    assert!(stdout.contains("current-inheritance validate"));
    assert!(stdout.contains("grip witness"));
    assert!(stdout.contains("engine status"));
    assert!(stdout.contains("player ..."));
    assert!(stdout.contains("player move"));
    assert!(stdout.contains("player decide"));
    assert!(stdout.contains("cleopatra tick"));
    assert!(stdout.contains("cleopatra run"));
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
fn main_binary_delegates_hueman_slice_help() {
    let output = Command::new(env!("CARGO_BIN_EXE_hollow-grove"))
        .args(["hueman-slice", "--help"])
        .output()
        .expect("hueman slice help should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(
        "Usage: hueman_slice_demo [scenario list|scenario use <slice-id>|status|next|next-start|next-complete|walk [route|defense]|reset|survey|gather|refine|name [tool name]|prove [route|defense]|clear|deploy [route|defense]|recognize|unlock]"
    ));
    assert!(stdout.contains("scenario list"));
    assert!(stdout.contains("status"));
    assert!(stdout.contains("next"));
    assert!(stdout.contains("next-start"));
    assert!(stdout.contains("next-complete"));
    assert!(stdout.contains("walk"));
    assert!(stdout.contains("reset"));
    assert!(stdout.contains("survey"));
    assert!(stdout.contains("name"));
    assert!(stdout.contains("route"));
    assert!(stdout.contains("defense"));
    assert!(stdout.contains("recognize"));
    assert!(stdout.contains("unlock"));
}

#[test]
fn main_binary_delegates_current_synthesis_tui_help() {
    let temp_root = unique_temp_dir("hollow-grove-main-engine-status");
    fs::create_dir_all(&temp_root).expect("temp root should create");
    let output = Command::new(env!("CARGO_BIN_EXE_hollow-grove"))
        .current_dir(&temp_root)
        .args(["engine", "status"])
        .output()
        .expect("engine status should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Current Synthesis Engine"));
    assert!(stdout.contains("PLEB"));
    assert!(stdout.contains("Cleopatra"));
    fs::remove_dir_all(&temp_root).expect("temp cleanup should succeed");
}

#[test]
fn main_binary_delegates_world_context() {
    let output = Command::new(env!("CARGO_BIN_EXE_hollow-grove"))
        .args(["world", "context"])
        .output()
        .expect("world context should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Current = blood"));
    assert!(stdout.contains("Hollow = pus"));
    assert!(stdout.contains("Frame = living mech only"));
}

#[test]
fn main_binary_delegates_world_witness() {
    let output = Command::new(env!("CARGO_BIN_EXE_hollow-grove"))
        .args(["world", "witness"])
        .output()
        .expect("world witness should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("HOLLOW GROVE ALIGNMENT WITNESS"));
    assert!(stdout.contains("Diamond claims"));
}

#[test]
fn main_binary_delegates_world_validate() {
    let output = Command::new(env!("CARGO_BIN_EXE_hollow-grove"))
        .args(["world", "validate"])
        .output()
        .expect("world validate should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("status: pass"));
}

#[test]
fn main_binary_delegates_progression_and_point_squared_surfaces() {
    let progression_witness = Command::new(env!("CARGO_BIN_EXE_hollow-grove"))
        .args(["progression", "witness"])
        .output()
        .expect("progression witness should run");
    assert!(progression_witness.status.success());
    let progression_witness_stdout = String::from_utf8_lossy(&progression_witness.stdout);
    assert!(progression_witness_stdout.contains("HOLLOW GROVE PROGRESSION WITNESS"));

    let progression_validate = Command::new(env!("CARGO_BIN_EXE_hollow-grove"))
        .args(["progression", "validate"])
        .output()
        .expect("progression validate should run");
    assert!(progression_validate.status.success());
    let progression_validate_stdout = String::from_utf8_lossy(&progression_validate.stdout);
    assert!(progression_validate_stdout.contains("status: pass"));

    let point_squared = Command::new(env!("CARGO_BIN_EXE_hollow-grove"))
        .args(["point-squared", "witness"])
        .output()
        .expect("point-squared witness should run");
    assert!(point_squared.status.success());
    let point_squared_stdout = String::from_utf8_lossy(&point_squared.stdout);
    assert!(point_squared_stdout.contains("HOLLOW GROVE POINT² ASCENSION WITNESS"));
    assert!(point_squared_stdout.contains("Stairway to Heaven"));

    let map_witness = Command::new(env!("CARGO_BIN_EXE_hollow-grove"))
        .args(["map", "witness"])
        .output()
        .expect("map witness should run");
    assert!(map_witness.status.success());
    let map_witness_stdout = String::from_utf8_lossy(&map_witness.stdout);
    assert!(map_witness_stdout.contains("HOLLOW GROVE ROTATIONAL MAP WITNESS"));

    let map_validate = Command::new(env!("CARGO_BIN_EXE_hollow-grove"))
        .args(["map", "validate"])
        .output()
        .expect("map validate should run");
    assert!(map_validate.status.success());
    let map_validate_stdout = String::from_utf8_lossy(&map_validate.stdout);
    assert!(map_validate_stdout.contains("status: pass"));

    let rule_of_twelve_witness = Command::new(env!("CARGO_BIN_EXE_hollow-grove"))
        .args(["rule-of-twelve", "witness"])
        .output()
        .expect("rule-of-twelve witness should run");
    assert!(rule_of_twelve_witness.status.success());
    let rule_of_twelve_witness_stdout = String::from_utf8_lossy(&rule_of_twelve_witness.stdout);
    assert!(rule_of_twelve_witness_stdout.contains("HOLLOW GROVE RULE OF TWELVE"));

    let rule_of_twelve_validate = Command::new(env!("CARGO_BIN_EXE_hollow-grove"))
        .args(["rule-of-twelve", "validate"])
        .output()
        .expect("rule-of-twelve validate should run");
    assert!(rule_of_twelve_validate.status.success());
    let rule_of_twelve_validate_stdout = String::from_utf8_lossy(&rule_of_twelve_validate.stdout);
    assert!(rule_of_twelve_validate_stdout.contains("status: pass"));

    let manager_language_witness = Command::new(env!("CARGO_BIN_EXE_hollow-grove"))
        .args(["manager-language", "witness"])
        .output()
        .expect("manager-language witness should run");
    assert!(manager_language_witness.status.success());
    let manager_language_witness_stdout = String::from_utf8_lossy(&manager_language_witness.stdout);
    assert!(manager_language_witness_stdout.contains("HOLLOW GROVE MANAGER LANGUAGE"));

    let manager_language_validate = Command::new(env!("CARGO_BIN_EXE_hollow-grove"))
        .args(["manager-language", "validate"])
        .output()
        .expect("manager-language validate should run");
    assert!(manager_language_validate.status.success());
    let manager_language_validate_stdout =
        String::from_utf8_lossy(&manager_language_validate.stdout);
    assert!(manager_language_validate_stdout.contains("status: pass"));

    let player_location_witness = Command::new(env!("CARGO_BIN_EXE_hollow-grove"))
        .args(["player-location", "witness"])
        .output()
        .expect("player-location witness should run");
    assert!(player_location_witness.status.success());
    let player_location_witness_stdout = String::from_utf8_lossy(&player_location_witness.stdout);
    assert!(player_location_witness_stdout.contains("PLAYER SPATIAL INTERPRETATION"));

    let being_object_witness = Command::new(env!("CARGO_BIN_EXE_hollow-grove"))
        .args(["being-object", "witness"])
        .output()
        .expect("being-object witness should run");
    assert!(being_object_witness.status.success());
    let being_object_witness_stdout = String::from_utf8_lossy(&being_object_witness.stdout);
    assert!(being_object_witness_stdout.contains("HOLLOW GROVE BEING / OBJECT ONTOLOGY"));

    let being_object_validate = Command::new(env!("CARGO_BIN_EXE_hollow-grove"))
        .args(["being-object", "validate"])
        .output()
        .expect("being-object validate should run");
    assert!(being_object_validate.status.success());
    let being_object_validate_stdout = String::from_utf8_lossy(&being_object_validate.stdout);
    assert!(being_object_validate_stdout.contains("status: pass"));

    let move_witness = Command::new(env!("CARGO_BIN_EXE_hollow-grove"))
        .args(["move", "witness"])
        .output()
        .expect("move witness should run");
    assert!(move_witness.status.success());
    let move_witness_stdout = String::from_utf8_lossy(&move_witness.stdout);
    assert!(move_witness_stdout.contains("HOLLOW GROVE MOVE WITNESS"));

    let civic_body_witness = Command::new(env!("CARGO_BIN_EXE_hollow-grove"))
        .args(["civic-body", "witness"])
        .output()
        .expect("civic-body witness should run");
    assert!(civic_body_witness.status.success());
    let civic_body_witness_stdout = String::from_utf8_lossy(&civic_body_witness.stdout);
    assert!(civic_body_witness_stdout.contains("HOLLOW GROVE CIVIC BODY"));

    let civic_body_validate = Command::new(env!("CARGO_BIN_EXE_hollow-grove"))
        .args(["civic-body", "validate"])
        .output()
        .expect("civic-body validate should run");
    assert!(civic_body_validate.status.success());
    let civic_body_validate_stdout = String::from_utf8_lossy(&civic_body_validate.stdout);
    assert!(civic_body_validate_stdout.contains("status: pass"));

    let civic_crisis_witness = Command::new(env!("CARGO_BIN_EXE_hollow-grove"))
        .args(["civic-crisis", "witness"])
        .output()
        .expect("civic-crisis witness should run");
    assert!(civic_crisis_witness.status.success());
    let civic_crisis_witness_stdout = String::from_utf8_lossy(&civic_crisis_witness.stdout);
    assert!(civic_crisis_witness_stdout.contains("World Breach"));

    let flow_glow_witness = Command::new(env!("CARGO_BIN_EXE_hollow-grove"))
        .args(["flow-glow", "witness"])
        .output()
        .expect("flow-glow witness should run");
    assert!(flow_glow_witness.status.success());
    let flow_glow_witness_stdout = String::from_utf8_lossy(&flow_glow_witness.stdout);
    assert!(flow_glow_witness_stdout.contains("HOLLOW GROVE FLOW / GLOW GRAMMAR"));

    let flow_glow_validate = Command::new(env!("CARGO_BIN_EXE_hollow-grove"))
        .args(["flow-glow", "validate"])
        .output()
        .expect("flow-glow validate should run");
    assert!(flow_glow_validate.status.success());
    let flow_glow_validate_stdout = String::from_utf8_lossy(&flow_glow_validate.stdout);
    assert!(flow_glow_validate_stdout.contains("status: pass"));

    let embodied_action_witness = Command::new(env!("CARGO_BIN_EXE_hollow-grove"))
        .args(["embodied-action", "witness"])
        .output()
        .expect("embodied-action witness should run");
    assert!(embodied_action_witness.status.success());
    let embodied_action_witness_stdout = String::from_utf8_lossy(&embodied_action_witness.stdout);
    assert!(embodied_action_witness_stdout.contains("HOLLOW GROVE EMBODIED ACTION WITNESS"));

    let current_inheritance_witness = Command::new(env!("CARGO_BIN_EXE_hollow-grove"))
        .args(["current-inheritance", "witness"])
        .output()
        .expect("current-inheritance witness should run");
    assert!(current_inheritance_witness.status.success());
    let current_inheritance_witness_stdout =
        String::from_utf8_lossy(&current_inheritance_witness.stdout);
    assert!(current_inheritance_witness_stdout.contains("HOLLOW GROVE CURRENT INHERITANCE"));

    let current_inheritance_validate = Command::new(env!("CARGO_BIN_EXE_hollow-grove"))
        .args(["current-inheritance", "validate"])
        .output()
        .expect("current-inheritance validate should run");
    assert!(current_inheritance_validate.status.success());
    let current_inheritance_validate_stdout =
        String::from_utf8_lossy(&current_inheritance_validate.stdout);
    assert!(current_inheritance_validate_stdout.contains("status: pass"));

    let grip_witness = Command::new(env!("CARGO_BIN_EXE_hollow-grove"))
        .args(["grip", "witness"])
        .output()
        .expect("grip witness should run");
    assert!(grip_witness.status.success());
    let grip_witness_stdout = String::from_utf8_lossy(&grip_witness.stdout);
    assert!(grip_witness_stdout.contains("HOLLOW GROVE GRIP WITNESS"));
}

#[test]
fn main_binary_runs_foundation_verification() {
    let output = Command::new(env!("CARGO_BIN_EXE_hollow-grove"))
        .args(["verify-foundation"])
        .output()
        .expect("verify-foundation should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("HOLLOW GROVE FOUNDATION VERIFICATION"));
    assert!(stdout.contains("world witness: pass"));
    assert!(stdout.contains("Point² paired capacity advancement: pass"));
    assert!(stdout.contains("Point² exactly-once application: pass"));
    assert!(stdout.contains("Ranina center: pass"));
    assert!(stdout.contains("twelve-position rotation: pass"));
    assert!(stdout.contains("four-House grammar: pass"));
    assert!(stdout.contains("three-pass repetition: pass"));
    assert!(stdout.contains("Position 12 Flynt completion: pass"));
    assert!(stdout.contains("PLEB / Proxy mapping: pass"));
    assert!(stdout.contains("META / Moxy mapping: pass"));
    assert!(stdout.contains("BLEP / Foxy mapping: pass"));
    assert!(stdout.contains("Proxy structural validation: pass"));
    assert!(stdout.contains("Moxy relation validation: pass"));
    assert!(stdout.contains("Foxy reflection validation: pass"));
    assert!(stdout.contains("Being / Object ontology: pass"));
    assert!(stdout.contains("Skill relation validation: pass"));
    assert!(stdout.contains("Move resolution validation: pass"));
    assert!(stdout.contains("natural inheritance: pass"));
    assert!(stdout.contains("Hollowing target rules: pass"));
    assert!(stdout.contains("Synthesis cross-boundary rules: pass"));
    assert!(stdout.contains("Proxy / Moxy / Foxy Being/Object addressing: pass"));
    assert!(stdout.contains("civic-body House mappings: pass"));
    assert!(stdout.contains("civic-body people mappings: pass"));
    assert!(stdout.contains("civic-body crisis loop: pass"));
    assert!(stdout.contains("civic-body Being/Object boundary: pass"));
    assert!(stdout.contains("Flow/Glow distinction: pass"));
    assert!(stdout.contains("Seam/Beam/Gleam distinction: pass"));
    assert!(stdout.contains("Grip/Show/Grit distinction: pass"));
    assert!(stdout.contains("canonical pairings: pass"));
    assert!(stdout.contains("Stonebend apex mapping: pass"));
    assert!(stdout.contains("embodied action grammar: pass"));
    assert!(stdout.contains("shared Grip root: pass"));
    assert!(stdout.contains("seven Grip expressions: pass"));
    assert!(stdout.contains("lower-expression retention: pass"));
    assert!(stdout.contains("current-inheritance command surface: pass"));
    assert!(stdout.contains("grip witness surface: pass"));
    assert!(stdout.contains("Stairway horizon fixture: pass"));
    assert!(stdout.contains("vertical witness: pass"));
    assert!(stdout.contains("V1.1 topology unchanged: pass"));
}

#[test]
fn main_binary_delegates_scenario_listing() {
    let output = Command::new(env!("CARGO_BIN_EXE_hollow-grove"))
        .args(["scenario", "list"])
        .output()
        .expect("scenario list should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Scenario List"));
    assert!(stdout.contains("flooded_quarry_night_watch"));
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
            "#!/usr/bin/env bash\nset -euo pipefail\necho \"$@\" >> \"{}\"\nif [[ \"$1\" == \"msg\" && \"$2\" == \"-j\" && \"${{3-}}\" == \"workspaces\" ]]; then\n  printf '%s\\n' '[{{\"id\":1,\"idx\":1,\"name\":null,\"output\":\"DP-2\",\"is_urgent\":false,\"is_active\":true,\"is_focused\":true,\"active_window_id\":4}}]'\nelif [[ \"$1\" == \"msg\" && \"$2\" == \"-j\" && \"${{3-}}\" == \"focused-window\" ]]; then\n  printf '%s\\n' '{{\"id\":4,\"title\":\"Terminal\",\"app_id\":\"kitty\",\"output\":\"DP-2\",\"geometry\":{{\"x\":320,\"y\":180,\"width\":1280,\"height\":720}}}}'\nelif [[ \"$1\" == \"msg\" && \"$2\" == \"-j\" && \"${{3-}}\" == \"outputs\" ]]; then\n  printf '%s\\n' '[{{\"name\":\"DP-2\",\"is_focused\":true,\"logical\":{{\"x\":0,\"y\":0,\"width\":2560,\"height\":1440}}}}]'\nelif [[ \"$1\" == \"msg\" && \"$2\" == \"overview-state\" ]]; then\n  printf '%s\\n' 'Overview is open.'\nelif [[ \"$1\" == \"msg\" && \"$2\" == \"action\" && \"${{3-}}\" == \"open-overview\" ]]; then\n  :\nelif [[ \"$1\" == \"msg\" && \"$2\" == \"action\" && \"${{3-}}\" == \"close-overview\" ]]; then\n  :\nelif [[ \"$1\" == \"msg\" && \"$2\" == \"action\" && \"${{3-}}\" == \"focus-workspace\"* ]]; then\n  :\nelif [[ \"$1\" == \"msg\" && \"$2\" == \"action\" && \"${{3-}}\" == \"set-workspace-name\"* ]]; then\n  :\nelse\n  printf '%s\\n' \"unexpected niri invocation: $*\" >&2\n  exit 1\nfi\n",
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
    assert!(wait_for_path(&temp_artifacts.join("screen_map_state.json")));
    assert!(wait_for_path(&niri_log));

    let runtime_status = fs::read_to_string(temp_artifacts.join("runtime_loop_status.md"))
        .expect("runtime status should exist");
    let bridge_status = fs::read_to_string(temp_artifacts.join("niri_bridge_status.md"))
        .expect("bridge status should exist");
    let screen_map_state = fs::read_to_string(temp_artifacts.join("screen_map_state.json"))
        .expect("screen map state should exist");
    let niri_log_contents = fs::read_to_string(&niri_log).expect("niri log should exist");

    assert!(runtime_status.contains("cycle: 2"));
    assert!(runtime_status.contains("refreshed pipeline"));
    assert!(bridge_status.contains("apply_enabled: true"));
    assert!(screen_map_state.contains("\"status\": \"ok\""));
    assert!(screen_map_state.contains("\"center\":{\"x\":0.375,\"y\":0.375}"));
    assert!(niri_log_contents.contains("msg -j workspaces"));
    assert!(niri_log_contents.contains("msg -j focused-window"));
    assert!(niri_log_contents.contains("msg -j outputs"));
    assert!(niri_log_contents.contains("msg overview-state"));
    assert!(niri_log_contents.contains("msg action set-workspace-name HollowGrove"));

    fs::remove_dir_all(&temp_root).expect("temp cleanup should succeed");
}
