#!/usr/bin/env bash
set -euo pipefail

SEED="${SEED:-0xC0FFEE42}"
HASH_CASES="${HASH_CASES:-10000}"
ARTIFACT_DIR="artifacts/verification"
mkdir -p "$ARTIFACT_DIR"

echo "== Hollow Grove adversarial verification: FAST =="
echo "seed: $SEED"

TEST_COUNT="$(cargo test -- --list | rg ': test$' | wc -l | tr -d ' ')"
GIT_HEAD="$(git rev-parse HEAD 2>/dev/null || printf 'unknown')"
WORKTREE_STATUS="$(git status --short 2>/dev/null || true)"

cargo fmt --check
cargo clippy --all-targets --all-features
cargo test
cargo build
cargo build --release
./bench-local.sh | tee "$ARTIFACT_DIR/bench-fast.log"
./stress-local.sh | tee "$ARTIFACT_DIR/stress-fast.log"
cargo run --quiet --bin client_desktop_status >/dev/null

RELEASE_HASH="$(cargo run --release --quiet --bin adversarial_verification -- hash-corpus --cases "$HASH_CASES" --seed "$SEED")"
cargo run --quiet --bin adversarial_verification -- report \
  --profile fast \
  --seed "$SEED" \
  --compare-other-label release \
  --compare-other-hash "$RELEASE_HASH" \
  --json artifacts/adversarial_verification_report.json \
  --md artifacts/adversarial_verification_report.md \
  --matrix "$ARTIFACT_DIR/exhaustive_matrix.tsv" | tee "$ARTIFACT_DIR/verify-fast-summary.log"

{
  printf 'head=%s\n' "$GIT_HEAD"
  printf 'test_count=%s\n' "$TEST_COUNT"
  printf 'seed=%s\n' "$SEED"
  printf 'release_hash=%s\n' "$RELEASE_HASH"
  printf 'working_tree_status_begin\n%s\nworking_tree_status_end\n' "$WORKTREE_STATUS"
} > "$ARTIFACT_DIR/run-context.txt"

echo "FAST verification artifacts written to:"
echo "  artifacts/adversarial_verification_report.json"
echo "  artifacts/adversarial_verification_report.md"
echo "  $ARTIFACT_DIR/exhaustive_matrix.tsv"
