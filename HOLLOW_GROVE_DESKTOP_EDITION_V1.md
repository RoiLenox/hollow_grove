# Hollow Grove Desktop Edition V1

Status: **IMPLEMENTED DISTRIBUTION PROFILE**

Hollow Grove Desktop is a clean, Niri-first Arch Linux environment. It borrows
no product identity from another operating system. Its coherence comes from a
small shell, one visual constitution, predictable navigation, and strict
separation between the base distribution and optional applications.

## Shell contract

```text
Hollow Grove top bar
├── Grove entrance       Walker application and command discovery
├── Spaces               Niri workspace state
├── Active context       focused-window title
└── System state         network, sound, power, time, and tray

Niri overview            window and workspace overview
Hollow keyboard          KEY = WHAT · MODIFIER = HOW
Hollow shell command     stable launcher, overview, and diagnostic actions
```

The base does not ship a second panel, desktop icon manager, widget engine,
dock, file-indexing daemon, or parallel settings shell. Walker remains the one
application entrance. Niri remains the one window and workspace authority.

## Visual identity

The shell uses the constitutional colors already owned by Hollow Grove:

- Oxford Blue `#0A1024` — outline and deepest shell surface
- Midnight Blue `#121B38` — primary bar surface
- Space Cadet `#1C2A52` — raised and hover surface
- Flynt Gunmetal `#242C42` — active operational surface
- Flynt Powder Blue `#A7B6D9` — primary text
- Flynt Tropical Indigo `#7F8BC4` — authority and active-space accent
- Glaüshouse Eton Blue `#78C9A4` — healthy/connected state
- Sandmanor Blush `#E07A86` — warning/urgent state

The profile uses no Apple marks, copied assets, or imitation controls.

## Distribution boundary

The General Edition package owns reproducible defaults under `/etc`, `/usr`,
and `/etc/skel`. Personal wallpaper, accounts, browser state, mail, credentials,
monitor geometry, and hardware tuning remain user- or machine-owned.

Base shell components are deliberately small:

- Niri for composition and Spaces;
- Waybar for the single top bar;
- Walker for discovery;
- Mako for notifications;
- Swaybg and Swaylock for wallpaper and lock state;
- the project-owned `hollow-shell`, `hollow-keyboard`, `hollow-launch`, and
  `hollow-niri` commands.

Applications remain replaceable fruit behind stable roles. Missing optional
software produces a clear notification instead of a broken binding.

## Installed paths

| Path | Purpose |
| --- | --- |
| `/etc/xdg/hollow-grove/waybar/config.jsonc` | distribution top-bar structure |
| `/etc/xdg/hollow-grove/waybar/style.css` | constitutional shell palette |
| `/usr/bin/hollow-shell` | stable public shell actions and doctor |
| `/usr/lib/hollow-grove/bin/start-shell` | idempotent Waybar startup with user overrides |
| `/etc/skel/.config/niri/config.kdl` | Niri-first new-user session |

User overrides may be placed in
`$XDG_CONFIG_HOME/hollow-grove/waybar/` without editing distribution files.

## Acceptance rules

1. One top bar, one launcher, one compositor, and one keyboard grammar.
2. The shell starts safely when optional state is absent and refuses duplicate
   bar processes.
3. The base contains no credentials, personal paths, fixed monitor output, or
   vendor-specific graphics policy.
4. The constitutional palette is reused rather than duplicated with unrelated
   colors.
5. Optional applications stay optional and discoverable.
6. `hollow-shell doctor`, `hollow-keyboard doctor`, and `hollow-niri doctor`
   expose the operational seams.
