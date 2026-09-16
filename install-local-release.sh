#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
INSTALL_ROOT="${HOLLOW_GROVE_INSTALL_ROOT:-$HOME/.local/lib/hollow-grove}"
BIN_DIR="$INSTALL_ROOT/bin"

cargo build --manifest-path "$SCRIPT_DIR/Cargo.toml" --release \
    --features dev-tools \
    --bin hollow-grove \
    --bin hollow_deck \
    --bin adaptive_topology_live \
    --bin hollow_grove_runtime \
    --bin hollow_grove_niri_bridge \
    --bin hollow_grove_game_service \
    --bin hg-dev
cargo build --manifest-path "$SCRIPT_DIR/Cargo.toml" --release \
    --package hollow-niri --bin hollow-niri

install -d -m 0755 "$BIN_DIR" "$HOME/.local/bin"
for binary in \
    hollow-grove \
    hollow_deck \
    adaptive_topology_live \
    hollow_grove_runtime \
    hollow_grove_niri_bridge \
    hollow_grove_game_service \
    hg-dev \
    hollow-niri
do
    install -m 0755 "$SCRIPT_DIR/target/release/$binary" "$BIN_DIR/$binary"
done
while IFS= read -r tool; do
    ln -sfn hg-dev "$BIN_DIR/$tool"
done < <("$BIN_DIR/hg-dev" --list)
system_broker="/usr/lib/hollow-grove/bin/hollow-niri"
if [[ -x "$system_broker" ]]; then
    # Keep interactive commands on the same broker revision as the systemd
    # service installed by hollow-grove-general. The local build remains
    # directly available from $BIN_DIR for development and comparison.
    ln -sfn "$system_broker" "$HOME/.local/bin/hollow-niri"
else
    # Avoid storing a second physical copy when the General Edition package is
    # absent; expose the optimized local build through one stable PATH entry.
    ln -sfn "$BIN_DIR/hollow-niri" "$HOME/.local/bin/hollow-niri"
fi
ln -sfn "$SCRIPT_DIR/hollow-grove-update.sh" "$HOME/.local/bin/hollow-grove-update"

printf 'installed optimized Hollow Grove binaries in %s\n' "$BIN_DIR"
