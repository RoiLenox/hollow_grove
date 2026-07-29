#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="${HOLLOW_GROVE_ROOT:-${SCRIPT_DIR}}"

cd "${ROOT_DIR}"
cargo build --bin hollow-grove --bin hollow_grove_runtime --bin hollow_grove_niri_bridge >/dev/null

exec target/debug/hollow-grove desktop "$@"
