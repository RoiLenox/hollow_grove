Current extension: [the adopted survey-frame case](../STONEBEND_SURVEY_FRAME_FIRST_CASE_V1.md)
now has its own fourth verified binding. Lore inspection V4 and architecture
inspection V2 supersede the original report versions below. The Current Sea
case's domain outcomes remain unchanged. Current witness exports use gameplay
V6, lore V4 and architecture V2; the prior V5 evidence remains preserved.

# Stonebend Continuity Lore Binding V1

This pass connects the already implemented Current Sea continuity case to
read-only lore inspection and architecture coverage. It also develops a
separate [survey-frame first-case proposal](../STONEBEND_SURVEY_FRAME_FIRST_CASE_V1_DRAFT.md).
At this original checkpoint the frame was still a proposal; this binding attests Mercy
Deep's existing case.

## Existing canon and execution

`lore.stonebend.current-sea-continuity` now binds to
`case.stonebend.current-sea-continuity.v1`, owned by
`src/gameplay/stonebend.rs` and described in
[Current Sea Vertical Slice](../STONEBEND_CURRENT_SEA_VERTICAL_SLICE_V1.md).
The historical twelve-route roster stays unchanged. Together with Boardwalk
Return and the additional Sympiote foundation, there are now three verified
gameplay bindings and ten unbound historical routes. Core architecture
coverage remains a bounded 21-component crosswalk.

The projection requires the actual case, evidence/faculties, supported choice,
committed event and matching subject/outcome/decision/authority/causal position.
It validates the existing typed Name/Seal determination, shows the accepted
live Hypergiant Name decision and projects the same immutable outcome already
used by GameView. Support alone creates no execution decision or completion.
Vacant authority appears as a pending stop and cannot be manufactured by
inspection. Checksum-valid forged authority is rejected during replay.

All three existing dispositions are supported: affirm the existing Name,
link a provisional transformed-form Name, or seal a referral for high review.
The bounded disposition can be complete while high review is still pending.
A referral records no final Name or Title; that limit remains explicit in the
report. The former Name, provisional status, uncertainties and challenge
terms stay visible. No new executor, delivery, witness or Freemason bearer
receipt is inferred from authored capacity labels: the existing Name decision
and typed Seal record are the records actually available.

## Read-only schema V2 → V3

Lore inspection has its own versioned output, now schema V3. It is not a
gameplay archive or independent authority evidence. The Boardwalk `execution`
field keeps its existing shape. The optional new `stonebend_execution` field
is emitted only for an actually started Stonebend case; it contains typed
stops, event/decision witnesses, the existing outcome projection and explicit
high-review/completion scope. An unplayed definition has a binding but no
execution. A source declaration cannot finish a case.

Gameplay archive V5, protocol V1, GameView, event IDs and payloads, reducer
behavior, House jurisdiction, consent/refusal, causal ordering, Rampart
binding and neutral kernels are unchanged. The architecture report remains
schema V1 with an additional verified row using its existing fields. No
persisted save migration or art-facing projection change is required.

```bash
cargo run --locked --offline --bin hollow-grove -- lore --save archive.json --json
cargo run --locked --offline --bin hollow-grove -- architecture --source-root . --save archive.json --json
cargo run --locked --offline --example stonebend_continuity_witness -- dist/stonebend-continuity-witness
```

The witness example writes all three existing V5 gameplay archives and their
inspection/coverage reports from disposable state. The prior V6 CLI can
validate/replay those same archives and produces its historical V2 unbound
Stonebend report; the new CLI produces V3 recorded execution. Archive bytes
stay unchanged after both reads. Raw before/after outputs belong in `dist/`.

## Validation

Four added integration tests prove unstarted versus supported versus committed
execution, all three dispositions and exact outcome equivalence, byte-identical
save/replay and read-only inspection, missing evidence/faculties/live authority,
and checksum-valid authority forgery rejection. Existing Boardwalk and
Sympiote inspection tests remain intact apart from the additive coverage/schema
expectations. Full workspace validation and exact source/tool synchronization
receipts are recorded with this checkpoint in `dist/`.

New object-lineage lore still requires adoption and its own complete typed
case, events, compatibility record, replay/refusal proof and presentation.
Mercy Deep's implemented identity case supplies an architectural example,
not completion of that new story.
