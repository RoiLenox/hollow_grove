# Guide State-Machine Hardening V2

## Constitutional statement

```text
Inside Hollow Grove:
Roi and Roselina.

Outside Hollow Grove:
Roï and Rosaliña.

Two beings.
Four form-names.
One continuous identity per Guide.
```

V2 does not change the V1 canon. It hardens the execution boundary so adapters
cannot construct a completed asymmetric crossing or choose a public name.

## Typed stable and transition states

New callers use:

```text
OutsideDualSeerSession
        ↓ begin_entry
EnteringAtJunction
        ↓ Festival confirmation
InsideDualGuideSession
        ↓ begin_exit
LeavingAtJunction
        ↓ Festival confirmation
OutsideDualSeerSession
```

The wrapped `DualGuideSession` fields are private. Constructors accept only a
fully validated paired state. Central Junction progresses explicitly through
`Approaching`, `AtOrigin`, and `FormChanging`.

The V1 records remain readable for save compatibility, but the typed V2 API is
the authoritative route for new crossings.

## Durable Festival transaction

Every durable crossing writes a prepared journal before changing form state.
The journal contains:

- the complete source session;
- the paired transition and both proofs;
- the stable transition and session identities;
- presentation-neutral transition events;
- the terminal witness and resulting session, or the rejection.

Writes use a flushed sibling file, atomic rename, and parent-directory sync.
A repeated transition ID returns the existing terminal result. Reusing the ID
for different input is rejected as an idempotency conflict.

A prepared journal is noncanonical but recoverable. Recovery reconstructs the
typed source state, completes the Festival once, and records a recovery event.

## Shared adapter contract

`GuideContextV1` is the only new Guide projection supplied to TUI, Godot,
Ollama, overlays, and future voice adapters. It contains:

- `schema_version`;
- stable `session_ref`;
- kernel-selected layer;
- exactly two Guide descriptors;
- stable identities and jurisdictions;
- kernel-derived public names;
- optional explicit seam state and transition reference.

The generated schema is stored at
`schemas/guide-context-v1.schema.json`. Canonical inside and outside fixtures
are stored under `tests/fixtures/`.

Adapters render this contract. They do not modify it.

## Transition events

The state machine emits:

```text
Proposed
JournalPrepared
Approaching
AtOrigin
FormChanging
Confirmed | Rejected
Recovered | Replayed
```

Events carry `GuideContextV1`, so each presentation layer can animate or speak
the same transition without owning identity logic.

## Audit

Run:

```text
hollow-grove guides audit
hollow-grove guides audit <journal-directory>
hollow-grove guides schema
```

The audit reports corrupt records, duplicate transition identities, invalid
form/layer combinations, mismatched sessions, incorrect witness names, and
prepared crossings that still require recovery.

An empty journal directory is clean. A prepared journal is an error until the
recovery operation reaches a confirmed or rejected terminal state.

## Migration

Migration artifacts now carry an explicit migration schema and ordered applied
lineage. Older V1 artifacts without lineage are upgraded on read without
rewriting identity records or historical witness text. Encoding is
deterministic, and applying the same source migration twice produces the same
canonical result.
