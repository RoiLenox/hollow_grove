# The Boardwalk — governing doctrine v0.1 implementation

Implemented 2026-10-28 in the existing Rust-authoritative Godot game.

"Nobody owns The Boardwalk, but everybody wants to control it."

The Boardwalk is neutral public infrastructure. The Pimp commands above through
the Trooners; The Gimp commands below through the Gooners. Neither commands the
whole or acquires legitimate sovereignty. Existing Hollow Grove institutions
retain their authority. The Boardwalk belongs to no criminal empire.

## Geography and play

The existing `boardwalk.return-vestibule` remains the canonical entry map for
the single straight southwest Flynt–Glaüshouse route. Two additional local
maps, `boardwalk.upper-span` and `boardwalk.lower-span`, represent stacked decks
of the same suspension bridge. They are not additional transportation routes.
The separate curved Riptide, House anchors, Central Junction, and all ten
constitutional route segments are unchanged.

Both spans connect to the public vestibule at tile (9,15), using its existing
exit selection (Tab) and traversal (B) controls. The far-end stair at (9,2)
connects the spans directly, retaining the arrival location on the other level.
The vestibule preserves its existing external exits, including the Aura Basin
map that provides the existing Flynt Basin connection. No direct span-to-other-
route shortcuts were added. Ordinary passage requires no case decision, Bond,
faction membership, fee, or reputation threshold.

The upper deck has striped commercial awnings, a stage, tall paired bridge
supports, suspension cables, and an open promenade. The lower deck shares the
supports, with overhead braces, pipes, machinery and storage. Both have marked
stairs, public passage signs, faction-labelled residents and explicit level
titles. Rendering uses the existing constitutional palette and deterministic
160×144 pixel drawing; no asset packs, shaders or dependencies were added.
Screen-reader context includes neutrality, influence, residents and exit tiles.

## Factions and ambient influence

The registry separately identifies Trooners and Gooners, their leaders' existing
participant IDs, home territories and reciprocal rivalry. Neither has a
sovereign House affiliation or a Syndicate organization assignment. Individual
presence identifies faction membership; it does not grant House authority.

Each local territory projects an ID, level, shared route ID, formal neutrality,
legal status, absence of a sovereign faction, public access, contested status,
two independent influence scores, dominant/rival faction, and local presence.
The scores are relative weights, not percentages or ownership shares.

The existing event-sourced Dawn/Day/Dusk/Night clock drives bounded changes to
influence and host placement. Each level has its leader, a resident host and a
rival courier. Hosts shift between reserved conversation bays; people cannot
appear on the player or block the central passage or stairs. Dialogue expresses
commercial or operational rivalry while affirming public access. No random
conquest, faction combat, or additional mandatory quest has been introduced.

`BoardwalkInfluence` and `BoardwalkTerritoryView::with_influence` provide a narrow
future integration point. Either faction can dominate or the scores can tie,
without changing legal status, passage rights, or physical occupation. Normal
ambient shifts retain each level's established home faction; the rivalry cycles
instead of ending in permanent control of the bridge.

The existing Returning Goon case and its public vestibule meetings remain
intact. Its finite relationships are not bridge sovereignty or the player's
permanent allegiance. Ambient span conversations do not collect case evidence
or commit case decisions. Existing campaign residents stay in their authored
maps, avoiding duplicate projections onto the new spans.

## Kernel and persistence inspection

Inspected the neutral recursion kernel, semantic kernel path/inheritance
contracts, `src/world/spatial_inheritance.rs`, constitutional geography,
route network, House identity contract, gameplay reducers, archives and the
application's kernel invariant/protocol boundary before integration.

- `hollow-grove-recursion-kernel` owns bounded cycle/Pleb–Meta routing and has no
  faction, character, House, or persistence responsibility. It is unchanged.
- `hollow-grove-semantic-kernel` owns neutral composition, geometry, path
  inheritance and provenance. The Boardwalk remains a straight path without
  curve controls; Riptide retains its distinct curved controls. No new kernel
  region, sovereign, path, or House was introduced.
- The application-level kernel audit now checks route identities, separate
  faction affiliations, and neutral, contested public passage. Its report is
  included in the existing read-only gameplay `kernel_audit` projection. It
  neither decides jurisdiction nor mutates institutional state.
- Existing map IDs, event types, archive schema V6, checksum rules and protocol
  ownership are retained. Additive map and interaction enum variants use the
  existing map-entry and interaction events. Old events retain their existing
  spawn/replay behavior; the new far stair has a deterministic local arrival.
- Territory snapshots are reconstructed from the already-persisted living
  clock. No second save source or implicit allegiance migration exists. Tests
  replay both a pre-span event history and a history with span traversal, shifts
  and the far-stair arrival. Older binaries do not understand the new map IDs;
  newly created span saves must be opened with this version or newer.

House jurisdiction, Central Junction connections, Current Synthesis, refusal,
Bond authority and existing institutional state have not been changed by this
work. Pre-existing unrelated working-tree changes were retained.

## Files changed by this implementation

Paths are relative to the repository root. `src/gameplay/view.rs` already had
uncommitted changes; this implementation adds only the Boardwalk projection,
ambient dialogue and local exit presentation to that file.

| File | Change |
| --- | --- |
| `src/gameplay/boardwalk_territory.rs` (new) | Faction registry, territory identity, influence values, shift-based presence and immutable projection |
| `src/gameplay/world.rs` | Two span maps, notice/ambient interaction IDs, local exit and arrival positions |
| `src/gameplay/runtime.rs` | Local transfer validation, physical stairs, ambient interaction resolution and replay |
| `src/gameplay/deep_pressure.rs` | Keep existing scheduled campaign NPCs off the new spans |
| `src/gameplay/view.rs` | Territory/faction records, signs, dialogue and named local exits |
| `src/gameplay/mod.rs` | Register and export the new domain module |
| `src/kernel_invariants.rs` | Audit neutrality, reciprocal rivalry and preserved route boundaries |
| `src/gameplay/protocol.rs` | Attach the Boardwalk audit to the existing kernel audit report |
| `hueman_godot/scripts/retro_overworld.gd` | Deck environments, residents, stairs, level labels, contextual exit selection and accessibility |
| `tests/boardwalk_territory.rs` (new) | Influence/sovereignty separation, accessibility across shifts, traversal, replay and institution-preservation tests |
| `hueman_godot/tests/boardwalk_territory_smoke.gd` (new) | Runtime-fixture renderer/semantics check and optional render capture |
| `Cargo.toml` | Register the focused integration test target |
| `hueman_godot/README.md` | Player navigation and implementation reference |
| `docs/BOARDWALK_GOVERNING_DOCTRINE_V0_1_IMPLEMENTATION.md` (new) | This implementation and validation record |

## Validation

- Full workspace tests: 1,691 passed, including both neutral kernel crates,
  existing geography/route-network, Current Synthesis and Boardwalk case tests.
- Focused Boardwalk tests: 3 new territorial tests and 12 existing case tests
  passed. Every stair, sign and NPC conversation is reachable in all four
  shifts. Changing influence leaves legal status and physical presence separate.
- Existing Godot semantics smoke and new Boardwalk smoke passed using exported
  authoritative Rust views. Both deck renders were captured and inspected.
- Godot → Rust save/restart/load lifecycle passed. Existing art verification
  passed (2 assets, 16 colors).
- Strict all-feature Clippy passed for the library and the new territorial
  tests. The full workspace/all-target lint run is blocked by the pre-existing
  `clippy::print_literal` issue at `src/bin/warden_registrar_v03.rs:206`.
- Whole-workspace formatting check reports pre-existing formatting differences
  in `src/bin/warden_registrar_v03.rs`; this unrelated file was not modified.
  Formatting of all changed Rust files and `git diff --check` passed. The
  existing Godot geography-signage smoke also passed without changing its
  source map contract.

To regenerate the focused runtime fixtures and inspect presentation:

```sh
mkdir -p /tmp/boardwalk-review
BOARDWALK_VIEW_OUTPUT_DIR=/tmp/boardwalk-review cargo test --offline --test boardwalk_territory
BOARDWALK_VIEW_OUTPUT_DIR=/tmp/boardwalk-review godot --headless --path hueman_godot --script res://tests/boardwalk_territory_smoke.gd
```

For rendered screenshots, omit `--headless`, use
`--rendering-method gl_compatibility`, and append `-- --capture-boardwalk`.

## Deferred

- Player reputation, commerce transactions, diplomacy, combat and dispute
  choices; no new questline or irreversible faction allegiance.
- Arbitrary persistent influence overrides or player-driven transfers of
  dominance. The implemented ambient model derives from the persisted clock;
  future mutations need an authored reducer/event and replay validation.
- A bridge-wide conquest state is intentionally absent. Environmental cultural
  associations guide this first implementation without restricting later content.
- Continuous multi-map elevation/camera simulation and free-roaming faction AI;
  this implementation uses the project's reliable tile-map/physical-exit model.
