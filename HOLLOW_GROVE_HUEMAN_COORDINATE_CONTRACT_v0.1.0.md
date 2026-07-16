# Hollow Grove Hueman Coordinate Contract v0.1.0

Date: 2026-07-13

## Rule

Hollow Grove and Hueman are one coordinate system with two faces.

- `Hollow Grove` is the operating, naming, and mutation-authoritative face.
- `Hueman` is the world-facing, rendered, and spatial face.
- every declared Hollow Grove term used by the visible field must resolve to a stable Hueman coordinate, band, node, surface, or overlay cell

## One-System Reading

This repository should no longer read the desktop layer and the Hueman layer as separate conceptual products for the purposes of screen/world mapping.

Use this reading instead:

- desktop windows are Hollow Grove actors
- Hueman is the visible coordinate projection underneath those actors
- paired-window position is a Hollow Grove position and a Hueman position at the same time
- civic anchors, route names, surface names, and motion-map cells belong to one shared spatial grammar

## Shared Coordinate Space

The canonical field is the normalized screen rectangle:

- origin: `top_left`
- x range: `[0.0, 1.0]`
- y range: `[0.0, 1.0]`

This coordinate space is the shared constitutional surface for:

- kingdom anchors
- route bands
- surface polygons
- overlay cells from the Hollow Grove motion map

## Shared Visible Anchors

### Center Anchor

- `Ranina`: `(0.50, 0.50)`

`Ranina` is the canonical Hollow Grove center. It is not a fifth House or a numbered rotational position.

### Kingdom Anchors

- `Stonebend`: `(0.50, 0.14)`
- `Flynt`: `(0.265, 0.50)`
- `Glaushouse`: `(0.50, 0.86)`
- `Sandmanor`: `(0.735, 0.50)`
- `Aura Ridge Junction`: `(0.50, 0.50)`

### Motion-Map Overlay Cells

These cells belong to the same normalized field and may be drawn above or below the civic map without changing the civic map coordinates.

- `Hollow Back`: `(0.20, 0.20)`
- `Hollow Grove`: `(0.50, 0.20)`
- `Hollow Bend`: `(0.80, 0.20)`
- `The Grove`: `(0.20, 0.50)`
- `Human Core`: `(0.50, 0.50)`
- `The Hollows`: `(0.80, 0.50)`
- `Grove Orchard`: `(0.20, 0.80)`
- `Grove Hollow`: `(0.50, 0.80)`
- `Grove Falls`: `(0.80, 0.80)`

`Aura Ridge Junction` and `Human Core` intentionally share the `Ranina` center anchor. That is the explicit declaration that the central Hollow Grove reading and the central Hueman reading occupy the same coordinate without replacing the center itself.

## Constitutional Correspondence

The following visible anchors remain the first practical constitutional correspondence set:

- `Stonebend` carries `Name It` and `Craft`
- `Sandmanor` carries `Prove It` and `Configuration`
- `Glaushouse` carries `Clear It`, `Repair`, and `Synthesis`
- `Flynt` carries `Recognize It` and `Engineering`

These are not separate symbolic tags. They are Hollow Grove operating terms with Hueman coordinates.

## Contract Files

This rule is machine-readably anchored in:

- `artifacts/hueman_screen_map.json`
- `artifacts/hollow_grove_hueman_coordinate_contract.json`

The screen map may remain the active resolver contract for now, but the coordinate contract is the canonical declaration that both layers share one map.

## Practical Implication

When the bridge, runtime, or Godot shell resolves a live position:

1. a window position is normalized once
2. that normalized point is a Hollow Grove point and a Hueman point at the same time
3. any resolved node, route, surface, or overlay term may be read by either face

## Boundary Reminder

This contract unifies coordinates, not authority.

- Hollow Grove still decides real runtime and filesystem consequences
- Hueman still renders and projects the world-facing field
- the coordinate map is shared even when mutation authority is not
