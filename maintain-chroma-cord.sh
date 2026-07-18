#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
CHROMA_CORD_ROOT="${CHROMA_CORD_ROOT:-/home/warren/chroma_cord}"
CHROMA_CORD_DATA_ROOT="${CHROMA_CORD_DATA_ROOT:-/home/warren/.local/share/chroma_cord}"
REGISTRY_PATH="$SCRIPT_DIR/artifacts/hollow_grove_application_registry.json"
CHROMA_CORD_BIN="$CHROMA_CORD_ROOT/chroma_cord"
FULL_CHECK=false
FAILURES=0
WARNINGS=0

if [[ "${1:-}" == "--full" ]]; then
    FULL_CHECK=true
elif [[ $# -gt 0 ]]; then
    printf 'usage: %s [--full]\n' "$0" >&2
    exit 2
fi

pass() {
    printf 'PASS  %s\n' "$1"
}

warn() {
    WARNINGS=$((WARNINGS + 1))
    printf 'WARN  %s\n' "$1"
}

fail() {
    FAILURES=$((FAILURES + 1))
    printf 'FAIL  %s\n' "$1"
}

check_command() {
    if command -v "$1" >/dev/null 2>&1; then
        pass "tool available: $1"
    else
        fail "required tool missing: $1"
    fi
}

run_check() {
    local label="$1"
    shift
    if "$@"; then
        pass "$label"
    else
        fail "$label"
    fi
}

check_command jq
check_command rg
check_command git
check_command bash
if [[ "$FULL_CHECK" == true ]]; then
    check_command cargo
fi

if [[ -f "$REGISTRY_PATH" ]] && jq empty "$REGISTRY_PATH" >/dev/null 2>&1; then
    pass "application registry is valid JSON"
else
    fail "application registry is missing or invalid"
fi

if jq -e '
    .schema_version == "0.1.0"
    and .control_plane == "hollow_grove"
    and (.applications | length) == 1
    and .applications[0].id == "application.glaushouse.chroma-cord"
    and .applications[0].canonical_name == "chroma_cord"
    and .applications[0].window_app_id == "hollow-grove.chroma-cord"
    and .applications[0].world_anchor.id == "glaushouse"
    and .applications[0].world_anchor.institution_id == "institution.glaushouse.medical-civilization"
    and .applications[0].world_anchor.site_id == "site.glaushouse.central-medical-district"
    and .applications[0].world_anchor.zone_id == "zone.glaushouse.medical-district.recovery-chambers"
    and .applications[0].world_anchor.normalized == {"x": 0.5, "y": 0.86}
    and .applications[0].privacy.capture_allowed == false
    and .applications[0].privacy.projection == "semantic_only"
    and .applications[0].authority.lifecycle == "hollow_grove"
    and .applications[0].authority.world_attachment == "hollow_grove"
    and .applications[0].authority.domain_grammar == "chroma_cord"
    and .applications[0].authority.storage == "chroma_cord"
    and (.applications[0].scope.minimum_required | index("privacy_enforcement")) != null
    and (.applications[0].scope.maximum_allowed | index("backup_coordination")) != null
    and (.applications[0].scope.prohibited | index("mutate_committed_records")) != null
    and (.applications[0].scope.prohibited | index("capture_clinical_window")) != null
' "$REGISTRY_PATH" >/dev/null 2>&1; then
    pass "registry identity, anchor, ownership, privacy, and scope agree"
else
    fail "registry contract drift detected"
fi

if [[ -x "$SCRIPT_DIR/launch-chroma-cord.sh" ]]; then
    pass "Hollow Grove launcher is executable"
else
    fail "Hollow Grove launcher is missing or not executable"
fi

if [[ -x "$CHROMA_CORD_BIN" ]]; then
    pass "chroma_cord binary is executable"
else
    fail "chroma_cord binary is missing or not executable: $CHROMA_CORD_BIN"
fi

for script in \
    launch-chroma-cord.sh \
    rebind-hueman-pair.sh \
    run-hueman-godot.sh \
    capture-hueman-pair-preview.sh \
    sync-hueman-pair-state.sh \
    clear-hueman-pair.sh \
    rotate-hueman-pair.sh \
    step-hueman-pair-spread.sh; do
    if bash -n "$SCRIPT_DIR/$script"; then
        pass "shell syntax: $script"
    else
        fail "shell syntax: $script"
    fi
done

if [[ -d "$CHROMA_CORD_DATA_ROOT" ]]; then
    invalid_data=false
    while IFS= read -r -d '' data_file; do
        if ! jq empty "$data_file" >/dev/null 2>&1; then
            fail "invalid chroma_cord JSON data: $data_file"
            invalid_data=true
        fi
    done < <(find "$CHROMA_CORD_DATA_ROOT" -type f -name '*.json' -print0)
    if [[ "$invalid_data" == false ]]; then
        pass "existing chroma_cord JSON data is readable"
    fi
else
    warn "chroma_cord data directory does not yet exist: $CHROMA_CORD_DATA_ROOT"
fi

if legacy_hits="$(rg -n -i 'chromachord|chroma-chord' "$CHROMA_CORD_ROOT" \
    --glob '!target/**' \
    --glob '!.git/**' \
    --glob '!*.docx' \
    --glob '!*.pdf' \
    --glob '!*.odt' \
    --glob '!*.tar.gz' \
    --glob '!docs/chroma_cord_whitepaper_v3_latest.txt' \
    --glob '!docs/chroma_cord_v3_abstract_and_introduction*.txt')"; then
    fail "legacy active naming remains"
    printf '%s\n' "$legacy_hits"
else
    pass "active chroma_cord naming is canonical"
fi

if rg -q '#\[(test|cfg\(test\))\]' "$CHROMA_CORD_ROOT/rust/src"; then
    pass "primary chroma_cord crate has in-tree Rust tests"
else
    warn "primary chroma_cord crate has no in-tree Rust tests; retained-project tests do not fully cover the active implementation"
fi

hollow_origin="$(git -C "$SCRIPT_DIR" remote get-url origin 2>/dev/null || true)"
if [[ "$hollow_origin" == *"RoyLenox/hollow_grove.git" ]]; then
    pass "Hollow Grove origin belongs to RoyLenox"
else
    warn "Hollow Grove origin is unexpected: ${hollow_origin:-none}"
fi

chroma_origin="$(git -C "$CHROMA_CORD_ROOT" remote get-url origin 2>/dev/null || true)"
if [[ "$chroma_origin" == *"RoyLenox/chroma_cord.git" ]]; then
    pass "chroma_cord origin belongs to RoyLenox"
else
    warn "chroma_cord origin remains historical: ${chroma_origin:-none}"
fi

for repo in "$SCRIPT_DIR" "$CHROMA_CORD_ROOT"; do
    author_name="$(git -C "$repo" config user.name 2>/dev/null || true)"
    author_email="$(git -C "$repo" config user.email 2>/dev/null || true)"
    if [[ "$author_name" == "Roy Lenox" && "$author_email" == "159462203+RoyLenox@users.noreply.github.com" ]]; then
        pass "Git authorship: $repo"
    else
        warn "Git authorship differs at $repo: ${author_name:-unset} <${author_email:-unset}>"
    fi
done

if [[ "$FULL_CHECK" == true ]]; then
    run_check "Hollow Grove application protocol tests" \
        cargo test --manifest-path "$SCRIPT_DIR/Cargo.toml" application_protocol --lib
    run_check "Hollow Grove chroma_cord adapter tests" \
        cargo test --manifest-path "$SCRIPT_DIR/Cargo.toml" world::chroma_cord --lib
    run_check "Hollow Grove institutional boundary tests" \
        cargo test --manifest-path "$SCRIPT_DIR/Cargo.toml" --test institutional_boundary
    run_check "Hollow Grove runtime application-intent test" \
        cargo test --manifest-path "$SCRIPT_DIR/Cargo.toml" --bin hollow_grove_runtime managed_application_intent
    run_check "chroma_cord primary Rust tests" \
        cargo test --manifest-path "$CHROMA_CORD_ROOT/rust/Cargo.toml"
    run_check "chroma_cord retained project tests" \
        cargo test --manifest-path "$CHROMA_CORD_ROOT/chroma_cord-project/Cargo.toml"
    run_check "chroma_cord clinical model tests" \
        cargo test --manifest-path "$CHROMA_CORD_ROOT/chromamed/Cargo.toml"

    if command -v godot >/dev/null 2>&1; then
        run_check "Hueman Godot project parses headlessly" \
            godot --headless --path "$SCRIPT_DIR/hueman_godot" --quit
    else
        warn "Godot is unavailable; skipped headless project parse"
    fi
fi

if [[ "$FAILURES" -gt 0 ]]; then
    printf 'RED   %s failure(s), %s warning(s)\n' "$FAILURES" "$WARNINGS"
    exit 1
fi

if [[ "$WARNINGS" -gt 0 ]]; then
    printf 'AMBER 0 failures, %s warning(s)\n' "$WARNINGS"
else
    printf 'GREEN 0 failures, 0 warnings\n'
fi
