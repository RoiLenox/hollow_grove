# General Edition Sanitization Report

## Method

The active Niri configuration was inspected directly. Every local command it
referenced was traced into its script, wrapper, user service, configuration, or
project file where available. Transitive dependencies were reviewed for fixed
paths, user state, hardware assumptions, credentials, location, and
checkout-relative behavior.

Sensitive values are described by category and source location below; they are
not reproduced.

## Removed or generalized values

| Source | Personal or machine-specific state found | General Edition treatment |
| --- | --- | --- |
| Active Niri output block | Full monitor identity including unique serial, forced mode, refresh, scale, and focus | Entire output block omitted; Niri discovers displays |
| Active Niri environment | GPU-vendor-specific VA-API and VDPAU driver selection | Removed; only toolkit-neutral Wayland variables remain |
| Active Niri startup | Absolute personal helper paths | Replaced by installed `/usr/bin` and `/usr/lib/hollow-grove` commands |
| Active Niri startup | Fixed laptop backlight device and startup percentage | Removed; brightness is unchanged at startup |
| Active Niri startup | Automatic machine-specific output manager | Removed from General Edition startup |
| Active Niri weather binding | Fixed geographic coordinates | Replaced by blank user-owned settings and an explanatory fallback |
| Output-manager configuration | Fixed connector, mode, refresh, scale, and placement | Not copied |
| Personal Niri service | Fixed PCI graphics device and workstation description | Not copied; the package supplies only a hardware-neutral movement broker unit |
| Personal Walker/Elephant services | Personal home, XDG, PATH, and user-local binary assumptions | Not copied; optional services start only if a package supplies a unit |
| Personal idle/lock service | Absolute personal lock-helper path | Not copied; portable lock helper uses installed commands |
| Personal tiling guard | Mutated the active personal Niri configuration | Excluded; distribution defaults are never rewritten at session startup |
| Lock-screen fallback | Workstation telemetry, a GPU sensor path, personal alias, and fixed-size generated artwork | Excluded; portable helper invokes a normal locker |
| Wallpaper helper and symlink | Personal image path and media | No image copied; XDG setting or neutral color is used |
| Kitty configuration | Fixed window geometry, temporary-directory assumptions, and personal appearance | Not copied |
| Firefox wrapper | Personal browser routing/profile behavior | Not copied; bindings invoke distribution browser commands |
| Outlook binding | Absent personal wrapper and implied mail/web state | Excluded; role remains an explicit unavailable integration |
| AudioTube wrapper | User-local patched executable, device-specific audio workaround, and library paths | Replaced by a neutral optional launcher |
| Ardour and LMMS wrappers | Workstation process/audio parameters | Replaced by neutral optional launchers |
| River scope helpers | Personal development commands, transitive GPU override, and local project utilities | Excluded from base; reserved for a future addon |
| AI command bindings | User-local package links and possible account state | Only command roles retained; no package state or credentials copied |
| Hueman bindings | Incorrect personal checkout path | Replaced with the stable installed `hollow-hueman` command |
| Hueman scripts/project | Repository-relative writable artifacts, repository-root discovery, and fixed ultra-high-resolution viewport | Project not copied; separate install-ready package remains required |

## Data deliberately not copied

- Home-directory contents, browser profiles, shell history, documents, media,
  mail configuration, and application databases.
- Wi-Fi profiles, NetworkManager connection secrets, machine IDs, or host
  identity.
- SSH keys/configuration, GPG keyrings, Git hosting authentication, API tokens,
  assistant credentials, and credential stores.
- Motherboard, fan, GPU, backlight, monitor, or workstation tuning.
- Personal systemd user units and user-local executables; the package-owned,
  hardware-neutral `hollow-niri.service` is not derived from either.
- Any assumption that a Hollow Grove checkout exists beneath the installed
  user's home directory.

## Portable replacements

- New-user files live only under `/etc/skel/.config/`.
- Distribution helpers live beneath `/usr/lib/hollow-grove/bin/`.
- The native movement broker uses Niri's session socket supplied with each
  command and retains no personal path, monitor, or GPU identity.
- Public commands are limited to `hollow-keyboard`, `hollow-launch`, and
  `hollow-hueman`, plus the account-neutral `hollow-shell` desktop seam.
- The Waybar profile contains only distribution colors, command roles, and
  generic system-state modules; it contains no account, host, output, or
  hardware identity.
- User settings, data, state, and runtime locks resolve through XDG paths with
  conventional home-directory fallbacks.
- Weather remains disabled until the user supplies a location.
- Brightness controls ask `brightnessctl` to choose supported hardware and
  report failure without selecting a device.
- Graphics and displays are selected by Niri, Mesa, and the installed system.

## Validation requirement

The final review runs both exact-source-value searches and broader credential,
private-key, personal-home, GPU-driver, display-mode, and backlight-pattern
searches over this packaging tree. A match is a release blocker unless it is a
generic explanatory term in these reports.
