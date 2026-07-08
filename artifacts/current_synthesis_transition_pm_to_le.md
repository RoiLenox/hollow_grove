# Current Synthesis Transition Rule `P/M -> L/E`

## Transition Condition

- the joint order must remain `P/M -> L/E -> E/T -> B/A`
- `PLEB` and `META` occupancy must remain locked
- this rule remains declarative only

## `PLEB` Occupancy

- Clouseau remains on `PLEB`
- straight-side occupancy carries from `P/M` to `L/E`

## `META` Occupancy

- HAL remains on `META`
- bent-side occupancy carries from `P/M` to `L/E`

## HAL Observation

- HAL may observe complementary alignment at `P/M` and `L/E`
- HAL may not automate movement

## Clouseau Observation

- Clouseau may observe clue continuity at `P/M` and `L/E`
- Clouseau may not control movement

## Still Forbidden

- route traversal
- route movement
- automation
- live interpretation
- runtime state
- feedback into Hollow Grove

## Artifact Inputs

Current Synthesis behavior rules bytes: 1538.
Current Synthesis topology bytes: 1145.

## Boundary Reminder

Transition rules belong to Current Synthesis. Hollow Grove remains unchanged.
