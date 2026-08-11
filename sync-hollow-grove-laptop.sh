#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
SSH_HOST=""
REMOTE_REPO=""
RELEASE_REF="foundation-v1-rc1"
ACTIVATE="no"
VERSION="$(sed -n 's/^version = "\([^"]*\)"/\1/p' "$SCRIPT_DIR/Cargo.toml" | head -n1)"
TRANSFER_ROOT=""

usage() {
    printf '%s\n' \
        "usage: $0 --host USER@HOST --remote-repo /absolute/path [options]" \
        "" \
        "Options:" \
        "  --ref REF       source ref to transfer (default: foundation-v1-rc1)" \
        "  --activate      atomically replace the old checkout after verification" \
        "  --stage-only    preserve and verify side-by-side without switching (default)"
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
if [[ ! -f "$SCRIPT_DIR/dist/hollow-grove-hueman-$VERSION.tar.gz" \
    || ! -f "$SCRIPT_DIR/dist/hollow-grove-general-$VERSION.tar.gz" \
    || ! -f "$SCRIPT_DIR/dist/SHA256SUMS" ]]; then
    printf 'portable release is missing; run ./build-portable-release.sh first\n' >&2
    exit 2
fi

release_commit="$(git -C "$SCRIPT_DIR" rev-parse "$RELEASE_REF^{commit}")"
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
    "$TRANSFER_ROOT/"

remote_inbox="$(ssh "$SSH_HOST" 'mktemp -d --tmpdir hollow-grove-incoming.XXXXXX')"
if [[ ! "$remote_inbox" =~ ^/tmp/[A-Za-z0-9._/-]+$ ]]; then
    printf 'remote host returned an unsafe temporary path: %s\n' "$remote_inbox" >&2
    exit 1
fi
scp "$TRANSFER_ROOT"/* "$SSH_HOST:$remote_inbox/"

ssh "$SSH_HOST" bash -s -- \
    "$REMOTE_REPO" "$RELEASE_REF" "$release_commit" "$VERSION" "$ACTIVATE" "$remote_inbox" <<'REMOTE_SCRIPT'
set -euo pipefail

repo_path="$1"
release_ref="$2"
release_commit="$3"
version="$4"
activate="$5"
incoming="$6"
stamp="$(date -u +%Y%m%dT%H%M%SZ)"
repo_parent="$(dirname -- "$repo_path")"
repo_name="$(basename -- "$repo_path")"
staged_path="$repo_parent/${repo_name}.incoming-$stamp"
backup_path="$repo_parent/${repo_name}.pre-foundation-v1-$stamp"
record_root="$repo_parent/hollow-grove-migration-records/$stamp"
install_parent="${XDG_DATA_HOME:-$HOME/.local/share}/hollow-grove/releases"
install_path="$install_parent/hueman-$version-$release_commit"
current_link="${XDG_DATA_HOME:-$HOME/.local/share}/hollow-grove/current-hueman"

cleanup_incoming() {
    if [[ -d "$incoming" ]]; then
        rm -rf -- "$incoming"
    fi
}
trap cleanup_incoming EXIT INT TERM

if [[ ! -d "$repo_path/.git" ]]; then
    printf 'remote path is not a Git checkout: %s\n' "$repo_path" >&2
    exit 1
fi
if [[ -e "$staged_path" || -e "$backup_path" || -e "$install_path" ]]; then
    printf 'migration target already exists; refusing to overwrite it\n' >&2
    exit 1
fi

mkdir -p "$record_root"
git -C "$repo_path" rev-parse HEAD >"$record_root/old-head.txt"
git -C "$repo_path" status --porcelain=v1 --branch >"$record_root/old-status.txt"
git -C "$repo_path" diff --binary >"$record_root/old-working-tree.patch"
git -C "$repo_path" diff --cached --binary >"$record_root/old-index.patch"
git -C "$repo_path" bundle create "$record_root/old-history.bundle" --all
git -C "$repo_path" ls-files --others --exclude-standard -z \
    >"$record_root/old-untracked-files.list"
tar -C "$repo_path" --null -czf "$record_root/old-untracked-files.tar.gz" \
    -T "$record_root/old-untracked-files.list"

git clone --no-local "$incoming/hollow-grove.bundle" "$staged_path"
git -C "$staged_path" checkout --detach "$release_commit"
if [[ "$(git -C "$staged_path" rev-parse HEAD)" != "$release_commit" ]]; then
    printf 'staged checkout did not resolve to the transferred release commit\n' >&2
    exit 1
fi

(
    cd "$staged_path"
    ./verify-art-assets.sh
    cargo fmt --all -- --check
    cargo test --workspace --locked
)

(
    cd "$incoming"
    sha256sum --check SHA256SUMS
)
mkdir -p "$install_parent" "$(dirname -- "$current_link")" "$HOME/.local/bin"
mkdir -p "$install_path"
tar -xzf "$incoming/hollow-grove-hueman-$version.tar.gz" \
    --strip-components=1 -C "$install_path"
(
    cd "$install_path"
    sha256sum --check SHA256SUMS
)

next_link="$current_link.next-$stamp"
ln -s "$install_path" "$next_link"
mv -Tf "$next_link" "$current_link"
ln -sfn "$current_link/bin/hollow-grove-hueman" \
    "$HOME/.local/bin/hollow-grove-hueman"

{
    printf 'release_ref=%s\n' "$release_ref"
    printf 'release_commit=%s\n' "$release_commit"
    printf 'staged_checkout=%s\n' "$staged_path"
    printf 'old_checkout=%s\n' "$repo_path"
    printf 'backup_record=%s\n' "$record_root"
    printf 'hueman_install=%s\n' "$install_path"
} >"$record_root/migration-result.txt"

if [[ "$activate" == "yes" ]]; then
    mv "$repo_path" "$backup_path"
    if ! mv "$staged_path" "$repo_path"; then
        mv "$backup_path" "$repo_path"
        printf 'activation failed; original checkout was restored\n' >&2
        exit 1
    fi
    {
        printf 'activated_checkout=%s\n' "$repo_path"
        printf 'recoverable_old_checkout=%s\n' "$backup_path"
    } >>"$record_root/migration-result.txt"
    printf 'activated %s at %s; prior checkout preserved at %s\n' \
        "$release_ref" "$repo_path" "$backup_path"
else
    printf 'staged and verified %s at %s; original checkout remains active\n' \
        "$release_ref" "$staged_path"
fi
printf 'Hueman installed at %s and linked through %s\n' "$install_path" "$current_link"
printf 'migration record: %s\n' "$record_root"
REMOTE_SCRIPT
