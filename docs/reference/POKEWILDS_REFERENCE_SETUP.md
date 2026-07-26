# PokéWilds External Reference Setup

## Acquisition Record

- Setup started: `2026-07-22T10:09:51-05:00` (US/Central)
- Release downloaded: `2026-07-22 10:10:09-05:00`
- Official latest-release URL:
  `https://github.com/SheerSt/pokewilds/releases/latest/download/pokewilds-linux64.zip`
- Detected release: `v0.8.11` from the archive root
  `pokewilds-v0.8.11-linux-64/`
- Archive size: `162313416` bytes
- Archive SHA-256:
  `b300b4c58c99a4b36c27eb5c2864883fae6ad67c176dedd1eece7cb58c169dc2`
- Archive validation: nonempty ZIP, readable by `bsdtar`, `30` archive
  entries

The archive, its checksum, and its complete archive listing are external:

- `/home/warren/Reference/pokewilds/downloads/pokewilds-linux64.zip`
- `/home/warren/Reference/pokewilds/downloads/pokewilds-linux64.zip.sha256`
- `/home/warren/Reference/pokewilds/inventories/linux-release-archive.txt`

## Runtime Layout

- Immutable reference runtime:
  `/home/warren/Reference/pokewilds/runtime-original/`
- Disposable Hueman conversion runtime:
  `/home/warren/Reference/pokewilds/runtime-hueman/`
- Detected original game root:
  `/home/warren/Reference/pokewilds/runtime-original/pokewilds-v0.8.11-linux-64`
- Detected README:
  `/home/warren/Reference/pokewilds/runtime-original/pokewilds-v0.8.11-linux-64/README.txt`
- Detected launcher: `PokeWilds-x64`, a 64-bit x86-64 Linux executable
- Extracted files per initial runtime: `23`

`runtime-original` was made recursively read-only after extraction. It had zero
writable files, and all 23 recorded checksums still passed after graphical
validation. `runtime-hueman` remains writable and disposable. Its files and
checksums still matched the original immediately after the launch test; no save
was created.

## Launch Validation

The included README directs Linux users to execute `PokeWilds-x64` from the
game directory. The validated disposable-runtime command was:

```bash
cd /home/warren/Reference/pokewilds/runtime-hueman/pokewilds-v0.8.11-linux-64
./PokeWilds-x64
```

Launch was validated on `2026-07-22T10:17:41-05:00`:

- the process started and reported Java 17 plus the active AMD/OpenGL renderer;
- Niri reported a `PokeWilds` window with application ID `PokeWilds`;
- the generated world rendered visibly;
- runtime output showed route and facing-tile keyboard processing;
- a compositor window-close request ended the process normally with exit code
  `0`;
- no long-term save was generated.

The title/menu phase was not captured separately: focus/input advanced the
fresh runtime into map generation before the visual capture. Window creation,
rendering, keyboard processing, and clean closure were all directly validated.
The host `ldd` probe exited with code `159` against the bundled executable, but
this did not prevent the executable from launching successfully.

Detailed local launch evidence is at
`/home/warren/Reference/pokewilds/notes/launch-validation.txt`.

## Asset Repository

- Repository URL: `https://github.com/SheerSt/pokewilds.git`
- Local checkout: `/home/warren/Reference/pokewilds/asset-repo/`
- Clone mode: complete current `main` worktree, shallow history (`--depth 1`)
- Commit: `2e1ad7126e57bd293b5610def7d9dd04e0c555f1`
- Worktree after clone: clean
- Total files excluding `.git`: `66989`
- Unresolved Git LFS pointers: `0`

All required asset categories exist and are nonempty:

| Category | Files |
|---|---:|
| `attacks` | 51352 |
| `battle` | 61 |
| `i18n` | 40 |
| `menu` | 45 |
| `music` | 67 |
| `player` | 144 |
| `pokemon` | 13739 |
| `sounds` | 96 |
| `tiles` | 1327 |

The external inventories are:

- `/home/warren/Reference/pokewilds/inventories/runtime-original-files.txt`
- `/home/warren/Reference/pokewilds/inventories/runtime-hueman-files.txt`
- `/home/warren/Reference/pokewilds/inventories/asset-repo-files.txt`
- `/home/warren/Reference/pokewilds/inventories/asset-repo-file-types.txt`
- `/home/warren/Reference/pokewilds/inventories/asset-extension-counts.txt`
- `/home/warren/Reference/pokewilds/inventories/runtime-original.sha256`
- `/home/warren/Reference/pokewilds/inventories/runtime-hueman-after-launch.sha256`
- `/home/warren/Reference/pokewilds/inventories/asset-repo.sha256`
- `/home/warren/Reference/pokewilds/inventories/unresolved-lfs-pointers.txt`

## Hollow Grove Boundary

PokéWilds remains external behavioral and temporary visual scaffolding. Hollow
Grove does not load it, link it, execute it, or inspect it during a build or
test. Removing `/home/warren/Reference/pokewilds/` has no effect on the
Rust-authoritative/Godot-presentational architecture.

Files introduced or changed by this setup are limited to:

- `.gitignore` — defensive local-reference, archive, and save patterns;
- `POKEWILDS_PARITY_LEDGER.md` — mechanical observation and translation ledger;
- `docs/reference/POKEWILDS_REFERENCE_SETUP.md` — this acquisition record.

No current gameplay or interaction implementation file was changed.

## Limitations and Rights Boundary

The public repository contains asset/mod data rather than the Java/libGDX
engine source and contains no root license file at the recorded commit. Its
README also describes console-derived graphics. Consequently, neither its
files nor the release executable are vendored, redistributed, or treated as a
Hollow Grove implementation dependency. Only observable behavior is eligible
for clean, original Rust/Godot reimplementation.
