use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use hollow_grove::{
    CANONICAL_WITNESS, DESKTOP_STATUS_ARTIFACT_PATH, PROMPT_ARTIFACT_PATH, SNAPSHOT_ARTIFACT_PATH,
};

const ARTIFACT_INDEX_PATH: &str = "artifacts/index.md";
const RUNTIME_INPUT_PATH: &str = "artifacts/runtime_input.txt";
const RUNTIME_MEMORY_PATH: &str = "artifacts/runtime_memory.txt";
const RUNTIME_STATUS_PATH: &str = "artifacts/runtime_loop_status.md";
fn unique_temp_dir(prefix: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("{prefix}-{nonce}"))
}

fn write_fixture(root: &Path, relative_path: &str, contents: &str) {
    let path = root.join(relative_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("fixture directory should create");
    }
    fs::write(path, contents).expect("fixture should write");
}

#[test]
fn runtime_binary_refreshes_the_loop_contract_end_to_end() {
    let temp_root = unique_temp_dir("hollow-grove-runtime-loop");
    fs::create_dir_all(&temp_root).expect("temp root should create");
    write_fixture(
        &temp_root,
        ARTIFACT_INDEX_PATH,
        "# Artifact Index\n\nindex\n",
    );

    let status = Command::new(env!("CARGO_BIN_EXE_hollow_grove_runtime"))
        .current_dir(&temp_root)
        .args(["--cycles", "1", "--quiet"])
        .status()
        .expect("runtime binary should launch");

    assert!(status.success());

    let runtime_input =
        fs::read_to_string(temp_root.join(RUNTIME_INPUT_PATH)).expect("runtime input should exist");
    let runtime_memory = fs::read_to_string(temp_root.join(RUNTIME_MEMORY_PATH))
        .expect("runtime memory should exist");
    let runtime_status = fs::read_to_string(temp_root.join(RUNTIME_STATUS_PATH))
        .expect("runtime status should exist");
    let snapshot =
        fs::read_to_string(temp_root.join(SNAPSHOT_ARTIFACT_PATH)).expect("snapshot should exist");
    let prompt =
        fs::read_to_string(temp_root.join(PROMPT_ARTIFACT_PATH)).expect("prompt should exist");
    let desktop_status = fs::read_to_string(temp_root.join(DESKTOP_STATUS_ARTIFACT_PATH))
        .expect("desktop status should exist");

    assert!(runtime_input.contains("runtime_mode: run"));
    assert!(runtime_memory.contains("last_cycle: 1"));
    assert!(runtime_memory.contains("last_runtime_mode: run"));
    assert!(runtime_status.contains("cycle: 1"));
    assert!(runtime_status.contains("refreshed pipeline"));
    assert!(snapshot.contains("\"start\": \"Symptom 1\""));
    assert!(prompt.contains("start Symptom 1"));
    assert!(desktop_status.contains(CANONICAL_WITNESS));

    fs::remove_dir_all(&temp_root).expect("temp cleanup should succeed");
}
