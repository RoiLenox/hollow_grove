# Hollow Grove Application Scope and Maintenance v0.1.0

Date: 2026-07-18

## Purpose

This protocol governs native applications that Hollow Grove presents inside the
Hueman/Glaüshouse experience. Its first registered application is `chroma_cord`.
The objective is a smooth attachment that remains explicit, recoverable, and
unable to acquire clinical authority merely because it is visible on screen.

The machine-readable source of truth is
`artifacts/hollow_grove_application_registry.json`. The typed source is
`src/application_protocol.rs`.

## System Boundary

| Layer | Owns | Must not infer or seize |
|---|---|---|
| Hollow Grove | application identity, lifecycle orchestration, Glaüshouse attachment, institutional access, semantic projection, privacy policy | chord grammar, hue rules, or committed chart content |
| `chroma_cord` | four-part chord grammar, hue calculation, input behavior, append-only clinical store | Hollow Grove identity, institutional clearance, world placement, or projection policy |
| Hueman/Godot | rendering the attachment and emitting a narrow intent | clinical content, access decisions, or runtime side effects |
| Niri | window focus and placement | world meaning or institutional authority |

Window geometry is presentation evidence. It never establishes a world
attachment. `chroma_cord` is attached to Glaüshouse only when its exact managed
window identity and the registered world anchor both agree.

## Minimum Scope

These capabilities are the smallest complete Hollow Grove integration and must
continue to work together:

- launch and lifecycle control;
- exact native window identity;
- explicit Glaüshouse world attachment;
- existing institutional access evaluation;
- semantic-only projection; and
- clinical capture prevention.

If any minimum capability is unavailable, the application may keep running, but
Hollow Grove must treat it as detached or masked rather than guessing.

## Maximum Scope

Hollow Grove may additionally coordinate health/version checks, supervised
protocol migrations, backup timing, rollback, and orphaned-window recovery.
This is the authority ceiling, not a list of features that must be built.

Even at maximum scope, Hollow Grove is prohibited from:

- rewriting chord grammar or hue semantics;
- mutating or deleting committed clinical records;
- manufacturing Clearance or institutional access;
- bypassing identity, consent, or retention rules;
- capturing the clinical window; or
- copying the clinical store into Hollow Grove artifacts.

Any proposal beyond this ceiling requires a new reviewed protocol version. It
must never arrive as an incidental launcher, renderer, or migration change.

## Health States

- **Green:** registry, launcher, app identity, privacy boundary, data readability,
  and relevant tests agree.
- **Amber:** the minimum integration works, but repository provenance, optional
  tooling, version evidence, or a non-runtime maintenance item needs attention.
- **Red:** identity or registry mismatch, invalid data, capture-policy regression,
  missing launcher/binary, failed contract test, or unauthorized scope expansion.

Amber does not authorize an automatic rewrite. Red causes attachment to fail
closed; it does not authorize changing chart data.

## Maintenance Cadence

Run the light check after changing a launcher, pair script, registry, app name,
data location, Niri identity, or Godot projection:

```bash
./maintain-chroma-cord.sh
```

Run the full check before a release, protocol/schema change, storage migration,
or restored backup:

```bash
./maintain-chroma-cord.sh --full
```

The light check is read-only. The full check compiles and tests both codebases
and asks Godot to parse the project headlessly; it still does not launch the
clinical UI or modify clinical records.

## Upgrade Protocol

1. Record the old and proposed registry, application, and transfer schema
   versions.
2. Run the light check and resolve Red conditions before touching a migration.
3. Create a recoverable backup through the application-owned storage process.
   Hollow Grove may coordinate timing but must not ingest the backup.
4. Dry-run the versioned transfer or migration against disposable data.
5. Run the full check, then smoke-test launch and focused-window attachment.
6. Confirm the live pair state says `application_world_anchor`, `attached`,
   `semantic_only`, and `capture_allowed: false`.
7. Keep the previous protocol/launcher available until the new attachment has
   completed a normal working session.

Compatibility should cover the current protocol and the immediately previous
protocol whenever a migration crosses a schema boundary.

## Failure and Rollback

On an identity, projection, or privacy failure:

1. detach or mask the projected surface;
2. leave `chroma_cord` and its data intact;
3. restore the previous Hollow Grove registry/launcher/projection code;
4. rerun the light check, then the focused-window smoke test; and
5. record the failure cause before attempting the upgrade again.

Never roll back by rewriting committed chart history. Data recovery belongs to
the application storage procedure and requires an explicit operator decision.

## Current Maintenance Notes

The Hollow Grove Git remote and local authorship identify Roy Lenox. The local
`chroma_cord` tree is canonically named and authored by Roy Lenox, but its
configured origin may still point to the preserved historical repository. That
is an Amber provenance condition until a deliberate `RoyLenox/chroma_cord`
remote exists and is selected; maintenance must report it without silently
changing or deleting the historical remote.

The active `chroma_cord/rust` crate currently compiles but has no in-tree unit
tests. Eight chord/database tests remain in the retained `chroma_cord-project`
implementation. This is also Amber: migrate or recreate coverage against the
active grammar, hue, input, and store modules before adding a live transfer
protocol or storage migration.
