#!/usr/bin/env bash
# Full source integration, without release fuzz/soak/benchmark workloads.
set -euo pipefail
cd -- "$(dirname -- "${BASH_SOURCE[0]}")"
cargo fmt --all -- --check
python3 tools/build_registrar.py --check
python3 tools/verify-active-geography.py
python3 tools/verify-backend-contracts.py
for suite in tools/test_*.py; do python3 "$suite"; done
./verify-art-assets.sh
packaging/hollow-grove-general/verify-keyboard-grove.sh
cargo clippy --workspace --all-targets --all-features --locked --offline -- -D warnings
cargo test --workspace --locked --offline
cargo test --workspace --all-features --locked --offline
./verify-godot.sh
./verify-playable-lifecycle.sh
python3 tools/verify_hueman_lifecycle.py
