# Hueman Progression Reconciliation

## Exact mismatch

- `FrameState` only tracked the active `Frame`, `Flow`, `Glow`, and `CurrentPrism`. After a transformation, the runtime had no explicit persistent player `Being` identity even though the canonical model requires the player Being to remain `Hueman`.
- The Hueman slice bridge encoded the unlock phase as `GremlinUnlocked` even when the active quarry slice unlock was the first `Goblin` node `Loadline Grip`. The state machine, persisted artifacts, and Current Synthesis feedback bridge were therefore using a stale phase name for a current-form unlock that was no longer Gremlin-specific.

## Files and types involved

- `src/frame_state.rs`
- `src/point.rs`
- `src/landing.rs`
- `src/hueman_progression.rs`
- `src/current_synthesis_engine.rs`
- `src/bin/hueman_slice_demo.rs`

## Intended canonical model

- `Being = Hueman` persists across transformations.
- `Frame` is the active stabilized mech form.
- `Flow` and `Glow` are learned capabilities that persist unless an explicit frame lock says otherwise.
- The Hueman slice unlock phase represents the first unlocked current-form node for the active slice, not a hard-coded Gremlin-only milestone.

## Smallest safe correction

- Added explicit `BeingId::Hueman` access at the runtime frame-state boundary without altering the frozen V1.1 topology.
- Replaced the stale `GremlinUnlocked` slice phase contract with `CurrentFormUnlocked`.
- Preserved backward compatibility by allowing the parser to read older persisted `GremlinUnlocked` states and normalize them into the new phase.

## Migration implications

- No kernel topology change.
- No V1.1 recipe / execute / point-squared behavior change.
- Older `artifacts/hueman_slice_state.txt` files using `GremlinUnlocked` remain readable and migrate on next write to `CurrentFormUnlocked`.

## Tests added or tightened

- Origin Hueman fixture now checks persistent `Being = Hueman`.
- Gremlin and Pixy transformation fixtures now assert `Being` persistence.
- Frame-switch persistence test confirms learned `Flow` and `Glow` survive legal frame changes.
- Hueman slice parser compatibility continues to accept older `GremlinUnlocked` state files while enforcing the new current-form unlock invariant.
