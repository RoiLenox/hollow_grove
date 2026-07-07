#!/usr/bin/env bash

set -euo pipefail

cd /home/warren/hollow-grove

cargo fmt
cargo test
cargo run --quiet
