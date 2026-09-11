#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
SSH_HOST=""
REMOTE_REPO=""
RELEASE_REF="HEAD"
ACTIVATE="no"
ALLOW_DIVERGED="no"
VERSION="$(sed -n 's/^version = "\([^"]*\)"/\1/p' "$SCRIPT_DIR/Cargo.toml" | head -n1)"
TRANSFER_ROOT=""

usage() {
    printf '%s\n' \
        "usage: $0 --host USER@HOST --remote-repo /absolute/path [options]" \
        "" \
        "Options:" \
        "  --ref REF       source ref to transfer (default: clean checked-out HEAD)" \
        "  --activate      atomically replace the old checkout after verification" \
        "  --stage-only    preserve and verify side-by-side without switching (default)" \
        "  --replace-diverged  allow activation over dirty or non-ancestor source"
}

while (( $# > 0 )); do
    case "$1" in
        --host)
            SSH_HOST="${2:-}"
            shift 2
            ;;
        --remote-repo)
            REMOTE_REPO="${2:-}"
            shift 2
            ;;
        --ref)
            RELEASE_REF="${2:-}"
            shift 2
            ;;
        --activate)
            ACTIVATE="yes"
            shift
            ;;
        --stage-only)
            ACTIVATE="no"
            shift
            ;;
        --replace-diverged)
            ALLOW_DIVERGED="yes"
            shift
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            printf 'unknown option: %s\n' "$1" >&2
            usage >&2
            exit 2
            ;;
    esac
done

if [[ ! "$SSH_HOST" =~ ^[A-Za-z0-9._@:-]+$ ]]; then
    printf 'invalid or missing SSH host\n' >&2
    exit 2
fi
if [[ ! "$REMOTE_REPO" =~ ^/[A-Za-z0-9._/-]+$ ]]; then
    printf 'remote repo must be an absolute path without spaces\n' >&2
    exit 2
fi
if ! git -C "$SCRIPT_DIR" rev-parse --verify --quiet "$RELEASE_REF^{commit}" >/dev/null; then
    printf 'release ref does not exist locally: %s\n' "$RELEASE_REF" >&2
    exit 2
fi
if [[ -n "$(git -C "$SCRIPT_DIR" status --porcelain=v1 --untracked-files=normal)" ]]; then
    printf 'source worktree is not clean; checkpoint the complete transfer state first\n' >&2
    exit 2
fi
if [[ ! -f "$SCRIPT_DIR/dist/hollow-grove-core-$VERSION.tar.gz" \
    || ! -f "$SCRIPT_DIR/dist/hueman-$VERSION.tar.gz" \
    || ! -f "$SCRIPT_DIR/dist/hollow-grove-general-$VERSION.tar.gz" \
    || ! -f "$SCRIPT_DIR/dist/SHA256SUMS" \
    || ! -f "$SCRIPT_DIR/dist/hollow-grove-authoritative-state-$VERSION.tar.gz" \
    || ! -f "$SCRIPT_DIR/dist/STATE-SHA256SUMS" ]]; then
    printf 'portable release is missing; run ./build-portable-release.sh first\n' >&2
    printf 'authoritative state is missing; run ./build-authoritative-state.sh next\n' >&2
    exit 2
fi

release_commit="$(git -C "$SCRIPT_DIR" rev-parse "$RELEASE_REF^{commit}")"
if [[ "$release_commit" != "$(git -C "$SCRIPT_DIR" rev-parse HEAD)" ]]; then
    printf 'release ref must identify the clean checked-out transfer state\n' >&2
    exit 2
fi
release_branch="$(git -C "$SCRIPT_DIR" branch --show-current)"
if [[ -z "$release_branch" ]] \
    || ! git -C "$SCRIPT_DIR" check-ref-format --branch "$release_branch" >/dev/null 2>&1; then
    printf 'source must be checked out on a named branch for two-device updates\n' >&2
    exit 2
fi
TRANSFER_ROOT="$(mktemp -d --tmpdir hollow-grove-laptop-transfer.XXXXXX)"
cleanup() {
    if [[ -n "$TRANSFER_ROOT" && -d "$TRANSFER_ROOT" ]]; then
        rm -rf -- "$TRANSFER_ROOT"
    fi
}
trap cleanup EXIT INT TERM

git -C "$SCRIPT_DIR" bundle create "$TRANSFER_ROOT/hollow-grove.bundle" --all
cp "$SCRIPT_DIR/dist/hollow-grove-core-$VERSION.tar.gz" \
    "$SCRIPT_DIR/dist/hueman-$VERSION.tar.gz" \
    "$SCRIPT_DIR/dist/hollow-grove-general-$VERSION.tar.gz" \
    "$SCRIPT_DIR/dist/SHA256SUMS" \
    "$SCRIPT_DIR/dist/hollow-grove-authoritative-state-$VERSION.tar.gz" \
    "$SCRIPT_DIR/dist/STATE-SHA256SUMS" \
    "$TRANSFER_ROOT/"

remote_inbox="$(ssh "$SSH_HOST" 'mktemp -d --tmpdir hollow-grove-incoming.XXXXXX')"
if [[ ! "$remote_inbox" =~ ^/tmp/[A-Za-z0-9._/-]+$ ]]; then
    printf 'remote host returned an unsafe temporary path: %s\n' "$remote_inbox" >&2
    exit 1
fi
scp "$TRANSFER_ROOT"/* "$SSH_HOST:$remote_inbox/"

ssh "$SSH_HOST" bash -s -- \
    "$REMOTE_REPO" "$RELEASE_REF" "$release_commit" "$release_branch" "$VERSION" \
    "$ACTIVATE" "$ALLOW_DIVERGED" "$remote_inbox" <<'REMOTE_SCRIPT'
set -euo pipefail

repo_path="$1"
release_ref="$2"
release_commit="$3"
release_branch="$4"
version="$5"
activate="$6"
allow_diverged="$7"
incoming="$8"
stamp="$(date -u +%Y%m%dT%H%M%S.%NZ)"
repo_parent="$(dirname -- "$repo_path")"
repo_name="$(basename -- "$repo_path")"
staged_path="$repo_parent/${repo_name}.incoming-$stamp"
backup_path="$repo_parent/${repo_name}.pre-update-$stamp"
record_root="$repo_parent/hollow-grove-migration-records/$stamp"
state_stage="$record_root/workstation-state"
state_name="hollow-grove-authoritative-state-$version"
data_home="${XDG_DATA_HOME:-$HOME/.local/share}"
state_home="${XDG_STATE_HOME:-$HOME/.local/state}"
core_install_parent="$data_home/hollow-grove/releases"
core_install_path="$core_install_parent/core-$version-$release_commit"
core_current_link="$data_home/hollow-grove/current-core"
hueman_install_parent="$data_home/hueman/releases"
hueman_install_path="$hueman_install_parent/hueman-$version-$release_commit"
hueman_current_link="$data_home/hueman/current"
general_stage="$record_root/general-release"
release_payloads="$record_root/release-payloads"

cleanup_incoming() {
    if [[ -d "$incoming" ]]; then
        rm -rf -- "$incoming"
    fi
}
trap cleanup_incoming EXIT INT TERM

runtime_lock_is_active() {
    local lock_path="$1"
    local owner=""
    local pid
    local expected_start
    local stat_line
    local after_command
    local actual_start

    IFS= read -r owner <"$lock_path" || [[ -n "$owner" ]] || return 0
    if [[ ! "$owner" =~ \.runtime-lease\.([0-9]+)\.([0-9]+)\.[0-9]+\.claim$ ]]; then
        return 0
    fi
    pid="${BASH_REMATCH[1]}"
    expected_start="${BASH_REMATCH[2]}"
    if [[ "$expected_start" == "0" || ! -r "/proc/$pid/stat" ]]; then
        [[ "$expected_start" == "0" ]]
        return
    fi
    stat_line="$(<"/proc/$pid/stat")"
    after_command="${stat_line##*) }"
    actual_start="$(awk '{print $20}' <<<"$after_command")"
    [[ -n "$actual_start" && "$actual_start" == "$expected_start" ]]
}

assert_no_active_runtime() {
    local state_path="$1"
    local label="$2"
    local lock_path

    [[ -d "$state_path" ]] || return 0
    while IFS= read -r -d '' lock_path; do
        if runtime_lock_is_active "$lock_path"; then
            printf '%s\n' \
                "active or unverifiable Hollow Grove runtime lease blocks $label activation: $lock_path" >&2
            exit 1
        fi
    done < <(find "$state_path" -type f -name '*.runtime.lock' -print0)
}

old_repo_exists="no"
if [[ -e "$repo_path" ]]; then
    if [[ ! -d "$repo_path/.git" ]]; then
        printf 'remote path exists but is not a Git checkout: %s\n' "$repo_path" >&2
        exit 1
    fi
    old_repo_exists="yes"
fi
if [[ -e "$staged_path" || -e "$backup_path" ]]; then
    printf 'migration target already exists; refusing to overwrite it\n' >&2
    exit 1
fi

mkdir -p "$repo_parent" "$record_root"
if [[ "$old_repo_exists" == "yes" ]]; then
    git -C "$repo_path" rev-parse HEAD >"$record_root/old-head.txt"
    git -C "$repo_path" status --porcelain=v1 --branch >"$record_root/old-status.txt"
    git -C "$repo_path" diff --binary >"$record_root/old-working-tree.patch"
    git -C "$repo_path" diff --cached --binary >"$record_root/old-index.patch"
    git -C "$repo_path" bundle create "$record_root/old-history.bundle" --all
    git -C "$repo_path" ls-files --others --exclude-standard -z \
        >"$record_root/old-untracked-files.list"
    tar -C "$repo_path" --null -czf "$record_root/old-untracked-files.tar.gz" \
        -T "$record_root/old-untracked-files.list"
else
    printf 'absent\n' >"$record_root/old-status.txt"
fi

git clone --no-local "$incoming/hollow-grove.bundle" "$staged_path"
git -C "$staged_path" checkout -B "$release_branch" "$release_commit"
if [[ "$(git -C "$staged_path" rev-parse HEAD)" != "$release_commit" ]]; then
    printf 'staged checkout did not resolve to the transferred release commit\n' >&2
    exit 1
fi

if [[ "$activate" == "yes" && "$old_repo_exists" == "yes" \
    && "$allow_diverged" != "yes" ]]; then
    old_head="$(git -C "$repo_path" rev-parse HEAD)"
    old_changes="$(git -C "$repo_path" status --porcelain=v1 --untracked-files=normal)"
    if [[ -n "$old_changes" ]]; then
        printf 'remote checkout is dirty; incoming checkout is staged but activation requires review or --replace-diverged\n' >&2
        exit 1
    fi
    if ! git -C "$staged_path" cat-file -e "$old_head^{commit}" 2>/dev/null \
        || ! git -C "$staged_path" merge-base --is-ancestor "$old_head" "$release_commit"; then
        printf 'remote checkout is not an ancestor of the update; incoming checkout is staged but activation requires review or --replace-diverged\n' >&2
        exit 1
    fi
fi

(
    cd "$staged_path"
    ./verify-art-assets.sh
)

(
    cd "$incoming"
    sha256sum --check SHA256SUMS
    sha256sum --check STATE-SHA256SUMS
)
mkdir -p "$state_stage"
tar -xzf "$incoming/$state_name.tar.gz" -C "$state_stage"
(
    cd "$state_stage/$state_name"
    sha256sum --check PAYLOAD-SHA256SUMS
)
mkdir -p "$release_payloads" "$staged_path/dist"
cp -a "$incoming/." "$release_payloads/"
cp "$incoming/hollow-grove-core-$version.tar.gz" \
    "$incoming/hueman-$version.tar.gz" \
    "$incoming/hollow-grove-general-$version.tar.gz" \
    "$incoming/SHA256SUMS" \
    "$incoming/$state_name.tar.gz" \
    "$incoming/STATE-SHA256SUMS" \
    "$staged_path/dist/"

verify_release_dir() {
    local release_path="$1"
    local label="$2"
    (
        cd "$release_path"
        sha256sum --check SHA256SUMS
    )
    if ! grep -Fqx "git_commit=$release_commit" "$release_path/BUILD-MANIFEST.txt"; then
        printf '%s release was not built from the transferred source commit\n' "$label" >&2
        exit 1
    fi
}

install_release() {
    local archive_path="$1"
    local install_path="$2"
    local label="$3"
    local next_path="$install_path.next-$stamp"

    mkdir -p "$(dirname -- "$install_path")"
    if [[ ! -e "$install_path" ]]; then
        if [[ -e "$next_path" ]]; then
            printf 'release staging path already exists: %s\n' "$next_path" >&2
            exit 1
        fi
        mkdir -p "$next_path"
        tar -xzf "$archive_path" --strip-components=1 -C "$next_path"
        verify_release_dir "$next_path" "$label"
        mv "$next_path" "$install_path"
    elif [[ ! -d "$install_path" ]]; then
        printf '%s release path exists and is not a directory: %s\n' \
            "$label" "$install_path" >&2
        exit 1
    fi
    verify_release_dir "$install_path" "$label"
}

install_release \
    "$incoming/hollow-grove-core-$version.tar.gz" \
    "$core_install_path" \
    "Hollow Grove Core"
install_release \
    "$incoming/hueman-$version.tar.gz" \
    "$hueman_install_path" \
    "Hueman"

mkdir -p "$general_stage"
tar -xzf "$incoming/hollow-grove-general-$version.tar.gz" -C "$general_stage"
verify_release_dir "$general_stage/hollow-grove-general-$version" \
    "Hollow Grove General"

{
    printf 'release_ref=%s\n' "$release_ref"
    printf 'release_branch=%s\n' "$release_branch"
    printf 'release_commit=%s\n' "$release_commit"
    printf 'validation=source-commit,art-assets,archive-checksums,payload-checksums,build-manifest\n'
    printf 'staged_checkout=%s\n' "$staged_path"
    printf 'old_checkout=%s\n' "$repo_path"
    printf 'backup_record=%s\n' "$record_root"
    printf 'core_install=%s\n' "$core_install_path"
    printf 'hueman_install=%s\n' "$hueman_install_path"
    printf 'general_release=%s\n' "$general_stage/hollow-grove-general-$version"
    printf 'retained_release_payloads=%s\n' "$release_payloads"
    printf 'staged_workstation_state=%s\n' "$state_stage/$state_name"
} >"$record_root/migration-result.txt"

if [[ "$activate" == "yes" ]]; then
    assert_no_active_runtime \
        "$data_home/hollow-grove/gameplay-saves" \
        "durable gameplay state"
    if [[ "$old_repo_exists" == "yes" ]]; then
        assert_no_active_runtime \
            "$repo_path/artifacts/gameplay-saves" \
            "checkout gameplay state"
    fi

    if [[ "$old_repo_exists" == "yes" ]]; then
        mv "$repo_path" "$backup_path"
        if ! mv "$staged_path" "$repo_path"; then
            mv "$backup_path" "$repo_path"
            printf 'activation failed; original checkout was restored\n' >&2
            exit 1
        fi
    else
        mv "$staged_path" "$repo_path"
    fi

    activate_state_dir() {
        local source_path="$1"
        local destination_path="$2"
        local record_name="$3"
        local destination_parent
        local destination_next
        local destination_previous

        if [[ ! -d "$source_path" ]]; then
            return
        fi
        destination_parent="$(dirname -- "$destination_path")"
        destination_next="$destination_path.next-$stamp"
        destination_previous="$destination_path.pre-update-$stamp"
        mkdir -p "$destination_parent"
        if [[ -e "$destination_next" || -e "$destination_previous" ]]; then
            printf 'state activation target already exists; refusing to overwrite it: %s\n' \
                "$destination_path" >&2
            exit 1
        fi
        cp -a -- "$source_path" "$destination_next"
        if [[ -e "$destination_path" ]]; then
            tar -C "$destination_parent" -czf "$record_root/$record_name.before.tar.gz" \
                "$(basename -- "$destination_path")"
            mv "$destination_path" "$destination_previous"
        fi
        if ! mv "$destination_next" "$destination_path"; then
            if [[ -e "$destination_previous" && ! -e "$destination_path" ]]; then
                mv "$destination_previous" "$destination_path"
            fi
            printf 'state activation failed; previous state was restored\n' >&2
            exit 1
        fi
        printf '%s=%s\n' "activated_$record_name" "$destination_path" \
            >>"$record_root/migration-result.txt"
        if [[ -e "$destination_previous" ]]; then
            printf '%s=%s\n' "recoverable_$record_name" "$destination_previous" \
                >>"$record_root/migration-result.txt"
        fi
    }

    activate_state_dir \
        "$state_stage/$state_name/payload/home/.local/share/hollow-grove/gameplay-saves" \
        "$data_home/hollow-grove/gameplay-saves" \
        "user_gameplay_saves"
    activate_state_dir \
        "$state_stage/$state_name/payload/home/.local/share/hollow-grove/world" \
        "$data_home/hollow-grove/world" \
        "user_world"
    activate_state_dir \
        "$state_stage/$state_name/payload/home/.local/state/hollow-grove" \
        "$state_home/hollow-grove" \
        "user_runtime_state"
    activate_state_dir \
        "$state_stage/$state_name/payload/home/.local/state/hueman" \
        "$state_home/hueman" \
        "hueman_presentation_state"
    activate_state_dir \
        "$state_stage/$state_name/payload/project-state/artifacts/gameplay-saves" \
        "$repo_path/artifacts/gameplay-saves" \
        "project_gameplay_saves"

    activate_current_link() {
        local install_path="$1"
        local current_link="$2"
        local next_link="$current_link.next-$stamp"
        mkdir -p "$(dirname -- "$current_link")"
        if [[ -e "$next_link" || -L "$next_link" ]]; then
            printf 'release link staging path already exists: %s\n' "$next_link" >&2
            exit 1
        fi
        ln -s "$install_path" "$next_link"
        mv -Tf "$next_link" "$current_link"
    }

    mkdir -p "$HOME/.local/bin"
    activate_current_link "$core_install_path" "$core_current_link"
    activate_current_link "$hueman_install_path" "$hueman_current_link"
    ln -sfn "$core_current_link/bin/hollow-grove-service" \
        "$HOME/.local/bin/hollow-grove-service"
    ln -sfn "$core_current_link/bin/hollow-grove-with-hueman" \
        "$HOME/.local/bin/hollow-grove-with-hueman"
    ln -sfn "$core_current_link/bin/hollow-grove-hueman" \
        "$HOME/.local/bin/hollow-grove-hueman"
    ln -sfn "$hueman_current_link/bin/hueman" \
        "$HOME/.local/bin/hueman"
    ln -sfn "$repo_path/hollow-grove-update.sh" \
        "$HOME/.local/bin/hollow-grove-update"
    {
        printf 'activated_checkout=%s\n' "$repo_path"
        if [[ "$old_repo_exists" == "yes" ]]; then
            printf 'recoverable_old_checkout=%s\n' "$backup_path"
        fi
    } >>"$record_root/migration-result.txt"
    printf 'activated %s at %s\n' "$release_ref" "$repo_path"
    if [[ "$old_repo_exists" == "yes" ]]; then
        printf 'prior checkout preserved at %s\n' "$backup_path"
    fi
else
    printf 'staged and verified %s at %s; active checkout is unchanged\n' \
        "$release_ref" "$staged_path"
    printf 'Hollow Grove Core staged at %s; current link is unchanged\n' \
        "$core_install_path"
    printf 'Hueman staged at %s; current link is unchanged\n' \
        "$hueman_install_path"
fi
if [[ "$activate" == "yes" ]]; then
    printf 'Hollow Grove Core installed at %s and linked through %s\n' \
        "$core_install_path" "$core_current_link"
    printf 'Hueman installed at %s and linked through %s\n' \
        "$hueman_install_path" "$hueman_current_link"
    printf 'Durable gameplay saves, world, and runtime state were activated with recoverable backups\n'
fi
printf 'Workstation state staged for laptop adaptation at %s\n' "$state_stage/$state_name"
printf 'migration record: %s\n' "$record_root"
REMOTE_SCRIPT
