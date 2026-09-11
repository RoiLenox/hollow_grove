# Laptop Migration and Installation Plan

The laptop upgrade is intentionally side-by-side and recoverable. It transfers
the complete authoritative source and selected live Hollow Grove state, while
never copying build caches, credentials, SSH material, or stale runtime locks.

## Required facts

Before connecting, identify:

- the SSH destination in `user@host` form;
- the laptop's absolute Hollow Grove checkout path;
- whether the laptop has enough free space for one old checkout, one source-only
  checkout, and a small Hueman release;
- whether Godot 4.7+ is installed if Hueman should launch there.

## Migration sequence

1. Finish, verify, commit, and tag the exact local transfer checkpoint.
2. Build the portable release with `./build-portable-release.sh`.
3. Build the live-state payload with `./build-authoritative-state.sh`. It
   includes the live Niri profile and keyboard standard, Hollow Grove HUD,
   user services and helpers, and durable gameplay saves. It excludes
   credentials, `target/`, package build caches, and runtime lock/claim files.
4. Run a stage-only migration first. The script transfers a Git bundle, the
   two checksummed release archives, and the checksummed state payload.
5. The laptop writes a complete recovery record: old HEAD/status, binary working
   and index patches, all Git refs, and an archive of every untracked file.
6. It clones the current source beside the old checkout, checks out the exact
   transferred commit, validates art, formatting, and the complete workspace
   test suite, then verifies both release archives.
7. Hueman is installed without root beneath
   `$XDG_DATA_HOME/hollow-grove/releases`, with an atomic `current-hueman`
   symlink and a command in `~/.local/bin`.
8. The workstation state is extracted inside the migration record. Review and
   adapt the Niri output, GPU, absolute-path, and package differences before
   installing those files on the laptop; stage-only never overwrites laptop
   configuration.
9. Only a second run with `--activate` renames the old checkout to a timestamped
   backup and puts the verified checkout at the original path. If the second
   rename fails, the old checkout is restored immediately.

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

## Rollback

The activation output names the preserved checkout and recovery-record
directory. To roll source back, stop any Hollow Grove process, rename the new
checkout aside, and rename the timestamped `pre-update` directory back
to the original path. To roll Hueman back, point `current-hueman` at the prior
release directory. No backup is removed automatically.

The state payload is evidence and migration input, not a blind dotfile
installer. In particular, the workstation monitor EDID and Radeon environment
must not replace laptop-specific output and graphics settings.

System-wide General Edition installation is deliberately separate because it
modifies `/etc` and `/usr` and therefore needs explicit administrator approval.
On an Arch-family laptop, transfer the validated `.pkg.tar.zst` files and review
`pacman -U` before installing them; on another distribution, retain the
portable user-local Hueman install and adapt the General Edition profile through
that distribution's package tooling.

## Ongoing two-device updates

Both devices use the same `hollow-grove-update` command. Configure each device
with the other as its peer; configuration contains only an SSH destination and
repository path, never credentials:

```bash
hollow-grove-update configure \
  --host user@other-device \
  --remote-repo /absolute/path/to/hollow_grove
```

The safe update sequence is:

```bash
hollow-grove-update status
hollow-grove-update prepare
hollow-grove-update stage
hollow-grove-update activate
```

Run it on the device whose clean checkpoint and durable state are intended to
be authoritative for that update. `stage` never switches the peer checkout or
active Hueman symlink. `activate` switches the verified source, Hueman release,
and durable gameplay saves, preserving timestamped source and save backups. It
refuses a dirty or divergent peer by default; `--replace-diverged` is an
explicit recovery operation after reviewing the preserved peer bundle,
patches, untracked archive, and checkout.

`prepare` runs the complete tests and strict lint on the sending device. The
peer accepts the already-built portable release without needing Rust or network
access, while still checking the exact source commit, art assets, archive and
payload checksums, and build manifest before it can activate.

For save/configuration-only changes on an otherwise unchanged commit, use
`hollow-grove-update prepare-state` before staging. Mutable saves are not
automatically merged in both directions: choosing the sending device avoids a
silent last-writer-wins loss. Desktop state is staged for hardware adaptation,
so one machine's display EDID or GPU settings never blindly replace the
other's. Built release directories and the per-device peer configuration are
also excluded; each peer rebuilds/releases locally and is configured once.
