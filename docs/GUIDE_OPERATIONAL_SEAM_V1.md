# Guide Operational Seam V1

Status: implemented live-service profile

Prominent canon:

```text
Inside:
Roi and Roselina.

Outside:
Roï and Rosaliña.

Two beings.
Four form-names.
One continuous identity per Guide.
```

This profile connects the typed Guide state machine to the authoritative
gameplay service. It does not add an identity, a third Guide, or presentation
authority.

## Live Service Ownership

`GameProtocolService` owns one `DualGuideSession` alongside gameplay state.
The live service created with `new_with_roots` begins outside Hollow Grove
unless it restores an existing operational state. Test-only in-memory
constructors retain their historical inside starting context.

The wire service exposes:

- `EnterHollowGroveIntent`;
- `LeaveHollowGroveIntent`;
- the current `GuideContextV1`;
- ordered `GuideTransitionEvent` records in the response view;
- one operational revision shared by gameplay commands and Guide crossings.

All ordinary Hueman gameplay intents require the confirmed Hollow Grove layer.
Save, load, synchronization, and seam intents remain available at the
appropriate outside or inside boundary.

## Entry and Exit

Entry executes:

```text
Roï + Rosaliña
→ prepared durable journal
→ Central Junction [0,0,0]
→ paired Festival confirmation
→ Roi + Roselina
```

Exit executes the reverse operation. Both directions use one stable Guide
session and the same two stable identities. A successful crossing advances the
service revision exactly once.

The protocol never accepts a completed one-Guide crossing. The Rust
coordinator supplies the public names and Godot renders the resulting context;
Godot does not choose form, layer, or names.

## Durable Artifacts

For session `<session-id>` and save root `<root>`:

```text
<root>/<session-id>.guides.json
<root>/<session-id>.guide-transitions-v2/*.json
<root>/<session-id>.<slot-id>.hgplay.json
<root>/<session-id>.runtime.lock
```

The operational state records:

- canonical `DualGuideSession`;
- unified service revision;
- confirmed transition IDs already incorporated into that state.

Each slot save is an atomic service archive containing both the checksummed
gameplay archive and the canonical Guide session. Legacy bare gameplay
archives remain readable and migrate to the historical inside Guide context.

## Restart Recovery

Startup completes before the service accepts requests:

1. acquire the session runtime lease;
2. restore and validate the operational Guide state;
3. read the session transition journals;
4. recover any prepared transition continuing from the restored state;
5. incorporate any confirmed journal not yet checkpointed;
6. persist the reconciled operational state;
7. deliver recovery events with the next synchronization view.

The applied-transition set prevents old entry/exit journals from being
reapplied after a later round trip returns to the same layer.

## Concurrency

Within one process, the service mutex serializes connected clients.
`expected_revision` is compare-and-swap protection across gameplay and Guide
operations. A stale client is rejected before mutation.

Across processes, a session-scoped hard-link lease permits only one live
service owner. The lease records both process ID and operating-system process
start token, so a killed process cannot permanently strand the session and a
reused process ID does not impersonate the old owner.

## Presentation

The outside TUI or any other protocol client receives Roï and Rosaliña from
the kernel context. Godot synchronizes outside, submits
`EnterHollowGroveIntent`, waits for confirmed transition events, and only then
establishes or resumes Hueman gameplay as Roi and Roselina.

Transition events remain presentation-neutral. Their existing `render_tui`
projection and Godot screen-semantics projection consume the same kernel
records.

## Verification

`tests/guide_operational_seam.rs` proves:

- live outside naming and inside naming;
- entry and exit preservation of exactly two identities;
- unified revision behavior across a crossing and gameplay event;
- stale-client rejection and exact request replay;
- second-process exclusion;
- restart restoration after entry and exit;
- prepared-journal crash recovery before request service;
- atomic gameplay and Guide-form save/load continuity;
- Godot entry routing from the kernel-supplied outside layer.
