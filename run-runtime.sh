#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="${HOLLOW_GROVE_ROOT:-${SCRIPT_DIR}}"

cd "${ROOT_DIR}"

cargo build --bin hollow-grove --bin hollow_grove_runtime >/dev/null

if [[ $# -eq 0 ]]; then
    exec target/debug/hollow-grove runtime --cycles 5 --interval-ms 1000
fi

exec target/debug/hollow-grove runtime "$@"
