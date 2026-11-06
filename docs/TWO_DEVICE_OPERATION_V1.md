# Installed world and two-device operation

The installed Core provides `hollow-grove-device`. This manages the default
XDG data world's catalog and gameplay saves independently of source, packages,
desktop configuration and Hueman performance preferences.

`hollow-grove-device status` is read-only. It reports catalog presence, the
clinical office record, installed Core/client/tool hashes, a durable-state
fingerprint, any handoff pause and interrupted-install recovery status. Presence
of an office record is a readiness hint; the Rust runtime still decides whether
each action has valid authority.

For an empty installed world, explicitly import an existing world:

```sh
hollow-grove-device import-world /absolute/path/to/existing/world
```

Import validates the source catalog using the installed Rust service with
temporary saves, preserves current gameplay saves, retains a backup, and copies
the existing `artifacts/institutional_state.txt` bytes. It refuses to overwrite
a nonempty world. It creates no new officeholder or source-law decision. The
normal launchers use this installed world; `HOLLOW_GROVE_WORLD_ROOT` remains an
explicit development override in both launchers. Handoff covers the default
installed world, so external development worlds must be managed separately.

Configure each machine with the other's existing SSH alias:

```sh
hollow-grove-device configure-peer other-machine
hollow-grove-device handoff
```

Run handoff on the machine whose current progress you want to take with you.
Close Hollow Grove on both machines first. SSH requires key authentication and
an already verified host key. The receiving machine must have byte-identical
Core, Hueman and device-tool payloads. Source synchronization and package
installation remain separate operations; this command does not rebuild or
replace packages, modify hardware preferences or copy credentials.

Handoff pauses the sending machine's launchers before transmitting, then checks
and installs the state on the receiver. A destination with independent progress
is refused. A previously parked receiver must still match its parked state and
the sending peer. On success, play on the receiving machine; run the same command
there when returning. A failed/uncertain transfer leaves the sender parked and
retains its outgoing packet. Repeat the same handoff command to obtain an
idempotent receipt. Do not delete the pause marker to bypass ownership.

The integrated and standalone service launchers hold a shared device lock;
handoff and imports require an exclusive lock and also check Rust runtime leases.
Raw development binaries and external save roots are outside that launcher
coordination. Runtime lock/claim files are excluded from transfers; all remaining
files under `world/` and `gameplay-saves/` are checksummed. Symlinks, traversal,
checksum mismatch and oversized payloads are refused. V1 permits at most 10,000
files and 128 MiB of durable data.

Incoming files are staged and the previous directories are retained under
`$XDG_DATA_HOME/hollow-grove/device-backups/`. A persistent transaction journal
blocks launch after interruption. `hollow-grove-device recover` restores that
transaction's saved prior state and preserves the interrupted files. It does
not undo a completed handoff. Keep the prior binaries with their compatible
state snapshot when rolling back a package upgrade.

Hueman presentation preferences are local to each device:

```ini
# ~/.config/hueman/performance.cfg
[performance]
max_fps=60
background_fps=15
render_on_demand=true
```

The defaults above bound rendering and background polling without changing
runtime authority, gameplay reducers or protocol revisions. A foreground value
of 120 is available for a faster display; 0 removes the application cap.
`HUEMAN_MAX_FPS` and `HUEMAN_BACKGROUND_FPS` override the file for a single run.
Settings are validated between 0 and 360 FPS; invalid values use the default.
The background limit never exceeds a positive foreground limit. Focus restores
the foreground rate. Rendering on demand avoids drawing unchanged frames;
transport polling continues. Actual frame pacing and power must be measured on
each GPU. See Godot's [Engine](https://docs.godotengine.org/en/stable/classes/class_engine.html)
and [OS](https://docs.godotengine.org/en/stable/classes/class_os.html) documentation.

For local Fish TTY login, the packaged skeleton contains
`hollow-niri-login.fish`. It starts `niri-session` only on tty1, outside SSH, with
a guard against the session launcher's login-shell reentry. Existing machines
must remove the old `default.target.wants/niri.service` autostart after backing
it up; it starts the compositor before login-session environment import.
Preserve machine GPU settings. Apply this migration without restarting the
current compositor, then verify the next real logout/login, lock and resume.
The official [Niri systemd setup](https://niri-wm.github.io/niri/Example-systemd-Setup.html)
describes session-bound services.
