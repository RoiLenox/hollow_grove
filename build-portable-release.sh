#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
OUTPUT_ROOT="${OUTPUT_ROOT:-$SCRIPT_DIR/dist}"
VERSION="$(sed -n 's/^version = "\([^"]*\)"/\1/p' "$SCRIPT_DIR/Cargo.toml" | head -n1)"
SOURCE_DATE_EPOCH="${SOURCE_DATE_EPOCH:-$(git -C "$SCRIPT_DIR" log -1 --format=%ct)}"
STAGING_ROOT="$OUTPUT_ROOT/.staging"
CORE_ROOT="$STAGING_ROOT/hollow-grove-core-$VERSION"
HUEMAN_ROOT="$STAGING_ROOT/hueman-$VERSION"
GENERAL_ROOT="$STAGING_ROOT/hollow-grove-general-$VERSION"
PCK_PATH="$STAGING_ROOT/hueman.pck"

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
if [[ -n "$(git status --porcelain=v1 --untracked-files=normal)" ]]; then
    printf '%s\n' \
        'portable release requires a clean worktree so BUILD-MANIFEST.txt identifies the complete source' >&2
    exit 2
fi
./verify-art-assets.sh
cargo build --release --locked --offline \
    --package hollow-grove \
    --bin hollow_grove_game_service
cargo build --release --locked --offline \
    --package hollow-niri \
    --bin hollow-niri

mkdir -p "$OUTPUT_ROOT"
rm -f -- "$OUTPUT_ROOT/hollow-grove-hueman-$VERSION.tar.gz"
if [[ -d "$STAGING_ROOT" ]]; then
    rm -rf -- "$STAGING_ROOT"
fi

mkdir -p "$CORE_ROOT/bin" \
    "$CORE_ROOT/lib/hollow-grove/core" \
    "$CORE_ROOT/share/doc/hollow-grove-core" \
    "$CORE_ROOT/share/licenses/hollow-grove-core"
install -m755 packaging/hollow-grove-core/hollow-grove-service \
    "$CORE_ROOT/bin/hollow-grove-service"
install -m755 packaging/hollow-grove-core/hollow-grove-with-hueman \
    "$CORE_ROOT/bin/hollow-grove-with-hueman"
install -m755 packaging/hollow-grove-core/hollow-grove-device \
    "$CORE_ROOT/bin/hollow-grove-device"
install -m644 packaging/hollow-grove-core/shared_session.py \
install -m644 packaging/hollow-grove-core/device.py \
    packaging/hollow-grove-core/device-launch.sh "$CORE_ROOT/lib/hollow-grove/"
ln -s hollow-grove-with-hueman "$CORE_ROOT/bin/hollow-grove-hueman"
install -m755 target/release/hollow_grove_game_service \
    "$CORE_ROOT/lib/hollow-grove/core/hollow_grove_game_service"
install -m644 packaging/hollow-grove-core/README.md \
    docs/HOLLOW_GROVE_HUEMAN_SEAM_V1.md \
    "$CORE_ROOT/share/doc/hollow-grove-core/"
install -m644 LICENSE "$CORE_ROOT/share/licenses/hollow-grove-core/LICENSE"

mkdir -p "$HUEMAN_ROOT/bin" \
    "$HUEMAN_ROOT/lib/hueman" \
    "$HUEMAN_ROOT/share/hueman" \
    "$HUEMAN_ROOT/share/doc/hueman" \
    "$HUEMAN_ROOT/share/licenses/hueman"
godot --headless --path hueman_godot --export-pack Linux "$PCK_PATH"
install -m755 packaging/hueman/hueman "$HUEMAN_ROOT/bin/hueman"
install -m755 packaging/hueman/launch "$HUEMAN_ROOT/lib/hueman/launch"
install -m644 "$PCK_PATH" "$HUEMAN_ROOT/share/hueman/hueman.pck"
install -m644 packaging/hueman/README.md \
    docs/HOLLOW_GROVE_HUEMAN_SEAM_V1.md \
    "$HUEMAN_ROOT/share/doc/hueman/"
install -m644 LICENSE "$HUEMAN_ROOT/share/licenses/hueman/LICENSE"

mkdir -p "$GENERAL_ROOT"
cp -a packaging/hollow-grove-general/etc packaging/hollow-grove-general/usr "$GENERAL_ROOT/"
install -m755 target/release/hollow-niri \
    "$GENERAL_ROOT/usr/lib/hollow-grove/bin/hollow-niri"
mkdir -p "$GENERAL_ROOT/usr/share/doc/hollow-grove-general" \
    "$GENERAL_ROOT/usr/share/licenses/hollow-grove-general"
install -m644 packaging/hollow-grove-general/PACKAGE_MANIFEST.md \
    packaging/hollow-grove-general/OPTIONAL_APPLICATIONS.md \
    packaging/hollow-grove-general/SANITIZATION_REPORT.md \
    packaging/hollow-grove-general/ISO_READINESS.md \
    "$GENERAL_ROOT/usr/share/doc/hollow-grove-general/"
install -m644 LICENSE "$GENERAL_ROOT/usr/share/licenses/hollow-grove-general/LICENSE"

if [[ ! -x "$CORE_ROOT/lib/hollow-grove/core/hollow_grove_game_service" \
    || -e "$CORE_ROOT/share/hueman" \
    || ! -f "$HUEMAN_ROOT/share/hueman/hueman.pck" \
    || -e "$HUEMAN_ROOT/lib/hollow-grove/core/hollow_grove_game_service" \
    || ! -x "$GENERAL_ROOT/usr/lib/hollow-grove/bin/hollow-niri" ]]; then
    printf 'release separation or required-payload invariant failed\n' >&2
    exit 1
fi

# The two-device installer exposes release commands through two levels of
# symlinks (`~/.local/bin` -> `current` -> versioned release). Probe that exact
# shape so launchers cannot accidentally resolve payloads relative to
# `~/.local` instead of their independent release roots.
LAUNCHER_PROBE_ROOT="$STAGING_ROOT/launcher-probe"
TRUE_BIN="$(type -P true)"
mkdir -p "$LAUNCHER_PROBE_ROOT"
ln -s "$CORE_ROOT/bin/hollow-grove-service" \
    "$LAUNCHER_PROBE_ROOT/hollow-grove-service"
ln -s "$CORE_ROOT/bin/hollow-grove-with-hueman" \
    "$LAUNCHER_PROBE_ROOT/hollow-grove-with-hueman"
ln -s "$HUEMAN_ROOT/bin/hueman" "$LAUNCHER_PROBE_ROOT/hueman"
"$LAUNCHER_PROBE_ROOT/hollow-grove-service" --help >/dev/null
GODOT_BIN="$TRUE_BIN" "$LAUNCHER_PROBE_ROOT/hueman"
if HOLLOW_GROVE_PORT=0 HUEMAN_BIN="$TRUE_BIN" \
    "$LAUNCHER_PROBE_ROOT/hollow-grove-with-hueman" \
    >/dev/null 2>"$LAUNCHER_PROBE_ROOT/integration-probe.log"; then
    printf 'integration launcher unexpectedly accepted port zero\n' >&2
    exit 1
fi
if ! grep -Fq 'invalid Hollow Grove port: 0' \
    "$LAUNCHER_PROBE_ROOT/integration-probe.log"; then
    printf 'release launcher symlink probe failed\n' >&2
    sed -n '1,20p' "$LAUNCHER_PROBE_ROOT/integration-probe.log" >&2
    exit 1
fi
# Prefer the installed presentation client even when an older command with the
# same name precedes it on PATH. Exercise the actual service and symlink layout
# with disposable state; Godot itself is replaced by a successful no-op here.
mkdir -p "$LAUNCHER_PROBE_ROOT/.local/share/hueman" \
    "$LAUNCHER_PROBE_ROOT/.local/share/hollow-grove" \
    "$LAUNCHER_PROBE_ROOT/.local/bin" \
    "$LAUNCHER_PROBE_ROOT/shadow-bin"
ln -s "$HUEMAN_ROOT" "$LAUNCHER_PROBE_ROOT/.local/share/hueman/current"
ln -s "$CORE_ROOT" "$LAUNCHER_PROBE_ROOT/.local/share/hollow-grove/current-core"
ln -s "$LAUNCHER_PROBE_ROOT/.local/share/hollow-grove/current-core/bin/hollow-grove-with-hueman" \
    "$LAUNCHER_PROBE_ROOT/.local/bin/hollow-grove-with-hueman"
cat >"$LAUNCHER_PROBE_ROOT/shadow-bin/hueman" <<'SHADOW_HUEMAN'
#!/usr/bin/env bash
printf 'unexpected PATH Hueman selected\n' >&2
exit 73
SHADOW_HUEMAN
chmod 755 "$LAUNCHER_PROBE_ROOT/shadow-bin/hueman"
probe_port="$(python3 - <<'PROBE_PORT'
import socket
with socket.socket() as probe:
    probe.bind(('127.0.0.1', 0))
    print(probe.getsockname()[1])
PROBE_PORT
)"
if ! env -u HUEMAN_BIN \
    PATH="$LAUNCHER_PROBE_ROOT/shadow-bin:$PATH" \
    XDG_DATA_HOME="$LAUNCHER_PROBE_ROOT/.local/share" \
    XDG_STATE_HOME="$LAUNCHER_PROBE_ROOT/.local/state" \
    GODOT_BIN="$TRUE_BIN" HOLLOW_GROVE_PORT="$probe_port" \
    "$LAUNCHER_PROBE_ROOT/.local/bin/hollow-grove-with-hueman" \
    >"$LAUNCHER_PROBE_ROOT/client-selection-probe.log" 2>&1; then
    printf 'installed Hueman client selection probe failed\n' >&2
    sed -n '1,40p' "$LAUNCHER_PROBE_ROOT/client-selection-probe.log" >&2
    exit 1
fi
printf 'installed Hueman client selection probe passed\n'
rm -rf -- "$LAUNCHER_PROBE_ROOT"

commit_id="$(git rev-parse HEAD)"
for release_root in "$CORE_ROOT" "$HUEMAN_ROOT" "$GENERAL_ROOT"; do
    release_name="$(basename -- "$release_root")"
    {
        printf 'name=%s\n' "$release_name"
        printf 'version=%s\n' "$VERSION"
        printf 'git_commit=%s\n' "$commit_id"
        printf 'source_date_epoch=%s\n' "$SOURCE_DATE_EPOCH"
        printf 'rust_toolchain=%s\n' "$(rustc --version)"
        printf 'godot_build_tool=%s\n' "$(godot --version)"
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
    sha256sum "hollow-grove-core-$VERSION.tar.gz" \
        "hueman-$VERSION.tar.gz" \
        "hollow-grove-general-$VERSION.tar.gz" >SHA256SUMS
)

printf 'independent Hollow Grove Core, Hueman, and General releases ready: %s\n' \
    "$OUTPUT_ROOT"
