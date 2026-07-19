#!/usr/bin/env bash
set -euo pipefail

SEED="${SEED:-0xC0FFEE42}"
HASH_CASES="${HASH_CASES:-100000}"
ARTIFACT_DIR="artifacts/verification"
mkdir -p "$ARTIFACT_DIR"

echo "== Hollow Grove adversarial verification: OVERNIGHT =="
echo "seed: $SEED"

./verify-full.sh
./fuzz-local.sh overnight
./soak-local.sh twenty-four-hour
./mutation-local.sh

RELEASE_HASH="$(cargo run --release --quiet --bin adversarial_verification -- hash-corpus --cases "$HASH_CASES" --seed "$SEED")"
cargo run --quiet --bin adversarial_verification -- report \
  --profile overnight \
  --seed "$SEED" \
  --compare-other-label release \
  --compare-other-hash "$RELEASE_HASH" \
  --json artifacts/adversarial_verification_report.json \
  --md artifacts/adversarial_verification_report.md \
  --matrix "$ARTIFACT_DIR/exhaustive_matrix.tsv" | tee "$ARTIFACT_DIR/verify-overnight-summary.log"

echo "OVERNIGHT verification complete."
