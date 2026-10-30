# Hollow Grove OS Distribution Architecture

Status: private-first, public-ready distribution foundation

Hollow Grove OS is an Arch-derived distribution whose authority is expressed
through source history, clean package builds, signed repositories, tested
promotion and recoverable installation media. An ISO is a projection of that
system, not its source of authority.

## Trust and machine roles

`noahsarch` is the canonical Git host, clean build forge, repository host and
stable consumer. `occamsrazer` is the hardware-diverse canary and mobile
development system. The canary consumes testing artifacts before those exact
package bytes may be promoted to stable.

Neither machine currently holds distribution signing authority. The signing
key is a separate trust root and must not be generated unattended, stored in
the repository, copied through SSH or embedded in an image. The repository
tracks only public key material and fingerprints after a key ceremony.

```text
occamsrazer source and canary feedback
                  |
                  | Git over pinned SSH
                  v
noahsarch canonical Git -> clean build -> signed testing repository
                                               |
                                               | validate without rebuild
                                               v
                                      signed stable repository
                                          |             |
                                          v             v
                                     installed OS   Archiso image
```

## Four separate planes

1. **Development:** Git transports authored source and review history.
2. **Updates:** pacman consumes immutable packages from signed repositories.
3. **Administration:** SSH bootstraps and repairs machines with constrained,
   key-only access. It is not the application collaboration protocol.
4. **Application:** Hollow Grove Core and Hueman use versioned, authenticated
   application messages. Future multiplayer must not inherit SSH authority.

## Promotion law

- Packages are built in a clean Arch chroot against a recorded repository
  snapshot.
- Every package and repository database is signed by an explicitly trusted
  distribution key.
- Testing and stable contain byte-identical package artifacts. Promotion never
  rebuilds a package.
- `occamsrazer` must pass installation, upgrade, graphical login, Hueman
  launch, save/load/restart and recovery checks before promotion.
- Stable publication fails closed if a signature, manifest, required package,
  source identity or validation record is absent.
- Machine hardware configuration and personal data are never promoted as
  distribution defaults.

## Package layers

The current product boundary remains:

- `hollow-grove-general`: hardware-neutral desktop defaults and native Niri
  integration.
- `hollow-grove-desktop`: complete desktop dependency profile.
- `hollow-grove-core`: authoritative local application service.
- `hollow-grove-cli`: command suite and read-only source/proof inspection.
- `hueman`: separately versioned presentation companion.

The release system will add `hollow-grove-keyring`,
`hollow-grove-mirrorlist`, `hollow-grove-release` and a minimal
`hollow-grove-base` only after their ownership and upgrade behavior have been
validated. In particular, a downstream `/usr/lib/os-release` must not collide
with Arch's `filesystem` package.

## Filesystem and recovery

`noahsarch` uses Btrfs and Snapper. `occamsrazer` remains an ext4 canary until
a tested Hollow Grove installer can migrate it intentionally; no live
conversion is implied. Until then its release gate requires a verified backup
and explicit recovery procedure before every system-level deployment.

Snapshots are rollback aids, not backups. A release is not recoverable until
source, repository artifacts, state and restoration instructions exist on a
different physical device.

## Current gates

The private development repository may be assembled unsigned only when it is
marked `development-unsigned`. Stable assembly forbids this mode. Public or
routine machine enrollment additionally requires:

- a deliberate signing-key ceremony and public keyring package;
- clean-chroot build evidence and independent package verification;
- key-only SSH and host firewall rules that preserve recovery access;
- a durable HTTPS or trusted-LAN repository endpoint;
- Archiso boot/install tests in UEFI and legacy modes;
- upgrade, rollback and disaster-recovery rehearsals.
