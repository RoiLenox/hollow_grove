# Hollow Grove General Edition

This directory is the shared-data root for the portable Niri desktop profile.
The base package deliberately contains no personal wallpaper, browser profile,
mail state, credentials, hardware output file, or repository checkout.

## Installed interfaces

- `/usr/bin/hollow-keyboard` explores, searches, displays, and audits the
  constitutional keyboard projection.
- `/usr/bin/hollow-launch` checks optional commands and reports clear failures.
- `/usr/bin/hollow-hueman` is the stable desktop seam into the independently
  installed Hollow Grove Core and Hueman client.
- `/usr/bin/hollow-shell` exposes the launcher, Niri overview, common desktop
  roles, The Marquise TUI entrance, and a compact shell doctor.
- `/usr/lib/hollow-grove/bin/` contains private desktop helpers.
- `/usr/lib/hollow-grove/bin/hollow-niri` is the native, bounded Niri command
  broker used by repeatable movement bindings.
- `/etc/hollow-grove/` contains distribution-owned defaults and the application
  inventory.
- `/etc/hollow-grove/keyboard-grove.json` records the complete A–Z WHAT
  vocabulary, HOW modifier grammar, and the distinction between landmarks,
  plants, and tools.
- `/etc/skel/.config/` contains clean defaults copied to newly created users.
- `/etc/xdg/hollow-grove/waybar/` contains the single Hollow Grove top bar and
  its constitutional palette. User overrides belong under
  `$XDG_CONFIG_HOME/hollow-grove/waybar/`.

The separately packaged Core installs `hollow-grove-with-hueman`; the separately
packaged Hueman client installs `hueman`. General Edition delegates launching
to that public seam and embeds neither package. Presentation pair state belongs
under `$XDG_STATE_HOME/hueman`; authoritative game state remains beneath the
Hollow Grove XDG namespace.

Machine-specific output configuration is intentionally outside this package.

The desktop keeps one compositor (Niri), one launcher (Walker), and one top bar
(Waybar). It deliberately ships no dock, desktop icons, widget engine, or
parallel settings shell. Run `hollow-shell doctor` to inspect this boundary.

The generated [keyboard Grove map](hollow-grove-keyboard-map.svg) provides a
visual guide to the profile without becoming a second binding source.

Run `hollow-keyboard menu` for Walker-based discovery,
`hollow-keyboard show` for the visual map, or `hollow-keyboard doctor` to audit
the active profile against the registry.

Run `hollow-niri doctor` to verify the resident broker and direct compositor
IPC, `hollow-niri benchmark` to measure non-mutating window-snapshot latency,
or `hollow-niri renderer-report` to summarize renderer errors from the current
boot. Movement commands use a persistent Niri connection, contain at most
eight pending inputs, and discard stale overflow instead of replaying movement
after the key is released. Ordinary horizontal conveyor movement preserves the
window's current logical height; anchored stretch commands use Niri's native
percentage-point IPC units.

The keyboard registry is an operational projection of
`HOLLOW_GROVE_KEYBOARD_CONSTITUTION_V1.md`; it does not create game or House
law. Every non-movement letter is a stable domain plant; H/J/K/L remain the
movement tool. Installed applications are replaceable fruit and unavailable
branches remain empty.
