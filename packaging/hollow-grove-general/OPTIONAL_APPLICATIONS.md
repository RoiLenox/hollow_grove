# Optional Applications and Binding Fallbacks

## Fallback contract

`/usr/bin/hollow-launch` verifies every declared command before execution. If
anything is missing, it emits a visible notification and a matching stderr
message naming the role and missing command, then exits with status 127. Niri
continues running.

`hollow-keyboard doctor` audits the registry, generated map, configured
semantic titles, and currently available command requirements. Missing fruit
is reported as an availability warning rather than filled with an unrelated
application.

Private wrappers for Walker, Ardour, AudioTube, LMMS, Hueman, weather,
screenshots, brightness, audio, and power profiles apply the same principle.
Startup-only helpers either use a neutral fallback or exit successfully when
an optional service is absent.

`hollow-shell` is the stable public desktop seam. It keeps launcher, overview,
keyboard, terminal, file, browser, and Marquise actions behind one small
command rather than adding another settings or launcher framework.

## A–Z application grid

`Key = WHAT · Modifier = HOW`. Every non-movement letter is an admitted
domain plant. Primary, Adjacent, Focused, and Alternate branches stay inside
that domain; applications are replaceable fruit.

| Key | Domain | Portable fruit | When absent |
|---|---|---|---|
| A | AI | Codex, OpenCode, AI tools / local-model chooser, Hollow Grove model | The missing AI tool is named; no credentials are included |
| B | Browse | Firefox, Chromium, Vivaldi, Brave | The missing browser is named |
| C | Code | Helix, Lapce, Zed; Alternate empty | The missing source editor is named |
| D | Downloads | qBittorrent, rTorrent; Focused/Alternate empty | The missing downloader is named |
| E | Email | Thunderbird, Himalaya, account inspection; Alternate empty | No account or credential is included |
| F | Files | Yazi, Nautilus, Dolphin, PCManFM-Qt | The missing file manager is named |
| G | Games | Steam, RPCS3, Dolphin Emulator, RetroArch | The missing game frontend is named |
| H/J/K/L | Movement | Native Niri focus, move, reshape, and transport | No application fallback applies |
| I | Inspect | htop, Fastfetch, sensors, user-session logs | The missing inspection tool is named |
| M | Music | AudioTube, youtui, Spotify + Spicetify, spotify-tui (`spt`) | The missing music tool is named |
| N | Network | nmtui, NetworkManager connection editor, nmcli, OpenSSH | The missing network tool is named |
| O | Office | LibreOffice Writer, Calc, Impress, Draw | The missing suite is named |
| P | Photos | imv, digiKam, darktable; Alternate empty | The missing photography tool is named |
| Q | Query | fzf, ripgrep, bat, man-db; Alternate empty | The missing search tool is named |
| R | Record | Waveform, Ardour, Audacity, REAPER | The missing recorder or DAW is named |
| S | Settings | niri configuration in Helix, Blueman, PWVuControl; Focused empty | The missing settings surface is named |
| T | Time | peaclock, GNOME Clocks, clock-tui (`tclock`), tty-clock | The missing clock application is named |
| U | Utilities | btop, ncdu, nvtop, battop | The missing system utility is named |
| V | Videos | Kodi library, collection folders, mpv file picker, FreeTube | The missing library or video tool is named |
| W | Wrappers | Bottles (official Flatpak), Zenity, Protontricks, YAD | The helper checks Bottles and names missing compatibility tools |
| X | eXecute | Kitty, Python, fzf; Cargo/npm for relevant projects | Alt remains empty; unsupported task definitions are not offered |
| Y | sYnc | Syncthing, Grsync, service controls; Alternate empty | The missing synchronization tool is named |
| Z | Zip | File Roller; creation, extraction, bsdtar inspection | The missing archive tool is named |

Special families remain outside the letter grid: Return is Terminal, Grave is
Navigate, Backspace is History, Escape is Exit, and Space shapes usable room.

## Compositor and device bindings

| Keys | Role | Availability behavior |
| --- | --- | --- |
| `Mod+Space`, `Mod+Shift+Space` | Glaüshouse usable room forward/backward | Native Niri preset-width actions; official keyboard Step 2 |
| `Mod+Ctrl+J/K`, `Mod+Ctrl+Shift+Space`, `Mod+Alt+Shift+Space` | Quarter-height flow down/up | All cycle the same 25/50/75/100 ladder through `hollow-niri` |
| `Mod+Alt+Shift+H/J/K/L` | Direct quarter-height selection | Select 25/50/75/100 respectively on the same bottom-anchored ladder |
| `Mod+Alt+Escape`, `Mod+Alt+Shift+Return`, `Mod+Alt+Slash` | Nested River development | Retained for muscle memory; reports that the optional addon commands are missing |
| `Mod+Shift+Comma`, `Mod+Shift+Period` | Media previous/next | Base `playerctl`; missing command is reported |
| `Mod+Shift+Equal` | OBS Studio | Missing OBS is reported |
| `Mod+Ctrl+Minus`, `Mod+Ctrl+Equal` | Screenshot to clipboard | Missing Grim, Slurp, or `wl-copy` is reported |
| Audio hardware keys | WirePlumber volume/mute | Missing `wpctl` is reported; microphone mute does not produce a sink OSD |
| Brightness hardware keys | Generic brightness change | Does not name a backlight device; unsupported hardware produces a clear notification |
| `Print` and the Minus/Equal family | Niri screenshots and casting | Native Niri actions remain available independently of optional applications |

## Startup behavior

- `start-shell` launches one Waybar instance and exits cleanly if one already
  exists. A user-owned XDG config or style overrides the distribution file
  independently.
- A missing Mako or Elephant user service is ignored without failing the
  compositor.
- Niri discovers XWayland Satellite automatically when installed; no duplicate
  startup process is created. A missing PolicyKit agent is reported, but Niri
  starts.
- A missing wallpaper image falls back to the neutral background color.
- Missing clipboard tools simply disable clipboard history.
- No optional command is evaluated while Niri parses its configuration.

Application absence is therefore a feature-level limitation, not a compositor
startup dependency.

### Spotify customization

The focused Music route uses `spotify-launcher`, `spicetify`, and
`hollow-spotify`. Configure Spicetify for the user-owned launcher installation
and install the adblock extension before using this route. The helper checks
for official client updates and reapplies the configured extensions when an
update restores `Apps/xpui.spa`; it does not restart an existing client.
The alternate route runs `spt` from spotify-tui. It requires Spotify developer
credentials, Premium for playback, and a separate active playback device such
as the desktop client. Use the explicit loopback redirect
`http://127.0.0.1:8888/callback`, as required by Spotify's current OAuth rules.
These optional tools are not bundled or authenticated by the general package.

### Query search

`hollow-query files`, `text`, and `manuals` provide a shared fzf interface.
Search defaults to the user's home directory; `--root DIRECTORY` changes its
scope. File and text enumeration respect ripgrep's hidden/ignored-file rules.
Text searches use literal smart-case matching, with up to 2,000 results per
query, and skip binary files. PDF and Office document contents are not indexed.
Previews use bat; text results open at the matching line in a pager. File
results open with the existing default application. Manuals require man-db
and an indexed manual database (`mandb`). No background indexer is introduced.

### Time alternatives

The Adjacent branch opens GNOME Clocks for world clocks, alarms, timers and
a stopwatch. The Focused `hollow-time` chooser uses clock-tui's `tclock`
command for a timer, stopwatch, or date countdown. Timer durations and
countdown targets are explicitly entered; empty input cancels. The helper
finds tclock on PATH or in the user's Cargo bin directory. No timer-completion
shell commands are configured. The Alternate branch opens tty-clock in Kitty
with seconds, bold colors, a blinking colon and rebound animation. These tools
are optional, not bundled by this keyboard refinement.

### Local video library

`hollow-video library` opens or focuses Kodi, preferring native Wayland when
available. A fresh Kodi profile is seeded with Movies and TV Shows under the
user's XDG Videos directory, plus Downloads as an unsorted browsing source.
The initial Kodi window is windowed. After the first initialization/close,
the helper configures the fresh managed catalog for movies and TV shows,
with bundled TMDB metadata providers and library scanning on startup. Catalog
and settings backups are made before this configuration. Existing Kodi
source/settings profiles are retained. The helper never moves media files or
changes torrent-client paths. For cataloged downloads, select the Movies or
TV Shows folder as the download destination; unsorted Downloads stay browsable.

`folders` uses the default file manager for the collection directory. `play`
uses Zenity's native file dialog and passes the selected path literally to
mpv. Cancellation does not start playback. FreeTube remains the Alternate
route. Shared media transport keys are a separate refinement; the personal
cmus transport bindings are preserved by this change.

## W / Windows compatibility setup

Install Flatpak and the native Protontricks GUI tools on Arch:

```sh
sudo pacman -S --needed flatpak protontricks yad zenity
flatpak remote-add --user --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo
flatpak install --user flathub com.usebottles.bottles
```

Open Mod+W, finish Bottles' first-run component setup, and create an Application
or Gaming bottle for your software. Shift+W selects an executable and uses
Bottles' own bottle picker. Flatpak receives just the selected file through its
document portal; multi-file installers may need you to grant their specific
folder access in Bottles' permissions. Ctrl+W lists existing non-Steam bottles
and opens Wine configuration for the selected bottle. Use the primary hub for
dependencies, runner preferences, and installed programs. Alt+W opens Protontricks
and requires an existing Steam/Proton prefix for the game you choose. Launching
these pickers does not install Windows software or change any existing prefix.

Sources: [Bottles installation](https://docs.usebottles.com/getting-started/installation),
[Bottles CLI](https://docs.usebottles.com/advanced/cli), and
[Protontricks](https://github.com/Matoking/protontricks).

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

Justfiles and Makefiles are not currently indexed; invoke their commands through
Mod+X. No new task runner is installed. Fish, Cargo, and npm retain their existing
configurations. The command route intentionally accepts shell syntax; script and
project pickers pass selected paths and task names literally as argument vectors.

Sources: [Cargo build](https://doc.rust-lang.org/cargo/commands/cargo-build.html),
[Cargo run](https://doc.rust-lang.org/cargo/commands/cargo-run.html), and
[fzf](https://github.com/junegunn/fzf).

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

On Arch: `sudo pacman -S --needed syncthing grsync file-roller 7zip zip`.
Other required runtime tools are Python, Zenity, Kitty, systemd user services,
xdg-utils, libarchive (bsdtar), and playerctl. Syncthing's dashboard and all
configured device/folder relationships remain user-managed after setup.

Sources: [Syncthing](https://docs.syncthing.net/users/autostart.html),
[Grsync](https://www.opbyte.it/grsync/),
[File Roller CLI](https://github.com/GNOME/file-roller/blob/master/README_COMMANDLINE),
and [Playerctl](https://github.com/altdesktop/playerctl).
