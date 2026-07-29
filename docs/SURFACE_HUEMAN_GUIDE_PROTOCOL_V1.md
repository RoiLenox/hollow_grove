# Surface–Hueman Guide Protocol V1

Status: canonical session, phase, model, and presentation contract

All new adapters consume the versioned `GuideContextV1` projection and
presentation-neutral transition events defined in
[Guide State-Machine Hardening V2](GUIDE_STATE_MACHINE_HARDENING_V2.md).

## Continuous Session

One dual-Guide session continues through:

```text
outside
→ Central Junction
→ inside Hollow Grove
→ Central Junction
→ outside
```

The session carries both stable identity form-states, current layer, focused
object, current witness, and pending transition. A public-name change never
starts a new conversation.

## Entry

```text
Roï and Rosaliña active outside
        ↓
The Way Back restores outside state
        ↓
The Initiation creates an entry proposal
        ↓
The Gathering verifies object, session, destination,
incarnation, continuity, embodiment, and paired transition
        ↓
The Festival confirms atomically
        ↓
Roï becomes Roi
Rosaliña becomes Roselina
        ↓
Roi and Roselina appear inside Hollow Grove
```

## Exit

```text
Roi and Roselina active inside
        ↓
The Way Back restores inside state
        ↓
The Initiation creates a return proposal
        ↓
The Gathering verifies Hueman result, Surface destination,
external consequences, continuity, embodiment, and paired transition
        ↓
The Festival confirms atomically
        ↓
Roi becomes Roï
Roselina becomes Rosaliña
        ↓
Roï and Rosaliña resume outside guidance
```

## Dialogue Routing

Speaker identity is one of Roi, Roselina, or Both. There are no separate
speaker identities for Roï or Rosaliña.

The display layer resolves speaker names from the confirmed Guide context:

- Roi identity renders Roi inside and Roï outside.
- Roselina identity renders Roselina inside and Rosaliña outside.
- Both renders `ROI + ROSELINA` inside and `ROÏ + ROSALIÑA` outside.

## Model Context

Rust supplies active layer, forms, names, jurisdictions, and transition state
to every model request. Model output cannot set or alter them.

The model instruction is exactly:

```text
There are two continuous Guide identities.

Inside Hollow Grove:
Roi and Roselina.

Outside Hollow Grove:
Roï and Rosaliña, their Seer forms.

Roi and Roï are the same being.

Roselina and Rosaliña are the same being.

Use only the names supplied in the active Guide context.

Do not invent a generic third Guide.

Do not refer to the inside forms by the outside names.

Do not refer to the outside forms by the inside names.

Do not decide or alter Guide form.

Guide form is supplied and confirmed by the Rust kernel.
```

The Ollama adapter is infrastructure. It may interpret an action proposal; it
cannot choose speakers, names, forms, layer, authority, or transition result.

## Presentation

Outside TUI, terminal, overlay, and future voice surfaces present Roï and
Rosaliña. Inside Godot presents Roi and Roselina as embodied Guides. Both read
the same stable identities and layer-derived names from Rust.

The TUI and Godot bridge are interfaces, not characters.

## Witness

A confirmed crossing witness includes stable identity, source public name,
source form and layer, destination public name, destination form and layer,
direction, session, continuity assessment, embodiment assessment, Federation
decision, and resulting session state.

Historical witness text remains immutable. Migration attaches corrected
interpretation metadata instead of rewriting prior audit prose.

## Operational Checks

```text
cargo run --bin current_synthesis_tui -- guides status
cargo run --bin guide_transition_audit
```

The first command presents the outside Seer forms through the TUI. The second
executes an entry and exit cycle, proving paired transition, stable identities,
and stable session continuity.
