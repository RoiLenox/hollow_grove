# Survey Frame Migration V1

This is the approved additive foundation migration for the adopted
[doorway case](../STONEBEND_SURVEY_FRAME_FIRST_CASE_V1.md). The user's “Yes do
that” authorized settling its responsibilities and continuing implementation.
It adds object-case state/events and art-facing read-only workshop controls
while retaining the frozen House, identity, consent and renderer boundaries.

## Gameplay V5 → V6

The event enum adds `SurveyFrameChanged` with the typed request and exact
actor/revision/authority/resident/delivery result. The envelope payload layout
and checksum algorithm are unchanged. The application preserves the migration receipts when a loaded archive is saved again. New archives write schema V6; decoders
explicitly accept V1–V5 and append a versioned V5→V6 migration record. Those
schemas reject backported survey-frame events. Checksums are verified before
normalization/replay. A checksummed forged result is still rejected by replay.

Old archives retain their prior events, authorities, object-independent
identity, Forms, clinical history, relationships and causal/revision semantics.
Their survey-frame state begins at `AwaitingNeed`, revision zero, no artifact,
registration, proof, offers, resident decisions or deliveries. Migration never
creates work or acceptance. Legacy outputs rewrite only the explicit archive
version/migration metadata when migrated; current V6 decode/re-encode remains
byte-equivalent. Original saves are not rewritten during inspection.

The transaction candidate explicitly clones the new bounded domain field
while retaining V6's journal ownership optimization. Failed requests leave
all live domain fields and journal bytes intact. A lawful failed trial is an
accepted negative-result event and survives replay. Retry IDs preserve exact
idempotence and conflict semantics.

## Views and consumers

Protocol V1 adds a typed `AdvanceSurveyFrameIntent` and defaultable
`GameView.survey_frame`. The view includes the exact object revisions,
authority/proof, resident decisions, delivery/recovery records, available
actions and missing offices. Godot sends only the selected typed action.
Requests cannot set `passed`, actor, proof, custody or consent directly.

Lore inspection moves V3→V4 with separate `survey_frame` and
`survey_frame_contract` fields. Existing Boardwalk and Current Sea execution
fields retain their shape. Architecture inspection moves V1→V2 with an
additional survey-frame contract, leaving the original twelve-route roster
and foundation row intact. There are 22 bounded components, four verified
case bindings including the two additional contracts, and ten unbound
historical route contracts.

The new `kernel_audit` includes bounded object continuity, distinction between
measurement/support, separated authority/choice/delivery, retained negative
trial/repair history, and withdrawal recovery. Universal recursion/semantic
crates and four House functions are unchanged. No new office, Nature,
Recipe, Title, Rampart or continuous witness is introduced.

## Acceptance

Before release, validate all three endings, a retained failed trial, refusal
surviving repair, stale proof rejection, missing/conflicted authority,
advisory versus actual choice, scope-specific refusal, identity preservation,
duplicate requests, checksum-valid forged results and withdrawal/recovery.
Exercise schema-V5 and older saves before/after migration, current V6 replay,
protocol save/restart/load, Godot controls and response bounds. Run full Rust,
Clippy, asset and lifecycle gates and synchronize matching source/packages to
both machines with recovery records. Exact receipts live in `dist/`.
