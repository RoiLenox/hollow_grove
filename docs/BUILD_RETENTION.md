# Build output and recovery retention

Keep one active checkout per machine, authored inputs and Git history, current
compatibility fixtures, a useful warm build cache, the accepted package set and
one compatible rollback set. Retain their source bundle, lockfile/toolchain
provenance, checksums, validation receipts, and world/configuration snapshots.

Cargo `target/` directories and completed makepkg build workspaces are derived.
They may be removed after confirming no process uses them, extracting any
required release binaries/metadata, and proving the retained source rebuilds.
Prefer clearing obsolete checkout caches first. Schedule cleanup after a
qualification cycle, not during concurrent builds. Keep package archives and
source bundles outside disposable build workspaces.

`dist/` is mixed release evidence: classify each artifact before pruning. Keep
measured before/after results when a report cites them. Runtime saves, legacy
fixtures, migration logs, original source-law documents and generated canonical
baselines are not caches. A Git bundle alone does not preserve untracked data.

Before deleting an old incoming/pre-update checkout, compare its Git ancestry,
working diff, untracked files and ignored runtime data. Preserve unique work and
write a path/bytes/reason/retained-copy manifest. Record actual filesystem space
afterwards; directory sizes can double-count hardlinks and shared extents.

The laptop's `organization-20261002` backups include personal media, creative
work and application state. They require a separate owner-directed retention
comparison and are excluded from project-cache cleanup. Keep the October 29
source-transfer and installation recovery packets.

Source and package transfers exclude caches and home backups. Hardware policy,
credentials and personal state remain machine-owned. Installed runtime upgrades
need compatible data rollback; an old service may not read newer Boardwalk saves.
