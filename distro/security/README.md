# Distribution security and enrollment gates

These gates are intentionally not applied by package preparation. Applying a
firewall, replacing SSH policy, changing boot mount permissions or repairing a
package database requires an interactive, recoverable administrative action on
the target machine.

## Both machines

- Require SSH public keys; disable password, keyboard-interactive and root
  authentication after a second recovery path has been tested.
- Restrict SSH to the trusted LAN or a private overlay network. Preserve the
  current session while testing new firewall rules from a second connection.
- Pin peer host keys and keep device identities separate from distribution
  package-signing identities.
- Schedule filesystem integrity maintenance and verify off-device backups.

## `noahsarch`

- Keep Btrfs/Snapper as the stable recovery layer and enable a periodic scrub.
- Build packages in clean chroots as an unprivileged builder.
- Serve repository files read-only. The network service must never have access
  to the signing private key or writable source history.

## `occamsrazer`

- Move the stray `/var/lib/pacman/local/CACHY_UPDATE_NOTICE` marker out of the
  local database and rerun `pacman -Dk` before package enrollment.
- Change the EFI mount mask so the systemd random seed is not world-readable.
- Use verified pre-deployment backups while the root filesystem remains ext4.
- Classify foreign and orphaned packages; never remove them in bulk.
- Enable firmware virtualization only if this machine will independently run
  QEMU validation. Otherwise disable the currently unusable libvirt service.

Administrative scripts must default to audit/dry-run, print their exact target,
and refuse to operate when the detected hostname or source state differs from
the declared machine profile.
