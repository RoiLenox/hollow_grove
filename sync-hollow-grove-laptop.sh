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
if [[ ! -f "$SCRIPT_DIR/dist/hollow-grove-hueman-$VERSION.tar.gz" \
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
cp "$SCRIPT_DIR/dist/hollow-grove-hueman-$VERSION.tar.gz" \
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
install_parent="${XDG_DATA_HOME:-$HOME/.local/share}/hollow-grove/releases"
install_path="$install_parent/hueman-$version-$release_commit"
current_link="${XDG_DATA_HOME:-$HOME/.local/share}/hollow-grove/current-hueman"

cleanup_incoming() {
    if [[ -d "$incoming" ]]; then
        rm -rf -- "$incoming"
    fi
}
trap cleanup_incoming EXIT INT TERM

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
mkdir -p "$install_parent" "$(dirname -- "$current_link")" "$HOME/.local/bin"
if [[ ! -e "$install_path" ]]; then
    mkdir -p "$install_path"
    tar -xzf "$incoming/hollow-grove-hueman-$version.tar.gz" \
        --strip-components=1 -C "$install_path"
elif [[ ! -d "$install_path" ]]; then
    printf 'Hueman release path exists and is not a directory: %s\n' "$install_path" >&2
    exit 1
fi
(
    cd "$install_path"
    sha256sum --check SHA256SUMS
)
if ! grep -Fqx "git_commit=$release_commit" "$install_path/BUILD-MANIFEST.txt"; then
    printf 'Hueman release was not built from the transferred source commit\n' >&2
    exit 1
fi

{
    printf 'release_ref=%s\n' "$release_ref"
    printf 'release_branch=%s\n' "$release_branch"
    printf 'release_commit=%s\n' "$release_commit"
    printf 'validation=source-commit,art-assets,archive-checksums,payload-checksums,build-manifest\n'
    printf 'staged_checkout=%s\n' "$staged_path"
    printf 'old_checkout=%s\n' "$repo_path"
    printf 'backup_record=%s\n' "$record_root"
    printf 'hueman_install=%s\n' "$install_path"
    printf 'staged_workstation_state=%s\n' "$state_stage/$state_name"
} >"$record_root/migration-result.txt"

if [[ "$activate" == "yes" ]]; then
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
        "${XDG_DATA_HOME:-$HOME/.local/share}/hollow-grove/gameplay-saves" \
        "user_gameplay_saves"
    activate_state_dir \
        "$state_stage/$state_name/payload/project-state/artifacts/gameplay-saves" \
        "$repo_path/artifacts/gameplay-saves" \
        "project_gameplay_saves"

    next_link="$current_link.next-$stamp"
    ln -s "$install_path" "$next_link"
    mv -Tf "$next_link" "$current_link"
    ln -sfn "$current_link/bin/hollow-grove-hueman" \
        "$HOME/.local/bin/hollow-grove-hueman"
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
    printf 'Hueman release staged at %s; current link is unchanged\n' "$install_path"
fi
if [[ "$activate" == "yes" ]]; then
    printf 'Hueman installed at %s and linked through %s\n' "$install_path" "$current_link"
    printf 'Durable gameplay saves were activated with recoverable backups\n'
fi
printf 'Workstation state staged for laptop adaptation at %s\n' "$state_stage/$state_name"
printf 'migration record: %s\n' "$record_root"
REMOTE_SCRIPT
