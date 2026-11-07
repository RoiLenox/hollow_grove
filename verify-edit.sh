#!/usr/bin/env bash
# Ordinary edits: source conformance and default runtime compilation.
# Optional arguments are passed to the integration test target as a filter.
set -euo pipefail
cd -- "$(dirname -- "${BASH_SOURCE[0]}")"
cargo fmt --all -- --check
python3 tools/source_boundary.py
python3 tools/build_registrar.py --check
python3 tools/verify-active-geography.py
cargo check --workspace --lib --bins --locked --offline
if (( $# )); then
    cargo test --locked --offline --test integration "$@"
fi
