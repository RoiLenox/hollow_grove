# Lore Sync Checkpoint V1

Checkpoint tag: `lore-sync-2026-10-07-v1`

This is the local handoff for the implemented lore/kernel milestone and the
Boardwalk's Pimp/Trooners and Gimp/Gooners occupant organization. The main PC
is offline. Remote comparison, staging, activation, and peer acceptance remain
for when it is available.

## Local preparation

The checkpoint includes all implementation, lore, tests, and working drafts.
`./hollow-grove-update.sh prepare` runs the release checks and rebuilds the
independent Core, Hueman, General, and selected durable-state archives from
the clean committed source.

The generated handoff lives in ignored `dist/`:

- `hollow-grove-core-0.1.0.tar.gz`
- `hueman-0.1.0.tar.gz`
- `hollow-grove-general-0.1.0.tar.gz`
- `hollow-grove-authoritative-state-0.1.0.tar.gz`
- `hollow-grove-lore-sync-2026-10-07-v1.bundle`, containing the source history
  and checkpoint tag
- `LOCAL-TRANSFER-READINESS.json`, recording the exact source commit, artifact
  checksums, and completed local acceptance checks
- `TRANSFER-SHA256SUMS`, covering the handoff files

Check the receipt and verify the handoff locally before sending:

```bash
git status --short
git rev-parse HEAD
git rev-parse lore-sync-2026-10-07-v1^{commit}
git bundle verify dist/hollow-grove-lore-sync-2026-10-07-v1.bundle
(cd dist && sha256sum --check TRANSFER-SHA256SUMS)
```

The state archive is a snapshot of this machine's selected durable state.
Runtime locks and claims are excluded. If saves change after preparation,
refresh with `./hollow-grove-update.sh prepare-state`; rebuild the recorded
handoff checksums after refreshing. Source changes require a new checkpoint
and full preparation.

## When the main PC is on

1. Confirm the SSH login and the active working checkout path. The Git remote
   uses a bare repository; that is a different target from the working
   checkout used by the updater.
2. Inspect the main PC's branch, commit, working changes, and saves. Preserve
   and reconcile any newer source work before choosing the transfer commit.
   Select which machine's durable saves should be authoritative.
3. Confirm the updater's peer configuration, then run
   `./hollow-grove-update.sh status`.
4. Stage the chosen update with `./hollow-grove-update.sh stage`. Review the
   printed recovery record, incoming checkout, and verified manifests.
5. Stop the peer gameplay runtime, then use
   `./hollow-grove-update.sh activate` after review. Dirty or divergent source
   is blocked by default; do not bypass that guard to skip reconciliation.
6. Confirm both machines have the same source commit and the intended release
   manifests. Launch Core/Hueman on the main PC and check save/restart/load.

The updater provides a selected checkpoint transfer, rather than an automatic
merge of two independently changing save histories. Hardware-specific desktop
settings remain staged for review. General system installation is a separate
operation. Recovery behavior and full commands are in
[Laptop Migration](LAPTOP_MIGRATION.md).

## Development still open

Stonebend's object lineage is a WORKING, unratified design draft. Eleven other
lore definitions still await verified gameplay bindings. The Boardwalk camp
organization is adopted lore and presentation; new camp membership mechanics
have not been invented. These remain explicit development work after this
transfer checkpoint.
