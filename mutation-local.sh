#!/usr/bin/env bash
set -euo pipefail

if command -v cargo-mutants >/dev/null 2>&1; then
  echo "cargo-mutants detected."
  echo "No mutation manifest is bundled yet; target decision_engine, landing, synthesis_recipe, synthesis_execution, and replay logic."
  exit 0
fi

if cargo mutest --help >/dev/null 2>&1; then
  echo "cargo-mutest detected."
  echo "No mutation scenario is bundled yet; target decision_engine, landing, synthesis_recipe, synthesis_execution, and replay logic."
  exit 0
fi

echo "mutation-local.sh: no supported mutation tool is installed in this workspace."
echo "Recommended next step: install cargo-mutants or cargo-mutest and target decision_engine, landing, synthesis_recipe, synthesis_execution, and replay logic."
