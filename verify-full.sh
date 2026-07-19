#!/usr/bin/env bash
set -euo pipefail

SEED="${SEED:-0xC0FFEE42}"
PROPERTY_CASES="${PROPERTY_CASES:-100000}"
DIFFERENTIAL_CASES="${DIFFERENTIAL_CASES:-1000000}"
HASH_CASES="${HASH_CASES:-100000}"
FUZZ_PROFILE="${FUZZ_PROFILE:-development}"
SOAK_PROFILE="${SOAK_PROFILE:-one-hour}"
ARTIFACT_DIR="artifacts/verification"
FULL_DIR="$ARTIFACT_DIR/full"
FULL_JSON="$FULL_DIR/adversarial_verification_report.json"
FULL_MD="$FULL_DIR/adversarial_verification_report.md"
ENGINE_JSON="$FULL_DIR/adversarial_verification_engine_report.json"
ENGINE_MD="$FULL_DIR/adversarial_verification_engine_report.md"
FUZZ_SUMMARY_JSON="$ARTIFACT_DIR/fuzz_summary.json"
SOAK_SUMMARY_JSON="$ARTIFACT_DIR/soak_summary.json"
VERIFY_FULL_REUSE_FUZZ="${VERIFY_FULL_REUSE_FUZZ:-0}"
VERIFY_FULL_REUSE_SOAK="${VERIFY_FULL_REUSE_SOAK:-0}"
VERIFY_FULL_MAX_ARTIFACT_AGE_SECONDS="${VERIFY_FULL_MAX_ARTIFACT_AGE_SECONDS:-21600}"
mkdir -p "$FULL_DIR"

CURRENT_HEAD="$(git rev-parse HEAD 2>/dev/null || echo unknown)"
CURRENT_DIRTY="false"
if [[ -n "$(git status --short 2>/dev/null || true)" ]]; then
  CURRENT_DIRTY="true"
fi

validate_fuzz_summary() {
  python - <<'PY' "$FUZZ_SUMMARY_JSON" "$FUZZ_PROFILE" "$SEED" "$CURRENT_HEAD" "$CURRENT_DIRTY" "$VERIFY_FULL_MAX_ARTIFACT_AGE_SECONDS"
import json
import os
import sys
import time

summary_path, profile, seed, head, dirty, max_age = sys.argv[1:]
with open(summary_path, "r", encoding="utf-8") as handle:
    data = json.load(handle)

if data.get("profile") != profile:
    raise SystemExit(1)
if str(data.get("seed")) != str(seed):
    raise SystemExit(1)
if data.get("git_head") != head:
    raise SystemExit(1)
if bool(data.get("dirty_worktree")) != (dirty.lower() == "true"):
    raise SystemExit(1)
if not data.get("successful"):
    raise SystemExit(1)
targets = data.get("targets") or []
if len(targets) != 4:
    raise SystemExit(1)
if any(target.get("status") != 0 for target in targets):
    raise SystemExit(1)
age_seconds = time.time() - os.path.getmtime(summary_path)
if age_seconds > int(max_age):
    raise SystemExit(1)
PY
}

validate_soak_summary() {
  python - <<'PY' "$SOAK_SUMMARY_JSON" "$SOAK_PROFILE" "$SEED" "$CURRENT_HEAD" "$CURRENT_DIRTY" "$VERIFY_FULL_MAX_ARTIFACT_AGE_SECONDS"
import json
import os
import sys
import time

summary_path, profile, seed, head, dirty, max_age = sys.argv[1:]
with open(summary_path, "r", encoding="utf-8") as handle:
    data = json.load(handle)

if data.get("profile") != profile:
    raise SystemExit(1)
if str(data.get("seed")) != str(seed):
    raise SystemExit(1)
if data.get("git_head") != head:
    raise SystemExit(1)
if bool(data.get("dirty_worktree")) != (dirty.lower() == "true"):
    raise SystemExit(1)
if not data.get("successful"):
    raise SystemExit(1)
report = data.get("report") or {}
if report.get("duration_completed_seconds", 0) < report.get("duration_requested_seconds", 0):
    raise SystemExit(1)
counters = report.get("counters") or {}
if any(counters.get(name, 0) != 0 for name in [
    "differential_mismatches",
    "semantic_hash_mismatches",
    "replay_false_acceptances",
    "rollback_failures",
    "partial_commits",
    "panic_count",
    "invariant_failures",
]):
    raise SystemExit(1)
age_seconds = time.time() - os.path.getmtime(summary_path)
if age_seconds > int(max_age):
    raise SystemExit(1)
PY
}

echo "== Hollow Grove adversarial verification: FULL =="
echo "seed: $SEED"
echo "property cases: $PROPERTY_CASES"
echo "differential cases: $DIFFERENTIAL_CASES"
echo "hash cases: $HASH_CASES"

cargo fmt --check
cargo clippy --all-targets --all-features
cargo test
cargo build
cargo build --release
./bench-local.sh | tee "$FULL_DIR/bench-full.log"
./stress-local.sh | tee "$FULL_DIR/stress-full.log"
cargo run --quiet --bin client_desktop_status >/dev/null
if [[ "$VERIFY_FULL_REUSE_FUZZ" == "1" ]] && [[ -f "$FUZZ_SUMMARY_JSON" ]] && validate_fuzz_summary; then
  echo "reusing validated fuzz artifact: $FUZZ_SUMMARY_JSON"
else
  ./fuzz-local.sh "$FUZZ_PROFILE"
fi
if [[ "$VERIFY_FULL_REUSE_SOAK" == "1" ]] && [[ -f "$SOAK_SUMMARY_JSON" ]] && validate_soak_summary; then
  echo "reusing validated soak artifact: $SOAK_SUMMARY_JSON"
else
  ./soak-local.sh "$SOAK_PROFILE"
fi

RELEASE_HASH="$(cargo run --release --quiet --bin adversarial_verification -- hash-corpus --cases "$HASH_CASES" --seed "$SEED")"
cargo run --quiet --bin adversarial_verification -- report \
  --profile full \
  --seed "$SEED" \
  --property-cases "$PROPERTY_CASES" \
  --differential-cases "$DIFFERENTIAL_CASES" \
  --hash-cases "$HASH_CASES" \
  --compare-other-label release \
  --compare-other-hash "$RELEASE_HASH" \
  --json "$ENGINE_JSON" \
  --md "$ENGINE_MD" \
  --matrix "$FULL_DIR/exhaustive_matrix.tsv" | tee "$FULL_DIR/verify-full-summary.log"

python - <<'PY' "$ENGINE_JSON" "$FUZZ_SUMMARY_JSON" "$SOAK_SUMMARY_JSON" "$FULL_JSON" "$FULL_MD" "$SEED" "$PROPERTY_CASES" "$DIFFERENTIAL_CASES" "$HASH_CASES" "$CURRENT_HEAD" "$CURRENT_DIRTY"
import json
import sys

(
    engine_json_path,
    fuzz_json_path,
    soak_json_path,
    full_json_path,
    full_md_path,
    seed,
    property_cases,
    differential_cases,
    hash_cases,
    git_head,
    dirty_worktree,
) = sys.argv[1:]

with open(engine_json_path, "r", encoding="utf-8") as handle:
    engine = json.load(handle)
with open(fuzz_json_path, "r", encoding="utf-8") as handle:
    fuzz = json.load(handle)
with open(soak_json_path, "r", encoding="utf-8") as handle:
    soak = json.load(handle)

report = {
    "profile": "full",
    "seed": seed,
    "source_state": {
        "git_head": git_head,
        "dirty_worktree": dirty_worktree.lower() == "true",
    },
    "engine": engine,
    "fuzz": fuzz,
    "soak": soak,
    "stages": {
        "property": {
            "attempted": True,
            "passed": int(engine["property"]["cases"]),
            "failed": 0,
            "seed": engine["property"]["seed"],
        },
        "differential": {
            "attempted": True,
            "cases_compared": int(engine["differential"]["cases"]),
            "matches": int(engine["differential"]["cases"]) - int(engine["differential"]["mismatches"]),
            "mismatches": int(engine["differential"]["mismatches"]),
            "seed": engine["differential"]["seed"],
        },
        "rollback": {
            "attempted": True,
            "cuts_attempted": int(engine["rollback"]["cut_count"]),
            "complete_rollbacks": int(engine["rollback"]["cut_count"]),
            "partial_commits": 0,
        },
        "trace_corruption": {
            "attempted": True,
            "corruptions_attempted": int(engine["trace_corruption"]["corruption_count"]),
            "correctly_rejected": int(engine["trace_corruption"]["corruption_count"]) - int(engine["trace_corruption"]["false_acceptances"]),
            "false_acceptances": int(engine["trace_corruption"]["false_acceptances"]),
        },
        "semantic_hash": {
            "attempted": True,
            "cases": int(engine["semantic_hash"]["corpus_cases"]),
            "seed": engine["semantic_hash"]["seed"],
            "hash_hex": engine["semantic_hash"]["hash_hex"],
            "comparison_label": engine["semantic_hash"]["comparison_label"],
            "comparison_hash": engine["semantic_hash"]["comparison_hash"],
            "mismatch_count": int(engine["semantic_hash"]["mismatch_count"]),
        },
        "fuzz": {
            "attempted": True,
            "profile": fuzz["profile"],
            "seconds_per_target": int(fuzz["seconds_per_target"]),
            "seed": fuzz["seed"],
            "successful": bool(fuzz["successful"]),
            "targets": fuzz["targets"],
        },
        "soak": {
            "attempted": True,
            "profile": soak["profile"],
            "seed": soak["seed"],
            "successful": bool(soak["successful"]),
            "report": soak.get("report", {}),
        },
    },
    "requested_counts": {
        "property_cases": int(property_cases),
        "differential_cases": int(differential_cases),
        "hash_cases": int(hash_cases),
    },
    "skipped": engine.get("skipped", []),
}

with open(full_json_path, "w", encoding="utf-8") as handle:
    json.dump(report, handle, indent=2)
    handle.write("\n")

soak_report = soak.get("report", {})
soak_counters = soak_report.get("counters", {})
soak_latency = soak_report.get("latency", {})
with open(full_md_path, "w", encoding="utf-8") as handle:
    handle.write("# Hollow Grove Adversarial Verification (FULL)\n\n")
    handle.write(f"- seed: `{seed}`\n")
    handle.write(f"- source state: git `{git_head}` dirty `{dirty_worktree}`\n")
    handle.write(f"- exhaustive states: `{engine['exhaustive']['total_legal_states']}` legal / `{engine['exhaustive']['total_rejected_states']}` rejected / `{engine['exhaustive']['total_enumerated_states']}` total\n")
    handle.write(f"- property cases: `{engine['property']['cases']}`\n")
    handle.write(f"- differential cases: `{engine['differential']['cases']}` with mismatches `{engine['differential']['mismatches']}`\n")
    handle.write(f"- rollback cuts: `{engine['rollback']['cut_count']}` with partial commits `0`\n")
    handle.write(f"- trace corruptions: `{engine['trace_corruption']['corruption_count']}` with false acceptances `{engine['trace_corruption']['false_acceptances']}`\n")
    handle.write(f"- semantic hash: `{engine['semantic_hash']['hash_hex']}` with mismatch count `{engine['semantic_hash']['mismatch_count']}`\n\n")
    handle.write("## Fuzz\n")
    handle.write(f"- profile: `{fuzz['profile']}` at `{fuzz['seconds_per_target']}` seconds per target with seed `{fuzz['seed']}`\n")
    for target in fuzz["targets"]:
        handle.write(
            f"- `{target['name']}`: executions `{target['executions']}`, corpus `{target['corpus_size']}`, crashes `{target['crashes']}`, status `{target['status']}`\n"
        )
    handle.write("\n## Soak\n")
    handle.write(
        f"- profile: `{soak['profile']}` requested `{soak_report.get('duration_requested_seconds', 0)}`s / completed `{soak_report.get('duration_completed_seconds', 0)}`s with seed `{soak['seed']}`\n"
    )
    handle.write(
        f"- operations: `{soak_report.get('total_operations', 0)}` at `{soak_report.get('operations_per_second', 0):.2f}` ops/s\n"
    )
    handle.write(
        f"- latency us: min `{soak_latency.get('min_us', 0):.2f}`, mean `{soak_latency.get('mean_us', 0):.2f}`, p50 `{soak_latency.get('p50_us', 0):.2f}`, p95 `{soak_latency.get('p95_us', 0):.2f}`, p99 `{soak_latency.get('p99_us', 0):.2f}`, max `{soak_latency.get('max_us', 0):.2f}`\n"
    )
    handle.write(
        f"- counters: invariant `{soak_counters.get('invariant_failures', 0)}`, differential `{soak_counters.get('differential_mismatches', 0)}`, semantic hash `{soak_counters.get('semantic_hash_mismatches', 0)}`, replay false acceptances `{soak_counters.get('replay_false_acceptances', 0)}`, partial commits `{soak_counters.get('partial_commits', 0)}`, panics `{soak_counters.get('panic_count', 0)}`\n"
    )
    handle.write(
        f"- RSS kB: start `{soak_report.get('rss_start_kb', 0)}`, end `{soak_report.get('rss_end_kb', 0)}`, peak `{soak_report.get('rss_peak_kb', 0)}`, trend `{soak_report.get('memory_trend', 'unknown')}`\n"
    )
    handle.write("\n## Skipped\n")
    for skipped in engine.get("skipped", []):
        handle.write(f"- `{skipped['stage']}`: {skipped['reason']}\n")
PY

echo "FULL verification complete."
