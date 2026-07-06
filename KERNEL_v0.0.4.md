# Kernel v0.0.4

Date: 2026-07-06

Root recursion:

`Point -> Triway -> HollowGrove -> Link + Atmosphere -> AuraSeam -> Point`

## Lock

- Point stays singular.
- Triway opens one Point into three Ways.
- HollowGrove receives the three Ways.
- One Way bonds into Link.
- Unused Ways become Atmosphere.
- AuraSeam still returns the singular Point.

## What changed

- `HollowGrove` no longer stores a raw `Triway`.
- `HollowGrove` now resolves Triway into:
  - `link: Way`
  - `atmosphere: [Way; 2]`
- The first bonded version is hard-coded:
  - `Way::One` becomes `Link`
  - `Way::Two` and `Way::Three` become `Atmosphere`
- The kernel proof and tests were updated to reflect this resolution step.

## Intentionally not built yet

- No climate
- No PLEB/META types or engine
- No route names
- No semantic meaning for the Ways beyond ordinal identity

## Exact proof output

```text
Current Synthesis creates Point #1
Point becomes Triway.
Triway carries one Point through three ways.
Triway becomes Hollow Grove
Hollow Grove bonds Way::One as Link and leaves two ways as Atmosphere.
Hollow Grove becomes AuraSeam
AuraSeam creates Point #2
Kernel recursion verified.
```
