# Flynt Revolution C recovery report — 26 October 2026

**Historical checkpoint:** this report records the earlier 33-test recovery.
For the latest 37-test closure boundary, use the
[current continuation](FLYNT_REV_C_CONTINUATION.md) and
[constitutional closure review](FLYNT_REV_C_CONSTITUTIONAL_CLOSURE.md).
The status listing, counts and restart prompt below remain recovery-session
evidence; they are not the current repository status or next task.

Recovered Checkpoint C and added tested Q preparation/contest records plus a
bounded Triple Crown custody contract. **Flynt is still PARTIAL:** legal
succession reconciliation and authenticated gameplay integration remain open.
No complete game or release is claimed.

## Repository and preservation

Branch: `os/package-foundation`.
HEAD: `d51f506d3a4d9f00381d5e4d44471b316f7a5c1c`.
Full final `git status --short` is in `FLYNT_REV_C_RECOVERY_STATUS.txt`.
Initial status had 47 modified tracked paths and 35 untracked status entries.
Final status has **47 modified tracked paths, 43 untracked status entries and
zero staged paths**. Of these, 15 are Flynt-related (including shared files):

| Git status | Path |
|---|---|
| M, unchanged this session | `HOLLOW_GROVE_CANON_AMENDMENT_FLYNT_SYNTHESIS_SANDMANOR_HANDS_V1.md` |
| M, unchanged this session | `src/world/flynt.rs` |
| M, unchanged this session | `tests/integration.rs` |
| ?? | `docs/FLYNT_REV_C_CONTINUATION.md` |
| ?? | `docs/FLYNT_REV_C_GOVERNANCE.md` |
| ?? | `docs/FLYNT_REV_C_RECOVERY_REPORT.md` |
| ?? | `docs/FLYNT_REV_C_RECOVERY_STATUS.txt` |
| ?? | `src/world/flynt/equipment.rs` |
| ?? | `src/world/flynt/governance.rs` |
| ?? | `src/world/flynt/q.rs` |
| ?? | `src/world/flynt/synthesis.rs` |
| ??, byte-preserved | `tests/fixtures/flynt-governance-a-v1.json` |
| ?? | `tests/flynt_rev_c_equipment.rs` |
| ?? | `tests/flynt_rev_c_governance.rs` |
| ?? | `tests/flynt_rev_c_q.rs` |

All **1,236 other pre-existing captured files** remained byte-identical.
The eight new files passed a separate trailing-whitespace check; all changes
remain unstaged. The source snapshot baseline intentionally includes existing
user edits; preservation does not mean the repository is clean.

No staging, commit, branch change, reset, clean, stash, package installation,
personal-save edits or OS/niri configuration changes occurred.

A SHA-256 snapshot captured 1,240 pre-existing tracked/untracked files.
Only four of those changed in this session:

- `src/world/flynt/governance.rs`: recover formatting, fix Clippy, integrate
  additive Q/equipment commands into transactional journal replay.
- `src/world/flynt/synthesis.rs`: recover formatting; expose Trisynthesis and
  The Trifecta as names for the one Chimera outcome.
- `tests/flynt_rev_c_governance.rs`: formatting, alias check, child-suite registration.
- `docs/FLYNT_REV_C_CONTINUATION.md`: append recovery/checkpoint evidence.

New implementation/tests: `src/world/flynt/q.rs`,
`src/world/flynt/equipment.rs`, `tests/flynt_rev_c_q.rs`,
`tests/flynt_rev_c_equipment.rs`. New documentation:
`docs/FLYNT_REV_C_GOVERNANCE.md`, `docs/SANDMANOR_REV_C_PAIRED_PATHS_DESIGN.md`,
this report and its status listing. Existing module exports in
`src/world/flynt.rs:9` and registration in `tests/integration.rs:41` were
preserved without further edits. The child suites run inside that harness.

## Recovery diagnosis

Original temporary logs `/tmp/hg-flynt-revc-rgpc3q74` are absent. The targeted
Flynt command reproduced **22 passed, exit 0** before editing; the independent
format check reproduced **exit 1** exclusively in the three interrupted Flynt
Rust files. Scoped formatting fixed that failure. The missing original
enclosing command prevents a proven historical root-cause attribution.
Strict Clippy subsequently caught a pre-existing nested `if` in expression
switching; the equivalent let-chain passes without suppressing the lint.

Q development had a Serde internal-tag collision, then a new vacancy test
incorrectly assuming JSON instead of the tab-separated world snapshot. Both
are documented in the continuation log. Reviewing the actual codec exposed
the canonical-Tross restoration limitation; Final Vacancy is now explicitly
blocked. No baseline test was weakened and no old archive was regenerated.

## Verified scope

| Checkpoint | Status | Scope |
|---|---|---|
| A | IMPLEMENTED+TESTED, bounded | Three peer command seats, one source Tross, elections/qualifications, history and tenure exit; missing legal policy remains blocked |
| B | IMPLEMENTED+TESTED, bounded | Six Forms, independent pair certificates, one expressed body and historical Bisynth provenance |
| C | IMPLEMENTED+TESTED, bounded | Distinct Chimera/Manticorp Recipes and qualifications, explicit interpretation adoption/review, no extra Chimera gate or automatic office |
| Q | PARTIAL | Eight regressions: certified FIFO, current readiness, 30-day schedule, separate panels, 2–0 and split/Sudden Death results, replay and blocked transfer |
| Triple Crown | IMPLEMENTED+TESTED, bounded | Three regressions: tangible component identities, qualified custody, consented transfer/release and Chimera presentation |

Q final succession, Final Vacancy, appeal/correction/revocation, seven-day
recovery and post-loss thirty-day/improvement reentry are **BLOCKED/NOT
IMPLEMENTED**. A result does not appoint a sovereign. A second contest cannot
replace an unresolved first one. Triple Crown fabrication, ballistics, ammo,
combat, ownership sales and repair are **DEFERRED**.

## Validation

Every command below runs from `/home/warren/Projects/active/hollow_grove`.
Logs/JSON summaries and the preservation snapshot are in
`/tmp/hg-flynt-revc-recovery-s0hymwaj` (temporary; durable results are recorded here).

| Command | Result |
|---|---|
| `cargo test --offline --locked --test integration flynt_rev_c_governance --no-fail-fast` | C: 22; Q: 30; Crown: 33 passed, each exit 0 |
| `cargo test --workspace --all-features --offline --locked --no-fail-fast` | 1,813 passed, 0 failed, 0 ignored; exit 0 |
| `cargo test --workspace --offline --locked --no-fail-fast` | 1,675 passed, 0 failed, 0 ignored; exit 0 |
| `cargo clippy --workspace --all-targets --all-features --offline --locked -- -D warnings` | Exit 0 after the scoped lint fix |
| `cargo fmt --all -- --check` | Exit 0 |
| `git diff --check` | Exit 0 |
| `python3 -B tools/verify-backend-contracts.py` | Exit 0 |
| `python3 -B tools/test_backend_contracts.py` | 26 passed; exit 0 |
| `./target/debug/hollow-grove architecture --json` | Exit 0; 14 verified bindings, 10 unbound gameplay routes |

Full all-features results include Flynt 33, Stonebend 27, Sandmanor 22 and
Glaüshouse 23 governance tests, including all 14 clinical regressions. Existing
House and clinical source files were byte-preserved. Counts are test executions,
not assertions or necessarily unique definitions.

## Archives, canon and remaining gates

Frozen A fixture `tests/fixtures/flynt-governance-a-v1.json` is unchanged:
SHA-256 `26b66ae35099bab6b719083308f17e438fa8e5fb95bb3177af4cc2c64094fa48`.
Its exact byte roundtrip still passes. New Q/equipment histories replay
deterministically, duplicate events are idempotent, stale/conflicting events
and failed commands leave both state and encoded journal unchanged. Shared
archive version and existing ruleset remain unchanged; old readers cannot
interpret the additive actions. Commander resignation/expiry retains earned
qualifications. Body changes disable Crown presentation without destroying
mastery/custody; release remains possible. There is no Q sovereign handoff to
claim as tested safe reversion.

Ambisynthesis/Ambisynth are active vocabulary; Bisynth survives as provenance.
Ordinary Chimera qualification is separate from the old unique First Companion.
Old combined-Manticorp/perfected-Chimera interpretations, divided-manticorp and
Manticore Recipe IDs, source Tross identities and historical office/cover
records remain untouched. An explicit new interpretation is tested with
simulated authority; no actual legal enactment, credentials, XP or physical
Recipe execution is manufactured.

Concrete blockers:

- `FLYNT_CONSTITUTION_V2.md:419–443`: recorded vacancy/removal triggers and
  four-House/Court/Stonebend seal succession conflict with newer master §6.4.
  `HOLLOW_GROVE_COMPROMISE_V1_DRAFT.md:28–44` bars silent replacement.
- `FLYNT_CONSTITUTION_V2.md:3–12,32–44,83–100`: current body boundary,
  historical combined/perfected order and unique companion need a lawful
  versioned reconciliation before production convergence adoption.
- `src/world/persistence.rs:117–131,155–183`: canonical holders are restored
  and additional holders appended. Final Vacancy needs an explicit durable
  vacancy event/migration; omission cannot remove the canonical Tross.
- Exact calendar/cohorts, veto window, recall, special elections and detailed
  Court/appeal dispositions remain legal parameters. No numerical defaults
  were copied from another House.
- Authentication, current conflicts/recusals, clinical scope, consent and
  actual external execution are trusted-caller obligations. String signers
  and checksums are not authentication or gameplay integration.

Sandmanor **Quad=Aura Fields**, **Wrangle=Content Farm**, and **Minoan
Peninsula** paired four-rung tables are preserved as approved future design in
`SANDMANOR_REV_C_PAIRED_PATHS_DESIGN.md`. Runtime Agency ranks, elected Mino/Cent,
clinical rules and geography were not migrated. Seer control, wider gameplay,
economy and signed-release work remain deferred.

## Next action and continuation prompt

Last verified stable implementation: C plus the bounded Q record and Triple
Crown contracts. The smallest independent next action is to reconcile the
succession amendment against the existing source, explicitly deciding which
four-House, Court and sealing requirements survive Q victory and how lawful
vacancy is represented. Keep authority transfer blocked until that decision
and its versioned contract are approved and tested.

Ready-to-paste continuation:

> Continue `/home/warren/Projects/active/hollow_grove` on its current branch.
> Read `docs/FLYNT_REV_C_CONTINUATION.md`,
> `docs/FLYNT_REV_C_RECOVERY_REPORT.md`, and
> `docs/FLYNT_REV_C_GOVERNANCE.md`; do not repeat the whole-repository audit.
> Preserve all dirty work and frozen archives; no reset/clean/stash/commit,
> branch change, package installation, saves or OS edits. C and the bounded
> Triple Crown contract are verified; 33 Flynt tests pass. Q preparation and
> contest results are partial, with succession/Final Vacancy explicitly blocked.
> First reconcile `FLYNT_CONSTITUTION_V2.md:419–443` and the canonical-holder
> codec with master §6.4; do not bypass these with signer strings. Specify a
> lawful versioned amendment, Court/appeal disposition and vacancy/tenure
> transaction before enabling any sovereign transfer. Then implement recovery,
> post-loss improvement/reentry and Final Vacancy in tested increments. Keep
> Ambisynth qualifications, one body, historical identities and clinical
> boundaries intact. Sandmanor paired paths remain documentation-only.
> Initial targeted command:
> `cargo test --offline --locked --test integration flynt_rev_c_governance --no-fail-fast`.
> Update the durable continuation at each checkpoint and report actual exit codes.
