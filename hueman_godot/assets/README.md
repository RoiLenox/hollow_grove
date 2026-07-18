# Hueman Asset Layout

This asset tree is intentionally strict.

Source of truth:

- `source/` contains hand-edited `.aseprite` or `.ase` files only.
- `export/` contains generated `.png` sprite sheets and `.json` metadata only.
- `reference/` contains internet reference images, moodboards, palettes, and screenshots that are not shipped.
- `third_party/` contains downloaded asset packs that you are legally allowed to use, along with their license files.

Folder layout:

- `characters/player/` for the main controllable actor and direct variants
- `characters/npcs/` for world characters, enemies, and non-player entities
- `tilesets/world/` for exterior map tiles, terrain, roads, coast, and routes
- `tilesets/interiors/` for buildings, rooms, and interior-specific tiles
- `props/` for objects that are placed in scenes but are not tiles
- `effects/` for particles, bursts, weather, and animated feedback
- `ui/` for cursors, frames, icons, prompts, and HUD pieces
- `portraits/` for dialogue portraits and close-up character art
- `overlays/` for Hueman-specific layer visuals, masks, and compositing art
- `prototypes/` for temporary test assets that are allowed to be ugly

Reference layout:

- `reference/characters/` for sprite and animation inspiration
- `reference/inbox/` for unsorted internet downloads before you decide where they belong
- `reference/tilesets/` for terrain, buildings, and map ideas
- `reference/ui/` for menus, HUDs, icons, and panel ideas
- `reference/world/` for environments, landmarks, and scene composition
- `reference/moodboards/` for broad visual collections
- `reference/palettes/` for color references and swatches

Naming rules:

- use lowercase snake_case
- keep names semantic, not visual
- prefer `stonebend_sign.aseprite` over `brown_sign_final2.aseprite`
- keep animation states in tags inside one file when the sprite belongs together
- split into separate files when the asset has a different gameplay role

Practical workflow:

1. Download internet references into `reference/inbox/`.
2. Run `./sort-asset-inbox.sh list` to see what is waiting.
3. Move each file into the right bucket with `./sort-asset-inbox.sh move <file> <bucket>`.
4. If you find a real asset pack you want to use, store it in `third_party/<pack_name>/` and keep the license text beside it.
5. Create your actual working `.aseprite` file in the correct `source/` category.
6. Redraw, adapt, or interpret from the reference rather than mixing reference files into `source/`.
7. Run `./run-hueman-live.sh`.
8. Save in Aseprite.
9. Let the watcher export into the mirrored path under `export/`.
10. Use the exported `.png` and `.json` in Godot.

Internet rule:

- `reference/` is inspiration only.
- `third_party/` is for licensed downloads you may actually use.
- `source/` is your editable game art.
- `export/` is generated runtime output.

What not to do:

- do not drop random downloaded PNGs into `source/`
- do not drop reference images into `export/`
- do not use ripped commercial game art unless you have the rights
- do not lose the license file for third-party packs

Good search terms:

- `cc0 pixel art tileset`
- `public domain pixel sprite`
- `pixel art ui kit cc0`
- `16x16 top down tileset cc0`
- `pixel palette reference`

Useful commands:

- `./sort-asset-inbox.sh list`
- `./sort-asset-inbox.sh move <file> <bucket>`
- `./export-aseprite.sh`
- `./watch-aseprite.sh`
- `./run-hueman-live.sh`
- `./asset-pipeline-status.sh`

Rule of thumb:

- If you are unsure where something goes, put it in `prototypes/` first.
- Move it into a permanent category only when its role is clear.
