# Hollow Grove General Edition Package Manifest

## Package boundary

This tree is the source payload for the `hollow-grove-general` Arch package.
It contains the portable Niri profile, distribution defaults, a
launcher contract, and the private helpers needed to preserve the keyboard
model. It does not contain an ISO, a personal home directory, constitutional
documents, the Hueman Godot project, or machine-specific configuration.

Package availability and names must be rechecked against the upstream Arch
repositories when the PKGBUILD and ArchISO package list are frozen. The source
workstation uses additional repositories, so a package being installed there
is not treated as proof that it is in an official Arch repository.

## Files

| Installed path | Mode | Purpose |
| --- | ---: | --- |
| `/etc/hollow-grove/applications.toml` | 0644 | Machine-readable application-role and source inventory |
| `/etc/hollow-grove/general-edition.toml` | 0644 | Distribution-owned profile metadata and path policy |
| `/etc/hollow-grove/keyboard-grove.json` | 0644 | Official A–Z WHAT vocabulary, HOW grammar, and keyboard ontology projection |
| `/etc/xdg/hollow-grove/waybar/config.jsonc` | 0644 | Single Hollow Grove top-bar structure |
| `/etc/xdg/hollow-grove/waybar/style.css` | 0644 | Constitutional desktop palette and restrained shell styling |
| `/etc/skel/.config/niri/config.kdl` | 0644 | Clean hardware-neutral new-user Niri profile |
| `/etc/skel/.config/hollow-grove/general-edition.toml` | 0644 | Blank user-owned wallpaper and weather settings |
| `/usr/bin/hollow-keyboard` | 0755 | Public keyboard discovery, search, map, and semantic-audit command |
| `/usr/bin/hollow-launch` | 0755 | Public missing-application resolver |
| `/usr/bin/hollow-hueman` | 0755 | Public stable Hueman command |
| `/usr/bin/hollow-shell` | 0755 | Stable desktop actions and shell diagnostics |
| `/usr/lib/hollow-grove/bin/*` | 0755 | Private compositor and application helpers listed below |
| `/usr/lib/systemd/user/hollow-niri.service` | 0644 | Resident bounded command broker for low-latency Niri movement |
| `/usr/share/hollow-grove/general-edition/README.md` | 0644 | Installed integration contract |
| `/usr/share/hollow-grove/general-edition/hollow-grove-keyboard-map.svg` | 0644 | Generated visual A–Z and modifier cheat sheet with landmarks, plants, and tools |
| `/usr/share/applications/hollow-grove-keyboard.desktop` | 0644 | Desktop entry for the visual keyboard map |

The four Markdown reports at the package-tree root are review and packaging
records installed under `/usr/share/doc/hollow-grove-general/`.

## Reproducible source boundary

The PKGBUILD pins one pushed Hollow Grove commit and the SHA-256 digest of its
GitHub source archive. Build and package functions consume that extracted
archive rather than the mutable source checkout. `Cargo.lock` fixes the Rust
dependency graph, while `check()` runs both the native `hollow-niri` tests and
the complete keyboard/Niri/Waybar package verifier before packaging.

The remaining reproducibility gate is a successful Arch clean-chroot build
against a recorded repository snapshot, followed by package linting and an
independent rebuild comparison.

`generate-keyboard-grove-map.py` and `verify-keyboard-grove.sh` are source-tree
release tools rather than installed commands. The generator renders the
review and installed SVG from the registry and constitutional palette. The
verifier rejects stale maps, validates the registry and portable Niri profile,
checks the public `hollow-keyboard` projections, and checks any additional
profile paths supplied as arguments.

## Required base packages

These are intended base-image dependencies from the official Arch
repositories:

| Package | Reason |
| --- | --- |
| `niri` | Compositor and configuration validator |
| `waybar` | One distribution-owned top bar for Spaces, context, and system state |
| `bash`, `coreutils`, `jq` | Portable helper runtime and keyboard/launcher JSON processing |
| `kitty` | Default terminal and terminal-hosted bindings |
| `systemd`, `dbus` | User services and session-environment propagation |
| `polkit-gnome` | Graphical privilege authentication |
| `xdg-utils` | Opening the installed visual keyboard map |
| `xdg-desktop-portal-gnome`, `xdg-desktop-portal-gtk` | Wayland desktop portals |
| `xwayland-satellite` | Optional X11 compatibility discovered and started by Niri itself |
| `mako`, `libnotify` | Notification daemon and clear fallback notices |
| `swaylock`, `swaybg` | Lock screen and neutral wallpaper |
| `wireplumber` | `wpctl` audio controls |
| `wl-clipboard`, `cliphist` | Clipboard capture and history |
| `grim`, `slurp` | Screenshot-to-clipboard helpers |
| `brightnessctl` | Device-agnostic, failure-safe brightness keys |
| `playerctl` | Media transport keys |
| `networkmanager`, `network-manager-applet` | Network TUI, CLI, and connection editor |
| `bluez`, `bluez-utils`, `blueman` | Bluetooth CLI and graphical controls |
| `udisks2` | Storage status binding |
| `curl` | Location-configured current-weather request |

The base image must also include a complete Wayland graphics stack selected by
Arch and Mesa for the detected hardware. This profile requests no vendor
driver.

## Hollow Grove packages

| Package | Status | Contents |
| --- | --- | --- |
| `hollow-grove-general` | This tree | Niri defaults, launchers, helpers, reports |
| `hollow-grove-hueman` | Required follow-up | Install-ready Hueman launcher/assets using XDG state and no checkout |
| `hollow-grove-river-scope-addon` | Optional, not yet defined | Nested River development helpers if that workflow is retained |

`hollow-hueman` is safe to ship before the application package: it explains
that Hueman is unavailable instead of assuming a source checkout.

## Reasonable general desktop applications

These are suitable image-profile choices, but need not all be in a minimal
installation:

- Firefox or Chromium; Helix, Lapce, Neovim, or Micro.
- Obsidian, Apostrophe, Ghostwriter, or MarkText.
- LibreOffice; Yazi, Thunar, Dolphin, or PCManFM-Qt.
- Btop, `ncdu`, NVTOP, and Battop.
- Audacity, LMMS, Cmus, AudioTube, Tauon, and OBS Studio.
- imv, Loupe, Gwenview, qView, darktable, RawTherapee, digiKam, or Shotwell.
- FreeTube, VLC, mpv, Haruna, Croc, and Magic Wormhole.

The exact package source for every selected application must be resolved in the
ArchISO package policy. `/etc/hollow-grove/applications.toml` is the canonical
role inventory for that work.

## Packages requiring another source or explicit review

These bindings are preserved but are not base-image dependencies:

| Software or role | Expected source |
| --- | --- |
| Walker and Elephant | AUR or a signed Hollow Grove package repository |
| Tracktion Waveform and REAPER | Vendor/AUR packaging with license review |
| FreeTube, LocalSend, Planify, Battop, Tauon, and proprietary browsers | AUR or reviewed external packaging where not in the selected Arch snapshot |
| youtui, Fluent Reader, qView, and PeaZip | Reviewed external/user package |
| Codex CLI, Gemini CLI, OpenCode, Ollama, and AIChat | Vendor or reviewed external package; credentials remain user-owned |
| Outlook role | No distributable integration selected |
| Nested River scope | Future optional project package |

Emulators, audio tools, task tools, and assistants must be resolved
individually because official-repository availability can change between Arch
snapshots.

## Helper audit

Classification numbers match the requested audit:
1 required for Hollow Grove/base behavior; 2 generic convenience worth
packaging; 3 optional application integration; 4 personal-only; 5 obsolete or
unused.

| Referenced helper | Class | General Edition decision |
| --- | ---: | --- |
| `start-wallpaper` | 2 | Packaged; reads an XDG user setting and falls back to a neutral solid color |
| `lock-session` | 1 | Packaged; tries portable lockers and clearly reports if none is installed |
| `river-scope-close` | 3 | Excluded from base; binding retained through a missing-addon notification |
| `river-scope-spawn` | 3 | Excluded from base; binding retained through a missing-addon notification |
| `river-kitty` | 3 | Excluded from base; original depends on the local River development stack |
| `river-scope-probe` | 3 | Excluded from base; binding retained through a missing-addon notification |
| `walker-toggle-niri` | 2 | Adapted and packaged; Walker itself remains optional |
| `ardour9-niri` | 3 | Adapted as `ardour-niri`; no workstation audio tuning is imposed |
| `audiotube-niri` | 3 | Adapted as `audiotube-niri`; local patched binaries and audio-device workarounds removed |
| `outlook` | 4 | Referenced file was absent; no personal web/mail profile copied |
| `niri-wrap-move` | 2 | Retained as a compatibility entrance to the native `hollow-niri` broker |
| `lmms-niri` | 3 | Adapted as `lmms-niri`; fixed PipeWire latency removed |
| `volume-notify` | 1 | Adapted and packaged with `volume-control` |
| `brightness-notify` | 1 | Adapted and packaged with generic `brightness-control` |
| `run-hueman-godot.sh` | 1 | Replaced by installed `hollow-hueman`; checkout-bound original not copied |
| `rebind-hueman-pair.sh` | 1 | Replaced by XDG-state `hollow-hueman rebind` contract |
| `clear-hueman-pair.sh` | 1 | Replaced by XDG-state `hollow-hueman clear` contract |
| `step-hueman-pair-spread.sh` | 1 | Replaced by XDG-state `hollow-hueman step-spread` contract |

The old Hueman path referenced by the personal Niri profile did not identify
the active checkout and is treated as obsolete (class 5). Neither path is
carried into this profile.

## Other packaged helpers

- `hollow-niri`: native persistent IPC broker for height-preserving wrap
  movement and percentage-correct anchored height snaps. It exposes `doctor`,
  `benchmark`, and `renderer-report`
  diagnostics without putting JSON parsing or shell processes on the hot path.
- `notify`: stderr plus desktop-notification reporting.
- `start-shell`: idempotently starts the one Waybar instance, preferring
  user-owned XDG overrides when present.
- `session-environment`: imports neutral Wayland session variables.
- `start-optional-service`: starts a user unit only when it exists.
- `clipboard-watch`: starts `cliphist` only when both commands exist.
- `screenshot-copy`: region or screen image copied to the clipboard.
- `power-profile`: safe wrapper for supported power profiles.

No personal systemd user unit is copied. The package supplies only the
distribution-owned `hollow-niri.service` required by its native movement
broker.

## Assets and configuration

The visual defaults are the constitutional Hollow Grove palette and the single
top-bar contract. No personal wallpaper or lock-screen image is included. No
monitor, GPU, backlight, browser, mail, credential, desktop-icon, dock, or
parallel settings-shell configuration is included.
