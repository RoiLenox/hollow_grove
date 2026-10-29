# Revision 100 integration checkpoint

## Current checkpoint — Stonebend continuity and Sandmanor governance

Date: 2026-10-25. Resumed through `~/implament.md`. The earlier Stonebend
checkpoint below is historical: its listed next milestone and test counts
predate the professional continuity importer and Sandmanor implementation
already present at the start of this continuation.

Repository, branch and HEAD remain `/home/warren/Projects/active/hollow_grove`,
`os/package-foundation`, `d51f506d3a4d9f00381d5e4d44471b316f7a5c1c`.
The supplied master canon and TUI instruction hashes still match the validation
record. This status document records implementation evidence, not constitutional
authority, an election, an in-world ratification or full release completion.

### Existing additions now verified

- Stonebend's `Governance::from_reconciled_snapshot` accepts explicit professional
  continuity evidence for existing holders, original terms, election IDs, rule
  provenance, qualifications, Court review and the existing Freemason seal.
  Missing, conflicting, expired or incompatible evidence is rejected. Its four
  additional tests pass, bringing Stonebend's governance suite to 20 tests.
- Sandmanor has a versioned governance amendment and shared-runtime adapter:
  elected Mino/Cent offices, four-year terms, majority/runoff ballots,
  independently authorized recall and special-term policies, and a Contest
  restricted to both sitting qualified governors. The existing reciprocal
  improvement evaluator is reused without manufacturing guardian mantles.
- Sandmanor covers Contest pauses/retries, winner-only regional vacancy,
  separate Sandman manifestation consent/clearance, safe reversion, historical
  holder projection, idempotent commands and deterministic archive replay.
  Unspecified legal parameters remain explicit refusals. Its original 12 tests
  passed before the repairs described below.
- The current Sandmanor paths and archive inspection CLI were already present.
  Both preserved Sandmanor source documents in `docs/reference/` match HEAD's
  pre-amendment originals byte for byte. Legacy Deck progress and guardian
  records retain their original meanings.

### Repairs in this continuation

Three new regression tests failed against the starting Sandmanor adapter:

1. A voter in a tied attempt could become the same Contest's Court reviewer on
   retry. Voter participation now persists across attempts and excludes Court
   pause, resume and outcome review, even when the later ballot omits that voter.
2. A replaced governor could certify baseline, improvement, learning-trial or
   teaching evidence for the same Contest. All prior contenders remain excluded
   from those independent reviewer roles.
3. A replaced governor could cast the deciding spectator vote in that Contest.
   Their judgment now remains conflicted and is recorded among the exclusions.

All 15 Sandmanor and 20 Stonebend governance tests pass. The new cases verify
rejection without state changes, neutral review succeeding, tenure replacement,
retry behavior and reconstruction of conflict history through archive replay.
The command wire format and shared archive version remain unchanged. The
additional voter set is derived from existing journal commands; historical
archives containing a previously accepted conflicting review fail validation
rather than being rewritten or silently ratified.

The Sandmanor amendment and authority map now describe these boundaries, and
the architecture table includes the elected offices and current governance
adapter. Existing Sandmanor CLI code needed formatting only.

### Fresh validation

| Check | Result |
| --- | --- |
| Targeted `rev100_governance::` integration tests | 35 passed |
| `cargo test --workspace --all-features --offline --locked --no-fail-fast` | 1,743 passed; 0 failed; 0 ignored |
| `cargo test --workspace --offline --locked --no-fail-fast` | 1,605 passed; 0 failed; 0 ignored |
| `cargo clippy --workspace --all-targets --all-features --offline --locked -- -D warnings` | Passed |
| `cargo fmt --all -- --check` and `git diff --check` | Passed |
| Backend contract validator and its Python tests | Passed; 26 tests |
| CI packaging, distribution and machine-audit Python tests | Passed; 4, 7 and 3 tests respectively |
| `./verify-art-assets.sh` | Passed |
| `python3 tools/hg-distribution.py audit` | Exit 0; reports dirty source and no signed-release readiness |
| Rebuilt Stonebend/Sandmanor `paths` commands | Passed; current paths displayed |
| Rebuilt `architecture --json` | Source links passed; no violations; 14 bindings, 10 gameplay bindings still deferred |
| Rebuilt `lore --save FIXTURE --json` | Gameplay V1, V2, V3 and V5 Sympiote fixtures load; source bytes unchanged |

The initial full all-feature run passed 1,740 tests before the three new
regressions. The regression run then reproduced all three gaps (12 passed,
3 failed). The final results above include the fixes.

Evidence logs, file hashes and the pre-edit snapshot of 1,227 existing repository
files are retained at `/tmp/hollow-grove-rev100-check-egolfm_1/`.
This continuation changed only:

- `src/world/sandmanor/governance.rs`
- `tests/sandmanor_rev100_governance.rs`
- `src/main.rs` (formatting the existing Sandmanor command only)
- `SANDMANOR_REV100_GOVERNANCE_AMENDMENT.md`
- `HOLLOW_GROVE_CONSTITUTIONAL_ARCHITECTURE_V1.md`
- `REPOSITORY_AUTHORITY_MAP.md`
- this checkpoint and the continuation header in `~/implament.md`

All 12 fixture files, unrelated existing work and HEAD are unchanged. No live
save was migrated, package installed, installed runtime activated or commit
created. Stonebend continuity code was verified but not changed here.

### Remaining work

The full four-House Revision 100 release remains incomplete. Stonebend and
Sandmanor have headless governance adapters and read-only inspection commands;
live authentication, shared Court docket dispatch, clinical integration and
gameplay command dispatch remain separate work. Signer strings are evidence
references, not authentication. Sandmanor regional legacy holders still require
explicit term/election reconciliation, and old Form XP has not been relabelled.

The next planned core milestone is Glaüshouse's nine distinct Muse institutions,
elected seats and The Call, with explicit historical identity/progression
mappings before changing serialized seats. Then follow Flynt Forms/commanders/
The Q, the distinct Highway road migration, and shared Agency/Factory/economy
contracts. Do not guess ambiguous identities or missing legal parameters, or
interpret these backend checks as approval or completion of those milestones.

Current contracts:
[Stonebend amendment](../STONEBEND_REV100_GOVERNANCE_AMENDMENT.md) and
[Sandmanor amendment](../SANDMANOR_REV100_GOVERNANCE_AMENDMENT.md).

---

## Earlier Stonebend checkpoint — historical

Date: 2026-10-25. Scope: continued the existing approved Stonebend backend
implementation. This is an implementation status record, not new constitutional
authority or an in-world ratification event.

## Sources and starting state

- Consulted `~/hollow_grove_rev_c.md`, `~/implament.md` and the three supplied
  files in `~/HOLLOW_GROVE_REV_C/`.
- Master canon SHA-256:
  `f4286d60d316ce52e252aec583dcd27c1adaebadc8f45c78c1ed8bf736a260c8`.
  Both master and TUI instruction hashes match the validation record.
- Repository: `/home/warren/Projects/active/hollow_grove`.
- Branch: `os/package-foundation`.
- HEAD: `d51f506d3a4d9f00381d5e4d44471b316f7a5c1c`.
- The initial read-only audit in `~/implament.md` predates the working tree's
  Stonebend amendment, governance adapter, TUI/CLI changes and tests. Those
  changes were already present when this session resumed. Keyboard, packaging
  and distribution work was also present and has been preserved.

## Implemented and checked in this continuation

The existing governance suite initially passed all 12 tests. Four new
regressions reproduced actual gaps before the implementation was changed:

1. Institutional projection discarded inactive Stonebend officeholders.
   It now retains historical source records, closes an imported holder's active
   record when its tenure ends, and projects ended governance tenures as
   inactive. Full tenure details remain in the governance journal.
2. A recall petitioner or voter could also provide its independent Court
   certification. Review now excludes petitioners, voters and the recall
   administrator, in addition to affected incumbents.
3. A former supporting officer could review the same Trial after resigning
   and being replaced by another supporting tenure. The Trial now retains
   professional participants throughout its history.
4. Recusal followed by resignation could erase that same conflict. Recusal
   participation now survives both changes and archive replay.

All 16 targeted governance tests pass after these fixes. They also check
unchanged state on rejection, neutral review succeeding after a conflict is
rejected, deterministic replay, institutional round-trip, and preservation of
the source snapshot and other Houses' officeholders.

The broader workspace run exposed stale developer-tool and lifecycle-document
assertions for the old Freemason replacement wording. They now check the
approved election and independent seal-transfer wording already emitted by
the tool, and distinguish branch election thresholds from witness quorums.
Five other failures identified the generated world-context mirror left behind
by the previous Stonebend edit. That mirror now matches its existing authored
source exactly; the source and the other Houses' rules were not changed.

No archive format, stored identifier or save schema was changed. The Trial's
participant set is rebuilt from existing commands. A previously accepted
archive containing conflicted Court certification fails validation under the
corrected checks; the original archive is not rewritten or automatically
ratified. No live gameplay saves were migrated, no packages were installed,
and no commit was created.

## Validation

All results below are fresh executions of this working tree, not results from
the October 8 binaries or the original audit:

| Check | Result |
| --- | --- |
| `cargo test --offline --test integration stonebend_rev100_governance:: -- --test-threads=1` | 16 passed |
| `cargo test --workspace --all-features --offline --locked --no-fail-fast` | 1,724 passed; 0 failed; 0 ignored |
| `cargo test --workspace --offline --locked --no-fail-fast` | 1,586 passed; 0 failed; 0 ignored |
| `cargo clippy --workspace --all-targets --all-features --offline --locked -- -D warnings` | Passed |
| `cargo fmt --all -- --check` and `git diff --check` | Passed |
| `python3 tools/verify-backend-contracts.py` | Passed; no world-state mutation |
| `python3 tools/test_backend_contracts.py` | 26 passed |
| Rebuilt `hollow-grove stonebend paths` | Correct three current paths |
| Rebuilt `hollow-grove architecture --json` | Source links passed; no violations; 14 existing bindings, 10 gameplay bindings still deferred |
| Rebuilt `hollow-grove lore --save FIXTURE --json` | Gameplay V1, V2, V3 and V5 Sympiote fixtures all load; source bytes unchanged |

The earlier targeted regression run failed all four newly added cases before
the fixes, with the original 12 tests passing. Full-suite failures then exposed
the stale documentation assertions and generated mirror described above. The
final full runs pass after those repairs. Strict Clippy also required a
semantics-preserving simplification of the existing outgoing-seal check.

Session evidence and the pre-edit working-tree snapshot are retained under
`/tmp/hollow-grove-rev100-resume-6f_fiwjz/`. A SHA-256 comparison of 1,221 existing
repository files confirms that only the six files listed below changed during
this continuation; all 12 files under `tests/fixtures/` are unchanged:

- `src/world/stonebend/governance.rs`
- `tests/stonebend_rev100_governance.rs`
- `src/bin/hueman_stonebend_roles.rs`
- `tests/stonebend_third_pass_documentation.rs`
- `artifacts/current_synthesis_world_context.md`
- `STONEBEND_REV100_GOVERNANCE_AMENDMENT.md`

This checkpoint is a new file. `~/implament.md` now points here and retains its
original audit below a historical heading. Branch, HEAD, pre-existing unrelated
changes, the supplied canon package and the original audit instructions remain
unchanged. No installed runtime was activated.

## Remaining work and next milestone

The full Revision 100 release is not integrated. The current Stonebend adapter
still trusts externally authenticated evidence; signer strings alone are not
authentication. It preserves shared Court references but does not automatically
open or adjudicate a Court docket. Gameplay dispatch and live election/clinical
adapters remain separate work.

The next smallest backend milestone is explicit professional legacy-tenure
reconciliation: require supplied, independently certified term, election and
seal evidence; preserve the original snapshot and rule provenance; reject
missing evidence instead of inventing dates or authority. The present import
correctly returns `RequiresLegalParameter` for unsupported professional holders.
Do not bypass that refusal by deleting legacy holders or guessing from labels.

Then continue the audit's ordered sequence with Sandmanor elections and Contest
of Improvement, Glaüshouse seats/The Call, Flynt Forms/commanders/The Q, the
distinct road migration, and shared Agency/Factory/economy contracts. Ambiguous
identity mappings and unspecified legal parameters remain unresolved. Existing
historical models and earned progression must remain readable.

The current Stonebend contract is
[the governance amendment](../STONEBEND_REV100_GOVERNANCE_AMENDMENT.md).
