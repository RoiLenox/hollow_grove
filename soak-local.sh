#!/usr/bin/env bash
set -euo pipefail

PROFILE="${1:-smoke}"
SEED="${SOAK_SEED:-${SEED:-0xC0FFEE42}}"
REPORT_INTERVAL="${SOAK_REPORT_INTERVAL:-60}"
ARTIFACT_DIR="artifacts/verification"
JSON_PATH="$ARTIFACT_DIR/soak_report.json"
MD_PATH="$ARTIFACT_DIR/soak_report.md"
LOG_PATH="$ARTIFACT_DIR/soak.log"
CHECKPOINT_JSON="$ARTIFACT_DIR/soak_checkpoint.json"
CHECKPOINT_MD="$ARTIFACT_DIR/soak_checkpoint.md"
SUMMARY_JSON="$ARTIFACT_DIR/soak_summary.json"
GIT_HEAD="$(git rev-parse HEAD 2>/dev/null || echo unknown)"
DIRTY_WORKTREE="false"
if [[ -n "$(git status --short 2>/dev/null || true)" ]]; then
  DIRTY_WORKTREE="true"
fi
HOST_PLATFORM="$(uname -srvmo 2>/dev/null || uname -a)"

case "$PROFILE" in
  smoke) DEFAULT_SECONDS=300 ;;
  one-hour) DEFAULT_SECONDS=3600 ;;
  eight-hour) DEFAULT_SECONDS=28800 ;;
  twenty-four-hour) DEFAULT_SECONDS=86400 ;;
  *)
    echo "usage: ./soak-local.sh [smoke|one-hour|eight-hour|twenty-four-hour]"
    exit 1
    ;;
esac

DURATION_SECONDS="${SOAK_SECONDS:-$DEFAULT_SECONDS}"
mkdir -p "$ARTIFACT_DIR"

echo "== Hollow Grove soak-local =="
echo "profile: $PROFILE"
echo "duration seconds: $DURATION_SECONDS"
echo "seed: $SEED"
echo "report interval seconds: $REPORT_INTERVAL"

cargo build --release

set +e
target/release/adversarial_verification soak \
  --duration-seconds "$DURATION_SECONDS" \
  --seed "$SEED" \
  --report-interval-seconds "$REPORT_INTERVAL" \
  --json "$JSON_PATH" \
  --md "$MD_PATH" \
  --checkpoint-json "$CHECKPOINT_JSON" \
  --checkpoint-md "$CHECKPOINT_MD" 2>&1 | tee "$LOG_PATH"
STATUS=${PIPESTATUS[0]}
set -e

python - <<'PY' "$SUMMARY_JSON" "$PROFILE" "$DURATION_SECONDS" "$REPORT_INTERVAL" "$SEED" "$GIT_HEAD" "$DIRTY_WORKTREE" "$HOST_PLATFORM" "$STATUS" "$JSON_PATH" "$MD_PATH" "$LOG_PATH" "$CHECKPOINT_JSON" "$CHECKPOINT_MD"
import json
import os
import sys

(
    summary_path,
    profile,
    requested_seconds,
    report_interval,
    seed,
    git_head,
    dirty_worktree,
    host_platform,
    status,
    json_path,
    md_path,
    log_path,
    checkpoint_json,
    checkpoint_md,
) = sys.argv[1:]

payload = {
    "profile": profile,
    "duration_requested_seconds": int(requested_seconds),
    "report_interval_seconds": int(report_interval),
    "seed": seed,
    "git_head": git_head,
    "dirty_worktree": dirty_worktree.lower() == "true",
    "host_platform": host_platform,
    "status": int(status),
    "successful": int(status) == 0,
    "report_json_path": json_path,
    "report_md_path": md_path,
    "log_path": log_path,
    "checkpoint_json_path": checkpoint_json,
    "checkpoint_md_path": checkpoint_md,
}

if os.path.exists(json_path):
    with open(json_path, "r", encoding="utf-8") as handle:
        payload["report"] = json.load(handle)

with open(summary_path, "w", encoding="utf-8") as handle:
    json.dump(payload, handle, indent=2)
    handle.write("\n")
PY

if [[ "$STATUS" -ne 0 ]]; then
  echo "soak failed or was interrupted; preserved artifacts:"
  echo "  $LOG_PATH"
  echo "  $CHECKPOINT_JSON"
  echo "  $CHECKPOINT_MD"
  exit "$STATUS"
fi

echo "soak artifacts:"
echo "  $JSON_PATH"
echo "  $MD_PATH"
echo "  $LOG_PATH"
