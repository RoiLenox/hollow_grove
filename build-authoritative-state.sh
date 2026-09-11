#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
OUTPUT_ROOT="${OUTPUT_ROOT:-$SCRIPT_DIR/dist}"
VERSION="$(sed -n 's/^version = "\([^"]*\)"/\1/p' "$SCRIPT_DIR/Cargo.toml" | head -n1)"
STATE_NAME="hollow-grove-authoritative-state-$VERSION"
STATE_ARCHIVE="$OUTPUT_ROOT/$STATE_NAME.tar.gz"
STAGING_ROOT="$(mktemp -d --tmpdir hollow-grove-authoritative-state.XXXXXX)"
STATE_ROOT="$STAGING_ROOT/$STATE_NAME"

cleanup() {
    if [[ -n "$STAGING_ROOT" && "$STAGING_ROOT" == /tmp/hollow-grove-authoritative-state.* \
        && -d "$STAGING_ROOT" ]]; then
        rm -rf -- "$STAGING_ROOT"
    fi
}
trap cleanup EXIT INT TERM

if [[ -z "$VERSION" || "$OUTPUT_ROOT" == "/" ]]; then
    printf 'refusing unsafe or versionless state path\n' >&2
    exit 2
fi
if [[ -n "$(git -C "$SCRIPT_DIR" status --porcelain=v1 --untracked-files=normal)" ]]; then
    printf 'authoritative state requires a clean, checkpointed source worktree\n' >&2
    exit 2
fi

mkdir -p "$OUTPUT_ROOT" "$STATE_ROOT/payload/home" "$STATE_ROOT/payload/project-state"

copy_home_path() {
    local relative_path="$1"
    local source_path="$HOME/$relative_path"
    if [[ ! -e "$source_path" ]]; then
        return
    fi
    mkdir -p "$STATE_ROOT/payload/home/$(dirname -- "$relative_path")"
    cp -a -- "$source_path" "$STATE_ROOT/payload/home/$relative_path"
}

home_paths=(
    .config/niri/config.kdl
    .config/niri/HOLLOW_GROVE_KEYBOARD_STANDARD_V2.md
    .config/hollow-grove-hud
    .config/systemd/user/hollow-niri.service
    .config/systemd/user/niri-tiling-guard.path
    .config/systemd/user/niri-tiling-guard.service
    .config/systemd/user/niri-tiling-guard.timer
    .config/systemd/user/niri.service
    .local/bin/ardour9-niri
    .local/bin/audiotube-niri
    .local/bin/firefox
    .local/bin/hollow-grove-controls
    .local/bin/hollow-keyboard
    .local/bin/lmms-niri
    .local/bin/lock-session
    .local/bin/niri-tiling-guard
    .local/bin/niri-window-to-adjacent-column
    .local/bin/niri-wrap-move
    .local/bin/walker-toggle-niri
)
for relative_path in "${home_paths[@]}"; do
    copy_home_path "$relative_path"
done

copy_filtered_state() {
    local source_path="$1"
    local destination_path="$2"
    if [[ ! -d "$source_path" ]]; then
        return
    fi
    mkdir -p "$destination_path"
    tar --exclude='*.lock' --exclude='*.claim' \
        -C "$source_path" -cf - . | tar -C "$destination_path" -xf -
}

copy_filtered_state \
    "$HOME/.local/share/hollow-grove" \
    "$STATE_ROOT/payload/home/.local/share/hollow-grove"
copy_filtered_state \
    "$HOME/.local/state/hollow-grove" \
    "$STATE_ROOT/payload/home/.local/state/hollow-grove"
copy_filtered_state \
    "$SCRIPT_DIR/artifacts/gameplay-saves" \
    "$STATE_ROOT/payload/project-state/artifacts/gameplay-saves"

{
    printf 'schema_version=1\n'
    printf 'version=%s\n' "$VERSION"
    printf 'git_commit=%s\n' "$(git -C "$SCRIPT_DIR" rev-parse HEAD)"
    printf 'source_host=%s\n' "$(uname -n 2>/dev/null || printf unknown)"
    printf 'created_utc=%s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
    printf 'policy=stage-before-hardware-adaptation\n'
    printf 'excluded=credentials,target,package-build-caches,runtime-locks,runtime-claims\n'
} >"$STATE_ROOT/TRANSFER-MANIFEST.txt"

(
    cd "$STATE_ROOT"
    find payload -type f -print0 | sort -z | xargs -0 sha256sum >PAYLOAD-SHA256SUMS
)
tar --sort=name --owner=0 --group=0 --numeric-owner \
    -C "$STAGING_ROOT" -czf "$STATE_ARCHIVE" "$STATE_NAME"
(
    cd "$OUTPUT_ROOT"
    sha256sum "$(basename -- "$STATE_ARCHIVE")" >STATE-SHA256SUMS
)

printf 'authoritative workstation state ready: %s\n' "$STATE_ARCHIVE"
