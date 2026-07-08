#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="${HOLLOW_GROVE_ROOT:-/home/warren/hollow-grove}"

cd "${ROOT_DIR}"
cargo build --bin hollow-grove --bin hollow_grove_runtime --bin hollow_grove_niri_bridge >/dev/null

exec target/debug/hollow-grove desktop "$@"
