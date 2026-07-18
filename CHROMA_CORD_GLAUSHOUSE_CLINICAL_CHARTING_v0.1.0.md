# chroma_cord Glaüshouse Clinical Charting v0.1.0

Date: 2026-07-18

## Status

Managed native-application integration implemented. Hollow Grove owns lifecycle,
identity, Glaüshouse attachment, access, projection, and privacy orchestration.
The external application remains independently buildable and no live
clinical-data synchronization is implied.

## Identity

`chroma_cord` is the clinical charting software used in Glaüshouse clinics,
local care centers, and hospitals. Its preserved implementation lives outside
this repository at:

`/home/warren/chroma_cord`

The application is built on the historical chroma_cord structured sequential
input language. Its chords preserve an ordered account of presence, performance,
perception, and position while chromatic hue makes clinical pressure legible at a
glance.

## Nightingale Use

Nightingales use `chroma_cord` to support their constitutional work of
recognition, clearing, and coordinated healing. They may chart clinical pressure,
review the sequence of observed events and actions, locate unresolved threats,
and challenge premature Clearance.

This does not redefine Nightingales as generic nurses or make Glaüshouse medicine
species-exclusive. Mixed-species clinicians may use the same software under the
institutional rules of their clinic or hospital.

## Authority Boundary

- `chroma_cord` is an application managed above the Hollow Grove kernel.
- A chart entry is evidence and presentation, not constitutional truth by itself.
- The software does not grant access, authorize treatment, or declare Clearance.
- Hollow Grove applies the existing clinical institution's identity, consent,
  access, and projection policy; it does not invent those facts.
- `chroma_cord` retains authority over chord grammar, hue rules, input behavior,
  and its append-only store.
- Hollow Grove and Hueman must continue to function without this application.

## Canonical Naming

- Canonical project identifier: `chroma_cord`.
- Canonical local path: `/home/warren/chroma_cord`.
- Rust packages and binaries, Python display names, data directories, database
  defaults, Plover integration, active documentation, and project-owned document
  names use `chroma_cord`.
- Existing chart data was moved intact to the canonical data directory; the
  naming migration did not reinterpret or discard chart content.

## Implemented Integration

The integrated control path now consists of:

- `src/application_protocol.rs`, the typed application registry, ownership split,
  scope ceiling, Glaüshouse anchor, access gate, and privacy contract;
- `launch-chroma-cord.sh`, which gives the native window the exact managed Niri
  application identity `hollow-grove.chroma-cord`;
- the existing Hueman pair controls, which attach the focused managed window to
  an explicit Glaüshouse anchor without adding or changing keybindings;
- a semantic-only Godot surface that refuses clinical window capture;
- runtime validation that rejects application intents whose identity, window,
  lifecycle, projection, or world anchor drifts from the registry; and
- `src/world/chroma_cord.rs`, which supplies a complete four-phase chart entry
  with a clinical hue;
- a Glaüshouse chart-view policy based on existing Nightingale, recovery-staff,
  Persephone, Prima Donna, and explicit-grant facts;
- projection into the existing `ClinicalFinding` presentation object only after
  the existing institutional gate allows access; and
- a deterministic Nightingale/outsider witness proving that chart evidence does
  not manufacture Clearance.

The operating ceiling and upgrade/rollback procedure are defined in
`HOLLOW_GROVE_APPLICATION_SCOPE_AND_MAINTENANCE_v0.1.0.md`. The next safe
extension is health/version negotiation followed by a versioned transfer record
from the append-only application store into the adapter. Identity, consent,
retention, and treatment authority must continue to come from the clinical host.
