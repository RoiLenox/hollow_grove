#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="${HOLLOW_GROVE_ROOT:-/home/warren/hollow-grove}"

cd "${ROOT_DIR}"

cargo build --bin hollow-grove --bin hollow_grove_runtime >/dev/null

if [[ $# -eq 0 ]]; then
    exec target/debug/hollow-grove runtime --cycles 5 --interval-ms 1000
fi

exec target/debug/hollow-grove runtime "$@"
