# Hollow Grove General Edition ISO Readiness

## Ready in this pass

- The new-user Niri profile is separate from the active personal
  configuration.
- The distribution now has one restrained Hollow Grove top bar using the
  constitutional palette, one Walker entrance, and one `hollow-shell`
  diagnostic/action seam without adding a dock or second desktop framework.
- The constitutional A–Z WHAT vocabulary, shared HOW modifier grammar, HJKL
  movement tool, workspace bindings, column proportions, wrap motion,
  screenshots, media, audio, brightness, Hollow Grove, and Hueman concepts are
  preserved.
- No output, resolution, refresh rate, scale, arrangement, GPU vendor, DRM
  device, or backlight device is forced.
- Optional applications fail through a named notification instead of blocking
  Niri startup.
- Wallpaper and weather use blank new-user settings with safe fallbacks.
- Installed commands and XDG paths replace personal paths and repository
  checkout assumptions.
- The keyboard Grove registry records every A–Z meaning and memory hook while
  distinguishing House landmarks, domain plants, modifier branches,
  replaceable application fruit, and operational tools; its verifier passes
  against the portable profile before release.
- `hollow-keyboard` and its desktop entry make that registry searchable,
  visually discoverable, and locally auditable after installation.
- The native `hollow-niri` broker removes shell, `jq`, fixed sleeps, and
  unbounded repeat backlog from the compositor movement path. Its service,
  planner tests, doctor, benchmark, and renderer report are package-owned.
- The Arch package recipe now pins one pushed source commit and its archive
  checksum, builds from that immutable extraction instead of the checkout,
  and runs the native and package-level validation suites in `check()`.

This desktop integration consumes the constitutional palette but changes no
game or House canon, runtime architecture, gameplay system, institution,
lineage, or metaphysics.

## Local package validation — 2026-08-31

The pinned `hollow-grove-general` recipe completed a rootless package build
from its verified GitHub archive. Its Rust checks, keyboard/Niri/Waybar
verifier, package assembly, and extracted-file audit passed. The resulting
x86_64 package was approximately 480 KiB compressed and 1 MiB installed, and
the shipped `hollow-niri` binary contained no personal checkout or Cargo
registry paths.

An extracted, non-installed package root passed these live checks against Niri
26.04:

- the Niri configuration and Waybar JSON/CSS contract validated;
- every public command and private helper retained its intended executable
  mode;
- a temporary `hollow-niri` daemon answered its health probe and queried the
  compositor without changing window state;
- 5,000 native planner/IPC benchmark iterations measured 23 microseconds p50,
  28 microseconds p95, and 262 microseconds maximum on this workstation;
- signed, temporarily extracted Waybar packages rendered the shipped
  `hollow-grove-top` namespace as a 34-pixel top layer, then exited cleanly.

The workstation still lacks installed Waybar, swaylock, Arch `devtools`, and
`namcap`. Root authentication was deliberately not passed through automation.
The next operator-owned gate is:

```bash
sudo pacman -S --needed devtools namcap
cd packaging/hollow-grove-general
extra-x86_64-build
namcap PKGBUILD hollow-grove-general-*.pkg.tar.zst
```

After that succeeds, install the package only in a disposable VM or test user
and perform the complete login, portal, lock, suspend, audio, network, and
upgrade-preservation checklist before changing a daily-use profile.

## Work remaining before ArchISO inclusion

1. Build the pinned `hollow-grove-general` recipe in an Arch clean chroot,
   run package linting, compare an independent rebuild, and record the Rust
   architecture matrix.
2. Decide which reasonable desktop applications belong in the base ISO and
   which remain discoverable optional bindings. Resolve each selected package
   against the exact official Arch snapshot or a signed project repository.
3. Create an install-ready `hollow-grove-hueman` package. Its Godot assets must
   be immutable under `/usr/share`, mutable artifacts must use XDG state/data,
   and its viewport must follow the active output rather than a fixed
   workstation resolution.
4. Decide whether the nested River workflow merits a separately maintained
   `hollow-grove-river-scope-addon`; do not place the personal development
   stack in the base image.
5. Add the package to an ArchISO profile overlay and
   `packages.x86_64`. Configure the Niri session entry, portals, PolicyKit
   agent, NetworkManager, and optional Bluetooth enablement.
6. Add Archinstall profile handling so the created user receives `/etc/skel`
   defaults without overwriting an existing user's configuration.
7. Add first-run UI or documentation for weather location, wallpaper choice,
   optional application groups, and post-install machine-local display setup.
8. Test signed packages and the profile in clean UEFI virtual machines and on
   representative desktop/laptop, single/multiple-monitor, and multiple-GPU
   hardware. Verify login, suspend/lock, portals, screen capture, clipboard,
   audio, network, Bluetooth, brightness-with-and-without-backlight, and every
   binding fallback.
9. Add upgrade tests proving package updates preserve user-owned files and
   never replace machine-local output configuration.
10. Produce the installation media, checksums, signatures, SBOM, and recovery
    documentation in a later explicitly authorized ISO build pass.

## Archinstall boundary

Archinstall should select the General Edition package and services; it should
not generate personal settings. Machine-specific output configuration, if
desired, belongs to a separate post-install probe whose result is user-owned
and reviewable.

## Current blockers

The profile itself is reviewable and package-shaped. ISO inclusion is blocked
only by the intentionally deferred package/installer work above, particularly
the Hueman application package and the final official-versus-external package
policy. No ISO has been built in this pass.
