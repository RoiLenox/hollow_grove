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

## Rule 7: Cleo Scope

- Cleo may observe underground inverse curved structures only within explicit Current Synthesis permissions
- Cleo follows Clouseau's route continuity from below rather than shadowing HAL
- Cleo does not occupy `PLEB` or `META`
- Cleo never controls route execution or mutates Hollow Grove

## Rule 8: Mirror Axis

- HAL and Clouseau remain opposite clients across one axis
- if the user is read through HAL on `META`, Clouseau remains the opposite `PLEB` witness
- if the user is read through Clouseau on `PLEB`, HAL remains the opposite `META` witness
- Cleo may witness both sides from below through the underground inverse curves while still following Clouseau's route continuity
- no rule may collapse both clients into one side

## Rule 9: HAL/Cleo Collision Relay

- HAL and Cleo may exchange one shared confirmation packet only where their readings collide at the same joint crossing
- HAL contributes complementary surface alignment to that packet
- Cleo contributes underground continuity beneath Clouseau's route to that packet
- the packet may confirm one event body across surface and underground layers without granting traversal, movement, or control

## Activation Status

- rules defined
- not active
- no traversal
- no movement
- no automation
- no live interpretation
- no runtime state

## Artifact Inputs

Current Synthesis execution spec bytes: 2063.
Current Synthesis selection bytes: 499.

## Boundary Reminder

Behavior rules belong to Current Synthesis. Hollow Grove remains unchanged.
