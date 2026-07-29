# Guide and Seer Identity V1

Status: canonical identity and presentation contract

The constructor-controlled states, shared adapter contract, durable crossing
journal, recovery, and invariant audit are defined in
[Guide State-Machine Hardening V2](GUIDE_STATE_MACHINE_HARDENING_V2.md).

## The Rule

```text
Inside:
Roi and Roselina.

Outside:
Roï and Rosaliña.

Two beings.
Four form-names.
One continuous identity per Guide.
```

Roi is inside Hollow Grove. Roï is Roi outside Hollow Grove in his Seer form.
Roselina is inside Hollow Grove. Rosaliña is Roselina outside Hollow Grove in
her Seer form.

The stable identities are always:

```text
guide.roi
guide.roselina
```

No crossing issues a new identity. No interface owns either identity.

## Identity, Form, and Layer

Identity is one of:

```text
Roi
Roselina
```

Form is one of:

```text
HollowGroveGuide
Seer
```

Layer is one of:

```text
OutsideHollowGrove
CentralJunction
HollowGrove
```

The ordinary confirmed combinations are:

| Stable identity | Form | Layer | Public name |
|---|---|---|---|
| `guide.roi` | `HollowGroveGuide` | `HollowGrove` | Roi |
| `guide.roi` | `Seer` | `OutsideHollowGrove` | Roï |
| `guide.roselina` | `HollowGroveGuide` | `HollowGrove` | Roselina |
| `guide.roselina` | `Seer` | `OutsideHollowGrove` | Rosaliña |

Central Junction presence is valid only within an explicit witnessed
transition. Identity alone never determines the public name.

## Jurisdiction

Roi/Roï continuously governs when, history, continuity, causality, sequence,
replay, succession, provenance, memory, and return.

Roselina/Rosaliña continuously governs where, what, embodiment, world-state,
territory, objects, containment, ecology, adaptation, consequence, and
incarnation.

Changing form changes manner of presence, not jurisdiction or authority.

## Inside Roles

Roi and Roselina are embodied characters, companions, Guides, and equal rulers
of Central Junction. Godot receives their names from the authoritative Rust
view; sprites and dialogue do not choose or mutate identity.

## Outside Roles

Roï and Rosaliña guide through desktop, terminal, TUI, overlay, notifications,
file and project context, operating-system state, external tools, and future
voice interfaces.

Joint outside presentation may say `ROÏ + ROSALIÑA` or `THE SEERS`. Joint
inside presentation may say `ROI + ROSELINA`. Neither heading creates a new
individual.

## Infrastructure Boundary

`SeerRuntime`, `OutsideGuideInterface`, `JunctionInterface`,
`DualSeerPresentation`, `GuideTransport`, and `SeerDialogueService` describe
machinery.

The TUI, terminal, overlay, voice interface, Ollama client, Rust kernel, Godot
bridge, notification system, and hotkey invocation are not Guide identities.

## Migration

Only exact known aliases migrate:

- Roi and Roï map to `guide.roi`.
- Roselina and Rosaliña map to `guide.roselina`.
- the historical ASCII misspelling `Rosalina` may be retained as migration
  metadata but is never emitted as a canonical public name.

Migration merges duplicate representations only when an exact recognized alias
agrees with an explicit base-identity hint from the source record.
If a legacy record incorrectly placed an accented outside name inside Hollow
Grove (or the reverse), migration preserves that original ID, name, and witness
text, records the source conflict, and derives the corrected active name from
the explicitly supplied current layer. It never guesses current state from
legacy text and never merges unrelated records merely because a display name
resembles one of the Guides.
