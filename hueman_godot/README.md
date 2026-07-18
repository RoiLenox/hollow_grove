# Hueman Godot Shell

This is the thinnest practical Godot 4 shell for the shared Hollow Grove / Hueman screen map.

It reads:

- `artifacts/hueman_screen_map.json`
- `artifacts/hollow_grove_hueman_coordinate_contract.json`
- `artifacts/screen_map_state.json`
- `artifacts/hollow_grove_application_registry.json`
- `artifacts/hueman_pair_state.json`

It writes:

- `artifacts/screen_map_intent.json`

Boundary:

- Godot renders the shared coordinate field, routes, surfaces, overlay terms, and the live player probe.
- Godot may emit narrow intents such as `inspect` and `move`.
- Hollow Grove remains the only layer allowed to decide real runtime or filesystem side effects.
- Hueman is the rendered face of the same map rather than a separate spatial product.
- Managed clinical applications use their registered world anchor instead of geometry-derived meaning.
- A capture-disallowed managed application is represented by a semantic card; its window pixels are never loaded into Godot.

Run from the repo root with:

```bash
./run-hueman-godot.sh
```

For live art iteration with Aseprite:

```bash
./run-hueman-live.sh
```

Put `.aseprite` files in `hueman_godot/assets/source/`. The watcher exports them into
`hueman_godot/assets/export/` as sprite sheets plus JSON metadata every time you save.
Godot will reimport the changed PNGs automatically while the shell is open.

Useful commands:

```bash
./export-aseprite.sh
./watch-aseprite.sh
./run-hueman-live.sh
./asset-pipeline-status.sh
```

Recommended loop:

- Keep Aseprite open on the source file in `assets/source/`.
- Keep Hueman/Godot open with `./run-hueman-live.sh`.
- Save in Aseprite and check the result immediately in Godot.
- Use exported PNGs from `assets/export/` inside your Godot scenes or UI nodes.

Asset organization:

- Read `hueman_godot/assets/README.md` for the fixed folder layout.
- Put unfinished experiments in `assets/source/prototypes/`.
- Put stable gameplay art into the matching permanent category.
- Run `./asset-pipeline-status.sh` to check for missing or orphaned exports.

Controls:

- `Enter`: write an `inspect` intent for the current resolved zone
- `Space`: write a `move` intent for the current resolved zone
- `Super+Alt+Enter`: open the Hueman/Godot shell
- `Super+Alt+Control+Enter`: attach the focused window through the existing pair control
- `Super+Alt+Backspace`: clear the paired window
- `Super+Alt+Shift+Space`: step the pair spread by the existing 25% interval

For `chroma_cord`, launch with `cargo run -- app launch chroma_cord`, focus that
window, then use the attachment control. Hollow Grove accepts the attachment only
when the focused Niri application ID is exactly `hollow-grove.chroma-cord`.
