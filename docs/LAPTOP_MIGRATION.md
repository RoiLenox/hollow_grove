# Laptop Migration and Installation Plan

The laptop upgrade is intentionally side-by-side and recoverable. It never
copies build caches, workstation state, SSH material, or ignored artifacts.

## Required facts

Before connecting, identify:

- the SSH destination in `user@host` form;
- the laptop's absolute Hollow Grove checkout path;
- whether the laptop has enough free space for one old checkout, one source-only
  checkout, and a small Hueman release;
- whether Godot 4.7+ is installed if Hueman should launch there.

## Migration sequence

1. Finish and tag the local `foundation-v1-rc1` commit.
2. Build the portable release with `./build-portable-release.sh`.
3. Run a stage-only migration first. The script transfers a Git bundle and the
   two checksummed release archives rather than `target/`, `.godot/`, saves, or
   machine-local artifacts.
4. The laptop writes a complete recovery record: old HEAD/status, binary working
   and index patches, all Git refs, and an archive of every untracked file.
5. It clones the current source beside the old checkout, checks out the exact
   transferred commit, validates art, formatting, and the complete workspace
   test suite, then verifies both release archives.
6. Hueman is installed without root beneath
   `$XDG_DATA_HOME/hollow-grove/releases`, with an atomic `current-hueman`
   symlink and a command in `~/.local/bin`.
7. Only a second run with `--activate` renames the old checkout to a timestamped
   backup and puts the verified checkout at the original path. If the second
   rename fails, the old checkout is restored immediately.

Stage first:

```bash
./sync-hollow-grove-laptop.sh \
  --host user@laptop \
  --remote-repo /absolute/path/to/hollow_grove \
  --stage-only
```

After reviewing the printed migration record, activate:

```bash
./sync-hollow-grove-laptop.sh \
  --host user@laptop \
  --remote-repo /absolute/path/to/hollow_grove \
  --activate
```

## Rollback

The activation output names the preserved checkout and recovery-record
directory. To roll source back, stop any Hollow Grove process, rename the new
checkout aside, and rename the timestamped `pre-foundation-v1` directory back
to the original path. To roll Hueman back, point `current-hueman` at the prior
release directory. No backup is removed automatically.

System-wide General Edition installation is deliberately separate because it
modifies `/etc` and `/usr` and therefore needs explicit administrator approval.
On an Arch-family laptop, transfer the validated `.pkg.tar.zst` files and review
`pacman -U` before installing them; on another distribution, retain the
portable user-local Hueman install and adapt the General Edition profile through
that distribution's package tooling.
