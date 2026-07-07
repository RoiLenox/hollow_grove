# Current Synthesis Behavior Rules

## Rule 1: Occupancy

- the selected side remains occupied by its assigned client
- the complementary side remains occupied by its assigned client

## Rule 2: Joint Order

- any future route behavior must follow `P/M -> L/E -> E/T -> B/A`
- no joint may be skipped

## Rule 3: `PLEB`

- `PLEB` remains the straight-side route context
- Clouseau remains the `PLEB` client
- clue production stays within Current Synthesis

## Rule 4: `META`

- `META` remains the bent-side route context
- HAL remains the `META` client
- complementary occupancy stays within Current Synthesis

## Rule 5: HAL Scope

- HAL may act only within explicit Current Synthesis permissions
- HAL never mutates Hollow Grove

## Rule 6: Clouseau Scope

- Clouseau may interpret only within explicit Current Synthesis permissions
- Clouseau never controls route execution

## Activation Status

- rules defined
- not active
- no traversal
- no movement
- no automation
- no live interpretation
- no runtime state

## Artifact Inputs

Current Synthesis execution spec bytes: 1291.
Current Synthesis selection bytes: 498.

## Boundary Reminder

Behavior rules belong to Current Synthesis. Hollow Grove remains unchanged.
