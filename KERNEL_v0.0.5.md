# Kernel v0.0.5

Date: 2026-07-06

Root recursion:

`Point -> Triway -> HollowGrove -> Bond -> Link + Atmosphere -> AuraSeam -> Point`

## Lock

- Bond answers which Way becomes Link.
- Bond belongs to HollowGrove.
- Bond selects one Way.
- Unused Ways become Atmosphere.
- Bond has no semantic meaning yet.
- Bond does not know Current, Aura, Climate, PLEB, META, or Routes.
- AuraSeam remains unchanged.

## What changed

- `Bond` was added as a minimal HollowGrove abstraction.
- `HollowGrove` now holds `bond: Bond` instead of a raw linked Way.
- `Bond` selects one Way.
- Remaining Ways continue as Atmosphere.
- The proof output now names Bond directly.

## Intentionally not built yet

- No semantic meaning for Bond
- No climate
- No PLEB/META
- No route names
- No change to AuraSeam

## Exact proof output

```text
Current Synthesis creates Point #1
Point becomes Triway.
Triway carries one Point through three ways.
Triway becomes Hollow Grove
Hollow Grove forms Bond on one Way and leaves two ways as Atmosphere.
Hollow Grove becomes AuraSeam
AuraSeam creates Point #2
Kernel recursion verified.
```
