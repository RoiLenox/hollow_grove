# Hollow Grove General Edition ISO Readiness

## Ready in this pass

- The new-user Niri profile is separate from the active personal
  configuration.
- The left-hand application/domain grid, right-hand focus/move/reshape/scope
  model, HJKL navigation, workspace bindings, column proportions, wrap motion,
  screenshots, media, audio, brightness, network, Bluetooth, Hollow Grove, and
  Hueman concepts are preserved.
- No output, resolution, refresh rate, scale, arrangement, GPU vendor, DRM
  device, or backlight device is forced.
- Optional applications fail through a named notification instead of blocking
  Niri startup.
- Wallpaper and weather use blank new-user settings with safe fallbacks.
- Installed commands and XDG paths replace personal paths and repository
  checkout assumptions.

This work is distribution packaging only. It changes no Hollow Grove canon,
constitutional architecture, gameplay system, institution, lineage, or
metaphysics.

## Work remaining before ArchISO inclusion

1. Create and review a PKGBUILD for `hollow-grove-general`, including licenses,
   checksums, ownership, reproducible build settings, and installation of these
   reports under the package documentation directory.
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
