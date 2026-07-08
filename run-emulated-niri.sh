#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="${HOLLOW_GROVE_REPO_ROOT:-$SCRIPT_DIR}"
EMULATED_ROOT="${HOLLOW_GROVE_EMULATED_ROOT:-$(mktemp -d -t hollow-grove-emulated-XXXXXX)}"
DEFAULT_ARGS=(--cycles 2 --interval-ms 1500 --quiet)

link_path() {
    local source_path="$1"
    local target_path="$2"

    rm -rf "$target_path"
    ln -s "$source_path" "$target_path"
}

write_fake_niri() {
    local fake_niri_path="$1"
    local state_root="$2"

    cat >"$fake_niri_path" <<'EOF'
#!/usr/bin/env bash

set -euo pipefail

STATE_ROOT="${HOLLOW_GROVE_EMULATED_ROOT:?}"
WORKSPACE_NAME_FILE="$STATE_ROOT/fake-niri-workspace-name.txt"
OVERVIEW_STATE_FILE="$STATE_ROOT/fake-niri-overview-state.txt"
LOG_FILE="$STATE_ROOT/fake-niri.log"

printf '%s\n' "$*" >>"$LOG_FILE"

workspace_name_json() {
    if [[ -f "$WORKSPACE_NAME_FILE" ]]; then
        local workspace_name
        workspace_name="$(<"$WORKSPACE_NAME_FILE")"
        printf '"%s"' "$workspace_name"
    else
        printf 'null'
    fi
}

overview_state() {
    if [[ -f "$OVERVIEW_STATE_FILE" ]]; then
        cat "$OVERVIEW_STATE_FILE"
    else
        printf 'Overview is closed.\n'
    fi
}

if [[ "${1-}" != "msg" ]]; then
    printf 'unexpected fake niri invocation: %s\n' "$*" >&2
    exit 1
fi

if [[ "${2-}" == "-j" && "${3-}" == "workspaces" ]]; then
    printf '[{"id":1,"idx":1,"name":%s,"output":"DP-2","is_urgent":false,"is_active":true,"is_focused":true,"active_window_id":4}]\n' "$(workspace_name_json)"
    exit 0
fi

if [[ "${2-}" == "overview-state" ]]; then
    overview_state
    exit 0
fi

if [[ "${2-}" == "action" && "${3-}" == "set-workspace-name" ]]; then
    printf '%s\n' "${4-}" >"$WORKSPACE_NAME_FILE"
    exit 0
fi

if [[ "${2-}" == "action" && "${3-}" == "open-overview" ]]; then
    printf 'Overview is open.\n' >"$OVERVIEW_STATE_FILE"
    exit 0
fi

if [[ "${2-}" == "action" && "${3-}" == "close-overview" ]]; then
    printf 'Overview is closed.\n' >"$OVERVIEW_STATE_FILE"
    exit 0
fi

if [[ "${2-}" == "action" && "${3-}" == "focus-workspace" ]]; then
    exit 0
fi

printf 'unexpected fake niri invocation: %s\n' "$*" >&2
exit 1
EOF

    chmod +x "$fake_niri_path"
}

write_fake_cargo() {
    local fake_cargo_path="$1"

    cat >"$fake_cargo_path" <<'EOF'
#!/usr/bin/env bash

set -euo pipefail

printf '%s\n' "$*" >>"${HOLLOW_GROVE_EMULATED_ROOT:?}/fake-cargo.log"
exit 0
EOF

    chmod +x "$fake_cargo_path"
}

mkdir -p "$EMULATED_ROOT/bin" "$EMULATED_ROOT/target/debug" "$EMULATED_ROOT/artifacts"

cargo build \
    --manifest-path "$REPO_ROOT/Cargo.toml" \
    --bin hollow-grove \
    --bin hollow_grove_runtime \
    --bin hollow_grove_niri_bridge >/dev/null

printf '# Artifact Index\n\nindex\n' >"$EMULATED_ROOT/artifacts/index.md"

link_path "$REPO_ROOT/Cargo.toml" "$EMULATED_ROOT/Cargo.toml"
if [[ -f "$REPO_ROOT/Cargo.lock" ]]; then
    link_path "$REPO_ROOT/Cargo.lock" "$EMULATED_ROOT/Cargo.lock"
fi
link_path "$REPO_ROOT/src" "$EMULATED_ROOT/src"
link_path "$REPO_ROOT/run-runtime-niri.sh" "$EMULATED_ROOT/run-runtime-niri.sh"
link_path "$REPO_ROOT/target/debug/hollow-grove" "$EMULATED_ROOT/target/debug/hollow-grove"
link_path "$REPO_ROOT/target/debug/hollow_grove_runtime" "$EMULATED_ROOT/target/debug/hollow_grove_runtime"
link_path "$REPO_ROOT/target/debug/hollow_grove_niri_bridge" "$EMULATED_ROOT/target/debug/hollow_grove_niri_bridge"

write_fake_cargo "$EMULATED_ROOT/bin/cargo"
write_fake_niri "$EMULATED_ROOT/bin/niri" "$EMULATED_ROOT"

export HOLLOW_GROVE_EMULATED_ROOT="$EMULATED_ROOT"

RUN_ARGS=("$@")
if [[ "${#RUN_ARGS[@]}" -eq 0 ]]; then
    RUN_ARGS=("${DEFAULT_ARGS[@]}")
fi

(
    cd "$EMULATED_ROOT"
    export HOLLOW_GROVE_ROOT="$EMULATED_ROOT"
    export PATH="$EMULATED_ROOT/bin:$PATH"
    ./run-runtime-niri.sh "${RUN_ARGS[@]}"
)

printf 'Emulated root: %s\n' "$EMULATED_ROOT"
printf 'Runtime status: %s\n' "$EMULATED_ROOT/artifacts/runtime_loop_status.md"
printf 'Bridge status: %s\n' "$EMULATED_ROOT/artifacts/niri_bridge_status.md"
printf 'Fake Niri log: %s\n' "$EMULATED_ROOT/fake-niri.log"
