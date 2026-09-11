# Laptop Migration and Installation Plan

The laptop upgrade is side-by-side and recoverable. It transfers the complete
authoritative source, three independent release payloads, and selected live
Hollow Grove state. It never copies credentials, SSH material, build caches, or
stale runtime locks.

## Required facts

Before connecting, identify:

- the SSH destination in `user@host` form;
- the laptop's absolute Hollow Grove checkout path;
- whether the laptop has enough free space for the old checkout, a source-only
  checkout, the migration record, and the three small release archives;
- whether Godot 4.7+ is installed if Hueman should launch there.

## Product and package boundary

The portable build produces three checksummed archives:

- `hollow-grove-core-VERSION.tar.gz` contains the authoritative service,
  persistent-state launch policy, and integration command. It contains no PCK.
- `hueman-VERSION.tar.gz` contains the Godot presentation client and immutable
  assets. It contains no gameplay service.
- `hollow-grove-general-VERSION.tar.gz` contains the hardware-neutral Niri
  profile and the native `hollow-niri` broker. It is retained for explicit
  system-level adaptation rather than installed into `/usr` automatically.

The boundary is specified by
[`HOLLOW_GROVE_HUEMAN_SEAM_V1.md`](HOLLOW_GROVE_HUEMAN_SEAM_V1.md).

## Migration sequence

1. Finish, verify, commit, and tag the exact local transfer checkpoint.
2. Run `hollow-grove-update prepare`. It runs strict lint, the complete Rust
   suite, keyboard-profile verification, Hueman transport/semantic smoke tests,
   and the save/restart/load playable lifecycle before building releases.
3. The state builder captures the live Niri profile and keyboard standard,
   Hollow Grove HUD, user services and helpers, durable gameplay saves, world,
   filtered Core runtime state, and Hueman presentation state. It excludes
   credentials, `target/`, package
   caches, and runtime lock/claim files.
4. Run a stage-only migration first. The script transfers a Git bundle, all
   three release archives, and the authoritative-state archive with checksums.
5. The laptop writes a recovery record containing its old HEAD/status, binary
   working/index patches, all Git refs, and every untracked file.
6. It clones the source beside the old checkout, selects the exact commit,
   validates art, and verifies outer checksums, inner payload checksums, and the
   source-commit manifests for every release and the state payload. The laptop
   does not need Rust or network access for this validation.
7. Hollow Grove Core and Hueman are installed independently without root. Core
   lives beneath `$XDG_DATA_HOME/hollow-grove/releases`; Hueman lives beneath
   `$XDG_DATA_HOME/hueman/releases`.
8. The workstation state is extracted inside the migration record. Stage-only
   does not overwrite configuration, checkout, active releases, or live state.
   Review laptop-specific output, GPU, absolute-path, and package differences.
9. All received archives and the Git bundle remain in the migration record.
   Release/state archives are also copied to the staged checkout's ignored
   `dist/`, allowing that device to become the next update source.
10. Only a second run with `--activate` switches the checkout, Core and Hueman
    links, saves, world, and filtered runtime state. An active or unverifiable
    gameplay-runtime lease blocks state activation even if source-divergence
    replacement was requested.

Stage first:

```bash
./sync-hollow-grove-laptop.sh \
  --host user@laptop \
  --remote-repo /absolute/path/to/hollow_grove \
  --ref exact-transfer-tag \
  --stage-only
```

After reviewing the printed migration record, activate:

```bash
./sync-hollow-grove-laptop.sh \
  --host user@laptop \
  --remote-repo /absolute/path/to/hollow_grove \
  --ref exact-transfer-tag \
  --activate
```

## Installed commands

Activation links these user commands:

- `hollow-grove-service` runs Hollow Grove Core alone.
- `hueman` runs the Hueman client alone and connects through the configured
  loopback endpoint.
- `hollow-grove-with-hueman` starts both independent processes as one seamless
  experience and owns service cleanup.
- `hollow-grove-hueman` is a compatibility alias for the integration command,
  not a combined package.
- `hollow-grove-update` runs the symmetric device updater.

## Rollback

The activation output names every preserved checkout/state path and the
recovery record. Stop Hollow Grove before rollback. Restore the timestamped
`pre-update` checkout and state directories, and point `current-core` plus the
Hueman `current` link at their prior independent releases. Nothing is removed
automatically.

The configuration payload remains evidence and migration input, not a blind
dotfile installer. In particular, the workstation monitor EDID and Radeon
environment must not replace laptop-specific output and graphics settings.

System-wide General Edition installation remains separate because it modifies
`/etc` and `/usr` and needs explicit administrator approval. On an Arch-family
laptop, build/review the package before `pacman -U`; on another distribution,
retain the portable user-local Core and Hueman installs and adapt the General
profile through that distribution's package tooling.

## Ongoing two-device updates

Both devices use the same command. Configuration contains only an SSH
destination and repository path, never credentials:

```bash
hollow-grove-update configure \
  --host user@other-device \
  --remote-repo /absolute/path/to/hollow_grove
```

The safe sequence is:

```bash
hollow-grove-update status
hollow-grove-update prepare
hollow-grove-update stage
hollow-grove-update activate
```

Run it on the device whose clean checkpoint and durable state should be
authoritative. `stage` never switches the peer. `activate` refuses a dirty or
divergent peer by default. `--replace-diverged` is an explicit source-recovery
operation after reviewing the preserved bundle, patches, untracked archive,
and checkout; it never overrides the live-runtime guard.

For state-only changes at the same commit, use
`hollow-grove-update prepare-state`. Mutable saves are not automatically merged
in both directions: explicitly choosing the sending device prevents silent
last-writer-wins loss. Desktop settings remain staged for hardware adaptation,
while installed releases and per-device peer configuration remain outside the
state archive.
