# Hueman Screen Map v0.1.0

Date: 2026-07-13

## Rule

This document freezes the first practical 1:1 screen-to-world projection for Hueman above Hollow Grove.

Read that now as one shared coordinate field:

- `Hollow Grove` is the operating face of the map
- `Hueman` is the rendered face of the map
- the field below is one map, not two independent spaces

## Intent

- one monitor-sized field maps directly onto one visible Hueman field
- the focused window center becomes the primary player/world probe
- Godot renders the field and avatar state
- Hollow Grove and its bridge remain the causal layer that may affect files, windows, and runtime state
- the same normalized point may be read as a Hollow Grove point and a Hueman point at the same time

## Shared Contract

The broader constitutional reading for this map now lives in:

- `HOLLOW_GROVE_HUEMAN_COORDINATE_CONTRACT_v0.1.0.md`
- `HOLLOW_GROVE_CONSTITUTIONAL_GEOGRAPHY_V2.md`
- `artifacts/hollow_grove_hueman_coordinate_contract.json`

This screen map remains the active civic resolver contract, but it no longer stands alone as a Hueman-only surface.

## Reference Output

The current practical calibration target is the user's active desktop monitor in `niri`:

- monitor: `AORUS FO32U2P`
- size: `32-inch`
- resolution: `3840x2160`
- aspect ratio: `16:9`

This means the Hueman shell should be composed for a wide 4K field rather than a generic square or 720p test canvas.

## Orientation

When the player looks at the field:

- `Stonebend` sits at the top-center
- `Flynt` sits at the left
- `Glaushouse` sits at the bottom
- `Sandmanor` sits at the right

This keeps the visible world aligned to the user's stated screen reading.

## Named Nodes

Use these normalized screen anchors on a `[0.0, 1.0] x [0.0, 1.0]` canvas, calibrated for a `3840x2160` `16:9` display:

- `Ranina`: `(0.50, 0.50)`
- `Stonebend`: `(0.50, 0.14)`
- `Flynt`: `(0.265, 0.50)`
- `Glaushouse`: `(0.50, 0.86)`
- `Sandmanor`: `(0.735, 0.50)`
- `Aura Ridge Junction`: `(0.50, 0.50)`

`Ranina` is the canonical center underneath the visible `Aura Ridge Junction` and `Human Core` overlays.

These coordinates intentionally compress the horizontal spread compared with the first draft so the field reads correctly on a wide `16:9` monitor instead of stretching into a flattened oval.

## Straight Routes

- `Aura Ridge` — Witness: `Glaushouse <-> Stonebend`
- `Aura Ridge East`: local screen segment from `Aura Ridge Junction -> Sandmanor`, not an additional major constitutional route
- `Aura Way` — Design: `Stonebend <-> Sandmanor`
- `Glausbahn` — Refine: `Sandmanor <-> Glaushouse`
- `Boardwalk` — Return: `Glaushouse -> Flynt`
- `Basin Motor Speedway` — Produce: `Flynt <-> Stonebend`

## Curved Routes

- `Mt. Aura` — Aspire: `Stonebend <-> Sandmanor`
- `Current Seanad` — Deliberate: `Sandmanor <-> Glaushouse`
- `Riptide` — Retrieve: `Flynt -> Glaushouse`
- `Stairway to Heaven` — Ascend: `Flynt -> Stonebend`

## Constitutional Sea Route

- `Current Sea` — Certify: `Glaushouse -> Stonebend`

Current Sea is an ordeal and certification crossing, not another road. It is constitutionally distinct from Current Seanad. The frozen Current Synthesis route token for Current Seanad must not be reused to represent Current Sea.

## Dominant Route Verbs

Route shape supports rendering and traversal. The constitutional verb answers why civilization maintains the route:

`Boardwalk / Return`, `Riptide / Retrieve`, `Current Sea / Certify`, `Aura Ridge / Witness`, `Glausbahn / Refine`, `Current Seanad / Deliberate`, `Aura Way / Design`, `Mt. Aura / Aspire`, `Basin Motor Speedway / Produce`, and `Stairway to Heaven / Ascend`.

## Surfaces

- `Aura Field`: the upper-right interior triangle between `Stonebend`, `Sandmanor`, and `Aura Ridge Junction`
- `Aura Beach`: the lower-right interior triangle between `Sandmanor`, `Glaushouse`, and `Aura Ridge Junction`
- `Aura Basin`: the left-side interior triangle between `Stonebend`, `Flynt`, and `Glaushouse`

## Resolver Grammar

Evaluate the focused window center against the normalized screen map in this order:

1. kingdom node
2. route band
3. surface polygon
4. motion-grid cell
5. nearest route fallback

Default geometry thresholds:

- node radius: `0.09`
- route band width: `0.045`

This keeps kingdoms and named roads crisp while still allowing large interior surface regions to exist.

## Practical Contract

The first practical implementation should stay narrow:

- `niri` bridge reads output and focused-window geometry
- bridge writes normalized screen state to an artifact file
- Godot reads the static map spec and the live screen-state artifact
- Godot resolves sprite placement and world highlighting locally
- Godot writes back only structured intents
- Hollow Grove decides whether any intent is allowed to mutate files or runtime state

## Godot Boundary

The most practical and effective Godot integration is not to let Godot touch arbitrary files directly.

Instead:

- Godot owns presentation, camera, sprite movement, hover state, and visible zone emphasis
- Hollow Grove owns real consequences
- the bridge owns compositor geometry

That yields a simple loop:

1. `niri` reports focused window geometry
2. bridge normalizes the geometry to the active output
3. bridge writes `artifacts/screen_map_state.json`
4. Godot reads `artifacts/hueman_screen_map.json` and `artifacts/screen_map_state.json`
5. Godot renders the avatar inside the resolved node, route, or surface
6. Godot emits a narrow intent such as `inspect`, `move`, `name`, `prove`, `clear`, or `recognize`
7. Hollow Grove consumes that intent and decides whether a real file mutation or runtime action occurs

## Boundary Reminder

This map belongs to Hueman's world-facing layer.
It does not move Hollow Grove into Godot, and it does not grant Godot direct authority over filesystem mutation.
The coordinates themselves are shared across both faces even when mutation authority is not.
