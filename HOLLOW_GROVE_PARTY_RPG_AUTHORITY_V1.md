# Hollow Grove Party RPG Authority Boundary V1

Status: implementation architecture

This document records the first implementation boundary for the compact
top-down party RPG. Pokémon Yellow is a scale, pacing, readability, party, and
menu-structure reference only. Hollow Grove does not import Pokémon assets,
characters, maps, dialogue, music, names, code, or proprietary content.

## Authority

Current Synthesis and the Hollow Grove runtime remain authoritative over:

- stable identity and active incarnation;
- Current, Aura, Frame, Flow, and Glow;
- Bonds and their full constitutional lifecycle;
- House and institutional authority;
- movement acceptance, progression, recognition, and unlocks;
- Synthesis legality, predecessor identity, lineage, and result identity;
- persistence, provenance, replay, and constitutional validation.

For Flynt specifically, `flynt-constitution` owns the Tross-rooted public and
underground commands, the one-person Tross/Mystery Man/Mr. X identity lock, the
distinct Manticorp Form and Manticorp Institution, the three divided Recipe
custodians, the three Founding Leader lineages, the Gallowry distinction, and
the one constitutional Chimera synthesis. Party, encounter, progression, and
Godot layers may project these facts but may not create another Chimera,
duplicate the Manticorp Form, split the Tross identity, or infer succession.

Godot is a presentation client. It may render maps, sprites, movement,
followers, dialogue, menus, encounters, sound, transitions, and effects. It
submits typed intents and presents committed runtime events. It does not infer
or commit authoritative outcomes.

The universal recursion kernel remains below gameplay and constitutional
adapters. Kernel code must not import gameplay, party, map, encounter, Godot,
or presentation types.

## First Runtime Seam

`src/gameplay/` owns the new presentation-neutral aggregate boundary. Its first
increment establishes:

- one caller-supplied `BeingContinuityId` for each gameplay Being;
- a permanent Hueman continuity mapped to legacy `BeingId::Hueman`;
- separate `ParticipantId`, `InstitutionalBeingId`, and `RegionalBeingId`
  references rather than a universal replacement ID;
- one authoritative game revision and append-only gameplay event sequence;
- atomic candidate application across the game identity index and regional
  reducer;
- deterministic reconstruction from gameplay events;
- authoritative tile movement, collision, facing, and faced-tile interaction
  through a typed map registry containing all ten constitutional routes and
  the three singular interior surfaces;
- the `boardwalk.return-vestibule` case map, evidence packet, five-faculty
  disclosure gate, supported option, and Returning-Goon-owned commitment;
- typed lawful outcome records for all four Boardwalk choices;
- actual finite Pimp Patronage, Goon Bond, and Limited Cooperation Bonds,
  formed, validated, and activated by the common constitutional runtime;
- Independent Return as a persistent protected refusal with no Bond;
- live `WorldSession` authority for Glaüshouse discharge Clearance, Stonebend
  Name, Sandmanor proof, and Flynt recognition;
- schema-V3 gameplay event archives with deterministic checksums, embedded
  institutional authority state, explicit Runtime Federation binding, V1/V2
  migrations, and exact replay for route, Aura Field, Aura Beach, Aura Basin,
  Current Sea, and Boardwalk entry.
- an event-sourced six-person party consisting of Hueman plus at most five
  companions;
- capable-subject recruitment derived from Deep Pressure memory, condition,
  outcome, trust, role boundary, and physically faced conversation;
- persistent debt-free refusal, resting recovery by living-world shift,
  selected member, ready lead, and one map-bound field action per recruit;
- rename-safe role identities and exact party archive replay.

Regional Synthesis will later change the active `RegionalBeingId` under a
stable `BeingContinuityId`. The predecessor and result remain distinct records.

## Canonical Migration Decisions

The target party-RPG compass is:

- Stonebend: north;
- Sandmanor: east;
- Glaüshouse: south;
- Flynt: west.

The repository currently contains an older typed Fourway mapping with
Glaüshouse east and Sandmanor south, while the checked screen-map artifact
already uses the target layout. World implementation must version and migrate
the direction mapping by stable House identity. It must not reinterpret stored
direction values in place.

`Aura Crossing` and `Current Summit` are new map identities. `Aura Field` is
the one canonical geographic and regional name. Older plural machine symbols,
including `AuraFields`, `AuraFieldsStewardship`, and
`site.sandmanor.aura-fields`, remain compatibility identifiers only; they never
mean that more than one Aura Field exists.

Gnome-to-Minotaur is regional Synthesis under the V2 constitution, not generic
experience-based evolution. Minotaur-to-Hecaton and Centaur-to-Pegasus remain
known adjacent forms without ratified gameplay execution.

## First Non-Goals

This seam does not yet implement general follower formation behavior, combat
recruitment, four-action preparation, encounters, equipment, levels, or
general-purpose quest scripting. Fixed route and interior-surface entry,
Boardwalk's bounded capable-subject choice, Current Sea continuity, and save/load are
implemented. Those bounded capabilities do not imply procedural world travel,
a generic dialogue tree, a generic relationship framework, or unrestricted
world saves. Deep Pressure later adds one bounded authored relationship-memory
aggregate and one cross-region campaign; that vertical slice does not convert
the non-goals into generic engines.

## Gameplay Protocol V1

`src/gameplay/protocol.rs` defines the versioned presentation boundary. Every
request carries a protocol version, session ID, request ID, expected game
revision, and typed intent. Exact request retries return the exact prior
response; conflicting request-ID reuse and stale mutation requests fail closed.
`SyncIntent` returns the current immutable view so a presentation client can
recover from a stale revision.

Protocol V1 implements movement, faced-tile interaction, typed entry across
the ten-route network and into all three interior surfaces, faculty disclosure,
Boardwalk and Current Sea option support and commitment, and versioned
checksum-verified save/load. Party opening and selection, capable-subject
recruitment, lead switching, and companion field actions are implemented.
Synthesis remains a reservation; the service returns `CapabilityUnavailable`
and commits no event.

`src/bin/hollow_grove_game_service.rs` hosts the same JSONL protocol over stdio
or a loopback-only TCP listener. `hueman_godot/scripts/runtime_client.gd` is a
transport adapter: it sends intents, tracks the returned revision, and emits
immutable response dictionaries. It contains no gameplay reducer.

The current gameplay archive intentionally rejects histories containing a
regional-registration event. That older seam already has its own persistence
model, and silently producing a partial mixed archive would be unsafe. The
implemented disk-save capability therefore covers the playable Hueman, all
route and surface entry, Current Sea, Boardwalk, their typed outcomes and
relationship Bonds, and their exact House-authority snapshot; mixed regional
gameplay history remains an explicit future migration.

The service accepts `--world-root` separately from `--save-root`. World root
loads schema-V2 dynamic institutional state and fails closed on malformed
records. Save root stores gameplay slots. Loading a slot replays against the
authority snapshot embedded in that slot, so a later office change cannot
silently rewrite the historical decision.
