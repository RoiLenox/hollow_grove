# Optional Applications and Binding Fallbacks

## Fallback contract

`/usr/bin/hollow-launch` verifies every declared command before execution. If
anything is missing, it emits a visible notification and a matching stderr
message naming the role and missing command, then exits with status 127. Niri
continues running.

Private wrappers for Walker, Ardour, AudioTube, LMMS, Hueman, weather,
screenshots, brightness, audio, and power profiles apply the same principle.
Startup-only helpers either use a neutral fallback or exit successfully when
an optional service is absent.

## Left-hand application grid

Office (`O`) and Music (`M`) are the first admitted domain plants. Their
modifier bindings are branches named Primary, Adjacent, Focused, and
Alternate. Applications are replaceable fruit beneath those stable names. All
other letter groups in this table remain provisional launch families until the
keyboard constitution explicitly admits them; a populated family is not
automatically an official plant.

| Keys | Role | Commands | When absent |
| --- | --- | --- | --- |
| `Mod+Return`, `Mod+Shift+Return`, `Mod+Ctrl+Return` | Terminal | Kitty, Bash, Zellij | Kitty/Bash are base; missing Zellij is reported |
| `Mod+Alt+Return` family | Hollow Grove / Hueman | `hollow-hueman` | Requests installation of the separate Hueman package |
| `Mod+Grave` | Flynt launcher | Walker | Walker is the exclusive action on the Grave/Tilde House anchor |
| `Mod+Shift+Q`, `Mod+Ctrl+Q`, `Mod+Alt+Q` | Browse and query variants | Firefox, Anyrun, Chromium | Missing role is named; plain `Mod+Q` remains open |
| `Mod+W` family | Write and read | Helix, Obsidian, Glow, Lapce | Missing editor/reader is named |
| `Mod+E` family | Emulators | mGBA, melonDS, Dolphin, RPCS3 | Missing emulator is named |
| `Mod+R` family | Audio production | Waveform, Ardour, Audacity, REAPER | Missing DAW is named; no audio-device tuning is injected |
| `Mod+T` family | Tasks | Taskwarrior TUI/CLI, Planify | Missing task application is named |
| `Mod+Y` family | Streaming | AudioTube, YouTUI, FreeTube, YewTube | Missing application is named |
| `Mod+U` family | Storage | Yazi, NCurses disk usage, UDisks, Dust | Missing utility is named; Dust receives the current user's home at runtime |
| `Mod+I` family | Time and weather | Peaclock, calendar, current weather, radar | Missing software is named; weather explains when location is unset |
| `Mod+O` family | Office plant | LibreOffice modules | Missing suite is named |
| `Mod+P` family | Power and battery | Battop, UPower, power profiles | Unsupported or missing power service is reported |
| `Mod+A` family | Mail and alerts | Himalaya, Outlook role, Mako history | No account is included; Outlook reports that no integration is installed |
| `Mod+S` family | System status | Bottom, Btop, sensors, NVTOP | Missing monitor is named |
| `Mod+D` family | Downloads/packages | rTorrent, qBittorrent, Pacseek, Paru | Missing client is named |
| `Mod+G` family | Assistants | Codex, Gemini, Ollama, OpenCode | Missing CLI is named; no credentials are included |
| `Mod+Z` family | Studio and readers | LMMS, imv, Poppler PDF info, Okular | Missing application is named |
| `Mod+X` family | Exchange | LocalSend, Croc, Magic Wormhole | Missing transfer tool is named |
| `Mod+V` family | Audio graph | WirePlumber status, Wiremix, PWVuControl, QPWGraph | Missing mixer is named |
| `Mod+B` family | Bluetooth | BlueTUI, Blueman, `bluetoothctl` | Missing frontend is named |
| `Mod+N` family | Network | NetworkManager TUI, editor, and CLI | Base package supplies these commands |
| `Mod+M` family | Music plant | Cmus and Cava | Missing player/visualizer is named |

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
| `Print` and `Mod+C` family | Niri screenshots and casting | Native Niri actions remain available independently of optional applications |

## Startup behavior

- A missing Mako or Elephant user service is ignored without failing the
  compositor.
- A missing XWayland satellite or PolicyKit agent is reported, but Niri starts.
- A missing wallpaper image falls back to the neutral background color.
- Missing clipboard tools simply disable clipboard history.
- No optional command is evaluated while Niri parses its configuration.

Application absence is therefore a feature-level limitation, not a compositor
startup dependency.
