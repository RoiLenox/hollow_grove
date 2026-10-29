# Hollow Grove Keyboard Standard V2

Status: active operational standard

This standard refines the V1 implementation without changing the governing
`Key = WHAT · Modifier = HOW` constitution. H, J, K, and L remain movement and
layout tools; Space remains the Glaüshouse room/layout landmark.

## Shared quarter-height contract

Space and H/J/K/L operate on one canonical height ladder:

```text
25% → 50% → 75% → 100%
```

| Binding | Operation |
|---|---|
| `Super+Ctrl+J` | Cycle the ladder, anchored downward |
| `Super+Ctrl+K` | Cycle the ladder, anchored upward |
| `Super+Ctrl+Shift+Space` | Same upward cycle as `Super+Ctrl+K` |
| `Super+Alt+Shift+Space` | Same downward cycle as `Super+Ctrl+J` |
| `Super+Alt+Shift+H` | Select 25% directly, anchored downward |
| `Super+Alt+Shift+J` | Select 50% directly, anchored downward |
| `Super+Alt+Shift+K` | Select 75% directly, anchored downward |
| `Super+Alt+Shift+L` | Select 100% directly, anchored downward |
| `Super+Ctrl+Alt+Shift+Space` | Toggle the 25% bookmark and 75% focus stages |

The cycle and direct-selection routes must use these same four stages in the
personal Niri profile and the General Edition portable profile. Machine-local
paths may differ; behavior may not.

## Verification

Validate both Niri profiles and run
`packaging/hollow-grove-general/verify-keyboard-grove.sh`. The verifier rejects
any future drift between the Space cycle and H/J/K/L direct selections.

## A / local AI amendment — 2026-10-10

A now means AI / Artificial Intelligence, as adopted in the keyboard
constitution. Mod+A opens local chat, Shift selects a reasoning-oriented
branch, Ctrl inspects/selects installed models, and Alt opens an alternate
local chat model. All use `hollow-ai` with loopback-only Ollama and refuse
to download missing models. Personal model selections live in
`~/.config/hollow-grove/local-ai.json`; portable defaults require models
to be installed separately. All movement bindings remain unchanged.

Personal ordering refinement: Qwen3 8B is the established everyday Primary,
DeepSeek R1 8B remains Adjacent reasoning, Ctrl remains model inspection, and
Hollow Grove is the Alternate/work-in-progress option. The model config and
personal overlay titles record this preference; portable defaults remain
independent of the user's experimental model.

## C / Code amendment — 2026-10-10

C now means Code / Source code under the user's adopted amendment.

| Binding | HOW | Workflow |
|---|---|---|
| Super+C | Primary | Codex coding agent |
| Super+Shift+C | Adjacent | Graphical coding in Zed |
| Super+Ctrl+C | Focused | Precise terminal editing in Helix |
| Super+Alt+C | Alternate | OpenCode coding agent |

D / Develop and E / Edit remain unchanged pending their own reviews, so some
launchers temporarily overlap. All movement remains unchanged. The personal
Controls menu preserves Himalaya and Discord as communication routes.

## Current A / automatic assistance and C / manual coding

| Binding | Route |
|---|---|
| Mod+A | Codex coding agent |
| Mod+Shift+A | OpenCode coding agent |
| Mod+Ctrl+A | AI tools and local-model chooser |
| Mod+Alt+A | Hollow Grove local chat (work in progress) |
| Mod+C | Helix terminal editor |
| Mod+Shift+C | Lapce manual source editor |
| Mod+Ctrl+C | Zed graphical editor |
| Mod+Alt+C | Unbound |

The agents retain existing provider and permission configurations. Automatic
refers to AI-assisted work, not automatic approval of agent actions. The local
model helper remains accessible inside the Ctrl+A chooser and directly on
Alt+A; its primary/adjacent chat modes are also available by command.
D and E still have their previous routes pending review.

## D / Downloads — accepted cluster

D now means Downloads / Acquire files, replacing Develop under the user's
accepted amendment. D acquires files and F manages them afterward.

| Binding | HOW | Route |
|---|---|---|
| Mod+D | Primary | qBittorrent graphical torrent workflow |
| Mod+Shift+D | Adjacent | rTorrent terminal torrent workflow |
| Mod+Ctrl+D | Focused | Unbound — distinct queue/transfer control reserved |
| Mod+Alt+D | Alternate | Unbound — another download method reserved |

Both active tools are already installed. The Focused and Alternate proposals
are workflow slots rather than instructions to invent tools or bind an
interactive downloader without defining its workflow. No downloads were
started and existing torrent configurations were not changed.

## E / Email — accepted cluster

E now means Email / Mail, replacing Edit under the user's adopted amendment.

| Binding | HOW | Route |
|---|---|---|
| Mod+E | Primary | Thunderbird graphical email |
| Mod+Shift+E | Adjacent | Himalaya terminal email |
| Mod+Ctrl+E | Focused | Himalaya account list |
| Mod+Alt+E | Alternate | Unbound — another email client/service reserved |

Himalaya runs in a held Kitty window so its setup prompts or output remain
visible. Account inspection uses `himalaya account list`; it does not initiate
a send action. Existing mail profiles/configurations are left intact. No mail
was opened, sent, deleted, or configured during keyboard verification.

## F / Files — graphical adjacent route

The user's established Primary route remains Yazi on Mod+F. Mod+Shift+F
now opens Nautilus (GNOME Files), the graphical Adjacent file workflow.
Nautilus is already installed. Personal Ctrl/Alt branches remain unbound;
all other bindings and movement remain unchanged. This is a fruit replacement
within Files, not a WHAT/HOW amendment.

## I / Inspect — Look inside

I now means Inspect, replacing Images under the user's adopted amendment.

| Binding | HOW | Route |
|---|---|---|
| Mod+I | Primary | Htop live process view |
| Mod+Shift+I | Adjacent | Fastfetch system snapshot |
| Mod+Ctrl+I | Focused | Sensors hardware readings |
| Mod+Alt+I | Alternate | Journalctl user-session logs |

Snapshot and sensor output stays visible in held Kitty windows. Journalctl
opens the current boot's last 200 user-session records with its usual pager.
All tools are already installed; no packages were added. Utilities retains
its prior routes until that family is reviewed, including the htop alias.
imv remains launcher-accessible. All non-I bindings, including movement,
remain unchanged.

## M / Music — Spotify with Spicetify and spotify-tui

| Binding | HOW | Route |
|---|---|---|
| Mod+M | Primary | AudioTube |
| Mod+Shift+M | Adjacent | youtui |
| Mod+Ctrl+M | Focused | Spotify + Spicetify with adblock.js |
| Mod+Alt+M | Alternate | spotify-tui (`spt`) in Kitty |

The focused route uses `hollow-spotify`: it checks official client updates,
reapplies the configured Spicetify extensions when stock app archives return,
and launches Spotify in native Wayland mode. Original Spotify app archives
are backed up by Spicetify. Ad blocking is configured and injected; playback
and effectiveness have not been tested with a signed-in account.

The alternate route uses the user's selected spotify-tui, locally built from
v0.25.0 with updated compatible dependencies, removal of an obsolete unused
serialization feature, and the loopback OAuth callback fixed to
`http://127.0.0.1:8888/callback`. It requires the user's own Spotify developer
credentials and Premium for playback. It controls a separate active playback
device, such as Spotify Desktop; it does not stream audio itself.

Spicetify 2.45.3 is installed from its official verified release in the user's
home directory. The adblock extension is pinned to revision
`5da6cf1bb723f9efd0177f5fa4f4673b9e0c0936` of rxri/spicetify-extensions.
The generic package exposes optional commands without bundling these tools,
credentials, or Warren's user configuration. Music / HEAR and the HOW grammar
remain unchanged. The existing cmus-specific transport keys remain pending
separate review. All bindings outside Ctrl+M and Alt+M are preserved.

## N / Network — Connections

The user selected Network in place of News. Key = WHAT remains atomic:
N handles connections, I inspects the machine, and S provides general settings.

| Binding | HOW | Route |
|---|---|---|
| Mod+N | Primary | nmtui — Wi-Fi/Ethernet and connection activation |
| Mod+Shift+N | Adjacent | NetworkManager profile editor — profiles and VPN configuration |
| Mod+Ctrl+N | Focused | nmcli device diagnostics — addresses, routes, DNS and device state |
| Mod+Alt+N | Alternate | SSH destination prompt — host alias or user@host |

All tools are already installed. VPN support depends on the available
NetworkManager plugins and configured profiles; no VPN service is selected
or activated by this amendment. SSH waits for an explicitly entered host,
with blank input canceling. Diagnostics query state without changing it.

The former personal Ctrl+S nmtui route moves to N Primary. Settings retains
its general Controls menu, Bluetooth and audio routes; Ctrl+S is reserved
until that family is reviewed. The portable profile moves both dedicated
NetworkManager routes into N and leaves S Primary and Focused empty pending
a general Settings entrance. Network links in the broad personal Settings
menu remain discoverable. Movement, layout and all other families are preserved.

## P / Photos — View, organize, edit

The user kept P as Photos and installed digiKam and darktable to adopt the
proposed branch ordering. imv was already installed.

| Binding | HOW | Route |
|---|---|---|
| Mod+P | Primary | imv — quick photo viewing |
| Mod+Shift+P | Adjacent | digiKam — photo library organization |
| Mod+Ctrl+P | Focused | darktable — photo development and editing |
| Mod+Alt+P | Alternate | Unbound pending a distinct photo workflow |

Photos / Photography and the modifier grammar remain unchanged. This is an
application refinement, requiring no constitutional vocabulary amendment.
Office retains Writer, Calc, Impress and Draw verbatim. No photo collections
are imported and no application preferences are changed by the shortcuts.

## Q / Query — Find information

The user accepted a shared fzf interface for local information searches,
keeping Grave/Tilde application and window navigation unchanged.

| Binding | HOW | Route |
|---|---|---|
| Mod+Q | Primary | Fuzzy file-name search with bat previews |
| Mod+Shift+Q | Adjacent | Live literal text search using ripgrep and fzf |
| Mod+Ctrl+Q | Focused | Search indexed system manuals and read with man |
| Mod+Alt+Q | Alternate | Unbound pending another distinct search workflow |

Search starts in the user's home directory, respects hidden/ignored-file
rules, and skips binary contents. `hollow-query MODE --root DIRECTORY`
changes the search scope when called directly. Text queries are literal,
use smart-case matching, and show up to 2,000 results per query. Text results
open read-only at the matching line; file results use the existing default
application. PDF/Office document contents require a separate document
indexer and are outside this text-search implementation. No background
indexer is installed. man-db was installed and its manual index built.

Ctrl+A now opens the AI tool chooser: Gemini, AIChat, Ollama's launcher,
and local-model inspection/selection. Local controls remain accessible
through that chooser. Mod+A Codex, Shift+A OpenCode, and Alt+A Hollow Grove
remain unchanged; no provider or approval configuration is modified.
Query's memory hook is Find information. Walker and Anyrun retain both
Grave shortcuts verbatim. Office, movement and every other binding outside
Ctrl+A and the three Q branches are preserved.

## R / Record — Accepted and locked (2026-10-10)

The user explicitly accepted the current Record cluster and asked to lock it.
Preserve its meaning, branch ordering and existing launch commands in future
keyboard refinements unless the user explicitly requests a change to R.

| Binding | HOW | Personal route |
|---|---|---|
| Mod+R | Primary | Waveform 13 |
| Mod+Shift+R | Adjacent | Ardour 9 via the existing niri wrapper |
| Mod+Ctrl+R | Focused | Audacity |
| Mod+Alt+R | Alternate | REAPER |

Record covers audio recording and production; Music covers listening.
The portable profile retains the corresponding Waveform, Ardour, Audacity
and REAPER routes with its existing guarded launch paths. This records an
accepted implementation, with no binding or application-setting changes.

## S / Settings — niri configuration as Primary

At the user's request, Mod+S now opens the live niri configuration in Helix
inside Kitty. Primary means configuring the compositor directly.

| Binding | HOW | Personal route |
|---|---|---|
| Mod+S | Primary | Helix editing the live niri config |
| Mod+Shift+S | Adjacent | Blueman Bluetooth manager |
| Mod+Ctrl+S | Focused | Unbound |
| Mod+Alt+S | Alternate | WireMix audio settings |

The personal primary target is the existing live niri config.
The portable profile resolves `${XDG_CONFIG_HOME:-$HOME/.config}/niri/config.kdl`
at launch and retains its existing graphical PipeWire volume route on Alt+S.
The broader personal settings menu remains in Hollow Grove Controls via
Settings · System Controls and `hollow-grove-controls settings`.
Settings / Configure and all modifier meanings remain unchanged. Every
other binding is preserved, including the locked Record cluster, Office,
Grave navigation, and movement/layout controls.

## T / Time — Large clock

The user chose Time instead of Tasks and requested a large peaclock clock on
Mod+T. peaclock was already installed; no installation was needed.

| Binding | HOW | Route |
|---|---|---|
| Mod+T | Primary | Large responsive peaclock digital clock in Kitty |
| Mod+Shift+T | Adjacent | Unbound pending selection |
| Mod+Ctrl+T | Focused | Unbound pending selection |
| Mod+Alt+T | Alternate | Unbound pending selection |

The clock uses a dedicated `hollow-grove/time/clock.conf` under the user's XDG
config directory, with automatic sizing and aspect ratio, the date visible,
and seconds initially hidden for larger hours/minutes. The installed clock's
`s` key toggles seconds and `q` closes it. It uses the system's existing local
timezone. Normal niri window placement, movement and layout remain unchanged.
The broader default peaclock invocation in Controls retains its defaults.
Former Taskwarrior/Planify routes are removed from T; the programs remain
installed and available independently. Time's memory hook is Clock and timing.
Office, the locked Record cluster, Grave navigation and all non-T bindings
are preserved verbatim.

## T / Time — Accepted clock alternatives

The user installed GNOME Clocks, clock-tui and tty-clock after selecting them
for the remaining Time branches. The complete ordering is now:

| Binding | HOW | Route |
|---|---|---|
| Mod+T | Primary | Existing large peaclock digital display |
| Mod+Shift+T | Adjacent | GNOME Clocks — graphical world clocks, alarms and timing |
| Mod+Ctrl+T | Focused | clock-tui timing chooser — timer, stopwatch, date countdown |
| Mod+Alt+T | Alternate | tty-clock — bouncing retro display |

Ctrl+T prompts for a mode, then a duration or target where needed. Empty input
cancels; no timer-completion commands are added. `tclock` is resolved on PATH
or in the user's Cargo bin directory. Alt+T uses seconds, bold colors and a
blinking colon alongside rebound animation. Mod+T and its peaclock config are
preserved exactly. GNOME Clocks' settings, alarms and world-clock locations
remain under the user's control. Time / Clock and timing remains unchanged.
All non-T bindings, including Record, Office, navigation and movement/layout,
are preserved verbatim.

## U / Utilities — Meaning accepted (2026-10-10)

The user explicitly retained Utilities as U's meaning. Preserve Utilities /
System tools in future keyboard refinements unless the user requests a change.
The current personal routes remain unchanged:

| Binding | HOW | Route |
|---|---|---|
| Mod+U | Primary | htop system monitor |
| Mod+Shift+U | Adjacent | ncdu disk usage |
| Mod+Ctrl+U | Focused | Unbound (nvtop unavailable) |
| Mod+Alt+U | Alternate | battop battery status |

The portable profile retains its existing btop, ncdu, nvtop and battop routes.
This records the accepted meaning and preserves current commands; no bindings,
application settings, or package installations change.

## V / Videos — Local movie library

The user requested a movie/TV collection connected to the Downloads domain.
The local single-computer implementation uses Kodi.

| Binding | HOW | Route |
|---|---|---|
| Mod+V | Primary | Kodi local movie/TV library |
| Mod+Shift+V | Adjacent | Open the collection folders in the default file manager |
| Mod+Ctrl+V | Focused | Native local-video picker and mpv playback |
| Mod+Alt+V | Alternate | FreeTube online videos |

Kodi has Movies and TV Shows roots under the user's Videos folder, configured
with bundled TMDB metadata providers and startup scanning. Downloads is also
accessible as an unsorted source. Save movie/show downloads into the respective
collection folder for cataloging; Downloads remains available for direct
browsing. The personal folders were empty during setup, so no actual titles,
artwork or playback were imported/tested. Kodi launches windowed on Wayland.

The fresh Kodi profile is initialized with local sources; its video database
and GUI settings are backed up before catalog configuration. Existing profiles
remain user-managed. The local-file picker supports common video formats and
passes filenames without shell interpolation; cancellation starts no player.
Download clients and torrent paths remain unchanged, and no media is moved.
FreeTube moves from Primary to Alternate. Existing mpv rendering preferences
are retained. Shared cmus transport bindings remain pending a separate review.
Videos / WATCH and the HOW grammar remain consistent; all non-V bindings,
including Office, locked Record, Grave navigation and movement, are preserved.

## W / Wrappers — accepted Windows compatibility cluster

W means Wrappers / Windows compatibility. G opens games; W manages the
compatibility environments beneath Windows applications and Proton games.

| Binding | HOW | Route |
|---|---|---|
| Mod+W | Primary | Bottles compatibility hub |
| Mod+Shift+W | Adjacent | Pick an .exe/.msi, then select its bottle in Bottles |
| Mod+Ctrl+W | Focused | Select a bottle and open Wine Windows configuration |
| Mod+Alt+W | Alternate | Protontricks graphical Steam/Proton repair picker |

`hollow-wrappers` supports the official Bottles Flatpak or a native Bottles
installation. Flatpak executable selection uses document-portal file forwarding.
Esc/Cancel exits without running software. Ctrl selections preserve exact bottle
names and open `winecfg` in the selected environment, without using a global
default Wine prefix. Dependency installation remains in the main Bottles hub;
the installed release does not reliably open a specific bottle's GUI page from
its command line. On a fresh profile, finish Bottles setup and create a bottle
before using Ctrl+W. No Windows software or dependencies are installed merely by
pressing a launcher shortcut. Writing apps remain available in application search.
All bindings outside W are preserved verbatim.

## X / eXecute — accepted command and workflow cluster

| Binding | HOW | Route |
|---|---|---|
| Mod+X | Primary | Enter a command and execute it in the login shell |
| Mod+Shift+X | Adjacent | Preview and select an executable script |
| Mod+Ctrl+X | Focused | Select a project and then a Cargo/npm task |
| Mod+Alt+X | Alternate | Reserved |

All routes use `hollow-execute` inside held Kitty windows so output and the exit
status remain visible. Mod+X starts at the home directory, uses Fish on this
personal machine, and does not save entered commands to a history file. Blank
input or Ctrl+C cancels the prompt; Esc cancels either fuzzy picker.
The script picker lists executable shebang scripts in ~/.local/bin, ~/bin,
~/Projects/active, ~/Projects/systems/src, and ~/Projects/learning. Selected
scripts run from their containing directory. Project tasks are discovered in
the three Projects roots, with build caches, releases, backups, third-party
directories, and hidden subdirectories excluded. Cargo projects provide check,
build, test, and declared runnable binaries (including required feature flags).
npm projects expose their actual package.json scripts. Task execution starts in
the selected project's directory. Previewing and indexing only read definitions.
`--root DIRECTORY` can override the search roots; `--cwd DIRECTORY` sets the
command route's directory. Alt remains empty. Every non-X binding is preserved.

## Y and Z / final accepted clusters and shared-media consistency

| Binding | Route |
|---|---|
| Mod+Y | Start Syncthing on demand and open its dashboard |
| Mod+Shift+Y | Grsync folder synchronization |
| Mod+Ctrl+Y | Syncthing status and explicit start/stop/restart/login controls |
| Mod+Alt+Y | Reserved for cloud sync |
| Mod+Z | File Roller archive manager |
| Mod+Shift+Z | Select a folder, then choose the archive name and format |
| Mod+Ctrl+Z | Select an archive, then choose its extraction destination |
| Mod+Alt+Z | Select an archive and inspect its contents in a held terminal |

Syncthing's new profile starts without selected folders or paired remote devices;
existing configurations are preserved. Mod+Y starts its user service on demand.
Login autostart changes only when explicitly selected in Ctrl+Y. Device pairing
and folder selection remain user choices in the dashboard. Grsync opens its GUI
without starting a synchronization job. croc and Wormhole remain available from
application search/terminals. The archive pickers support literal paths, including
spaces and line breaks. Cancel exits without creating/extracting archives.
File Roller controls destination and overwrite dialogs; no force flag is used.
Alt+Z lists contents only and filters embedded terminal control characters.

Shared media Play/Pause/Previous/Next/Stop keys and Shift+Comma/Period now use
`hollow-media`, which activates playerctld on demand and uses playerctl to control
the most recently active MPRIS player. It falls back to an available supported
player if the proxy is unavailable. This replaces cmus-only personal commands.
Applications must expose MPRIS for these keys; mpv without an MPRIS plugin uses
its own playback keys. Every movement/layout, Office, and Record binding remains
byte-for-byte unchanged. The letter-by-letter review is complete.
