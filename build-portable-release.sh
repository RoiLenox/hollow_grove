#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
OUTPUT_ROOT="${OUTPUT_ROOT:-$SCRIPT_DIR/dist}"
VERSION="$(sed -n 's/^version = "\([^"]*\)"/\1/p' "$SCRIPT_DIR/Cargo.toml" | head -n1)"
SOURCE_DATE_EPOCH="${SOURCE_DATE_EPOCH:-$(git -C "$SCRIPT_DIR" log -1 --format=%ct)}"
STAGING_ROOT="$OUTPUT_ROOT/.staging"
HUEMAN_ROOT="$STAGING_ROOT/hollow-grove-hueman-$VERSION"
GENERAL_ROOT="$STAGING_ROOT/hollow-grove-general-$VERSION"
PCK_PATH="$STAGING_ROOT/hollow-grove-hueman.pck"

if [[ -z "$VERSION" || "$OUTPUT_ROOT" == "/" || "$STAGING_ROOT" == "/" ]]; then
    printf 'refusing unsafe or versionless release path\n' >&2
    exit 2
fi

for command_name in cargo git godot install python3 sha256sum tar gzip; do
    if ! command -v "$command_name" >/dev/null 2>&1; then
        printf 'release build requires %s\n' "$command_name" >&2
        exit 127
    fi
done

cd "$SCRIPT_DIR"
./verify-art-assets.sh
cargo build --release --locked --offline \
    --bin hollow-grove \
    --bin hollow_grove_game_service \
    --bin hollow_grove_runtime \
    --bin hollow_grove_niri_bridge \
    --bin hollow_grove_semantic

mkdir -p "$OUTPUT_ROOT"
if [[ -d "$STAGING_ROOT" ]]; then
    rm -rf -- "$STAGING_ROOT"
fi
mkdir -p "$HUEMAN_ROOT/bin" \
    "$HUEMAN_ROOT/lib/hollow-grove/hueman" \
    "$HUEMAN_ROOT/share/hollow-grove-hueman" \
    "$HUEMAN_ROOT/share/doc/hollow-grove-hueman" \
    "$HUEMAN_ROOT/share/licenses/hollow-grove-hueman"

godot --headless --path hueman_godot --export-pack Linux "$PCK_PATH"
install -m755 packaging/hollow-grove-hueman/hollow-grove-hueman \
    "$HUEMAN_ROOT/bin/hollow-grove-hueman"
install -m755 packaging/hollow-grove-hueman/launch \
    "$HUEMAN_ROOT/lib/hollow-grove/hueman/launch"
install -m755 target/release/hollow_grove_game_service \
    "$HUEMAN_ROOT/lib/hollow-grove/hueman/hollow_grove_game_service"
install -m644 "$PCK_PATH" \
    "$HUEMAN_ROOT/share/hollow-grove-hueman/hollow-grove-hueman.pck"
install -m644 packaging/hollow-grove-hueman/README.md \
    "$HUEMAN_ROOT/share/doc/hollow-grove-hueman/README.md"
install -m644 LICENSE "$HUEMAN_ROOT/share/licenses/hollow-grove-hueman/LICENSE"

mkdir -p "$GENERAL_ROOT"
cp -a packaging/hollow-grove-general/etc packaging/hollow-grove-general/usr "$GENERAL_ROOT/"
mkdir -p "$GENERAL_ROOT/usr/share/doc/hollow-grove-general" \
    "$GENERAL_ROOT/usr/share/licenses/hollow-grove-general"
install -m644 packaging/hollow-grove-general/PACKAGE_MANIFEST.md \
    packaging/hollow-grove-general/OPTIONAL_APPLICATIONS.md \
    packaging/hollow-grove-general/SANITIZATION_REPORT.md \
    packaging/hollow-grove-general/ISO_READINESS.md \
    "$GENERAL_ROOT/usr/share/doc/hollow-grove-general/"
install -m644 LICENSE "$GENERAL_ROOT/usr/share/licenses/hollow-grove-general/LICENSE"

commit_id="$(git rev-parse HEAD)"
for release_root in "$HUEMAN_ROOT" "$GENERAL_ROOT"; do
    release_name="$(basename -- "$release_root")"
    {
        printf 'name=%s\n' "$release_name"
        printf 'version=%s\n' "$VERSION"
        printf 'git_commit=%s\n' "$commit_id"
        printf 'source_date_epoch=%s\n' "$SOURCE_DATE_EPOCH"
        printf 'rust_toolchain=%s\n' "$(rustc --version)"
        printf 'godot_runtime=%s\n' "$(godot --version)"
    } >"$release_root/BUILD-MANIFEST.txt"
    (
        cd "$release_root"
        find . -type f ! -name SHA256SUMS -print0 \
            | sort -z \
            | xargs -0 sha256sum >SHA256SUMS
    )
    tar --sort=name --mtime="@$SOURCE_DATE_EPOCH" --owner=0 --group=0 --numeric-owner \
        -C "$STAGING_ROOT" -cf - "$release_name" \
        | gzip -n >"$OUTPUT_ROOT/$release_name.tar.gz"
done

(
    cd "$OUTPUT_ROOT"
    sha256sum "hollow-grove-hueman-$VERSION.tar.gz" \
        "hollow-grove-general-$VERSION.tar.gz" >SHA256SUMS
)

printf 'portable release ready: %s\n' "$OUTPUT_ROOT"
