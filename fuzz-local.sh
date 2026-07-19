#!/usr/bin/env bash
set -euo pipefail

PROFILE="${1:-smoke}"
SEED="${FUZZ_SEED:-${SEED:-0xC0FFEE42}}"
JOBS="${FUZZ_JOBS:-1}"
ARTIFACT_DIR="artifacts/verification/fuzz"
SUMMARY_MD="artifacts/verification/fuzz_summary.md"
SUMMARY_JSON="artifacts/verification/fuzz_summary.json"
PATH="/home/warren/.cargo/bin:$PATH"
ASAN_OPTIONS="${ASAN_OPTIONS:-detect_leaks=0}"
LSAN_OPTIONS="${LSAN_OPTIONS:-detect_leaks=0}"
RUN_STAMP="$(date +%Y%m%dT%H%M%S)"
GIT_HEAD="$(git rev-parse HEAD 2>/dev/null || echo unknown)"
DIRTY_WORKTREE="false"
if [[ -n "$(git status --short 2>/dev/null || true)" ]]; then
  DIRTY_WORKTREE="true"
fi
HOST_PLATFORM="$(uname -srvmo 2>/dev/null || uname -a)"

case "$PROFILE" in
  smoke) DEFAULT_SECONDS=60 ;;
  development) DEFAULT_SECONDS=900 ;;
  verification) DEFAULT_SECONDS=7200 ;;
  overnight) DEFAULT_SECONDS=28800 ;;
  *)
    echo "usage: ./fuzz-local.sh [smoke|development|verification|overnight]"
    exit 1
    ;;
esac

SECONDS_PER_TARGET="${FUZZ_SECONDS:-$DEFAULT_SECONDS}"
mkdir -p "$ARTIFACT_DIR"

if ! cargo fuzz --version >/dev/null 2>&1; then
  echo "cargo-fuzz is unavailable; install it before running fuzz-local.sh"
  exit 1
fi

if ! /home/warren/.cargo/bin/rustup toolchain list | rg -q '^nightly'; then
  echo "nightly toolchain is unavailable; install it before running fuzz-local.sh"
  exit 1
fi

CARGO_FUZZ_VERSION="$(cargo fuzz --version | head -n 1)"
NIGHTLY_VERSION="$(/home/warren/.cargo/bin/rustup run nightly rustc --version | head -n 1)"

TARGETS=(
  decision_input
  decision_trace_replay
  recipe_compiler
  snapshot_boundaries
)

if [[ -n "${FUZZ_TARGET:-}" ]]; then
  TARGETS=("${FUZZ_TARGET}")
fi

{
  echo "# Hollow Grove Fuzz Summary"
  echo
  echo "- profile: \`$PROFILE\`"
  echo "- seconds per target: \`$SECONDS_PER_TARGET\`"
  echo "- jobs: \`$JOBS\`"
  echo "- seed: \`$SEED\`"
  echo
} > "$SUMMARY_MD"

echo "== Hollow Grove fuzz-local =="
echo "profile: $PROFILE"
echo "seconds per target: $SECONDS_PER_TARGET"
echo "seed: $SEED"

TARGET_JSON=""
RUN_SUCCESSFUL=true
FAILED_TARGET=""

for target in "${TARGETS[@]}"; do
  if [[ ! -f "fuzz/fuzz_targets/${target}.rs" ]]; then
    echo "missing fuzz target: $target"
    exit 1
  fi

  CORPUS_DIR="fuzz/corpus/$target"
  CRASH_DIR="$ARTIFACT_DIR/crashes/$RUN_STAMP/$target"
  LOG_PATH="$ARTIFACT_DIR/${RUN_STAMP}_${target}.log"
  mkdir -p "$CORPUS_DIR" "$CRASH_DIR"

  echo "running target: $target"
  CMD=(
    env
    "ASAN_OPTIONS=$ASAN_OPTIONS"
    "LSAN_OPTIONS=$LSAN_OPTIONS"
    cargo +nightly fuzz run "$target" "$CORPUS_DIR" --
    -artifact_prefix="$CRASH_DIR/"
    -max_total_time="$SECONDS_PER_TARGET"
    -print_final_stats=1
    -jobs="$JOBS"
    -workers="$JOBS"
    -seed="$SEED"
  )

  set +e
  "${CMD[@]}" 2>&1 | tee "$LOG_PATH"
  STATUS=${PIPESTATUS[0]}
  set -e

  EXECUTIONS="$(rg -o 'stat::number_of_executed_units: [0-9]+' "$LOG_PATH" | tail -1 | awk '{print $2}' || true)"
  if [[ -z "$EXECUTIONS" ]]; then
    EXECUTIONS="unavailable"
  fi
  CORPUS_SIZE="$(find "$CORPUS_DIR" -type f | wc -l | tr -d ' ')"
  CRASH_COUNT="$(find "$CRASH_DIR" -type f | wc -l | tr -d ' ')"

  {
    echo "- \`$target\`: duration \`${SECONDS_PER_TARGET}s\`, executions \`${EXECUTIONS}\`, corpus \`${CORPUS_SIZE}\`, crashes \`${CRASH_COUNT}\`, status \`${STATUS}\`, log \`$LOG_PATH\`, crashes dir \`$CRASH_DIR\`"
  } >> "$SUMMARY_MD"

  TARGET_JSON+="${TARGET_JSON:+,}
    {\"name\":\"$target\",\"duration_seconds\":$SECONDS_PER_TARGET,\"executions\":\"$EXECUTIONS\",\"corpus_size\":$CORPUS_SIZE,\"crashes\":$CRASH_COUNT,\"status\":$STATUS,\"result\":\"$([[ "$STATUS" -eq 0 ]] && echo passed || echo failed)\",\"log_path\":\"$LOG_PATH\",\"crashes_dir\":\"$CRASH_DIR\"}"

  if [[ "$STATUS" -ne 0 ]]; then
    RUN_SUCCESSFUL=false
    FAILED_TARGET="$target"
    break
  fi
done

cat > "$SUMMARY_JSON" <<EOF
{
  "profile": "$PROFILE",
  "seconds_per_target": $SECONDS_PER_TARGET,
  "jobs": $JOBS,
  "seed": "$SEED",
  "run_stamp": "$RUN_STAMP",
  "git_head": "$GIT_HEAD",
  "dirty_worktree": $DIRTY_WORKTREE,
  "host_platform": "$HOST_PLATFORM",
  "cargo_fuzz_version": "$CARGO_FUZZ_VERSION",
  "nightly_version": "$NIGHTLY_VERSION",
  "targets": [
$TARGET_JSON
  ],
  "completed": true,
  "successful": $RUN_SUCCESSFUL,
  "failed_target": $([[ -n "$FAILED_TARGET" ]] && printf '"%s"' "$FAILED_TARGET" || printf 'null')
}
EOF

if [[ "$RUN_SUCCESSFUL" != true ]]; then
  echo "fuzz target failed: $FAILED_TARGET"
  echo "summary: $SUMMARY_JSON"
  exit 1
fi

echo
echo "fuzz summary: $SUMMARY_MD"
