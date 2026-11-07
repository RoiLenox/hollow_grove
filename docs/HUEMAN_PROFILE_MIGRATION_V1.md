# Hueman profile archive migration: V6 to V7

The HUEMAN-02 source entry adds one authoritative event, `HuemanCreated`, with
a version-1 cosmetic profile. Writers now emit gameplay archive V7. The existing
envelope fields, checksum algorithm, institutional snapshot and Rampart binding
remain unchanged. V1–V6 input remains readable through the existing migrations
and the additional `migration.gameplay.v6-to-v7.hueman-profile` record.

Reading an older archive does not write it. Re-encoding produces V7 and records
the migration once; migrating that result again is byte-idempotent. Historical
events, identities, position, object cases, Forms and authority retain their
meaning. Legacy characters acquire no invented cosmetics, introductory mission,
or new identity. A profile event in any pre-V7 envelope is rejected, even with a
valid payload checksum. Unsupported future versions continue to fail closed.

`CreateHuemanIntent` accepts only schema version, avatar ID and Skin/Face/Hair
presets. Rust validates the vocabulary, establishes the existing continuity,
participant and institutional identity references in one transaction, starts
the character on the existing Aura Way map, and records the profile in the
same replayable event. A duplicate or invalid creation leaves state unchanged.
The avatar ID is a local identity namespace, not an authenticated account or a
Stonebend Name, Title, office, consent, proof or Permanence Seal.

The gameplay archive owns the accepted profile. The creator's separately saved
appearance is only a draft; changing it later does not overwrite an established
character. The entry resumes the complete saved world before considering a
new creation. It preserves legacy-character saves; the existing game launcher
remains their supported editing/playing path. Automatic legacy cosmetic adoption
and multiplayer account ownership remain future work.

The optional introductory objectives are a read-only projection of recorded
creation, movement, the exact Aura Way witness interaction and Survey Frame
events. The existing repair case still enforces custody and institutional
authority. The existing Winged Gargoyle pilot owns Form changes; returning to
Hueman restores the same base appearance. The `baseline_stats` view reads
`hollow_deck::HUEMAN_STATS`: Strength, Speed, Attack and Defense, each 50. These
are base-Hueman catalog values, not faculty scores, current Form statistics or
newly earned progression.

The opt-in source launcher uses `session.hollow-grove.hueman`, distinct from the
ordinary installed/local-world session. Test runners use temporary save roots.
V7 writes must not be handed to a V6-only installed Core. Keep prior binaries
with their compatible saved-state backup when rolling back; no downgrade is
implied. The existing device handoff requires identical installed payloads and
does not transfer this source-development world automatically.

The active-geography policy records the reviewed changes to the archive and
runtime source hashes. Its original October 27 inventory and all compatibility
fixtures remain intact. The review adds profile creation/replay on an existing
route; it changes no route edge, retired geography, founding law or succession.

Validation includes legacy V1/V2/V3/V5 fixtures, a V6 identity archive, once-only
migration, backdated profile rejection, identity/appearance/replay continuity,
invalid creation, unrelated witness rejection and service restart/save recovery.
The full integration runner additionally exercises the Godot creator and four
independent Rust-service lifecycle processes.
