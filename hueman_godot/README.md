# Hueman Godot Shell

## Playable retro overworld

The playable game now spans all ten constitutional routes and all three
singular interior surfaces—Aura Field, Aura Beach, and Aura Basin—including
the Current Sea deep-certification landing, Boardwalk return vestibule, three
solid mines, and two offshore Current rigs in a 160×144 integer-scaled
overworld. It uses an original multi-scale Hueman
design, the constitutional Hollow Grove palette, and a compact color-handheld
RPG presentation. The Rust gameplay runtime owns maps, collision, position,
facing, case state, constitutional Bonds, revision, events, and persistence;
Godot renders the returned immutable view—including outcome authority,
relationship term, uncertainty, and refusal protections—and animates accepted
steps.

Run it from the repository root:

```bash
./run-hollow-grove-game.sh
```

Controls:

- Arrow keys, WASD, or HJKL: walk one tile
- Enter, Space, Z, or X: interact with the faced guide or witness marker and
  advance dialogue
- Tab: select another connected destination while standing at a physical exit
- B: traverse the selected physical exit; away from the exit tile, traversal
  fails without moving the player
- T: advance one deterministic Dawn/Day/Dusk/Night living-world shift
- F: disclose the next Hueman faculty observation for the active case
- living-region 1–3: after inspecting the three evidence facilities, support
  one of two lawful duty-officer responses or exercise the authored refusal
  path
- Current Sea 1–3: support existing Name, provisional transformed-form Name, or
  high identity review
- Boardwalk 1–4: support Pimp Patronage, Goon Bond, Limited Cooperation, or
  Independent Return
- Deep Pressure final Boardwalk 1–4: support Shared Burden, Crew-and-Coast
  Restitution, Production Under Review, or Protected Refusal; this takes
  priority after the seven linked cases and required affected statements
- C: ask Stonebend, the Returning Goon, or the local duty officer to commit the
  decision after support; in Deep Pressure, ask the affected assembly to commit
- P: open the authoritative six-person party panel; Up/Down selects, Enter
  asks the selected ready member to lead, U uses that lead's map-bound field
  action, and P closes the panel
- R while facing a recruitment candidate after Deep Pressure: open the
  nonbinding request; 1 asks through Shared Work, 2 through Recovery First,
  and 3 through Independent Company
- F5 / F9: save / load `slot-a`

The launcher starts the loopback-only authoritative gameplay service and then
opens `scenes/retro_overworld.tscn`. The original Hueman screen shell remains
available through `./run-hueman-godot.sh`.

Visual development references live in the non-shipping reference tree:

- `assets/reference/characters/hueman_multiscale_character_concept.png`
- `assets/reference/moodboards/hollow_grove_handheld_overworld_mood.png`

They establish character scale and environment density only. Runtime art is
original, deterministic, pixel-aligned, and resolved from the canonical palette.

This is the thinnest practical Godot 4 shell for the shared Hollow Grove / Hueman screen map.

It reads:

- `src/constitutional/hollow_grove_visual_color_palette.json`
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
- Godot resolves all visual colors by constitutional semantic identity from the
  canonical palette; it owns no local RGB palette or pure-black default.

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

## Party RPG protocol client

`scripts/runtime_client.gd` is the transport-only client for gameplay protocol
V1. The authoritative Rust service can be started from the repository root:

```bash
cargo run --bin hollow_grove_game_service -- \
  --listen 127.0.0.1:47819 \
  --session session.hollow-grove.local \
  --world-root . \
  --save-root artifacts/gameplay-saves
```

The client sends versioned JSONL intents and presents returned event/view data.
Grid movement, collision, facing, faced-tile interaction, the route network,
all three surfaces and their facilities, the Boardwalk and Current Sea cases,
five extraction maps, physical exits, Aura weather, worker schedules, seven
living-region/extraction cases, material custody, cross-region consequences,
the character-driven Deep Pressure campaign, the Hueman-plus-five party,
capable-subject recruitment, companion field actions, and disk save/load are
implemented by Rust reducers. Scheduled people are visible, move by shift, block movement,
offer explicitly classified speech, and retain persistent relationship memory.
Deep Pressure links all seven cases into four Boardwalk endings with separate
four-House acts, optional finite recovery Bonds, and visible regional
aftermath. Recruited people leave their scheduled-map projection; acceptance,
refusal, exhaustion recovery, party lead, and field evidence persist and
replay. The
Current Sea case creates typed Stonebend evidence, Name, decision, and Seal
records without granting Title. Pimp Patronage, Goon Bond,
and Limited Cooperation each form, validate, and activate a distinct finite
Bond in the common constitutional runtime. Independent Return records
recognized refusal without a Bond. Godot only asks and presents. World
authority is loaded from schema-V2 institutional state and embedded in
schema-V2 gameplay saves for exact replay. Encounter, combat, general
progression, equipment, and Synthesis decisions remain unavailable until their
Rust reducers exist. Shared request/response fixtures live under
`protocol/fixtures/`.

The extraction and living-state law is documented in
`HOLLOW_GROVE_LIVING_SURFACES_AND_EXTRACTION_V1.md`.
The cross-region campaign is documented in
`HOLLOW_GROVE_DEEP_PRESSURE_CAMPAIGN_V1.md`.
The party and recruitment slice is documented in
`HOLLOW_GROVE_PARTY_AND_RECRUITMENT_V1.md`.
