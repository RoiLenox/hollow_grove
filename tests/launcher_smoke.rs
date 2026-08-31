use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
#[cfg(unix)]
use std::os::unix::fs::symlink;

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

fn symlink_file(src: &Path, dst: &Path) {
    #[cfg(unix)]
    {
        symlink(src, dst).expect("symlink should create");
    }
    #[cfg(not(unix))]
    {
        fs::copy(src, dst).expect("file copy should succeed");
    }
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
fn local_scripts_derive_the_checkout_root_instead_of_naming_one() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for script in [
        "bench-local.sh",
        "stress-local.sh",
        "run-local.sh",
        "run-runtime.sh",
        "run-runtime-niri.sh",
        "verify-fast.sh",
        "verify-playable-lifecycle.sh",
    ] {
        let contents =
            fs::read_to_string(repo_root.join(script)).expect("local script should read");
        assert!(
            contents.contains("${BASH_SOURCE[0]}"),
            "{script} should derive its own location"
        );
        assert!(
            !contents.contains("/home/"),
            "{script} should not name a personal checkout"
        );
    }
}

#[test]
fn runtime_launchers_use_their_staged_checkout_in_explicit_dev_mode() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let temp_root = unique_temp_dir("hollow-grove-portable-launchers");
    let temp_bin = temp_root.join("bin");
    let temp_target_debug = temp_root.join("target/debug");
    fs::create_dir_all(&temp_bin).expect("temp bin should create");
    fs::create_dir_all(&temp_target_debug).expect("temp target should create");

    for script in ["run-runtime.sh", "run-runtime-niri.sh"] {
        create_executable_script(
            &temp_root.join(script),
            &fs::read_to_string(repo_root.join(script)).expect("launcher should read"),
        );
    }

    let cargo_log = temp_root.join("cargo.log");
    let launch_log = temp_root.join("launch.log");
    create_executable_script(
        &temp_bin.join("cargo"),
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s|%s\\n' \"$PWD\" \"$*\" >> '{}'\n",
            cargo_log.display()
        ),
    );
    create_executable_script(
        &temp_target_debug.join("hollow-grove"),
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s|%s\\n' \"$PWD\" \"$*\" >> '{}'\n",
            launch_log.display()
        ),
    );
    create_executable_script(
        &temp_target_debug.join("hollow_grove_runtime"),
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s|%s\\n' \"$PWD\" \"$*\" >> '{}'\n",
            launch_log.display()
        ),
    );

    let path = format!(
        "{}:{}",
        temp_bin.display(),
        std::env::var("PATH").unwrap_or_default()
    );
    for (script, argument) in [
        ("run-runtime.sh", "runtime-probe"),
        ("run-runtime-niri.sh", "desktop-probe"),
    ] {
        let status = Command::new("bash")
            .current_dir(std::env::temp_dir())
            .env_remove("HOLLOW_GROVE_ROOT")
            .env("PATH", &path)
            .arg(temp_root.join(script))
            .arg("--dev")
            .arg(argument)
            .status()
            .expect("portable launcher should run");
        assert!(status.success());
    }

    let cargo_log = fs::read_to_string(cargo_log).expect("cargo log should read");
    let launch_log = fs::read_to_string(launch_log).expect("launch log should read");
    assert_eq!(cargo_log.lines().count(), 2);
    assert!(
        cargo_log
            .lines()
            .all(|line| line.contains("build --manifest-path"))
    );
    assert!(launch_log.contains(&format!("{}|runtime-probe", temp_root.display())));
    assert!(launch_log.contains(&format!("{}|desktop desktop-probe", temp_root.display())));

    fs::remove_dir_all(&temp_root).expect("temp cleanup should succeed");
}

#[test]
fn launcher_runs_runtime_and_bridge_in_a_staged_workspace() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let temp_root = unique_temp_dir("hollow-grove-launcher");
    let temp_bin = temp_root.join("bin");
    let temp_target_debug = temp_root.join("target").join("debug");
    let temp_artifacts = temp_root.join("artifacts");
    fs::create_dir_all(&temp_bin).expect("temp bin should create");
    fs::create_dir_all(&temp_target_debug).expect("temp target should create");
    fs::create_dir_all(&temp_artifacts).expect("temp artifacts should create");
    fs::write(
        temp_artifacts.join("index.md"),
        "# Artifact Index\n\nindex\n",
    )
    .expect("artifact index should write");

    symlink_file(&repo_root.join("Cargo.toml"), &temp_root.join("Cargo.toml"));
    if repo_root.join("Cargo.lock").exists() {
        symlink_file(&repo_root.join("Cargo.lock"), &temp_root.join("Cargo.lock"));
    }
    symlink_file(&repo_root.join("src"), &temp_root.join("src"));
    symlink_file(
        &repo_root.join("run-runtime-niri.sh"),
        &temp_root.join("run-runtime-niri.sh"),
    );

    symlink_file(
        &repo_root.join("target/debug/hollow-grove"),
        &temp_target_debug.join("hollow-grove"),
    );
    symlink_file(
        &repo_root.join("target/debug/hollow_grove_runtime"),
        &temp_target_debug.join("hollow_grove_runtime"),
    );
    symlink_file(
        &repo_root.join("target/debug/hollow_grove_niri_bridge"),
        &temp_target_debug.join("hollow_grove_niri_bridge"),
    );

    let cargo_log = temp_root.join("cargo.log");
    let niri_log = temp_root.join("niri.log");

    create_executable_script(
        &temp_bin.join("cargo"),
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\necho \"$@\" >> \"{}\"\nexit 0\n",
            cargo_log.display()
        ),
    );
    create_executable_script(
        &temp_bin.join("niri"),
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\necho \"$@\" >> \"{}\"\nif [[ \"$1\" == \"msg\" && \"$2\" == \"-j\" && \"${{3-}}\" == \"workspaces\" ]]; then\n  printf '%s\\n' '[{{\"id\":1,\"idx\":1,\"name\":\"HollowGrove\",\"output\":\"DP-2\",\"is_urgent\":false,\"is_active\":true,\"is_focused\":true,\"active_window_id\":4}}]'\nelif [[ \"$1\" == \"msg\" && \"$2\" == \"-j\" && \"${{3-}}\" == \"focused-window\" ]]; then\n  printf '%s\\n' '{{\"id\":4,\"title\":\"Terminal\",\"app_id\":\"kitty\",\"output\":\"DP-2\",\"geometry\":{{\"x\":320,\"y\":180,\"width\":1280,\"height\":720}}}}'\nelif [[ \"$1\" == \"msg\" && \"$2\" == \"-j\" && \"${{3-}}\" == \"outputs\" ]]; then\n  printf '%s\\n' '[{{\"name\":\"DP-2\",\"is_focused\":true,\"logical\":{{\"x\":0,\"y\":0,\"width\":2560,\"height\":1440}}}}]'\nelif [[ \"$1\" == \"msg\" && \"$2\" == \"overview-state\" ]]; then\n  printf '%s\\n' 'Overview is open.'\nelif [[ \"$1\" == \"msg\" && \"$2\" == \"action\" && \"${{3-}}\" == \"open-overview\" ]]; then\n  :\nelif [[ \"$1\" == \"msg\" && \"$2\" == \"action\" && \"${{3-}}\" == \"close-overview\" ]]; then\n  :\nelif [[ \"$1\" == \"msg\" && \"$2\" == \"action\" && \"${{3-}}\" == \"focus-workspace\"* ]]; then\n  :\nelse\n  printf '%s\\n' \"unexpected niri invocation: $*\" >&2\n  exit 1\nfi\n",
            niri_log.display()
        ),
    );

    let runtime_input = temp_artifacts.join("runtime_input.txt");
    let runtime_memory = temp_artifacts.join("runtime_memory.txt");
    fs::write(
        &runtime_input,
        "runtime_mode: run\norigin: symptom-origin\noperator_note: launcher smoke\n",
    )
    .expect("runtime input should write");
    fs::write(
        &runtime_memory,
        "# Hollow Grove Runtime Memory\n\
         last_cycle: 1\n\
         last_unix_time_s: 1234567890\n\
         last_runtime_mode: run\n\
         last_action_taken: refreshed pipeline\n\
         last_origin: symptom-origin\n\
         last_operator_note: launcher smoke\n\
         last_should_stop: false\n\
         last_witness: Point\\n↓\\nTriway\\n↓\\nFourway\\n↓\\nHollowGrove\\n↓\\nCurrentSeam\\n↓\\nAuraBeam\\n↓\\nPoint² (Landed Point)\n",
    )
    .expect("runtime memory should write");

    let status = Command::new("bash")
        .current_dir(&temp_root)
        .env("HOLLOW_GROVE_ROOT", &temp_root)
        .env(
            "PATH",
            format!(
                "{}:{}",
                temp_bin.display(),
                std::env::var("PATH").unwrap_or_default()
            ),
        )
        .arg("./run-runtime-niri.sh")
        .args(["--dev", "--cycles", "2", "--interval-ms", "1500", "--quiet"])
        .env("HOLLOW_GROVE_NIRI_CLI", "1")
        .status()
        .expect("launcher should run");

    assert!(status.success());

    assert!(wait_for_path(
        &temp_artifacts.join("runtime_loop_status.md")
    ));
    assert!(wait_for_path(&niri_log));

    let launcher_runtime_status = fs::read_to_string(temp_artifacts.join("runtime_loop_status.md"))
        .expect("runtime status should exist");
    assert!(launcher_runtime_status.contains("cycle: 2"));
    assert!(launcher_runtime_status.contains("refreshed pipeline"));
    let niri_log_contents = fs::read_to_string(&niri_log).expect("niri log should exist");
    let cargo_log_contents = fs::read_to_string(&cargo_log).expect("cargo log should exist");
    assert!(cargo_log_contents.contains("--bin hollow-grove"));
    assert!(niri_log_contents.contains("msg -j workspaces"));

    fs::remove_dir_all(&temp_root).expect("temp cleanup should succeed");
}
