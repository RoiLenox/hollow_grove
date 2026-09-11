#!/usr/bin/env bash

set -euo pipefail

SCRIPT_PATH="$(readlink -f -- "${BASH_SOURCE[0]}")"
REPO_ROOT="$(cd -- "$(dirname -- "$SCRIPT_PATH")" && pwd)"
CONFIG_ROOT="${XDG_CONFIG_HOME:-$HOME/.config}/hollow-grove"
CONFIG_PATH="${HOLLOW_GROVE_SYNC_CONFIG:-$CONFIG_ROOT/device-sync.conf}"
PEER_HOST=""
PEER_REPO=""

usage() {
    printf '%s\n' \
        'usage: hollow-grove-update COMMAND [options]' \
        '' \
        'Commands:' \
        '  configure --host USER@HOST --remote-repo /absolute/path' \
        '      Save the peer address without storing credentials.' \
        '  status' \
        '      Show the local checkpoint and the peer state when reachable.' \
        '  prepare' \
        '      Run release gates and build source, release, and state payloads.' \
        '  prepare-state' \
        '      Refresh only the checksummed saves/configuration payload.' \
        '  stage' \
        '      Send and verify an update without switching the peer.' \
        '  activate [--replace-diverged]' \
        '      Send again and atomically switch the peer after verification.' \
        '' \
        'Run the same command on either device; the device invoking stage or' \
        'activate is the explicit source of truth for that update.'
}

validate_peer() {
    if [[ ! "$PEER_HOST" =~ ^[A-Za-z0-9._@:-]+$ ]]; then
        printf 'invalid or missing peer_host in %s\n' "$CONFIG_PATH" >&2
        exit 2
    fi
    if [[ ! "$PEER_REPO" =~ ^/[A-Za-z0-9._/-]+$ ]]; then
        printf 'invalid or missing peer_repo in %s\n' "$CONFIG_PATH" >&2
        exit 2
    fi
}

load_peer() {
    if [[ ! -f "$CONFIG_PATH" ]]; then
        printf 'peer is not configured; run hollow-grove-update configure first\n' >&2
        exit 2
    fi
    while IFS='=' read -r key value; do
        case "$key" in
            peer_host) PEER_HOST="$value" ;;
            peer_repo) PEER_REPO="$value" ;;
        esac
    done <"$CONFIG_PATH"
    validate_peer
}

require_clean_source() {
    if [[ -n "$(git -C "$REPO_ROOT" status --porcelain=v1 --untracked-files=normal)" ]]; then
        printf 'source worktree is dirty; commit or recover changes before preparing an update\n' >&2
        exit 2
    fi
}

configure_peer() {
    shift
    while (( $# > 0 )); do
        case "$1" in
            --host)
                PEER_HOST="${2:-}"
                shift 2
                ;;
            --remote-repo)
                PEER_REPO="${2:-}"
                shift 2
                ;;
            *)
                printf 'unknown configure option: %s\n' "$1" >&2
                exit 2
                ;;
        esac
    done
    validate_peer
    mkdir -p "$CONFIG_ROOT"
    temporary_config="$(mktemp "$CONFIG_ROOT/.device-sync.conf.XXXXXX")"
    chmod 0600 "$temporary_config"
    {
        printf 'peer_host=%s\n' "$PEER_HOST"
        printf 'peer_repo=%s\n' "$PEER_REPO"
    } >"$temporary_config"
    mv -f "$temporary_config" "$CONFIG_PATH"
    printf 'configured peer %s:%s\n' "$PEER_HOST" "$PEER_REPO"
}

show_local_status() {
    local worktree_clean="no"
    printf 'local_device=%s\n' "$(uname -n 2>/dev/null || printf unknown)"
    printf 'local_repo=%s\n' "$REPO_ROOT"
    printf 'local_branch=%s\n' "$(git -C "$REPO_ROOT" branch --show-current)"
    printf 'local_commit=%s\n' "$(git -C "$REPO_ROOT" rev-parse HEAD)"
    printf 'local_describe=%s\n' "$(git -C "$REPO_ROOT" describe --tags --always --dirty)"
    if [[ -n "$(git -C "$REPO_ROOT" status --porcelain=v1 --untracked-files=normal)" ]]; then
        printf 'local_worktree=dirty\n'
    else
        printf 'local_worktree=clean\n'
        worktree_clean="yes"
    fi
    version="$(sed -n 's/^version = "\([^"]*\)"/\1/p' "$REPO_ROOT/Cargo.toml" | head -n1)"
    head_commit="$(git -C "$REPO_ROOT" rev-parse HEAD)"
    core_commit="$(
        {
            tar -xOf "$REPO_ROOT/dist/hollow-grove-core-$version.tar.gz" \
                "hollow-grove-core-$version/BUILD-MANIFEST.txt" 2>/dev/null \
                | sed -n 's/^git_commit=//p'
        } || true
    )"
    hueman_commit="$(
        {
            tar -xOf "$REPO_ROOT/dist/hueman-$version.tar.gz" \
                "hueman-$version/BUILD-MANIFEST.txt" 2>/dev/null \
                | sed -n 's/^git_commit=//p'
        } || true
    )"
    general_commit="$(
        {
            tar -xOf "$REPO_ROOT/dist/hollow-grove-general-$version.tar.gz" \
                "hollow-grove-general-$version/BUILD-MANIFEST.txt" 2>/dev/null \
                | sed -n 's/^git_commit=//p'
        } || true
    )"
    state_commit="$(
        {
            tar -xOf "$REPO_ROOT/dist/hollow-grove-authoritative-state-$version.tar.gz" \
                "hollow-grove-authoritative-state-$version/TRANSFER-MANIFEST.txt" 2>/dev/null \
                | sed -n 's/^git_commit=//p'
        } || true
    )"
    if [[ "$worktree_clean" == "yes" && "$core_commit" == "$head_commit" \
        && "$hueman_commit" == "$head_commit" \
        && "$general_commit" == "$head_commit" \
        && "$state_commit" == "$head_commit" ]] \
        && (cd "$REPO_ROOT/dist" 2>/dev/null \
            && sha256sum --check --status SHA256SUMS \
            && sha256sum --check --status STATE-SHA256SUMS); then
        printf 'local_payloads=current-and-verified\n'
    else
        printf 'local_payloads=missing-or-stale\n'
    fi
}

show_status() {
    show_local_status
    if [[ ! -f "$CONFIG_PATH" ]]; then
        printf 'peer=not-configured\n'
        return
    fi
    load_peer
    printf 'peer_target=%s:%s\n' "$PEER_HOST" "$PEER_REPO"
    if ssh -o BatchMode=yes -o ConnectTimeout=5 "$PEER_HOST" bash -s -- "$PEER_REPO" <<'REMOTE_STATUS'
set -euo pipefail
repo_path="$1"
printf 'peer_device=%s\n' "$(uname -n 2>/dev/null || printf unknown)"
if [[ ! -d "$repo_path/.git" ]]; then
    printf 'peer_repo=absent\n'
    exit 0
fi
printf 'peer_repo=%s\n' "$repo_path"
printf 'peer_branch=%s\n' "$(git -C "$repo_path" branch --show-current)"
printf 'peer_commit=%s\n' "$(git -C "$repo_path" rev-parse HEAD)"
printf 'peer_describe=%s\n' "$(git -C "$repo_path" describe --tags --always --dirty)"
if [[ -n "$(git -C "$repo_path" status --porcelain=v1 --untracked-files=normal)" ]]; then
    printf 'peer_worktree=dirty\n'
else
    printf 'peer_worktree=clean\n'
fi
REMOTE_STATUS
    then
        :
    else
        printf 'peer=unreachable\n'
    fi
}

prepare_all() {
    require_clean_source
    cd "$REPO_ROOT"
    ./verify-art-assets.sh
    cargo fmt --all -- --check
    cargo clippy --workspace --all-targets --all-features --locked --offline -- -D warnings
    cargo test --workspace --all-features --locked --offline
    packaging/hollow-grove-general/verify-keyboard-grove.sh
    godot --headless --path hueman_godot \
        --script res://tests/runtime_client_smoke.gd
    godot --headless --path hueman_godot \
        --script res://tests/retro_overworld_semantics_smoke.gd
    ./verify-playable-lifecycle.sh
    ./build-portable-release.sh
    ./build-authoritative-state.sh
    printf 'update payloads prepared for %s\n' "$(git rev-parse HEAD)"
}

prepare_state() {
    require_clean_source
    cd "$REPO_ROOT"
    ./build-authoritative-state.sh
    printf 'state payload refreshed for %s\n' "$(git rev-parse HEAD)"
}

send_update() {
    local mode="$1"
    shift
    load_peer
    extra_args=()
    if [[ "$mode" == "activate" ]]; then
        extra_args+=(--activate)
        if (( $# > 0 )); then
            if [[ "$1" == "--replace-diverged" && $# == 1 ]]; then
                extra_args+=(--replace-diverged)
            else
                printf 'unknown activate option\n' >&2
                exit 2
            fi
        fi
    else
        extra_args+=(--stage-only)
        if (( $# > 0 )); then
            printf 'stage accepts no options\n' >&2
            exit 2
        fi
    fi
    exec "$REPO_ROOT/sync-hollow-grove-laptop.sh" \
        --host "$PEER_HOST" \
        --remote-repo "$PEER_REPO" \
        --ref HEAD \
        "${extra_args[@]}"
}

command_name="${1:-help}"
case "$command_name" in
    configure) configure_peer "$@" ;;
    status) show_status ;;
    prepare) prepare_all ;;
    prepare-state) prepare_state ;;
    stage) shift; send_update stage "$@" ;;
    activate) shift; send_update activate "$@" ;;
    help|-h|--help) usage ;;
    *)
        printf 'unknown command: %s\n' "$command_name" >&2
        usage >&2
        exit 2
        ;;
esac
