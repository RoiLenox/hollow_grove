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
| A | Assets | Aseprite, Blender, LMMS | The missing asset tool is named |
| B | Browse | Firefox, Chromium, Vivaldi, Brave | The missing browser is named |
| C | Chat | Himalaya, Outlook role, Element, Discord | No account or credential is included |
| D | Develop | Codex, OpenCode, Code-OSS, Zed | The missing development tool is named; no credentials are included |
| E | Edit | Helix, Lapce, Neovim, Micro | The missing editor is named |
| F | Files | Yazi, Thunar, Dolphin, PCManFM-Qt | The missing file manager is named |
| G | Games | Steam, RPCS3, Dolphin Emulator, RetroArch | The missing game frontend is named |
| H/J/K/L | Movement | Native Niri focus, move, reshape, and transport | No application fallback applies |
| I | Images | imv, Loupe, Gwenview, qView | The missing image viewer is named |
| M | Music | AudioTube, youtui, cmus, Tauon | The missing music tool is named |
| N | News | Newsboat, Liferea, RSS Guard, Fluent Reader | The missing feed reader is named |
| O | Office | LibreOffice Writer, Calc, Impress, Draw | The missing suite is named |
| P | Photos | darktable, RawTherapee, digiKam, Shotwell | The missing photography tool is named |
| Q | Query | Gemini, Ollama, AIChat; Alternate empty | The missing query environment is named |
| R | Record | Waveform, Ardour, Audacity, REAPER | The missing recorder or DAW is named |
| S | Settings | Network TUI, Blueman, Network Connections, PWVuControl | The missing settings surface is named |
| T | Tasks | Taskwarrior TUI/CLI, Planify | The missing task application is named |
| U | Utilities | btop, ncdu, nvtop, battop | The missing system utility is named |
| V | Video | FreeTube, VLC, mpv, Haruna | The missing video tool is named |
| W | Write | Obsidian, Apostrophe, Ghostwriter, MarkText | The missing writing tool is named |
| X | eXchange | No finance/markets application selected | All branches remain honestly empty |
| Y | sYnc | LocalSend, Croc, Magic Wormhole | The missing transfer tool is named |
| Z | Zip | File Roller, Ark, Xarchiver, PeaZip | The missing archive tool is named |

Special families remain outside the letter grid: Return is Terminal, Grave is
Navigate, Backspace is History, Escape is Exit, and Space shapes usable room.

## Compositor and device bindings

| Keys | Role | Availability behavior |
| --- | --- | --- |
| `Mod+Space`, `Mod+Shift+Space` | Glaüshouse usable room forward/backward | Native Niri preset-width actions; official keyboard Step 2 |
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
