#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
INSTALL_ROOT="${HOLLOW_GROVE_INSTALL_ROOT:-$HOME/.local/lib/hollow-grove}"
BIN_DIR="$INSTALL_ROOT/bin"

cargo build --manifest-path "$SCRIPT_DIR/Cargo.toml" --release \
    --features dev-tools \
    --bin hollow-grove \
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
install -m 0755 "$SCRIPT_DIR/target/release/hollow-niri" "$HOME/.local/bin/hollow-niri"

printf 'installed optimized Hollow Grove binaries in %s\n' "$BIN_DIR"
