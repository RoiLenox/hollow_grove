# Godot Semantic Event Protocol V1

Schema:

```text
hollow-grove-semantic-event-v1
```

This is an outbound, Rust-owned presentation contract. Godot may display these
events, animate them, and retain nonauthoritative UI history. It may not write
semantic positions, assign IDs, confirm proposals, create evidence, or bypass
Rampart and The Festival.

The event vocabulary is:

```text
semantic_position_proposed
semantic_position_confirmed
semantic_position_rejected
semantic_translation_proposed
semantic_translation_confirmed
semantic_translation_rejected
central_junction_touched
central_junction_crossed
festival_completed
```

Every event is wrapped with `schema_version`. Rust constructs cycle, proposal,
Point, scale, and witness references. Unknown JSON fields are rejected by the
Rust schema.

## Presentation law

```text
Proposed Aura:
temporary, translucent, unconfirmed

Confirmed Current:
persistent, solid, witnessed
```

Rejected proposals remain visible as witnessed decisions but never appear as
canonical Current.

## Origin relation

`central_junction_touched` and `central_junction_crossed` describe the vertical
origin-axis relation of a straight-line V1 translation. A crossing may have a
nonzero X coordinate. It does not claim that the path passed through exact
Central Junction `[0,0,0]`.

## Example

```json
{
  "schema_version": "hollow-grove-semantic-event-v1",
  "event": "semantic_translation_confirmed",
  "cycle_id": "cycle.semantic.repair.041",
  "proposal_ref": "proposal.semantic.repair.041",
  "point_ref": "point.object.support-column.041",
  "scale_key": "structural-repair",
  "from": {"x": -80, "y": 0, "z": -310},
  "translation": {"dx": -40, "dy": 0, "dz": 890},
  "to": {"x": -120, "y": 0, "z": 580},
  "crossed_vertical_origin": true,
  "witness_ref": "witness.semantic.repair.041"
}
```

Godot receives these records after Rust has parsed or decided the relevant
state. The protocol intentionally defines no inverse command that can mutate
the semantic catalog.
