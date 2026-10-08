# Sympiote Foundation Migration V1

Date: 2026-10-08

Status: applied additive foundation migration; authorized by the user's request
to incorporate the accepted starter Form into the kernel/runtime, then to use
the established Sympiote name throughout.

## Contracts and versions

Gameplay archive V5 adds `SympioteFoundationChanged`. Its envelope and checksum
algorithm retain their existing shape. V4 receives an explicit V4→V5 migration
record; V1–V3 retain their prior migrations and then receive this record.
Versions before V5 reject foundation events even when their payload checksum
is valid. Unknown future versions continue to fail closed.

Migration preserves prior identity, graft-free history, existing pilot Forms,
office holders, and recorded outcomes. A legacy archive starts with an empty
foundation state. It receives no inferred consent, graft, practice, clinical
receipt, membership, or later Synthesis. Migration is idempotent.

The original Sympiote graft archive HGSYM/V1, Deck catalog and cultivation
state, functional-lore archive V2, House authority, Guide identities, Rampart,
and neutral recursion/semantic crates are unchanged.

Protocol V1 adds `AdvanceSympioteFoundationIntent`. Clients send typed requests
and personal choices; they cannot supply an authority actor, success flag,
resulting Form, privilege, or arbitrary power package. The core checks the
active clinical office in its loaded world and records the actual result.

GameView adds defaultable `sympiote_foundation` data, so older view records
deserialize with an empty foundation. Clinical receipt projection retains the
latest 32 records; archives retain full history. Existing Godot rendering
continues to consume its established fields. No clinic scene, sprite, palette,
or movement/combat statistic is created by this migration.

Lore inspection V2 adds the separate typed foundation contract and replay-backed
foundation projection. The historical twelve-route roster remains exact.
Read-only inspection still verifies a supplied archive without rewriting it.

## Causality, consent, and replay

Each parent action retains the gameplay event's causal position and subject.
The seven reused graft actions use local positions derived as eight times the
parent position plus the substep index. Checked arithmetic rejects overflow
without partial mutation. No global causal ordering rule changes.

The reducer is clone-and-commit. Missing identity/authority, changed operator,
wrong stage, missing rest, subject substitution, and invalid arithmetic leave
the live state and archive untouched. Withdrawal forms no body or Bond.
Post-establishment strain retains the body and opens a care obligation.

Replay reconstructs state and graft history from the parent actions and loaded
institutional world, then compares each complete event including clinical
receipt, subject, scope, phase, rest, and milestone. A forged receipt is
rejected even after recomputing a valid envelope checksum.

## Before and after fixtures

`src/gameplay/archive.rs::tests::v4_migration_preserves_existing_form_and_adds_no_sympiote_history`
constructs a V4 fixture with a continuing Hueman and an existing Gargoyle pilot
Form, migrates to V5, replays it, and verifies unchanged identity, pilot state,
events, and an empty foundation. It also verifies an idempotent second migration.

`old_envelopes_cannot_smuggle_sympiote_events` verifies that valid-checksum
V3/V4 envelopes reject the new event family. The integration suite
`tests/sympiote_foundation.rs` exercises establishment, withdrawal, rest,
repeatability, strain/review, actor changes, all five practice directions,
both Boardwalk offers, every replay checkpoint, request retries, tampering,
and overflow/client override rejection.

## Invariant audit

`audit_sympiote_foundation` is joined into the existing read-only service
kernel audit. It checks host/Form continuity, clinical authority and consent,
rest/repeatability, typed responsibility handoffs, and the distinction between
exploring a path and granting a later Form.

The detailed adopted contract is
[Sympiote Foundation V1](../HOLLOW_GROVE_SYMPIOTE_FOUNDATION_V1.md).
The V3 transfer kit remains a preserved prior milestone. A later release must
be built from this migration's committed source before that new code is sent.
